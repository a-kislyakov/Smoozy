//! The first connection, driven — `Servers` §10, Q1.
//!
//! **[`crate::first_connection`] is the half that decides and this is the half
//! that runs it.** Every judgement below is that module's: the three host-trust
//! paths, Q13's three not-Linux outcomes, Q17's bound on the changed-identity
//! warning, and Q19's fourth-branch re-take. **Nothing here retakes one.**
//!
//! **The ordering is the protocol's and not a setting.** `Server protocol and
//! transfers` A5 puts the host key comparison **before any connection that
//! presents Smoozy's credential**, and the transport makes that structural:
//! the comparison happens inside key exchange, which RFC 4253 puts before
//! authentication is asked for at all. **There is no path through this file
//! where a credential goes first.**
//!
//! **Q7's throwaway known-hosts file is not built here and must not be.**
//! `Security and licence` **T2** struck it — *"there is no client to hand one
//! to"* — and **T2a says Q7 is routed to the architecture runner rather than
//! edited**, so a builder reading Q7 alone builds the wrong thing. What closes
//! the gap between the scan and the connection instead is **T2a's own answer**:
//! the remembered entry is read from the local record at the attempt's own
//! moment and compared in Smoozy's own process, and the transport stops the
//! attempt where the two differ.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` A4, C16,
//! F16. `Job files and outputs` L1. `Security and licence` T2a.
//!
//! **`Server protocol and transfers` A5 is deliberately not listed above and is
//! written out with its document at the site**, because this plan defines an A5
//! of its own and a bare one could mean either.

use std::sync::{Arc, Mutex};

use smoozy_server_protocol::transport::{DecideHostTrust, Endpoint, HostTrust, OpenFailure};
use smoozy_state::identities::InstallationCheck;

use crate::acting::{
    OpenConnections, SupplyTheCredential, TheActingHalf, TheInstallationIdentityRead,
    TheServerBeingConnectedTo, Wait, WhatServersPublishes,
};
use crate::first_connection::{
    self, ChangedIdentityWarning, HostTrustPath, NotLinuxOutcome, Step, TheFourthBranchRetake,
};
use crate::hardware::{what_the_branch_does, HardwareReading};
use crate::key_moments::{TheTwoAnswers, WhoStartedIt};
use crate::readings;
use crate::store::{RememberedHostKey, ServerRow};

/// Which connection an attempt is making.
///
/// **Q16: the additional connection runs steps 1 to 3 and no others**, and S14
/// condition 4 is the single home of that exception.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionRole {
    Primary,
    Additional,
}

/// What one attempt is being made against.
pub struct AnAttempt<'a> {
    pub row: &'a ServerRow,
    /// **P4 and Q17 both turn on it**: who started it decides whether a
    /// passphrase may be asked for, and whether the changed-identity warning is
    /// raised on every attempt or once per repair.
    pub who: WhoStartedIt,
    /// **Step 4's *Runs on* column**: the not-Linux check runs on the first
    /// connection to a server and before anything else acts on the machine.
    pub the_first_connection_to_this_server: bool,
    pub role: ConnectionRole,
    /// **N2's connect deadline, or the run's own remaining time where that is
    /// shorter** ([`crate::repair::Run::the_deadline_for_an_attempt`]). N2a
    /// abandons an attempt still in flight when the span closes, and this is
    /// the figure that carries it: **whichever of the two comes first**. An
    /// attempt that belongs to no run carries [`crate::repair::CONNECT_DEADLINE`]
    /// and nothing shorter.
    pub deadline: std::time::Duration,
}

/// How one attempt ended.
pub enum Attempt<C> {
    /// Every step that runs on this connection completed, and the connection is
    /// live.
    ItIsLive(WhatTheConnectionGave<C>),
    /// **S10 item 5: the connection the act was to make never became live** —
    /// including where the attempt was abandoned by a step of the sequence, and
    /// including where P3 meant no attempt was made at all.
    ItStopped(WhereItStopped),
}

/// The live connection and what the steps after authentication read.
pub struct WhatTheConnectionGave<C> {
    pub connection: C,
    /// **Step 5**, on a primary connection. Absent on the additional one, which
    /// runs steps 1 to 3 and no others.
    pub installation: Option<InstallationCheck>,
    /// **Step 6**, on the three branches that take a fresh reading (H18).
    /// **Absent on the fourth branch, and that absence is H21's requirement
    /// rather than a gap**: a reading taken there would carry an unconfirmed
    /// identity and would be shown as live.
    pub hardware: Option<HardwareReading>,
    /// **What is not built, named rather than silently skipped.**
    pub steps_not_built: &'static [Step],
}

/// Which step stopped it, because what Smoozy says differs by step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereItStopped {
    /// **Step 2, P3: where either answer is no, no attempt is made.** The server
    /// is left disconnected, marked with what is wrong, and Smoozy says so.
    /// **This is what makes *an automatic attempt never puts a passphrase prompt
    /// on screen* true by not making the attempt.**
    TheKeyCheck(TheTwoAnswers),
    /// Both answers were yes and no credential came back — `Keys`' own supply
    /// answering nothing. **Nothing is invented in its place.**
    NoCredentialWasSupplied,
    /// **Q9: a scan that gets no answer is not a changed identity.** A
    /// connection that could not be made, reported as one, **with no claim about
    /// the machine's identity**.
    TheAddressDidNotAnswer,
    TheExchangeDidNotComplete,
    /// **Q6's third path.** The attempt stopped before anything else happened
    /// and **no credential was presented**.
    TheIdentityDiffers,
    TheKeyCouldNotBeLoaded,
    /// **A4: nothing retries an authentication.** Reported as what it is.
    TheMachineRefusedTheCredential,
    /// **N2's connect deadline arrived.** A waiter's deadline under `Server
    /// protocol and transfers` C16, which **changes no durable state**, and
    /// which claims nothing about whether the address answered.
    TheConnectDeadlineExpired,
    /// **Q13's outcomes 2 and 3.** Which of the two it was is carried, because
    /// only outcome 2 says what the machine is.
    TheNotLinuxCheck(NotLinuxOutcome),
}

/// **Steps 7 and 8 are not built at this step and saying so is the point.**
///
/// Step 7 is desktop reconciliation steps 2 to 5, the queue reading and the
/// adoption offer — `State, identity and recovery` §7, `Queue & History` and
/// §11.3. Step 8 hands a remembered setup to `Server Setup`. **What removes
/// each is the module that owns it**, and neither is a gap in this sequence.
///
/// *(Corrected 2026-09-07. This said the modules step 7 needs did not exist
/// yet, and pointed at step 6 of `Preparation/Build/Plan.md` as the thing that
/// would bring `Server Setup`. Both have since landed: the catch-up is
/// `smoozy_queue_history::acting::catching_up`, the queue reading and the adoption
/// offer are `smoozy_queue_history::acting::adoption`, and `Server Setup` is a crate.
/// **What is still true is why these two steps are not built here**: this
/// crate depends on none of them — the hand-over runs the other way, from
/// whoever drives this sequence — so a connection made here still stops at
/// step 6.)*
pub const STEPS_NOT_BUILT: [Step; 2] = [
    Step::ReconciliationTheQueueReadingAndTheOffer,
    Step::ARememberedSetup,
];

/// **Q1's sequence, in order.**
///
/// Step 2 is taken before step 1 in this file and after it in Q1's table, which
/// **Q2 settles: the order between them does not matter and what matters is
/// that both are before step 3.** Neither reaches the machine with Smoozy's
/// credential and neither depends on the other, and step 2 is the cheaper of
/// the two — which is Q1's own *before step 1 costs anything*.
///
/// **`the_connection_is_live` is called once, at Q1 step 3's own moment** — the
/// instant the transport reports the connection authenticated and usable, and
/// **before any of the readings above it run**. It is the caller's, because
/// K6's value is composed from the period's two facts and this module holds
/// neither. **Q19 has step 5's re-take running *"on the primary connection that
/// is already live"***, K12 says a supervisor that does not answer is never
/// read as a broken connection, and W16 says the server connection is not
/// marked down after a username edit; all three are false if the value moves
/// only when the last reading settles.
pub async fn run<O, K, I, P, W>(
    attempt: AnAttempt<'_>,
    half: &mut TheActingHalf<O, K, I, P, W>,
    warning: &mut ChangedIdentityWarning,
    the_connection_is_live: &mut dyn FnMut(&mut P),
) -> Attempt<O::Connection>
where
    O: OpenConnections,
    K: SupplyTheCredential,
    I: TheInstallationIdentityRead<O::Connection>,
    P: WhatServersPublishes,
    W: Wait,
{
    let row = attempt.row;
    let server = row.local_server_identifier.as_str();

    // ---- Step 2: the key check of §9.1, taken at the moment of the attempt.
    //
    // **P3 binds a connection Smoozy makes on its own entire, and P4 lets a
    // connect the user started ask.** So the second answer gates the attempt
    // only where nobody may be asked: on a user-started connect the ask is
    // `Keys`' and what comes back is a credential or nothing. **The first
    // answer gates every attempt** — Row 33's *no attempt is made* is about the
    // key being usable and is not a rule about who is standing there.
    let answers = half.credentials.the_two_answers(&row.key);
    let may_be_made = if attempt.who.may_ask_for_a_passphrase() {
        answers.the_key_is_usable
    } else {
        answers.an_attempt_may_be_made()
    };
    if !may_be_made {
        return Attempt::ItStopped(WhereItStopped::TheKeyCheck(answers));
    }

    let (label, username, host) = row.what_the_passphrase_box_is_handed();
    let Some(credential) = half.credentials.for_one_attempt(
        &row.key,
        attempt.who,
        TheServerBeingConnectedTo {
            label,
            username,
            host,
        },
    ) else {
        return Attempt::ItStopped(WhereItStopped::NoCredentialWasSupplied);
    };

    // ---- Steps 1 and 3: the comparison inside key exchange, then
    // authentication with the compared entry enforced.
    let comparison = TheComparison::against(row.remembered_host_key.clone());
    let found = comparison.what_it_found();
    let opened = half
        .opener
        .open(
            Endpoint {
                host: row.host.as_str().to_string(),
                port: row.port.number(),
                username: row.username.as_str().to_string(),
            },
            credential,
            Box::new(comparison),
            attempt.deadline,
        )
        .await;

    let the_comparison_found = found.lock().ok().and_then(|found| found.clone());
    if let Some(WhatTheComparisonFound {
        path: HostTrustPath::ItDiffers,
        ..
    }) = the_comparison_found
    {
        // **Q17: raised once per repair, and every time on a connect the user
        // started.** Declining leaves the record as it is and the standing
        // repair goes on failing at step 1.
        if warning.should_be_raised(HostTrustPath::ItDiffers, attempt.who.may_ask_for_a_passphrase())
        {
            half.publishes.the_changed_identity_warning(server);
        }
    }

    let mut connection = match opened {
        Ok(connection) => connection,
        Err(failure) => {
            return Attempt::ItStopped(match failure {
                OpenFailure::TheAddressDidNotAnswer => WhereItStopped::TheAddressDidNotAnswer,
                OpenFailure::TheExchangeDidNotComplete => WhereItStopped::TheExchangeDidNotComplete,
                OpenFailure::HostTrustStoppedIt => WhereItStopped::TheIdentityDiffers,
                OpenFailure::TheKeyCouldNotBeLoaded => WhereItStopped::TheKeyCouldNotBeLoaded,
                OpenFailure::TheMachineRefusedTheCredential => {
                    WhereItStopped::TheMachineRefusedTheCredential
                }
                OpenFailure::TheDeadlineExpired => WhereItStopped::TheConnectDeadlineExpired,
            })
        }
    };

    // ---- Step 3 has completed: the transport reports the connection
    // authenticated and usable. **That is the moment the connection is live**,
    // and every step below runs under a server that reads *connected*.
    the_connection_is_live(&mut half.publishes);

    // **Q6's first path: the scanned key is recorded against the server and the
    // connection proceeds. Nothing is asked and nothing is shown.** It is
    // written after the connection is up rather than at the comparison, so a
    // machine that offered a key and then refused the credential leaves nothing
    // recorded against a connection that never happened.
    if let Some(WhatTheComparisonFound {
        path: HostTrustPath::NothingRemembered,
        offered,
    }) = the_comparison_found
    {
        half.publishes.record_the_scanned_host_key(server, offered);
    }

    // ---- Q16: the additional connection runs steps 1 to 3 and no others.
    if attempt.role == ConnectionRole::Additional {
        return Attempt::ItIsLive(WhatTheConnectionGave {
            connection,
            installation: None,
            hardware: None,
            steps_not_built: &[],
        });
    }

    // ---- Step 4: the not-Linux check, on the first connection to a server and
    // before anything else acts on the machine.
    if attempt.the_first_connection_to_this_server {
        let outcome = readings::read_the_operating_system(&mut connection).await;
        if outcome != NotLinuxOutcome::ItNamedLinux {
            // The sequence is abandoned here, so the connection the opening act
            // was to make never becomes live — and a connection left open under
            // an abandoned sequence would be one nothing owns.
            half.opener.tear_down(connection).await;
            return Attempt::ItStopped(WhereItStopped::TheNotLinuxCheck(outcome));
        }
    }

    // ---- Step 5: the installation identity and its four branches, with Q19's
    // re-take on the fourth.
    let mut read = half.installation.read(&mut connection).await;
    let mut retake = TheFourthBranchRetake::begun();
    while !retake.settled(read.check) {
        // **What ends it, and the list is closed**: the read settling on one of
        // the other three branches, **the break declaration, there being no
        // connection left to read on**, and the period ending — which is the
        // caller dropping this future.
        if half.opener.ended(&connection).is_some() {
            return Attempt::ItStopped(WhereItStopped::TheExchangeDidNotComplete);
        }
        // **The cadence is the two figures this document already derives**,
        // measured from the **end** of the previous read. No figure is added.
        half.wait.until(retake.pause_before_the_next_read()).await;
        retake.reads_taken = retake.reads_taken.saturating_add(1);
        read = half.installation.read(&mut connection).await;
    }

    // ---- Step 6: the hardware reading, on the three branches that take one.
    let hardware = if what_the_branch_does(read.check).a_fresh_reading_is_taken {
        let now = half.wait.now();
        readings::take_the_hardware_reading(
            &mut connection,
            server,
            // **H17: the identity present on the machine when the reading was
            // taken, or the explicit absence of one.**
            read.identity_on_the_machine.clone(),
            // **H10's path, and it is the recorded root.** `Job files and
            // outputs` **L1** chooses the root once per installation at
            // hand-over and records it against the server row; that column now
            // exists, so the reading has a path to be taken against. **A
            // machine that has never been set up has no such root**, and there
            // the figure is absent — **nothing is substituted for it**, which
            // is what H10 and `Server protocol and transfers` F16 require.
            attempt
                .row
                .recorded_machine_root
                .as_ref()
                .map(|root| root.path.as_str()),
            now,
        )
        .await
    } else {
        None
    };

    Attempt::ItIsLive(WhatTheConnectionGave {
        connection,
        installation: Some(read.check),
        hardware,
        steps_not_built: &STEPS_NOT_BUILT,
    })
}

/// **Q4 and Q5: the comparison, made in Smoozy's own process at the one moment
/// it can be made — before a connection presenting Smoozy's credential
/// exists.**
///
/// **Nothing here parses a message** (Q5, Fact 37): no OpenSSH manual documents
/// a single one of the diagnostic strings the tools emit, and a changed host key
/// is observable at default log level only through a banner whose text is
/// undocumented. What this compares is the value in the local record against the
/// bytes the machine offered.
///
/// **What leaves this type is a decision and never a value** (D9). The offered
/// key is held for the one destination `Security and licence` §7 names — the
/// local record, on Q6's first path — and reaches nothing else.
pub struct TheComparison {
    remembered: Option<RememberedHostKey>,
    found: Arc<Mutex<Option<WhatTheComparisonFound>>>,
}

/// What the comparison came out at, read by the caller after the attempt.
#[derive(Debug, Clone)]
pub struct WhatTheComparisonFound {
    pub path: HostTrustPath,
    /// The key the machine offered. **Recorded only on Q6's first path**, and
    /// never shown, logged or handed to another module.
    pub offered: RememberedHostKey,
}

impl TheComparison {
    pub fn against(remembered: Option<RememberedHostKey>) -> TheComparison {
        TheComparison {
            remembered,
            found: Arc::new(Mutex::new(None)),
        }
    }

    /// The handle the caller keeps, because the comparison itself is moved into
    /// the transport.
    pub fn what_it_found(&self) -> Arc<Mutex<Option<WhatTheComparisonFound>>> {
        Arc::clone(&self.found)
    }
}

impl DecideHostTrust for TheComparison {
    fn decide(&mut self, offered: &[u8]) -> HostTrust {
        let path = first_connection::compare(self.remembered.as_ref(), offered);
        if let Ok(mut found) = self.found.lock() {
            *found = Some(WhatTheComparisonFound {
                path,
                offered: RememberedHostKey::approved(offered.to_vec()),
            });
        }
        match path {
            // **Q6: nothing remembered trusts silently, and a match says
            // nothing.**
            HostTrustPath::NothingRemembered | HostTrustPath::ItMatches => HostTrust::Proceed,
            // **The attempt stops before anything else happens.**
            HostTrustPath::ItDiffers => HostTrust::Stop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{AMachine, TheDesk};
    use crate::identity::{Host, KeyReference, Port, Username};
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_row() -> ServerRow {
        ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer)
                .expect("randomness")
                .as_text(),
            Host::what_the_user_typed("5.180.183.77").expect("a host"),
            Port::what_the_user_typed(33510).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        )
    }

    fn an_attempt(row: &ServerRow) -> AnAttempt<'_> {
        AnAttempt {
            row,
            who: WhoStartedIt::TheUser,
            the_first_connection_to_this_server: true,
            role: ConnectionRole::Primary,
            deadline: crate::repair::CONNECT_DEADLINE,
        }
    }

    /// The callback Q1 step 3 calls, where a test does not care what it does.
    fn nothing_to_publish(_publishes: &mut crate::testing::ThePublisher) {}

    #[tokio::test]
    async fn the_key_check_stops_the_attempt_before_anything_reaches_the_machine() {
        // P3, and it is what makes *an automatic attempt never puts a
        // passphrase prompt on screen* true by not making the attempt.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering());
        desk.half.credentials.usable = false;
        let ended = run(an_attempt(&row), &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish)
            .await;
        assert!(matches!(
            ended,
            Attempt::ItStopped(WhereItStopped::TheKeyCheck(_))
        ));
        assert_eq!(desk.half.opener.attempts_made(), 0);
    }

    #[tokio::test]
    async fn no_passphrase_available_and_nobody_to_ask_makes_no_attempt() {
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering());
        desk.half.credentials.a_passphrase_is_available = false;
        let mut attempt = an_attempt(&row);
        attempt.who = WhoStartedIt::SmoozyOnItsOwn;
        let ended = run(attempt, &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish).await;
        assert!(matches!(
            ended,
            Attempt::ItStopped(WhereItStopped::TheKeyCheck(_))
        ));
        assert_eq!(desk.half.opener.attempts_made(), 0);
    }

    #[tokio::test]
    async fn a_first_connection_records_the_scanned_key_and_says_nothing() {
        // Q6's first path: silent first trust. Nothing is asked and nothing is
        // shown, and the key is recorded against the server.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering());
        let ended = run(an_attempt(&row), &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish)
            .await;
        assert!(matches!(ended, Attempt::ItIsLive(_)));
        assert_eq!(desk.half.publishes.host_keys_recorded, 1);
        assert_eq!(desk.half.publishes.changed_identity_warnings, 0);
    }

    #[tokio::test]
    async fn a_matching_identity_records_nothing_and_says_nothing() {
        let mut row = a_row();
        row.remembered_host_key = Some(RememberedHostKey::approved(
            AMachine::THE_HOST_KEY.to_vec(),
        ));
        let mut desk = TheDesk::with(AMachine::answering());
        let ended = run(an_attempt(&row), &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish)
            .await;
        assert!(matches!(ended, Attempt::ItIsLive(_)));
        assert_eq!(desk.half.publishes.host_keys_recorded, 0);
        assert_eq!(desk.half.publishes.changed_identity_warnings, 0);
    }

    #[tokio::test]
    async fn a_differing_identity_stops_the_attempt_and_raises_the_warning() {
        // Q6's third path, and `Server protocol and transfers` A5's ordering:
        // no credential is presented.
        let mut row = a_row();
        row.remembered_host_key = Some(RememberedHostKey::approved(b"a different key".to_vec()));
        let mut desk = TheDesk::with(AMachine::answering());
        let ended = run(an_attempt(&row), &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish)
            .await;
        assert!(matches!(
            ended,
            Attempt::ItStopped(WhereItStopped::TheIdentityDiffers)
        ));
        assert_eq!(desk.half.publishes.changed_identity_warnings, 1);
        assert_eq!(desk.half.publishes.host_keys_recorded, 0);
        assert_eq!(desk.half.opener.credentials_presented(), 0);
    }

    #[tokio::test]
    async fn a_machine_that_is_not_linux_is_said_plainly_and_the_connection_is_torn_down() {
        // Q13 outcome 2, and S10 item 5: the connection never became live.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().naming_the_operating_system("Darwin"));
        let ended = run(an_attempt(&row), &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish)
            .await;
        assert_eq!(
            ended_at(&ended),
            Some(WhereItStopped::TheNotLinuxCheck(
                NotLinuxOutcome::ItNamedSomethingElse
            ))
        );
        assert_eq!(desk.half.opener.torn_down(), 1);
    }

    #[tokio::test]
    async fn a_machine_that_answers_nothing_claims_nothing_about_what_it_is() {
        // Q13 outcome 3, which is the whole-app rule kept.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().saying_nothing_about_its_operating_system());
        let ended = run(an_attempt(&row), &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish)
            .await;
        assert_eq!(
            ended_at(&ended),
            Some(WhereItStopped::TheNotLinuxCheck(
                NotLinuxOutcome::NothingSmoozyCouldRead
            ))
        );
    }

    #[tokio::test]
    async fn the_not_linux_check_runs_only_on_the_first_connection_to_a_server() {
        // Step 4's *Runs on* column: a later connection does not re-ask.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().naming_the_operating_system("Darwin"));
        let mut attempt = an_attempt(&row);
        attempt.the_first_connection_to_this_server = false;
        let ended = run(attempt, &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish).await;
        assert!(matches!(ended, Attempt::ItIsLive(_)));
    }

    #[tokio::test]
    async fn the_additional_connection_runs_the_first_three_steps_and_no_others() {
        // Q16, and S14 condition 4: no command, no identity read, no reading.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering());
        let mut attempt = an_attempt(&row);
        attempt.role = ConnectionRole::Additional;
        let ended = run(attempt, &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish).await;
        let Attempt::ItIsLive(gave) = ended else {
            panic!("it is live")
        };
        assert!(gave.installation.is_none());
        assert!(gave.hardware.is_none());
        assert_eq!(desk.half.installation.reads_taken, 0);
        assert_eq!(gave.connection.commands_run(), 0);
    }

    #[tokio::test]
    async fn a_confirmed_branch_takes_a_fresh_reading_carrying_the_identity_it_was_read_under() {
        // H17, H18 and H19.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering());
        desk.half.installation.answers = vec![InstallationCheck::PresentAndEqualConfirmed];
        desk.half.installation.identity = Some("inst-1".into());
        let ended = run(an_attempt(&row), &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish)
            .await;
        let Attempt::ItIsLive(gave) = ended else {
            panic!("it is live")
        };
        let reading = gave.hardware.expect("a fresh reading");
        assert_eq!(reading.read_under, Some("inst-1".into()));
        // **H10's figure is absent on a machine with no recorded root**, and
        // nothing is substituted for it. This row is a plain saved server that
        // no hand-over has reached.
        assert!(reading.free_space.is_none());
        assert!(
            gave.connection.free_space_paths().is_empty(),
            "a reading was taken against a path nobody recorded"
        );
    }

    #[tokio::test]
    async fn h10s_free_space_is_read_against_the_root_the_row_records_and_no_other_path() {
        // **`Job files and outputs` L1's column joined up.** The root is chosen
        // once per installation at hand-over and recorded against the server
        // row; H10's reading is per path and this is the path. **Nothing
        // substitutes a whole-machine figure for it** (H10, `Server protocol
        // and transfers` F16), which is why the reading carries the path it was
        // taken against and this test reads that field rather than the number.
        let mut row = a_row();
        row.recorded_machine_root = Some(crate::store::RecordedMachineRoot {
            path: "/ephemeral/smoozy".into(),
            the_package_areas_own_location: None,
            read_under: "inst-1".into(),
        });
        let mut desk = TheDesk::with(AMachine::answering());
        desk.half.installation.answers = vec![InstallationCheck::PresentAndEqualConfirmed];
        desk.half.installation.identity = Some("inst-1".into());
        let ended = run(an_attempt(&row), &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish)
            .await;
        let Attempt::ItIsLive(gave) = ended else {
            panic!("it is live")
        };
        let reading = gave.hardware.expect("a fresh reading");
        let space = reading.free_space.expect("H10's figure");
        assert_eq!(space.the_path_it_was_taken_against, "/ephemeral/smoozy");
        assert_eq!(gave.connection.free_space_paths(), vec!["/ephemeral/smoozy"]);
    }

    #[tokio::test]
    async fn the_fourth_branch_is_retaken_alone_until_it_settles_and_step_six_waits() {
        // Q19: steps 1 to 3 are per attempt and this is no new attempt, step 4
        // is the first connection's, and steps 6 and 7 have not run.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering());
        desk.half.installation.answers = vec![
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer,
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer,
            InstallationCheck::PresentAndEqualConfirmed,
        ];
        let ended = run(an_attempt(&row), &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish)
            .await;
        let Attempt::ItIsLive(gave) = ended else {
            panic!("it is live")
        };
        assert_eq!(desk.half.installation.reads_taken, 3);
        assert_eq!(desk.half.opener.attempts_made(), 1, "no new attempt");
        assert!(gave.hardware.is_some(), "step 6 runs when it settles");
        // The cadence is the repair's own two figures and no third is added.
        assert_eq!(
            desk.half.wait.waited_for,
            vec![crate::repair::PAUSE_BETWEEN_ATTEMPTS; 2]
        );
    }

    #[tokio::test]
    async fn nothing_is_read_on_the_fourth_branch_while_it_stands() {
        // H21: a reading taken there would carry an unconfirmed identity and
        // would be shown as live, which §2.3 forbids in terms. The retake ends
        // at the break declaration, there being no connection left to read on.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().whose_connection_has_ended());
        desk.half.installation.answers =
            vec![InstallationCheck::PresentAndTheSupervisorDidNotAnswer];
        let ended = run(an_attempt(&row), &mut desk.half, &mut ChangedIdentityWarning::none_yet(), &mut nothing_to_publish)
            .await;
        assert!(matches!(ended, Attempt::ItStopped(_)));
    }

    #[tokio::test]
    async fn steps_seven_and_eight_are_named_rather_than_silently_skipped() {
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering());
        let Attempt::ItIsLive(gave) =
            run(
            an_attempt(&row),
            &mut desk.half,
            &mut ChangedIdentityWarning::none_yet(),
            &mut nothing_to_publish,
        )
        .await
        else {
            panic!("it is live")
        };
        assert_eq!(gave.steps_not_built, &STEPS_NOT_BUILT);
        assert_eq!(STEPS_NOT_BUILT.len(), 2);
    }

    fn ended_at<C>(attempt: &Attempt<C>) -> Option<WhereItStopped> {
        match attempt {
            Attempt::ItStopped(where_it_stopped) => Some(*where_it_stopped),
            Attempt::ItIsLive(_) => None,
        }
    }
}

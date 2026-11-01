//! The first connection, in order — `Servers` §10.
//!
//! **The constraint** is Row 35's ordering — the not-Linux check runs **on the
//! first connection and before anything else acts on the machine** — and
//! `Server protocol and transfers` A5, which puts the host key comparison
//! **before any connection that presents Smoozy's credential**. `Security and
//! licence` §7 owns host trust and is cited rather than restated.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` P10d.
//!
//! **`Server protocol and transfers` A5 is deliberately not listed above and is
//! written out with its document at the site**, because this plan defines an A5
//! of its own and a bare one could mean either.

use smoozy_state::identities::InstallationCheck;

use crate::period::PeriodEnding;
use crate::store::RememberedHostKey;

/// **Q1: the sequence, in order, and every step's own section says what it
/// does.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// **1. Host trust** — obtain the offered host key without authenticating,
    /// compare, decide. Runs on **every attempt**.
    HostTrust,
    /// **2. The key check** of §9.1. Runs on **every attempt, before step 1
    /// costs anything**.
    ///
    /// **Q2: step 2 sits before step 1 in cost and after it in the list, and
    /// the order between them does not matter.** Neither reaches the machine
    /// with Smoozy's credential and neither depends on the other. **What matters
    /// is that both are before step 3.**
    TheKeyCheck,
    /// **3. Authentication**, with the compared entry enforced. Every attempt.
    Authentication,
    /// **4. The not-Linux check.** The **first connection to a server, before
    /// anything else acts on the machine**.
    TheNotLinuxCheck,
    /// **5. The installation identity** and its four branches. Every primary
    /// connection.
    TheInstallationIdentity,
    /// **6. The hardware reading.** Every primary connection, **on the three
    /// branches that take a fresh reading**.
    TheHardwareReading,
    /// **7. Desktop reconciliation steps 2 to 5, then the queue reading and the
    /// adoption offer.** Every primary connection, **only on the confirmed
    /// branch**.
    ///
    /// **W41a: the running-time figure's request is here too** — one per primary
    /// connection, **step 7 and not step 6**, because it is a health reading and
    /// needs the supervisor answering.
    ReconciliationTheQueueReadingAndTheOffer,
    /// **8. A remembered setup**, handed to `Server Setup`. A connect carrying
    /// one.
    ARememberedSetup,
}

impl Step {
    pub const IN_ORDER: [Step; 8] = [
        Step::HostTrust,
        Step::TheKeyCheck,
        Step::Authentication,
        Step::TheNotLinuxCheck,
        Step::TheInstallationIdentity,
        Step::TheHardwareReading,
        Step::ReconciliationTheQueueReadingAndTheOffer,
        Step::ARememberedSetup,
    ];

    /// **Q16: the additional connection runs steps 1 to 3 and no others.** S14
    /// condition 4 is the single home of that exception.
    pub fn runs_on_the_additional_connection(self) -> bool {
        matches!(self, Step::HostTrust | Step::TheKeyCheck | Step::Authentication)
    }

    /// **The *Runs on* column, performed.**
    pub fn runs_on_every_attempt(self) -> bool {
        matches!(self, Step::HostTrust | Step::TheKeyCheck | Step::Authentication)
    }
}

/// **Q3: the policy is settled and is not this document's to retake.** Silent
/// first trust plus a **user-decided** replacement of a **changed** identity is
/// Smoozy's own policy, Creator-stated 2026-07-17. It is **neither** SSH's own
/// default, which asks before recording a new identity, **nor** the documented
/// setting that accepts new identities without asking while still refusing
/// changed ones — **that setting does not deliver the accept path.** **Carrying
/// it out is this document's.**
///
/// **Q6: the three paths are `Security and licence` §7's and are performed
/// here.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostTrustPath {
    /// **Nothing remembered.** The scanned key is recorded against the server
    /// and the connection proceeds. **Nothing is asked and nothing is shown.**
    NothingRemembered,
    /// **It matches.** The connection proceeds. **Nothing is said.**
    ItMatches,
    /// **It differs. The attempt stops before anything else happens.** The
    /// changed-identity warning is raised and the user decides. **Accepting**
    /// replaces the recorded value and repeats the first-connection path, and
    /// the attempt goes ahead. **Declining** keeps the old record, leaves the
    /// server disconnected, and **the same warning returns on the next attempt**
    /// until the user accepts or the machine answers with the identity Smoozy
    /// remembers.
    ItDiffers,
}

/// **Q4: the offered host key is obtained by a scan that presents no credential,
/// and the scan is what the decision is taken from.**
///
/// Observed (Fact 82), in the observation's own words — the scan *"exited 0,
/// wrote nothing to stderr"* and returned every key type the machine offers, and
/// *"No identity file was supplied and none was asked for"*; and *"Two
/// independent scans, minutes apart, returned byte-identical key material."*
///
/// **So the trust decision happens before a connection presenting Smoozy's
/// credential exists**, which is `Server protocol and transfers` A5's
/// requirement and **the whole reason the
/// accept-a-change path can be honoured at all**.
///
/// **Q5: the comparison is against the value in the local record and never
/// against a diagnostic string.** Fact 37 is why: **no OpenSSH manual documents
/// a single one of the diagnostic strings the tools emit**, and a changed host
/// key is observable at default log level only through a banner whose text is
/// undocumented. **Nothing here parses a message.**
pub fn compare(remembered: Option<&RememberedHostKey>, offered: &[u8]) -> HostTrustPath {
    match remembered {
        None => HostTrustPath::NothingRemembered,
        Some(remembered) if remembered.matches(offered) => HostTrustPath::ItMatches,
        Some(_) => HostTrustPath::ItDiffers,
    }
}

/// **Q9: a scan that gets no answer is not a changed identity.** It is a
/// connection that could not be made, reported as one, **with no claim about the
/// machine's identity**.
pub fn no_answer_is_a_changed_identity() -> bool {
    false
}

/// **Q8: accepting is a choice to trust the machine and not a check that has
/// been passed.** No fingerprint is shown at a first connection, at a changed
/// one, in the warning, or in any log. **There is nothing here for the user to
/// check the machine against**, which is silent first trust holding at the
/// second connection as well as the first.
pub const NOTHING_IS_SHOWN_FOR_THE_USER_TO_CHECK_AGAINST: bool = true;

/// **Q17: on a repair attempt the user did not start, the changed-identity
/// warning is bounded to the declared break and not to the attempt.**
///
/// Q6's *It differs* path is literal for a connect the user started and for the
/// edit put-back, where the user is standing in front of the act that raised it.
/// **Under the standing repair the next attempt is Smoozy's, fifteen seconds
/// later, for as long as the app is open** — so the bound is stated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChangedIdentityWarning {
    raised_in_this_repair: bool,
}

impl ChangedIdentityWarning {
    pub fn none_yet() -> ChangedIdentityWarning {
        ChangedIdentityWarning::default()
    }

    /// **The warning is raised once**, on the first attempt of the repair that
    /// meets a differing identity. **The run goes on** at the repair's cadence,
    /// **with the identity comparison still made at every attempt** (N9) and **no
    /// further warning raised while the question stands**. **Nothing stops the
    /// clock and nothing loops**: N2a's 105-second run span and M4's 600-second
    /// wait are untouched by it.
    pub fn should_be_raised(&mut self, path: HostTrustPath, the_user_started_it: bool) -> bool {
        if !matches!(path, HostTrustPath::ItDiffers) {
            return false;
        }
        if the_user_started_it {
            return true;
        }
        if self.raised_in_this_repair {
            return false;
        }
        self.raised_in_this_repair = true;
        true
    }

    /// **Accepting** replaces the recorded value and the next attempt goes
    /// ahead. **Declining** leaves the record as it is and **the standing repair
    /// goes on failing at step 1**, the server reading *reconnecting* while a run
    /// is in flight and *disconnected* between runs.
    pub fn the_user_accepted(&mut self) {
        self.raised_in_this_repair = false;
    }
}

/// **Q7: the scan and the connection are two reads of the host key, and the
/// throwaway file is what closes the gap between them.**
///
/// Each connection attempt is given a throwaway known-hosts file holding **that
/// server's approved entry and nothing else**, with the user's own configuration
/// excluded, and the client is configured so that it can **neither ask nor
/// accept on its own**. So the connection **either presents Smoozy's credential
/// to the machine whose key the comparison approved, or it does not connect at
/// all**.
///
/// Where the file is written, that it is **never written into `~/.ssh` or into
/// any directory Smoozy scans for keys**, that it is discarded when the attempt
/// ends, and that any left behind by a process that exited first is **swept at
/// application start before any connection is attempted**, are all `Security and
/// licence` §7's and are **performed rather than re-decided**.
pub const THE_THROWAWAY_FILE_IS_SECURITY_AND_LICENCES: bool = true;

/// **Q10: what the promise does not catch, and it is observed rather than
/// supposed. Re-decided 2026-08-22 on corrected evidence, and it catches less
/// than this clause used to claim.**
///
/// **What this clause used to say, and the half of it that was wrong:** that
/// container images *carry host keys baked in at build time*, and that the
/// promise reaches a virtual machine, which *"generated its own host keys at
/// first boot"*. **Both halves of that mechanism are now disproved.** The keys a
/// container presents are **the host's**, not the image's. And a **virtual
/// machine** was found to be **shipped** its keys in the image — files dated
/// `2025-11-06 20:55:50` to the nanosecond, byte-identical after a recycle that
/// destroyed everything else on the box.
///
/// **Fact 83 is still answered negatively, and more widely than before.** A host
/// key identifies **neither the rental nor the machine** — on a container it
/// identifies the **host**, which outlives the rental and rents itself out
/// again; on the machine kind observed it identifies the **image**, which every
/// rental of that image shares.
///
/// **So a replacement under one address raises no changed-identity warning on
/// either kind**, and the promise that a change can be caught is **not carried by
/// the host key on any rented machine this project has seen.** **A changed key
/// still means something happened** — nothing here reads it as noise — **but an
/// unchanged key means nothing at all.**
///
/// **What covers it instead is the installation identity of §2.3**, which is why
/// nothing in this document is keyed on the host key. **That decision is
/// vindicated rather than merely unaffected**: it was written against the
/// container case alone and it holds against both.
pub fn an_unchanged_host_key_means_the_machine_is_the_same() -> bool {
    false
}

/// **Q13: the not-Linux check has three outcomes and they are not two.**
///
/// **Q12: how Smoozy tells — one reading over the command channel of the
/// pre-package surface, asking the machine to name its own operating system,
/// taken as a value.** The value is compared against **the one value that means
/// Linux**; **nothing is parsed for a cause**, and `Server protocol and
/// transfers` §1.3's rule that no tool's own output is mined for a cause is kept,
/// because **this is a value being read and not a failure being diagnosed**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotLinuxOutcome {
    /// **1. The machine named the Linux value.** The sequence carries on.
    ItNamedLinux,
    /// **2. The machine named something else.** Smoozy says plainly that it is
    /// not a Linux machine and leaves the server **disconnected**. The sequence
    /// is abandoned at this step, so **S10 item 5 ends the period the act
    /// began** and no repair runs under it. **The server stays in the list like
    /// any other, to be edited or removed.** This is Row 35's promise.
    ItNamedSomethingElse,
    /// **3. The machine named nothing Smoozy could read** — no answer, a
    /// refused command, an unreadable one. Smoozy says the first connection did
    /// not complete and what it knows, leaves the server disconnected, and
    /// **claims nothing about what the machine is**. That is the whole-app rule
    /// kept: **Smoozy cannot tell three causes apart, so it never claims to know
    /// which it is.**
    NothingSmoozyCouldRead,
}

impl NotLinuxOutcome {
    pub const ALL: [NotLinuxOutcome; 3] = [
        NotLinuxOutcome::ItNamedLinux,
        NotLinuxOutcome::ItNamedSomethingElse,
        NotLinuxOutcome::NothingSmoozyCouldRead,
    ];

    /// Both failing outcomes abandon the sequence, so the opening act's
    /// connection never becomes live.
    pub fn ends_the_period(self) -> Option<PeriodEnding> {
        match self {
            NotLinuxOutcome::ItNamedLinux => None,
            _ => Some(PeriodEnding::TheOpeningConnectionNeverBecameLive),
        }
    }

    /// **Only outcome 2 says what the machine is.**
    pub fn claims_what_the_machine_is(self) -> bool {
        matches!(self, NotLinuxOutcome::ItNamedSomethingElse)
    }
}

/// **Q12: the one value that means Linux**, compared as a value.
pub const THE_LINUX_VALUE: &str = "Linux";

pub fn read_the_operating_system(named: Option<&str>) -> NotLinuxOutcome {
    match named {
        None => NotLinuxOutcome::NothingSmoozyCouldRead,
        Some(value) if value == THE_LINUX_VALUE => NotLinuxOutcome::ItNamedLinux,
        Some(_) => NotLinuxOutcome::ItNamedSomethingElse,
    }
}

/// **Q14: the three outcomes are what make Row 35 keepable while Fact 59 is
/// unanswered.** The promise is that Smoozy says so plainly and refuses once
/// rather than letting every later action fail on its own, **and outcome 2 keeps
/// it wherever a machine answers at all**. What Fact 59 would settle is **which
/// machines land in outcome 2 rather than outcome 3**.
///
/// **§16 item 1: permanently unresolved, 2026-09-02. There is no standing
/// Windows batch.** The Creator ruled on 2026-08-31 that there will be **no
/// further rented machine for answering a question**, and this reading needs a
/// Windows machine with the in-box server feature enabled, which no computer on
/// this project is and **none is coming**. **This is not a pending item, and no
/// reader may leave a space in the code or the record for the answer.**
///
/// **Q15: the SSH version banner is not used, and that is now an observation
/// rather than a caution.** **Observed on the Creator's own Mac**, macOS 26.5.2,
/// 2026-08-19: the banner is `SSH-2.0-OpenSSH_10.2`, carrying **no
/// operating-system marker of any kind**, while a rented Ubuntu box announced
/// `SSH-2.0-OpenSSH_8.9p1 Ubuntu-3ubuntu0.16`. **So a bare banner is what a
/// macOS machine gives and is also what a Linux build with no vendor suffix
/// would give, and the banner cannot separate those two.**
///
/// **That narrow claim is all this observation supports.** Microsoft's own fork
/// puts an operating-system marker in the version macro itself — `version.h`:
/// `#define SSH_WINDOWS_VERSION "OpenSSH_for_Windows_10.0"` — and RFC 4253 §4.2
/// puts the identification string on the wire before key exchange and so before
/// any authentication. **So the banner does not separate macOS from Linux, and
/// it does separate Windows from both. Whether to use it for that is a design
/// change** and is carried at §16 item 1, **which takes no decision here**.
pub fn the_ssh_version_banner_is_used_for_the_check() -> bool {
    false
}

/// **Q18: step 7's queue reading is also the start of `Desktop app and delivery`
/// P10d's live-reading sequence for that server.**
///
/// Within the connected stretch that sequence repeats the queue reading, the
/// journal read-forward and `Queue & History` E8's commentary read, **on that
/// clause's own bounds**; it ends at the break declaration and with the period,
/// and a later connection's own step 7 starts a fresh one. **It repeats nothing
/// else in Q1's table**: steps 1 to 6 run on their own *Runs on* column, and
/// **the adoption offer is composed once at step 7 and never recomposed by a
/// pass.**
pub const THE_LIVE_READING_SEQUENCE_STARTS_AT_STEP_SEVEN: Step =
    Step::ReconciliationTheQueueReadingAndTheOffer;

/// **Q19: where step 5 takes §2.3's fourth branch — *present, and the supervisor
/// did not answer* — that step alone is re-taken on the live connection until it
/// settles, and the sequence carries on from step 6 when it does.**
///
/// §7 step 1 stops there, H18 takes no reading on that branch, and step 7 never
/// runs — **so nothing composes a request and nothing else in this document or
/// any other re-runs the read**. A reconnection that arrives before the machine's
/// supervisor is back is **the ordinary case rather than a corner**: the
/// container hook runs twelve seconds after the first request and the systemd
/// unit is back at 6.71 seconds, against an attempt every fifteen seconds.
/// **Without this clause the server reads *connected* and the queue goes on
/// showing a reading taken before the machine went, for the rest of the run.**
///
/// - **What is re-taken, and it is one step.** The identity read of step 5, on
///   the primary connection that is already live. **Steps 1 to 3 are per attempt
///   and this is no new attempt**, step 4 is the first connection's, and steps 6
///   and 7 have not run.
/// - **The cadence is the two figures this document already derives** — fifteen
///   seconds for the first eight reads and sixty after that, measured from the
///   **end** of the previous read. **No figure is added and no run fails**: this
///   sequence raises no record, withdraws none, and says nothing of its own.
/// - **It is not a second timer.** A declared event starts it, it is bounded by
///   the connection and the period it belongs to, and it fires nothing with
///   nothing under way. It is run-held and **writes nothing durable**.
/// - **It concludes nothing from failing.** Where the branch never settles — a
///   username edit is the case — **the condition simply goes on standing**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheFourthBranchRetake {
    pub reads_taken: u32,
}

impl TheFourthBranchRetake {
    pub fn begun() -> TheFourthBranchRetake {
        TheFourthBranchRetake { reads_taken: 0 }
    }

    /// The pause before the next read, **measured from the end of the previous
    /// one**. **No figure is added**: these are the repair's own two.
    pub fn pause_before_the_next_read(&self) -> std::time::Duration {
        if self.reads_taken < crate::repair::ATTEMPTS_IN_A_RUN {
            crate::repair::PAUSE_BETWEEN_ATTEMPTS
        } else {
            crate::repair::STANDING_PAUSE
        }
    }

    /// **What ends it, and the list is closed**: the read settling on one of
    /// §2.3's other three branches, the break declaration, and the period
    /// ending. **A later connection's own step 5 starts a fresh one.**
    pub fn settled(&self, check: InstallationCheck) -> bool {
        !matches!(
            check,
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer
        )
    }

    /// **It raises no record and withdraws none.**
    pub fn raises_a_record(&self) -> bool {
        false
    }
}

/// **Q11: the not-Linux check runs on the first connection to a server and
/// before anything else acts on the machine** — ahead of the installation
/// identity read, ahead of the hardware reading, ahead of any untracked-work
/// read, and ahead of whatever a ticked setup option would do.
pub fn runs_before(step: Step) -> bool {
    Step::IN_ORDER
        .iter()
        .position(|other| *other == step)
        .map(|position| {
            position
                > Step::IN_ORDER
                    .iter()
                    .position(|other| *other == Step::TheNotLinuxCheck)
                    .expect("the check is in the sequence")
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sequence_is_eight_steps_in_order() {
        assert_eq!(Step::IN_ORDER.len(), 8);
        assert_eq!(Step::IN_ORDER[0], Step::HostTrust);
        assert_eq!(Step::IN_ORDER[7], Step::ARememberedSetup);
    }

    #[test]
    fn the_not_linux_check_runs_before_everything_that_acts_on_the_machine() {
        for later in [
            Step::TheInstallationIdentity,
            Step::TheHardwareReading,
            Step::ReconciliationTheQueueReadingAndTheOffer,
            Step::ARememberedSetup,
        ] {
            assert!(runs_before(later));
        }
        assert!(!runs_before(Step::Authentication));
    }

    #[test]
    fn the_additional_connection_runs_the_first_three_steps_and_no_others() {
        let on_it: Vec<_> = Step::IN_ORDER
            .into_iter()
            .filter(|step| step.runs_on_the_additional_connection())
            .collect();
        assert_eq!(
            on_it,
            vec![Step::HostTrust, Step::TheKeyCheck, Step::Authentication]
        );
    }

    #[test]
    fn nothing_remembered_trusts_silently_and_a_match_says_nothing() {
        assert_eq!(compare(None, b"ssh-ed25519 AAAA"), HostTrustPath::NothingRemembered);
        let remembered = RememberedHostKey::approved(b"ssh-ed25519 AAAA".to_vec());
        assert_eq!(
            compare(Some(&remembered), b"ssh-ed25519 AAAA"),
            HostTrustPath::ItMatches
        );
        assert_eq!(
            compare(Some(&remembered), b"ssh-ed25519 BBBB"),
            HostTrustPath::ItDiffers
        );
    }

    #[test]
    fn a_repair_raises_the_changed_identity_warning_once_and_not_every_attempt() {
        // Q17: the next attempt is Smoozy's, fifteen seconds later, for as long
        // as the app is open.
        let mut warning = ChangedIdentityWarning::none_yet();
        assert!(warning.should_be_raised(HostTrustPath::ItDiffers, false));
        for _ in 0..8 {
            assert!(!warning.should_be_raised(HostTrustPath::ItDiffers, false));
        }
    }

    #[test]
    fn a_connect_the_user_started_raises_it_every_time() {
        let mut warning = ChangedIdentityWarning::none_yet();
        assert!(warning.should_be_raised(HostTrustPath::ItDiffers, true));
        assert!(warning.should_be_raised(HostTrustPath::ItDiffers, true));
    }

    #[test]
    fn a_matching_identity_raises_nothing() {
        let mut warning = ChangedIdentityWarning::none_yet();
        assert!(!warning.should_be_raised(HostTrustPath::ItMatches, true));
        assert!(!warning.should_be_raised(HostTrustPath::NothingRemembered, true));
    }

    #[test]
    fn the_not_linux_check_has_three_outcomes_and_only_one_claims_what_the_machine_is() {
        assert_eq!(NotLinuxOutcome::ALL.len(), 3);
        assert!(NotLinuxOutcome::ItNamedSomethingElse.claims_what_the_machine_is());
        assert!(!NotLinuxOutcome::NothingSmoozyCouldRead.claims_what_the_machine_is());
    }

    #[test]
    fn a_machine_that_answers_nothing_is_not_reported_as_not_linux() {
        assert_eq!(read_the_operating_system(None), NotLinuxOutcome::NothingSmoozyCouldRead);
        assert_eq!(read_the_operating_system(Some("Linux")), NotLinuxOutcome::ItNamedLinux);
        assert_eq!(
            read_the_operating_system(Some("Darwin")),
            NotLinuxOutcome::ItNamedSomethingElse
        );
    }

    #[test]
    fn both_failing_outcomes_end_the_period_at_s10_item_five() {
        assert_eq!(
            NotLinuxOutcome::ItNamedSomethingElse.ends_the_period(),
            Some(PeriodEnding::TheOpeningConnectionNeverBecameLive)
        );
        assert_eq!(
            NotLinuxOutcome::NothingSmoozyCouldRead.ends_the_period(),
            Some(PeriodEnding::TheOpeningConnectionNeverBecameLive)
        );
        assert_eq!(NotLinuxOutcome::ItNamedLinux.ends_the_period(), None);
    }

    #[test]
    fn the_banner_and_a_silent_scan_are_both_declined() {
        assert!(!the_ssh_version_banner_is_used_for_the_check());
        assert!(!no_answer_is_a_changed_identity());
    }

    #[test]
    fn an_unchanged_host_key_means_nothing_at_all() {
        // Fact 83, answered negatively and more widely than before.
        assert!(!an_unchanged_host_key_means_the_machine_is_the_same());
    }

    #[test]
    fn the_fourth_branch_retake_adds_no_figure_and_raises_no_record() {
        let mut retake = TheFourthBranchRetake::begun();
        assert_eq!(
            retake.pause_before_the_next_read(),
            crate::repair::PAUSE_BETWEEN_ATTEMPTS
        );
        retake.reads_taken = crate::repair::ATTEMPTS_IN_A_RUN;
        assert_eq!(
            retake.pause_before_the_next_read(),
            crate::repair::STANDING_PAUSE
        );
        assert!(!retake.raises_a_record());
        assert!(!retake.settled(InstallationCheck::PresentAndTheSupervisorDidNotAnswer));
        assert!(retake.settled(InstallationCheck::PresentAndEqualConfirmed));
    }
}

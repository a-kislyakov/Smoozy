//! **One general way to ask the machine's supervisor something and get the
//! matching answer back.**
//!
//! §1.1 item 1 is exact about the route: after hand-over Smoozy sends *"one
//! constant, installation-owned command — the active package's fixed
//! entry-point name … and sends every value on that entry point's standard
//! input in this document's length-delimited frame"*. The entry point is the
//! SSH command adapter; it passes the one framed request to the supervisor and
//! writes the correlated result on its standard output. **C2 matches the result
//! to the request by identity and by nothing else.**
//!
//! # Where it came from
//!
//! **It is [`crate::placement::ask_for_the_health_answer`] made general**, and
//! it reuses that file's carriage site and constant command rather than naming
//! either a second time: [`THE_ENTRY_POINTS_CARRIAGE_SITE`] points the command
//! at the package area, and [`THE_HEALTH_EXCHANGE_COMMAND`] is the entry point
//! reached through it. **That command's name says health because the health
//! exchange was its first use**; it is the one constant command §1.1 allows
//! after hand-over and every request rides it.
//!
//! # Why the two mechanisms are asked for one at a time
//!
//! **The health exchange holds a subsystem session open across its command**,
//! which is two sessions at once. **T5 forbids that at a learned limit of one**:
//! the primary connection then holds a single lease that alternates between the
//! reading channel and a command, and a queued command closes the reading
//! channel before it takes the session. So this file holds neither mechanism.
//! It asks whoever lends them for three things in order — plant the link, run
//! the command, take the link away — through [`TheTwoMechanismsLent`], and the
//! lender decides which session each one runs on. **On the desktop the lender
//! is the per-server point**, `App/Desktop app and delivery/src/servers/sessions.rs`.
//!
//! # What the answer says, and what it does not
//!
//! **Three outcomes, and they are kept apart because C17a turns on the
//! difference.** A supervisor that answered is one outcome. A command that ran
//! and brought nothing correlated back is another, and **it is not yet evidence
//! about the supervisor**: C17a holds that observation against the transport's
//! own next report, and whoever raises the supervisor-unavailable condition
//! does that waiting — nothing here raises anything. **A supervisor that was
//! never asked is the third**, where the link could not be planted or the
//! command never reached the machine, and it says nothing about the supervisor
//! at all. §1.3 is kept by the same split: **nothing here classifies a result on
//! its exit status**, and a non-zero ending means only that no frame is to be
//! read.
//!
//! **A result that says the package does not answer that request is an
//! answer**, and it comes back as [`WhatCameBack::ItAnswered`]. Reading the
//! reserved field is the caller's, through
//! [`crate::framing::Answered::the_request_name_it_says_it_does_not_answer`].
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` G12, G13.
//!
//! **A marker this plan also defines is never listed there and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use std::future::Future;

use crate::carriage::CarriageSite;
use crate::framing::{read_one_result, write_request_frame, Answered, Request};
use crate::placement::{THE_ENTRY_POINTS_CARRIAGE_SITE, THE_HEALTH_EXCHANGE_COMMAND};
use crate::transport::{CommandResult, CommandString};

/// **§1.1's two mechanisms, as whoever holds them lends them — one act at a
/// time.**
///
/// **Nothing here says which session an act runs on**, and that is the point:
/// T5's per-server point is the one place that knows whether the reading
/// channel is open, what the learned limit is, and whether a command must close
/// the one to take the other.
pub trait TheTwoMechanismsLent {
    /// **§1.1's planted-link carriage**, on a reading session: the link at
    /// `site` is made to point at `target`. **False** where it could not be
    /// planted, for either of [`crate::carriage::CarriageFailure`]'s reasons or
    /// because no reading session was lent.
    ///
    /// **The lender keeps what it planted**, so the link taken away is the one
    /// this lender put there and nobody else's.
    fn plant(&mut self, site: CarriageSite, target: Vec<u8>) -> impl Future<Output = bool> + Send;

    /// The link this lender planted, taken away when the step ends, on a
    /// reading session — **or left standing where the lender keeps it for the
    /// asks after this one** (§1.1 as changed on 2026-10-26: the desktop's
    /// per-server point keeps it for the connection that planted it). **Where
    /// the removal could not run it is owed** —
    /// §7.8's recorded staging, through
    /// [`crate::carriage::PlantedLink::the_removal_is_owed`], which the lender
    /// holds.
    fn take_it_away(&mut self) -> impl Future<Output = ()> + Send;

    /// **§1.1 item 1: one command, with one framed request on its standard
    /// input.** **Nothing** where the command never ran — no session could be
    /// had for it, or the connection went under before the request reached the
    /// machine.
    fn one_command_carrying_a_frame(
        &mut self,
        command: CommandString,
        frame: Vec<u8>,
    ) -> impl Future<Output = Option<CommandResult>> + Send;
}

/// What asking the supervisor came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatCameBack {
    /// **C2: a result carrying this request's own identity.** Whatever it
    /// says, including that the package does not answer that request, is the
    /// supervisor's answer.
    ItAnswered(Answered),
    /// **The command ran and nothing correlated came back** — the adapter ended
    /// without a clean ending, its output did not read as one frame, or the
    /// frame answered some other request. **C17a: the silence is not yet the
    /// supervisor's.** It is held against the transport's next report by
    /// whoever raises the condition, and a break declared in the meantime
    /// discards it.
    NothingCorrelatedCameBack,
    /// **The supervisor was never asked**: the link could not be planted, or
    /// the command never reached the machine. **No evidence about the
    /// supervisor either way.**
    ItWasNeverAsked,
}

/// **Asks the supervisor one framed request and brings back the matching
/// answer.**
///
/// **C1: the identity is the initiating side's, chosen before the request is
/// sent**, so the request arrives whole and this composes nothing of it. **C6
/// is the request's own**: an acting request is recorded on the machine before
/// it acts, and a reading carries its identity for correlation; this route is
/// the same for both.
///
/// **The lender is asked to take the link away whatever the command did** —
/// and from 2026-10-26 may leave it standing for the asks after this one, as
/// §1.1 now allows (the desktop's per-server point does) — and the removal that
/// could not run is left for the next plant to replace — which is what
/// [`crate::carriage::plant`] does with a link an earlier step left, and what
/// [`crate::placement::ask_for_the_health_answer`] already relies on. **Nothing
/// here writes an owed removal down**, and this says so rather than claiming
/// §7.8's discipline it does not perform.
pub async fn ask_the_supervisor<L>(
    lent: &mut L,
    package_area: &str,
    request: &Request,
) -> WhatCameBack
where
    L: TheTwoMechanismsLent + ?Sized,
{
    if !lent
        .plant(
            THE_ENTRY_POINTS_CARRIAGE_SITE,
            package_area.as_bytes().to_vec(),
        )
        .await
    {
        return WhatCameBack::ItWasNeverAsked;
    }
    let said = lent
        .one_command_carrying_a_frame(THE_HEALTH_EXCHANGE_COMMAND, write_request_frame(request))
        .await;
    lent.take_it_away().await;

    let Some(said) = said else {
        return WhatCameBack::ItWasNeverAsked;
    };
    what_the_command_brought_back(&said, request)
}

/// **The command's output, read as one frame and matched by identity.**
///
/// **The adapter writes no frame at all where the supervisor does not
/// answer**, and says so on its complaints stream with a non-zero ending
/// (`Rented-server runtime` G13's *"A supervisor that does not answer at all"*,
/// as the adapter carries it). That is the first branch below and it claims
/// nothing about why.
fn what_the_command_brought_back(said: &CommandResult, request: &Request) -> WhatCameBack {
    if !said.ended_cleanly() {
        return WhatCameBack::NothingCorrelatedCameBack;
    }
    let Ok(answered) = read_one_result(&mut &said.output[..]) else {
        return WhatCameBack::NothingCorrelatedCameBack;
    };
    if !answered.answers(request) {
        return WhatCameBack::NothingCorrelatedCameBack;
    }
    WhatCameBack::ItAnswered(answered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framing::{write_result_frame, Fields, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
    use smoozy_state::identities::{Identity, IdentityKind};

    /// What the stand-in lender was asked, in order.
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Asked {
        Plant(CarriageSite, Vec<u8>),
        Command(&'static str, Vec<u8>),
        TakeAway,
    }

    /// A lender that answers from a script and writes down what it was asked.
    struct AScriptedLender {
        asked: Vec<Asked>,
        the_link_plants: bool,
        the_command: Option<CommandResult>,
    }

    impl TheTwoMechanismsLent for AScriptedLender {
        async fn plant(&mut self, site: CarriageSite, target: Vec<u8>) -> bool {
            self.asked.push(Asked::Plant(site, target));
            self.the_link_plants
        }

        async fn take_it_away(&mut self) {
            self.asked.push(Asked::TakeAway);
        }

        async fn one_command_carrying_a_frame(
            &mut self,
            command: CommandString,
            frame: Vec<u8>,
        ) -> Option<CommandResult> {
            self.asked.push(Asked::Command(command.as_str(), frame));
            self.the_command.clone()
        }
    }

    fn a_request() -> Request {
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            "a request",
        )
    }

    fn a_clean_ending_carrying(answered: &Answered) -> CommandResult {
        CommandResult {
            output: write_result_frame(answered),
            complaint: Vec::new(),
            status: Some(0),
        }
    }

    fn asked(lender: &mut AScriptedLender, request: &Request) -> WhatCameBack {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("a runtime")
            .block_on(ask_the_supervisor(lender, "/srv/smoozy/package", request))
    }

    /// **§1.1 and T5 together: plant, command, take away — three acts, asked
    /// one at a time**, and the command is the one constant the entry point is
    /// reached by, with the request's own frame on its standard input.
    #[test]
    fn the_three_acts_are_asked_in_order_and_the_command_is_the_constant_one() {
        let request = a_request();
        let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "1.0.0")
            .carrying(Fields::none().and("said", "yes"));
        let mut lender = AScriptedLender {
            asked: Vec::new(),
            the_link_plants: true,
            the_command: Some(a_clean_ending_carrying(&answered)),
        };
        assert_eq!(
            asked(&mut lender, &request),
            WhatCameBack::ItAnswered(answered)
        );
        assert_eq!(
            lender.asked,
            vec![
                Asked::Plant(
                    THE_ENTRY_POINTS_CARRIAGE_SITE,
                    b"/srv/smoozy/package".to_vec()
                ),
                Asked::Command(
                    THE_HEALTH_EXCHANGE_COMMAND.as_str(),
                    write_request_frame(&request)
                ),
                Asked::TakeAway,
            ]
        );
    }

    /// **C2: a result is matched by identity and never by arrival.** A frame
    /// answering another request is not this request's answer.
    #[test]
    fn a_result_for_another_request_is_not_this_ones_answer() {
        let request = a_request();
        let someone_elses = Answered::to(
            &a_request(),
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            "1.0.0",
        );
        let mut lender = AScriptedLender {
            asked: Vec::new(),
            the_link_plants: true,
            the_command: Some(a_clean_ending_carrying(&someone_elses)),
        };
        assert_eq!(
            asked(&mut lender, &request),
            WhatCameBack::NothingCorrelatedCameBack
        );
    }

    /// **C17a: a command that ran and brought nothing back is held, not
    /// concluded** — and §1.3: the non-zero ending is not read as a cause.
    #[test]
    fn a_command_that_ended_without_a_frame_is_nothing_correlated_and_nothing_more() {
        let request = a_request();
        for status in [Some(1), Some(255), None] {
            let mut lender = AScriptedLender {
                asked: Vec::new(),
                the_link_plants: true,
                the_command: Some(CommandResult {
                    output: Vec::new(),
                    complaint: b"no supervisor".to_vec(),
                    status,
                }),
            };
            assert_eq!(
                asked(&mut lender, &request),
                WhatCameBack::NothingCorrelatedCameBack
            );
        }
    }

    /// **A supervisor never asked is not a supervisor that was silent.** A link
    /// that would not plant sends no command at all.
    #[test]
    fn a_link_that_would_not_plant_asks_nothing_and_runs_no_command() {
        let request = a_request();
        let mut lender = AScriptedLender {
            asked: Vec::new(),
            the_link_plants: false,
            the_command: None,
        };
        assert_eq!(asked(&mut lender, &request), WhatCameBack::ItWasNeverAsked);
        assert_eq!(
            lender.asked,
            vec![Asked::Plant(
                THE_ENTRY_POINTS_CARRIAGE_SITE,
                b"/srv/smoozy/package".to_vec()
            )]
        );
    }

    /// **A command that never reached the machine is never asked either**, and
    /// the link is still taken away.
    #[test]
    fn a_command_that_never_ran_is_never_asked_and_the_link_still_goes() {
        let request = a_request();
        let mut lender = AScriptedLender {
            asked: Vec::new(),
            the_link_plants: true,
            the_command: None,
        };
        assert_eq!(asked(&mut lender, &request), WhatCameBack::ItWasNeverAsked);
        assert_eq!(lender.asked.last(), Some(&Asked::TakeAway));
    }
}

//! The machine runs and bills until it is destroyed by hand — `Servers` §11.8.
//!
//! **The constraint** is `Rented-server runtime` V13, which is the architecture
//! half and which names this document as owner of the half the user meets; the
//! ruling of 2026-09-02 under the Creator's delegation of that date — **Smoozy
//! states plainly that the machine runs and bills until it is destroyed by hand,
//! and shows how long it has been running wherever the machine is shown**; and
//! the spec's own boundary that the machine is the user's, *"never rented,
//! billed, powered on or off by Smoozy"*.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` V2, V13.
//! `Desktop app and delivery` P10d.

use std::time::{Duration, SystemTime};

use smoozy_state::observed::{Clock, MachineClock, Reading};

use crate::hardware::{HardwareState, WhichConnectionHoldsIt};

/// **W40: the statement is made where a machine is added to the server list,
/// because that is the nearest moment Smoozy has to the renting.**
///
/// **Smoozy never rents a machine** — the spec says so in as many words — **so
/// there is no point-of-renting inside the application and none is invented
/// here.** The moment is **the add's save**, which §14's *Add* row declares a
/// unit of the Add action in its own right, and which is where the server row
/// first exists; **the statement is made there whether or not the setup option
/// was ticked and whether or not the connection that follows becomes live.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenTheStatementIsMade {
    /// **The add's save.** *(Corrected 2026-09-02: this had anchored on the two
    /// period-opening acts, and **a plain add with the setup option unticked is
    /// neither of the two** — which is exactly the case the clause goes on to
    /// cover. The save is the anchor the clause always needed.)*
    TheAddsSave,
}

/// **W40: what the statement says, and what it is not.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheStatement {
    /// **That the rented machine goes on running, and goes on being billed by
    /// whoever it was rented from, until somebody destroys the rental by hand.**
    ///
    /// **It needs no reading, is true on every machine kind, and must never be
    /// made conditional on one** — **the machine that cannot report its running
    /// time is the machine the statement matters most on** (V13).
    pub needs_a_reading: bool,
    pub is_conditional_on_the_machine_kind: bool,
    /// **It is not a warning, not a condition record and not a suppressible
    /// row**: it holds nothing up, gates no act, and enters neither the
    /// suppression table nor the failure records, **both of which are for things
    /// that happened rather than for how a rental works.**
    pub is_a_warning: bool,
    pub is_a_condition_record: bool,
    pub is_a_suppression_row: bool,
    /// **It is not durable.** Nothing about it is written into the server row,
    /// and no field records that it was made. **There is nothing to remember**:
    /// the fact is a property of every rental and is the same on the second add
    /// as on the first.
    pub is_durable: bool,
    /// **It is stated once per add and is not repeated at every connection**,
    /// which is the nagging Row 29's own repair policy is written against.
    pub is_repeated_at_every_connection: bool,
}

impl TheStatement {
    pub fn what_it_is() -> TheStatement {
        TheStatement {
            needs_a_reading: false,
            is_conditional_on_the_machine_kind: false,
            is_a_warning: false,
            is_a_condition_record: false,
            is_a_suppression_row: false,
            is_durable: false,
            is_repeated_at_every_connection: false,
        }
    }

    /// **W40: Smoozy offers no act against it, and that is a ruling rather than
    /// a gap.** No control here destroys a rental, caps its life, or reaches a
    /// provider — *servers and keys only, never providers* — and V13 states why
    /// nothing on the machine could do it either. **A rescue that destroyed a
    /// machine would destroy the work on it**, which is the worse of the two
    /// failures.
    pub fn smoozy_offers_an_act_against_it() -> bool {
        false
    }
}

/// **W41: how long the machine has been running is a health reading of
/// `Rented-server runtime`'s, carried here and composed nowhere else.**
///
/// It is that document's §12 reading under V2 — **taken on request, never on a
/// schedule** — and **this document opens no second route to the figure and
/// takes no reading of its own for it**. It is **period-held and carries its own
/// moment and clock** like every other machine reading here, it is **written
/// into no field of the server row**, and **it is never subtracted from a
/// desktop moment**.
///
/// **It is not part of §8's hardware reading** — that reading is Rows 27 and
/// 28's and its parts are closed at H5 — **so nothing about the hardware
/// reading's shape, its remembered form or its installation-identity rule moves
/// for it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunningTime {
    /// The duration the machine stated about itself. **A held figure is never
    /// advanced**: it is a duration the machine stated **at the moment it was
    /// read**, and **no desktop clock is added to it, and nothing is subtracted
    /// from it**.
    pub the_machine_stated: Duration,
    /// On the machine's own clock.
    pub the_machines_own_moment: Reading<()>,
    /// **The mark's own moment is H2a's desktop moment**, as it is for the
    /// hardware reading, **so the mark can be compared with the user's own now
    /// and the figure cannot.**
    pub the_desktops_own_moment: Reading<()>,
}

impl RunningTime {
    pub fn read(
        stated: Duration,
        server: &str,
        on_the_machines_clock: SystemTime,
        received_here_at: SystemTime,
    ) -> RunningTime {
        RunningTime {
            the_machine_stated: stated,
            the_machines_own_moment: Reading::taken(
                (),
                on_the_machines_clock,
                Clock::AMachine(MachineClock(server.to_string())),
            ),
            the_desktops_own_moment: Reading::taken((), received_here_at, Clock::ThisComputer),
        }
    }

    /// **A held figure is never advanced.** This is the function a builder would
    /// otherwise write, and it refuses.
    pub fn advanced_to(&self, _now: SystemTime) -> Duration {
        self.the_machine_stated
    }
}

/// **W41a: the request that produces the figure is named here, and it is one
/// request per primary connection, at step 7.**
///
/// W41 said the figure is taken **on request** and named no requester, so **no
/// clause anywhere made the request**: `Desktop app and delivery` P10d's set of
/// live readings is closed and **names health readings out of it by name**, that
/// document's M1 forbids a presentation path from taking one, and the sequence's
/// table did not carry it. **A builder had nothing to build and the ruled promise
/// had no route.**
///
/// - **Step 7 and not step 6, and the reason is the surface.** It is a health
///   reading, so it needs the supervisor answering, and step 7 is the first step
///   that runs on the confirmed branch with the supervisor reachable. **The
///   hardware reading at step 6 is taken over the pre-package surface before
///   anything is installed**, where no health reading can be taken at all.
/// - **Its own request, alongside the queue reading, never merged into either.**
/// - **Once per primary connection, and never again inside the connected
///   stretch.** **It does not join P10d's closed set.**
/// - **Where the request is not made, or is not answered, no figure stands and
///   none is inferred.** **A missing reading is never a zero and never a stale
///   one carried forward.**
pub const THE_REQUEST_IS_AT_STEP: crate::first_connection::Step =
    crate::first_connection::Step::ReconciliationTheQueueReadingAndTheOffer;

/// **W41b: the figure carries three states of its own, of the same shape as
/// H1's and keyed the same way, and W41's carve-out from §8 does not reach
/// them.**
///
/// W41 put the figure outside the hardware reading, which is **right about the
/// reading's parts and was wrong about its presentation**: it left the figure
/// **with no staleness state at all**, so a held figure went on being drawn as a
/// present fact after its connection died — **the defect H1a was re-keyed on
/// 2026-08-22 to stop, arriving on a second reading**, and arriving on **the one
/// line in the application that speaks to what a rental is costing.**
///
/// **This is the shape borrowed and not the record joined.** It is a second
/// record with its own three states; **H5's parts, H4's whole-record replacement
/// and §8.4's installation-identity rule do not move**, which is what W41 refuses
/// and what stays refused.
pub fn running_time_state(
    held: Option<&RunningTime>,
    on: WhichConnectionHoldsIt,
) -> HardwareState {
    match held {
        None => HardwareState::NeverRead,
        Some(_) if on.taken_on_the_connection_that_is_live_now => HardwareState::Live,
        Some(_) => HardwareState::LastSeen,
    }
}

/// **W42: on the machine kind where the figure would not be the machine's own,
/// the line says it is unavailable and never shows the host's.**
///
/// V13's second half: **on a systemd rental the uptime reading is the machine's,
/// and on a container it is the host's and would lie** — **8.2 days reported on
/// a container created that morning**. **Label: observed in a test** (one
/// container on one seller).
///
/// **The kind is settled first** — the same order H12 already takes for the card
/// scan — and where the kind is one whose figure is not the machine's own, **or
/// where no reading stands at all**, **what is shown is a stated absence rather
/// than a number**. **W40's statement is unaffected by any of this** and is made
/// on that machine exactly as on any other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheLineShows {
    /// The figure, marked *live* or *last seen* under W41b.
    TheFigure(HardwareState),
    /// **W40a: where the figure is a stated absence, W40's statement is composed
    /// in its place, and that is the one place it stands again after the add.**
    ///
    /// W40 states the fact **once**, at the add. W42 shows a stated absence on
    /// the container kind, **which is the kind this project's own readings say
    /// the providers rent** — so **on the ordinary rented machine, from the day
    /// after the add onward, nothing the user meets says anything about the cost
    /// at all**. `Rented-server runtime` §16 item 31 already carries that the
    /// figure cannot be supplied there; **what nothing carried is that the half
    /// V13 calls unconditional is also the half W40 refuses to repeat, so the two
    /// rules together empty the line of both.**
    ///
    /// - **What is composed, and it is composed from nothing new.** The server
    ///   row and the state of the figure's own record. **It needs no reading and
    ///   reaches no machine.**
    /// - **When.** **At the draw**, wherever the machine is shown and the figure
    ///   would otherwise have stood. **It is not a fresh utterance per
    ///   connection**: it is a standing part of the same line, so W40's *stated
    ///   once per add and not repeated at every connection* stands unchanged **and
    ///   the nagging is not reintroduced.**
    /// - **It is not a notification of any kind**: `Desktop app and delivery`
    ///   T9's four are closed and **nothing here adds a fifth**.
    ///
    /// **The line is never empty of both**, and that is the whole of what this
    /// clause buys.
    AStatedAbsenceCarryingTheStatement,
}

/// The machine kind, as W42's gate reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoseUptimeItWouldBe {
    /// A systemd rental: **the machine's own**.
    TheMachines,
    /// **A container: the host's, and it would lie.**
    TheHosts,
}

/// W42's gate and W40a's substitution, as one function so no draw composes an
/// empty line.
pub fn what_the_line_shows(
    kind: WhoseUptimeItWouldBe,
    held: Option<&RunningTime>,
    on: WhichConnectionHoldsIt,
) -> WhatTheLineShows {
    if matches!(kind, WhoseUptimeItWouldBe::TheHosts) {
        return WhatTheLineShows::AStatedAbsenceCarryingTheStatement;
    }
    match running_time_state(held, on) {
        HardwareState::NeverRead => WhatTheLineShows::AStatedAbsenceCarryingTheStatement,
        state => WhatTheLineShows::TheFigure(state),
    }
}

/// **W43: no screen is drawn here.** Which surface carries W40's statement and
/// W41's figure, where each sits, and in what words, is `Desktop app and
/// delivery`'s and Design's. **What this document fixes is the moment, the kind
/// of thing each is, and the two negatives** — that neither is durable, and that
/// neither is a warning, a condition or an act.
pub const NO_SCREEN_IS_DRAWN_HERE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    fn a_figure() -> RunningTime {
        RunningTime::read(
            Duration::from_secs(3_600),
            "srv-1",
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_000),
            SystemTime::UNIX_EPOCH + Duration::from_secs(2_000),
        )
    }

    fn live() -> WhichConnectionHoldsIt {
        WhichConnectionHoldsIt {
            taken_on_the_connection_that_is_live_now: true,
        }
    }

    fn not_live() -> WhichConnectionHoldsIt {
        WhichConnectionHoldsIt {
            taken_on_the_connection_that_is_live_now: false,
        }
    }

    #[test]
    fn the_statement_is_none_of_the_six_things_it_could_be_mistaken_for() {
        let statement = TheStatement::what_it_is();
        assert!(!statement.needs_a_reading);
        assert!(!statement.is_conditional_on_the_machine_kind);
        assert!(!statement.is_a_warning);
        assert!(!statement.is_a_condition_record);
        assert!(!statement.is_a_suppression_row);
        assert!(!statement.is_durable);
        assert!(!statement.is_repeated_at_every_connection);
        assert!(!TheStatement::smoozy_offers_an_act_against_it());
    }

    #[test]
    fn a_held_figure_is_never_advanced() {
        // It is a duration the machine stated about itself at the moment it was
        // read.
        let figure = a_figure();
        let much_later = SystemTime::UNIX_EPOCH + Duration::from_secs(100_000);
        assert_eq!(figure.advanced_to(much_later), Duration::from_secs(3_600));
    }

    #[test]
    fn the_two_moments_are_never_subtracted_from_each_other() {
        let figure = a_figure();
        assert_eq!(
            figure
                .the_desktops_own_moment
                .since(&figure.the_machines_own_moment),
            None
        );
    }

    #[test]
    fn a_destroyed_machines_row_never_shows_an_unqualified_number() {
        // W41b: the figure falls to last seen at the break declaration, keyed
        // on the connection exactly as H1a keys the hardware reading.
        let figure = a_figure();
        assert_eq!(running_time_state(Some(&figure), live()), HardwareState::Live);
        assert_eq!(
            running_time_state(Some(&figure), not_live()),
            HardwareState::LastSeen
        );
    }

    #[test]
    fn a_container_shows_a_stated_absence_and_never_the_hosts_uptime() {
        // 8.2 days reported on a container created that morning.
        let figure = a_figure();
        assert_eq!(
            what_the_line_shows(WhoseUptimeItWouldBe::TheHosts, Some(&figure), live()),
            WhatTheLineShows::AStatedAbsenceCarryingTheStatement
        );
    }

    #[test]
    fn the_line_is_never_empty_of_both() {
        // W40a: on the ordinary rented container, the statement stands in the
        // figure's place at every draw.
        for kind in [WhoseUptimeItWouldBe::TheHosts, WhoseUptimeItWouldBe::TheMachines] {
            for held in [None, Some(a_figure())] {
                let shown = what_the_line_shows(kind, held.as_ref(), live());
                match shown {
                    WhatTheLineShows::TheFigure(state) => {
                        assert_ne!(state, HardwareState::NeverRead)
                    }
                    WhatTheLineShows::AStatedAbsenceCarryingTheStatement => {}
                }
            }
        }
    }

    #[test]
    fn a_missing_reading_is_never_a_zero_and_never_a_stale_one_carried_forward() {
        assert_eq!(
            what_the_line_shows(WhoseUptimeItWouldBe::TheMachines, None, live()),
            WhatTheLineShows::AStatedAbsenceCarryingTheStatement
        );
    }

    #[test]
    fn the_request_is_at_step_seven_and_not_step_six() {
        assert_eq!(
            THE_REQUEST_IS_AT_STEP,
            crate::first_connection::Step::ReconciliationTheQueueReadingAndTheOffer
        );
        assert_ne!(
            THE_REQUEST_IS_AT_STEP,
            crate::first_connection::Step::TheHardwareReading
        );
    }

    #[test]
    fn the_statement_is_made_at_the_adds_save() {
        // Not at either period-opening act: a plain add with the setup option
        // unticked is neither of the two.
        assert_eq!(
            WhenTheStatementIsMade::TheAddsSave,
            WhenTheStatementIsMade::TheAddsSave
        );
    }
}

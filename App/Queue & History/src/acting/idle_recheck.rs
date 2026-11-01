//! **§12 W4, W5 and W6 performed — the desktop's timer that schedules the
//! re-check, the fresh reading re-taken from the machine before speaking, the
//! mark's write, and the telling through `Desktop app and delivery`'s
//! notification kind.**
//!
//! # No promise rests on the desktop's clock
//!
//! **W4: the desktop's own timer only schedules the re-check, and the claim is
//! re-taken from the machine's readings at that moment.** So drift costs a
//! re-check and never a wrong statement. **The arithmetic is machine-clock
//! arithmetic** — [`crate::idle::how_long_quiet`] and
//! [`crate::idle::the_remaining_interval`] — and nothing here subtracts across
//! clocks, because `State, identity and recovery` D5's type refuses it.
//!
//! **Disconnected, nothing fires.** An idle machine Smoozy is not connected to
//! goes unremarked, which is the spec's own limit and not a gap.
//!
//! # It is not a second timer, and the admission is cited from this side
//!
//! **`Desktop app and delivery` P10e admits a third shape for this re-check
//! and names W4 as its one instance**, on three parts this clause already
//! states: the wake-up is scheduled from a durable fact already held, it
//! re-verifies against the machine before it speaks, and it dies with the
//! connected period. **Nothing of W4 moved when the admission was written**,
//! and the citation is here so a builder reading P10b alone finds the route
//! back. **No watcher, no poll and no second timer is added.**
//!
//! # What the mark bounds, and what it does not
//!
//! **W5 gates one act: speaking.** The mark is looked up before the telling
//! and written by it, and **nothing else reads it**. **W5a: the heading is a
//! composition over W1 to W3's readings and is redrawn like any other
//! reading** — [`crate::idle::the_heading`] takes no mark, so a heading cannot
//! be gated on it here either.
//!
//! # W6 is closed and moves nothing
//!
//! **Vast.ai's own billing page settles it**: the rate is hourly and the
//! charge is per second. **What it broke is the spec's stated reason for the
//! half hour and not one clause of the arrangement** — a machine doing nothing
//! costs money however it is billed. [`THE_BILLING_ANSWER_MOVES_NOTHING`]
//! carries it beside the code.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` T9, T14, T22, P10b,
//! P10e. `Rented-server runtime` U5, U6.
//!
//! **`Desktop app and delivery` P10a and `State, identity and recovery` D5 are
//! deliberately not on that list and are written out at every site instead**,
//! because this plan defines a clause of each of those names too.

use std::time::Duration;

use smoozy_state::identities::Identity;
use smoozy_state::observed::Reading;

use crate::idle::{
    how_long_quiet, is_idle, is_past_the_half_hour, may_speak, the_quiet_stretch,
    the_remaining_interval, ARecordedEvent, IdleSpokenMark,
};
use crate::line::JobRecord;

/// **W5's mark, in the local record, reached through one trait.**
///
/// **It is Smoozy's own note of Smoozy's own act** — like the event cursor,
/// not a copy of machine state — and it is subject-held on the server row.
pub trait KeepsTheIdleMarks {
    /// **A record that will not answer is an error and not an empty list.**
    /// Read as empty, it would tell the user again about a stretch already
    /// told about; read as an error, the re-check says nothing this time and
    /// is taken again. Changed 2026-10-03 (the 38's piece 1), when the trait
    /// got its first implementation outside this crate's tests.
    fn the_marks_for(&self, server: &Identity) -> Result<Vec<IdleSpokenMark>, String>;

    /// **Written by the act of speaking and by nothing else.**
    fn write_the_mark(&mut self, mark: &IdleSpokenMark) -> Result<(), String>;
}

/// **One reading of a machine, taken fresh at the moment of the re-check.**
///
/// **Every field is taken from the machine in one reading** (W3), so the
/// machine's own now and the events it retains are of one moment and one
/// clock. **Nothing here is remembered between re-checks**: W2's quiet moment
/// is read fresh, never held durably, because it is exactly an observed
/// machine state the machine still holds (`State, identity and recovery` §3
/// D4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFreshReading {
    /// The machine's own now, on its own clock.
    pub machine_now: Reading<()>,
    pub installation: Identity,
    pub a_render_operation_is_running: bool,
    pub records: Vec<JobRecord>,
    /// The events the machine's journal retains, with their moments.
    pub retained: Vec<ARecordedEvent>,
}

/// **The machine, on the far side of one trait, because it genuinely is.**
///
/// **`None` where no connection stands**, and that is W4's *disconnected,
/// nothing fires* rather than a failure to report.
pub trait ReadsTheMachineFresh {
    fn take_a_fresh_reading(&self, server: &Identity) -> Option<AFreshReading>;
}

/// **`Desktop app and delivery` T14's idle kind, composed here and fired
/// there.**
///
/// **This document supplies the reading and the half hour; the notification is
/// that document's** (T9). **Nothing here draws a screen or writes a word the
/// user reads.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheTelling {
    pub server: Identity,
    /// **On the machine's clock**, and the figure the entry states.
    pub quiet_for: Duration,
}

/// What the re-check did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheRecheckDid {
    /// **W4: disconnected, nothing fires.**
    NothingFiredBecauseTheMachineCouldNotBeRead,
    /// The machine is not idle, so there is nothing to say and nothing to
    /// schedule. **Untracked work counts as work.**
    ItIsNotIdle,
    /// **W2's floor could not be reached**: the machine retains no event at
    /// all, so there is no moment to date the stretch from and nothing is
    /// invented.
    ThereIsNoStretchToDate,
    /// **W5: already spoken for this stretch.** A machine that takes work and
    /// falls quiet again has a new opening event and speaks again.
    AlreadySpokenForThisStretch,
    /// **W4: short of the half hour, so the timer schedules the re-check** and
    /// the claim is re-taken from the machine then.
    ScheduleTheRecheckIn(Duration),
    /// **W3 and W5: past the half hour and not yet spoken for this stretch.**
    /// The mark was written by this act.
    ItWasToldAbout(TheTelling),
}

/// **W3, W4 and W5, performed as one pass, in the order the clauses fix.**
///
/// **The claim is re-taken from the machine before anything is said** — the
/// reading arrives at this call and is never one held from earlier. **A
/// connection that finds a machine which went quiet an hour ago finds it long
/// overdue and says so at once**, rather than starting the wait over, because
/// the quiet moment is the machine's and not the connection's.
///
/// **The mark is written before this returns and by this act alone.** A
/// telling reported without its mark written would speak twice on the next
/// pass.
pub fn the_recheck(
    machine: &dyn ReadsTheMachineFresh,
    marks: &mut dyn KeepsTheIdleMarks,
    server: &Identity,
) -> Result<WhatTheRecheckDid, String> {
    let Some(fresh) = machine.take_a_fresh_reading(server) else {
        return Ok(WhatTheRecheckDid::NothingFiredBecauseTheMachineCouldNotBeRead);
    };
    // **W1: connected is what the reading having arrived means**, and a
    // reading that arrived is a connection that stood at that moment.
    if !is_idle(true, fresh.a_render_operation_is_running, &fresh.records) {
        return Ok(WhatTheRecheckDid::ItIsNotIdle);
    }
    let Some(stretch) = the_quiet_stretch(&fresh.installation, &fresh.retained) else {
        return Ok(WhatTheRecheckDid::ThereIsNoStretchToDate);
    };
    let Some(quiet_for) = how_long_quiet(&fresh.machine_now, &stretch) else {
        // **`State, identity and recovery` D5's type refused the
        // subtraction**, which means the two readings
        // were not on one clock. Nothing is said and nothing is guessed.
        return Ok(WhatTheRecheckDid::ThereIsNoStretchToDate);
    };
    if !is_past_the_half_hour(quiet_for) {
        // **W4: the desktop's own timer only schedules**, and the remaining
        // interval is machine-clock arithmetic.
        let remaining = the_remaining_interval(quiet_for).unwrap_or(Duration::ZERO);
        return Ok(WhatTheRecheckDid::ScheduleTheRecheckIn(remaining));
    }
    if !may_speak(&marks.the_marks_for(server)?, server, &stretch) {
        return Ok(WhatTheRecheckDid::AlreadySpokenForThisStretch);
    }
    marks.write_the_mark(&IdleSpokenMark {
        server: server.clone(),
        stretch: stretch.key.clone(),
    })?;
    Ok(WhatTheRecheckDid::ItWasToldAbout(TheTelling {
        server: server.clone(),
        quiet_for,
    }))
}

/// **W5a, performed: the heading is a composition over the same fresh reading
/// and takes no mark.**
///
/// **A heading gated on the mark would state *not idle* about an idle
/// machine**, which is the one thing W1a promised the heading would carry.
pub fn the_heading_for(fresh: &AFreshReading) -> crate::idle::TheHeading {
    let idle = is_idle(true, fresh.a_render_operation_is_running, &fresh.records);
    let quiet_for = the_quiet_stretch(&fresh.installation, &fresh.retained)
        .and_then(|stretch| how_long_quiet(&fresh.machine_now, &stretch));
    crate::idle::the_heading(idle, quiet_for)
}

/// **W6, closed, and it moves nothing.**
///
/// *"You are charged the base active rental cost for every second your
/// instance is in the active/connected state"*, at a rate quoted in $/hr —
/// Vast.ai's own billing page, read 2026-08-20. **The rate is hourly, the
/// charge is per second, and it settles Vast.ai only.** What it broke is the
/// spec's stated *reason* for the half hour and not one clause of the
/// arrangement, so **nothing in this module is conditional on how a provider
/// bills.**
pub const THE_BILLING_ANSWER_MOVES_NOTHING: bool = true;

/// **W4, stated as a value: the re-check dies with the connected period.**
///
/// It is scheduled from a durable fact already held — W5's mark and W2's
/// event, both records rather than a countdown — and it re-verifies against
/// the machine before it speaks. **Nothing here survives the period, and
/// nothing here polls.**
pub const THE_RECHECK_DIES_WITH_THE_CONNECTED_PERIOD: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::SequenceKey;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::observed::{Clock, MachineClock};
    use std::time::SystemTime;

    fn a_server() -> Identity {
        Identity::issue(IdentityKind::LocalServer).expect("randomness")
    }

    fn an_installation() -> Identity {
        Identity::issue(IdentityKind::Installation).expect("randomness")
    }

    fn an_operation() -> Identity {
        Identity::issue(IdentityKind::Operation).expect("randomness")
    }

    fn machine() -> Clock {
        Clock::AMachine(MachineClock("server-7".to_string()))
    }

    fn at(seconds: u64) -> Reading<()> {
        Reading::taken(
            (),
            SystemTime::UNIX_EPOCH + Duration::from_secs(seconds),
            machine(),
        )
    }

    fn an_ending(seconds: u64) -> ARecordedEvent {
        ARecordedEvent {
            identity: an_operation(),
            at: at(seconds),
            left_the_line_without_startable_work: true,
        }
    }

    struct AMachineInHand(Option<AFreshReading>);

    impl ReadsTheMachineFresh for AMachineInHand {
        fn take_a_fresh_reading(&self, _server: &Identity) -> Option<AFreshReading> {
            self.0.clone()
        }
    }

    #[derive(Default)]
    struct MarksInHand(Vec<IdleSpokenMark>);

    impl KeepsTheIdleMarks for MarksInHand {
        fn the_marks_for(&self, server: &Identity) -> Result<Vec<IdleSpokenMark>, String> {
            Ok(self
                .0
                .iter()
                .filter(|mark| &mark.server == server)
                .cloned()
                .collect())
        }

        fn write_the_mark(&mut self, mark: &IdleSpokenMark) -> Result<(), String> {
            self.0.push(mark.clone());
            Ok(())
        }
    }

    fn a_reading(now: u64, events: Vec<ARecordedEvent>, records: Vec<JobRecord>) -> AFreshReading {
        AFreshReading {
            machine_now: at(now),
            installation: an_installation(),
            a_render_operation_is_running: false,
            records,
            retained: events,
        }
    }

    /// **W3 and W5: past the half hour it is told about once, and the mark is
    /// written by the act.**
    #[test]
    fn a_machine_quiet_past_the_half_hour_is_told_about_once() {
        let server = a_server();
        let machine = AMachineInHand(Some(a_reading(4000, vec![an_ending(0)], Vec::new())));
        let mut marks = MarksInHand::default();
        let told = the_recheck(&machine, &mut marks, &server).expect("checked");
        assert_eq!(
            told,
            WhatTheRecheckDid::ItWasToldAbout(TheTelling {
                server: server.clone(),
                quiet_for: Duration::from_secs(4000)
            })
        );
        assert_eq!(marks.0.len(), 1, "the mark is written by the act");
        // **W5: once per stretch, across restarts.**
        assert_eq!(
            the_recheck(&machine, &mut marks, &server).expect("checked"),
            WhatTheRecheckDid::AlreadySpokenForThisStretch
        );
        assert_eq!(marks.0.len(), 1);
    }

    /// **W3: a connection that finds a machine which went quiet an hour ago
    /// says so at once, rather than starting the wait over.**
    #[test]
    fn a_long_overdue_machine_is_told_about_at_once_and_nothing_is_scheduled() {
        let machine = AMachineInHand(Some(a_reading(3600, vec![an_ending(0)], Vec::new())));
        let mut marks = MarksInHand::default();
        let told = the_recheck(&machine, &mut marks, &a_server()).expect("checked");
        assert!(matches!(told, WhatTheRecheckDid::ItWasToldAbout(_)));
    }

    /// **W4: short of the half hour, the timer schedules the re-check, and
    /// the interval is machine-clock arithmetic.**
    #[test]
    fn short_of_the_half_hour_the_re_check_is_scheduled_and_nothing_is_said() {
        let machine = AMachineInHand(Some(a_reading(600, vec![an_ending(0)], Vec::new())));
        let mut marks = MarksInHand::default();
        assert_eq!(
            the_recheck(&machine, &mut marks, &a_server()).expect("checked"),
            WhatTheRecheckDid::ScheduleTheRecheckIn(Duration::from_secs(1200))
        );
        assert!(
            marks.0.is_empty(),
            "nothing was said, so nothing was marked"
        );
    }

    /// **W4: disconnected, nothing fires.**
    #[test]
    fn nothing_fires_where_the_machine_cannot_be_read() {
        let machine = AMachineInHand(None);
        let mut marks = MarksInHand::default();
        assert_eq!(
            the_recheck(&machine, &mut marks, &a_server()).expect("checked"),
            WhatTheRecheckDid::NothingFiredBecauseTheMachineCouldNotBeRead
        );
        assert!(marks.0.is_empty());
    }

    /// **W1: a machine with a job that can start is not idle**, tracked or
    /// not, and nothing is said about it.
    #[test]
    fn a_machine_with_startable_work_is_not_idle() {
        let mut waiting = JobRecord::submitted(
            Identity::issue(IdentityKind::Job).expect("randomness"),
            None,
            "/ephemeral/smoozy/job-records/x".to_string(),
            &[],
        );
        waiting.sequence_key = Some(SequenceKey::read_back(1));
        let machine = AMachineInHand(Some(a_reading(9000, vec![an_ending(0)], vec![waiting])));
        let mut marks = MarksInHand::default();
        assert_eq!(
            the_recheck(&machine, &mut marks, &a_server()).expect("checked"),
            WhatTheRecheckDid::ItIsNotIdle
        );
    }

    /// **W5a: the heading is redrawn from the readings and takes no mark** —
    /// a machine idle for six hours says so on its heading throughout, after
    /// it has been told about once.
    #[test]
    fn the_heading_keeps_saying_idle_after_the_telling_has_fired() {
        let server = a_server();
        let reading = a_reading(21600, vec![an_ending(0)], Vec::new());
        let machine = AMachineInHand(Some(reading.clone()));
        let mut marks = MarksInHand::default();
        the_recheck(&machine, &mut marks, &server).expect("checked");
        assert_eq!(
            the_heading_for(&reading),
            crate::idle::TheHeading::Idle {
                quiet_for: Duration::from_secs(21600)
            }
        );
        // And again, with the mark standing.
        assert_eq!(
            the_recheck(&machine, &mut marks, &server).expect("checked"),
            WhatTheRecheckDid::AlreadySpokenForThisStretch
        );
        assert_eq!(
            the_heading_for(&reading),
            crate::idle::TheHeading::Idle {
                quiet_for: Duration::from_secs(21600)
            }
        );
    }

    /// **W5: a machine that takes work and falls quiet again has a new
    /// opening event, a new stretch, and speaks again.**
    #[test]
    fn a_second_stretch_speaks_again() {
        let server = a_server();
        let first = an_ending(0);
        let mut marks = MarksInHand::default();
        let installation = an_installation();
        let one = AFreshReading {
            installation: installation.clone(),
            ..a_reading(4000, vec![first.clone()], Vec::new())
        };
        assert!(matches!(
            the_recheck(&AMachineInHand(Some(one)), &mut marks, &server).expect("checked"),
            WhatTheRecheckDid::ItWasToldAbout(_)
        ));
        let two = AFreshReading {
            installation,
            ..a_reading(9000, vec![first, an_ending(5000)], Vec::new())
        };
        assert!(matches!(
            the_recheck(&AMachineInHand(Some(two)), &mut marks, &server).expect("checked"),
            WhatTheRecheckDid::ItWasToldAbout(_)
        ));
        assert_eq!(marks.0.len(), 2, "one mark per stretch");
    }

    /// **W2's floor is not invented**: a machine retaining no event at all
    /// has no moment to date the stretch from, and nothing is said.
    #[test]
    fn a_machine_retaining_no_event_dates_no_stretch() {
        let machine = AMachineInHand(Some(a_reading(9000, Vec::new(), Vec::new())));
        let mut marks = MarksInHand::default();
        assert_eq!(
            the_recheck(&machine, &mut marks, &a_server()).expect("checked"),
            WhatTheRecheckDid::ThereIsNoStretchToDate
        );
        const _: () = assert!(THE_BILLING_ANSWER_MOVES_NOTHING);
        const _: () = assert!(THE_RECHECK_DIES_WITH_THE_CONNECTED_PERIOD);
    }

    /// **`State, identity and recovery` D5: two readings on different clocks
    /// yield no figure**, and nothing
    /// is said rather than a wrong number.
    #[test]
    fn a_now_on_another_clock_yields_no_figure_and_nothing_is_said() {
        let mut reading = a_reading(9000, vec![an_ending(0)], Vec::new());
        reading.machine_now = Reading::taken(
            (),
            SystemTime::UNIX_EPOCH + Duration::from_secs(9000),
            Clock::ThisComputer,
        );
        let machine = AMachineInHand(Some(reading));
        let mut marks = MarksInHand::default();
        assert_eq!(
            the_recheck(&machine, &mut marks, &a_server()).expect("checked"),
            WhatTheRecheckDid::ThereIsNoStretchToDate
        );
    }

    /// The stretch's key is what the mark carries, and it is stable across a
    /// restart because it is the installation identity beside the opening
    /// event's.
    #[test]
    fn the_mark_carries_the_stretchs_own_key() {
        let installation = an_installation();
        let opening = an_ending(0);
        let stretch =
            the_quiet_stretch(&installation, std::slice::from_ref(&opening)).expect("dated");
        assert_eq!(stretch.key.installation, installation);
        assert_eq!(stretch.key.opening_event, Some(opening.identity));
    }
}

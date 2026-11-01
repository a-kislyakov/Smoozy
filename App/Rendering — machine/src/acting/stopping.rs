//! **§3.2 and §3.3 performed — the seven-step stop sequence carried out on
//! the machine, and the caller's waiter with its deadline.**
//!
//! # Two halves that never meet in one process
//!
//! **The sequence is the machine's**: the supervisor writes the stop's own
//! command record, writes the desired state, validates the recorded pair,
//! issues the interrupt to the recorded render process, infers nothing from
//! delivery, records the outcome when the group has ended, and attributes the
//! ending at the issuance. [`perform_the_stop`] is that, in that order.
//!
//! **The waiter is the caller's**, wherever the caller is — the desk, or the
//! machine's own role worker where a phone asked. [`wait_for_the_stop`] is
//! that, and it holds nothing but a clock and a way to ask.
//!
//! # What is reused rather than written again
//!
//! **The seven steps are `Rented-server runtime` X8's** and their order is
//! that crate's own `StopStep::IN_ORDER`, walked here rather than restated.
//! **The interrupt is that crate's**
//! [`smoozy_runtime::acting::execution_group::send_the_interrupt_to`], and
//! **which member receives it is this document's**: K4 aims it at the
//! recorded render process — never the worker, never a shell, never anything
//! found by name, pattern or process search. **The validation is that
//! crate's** `RecordedPair::validates_against`, and **step 7's boundary is
//! its `whose_ending_it_is`**, which is the one boundary the supervisor holds
//! a record of its own about.
//!
//! # K4, and why the interrupt and never the kill
//!
//! `Job files and outputs` 10.7, cited whole: an interrupt aimed at the
//! instant the frame's own file appears finishes the write and leaves the
//! frame whole, while a polite terminate and an outright kill each leave a
//! short file at the frame's own final name — **and a short file sitting at
//! exactly the finished frame's name is *present*, which is what the
//! already-there test reads, so a killed job leaves a frame that is never
//! rendered.** For a video the difference is total: an interrupt leaves the
//! frames that were finished; the other two leave the file with nothing in
//! it at all.
//!
//! **So there is no kill anywhere in this module, at any point, including
//! after the deadline** (K10). A second stop is a fresh command with a fresh
//! identity, which is the user's to ask for.
//!
//! # K7 — the cleanup owed after a termination is none beyond the ending step
//!
//! The render writes straight to final names and grows in place: there is no
//! staging to sweep and no publication to unwind, and what a stop leaves on
//! disk is the frames, which are the user's. **Nothing in this module
//! removes a file.**
//!
//! # The waiter, and the one shape that hangs
//!
//! **K8: *not taking* is the stop command's own identity having no recorded
//! outcome when the caller's deadline expires, with nothing recorded saying
//! the group has ended.** Both halves are needed and
//! [`crate::stop::the_interrupt_did_not_take`] is the judgement.
//!
//! **The deadline is not optional and this module is written so that its
//! absence fails rather than hangs.** [`wait_for_the_stop`] takes its clock
//! as a seam and asks it before every question, so a waiter given a target
//! that never ends **comes back** at the deadline; a test drives it with a
//! clock that never moves and asserts it is refused rather than entered,
//! which is the fault this shape exists against. A loop with no deadline in
//! this position is the one thing in the document that would hang a caller
//! for ever with nothing recorded.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Rented-server runtime` X1, X6, X8, X12,
//! X13. `Server protocol and transfers` C4, C16, C17, C21. `State, identity
//! and recovery` D5.
//!
//! **`Rented-server runtime` X16 and `Server protocol and transfers` C8 are
//! written out at their sites rather than listed here**, each because this
//! plan defines a clause of the same name.

use std::time::{Duration, SystemTime};

use smoozy_runtime::acting::execution_group::send_the_interrupt_to;
use smoozy_runtime::execution_group::{
    whose_ending_it_is, RecordedPair, StopStep, ValidatesTheRecordedPair, WhoseEnding,
};

use crate::stop::{the_interrupt_did_not_take, WhatTheUserIsTold, WhatTheWaiterHolds};

/// The durable writes the stop's sequence makes, behind a trait — **and
/// every one of them is the supervisor's** (`Rented-server runtime` S1 makes
/// it the only writer of every durable store on the machine).
///
/// **A write that is refused ends the sequence where it stands**, naming what
/// stopped it, which is `State, identity and recovery` §9.3's ordering: the
/// desired state is durable **before** the group is looked at, so a
/// supervisor that dies between steps two and four leaves a stop
/// reconciliation will honour.
pub trait TheSupervisorsOwnWritesForAStop {
    /// **Step one**: the stop's own command record, written before anything
    /// (`Server protocol and transfers` C4). It carries the stop's own
    /// operation identity, naming the target's (`Server protocol and
    /// transfers` C21).
    fn write_the_stops_command_record(&mut self) -> Result<(), String>;

    /// **Step two**: the target's desired state, *should be stopped*,
    /// durable **before the group is looked at**.
    fn write_the_desired_state_should_be_stopped(&mut self) -> Result<(), String>;

    /// **Step six**: the recorded outcome, written when the group has ended,
    /// and the event journalled with it.
    fn record_the_outcome_and_journal_it(&mut self, outcome: WhatWasRecorded)
        -> Result<(), String>;
}

/// What step six wrote.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatWasRecorded {
    /// **K5, step 7**: an ending past the issuance.
    LeftStopped,
    /// **K5, step 7**: the worker ended on its own **before** step four's
    /// termination was issued, so the recorded outcome is that ending and
    /// not *left stopped*.
    ItsOwnEnding,
}

impl From<WhoseEnding> for WhatWasRecorded {
    fn from(whose: WhoseEnding) -> Self {
        match whose {
            WhoseEnding::ItsOwn => WhatWasRecorded::ItsOwnEnding,
            WhoseEnding::LeftStopped => WhatWasRecorded::LeftStopped,
        }
    }
}

/// What the sequence did, as it ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheStopDid {
    /// The sequence ran to its end and step six wrote this.
    ItRecorded(WhatWasRecorded),
    /// **Step three: a pair that does not validate ends the sequence** — the
    /// group is gone, the recorded outcome is written from what the operation
    /// record and the reaped status say, **and no signal is sent to
    /// anything.**
    ThePairDidNotValidateSoNothingWasSent,
    /// **Step four ran and the group had not ended when the sequence
    /// stopped looking.** Nothing infers completion from delivery (step
    /// five), so this is not a failure — it is the sequence saying the
    /// interrupt is out and the outcome is not in yet. **The caller's waiter
    /// is what decides what to say about it** (K8, K10).
    TheInterruptIsOutAndNoOutcomeIsInYet,
    /// A durable write was refused. **The sequence ends where it stands,
    /// naming what stopped it**, and nothing further is attempted.
    ADurableWriteWasRefused { at: StopStep, why: String },
    /// **The interrupt itself was refused by the machine.** Recorded rather
    /// than retried: `Rented-server runtime` X16 records that a stop
    /// reaching another login's process is refused, and under one login the
    /// case does not arise — so a refusal here is a machine fault and never
    /// a reason to reach for the kill.
    TheInterruptWasRefused { why: String },
}

/// What the sequence is handed about the target, all of it recorded before
/// this act begins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheTarget {
    /// **X1's pair, recorded at the moment the render was started** (L5).
    /// **The stop is aimed at this and at nothing found later** (`Rented-server
    /// runtime` X6, X12).
    pub the_recorded_render_process: RecordedPair,
    /// Whether the worker's ending had already been reaped when the sequence
    /// began — step 7's one boundary, the issuance.
    pub the_ending_was_reaped_before_the_issuance: bool,
    /// Whether the group has ended, as the supervisor's own records say.
    /// **Never a live validation the caller took** (K8).
    pub the_records_say_the_group_has_ended: bool,
}

/// **K5 and `Rented-server runtime` X8, performed: the seven steps and
/// nothing shorter.**
///
/// The steps are walked in `StopStep::IN_ORDER`, so a step added to that
/// crate's own sequence is a step this walk meets rather than one somebody
/// has to remember.
pub fn perform_the_stop(
    writes: &mut impl TheSupervisorsOwnWritesForAStop,
    kernel: &impl ValidatesTheRecordedPair,
    target: TheTarget,
) -> WhatTheStopDid {
    for step in StopStep::IN_ORDER {
        match step {
            StopStep::TheStopsOwnCommandRecordIsWritten => {
                if let Err(why) = writes.write_the_stops_command_record() {
                    return WhatTheStopDid::ADurableWriteWasRefused { at: step, why };
                }
            }
            StopStep::TheDesiredStateIsWrittenShouldBeStopped => {
                if let Err(why) = writes.write_the_desired_state_should_be_stopped() {
                    return WhatTheStopDid::ADurableWriteWasRefused { at: step, why };
                }
            }
            StopStep::ThePairIsValidated => {
                let what_is_there = match kernel
                    .what_is_at(target.the_recorded_render_process.identifier)
                {
                    Ok(what_is_there) => what_is_there,
                    // A reading that did not answer is not a validation, and
                    // a signal sent on one would be a signal at an
                    // identifier the machine may have handed out again.
                    Err(why) => return WhatTheStopDid::ADurableWriteWasRefused { at: step, why },
                };
                if target
                    .the_recorded_render_process
                    .validates_against(what_is_there)
                    .the_group_is_gone()
                {
                    return WhatTheStopDid::ThePairDidNotValidateSoNothingWasSent;
                }
            }
            StopStep::TheKindsOwnTerminationIsIssued => {
                // **K4: the interrupt, to the recorded render process.**
                if let Err(why) =
                    send_the_interrupt_to(target.the_recorded_render_process.identifier)
                {
                    return WhatTheStopDid::TheInterruptWasRefused { why };
                }
            }
            StopStep::NothingInfersCompletionFromDelivery => {
                // **Step five is an absence and is kept as one.** Nothing is
                // read here, nothing is waited on here, and delivery is
                // never taken for the group having ended — a check made two
                // seconds after sending the signal reported the program
                // still running and would have been wrong.
            }
            StopStep::TheOutcomeIsRecordedWhenTheGroupHasEnded => {
                if !target.the_records_say_the_group_has_ended {
                    return WhatTheStopDid::TheInterruptIsOutAndNoOutcomeIsInYet;
                }
            }
            StopStep::TheEndingIsAttributedAtTheIssuance => {
                let whose = whose_ending_it_is(target.the_ending_was_reaped_before_the_issuance);
                let recorded = WhatWasRecorded::from(whose);
                if let Err(why) = writes.record_the_outcome_and_journal_it(recorded) {
                    return WhatTheStopDid::ADurableWriteWasRefused { at: step, why };
                }
                return WhatTheStopDid::ItRecorded(recorded);
            }
        }
    }
    unreachable!("the seventh step returns")
}

/// **K7: the cleanup owed after a termination is none beyond the ending
/// step**, kept by there being no removal in this module.
pub const NOTHING_HERE_REMOVES_A_FILE: bool = true;

// ---------------------------------------------------------------------------
// The caller's waiter — K8, K9, K10
// ---------------------------------------------------------------------------

/// The waiter's clock, behind a trait.
///
/// **It is a seam so the deadline can be proved rather than argued.** A
/// waiter with a real clock and a target that never ends takes two minutes
/// to demonstrate; with this it takes none, and the case that matters — the
/// deadline removed — fails at the door instead of hanging a test runner.
pub trait TheWaitersClock {
    /// **`State, identity and recovery` D5**: the moment, on the caller's own
    /// clock. **Nothing here is measured against a machine's.**
    fn now(&mut self) -> SystemTime;
    /// Wait, before asking again.
    fn wait(&mut self, how_long: Duration);
}

/// How long the waiter leaves between two askings.
///
/// **A pace and not a judgement.** Nothing turns on it: the deadline is
/// K9's, the answer is `Server protocol and transfers` C8's, and a longer
/// or shorter pace changes only how
/// soon a stop that has landed is noticed.
pub const BETWEEN_TWO_ASKINGS: Duration = Duration::from_millis(250);

/// What the wait ended as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheWaitEnded {
    /// The stop's identity has a recorded outcome, or the records say the
    /// group has ended. **Nothing is said to the user.**
    TheStopTook,
    /// **K10: the deadline expired with the interrupt not taken.** What the
    /// user is told is the waiter's truth and nothing stronger.
    TheDeadlineExpired(WhatTheUserIsTold),
    /// **The deadline was zero or less, which is a waiter with no deadline
    /// at all.** It is refused at the door rather than entered: a waiter
    /// with no deadline in this position is the one loop in this document
    /// that would hold a caller for ever with nothing recorded.
    ThereWasNoDeadlineSoTheWaitWasRefused,
}

/// **K8, K9 and K10, performed.**
///
/// `ask` is `Server protocol and transfers` C8's answer about the stop
/// command's own identity — *"both halves reach the caller inside `Server
/// protocol and transfers` C8's answer
/// to that one identity, and neither is a second reading the desktop
/// takes."*
///
/// **Expiry changes no durable state** (`Server protocol and transfers` C17):
/// this function writes nothing, removes nothing, and re-signals nothing.
pub fn wait_for_the_stop(
    clock: &mut impl TheWaitersClock,
    deadline: Duration,
    mut ask: impl FnMut() -> WhatTheWaiterHolds,
) -> HowTheWaitEnded {
    if deadline.is_zero() {
        return HowTheWaitEnded::ThereWasNoDeadlineSoTheWaitWasRefused;
    }
    let began = clock.now();
    loop {
        let held = ask();
        if !the_interrupt_did_not_take(held) {
            return HowTheWaitEnded::TheStopTook;
        }
        let now = clock.now();
        // A clock that went backwards is a clock that moved, and the wait
        // ends rather than running on an arithmetic nobody can trust.
        let elapsed = now.duration_since(began).unwrap_or(deadline);
        if elapsed >= deadline {
            return HowTheWaitEnded::TheDeadlineExpired(
                crate::stop::what_the_user_is_told(held)
                    .expect("the interrupt did not take, so there is a sentence"),
            );
        }
        clock.wait(BETWEEN_TWO_ASKINGS.min(deadline - elapsed));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acting::launch::tests::AKernelThat;

    #[derive(Debug, Default)]
    struct WhatWasWritten {
        wrote: Vec<StopStep>,
        outcome: Option<WhatWasRecorded>,
        refuse_at: Option<StopStep>,
    }

    impl TheSupervisorsOwnWritesForAStop for WhatWasWritten {
        fn write_the_stops_command_record(&mut self) -> Result<(), String> {
            self.one(StopStep::TheStopsOwnCommandRecordIsWritten)
        }
        fn write_the_desired_state_should_be_stopped(&mut self) -> Result<(), String> {
            self.one(StopStep::TheDesiredStateIsWrittenShouldBeStopped)
        }
        fn record_the_outcome_and_journal_it(
            &mut self,
            outcome: WhatWasRecorded,
        ) -> Result<(), String> {
            self.one(StopStep::TheEndingIsAttributedAtTheIssuance)?;
            self.outcome = Some(outcome);
            Ok(())
        }
    }

    impl WhatWasWritten {
        fn one(&mut self, step: StopStep) -> Result<(), String> {
            if self.refuse_at == Some(step) {
                return Err("No space left on device".to_string());
            }
            self.wrote.push(step);
            Ok(())
        }
    }

    /// A render this process really started, so the interrupt is sent to a
    /// real process and the sequence's fourth step is run rather than
    /// reasoned about.
    fn a_render_of_our_own() -> std::process::Child {
        std::process::Command::new("/bin/sh")
            .args(["-c", "trap '' INT; sleep 20"])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("a process of our own")
    }

    fn a_target(child: &std::process::Child) -> TheTarget {
        TheTarget {
            the_recorded_render_process: RecordedPair {
                identifier: child.id(),
                start_time: 19186,
            },
            the_ending_was_reaped_before_the_issuance: false,
            the_records_say_the_group_has_ended: true,
        }
    }

    /// **The seven steps, in order, against a real process** — the interrupt
    /// delivered and the outcome recorded.
    #[test]
    fn the_sequence_takes_the_seven_steps_in_order_and_records_left_stopped() {
        let mut child = a_render_of_our_own();
        let mut written = WhatWasWritten::default();
        let did = perform_the_stop(&mut written, &AKernelThat(Some(19186)), a_target(&child));
        assert_eq!(
            did,
            WhatTheStopDid::ItRecorded(WhatWasRecorded::LeftStopped)
        );
        assert_eq!(
            written.wrote,
            vec![
                StopStep::TheStopsOwnCommandRecordIsWritten,
                StopStep::TheDesiredStateIsWrittenShouldBeStopped,
                StopStep::TheEndingIsAttributedAtTheIssuance,
            ]
        );
        assert_eq!(written.outcome, Some(WhatWasRecorded::LeftStopped));
        child.kill().ok();
        child.wait().ok();
    }

    /// **Step 7's boundary is the issuance**: an ending reaped before it is
    /// the render's own and not *left stopped*. Getting this wrong the other
    /// way would show an ordinary successful stop as failed against the
    /// spec's *marked as stopped*.
    #[test]
    fn an_ending_reaped_before_the_issuance_is_its_own_and_never_left_stopped() {
        let mut child = a_render_of_our_own();
        let mut written = WhatWasWritten::default();
        let mut target = a_target(&child);
        target.the_ending_was_reaped_before_the_issuance = true;
        let did = perform_the_stop(&mut written, &AKernelThat(Some(19186)), target);
        assert_eq!(
            did,
            WhatTheStopDid::ItRecorded(WhatWasRecorded::ItsOwnEnding)
        );
        child.kill().ok();
        child.wait().ok();
    }

    /// **Step three: a pair that does not validate ends the sequence with no
    /// signal sent to anything.** The fault is introduced by the kernel
    /// answering a different start time — an identifier handed out again —
    /// and the sequence must stop rather than signal a stranger.
    #[test]
    fn a_pair_that_does_not_validate_ends_the_sequence_and_nothing_is_sent() {
        let mut child = a_render_of_our_own();
        let mut written = WhatWasWritten::default();
        let did = perform_the_stop(
            &mut written,
            // Exists, does not match: a later, unrelated process.
            &AKernelThat(Some(99999)),
            a_target(&child),
        );
        assert_eq!(did, WhatTheStopDid::ThePairDidNotValidateSoNothingWasSent);
        assert_eq!(written.outcome, None);
        // The process is still running: nothing was sent to it.
        assert!(child.try_wait().expect("readable").is_none());
        child.kill().ok();
        child.wait().ok();
        // And a group that is gone ends the sequence the same way.
        let mut written = WhatWasWritten::default();
        assert_eq!(
            perform_the_stop(
                &mut written,
                &AKernelThat(None),
                TheTarget {
                    the_recorded_render_process: RecordedPair {
                        identifier: 1,
                        start_time: 19186
                    },
                    the_ending_was_reaped_before_the_issuance: false,
                    the_records_say_the_group_has_ended: true,
                }
            ),
            WhatTheStopDid::ThePairDidNotValidateSoNothingWasSent
        );
    }

    /// **The desired state is durable before the group is looked at**, so a
    /// refused write at step two stops the sequence before any signal.
    #[test]
    fn a_refused_durable_write_ends_the_sequence_where_it_stands_and_names_it() {
        let mut child = a_render_of_our_own();
        let mut written = WhatWasWritten {
            refuse_at: Some(StopStep::TheDesiredStateIsWrittenShouldBeStopped),
            ..WhatWasWritten::default()
        };
        let did = perform_the_stop(&mut written, &AKernelThat(Some(19186)), a_target(&child));
        assert_eq!(
            did,
            WhatTheStopDid::ADurableWriteWasRefused {
                at: StopStep::TheDesiredStateIsWrittenShouldBeStopped,
                why: "No space left on device".to_string()
            }
        );
        assert!(child.try_wait().expect("readable").is_none());
        child.kill().ok();
        child.wait().ok();
    }

    /// **Step six: the outcome is recorded when the group has ended**, and
    /// where it has not the sequence says the interrupt is out and no
    /// outcome is in — which is not a failure and is not *left stopped*.
    #[test]
    fn no_outcome_is_recorded_while_the_records_do_not_say_the_group_has_ended() {
        let mut child = a_render_of_our_own();
        let mut written = WhatWasWritten::default();
        let mut target = a_target(&child);
        target.the_records_say_the_group_has_ended = false;
        let did = perform_the_stop(&mut written, &AKernelThat(Some(19186)), target);
        assert_eq!(did, WhatTheStopDid::TheInterruptIsOutAndNoOutcomeIsInYet);
        assert_eq!(written.outcome, None);
        child.kill().ok();
        child.wait().ok();
    }

    /// **Nothing in this module reaches for the kill**, read off the source.
    ///
    /// The tests above end their own stand-in processes when they are done
    /// with them, which is a test tidying up after itself and not the stop
    /// path — the stop path is [`perform_the_stop`], and what it sends is
    /// the interrupt and nothing else.
    #[test]
    fn no_line_of_this_module_sends_anything_but_the_interrupt() {
        const _: () = assert!(NOTHING_HERE_REMOVES_A_FILE);
        let text = crate::markers::tests::every_source_of_this_crate()
            .into_iter()
            .find(|(path, _)| path.ends_with("stopping.rs"))
            .expect("this file is among the crate's sources")
            .1;
        for word in [
            ["SIG", "KILL"].concat(),
            ["SIG", "TERM"].concat(),
            ["remove_", "file"].concat(),
        ] {
            for line in text.lines() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("//") || trimmed.starts_with("///") {
                    continue;
                }
                assert!(!trimmed.contains(&word), "{word}: {line}");
            }
        }
    }

    #[derive(Debug)]
    struct AClockThat {
        now: SystemTime,
        by: Duration,
        waits: usize,
    }

    impl TheWaitersClock for AClockThat {
        fn now(&mut self) -> SystemTime {
            self.now
        }
        fn wait(&mut self, _how_long: Duration) {
            self.waits += 1;
            self.now += self.by;
        }
    }

    /// **A stop that lands is noticed and nothing is said.**
    #[test]
    fn a_stop_that_lands_ends_the_wait_with_nothing_said() {
        let mut clock = AClockThat {
            now: SystemTime::UNIX_EPOCH,
            by: Duration::from_millis(250),
            waits: 0,
        };
        let mut asked = 0;
        let ended = wait_for_the_stop(&mut clock, crate::stop::THE_WAITERS_DEADLINE, || {
            asked += 1;
            WhatTheWaiterHolds {
                the_stop_has_a_recorded_outcome: asked >= 3,
                the_records_say_the_group_has_ended: false,
            }
        });
        assert_eq!(ended, HowTheWaitEnded::TheStopTook);
        assert_eq!(asked, 3);
    }

    /// **The deadline is what makes this terminate**, and the proof is that
    /// a target that never ends comes back rather than running on. The clock
    /// advances by the waiter's own pace, so the number of askings is
    /// bounded by the deadline divided by that pace and nothing else.
    #[test]
    fn a_target_that_never_ends_comes_back_at_the_deadline_with_the_waiters_truth() {
        let mut clock = AClockThat {
            now: SystemTime::UNIX_EPOCH,
            by: BETWEEN_TWO_ASKINGS,
            waits: 0,
        };
        let mut asked = 0;
        let ended = wait_for_the_stop(&mut clock, crate::stop::THE_WAITERS_DEADLINE, || {
            asked += 1;
            WhatTheWaiterHolds {
                the_stop_has_a_recorded_outcome: false,
                the_records_say_the_group_has_ended: false,
            }
        });
        assert_eq!(
            ended,
            HowTheWaitEnded::TheDeadlineExpired(
                WhatTheUserIsTold::TheStopWasSentAndTheRenderHadNotEndedWhenSmoozyStoppedWaiting
            )
        );
        let most = (crate::stop::THE_WAITERS_DEADLINE.as_millis() / BETWEEN_TWO_ASKINGS.as_millis())
            as usize
            + 1;
        assert!(asked <= most, "{asked} askings");
    }

    /// **Removing the deadline fails rather than hangs**, which is this
    /// module's own shape and the one thing about it worth proving. A wait
    /// with no deadline is refused at the door and never entered, so a
    /// caller cannot be held for ever with nothing recorded.
    #[test]
    fn a_wait_with_no_deadline_is_refused_rather_than_entered() {
        let mut clock = AClockThat {
            now: SystemTime::UNIX_EPOCH,
            by: Duration::ZERO,
            waits: 0,
        };
        let mut asked = 0;
        let ended = wait_for_the_stop(&mut clock, Duration::ZERO, || {
            asked += 1;
            WhatTheWaiterHolds {
                the_stop_has_a_recorded_outcome: false,
                the_records_say_the_group_has_ended: false,
            }
        });
        assert_eq!(
            ended,
            HowTheWaitEnded::ThereWasNoDeadlineSoTheWaitWasRefused
        );
        assert_eq!(asked, 0, "not one asking, and no loop entered");
        assert_eq!(clock.waits, 0);
    }

    /// **A clock that goes backwards ends the wait rather than running on**
    /// — a duration nobody can trust is not a reason to hold a caller.
    #[test]
    fn a_clock_that_went_backwards_ends_the_wait() {
        struct GoesBackwards(SystemTime);
        impl TheWaitersClock for GoesBackwards {
            fn now(&mut self) -> SystemTime {
                let now = self.0;
                self.0 -= Duration::from_secs(1);
                now
            }
            fn wait(&mut self, _: Duration) {}
        }
        let mut clock = GoesBackwards(SystemTime::UNIX_EPOCH + Duration::from_secs(60));
        let ended = wait_for_the_stop(&mut clock, crate::stop::THE_WAITERS_DEADLINE, || {
            WhatTheWaiterHolds {
                the_stop_has_a_recorded_outcome: false,
                the_records_say_the_group_has_ended: false,
            }
        });
        assert!(matches!(ended, HowTheWaitEnded::TheDeadlineExpired(_)));
    }
}

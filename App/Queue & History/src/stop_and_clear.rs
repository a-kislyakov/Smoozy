//! Stop-and-clear, in that order, and the clear that could not be done —
//! `Queue & History` §4 L13–L17, and L19's group-scoped form.
//!
//! **Constraint:** Row 59 — stop and clear in that order, and the order is
//! the promise. `State, identity and recovery` §9.3: the pair is one unit
//! because the second is unsafe without the first; its §9.4 outcome record
//! holds the enumeration durably before the first unit is attempted; its D2
//! forbids a desired state inferred from an observed reading, which is what
//! rules out naming the stop's target from the desktop.
//!
//! **What is here**: the pair's composition, what the clear covers, how the
//! stop's subject is resolved, the refused-clear rule and the three results.
//! **What is the acting half's**: the two command records, their travel, the
//! serialized write, and the interrupt itself.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` S1.

use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::outcomes::{ActionOutcome, Recorded, UnitOutcome};


/// **L13: stop-and-clear is one request, two ordered halves.** The desktop
/// submits the clear and the stop as two commands with their own operation
/// identities, marked as one ordered pair. **The pair names no render
/// operation**: the stop carries the clear's operation identity as its pair
/// and no target, because the only identity the desktop holds is the one its
/// period-held reading last showed, and `State, identity and recovery` D2
/// forbids a desired state inferred from an observed reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePair {
    pub clear: Identity,
    pub stop: Identity,
    /// **L19: the group-scoped form**, where the request was cancel-group.
    pub scoped_to: Option<Identity>,
}

impl ThePair {
    /// Composed on the desktop from two freshly issued operation identities.
    /// **There is no field for a target**, which is how L13 is kept by the
    /// type rather than by care.
    pub fn composed(clear: Identity, stop: Identity, scoped_to: Option<Identity>) -> ThePair {
        debug_assert_eq!(clear.kind(), IdentityKind::Operation);
        debug_assert_eq!(stop.kind(), IdentityKind::Operation);
        debug_assert!(scoped_to
            .as_ref()
            .is_none_or(|group| group.kind() == IdentityKind::Group));
        ThePair {
            clear,
            stop,
            scoped_to,
        }
    }
}

/// **L13, L14 and L19's four rules — the running one, what the clear covers,
/// the stop's subject and its resolution — live in `Queue & History —
/// machine`'s `line`** from 2026-09-29, step 10c's first stop-and-clear piece, because the program a
/// rented machine runs is what applies them and it cannot reach this crate.
/// **Re-exported here at the path they had**, and their tests stay below.
pub use smoozy_queue_history_machine::line::{
    resolve_the_stops_subject, what_the_clear_covers, TheRunningOne, TheStopsSubject,
};

/// **L15: there are two windows, not one, and both are closed** — the first
/// by the single writer, the second by the stop having no target until the
/// clear's write. The running job ending on its own **between** the clear
/// and the stop: the clear's one write emptied the line entire, so nothing
/// is left for the admission point to take. The running job ending
/// **before** the clear's write while the pair is in flight, the admission
/// point starting the next: at the write, the operation the admission point
/// started is *the running one* — exempted by L14, resolved as the stop's
/// subject, and stopped. **Neither window rests on what the desktop saw.**
pub const BOTH_WINDOWS_ARE_CLOSED_AND_NEITHER_RESTS_ON_WHAT_THE_DESKTOP_SAW: bool = true;

/// The two halves' names, as the §9.4 outcome record enumerates them —
/// durable on the desktop before the first unit is attempted (L17).
pub const THE_CLEAR: &str = "the clear";
pub const THE_STOP: &str = "the stop";
pub const THE_ACTION: &str = "stop and clear";

/// **L16: where the clear's write is refused, the stop is not attempted, and
/// says so** — its recorded state is *not attempted*, naming the clear as
/// what stopped it. The observed refusal is the disk-full one, loud, nothing
/// moved behind it; the promise is kept by the ordering, not by that
/// accident.
pub fn the_stops_unit_where_the_clear_was_refused() -> UnitOutcome {
    UnitOutcome::NotAttempted {
        stopped_by: THE_CLEAR.to_string(),
    }
}

/// **L17: three results are distinguishable at the desktop**, from the two
/// halves' separate outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreeResults {
    /// Could not begin at all — the clear refused, the stop not attempted.
    CouldNotBeginAtAll,
    ClearedAndStopped,
    /// Cleared, and the stop failed or expired its waiter.
    ClearedAndTheStopFailedOrExpiredItsWaiter,
}

/// L17, read off the `State, identity and recovery` §9.4 outcome record.
/// `None` while either half's ending has not landed — an interruption loses
/// the saying and never the fact, and nothing here invents the missing half.
pub fn the_three_results(record: &Recorded) -> Option<ThreeResults> {
    let of = |unit: &str| {
        record
            .units
            .iter()
            .find(|(owed, _)| owed == unit)
            .and_then(|(_, outcome)| outcome.clone())
    };
    let clear = of(THE_CLEAR)?;
    let stop = of(THE_STOP)?;
    Some(match (clear, stop) {
        (UnitOutcome::Done, UnitOutcome::Done) => ThreeResults::ClearedAndStopped,
        (UnitOutcome::Done, _) => ThreeResults::ClearedAndTheStopFailedOrExpiredItsWaiter,
        (_, _) => ThreeResults::CouldNotBeginAtAll,
    })
}

/// The three results against the record's own derived outcome, so the two
/// readings cannot disagree: *could not begin* is the record's *nothing*,
/// *cleared and stopped* its *whole*, and the third its *partial*.
pub fn agrees_with_the_records_own_outcome(record: &Recorded) -> bool {
    matches!(
        (the_three_results(record), record.the_actions_outcome()),
        (
            Some(ThreeResults::CouldNotBeginAtAll),
            Some(ActionOutcome::Nothing)
        ) | (
            Some(ThreeResults::ClearedAndStopped),
            Some(ActionOutcome::Whole)
        ) | (
            Some(ThreeResults::ClearedAndTheStopFailedOrExpiredItsWaiter),
            Some(ActionOutcome::Partial),
        ) | (None, None)
    )
}

/// **L19: L15 does not scope and is not imported.** Its proof is that the
/// clear empties the line entire, and a group clear leaves non-members the
/// admission point can and should take — which is correct, because the user
/// asked for the group and not the machine.
pub const THE_GROUP_CLEAR_LEAVES_NON_MEMBERS_FOR_THE_ADMISSION_POINT: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::testing::{a_group, a_line_of, an_operation};
    use crate::line::QueueState;
    use smoozy_state::outcomes::{Keeps, Refused, Writing};

    /// A keeper that holds its records in a list — `State, identity and
    /// recovery` §9.4's rules with none of an engine, for the desk.
    #[derive(Default)]
    struct AList {
        records: Vec<Recorded>,
    }

    impl Keeps for AList {
        fn enumerate(&mut self, action: &str, units: &[&str]) -> Result<Identity, Refused> {
            let identity = smoozy_state::outcomes::issue()
                .map_err(|_| Refused::TheStore("no randomness".into()))?;
            self.records.push(Recorded {
                identity: identity.clone(),
                action: action.to_string(),
                units: units.iter().map(|unit| (unit.to_string(), None)).collect(),
            });
            Ok(identity)
        }

        fn a_unit_ended(
            &mut self,
            record: &Identity,
            unit: &str,
            outcome: UnitOutcome,
        ) -> Result<(), Refused> {
            let held = self
                .records
                .iter_mut()
                .find(|held| held.identity == *record)
                .ok_or_else(|| Refused::TheStore("no such record".into()))?;
            let entry = held
                .units
                .iter_mut()
                .find(|(owed, _)| owed == unit)
                .ok_or(Refused::ThatUnitWasNotOwed)?;
            if entry.1.is_some() {
                return Err(Refused::ThatUnitHasAlreadyEnded);
            }
            entry.1 = Some(outcome);
            Ok(())
        }

        fn read(&self, record: &Identity) -> Result<Option<Recorded>, Refused> {
            Ok(self
                .records
                .iter()
                .find(|held| held.identity == *record)
                .cloned())
        }

        fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
            self.records.retain(|held| held.identity != *record);
            Ok(())
        }
    }

    fn a_pair() -> ThePair {
        ThePair::composed(an_operation(), an_operation(), None)
    }

    #[test]
    fn the_pair_carries_two_operation_identities_and_no_target() {
        let pair = a_pair();
        assert_ne!(pair.clear, pair.stop);
        assert_eq!(pair.scoped_to, None);
    }

    #[test]
    fn the_clear_covers_every_waiting_and_held_record_and_exempts_the_running_one() {
        let mut records = a_line_of(3);
        let running_operation = an_operation();
        records[0].state = QueueState::Rendering {
            operation: running_operation.clone(),
        };
        records[2].state = QueueState::Held {
            hold: an_operation(),
        };
        let running = TheRunningOne {
            operation: running_operation,
            job: records[0].job.clone(),
        };
        let covered = what_the_clear_covers(&records, Some(&running), None);
        assert_eq!(covered.len(), 2);
        assert!(covered.iter().all(|record| record.job != running.job));
    }

    #[test]
    fn the_running_one_is_the_operation_at_the_write_and_never_what_the_desktop_last_showed() {
        // The desktop composed the pair while job A rendered; A ended, the
        // admission point took B. At the write, B is the running one.
        let records = a_line_of(2);
        let a = an_operation();
        let b = TheRunningOne {
            operation: an_operation(),
            job: records[1].job.clone(),
        };
        let subject = resolve_the_stops_subject(Some(&b), None, None, Some(&a));
        assert_eq!(
            subject,
            TheStopsSubject::Resolved {
                operation: b.operation.clone(),
                the_desktop_last_showed: Some(a)
            }
        );
        // And where they agree, the outcome names one.
        let same = resolve_the_stops_subject(Some(&b), None, None, Some(&b.operation));
        assert_eq!(
            same,
            TheStopsSubject::Resolved {
                operation: b.operation.clone(),
                the_desktop_last_showed: None
            }
        );
        assert_eq!(
            resolve_the_stops_subject(None, None, None, Some(&b.operation)),
            TheStopsSubject::NothingWasRendering
        );
    }

    /// **L14: the exemption is by the resolved running operation's identity
    /// and never by a record's state.**
    ///
    /// **The fault this catches is a record left reading *rendering* by an
    /// interruption.** Only one operation holds the exclusive render, so such
    /// a record has no worker; exempting it by state left a job behind on a
    /// machine the user asked to have emptied, which L14 says in as many words
    /// never happens. **The check is the branch itself**, so it holds in the
    /// build the user runs and not only in a test build.
    #[test]
    fn a_record_reading_rendering_that_is_not_the_resolved_one_is_covered() {
        let mut records = a_line_of(3);
        let running_operation = an_operation();
        records[0].state = QueueState::Rendering {
            operation: running_operation.clone(),
        };
        // Left *rendering* by an interruption, and holding no worker.
        records[1].state = QueueState::Rendering {
            operation: an_operation(),
        };
        let running = TheRunningOne {
            operation: running_operation,
            job: records[0].job.clone(),
        };
        let covered = what_the_clear_covers(&records, Some(&running), None);
        assert_eq!(covered.len(), 2);
        assert!(covered.iter().any(|record| record.job == records[1].job));
        assert!(covered.iter().all(|record| record.job != running.job));

        // **And with nothing resolved as running, every rendering record is
        // covered** — the machine is emptied entire.
        let covered = what_the_clear_covers(&records, None, None);
        assert_eq!(covered.len(), 3);
    }

    #[test]
    fn a_group_clear_covers_the_members_alone_and_stops_only_a_member() {
        let group = a_group();
        let mut records = a_line_of(4);
        records[1].group = Some(group.clone());
        records[2].group = Some(group.clone());
        let running = TheRunningOne {
            operation: an_operation(),
            job: records[0].job.clone(),
        };
        records[0].state = QueueState::Rendering {
            operation: running.operation.clone(),
        };
        let covered = what_the_clear_covers(&records, Some(&running), Some(&group));
        assert_eq!(covered.len(), 2);
        assert!(covered
            .iter()
            .all(|record| record.group.as_ref() == Some(&group)));
        // The running job is not a member: no stop is attempted, and it
        // says so.
        assert_eq!(
            resolve_the_stops_subject(Some(&running), None, Some(&group), None),
            TheStopsSubject::NotAttemptedTheRunningJobIsNotAMember {
                running: running.operation.clone()
            }
        );
        // A member running: stopped.
        assert!(matches!(
            resolve_the_stops_subject(Some(&running), Some(&group), Some(&group), None),
            TheStopsSubject::Resolved { .. }
        ));
    }

    #[test]
    fn a_refused_clear_leaves_the_stop_not_attempted_naming_the_clear() {
        let mut keeper = AList::default();
        let mut writing = Writing::open(&mut keeper, THE_ACTION, &[THE_CLEAR, THE_STOP]).unwrap();
        writing
            .a_unit_ended(THE_CLEAR, UnitOutcome::Failed)
            .unwrap();
        writing
            .a_unit_ended(THE_STOP, the_stops_unit_where_the_clear_was_refused())
            .unwrap();
        let record = writing.read().unwrap().unwrap();
        assert_eq!(
            the_three_results(&record),
            Some(ThreeResults::CouldNotBeginAtAll)
        );
        assert!(agrees_with_the_records_own_outcome(&record));
    }

    #[test]
    fn the_three_results_are_read_off_the_two_halves_and_agree_with_the_records_own_outcome() {
        let cases = [
            (
                UnitOutcome::Done,
                UnitOutcome::Done,
                ThreeResults::ClearedAndStopped,
            ),
            (
                UnitOutcome::Done,
                UnitOutcome::Failed,
                ThreeResults::ClearedAndTheStopFailedOrExpiredItsWaiter,
            ),
        ];
        for (clear, stop, expected) in cases {
            let mut keeper = AList::default();
            let mut writing =
                Writing::open(&mut keeper, THE_ACTION, &[THE_CLEAR, THE_STOP]).unwrap();
            writing.a_unit_ended(THE_CLEAR, clear).unwrap();
            writing.a_unit_ended(THE_STOP, stop).unwrap();
            let record = writing.read().unwrap().unwrap();
            assert_eq!(the_three_results(&record), Some(expected));
            assert!(agrees_with_the_records_own_outcome(&record));
        }
    }

    #[test]
    fn an_interrupted_pair_says_nothing_rather_than_inventing_the_missing_half() {
        let mut keeper = AList::default();
        let mut writing = Writing::open(&mut keeper, THE_ACTION, &[THE_CLEAR, THE_STOP]).unwrap();
        writing.a_unit_ended(THE_CLEAR, UnitOutcome::Done).unwrap();
        let record = writing.read().unwrap().unwrap();
        assert_eq!(the_three_results(&record), None);
        assert!(agrees_with_the_records_own_outcome(&record));
    }
}

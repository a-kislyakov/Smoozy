//! The atomic-or-partial boundary — `State, identity and recovery` §9.
//!
//! **P5's outcome record, which is the durable half of Row 292** — *do what
//! can be done, and say what can't*. Every action whose partition unit is
//! smaller than the action has exactly one, it sits in the local record
//! (§1.1), it is a durable subject in its own right (§8.7 F21), and it is
//! **user-held** (§11.1).
//!
//! **What is here is the rule; the table is `Desktop app and delivery` §5's.**
//! That is the split every member of this stage already draws: this document
//! owns what a record holds and what may be written into it, that one owns the
//! store. So nothing in this file opens a file, and the whole of §9's
//! judgement is testable at the desk.
//!
//! **The unit is named by the owning module and is opaque here.** §9.2 has
//! each feature document declare its own partition units, so this file never
//! knows what a unit is — only that it was owed, and how it ended. A module
//! whose units carry a rule about what a name may contain keeps that rule at
//! its own end: `Keys` §11 enumerates by role and forbids a path, a
//! fingerprint, a name or a comment in the record, and that is `Keys`' rule
//! about `Keys`' units.

use crate::identities::{Identity, IdentityKind};

/// **The identity an outcome record is kept under, and it is the action's own
/// operation identity** — §9.5 P7's own words: *"Trying again is a new action
/// with a new operation identity."*
///
/// **A purely local action holds it here rather than in a pending-command
/// record**, which is the one narrowing of §2.2's register row this needs:
/// that row names the caller's pending-command record and the machine's
/// operation record as where the identity is held durably, and an action that
/// sends no command to a machine has neither. Nothing else about the row
/// moves — it is still issued once, by the side that initiates the action,
/// before the action's first effect, and it is still never reused.
pub const KIND: IdentityKind = IdentityKind::Operation;

/// How one unit ended — **P5 item 2, and the list is closed at three.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitOutcome {
    Done,
    Failed,
    /// **§9.3 P4: a recorded state of its own, distinct from *failed*, and it
    /// names the unit that stopped it.** A unit reported as failed when it was
    /// never tried is not *say what you could not do*.
    NotAttempted { stopped_by: String },
}

/// **P5 item 3: the action's own outcome, derived from the list of unit
/// outcomes and never a separate durable field.** Two fields would drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionOutcome {
    /// Every unit done.
    Whole,
    /// Some units done and some not. This is the case Row 292 exists for.
    Partial,
    /// No unit done.
    Nothing,
}

/// What a written outcome record holds when it is read back.
///
/// **The three fields are P5's three items, less the third** — the action's
/// own outcome is [`Recorded::the_actions_outcome`] and is derived at every
/// reading rather than stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recorded {
    pub identity: Identity,
    /// Which action this is, in the owning module's own words.
    pub action: String,
    /// **The enumeration, in the order the units run**, each with its outcome
    /// where one has landed. An absent outcome is a unit that was owed and
    /// whose ending has not been written.
    pub units: Vec<(String, Option<UnitOutcome>)>,
}

impl Recorded {
    /// **Derived from the unit outcomes alone and never from the failure
    /// records raised against them**, because those the user may clear and the
    /// outcomes they may not (P5 item 3).
    ///
    /// **`None` while any unit's outcome has not landed.** Such a record is
    /// either an action still running or one an interruption caught between
    /// doing a part and saying so, and **nothing here pretends to tell the two
    /// apart** — deriving an ending over a unit whose ending was never written
    /// would be inventing the missing half rather than reporting it.
    /// [`Recorded::unfinished`] names those units, and presenting them is
    /// `Desktop app and delivery`'s.
    pub fn the_actions_outcome(&self) -> Option<ActionOutcome> {
        if self.units.is_empty() || self.unfinished().next().is_some() {
            return None;
        }
        let done = self
            .units
            .iter()
            .filter(|(_, outcome)| matches!(outcome, Some(UnitOutcome::Done)))
            .count();
        Some(if done == self.units.len() {
            ActionOutcome::Whole
        } else if done == 0 {
            ActionOutcome::Nothing
        } else {
            ActionOutcome::Partial
        })
    }

    /// The units that were owed and whose ending was never written.
    pub fn unfinished(&self) -> impl Iterator<Item = &str> + '_ {
        self.units
            .iter()
            .filter(|(_, outcome)| outcome.is_none())
            .map(|(unit, _)| unit.as_str())
    }

    /// The units that failed, and the units never tried — what Row 292's
    /// second half is said from.
    pub fn what_could_not_be_done(&self) -> Vec<(&str, &UnitOutcome)> {
        self.units
            .iter()
            .filter_map(|(unit, outcome)| match outcome {
                Some(outcome @ (UnitOutcome::Failed | UnitOutcome::NotAttempted { .. })) => {
                    Some((unit.as_str(), outcome))
                }
                _ => None,
            })
            .collect()
    }
}

/// Why a write into the record was refused. **Every one of these is a rule of
/// §9.4 or §8.6 kept rather than a fault of the store's**; the store's own
/// troubles are the keeper's to report in its own words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// **§8.6 F19: a unit outcome is immutable.** Nothing anywhere is
    /// permitted to write over one.
    ThatUnitHasAlreadyEnded,
    /// A unit the enumeration never owed. P5 item 1 makes the enumeration
    /// complete before the first unit is attempted, so a unit arriving later
    /// is a caller that enumerated wrongly.
    ThatUnitWasNotOwed,
    /// The store said no. Its own words.
    TheStore(String),
}

/// The durable half — `Desktop app and delivery` §5 implements it over the
/// local record, and a test implements it over a list.
///
/// **The ordering is the whole of P6** and it is why this trait has two
/// separate writes rather than one save at the end: the enumeration is durable
/// **before the first unit is attempted**, and each outcome is durable **at
/// the moment it becomes true**. A design that presented first and recorded
/// afterwards would satisfy Row 292 in the ordinary case and go quiet in
/// exactly the case the sentence exists for.
pub trait Keeps {
    /// **P5 item 1**: write the enumeration, under a freshly issued identity,
    /// before the first unit is attempted.
    fn enumerate(&mut self, action: &str, units: &[&str]) -> Result<Identity, Refused>;

    /// **P5 item 2**: write one unit's outcome, at the moment it becomes true.
    fn a_unit_ended(
        &mut self,
        record: &Identity,
        unit: &str,
        outcome: UnitOutcome,
    ) -> Result<(), Refused>;

    fn read(&self, record: &Identity) -> Result<Option<Recorded>, Refused>;

    /// **§11.1: user-held.** It ends when the user removes it and never
    /// otherwise — no age, no cap, no sweep. This is the act of the user
    /// removing it, and nothing else calls it.
    fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused>;
}

/// One action's record while the action runs.
///
/// **It is a handle and not a copy.** Every write goes straight through to the
/// keeper, because a copy held here and saved at the end would be exactly the
/// design P6 forbids.
pub struct Writing<'a> {
    identity: Identity,
    units: Vec<String>,
    keeper: &'a mut dyn Keeps,
}

impl<'a> Writing<'a> {
    /// Open the record: issue the identity and write the enumeration.
    /// **Called before the first unit is attempted**, and a failure here is a
    /// failure of the action rather than of its bookkeeping — an action whose
    /// enumeration could not be made durable cannot keep Row 292's second half
    /// at all.
    pub fn open(
        keeper: &'a mut dyn Keeps,
        action: &str,
        units: &[&str],
    ) -> Result<Self, Refused> {
        let identity = keeper.enumerate(action, units)?;
        Ok(Writing {
            identity,
            units: units.iter().map(|unit| unit.to_string()).collect(),
            keeper,
        })
    }

    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    pub fn a_unit_ended(&mut self, unit: &str, outcome: UnitOutcome) -> Result<(), Refused> {
        self.keeper.a_unit_ended(&self.identity, unit, outcome)
    }

    /// **§9.3 P4: every unit after the one that stopped the act is written
    /// *not attempted*, naming it.** This is a write and not a silence: a unit
    /// left with no outcome reads as one whose ending was lost, which is a
    /// different thing from one that was deliberately never tried.
    pub fn the_rest_were_not_attempted(&mut self, stopped_by: &str) -> Result<(), Refused> {
        let after: Vec<String> = match self.units.iter().position(|unit| unit == stopped_by) {
            Some(at) => self.units[at + 1..].to_vec(),
            None => Vec::new(),
        };
        for unit in after {
            self.a_unit_ended(
                &unit,
                UnitOutcome::NotAttempted {
                    stopped_by: stopped_by.to_string(),
                },
            )?;
        }
        Ok(())
    }

    pub fn read(&self) -> Result<Option<Recorded>, Refused> {
        self.keeper.read(&self.identity)
    }
}

/// **P7: no unit is retried inside its action.** A failed unit is recorded and
/// reported; trying again is a new action with a new operation identity, which
/// means a new record. Stated here as a constant so the rule has somewhere to
/// be cited from, because it is a rule about what this file's callers must not
/// do rather than something this file can enforce.
pub const NO_UNIT_IS_RETRIED_INSIDE_ITS_ACTION: bool = true;

/// Issue an action's identity. The keeper calls this; nothing else does.
pub fn issue() -> Result<Identity, crate::identities::NoRandomness> {
    Identity::issue(KIND)
}

#[cfg(test)]
pub mod a_list {
    //! A keeper that holds its records in a list — the whole of §9.4's rules
    //! with none of an engine, so the tests below and `Keys`' own can run at
    //! the desk.

    use super::*;

    #[derive(Default)]
    pub struct AList {
        pub records: Vec<Recorded>,
    }

    impl AList {
        fn find(&mut self, record: &Identity) -> Option<&mut Recorded> {
            self.records.iter_mut().find(|held| held.identity == *record)
        }
    }

    impl Keeps for AList {
        fn enumerate(&mut self, action: &str, units: &[&str]) -> Result<Identity, Refused> {
            let identity = issue().map_err(|_| Refused::TheStore("no randomness".into()))?;
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
            let Some(held) = self.find(record) else {
                return Err(Refused::TheStore("no such record".into()));
            };
            let Some(entry) = held.units.iter_mut().find(|(owed, _)| owed == unit) else {
                return Err(Refused::ThatUnitWasNotOwed);
            };
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
}

#[cfg(test)]
mod tests {
    use super::a_list::AList;
    use super::*;

    #[test]
    fn the_enumeration_is_durable_before_any_unit_is_attempted() {
        let mut keeper = AList::default();
        let writing = Writing::open(&mut keeper, "delete a managed key", &["the private half", "the erasure"])
            .expect("the enumeration is written");
        let read = writing.read().unwrap().unwrap();
        assert_eq!(read.units.len(), 2);
        assert!(read.units.iter().all(|(_, outcome)| outcome.is_none()));
        // P6: the fact that a unit was owed survives an interruption here.
        assert_eq!(read.the_actions_outcome(), None);
    }

    #[test]
    fn a_unit_outcome_is_written_once_and_never_over() {
        let mut keeper = AList::default();
        let mut writing = Writing::open(&mut keeper, "generation", &["the key pair"]).unwrap();
        writing.a_unit_ended("the key pair", UnitOutcome::Done).unwrap();
        assert_eq!(
            writing.a_unit_ended("the key pair", UnitOutcome::Failed),
            Err(Refused::ThatUnitHasAlreadyEnded)
        );
        assert_eq!(
            writing.a_unit_ended("a unit nobody owed", UnitOutcome::Done),
            Err(Refused::ThatUnitWasNotOwed)
        );
    }

    #[test]
    fn the_actions_outcome_is_derived_three_ways_and_stored_none() {
        let cases = [
            (vec![UnitOutcome::Done, UnitOutcome::Done], ActionOutcome::Whole),
            (
                vec![UnitOutcome::Done, UnitOutcome::Failed],
                ActionOutcome::Partial,
            ),
            (
                vec![
                    UnitOutcome::Failed,
                    UnitOutcome::NotAttempted {
                        stopped_by: "a".to_string(),
                    },
                ],
                ActionOutcome::Nothing,
            ),
        ];
        for (outcomes, expected) in cases {
            let mut keeper = AList::default();
            let units = ["a", "b"];
            let mut writing = Writing::open(&mut keeper, "an act", &units).unwrap();
            for (unit, outcome) in units.iter().zip(outcomes) {
                writing.a_unit_ended(unit, outcome).unwrap();
            }
            assert_eq!(writing.read().unwrap().unwrap().the_actions_outcome(), Some(expected));
        }
    }

    #[test]
    fn a_unit_never_tried_is_recorded_as_such_and_names_what_stopped_it() {
        let mut keeper = AList::default();
        let mut writing = Writing::open(
            &mut keeper,
            "delete a managed key",
            &["the private half", "the public half", "the erasure"],
        )
        .unwrap();
        writing
            .a_unit_ended("the private half", UnitOutcome::Failed)
            .unwrap();
        writing.the_rest_were_not_attempted("the private half").unwrap();

        let read = writing.read().unwrap().unwrap();
        assert_eq!(read.the_actions_outcome(), Some(ActionOutcome::Nothing));
        assert_eq!(
            read.what_could_not_be_done(),
            vec![
                ("the private half", &UnitOutcome::Failed),
                (
                    "the public half",
                    &UnitOutcome::NotAttempted {
                        stopped_by: "the private half".to_string()
                    }
                ),
                (
                    "the erasure",
                    &UnitOutcome::NotAttempted {
                        stopped_by: "the private half".to_string()
                    }
                ),
            ]
        );
    }

    #[test]
    fn an_interrupted_record_says_which_unit_lost_its_ending_rather_than_deriving_one() {
        let mut keeper = AList::default();
        let mut writing =
            Writing::open(&mut keeper, "generation", &["the key pair", "the row"]).unwrap();
        writing.a_unit_ended("the key pair", UnitOutcome::Done).unwrap();
        // The application stops here. The saying is lost; the fact is not.
        let read = writing.read().unwrap().unwrap();
        assert_eq!(read.the_actions_outcome(), None);
        assert_eq!(read.unfinished().collect::<Vec<_>>(), vec!["the row"]);
    }

    #[test]
    fn it_ends_when_the_user_removes_it_and_never_otherwise() {
        let mut keeper = AList::default();
        let identity = {
            let writing = Writing::open(&mut keeper, "an act", &["a"]).unwrap();
            writing.identity().clone()
        };
        assert!(keeper.read(&identity).unwrap().is_some());
        keeper.the_user_removed_it(&identity).unwrap();
        assert!(keeper.read(&identity).unwrap().is_none());
    }

    #[test]
    fn the_identity_is_an_operation_identity() {
        assert_eq!(issue().unwrap().kind(), IdentityKind::Operation);
    }
}

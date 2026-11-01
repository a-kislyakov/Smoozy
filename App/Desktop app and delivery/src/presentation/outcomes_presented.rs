//! **Atomic and partial outcomes presented** — §16.6 M23 to M27.
//!
//! The record is `State, identity and recovery` §9.4's and is built at
//! [`crate::record::outcome_records`]; its shape is
//! [`smoozy_state::outcomes::Recorded`]. **This file reads it and derives
//! nothing of its own**, which is M23: *"The outcome record is read and its
//! boundary is not re-decided here … The presentation derives nothing of its
//! own and shows what that record says."*
//!
//! ## The four things this file is
//!
//! - **M24. *Not attempted* is presented as *not attempted*, naming the unit
//!   that stopped it, and never as *failed*.** Row 292 requires the user to be
//!   told what Smoozy could not do, and *"a unit reported as failed when it was
//!   never tried is not that"*. These are different things to a user and the
//!   distinction is the whole point of the clause. **[`AUnit`] has three
//!   states and no way to collapse two of them.**
//! - **M25. An interruption loses the saying and never the fact**, because each
//!   unit's outcome is durable before it is presented and the next presentation
//!   reads the record. **Nothing here holds a unit outcome anywhere but in the
//!   record.**
//! - **M26. A failure record raised against a failed unit hangs off the outcome
//!   record as its durable subject**; it is the mark, carries no clearing
//!   gesture and goes with its subject (as M26 reads from 2026-09-19). The unit
//!   outcome itself is immutable and is not the user's to clear — which is
//!   `State, identity and recovery` F19, and it is why
//!   [`crate::record::failure_records`] has no route to a unit outcome at all.
//! - **M27. Where an action's partition unit is the whole action, there is no
//!   unit list to show** — not an empty one, none — *"and the presentation says
//!   the one outcome. Which kind an action is comes from its own document's
//!   declaration (A7) and is never decided at the moment of drawing it."*
//!
//! ## Nothing in the application can reach this file, and that is recorded
//! here rather than left to be found
//!
//! **§16.6 is built entire and it is the one part of §16 with no route to a
//! screen at all**, while the other five each have one. There is **no surface
//! file for it, no command, no event kind and no page module**: nothing outside
//! this file's own tests calls [`presented`].
//!
//! **That is very likely right rather than a gap.** The actions that produce
//! outcome records — a move, a delete, a batch upload — belong to later steps'
//! screens, and `State, identity and recovery` §9.4's record is written by
//! whoever performs the action. **What removes this note is the screen for an
//! action that produces outcome records**, which is where the unit list is
//! drawn.
//!
//! **What was missing was the record, and the record was the defect.** The
//! counted gate's clause table calls all five of §16.6's clauses *Built*, its
//! owed items do not include this, and the board does not carry it either — so
//! a reader had no way to tell a deliberate absence from a forgotten one.
//! `src/surface/said_once.rs` names both of its own callerless writers at its
//! head and says why each is one; **this is that habit kept.** Written
//! 2026-09-12 at round 1 of the code audit of what is drawn inside the window
//! (F-14).
//!
//! ## The one thing M27 is easy to get wrong
//!
//! **An outcome record holding one unit is not the same thing as an action that
//! is atomic entire.** A move of one file is an action whose partition unit is
//! one file and whose enumeration happens to hold one; a rename is an action
//! whose unit is the whole action. Counting the units would draw the two the
//! same way. **So [`presented`] asks the declaration and never the record**,
//! and answers [`TheOutcome::TheOneOutcome`] only where the declaration says
//! so.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §9.4.
//!
//! **`State, identity and recovery` F19 and P5 are deliberately not listed above
//! and are written out with their document at every site instead** — this plan
//! defines an F19 and a P5 of its own, so a bare one would resolve to a real and
//! wrong clause.

use smoozy_state::outcomes::{ActionOutcome, Recorded, UnitOutcome};

use crate::presentation::declarations::TheDeclarations;

/// One unit, as a surface receives it. **Three states, and *not attempted*
/// names the unit that stopped it** — M24.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "outcome")]
pub enum AUnit {
    Done { role: String },
    Failed { role: String },
    /// **Never *failed*.** The unit that stopped it travels with it, because
    /// M24 asks for the naming and not only for the word.
    NotAttempted { role: String, stopped_by: String },
    /// **`State, identity and recovery` P5 item 3's `None`: a unit that was owed
    /// and whose ending has not been
    /// written.** *"Such a record is either an action still running or one an
    /// interruption caught between doing a part and saying so, and nothing here
    /// pretends to tell the two apart."* M3 is why it is said rather than
    /// smoothed: nothing is presented as a whole account where a gap is known.
    NotYetWritten { role: String },
}

/// What a surface is handed about one action.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "shape")]
pub enum TheOutcome {
    /// **M27: the partition unit is the whole action, so there is no unit list
    /// — not an empty one, none.**
    TheOneOutcome {
        action: String,
        /// `None` while the one unit's ending has not been written. **M3:
        /// nothing is presented as finished before its outcome is durable.**
        outcome: Option<&'static str>,
    },
    /// The action is partial across its units, so the list is what there is to
    /// show. **The action's own outcome is derived from the list and is never a
    /// separate field** (`State, identity and recovery` P5 item 3), which is why
    /// it is read from
    /// [`Recorded::the_actions_outcome`] rather than stored.
    TheUnits {
        action: String,
        units: Vec<AUnit>,
        /// `None` while any unit's outcome has not landed.
        the_actions_outcome: Option<&'static str>,
    },
}

/// **M23 and M27: read the record, ask the declaration which shape it is, and
/// derive nothing else.**
///
/// `module` is the module the action belongs to; the declaration route is
/// [`crate::presentation::declarations`]. **Where the module declared nothing
/// about the action the answer is `None`** — A7's hosting fault, reported
/// rather than guessed past, because guessing either way draws a wrong shape
/// and neither is safer than the other.
pub fn presented(
    record: &Recorded,
    module: &str,
    declarations: &TheDeclarations,
) -> Option<TheOutcome> {
    let whole = declarations.the_unit_is_the_whole_action(module, &record.action)?;
    if whole {
        return Some(TheOutcome::TheOneOutcome {
            action: record.action.clone(),
            outcome: record
                .units
                .first()
                .and_then(|(_, outcome)| outcome.as_ref())
                .map(the_word_for),
        });
    }
    Some(TheOutcome::TheUnits {
        action: record.action.clone(),
        units: record.units.iter().map(a_unit).collect(),
        the_actions_outcome: record.the_actions_outcome().map(the_word_for_the_action),
    })
}

fn a_unit((role, outcome): &(String, Option<UnitOutcome>)) -> AUnit {
    match outcome {
        None => AUnit::NotYetWritten { role: role.clone() },
        Some(UnitOutcome::Done) => AUnit::Done { role: role.clone() },
        Some(UnitOutcome::Failed) => AUnit::Failed { role: role.clone() },
        Some(UnitOutcome::NotAttempted { stopped_by }) => AUnit::NotAttempted {
            role: role.clone(),
            stopped_by: stopped_by.clone(),
        },
    }
}

/// The word for one unit's outcome. **Three words and never two** — M24.
fn the_word_for(outcome: &UnitOutcome) -> &'static str {
    match outcome {
        UnitOutcome::Done => "done",
        UnitOutcome::Failed => "failed",
        UnitOutcome::NotAttempted { .. } => "not attempted",
    }
}

fn the_word_for_the_action(outcome: ActionOutcome) -> &'static str {
    match outcome {
        ActionOutcome::Whole => "whole",
        ActionOutcome::Partial => "partial",
        ActionOutcome::Nothing => "nothing",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::declarations::{Declared, ThePartitionUnit};
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_declaration() -> TheDeclarations {
        let mut declarations = TheDeclarations::new();
        declarations.module_declares(
            "File Browser",
            Declared {
                conditions: Vec::new(),
                dismissible_warnings: Vec::new(),
                partition_units: vec![
                    ThePartitionUnit {
                        action: "rename",
                        the_unit_is_the_whole_action: true,
                    },
                    ThePartitionUnit {
                        action: "an upload request",
                        the_unit_is_the_whole_action: false,
                    },
                    ThePartitionUnit {
                        action: "move, same filesystem",
                        the_unit_is_the_whole_action: false,
                    },
                ],
            },
        );
        declarations
    }

    fn recorded(action: &str, units: Vec<(&str, Option<UnitOutcome>)>) -> Recorded {
        Recorded {
            identity: Identity::issue(IdentityKind::Operation).expect("the desk has randomness"),
            action: action.to_string(),
            units: units
                .into_iter()
                .map(|(role, outcome)| (role.to_string(), outcome))
                .collect(),
        }
    }

    /// **M24: *not attempted* is presented as *not attempted*, naming the unit
    /// that stopped it, and never as *failed*.**
    #[test]
    fn not_attempted_is_never_drawn_as_failed_and_names_what_stopped_it() {
        let record = recorded(
            "an upload request",
            vec![
                ("one file's transfer", Some(UnitOutcome::Failed)),
                (
                    "one file's transfer",
                    Some(UnitOutcome::NotAttempted {
                        stopped_by: "one file's transfer".to_string(),
                    }),
                ),
            ],
        );
        let TheOutcome::TheUnits { units, .. } =
            presented(&record, "File Browser", &a_declaration()).unwrap()
        else {
            panic!("an upload request is partial across its files")
        };
        assert!(matches!(units[0], AUnit::Failed { .. }));
        match &units[1] {
            AUnit::NotAttempted { stopped_by, .. } => {
                assert_eq!(stopped_by, "one file's transfer")
            }
            other => panic!("a unit that was never tried is not {other:?}"),
        }
    }

    /// **M27: where the partition unit is the whole action there is no unit
    /// list — not an empty one, none.**
    #[test]
    fn an_atomic_action_has_no_unit_list_at_all() {
        let record = recorded("rename", vec![("the rename", Some(UnitOutcome::Failed))]);
        match presented(&record, "File Browser", &a_declaration()).unwrap() {
            TheOutcome::TheOneOutcome { outcome, .. } => assert_eq!(outcome, Some("failed")),
            other => panic!("an atomic action shows one outcome, not {other:?}"),
        }
    }

    /// **M27's trap: an outcome record holding one unit is not an atomic
    /// action.** The declaration decides and the count never does.
    #[test]
    fn one_unit_in_the_record_is_not_the_same_as_an_atomic_action() {
        let record = recorded(
            "move, same filesystem",
            vec![("one top-level item", Some(UnitOutcome::Done))],
        );
        match presented(&record, "File Browser", &a_declaration()).unwrap() {
            TheOutcome::TheUnits { units, .. } => assert_eq!(units.len(), 1),
            other => panic!("a partial action shows its list however short it is, not {other:?}"),
        }
    }

    /// **A7: a module with no declaration cannot be hosted, and this is that
    /// reported rather than guessed past.**
    #[test]
    fn an_undeclared_action_has_no_shape_to_draw() {
        let record = recorded("something nobody declared", vec![]);
        assert!(presented(&record, "File Browser", &a_declaration()).is_none());
        assert!(presented(&record, "Nobody", &a_declaration()).is_none());
    }

    /// **M3 and `State, identity and recovery` P5 item 3: nothing is presented
    /// as finished before its outcome
    /// is durable**, and a unit whose ending was never written is said rather
    /// than smoothed.
    #[test]
    fn a_unit_whose_ending_was_never_written_is_said_and_the_action_has_no_outcome() {
        let record = recorded(
            "an upload request",
            vec![
                ("one file's transfer", Some(UnitOutcome::Done)),
                ("one file's transfer", None),
            ],
        );
        let TheOutcome::TheUnits {
            units,
            the_actions_outcome,
            ..
        } = presented(&record, "File Browser", &a_declaration()).unwrap()
        else {
            panic!("an upload request is partial across its files")
        };
        assert!(matches!(units[1], AUnit::NotYetWritten { .. }));
        assert_eq!(the_actions_outcome, None);
    }

    /// **M23 and `State, identity and recovery` P5 item 3: the action's own
    /// outcome is derived from the list
    /// and is never a separate field.** Row 292's case is *partial*.
    #[test]
    fn the_actions_own_outcome_is_derived_from_the_list() {
        let record = recorded(
            "an upload request",
            vec![
                ("one file's transfer", Some(UnitOutcome::Done)),
                ("one file's transfer", Some(UnitOutcome::Failed)),
            ],
        );
        let TheOutcome::TheUnits {
            the_actions_outcome,
            ..
        } = presented(&record, "File Browser", &a_declaration()).unwrap()
        else {
            panic!("an upload request is partial across its files")
        };
        assert_eq!(the_actions_outcome, Some("partial"));
    }

    /// **M25: an interruption loses the saying and never the fact.** The same
    /// record read a second time says the same thing, because nothing here
    /// holds a unit outcome anywhere but in the record.
    #[test]
    fn reading_the_record_again_says_the_same_thing() {
        let record = recorded(
            "an upload request",
            vec![("one file's transfer", Some(UnitOutcome::Failed))],
        );
        let declarations = a_declaration();
        assert_eq!(
            presented(&record, "File Browser", &declarations),
            presented(&record, "File Browser", &declarations)
        );
    }
}

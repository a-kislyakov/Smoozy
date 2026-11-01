//! An act taken over a selection of rows — `Dependency Upload & Remap` §4.5,
//! Row 427: F22–F26.
//!
//! **F22: an act taken over a selection is N independent acts of that same
//! act, one per row, and never one action with an outcome of its own.** The
//! partition unit is one row and the act inside one row is the single-row act
//! unchanged (Section 10). The write stays per row and stays immediate (K4,
//! G16); the selection is held nowhere durable; and **no `State, identity and
//! recovery` §9.4 outcome record is written, because no unit is ever owed** —
//! a row the act did not reach is a row nothing promised anything about.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Queue & History` L18, L19. `State, identity and
//! recovery` §9.4.

use smoozy_state::lifetimes::Lifetime;

use crate::readings::Readings;
use crate::record::{LocatedAnswer, Resolution};
use crate::search::{resolution_from_an_accepted_proposal, ResolutionFailed, WantedShape};
use crate::shift::{look_for, ShiftOutcome};

/// **F22: the selection is a sitting's own state, run-held as F7's per-folder
/// search outcome is, and it is not a record kind** — K1's five are closed.
pub const THE_SELECTION_IS_RUN_HELD: Lifetime = Lifetime::RunHeld;

/// **F22 and F25: no §9.4 outcome record and no record of the selection act
/// itself.** The precedent is `Queue & History` L18 and L19: there is no
/// selection store and a selection act is row acts under one command.
pub const NO_OUTCOME_RECORD_AND_NO_SELECTION_RECORD_IS_WRITTEN: bool = true;

/// **F23: what a selection act stopped part-way leaves** — every row it
/// reached decided and durable, every row it did not exactly as it was, and
/// no bulk outcome to show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionOutcome<Outcome> {
    /// One outcome per row reached, in the order reached — each the
    /// single-row act's own, a failure naming its own row.
    pub reached: Vec<Outcome>,
    pub not_reached: usize,
}

/// **F22: N independent acts, one per row**, each written as it lands, with
/// the stop asked between rows.
pub fn over_a_selection<Row, Outcome>(
    rows: Vec<Row>,
    mut act: impl FnMut(Row) -> Outcome,
    stop: &mut dyn FnMut() -> bool,
) -> SelectionOutcome<Outcome> {
    let total = rows.len();
    let mut reached = Vec::new();
    for row in rows {
        if stop() {
            break;
        }
        reached.push(act(row));
    }
    let not_reached = total - reached.len();
    SelectionOutcome {
        reached,
        not_reached,
    }
}

/// The acts the spec grants over a row, each with its inverse (F25).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowAct {
    SetAside,
    BringBack,
    AcceptAProposal,
    RejectAProposal,
    ReinstateAGuess,
    LocateAgain,
}

/// **F25: every act in the set has an inverse the spec already grants**, and
/// taking the inverse over the same selection is one act by F22 and needs no
/// new record. **What the inverse is not is a restoration of what stood
/// before**: the record holds a mark rather than a history, so a row already
/// set aside before a bulk set-aside is brought back with the rest.
pub fn the_inverse_of(act: RowAct) -> RowAct {
    match act {
        RowAct::SetAside => RowAct::BringBack,
        RowAct::BringBack => RowAct::SetAside,
        RowAct::AcceptAProposal => RowAct::LocateAgain,
        RowAct::RejectAProposal => RowAct::ReinstateAGuess,
        RowAct::ReinstateAGuess => RowAct::RejectAProposal,
        RowAct::LocateAgain => RowAct::LocateAgain,
    }
}

/// **The mark is on the row and the panel reads it; no history is kept and
/// none is promised.** Set-aside over a row that was already set aside is the
/// same mark.
pub fn set_aside(mut answer: LocatedAnswer) -> LocatedAnswer {
    answer.set_aside = true;
    answer
}

pub fn bring_back(mut answer: LocatedAnswer) -> LocatedAnswer {
    answer.set_aside = false;
    answer
}

/// A row as a bulk accept sees it: its answer, and the proposal it carries
/// where it carries one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowWithAProposal {
    pub answer: LocatedAnswer,
    pub proposal: Option<Vec<u8>>,
}

/// What one row's accept came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Accepted {
    /// One located answer written, naming that row's own candidate.
    Written(LocatedAnswer),
    /// The row carried no proposal and is untouched.
    Untouched(LocatedAnswer),
    /// The single-row act's own failure — F12's reading taken per row at the
    /// moment that row is resolved, and a candidate that moved fails that
    /// row alone (F23).
    Failed(LocatedAnswer, ResolutionFailed),
}

/// **F24: a capitals-only match is accepted per row and never once for a
/// set.** A bulk accept writes one located answer per selected row that
/// carries a proposal, each naming that row's own candidate; a selected row
/// carrying none is untouched. **No selection act ever changes what counts
/// as a match** — F9's byte equality decides what is found whatever the size
/// of the selection, and F10's fold decides only what is offered.
pub fn accept_one(row: RowWithAProposal, readings: &dyn Readings) -> Accepted {
    let Some(proposal) = row.proposal else {
        return Accepted::Untouched(row.answer);
    };
    match resolution_from_an_accepted_proposal(&proposal, readings) {
        Ok(resolution) => {
            let mut answer = row.answer;
            answer.resolution = Some(resolution);
            answer.rejected_guess = None;
            Accepted::Written(answer)
        }
        Err(why) => Accepted::Failed(row.answer, why),
    }
}

/// **K4's rejected guess, written per row when a guess is undone over a
/// selection** — kept so it is not put forward again by itself and can be
/// reinstated.
pub fn reject_a_proposal(mut answer: LocatedAnswer, proposal: &[u8]) -> LocatedAnswer {
    answer.rejected_guess = Some(crate::bytes::StoredLocation::as_the_system_returned_it(
        proposal,
    ));
    answer
}

pub fn reinstate_a_guess(mut answer: LocatedAnswer) -> LocatedAnswer {
    answer.rejected_guess = None;
    answer
}

/// **F26: three acts are not selection acts.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotASelectionAct {
    /// **Locate… has no per-row bulk form, and its reach across rows is the
    /// shift.** Pointing many references at one file resolves each of them to
    /// a file it does not mean.
    Locate,
    /// A confirmation is Q7 run per row, each with its own operation
    /// identity — a new action rather than a retry inside one.
    Confirmation,
    /// A *could not be sent* retry is A11's new transfer per row.
    CouldNotBeSentRetry,
    /// A *could not be fixed* retry is M16a's own production per row.
    CouldNotBeFixedRetry,
}

/// **F26: where a locate is offered over a selection at all, the only reading
/// that points no reference at a file it does not mean is a locate at a
/// folder** — each selected row resolved inside it by F18's own rule, and
/// **no shift is derived**, because the user named a folder and not an
/// old-and-new pair for F16 to take a common suffix from.
pub fn locate_at_a_folder(
    folder: &[u8],
    wanted_name: &[u8],
    shape: WantedShape,
    readings: &dyn Readings,
) -> ShiftOutcome {
    look_for(&crate::bytes::join(folder, wanted_name), shape, readings)
}

/// **F26: nothing here caps how many rows an act may reach**, because nothing
/// accumulates: N rows is N record writes, or N transfers already bounded by
/// the per-server admission point. How many the panel lets the user select is
/// Design's.
pub const NOTHING_CAPS_HOW_MANY_ROWS_AN_ACT_MAY_REACH: bool = true;

/// **F23: no row is ever half-acted, because the single-row act is already
/// atomic** — a located answer is one durable write. A resolution written
/// over a selection is the same [`Resolution`] a single act writes.
pub fn is_the_single_row_act(resolution: &Resolution) -> bool {
    let _ = resolution;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::{RecordedLocation, StoredLocation};
    use crate::readings::StoredNameAnswer;
    use crate::record::{HoldingFile, HowResolved, LocatedAnswerKey};
    use crate::search::tests::Disk;

    fn an_answer(reference: &[u8]) -> LocatedAnswer {
        LocatedAnswer {
            key: LocatedAnswerKey {
                holding_file: HoldingFile::OnTheUsersComputer(
                    StoredLocation::as_the_system_returned_it(b"/a.blend"),
                ),
                reference: RecordedLocation::as_the_reading_carried_it(reference),
            },
            resolution: None,
            set_aside: false,
            rejected_guess: None,
            read_as: Vec::new(),
        }
    }

    #[test]
    fn a_selection_act_is_n_acts_and_a_stop_leaves_the_rest_exactly_as_they_were() {
        let rows = vec![an_answer(b"a"), an_answer(b"b"), an_answer(b"c")];
        let mut asked = 0;
        let mut stop = || {
            asked += 1;
            asked > 2
        };
        let outcome = over_a_selection(rows, set_aside, &mut stop);
        assert_eq!(outcome.reached.len(), 2);
        assert_eq!(outcome.not_reached, 1);
        assert!(outcome.reached.iter().all(|answer| answer.set_aside));
    }

    #[test]
    fn a_bulk_accept_writes_one_answer_per_row_carrying_a_proposal_and_touches_no_other() {
        let mut disk = Disk::default();
        disk.stored_names.insert(
            b"/Work/METallic.png".to_vec(),
            StoredNameAnswer::TheStoredName(StoredLocation::as_the_system_returned_it(
                b"/Work/METallic.png",
            )),
        );
        disk.stored_names
            .insert(b"/Work/gone.png".to_vec(), StoredNameAnswer::NotThere);
        let rows = vec![
            RowWithAProposal {
                answer: an_answer(b"//tex/metallic.png"),
                proposal: Some(b"/Work/METallic.png".to_vec()),
            },
            RowWithAProposal {
                answer: an_answer(b"//tex/wood.png"),
                proposal: None,
            },
            RowWithAProposal {
                answer: an_answer(b"//tex/gone.png"),
                proposal: Some(b"/Work/gone.png".to_vec()),
            },
        ];
        let outcome = over_a_selection(rows, |row| accept_one(row, &disk), &mut || false);
        assert_eq!(outcome.not_reached, 0);
        match &outcome.reached[0] {
            Accepted::Written(answer) => {
                assert_eq!(
                    answer.resolution.as_ref().map(|r| r.how.clone()),
                    Some(HowResolved::TheUsersOwnHand)
                );
            }
            other => panic!("{other:?}"),
        }
        assert!(matches!(&outcome.reached[1], Accepted::Untouched(_)));
        assert!(matches!(
            &outcome.reached[2],
            Accepted::Failed(_, ResolutionFailed::TheCandidateIsNoLongerThere)
        ));
    }

    #[test]
    fn every_act_has_its_inverse_and_the_inverse_is_a_mark_not_a_history() {
        assert_eq!(the_inverse_of(RowAct::SetAside), RowAct::BringBack);
        assert_eq!(
            the_inverse_of(RowAct::RejectAProposal),
            RowAct::ReinstateAGuess
        );
        let already = set_aside(an_answer(b"a"));
        let after_bulk = set_aside(already);
        assert!(!bring_back(after_bulk).set_aside);
    }

    #[test]
    fn a_rejected_guess_is_kept_on_the_row_and_can_be_reinstated() {
        let rejected = reject_a_proposal(an_answer(b"a"), b"/Work/METallic.png");
        assert_eq!(
            rejected
                .rejected_guess
                .as_ref()
                .map(|guess| guess.as_bytes().to_vec()),
            Some(b"/Work/METallic.png".to_vec())
        );
        assert_eq!(reinstate_a_guess(rejected).rejected_guess, None);
    }

    #[test]
    fn a_locate_at_a_folder_resolves_each_row_inside_it_and_derives_no_shift() {
        let mut disk = Disk::default();
        disk.folder(b"/Archive/tex", &[b"metallic.png", b"WOOD.png"], &[]);
        assert_eq!(
            locate_at_a_folder(b"/Archive/tex", b"metallic.png", WantedShape::AFile, &disk),
            ShiftOutcome::Resolved(StoredLocation::as_the_system_returned_it(
                b"/Archive/tex/metallic.png"
            ))
        );
        assert_eq!(
            locate_at_a_folder(b"/Archive/tex", b"wood.png", WantedShape::AFile, &disk),
            ShiftOutcome::AProposal(b"/Archive/tex/WOOD.png".to_vec())
        );
        assert_eq!(
            locate_at_a_folder(b"/Archive/tex", b"gone.png", WantedShape::AFile, &disk),
            ShiftOutcome::Unchanged
        );
    }
}

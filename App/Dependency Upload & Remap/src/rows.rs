//! A row's seven states and the action that fits each, and the four things
//! that can qualify what Smoozy found — `Dependency Upload & Remap` Rows 403
//! and 404, answered across F9, Q5, Q7, K4, A5, A3, A7 and M14, and F10.
//!
//! **The spec's own rule binds the shape: *a row that cannot be acted on is a
//! control that does nothing*** — so every state carries its action, and what
//! is not a row is a statement ([`crate::knowledge::Statement`]).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** *(none)*

use crate::record::{HowResolved, LocatedAnswer};

/// **The seven states.** Each names the clause it stands on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RowState {
    /// F9: no exact match anywhere — the **absence** of a located answer
    /// (F7).
    NotFound,
    /// Q5's establishment not yet made: found on the user's computer and not
    /// sent.
    FoundOnYourComputer,
    /// Q5: established equal by record or by reading.
    AlreadyOnTheMachine,
    /// Q7: the record says a file was sent and the local original is no
    /// longer there to compare.
    SentBeforeNotConfirmed,
    /// K4's mark.
    SetAside,
    /// A5 (the mirror will not fit), A7 (the directory could not be made) or
    /// a transfer that did not publish.
    CouldNotBeSent,
    /// A3 (the address will not fit) or M14 (the production did not publish).
    ///
    /// **M20's second half: what cannot be rewritten is not promised as
    /// fixed, and it is stated in the rows rather than smoothed.** This state
    /// is where that statement lands, and its action is
    /// [`Action::TryAgainTheFixing`] rather than silence.
    CouldNotBeFixed,
}

impl RowState {
    pub const ALL: [RowState; 7] = [
        RowState::NotFound,
        RowState::FoundOnYourComputer,
        RowState::AlreadyOnTheMachine,
        RowState::SentBeforeNotConfirmed,
        RowState::SetAside,
        RowState::CouldNotBeSent,
        RowState::CouldNotBeFixed,
    ];

    /// **Each state with the action that fits it, and each is the Design
    /// table's own.** *Already on the machine* needs nothing, and that is an
    /// action of none rather than a control that does nothing.
    ///
    /// **A *found on your computer* row's action is Locate…, not Send.** The
    /// sending sits on the panel's own control, *"naming the number of files
    /// rather than saying Send"*, and Section 10 declares the upload set as
    /// an action over rows rather than a row's own — so a row has no Send to
    /// carry. **The rows most likely to hold a wrong guess are the found
    /// ones**, and this is the state the correction control was taken away
    /// from. Corrected 2026-09-05, out of round 1 of this module's code
    /// audit; a test holds all seven states against the Design table.
    pub fn the_action_that_fits(self) -> Action {
        match self {
            RowState::NotFound => Action::Locate,
            RowState::FoundOnYourComputer => Action::Locate,
            RowState::AlreadyOnTheMachine => Action::Nothing,
            RowState::SentBeforeNotConfirmed => Action::Confirm,
            RowState::SetAside => Action::BringBack,
            RowState::CouldNotBeSent => Action::TryAgainTheSend,
            RowState::CouldNotBeFixed => Action::TryAgainTheFixing,
        }
    }
}

/// The actions, each the clause's own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Locate… by hand, which may teach a shift (F16).
    Locate,
    /// **There is no Send here.** The upload set after a check is Section
    /// 10's action **over rows**, taken from the panel's own control, which
    /// names the number of files rather than saying *Send*. A row's action is
    /// what the row itself carries, and no row carries that one.
    Nothing,
    /// Q7 run per row.
    Confirm,
    /// K4's mark cleared.
    BringBack,
    /// A11: a new transfer, re-sending only what did not publish.
    TryAgainTheSend,
    /// M16a: M14's production run once more for that one file.
    TryAgainTheFixing,
}

/// **The four things that can qualify what Smoozy found** (F10, K4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Qualifier {
    /// F10: a capitals-only near-miss, offered as a proposal with its Accept
    /// on the qualifier (Row 406).
    ACaseNearMissProposal,
    /// K4 and F5: the pick was arbitrary among several candidates, and the
    /// row says so.
    AnArbitraryPick,
    /// K4 and F19: resolved by a shift, shown as the move it is.
    FromAShift,
    /// K4: a proposal the user turned down stands on the row and can be
    /// reinstated.
    ARejectedGuessStands,
}

/// The qualifiers a located answer carries, from the record and the
/// sitting's proposal.
pub fn the_qualifiers_of(answer: &LocatedAnswer, a_proposal_stands: bool) -> Vec<Qualifier> {
    let mut out = Vec::new();
    if a_proposal_stands {
        out.push(Qualifier::ACaseNearMissProposal);
    }
    match answer.resolution.as_ref().map(|resolution| &resolution.how) {
        Some(HowResolved::TheSearch {
            arbitrary: Some(_), ..
        }) => out.push(Qualifier::AnArbitraryPick),
        Some(HowResolved::AShift(_)) => out.push(Qualifier::FromAShift),
        _ => {}
    }
    if answer.rejected_guess.is_some() {
        out.push(Qualifier::ARejectedGuessStands);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::{RecordedLocation, StoredLocation};
    use crate::record::{
        ArbitraryPick, HoldingFile, LocatedAnswerKey, Resolution, ResolvedTo, ShiftKey,
    };

    /// **Every state's action read off the Design table itself**, so a row's
    /// action cannot drift from the document that owns it. The table names
    /// each state twice — the screen's name and the spec's, in its second
    /// column — and its fourth column is *Visible action on the row*; its
    /// words are Design's and the state each row names is this enum's.
    ///
    /// **This is the test that would have caught it**: *found on your
    /// computer* carried `Send` while the table said `Locate…`, and six of
    /// the seven already agreed.
    ///
    /// **From 2026-10-18 (the dependency check's fresh look) a row Smoozy
    /// will upload shows no action**: the table's fourth column for *Found on
    /// your computer* is *—*, and its *Locate…* sits behind the row's ⋯
    /// (`§ A row`: *"⋯, holding the rest: Locate… (point the row at a
    /// different file)"*). The action that fits the state is still Locate…;
    /// only where the screen puts it moved, so this reads both.
    #[test]
    fn every_states_action_is_the_design_tables_own() {
        let design = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Design/Dependency Upload & Remap.md"),
        )
        .expect("the design document is readable");

        // The spec's own name for each state, as the table's second column
        // writes it, then the action.
        let expected: [(RowState, &str, Action); 7] = [
            (
                RowState::FoundOnYourComputer,
                "Found on your computer",
                Action::Locate,
            ),
            (
                RowState::AlreadyOnTheMachine,
                "Already on the machine",
                Action::Nothing,
            ),
            (
                RowState::SentBeforeNotConfirmed,
                "Sent before, not confirmed",
                Action::Confirm,
            ),
            (RowState::NotFound, "Not found", Action::Locate),
            (RowState::SetAside, "Set aside", Action::BringBack),
            (
                RowState::CouldNotBeSent,
                "Could not be sent",
                Action::TryAgainTheSend,
            ),
            (
                RowState::CouldNotBeFixed,
                "Could not be fixed",
                Action::TryAgainTheFixing,
            ),
        ];

        // Design's own word for each action, so the reading is over the
        // document's text and not over a guess. **Confirm is *Verify* on the
        // screen from 2026-10-18.**
        let drawn_as = |action: Action| match action {
            Action::Locate => "Locate…",
            Action::Nothing => "—",
            Action::Confirm => "Verify",
            Action::BringBack => "Restore",
            Action::TryAgainTheSend | Action::TryAgainTheFixing => "Retry",
        };

        let mut seen = 0;
        for (state, word, action) in expected {
            let row = design
                .lines()
                .find(|line| {
                    line.starts_with("| ")
                        && line.split('|').nth(2).map(str::trim) == Some(word)
                })
                .unwrap_or_else(|| panic!("the design table names {word}"));
            let column = row
                .split('|')
                .nth(4)
                .expect("the fourth column")
                .trim()
                .to_string();
            if state == RowState::FoundOnYourComputer {
                // Shown nowhere on the row itself; behind ⋯.
                assert_eq!(column, "—", "{state:?}: the design table says {column:?}");
                assert!(
                    design.contains("**⋯**, holding the rest: *Locate…*"),
                    "Design no longer puts Locate… behind the row's ⋯"
                );
            } else {
                assert!(
                    column.starts_with(drawn_as(state.the_action_that_fits())),
                    "{state:?}: the design table says {column:?}"
                );
            }
            assert_eq!(state.the_action_that_fits(), action, "{state:?}");
            seen += 1;
        }
        assert_eq!(seen, RowState::ALL.len());
    }

    #[test]
    fn every_state_has_an_action_and_only_already_there_needs_nothing() {
        for state in RowState::ALL {
            let action = state.the_action_that_fits();
            assert_eq!(
                action == Action::Nothing,
                state == RowState::AlreadyOnTheMachine,
                "{state:?}"
            );
        }
        assert_eq!(RowState::ALL.len(), 7);
    }

    fn an_answer(how: Option<HowResolved>, rejected: Option<&[u8]>) -> LocatedAnswer {
        LocatedAnswer {
            key: LocatedAnswerKey {
                holding_file: HoldingFile::OnTheUsersComputer(
                    StoredLocation::as_the_system_returned_it(b"/a.blend"),
                ),
                reference: RecordedLocation::as_the_reading_carried_it(b"//x.png"),
            },
            resolution: how.map(|how| Resolution {
                to: ResolvedTo::AWholeLocation(StoredLocation::as_the_system_returned_it(
                    b"/x.png",
                )),
                how,
            }),
            set_aside: false,
            rejected_guess: rejected.map(StoredLocation::as_the_system_returned_it),
            read_as: Vec::new(),
        }
    }

    #[test]
    fn the_qualifiers_come_from_the_record_and_the_proposal() {
        let arbitrary = an_answer(
            Some(HowResolved::TheSearch {
                from_folder: b"/Work".to_vec(),
                arbitrary: Some(ArbitraryPick {
                    taken: StoredLocation::as_the_system_returned_it(b"/x.png"),
                    among: vec![],
                }),
            }),
            None,
        );
        assert_eq!(
            the_qualifiers_of(&arbitrary, false),
            vec![Qualifier::AnArbitraryPick]
        );
        let shifted = an_answer(Some(HowResolved::AShift(ShiftKey(1))), None);
        assert_eq!(
            the_qualifiers_of(&shifted, false),
            vec![Qualifier::FromAShift]
        );
        let not_found = an_answer(None, Some(b"/X.png"));
        assert_eq!(
            the_qualifiers_of(&not_found, true),
            vec![
                Qualifier::ACaseNearMissProposal,
                Qualifier::ARejectedGuessStands
            ]
        );
        let plain = an_answer(Some(HowResolved::TheRecordedLocationItself), None);
        assert!(the_qualifiers_of(&plain, false).is_empty());
    }
}

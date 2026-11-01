//! §12 — atomic or partial, this document's declarations.
//!
//! **Declared per action against `State, identity and recovery` §9, with the
//! test that decided each. Nothing below is decided at run time.**
//!
//! **T1: no unit here is retried inside its action** (`State, identity and
//! recovery` §9.5). A failed submission is a new submission with a new job
//! identity and a new folder, which is `Rendering`'s.
//!
//! **T2: two units rest on Fact 78 and are written as atomic because `State,
//! identity and recovery` §9.6 says so** — the job's material, and a
//! derivative's publication. **Fact 78 answered
//! the interruption boundaries on 2026-08-18 on ext4, xfs and overlayfs, so
//! these two units stand on ground that exists rather than on a rule written
//! against a gap**; §14 item 1 carries what is still open, which is the
//! rollback boundary and Fact 44.

/// The eight actions §12's table declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    PlacingAJobsMaterialOnTheMachineAtSubmission,
    CreatingTheJobOutputFolder,
    ComposingAndRecordingTheReading,
    ApplyingTheRedirectAtLaunch,
    WritingAnAttemptRecord,
    ProducingTheMachineSideDerivativesForAJob,
    FetchingDerivativesToTheUsersComputer,
    ClearingAJobsMachineSideMaterial,
}

/// The eight, as one list.
pub const EVERY_ACTION: [Action; 8] = [
    Action::PlacingAJobsMaterialOnTheMachineAtSubmission,
    Action::CreatingTheJobOutputFolder,
    Action::ComposingAndRecordingTheReading,
    Action::ApplyingTheRedirectAtLaunch,
    Action::WritingAnAttemptRecord,
    Action::ProducingTheMachineSideDerivativesForAJob,
    Action::FetchingDerivativesToTheUsersComputer,
    Action::ClearingAJobsMachineSideMaterial,
];

/// The partition unit, in the table's own words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionUnit {
    TheWholeAction,
    TheWholeReading,
    /// **The whole map, the regular output path's own command-line argument
    /// included** (D10).
    TheWholeMap,
    TheWholeRecord,
    OneDerivative,
    OneDirectory,
}

/// Which of `State, identity and recovery` §9.2's three ordered tests decided
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Test {
    /// The promise is about the whole thing.
    One,
    /// Ordering or care cannot be the mechanism.
    Two,
    /// Independent units.
    Three,
}

/// The kind declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    AtomicEntire,
    /// **One unit with the action above it.** *"Where the folder cannot be
    /// made, the record is **not attempted**, naming the folder as the unit
    /// that stopped it."*
    OneUnitWithThePlacement,
    Atomic,
    /// **Partial across, atomic within.**
    PartialAcrossAtomicWithin,
}

/// Whether the act itself exists in this crate yet.
///
/// **Named rather than left to be discovered.** Step 4 builds the layout, the
/// manifest and the publication contract; the reading, the redirect, the
/// inventory, the previews and the reckoning land with steps 6 to 8
/// (`arriving_later` in `smoozy-job-files`). **A declaration is this document's whatever
/// stage performs the act**, so all eight are declared and each says where its
/// act is.
///
/// **The third value exists because one act is honestly neither of the other
/// two.** Half of the clearing is built here and half of it is not this
/// document's at all, so *built here* and *it arrives with a later step* are
/// both false of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereTheActIs {
    /// The whole act is in this crate.
    BuiltHere,
    /// **It arrives with a later step of `Preparation/Build/Plan.md`, and the
    /// step is named** rather than left as *later*.
    ItArrivesAtStep(u8),
    /// **Part of the act is here and the rest is not this document's.** The
    /// abandoned-material sweep is built here in full and is exactly this
    /// action's declared shape — one directory, partial across, atomic within.
    /// **What is not here is not waiting on a step either**: `Queue & History`
    /// `Queue & History` X2's machine-side erase is that document's act, and
    /// the job record directory's own removal goes with the machine.
    TheSweepIsHereAndTheRestIsAnotherDocumentsAct,
}

/// One row of §12's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declaration {
    pub unit: PartitionUnit,
    pub test: Test,
    pub kind: Kind,
    pub act: WhereTheActIs,
}

/// **§12's table, performed.**
pub fn declaration_for(action: Action) -> Declaration {
    match action {
        // *"`Rendering`'s Row 2 promises an outcome about the whole job and
        // about nothing smaller."* M5's single directory rename is the
        // publication; test 2 forbids reaching it by ordering, and Fact 78's
        // negative two-file half forbids doing it file by file.
        Action::PlacingAJobsMaterialOnTheMachineAtSubmission => Declaration {
            unit: PartitionUnit::TheWholeAction,
            test: Test::One,
            kind: Kind::AtomicEntire,
            act: WhereTheActIs::BuiltHere,
        },
        // *"The record's publication is unsafe unless the folder exists, so the
        // ordered pair is one unit."*
        Action::CreatingTheJobOutputFolder => Declaration {
            unit: PartitionUnit::TheWholeAction,
            test: Test::Three,
            kind: Kind::OneUnitWithThePlacement,
            act: WhereTheActIs::BuiltHere,
        },
        // *"Five promises read it and a half-composed reading would be read as
        // a whole one."*
        Action::ComposingAndRecordingTheReading => Declaration {
            unit: PartitionUnit::TheWholeReading,
            test: Test::One,
            kind: Kind::Atomic,
            // **Step 7 and not step 6.** §4.3 R7 puts the reading on the
            // desktop (`Render Settings`) or on the render worker
            // (`Rendering`), and both are built at step 7; step 6 builds
            // `Rented-server runtime` and `Server Setup`, neither of which
            // composes a reading.
            act: WhereTheActIs::ItArrivesAtStep(7),
        },
        // *"The promise is that **everything** a job writes lands under the
        // job's folder."* A map applied in part is a job writing to the artist's
        // own machine, which is the failure Row 118 exists to prevent.
        Action::ApplyingTheRedirectAtLaunch => Declaration {
            unit: PartitionUnit::TheWholeMap,
            test: Test::One,
            kind: Kind::Atomic,
            act: WhereTheActIs::ItArrivesAtStep(7),
        },
        // *"A half-written frame list read by a later re-run would name a last
        // frame that was never asked for."*
        Action::WritingAnAttemptRecord => Declaration {
            unit: PartitionUnit::TheWholeRecord,
            test: Test::Two,
            kind: Kind::Atomic,
            act: WhereTheActIs::BuiltHere,
        },
        // *"No promise is made about the set, and `Queue & History` promises a
        // preview rather than a number of them."*
        Action::ProducingTheMachineSideDerivativesForAJob => Declaration {
            unit: PartitionUnit::OneDerivative,
            test: Test::One,
            kind: Kind::PartialAcrossAtomicWithin,
            act: WhereTheActIs::ItArrivesAtStep(7),
        },
        // **Step 8 is downloads**, and the act itself is `Download`'s.
        Action::FetchingDerivativesToTheUsersComputer => Declaration {
            unit: PartitionUnit::OneDerivative,
            test: Test::One,
            kind: Kind::PartialAcrossAtomicWithin,
            act: WhereTheActIs::ItArrivesAtStep(8),
        },
        // *"Nothing promises an outcome about the set."* **The sweep of
        // `cleanup` in `smoozy-job-files` is this act's declared shape and is built here in
        // full; the rest of the clearing — `Queue & History` X2's machine-side
        // erase, and the job record directory's own removal — is not this
        // document's act at all.**
        Action::ClearingAJobsMachineSideMaterial => Declaration {
            unit: PartitionUnit::OneDirectory,
            test: Test::One,
            kind: Kind::PartialAcrossAtomicWithin,
            act: WhereTheActIs::TheSweepIsHereAndTheRestIsAnotherDocumentsAct,
        },
    }
}

/// **T1, kept where a builder would otherwise write a retry.**
pub const NO_UNIT_HERE_IS_RETRIED_INSIDE_ITS_ACTION: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_action_is_declared_and_none_is_decided_at_run_time() {
        for action in EVERY_ACTION {
            let declared = declaration_for(action);
            // The declaration is a value of this module and takes nothing from
            // the moment it is asked in.
            assert_eq!(declared, declaration_for(action));
        }
    }

    #[test]
    fn the_folder_and_the_placement_are_one_unit() {
        assert_eq!(
            declaration_for(Action::CreatingTheJobOutputFolder).kind,
            Kind::OneUnitWithThePlacement
        );
        assert_eq!(
            declaration_for(Action::PlacingAJobsMaterialOnTheMachineAtSubmission).kind,
            Kind::AtomicEntire
        );
    }

    #[test]
    fn every_action_says_where_its_act_is_and_none_of_the_eight_answers_later_and_stops() {
        // Three acts are here whole; one is here in part and says which part;
        // and the four that are not here name the step they land at. **The
        // clearing act is the one neither of the first two values is honest
        // about** — the sweep is built here and `Queue & History` X2's erase is
        // not this document's at all.
        assert_eq!(
            EVERY_ACTION.map(|action| declaration_for(action).act),
            [
                WhereTheActIs::BuiltHere,
                WhereTheActIs::BuiltHere,
                WhereTheActIs::ItArrivesAtStep(7),
                WhereTheActIs::ItArrivesAtStep(7),
                WhereTheActIs::BuiltHere,
                WhereTheActIs::ItArrivesAtStep(7),
                WhereTheActIs::ItArrivesAtStep(8),
                WhereTheActIs::TheSweepIsHereAndTheRestIsAnotherDocumentsAct,
            ]
        );
        // And no act that is not here answers with a step outside the plan's
        // own range for this document's remaining parts.
        for action in EVERY_ACTION {
            if let WhereTheActIs::ItArrivesAtStep(step) = declaration_for(action).act {
                assert!((6..=8).contains(&step), "{action:?} lands at step {step}");
            }
        }
    }

    #[test]
    fn the_clearing_act_is_neither_built_here_nor_waiting_on_a_step() {
        // **Marking it *built here* would be false in the other direction**:
        // `Queue & History` X2's machine-side erase and the record directory's
        // own removal are not this crate's, and no later step of the Plan
        // brings them here.
        assert_eq!(
            declaration_for(Action::ClearingAJobsMachineSideMaterial).act,
            WhereTheActIs::TheSweepIsHereAndTheRestIsAnotherDocumentsAct
        );
        // The half that is here is exactly the declared shape.
        assert_eq!(
            declaration_for(Action::ClearingAJobsMachineSideMaterial).kind,
            Kind::PartialAcrossAtomicWithin
        );
    }
}

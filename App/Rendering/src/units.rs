//! Declarations — atomic or partial, and arbitration. `Rendering` §7.
//!
//! **Declared per action against `State, identity and recovery` §9, with
//! the test that decided each. Nothing below is decided at run time.**
//!
//! **Arbitration** (`Rented-server runtime` A1): the render kind's
//! declarations are K1's ([`crate::kind`]); every other action this
//! document owns on the machine is a command that starts no worker and
//! takes no contended resource; the reading and version-confirmation
//! invocations of P1 run inside the admitted render operation and contend
//! with nothing.
//!
//! **The transfer class** (`Server protocol and transfers` T8): every
//! transfer a submission act arranges carries the submission class,
//! declared here — the remap-material placement and the submission
//! material's staging and publication — ranked against `File Browser` O4's
//! list **below manual download, above dependency-upload**, by that clause's
//! own provision for a later document's class.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `State, identity and recovery` §9, §9.2,
//! §9.3, §9.4, §9.6. `Server protocol and
//! transfers` T8, C4. `File Browser` O4, O5. `Dependency Upload & Remap`
//! M10. `Job files and outputs` M5. `Queue & History` Q3.
//!
//! **`Rented-server runtime` A1 and `Dependency Upload & Remap` A9 are not
//! listed above and are written out at each site**, because this plan
//! defines an A1 and an A9 of its own.

/// The six actions §7's table declares for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    ASubmissionAct,
    OneSceneJobsSubmission,
    TheFirstFramePairForOneScene,
    OneRenderAttempt,
    AskingForAStop,
    TheCarryOnReading,
}

pub const EVERY_ACTION: [Action; 6] = [
    Action::ASubmissionAct,
    Action::OneSceneJobsSubmission,
    Action::TheFirstFramePairForOneScene,
    Action::OneRenderAttempt,
    Action::AskingForAStop,
    Action::TheCarryOnReading,
];

/// **`State, identity and recovery` §9.2's three tests, named so a
/// declaration says which decided it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Test {
    /// Test 1 — what the promise is about, and nothing smaller is promised.
    ThePromiseIsAboutNothingSmaller,
    /// Test 2 — a half-written thing read later would be wrong.
    AHalfWrittenThingWouldBeWrong,
    /// Test 3 — one part is unsafe without the other, so the ordered set is
    /// one unit.
    OneIsUnsafeWithoutTheOther,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    PartialAcrossAtomicWithin,
    OneOrderedUnit,
    Atomic,
}

/// One row of §7's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declaration {
    pub action: Action,
    pub partition_unit: &'static str,
    pub test: Test,
    pub kind: Kind,
    pub why: &'static str,
}

/// **§7's table, entire.**
pub const THE_TABLE: [Declaration; 6] = [
    Declaration {
        action: Action::ASubmissionAct,
        partition_unit: "one scene-job",
        test: Test::ThePromiseIsAboutNothingSmaller,
        kind: Kind::PartialAcrossAtomicWithin,
        why: "the spec promises *whatever reached the machine stays* about the act and *all or nothing* about the job, so the smallest promised outcome is the scene-job; the §9.4 outcome record enumerates the scene-jobs durably before the first is attempted, the units are attempted serially under S4a, and the unaccounted case is S9's third answer, not a fourth outcome",
    },
    Declaration {
        action: Action::OneSceneJobsSubmission,
        partition_unit: "the ordered unit of S4",
        test: Test::OneIsUnsafeWithoutTheOther,
        kind: Kind::OneOrderedUnit,
        why: "the submission is unsafe unless the remap material is placed, and the record unsafe unless the material published: remap material → job material → job record; a failed earlier unit reports the later as not attempted, naming it",
    },
    Declaration {
        action: Action::TheFirstFramePairForOneScene,
        partition_unit: "one job of the pair",
        test: Test::ThePromiseIsAboutNothingSmaller,
        kind: Kind::PartialAcrossAtomicWithin,
        why: "each is an ordinary job with its own identity and folder; the hold instruction rides the second job's unit, so a second job cannot exist unheld",
    },
    Declaration {
        action: Action::OneRenderAttempt,
        partition_unit: "the whole attempt",
        test: Test::OneIsUnsafeWithoutTheOther,
        kind: Kind::OneOrderedUnit,
        why: "the launch is unsafe unless the remap material was read, the version confirmed, and the attempt record written, in that order — P1's steps; a failed earlier step reports every later one as not attempted, naming what stopped it",
    },
    Declaration {
        action: Action::AskingForAStop,
        partition_unit: "the command",
        test: Test::ThePromiseIsAboutNothingSmaller,
        kind: Kind::Atomic,
        why: "the promise is that the stop is asked for — one command record (C4); the machine-side pair of desired-state-then-signal is §9.3's unit, owned there",
    },
    Declaration {
        action: Action::TheCarryOnReading,
        partition_unit: "the whole reading",
        test: Test::AHalfWrittenThingWouldBeWrong,
        kind: Kind::Atomic,
        why: "a partial reading read as whole is a figure vouching for frames nobody counted — complete or not a reading (A8); it is a reading, never a durable write",
    },
];

/// **Arbitration: every other action this document owns on the machine is
/// a command that starts no worker and takes no contended resource**, and
/// the reading and version-confirmation invocations of P1 run inside the
/// admitted render operation and contend with nothing.
pub const EVERY_OTHER_ACTION_HERE_IS_A_COMMAND_THAT_STARTS_NO_WORKER: bool = true;

/// **The transfer class: the submission class, declared here, per action,
/// decided nowhere at run time.** It covers exactly the two transfers of
/// S4's unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheSubmissionClass {
    pub covers: [&'static str; 2],
    /// Against `File Browser` O4's list: below manual download, above
    /// dependency-upload.
    pub ranked_below: &'static str,
    pub ranked_above: &'static str,
}

pub const THE_SUBMISSION_CLASS: TheSubmissionClass = TheSubmissionClass {
    covers: [
        "the remap-material placement (`Dependency Upload & Remap` M10; its `Dependency Upload & Remap` A9 rides the submission's own class)",
        "the submission material's staging and publication (`Job files and outputs` M5)",
    ],
    ranked_below: "manual download",
    ranked_above: "dependency-upload",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_action_is_declared_exactly_once_and_the_table_is_six_rows() {
        assert_eq!(THE_TABLE.len(), 6);
        for action in EVERY_ACTION {
            assert_eq!(
                THE_TABLE.iter().filter(|row| row.action == action).count(),
                1,
                "{action:?}"
            );
        }
    }

    #[test]
    fn the_two_ordered_units_are_the_scene_jobs_submission_and_the_attempt() {
        for row in THE_TABLE {
            match row.action {
                Action::OneSceneJobsSubmission | Action::OneRenderAttempt => {
                    assert_eq!(row.kind, Kind::OneOrderedUnit);
                    assert_eq!(row.test, Test::OneIsUnsafeWithoutTheOther);
                }
                Action::ASubmissionAct | Action::TheFirstFramePairForOneScene => {
                    assert_eq!(row.kind, Kind::PartialAcrossAtomicWithin);
                }
                Action::AskingForAStop | Action::TheCarryOnReading => {
                    assert_eq!(row.kind, Kind::Atomic);
                }
            }
        }
    }

    #[test]
    fn the_submission_class_covers_exactly_the_two_transfers_of_s4s_unit() {
        assert_eq!(THE_SUBMISSION_CLASS.covers.len(), 2);
        assert!(THE_SUBMISSION_CLASS.covers[0].contains("remap-material placement"));
        assert!(THE_SUBMISSION_CLASS.covers[1].contains("staging and publication"));
        assert_eq!(THE_SUBMISSION_CLASS.ranked_below, "manual download");
        assert_eq!(THE_SUBMISSION_CLASS.ranked_above, "dependency-upload");
        const _: () = assert!(EVERY_OTHER_ACTION_HERE_IS_A_COMMAND_THAT_STARTS_NO_WORKER);
    }
}

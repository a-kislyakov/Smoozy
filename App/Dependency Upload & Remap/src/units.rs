//! Atomic or partial — `Dependency Upload & Remap` §10: this document's
//! declarations.
//!
//! **Declared per action against `State, identity and recovery` §9, with the
//! test that decided each. Nothing below is decided at run time.** The
//! redirect's atomicity is untouched: `Job files and outputs` Section 12
//! declares the map's application atomic, and the remap pairs riding the same
//! material carry their own declared unit here — two declarations governing
//! two different sets of pairs in one material.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §9, §9.2, §9.4.

/// The eleven actions §10's table declares for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    ACheckSitting,
    OneReading,
    TheUploadSetAfterACheck,
    OneSetRowsSending,
    OneAssetsPlacement,
    OneReferencesResolution,
    AnActTakenOverASelectionOfRows,
    AShiftsApplication,
    ProducingOrReplacingAFixedCopy,
    RemapMaterialPlacementPlusSubmission,
    ApplyingTheRemapAtLaunch,
}

pub const EVERY_ACTION: [Action; 11] = [
    Action::ACheckSitting,
    Action::OneReading,
    Action::TheUploadSetAfterACheck,
    Action::OneSetRowsSending,
    Action::OneAssetsPlacement,
    Action::OneReferencesResolution,
    Action::AnActTakenOverASelectionOfRows,
    Action::AShiftsApplication,
    Action::ProducingOrReplacingAFixedCopy,
    Action::RemapMaterialPlacementPlusSubmission,
    Action::ApplyingTheRemapAtLaunch,
];

/// **`State, identity and recovery` §9.2's tests, named so a declaration says
/// which one decided it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Test {
    /// Test 1 — each is its own promised outcome.
    EachIsItsOwnPromise,
    /// Test 2 — a half-written thing read later would be wrong.
    AHalfWrittenThingWouldBeWrong,
    /// Test 3 — the second is unsafe unless the first is done.
    UnsafeUnlessTheFirstIsDone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Atomic,
    PartialAcrossUnitsAtomicWithinOne,
    /// One ordered unit: the first, then the second, and on the first's
    /// failure the second is not attempted, naming the first.
    OneOrderedUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declaration {
    pub action: Action,
    pub partition_unit: &'static str,
    pub test: Test,
    pub kind: Kind,
    pub why: &'static str,
}

/// **§10's table, entire.**
pub const THE_TABLE: [Declaration; 11] = [
    Declaration {
        action: Action::ACheckSitting,
        partition_unit: "one file's reading",
        test: Test::EachIsItsOwnPromise,
        kind: Kind::PartialAcrossUnitsAtomicWithinOne,
        why: "the spec promises per-row and per-file outcomes, never an outcome about the whole tree; a dropped connection leaves completed readings standing (G16)",
    },
    Declaration {
        action: Action::OneReading,
        partition_unit: "the whole reading",
        test: Test::AHalfWrittenThingWouldBeWrong,
        kind: Kind::Atomic,
        why: "a half-composed reading read as whole is the check lying — sentinel-framed, complete or not a reading",
    },
    Declaration {
        action: Action::TheUploadSetAfterACheck,
        partition_unit: "one row",
        test: Test::EachIsItsOwnPromise,
        kind: Kind::PartialAcrossUnitsAtomicWithinOne,
        why: "the spec's states are per row; what could not be found does not stop the rest. A row's failure names the row and holds up nothing",
    },
    Declaration {
        action: Action::OneSetRowsSending,
        partition_unit: "one member file",
        test: Test::EachIsItsOwnPromise,
        kind: Kind::PartialAcrossUnitsAtomicWithinOne,
        why: "the promise about the set is its state, derived; each member is its own transfer; the row's state derived from members' publications",
    },
    Declaration {
        action: Action::OneAssetsPlacement,
        partition_unit: "the whole placement",
        test: Test::AHalfWrittenThingWouldBeWrong,
        kind: Kind::Atomic,
        why: "a partial mistaken for a placed asset is Fact 18's exact failure — staging and one publication, under the contract",
    },
    Declaration {
        action: Action::OneReferencesResolution,
        partition_unit: "the record write",
        test: Test::EachIsItsOwnPromise,
        kind: Kind::Atomic,
        why: "one durable write in the local record",
    },
    Declaration {
        action: Action::AnActTakenOverASelectionOfRows,
        partition_unit: "one row",
        test: Test::EachIsItsOwnPromise,
        kind: Kind::PartialAcrossUnitsAtomicWithinOne,
        why: "the row is the smallest thing the spec or Design promises an outcome about, and nothing anywhere promises one about the selection — N independent acts rather than one action, so §9.4 owes no outcome record and none is written (F22, F23)",
    },
    Declaration {
        action: Action::AShiftsApplication,
        partition_unit: "one row's resolution",
        test: Test::EachIsItsOwnPromise,
        kind: Kind::PartialAcrossUnitsAtomicWithinOne,
        why: "each resolved row is its own promised outcome; the one-action undo is the shift record's enumeration (F20); stopped part-way, the undo still catches everything applied",
    },
    Declaration {
        action: Action::ProducingOrReplacingAFixedCopy,
        partition_unit: "the whole copy",
        test: Test::AHalfWrittenThingWouldBeWrong,
        kind: Kind::Atomic,
        why: "a half-fixed library read by a render is a silently wrong render — staged, saved, published by one rename (M14, M16)",
    },
    Declaration {
        action: Action::RemapMaterialPlacementPlusSubmission,
        partition_unit: "the ordered pair",
        test: Test::UnsafeUnlessTheFirstIsDone,
        kind: Kind::OneOrderedUnit,
        why: "the submission is unsafe unless the material is placed: material first; on failure the submission is not attempted, naming the material (M11)",
    },
    Declaration {
        action: Action::ApplyingTheRemapAtLaunch,
        partition_unit: "one pair",
        test: Test::EachIsItsOwnPromise,
        kind: Kind::PartialAcrossUnitsAtomicWithinOne,
        why: "the spec's own iteration rule makes each reference's remap its own outcome: an edited reference is an addition, not a failure of the rest (M8) — each pair applied-and-reloaded or reported not applied; the marker carries every pair's outcome",
    },
];

/// **§9.5's rule, kept here as well as at each act: no unit is retried inside
/// its action** — a retry is a new action with its own identity (A11, M16a,
/// F26).
pub const NO_UNIT_IS_RETRIED_INSIDE_ITS_ACTION: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_action_is_declared_exactly_once() {
        for action in EVERY_ACTION {
            assert_eq!(
                THE_TABLE.iter().filter(|row| row.action == action).count(),
                1,
                "{action:?}"
            );
        }
        assert_eq!(THE_TABLE.len(), 11);
    }

    #[test]
    fn the_test_decides_the_kind() {
        for row in THE_TABLE {
            match row.test {
                Test::AHalfWrittenThingWouldBeWrong => {
                    assert_eq!(row.kind, Kind::Atomic, "{:?}", row.action)
                }
                Test::UnsafeUnlessTheFirstIsDone => {
                    assert_eq!(row.kind, Kind::OneOrderedUnit, "{:?}", row.action)
                }
                Test::EachIsItsOwnPromise => assert!(
                    matches!(
                        row.kind,
                        Kind::PartialAcrossUnitsAtomicWithinOne | Kind::Atomic
                    ),
                    "{:?}",
                    row.action
                ),
            }
        }
        assert_eq!(
            THE_TABLE
                .iter()
                .filter(|row| row.kind == Kind::OneOrderedUnit)
                .map(|row| row.action)
                .collect::<Vec<_>>(),
            vec![Action::RemapMaterialPlacementPlusSubmission]
        );
    }
}

//! §14 — atomic or partial: this document's own declarations.
//!
//! **Declared per action against `State, identity and recovery` §9, with the test
//! that decided each. Nothing below is decided at run time.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §9, §9.2 tests 1, 2
//! and 3, §9.5, §9.6.

/// The seven actions this document declares for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    ThePackageTransaction,
    WritingTheInstallationIdentityWithTheCommit,
    PlacingAHold,
    ReleasingAHold,
    StoppingAWorker,
    CapturingARendersStreams,
    StartingAWorker,
}

impl Action {
    pub const EVERY_ACTION: [Action; 7] = [
        Action::ThePackageTransaction,
        Action::WritingTheInstallationIdentityWithTheCommit,
        Action::PlacingAHold,
        Action::ReleasingAHold,
        Action::StoppingAWorker,
        Action::CapturingARendersStreams,
        Action::StartingAWorker,
    ];
}

/// **`State, identity and recovery` §9.2's three tests, named so a declaration
/// says which one decided it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Test {
    /// **Test 1** — what the promise is about, and nothing smaller is promised.
    ThePromiseIsAboutNothingSmaller,
    /// **Test 2** — a half-written thing read later would be wrong.
    AHalfWrittenThingWouldBeWrong,
    /// **Test 3** — one part is unsafe without the other, so the ordered pair is
    /// one unit.
    OneIsUnsafeWithoutTheOther,
}

/// What the declaration comes out as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// **Atomic entire.**
    AtomicEntire,
    /// **One unit**, by ordering.
    OneUnit,
    /// **Atomic**, by staging and one publication.
    Atomic,
    /// **Partial across the two streams, atomic within one outcome.**
    PartialAcrossTheTwoStreamsAtomicWithinOneOutcome,
}

/// **What happens where the unit cannot be completed** — written out per action,
/// because it is the half a builder acts on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereItCannotBeDone {
    /// G7's single deciding write is the commit; **test 2 forbids reaching it by
    /// ordering**, and Fact 78's negative two-file half is why the registration is
    /// made version-independent rather than written beside the pointer.
    TheCommitIsOneWriteAndIsNotReachedByOrdering,
    /// **G10 orders the identity first; where it cannot be written the pointer is
    /// *not attempted*, naming the identity as the unit that stopped it.**
    ThePointerIsNotAttemptedNamingTheIdentity,
    /// **X8 step 2 before step 4; where the desired state cannot be written, no
    /// signal is sent.**
    NoSignalIsSent,
    /// **W3 and M1 before the start; where either fails the worker is *not
    /// attempted*, naming what stopped it.**
    TheWorkerIsNotAttemptedNamingWhatStoppedIt,
    /// **The single write is what releases; nothing is started before it and the
    /// started work is a separate action with its own identity.**
    NothingIsStartedBeforeTheWrite,
    /// **One stream's collection failing does not abandon the other, and M18's
    /// write is one write.**
    TheOtherStreamIsUnaffected,
    /// A half-written hold read later would name a subject with no reason or a
    /// reason with no subject, **so it is staged and published in one act.**
    StagedAndPublishedInOneAct,
}

/// One declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declaration {
    pub action: Action,
    /// The partition unit, in the plan's own words.
    pub partition_unit: &'static str,
    pub test: Test,
    pub kind: Kind,
    pub where_it_cannot_be_done: WhereItCannotBeDone,
}

/// **§14's table, and nothing below is decided at run time.**
pub fn declaration_for(action: Action) -> Declaration {
    match action {
        Action::ThePackageTransaction => Declaration {
            action,
            partition_unit: "the whole transaction",
            test: Test::ThePromiseIsAboutNothingSmaller,
            kind: Kind::AtomicEntire,
            where_it_cannot_be_done:
                WhereItCannotBeDone::TheCommitIsOneWriteAndIsNotReachedByOrdering,
        },
        Action::WritingTheInstallationIdentityWithTheCommit => Declaration {
            action,
            partition_unit: "one unit with the transaction",
            test: Test::OneIsUnsafeWithoutTheOther,
            kind: Kind::OneUnit,
            where_it_cannot_be_done: WhereItCannotBeDone::ThePointerIsNotAttemptedNamingTheIdentity,
        },
        Action::PlacingAHold => Declaration {
            action,
            partition_unit: "the whole hold record",
            test: Test::AHalfWrittenThingWouldBeWrong,
            kind: Kind::Atomic,
            where_it_cannot_be_done: WhereItCannotBeDone::StagedAndPublishedInOneAct,
        },
        Action::ReleasingAHold => Declaration {
            action,
            partition_unit: "the release outcome",
            test: Test::ThePromiseIsAboutNothingSmaller,
            kind: Kind::Atomic,
            where_it_cannot_be_done: WhereItCannotBeDone::NothingIsStartedBeforeTheWrite,
        },
        Action::StoppingAWorker => Declaration {
            action,
            partition_unit: "the whole stop",
            test: Test::OneIsUnsafeWithoutTheOther,
            kind: Kind::OneUnit,
            where_it_cannot_be_done: WhereItCannotBeDone::NoSignalIsSent,
        },
        Action::CapturingARendersStreams => Declaration {
            action,
            partition_unit: "one stream",
            test: Test::ThePromiseIsAboutNothingSmaller,
            kind: Kind::PartialAcrossTheTwoStreamsAtomicWithinOneOutcome,
            where_it_cannot_be_done: WhereItCannotBeDone::TheOtherStreamIsUnaffected,
        },
        Action::StartingAWorker => Declaration {
            action,
            partition_unit: "the whole start",
            test: Test::OneIsUnsafeWithoutTheOther,
            kind: Kind::OneUnit,
            where_it_cannot_be_done: WhereItCannotBeDone::TheWorkerIsNotAttemptedNamingWhatStoppedIt,
        },
    }
}

/// **Y1. No unit here is retried inside its action** (`State, identity and
/// recovery` §9.5).
///
/// **A failed transaction is a fresh replacement under G17; a failed stop is a new
/// stop command with a new identity.**
pub const NO_UNIT_HERE_IS_RETRIED_INSIDE_ITS_ACTION: bool = true;

/// **Y2. Two units above rest on Fact 78 and are written as atomic because `State,
/// identity and recovery` §9.6 says so** — **the package transaction and the hold
/// record.** §16 item 3 states what breaks.
pub const THE_TWO_UNITS_THAT_REST_ON_FACT_78: [Action; 2] =
    [Action::ThePackageTransaction, Action::PlacingAHold];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_action_has_a_declaration_with_a_unit_a_test_and_a_kind() {
        for action in Action::EVERY_ACTION {
            let declaration = declaration_for(action);
            assert_eq!(declaration.action, action);
            assert!(!declaration.partition_unit.is_empty());
        }
        // **The list is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so an action added to the
        // enum has none, the crate stops compiling, and whoever added it is sent
        // to this list — where the length assertion this replaced could never
        // have been false, the array's own type having fixed it.
        for (at, action) in Action::EVERY_ACTION.iter().enumerate() {
            let where_the_list_puts_it = match action {
                Action::ThePackageTransaction => 0,
                Action::WritingTheInstallationIdentityWithTheCommit => 1,
                Action::PlacingAHold => 2,
                Action::ReleasingAHold => 3,
                Action::StoppingAWorker => 4,
                Action::CapturingARendersStreams => 5,
                Action::StartingAWorker => 6,
            };
            assert_eq!(at, where_the_list_puts_it, "{action:?} is not where the list puts it");
        }
    }

    #[test]
    fn the_package_transaction_is_atomic_entire_and_is_not_reached_by_ordering() {
        let declaration = declaration_for(Action::ThePackageTransaction);
        assert_eq!(declaration.kind, Kind::AtomicEntire);
        assert_eq!(
            declaration.where_it_cannot_be_done,
            WhereItCannotBeDone::TheCommitIsOneWriteAndIsNotReachedByOrdering
        );
    }

    #[test]
    fn capturing_is_partial_across_the_two_streams_and_atomic_within_one_outcome() {
        let declaration = declaration_for(Action::CapturingARendersStreams);
        assert_eq!(
            declaration.kind,
            Kind::PartialAcrossTheTwoStreamsAtomicWithinOneOutcome
        );
        assert_eq!(declaration.partition_unit, "one stream");
    }

    #[test]
    fn the_three_ordered_pairs_all_refuse_the_second_half_where_the_first_fails() {
        for action in [
            Action::WritingTheInstallationIdentityWithTheCommit,
            Action::StoppingAWorker,
            Action::StartingAWorker,
        ] {
            let declaration = declaration_for(action);
            assert_eq!(declaration.kind, Kind::OneUnit);
            assert_eq!(declaration.test, Test::OneIsUnsafeWithoutTheOther);
        }
    }

    #[test]
    fn the_two_units_resting_on_fact_seventy_eight_are_the_transaction_and_the_hold() {
        // **The two are named rather than counted**, a count over an array
        // whose type fixes its length being an assertion that cannot be false.
        for (at, action) in THE_TWO_UNITS_THAT_REST_ON_FACT_78.iter().enumerate() {
            let where_the_list_puts_it = match action {
                Action::ThePackageTransaction => 0,
                Action::PlacingAHold => 1,
                other => panic!("{other:?} does not rest on that fact and is on the list"),
            };
            assert_eq!(at, where_the_list_puts_it, "{action:?} is not where the list puts it");
        }
        for action in THE_TWO_UNITS_THAT_REST_ON_FACT_78 {
            let declaration = declaration_for(action);
            assert!(matches!(declaration.kind, Kind::AtomicEntire | Kind::Atomic));
        }
    }
}

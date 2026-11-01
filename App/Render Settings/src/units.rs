//! Declarations — atomic or partial. `Render Settings` §11.
//!
//! **Declared per action against `State, identity and recovery` §9, with the
//! test that decided each. Nothing below is decided at run time.**
//!
//! **Arbitration** (`Rented-server runtime` A1): the file-inspection kind's
//! declarations are §10's ([`crate::declarations`]), A1's five with the fifth
//! left unlimited. Q14's comparison read is not a worker and is not admitted
//! (Q15); the version-list ask is `Server Setup`'s search, declared there;
//! everything else this document does on the machine rides existing
//! operations.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §9, §9.2.
//! `Rented-server runtime` A1.

/// The six actions §11's table declares for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    OneReading,
    ReplacingThePagesRecordedReading,
    AStoreWrite,
    TheTidy,
    ComposingThePayload,
    AnchoringAtTheAttempt,
}

/// **`State, identity and recovery` §9.2's tests, named so a declaration says
/// which one decided it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Test {
    /// **Test 1** — each answer, each name, is its own promise.
    EachIsItsOwnPromise,
    /// **Test 2** — a half-written thing read later would be wrong.
    AHalfWrittenThingWouldBeWrong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Atomic,
    AtomicPerAct,
    /// Per name; a failed anchor fails the attempt whole (Y14), the attempt
    /// being `Rendering`'s ordered unit.
    PerNameAndAFailedOneFailsTheAttemptWhole,
}

/// One row of §11's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declaration {
    pub action: Action,
    pub partition_unit: &'static str,
    pub test: Test,
    pub kind: Kind,
    pub why: &'static str,
}

/// **§11's table, entire.**
pub const THE_TABLE: [Declaration; 6] = [
    Declaration {
        action: Action::OneReading,
        partition_unit: "the whole reading",
        test: Test::AHalfWrittenThingWouldBeWrong,
        kind: Kind::Atomic,
        why: "a half-composed reading read as whole is the page lying about the file — sentinel-framed, complete with an empty error map, or not a reading (Q7)",
    },
    Declaration {
        action: Action::ReplacingThePagesRecordedReading,
        partition_unit: "the (server, file) reading",
        test: Test::AHalfWrittenThingWouldBeWrong,
        kind: Kind::Atomic,
        why: "a merged reading shows one file's scenes beside another's values — replaced whole or left standing (Q6)",
    },
    Declaration {
        action: Action::AStoreWrite,
        partition_unit: "the entries the act names — an answer, a clear, the hand-everything-back",
        test: Test::EachIsItsOwnPromise,
        kind: Kind::AtomicPerAct,
        why: "each answer is its own promise; the reset is one act over a named set — volatile, single-writer, no partial visible",
    },
    Declaration {
        action: Action::TheTidy,
        partition_unit: "the typed text's parse",
        test: Test::AHalfWrittenThingWouldBeWrong,
        kind: Kind::Atomic,
        why: "a half-parsed list counted whole is a wrong count beside a right-looking field — parsed whole or refused whole, nothing stored on refusal (F1)",
    },
    Declaration {
        action: Action::ComposingThePayload,
        partition_unit: "the payload",
        test: Test::AHalfWrittenThingWouldBeWrong,
        kind: Kind::Atomic,
        why: "a payload missing entries reads as *the file's own* for things the user answered — composed whole in one act at submission from the store, plus the resolved version pick (B4), which is on every payload and is nobody's answer; immutable after (Y2)",
    },
    Declaration {
        action: Action::AnchoringAtTheAttempt,
        partition_unit: "one name",
        test: Test::EachIsItsOwnPromise,
        kind: Kind::PerNameAndAFailedOneFailsTheAttemptWhole,
        why: "each name is its own promise, and one failing must name itself (Y14)",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_action_is_declared_exactly_once() {
        let every = [
            Action::OneReading,
            Action::ReplacingThePagesRecordedReading,
            Action::AStoreWrite,
            Action::TheTidy,
            Action::ComposingThePayload,
            Action::AnchoringAtTheAttempt,
        ];
        for action in every {
            assert_eq!(
                THE_TABLE.iter().filter(|row| row.action == action).count(),
                1,
                "{action:?}"
            );
        }
    }

    #[test]
    fn the_two_per_promise_rows_are_the_store_write_and_the_anchor_and_the_rest_are_whole() {
        for row in THE_TABLE {
            match row.action {
                Action::AStoreWrite | Action::AnchoringAtTheAttempt => {
                    assert_eq!(row.test, Test::EachIsItsOwnPromise)
                }
                _ => {
                    assert_eq!(row.test, Test::AHalfWrittenThingWouldBeWrong);
                    assert_eq!(row.kind, Kind::Atomic);
                }
            }
        }
    }
}

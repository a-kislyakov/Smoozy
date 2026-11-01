//! The declarations — `Download` §12.
//!
//! **Per `State, identity and recovery` §9.2 — each action's partition unit
//! and the test that decided it. Nothing below is decided at run time.**
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §1.1, §9.2, §9.4,
//! §9.5, §11.1. `File Browser` O4.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Rented-server runtime` W1 is the
//! one that arises here.

/// **Which of `State, identity and recovery` §9.2's ordered tests decided a
/// partition unit.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichTestDecidedIt {
    /// Test 1 alone.
    One,
    /// Test 1, then test 2 within the unit.
    OneThenTwoWithinTheUnit,
}

/// **One row of §12's table**: an action, its partition unit, the test that
/// decided it, and what kind of action it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct APartitionUnit {
    pub action: &'static str,
    pub unit: &'static str,
    pub test: WhichTestDecidedIt,
    pub kind: &'static str,
}

/// **§12's table, whole. Nothing below is decided at run time.**
pub const THE_PARTITION_UNITS: [APartitionUnit; 6] = [
    APartitionUnit {
        action: "a manual download request",
        unit: "one file's transfer",
        test: WhichTestDecidedIt::OneThenTwoWithinTheUnit,
        kind: "partial across files, atomic within one — staging and a single \
               publication; the request's outcome is the enumeration's list",
    },
    APartitionUnit {
        action: "the request enumeration",
        unit: "the whole question",
        test: WhichTestDecidedIt::One,
        kind: "a reading; complete when every branch ended in a listing or a \
               recorded refusal — refused subtrees are recorded outcomes \
               inside a complete enumeration, never a partial one",
    },
    APartitionUnit {
        action: "an auto-download fetch",
        unit: "one file's transfer",
        test: WhichTestDecidedIt::OneThenTwoWithinTheUnit,
        kind: "each fetch is its own action arranged by the standing \
               arrangement; a failed unit is re-arranged as a new action, \
               never retried inside one",
    },
    APartitionUnit {
        action: "the walk",
        unit: "the walk",
        test: WhichTestDecidedIt::One,
        kind: "a reading; complete or discarded",
    },
    APartitionUnit {
        action: "the destination probe",
        unit: "the probe",
        test: WhichTestDecidedIt::One,
        kind: "the arrangement's staging directory created inside the \
               destination folder where it is absent, then one real write into \
               it, both removed at once where the probe created the directory, \
               and the byte alone where it already stood",
    },
    APartitionUnit {
        action: "switching an arrangement on or off, and a narrowing change",
        unit: "the record write",
        test: WhichTestDecidedIt::One,
        kind: "atomic entire — one durable write of the arrangement record",
    },
];

/// The partition unit declared for an action, by the action's own name.
pub fn the_partition_unit_of(action: &str) -> Option<&'static APartitionUnit> {
    THE_PARTITION_UNITS
        .iter()
        .find(|declared| declared.action == action)
}

/// **§12: the two transfer-priority classes are declared at Q3 and ranked by
/// `File Browser` O4.** They are not partition units.
pub const THE_TWO_TRANSFER_PRIORITY_CLASSES_ARE_DECLARED_AT_Q3: bool = true;

/// **§12: no suppression row exists.**
///
/// **Nothing in this document is a dismissible warning** — the trouble
/// conditions are failure records under the whole-app rules, and the vouching
/// and left-alone tellings are per-request facts, not warnings with an off
/// switch.
pub const NO_SUPPRESSION_ROW_EXISTS: bool = true;

/// **§12: no worker kind is declared or wanted.**
///
/// The walk and the attempt-record read are readings on the subsystem and
/// command routes, a fetch is a transfer channel whose machine half is the
/// existing transfer-support kind, and the probe is a local write —
/// **`Rented-server runtime` W1's closed set is untouched.**
pub const NO_WORKER_KIND_IS_DECLARED_OR_WANTED: bool = true;

/// **§12's lifetimes, named against `State, identity and recovery` §11.1.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatIsHeldHowLong {
    /// **The arrangement record and its reckoning — user-held**, the class
    /// `State, identity and recovery` §11.1 lists by name and which its §1.1
    /// inventory already names *transfer arrangements*, riding the job's
    /// entry.
    TheArrangementRecordAndItsReckoning,
    /// **The manual request record — user-held.**
    TheManualRequestRecord,
    /// **Walks, judgements, frontiers — period-held or shorter, never
    /// durable.**
    WalksJudgementsAndFrontiers,
    /// **Staged material — action-held**, under the transfer contract's
    /// cleanup.
    StagedMaterial,
    /// **The transfer records are `Server protocol and transfers`' and are not
    /// re-homed.**
    TheTransferRecords,
}

impl WhatIsHeldHowLong {
    pub const ALL: [WhatIsHeldHowLong; 5] = [
        WhatIsHeldHowLong::TheArrangementRecordAndItsReckoning,
        WhatIsHeldHowLong::TheManualRequestRecord,
        WhatIsHeldHowLong::WalksJudgementsAndFrontiers,
        WhatIsHeldHowLong::StagedMaterial,
        WhatIsHeldHowLong::TheTransferRecords,
    ];

    /// The `State, identity and recovery` §11.1 class, in that document's own
    /// words.
    pub fn the_class(self) -> &'static str {
        match self {
            WhatIsHeldHowLong::TheArrangementRecordAndItsReckoning
            | WhatIsHeldHowLong::TheManualRequestRecord => "user-held",
            WhatIsHeldHowLong::WalksJudgementsAndFrontiers => "period-held or shorter",
            WhatIsHeldHowLong::StagedMaterial => "action-held",
            WhatIsHeldHowLong::TheTransferRecords => {
                "`Server protocol and transfers`', not re-homed"
            }
        }
    }

    /// **Whether anything of this crate's own writes it durably.** Two of the
    /// five are durable and the rest are not, which is R5's rule read from the
    /// other side.
    pub fn is_durable(self) -> bool {
        matches!(
            self,
            WhatIsHeldHowLong::TheArrangementRecordAndItsReckoning
                | WhatIsHeldHowLong::TheManualRequestRecord
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_action_this_document_declares_has_exactly_one_partition_unit() {
        for declared in THE_PARTITION_UNITS {
            assert_eq!(
                the_partition_unit_of(declared.action),
                Some(&declared),
                "{}",
                declared.action
            );
        }
        assert!(the_partition_unit_of("an upload request").is_none());
    }

    #[test]
    fn the_two_readings_take_test_one_alone_and_the_two_transfers_take_both() {
        assert_eq!(
            the_partition_unit_of("the walk").unwrap().test,
            WhichTestDecidedIt::One
        );
        assert_eq!(
            the_partition_unit_of("the request enumeration")
                .unwrap()
                .test,
            WhichTestDecidedIt::One
        );
        assert_eq!(
            the_partition_unit_of("a manual download request")
                .unwrap()
                .test,
            WhichTestDecidedIt::OneThenTwoWithinTheUnit
        );
        assert_eq!(
            the_partition_unit_of("an auto-download fetch")
                .unwrap()
                .test,
            WhichTestDecidedIt::OneThenTwoWithinTheUnit
        );
    }

    #[test]
    fn only_the_two_records_are_durable_and_no_judgement_is() {
        let durable: Vec<WhatIsHeldHowLong> = WhatIsHeldHowLong::ALL
            .into_iter()
            .filter(|held| held.is_durable())
            .collect();
        assert_eq!(
            durable,
            vec![
                WhatIsHeldHowLong::TheArrangementRecordAndItsReckoning,
                WhatIsHeldHowLong::TheManualRequestRecord
            ]
        );
        assert_eq!(
            WhatIsHeldHowLong::WalksJudgementsAndFrontiers.the_class(),
            "period-held or shorter"
        );
    }
}

//! Declarations — atomic or partial, arbitration, and the one termination
//! contract. `Queue & History` §16.
//!
//! **Declared per action against `State, identity and recovery` §9, with the
//! test that decided each. Nothing below is decided at run time.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` M5, B24, B29.
//!
//! **Every `Rented-server runtime` marker used below — its arbitration
//! clause, its displaced-operation clause, its ordering clause and its four
//! execution-group clauses — is written out at the site and not listed
//! above**, this plan defining a clause of each name.

/// The thirteen actions §16's table declares for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    WritingAJobsQueueRecordAndItsNoteAtSubmission,
    CancelOfOneWaitingOrHeldJob,
    ReorderOfOneJob,
    GroupBlockMove,
    StopAndClear,
    TheClearInsideIt,
    DeletingAHistoryEntry,
    TheOwedMachineSideErase,
    AdoptionAccept,
    ApplyingCatchUpEvents,
    FetchingPreviewDerivatives,
    WritingTheIdleSpokenMark,
    ClearingHistory,
}

pub const EVERY_ACTION: [Action; 13] = [
    Action::WritingAJobsQueueRecordAndItsNoteAtSubmission,
    Action::CancelOfOneWaitingOrHeldJob,
    Action::ReorderOfOneJob,
    Action::GroupBlockMove,
    Action::StopAndClear,
    Action::TheClearInsideIt,
    Action::DeletingAHistoryEntry,
    Action::TheOwedMachineSideErase,
    Action::AdoptionAccept,
    Action::ApplyingCatchUpEvents,
    Action::FetchingPreviewDerivatives,
    Action::WritingTheIdleSpokenMark,
    Action::ClearingHistory,
];

/// **`State, identity and recovery` §9.2's tests, and §9.3's rule, named so
/// a declaration says which decided it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Test {
    /// §9.2 test 1 — the promise is about the whole thing.
    ThePromiseIsAboutTheWholeThing,
    /// §9.2 test 2 — a half-written thing read later would be wrong.
    AHalfWrittenThingReadLaterWouldBeWrong,
    /// §9.3 — an ordered set whose second is unsafe without the first.
    AnOrderedSetIsOneUnit,
    /// `State, identity and recovery` §4's own rule for applying events.
    EachEventAppliedIdempotently,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Atomic,
    AtomicLocally,
    AtomicEntire,
    OneUnitAcrossThePair,
    PartialAcrossAtomicWithin,
    /// **The shape §16 gives an act that spans several job records: partial
    /// across records, atomic within each removal or rewrite, made whole by a
    /// durable record of the act.** Added 2026-09-05, out of a code round: the
    /// three rows carrying it read **atomic**, and `State, identity and
    /// recovery` §9.6 P8 settles that no single filesystem operation updates
    /// two independent files atomically, so the declaration was unbuildable
    /// rather than merely unbuilt.
    PartialAcrossRecordsMadeWholeByARecordOfTheAct,
}

/// One row of §16's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Declaration {
    pub action: Action,
    pub partition_unit: &'static str,
    pub test: Test,
    pub kind: Kind,
    /// Whether a `State, identity and recovery` §9.4 outcome record is
    /// stated for it, and where its units come from.
    pub outcome_record: Option<&'static str>,
    pub why: &'static str,
}

/// **§16's table, entire.**
pub const THE_TABLE: [Declaration; 13] = [
    Declaration {
        action: Action::WritingAJobsQueueRecordAndItsNoteAtSubmission,
        partition_unit: "the pair",
        test: Test::AnOrderedSetIsOneUnit,
        kind: Kind::Atomic,
        outcome_record: None,
        why: "the note is unsafe as an absence, because N2 gives it no second creation moment; the two are staged and brought into being by a single publication step in the shape `Job files and outputs` M5 already uses, never by ordering and never by care (Q3, N1)",
    },
    Declaration {
        action: Action::CancelOfOneWaitingOrHeldJob,
        partition_unit: "the job's leaving the line",
        test: Test::ThePromiseIsAboutTheWholeThing,
        kind: Kind::Atomic,
        outcome_record: None,
        why: "one durable removal of the job record (L1); the material cleanup is `Job files and outputs`' declared partial action, sequenced after, and can restore nothing",
    },
    Declaration {
        action: Action::ReorderOfOneJob,
        partition_unit: "one job record's key",
        test: Test::ThePromiseIsAboutTheWholeThing,
        kind: Kind::PartialAcrossRecordsMadeWholeByARecordOfTheAct,
        outcome_record: Some("L24's record carries the whole intended key assignment, written before the first key moves and cleared only when the last moved key is written; D14 finishes an outstanding one before the admission point may start anything"),
        why: "the move spans several job records and this store is one file per record (Q3, Q6), so the one durable rewrite this row used to declare cannot exist — `State, identity and recovery` §9.6 P8 (L6, L24, D14)",
    },
    Declaration {
        action: Action::GroupBlockMove,
        partition_unit: "one member's key",
        test: Test::AnOrderedSetIsOneUnit,
        kind: Kind::PartialAcrossRecordsMadeWholeByARecordOfTheAct,
        outcome_record: Some("the same L24 record, carrying the gathered block's intended keys with every other waiting record's"),
        why: "a half-moved group could render in a mixed order, so the block's intended order is one unit — and what makes it one is the record rather than the write, the block being several job records in a store of one file per record (L7, L24, D14); a member no longer waiting is named in the outcome as not moved",
    },
    Declaration {
        action: Action::StopAndClear,
        partition_unit: "the ordered pair",
        test: Test::AnOrderedSetIsOneUnit,
        kind: Kind::OneUnitAcrossThePair,
        outcome_record: Some("the desktop-side outcome record holds both halves' outcomes durably before the first is attempted (L17)"),
        why: "the stop is unsafe unless the clear succeeded: clear first, one durable write emptying the line (L14); where it fails the stop is not attempted, naming the clear (L16)",
    },
    Declaration {
        action: Action::TheClearInsideIt,
        partition_unit: "one covered record's removal",
        test: Test::AHalfWrittenThingReadLaterWouldBeWrong,
        kind: Kind::PartialAcrossRecordsMadeWholeByARecordOfTheAct,
        outcome_record: Some("L23's record names every identity the clear covers and is written before the first removal, each outcome written as it becomes true, the record cleared only when the last covered record is gone; D14 finishes an outstanding one before the admission point may start anything"),
        why: "a half-cleared line read by the admission point is the failure the order exists to prevent, and what shuts that window is L23's record rather than one write — the clear spans every covered job record and this store is one file per record (L14, L15, L23, D14)",
    },
    Declaration {
        action: Action::DeletingAHistoryEntry,
        partition_unit: "record-plus-tombstone",
        test: Test::AnOrderedSetIsOneUnit,
        kind: Kind::AtomicLocally,
        outcome_record: None,
        why: "a record gone with no tombstone is a deletion a later connection undoes (X1)",
    },
    Declaration {
        action: Action::TheOwedMachineSideErase,
        partition_unit: "one material kind",
        test: Test::ThePromiseIsAboutTheWholeThing,
        kind: Kind::PartialAcrossAtomicWithin,
        outcome_record: Some("in the local record, its units X2's enumerated kinds — the hold record among them — durable before the first is attempted, each carrying done, failed or not attempted; an already-absent unit is done; the remainder is a new action composed from this record"),
        why: "the promise names what the erase reaches, per kind; the tombstone's flag is set only on all units done (X3)",
    },
    Declaration {
        action: Action::AdoptionAccept,
        partition_unit: "the whole offer",
        test: Test::ThePromiseIsAboutTheWholeThing,
        kind: Kind::AtomicEntire,
        outcome_record: None,
        why: "the offer is one per machine, all or nothing — every record entered in one local write (A6)",
    },
    Declaration {
        action: Action::ApplyingCatchUpEvents,
        partition_unit: "one event",
        test: Test::EachEventAppliedIdempotently,
        kind: Kind::PartialAcrossAtomicWithin,
        outcome_record: Some("its units are the events the pass reads forward over, enumerated from the cursor before the first is applied, each carrying its own outcome, the action's own outcome derived from them"),
        why: "each applied idempotently, cursor advanced after each application is durable; the notice's composition is part of each application (C2), and a gap is stated (C4)",
    },
    Declaration {
        action: Action::FetchingPreviewDerivatives,
        partition_unit: "one derivative",
        test: Test::ThePromiseIsAboutTheWholeThing,
        kind: Kind::PartialAcrossAtomicWithin,
        outcome_record: Some("its units are the derivatives the pass learned of, enumerated before the first fetch is attempted, each carrying its own outcome, the action's own outcome derived from them"),
        why: "the transfer contract makes each atomic (P4)",
    },
    Declaration {
        action: Action::WritingTheIdleSpokenMark,
        partition_unit: "the mark",
        test: Test::ThePromiseIsAboutTheWholeThing,
        kind: Kind::Atomic,
        outcome_record: None,
        why: "one small write (W5)",
    },
    Declaration {
        action: Action::ClearingHistory,
        partition_unit: "one entry's record-plus-tombstone",
        test: Test::ThePromiseIsAboutTheWholeThing,
        kind: Kind::PartialAcrossAtomicWithin,
        outcome_record: None,
        why: "each entry's deletion is X1's declared unit, and neither act promises anything smaller than an entry or anything that spans two of them; an interrupted clear leaves the rest standing and drawn, the act the user's to take again, and each deleted entry's machine half is its own recorded debt (X6, X6a, X4)",
    },
];

/// **Arbitration (`Rented-server runtime` A1): every operation this document
/// owns on the machine is a command that starts no worker.** A cancel, a
/// clear, a reorder, a block move, an answer, an erase — each is a command
/// record on the supervisor's path, takes no contended resource, and needs
/// no resource class, priority or cancellability declaration. **What this
/// document does own at the arbitration point is `Rented-server runtime`
/// A9's ordering**: what waits behind the exclusive render is ordered by the
/// machine-local queue's sequence keys and by nothing else (Q4). This
/// document adds no worker kind of its own.
pub const EVERY_OPERATION_HERE_IS_A_COMMAND_THAT_STARTS_NO_WORKER: bool = true;

/// **The preview kind's termination contract**, stated because this document
/// is what starts that kind running (P1–P2) and no document had stated one
/// — `Rented-server runtime` X11's four things. The kind's declarations are
/// `Job files and outputs` B24's, and that document is closed: the contract
/// is recorded here and was handed to the runner as a seam (§18 item 14,
/// closed with this section as its home).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThePreviewKindsTerminationContract {
    /// Delivered to the recorded external process — the Blender the worker
    /// drives.
    pub which_member_receives_the_termination: &'static str,
    /// At any moment, cancellation being B24's own declaration.
    pub at_any_moment: bool,
    /// The sweep of the derivative's staging path, so a killed production
    /// never publishes a partial (`Job files and outputs` B29 publishes by
    /// rename, and an unpublished staging is swept material of the shape that
    /// document names).
    pub the_cleanup_owed: &'static str,
    /// The worker reaps it and exits, so the worker outlives every member of
    /// its group — what keeps `Rented-server runtime` X1's leader-only
    /// recording and `Rented-server runtime` X9's leader-gone-means-group-gone sound for this kind.
    pub the_worker_outlives_every_member: bool,
    /// The displaced operation's outcome is its own — cancelled — never a
    /// failure of the work that displaced it (`Rented-server runtime` A6).
    pub a_displaced_productions_outcome_is_cancelled_never_failed: bool,
}

pub const THE_PREVIEW_KINDS_TERMINATION_CONTRACT: ThePreviewKindsTerminationContract =
    ThePreviewKindsTerminationContract {
        which_member_receives_the_termination:
            "the recorded external Blender process the preview worker drives",
        at_any_moment: true,
        the_cleanup_owed: "the sweep of the derivative's staging path",
        the_worker_outlives_every_member: true,
        a_displaced_productions_outcome_is_cancelled_never_failed: true,
    };

impl ThePreviewKindsTerminationContract {
    /// `Rented-server runtime` X11 names four things and this answers every
    /// one.
    pub fn names_all_four(&self) -> bool {
        !self.which_member_receives_the_termination.is_empty()
            && self.at_any_moment
            && !self.the_cleanup_owed.is_empty()
            && self.the_worker_outlives_every_member
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_action_is_declared_exactly_once_and_the_table_is_thirteen_rows() {
        assert_eq!(THE_TABLE.len(), 13);
        for action in EVERY_ACTION {
            assert_eq!(
                THE_TABLE.iter().filter(|row| row.action == action).count(),
                1,
                "{action:?}"
            );
        }
    }

    #[test]
    fn every_partial_action_states_its_outcome_record_or_is_a_set_of_declared_units() {
        for row in THE_TABLE {
            if row.kind == Kind::PartialAcrossAtomicWithin {
                // Clearing history is the one partial action with no record
                // of its own: each entry's deletion is X1's declared unit and
                // carries its own tombstone and debt.
                assert!(
                    row.outcome_record.is_some() || row.action == Action::ClearingHistory,
                    "{:?}",
                    row.action
                );
            }
        }
    }

    #[test]
    fn stop_and_clear_is_one_unit_across_the_pair_and_the_clear_inside_it_is_made_whole_by_a_record(
    ) {
        let pair = THE_TABLE
            .iter()
            .find(|row| row.action == Action::StopAndClear)
            .unwrap();
        assert_eq!(pair.kind, Kind::OneUnitAcrossThePair);
        assert_eq!(pair.test, Test::AnOrderedSetIsOneUnit);
        let clear = THE_TABLE
            .iter()
            .find(|row| row.action == Action::TheClearInsideIt)
            .unwrap();
        assert_eq!(
            clear.kind,
            Kind::PartialAcrossRecordsMadeWholeByARecordOfTheAct
        );
        assert_eq!(clear.test, Test::AHalfWrittenThingReadLaterWouldBeWrong);
    }

    /// **No act that spans several job records is declared atomic**, because
    /// no single filesystem operation updates two independent files atomically
    /// and this store is one file per record.
    #[test]
    fn every_act_spanning_several_job_records_is_made_whole_by_a_record_of_the_act() {
        for action in [
            Action::ReorderOfOneJob,
            Action::GroupBlockMove,
            Action::TheClearInsideIt,
        ] {
            let row = THE_TABLE
                .iter()
                .find(|row| row.action == action)
                .expect("declared");
            assert_eq!(
                row.kind,
                Kind::PartialAcrossRecordsMadeWholeByARecordOfTheAct,
                "{action:?}"
            );
            assert!(
                row.outcome_record.is_some(),
                "{action:?} is made whole by a record and has to name it"
            );
        }
    }

    #[test]
    fn the_preview_kinds_contract_names_all_four_of_x11s_things() {
        assert!(THE_PREVIEW_KINDS_TERMINATION_CONTRACT.names_all_four());
        const _: () = assert!(
            THE_PREVIEW_KINDS_TERMINATION_CONTRACT
                .a_displaced_productions_outcome_is_cancelled_never_failed
        );
        const _: () = assert!(EVERY_OPERATION_HERE_IS_A_COMMAND_THAT_STARTS_NO_WORKER);
    }
}

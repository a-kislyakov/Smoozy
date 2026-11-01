//! §13 — retention on the machine.
//!
//! **`State, identity and recovery` §11.4's one rule is taken whole: nothing on a
//! machine is cleared on the strength of a free-space reading alone.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §4 E5, E6, E6a, §11.4.
//! `Server protocol and transfers` S30. `Security and licence` §10. `Server
//! Setup` B8a, Y6a, Y6b. `Queue & History` — its no-expiry rule.

/// **R1's first column — what this document retains.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Retained {
    /// **The two captured streams and the capture record.**
    TheCapturedStreamsAndTheCaptureRecord,
    /// **An operation record and its command records.**
    AnOperationRecordAndItsCommandRecords,
    /// **The command record of the desktop's cursor move** —
    /// `State, identity and recovery` E3's acknowledgement, **the one command
    /// that acts on no subject**, so the row above cannot end it.
    ///
    /// **Added to R1 on 2026-09-19**, out of step 10c piece 5's finding: every
    /// acknowledgement `Desktop app and delivery` P10d sends left one record on
    /// the machine for the life of the rental.
    TheCommandRecordOfTheCursorMove,
    /// **A setup operation record**, which has its own term (R1's own note).
    ASetupOperationRecord,
    /// **A hold record.**
    AHoldRecord,
    /// **The durable package-phase record.**
    ThePackagePhaseRecord,
    /// **A previous package version.**
    APreviousPackageVersion,
    /// **Staged package material.**
    StagedPackageMaterial,
    /// **The event journal**, whose rule is another document's entirely.
    TheEventJournal,
}

/// **R1's second column — when each ends.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndsWhen {
    /// **The job's machine-side record ends** — the machine-reaching server
    /// removal, the history tombstone's owed erase, or the machine's destruction.
    TheJobsMachineSideRecordEnds,
    /// **The same moment as their subject's machine-side record.**
    TheirSubjectsMachineSideRecordEnds,
    /// **Its outcome is written: it ends as the command closes, in the same
    /// act.**
    ///
    /// **What ending it at the close gives up is `Server protocol and
    /// transfers` C4's and `Server protocol and transfers` C8's service after
    /// the close**: a repeat of the identity acts again, and an ask for it
    /// finds *no record of that identity*, which for this one command does not
    /// say the move did not happen. **Neither costs anything**: the machine
    /// moves the desktop's registration forward only
    /// (`State, identity and recovery` E3's *advances*), so a repeated or late
    /// move leaves it where it stands, and the desktop's own cursor is its own
    /// durable record (`State, identity and recovery` E2).
    ///
    /// **A move whose run ended before it closed has no outcome and this term
    /// does not end it** — one per such ending, never one per acknowledgement.
    ItsOutcomeIsWrittenAndItEndsAsTheCommandCloses,
    /// **The next setup run *of the same kind* on that machine replaces it at run
    /// start — one per machine *per kind of setup act*** (`Server Setup` Y6b's
    /// kinds: the ready action, a Blender install, a Blender removal, an add-on
    /// install, the Telegram placement; **a recovery run takes its taken-over
    /// run's kind**), so the machine holds the last run of each kind and **never a
    /// history of any**.
    ///
    /// **Except where an owning clause ties the record to a subject that came to
    /// exist** (`Server Setup` B8a's install records stand and end with their
    /// build), where [`EndsWhen::TheirSubjectsMachineSideRecordEnds`] governs.
    ///
    /// **Narrowed to per kind because under the flat term a Telegram placement or
    /// an install run would replace the ready action's record at run start** and
    /// strip the card `Server Setup` composes from it of the setup it reports.
    TheNextSetupRunOfTheSameKindReplacesItAtRunStart,
    /// **Its subject's machine-side record ends; a released or dropped hold's
    /// record ends with its subject and not at its release** (§8.6), **so nothing
    /// is written twice about what the answer was.**
    ItsSubjectsMachineSideRecordEnds,
    /// **It is replaced by the next transaction's; a *verified* record is kept
    /// until then.**
    TheNextTransactionsRecordReplacesIt,
    /// **The health exchange succeeds (G15), or the next fresh replacement's first
    /// act (G5).**
    TheHealthExchangeSucceedsOrTheNextFreshReplacementsFirstAct,
    /// **Its transaction reaches *verified*, *rolled back* or *failed before
    /// commit*, and G20's sweep runs.**
    ItsTransactionIsTerminalAndTheSweepRuns,
    /// `State, identity and recovery` E6 and E6a's, **which are that document's and
    /// are applied here unchanged.**
    AnotherDocumentsRuleEntirely,
}

/// **R1's third column — and for every row but the journal it is the same list.**
///
/// **An age, a cap, a sweep, a schedule, or a free-space reading.** None of them
/// ends anything here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeverEndedBy {
    AnAge,
    ACap,
    ASweep,
    ASchedule,
    AFreeSpaceReading,
}

impl NeverEndedBy {
    pub const THE_LIST: [NeverEndedBy; 5] = [
        NeverEndedBy::AnAge,
        NeverEndedBy::ACap,
        NeverEndedBy::ASweep,
        NeverEndedBy::ASchedule,
        NeverEndedBy::AFreeSpaceReading,
    ];

    /// **R1's staged-package row has a list of its own, and it is the shared one
    /// with the sweep dropped.**
    ///
    /// That row's *Ends when* column names G20's bounded sweep as what ends the
    /// material, so a *never ended by* list that included a sweep would have the
    /// row saying a sweep both ends the thing and never ends it. **Its column
    /// reads an age, a cap, a schedule, or a free-space reading.**
    pub const THE_LIST_WITHOUT_THE_SWEEP: [NeverEndedBy; 4] = [
        NeverEndedBy::AnAge,
        NeverEndedBy::ACap,
        NeverEndedBy::ASchedule,
        NeverEndedBy::AFreeSpaceReading,
    ];
}

/// **R1. What this document retains, and when each ends.**
pub fn ends_when(thing: Retained) -> EndsWhen {
    match thing {
        Retained::TheCapturedStreamsAndTheCaptureRecord => EndsWhen::TheJobsMachineSideRecordEnds,
        Retained::AnOperationRecordAndItsCommandRecords => {
            EndsWhen::TheirSubjectsMachineSideRecordEnds
        }
        Retained::TheCommandRecordOfTheCursorMove => {
            EndsWhen::ItsOutcomeIsWrittenAndItEndsAsTheCommandCloses
        }
        Retained::ASetupOperationRecord => EndsWhen::TheNextSetupRunOfTheSameKindReplacesItAtRunStart,
        Retained::AHoldRecord => EndsWhen::ItsSubjectsMachineSideRecordEnds,
        Retained::ThePackagePhaseRecord => EndsWhen::TheNextTransactionsRecordReplacesIt,
        Retained::APreviousPackageVersion => {
            EndsWhen::TheHealthExchangeSucceedsOrTheNextFreshReplacementsFirstAct
        }
        Retained::StagedPackageMaterial => EndsWhen::ItsTransactionIsTerminalAndTheSweepRuns,
        Retained::TheEventJournal => EndsWhen::AnotherDocumentsRuleEntirely,
    }
}

/// **R1's third column, row by row.** The journal's own rule is another
/// document's, **so the list does not reach it**; staged package material has a
/// list of its own, **the shared one without the sweep**, because G20's bounded
/// sweep is what its *Ends when* column names. Every other row takes the shared
/// list entire.
pub fn never_ended_by(thing: Retained) -> &'static [NeverEndedBy] {
    match thing {
        Retained::TheEventJournal => &[],
        Retained::StagedPackageMaterial => &NeverEndedBy::THE_LIST_WITHOUT_THE_SWEEP,
        _ => &NeverEndedBy::THE_LIST,
    }
}

/// **`Server Setup` Y6b's kinds of setup act, carried so R1's per-kind term can be
/// applied rather than described.**
///
/// **A recovery run takes its taken-over run's kind.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetupActKind {
    TheReadyAction,
    ABlenderInstall,
    ABlenderRemoval,
    AnAddOnInstall,
    TheTelegramPlacement,
}

impl SetupActKind {
    pub const ALL: [SetupActKind; 5] = [
        SetupActKind::TheReadyAction,
        SetupActKind::ABlenderInstall,
        SetupActKind::ABlenderRemoval,
        SetupActKind::AnAddOnInstall,
        SetupActKind::TheTelegramPlacement,
    ];
}

/// **R1's per-kind term, applied: whether a starting run replaces a held setup
/// operation record.**
///
/// **One per machine per kind of setup act**, so the machine holds the last run of
/// each kind and never a history of any.
pub fn a_starting_setup_run_replaces(held: SetupActKind, starting: SetupActKind) -> bool {
    held == starting
}

/// **The journal's rule and a new stream capture's are both finite, by different
/// rules — R2.**
///
/// **The journal's rule is `State, identity and recovery` §4 E6's and its figure is
/// that document's E6a's**: every registered cursor passed, then a machine-level
/// cap of **32,768 entries per machine** at a **512-byte** per-entry budget,
/// dropping the oldest acknowledged entries first, **with the oldest-retained
/// number moving so that document's E5 makes a gap visible.**
///
/// **That figure is a constant of the server package and no free-space reading
/// raises or lowers it** — which is R5's rule holding for **the one store on this
/// machine that does drop what it holds.**
///
/// **THEY MOVED, 2026-09-06, which is what this entry said would happen.** It
/// read *"they are `State, identity and recovery` §4 E6a's, not this document's,
/// and the machine-side journal that owns them is built in no crate today …
/// they move to that document's crate the day its machine half is built"*. That
/// day is this one: the journal's rules and figures are
/// `smoozy_state::journal`'s and its store is [`crate::acting::journal`]'s.
/// **What is left here is a re-export and R2's own arithmetic**, because that
/// arithmetic is what makes the reserve's shape checkable — see
/// [`the_journals_bytes`] — and it is this document's rather than that one's.
pub use smoozy_state::journal::THE_ENTRY_CAP as THE_JOURNALS_ENTRY_CAP;

/// **R2.** The per-entry budget the cap multiplies against. Its home is the same
/// as the cap's, above.
pub use smoozy_state::journal::THE_PER_ENTRY_BUDGET as THE_JOURNALS_PER_ENTRY_BUDGET;

/// **R2's arithmetic, and the reason it matters: the bytes those two figures
/// multiply to sit *inside* the control-state reserve M4a subtracts, so the
/// journal and a stream capture never draw on the same bytes.**
pub const fn the_journals_bytes() -> u64 {
    THE_JOURNALS_ENTRY_CAP * THE_JOURNALS_PER_ENTRY_BUDGET
}

/// **R2's second half: a stream attempt's cap is M4a's immutable ceiling above that
/// same reserve.**
///
/// **Reaching it removes or edits nothing already retained and ends no history
/// entry**; the retained prefix stays for R1's whole lifetime, and the explicit
/// *collection incomplete* outcome tells the user that Smoozy could not keep the
/// rest. **This preserves `Queue & History`'s no-expiry rule while refusing the
/// impossible promise that an unbounded stream can fit on a finite state mount.**
pub const REACHING_THE_CEILING_REMOVES_OR_EDITS_NOTHING_ALREADY_RETAINED: bool = true;

/// **R3. Nothing is cleared because a machine is reachable again** (`Server
/// protocol and transfers` S30, `Security and licence` §10), **and nothing is
/// cleared because a container recycle destroyed it** — that is the provider's act
/// and **not an erasure Smoozy can ask for or observe.**
pub const NOTHING_IS_CLEARED_BECAUSE_A_MACHINE_IS_REACHABLE_AGAIN: () = ();

/// **R4. Two sweeps exist on this side and both are bounded to paths this document
/// names.**
///
/// **Neither reaches anything the user put there and neither is triggered by a
/// reading of free space.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheOnlyTwoSweeps {
    /// G20's abandoned package staging, bounded to the locations the phase record
    /// names.
    AbandonedPackageStaging,
    /// Z2's stale endpoint object, at the supervisor's own recorded path.
    TheStaleEndpointObject,
}

impl TheOnlyTwoSweeps {
    pub const BOTH: [TheOnlyTwoSweeps; 2] = [
        TheOnlyTwoSweeps::AbandonedPackageStaging,
        TheOnlyTwoSweeps::TheStaleEndpointObject,
    ];
}

/// **R5. A machine whose disk fills does not have its records cleared to make
/// room.**
///
/// `State, identity and recovery` §11.4's reason is taken whole: **the reading is a
/// choice of mount, and a machine whose disk is full is precisely the machine whose
/// state records are most needed.**
///
/// The value is `()` because the clause is an absence: **no function in this crate
/// takes a free-space figure and removes anything**, and there is nothing here such
/// a rule could be written against.
pub const NO_RECORD_IS_CLEARED_TO_MAKE_ROOM: () = ();

#[cfg(test)]
mod tests {
    use super::*;

    /// **The list is closed by the compiler rather than by its own length.** The
    /// match below is total and carries no default arm, so a row added to
    /// [`Retained`] has none, the crate stops compiling, and whoever added it is
    /// sent to this list — where the length assertion this replaced could never
    /// have been false, the array's own type having fixed it.
    #[test]
    fn every_retained_thing_is_on_this_list_and_names_when_it_ends() {
        let all = [
            Retained::TheCapturedStreamsAndTheCaptureRecord,
            Retained::AnOperationRecordAndItsCommandRecords,
            Retained::TheCommandRecordOfTheCursorMove,
            Retained::ASetupOperationRecord,
            Retained::AHoldRecord,
            Retained::ThePackagePhaseRecord,
            Retained::APreviousPackageVersion,
            Retained::StagedPackageMaterial,
            Retained::TheEventJournal,
        ];
        for (at, thing) in all.iter().enumerate() {
            let where_the_list_puts_it = match thing {
                Retained::TheCapturedStreamsAndTheCaptureRecord => 0,
                Retained::AnOperationRecordAndItsCommandRecords => 1,
                // R1's table puts the cursor move's row straight after the
                // operation record's, which is the row that cannot end it.
                Retained::TheCommandRecordOfTheCursorMove => 2,
                Retained::ASetupOperationRecord => 3,
                Retained::AHoldRecord => 4,
                Retained::ThePackagePhaseRecord => 5,
                Retained::APreviousPackageVersion => 6,
                Retained::StagedPackageMaterial => 7,
                Retained::TheEventJournal => 8,
            };
            assert_eq!(at, where_the_list_puts_it, "{thing:?} is not where this list puts it");
            let _ = ends_when(*thing);
        }
    }

    #[test]
    fn every_row_but_the_journal_and_the_staged_material_carries_the_whole_list() {
        assert_eq!(
            never_ended_by(Retained::TheCapturedStreamsAndTheCaptureRecord),
            &NeverEndedBy::THE_LIST
        );
        assert_eq!(never_ended_by(Retained::AHoldRecord), &NeverEndedBy::THE_LIST);
        // The journal is the one store that drops what it holds, and its rule is
        // another document's entirely.
        assert!(never_ended_by(Retained::TheEventJournal).is_empty());
        assert_eq!(ends_when(Retained::TheEventJournal), EndsWhen::AnotherDocumentsRuleEntirely);
    }

    /// **R1's staged-package row, both columns read together.** G20's bounded
    /// sweep is what ends the material, so a sweep is not on that row's *never
    /// ended by* list — and it is on every other row's.
    #[test]
    fn staged_package_material_is_ended_by_the_sweep_and_so_a_sweep_is_not_on_its_list() {
        assert_eq!(
            ends_when(Retained::StagedPackageMaterial),
            EndsWhen::ItsTransactionIsTerminalAndTheSweepRuns
        );
        assert!(
            !never_ended_by(Retained::StagedPackageMaterial).contains(&NeverEndedBy::ASweep),
            "the row that a sweep ends must not also say a sweep never ends it"
        );
        for still_named in [
            NeverEndedBy::AnAge,
            NeverEndedBy::ACap,
            NeverEndedBy::ASchedule,
            NeverEndedBy::AFreeSpaceReading,
        ] {
            assert!(never_ended_by(Retained::StagedPackageMaterial).contains(&still_named));
        }
        assert!(never_ended_by(Retained::APreviousPackageVersion).contains(&NeverEndedBy::ASweep));
    }

    /// **R1's cursor-move row, both columns read together.** The command that
    /// acts on no subject cannot be ended by the row above it, so it has its
    /// own term — and nothing on the shared list ends it either.
    #[test]
    fn the_cursor_moves_command_record_ends_as_the_command_closes() {
        assert_eq!(
            ends_when(Retained::TheCommandRecordOfTheCursorMove),
            EndsWhen::ItsOutcomeIsWrittenAndItEndsAsTheCommandCloses
        );
        assert_eq!(
            never_ended_by(Retained::TheCommandRecordOfTheCursorMove),
            &NeverEndedBy::THE_LIST
        );
        // And it is a different term from the row it sits beside, which is the
        // whole reason the row exists.
        assert_ne!(
            ends_when(Retained::TheCommandRecordOfTheCursorMove),
            ends_when(Retained::AnOperationRecordAndItsCommandRecords)
        );
    }

    #[test]
    fn a_hold_record_ends_with_its_subject_and_not_at_its_release() {
        assert_eq!(
            ends_when(Retained::AHoldRecord),
            EndsWhen::ItsSubjectsMachineSideRecordEnds
        );
    }

    /// The narrowing: a Telegram placement does not replace the ready action's
    /// record, which the flat term would have done.
    #[test]
    fn a_setup_record_is_replaced_only_by_a_run_of_its_own_kind() {
        assert!(a_starting_setup_run_replaces(
            SetupActKind::TheReadyAction,
            SetupActKind::TheReadyAction
        ));
        assert!(!a_starting_setup_run_replaces(
            SetupActKind::TheReadyAction,
            SetupActKind::TheTelegramPlacement
        ));
        assert!(!a_starting_setup_run_replaces(
            SetupActKind::TheReadyAction,
            SetupActKind::ABlenderInstall
        ));
        // **The kinds are closed by the compiler rather than by a count.** The
        // match is total and has no default arm, so a kind added to
        // `Server Setup` Y6b's set has none and the crate stops compiling.
        for (at, kind) in SetupActKind::ALL.iter().enumerate() {
            let where_the_list_puts_it = match kind {
                SetupActKind::TheReadyAction => 0,
                SetupActKind::ABlenderInstall => 1,
                SetupActKind::ABlenderRemoval => 2,
                SetupActKind::AnAddOnInstall => 3,
                SetupActKind::TheTelegramPlacement => 4,
            };
            assert_eq!(at, where_the_list_puts_it, "{kind:?} is not where the list puts it");
        }
    }

    /// R2's own arithmetic: the journal's bytes sit inside the reserve, so the
    /// journal and a stream capture never draw on the same bytes.
    #[test]
    fn the_journals_bytes_sit_inside_the_control_state_reserve() {
        assert_eq!(the_journals_bytes(), 16 * 1024 * 1024);
        assert!(the_journals_bytes() < crate::capture::THE_CONTROL_STATE_RESERVE);
    }

    #[test]
    fn there_are_exactly_two_sweeps_on_this_side() {
        // **A third sweep fails the build here.** The match is total and has no
        // default arm, so a sweep added to R4 has no arm and the crate stops
        // compiling — where the length assertion this replaced could not be
        // false.
        for (at, sweep) in TheOnlyTwoSweeps::BOTH.iter().enumerate() {
            let where_the_list_puts_it = match sweep {
                TheOnlyTwoSweeps::AbandonedPackageStaging => 0,
                TheOnlyTwoSweeps::TheStaleEndpointObject => 1,
            };
            assert_eq!(at, where_the_list_puts_it, "{sweep:?} is not where R4 puts it");
        }
    }
}

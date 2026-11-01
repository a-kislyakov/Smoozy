//! The single-job acts on the line — `Queue & History` §4 L1–L5 (the
//! cancel), L10–L12a (the stop's queue side).
//!
//! **Constraint:** Rows 56 and 58. Every act is a write to `State, identity
//! and recovery` D1's desired state, serialized by the single writer, applied
//! by the machine with or without anybody connected; its D2 — no desired
//! state inferred from an observed reading — is the tool throughout.
//!
//! **What is here is the judgement each act turns on**: what a cancel is for
//! a job in each state, what it removes, what the local half waits for, how a
//! stop's outcome is told from a job's own ending, and what the entry says in
//! between. **The command's record, its travel and its durable write are the
//! acting half's.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` U22, X8, M20.
//! `Job files and outputs` §11. `Dependency Upload & Remap` U2. `Server
//! protocol and transfers` C16, C17.

use smoozy_state::identities::Identity;

use crate::line::QueueState;

/// **L1 and L4: what act a cancel is, decided by the job's state on the
/// machine and not by what the desktop last saw.** A cancel covers waiting
/// jobs alone; a cancelled job that was rendering is not a cancel but a stop
/// (L4) and takes L10's path; a held job is cancellable on the same terms
/// (L5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatACancelIs {
    /// **L1: the supervisor removes the job record from the line**, journals
    /// the removal, and the material cleanup follows.
    ARemovalFromTheLine,
    /// **L5: the hold record's subject is removed, the hold ends with its
    /// subject** (`Rented-server runtime` U22's third ending), and where
    /// another hold names the removed job as dependency, H9 fires.
    ARemovalOfAHeldJob { hold: Identity },
    /// **L4: not a cancel but a stop.**
    AStop { operation: Identity },
    /// **L1: a named job that is no longer waiting — it ended while the
    /// command travelled — is not cancelled**, the outcome says so and names
    /// why, and the material cleanup is not sequenced.
    NotCancelledItAlreadyEnded { outcome: Identity },
}

pub fn what_a_cancel_is(state: &QueueState) -> WhatACancelIs {
    match state {
        QueueState::Waiting => WhatACancelIs::ARemovalFromTheLine,
        QueueState::Held { hold } => WhatACancelIs::ARemovalOfAHeldJob { hold: hold.clone() },
        QueueState::Rendering { operation } => WhatACancelIs::AStop {
            operation: operation.clone(),
        },
        QueueState::Ended { outcome } => WhatACancelIs::NotCancelledItAlreadyEnded {
            outcome: outcome.clone(),
        },
    }
}

/// **L1's material, removed after the deciding write — partial across
/// directories and atomic within each**, and none of it can resurrect the
/// job, because the record is already gone. The deciding write is the job
/// record's removal, after which the job cannot start.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialACancelRemoves {
    /// The job record directory with its manifest, reading, attempt records
    /// and derivatives (`Job files and outputs` §11).
    TheJobRecordDirectory,
    /// The remap material (`Dependency Upload & Remap` U2).
    TheRemapMaterial,
    ThePerJobNote,
    /// Empty, because the job never rendered.
    TheEmptyJobOutputFolder,
}

impl MaterialACancelRemoves {
    /// **The enumeration, in the order the cleanup runs**, after the record's
    /// removal. A unit that fails is retried by the ordinary owed-work route.
    pub const IN_ORDER: [MaterialACancelRemoves; 4] = [
        MaterialACancelRemoves::TheJobRecordDirectory,
        MaterialACancelRemoves::TheRemapMaterial,
        MaterialACancelRemoves::ThePerJobNote,
        MaterialACancelRemoves::TheEmptyJobOutputFolder,
    ];
}

/// **L1's second unit, located by the document that owns it**: the remap
/// material at `remaps/<job identity>` goes with the job's machine-side
/// record, by the same acts (`Dependency Upload & Remap` U2). Composed by
/// that crate's own rule and never by a path made here.
pub fn the_remap_material_a_cancel_removes(
    layout: &smoozy_job_files_machine::layout::TheLayout,
    job: &Identity,
) -> smoozy_dependencies_machine::bytes::MachineLocation {
    smoozy_dependencies_machine::cleanup::what_goes_with_the_jobs_machine_side_record(layout, job)
}

/// **L2: the two copies disappear in a fixed order — machine first, then the
/// desktop.** The local record is removed only when the cancel's recorded
/// outcome (*removed*) is learned: on the acknowledgement, on a later
/// connection asking the command's identity, or on the journal event. Until
/// then the entry may show the job cancelling. After it there is no entry,
/// no hidden row and no tombstone (`State, identity and recovery` §10.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheDesktopLearned {
    /// The machine's recorded outcome: *removed*.
    TheOutcomeRemoved,
    /// The machine's refusal: the job was no longer waiting.
    TheOutcomeNotCancelled,
    /// Nothing yet — the command is outstanding.
    NothingYet,
}

/// The local half's answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheLocalHalf {
    /// Remove the local record now — provisional or confirmed alike — and
    /// write no tombstone.
    RemoveTheRecordAndNoTombstone,
    /// The entry shows the job cancelling; nothing local is removed.
    TheEntrySaysCancelling,
    /// The job stands, and the entry shows what the records say.
    TheJobStands,
}

pub fn the_local_half_of_a_cancel(learned: WhatTheDesktopLearned) -> TheLocalHalf {
    match learned {
        WhatTheDesktopLearned::TheOutcomeRemoved => TheLocalHalf::RemoveTheRecordAndNoTombstone,
        WhatTheDesktopLearned::NothingYet => TheLocalHalf::TheEntrySaysCancelling,
        WhatTheDesktopLearned::TheOutcomeNotCancelled => TheLocalHalf::TheJobStands,
    }
}

/// **L3: a job whose cancel never reached the machine is not a cancelled job
/// — it is still submitted work.** If its server row was removed meanwhile,
/// the local record went subject-held with the row, and a later connection
/// finds the work untracked and offers adoption. **A cancel is only ever
/// reported done when the machine's copy is gone**, so nothing the user was
/// told was deleted comes back.
pub const A_CANCEL_IS_REPORTED_DONE_ONLY_WHEN_THE_MACHINES_COPY_IS_GONE: bool = true;

/// **L10 and L11: the stop's queue side.** *Left stopped* is written only by
/// the stop path; a job that ended on its own **before the stop's
/// termination was issued** records its own ending, and the entry shows that
/// ending, not *stopped*; an ending past the issuance is *left stopped*. The
/// boundary is `State, identity and recovery` §5.3 step 7's one boundary —
/// the issuance. **No stream, no exit number and no timing is read to decide
/// it** (`Rented-server runtime` X8, M20): the recorded outcome alone says
/// which, and this only reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheStopLeft {
    /// The outcome *left stopped*: the entry moves to history marked
    /// *stopped*, and the admission point starts the next job on the
    /// outcome's write (Q4).
    LeftStopped,
    /// The job's own ending, recorded before the issuance: the entry shows
    /// that ending.
    TheJobsOwnEnding,
}

/// **L12: a stop is not instant and nothing here waits on it without a
/// deadline of its own.** Observed 1–3 seconds on the virtual machine and
/// 1.9 to 14 on the container. Every waiter uses its own per-waiter deadline
/// (`Server protocol and transfers` C16 and C17); **expiry is reported as
/// the waiter giving up, never as the stop having failed or succeeded.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AWaitersEnd {
    TheOutcomeWasLearned(WhatTheStopLeft),
    /// The waiter gave up. Nothing about the stop is concluded.
    TheWaiterGaveUp,
}

/// **L12a: between the stop's submission and its outcome, the entry says the
/// stop is happening.** From the moment the stop command's record is
/// written until its recorded outcome is learned or its own waiter expires,
/// the entry reads *stopping*, composed from the desktop's knowledge of its
/// own outstanding command — the same shape as L2's *cancelling*. **It is
/// Smoozy's statement about Smoozy's own act and never a sixth queue state**:
/// the machine's record still reads *rendering* until the outcome is
/// written, and no reading is reinterpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheEntryReads {
    Stopping,
    /// The job's ending, as the records say: *stopped*, or its own.
    ItsEnding(WhatTheStopLeft),
    /// The waiter expired: the entry returns to what the records say, with
    /// the expiry reported as the waiter giving up.
    WhatTheRecordsSay,
}

pub fn what_the_entry_reads_during_a_stop(
    the_command_record_is_written: bool,
    waiter: Option<AWaitersEnd>,
) -> WhatTheEntryReads {
    match waiter {
        Some(AWaitersEnd::TheOutcomeWasLearned(left)) => WhatTheEntryReads::ItsEnding(left),
        Some(AWaitersEnd::TheWaiterGaveUp) => WhatTheEntryReads::WhatTheRecordsSay,
        None if the_command_record_is_written => WhatTheEntryReads::Stopping,
        None => WhatTheEntryReads::WhatTheRecordsSay,
    }
}

/// **L1a: two controls sit on the job's own entry and belong to `Download`
/// rather than to this document** — an auto-download switch and a narrowing
/// control. What this crate owns is only that the entry is where they sit,
/// and that neither is a desired state of the job: switching auto-download
/// on writes nothing to the machine and touches nothing in the line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AControlOnTheEntryThatIsDownloads {
    TheAutoDownloadSwitch,
    TheNarrowingControl,
}

impl AControlOnTheEntryThatIsDownloads {
    /// Neither writes anything to the machine's job record.
    pub fn writes_to_the_jobs_desired_state(self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::tests::an_operation;

    #[test]
    fn a_cancel_is_a_removal_for_a_waiting_job_and_a_stop_for_a_rendering_one() {
        assert_eq!(
            what_a_cancel_is(&QueueState::Waiting),
            WhatACancelIs::ARemovalFromTheLine
        );
        let operation = an_operation();
        assert_eq!(
            what_a_cancel_is(&QueueState::Rendering {
                operation: operation.clone()
            }),
            WhatACancelIs::AStop { operation }
        );
        let hold = an_operation();
        assert_eq!(
            what_a_cancel_is(&QueueState::Held { hold: hold.clone() }),
            WhatACancelIs::ARemovalOfAHeldJob { hold }
        );
        let outcome = an_operation();
        assert_eq!(
            what_a_cancel_is(&QueueState::Ended {
                outcome: outcome.clone()
            }),
            WhatACancelIs::NotCancelledItAlreadyEnded { outcome }
        );
    }

    #[test]
    fn the_material_is_removed_after_the_record_and_the_record_directory_first() {
        assert_eq!(MaterialACancelRemoves::IN_ORDER.len(), 4);
        assert_eq!(
            MaterialACancelRemoves::IN_ORDER[0],
            MaterialACancelRemoves::TheJobRecordDirectory
        );
    }

    #[test]
    fn the_remap_material_is_located_by_the_document_that_owns_it_and_named_for_the_job() {
        use smoozy_job_files_machine::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
        let layout = TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        let job = crate::line::tests::a_job();
        let location = the_remap_material_a_cancel_removes(&layout, &job);
        let text = String::from_utf8(location.as_bytes().to_vec()).unwrap();
        assert!(text.ends_with(&job.as_text()), "{text}");
        assert!(text.contains("remaps"), "{text}");
    }

    #[test]
    fn the_local_record_goes_only_on_the_recorded_outcome_removed_and_gets_no_tombstone() {
        assert_eq!(
            the_local_half_of_a_cancel(WhatTheDesktopLearned::TheOutcomeRemoved),
            TheLocalHalf::RemoveTheRecordAndNoTombstone
        );
        assert_eq!(
            the_local_half_of_a_cancel(WhatTheDesktopLearned::NothingYet),
            TheLocalHalf::TheEntrySaysCancelling
        );
        assert_eq!(
            the_local_half_of_a_cancel(WhatTheDesktopLearned::TheOutcomeNotCancelled),
            TheLocalHalf::TheJobStands
        );
    }

    #[test]
    fn the_entry_says_stopping_only_while_the_command_stands_and_never_after_the_waiter_gives_up() {
        assert_eq!(
            what_the_entry_reads_during_a_stop(true, None),
            WhatTheEntryReads::Stopping
        );
        assert_eq!(
            what_the_entry_reads_during_a_stop(false, None),
            WhatTheEntryReads::WhatTheRecordsSay
        );
        assert_eq!(
            what_the_entry_reads_during_a_stop(true, Some(AWaitersEnd::TheWaiterGaveUp)),
            WhatTheEntryReads::WhatTheRecordsSay
        );
        assert_eq!(
            what_the_entry_reads_during_a_stop(
                true,
                Some(AWaitersEnd::TheOutcomeWasLearned(
                    WhatTheStopLeft::TheJobsOwnEnding
                ))
            ),
            WhatTheEntryReads::ItsEnding(WhatTheStopLeft::TheJobsOwnEnding)
        );
    }

    #[test]
    fn downloads_two_controls_write_nothing_to_the_line() {
        for control in [
            AControlOnTheEntryThatIsDownloads::TheAutoDownloadSwitch,
            AControlOnTheEntryThatIsDownloads::TheNarrowingControl,
        ] {
            assert!(!control.writes_to_the_jobs_desired_state());
        }
    }
}

//! §11 — the captured streams, the exit, and the explicit collection outcome.
//!
//! **The raw record is this document's and nothing edits it** — `Security and
//! licence` §13's own boundary: *"Nothing in these rules edits it — Smoozy does
//! not alter what the machine said."*
//!
//! **M12. The collection outcome is a recorded fact about the collection, written
//! by Smoozy about its own act, and it is what turns an absence into something
//! that can be reported.** Row 286 says the plain sentence must say the render
//! ended without printing anything **rather than leaving the user to work out
//! whether Smoozy failed to collect the words or there were none** — and **an
//! absence cannot prove itself. What can be recorded is what Smoozy did.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §1.2, §1.3 rules 4
//! and 5, §5.3, §6 step 2, §8.1, §8.6. `Job files and outputs` C9, C26, C28, C36,
//! D16, D17, L4a, L6, L7, L11, and its §10 and §14 item 5. `Security and licence`
//! §13 and its rules 1, 2 and 5. `Server protocol and transfers` R12, S22.
//! `Rendering` Row 10. `Desktop app and delivery` — the presentation.

use smoozy_state::identities::Identity;

/// Which of the two streams.
///
/// **M7. The two streams are two sinks and are never merged.** The reason is
/// Blender's own and is already settled: in one version the line naming what did
/// the work is reached only through a debugging switch writing to the complaints
/// stream, while from the next it is on the ordinary output (`Rendering` Row 10),
/// **and the complaints stream also carries the failure text shown to the user.**
/// **Anything mixing the two into one record loses which one a line came from.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stream {
    /// The ordinary output. **It carries D16's marker** (M9).
    Ordinary,
    /// **M16. There is no equivalent marker on the complaints stream and this
    /// document does not add one.** `Job files and outputs` D16 is that
    /// document's and prints on the render's own output; **adding a second marker
    /// would be a change to another document's launch material**, so what stands
    /// for the complaints stream is M13's own four outcomes and no more
    /// (§16 item 9).
    Complaints,
}

/// **M1. The supervisor creates two sinks per attempt inside the package area,
/// keyed to the job identity and the attempt, before the worker is started, and
/// records their paths in the operation record's per-kind field.**
///
/// That is where the captured streams already live — `State, identity and
/// recovery` §1.2 lists them as a machine store inside the server package's own
/// state area, and `Job files and outputs` L6 puts them in the same area — **and
/// the job-record area is that document's to own.** They are **outside every job
/// output folder**, so nothing the user browses, downloads or renders reaches
/// them (that document's L7). **When they end is §13 R1's**, which names it by the
/// job's machine-side record rather than by a sweep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureRecord {
    pub job: Identity,
    pub attempt: u32,
    pub ordinary_sink: String,
    pub complaints_sink: String,
    /// **M4a's ceiling, recorded by value before the worker starts.**
    pub ceiling: CaptureCeiling,
}

/// **M2. No probe byte is written into a sink to confirm its placement.**
///
/// `State, identity and recovery` §1.3 rule 4's real-write confirmation **is
/// performed once for the whole root at hand-over** (`Job files and outputs` L11)
/// **and is not repeated per stream**, because **a byte written to confirm a
/// placement would be a byte in the raw record that the machine did not print.**
pub const NO_PROBE_BYTE_IS_EVER_WRITTEN_INTO_A_SINK: () = ();

/// **M3. The worker holds both of the render's streams and copies them to the
/// sinks itself.**
///
/// **It does not hand the render a descriptor onto a file and it does not pipe
/// them through the supervisor. Two things force that and both are observed.**
/// The supervisor may die while the render lives — a detached program outlived
/// the shell that started it, *"Its parent is **1**"* — **so a capture that ran in
/// the supervisor would stop mid-render in exactly the case the *resumed* branch
/// exists for.** And the worker is already the process that remains until its
/// external processes have exited (X12), **so it is alive for the whole of the
/// render by construction.**
pub const THE_WORKER_COPIES_THE_STREAMS_AND_NOT_THE_SUPERVISOR: bool = true;

/// **The 64 MiB control-state reserve — a Build constant at one declaration point
/// and not a user setting.**
///
/// **It exists so the worker and supervisor can still write the refused-write
/// fact, the two collection outcomes and the job ending after capture reaches its
/// ceiling.**
///
/// **The reserve protects the state mount from Smoozy's own stream capture and
/// claims no protection from the user's outputs or another process filling that
/// mount.** What keeps a render's own outputs off this mount at all, where
/// anything does, is placement rather than this reserve: `Job files and outputs`
/// L4a puts the package area on a mount chosen apart from the job area's where the
/// machine offers more than one, and that document's §14 item 21 states what still
/// breaks where it does not.
pub const THE_CONTROL_STATE_RESERVE: u64 = 64 * 1024 * 1024;

/// **M4a. The pair of sinks receives one finite capture ceiling when M1 creates
/// it.**
///
/// The supervisor reads the bytes available to its login on the state area's
/// recorded mount through V7's free-space route and **subtracts the reserve; the
/// non-negative remainder is the attempt's capture ceiling, recorded by value
/// before the worker starts.** **Existing retained streams are already absent from
/// that available-byte reading and are not subtracted a second time.**
///
/// **The two streams draw from one remaining-byte counter**: a write is offered no
/// more than the remainder, **only bytes successfully retained decrement it**, and
/// once it is zero **neither sink receives another durable write.**
///
/// **No later free-space reading raises it, and a lower reading clears nothing**:
/// the ceiling limits only new stream bytes and **does not age, trim or remove any
/// byte already retained.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureCeiling {
    remaining: u64,
}

impl CaptureCeiling {
    /// **M4a.** The available reading less the reserve, floored at zero.
    pub fn from(available_bytes: u64) -> CaptureCeiling {
        CaptureCeiling {
            remaining: available_bytes.saturating_sub(THE_CONTROL_STATE_RESERVE),
        }
    }

    pub fn remaining(self) -> u64 {
        self.remaining
    }

    /// **How many of `wanted` may be offered to a sink.** Never more than the
    /// remainder.
    pub fn may_offer(self, wanted: u64) -> u64 {
        wanted.min(self.remaining)
    }

    /// **Only bytes successfully retained decrement it.**
    pub fn retained(&mut self, bytes: u64) {
        self.remaining = self.remaining.saturating_sub(bytes);
    }

    /// **Where the remainder is zero, the capture record and empty sinks are
    /// still established from the control-state reserve, the worker starts, and
    /// every stream byte is drained and discarded under M5 with *collection
    /// incomplete* recorded.**
    pub fn is_spent(self) -> bool {
        self.remaining == 0
    }
}

/// **M4a's last requirement, and it is a refusal rather than a degradation: a
/// placement that cannot establish the reserve does not admit a render; it reports
/// that the machine has no room for control state.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherAPlacementMayAdmitARender {
    ItMay,
    NoRoomForControlState,
}

/// **M4a.**
pub fn whether_a_placement_may_admit_a_render(
    available_bytes: u64,
) -> WhetherAPlacementMayAdmitARender {
    if available_bytes >= THE_CONTROL_STATE_RESERVE {
        WhetherAPlacementMayAdmitARender::ItMay
    } else {
        WhetherAPlacementMayAdmitARender::NoRoomForControlState
    }
}

/// **M4 and M28 — what the worker observes at its own loop.**
///
/// **M4. The worker counts the bytes it writes to each sink, and the count is
/// Smoozy's own count.** It is **never a reading of a sink's size**: `State,
/// identity and recovery` §1.3 rule 5 forbids reading a file's size as state, and
/// the reason is the one that defeats every other size reading in this project — a
/// zero-byte file *"is indistinguishable between 'still being written', 'the disk
/// filled' and 'the render wrote nothing'"*.
///
/// **This is the same discipline `Server protocol and transfers` R12 applies to a
/// transfer's progress and S22 to its digest: the count is taken at Smoozy's own
/// loop and never asked of anything else.**
///
/// **M28. At that same loop, and on the ordinary stream, the worker records two
/// further observations of its own: whether the launch material's marker passed,
/// and whether anything followed it.** They are observations of Smoozy's own act
/// on bytes Smoozy is copying, exactly as the count is, **and neither is a reading
/// of a sink or an arithmetic comparison against the marker's length** — the
/// marker names the job identity and each destination it set (M9), so its length
/// varies by job and **nothing here compares against it.** **The complaints stream
/// carries no marker and takes neither observation.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheWorkerObserved {
    pub stream: Stream,
    /// **Smoozy's own count of the bytes it retained.**
    pub retained: u64,
    /// **M5's discarded count.**
    pub discarded: u64,
    /// **M5's first loss reason, recorded once.**
    pub first_loss_reason: Option<String>,
    /// Whether the worker reported an end of stream.
    pub end_of_stream: bool,
    /// **M28's first, on the ordinary stream only.** `None` on the complaints
    /// stream, which has no marker (M16).
    pub the_marker_passed: Option<bool>,
    /// **M28's second, on the ordinary stream only.**
    pub something_followed_the_marker: Option<bool>,
}

/// **M5. Reaching M4a's ceiling, or a sink write being refused first, ends durable
/// writes for that stream and never ends its read.**
///
/// **The worker records the first reason once, keeps reading that child pipe to
/// end of stream, counts the bytes it discards, and writes none of them
/// anywhere.** It continues the other stream independently until that stream
/// reaches the shared ceiling or its own sink refuses.
///
/// **Documented**: a write to a pipe with no reader gives a broken-pipe outcome,
/// and a pipe writer may wait where capacity is unavailable — **so closing the
/// reader or leaving it present but undrained can change or stop the child rather
/// than merely shorten Smoozy's record. Draining and discarding keeps the
/// collection failure from changing the render.** On a full filesystem every sink
/// write is refused with *No space left on device*, uniformly.
///
/// **Smoozy does not stop a render because it can no longer durably record what
/// the render is saying**, and §11.3 says exactly which prefix was retained and
/// why the rest was lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheWorkerDoesWithTheRestOfTheStream {
    /// The ordinary case: it retains it.
    RetainsIt,
    /// **Reads to end of stream, counts what it discards, writes none of it.**
    DrainsAndDiscards,
}

/// **M5.**
pub fn what_the_worker_does(
    ceiling: CaptureCeiling,
    this_sinks_write_was_refused: bool,
) -> WhatTheWorkerDoesWithTheRestOfTheStream {
    if ceiling.is_spent() || this_sinks_write_was_refused {
        WhatTheWorkerDoesWithTheRestOfTheStream::DrainsAndDiscards
    } else {
        WhatTheWorkerDoesWithTheRestOfTheStream::RetainsIt
    }
}

/// **M6. The worker reports its retained and discarded counts, the first loss
/// reason per stream, M28's two observations and the exit it reaped to the
/// supervisor, which writes them into the operation record.**
///
/// **Workers report observations and the supervisor is the only writer of state**
/// (S1); **the sinks are material written under the supervisor's control, in the
/// same sense the staging areas are.**
///
/// **A report that crosses a supervisor restart is the case this channel is
/// written for, and W12 states how it survives one.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkersCaptureReport {
    pub ordinary: WhatTheWorkerObserved,
    pub complaints: WhatTheWorkerObserved,
    /// **M22's one use of the exit status**: it is recorded, verbatim as a
    /// number, and handed on.
    pub exit_status: Option<i32>,
}

/// **M8. Nothing edits any byte retained from either stream — not a redaction, not
/// a filter, not a prefix, not a trim.**
///
/// `Security and licence` §13's protection is *never emitted* rather than
/// *filtered before display*, and its rules 1 and 2 keep secrets out of the
/// machine's reach in the first place: **no secret is ever a process argument or
/// an environment value, and the machine's own environment is never captured into
/// any record Smoozy keeps.**
///
/// **M5's post-ceiling bytes are not retained and are represented only by the
/// discarded-byte count and loss reason; Smoozy never presents the retained prefix
/// as the whole stream.**
pub const NOTHING_EDITS_A_RETAINED_BYTE: () = ();

/// **M9. The launch material's marker is part of the raw record and is not an edit
/// of it.**
///
/// `Job files and outputs` D16 has the material's first act print a marker on the
/// render's own output **naming the job identity and each destination it set**.
/// **That is the render printing, through material Smoozy handed it**, and it is
/// what §11.3 leans on.
pub const THE_MARKER_IS_THE_RENDER_PRINTING_AND_NOT_AN_EDIT: bool = true;

/// **M10. Both streams are read back with nobody connected, and searched.**
///
/// Fact 70's plain half is answered: a program printing to both and ending at
/// status 3, started detached, **had both streams kept separate, the ending
/// recorded, and the kept output searchable for a line.**
///
/// **The paths worth searching for are `Job files and outputs`' completeness
/// statement's** (that document's C28); **whether anything searches them is that
/// document's and `Desktop app and delivery`'s.**
pub const WHAT_IS_WORTH_SEARCHING_FOR_IS_ANOTHER_DOCUMENTS: bool = true;

/// **M11. What is retained is read back whole or not at all.**
///
/// A read of the retained prefix that did not complete **is not a reading** — the
/// same rule and the same evidence as `Job files and outputs` C9, where *"Partial
/// output is delivered before the connection dies"* and a truncated listing and a
/// genuinely short one were separable only by exit status.
///
/// **Completing this read proves only that the retained prefix arrived whole;
/// M13's outcome says whether that prefix is the whole stream.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ARetainedPrefixRead {
    Whole,
    /// **Not a reading.**
    NotAReading,
}

impl ARetainedPrefixRead {
    /// **M11, as the reading itself.** A read that completed is the retained
    /// prefix whole; **a read that did not complete is not a reading**, whatever
    /// bytes it brought back.
    pub fn of(the_read_completed: bool) -> ARetainedPrefixRead {
        if the_read_completed {
            ARetainedPrefixRead::Whole
        } else {
            ARetainedPrefixRead::NotAReading
        }
    }

    /// Whether anything may be read off this at all.
    ///
    /// **A partial read answers no**, which is the whole of M11: a truncated
    /// listing and a genuinely short one were separable only by exit status, so
    /// the bytes a partial read brought back stand for nothing.
    pub fn is_a_reading(self) -> bool {
        match self {
            ARetainedPrefixRead::Whole => true,
            ARetainedPrefixRead::NotAReading => false,
        }
    }
}

/// **M13. One outcome per stream per attempt, from a closed set of four.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionOutcome {
    /// **The record is the whole of what was printed on that stream.**
    Collected,
    /// **The render printed nothing on that stream, and this is a recorded fact
    /// rather than an inference from an empty file.**
    CollectedAndTheStreamWasEmpty,
    /// **The retained bytes are a verbatim prefix and may be empty; nothing says
    /// they are all of it**; retained and discarded counts and the first loss
    /// reason are recorded.
    CollectionIncomplete,
    /// **Nothing of the render's own printing was established** — a run that ended
    /// before it reached the launch material, or an attempt with no capture record
    /// at all — **and Smoozy says so rather than showing an empty panel.**
    NeverEstablished,
}

impl CollectionOutcome {
    /// **The four, closed.**
    pub const EVERY: [CollectionOutcome; 4] = [
        CollectionOutcome::Collected,
        CollectionOutcome::CollectedAndTheStreamWasEmpty,
        CollectionOutcome::CollectionIncomplete,
        CollectionOutcome::NeverEstablished,
    ];

    /// **The outcome's own words** — what the worker's report carries, what
    /// the operation record's per-kind field keeps, and what the note reading's
    /// answer sends (`Queue & History` E9). **One home for every leg**, so the
    /// record a worker wrote and the answer the desktop reads cannot spell one
    /// outcome two ways. Moved here from the machine program's render worker on
    /// 2026-10-03, the Creator's notes on 0.1.8, piece 6d, unchanged.
    pub fn in_words(self) -> &'static str {
        match self {
            CollectionOutcome::Collected => "collected",
            CollectionOutcome::CollectedAndTheStreamWasEmpty => {
                "collected, and the stream was empty"
            }
            CollectionOutcome::CollectionIncomplete => "collection incomplete",
            CollectionOutcome::NeverEstablished => "never established",
        }
    }

    /// **The outcome its words name**, and nothing for words that name none —
    /// never a default.
    pub fn from_words(words: &str) -> Option<CollectionOutcome> {
        CollectionOutcome::EVERY
            .into_iter()
            .find(|outcome| outcome.in_words() == words)
    }
}

/// **M13's table.**
///
/// **M14. The two that Row 286 exists to separate are separated by two different
/// records, not by two readings of one thing.** *Collected and empty* rests on an
/// end Smoozy observed and on what Smoozy observed passing its own loop — the
/// count on the complaints stream, the marker and what followed it on the ordinary
/// one (M28). *Never established* rests on **the absence of a capture record the
/// supervisor writes before the worker starts**, or on **that marker never having
/// passed at all**. **Neither is derived from the sink**, and both are Smoozy's own
/// record of its own act.
///
/// **M15. The marker is the positive signal that makes *collected and empty*
/// believable on the ordinary stream.** So **an ordinary stream holding the marker
/// and nothing after it is a render that printed nothing, and an ordinary stream
/// with no marker at all is a run that did not reach the material** — which `Job
/// files and outputs` D17 already forbids anyone treating as a loud failure.
/// **Neither is read off the stream afterwards**; both are M28's observations,
/// taken as the bytes passed.
pub fn collection_outcome(
    capture_record_exists: bool,
    the_worker_was_started_before_its_sinks_were_made: bool,
    observed: &WhatTheWorkerObserved,
    ceiling_was_reached: bool,
) -> CollectionOutcome {
    // **Never established, first branch: no capture record exists for the
    // attempt.** And, defensively, where a worker was started before its sinks
    // were made — which §14 forbids.
    if !capture_record_exists || the_worker_was_started_before_its_sinks_were_made {
        return CollectionOutcome::NeverEstablished;
    }
    // **Collection incomplete: the worker did not report an end of stream; or
    // M4a's ceiling was reached; or a sink write was refused (M5).**
    if !observed.end_of_stream || ceiling_was_reached || observed.first_loss_reason.is_some() {
        return CollectionOutcome::CollectionIncomplete;
    }
    match observed.stream {
        Stream::Ordinary => match observed.the_marker_passed {
            // **The worker reported an end of stream and never observed the
            // marker**: a run that did not reach the material.
            Some(false) | None => CollectionOutcome::NeverEstablished,
            Some(true) => {
                if observed.something_followed_the_marker == Some(true) {
                    CollectionOutcome::Collected
                } else {
                    CollectionOutcome::CollectedAndTheStreamWasEmpty
                }
            }
        },
        Stream::Complaints => {
            if observed.retained == 0 {
                CollectionOutcome::CollectedAndTheStreamWasEmpty
            } else {
                CollectionOutcome::Collected
            }
        }
    }
}

/// **M17. Reconciliation writes *collection incomplete* for any attempt whose
/// capture record was established and whose worker never reported an end.**
///
/// **That is the case a reboot, a container restart and a worker's own death all
/// produce**, and it is written at the reconciliation that finds the operation
/// record's group gone (`State, identity and recovery` §6 step 2). **Without that
/// step an interrupted capture would be indistinguishable from an empty one**,
/// which is exactly the confusion Row 286 forbids.
pub fn what_reconciliation_writes(
    capture_record_was_established: bool,
    the_worker_reported_an_end: bool,
) -> Option<CollectionOutcome> {
    if capture_record_was_established && !the_worker_reported_an_end {
        Some(CollectionOutcome::CollectionIncomplete)
    } else {
        None
    }
}

/// **M18. The outcome is written once per stream per attempt and is never
/// rewritten.**
///
/// It is a subject outcome in `State, identity and recovery` §8.1's sense:
/// **immutable, and not the user's to clear.**
///
/// **M19. A re-run is a new attempt with its own two sinks and its own two
/// outcomes.** Nothing appends to a previous attempt's record and **nothing merges
/// two attempts' streams** — which is why this type carries the attempt number and
/// nothing accumulates across one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrittenOnce {
    pub job: Identity,
    pub attempt: u32,
    pub ordinary: CollectionOutcome,
    pub complaints: CollectionOutcome,
}

/// **§11.4 — the ending is never read from what was printed.**
///
/// **M20. A job's state comes from the operation record, the execution group's
/// validation and the reaped exit status, and never from either stream.** That is
/// Row 286's third clause and it is kept by **the streams being material that
/// nothing reads as state** (`State, identity and recovery` §1.3 rule 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAJobsStateComesFrom {
    TheOperationRecord,
    TheExecutionGroupsValidation,
    TheReapedExitStatus,
}

impl WhatAJobsStateComesFrom {
    /// **The three, and neither stream is among them.**
    pub const THE_ONLY_THREE: [WhatAJobsStateComesFrom; 3] = [
        WhatAJobsStateComesFrom::TheOperationRecord,
        WhatAJobsStateComesFrom::TheExecutionGroupsValidation,
        WhatAJobsStateComesFrom::TheReapedExitStatus,
    ];
}

/// **M21. Where the group is gone and no exit status was reaped, the outcome is
/// recorded as an ending with no status, and an ending with no status is
/// failed.**
///
/// *"A job that stops with nothing to read is failed, not still rendering."*
/// **Nothing stays in a running state because nothing was printed**, and **no
/// absence of output ever extends a job's life.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheEndingReads {
    /// The status, verbatim.
    WithAStatus(i32),
    /// **Failed.**
    AnEndingWithNoStatusWhichIsFailed,
}

/// **M21.**
pub fn how_the_ending_reads(
    the_group_is_gone: bool,
    reaped: Option<i32>,
) -> Option<HowTheEndingReads> {
    match (the_group_is_gone, reaped) {
        (_, Some(status)) => Some(HowTheEndingReads::WithAStatus(status)),
        (true, None) => Some(HowTheEndingReads::AnEndingWithNoStatusWhichIsFailed),
        (false, None) => None,
    }
}

/// **M22. A successful exit status is never read as *everything was written*.**
///
/// Blender's own research counts twelve endings that report success, and this
/// stage's own evidence from outside Blender adds three conditions that all end at
/// **0** — a full disk, a purged driver and a mismatched driver. **Three
/// conditions, one ending, and neither figure is added to the other.**
///
/// **This document reads the exit status for one thing only:** it is recorded,
/// verbatim as a number, in the capture record, and it is handed on. **What
/// completeness is read off is `Job files and outputs` §10's and is not this
/// document's.**
pub const THE_EXIT_STATUS_IS_RECORDED_VERBATIM_AND_NOTHING_MORE: bool = true;

/// **M23. Nothing here matches an ending on its number** (X13), **and nothing
/// reads the machine's own system log for a job's ending.** Fact 55 is why: *"The
/// system log records only that it sent a termination signal to remaining
/// processes, naming neither Blender nor the render nor its output."*
pub const NOTHING_READS_THE_MACHINES_OWN_SYSTEM_LOG_FOR_AN_ENDING: () = ();

/// **M24. Nothing mines either stream for a cause.**
///
/// `Security and licence` §13 rule 5 — *"A tool's own output is captured whole or
/// not at all, and is never mined for a cause"* — and `Job files and outputs` C26,
/// **which records four different wordings for one failure across versions and
/// operating systems.**
pub const NOTHING_MINES_EITHER_STREAM_FOR_A_CAUSE: () = ();

/// **§11.5 — what the arrangement does not close.**
///
/// **M25. Fact 70's second half is not closed by this section and this section
/// does not claim to close it.** What is written here is the arrangement; **whether
/// it delivers the distinction on a real render on a real machine** was Batch E's,
/// and it is the whole difficulty Row 286 names. **Batch E posed it and the
/// arrangement delivers**: all four outcomes came back distinguishable on both
/// machine kinds, read with nobody connected, with every count agreeing exactly
/// with what the sinks held, *collected and empty* never written where the worker
/// was killed, and *never established* plainly distinct from it. **Nothing in this
/// section changed; what changed is that it is observed.**
pub const THE_FOUR_OUTCOMES_ARE_OBSERVED_TO_BE_DISTINGUISHABLE: bool = true;

/// **M26. The residual is named exactly.**
///
/// **A worker that died can leave bytes in a sink that its count never reached, so
/// a *collection incomplete* stream can hold content while the count says none.**
///
/// **That is why *collection incomplete* is its own outcome and never collapses
/// into *collected and empty***, and why **the raw record is still offered for an
/// incomplete stream, marked as incomplete.**
pub fn a_raw_record_is_still_offered(outcome: CollectionOutcome) -> bool {
    !matches!(outcome, CollectionOutcome::NeverEstablished)
}

/// **M27. `Job files and outputs` §14 item 5 states what breaks without the
/// fact**: that document's C28's route from an absent path to the line naming it,
/// which is Row 136's only signal. **What holds without it is that document's
/// completeness statement**, which is produced from the inventory and one walk and
/// **needs no stream at all.**
pub const THE_COMPLETENESS_STATEMENT_NEEDS_NO_STREAM_AT_ALL: bool = true;

/// Making the sinks and copying into them, behind a trait.
///
/// **The supervisor makes them (M1) and the worker copies (M3)**, and the split is
/// the two halves of this trait rather than a note.
pub trait MakesTheSinks {
    /// **M1.** Before the worker is started. **§14: where either the operation
    /// record or the sinks cannot be made, the worker is *not attempted*, naming
    /// what stopped it.**
    fn make(&self, record: &CaptureRecord) -> Result<(), String>;
}

/// **M3 and M4's loop, behind a trait.**
///
/// **The count is Smoozy's own and is returned by the write rather than read back
/// from anything.**
pub trait CopiesAStreamIntoItsSink {
    /// Offers at most `may_write` bytes. **Returns how many were successfully
    /// retained**, which is what decrements the ceiling, or the refusal's own
    /// reason.
    fn write(&self, stream: Stream, bytes: &[u8], may_write: u64) -> Result<u64, String>;
}

#[cfg(test)]
mod tests {
    use smoozy_state::identities::IdentityKind;

    use super::*;

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    fn an_ordinary_stream() -> WhatTheWorkerObserved {
        WhatTheWorkerObserved {
            stream: Stream::Ordinary,
            retained: 5_000,
            discarded: 0,
            first_loss_reason: None,
            end_of_stream: true,
            the_marker_passed: Some(true),
            something_followed_the_marker: Some(true),
        }
    }

    fn a_complaints_stream() -> WhatTheWorkerObserved {
        WhatTheWorkerObserved {
            stream: Stream::Complaints,
            retained: 0,
            discarded: 0,
            first_loss_reason: None,
            end_of_stream: true,
            the_marker_passed: None,
            something_followed_the_marker: None,
        }
    }

    #[test]
    fn the_ceiling_is_the_available_reading_less_the_reserve_and_never_negative() {
        assert_eq!(
            CaptureCeiling::from(THE_CONTROL_STATE_RESERVE + 1_000).remaining(),
            1_000
        );
        assert_eq!(CaptureCeiling::from(0).remaining(), 0);
        assert!(CaptureCeiling::from(THE_CONTROL_STATE_RESERVE).is_spent());
    }

    #[test]
    fn the_two_streams_draw_from_one_counter_and_only_retained_bytes_decrement_it() {
        let mut ceiling = CaptureCeiling::from(THE_CONTROL_STATE_RESERVE + 100);
        assert_eq!(ceiling.may_offer(400), 100);
        ceiling.retained(60);
        assert_eq!(ceiling.remaining(), 40);
        // The other stream draws from the same remainder.
        assert_eq!(ceiling.may_offer(400), 40);
        ceiling.retained(40);
        assert!(ceiling.is_spent());
        assert_eq!(ceiling.may_offer(1), 0);
    }

    /// M4a: no later reading raises it, and there is no route by which one
    /// could — the type exposes no way to grow the remainder.
    #[test]
    fn nothing_raises_a_spent_ceiling() {
        let mut ceiling = CaptureCeiling::from(THE_CONTROL_STATE_RESERVE);
        ceiling.retained(0);
        assert!(ceiling.is_spent());
    }

    #[test]
    fn a_placement_that_cannot_establish_the_reserve_does_not_admit_a_render() {
        assert_eq!(
            whether_a_placement_may_admit_a_render(THE_CONTROL_STATE_RESERVE - 1),
            WhetherAPlacementMayAdmitARender::NoRoomForControlState
        );
        assert_eq!(
            whether_a_placement_may_admit_a_render(THE_CONTROL_STATE_RESERVE),
            WhetherAPlacementMayAdmitARender::ItMay
        );
    }

    #[test]
    fn a_spent_ceiling_drains_and_discards_rather_than_stopping_the_render() {
        let spent = CaptureCeiling::from(THE_CONTROL_STATE_RESERVE);
        assert_eq!(
            what_the_worker_does(spent, false),
            WhatTheWorkerDoesWithTheRestOfTheStream::DrainsAndDiscards
        );
        let room = CaptureCeiling::from(THE_CONTROL_STATE_RESERVE + 10);
        assert_eq!(
            what_the_worker_does(room, false),
            WhatTheWorkerDoesWithTheRestOfTheStream::RetainsIt
        );
        // A refused sink write ends durable writes for that stream even with
        // room in the counter.
        assert_eq!(
            what_the_worker_does(room, true),
            WhatTheWorkerDoesWithTheRestOfTheStream::DrainsAndDiscards
        );
    }

    #[test]
    fn a_marker_and_something_after_it_is_collected() {
        assert_eq!(
            collection_outcome(true, false, &an_ordinary_stream(), false),
            CollectionOutcome::Collected
        );
    }

    /// Row 286's own case: the marker passed and nothing followed it.
    #[test]
    fn a_marker_and_nothing_after_it_is_collected_and_empty() {
        let observed = WhatTheWorkerObserved {
            something_followed_the_marker: Some(false),
            retained: 120,
            ..an_ordinary_stream()
        };
        assert_eq!(
            collection_outcome(true, false, &observed, false),
            CollectionOutcome::CollectedAndTheStreamWasEmpty
        );
    }

    /// And the other half of the pair: no marker at all is a run that did not
    /// reach the material.
    #[test]
    fn no_marker_at_all_is_never_established_and_not_an_empty_stream() {
        let observed = WhatTheWorkerObserved {
            the_marker_passed: Some(false),
            something_followed_the_marker: Some(false),
            retained: 0,
            ..an_ordinary_stream()
        };
        assert_eq!(
            collection_outcome(true, false, &observed, false),
            CollectionOutcome::NeverEstablished
        );
    }

    #[test]
    fn no_capture_record_is_never_established() {
        assert_eq!(
            collection_outcome(false, false, &an_ordinary_stream(), false),
            CollectionOutcome::NeverEstablished
        );
        // And the defensive case §14 forbids.
        assert_eq!(
            collection_outcome(true, true, &an_ordinary_stream(), false),
            CollectionOutcome::NeverEstablished
        );
    }

    #[test]
    fn no_end_of_stream_a_reached_ceiling_or_a_refused_write_is_collection_incomplete() {
        let no_end = WhatTheWorkerObserved {
            end_of_stream: false,
            ..an_ordinary_stream()
        };
        assert_eq!(
            collection_outcome(true, false, &no_end, false),
            CollectionOutcome::CollectionIncomplete
        );
        assert_eq!(
            collection_outcome(true, false, &an_ordinary_stream(), true),
            CollectionOutcome::CollectionIncomplete
        );
        let refused = WhatTheWorkerObserved {
            first_loss_reason: Some("No space left on device".into()),
            ..an_ordinary_stream()
        };
        assert_eq!(
            collection_outcome(true, false, &refused, false),
            CollectionOutcome::CollectionIncomplete
        );
    }

    /// M16: the complaints stream has no marker, so its two outcomes rest on the
    /// count and the end-of-stream report.
    #[test]
    fn the_complaints_stream_reads_its_outcome_off_the_count() {
        assert_eq!(
            collection_outcome(true, false, &a_complaints_stream(), false),
            CollectionOutcome::CollectedAndTheStreamWasEmpty
        );
        let spoke = WhatTheWorkerObserved {
            retained: 42,
            ..a_complaints_stream()
        };
        assert_eq!(
            collection_outcome(true, false, &spoke, false),
            CollectionOutcome::Collected
        );
    }

    #[test]
    fn reconciliation_writes_incomplete_for_an_established_record_with_no_end_reported() {
        assert_eq!(
            what_reconciliation_writes(true, false),
            Some(CollectionOutcome::CollectionIncomplete)
        );
        assert_eq!(what_reconciliation_writes(true, true), None);
        assert_eq!(what_reconciliation_writes(false, false), None);
    }

    #[test]
    fn an_ending_with_no_status_is_failed_and_nothing_stays_running_for_want_of_output() {
        assert_eq!(
            how_the_ending_reads(true, None),
            Some(HowTheEndingReads::AnEndingWithNoStatusWhichIsFailed)
        );
        assert_eq!(
            how_the_ending_reads(false, Some(0)),
            Some(HowTheEndingReads::WithAStatus(0))
        );
        // A group still live with nothing reaped is not an ending at all.
        assert_eq!(how_the_ending_reads(false, None), None);
    }

    #[test]
    fn a_jobs_state_comes_from_three_things_and_neither_stream_is_one_of_them() {
        // **The set is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so a fourth thing a job's
        // state comes from has no arm, the crate stops compiling, and whoever
        // added it is sent to this list — where the length assertion this
        // replaced could never have been false.
        for (at, comes_from) in WhatAJobsStateComesFrom::THE_ONLY_THREE.iter().enumerate() {
            let where_the_list_puts_it = match comes_from {
                WhatAJobsStateComesFrom::TheOperationRecord => 0,
                WhatAJobsStateComesFrom::TheExecutionGroupsValidation => 1,
                WhatAJobsStateComesFrom::TheReapedExitStatus => 2,
            };
            assert_eq!(at, where_the_list_puts_it, "{comes_from:?} is not where the list puts it");
        }
    }

    #[test]
    fn the_raw_record_is_offered_for_an_incomplete_stream_and_not_for_one_never_established() {
        assert!(a_raw_record_is_still_offered(CollectionOutcome::CollectionIncomplete));
        assert!(a_raw_record_is_still_offered(CollectionOutcome::Collected));
        assert!(a_raw_record_is_still_offered(
            CollectionOutcome::CollectedAndTheStreamWasEmpty
        ));
        assert!(!a_raw_record_is_still_offered(CollectionOutcome::NeverEstablished));
    }

    #[test]
    fn a_re_run_is_a_new_attempt_with_its_own_two_outcomes() {
        let job = a_job();
        let first = WrittenOnce {
            job: job.clone(),
            attempt: 1,
            ordinary: CollectionOutcome::CollectionIncomplete,
            complaints: CollectionOutcome::CollectedAndTheStreamWasEmpty,
        };
        let second = WrittenOnce {
            job,
            attempt: 2,
            ordinary: CollectionOutcome::Collected,
            complaints: CollectionOutcome::CollectedAndTheStreamWasEmpty,
        };
        assert_ne!(first, second);
        assert_ne!(first.attempt, second.attempt);
    }

    #[test]
    /// **M11 exercised rather than compared.** The assertion this replaced put
    /// two members of the enum against each other that were never the same
    /// value, so it held by construction and checked the named behaviour
    /// nowhere.
    fn a_partial_read_of_the_retained_prefix_is_not_a_reading() {
        assert_eq!(ARetainedPrefixRead::of(false), ARetainedPrefixRead::NotAReading);
        assert!(
            !ARetainedPrefixRead::of(false).is_a_reading(),
            "a read that did not complete stands for nothing, whatever bytes it brought"
        );
        assert_eq!(ARetainedPrefixRead::of(true), ARetainedPrefixRead::Whole);
        assert!(ARetainedPrefixRead::of(true).is_a_reading());
    }

    #[test]
    fn a_capture_record_names_two_sinks_and_carries_the_ceiling_by_value() {
        let record = CaptureRecord {
            job: a_job(),
            attempt: 1,
            ordinary_sink: "/opt/smoozy/state/streams/ordinary".into(),
            complaints_sink: "/opt/smoozy/state/streams/complaints".into(),
            ceiling: CaptureCeiling::from(THE_CONTROL_STATE_RESERVE + 1),
        };
        assert_ne!(record.ordinary_sink, record.complaints_sink);
        assert_eq!(record.ceiling.remaining(), 1);
    }

    #[test]
    fn a_capture_report_carries_the_exit_status_verbatim() {
        let report = WorkersCaptureReport {
            ordinary: an_ordinary_stream(),
            complaints: a_complaints_stream(),
            exit_status: Some(143),
        };
        assert_eq!(report.exit_status, Some(143));
    }

    /// **Each of the four reads back from its own words, and words that name
    /// none read back as nothing** — the one home every leg spells them from
    /// (`Queue & History` E9, the Creator's notes on 0.1.8, piece 6d).
    #[test]
    fn each_outcome_reads_back_from_its_own_words_and_nothing_else_does() {
        for outcome in CollectionOutcome::EVERY {
            assert_eq!(CollectionOutcome::from_words(outcome.in_words()), Some(outcome));
        }
        assert_eq!(CollectionOutcome::from_words(""), None);
        assert_eq!(CollectionOutcome::from_words("Collected"), None);
    }
}

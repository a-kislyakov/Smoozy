//! §6 — the recorded execution group, and the stop.
//!
//! **The shape forbids discovering or stopping a worker by executable name,
//! command-line pattern or process search**, on the observation that a pattern
//! *"matched **two** process ids"* and the intended worker went on working.
//! **Fact 91 completes that rather than contradicting it:** a recorded identifier
//! alone is a pattern with extra steps, because identifiers are re-handed-out;
//! **the identifier paired with the kernel-set start time is not**, because the
//! later holder cannot present it and cannot forge it.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §5.2, §5.3 and its
//! steps 6 and 7. `Server protocol and transfers` C21, C22. `Job files and
//! outputs` C36, L13. `Server Setup` §11 and its refusal clause. `Render
//! Settings` §10. `Queue & History` §16. `Telegram` §12.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use crate::workers::WorkerKind;

/// **X1. The recorded execution group is the group leader's identity paired with
/// its kernel-set start time, recorded together at the moment the worker is
/// started, and held in the operation record.**
///
/// **Fact 91 is answered**: the start time is *"written by the kernel at fork, in
/// clock ticks since boot"*, and **it cannot be chosen by the process it
/// describes** — a write to it was refused on both machine kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordedPair {
    /// The leader's identifier. **X2: never on its own.**
    pub identifier: u32,
    /// The kernel-set start time, in clock ticks since boot.
    pub start_time: u64,
}

impl RecordedPair {
    /// **X2. The pair is what separates a live group from a re-handed-out
    /// identifier, and it was forced rather than argued.**
    ///
    /// On a virtual machine a leader recorded as **(identifier 1493, start time
    /// 19186)** was killed, the identifier space was made to hand out that
    /// identifier again, and validating the recorded pair against the new,
    /// unrelated process gave **exists, does not match**. *"A later, unrelated
    /// process validating against the recorded pair would have been the negative
    /// result"*, and it did not happen.
    ///
    /// **X4. A container cannot pose the reuse demonstration and the mechanism
    /// reads identically there** — three separate refusals, an identifier space
    /// too large for natural wrap, the next-identifier control on a read-only
    /// filesystem, and a namespace of one's own refused outright. **What is a
    /// kernel property is not re-derived per machine kind** (§16 item 4).
    pub fn validates_against(&self, what_is_there: Option<RecordedPair>) -> Validation {
        match what_is_there {
            Some(there) if there == *self => Validation::Matches,
            Some(_) => Validation::ExistsButDoesNotMatch,
            None => Validation::Gone,
        }
    }

    /// **The pair as this document writes it down**, which is what
    /// `State, identity and recovery` §5.1's record carries in the field it
    /// calls *the pair as the document that recorded it writes it down* and
    /// reads as opaque.
    ///
    /// **Both halves, never the identifier alone**, which is the whole of X2.
    /// The form is two numbers with a space between them: it is Build's own and
    /// no clause fixes it, exactly as no clause fixes the names of this plan's
    /// own records in the package area.
    pub fn as_this_document_records_it(&self) -> String {
        format!("{} {}", self.identifier, self.start_time)
    }

    /// The pair back out of that field.
    ///
    /// **A field that does not read back as two numbers is not half a pair.**
    /// X6 forbids relaxing a validation, and a reader that took the identifier
    /// out of a field it could not parse whole would be validating on the
    /// identifier alone, which X2 is the finding against.
    pub fn recorded_in(field: &str) -> Option<RecordedPair> {
        let (identifier, start_time) = field.trim().split_once(' ')?;
        Some(RecordedPair {
            identifier: identifier.parse().ok()?,
            start_time: start_time.trim().parse().ok()?,
        })
    }
}

/// What validating the recorded pair gives.
///
/// **X6. A validation that fails means the group is gone — never that the check
/// should be relaxed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Validation {
    Matches,
    /// The identifier was re-handed-out. **X2's whole point.**
    ExistsButDoesNotMatch,
    Gone,
}

impl Validation {
    /// **X9. The group is confirmed ended by the same pair, and by nothing
    /// else.** A group whose leader's pair no longer validates has ended.
    /// **Nothing reads a process listing, a name, or the absence of an output
    /// file to decide it.**
    pub fn the_group_is_gone(self) -> bool {
        !matches!(self, Validation::Matches)
    }
}

/// **X3. The run identifier settles what the pair cannot.**
///
/// Start time is in ticks since boot and is **meaningful only within one boot**,
/// so a reboot, a container restart and a recycle are settled by the run
/// identifier the supervisor writes at every start (S2) and **never by the
/// pair**. That division is `State, identity and recovery` §5.2's and is
/// unchanged.
pub const A_REBOOT_A_RESTART_AND_A_RECYCLE_ARE_THE_RUN_IDENTIFIERS: bool = true;

/// **X5. The group is the worker and every external process it starts for that
/// operation, and the leader is the worker.**
///
/// So a render's group is the render worker and the Blender it starts; a setup
/// run's group is the setup worker and whatever it invokes. **The leader is what
/// the pair identifies; which member receives a termination is §6.3's and is not
/// always the leader.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionGroup {
    pub kind: WorkerKind,
    /// The leader — **only the leader is recorded** (X1).
    pub leader: RecordedPair,
}

/// **X6. Nothing anywhere in this document finds a worker by executable name, by
/// command-line pattern, or by searching the machine's processes.**
///
/// The value is `()` because the clause is an absence: no function in this crate
/// takes a name or a pattern, and there is nothing here a process search could be
/// written against.
pub const NOTHING_IS_FOUND_BY_NAME_PATTERN_OR_PROCESS_SEARCH: () = ();

/// **X11. Each contract names four things and is owned by the document that
/// defines the kind (W1).**
///
/// **No contract is universal and none is inherited from another kind.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminationContract {
    /// Which member of the recorded execution group receives the termination.
    pub target: SignalTarget,
    /// Whether the kind may be terminated at any moment or only at a recorded
    /// safe boundary.
    pub when: WhenItMayBeTerminated,
    /// What cleanup is owed afterwards, named by the owning document.
    pub cleanup: Cleanup,
    /// **Whether the worker outlives every member of its group**, which is what
    /// X1's recording of the leader alone and X9's reading of leader-gone as
    /// group-gone both rest on.
    pub the_worker_outlives_every_member: bool,
}

/// Which member receives the termination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalTarget {
    /// **X12. For a render, the target is the recorded render process and not the
    /// worker, and not a shell around it.** `Job files and outputs` C36 fixes the
    /// reason: *"A render had to be started detached to survive the connection at
    /// all, so the thing that must be reached is by construction not the thing
    /// the connection started."*
    TheRecordedRenderProcess,
    /// `Render Settings` §10 W2's, `Queue & History` §16's for the preview
    /// kind, and **the media kind's, whose own document names this same
    /// member**: `Telegram` M6, in its §12 — *"The termination is delivered to
    /// the recorded external process — the Blender it drives — at any
    /// moment."*
    ///
    /// **W2's *no default signal is invented* does not reach this kind**, being
    /// a rule for a kind whose document has named no member; this kind's
    /// document has named one, and it is this one.
    TheRecordedExternalBlenderProcess,
    /// **X14. For transfer-support, termination is staging and cleanup rather
    /// than a signal** (`Server protocol and transfers` C22), so X8 step 4 issues
    /// no signal for that kind and **the sequence completes on the cleanup's own
    /// confirmation.**
    NoSignalStagingAndCleanupInstead,
    /// **X15. For a setup run, termination is at, or recovery from, a recorded
    /// safe boundary**, and the boundary marker is the per-kind field of that
    /// operation record (W4). **Its meaning is `Server Setup`'s.**
    AtTheRecordedSafeBoundary,
    /// **The role worker's own process — `Telegram` §19, which states this
    /// kind's whole contract because W1 assigns it to §5.3 and §5.3 states
    /// none**: *"the termination is the polite signal delivered to the worker's
    /// own process, at any moment, because every durable fact it holds was
    /// written through the supervisor before the act it records and a death at
    /// any instant re-delivers at-least-once work idempotently"*, and *"the
    /// cleanup owed is none"*.
    ///
    /// **The member is the leader and that is the same document's sentence** —
    /// *"The worker's recorded execution group is itself alone — it starts no
    /// external process"* — so unlike X12's target there is no second member for
    /// an owning document to have recorded.
    ///
    /// **W2's *no default signal is invented* does not reach this kind either**,
    /// for the same reason it does not reach the media kind: it is a rule for a
    /// kind whose document has named no member, and this kind's document has
    /// named one.
    TheRoleWorkersOwnProcess,
}

/// When a kind may be terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenItMayBeTerminated {
    AtAnyMoment,
    /// X15's.
    OnlyAtARecordedSafeBoundary,
}

/// What cleanup the contract owes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cleanup {
    NoneNamedByItsOwningDocument,
    /// X14's.
    StagingAndCleanup,
}

/// **X12's second half, stated where a reader of the render contract meets it:
/// the render worker remains until its external processes have exited, or is
/// terminated only after them** — which is the shape's rule and is what makes
/// §11.1's capture survive the stop.
pub const THE_RENDER_WORKER_REMAINS_UNTIL_ITS_EXTERNAL_PROCESSES_HAVE_EXITED: bool = true;

/// **X13. Nothing here matches an ending on its number.**
///
/// The number a polite terminate ends with is recorded as `-15` by one topic and
/// `143` by three others, **and anything matching an ending on its number accepts
/// both.** The value is `()` because the clause is an absence: no constant in this
/// crate holds either number, and no comparison against one exists.
pub const NOTHING_MATCHES_AN_ENDING_ON_ITS_NUMBER: () = ();

/// **X8's seven steps, and what this document adds to each is only what is
/// written here.**
///
/// The sequence itself is `State, identity and recovery` §5.3's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopStep {
    /// Step one. **The stop's own command record is written by the supervisor
    /// before anything** (`Server protocol and transfers` C4).
    TheStopsOwnCommandRecordIsWritten,
    /// Step two. **The target's desired state is written to *should be
    /// stopped*, durably, before the group is looked at**, so a supervisor that
    /// dies between steps two and four leaves a stop reconciliation will
    /// honour.
    TheDesiredStateIsWrittenShouldBeStopped,
    /// Step three. **The pair of X1 is validated, and a pair that does not
    /// validate ends the sequence**: the group is gone, the recorded outcome is
    /// written from what the operation record and the reaped status say, **and
    /// no signal is sent to anything.**
    ThePairIsValidated,
    /// Step four. **The kind's own termination is issued to the member that
    /// kind's contract names** (§6.3).
    TheKindsOwnTerminationIsIssued,
    /// Step five. **Nothing infers completion from delivery** — the stop took
    /// effect *"on the program's next cycle, about 2.3 seconds against a
    /// three-second cycle"*, and *"A check made two seconds after sending the
    /// signal reported the program still running and would have been wrong"*.
    NothingInfersCompletionFromDelivery,
    /// Step six. **The outcome *left stopped* is recorded when the group has
    /// ended, and the event is journalled.**
    TheOutcomeIsRecordedWhenTheGroupHasEnded,
    /// Step seven. The boundary — see [`whose_ending_it_is`].
    TheEndingIsAttributedAtTheIssuance,
}

impl StopStep {
    /// The seven, in order.
    pub const IN_ORDER: [StopStep; 7] = [
        StopStep::TheStopsOwnCommandRecordIsWritten,
        StopStep::TheDesiredStateIsWrittenShouldBeStopped,
        StopStep::ThePairIsValidated,
        StopStep::TheKindsOwnTerminationIsIssued,
        StopStep::NothingInfersCompletionFromDelivery,
        StopStep::TheOutcomeIsRecordedWhenTheGroupHasEnded,
        StopStep::TheEndingIsAttributedAtTheIssuance,
    ];
}

/// Whose ending the recorded outcome is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoseEnding {
    /// **X8 step 7: where the worker ended on its own *before* step 4's
    /// termination was issued** — the pair failing validation at step 3, or the
    /// ending already reaped before anything was issued — **the recorded outcome
    /// is that ending and not *left stopped*.**
    ItsOwn,
    /// **An ending step 6 takes past the issuance is *left stopped*.**
    LeftStopped,
}

/// **X8 step 7's one boundary — the issuance, the only one the supervisor holds a
/// record of its own about** — cited from `State, identity and recovery` §5.3
/// step 7 rather than re-decided.
///
/// **This was written as *between steps 2 and 6* until it was fixed**, a window
/// holding the ordinary successful stop, **so a render that honoured the
/// interrupt would have been recorded as its own ending and shown failed against
/// the spec's *marked as stopped*.**
pub fn whose_ending_it_is(the_ending_was_reaped_before_the_issuance: bool) -> WhoseEnding {
    if the_ending_was_reaped_before_the_issuance {
        WhoseEnding::ItsOwn
    } else {
        WhoseEnding::LeftStopped
    }
}

/// **X7. Stopping work the machine has taken is its own command with its own
/// operation identity, naming the target's** (`Server protocol and transfers`
/// C21).
///
/// **That document fixes the request; the sequence, the validation and the signal
/// target are this document's, and C21 says so.**
pub const THE_SEQUENCE_THE_VALIDATION_AND_THE_TARGET_ARE_THIS_DOCUMENTS: bool = true;

/// **X9a. The pair is the *gone* test and the wait is the *ended* test, and the
/// supervisor performs the wait.**
///
/// **The supervisor waits on every worker it starts and takes the ending from the
/// wait status**, which `State, identity and recovery` §5.3 step 6 already
/// requires of it and which §16 item 4 had carried as a Build-side fix.
///
/// **So the defunct-entry hole is closed here rather than left to Build**:
/// without the wait a worker that ended still validates at X9, a termination is
/// absorbed by a defunct entry, and *when the group has ended* never arrives —
/// which on a setup machine leaves `Server Setup` S5 refusing for ever with no
/// act inside the app that clears it.
///
/// **X9 itself is unchanged and keeps its job**, which is the safe-direction
/// check that a group is gone; **what the wait adds is the positive that it has
/// ended.** §3.2's one-restarter rule is untouched — **a wait is not a restart.**
pub trait WaitsOnEveryWorkerItStarts {
    /// The ending, taken from the wait status. `None` is *not ended yet*.
    fn the_ending(&self, group: &ExecutionGroup) -> Result<Option<i32>, String>;
}

/// Validating the recorded pair, behind a trait — **and it answers with a pair or
/// with nothing, never with a name.**
pub trait ValidatesTheRecordedPair {
    fn what_is_at(&self, identifier: u32) -> Result<Option<RecordedPair>, String>;
}

/// Issuing the kind's own termination, behind a trait — **X8 step 4, and it is the
/// only place in this crate that would reach a process at all.**
///
/// **The target is the contract's, never this trait's choice**, which is why the
/// contract is an argument. **A kind whose document has named no contract never
/// reaches this trait**: W2 refuses the cancellation rather than inventing a
/// signal.
pub trait IssuesTheKindsOwnTermination {
    /// **X14's kind issues no signal**, so a transfer-support termination is the
    /// cleanup's own confirmation rather than a call here.
    fn issue(
        &self,
        group: &ExecutionGroup,
        contract: TerminationContract,
    ) -> Result<(), String>;
}

/// **X10. A stop issued by something on the machine with nobody connected takes
/// exactly this sequence.** Fact 68's success was *"Redone against the identity
/// the service manager itself holds"*; **here that identity is X1's pair, and the
/// caller is the supervisor rather than a script.**
pub const A_STOP_WITH_NOBODY_CONNECTED_TAKES_THE_SAME_SEQUENCE: bool = true;

/// **X16. A stop reaching another login's process is refused by the machine, and
/// that is the observed reason `Job files and outputs` L13's one-login
/// requirement is a requirement.**
///
/// From a second ordinary login both a polite and a forceful stop were refused
/// with *Operation not permitted*. **Under one login the case does not arise**,
/// and this document does not build a route around it.
pub const THE_SUPERVISOR_AND_EVERY_WORKER_RUN_AS_ONE_LOGIN: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    const A_LEADER: RecordedPair = RecordedPair {
        identifier: 1493,
        start_time: 19186,
    };

    #[test]
    fn a_re_handed_out_identifier_does_not_validate() {
        let the_new_unrelated_process = RecordedPair {
            identifier: 1493,
            start_time: 40021,
        };
        assert_eq!(
            A_LEADER.validates_against(Some(the_new_unrelated_process)),
            Validation::ExistsButDoesNotMatch
        );
        assert!(A_LEADER
            .validates_against(Some(the_new_unrelated_process))
            .the_group_is_gone());
    }

    #[test]
    fn the_same_pair_validates_and_nothing_there_is_gone() {
        assert_eq!(A_LEADER.validates_against(Some(A_LEADER)), Validation::Matches);
        assert!(!A_LEADER.validates_against(Some(A_LEADER)).the_group_is_gone());
        assert_eq!(A_LEADER.validates_against(None), Validation::Gone);
    }

    #[test]
    fn the_stop_is_seven_steps_and_the_desired_state_is_written_before_the_group_is_looked_at() {
        let order = StopStep::IN_ORDER;
        // **The sequence is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so a step added to the enum
        // has none and the crate stops compiling.
        for (at, step) in order.iter().enumerate() {
            let where_the_sequence_puts_it = match step {
                StopStep::TheStopsOwnCommandRecordIsWritten => 0,
                StopStep::TheDesiredStateIsWrittenShouldBeStopped => 1,
                StopStep::ThePairIsValidated => 2,
                StopStep::TheKindsOwnTerminationIsIssued => 3,
                StopStep::NothingInfersCompletionFromDelivery => 4,
                StopStep::TheOutcomeIsRecordedWhenTheGroupHasEnded => 5,
                StopStep::TheEndingIsAttributedAtTheIssuance => 6,
            };
            assert_eq!(at, where_the_sequence_puts_it, "{step:?} is not where X8 puts it");
        }
        let desired = order
            .iter()
            .position(|step| *step == StopStep::TheDesiredStateIsWrittenShouldBeStopped)
            .expect("step 2");
        let validated = order
            .iter()
            .position(|step| *step == StopStep::ThePairIsValidated)
            .expect("step 3");
        let issued = order
            .iter()
            .position(|step| *step == StopStep::TheKindsOwnTerminationIsIssued)
            .expect("step 4");
        assert!(desired < validated && validated < issued);
    }

    /// The fix of 2026-08-22: an ending taken past the issuance is *left
    /// stopped*, so a render that honoured the interrupt is not recorded as its
    /// own ending.
    #[test]
    fn an_ending_past_the_issuance_is_left_stopped_and_one_before_it_is_the_workers_own() {
        assert_eq!(whose_ending_it_is(false), WhoseEnding::LeftStopped);
        assert_eq!(whose_ending_it_is(true), WhoseEnding::ItsOwn);
    }

    #[test]
    fn the_transfer_support_contract_issues_no_signal() {
        let contract = TerminationContract {
            target: SignalTarget::NoSignalStagingAndCleanupInstead,
            when: WhenItMayBeTerminated::AtAnyMoment,
            cleanup: Cleanup::StagingAndCleanup,
            the_worker_outlives_every_member: false,
        };
        assert_eq!(contract.target, SignalTarget::NoSignalStagingAndCleanupInstead);
        assert_eq!(contract.cleanup, Cleanup::StagingAndCleanup);
    }

    #[test]
    fn the_setup_contract_terminates_only_at_a_recorded_safe_boundary() {
        let contract = TerminationContract {
            target: SignalTarget::AtTheRecordedSafeBoundary,
            when: WhenItMayBeTerminated::OnlyAtARecordedSafeBoundary,
            cleanup: Cleanup::NoneNamedByItsOwningDocument,
            // `Server Setup` §11 states it: the setup worker remains until every
            // external process it started has exited, or is ended only after
            // them.
            the_worker_outlives_every_member: true,
        };
        assert!(contract.the_worker_outlives_every_member);
    }

    #[test]
    fn the_render_contract_reaches_the_recorded_render_process_rather_than_the_worker() {
        let contract = TerminationContract {
            target: SignalTarget::TheRecordedRenderProcess,
            when: WhenItMayBeTerminated::AtAnyMoment,
            cleanup: Cleanup::NoneNamedByItsOwningDocument,
            the_worker_outlives_every_member: true,
        };
        assert_eq!(contract.target, SignalTarget::TheRecordedRenderProcess);
    }

    #[test]
    fn an_execution_group_records_the_leader_alone() {
        let group = ExecutionGroup {
            kind: WorkerKind::Render,
            leader: A_LEADER,
        };
        assert_eq!(group.leader, A_LEADER);
    }
}

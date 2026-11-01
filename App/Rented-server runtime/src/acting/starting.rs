//! **Starting a worker as one unit**: the admission point, then the operation
//! record, the sinks and the start, in the order §14 declares them.
//!
//! # What this module is, and what it is not
//!
//! **[`crate::arbitration`] holds the admission rules**, [`crate::acting::workers`]
//! the start itself, [`crate::acting::capture`] the two sinks, and
//! `State, identity and recovery` §5.1's store the record. **Every one of them
//! was built at step 6 and had no caller.** This module is the caller: it puts
//! the four in one order and refuses to do any of them out of it.
//!
//! **There are two starts here and not one, because §5 divides the kinds.**
//! [`start_one_worker`] is §14's one unit and it is §5.2's — the operation
//! record, the sinks and the start, for the six per-operation kinds.
//! [`start_the_role_worker`] is §5.3's, and it writes no operation record
//! because **W7 says the one role worker has none**. They share the admission
//! point, the wait and the start, which is A4's *every worker start passes
//! through it* kept rather than restated.
//!
//! **It does not decide what a worker of a kind is.** W1's table gives that to
//! the document that owns each kind, and [`crate::acting::workers`] takes the
//! invocation as something handed in. Nothing here guesses one from a kind.
//!
//! **It does not compose the per-kind field.** W4 makes that the kind's
//! document's, so it arrives here as bytes and is written down rather than made
//! — with one check, M1's, described at [`WhatIsStarted`].
//!
//! # Why the admission point is a value here rather than a free reading
//!
//! **A4 gives the machine one admission point, held by the supervisor**, and A5
//! and A5a are both decided against *what is admitted on this machine*. So there
//! has to be somewhere that is; [`TheAdmissionPoint`] is it, and the supervisor
//! holds one.
//!
//! **What takes an operation out of it is its group having ended, and the wait is
//! how that is learnt.** X9a puts the wait in the supervisor's hands and makes it
//! the positive test that a group has ended, and X9 makes the pair the *gone*
//! test. **It is taken at the moment an admission is decided and never on a
//! schedule** — there is no timer here, no watcher and no thread.
//!
//! **What is NOT done here, said rather than implied: nothing writes the ending
//! into the operation record.** `State, identity and recovery` §5.3 step 6 and §6
//! are what record an outcome, and neither is this module. So an operation that
//! ends leaves this admission point and its record stays open until one of those
//! two writes the outcome. **§6's loop is built and runs** — the machine's own
//! program performs it at every supervisor start and at the clear of the
//! setup-in-progress state — and this sentence said it was not built until
//! 2026-09-12, which was true when it was written and stopped being true when
//! that loop landed.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` — B8, C4.
//!
//! **A marker this plan also defines is never listed here and is written out with
//! its document at the site**, because a header list cannot tell the two apart and
//! a bare one there would resolve to a real and wrong clause.

use smoozy_state::identities::Identity;
use smoozy_state::machine_operations::{
    Keeps, OperationRecord, Refused, TheRecordedExecutionGroup,
};

use crate::arbitration::{
    admit, the_declaration_in_force_for, Admission, Operation, SetupInProgress, WhatIsAdmitted,
};
use crate::capture::{CaptureRecord, MakesTheSinks};
use crate::execution_group::{ExecutionGroup, RecordedPair, WaitsOnEveryWorkerItStarts};
use crate::the_program::the_word_for;
use crate::workers::{StartedWorker, StartsWorkers, WhatAStartAnswers, WorkerKind};

/// **What one worker is started for**, composed by whoever asks and never by
/// this module.
///
/// # The one check this module makes on the per-kind field, and why it is a
/// check rather than a composition
///
/// **M1 requires the two sinks' paths to be recorded in the operation record's
/// per-kind field before the worker is started**, and **W4 makes that field the
/// kind's document's**. Both are kept by refusing a start whose capture record
/// names sinks the handed-in field does not carry: the field stays the owning
/// document's to shape, and a worker is never started against a record that
/// cannot say where its streams went. **M13's *never established* names the
/// opposite case defensively** — a worker started before its sinks were made,
/// which §14 forbids — and the ordering below is what forbids it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatIsStarted {
    /// **`Server protocol and transfers` C4's idempotency key**, and the
    /// operation the record is written under.
    pub operation: Identity,
    /// W1's kind.
    pub kind: WorkerKind,
    /// What the operation acts on — a job, a setup run, a transfer.
    pub subject: Identity,
    /// **S2's run identifier**, which the worker's report carries (W12) and
    /// which `State, identity and recovery` §5.1's record holds beside the pair.
    pub under_run: Identity,
    /// **M1's two sinks, where the kind has a capture record.** A kind with none
    /// has no sinks to make: §11's whole subject is a render's two streams, and
    /// W4 puts the capture record in the render kind's per-kind field.
    pub capture: Option<CaptureRecord>,
    /// **W4's per-kind field, opaque here.** The owning document reads it; this
    /// unit writes it down.
    pub the_kinds_own_field: Option<Vec<u8>>,
}

/// What the start of one worker produces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheStartOfAWorkerProduces {
    /// **A4: the admission point answered something other than *admitted*.**
    /// Nothing was written and nothing was started — A7's *it is not started and
    /// then blocked* kept by there being nothing before the decision.
    ItWasNotAdmitted(Admission),
    /// **§14: where the record or the sinks cannot be made, the worker is not
    /// attempted, naming what stopped it.**
    TheWorkerWasNotAttempted(String),
    /// **X1's pair, recorded at the moment the worker was started.**
    ItStarted(RecordedPair),
    /// **The worker is running and its group did not reach the record.**
    ///
    /// **It is its own answer rather than a failure**, because the two are
    /// different things to whoever meets them: the process is alive, it is in
    /// X9a's wait, it is admitted, and what is missing is the durable half.
    /// **S14's first bullet is what holds** — a durable write refused does not
    /// end the run. **S14's hold-and-re-attempt mechanism exists** at
    /// [`crate::supervisor::PendingWrites`] and this write is not one of the
    /// things put into it: what it holds are the durable writes owed **about
    /// work that has already happened**, and a group recorded late would be a
    /// pair written against a worker whose ending may already have been taken.
    /// So the reason is said here and is not hidden.
    ///
    /// **The pair is absent where none could be taken at all.** A child that ran
    /// and whose kernel record could not then be read has no pair, and none is
    /// invented: `State, identity and recovery` §2.1 rule 5 forbids a bare
    /// identifier as a runtime identity.
    ItStartedAndItsGroupCouldNotBeRecorded(Option<RecordedPair>, String),
}

/// **A4's one admission point per machine, as the supervisor holds it.**
///
/// **There is no second scheduler and no per-kind queue**, which is that
/// clause's own sentence; a single point is what makes exclusion a property
/// rather than a convention.
///
/// **A10 is kept by absence.** A held job is not at the admission point at all,
/// so nothing here has a *held* state and no caller passes a held subject in.
#[derive(Debug, Default)]
pub struct TheAdmissionPoint {
    admitted: Vec<(Operation, ExecutionGroup)>,
}

impl TheAdmissionPoint {
    /// A supervisor's start: nothing is admitted.
    pub fn holding_nothing() -> Self {
        TheAdmissionPoint::default()
    }

    /// **What is admitted on this machine**, as A5 and A5a are decided against.
    pub fn what_is_admitted(&self) -> WhatIsAdmitted {
        WhatIsAdmitted {
            operations: self
                .admitted
                .iter()
                .map(|(operation, _)| operation.clone())
                .collect(),
        }
    }

    /// How many operations stand admitted.
    pub fn how_many_it_holds(&self) -> usize {
        self.admitted.len()
    }

    /// **The group one admitted operation is held by**, or nothing where that
    /// operation is not admitted (any more).
    ///
    /// **Where no pair could be taken, this is the wait's own key and not a
    /// recorded pair** — [`the_waits_own_key_for`] says what that is. Added
    /// 2026-10-16 for the role worker the supervisor started with no pair: the
    /// program holds it by this key so that a later W8 act cannot start a
    /// second worker beside it, and so that its requests can be matched to the
    /// process that makes them while it is still unreaped.
    pub fn the_group_admitted_under(&self, operation: &Identity) -> Option<ExecutionGroup> {
        self.admitted
            .iter()
            .find(|(admitted, _)| admitted.identity == *operation)
            .map(|(_, group)| group.clone())
    }

    /// **X9a's wait, taken at the moment an admission is decided.**
    ///
    /// **The supervisor waits on every worker it starts and takes the ending
    /// from the wait status**, and an operation whose group has ended is not
    /// admitted any more. **Nothing here is a schedule**: this runs when a
    /// decision is being made and at no other moment, and it writes nothing.
    ///
    /// **W5 is untouched.** A worker that exits is an observation; what happens
    /// next comes from the operation record's desired state at the next
    /// reconciliation, and nothing here restarts anything.
    ///
    /// Answers with the operations that left, and the ending each was taken at.
    pub fn take_the_endings(
        &mut self,
        wait: &dyn WaitsOnEveryWorkerItStarts,
    ) -> Vec<(Identity, i32)> {
        let mut ended = Vec::new();
        self.admitted.retain(|(operation, group)| {
            match wait.the_ending(group) {
                Ok(Some(ending)) => {
                    ended.push((operation.identity.clone(), ending));
                    false
                }
                // **Not ended, and a wait this supervisor could not take, both
                // keep the operation admitted.** X6's direction: a check that
                // did not answer never relaxes anything.
                Ok(None) | Err(_) => true,
            }
        });
        ended
    }

    /// **An operation is admitted only once its worker is genuinely started.**
    ///
    /// A start that was not attempted holds no resource, so putting it here
    /// would exclude the next asker on nothing.
    fn now_admitted(&mut self, operation: Operation, group: ExecutionGroup) {
        self.admitted.push((operation, group));
    }
}

/// **§14's one unit — the whole start, in one place, because that is what the
/// declaration means.**
///
/// *"starting a worker | the whole start | §9.2 test 3 — the worker is unsafe
/// without its operation record and its sinks, so the ordered set is one unit |
/// **one unit.** W3 and M1 before the start; where either fails the worker is
/// **not attempted**, naming what stopped it."*
///
/// The order, and every step of it is a clause:
///
/// 1. **X9a's wait taken**, so A5 and A5a decide against what is admitted now.
/// 2. **A4's admission** — A5, A5a, A5b, A6, A7, A9, A11 and A11a, all of them
///    in [`admit`], which is the single point.
/// 3. **W3's durable operation record**, written before the worker is started,
///    through `Server protocol and transfers` C4's record-before-acting. **A
///    repeat of an identity already recorded never acts again**, which the store
///    refuses rather than this function checking for.
/// 4. **M1's two sinks**, where the kind has a capture record.
/// 5. **The start**, which composes X1's pair and puts the worker into X9a's
///    wait.
/// 6. **The pair written into the record.** X1 holds it *in the operation
///    record*, and the pair does not exist until step 5 — so this is the one act
///    that cannot come before the start, and the answer above says plainly where
///    it did not land.
///
/// **Y1: no unit here is retried inside its action.** Nothing below tries
/// anything twice.
#[allow(clippy::too_many_arguments)]
pub fn start_one_worker(
    what: &WhatIsStarted,
    point: &mut TheAdmissionPoint,
    setup_in_progress: Option<&SetupInProgress>,
    records: &mut dyn Keeps,
    sinks: &dyn MakesTheSinks,
    start: &dyn StartsWorkers,
    wait: &dyn WaitsOnEveryWorkerItStarts,
) -> WhatTheStartOfAWorkerProduces {
    point.take_the_endings(wait);

    let asking = Operation {
        identity: what.operation.clone(),
        kind: what.kind,
        declaration: the_declaration_in_force_for(what.kind),
    };
    let admission = admit(&asking, &point.what_is_admitted(), setup_in_progress);
    if admission != Admission::Admitted {
        return WhatTheStartOfAWorkerProduces::ItWasNotAdmitted(admission);
    }

    // **M1's own requirement, checked before anything durable is written.** The
    // field is the kind's document's (W4) and this is the one thing this unit
    // asks of it.
    if let Some(capture) = &what.capture {
        if let Err(why) = the_field_names_both_sinks(capture, what.the_kinds_own_field.as_deref()) {
            return WhatTheStartOfAWorkerProduces::TheWorkerWasNotAttempted(why);
        }
    }

    let mut record = OperationRecord::opened(
        what.operation.clone(),
        what.subject.clone(),
        the_word_for(what.kind),
    );
    record.the_kinds_safe_boundary_marker = what.the_kinds_own_field.clone();
    if let Err(why) = records.record_before_acting(&record) {
        return WhatTheStartOfAWorkerProduces::TheWorkerWasNotAttempted(format!(
            "the {} worker was not attempted: its operation record was not written: {}",
            the_word_for(what.kind),
            what_the_store_said(&why)
        ));
    }

    if let Some(capture) = &what.capture {
        if let Err(why) = sinks.make(capture) {
            return WhatTheStartOfAWorkerProduces::TheWorkerWasNotAttempted(format!(
                "the {} worker was not attempted: its two sinks were not made: {why}",
                the_word_for(what.kind)
            ));
        }
    }

    let pair = match start.start(&StartedWorker {
        kind: what.kind,
        operation: what.operation.clone(),
        under_run: what.under_run.clone(),
    }) {
        WhatAStartAnswers::ItStarted(pair) => pair,
        WhatAStartAnswers::ItWasNotAttempted(why) => {
            return WhatTheStartOfAWorkerProduces::TheWorkerWasNotAttempted(why)
        }
        // **The child ran, so it is not *not attempted*.** It is admitted, so
        // A5 and A5a decide against a machine that really is holding this
        // worker's resources, and it is in X9a's wait, so its ending is taken —
        // both of which the *not attempted* answer used to lose. **The group
        // this admits under is the wait's own key and never a recorded pair**:
        // the start time is the one the wait already holds for a child with no
        // reading, and nothing writes it into the operation record.
        WhatAStartAnswers::ItStartedAndNoPairCouldBeTaken { identifier, why } => {
            point.now_admitted(
                asking,
                ExecutionGroup {
                    kind: what.kind,
                    leader: the_waits_own_key_for(identifier),
                },
            );
            return WhatTheStartOfAWorkerProduces::ItStartedAndItsGroupCouldNotBeRecorded(
                None, why,
            );
        }
    };

    let group = ExecutionGroup {
        kind: what.kind,
        leader: pair,
    };
    record.execution_group = Some(TheRecordedExecutionGroup {
        as_that_document_recorded_it: pair.as_this_document_records_it(),
        started_under_run: what.under_run.clone(),
    });
    if let Err(why) = records.write_it_back(&record) {
        point.now_admitted(asking, group);
        return WhatTheStartOfAWorkerProduces::ItStartedAndItsGroupCouldNotBeRecorded(
            Some(pair),
            what_the_store_said(&why),
        );
    }

    point.now_admitted(asking, group);
    WhatTheStartOfAWorkerProduces::ItStarted(pair)
}

/// **§5.3's start, and it is deliberately not §14's one unit above.**
///
/// **W7: the role worker takes none of the four operation outcomes and has no
/// operation record**, so W3 — which is §5.2's, over per-operation workers — does
/// not reach it and nothing here writes one. **§11's two sinks are a render's
/// streams** and the role kind carries no capture record, so there are none to
/// make either. What is left of the order above is the two acts that do bind
/// every worker start:
///
/// 1. **X9a's wait taken**, so the decision is made against what is admitted now.
/// 2. **A4's admission**, which *"every worker start passes through"* — and
///    A11a is the one this exists for: while the setup-in-progress state stands,
///    **not a role worker's work** is admitted, and the refusal names the run the
///    state holds.
/// 3. **The start**, which composes X1's pair and puts the worker into X9a's
///    wait exactly as any other worker's does.
///
/// **The identity it is admitted under is the command's own** — W8's set of the
/// desired state — because A4's point is decided against operation identities and
/// the role worker has none of its own. **Nothing durable is written under it**,
/// which is what keeps W7.
///
/// **W10's *starts it while enabled* is the caller's**, and so is *never
/// resurrects it after it is disabled*: this function starts one worker when it
/// is called and holds no policy.
pub fn start_the_role_worker(
    asked_under: &Identity,
    under_run: &Identity,
    point: &mut TheAdmissionPoint,
    setup_in_progress: Option<&SetupInProgress>,
    start: &dyn StartsWorkers,
    wait: &dyn WaitsOnEveryWorkerItStarts,
) -> WhatTheStartOfAWorkerProduces {
    point.take_the_endings(wait);

    let asking = Operation {
        identity: asked_under.clone(),
        kind: WorkerKind::Telegram,
        declaration: the_declaration_in_force_for(WorkerKind::Telegram),
    };
    let admission = admit(&asking, &point.what_is_admitted(), setup_in_progress);
    if admission != Admission::Admitted {
        return WhatTheStartOfAWorkerProduces::ItWasNotAdmitted(admission);
    }

    let pair = match start.start(&StartedWorker {
        kind: WorkerKind::Telegram,
        operation: asked_under.clone(),
        under_run: under_run.clone(),
    }) {
        WhatAStartAnswers::ItStarted(pair) => pair,
        WhatAStartAnswers::ItWasNotAttempted(why) => {
            return WhatTheStartOfAWorkerProduces::TheWorkerWasNotAttempted(why)
        }
        // **The bot is running.** Answering *not attempted* here made the
        // supervisor forget the worker it had just started, and W10's next look
        // would have started a second one — which `Telegram` section 19 says
        // there is never more than one of on a machine.
        WhatAStartAnswers::ItStartedAndNoPairCouldBeTaken { identifier, why } => {
            point.now_admitted(
                asking,
                ExecutionGroup {
                    kind: WorkerKind::Telegram,
                    leader: the_waits_own_key_for(identifier),
                },
            );
            return WhatTheStartOfAWorkerProduces::ItStartedAndItsGroupCouldNotBeRecorded(
                None, why,
            );
        }
    };
    point.now_admitted(
        asking,
        ExecutionGroup {
            kind: WorkerKind::Telegram,
            leader: pair,
        },
    );
    WhatTheStartOfAWorkerProduces::ItStarted(pair)
}

/// **What X9a's wait holds a child by where no pair could be taken for it.**
///
/// [`crate::acting::workers`] puts such a child into the wait under its
/// identifier with a start time of nought, because a wait needs one value to
/// hold it by. **This is that value and it is not a recorded pair**: nothing
/// writes it into an operation record, nothing validates against it, and X9's
/// *gone* test never sees it. It exists so a running worker can be admitted and
/// reaped rather than lost.
fn the_waits_own_key_for(identifier: u32) -> RecordedPair {
    RecordedPair {
        identifier,
        start_time: 0,
    }
}

/// **How the program a rented machine runs invokes a worker of a kind: its own
/// path, with that kind's word after it.**
///
/// **[`crate::the_program`] already composes both halves and this is the
/// caller.** A bare invocation is the SSH command adapter, one word selects the
/// supervisor, and the worker word with a kind selects a worker — so the
/// argument vector below is exactly what
/// [`crate::the_program::the_role_selected_by`] reads back.
///
/// **This is not a decision about what a worker of a kind is.** W1 gives that to
/// the document that owns each kind, and every one of them names the same
/// program because `Server protocol and transfers` B8 gives an artifact one
/// fixed entry-point name. **What is composed here is this plan's own
/// program's invocation**, which is this plan's to compose and nobody else's.
///
/// **The path is the active-package pointer's and never the resolved
/// executable's.** G7 makes the boot registration start whatever the pointer
/// names, the pointer carries the fixed entry-point name, and the directory it
/// sits in is the package area — so a worker invoked through the pointer finds
/// the same package area its supervisor did, and a package replaced under it is
/// the next worker's rather than a path a supervisor cached.
///
/// **What the mode does when it gets there is the owning document's**, and the
/// program says so itself. **Two of the seven are built** — the render kind
/// from step 10c piece 5 and the setup kind from its piece 8 — **and the other
/// five refuse by kind**, naming the document that owns what a worker of that
/// kind does. *(Corrected 2026-09-19: this read *today every one of the seven
/// refuses by kind*, which stopped being true when the render worker landed.)*
/// **A refusal is a real child process, started and reaped**, which is all this
/// side of the start promises either way.
pub fn how_the_program_invokes_a_worker(
    package_area: &str,
) -> crate::acting::workers::SaysHowThisWorkerIsInvoked {
    let pointer = crate::the_program::the_active_package_pointer_at(package_area);
    Box::new(move |worker| {
        Some(crate::acting::workers::HowAWorkerOfThisKindIsInvoked {
            program: pointer.clone(),
            arguments: vec![
                crate::the_program::THE_WORKER_WORD.to_string(),
                the_word_for(worker.kind).to_string(),
            ],
        })
    })
}

/// **M1's check.** Both sink paths are in the per-kind field, or the worker is
/// not attempted.
fn the_field_names_both_sinks(
    capture: &CaptureRecord,
    field: Option<&[u8]>,
) -> Result<(), String> {
    let carried = field.map(String::from_utf8_lossy).unwrap_or_default();
    for sink in [&capture.ordinary_sink, &capture.complaints_sink] {
        if !carried.contains(sink.as_str()) {
            return Err(format!(
                "the sink at {sink} is not named by the operation record's per-kind field, and \
                 M1 records both paths there before the worker is started"
            ));
        }
    }
    Ok(())
}

/// A refusal from the record store, in words a reader of a complaints stream can
/// act on.
///
/// **The store's own words are carried where it gave any** — what a mount or a
/// filesystem said is not this file's to paraphrase.
///
/// **It is public because the answer to a hold refuses the same way.** U9's
/// answer writes a command record before it acts on the hold, and a store that
/// refused for any reason but *already recorded* has to end that answer with the
/// store's own words on the complaints stream — the same shape and the same
/// words as the start above, taken from here rather than written a second time.
pub fn what_the_store_said(why: &Refused) -> String {
    match why {
        Refused::TheStore(said) => said.clone(),
        Refused::ThatIdentityIsAlreadyRecorded => {
            "that identity is already recorded, and `Server protocol and transfers` C4 makes a \
             request repeating one never act again"
                .to_string()
        }
        Refused::ItAlreadyCarriesARecordedOutcome => {
            "that record already carries a recorded outcome".to_string()
        }
        Refused::ItIsClosedAndTakesNoFurtherDisposition => {
            "that record is closed and takes no further disposition".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use smoozy_state::identities::IdentityKind;
    use smoozy_state::machine_operations::WhatTheMachineHoldsUnderAnIdentity;

    use super::*;
    use crate::acting::capture::TheSinksOnTheMachine;
    use crate::acting::execution_group::{
        ThePairAsTheKernelReportsIt, TheWaitTheSupervisorPerforms,
    };
    use crate::acting::operation_records::{
        TheOperationRecordsOnTheMachine, THE_AREA_THE_TWO_RECORDS_SIT_IN,
    };
    use crate::acting::workers::{HowAWorkerOfThisKindIsInvoked, TheWorkerStartOnTheMachine};
    use crate::arbitration::WhatTheRefusalCarries;
    use crate::capture::CaptureCeiling;
    use crate::execution_group::{Validation, ValidatesTheRecordedPair};

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("randomness")
    }

    /// A package area that goes when the value does.
    struct AnArea {
        at: std::path::PathBuf,
    }

    impl AnArea {
        fn made() -> AnArea {
            let at = std::env::temp_dir().join(format!(
                "smoozy-runtime-start-one-{}",
                an_identity(IdentityKind::Operation).as_text()
            ));
            std::fs::create_dir_all(&at).expect("the area is made");
            AnArea { at }
        }

        fn records(&self) -> TheOperationRecordsOnTheMachine {
            TheOperationRecordsOnTheMachine::in_the_area(
                &self.at.join(THE_AREA_THE_TWO_RECORDS_SIT_IN),
            )
            .expect("the store is made")
        }
    }

    impl Drop for AnArea {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.at).ok();
        }
    }

    /// Whether the kernel of the machine this test runs on answers for a
    /// process, through the reader the program itself uses.
    ///
    /// **Every rented machine does, and the desk does too since 2026-10-24**
    /// (step 12's piece SC0b, the desk's own route in
    /// `crate::acting::execution_group`). A kernel that records nothing is
    /// posed by pointing the reader at an empty place, which is
    /// [`a_start_whose_kernel_records_nothing`].
    fn the_kernel_here_publishes_records() -> bool {
        matches!(
            ThePairAsTheKernelReportsIt::the_rented_machine().what_is_at(std::process::id()),
            Ok(Some(_))
        )
    }

    /// **A real start whose kernel records nothing**: the reader is the real
    /// one, pointed at an empty place, so the start meets exactly what it met
    /// at the desk before the desk had a route of its own.
    fn a_start_whose_kernel_records_nothing(
        wait: Arc<TheWaitTheSupervisorPerforms>,
        program: &str,
        arguments: Vec<String>,
        nothing: &std::path::Path,
    ) -> TheWorkerStartOnTheMachine {
        let mut start = a_real_start(wait, program, arguments);
        start.kernel = ThePairAsTheKernelReportsIt {
            where_the_kernel_publishes: nothing.to_path_buf(),
        };
        start
    }

    fn a_real_start(
        wait: Arc<TheWaitTheSupervisorPerforms>,
        program: &str,
        arguments: Vec<String>,
    ) -> TheWorkerStartOnTheMachine {
        let program = program.to_string();
        TheWorkerStartOnTheMachine {
            kernel: ThePairAsTheKernelReportsIt::the_rented_machine(),
            wait,
            how_it_is_invoked: Box::new(move |_| {
                Some(HowAWorkerOfThisKindIsInvoked {
                    program: program.clone(),
                    arguments: arguments.clone(),
                })
            }),
        }
    }

    fn what_is_started(kind: WorkerKind) -> WhatIsStarted {
        WhatIsStarted {
            operation: an_identity(IdentityKind::Operation),
            kind,
            subject: an_identity(IdentityKind::Job),
            under_run: an_identity(IdentityKind::Run),
            capture: None,
            the_kinds_own_field: None,
        }
    }

    /// **The whole unit, with a real child**: admitted, the record written, the
    /// worker started, the pair recorded, and the pair in the record reading
    /// back as the one the start answered with.
    ///
    /// **It runs whole only where the kernel publishes per-process records** —
    /// every rented machine, and the Linux runner. The desk's own half is the
    /// test below it.
    #[test]
    fn a_worker_is_admitted_started_and_its_pair_is_in_its_record() {
        if !the_kernel_here_publishes_records() {
            return;
        }
        let area = AnArea::made();
        let mut records = area.records();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_real_start(
            wait.clone(),
            "/bin/sh",
            vec!["-c".into(), "exit 0".into()],
        );
        let mut point = TheAdmissionPoint::holding_nothing();
        let what = what_is_started(WorkerKind::Render);

        let produced = start_one_worker(
            &what,
            &mut point,
            None,
            &mut records,
            &TheSinksOnTheMachine,
            &start,
            wait.as_ref(),
        );
        let WhatTheStartOfAWorkerProduces::ItStarted(pair) = produced else {
            panic!("the worker did not start: {produced:?}");
        };
        assert_eq!(point.how_many_it_holds(), 1);

        let held = records
            .what_is_held_under(&what.operation)
            .expect("the store answers");
        let WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record) = held else {
            panic!("no operation record was written");
        };
        let group = record.execution_group.expect("X1's pair is in the record");
        assert_eq!(
            RecordedPair::recorded_in(&group.as_that_document_recorded_it),
            Some(pair)
        );
        assert_eq!(group.started_under_run, what.under_run);

        // **The pair validates while the worker has not been waited on, and the
        // wait is what ends it** — X9a's own division, and the reason the wait
        // exists at all.
        let kernel = ThePairAsTheKernelReportsIt::the_rented_machine();
        let ended = ExecutionGroup {
            kind: WorkerKind::Render,
            leader: pair,
        };
        let mut taken = None;
        for _ in 0..500 {
            taken = wait.the_ending(&ended).expect("the wait is taken");
            if taken.is_some() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert_eq!(taken, Some(0));
        assert_eq!(
            pair.validates_against(kernel.what_is_at(pair.identifier).expect("the kernel answers")),
            Validation::Gone,
            "X9: a leader whose pair no longer validates has ended"
        );
    }

    /// **A real child is started and reaped where the kernel records nothing
    /// for it** — and the unit says so rather than inventing a pair.
    ///
    /// **The record is written before the start whatever happens next**, which
    /// is W3, so it is there afterwards and carries no group. *(Posed with an
    /// empty place from 2026-10-24, when the desk's kernel began to answer; it
    /// ran only at the desk before, and now runs everywhere.)*
    #[test]
    fn where_the_kernel_records_nothing_the_child_still_ran_and_no_pair_is_invented() {
        let area = AnArea::made();
        let mut records = area.records();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let nothing = area.at.join("a-kernel-that-records-nothing");
        std::fs::create_dir_all(&nothing).expect("the empty place is made");
        let start = a_start_whose_kernel_records_nothing(
            wait.clone(),
            "/bin/sh",
            vec!["-c".into(), "exit 7".into()],
            &nothing,
        );
        let mut point = TheAdmissionPoint::holding_nothing();
        let what = what_is_started(WorkerKind::Render);

        let produced = start_one_worker(
            &what,
            &mut point,
            None,
            &mut records,
            &TheSinksOnTheMachine,
            &start,
            wait.as_ref(),
        );
        // **The child ran, so this is not *not attempted*.** It said so until
        // 2026-09-12, and a caller told that meant a render running on a paid
        // machine read as one that never started.
        assert!(
            matches!(
                produced,
                WhatTheStartOfAWorkerProduces::ItStartedAndItsGroupCouldNotBeRecorded(None, _)
            ),
            "{produced:?}"
        );
        // **The child is real and is in the wait**, so its ending is taken
        // rather than left on a machine the user is paying for.
        assert_eq!(wait.how_many_it_holds(), 1);
        // **And it is admitted**, so A5 and A5a decide against a machine that
        // really is holding this worker's resources.
        assert_eq!(point.how_many_it_holds(), 1);
        assert!(matches!(
            records.what_is_held_under(&what.operation),
            Ok(WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(_))
        ));
    }

    /// **A11: while the setup-in-progress state stands nothing is admitted**,
    /// and **nothing is written and nothing is started** — the refusal comes
    /// before the record.
    #[test]
    fn nothing_is_written_or_started_for_an_operation_the_admission_point_refuses() {
        let area = AnArea::made();
        let mut records = area.records();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_real_start(wait.clone(), "/bin/sh", vec!["-c".into(), "exit 0".into()]);
        let mut point = TheAdmissionPoint::holding_nothing();
        let what = what_is_started(WorkerKind::Render);
        let standing = SetupInProgress {
            the_run_it_names: an_identity(IdentityKind::Operation),
        };

        let produced = start_one_worker(
            &what,
            &mut point,
            Some(&standing),
            &mut records,
            &TheSinksOnTheMachine,
            &start,
            wait.as_ref(),
        );
        assert_eq!(
            produced,
            WhatTheStartOfAWorkerProduces::ItWasNotAdmitted(Admission::Refused(
                WhatTheRefusalCarries::TheSetupRunTheStandingStateNames(
                    standing.the_run_it_names.clone()
                )
            ))
        );
        assert_eq!(wait.how_many_it_holds(), 0, "nothing was started");
        assert_eq!(point.how_many_it_holds(), 0);
        assert_eq!(
            records.what_is_held_under(&what.operation),
            Ok(WhatTheMachineHoldsUnderAnIdentity::Nothing),
            "nothing was written"
        );
    }

    /// **A11a's one exception, through the whole unit**: the worker start of the
    /// setup run the standing state itself names is admitted while it stands.
    #[test]
    fn the_run_the_standing_state_names_is_the_one_thing_admitted_under_it() {
        let area = AnArea::made();
        let mut records = area.records();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_real_start(wait.clone(), "/bin/sh", vec!["-c".into(), "exit 0".into()]);
        let mut point = TheAdmissionPoint::holding_nothing();
        let what = what_is_started(WorkerKind::Setup);
        let standing = SetupInProgress {
            the_run_it_names: what.operation.clone(),
        };

        let produced = start_one_worker(
            &what,
            &mut point,
            Some(&standing),
            &mut records,
            &TheSinksOnTheMachine,
            &start,
            wait.as_ref(),
        );
        assert!(
            !matches!(
                produced,
                WhatTheStartOfAWorkerProduces::ItWasNotAdmitted(_)
            ),
            "{produced:?}"
        );
        assert_eq!(wait.how_many_it_holds(), 1, "a real child was started");
    }

    /// **M1: a capture record whose sinks the per-kind field does not name stops
    /// the start**, and the worker is not attempted — §14's own wording.
    #[test]
    fn a_capture_record_the_per_kind_field_does_not_name_stops_the_start() {
        let area = AnArea::made();
        let mut records = area.records();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_real_start(wait.clone(), "/bin/sh", vec!["-c".into(), "exit 0".into()]);
        let mut point = TheAdmissionPoint::holding_nothing();
        let mut what = what_is_started(WorkerKind::Render);
        what.capture = Some(CaptureRecord {
            job: what.subject.clone(),
            attempt: 1,
            ordinary_sink: area.at.join("ordinary").display().to_string(),
            complaints_sink: area.at.join("complaints").display().to_string(),
            ceiling: CaptureCeiling::from(0),
        });

        let produced = start_one_worker(
            &what,
            &mut point,
            None,
            &mut records,
            &TheSinksOnTheMachine,
            &start,
            wait.as_ref(),
        );
        let WhatTheStartOfAWorkerProduces::TheWorkerWasNotAttempted(why) = produced else {
            panic!("a start with no sink paths in the field is refused: {produced:?}");
        };
        assert!(why.contains("per-kind field"), "{why}");
        assert_eq!(wait.how_many_it_holds(), 0, "nothing was started");
        assert_eq!(
            records.what_is_held_under(&what.operation),
            Ok(WhatTheMachineHoldsUnderAnIdentity::Nothing),
            "the refusal comes before the record"
        );
    }

    /// **The sinks are made before the worker is started**, and the per-kind
    /// field naming both is what lets the start proceed.
    ///
    /// **The proof is inside the invocation**, which the start asks for at the
    /// moment it starts the child: both sinks are read there, and a start that
    /// made them afterwards would fail inside that closure. **So this test does
    /// not turn on whether the kernel here publishes per-process records** —
    /// what it asserts is that a real child was started and that the two files
    /// were on the disk when it was.
    #[test]
    fn the_two_sinks_are_there_before_the_worker_is_started() {
        let area = AnArea::made();
        let mut records = area.records();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let mut point = TheAdmissionPoint::holding_nothing();
        let mut what = what_is_started(WorkerKind::Render);
        let ordinary = area.at.join("ordinary").display().to_string();
        let complaints = area.at.join("complaints").display().to_string();
        what.capture = Some(CaptureRecord {
            job: what.subject.clone(),
            attempt: 1,
            ordinary_sink: ordinary.clone(),
            complaints_sink: complaints.clone(),
            ceiling: CaptureCeiling::from(0),
        });
        what.the_kinds_own_field = Some(format!("{ordinary}\n{complaints}").into_bytes());

        // **The start reads the two sinks to prove they were there first.** A
        // worker started before its sinks were made is M13's *never
        // established*, and §14 forbids it.
        let both = (ordinary.clone(), complaints.clone());
        let start = TheWorkerStartOnTheMachine {
            kernel: ThePairAsTheKernelReportsIt::the_rented_machine(),
            wait: wait.clone(),
            how_it_is_invoked: Box::new(move |_| {
                assert!(std::path::Path::new(&both.0).exists());
                assert!(std::path::Path::new(&both.1).exists());
                Some(HowAWorkerOfThisKindIsInvoked {
                    program: "/bin/sh".into(),
                    arguments: vec!["-c".into(), "exit 0".into()],
                })
            }),
        };

        start_one_worker(
            &what,
            &mut point,
            None,
            &mut records,
            &TheSinksOnTheMachine,
            &start,
            wait.as_ref(),
        );
        assert_eq!(wait.how_many_it_holds(), 1, "a real child was started");
        assert!(std::path::Path::new(&ordinary).exists());
        assert!(std::path::Path::new(&complaints).exists());
    }

    /// **A repeat of an identity already recorded never acts again**, which is
    /// `Server protocol and transfers` C4 kept at the store and met here.
    #[test]
    fn a_second_start_under_one_identity_is_not_attempted() {
        let area = AnArea::made();
        let mut records = area.records();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_real_start(wait.clone(), "/bin/sh", vec!["-c".into(), "exit 0".into()]);
        let mut point = TheAdmissionPoint::holding_nothing();
        let what = what_is_started(WorkerKind::Preview);

        let first = start_one_worker(
            &what,
            &mut point,
            None,
            &mut records,
            &TheSinksOnTheMachine,
            &start,
            wait.as_ref(),
        );
        assert!(
            !matches!(first, WhatTheStartOfAWorkerProduces::ItWasNotAdmitted(_)),
            "{first:?}"
        );
        let started_once = wait.how_many_it_holds();

        let again = start_one_worker(
            &what,
            &mut point,
            None,
            &mut records,
            &TheSinksOnTheMachine,
            &start,
            wait.as_ref(),
        );
        let WhatTheStartOfAWorkerProduces::TheWorkerWasNotAttempted(why) = again else {
            panic!("a repeated identity never acts again: {again:?}");
        };
        assert!(why.contains("already recorded"), "{why}");
        assert_eq!(
            wait.how_many_it_holds(),
            started_once,
            "no second child was started"
        );
    }

    /// **X9a's wait is what takes an operation out of the admission point**, and
    /// nothing else does.
    #[test]
    fn an_operation_whose_group_has_ended_leaves_the_admission_point() {
        if !the_kernel_here_publishes_records() {
            return;
        }
        let area = AnArea::made();
        let mut records = area.records();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_real_start(wait.clone(), "/bin/sh", vec!["-c".into(), "exit 0".into()]);
        let mut point = TheAdmissionPoint::holding_nothing();

        let what = what_is_started(WorkerKind::Render);
        let produced = start_one_worker(
            &what,
            &mut point,
            None,
            &mut records,
            &TheSinksOnTheMachine,
            &start,
            wait.as_ref(),
        );
        assert!(
            matches!(produced, WhatTheStartOfAWorkerProduces::ItStarted(_)),
            "{produced:?}"
        );
        assert_eq!(point.how_many_it_holds(), 1);

        let mut ended = Vec::new();
        for _ in 0..500 {
            ended = point.take_the_endings(wait.as_ref());
            if !ended.is_empty() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert_eq!(ended, vec![(what.operation.clone(), 0)]);
        assert_eq!(point.how_many_it_holds(), 0);
    }

    /// **A worker is invoked as the program's own path with its kind's word
    /// after it**, and the role reader takes it back as that kind.
    #[test]
    fn a_worker_is_invoked_as_the_program_with_its_kind_after_it() {
        let composed = how_the_program_invokes_a_worker("/ephemeral/smoozy/package");
        for kind in WorkerKind::ALL {
            let invocation = composed(&StartedWorker {
                kind,
                operation: an_identity(IdentityKind::Operation),
                under_run: an_identity(IdentityKind::Run),
            })
            .expect("every kind is invoked");
            assert_eq!(
                invocation.program, "/ephemeral/smoozy/package/smoozy-server",
                "G7: the pointer, and never the resolved executable"
            );
            assert_eq!(
                crate::the_program::the_role_selected_by(&invocation.arguments),
                Ok(crate::the_program::Role::AnOperationWorker(kind))
            );
        }
    }

    /// **The pair reads back whole or not at all**, so nothing ever validates on
    /// the identifier alone.
    #[test]
    fn a_pair_that_does_not_read_back_whole_is_not_half_a_pair() {
        let pair = RecordedPair {
            identifier: 1493,
            start_time: 19186,
        };
        assert_eq!(
            RecordedPair::recorded_in(&pair.as_this_document_records_it()),
            Some(pair)
        );
        for broken in ["1493", "", "1493 ", "1493 nineteen", "one 19186"] {
            assert_eq!(RecordedPair::recorded_in(broken), None, "{broken}");
        }
    }

    // --- §5.3's own start: the one role worker ---

    /// **W7: the role worker has no operation record**, and this is what that
    /// costs a reader to check — the store is empty afterwards, and the child
    /// really ran.
    ///
    /// **W9: a change of the desired state restarts nothing but the role
    /// worker.** Nothing here touches the supervisor, the machine, or any
    /// per-operation worker, and the admission point holds exactly what it held
    /// plus this one.
    ///
    /// **A real child is started and reaped either way**, and the two halves are
    /// split exactly as [`start_one_worker`]'s own tests are: where the kernel
    /// answers for a process the pair is composed and the operation is
    /// admitted; where it does not, **the child still ran, it is in the wait,
    /// and no pair is invented.** *(The desk answers from 2026-10-24, so the
    /// first half is the one run at the desk too.)*
    #[test]
    fn the_role_worker_starts_with_no_operation_record_of_its_own() {
        let area = AnArea::made();
        let records = area.records();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_real_start(wait.clone(), "/bin/sh", vec!["-c".into(), "exit 0".into()]);
        let mut point = TheAdmissionPoint::holding_nothing();
        let asked_under = an_identity(IdentityKind::Operation);
        let under_run = an_identity(IdentityKind::Run);

        let produced = start_the_role_worker(
            &asked_under,
            &under_run,
            &mut point,
            None,
            &start,
            wait.as_ref(),
        );
        if the_kernel_here_publishes_records() {
            assert!(
                matches!(produced, WhatTheStartOfAWorkerProduces::ItStarted(_)),
                "the role worker did not start: {produced:?}"
            );
            assert_eq!(point.how_many_it_holds(), 1);
        } else {
            let WhatTheStartOfAWorkerProduces::ItStartedAndItsGroupCouldNotBeRecorded(
                pair,
                why,
            ) = &produced
            else {
                panic!("this kernel records nothing, so no pair can be composed: {produced:?}");
            };
            assert_eq!(*pair, None, "no pair is invented where none could be taken");
            assert!(why.contains("the kernel recorded nothing for it"), "{why}");
            // **The bot is running and the supervisor is told so**, which is
            // what stops W10's next look starting a second one.
            assert_eq!(point.how_many_it_holds(), 1);
        }
        assert_eq!(
            records
                .what_is_held_under(&asked_under)
                .expect("the store answers"),
            WhatTheMachineHoldsUnderAnIdentity::Nothing,
            "W7: the role worker takes no operation record"
        );
        assert_eq!(
            wait.how_many_it_holds(),
            1,
            "X9a: a real child ran and it is in the wait"
        );
    }

    /// **A11a: not a role worker's work.** While the setup-in-progress state
    /// stands, the role worker is refused and the refusal names the run the
    /// state holds — and no child is started.
    #[test]
    fn the_role_worker_is_not_admitted_while_a_setup_is_in_progress() {
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_real_start(
            wait.clone(),
            "/this/is/not/a/program/on/any/machine",
            Vec::new(),
        );
        let mut point = TheAdmissionPoint::holding_nothing();
        let the_run_it_names = an_identity(IdentityKind::Operation);

        let produced = start_the_role_worker(
            &an_identity(IdentityKind::Operation),
            &an_identity(IdentityKind::Run),
            &mut point,
            Some(&SetupInProgress {
                the_run_it_names: the_run_it_names.clone(),
            }),
            &start,
            wait.as_ref(),
        );
        assert_eq!(
            produced,
            WhatTheStartOfAWorkerProduces::ItWasNotAdmitted(Admission::Refused(
                WhatTheRefusalCarries::TheSetupRunTheStandingStateNames(the_run_it_names)
            ))
        );
        assert_eq!(wait.how_many_it_holds(), 0, "nothing was started");
    }
}

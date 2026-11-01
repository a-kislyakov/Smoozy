//! §5 — worker classes and their lifecycles.
//!
//! **W1. There are six per-operation kinds and one role kind, and the set is
//! closed by the shape rather than by this document.**
//!
//! **W5. No worker has a restart policy of its own.** A worker that exits is an
//! observation; what happens next comes from the operation record's desired state
//! at the next reconciliation, and from nothing else.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` — the render kind's declarations and its
//! termination contract. `Render Settings` §10. `Dependency Upload & Remap` §9.
//! `Server Setup` §11 and its termination-contract paragraph. `Server protocol
//! and transfers` C22. `Job files and outputs` B24. `Queue & History` §16.
//! `Telegram` §12. `State, identity and recovery` §1.2, §4, §5.1, §6.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.

use smoozy_state::identities::Identity;

/// **W1's seven kinds. The set is closed by the shape.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkerKind {
    /// Started for one job attempt. Declarations `Rendering`'s; termination the
    /// interrupt to the recorded render process (§6.3).
    Render,
    /// One reading of a `.blend`. Declarations `Render Settings` §10 W1, **and
    /// `Dependency Upload & Remap` §9 for its check reading, whose declarations
    /// agree value for value**; termination `Render Settings` §10 W2.
    FileInspection,
    /// One setup run. Declarations and termination `Server Setup`'s — at, or
    /// recovery from, a recorded safe boundary.
    Setup,
    /// One transfer arrangement on the machine. Declarations and termination
    /// `Server protocol and transfers`' — **staging and cleanup rather than a
    /// signal** (that document's C22).
    TransferSupport,
    /// One job's still copies. Declarations `Job files and outputs` B24 — **low
    /// priority, cancellable, never blocking a render**; termination `Queue &
    /// History` §16, the worker outliving its group.
    Preview,
    /// One job's moving picture and contact sheet. Declarations `Telegram` §12
    /// M5; termination that document's M6.
    Media,
    /// **The one role kind** (§5.3).
    Telegram,
}

impl WorkerKind {
    /// The whole closed set.
    pub const ALL: [WorkerKind; 7] = [
        WorkerKind::Render,
        WorkerKind::FileInspection,
        WorkerKind::Setup,
        WorkerKind::TransferSupport,
        WorkerKind::Preview,
        WorkerKind::Media,
        WorkerKind::Telegram,
    ];

    /// The six per-operation kinds. **The role kind takes none of the four
    /// operation outcomes and has no operation record** (W7).
    pub const PER_OPERATION: [WorkerKind; 6] = [
        WorkerKind::Render,
        WorkerKind::FileInspection,
        WorkerKind::Setup,
        WorkerKind::TransferSupport,
        WorkerKind::Preview,
        WorkerKind::Media,
    ];

    /// **Which document owns this kind's declarations**, named rather than
    /// inferred. W1's table is the authority and this carries it.
    pub fn declarations_owned_by(self) -> &'static [&'static str] {
        match self {
            WorkerKind::Render => &["Rendering"],
            WorkerKind::FileInspection => &["Render Settings", "Dependency Upload & Remap"],
            WorkerKind::Setup => &["Server Setup"],
            WorkerKind::TransferSupport => &["Server protocol and transfers"],
            WorkerKind::Preview => &["Job files and outputs"],
            WorkerKind::Media => &["Telegram"],
            WorkerKind::Telegram => &["Telegram"],
        }
    }
}

/// **W2. A kind with no termination contract may be started and may not be
/// stopped by a signal.**
///
/// It is ended by the ordinary route its own work provides — a reading that
/// completes, a derivative that is produced — and where the arbitration mechanism
/// needs it cancelled, **cancellation is refused and the higher-priority work
/// waits or is admitted alongside** (§7.2).
///
/// **No default signal is invented for a kind whose document has not named one**,
/// because no signal or cleanup contract in this project is universal across
/// worker kinds and **inventing one is how the wrong process gets signalled.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherASignalMayBeSent {
    TheKindsOwnContractNamesOne,
    /// **Cancellation is refused**; nothing is signalled.
    NoContractSoNoSignalIsInvented,
}

/// **W2.**
pub fn whether_a_signal_may_be_sent(
    the_kinds_document_has_named_a_contract: bool,
) -> WhetherASignalMayBeSent {
    if the_kinds_document_has_named_a_contract {
        WhetherASignalMayBeSent::TheKindsOwnContractNamesOne
    } else {
        WhetherASignalMayBeSent::NoContractSoNoSignalIsInvented
    }
}

/// **W3. Every worker is started against one durable operation record, written
/// before the worker is started** (`State, identity and recovery` §5.1).
///
/// **There is no worker with no operation record and no operation record whose
/// worker was started before it was written.** The type carries the operation
/// identity for that reason: a started worker cannot be described without one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartedWorker {
    pub kind: WorkerKind,
    /// **W3.** The operation this worker was started against.
    pub operation: Identity,
    /// **S2's run identifier**, which the worker's report carries (W12).
    pub under_run: Identity,
}

/// **W4. The four outcomes and the four dispositions are `State, identity and
/// recovery`'s and are not restated.**
///
/// **What this document adds is that a per-kind field of the operation record is
/// the kind's document's** — which is where §11.3's capture record lives for a
/// render and where a setup run's safe-boundary marker lives for a setup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerKindField {
    pub kind: WorkerKind,
    /// Opaque here. **The owning document reads it; this one carries it.**
    pub contents: Vec<u8>,
}

/// **W5. No worker has a restart policy of its own.**
///
/// **A worker that exits is an observation**; what happens next comes from the
/// operation record's desired state at the next reconciliation, and from nothing
/// else. The value is `()` because the clause is an absence: nothing in this
/// crate restarts a worker, and no restart policy is expressible for one.
pub const NO_WORKER_HAS_A_RESTART_POLICY_OF_ITS_OWN: () = ();

/// **W6. A worker that outlives its supervisor is the case the *resumed* branch
/// exists for.**
///
/// Observed: a detached program outlived the connection and the shell that
/// started it, *"Its parent is **1**"*. **A worker is started so that it survives
/// the supervisor's own exit, because a render that died with a crashed
/// supervisor would lose work the machine was paid for.**
pub const A_WORKER_IS_STARTED_TO_SURVIVE_THE_SUPERVISORS_OWN_EXIT: bool = true;

/// **W12. What makes that branch work rather than merely name it is that the
/// worker's report survives the restart, and the route is the endpoint's recorded
/// path.**
///
/// `State, identity and recovery` §6 writes *resumed* *"before the supervisor
/// re-attaches"*, and re-attaching is this.
///
/// **Without it the counts, the exit and the end of stream have nowhere to
/// arrive**: M6 gives the worker one channel to the supervisor, Z1 and Z4 destroy
/// the endpoint object with the run that made it, S1 forbids the worker writing
/// the outcome itself and §1.3 rule 5 forbids the next supervisor reading it off
/// the disk — so **a finished render would leave an operation record with no
/// recorded outcome, blocking every package commit under G21 and re-running on a
/// paid machine at the next reconciliation.**
///
/// **S1 is untouched**: the worker reports and the supervisor writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkersReport {
    /// The group this report is about.
    pub operation: Identity,
    /// **The run identifier the worker was started under** (S2).
    pub under_run: Identity,
}

/// Whether a supervisor accepts a worker's report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheReportIsAccepted {
    Accepted,
    /// **A stale one is refused.**
    RefusedStale,
}

/// **W12. A supervisor that has written *resumed* accepts a report from a group
/// recorded under an earlier run identifier while refusing a stale one.**
///
/// The *resumed* disposition is what separates the two: without it, a report
/// carrying a run identifier that is not this run's is stale.
pub fn whether_the_report_is_accepted(
    report: &WorkersReport,
    this_supervisors_run: &Identity,
    resumed_was_written_for_this_group: bool,
) -> WhetherTheReportIsAccepted {
    if &report.under_run == this_supervisors_run {
        return WhetherTheReportIsAccepted::Accepted;
    }
    if resumed_was_written_for_this_group {
        WhetherTheReportIsAccepted::Accepted
    } else {
        WhetherTheReportIsAccepted::RefusedStale
    }
}

/// **W12's route: the report is re-attempted against the endpoint at Z2's
/// recorded path — a path that does not vary with the package version and
/// outlives any one supervisor run, though the object at it does not (Z1) — until
/// it is accepted or the worker's own work is over.**
pub trait ReportsToTheSupervisor {
    /// One attempt. **The re-attempt loop is the acting half's**; what is here is
    /// that there is one channel, one path, and no second route.
    fn attempt(&self, report: &WorkersReport) -> Result<WhetherTheReportIsAccepted, String>;
}

/// **What one start answers, and the three are different things to whoever
/// meets them.**
///
/// **This was a `Result` until 2026-09-12 and the middle member is why it is
/// not.** A child that ran and whose kernel record could not then be read came
/// back as an error, and the caller could only report it as *not attempted* —
/// so a render running on a paid machine read as a render that never started,
/// and the next reconciliation pass would start a second one beside it.
/// **§14's *not attempted* is about a start that did not happen**, and this one
/// did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatAStartAnswers {
    /// **X1's pair, composed at the moment of the start.**
    ItStarted(crate::execution_group::RecordedPair),
    /// **The child is running and no pair could be taken for it**, so there is
    /// nothing to record and **nothing is invented** — `State, identity and
    /// recovery` §2.1 rule 5 forbids a bare identifier as a runtime identity and
    /// X2 shows the cost of validating on one. The identifier is carried so the
    /// caller can say which process is alive and so the wait this start already
    /// put it into can be reached; it is never a recorded pair.
    ItStartedAndNoPairCouldBeTaken {
        identifier: u32,
        why: String,
    },
    /// **Nothing was started**, naming what stopped it — §14's own wording.
    ItWasNotAttempted(String),
}

/// Starting a worker, behind a trait.
///
/// **W3's ordering is the trait's shape rather than a note**: the operation record
/// is the argument, so there is no way to ask for a worker without one. **§14
/// declares the whole start one unit** — where the record or the sinks cannot be
/// made, the worker is *not attempted*, naming what stopped it.
pub trait StartsWorkers {
    /// **The recorded execution group is composed at the moment of the start**
    /// (X1), which is why this answers with it rather than with a bare identifier.
    fn start(&self, worker: &StartedWorker) -> WhatAStartAnswers;
}

/// **§5.3 — the one role worker.**
///
/// **W7. The Telegram worker is the only role worker, and it is started,
/// restarted and stopped from a durable enabled-or-disabled desired state held at
/// machine level** (`State, identity and recovery` §1.2). **It takes none of the
/// four operation outcomes and has no operation record.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RoleDesiredState {
    Enabled,
    Disabled,
}

/// **W7's store, behind a trait**, exactly as P1's setup-in-progress state is.
///
/// **One durable value at machine level**, in the package area, and
/// `State, identity and recovery` §1.2 already names it in the machine-level
/// desired states row with this document as its owner. **The supervisor is its
/// only writer** (that document's §1.3 rule 1) and `Server Setup` reaches it
/// through the command path (W8) rather than through this trait.
///
/// **A machine nothing has written it on reads as *disabled*, and that is a
/// clause rather than a convenience.** W8 makes the second entry point set the
/// state; a machine that entry point has never reached has not been given the
/// role, so a supervisor that read an absent value as *enabled* would start a
/// worker nobody asked for, on every machine Smoozy ever placed a package on.
pub trait KeepsTheRolesDesiredState {
    /// **W8's write**, durable before the supervisor acts on it.
    fn write(&self, desired: RoleDesiredState) -> Result<(), String>;

    /// What the machine holds. **An absent value is *disabled***.
    fn read(&self) -> Result<RoleDesiredState, String>;
}

/// What a change of the role's desired state reaches.
///
/// **W9. A change of that desired state restarts nothing but the role worker.**
/// The supervisor is not restarted, the machine is not restarted, and no
/// per-operation worker is disturbed — **which is Row 91's second promise, that
/// *"a render already going on that machine is left alone"*.** The mechanism is
/// that the role worker is a child of the supervisor like any other and its start
/// needs nothing the machine is not already running.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatARoleStateChangeReaches {
    pub the_role_worker: bool,
    pub the_supervisor: bool,
    pub the_machine: bool,
    pub any_per_operation_worker: bool,
}

impl WhatARoleStateChangeReaches {
    /// **W9.**
    pub const THE_ROLE_WORKER_AND_NOTHING_ELSE: WhatARoleStateChangeReaches =
        WhatARoleStateChangeReaches {
            the_role_worker: true,
            the_supervisor: false,
            the_machine: false,
            any_per_operation_worker: false,
        };
}

/// **W8. `Server Setup` sets that desired state through the same command path as
/// every other command, and never starts or stops the process.**
///
/// That is the shape's rule and it is the mechanism **Row 91** needs: the second
/// entry point puts the role's material on a machine prepared earlier and sets
/// the state, **and the supervisor does the rest.**
pub const SERVER_SETUP_SETS_THE_STATE_AND_NEVER_THE_PROCESS: bool = true;

/// **W10. The supervisor restarts it for as long as it remains enabled, records
/// and reports repeated failure rather than silently giving up, and never
/// resurrects it after it is disabled.** That is the shape's own sentence and it
/// is taken whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheSupervisorDoesWithTheRoleWorker {
    StartItAgain,
    /// **Never resurrected after it is disabled.**
    LeaveItStopped,
}

/// **What *stopped* means for this kind, and it is an act rather than an
/// absence.**
///
/// **W7 says the role worker is started, restarted and stopped from the desired
/// state, and W1's table gives the kind's termination contract to §5.3, which
/// states none — so it is stated by the document that defines the kind.**
/// `Telegram` [Section 19](../../../Preparation/Architecture/Telegram.md) gives
/// all four of X11's things in its own words: the worker's recorded execution
/// group *"is itself alone — it starts no external process"*; **the termination
/// is *"the polite signal delivered to the worker's own process, at any
/// moment"***, because every durable fact it holds was written through the
/// supervisor before the act it records and a death at any instant re-delivers
/// at-least-once work idempotently; *"the cleanup owed is none"*; and the worker
/// outlives every member of its group trivially. **W2 is not reached**: it
/// refuses a signal for a kind whose document has named none, and this kind's
/// document has named one.
///
/// **This was written as an absence until 2026-09-12** — *"nothing signals a
/// running one"* — and it recorded something that was never true: §5.3 states no
/// contract, and W1 sends a reader to the defining document rather than to a
/// gap. What that absence cost was a user switching Telegram off and going on
/// getting progress and endings on their phone, with the machine going on taking
/// instructions from it.
pub const A_DISABLED_ROLE_IS_SIGNALLED_AND_NEVER_RESURRECTED: bool = true;

/// **W10.**
pub fn what_the_supervisor_does_with_the_role_worker(
    desired: RoleDesiredState,
) -> WhatTheSupervisorDoesWithTheRoleWorker {
    match desired {
        RoleDesiredState::Enabled => WhatTheSupervisorDoesWithTheRoleWorker::StartItAgain,
        RoleDesiredState::Disabled => WhatTheSupervisorDoesWithTheRoleWorker::LeaveItStopped,
    }
}

/// **W11. Repeated failure is reported through the event journal**, so a reader
/// learns of it by reading forward from its own cursor rather than by comparing
/// two readings (`State, identity and recovery` §4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatedFailureReported {
    /// How many times it has failed under this run — recorded, so *repeated* is a
    /// fact rather than an impression.
    pub failures_under_this_run: u32,
}

#[cfg(test)]
mod tests {
    use smoozy_state::identities::IdentityKind;

    use super::*;

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("randomness")
    }

    #[test]
    fn the_kinds_are_seven_six_of_them_per_operation_and_one_role() {
        // **W1's set is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so a kind added to the
        // enum has none, the crate stops compiling, and whoever added it is sent
        // to both lists — where the two length assertions this replaced could
        // never have been false, each array's own type having fixed it.
        for kind in WorkerKind::ALL {
            let (where_the_whole_set_puts_it, whether_it_is_per_operation) = match kind {
                WorkerKind::Render => (0, true),
                WorkerKind::FileInspection => (1, true),
                WorkerKind::Setup => (2, true),
                WorkerKind::TransferSupport => (3, true),
                WorkerKind::Preview => (4, true),
                WorkerKind::Media => (5, true),
                WorkerKind::Telegram => (6, false),
            };
            assert_eq!(WorkerKind::ALL[where_the_whole_set_puts_it], kind);
            assert_eq!(
                WorkerKind::PER_OPERATION.contains(&kind),
                whether_it_is_per_operation,
                "{kind:?} is on the wrong side of the per-operation split"
            );
        }
        assert!(!WorkerKind::PER_OPERATION.contains(&WorkerKind::Telegram));
    }

    #[test]
    fn every_kind_names_the_document_that_owns_its_declarations() {
        for kind in WorkerKind::ALL {
            assert!(!kind.declarations_owned_by().is_empty());
        }
        // The one kind with two owners, whose declarations agree value for
        // value.
        assert_eq!(
            WorkerKind::FileInspection.declarations_owned_by(),
            &["Render Settings", "Dependency Upload & Remap"]
        );
    }

    #[test]
    fn a_kind_with_no_contract_is_never_given_an_invented_signal() {
        assert_eq!(
            whether_a_signal_may_be_sent(false),
            WhetherASignalMayBeSent::NoContractSoNoSignalIsInvented
        );
    }

    #[test]
    fn a_report_under_this_run_is_accepted() {
        let run = an_identity(IdentityKind::Run);
        let report = WorkersReport {
            operation: an_identity(IdentityKind::Operation),
            under_run: run.clone(),
        };
        assert_eq!(
            whether_the_report_is_accepted(&report, &run, false),
            WhetherTheReportIsAccepted::Accepted
        );
    }

    #[test]
    fn a_report_from_an_earlier_run_is_accepted_only_where_resumed_was_written() {
        let this_run = an_identity(IdentityKind::Run);
        let report = WorkersReport {
            operation: an_identity(IdentityKind::Operation),
            under_run: an_identity(IdentityKind::Run),
        };
        assert_eq!(
            whether_the_report_is_accepted(&report, &this_run, false),
            WhetherTheReportIsAccepted::RefusedStale
        );
        assert_eq!(
            whether_the_report_is_accepted(&report, &this_run, true),
            WhetherTheReportIsAccepted::Accepted
        );
    }

    #[test]
    fn a_role_state_change_reaches_the_role_worker_and_nothing_else() {
        let reaches = WhatARoleStateChangeReaches::THE_ROLE_WORKER_AND_NOTHING_ELSE;
        assert!(reaches.the_role_worker);
        assert!(!reaches.the_supervisor);
        assert!(!reaches.the_machine);
        assert!(!reaches.any_per_operation_worker);
    }

    #[test]
    fn a_disabled_role_worker_is_never_resurrected() {
        assert_eq!(
            what_the_supervisor_does_with_the_role_worker(RoleDesiredState::Disabled),
            WhatTheSupervisorDoesWithTheRoleWorker::LeaveItStopped
        );
        assert_eq!(
            what_the_supervisor_does_with_the_role_worker(RoleDesiredState::Enabled),
            WhatTheSupervisorDoesWithTheRoleWorker::StartItAgain
        );
    }

    #[test]
    fn a_started_worker_cannot_be_described_without_its_operation_record() {
        let worker = StartedWorker {
            kind: WorkerKind::Render,
            operation: an_identity(IdentityKind::Operation),
            under_run: an_identity(IdentityKind::Run),
        };
        assert_eq!(worker.kind, WorkerKind::Render);
    }

    #[test]
    fn a_per_kind_field_is_carried_and_not_read() {
        let field = PerKindField {
            kind: WorkerKind::Setup,
            contents: b"a safe-boundary marker this document does not read".to_vec(),
        };
        assert_eq!(field.kind, WorkerKind::Setup);
    }

    #[test]
    fn repeated_failure_is_a_recorded_count_rather_than_an_impression() {
        let reported = RepeatedFailureReported {
            failures_under_this_run: 3,
        };
        assert_eq!(reported.failures_under_this_run, 3);
    }
}

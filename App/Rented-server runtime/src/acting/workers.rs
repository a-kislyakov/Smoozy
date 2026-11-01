//! Starting a worker, and recording its execution group at the moment it starts.
//!
//! **[`crate::workers`] holds the rules** — the seven kinds, which document owns
//! each kind's declarations, and that no worker has a restart policy of its own.
//! This module is the start.
//!
//! # What is started, and who says so
//!
//! **What program a worker of a kind is, and what it is handed, is the owning
//! document's and never this one's.** W1's own table names the owner of every
//! kind's declarations, and this module takes the invocation as something handed
//! in: a kind whose document has named none is not started, and nothing is
//! guessed at from the kind.
//!
//! # What the start does not do, each a clause rather than an omission
//!
//! **Nothing here gives a worker a restart policy.** W5: a worker that exits is
//! an observation, and what happens next comes from the operation record's
//! desired state at the next reconciliation and from nothing else. **The
//! observed constraint is what forces it** — the restart policy that recovered
//! work also restarted it two seconds after a deliberate stop.
//!
//! **Nothing here arranges for the worker to outlive the supervisor, because
//! the arrangement is elsewhere on both machine kinds.** W6 requires it — a
//! render that died with a crashed supervisor would lose work the machine was
//! paid for — and K18 is where it is kept on a systemd machine, in the unit's
//! own kill behaviour, *"a requirement of the registration rather than a detail
//! of it"*. On a container S5's keeper starts the supervisor as an ordinary
//! process with no control group of its own to collapse, and K18 says outright
//! that **nothing equivalent is owed there**, the orphaning being the machine's
//! default. **So a start that did something extra here would be a third
//! arrangement beside those two.**
//!
//! **Nothing here writes the operation record or makes the sinks.** §14 declares
//! starting a worker one unit — the worker is unsafe without its operation
//! record and its sinks, so the ordered set is one unit, and **where either
//! fails the worker is not attempted, naming what stopped it**. That ordering is
//! the supervisor's, which holds all three; what this module offers is the third
//! act, and it is reached only after the other two.
//!
//! **Nothing here passes a secret.** `Security and licence` §13's first rule:
//! no secret is ever a process argument or an environment value.

use std::sync::Arc;
use std::time::Duration;

use smoozy_framing::framing::{
    Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};

use crate::acting::execution_group::{ThePairAsTheKernelReportsIt, TheWaitTheSupervisorPerforms};
use crate::execution_group::{RecordedPair, ValidatesTheRecordedPair};
use crate::workers::{
    ReportsToTheSupervisor, StartedWorker, StartsWorkers, WhatAStartAnswers,
    WhetherTheReportIsAccepted, WorkersReport,
};

/// How a worker of one kind is invoked, as the document that owns that kind
/// states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HowAWorkerOfThisKindIsInvoked {
    /// The program, by path.
    pub program: String,
    /// What it is handed, as a list rather than as a line.
    pub arguments: Vec<String>,
}

/// The owning document's answer for one worker about to be started.
pub type SaysHowThisWorkerIsInvoked =
    Box<dyn Fn(&StartedWorker) -> Option<HowAWorkerOfThisKindIsInvoked> + Send + Sync>;

/// The start, performed on the machine.
pub struct TheWorkerStartOnTheMachine {
    /// The reading that records the second half of X1's pair.
    pub kernel: ThePairAsTheKernelReportsIt,
    /// X9a's wait, which every started worker is put into.
    pub wait: Arc<TheWaitTheSupervisorPerforms>,
    /// What the owning document says this worker is.
    pub how_it_is_invoked: SaysHowThisWorkerIsInvoked,
}

impl std::fmt::Debug for TheWorkerStartOnTheMachine {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.debug_struct("TheWorkerStartOnTheMachine")
            .field("kernel", &self.kernel)
            .finish_non_exhaustive()
    }
}

impl StartsWorkers for TheWorkerStartOnTheMachine {
    /// **X1's recording, taken at the moment of the start.**
    ///
    /// The identifier comes from the start itself and the start time from the
    /// kernel, and the pair is what is handed back — never the identifier
    /// alone, which `State, identity and recovery` §2.1 rule 5 forbids as a
    /// runtime identity and X2 shows the cost of.
    ///
    /// **The worker is put into the wait before the pair is answered**, so
    /// there is no moment at which this supervisor has started something it is
    /// not waiting on. That is what X9a's own hole is closed by.
    fn start(&self, worker: &StartedWorker) -> WhatAStartAnswers {
        let Some(invocation) = (self.how_it_is_invoked)(worker) else {
            return WhatAStartAnswers::ItWasNotAttempted(format!(
                "the document that owns the {:?} kind has not said what its worker is; \
                 nothing is guessed at from the kind",
                worker.kind
            ));
        };
        let started = match std::process::Command::new(&invocation.program)
            .args(&invocation.arguments)
            .spawn()
        {
            Ok(started) => started,
            Err(why) => {
                return WhatAStartAnswers::ItWasNotAttempted(format!(
                    "the {:?} worker could not be started: {} could not be run: {why}",
                    worker.kind, invocation.program
                ))
            }
        };
        let identifier = started.id();
        // **The entry is still there to be read**, because this supervisor is
        // holding the started worker and has not waited on it — which is the
        // accidentally protective half of the plan's own Section 16 item 4.
        //
        // **The reading has three answers and every one of them keeps the
        // worker.** A pair, an absent record, and a refusal: the refusal goes
        // through the same wait the absent branch uses, because returning here
        // with the started child still a local would neither end the process nor
        // wait on it — a render worker and the Blender under it would keep
        // running on a machine the user is paying for, in no record and in no
        // wait.
        let at_the_moment_it_started = match self.kernel.what_is_at(identifier) {
            Ok(reading) => reading,
            Err(why) => {
                self.wait.keep(
                    RecordedPair {
                        identifier,
                        start_time: 0,
                    },
                    started,
                );
                return WhatAStartAnswers::ItStartedAndNoPairCouldBeTaken {
                    identifier,
                    why: format!(
                        "the {:?} worker started at {identifier} and the kernel's own record \
                         could not be read, so no execution group could be recorded: {why}",
                        worker.kind
                    ),
                };
            }
        };
        let Some(pair) = at_the_moment_it_started else {
            // Nothing was recorded, so nothing can be validated or stopped
            // later. The worker is not left running unrecorded: it is put into
            // the wait so its ending is taken, and the caller is told **that it
            // started**, which is what it did.
            self.wait.keep(
                RecordedPair {
                    identifier,
                    start_time: 0,
                },
                started,
            );
            return WhatAStartAnswers::ItStartedAndNoPairCouldBeTaken {
                identifier,
                why: format!(
                    "the {:?} worker started at {identifier} and the kernel recorded nothing \
                     for it, so no execution group could be recorded",
                    worker.kind
                ),
            };
        };
        self.wait.keep(pair, started);
        WhatAStartAnswers::ItStarted(pair)
    }
}

/// **W12's request, by name**, and the two things it carries.
///
/// The name is this document's because W12 is, and the field is S2's run
/// identifier — the one the worker was started under, which is what a supervisor
/// that has written *resumed* judges the report by.
pub const THE_WORKERS_REPORT_REQUEST: &str = "the worker's report";

/// The field the report's run identifier rides in.
pub const THE_RUN_THE_WORKER_WAS_STARTED_UNDER: &str = "run";

/// The field the supervisor's answer rides in, and its two values.
pub const WHETHER_THE_REPORT_WAS_ACCEPTED: &str = "accepted";
/// The supervisor accepted the report.
pub const IT_WAS_ACCEPTED: &str = "accepted";
/// **A stale one is refused.**
pub const IT_WAS_REFUSED_AS_STALE: &str = "refused stale";

/// **W12's route: the worker's one channel to the supervisor, which is the
/// machine-local endpoint at Z2's recorded path.**
///
/// **M6 gives the worker one channel to the supervisor** and W12 says what makes
/// the *resumed* branch work rather than merely name it: the report is
/// re-attempted against the endpoint at that path — **a path that does not vary
/// with the package version and outlives any one supervisor run, though the
/// object at it does not** (Z1) — carrying the run identifier the worker was
/// started under.
///
/// **S1 is untouched**: the worker reports and the supervisor writes. Nothing
/// here writes an outcome, and there is no second route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheReportChannelToTheSupervisor {
    pub endpoint: crate::acting::endpoint::TheEndpointAsACaller,
}

impl TheReportChannelToTheSupervisor {
    pub fn at(recorded_endpoint_path: &str, how_long_a_result_may_take: Duration) -> Self {
        TheReportChannelToTheSupervisor {
            endpoint: crate::acting::endpoint::TheEndpointAsACaller::at(
                recorded_endpoint_path,
                how_long_a_result_may_take,
            ),
        }
    }

    /// **W12's re-attempt, which the trait says is the acting half's: the report
    /// is re-attempted until it is accepted or the worker's own work is over.**
    ///
    /// **An attempt that did not land is the one that is tried again.** A
    /// supervisor that answered — accepting the report, or refusing it as stale —
    /// has answered, and asking the same question again would get the same
    /// answer for as long as the worker lived. So what this loop repeats is an
    /// attempt with no answer behind it: nothing holding the endpoint, an object
    /// left by a run that ended, a supervisor that has not bound yet.
    ///
    /// `None` is what the worker's work being over leaves: no answer was ever
    /// got, and none is invented.
    pub fn keep_attempting(
        &self,
        report: &WorkersReport,
        while_the_workers_own_work_is_not_over: impl Fn() -> bool,
        between_two_attempts: Duration,
    ) -> Option<WhetherTheReportIsAccepted> {
        loop {
            if let Ok(answered) = self.attempt(report) {
                return Some(answered);
            }
            if !while_the_workers_own_work_is_not_over() {
                return None;
            }
            std::thread::sleep(between_two_attempts);
        }
    }
}

impl ReportsToTheSupervisor for TheReportChannelToTheSupervisor {
    /// **One attempt.**
    ///
    /// **The correlation identity is the group's own operation identity and not a
    /// second value beside it**, which is `Server protocol and transfers` C1:
    /// the identity is chosen by the initiating side before the request is sent,
    /// it is the idempotency key, and there is not a second value beside it. So a
    /// re-attempt of this report carries the same identity, which is
    /// `Server protocol and transfers` C5 — re-sending is the ordinary way a
    /// lost answer is resolved and it is not a retry.
    ///
    /// **An attempt that got no answer is an error rather than a refusal**, and
    /// the difference matters: *refused stale* is something the supervisor said,
    /// and a report that reached nobody is not a report the supervisor refused.
    /// [`Self::keep_attempting`] is what tries again.
    fn attempt(&self, report: &WorkersReport) -> Result<WhetherTheReportIsAccepted, String> {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            report.operation.clone(),
            THE_WORKERS_REPORT_REQUEST,
        )
        .carrying(Fields::none().and(
            THE_RUN_THE_WORKER_WAS_STARTED_UNDER,
            report.under_run.as_text(),
        ));
        let Some(answered) = self.endpoint.ask(&request)? else {
            return Err(format!(
                "the report was not accepted and was not refused: nothing answered at {}",
                self.endpoint.recorded_path
            ));
        };
        match answered.fields.text(WHETHER_THE_REPORT_WAS_ACCEPTED) {
            Some(IT_WAS_ACCEPTED) => Ok(WhetherTheReportIsAccepted::Accepted),
            Some(IT_WAS_REFUSED_AS_STALE) => Ok(WhetherTheReportIsAccepted::RefusedStale),
            // A result that says neither is not an answer to this question, and
            // it is never read as one of the two.
            _ => Err(format!(
                "the result at {} said neither that the report was accepted nor that it was \
                 refused",
                self.endpoint.recorded_path
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_group::{ExecutionGroup, WaitsOnEveryWorkerItStarts};
    use crate::workers::WorkerKind;
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::path::{Path, PathBuf};

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    /// **W12's route, run over a real endpoint**: the report goes out, the
    /// supervisor judges it by the rule already built, and the answer comes back.
    ///
    /// **A supervisor that has written *resumed* accepts a report from a group
    /// recorded under an earlier run identifier while refusing a stale one** —
    /// which is the case the whole clause exists for: the worker outlived the
    /// supervisor that started it.
    #[test]
    fn a_report_from_an_earlier_run_is_accepted_where_resumed_was_written_and_refused_where_not() {
        use crate::acting::endpoint::as_a_supervisor;
        use crate::workers::whether_the_report_is_accepted;
        use smoozy_framing::framing::Answered;

        let path = as_a_supervisor::a_path();
        let this_run = an_identity(IdentityKind::Run);
        let an_earlier_run = an_identity(IdentityKind::Run);
        let resumed = WorkersReport {
            operation: an_identity(IdentityKind::Operation),
            under_run: an_earlier_run.clone(),
        };
        let never_resumed = WorkersReport {
            operation: an_identity(IdentityKind::Operation),
            under_run: an_earlier_run,
        };

        let supervisor = as_a_supervisor::answering(&path, 2, {
            let this_run = this_run.clone();
            let the_resumed_group = resumed.operation.clone();
            move |request| {
                assert_eq!(request.name, THE_WORKERS_REPORT_REQUEST);
                let carried = Identity::read_back(
                    IdentityKind::Run,
                    request
                        .fields
                        .text(THE_RUN_THE_WORKER_WAS_STARTED_UNDER)
                        .expect("the run identifier rides with the report"),
                )
                .expect("it reads back");
                // **The group is the request's own correlation identity**, which
                // is what lets the supervisor look up whether *resumed* was
                // written for it.
                let read = WorkersReport {
                    operation: request.identity.clone(),
                    under_run: carried,
                };
                let resumed_was_written = read.operation == the_resumed_group;
                let said =
                    match whether_the_report_is_accepted(&read, &this_run, resumed_was_written) {
                        WhetherTheReportIsAccepted::Accepted => IT_WAS_ACCEPTED,
                        WhetherTheReportIsAccepted::RefusedStale => IT_WAS_REFUSED_AS_STALE,
                    };
                Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "2026.09.04")
                    .carrying(Fields::none().and(WHETHER_THE_REPORT_WAS_ACCEPTED, said))
            }
        });

        let channel = TheReportChannelToTheSupervisor::at(&path, Duration::from_millis(2000));
        assert_eq!(
            channel.attempt(&resumed).unwrap(),
            WhetherTheReportIsAccepted::Accepted,
            "the group *resumed* was written for, reported under an earlier run"
        );
        assert_eq!(
            channel.attempt(&never_resumed).unwrap(),
            WhetherTheReportIsAccepted::RefusedStale,
            "the same earlier run, and no *resumed* for this group"
        );
        supervisor.join().unwrap();
        std::fs::remove_file(&path).ok();
    }

    /// **An attempt that reached nobody is not a refusal.** *Refused stale* is
    /// something the supervisor said; a report that reached nothing is not a
    /// report the supervisor refused, and reading it as one would let a worker
    /// give up on a supervisor that was merely restarting.
    #[test]
    fn an_attempt_that_reached_nobody_is_not_read_as_a_refusal() {
        use crate::acting::endpoint::as_a_supervisor;
        let path = as_a_supervisor::a_path();
        let channel = TheReportChannelToTheSupervisor::at(&path, Duration::from_millis(150));
        let report = WorkersReport {
            operation: an_identity(IdentityKind::Operation),
            under_run: an_identity(IdentityKind::Run),
        };
        let why = channel.attempt(&report).expect_err("nothing answered");
        assert!(why.contains("was not accepted and was not refused"));
    }

    /// **W12's own bound: until it is accepted or the worker's own work is
    /// over.** With nothing ever at the path, the loop ends when the work does
    /// and no answer is invented.
    #[test]
    fn the_re_attempt_ends_when_the_workers_own_work_is_over_and_invents_no_answer() {
        use crate::acting::endpoint::as_a_supervisor;
        let path = as_a_supervisor::a_path();
        let channel = TheReportChannelToTheSupervisor::at(&path, Duration::from_millis(50));
        let report = WorkersReport {
            operation: an_identity(IdentityKind::Operation),
            under_run: an_identity(IdentityKind::Run),
        };
        let attempts = std::sync::atomic::AtomicUsize::new(0);
        let answered = channel.keep_attempting(
            &report,
            || attempts.fetch_add(1, std::sync::atomic::Ordering::SeqCst) < 3,
            Duration::from_millis(1),
        );
        assert_eq!(answered, None);
        assert_eq!(attempts.load(std::sync::atomic::Ordering::SeqCst), 4);
    }

    fn a_worker(kind: WorkerKind) -> StartedWorker {
        StartedWorker {
            kind,
            operation: an_identity(IdentityKind::Operation),
            under_run: an_identity(IdentityKind::Run),
        }
    }

    /// The kernel, as the desk can be pointed at it: a root written the moment
    /// the start asks, in the kernel's own published shape.
    struct ARootWrittenAsTheStartAsks {
        root: PathBuf,
    }

    impl ARootWrittenAsTheStartAsks {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "smoozy-runtime-start-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir_all(&root).expect("the root is made");
            ARootWrittenAsTheStartAsks { root }
        }
    }

    fn a_start(
        root: &Path,
        wait: Arc<TheWaitTheSupervisorPerforms>,
        invoked: Option<HowAWorkerOfThisKindIsInvoked>,
    ) -> TheWorkerStartOnTheMachine {
        TheWorkerStartOnTheMachine {
            kernel: ThePairAsTheKernelReportsIt {
                where_the_kernel_publishes: root.to_path_buf(),
            },
            wait,
            how_it_is_invoked: Box::new(move |_| invoked.clone()),
        }
    }

    /// Whether the kernel of the machine this test is running on answers for a
    /// process, through the reader the program itself uses.
    ///
    /// **Every rented machine does, and the desk does too since 2026-10-24**
    /// (step 12's piece SC0b). The branch for a kernel that records nothing is
    /// the test below this one's, posed with an empty root so it runs
    /// everywhere. **Neither stands in for the reading**, which is proved on a
    /// kernel-shaped record in `crate::acting::execution_group`.
    fn the_kernel_here_publishes_records() -> bool {
        matches!(
            ThePairAsTheKernelReportsIt::the_rented_machine().what_is_at(std::process::id()),
            Ok(Some(_))
        )
    }

    /// **A worker is started, its group is recorded as a pair, and the wait has
    /// it.** Runs whole where the kernel answers — every rented machine, the
    /// Linux runner, and the desk.
    #[test]
    fn a_started_worker_is_recorded_as_a_pair_and_is_waited_on() {
        if !the_kernel_here_publishes_records() {
            return;
        }
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = TheWorkerStartOnTheMachine {
            kernel: ThePairAsTheKernelReportsIt::the_rented_machine(),
            wait: wait.clone(),
            how_it_is_invoked: Box::new(|_| {
                Some(HowAWorkerOfThisKindIsInvoked {
                    program: "/bin/sh".into(),
                    arguments: vec!["-c".into(), "exit 5".into()],
                })
            }),
        };
        let WhatAStartAnswers::ItStarted(pair) = start.start(&a_worker(WorkerKind::Render))
        else {
            panic!("it starts and a pair is taken");
        };
        assert!(pair.identifier > 0 && pair.start_time > 0);
        assert_eq!(wait.how_many_it_holds(), 1);
        let group = ExecutionGroup {
            kind: WorkerKind::Render,
            leader: pair,
        };
        let mut ending = None;
        for _ in 0..500 {
            ending = wait.the_ending(&group).expect("the wait is taken");
            if ending.is_some() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert_eq!(ending, Some(5));
    }

    /// **Where the kernel records nothing for a worker that started, the start
    /// says so and the worker is still waited on** — so nothing is left running
    /// unrecorded and no pair is invented to fill the answer.
    #[test]
    fn a_worker_the_kernel_recorded_nothing_for_is_said_and_is_still_waited_on() {
        let holding = ARootWrittenAsTheStartAsks::new();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        // An empty root: the kernel here publishes nothing at all.
        let start = a_start(
            &holding.root,
            wait.clone(),
            Some(HowAWorkerOfThisKindIsInvoked {
                program: "/bin/sh".into(),
                arguments: vec!["-c".into(), "exit 0".into()],
            }),
        );
        let WhatAStartAnswers::ItStartedAndNoPairCouldBeTaken { why, .. } =
            start.start(&a_worker(WorkerKind::Render))
        else {
            panic!("the child ran and nothing was recorded for it, which is not *not attempted*");
        };
        assert!(why.contains("recorded nothing"));
        assert_eq!(
            wait.how_many_it_holds(),
            1,
            "the worker is waited on even where no pair could be recorded"
        );
        std::fs::remove_dir_all(&holding.root).ok();
    }

    /// **The kernel reading's third answer keeps the worker too.**
    ///
    /// The reading has three answers, not two — a pair, an absent record, and a
    /// refusal — and the refusal is the one that used to return with the started
    /// child still a local. Dropping that handle neither ends the process nor
    /// waits on it, so a render worker and the Blender under it would keep
    /// running on a machine the user is paying for, in no record and in no wait.
    ///
    /// **The refusal is made real rather than stood in for**: the reader is
    /// pointed at a place that is not a directory, so every record under it is a
    /// refusal the machine gives rather than an absence — the same shape as a
    /// record that is there and carries no start time, which is the other way
    /// into this branch and is proved at
    /// `crate::acting::execution_group`.
    #[test]
    fn a_worker_whose_kernel_reading_refused_is_said_and_is_still_waited_on() {
        let holding = ARootWrittenAsTheStartAsks::new();
        let not_a_directory = holding.root.join("where the kernel does not publish");
        std::fs::write(&not_a_directory, b"not a directory").expect("it is written");
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_start(
            &not_a_directory,
            wait.clone(),
            Some(HowAWorkerOfThisKindIsInvoked {
                program: "/bin/sh".into(),
                arguments: vec!["-c".into(), "exit 0".into()],
            }),
        );
        let WhatAStartAnswers::ItStartedAndNoPairCouldBeTaken { why, .. } =
            start.start(&a_worker(WorkerKind::Render))
        else {
            panic!("the child ran and the reading refused, which is not *not attempted*");
        };
        assert!(why.contains("could not be read"));
        assert_eq!(
            wait.how_many_it_holds(),
            1,
            "the worker is waited on even where the reading refused"
        );
        std::fs::remove_dir_all(&holding.root).ok();
    }

    /// **A kind whose document has named no worker is not started at all**, and
    /// nothing is guessed at from the kind.
    #[test]
    fn a_kind_whose_document_has_named_no_worker_is_not_started() {
        let holding = ARootWrittenAsTheStartAsks::new();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_start(&holding.root, wait.clone(), None);
        let WhatAStartAnswers::ItWasNotAttempted(why) = start.start(&a_worker(WorkerKind::Media))
        else {
            panic!("nothing names it, so nothing was started");
        };
        assert!(why.contains("has not said what its worker is"));
        assert_eq!(wait.how_many_it_holds(), 0);
        std::fs::remove_dir_all(&holding.root).ok();
    }

    /// **A worker that could not be started is a refusal naming what stopped
    /// it**, which is §14's own wording for the unit that failed.
    #[test]
    fn a_worker_whose_program_is_not_there_is_refused_naming_what_stopped_it() {
        let holding = ARootWrittenAsTheStartAsks::new();
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_start(
            &holding.root,
            wait.clone(),
            Some(HowAWorkerOfThisKindIsInvoked {
                program: "/no/such/worker/at/all".into(),
                arguments: Vec::new(),
            }),
        );
        let WhatAStartAnswers::ItWasNotAttempted(why) = start.start(&a_worker(WorkerKind::Setup))
        else {
            panic!("there is no such program, so nothing was started");
        };
        assert!(why.contains("/no/such/worker/at/all"));
        assert_eq!(wait.how_many_it_holds(), 0);
        std::fs::remove_dir_all(&holding.root).ok();
    }

    /// **The arguments are a list and never a line**, read off what the worker
    /// was actually handed.
    ///
    /// **This asserted the length of a list written out three lines above it
    /// until 2026-09-04**, which could not be false and checked nothing its own
    /// name claims: the start could have joined the list into one line and put a
    /// shell in front of it, and the assertion would still have held. It is the
    /// class must-do 14 swept and it was missed. What stands here instead is a
    /// real start whose one argument is a string no shell leaves alone — spaces,
    /// a semicolon, a variable, quotes and a glob — and the worker writes down
    /// what it received. A line would arrive split, expanded and unquoted.
    #[test]
    fn what_a_worker_is_handed_is_a_list_and_never_a_line() {
        let holding = ARootWrittenAsTheStartAsks::new();
        let wrote_it_down = holding.root.join("what the worker was handed");
        let one_argument = "one two;$HOME 'three' *";
        let wait = Arc::new(TheWaitTheSupervisorPerforms::new());
        let start = a_start(
            &holding.root,
            wait.clone(),
            Some(HowAWorkerOfThisKindIsInvoked {
                program: "/bin/sh".into(),
                arguments: vec![
                    "-c".into(),
                    "printf '%s' \"$1\" > \"$2\"".into(),
                    "the worker".into(),
                    one_argument.into(),
                    wrote_it_down.display().to_string(),
                ],
            }),
        );
        // Nothing is published at this root, so the start says
        // so — and the worker ran all the same, which is what is read below.
        assert!(matches!(
            start.start(&a_worker(WorkerKind::Render)),
            WhatAStartAnswers::ItStartedAndNoPairCouldBeTaken { .. }
        ));
        let mut handed = None;
        for _ in 0..500 {
            match std::fs::read_to_string(&wrote_it_down) {
                Ok(read) if !read.is_empty() => {
                    handed = Some(read);
                    break;
                }
                _ => std::thread::sleep(std::time::Duration::from_millis(10)),
            }
        }
        assert_eq!(handed.as_deref(), Some(one_argument));
        std::fs::remove_dir_all(&holding.root).ok();
    }
}

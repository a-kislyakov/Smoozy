//! The recorded pair read from the kernel, the wait the supervisor performs,
//! and the one place in this crate that reaches a process at all.
//!
//! **[`crate::execution_group`] holds the rules** — what the pair is, what a
//! failed validation means, the seven-step stop and the one boundary at step 7.
//! This module is the reading, the wait and the signal.
//!
//! # The pair, and where its second half comes from
//!
//! X1 records the group leader's identity **paired with its kernel-set start
//! time**, and Fact 91 is the answer that makes the pair worth having: the start
//! time is *"written by the kernel at fork, in clock ticks since boot"* and
//! **it cannot be chosen by the process it describes** — a write to it was
//! refused on both machine kinds.
//!
//! `proc_pid_stat(5)`, read 2026-09-04 at
//! `https://man7.org/linux/man-pages/man5/proc_pid_stat.5.html`, field **(22)**
//! `starttime`: *"The time the process started after system boot. Before Linux
//! 2.6, this value was expressed in jiffies. Since Linux 2.6, the value is
//! expressed in clock ticks (divide by sysconf(_SC_CLK_TCK))."*
//!
//! **Nothing here divides it or turns it into a duration.** X3 hands every
//! question the pair cannot answer — a reboot, a container restart, a recycle —
//! to the run identifier, and this crate subtracts no two readings and states no
//! duration anywhere. The value is compared for equality and for nothing else.
//!
//! **The name field is what makes the parse a parse rather than a split.** Field
//! **(2)** `comm` is *"The filename of the executable, in parentheses."*, and a
//! name may hold spaces and brackets of its own — so the fields after it are
//! taken from **after the last closing bracket on the line** and never by
//! counting from the start. A reader that split the whole line on spaces would
//! be right on every ordinary name and wrong on one process, which is exactly
//! the shape of fault X6 exists to refuse.
//!
//! # What this reading deliberately does not repair
//!
//! **A worker that ended and has not been waited on keeps its entry, and its
//! entry keeps the start time the pair was recorded with** — so the recorded
//! pair goes on validating for a process that no longer exists. That is the
//! plan's own Section 16 item 4 rider, observed twice. **It is not repaired
//! here**: X6 forbids relaxing a validation, X9's own direction is the safe one,
//! and the answer the plan gives is X9a's wait, which is
//! [`TheWaitTheSupervisorPerforms`] below. **So this reader reports what the
//! kernel reports** and does not read a process's state to decide the group has
//! ended.
//!
//! # The signal
//!
//! `kill(2)`, read 2026-09-04 at
//! `https://man7.org/linux/man-pages/man2/kill.2.html`: *"If pid is positive,
//! then signal sig is sent to the process with the ID specified by pid."*, with
//! *"On success, zero is returned."*, *"The target process or process group does
//! not exist. Note that an existing process might be a zombie, a process that
//! has terminated execution, but has not yet been waited for."* and *"The
//! calling process does not have permission to send the signal to any of the
//! target processes."* — which is X16's own refusal, and the observed reason
//! `Job files and outputs` L13 requires one login.
//!
//! **Which signal is not chosen here.** The shape fixes it for the one kind that
//! has it: the interrupt, delivered to the recorded render process rather than
//! to the render worker. **W2 is what stops a second one being invented**: no
//! default signal exists for a kind whose document has not named one, and this
//! module refuses rather than guessing.

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Child;
use std::sync::Mutex;

use crate::execution_group::{
    ExecutionGroup, IssuesTheKindsOwnTermination, RecordedPair, SignalTarget, TerminationContract,
    ValidatesTheRecordedPair, WaitsOnEveryWorkerItStarts,
};

/// Field **(22)** of the kernel's own per-process record, counted from the
/// first field after the name.
///
/// The name is field (2), so the first field after the closing bracket is (3)
/// and the start time sits nineteen further along.
pub const HOW_FAR_PAST_THE_NAME_THE_START_TIME_SITS: usize = 22 - 3;

/// The kernel's own record of a process, read where the kernel publishes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePairAsTheKernelReportsIt {
    /// Where the kernel publishes its per-process records. `/proc` on the
    /// machine Smoozy rents.
    pub where_the_kernel_publishes: PathBuf,
}

impl ThePairAsTheKernelReportsIt {
    /// The rented machine.
    pub fn the_rented_machine() -> Self {
        ThePairAsTheKernelReportsIt {
            where_the_kernel_publishes: PathBuf::from("/proc"),
        }
    }

    fn the_record_of(&self, identifier: u32) -> PathBuf {
        self.where_the_kernel_publishes
            .join(identifier.to_string())
            .join("stat")
    }
}

/// The start time out of one of the kernel's per-process records.
///
/// **The fields are taken from after the last closing bracket**, for the reason
/// at the head of this file.
pub fn the_start_time_in(record: &str) -> Option<u64> {
    let after_the_name = record.rfind(')')? + 1;
    record[after_the_name..]
        .split_whitespace()
        .nth(HOW_FAR_PAST_THE_NAME_THE_START_TIME_SITS)?
        .parse()
        .ok()
}

impl ValidatesTheRecordedPair for ThePairAsTheKernelReportsIt {
    /// What is at an identifier, as a pair; whether the group has ended is
    /// [`RecordedPair::validates_against`]'s. The desk's route: `at_the_desk`.
    fn what_is_at(&self, identifier: u32) -> Result<Option<RecordedPair>, String> {
        #[cfg(all(target_os = "macos", debug_assertions))]
        if self.where_the_kernel_publishes == std::path::Path::new("/proc") {
            return at_the_desk::what_the_desks_kernel_reports_at(identifier);
        }
        let at = self.the_record_of(identifier);
        let record = match std::fs::read_to_string(&at) {
            Ok(record) => record,
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(why) => return Err(format!("{}: {why}", at.display())),
        };
        match the_start_time_in(&record) {
            Some(start_time) => Ok(Some(RecordedPair {
                identifier,
                start_time,
            })),
            // A record that is there and does not carry the field is not an
            // absent process. It is said rather than read as one, because
            // reading it as absent would be X9 firing on a machine that
            // answered something this reader did not understand.
            None => Err(format!(
                "{}: the kernel's record does not carry the start time",
                at.display()
            )),
        }
    }
}

/// **X9a's wait**, which is the positive test that a group has ended.
///
/// The supervisor waits on every worker it starts and takes the ending from the
/// wait status. **Without it a worker that ended still validates at X9, a
/// termination is absorbed by an entry that is no longer a process, and *when
/// the group has ended* never arrives** — which on a setup machine leaves that
/// document's own refusal standing for ever with no act inside the app that
/// clears it.
/// **Every worker this supervisor started, held under the identifier it was
/// started at.**
///
/// **More than one may stand at one identifier and none of them is dropped to
/// make room.** An identifier is re-handed-out — X2's whole point — so a later
/// start can arrive at an identifier this supervisor already holds a worker at.
/// **Replacing that entry would drop the earlier worker's handle unwaited**, and
/// dropping a handle neither ends the process nor takes its ending: the worker
/// would go on running on a machine the user is paying for, in no record and in
/// no wait, which is exactly the hole X9a's wait exists to close.
#[derive(Debug, Default)]
pub struct TheWaitTheSupervisorPerforms {
    workers: Mutex<HashMap<u32, Vec<AWorkerThisSupervisorStarted>>>,
}

#[derive(Debug)]
enum AWorkerThisSupervisorStarted {
    /// Started by this supervisor and not yet ended.
    Running { pair: RecordedPair, child: Child },
    /// Ended, with the ending this supervisor took from the wait status.
    Ended { pair: RecordedPair, ending: i32 },
}

impl AWorkerThisSupervisorStarted {
    /// The recorded pair this entry is, ended or not.
    fn pair(&self) -> RecordedPair {
        match self {
            AWorkerThisSupervisorStarted::Running { pair, .. } => *pair,
            AWorkerThisSupervisorStarted::Ended { pair, .. } => *pair,
        }
    }
}

impl TheWaitTheSupervisorPerforms {
    /// Nothing is being waited on yet.
    pub fn new() -> Self {
        TheWaitTheSupervisorPerforms::default()
    }

    /// Take a worker this supervisor has just started, so that it is waited on.
    ///
    /// **§3.2's one-restarter rule is untouched by this: a wait is not a
    /// restart**, and nothing here starts anything again.
    ///
    /// **Nothing already held is displaced.** A worker arriving at an identifier
    /// this supervisor already holds one at is added beside it, so no started
    /// child is ever dropped without its ending being taken.
    pub fn keep(&self, pair: RecordedPair, child: Child) {
        self.workers
            .lock()
            .expect("the wait is not poisoned")
            .entry(pair.identifier)
            .or_default()
            .push(AWorkerThisSupervisorStarted::Running { pair, child });
    }

    /// How many workers this supervisor is holding, ended or not.
    pub fn how_many_it_holds(&self) -> usize {
        self.workers
            .lock()
            .expect("the wait is not poisoned")
            .values()
            .map(Vec::len)
            .sum()
    }
}

/// **X13's own division, made a value.**
///
/// A worker that was ended by a signal has no exit status of its own. The
/// number recorded for it here is the **negative** of the signal, which cannot
/// collide with an ordinary status and so needs no second field to tell the two
/// apart. **X13 is what makes the choice safe rather than load-bearing:**
/// *"Nothing here matches an ending on its number"* — the number a polite
/// terminate ends with is recorded as `-15` by one topic and `143` by three
/// others, and nothing in this crate branches on either.
pub fn the_number_an_ending_is_recorded_as(status: std::process::ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    match status.code() {
        Some(code) => code,
        None => -status.signal().unwrap_or(0),
    }
}

impl WaitsOnEveryWorkerItStarts for TheWaitTheSupervisorPerforms {
    fn the_ending(&self, group: &ExecutionGroup) -> Result<Option<i32>, String> {
        let mut held = self.workers.lock().expect("the wait is not poisoned");
        let at_that_identifier = held.get_mut(&group.leader.identifier).ok_or_else(|| {
            format!(
                "this supervisor did not start the worker at {}",
                group.leader.identifier
            )
        })?;
        // **The pair is what identifies the worker, and the identifier alone is
        // not** — X2. Where this supervisor holds more than one at an
        // identifier, the recorded pair picks out which.
        let worker = at_that_identifier
            .iter_mut()
            .find(|worker| worker.pair() == group.leader)
            .ok_or_else(|| the_identifier_is_another_workers(group))?;
        match worker {
            AWorkerThisSupervisorStarted::Ended { ending, .. } => Ok(Some(*ending)),
            AWorkerThisSupervisorStarted::Running { pair, child } => {
                match child.try_wait() {
                    Ok(None) => Ok(None),
                    Ok(Some(status)) => {
                        let ending = the_number_an_ending_is_recorded_as(status);
                        let pair = *pair;
                        *worker = AWorkerThisSupervisorStarted::Ended { pair, ending };
                        Ok(Some(ending))
                    }
                    Err(why) => Err(format!(
                        "the wait on the worker at {} could not be taken: {why}",
                        group.leader.identifier
                    )),
                }
            }
        }
    }
}

/// **X2's own finding, made a refusal.** An identifier is re-handed-out; the
/// pair is not.
fn the_identifier_is_another_workers(group: &ExecutionGroup) -> String {
    format!(
        "the worker this supervisor holds at {} is not the recorded pair",
        group.leader.identifier
    )
}

/// Which member of a recorded execution group a contract's target names, as a
/// pair.
///
/// **This is the owning document's record and never this crate's.** X5 says the
/// group is the worker and every external process it starts, X1 records the
/// leader alone, and X12 makes the render's target the recorded render process
/// rather than the leader — so the pair of the member that is **not** the leader
/// is held by the document that recorded it, and it is handed in here.
pub type NamesTheMemberAContractPointsAt =
    Box<dyn Fn(&ExecutionGroup, SignalTarget) -> Option<RecordedPair> + Send + Sync>;

/// The one place in this crate that reaches a process.
pub struct TheKindsOwnTerminationIssued {
    /// The reading that validates a pair before anything is sent to it.
    pub kernel: ThePairAsTheKernelReportsIt,
    /// The owning document's record of which member is which.
    pub the_member: NamesTheMemberAContractPointsAt,
}

impl std::fmt::Debug for TheKindsOwnTerminationIssued {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.debug_struct("TheKindsOwnTerminationIssued")
            .field("kernel", &self.kernel)
            .finish_non_exhaustive()
    }
}

impl TheKindsOwnTerminationIssued {
    /// Whether a signal is sent at all for a target.
    ///
    /// **Two of the four targets send none and each is a clause rather than an
    /// omission**: X14 makes transfer-support's termination staging and cleanup
    /// rather than a signal, so X8 step 4 issues none for that kind; and X15
    /// makes a setup run's termination at, or recovery from, a recorded safe
    /// boundary, whose meaning is `Server Setup`'s.
    ///
    /// **This answers the same question [`Self::issue`] answers**, and the two
    /// agree by being written over the same total match: a target added to the
    /// enum has no arm here and the crate stops compiling, rather than the
    /// helper saying *no signal* while the issuer answers with an error.
    pub fn whether_a_signal_is_sent_for(target: SignalTarget) -> bool {
        match target {
            SignalTarget::TheRecordedRenderProcess
            | SignalTarget::TheRecordedExternalBlenderProcess
            | SignalTarget::TheRoleWorkersOwnProcess => true,
            SignalTarget::NoSignalStagingAndCleanupInstead
            | SignalTarget::AtTheRecordedSafeBoundary => false,
        }
    }
}

impl IssuesTheKindsOwnTermination for TheKindsOwnTerminationIssued {
    fn issue(
        &self,
        group: &ExecutionGroup,
        contract: TerminationContract,
    ) -> Result<(), String> {
        match contract.target {
            SignalTarget::NoSignalStagingAndCleanupInstead
            | SignalTarget::AtTheRecordedSafeBoundary => Ok(()),
            SignalTarget::TheRecordedRenderProcess
            | SignalTarget::TheRecordedExternalBlenderProcess
            | SignalTarget::TheRoleWorkersOwnProcess => {
                let recorded = match contract.target {
                    // **`Telegram` §19: the group is itself alone**, so the
                    // member this contract points at is the leader X1 recorded
                    // and there is no second member for an owning document to
                    // have recorded one of.
                    SignalTarget::TheRoleWorkersOwnProcess => group.leader,
                    _ => (self.the_member)(group, contract.target).ok_or_else(|| {
                        format!(
                            "the document that owns the {:?} kind has recorded no pair for {:?}",
                            group.kind, contract.target
                        )
                    })?,
                };
                // **The member is validated before anything is sent to it.**
                // A bare identifier is re-handed-out, so signalling one is the
                // thing X6 refuses and `State, identity and recovery` §2.1
                // rule 5 forbids outright.
                let what_is_there = self.kernel.what_is_at(recorded.identifier)?;
                if recorded.validates_against(what_is_there).the_group_is_gone() {
                    return Err(format!(
                        "the recorded member at {} no longer validates; nothing was sent to it",
                        recorded.identifier
                    ));
                }
                send_the_interrupt_to(recorded.identifier)
            }
        }
    }
}

/// The interrupt, to one identifier.
///
/// **The shape fixes the signal and `Job files and outputs` C36 fixes the
/// reason**: *"A render had to be started detached to survive the connection at
/// all, so the thing that must be reached is by construction not the thing the
/// connection started."*
pub fn send_the_interrupt_to(identifier: u32) -> Result<(), String> {
    let sent = unsafe { libc::kill(identifier as libc::pid_t, libc::SIGINT) };
    if sent == 0 {
        Ok(())
    } else {
        Err(format!(
            "the interrupt to {identifier} was refused: {}",
            std::io::Error::last_os_error()
        ))
    }
}

/// A kernel-shaped record written where a reader can be pointed at it.
///
/// **This stands in for the kernel and never for the reading.** The reading is
/// [`ThePairAsTheKernelReportsIt::what_is_at`] run whole, over a record in the
/// kernel's own published shape.
#[cfg(test)]
pub(crate) fn a_record_in_the_kernels_shape(
    root: &std::path::Path,
    identifier: u32,
    named: &str,
    state: char,
    start_time: u64,
) {
    let at = root.join(identifier.to_string());
    std::fs::create_dir_all(&at).expect("the record's place is made");
    let mut fields = vec![identifier.to_string(), format!("({named})"), state.to_string()];
    // Fields (4) to (21), whose values nothing here reads.
    for field in 4..=21 {
        fields.push(field.to_string());
    }
    fields.push(start_time.to_string());
    for field in 23..=52 {
        fields.push(field.to_string());
    }
    std::fs::write(at.join("stat"), fields.join(" ")).expect("the record is written");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use crate::execution_group::Validation;
    use crate::workers::WorkerKind;

    fn a_root() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "smoozy-runtime-group-{}-{:?}",
            std::process::id(),
            {
                static NEXT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }
        ));
        std::fs::create_dir_all(&root).expect("the root is made");
        root
    }

    fn reading(root: &Path) -> ThePairAsTheKernelReportsIt {
        ThePairAsTheKernelReportsIt {
            where_the_kernel_publishes: root.to_path_buf(),
        }
    }

    #[test]
    fn the_start_time_is_taken_from_after_the_name_and_never_by_counting_from_the_start() {
        let root = a_root();
        a_record_in_the_kernels_shape(&root, 1493, "blender", 'S', 19186);
        assert_eq!(
            reading(&root).what_is_at(1493).unwrap(),
            Some(RecordedPair {
                identifier: 1493,
                start_time: 19186
            })
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **The parse that a split on spaces gets wrong.** A name holding spaces
    /// and brackets of its own is the one process a counted reader is wrong on.
    #[test]
    fn a_name_holding_spaces_and_brackets_does_not_move_the_field() {
        let root = a_root();
        a_record_in_the_kernels_shape(&root, 77, "a name (with) brackets", 'S', 4242);
        assert_eq!(
            reading(&root).what_is_at(77).unwrap().unwrap().start_time,
            4242
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **X2, taken through this crate's own reading.** The identifier is handed
    /// out again to an unrelated process and the recorded pair does not
    /// validate.
    #[test]
    fn a_re_handed_out_identifier_does_not_validate_against_the_recorded_pair() {
        let root = a_root();
        let recorded = RecordedPair {
            identifier: 1493,
            start_time: 19186,
        };
        a_record_in_the_kernels_shape(&root, 1493, "something else", 'S', 908_070);
        let what_is_there = reading(&root).what_is_at(1493).unwrap();
        assert_eq!(
            recorded.validates_against(what_is_there),
            Validation::ExistsButDoesNotMatch
        );
        assert!(recorded.validates_against(what_is_there).the_group_is_gone());
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn an_identifier_the_kernel_holds_nothing_for_is_gone_rather_than_a_refusal() {
        let root = a_root();
        assert_eq!(reading(&root).what_is_at(4242).unwrap(), None);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **The rider this module refuses to repair.** An entry that is there but
    /// is no longer a running process keeps the start time the pair was
    /// recorded with, and the pair goes on validating. X9a's wait is the answer
    /// and X6 forbids relaxing the validation instead.
    #[test]
    fn an_entry_that_is_no_longer_a_process_still_validates_and_is_not_repaired_here() {
        let root = a_root();
        let recorded = RecordedPair {
            identifier: 999,
            start_time: 5150,
        };
        a_record_in_the_kernels_shape(&root, 999, "a worker", 'Z', 5150);
        assert_eq!(
            recorded.validates_against(reading(&root).what_is_at(999).unwrap()),
            Validation::Matches,
            "the entry's state is not read to decide the group has ended"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_record_that_does_not_carry_the_field_is_said_rather_than_read_as_absent() {
        let root = a_root();
        let at = root.join("5");
        std::fs::create_dir_all(&at).unwrap();
        std::fs::write(at.join("stat"), "5 (a worker) S").unwrap();
        let why = reading(&root).what_is_at(5).expect_err("it does not carry it");
        assert!(why.contains("start time"));
        std::fs::remove_dir_all(&root).ok();
    }

    fn a_group(pair: RecordedPair) -> ExecutionGroup {
        ExecutionGroup {
            kind: WorkerKind::Render,
            leader: pair,
        }
    }

    /// **X9a's wait, run against a real process.**
    #[test]
    fn the_wait_takes_a_real_endings_status_and_answers_the_same_way_afterwards() {
        let child = std::process::Command::new("/bin/sh")
            .args(["-c", "exit 7"])
            .spawn()
            .expect("the desk starts it");
        let pair = RecordedPair {
            identifier: child.id(),
            start_time: 1,
        };
        let waiting = TheWaitTheSupervisorPerforms::new();
        waiting.keep(pair, child);
        let group = a_group(pair);
        let mut ending = None;
        for _ in 0..500 {
            ending = waiting.the_ending(&group).expect("the wait is taken");
            if ending.is_some() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert_eq!(ending, Some(7));
        assert_eq!(waiting.the_ending(&group).unwrap(), Some(7));
    }

    /// **A worker still running has no ending**, which is what keeps *when the
    /// group has ended* from arriving early.
    #[test]
    fn a_worker_that_is_still_running_has_no_ending_yet() {
        let child = std::process::Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .expect("the desk starts it");
        let pair = RecordedPair {
            identifier: child.id(),
            start_time: 1,
        };
        let group = a_group(pair);
        let waiting = TheWaitTheSupervisorPerforms::new();
        waiting.keep(pair, child);
        assert_eq!(waiting.the_ending(&group).unwrap(), None);
        end_it(&waiting, &group);
    }

    /// End a worker the wait is holding and take its ending, so a test leaves
    /// nothing running behind it.
    fn end_it(waiting: &TheWaitTheSupervisorPerforms, group: &ExecutionGroup) -> Option<i32> {
        unsafe { libc::kill(group.leader.identifier as libc::pid_t, libc::SIGKILL) };
        for _ in 0..500 {
            if let Ok(Some(ending)) = waiting.the_ending(group) {
                return Some(ending);
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        None
    }

    /// Wait for a worker the wait is holding to end on its own.
    fn its_ending(waiting: &TheWaitTheSupervisorPerforms, group: &ExecutionGroup) -> Option<i32> {
        for _ in 0..500 {
            match waiting.the_ending(group) {
                Ok(Some(ending)) => return Some(ending),
                Ok(None) => std::thread::sleep(std::time::Duration::from_millis(10)),
                Err(why) => panic!("{why}"),
            }
        }
        None
    }

    #[test]
    fn a_worker_this_supervisor_did_not_start_is_not_waited_on() {
        let waiting = TheWaitTheSupervisorPerforms::new();
        let why = waiting
            .the_ending(&a_group(RecordedPair {
                identifier: 1,
                start_time: 1,
            }))
            .expect_err("it started nothing");
        assert!(why.contains("did not start"));
    }

    /// **An ending by signal is recorded as a number that cannot be an exit
    /// status**, and nothing anywhere matches on it.
    #[test]
    fn an_ending_by_signal_is_recorded_as_a_number_of_its_own() {
        let child = std::process::Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .expect("the desk starts it");
        let pair = RecordedPair {
            identifier: child.id(),
            start_time: 1,
        };
        let waiting = TheWaitTheSupervisorPerforms::new();
        waiting.keep(pair, child);
        assert_eq!(end_it(&waiting, &a_group(pair)), Some(-libc::SIGKILL));
    }

    fn a_contract(target: SignalTarget) -> TerminationContract {
        use crate::execution_group::{Cleanup, WhenItMayBeTerminated};
        TerminationContract {
            target,
            when: WhenItMayBeTerminated::AtAnyMoment,
            cleanup: Cleanup::NoneNamedByItsOwningDocument,
            the_worker_outlives_every_member: true,
        }
    }

    fn issuing(root: &Path, member: Option<RecordedPair>) -> TheKindsOwnTerminationIssued {
        TheKindsOwnTerminationIssued {
            kernel: reading(root),
            the_member: Box::new(move |_, _| member),
        }
    }

    /// **X12, run rather than reasoned.** The interrupt reaches the recorded
    /// member the owning document named, and the leader — a second real process
    /// — is untouched by it.
    #[test]
    fn the_interrupt_reaches_the_recorded_member_and_not_the_leader() {
        let root = a_root();
        let waiting = TheWaitTheSupervisorPerforms::new();

        let leader_process = std::process::Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .expect("the desk starts the leader");
        let leader = RecordedPair {
            identifier: leader_process.id(),
            start_time: 1,
        };
        waiting.keep(leader, leader_process);

        let member_process = std::process::Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .expect("the desk starts the member");
        let member = RecordedPair {
            identifier: member_process.id(),
            start_time: 3131,
        };
        waiting.keep(member, member_process);
        a_record_in_the_kernels_shape(&root, member.identifier, "sleep", 'S', 3131);

        issuing(&root, Some(member))
            .issue(
                &a_group(leader),
                a_contract(SignalTarget::TheRecordedRenderProcess),
            )
            .expect("the interrupt is issued");

        assert_eq!(
            its_ending(&waiting, &a_group(member)),
            Some(-libc::SIGINT),
            "the member took the interrupt"
        );
        assert_eq!(
            waiting.the_ending(&a_group(leader)).unwrap(),
            None,
            "nothing was sent to the leader"
        );
        end_it(&waiting, &a_group(leader));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **`Telegram` section 19, run rather than reasoned: the polite signal
    /// reaches the role worker's own process.**
    ///
    /// **The group is itself alone**, that document's own words, so the member
    /// is the leader X1 recorded — and no owning document has a second member
    /// recorded for this kind, which is why the issuance answers without asking
    /// for one. **This program signalled nothing at all until 2026-09-12**, on a
    /// written-down note saying no document said how, and a user who switched
    /// Telegram off went on getting messages on their phone.
    #[test]
    fn the_role_workers_polite_signal_reaches_its_own_process() {
        let root = a_root();
        let waiting = TheWaitTheSupervisorPerforms::new();

        let worker = std::process::Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .expect("the desk starts the role worker");
        let leader = RecordedPair {
            identifier: worker.id(),
            start_time: 5150,
        };
        waiting.keep(leader, worker);
        a_record_in_the_kernels_shape(&root, leader.identifier, "sleep", 'S', 5150);

        // **No member is recorded and none is needed.** A callback answering
        // `None` is what this program hands the issuance, so a target that
        // reached for one would fail here rather than signal the worker.
        issuing(&root, None)
            .issue(
                &ExecutionGroup {
                    kind: WorkerKind::Telegram,
                    leader,
                },
                a_contract(SignalTarget::TheRoleWorkersOwnProcess),
            )
            .expect("the polite signal is issued to the worker's own process");

        assert_eq!(
            its_ending(
                &waiting,
                &ExecutionGroup {
                    kind: WorkerKind::Telegram,
                    leader,
                }
            ),
            Some(-libc::SIGINT),
            "the role worker's own process took the polite signal"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **The role worker's target sends a signal**, and the two places that
    /// answer that question agree because both are written over the same total
    /// match.
    #[test]
    fn the_role_workers_target_is_one_that_sends_a_signal() {
        assert!(TheKindsOwnTerminationIssued::whether_a_signal_is_sent_for(
            SignalTarget::TheRoleWorkersOwnProcess
        ));
    }

    #[test]
    fn a_member_whose_pair_no_longer_validates_is_sent_nothing() {
        let root = a_root();
        let member = RecordedPair {
            identifier: 4242,
            start_time: 1,
        };
        a_record_in_the_kernels_shape(&root, 4242, "somebody else", 'S', 99);
        let why = issuing(&root, Some(member))
            .issue(
                &a_group(member),
                a_contract(SignalTarget::TheRecordedRenderProcess),
            )
            .expect_err("it does not validate");
        assert!(why.contains("nothing was sent"));
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_kind_whose_document_recorded_no_member_is_sent_nothing() {
        let root = a_root();
        let why = issuing(&root, None)
            .issue(
                &a_group(RecordedPair {
                    identifier: 1,
                    start_time: 1,
                }),
                a_contract(SignalTarget::TheRecordedRenderProcess),
            )
            .expect_err("no member is recorded");
        assert!(why.contains("recorded no pair"));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **The media kind is signalled rather than refused.** Its own document
    /// names the member — the recorded external process, the Blender it drives —
    /// so W2's *no default signal is invented* is not the rule for it, and the
    /// contract is filed under the member the enum already had.
    ///
    /// **The case is A6's**: a media production displaced by a render.
    #[test]
    fn a_media_production_displaced_by_a_render_is_signalled_rather_than_refused() {
        let root = a_root();
        let waiting = TheWaitTheSupervisorPerforms::new();

        let blender = std::process::Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .expect("the desk starts the Blender it drives");
        let member = RecordedPair {
            identifier: blender.id(),
            start_time: 7171,
        };
        waiting.keep(member, blender);
        a_record_in_the_kernels_shape(&root, member.identifier, "blender", 'S', 7171);

        let target = SignalTarget::TheRecordedExternalBlenderProcess;
        assert!(TheKindsOwnTerminationIssued::whether_a_signal_is_sent_for(target));
        let media = ExecutionGroup {
            kind: WorkerKind::Media,
            leader: member,
        };
        issuing(&root, Some(member))
            .issue(&media, a_contract(target))
            .expect("the media kind's own document names this member");
        assert_eq!(
            its_ending(&waiting, &media),
            Some(-libc::SIGINT),
            "the recorded external process took the termination"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A worker arriving at an identifier this supervisor already holds one at
    /// is added beside it, and the earlier one is not dropped unwaited.**
    ///
    /// An identifier is re-handed-out, so this arrives; a handle dropped to make
    /// room would leave a started worker in no wait at all.
    #[test]
    fn a_second_worker_at_one_identifier_does_not_displace_the_first() {
        let child = std::process::Command::new("/bin/sh")
            .args(["-c", "exit 4"])
            .spawn()
            .expect("the desk starts it");
        let identifier = child.id();
        let earlier = RecordedPair {
            identifier,
            start_time: 11,
        };
        let waiting = TheWaitTheSupervisorPerforms::new();
        waiting.keep(earlier, child);

        // A later start at the same identifier, which is what a re-handed-out
        // identifier looks like from this side.
        let second = std::process::Command::new("/bin/sh")
            .args(["-c", "exit 6"])
            .spawn()
            .expect("the desk starts it");
        let later = RecordedPair {
            identifier,
            start_time: 22,
        };
        // The second child's own identifier is its own; what is under test is
        // the entry at `identifier`, so it is kept under the recorded pair the
        // caller hands in.
        waiting.keep(later, second);

        assert_eq!(waiting.how_many_it_holds(), 2, "neither was displaced");
        assert_eq!(
            its_ending(&waiting, &a_group(earlier)),
            Some(4),
            "the earlier worker's ending is still taken"
        );
        assert_eq!(its_ending(&waiting, &a_group(later)), Some(6));
    }

    #[test]
    fn the_two_kinds_whose_termination_is_not_a_signal_send_none_and_do_not_fail() {
        let root = a_root();
        for target in [
            SignalTarget::NoSignalStagingAndCleanupInstead,
            SignalTarget::AtTheRecordedSafeBoundary,
        ] {
            assert!(!TheKindsOwnTerminationIssued::whether_a_signal_is_sent_for(target));
            issuing(&root, None)
                .issue(
                    &a_group(RecordedPair {
                        identifier: 1,
                        start_time: 1,
                    }),
                    a_contract(target),
                )
                .expect("the sequence completes on its own document's act");
        }
        std::fs::remove_dir_all(&root).ok();
    }
}

/// **X1's pair at the desk, which is macOS and has no `/proc`.** Added
/// 2026-10-24, step 12's piece SC0b.
///
/// **It serves the desk's tests and no rented machine.** It is compiled only
/// for macOS and only into a development build, so the program a rented Linux
/// machine runs holds none of it, and a release build of the desktop holds
/// none of it either. [`ThePairAsTheKernelReportsIt::what_is_at`] takes this
/// route only where it was pointed at `/proc` itself; a reader a test pointed
/// at a folder of its own reads that folder, at the desk as on Linux.
///
/// **Exactly what the Linux reading takes is taken here, and nothing more**:
/// the identifier the start handed back and the kernel's own start time for
/// it. The Linux reading reads field (22) alone out of the kernel's record, so
/// no process group, parent or name is read here either.
///
/// # The call, and where each part of it is published
///
/// - **`sysctl(3)` with `CTL_KERN`, `KERN_PROC`, `KERN_PROC_PID` and the
///   identifier.** Documented: Apple's own manual page, *Mac OS X Manual Page
///   For sysctl(3)*, read 2026-10-24 at
///   `https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/sysctl.3.html`:
///   *"KERN_PROC Return the entire process table, or a subset of it."*, whose
///   third level name `KERN_PROC_PID` takes *"A process ID"* as its fourth;
///   and *"Upon successful completion, the value 0 is returned; otherwise the
///   value -1 is returned and the global variable errno is set to indicate the
///   error."* Apple's Technical Q&A QA1123, read the same day at
///   `https://developer.apple.com/library/archive/qa/qa2001/qa1123.html`,
///   uses the same call and says *"You do not need any special privileges to
///   make this sysctl"*. **The manual page that ships with macOS 26.6.2 (build
///   25G83) no longer lists `KERN_PROC`**; the system's development files
///   still declare it, below.
/// - **What it returns, and the two fields.** Documented in the system's
///   development files, read 2026-10-24 in `MacOSX.sdk/usr/include` on macOS
///   26.6.2: `sys/sysctl.h` declares *"#define KERN_PROC 14 /\* struct: process
///   entries \*/"*, *"#define KERN_PROC_PID 1 /\* by process id \*/"* and
///   `struct kinfo_proc`, whose first member is *"struct extern_proc kp_proc;
///   /\* proc structure \*/"*; `sys/proc.h` declares in `struct extern_proc`
///   *"struct timeval __p_starttime; /\* process start time \*/"* inside its
///   first member, the union `p_un`, and *"pid_t p_pid; /\* Process identifier.
///   \*/"* further on.
/// - **Observed at the desk, 2026-10-24, with a program built against those
///   files**: the entry is 648 bytes, the start time sits at offset 0 and is 16
///   bytes, the identifier sits at offset 40; and **an identifier nothing holds
///   is answered with success and a length of nought** — the manual page does
///   not say what it is answered with, so that reading is this observation's.
///   The layout below is checked against an independent route by this
///   module's own test.
///
/// # What is not established, and why it does not matter here
///
/// **Fact 91's second half — that a process cannot choose its own start time
/// — is the Linux kernel's and is not established for macOS.** Nothing at the
/// desk relies on it: the desk is not a machine a user pays for, and the pair
/// is compared for equality and for nothing else, exactly as on Linux. **The
/// value is the start time in microseconds**, seconds and microseconds joined
/// into one number, which no Linux reading is ever compared with.
#[cfg(all(target_os = "macos", debug_assertions))]
mod at_the_desk {
    use crate::execution_group::RecordedPair;

    /// The head of `struct kinfo_proc`, as far as `p_pid`, in the order
    /// `sys/proc.h` declares it. **Only the first and the last are read.**
    ///
    /// The union `p_un` is as large as its larger member, and both of its
    /// members are 16 bytes on this desk, so the start time stands for it.
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct TheHeadOfTheKernelsEntry {
        p_starttime: libc::timeval,
        p_vmspace: *mut libc::c_void,
        p_sigacts: *mut libc::c_void,
        p_flag: libc::c_int,
        p_stat: libc::c_char,
        p_pid: libc::pid_t,
    }

    /// Room for one entry, larger than the 648 bytes this desk measured, so a
    /// later macOS that adds to the end of the entry is still answered.
    const ROOM_FOR_ONE_ENTRY: usize = 4096;

    /// **The pair at an identifier, from the kernel of the desk.**
    pub(super) fn what_the_desks_kernel_reports_at(
        identifier: u32,
    ) -> Result<Option<RecordedPair>, String> {
        let mut name = [
            libc::CTL_KERN,
            libc::KERN_PROC,
            libc::KERN_PROC_PID,
            identifier as libc::c_int,
        ];
        let mut room = vec![0u8; ROOM_FOR_ONE_ENTRY];
        let mut length = room.len();
        let answered = unsafe {
            libc::sysctl(
                name.as_mut_ptr(),
                name.len() as libc::c_uint,
                room.as_mut_ptr() as *mut libc::c_void,
                &mut length,
                std::ptr::null_mut(),
                0,
            )
        };
        if answered != 0 {
            return Err(format!(
                "the desk's kernel did not answer for {identifier}: {}",
                std::io::Error::last_os_error()
            ));
        }
        // **Nothing holds that identifier.** Observed, as said above.
        if length == 0 {
            return Ok(None);
        }
        if length < std::mem::size_of::<TheHeadOfTheKernelsEntry>() {
            return Err(format!(
                "the desk's kernel answered {length} bytes for {identifier}, too few for its own entry"
            ));
        }
        let head: TheHeadOfTheKernelsEntry =
            unsafe { std::ptr::read_unaligned(room.as_ptr() as *const TheHeadOfTheKernelsEntry) };
        // **An answer for another process is said rather than taken**, the
        // same discipline as a record the Linux reading does not understand.
        if head.p_pid as u32 != identifier {
            return Err(format!(
                "the desk's kernel was asked about {identifier} and answered about {}",
                head.p_pid
            ));
        }
        Ok(Some(RecordedPair {
            identifier,
            start_time: (head.p_starttime.tv_sec as u64) * 1_000_000
                + head.p_starttime.tv_usec as u64,
        }))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::acting::execution_group::ThePairAsTheKernelReportsIt;
        use crate::execution_group::{ValidatesTheRecordedPair, Validation};

        /// **A second route to the same two fields, for checking the layout
        /// and nothing else**: `proc_pidinfo` with `PROC_PIDTBSDINFO`, whose
        /// `struct proc_bsdinfo` declares `pbi_start_tvsec` and
        /// `pbi_start_tvusec` (`sys/proc_info.h`, same files, same day).
        /// `libproc.h` calls it a private interface, which is why the reading
        /// above does not use it.
        fn the_start_time_by_the_other_route(identifier: u32) -> u64 {
            let mut info: libc::proc_bsdinfo = unsafe { std::mem::zeroed() };
            let size = std::mem::size_of::<libc::proc_bsdinfo>() as libc::c_int;
            let written = unsafe {
                libc::proc_pidinfo(
                    identifier as libc::c_int,
                    libc::PROC_PIDTBSDINFO,
                    0,
                    &mut info as *mut _ as *mut libc::c_void,
                    size,
                )
            };
            assert_eq!(written, size, "the other route answered");
            info.pbi_start_tvsec * 1_000_000 + info.pbi_start_tvusec
        }

        #[test]
        fn the_desks_pair_is_the_start_the_other_route_reports_and_it_holds_still() {
            let mut child = std::process::Command::new("/bin/sleep")
                .arg("30")
                .spawn()
                .expect("the desk starts it");
            let first = what_the_desks_kernel_reports_at(child.id())
                .expect("the desk's kernel answers")
                .expect("it holds the process");
            assert_eq!(first.identifier, child.id());
            assert_eq!(first.start_time, the_start_time_by_the_other_route(child.id()));
            std::thread::sleep(std::time::Duration::from_millis(50));
            assert_eq!(
                what_the_desks_kernel_reports_at(child.id()).unwrap(),
                Some(first),
                "the same process reads as the same pair"
            );
            child.kill().ok();
            child.wait().ok();
        }

        /// **The route the program takes**: the reader pointed at `/proc`, as
        /// the program points it, answers at the desk.
        #[test]
        fn the_reader_pointed_at_proc_answers_at_the_desk() {
            let reader = ThePairAsTheKernelReportsIt::the_rented_machine();
            let mine = std::process::id();
            let pair = reader.what_is_at(mine).unwrap().expect("this process is held");
            assert_eq!(pair.validates_against(reader.what_is_at(mine).unwrap()), Validation::Matches);
        }

        /// **A process that ended and was reaped is gone**, and a pair whose
        /// start time is not the process's own does not validate — X2's two
        /// halves, at the desk.
        #[test]
        fn a_reaped_process_is_gone_and_a_wrong_start_time_does_not_match() {
            let mut child = std::process::Command::new("/bin/sleep")
                .arg("30")
                .spawn()
                .expect("the desk starts it");
            let pair = what_the_desks_kernel_reports_at(child.id()).unwrap().unwrap();
            let another = RecordedPair {
                identifier: pair.identifier,
                start_time: pair.start_time + 1,
            };
            assert_eq!(
                another.validates_against(what_the_desks_kernel_reports_at(child.id()).unwrap()),
                Validation::ExistsButDoesNotMatch
            );
            child.kill().ok();
            child.wait().ok();
            assert_eq!(what_the_desks_kernel_reports_at(pair.identifier).unwrap(), None);
        }

        /// **A process that ended and has not been reaped still reads as its
        /// own pair at the desk**, which is the same rider the Linux reading
        /// keeps and does not repair (the head of this file). Observed
        /// 2026-10-24 rather than documented; X9a's wait is the answer here
        /// as there.
        #[test]
        fn an_ended_process_not_yet_reaped_still_reads_as_its_pair() {
            let mut child = std::process::Command::new("/bin/sleep")
                .arg("30")
                .spawn()
                .expect("the desk starts it");
            let pair = what_the_desks_kernel_reports_at(child.id()).unwrap().unwrap();
            unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGKILL) };
            std::thread::sleep(std::time::Duration::from_millis(200));
            let unreaped = what_the_desks_kernel_reports_at(pair.identifier);
            child.wait().ok();
            assert_eq!(unreaped.unwrap(), Some(pair));
        }
    }
}

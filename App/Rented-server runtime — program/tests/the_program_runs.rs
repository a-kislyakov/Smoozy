//! The program the rented machine runs, **run** — a supervisor started from a
//! package area, and the SSH command adapter asking it G12's health question
//! through the machine-local endpoint.
//!
//! # Why this is here rather than beside the code
//!
//! **It is the only place in this crate that starts the real binary**, which a
//! unit test inside that binary cannot do. What it proves is the thing step 8a
//! exists for: that the package's own entry point, invoked the way the boot
//! registration and the command channel invoke it, is a supervisor and an adapter
//! and that the two reach each other.
//!
//! # What it proves and what it cannot
//!
//! **Proved here, on the desk, with the real acts**: the entry point is entered at
//! its fixed name through the active-package pointer; the directory the pointer
//! sits in is the package area and every record is found from it; the supervisor
//! binds the endpoint at the path the placement recorded; the adapter reads one
//! framed request on its standard input, passes it over the endpoint and writes
//! the correlated result on its standard output; and G12's four things come back
//! and agree with the records on disk.
//!
//! **And, from 2026-09-07, §4's dispatch over that same route**: every name this
//! package serves answered and every other refused by name; U9's answer to a hold
//! written once and read back off the disk; P3's two acts on the
//! setup-in-progress state with V9's report of it riding the health answer
//! between them; and **§14's one unit starting a real worker of this program and
//! reaping it.**
//!
//! **Not proved here, and neither is claimed:** anything about a rented machine.
//! The desk is macOS and the machine is Linux; K4's enable, K6's lingering grant
//! and K5's read-back are the machine's own tools and do not exist here, and
//! `The shape`'s own caution binds — *"Read it as this is what that box did, never
//! as this is what a rented machine does."* **X1's pair is taken differently at
//! the desk**: macOS publishes no per-process records, so a development build
//! there asks macOS's own process table instead (from 2026-10-24, step 12's
//! piece SC0b). Where no pair can be taken, the honest half asserts what it can
//! — a real child started, waited on, and no pair invented.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` — B10, C2.
//!
//! **A marker this plan also defines is never listed here and is written out with
//! its document at the site**, because a header list cannot tell the two apart and
//! a bare one there would resolve to a real and wrong clause.

#![cfg(unix)]

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use smoozy_framing::framing::{
    read_one_result, write_one_frame, write_request_frame, Request,
    THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};
use smoozy_runtime::acting::endpoint::TheEndpointAsACaller;
use smoozy_runtime::acting::operation_records::TheOperationRecordsOnTheMachine;
use smoozy_runtime::execution_group::RecordedPair;
use smoozy_runtime::machine_kind::MachineKind;
use smoozy_runtime::package::phase::{a_fresh_attempts_first_write, Phase};
use smoozy_runtime::package::release::VerifiedReleaseDescriptor;
use smoozy_runtime::the_program::{
    records, the_active_package_pointer_at, the_record_at, THE_FIXED_ENTRY_POINT_NAME,
    THE_SUPERVISOR_WORD,
};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::machine_operations::{
    Disposition, Keeps, OperationRecord, RecordedOutcome, TheRecordedExecutionGroup,
    WhatTheMachineHoldsUnderAnIdentity,
};

/// How long this test waits for the supervisor to be holding the endpoint.
///
/// **A test's own bound and nothing of the plan's.** K17 forbids budgeting a
/// fixed wait for a machine; what is bounded here is a test at the desk, so that
/// a supervisor that never binds fails the test instead of hanging it.
///
/// **Ten seconds until 2026-09-07, and what the change is about is the desk's
/// load rather than the program.** Every test in this file that starts a
/// supervisor failed together at that bound during a whole-workspace run, and
/// every one of them passed on its own straight afterwards — the shape the
/// sitting before this one saw once and could not reproduce in a hundred and
/// twenty-seven runs. **A whole-workspace run puts twenty-odd test binaries on
/// this machine at once**, and a bound tight enough to fail there is a bound
/// about how busy the machine is. **No assertion moved**: what a supervisor that
/// never comes up meets is still a failure, and it is still bounded rather than
/// hanging.
const HOW_LONG_THE_SUPERVISOR_IS_GIVEN: Duration = Duration::from_secs(60);

/// How long **one** exchange of the readiness gate is given before it is asked
/// again.
///
/// **A test's own bound, and it is short so that the gate really is a run of
/// attempts.** Handing the caller the whole ten seconds for a single exchange
/// would make the gate one attempt with a long deadline, and an exchange lost for
/// any reason at all would then fail the test rather than be asked again. The
/// deadline the supervisor gives one caller's request is thirty seconds, so
/// nothing on its side is being cut short by this.
const HOW_LONG_ONE_GATE_EXCHANGE_IS_GIVEN: Duration = Duration::from_secs(1);

/// A package area a placement might have written, with the four records the
/// program reads and the pointer it is entered through.
struct APackageArea {
    at: PathBuf,
    installation: Identity,
    supervisor: Option<Child>,
}

impl APackageArea {
    fn made() -> APackageArea {
        let at = std::env::temp_dir().join(format!(
            "smoozy-server-run-{}",
            Identity::issue(IdentityKind::Operation)
                .expect("randomness")
                .as_text()
        ));
        std::fs::create_dir_all(&at).expect("the area is made");

        // **The active-package pointer**, which is a link whose target is the
        // placed artifact and whose own name is the fixed entry point.
        std::os::unix::fs::symlink(
            Path::new(env!("CARGO_BIN_EXE_smoozy-server")),
            the_active_package_pointer_at(&at.display().to_string()),
        )
        .expect("the pointer is made");

        // **Z2's recorded path.** It is short on purpose: a filesystem-named
        // socket has a length limit the temporary directory can otherwise
        // exhaust, and this is the test's own arrangement rather than the plan's.
        let endpoint = std::env::temp_dir().join(format!(
            "smzy-{}.sock",
            &Identity::issue(IdentityKind::Operation)
                .expect("randomness")
                .as_text()[..12]
        ));
        std::fs::write(
            the_record_at(
                &at.display().to_string(),
                records::THE_ENDPOINTS_RECORDED_PATH,
            ),
            endpoint.display().to_string(),
        )
        .expect("the endpoint's path is recorded");

        // **G10's installation identity**, written where the commit's own read
        // expects it.
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        std::fs::write(
            the_record_at(
                &at.display().to_string(),
                records::THE_INSTALLATION_IDENTITY,
            ),
            installation.as_text(),
        )
        .expect("the identity is written");

        // **G2's phase record**, carrying B10's verified release descriptor by
        // value and the kind detected for this attempt.
        let record = a_fresh_attempts_first_write(
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            VerifiedReleaseDescriptor {
                manifest_format: "1".to_string(),
                verified_release_sequence: 1,
                requested_package_version: env!("CARGO_PKG_VERSION").to_string(),
                manifest_digest: [0; 32],
                artifact_identity: THE_FIXED_ENTRY_POINT_NAME.to_string(),
                artifact_byte_length: 0,
                artifact_digest: [0; 32],
            },
            None,
            MachineKind::Systemd,
            at.join("staged").display().to_string(),
        );
        std::fs::write(
            the_record_at(&at.display().to_string(), records::THE_PHASE_RECORD),
            smoozy_runtime::written_form::write(&record).expect("the record writes"),
        )
        .expect("the phase record is written");

        if let Ok(mut standing) = THE_AREAS_STANDING.lock() {
            standing.push(at.display().to_string());
        }
        APackageArea {
            at,
            installation,
            supervisor: None,
        }
    }

    fn pointer(&self) -> String {
        the_active_package_pointer_at(&self.at.display().to_string())
    }

    fn endpoint(&self) -> String {
        String::from_utf8(
            std::fs::read(the_record_at(
                &self.at.display().to_string(),
                records::THE_ENDPOINTS_RECORDED_PATH,
            ))
            .expect("the record is there"),
        )
        .expect("it is text")
    }

    /// Start the supervisor **through the pointer**, which is what the boot
    /// registration does, and wait until it is answering.
    fn start_the_supervisor(&mut self) {
        self.spawn_the_supervisor();
        self.waited_until_it_answers();
    }

    /// Put the process there and wait for nothing at all.
    fn spawn_the_supervisor(&mut self) {
        self.spawn_the_supervisor_with_path(None);
    }

    fn spawn_the_supervisor_with_path(&mut self, path: Option<&std::ffi::OsStr>) {
        let mut command = Command::new(self.pointer());
        if let Some(path) = path {
            command.env("PATH", path);
        }
        self.supervisor = Some(
            command
                .arg(THE_SUPERVISOR_WORD)
                .stdout(Stdio::null())
                .stderr(Stdio::from(
                    std::fs::File::create(self.at.join("complaints")).expect("a file for them"),
                ))
                .spawn()
                .expect("the supervisor starts"),
        );
    }

    /// **The gate is a framed answer and never a connection**, and that is the
    /// whole of the repair of 2026-09-07.
    ///
    /// **This waited on `connect` succeeding, and `connect` succeeds at the
    /// instant of the bind** — which is the first act of the start and comes
    /// before the two stores are made, because Z12's interlock has to settle
    /// before S3 will let a start touch a store at all. So every test that
    /// started a supervisor and then read the package area was reading it
    /// through a window in which the supervisor was bound and had made nothing:
    /// `a_supervisor_makes_the_two_record_stores_at_its_start` failed seven
    /// times in twenty-six whole-binary runs, every one of them while a compile
    /// was running beside it, and nobody could make it fail on demand.
    ///
    /// **A connection promises exactly one thing and it is not readiness.** Z12
    /// makes an accepted connection mean *another supervisor holds this
    /// endpoint*, which is true from the bind; `Server protocol and transfers`
    /// C2 correlates a result to a request by identity, and G25's own health
    /// exchange waits for one. **So the gate is G12's own request answered**,
    /// which cannot come back before the loop has been entered — and the loop is
    /// entered with the start's own value.
    fn waited_until_it_answers(&self) {
        let asked = TheEndpointAsACaller::at(&self.endpoint(), HOW_LONG_ONE_GATE_EXCHANGE_IS_GIVEN)
            .ask_waiting_for_a_supervisor(&a_health_request(), HOW_LONG_THE_SUPERVISOR_IS_GIVEN)
            .expect("the endpoint is reachable at all");
        assert!(
            asked.is_some(),
            "the supervisor never answered on {}: {}",
            self.endpoint(),
            String::from_utf8_lossy(&std::fs::read(self.at.join("complaints")).unwrap_or_default())
        );
    }

    /// Where one of the supervisor's own stores sits under this area.
    fn where_a_store_sits(&self, named: &str) -> PathBuf {
        PathBuf::from(the_record_at(&self.at.display().to_string(), named))
    }

    /// **Whether the two record stores hold anything at all under one
    /// identity**, read off the area rather than through a request — because
    /// what is being asked is whether a file stands, not what C8 answers.
    ///
    /// The records are named for their identity with a suffix saying which
    /// store they belong to, so the identity's own text is the prefix to look
    /// for.
    fn holds_a_record_under(&self, operation: &Identity) -> bool {
        let area = self.where_a_store_sits(
            smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
        );
        let Ok(entries) = std::fs::read_dir(&area) else {
            return false;
        };
        entries.filter_map(|entry| entry.ok()).any(|entry| {
            entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.starts_with(&operation.as_text()))
        })
    }

    /// **Killed, not asked to stop.** What is on the device is what the last
    /// durable write left and nothing more — which is the machine
    /// `State, identity and recovery` §6 exists for.
    fn kill_the_supervisor(&mut self) {
        let mut killed = self.supervisor.take().expect("it is running");
        killed.kill().expect("the supervisor is killed");
        killed.wait().expect("it is reaped");
    }
}

/// G12's request, freshly identified.
fn a_health_request() -> Request {
    Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        smoozy_runtime::acting::package::THE_HEALTH_ANSWER_REQUEST,
    )
}

/// **One framed request over the real endpoint, through the real adapter**, and
/// the correlated result back.
///
/// **This is the whole route a desktop request takes on the machine**: the
/// entry point invoked with no operand is the SSH command adapter, it reads one
/// framed request on its standard input, passes it to the supervisor over the
/// machine-local endpoint, and writes the correlated result on its standard
/// output.
fn asked_through_the_adapter(
    area: &APackageArea,
    request: &Request,
) -> smoozy_framing::framing::Answered {
    let mut adapter = Command::new(area.pointer())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the adapter starts");
    write_one_frame(
        adapter.stdin.as_mut().expect("it has a standard input"),
        &write_request_frame(request),
    )
    .expect("the request goes in");
    drop(adapter.stdin.take());
    let said = ended_within(adapter, "the adapter's answer, carried from the supervisor");
    assert!(
        said.status.success(),
        "the adapter ended at {}: {}",
        said.status,
        String::from_utf8_lossy(&said.stderr)
    );
    let answered = read_one_result(&mut said.stdout.as_slice()).expect("one framed result");
    assert!(
        answered.answers(request),
        "`Server protocol and transfers` C2: correlated by identity and by nothing else"
    );
    answered
}

/// **How long this test gives one asking through the adapter.** A test's own
/// bound and nothing of the plan's.
///
/// **Without it the bound would be the adapter's own, and that is an hour**
/// (`HOW_LONG_THE_ADAPTER_WAITS_FOR_THE_SUPERVISOR`), which is right for a
/// command on a machine nobody is watching and is no bound at all for a test: a
/// supervisor that took the caller and never answered would hold the run for
/// an hour and say nothing. An answer here takes well under a second.
const HOW_LONG_ONE_ASKING_IS_GIVEN: Duration = Duration::from_secs(60);

/// **The adapter's ending and everything it said, inside
/// [`HOW_LONG_ONE_ASKING_IS_GIVEN`]** — or the adapter is killed and the test
/// fails naming what it was waiting for. Added 2026-09-21, after hosted runs
/// `35595657437` and `35601023619` sat three and a half hours on two tests
/// elsewhere that waited with no deadline.
fn ended_within(adapter: Child, what: &str) -> std::process::Output {
    let identifier = adapter.id();
    let (sent, heard) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = sent.send(adapter.wait_with_output());
    });
    match heard.recv_timeout(HOW_LONG_ONE_ASKING_IS_GIVEN) {
        Ok(ended) => ended.expect("the adapter ends"),
        Err(_) => {
            let _ = Command::new("kill")
                .args(["-KILL", &identifier.to_string()])
                .status();
            panic!("DEADLINE MISSED: {what}, still waiting after {HOW_LONG_ONE_ASKING_IS_GIVEN:?}")
        }
    }
}

/// **Every package area this binary has made and not yet dropped**, so that
/// [`TheTestsDeadline`], which ends the binary without running any drop, can
/// still end what each one started.
static THE_AREAS_STANDING: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

/// **Every program started through one package area, ended.** Added
/// 2026-09-29, step 10c's dependency wave piece 8, when two
/// `smoozy-server work file-inspection` processes this file had left were found
/// running after a sitting.
///
/// **The supervisor's own death does not reach them.** A worker is started
/// through the area's pointer and outlives the supervisor that started it; a
/// worker whose supervisor is gone repeats its report for ever (`Rented-server
/// runtime` W12, *"re-attempted until it is accepted"*), and one whose Blender
/// never ends waits on it for ever. **What they all carry is the area's own
/// path on their command line** — the pointer for a worker, the stand-in's own
/// path for a Blender a test put under the area — so that is what is matched.
/// **Asked again until nothing is left**, because a worker may start its
/// Blender between one listing and the kill.
fn end_every_program_started_under(area: &str) {
    for _ in 0..50 {
        let _ = Command::new("pkill").args(["-KILL", "-f", area]).status();
        let still = Command::new("pgrep")
            .args(["-f", area])
            .output()
            .map(|listed| !listed.stdout.is_empty())
            .unwrap_or(false);
        if !still {
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

impl Drop for APackageArea {
    fn drop(&mut self) {
        if let Some(supervisor) = self.supervisor.as_mut() {
            supervisor.kill().ok();
            supervisor.wait().ok();
        }
        // **Then every program started through this area**, which the
        // supervisor's own death does not reach. Added 2026-09-29; see
        // [`end_every_program_started_under`].
        let started_under = self.at.display().to_string();
        end_every_program_started_under(&started_under);
        if let Ok(mut standing) = THE_AREAS_STANDING.lock() {
            standing.retain(|area| *area != started_under);
        }
        std::fs::remove_file(self.endpoint()).ok();
        std::fs::remove_dir_all(&self.at).ok();
    }
}

/// **The whole of `The shape`'s class 2, run: the adapter reads one framed
/// request on its standard input, passes it to the supervisor, and writes the
/// correlated result on its standard output.**
///
/// **The request asked is G12's**, because it is the one this plan defines and
/// the one whose four things the supervisor answers.
#[test]
fn the_adapter_carries_the_health_request_to_the_supervisor_and_the_result_back() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();

    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        smoozy_runtime::acting::package::THE_HEALTH_ANSWER_REQUEST,
    );

    // **The command channel's own constant command**: the entry point's path,
    // with nothing after it, and every value on its standard input.
    let mut adapter = Command::new(area.pointer())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the adapter starts");
    write_one_frame(
        adapter.stdin.as_mut().expect("it has a standard input"),
        &write_request_frame(&request),
    )
    .expect("the request goes in");
    drop(adapter.stdin.take());

    let mut said = Vec::new();
    adapter
        .stdout
        .as_mut()
        .expect("it has a standard output")
        .read_to_end(&mut said)
        .expect("the result comes back");
    let mut why = String::new();
    adapter
        .stderr
        .as_mut()
        .expect("it has a complaints stream")
        .read_to_string(&mut why)
        .ok();
    let ended = adapter.wait().expect("the adapter ends");
    assert!(ended.success(), "the adapter ended at {ended}: {why}");

    let answered = read_one_result(&mut said.as_slice()).expect("one framed result");
    assert!(
        answered.answers(&request),
        "`Server protocol and transfers` C2: correlated by identity and by nothing else"
    );

    let health = smoozy_runtime::acting::package::the_health_answer_in(&answered)
        .expect("all four of G12's things");
    assert_eq!(health.package_version, env!("CARGO_PKG_VERSION"));
    assert_eq!(health.installation, area.installation);
    assert_eq!(health.phase, Phase::Inspecting);
    assert_eq!(health.run.kind(), IdentityKind::Run);
}

/// **A supervisor makes the two record stores `State, identity and recovery`
/// §1.2 gives it, and it does so at the start rather than at the first write.**
///
/// §1.3 rule 1 makes the supervisor the only writer of both, so a package area
/// it cannot keep records in is a fault of this run and not of the first request
/// that needs one. **What is proved here is that a real supervisor makes the
/// area** — the store's own rules are exercised at the desk in that module's
/// tests, and there is no request that writes a record yet, which is what
/// `smoozy_runtime::WHAT_A_SUPERVISOR_STILL_OWES` says on this machine's
/// complaints stream at every start.
#[test]
fn a_supervisor_makes_the_two_record_stores_at_its_start() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    let records = std::path::Path::new(&area.at)
        .join(smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN);
    assert!(
        !records.exists(),
        "the area is the supervisor's to make and no placement made it"
    );

    area.start_the_supervisor();
    assert!(
        records.is_dir(),
        "the supervisor made no home for the operation records at {}",
        records.display()
    );

    // **And it is empty**, because nothing on this machine takes a request as
    // an operation yet.
    assert_eq!(
        std::fs::read_dir(&records)
            .expect("the area is readable")
            .count(),
        0
    );
}

/// **A caller is answered only by a supervisor that has finished starting**, and
/// this asks at the first instant a connection is possible so that the claim is
/// made where it is not obvious.
///
/// # The window this is about, and why the order above it is right
///
/// **The bind comes first and it has to.** Z12's interlock is *reach the
/// endpoint as a caller*, because a stale object left by a crashed run and a live
/// endpoint refuse a bind alike; and S3 makes a start that finds another
/// supervisor holding it end **without touching a store**. A start that made its
/// stores before the interlock settled would be a second supervisor writing into
/// the first one's package area, which is exactly what S3 forbids. **So the
/// stores cannot be made before the bind, and the window between the two is not
/// a mistake in the ordering.**
///
/// **What is inside the window is a queued connection and never an answer**, and
/// that rests on two published facts rather than on reasoning.
///
/// **The bind is also a listen.** `UnixListener::bind` performs `bind(2)` and
/// then `listen(2)` with a backlog in one call — read in the toolchain's own
/// shipped source at
/// `~/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/os/unix/net/listener.rs`,
/// rustc 1.98.0, 2026-09-07. So the object is accepting connections from the
/// instant of the bind, which is why Z12's probe works at all.
///
/// **A connection nobody has accepted yet waits in a queue.** The desk's own
/// `listen(2)` manual: *"The backlog parameter defines the maximum length for
/// the queue of pending connections"*, and *"the connections are accepted with
/// accept(2)"* — `man 2 listen`, macOS, read 2026-09-07. The supervisor's accept
/// is inside `answer_one_caller`, which is inside the turn loop, which is entered
/// with the start's own value — so the first byte of a request is read by a
/// supervisor that has already made its stores.
///
/// **The two ways out of the window are both honest.** Where the start fails
/// after the bind the process ends, the connection closes, and the caller meets
/// V4's *did not answer*, which is the reading the automatic repair trigger
/// takes. And where the queue is full the same manual says the caller *"may
/// receive an error with an indication of ECONNREFUSED"* — which the caller
/// reads as nothing holding the endpoint, and is V4 again. **Neither of them is
/// an answer composed by a supervisor that had not finished starting**, which is
/// the only thing that would be wrong.
///
/// **This test connects at the earliest instant it can and asserts the promise
/// on the far side of it.** Whether the window was open at that instant is not
/// asserted, because nothing can hold it open on demand — that is why the flake
/// it caused took twenty-six whole-binary runs to catch — but it is carried into
/// the failure message so a reader of a failure knows which case it saw.
#[test]
fn a_caller_is_answered_only_by_a_supervisor_that_has_finished_starting() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    let records = area.where_a_store_sits(
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
    );
    let journal =
        area.where_a_store_sits(smoozy_runtime::acting::journal::THE_AREA_THE_JOURNAL_SITS_IN);
    assert!(
        !records.exists() && !journal.exists(),
        "both stores are the supervisor's to make and no placement made either"
    );

    area.spawn_the_supervisor();

    // **The first instant a connection is possible is the instant of the bind**,
    // and the bind is the first thing the start does. Nothing pauses between two
    // attempts, so this is as early as a caller can be.
    let endpoint = area.endpoint();
    let waited_from = Instant::now();
    let mut caller = loop {
        match std::os::unix::net::UnixStream::connect(&endpoint) {
            Ok(caller) => break caller,
            Err(_) => assert!(
                waited_from.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN,
                "the supervisor never bound {endpoint}: {}",
                String::from_utf8_lossy(
                    &std::fs::read(area.at.join("complaints")).unwrap_or_default()
                )
            ),
        }
    };
    let the_window_was_open = !records.is_dir() || !journal.is_dir();

    let request = a_health_request();
    caller
        .set_read_timeout(Some(HOW_LONG_THE_SUPERVISOR_IS_GIVEN))
        .expect("the deadline is set");
    write_one_frame(&mut caller, &write_request_frame(&request)).expect("the request goes over");
    let answered = read_one_result(&mut caller).expect("one framed result");
    assert!(
        answered.answers(&request),
        "`Server protocol and transfers` C2"
    );

    assert!(
        records.is_dir() && journal.is_dir(),
        "a caller was answered by a supervisor that had not made its stores. The connection was \
         taken with the window {}",
        if the_window_was_open {
            "open"
        } else {
            "already shut"
        }
    );
}

/// **`State, identity and recovery` §1.2's journal row: the supervisor makes the
/// event journal, and it makes it at the start.** §1.3 rule 1 is why it is the
/// supervisor and nobody else.
///
/// **And a run that was killed leaves a journal the next run carries on from.**
/// The first supervisor is killed outright rather than asked to stop — nothing
/// is closed, flushed or tidied on the way out — and the second one starts over
/// the same area, opens what the first left, and answers.
///
/// # What this cannot prove yet, said rather than implied
///
/// **No entry is in the journal, because this program journals exactly one thing
/// and it cannot be caused at this desk.** Z9's refusal is the only event the
/// supervisor journals today — every other journalled thing in these plans
/// belongs to reconciliation, to a hold, a release or a drop, and none of that is
/// built — and Z9 fires only for **a caller whose login is not the supervisor's
/// own**. A test at this desk runs as the same login the supervisor does, so it
/// cannot pose one, and posing one would want a second user account on the
/// machine. **So the sequence-across-a-kill half of §4's proof is the store's own
/// and is at `App/Rented-server runtime/src/acting/journal.rs`**, where a journal
/// written by one holder is read by another that did not write it, the numbers
/// carry on and nothing repeats. **What is added here is the wiring and the
/// restart**, which that test cannot reach.
///
/// **It becomes provable here with reconciliation**, the next piece of this step:
/// S8 requires a disposition journalled every time it is written and
/// `State, identity and recovery` §6 runs at every start, so a plain start will
/// journal, and this test grows a kill and a second start around it with nothing
/// new invented.
#[test]
fn a_supervisor_makes_the_event_journal_at_its_start_and_a_kill_leaves_one_the_next_start_reads() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    let journal =
        area.where_a_store_sits(smoozy_runtime::acting::journal::THE_AREA_THE_JOURNAL_SITS_IN);
    assert!(
        !journal.exists(),
        "the journal is the supervisor's to make and no placement made it"
    );

    area.start_the_supervisor();
    assert!(
        journal.is_dir(),
        "the supervisor made no home for the event journal at {}",
        journal.display()
    );

    // **Killed, not asked to stop.** What is on the device is what the last
    // durable append left and nothing more.
    let mut killed = area.supervisor.take().expect("it is running");
    killed.kill().expect("the supervisor is killed");
    killed.wait().expect("it is reaped");
    // The killed run's endpoint object outlives it, which is Z1 and is the case
    // the next start's interlock exists for.
    let what_the_kill_left: Vec<PathBuf> = std::fs::read_dir(&journal)
        .expect("the journal's area is readable")
        .map(|entry| entry.expect("an entry").path())
        .collect();

    area.start_the_supervisor();
    assert!(
        journal.is_dir(),
        "the second start did not keep the journal the first one left"
    );
    let what_the_second_start_found: Vec<PathBuf> = std::fs::read_dir(&journal)
        .expect("the journal's area is readable")
        .map(|entry| entry.expect("an entry").path())
        .collect();
    assert_eq!(
        what_the_second_start_found, what_the_kill_left,
        "the second start neither removed nor added to what the killed run left"
    );
}

/// **S3: a second supervisor never runs.**
///
/// A start that finds another supervisor holding the endpoint ends without
/// touching a store and without signalling anything — which is an ordinary ending
/// rather than a failure, so the second process ends at zero.
#[test]
fn a_second_supervisor_ends_without_touching_anything() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();

    let second = Command::new(area.pointer())
        .arg(THE_SUPERVISOR_WORD)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("the second start runs");
    assert!(
        second.status.success(),
        "S3's ending is not a failure: {}",
        String::from_utf8_lossy(&second.stderr)
    );

    // And the first is still holding it.
    assert!(std::os::unix::net::UnixStream::connect(area.endpoint()).is_ok());
}

/// **The adapter says so plainly where the supervisor did not answer**, and it
/// writes no frame at all — an invented result would be one the desktop
/// correlated and believed.
#[test]
fn an_adapter_with_no_supervisor_writes_no_frame_and_says_why() {
    let _deadline = TheTestsDeadline::started();
    let area = APackageArea::made();

    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        smoozy_runtime::acting::package::THE_HEALTH_ANSWER_REQUEST,
    );
    let mut adapter = Command::new(area.pointer())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the adapter starts");
    write_one_frame(
        adapter.stdin.as_mut().expect("it has a standard input"),
        &write_request_frame(&request),
    )
    .expect("the request goes in");
    drop(adapter.stdin.take());

    let said = ended_within(adapter, "the adapter ending with no supervisor to ask");
    assert!(!said.status.success());
    assert!(said.stdout.is_empty(), "no frame is written");
    assert!(String::from_utf8_lossy(&said.stderr).contains("did not answer"));
}

/// **`Server protocol and transfers` §1.1's copy on the machine is answered by
/// the adapter itself, with no supervisor running at all** — the paragraph of
/// 2026-10-02: *"answered in the invocation that carries it and is never
/// passed to the supervisor"*. The file lands whole at the staging path the
/// request names and nowhere else, and nothing is published. Step 10c's Files'
/// owed acts piece 7.
/// **`Server protocol and transfers` §1.1's four values of files, through the
/// real program's real adapter with no supervisor running**, one bad path among
/// the good. Built 2026-11-04.
#[cfg(unix)]
#[test]
fn the_four_values_of_files_are_answered_by_the_adapter_with_no_supervisor_running() {
    use smoozy_framing::requests as asked;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::MetadataExt;
    let _deadline = TheTestsDeadline::started();
    let area = APackageArea::made();
    let folder = area.at.join("values-here");
    std::fs::create_dir_all(&folder).expect("made");
    let there = folder.join("wood.png");
    std::fs::write(&there, b"four bytes").expect("written");
    let paths = vec![
        there.as_os_str().as_bytes().to_vec(),
        folder.join("gone.png").as_os_str().as_bytes().to_vec(),
        there.as_os_str().as_bytes().to_vec(),
    ];
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        asked::THE_FOUR_VALUES_OF_FILES_REQUEST,
    )
    .carrying(
        smoozy_framing::framing::Fields::none()
            .and(asked::THE_PATHS_READ_FIELD, asked::the_paths_written(&paths)),
    );
    let answered = asked_through_the_adapter(&area, &request);
    let answers = asked::the_values_read_back(
        answered.fields.text(asked::THE_VALUES_READ_FIELD).expect("the values"),
        3,
    )
    .expect("one answer for each path");
    let seen = std::fs::metadata(&there).expect("there");
    let values = asked::OnePathAnswered::Values {
        size: 10,
        inode: seen.ino(),
        modification_time: seen.mtime(),
        change_time: seen.ctime(),
    };
    assert_eq!(
        answers,
        vec![values.clone(), asked::OnePathAnswered::NothingThere, values]
    );
}

#[test]
fn the_copy_on_the_machine_is_answered_by_the_adapter_with_no_supervisor_running() {
    use std::os::unix::ffi::OsStrExt;
    let _deadline = TheTestsDeadline::started();
    let area = APackageArea::made();
    let folder = area.at.join("copy-here");
    std::fs::create_dir_all(&folder).expect("made");
    let source = folder.join("wood.png");
    let bytes: Vec<u8> = (0..300_000u32).map(|n| (n % 253) as u8).collect();
    std::fs::write(&source, &bytes).expect("written");
    let staging = folder.join("staged-copy");

    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        smoozy_framing::requests::THE_FILE_IS_COPIED_ON_THE_MACHINE_REQUEST,
    )
    .carrying(
        smoozy_framing::framing::Fields::none()
            .and(
                smoozy_framing::requests::THE_COPYS_SOURCE_FIELD,
                source.as_os_str().as_bytes(),
            )
            .and(
                smoozy_framing::requests::THE_COPYS_STAGING_PATH_FIELD,
                staging.as_os_str().as_bytes(),
            ),
    );
    let answered = asked_through_the_adapter(&area, &request);
    assert_eq!(
        answered
            .fields
            .text(smoozy_framing::requests::THE_COPYS_OUTCOME_FIELD),
        Some(smoozy_framing::requests::IT_WAS_COPIED),
        "{answered:?}"
    );
    assert_eq!(
        std::fs::read(&staging).expect("the copy sits at the staging path"),
        bytes
    );
    assert_eq!(std::fs::read(&source).expect("the source"), bytes);
    assert_eq!(
        std::fs::read_dir(&folder).expect("reads").count(),
        2,
        "nothing else was made"
    );
}

/// **A worker of every kind is refused by name, and the refusal names the
/// document that owes the work.**
#[test]
fn a_worker_is_refused_and_names_who_owes_it() {
    let _deadline = TheTestsDeadline::started();
    let area = APackageArea::made();
    for kind in smoozy_runtime::workers::WorkerKind::ALL {
        // **The render kind is not refused** from step 10c piece 5's second
        // sitting: it is `Rendering`'s render worker, and with no supervisor
        // answering it has no work and ends saying so — which still names
        // `Rendering`, and is asserted below with the rest.
        let said = Command::new(area.pointer())
            .arg(smoozy_runtime::the_program::THE_WORKER_WORD)
            .arg(smoozy_runtime::the_program::the_word_for(kind))
            .output()
            .expect("it runs");
        assert!(!said.status.success());
        let why = String::from_utf8_lossy(&said.stderr);
        for owner in kind.declarations_owned_by() {
            assert!(why.contains(owner), "{why}");
        }
    }
}

/// **§4's dispatch, over the real endpoint: every name this package serves is
/// answered and every other is refused by name.**
///
/// **Each new name is one more case in the exchange that already worked**, which
/// is what the reading behind this piece asked for. What it asserts of each is
/// the thing the refusal would break: that the result does **not** carry
/// `Server protocol and transfers`' reserved field for *a request this package
/// does not answer*.
///
/// **And the negative is asserted beside them**, so the check is worth
/// something: a name no document gives this package still comes back refused.
#[test]
fn every_name_this_package_serves_is_answered_and_every_other_is_refused_by_name() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();

    for name in [
        smoozy_runtime::acting::package::THE_HEALTH_ANSWER_REQUEST,
        smoozy_framing::framing::THE_READINESS_REQUEST,
        smoozy_runtime::acting::readings::THE_HEALTH_READINGS_REQUEST,
    ] {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            name,
        );
        let answered = asked_through_the_adapter(&area, &request);
        assert_eq!(
            answered.the_request_name_it_says_it_does_not_answer(),
            None,
            "{name} is a name this package serves"
        );
    }

    let refused = asked_through_the_adapter(
        &area,
        &Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            "a request no document gives this package",
        ),
    );
    assert_eq!(
        refused.the_request_name_it_says_it_does_not_answer(),
        Some("a request no document gives this package")
    );
}

/// **U9's answer to a hold, over the real endpoint, against a real supervisor.**
///
/// A hold is placed in the package area the way U1's first origin places one,
/// the answer goes over SSH's own route — the adapter and the machine-local
/// endpoint — and **U12's single write is what releases the subject**, read back
/// off the disk afterwards.
///
/// **U13 and U14 ride with it**: the second answer acts on nothing and is told
/// which answer was applied.
#[test]
fn a_hold_is_answered_over_the_endpoint_and_the_release_is_written_once() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();

    let holds = smoozy_runtime::acting::stores::TheHoldRecordsOnTheMachine::in_the_area(
        &area.where_a_store_sits(
            smoozy_runtime::the_program::records::THE_AREA_THE_HOLD_RECORDS_SIT_IN,
        ),
    )
    .expect("the area is there");
    let hold = Identity::issue(IdentityKind::Operation).expect("randomness");
    smoozy_runtime::holds::KeepsHoldRecords::place(
        &holds,
        &smoozy_runtime::holds::HoldRecord {
            hold: hold.clone(),
            subject: smoozy_runtime::holds::Subject::Job(
                Identity::issue(IdentityKind::Job).expect("randomness"),
            ),
            reason: "the first-frame check".to_string(),
            origin: smoozy_runtime::holds::Origin::Submission,
            dependency: None,
            release: None,
        },
    )
    .expect("the hold is placed");

    let yes = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        smoozy_runtime::acting::stores::THE_ANSWER_TO_A_HOLD_REQUEST,
    )
    .carrying(
        smoozy_framing::framing::Fields::none()
            .and(
                smoozy_runtime::acting::stores::THE_HOLD_FIELD,
                hold.as_text(),
            )
            .and(
                smoozy_runtime::acting::stores::THE_ANSWER_FIELD,
                smoozy_runtime::acting::stores::THE_ANSWER_IS_YES,
            )
            .and(
                smoozy_runtime::acting::stores::THE_ROUTE_FIELD,
                smoozy_runtime::acting::stores::THE_ROUTE_IS_THE_DESK,
            ),
    );
    let answered = asked_through_the_adapter(&area, &yes);
    assert_eq!(
        answered
            .fields
            .text(smoozy_runtime::acting::stores::THE_RELEASE_OUTCOME_FIELD),
        Some("released")
    );

    let written = smoozy_runtime::holds::KeepsHoldRecords::read(&holds, &hold)
        .expect("it reads")
        .expect("the record is there")
        .release
        .expect("U12's single write landed");
    assert_eq!(
        written.outcome,
        smoozy_runtime::holds::ReleaseOutcome::Released
    );

    // **U15 and U13**: a later *no* is not converted into a stop and acts on
    // nothing; it is told which answer was applied.
    let no = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        smoozy_runtime::acting::stores::THE_ANSWER_TO_A_HOLD_REQUEST,
    )
    .carrying(
        smoozy_framing::framing::Fields::none()
            .and(
                smoozy_runtime::acting::stores::THE_HOLD_FIELD,
                hold.as_text(),
            )
            .and(
                smoozy_runtime::acting::stores::THE_ANSWER_FIELD,
                smoozy_runtime::acting::stores::THE_ANSWER_IS_NO,
            )
            .and(
                smoozy_runtime::acting::stores::THE_ROUTE_FIELD,
                smoozy_runtime::acting::stores::THE_ROUTE_IS_THE_PHONE,
            ),
    );
    let late = asked_through_the_adapter(&area, &no);
    assert_eq!(
        late.fields
            .text(smoozy_runtime::acting::stores::THE_RELEASE_OUTCOME_FIELD),
        Some("released"),
        "U14: the one that lost is told which answer was applied"
    );
    assert_eq!(
        smoozy_runtime::holds::KeepsHoldRecords::read(&holds, &hold)
            .expect("it reads")
            .expect("the record is there")
            .release
            .expect("it is there")
            .at
            .taken_at(),
        written.at.taken_at(),
        "U12: written once and never rewritten"
    );
}

/// **P3's two acts, over the real endpoint**, and V9's report of the state on
/// the health answer between them.
#[test]
fn the_setup_state_is_set_and_cleared_over_the_endpoint_and_rides_the_health_answer() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let run = Identity::issue(IdentityKind::Operation).expect("randomness");

    let set = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
    )
    .carrying(smoozy_framing::framing::Fields::none().and(
        smoozy_runtime::acting::stores::THE_SETUP_RUN_FIELD,
        run.as_text(),
    ));
    let answered = asked_through_the_adapter(&area, &set);
    assert_eq!(
        answered
            .fields
            .text(smoozy_runtime::acting::stores::THE_SETUP_RUN_FIELD),
        Some(run.as_text().as_str())
    );

    // **V9: reported with every health answer**, so the first thing a
    // reconnecting Smoozy learns is that a setup is unfinished.
    let health = asked_through_the_adapter(&area, &a_health_request());
    assert_eq!(
        health
            .fields
            .text(smoozy_runtime::acting::readings::THE_SETUP_IN_PROGRESS_FIELD),
        Some(run.as_text().as_str())
    );

    let clear = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST,
    )
    .carrying(smoozy_framing::framing::Fields::none().and(
        smoozy_runtime::acting::stores::THE_SETUP_RUN_FIELD,
        run.as_text(),
    ));
    asked_through_the_adapter(&area, &clear);

    let after = asked_through_the_adapter(&area, &a_health_request());
    assert_eq!(
        after
            .fields
            .text(smoozy_runtime::acting::readings::THE_SETUP_IN_PROGRESS_FIELD),
        None,
        "the state is gone and the field says nothing rather than something"
    );
}

/// **W8's two acts on the role's desired state, over the real endpoint, against
/// a real supervisor — and it is the first request in this program's reach that
/// asks for a worker.**
///
/// **W7: the state is durable and machine-level**, so it is read off the disk
/// here rather than out of an answer. **W10: the supervisor starts the role
/// worker while the state is enabled and never resurrects it after it is
/// disabled** — and what the supervisor did is carried back to the caller rather
/// than left to be guessed at.
///
/// **The child that is started is this program in its Telegram worker mode**,
/// which is the bot itself from 2026-10-16. It refused by kind and ended
/// non-zero until then; **now it stays up** — this area holds no token file, so
/// it says so once and waits for the polite stop (`Telegram` F1's paragraph of
/// 2026-10-12) — and the disable meets a worker that is there to be stopped.
#[test]
fn the_roles_desired_state_is_written_over_the_endpoint_and_the_supervisor_acts_on_it() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let state = area
        .at
        .join(smoozy_runtime::the_program::records::THE_TELEGRAM_ROLE_STATE);
    assert!(!state.exists(), "W7: nothing has written the role on yet");

    let enabled = asked_through_the_adapter(
        &area,
        &Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_runtime::acting::stores::THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST,
        ),
    );
    assert!(
        state.is_file(),
        "W8's write is durable and is on the disk when the answer comes back"
    );
    let said = enabled
        .fields
        .text(smoozy_runtime::acting::stores::WHAT_THE_ROLE_WORKER_DID_FIELD)
        .expect("the caller is told what the supervisor did with the worker");
    assert!(
        said.starts_with("started") || said.contains("the kernel recorded nothing for it"),
        "W10: the supervisor starts it while the state is enabled — {said}"
    );

    let disabled = asked_through_the_adapter(
        &area,
        &Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_runtime::acting::stores::THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST,
        ),
    );
    assert!(!state.exists(), "the state is gone");
    let said = disabled
        .fields
        .text(smoozy_runtime::acting::stores::WHAT_THE_ROLE_WORKER_DID_FIELD)
        .expect("the caller is told what the supervisor did with the worker");
    // **Turning it off stops the bot** — `Telegram` section 19's polite signal
    // to the worker's own process, which this program issued to nothing until
    // 2026-09-12.
    //
    // **One answer is accepted on each kind of machine, and no longer four.**
    // Until 2026-10-16 the child ended the instant it started, so whether the
    // supervisor still held it at the disable was a race, and three answers
    // were all correct conduct (the step 10 map's section 8, item 11, and
    // `Audits/Build/Step 10a/The Linux Telegram stop — what it was.md`). **The
    // worker now stays up**, so:
    //
    // - **where the kernel publishes per-process records** — every rented
    //   machine and the Linux runner — a pair was taken, the worker is there, and
    //   the answer is the signalled one alone; and the worker is gone soon after,
    //   since nothing restarts a disabled role;
    // - **where no pair can be taken**, none is invented, so the one answer is
    //   that it was not signalled for that reason (the Telegram worker's map, §6
    //   item 2). The worker is ended with the area. *(This was the desk's half
    //   until 2026-10-24; the desk takes the pair since step 12's piece SC0b.)*
    if the_pair_of_this_process().is_some() {
        assert_eq!(
            said, "the role is disabled and the worker was signalled",
            "W10 and `Telegram` section 19: turning it off stops the bot"
        );
        let from = Instant::now();
        while !the_telegram_workers_running_under(&area).is_empty() {
            assert!(
                from.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN,
                "DEADLINE MISSED: the signalled worker is still running"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    } else {
        assert!(
            said.contains("was not signalled: no pair was taken"),
            "at the desk no pair is taken, and none is invented to aim a signal by — {said}"
        );
    }
}

/// **The processes running this area's program as the Telegram role worker**,
/// read off the process table by their command line, which is the pointer's
/// path with the kind's word after it (G7).
fn the_telegram_workers_running_under(area: &APackageArea) -> Vec<String> {
    let listed = Command::new("pgrep")
        .args(["-f", &format!("{} work telegram", area.pointer())])
        .output()
        .expect("pgrep runs");
    String::from_utf8_lossy(&listed.stdout)
        .lines()
        .map(str::to_string)
        .collect()
}

/// What the Telegram role worker says, once, where the area holds no token file.
const IT_SAYS_IT_CANNOT_WORK: &str = "the Telegram worker cannot work: there is no token file";

/// The supervisor's complaints stream, which its workers write to as well.
fn the_complaints_of(area: &APackageArea) -> String {
    String::from_utf8_lossy(&std::fs::read(area.at.join("complaints")).unwrap_or_default())
        .to_string()
}

/// **`Telegram` F1's paragraph of 2026-10-12, run: a role worker started where
/// there is no token file says so once and stays — and is not started again.**
/// Added 2026-10-16, the second piece of the Telegram worker's map.
///
/// **What it guards is the loop the map's second hole found**: a worker that
/// ended at once was started again on every turn of the supervisor and each
/// start journalled, until the journal dropped events the desktop had not
/// read. W10's pace now spaces such restarts out; **this is the other half —
/// the worker does not end at all on a condition it cannot fix.** So after
/// eight seconds, room for W10's pace to have started it three times over had
/// it ended, the same process is running, the condition was said once, and W11
/// journalled no ending. **Before this piece the mode refused by kind and
/// ended at once**, and none of that held.
#[test]
fn a_role_worker_with_no_token_file_says_so_once_and_stays_and_is_not_started_again() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    asked_through_the_adapter(
        &area,
        &Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_runtime::acting::stores::THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST,
        ),
    );
    let from = Instant::now();
    while !the_complaints_of(&area).contains(IT_SAYS_IT_CANNOT_WORK) {
        assert!(
            from.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN,
            "DEADLINE MISSED: the worker never said why it cannot work: {}",
            the_complaints_of(&area)
        );
        std::thread::sleep(Duration::from_millis(50));
    }
    let running = the_telegram_workers_running_under(&area);
    assert_eq!(running.len(), 1, "one role worker: {running:?}");

    std::thread::sleep(Duration::from_secs(8));
    assert_eq!(
        the_complaints_of(&area)
            .matches(IT_SAYS_IT_CANNOT_WORK)
            .count(),
        1,
        "said once: {}",
        the_complaints_of(&area)
    );
    assert_eq!(
        the_telegram_workers_running_under(&area),
        running,
        "the same worker, still running: it did not end and was not started again"
    );
    let endings: Vec<_> = every_event(&area)
        .into_iter()
        .filter(|(kind, _, _)| kind.contains("role worker ended"))
        .collect();
    assert!(endings.is_empty(), "W11 journalled no ending: {endings:?}");
}

/// **§5.3's paragraph after W7, run to its end: a role worker this supervisor
/// run did not start is told to end itself, and ends.** Added 2026-10-16, the
/// second piece of the Telegram worker's map.
///
/// The role is on and this run's own worker is up. **A second one is started
/// by hand**, as an older run's worker would be still running after a restart:
/// it asks its start, is refused, says so and ends cleanly — before it reads a
/// file — while this run's own goes on. **Before this piece the mode refused
/// by kind and ended non-zero**, having asked nothing.
#[test]
fn a_role_worker_this_run_did_not_start_is_told_to_end_itself_and_ends() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    asked_through_the_adapter(
        &area,
        &Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_runtime::acting::stores::THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST,
        ),
    );
    let from = Instant::now();
    while the_telegram_workers_running_under(&area).is_empty() {
        assert!(
            from.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN,
            "DEADLINE MISSED: this run's worker never started"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let this_runs = the_telegram_workers_running_under(&area);

    let another = Command::new(area.pointer())
        .args([smoozy_runtime::the_program::THE_WORKER_WORD, "telegram"])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("it starts");
    let ended = ended_within(
        another,
        "a role worker this run did not start, told to end itself",
    );
    let said = String::from_utf8_lossy(&ended.stderr);
    assert!(
        ended.status.success(),
        "it ends cleanly, at {}: {said}",
        ended.status
    );
    assert!(said.contains("was told to end itself"), "{said}");
    assert!(
        !said.contains("cannot work"),
        "it ended before it read anything: {said}"
    );
    assert_eq!(
        the_telegram_workers_running_under(&area),
        this_runs,
        "this run's own worker goes on"
    );
}

/// **`Telegram` F1 as narrowed 2026-10-16, run: a staying role worker of an
/// earlier run asks for its start once a minute, is refused by the next run,
/// and ends** — rather than idling beside the new run's worker until a reboot.
/// Added 2026-10-16, the third piece of the Telegram worker's map.
///
/// The role is on and there is no token file, so this run's worker stays. **The
/// supervisor is killed** — its worker goes on (W6) — **and started again**,
/// and the new run starts a worker of its own. **The earlier one is gone within
/// about a minute of that**, said so, and the new run's own goes on. **The
/// minute is the program's own figure, waited out rather than shortened**: no
/// seam was put into the program for it. Before this piece the staying worker
/// called nothing, so it was never refused and ran on.
#[test]
fn a_staying_role_worker_of_an_earlier_run_asks_for_its_start_and_ends_within_a_minute() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    asked_through_the_adapter(
        &area,
        &Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_runtime::acting::stores::THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST,
        ),
    );
    let from = Instant::now();
    while !the_complaints_of(&area).contains(IT_SAYS_IT_CANNOT_WORK) {
        assert!(
            from.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN,
            "DEADLINE MISSED: the worker never said why it cannot work: {}",
            the_complaints_of(&area)
        );
        std::thread::sleep(Duration::from_millis(50));
    }
    let the_earlier_runs = the_telegram_workers_running_under(&area);
    assert_eq!(
        the_earlier_runs.len(),
        1,
        "one role worker: {the_earlier_runs:?}"
    );

    area.kill_the_supervisor();
    area.start_the_supervisor();
    let restarted = Instant::now();
    let from = Instant::now();
    let this_runs = loop {
        let running = the_telegram_workers_running_under(&area);
        if let Some(new) = running.iter().find(|one| !the_earlier_runs.contains(one)) {
            break new.clone();
        }
        assert!(
            from.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN,
            "DEADLINE MISSED: the new run never started its own worker: {running:?}"
        );
        std::thread::sleep(Duration::from_millis(50));
    };

    // **About a minute**: the program's minute, and half as long again for a
    // loaded desk, from the moment the new run was up.
    let the_bound = Duration::from_secs(90);
    while the_telegram_workers_running_under(&area).contains(&the_earlier_runs[0]) {
        assert!(
            restarted.elapsed() < the_bound,
            "DEADLINE MISSED: the earlier run's staying worker was still running {:?} after the restart: {}",
            restarted.elapsed(),
            the_complaints_of(&area)
        );
        std::thread::sleep(Duration::from_millis(200));
    }
    assert!(
        the_complaints_of(&area).contains("was told to end itself"),
        "it said why it ended: {}",
        the_complaints_of(&area)
    );
    assert_eq!(
        the_telegram_workers_running_under(&area),
        vec![this_runs],
        "the new run's own worker goes on"
    );
}

/// **The Telegram worker's own six requests, by the names the program gives
/// them** in `src/the_telegram_worker.rs`. A program's own words cannot be
/// reached from here, so they are spelt again, and a test in the program
/// itself holds this file to its spelling.
const THE_TELEGRAM_WORKERS_START_REQUEST: &str = "the telegram worker's start";
const THE_TELEGRAM_WORKERS_OWN_REQUESTS: [&str; 8] = [
    THE_TELEGRAM_WORKERS_START_REQUEST,
    "a telegram worker's fact is written",
    "the telegram worker reads its journal forward",
    "the telegram worker's cursor moves forward",
    "an instruction's recorded outcome is read",
    "an instruction's outcome is recorded",
    THE_JOB_IS_SUBMITTED_AGAIN_REQUEST,
    // **`Telegram` I4's request**, the eighth, the map's piece 6.
    "a frame's picture is made again",
];
/// **`Telegram` Y1's request**, the seventh, spelt as the program spells it.
const THE_JOB_IS_SUBMITTED_AGAIN_REQUEST: &str = "a job is submitted again by this machine";
/// The refusal's field and its one value, spelt as the program spells them.
const THE_REFUSAL_FIELD: &str = "the telegram worker is refused";
const END_YOURSELF: &str = "end yourself";

/// **§5.3's paragraph after W7, over the real endpoint: a caller that is not
/// the role worker this run started is told to end itself**, for each of the
/// worker's six requests — on a machine whose role was never switched on, and
/// on one where it is on and a worker of this run has been started. Added
/// 2026-10-16, the first piece of the Telegram worker's map.
///
/// **The caller is the SSH command adapter**, the route every desktop request
/// takes: a process that is plainly not the role worker. **Before that piece
/// these six names were refused by name**, as names this package did not
/// answer at all.
#[test]
fn a_caller_that_is_not_the_role_worker_is_refused_the_workers_requests() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let every_one_is_refused = |area: &APackageArea, when: &str| {
        for name in THE_TELEGRAM_WORKERS_OWN_REQUESTS {
            let answered = asked_through_the_adapter(
                area,
                &Request::speaking(
                    THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                    Identity::issue(IdentityKind::Operation).expect("randomness"),
                    name,
                ),
            );
            assert_eq!(
                answered.fields.text(THE_REFUSAL_FIELD),
                Some(END_YOURSELF),
                "{name}, {when}: {answered:?}"
            );
        }
    };
    every_one_is_refused(&area, "with the role never switched on");

    asked_through_the_adapter(
        &area,
        &Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_runtime::acting::stores::THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST,
        ),
    );
    every_one_is_refused(&area, "with the role on and a worker of this run started");
}

/// **A role worker this test can make ask**, written over the active-package
/// pointer: the supervisor starts it as `work telegram` like any role worker,
/// it says which process it is in a file named for that process, and it waits
/// — bounded, a minute — for the test to tell it to ask. **Then it becomes the
/// SSH command adapter in the same process** (`exec`), with one framed request
/// on its standard input, so the process the supervisor reads off the
/// connection is the very process it started as its role worker.
///
/// Every other invocation is handed to the real program unchanged, from the
/// same folder, so the package area is the same area on every route.
fn a_role_worker_that_asks_when_told(area: &APackageArea) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let program = area.at.join("the-program");
    std::os::unix::fs::symlink(Path::new(env!("CARGO_BIN_EXE_smoozy-server")), &program)
        .expect("the program is linked beside the pointer");
    let workers = area.at.join("telegram-workers");
    std::fs::create_dir_all(&workers).expect("made");
    std::fs::write(
        area.at.join("telegram-request"),
        write_request_frame(&Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            THE_TELEGRAM_WORKERS_START_REQUEST,
        )),
    )
    .expect("the request is written");
    let pointer = area.pointer();
    std::fs::remove_file(&pointer).expect("the link is taken away");
    std::fs::write(
        &pointer,
        "#!/bin/sh\n\
         area=$(dirname \"$0\")\n\
         if [ \"$1\" = work ] && [ \"$2\" = telegram ]; then\n\
           echo $$ > \"$area/telegram-workers/$$\"\n\
           n=0\n\
           while [ ! -f \"$area/telegram-workers/$$.go\" ]; do\n\
             n=$((n+1)); [ \"$n\" -gt 1200 ] && exit 3\n\
             sleep 0.05\n\
           done\n\
           exec \"$area/the-program\" < \"$area/telegram-request\" > \"$area/telegram-workers/$$.answer\"\n\
         fi\n\
         exec \"$area/the-program\" \"$@\"\n",
    )
    .expect("the stand-in is written");
    std::fs::set_permissions(&pointer, std::fs::Permissions::from_mode(0o755))
        .expect("it is made runnable");
    workers
}

/// The processes the stand-in has said it is, in no order.
fn the_workers_that_said_who_they_are(workers: &Path) -> Vec<String> {
    std::fs::read_dir(workers)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
                .filter(|name| name.chars().all(|c| c.is_ascii_digit()))
                .collect()
        })
        .unwrap_or_default()
}

/// **One more role worker than `known`**, within the supervisor's bound.
fn a_new_role_worker(workers: &Path, known: &[String]) -> String {
    let from = Instant::now();
    while from.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN {
        if let Some(new) = the_workers_that_said_who_they_are(workers)
            .into_iter()
            .find(|said| !known.contains(said))
        {
            return new;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    panic!("DEADLINE MISSED: no new role worker was started");
}

/// **That worker told to ask, and the one framed answer it was given**,
/// within the supervisor's bound.
fn what_the_worker_was_answered(workers: &Path, worker: &str) -> smoozy_framing::framing::Answered {
    std::fs::write(workers.join(format!("{worker}.go")), b"").expect("told");
    let at = workers.join(format!("{worker}.answer"));
    let from = Instant::now();
    while from.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN {
        if let Ok(bytes) = std::fs::read(&at) {
            if let Ok(answered) = read_one_result(&mut bytes.as_slice()) {
                return answered;
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    panic!("DEADLINE MISSED: role worker {worker} was never answered");
}

/// **§5.3's paragraph after W7, over the real endpoint: a role worker an
/// earlier supervisor run started is told to end itself by the next run**, and
/// the role worker the next run started is answered. Added 2026-10-16, the
/// first piece of the Telegram worker's map.
///
/// **This is the case the paragraph exists for.** The supervisor is killed,
/// its role worker goes on running (W6), and the run the restarter brings up
/// starts a worker of its own — two bots on one machine. **The old one asks
/// the new run and is refused; the new one asks and is given its start.** So
/// the refusal is about which run started the caller, and nothing else.
#[test]
fn a_role_worker_of_an_earlier_run_is_refused_and_this_runs_is_answered() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    let workers = a_role_worker_that_asks_when_told(&area);
    area.start_the_supervisor();
    asked_through_the_adapter(
        &area,
        &Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_runtime::acting::stores::THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST,
        ),
    );
    let the_earlier_runs = a_new_role_worker(&workers, &[]);

    area.kill_the_supervisor();
    area.start_the_supervisor();
    let this_runs = a_new_role_worker(&workers, std::slice::from_ref(&the_earlier_runs));

    let refused = what_the_worker_was_answered(&workers, &the_earlier_runs);
    assert_eq!(
        refused.fields.text(THE_REFUSAL_FIELD),
        Some(END_YOURSELF),
        "the earlier run's worker is told to end itself: {refused:?}"
    );

    let answered = what_the_worker_was_answered(&workers, &this_runs);
    assert_eq!(
        answered.fields.text(THE_REFUSAL_FIELD),
        None,
        "this run's own worker is answered: {answered:?} — {}",
        String::from_utf8_lossy(&std::fs::read(area.at.join("complaints")).unwrap_or_default())
    );
    assert_eq!(
        answered.fields.text("routing key").map(str::len),
        Some(32),
        "its start carries the routing key the enable issued: {answered:?}"
    );
}

/// **W12's report, over the real endpoint.**
///
/// **A report from an earlier run is refused as stale**, because what would
/// accept one is this supervisor having written the *resumed* disposition for
/// that operation, and no record on a fresh machine carries one. **This note said
/// §6's reconciliation was not built until 2026-09-12**, and it is: the test
/// below this one drives the real binary through the loop and watches it write a
/// *resumed*. What is posed here is the refusing half, over the real endpoint.
#[test]
fn a_workers_report_from_an_earlier_run_is_refused_as_stale_over_the_endpoint() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();

    let answered = asked_through_the_adapter(
        &area,
        &Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_runtime::acting::workers::THE_WORKERS_REPORT_REQUEST,
        )
        .carrying(
            smoozy_framing::framing::Fields::none().and(
                smoozy_runtime::acting::workers::THE_RUN_THE_WORKER_WAS_STARTED_UNDER,
                Identity::issue(IdentityKind::Run)
                    .expect("randomness")
                    .as_text(),
            ),
        ),
    );
    assert_eq!(
        answered
            .fields
            .text(smoozy_runtime::acting::workers::WHETHER_THE_REPORT_WAS_ACCEPTED),
        Some(smoozy_runtime::acting::workers::IT_WAS_REFUSED_AS_STALE),
        "nothing on this machine writes resumed, so an earlier run's report is stale"
    );
}

/// **§14's one unit, with a real child that is this program's own worker mode.**
///
/// **This is the only place the worker start is driven against the real
/// binary.** The invocation is composed by the same function the supervisor's own
/// start uses, so what is started is the active-package pointer with the kind's
/// word after it — and the mode refuses by name, which is a real child process
/// started and reaped and is all this side of the start promises.
///
/// **X1's pair and X9a's wait are asserted in the order the clauses give them**:
/// the pair validates while the worker has not been waited on, the wait is what
/// takes the ending, and the pair then no longer validates — X9's *a group whose
/// leader's pair no longer validates has ended*.
///
/// **The pair half runs only where the kernel publishes per-process records** —
/// every rented machine, and the Linux runner. **The desk's own half is
/// asserted whatever happens**: a real child was started and it is in the wait,
/// and where no pair could be read none is invented.
#[test]
fn the_supervisors_own_start_runs_a_real_worker_of_this_program_and_reaps_it() {
    let _deadline = TheTestsDeadline::started();
    let area = APackageArea::made();
    let package_area = area.at.display().to_string();
    let records =
        smoozy_runtime::acting::operation_records::TheOperationRecordsOnTheMachine::in_the_area(
            &area.where_a_store_sits(
                smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
            ),
        )
        .expect("the store is made");
    let mut records = records;
    let wait = std::sync::Arc::new(
        smoozy_runtime::acting::execution_group::TheWaitTheSupervisorPerforms::new(),
    );
    let start = smoozy_runtime::acting::workers::TheWorkerStartOnTheMachine {
        kernel:
            smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt::the_rented_machine(
            ),
        wait: wait.clone(),
        how_it_is_invoked: smoozy_runtime::acting::starting::how_the_program_invokes_a_worker(
            &package_area,
        ),
    };
    let mut point = smoozy_runtime::acting::starting::TheAdmissionPoint::holding_nothing();
    let what = smoozy_runtime::acting::starting::WhatIsStarted {
        operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
        kind: smoozy_runtime::workers::WorkerKind::Render,
        subject: Identity::issue(IdentityKind::Job).expect("randomness"),
        under_run: Identity::issue(IdentityKind::Run).expect("randomness"),
        capture: None,
        the_kinds_own_field: None,
    };

    let produced = smoozy_runtime::acting::starting::start_one_worker(
        &what,
        &mut point,
        None,
        &mut records,
        &smoozy_runtime::acting::capture::TheSinksOnTheMachine,
        &start,
        wait.as_ref(),
    );
    assert_eq!(
        wait.how_many_it_holds(),
        1,
        "a real child of this program was started and is in X9a's wait: {produced:?}"
    );

    let kernel =
        smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt::the_rented_machine();
    let this_machine_publishes_records = the_kernel_here_answers_for_a_process();
    if !this_machine_publishes_records {
        // **No pair is invented where the kernel records nothing**, and the
        // start says so rather than answering with half of one — **but it says
        // the child started**, which it did. It said *not attempted* until
        // 2026-09-12, and a caller told that would have read a render running on
        // a paid machine as one that never began.
        assert!(
            matches!(
                produced,
                smoozy_runtime::acting::starting::WhatTheStartOfAWorkerProduces::ItStartedAndItsGroupCouldNotBeRecorded(None, _)
            ),
            "{produced:?}"
        );
        assert_eq!(
            point.how_many_it_holds(),
            1,
            "the running worker is admitted, so A5 and A5a decide against a machine that really \
             is holding its resources"
        );
        return;
    }

    let smoozy_runtime::acting::starting::WhatTheStartOfAWorkerProduces::ItStarted(pair) = produced
    else {
        panic!("the worker did not start: {produced:?}");
    };
    // **The pair validates while the worker has not been waited on.**
    assert_eq!(
        pair.validates_against(
            smoozy_runtime::execution_group::ValidatesTheRecordedPair::what_is_at(
                &kernel,
                pair.identifier
            )
            .expect("the kernel answers")
        ),
        smoozy_runtime::execution_group::Validation::Matches
    );

    // **X9a's wait is what takes the ending**, and a render worker with no
    // supervisor to ask what it was started for ends non-zero.
    let group = smoozy_runtime::execution_group::ExecutionGroup {
        kind: smoozy_runtime::workers::WorkerKind::Render,
        leader: pair,
    };
    let mut ending = None;
    let waited_from = Instant::now();
    while waited_from.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN {
        ending = smoozy_runtime::execution_group::WaitsOnEveryWorkerItStarts::the_ending(
            wait.as_ref(),
            &group,
        )
        .expect("the wait is taken");
        if ending.is_some() {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(
        ending,
        Some(1),
        "the render worker finds no supervisor answering at the endpoint, has no work, and ends \
         non-zero — which is the real child this start ran"
    );

    // **X9: a group whose leader's pair no longer validates has ended.**
    assert_eq!(
        pair.validates_against(
            smoozy_runtime::execution_group::ValidatesTheRecordedPair::what_is_at(
                &kernel,
                pair.identifier
            )
            .expect("the kernel answers")
        ),
        smoozy_runtime::execution_group::Validation::Gone
    );
}

/// **Whether the kernel of this machine answers for a process, through the
/// reader the program itself uses.** Every rented machine does, and the desk
/// does too since 2026-10-24 (step 12's piece SC0b, a development build's own
/// route on macOS); until then these tests asked whether `/proc/1/stat` existed,
/// or whether this was Linux.
fn the_kernel_here_answers_for_a_process() -> bool {
    matches!(
        smoozy_runtime::execution_group::ValidatesTheRecordedPair::what_is_at(
            &smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt::the_rented_machine(),
            std::process::id()
        ),
        Ok(Some(_))
    )
}

/// **An open operation record in the package area, written before the supervisor
/// starts**, through the store a supervisor writes through.
///
/// **The pair is at an identifier nothing is at**, so the recorded execution
/// group reads as gone — which is the side of `State, identity and recovery` §6's
/// table a machine that was killed mid-render comes back on.
fn seed_an_open_record(
    area: &APackageArea,
    identifier: u32,
    start_time: u64,
    stopped: bool,
    attempt_count: u32,
) -> Identity {
    let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area.where_a_store_sits(
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
    ))
    .expect("the store is made");
    let mut record = OperationRecord::opened(
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        Identity::issue(IdentityKind::Job).expect("randomness"),
        "render",
    );
    record.execution_group = Some(TheRecordedExecutionGroup {
        as_that_document_recorded_it: RecordedPair {
            identifier,
            start_time,
        }
        .as_this_document_records_it(),
        started_under_run: Identity::issue(IdentityKind::Run).expect("randomness"),
    });
    record.attempt_count = attempt_count;
    if stopped {
        record.the_stop_was_asked_for().expect("it is open");
    }
    let operation = record.operation.clone();
    store
        .record_before_acting(&record)
        .expect("the record is written");
    if stopped || attempt_count != 1 {
        store.write_it_back(&record).expect("the record is written");
    }
    operation
}

/// What the machine's own records hold for an operation, read off the disk.
fn what_the_machine_holds(
    area: &APackageArea,
    operation: &Identity,
) -> WhatTheMachineHoldsUnderAnIdentity {
    TheOperationRecordsOnTheMachine::in_the_area(&area.where_a_store_sits(
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
    ))
    .expect("the store is there")
    .what_is_held_under(operation)
    .expect("it reads")
}

/// An identifier the kernel never hands out, so the recorded pair reads as gone.
const AN_IDENTIFIER_NOTHING_IS_AT: u32 = u32::MAX;

/// **`State, identity and recovery` §6 over a real machine's records, by a real
/// supervisor started through the pointer** — one record per branch of the
/// table, and each disposition watched happening rather than argued about.
///
/// **This is what the whole piece exists for.** A machine killed mid-render used
/// to come back and do nothing about the render: the record stood open, nobody
/// closed it, and the next start would have re-run it on a machine being paid
/// for. Here three records are seeded before the supervisor exists, the
/// supervisor is started, and all three come back off the disk decided.
///
/// **Nothing is rented, nothing is bought and no account is touched.**
///
/// **One of the four branches is not here and it is *resumed***: it wants a
/// recorded group that still validates, which is the test below this one.
#[test]
fn a_supervisor_start_reconciles_every_open_record_one_branch_apiece() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();

    // **`State, identity and recovery` §5.3 step 2's durable *should be
    // stopped*, written before the machine went down.**
    let stopped = seed_an_open_record(&area, AN_IDENTIFIER_NOTHING_IS_AT, 1, true, 1);
    // **The first interruption**, which the table re-runs once.
    let first = seed_an_open_record(&area, AN_IDENTIFIER_NOTHING_IS_AT, 2, false, 1);
    // **The second**, which it gives up.
    let second = seed_an_open_record(&area, AN_IDENTIFIER_NOTHING_IS_AT, 3, false, 2);

    area.start_the_supervisor();

    // **Left stopped, with the recorded outcome written in the same act.**
    let held = what_the_machine_holds(&area, &stopped);
    assert_eq!(held.recorded_outcome(), Some(&RecordedOutcome::LeftStopped));
    assert_eq!(
        held.what_an_open_record_carries().map(|(which, _)| which),
        Some(Disposition::LeftStopped),
        "the disposition and the recorded outcome are written together"
    );

    // **Re-run once, with the count of 2 written with it, and the record still
    // open** — `State, identity and recovery` G3.
    let held = what_the_machine_holds(&area, &first);
    assert_eq!(
        held.what_an_open_record_carries(),
        Some((Disposition::ReRunOnce, 2))
    );
    assert_eq!(held.recorded_outcome(), None);

    // **Given up, with the recorded outcome.**
    let held = what_the_machine_holds(&area, &second);
    assert_eq!(held.recorded_outcome(), Some(&RecordedOutcome::GivenUp));

    // **The absence is loud**: what the pass did is on the machine's own
    // complaints stream, record by record.
    let said = String::from_utf8_lossy(
        &std::fs::read(area.at.join("complaints")).expect("the stream is there"),
    )
    .to_string();
    assert!(
        said.contains("reconciliation ran over 3 open operation records"),
        "{said}"
    );
    assert!(said.contains("never four empty sets"), "{said}");
}

/// **The *resumed* branch, against a group that genuinely still validates.**
///
/// **The live group is this test's own process**, which the kernel is publishing
/// a record for throughout — so what is posed is `State, identity and recovery`
/// §5.2's positive case with nothing invented.
///
/// **It runs only where the kernel answers for a process**, and the test says
/// so rather than passing quietly on a machine that could not have run it.
/// *(Corrected 2026-10-24, step 12's piece SC0b: a development build at the
/// desk now takes the pair from macOS's own process table, so the whole half
/// runs here too.)*
#[test]
fn a_group_that_still_validates_is_resumed_by_a_real_supervisor() {
    let _deadline = TheTestsDeadline::started();
    let Some(ours) = the_pair_of_this_process() else {
        // **The honest half, asserted rather than skipped.** Where the kernel
        // publishes no per-process record, every recorded group reads as gone —
        // `State, identity and recovery` §5.2's own safe direction — so what is
        // proved here is that nothing invents *resumed* on such a machine.
        let mut area = APackageArea::made();
        let seeded = seed_an_open_record(&area, 1, 1, false, 1);
        area.start_the_supervisor();
        assert_eq!(
            what_the_machine_holds(&area, &seeded)
                .what_an_open_record_carries()
                .map(|(which, _)| which),
            Some(Disposition::ReRunOnce),
            "a machine that publishes no per-process record reads every group as gone"
        );
        return;
    };
    let mut area = APackageArea::made();
    let alive = seed_an_open_record(&area, ours.identifier, ours.start_time, false, 1);
    area.start_the_supervisor();

    let held = what_the_machine_holds(&area, &alive);
    assert_eq!(
        held.what_an_open_record_carries(),
        Some((Disposition::Resumed, 1)),
        "a group that still validates is resumed and the count does not move"
    );
    assert_eq!(
        held.recorded_outcome(),
        None,
        "resumed closes nothing and the record stays eligible at the next run"
    );
}

/// This process's own pair, where the kernel publishes one.
fn the_pair_of_this_process() -> Option<RecordedPair> {
    smoozy_runtime::execution_group::ValidatesTheRecordedPair::what_is_at(
        &smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt::the_rented_machine(),
        std::process::id(),
    )
    .ok()
    .flatten()
}

/// **The second moment, over the real endpoint: clearing the setup-in-progress
/// state runs reconciliation.**
///
/// **This is Row 110's ordering from the machine's own side.** The state is set
/// before the supervisor starts, so the start's own pass is held back and the
/// record is untouched; the clear then arrives over the command path and the
/// record is decided. **Without the second moment the interrupted job would wait
/// for the next supervisor start.**
#[test]
fn clearing_the_setup_state_over_the_endpoint_reconciles_what_the_setup_held_back() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    let waiting = seed_an_open_record(&area, AN_IDENTIFIER_NOTHING_IS_AT, 4, false, 1);

    // **P4: set durably before the supervisor exists**, which is the machine
    // that comes back with a setup still unfinished.
    let run = Identity::issue(IdentityKind::Operation).expect("randomness");
    smoozy_runtime::setup_state::SetsAndClearsTheState::set(
        &smoozy_runtime::acting::stores::TheSetupStateOnTheMachine {
            at: area.where_a_store_sits(records::THE_SETUP_IN_PROGRESS_STATE),
        },
        &run,
    )
    .expect("the state is written");

    area.start_the_supervisor();

    // **P2: while it stands, nothing is decided.**
    assert_eq!(
        what_the_machine_holds(&area, &waiting).what_an_open_record_carries(),
        None,
        "the setup holds the whole loop back"
    );

    let clear = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST,
    )
    .carrying(smoozy_framing::framing::Fields::none().and(
        smoozy_runtime::acting::stores::THE_SETUP_RUN_FIELD,
        run.as_text(),
    ));
    asked_through_the_adapter(&area, &clear);

    assert_eq!(
        what_the_machine_holds(&area, &waiting).what_an_open_record_carries(),
        Some((Disposition::ReRunOnce, 2)),
        "P5's second act: the clear runs reconciliation immediately afterwards"
    );
}

/// **A machine killed mid-render comes back and decides**, which is the sentence
/// this whole piece exists to make true — and it is watched with the real binary
/// killed and started again over one package area.
///
/// **`State, identity and recovery` M5's own case**: the supervisor did not
/// initiate the boot, and it trusts no record to be more recent than the last one
/// it can read. **The re-run-once rule is what bounds the cost** — the first
/// return writes *re-run once* with the count of 2, the second gives up, and a
/// third finds a closed record and leaves it alone.
///
/// **Nothing re-runs, and the name says so.** §6 step 2 writes the disposition
/// and the count **before** the new attempt starts, and the attempt itself is the
/// kind's own document's, behind a wall this program may not reach — so what is
/// watched here is the two branches the record takes and never a second render.
#[test]
fn a_machine_killed_mid_render_writes_re_run_once_and_then_gives_up() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    let interrupted = seed_an_open_record(&area, AN_IDENTIFIER_NOTHING_IS_AT, 5, false, 1);

    area.start_the_supervisor();
    assert_eq!(
        what_the_machine_holds(&area, &interrupted).what_an_open_record_carries(),
        Some((Disposition::ReRunOnce, 2))
    );

    area.kill_the_supervisor();
    area.start_the_supervisor();
    let held = what_the_machine_holds(&area, &interrupted);
    assert_eq!(held.recorded_outcome(), Some(&RecordedOutcome::GivenUp));

    area.kill_the_supervisor();
    area.start_the_supervisor();
    assert_eq!(
        what_the_machine_holds(&area, &interrupted).recorded_outcome(),
        Some(&RecordedOutcome::GivenUp),
        "the recorded outcome is written once and never rewritten"
    );
}

// --- step 10c piece 5: the job arrives and can be read back ---

/// **`Job files and outputs` L1's root, recorded in the package area, at a
/// real folder this test owns** — the one a hand-over would record — and gone
/// with the value.
///
/// **It sits in the build's own temporary folder and not the system's**,
/// because `Job files and outputs` L3 refuses a root under a shared temporary
/// area, and on Linux the system's temporary folder is one.
struct ARoot {
    at: PathBuf,
}

impl ARoot {
    fn recorded_in(area: &APackageArea) -> ARoot {
        let at = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
            "smoozy-root-{}",
            Identity::issue(IdentityKind::Operation)
                .expect("randomness")
                .as_text()
        ));
        std::fs::create_dir_all(&at).expect("the root is made");
        std::fs::write(
            area.where_a_store_sits(smoozy_job_files_machine::layout::THE_RECORD_OF_THE_ROOT),
            at.display().to_string(),
        )
        .expect("the root is recorded");
        ARoot { at }
    }

    /// **A job's material, published** — its manifest where `Job files and
    /// outputs` M5's rename puts it, which is all the supervisor reads of it.
    fn a_published_job(&self) -> Identity {
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let root =
            smoozy_job_files_machine::layout::MachineRoot::chosen(&self.at.display().to_string())
                .expect("a root");
        let directory =
            smoozy_job_files_machine::publication::the_job_record_directory(&root, &job);
        std::fs::create_dir_all(&directory).expect("the record directory is made");
        std::fs::write(
            Path::new(&directory).join(smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME),
            smoozy_rendering_machine::composition::testing::a_manifest(&job, "5.2", "Scene")
                .as_written(),
        )
        .expect("the manifest is published");
        job
    }
}

impl Drop for ARoot {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.at).ok();
    }
}

/// **The line held back**, by `Rented-server runtime` P1's setup-in-progress
/// state set through the ordinary command path — so every job written stays
/// *waiting* and a reading reads a line nobody has started (P2, A11).
///
/// **Added at step 10c piece 5's second sitting, when the line began to run.**
/// A job written to a machine with nothing rendering is started straight away
/// from then on, so a test about what a waiting line reads back holds the line
/// first; the one about the line moving is below it.
fn the_line_is_held_by_a_setup_run(area: &APackageArea) -> Identity {
    let run = Identity::issue(IdentityKind::Operation).expect("randomness");
    let set = a_request_of(
        smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
        smoozy_framing::framing::Fields::none().and(
            smoozy_runtime::acting::stores::THE_SETUP_RUN_FIELD,
            run.as_text(),
        ),
    );
    asked_through_the_adapter(area, &set);
    run
}

/// One request of a name, carrying fields.
fn a_request_of(name: &str, fields: smoozy_framing::framing::Fields) -> Request {
    Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        name,
    )
    .carrying(fields)
}

/// **Step 10c piece 5, proved by name at a running supervisor: every request
/// it now answers sent through the real adapter, and each answer read.**
///
/// A job is written and read back by each reading; a second job arrives held
/// behind the first, with its hold record on the disk; the journal is read
/// forward and the desktop's cursor moved. **And the order is broken on
/// purpose**: with the note store made unwritable, a submission leaves no job
/// record behind and says so — a record written without its note would leave
/// the desktop's fill-in with nothing to read and nothing anywhere would fail.
#[test]
fn a_job_is_written_read_back_by_every_reading_and_journalled_after_its_record() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::framing::Fields;
    use smoozy_framing::requests as asked;

    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    area.start_the_supervisor();
    the_line_is_held_by_a_setup_run(&area);

    // **The job record is written**, with its label.
    let first = root.a_published_job();
    let submitted = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_JOB_RECORD_IS_WRITTEN_REQUEST,
            Fields::none()
                .and(asked::THE_JOB_FIELD, first.as_text())
                .and(asked::THE_LABEL_FIELD, "Kitchen"),
        ),
    );
    assert_eq!(
        submitted.the_request_name_it_says_it_does_not_answer(),
        None
    );
    assert_eq!(
        submitted.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME)
    );
    assert_eq!(
        submitted.fields.text(asked::ON_THIS_MACHINE_FIELD),
        Some(asked::IT_IS_PRESENT)
    );
    assert_eq!(
        submitted.fields.text(asked::THE_QUEUE_STATE_FIELD),
        Some(asked::IT_IS_WAITING)
    );

    // **The second job arrives held behind the first** (`Rendering` S13).
    let second = root.a_published_job();
    let held = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_JOB_RECORD_IS_WRITTEN_REQUEST,
            Fields::none()
                .and(asked::THE_JOB_FIELD, second.as_text())
                .and(asked::THE_HELD_BEHIND_FIELD, first.as_text())
                .and(asked::THE_HOLD_REASON_FIELD, "the first-frame check"),
        ),
    );
    assert_eq!(
        held.fields.text(asked::THE_QUEUE_STATE_FIELD),
        Some(asked::IT_IS_HELD)
    );
    let hold = Identity::read_back(
        IdentityKind::Operation,
        held.fields
            .text(asked::THE_HOLD_IT_IS_HELD_UNDER_FIELD)
            .expect("the hold is named"),
    )
    .expect("it reads back");
    let holds = smoozy_runtime::acting::stores::TheHoldRecordsOnTheMachine::in_the_area(
        &area.where_a_store_sits(records::THE_AREA_THE_HOLD_RECORDS_SIT_IN),
    )
    .expect("the area is there");
    let placed = smoozy_runtime::holds::KeepsHoldRecords::read(&holds, &hold)
        .expect("it reads")
        .expect("the hold record is on the disk");
    assert_eq!(
        placed.subject,
        smoozy_runtime::holds::Subject::Job(second.clone())
    );
    assert_eq!(placed.origin, smoozy_runtime::holds::Origin::Submission);
    assert_eq!(placed.reason, "the first-frame check");
    assert_eq!(
        placed.dependency.expect("it has one").work,
        first,
        "the dependency is the first job"
    );
    assert!(placed.release.is_none());

    // **Does the machine hold this job**: present, and absent for one it
    // never took.
    for (job, standing) in [
        (&first, asked::IT_IS_PRESENT),
        (
            &Identity::issue(IdentityKind::Job).expect("randomness"),
            asked::IT_IS_ABSENT,
        ),
    ] {
        let said = asked_through_the_adapter(
            &area,
            &a_request_of(
                asked::THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
                Fields::none().and(asked::THE_JOB_FIELD, job.as_text()),
            ),
        );
        assert_eq!(
            said.fields.text(asked::ON_THIS_MACHINE_FIELD),
            Some(standing)
        );
    }

    // **The queue reading**: both jobs, the first in the line and next.
    let queue = asked_through_the_adapter(
        &area,
        &a_request_of(asked::THE_QUEUE_READING_REQUEST, Fields::none()),
    );
    assert_eq!(queue.fields.text(asked::HOW_MANY_JOBS_FIELD), Some("2"));
    let first_text = first.as_text();
    assert_eq!(
        queue.fields.text(&asked::the_field_of_item(
            asked::THE_JOB_FIELD,
            0,
            asked::THE_JOB_FIELD
        )),
        Some(first_text.as_str())
    );
    assert_eq!(
        queue.fields.text(&asked::the_field_of_item(
            asked::THE_JOB_FIELD,
            0,
            asked::THE_PLACE_IN_THE_LINE_FIELD
        )),
        Some("1")
    );
    assert_eq!(
        queue.fields.text(asked::THE_JOB_THAT_STARTS_NEXT_FIELD),
        Some(first_text.as_str())
    );

    // **The per-job note**: the label, and a null ending.
    let note = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_PER_JOB_NOTE_READING_REQUEST,
            Fields::none().and(asked::THE_JOB_FIELD, first.as_text()),
        ),
    );
    assert_eq!(note.fields.text(asked::THE_LABEL_FIELD), Some("Kitchen"));
    assert_eq!(note.fields.text(asked::THE_ENDING_FIELD), None);

    // **Read forward from zero**: the first job's record, then the hold
    // placed, then the second job's record — each event after the write it
    // is about.
    let events = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_READ_FORWARD_REQUEST,
            Fields::none().and(asked::THE_CURSOR_FIELD, "0"),
        ),
    );
    let how_many: usize = events
        .fields
        .text(asked::HOW_MANY_EVENTS_FIELD)
        .expect("a count")
        .parse()
        .expect("a number");
    let kinds_about: Vec<(String, String)> = (0..how_many)
        .map(|at| {
            let field = |name| {
                events
                    .fields
                    .text(&asked::the_field_of_item(asked::AN_EVENT, at, name))
                    .unwrap_or_default()
                    .to_string()
            };
            (field(asked::THE_KIND_FIELD), field(asked::THE_ABOUT_FIELD))
        })
        .filter(|(kind, _)| {
            kind == asked::THE_JOB_RECORD_WRITTEN_EVENT || kind == asked::THE_HOLD_PLACED_EVENT
        })
        .collect();
    assert_eq!(
        kinds_about,
        vec![
            (
                asked::THE_JOB_RECORD_WRITTEN_EVENT.to_string(),
                first.as_text()
            ),
            (asked::THE_HOLD_PLACED_EVENT.to_string(), hold.as_text()),
            (
                asked::THE_JOB_RECORD_WRITTEN_EVENT.to_string(),
                second.as_text()
            ),
        ]
    );

    // **The cursor moves forward**, and is held for the desktop.
    let next = events
        .fields
        .text(asked::THE_NEXT_SEQUENCE_FIELD)
        .expect("the next sequence")
        .to_string();
    let the_move = a_request_of(
        asked::THE_CURSOR_MOVES_FORWARD_REQUEST,
        Fields::none().and(asked::THE_CURSOR_FIELD, next.clone()),
    );
    let moved = asked_through_the_adapter(&area, &the_move);
    assert_eq!(
        moved.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME)
    );
    assert_eq!(
        moved.fields.text(asked::THE_CURSOR_FIELD),
        Some(next.as_str())
    );
    // **`Rented-server runtime` §13 R1's row for this one command: it ends as
    // the command closes.** The machine holds no record under its identity
    // afterwards, which is what stops every acknowledgement P10d sends leaving
    // a file behind for the life of the rental.
    assert!(
        !area.holds_a_record_under(&the_move.identity),
        "R1: the cursor move's command record ends as the command closes"
    );
    let after = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_READ_FORWARD_REQUEST,
            Fields::none().and(asked::THE_CURSOR_FIELD, next),
        ),
    );
    assert_eq!(after.fields.text(asked::HOW_MANY_EVENTS_FIELD), Some("0"));

    // **Material first** (`Queue & History` Q3): a job whose manifest was
    // never published is refused, and no record stands for it.
    let unpublished = Identity::issue(IdentityKind::Job).expect("randomness");
    let not_yet = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_JOB_RECORD_IS_WRITTEN_REQUEST,
            Fields::none().and(asked::THE_JOB_FIELD, unpublished.as_text()),
        ),
    );
    assert_eq!(
        not_yet.fields.text(asked::ON_THIS_MACHINE_FIELD),
        Some(asked::IT_IS_ABSENT)
    );
    assert!(not_yet
        .fields
        .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        .is_some_and(|words| words.contains("not published")));

    // **The order broken on purpose: the note cannot be written.** The notes'
    // area is taken away and a plain file put in its place, so every note
    // write is refused by the system whoever runs the test.
    let notes = area
        .where_a_store_sits(smoozy_queue_history_machine::acting::note::THE_AREA_THE_NOTES_SIT_IN);
    std::fs::rename(&notes, notes.with_extension("aside")).expect("the notes are moved aside");
    std::fs::write(&notes, b"not a folder").expect("a file stands in the way");
    let third = root.a_published_job();
    let refused = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_JOB_RECORD_IS_WRITTEN_REQUEST,
            Fields::none().and(asked::THE_JOB_FIELD, third.as_text()),
        ),
    );
    assert_eq!(
        refused.fields.text(asked::ON_THIS_MACHINE_FIELD),
        Some(asked::IT_IS_ABSENT)
    );
    assert!(refused
        .fields
        .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        .is_some());
    assert!(
        !area
            .where_a_store_sits(
                smoozy_queue_history_machine::acting::queue_store::THE_AREA_THE_QUEUE_SITS_IN
            )
            .join(format!("{}.json", third.as_text()))
            .exists(),
        "no job record stands without its note"
    );
    // **And no event says it was written**: the event follows the record and
    // never stands in for one.
    let since = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_READ_FORWARD_REQUEST,
            Fields::none().and(asked::THE_CURSOR_FIELD, "0"),
        ),
    );
    let third_text = third.as_text();
    let how_many: usize = since
        .fields
        .text(asked::HOW_MANY_EVENTS_FIELD)
        .expect("a count")
        .parse()
        .expect("a number");
    assert!(
        (0..how_many).all(|at| since.fields.text(&asked::the_field_of_item(
            asked::AN_EVENT,
            at,
            asked::THE_ABOUT_FIELD
        )) != Some(third_text.as_str())),
        "no event is journalled about a job whose record was not written"
    );
    let asked_again = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
            Fields::none().and(asked::THE_JOB_FIELD, third.as_text()),
        ),
    );
    assert_eq!(
        asked_again.fields.text(asked::ON_THIS_MACHINE_FIELD),
        Some(asked::IT_IS_ABSENT)
    );
    std::fs::remove_file(&notes).ok();
    std::fs::rename(notes.with_extension("aside"), &notes).ok();
}

// --- step 10c piece 5, sitting 2: the line runs and the render worker ends
// every attempt ---

/// **A stand-in for Blender, and it is named as one.** A shell script at the
/// path a manifest names as its Blender build, which leaves a mark beside
/// itself if it is ever run. **Nothing here claims what a real Blender does**:
/// the render worker's attempts end at the render-start reading, before any
/// Blender is invoked, and the mark is how that is watched rather than argued.
fn a_stand_in_for_blender(root: &ARoot) -> (PathBuf, PathBuf) {
    use std::os::unix::fs::PermissionsExt;
    let stand_in = root.at.join("a-stand-in-for-blender");
    let mark = root.at.join("the-stand-in-was-run");
    // **It answers `Rendering` L14's version ask and nothing else.** The ask is
    // its own short invocation of the binary about to be launched, and a
    // stand-in that did not answer it would end every attempt there rather than
    // at the reading this test is about. **Its reading run prints no document
    // at all**, which is `Job files and outputs` C9's *a reading that did not
    // end successfully*; the mark is left only by the render's own launch, so
    // it still says whether a render was ever started.
    std::fs::write(
        &stand_in,
        format!(
            "#!/bin/sh\n             for word in \"$@\"; do\n             case \"$word\" in\n             *version-ask-*.py) printf '<<<SMOOZY-READING>>>5.2.0<<<END-SMOOZY-READING>>>'; exit 0;;\n             *reading-*.py) exit 0;;\n             *launch-material-*.py) touch '{}'; exit 0;;\n             esac\n             done\n             exit 0\n",
            mark.display()
        ),
    )
    .expect("the stand-in is written");
    std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in may be run");
    (stand_in, mark)
}

impl ARoot {
    /// **A job whose whole material is on this root the way a submission
    /// leaves it** — the manifest in its job record directory, the remap
    /// material at `Dependency Upload & Remap` M10's place in the asset area,
    /// the `.blend` the manifest names, and the job output folder — **naming
    /// the stand-in above as its Blender**. The file was not read before
    /// submitting, which is the only kind of job the desktop submits today.
    fn a_job_the_render_worker_can_take(&self, stand_in: &Path) -> Identity {
        self.a_job_the_render_worker_can_take_over(
            stand_in,
            smoozy_job_files_machine::manifest::FrameRange::TheUsersParsedIntegers(vec![1, 2, 3]),
        )
    }

    /// The same, over the frame range named — `Render Settings` Y9's two
    /// forms, the user's integers or *the file's own*.
    fn a_job_the_render_worker_can_take_over(
        &self,
        stand_in: &Path,
        frame_range: smoozy_job_files_machine::manifest::FrameRange,
    ) -> Identity {
        self.a_job_written(stand_in, frame_range, None)
    }

    /// **A job whose folder is chosen at render start** (`Job files and
    /// outputs` P21's second case, every job the desktop submits from
    /// 2026-10-04): no output folder is made at submission, the `.blend` is
    /// the one at `blend` — wherever the test put it — and the manifest
    /// carries N3's name and the server's chosen fallback folder.
    fn a_job_whose_folder_is_chosen_at_render_start(
        &self,
        stand_in: &Path,
        frame_range: smoozy_job_files_machine::manifest::FrameRange,
        blend: &Path,
        the_servers_fallback_folder: Option<&str>,
    ) -> Identity {
        self.a_job_written(
            stand_in,
            frame_range,
            Some((blend, the_servers_fallback_folder)),
        )
    }

    fn a_job_written(
        &self,
        stand_in: &Path,
        frame_range: smoozy_job_files_machine::manifest::FrameRange,
        at_render_start: Option<(&Path, Option<&str>)>,
    ) -> Identity {
        use smoozy_job_files_machine::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
        use smoozy_job_files_machine::manifest::{
            AnchorState, CarriedAndNotInterpretedHere, SceneField, SubmissionManifest,
            TheBlenderToRenderBy, TheCardMakeAtSubmission, TheRecordedReading,
        };
        use smoozy_rendering_machine::material::testing::{
            a_remap, an_untouched_payload, four_values,
        };
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let root = MachineRoot::chosen(&self.at.display().to_string()).expect("a root");
        let layout = TheLayout::of(root, WhereThePackageAreaSits::UnderTheRoot);
        let paths = smoozy_job_files_machine::publication::AJobsPaths::composed(
            &layout,
            &job,
            Some("Kitchen"),
        )
        .expect("the paths compose");
        std::fs::create_dir_all(&paths.job_record_directory).expect("the record directory");
        let blend = match at_render_start {
            Some((blend, _)) => blend.to_path_buf(),
            None => {
                std::fs::create_dir_all(&paths.job_output_folder).expect("the output folder");
                let blend = self.at.join("kitchen.blend");
                std::fs::write(&blend, b"a stand-in for a .blend").expect("the file");
                blend
            }
        };
        let remap = a_remap(&job, false);
        let remap_at = PathBuf::from(
            String::from_utf8(
                smoozy_dependencies_machine::remap::RemapMaterial::location_in(&layout, &job)
                    .as_bytes()
                    .to_vec(),
            )
            .expect("a path"),
        );
        std::fs::create_dir_all(remap_at.parent().expect("a parent")).expect("the asset area");
        std::fs::write(&remap_at, remap.as_written()).expect("the remap material");
        let manifest = SubmissionManifest::composed(
            &job,
            None,
            TheBlenderToRenderBy {
                version: "5.2".to_string(),
                build_path: stand_in.display().to_string(),
            },
            SceneField {
                name: "Scene".to_string(),
                anchor: AnchorState::ToAnchor {
                    provenance: "typed".to_string(),
                },
            },
            frame_range,
            an_untouched_payload().as_carried(),
            smoozy_dependencies_machine::remap::RemapMaterial::manifest_reference(&layout, &job),
            CarriedAndNotInterpretedHere(serde_json::json!("everything")),
            blend.display().to_string(),
            four_values(),
            TheRecordedReading::ThereIsNone,
            // **M1b's card make**, as the desktop composes it for an ordinary
            // rented GPU machine — `Rendering` L23's only input here.
            TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
            match at_render_start {
                None => smoozy_job_files_machine::manifest::WhereTheJobOutputFolderIs::named_by(&paths),
                Some((_, fallback)) => smoozy_job_files_machine::manifest::WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                    the_folders_name: smoozy_job_files_machine::names::job_output_folder_name(&job, Some("Kitchen")),
                    the_servers_fallback_folder_at_submission: fallback.map(str::to_string),
                },
            },
        );
        std::fs::write(
            Path::new(&paths.job_record_directory)
                .join(smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME),
            manifest.as_written(),
        )
        .expect("the manifest is published");
        job
    }
}

/// **A job written, asked for through the real adapter.**
fn submitted(area: &APackageArea, job: &Identity) {
    use smoozy_framing::requests as asked;
    let answered = asked_through_the_adapter(
        area,
        &a_request_of(
            asked::THE_JOB_RECORD_IS_WRITTEN_REQUEST,
            smoozy_framing::framing::Fields::none()
                .and(asked::THE_JOB_FIELD, job.as_text())
                .and(asked::THE_LABEL_FIELD, "Kitchen"),
        ),
    );
    assert_eq!(
        answered.fields.text(asked::ON_THIS_MACHINE_FIELD),
        Some(asked::IT_IS_PRESENT),
        "{:?}",
        answered.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
}

/// **One job's note, read through the real adapter, once it carries an
/// ending** — waited for with a test's own bound, so a line that never moves
/// fails the test rather than hanging it.
fn the_note_once_it_has_ended(
    area: &APackageArea,
    job: &Identity,
) -> smoozy_framing::framing::Answered {
    use smoozy_framing::requests as asked;
    let began = Instant::now();
    loop {
        let note = asked_through_the_adapter(
            area,
            &a_request_of(
                asked::THE_PER_JOB_NOTE_READING_REQUEST,
                smoozy_framing::framing::Fields::none().and(asked::THE_JOB_FIELD, job.as_text()),
            ),
        );
        if note.fields.text(asked::THE_ENDING_FIELD).is_some() {
            return note;
        }
        assert!(
            began.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN,
            "{} never ended: {}",
            job.as_text(),
            String::from_utf8_lossy(&std::fs::read(area.at.join("complaints")).unwrap_or_default())
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Every event from the start of the journal, as (kind, about, says).
fn every_event(area: &APackageArea) -> Vec<(String, String, String)> {
    use smoozy_framing::requests as asked;
    let events = asked_through_the_adapter(
        area,
        &a_request_of(
            asked::THE_READ_FORWARD_REQUEST,
            smoozy_framing::framing::Fields::none().and(asked::THE_CURSOR_FIELD, "0"),
        ),
    );
    let how_many: usize = events
        .fields
        .text(asked::HOW_MANY_EVENTS_FIELD)
        .expect("a count")
        .parse()
        .expect("a number");
    (0..how_many)
        .map(|at| {
            let field = |name| {
                events
                    .fields
                    .text(&asked::the_field_of_item(asked::AN_EVENT, at, name))
                    .unwrap_or_default()
                    .to_string()
            };
            (
                field(asked::THE_KIND_FIELD),
                field(asked::THE_ABOUT_FIELD),
                field(asked::THE_SAYS_FIELD),
            )
        })
        .collect()
}

/// The render kind's own field of an operation record, read off the disk.
fn the_render_kinds_own_field(held: &WhatTheMachineHoldsUnderAnIdentity) -> serde_json::Value {
    let WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record) = held else {
        panic!("an operation record is held: {held:?}");
    };
    serde_json::from_slice(
        record
            .the_kinds_safe_boundary_marker
            .as_deref()
            .expect("the render kind's field is written"),
    )
    .expect("it is a document")
}

/// The render operation a job's record names, read off the disk.
fn the_render_operation_of(area: &APackageArea, job: &Identity) -> Identity {
    let queue = smoozy_queue_history_machine::acting::queue_store::TheQueueOnTheMachine::at(
        area.where_a_store_sits(
            smoozy_queue_history_machine::acting::queue_store::THE_AREA_THE_QUEUE_SITS_IN,
        ),
    );
    match queue
        .read_one(job)
        .expect("it reads")
        .expect("it is there")
        .state
    {
        smoozy_queue_history_machine::line::QueueState::Ended { outcome } => outcome,
        other => panic!("{} has not ended: {other:?}", job.as_text()),
    }
}

/// **Step 10c piece 5's second sitting, proved by name at a running
/// supervisor: the line runs with nobody connected, a render worker of this
/// program is started for each job in turn and reaped, and each attempt's
/// ending is written by the supervisor with everything that rides it.**
///
/// **Two jobs are written and nothing else is asked for.** The first is
/// started straight away, because nothing is rendering (`Queue & History` Q4);
/// its worker asks what it was started for, composes the attempt from the
/// manifest, reads the remap material and the file's four values, and **ends
/// at the render-start reading, which it does not take because no script for it
/// is on record** — `failed`, cause *undetermined*, no completeness statement
/// and the reason said. The supervisor writes the outcome, the note's ending,
/// the record *ended* and the event; and only then does the second job start.
///
/// **What this does not prove, said plainly**: that a render runs. The
/// stand-in for Blender is never invoked — the mark it would leave is looked for
/// and is not there — and a real render wants a Blender on a Linux machine,
/// which is the passes' work.
#[test]
fn the_line_starts_each_job_in_turn_and_every_attempt_is_ended_by_the_supervisor() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let (stand_in, mark) = a_stand_in_for_blender(&root);
    let first = root.a_job_the_render_worker_can_take(&stand_in);
    let second = root.a_job_the_render_worker_can_take(&stand_in);
    area.start_the_supervisor();

    submitted(&area, &first);
    submitted(&area, &second);

    for job in [&first, &second] {
        let note = the_note_once_it_has_ended(&area, job);
        assert_eq!(
            note.fields.text(asked::THE_ENDING_FIELD),
            Some(asked::IT_FAILED)
        );
        assert_eq!(note.fields.text(asked::THE_ATTEMPTS_FIELD), Some("1"));
        assert!(note.fields.text(asked::THE_STARTED_FIELD).is_some());
        assert!(
            note.fields
                .text(asked::WHY_NO_COMPLETENESS_STATEMENT_FIELD)
                .is_some_and(|why| why.contains("no inventory")),
            "`Rented-server runtime` S13: an ending with no statement says why"
        );
        assert_eq!(note.fields.text(asked::THE_FILES_EXPECTED_FIELD), None);

        // **The operation record: closed with the outcome, and the render
        // kind's own field carrying what the worker reported** (M6, W4).
        let operation = the_render_operation_of(&area, job);
        let held = what_the_machine_holds(&area, &operation);
        assert_eq!(
            held.recorded_outcome(),
            Some(&RecordedOutcome::TheKindsOwn(asked::IT_FAILED.to_string()))
        );
        let field = the_render_kinds_own_field(&held);
        let attempt = &field["attempts"][0];
        assert_eq!(attempt["attempt"], 1);
        assert_eq!(attempt["ending"]["cause"], "undetermined");
        // **The reason is the ending's own sentence, never its type's name**
        // (step 10c's sixth pass on Vast).
        assert_eq!(
            attempt["ending"]["reason"].as_str(),
            Some(
                smoozy_rendering_machine::endings::FailedReason::BeforeTheRenderInstruction(
                    smoozy_rendering_machine::endings::BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully,
                )
                .in_words()
                .as_str()
            )
        );
        assert_eq!(
            attempt["ending"]["ordinary"], "never established",
            "`Rented-server runtime` M13: no launch, so nothing of the render's printing"
        );
        // **M1: the two sinks were made before the worker started.**
        for sink in ["ordinary_sink", "complaints_sink"] {
            assert!(Path::new(attempt[sink].as_str().expect("a path")).is_file());
        }
    }

    // **Each job's events in their order, and the second started only after
    // the first had ended** — one render at a time (`Rendering` K1).
    let events = every_event(&area);
    let at = |kind: &str, job: &Identity| {
        events
            .iter()
            .position(|(k, about, _)| k == kind && *about == job.as_text())
            .unwrap_or_else(|| panic!("no {kind} about {}: {events:#?}", job.as_text()))
    };
    for job in [&first, &second] {
        assert!(
            at(asked::THE_JOB_RECORD_WRITTEN_EVENT, job)
                < at(asked::THE_RENDER_ATTEMPT_STARTED_EVENT, job)
        );
        assert!(
            at(asked::THE_RENDER_ATTEMPT_STARTED_EVENT, job)
                < at(asked::THE_RECORDED_OUTCOME_WRITTEN_EVENT, job)
        );
    }
    assert!(
        at(asked::THE_RECORDED_OUTCOME_WRITTEN_EVENT, &first)
            < at(asked::THE_RENDER_ATTEMPT_STARTED_EVENT, &second),
        "the second job started only once the first one's outcome was written"
    );

    // **The queue reading: both ended, nothing waiting, nothing next.**
    let queue = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_QUEUE_READING_REQUEST,
            smoozy_framing::framing::Fields::none(),
        ),
    );
    assert_eq!(
        queue.fields.text(asked::THE_JOB_THAT_STARTS_NEXT_FIELD),
        None
    );
    for at in 0..2 {
        assert_eq!(
            queue.fields.text(&asked::the_field_of_item(
                asked::THE_JOB_FIELD,
                at,
                asked::THE_QUEUE_STATE_FIELD
            )),
            Some(asked::IT_HAS_ENDED)
        );
    }

    assert!(
        !mark.exists(),
        "the stand-in for Blender was never run: every attempt ended at the reading"
    );
}

/// **`State, identity and recovery` §6's *re-run once*, and the second attempt
/// it owes, started by the line at the next supervisor start with nobody
/// connected** — `Queue & History` D4: *the machine's own act, from durable
/// state*.
///
/// A job is written while a setup run holds the line, the supervisor is
/// killed, and the machine is left the way a render killed mid-attempt leaves
/// it: the job *rendering* under an operation whose recorded group is gone.
/// **The next start re-runs it once and starts attempt 2 under the same
/// operation**, whose worker ends it as every attempt ends today.
#[test]
fn a_render_re_run_once_is_started_again_by_the_line_and_ended_at_attempt_two() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let (stand_in, _) = a_stand_in_for_blender(&root);
    let job = root.a_job_the_render_worker_can_take(&stand_in);
    area.start_the_supervisor();
    the_line_is_held_by_a_setup_run(&area);
    submitted(&area, &job);
    area.kill_the_supervisor();

    // **The machine as a kill mid-render leaves it.** The setup run is gone,
    // the job is *rendering* under an operation, and that operation's record
    // is open with a group nothing is at.
    std::fs::remove_file(area.where_a_store_sits(records::THE_SETUP_IN_PROGRESS_STATE))
        .expect("the setup state is taken away");
    let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
    let mut queue = smoozy_queue_history_machine::acting::queue_store::TheQueueOnTheMachine::at(
        area.where_a_store_sits(
            smoozy_queue_history_machine::acting::queue_store::THE_AREA_THE_QUEUE_SITS_IN,
        ),
    );
    let mut record = queue
        .read_one(&job)
        .expect("it reads")
        .expect("it is there");
    record.state = smoozy_queue_history_machine::line::QueueState::Rendering {
        operation: operation.clone(),
    };
    queue.write(&record).expect("the record is rendering");
    let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area.where_a_store_sits(
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
    ))
    .expect("the store is there");
    let mut open = OperationRecord::opened(operation.clone(), job.clone(), "render");
    open.execution_group = Some(TheRecordedExecutionGroup {
        as_that_document_recorded_it: RecordedPair {
            identifier: AN_IDENTIFIER_NOTHING_IS_AT,
            start_time: 1,
        }
        .as_this_document_records_it(),
        started_under_run: Identity::issue(IdentityKind::Run).expect("randomness"),
    });
    store
        .record_before_acting(&open)
        .expect("the record is written");

    area.start_the_supervisor();
    let note = the_note_once_it_has_ended(&area, &job);
    assert_eq!(
        note.fields.text(asked::THE_ENDING_FIELD),
        Some(asked::IT_FAILED)
    );
    assert_eq!(note.fields.text(asked::THE_ATTEMPTS_FIELD), Some("2"));
    assert_eq!(note.fields.text(asked::HOW_MANY_RE_RUNS_FIELD), Some("1"));
    assert_eq!(
        note.fields
            .text(asked::THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD),
        Some("2")
    );

    let held = what_the_machine_holds(&area, &operation);
    assert_eq!(
        held.recorded_outcome(),
        Some(&RecordedOutcome::TheKindsOwn(asked::IT_FAILED.to_string())),
        "the second attempt ran under the same operation and closed it"
    );
    let WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(closed) = &held else {
        panic!("an operation record");
    };
    assert_eq!(closed.attempt_count, 2);
    let field = the_render_kinds_own_field(&held);
    assert_eq!(
        field["attempts"][0]["attempt"], 2,
        "M19: the second attempt's own sinks"
    );
    assert!(field["attempts"][0]["ordinary_sink"]
        .as_str()
        .is_some_and(|sink| sink.contains("attempt-2")));

    let events = every_event(&area);
    let disposition = events
        .iter()
        .find(|(kind, about, _)| {
            kind == "a reconciliation disposition was written" && *about == operation.as_text()
        })
        .expect("the disposition was journalled");
    assert!(disposition
        .2
        .contains("the second attempt is started by the line"));
    assert!(events.iter().any(|(kind, about, says)| kind
        == asked::THE_RENDER_ATTEMPT_STARTED_EVENT
        && *about == job.as_text()
        && says == "attempt 2"));
}

/// **Step 10c's Sesterce-pass fix piece 2: a job a restart ends says which
/// frames it saved** — `Rented-server runtime` S11's one walk at
/// reconciliation, through the real program.
///
/// **What the pass saw**: *pbv lion2* was stopped, the supervisor restarted,
/// and reconciliation closed it *left stopped*; its `0001.png` had been saved
/// before the stop and was downloaded, and History read *Last frame — none
/// finished*. **The walk took the inventory from the manifest's recorded
/// reading, which the desktop has written as *there is none* since
/// 2026-09-29**, so every reconciliation-written ending carried no statement.
/// **The inventory for the map in force is the attempt's own recorded
/// reading**, which the render worker writes into the job record directory at
/// its step 4 (`State, identity and recovery` §1.2).
///
/// The machine is left the way a stop, a restart and a render that has since
/// gone leave it: the job *rendering*, its operation *should be stopped* with a
/// group nothing is at, the attempt's recorded reading on the disk, composed by
/// the function the worker composes it with, and frame 1 saved. **The job's
/// range is the file's own**, which is what an untouched range sends, so the
/// frames come from that reading too (`Render Settings` Y9). The next start
/// takes §6's *gone* row: *left stopped*, with the statement.
#[test]
fn a_job_a_restart_ends_says_which_frames_it_saved() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use smoozy_job_files_machine::inventory::WhatTheInventoryIs;
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let (stand_in, _) = a_stand_in_for_blender(&root);
    let job = root.a_job_the_render_worker_can_take_over(
        &stand_in,
        smoozy_job_files_machine::manifest::FrameRange::TheFilesOwn,
    );
    area.start_the_supervisor();
    the_line_is_held_by_a_setup_run(&area);
    submitted(&area, &job);
    area.kill_the_supervisor();
    std::fs::remove_file(area.where_a_store_sits(records::THE_SETUP_IN_PROGRESS_STATE))
        .expect("the setup state is taken away");

    // **The attempt's recorded reading, as the worker's step 4 writes it.**
    let record_directory = root.at.join("job-records").join(job.as_text());
    let manifest = smoozy_job_files_machine::manifest::SubmissionManifest::read_back(
        &std::fs::read(
            record_directory.join(smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME),
        )
        .expect("the manifest"),
    )
    .expect("it reads back");
    let mut printed: smoozy_job_files_machine::reading_script::ThePrintedReading =
        serde_json::from_str(&the_document_a_reading_prints()).expect("it reads");
    // **The scene's own span is the three frames the reading covers.**
    printed.settings.0["scenes"][0]["frame_end"] = serde_json::json!(3);
    printed.settings.0["scenes"][0]["frame_count"] = serde_json::json!(3);
    let enters_at = smoozy_job_files_machine::main_output::WhereTheRenderEnters {
        scene: smoozy_job_files_machine::reading::ArtistsText::of(b"Scene".to_vec()),
        view_layer: smoozy_job_files_machine::reading::ArtistsText::of(b"ViewLayer".to_vec()),
    };
    let folder = root.at.display().to_string();
    let composed = smoozy_job_files_machine::composing::the_reading_and_everything_derived_from(
        printed,
        smoozy_job_files_machine::composing::WhatTheReadingIsComposedAgainst {
            the_folder_the_blend_sits_in: folder.as_bytes(),
            job_output_folder: manifest
                .job_output_folder
                .the_path_it_names()
                .expect("named"),
            enters_at: &enters_at,
            the_jobs_pins: &[],
        },
    )
    .expect("the reading composes");
    std::fs::write(
        record_directory
            .join(smoozy_job_files_machine::manifest::the_recorded_readings_file_name(1)),
        serde_json::to_vec(&composed).expect("it renders"),
    )
    .expect("the recorded reading is written");

    // **Frame 1 was saved before the stop.**
    let WhatTheInventoryIs::It(inventory) = &composed.inventory else {
        panic!("an inventory: {:?}", composed.inventory);
    };
    assert_eq!(inventory.the_count(), 3);
    let saved = inventory
        .entries
        .iter()
        .find(|entry| entry.frame == Some(1))
        .expect("frame 1 is owed a file");
    let saved = PathBuf::from(String::from_utf8(saved.predicted_path.clone()).expect("a path"));
    std::fs::create_dir_all(saved.parent().expect("a folder")).expect("the folder is made");
    std::fs::write(&saved, b"a frame").expect("frame 1 is saved");

    // **The stop, the restart, and the render gone.**
    let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
    let mut queue = smoozy_queue_history_machine::acting::queue_store::TheQueueOnTheMachine::at(
        area.where_a_store_sits(
            smoozy_queue_history_machine::acting::queue_store::THE_AREA_THE_QUEUE_SITS_IN,
        ),
    );
    let mut record = queue
        .read_one(&job)
        .expect("it reads")
        .expect("it is there");
    record.state = smoozy_queue_history_machine::line::QueueState::Rendering {
        operation: operation.clone(),
    };
    queue.write(&record).expect("the record is rendering");
    let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area.where_a_store_sits(
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
    ))
    .expect("the store is there");
    let mut open = OperationRecord::opened(operation.clone(), job.clone(), "render");
    open.execution_group = Some(TheRecordedExecutionGroup {
        as_that_document_recorded_it: RecordedPair {
            identifier: AN_IDENTIFIER_NOTHING_IS_AT,
            start_time: 1,
        }
        .as_this_document_records_it(),
        started_under_run: Identity::issue(IdentityKind::Run).expect("randomness"),
    });
    open.the_stop_was_asked_for().expect("it is open");
    store
        .record_before_acting(&open)
        .expect("the record is written");

    area.start_the_supervisor();
    let note = the_note_once_it_has_ended(&area, &job);
    assert_eq!(
        note.fields.text(asked::THE_ENDING_FIELD),
        Some(asked::IT_WAS_LEFT_STOPPED)
    );
    assert_eq!(
        note.fields.text(asked::THE_FILES_EXPECTED_FIELD),
        Some("3"),
        "the walk counts against the attempt's own inventory; it said {:?}",
        note.fields.text(asked::WHY_NO_COMPLETENESS_STATEMENT_FIELD)
    );
    assert_eq!(note.fields.text(asked::THE_FILES_PRESENT_FIELD), Some("1"));
    assert_eq!(note.fields.text(asked::THE_FILES_ABSENT_FIELD), Some("2"));
    assert_eq!(
        note.fields.text(asked::THE_LAST_FRAME_FIELD),
        Some("1"),
        "History names the frame saved before the stop"
    );
    assert_eq!(
        what_the_machine_holds(&area, &operation).recorded_outcome(),
        Some(&RecordedOutcome::LeftStopped)
    );
}

/// **A stand-in for the Blender a job names that makes a derivative** — the
/// 38's piece 3. It answers `Rendering` L14's version ask as the stand-in above
/// does, and **for the preview worker's script it does what the script asks**:
/// it says the source's size between the size marks, reads the size it is
/// told back on its standard input, and writes a picture at the staging path
/// the script's own data names, and says it has written it. What it writes is
/// these words and the size, so the test can tell the derivative it published
/// from anything else.
fn a_stand_in_that_makes_a_derivative(root: &ARoot) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let stand_in = root.at.join("a-stand-in-that-makes-a-derivative");
    std::fs::write(
        &stand_in,
        r#"#!/bin/sh
for word in "$@"; do
case "$word" in
*version-ask-*.py) printf '<<<SMOOZY-READING>>>5.2.0<<<END-SMOOZY-READING>>>'; exit 0;;
*derivative.py)
staged=$(grep -o '/[^"\\]*staging-preview-[0-9-]*[.]png' "$word" | head -n 1)
printf '<<<SMOOZY-SIZE>>>64 48<<<END-SMOOZY-SIZE>>>\n'
read told
printf 'a derivative at %s\n' "$told" > "$staged"
printf '<<<SMOOZY-DERIVATIVE-MADE>>>\n'
exit 0;;
esac
done
exit 0
"#,
    )
    .expect("the stand-in is written");
    std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in may be run");
    stand_in
}

/// **`Queue & History` P1b through the real program: the ending's write is the
/// catch-up.** A job a supervisor restart ends — its render gone, its stop
/// outstanding — is ended by reconciliation with the completeness reading
/// counting frames 1 and 2 of 3 already there, and **in the same pass preview
/// work is queued for those two and no other**: a preview worker of this
/// program is started for each in turn; frame 1's makes the derivative with
/// the Blender the job names (a stand-in here), publishes it by a rename into
/// the job record directory (`Job files and outputs` B29, B30), and the
/// supervisor journals the production (`Queue & History` P3) — about the job,
/// saying the frame; **frame 2, the interrupted job's last frame asked for
/// that is present, has nothing made** (`Job files and outputs` B20), and its
/// record says so. The 38's piece 3.
///
/// **What is stood in for, said plainly**: Blender. The render's own frames
/// never run through the real program here (`Rented-server runtime` X1), so
/// the two frames are written by the test where the attempt's inventory
/// predicts them, as the restart test above does.
#[test]
fn a_job_a_restart_ends_has_a_preview_made_of_each_frame_it_saved() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use smoozy_job_files_machine::inventory::WhatTheInventoryIs;
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let stand_in = a_stand_in_that_makes_a_derivative(&root);
    let job = root.a_job_the_render_worker_can_take_over(
        &stand_in,
        smoozy_job_files_machine::manifest::FrameRange::TheFilesOwn,
    );
    area.start_the_supervisor();
    the_line_is_held_by_a_setup_run(&area);
    submitted(&area, &job);
    area.kill_the_supervisor();
    std::fs::remove_file(area.where_a_store_sits(records::THE_SETUP_IN_PROGRESS_STATE))
        .expect("the setup state is taken away");

    // **The attempt's recorded reading, as the worker's step 4 writes it.**
    let record_directory = root.at.join("job-records").join(job.as_text());
    let manifest = smoozy_job_files_machine::manifest::SubmissionManifest::read_back(
        &std::fs::read(
            record_directory.join(smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME),
        )
        .expect("the manifest"),
    )
    .expect("it reads back");
    let mut printed: smoozy_job_files_machine::reading_script::ThePrintedReading =
        serde_json::from_str(&the_document_a_reading_prints()).expect("it reads");
    printed.settings.0["scenes"][0]["frame_end"] = serde_json::json!(3);
    printed.settings.0["scenes"][0]["frame_count"] = serde_json::json!(3);
    let enters_at = smoozy_job_files_machine::main_output::WhereTheRenderEnters {
        scene: smoozy_job_files_machine::reading::ArtistsText::of(b"Scene".to_vec()),
        view_layer: smoozy_job_files_machine::reading::ArtistsText::of(b"ViewLayer".to_vec()),
    };
    let folder = root.at.display().to_string();
    let composed = smoozy_job_files_machine::composing::the_reading_and_everything_derived_from(
        printed,
        smoozy_job_files_machine::composing::WhatTheReadingIsComposedAgainst {
            the_folder_the_blend_sits_in: folder.as_bytes(),
            job_output_folder: manifest
                .job_output_folder
                .the_path_it_names()
                .expect("named"),
            enters_at: &enters_at,
            the_jobs_pins: &[],
        },
    )
    .expect("the reading composes");
    std::fs::write(
        record_directory
            .join(smoozy_job_files_machine::manifest::the_recorded_readings_file_name(1)),
        serde_json::to_vec(&composed).expect("it renders"),
    )
    .expect("the recorded reading is written");

    // **Frames 1 and 2 were saved before the stop; frame 3 was not.**
    let WhatTheInventoryIs::It(inventory) = &composed.inventory else {
        panic!("an inventory: {:?}", composed.inventory);
    };
    for frame in [1, 2] {
        let saved = inventory
            .entries
            .iter()
            .find(|entry| entry.frame == Some(frame))
            .expect("the frame is owed a file");
        let saved = PathBuf::from(String::from_utf8(saved.predicted_path.clone()).expect("a path"));
        std::fs::create_dir_all(saved.parent().expect("a folder")).expect("the folder is made");
        std::fs::write(&saved, b"a frame").expect("the frame is saved");
    }

    // **The stop, the restart, and the render gone.**
    let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
    let mut queue = smoozy_queue_history_machine::acting::queue_store::TheQueueOnTheMachine::at(
        area.where_a_store_sits(
            smoozy_queue_history_machine::acting::queue_store::THE_AREA_THE_QUEUE_SITS_IN,
        ),
    );
    let mut record = queue
        .read_one(&job)
        .expect("it reads")
        .expect("it is there");
    record.state = smoozy_queue_history_machine::line::QueueState::Rendering {
        operation: operation.clone(),
    };
    queue.write(&record).expect("the record is rendering");
    let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area.where_a_store_sits(
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
    ))
    .expect("the store is there");
    let mut open = OperationRecord::opened(operation.clone(), job.clone(), "render");
    open.execution_group = Some(TheRecordedExecutionGroup {
        as_that_document_recorded_it: RecordedPair {
            identifier: AN_IDENTIFIER_NOTHING_IS_AT,
            start_time: 1,
        }
        .as_this_document_records_it(),
        started_under_run: Identity::issue(IdentityKind::Run).expect("randomness"),
    });
    open.the_stop_was_asked_for().expect("it is open");
    store
        .record_before_acting(&open)
        .expect("the record is written");

    area.start_the_supervisor();
    let note = the_note_once_it_has_ended(&area, &job);
    assert_eq!(note.fields.text(asked::THE_FILES_PRESENT_FIELD), Some("2"));

    // **P3: the production is journalled, about the job, saying the frame.**
    let began = Instant::now();
    let the_previews_records = |store: &TheOperationRecordsOnTheMachine| {
        store
            .every_operation_record_of_the_kind("preview")
            .expect("the store reads")
            .into_iter()
            .filter(|record| record.recorded_outcome().is_some())
            .collect::<Vec<_>>()
    };
    let published = loop {
        let published: Vec<(String, String, String)> = every_event(&area)
            .into_iter()
            .filter(|(kind, about, _)| {
                kind == asked::THE_PREVIEW_PUBLISHED_EVENT && about == &job.as_text()
            })
            .collect();
        if !published.is_empty() && the_previews_records(&store).len() >= 2 {
            break published;
        }
        assert!(
            began.elapsed() < Duration::from_secs(30),
            "no preview was published for {}: {}",
            job.as_text(),
            String::from_utf8_lossy(&std::fs::read(area.at.join("complaints")).unwrap_or_default())
        );
        std::thread::sleep(Duration::from_millis(200));
    };
    assert_eq!(published.len(), 1, "frame 1's alone: {published:?}");
    assert!(
        published[0].2.contains("\"frame\":1"),
        "the event says the frame: {}",
        published[0].2
    );

    // **B29 and B30: published by a rename into the job record directory**,
    // never into the job output folder, and nothing left at the staged name.
    let derivative = record_directory.join("preview-1.png");
    assert_eq!(
        std::fs::read(&derivative).expect("the derivative is published"),
        b"a derivative at 64 48\n",
        "made at the size the worker told the Blender"
    );
    assert!(!record_directory.join("staging-preview-1.png").exists());

    // **P1b queues the frames the reading counts and no others**: frame 3,
    // never saved, has no production at all; and **frame 2 is B20's
    // interrupted job's last frame asked for that is present**, so it has
    // nothing made, recorded rather than retried.
    let records = the_previews_records(&store);
    let outcome_of = |frame: i64| {
        records
            .iter()
            .filter(|record| {
                serde_json::from_slice::<serde_json::Value>(
                    record
                        .the_kinds_safe_boundary_marker
                        .as_deref()
                        .expect("the field"),
                )
                .expect("a document")["frame"]
                    == frame
            })
            .map(|record| record.recorded_outcome().cloned())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        outcome_of(1),
        vec![Some(RecordedOutcome::TheKindsOwn("published".to_string()))]
    );
    assert_eq!(
        outcome_of(2),
        vec![Some(RecordedOutcome::TheKindsOwn(
            "nothing to make".to_string()
        ))]
    );
    assert!(outcome_of(3).is_empty(), "frame 3 was never queued");
    assert!(!record_directory.join("preview-2.png").exists());
}

/// What the stand-in render below says when the first interrupt reaches it —
/// Blender's own line on step 10c's thirteenth pass.
const IT_HEARD_THE_FIRST: &str = "Sent an internal break event. Press ^C again to kill Blender";

/// **A stand-in render that takes the first interrupt as the real Blender did
/// on step 10c's thirteenth pass**: it says Blender's line and runs on, and
/// only a second interrupt ends it, with status 2. Every interrupt it hears is
/// written down, so the test counts what was sent. It lives a minute at most.
fn a_stand_in_that_wants_a_second_interrupt(at: &Path) -> (Child, PathBuf) {
    let heard = at.join("what-the-stand-in-heard");
    let script = format!(
        "n=0\n\
         trap 'n=$((n+1)); if [ \"$n\" -ge 2 ]; then echo \"Blender killed\" >> \"$1\"; exit 2; fi; \
         echo \"{IT_HEARD_THE_FIRST}\" >> \"$1\"' INT\n\
         echo ready >> \"$1\"\n\
         i=0\n\
         while [ \"$i\" -lt 60 ]; do sleep 1 & wait $!; i=$((i+1)); done\n\
         exit 0\n"
    );
    let child = Command::new("/bin/sh")
        .args(["-c", &script, "a-stand-in-render"])
        .arg(&heard)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("a stand-in render");
    let began = Instant::now();
    while !std::fs::read_to_string(&heard)
        .unwrap_or_default()
        .contains("ready")
    {
        assert!(
            began.elapsed() < Duration::from_secs(10),
            "the stand-in never became ready"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    (child, heard)
}

/// Every interrupt the stand-in has heard, in order.
fn what_it_heard(heard: &Path) -> Vec<String> {
    std::fs::read_to_string(heard)
        .unwrap_or_default()
        .lines()
        .filter(|line| *line != "ready")
        .map(str::to_string)
        .collect()
}

/// Waits, within a bound, until the stand-in has heard `how_many`.
fn it_heard(heard: &Path, how_many: usize) -> bool {
    let began = Instant::now();
    while began.elapsed() < Duration::from_secs(10) {
        if what_it_heard(heard).len() >= how_many {
            return true;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    false
}

/// A stop of a render, through the real adapter, by its own identity.
fn a_stop_of(operation: &Identity) -> Request {
    use smoozy_framing::requests as asked;
    a_request_of(
        asked::THE_STOP_REQUEST,
        smoozy_framing::framing::Fields::none()
            .and(asked::THE_OPERATION_TO_STOP_FIELD, operation.as_text()),
    )
}

/// **Step 10c's Sesterce-pass fix piece 2, through the real program: a render
/// outlives a supervisor restart with its stop outstanding, and the restarted
/// supervisor keeps hold of it.**
///
/// The pass before Vast on Sesterce: *Stop*, then `systemctl --user restart
/// smoozy-supervisor` inside the wait. Reconciliation closed the job *left
/// stopped* while Blender walked on, and a later job's Blender started beside
/// it. **Here the render is a stand-in of the second shape and is its own
/// recorded group**; a supervisor takes the stop over the endpoint and sends
/// the interrupt, and is then killed — which is all S4's restart does to it,
/// the unit being written to K18 — and another is started through the pointer.
/// **Then**: *resumed*, open, *should be stopped*; nothing sent at the
/// re-attach nor by the waiter's repeats of the same stop; a fresh stop sends
/// the second interrupt, which ends it; the worker's report from the earlier
/// run is accepted and recorded *left stopped*, and both stops close with it.
///
/// **It runs whole only where the kernel answers for a process** — every
/// rented machine, the Linux runner, and the desk from 2026-10-24. Where it
/// answers nothing, every recorded group reads as gone, no interrupt can be
/// validated and §6 takes its *gone* row; that half is asserted rather than
/// skipped.
#[test]
fn a_render_outlives_a_supervisor_restart_with_its_stop_outstanding() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use smoozy_queue_history_machine::acting::note::{
        the_job_record_and_its_note_in_one_act, ThePerJobNotesOnTheMachine,
        THE_AREA_THE_NOTES_SIT_IN,
    };
    use smoozy_queue_history_machine::acting::queue_store::{
        TheQueueOnTheMachine, THE_AREA_THE_QUEUE_SITS_IN,
    };
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    area.kill_the_supervisor();

    let (mut child, heard) = a_stand_in_that_wants_a_second_interrupt(&area.at);
    let kernel =
        smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt::the_rented_machine();
    let its_pair =
        smoozy_runtime::execution_group::ValidatesTheRecordedPair::what_is_at(&kernel, child.id())
            .ok()
            .flatten();

    // **A job rendering under an operation whose group is the stand-in.**
    let job = Identity::issue(IdentityKind::Job).expect("randomness");
    let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
    let the_run_it_was_started_under = Identity::issue(IdentityKind::Run).expect("randomness");
    let mut record = smoozy_queue_history_machine::line::JobRecord::submitted(
        job.clone(),
        None,
        area.at.join("job-record").display().to_string(),
        &[],
    );
    record.state = smoozy_queue_history_machine::line::QueueState::Rendering {
        operation: operation.clone(),
    };
    the_job_record_and_its_note_in_one_act(
        &mut TheQueueOnTheMachine::at(area.where_a_store_sits(THE_AREA_THE_QUEUE_SITS_IN)),
        &mut ThePerJobNotesOnTheMachine::at(area.where_a_store_sits(THE_AREA_THE_NOTES_SIT_IN)),
        &record,
        "Kitchen",
    )
    .expect("the job is written");
    let pair = its_pair.unwrap_or(RecordedPair {
        identifier: AN_IDENTIFIER_NOTHING_IS_AT,
        start_time: 1,
    });
    let mut open = OperationRecord::opened(operation.clone(), job.clone(), "render");
    open.execution_group = Some(TheRecordedExecutionGroup {
        as_that_document_recorded_it: pair.as_this_document_records_it(),
        started_under_run: the_run_it_was_started_under.clone(),
    });
    open.the_kinds_safe_boundary_marker = Some(
        serde_json::to_vec(&serde_json::json!({
            "job": job.as_text(),
            "attempts": [{
                "attempt": 1,
                "ordinary_sink": area.at.join("o").display().to_string(),
                "complaints_sink": area.at.join("c").display().to_string(),
                "available_bytes": 1,
                "render_process": pair.as_this_document_records_it(),
            }],
        }))
        .expect("it renders"),
    );
    TheOperationRecordsOnTheMachine::in_the_area(&area.where_a_store_sits(
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
    ))
    .expect("the store is there")
    .record_before_acting(&open)
    .expect("the record is written");

    let Some(_) = its_pair else {
        // **The Mac's half.** Nothing here can validate a group, so a stop
        // outstanding across a restart meets §6's *gone* row: *left stopped*,
        // and nothing sent to what could not be validated.
        open.the_stop_was_asked_for().expect("it is open");
        TheOperationRecordsOnTheMachine::in_the_area(&area.where_a_store_sits(
            smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
        ))
        .expect("the store is there")
        .write_it_back(&open)
        .expect("the stop is written");
        area.start_the_supervisor();
        assert_eq!(
            what_the_machine_holds(&area, &operation).recorded_outcome(),
            Some(&RecordedOutcome::LeftStopped),
            "a machine that publishes no per-process record reads every group as gone"
        );
        let late = asked_through_the_adapter(&area, &a_stop_of(&operation));
        assert_eq!(
            late.fields.text(asked::THE_STOPS_OUTCOME_FIELD),
            Some(asked::IT_WAS_LEFT_STOPPED)
        );
        assert!(
            what_it_heard(&heard).is_empty(),
            "nothing was sent to what could not be validated"
        );
        child.kill().ok();
        child.wait().ok();
        return;
    };

    // ***Stop***, taken by a supervisor, and Blender's own line.
    area.start_the_supervisor();
    let first = a_stop_of(&operation);
    let asked_first = asked_through_the_adapter(&area, &first);
    assert_eq!(
        asked_first.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND),
        "{:?}",
        asked_first.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    assert!(
        it_heard(&heard, 1),
        "the first interrupt reached the render"
    );

    // **The supervisor is restarted inside the wait.**
    area.kill_the_supervisor();
    area.start_the_supervisor();
    let held = what_the_machine_holds(&area, &operation);
    assert_eq!(
        held.recorded_outcome(),
        None,
        "the stop is not closed while its render runs"
    );
    assert_eq!(
        held.what_an_open_record_carries().map(|(which, _)| which),
        Some(Disposition::Resumed)
    );

    // **The waiter goes on asking the same stop.**
    for _ in 0..3 {
        let asked = asked_through_the_adapter(&area, &first);
        assert_eq!(
            asked.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::IT_IS_STILL_IN_HAND)
        );
    }
    std::thread::sleep(Duration::from_secs(1));
    assert_eq!(
        what_it_heard(&heard),
        vec![IT_HEARD_THE_FIRST.to_string()],
        "nothing is sent at the re-attach, and the waiter's repeats send nothing"
    );
    assert!(
        child.try_wait().expect("readable").is_none(),
        "the render runs on, held"
    );

    // **A fresh *Stop* ends it.**
    asked_through_the_adapter(&area, &a_stop_of(&operation));
    let began = Instant::now();
    let ended = loop {
        if let Some(status) = child.try_wait().expect("readable") {
            break Some(status);
        }
        if began.elapsed() > Duration::from_secs(10) {
            child.kill().ok();
            child.wait().ok();
            break None;
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(
        ended.and_then(|status| status.code()),
        Some(2),
        "{:?}",
        what_it_heard(&heard)
    );

    // **The worker's report, from the run it was started under.**
    let report = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        operation.clone(),
        smoozy_runtime::acting::workers::THE_WORKERS_REPORT_REQUEST,
    )
    .carrying(
        smoozy_framing::framing::Fields::none()
            .and(
                smoozy_runtime::acting::workers::THE_RUN_THE_WORKER_WAS_STARTED_UNDER,
                the_run_it_was_started_under.as_text(),
            )
            .and(asked::THE_ENDING_FIELD, asked::IT_FAILED)
            .and(asked::WHY_NO_COMPLETENESS_STATEMENT_FIELD, "a stand-in"),
    );
    let accepted = asked_through_the_adapter(&area, &report);
    assert_eq!(
        accepted
            .fields
            .text(smoozy_runtime::acting::workers::WHETHER_THE_REPORT_WAS_ACCEPTED),
        Some(smoozy_runtime::acting::workers::IT_WAS_ACCEPTED)
    );
    assert_eq!(
        what_the_machine_holds(&area, &operation).recorded_outcome(),
        Some(&RecordedOutcome::LeftStopped)
    );
    let again = asked_through_the_adapter(&area, &first);
    assert_eq!(
        again.fields.text(asked::THE_STOPS_OUTCOME_FIELD),
        Some(asked::IT_WAS_LEFT_STOPPED)
    );
}

/// **What a reading of a `.blend` prints, as this test's stand-in prints it.**
///
/// **Nothing here says what a real Blender prints.** It is a document of this
/// workspace's own shapes — `Job files and outputs` R4's items 1 to 6 and
/// `Render Settings`' settings half — so what the test proves is the driver's
/// step 4 and everything hanging off it, never the script and never Blender.
/// One destination, the scene's own output path, over the three frames the
/// manifest asks for.
fn the_document_a_reading_prints() -> String {
    use smoozy_job_files_machine::reading::{
        ArtistsText, AtOneFrame, Destination, DestinationKind, Format, GatingConditions,
        HowManyViewsTheSceneWrites, RecordedLocation, TheVersionInHand, WhereItComesFrom,
        WhichScene,
    };
    use smoozy_job_files_machine::reading_script::{TheObservedOutputs, ThePrintedReading};
    let layer = || ArtistsText::of(b"ViewLayer".to_vec());
    // **No compositing tree at all**, which is `Job files and outputs` B5's
    // case: the raw render lands at the regular output path and no walk is
    // needed.
    let gating = GatingConditions {
        the_effects_on_for_render: None,
        compositing_switch: true,
        a_compositing_tree_is_present: false,
        a_live_final_output_node: false,
        the_final_output_node_is_muted: false,
        four_fives_use_nodes_switch: None,
        save_output_switch: None,
        the_render_goes_out_through_a_sequencer_holding_a_scene_strip: false,
    };
    let printed = ThePrintedReading {
        outputs: TheObservedOutputs {
            taken_by: TheVersionInHand::of(5, 2),
            gating: gating.clone(),
            destinations: vec![Destination {
                kind: DestinationKind::TheRegularOutputPath,
                recorded_location: Some(RecordedLocation::of(b"//out/f_".to_vec())),
                format: Some(Format {
                    as_the_file_records_it: ArtistsText::of(b"PNG".to_vec()),
                    the_ending_it_writes: Some(ArtistsText::of(b".png".to_vec())),
                    is_video_shaped: false,
                }),
                origin: WhereItComesFrom::TheScenesOutputPath,
                scene: WhichScene::TheJobsOwnScene,
                the_view_layers_that_feed_it: vec![layer()],
                the_location_with_its_folder_replaced: None,
            }],
            scenes_reached_through_a_sequencer_strip: Vec::new(),
            the_tree_is_shared_with: None,
            per_frame: (1..=3)
                .map(|frame| AtOneFrame {
                    frame,
                    view_layers_that_render: vec![layer()],
                    gating: gating.clone(),
                    the_regular_output_paths_answered_path: Some(RecordedLocation::of(
                        format!("/artist/out/f_{frame:04}.png").into_bytes(),
                    )),
                })
                .collect(),
            recorded_locations: Vec::new(),
            views: HowManyViewsTheSceneWrites::One,
            the_compositing_tree: None,
            the_view_transform: None,
        },
        settings: smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere(
            serde_json::to_value(smoozy_render_settings_machine::reading::SettingsHalf {
                scenes: vec![{
                    // **The scene renders on the processor, and that is not a
                    // tidy choice.** `Rendering` L23 looks the technology up
                    // from the machine's card make, `Rendering` L29 says the
                    // manifest carries it, and `Job files and outputs` M1's
                    // list of what the manifest holds has no field for it — so
                    // the worker answers *the cards could not be established*
                    // and every cards job ends at L23's loud failure before the
                    // render. **That is the plan seam the runner's R8 ruling
                    // routed to a plan fix**, and it is not this piece's; with
                    // step 4 real it is now the first thing a cards job meets
                    // rather than the second.
                    let mut scene = smoozy_rendering_machine::material::testing::a_scene("Scene");
                    scene.device = smoozy_render_settings_machine::reading::DeviceSetting::TheProcessor;
                    scene
                }],
                offered_engine_identifiers:
                    smoozy_render_settings_machine::reading::OfferedEngineIdentifiers {
                        cycles: Some(
                            smoozy_render_settings_machine::names::ExactName::as_the_reading_produced_it(
                                "CYCLES",
                            ),
                        ),
                        eevee: Some(
                            smoozy_render_settings_machine::names::ExactName::as_the_reading_produced_it(
                                "BLENDER_EEVEE",
                            ),
                        ),
                    },
                announced_add_on_case: None,
                the_folders_at_the_machines_top: None,
            })
            .expect("the settings half renders"),
        ),
        dependencies: smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere(
            serde_json::json!({}),
        ),
        errors: std::collections::BTreeMap::new(),
        the_baked_caches_on_the_machine: Vec::new(),
        the_scene_holds_a_simulation_not_baked: false,
        the_scene_it_read: None,
        the_open_files_path: None,
        the_check_is_written_as_a_picture: false,
    };
    serde_json::to_string(&printed).expect("the document renders")
}

/// **A stand-in for Blender that answers all three of the attempt's
/// invocations** — `Rendering` L14's version ask, `Job files and outputs` R1's
/// reading, and the render itself.
///
/// **Nothing about a real Blender is proved by it.** It prints the material's
/// own marker head on the render, because that is what `Rented-server runtime`
/// M28 observes and what the ending turns on.
fn a_stand_in_that_reads_and_renders(root: &ARoot) -> PathBuf {
    a_stand_in_that_prints_this_reading_and_renders(root, &the_document_a_reading_prints())
}

/// The same stand-in, printing a reading the test hands it.
fn a_stand_in_that_prints_this_reading_and_renders(root: &ARoot, reading: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let stand_in = root.at.join("a-stand-in-that-renders");
    let document = root.at.join("what-the-reading-prints");
    std::fs::write(&document, reading).expect("the document is written");
    std::fs::write(
        &stand_in,
        format!(
            "#!/bin/sh\n\
             for word in \"$@\"; do\n\
             case \"$word\" in\n\
             *version-ask-*.py) printf '<<<SMOOZY-READING>>>5.2.0<<<END-SMOOZY-READING>>>'; exit 0;;\n\
             *reading-*.py) printf '<<<SMOOZY-READING>>>'; cat '{document}'; printf '<<<END-SMOOZY-READING>>>'; exit 0;;\n\
             esac\n\
             done\n\
             echo 'SMOOZY-MATERIAL {{\"job\":\"a stand-in\"}}'\n\
             exit 0\n",
            document = document.display()
        ),
    )
    .expect("the stand-in is written");
    std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in may be run");
    stand_in
}

/// **Step 10c piece 14: the render-start reading is taken, and everything the
/// attempt composes comes out of it.**
///
/// A job whose file was never read before submitting is taken by the render
/// worker, which reads the file at `Rendering` P1 step 4, composes the
/// destination map, the expected-output inventory and the two picks from what
/// it read, writes the recorded reading into the job record directory
/// (`State, identity and recovery` §1.2) and has it journalled
/// (`Job files and outputs` H9), then reckons, composes and launches.
///
/// **Before this piece every attempt this program ever made ended at step 4**:
/// no script for the reading existed and the driver took the map, the inventory
/// and the hold answer before the step that produces them.
///
/// **What this does not prove, said plainly**: that a real Blender reads a real
/// `.blend`. The stand-in prints a document of this workspace's own shapes, and
/// a real render wants a Blender on a Linux machine, which is the passes' work.
#[test]
fn the_render_start_reading_is_taken_and_the_redirect_reaches_the_launch() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let stand_in = a_stand_in_that_reads_and_renders(&root);
    let job = root.a_job_the_render_worker_can_take(&stand_in);
    area.start_the_supervisor();
    submitted(&area, &job);

    let note = the_note_once_it_has_ended(&area, &job);

    // **`Job files and outputs` C7: the completeness statement is counted
    // against the inventory for the map in force**, which step 4 composed —
    // three frames, one destination. **Before this piece there was no
    // inventory at all** and the ending said so instead of counting.
    assert_eq!(note.fields.text(asked::THE_FILES_EXPECTED_FIELD), Some("3"));

    // **The attempt record, which is what step 7 wrote before the launch.**
    // **`Job files and outputs` M8: the map in force for that attempt**, and
    // the frame list the reckoning composed against the inventory step 4
    // made. Before this piece the record carried *no map was composed for this
    // attempt* and no attempt ever reached step 7 at all.
    //
    // **The ending itself is not asserted and that is deliberate.** A render's
    // recorded pair is the kernel's own per-process record (`Rendering` L5,
    // `Rented-server runtime` X1), and **the Mac this is run at publishes
    // none** — so at the desk the start is undone and the attempt ends failed,
    // while on the Linux the program actually runs on the same stand-in would
    // reach the marker. **What both systems agree on is everything step 4 and
    // step 7 wrote down**, and that is what is read here. **The words the
    // launch was made with are read in `Rendering — machine`'s own driver
    // test**, where the start is a stand-in rather than the kernel's.

    // **`State, identity and recovery` §1.2: the recorded reading is in the job
    // record directory**, and `Job files and outputs` M8's attempt record
    // carries the map in force beside it.
    let record_directory = root.at.join("job-records").join(job.as_text());
    let written: Vec<String> = std::fs::read_dir(&record_directory)
        .expect("the job record directory")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .to_string()
        })
        .collect();
    assert!(
        written.iter().any(|name| {
            *name == smoozy_job_files_machine::manifest::the_recorded_readings_file_name(1)
        }),
        "the recorded reading is written where §1.2 says: {written:?}"
    );
    let attempt: serde_json::Value = serde_json::from_slice(
        &std::fs::read(record_directory.join("attempt-1.json")).expect("the attempt record"),
    )
    .expect("it is a document");
    assert!(
        attempt["map_in_force"]["It"]["pairs"]
            .as_array()
            .is_some_and(|pairs| !pairs.is_empty()),
        "M8: the attempt record carries the map in force: {attempt}"
    );
    assert_eq!(
        attempt["frame_list_in_the_order_it_was_written"],
        serde_json::json!([1, 2, 3]),
        "the reckoning's own list, against the inventory step 4 composed"
    );

    // **`Job files and outputs` H9: journalled as an event when it is
    // written**, about the job, saying the attempt.
    let events = every_event(&area);
    assert!(
        events.iter().any(
            |(kind, about, says)| kind == asked::THE_RECORDED_READING_WRITTEN_EVENT
                && *about == job.as_text()
                && says == "attempt 1"
        ),
        "{events:#?}"
    );
}

// --- step 10c's dependency wave piece 8: an uploaded bake copied into the
// job folder before the render ---

/// **The reading above, with the scene's bake output path recorded as
/// `//bake/`** — one of `Job files and outputs` D20's four added write places,
/// which the map puts under the job's folder.
fn a_reading_that_records_a_bake_output_path() -> String {
    use smoozy_job_files_machine::reading::{
        AnAddedWritePlace, Destination, DestinationKind, RecordedLocation, WhereItComesFrom,
        WhichScene,
    };
    use smoozy_job_files_machine::reading_script::ThePrintedReading;
    let mut printed: ThePrintedReading =
        serde_json::from_str(&the_document_a_reading_prints()).expect("it reads");
    printed.outputs.destinations.push(Destination {
        kind: DestinationKind::AnAddedWritePlace(AnAddedWritePlace::TheBakeOutputPath),
        recorded_location: Some(RecordedLocation::of(b"//bake/".to_vec())),
        format: None,
        origin: WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::TheBakeOutputPath),
        scene: WhichScene::TheJobsOwnScene,
        the_view_layers_that_feed_it: Vec::new(),
        the_location_with_its_folder_replaced: None,
    });
    serde_json::to_string(&printed).expect("the document renders")
}

/// **`Dependency Upload & Remap` M22c, at a running supervisor: a bake the
/// check sent is in the asset area, the job's remap material says a copy of it
/// is owed, and by the time the render is started the copy is at the
/// job-folder place `Job files and outputs` D20 gives the bake output path —
/// with its four values read the moment it was made (`Dependency Upload &
/// Remap` M22f).**
///
/// **Written first, against the program as it stood, and it failed there**:
/// the render was started and the bake was not in the job's folder, so a bake
/// the check had sent would never have been read, and nothing said so.
///
/// **What it does not prove**: that a real Blender reads the placed bake
/// (`Dependency Upload & Remap` M22e, §12 item 7, permanently unresolved). It
/// proves the bake is where the setting points.
#[test]
fn an_uploaded_bake_is_copied_into_the_job_folder_before_the_render_starts() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_dependencies_machine::placed_copies::{
        the_placed_copies_file_name, TheCopyTheMachineOwes, ThePlacedCopies,
    };
    use smoozy_job_files_machine::attempts::{AttemptRecord, TheMapInForce};
    use smoozy_job_files_machine::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
    use smoozy_job_files_machine::mapping::HowItMapped;
    use smoozy_job_files_machine::reading::{AnAddedWritePlace, DestinationKind};
    use std::os::unix::fs::MetadataExt;
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let stand_in = a_stand_in_that_prints_this_reading_and_renders(
        &root,
        &a_reading_that_records_a_bake_output_path(),
    );
    let job = root.a_job_the_render_worker_can_take(&stand_in);

    // **The bake as the check's send leaves it**: a folder in the asset area,
    // one file at its top and one a level down.
    let layout = TheLayout::of(
        MachineRoot::chosen(&root.at.display().to_string()).expect("a root"),
        WhereThePackageAreaSits::UnderTheRoot,
    );
    let mirror = root.at.join("assets/files/Users/a/proj/bake");
    std::fs::create_dir_all(mirror.join("tiles")).expect("the mirror folder");
    std::fs::write(mirror.join("lightmap.png"), b"a baked lightmap").expect("a baked file");
    std::fs::write(mirror.join("tiles/t1.exr"), b"a baked tile").expect("a baked file");

    // **The instruction, riding the remap material the render already reads.**
    let remap_at = PathBuf::from(
        String::from_utf8(
            smoozy_dependencies_machine::remap::RemapMaterial::location_in(&layout, &job)
                .as_bytes()
                .to_vec(),
        )
        .expect("a path"),
    );
    let mut remap = smoozy_dependencies_machine::remap::RemapMaterial::read_back(
        &std::fs::read(&remap_at).expect("the material"),
    )
    .expect("it reads");
    remap.copies_owed = vec![TheCopyTheMachineOwes {
        from: smoozy_dependencies_machine::bytes::MachineLocation::composed(
            mirror.display().to_string().into_bytes(),
        ),
        place: AnAddedWritePlace::TheBakeOutputPath,
        recorded: smoozy_dependencies_machine::bytes::RecordedLocation::as_the_reading_carried_it(
            b"//bake/",
        ),
    }];
    std::fs::write(&remap_at, remap.as_written()).expect("the material is written");

    area.start_the_supervisor();
    submitted(&area, &job);
    the_note_once_it_has_ended(&area, &job);

    // **Step 7 reached the launch**: the attempt record is what it writes
    // just before the render is started, and it carries the map in force.
    let record_directory = root.at.join("job-records").join(job.as_text());
    let attempt = AttemptRecord::read_back(
        &std::fs::read(record_directory.join("attempt-1.json")).expect("the attempt record"),
    )
    .expect("it reads");
    let TheMapInForce::It(map) = attempt.map_in_force else {
        panic!("the attempt carries the map in force")
    };
    let the_place = map
        .pairs
        .iter()
        .find_map(|pair| match (&pair.destination.kind, &pair.outcome) {
            (
                DestinationKind::AnAddedWritePlace(AnAddedWritePlace::TheBakeOutputPath),
                HowItMapped::UnderTheJobsFolder { new_folder, .. },
            ) => Some(PathBuf::from(
                String::from_utf8(new_folder.clone()).expect("a path"),
            )),
            _ => None,
        })
        .expect("the map gives the bake output path a folder under the job's");

    // **The copy is at the place, byte for byte, and the master is untouched.**
    assert_eq!(
        std::fs::read(the_place.join("lightmap.png"))
            .ok()
            .as_deref(),
        Some(&b"a baked lightmap"[..]),
        "the bake is not at the job-folder place {} though the render was started: {}",
        the_place.display(),
        String::from_utf8_lossy(&std::fs::read(area.at.join("complaints")).unwrap_or_default())
    );
    assert_eq!(
        std::fs::read(the_place.join("tiles/t1.exr"))
            .ok()
            .as_deref(),
        Some(&b"a baked tile"[..])
    );
    assert_eq!(
        std::fs::read(mirror.join("lightmap.png")).expect("the master"),
        b"a baked lightmap"
    );

    // **M22f: every placed file, with the four values read when it was made.**
    let placed = ThePlacedCopies::read_back(
        &std::fs::read(record_directory.join(the_placed_copies_file_name(1)))
            .expect("the placed-copy list is in the job record directory"),
    )
    .expect("it reads");
    assert_eq!(placed.files.len(), 2, "{placed:?}");
    for file in &placed.files {
        let at = PathBuf::from(
            String::from_utf8(file.job_folder_path.as_bytes().to_vec()).expect("a path"),
        );
        assert!(at.starts_with(&the_place), "{}", at.display());
        let now = std::fs::metadata(&at).expect("the placed file");
        assert_eq!(
            (
                file.size,
                file.inode.clone(),
                file.modification_time,
                file.change_time
            ),
            (now.size(), now.ino().to_string(), now.mtime(), now.ctime()),
            "the four values are the copy's own"
        );
    }

    // **`Download` N5a's *now*: the same files read again the moment the render
    // ended, beside the list** — the fixes before 0.1.8, fault 3. The stand-in
    // render touched none of them, so each reads as it did when it was made,
    // which is what lets the job's download leave them on the machine.
    let the_reading_after = record_directory.join(
        smoozy_dependencies_machine::placed_copies::the_placed_copies_after_the_render_file_name(1),
    );
    let after =
        ThePlacedCopies::read_back(&std::fs::read(&the_reading_after).expect(
            "the placed files are read again after the render, in the job record directory",
        ))
        .expect("it reads");
    assert_eq!(
        after, placed,
        "a render that touched nothing leaves every placed file as it was made"
    );
}

// --- step 10c's fix piece 2: the note carries why a job failed and what did
// the work ---

/// **The device line a stand-in prints, in the 5.x shape** — the head the
/// reader matches and words after it that are illustrative and carried as
/// printed. **Nothing here says what a real Blender prints.**
const A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT: &str =
    "Path tracing on: NVIDIA GeForce RTX 4090 (CUDA)";

/// **The stand-in that reads and renders, which also prints the device line
/// before the marker** — `Queue & History` E13's observation, for the tee at
/// the worker's own loop to take. **It writes one frame of what the job owes**,
/// since a render ending at 0 having written none of it is failed (`Rendering`
/// K14b, 2026-11-02).
fn a_stand_in_that_reads_renders_and_names_its_device(root: &ARoot) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let stand_in = root.at.join("a-stand-in-that-names-its-device");
    let document = root.at.join("what-the-reading-prints");
    std::fs::write(&document, the_document_a_reading_prints()).expect("the document is written");
    std::fs::write(
        &stand_in,
        format!(
            "#!/bin/sh\n\
             for word in \"$@\"; do\n\
             case \"$word\" in\n\
             *version-ask-*.py) printf '<<<SMOOZY-READING>>>5.2.0<<<END-SMOOZY-READING>>>'; exit 0;;\n\
             *reading-*.py) printf '<<<SMOOZY-READING>>>'; cat '{document}'; printf '<<<END-SMOOZY-READING>>>'; exit 0;;\n\
             esac\n\
             [ \"$before\" = '--render-output' ] && out=\"$word\"\n\
             before=\"$word\"\n\
             done\n\
             echo '00:00.600  cycles           | {A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT}'\n\
             echo 'SMOOZY-MATERIAL {{\"job\":\"a stand-in\"}}'\n\
             [ -n \"$out\" ] && printf a-frame > \"${{out}}0001.png\"\n\
             exit 0\n",
            document = document.display()
        ),
    )
    .expect("the stand-in is written");
    std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in may be run");
    stand_in
}

/// **A failed job, end to end through the real program: the reason the
/// machine recorded reaches the note reading's answer in words, and the
/// desktop's own reader reads it back** — the paragraph after
/// `Queue & History` N3, and `Rendering` K14.
///
/// The supervisor is started through the pointer, the job is written through
/// the real adapter, a real render worker of this program takes it and ends it
/// at the render-start reading (the stand-in prints no document), and the note
/// is read through the adapter. **Undone, this fails at the reason**: before
/// fix piece 2 the reason stayed in the operation record, and History said
/// only that the render did not finish.
#[test]
fn a_failed_jobs_reason_reaches_the_note_answer_in_words_through_the_real_program() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let (stand_in, _) = a_stand_in_for_blender(&root);
    let job = root.a_job_the_render_worker_can_take(&stand_in);
    area.start_the_supervisor();
    submitted(&area, &job);

    let note = the_note_once_it_has_ended(&area, &job);
    assert_eq!(
        note.fields.text(asked::THE_ENDING_FIELD),
        Some(asked::IT_FAILED)
    );
    // **The reason names the reading that failed** — `Rendering` P1 step 4's
    // reading of the job's `.blend`, which is where this stand-in fails — and
    // never the output folder, which nothing here read.
    let said = note
        .fields
        .text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD)
        .unwrap_or_default();
    assert!(
        !said.contains("output folder"),
        "the step-4 reading's failure blames the output folder: {said}"
    );
    let words = smoozy_rendering_machine::endings::FailedReason::BeforeTheRenderInstruction(
        smoozy_rendering_machine::endings::BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully,
    )
    .in_words();
    assert_eq!(
        note.fields
            .text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD),
        Some(words.as_str()),
        "the reason the machine recorded travels, in its own words"
    );
    assert_eq!(
        note.fields.text(asked::THE_DEVICE_LINE_FIELD),
        None,
        "no launch was made, so no line passed"
    );
    let read =
        smoozy_queue_history_machine::acting::note::TheNote::read_from_the_answer(&note.fields)
            .expect("the desktop's reader reads the answer whole");
    let ending = read.ending.expect("an ending");
    assert_eq!(
        ending.the_reason_the_machine_recorded.as_deref(),
        Some(words.as_str())
    );
    assert_eq!(ending.the_device_line_as_observed, None);
}

/// **`Rendering` K14b, end to end through the real program: a render that ends
/// at 0 having written none of what the job owed reaches the note as *failed*,
/// and a supervisor's restart reads the same word back.** The stand-in reads,
/// prints the marker, writes nothing and ends at 0, which is the ending *Reports
/// 1* records for a 5.3 scene with no camera; nothing here says a real Blender
/// does it. **It runs where the kernel answers for a process**, as the finished
/// test below does.
#[test]
fn a_render_ending_at_nought_having_written_nothing_reaches_the_note_as_failed_and_stays_so() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    if the_pair_of_this_process().is_none() {
        println!("UNPROVED HERE: this system publishes no per-process record.");
        return;
    }
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let stand_in = a_stand_in_that_reads_and_renders(&root);
    let job = root.a_job_the_render_worker_can_take(&stand_in);
    area.start_the_supervisor();
    submitted(&area, &job);

    let words = smoozy_rendering_machine::endings::FailedReason::NothingTheJobOwedWasWritten.in_words();
    let it_reads_failed = |note: &smoozy_framing::framing::Answered| {
        assert_eq!(note.fields.text(asked::THE_ENDING_FIELD), Some(asked::IT_FAILED));
        assert_eq!(note.fields.text(asked::THE_EXIT_STATUS_FIELD), Some("0"));
        assert_eq!(
            note.fields.text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD),
            Some(words.as_str())
        );
        let read = smoozy_queue_history_machine::acting::note::TheNote::read_from_the_answer(&note.fields)
            .expect("the desktop's reader reads the answer whole");
        assert_eq!(
            read.ending.expect("an ending").the_reason_the_machine_recorded.as_deref(),
            Some(words.as_str())
        );
    };
    it_reads_failed(&the_note_once_it_has_ended(&area, &job));
    area.kill_the_supervisor();
    area.start_the_supervisor();
    it_reads_failed(&the_note_once_it_has_ended(&area, &job));
}

/// **A finished job, end to end through the real program: the device line
/// the render printed reaches the note reading's answer verbatim, and the
/// desktop's own reader and `Queue & History` E12 state it** —
/// the paragraph after `Queue & History` N3.
///
/// **It runs whole only where the kernel answers for a process** — every
/// rented machine, the Linux runner, and the desk from 2026-10-24. Where it
/// answers nothing, the render worker undoes the start it cannot record
/// (`Rendering` L5a) and the job fails; what is asserted here then is that the
/// failure says why, in words, and that no line is invented. The finished half
/// in one process is `unix::tests::a_finished_job_played_by_a_stand_in_reaches_the_answer_with_its_device_line`,
/// which plays the same render through the same worker loop and supervisor in
/// one process.
#[test]
fn a_finished_jobs_device_line_reaches_the_note_answer_through_the_real_program() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use smoozy_queue_history_machine::ending::{
        the_device_field_from_the_line_as_printed, DevicesThatDidTheWork,
    };
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let stand_in = a_stand_in_that_reads_renders_and_names_its_device(&root);
    let job = root.a_job_the_render_worker_can_take(&stand_in);
    area.start_the_supervisor();
    submitted(&area, &job);

    let note = the_note_once_it_has_ended(&area, &job);
    let read =
        smoozy_queue_history_machine::acting::note::TheNote::read_from_the_answer(&note.fields)
            .expect("the desktop's reader reads the answer whole");
    let ending = read.ending.expect("an ending");
    let what_did_the_work = the_device_field_from_the_line_as_printed(
        ending.outcome.as_a_recorded_outcome(),
        ending.the_device_line_as_observed.as_deref(),
    );

    if the_pair_of_this_process().is_some() {
        assert_eq!(
            note.fields.text(asked::THE_ENDING_FIELD),
            Some(asked::IT_FINISHED)
        );
        assert_eq!(
            note.fields.text(asked::THE_DEVICE_LINE_FIELD),
            Some(A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT)
        );
        assert_eq!(
            note.fields
                .text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD),
            None
        );
        assert_eq!(
            what_did_the_work,
            DevicesThatDidTheWork::Known {
                the_line_as_printed: A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT.to_string()
            }
        );
        return;
    }

    // **The honest half, at a desk that publishes no per-process record.**
    println!(
        "UNPROVED HERE: this system ({}) publishes no per-process record, so the render's start \
         is undone and the job cannot finish. The finished half is proved in one process at this \
         desk and whole on Linux.",
        std::env::consts::OS
    );
    assert_eq!(
        note.fields.text(asked::THE_ENDING_FIELD),
        Some(asked::IT_FAILED)
    );
    let reason = note
        .fields
        .text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD)
        .expect("the failure says why");
    assert!(
        reason.contains("no execution group could be recorded"),
        "the machine's own words for the start it undid: {reason}"
    );
    assert_eq!(
        note.fields.text(asked::THE_DEVICE_LINE_FIELD),
        None,
        "no line is invented"
    );
    assert_eq!(what_did_the_work, DevicesThatDidTheWork::NotKnown);
}

/// **`Queue & History` E9 through the real program: a job's ending, read
/// through the real adapter, carries its attempt's capture record — two sinks
/// that are there on the machine, the bytes standing in each, and what each
/// stream came to — and the desktop's own reader reads it whole.** The
/// Creator's notes on 0.1.8, piece 6d.
///
/// **Where the kernel answers nothing the render's start is undone**, as
/// above, so the job fails; what is proved is that whatever the attempt came
/// to, its record travels with the ending and names files whose sizes are the
/// lengths it gives. **Undone, this fails at the count**: before piece 6d the
/// answer carried the attempt's number and no sink at all.
#[test]
fn an_ended_jobs_capture_record_reaches_the_note_answer_through_the_real_program() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use smoozy_queue_history_machine::acting::words::the_capture_records_read_from;
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let stand_in = a_stand_in_that_reads_renders_and_names_its_device(&root);
    let job = root.a_job_the_render_worker_can_take(&stand_in);
    area.start_the_supervisor();
    submitted(&area, &job);

    let note = the_note_once_it_has_ended(&area, &job);
    // **A stand-in run with no output path wrote `0001.png` into this crate's
    // folder**, on every run of the whole workspace.
    assert!(
        !std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("0001.png").exists(),
        "the stand-in left a file in the crate's folder"
    );
    let records = the_capture_records_read_from(&note.fields)
        .expect("the ending carries the capture records, read back whole");
    assert_eq!(records.len(), 1, "one attempt, one capture record");
    let record = &records[0];
    assert_eq!(
        Some(record.attempt.to_string().as_str()),
        note.fields
            .text(asked::THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD),
        "the record is the attempt the note points at"
    );
    for sink in [&record.sinks.ordinary, &record.sinks.complaints] {
        let on_disk = std::fs::metadata(&sink.path)
            .unwrap_or_else(|why| panic!("{} is not on the machine: {why}", sink.path));
        assert_eq!(
            sink.length,
            Some(on_disk.len()),
            "{} is named at its own size",
            sink.path
        );
    }
    assert!(
        record.ordinary.outcome.is_some(),
        "an ended attempt's outcome is written"
    );
    assert!(
        record.complaints.outcome.is_some(),
        "an ended attempt's outcome is written"
    );
}

// --- step 10c piece 8: the setup run, its worker and its boundaries ---

use smoozy_framing::framing::Fields;
use smoozy_framing::requests as asked;

/// **A stand-in for a Linux Blender, and it is named as one.**
///
/// It is a shell script that prints `Server Setup` B1's own line on its
/// ordinary stream, with a line of noise before it as the apt build printed.
/// **Nothing about a real Blender is proved by it** — what is proved is the
/// order, the records and the resumption around it.
const A_STAND_IN_FOR_A_LINUX_BLENDER: &str =
    "#!/bin/sh\nprintf 'Color management: using fallback\\n'\nprintf 'Blender 5.2.0\\n'\n";

/// **A stand-in for a Linux Blender archive, made with this computer's own
/// `tar`.**
///
/// **The unpack step is real and the archive is not Blender's.** `Server Setup`
/// R1 item 3's unpack is one invocation of `tar`, which this computer has; what
/// it unpacks here is a folder holding the stand-in above.
fn a_stand_in_archive_at(staged_at: &Path) {
    let making = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "a-stand-in-archive-{}",
        Identity::issue(IdentityKind::Operation)
            .expect("randomness")
            .as_text()
    ));
    let inside = making.join("blender-5.2.0-linux-x64");
    std::fs::create_dir_all(&inside).expect("the folder is made");
    let build = inside.join("blender");
    std::fs::write(&build, A_STAND_IN_FOR_A_LINUX_BLENDER).expect("the stand-in is written");
    let mut how_it_stands = std::fs::metadata(&build)
        .expect("it is there")
        .permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut how_it_stands, 0o755);
    std::fs::set_permissions(&build, how_it_stands).expect("it is runnable");
    std::fs::create_dir_all(staged_at.parent().expect("a staging area")).expect("the area is made");
    let made = Command::new("tar")
        .arg("-cf")
        .arg(staged_at)
        .arg("-C")
        .arg(&making)
        .arg("blender-5.2.0-linux-x64")
        .status()
        .expect("tar runs on this computer");
    assert!(made.success(), "the stand-in archive was made");
    std::fs::remove_dir_all(&making).ok();
}

/// One request naming a setup run.
fn about_the_run(run: &Identity) -> Fields {
    Fields::none().and(asked::THE_SETUP_RUN_FIELD, run.as_text())
}

/// **The state reading, asked until a boundary appears or the wait runs out.**
///
/// **The worker is a separate process and the step command is answered before
/// it has run** (`Server protocol and transfers` C8's *still in hand*), so the
/// desktop learns the step finished from this reading — which is the same one
/// it refreshes on.
fn the_state_until(
    area: &APackageArea,
    run: &Identity,
    a_boundary: &str,
) -> smoozy_framing::framing::Answered {
    let asked_at = Instant::now();
    loop {
        let read = asked_through_the_adapter(
            area,
            &a_request_of(asked::THE_SETUP_RUNS_STATE_REQUEST, about_the_run(run)),
        );
        let how_many: usize = read
            .fields
            .text(asked::HOW_MANY_BOUNDARIES_REACHED_FIELD)
            .and_then(|text| text.parse().ok())
            .unwrap_or(0);
        let reached: Vec<String> = (0..how_many)
            .filter_map(|at| {
                read.fields
                    .text(&asked::the_field_of_item(
                        asked::A_BOUNDARY_REACHED,
                        at,
                        asked::THE_BOUNDARY_FIELD,
                    ))
                    .map(str::to_string)
            })
            .collect();
        if reached.iter().any(|named| named == a_boundary) {
            return read;
        }
        assert!(
            asked_at.elapsed() < Duration::from_secs(60),
            "the boundary {a_boundary} never appeared; the run reached {reached:?} and its ending \
             is {:?} ({:?})",
            read.fields.text(asked::THE_RUNS_ENDING_FIELD),
            read.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// **The boundaries a state reading carries, with their marks.**
fn the_boundaries_in(read: &smoozy_framing::framing::Answered) -> Vec<(String, String)> {
    let how_many: usize = read
        .fields
        .text(asked::HOW_MANY_BOUNDARIES_REACHED_FIELD)
        .and_then(|text| text.parse().ok())
        .unwrap_or(0);
    (0..how_many)
        .filter_map(|at| {
            Some((
                read.fields
                    .text(&asked::the_field_of_item(
                        asked::A_BOUNDARY_REACHED,
                        at,
                        asked::THE_BOUNDARY_FIELD,
                    ))?
                    .to_string(),
                read.fields
                    .text(&asked::the_field_of_item(
                        asked::A_BOUNDARY_REACHED,
                        at,
                        asked::WHAT_THE_STEP_DID_FIELD,
                    ))?
                    .to_string(),
            ))
        })
        .collect()
}

/// **The open request for a run, carrying `Server Setup` Y4's composed list.**
fn opening_a_run(kind: &str, composed: &[&str]) -> Request {
    let mut fields = Fields::none()
        .and(asked::THE_KIND_OF_SETUP_FIELD, kind)
        .and(
            asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD,
            composed.len().to_string(),
        );
    for (at, boundary) in composed.iter().enumerate() {
        fields = fields.and(
            &asked::the_field_of_item(asked::A_COMPOSED_BOUNDARY, at, asked::THE_BOUNDARY_FIELD),
            *boundary,
        );
    }
    a_request_of(asked::THE_SETUP_RUN_IS_OPENED_REQUEST, fields)
}

/// **Step 10c piece 8, proved at a running supervisor: a setup run is opened,
/// driven one step at a time, and its boundaries are written with their marks
/// and their moments.**
///
/// **The fetch is not one of the steps here and that is deliberate**: it would
/// reach Blender's own downloads, which no test in this workspace does. B4's
/// fetch is proved with the program's own client against an HTTPS server on
/// this computer, in the test module `the_fetch_on_the_machines_own_line` of
/// `src/the_setup_worker.rs`.
///
/// **The archive and the build are stand-ins and are named as such.** This desk
/// is a Mac; what is proved is the worker's order, the records the supervisor
/// writes, and the rule that a step at or before the marker is never run again.
#[test]
fn a_setup_run_is_driven_one_step_at_a_time_and_its_boundaries_are_written() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    // The build is a script, so its library chain and loader lookup are
    // stand-ins too. This checks a successful final step on every test host.
    let tools = area.at.join("stand-in-tools");
    std::fs::create_dir_all(&tools).unwrap();
    for (name, script) in [
        (
            "ldd",
            "#!/bin/sh\nprintf 'libc.so => /stand-in/libc.so\\n'\n".to_string(),
        ),
        (
            "sh",
            format!(
                "#!/bin/sh\n# Stand-in for this loader lookup only: the library is present.\n\
             if [ \"$1\" = -c ] && [ \"$2\" = '{}' ]; then exit 0; fi\n\
             exec /bin/sh \"$@\"\n",
                smoozy_server_setup_machine::ready_action::WHAT_BLENDER_OPENS_BY_NAME[0]
                    .the_lookup
                    .replace('\'', "'\\''"),
            ),
        ),
    ] {
        let at = tools.join(name);
        std::fs::write(&at, script).unwrap();
        std::fs::set_permissions(&at, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    }
    let path = std::env::join_paths(std::iter::once(tools).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("the tools and the existing path");
    area.spawn_the_supervisor_with_path(Some(&path));
    area.waited_until_it_answers();
    let root = ARoot::recorded_in(&area);

    // **The run, with `Server Setup` Y4's list composed by the desktop.**
    let opening = opening_a_run(
        asked::THE_READY_ACTION,
        &[
            asked::THE_BOUNDARY_STARTED,
            asked::THE_BOUNDARY_TOOLING,
            asked::THE_BOUNDARY_BLENDER,
            asked::THE_BOUNDARY_DONE,
        ],
    );
    let run = opening.identity.clone();
    let opened = asked_through_the_adapter(&area, &opening);
    assert_eq!(
        opened.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND),
        "{:?}",
        opened.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );

    // **`Server Setup` Y4's *started*: the record exists and the machine is
    // untouched**, and the composed list reads back as the desktop composed it.
    let read = the_state_until(&area, &run, asked::THE_BOUNDARY_STARTED);
    assert_eq!(
        read.fields.text(asked::ON_THIS_MACHINE_FIELD),
        Some(asked::IT_IS_PRESENT)
    );
    assert_eq!(
        read.fields.text(asked::THE_RUNS_KIND_FIELD),
        Some(asked::THE_READY_ACTION)
    );
    assert_eq!(
        read.fields.text(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD),
        Some("4")
    );

    // The desktop sets this before the machine-changing work. The recovery
    // takes it over; after clearing it, the desktop writes done.
    let set = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
            about_the_run(&run),
        ),
    );
    assert_eq!(
        set.fields.text(asked::THE_SETUP_RUN_FIELD),
        Some(run.as_text().as_str())
    );

    // **The unpack, against a stand-in for the Linux archive.**
    let staging = format!("{}/blender/staging-setup", root.at.display());
    let staged_at =
        PathBuf::from(&staging).join(format!("{}-blender-5.2.0-linux-x64.tar.xz", run.as_text()));
    a_stand_in_archive_at(&staged_at);
    let step = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_ONE_SETUP_STEP_REQUEST,
            about_the_run(&run)
                .and(asked::THE_STEP_FIELD, asked::THE_UNPACK_STEP)
                .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.0"),
        ),
    );
    assert_eq!(
        step.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND),
        "C8: a step in flight is still in hand"
    );
    let read = the_state_until(&area, &run, asked::THE_BOUNDARY_TOOLING);
    assert_eq!(
        the_boundaries_in(&read),
        vec![
            (
                asked::THE_BOUNDARY_STARTED.to_string(),
                asked::IT_HAD_NOTHING_TO_DO.to_string()
            ),
            (
                asked::THE_BOUNDARY_TOOLING.to_string(),
                asked::IT_PERFORMED.to_string()
            ),
        ]
    );
    // **`Server Setup` Y3: the moment on the machine's own clock, and the clock
    // named beside it.**
    assert!(read
        .fields
        .text(&asked::the_field_of_item(
            asked::A_BOUNDARY_REACHED,
            1,
            asked::THE_BOUNDARY_AT_FIELD
        ))
        .is_some_and(|text| text.parse::<u64>().is_ok_and(|at| at > 0)));
    assert!(read
        .fields
        .text(&asked::the_field_of_item(
            asked::A_BOUNDARY_REACHED,
            1,
            asked::THE_BOUNDARY_CLOCK_FIELD
        ))
        .is_some());

    // **`Server Setup` R1 item 7: the staged build is asked its version in
    // staging and only on that answer renamed to its final folder.**
    asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_ONE_SETUP_STEP_REQUEST,
            about_the_run(&run)
                .and(asked::THE_STEP_FIELD, asked::THE_BUILD_IS_PUBLISHED_STEP)
                .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.0"),
        ),
    );
    let read = the_state_until(&area, &run, asked::THE_BOUNDARY_BLENDER);
    assert_eq!(the_boundaries_in(&read).len(), 3);
    let published = std::fs::read_dir(format!("{}/blender", root.at.display()))
        .expect("the Blender area is there")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .find(|named| named.contains("blender-5.2.0"))
        .expect("the build was published into `Server Setup` B9's area");
    // **B8, 2026-10-08: the folder is the version and the run's start**, never
    // the run's identity — the Creator's third notes on the test copy, note 4.
    assert!(published.starts_with("blender-5.2.0-"), "{published}");
    assert_eq!(
        published.len(),
        "blender-5.2.0-20261006-153212".len(),
        "{published}"
    );
    assert!(
        !published.contains(&run.as_text()),
        "B8: no run identity in {published}"
    );
    // **The rename consumes the staged material**, which is what makes the step
    // re-runnable: a version ask placed after the rename would leave a re-run
    // nothing to re-validate by content.
    assert!(
        !PathBuf::from(&staging)
            .join(format!("{}-unpacked", run.as_text()))
            .join("blender-5.2.0-linux-x64")
            .exists(),
        "the rename consumes the staged material"
    );

    // **`Server Setup` S4 item 3: a step at or before the marker is never run
    // again**, and the run ends saying so rather than doing it.
    asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_ONE_SETUP_STEP_REQUEST,
            about_the_run(&run)
                .and(asked::THE_STEP_FIELD, asked::THE_UNPACK_STEP)
                .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.0"),
        ),
    );
    let asked_at = Instant::now();
    let ended = loop {
        let read = asked_through_the_adapter(
            &area,
            &a_request_of(asked::THE_SETUP_RUNS_STATE_REQUEST, about_the_run(&run)),
        );
        if read.fields.text(asked::THE_RUNS_ENDING_FIELD).is_some() {
            break read;
        }
        assert!(
            asked_at.elapsed() < Duration::from_secs(60),
            "the run never ended"
        );
        std::thread::sleep(Duration::from_millis(100));
    };
    assert_eq!(
        ended.fields.text(asked::THE_RUNS_ENDING_FIELD),
        Some(asked::A_STEP_FAILED)
    );
    // **`Server Setup` S2a: the ending names the step that failed** — the
    // unpack, which is `Server Setup` R1's tooling step, named by the first
    // boundary that step reaches. Added 2026-09-21.
    assert_eq!(
        ended.fields.text(asked::THE_FAILED_STEP_FIELD),
        Some(asked::THE_BOUNDARY_TOOLING)
    );
    assert!(
        ended
            .fields
            .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
            .is_some_and(|why| why.contains("S4 item 3")),
        "the words say why: {:?}",
        ended.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    // **And the boundaries it did reach are still there**, which is what a
    // recovery inherits.
    assert_eq!(the_boundaries_in(&ended).len(), 3);

    // **`Server Setup` B8, 2026-10-08: a recovery past *blender* finds the
    // build where the publication put it**, by composing the same name from
    // the run's start its record holds. Were the name made from anything the
    // record does not hold, the final check would have nothing to read — step
    // 10c's fifth pass on Vast.
    let asked_at = Instant::now();
    let recovery = loop {
        let opening = a_request_of(
            asked::THE_SETUP_RUN_IS_OPENED_REQUEST,
            Fields::none()
                .and(asked::THE_RUN_TAKEN_OVER_FIELD, run.as_text())
                .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "0"),
        );
        let identity = opening.identity.clone();
        let opened = asked_through_the_adapter(&area, &opening);
        if opened.fields.text(asked::WHAT_BECAME_OF_IT_FIELD) == Some(asked::IT_IS_STILL_IN_HAND) {
            break identity;
        }
        assert!(
            asked_at.elapsed() < Duration::from_secs(60),
            "the recovery was never admitted: {:?}",
            opened.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        );
        std::thread::sleep(Duration::from_millis(200));
    };
    let accepted = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_ONE_SETUP_STEP_REQUEST,
            about_the_run(&recovery).and(asked::THE_STEP_FIELD, asked::THE_FINAL_VERIFICATION_STEP),
        ),
    );
    assert_eq!(
        accepted.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND),
        "the final check must be accepted: {:?}",
        accepted.fields
    );
    let asked_at = Instant::now();
    let checked = loop {
        let read = asked_through_the_adapter(
            &area,
            &a_request_of(
                asked::THE_SETUP_RUNS_STATE_REQUEST,
                about_the_run(&recovery),
            ),
        );
        if read.fields.text(asked::THE_STEP_FIELD).is_none() {
            break read;
        }
        assert!(
            asked_at.elapsed() < Duration::from_secs(60),
            "the recovery's check never ended"
        );
        std::thread::sleep(Duration::from_millis(100));
    };
    assert_eq!(
        checked.fields.text(asked::ON_THIS_MACHINE_FIELD),
        Some(asked::IT_IS_PRESENT)
    );
    assert_eq!(
        checked.fields.text(asked::THE_RUNS_ENDING_FIELD),
        None,
        "the final check failed: {:?}",
        checked.fields
    );
    assert_eq!(checked.fields.text(asked::THE_FAILED_STEP_FIELD), None);
    assert_eq!(
        checked.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD),
        None
    );
    assert_eq!(
        checked
            .fields
            .text(asked::THE_STEP_THE_WORDS_BELONG_TO_FIELD),
        Some(asked::THE_FINAL_VERIFICATION_STEP)
    );
    assert_eq!(
        the_boundaries_in(&checked).len(),
        3,
        "verification itself writes no done boundary"
    );

    // The desktop clears after a successful check, then sends a separate
    // request writing done (`setting_up.rs`, Step::Done).
    let cleared = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST,
            about_the_run(&recovery),
        ),
    );
    assert_eq!(
        cleared
            .fields
            .text(asked::THE_SETUP_RUN_THE_CLEAR_REMOVED_FIELD),
        Some(recovery.as_text().as_str())
    );
    let health = asked_through_the_adapter(&area, &a_health_request());
    assert_eq!(
        health
            .fields
            .text(smoozy_runtime::acting::readings::THE_SETUP_IN_PROGRESS_FIELD),
        None
    );
    let wrote = asked_through_the_adapter(
        &area,
        &writing_the_boundary(&recovery, asked::THE_BOUNDARY_DONE, asked::IT_PERFORMED),
    );
    assert_eq!(
        wrote.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME),
        "the desktop's done boundary was refused: {:?}",
        wrote.fields
    );
    let done = the_state_until(&area, &recovery, asked::THE_BOUNDARY_DONE);
    assert_eq!(
        done.fields.text(asked::THE_RUNS_ENDING_FIELD),
        Some(asked::IT_COMPLETED)
    );
    assert_eq!(the_boundaries_in(&done).len(), 4);
}

/// **`Server Setup` S4 item 2's recovery run, proved at a running supervisor:
/// it takes the earlier run over rather than clearing it, and it begins at the
/// boundary it inherited.**
#[test]
fn a_recovery_run_takes_the_earlier_run_over_and_inherits_its_boundary() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let root = ARoot::recorded_in(&area);

    let opening = opening_a_run(
        asked::THE_READY_ACTION,
        &[
            asked::THE_BOUNDARY_STARTED,
            asked::THE_BOUNDARY_TOOLING,
            asked::THE_BOUNDARY_DONE,
        ],
    );
    let first = opening.identity.clone();
    asked_through_the_adapter(&area, &opening);
    the_state_until(&area, &first, asked::THE_BOUNDARY_STARTED);

    // **`Server Setup` Y1: the desktop sets the machine-level state before the
    // first machine-changing step**, which on a run that fetches a Blender is
    // `Server Setup` R1 item 3's tooling install — the elevated act whose
    // worker half is the unpack below.
    let set = a_request_of(
        smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
        about_the_run(&first),
    );
    asked_through_the_adapter(&area, &set);

    let staging = format!("{}/blender/staging-setup", root.at.display());
    let staged_at = PathBuf::from(&staging).join(format!(
        "{}-blender-5.2.0-linux-x64.tar.xz",
        first.as_text()
    ));
    a_stand_in_archive_at(&staged_at);
    asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_ONE_SETUP_STEP_REQUEST,
            about_the_run(&first)
                .and(asked::THE_STEP_FIELD, asked::THE_UNPACK_STEP)
                .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.0"),
        ),
    );
    the_state_until(&area, &first, asked::THE_BOUNDARY_TOOLING);

    // **A step the marker is past ends the first run**, which is the state a
    // recovery meets: a run with a marker and no *done*.
    asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_ONE_SETUP_STEP_REQUEST,
            about_the_run(&first)
                .and(asked::THE_STEP_FIELD, asked::THE_UNPACK_STEP)
                .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.0"),
        ),
    );
    let asked_at = Instant::now();
    loop {
        let read = asked_through_the_adapter(
            &area,
            &a_request_of(asked::THE_SETUP_RUNS_STATE_REQUEST, about_the_run(&first)),
        );
        if read.fields.text(asked::THE_RUNS_ENDING_FIELD).is_some() {
            break;
        }
        assert!(
            asked_at.elapsed() < Duration::from_secs(60),
            "the first run never ended"
        );
        std::thread::sleep(Duration::from_millis(100));
    }

    // **The recovery: an ordinary command with its own operation identity,
    // naming the setup operation the state names, and carrying no kind.**
    let asked_at = Instant::now();
    let (recovery, opened) = loop {
        let opening = a_request_of(
            asked::THE_SETUP_RUN_IS_OPENED_REQUEST,
            Fields::none()
                .and(asked::THE_RUN_TAKEN_OVER_FIELD, first.as_text())
                .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "0"),
        );
        let identity = opening.identity.clone();
        let opened = asked_through_the_adapter(&area, &opening);
        if opened.fields.text(asked::WHAT_BECAME_OF_IT_FIELD) == Some(asked::IT_IS_STILL_IN_HAND) {
            break (identity, opened);
        }
        assert!(
            asked_at.elapsed() < Duration::from_secs(60),
            "the recovery was never admitted: {:?}",
            opened.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        );
        std::thread::sleep(Duration::from_millis(200));
    };
    let _ = opened;

    // **The taken-over run's record is closed with the kind's own ending**, and
    // the recovery's own record carries the inherited list and marker.
    let earlier = asked_through_the_adapter(
        &area,
        &a_request_of(asked::THE_SETUP_RUNS_STATE_REQUEST, about_the_run(&first)),
    );
    assert!(earlier.fields.text(asked::THE_RUNS_ENDING_FIELD).is_some());
    let taken = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_SETUP_RUNS_STATE_REQUEST,
            about_the_run(&recovery),
        ),
    );
    assert_eq!(
        taken.fields.text(asked::ON_THIS_MACHINE_FIELD),
        Some(asked::IT_IS_PRESENT)
    );
    assert_eq!(
        taken.fields.text(asked::THE_RUNS_KIND_FIELD),
        Some(asked::THE_READY_ACTION),
        "`Server Setup` Y6b: a recovery run takes its taken-over run's kind"
    );
    assert_eq!(
        taken.fields.text(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD),
        Some("3"),
        "`Server Setup` S4 item 2: the composed list is inherited"
    );
    let inherited = the_boundaries_in(&taken);
    assert_eq!(
        inherited.last().map(|(boundary, _)| boundary.as_str()),
        Some(asked::THE_BOUNDARY_TOOLING),
        "the recovery begins at the boundary it inherited"
    );

    // **`Server Setup` S4 item 2's other half: the state is re-pointed at the
    // recovery run's own operation identity, in the take-over's own act.**
    //
    // **It is what A11a admitted the recovery's worker on** — the worker start
    // above succeeded, and A11a admits *"the worker start of the setup run the
    // state itself names"* and nothing else. **Read off the health answer**,
    // which is V9's report of the state.
    let health = asked_through_the_adapter(&area, &a_health_request());
    assert_eq!(
        health
            .fields
            .text(smoozy_runtime::acting::readings::THE_SETUP_IN_PROGRESS_FIELD),
        Some(recovery.as_text().as_str()),
        "the state still names the run the recovery took over"
    );

    // **P8: a clear naming the run the state no longer names does not clear
    // it**, and the standing identity comes back instead.
    let refused = asked_through_the_adapter(
        &area,
        &a_request_of(
            smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST,
            about_the_run(&first),
        ),
    );
    assert_eq!(
        refused
            .fields
            .text(smoozy_runtime::acting::stores::THE_SETUP_RUN_FIELD),
        Some(recovery.as_text().as_str())
    );

    // **And the recovery can clear it, which is the whole point of the
    // re-point**: `Server Setup` S6's first clearing act, reached by the run
    // that took the state over.
    let cleared = asked_through_the_adapter(
        &area,
        &a_request_of(
            smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST,
            about_the_run(&recovery),
        ),
    );
    assert_eq!(
        cleared
            .fields
            .text(smoozy_framing::requests::THE_SETUP_RUN_THE_CLEAR_REMOVED_FIELD),
        Some(recovery.as_text().as_str())
    );
    let after = asked_through_the_adapter(&area, &a_health_request());
    assert_eq!(
        after
            .fields
            .text(smoozy_runtime::acting::readings::THE_SETUP_IN_PROGRESS_FIELD),
        None
    );
}

/// **`Server Setup` S6's second clear, at a running supervisor: a recovery the
/// user asked to abandon the machine-changing work.** Added 2026-09-22, out of
/// step 10c's third pass, where a driver step that failed the same way every
/// time left the user no way on but destroying the server.
///
/// **What is proved, in order.** The abandon takes the standing state over as
/// any recovery does (`Server Setup` S4 item 2); it is refused every step but
/// the final verification and every boundary; the verification's report ends
/// the run *abandoned* whatever it found, naming no failed step; and the
/// desktop's clear, `Rented-server runtime` P3's second act, then clears the
/// state — `Server Setup` S6's second way the state is ever cleared.
#[test]
fn an_abandoned_setup_takes_the_state_over_runs_its_final_verification_and_is_cleared() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let _root = ARoot::recorded_in(&area);

    // **The third pass's state**: a run that set the state and whose driver
    // step failed, its words recorded by the desk.
    let opening = opening_a_run(
        asked::THE_READY_ACTION,
        &[
            asked::THE_BOUNDARY_STARTED,
            asked::THE_BOUNDARY_DRIVER_PLACED,
            asked::THE_BOUNDARY_RESTART_ISSUED,
            asked::THE_BOUNDARY_CARD_ANSWERING,
            asked::THE_BOUNDARY_DONE,
        ],
    );
    let first = opening.identity.clone();
    asked_through_the_adapter(&area, &opening);
    the_state_until(&area, &first, asked::THE_BOUNDARY_STARTED);
    asked_through_the_adapter(
        &area,
        &a_request_of(
            smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
            about_the_run(&first),
        ),
    );
    asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_FAILED_STEP_IS_RECORDED_REQUEST,
            about_the_run(&first)
                .and(
                    asked::THE_FAILED_STEP_FIELD,
                    asked::THE_BOUNDARY_DRIVER_PLACED,
                )
                .and(
                    asked::THE_MACHINES_OWN_WORDS_FIELD,
                    "E: Failed to fetch 404  Not Found\n",
                ),
        ),
    );

    // **An abandon that names no run to take over is refused**: `Server Setup` S6's abandon
    // is a recovery.
    let alone = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_SETUP_RUN_IS_OPENED_REQUEST,
            Fields::none()
                .and(asked::THE_KIND_OF_SETUP_FIELD, asked::THE_READY_ACTION)
                .and(
                    asked::THE_RECOVERY_ABANDONS_FIELD,
                    asked::THE_RECOVERY_ABANDONS,
                )
                .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "0"),
        ),
    );
    assert_eq!(
        alone.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::THERE_IS_NO_RECORD_OF_IT)
    );

    // **The abandon: a recovery naming the run the state names, marked.** It
    // waits while the earlier run's group is still recorded as live, which is
    // `Server Setup` S5's safe direction.
    let asked_at = Instant::now();
    let abandon = loop {
        let opening = a_request_of(
            asked::THE_SETUP_RUN_IS_OPENED_REQUEST,
            Fields::none()
                .and(asked::THE_RUN_TAKEN_OVER_FIELD, first.as_text())
                .and(
                    asked::THE_RECOVERY_ABANDONS_FIELD,
                    asked::THE_RECOVERY_ABANDONS,
                )
                .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "0"),
        );
        let identity = opening.identity.clone();
        let opened = asked_through_the_adapter(&area, &opening);
        if opened.fields.text(asked::WHAT_BECAME_OF_IT_FIELD) == Some(asked::IT_IS_STILL_IN_HAND) {
            assert_eq!(
                opened.fields.text(asked::THE_RECOVERY_ABANDONS_FIELD),
                Some(asked::THE_RECOVERY_ABANDONS),
                "the abandon was not said back"
            );
            break identity;
        }
        assert!(
            asked_at.elapsed() < Duration::from_secs(60),
            "the abandon was never admitted: {:?}",
            opened.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        );
        std::thread::sleep(Duration::from_millis(200));
    };
    let health = asked_through_the_adapter(&area, &a_health_request());
    assert_eq!(
        health
            .fields
            .text(smoozy_runtime::acting::readings::THE_SETUP_IN_PROGRESS_FIELD),
        Some(abandon.as_text().as_str()),
        "the abandon took the state over"
    );

    // **Nothing but the final verification, and no boundary.**
    let a_step = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_ONE_SETUP_STEP_REQUEST,
            about_the_run(&abandon)
                .and(asked::THE_STEP_FIELD, asked::THE_UNPACK_STEP)
                .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.0"),
        ),
    );
    assert_eq!(
        a_step.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::THERE_IS_NO_RECORD_OF_IT),
        "an abandon was commanded an unpack"
    );
    let a_boundary = asked_through_the_adapter(
        &area,
        &writing_the_boundary(&abandon, asked::THE_BOUNDARY_DONE, asked::IT_PERFORMED),
    );
    assert_eq!(
        a_boundary.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::THERE_IS_NO_RECORD_OF_IT),
        "an abandon was given *done*"
    );

    // **The final verification, and the ending it leaves.**
    asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_ONE_SETUP_STEP_REQUEST,
            about_the_run(&abandon).and(asked::THE_STEP_FIELD, asked::THE_FINAL_VERIFICATION_STEP),
        ),
    );
    let asked_at = Instant::now();
    let ended = loop {
        let read = asked_through_the_adapter(
            &area,
            &a_request_of(asked::THE_SETUP_RUNS_STATE_REQUEST, about_the_run(&abandon)),
        );
        if read.fields.text(asked::THE_RUNS_ENDING_FIELD).is_some() {
            break read;
        }
        assert!(
            asked_at.elapsed() < Duration::from_secs(60),
            "the abandon never ended"
        );
        std::thread::sleep(Duration::from_millis(100));
    };
    assert_eq!(
        ended.fields.text(asked::THE_RUNS_ENDING_FIELD),
        Some(asked::IT_WAS_ABANDONED)
    );
    assert_eq!(
        ended.fields.text(asked::THE_FAILED_STEP_FIELD),
        None,
        "an abandon names no failed step"
    );
    assert!(
        !the_boundaries_in(&ended)
            .iter()
            .any(|(boundary, _)| boundary == asked::THE_BOUNDARY_DONE),
        "an abandon reached *done*"
    );

    // **The desktop's clear, and the state is gone.**
    let cleared = asked_through_the_adapter(
        &area,
        &a_request_of(
            smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST,
            about_the_run(&abandon),
        ),
    );
    assert_eq!(
        cleared
            .fields
            .text(smoozy_framing::requests::THE_SETUP_RUN_THE_CLEAR_REMOVED_FIELD),
        Some(abandon.as_text().as_str())
    );
    let after = asked_through_the_adapter(&area, &a_health_request());
    assert_eq!(
        after
            .fields
            .text(smoozy_runtime::acting::readings::THE_SETUP_IN_PROGRESS_FIELD),
        None
    );
}

/// One boundary written from the desktop, with `Server Setup` Y4a's mark.
fn writing_the_boundary(run: &Identity, boundary: &str, mark: &str) -> Request {
    a_request_of(
        asked::THE_BOUNDARY_IS_WRITTEN_REQUEST,
        about_the_run(run)
            .and(asked::THE_BOUNDARY_FIELD, boundary)
            .and(asked::WHAT_THE_STEP_DID_FIELD, mark),
    )
}

/// **Step 10c: `Server Setup` Y7's other half at a running supervisor — the
/// five boundaries the desktop writes, and a run that reaches
/// `Server Setup` Y4's *done*.**
///
/// **Before this route no request carried a boundary from the desktop**, so
/// *driver placed*, *restart issued*, *card answering*, *telegram* and *done*
/// could not be reached on any machine and no setup run could complete its own
/// list.
///
/// **The run is composed of *started* and the five and nothing the worker
/// feeds**, and the desktop writes only its own; what is proved is the route,
/// the record, the marks, the moments and the ending. *(Until 2026-09-22 it was
/// composed with all ten and reached *done* with four of them unreached — which
/// is exactly the false *done* step 10c's third pass on Vast met, and which
/// `Server Setup` Y3 now refuses; the refusal is
/// [`a_boundary_is_never_written_past_one_its_run_has_not_reached`]'s.)*
#[test]
fn the_desktop_writes_the_five_boundaries_no_worker_report_can_reach() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let _root = ARoot::recorded_in(&area);

    let opening = opening_a_run(
        asked::THE_READY_ACTION,
        &[
            asked::THE_BOUNDARY_STARTED,
            asked::THE_BOUNDARY_DRIVER_PLACED,
            asked::THE_BOUNDARY_RESTART_ISSUED,
            asked::THE_BOUNDARY_CARD_ANSWERING,
            asked::THE_BOUNDARY_TELEGRAM,
            asked::THE_BOUNDARY_DONE,
        ],
    );
    let run = opening.identity.clone();
    let opened = asked_through_the_adapter(&area, &opening);
    assert_eq!(
        opened.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND),
        "{:?}",
        opened.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    the_state_until(&area, &run, asked::THE_BOUNDARY_STARTED);

    // **A boundary this machine's own worker report feeds is refused**, and so
    // is `Server Setup` Y4's *started*, which the supervisor wrote itself.
    for its_own in [asked::THE_BOUNDARY_FETCHED, asked::THE_BOUNDARY_STARTED] {
        let refused = asked_through_the_adapter(
            &area,
            &writing_the_boundary(&run, its_own, asked::IT_PERFORMED),
        );
        assert_eq!(
            refused.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::THERE_IS_NO_RECORD_OF_IT),
            "{its_own} was written from the desktop"
        );
    }

    // **A mark outside `Server Setup` Y4a's closed set of two is refused**, and
    // no third is invented for it.
    let refused = asked_through_the_adapter(
        &area,
        &writing_the_boundary(&run, asked::THE_BOUNDARY_DRIVER_PLACED, "half done"),
    );
    assert_eq!(
        refused.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::THERE_IS_NO_RECORD_OF_IT)
    );

    // **The five, in `Server Setup` Y4's order.**
    let the_five = [
        (asked::THE_BOUNDARY_DRIVER_PLACED, asked::IT_PERFORMED),
        (
            asked::THE_BOUNDARY_RESTART_ISSUED,
            asked::IT_HAD_NOTHING_TO_DO,
        ),
        (asked::THE_BOUNDARY_CARD_ANSWERING, asked::IT_PERFORMED),
        (asked::THE_BOUNDARY_TELEGRAM, asked::IT_PERFORMED),
        (asked::THE_BOUNDARY_DONE, asked::IT_PERFORMED),
    ];
    let mut last_written = None;
    for (boundary, mark) in the_five {
        let written = writing_the_boundary(&run, boundary, mark);
        let wrote = asked_through_the_adapter(&area, &written);
        assert_eq!(
            wrote.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::IT_HAS_A_RECORDED_OUTCOME),
            "{boundary} was not written: {:?}",
            wrote.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        );
        // **`Server Setup` Y3: the moment is the machine's own and the clock is
        // named beside it.** Nothing about it was carried in the request.
        assert!(wrote
            .fields
            .text(asked::THE_BOUNDARY_AT_FIELD)
            .is_some_and(|text| text.parse::<u64>().is_ok_and(|at| at > 0)));
        assert!(wrote.fields.text(asked::THE_BOUNDARY_CLOCK_FIELD).is_some());
        last_written = Some(written);
    }

    // **`Server protocol and transfers` C4: a repeat of the identity never
    // writes again**, and C8 says what became of it.
    let again = asked_through_the_adapter(&area, &last_written.expect("a write was made"));
    assert_eq!(
        again.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME)
    );

    // **The record reads back with all six boundaries and their marks**, and
    // `Server Setup` Y4's *done* ended the run as completed.
    let read = the_state_until(&area, &run, asked::THE_BOUNDARY_DONE);
    assert_eq!(
        the_boundaries_in(&read),
        vec![
            (
                asked::THE_BOUNDARY_STARTED.to_string(),
                asked::IT_HAD_NOTHING_TO_DO.to_string()
            ),
            (
                asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                asked::IT_PERFORMED.to_string()
            ),
            (
                asked::THE_BOUNDARY_RESTART_ISSUED.to_string(),
                asked::IT_HAD_NOTHING_TO_DO.to_string()
            ),
            (
                asked::THE_BOUNDARY_CARD_ANSWERING.to_string(),
                asked::IT_PERFORMED.to_string()
            ),
            (
                asked::THE_BOUNDARY_TELEGRAM.to_string(),
                asked::IT_PERFORMED.to_string()
            ),
            (
                asked::THE_BOUNDARY_DONE.to_string(),
                asked::IT_PERFORMED.to_string()
            ),
        ]
    );
    assert_eq!(
        read.fields.text(asked::THE_RUNS_ENDING_FIELD),
        Some(asked::IT_COMPLETED)
    );

    // **`Server Setup` Y8 and `Server Setup` Y3: a run that is over is given no
    // boundary at all, so nothing can be written past the last it passed.**
    let after = asked_through_the_adapter(
        &area,
        &writing_the_boundary(&run, asked::THE_BOUNDARY_DONE, asked::IT_PERFORMED),
    );
    assert_eq!(
        after.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::THERE_IS_NO_RECORD_OF_IT)
    );
}

/// **`Server Setup` Y3 at a running supervisor: a boundary means every step up
/// to it is done, so nothing is written or run past a composed boundary the run
/// has not reached — *done* least of all.** Added 2026-09-22.
///
/// **This is step 10c's third pass on Vast, as the machine met it.** A Blender
/// install fetched its archive and failed at the unpack; *Set up this server*
/// then took the install over, was commanded the final verification with
/// *tooling* and *blender* never reached, and wrote *done* — and the screen
/// said the server was ready to render with no Blender on it. Here a run
/// composed as a Blender install is refused both the final verification and
/// *done* while *fetched* and the rest are unreached, and a recovery that takes
/// it over inherits the same refusal.
/// **The 38's piece 6, at a running supervisor: the Set up card's reading
/// names the ready action's retained run, the last job to end and the finished
/// job's device line with what it asked for — after a restart, to a desktop
/// that drove nothing** (`Server Setup` Y6a, Y6b, W3a, W4).
///
/// **Undone, this fails at the first ask**: the program refused the request by
/// name, and nothing else names a run that finished — the health answer names
/// none once the state is cleared at *done* — so a copy opened after the setup
/// read *No setup read from this server* (`Open Threads`).
#[test]
fn the_set_up_cards_reading_names_the_ready_run_and_the_last_finished_job_after_a_restart() {
    use smoozy_queue_history_machine::acting::note::{
        TheEndingOnTheNote, TheOutcomeAsWritten, ThePerJobNotesOnTheMachine,
        TheStatementAtTheEnding, WhatTheFigureIs, THE_AREA_THE_NOTES_SIT_IN,
    };
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let root = ARoot::recorded_in(&area);
    let the_cards_reading = |area: &APackageArea| {
        asked_through_the_adapter(
            area,
            &a_request_of(
                asked::THE_SET_UP_CARDS_READING_REQUEST,
                smoozy_framing::framing::Fields::none(),
            ),
        )
    };

    // **A fresh machine**: no ready-action record, no job ended, the role off.
    let fresh = the_cards_reading(&area);
    assert_eq!(fresh.the_request_name_it_says_it_does_not_answer(), None);
    assert_eq!(fresh.fields.text(asked::THE_READY_ACTIONS_RUN_FIELD), None);
    assert_eq!(
        fresh.fields.text(asked::THE_JOBS_THAT_ENDED_HERE_FIELD),
        Some(asked::NO_JOB_HAS_ENDED_HERE)
    );
    assert_eq!(
        fresh
            .fields
            .text(asked::THE_FINISHED_JOB_WITH_A_DEVICE_LINE_FIELD),
        None
    );
    assert_eq!(
        fresh.fields.text(asked::THE_TELEGRAM_ROLES_STATE_FIELD),
        Some(asked::THE_ROLE_IS_DISABLED)
    );

    // **A ready action, run to *done*.**
    let opening = opening_a_run(
        asked::THE_READY_ACTION,
        &[asked::THE_BOUNDARY_STARTED, asked::THE_BOUNDARY_DONE],
    );
    let run = opening.identity.clone();
    asked_through_the_adapter(&area, &opening);
    the_state_until(&area, &run, asked::THE_BOUNDARY_STARTED);
    asked_through_the_adapter(
        &area,
        &writing_the_boundary(&run, asked::THE_BOUNDARY_DONE, asked::IT_PERFORMED),
    );
    let done = the_state_until(&area, &run, asked::THE_BOUNDARY_DONE);
    assert_eq!(
        done.fields.text(asked::THE_RUNS_ENDING_FIELD),
        Some(asked::IT_COMPLETED)
    );

    // **A job that finished on the processor, its scene set to the cards.**
    // Its manifest asks for nothing itself, so the ask is the scene's own in
    // the reading attempt 1 took (`Render Settings` D1).
    let job = root.a_published_job();
    let record_directory = root.at.join("job-records").join(job.as_text());
    let manifest = smoozy_job_files_machine::manifest::SubmissionManifest::read_back(
        &std::fs::read(
            record_directory.join(smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME),
        )
        .expect("the manifest"),
    )
    .expect("it reads back");
    let mut printed: smoozy_job_files_machine::reading_script::ThePrintedReading =
        serde_json::from_str(&the_document_a_reading_prints()).expect("it reads");
    printed.settings.0["scenes"][0]["device"] = serde_json::json!("TheCards");
    let enters_at = smoozy_job_files_machine::main_output::WhereTheRenderEnters {
        scene: smoozy_job_files_machine::reading::ArtistsText::of(b"Scene".to_vec()),
        view_layer: smoozy_job_files_machine::reading::ArtistsText::of(b"ViewLayer".to_vec()),
    };
    let folder = root.at.display().to_string();
    let composed = smoozy_job_files_machine::composing::the_reading_and_everything_derived_from(
        printed,
        smoozy_job_files_machine::composing::WhatTheReadingIsComposedAgainst {
            the_folder_the_blend_sits_in: folder.as_bytes(),
            job_output_folder: manifest
                .job_output_folder
                .the_path_it_names()
                .expect("named"),
            enters_at: &enters_at,
            the_jobs_pins: &[],
        },
    )
    .expect("the reading composes");
    std::fs::write(
        record_directory
            .join(smoozy_job_files_machine::manifest::the_recorded_readings_file_name(1)),
        serde_json::to_vec(&composed).expect("it renders"),
    )
    .expect("the recorded reading is written");
    let mut notes = ThePerJobNotesOnTheMachine::at(the_record_at(
        &area.at.display().to_string(),
        THE_AREA_THE_NOTES_SIT_IN,
    ));
    notes.create(&job, "Kitchen").expect("the note");
    let the_line = "Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]";
    notes
        .the_outcome_was_written(
            &job,
            TheEndingOnTheNote {
                outcome: TheOutcomeAsWritten::Finished,
                exit_status: Some(0),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                no_render_began: false,
                the_device_line_as_observed: Some(the_line.to_string()),
            },
            smoozy_state::observed::Reading::taken(
                (),
                std::time::SystemTime::now(),
                smoozy_state::observed::Clock::AMachine(smoozy_state::observed::MachineClock(
                    area.installation.as_text(),
                )),
            ),
            WhatTheFigureIs::It(3),
            TheStatementAtTheEnding::NoneWasTaken {
                why: "a stand-in".to_string(),
            },
            smoozy_queue_history_machine::acting::note::TheFreeSpaceAtTheEnding::NoneWasTaken {
                why: "a stand-in".to_string(),
            },
        )
        .expect("the ending is written");

    // **A restart**, so nothing this supervisor held is what answers.
    area.kill_the_supervisor();
    area.start_the_supervisor();
    let health = asked_through_the_adapter(&area, &a_health_request());
    assert_eq!(
        health
            .fields
            .text(smoozy_runtime::acting::readings::THE_SETUP_IN_PROGRESS_FIELD),
        None,
        "the state was cleared at done, so the health answer names no run"
    );

    let read = the_cards_reading(&area);
    assert_eq!(
        read.fields.text(asked::THE_READY_ACTIONS_RUN_FIELD),
        Some(run.as_text().as_str()),
        "the ready action's retained run is named: {:?}",
        read.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    assert_eq!(
        read.fields.text(asked::THE_JOBS_THAT_ENDED_HERE_FIELD),
        Some(asked::SOME_JOBS_HAVE_ENDED_HERE)
    );
    assert_eq!(
        read.fields.text(asked::THE_MOST_RECENT_JOB_TO_END_FIELD),
        Some(job.as_text().as_str())
    );
    assert_eq!(
        read.fields
            .text(asked::THE_FINISHED_JOB_WITH_A_DEVICE_LINE_FIELD),
        Some(job.as_text().as_str())
    );
    assert_eq!(
        read.fields.text(asked::THE_DEVICE_LINE_FIELD),
        Some(the_line)
    );
    assert_eq!(
        read.fields.text(asked::THE_DEVICE_ASKED_FOR_FIELD),
        Some(asked::IT_ASKED_FOR_THE_CARDS),
        "the ask is the job's scene's own, read off attempt 1's reading"
    );
    // **And the run it names reads back whole by the run's own reading.**
    let state = asked_through_the_adapter(
        &area,
        &a_request_of(asked::THE_SETUP_RUNS_STATE_REQUEST, about_the_run(&run)),
    );
    assert_eq!(
        state.fields.text(asked::THE_RUNS_ENDING_FIELD),
        Some(asked::IT_COMPLETED)
    );
    assert_eq!(the_boundaries_in(&state).len(), 2);
}

#[test]
fn a_boundary_is_never_written_past_one_its_run_has_not_reached() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let _root = ARoot::recorded_in(&area);

    let composed = [
        asked::THE_BOUNDARY_STARTED,
        asked::THE_BOUNDARY_FETCHED,
        asked::THE_BOUNDARY_TOOLING,
        asked::THE_BOUNDARY_BLENDER,
        asked::THE_BOUNDARY_DONE,
    ];
    let opening = opening_a_run(asked::A_BLENDER_INSTALL, &composed);
    let first = opening.identity.clone();
    let opened = asked_through_the_adapter(&area, &opening);
    assert_eq!(
        opened.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND),
        "{:?}",
        opened.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    the_state_until(&area, &first, asked::THE_BOUNDARY_STARTED);

    let the_two_refusals = |run: &Identity| {
        let verification = asked_through_the_adapter(
            &area,
            &a_request_of(
                asked::THE_ONE_SETUP_STEP_REQUEST,
                about_the_run(run).and(asked::THE_STEP_FIELD, asked::THE_FINAL_VERIFICATION_STEP),
            ),
        );
        assert_eq!(
            verification.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::THERE_IS_NO_RECORD_OF_IT),
            "the final verification ran with the Blender half unreached"
        );
        assert!(verification
            .fields
            .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
            .is_some_and(|words| words.contains(asked::THE_BOUNDARY_FETCHED)));
        let done = asked_through_the_adapter(
            &area,
            &writing_the_boundary(run, asked::THE_BOUNDARY_DONE, asked::IT_PERFORMED),
        );
        assert_eq!(
            done.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::THERE_IS_NO_RECORD_OF_IT),
            "*done* was written with the Blender half unreached"
        );
    };
    the_two_refusals(&first);

    // **The install stops there**, its state standing, as the third pass's did.
    asked_through_the_adapter(
        &area,
        &a_request_of(
            smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
            about_the_run(&first),
        ),
    );
    asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_FAILED_STEP_IS_RECORDED_REQUEST,
            about_the_run(&first)
                .and(asked::THE_FAILED_STEP_FIELD, asked::THE_BOUNDARY_TOOLING)
                .and(
                    asked::THE_MACHINES_OWN_WORDS_FIELD,
                    "the unpack was commanded with no version to name its archive",
                ),
        ),
    );

    // **A recovery takes it over — composed as the ready action's *started,
    // done*, which is what *Set up this server* sends — and inherits the
    // install's list, so it is refused exactly as the install was.** While the
    // stopped run's worker is still going, the refusal says so in its own
    // field, which is `Server Setup` S5's wait.
    let asked_at = Instant::now();
    let recovery = loop {
        let mut fields = Fields::none()
            .and(asked::THE_RUN_TAKEN_OVER_FIELD, first.as_text())
            .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "2");
        for (at, boundary) in [asked::THE_BOUNDARY_STARTED, asked::THE_BOUNDARY_DONE]
            .into_iter()
            .enumerate()
        {
            fields = fields.and(
                &asked::the_field_of_item(
                    asked::A_COMPOSED_BOUNDARY,
                    at,
                    asked::THE_BOUNDARY_FIELD,
                ),
                boundary,
            );
        }
        let opening = a_request_of(asked::THE_SETUP_RUN_IS_OPENED_REQUEST, fields);
        let identity = opening.identity.clone();
        let opened = asked_through_the_adapter(&area, &opening);
        if opened.fields.text(asked::WHAT_BECAME_OF_IT_FIELD) == Some(asked::IT_IS_STILL_IN_HAND) {
            break identity;
        }
        assert!(
            opened
                .fields
                .text(asked::THE_RUN_THAT_STOPPED_IS_STILL_RUNNING_FIELD)
                .is_some(),
            "the recovery was refused for some other reason: {:?}",
            opened.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        );
        assert!(
            asked_at.elapsed() < Duration::from_secs(60),
            "the recovery was never admitted"
        );
        std::thread::sleep(Duration::from_millis(200));
    };
    let read = the_state_until(&area, &recovery, asked::THE_BOUNDARY_STARTED);
    assert_eq!(
        read.fields.text(asked::THE_RUNS_KIND_FIELD),
        Some(asked::A_BLENDER_INSTALL)
    );
    the_two_refusals(&recovery);
    let read = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_SETUP_RUNS_STATE_REQUEST,
            about_the_run(&recovery),
        ),
    );
    assert_ne!(
        read.fields.text(asked::THE_RUNS_ENDING_FIELD),
        Some(asked::IT_COMPLETED)
    );
    assert!(!the_boundaries_in(&read)
        .iter()
        .any(|(boundary, _)| boundary == asked::THE_BOUNDARY_DONE));
}

/// **Step 10c: `Server Setup` Y8's stop, as the machine's own durable desired
/// state — proved against the real supervisor and its real setup worker.**
///
/// **Three things are proved and the third is the point.** The stop rides
/// `Rented-server runtime` X8's ordinary command path and writes the run's
/// desired state; **the setup worker reads it at a boundary and ends there**,
/// with the kind's own ending *left stopped*; and **nothing is cleared** —
/// `Server Setup` Y8b holds `Server Setup` S6's clearing set at two and a stop
/// is neither, so the machine-level state stands and the interrupted job stays
/// held back.
///
/// **No signal is sent to anything**, which is `Server Setup` Y8's own rule and
/// `Rented-server runtime` X15's: termination for this kind is at a recorded
/// safe boundary. What the worker meets is its next ask for work.
#[test]
fn a_stopped_setup_run_ends_at_its_boundary_and_clears_nothing() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let _root = ARoot::recorded_in(&area);

    let opening = opening_a_run(
        asked::THE_READY_ACTION,
        &[
            asked::THE_BOUNDARY_STARTED,
            asked::THE_BOUNDARY_DRIVER_PLACED,
            asked::THE_BOUNDARY_DONE,
        ],
    );
    let run = opening.identity.clone();
    let opened = asked_through_the_adapter(&area, &opening);
    assert_eq!(
        opened.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND),
        "{:?}",
        opened.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    the_state_until(&area, &run, asked::THE_BOUNDARY_STARTED);

    // **`Server Setup` Y1's machine-level state, standing under this run.**
    let set = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
    )
    .carrying(about_the_run(&run));
    assert_eq!(
        asked_through_the_adapter(&area, &set)
            .fields
            .text(asked::THE_SETUP_RUN_FIELD),
        Some(run.as_text().as_str())
    );

    // **The run reached a boundary before the stop**, so there is something for
    // `Server Setup` S4's recovery to take over from.
    let wrote = asked_through_the_adapter(
        &area,
        &writing_the_boundary(&run, asked::THE_BOUNDARY_DRIVER_PLACED, asked::IT_PERFORMED),
    );
    assert_eq!(
        wrote.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME),
        "{:?}",
        wrote.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );

    // **`Rented-server runtime` X7: the stop is its own command with its own
    // operation identity, naming the target's.**
    let stopping = a_request_of(asked::THE_SETUP_RUN_IS_STOPPED_REQUEST, about_the_run(&run));
    let stopped = asked_through_the_adapter(&area, &stopping);
    assert_eq!(
        stopped.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME),
        "the stop was not taken: {:?}",
        stopped.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    assert_eq!(
        stopped.fields.text(asked::ON_THIS_MACHINE_FIELD),
        Some(asked::IT_IS_PRESENT),
        "the supervisor did not recognise the run as its own open one"
    );
    // **`Server protocol and transfers` C4 and C8: a repeat writes nothing
    // twice** and is answered as the record stands.
    let again = asked_through_the_adapter(&area, &stopping);
    assert_eq!(
        again.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME)
    );

    // **The worker ends at its next boundary**, which is the next time it asks
    // the supervisor what to do. Nothing was signalled.
    let asked_at = Instant::now();
    let read = loop {
        let read = asked_through_the_adapter(
            &area,
            &a_request_of(asked::THE_SETUP_RUNS_STATE_REQUEST, about_the_run(&run)),
        );
        if read.fields.text(asked::THE_RUNS_ENDING_FIELD) == Some(asked::IT_WAS_STOPPED) {
            break read;
        }
        assert!(
            asked_at.elapsed() < Duration::from_secs(60),
            "the run never ended stopped; it ended {:?}",
            read.fields.text(asked::THE_RUNS_ENDING_FIELD)
        );
        std::thread::sleep(Duration::from_millis(100));
    };

    // **What it leaves is what a recovery takes over**: the composed list and
    // the marker stand, and the marker is the boundary the run really reached.
    assert_eq!(
        the_boundaries_in(&read),
        vec![
            (
                asked::THE_BOUNDARY_STARTED.to_string(),
                asked::IT_HAD_NOTHING_TO_DO.to_string()
            ),
            (
                asked::THE_BOUNDARY_DRIVER_PLACED.to_string(),
                asked::IT_PERFORMED.to_string()
            ),
        ],
        "a stopped run lost or gained a boundary"
    );

    // **`Server Setup` Y8b and `Server Setup` T6: nothing is cleared.** The
    // machine-level state stands, naming the run that set it.
    let health = asked_through_the_adapter(&area, &a_health_request());
    assert_eq!(
        health
            .fields
            .text(smoozy_runtime::acting::readings::THE_SETUP_IN_PROGRESS_FIELD),
        Some(run.as_text().as_str()),
        "stopping a setup cleared the machine-level state"
    );

    // **And the run is over**, so nothing can be written past the boundary it
    // stopped at.
    let after = asked_through_the_adapter(
        &area,
        &writing_the_boundary(&run, asked::THE_BOUNDARY_DONE, asked::IT_PERFORMED),
    );
    assert_eq!(
        after.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::THERE_IS_NO_RECORD_OF_IT)
    );
}

/// **`Server Setup` S2a's desk half, proved at a running supervisor: a step the
/// desktop held has failed, and the run's recorded ending says so — *a step
/// failed*, the step named, the words whole — and still says so after
/// `Server Setup` Y7's window has closed.** Added 2026-09-21.
///
/// **Why the window is waited out rather than assumed.** Before this request
/// existed, an elevated step's failure left the run open on the machine, and
/// the window then ended it as *ended at its boundary, not completed* with no
/// words — which is what the card and panel then showed. **So the proof that
/// the machine tells the truth is a reading taken after the window**, with no
/// refresh in between. It costs this test about a minute and a half, and it is
/// the only test here that waits that long.
///
/// **And the desk never writes over the machine**: once the run has an ending,
/// a second failure is refused and the ending stands.
#[test]
fn a_step_that_failed_at_the_desk_is_recorded_with_its_words_and_outlasts_the_window() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let _root = ARoot::recorded_in(&area);

    let opening = opening_a_run(
        asked::THE_READY_ACTION,
        &[
            asked::THE_BOUNDARY_STARTED,
            asked::THE_BOUNDARY_DRIVER_PLACED,
            asked::THE_BOUNDARY_DONE,
        ],
    );
    let run = opening.identity.clone();
    let opened = asked_through_the_adapter(&area, &opening);
    assert_eq!(
        opened.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND),
        "{:?}",
        opened.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    the_state_until(&area, &run, asked::THE_BOUNDARY_STARTED);

    // **The words as an elevated package install prints them**: two lines,
    // quotes and a trailing newline, which must come back exactly.
    let the_words = "Reading package lists...\nE: Unable to locate package \"nvidia-driver-999\"\n";
    let failing = a_request_of(
        asked::THE_FAILED_STEP_IS_RECORDED_REQUEST,
        about_the_run(&run)
            .and(
                asked::THE_FAILED_STEP_FIELD,
                asked::THE_BOUNDARY_DRIVER_PLACED,
            )
            .and(asked::THE_MACHINES_OWN_WORDS_FIELD, the_words),
    );
    let recorded = asked_through_the_adapter(&area, &failing);
    assert_eq!(
        recorded.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME),
        "the failure was not taken: {:?}",
        recorded.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    // **`Server protocol and transfers` C4 and C8: a repeat writes nothing
    // twice** and is answered as the record stands.
    assert_eq!(
        asked_through_the_adapter(&area, &failing)
            .fields
            .text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME)
    );

    let the_ending_is_the_failure = |read: &smoozy_framing::framing::Answered, when: &str| {
        assert_eq!(
            read.fields.text(asked::THE_RUNS_ENDING_FIELD),
            Some(asked::A_STEP_FAILED),
            "{when}: the run's ending is not the failure"
        );
        assert_eq!(
            read.fields.text(asked::THE_FAILED_STEP_FIELD),
            Some(asked::THE_BOUNDARY_DRIVER_PLACED),
            "{when}: the ending does not name the step that failed"
        );
        assert_eq!(
            read.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD),
            Some(the_words),
            "{when}: the words did not come back whole"
        );
        assert_eq!(
            the_boundaries_in(read),
            vec![(
                asked::THE_BOUNDARY_STARTED.to_string(),
                asked::IT_HAD_NOTHING_TO_DO.to_string()
            )],
            "{when}: the run lost or gained a boundary"
        );
    };
    let now = asked_through_the_adapter(
        &area,
        &a_request_of(asked::THE_SETUP_RUNS_STATE_REQUEST, about_the_run(&run)),
    );
    the_ending_is_the_failure(&now, "at once");

    // **The desk never writes over an ending the machine holds.**
    let again = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_FAILED_STEP_IS_RECORDED_REQUEST,
            about_the_run(&run)
                .and(asked::THE_FAILED_STEP_FIELD, asked::THE_BOUNDARY_DONE)
                .and(asked::THE_MACHINES_OWN_WORDS_FIELD, "something else"),
        ),
    );
    assert_eq!(
        again.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::THERE_IS_NO_RECORD_OF_IT),
        "a second failure was written over the first"
    );

    // **`Server Setup` Y7's window, waited out with nothing asked about the
    // run**, then one supervisor turn so the window is looked at, then the
    // reading.
    std::thread::sleep(
        smoozy_server_setup_machine::driving::THE_COMMAND_GAP_WINDOW + Duration::from_secs(3),
    );
    let _ = asked_through_the_adapter(&area, &a_health_request());
    let after = asked_through_the_adapter(
        &area,
        &a_request_of(asked::THE_SETUP_RUNS_STATE_REQUEST, about_the_run(&run)),
    );
    the_ending_is_the_failure(&after, "after the window");
}

// ------------------------------------------------------ this test's own deadline

/// **What every test in this file waits on**, named in the message a missed
/// deadline prints.
// ------------------------------------------ G15a and G25, piece 10 of the wave

/// **`State, identity and recovery` §1.3 rule 7's fourth bullet, run: a
/// program meeting a store version newer than its own opens none of the
/// stores and says so**, answering only G12's health exchange.
///
/// **What it meets is a machine a newer release wrote**, which `Desktop app and
/// delivery` V14 makes unreachable by any act of Smoozy's; the rule is written
/// for the case nothing plans. **No store area is made**, and a request that
/// would read one is refused in the machine's own words rather than answered
/// out of stores this program cannot read.
#[test]
fn a_store_version_newer_than_the_program_opens_no_store_and_says_so() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    std::fs::write(
        the_record_at(&area.at.display().to_string(), records::THE_STORE_VERSION),
        "99",
    )
    .expect("a newer program's record is written");

    area.start_the_supervisor();

    let asked = asked_through_the_adapter(
        &area,
        &a_request_of(
            smoozy_framing::requests::THE_QUEUE_READING_REQUEST,
            smoozy_framing::framing::Fields::none(),
        ),
    );
    let said = asked
        .fields
        .text(smoozy_framing::requests::THE_MACHINES_OWN_WORDS_FIELD)
        .unwrap_or_default()
        .to_string();
    assert!(
        said.contains("newer"),
        "the refusal says why, in the machine's own words: {}",
        format!("{asked:?}")
    );
    for store in [
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
        smoozy_runtime::acting::journal::THE_AREA_THE_JOURNAL_SITS_IN,
        records::THE_AREA_THE_HOLD_RECORDS_SIT_IN,
    ] {
        assert!(
            !area.where_a_store_sits(store).exists(),
            "{store} was made by a program that may open none of the stores"
        );
    }
    assert_eq!(
        std::fs::read_to_string(the_record_at(
            &area.at.display().to_string(),
            records::THE_STORE_VERSION
        ))
        .expect("the record is still there"),
        "99",
        "the newer program's record is not rewritten"
    );
}

/// **The environment value that turns the stand-in below on**, and the
/// endpoint path it holds.
const THE_STAND_IN_HOLDS: &str = "SMOOZY_TEST_STAND_IN_HOLDS";

/// **A supervisor of some older program that knows no request at all.**
///
/// **It is this test binary itself, started again with only this test
/// selected**, so the process holding the endpoint is running a different
/// program from the one the pointer names — which is exactly what a container
/// looks like after the pointer has moved and before G25's ending. It refuses
/// every request by name, as a program placed before the ending request
/// existed does. **With the value unset it does nothing**, so a run of the
/// ignored tests does not hang on it.
#[test]
#[ignore]
fn a_stand_in_for_an_older_supervisor_that_knows_no_request() {
    let Ok(path) = std::env::var(THE_STAND_IN_HOLDS) else {
        return;
    };
    let listener = std::os::unix::net::UnixListener::bind(&path).expect("the stand-in binds");
    for caller in listener.incoming() {
        let Ok(mut caller) = caller else { continue };
        let Ok(request) = smoozy_framing::framing::read_one_request(&mut caller) else {
            continue;
        };
        let answered = smoozy_framing::framing::Answered::to(
            &request,
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            "0.0.0",
        )
        .saying_it_does_not_answer_that_request(&request.name);
        let _ = write_one_frame(
            &mut caller,
            &smoozy_framing::framing::write_result_frame(&answered),
        );
    }
}

/// The stand-in, running, and ended with the test whatever happens.
struct AStandIn(Child);

impl AStandIn {
    fn holding(area: &APackageArea) -> AStandIn {
        let child = Command::new(std::env::current_exe().expect("this test binary"))
            .args([
                "--ignored",
                "--exact",
                "a_stand_in_for_an_older_supervisor_that_knows_no_request",
                "--test-threads=1",
            ])
            .env(THE_STAND_IN_HOLDS, area.endpoint())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("the stand-in starts");
        let started = Instant::now();
        while std::os::unix::net::UnixStream::connect(area.endpoint()).is_err() {
            assert!(
                started.elapsed() < HOW_LONG_THE_SUPERVISOR_IS_GIVEN,
                "the stand-in never held the endpoint"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        AStandIn(child)
    }

    /// Whether it has ended, waited for up to `bound`.
    fn ended_inside(&mut self, bound: Duration) -> bool {
        let started = Instant::now();
        loop {
            if self.0.try_wait().expect("the stand-in is asked").is_some() {
                return true;
            }
            if started.elapsed() >= bound {
                return false;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}

impl Drop for AStandIn {
    fn drop(&mut self) {
        self.0.kill().ok();
        self.0.wait().ok();
    }
}

/// G25's ending request, freshly identified.
fn the_ending_request() -> Request {
    a_request_of(
        smoozy_framing::requests::THE_SUPERVISOR_ENDS_ITSELF_REQUEST,
        smoozy_framing::framing::Fields::none(),
    )
}

/// **`Rented-server runtime` G25 as struck and replaced on 2026-09-28, run:
/// the entry point, now naming the new program, ends a supervisor of another
/// program without asking it anything — and a second invocation, once the
/// new supervisor holds the endpoint, ends nothing.**
///
/// **The case the ending was first written for** was a supervisor that knew
/// the request and ended itself. **The case it now also meets** is one that
/// does not know it, or cannot be spoken to at all, which is every program
/// placed before 2026-09-22 and any too old for `Server protocol and
/// transfers` P3b.
#[test]
fn the_entry_point_ends_an_older_supervisor_that_knows_no_request_and_a_repeat_ends_nothing() {
    let _deadline = TheTestsDeadline::started();
    let mut area = APackageArea::made();
    let mut older = AStandIn::holding(&area);

    let asked = asked_through_the_adapter(&area, &the_ending_request());
    assert!(
        older.ended_inside(Duration::from_secs(20)),
        "the older supervisor is still holding the endpoint: {}",
        format!("{asked:?}")
    );
    assert_eq!(
        asked
            .fields
            .text(smoozy_framing::requests::WHAT_BECAME_OF_IT_FIELD),
        Some(smoozy_framing::requests::IT_HAS_A_RECORDED_OUTCOME),
        "{}",
        format!("{asked:?}")
    );

    // **The restarter's part**, which on a container is the keeper already
    // running: the supervisor the pointer names is started.
    area.start_the_supervisor();
    let asked = asked_through_the_adapter(&area, &the_ending_request());
    assert_eq!(
        asked
            .fields
            .text(smoozy_framing::requests::WHAT_BECAME_OF_IT_FIELD),
        Some(smoozy_framing::requests::IT_HAS_A_RECORDED_OUTCOME),
        "{}",
        format!("{asked:?}")
    );
    std::thread::sleep(Duration::from_secs(1));
    assert!(
        area.supervisor
            .as_mut()
            .expect("it was started")
            .try_wait()
            .expect("it is asked")
            .is_none(),
        "the newly placed program was taken down by a repeat of the ending"
    );
    area.waited_until_it_answers();
}

// ---------------------------------------------------------------------------
// A reading of a `.blend`, by a file-inspection worker — step 10c's dependency
// wave piece 2
// ---------------------------------------------------------------------------

/// **A stand-in for Blender that prints one document between the marks it is
/// given, and writes down the words it was started with.**
///
/// **Nothing about a real Blender is proved by it**: what is proved is that the
/// worker writes the script, starts the binary it was named by that path and
/// no other, keeps both streams, reaps the process and answers.
fn a_stand_in_that_prints_a_document(
    root: &Path,
    opens: &str,
    document: &[u8],
    closes: &str,
) -> (PathBuf, PathBuf) {
    use std::os::unix::fs::PermissionsExt;
    let stand_in = root.join("a-stand-in-for-blender");
    let printed = root.join("what-it-prints");
    let words = root.join("the-words-it-was-started-with");
    std::fs::write(&printed, document).expect("the document is written");
    std::fs::write(
        &stand_in,
        format!(
            "#!/bin/sh\n\
             for word in \"$@\"; do printf '%s\\n' \"$word\" >> '{words}'; done\n\
             echo 'Blender quit, as a stand-in says it' >&2\n\
             printf '{opens}'; cat '{printed}'; printf '{closes}'\n\
             exit 0\n",
            words = words.display(),
            printed = printed.display(),
        ),
    )
    .expect("the stand-in is written");
    std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in may be run");
    (stand_in, words)
}

/// **One reading asked, and the same request asked again until it has a
/// recorded outcome** — `Server protocol and transfers` C4's repeat, which is
/// how the desktop learns the reading ended.
fn a_reading_asked_until_it_ends(
    area: &APackageArea,
    which: &str,
    build: &Path,
    blend: &str,
) -> smoozy_framing::framing::Answered {
    a_reading_of_this_version_asked_until_it_ends(area, which, build, blend, "5.2.2")
}

/// The same, for a named version — the one the file's reading composes its
/// script for (`Job files and outputs` R2).
fn a_reading_of_this_version_asked_until_it_ends(
    area: &APackageArea,
    which: &str,
    build: &Path,
    blend: &str,
    version: &str,
) -> smoozy_framing::framing::Answered {
    use smoozy_framing::requests as asked;
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        asked::THE_READING_OF_A_BLEND_REQUEST,
    )
    .carrying(
        smoozy_framing::framing::Fields::none()
            .and(asked::THE_WHICH_READING_FIELD, which)
            .and(asked::THE_BUILD_FIELD, build.display().to_string())
            .and(asked::THE_BLEND_FIELD, blend)
            .and(asked::THE_BLENDER_VERSION_FIELD, version),
    );
    let first = asked_through_the_adapter(area, &request);
    assert_eq!(
        first.the_request_name_it_says_it_does_not_answer(),
        None,
        "the program refused the reading by name: {:?}",
        first.fields
    );
    let deadline = Instant::now() + Duration::from_secs(60);
    let mut answered = first;
    while answered.fields.text(asked::WHAT_BECAME_OF_IT_FIELD)
        != Some(asked::IT_HAS_A_RECORDED_OUTCOME)
    {
        assert_eq!(
            answered.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::IT_IS_STILL_IN_HAND),
            "the reading is neither in hand nor ended: {:?}",
            answered.fields
        );
        assert!(
            Instant::now() < deadline,
            "DEADLINE MISSED: the reading never reported an ending: {}",
            String::from_utf8_lossy(&std::fs::read(area.at.join("complaints")).unwrap_or_default())
        );
        std::thread::sleep(Duration::from_millis(100));
        answered = asked_through_the_adapter(area, &request);
    }
    answered
}

/// **The stream the answer names, read back whole, and held to the length it
/// states** — the desktop's half of the runner's ruling of 2026-09-28, done
/// here by reading the file rather than over the file session.
fn the_stream_it_names(
    answered: &smoozy_framing::framing::Answered,
    at: &str,
    length: &str,
) -> Vec<u8> {
    let path = answered
        .fields
        .text(at)
        .expect("the answer names where the stream sits");
    let stated: u64 = answered
        .fields
        .text(length)
        .and_then(|text| text.parse().ok())
        .expect("the answer states the stream's length");
    let read = std::fs::read(path).expect("the stream is on the disk");
    assert_eq!(
        read.len() as u64,
        stated,
        "the stated length is the stream's own"
    );
    read
}

/// **A document over one frame's ceiling**, built from the one a reading
/// prints, its per-frame table repeated until it passes
/// `smoozy_framing::framing::THE_LARGEST_FRAME` — the length a long animation
/// reaches, which a document carried inside the answer could never cross.
fn a_file_reading_over_the_largest_frame() -> Vec<u8> {
    let mut document: serde_json::Value =
        serde_json::from_str(&the_document_a_reading_prints()).expect("it reads");
    let per_frame = document["outputs"]["per_frame"]
        .as_array_mut()
        .expect("the per-frame table");
    let one = per_frame[0].clone();
    let mut frame = per_frame.len() as i64;
    while serde_json::to_vec(&per_frame).expect("it renders").len()
        <= smoozy_framing::framing::THE_LARGEST_FRAME + 64 * 1024
    {
        frame += 1;
        let mut another = one.clone();
        another["frame"] = serde_json::json!(frame);
        per_frame.push(another);
    }
    serde_json::to_vec(&document).expect("it renders")
}

/// **Step 10c's dependency wave piece 2: the file's reading is taken by a
/// file-inspection worker, by the build's own path, and its document — over
/// one frame's ceiling — comes back through the captured stream.**
///
/// **`Render Settings` Q4**: one file-inspection operation driven through the
/// supervisor's command path. **`Job files and outputs` R7**: it launches the
/// way the render launches, with no factory-startup switch. **The runner's
/// ruling of 2026-09-28**: the answer says where the stream sits and how long
/// it is, never the document.
///
/// **What it does not prove**: a real Blender reading a real file without
/// factory startup, which only a rental can, because the four Blenders at the
/// desk are the Creator's and carry the Creator's settings.
#[test]
fn the_files_reading_is_taken_by_a_file_inspection_worker_and_comes_back_through_the_stream() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let document = a_file_reading_over_the_largest_frame();
    assert!(document.len() > smoozy_framing::framing::THE_LARGEST_FRAME);
    let (stand_in, words) = a_stand_in_that_prints_a_document(
        &area.at,
        "<<<SMOOZY-READING>>>",
        &document,
        "<<<END-SMOOZY-READING>>>",
    );

    let ended = a_reading_asked_until_it_ends(
        &area,
        asked::THE_FILES_READING,
        &stand_in,
        "/ephemeral/smoozy/assets/kitchen.blend",
    );

    assert_eq!(ended.fields.text(asked::THE_EXIT_STATUS_FIELD), Some("0"));
    assert_eq!(
        ended.fields.text(asked::THE_START_MARK_SEEN_FIELD),
        Some(asked::THE_ANSWER_IS_YES)
    );
    assert_eq!(
        ended.fields.text(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD),
        Some(asked::THE_ANSWER_IS_NO)
    );
    let stream = the_stream_it_names(
        &ended,
        asked::THE_ORDINARY_STREAM_FIELD,
        asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
    );
    let complaints = the_stream_it_names(
        &ended,
        asked::THE_COMPLAINTS_STREAM_FIELD,
        asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD,
    );
    assert!(
        String::from_utf8_lossy(&complaints).contains("Blender quit"),
        "both streams are kept"
    );
    let read = smoozy_job_files_machine::reading_script::the_printed_reading_in(&stream)
        .expect("the document is found between the marks and reads whole");
    assert!(
        read.outputs.per_frame.len() > 1000,
        "the whole of it came back"
    );

    // **Started by the build's own path, with the reading's own line**: the
    // process options, the file, the script — and no factory-startup switch.
    let words = std::fs::read_to_string(&words).expect("the stand-in wrote its words");
    let words: Vec<&str> = words.lines().collect();
    assert_eq!(words.first(), Some(&"--background"), "{words:?}");
    let file = words
        .iter()
        .position(|word| *word == "/ephemeral/smoozy/assets/kitchen.blend");
    let script = words.iter().rposition(|word| *word == "--python");
    assert!(
        file.is_some() && script.is_some() && file < script,
        "{words:?}"
    );
    assert!(
        !words.iter().any(|word| word.contains("factory")),
        "{words:?}"
    );
    // **`Job files and outputs` R18 (simulation fix piece 5): the script
    // before the file stands right before it**, and its handler's throwaway
    // folder is gone when the reading ends.
    the_reads_load_time_script_stands_before_the_file(&words, file.unwrap());

    // `Render Settings` Q4: the saved library pairs pass through admission,
    // the operation record and the worker's first answer, not only a direct call.
    let files=area.at.join("files");
    std::fs::create_dir_all(&files).unwrap();
    let top=files.join("top.blend");
    let library=area.at.join("fixed-library.blend");
    std::fs::write(&top,b"top").unwrap();
    std::fs::write(&library,b"linked simulation").unwrap();
    let pair=smoozy_dependencies_machine::remap::compose_pair(
        smoozy_dependencies_machine::check::Kind::Library,"filepath",
        &smoozy_dependencies_machine::bytes::RecordedLocation::as_the_reading_carried_it(b"//lib/rig.blend"),
        smoozy_dependencies_machine::remap::NewLocation::AFixedCopy(
            smoozy_dependencies_machine::bytes::MachineLocation::composed(library.display().to_string().into_bytes()))).unwrap();
    let script=std::fs::read_to_string(&stand_in).unwrap().replace("exit 0\n",r#"for word in "$@"; do
  case "$word" in *.blend)
    if [ "$(cat "$(dirname "$word")/lib/rig.blend")" != "linked simulation" ]; then exit 27; fi
  ;; esac
done
exit 0
"#);
    std::fs::write(&stand_in,script).unwrap();
    let request=a_request_of(asked::THE_READING_OF_A_BLEND_REQUEST,
        smoozy_framing::framing::Fields::none()
            .and(asked::THE_WHICH_READING_FIELD,asked::THE_FILES_READING)
            .and(asked::THE_BUILD_FIELD,stand_in.display().to_string())
            .and(asked::THE_BLEND_FIELD,top.display().to_string())
            .and(asked::THE_BLENDER_VERSION_FIELD,"5.3.0")
            .and(asked::THE_READINGS_LIBRARY_PAIRS_FIELD,serde_json::to_vec(&vec![pair]).unwrap()));
    let linked=asked_until_it_ends(&area,&request);
    assert_eq!(linked.fields.text(asked::THE_EXIT_STATUS_FIELD),Some("0"),"{:?}",linked.fields);
    let record=std::path::Path::new(linked.fields.text(asked::THE_ORDINARY_STREAM_FIELD).unwrap()).parent().unwrap();
    assert!(!record.join("opening").exists(),"opening links are removed after the reading");
    assert_eq!(std::fs::read(&library).unwrap(),b"linked simulation");

}

/// **`Job files and outputs` R18 on a read's line**: `--python` and the
/// script before the file right before it, the script beside the reading's
/// own, and its throwaway folder removed after the run.
fn the_reads_load_time_script_stands_before_the_file(words: &[&str], file: usize) {
    assert_eq!(words[file - 2], "--python", "{words:?}");
    let load_time = words[file - 1];
    assert!(load_time.ends_with("-load-time.py"), "{words:?}");
    let text = std::fs::read_to_string(load_time).expect("the load-time script was written");
    assert!(text.contains("bpy.app.handlers.load_post.append(smoozy_as_the_file_loads)"));
    let throwaway = format!("{}-caches", load_time.trim_end_matches("-load-time.py"));
    assert!(
        !std::path::Path::new(&throwaway).exists(),
        "{throwaway} is removed after"
    );
}

/// **The check's reading, by the same worker and the same request**, with the
/// check's own line — `Dependency Upload & Remap` G1 — and a document over one
/// frame's ceiling found between the check's own marks.
#[test]
fn the_checks_reading_is_taken_by_the_same_worker_with_its_own_line() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let mut document = b"{\"references\":[".to_vec();
    while document.len() <= smoozy_framing::framing::THE_LARGEST_FRAME {
        document.extend_from_slice(b"{\"kind\":\"Image\",\"recorded\":[47,116,101,120,47,108,105,111,110,46,106,112,103]},");
    }
    document.extend_from_slice(b"{}]}");
    let (stand_in, words) = a_stand_in_that_prints_a_document(
        &area.at,
        "<<<SMOOZY-DEPENDENCIES>>>",
        &document,
        "<<<END-SMOOZY-DEPENDENCIES>>>",
    );

    let ended = a_reading_asked_until_it_ends(
        &area,
        asked::THE_CHECKS_READING,
        &stand_in,
        "/root/assets/files/Users/a/lion.blend",
    );

    assert_eq!(ended.fields.text(asked::THE_EXIT_STATUS_FIELD), Some("0"));
    assert_eq!(
        ended.fields.text(asked::THE_START_MARK_SEEN_FIELD),
        Some(asked::THE_ANSWER_IS_YES)
    );
    let stream = the_stream_it_names(
        &ended,
        asked::THE_ORDINARY_STREAM_FIELD,
        asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
    );
    assert_eq!(
        smoozy_dependencies_machine::check_script::between_the_sentinels(&stream),
        Some(document.as_slice()),
        "the document came back byte for byte"
    );
    let words = std::fs::read_to_string(&words).expect("the stand-in wrote its words");
    let words: Vec<&str> = words.lines().collect();
    assert!(words.contains(&"--factory-startup"), "{words:?}");
    let file = words
        .iter()
        .position(|word| *word == "/root/assets/files/Users/a/lion.blend")
        .expect("the file is on the line");
    the_reads_load_time_script_stands_before_the_file(&words, file);
}

/// **A reading that could not be started is a recorded outcome that names
/// why**, never a reading that is still in hand for ever and never an empty
/// one — `Dependency Upload & Remap` G3.
#[test]
fn a_reading_whose_build_cannot_be_run_ends_saying_so() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let ended = a_reading_asked_until_it_ends(
        &area,
        asked::THE_CHECKS_READING,
        Path::new("/no/such/blender"),
        "/root/a.blend",
    );
    assert_eq!(ended.fields.text(asked::THE_EXIT_STATUS_FIELD), None);
    assert_eq!(
        ended.fields.text(asked::THE_START_MARK_SEEN_FIELD),
        Some(asked::THE_ANSWER_IS_NO)
    );
    let why = ended
        .fields
        .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        .expect("the machine says why");
    assert!(why.contains("/no/such/blender"), "{why}");
}

/// **The 38's piece 7, item 36: *Stop* on the check reaches a reading already
/// on the machine** — `Server protocol and transfers` C21's stop, naming the
/// reading's own operation, is taken for a file-inspection operation this
/// supervisor started (`Dependency Upload & Remap` G15 and O1). The reading's
/// Blender here is a stand-in that would take half a minute.
///
/// **Where the kernel answers for a process** — every rented machine, the
/// Linux runner, and the desk since 2026-10-24 — the reading's Blender is
/// recorded as it starts, the interrupt reaches it, and the reading's answer
/// says Smoozy ended it. **Where it answers nothing**, no Blender is recorded
/// and the interrupt cannot be validated: the stop is taken and recorded, and
/// the reading runs to its own end. That half is asserted rather than skipped.
#[test]
fn a_stop_naming_a_reading_is_taken_and_ends_it_where_its_blender_is_recorded() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use std::os::unix::fs::PermissionsExt;
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let stand_in = area.at.join("a-stand-in-that-reads-for-half-a-minute");
    // **The stand-in becomes `sleep` itself, by `exec`**, so the process the
    // reading started is the one that is interrupted, as a real Blender is.
    // A shell that started `sleep` as its own child waits for that child
    // before it ends — `dash` and `bash` alike — so the interrupt took the
    // whole half-minute (hosted run `37188697561`).
    std::fs::write(&stand_in, "#!/bin/sh\nexec sleep 30\n").expect("the stand-in is written");
    std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in may be run");
    let reading = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        asked::THE_READING_OF_A_BLEND_REQUEST,
    )
    .carrying(
        smoozy_framing::framing::Fields::none()
            .and(asked::THE_WHICH_READING_FIELD, asked::THE_CHECKS_READING)
            .and(asked::THE_BUILD_FIELD, stand_in.display().to_string())
            .and(asked::THE_BLEND_FIELD, "/root/a.blend")
            .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.2"),
    );
    let first = asked_through_the_adapter(&area, &reading);
    assert_eq!(
        first.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND)
    );
    std::thread::sleep(Duration::from_secs(1));

    let stopped = asked_through_the_adapter(&area, &a_stop_of(&reading.identity));
    println!("THE STOP: {:?}", stopped.fields);
    assert_eq!(stopped.the_request_name_it_says_it_does_not_answer(), None);
    assert_ne!(
        stopped.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::THERE_IS_NO_RECORD_OF_IT),
        "the machine refused the stop of a reading: {:?}",
        stopped.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    let began = Instant::now();
    let mut answered = asked_through_the_adapter(&area, &reading);
    while answered.fields.text(asked::WHAT_BECAME_OF_IT_FIELD)
        != Some(asked::IT_HAS_A_RECORDED_OUTCOME)
    {
        assert!(
            began.elapsed() < Duration::from_secs(60),
            "DEADLINE MISSED: the reading never ended"
        );
        std::thread::sleep(Duration::from_millis(200));
        answered = asked_through_the_adapter(&area, &reading);
    }
    println!(
        "THE READING, ENDED AFTER {:?}: {:?}",
        began.elapsed(),
        answered.fields
    );
    if the_kernel_here_answers_for_a_process() {
        assert_eq!(
            answered
                .fields
                .text(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD),
            Some(asked::THE_ANSWER_IS_YES)
        );
        assert!(
            began.elapsed() < Duration::from_secs(20),
            "the interrupt did not end it"
        );
    } else {
        assert_eq!(
            answered
                .fields
                .text(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD),
            Some(asked::THE_ANSWER_IS_NO),
            "no Blender was recorded on this system, so nothing could be interrupted"
        );
    }
}

/// **One request asked through the adapter, and the same identity asked
/// again until it has a recorded outcome** — `Server protocol and transfers`
/// C4's repeat, for any file-inspection operation.
fn asked_until_it_ends(
    area: &APackageArea,
    request: &Request,
) -> smoozy_framing::framing::Answered {
    use smoozy_framing::requests as asked;
    let mut answered = asked_through_the_adapter(area, request);
    assert_eq!(
        answered.the_request_name_it_says_it_does_not_answer(),
        None,
        "the program refused {} by name",
        request.name
    );
    let deadline = Instant::now() + Duration::from_secs(60);
    while answered.fields.text(asked::WHAT_BECAME_OF_IT_FIELD)
        != Some(asked::IT_HAS_A_RECORDED_OUTCOME)
    {
        assert_eq!(
            answered.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::IT_IS_STILL_IN_HAND),
            "neither in hand nor ended: {:?}",
            answered.fields
        );
        assert!(
            Instant::now() < deadline,
            "DEADLINE MISSED: {} never ended",
            request.name
        );
        std::thread::sleep(Duration::from_millis(100));
        answered = asked_through_the_adapter(area, request);
    }
    answered
}

/// **The 38's piece 7, item 27: `Server protocol and transfers` §1.1 item 1's
/// digest over a byte range, through the real program and the real adapter** —
/// `Dependency Upload & Remap` Q3's contents reading, a file-inspection worker
/// that starts no Blender. The digest that comes back is S23's SHA-256 of the
/// bytes asked for, and a file that is not there is *nothing there*.
#[test]
fn a_digest_over_a_byte_range_is_taken_by_a_file_inspection_worker_and_comes_back_in_the_answer() {
    let _deadline = TheTestsDeadline::started();
    use sha2::Digest as _;
    use smoozy_framing::requests as asked;
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let file = area.at.join("a texture.png");
    let bytes: Vec<u8> = (0..2_000_000u32).map(|at| (at % 241) as u8).collect();
    std::fs::write(&file, &bytes).expect("the file");
    let digest_of = |part: &[u8]| -> String {
        sha2::Sha256::digest(part)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    };
    let asked_for = |at: &Path, range: Option<(u64, u64)>| {
        let mut fields = smoozy_framing::framing::Fields::none()
            .and(asked::THE_DIGESTED_FILE_FIELD, at.display().to_string())
            .and(
                asked::THE_RANGE_STARTS_AT_FIELD,
                range.map_or(0, |range| range.0).to_string(),
            );
        if let Some((_, length)) = range {
            fields = fields.and(asked::THE_RANGE_LENGTH_FIELD, length.to_string());
        }
        a_request_of(asked::THE_DIGEST_OVER_A_BYTE_RANGE_REQUEST, fields)
    };

    let whole = asked_until_it_ends(&area, &asked_for(&file, None));
    assert_eq!(
        whole.fields.text(asked::THE_DIGESTS_OUTCOME_FIELD),
        Some(asked::IT_WAS_DIGESTED)
    );
    assert_eq!(
        whole.fields.text(asked::THE_DIGEST_FIELD),
        Some(digest_of(&bytes).as_str())
    );
    assert_eq!(
        whole.fields.text(asked::THE_BYTES_DIGESTED_FIELD),
        Some("2000000")
    );
    assert_eq!(
        whole.fields.text(asked::THE_ORDINARY_STREAM_FIELD),
        None,
        "no Blender, no stream"
    );

    let part = asked_until_it_ends(&area, &asked_for(&file, Some((65_536, 100_000))));
    assert_eq!(
        part.fields.text(asked::THE_DIGEST_FIELD),
        Some(digest_of(&bytes[65_536..165_536]).as_str())
    );

    let gone = asked_until_it_ends(&area, &asked_for(&area.at.join("no such file"), None));
    assert_eq!(
        gone.fields.text(asked::THE_DIGESTS_OUTCOME_FIELD),
        Some(asked::NOTHING_IS_THERE)
    );
    assert_eq!(gone.fields.text(asked::THE_DIGEST_FIELD), None);
}

/// **The 38's piece 7, item 7: `Dependency Upload & Remap` M14's fixing is
/// taken by the same worker, by the build's own path, with M14's line** — the
/// file, then the fixing's own script, and **no load-time script**; the pairs
/// and the staging path reach the script as the request carried them. A
/// stand-in stands where the build's path goes and keeps the script it was
/// handed; nothing here says what a real Blender does with it (the desk test
/// below does).
#[test]
fn a_fixed_copy_is_produced_by_a_file_inspection_worker_with_m14s_line() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use std::os::unix::fs::PermissionsExt;
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let stand_in = area.at.join("a-stand-in-that-fixes");
    let words = area.at.join("the-words-it-was-started-with");
    let kept = area.at.join("the-script-it-was-handed");
    std::fs::write(
        &stand_in,
        format!(
            "#!/bin/sh\n\
             for word in \"$@\"; do printf '%s\\n' \"$word\" >> '{words}'; last=\"$word\"; done\n\
             cp \"$last\" '{kept}'\n\
             printf '%s\\n' '{mark}'\n\
             exit 0\n",
            words = words.display(),
            kept = kept.display(),
            mark = smoozy_dependencies_machine::fixing_script::THE_FIXINGS_START_MARK,
        ),
    )
    .expect("the stand-in is written");
    std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755)).expect("runnable");
    let pair = smoozy_dependencies_machine::remap::compose_pair(
        smoozy_dependencies_machine::check::Kind::Image,
        "filepath",
        &smoozy_dependencies_machine::bytes::RecordedLocation::as_the_reading_carried_it(
            b"//tex/checker.png",
        ),
        smoozy_dependencies_machine::remap::NewLocation::AMirror(
            smoozy_dependencies_machine::bytes::MachineLocation::composed(
                b"/root/assets/files/Users/a/tex/checker.png".to_vec(),
            ),
        ),
    )
    .expect("the pair composes");
    let pairs_text = serde_json::to_string(&vec![pair]).expect("the pairs write");
    let pairs = pairs_text.clone().into_bytes();
    let staging =
        b"/root/assets/fixed/assets/files/Users/a/.smoozy-staging/set.blend.fixing".to_vec();
    let request = a_request_of(
        asked::THE_FIXED_COPY_IS_PRODUCED_REQUEST,
        smoozy_framing::framing::Fields::none()
            .and(asked::THE_BUILD_FIELD, stand_in.display().to_string())
            .and(
                asked::THE_BLEND_FIELD,
                "/root/assets/files/Users/a/set.blend",
            )
            .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.2")
            .and(asked::THE_FIXINGS_STAGING_FIELD, staging.clone())
            .and(asked::THE_FIXINGS_PAIRS_FIELD, pairs),
    );
    let ended = asked_until_it_ends(&area, &request);
    assert_eq!(ended.fields.text(asked::THE_EXIT_STATUS_FIELD), Some("0"));
    assert_eq!(
        ended.fields.text(asked::THE_START_MARK_SEEN_FIELD),
        Some(asked::THE_ANSWER_IS_YES)
    );
    assert_eq!(
        ended.fields.text(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD),
        Some(asked::THE_ANSWER_IS_NO)
    );
    let stream = the_stream_it_names(
        &ended,
        asked::THE_ORDINARY_STREAM_FIELD,
        asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
    );
    assert!(smoozy_dependencies_machine::fixing_script::the_fixing_started_in(&stream));

    let words = std::fs::read_to_string(&words).expect("the stand-in wrote its words");
    let words: Vec<&str> = words.lines().collect();
    assert_eq!(
        &words[..5],
        &[
            "-b",
            "--factory-startup",
            "/root/assets/files/Users/a/set.blend",
            "--python-exit-code",
            "1"
        ]
    );
    assert_eq!(words[5], "--python");
    assert_eq!(
        words.len(),
        7,
        "one script and no load-time script: {words:?}"
    );
    let script = std::fs::read_to_string(&kept).expect("the script was handed");
    assert!(
        script.contains(&serde_json::to_string(&staging).expect("it renders")),
        "the staging path"
    );
    assert!(script.contains("relative_remap=False"));
    assert!(
        script.contains(&pairs_text),
        "the pairs, as the request carried them"
    );

    // **A repeat of the identity acts on nothing**: the same answer.
    let again = asked_through_the_adapter(&area, &request);
    assert_eq!(again.fields.text(asked::THE_EXIT_STATUS_FIELD), Some("0"));
}

/// **The 38's piece 7 at the desk: a real Blender fixes the piece's own linked
/// set through the real program** — `App/Dependency Upload & Remap/tests/a
/// linked set/assets/set.blend`, whose cube reads `//tex/checker.png`, fixed
/// with that one reference pointed at a full location. The copy lands at the
/// staging path, and opened again it reads the new location; the set the user
/// sent is byte for byte what it was. Run by hand with `SMOOZY_DESK_BLENDER`.
#[test]
#[ignore = "run by hand at the desk with a real Blender"]
fn a_real_blender_fixes_the_linked_set_through_the_real_program() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    let blender = std::env::var("SMOOZY_DESK_BLENDER")
        .unwrap_or_else(|_| "/Applications/Blender.app/Contents/MacOS/Blender".to_string());
    let mut area = APackageArea::made();
    area.start_the_supervisor();
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../Dependency Upload & Remap/tests/a linked set/assets/set.blend");
    let machine_copy = area.at.join("files/set.blend");
    std::fs::create_dir_all(machine_copy.parent().expect("a folder")).expect("the folder");
    std::fs::copy(&fixture, &machine_copy).expect("the machine copy");
    let before = std::fs::read(&machine_copy).expect("it reads");
    let texture = area.at.join("files/tex/checker.png");
    let staging = area.at.join("fixed/.smoozy-staging/set.blend.fixing");
    std::fs::create_dir_all(staging.parent().expect("a folder")).expect("the folder");
    let pair = smoozy_dependencies_machine::remap::compose_pair(
        smoozy_dependencies_machine::check::Kind::Image,
        "filepath",
        &smoozy_dependencies_machine::bytes::RecordedLocation::as_the_reading_carried_it(
            b"//tex/checker.png",
        ),
        smoozy_dependencies_machine::remap::NewLocation::AMirror(
            smoozy_dependencies_machine::bytes::MachineLocation::composed(
                texture.display().to_string().into_bytes(),
            ),
        ),
    )
    .expect("the pair composes");
    let request = a_request_of(
        asked::THE_FIXED_COPY_IS_PRODUCED_REQUEST,
        smoozy_framing::framing::Fields::none()
            .and(asked::THE_BUILD_FIELD, blender.clone())
            .and(asked::THE_BLEND_FIELD, machine_copy.display().to_string())
            .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.2")
            .and(
                asked::THE_FIXINGS_STAGING_FIELD,
                staging.display().to_string(),
            )
            .and(
                asked::THE_FIXINGS_PAIRS_FIELD,
                serde_json::to_vec(&vec![pair]).expect("it writes"),
            ),
    );
    let ended = asked_until_it_ends(&area, &request);
    let stream = the_stream_it_names(
        &ended,
        asked::THE_ORDINARY_STREAM_FIELD,
        asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
    );
    println!("THE FIXING'S STREAM:\n{}", String::from_utf8_lossy(&stream));
    assert_eq!(ended.fields.text(asked::THE_EXIT_STATUS_FIELD), Some("0"));
    assert!(staging.is_file(), "the copy is at the staging path");
    assert_eq!(
        std::fs::read(&machine_copy).expect("it reads"),
        before,
        "the user's copy is untouched"
    );
    let probe = area.at.join("probe.py");
    std::fs::write(
        &probe,
        "import bpy\nfor image in bpy.data.images:\n    print('SMOOZY-PROBE', image.filepath)\n",
    )
    .expect("the probe");
    let read = Command::new(&blender)
        .args(["-b", "--factory-startup"])
        .arg(&staging)
        .arg("--python")
        .arg(&probe)
        .output()
        .expect("Blender opens the copy");
    let said = String::from_utf8_lossy(&read.stdout);
    println!(
        "THE COPY READS: {}",
        said.lines()
            .filter(|line| line.contains("SMOOZY-PROBE"))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    assert!(
        said.contains(&format!("SMOOZY-PROBE {}", texture.display())),
        "the copy records the new location"
    );
}

/// **Every program this area's supervisor started, and every program those
/// started, found by the one thing they all carry: the area's own path on
/// their command line.** A worker is started through the area's pointer
/// (`smoozy_runtime::acting::starting::how_the_program_invokes_a_worker`), and
/// a stand-in for Blender lives under the area too.
fn the_programs_started_under(at: &Path) -> Vec<String> {
    let listed = Command::new("pgrep")
        .args(["-f", &at.display().to_string()])
        .output()
        .expect("pgrep runs");
    String::from_utf8_lossy(&listed.stdout)
        .lines()
        .map(str::to_string)
        .collect()
}

/// **Added 2026-09-29, step 10c's dependency wave piece 8: two
/// `smoozy-server work file-inspection` processes were found running after a
/// sitting, left by this file.**
///
/// **A worker is not the supervisor's child for long enough to die with it.**
/// Killing the supervisor, which is all a dropped area did, leaves a worker
/// running — and a file-inspection worker whose supervisor is gone repeats its
/// report for ever (`Rented-server runtime` W12, *"re-attempted until it is
/// accepted"*), while one whose Blender never ends waits on it for ever. So a
/// test that gave up on a reading, or ended one tick before the worker heard
/// its report was accepted, left the worker behind with nobody knowing it
/// existed.
///
/// **A reading is started whose Blender never ends, and the area is dropped
/// while it runs.** Nothing started under the area may still be running
/// afterwards.
#[test]
fn a_dropped_package_area_leaves_no_program_it_started_running() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use std::os::unix::fs::PermissionsExt;
    let at;
    {
        let mut area = APackageArea::made();
        area.start_the_supervisor();
        at = area.at.clone();
        let stand_in = area.at.join("a-stand-in-that-never-ends");
        std::fs::write(&stand_in, "#!/bin/sh\nwhile :; do sleep 1; done\n")
            .expect("the stand-in is written");
        std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755))
            .expect("the stand-in may be run");
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            asked::THE_READING_OF_A_BLEND_REQUEST,
        )
        .carrying(
            smoozy_framing::framing::Fields::none()
                .and(asked::THE_WHICH_READING_FIELD, asked::THE_CHECKS_READING)
                .and(asked::THE_BUILD_FIELD, stand_in.display().to_string())
                .and(asked::THE_BLEND_FIELD, "/root/a.blend")
                .and(asked::THE_BLENDER_VERSION_FIELD, "5.2.2"),
        );
        let first = asked_through_the_adapter(&area, &request);
        assert_eq!(
            first.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::IT_IS_STILL_IN_HAND),
            "{:?}",
            first.fields
        );
        // **The worker and the stand-in it started, both running**, before the
        // area is let go.
        let deadline = Instant::now() + Duration::from_secs(30);
        while the_programs_started_under(&at).len() < 3 {
            assert!(
                Instant::now() < deadline,
                "DEADLINE MISSED: the worker and its stand-in never both ran: {:?}",
                the_programs_started_under(&at)
            );
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut left = the_programs_started_under(&at);
    while !left.is_empty() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(100));
        left = the_programs_started_under(&at);
    }
    assert!(
        left.is_empty(),
        "programs started under {} are still running after it was dropped: {left:?}",
        at.display()
    );
}

/// **The desk's own proof with a real Blender and a real file, kept in the tree
/// and ignored** — step 10c's dependency wave piece 2, run by hand:
///
/// ```text
/// SMOOZY_DESK_BLENDER=/Applications/Blender.app/Contents/MacOS/Blender \
/// SMOOZY_DESK_BLEND=/path/to/blender-5.2-splash.blend \
/// SMOOZY_DESK_SPLASH_READING=/path/to/reading-5.2-as.json \
/// SMOOZY_DESK_VERSION=5.2.2 \
/// cargo test -p smoozy-server --test the_program_runs -- --ignored --nocapture a_real_blender
/// ```
///
/// **What it proves**: the real program, its real supervisor and a real
/// file-inspection worker start the real Blender by its own path for the
/// check's reading and for the file's reading, and both documents come back
/// through the stream and read. **The file's reading is started through a
/// two-line wrapper that adds the factory-startup switch**, because the four
/// Blenders at the desk are the Creator's and carry the Creator's settings; the
/// machine launches it without the switch (`Job files and outputs` R7), and
/// only a rental proves that launch. **And, where the splash reading is named,
/// its printed document with the per-frame table repeated past one frame's
/// ceiling** crosses the same route through a stand-in and reads whole.
#[test]
#[ignore = "run by hand at the desk with a real Blender and a real .blend"]
fn a_real_blender_reads_a_real_file_through_the_real_program() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use std::os::unix::fs::PermissionsExt;
    let blender = PathBuf::from(std::env::var("SMOOZY_DESK_BLENDER").expect("SMOOZY_DESK_BLENDER"));
    let blend = std::env::var("SMOOZY_DESK_BLEND").expect("SMOOZY_DESK_BLEND");
    let mut area = APackageArea::made();
    area.start_the_supervisor();

    let started = Instant::now();
    let check = a_reading_asked_until_it_ends(&area, asked::THE_CHECKS_READING, &blender, &blend);
    let took = started.elapsed();
    assert_eq!(
        check.fields.text(asked::THE_EXIT_STATUS_FIELD),
        Some("0"),
        "{:?}",
        check.fields
    );
    let stream = the_stream_it_names(
        &check,
        asked::THE_ORDINARY_STREAM_FIELD,
        asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
    );
    let document = smoozy_dependencies_machine::check_script::between_the_sentinels(&stream)
        .expect("the check's document is between its marks");
    let read: serde_json::Value = serde_json::from_slice(document).expect("it reads");
    println!(
        "the check's reading: {} bytes on the stream, {} references, {took:?} end to end",
        stream.len(),
        read["references"].as_array().map(Vec::len).unwrap_or(0)
    );

    let wrapper = area.at.join("blender-with-factory-startup");
    std::fs::write(
        &wrapper,
        format!(
            "#!/bin/sh\nexec '{}' --factory-startup \"$@\"\n",
            blender.display()
        ),
    )
    .expect("the wrapper is written");
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).expect("runnable");
    let started = Instant::now();
    let version = std::env::var("SMOOZY_DESK_VERSION").unwrap_or_else(|_| "5.2.2".to_string());
    let file = a_reading_of_this_version_asked_until_it_ends(
        &area,
        asked::THE_FILES_READING,
        &wrapper,
        &blend,
        &version,
    );
    let took = started.elapsed();
    assert_eq!(
        file.fields.text(asked::THE_EXIT_STATUS_FIELD),
        Some("0"),
        "{:?}",
        file.fields
    );
    let stream = the_stream_it_names(
        &file,
        asked::THE_ORDINARY_STREAM_FIELD,
        asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
    );
    let printed = smoozy_job_files_machine::reading_script::the_printed_reading_in(&stream)
        .expect("the file's document is between its marks and reads whole");
    println!(
        "the file's reading: {} bytes on the stream, {} frames stepped, {} errors, {took:?} end to end",
        stream.len(),
        printed.outputs.per_frame.len(),
        printed.errors.len()
    );

    if let Ok(splash) = std::env::var("SMOOZY_DESK_SPLASH_READING") {
        let whole: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&splash).expect("the splash reading"))
                .expect("it reads");
        let mut document = whole["printed"].clone();
        let frames = document["outputs"]["per_frame"]
            .as_array()
            .expect("the per-frame table")
            .clone();
        let mut repeated = 1;
        while serde_json::to_vec(&document).expect("it renders").len()
            <= smoozy_framing::framing::THE_LARGEST_FRAME
        {
            document["outputs"]["per_frame"]
                .as_array_mut()
                .expect("the per-frame table")
                .extend(frames.iter().cloned());
            repeated += 1;
        }
        let document = serde_json::to_vec(&document).expect("it renders");
        let (stand_in, _) = a_stand_in_that_prints_a_document(
            &area.at,
            "<<<SMOOZY-READING>>>",
            &document,
            "<<<END-SMOOZY-READING>>>",
        );
        let long =
            a_reading_asked_until_it_ends(&area, asked::THE_FILES_READING, &stand_in, &blend);
        let stream = the_stream_it_names(
            &long,
            asked::THE_ORDINARY_STREAM_FIELD,
            asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
        );
        let printed = smoozy_job_files_machine::reading_script::the_printed_reading_in(&stream)
            .expect("the long document reads whole");
        println!(
            "the 5.2 splash reading repeated {repeated} times: {} bytes, {} frames, read whole",
            document.len(),
            printed.outputs.per_frame.len()
        );
        assert!(document.len() > smoozy_framing::framing::THE_LARGEST_FRAME);
    }
}

// ---------------------------------------------------------------------------
// The live figures — the Creator's notes on 0.1.8, piece 6a
// ---------------------------------------------------------------------------

/// **A stand-in for Blender whose render-start reading takes six seconds and
/// then prints nothing**, so its job reads *rendering* for that long with its
/// worker alive, and then ends failed at that reading (`Job files and outputs`
/// C9's *a reading that did not end successfully*). Nothing about a real
/// Blender is proved by it.
fn a_stand_in_whose_reading_takes_a_while(root: &ARoot) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let stand_in = root.at.join("a-stand-in-that-reads-slowly");
    std::fs::write(
        &stand_in,
        "#!/bin/sh\n\
         for word in \"$@\"; do\n\
         case \"$word\" in\n\
         *version-ask-*.py) printf '<<<SMOOZY-READING>>>5.2.0<<<END-SMOOZY-READING>>>'; exit 0;;\n\
         *reading-*.py) sleep 6; exit 0;;\n\
         esac\n\
         done\n\
         exit 0\n",
    )
    .expect("the stand-in is written");
    std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in may be run");
    stand_in
}

/// **`Queue & History` R5 through the real program**: a report sent over the
/// real machine-local endpoint, the way the render worker sends it, is held by
/// the real supervisor and carried in the queue reading the real adapter
/// returns — with the job's two sinks and their lengths (`Queue & History`
/// E8). A report under a run that is not the supervisor's is refused. The
/// Creator's notes on 0.1.8, piece 6a.
///
/// **What is stood in for, said plainly**: the render worker's own send. It
/// was written when no render's frames could run through the real program at
/// the desk (`Rented-server runtime` X1, until 2026-10-24); the test sends what
/// the worker sends, composed from the same fields, while the job is *rendering*.
/// The worker's loop sending it is proved in one process by
/// `unix::tests::a_renders_figures_reach_the_queue_reading_while_it_runs_and_go_with_its_ending`.
///
/// **What it bites on**: an endpoint that does not hand the report to the
/// supervisor's judgement, a queue reading that carries no figures or no
/// sinks, and one that carries a stale worker's.
#[test]
fn a_workers_figures_sent_over_the_endpoint_reach_the_queue_reading_through_the_adapter() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_queue_history_machine::readings::{
        AWorkersReport, LiveReadings, SamplePair, TheSinksOfARenderingJob,
    };
    use smoozy_runtime::acting::workers::{
        IT_WAS_ACCEPTED, IT_WAS_REFUSED_AS_STALE, THE_RUN_THE_WORKER_WAS_STARTED_UNDER,
        THE_WORKERS_REPORT_REQUEST, WHETHER_THE_REPORT_WAS_ACCEPTED,
    };
    use smoozy_state::observed::{Clock, MachineClock, Reading};
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let stand_in = a_stand_in_whose_reading_takes_a_while(&root);
    let job = root.a_job_the_render_worker_can_take(&stand_in);
    area.start_the_supervisor();
    submitted(&area, &job);

    let the_queue_reading = || {
        asked_through_the_adapter(
            &area,
            &a_request_of(asked::THE_QUEUE_READING_REQUEST, Fields::none()),
        )
    };
    let began = Instant::now();
    let operation = loop {
        let reading = the_queue_reading();
        let item = |field: &str| {
            reading
                .fields
                .text(&asked::the_field_of_item(asked::THE_JOB_FIELD, 0, field))
                .map(str::to_string)
        };
        if item(asked::THE_QUEUE_STATE_FIELD).as_deref() == Some(asked::IT_IS_RENDERING) {
            break Identity::read_back(
                IdentityKind::Operation,
                &item(asked::THE_RENDER_OPERATION_FIELD)
                    .expect("a rendering job names its operation"),
            )
            .expect("it reads back");
        }
        assert!(
            began.elapsed() < Duration::from_secs(5),
            "the job never read as rendering"
        );
        std::thread::sleep(Duration::from_millis(100));
    };
    let run = smoozy_runtime::acting::package::the_health_answer_in(&asked_through_the_adapter(
        &area,
        &a_health_request(),
    ))
    .expect("G12's answer")
    .run;

    let report = AWorkersReport {
        attempt: 1,
        readings: Reading::taken(
            LiveReadings {
                frame_in_hand: Some(2),
                samples: Some(SamplePair {
                    seen: 812,
                    of: 2048,
                }),
                blenders_own_estimate_for_the_frame: Some("00:42.17".to_string()),
                frames_completed: 1,
                ..LiveReadings::default()
            },
            std::time::SystemTime::UNIX_EPOCH + Duration::from_secs(1_790_000_000),
            Clock::AMachine(MachineClock(area.installation.as_text())),
        ),
        newest_frame_saved: Some(1),
        the_plan_total: Some(4096),
        the_units_finished: Some(1024),
    };
    let sent_under = |run: &Identity, report: &AWorkersReport| {
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            operation.clone(),
            THE_WORKERS_REPORT_REQUEST,
        )
        .carrying(report.as_pairs().into_iter().fold(
            Fields::none().and(THE_RUN_THE_WORKER_WAS_STARTED_UNDER, run.as_text()),
            |fields, (name, value)| fields.and(name, value),
        ));
        TheEndpointAsACaller::at(&area.endpoint(), Duration::from_secs(10))
            .ask(&request)
            .expect("the endpoint is reachable")
            .expect("the supervisor answers")
            .fields
            .text(WHETHER_THE_REPORT_WAS_ACCEPTED)
            .map(str::to_string)
    };
    assert_eq!(sent_under(&run, &report).as_deref(), Some(IT_WAS_ACCEPTED));

    let reading = the_queue_reading();
    let item = |field: &str| {
        reading
            .fields
            .text(&asked::the_field_of_item(asked::THE_JOB_FIELD, 0, field))
    };
    assert_eq!(
        item(asked::THE_QUEUE_STATE_FIELD),
        Some(asked::IT_IS_RENDERING)
    );
    assert_eq!(
        AWorkersReport::read_back(item),
        Some(report.clone()),
        "the figures, whole"
    );
    let sinks = TheSinksOfARenderingJob::read_back(item).expect("the two sinks are named");
    for sink in [&sinks.ordinary, &sinks.complaints] {
        assert!(
            Path::new(&sink.path).is_file(),
            "{} is the supervisor's own sink",
            sink.path
        );
        assert_eq!(sink.length, Some(0), "nothing printed into it yet");
    }

    // **A stale worker's figures are refused and never shown.**
    let mut stale = report.clone();
    stale.the_plan_total = Some(1);
    let earlier = Identity::issue(IdentityKind::Run).expect("randomness");
    assert_eq!(
        sent_under(&earlier, &stale).as_deref(),
        Some(IT_WAS_REFUSED_AS_STALE)
    );
    let reading = the_queue_reading();
    let item = |field: &str| {
        reading
            .fields
            .text(&asked::the_field_of_item(asked::THE_JOB_FIELD, 0, field))
    };
    assert_eq!(
        AWorkersReport::read_back(item).and_then(|read| read.the_plan_total),
        Some(4096)
    );

    // **The job ends and its worker with it**, so nothing is left running.
    let note = the_note_once_it_has_ended(&area, &job);
    assert_eq!(
        note.fields.text(asked::THE_ENDING_FIELD),
        Some(asked::IT_FAILED)
    );
}

/// **The queue reading asked while a real Blender works on a real ten-frame
/// file, through the real program and the real adapter.** Run by hand at the
/// desk; it prints what each reading carried for the job while it was
/// rendering, and how the attempt ended.
///
/// **What the desk cannot show, said plainly.** This Mac publishes no
/// per-process record, so `Rendering` L5's pair is never recorded here and the
/// started render is undone at once (`Rented-server runtime` X1). The job is
/// *rendering* through the version ask and the render-start reading, which
/// run the real Blender; the render's own frames are run only where the
/// kernel publishes the record.
#[test]
#[ignore = "run by hand at the desk with a real Blender and a real .blend"]
fn a_real_blender_job_and_the_queue_reading_asked_while_it_is_rendering() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::framing::Fields;
    use smoozy_framing::requests as asked;
    use std::os::unix::fs::PermissionsExt;
    let blender = PathBuf::from(std::env::var("SMOOZY_DESK_BLENDER").expect("SMOOZY_DESK_BLENDER"));
    let blend = std::env::var("SMOOZY_DESK_BLEND").expect("SMOOZY_DESK_BLEND");
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let wrapper = area.at.join("blender-with-factory-startup");
    std::fs::write(
        &wrapper,
        format!(
            "#!/bin/sh\nexec '{}' --factory-startup \"$@\"\n",
            blender.display()
        ),
    )
    .expect("the wrapper is written");
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).expect("runnable");
    let job = root.a_job_the_render_worker_can_take_over(
        &wrapper,
        smoozy_job_files_machine::manifest::FrameRange::TheFilesOwn,
    );
    std::fs::copy(&blend, root.at.join("kitchen.blend")).expect("the real file is put in place");
    // **The version the real Blender answers `Rendering` L14's ask with**, in
    // place of the helper's own.
    let manifest_at = root
        .at
        .join("job-records")
        .join(job.as_text())
        .join(smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME);
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&manifest_at).expect("the manifest"))
            .expect("it reads");
    manifest["blender"]["version"] = serde_json::Value::from(
        std::env::var("SMOOZY_DESK_VERSION").unwrap_or_else(|_| "5.2.2".to_string()),
    );
    std::fs::write(
        &manifest_at,
        serde_json::to_vec(&manifest).expect("it writes"),
    )
    .expect("written back");
    area.start_the_supervisor();
    submitted(&area, &job);

    let began = Instant::now();
    let mut seen_rendering = 0;
    loop {
        let reading = asked_through_the_adapter(
            &area,
            &a_request_of(asked::THE_QUEUE_READING_REQUEST, Fields::none()),
        );
        let item = |field: &str| {
            reading
                .fields
                .text(&asked::the_field_of_item(asked::THE_JOB_FIELD, 0, field))
                .map(str::to_string)
        };
        let state = item(asked::THE_QUEUE_STATE_FIELD);
        if state.as_deref() == Some(asked::IT_IS_RENDERING) {
            seen_rendering += 1;
            println!(
                "{:>5.1}s rendering: the answer carries {} fields in all",
                began.elapsed().as_secs_f64(),
                reading.fields.how_many(),
            );
        }
        if state.as_deref() == Some(asked::IT_HAS_ENDED)
            || began.elapsed() > Duration::from_secs(240)
        {
            println!(
                "{:>5.1}s the job reads {state:?}",
                began.elapsed().as_secs_f64()
            );
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    let note = the_note_once_it_has_ended(&area, &job);
    println!(
        "the note's ending: {:?}, {:?}",
        note.fields.text(asked::THE_ENDING_FIELD),
        note.fields
            .text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD)
    );
    assert!(seen_rendering > 0, "the job was never read while rendering");
}

/// **`Job files and outputs` §6.6 at the desk, through the real program and
/// a real Blender**: a job whose folder is chosen at render start, its file a
/// stand-in for the Creator's own (`//../Render/…` outputs) saved one folder
/// below a stand-in for `/home/ubuntu`. **The render worker's step 4 reads the
/// file with the real Blender, chooses the job's folder where the file
/// points, makes it, records the choice, and composes the map under it** — so
/// the redirect points every output at `<home>/Render/<job>/…`. Run by hand
/// with `SMOOZY_DESK_BLENDER`, `SMOOZY_DESK_BLEND` (the stand-in) and
/// `SMOOZY_DESK_VERSION`; `SMOOZY_DESK_FALLBACK` names a chosen fallback
/// folder and `SMOOZY_DESK_BLEND_UNDER_THE_ROOT=1` puts the file under
/// Smoozy's own root, so the fallback is the one taken. **What the desk
/// could not show when this was written**: the render's own frames
/// (`Rented-server runtime` X1; the desk takes the pair since 2026-10-24). The
/// Creator's second notes, piece 4.
#[test]
#[ignore = "run by hand at the desk with a real Blender and the stand-in .blend"]
fn a_real_blender_job_chooses_its_folder_where_the_file_points() {
    let _deadline = TheTestsDeadline::started();
    use std::os::unix::fs::PermissionsExt;
    let blender = PathBuf::from(std::env::var("SMOOZY_DESK_BLENDER").expect("SMOOZY_DESK_BLENDER"));
    let source = std::env::var("SMOOZY_DESK_BLEND").expect("SMOOZY_DESK_BLEND");
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let wrapper = area.at.join("blender-with-factory-startup");
    std::fs::write(
        &wrapper,
        format!(
            "#!/bin/sh\nexec '{}' --factory-startup \"$@\"\n",
            blender.display()
        ),
    )
    .expect("the wrapper is written");
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).expect("runnable");
    let home = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "smoozy-home-{}",
        Identity::issue(IdentityKind::Operation)
            .expect("randomness")
            .as_text()
    ));
    let rendering = if std::env::var("SMOOZY_DESK_BLEND_UNDER_THE_ROOT").is_ok() {
        root.at.join("assets/rendering")
    } else {
        home.join("home/ubuntu/rendering")
    };
    std::fs::create_dir_all(&rendering).expect("the file's folder");
    let blend = rendering.join("standin.blend");
    std::fs::copy(&source, &blend).expect("the stand-in is put in place");
    // **The job area exists before the first job**, as the hand-over leaves
    // it (`Job files and outputs` N11).
    std::fs::create_dir_all(root.at.join("jobs")).expect("the job area");
    let fallback = std::env::var("SMOOZY_DESK_FALLBACK").ok();
    let job = root.a_job_whose_folder_is_chosen_at_render_start(
        &wrapper,
        smoozy_job_files_machine::manifest::FrameRange::TheUsersParsedIntegers(vec![1]),
        &blend,
        fallback.as_deref(),
    );
    let record_directory = root.at.join("job-records").join(job.as_text());
    let manifest_at = record_directory.join(smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME);
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&manifest_at).expect("the manifest"))
            .expect("it reads");
    manifest["blender"]["version"] = serde_json::Value::from(
        std::env::var("SMOOZY_DESK_VERSION").unwrap_or_else(|_| "5.2.2".to_string()),
    );
    std::fs::write(
        &manifest_at,
        serde_json::to_vec(&manifest).expect("it writes"),
    )
    .expect("written back");
    println!(
        "the manifest's job output folder: {}",
        manifest["job_output_folder"]
    );
    area.start_the_supervisor();
    submitted(&area, &job);
    let note = the_note_once_it_has_ended(&area, &job);
    println!(
        "the note's ending: {:?}, {:?}",
        note.fields.text(smoozy_framing::requests::THE_ENDING_FIELD),
        note.fields
            .text(smoozy_framing::requests::THE_REASON_THE_MACHINE_RECORDED_FIELD)
    );
    let recorded = std::fs::read(record_directory.join(
        smoozy_job_files_machine::where_the_folder_sits::THE_RECORD_OF_WHERE_THE_FOLDER_SITS,
    ))
    .ok()
    .and_then(|bytes| {
        smoozy_job_files_machine::where_the_folder_sits::WhereTheJobOutputFolderSits::read_back(
            &bytes,
        )
    })
    .expect("the render worker recorded where the job's folder sits");
    println!("recorded: {recorded:?}");
    println!("made: {}", Path::new(&recorded.path).is_dir());
    let reading = std::fs::read(
        record_directory
            .join(smoozy_job_files_machine::manifest::the_recorded_readings_file_name(1)),
    )
    .ok()
    .and_then(|bytes| {
        smoozy_job_files_machine::composing::TheReadingAndWhatFollows::read_back(&bytes)
    })
    .expect("the reading the worker took");
    for pair in &reading.map.pairs {
        if let smoozy_job_files_machine::mapping::HowItMapped::UnderTheJobsFolder {
            new_folder,
            ..
        } = &pair.outcome
        {
            println!(
                "the map: {:?} -> {}",
                pair.destination.kind,
                String::from_utf8_lossy(new_folder)
            );
        }
    }
    let name = smoozy_job_files_machine::names::job_output_folder_name(&job, Some("Kitchen"));
    if std::env::var("SMOOZY_DESK_BLEND_UNDER_THE_ROOT").is_err() && fallback.is_none() {
        assert_eq!(
            recorded.path,
            home.join("home/ubuntu/Render")
                .join(&name)
                .display()
                .to_string()
        );
        assert_eq!(
            recorded.where_it_is,
            smoozy_job_files_machine::where_the_folder_sits::WhereItIs::WhereTheFilePoints
        );
    }
    assert!(Path::new(&recorded.path).is_dir(), "the folder was made");
    std::fs::remove_dir_all(&home).ok();
}

// ---------------------------------------------------------------------------
// Stop and clear — step 10c's first stop-and-clear piece
// ---------------------------------------------------------------------------

/// **The jobs the machine-local queue holds, read off the disk** — what the
/// supervisor wrote, not what it answers.
fn the_jobs_on_the_disk(area: &APackageArea) -> Vec<Identity> {
    let queue = smoozy_queue_history_machine::acting::queue_store::TheQueueOnTheMachine::at(
        area.where_a_store_sits(
            smoozy_queue_history_machine::acting::queue_store::THE_AREA_THE_QUEUE_SITS_IN,
        ),
    );
    queue
        .every_record()
        .expect("the queue reads")
        .0
        .into_iter()
        .map(|record| record.job)
        .collect()
}

/// **`Queue & History` L13's two halves, as the desktop composes them**: each
/// names the other, and neither names a target.
fn the_clear(clear: &Identity, stop: &Identity) -> Request {
    use smoozy_framing::requests as asked;
    Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        clear.clone(),
        asked::THE_CLEAR_REQUEST,
    )
    .carrying(
        smoozy_framing::framing::Fields::none().and(asked::THE_PAIRED_STOP_FIELD, stop.as_text()),
    )
}

fn the_paired_stop(stop: &Identity, clear: &Identity) -> Request {
    use smoozy_framing::requests as asked;
    Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        stop.clone(),
        asked::THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST,
    )
    .carrying(
        smoozy_framing::framing::Fields::none().and(asked::THE_PAIRED_CLEAR_FIELD, clear.as_text()),
    )
}

/// **Three jobs on a machine whose line is held**: two waiting and one held
/// behind the first, each with its material published — the machine the user
/// asks to have emptied.
fn three_jobs_on_a_held_line(area: &APackageArea, root: &ARoot) -> (Identity, Identity, Identity) {
    use smoozy_framing::framing::Fields;
    use smoozy_framing::requests as asked;
    the_line_is_held_by_a_setup_run(area);
    let first = root.a_published_job();
    submitted(area, &first);
    let second = root.a_published_job();
    submitted(area, &second);
    let held = root.a_published_job();
    let answered = asked_through_the_adapter(
        area,
        &a_request_of(
            asked::THE_JOB_RECORD_IS_WRITTEN_REQUEST,
            Fields::none()
                .and(asked::THE_JOB_FIELD, held.as_text())
                .and(asked::THE_HELD_BEHIND_FIELD, first.as_text())
                .and(asked::THE_HOLD_REASON_FIELD, "the first-frame check"),
        ),
    );
    assert_eq!(
        answered.fields.text(asked::THE_QUEUE_STATE_FIELD),
        Some(asked::IT_IS_HELD)
    );
    (first, second, held)
}

/// **`Queue & History` L13, as the user meets it: the clear and its stop sent
/// as two requests, and the line drops between them.** The stop's request
/// never arrives, and the machine carries the pair out anyway — every job it
/// held is gone, each removal journalled with the job's identity (L23's last
/// bullet), each job's material cleaned up after it (L14, L1), and the stop
/// recorded and resolved with nobody connected. **The desktop learns the
/// stop's outcome by asking its identity again**, which is C8's route.
///
/// **Then the two identities are sent again** — the clear after a new job
/// arrived — **and nothing acts twice**: both are answered from their records,
/// and the new job stands.
///
/// **What this does not pose is a render to stop**: written when no render
/// could be launched at the desk (until 2026-10-24), it asserts only that both
/// are answered from their records. The interrupt, the issuance
/// and *left stopped* are proved in the program's own tests over a stand-in
/// render process and a kernel-shaped record.
#[test]
fn a_clear_and_its_paired_stop_both_happen_when_the_line_drops_between_them() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;

    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    area.start_the_supervisor();
    let (first, second, held) = three_jobs_on_a_held_line(&area, &root);
    assert_eq!(the_jobs_on_the_disk(&area).len(), 3);

    let clear = Identity::issue(IdentityKind::Operation).expect("randomness");
    let stop = Identity::issue(IdentityKind::Operation).expect("randomness");

    // **The clear arrives. The stop's request never does.**
    let cleared = asked_through_the_adapter(&area, &the_clear(&clear, &stop));
    assert_eq!(
        cleared.the_request_name_it_says_it_does_not_answer(),
        None,
        "the clear is answered, not refused by name"
    );
    assert_eq!(
        cleared.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME),
        "{:?}",
        cleared.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
    );
    assert_eq!(
        cleared.fields.text(asked::THE_CLEARS_OUTCOME_FIELD),
        Some(asked::IT_WAS_CLEARED)
    );
    assert!(
        the_jobs_on_the_disk(&area).is_empty(),
        "the machine is emptied"
    );

    // **The stop was recorded from the clear's own arrival**, before anything
    // acted — which is what makes the dropped line change nothing.
    assert!(
        area.holds_a_record_under(&stop),
        "the stop is recorded on the machine"
    );

    // **Each removal is journalled, about the job and nothing else.**
    let removed: Vec<String> = every_event(&area)
        .into_iter()
        .filter(|(kind, _, _)| kind == asked::THE_JOB_RECORD_REMOVED_EVENT)
        .map(|(_, about, _)| about)
        .collect();
    for job in [&first, &second, &held] {
        assert!(
            removed.contains(&job.as_text()),
            "{} was not journalled",
            job.as_text()
        );
    }

    // **The material goes after the record** (L1): each job record directory.
    let the_root =
        smoozy_job_files_machine::layout::MachineRoot::chosen(&root.at.display().to_string())
            .expect("a root");
    for job in [&first, &second, &held] {
        let directory =
            smoozy_job_files_machine::publication::the_job_record_directory(&the_root, job);
        assert!(
            !Path::new(&directory).exists(),
            "{directory} is still there"
        );
    }

    // **The desktop asks the stop's identity on a later connection.**
    let stopped = asked_through_the_adapter(&area, &the_paired_stop(&stop, &clear));
    assert_eq!(stopped.the_request_name_it_says_it_does_not_answer(), None);
    assert_eq!(
        stopped.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME)
    );
    assert_eq!(
        stopped.fields.text(asked::THE_STOPS_OUTCOME_FIELD),
        Some(asked::NOTHING_WAS_RENDERING)
    );

    // **The same two identities again, after a new job arrived: nothing acts
    // twice.**
    let later = root.a_published_job();
    submitted(&area, &later);
    let again = asked_through_the_adapter(&area, &the_clear(&clear, &stop));
    assert_eq!(
        again.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_HAS_A_RECORDED_OUTCOME)
    );
    assert_eq!(
        again.fields.text(asked::THE_CLEARS_OUTCOME_FIELD),
        Some(asked::IT_WAS_CLEARED)
    );
    let again = asked_through_the_adapter(&area, &the_paired_stop(&stop, &clear));
    assert_eq!(
        again.fields.text(asked::THE_STOPS_OUTCOME_FIELD),
        Some(asked::NOTHING_WAS_RENDERING)
    );
    assert_eq!(
        the_jobs_on_the_disk(&area),
        vec![later],
        "a repeat of the clear's identity removed nothing"
    );
}

/// **`Queue & History` L23 and D14, as the user meets them: a clear the store
/// refuses part-way leaves its record standing, and the next start finishes it
/// before the admission point may start anything.**
///
/// **The refusal is the queue's own directory refusing removals** — the shape a
/// full or read-only mount gives, and one every unix keeps. It refuses the
/// first removal rather than the second, because no per-file refusal of an
/// unlink exists on a unix; the removal part-way past the first is proved in
/// `Queue & History — machine`'s own tests, where a record's name can be taken
/// by a directory.
///
/// **Then the line is let go**: the setup-in-progress state is taken away while
/// the supervisor is down, so a start that admitted before it finished the
/// clear would open a render operation for one of the jobs the user asked to
/// have cleared. **None is opened.**
#[test]
fn a_clear_refused_part_way_is_finished_by_the_next_start_before_anything_is_admitted() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use std::os::unix::fs::PermissionsExt;

    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    area.start_the_supervisor();
    let (first, second, held) = three_jobs_on_a_held_line(&area, &root);

    let queue_area = area.where_a_store_sits(
        smoozy_queue_history_machine::acting::queue_store::THE_AREA_THE_QUEUE_SITS_IN,
    );
    std::fs::set_permissions(&queue_area, std::fs::Permissions::from_mode(0o555))
        .expect("the queue's directory refuses removals");

    let clear = Identity::issue(IdentityKind::Operation).expect("randomness");
    let stop = Identity::issue(IdentityKind::Operation).expect("randomness");
    let refused = asked_through_the_adapter(&area, &the_clear(&clear, &stop));
    std::fs::set_permissions(&queue_area, std::fs::Permissions::from_mode(0o755))
        .expect("the directory is given back");
    assert_eq!(refused.the_request_name_it_says_it_does_not_answer(), None);
    assert_eq!(
        refused.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND),
        "a clear refused part-way is outstanding, not ended"
    );
    assert!(refused
        .fields
        .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        .is_some());
    assert_eq!(the_jobs_on_the_disk(&area).len(), 3, "nothing was removed");
    let acts = area.where_a_store_sits(
        smoozy_queue_history_machine::acting::commands::THE_AREA_THE_ACT_RECORDS_SIT_IN,
    );
    assert_eq!(
        std::fs::read_dir(&acts)
            .expect("the act records' area")
            .count(),
        1,
        "the clear's own record stands"
    );

    // **The machine goes down, and comes back with its line let go.**
    area.kill_the_supervisor();
    std::fs::remove_file(area.where_a_store_sits(records::THE_SETUP_IN_PROGRESS_STATE))
        .expect("the setup-in-progress state is taken away");
    area.start_the_supervisor();

    assert!(
        the_jobs_on_the_disk(&area).is_empty(),
        "the next start finished the clear"
    );
    assert_eq!(
        std::fs::read_dir(&acts)
            .expect("the act records' area")
            .count(),
        0,
        "and cleared its record"
    );
    let operations = TheOperationRecordsOnTheMachine::in_the_area(&area.where_a_store_sits(
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
    ))
    .expect("the store opens");
    let renders: Vec<_> = operations
        .every_record_eligible_for_reconciliation()
        .expect("it reads")
        .into_iter()
        .filter(|record| record.kind == "render")
        .collect();
    assert!(renders.is_empty(), "nothing was admitted: {renders:?}");
    for job in [&first, &second, &held] {
        let removed = every_event(&area).into_iter().any(|(kind, about, _)| {
            kind == asked::THE_JOB_RECORD_REMOVED_EVENT && about == job.as_text()
        });
        assert!(removed, "{} was not journalled", job.as_text());
    }

    // **Both identities now answer from their records.**
    let cleared = asked_through_the_adapter(&area, &the_clear(&clear, &stop));
    assert_eq!(
        cleared.fields.text(asked::THE_CLEARS_OUTCOME_FIELD),
        Some(asked::IT_WAS_CLEARED)
    );
    let stopped = asked_through_the_adapter(&area, &the_paired_stop(&stop, &clear));
    assert_eq!(
        stopped.fields.text(asked::THE_STOPS_OUTCOME_FIELD),
        Some(asked::NOTHING_WAS_RENDERING)
    );
}

/// **`Queue & History` L1 and L5 through the real program: a waiting job and a
/// held job cancelled, each removal journalled, the held job's hold record
/// gone with its subject** (`Rented-server runtime` U22's third ending), **and
/// a job that is not there answered as removed**, because a cancel is reported
/// done exactly when the machine's copy is gone (`Queue & History` L2).
#[test]
fn a_waiting_job_and_a_held_one_are_cancelled_and_their_holds_go_with_them() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::framing::Fields;
    use smoozy_framing::requests as asked;

    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    area.start_the_supervisor();
    let (first, second, held) = three_jobs_on_a_held_line(&area, &root);
    let holds_area = area.where_a_store_sits(records::THE_AREA_THE_HOLD_RECORDS_SIT_IN);
    let holds_before = std::fs::read_dir(&holds_area)
        .expect("the holds' area")
        .count();
    assert_eq!(holds_before, 1);

    for job in [&second, &held] {
        let cancelled = asked_through_the_adapter(
            &area,
            &a_request_of(
                asked::THE_CANCEL_REQUEST,
                Fields::none().and(asked::THE_JOB_FIELD, job.as_text()),
            ),
        );
        assert_eq!(
            cancelled.the_request_name_it_says_it_does_not_answer(),
            None
        );
        assert_eq!(
            cancelled.fields.text(asked::THE_CANCELS_OUTCOME_FIELD),
            Some(asked::IT_WAS_REMOVED),
            "{:?}",
            cancelled.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        );
    }
    assert_eq!(the_jobs_on_the_disk(&area), vec![first]);
    assert_eq!(
        std::fs::read_dir(&holds_area)
            .expect("the holds' area")
            .count(),
        0,
        "the hold went with its subject"
    );
    let removed: Vec<String> = every_event(&area)
        .into_iter()
        .filter(|(kind, _, _)| kind == asked::THE_JOB_RECORD_REMOVED_EVENT)
        .map(|(_, about, _)| about)
        .collect();
    assert!(removed.contains(&second.as_text()) && removed.contains(&held.as_text()));
}

/// **The hold a held job is under, read through the real adapter.**
fn the_hold_it_is_under(area: &APackageArea, job: &Identity) -> Identity {
    use smoozy_framing::framing::Fields;
    use smoozy_framing::requests as asked;
    let read = asked_through_the_adapter(
        area,
        &a_request_of(
            asked::THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
            Fields::none().and(asked::THE_JOB_FIELD, job.as_text()),
        ),
    );
    Identity::read_back(
        IdentityKind::Operation,
        read.fields
            .text(asked::THE_HOLD_IT_IS_HELD_UNDER_FIELD)
            .expect("it is held"),
    )
    .expect("a hold identity")
}

/// **`Rented-server runtime` U12a through the real program: a *no* removes its
/// subject, by either route, and leaves nothing behind** — `Specs/Queue &
/// History`: *"Saying no leaves nothing behind … Cancelling it and saying no
/// are one act, not two."* Step 10c's eleventh-pass fix piece 8b.
///
/// **Until then the machine's *no* wrote *stopped* and removed nothing**, so a
/// *no* from the phone left the job held on the machine for ever, in every
/// queue reading, and only the desk's *Cancel it* — which sent a cancel after
/// its *no* — took it away.
///
/// **The same *no* sent again is answered from its record** and acts on
/// nothing: the hold record went with its subject, so the answer is read off
/// the command's own recorded outcome (`Server protocol and transfers` C4, C8).
#[test]
fn a_no_by_either_route_removes_the_held_job_and_leaves_nothing_behind() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::framing::Fields;
    use smoozy_framing::requests as asked;

    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    area.start_the_supervisor();
    let (first, second, held) = three_jobs_on_a_held_line(&area, &root);
    let hold = the_hold_it_is_under(&area, &held);
    let holds_area = area.where_a_store_sits(records::THE_AREA_THE_HOLD_RECORDS_SIT_IN);

    let no = a_request_of(
        asked::THE_ANSWER_TO_A_HOLD_REQUEST,
        Fields::none()
            .and(asked::THE_HOLD_FIELD, hold.as_text())
            .and(asked::THE_ANSWER_FIELD, asked::THE_ANSWER_IS_NO)
            .and(asked::THE_ROUTE_FIELD, asked::THE_ROUTE_IS_THE_PHONE),
    );
    let answered = asked_through_the_adapter(&area, &no);
    assert_eq!(
        answered.fields.text(asked::THE_RELEASE_OUTCOME_FIELD),
        Some("stopped")
    );
    let on_the_disk = the_jobs_on_the_disk(&area);
    assert!(
        !on_the_disk.contains(&held)
            && on_the_disk.contains(&first)
            && on_the_disk.contains(&second),
        "the held job is gone from the machine, and nothing else: {on_the_disk:?}"
    );
    assert_eq!(
        answered.fields.text(asked::THE_CANCELS_OUTCOME_FIELD),
        Some(asked::IT_WAS_REMOVED),
        "the answer says the job is gone, so the desk sends no cancel after it"
    );
    assert_eq!(
        std::fs::read_dir(&holds_area)
            .expect("the holds' area")
            .count(),
        0,
        "the hold went with its subject (`Rented-server runtime` U22)"
    );
    let events = every_event(&area);
    let release_at = events
        .iter()
        .position(|(kind, about, _)| kind == asked::THE_RELEASE_EVENT && *about == hold.as_text())
        .expect("the release is journalled (`Rented-server runtime` U26)");
    let removal_at = events
        .iter()
        .position(|(kind, about, _)| {
            kind == asked::THE_JOB_RECORD_REMOVED_EVENT && *about == held.as_text()
        })
        .expect("the removal is journalled, so the desktop takes its local record");
    assert!(
        release_at < removal_at,
        "the answer is written first, then the job goes"
    );

    // **The same *no* again**: answered from its record, and nothing acts.
    let again = asked_through_the_adapter(&area, &no);
    assert_eq!(
        again.fields.text(asked::THE_RELEASE_OUTCOME_FIELD),
        Some("stopped")
    );
    assert_eq!(
        again.fields.text(asked::THE_CANCELS_OUTCOME_FIELD),
        Some(asked::IT_WAS_REMOVED)
    );
    assert_eq!(
        the_jobs_on_the_disk(&area).len(),
        2,
        "nothing acted a second time"
    );
}

/// **`Queue & History` A1 through the real program: a held job's item says why
/// it is held and what it waits behind** — `Queue & History` H1 and H6 build
/// the entry from the hold record, and a Smoozy opened after the submission
/// has nothing else to build it from. Step 10c's eleventh-pass fix piece 8b.
///
/// **Until then the item carried the hold identity alone**, so the desk drew
/// *Render the rest* and *Cancel it* only for a check the same run of Smoozy
/// had sent.
#[test]
fn the_queue_reading_says_why_a_held_job_is_held_and_what_it_waits_behind() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::framing::Fields;
    use smoozy_framing::requests as asked;

    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    area.start_the_supervisor();
    let (first, second, held) = three_jobs_on_a_held_line(&area, &root);

    let read = asked_through_the_adapter(
        &area,
        &a_request_of(asked::THE_QUEUE_READING_REQUEST, Fields::none()),
    );
    let item = |job: &Identity, name: &str| {
        (0..3)
            .find(|at| {
                read.fields.text(&asked::the_field_of_item(
                    asked::THE_JOB_FIELD,
                    *at,
                    asked::THE_JOB_FIELD,
                )) == Some(job.as_text().as_str())
            })
            .and_then(|at| {
                read.fields
                    .text(&asked::the_field_of_item(asked::THE_JOB_FIELD, at, name))
            })
            .map(str::to_string)
    };
    assert_eq!(
        item(&held, asked::THE_QUEUE_STATE_FIELD).as_deref(),
        Some(asked::IT_IS_HELD)
    );
    assert_eq!(
        item(&held, asked::THE_HOLD_REASON_FIELD).as_deref(),
        Some("the first-frame check")
    );
    assert_eq!(
        item(&held, asked::THE_HELD_BEHIND_FIELD),
        Some(first.as_text())
    );
    // **A job that is not held carries neither.**
    assert_eq!(item(&second, asked::THE_HOLD_REASON_FIELD), None);
    assert_eq!(item(&second, asked::THE_HELD_BEHIND_FIELD), None);
}

/// **`Rendering` S18a through the real program: a job held for the driver names
/// no dependency, no predicate is ever evaluated for it, and it goes on only at
/// an answer** — the desktop's *go on*, `Rented-server runtime` U9's ordinary
/// command (`Server Setup` R1c). Held-job piece A, 2026-10-12.
///
/// **The reason is carried and never interpreted** (`Rented-server runtime`
/// U2): this program cannot reach `Rendering`'s declaration of it, so the test
/// writes its words, and the machine would place the same hold under any
/// reason. **Until this piece the request was refused by name** as half an
/// instruction, so the hold could not be placed at all.
///
/// **The removal of another job is the one moment a hold's predicate is
/// evaluated on this machine** (`Queue & History` H9), so a cancel is sent
/// first: the job held for the driver stands through it.
#[test]
fn a_job_held_for_the_driver_names_no_dependency_and_goes_on_only_at_an_answer() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::framing::Fields;
    use smoozy_framing::requests as asked;

    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    area.start_the_supervisor();
    the_line_is_held_by_a_setup_run(&area);
    let first = root.a_published_job();
    submitted(&area, &first);

    let held = root.a_published_job();
    let answered = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_JOB_RECORD_IS_WRITTEN_REQUEST,
            Fields::none()
                .and(asked::THE_JOB_FIELD, held.as_text())
                .and(asked::THE_HOLD_REASON_FIELD, "the driver"),
        ),
    );
    assert_eq!(answered.the_request_name_it_says_it_does_not_answer(), None);
    assert_eq!(
        answered.fields.text(asked::THE_QUEUE_STATE_FIELD),
        Some(asked::IT_IS_HELD)
    );
    let hold = the_hold_it_is_under(&area, &held);
    let holds = smoozy_runtime::acting::stores::TheHoldRecordsOnTheMachine::in_the_area(
        &area.where_a_store_sits(records::THE_AREA_THE_HOLD_RECORDS_SIT_IN),
    )
    .expect("the area is there");
    let placed = smoozy_runtime::holds::KeepsHoldRecords::read(&holds, &hold)
        .expect("it reads")
        .expect("the hold record is on the disk");
    assert_eq!(
        placed.subject,
        smoozy_runtime::holds::Subject::Job(held.clone())
    );
    assert_eq!(placed.origin, smoozy_runtime::holds::Origin::Submission);
    assert_eq!(placed.reason, "the driver");
    assert_eq!(placed.dependency, None, "no dependency and no predicate");
    assert!(placed.release.is_none());

    // **The queue reading says why, and names nothing it waits behind.**
    let item = |read: &smoozy_framing::framing::Answered, job: &Identity, name: &str| {
        (0..4)
            .find(|at| {
                read.fields.text(&asked::the_field_of_item(
                    asked::THE_JOB_FIELD,
                    *at,
                    asked::THE_JOB_FIELD,
                )) == Some(job.as_text().as_str())
            })
            .and_then(|at| {
                read.fields
                    .text(&asked::the_field_of_item(asked::THE_JOB_FIELD, at, name))
            })
            .map(str::to_string)
    };
    let read = asked_through_the_adapter(
        &area,
        &a_request_of(asked::THE_QUEUE_READING_REQUEST, Fields::none()),
    );
    assert_eq!(
        item(&read, &held, asked::THE_QUEUE_STATE_FIELD).as_deref(),
        Some(asked::IT_IS_HELD)
    );
    assert_eq!(
        item(&read, &held, asked::THE_HOLD_REASON_FIELD).as_deref(),
        Some("the driver")
    );
    assert_eq!(item(&read, &held, asked::THE_HELD_BEHIND_FIELD), None);
    assert_eq!(
        item(&read, &held, asked::THE_PLACE_IN_THE_LINE_FIELD),
        None,
        "no place in the line"
    );

    // **Another job's removal evaluates nothing for it.**
    let cancelled = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_CANCEL_REQUEST,
            Fields::none().and(asked::THE_JOB_FIELD, first.as_text()),
        ),
    );
    assert_eq!(
        cancelled.fields.text(asked::THE_CANCELS_OUTCOME_FIELD),
        Some(asked::IT_WAS_REMOVED)
    );
    assert_eq!(the_jobs_on_the_disk(&area), vec![held.clone()]);
    let still = smoozy_runtime::holds::KeepsHoldRecords::read(&holds, &hold)
        .expect("it reads")
        .expect("the hold stands");
    assert!(still.release.is_none(), "nothing dropped it");

    // **The desk's *go on*: released, and in the line.**
    let go_on = asked_through_the_adapter(
        &area,
        &a_request_of(
            asked::THE_ANSWER_TO_A_HOLD_REQUEST,
            Fields::none()
                .and(asked::THE_HOLD_FIELD, hold.as_text())
                .and(asked::THE_ANSWER_FIELD, asked::THE_ANSWER_IS_YES)
                .and(asked::THE_ROUTE_FIELD, asked::THE_ROUTE_IS_THE_DESK),
        ),
    );
    assert_eq!(
        go_on.fields.text(asked::THE_RELEASE_OUTCOME_FIELD),
        Some("released")
    );
    let read = asked_through_the_adapter(
        &area,
        &a_request_of(asked::THE_QUEUE_READING_REQUEST, Fields::none()),
    );
    assert_eq!(
        item(&read, &held, asked::THE_QUEUE_STATE_FIELD).as_deref(),
        Some(asked::IT_IS_WAITING)
    );
    assert_eq!(
        item(&read, &held, asked::THE_PLACE_IN_THE_LINE_FIELD).as_deref(),
        Some("1")
    );
}

/// **`Queue & History` D14 at the line, in the same run: while a clear refused part-way
/// stands, the line admits nothing.** The line is held by a setup run while
/// the clear is refused; the store is given back; then the setup-in-progress
/// state is cleared through the ordinary command path, which moves the line —
/// and **the line finishes the clear before it looks for a job to start**, so
/// no job the user asked to have cleared is ever given a render.
#[test]
fn a_clear_refused_part_way_holds_the_line_in_the_same_run_until_it_is_finished() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    use std::os::unix::fs::PermissionsExt;

    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    area.start_the_supervisor();
    let run = the_line_is_held_by_a_setup_run(&area);
    let first = root.a_published_job();
    submitted(&area, &first);
    let second = root.a_published_job();
    submitted(&area, &second);

    let queue_area = area.where_a_store_sits(
        smoozy_queue_history_machine::acting::queue_store::THE_AREA_THE_QUEUE_SITS_IN,
    );
    std::fs::set_permissions(&queue_area, std::fs::Permissions::from_mode(0o555))
        .expect("the queue's directory refuses removals");
    let clear = Identity::issue(IdentityKind::Operation).expect("randomness");
    let stop = Identity::issue(IdentityKind::Operation).expect("randomness");
    let refused = asked_through_the_adapter(&area, &the_clear(&clear, &stop));
    std::fs::set_permissions(&queue_area, std::fs::Permissions::from_mode(0o755))
        .expect("the directory is given back");
    assert_eq!(
        refused.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
        Some(asked::IT_IS_STILL_IN_HAND)
    );

    // **The line is let go, in this run.**
    asked_through_the_adapter(
        &area,
        &a_request_of(
            smoozy_runtime::acting::stores::THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST,
            smoozy_framing::framing::Fields::none().and(
                smoozy_runtime::acting::stores::THE_SETUP_RUN_FIELD,
                run.as_text(),
            ),
        ),
    );
    // One more turn, so the line has been taken after the clear of the state.
    area.waited_until_it_answers();

    assert!(
        the_jobs_on_the_disk(&area).is_empty(),
        "the line finished the clear"
    );
    let operations = TheOperationRecordsOnTheMachine::in_the_area(&area.where_a_store_sits(
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
    ))
    .expect("the store opens");
    let started_for_a_cleared_job = operations
        .every_record_eligible_for_reconciliation()
        .expect("it reads")
        .into_iter()
        .any(|record| record.subject == first || record.subject == second);
    assert!(
        !started_for_a_cleared_job,
        "no cleared job was given a render"
    );
    let cleared = asked_through_the_adapter(&area, &the_clear(&clear, &stop));
    assert_eq!(
        cleared.fields.text(asked::THE_CLEARS_OUTCOME_FIELD),
        Some(asked::IT_WAS_CLEARED)
    );
}

/// **A first-frame job and the job held behind it, then the first-frame job
/// left on the disk the way a restart finds a render that has gone** — the
/// shape [`a_job_a_restart_ends_says_which_frames_it_saved`] poses, with
/// **no frame saved**. `stopped` writes *should be stopped* on the open
/// record, which §6 then ends *left stopped*; otherwise the record is at its
/// second attempt, which §6 gives up. The supervisor must be stopped, and is
/// left stopped. The held job's label is the one given.
fn a_first_frame_job_a_restart_ends_with_no_frame(
    area: &APackageArea,
    root: &ARoot,
    stand_in: &Path,
    label: &str,
    stopped: bool,
) -> (Identity, Identity) {
    use smoozy_framing::framing::Fields;
    use smoozy_framing::requests as asked;
    let first = root.a_job_the_render_worker_can_take_over(
        stand_in,
        smoozy_job_files_machine::manifest::FrameRange::TheFilesOwn,
    );
    let record_directory = root.at.join("job-records").join(first.as_text());
    let manifest = smoozy_job_files_machine::manifest::SubmissionManifest::read_back(
        &std::fs::read(
            record_directory.join(smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME),
        )
        .expect("the manifest"),
    )
    .expect("it reads back");
    let mut printed: smoozy_job_files_machine::reading_script::ThePrintedReading =
        serde_json::from_str(&the_document_a_reading_prints()).expect("it reads");
    printed.settings.0["scenes"][0]["frame_end"] = serde_json::json!(1);
    printed.settings.0["scenes"][0]["frame_count"] = serde_json::json!(1);
    let enters_at = smoozy_job_files_machine::main_output::WhereTheRenderEnters {
        scene: smoozy_job_files_machine::reading::ArtistsText::of(b"Scene".to_vec()),
        view_layer: smoozy_job_files_machine::reading::ArtistsText::of(b"ViewLayer".to_vec()),
    };
    let folder = root.at.display().to_string();
    let composed = smoozy_job_files_machine::composing::the_reading_and_everything_derived_from(
        printed,
        smoozy_job_files_machine::composing::WhatTheReadingIsComposedAgainst {
            the_folder_the_blend_sits_in: folder.as_bytes(),
            job_output_folder: manifest
                .job_output_folder
                .the_path_it_names()
                .expect("named"),
            enters_at: &enters_at,
            the_jobs_pins: &[],
        },
    )
    .expect("the reading composes");
    std::fs::write(
        record_directory
            .join(smoozy_job_files_machine::manifest::the_recorded_readings_file_name(1)),
        serde_json::to_vec(&composed).expect("it renders"),
    )
    .expect("the recorded reading is written");

    submitted(area, &first);
    let held = root.a_published_job();
    let answered = asked_through_the_adapter(
        area,
        &a_request_of(
            asked::THE_JOB_RECORD_IS_WRITTEN_REQUEST,
            Fields::none()
                .and(asked::THE_JOB_FIELD, held.as_text())
                .and(asked::THE_LABEL_FIELD, label)
                .and(asked::THE_HELD_BEHIND_FIELD, first.as_text())
                .and(asked::THE_HOLD_REASON_FIELD, "the first-frame check"),
        ),
    );
    assert_eq!(
        answered.fields.text(asked::THE_QUEUE_STATE_FIELD),
        Some(asked::IT_IS_HELD)
    );

    let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
    let mut queue = smoozy_queue_history_machine::acting::queue_store::TheQueueOnTheMachine::at(
        area.where_a_store_sits(
            smoozy_queue_history_machine::acting::queue_store::THE_AREA_THE_QUEUE_SITS_IN,
        ),
    );
    let mut record = queue
        .read_one(&first)
        .expect("it reads")
        .expect("it is there");
    record.state = smoozy_queue_history_machine::line::QueueState::Rendering {
        operation: operation.clone(),
    };
    queue.write(&record).expect("the record is rendering");
    let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area.where_a_store_sits(
        smoozy_runtime::acting::operation_records::THE_AREA_THE_TWO_RECORDS_SIT_IN,
    ))
    .expect("the store is there");
    let mut open = OperationRecord::opened(operation.clone(), first.clone(), "render");
    open.execution_group = Some(TheRecordedExecutionGroup {
        as_that_document_recorded_it: RecordedPair {
            identifier: AN_IDENTIFIER_NOTHING_IS_AT,
            start_time: 1,
        }
        .as_this_document_records_it(),
        started_under_run: Identity::issue(IdentityKind::Run).expect("randomness"),
    });
    if stopped {
        open.the_stop_was_asked_for().expect("it is open");
    } else {
        open.attempt_count = 2;
    }
    store
        .record_before_acting(&open)
        .expect("the record is written");
    store.write_it_back(&open).expect("the record is written");
    (first, held)
}

/// **`Specs/Queue & History`'s *Where the first frame never arrives* and
/// `Queue & History` H8, through the real program: a first-frame job stopped
/// with no frame saved drops the job held behind it** — the held job's record
/// and its hold record gone, the release *dropped* journalled with the ending
/// as its reason, and the removal journalled next to it so the desktop takes
/// its local record. **The drop's entry carries what the phone is told from**
/// (`Rented-server runtime` U20 as dated 2026-10-16): the reason, the held
/// count, the first frame and the held job's label, ahead of the words every
/// release says. Added 2026-10-16, the Telegram worker's piece 4.
///
/// **A first-frame job given up after a second interruption is not dropped
/// behind here, and that is H8's own rule**: reconciliation's *given up*
/// carries no completeness statement on this machine (it says so on its
/// stream), and where the ending carries no reading of a job that ran, the
/// predicate is not evaluated — that job may have written its first frame, and
/// a drop cannot be undone. The hold stands with its own answer still the
/// route. **A given-up ending that does carry a reading drops**, and so does a
/// failure: those two are the program's own test
/// (`unix::tests::a_first_frame_job_that_fails_is_stopped_or_is_given_up_drops_its_held_job_told_once`),
/// since a render that fails needs a render to run, which this desk's kernel
/// cannot record.
///
/// **Before this piece only a first frame cancelled before it ran dropped
/// anything** (`Queue & History` H9), and the stopped job's held job here
/// waited for an answer for ever.
#[test]
fn a_first_frame_job_stopped_with_no_frame_drops_its_held_job_through_the_real_program() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    let mut area = APackageArea::made();
    let root = ARoot::recorded_in(&area);
    let (stand_in, _) = a_stand_in_for_blender(&root);
    area.start_the_supervisor();
    the_line_is_held_by_a_setup_run(&area);
    let (stopped_first, stopped_held) = a_first_frame_job_a_restart_ends_with_no_frame(
        &area,
        &root,
        &stand_in,
        "Kitchen rest",
        true,
    );
    let (given_up_first, given_up_held) = a_first_frame_job_a_restart_ends_with_no_frame(
        &area,
        &root,
        &stand_in,
        "Porch rest",
        false,
    );
    let (hold, standing_hold) = (
        the_hold_it_is_under(&area, &stopped_held),
        the_hold_it_is_under(&area, &given_up_held),
    );
    area.kill_the_supervisor();
    std::fs::remove_file(area.where_a_store_sits(records::THE_SETUP_IN_PROGRESS_STATE))
        .expect("the setup state is taken away");

    area.start_the_supervisor();
    assert_eq!(
        the_note_once_it_has_ended(&area, &stopped_first)
            .fields
            .text(asked::THE_ENDING_FIELD),
        Some(asked::IT_WAS_LEFT_STOPPED)
    );
    assert_eq!(
        the_note_once_it_has_ended(&area, &given_up_first)
            .fields
            .text(asked::THE_ENDING_FIELD),
        Some(asked::IT_WAS_GIVEN_UP)
    );

    let on_the_disk = the_jobs_on_the_disk(&area);
    assert!(
        !on_the_disk.contains(&stopped_held),
        "the stopped first frame's held job was dropped: {}",
        the_complaints_of(&area)
    );
    assert!(
        on_the_disk.contains(&given_up_held),
        "a given-up ending with no reading drops nothing (H8)"
    );
    let holds_area = area.where_a_store_sits(records::THE_AREA_THE_HOLD_RECORDS_SIT_IN);
    let left: Vec<String> = std::fs::read_dir(&holds_area)
        .expect("the holds' area")
        .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
        .collect();
    assert_eq!(
        left.len(),
        1,
        "the dropped hold went with its subject, and the other stands: {left:?}"
    );
    assert!(left[0].contains(&standing_hold.as_text()), "{left:?}");

    let events = every_event(&area);
    let at = events
        .iter()
        .position(|(kind, about, _)| kind == asked::THE_RELEASE_EVENT && *about == hold.as_text())
        .expect("the drop's release is journalled");
    assert_eq!(
        events[at].2,
        "the dependency was stopped: 3: 1: Kitchen rest: the hold's release outcome was written: dropped",
        "the entry carries the reason, the held frames, the first frame and the label, and ends in the outcome"
    );
    assert_eq!(
        (events[at + 1].0.as_str(), events[at + 1].1.as_str()),
        (
            asked::THE_JOB_RECORD_REMOVED_EVENT,
            stopped_held.as_text().as_str()
        ),
        "the removal is journalled next to the release"
    );
    assert!(
        !events
            .iter()
            .any(|(kind, about, _)| kind == asked::THE_RELEASE_EVENT
                && *about == standing_hold.as_text()),
        "nothing was released for the hold that stands"
    );
}

/// **`Telegram` Y1 with `State, identity and recovery` Y7a, through the real
/// program: the role worker's *try it again* makes one job on the machine,
/// with a new identity and a copied manifest, journalled — and a second press
/// finds the recorded one and makes nothing.** Added 2026-10-16, the Telegram
/// worker's piece 5.
///
/// **The caller is the role worker this run started**, the stand-in that asks
/// when told and then becomes the SSH command adapter in its own process
/// ([`a_role_worker_that_asks_when_told`]), so the supervisor singles it out
/// by the process exactly as it does the real bot. A job is submitted through
/// the adapter and fails at the render-start reading (the stand-in prints no
/// document). **The first worker asks under the button's submission
/// identity**; it ends, the supervisor starts its next role worker, and that
/// one asks the same request again — the second press, the same identity.
///
/// **Before this piece** the request was refused by name and nothing was made.
#[test]
fn try_it_again_makes_one_job_through_the_real_program_and_a_second_press_finds_it() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_framing::requests as asked;
    let mut area = APackageArea::made();
    let workers = a_role_worker_that_asks_when_told(&area);
    let root = ARoot::recorded_in(&area);
    let (stand_in, _) = a_stand_in_for_blender(&root);
    let failed = root.a_job_the_render_worker_can_take(&stand_in);
    area.start_the_supervisor();
    submitted(&area, &failed);
    assert_eq!(
        the_note_once_it_has_ended(&area, &failed)
            .fields
            .text(asked::THE_ENDING_FIELD),
        Some(asked::IT_FAILED)
    );
    let machine_root =
        smoozy_job_files_machine::layout::MachineRoot::chosen(root.at.to_str().expect("text"))
            .expect("a root");
    let manifest_of = |job: &Identity| {
        std::fs::read(format!(
            "{}/{}",
            smoozy_job_files_machine::publication::the_job_record_directory(&machine_root, job),
            smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME
        ))
    };
    let the_failed_manifest = manifest_of(&failed).expect("the failed job's manifest");

    asked_through_the_adapter(
        &area,
        &Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            smoozy_runtime::acting::stores::THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST,
        ),
    );
    let submission = Identity::issue(IdentityKind::Operation).expect("randomness");
    std::fs::write(
        area.at.join("telegram-request"),
        write_request_frame(
            &Request::speaking(
                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                submission,
                THE_JOB_IS_SUBMITTED_AGAIN_REQUEST,
            )
            .carrying(
                smoozy_framing::framing::Fields::none().and(asked::THE_JOB_FIELD, failed.as_text()),
            ),
        ),
    )
    .expect("the press is written");

    let first_worker = a_new_role_worker(&workers, &[]);
    let first = what_the_worker_was_answered(&workers, &first_worker);
    assert_eq!(
        first.fields.text("what became of the submission"),
        Some("submitted now"),
        "{first:?} — {}",
        the_complaints_of(&area)
    );
    let job = Identity::read_back(
        IdentityKind::Job,
        first.fields.text("new job").expect("the new job"),
    )
    .expect("a job identity, issued by the supervisor");
    assert_ne!(job, failed);
    assert!(
        the_jobs_on_the_disk(&area).contains(&job),
        "its record is written"
    );
    let copied = smoozy_job_files_machine::manifest::SubmissionManifest::read_back(
        &manifest_of(&job).expect("the copy is published in the new job's record directory"),
    )
    .expect("it reads back");
    let original =
        smoozy_job_files_machine::manifest::SubmissionManifest::read_back(&the_failed_manifest)
            .expect("it reads");
    assert_eq!(
        copied,
        original
            .copied_under(&job, Some("Kitchen"))
            .expect("it fits"),
        "copied under the new identity"
    );
    assert_eq!(
        manifest_of(&failed).expect("still there"),
        the_failed_manifest,
        "the original is never touched"
    );
    let folder = copied.job_output_folder.the_path_it_names().expect("named");
    assert!(
        std::path::Path::new(folder).is_dir(),
        "the new folder is made, as a placement makes the one its manifest names"
    );
    assert_ne!(
        Some(folder),
        original.job_output_folder.the_path_it_names(),
        "a new folder"
    );
    assert!(
        std::path::Path::new(folder)
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with(
                &job.as_text()
                    [..smoozy_job_files_machine::names::THE_IDENTITY_CHARACTERS_IN_A_FOLDER_NAME]
            )),
        "named for the new identity: {folder}"
    );
    let written: Vec<String> = every_event(&area)
        .into_iter()
        .filter(|(kind, about, _)| {
            kind == asked::THE_JOB_RECORD_WRITTEN_EVENT && *about == job.as_text()
        })
        .map(|(_, _, says)| says)
        .collect();
    assert_eq!(
        written,
        vec![format!(
            "waiting: submitted again by this machine from {}",
            failed.as_text()
        )],
        "journalled once, saying the machine made it and from which job"
    );

    // **`Telegram` R4: the second press**, the same request under the same
    // identity, from the next role worker this run starts.
    let second_worker = a_new_role_worker(&workers, &[first_worker]);
    let second = what_the_worker_was_answered(&workers, &second_worker);
    assert_eq!(
        second.fields.text("what became of the submission"),
        Some("submitted before"),
        "{second:?}"
    );
    assert_eq!(
        second.fields.text("new job"),
        Some(job.as_text().as_str()),
        "the recorded one"
    );
    assert_eq!(
        second.fields.text("submitted at"),
        first.fields.text("submitted at"),
        "and when it was made"
    );
    let on_the_disk = the_jobs_on_the_disk(&area);
    assert_eq!(
        on_the_disk.len(),
        2,
        "a second press makes nothing: {on_the_disk:?}"
    );
    assert_eq!(
        every_event(&area)
            .iter()
            .filter(
                |(kind, _, says)| kind == asked::THE_JOB_RECORD_WRITTEN_EVENT
                    && says.contains("submitted again")
            )
            .count(),
        1,
        "and journals nothing"
    );
}

const WHAT_THE_TESTS_HERE_WAIT_ON: &str =
    "the real smoozy-server: its supervisor, its adapter and the workers it starts";

/// **No test in this file can wait for ever.** Added 2026-09-21, after hosted
/// runs `35595657437` and `35601023619` sat three and a half hours on two tests
/// that were waiting for an answer that could never come, and told nobody.
///
/// **A test's own bound and nothing of the plan's**: five minutes, where the
/// slowest test in the workspace waits a minute and a half on purpose and the
/// rest take seconds. It is started as a test's first line and ends when the
/// test does, pass or panic, because dropping it closes the channel it watches.
/// **If it is still standing at the bound, a wait inside the test never
/// ended**: it says which test and what it was waiting on, ends every program
/// this binary started, and ends the binary, so the run fails instead of
/// hanging.
struct TheTestsDeadline {
    _held: std::sync::mpsc::Sender<()>,
}

impl TheTestsDeadline {
    const THE_BOUND: std::time::Duration = std::time::Duration::from_secs(300);

    fn started() -> TheTestsDeadline {
        let (held, watched) = std::sync::mpsc::channel::<()>();
        let test = std::thread::current()
            .name()
            .unwrap_or("a test in this file")
            .to_string();
        std::thread::spawn(move || {
            if watched.recv_timeout(Self::THE_BOUND)
                == Err(std::sync::mpsc::RecvTimeoutError::Timeout)
            {
                // **Straight to the complaints stream, not through `eprintln!`**: the
                // test harness captures that in a test's threads and never prints what
                // it captured when the binary is ended from inside, which is what
                // happens next.
                let said = format!(
                    "DEADLINE MISSED: {test} was still waiting after {:?}, on {}. Every \
                     program this test binary started is ended now, and so is the binary, so \
                     that this fails rather than hangs.\n",
                    Self::THE_BOUND,
                    WHAT_THE_TESTS_HERE_WAIT_ON
                );
                let _ = std::io::Write::write_all(&mut std::io::stderr(), said.as_bytes());
                if cfg!(unix) {
                    let _ = std::process::Command::new("pkill")
                        .args(["-KILL", "-P", &std::process::id().to_string()])
                        .status();
                    // **And what those started, which is not this binary's
                    // child**: no drop runs past the exit below, so each area
                    // still standing is ended here (added 2026-09-29).
                    let standing = THE_AREAS_STANDING
                        .lock()
                        .map(|standing| standing.clone())
                        .unwrap_or_default();
                    for area in standing {
                        end_every_program_started_under(&area);
                    }
                }
                std::process::exit(101);
            }
        });
        TheTestsDeadline { _held: held }
    }
}

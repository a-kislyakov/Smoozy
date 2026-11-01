//! `Servers`' acting half driven against a real SSH server, on this machine,
//! with a key the real `ssh-keygen` wrote.
//!
//! **Why this exists.** The unit tests drive the sequence and the repair
//! against stand-ins, which is what makes every judgement testable at the desk.
//! What they cannot show is that the same code works when the connection is a
//! real one: that the value the comparison is handed is the machine's own host
//! key, that the not-Linux check reads a real `exec` reply, that the hardware
//! reading is composed out of what a real command channel answered, and that a
//! break the transport reports on its own starts the repair.
//!
//! **What is proved here, in the plans' own terms.**
//!
//! - **Q1's sequence, end to end** — steps 1 to 6, in order, over one real
//!   connection, with the hardware reading composed from the machine's own
//!   answers.
//! - **Q6's three paths, on a real host key.** Nothing remembered records the
//!   machine's own key byte for byte; a match says nothing; a difference stops
//!   the attempt.
//! - **`Server protocol and transfers` A5 and `Security and licence` T2a** —
//!   where the comparison stops the attempt, the machine counts **no**
//!   authentication attempt at all. That is the ordering held by the protocol
//!   rather than by a setting, observed from the far end.
//! - **`Server protocol and transfers` §1.1** — every command that crossed is
//!   one of the module's own constants, read off the machine's own record of
//!   what it was asked.
//! - **Q13 outcome 2 and S10 item 5** — a machine that names something else
//!   ends the sequence, and the connection it opened is torn down.
//! - **N1, N5 and N9** — a break the transport declares starts a repair with
//!   the user doing nothing, every attempt of it is a fresh connection, and the
//!   condition record is raised at the run's failure.
//! - **S10 item 1 and S11 against a connection that does not end by itself** —
//!   an ending sent from another task while the driving waits on a healthy
//!   connection, or on a repair's real pause, returns the driving promptly with
//!   *disconnected* published. *(Added 2026-09-18: the stand-in connection
//!   ends by itself, which is what hid a disconnect that did nothing.)*
//!
//! **Nothing here rents anything.** The server is russh's own, run on the
//! loopback address in this process, and the key is made in a temporary
//! directory and thrown away.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Security and licence` T2a.
//!
//! **`Server protocol and transfers` A5 is deliberately not listed above and is
//! written out with its document at each site**, because this plan defines an
//! A5 of its own and a bare one could mean either.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan. Every other document's marker is written with that
//! document — at the site, or in the file's own header list of markers that
//! belong elsewhere. **Where a marker is a clause of this crate's own plan
//! *and* of the document meant, the header list is not enough and the document
//! is written out at the site**, because a bare one there does not fail to
//! resolve: it resolves to a real and wrong clause, which nobody reading it
//! can tell from a right one.
//!
//! **The plan here is `Preparation/Architecture/Servers.md`, and the words above
//! are the words `src/lib.rs` carries.** They are repeated because a file under
//! `tests/` is its own crate and never sees that one, and two of the three
//! sites the step 4 gate's fourth run caught were in a file under `tests/`.
//! Keep the wording identical wherever it appears.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use russh::server::{Auth, Msg, Session};
use russh::{Channel, ChannelId};
use smoozy_server_protocol::transport::Credential;
use smoozy_servers::acting::{
    SupplyTheCredential, TheActingHalf, TheIdentityRead, TheInstallationIdentityRead,
    TheServerBeingConnectedTo, Wait, WhatServersPublishes,
};
use smoozy_servers::condition::ObservedValue;
use smoozy_servers::driving::{APeriod, EndThePeriod, HowTheDrivingEnded, TheDesktopsSide};
use smoozy_servers::hardware::HardwareReading;
use smoozy_servers::first_connection::{ChangedIdentityWarning, NotLinuxOutcome};
use smoozy_servers::hardware::{Cards, MachineMemory};
use smoozy_servers::identity::{Host, KeyReference, Port, Username};
use smoozy_servers::key_moments::{TheTwoAnswers, WhoStartedIt};
use smoozy_servers::period::{OpeningAct, PeriodEnding, WhoTookIt};
use smoozy_servers::repair::{NextAttemptDue, PermanentRefusal, WhatTheRecordSays};
use smoozy_servers::sequence::{self, AnAttempt, Attempt, ConnectionRole, WhereItStopped};
use smoozy_servers::store::{RememberedHostKey, ServerRow};
use smoozy_servers::wiring::{TheDesktopsClock, TheTransport};
use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};
use tokio::net::TcpListener;

// ---------------------------------------------------------------- the machine

#[derive(Default)]
struct WhatTheMachineSaw {
    authentication_attempts: AtomicUsize,
    commands: Mutex<Vec<String>>,
}

#[derive(Clone)]
struct Machine {
    saw: Arc<WhatTheMachineSaw>,
    operating_system: &'static str,
    /// **The machine goes away the moment the sequence has finished with it.**
    /// Set for the one test that needs a real break: the connection is closed
    /// by the far end — K2's first bullet — and the listener is dropped with
    /// it, so every repair attempt afterwards is made against an address that
    /// is genuinely not answering.
    goes_away_after_the_reading: Arc<Mutex<Option<tokio::sync::oneshot::Sender<()>>>>,
}

impl russh::server::Server for Machine {
    type Handler = Self;
    fn new_client(&mut self, _address: Option<SocketAddr>) -> Self {
        self.clone()
    }
}

impl russh::server::Handler for Machine {
    type Error = russh::Error;

    async fn auth_publickey(
        &mut self,
        _user: &str,
        _key: &russh::keys::PublicKey,
    ) -> Result<Auth, Self::Error> {
        self.saw
            .authentication_attempts
            .fetch_add(1, Ordering::SeqCst);
        Ok(Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        _channel: Channel<Msg>,
        reply: russh::server::ChannelOpenHandle,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        reply.accept().await;
        Ok(())
    }

    async fn exec_request(
        &mut self,
        channel: ChannelId,
        command: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let asked = String::from_utf8_lossy(command).into_owned();
        self.saw
            .commands
            .lock()
            .expect("nothing panics holding this")
            .push(asked.clone());

        // What a Linux machine with one card answers. Every string here is the
        // machine's, and every command it answers is one of the module's own
        // constants — which is what the last test in this file checks.
        let (output, status) = match asked.as_str() {
            "uname -s" => (format!("{}\n", self.operating_system), 0),
            "test -e /.dockerenv" => (String::new(), 1),
            "systemd-detect-virt --container" => (String::new(), 1),
            "lspci -d 10de:" => (
                "01:00.0 3D controller: NVIDIA Corporation AD102 [GeForce RTX 4090]\n".to_string(),
                0,
            ),
            "nvidia-smi --query-gpu=uuid,pci.bus_id,name,memory.total --format=csv,noheader" => (
                "GPU-3a1b, 00000000:01:00.0, NVIDIA GeForce RTX 4090, 24564 MiB\n".to_string(),
                0,
            ),
            "cat /sys/fs/cgroup/memory.max" => ("16051527680\n".to_string(), 0),
            "cat /sys/fs/cgroup/memory/memory.limit_in_bytes" => (String::new(), 1),
            "cat /proc/meminfo" => ("MemTotal:       65805484 kB\n".to_string(), 0),
            "date +%s" => ("1770000000\n".to_string(), 0),
            _ => (String::new(), 127),
        };

        session.channel_success(channel)?;
        if !output.is_empty() {
            session.data(channel, output.into_bytes())?;
        }
        session.exit_status_request(channel, status)?;
        session.eof(channel)?;
        session.close(channel)?;

        // The last reading of the sequence, so the machine has answered
        // everything it is going to.
        if asked == "date +%s" {
            if let Some(go_away) = self
                .goes_away_after_the_reading
                .lock()
                .expect("nothing panics holding this")
                .take()
            {
                session.disconnect(
                    russh::Disconnect::ByApplication,
                    "the machine is going away",
                    "en-GB",
                )?;
                let _ = go_away.send(());
            }
        }
        Ok(())
    }
}

struct RunningMachine {
    address: SocketAddr,
    host_public_key: Vec<u8>,
    saw: Arc<WhatTheMachineSaw>,
    /// **Taking the machine away for real.** Sending on this drops the
    /// listener *and* the session under it, so an address that answered a
    /// moment ago stops answering — which is what a machine going away looks
    /// like from Smoozy's side, and what makes the repair's attempts fail at
    /// once rather than at their deadline.
    go_away: tokio::sync::oneshot::Sender<()>,
}

async fn a_machine(scratch: &Scratch, operating_system: &'static str) -> RunningMachine {
    a_machine_that(scratch, operating_system, false).await
}

/// A machine that closes the connection and stops listening the moment the
/// sequence has finished reading it.
async fn a_machine_that_goes_away(scratch: &Scratch) -> RunningMachine {
    a_machine_that(scratch, "Linux", true).await
}

async fn a_machine_that(
    scratch: &Scratch,
    operating_system: &'static str,
    goes_away_after_the_reading: bool,
) -> RunningMachine {
    let host_key_file = scratch.a_key("host_key");
    let host_key = russh::keys::load_secret_key(&host_key_file, None)
        .expect("ssh-keygen wrote this file a moment ago");
    let host_public_key = host_key
        .public_key()
        .to_bytes()
        .expect("an ed25519 public key encodes");

    let config = Arc::new(russh::server::Config {
        keys: vec![host_key],
        auth_rejection_time: Duration::from_millis(0),
        auth_rejection_time_initial: Some(Duration::from_millis(0)),
        ..russh::server::Config::default()
    });

    let socket = TcpListener::bind(("127.0.0.1", 0))
        .await
        .expect("the loopback address takes a listener");
    let address = socket.local_addr().expect("a bound listener has an address");
    let saw = Arc::new(WhatTheMachineSaw::default());
    let (go_away, mut told_to_go) = tokio::sync::oneshot::channel::<()>();
    let (told_by_the_machine, mut the_machine_left) = tokio::sync::oneshot::channel::<()>();
    let machine = Machine {
        saw: Arc::clone(&saw),
        operating_system,
        goes_away_after_the_reading: Arc::new(Mutex::new(
            goes_away_after_the_reading.then_some(told_by_the_machine),
        )),
    };
    tokio::spawn(async move {
        // The accept loop is this test's own rather than the library's, so the
        // listener and the session are both droppable — which is the whole
        // point: a machine that has gone away has to stop answering.
        let mut session_task: Option<tokio::task::JoinHandle<()>> = None;
        loop {
            tokio::select! {
                _ = &mut told_to_go => break,
                // **Only where the machine is one that goes away.** Without
                // the guard the unused sender is dropped at once, the receiver
                // resolves, and every machine stops listening the moment it
                // starts.
                _ = &mut the_machine_left, if goes_away_after_the_reading => break,
                accepted = socket.accept() => {
                    let Ok((stream, _)) = accepted else { break };
                    let config = Arc::clone(&config);
                    let handler = machine.clone();
                    session_task = Some(tokio::spawn(async move {
                        if let Ok(session) =
                            russh::server::run_stream(config, stream, handler).await
                        {
                            let _ = session.await;
                        }
                    }));
                }
            }
        }
        if let Some(task) = session_task {
            task.abort();
        }
        drop(socket);
    });

    RunningMachine {
        address,
        host_public_key,
        saw,
        go_away,
    }
}

// ------------------------------------------------------------------- the keys

struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!(
            "smoozy-servers-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a temporary directory can be made");
        Scratch(path)
    }

    fn a_key(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        let made = std::process::Command::new("ssh-keygen")
            .arg("-t")
            .arg("ed25519")
            .arg("-N")
            .arg("")
            .arg("-C")
            .arg("smoozy servers test")
            .arg("-f")
            .arg(&path)
            .output()
            .expect("ssh-keygen is on the path of all three systems");
        assert!(
            made.status.success(),
            "ssh-keygen: {}",
            String::from_utf8_lossy(&made.stderr)
        );
        path
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

// ------------------------------------------------------------- the other ends

/// `Keys`' supply, answering for a key with no passphrase.
struct AKeyThatWorks {
    file: PathBuf,
}

impl SupplyTheCredential for AKeyThatWorks {
    fn the_two_answers(&mut self, _key: &KeyReference) -> TheTwoAnswers {
        TheTwoAnswers {
            the_key_is_usable: true,
            a_passphrase_is_available_without_asking: true,
        }
    }

    fn for_one_attempt(
        &mut self,
        _key: &KeyReference,
        _who: WhoStartedIt,
        _naming: TheServerBeingConnectedTo<'_>,
    ) -> Option<Credential> {
        Some(Credential {
            private_key_file: self.file.clone(),
            passphrase: None,
        })
    }

    fn the_refusal_is_permanent(&mut self, _key: &KeyReference) -> PermanentRefusal {
        PermanentRefusal {
            the_store_standing_reads_never_answered: false,
            the_rows_key_is_protected: false,
        }
    }
}

/// Step 5's read, which belongs to modules built at step 6.
struct TheIdentityIsAbsent;

impl<C: Send> TheInstallationIdentityRead<C> for TheIdentityIsAbsent {
    async fn read(&mut self, _channel: &mut C) -> TheIdentityRead {
        TheIdentityRead {
            check: InstallationCheck::Absent,
            identity_on_the_machine: None,
        }
    }
}

#[derive(Default)]
struct Caught {
    observed_values: Vec<ObservedValue>,
    next_attempt_due: Vec<NextAttemptDue>,
    conditions_raised: Vec<WhatTheRecordSays>,
    conditions_withdrawn: usize,
    changed_identity_warnings: usize,
    host_keys_recorded: Vec<Vec<u8>>,
}

impl WhatServersPublishes for Caught {
    fn the_observed_value(&mut self, _server: &str, value: ObservedValue) {
        self.observed_values.push(value);
    }

    fn the_next_attempt_due(&mut self, _server: &str, reading: NextAttemptDue) {
        self.next_attempt_due.push(reading);
    }

    fn raise_the_connection_condition(&mut self, _server: &str, says: WhatTheRecordSays) {
        self.conditions_raised.push(says);
    }

    fn withdraw_the_connection_condition(&mut self, _server: &str) {
        self.conditions_withdrawn += 1;
    }

    fn the_changed_identity_warning(&mut self, _server: &str) {
        self.changed_identity_warnings += 1;
    }

    fn record_the_scanned_host_key(&mut self, _server: &str, key: RememberedHostKey) {
        self.host_keys_recorded
            .push(key.as_the_local_record_writes_it().to_vec());
    }
}

/// The repair's pauses, taken on a clock rather than on the wall.
///
/// **The figures are `Servers`' own and are not changed here.** What this
/// stands in for is the waiting, so a run of eight attempts against a machine
/// that is not there takes a moment rather than 105 seconds — the mechanism is
/// what is being observed, and the figures are proved at the desk.
struct WithoutWaiting {
    clock: SystemTime,
    waits: usize,
    stop_after: Option<(usize, EndThePeriod)>,
}

impl Wait for WithoutWaiting {
    async fn until(&mut self, how_long: Duration) {
        self.clock += how_long;
        self.waits += 1;
        if let Some((after, end)) = &self.stop_after {
            if self.waits >= *after {
                end.at(PeriodEnding::TheUserDisconnects);
            }
        }
    }

    fn now(&mut self) -> SystemTime {
        self.clock
    }
}

/// **The desktop's side of a period, as these tests hold it.** The record
/// answers nothing, so each attempt is made on the row the act was handed; what
/// each live connection read is kept so a test could look.
#[derive(Default)]
struct TheRecordAnswersNothing {
    handed_back: Vec<(InstallationCheck, Option<HardwareReading>)>,
}

impl TheDesktopsSide for TheRecordAnswersNothing {
    fn the_row_now(&mut self, _server: &str) -> Option<ServerRow> {
        None
    }

    fn the_connection_read(
        &mut self,
        _server: &str,
        check: InstallationCheck,
        hardware: Option<HardwareReading>,
    ) {
        self.handed_back.push((check, hardware));
    }
}

fn a_row(machine: &RunningMachine) -> ServerRow {
    ServerRow::saved(
        Identity::issue(IdentityKind::LocalServer)
            .expect("randomness")
            .as_text(),
        Host::what_the_user_typed(&machine.address.ip().to_string()).expect("a host"),
        Port::what_the_user_typed(machine.address.port()).expect("a port"),
        Username::what_the_user_typed("smoozy").expect("a username"),
        KeyReference {
            local_key_identifier: "key-1".into(),
            fingerprint: "SHA256:aaa".into(),
        },
    )
}

type Half = TheActingHalf<TheTransport, AKeyThatWorks, TheIdentityIsAbsent, Caught, WithoutWaiting>;

fn a_half(key: PathBuf) -> Half {
    TheActingHalf {
        opener: TheTransport,
        credentials: AKeyThatWorks { file: key },
        installation: TheIdentityIsAbsent,
        publishes: Caught::default(),
        wait: WithoutWaiting {
            clock: SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000),
            waits: 0,
            stop_after: None,
        },
    }
}

fn an_attempt(row: &ServerRow) -> AnAttempt<'_> {
    AnAttempt {
        row,
        who: WhoStartedIt::TheUser,
        the_first_connection_to_this_server: true,
        role: ConnectionRole::Primary,
        // An attempt that belongs to no run carries N2's own connect deadline
        // and nothing shorter.
        deadline: smoozy_servers::repair::CONNECT_DEADLINE,
    }
}

/// The callback Q1 step 3 calls the instant the transport reports the
/// connection authenticated and usable. **K6's *connected* is the period's to
/// compose**, and these tests drive the sequence rather than a period.
fn nothing_to_publish(_publishes: &mut Caught) {}

// ---------------------------------------------------------------- the tests

#[tokio::test]
async fn the_whole_sequence_runs_over_one_real_connection() {
    // Q1 steps 1 to 6, in order, against a machine that answers.
    let scratch = Scratch::new("the-whole-sequence");
    let machine = a_machine(&scratch, "Linux").await;
    let row = a_row(&machine);
    let mut half = a_half(scratch.a_key("id_ed25519"));

    let ended = sequence::run(
        an_attempt(&row),
        &mut half,
        &mut ChangedIdentityWarning::none_yet(),
        &mut nothing_to_publish,
    )
    .await;
    let Attempt::ItIsLive(gave) = ended else {
        panic!("the machine answered everything the sequence asked it")
    };

    // **Q6's first path, on the machine's own key, byte for byte.**
    assert_eq!(
        half.publishes.host_keys_recorded,
        vec![machine.host_public_key.clone()]
    );
    assert_eq!(half.publishes.changed_identity_warnings, 0);

    // **Step 5's branch and H18: the absent branch takes a fresh reading.**
    assert_eq!(gave.installation, Some(InstallationCheck::Absent));
    let reading = gave.hardware.expect("a fresh reading");
    let Cards::TheCards(cards) = &reading.cards else {
        panic!("the machine's own card list")
    };
    // **H5: the count is the number of rows** the driver's own query returned.
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].unique_identifier, "GPU-3a1b");
    assert_eq!(cards[0].total_memory_bytes, Some(24_564 * 1024 * 1024));
    // **H9a: the newer pair answered, so that figure is the allocation.**
    assert_eq!(
        reading.machine_memory,
        MachineMemory::TheAllocation(16_051_527_680)
    );
    // **H10's figure is absent and nothing is substituted for it.**
    assert!(reading.free_space.is_none());
    // **H2: the moment is the machine's own**, read from the machine.
    assert_eq!(
        reading.the_machines_own_moment.taken_at(),
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_770_000_000)
    );
    // **H17: the explicit absence of an installation identity.**
    assert_eq!(reading.read_under, None);
    // **`State, identity and recovery` D5: the two moments are never
    // subtracted from each other.** That document's D5 and not `Servers`',
    // which is about history records being keyed by the job identity.
    assert_eq!(reading.how_long_between_the_two_moments(), None);

    // **§1.1: every command that crossed is one of the module's own
    // constants**, read off what the machine says it was asked.
    let asked = machine.saw.commands.lock().expect("no panic").clone();
    let constants: Vec<&str> = smoozy_servers::readings::EVERY_COMMAND_THIS_MODULE_SENDS
        .iter()
        .map(|command| command.as_str())
        .collect();
    for command in &asked {
        assert!(constants.contains(&command.as_str()), "{command}");
    }
    assert!(asked.contains(&"uname -s".to_string()));

    drop(gave.connection);
    let _ = machine.go_away.send(());
}

#[tokio::test]
async fn a_machine_whose_identity_differs_is_never_shown_a_credential() {
    // Q6's third path, `Server protocol and transfers` A5 and `Security and
    // licence` T2a — observed from the far end, which counts authentication
    // attempts and sees none.
    let scratch = Scratch::new("identity-differs");
    let machine = a_machine(&scratch, "Linux").await;
    let mut row = a_row(&machine);
    row.remembered_host_key = Some(RememberedHostKey::approved(
        b"a key this machine does not have".to_vec(),
    ));
    let mut half = a_half(scratch.a_key("id_ed25519"));

    let ended = sequence::run(
        an_attempt(&row),
        &mut half,
        &mut ChangedIdentityWarning::none_yet(),
        &mut nothing_to_publish,
    )
    .await;

    assert!(matches!(
        ended,
        Attempt::ItStopped(WhereItStopped::TheIdentityDiffers)
    ));
    assert_eq!(
        machine
            .saw
            .authentication_attempts
            .load(Ordering::SeqCst),
        0,
        "no credential reaches a machine the comparison refused"
    );
    assert_eq!(half.publishes.changed_identity_warnings, 1);
    // **Q6: declining keeps the old record.** Nothing is recorded here.
    assert!(half.publishes.host_keys_recorded.is_empty());

    let _ = machine.go_away.send(());
}

#[tokio::test]
async fn a_remembered_identity_that_matches_says_nothing_and_records_nothing() {
    let scratch = Scratch::new("identity-matches");
    let machine = a_machine(&scratch, "Linux").await;
    let mut row = a_row(&machine);
    row.remembered_host_key = Some(RememberedHostKey::approved(
        machine.host_public_key.clone(),
    ));
    let mut half = a_half(scratch.a_key("id_ed25519"));

    let ended = sequence::run(
        an_attempt(&row),
        &mut half,
        &mut ChangedIdentityWarning::none_yet(),
        &mut nothing_to_publish,
    )
    .await;

    assert!(matches!(ended, Attempt::ItIsLive(_)));
    assert!(half.publishes.host_keys_recorded.is_empty());
    assert_eq!(half.publishes.changed_identity_warnings, 0);

    let _ = machine.go_away.send(());
}

#[tokio::test]
async fn a_machine_that_is_not_linux_ends_the_sequence_at_step_four() {
    // Q13 outcome 2, over a real `exec` reply, and S10 item 5 with it.
    let scratch = Scratch::new("not-linux");
    let machine = a_machine(&scratch, "Darwin").await;
    let row = a_row(&machine);
    let mut half = a_half(scratch.a_key("id_ed25519"));

    let ended = sequence::run(
        an_attempt(&row),
        &mut half,
        &mut ChangedIdentityWarning::none_yet(),
        &mut nothing_to_publish,
    )
    .await;

    let Attempt::ItStopped(WhereItStopped::TheNotLinuxCheck(outcome)) = ended else {
        panic!("the machine named something else")
    };
    assert!(outcome.claims_what_the_machine_is());
    // **Q11: ahead of everything else that acts on the machine.** Nothing after
    // step 4 was asked.
    let asked = machine.saw.commands.lock().expect("no panic").clone();
    assert_eq!(asked, vec!["uname -s".to_string()]);

    let _ = machine.go_away.send(());
}

#[tokio::test]
async fn a_period_driven_against_a_machine_that_is_not_linux_answers_with_which_outcome_it_was() {
    // **Q13 outcome 2, over a real connection, as far as a caller of this crate
    // can see.** The test above proves the sequence knows which outcome it was,
    // and that is worth nothing to the user if the answer the driving gives
    // back has dropped it: outcome 2's promise — *says plainly that it is not a
    // Linux machine* — is kept by whoever drives the period, and outcome 3
    // beside it is the one that claims nothing.
    let scratch = Scratch::new("not-linux-period");
    let machine = a_machine(&scratch, "Darwin").await;
    let row = a_row(&machine);
    let mut half = a_half(scratch.a_key("id_ed25519"));

    let mut period = APeriod::begun(
        &row.local_server_identifier,
        OpeningAct::TheUsersConnectAction,
        WhoTookIt::TheUser,
        SystemTime::UNIX_EPOCH,
    );
    let ended = period.drive(&row, &mut half, &mut TheRecordAnswersNothing::default()).await;

    assert_eq!(
        ended,
        HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheNotLinuxCheck(
            NotLinuxOutcome::ItNamedSomethingElse
        ))
    );
    // S10 item 5 all the same: the period ended and the server reads
    // disconnected.
    assert_eq!(
        half.publishes.observed_values.last(),
        Some(&ObservedValue::Disconnected)
    );

    let _ = machine.go_away.send(());
}

#[tokio::test]
async fn a_break_the_transport_declares_starts_a_repair_with_the_user_doing_nothing() {
    // N1, N5 and N9, over a real connection: the machine goes away, the
    // transport reports it on its own, and the repair makes fresh attempts
    // against an address that no longer answers.
    let scratch = Scratch::new("a-real-break");
    let machine = a_machine_that_goes_away(&scratch).await;
    let row = a_row(&machine);
    let mut half = a_half(scratch.a_key("id_ed25519"));

    let mut period = APeriod::begun(
        &row.local_server_identifier,
        OpeningAct::TheUsersConnectAction,
        WhoTookIt::TheUser,
        SystemTime::UNIX_EPOCH,
    );
    // The user disconnects once a run has failed, which is what stops a
    // standing repair that would otherwise run for as long as the app is open.
    half.wait.stop_after = Some((9, period.end_it()));

    // The machine goes away as soon as the period is open, so the break arrives
    // from the transport rather than from anything Smoozy asked.
    let ended = period.drive(&row, &mut half, &mut TheRecordAnswersNothing::default()).await;

    assert_eq!(
        ended,
        HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheUserDisconnects)
    );
    // The opening connection was live, so *connected* was published once.
    assert!(half
        .publishes
        .observed_values
        .contains(&ObservedValue::Connected));
    // **N5: raised at the run's failure**, and it says only that the connection
    // is down.
    assert_eq!(half.publishes.conditions_raised.len(), 1);
    assert!(!half.publishes.conditions_raised[0].any_claim_about_why);
    // **N10a: absent while an attempt is in flight, and a moment after one
    // ends.**
    assert!(half
        .publishes
        .next_attempt_due
        .iter()
        .any(|reading| matches!(reading, NextAttemptDue::At(_))));
    assert_eq!(
        half.publishes.observed_values.last(),
        Some(&ObservedValue::Disconnected)
    );
}

// ------------------------------------------ an ending from outside the driving

/// **The repair's pauses, waited on the desktop's own clock**, so a pause is a
/// real fifteen seconds and a disconnect taken inside one is a real wait being
/// woken.
type ARealHalf =
    TheActingHalf<TheTransport, AKeyThatWorks, TheIdentityIsAbsent, Caught, TheDesktopsClock>;

fn a_real_half(key: PathBuf) -> ARealHalf {
    TheActingHalf {
        opener: TheTransport,
        credentials: AKeyThatWorks { file: key },
        installation: TheIdentityIsAbsent,
        publishes: Caught::default(),
        wait: TheDesktopsClock,
    }
}

/// An ending sent from another task after `after`, and the moment it was sent.
fn an_ending_sent_after(
    end: EndThePeriod,
    after: Duration,
) -> tokio::task::JoinHandle<std::time::Instant> {
    tokio::spawn(async move {
        tokio::time::sleep(after).await;
        end.at(PeriodEnding::TheUserDisconnects);
        std::time::Instant::now()
    })
}

#[tokio::test]
async fn a_disconnect_ends_a_healthy_connection_at_once() {
    // **S10 item 1 and S11, over a real connection that does not end by
    // itself.** The driving is waiting on the live connection when the ending
    // arrives three seconds in; it must return promptly, with the period ended
    // and *disconnected* published. Before 2026-09-18 the ending set a flag and
    // nothing woke the wait, so the row stayed *Connected*.
    let scratch = Scratch::new("a-disconnect-ends-it");
    let machine = a_machine(&scratch, "Linux").await;
    let row = a_row(&machine);
    let mut half = a_real_half(scratch.a_key("id_ed25519"));

    let mut period = APeriod::begun(
        &row.local_server_identifier,
        OpeningAct::TheUsersConnectAction,
        WhoTookIt::TheUser,
        SystemTime::now(),
    );
    let sent = an_ending_sent_after(period.end_it(), Duration::from_secs(3));
    let ended = tokio::time::timeout(
        Duration::from_secs(20),
        period.drive(&row, &mut half, &mut TheRecordAnswersNothing::default()),
    )
    .await
    .expect("the driving returned inside twenty seconds");
    let returned = std::time::Instant::now();
    let sent = sent.await.expect("the ending was sent");

    assert_eq!(
        ended,
        HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheUserDisconnects)
    );
    assert!(
        returned.duration_since(sent) < Duration::from_secs(2),
        "**the driving returned promptly after the ending**, not when the \
         connection ended by itself: {:?} after it",
        returned.duration_since(sent)
    );
    assert!(half
        .publishes
        .observed_values
        .contains(&ObservedValue::Connected));
    assert_eq!(
        half.publishes.observed_values.last(),
        Some(&ObservedValue::Disconnected)
    );

    let _ = machine.go_away.send(());
}

#[tokio::test]
async fn a_disconnect_during_a_repairs_pause_ends_it_at_once() {
    // **The same ending inside a repair's real pause**, which used to be waited
    // out — up to sixty seconds with nothing said. The machine goes away as
    // soon as the sequence has read it, the repair's attempts are refused at
    // once by an address that no longer answers, and the driving is inside a
    // fifteen-second pause on the desktop's own clock when the ending arrives.
    let scratch = Scratch::new("a-disconnect-in-a-pause");
    let machine = a_machine_that_goes_away(&scratch).await;
    let row = a_row(&machine);
    let mut half = a_real_half(scratch.a_key("id_ed25519"));

    let mut period = APeriod::begun(
        &row.local_server_identifier,
        OpeningAct::TheUsersConnectAction,
        WhoTookIt::TheUser,
        SystemTime::now(),
    );
    let sent = an_ending_sent_after(period.end_it(), Duration::from_secs(3));
    let ended = tokio::time::timeout(
        Duration::from_secs(20),
        period.drive(&row, &mut half, &mut TheRecordAnswersNothing::default()),
    )
    .await
    .expect("the driving returned inside twenty seconds");
    let returned = std::time::Instant::now();
    let sent = sent.await.expect("the ending was sent");

    assert_eq!(
        ended,
        HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheUserDisconnects)
    );
    assert!(
        returned.duration_since(sent) < Duration::from_secs(2),
        "**the pause was woken**, not waited out: {:?} after the ending",
        returned.duration_since(sent)
    );
    // It was a repair that was ended: the moment of the next attempt was
    // published at least once before the ending arrived.
    assert!(half
        .publishes
        .next_attempt_due
        .iter()
        .any(|reading| matches!(reading, NextAttemptDue::At(_))));
    assert_eq!(
        half.publishes.observed_values.last(),
        Some(&ObservedValue::Disconnected)
    );
}

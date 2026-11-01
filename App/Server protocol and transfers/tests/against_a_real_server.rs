//! The transport driven against a real SSH server, on this machine, with keys
//! the real `ssh-keygen` wrote.
//!
//! **Why this exists and what it is worth.** `Servers` K1a puts the whole
//! connection inside Smoozy's own process, so every clause about ordering,
//! about the keepalive and about what a break is, is a claim about code in
//! this crate rather than about a program's settings. A claim like that is
//! either observed or it is a sentence. These tests observe them.
//!
//! **What is proved here, in the plans' own terms.**
//!
//! - **`Server protocol and transfers` A5 and `Security and licence` T2a** —
//!   the offered host key is comparable **before** any connection that
//!   presents Smoozy's credential. Proved by the server counting
//!   authentication attempts and seeing **none** when the comparison stops the
//!   attempt.
//! - **`Servers` Q6's three paths** — the value the comparison is handed is
//!   the machine's own host key, byte for byte.
//! - **`Security and licence` §4 R2a** — the transport reads the key file at
//!   the path it was given and unlocks it in memory for one authentication
//!   exchange, on a container OpenSSH itself wrote.
//! - **`Servers` K2's second bullet and K3** — a caller whose line stops
//!   answering learns of it, at the figures it was given, without having
//!   anything to do on the connection. **That is the case that went undetected
//!   for 21 minutes 45 seconds with no keepalive**, and it is the reason
//!   `State, identity and recovery` §8.5 exists.
//! - **`Servers` K2's first bullet** — an announced ending is told at once and
//!   is not the keepalive one.
//! - **`Servers` S10 and K5** — a tear-down Smoozy started is not a break. It
//!   is one of the six moments that document's S10 ends a period at. **That
//!   document's S10 and not this one's**, which is about a transfer's
//!   accounting.
//! - **§1.1** — one `exec` request carries one command string and returns
//!   output, complaint output and exit status.
//! - **§1.1's second carriage** — *"The constant command's standard input,
//!   read as data."* The value arrives on the command's own standard input,
//!   the command string that arrives carries no part of it, and the stream is
//!   ended so a program that reads until it ends is not left waiting.
//!
//! **Nothing here rents anything.** The server is russh's own, run on the
//! loopback address in this process, and the keys are made in a temporary
//! directory and thrown away.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Servers` K1a, K2, K3, K5, Q6.
//! `Security and licence` R2a, T2a.
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
//! **The plan here is
//! `Preparation/Architecture/Server protocol and transfers.md`, and the words
//! above are the words `src/lib.rs` carries.** They are repeated because a
//! file under `tests/` is its own crate and never sees that one, and two of
//! the three
//! sites the step 4 gate's fourth run caught were in a file under `tests/`.
//! Keep the wording identical wherever it appears.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use russh::server::{Auth, Msg, Server as _, Session};
use russh::{Channel, ChannelId};
use smoozy_security_and_licence::plaintext::Plaintext;
use smoozy_server_protocol::transport::ssh::open;
use smoozy_server_protocol::transport::{
    CommandString, Credential, DecideHostTrust, Ending, Endpoint, Figures, HostTrust, OpenFailure,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// ---------------------------------------------------------------- the machine

/// What the far end saw, so a test can assert about the order of things rather
/// than about this crate's own account of them.
#[derive(Default)]
struct WhatTheMachineSaw {
    authentication_attempts: AtomicUsize,
    commands: Mutex<Vec<String>>,
    /// Every byte that arrived on a command's standard input, in order.
    standard_input: Mutex<Vec<u8>>,
    /// Whether the caller ended that stream. **A program reading a password
    /// reads until the stream ends**, so a route that never ends it is a route
    /// that hangs, and this is what lets a test say so.
    the_input_stream_was_ended: AtomicBool,
    /// **Every channel [`RUNS_UNTIL_ITS_CHANNEL_IS_CLOSED`] ran on, and every
    /// channel the caller closed.** Added 2026-09-27, step 10c's eighth-pass
    /// fix piece 1.
    held_open: Mutex<Vec<ChannelId>>,
    closed_by_the_caller: Mutex<Vec<ChannelId>>,
}

/// The one command this machine holds open for its input instead of answering
/// at once. **Nothing about the value is in it**, which is the point of §1.1's
/// second carriage.
const READS_ITS_INPUT: &str = "sudo -S -v";

/// **The one command this machine never answers**: it runs until its channel is
/// closed, the way a long admin step runs for minutes. Added 2026-09-27.
const RUNS_UNTIL_ITS_CHANNEL_IS_CLOSED: &str = "a command that runs until its channel is closed";

/// **A command that prints a line on each stream and then runs on**, the way
/// a long admin step prints while it works. Added 2026-10-18, the Creator's
/// thirteenth notes, piece H.
const PRINTS_AND_RUNS_ON: &str = "a command that prints and runs on";

#[derive(Clone)]
struct Machine {
    saw: Arc<WhatTheMachineSaw>,
    /// The channel [`READS_ITS_INPUT`] is running on, where one is.
    reading_its_input_on: Option<ChannelId>,
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
        self.saw
            .commands
            .lock()
            .expect("nothing panics holding this")
            .push(String::from_utf8_lossy(command).into_owned());
        session.channel_success(channel)?;
        if command == RUNS_UNTIL_ITS_CHANNEL_IS_CLOSED.as_bytes() {
            self.saw
                .held_open
                .lock()
                .expect("nothing panics holding this")
                .push(channel);
            return Ok(());
        }
        if command == PRINTS_AND_RUNS_ON.as_bytes() {
            session.data(channel, &b"Unpacking nvidia-driver-595 ...\n"[..])?;
            session.extended_data(channel, 1, &b"debconf: delaying\n"[..])?;
            return Ok(());
        }
        if command == READS_ITS_INPUT.as_bytes() {
            // **It answers nothing yet.** The whole point of the route being
            // tested is that the value arrives after the request, on the
            // command's own standard input, and a machine that answered here
            // would prove nothing about it.
            self.reading_its_input_on = Some(channel);
            return Ok(());
        }
        // The three things §1.1 says one `exec` returns.
        session.data(channel, &b"Linux\n"[..])?;
        session.extended_data(channel, 1, &b"a complaint\n"[..])?;
        session.exit_status_request(channel, 7)?;
        session.eof(channel)?;
        session.close(channel)?;
        Ok(())
    }

    async fn data(
        &mut self,
        _channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.saw
            .standard_input
            .lock()
            .expect("nothing panics holding this")
            .extend_from_slice(data);
        Ok(())
    }

    async fn channel_close(
        &mut self,
        channel: ChannelId,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.saw
            .closed_by_the_caller
            .lock()
            .expect("nothing panics holding this")
            .push(channel);
        Ok(())
    }

    async fn channel_eof(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.saw
            .the_input_stream_was_ended
            .store(true, Ordering::SeqCst);
        if self.reading_its_input_on == Some(channel) {
            // **It answers with what it was given**, so the test reads the
            // value back off the far end rather than off this crate's own
            // account of what it sent.
            let given = self
                .saw
                .standard_input
                .lock()
                .expect("nothing panics holding this")
                .clone();
            session.data(channel, given)?;
            session.exit_status_request(channel, 0)?;
            session.eof(channel)?;
            session.close(channel)?;
        }
        Ok(())
    }
}

/// A machine listening on the loopback address, with its own host key.
struct RunningMachine {
    address: SocketAddr,
    host_public_key: Vec<u8>,
    saw: Arc<WhatTheMachineSaw>,
    shut_down: russh::server::RunningServerHandle,
}

async fn a_machine(host_key_file: &Path) -> RunningMachine {
    let host_key = russh::keys::load_secret_key(host_key_file, None)
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

    // The listener and the server both have to outlive the task that runs
    // them, and a test process is the one place that is free.
    let socket: &'static TcpListener = Box::leak(Box::new(
        TcpListener::bind(("127.0.0.1", 0))
            .await
            .expect("the loopback address takes a listener"),
    ));
    let address = socket.local_addr().expect("a bound listener has an address");
    let saw = Arc::new(WhatTheMachineSaw::default());
    let machine: &'static mut Machine = Box::leak(Box::new(Machine {
        saw: Arc::clone(&saw),
        reading_its_input_on: None,
    }));

    let running = machine.run_on_socket(config, socket);
    let shut_down = running.handle();
    tokio::spawn(running);

    RunningMachine {
        address,
        host_public_key,
        saw,
        shut_down,
    }
}

// ------------------------------------------------------------------- the keys

/// A temporary directory that takes itself away.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!(
            "smoozy-transport-{name}-{}-{}",
            std::process::id(),
            Instant::now().elapsed().as_nanos()
        ));
        std::fs::create_dir_all(&path).expect("a temporary directory can be made");
        Scratch(path)
    }

    /// Makes a key with the real `ssh-keygen`, which is what `Keys` does and
    /// what makes a container this transport reads a container OpenSSH wrote.
    fn a_key(&self, name: &str, passphrase: &str) -> PathBuf {
        let path = self.0.join(name);
        let made = std::process::Command::new("ssh-keygen")
            .arg("-t")
            .arg("ed25519")
            .arg("-N")
            .arg(passphrase)
            .arg("-C")
            .arg("smoozy transport test")
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

// --------------------------------------------------------------- the decision

/// A comparison that answers as it is told and keeps what it was shown, so a
/// test can assert the value reaching it is the machine's own key.
struct Compares {
    answer: HostTrust,
    offered: Arc<Mutex<Option<Vec<u8>>>>,
}

impl DecideHostTrust for Compares {
    fn decide(&mut self, offered: &[u8]) -> HostTrust {
        *self
            .offered
            .lock()
            .expect("nothing panics holding this") = Some(offered.to_vec());
        self.answer
    }
}

const AT_THE_DESK: Figures = Figures {
    // Not K3's figures. K3's own are 15 seconds and 3, which would make this
    // file take a minute to say what it says in under a second; what is being
    // proved is the mechanism, and the figures are `Servers`'.
    interval: Duration::from_millis(120),
    unanswered_before_the_break: 2,
};

fn endpoint(address: SocketAddr) -> Endpoint {
    Endpoint {
        host: address.ip().to_string(),
        port: address.port(),
        username: "whoever".to_string(),
    }
}

fn credential(path: &Path, passphrase: Option<&str>) -> Credential {
    Credential {
        private_key_file: path.to_path_buf(),
        passphrase: passphrase.map(Plaintext::new),
    }
}

// ------------------------------------------------------------------ the tests

/// **A5 and T2a, observed rather than argued.** The comparison refuses, and the
/// machine records **no authentication attempt at all** — so Smoozy's
/// credential was never presented to a machine the comparison had not
/// approved. That is what the struck throwaway known-hosts file used to be for
/// (`Security and licence` T2), and it holds here by the ordering the protocol
/// itself fixes.
#[tokio::test]
async fn a_refused_host_key_stops_before_any_credential_is_presented() {
    let scratch = Scratch::new("refused");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "");
    let machine = a_machine(&host_key).await;

    let failure = open(
        &endpoint(machine.address),
        credential(&client_key, None),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Stop,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect_err("a refused host key stops the attempt");

    assert!(matches!(failure, OpenFailure::HostTrustStoppedIt));
    assert_eq!(
        machine.saw.authentication_attempts.load(Ordering::SeqCst),
        0,
        "no credential may be presented to a machine the comparison refused"
    );
    machine.shut_down.shutdown("done".into());
}

/// **Q6's three paths compare a value, and this is the value.** What reaches
/// the comparison is the machine's own host key, byte for byte — not a
/// fingerprint, not a rendering, and not a diagnostic string (`Security and
/// licence` T4, and Fact 37's reason for it).
#[tokio::test]
async fn what_the_comparison_is_handed_is_the_machines_own_host_key() {
    let scratch = Scratch::new("offered");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "");
    let machine = a_machine(&host_key).await;

    let offered = Arc::new(Mutex::new(None));
    let connection = open(
        &endpoint(machine.address),
        credential(&client_key, None),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::clone(&offered),
        }),
    )
    .await
    .expect("an approved host key and an accepted credential");

    let seen = offered
        .lock()
        .expect("nothing panics holding this")
        .clone()
        .expect("the comparison was asked");
    assert_eq!(seen, machine.host_public_key);

    let _ = connection.tear_down().await;
    machine.shut_down.shutdown("done".into());
}

/// **`Security and licence` §4 R2a**: the transport reads the file at the path
/// it was given and unlocks it in memory for one authentication exchange. The
/// container here is one `ssh-keygen` wrote with a passphrase, so this is the
/// real encrypted form rather than a shape the library made for itself.
#[tokio::test]
async fn a_key_with_a_passphrase_opens_the_connection() {
    let scratch = Scratch::new("locked");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "the passphrase");
    let machine = a_machine(&host_key).await;

    let connection = open(
        &endpoint(machine.address),
        credential(&client_key, Some("the passphrase")),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect("a protected key opens with the value it is protected by");

    assert_eq!(
        machine.saw.authentication_attempts.load(Ordering::SeqCst),
        1,
        "one attempt, and A4 forbids a second on the same credential"
    );
    let _ = connection.tear_down().await;
    machine.shut_down.shutdown("done".into());
}

/// A wrong value is a key that could not be loaded, **and no credential goes
/// out on it.** The failure is named for what happened here rather than
/// classified from anything the machine said.
#[tokio::test]
async fn a_wrong_passphrase_never_reaches_the_machine() {
    let scratch = Scratch::new("wrong");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "the passphrase");
    let machine = a_machine(&host_key).await;

    let failure = open(
        &endpoint(machine.address),
        credential(&client_key, Some("not the passphrase")),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect_err("a wrong passphrase does not open the container");

    assert!(matches!(failure, OpenFailure::TheKeyCouldNotBeLoaded));
    assert_eq!(
        machine.saw.authentication_attempts.load(Ordering::SeqCst),
        0
    );
    machine.shut_down.shutdown("done".into());
}

/// **§1.1**: one `exec` request carries one command string and returns output,
/// complaint output and exit status. All three come back, and the string that
/// arrived is the constant the binary holds.
#[tokio::test]
async fn one_command_returns_output_complaint_and_status() {
    let scratch = Scratch::new("command");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "");
    let machine = a_machine(&host_key).await;

    let mut connection = open(
        &endpoint(machine.address),
        credential(&client_key, None),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect("the connection opens");

    const UNAME: CommandString = CommandString::constant("uname -s");
    let result = connection
        .run_one_command(UNAME)
        .await
        .expect("the machine ran it");

    assert_eq!(result.output_as_text(), Some("Linux\n"));
    assert_eq!(result.complaint, b"a complaint\n");
    assert_eq!(result.status, Some(7));
    assert!(!result.ended_cleanly());
    assert_eq!(
        *machine.saw.commands.lock().expect("nothing panics holding this"),
        vec!["uname -s".to_string()]
    );

    let _ = connection.tear_down().await;
    machine.shut_down.shutdown("done".into());
}

/// **A command begun is read to its end apart from the connection: a second
/// runs and comes back while the first still runs, and the first, given up, has
/// its channel closed.** Added 2026-09-27, step 10c's eighth-pass fix piece 1.
///
/// **Why.** `Desktop app and delivery`'s point serves every ask on a
/// connection from one place, and a command read to its end there held it for
/// the whole command — on step 10c's eighth pass on Hyperstack, a 52-second
/// driver install held back the run's own state reading and the machine ended
/// the setup. **So the request is sent on the connection and the rest is read
/// elsewhere**; this is that route on a real SSH server. **`Server Setup` Y8**
/// has the desk close a stopped step's channel, and the far end is what says
/// the channel was closed.
#[tokio::test]
async fn a_command_begun_runs_beside_another_and_one_given_up_has_its_channel_closed() {
    let scratch = Scratch::new("begun");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "");
    let machine = a_machine(&host_key).await;

    let mut connection = open(
        &endpoint(machine.address),
        credential(&client_key, None),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect("the connection opens");

    const HELD: CommandString = CommandString::constant(RUNS_UNTIL_ITS_CHANNEL_IS_CLOSED);
    const UNAME: CommandString = CommandString::constant("uname -s");
    let held = connection
        .one_command_begun(HELD, None)
        .await
        .expect("the long command was begun");
    let beside = connection
        .one_command_begun(UNAME, Some(&b"read as data"[..]))
        .await
        .expect("a second command was begun beside it");
    let result = tokio::time::timeout(Duration::from_secs(10), beside.to_its_end())
        .await
        .expect("the command beside came back while the long one ran");
    assert_eq!(result.output_as_text(), Some("Linux\n"));
    assert_eq!(result.complaint, b"a complaint\n");
    assert_eq!(result.status, Some(7));
    assert!(machine
        .saw
        .closed_by_the_caller
        .lock()
        .expect("nothing panics holding this")
        .is_empty());

    drop(held);
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let closed = machine
            .saw
            .closed_by_the_caller
            .lock()
            .expect("nothing panics holding this")
            .clone();
        if !closed.is_empty() {
            let held_open = machine
                .saw
                .held_open
                .lock()
                .expect("nothing panics holding this")
                .clone();
            assert_eq!(closed, held_open, "a channel other than the given-up one was closed");
            break;
        }
        assert!(Instant::now() < deadline, "the given-up command's channel was never closed");
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    let _ = connection.tear_down().await;
    machine.shut_down.shutdown("done".into());
}

/// **`Server Setup` Y7a on a real SSH server: a command's two streams are
/// told as they are printed**, each on its own, while the command is still
/// running — the reading the desktop's elevated channel makes. Added
/// 2026-10-18, the Creator's thirteenth notes, piece H. Before it, a command's
/// words were read only at its end.
#[tokio::test]
async fn a_commands_two_streams_are_told_as_they_print_while_it_runs() {
    use smoozy_server_protocol::transport::ssh::TheStream;
    let scratch = Scratch::new("told");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "");
    let machine = a_machine(&host_key).await;
    let mut connection = open(
        &endpoint(machine.address),
        credential(&client_key, None),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect("the connection opens");

    const PRINTS: CommandString = CommandString::constant(PRINTS_AND_RUNS_ON);
    let under_way = connection
        .one_command_begun(PRINTS, None)
        .await
        .expect("the command was begun");
    let (told, mut heard) = tokio::sync::mpsc::unbounded_channel();
    let reading = tokio::spawn(under_way.to_its_end_told_as_it_comes(move |stream, bytes: &[u8]| {
        let _ = told.send((stream, bytes.to_vec()));
    }));
    let mut pieces = Vec::new();
    while pieces.len() < 2 {
        let piece = tokio::time::timeout(Duration::from_secs(10), heard.recv())
            .await
            .expect("nothing the command printed was told while it ran")
            .expect("the reading ended");
        pieces.push(piece);
    }
    assert!(!reading.is_finished(), "the command ended, so this proves nothing");
    assert!(pieces.contains(&(TheStream::Output, b"Unpacking nvidia-driver-595 ...\n".to_vec())));
    assert!(pieces.contains(&(TheStream::Complaint, b"debconf: delaying\n".to_vec())));
    reading.abort();
    let _ = connection.tear_down().await;
    machine.shut_down.shutdown("done".into());
}

/// **§1.1's second carriage, observed rather than argued** — *"The constant
/// command's standard input, read as data."* The value reaches the far end on
/// the command's own standard input, the command string that arrived is the
/// constant and nothing else, and the caller ends the stream so a program that
/// reads until it ends is not left waiting.
///
/// **The far end is what says so.** It answers with the bytes it was given
/// rather than with anything this crate told it, so a route that wrote nothing
/// would come back empty.
#[tokio::test]
async fn a_value_reaches_the_command_on_its_own_standard_input() {
    let scratch = Scratch::new("standard-input");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "");
    let machine = a_machine(&host_key).await;

    let mut connection = open(
        &endpoint(machine.address),
        credential(&client_key, None),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect("the connection opens");

    const VALIDATE: CommandString = CommandString::constant(READS_ITS_INPUT);
    let result = connection
        .run_one_command_carrying_a_value_on_its_standard_input(
            VALIDATE,
            Plaintext::new("a value that is not in the command string\n"),
        )
        .await
        .expect("the machine ran it");

    assert_eq!(
        machine
            .saw
            .standard_input
            .lock()
            .expect("nothing panics holding this")
            .as_slice(),
        b"a value that is not in the command string\n",
        "the value did not reach the command's standard input"
    );
    assert_eq!(
        result.output_as_text(),
        Some("a value that is not in the command string\n"),
        "the far end answered with something other than what it was given"
    );
    assert_eq!(result.status, Some(0));

    // **§1.1's prohibition, read off the far end.** The command string that
    // arrived carries no part of the value.
    let arrived = machine
        .saw
        .commands
        .lock()
        .expect("nothing panics holding this")
        .clone();
    assert_eq!(arrived, vec![READS_ITS_INPUT.to_string()]);
    assert!(!arrived[0].contains("a value"));

    // **The stream was ended.** A program reading a password reads until it
    // ends, so this is part of the carriage rather than tidying. **Checked by
    // breaking it: with the close taken out this test does not fail, it hangs**
    // — the far end waits for an ending that never comes — which is exactly
    // what the same fault would do on a machine.
    assert!(machine
        .saw
        .the_input_stream_was_ended
        .load(Ordering::SeqCst));

    let _ = connection.tear_down().await;
    machine.shut_down.shutdown("done".into());
}

/// **The other route writes nothing and still ends the stream.**
///
/// A command that reads its input is answered by a caller that has none to
/// give: the stream ends at once, so the far end is never left waiting for a
/// value that is not coming.
#[tokio::test]
async fn a_command_with_no_value_to_carry_ends_the_stream_and_writes_nothing() {
    let scratch = Scratch::new("no-value");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "");
    let machine = a_machine(&host_key).await;

    let mut connection = open(
        &endpoint(machine.address),
        credential(&client_key, None),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect("the connection opens");

    const VALIDATE: CommandString = CommandString::constant(READS_ITS_INPUT);
    let result = connection
        .run_one_command(VALIDATE)
        .await
        .expect("the machine ran it");

    assert_eq!(result.output, Vec::<u8>::new());
    assert!(machine
        .saw
        .standard_input
        .lock()
        .expect("nothing panics holding this")
        .is_empty());
    assert!(machine
        .saw
        .the_input_stream_was_ended
        .load(Ordering::SeqCst));

    let _ = connection.tear_down().await;
    machine.shut_down.shutdown("done".into());
}

/// **`Servers` S10 and K5.** Smoozy's own tear-down is not a break, and the
/// far end answering it with a close does not rename it. A tear-down is one of
/// the six moments that document's S10 ends a period at. **That document's S10
/// and not this one's**, which is about a transfer's accounting.
#[tokio::test]
async fn a_tear_down_smoozy_started_is_not_a_break() {
    let scratch = Scratch::new("teardown");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "");
    let machine = a_machine(&host_key).await;

    let connection = open(
        &endpoint(machine.address),
        credential(&client_key, None),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect("the connection opens");

    assert_eq!(connection.ended(), None, "a live connection has not ended");
    let ending = connection.tear_down().await;
    assert_eq!(ending, Ending::TornDownBySmoozy);
    assert!(!ending.is_a_break());
    machine.shut_down.shutdown("done".into());
}

/// **K2's first bullet.** The far end going away is announced on the transport
/// and is told at once — observed as *"both told to the caller in the same
/// second"* — so it is never the keepalive's business.
#[tokio::test]
async fn a_machine_that_goes_away_announces_it() {
    let scratch = Scratch::new("announced");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "");
    let machine = a_machine(&host_key).await;

    let mut connection = open(
        &endpoint(machine.address),
        credential(&client_key, None),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect("the connection opens");

    let began = Instant::now();
    machine.shut_down.shutdown("the machine is going".into());
    let ending = connection.wait_until_it_ends().await;

    assert_eq!(ending, Ending::Announced);
    assert!(ending.is_a_break());
    // **The point of the clause**: an announced ending does not wait for the
    // keepalive to notice. Its whole window is three intervals away.
    assert!(
        began.elapsed() < AT_THE_DESK.interval,
        "an announced ending is told at once, not counted out"
    );
}

/// **K2's second bullet and K3, and this is the observation the whole section
/// exists for.** The caller's own line stops carrying bytes back. Nothing is
/// wrong with the machine, nothing announces anything, and the caller has
/// nothing to do on the connection — which is exactly the case that went
/// **undetected for 21 minutes 45 seconds** with keepalives off. The break is
/// declared, and it is declared inside the window the figures fix.
#[tokio::test]
async fn a_line_that_stops_answering_declares_a_break_at_the_figures() {
    let scratch = Scratch::new("silence");
    let host_key = scratch.a_key("host", "");
    let client_key = scratch.a_key("client", "");
    let machine = a_machine(&host_key).await;

    // A relay in the middle, so one direction can be silenced without the
    // socket being closed. Closing it would be K2's *first* bullet, which is
    // the thing this test must not be.
    let silence = Arc::new(AtomicBool::new(false));
    let relay = a_one_way_relay(machine.address, Arc::clone(&silence)).await;

    let mut connection = open(
        &endpoint(relay),
        credential(&client_key, None),
        AT_THE_DESK,
        Box::new(Compares {
            answer: HostTrust::Proceed,
            offered: Arc::new(Mutex::new(None)),
        }),
    )
    .await
    .expect("the connection opens through the relay");

    silence.store(true, Ordering::SeqCst);
    let began = Instant::now();
    let ending = connection.wait_until_it_ends().await;
    let took = began.elapsed();

    assert_eq!(ending, Ending::TheKeepaliveWentUnanswered);
    assert!(ending.is_a_break());

    // **The window the figures fix**, read from the figures rather than
    // written again here. The lower bound is the whole point: a break declared
    // sooner would mean the count was not being kept.
    let (opens, closes) = AT_THE_DESK.the_declaration_window();
    assert!(
        took >= opens,
        "declared after {took:?}, which is inside the count: {opens:?}"
    );
    assert!(
        took < closes * 3,
        "declared after {took:?}, far outside the window closing at {closes:?}"
    );

    machine.shut_down.shutdown("done".into());
}

/// One connection through, with the answers in one direction droppable.
///
/// It reads everything the machine sends and throws it away while silenced, so
/// the socket stays open and nothing is announced. That is what a line failing
/// looks like from the caller's end.
async fn a_one_way_relay(machine: SocketAddr, silence: Arc<AtomicBool>) -> SocketAddr {
    let listener = TcpListener::bind(("127.0.0.1", 0))
        .await
        .expect("the loopback address takes a listener");
    let address = listener.local_addr().expect("a bound listener has an address");

    tokio::spawn(async move {
        let (caller, _) = listener.accept().await.expect("one caller arrives");
        let upstream = tokio::net::TcpStream::connect(machine)
            .await
            .expect("the machine is listening");
        let (mut from_caller, mut to_caller) = caller.into_split();
        let (mut from_machine, mut to_machine) = upstream.into_split();

        // Caller to machine, always carried: the caller's keepalives keep
        // going out, which is what makes the far end's silence the only
        // difference.
        tokio::spawn(async move {
            let mut buffer = [0u8; 8192];
            loop {
                match from_caller.read(&mut buffer).await {
                    Ok(0) | Err(_) => break,
                    Ok(read) => {
                        if to_machine.write_all(&buffer[..read]).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        let mut buffer = [0u8; 8192];
        loop {
            match from_machine.read(&mut buffer).await {
                Ok(0) | Err(_) => break,
                Ok(read) => {
                    if silence.load(Ordering::SeqCst) {
                        continue;
                    }
                    if to_caller.write_all(&buffer[..read]).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    address
}

//! **Step 8b's placement performer, run rather than argued: a verified release
//! is claimed, staged, read back at rest, registered, committed, health-checked
//! and — where the exchange fails — rolled back, with the boot-registration
//! record restored beside the registration.**
//!
//! **What the far end is.** A real SSH connection, opened by Smoozy's own
//! transport on the loopback address, carrying **OpenSSH's own `sftp-server`**
//! on a subsystem session and **the machine's own shell** on the command
//! channel. Nothing is rented; nothing on the Creator's machine outside a
//! temporary directory is touched — the file-transfer server and every command
//! are started with that directory as their working directory and as their
//! home, so a relative path and a tilde alike land inside it.
//!
//! **What is proved here, in the plan's own terms.**
//!
//! - **`Rented-server runtime` G1b** — the claim is the exclusive create, a
//!   second claimant is told a transaction is in progress and changes nothing,
//!   and a claim naming this installation is taken back with no timer.
//! - **`Rented-server runtime` G3, G4 and G7** — the phase record round-trips
//!   through the surface, and the commit is the single rename of the pointer.
//! - **`Rented-server runtime` K16, K16a, K16b and K15** — the record is
//!   written before the registration, and a provider default over the
//!   registration reads back as *content differs*.
//! - **`Rented-server runtime` G12 and G13** — the health exchange runs over
//!   §1.1's own two mechanisms, through the planted-link carriage, and a
//!   supervisor that does not answer is the same failure as one that answers
//!   wrongly.
//! - **`Rented-server runtime` G14 and G14a** — the rollback restores the
//!   pointer, the registration and **the record that describes it**, and the
//!   read-back afterwards says *content matches the record* rather than
//!   *content differs*.
//!
//! **What is not the real thing, and each says so where it stands.** The staged
//! artifact is a shell script this test wrote rather than the Linux binary a
//! release would carry, **which cannot be linked at this desk** — the desk's
//! linker is Apple's and the machine's target wants the GNU one. The script
//! stands in for the SSH command adapter and answers one framed request; **it is
//! not the machine's own program**, and what that program would do with the same
//! frame is proved in its own package and not here. And the systemd machine
//! kind's three user-level commands are not run at all, because this desk
//! carries neither a service manager nor a login manager.
//!
//! **Where `sftp-server` is not on this system the tests say so and stop.** They
//! do not quietly pass.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` G1b, G10, G12, G13, G14,
//! G14a, G15, G16, G18, G25, G31a, K11, K15, K16, K16a, K16b, K19.
//!
//! **A marker this plan also defines is never listed there and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.
//! `Rented-server runtime` G3, `Rented-server runtime` G4,
//! `Rented-server runtime` G7 and `Rented-server runtime` G9 are of that kind
//! here.
//!
//! **The plan here is
//! `Preparation/Architecture/Server protocol and transfers.md`, and the words
//! above are the words `src/lib.rs` carries.** They are repeated because a file
//! under `tests/` is its own crate and never sees that one.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::{Duration, Instant};

use russh::server::{Auth, Msg, Server as _, Session};
use russh::{Channel, ChannelId};
use smoozy_framing::framing::{
    read_one_request, write_result_frame, Answered, Fields, Request,
    THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};
use smoozy_runtime::machine_kind::MachineKind;
use smoozy_runtime::package::commit::{
    the_record_a_rollback_restores, HealthAnswer, TheRestoresTwoSteps,
};
use smoozy_runtime::package::phase::{
    a_fresh_attempts_first_write, ClaimOutcome, Phase, PreviousVersion, Route, TransactionOwner,
};
use smoozy_runtime::package::release::RetainedAdapter;
use smoozy_runtime::registration::{confirm, BootRegistrationRecord, RegistrationConfirmation};
use smoozy_runtime::the_program::THE_FIXED_ENTRY_POINT_NAME;
use smoozy_server_protocol::bootstrap::{inspect, WhatARecordHeld};
use smoozy_server_protocol::placement::performing::{
    ask_for_the_health_answer, complete_the_rollback, place_a_verified_release,
    ThePlacementsSurface, WhatThePlacementNeeds,
    WhatThePlacementReached, THE_ENTRY_POINTS_CARRIAGE_SITE,
    THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR, THE_HEALTH_ANSWER_REQUEST,
};
use smoozy_server_protocol::placement::{
    read_the_boot_registration_record, read_the_phase_record, take_the_claim,
    the_active_pointers_value, the_staging_location_for, write_the_boot_registration_record,
    write_the_phase_record,
    WhatTakingTheClaimReached, WhetherTheDurabilityOrderingApplied,
};
use smoozy_server_protocol::release::composing::{
    self, AFetchFromALocalFileStandingInForHostingThatDoesNotExist,
    FetchesFromAnExactSourceTheManifestNamed,
};
use smoozy_server_protocol::release::trust::{
    AnAllowedKey, KeySetStatement, THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
    THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE, THE_KEY_SET_FORMAT,
};
use smoozy_server_protocol::release::verifying::{
    verify_before_download, verify_what_was_downloaded, AVerifiedRelease, AnOfferedRelease,
};
use smoozy_server_protocol::release::{
    a_signature_over, THE_KEY_SET_SIGNING_CONTEXT, THE_MANIFEST_SIGNING_CONTEXT,
};
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::transport::ssh::{open, Connection};
use smoozy_server_protocol::transport::{
    Credential, DecideHostTrust, Endpoint, Figures, HostTrust,
};
use smoozy_state::identities::{Identity, IdentityKind};
use tokio::net::TcpListener;

// ------------------------------------------- where OpenSSH's own server is

const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
    "/usr/libexec/sftp-server",
    "/usr/lib/openssh/sftp-server",
    "/usr/libexec/openssh/sftp-server",
    "/usr/lib/ssh/sftp-server",
    "C:\\Windows\\System32\\OpenSSH\\sftp-server.exe",
];

fn the_machines_own_file_transfer_server() -> Option<PathBuf> {
    WHERE_SFTP_SERVER_LIVES
        .iter()
        .map(PathBuf::from)
        .find(|path| path.exists())
}

macro_rules! the_server_or_say_so {
    () => {
        match the_machines_own_file_transfer_server() {
            Some(path) => path,
            None => {
                println!(
                    "SKIPPED: no OpenSSH sftp-server on this system ({}). \
                     The placement through the surface is unproved here.",
                    std::env::consts::OS
                );
                return;
            }
        }
    };
}

/// Says out loud that what comes next needs a Linux-shaped far end, and stops —
/// **never a quieter assertion**.
///
/// The pointer is a symbolic link, the staged artifact is made runnable by its
/// owner alone, and the command channel runs a shell script. **Microsoft's own
/// port of OpenSSH keeps none of the three**, so a Windows far end standing in
/// would be judged on parts it never plays.
macro_rules! a_linux_shaped_far_end {
    () => {
        if cfg!(windows) {
            println!(
                "SKIPPED ({}): the pointer, the mode and the shell are Linux rules about the \
                 rented machine. Unproved here.",
                std::env::consts::OS
            );
            return;
        }
    };
}

// --------------------------------------------------- the machine, on loopback

/// The far end. **Its working directory and its home are the scratch
/// directory**, so a relative subsystem path and a tilde in a command string
/// both land inside it and nothing on the Creator's own machine is touched.
struct Machine {
    sftp_server: PathBuf,
    home: PathBuf,
    /// **A folder of the machine's own tools put in front of the path**, for the
    /// tests that stand in a service manager and a login manager this desk has
    /// none of. `None` leaves the command channel's path as it is.
    tools: Option<PathBuf>,
    channels: std::collections::HashMap<ChannelId, Channel<Msg>>,
}

impl Clone for Machine {
    fn clone(&self) -> Machine {
        Machine {
            sftp_server: self.sftp_server.clone(),
            home: self.home.clone(),
            tools: self.tools.clone(),
            channels: std::collections::HashMap::new(),
        }
    }
}

impl russh::server::Server for Machine {
    type Handler = Self;
    fn new_client(&mut self, _address: Option<std::net::SocketAddr>) -> Self {
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
        Ok(Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        reply: russh::server::ChannelOpenHandle,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        reply.accept().await;
        self.channels.insert(channel.id(), channel);
        Ok(())
    }

    /// **§1.1's first mechanism, served by the machine's own login shell** —
    /// which is what `sshd(8)` says runs a command string: *"All commands are
    /// run under the user's login shell as specified in the system password
    /// database"*.
    async fn exec_request(
        &mut self,
        channel: ChannelId,
        command: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        session.channel_success(channel)?;
        let Some(taken) = self.channels.remove(&channel) else {
            return Ok(());
        };
        let line = String::from_utf8_lossy(command).into_owned();
        let home = self.home.clone();
        let path = match &self.tools {
            Some(tools) => format!(
                "{}:{}",
                tools.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
            None => std::env::var("PATH").unwrap_or_default(),
        };
        // **§1.1 says one `exec` returns output, complaint output and exit
        // status**, so the far end sends all three — the status through the
        // session's own handle, which is the only route a server has for it.
        let handle = session.handle();
        tokio::spawn(async move {
            let mut child = match tokio::process::Command::new("/bin/sh")
                .arg("-c")
                .arg(&line)
                .current_dir(&home)
                .env("HOME", &home)
                .env("PATH", &path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(child) => child,
                Err(_) => {
                    let _ = handle.exit_status_request(channel, 127).await;
                    let _ = handle.eof(channel).await;
                    let _ = handle.close(channel).await;
                    return;
                }
            };
            let mut to_the_command = child.stdin.take().expect("piped");
            let mut from_the_command = child.stdout.take().expect("piped");
            let stream = taken.into_stream();
            let (mut reading, mut writing) = tokio::io::split(stream);
            let inward = tokio::spawn(async move {
                let _ = tokio::io::copy(&mut reading, &mut to_the_command).await;
                drop(to_the_command);
            });
            let _ = tokio::io::copy(&mut from_the_command, &mut writing).await;
            let _ = inward.await;
            let ended = child.wait().await;
            let code = ended.ok().and_then(|status| status.code()).unwrap_or(255);
            let _ = handle.exit_status_request(channel, code as u32).await;
            let _ = handle.eof(channel).await;
            let _ = handle.close(channel).await;
        });
        Ok(())
    }

    async fn subsystem_request(
        &mut self,
        channel: ChannelId,
        name: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        if name != "sftp" {
            session.channel_failure(channel)?;
            return Ok(());
        }
        session.channel_success(channel)?;
        let Some(taken) = self.channels.remove(&channel) else {
            return Ok(());
        };
        let program = self.sftp_server.clone();
        let home = self.home.clone();
        tokio::spawn(async move {
            let mut child = match tokio::process::Command::new(program)
                .current_dir(&home)
                .env("HOME", &home)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(child) => child,
                Err(_) => return,
            };
            let mut to_the_server = child.stdin.take().expect("piped");
            let mut from_the_server = child.stdout.take().expect("piped");
            let stream = taken.into_stream();
            let (mut reading, mut writing) = tokio::io::split(stream);
            let inward = tokio::spawn(async move {
                let _ = tokio::io::copy(&mut reading, &mut to_the_server).await;
            });
            let _ = tokio::io::copy(&mut from_the_server, &mut writing).await;
            inward.abort();
            let _ = child.kill().await;
        });
        Ok(())
    }
}

struct RunningMachine {
    address: std::net::SocketAddr,
    _shut_down: russh::server::RunningServerHandle,
}

async fn a_machine(host_key_file: &Path, sftp_server: PathBuf, home: PathBuf) -> RunningMachine {
    a_machine_with_its_own_tools(host_key_file, sftp_server, home, None).await
}

async fn a_machine_with_its_own_tools(
    host_key_file: &Path,
    sftp_server: PathBuf,
    home: PathBuf,
    tools: Option<PathBuf>,
) -> RunningMachine {
    let host_key = russh::keys::load_secret_key(host_key_file, None).expect("ssh-keygen wrote it");
    let config = Arc::new(russh::server::Config {
        keys: vec![host_key],
        auth_rejection_time: Duration::from_millis(0),
        auth_rejection_time_initial: Some(Duration::from_millis(0)),
        ..russh::server::Config::default()
    });
    let socket: &'static TcpListener = Box::leak(Box::new(
        TcpListener::bind(("127.0.0.1", 0)).await.expect("loopback"),
    ));
    let address = socket.local_addr().expect("bound");
    let machine: &'static mut Machine = Box::leak(Box::new(Machine {
        sftp_server,
        home,
        tools,
        channels: std::collections::HashMap::new(),
    }));
    let running = machine.run_on_socket(config, socket);
    let shut_down = running.handle();
    tokio::spawn(running);
    RunningMachine {
        address,
        _shut_down: shut_down,
    }
}

// ------------------------------------------------------------------ the desk

struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!(
            "smoozy-placement-{name}-{}-{}",
            std::process::id(),
            Instant::now().elapsed().as_nanos()
        ));
        std::fs::create_dir_all(&path).expect("a temporary directory can be made");
        Scratch(path)
    }

    fn a_key(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        let made = std::process::Command::new("ssh-keygen")
            .args(["-t", "ed25519", "-N", "", "-C", "smoozy placement test", "-f"])
            .arg(&path)
            .output()
            .expect("ssh-keygen is on the path");
        assert!(made.status.success());
        path
    }

    fn at(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

/// **`Rented-server runtime` K11's and K19's root step, pointed into the scratch
/// directory** — the one thing about it a desk cannot keep, `/root` being the
/// Creator's own machine's and not the test's.
///
/// **The commands are `Server Setup`'s own shape with the path changed and
/// nothing else**: the plain copy from the staging token, and no elevated form,
/// because the desk's login can write its own scratch directory. **A command
/// string takes only a constant**, so each is leaked — the deliberate act
/// `CommandString`'s own page names as the only way round, taken in a test.
fn a_root_step_into(scratch: &Scratch) -> smoozy_server_protocol::placement::TheRootStepOnAContainer {
    let hook = scratch.at("onstart.sh").to_string_lossy().into_owned();
    let copy: &'static str = Box::leak(
        format!("cp ~/.smoozy-carriage/container-hook {hook}").into_boxed_str(),
    );
    smoozy_server_protocol::placement::TheRootStepOnAContainer {
        the_hooks_path: hook,
        the_staging_token_on_the_subsystem: ".smoozy-carriage/container-hook".to_string(),
        the_copy: smoozy_server_protocol::placement::TheCommandAndItsElevatedForm {
            plain: smoozy_server_protocol::transport::CommandString::constant(copy),
            elevated: None,
        },
        the_hook_made_readable: None,
        the_directory_made_traversable: None,
        the_hook_read: None,
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct Approves;
impl DecideHostTrust for Approves {
    fn decide(&mut self, _offered: &[u8]) -> HostTrust {
        HostTrust::Proceed
    }
}

async fn a_connection(machine: &RunningMachine, key: &Path) -> Connection {
    open(
        &Endpoint {
            host: machine.address.ip().to_string(),
            port: machine.address.port(),
            username: "whoever".to_string(),
        },
        Credential {
            private_key_file: key.to_path_buf(),
            passphrase: None,
        },
        Figures {
            interval: Duration::from_secs(15),
            unanswered_before_the_break: 3,
        },
        Box::new(Approves),
    )
    .await
    .expect("the loopback machine takes the connection")
}

async fn a_subsystem(
    connection: &Connection,
) -> Subsystem<russh::ChannelStream<russh::client::Msg>> {
    let channel = connection
        .open_a_subsystem_channel()
        .await
        .expect("the machine serves the file-transfer subsystem");
    Subsystem::negotiate(channel)
        .await
        .expect("the subsystem negotiates")
}

// ------------------------------------------------------------- the release

/// **The keys are derived from seeds written out here, so they are public.**
const THE_ROOT_SEED: [u8; 32] = *b"a release root for this test onl";
const THE_ONLINE_SEED: [u8; 32] = *b"an online signing key, this test";

/// **The source is an address nobody serves.** `example.invalid` is reserved by
/// its own registry for exactly this.
const THE_SOURCE: &str = "https://example.invalid/smoozy-server/0.1.0/linux-x86_64-gnu";

struct AReleaseAsHostingWouldServeIt {
    offered: AnOfferedRelease,
    root_public_key: [u8; 32],
}

fn a_release(artifact: &[u8]) -> AReleaseAsHostingWouldServeIt {
    a_release_at(artifact, "0.1.0")
}

/// The same release, **at a package version the caller names** — the real
/// program answers with the version it was built as, so a release carrying it
/// has to say the same.
/// **Published the way the next real release will be: one past the release
/// sequence this build carries as its floor, at the key-set generation this
/// build carries** — the release a desktop built today actually meets. *(Until
/// 2026-09-21 it was sequence 7 and generation 1, which passed only while
/// `Server protocol and transfers` B9's built-in pair was zero; the pair is now
/// the first published release's, and a sequence below it is refused, as it
/// should be.)*
fn a_release_at(artifact: &[u8], package_version: &str) -> AReleaseAsHostingWouldServeIt {
    let root = composing::test_only_keys::a_key_pair_from_a_written_down_seed(THE_ROOT_SEED);
    let online = composing::test_only_keys::a_key_pair_from_a_written_down_seed(THE_ONLINE_SEED);

    let statement = KeySetStatement {
        statement_format: THE_KEY_SET_FORMAT.into(),
        generation: THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
        allowed: vec![AnAllowedKey {
            identity: "the online signing key".into(),
            public_key: online.public,
            valid_from_release_sequence: 1,
        }],
        revocations: Vec::new(),
    };
    let statement_bytes = statement.canonical_bytes();

    let entry = composing::compose_an_artifact_entry(
        "linux-x86_64-gnu",
        composing::the_tuple_this_build_is_made_for(),
        artifact,
        composing::THE_FIXED_ENTRY_POINT_NAME,
        &[THE_SOURCE.to_string()],
    );
    let manifest = composing::compose_a_manifest(
        THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE + 1,
        package_version,
        2,
        None,
        vec![entry],
    );
    let manifest_bytes = manifest.canonical_bytes();

    AReleaseAsHostingWouldServeIt {
        offered: AnOfferedRelease {
            its_root_signature: a_signature_over(
                &root.secret,
                THE_KEY_SET_SIGNING_CONTEXT,
                &statement_bytes,
            ),
            the_key_set_statements_bytes: statement_bytes,
            its_online_signature: a_signature_over(
                &online.secret,
                THE_MANIFEST_SIGNING_CONTEXT,
                &manifest_bytes,
            ),
            the_manifests_bytes: manifest_bytes,
        },
        root_public_key: root.public,
    }
}

fn verified_and_fetched(
    release: &AReleaseAsHostingWouldServeIt,
    served: &Path,
) -> (AVerifiedRelease, Vec<u8>) {
    let mut fetch =
        AFetchFromALocalFileStandingInForHostingThatDoesNotExist::where_each_source_stands_for_a_file(
            vec![(THE_SOURCE.to_string(), served.to_path_buf())],
        );
    let verified = verify_before_download(
        &release.root_public_key,
        None,
        &release.offered,
        &composing::the_tuple_this_build_is_made_for(),
        None,
    )
    .expect("the release verifies");
    let source = verified.selected.sources[0].clone();
    let bytes = fetch.fetch(&source).expect("the source answers");
    verify_what_was_downloaded(&verified.selected, &source, &bytes)
        .expect("the downloaded bytes are the artifact");
    (verified, bytes)
}

// -------------------------------------------------------- the stand-in adapter

/// **The staged artifact, standing in for the machine's own program.**
///
/// It reads the framed request off its standard input, writes it down where the
/// test can read it, and answers with a frame the test composed. **It is not the
/// machine's own adapter** and proves nothing about what that program does with
/// the same frame; what it proves is the carriage — that the constant command
/// reaches the entry point the pointer names, through the link, with the frame
/// on standard input and the result on standard output.
fn a_stand_in_adapter(what_it_was_given: &Path, the_answer_frame: &Path) -> Vec<u8> {
    format!(
        "#!/bin/sh\n\
         # Smoozy's own program would frame one request and pass it to the\n\
         # supervisor over the machine-local endpoint. This writes the request\n\
         # down and answers with a frame the test composed.\n\
         cat > '{}'\n\
         printf '%s' \"$0\" > '{}.invoked-as'\n\
         cat '{}'\n",
        what_it_was_given.display(),
        what_it_was_given.display(),
        the_answer_frame.display()
    )
    .into_bytes()
}

/// The framed result the stand-in answers with — `Rented-server runtime` G12's
/// four things and nothing else.
fn the_answer_frame(
    request: &Request,
    package_version: &str,
    run: &Identity,
    installation: &Identity,
    phase: Phase,
) -> Vec<u8> {
    let answered = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, package_version)
        .carrying(
            Fields::none()
                .and(THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[0], package_version)
                .and(THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[1], run.as_text())
                .and(
                    THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[2],
                    installation.as_text(),
                )
                .and(
                    THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR[3],
                    serde_json::to_string(&phase).expect("a phase writes back"),
                ),
        );
    write_result_frame(&answered)
}

// ------------------------------------------------------------ small helpers

fn an_identity(kind: IdentityKind) -> Identity {
    Identity::issue(kind).expect("the desk has randomness")
}

fn an_owner(installation: Identity, route: Route) -> TransactionOwner {
    TransactionOwner {
        invocation: an_identity(IdentityKind::Operation),
        route,
        process: (std::process::id(), 19_186),
        installation,
    }
}

fn a_previous_version(location: &str, registration: &[u8]) -> PreviousVersion {
    PreviousVersion {
        descriptor: smoozy_runtime::package::release::VerifiedReleaseDescriptor {
            manifest_format: "smoozy-server-package-manifest-1".into(),
            verified_release_sequence: 6,
            requested_package_version: "0.0.9".into(),
            manifest_digest: [1; 32],
            artifact_identity: "linux-x86_64-gnu".into(),
            artifact_byte_length: 11,
            artifact_digest: [2; 32],
        },
        retained_adapter: RetainedAdapter {
            declared_interval: (1, 1),
            declared_request_set: vec!["readiness".into()],
        },
        location: location.to_string(),
        previous_boot_registration_content: registration.to_vec(),
    }
}

// ------------------------------------------------------------------ the tests

/// **`Rented-server runtime` G1b, run against a real file-transfer server: the
/// claim is the exclusive create, and a claim that stands changes nothing.**
///
/// **Three things, and the third is the one no desk test could do before.** An
/// absent claim is acquired. A claim naming another installation is met with
/// *a transaction is in progress*, carrying the owner and the phase, and
/// nothing is written. And a claim naming this installation is taken back with
/// no timer — `Rented-server runtime` G18 refuses a deadline and no function in
/// that file takes one.
#[tokio::test]
async fn the_claim_is_the_exclusive_create_and_a_standing_one_changes_nothing() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("claim");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server, scratch.0.clone()).await;
    let connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let area = scratch.at("package-area");
    std::fs::create_dir_all(&area).expect("the area is made");
    let area = area.to_string_lossy().into_owned();

    let mine = an_identity(IdentityKind::Installation);
    let theirs = an_identity(IdentityKind::Installation);

    assert_eq!(
        take_the_claim(
            &mut subsystem,
            &area,
            &an_owner(theirs.clone(), Route::SetupPlacement),
            Phase::Staging
        )
        .await
        .expect("the claim answered")
        .0,
        WhatTakingTheClaimReached::TheRulesHalfSays(ClaimOutcome::Acquired),
        "an absent claim is acquired"
    );

    // **A claim naming another installation is never taken over, whatever its
    // age**, and this one is seconds old.
    let met = take_the_claim(
        &mut subsystem,
        &area,
        &an_owner(mine.clone(), Route::AutomaticRepair),
        Phase::Inspecting,
    )
    .await
    .expect("the claim answered");
    match met.0 {
        WhatTakingTheClaimReached::TheRulesHalfSays(ClaimOutcome::InProgress { owner, phase }) => {
            assert_eq!(owner.installation, theirs);
            assert_eq!(phase, Phase::Staging);
        }
        other => panic!("a standing claim was taken over: {other:?}"),
    }

    // **One take-over is permitted and it needs no timer.**
    assert_eq!(
        take_the_claim(
            &mut subsystem,
            &area,
            &an_owner(theirs.clone(), Route::UserFallback),
            Phase::Committing
        )
        .await
        .expect("the claim answered")
        .0,
        WhatTakingTheClaimReached::TheRulesHalfSays(ClaimOutcome::TakenBackFromThisInstallation)
    );

    // **Removed at the terminal write, and by nothing else** — after which the
    // exclusive create succeeds again, which is what says the file was gone.
    smoozy_server_protocol::placement::release_the_claim_at_the_terminal_write(
        &mut subsystem,
        &area,
    )
    .await
    .expect("the claim is released");
    assert_eq!(
        take_the_claim(
            &mut subsystem,
            &area,
            &an_owner(mine, Route::AutomaticRepair),
            Phase::Inspecting
        )
        .await
        .expect("the claim answered")
        .0,
        WhatTakingTheClaimReached::TheRulesHalfSays(ClaimOutcome::Acquired)
    );
}

/// **`Rented-server runtime` G14 and G14a, run end to end: the rollback puts
/// back the pointer, the registration and the record that describes it — and
/// the read-back afterwards agrees.**
///
/// **This is the whole of what G14a is for.** Before it, a restore put the
/// previous content back on the machine and left the record holding this
/// placement's, so `Rented-server runtime` K15's read-back answered *content
/// differs* — silently at the moment it happened, permanently afterwards, on a
/// machine the plan calls correct. **The test asserts both halves**: that the
/// record on the machine would have said *differs* without the restore, and
/// that it says *matches* with it.
#[tokio::test]
async fn a_rollback_restores_the_registration_and_the_record_that_describes_it() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    a_linux_shaped_far_end!();
    let scratch = Scratch::new("rollback");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server, scratch.0.clone()).await;
    let mut connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let area_path = scratch.at("package-area");
    std::fs::create_dir_all(&area_path).expect("the area is made");
    let area = area_path.to_string_lossy().into_owned();

    // The previous version's own material, and the registration it was placed
    // with. **`Rented-server runtime` G9's case**: this placement changed the
    // registration's shape, so there is something to restore on both halves.
    let previous_location = area_path.join("staged-0.0.9").to_string_lossy().into_owned();
    std::fs::write(&previous_location, b"the previous").expect("the previous package is there");
    let the_registrations_path = area_path.join("boot-hook").to_string_lossy().into_owned();
    let the_previous_registration = b"#!/bin/sh\n# the previous registration\n".to_vec();
    let this_placements_registration = b"#!/bin/sh\n# this placement's registration\n".to_vec();
    std::fs::write(&the_registrations_path, &this_placements_registration)
        .expect("this placement's registration is on the machine");

    // **K16b's order in the forward direction**: the record describes what is on
    // the machine.
    let this_placements_record = BootRegistrationRecord {
        kind: MachineKind::Container,
        at_path: the_registrations_path.clone(),
        content: this_placements_registration.clone(),
    };
    write_the_boot_registration_record(&mut subsystem, &area, &this_placements_record)
        .await
        .expect("the record is written");

    let previous = a_previous_version(&previous_location, &the_previous_registration);
    let staging = area_path.join("staged-0.1.0").to_string_lossy().into_owned();
    std::fs::write(&staging, b"this placement").expect("this placement's package is there");

    let mut record = a_fresh_attempts_first_write(
        an_identity(IdentityKind::Operation),
        smoozy_runtime::package::release::VerifiedReleaseDescriptor {
            manifest_format: "smoozy-server-package-manifest-1".into(),
            verified_release_sequence: 7,
            requested_package_version: "0.1.0".into(),
            manifest_digest: [3; 32],
            artifact_identity: "linux-x86_64-gnu".into(),
            artifact_byte_length: 14,
            artifact_digest: [4; 32],
        },
        Some(previous.clone()),
        MachineKind::Container,
        staging.clone(),
    );
    record.phase = Phase::CommittedUnverified;
    write_the_phase_record(&mut subsystem, &area, &record)
        .await
        .expect("the phase record is written");

    // The pointer names this placement, which is the state a failed exchange
    // meets.
    subsystem
        .plant_a_link(
            staging.as_bytes(),
            area_path
                .join(THE_FIXED_ENTRY_POINT_NAME)
                .to_string_lossy()
                .into_owned()
                .as_bytes(),
        )
        .await
        .expect("the pointer is planted");

    // **What the read-back said before the rollback**, so the test says what
    // G14a is for rather than asserting the happy end alone.
    let restored_record = the_record_a_rollback_restores(&this_placements_record, &previous);
    assert_eq!(
        confirm(&this_placements_record, Ok(&the_previous_registration)),
        RegistrationConfirmation::ContentDiffers {
            bytes_on_the_machine: the_previous_registration.len(),
            bytes_recorded: this_placements_registration.len(),
        },
        "without the record's own restore, every later read-back answers content differs"
    );

    let mut surface = ThePlacementsSurface::of(&mut subsystem, &mut connection);
    assert_eq!(
        complete_the_rollback(&mut surface, &area, &record)
            .await
            .expect("the rollback completed"),
        WhatThePlacementReached::RolledBack
    );

    // **G14: the pointer is the previous version's, from the recorded value.**
    assert_eq!(
        the_active_pointers_value(&mut subsystem, &area)
            .await
            .expect("the pointer reads"),
        Some(previous_location.clone())
    );

    // **G14: the registration's own bytes are the previous version's.**
    assert_eq!(
        std::fs::read(&the_registrations_path).expect("the registration is there"),
        the_previous_registration
    );

    // **G14a: and so is the record that describes it** — read back the way
    // package inspection reads it, over the real file-transfer server.
    let WhatARecordHeld::It(back) = read_the_boot_registration_record(&mut subsystem, &area)
        .await
        .expect("the record reads")
    else {
        panic!("the boot-registration record did not read back after the rollback");
    };
    assert_eq!(back, restored_record);
    assert_eq!(
        confirm(
            &back,
            Ok(&std::fs::read(&the_registrations_path).expect("the registration is there"))
        ),
        RegistrationConfirmation::ContentMatchesTheRecord,
        "the record and the machine agree after the rollback, which is what G14a is for"
    );

    // **The two steps are K16b's order and it is read from the plan's own
    // value** rather than from the order two calls sit in.
    assert_eq!(
        TheRestoresTwoSteps::IN_ORDER[0],
        TheRestoresTwoSteps::TheRecordHoldingThePreviousContent
    );

    // **G16's own condition is discharged**: the phase reached *rolled back*
    // rather than being left at *rolling back*.
    let WhatARecordHeld::It(after) = read_the_phase_record(&mut subsystem, &area)
        .await
        .expect("the phase record reads")
    else {
        panic!("the phase record did not read back");
    };
    assert_eq!(after.phase, Phase::RolledBack);
}

/// **`Rented-server runtime` G12 and G13, run over both of §1.1's mechanisms:
/// the health exchange reaches the entry point through the planted-link
/// carriage, and a machine with no adapter answers nothing.**
///
/// **The thing this proves that no unit test could.** The constant command names
/// a carriage token; the link points at the package area; the shell expands the
/// tilde; and the program invoked is the one the active-package pointer names —
/// **so the directory it was invoked from is the package area, through the
/// link**, which is what the machine's own program derives its package area
/// from. The stand-in writes its own invoked path down and the test reads it.
#[tokio::test]
async fn the_health_exchange_reaches_the_entry_point_through_the_carriage() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    a_linux_shaped_far_end!();
    let scratch = Scratch::new("health");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server, scratch.0.clone()).await;
    let mut connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let area_path = scratch.at("package-area");
    std::fs::create_dir_all(&area_path).expect("the area is made");
    let area = area_path.to_string_lossy().into_owned();

    // **A machine with no adapter at all answers nothing**, which is G13's
    // *did not answer* and is the same failure as a wrong answer.
    assert!(
        ask_for_the_health_answer(
            &mut subsystem,
            &mut connection,
            &area,
            an_identity(IdentityKind::Operation)
        )
        .await
        .expect("the exchange answered")
        .is_none(),
        "a machine with no entry point answered a health request"
    );

    // Now the stand-in, at the staging location, with the pointer naming it.
    let given = scratch.at("what-the-adapter-was-given");
    let frame_file = scratch.at("the-answer-frame");
    let staged = area_path.join("staged-0.1.0");
    std::fs::write(&staged, a_stand_in_adapter(&given, &frame_file))
        .expect("the stand-in is written");
    // **The Linux-shaped arrangement, in one block, because it is one thing:**
    // the stand-in is made executable and the pointer is planted at the fixed
    // entry-point name. **Both are rules about a rented machine, which is always
    // Linux** — the machine's own program is built for that one target and no
    // other — so neither has a Windows meaning to keep, and this test's `a_linux
    // _shaped_far_end!()` above has already returned before this line is reached
    // on Windows.
    //
    // **The guard is here so that the file compiles on Windows, not so that the
    // test behaves differently there.** Windows has no `std::os::unix`, and one
    // unguarded call to it stops the whole crate's test binary from building —
    // which is what it did on hosted run `34110919264`, taking the portable test
    // in this same file down with it. **The runtime skip and this compile guard
    // are two mechanisms for one fact and each does what the other cannot**: the
    // skip prints a reason a person reading the log can see, and the guard makes
    // the file buildable. Removing the skip would leave this block silently
    // absent on Windows, so the two move together.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&staged, std::fs::Permissions::from_mode(0o700))
            .expect("the mode is set");
        std::os::unix::fs::symlink(&staged, area_path.join(THE_FIXED_ENTRY_POINT_NAME))
            .expect("the pointer is planted");
    }

    let identity = an_identity(IdentityKind::Operation);
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity.clone(),
        THE_HEALTH_ANSWER_REQUEST,
    );
    let run = an_identity(IdentityKind::Run);
    let installation = an_identity(IdentityKind::Installation);
    std::fs::write(
        &frame_file,
        the_answer_frame(
            &request,
            "0.1.0",
            &run,
            &installation,
            Phase::CommittedUnverified,
        ),
    )
    .expect("the answer frame is written");

    let answered = ask_for_the_health_answer(&mut subsystem, &mut connection, &area, identity)
        .await
        .expect("the exchange answered")
        .expect("the stand-in answered with all four");
    assert_eq!(
        answered,
        HealthAnswer {
            package_version: "0.1.0".to_string(),
            run,
            installation,
            phase: Phase::CommittedUnverified,
        }
    );

    // **The request rode standard input in this document's own frame**, and it
    // is read back off the far end rather than off this crate's account of what
    // it sent.
    let what_it_was_given = std::fs::read(&given).expect("the stand-in wrote it down");
    let back = read_one_request(&mut &what_it_was_given[..]).expect("the frame reads");
    assert_eq!(back.name, THE_HEALTH_ANSWER_REQUEST);
    assert_eq!(back.protocol_version, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS);

    // **And the package area the program would derive is the package area** —
    // the carriage token's own directory, which resolves through the link.
    let invoked_as = std::fs::read_to_string(format!("{}.invoked-as", given.display()))
        .expect("the stand-in wrote its own invoked path down");
    let (directory, name) = invoked_as.rsplit_once('/').expect("it names a directory");
    assert_eq!(name, THE_FIXED_ENTRY_POINT_NAME);
    // **The directory it was invoked from is the carriage token's**, which is
    // what the machine's own program would take as its package area — and
    // that it ran at all is what says the link resolved to the package area,
    // there being nothing at that path otherwise. **The link is gone by now**,
    // so this is a comparison of the recorded text rather than of a path that
    // still resolves.
    assert_eq!(
        directory,
        scratch
            .at(THE_ENTRY_POINTS_CARRIAGE_SITE.link_path_on_the_subsystem)
            .to_string_lossy()
    );
    assert!(!std::fs::read(&given)
        .expect("the stand-in wrote it down")
        .is_empty());

    // **The carriage is taken away when the step ends** — recorded staging under
    // §7.8's discipline, never a forgotten one.
    assert!(
        !scratch
            .at(THE_ENTRY_POINTS_CARRIAGE_SITE.link_path_on_the_subsystem)
            .symlink_metadata()
            .is_ok(),
        "the planted link was left behind"
    );
}

/// **The whole transaction, walked through the surface on a bare machine — the
/// caller `accept_a_release` never had.**
///
/// **A container rental is the kind this runs as**, because it is the kind whose
/// registration is `Rented-server runtime` K11's elevated step: the record is
/// written here and the hook is left to `Server Setup`, which is
/// `Rented-server runtime` K16a's own ending, and no user-level command is
/// issued. **The systemd kind's three commands want a service manager and a
/// login manager, which this desk has neither of**, and that half is unproved
/// here rather than passed over.
///
/// **What is proved.** The claim, the phase record at every phase,
/// `Rented-server runtime` G4's carried-forward write, the staging, B10's
/// at-rest read-back with the mode it earns, `Rented-server runtime` K16's
/// record, `Rented-server runtime` G7's single rename, the health exchange
/// through the carriage, and `Rented-server runtime` G15's removal of the
/// previous package after the exchange succeeds — ending at *verified* with the
/// claim released.
#[tokio::test]
async fn a_verified_release_is_placed_and_committed_through_the_surface() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    a_linux_shaped_far_end!();
    let scratch = Scratch::new("whole");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server, scratch.0.clone()).await;
    let mut connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let area_path = scratch.at("package-area");
    std::fs::create_dir_all(&area_path).expect("the area is made");
    let area = area_path.to_string_lossy().into_owned();
    // The previous version, present so that G15's removal has something to
    // reach.
    let previous_location = area_path.join("staged-0.0.9").to_string_lossy().into_owned();
    std::fs::write(&previous_location, b"the previous").expect("the previous package is there");

    let installation = an_identity(IdentityKind::Installation);
    let attempt = an_identity(IdentityKind::Operation);
    let run = an_identity(IdentityKind::Run);
    // **S4: the placement synthesises where it stages, from the attempt's
    // identity** — the caller no longer hands a path in.
    let staging = the_staging_location_for(&area, &attempt);

    // **Standing in for the elevated step `Server Setup` owns.**
    // `Rented-server runtime` K11 makes installing the hook root's, and K19
    // makes leaving it readable to the login that performs the read-back a
    // second obligation on the same step. **Neither is this performer's**, and
    // without both K15's read-back fails before the commit — which is exactly
    // what this test saw before the hook was put there, and is the right
    // answer rather than a fault.
    let the_hook = b"#!/bin/sh\n# the hook root installs\n".to_vec();
    let the_providers_hook_path = scratch.at("onstart.sh");
    std::fs::write(&the_providers_hook_path, &the_hook)
        .expect("the root step's own act, standing in");

    // **The exchange's own correlation identity is chosen before the request
    // is sent**, which is C1, so the stand-in's answer can be composed against
    // the very request the walk will send.
    let exchange_identity = an_identity(IdentityKind::Operation);
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        exchange_identity.clone(),
        THE_HEALTH_ANSWER_REQUEST,
    );

    let given = scratch.at("what-the-adapter-was-given");
    let frame_file = scratch.at("the-answer-frame");
    std::fs::write(
        &frame_file,
        the_answer_frame(
            &request,
            "0.1.0",
            &run,
            &installation,
            Phase::CommittedUnverified,
        ),
    )
    .expect("the answer frame is written");
    let artifact = a_stand_in_adapter(&given, &frame_file);

    let served = scratch.at("what-hosting-would-serve");
    std::fs::write(&served, &artifact).expect("the file is written");
    let release = a_release(&artifact);
    let (verified, bytes) = verified_and_fetched(&release, &served);

    let mut surface = ThePlacementsSurface::of(&mut subsystem, &mut connection);
    let reached = place_a_verified_release(
        &mut surface,
        WhatThePlacementNeeds {
            package_area: area.clone(),
            attempt: attempt.clone(),
            owner: an_owner(installation.clone(), Route::SetupPlacement),
            installation: installation.clone(),
            verified: &verified,
            artifact: &bytes,
            kind: MachineKind::Container,
            previous: Some(a_previous_version(&previous_location, b"the previous hook")),
            the_root_step_on_a_container: Some(a_root_step_into(&scratch)),
            accepted_work: Vec::new(),
            the_health_exchanges_identity: exchange_identity,
            how_long_the_health_answer_is_waited_for: Duration::ZERO,
            how_long_the_user_manager_is_waited_for: Duration::ZERO,
        },
    )
    .await
    .expect("the surface carried the transaction");

    assert_eq!(reached.reached, WhatThePlacementReached::Verified);
    // **S20 is scoped *"where the server offers it"*, and this server offers
    // it**, so the placement says the ordering held for every record it wrote.
    assert_eq!(
        reached.the_durability_ordering,
        WhetherTheDurabilityOrderingApplied::TheStagedBytesWereMadeDurableFirst
    );

    // **The phase record says so, read back over the real file-transfer
    // server.**
    let WhatARecordHeld::It(after) = read_the_phase_record(&mut subsystem, &area)
        .await
        .expect("the phase record reads")
    else {
        panic!("the phase record did not read back");
    };
    assert_eq!(after.phase, Phase::Verified);
    assert_eq!(after.attempt, attempt);

    // **`Rented-server runtime` G7: the pointer names the staged artifact**, and
    // the value is the link's own rather than a resolved path.
    assert_eq!(
        the_active_pointers_value(&mut subsystem, &area)
            .await
            .expect("the pointer reads"),
        Some(staging.clone())
    );

    // **`Rented-server runtime` G15: the previous package is gone**, and only
    // after the exchange succeeded.
    assert!(!Path::new(&previous_location).exists());

    // **`Rented-server runtime` K16 and K16a: the record is there on a
    // container**, holding the hook's content by value, with the hook itself
    // left to the root step.
    let WhatARecordHeld::It(registration) =
        read_the_boot_registration_record(&mut subsystem, &area)
            .await
            .expect("the record reads")
    else {
        panic!("the boot-registration record did not read back");
    };
    assert_eq!(registration.kind, MachineKind::Container);
    // **The provider's own hook is kept, and Smoozy's part is in it** — step
    // 10c, 2026-09-21. **And the hook on the machine is the record's content**,
    // which is what K15's read-back confirmed before the commit.
    let text = String::from_utf8(registration.content.clone()).expect("the hook is text");
    assert!(text.starts_with("#!/bin/sh\n"));
    assert!(text.ends_with("# the hook root installs\n"));
    assert!(text.contains(
        smoozy_runtime::registration_content::THE_LINE_SMOOZYS_PART_BEGINS_WITH
    ));
    assert_eq!(registration.at_path, the_providers_hook_path.to_string_lossy());
    assert_eq!(
        std::fs::read(&the_providers_hook_path).expect("the hook is there"),
        registration.content
    );
    assert!(
        !scratch.at(".smoozy-carriage/container-hook").exists(),
        "the staging token was left behind"
    );

    // **`Rented-server runtime` G10: the machine has an identity**, written
    // before the pointer moved. It is read the way package inspection reads it.
    let reading = inspect(&mut subsystem, area.as_bytes())
        .await
        .expect("the reading answered");
    assert_eq!(
        reading.value().the_installation_identity,
        smoozy_server_protocol::reconnection::WhatTheStateAreaHeld::AnIdentity(installation)
    );
    assert_eq!(
        reading
            .value()
            .the_active_packages_descriptor
            .as_ref()
            .map(|found| found.requested_package_version.clone()),
        Some("0.1.0".to_string()),
        "package inspection does not read the placement's own descriptor back"
    );

    // **The claim is released at the terminal write**, so the next exclusive
    // create succeeds.
    assert_eq!(
        take_the_claim(
            &mut subsystem,
            &area,
            &an_owner(an_identity(IdentityKind::Installation), Route::AutomaticRepair),
            Phase::Inspecting
        )
        .await
        .expect("the claim answered")
        .0,
        WhatTakingTheClaimReached::TheRulesHalfSays(ClaimOutcome::Acquired)
    );
}

/// **`Rented-server runtime` K15 on a container, the failing half: a root step
/// that left nothing at the hook's path is a failure before the commit, naming
/// the path** — and a copy the login may not make, with no elevation to be had
/// without a password, says so rather than reading back a hook that is not
/// there. Step 10c, 2026-09-21: until then every container placement failed
/// here with the record's path empty, *"could not be read back: : there is
/// nothing at that path"*.
#[tokio::test]
async fn a_container_hook_that_was_not_installed_fails_before_the_commit_and_names_its_path() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    a_linux_shaped_far_end!();
    let scratch = Scratch::new("hook-missing");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server, scratch.0.clone()).await;
    let mut connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let area_path = scratch.at("package-area");
    std::fs::create_dir_all(&area_path).expect("the area is made");
    let area = area_path.to_string_lossy().into_owned();
    let artifact = b"the package, whatever it is".to_vec();
    let served = scratch.at("what-hosting-would-serve");
    std::fs::write(&served, &artifact).expect("the file is written");
    let release = a_release(&artifact);
    let (verified, bytes) = verified_and_fetched(&release, &served);
    let hook = scratch.at("onstart.sh").to_string_lossy().into_owned();

    for (copy, elevated, what_it_says) in [
        // **A copy that runs and puts nothing there**: K15's read-back finds
        // nothing, and says where it looked.
        ("true", None, format!("the registration could not be read back: {hook}: there is nothing at that path")),
        // **A copy the login may not make and nothing to elevate it with.**
        ("false", None, format!("the boot hook could not be installed: {hook}: this login may not write it")),
    ] {
        let _ = std::fs::remove_file(&hook);
        let installation = an_identity(IdentityKind::Installation);
        let mut surface = ThePlacementsSurface::of(&mut subsystem, &mut connection);
        let reached = place_a_verified_release(
            &mut surface,
            WhatThePlacementNeeds {
                package_area: area.clone(),
                attempt: an_identity(IdentityKind::Operation),
                owner: an_owner(installation.clone(), Route::SetupPlacement),
                installation,
                verified: &verified,
                artifact: &bytes,
                kind: MachineKind::Container,
                previous: None,
                the_root_step_on_a_container: Some(
                    smoozy_server_protocol::placement::TheRootStepOnAContainer {
                        the_hooks_path: hook.clone(),
                        the_staging_token_on_the_subsystem: ".smoozy-carriage/container-hook"
                            .to_string(),
                        the_copy: smoozy_server_protocol::placement::TheCommandAndItsElevatedForm {
                            plain: smoozy_server_protocol::transport::CommandString::constant(copy),
                            elevated,
                        },
                        the_hook_made_readable: None,
                        the_directory_made_traversable: None,
                        the_hook_read: None,
                    },
                ),
                accepted_work: Vec::new(),
                the_health_exchanges_identity: an_identity(IdentityKind::Operation),
                how_long_the_health_answer_is_waited_for: Duration::ZERO,
                how_long_the_user_manager_is_waited_for: Duration::ZERO,
            },
        )
        .await
        .expect("the surface carried the transaction");
        let WhatThePlacementReached::FailedBeforeCommit { because } = &reached.reached else {
            panic!("a hook that is not there was not a failure before the commit: {:?}", reached.reached);
        };
        assert!(
            because.starts_with(&what_it_says),
            "it said {because:?}, not {what_it_says:?}"
        );
        assert!(!because.contains(": :"), "an empty path reached the words: {because}");

        // **The record was written first all the same (K16b), with a real path
        // and real content**, and nothing was committed.
        let WhatARecordHeld::It(record) = read_the_boot_registration_record(&mut subsystem, &area)
            .await
            .expect("the record reads")
        else {
            panic!("no boot-registration record");
        };
        assert_eq!(record.at_path, hook);
        assert!(String::from_utf8_lossy(&record.content)
            .contains(smoozy_runtime::registration_content::THE_LINE_SMOOZYS_PART_BEGINS_WITH));
        assert_eq!(
            the_active_pointers_value(&mut subsystem, &area)
                .await
                .expect("the pointer reads"),
            None
        );
    }
}

/// **`Rented-server runtime` G1a's window, against a real file-transfer
/// server: a claim that stands and does not read back is said as that and not
/// as the machine's file server failing.**
///
/// **The window is real and the file's own opening names it.** The exclusive
/// create and the write of the owner's fields are two operations, so a claimant
/// can meet a claim file holding nothing readable — one copy of Smoozy racing
/// another, or one that stopped between the two writes.
///
/// **What it cost before this was answered.** The read failure came back as a
/// subsystem failure, which the performer's own words call the connection or
/// the machine's own file-transfer server failing under the transaction. It was
/// neither, and **a user told a rented machine's file server will not carry the
/// transaction may destroy a rental that is perfectly fine.**
#[tokio::test]
async fn a_claim_that_will_not_read_back_is_said_as_a_claim_and_not_as_a_broken_machine() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("claim-unreadable");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server, scratch.0.clone()).await;
    let connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let area_path = scratch.at("package-area");
    std::fs::create_dir_all(&area_path).expect("the area is made");
    let area = area_path.to_string_lossy().into_owned();

    // **The window itself: the file is there and holds nothing.** This is what
    // a claim looks like between its two writes.
    let claim_path = smoozy_server_protocol::placement::TheClaimsOwnPath::in_the_package_area(&area);
    std::fs::write(&claim_path.0, b"").expect("the half-written claim is there");

    let met = take_the_claim(
        &mut subsystem,
        &area,
        &an_owner(an_identity(IdentityKind::Installation), Route::SetupPlacement),
        Phase::Inspecting,
    )
    .await
    .expect("a claim that stands is an answer rather than a failure");
    assert_eq!(
        met.0,
        WhatTakingTheClaimReached::AClaimStandsWhoseOwnerCouldNotBeRead
    );

    // **It changes no phase, no pointer, no staging path and no health
    // result**, which is the other half of `Rented-server runtime` G1a's own
    // sentence.
    assert!(matches!(
        read_the_phase_record(&mut subsystem, &area)
            .await
            .expect("the read answered"),
        WhatARecordHeld::Nothing
    ));
    assert_eq!(std::fs::read(&claim_path.0).expect("it is still there"), b"");
}

/// **`Rented-server runtime` G17's *committed, unverified* row, run end to end:
/// the retry finishes the standing attempt instead of staging the whole
/// artifact again.**
///
/// **This is the row whose failure path is ordinary.** The machine is permitted
/// to stand at *committed, unverified* — the pointer renamed, the connection
/// then dropped — and the same request is re-sent under its own identity, which
/// is exactly what idempotency invites. **What stood here before wrote a fresh
/// attempt's first record over the standing one and swept the staging location,
/// which at this phase is the file the active pointer names.**
///
/// **How the test tells the two apart.** The material already staged is a
/// stand-in that writes its own name down, and the request carries a different
/// one. **If the attempt were started over, the request's artifact would be the
/// one the health exchange ran**; because the standing attempt is continued, the
/// staged one runs and the request's is never written to the machine at all.
#[tokio::test]
async fn a_retry_at_committed_unverified_finishes_the_standing_attempt_rather_than_starting_over() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    a_linux_shaped_far_end!();
    let scratch = Scratch::new("retry-committed");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server, scratch.0.clone()).await;
    let mut connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let area_path = scratch.at("package-area");
    std::fs::create_dir_all(&area_path).expect("the area is made");
    let area = area_path.to_string_lossy().into_owned();
    let staging = area_path.join("staged-0.1.0").to_string_lossy().into_owned();

    let installation = an_identity(IdentityKind::Installation);
    let attempt = an_identity(IdentityKind::Operation);
    let run = an_identity(IdentityKind::Run);
    let exchange_identity = an_identity(IdentityKind::Operation);
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        exchange_identity.clone(),
        THE_HEALTH_ANSWER_REQUEST,
    );
    let frame_file = scratch.at("the-answer-frame");
    std::fs::write(
        &frame_file,
        the_answer_frame(
            &request,
            "0.1.0",
            &run,
            &installation,
            Phase::CommittedUnverified,
        ),
    )
    .expect("the answer frame is written");

    // The two stand-ins: the one already on the machine, and the one this
    // request carries. Each writes down that it was the one that ran.
    let the_staged_one_ran = scratch.at("the-staged-one-ran");
    let the_requests_one_ran = scratch.at("the-requests-one-ran");
    let staged_artifact = a_stand_in_adapter(&the_staged_one_ran, &frame_file);
    let requests_artifact = a_stand_in_adapter(&the_requests_one_ran, &frame_file);

    // **The machine as the interrupted attempt left it**: the artifact staged
    // and runnable, the pointer already renamed onto it, and the record at
    // *committed, unverified* under this attempt's own identity.
    std::fs::write(&staging, &staged_artifact).expect("the staged artifact is there");
    // **Making the staged artifact runnable is a rule about a rented machine,
    // which is always Linux** — the machine's own program is built for that one
    // target and no other — so an execute bit has no Windows meaning to keep,
    // and `a_linux_shaped_far_end!()` above has already returned before this
    // line is reached on Windows.
    //
    // **The guard is here so that the file compiles on Windows, not so that the
    // test behaves differently there**, and it is the same pairing the health
    // exchange's own arrangement above carries for the same reason: Windows has
    // no `std::os::unix`, and one unguarded call to it stops the whole crate's
    // test binary from building. That is what it did on hosted run
    // `34697517090`, taking the three portable tests in this same file down
    // with it. **The runtime skip and this compile guard are two mechanisms for
    // one fact and each does what the other cannot**, so the two move together.
    //
    // **What Windows therefore does not check is this whole test**, and with it
    // `Rented-server runtime` G17's *committed, unverified* row — that a retry
    // at that phase finishes the standing attempt rather than starting over.
    // **Windows never checked it**: the skip predates this guard and the guard
    // takes nothing away.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&staging)
            .expect("it is there")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&staging, permissions).expect("it is runnable");
    }
    smoozy_server_protocol::placement::rename_the_pointer_to(&mut subsystem, &area, &staging)
        .await
        .expect("the pointer names the staged artifact");

    let served = scratch.at("what-hosting-would-serve");
    std::fs::write(&served, &requests_artifact).expect("the file is written");
    let release = a_release(&requests_artifact);
    let (verified, bytes) = verified_and_fetched(&release, &served);

    let mut standing = smoozy_runtime::package::phase::a_fresh_attempts_first_write(
        attempt.clone(),
        smoozy_server_protocol::placement::performing::the_phase_records_descriptor_for(
            &verified.the_release_descriptors_values(),
        ),
        None,
        MachineKind::Container,
        staging.clone(),
    );
    standing.phase = Phase::CommittedUnverified;
    standing.owner = Some(an_owner(installation.clone(), Route::SetupPlacement));
    write_the_phase_record(&mut subsystem, &area, &standing)
        .await
        .expect("the standing record is written");

    let mut surface = ThePlacementsSurface::of(&mut subsystem, &mut connection);
    let reached = place_a_verified_release(
        &mut surface,
        WhatThePlacementNeeds {
            package_area: area.clone(),
            attempt: attempt.clone(),
            owner: an_owner(installation.clone(), Route::AutomaticRepair),
            installation: installation.clone(),
            verified: &verified,
            artifact: &bytes,
            kind: MachineKind::Container,
            previous: None,
            the_root_step_on_a_container: Some(a_root_step_into(&scratch)),
            accepted_work: Vec::new(),
            the_health_exchanges_identity: exchange_identity,
            how_long_the_health_answer_is_waited_for: Duration::ZERO,
            how_long_the_user_manager_is_waited_for: Duration::ZERO,
        },
    )
    .await
    .expect("the surface carried the transaction");

    assert_eq!(reached.reached, WhatThePlacementReached::Verified);

    // **The staged artifact is the one that ran**, so nothing was swept and
    // nothing was staged again.
    assert!(
        the_staged_one_ran.exists(),
        "the standing attempt's own artifact was not the one the exchange reached"
    );
    assert!(
        !the_requests_one_ran.exists(),
        "the attempt was started over: the request's artifact was staged and run"
    );
    assert_eq!(
        std::fs::read(&staging).expect("the staged artifact is still there"),
        staged_artifact,
        "the staging location was written over"
    );

    // **The record is the standing attempt's, carried to *verified***.
    let WhatARecordHeld::It(after) = read_the_phase_record(&mut subsystem, &area)
        .await
        .expect("the phase record reads")
    else {
        panic!("the phase record did not read back");
    };
    assert_eq!(after.phase, Phase::Verified);
    assert_eq!(after.attempt, attempt);
}

/// **`Rented-server runtime` G17's *verified* row, and G1b's claim with it: a
/// retry that answers from the record gives the claim back.**
///
/// **What it cost before.** The arm returned the recorded outcome and returned
/// without releasing, and **nothing anywhere else removes a claim**: the bounded
/// sweep does not reach the claim path, the release is reached only by a caller
/// that acquired, and G18 refuses to judge a claim on age. A licence covers two
/// computers, so the user's second computer would read the claim, be told a
/// package transaction is in progress, and **never be able to set that machine
/// up, with no control anywhere in Smoozy to clear it.**
#[tokio::test]
async fn a_retry_that_answers_from_a_verified_record_gives_the_claim_back() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("retry-verified");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server, scratch.0.clone()).await;
    let mut connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let area_path = scratch.at("package-area");
    std::fs::create_dir_all(&area_path).expect("the area is made");
    let area = area_path.to_string_lossy().into_owned();
    let staging = area_path.join("staged-0.1.0").to_string_lossy().into_owned();

    let installation = an_identity(IdentityKind::Installation);
    let attempt = an_identity(IdentityKind::Operation);

    let artifact = b"the package, whatever it is".to_vec();
    let served = scratch.at("what-hosting-would-serve");
    std::fs::write(&served, &artifact).expect("the file is written");
    let release = a_release(&artifact);
    let (verified, bytes) = verified_and_fetched(&release, &served);

    let mut standing = smoozy_runtime::package::phase::a_fresh_attempts_first_write(
        attempt.clone(),
        smoozy_server_protocol::placement::performing::the_phase_records_descriptor_for(
            &verified.the_release_descriptors_values(),
        ),
        None,
        MachineKind::Container,
        staging.clone(),
    );
    standing.phase = Phase::Verified;
    standing.owner = Some(an_owner(installation.clone(), Route::SetupPlacement));
    write_the_phase_record(&mut subsystem, &area, &standing)
        .await
        .expect("the standing record is written");

    let mut surface = ThePlacementsSurface::of(&mut subsystem, &mut connection);
    let reached = place_a_verified_release(
        &mut surface,
        WhatThePlacementNeeds {
            package_area: area.clone(),
            attempt: attempt.clone(),
            owner: an_owner(installation.clone(), Route::AutomaticRepair),
            installation: installation.clone(),
            verified: &verified,
            artifact: &bytes,
            kind: MachineKind::Container,
            previous: None,
            the_root_step_on_a_container: Some(a_root_step_into(&scratch)),
            accepted_work: Vec::new(),
            the_health_exchanges_identity: an_identity(IdentityKind::Operation),
            how_long_the_health_answer_is_waited_for: Duration::ZERO,
            how_long_the_user_manager_is_waited_for: Duration::ZERO,
        },
    )
    .await
    .expect("the retry answered");
    assert_eq!(reached.reached, WhatThePlacementReached::Verified);

    // **The claim is gone**, which the exclusive create is what says.
    let claim_path = smoozy_server_protocol::placement::TheClaimsOwnPath::in_the_package_area(&area);
    assert!(
        !Path::new(&claim_path.0).exists(),
        "the retry took the claim and kept it for ever"
    );
}

/// **`Rented-server runtime` G18's *deferred* carve-out, run end to end: a
/// request under another identity is answered with the deferred phase and acts
/// on nothing.**
///
/// **This is the sharpest branch of the rule the code never asked.** A render is
/// running, so the commit was deferred and the staged package is sitting on the
/// machine; automatic repair fires with its own identity, or the user presses
/// the fallback re-run. **What stood here wrote a fresh attempt over the
/// deferred record and swept the staging location — deleting the very thing the
/// deferral was protecting** — then sent the whole package up the user's own
/// line again, and only discovered at the end that the commit was still blocked.
#[tokio::test]
async fn another_identity_meeting_a_deferred_record_acts_on_nothing_and_keeps_the_staging() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("deferred");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server, scratch.0.clone()).await;
    let mut connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let area_path = scratch.at("package-area");
    std::fs::create_dir_all(&area_path).expect("the area is made");
    let area = area_path.to_string_lossy().into_owned();
    let staging = area_path.join("staged-0.1.0").to_string_lossy().into_owned();
    std::fs::write(&staging, b"the staged package the deferral is keeping")
        .expect("the staged material is there");

    let installation = an_identity(IdentityKind::Installation);
    let the_deferred_attempt = an_identity(IdentityKind::Operation);

    let artifact = b"the package, whatever it is".to_vec();
    let served = scratch.at("what-hosting-would-serve");
    std::fs::write(&served, &artifact).expect("the file is written");
    let release = a_release(&artifact);
    let (verified, bytes) = verified_and_fetched(&release, &served);

    let mut standing = smoozy_runtime::package::phase::a_fresh_attempts_first_write(
        the_deferred_attempt.clone(),
        smoozy_server_protocol::placement::performing::the_phase_records_descriptor_for(
            &verified.the_release_descriptors_values(),
        ),
        None,
        MachineKind::Container,
        staging.clone(),
    );
    standing.phase = Phase::Deferred;
    standing.deferred_because = Some("a render is running on the current package".to_string());
    standing.owner = Some(an_owner(installation.clone(), Route::SetupPlacement));
    write_the_phase_record(&mut subsystem, &area, &standing)
        .await
        .expect("the standing record is written");

    let mut surface = ThePlacementsSurface::of(&mut subsystem, &mut connection);
    let reached = place_a_verified_release(
        &mut surface,
        WhatThePlacementNeeds {
            package_area: area.clone(),
            // **A different operation identity**, which is what automatic
            // repair and the fallback re-run each carry.
            attempt: an_identity(IdentityKind::Operation),
            owner: an_owner(installation.clone(), Route::AutomaticRepair),
            installation: installation.clone(),
            verified: &verified,
            artifact: &bytes,
            kind: MachineKind::Container,
            previous: None,
            the_root_step_on_a_container: Some(a_root_step_into(&scratch)),
            accepted_work: Vec::new(),
            the_health_exchanges_identity: an_identity(IdentityKind::Operation),
            how_long_the_health_answer_is_waited_for: Duration::ZERO,
            how_long_the_user_manager_is_waited_for: Duration::ZERO,
        },
    )
    .await
    .expect("the request answered");

    assert!(
        matches!(reached.reached, WhatThePlacementReached::Deferred { .. }),
        "a different identity was not answered with the deferred phase: {:?}",
        reached.reached
    );

    // **`Rented-server runtime` G22: the staged material is kept.**
    assert_eq!(
        std::fs::read(&staging).expect("the staged material is still there"),
        b"the staged package the deferral is keeping".to_vec(),
        "the staging the deferral was protecting was swept"
    );

    // **The record is untouched: still the deferred attempt's, still
    // deferred.**
    let WhatARecordHeld::It(after) = read_the_phase_record(&mut subsystem, &area)
        .await
        .expect("the phase record reads")
    else {
        panic!("the phase record did not read back");
    };
    assert_eq!(after.phase, Phase::Deferred);
    assert_eq!(after.attempt, the_deferred_attempt);

    // **And the claim this request created is given back**, so a deferral does
    // not leave a machine claimed for ever.
    let claim_path = smoozy_server_protocol::placement::TheClaimsOwnPath::in_the_package_area(&area);
    assert!(
        !Path::new(&claim_path.0).exists(),
        "the request kept the claim it took"
    );
}

// ------------------------------------------ the real program, placed for real

/// **The machine's own program, placed by the real placement and started
/// through the pointer the commit made — the test step 10c piece 12 found
/// missing.**
///
/// **Why these exist.** Every test above stages a stand-in or stops before the
/// placed program is run, so none of them ever met a pointer naming nothing.
/// **On the first rental Smoozy deleted its own program in the act that
/// committed it** — the artifact had been staged at the pointer's own staging
/// name, so the pointer's switch removed it — and piece 9's loopback run had
/// already recorded *"the stand-in logged nothing on any run, so the placed
/// artifact was never invoked"* without anybody chasing it. **A test that does
/// not run what it placed cannot see that**, and the second thing these found
/// is the same kind: the placement never recorded `Rented-server runtime` Z2's
/// endpoint path, which the program refuses to start without, and every test
/// that ran the program had written that record by hand.
///
/// **What the far end is.** The loopback machine above, with its home in a
/// folder a unit file can name, and two of the machine's own tools stood in:
/// a service manager whose start runs the unit's own `ExecStart=` line — which
/// is the pointer and the supervisor word, composed by the real registration —
/// and a login manager that grants lingering. **Nothing else is stood in**: the
/// artifact is the real `smoozy-server` this workspace built, verified against
/// a release this test signed, staged, read back, made runnable, committed,
/// started and asked over the real carriage, through the real adapter, of the
/// real supervisor.
///
/// **Where the folder is, and why not the temporary area.** A unit file's
/// command line splits on whitespace, so the registration refuses any path
/// with a space in it — and this workspace's own path has one. The system's
/// temporary area is not used at all. So the folder is the login's own cache
/// folder, made for the test and removed after it, and a home whose path the
/// registration would refuse is said out loud and the test stops.
#[cfg(unix)]
mod the_real_program {
    use super::*;
    use smoozy_runtime::the_program::{the_active_package_pointer_at, THE_SUPERVISOR_WORD};
    use std::os::unix::fs::PermissionsExt;

    /// **How long one placement is given here**: the staging of the whole
    /// program, its read-back, the commit, the start and the health exchange's
    /// own minute. A placement at the desk takes about three seconds.
    const HOW_LONG_A_PLACEMENT_IS_GIVEN: Duration = Duration::from_secs(180);

    /// **How long one reading of the machine is given here**: a connection, a
    /// record read back, or one asking of the supervisor.
    const HOW_LONG_ONE_READING_IS_GIVEN: Duration = Duration::from_secs(30);

    /// **Every wait on the machine in this module has its own deadline, and
    /// missing it fails the test with what was being waited for.** Added
    /// 2026-09-21: hosted runs `35595657437` and `35601023619` sat three and a
    /// half hours in these two tests, in a placement that could never finish,
    /// and said nothing. A panic here ends the test the ordinary way, so the
    /// machine below is dropped and every supervisor it started is killed. The
    /// file's own [`TheTestsDeadline`] stands behind this and is not what is
    /// relied on.
    async fn within<T>(
        what: String,
        limit: Duration,
        doing: impl std::future::Future<Output = T>,
    ) -> T {
        match tokio::time::timeout(limit, doing).await {
            Ok(done) => done,
            Err(_) => panic!("DEADLINE MISSED: {what}, still waiting after {limit:?}"),
        }
    }

    /// **Where the workspace built the machine's program**, beside this test's
    /// own binary. A workspace run builds every package's programs before it
    /// runs any test, so there it is always this build's.
    fn the_machines_own_program() -> Option<PathBuf> {
        let here = std::env::current_exe().ok()?;
        let built = here.parent()?.parent()?.join(THE_FIXED_ENTRY_POINT_NAME);
        built.is_file().then_some(built)
    }

    /// A service manager this desk has none of. **Its start runs the unit's own
    /// `ExecStart=` line**, split on whitespace the way the unit's own manual
    /// says that line is, in the background with its output kept — and it
    /// refuses the start where the test has asked it to.
    ///
    /// **Added 2026-09-21, step 10c: a login with no session.** Where the test
    /// has asked for one (`no-session`), every user-level command fails the way
    /// Sesterce's did — *"Failed to connect to bus: No such file or
    /// directory"* — until lingering has been granted and the login manager's
    /// queued start of the user's manager has landed (`manager-started`). The
    /// enable can also be refused on its own (`refuse-the-enable`). **Every
    /// call is written down in order** (`what-was-asked`), so a test can read
    /// the order the placement took.
    const THE_SERVICE_MANAGER: &str = "#!/bin/sh\n\
        # Stands in for a user service manager. Nothing here is Smoozy's.\n\
        echo \"systemctl $*\" >> \"$HOME/what-was-asked\"\n\
        if [ -f \"$HOME/no-session\" ] && [ ! -f \"$HOME/manager-started\" ]; then\n\
          echo 'Failed to connect to bus: No such file or directory' >&2\n\
          exit 1\n\
        fi\n\
        case \"$*\" in\n\
          *\"--user show-environment\"*) echo 'PATH=/usr/bin'; exit 0 ;;\n\
          *\"--user enable\"*)\n\
            if [ -f \"$HOME/refuse-the-enable\" ]; then\n\
              echo 'Failed to enable unit: Unit file smoozy-supervisor.service does not exist.' >&2\n\
              exit 1\n\
            fi\n\
            exit 0 ;;\n\
          *\"--user start\"*|*\"--user restart\"*)\n\
            if [ -f \"$HOME/refuse-the-start\" ]; then\n\
              echo 'Job for smoozy-supervisor.service failed because the control process exited with error code.' >&2\n\
              exit 1\n\
            fi\n\
            case \"$*\" in *restart*) for p in $(cat \"$HOME/supervisor.pids\" 2>/dev/null); do kill $p 2>/dev/null; done ;; esac\n\
            line=$(grep '^ExecStart=' \"$HOME/.config/systemd/user/smoozy-supervisor.service\")\n\
            command=${line#ExecStart=}\n\
            echo \"$command\" >> \"$HOME/what-was-started\"\n\
            nohup $command >> \"$HOME/supervisor.log\" 2>&1 < /dev/null &\n\
            echo $! >> \"$HOME/supervisor.pids\"\n\
            exit 0 ;;\n\
        esac\n\
        exit 0\n";

    /// A login manager this desk has none of, answering as one that granted
    /// lingering.
    ///
    /// **Its grant starts the user's service manager the way systemd's own
    /// source says the real one does**: queued, and landing a moment later
    /// (`manager-started`, three seconds after the grant), which is what makes the
    /// placement's wait for the manager a thing a test can see.
    const THE_LOGIN_MANAGER: &str = "#!/bin/sh\n\
        echo \"loginctl $*\" >> \"$HOME/what-was-asked\"\n\
        case \"$*\" in\n\
          *\"enable-linger\"*)\n\
            touch \"$HOME/lingering\"\n\
            ( sleep 3; touch \"$HOME/manager-started\" ) </dev/null >/dev/null 2>&1 &\n\
            ;;\n\
          *\"show-user\"*)\n\
            if [ -f \"$HOME/lingering\" ]; then echo 'Linger=yes'; else echo 'Linger=no'; fi\n\
            echo 'State=active' ;;\n\
        esac\n\
        exit 0\n";

    /// **`setsid`, which this desk has none of**: it writes down the keeper it
    /// is about to become, so the test can end it, and then makes a new session
    /// the way util-linux's own does (`setsid(2)`, through the desk's `perl`).
    const THE_SESSION_MAKER: &str = "#!/bin/sh\n\
        # Stands in for util-linux's setsid. Nothing here is Smoozy's.\n\
        echo $$ >> \"$HOME/keeper.pids\"\n\
        exec perl -MPOSIX -e 'POSIX::setsid(); exec @ARGV or exit 127' \"$@\"\n";

    /// **`flock -n <number>`, which this desk has none of**: an exclusive,
    /// non-blocking `flock(2)` on the descriptor named last, through the desk's
    /// `perl` — **the same call on the same open file description** util-linux's
    /// own makes, so a lock the keeper holds is really held.
    const THE_LOCK_TAKER: &str = "#!/bin/sh\n\
        # Stands in for util-linux's flock, in its descriptor form. Nothing here is Smoozy's.\n\
        for last; do :; done\n\
        exec perl -MFcntl=:flock -e 'open(my $h, \">&=\", $ARGV[0]) or exit 1; exit(flock($h, LOCK_EX|LOCK_NB) ? 0 : 1)' \"$last\"\n";

    /// The machine's home, with its two tools, and every supervisor the service
    /// manager started — which are killed when it goes.
    struct AMachineThatStartsWhatItIsTold {
        home: PathBuf,
    }

    impl AMachineThatStartsWhatItIsTold {
        fn made() -> Result<AMachineThatStartsWhatItIsTold, String> {
            let login_home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .ok_or("this desk names no home")?;
            let caches = if cfg!(target_os = "macos") {
                login_home.join("Library").join("Caches")
            } else {
                std::env::var_os("XDG_CACHE_HOME")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| login_home.join(".cache"))
            };
            let home = caches.join(format!(
                "smoozy-placement-{}",
                &an_identity(IdentityKind::Operation).as_text()[..12]
            ));
            let text = home.to_str().ok_or("the folder's path is not text")?;
            if !text
                .chars()
                .all(|held| held.is_ascii_alphanumeric() || matches!(held, '/' | '-' | '_' | '.'))
            {
                return Err(format!(
                    "{text} holds a character a unit file's command line refuses"
                ));
            }
            std::fs::create_dir_all(home.join("tools")).map_err(|why| why.to_string())?;
            for (name, text) in [
                ("systemctl", THE_SERVICE_MANAGER),
                ("loginctl", THE_LOGIN_MANAGER),
                ("setsid", THE_SESSION_MAKER),
                ("flock", THE_LOCK_TAKER),
            ] {
                let at = home.join("tools").join(name);
                std::fs::write(&at, text).map_err(|why| why.to_string())?;
                std::fs::set_permissions(&at, std::fs::Permissions::from_mode(0o755))
                    .map_err(|why| why.to_string())?;
            }
            Ok(AMachineThatStartsWhatItIsTold { home })
        }

        fn tools(&self) -> PathBuf {
            self.home.join("tools")
        }

        /// **A flag file in the machine's home**, set or cleared.
        fn flag(&self, name: &str, set: bool) {
            let at = self.home.join(name);
            if set {
                std::fs::write(at, b"").expect("the flag is written");
            } else {
                std::fs::remove_file(at).ok();
            }
        }

        /// Every call the two stand-in managers answered, in order.
        fn what_was_asked(&self) -> Vec<String> {
            String::from_utf8_lossy(
                &std::fs::read(self.home.join("what-was-asked")).unwrap_or_default(),
            )
            .lines()
            .map(str::to_string)
            .collect()
        }

        fn refuse_the_next_start(&self, refuse: bool) {
            let at = self.home.join("refuse-the-start");
            if refuse {
                std::fs::write(at, b"").expect("the flag is written");
            } else {
                std::fs::remove_file(at).ok();
            }
        }

        /// What the started supervisor said, for a failure's own message.
        fn what_the_supervisor_said(&self) -> String {
            String::from_utf8_lossy(
                &std::fs::read(self.home.join("supervisor.log")).unwrap_or_default(),
            )
            .into_owned()
        }

        fn what_the_service_manager_started(&self) -> String {
            String::from_utf8_lossy(
                &std::fs::read(self.home.join("what-was-started")).unwrap_or_default(),
            )
            .into_owned()
        }
    }

    impl Drop for AMachineThatStartsWhatItIsTold {
        fn drop(&mut self) {
            let pids = std::fs::read_to_string(self.home.join("supervisor.pids"))
                .unwrap_or_default();
            for pid in pids.split_whitespace() {
                // A supervisor that already ended is nothing to kill, and
                // the tool's complaint about it is not this test's to show.
                let _ = std::process::Command::new("kill")
                    .arg(pid)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
            // **Every keeper a container placement started, with its whole
            // session** — the keeper's loop, and the supervisor it started — and
            // then anything still running out of this machine's home, which a
            // keeper ended first could otherwise have started again.
            let keepers = std::fs::read_to_string(self.home.join("keeper.pids"))
                .unwrap_or_default();
            for pid in keepers.split_whitespace() {
                let _ = std::process::Command::new("kill")
                    .args(["-TERM", "--", &format!("-{pid}")])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
            if !keepers.is_empty() {
                let _ = std::process::Command::new("pkill")
                    .args(["-TERM", "-f", &self.home.to_string_lossy()])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
            let _ = std::fs::remove_dir_all(&self.home);
        }
    }

    /// Everything one placement against that machine needs, made once per test.
    struct TheDesk {
        machine: AMachineThatStartsWhatItIsTold,
        _keys: Scratch,
        _running: RunningMachine,
        connection: Connection,
        subsystem: Subsystem<russh::ChannelStream<russh::client::Msg>>,
        area: String,
        verified: AVerifiedRelease,
        bytes: Vec<u8>,
    }

    /// `None` is a desk that cannot run this, said out loud by the caller.
    async fn the_desk(sftp_server: PathBuf) -> Result<TheDesk, String> {
        let program = the_machines_own_program().ok_or(
            "the machine's own program is not built beside this test; a workspace run builds it",
        )?;
        let machine = AMachineThatStartsWhatItIsTold::made()?;
        let keys = Scratch(machine.home.join("desk"));
        std::fs::create_dir_all(&keys.0).map_err(|why| why.to_string())?;
        let host_key = keys.a_key("host");
        let user_key = keys.a_key("user");
        let running = a_machine_with_its_own_tools(
            &host_key,
            sftp_server,
            machine.home.clone(),
            Some(machine.tools()),
        )
        .await;
        let connection = within(
            "the loopback machine taking the connection".to_string(),
            HOW_LONG_ONE_READING_IS_GIVEN,
            a_connection(&running, &user_key),
        )
        .await;
        let subsystem = within(
            "the loopback machine serving the file-transfer subsystem".to_string(),
            HOW_LONG_ONE_READING_IS_GIVEN,
            a_subsystem(&connection),
        )
        .await;

        let area_path = machine.home.join("package");
        std::fs::create_dir_all(&area_path).map_err(|why| why.to_string())?;
        let area = area_path.to_string_lossy().into_owned();

        let the_program = std::fs::read(&program).map_err(|why| why.to_string())?;
        let served = keys.at("what-hosting-would-serve");
        std::fs::write(&served, &the_program).map_err(|why| why.to_string())?;
        // **The program answers with the version it was built as**, which is
        // the workspace's, so the release says the same.
        let release = a_release_at(&the_program, env!("CARGO_PKG_VERSION"));
        let (verified, bytes) = verified_and_fetched(&release, &served);

        Ok(TheDesk {
            machine,
            _keys: keys,
            _running: running,
            connection,
            subsystem,
            area,
            verified,
            bytes,
        })
    }

    impl TheDesk {
        /// **One setup's placement**, the way the Set up screen's own driver
        /// makes it: a fresh attempt and, because nothing on a machine never set
        /// up names one, a fresh installation.
        async fn one_placement(
            &mut self,
            attempt: &Identity,
            installation: &Identity,
        ) -> WhatThePlacementReached {
            let chunk = self.subsystem.announcement().limits.write_chunk().max(1);
            let what = format!(
                "the placement of the real program: {} bytes staged in {} writes of at most {} \
                 bytes, read back, committed, started through the pointer and asked for its \
                 health answer",
                self.bytes.len(),
                self.bytes.len().div_ceil(chunk),
                chunk
            );
            let mut surface = ThePlacementsSurface::of(&mut self.subsystem, &mut self.connection);
            let placing = place_a_verified_release(
                &mut surface,
                WhatThePlacementNeeds {
                    package_area: self.area.clone(),
                    attempt: attempt.clone(),
                    owner: an_owner(installation.clone(), Route::SetupPlacement),
                    installation: installation.clone(),
                    verified: &self.verified,
                    artifact: &self.bytes,
                    kind: MachineKind::Systemd,
                    previous: None,
                    the_root_step_on_a_container: None,
                    accepted_work: Vec::new(),
                    the_health_exchanges_identity: an_identity(IdentityKind::Operation),
                    how_long_the_health_answer_is_waited_for: Duration::from_secs(60),
                    how_long_the_user_manager_is_waited_for: Duration::from_secs(60),
                },
            );
            within(what, HOW_LONG_A_PLACEMENT_IS_GIVEN, placing)
                .await
                .expect("the surface carried the transaction")
                .reached
        }

        /// **One setup's placement on a container**: `Rented-server runtime` K11's
        /// root step into this machine's own home, and G25's start step starting
        /// S5's keeper through the desk's stand-in `setsid` and `flock`.
        async fn one_placement_on_a_container(
            &mut self,
            attempt: &Identity,
            installation: &Identity,
        ) -> WhatThePlacementReached {
            let hook = self.machine.home.join("onstart.sh").to_string_lossy().into_owned();
            let copy: &'static str = Box::leak(
                format!("cp ~/.smoozy-carriage/container-hook {hook}").into_boxed_str(),
            );
            let step = smoozy_server_protocol::placement::TheRootStepOnAContainer {
                the_hooks_path: hook,
                the_staging_token_on_the_subsystem: ".smoozy-carriage/container-hook".to_string(),
                the_copy: smoozy_server_protocol::placement::TheCommandAndItsElevatedForm {
                    plain: smoozy_server_protocol::transport::CommandString::constant(copy),
                    elevated: None,
                },
                the_hook_made_readable: None,
                the_directory_made_traversable: None,
                the_hook_read: None,
            };
            let mut surface = ThePlacementsSurface::of(&mut self.subsystem, &mut self.connection);
            let placing = place_a_verified_release(
                &mut surface,
                WhatThePlacementNeeds {
                    package_area: self.area.clone(),
                    attempt: attempt.clone(),
                    owner: an_owner(installation.clone(), Route::SetupPlacement),
                    installation: installation.clone(),
                    verified: &self.verified,
                    artifact: &self.bytes,
                    kind: MachineKind::Container,
                    previous: None,
                    the_root_step_on_a_container: Some(step),
                    accepted_work: Vec::new(),
                    the_health_exchanges_identity: an_identity(IdentityKind::Operation),
                    how_long_the_health_answer_is_waited_for: Duration::from_secs(60),
                    how_long_the_user_manager_is_waited_for: Duration::from_secs(60),
                },
            );
            within(
                "the placement of the real program on a container".to_string(),
                HOW_LONG_A_PLACEMENT_IS_GIVEN,
                placing,
            )
            .await
            .expect("the surface carried the transaction")
            .reached
        }

        /// How many supervisors are running out of this machine's package
        /// area — the pointer's own path with the supervisor's word after it,
        /// which is what both the keeper and the unit start.
        fn supervisors_running(&self) -> usize {
            let said = std::process::Command::new("pgrep")
                .args([
                    "-f",
                    &format!(
                        "{} {THE_SUPERVISOR_WORD}",
                        the_active_package_pointer_at(&self.area)
                    ),
                ])
                .output()
                .expect("pgrep runs");
            String::from_utf8_lossy(&said.stdout).split_whitespace().count()
        }

        /// How many keeper processes are running out of this machine's home.
        fn keepers_running(&self) -> usize {
            let said = std::process::Command::new("pgrep")
                .args([
                    "-f",
                    &format!("sh {}/.smoozy-carriage/keeper", self.machine.home.to_string_lossy()),
                ])
                .output()
                .expect("pgrep runs");
            String::from_utf8_lossy(&said.stdout).split_whitespace().count()
        }

        /// **The placed program asked, again, after the placement is over** —
        /// through the same carriage and adapter, of whatever supervisor is
        /// running.
        async fn asked_now(&mut self) -> Option<HealthAnswer> {
            within(
                "the placed supervisor's health answer, asked after the placement".to_string(),
                HOW_LONG_ONE_READING_IS_GIVEN,
                ask_for_the_health_answer(
                    &mut self.subsystem,
                    &mut self.connection,
                    &self.area,
                    an_identity(IdentityKind::Operation),
                ),
            )
            .await
            .expect("the surface carried the question")
        }

        /// **Outcome 2, read off the machine**: the pointer names a file that
        /// exists, it is this attempt's, and its bytes are the verified
        /// program's.
        async fn the_pointer_names_the_verified_program(&mut self, attempt: &Identity) {
            let named = within(
                "the active pointer read back".to_string(),
                HOW_LONG_ONE_READING_IS_GIVEN,
                the_active_pointers_value(&mut self.subsystem, &self.area),
            )
            .await
            .expect("the pointer reads")
            .expect("there is a pointer");
            assert_eq!(named, the_staging_location_for(&self.area, attempt));
            assert_ne!(
                named,
                format!("{}-being-written", the_active_package_pointer_at(&self.area)),
                "the program is staged at the pointer's own staging name again"
            );
            let placed = std::fs::read(&named).unwrap_or_else(|why| {
                panic!("the pointer names {named}, and there is nothing there: {why}")
            });
            assert!(
                placed == self.bytes,
                "the file the pointer names is not the verified program"
            );
        }
    }

    macro_rules! the_desk_or_say_so {
        ($sftp:expr) => {
            match the_desk($sftp).await {
                Ok(desk) => desk,
                Err(why) => {
                    println!("SKIPPED: {why}. The placed program is unproved here.");
                    return;
                }
            }
        };
    }

    /// **Outcomes 2 and 3: the placed program survives the commit, is started
    /// through the pointer the commit made, and answers.**
    ///
    /// `Rented-server runtime` G13 makes *verified* reachable only by a health
    /// answer whose package version, installation and phase agree with the
    /// phase record — **so a verified placement here is the real supervisor
    /// having answered**, and nothing else in this test can produce one.
    #[tokio::test]
    async fn the_real_program_placed_by_the_real_placement_starts_through_the_pointer_and_answers() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        a_linux_shaped_far_end!();
        let mut desk = the_desk_or_say_so!(sftp_server);

        let attempt = an_identity(IdentityKind::Operation);
        let installation = an_identity(IdentityKind::Installation);
        let reached = desk.one_placement(&attempt, &installation).await;
        assert_eq!(
            reached,
            WhatThePlacementReached::Verified,
            "the placed program did not answer. The service manager started: {:?}. \
             The supervisor said: {}",
            desk.machine.what_the_service_manager_started(),
            desk.machine.what_the_supervisor_said()
        );

        desk.the_pointer_names_the_verified_program(&attempt).await;

        // **Started through the pointer**: what the service manager ran is the
        // unit's own line, and that line is the pointer and the supervisor word.
        assert_eq!(
            desk.machine.what_the_service_manager_started().trim(),
            format!(
                "{} {THE_SUPERVISOR_WORD}",
                the_active_package_pointer_at(&desk.area)
            )
        );

        // **And it is still answering once the placement is over**, with the
        // installation this placement wrote.
        let answer = desk
            .asked_now()
            .await
            .expect("the placed supervisor answers after the placement");
        assert_eq!(answer.installation, installation);
        assert_eq!(answer.package_version, env!("CARGO_PKG_VERSION"));

        // `Rented-server runtime` Z2: the endpoint's path is the one the
        // placement recorded, under the package area.
        assert_eq!(
            std::fs::read_to_string(
                smoozy_server_protocol::placement::the_record_at(
                    &desk.area,
                    smoozy_runtime::the_program::records::THE_ENDPOINTS_RECORDED_PATH
                )
            )
            .expect("the endpoint's path is recorded"),
            smoozy_runtime::the_program::the_endpoints_path_under(&desk.area)
        );
    }

    /// **Outcome 5, both ends: a commit whose start is refused leaves the
    /// machine as `Rented-server runtime` says a failed commit leaves it, and
    /// the next setup does what that document says with it.**
    ///
    /// **The first end.** G25: a restarter that produces no answering
    /// supervisor is G13's *did not answer*; G14 restores; G6 makes a first
    /// placement's restore *no active pointer*; G5 and §13 R1 sweep what the
    /// rolled-back attempt staged; and G1b's claim is removed at the terminal
    /// write. **What the rental was left with instead**: the claim standing, the
    /// phase at *committed, unverified*, and the next setup told a package
    /// transaction is in progress.
    ///
    /// **The second end.** The next setup comes with its own attempt and its
    /// own installation, as the Set up screen's driver makes them. G18 finds the
    /// recorded attempt already terminal, so a fresh replacement begins — and
    /// this time the program starts and answers.
    #[tokio::test]
    async fn a_refused_start_rolls_back_and_gives_the_claim_back_and_the_next_setup_places() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        a_linux_shaped_far_end!();
        let mut desk = the_desk_or_say_so!(sftp_server);

        // ---------------------------------------------------- the first end
        desk.machine.refuse_the_next_start(true);
        let first = an_identity(IdentityKind::Operation);
        let reached = desk
            .one_placement(&first, &an_identity(IdentityKind::Installation))
            .await;
        assert_eq!(reached, WhatThePlacementReached::RolledBack);

        let WhatARecordHeld::It(left) = within(
            "the phase record read back after the rollback".to_string(),
            HOW_LONG_ONE_READING_IS_GIVEN,
            read_the_phase_record(&mut desk.subsystem, &desk.area),
        )
        .await
        .expect("the phase record reads") else {
            panic!("the phase record did not read back");
        };
        assert_eq!(left.phase, Phase::RolledBack);
        assert_eq!(left.attempt, first);
        assert_eq!(
            within(
                "the active pointer read back after the rollback".to_string(),
                HOW_LONG_ONE_READING_IS_GIVEN,
                the_active_pointers_value(&mut desk.subsystem, &desk.area),
            )
            .await
            .expect("the pointer reads"),
            None,
            "`Rented-server runtime` G6: a first placement's restore leaves no active pointer"
        );
        assert!(
            !Path::new(&the_staging_location_for(&desk.area, &first)).exists(),
            "`Rented-server runtime` G5 and §13 R1: what the rolled-back attempt staged is swept"
        );
        assert!(
            !Path::new(&smoozy_server_protocol::placement::the_record_at(
                &desk.area,
                smoozy_runtime::the_program::records::THE_PACKAGE_TRANSACTION_CLAIM
            ))
            .exists(),
            "`Rented-server runtime` G1b: the claim is removed at the terminal write"
        );

        // --------------------------------------------------- the second end
        desk.machine.refuse_the_next_start(false);
        let second = an_identity(IdentityKind::Operation);
        let installation = an_identity(IdentityKind::Installation);
        let reached = desk.one_placement(&second, &installation).await;
        assert_eq!(
            reached,
            WhatThePlacementReached::Verified,
            "the next setup did not place. The supervisor said: {}",
            desk.machine.what_the_supervisor_said()
        );
        desk.the_pointer_names_the_verified_program(&second).await;
        assert_eq!(
            desk.asked_now()
                .await
                .expect("the placed supervisor answers")
                .installation,
            installation
        );
    }

    /// **Step 10c, 2026-09-21: a login that opened no session is registered by
    /// granting lingering first** — `Rented-server runtime` K7 as corrected,
    /// run rather than argued.
    ///
    /// **What it proves.** On a machine where every user-level command fails
    /// with Sesterce's own words until the user's service manager is up, the
    /// placement grants lingering, reads it back, waits for the manager the
    /// grant started, enables, and reaches *verified* — which only the real
    /// supervisor answering can produce. **And the order is read off the
    /// machine**: the grant before the enable, and the enable after the manager
    /// first answered. Before the change the enable came first and failed.
    #[tokio::test]
    async fn a_login_with_no_session_is_registered_by_granting_lingering_first() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        a_linux_shaped_far_end!();
        let mut desk = the_desk_or_say_so!(sftp_server);
        desk.machine.flag("no-session", true);

        let attempt = an_identity(IdentityKind::Operation);
        let installation = an_identity(IdentityKind::Installation);
        let reached = desk.one_placement(&attempt, &installation).await;
        assert_eq!(
            reached,
            WhatThePlacementReached::Verified,
            "a login with no session was not registered. Asked: {:?}. The supervisor said: {}",
            desk.machine.what_was_asked(),
            desk.machine.what_the_supervisor_said()
        );

        let asked = desk.machine.what_was_asked();
        let at = |needle: &str| {
            asked
                .iter()
                .position(|line| line.contains(needle))
                .unwrap_or_else(|| panic!("{needle} was never asked: {asked:?}"))
        };
        let granted = at("enable-linger");
        let read_back = at("show-user");
        let enabled = at("--user enable");
        let the_manager_answered = asked
            .iter()
            .enumerate()
            .filter(|(_, line)| line.contains("--user show-environment"))
            .map(|(at, _)| at)
            .last()
            .expect("the manager was asked");
        assert!(granted < read_back, "{asked:?}");
        assert!(read_back < the_manager_answered, "{asked:?}");
        assert!(the_manager_answered < enabled, "{asked:?}");
        assert!(
            asked
                .iter()
                .filter(|line| line.contains("--user show-environment"))
                .count()
                > 1,
            "the manager's queued start was not waited for: {asked:?}"
        );
        desk.the_pointer_names_the_verified_program(&attempt).await;
    }

    /// **Step 10c, 2026-09-21: a refused enable is a failure before the commit,
    /// takes the rollback and gives the claim back, and the next setup places**
    /// — Sesterce's fault 2, where the refusal returned straight out of the
    /// walk and every later setup was told *already being installed*.
    #[tokio::test]
    async fn a_refused_enable_fails_before_the_commit_and_the_next_setup_places() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        a_linux_shaped_far_end!();
        let mut desk = the_desk_or_say_so!(sftp_server);

        desk.machine.flag("refuse-the-enable", true);
        let first = an_identity(IdentityKind::Operation);
        let installation = an_identity(IdentityKind::Installation);
        let reached = desk.one_placement(&first, &installation).await;
        match &reached {
            // **The loopback machine keeps no complaint stream** (its command
            // handler discards it), so what can be held here is the
            // placement's own words for the act; the machine's words beside
            // them are the real server's to carry.
            WhatThePlacementReached::FailedBeforeCommit { because } => assert!(
                because.starts_with("the machine would not enable the unit"),
                "the refusal was not said as the enable's: {because}"
            ),
            other => panic!("a refused enable reached {other:?}"),
        }
        let WhatARecordHeld::It(left) = within(
            "the phase record read back after the refused enable".to_string(),
            HOW_LONG_ONE_READING_IS_GIVEN,
            read_the_phase_record(&mut desk.subsystem, &desk.area),
        )
        .await
        .expect("the phase record reads") else {
            panic!("the phase record did not read back");
        };
        assert_eq!(left.phase, Phase::FailedBeforeCommit);
        assert!(
            !Path::new(&the_staging_location_for(&desk.area, &first)).exists(),
            "`Rented-server runtime` G20: the failed attempt's staging is swept"
        );
        assert!(
            !Path::new(&smoozy_server_protocol::placement::the_record_at(
                &desk.area,
                smoozy_runtime::the_program::records::THE_PACKAGE_TRANSACTION_CLAIM
            ))
            .exists(),
            "`Rented-server runtime` G1b: the claim is removed at the terminal write"
        );

        // **The next setup comes with its own attempt and, this time, another
        // installation** — so a claim left standing would have refused it.
        desk.machine.flag("refuse-the-enable", false);
        let second = an_identity(IdentityKind::Operation);
        let reached = desk
            .one_placement(&second, &an_identity(IdentityKind::Installation))
            .await;
        assert_eq!(
            reached,
            WhatThePlacementReached::Verified,
            "the next setup did not place. The supervisor said: {}",
            desk.machine.what_the_supervisor_said()
        );
        desk.the_pointer_names_the_verified_program(&second).await;
    }

    /// **Step 10c, 2026-09-21: a claim left by this installation's own placement
    /// whose process is gone is taken back, and the next setup places in the
    /// same request** — `Rented-server runtime` G1b's one take-over and G18's
    /// *"only after that terminal write may it … begin a fresh replacement"*.
    ///
    /// **The standing claim and record are written the way a placement cut off
    /// at *staged and validated* leaves them** — which is exactly what the
    /// Sesterce pass found on its machine — **and a claim naming another
    /// installation is still refused**, whatever its age.
    #[tokio::test]
    async fn a_claim_left_by_this_installation_is_taken_back_and_the_setup_places() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        a_linux_shaped_far_end!();
        let mut desk = the_desk_or_say_so!(sftp_server);

        let installation = an_identity(IdentityKind::Installation);
        let gone = an_identity(IdentityKind::Operation);
        let (taken, _) = within(
            "the claim a cut-off placement left".to_string(),
            HOW_LONG_ONE_READING_IS_GIVEN,
            take_the_claim(
                &mut desk.subsystem,
                &desk.area,
                &an_owner(installation.clone(), Route::SetupPlacement),
                Phase::Inspecting,
            ),
        )
        .await
        .expect("the claim answered");
        assert_eq!(
            taken,
            WhatTakingTheClaimReached::TheRulesHalfSays(ClaimOutcome::Acquired)
        );
        let mut left = smoozy_runtime::package::phase::a_fresh_attempts_first_write(
            gone.clone(),
            smoozy_server_protocol::placement::performing::the_phase_records_descriptor_for(
                &desk.verified.the_release_descriptors_values(),
            ),
            None,
            MachineKind::Systemd,
            the_staging_location_for(&desk.area, &gone),
        );
        left.owner = Some(an_owner(installation.clone(), Route::SetupPlacement));
        left.phase = Phase::StagedAndValidated;
        within(
            "the record a cut-off placement left".to_string(),
            HOW_LONG_ONE_READING_IS_GIVEN,
            write_the_phase_record(&mut desk.subsystem, &desk.area, &left),
        )
        .await
        .expect("the record is written");
        std::fs::write(the_staging_location_for(&desk.area, &gone), b"half a program")
            .expect("the cut-off attempt's staging is written");

        // **Another installation meets it and is refused**, whatever its age.
        let refused = desk
            .one_placement(
                &an_identity(IdentityKind::Operation),
                &an_identity(IdentityKind::Installation),
            )
            .await;
        assert!(
            matches!(
                refused,
                WhatThePlacementReached::APackageTransactionIsInProgress { .. }
            ),
            "another installation's claim was taken over: {refused:?}"
        );

        // **This installation takes it back and places, in one request.**
        let attempt = an_identity(IdentityKind::Operation);
        let reached = desk.one_placement(&attempt, &installation).await;
        assert_eq!(
            reached,
            WhatThePlacementReached::Verified,
            "the setup did not place over its own cut-off attempt. The supervisor said: {}",
            desk.machine.what_the_supervisor_said()
        );
        assert!(
            !Path::new(&the_staging_location_for(&desk.area, &gone)).exists(),
            "`Rented-server runtime` G20: the cut-off attempt's staging is swept"
        );
        desk.the_pointer_names_the_verified_program(&attempt).await;
    }

    /// **Step 10c, 2026-09-21: a container placement installs its hook and its
    /// start step starts something** — `Rented-server runtime` K11, K15, K16,
    /// G25 and S5, run rather than argued.
    ///
    /// **What it proves.** The hook is written at the provider's path with the
    /// provider's own content kept (K11, K16), K15's read-back confirms it, the
    /// commit's start step starts S5's keeper, the keeper starts the real
    /// supervisor through the pointer, and **that supervisor's health answer is
    /// what makes the placement verified** — G13 accepts nothing else. **Then a
    /// second start while the first keeper runs ends at once** (S5's lock), and
    /// the supervisor still answers.
    ///
    /// **What is stood in, and only this**: `setsid` and `flock`, which this desk
    /// does not carry, each by a few lines that make the same system call
    /// (`setsid(2)`, `flock(2)`); and `/root/onstart.sh`, which is a file in this
    /// machine's home. **The provider's boot chain is not run** — nothing here
    /// restarts a container.
    #[tokio::test]
    async fn a_container_placement_installs_its_hook_and_its_start_step_starts_the_keeper() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        a_linux_shaped_far_end!();
        let mut desk = the_desk_or_say_so!(sftp_server);
        if std::process::Command::new("perl").arg("-e").arg("1").status().map(|it| !it.success()).unwrap_or(true) {
            println!("SKIPPED: this desk has no perl to stand in setsid and flock. Unproved here.");
            return;
        }

        // **The provider's own hook, as the second pass read it on Vast.**
        let theirs = b"#!/bin/bash\n# This file is run on instance start. Output in /var/log/onstart.log\n";
        std::fs::write(desk.machine.home.join("onstart.sh"), theirs).expect("the provider's hook");

        let attempt = an_identity(IdentityKind::Operation);
        let installation = an_identity(IdentityKind::Installation);
        let reached = desk.one_placement_on_a_container(&attempt, &installation).await;
        let words = std::fs::read_to_string(format!("{}/supervisor-words", desk.area))
            .unwrap_or_default();
        assert_eq!(
            reached,
            WhatThePlacementReached::Verified,
            "the keeper's supervisor did not answer. It said: {words}"
        );
        desk.the_pointer_names_the_verified_program(&attempt).await;

        // **K11 and K16: the hook is at the provider's path, the provider's own
        // lines are kept, and the record holds exactly what is there.**
        let hook = std::fs::read(desk.machine.home.join("onstart.sh")).expect("the hook is there");
        let text = String::from_utf8(hook.clone()).expect("the hook is text");
        assert!(text.starts_with("#!/bin/bash\n"));
        assert!(text.ends_with("# This file is run on instance start. Output in /var/log/onstart.log\n"));
        assert!(text.contains(&format!(
            "{} {THE_SUPERVISOR_WORD} 9>&- ",
            the_active_package_pointer_at(&desk.area)
        )));
        let WhatARecordHeld::It(record) =
            read_the_boot_registration_record(&mut desk.subsystem, &desk.area)
                .await
                .expect("the record reads")
        else {
            panic!("no boot-registration record");
        };
        assert_eq!(record.kind, MachineKind::Container);
        assert_eq!(record.at_path, desk.machine.home.join("onstart.sh").to_string_lossy());
        assert_eq!(record.content, hook);

        // **G25: the start step started the keeper**, which is still running.
        let before = desk.keepers_running();
        assert!(before > 0, "no keeper is running after the placement");

        // **S5: a second start while the first keeper holds its lock ends at
        // once and starts nothing.**
        within(
            "a second start of the keeper".to_string(),
            HOW_LONG_ONE_READING_IS_GIVEN,
            smoozy_server_protocol::placement::start_the_supervisor_the_pointer_names(
                &mut desk.subsystem,
                &mut desk.connection,
                MachineKind::Container,
                &desk.area,
            ),
        )
        .await
        .expect("the second start was sent");
        tokio::time::sleep(Duration::from_secs(2)).await;
        assert_eq!(
            desk.keepers_running(),
            before,
            "a second keeper stayed running beside the first"
        );
        assert_eq!(
            desk.asked_now()
                .await
                .expect("the placed supervisor still answers")
                .installation,
            installation
        );
    }

    /// **Step 10c, 2026-09-22: a second placement on a container leaves the
    /// newly placed program running, and the one the old pointer started does
    /// not go on serving.**
    ///
    /// **What it proves, and what watching a rental proved instead.** On the
    /// third and fourth passes on Vast the supervisor kept the same process
    /// through three placements; only a restart at the provider ever started
    /// the newly placed program, and on the fourth pass a setup run failed
    /// outright because the desktop's question reached a program that knew
    /// nothing about it. `Rented-server runtime` G25 orders the running
    /// supervisor ended before the restarter is asked, and on a container
    /// nothing ended it: S5's lock makes the keeper the start step launches end
    /// at once, and a keeper never restarts a supervisor that never ended.
    ///
    /// **What the fix is**: the running supervisor is asked to end itself over
    /// the ordinary command path, records the asking before it acts, answers,
    /// and ends — and the keeper already running, which S5 calls the same
    /// restarter reached by the other door, starts the one the pointer now
    /// names.
    ///
    /// **How it is proved here**: the supervisor answering after the second
    /// placement carries a different run identifier from the one answering
    /// before it. `Rented-server runtime` S2 issues a fresh one at every start,
    /// so a run that changed is a program that ended and was started again —
    /// and the same number of supervisors is running, so nothing was left
    /// beside it.
    #[tokio::test]
    async fn a_second_container_placement_ends_the_supervisor_the_first_one_started() {
        let _deadline = TheTestsDeadline::started();
        let sftp_server = the_server_or_say_so!();
        a_linux_shaped_far_end!();
        let mut desk = the_desk_or_say_so!(sftp_server);
        if std::process::Command::new("perl").arg("-e").arg("1").status().map(|it| !it.success()).unwrap_or(true) {
            println!("SKIPPED: this desk has no perl to stand in setsid and flock. Unproved here.");
            return;
        }
        std::fs::write(desk.machine.home.join("onstart.sh"), b"#!/bin/sh\n")
            .expect("the provider's hook");

        let installation = an_identity(IdentityKind::Installation);
        let first = an_identity(IdentityKind::Operation);
        assert_eq!(
            desk.one_placement_on_a_container(&first, &installation).await,
            WhatThePlacementReached::Verified,
            "the first placement did not verify. The supervisor said: {}",
            std::fs::read_to_string(format!("{}/supervisor-words", desk.area)).unwrap_or_default()
        );
        let before = desk
            .asked_now()
            .await
            .expect("the first placement's supervisor answers");
        let keepers = desk.keepers_running();
        assert!(keepers > 0, "no keeper is running after the first placement");

        // **The second placement of the same version**, which is the case the
        // rental met: the pointer moves, and until this repair the program the
        // old pointer named went on answering.
        let second = an_identity(IdentityKind::Operation);
        assert_eq!(
            desk.one_placement_on_a_container(&second, &installation).await,
            WhatThePlacementReached::Verified,
            "the second placement did not verify. The supervisor said: {}",
            std::fs::read_to_string(format!("{}/supervisor-words", desk.area)).unwrap_or_default()
        );
        desk.the_pointer_names_the_verified_program(&second).await;

        // **The supervisor answering is not the one that was answering.** The
        // keeper's own pause stands between the ending and the next start, so
        // this is asked until it changes, inside a deadline of its own — never
        // a wait with nothing that would end it.
        let started = std::time::Instant::now();
        let mut now = before.clone();
        while started.elapsed() < HOW_LONG_ONE_READING_IS_GIVEN {
            now = desk
                .asked_now()
                .await
                .expect("a supervisor answers after the second placement");
            if now.run != before.run {
                break;
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        assert_ne!(
            now.run, before.run,
            "the supervisor the first placement started is still the one answering, so the \
             newly placed program is not running"
        );
        assert_eq!(now.installation, installation);
        assert_eq!(
            desk.supervisors_running(),
            1,
            "the old supervisor is running beside the new one"
        );
        assert_eq!(
            desk.keepers_running(),
            keepers,
            "a second keeper stayed running beside the first"
        );
    }
}

// ------------------------------ the guard that would have caught this twice

/// **No file this crate compiles reaches `std::os::unix` outside a
/// `#[cfg(unix)]`, and this is the test that says so at the desk.**
///
/// **It exists because the same fault broke the whole Windows build twice in
/// five days, both times in this file** — hosted run `34110919264` on
/// 2026-09-07 and hosted run `34697517090` on 2026-09-12 — and each time it
/// cost the two systems a whole run, because one test target that will not
/// build stops every other crate's tests from running at all. The second time
/// it took **four portable tests of this file that Windows was meant to run**
/// down with the one Linux-shaped test that caused it.
///
/// **Nothing else can catch it here.** This desk is a Mac, where
/// `std::os::unix` compiles perfectly; the compiler that objects is on a
/// runner nobody here can start cheaply. So what a desk test can hold is the
/// rule, and this holds the rule.
///
/// **How it reads the files.** It strips every `#[cfg(unix)]` region — the
/// attribute on a block and the attribute on an item both open one at the next
/// brace and close it when that brace closes — and then looks for
/// `std::os::unix` in what is left, ignoring whole-line comments so that the
/// prose above a guard does not accuse it. **What survives is what Windows
/// would try to resolve, and there must be none of it.**
#[test]
fn nothing_this_crate_compiles_reaches_std_os_unix_outside_a_unix_guard() {
    let _deadline = TheTestsDeadline::started();
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files: Vec<PathBuf> = Vec::new();
    for folder in ["src", "tests"] {
        the_rust_files_under(&crate_root.join(folder), &mut files);
    }
    assert!(
        files.len() > 1,
        "the sweep found no files to read, so it is proving nothing"
    );

    // **The thing being looked for is spelled out of its own three pieces**,
    // because a test that wrote it whole would find itself and accuse its own
    // message of being the fault. It did, the first time it was run.
    let the_path_windows_cannot_resolve = ["std", "os", "unix"].join("::");

    let mut unguarded: Vec<String> = Vec::new();
    for file in &files {
        let text = std::fs::read_to_string(file).expect("the file reads");
        let mut depth: usize = 0;
        let mut guarded_since: Vec<usize> = Vec::new();
        let mut the_next_brace_is_a_guard = false;
        for (number, line) in text.lines().enumerate() {
            let trimmed = line.trim_start();
            let it_is_a_comment = trimmed.starts_with("//");
            if !it_is_a_comment
                && guarded_since.is_empty()
                && line.contains(&the_path_windows_cannot_resolve)
            {
                unguarded.push(format!(
                    "{}:{}: {}",
                    file.strip_prefix(crate_root).unwrap_or(file).display(),
                    number + 1,
                    trimmed
                ));
            }
            if !it_is_a_comment && trimmed.starts_with("#[cfg(unix)]") {
                the_next_brace_is_a_guard = true;
            }
            if it_is_a_comment {
                continue;
            }
            for character in line.chars() {
                match character {
                    '{' => {
                        depth += 1;
                        if the_next_brace_is_a_guard {
                            guarded_since.push(depth);
                            the_next_brace_is_a_guard = false;
                        }
                    }
                    '}' => {
                        if guarded_since.last() == Some(&depth) {
                            guarded_since.pop();
                        }
                        depth = depth.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
    }

    assert!(
        unguarded.is_empty(),
        "these reach `{}` where Windows will try to resolve it, and a test target \
         that will not build stops every crate's tests on that system:\n  {}",
        the_path_windows_cannot_resolve,
        unguarded.join("\n  ")
    );
}

/// Every `.rs` file under a folder, however deep.
fn the_rust_files_under(folder: &Path, into: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            the_rust_files_under(&path, into);
        } else if path.extension().is_some_and(|it| it == "rs") {
            into.push(path);
        }
    }
}

// ------------------------------------------------------ this test's own deadline

/// **What every test in this file waits on**, named in the message a missed
/// deadline prints.
const WHAT_THE_TESTS_HERE_WAIT_ON: &str =
    "the loopback machine: OpenSSH's own sftp-server, the machine's own shell, and the placed smoozy-server";

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
                }
                std::process::exit(101);
            }
        });
        TheTestsDeadline { _held: held }
    }
}

//! **Step 8a's third piece, run rather than argued: a package is composed,
//! signed, verified, fetched, staged on a machine, read back at rest and only
//! then made executable.**
//!
//! **What the far end is.** A real SSH connection, opened by Smoozy's own
//! transport on the loopback address, carrying a **subsystem session served by
//! OpenSSH's own `sftp-server`** — the same program a rental's own
//! file-transfer server is. Nothing is rented; nothing on the Creator's machine
//! outside a temporary directory is touched.
//!
//! **What is proved here, in the plan's own terms.**
//!
//! - **B8** — an artifact entry is composed from bytes that exist, the manifest
//!   carries the byte length, the digest, the fixed entry-point name and the
//!   exact source, and selection is an exact tuple match.
//! - **B9** — the root-signed key-set statement admits the online key, and a
//!   manifest nobody allowed does not verify.
//! - **B10, all three verifications** — before download, before upload, and at
//!   rest from the desktop, with the artifact made executable only after the
//!   third agreed.
//!
//! **Two things here are not the real thing and each says so where it stands.**
//! **The keys come from written-down seeds** and are therefore public, because
//! the release key is the Creator's; **the source is a local file**, because
//! there is nowhere to host the package. Everything between them is the real
//! code.
//!
//! **What this test does not prove, said rather than glossed.** The staging
//! write below is the subsystem's own writes and not §7's byte loop — **§7's
//! whole contract is proved next door**, in `a_file_goes_up_and_comes_back.rs`,
//! and what is under test here is the verification either side of it. And the
//! artifact is a file this test wrote rather than the Linux binary a release
//! would carry, **which cannot be linked at this desk**: the desk's linker is
//! Apple's and the machine's target wants the GNU one, which is the hosted
//! runner's job and is recorded in this step's decisions record.
//!
//! **Where `sftp-server` is not on this system the tests say so and stop.**
//! They do not quietly pass.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` G29.
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
//! above are the words `src/lib.rs` carries.** They are repeated because a file
//! under `tests/` is its own crate and never sees that one.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::{Duration, Instant};

use russh::server::{Auth, Msg, Server as _, Session};
use russh::{Channel, ChannelId};
use smoozy_server_protocol::release::composing::{
    self, AFetchFromALocalFileStandingInForHostingThatDoesNotExist,
    FetchesFromAnExactSourceTheManifestNamed,
};
use smoozy_server_protocol::release::manifest::TargetTuple;
use smoozy_server_protocol::release::trust::{
    AnAllowedKey, KeySetStatement, THE_GREATEST_KEY_SET_GENERATION_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE,
    THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE, THE_KEY_SET_FORMAT,
};
use smoozy_server_protocol::release::verifying::{
    compare_at_rest, make_the_artifact_executable, read_back_at_rest, verify_before_download,
    verify_what_was_downloaded, AVerifiedRelease, AnOfferedRelease, WhatTheReadBackFound,
    WhyTheDownloadWasRefused, THE_MODE_A_PLACED_ARTIFACT_IS_GIVEN,
};
use smoozy_server_protocol::release::{
    a_signature_over, THE_KEY_SET_SIGNING_CONTEXT, THE_MANIFEST_SIGNING_CONTEXT,
};
use smoozy_runtime::machine_kind::MachineKind;
use smoozy_runtime::package::phase::{a_fresh_attempts_first_write, Phase};
use smoozy_runtime::package::release::{RetainedAdapter, VerifiedReleaseDescriptor};
use smoozy_runtime::registration::BootRegistrationRecord;
use smoozy_runtime::the_program::{records, THE_FIXED_ENTRY_POINT_NAME};
use smoozy_runtime::written_form;
use smoozy_server_protocol::bootstrap::{inspect, WhatARecordHeld};
use smoozy_server_protocol::reconnection::{
    read_the_installation_identity, what_a_replaced_connection_does, WhatTheStateAreaHeld,
    THE_RECORD_THE_IDENTITY_SITS_IN,
};
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::transport::ssh::{open, Connection};
use smoozy_server_protocol::transport::{
    Credential, DecideHostTrust, Endpoint, Figures, HostTrust,
};
use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};
use smoozy_state::observed::Clock;
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
                     B10's at-rest read-back is unproved here.",
                    std::env::consts::OS
                );
                return;
            }
        }
    };
}

/// Says out loud that what comes next is a rule about a Linux far end, and
/// stops — **never a quieter assertion**.
///
/// **The same shape `a_file_goes_up_and_comes_back.rs` uses**, and on the same
/// ground: §7's head-note, *"Three operating systems on the user's end, Linux
/// alone on the rented end"*, with `Servers` Q13's not-Linux check refusing
/// anything else. **The permission bits are the case here**: making a file
/// runnable by its owner alone is a Linux rule, and Microsoft's own port of
/// OpenSSH does not keep it, so a Windows far end standing in would be judged
/// on a part it never plays.
macro_rules! a_linux_rule_about_the_rented_machine {
    ($why:expr) => {
        if cfg!(windows) {
            println!(
                "SKIPPED from here on ({}): {}. Unproved here.",
                std::env::consts::OS,
                $why
            );
            return;
        }
    };
}

// --------------------------------------------------- the machine, on loopback

struct Machine {
    sftp_server: PathBuf,
    channels: std::collections::HashMap<ChannelId, Channel<Msg>>,
}

impl Clone for Machine {
    fn clone(&self) -> Machine {
        Machine {
            sftp_server: self.sftp_server.clone(),
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
        tokio::spawn(async move {
            let mut child = match tokio::process::Command::new(program)
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

async fn a_machine(host_key_file: &Path, sftp_server: PathBuf) -> RunningMachine {
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
            "smoozy-release-{name}-{}-{}",
            std::process::id(),
            Instant::now().elapsed().as_nanos()
        ));
        std::fs::create_dir_all(&path).expect("a temporary directory can be made");
        Scratch(path)
    }

    fn a_key(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        let made = std::process::Command::new("ssh-keygen")
            .args(["-t", "ed25519", "-N", "", "-C", "smoozy release test", "-f"])
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
/// A release key is the Creator's and lives outside this project.
const THE_ROOT_SEED: [u8; 32] = *b"a release root for this test onl";
const THE_ONLINE_SEED: [u8; 32] = *b"an online signing key, this test";

/// **The source is an address nobody serves.** `example.invalid` is reserved by
/// its own registry for exactly this, and the fetch below maps it to a file on
/// this computer. **What replaces it:** hosting, and a fetcher that speaks
/// HTTPS to it.
const THE_SOURCE: &str = "https://example.invalid/smoozy-server/0.1.0/linux-x86_64-gnu";

/// Bytes standing in for the machine program. **Deterministic**, so a failing
/// run reproduces.
fn the_artifacts_bytes() -> Vec<u8> {
    let mut out = Vec::with_capacity(300_000);
    let mut value: u32 = 0x5eed_1cbd;
    while out.len() < 300_000 {
        value = value.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}

struct AReleaseAsHostingWouldServeIt {
    offered: AnOfferedRelease,
    root_public_key: [u8; 32],
}

/// **Published the way the next real release will be: one past the release
/// sequence this build carries as its floor, at the key-set generation this
/// build carries** — the release a desktop built today actually meets. *(Until
/// 2026-09-21 it was sequence 7 and generation 1, which passed only while
/// `Server protocol and transfers` B9's built-in pair was zero; the pair is now
/// the first published release's, and a sequence below it is refused, as it
/// should be.)*
fn a_release(sources: &[String]) -> AReleaseAsHostingWouldServeIt {
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

    let artifact = composing::compose_an_artifact_entry(
        "linux-x86_64-gnu",
        composing::the_tuple_this_build_is_made_for(),
        &the_artifacts_bytes(),
        composing::THE_FIXED_ENTRY_POINT_NAME,
        sources,
    );
    let manifest = composing::compose_a_manifest(
        THE_GREATEST_RELEASE_SEQUENCE_PUBLISHED_WHEN_THIS_BUILD_WAS_MADE + 1,
        "0.1.0",
        2,
        None,
        vec![artifact],
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

/// **The tuple selection is made against, and it is the constant naming what
/// this build produced rather than anything read off a machine.**
///
/// **B8's supplier does not exist.** That clause says `Server Setup` supplies
/// the detected rental tuple, and nothing anywhere in this workspace reads a
/// processor architecture or a C library generation off a machine. **The name
/// says so** so that nobody wiring the real selection up reaches for a function
/// that looks like a reading, finds it already written, and ships a selection
/// that always matches — which is the one thing the exact match exists to
/// catch.
fn the_build_constant_standing_in_for_a_tuple_nothing_reads() -> TargetTuple {
    composing::the_tuple_this_build_is_made_for()
}

/// Verify the release and fetch the artifact from the source it names, in
/// B10's own order.
fn verified_and_fetched(
    release: &AReleaseAsHostingWouldServeIt,
    fetch: &mut impl FetchesFromAnExactSourceTheManifestNamed,
) -> (AVerifiedRelease, Vec<u8>) {
    let verified = verify_before_download(
        &release.root_public_key,
        None,
        &release.offered,
        &the_build_constant_standing_in_for_a_tuple_nothing_reads(),
        None,
    )
    .expect("the release verifies");
    let source = verified.selected.sources[0].clone();
    let bytes = fetch.fetch(&source).expect("the source answers");
    verify_what_was_downloaded(&verified.selected, &source, &bytes)
        .expect("the downloaded bytes are the artifact");
    (verified, bytes)
}

async fn stage(
    subsystem: &mut Subsystem<russh::ChannelStream<russh::client::Msg>>,
    path: &[u8],
    bytes: &[u8],
) {
    let handle = subsystem
        .create_exclusively(path)
        .await
        .expect("the staging path is fresh");
    let mut at = 0u64;
    for piece in bytes.chunks(32_768) {
        subsystem
            .send_a_write(&handle, at, piece)
            .await
            .expect("the write is handed over");
        at += piece.len() as u64;
    }
    subsystem
        .settle_the_writes()
        .await
        .expect("every write settles");
    subsystem.close(&handle).await.expect("the handle closes");
}

// ------------------------------------------------------------------ the tests

#[tokio::test]
async fn a_package_is_verified_fetched_staged_read_back_and_only_then_made_executable() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("whole");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server).await;

    // Hosting, standing in as a file on this computer.
    let served = scratch.at("what-hosting-would-serve");
    std::fs::write(&served, the_artifacts_bytes()).expect("the file is written");
    let mut fetch =
        AFetchFromALocalFileStandingInForHostingThatDoesNotExist::where_each_source_stands_for_a_file(
            vec![(THE_SOURCE.to_string(), served)],
        );

    let release = a_release(&[THE_SOURCE.to_string()]);
    let (verified, bytes) = verified_and_fetched(&release, &mut fetch);

    // **The exact-source rule, held rather than assumed**: the fetcher was
    // asked for the address the verified manifest named and for nothing else.
    assert_eq!(fetch.asked_for, vec![THE_SOURCE.to_string()]);
    assert_eq!(verified.selected.entry_point_name, "smoozy-server");
    assert_eq!(verified.signed_by, "the online signing key");

    let connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let staged = scratch.at("staged-artifact");
    let staged_path = staged.to_string_lossy().into_owned().into_bytes();
    stage(&mut subsystem, &staged_path, &bytes).await;

    // **B10's at-rest read-back, over the machine's own file-transfer server,
    // with the digest computed in Smoozy's own loop** — `Rented-server runtime`
    // G29's comparison, performed by the desktop.
    let descriptor = verified.the_release_descriptors_values();
    let found = read_back_at_rest(
        &mut subsystem,
        &staged_path,
        descriptor.artifact_byte_length,
        32_768,
    )
    .await
    .expect("the staged bytes read back");
    let WhatTheReadBackFound::ItRead(reading) = found else {
        panic!("the staged artifact read as more bytes than the manifest names");
    };
    assert_eq!(reading.byte_length, bytes.len() as u64);

    let agreed =
        compare_at_rest(&descriptor, &staged_path, reading).expect("the comparison agrees");

    // **Only now.** Nothing above this line could have made it executable
    // through this file: the token this call takes has no other producer, no
    // copy and no clone, and it names the path and the artifact it was earned
    // over.
    make_the_artifact_executable(
        &mut subsystem,
        &staged_path,
        &descriptor.artifact_digest,
        agreed,
    )
    .await
    .expect("the mode is set");

    a_linux_rule_about_the_rented_machine!(
        "the mode a placed artifact is given is a Linux rule about the rented machine"
    );
    let attributes = subsystem
        .stat(&staged_path)
        .await
        .expect("the staged artifact is there");
    assert_eq!(
        attributes.permissions.map(|mode| mode & 0o777),
        Some(THE_MODE_A_PLACED_ARTIFACT_IS_GIVEN),
        "the artifact was not made runnable by its owner alone"
    );
}

#[tokio::test]
async fn a_staged_artifact_a_byte_of_which_did_not_arrive_is_never_made_executable() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("corrupt");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server).await;

    let served = scratch.at("what-hosting-would-serve");
    std::fs::write(&served, the_artifacts_bytes()).expect("the file is written");
    let mut fetch =
        AFetchFromALocalFileStandingInForHostingThatDoesNotExist::where_each_source_stands_for_a_file(
            vec![(THE_SOURCE.to_string(), served)],
        );
    let release = a_release(&[THE_SOURCE.to_string()]);
    let (verified, bytes) = verified_and_fetched(&release, &mut fetch);

    let connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    // One byte of the staged copy is not the artifact's. Nothing about the
    // length changes, so the length alone would pass.
    let mut damaged = bytes.clone();
    damaged[bytes.len() / 2] ^= 1;
    let staged = scratch.at("staged-artifact");
    let staged_path = staged.to_string_lossy().into_owned().into_bytes();
    stage(&mut subsystem, &staged_path, &damaged).await;

    let descriptor = verified.the_release_descriptors_values();
    let found = read_back_at_rest(
        &mut subsystem,
        &staged_path,
        descriptor.artifact_byte_length,
        32_768,
    )
    .await
    .expect("the staged bytes read back");
    let WhatTheReadBackFound::ItRead(reading) = found else {
        panic!("the staged artifact read as more bytes than the manifest names");
    };
    assert_eq!(reading.byte_length, bytes.len() as u64);

    assert!(
        compare_at_rest(&descriptor, &staged_path, reading).is_none(),
        "a staged artifact that is not the verified one was accepted"
    );

    // And the mode was never set: the file is still what the staging create
    // made it.
    a_linux_rule_about_the_rented_machine!(
        "the mode a placed artifact is given is a Linux rule about the rented machine"
    );
    let attributes = subsystem
        .stat(&staged_path)
        .await
        .expect("the staged artifact is there");
    assert_ne!(
        attributes.permissions.map(|mode| mode & 0o777),
        Some(THE_MODE_A_PLACED_ARTIFACT_IS_GIVEN)
    );
}

#[tokio::test]
async fn bytes_from_an_address_the_manifest_does_not_name_are_refused_whatever_they_are() {
    let _deadline = TheTestsDeadline::started();
    let scratch = Scratch::new("source");
    let served = scratch.at("what-hosting-would-serve");
    std::fs::write(&served, the_artifacts_bytes()).expect("the file is written");

    // The manifest names one address; the bytes arrive from another, and they
    // are the right bytes.
    let release = a_release(&[THE_SOURCE.to_string()]);
    let verified = verify_before_download(
        &release.root_public_key,
        None,
        &release.offered,
        &the_build_constant_standing_in_for_a_tuple_nothing_reads(),
        None,
    )
    .expect("the release verifies");

    assert_eq!(
        verify_what_was_downloaded(
            &verified.selected,
            "https://example.invalid/somewhere-else",
            &the_artifacts_bytes(),
        ),
        Err(WhyTheDownloadWasRefused::ThatSourceIsNotOneTheManifestNamed)
    );
}

/// **A staged object with more bytes at rest than the manifest names stops the
/// read rather than being read to its end.**
///
/// `Server protocol and transfers` A6 says an authenticated connection is not
/// evidence about the machine at the other end of it, and this reads from
/// exactly such a machine. A machine that answered every read with a full chunk
/// and never ended would hold the desktop for ever, computing a digest it would
/// never compare, and **a file that grows under the read does the same thing by
/// accident.** The ceiling is the manifest's own byte length and the read stops
/// at the moment it would be passed.
#[tokio::test]
async fn a_staged_object_longer_than_the_manifest_names_stops_the_read_and_is_refused() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("longer");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server).await;

    let served = scratch.at("what-hosting-would-serve");
    std::fs::write(&served, the_artifacts_bytes()).expect("the file is written");
    let mut fetch =
        AFetchFromALocalFileStandingInForHostingThatDoesNotExist::where_each_source_stands_for_a_file(
            vec![(THE_SOURCE.to_string(), served)],
        );
    let release = a_release(&[THE_SOURCE.to_string()]);
    let (verified, bytes) = verified_and_fetched(&release, &mut fetch);

    let connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    // The artifact's own bytes, and then a great many more.
    let mut longer = bytes.clone();
    longer.extend(std::iter::repeat(b'.').take(4_000_000));
    let staged = scratch.at("staged-artifact");
    let staged_path = staged.to_string_lossy().into_owned().into_bytes();
    stage(&mut subsystem, &staged_path, &longer).await;

    let descriptor = verified.the_release_descriptors_values();
    let found = read_back_at_rest(
        &mut subsystem,
        &staged_path,
        descriptor.artifact_byte_length,
        32_768,
    )
    .await
    .expect("the read-back answered");
    assert_eq!(
        found,
        WhatTheReadBackFound::ThereAreMoreBytesThanTheManifestNames {
            the_manifest_names: descriptor.artifact_byte_length
        },
        "the read went past the length the manifest names"
    );

    // And nothing was made executable: there is no reading to compare, so
    // there is no token to hand `make_the_artifact_executable`.
    a_linux_rule_about_the_rented_machine!(
        "the mode a placed artifact is given is a Linux rule about the rented machine"
    );
    let attributes = subsystem
        .stat(&staged_path)
        .await
        .expect("the staged object is there");
    assert_ne!(
        attributes.permissions.map(|mode| mode & 0o777),
        Some(THE_MODE_A_PLACED_ARTIFACT_IS_GIVEN)
    );
}

/// **`Server protocol and transfers` C15, run against a real file-transfer
/// server: the installation identity read on the pre-package surface, from
/// Smoozy's own state area.**
///
/// **Three things are proved and the third is the one that matters for
/// `Server protocol and transfers` C12.** A package area holding the record
/// answers with the identity; an area holding no record answers *nothing*,
/// which is `State, identity and recovery` §2.3's absent branch; and **the read
/// asks no supervisor and runs no command** — it is a read of the second of
/// §1.1's two mechanisms and nothing else, which is why it works in the branch
/// where the supervisor is not answering.
#[tokio::test]
async fn the_installation_identity_is_read_off_smoozys_own_state_area() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("identity");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server).await;

    let connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    // A package area a placement wrote, with the record `Rented-server runtime`
    // G10 publishes into it.
    let area = scratch.at("package-area");
    std::fs::create_dir_all(&area).expect("the area is made");
    let placed = smoozy_state::identities::Identity::issue(
        smoozy_state::identities::IdentityKind::Installation,
    )
    .expect("randomness");
    std::fs::write(
        area.join(THE_RECORD_THE_IDENTITY_SITS_IN),
        placed.as_text().as_bytes(),
    )
    .expect("the record is written");

    let area_bytes = area.to_string_lossy().into_owned().into_bytes();
    assert_eq!(
        read_the_installation_identity(&mut subsystem, &area_bytes)
            .await
            .expect("the read answered"),
        WhatTheStateAreaHeld::AnIdentity(placed.clone())
    );

    // **A machine that has had no placement.** The record is absent and the
    // answer is the absent branch rather than a failure.
    let never_prepared = scratch.at("no-placement-here");
    std::fs::create_dir_all(&never_prepared).expect("the area is made");
    assert_eq!(
        read_the_installation_identity(
            &mut subsystem,
            &never_prepared.to_string_lossy().into_owned().into_bytes()
        )
        .await
        .expect("the read answered"),
        WhatTheStateAreaHeld::Nothing
    );

    // **C12, from the reading's own side.** The read above asked no supervisor,
    // so the branch where the supervisor did not answer still has its first
    // half — and that branch resumes transfers while command correlation waits.
    let unconfirmed = InstallationCheck::from_the_two_halves(Some(&placed), Some(&placed), false);
    assert_eq!(
        unconfirmed,
        InstallationCheck::PresentAndTheSupervisorDidNotAnswer
    );
    let what = what_a_replaced_connection_does(unconfirmed);
    assert!(what.transfers_resume);
    assert!(!what.command_correlation_resumes);
    assert!(!what.everything_held_against_the_row_is_disqualified);
}

/// **B4, over a real file-transfer server: package inspection is a composed
/// reading of the installation's own durable records, and it answers honestly
/// on a machine that has never held a package.**
///
/// **Three things this proves that a desk test cannot.** The records are read
/// over the second of §1.1's two mechanisms and nothing else — no command runs
/// and no supervisor is asked, which is why the answer stands beside a dead one.
/// The paths cross as protocol fields. And the active-package pointer is read
/// with the subsystem's own `readlink`, whose reply this crate had never taken
/// before.
///
/// **B5** rides with it: what comes back carries the moment it was taken and the
/// clock it was taken on.
#[tokio::test]
async fn package_inspection_is_composed_from_the_installations_own_records() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("inspection");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server).await;

    let connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    // **A machine that has never held a package**, and the area is there and
    // empty. Every record is absent and that is the answer rather than a
    // failure.
    let bare = scratch.at("a-bare-machine");
    std::fs::create_dir_all(&bare).expect("the area is made");
    let reading = inspect(
        &mut subsystem,
        &bare.to_string_lossy().into_owned().into_bytes(),
    )
    .await
    .expect("the reading answered");
    assert!(reading.value().every_record_is_absent());
    assert!(!reading.value().a_record_is_present_and_not_smoozys());
    // **B5.** The moment and the clock ride with it, and the clock is the
    // desktop's own because the desktop composed the answer.
    assert_eq!(reading.clock(), &Clock::ThisComputer);

    // **A machine a placement wrote records into.** Each is written in
    // `Rented-server runtime`'s own written form, at that document's own record
    // names, which is what the desktop reads back.
    let area = scratch.at("package-area");
    std::fs::create_dir_all(&area).expect("the area is made");
    let staging = area.join("staged-0.1.0").to_string_lossy().into_owned();

    let attempt = Identity::issue(IdentityKind::Operation).expect("randomness");
    let mut record = a_fresh_attempts_first_write(
        attempt,
        VerifiedReleaseDescriptor {
            manifest_format: "smoozy-server-package-manifest-1".to_string(),
            verified_release_sequence: 11,
            requested_package_version: "0.1.0".to_string(),
            manifest_digest: [3; 32],
            artifact_identity: "linux-x86_64-gnu".to_string(),
            artifact_byte_length: 4096,
            artifact_digest: [4; 32],
        },
        None,
        MachineKind::Systemd,
        staging.clone(),
    );
    record.phase = Phase::Verified;
    record.staged_packages_retained_adapters = vec![RetainedAdapter {
        declared_interval: (1, 1),
        declared_request_set: vec!["readiness".to_string(), "health".to_string()],
    }];
    std::fs::write(
        area.join(records::THE_PHASE_RECORD),
        written_form::write(&record).expect("it writes"),
    )
    .expect("the record is written");
    std::fs::write(
        area.join(records::THE_BOOT_REGISTRATION_RECORD),
        written_form::write(&BootRegistrationRecord {
            kind: MachineKind::Systemd,
            at_path: "/home/whoever/.config/systemd/user/smoozy.service".to_string(),
            content: b"[Unit]\nDescription=Smoozy\n".to_vec(),
        })
        .expect("it writes"),
    )
    .expect("the record is written");
    let placed = Identity::issue(IdentityKind::Installation).expect("randomness");
    std::fs::write(
        area.join(records::THE_INSTALLATION_IDENTITY),
        placed.as_text().as_bytes(),
    )
        .expect("the record is written");

    let area_bytes = area.to_string_lossy().into_owned().into_bytes();

    // **Without the pointer there is no active package**, and the rest of the
    // reading still answers. That is `Rented-server runtime` G31a's own rule:
    // no descriptor is claimed for a pointer that names nothing.
    let reading = inspect(&mut subsystem, &area_bytes)
        .await
        .expect("the reading answered");
    let inspection = reading.value();
    assert!(inspection.the_active_packages_descriptor.is_none());
    assert_eq!(inspection.the_phase, WhatARecordHeld::It(Phase::Verified));
    assert_eq!(
        inspection.the_installation_identity,
        WhatTheStateAreaHeld::AnIdentity(placed)
    );
    assert_eq!(
        inspection.the_declared_request_set(),
        vec!["readiness".to_string(), "health".to_string()]
    );
    let WhatARecordHeld::It(registration) = &inspection.the_boot_registration_record else {
        panic!("the boot-registration record reads back");
    };
    assert_eq!(registration.content, b"[Unit]\nDescription=Smoozy\n".to_vec());
    assert!(!inspection.every_record_is_absent());
    assert!(!inspection.a_record_is_present_and_not_smoozys());

    // **The active-package pointer, planted and read over the subsystem's own
    // two link operations.** The link's own value is what decides which
    // recorded descriptor is the active one, and a resolving read would answer
    // with the staging directory instead.
    a_linux_rule_about_the_rented_machine!(
        "the pointer is a symbolic link, and Microsoft's own port makes a \
         directory link and refuses the ordinary reads of one"
    );
    subsystem
        .plant_a_link(
            staging.as_bytes(),
            area.join(THE_FIXED_ENTRY_POINT_NAME)
                .to_string_lossy()
                .into_owned()
                .as_bytes(),
        )
        .await
        .expect("the pointer is planted");
    let reading = inspect(&mut subsystem, &area_bytes)
        .await
        .expect("the reading answered");
    assert_eq!(
        reading
            .value()
            .the_active_packages_descriptor
            .as_ref()
            .map(|found| found.verified_release_sequence),
        Some(11),
        "the pointer names the staged location, so the requested descriptor is the active one"
    );

    // **A record that is there and does not read back is never the absent
    // branch**, and it is B4's *present but not Smoozy's*.
    std::fs::write(area.join(records::THE_PHASE_RECORD), b"an 82-byte provider default")
        .expect("the record is overwritten");
    let reading = inspect(&mut subsystem, &area_bytes)
        .await
        .expect("the reading answered");
    assert!(reading.value().a_record_is_present_and_not_smoozys());
    assert!(!reading.value().every_record_is_absent());
    assert!(reading.value().the_active_packages_descriptor.is_none());
}

/// **`Rented-server runtime` K16, run end to end: the registration is placed,
/// and the record that says what was placed is read back the way package
/// inspection reads it.**
///
/// **This is the round trip and not a call to the composer.** The unit's text is
/// composed, the record is made from that same text, and **one act writes both**
/// — the record into the package area and the unit at the path the record names.
/// The record then comes back over a real file-transfer server, through the same
/// `inspect` a desktop uses on a rented machine, and says what was placed.
///
/// **What it is worth, said rather than assumed.** Before this, nothing in the
/// workspace wrote that record, so `Rented-server runtime` K15 had nothing to
/// compare against and B4's inspection reported this half absent on every real
/// machine — which is safe and which means **no inspection could ever tell a
/// healthy machine from one whose hook a provider had silently replaced.** The
/// last part of this test is that case: the registration's bytes are replaced by
/// an 82-byte provider default, exactly as the recycle observation records, and
/// the record read back through inspection is the only thing that catches it.
///
/// **Both machine kinds are covered**, because the record is the same act on
/// both: on a container the hook's own install is `Rented-server runtime` K11's
/// elevated step and `Server Setup` owns it, and the record is written here all
/// the same — without it the installed hook could never be confirmed at all.
///
#[cfg(unix)]
#[tokio::test]
async fn the_registration_step_leaves_the_record_package_inspection_reads_back() {
    let _deadline = TheTestsDeadline::started();
    use smoozy_runtime::acting::registration::{where_a_user_unit_sits, TheRegistrationOnTheMachine};
    use smoozy_runtime::registration::{
        confirm, PlacesTheRegistration, ReadsTheRegistrationBack, RegistrationConfirmation,
        WhatTheRegistrationStepLeft,
    };
    use smoozy_runtime::registration_content::{
        the_container_hook, the_record_of, the_systemd_unit, THE_UNIT_NAME,
    };

    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("boot-registration");
    let host_key = scratch.a_key("host");
    let user_key = scratch.a_key("user");
    let machine = a_machine(&host_key, sftp_server).await;
    let connection = a_connection(&machine, &user_key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let area = scratch.at("package-area");
    std::fs::create_dir_all(&area).expect("the area is made");
    let area_bytes = area.to_string_lossy().into_owned().into_bytes();

    // **What every real machine held before this: the record absent.** The
    // reading is honest and it is the never-held answer.
    let reading = inspect(&mut subsystem, &area_bytes)
        .await
        .expect("the reading answered");
    assert!(reading.value().the_boot_registration_record.is_nothing());

    // ------------------------------------------------- the systemd user unit

    // **`Rented-server runtime` G7: the registration names the active-package
    // pointer and no version.**
    let pointer = format!(
        "{}/{THE_FIXED_ENTRY_POINT_NAME}",
        area.to_string_lossy()
    );
    let unit = the_systemd_unit(&pointer).expect("the pointer is admissible");
    let at = where_a_user_unit_sits(&scratch.at("home")).join(THE_UNIT_NAME);
    let record = the_record_of(
        MachineKind::Systemd,
        &at.to_string_lossy().into_owned(),
        &unit,
    );

    let placement = TheRegistrationOnTheMachine::on(
        MachineKind::Systemd,
        &area,
        THE_UNIT_NAME,
        Duration::from_secs(5),
    )
    .expect("this computer answers for its own login");
    assert_eq!(
        placement.place_the_registration(&record),
        Ok(WhatTheRegistrationStepLeft::TheRecordAndTheRegistrationItself),
        "one act writes the record and the unit"
    );

    // **The round trip.** Read back over the real file-transfer server, the way
    // package inspection reads it, and it says what was placed.
    let reading = inspect(&mut subsystem, &area_bytes)
        .await
        .expect("the reading answered");
    let WhatARecordHeld::It(back) = &reading.value().the_boot_registration_record else {
        panic!("the boot-registration record reads back");
    };
    assert_eq!(back, &record);
    assert_eq!(back.content, unit, "the content, by value");
    assert_eq!(back.at_path, at.to_string_lossy().into_owned());
    assert!(!reading.value().every_record_is_absent());
    assert!(!reading.value().a_record_is_present_and_not_smoozys());

    // **What the record is for.** The hook was *"not deleted but replaced"* by
    // an 82-byte provider default at a container recycle, so a check for
    // presence passes while the registration does nothing Smoozy asked for.
    // **`Rented-server runtime` K15 compares content against the record**, and
    // the record it compares against is the one that just came back through
    // inspection.
    std::fs::write(&at, vec![b'x'; 82]).expect("the provider's own default lands");
    let held = placement
        .the_content_at(&back.at_path)
        .expect("it reads back");
    assert_eq!(
        confirm(back, Ok(&held)),
        RegistrationConfirmation::ContentDiffers {
            bytes_on_the_machine: 82,
            bytes_recorded: unit.len(),
        },
        "the record is what separates a healthy machine from a replaced hook"
    );

    // ------------------------------------------------------ the container hook

    // **`Rented-server runtime` K11: the hook's own install is root's and
    // `Server Setup` owns it — and the record is written here all the same**,
    // because `Rented-server runtime` K15's read-back is performed by this same
    // non-root placement on both kinds (`Rented-server runtime` K19).
    let container_area = scratch.at("container-package-area");
    std::fs::create_dir_all(&container_area).expect("the area is made");
    let container_pointer = format!(
        "{}/{THE_FIXED_ENTRY_POINT_NAME}",
        container_area.to_string_lossy()
    );
    let hook = the_container_hook(&container_pointer, "smoozy").expect("both values are admissible");
    let hook_record = the_record_of(MachineKind::Container, "/root/onstart.sh", &hook);
    let container = TheRegistrationOnTheMachine::on(
        MachineKind::Container,
        &container_area,
        THE_UNIT_NAME,
        Duration::from_secs(5),
    )
    .expect("this computer answers for its own login");
    assert_eq!(
        container.place_the_registration(&hook_record),
        Ok(WhatTheRegistrationStepLeft::TheRecordAndTheHookLeftToTheRootStep)
    );

    let reading = inspect(
        &mut subsystem,
        &container_area.to_string_lossy().into_owned().into_bytes(),
    )
    .await
    .expect("the reading answered");
    let WhatARecordHeld::It(back) = &reading.value().the_boot_registration_record else {
        panic!("the boot-registration record reads back on a container too");
    };
    assert_eq!(back, &hook_record);
    assert_eq!(back.content, hook, "the hook's content, by value");
    assert_eq!(back.at_path, "/root/onstart.sh");
}

// ------------------------------------------------------ this test's own deadline

/// **What every test in this file waits on**, named in the message a missed
/// deadline prints.
const WHAT_THE_TESTS_HERE_WAIT_ON: &str =
    "the loopback machine: OpenSSH's own sftp-server and the machine's own shell";

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

//! **Step 4's own milestone, run rather than argued: a file goes up, comes
//! back whole, and resumes correctly after being interrupted.**
//!
//! **What the far end is.** A real SSH connection, opened by Smoozy's own
//! transport on the loopback address, carrying a **subsystem session served by
//! OpenSSH's own `sftp-server`** — the same program the observations behind §7
//! were taken against, and the program a rental's own file-transfer server is.
//! Nothing is rented; nothing on the Creator's machine outside a temporary
//! directory is touched.
//!
//! **What that is worth, and what it is not.** It is the protocol, the
//! extensions, the ordering and the whole of §7's contract against a server
//! this project did not write. **It is not a rented Linux machine**, and it
//! says nothing about a provider's own image — §12 items 3 and 11 hold that
//! ground and it is permanently unresolved.
//!
//! **Where `sftp-server` is not on this system the tests say so and stop.**
//! They do not quietly pass: a printed line names the system and the miss, and
//! the board carries it.
//!
//! **And on Windows some of them say so and stop part of the way through, for
//! a second reason.** §7's head-note is *"Three operating systems on the user's
//! end, Linux alone on the rented end"*, and `Servers` Q13's not-Linux check
//! refuses anything else — so a Windows `sftp-server` standing in as the far
//! end is playing a part it never plays. Three pieces of the contract below are
//! rules about the far end's own filesystem rather than about Smoozy's code,
//! and Windows answers all three differently:
//!
//! - **A resume reads the staged prefix back over a second connection while the
//!   run before it still holds that file open.** Microsoft's own port of
//!   OpenSSH opens a write-only file sharing writes alone, and Windows refuses
//!   a second opener that asks to read it. The same reader on Linux is
//!   ordinary.
//! - **The carriage's removal is the far end's own unlink on a symbolic link.**
//!   That port makes a *directory* symbolic link where the target is a folder,
//!   and its unlink is the one that refuses a directory.
//! - **The deepest-existing-ancestor walk climbs a machine path**, and a
//!   machine path is a Linux path.
//!
//! **What runs before each of those stops still runs on Windows**, because the
//! desktop half of this crate really does run there. **The same shape `Job
//! files and outputs`' own integration tests use**: a printed line naming the
//! ground, and a stop — never a quieter assertion.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Servers` H10, Q13. `Desktop app and delivery`
//! F15a.
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

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::{Duration, Instant};

use russh::server::{Auth, Msg, Server as _, Session};
use russh::{Channel, ChannelId};
use smoozy_server_protocol::carriage::{self, THE_FREE_SPACE_SITE};
use smoozy_server_protocol::free_space;
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::{extension, Announcement};
use smoozy_server_protocol::transfers::integrity::Digest;
use smoozy_server_protocol::transfers::moving::{
    carry_out, carry_out_a_fresh_file_under_a_publication_gate, DrawsFromTheSharedBudget,
    KeepsTheRecord, LocalFailure, NobodyIsWatching, TheCallersPublicationGate, TheUsersOwnComputer,
    Transfer, WatchesTheTransfer, WhatTheGateSaid, THE_ACCOUNTING_PERIOD, THE_CHECKPOINT_INTERVAL,
};
use smoozy_server_protocol::transfers::publication::{
    Intent, PublicationPreState, WhatTheFinalObjectReadsAs,
};
use smoozy_server_protocol::transfers::{
    Checkpoint, Direction, Ending, Report, StableFileIdentity, StoppedReason, TransferRecord,
};
use smoozy_server_protocol::transport::ssh::{open, Connection};
use smoozy_server_protocol::transport::{
    Credential, DecideHostTrust, Endpoint, Figures, HostTrust,
};
use smoozy_state::identities::{Identity, IdentityKind};
use tokio::net::TcpListener;

// ------------------------------------------------- where OpenSSH's own server is

/// The places each system's own OpenSSH puts its file-transfer server.
///
/// **Read from the disk rather than assumed**: the test asks whether the file
/// is there and says so when it is not.
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

/// Says the miss out loud rather than passing quietly.
macro_rules! the_server_or_say_so {
    () => {
        match the_machines_own_file_transfer_server() {
            Some(path) => path,
            None => {
                println!(
                    "SKIPPED: no OpenSSH sftp-server on this system ({}). \
                     The transfer contract is unproved here.",
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
/// **The same shape `Job files and outputs`' own integration tests use**, and
/// on the same ground: §7's head-note, *"Three operating systems on the user's
/// end, Linux alone on the rented end"*, with `Servers` Q13's not-Linux check
/// refusing anything else.
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

// ---------------------------------------------------------------- the machine

struct Machine {
    sftp_server: PathBuf,
    /// **Where each `sftp-server` starts, and it is this test's own throwaway
    /// folder rather than whatever folder the test binary was started in.**
    ///
    /// A file-transfer session resolves every *relative* path against the
    /// login's own area, and a real machine's is that login's home. With no
    /// working directory given, `sftp-server` inherits the test process's,
    /// which Cargo sets to this package's own directory **inside the
    /// repository** — so `Server protocol and transfers` §1.1's carriage
    /// planted its `.smoozy-carriage` folder in the source tree, left it there,
    /// and shared that one fixed path with every run on this desk for ever.
    /// **Anything a stopped run leaves at `.smoozy-carriage/free-space` that a
    /// removal cannot clear then fails the planting with
    /// `TheServerRefused(Failure)` on every run afterwards**, which is what the
    /// workspace run of 2026-09-20 met and what a fresh folder per test ends.
    home: PathBuf,
    /// The channels this client has opened, held rather than dropped.
    ///
    /// **Holding the channel is what keeps the flow control honest**, and it
    /// was learned by watching it fail: a server that drops the channel and
    /// takes its bytes through the handler's own `data` call never sends a
    /// window adjustment, so a transfer stops dead at the channel's opening
    /// window — 200 kilobytes crossed and three megabytes hung. **That is a
    /// property of this test's far end and not of Smoozy's code**, and it is
    /// recorded here because a reader who removes this will reproduce it.
    channels: std::collections::HashMap<ChannelId, Channel<Msg>>,
}

impl Clone for Machine {
    fn clone(&self) -> Machine {
        Machine {
            sftp_server: self.sftp_server.clone(),
            home: self.home.clone(),
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

    /// **The machine's own file-transfer server, on a subsystem session.**
    ///
    /// The channel's own stream is pumped into `sftp-server`'s standard input
    /// and its answers back out — which is what an `sshd` does with the
    /// subsystem line.
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
    host_public_key: Vec<u8>,
    _shut_down: russh::server::RunningServerHandle,
}

async fn a_machine(
    host_key_file: &Path,
    sftp_server: PathBuf,
    scratch: &Scratch,
) -> RunningMachine {
    let host_key = russh::keys::load_secret_key(host_key_file, None).expect("ssh-keygen wrote it");
    let host_public_key = host_key.public_key().to_bytes().expect("it encodes");
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
        home: scratch.the_logins_own_area(),
        channels: std::collections::HashMap::new(),
    }));
    let running = machine.run_on_socket(config, socket);
    let shut_down = running.handle();
    tokio::spawn(running);
    RunningMachine {
        address,
        host_public_key,
        _shut_down: shut_down,
    }
}

// ------------------------------------------------------------------ the desk

struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!(
            "smoozy-transfer-{name}-{}-{}",
            std::process::id(),
            Instant::now().elapsed().as_nanos()
        ));
        std::fs::create_dir_all(&path).expect("a temporary directory can be made");
        Scratch(path)
    }

    fn a_key(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        let made = std::process::Command::new("ssh-keygen")
            .args(["-t", "ed25519", "-N", "", "-C", "smoozy transfer test", "-f"])
            .arg(&path)
            .output()
            .expect("ssh-keygen is on the path");
        assert!(made.status.success());
        path
    }

    fn at(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }

    /// **The folder every session of this scratch's machine starts in**, made
    /// here so that a relative path a session resolves lands in this test's own
    /// throwaway folder and goes with it.
    fn the_logins_own_area(&self) -> PathBuf {
        let at = self.0.join("home");
        std::fs::create_dir_all(&at).expect("the login's own area can be made");
        at
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
    let _ = &machine.host_public_key;
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

// ------------------------------------------------- the user's own computer

/// The desk's stand-in for `Desktop app and delivery`'s platform file adapter.
///
/// **It is not the adapter and does not claim to be.** S18's four requirements
/// are the adapter's, and its per-system demonstration is that document's under
/// Fact 79; what this proves is that §7's contract drives correctly against a
/// local end that keeps them.
struct APlainDisk;

/// **`Desktop app and delivery` F15a's route, per system**, copied here because
/// this crate does not depend on that one and a test crate cannot reach its
/// adapter.
///
/// macOS and Linux: the `st_dev`/`st_ino` pair.
#[cfg(unix)]
fn the_platforms_own_file_identity(path: &Path) -> Option<String> {
    use std::os::unix::fs::MetadataExt;
    let found = std::fs::metadata(path).ok()?;
    Some(format!("{}:{}", found.dev(), found.ino()))
}

/// **Windows: `GetFileInformationByHandleEx` with `FileIdInfo`.** Microsoft's
/// own words for that structure: *"The file identifier and the volume serial
/// number uniquely identify a file on a single computer."* Documented,
/// `FILE_ID_INFO` (winbase.h), read 2026-09-03 at
/// `https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_id_info`.
///
/// **Where the call refuses, the identity is reported unavailable rather than
/// substituted for** — `Desktop app and delivery` F15a's own branch, and the
/// same thing the real adapter does.
#[cfg(windows)]
fn the_platforms_own_file_identity(path: &Path) -> Option<String> {
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        FileIdInfo, GetFileInformationByHandleEx, FILE_FLAG_BACKUP_SEMANTICS, FILE_ID_INFO,
    };

    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)
        .ok()?;

    let mut info: FILE_ID_INFO = unsafe { std::mem::zeroed() };
    let answered = unsafe {
        GetFileInformationByHandleEx(
            file.as_raw_handle() as _,
            FileIdInfo,
            &mut info as *mut _ as *mut core::ffi::c_void,
            std::mem::size_of::<FILE_ID_INFO>() as u32,
        )
    };
    if answered == 0 {
        return None;
    }
    Some(format!(
        "{}:{}",
        info.VolumeSerialNumber,
        u128::from_le_bytes(info.FileId.Identifier)
    ))
}

impl TheUsersOwnComputer for APlainDisk {
    fn staged_size(&mut self, path: &str) -> Option<u64> {
        std::fs::metadata(path).ok().map(|read| read.len())
    }

    fn read_at(&mut self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>, LocalFailure> {
        use std::io::{Read, Seek, SeekFrom};
        let mut file = std::fs::File::open(path).map_err(|why| LocalFailure(why.to_string()))?;
        file.seek(SeekFrom::Start(offset))
            .map_err(|why| LocalFailure(why.to_string()))?;
        let mut buffer = vec![0u8; length];
        let mut filled = 0;
        while filled < length {
            match file.read(&mut buffer[filled..]) {
                Ok(0) => break,
                Ok(read) => filled += read,
                Err(why) => return Err(LocalFailure(why.to_string())),
            }
        }
        buffer.truncate(filled);
        Ok(buffer)
    }

    fn create_staging_exclusively(&mut self, path: &str) -> Result<(), LocalFailure> {
        if let Some(parent) = Path::new(path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map(|_| ())
            .map_err(|why| LocalFailure(why.to_string()))
    }

    fn write_at(&mut self, path: &str, offset: u64, bytes: &[u8]) -> Result<(), LocalFailure> {
        use std::io::{Seek, SeekFrom, Write};
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|why| LocalFailure(why.to_string()))?;
        file.seek(SeekFrom::Start(offset))
            .map_err(|why| LocalFailure(why.to_string()))?;
        file.write_all(bytes)
            .map_err(|why| LocalFailure(why.to_string()))
    }

    fn make_durable(&mut self, path: &str) -> Result<(), LocalFailure> {
        let file = std::fs::File::open(path).map_err(|why| LocalFailure(why.to_string()))?;
        file.sync_all().map_err(|why| LocalFailure(why.to_string()))
    }

    fn the_sources_time(&mut self, source_path: &str) -> Option<i64> {
        let modified = std::fs::metadata(source_path).ok()?.modified().ok()?;
        Some(modified.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64)
    }

    fn keep_the_time(&mut self, staging: &str, seconds: i64) -> Result<(), LocalFailure> {
        let at = std::time::UNIX_EPOCH + Duration::from_secs(seconds.max(0) as u64);
        std::fs::OpenOptions::new()
            .write(true)
            .open(staging)
            .and_then(|file| {
                file.set_times(std::fs::FileTimes::new().set_accessed(at).set_modified(at))
            })
            .map_err(|why| LocalFailure(why.to_string()))
    }

    /// **S18 requirement 4: the identity is supplied where the platform has
    /// one, and all three systems have one.**
    ///
    /// `Desktop app and delivery` F15a names the route per system — the
    /// `st_dev`/`st_ino` pair on macOS and Linux, the volume serial number with
    /// the 128-bit file identifier on Windows — and the real adapter keeps all
    /// three. **This stand-in answered on Unix only**, which made Windows the
    /// one system where S15's recovery had no recorded value it could read a
    /// counterpart for, so the proof could not be composed and a publication
    /// that had plainly happened started over.
    fn stable_identity_of(&mut self, path: &str) -> StableFileIdentity {
        match the_platforms_own_file_identity(Path::new(path)) {
            Some(identity) => StableFileIdentity::The(identity),
            None => StableFileIdentity::TheSupplierCouldNotAnswer,
        }
    }

    fn publish(
        &mut self,
        staging: &str,
        final_path: &str,
        intent: Intent,
    ) -> Result<(), LocalFailure> {
        if intent == Intent::CreateIntended && Path::new(final_path).exists() {
            return Err(LocalFailure("the final name is taken".to_string()));
        }
        std::fs::rename(staging, final_path).map_err(|why| LocalFailure(why.to_string()))
    }

    fn remove(&mut self, path: &str) -> Result<(), LocalFailure> {
        std::fs::remove_file(path).map_err(|why| LocalFailure(why.to_string()))
    }

    fn read_the_final_object(&mut self, path: &str) -> WhatTheFinalObjectReadsAs {
        match std::fs::metadata(path) {
            Ok(read) => WhatTheFinalObjectReadsAs {
                exists: true,
                byte_length: Some(read.len()),
                identity: match self.stable_identity_of(path) {
                    StableFileIdentity::The(identity) => Some(identity),
                    StableFileIdentity::TheSupplierCouldNotAnswer => None,
                },
                at_rest_digest: None,
            },
            Err(_) => WhatTheFinalObjectReadsAs::default(),
        }
    }
}

/// The desk's stand-in for the local record's transfer table.
#[derive(Default)]
struct WritesItDown {
    last: Option<TransferRecord>,
    writes: usize,
}

impl KeepsTheRecord for WritesItDown {
    fn write_it_down(&mut self, record: &TransferRecord) -> Result<(), LocalFailure> {
        self.last = Some(record.clone());
        self.writes += 1;
        Ok(())
    }
}

/// A budget that hands out one chunk and then refuses, so the loop's own
/// draw-and-wait is exercised rather than assumed.
struct ACapThatBites {
    left: usize,
}

impl DrawsFromTheSharedBudget for ACapThatBites {
    fn draw(&mut self, wanted: usize) -> usize {
        let allowed = wanted.min(self.left);
        self.left -= allowed;
        allowed
    }
    fn give_back(&mut self, unused: usize) {
        self.left += unused;
    }
}

/// A local end that keeps every requirement except the one named: **it cannot
/// make the staging file on the user's own disk.**
///
/// This is the full-disk case, stood in for by a refusal rather than by
/// actually filling a disk. **S26's *failed* row, third reason**: no
/// reconnection clears it.
struct ADiskThatCannotMakeTheFile(APlainDisk);

impl TheUsersOwnComputer for ADiskThatCannotMakeTheFile {
    fn staged_size(&mut self, path: &str) -> Option<u64> {
        self.0.staged_size(path)
    }
    fn read_at(&mut self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>, LocalFailure> {
        self.0.read_at(path, offset, length)
    }
    fn create_staging_exclusively(&mut self, _path: &str) -> Result<(), LocalFailure> {
        Err(LocalFailure("no space left on device".to_string()))
    }
    fn write_at(&mut self, path: &str, offset: u64, bytes: &[u8]) -> Result<(), LocalFailure> {
        self.0.write_at(path, offset, bytes)
    }
    fn make_durable(&mut self, path: &str) -> Result<(), LocalFailure> {
        self.0.make_durable(path)
    }
    fn stable_identity_of(&mut self, path: &str) -> StableFileIdentity {
        self.0.stable_identity_of(path)
    }
    fn publish(
        &mut self,
        staging: &str,
        final_path: &str,
        intent: Intent,
    ) -> Result<(), LocalFailure> {
        self.0.publish(staging, final_path, intent)
    }
    fn remove(&mut self, path: &str) -> Result<(), LocalFailure> {
        self.0.remove(path)
    }
    fn read_the_final_object(&mut self, path: &str) -> WhatTheFinalObjectReadsAs {
        self.0.read_the_final_object(path)
    }
    fn the_sources_time(&mut self, source_path: &str) -> Option<i64> {
        self.0.the_sources_time(source_path)
    }
    fn keep_the_time(&mut self, staging: &str, seconds: i64) -> Result<(), LocalFailure> {
        self.0.keep_the_time(staging, seconds)
    }
}

/// A watcher that counts what it was told and when, so **G4's bound in time**
/// can be observed rather than read.
#[derive(Default)]
struct CountsWhatItIsTold {
    reports: usize,
}

impl WatchesTheTransfer for CountsWhatItIsTold {
    fn report(&mut self, _report: &Report) {
        self.reports += 1;
    }
}

fn a_file_of(bytes: usize) -> Vec<u8> {
    // Not a repeated byte: a hybrid of two versions is exactly what §7 exists
    // against, so the contents have to be able to show one.
    (0..bytes).map(|at| (at % 251) as u8).collect()
}

fn an_upload(
    scratch: &Scratch,
    source: &Path,
    destination: &Path,
    size: u64,
    digest: Digest,
) -> TransferRecord {
    let issue = |kind| Identity::issue(kind).expect("random bytes");
    let staging_directory = destination
        .parent()
        .expect("a destination has a folder")
        .join(".smoozy-staging");
    let _ = scratch;
    let mut record = TransferRecord::arranged(
        issue(IdentityKind::Transfer),
        Direction::Up,
        issue(IdentityKind::LocalServer),
        Some(issue(IdentityKind::Installation)),
        source.to_string_lossy().into_owned(),
        destination.to_string_lossy().into_owned(),
        &staging_directory.to_string_lossy(),
        size,
        issue(IdentityKind::Run),
    );
    record.expected_digest = Some(digest);
    record
}

/// The download's own arrangement, so the direction that carries §7.4's gate
/// can be driven from more than one test.
fn a_download(
    on_the_machine: &Path,
    here: &Path,
    size: u64,
    digest: Option<Digest>,
) -> TransferRecord {
    let issue = |kind| Identity::issue(kind).expect("random bytes");
    let staging_directory = here.parent().expect("a folder").join(".smoozy-staging");
    let mut record = TransferRecord::arranged(
        issue(IdentityKind::Transfer),
        Direction::Down,
        issue(IdentityKind::LocalServer),
        Some(issue(IdentityKind::Installation)),
        on_the_machine.to_string_lossy().into_owned(),
        here.to_string_lossy().into_owned(),
        &staging_directory.to_string_lossy(),
        size,
        issue(IdentityKind::Run),
    );
    record.expected_digest = digest;
    record
}

// --------------------------------------------------------------- the milestone

/// **The first half of step 4's milestone: a file goes up.**
#[tokio::test]
async fn a_file_goes_up_and_the_final_name_holds_it_whole() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("up");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    // The machine's own server announces revision 3 and the three extensions
    // §7 leans on. Asserted rather than assumed — §1.2's whole point.
    assert_eq!(
        subsystem.announcement().version,
        Announcement::REVISION_THIS_DOCUMENT_USES
    );
    assert!(subsystem.announcement().announces(extension::POSIX_RENAME));
    assert!(subsystem.announcement().announces(extension::FSYNC));
    assert!(subsystem.announcement().announces(extension::STATVFS));

    let bytes = a_file_of(3_145_728);
    let source = scratch.at("scene.blend");
    std::fs::write(&source, &bytes).expect("the scratch takes a file");
    let destination = scratch.at("machine/scene.blend");
    std::fs::create_dir_all(destination.parent().expect("a folder")).expect("made");

    let mut record = an_upload(
        &scratch,
        &source,
        &destination,
        bytes.len() as u64,
        Digest::of(&bytes),
    );
    let mut local = APlainDisk;
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
    )
    .await;

    assert_eq!(happened.ending, Ending::Published);
    // **7.1 clause 1 kept, and the whole file at the final name.**
    let landed = std::fs::read(&destination).expect("the final name holds it");
    assert_eq!(landed.len(), bytes.len());
    assert_eq!(Digest::of(&landed), Digest::of(&bytes));
    // **Observed: the staging name was gone**, the rename being the cleanup.
    assert!(!Path::new(&record.staging_path.as_text()).exists());
    // **S8: the record was durable before the first byte**, and it was written
    // more than once — the first write is the one that matters.
    assert!(keeper.writes >= 2);
}

/// **The second half: it comes back whole.**
#[tokio::test]
async fn a_file_comes_back_whole() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("down");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(2_097_152);
    let on_the_machine = scratch.at("machine/frame.exr");
    std::fs::create_dir_all(on_the_machine.parent().expect("a folder")).expect("made");
    std::fs::write(&on_the_machine, &bytes).expect("written");

    let here = scratch.at("downloads/frame.exr");
    std::fs::create_dir_all(here.parent().expect("a folder")).expect("made");

    let issue = |kind| Identity::issue(kind).expect("random bytes");
    let staging_directory = here.parent().expect("a folder").join(".smoozy-staging");
    let mut record = TransferRecord::arranged(
        issue(IdentityKind::Transfer),
        Direction::Down,
        issue(IdentityKind::LocalServer),
        Some(issue(IdentityKind::Installation)),
        on_the_machine.to_string_lossy().into_owned(),
        here.to_string_lossy().into_owned(),
        &staging_directory.to_string_lossy(),
        bytes.len() as u64,
        issue(IdentityKind::Run),
    );
    record.expected_digest = Some(Digest::of(&bytes));

    let mut local = APlainDisk;
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
    )
    .await;

    assert_eq!(happened.ending, Ending::Published);
    let landed = std::fs::read(&here).expect("the final name holds it");
    assert_eq!(Digest::of(&landed), Digest::of(&bytes));
    assert!(!Path::new(&record.staging_path.as_text()).exists());
}

// ------------------------------------------- S15c: a transfer keeps the file's time

/// **A modification time months before the test runs**, in whole seconds,
/// because whole seconds are all the subsystem's set-attributes call carries.
const A_TIME_LONG_AGO: u64 = 1_767_225_600;

fn a_time_long_ago() -> std::time::SystemTime {
    std::time::UNIX_EPOCH + Duration::from_secs(A_TIME_LONG_AGO)
}

/// Gives a file the old time, as the user's own editor would have left it.
fn made_long_ago(path: &Path) {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .open(path)
        .expect("the scratch file opens");
    file.set_times(
        std::fs::FileTimes::new()
            .set_accessed(a_time_long_ago())
            .set_modified(a_time_long_ago()),
    )
    .expect("the scratch takes a time");
}

fn the_time_of(path: &Path) -> std::time::SystemTime {
    std::fs::metadata(path)
        .and_then(|read| read.modified())
        .expect("the file has a time")
}

/// **S15c, the upload half: the file on the server reads the source's own
/// time**, and not the moment it went up.
#[tokio::test]
async fn an_uploaded_file_keeps_the_sources_own_time() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("up-time");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(1_048_576);
    let source = scratch.at("texture.png");
    std::fs::write(&source, &bytes).expect("the scratch takes a file");
    made_long_ago(&source);
    let destination = scratch.at("machine/texture.png");
    std::fs::create_dir_all(destination.parent().expect("a folder")).expect("made");

    let mut record = an_upload(
        &scratch,
        &source,
        &destination,
        bytes.len() as u64,
        Digest::of(&bytes),
    );
    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut APlainDisk,
            budget: &mut smoozy_server_protocol::transfers::moving::NoCapIsInForce,
            keeper: &mut WritesItDown::default(),
            watcher: &mut NobodyIsWatching,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
    )
    .await;

    assert_eq!(happened.ending, Ending::Published);
    assert_eq!(the_time_of(&destination), a_time_long_ago());
}

/// **S15c, the download half: the file on this computer reads the server
/// file's time** — a frame reads the time it was rendered.
#[tokio::test]
async fn a_downloaded_file_keeps_the_servers_own_time() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("down-time");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(1_048_576);
    let on_the_machine = scratch.at("machine/frame.exr");
    std::fs::create_dir_all(on_the_machine.parent().expect("a folder")).expect("made");
    std::fs::write(&on_the_machine, &bytes).expect("written");
    made_long_ago(&on_the_machine);
    let here = scratch.at("downloads/frame.exr");
    std::fs::create_dir_all(here.parent().expect("a folder")).expect("made");

    let mut record = a_download(&on_the_machine, &here, bytes.len() as u64, Some(Digest::of(&bytes)));
    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut APlainDisk,
            budget: &mut smoozy_server_protocol::transfers::moving::NoCapIsInForce,
            keeper: &mut WritesItDown::default(),
            watcher: &mut NobodyIsWatching,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
    )
    .await;

    assert_eq!(happened.ending, Ending::Published);
    assert_eq!(the_time_of(&here), a_time_long_ago());
}

/// **S15c's *when*: the time is set before S15's pre-state**, so the
/// staged file the pre-state names already carries the source's time, and the
/// published file a server-side check reads at once — size, inode and
/// modification time of S13's four — is that same object with that same time.
/// The keeper reads the staged file on the loopback machine at the moment the
/// pre-state is written down.
#[cfg(unix)]
#[tokio::test]
async fn a_file_read_just_after_publication_already_carries_the_sources_time() {
    use std::os::unix::fs::MetadataExt;
    struct ReadsTheStagedFileAtThePreState {
        at_the_pre_state: Option<(u64, u64, i64)>,
    }
    impl KeepsTheRecord for ReadsTheStagedFileAtThePreState {
        fn write_it_down(&mut self, record: &TransferRecord) -> Result<(), LocalFailure> {
            if let (Some(pending), None) = (&record.publication_pending, self.at_the_pre_state) {
                let read = std::fs::metadata(&pending.staging_path).expect("the staged file reads");
                self.at_the_pre_state = Some((read.len(), read.ino(), read.mtime()));
            }
            Ok(())
        }
    }

    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("up-time-identity");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(262_144);
    let source = scratch.at("cloth.vdb");
    std::fs::write(&source, &bytes).expect("the scratch takes a file");
    made_long_ago(&source);
    let destination = scratch.at("machine/cloth.vdb");
    std::fs::create_dir_all(destination.parent().expect("a folder")).expect("made");

    let mut record = an_upload(
        &scratch,
        &source,
        &destination,
        bytes.len() as u64,
        Digest::of(&bytes),
    );
    let mut keeper = ReadsTheStagedFileAtThePreState {
        at_the_pre_state: None,
    };
    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut APlainDisk,
            budget: &mut smoozy_server_protocol::transfers::moving::NoCapIsInForce,
            keeper: &mut keeper,
            watcher: &mut NobodyIsWatching,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
    )
    .await;
    assert_eq!(happened.ending, Ending::Published);

    let at_the_pre_state = keeper.at_the_pre_state.expect("the pre-state was written down");
    assert_eq!(at_the_pre_state.2, A_TIME_LONG_AGO as i64);
    let published = std::fs::metadata(&destination).expect("the published file reads");
    assert_eq!(
        (published.len(), published.ino(), published.mtime()),
        at_the_pre_state
    );
}

/// **The third half, and it is the one worth watching: it resumes correctly
/// after being interrupted.**
///
/// The first run is stopped part-way by a budget that runs out; the record is
/// left *interrupted* with its staged material kept, and the second run verifies
/// the staged prefix against the record's own checkpoint and carries on from it.
#[tokio::test]
async fn an_interrupted_upload_resumes_from_its_verified_checkpoint() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("resume");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(12 * 1024 * 1024);
    let source = scratch.at("big.blend");
    std::fs::write(&source, &bytes).expect("written");
    // **S15c: the resume is untouched and the time is still kept**, set once,
    // at the end.
    made_long_ago(&source);
    let destination = scratch.at("machine/big.blend");
    std::fs::create_dir_all(destination.parent().expect("a folder")).expect("made");

    let mut record = an_upload(
        &scratch,
        &source,
        &destination,
        bytes.len() as u64,
        Digest::of(&bytes),
    );
    let mut local = APlainDisk;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    // ---- The first run, cut short after one checkpoint's worth has crossed.
    let mut a_cap = ACapThatBites {
        left: 5 * 1024 * 1024,
    };
    let first = tokio::time::timeout(
        Duration::from_secs(20),
        carry_out(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut local,
                budget: &mut a_cap,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(30),
                intent: Intent::ReplaceIntended,
            },
            &mut record,
        ),
    )
    .await;
    // The loop is waiting on a budget that will never give again, which is
    // R7's *waits and does not spin*; the timeout is the test's own stop.
    assert!(first.is_err(), "the capped run does not finish");

    // What is on the machine now is the staged prefix and nothing at the final
    // name — **7.1 clause 1, mid-transfer.**
    let staged = subsystem
        .stat(record.staging_path.as_bytes().as_slice())
        .await
        .expect("the staging path is there");
    let staged_size = staged.size.expect("a size comes back");
    assert!(staged_size >= 4 * 1024 * 1024);
    assert!(
        subsystem
            .stat(destination.to_string_lossy().as_bytes())
            .await
            .is_err(),
        "the final name is absent until publication"
    );
    // The record carries a checkpoint the disk actually reaches.
    let checkpoint = record
        .highest_checkpoint_at_or_below(staged_size)
        .expect("a checkpoint at or below the staged size")
        .offset;
    assert!(checkpoint > 0);

    // Everything above is Smoozy's own loop against a real server and it runs
    // on every system. **What follows is not.** The resume reads this staged
    // prefix back over a second connection while the run above still holds the
    // file open, and on Windows the far end refuses that reader — its own
    // server opened the file for writing alone and Windows will not let a
    // second opener read it. On the far end this document has, it is ordinary.
    a_linux_rule_about_the_rented_machine!(
        "a resume reads the staged prefix back while the run before it still \
         holds that file open, and this far end refuses the reader"
    );

    // ---- The second run, on a fresh connection, resuming.
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let attempt_before = record.attempt;
    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::ReplaceIntended,
        },
        &mut record,
    )
    .await;

    assert_eq!(happened.ending, Ending::Published);
    // **It resumed rather than starting over**: the attempt number did not move
    // and the staging path is the one the first run wrote.
    assert_eq!(record.attempt, attempt_before);
    // **And the file is whole** — which is what a resume that trusted an
    // unverified partial would not give.
    let landed = std::fs::read(&destination).expect("the final name holds it");
    assert_eq!(landed.len(), bytes.len());
    assert_eq!(Digest::of(&landed), Digest::of(&bytes));
    assert_eq!(the_time_of(&destination), a_time_long_ago());
}

/// **A gate that lets every publication through** — for a caller that says
/// its file is fresh (S12a), whose entry point takes
/// one.
struct LetsItThrough;

impl TheCallersPublicationGate for LetsItThrough {
    fn may_this_be_published(&mut self, _what_was_sent: &Digest) -> WhatTheGateSaid {
        WhatTheGateSaid::ItMayBePublished
    }
}

/// **What a run reported about verifying a staged prefix**, and the lowest
/// count it reported — so a test can tell a carry-on from a start from nought
/// into the same staging name.
#[derive(Default)]
struct WatchesTheResume {
    verified_a_prefix_of: Option<u64>,
    lowest_count: Option<u64>,
}

impl WatchesTheTransfer for WatchesTheResume {
    fn report(&mut self, report: &Report) {
        if report.verifying_a_prefix_of.is_some() {
            self.verified_a_prefix_of = report.verifying_a_prefix_of;
        }
        self.lowest_count = Some(
            self.lowest_count
                .map_or(report.bytes_staged, |lowest| lowest.min(report.bytes_staged)),
        );
    }
}

/// **A staged part on record is carried on even where the caller says the
/// file is fresh** — S12a (the build runner's ruling of 2026-10-16): the skip of the
/// staging folder, the staged part's reading and its removal must never lose
/// a resume, so the loop reads the record as well as the caller's word.
#[tokio::test]
async fn a_staged_part_on_record_is_carried_on_even_where_the_caller_says_fresh() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("fresh-resume");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(12 * 1024 * 1024);
    let source = scratch.at("big.blend");
    std::fs::write(&source, &bytes).expect("written");
    let destination = scratch.at("machine/big.blend");
    std::fs::create_dir_all(destination.parent().expect("a folder")).expect("made");
    let mut record = an_upload(
        &scratch,
        &source,
        &destination,
        bytes.len() as u64,
        Digest::of(&bytes),
    );
    let mut local = APlainDisk;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    // ---- The first run, cut short after one checkpoint's worth has crossed.
    let mut a_cap = ACapThatBites {
        left: 5 * 1024 * 1024,
    };
    let first = tokio::time::timeout(
        Duration::from_secs(20),
        carry_out(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut local,
                budget: &mut a_cap,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(30),
                intent: Intent::CreateIntended,
            },
            &mut record,
        ),
    )
    .await;
    assert!(first.is_err(), "the capped run does not finish");
    assert!(
        !record.checkpoints.is_empty(),
        "the record carries a staged part"
    );
    a_linux_rule_about_the_rented_machine!(
        "a resume reads the staged prefix back while the run before it still \
         holds that file open, and this far end refuses the reader"
    );

    // ---- The second run, on a fresh connection, its caller saying fresh.
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let attempt_before = record.attempt;
    let staging_before = record.staging_path.as_text();
    let checkpoint = record.checkpoints.last().expect("a checkpoint").offset;
    let mut resumed = WatchesTheResume::default();
    let happened = carry_out_a_fresh_file_under_a_publication_gate(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut resumed,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
        &mut LetsItThrough,
    )
    .await;

    assert_eq!(happened.ending, Ending::Published);
    // **It carried on rather than starting over**: the staged prefix was
    // read back and verified, the count the loop reported never fell below
    // the checkpoint, the attempt did not move, and the staging path is the
    // one the first run wrote.
    assert_eq!(resumed.verified_a_prefix_of, Some(checkpoint));
    assert!(
        resumed.lowest_count >= Some(checkpoint),
        "the count fell to {:?}, below the checkpoint at {checkpoint}: the file started from nought",
        resumed.lowest_count
    );
    assert_eq!(record.attempt, attempt_before);
    assert_eq!(record.staging_path.as_text(), staging_before);
    assert!(record.superseded_attempts.is_empty());
    let landed = std::fs::read(&destination).expect("the final name holds it");
    assert_eq!(Digest::of(&landed), Digest::of(&bytes));
}

/// **A fresh file whose staging folder is not there after all is still
/// published** — S12a: where the exclusive create is
/// refused, the folder is made, the name removed and the create made again,
/// so a reading that did not hold costs round trips and nothing else.
#[tokio::test]
async fn a_fresh_file_whose_staging_folder_is_gone_makes_it_and_is_published() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("fresh-gone");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(64 * 1024);
    let source = scratch.at("small.blend");
    std::fs::write(&source, &bytes).expect("written");
    let destination = scratch.at("machine/small.blend");
    std::fs::create_dir_all(destination.parent().expect("a folder")).expect("made");
    let mut record = an_upload(
        &scratch,
        &source,
        &destination,
        bytes.len() as u64,
        Digest::of(&bytes),
    );
    // **The caller says its request made the folder, and it is not there.**
    assert!(!Path::new(&record.staging_path.directory()).exists());
    let mut local = APlainDisk;
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;
    let happened = carry_out_a_fresh_file_under_a_publication_gate(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
        &mut LetsItThrough,
    )
    .await;

    assert_eq!(happened.ending, Ending::Published);
    assert_eq!(record.attempt, 1);
    let landed = std::fs::read(&destination).expect("the final name holds it");
    assert_eq!(Digest::of(&landed), Digest::of(&bytes));
}

/// **A staged prefix that does not match its recorded digest starts the
/// transfer over rather than being carried on** — S12 step 5, driven against a
/// real staging file somebody else has written into.
#[tokio::test]
async fn a_staged_prefix_that_was_tampered_with_starts_over() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("tamper");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(9 * 1024 * 1024);
    let source = scratch.at("scene.blend");
    std::fs::write(&source, &bytes).expect("written");
    let destination = scratch.at("machine/scene.blend");
    std::fs::create_dir_all(destination.parent().expect("a folder")).expect("made");

    let mut record = an_upload(
        &scratch,
        &source,
        &destination,
        bytes.len() as u64,
        Digest::of(&bytes),
    );
    let mut local = APlainDisk;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    let mut a_cap = ACapThatBites {
        left: 5 * 1024 * 1024,
    };
    let _ = tokio::time::timeout(
        Duration::from_secs(20),
        carry_out(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut local,
                budget: &mut a_cap,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(30),
                intent: Intent::ReplaceIntended,
            },
            &mut record,
        ),
    )
    .await;

    // **The same stop as the resume test's, on the same ground**: what follows
    // is a start-over decided by reading the staged prefix back over a second
    // connection, and this far end refuses that reader while the run above
    // still holds the file open.
    a_linux_rule_about_the_rented_machine!(
        "the start-over is decided by reading the staged prefix back while the \
         run before it still holds that file open, and this far end refuses \
         the reader"
    );

    // Four bytes overwritten in place inside the verified prefix — Fact 72's
    // own case, which travelled through `scp` at exit 0.
    let staged = record.staging_path.as_text();
    let handle = subsystem
        .open_to_carry_on(staged.as_bytes())
        .await
        .expect("the staging path opens");
    subsystem
        .send_a_write(&handle, 1024, b"oops")
        .await
        .expect("the write goes");
    subsystem
        .settle_the_writes()
        .await
        .expect("the write settles");
    subsystem.close(&handle).await.expect("closed");

    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let attempt_before = record.attempt;
    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::ReplaceIntended,
        },
        &mut record,
    )
    .await;

    assert_eq!(happened.ending, Ending::Published);
    // **The attempt number moved**, which is S12 step 5's start-over into a
    // fresh staging path rather than a repair in place.
    assert_eq!(record.attempt, attempt_before + 1);
    let landed = std::fs::read(&destination).expect("the final name holds it");
    assert_eq!(Digest::of(&landed), Digest::of(&bytes));
    // **And the superseded path stays derivable**, which is what S4's attempt
    // number was added for.
    assert!(record
        .every_staging_path_it_has_named()
        .iter()
        .any(|path| path.as_text() != record.staging_path.as_text()));
}

/// **H10's free space, taken through F2's first route, per path.**
#[tokio::test]
async fn free_space_is_read_per_path_and_names_the_path_it_was_taken_against() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("space");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let jobs = scratch.at("jobs");
    std::fs::create_dir_all(&jobs).expect("made");
    let reading = free_space::read_it(&mut subsystem, &jobs.to_string_lossy())
        .await
        .expect("the extension answers");
    assert!(reading.available_bytes > 0);
    assert_eq!(
        reading.the_path_it_was_taken_against,
        jobs.to_string_lossy()
    );
    assert_eq!(reading.route, free_space::WhichRoute::TheStatvfsExtension);

    // **H10's own rule for a path that does not exist yet**: the reading is
    // taken against the deepest existing ancestor and the record names the path
    // it was actually taken against.
    //
    // **The walk climbs a machine path, and a machine path is a Linux path.**
    // The scratch folder below stands in for one, so on Windows it is not a
    // path this rule is about at all. The walk's own steps are held by the
    // desk tests in `free_space`.
    a_linux_rule_about_the_rented_machine!(
        "the deepest-existing-ancestor walk climbs a machine path, and a \
         machine path is a Linux path"
    );
    let not_yet = jobs.join("a/b/c");
    let ancestor_reading =
        free_space::read_it_at_the_deepest_ancestor_that_exists(&mut subsystem, &not_yet.to_string_lossy())
            .await
            .expect("an ancestor answers");
    assert_ne!(
        ancestor_reading.the_path_it_was_taken_against,
        not_yet.to_string_lossy()
    );
    assert!(not_yet
        .to_string_lossy()
        .starts_with(&ancestor_reading.the_path_it_was_taken_against));
}

/// **§1.1's planted-link carriage, performed and read back.**
///
/// The value crosses as a protocol field; the constant command names the token;
/// and the landing is checked at the exact target path rather than by the
/// command's exit alone.
#[tokio::test]
async fn a_machine_origin_path_crosses_as_a_protocol_field_and_the_command_names_the_token() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("carriage");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    // A target with every hazard §1.1 exists against: a leading dash, a space
    // and a newline in the name of the folder it points at.
    let hostile = scratch.at("-a folder with a space");
    std::fs::create_dir_all(&hostile).expect("made");
    let target = hostile.to_string_lossy().into_owned();

    let mut planted = carriage::plant(&mut subsystem, THE_FREE_SPACE_SITE, target.as_bytes())
        .await
        .expect("the link is planted");
    assert_eq!(planted.target, target.as_bytes());

    // The constant command names the token and never the value. It is a
    // `&'static str` in this binary, which is the mechanism.
    let naming_the_token = carriage::a_command_naming("ls -d -- ~/.smoozy-carriage/free-space");
    assert!(naming_the_token
        .as_str()
        .contains(THE_FREE_SPACE_SITE.link_path_on_the_subsystem));
    assert!(!naming_the_token.as_str().contains(&target));

    // **The landing, checked at the exact target path.** A file written through
    // the token appears at the target, which is what says the link carried the
    // value the subsystem put in it.
    let through_the_token = format!("{}/landed", THE_FREE_SPACE_SITE.link_path_on_the_subsystem);
    let handle = subsystem
        .create_exclusively(through_the_token.as_bytes())
        .await
        .expect("the token resolves to a folder that takes a file");
    subsystem
        .send_a_write(&handle, 0, b"it landed")
        .await
        .expect("the write goes");
    subsystem.settle_the_writes().await.expect("it settles");
    subsystem.close(&handle).await.expect("closed");
    assert_eq!(
        std::fs::read(hostile.join("landed")).expect("at the exact target path"),
        b"it landed"
    );

    // **The removal is the far end's own unlink on a symbolic link.** Where the
    // target is a folder, Microsoft's own port makes a *directory* symbolic
    // link, and the unlink it calls is the one that refuses a directory — so
    // the removal comes back owed rather than done. That is §7.8 working, not
    // failing, but it is a statement about a far end this document never has.
    // **The link is left standing on such a machine**; the next plant at this
    // token replaces it.
    a_linux_rule_about_the_rented_machine!(
        "taking the carriage away is the far end's own unlink on a symbolic \
         link, and this far end makes one its unlink refuses"
    );

    carriage::take_it_away(&mut subsystem, &mut planted).await;
    assert!(!planted.the_removal_is_owed());
}

// ---------------------------------------- what the code round of 2026-09-03 fixed
//
// **Six tests, each the one that would have caught its own fault.** Every fault
// below was a decision function that was written, was right, had tests of its
// own, and was never asked by the byte loop — so the crate read as though the
// clause were kept. A test at this level is the only one that can tell the
// difference, because it drives the loop rather than the function.

/// **§7.4's unchanged-source gate is asked on a download resume, and today it
/// cannot be confirmed, so the download starts over.**
///
/// **The staged prefix here is good** — it is the file's own first four
/// mebibytes with the checkpoint digest to match — so without the gate the
/// resume carries on from it and the attempt number does not move. That is
/// exactly the hole: the four values were never compared, so a machine that had
/// rewritten the file overnight would have been spliced into the middle of the
/// user's own copy at full size with nothing said.
///
/// **The upload half is deliberately untouched**, and
/// `an_interrupted_upload_resumes_from_its_verified_checkpoint` above is what
/// holds it: that transfer resumes with its attempt number unmoved.
#[tokio::test]
async fn a_download_resume_asks_the_source_gate_and_starts_over_rather_than_carrying_on() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("gate");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(6 * 1024 * 1024);
    let on_the_machine = scratch.at("machine/frame.exr");
    std::fs::create_dir_all(on_the_machine.parent().expect("a folder")).expect("made");
    std::fs::write(&on_the_machine, &bytes).expect("written");
    let here = scratch.at("downloads/frame.exr");
    std::fs::create_dir_all(here.parent().expect("a folder")).expect("made");

    let mut record = a_download(
        &on_the_machine,
        &here,
        bytes.len() as u64,
        Some(Digest::of(&bytes)),
    );

    // A staged prefix that **would** verify, with the checkpoint that matches
    // it — the state a resume is supposed to carry on from.
    let staged = record.staging_path.as_text();
    std::fs::create_dir_all(Path::new(&staged).parent().expect("a folder")).expect("made");
    let prefix = THE_CHECKPOINT_INTERVAL as usize;
    std::fs::write(&staged, &bytes[..prefix]).expect("the staged prefix");
    record.checkpoints.push(Checkpoint {
        offset: prefix as u64,
        digest_of_everything_up_to_it: Digest::of(&bytes[..prefix]),
    });
    let the_path_it_would_have_carried_on = staged.clone();

    let mut local = APlainDisk;
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
    )
    .await;

    // **The attempt number moved**, which is the start-over the gate's
    // *could not be confirmed* branch requires.
    assert_eq!(record.attempt, 2, "the resume did not happen");
    assert!(record
        .superseded_attempts
        .iter()
        .any(|superseded| superseded.attempt == 1));
    assert_ne!(record.staging_path.as_text(), the_path_it_would_have_carried_on);
    // **And the count truthfully says a file started over**, which is G4a's
    // mark rather than a number that quietly falls.
    assert!(happened.report.started_over.is_some());
    // It costs the user the fetch again and it cannot hand them a spliced file.
    assert_eq!(happened.ending, Ending::Published);
    let landed = std::fs::read(&here).expect("the final name holds it");
    assert_eq!(Digest::of(&landed), Digest::of(&bytes));
}

/// **A download whose source ends short starts over rather than stopping.**
///
/// A short read used to be reported as *the source changed under it*, which is
/// the **upload's** answer — it carries `File Browser`'s offer of the new
/// version, and that means nothing about a file on the machine. The download's
/// answer is a start-over into a fresh staging path with the transfer neither
/// failed nor ended.
#[tokio::test]
async fn a_download_whose_source_ends_short_starts_over_rather_than_stopping() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("short");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    // The record says three mebibytes and the machine holds one.
    let bytes = a_file_of(1024 * 1024);
    let on_the_machine = scratch.at("machine/frame.exr");
    std::fs::create_dir_all(on_the_machine.parent().expect("a folder")).expect("made");
    std::fs::write(&on_the_machine, &bytes).expect("written");
    let here = scratch.at("downloads/frame.exr");
    std::fs::create_dir_all(here.parent().expect("a folder")).expect("made");

    let mut record = a_download(&on_the_machine, &here, 3 * 1024 * 1024, None);

    let mut local = APlainDisk;
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
    )
    .await;

    // **It started over**, into the fresh staging path the second attempt
    // number synthesises.
    assert_eq!(record.attempt, 2);
    // **And it is not the upload's answer.** *Stopped, the source changed under
    // it* would remove the material and offer a new version of a file on a
    // machine, which is not a thing this direction has.
    assert_ne!(
        happened.ending,
        Ending::Stopped(StoppedReason::TheSourceChangedUnderIt)
    );
    // The second attempt met the same short source at the same place, so the
    // run ends rather than turning for ever — and *interrupted* keeps the
    // material, which is what it means.
    assert_eq!(happened.ending, Ending::Interrupted);
    assert!(Path::new(&record.staging_path.as_text()).exists());
}

/// **A local failure a reconnection cannot clear is *failed*, not
/// *interrupted*.**
///
/// This is the full disk. Under *interrupted* it composed to **waiting for the
/// connection** on a perfect line: the user checked their internet, restarted
/// their router, and nothing anywhere said the one thing they could act on.
#[tokio::test]
async fn a_local_failure_a_reconnection_cannot_clear_is_failed_and_not_interrupted() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("fulldisk");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(1024 * 1024);
    let on_the_machine = scratch.at("machine/frame.exr");
    std::fs::create_dir_all(on_the_machine.parent().expect("a folder")).expect("made");
    std::fs::write(&on_the_machine, &bytes).expect("written");
    let here = scratch.at("downloads/frame.exr");
    std::fs::create_dir_all(here.parent().expect("a folder")).expect("made");

    let mut record = a_download(
        &on_the_machine,
        &here,
        bytes.len() as u64,
        Some(Digest::of(&bytes)),
    );

    let mut local = ADiskThatCannotMakeTheFile(APlainDisk);
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
    )
    .await;

    assert_eq!(happened.ending, Ending::Failed);
    assert_ne!(happened.ending, Ending::Interrupted);
    // **The record says so durably**, which is what the next reconnection reads
    // instead of picking the work up again.
    assert_eq!(record.ending, Some(Ending::Failed));
    assert!(!here.exists(), "nothing reached the final name");
}

/// **The publication recovery is asked, instead of the rename being retried
/// from a path the rename itself took away.**
///
/// The judgement was written and tested and called by nothing. A publication
/// that landed and was not recorded was retried for ever against a staging file
/// that was already gone, reporting *interrupted* every time on a live line.
#[tokio::test]
async fn a_publication_whose_staging_is_gone_is_judged_rather_than_retried_from_it() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("recovery");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(64 * 1024);
    let on_the_machine = scratch.at("machine/frame.exr");
    std::fs::create_dir_all(on_the_machine.parent().expect("a folder")).expect("made");
    std::fs::write(&on_the_machine, &bytes).expect("written");
    let here = scratch.at("downloads/frame.exr");
    std::fs::create_dir_all(here.parent().expect("a folder")).expect("made");

    // **The world after a crash between the rename and the outcome write**: the
    // final name holds the whole file and the staging path is gone.
    std::fs::write(&here, &bytes).expect("the rename had already happened");
    let mut local = APlainDisk;
    let identity = local.stable_identity_of(&here.to_string_lossy());

    let mut record = a_download(
        &on_the_machine,
        &here,
        bytes.len() as u64,
        Some(Digest::of(&bytes)),
    );
    record.publication_pending = Some(PublicationPreState {
        intent: Intent::CreateIntended,
        staging_path: record.staging_path.as_text(),
        final_path: here.to_string_lossy().into_owned(),
        staged_identity: identity,
        staged_byte_length: bytes.len() as u64,
        staged_digest: Digest::of(&bytes),
    });
    assert!(
        !Path::new(&record.staging_path.as_text()).exists(),
        "the rename took the staging name away"
    );

    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    let happened = carry_out(
        Transfer {
            subsystem: &mut subsystem,
            local: &mut local,
            budget: &mut budget,
            keeper: &mut keeper,
            watcher: &mut watcher,
            inactivity_deadline: Duration::from_secs(30),
            intent: Intent::CreateIntended,
        },
        &mut record,
    )
    .await;

    // **The exact match with the evidence recorded before the rename is what
    // proves the object crossed**, and nothing else would have.
    assert_eq!(happened.ending, Ending::Published);
    assert_eq!(record.publication_pending, None);
    assert_eq!(record.attempt, 1, "nothing started over");
    assert_eq!(
        Digest::of(&std::fs::read(&here).expect("still there")),
        Digest::of(&bytes)
    );
}

/// **The count reaches the watcher at least once per accounting period**, and
/// not only at a checkpoint.
///
/// A checkpoint is four mebibytes. At a 50 KB/s cap across four transfers that
/// is five and a half minutes of a number that does not move on a transfer
/// working perfectly, and the clause wants the bound in time as well as in
/// bytes. **The cap here never gives at all**, which is the same case at its
/// limit: the loop draws nothing, waits, and must still report.
#[tokio::test]
async fn the_count_reaches_the_watcher_at_least_once_per_accounting_period() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("cadence");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(2 * 1024 * 1024);
    let on_the_machine = scratch.at("machine/frame.exr");
    std::fs::create_dir_all(on_the_machine.parent().expect("a folder")).expect("made");
    std::fs::write(&on_the_machine, &bytes).expect("written");
    let here = scratch.at("downloads/frame.exr");
    std::fs::create_dir_all(here.parent().expect("a folder")).expect("made");

    let mut record = a_download(&on_the_machine, &here, bytes.len() as u64, None);

    let mut local = APlainDisk;
    // Nothing to draw, so no byte ever moves and no checkpoint is ever reached.
    let mut a_cap = ACapThatBites { left: 0 };
    let mut keeper = WritesItDown::default();
    let mut watcher = CountsWhatItIsTold::default();

    let waited = THE_ACCOUNTING_PERIOD * 8;
    let cut_short = tokio::time::timeout(
        waited,
        carry_out(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut local,
                budget: &mut a_cap,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_secs(30),
                intent: Intent::CreateIntended,
            },
            &mut record,
        ),
    )
    .await;
    assert!(cut_short.is_err(), "the capped run does not finish");

    // The head of the attempt reports once. **Everything after it is the time
    // bound**, and before it was built there was nothing after it at all.
    assert!(
        watcher.reports >= 4,
        "the count went out {} times in {waited:?}, which is not a bound in time",
        watcher.reports
    );
}

/// **The inactivity deadline is on the download's own read.**
///
/// It used to be applied in exactly two places and both wrapped the settling of
/// writes — and a download writes nothing to the machine, so it had nothing to
/// settle. A machine that stopped answering a read left the loop waiting for
/// ever, no record was written, and the row on screen kept whatever it last
/// said. **The deadline here is set so short that the first read cannot beat
/// it**, which stands in for the machine that stops answering.
#[tokio::test]
async fn a_download_read_that_outlasts_the_deadline_stalls_rather_than_waiting_for_ever() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("stall");
    let key = scratch.a_key("id_ed25519");
    let machine = a_machine(&key, sftp_server, &scratch).await;
    let connection = a_connection(&machine, &key).await;
    let mut subsystem = a_subsystem(&connection).await;

    let bytes = a_file_of(1024 * 1024);
    let on_the_machine = scratch.at("machine/frame.exr");
    std::fs::create_dir_all(on_the_machine.parent().expect("a folder")).expect("made");
    std::fs::write(&on_the_machine, &bytes).expect("written");
    let here = scratch.at("downloads/frame.exr");
    std::fs::create_dir_all(here.parent().expect("a folder")).expect("made");

    let mut record = a_download(&on_the_machine, &here, bytes.len() as u64, None);

    let mut local = APlainDisk;
    let mut budget = smoozy_server_protocol::transfers::moving::NoCapIsInForce;
    let mut keeper = WritesItDown::default();
    let mut watcher = NobodyIsWatching;

    // **The whole run is itself bounded**, so a loop that waits for ever fails
    // this test by timing out rather than by hanging the suite.
    let happened = tokio::time::timeout(
        Duration::from_secs(20),
        carry_out(
            Transfer {
                subsystem: &mut subsystem,
                local: &mut local,
                budget: &mut budget,
                keeper: &mut keeper,
                watcher: &mut watcher,
                inactivity_deadline: Duration::from_nanos(1),
                intent: Intent::CreateIntended,
            },
            &mut record,
        ),
    )
    .await
    .expect("the loop does not wait for ever on a read");

    // **Stalled is a statement about the channel and never about the
    // connection**, and it is its own ending rather than *interrupted*.
    assert_eq!(happened.ending, Ending::Stalled);
    assert_eq!(record.ending, Some(Ending::Stalled));
    assert!(!here.exists(), "nothing reached the final name");
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

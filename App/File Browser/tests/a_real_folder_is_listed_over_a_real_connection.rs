//! **Step 5's own milestone, run rather than argued: a real folder is listed
//! over a real connection, a folder this login may not read is told apart from
//! one that is not there, and an upload request written before the app closes
//! is picked up in a fresh sitting.**
//!
//! **What the far end is.** A real SSH connection, opened by Smoozy's own
//! transport on the loopback address, carrying a **subsystem session served by
//! OpenSSH's own `sftp-server`** — the same program step 4's own milestone was
//! watched against, and the program a rental's own file-transfer server is.
//! Nothing is rented; nothing on the Creator's machine outside a temporary
//! directory is touched.
//!
//! **What that is worth, and what it is not.** Everything below runs on a real
//! folder on a real disk: real names, real sizes, real modification times, real
//! `.blend` files Blender itself wrote, a real permission bit, a real absence,
//! and a real free-space reading from the server's own extension. **It is not a
//! rented Linux machine**, and it says nothing about a provider's own image.
//! **And it is not the holder**: nobody yet opens, holds or reopens the reading
//! channel for a connected period — that is `Servers`' period and `Server
//! protocol and transfers`' channel admission — so the lease values here come
//! from this test, exactly as `src/wiring.rs` says they must.
//!
//! **Where `sftp-server` is not on this system the tests say so and stop.**
//! They do not quietly pass: a printed line names the system and the miss.
//!
//! **And the permission half says so and stops on two systems where the
//! condition it rests on cannot be made.** That half wants a real folder this
//! login may not read. On Windows the bit that makes one is not a Unix mode and
//! `set_permissions` will not make it. On a machine where the tests run as the
//! superuser — which some hosted runners do — a mode of zero refuses nobody, so
//! **the folder is asked whether it really refuses this login and the test stops
//! where it does not.** Smoozy's own telling apart is unchanged in both cases —
//! the two statuses are read off the protocol either way — but a condition the
//! machine will not make is not manufactured here.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` F1, F2, F3, F16.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/File Browser.md`. Every other
//! document's marker is written with that document — at the site, or in the
//! file's own header list of markers that belong elsewhere. **Where a marker is
//! a clause of this crate's own plan *and* of the document meant, the header
//! list is not enough and the document is written out at the site**, because a
//! bare one there does not fail to resolve: it resolves to a real and wrong
//! clause, which nobody reading it can tell from a right one.
//!
//! **The words above are the words `src/lib.rs` carries.** They are repeated
//! because a file under `tests/` is its own crate and never sees that one.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::{Duration, SystemTime};

use russh::server::{Auth, Msg, Server as _, Session};
use russh::{Channel, ChannelId};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::TcpListener;

use smoozy_file_browser::already_exists::{
    self, Arriving, Comparison, ReadsAReceivingFolder, ReceivingFolder, TheQuestionStopped,
};
use smoozy_file_browser::blend_version::{read_the_tag, PrefixGrowth, Tag};
use smoozy_file_browser::free_space::BrowsingFigure;
use smoozy_file_browser::listing::{
    CompleteListing, DirectoryAnswer, EntryName, Kind, Lease, ListedEntry, ReadsADirectory,
};
use smoozy_file_browser::permission::{self, WalkOutcome, WhatSmoozyMet};
use smoozy_file_browser::request::{self, EnumerationEntry, KeepsTheRequest, UploadRequest};
use smoozy_file_browser::wiring::{TheReadingChannel, TheZstandardDecoder};
use smoozy_server_protocol::free_space;
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::subsystem::{extension, kind};
use smoozy_server_protocol::transfers::{Ending, TransferRecord};
use smoozy_server_protocol::transport::ssh::{open, Connection};
use smoozy_server_protocol::transport::{Credential, DecideHostTrust, Endpoint, Figures, HostTrust};
use smoozy_state::identities::{Identity, IdentityKind};

// ------------------------------------------- where OpenSSH's own server is

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
                     The listing, the permission walk and the already-exists walk \
                     are unproved against a real connection here.",
                    std::env::consts::OS
                );
                return;
            }
        }
    };
}

/// Closes a real folder to this login, and **says whether the condition
/// actually took**.
///
/// **A login that may read anything cannot be refused**, so a run as the
/// superuser — which some hosted runners are — reads the folder straight
/// through with the mode at zero. That is not a fault in Smoozy and the test
/// says so and stops rather than asserting something the machine cannot make.
#[cfg(unix)]
fn close_it(folder: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(folder, std::fs::Permissions::from_mode(0o000))
        .expect("the mode is set");
    std::fs::read_dir(folder).is_err()
}

#[cfg(not(unix))]
fn close_it(_folder: &Path) -> bool {
    false
}

/// Says a condition the machine will not make out loud, and stops.
macro_rules! the_closed_folder_or_say_so {
    ($it_took:expr) => {
        if !$it_took {
            println!(
                "SKIPPED from here on ({}): this login reads a folder whose mode is \
                 zero, so no folder on this machine can refuse it. Smoozy's own \
                 telling apart is unchanged — both statuses are read off the \
                 protocol — but it is unproved here.",
                std::env::consts::OS
            );
            return;
        }
    };
}

// ---------------------------------------------------------------- the machine

struct Machine {
    sftp_server: PathBuf,
    /// The channels this client has opened, held rather than dropped — the same
    /// reason `Server protocol and transfers`' own integration test holds them:
    /// a server that takes its bytes through the handler's own `data` call
    /// never sends a window adjustment, and a transfer stops dead at the
    /// channel's opening window.
    channels: HashMap<ChannelId, Channel<Msg>>,
}

impl Clone for Machine {
    fn clone(&self) -> Machine {
        Machine {
            sftp_server: self.sftp_server.clone(),
            channels: HashMap::new(),
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

    /// **The machine's own file-transfer server, on a subsystem session** —
    /// what an `sshd` does with the subsystem line.
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
        channels: HashMap::new(),
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

/// A temporary folder that removes itself. **Nothing outside it is touched.**
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!(
            "smoozy-file-browser-{name}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("after 1970")
                .as_nanos()
        ));
        std::fs::create_dir_all(&path).expect("a temporary directory can be made");
        Scratch(path)
    }

    fn a_key(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        let made = std::process::Command::new("ssh-keygen")
            .args(["-t", "ed25519", "-N", "", "-C", "smoozy browser test", "-f"])
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
        // A folder made unreadable on purpose has to be made readable again or
        // nothing can remove what is under it.
        let _ = make_it_readable_again(&self.0);
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(unix)]
fn make_it_readable_again(root: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let _ = std::fs::set_permissions(entry.path(), std::fs::Permissions::from_mode(0o755));
            let _ = make_it_readable_again(&entry.path());
        }
    }
    Ok(())
}

#[cfg(not(unix))]
fn make_it_readable_again(_root: &Path) -> std::io::Result<()> {
    Ok(())
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

/// **The reading channel, with every packet Smoozy sends on it recorded.**
///
/// The recording is a wrapper around the channel's own stream and changes
/// nothing about what crosses it. It is what lets a test say *no stat per
/// entry was issued* (L1) against a server this project did not write — the
/// far end is OpenSSH's own and cannot be asked what it was sent.
async fn a_subsystem(connection: &Connection) -> (Subsystem<Watched>, Arc<Mutex<Vec<u8>>>) {
    let channel = connection
        .open_a_subsystem_channel()
        .await
        .expect("the machine serves the file-transfer subsystem");
    let asked = Arc::new(Mutex::new(Vec::new()));
    let watched = Watched {
        inner: Box::pin(channel),
        pending: Vec::new(),
        asked: asked.clone(),
    };
    (
        Subsystem::negotiate(watched)
            .await
            .expect("the subsystem negotiates"),
        asked,
    )
}

/// The stream, with the kind byte of every whole packet Smoozy writes recorded
/// in order.
struct Watched {
    inner: Pin<Box<russh::ChannelStream<russh::client::Msg>>>,
    /// Bytes written and not yet resolved into whole packets. **A write is not
    /// promised to be one packet**, so the frames are taken out of a running
    /// buffer rather than off each call.
    pending: Vec<u8>,
    asked: Arc<Mutex<Vec<u8>>>,
}

impl Watched {
    fn take_the_whole_packets(&mut self) {
        loop {
            if self.pending.len() < 4 {
                return;
            }
            let length = u32::from_be_bytes([
                self.pending[0],
                self.pending[1],
                self.pending[2],
                self.pending[3],
            ]) as usize;
            if self.pending.len() < 4 + length || length == 0 {
                return;
            }
            self.asked
                .lock()
                .expect("the record of what was asked")
                .push(self.pending[4]);
            self.pending.drain(..4 + length);
        }
    }
}

impl AsyncRead for Watched {
    fn poll_read(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.inner.as_mut().poll_read(context, buffer)
    }
}

impl AsyncWrite for Watched {
    fn poll_write(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let written = std::task::ready!(self.inner.as_mut().poll_write(context, bytes));
        if let Ok(count) = &written {
            let taken = bytes[..*count].to_vec();
            self.pending.extend_from_slice(&taken);
            self.take_the_whole_packets();
        }
        Poll::Ready(written)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.inner.as_mut().poll_flush(context)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.inner.as_mut().poll_shutdown(context)
    }
}

/// The two `.blend` files beside this test, **saved by Blender itself**.
///
/// **Neither name says a version**, which is the point: L8 reads the version
/// out of the file's own opening text and the two answer differently.
///
/// - `scene.blend` — saved by Blender 5.0.1, **Zstandard-compressed**, which is
///   L8's decode path.
/// - `props.blend` — saved by Blender 4.5.12 LTS, **plain**, which is L8's
///   first attempt.
fn a_blend_saved_by_blender(name: &str) -> PathBuf {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/blend files");
    let path = here.join(name);
    assert!(path.exists(), "the fixture is beside this test: {path:?}");
    path
}

/// A real folder on a real disk, with real files in it.
///
/// Returns the folder and, per name, what was put there — the size and the
/// modification time read back off the disk, so the assertions are against the
/// disk rather than against what the test meant to write.
fn a_folder_with_real_files_in_it(scratch: &Scratch) -> (PathBuf, HashMap<String, (u64, i64)>) {
    let folder = scratch.at("jobs");
    std::fs::create_dir_all(&folder).expect("the folder is made");
    std::fs::copy(
        a_blend_saved_by_blender("scene.blend"),
        folder.join("scene.blend"),
    )
    .expect("the compressed save is copied in");
    std::fs::copy(
        a_blend_saved_by_blender("props.blend"),
        folder.join("props.blend"),
    )
    .expect("the plain save is copied in");
    std::fs::write(folder.join("notes.txt"), b"three lines\nof nothing\nmuch\n")
        .expect("a plain file");
    std::fs::create_dir_all(folder.join("renders")).expect("a folder among the files");

    let mut what_is_there = HashMap::new();
    for entry in std::fs::read_dir(&folder).expect("the folder reads") {
        let entry = entry.expect("an entry");
        let found = entry.metadata().expect("its metadata");
        let seconds = found
            .modified()
            .expect("a modification time")
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("after 1970")
            .as_secs() as i64;
        what_is_there.insert(
            entry.file_name().to_string_lossy().into_owned(),
            (found.len(), seconds),
        );
    }
    (folder, what_is_there)
}

/// **What this computer itself says about the disk a folder is on** — the
/// second observer the free-space limb below holds the machine's answer
/// against.
///
/// It is the same POSIX call the far end makes: `statvfs`'s `f_bavail` counted
/// in `f_frsize`, which is `Server protocol and transfers` F3's **available**
/// figure rather than the free one, so the two are answering the same question.
/// The machine program reads its own disk exactly this way.
///
/// **`None` means this computer cannot answer**, not that the answer is zero —
/// on Windows, where there is no `statvfs`, and where the call itself refuses.
/// The caller says so and stops rather than asserting on a number it does not
/// have.
#[cfg(unix)]
fn what_this_computer_says(folder: &Path) -> Option<u64> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let named = CString::new(folder.as_os_str().as_bytes()).ok()?;
    let mut read: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(named.as_ptr(), &mut read) } != 0 {
        return None;
    }
    Some((read.f_bavail as u64).saturating_mul(read.f_frsize as u64))
}

#[cfg(not(unix))]
fn what_this_computer_says(_folder: &Path) -> Option<u64> {
    None
}

/// **One free-space reading off the machine, with this computer's own reading
/// of the same path taken on either side of it.**
///
/// The far end's `statvfs` happens strictly between the two taken here, so
/// **whatever the rest of the computer did during the call is inside the
/// bracket** — which is what makes this judgeable on a disk everything else on
/// the machine is also using. The gap between one reading and the next is a
/// round trip on the loopback address.
struct ASandwichedReading {
    this_computer_held: u64,
    the_machine_answered: u64,
    this_computer_held_after: u64,
}

impl ASandwichedReading {
    /// What a neighbour may move **up and back again** inside one loopback
    /// round trip — a monotone move of any size is inside the bracket and needs
    /// nothing. It is deliberately far below the 64 MiB the limb writes, so a
    /// figure that is a constant, is stale, is another disk's or is added up
    /// from listed sizes cannot sit inside it.
    const A_NEIGHBOURS_EXCURSION: u64 = 16 * 1024 * 1024;

    fn the_machine_answered_what_this_computer_was_holding(&self, when: &str) {
        let floor = self.this_computer_held.min(self.this_computer_held_after);
        let ceiling = self.this_computer_held.max(self.this_computer_held_after);
        let outside = if self.the_machine_answered < floor {
            floor - self.the_machine_answered
        } else {
            self.the_machine_answered.saturating_sub(ceiling)
        };
        assert!(
            outside <= Self::A_NEIGHBOURS_EXCURSION,
            "{when}, the machine answered {} for this path while this computer read {} just \
             before the call and {} just after it — {outside} bytes outside the bracket",
            self.the_machine_answered,
            self.this_computer_held,
            self.this_computer_held_after
        );
    }
}

async fn a_reading_sandwiched<T>(
    subsystem: &mut Subsystem<T>,
    folder: &Path,
    folder_text: &str,
) -> Option<ASandwichedReading>
where
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    let this_computer_held = what_this_computer_says(folder)?;
    let the_machine_answered = free_space::read_it(subsystem, folder_text)
        .await
        .expect("the server's own extension answered again")
        .available_bytes;
    let this_computer_held_after = what_this_computer_says(folder)?;
    Some(ASandwichedReading {
        this_computer_held,
        the_machine_answered,
        this_computer_held_after,
    })
}

fn the_entry<'a>(entries: &'a [ListedEntry], name: &str) -> &'a ListedEntry {
    entries
        .iter()
        .find(|entry| entry.name.as_bytes() == name.as_bytes())
        .unwrap_or_else(|| panic!("the machine listed {name}"))
}

// ------------------------------------------------------------- limb one

/// **A real folder listed over a real connection, in one reading, carrying each
/// entry's clock, its size and its `.blend` version, with the free-space figure
/// beside them.**
///
/// Every value is asserted against the disk the far end is reading — the sizes
/// and the modification times off `std::fs`, the version off two files Blender
/// itself saved.
///
/// **L1's *one reading* is asserted off the packets Smoozy actually sent**, not
/// off a comment: one directory opening, and **no stat of any kind**, against a
/// server this project did not write.
///
/// **L11's figure is the file-transfer subsystem's own per-path reading**
/// (`Server protocol and transfers` F2 route 1), taken against the folder being
/// browsed and naming that folder — never a mount, a device or the machine
/// (F1, F16), and the available figure rather than the free one (F3).
#[tokio::test]
async fn a_real_folder_is_listed_in_one_reading_with_its_clock_size_version_and_free_space() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("listing");
    let key = scratch.a_key("desk");
    let host_key = scratch.a_key("host");
    let machine = a_machine(&host_key, sftp_server).await;
    let (folder, what_is_there) = a_folder_with_real_files_in_it(&scratch);
    let folder_text = folder.to_string_lossy().into_owned();

    let connection = a_connection(&machine, &key).await;
    let (mut subsystem, asked) = a_subsystem(&connection).await;

    // **The reading.** One channel, and the listing, the version reads and the
    // free-space reading all ride it (L1, L7).
    let mut channel = TheReadingChannel::on(&mut subsystem, Lease(1));
    let answer = channel.read_a_directory(&folder_text, Lease(1)).await;
    let DirectoryAnswer::Entries {
        entries,
        the_subsystem_reported_the_end,
    } = answer
    else {
        panic!("the folder listed");
    };
    // **L3: complete because the subsystem reported its end**, and never
    // because output arrived.
    assert!(the_subsystem_reported_the_end);

    // **The listing is the folder's contents and nothing else.**
    //
    // **This is what driving the real end against a real server found.** Before
    // 2026-09-07 the listing came back holding `.` and `..` ahead of the four
    // real names, each carrying the size and the modification time of the
    // folder itself and of its parent — so a real folder holding four things
    // listed as holding six. No test above the wiring could have caught it,
    // because every reading above it takes entries as values. The account is in
    // this crate's decisions record.
    assert_eq!(
        entries.len(),
        what_is_there.len(),
        "the listing holds what the folder holds: {:?}",
        entries
            .iter()
            .map(|entry| entry.name.for_display())
            .collect::<Vec<_>>()
    );
    assert!(
        !entries
            .iter()
            .any(|entry| entry.name.as_bytes() == b"." || entry.name.as_bytes() == b".."),
        "neither the folder itself nor its parent is a thing inside it"
    );

    // **L1's four fields, against the disk.**
    for (name, (size, modified)) in &what_is_there {
        let listed = the_entry(&entries, name);
        assert_eq!(
            listed.size.bytes(),
            *size,
            "{name}'s size came back as the disk holds it"
        );
        assert_eq!(
            listed
                .last_changed
                .expect("the machine sent a modification time")
                .epoch_seconds(),
            *modified,
            "{name}'s clock came back as the disk holds it"
        );
    }
    assert_eq!(the_entry(&entries, "renders").kind, Kind::Folder);
    assert_eq!(the_entry(&entries, "scene.blend").kind, Kind::File);
    assert_eq!(the_entry(&entries, "notes.txt").kind, Kind::File);

    // **L8's version, read off the file and never off its name.** Two files
    // whose names say nothing, saved by two different Blenders, one compressed
    // and one plain.
    let scene = format!("{folder_text}/scene.blend");
    let props = format!("{folder_text}/props.blend");
    let notes = format!("{folder_text}/notes.txt");
    let scene_tag = read_the_tag(
        &scene,
        &mut channel,
        &mut TheZstandardDecoder,
        PrefixGrowth::DECLARED,
    )
    .await;
    let props_tag = read_the_tag(
        &props,
        &mut channel,
        &mut TheZstandardDecoder,
        PrefixGrowth::DECLARED,
    )
    .await;
    let notes_tag = read_the_tag(
        &notes,
        &mut channel,
        &mut TheZstandardDecoder,
        PrefixGrowth::DECLARED,
    )
    .await;
    match scene_tag {
        Tag::SavedWith(version) => assert_eq!(version.as_a_tag(), "5.0"),
        other => panic!("the compressed save's tag came back {other:?}"),
    }
    match props_tag {
        Tag::SavedWith(version) => assert_eq!(version.as_a_tag(), "4.5"),
        other => panic!("the plain save's tag came back {other:?}"),
    }
    // **A file that is not a `.blend` leaves the tag blank and nothing is
    // guessed from it** (L8).
    assert!(
        matches!(notes_tag, Tag::Blank(_)),
        "a plain text file's tag came back {notes_tag:?}"
    );

    // **The listing, as the browser holds it** — the entries with the moment
    // Smoozy looked, on this computer's clock.
    let listing = CompleteListing::of(entries.clone(), SystemTime::now(), false);
    assert!(listing
        .holds(&EntryName::from_the_machine(b"scene.blend".to_vec()))
        .is_some());
    assert!(!listing.read_elevated);

    drop(channel);

    // **The free-space figure, beside them** (L11 to L13).
    let reading = free_space::read_it(&mut subsystem, &folder_text)
        .await
        .expect("the server's own extension answered");
    assert_eq!(
        reading.route,
        free_space::WhichRoute::TheStatvfsExtension,
        "F2 route 1, the extension OpenSSH announces"
    );
    assert!(
        subsystem.announcement().announces(extension::STATVFS),
        "the route was taken because the server announced it"
    );
    assert_eq!(
        reading.the_path_it_was_taken_against, folder_text,
        "F1: the reading names the exact path it was taken against"
    );
    assert!(
        reading.available_bytes > 0,
        "the disk this test is running on has room on it"
    );
    // **The figure is this disk's own, live, and for this path.** Sixty-four
    // mebibytes are written into the browsed folder so the figure has something
    // to do, and a reading is taken before the write and after it — **each one
    // sandwiched between two readings this computer takes for itself**
    // ([`a_reading_sandwiched`]). What that rules out is a figure describing
    // something other than the place the user is browsing, a constant, a stub,
    // a stale answer, and a figure added up from listed sizes — **nothing
    // subtracts a listed size to get here** (`Server protocol and transfers`
    // F4, whose own evidence is a 500 GB sparse file that moved the reading by
    // nothing), and no listing is taken between the readings.
    //
    // **THIS IS NOT WHAT THE TEST USED TO ASSERT, AND THE CHANGE OF 2026-09-20
    // IS THE POINT.** Until then it asserted the *size of the move*: sixty-four
    // mebibytes written, and the figure had to fall by between half and four
    // times that, with the band written wide on the grounds that *a working
    // computer writes and frees bytes of its own while a test runs*. **That is
    // a promise no filesystem makes and no band can be widened to cover.** The
    // free-space figure belongs to the whole disk, so anything else on the
    // computer that frees bytes while the write is happening subtracts from the
    // move, and enough of it subtracts all of it — and the failure is at the
    // *low* edge, so the only widening that would hide it is widening the band
    // down to nothing, which deletes the assertion. **The desk saw exactly
    // that**: *the figure moved by 8192 bytes when 67108864 were written*, once
    // in four whole-workspace runs and never when the test ran alone, 8192
    // being the file's own metadata with none of its bytes counted.
    //
    // **Driven deliberately on 2026-09-20**, with other processes writing and
    // deleting files on the same volume, the old assertion failed 2 of 10 runs
    // against 0 of 10 for the same test alone, and the measured move ranged
    // from -253 MB to +268 MB. **Neither `fsync` nor `F_FULLFSYNC` mends it**:
    // with the neighbours moving an amount that does not cancel this test's own,
    // a fully flushed write still read as never having happened in 1 of 30
    // measurements. **So the size of the move is not asserted at all any more.**
    // What is asserted is the thing that *is* promised and is what the old
    // assertion was a proxy for: that the number Smoozy reads over the wire is
    // the number this computer itself holds for that path at that moment.
    // **The bracket is what makes that judgeable on a shared disk** — the far
    // end's own `statvfs` happens strictly between this computer's two, so
    // anything the neighbours move during the call is inside it, however much
    // it is. Measured on a quiet desk the machine's answer and this computer's
    // were **equal to the byte, twelve times out of twelve**; under five other
    // processes writing and deleting 37 MiB files on the same volume, twenty
    // runs, the bracket held every time and the widest a neighbour stretched it
    // was one whole file of its own.
    let a_known_weight: u64 = 64 * 1024 * 1024;
    let heavy = folder.join("big.bin");
    let before = a_reading_sandwiched(&mut subsystem, &folder, &folder_text).await;
    std::fs::write(&heavy, vec![7u8; a_known_weight as usize]).expect("the disk takes it");
    let after = a_reading_sandwiched(&mut subsystem, &folder, &folder_text).await;
    std::fs::remove_file(&heavy).expect("and it goes again");
    match (before, after) {
        (Some(before), Some(after)) => {
            before.the_machine_answered_what_this_computer_was_holding("before the write");
            after.the_machine_answered_what_this_computer_was_holding("after the write");
        }
        _ => println!(
            "SKIPPED, one limb of this test ({}): this computer has no `statvfs` of its own to \
             read the same path with, so there is nothing to hold the machine's figure against. \
             The reading, its route, the path it names and that it is above zero are asserted \
             above and are unaffected.",
            std::env::consts::OS
        ),
    }

    let figure = BrowsingFigure {
        folder: folder_text.clone(),
        reading: Ok(reading),
        taken_at_on_the_machines_clock: SystemTime::now(),
        the_listing_it_rides_is_stale: false,
    };
    assert_eq!(figure.folder, folder_text);
    assert!(!figure.the_listing_it_rides_is_stale);

    // **L1, off the wire.** One directory opening for the one folder, and **no
    // stat per entry** — asserted against what was actually sent.
    let asked = asked.lock().expect("the record of what was asked").clone();
    assert!(
        !asked.contains(&kind::STAT) && !asked.contains(&kind::LSTAT),
        "L1: no reading here ever issues a stat per entry, and this asked {asked:?}"
    );
    assert_eq!(
        asked.iter().filter(|k| **k == kind::OPENDIR).count(),
        1,
        "one folder, one directory opening"
    );
    // The three prefix reads, each an open, some reads and a close (L7). No
    // program on the machine is asked and nothing opens the file for Blender.
    assert_eq!(asked.iter().filter(|k| **k == kind::OPEN).count(), 3);
    assert!(asked.contains(&kind::READ));
}

// ------------------------------------------------------------- limb two

/// **A folder this login may not read, told apart from one that is not there —
/// against real conditions on a real disk.**
///
/// Two folders: one made and then closed to this login, one never made at all.
/// **The two answers are asserted to be different**, and each is asserted to be
/// the right one of the two (P1).
///
/// The same pair is then driven through the already-exists walk's own reading,
/// where X2 gives them different endings: a folder that is not there ends its
/// branch, and a folder that cannot be read **stops the question** rather than
/// being skipped — the count wrong in the quiet direction that X2 exists
/// against.
#[tokio::test]
async fn a_folder_this_login_may_not_read_is_told_apart_from_one_that_is_not_there() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    if cfg!(windows) {
        println!(
            "SKIPPED ({}): the condition this rests on is a Unix mode bit, and \
             a Windows access list is not made with set_permissions. Smoozy's \
             own telling apart is unchanged — both statuses are read off the \
             protocol — but it is unproved on this system.",
            std::env::consts::OS
        );
        return;
    }
    let scratch = Scratch::new("permission");
    let key = scratch.a_key("desk");
    let host_key = scratch.a_key("host");
    let machine = a_machine(&host_key, sftp_server).await;

    // **A real folder this login may not read.** It exists, it holds a file,
    // and the mode says nobody may look inside.
    let closed = scratch.at("closed");
    std::fs::create_dir_all(&closed).expect("the folder is made");
    std::fs::write(closed.join("inside.txt"), b"something").expect("a file inside it");
    let it_took = close_it(&closed);
    the_closed_folder_or_say_so!(it_took);
    // **And a folder that was never made.**
    let absent = scratch.at("never-made");
    assert!(!absent.exists());

    let connection = a_connection(&machine, &key).await;
    let (mut subsystem, _asked) = a_subsystem(&connection).await;
    let mut channel = TheReadingChannel::on(&mut subsystem, Lease(1));

    let DirectoryAnswer::Refused(refused) = channel
        .read_a_directory(&closed.to_string_lossy(), Lease(1))
        .await
    else {
        panic!("a folder this login may not read does not list");
    };
    let DirectoryAnswer::Refused(missing) = channel
        .read_a_directory(&absent.to_string_lossy(), Lease(1))
        .await
    else {
        panic!("a folder that is not there does not list");
    };

    // **The two are different, and each is the right one of the two.**
    assert_ne!(
        refused, missing,
        "P1: the two cases are separable on this route"
    );
    assert_eq!(WhatSmoozyMet::from(&refused), WhatSmoozyMet::Refused);
    assert_eq!(WhatSmoozyMet::from(&missing), WhatSmoozyMet::Gone);
    // **P2: the elevation offer attaches to the refusal alone.**
    assert!(WhatSmoozyMet::from(&refused).the_elevation_question_arises());
    assert!(!WhatSmoozyMet::from(&missing).the_elevation_question_arises());

    // **X1's own reading gives the two the same difference** — the absent one
    // is the protocol's no-such-file and nothing else.
    assert_eq!(
        ReadsAReceivingFolder::read(&mut channel, absent.to_string_lossy().as_bytes()).await,
        ReceivingFolder::NotThere
    );
    assert_eq!(
        ReadsAReceivingFolder::read(&mut channel, closed.to_string_lossy().as_bytes()).await,
        ReceivingFolder::CouldNotBeRead(WhatSmoozyMet::Refused)
    );

    // **X2's own ending for the refused one: it stops the question rather than
    // being skipped**, which is the count wrong in the quiet direction that
    // clause exists against.
    let stopped = already_exists::walk(
        closed.to_string_lossy().as_bytes(),
        &[an_arriving_file(&[b"anything.blend"])],
        &mut channel,
        SystemTime::now(),
    )
    .await
    .expect_err("a receiving folder this login may not read stops the question");
    assert_eq!(
        stopped,
        TheQuestionStopped::AFolderCouldNotBeRead {
            folder: closed.to_string_lossy().as_bytes().to_vec(),
            met: WhatSmoozyMet::Refused
        }
    );
    // **And the one that is not there ends its branch instead**, which is a
    // different ending and not a stop.
    let answered = already_exists::walk(
        absent.to_string_lossy().as_bytes(),
        &[an_arriving_file(&[b"anything.blend"])],
        &mut channel,
        SystemTime::now(),
    )
    .await
    .expect("an absent receiving folder is an answer");
    assert_eq!(answered.colliding, 0);
}

// ----------------------------------------------------------- limb three

/// **An upload request written before the app closes, picked up in a fresh
/// sitting.**
///
/// The store is `Desktop app and delivery` §5's — the local record on disk,
/// opened and migrated the way the application opens it — and the request is
/// composed and answered through this crate's own calls. **Nothing here writes
/// a row by hand.**
///
/// The sitting ends by dropping the record, which is all an application close
/// leaves behind (U7: the activity dies with the process and only the record is
/// left). A second run opens the same file, reads the request back and finds
/// its unfinished unit as a leftover (U8).
#[test]
fn an_upload_request_written_before_the_app_closes_is_picked_up_in_a_fresh_sitting() {
    let _deadline = TheTestsDeadline::started();
    let scratch = Scratch::new("request");
    let record_file = scratch.at("record.sqlite");
    let issue = |kind| Identity::issue(kind).expect("random bytes");

    let server_row = issue(IdentityKind::LocalServer);
    let installation = issue(IdentityKind::Installation);
    let first_run = issue(IdentityKind::Run);

    // Two real files on this disk, so the enumeration's sizes are the disk's.
    let sending = scratch.at("sending");
    std::fs::create_dir_all(&sending).expect("the folder is made");
    std::fs::copy(
        a_blend_saved_by_blender("scene.blend"),
        sending.join("scene.blend"),
    )
    .expect("copied");
    std::fs::write(sending.join("notes.txt"), b"three lines\nof nothing\nmuch\n")
        .expect("a plain file");

    let entries: Vec<EnumerationEntry> = ["scene.blend", "notes.txt"]
        .iter()
        .map(|name| {
            let source = sending.join(name);
            let size = std::fs::metadata(&source).expect("it is there").len();
            EnumerationEntry::enumerated(
                source.to_string_lossy().as_bytes().to_vec(),
                format!("/machine/jobs/{name}").into_bytes(),
                size,
            )
        })
        .collect();

    let (written, transfer_identity) = {
        // **The application's own opening**, and its own migration.
        let record = smoozy_desktop::record::LocalRecord::open_at(&record_file)
            .expect("the local record opens");
        let mut keeper =
            smoozy_desktop::record::upload_requests::UploadRequests::over(record.connection());

        // **Open the upload request before composing its files.**
        let was_open = request::open_an_upload_request().expect("the act is open");
        let mut request = UploadRequest::confirmed(
            was_open,
            issue(IdentityKind::Operation),
            server_row.clone(),
            Some(installation.clone()),
            b"/machine/jobs".to_vec(),
            entries,
            first_run.clone(),
        );
        // **U1's write: the enumeration is durable before the first byte
        // moves** (`Server protocol and transfers` S8's ordering).
        keeper.write_it_down(&request).expect("it is written down");

        // **U4's answer, applied once**, through this crate's own call — which
        // writes the dispositions down through the same keeper.
        let comparisons = request
            .entries
            .iter()
            .map(|entry| already_exists::AComparison {
                landing_path: entry.landing_path.clone(),
                what: Comparison::LandsFresh,
            })
            .collect::<Vec<_>>();
        request::apply_the_collision_answer(
            &mut request,
            &comparisons,
            already_exists::CollisionAnswer::Replace,
            &mut std::iter::empty(),
            &mut keeper,
            SystemTime::now(),
        )
        .expect("the answer applies");

        // **U3: the transfer this request arranges is `Server protocol and
        // transfers` §7's own record**, composed by this crate and put in the
        // record by that document's store.
        let transfer = issue(IdentityKind::Transfer);
        let arranged: TransferRecord = request::arrange_a_transfer(
            &request,
            &request.entries[0],
            transfer.clone(),
            "/machine/jobs/.smoozy-staging",
            first_run.clone(),
        )
        .expect("the name carries")
        .expect("a transfer is arranged for a unit that is to send");
        let mut arranged = arranged;
        // The sitting ended in the middle of it, which is what a leftover is.
        arranged.ending = Some(Ending::Interrupted);
        smoozy_desktop::record::transfer_records::put(record.connection(), &arranged)
            .expect("the transfer record is written");

        request.entries[0].transfer = Some(transfer.clone());
        keeper
            .write_it_down(&request)
            .expect("the transfer identity goes down at the moment it becomes true");
        (request, transfer)
    };
    // **The application closes here.** Nothing is left but the file.

    // ---- a fresh sitting ----
    let record = smoozy_desktop::record::LocalRecord::open_at(&record_file)
        .expect("the local record opens again");
    let second_run = issue(IdentityKind::Run);
    let keeper = smoozy_desktop::record::upload_requests::UploadRequests::over(record.connection());
    let read = keeper
        .every_request_for(&server_row)
        .expect("the store answers");

    assert_eq!(read, vec![written.clone()], "the request came back whole");
    let picked_up = &read[0];
    assert_eq!(picked_up.destination, b"/machine/jobs".to_vec());
    assert_eq!(picked_up.entries.len(), 2);
    assert_eq!(
        picked_up.collision_answer,
        Some(already_exists::CollisionAnswer::Replace),
        "U4: the one field that cannot be rebuilt from the machine survived"
    );
    // **U8: the record carries which run wrote it**, so the fresh run reads it
    // as an earlier run's.
    assert!(picked_up.ended_by_the_run_ending(&second_run));
    assert!(!picked_up.ended_by_the_run_ending(&first_run));

    // **U8's first bullet: the leftover is composed from the durable records**
    // — the request's unfinished unit against the transfer record reading
    // interrupted under the installation identity in force now.
    let transfers = smoozy_desktop::record::transfer_records::all(record.connection())
        .expect("the transfer records come back");
    let leftovers = request::the_leftovers(&read, &transfers, &second_run, Some(&installation));
    assert_eq!(leftovers.len(), 1, "one unfinished unit, one leftover");
    assert_eq!(leftovers[0].transfer, transfer_identity);
    assert_eq!(leftovers[0].landing_path, b"/machine/jobs/scene.blend".to_vec());
    assert_eq!(leftovers[0].server_row, server_row);
    // **U8's fourth bullet: nothing starts itself.** The leftover is a telling,
    // and this run arranged nothing by reading it.
    assert!(request::NOTHING_STARTS_ITSELF_ON_A_LATER_CONNECTION);

    drop(record);
}

// ------------------------------------------------------------ limb four

/// **The permission walk and the already-exists walk, over real conditions on a
/// real disk.**
///
/// **P3's walk** names the component the machine actually refused, walking down
/// from the root over a real tree with one real closed folder in it — so the
/// component named is the one whose mode was changed and not a guess.
///
/// **X1's walk** is one directory reading per receiving folder that exists,
/// against a real destination holding a real file: the arriving name that is
/// already there comes back colliding, the one that is not comes back fresh,
/// and a branch under a folder that does not exist costs one reading and ends.
#[tokio::test]
async fn the_permission_walk_and_the_already_exists_walk_run_over_real_conditions_on_disk() {
    let _deadline = TheTestsDeadline::started();
    let sftp_server = the_server_or_say_so!();
    let scratch = Scratch::new("walks");
    let key = scratch.a_key("desk");
    let host_key = scratch.a_key("host");
    let machine = a_machine(&host_key, sftp_server).await;

    // **The destination, with one of the arriving names already in it.**
    let destination = scratch.at("dest");
    std::fs::create_dir_all(&destination).expect("the folder is made");
    std::fs::write(destination.join("scene.blend"), b"an older save").expect("already there");

    let connection = a_connection(&machine, &key).await;
    let (mut subsystem, asked) = a_subsystem(&connection).await;
    let mut channel = TheReadingChannel::on(&mut subsystem, Lease(1));

    let arriving = vec![
        an_arriving_file(&[b"scene.blend"]),
        an_arriving_file(&[b"fresh.blend"]),
        // Beneath a folder the destination does not hold: X2 ends the branch.
        an_arriving_file(&[b"textures", b"bark.png"]),
    ];
    let answer = already_exists::walk(
        destination.to_string_lossy().as_bytes(),
        &arriving,
        &mut channel,
        SystemTime::now(),
    )
    .await
    .expect("the question was answered");
    assert_eq!(
        answer
            .comparisons
            .iter()
            .map(|comparison| comparison.what)
            .collect::<Vec<_>>(),
        vec![
            Comparison::CollidesWithAFile,
            Comparison::LandsFresh,
            Comparison::LandsFresh
        ]
    );
    assert_eq!(answer.colliding, 1);
    // The destination and the absent folder beneath it, and never one per
    // landing place.
    assert_eq!(answer.readings_taken, 2);

    // **The receiving folder's own listing is what the destination holds** —
    // the same reading the walk took, asserted directly, because this is the
    // other place `.` and `..` reached before 2026-09-07.
    let ReceivingFolder::Listing(listing) =
        ReadsAReceivingFolder::read(&mut channel, destination.to_string_lossy().as_bytes()).await
    else {
        panic!("the destination lists");
    };
    assert_eq!(listing.entries.len(), 1);
    assert!(listing
        .holds(&EntryName::from_the_machine(b"scene.blend".to_vec()))
        .is_some());

    drop(channel);

    // **P3's walk, down a real tree.** The closed folder sits two levels under
    // the scratch folder, and everything above it is readable.
    let closed = scratch.at("tree/shut");
    std::fs::create_dir_all(&closed).expect("the tree is made");
    std::fs::create_dir_all(closed.join("deeper")).expect("something under it");
    let it_took = close_it(&closed);

    if it_took {
        let mut channel = TheReadingChannel::on(&mut subsystem, Lease(1));
        let asking_about = format!("{}/deeper/frame.exr", closed.to_string_lossy());
        let outcome = permission::walk(&asking_about, &WhatSmoozyMet::Refused, &mut channel).await;
        assert_eq!(
            outcome,
            WalkOutcome::RefusedAt(closed.to_string_lossy().into_owned()),
            "P3: the component named is the one whose mode was changed"
        );
        assert!(outcome.the_elevation_question_arises());

        // **A component that is not there is *gone*, named at the component**,
        // and the elevation offer does not attach to it.
        let never_made = scratch.at("tree/never-made");
        let asking_about = format!("{}/frame.exr", never_made.to_string_lossy());
        let outcome = permission::walk(&asking_about, &WhatSmoozyMet::Refused, &mut channel).await;
        assert_eq!(
            outcome,
            WalkOutcome::GoneAt(never_made.to_string_lossy().into_owned())
        );
        assert!(!outcome.the_elevation_question_arises());
    } else {
        println!(
            "SKIPPED from here on ({}): no folder on this machine refuses this \
             login, so there is no component for the walk to name. The walk \
             itself is unchanged and its already-exists half above ran in full.",
            std::env::consts::OS
        );
    }

    // **No stat anywhere in either walk** — both are directory readings.
    let asked = asked.lock().expect("the record of what was asked").clone();
    assert!(
        !asked.contains(&kind::STAT) && !asked.contains(&kind::LSTAT),
        "the walks ask for directory readings and nothing else, and this asked {asked:?}"
    );
}

fn an_arriving_file(relative: &[&[u8]]) -> Arriving {
    Arriving {
        relative: relative
            .iter()
            .map(|bytes| EntryName::from_the_machine(bytes.to_vec()))
            .collect(),
        is_a_folder: false,
    }
}

// ------------------------------------------------------ this test's own deadline

/// **What every test in this file waits on**, named in the message a missed
/// deadline prints.
const WHAT_THE_TESTS_HERE_WAIT_ON: &str = "the loopback machine: OpenSSH's own sftp-server";

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

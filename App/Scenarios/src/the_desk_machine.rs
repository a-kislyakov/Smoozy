//! **The desk machine: one far end every scenario test plays against.**
//!
//! It joins three things the desk tests already had apart, and adds the two
//! acts only a machine does.
//!
//! - **OpenSSH's own server** (`sshd -i`, one per connection) with its own
//!   `sftp-server`, in a throwaway home — so every byte the desk sends goes
//!   through the SSH transport the application drives to the programs a
//!   rental runs. Copied from the desktop's
//!   `tests/a_dead_line_under_the_first_half_of_the_acts.rs`.
//! - **The line between them**: a relay this harness holds, which can go
//!   silent both ways, hold only the answers, come back, and end the
//!   connections it carries politely or with a reset. Copied from that file
//!   and from `tests/a_line_cut_silently_under_a_real_connection.rs`.
//! - **The machine's own program**, `smoozy-server`, as the workspace built
//!   it, in a throwaway package area: the active-package pointer, the four
//!   records a placement writes, the recorded root, and the real supervisor
//!   started through the pointer — the way
//!   `tests/the_set_up_card_read_from_the_machine.rs` stands it up. **The
//!   point reaches it exactly as on a rental**: it plants the carriage link
//!   over the file-transfer session, and the health exchange runs the entry
//!   point through that link over SSH.
//! - **A stand-in Blender**, a Perl script named as one, which answers the
//!   render worker's version ask and file reading, and renders by writing
//!   frames on its own clock where the attempt's inventory says they go.
//!   Nothing about a real Blender is claimed by it.
//! - **A local record and a temporary area** ([`TheDesk`]), in a throwaway
//!   folder removed whole, holding the server's row.
//!
//! **The two acts.** [`TheDeskMachine::reboots`]: the line goes silent, every
//! program on the machine is killed — the supervisor, its workers, the
//! stand-in Blender, the file-transfer servers — the supervisor is started
//! again from its package area as boot registration would start it, and the
//! line comes back, ending every connection made before the reboot the way
//! the test asks. [`TheDeskMachine::is_destroyed`]: every program is killed
//! and the line ends for good, refusing every new connection or answering
//! none.
//!
//! **How a render runs at this desk.** The machine's program takes a render's
//! process record from `/proc` on Linux (`Rented-server runtime` X1, and
//! `Rendering — machine`'s `acting/launch.rs`). This Mac has no `/proc`, so
//! **a development build of the program asks macOS's own process table for
//! the same two things instead** — added 2026-10-24 by step 12's piece SC0b,
//! in `Rented-server runtime`'s `acting/execution_group.rs`. Until then the
//! program undid every render it started here. The stand-in's frames are
//! written through the real program wherever
//! [`a_kernel_publishes_what_a_render_start_reads`] answers yes, which a
//! development build at this desk does.
//!
//! **Nothing of the Creator's is touched.** The keys, the server's
//! configuration, the machine's disk, the record and every file are inside
//! one throwaway folder removed with the machine or the desk, every program
//! started is ended with it, and the machine's Telegram worker is kept off the
//! line by its development switch. **One thing is the Creator's and is
//! unavoidable**: OpenSSH starts each session's commands in the login's own
//! home folder before `HOME` is set, so a command naming a relative path
//! would land there. Every command the application sends names `~`, which is
//! the throwaway home.
//!
//! **How a clause marker is written here**: always with its document, because
//! this crate has no plan of its own.

use std::net::SocketAddr;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::watch;

use smoozy_desktop::record::LocalRecord;
use smoozy_desktop::start::Started;
use smoozy_desktop::servers::sessions::{lend_until_it_ends, TheAdditionalConnection, ThisServersSessions};
use smoozy_desktop::servers::the_machines_work::OverThisServersPoint;
use smoozy_desktop::submitting::WhatThePageHandsOver;
use smoozy_job_files::layout::{Area, MachineRoot, TheLayout, WhereThePackageAreaSits};
use smoozy_server_protocol::framing::{Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_server_protocol::transport::ssh::{self, Connection};
use smoozy_server_protocol::transport::{Credential, DecideHostTrust, Ending, Endpoint, Figures, HostTrust};
use smoozy_servers::store::ServerRow;
use smoozy_state::identities::{Identity, IdentityKind};

// ════════════════════════════════════════════════════════════════════════════
// What the desk must have
// ════════════════════════════════════════════════════════════════════════════

/// **The desk's figures**: `Servers` K3's mechanism at a one-second interval
/// and three unanswered, so a break is declared in three to four seconds
/// rather than 45 to 60. A test that plays the plan's own figures opens its
/// connection through `smoozy_servers::wiring::TheTransport` instead.
pub const THE_DESKS_FIGURES: Figures = Figures {
    interval: Duration::from_secs(1),
    unanswered_before_the_break: 3,
};

/// **How long a desk machine may stand before it is ended**, pass or panic —
/// a test's own bound, so a wait that never ends fails the run rather than
/// holding it.
pub const HOW_LONG_A_DESK_MACHINE_MAY_STAND: Duration = Duration::from_secs(600);

/// **How long the supervisor is given to answer after it is started** — a
/// test's own bound; the program's own tests give it the same, because a
/// whole-workspace run puts twenty-odd test binaries on this Mac at once.
const HOW_LONG_THE_SUPERVISOR_IS_GIVEN: Duration = Duration::from_secs(60);

/// **What the desk must have before a machine can stand on it**, found or the
/// reason it was not.
struct WhatTheDeskHas {
    sshd: PathBuf,
    sftp_server: PathBuf,
    perl: PathBuf,
    /// The machine's program, as this workspace built it.
    the_program: PathBuf,
}

fn the_first_of(paths: &[&str]) -> Option<PathBuf> {
    paths.iter().map(PathBuf::from).find(|path| path.exists())
}

fn what_the_desk_has() -> Result<WhatTheDeskHas, String> {
    // **A release build of these tests stands no machine**: the program beside
    // it is a release build, which has no switch to keep its Telegram worker
    // off the line.
    if !cfg!(debug_assertions) {
        return Err(
            "a release build has no switch to keep the machine's Telegram worker from reaching \
             Telegram, so no machine is started"
                .to_string(),
        );
    }
    let sshd = the_first_of(&["/usr/sbin/sshd", "/usr/local/sbin/sshd"])
        .ok_or("no OpenSSH sshd on this system")?;
    let sftp_server = the_first_of(&[
        "/usr/libexec/sftp-server",
        "/usr/lib/openssh/sftp-server",
        "/usr/libexec/openssh/sftp-server",
        "/usr/lib/ssh/sftp-server",
    ])
    .ok_or("no OpenSSH sftp-server on this system")?;
    let perl = the_first_of(&["/usr/bin/perl", "/usr/local/bin/perl"])
        .ok_or("no Perl on this system for the stand-in Blender")?;
    let the_program = the_program_built_beside_this_test()
        .ok_or("the machine's program is not built beside this test (`cargo build -p smoozy-server`)")?;
    Ok(WhatTheDeskHas {
        sshd,
        sftp_server,
        perl,
        the_program,
    })
}

/// **The program this workspace built**, beside the running test's own
/// executable — present whenever the workspace's tests or the program itself
/// were built into the same folder.
fn the_program_built_beside_this_test() -> Option<PathBuf> {
    let built = std::env::current_exe()
        .ok()?
        .parent()?
        .parent()?
        .join(smoozy_runtime::the_program::THE_FIXED_ENTRY_POINT_NAME);
    built.is_file().then_some(built)
}

/// **Whether the program's own reader takes the process record a render's
/// start is read from** (`Rented-server runtime` X1), asked of this test's own
/// process. Linux answers from `/proc`; this Mac answers from its own process
/// table in a development build (step 12's piece SC0b). Where it answers
/// nothing, the machine's program undoes every render start, so no frame is
/// written through it, and a test says so rather than claiming one.
pub fn a_kernel_publishes_what_a_render_start_reads() -> bool {
    matches!(
        smoozy_runtime::execution_group::ValidatesTheRecordedPair::what_is_at(
            &smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt::the_rented_machine(),
            std::process::id(),
        ),
        Ok(Some(_))
    )
}

// ════════════════════════════════════════════════════════════════════════════
// The throwaway folder
// ════════════════════════════════════════════════════════════════════════════

static FOLDERS_MADE: AtomicUsize = AtomicUsize::new(0);

/// **A throwaway folder that takes itself away.**
///
/// **In the system's own temporary folder on a Mac**, which has no space in
/// its path for OpenSSH's configuration to trip on, and which `Job files and
/// outputs` L3 does not refuse as a machine root. **Where that folder is a
/// shared temporary area** — `/tmp` on Linux, which that clause refuses — it goes in
/// the build folder's own temporary area instead.
struct TheThrowawayFolder(PathBuf);

impl TheThrowawayFolder {
    fn made(kind: &str, named: &str) -> Result<TheThrowawayFolder, String> {
        let name = format!(
            "smzy-{kind}-{named}-{}-{}",
            std::process::id(),
            FOLDERS_MADE.fetch_add(1, Ordering::SeqCst)
        );
        for under in [Some(std::env::temp_dir()), the_build_folders_temporary_area()]
            .into_iter()
            .flatten()
        {
            let at = under.join(&name);
            let _ = std::fs::remove_dir_all(&at);
            if std::fs::create_dir_all(&at).is_err() {
                continue;
            }
            let Ok(at) = std::fs::canonicalize(&at) else {
                continue;
            };
            let text = at.display().to_string();
            let refused = ["/tmp", "/private/tmp", "/var/tmp", "/private/var/tmp"]
                .iter()
                .any(|shared| text == *shared || text.starts_with(&format!("{shared}/")));
            if !refused && MachineRoot::chosen(&text).is_ok() {
                return Ok(TheThrowawayFolder(at));
            }
            let _ = std::fs::remove_dir_all(&at);
        }
        Err("no throwaway folder could be made outside a shared temporary area".to_string())
    }

    fn folder(&self, named: &str) -> PathBuf {
        let folder = self.0.join(named);
        std::fs::create_dir_all(&folder).expect("a folder in the throwaway folder");
        folder
    }
}

/// `<the build folder>/tmp`, read off the running test's own path.
fn the_build_folders_temporary_area() -> Option<PathBuf> {
    Some(std::env::current_exe().ok()?.parent()?.parent()?.parent()?.join("tmp"))
}

impl Drop for TheThrowawayFolder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn a_program(at: &Path, text: &str) {
    std::fs::write(at, text).expect("a program is written");
    std::fs::set_permissions(at, std::fs::Permissions::from_mode(0o755)).expect("it may be run");
}

/// **Every program whose command line names `under`, ended** — asked again
/// until none is left, because a worker may start its Blender between one
/// listing and the kill. The supervisor's own death does not reach its
/// workers, which is why this exists (the program's own
/// `tests/the_program_runs.rs` says so first).
fn every_program_started_under_is_ended(under: &Path) {
    let pattern: String = under
        .display()
        .to_string()
        .chars()
        .flat_map(|c| {
            let escaped = r"\.^$*+?()[]{}|".contains(c);
            escaped.then_some('\\').into_iter().chain(std::iter::once(c))
        })
        .collect();
    for _ in 0..50 {
        let _ = Command::new("pkill")
            .args(["-KILL", "-f", &pattern])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let still = Command::new("pgrep")
            .args(["-f", &pattern])
            .output()
            .map(|listed| !listed.stdout.is_empty())
            .unwrap_or(false);
        if !still {
            return;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The deadline
// ════════════════════════════════════════════════════════════════════════════

/// **The desk machine's own deadline**: dropped with the machine, pass or
/// panic. Still standing at [`HOW_LONG_A_DESK_MACHINE_MAY_STAND`], a wait in
/// the test never ended — so every program under the machine's folder and
/// every child of this binary is ended, and so is the binary.
struct TheDeadline {
    _held: std::sync::mpsc::Sender<()>,
}

impl TheDeadline {
    fn started(folder: PathBuf) -> TheDeadline {
        let (held, watched) = std::sync::mpsc::channel::<()>();
        let test = std::thread::current().name().unwrap_or("a scenario test").to_string();
        std::thread::spawn(move || {
            if watched.recv_timeout(HOW_LONG_A_DESK_MACHINE_MAY_STAND)
                == Err(std::sync::mpsc::RecvTimeoutError::Timeout)
            {
                // **Straight to the complaints stream**: the harness captures
                // `eprintln!` in a test's threads and never prints it when the
                // binary is ended from inside.
                let said = format!(
                    "DEADLINE MISSED: {test} still held its desk machine after {:?}. Every \
                     program it started is ended now, and so is this binary.\n",
                    HOW_LONG_A_DESK_MACHINE_MAY_STAND
                );
                let _ = std::io::Write::write_all(&mut std::io::stderr(), said.as_bytes());
                every_program_started_under_is_ended(&folder);
                let _ = Command::new("pkill")
                    .args(["-KILL", "-P", &std::process::id().to_string()])
                    .status();
                // **A small volume the machine's root sits on is detached
                // before the folder goes**, so nothing is left attached to this
                // Mac ([`TheSmallVolume`]). Where none was made this does
                // nothing.
                TheSmallVolume::detached_from(&folder.join(TheSmallVolume::WHERE_IT_IS_MOUNTED));
                let _ = std::fs::remove_dir_all(&folder);
                std::process::exit(101);
            }
        });
        TheDeadline { _held: held }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// A small volume for the machine's root
// ════════════════════════════════════════════════════════════════════════════

/// **A small throwaway volume, made and attached by the system's own disk-image
/// tool as the user running the test, with the machine's root mounted on it** —
/// so a scenario can fill the machine's disk to nothing without touching this
/// Mac's own. Added 2026-10-25 by step 12's SC4, *The server disk fills up
/// during render*: the machine's own program, its records and the job's frames
/// all sit on one disk there, as on a rental with one disk.
///
/// **The tool, and what its manual says.** `hdiutil(1)`, read 2026-10-25 on
/// macOS 26.6.2 (`man hdiutil`). *Documented*: `create` with `-size`, `-fs` and
/// `-type SPARSE` makes *"a read/write single-file image which expands as is is
/// filled with data"*; `attach` with `-mountpoint path` mounts the one volume
/// *"at path instead of in /Volumes"*, and `-nobrowse` renders it *"invisible in
/// applications such as the macOS Finder"*; `detach` takes a mount point and
/// `-force` *"ignore[s] open files on mounted volumes"*. The manual marks one
/// attach option as needing an administrator — `-notremovable`, *"Only root can
/// use this option"* — and no other; that option is not used. *Observed* the
/// same day: the image is made, attached, written and detached as this user,
/// with no administrator asked.
///
/// **The image grows only as it fills and is capped at the size asked for**, so
/// the most it can take of this Mac's disk is that size; it sits in the
/// machine's throwaway folder and goes with it.
struct TheSmallVolume {
    mounted_at: PathBuf,
}

impl TheSmallVolume {
    /// Where, under the machine's throwaway folder, it is mounted: the
    /// machine's root.
    const WHERE_IT_IS_MOUNTED: &'static str = "home/root";

    /// **Made, and attached at `at`**, or why not.
    fn made(folder: &Path, at: &Path, megabytes: u64) -> Result<TheSmallVolume, String> {
        if !cfg!(target_os = "macos") {
            return Err(format!(
                "no small volume is made on this system ({}): the tool this desk uses is macOS's own",
                std::env::consts::OS
            ));
        }
        std::fs::create_dir_all(at).map_err(|why| format!("the mount point: {why}"))?;
        let image = folder.join("the machine's disk");
        let made = Command::new("hdiutil")
            .args(["create", "-quiet", "-size", &format!("{megabytes}m"), "-fs", "HFS+", "-volname", "smzy-machine", "-type", "SPARSE"])
            .arg(&image)
            .output()
            .map_err(|why| format!("hdiutil could not be run: {why}"))?;
        if !made.status.success() {
            return Err(format!("hdiutil made no image: {}", String::from_utf8_lossy(&made.stderr).trim()));
        }
        let attached = Command::new("hdiutil")
            .arg("attach")
            .arg(image.with_extension("sparseimage"))
            .arg("-mountpoint")
            .arg(at)
            .args(["-nobrowse", "-quiet"])
            .output()
            .map_err(|why| format!("hdiutil could not be run: {why}"))?;
        if !attached.status.success() {
            return Err(format!("hdiutil attached nothing: {}", String::from_utf8_lossy(&attached.stderr).trim()));
        }
        Ok(TheSmallVolume { mounted_at: at.to_path_buf() })
    }

    /// **Detached, open files or not.** Nothing where nothing is mounted there.
    fn detached_from(at: &Path) {
        if cfg!(target_os = "macos") && at.exists() {
            let _ = Command::new("hdiutil")
                .arg("detach")
                .arg(at)
                .args(["-force", "-quiet"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
}

impl Drop for TheSmallVolume {
    fn drop(&mut self) {
        TheSmallVolume::detached_from(&self.mounted_at);
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The far end: OpenSSH's own server
// ════════════════════════════════════════════════════════════════════════════

/// **OpenSSH's own server, ready to take one connection at a time on a socket
/// it is handed**, with a host key and the client's key made by OpenSSH's own
/// `ssh-keygen`, its sessions in the machine's home.
struct TheFarEnd {
    sshd: PathBuf,
    config: PathBuf,
    client_key: PathBuf,
    host_public_key: PathBuf,
}

impl TheFarEnd {
    fn made(
        folder: &Path,
        home: &Path,
        bin: &Path,
        sshd: &Path,
        sftp_server: &Path,
    ) -> Result<TheFarEnd, String> {
        for key in ["host", "client"] {
            let made = Command::new("ssh-keygen")
                .args(["-q", "-t", "ed25519", "-N", "", "-C", key, "-f"])
                .arg(folder.join(key))
                .status()
                .map_err(|why| format!("ssh-keygen could not be run: {why}"))?;
            if !made.success() {
                return Err("ssh-keygen did not make a key".to_string());
            }
        }
        std::fs::copy(folder.join("client.pub"), folder.join("authorized_keys"))
            .map_err(|why| format!("the authorised keys: {why}"))?;
        let config = folder.join("sshd_config");
        // **`HOME` and `PATH` set for every session**, so `~` is the machine's
        // home and the commands a rental answers in GNU's shape find their
        // stand-ins first; the file-transfer server starts in that home.
        std::fs::write(
            &config,
            format!(
                "HostKey {}\nAuthorizedKeysFile {}\nPidFile none\nUsePAM no\nStrictModes no\n\
                 PasswordAuthentication no\nKbdInteractiveAuthentication no\n\
                 Subsystem sftp {} -d {}\nSetEnv HOME={} PATH={}:/usr/bin:/bin\n",
                folder.join("host").display(),
                folder.join("authorized_keys").display(),
                sftp_server.display(),
                home.display(),
                home.display(),
                bin.display(),
            ),
        )
        .map_err(|why| format!("the server's configuration: {why}"))?;
        // **The server's own check of its configuration**, so a system whose
        // `sshd` cannot run for this user is said and skipped rather than
        // failed.
        let checked = Command::new(sshd)
            .args(["-t", "-f"])
            .arg(&config)
            .output()
            .map_err(|why| format!("sshd could not be run: {why}"))?;
        if !checked.status.success() {
            return Err(format!(
                "sshd will not run here: {}",
                String::from_utf8_lossy(&checked.stderr).trim()
            ));
        }
        Ok(TheFarEnd {
            sshd: sshd.to_path_buf(),
            config,
            client_key: folder.join("client"),
            host_public_key: folder.join("host.pub"),
        })
    }

    /// **One connection's server**: `sshd -i`, its standard input and output
    /// one end of a loopback TCP pair, whose other end is handed back. **TCP
    /// rather than a Unix pair**, because macOS's own `sshd` will not open an
    /// audit session on a socket with no address, and refuses the login.
    fn one_connection(&self) -> std::io::Result<(Child, tokio::net::TcpStream)> {
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
        let ours = std::net::TcpStream::connect(listener.local_addr()?)?;
        let (theirs, _) = listener.accept()?;
        ours.set_nonblocking(true)?;
        let reading = unsafe { Stdio::from_raw_fd(a_copy_of(theirs.as_raw_fd())?) };
        let writing = unsafe { Stdio::from_raw_fd(a_copy_of(theirs.as_raw_fd())?) };
        let child = Command::new(&self.sshd)
            .args(["-i", "-e", "-f"])
            .arg(&self.config)
            .stdin(reading)
            .stdout(writing)
            .stderr(
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(self.config.with_file_name("sshd.log"))
                    .map(Stdio::from)
                    .unwrap_or_else(|_| Stdio::null()),
            )
            .spawn()?;
        drop(theirs);
        Ok((child, tokio::net::TcpStream::from_std(ours)?))
    }
}

fn a_copy_of(descriptor: std::os::unix::io::RawFd) -> std::io::Result<std::os::unix::io::RawFd> {
    let copy = unsafe { libc::dup(descriptor) };
    if copy < 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(copy)
}

// ════════════════════════════════════════════════════════════════════════════
// The line
// ════════════════════════════════════════════════════════════════════════════

/// **How a connection made before a reboot ends when the machine comes back.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheOldConnectionsEnd {
    /// **Its service closed the socket as it shut down** — `Servers` K2's
    /// first bullet: the desk reads the close.
    Politely,
    /// **The machine went down without closing anything and came back on a
    /// fresh boot that knows nothing of the old connection**, so the desk's
    /// next packet on it is answered with a reset.
    WithAReset,
}

/// **How a destroyed machine looks from the desk.** Only a real destroy shows
/// which a provider does, so a test plays the one it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheMachineGoes {
    /// **Every connection is reset and every new one is refused.**
    WithARefusal,
    /// **Every connection goes silent for good and a new one is answered by
    /// nothing** — the address no longer leads anywhere.
    InSilence,
}

/// One connection the line carries: its two halves, and how it was ended.
struct OneConnection {
    towards_the_machine: watch::Sender<bool>,
    back_to_the_desk: watch::Sender<bool>,
    ended: watch::Sender<Option<HowTheOldConnectionsEnd>>,
}

/// **The line between the desk and the machine, one relay under every
/// connection.** Up, a half carries every byte. **Silent, it carries nothing
/// and closes nothing**: what is sent stays unread, which is what a provider's
/// firewall dropping every packet looks like from either end. **Each
/// connection it carries is an OpenSSH server of its own.**
pub struct TheLine {
    address: SocketAddr,
    /// **What a connection made now starts as**, both halves.
    the_route: watch::Sender<bool>,
    each: Arc<Mutex<Vec<OneConnection>>>,
    servers: Arc<Mutex<Vec<Child>>>,
    accepting: Mutex<Option<tokio::task::JoinHandle<()>>>,
    /// **Gone for good**: nothing brings the line back.
    gone: std::sync::atomic::AtomicBool,
}

impl TheLine {
    async fn to(far_end: Arc<TheFarEnd>) -> std::io::Result<TheLine> {
        let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0)).await?;
        let address = listener.local_addr()?;
        let (the_route, route) = watch::channel(true);
        let each: Arc<Mutex<Vec<OneConnection>>> = Arc::default();
        let servers: Arc<Mutex<Vec<Child>>> = Arc::default();
        let accepting = tokio::spawn({
            let (each, servers) = (Arc::clone(&each), Arc::clone(&servers));
            async move {
                while let Ok((desk, _)) = listener.accept().await {
                    let Ok((server, machine)) = far_end.one_connection() else {
                        continue;
                    };
                    if let Ok(mut held) = servers.lock() {
                        held.push(server);
                    }
                    // **A socket closed with no linger is closed with a
                    // reset**, which is how the desk's half is ended when the
                    // machine comes back knowing nothing of it. Closed
                    // politely, its close is said first.
                    #[allow(deprecated)]
                    let _ = desk.set_zero_linger();
                    let carrying = *route.borrow();
                    let (towards, _) = watch::channel(carrying);
                    let (back, _) = watch::channel(carrying);
                    let (ended, _) = watch::channel(None);
                    let (desk_reads, desk_writes) = tokio::io::split(desk);
                    let (machine_reads, machine_writes) = tokio::io::split(machine);
                    tokio::spawn(one_direction(desk_reads, machine_writes, towards.subscribe(), ended.subscribe()));
                    tokio::spawn(one_direction(machine_reads, desk_writes, back.subscribe(), ended.subscribe()));
                    if let Ok(mut held) = each.lock() {
                        held.push(OneConnection {
                            towards_the_machine: towards,
                            back_to_the_desk: back,
                            ended,
                        });
                    }
                }
            }
        });
        Ok(TheLine {
            address,
            the_route,
            each,
            servers,
            accepting: Mutex::new(Some(accepting)),
            gone: std::sync::atomic::AtomicBool::new(false),
        })
    }

    /// The address the desk connects to.
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// How many connections the line has carried, from the first.
    pub fn connections_made(&self) -> usize {
        self.each.lock().expect("the connections").len()
    }

    fn every_connection_still_carried(&self, mut each: impl FnMut(&OneConnection)) {
        let held = self.each.lock().expect("the connections");
        held.iter().filter(|one| one.ended.borrow().is_none()).for_each(&mut each);
    }

    /// **The line goes silent, both ways**, under every connection and for
    /// every new one, until [`TheLine::comes_back`].
    pub fn goes_silent(&self) {
        self.the_route.send_replace(false);
        self.every_connection_still_carried(|one| {
            one.towards_the_machine.send_replace(false);
            one.back_to_the_desk.send_replace(false);
        });
    }

    /// **Only the half coming back goes silent**: what the desk sends still
    /// reaches the machine and the machine acts on it, but its answer never
    /// arrives — nor any answer to a keepalive.
    pub fn holds_the_answers(&self) {
        self.the_route.send_replace(false);
        self.every_connection_still_carried(|one| {
            one.back_to_the_desk.send_replace(false);
        });
    }

    /// **The line comes back**, under every connection and for every new one.
    /// Nothing comes back to a destroyed machine.
    pub fn comes_back(&self) {
        if self.gone.load(Ordering::SeqCst) {
            return;
        }
        self.the_route.send_replace(true);
        self.every_connection_still_carried(|one| {
            one.towards_the_machine.send_replace(true);
            one.back_to_the_desk.send_replace(true);
        });
    }

    /// **Every connection carried now is ended the way `how` says**, and the
    /// line carries none of them again.
    fn every_connection_ends(&self, how: HowTheOldConnectionsEnd) {
        self.every_connection_still_carried(|one| {
            one.ended.send_replace(Some(how));
        });
        if let Ok(mut servers) = self.servers.lock() {
            for server in servers.iter_mut() {
                let _ = server.kill();
                let _ = server.wait();
            }
            servers.clear();
        }
    }

    /// **Nothing more is accepted, and the listener is closed before this
    /// returns** — so a refusal is in place before anything the desk could
    /// react to has happened.
    async fn stops_accepting(&self) {
        let accepting = self.accepting.lock().expect("the listener").take();
        if let Some(accepting) = accepting {
            accepting.abort();
            let _ = accepting.await;
        }
    }

    /// **Gone for good**: every connection ends and nothing more is accepted
    /// (`WithARefusal`), or every connection and every new one goes silent
    /// for ever (`InSilence`).
    fn is_gone(&self, how: HowTheMachineGoes) {
        self.gone.store(true, Ordering::SeqCst);
        match how {
            HowTheMachineGoes::WithARefusal => {
                // **Nothing is accepted from the moment the machine is gone,
                // and that comes before the reset**: the reset is what starts
                // the repair, whose first attempt is made at once, and with
                // the listener still open that attempt reached a fresh
                // server on a machine that no longer exists and connected
                // (one run in four, 2026-10-24).
                if let Some(accepting) = self.accepting.lock().expect("the listener").take() {
                    accepting.abort();
                }
                self.every_connection_ends(HowTheOldConnectionsEnd::WithAReset);
            }
            HowTheMachineGoes::InSilence => self.goes_silent(),
        }
    }
}

impl Drop for TheLine {
    fn drop(&mut self) {
        if let Ok(mut accepting) = self.accepting.lock() {
            if let Some(accepting) = accepting.take() {
                accepting.abort();
            }
        }
        self.every_connection_still_carried(|one| {
            one.ended.send_replace(Some(HowTheOldConnectionsEnd::WithAReset));
        });
        if let Ok(mut servers) = self.servers.lock() {
            for server in servers.iter_mut() {
                let _ = server.kill();
                let _ = server.wait();
            }
        }
    }
}

async fn until_it_is(up: &mut watch::Receiver<bool>, wanted: bool) {
    loop {
        if *up.borrow_and_update() == wanted {
            return;
        }
        if up.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
    }
}

async fn until_it_is_ended(ended: &mut watch::Receiver<Option<HowTheOldConnectionsEnd>>) -> HowTheOldConnectionsEnd {
    loop {
        if let Some(how) = *ended.borrow_and_update() {
            return how;
        }
        if ended.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
    }
}

/// **One direction of one connection**, carried while its half is up, and
/// ended the way the machine ends it. A read is given up the moment the half
/// goes silent, and a socket keeps what it has not handed over, so nothing is
/// lost and nothing is read while it is silent.
async fn one_direction<R, W>(
    mut from: R,
    mut to: W,
    mut up: watch::Receiver<bool>,
    mut ended: watch::Receiver<Option<HowTheOldConnectionsEnd>>,
) where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut buffer = vec![0u8; 64 * 1024];
    let carried = async {
        loop {
            until_it_is(&mut up, true).await;
            let read = tokio::select! {
                read = from.read(&mut buffer) => read,
                () = until_it_is(&mut up, false) => continue,
            };
            let got = match read {
                Ok(0) | Err(_) => return,
                Ok(got) => got,
            };
            until_it_is(&mut up, true).await;
            if to.write_all(&buffer[..got]).await.is_err() {
                return;
            }
        }
    };
    let ends = tokio::select! {
        () = carried => HowTheOldConnectionsEnd::Politely,
        ends = until_it_is_ended(&mut ended) => ends,
    };
    // **Said politely, the close goes on to the other end**; ended with a
    // reset, nothing is said, and the desk's socket — dropped with no linger
    // once both directions have let it go — answers with a reset.
    if ends == HowTheOldConnectionsEnd::Politely {
        let _ = to.shutdown().await;
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The stand-in Blender
// ════════════════════════════════════════════════════════════════════════════

/// **A stand-in for Blender, and it is named as one.** A Perl script at the
/// path the job's settings name as its Blender. It answers the render
/// worker's three invocations — `Rendering` L14's version ask, `Job files and
/// outputs` R1's reading, and the render — and nothing else.
///
/// - **The version ask**: the version, between the reading's own marks.
/// - **The reading**: the document [`TheStandInBlender::prints_a_reading_of`]
///   last wrote, between the same marks — a document of this workspace's own
///   shapes, one destination, the scene's own output path.
/// - **The render**: the material's marker head first, because that is what
///   `Rented-server runtime` M28 observes; then **one frame at a time, on its
///   own clock**, each written where the attempt's expected-output inventory
///   predicts it — read from the recorded reading the render worker wrote
///   beside the launch material. A frame a test holds is waited at.
///   **Only the frames the render's own frame list names are written**
///   (`Rendering — machine`'s `FRAME_LIST_ARGUMENT`, a comma list of frames
///   and `..` ranges), so a re-run the machine hands only the missing frames
///   writes only those. Added 2026-10-24 by step 12's SC3: until then the
///   stand-in wrote every frame of the inventory on every attempt, whatever
///   the program asked for.
/// - **Two lines a frame on its ordinary output**: the frame begun, before
///   any hold on it, and the frame saved, once its file is written. **They
///   are written in the shapes the machine's own reader reads**
///   (`acting/reader.rs` in `App/Queue & History — machine/`, its newer
///   grammar, which the stand-in's version selects), so the worker's reports
///   carry the frames reached and the newest frame saved, and the queue's
///   figures move as frames finish. Added 2026-10-24 by step 12's SC1 and
///   SC8, both of which watch frames complete. **What those shapes are is
///   that reader's claim, carried by its own tests, and not this stand-in's.**
///
/// **Nothing here claims what a real Blender prints or does.**
pub struct TheStandInBlender {
    at: PathBuf,
    folder: PathBuf,
}

/// The version the stand-in answers to, and the one every job here asks for.
const THE_STAND_INS_VERSION: (u32, u32, u32) = (5, 2, 0);

impl TheStandInBlender {
    fn made(folder: PathBuf, perl: &Path) -> TheStandInBlender {
        let at = folder.join("blender");
        std::fs::write(folder.join("a-frame.png"), a_one_pixel_png()).expect("the frame's picture");
        std::fs::write(folder.join("seconds-a-frame"), "0.2").expect("the clock");
        let (major, minor, patch) = THE_STAND_INS_VERSION;
        let recorded_reading = smoozy_job_files::manifest::the_recorded_readings_file_name(424_242);
        let (before, after) = recorded_reading.split_once("424242").expect("the attempt is in the name");
        let opens = String::from_utf8_lossy(smoozy_job_files::reading::THE_OPENING_SENTINEL);
        let closes = String::from_utf8_lossy(smoozy_job_files::reading::THE_CLOSING_SENTINEL);
        let marker = smoozy_rendering_machine::acting::material::THE_MARKER_HEAD;
        let frame_list = smoozy_rendering_machine::spellings::FRAME_LIST_ARGUMENT
            .text()
            .expect("the frame list's spelling is on record");
        a_program(
            &at,
            &format!(
                r#"#!{perl}
# A stand-in for Blender, written by the scenarios' desk machine. Nothing about
# a real Blender is claimed by it.
use strict;
use warnings;
use JSON::PP;
use File::Basename qw(dirname);
use File::Copy qw(copy);
use File::Path qw(make_path);
$| = 1;
my $here = '{folder}';
my ($asked_the_version, $asked_a_reading, $the_material, $the_frame_list);
for my $at (0 .. $#ARGV) {{
    my $word = $ARGV[$at];
    $asked_the_version = 1 if $word =~ m{{version-ask-\d+\.py$}};
    $asked_a_reading = 1 if $word =~ m{{(^|/)reading-\d+\.py$}};
    $the_material = $word if $word =~ m{{launch-material-\d+\.py$}};
    $the_frame_list = $ARGV[$at + 1] if $word eq '{frame_list}' && $at < $#ARGV;
}}
my %asked;
if (defined $the_frame_list) {{
    for my $part (split /,/, $the_frame_list) {{
        if ($part =~ /^(-?\d+)\.\.(-?\d+)$/) {{ $asked{{$_}} = 1 for $1 .. $2; }}
        elsif ($part =~ /^-?\d+$/) {{ $asked{{$part + 0}} = 1; }}
    }}
}}
if ($asked_the_version) {{
    print '{opens}{major}.{minor}.{patch}{closes}';
    exit 0;
}}
if ($asked_a_reading) {{
    open(my $document, '<', "$here/what-the-reading-prints") or exit 1;
    local $/;
    my $text = <$document>;
    print '{opens}', $text, '{closes}';
    exit 0;
}}
exit 0 unless defined $the_material;
print "{marker}{{\"job\":\"the desk machine's stand-in\"}}\n";
my ($attempt) = $the_material =~ m{{launch-material-(\d+)\.py$}};
my $reading_at = dirname($the_material) . "/{before}$attempt{after}";
open(my $reading, '<', $reading_at) or die "no recorded reading at $reading_at\n";
my $text = do {{ local $/; <$reading> }};
my $document = JSON::PP->new->decode($text);
my $entries = $document->{{inventory}}{{It}}{{entries}} or die "no inventory in $reading_at\n";
my @frames = sort {{ ($a->{{frame}} // 0) <=> ($b->{{frame}} // 0) }} @$entries;
for my $entry (@frames) {{
    my $frame = $entry->{{frame}} // 0;
    next if defined $the_frame_list && !$asked{{$frame}};
    my $path = $entry->{{predicted_path}};
    $path = join('', map {{ chr($_) }} @$path) if ref($path) eq 'ARRAY';
    print "00:00.000  render           | Rendering frame $frame\n";
    while (-e "$here/hold-before-frame-$frame") {{
        select(undef, undef, undef, 0.05);
    }}
    my $seconds = 0.2;
    if (open(my $clock, '<', "$here/seconds-a-frame")) {{
        my $read = <$clock>;
        $seconds = $read + 0 if defined $read;
    }}
    select(undef, undef, undef, $seconds);
    make_path(dirname($path));
    copy("$here/a-frame.png", $path) or die "frame $frame could not be written at $path\n";
    print "00:00.000  render           | Saved: '$path'\n";
    if (open(my $log, '>>', "$here/frames-written")) {{
        print $log "$frame\n";
    }}
}}
exit 0;
"#,
                perl = perl.display(),
                folder = folder.display(),
                frame_list = frame_list,
            ),
        );
        let stand_in = TheStandInBlender { at, folder };
        stand_in.prints_a_reading_of(1..=1);
        stand_in
    }

    /// Where the stand-in is: the path a job's settings name as its Blender.
    pub fn path(&self) -> &Path {
        &self.at
    }

    /// **How long the stand-in takes over each frame**, from the next frame
    /// on.
    pub fn takes_this_long_a_frame(&self, each: Duration) {
        std::fs::write(self.folder.join("seconds-a-frame"), format!("{}", each.as_secs_f64()))
            .expect("the clock");
    }

    /// **The render waits before writing this frame**, until
    /// [`TheStandInBlender::lets_frame_go`].
    pub fn holds_before_frame(&self, frame: i64) {
        std::fs::write(self.folder.join(format!("hold-before-frame-{frame}")), b"").expect("the hold");
    }

    /// The hold on this frame is let go.
    pub fn lets_frame_go(&self, frame: i64) {
        let _ = std::fs::remove_file(self.folder.join(format!("hold-before-frame-{frame}")));
    }

    /// **Every frame the stand-in has written**, in the order written, by
    /// every render it was ever started for.
    pub fn frames_written(&self) -> Vec<i64> {
        std::fs::read_to_string(self.folder.join("frames-written"))
            .unwrap_or_default()
            .lines()
            .filter_map(|line| line.trim().parse().ok())
            .collect()
    }

    /// **The document its reading prints from now on**: one scene, *Scene*,
    /// over `frames`, rendered on the processor with Cycles, writing PNG files
    /// through the scene's own output path.
    ///
    /// **On the processor, and that is deliberate**: a cards job looks its
    /// technology up from the machine's card make (`Rendering` L23), which
    /// this desk's machine has none of.
    pub fn prints_a_reading_of(&self, frames: std::ops::RangeInclusive<i64>) {
        std::fs::write(
            self.folder.join("what-the-reading-prints"),
            the_document_a_reading_prints(frames),
        )
        .expect("the document is written");
    }
}

/// **What a reading of a `.blend` prints, as the stand-in prints it** — a
/// document of this workspace's own shapes (`Job files and outputs` R4 items
/// 1 to 6, and `Render Settings`' settings half), copied from the machine
/// program's own `tests/the_program_runs.rs`. **Nothing here says what a real
/// Blender prints.**
fn the_document_a_reading_prints(frames: std::ops::RangeInclusive<i64>) -> String {
    use smoozy_job_files::manifest::CarriedAndNotInterpretedHere;
    use smoozy_job_files::reading::{
        ArtistsText, AtOneFrame, Destination, DestinationKind, Format, GatingConditions,
        HowManyViewsTheSceneWrites, RecordedLocation, TheVersionInHand, WhereItComesFrom, WhichScene,
    };
    use smoozy_job_files::reading_script::{TheObservedOutputs, ThePrintedReading};
    use smoozy_render_settings::names::ExactName;
    use smoozy_render_settings::reading::{
        DeviceSetting, EarlyStop, LayerSamplesRule, OfferedEngineIdentifiers, PerEngineCounts,
        SceneReading, SettingsHalf, SimplifyReadBack, SwitchAnimation, ViewLayerReading,
    };
    let layer = || ArtistsText::of(b"ViewLayer".to_vec());
    // **No compositing tree at all**: the raw render lands at the regular
    // output path and no walk is needed (`Job files and outputs` B5).
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
    let (first, last) = (*frames.start(), *frames.end());
    let count = u32::try_from(last - first + 1).unwrap_or(1);
    let scene = SceneReading {
        prepare_simulations: None,
            motion_blur: None,
        name: ExactName::as_the_reading_produced_it("Scene"),
        frame_start: first,
        frame_end: last,
        frame_step: 1,
        frame_count: count.into(),
        full_size: (640, 360),
        resolution_percentage: 100,
        size_at_the_files_percentage: (640, 360),
        engine_as_read: ExactName::as_the_reading_produced_it("CYCLES"),
        sample_counts: PerEngineCounts { cycles: 16, eevee: 16 },
        device: DeviceSetting::TheProcessor,
        early_stop: EarlyStop {
            adaptive_sampling_is_on: true,
            threshold: 0.01,
            minimum: 0,
        },
        time_limit: 0.0,
        eevee_time_limit: None,
        layer_samples_rule: LayerSamplesRule::Use,
        saved_with: "5.2.0".to_string(),
        view_layers: vec![ViewLayerReading {
            name: ExactName::as_the_reading_produced_it("ViewLayer"),
            renders_at: frames.clone().map(|frame| (frame, true)).collect(),
            animation: SwitchAnimation::None { value: true },
            own_sample_count: 0,
            overrides_the_scenes_count: false,
        }],
        pulls_in: Vec::new(),
        simplify: SimplifyReadBack {
            use_simplify: false,
            subdivision_render: 6,
            child_particles_render: 1.0,
            texture_limit_render: "OFF".to_string(),
        },
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
            per_frame: frames
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
        settings: CarriedAndNotInterpretedHere(
            serde_json::to_value(SettingsHalf {
                scenes: vec![scene],
                offered_engine_identifiers: OfferedEngineIdentifiers {
                    cycles: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                    eevee: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE")),
                },
                announced_add_on_case: None,
                the_folders_at_the_machines_top: None,
            })
            .expect("the settings half renders"),
        ),
        dependencies: CarriedAndNotInterpretedHere(serde_json::json!({})),
        errors: std::collections::BTreeMap::new(),
        the_baked_caches_on_the_machine: Vec::new(),
        the_scene_holds_a_simulation_not_baked: false,
        // **The scene it read, by name** — the one scene above. A reading that
        // names none leaves the entry's picture size not known (`Queue &
        // History` N9's fill-in looks the size up by the scene's name). Added
        // 2026-10-24 by step 12's SC1 and SC8, whose entries draw it.
        the_scene_it_read: Some(ArtistsText::of(b"Scene".to_vec())),
        the_open_files_path: None,
        the_check_is_written_as_a_picture: false,
    };
    serde_json::to_string(&printed).expect("the document renders")
}

/// **PNG's chunk check**, CRC-32 over the chunk's name and data.
fn crc_32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 == 1 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

/// **A real PNG of one grey pixel**, so a frame the stand-in writes opens as
/// a picture: the signature, a header, one stored zlib block, and the end.
fn a_one_pixel_png() -> Vec<u8> {
    fn chunk(out: &mut Vec<u8>, kind: &[u8], data: &[u8]) {
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        let mut body = kind.to_vec();
        body.extend_from_slice(data);
        out.extend_from_slice(&body);
        out.extend_from_slice(&crc_32(&body).to_be_bytes());
    }
    let scanline = [0u8, 128, 128, 128];
    let (mut a, mut b) = (1u32, 0u32);
    for byte in scanline {
        a = (a + u32::from(byte)) % 65_521;
        b = (b + a) % 65_521;
    }
    let mut zlib = vec![0x78, 0x01, 0x01, 0x04, 0x00, 0xFB, 0xFF];
    zlib.extend_from_slice(&scanline);
    zlib.extend_from_slice(&((b << 16) | a).to_be_bytes());
    let mut png = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    chunk(&mut png, b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 2, 0, 0, 0]);
    chunk(&mut png, b"IDAT", &zlib);
    chunk(&mut png, b"IEND", &[]);
    png
}

// ════════════════════════════════════════════════════════════════════════════
// The machine's own program
// ════════════════════════════════════════════════════════════════════════════

/// **The machine's program in its package area**, and its supervisor.
struct TheMachinesOwnProgram {
    area: PathBuf,
    endpoint: PathBuf,
    home: PathBuf,
    the_path_its_programs_search: String,
    complaints: PathBuf,
    supervisor: Mutex<Option<Child>>,
}

impl TheMachinesOwnProgram {
    /// **The package area a placement would have written**: the pointer to
    /// the built program, the endpoint's recorded path, the installation
    /// identity, the phase record, and the root (`Job files and outputs` L1).
    fn placed(
        built: &Path,
        layout: &TheLayout,
        home: &Path,
        bin: &Path,
        installation: &Identity,
        complaints: PathBuf,
    ) -> Result<TheMachinesOwnProgram, String> {
        use smoozy_runtime::the_program::{records, the_active_package_pointer_at, the_record_at, THE_FIXED_ENTRY_POINT_NAME};
        let area = PathBuf::from(layout.area(Area::Package));
        let area_text = area.display().to_string();
        let written = |at: String, what: &[u8]| std::fs::write(&at, what).map_err(|why| format!("{at}: {why}"));
        std::os::unix::fs::symlink(built, the_active_package_pointer_at(&area_text))
            .map_err(|why| format!("the active-package pointer: {why}"))?;
        // **A short endpoint path**: a socket's name has a length limit.
        let endpoint = std::env::temp_dir().join(format!(
            "smzy-{}.sock",
            &Identity::issue(IdentityKind::Operation).map_err(|_| "no randomness")?.as_text()[..12]
        ));
        written(
            the_record_at(&area_text, records::THE_ENDPOINTS_RECORDED_PATH),
            endpoint.display().to_string().as_bytes(),
        )?;
        written(the_record_at(&area_text, records::THE_INSTALLATION_IDENTITY), installation.as_text().as_bytes())?;
        let phase = smoozy_runtime::package::phase::a_fresh_attempts_first_write(
            Identity::issue(IdentityKind::Operation).map_err(|_| "no randomness")?,
            smoozy_runtime::package::release::VerifiedReleaseDescriptor {
                manifest_format: "1".to_string(),
                verified_release_sequence: 1,
                requested_package_version: env!("CARGO_PKG_VERSION").to_string(),
                manifest_digest: [0; 32],
                artifact_identity: THE_FIXED_ENTRY_POINT_NAME.to_string(),
                artifact_byte_length: 0,
                artifact_digest: [0; 32],
            },
            None,
            smoozy_runtime::machine_kind::MachineKind::Systemd,
            area.join("staged").display().to_string(),
        );
        written(
            the_record_at(&area_text, records::THE_PHASE_RECORD),
            &smoozy_runtime::written_form::write(&phase).map_err(|why| format!("the phase record: {why:?}"))?,
        )?;
        written(
            the_record_at(&area_text, smoozy_job_files::layout::THE_RECORD_OF_THE_ROOT),
            layout.root().as_path().as_bytes(),
        )?;
        Ok(TheMachinesOwnProgram {
            area,
            endpoint,
            home: home.to_path_buf(),
            the_path_its_programs_search: format!("{}:/usr/bin:/bin", bin.display()),
            complaints,
            supervisor: Mutex::new(None),
        })
    }

    /// **The supervisor started through the pointer, as boot registration
    /// starts it, and waited for until it answers `Rented-server runtime`
    /// G12's question.** Its environment is what every worker it starts
    /// inherits, so the Telegram switch reaches the Telegram worker and the
    /// stand-in commands reach every program it runs.
    fn the_supervisor_starts(&self) -> Result<(), String> {
        use smoozy_runtime::the_program::{the_active_package_pointer_at, THE_SUPERVISOR_WORD};
        let complaints = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.complaints)
            .map_err(|why| format!("the complaints file: {why}"))?;
        let supervisor = Command::new(the_active_package_pointer_at(&self.area.display().to_string()))
            .arg(THE_SUPERVISOR_WORD)
            .current_dir(&self.home)
            .env("HOME", &self.home)
            .env("PATH", &self.the_path_its_programs_search)
            .env(smoozy_telegram::transport::THE_DEVELOPMENT_SWITCH_THAT_REACHES_NOTHING, "1")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::from(complaints))
            .spawn()
            .map_err(|why| format!("the supervisor would not start: {why}"))?;
        *self.supervisor.lock().expect("the supervisor") = Some(supervisor);
        let asked = smoozy_runtime::acting::endpoint::TheEndpointAsACaller::at(
            &self.endpoint.display().to_string(),
            Duration::from_secs(1),
        )
        .ask_waiting_for_a_supervisor(&a_request(smoozy_server_protocol::requests::THE_HEALTH_ANSWER_REQUEST), HOW_LONG_THE_SUPERVISOR_IS_GIVEN)
        .map_err(|why| format!("the endpoint is not reachable at all: {why}"))?;
        if asked.is_none() {
            return Err(format!(
                "the machine's own supervisor never answered: {}",
                String::from_utf8_lossy(&std::fs::read(&self.complaints).unwrap_or_default())
            ));
        }
        Ok(())
    }

    /// **Every program on the machine stops at once**: the supervisor, and
    /// every program whose command line names the machine's home — its
    /// workers, the stand-in Blender, the adapters and file-transfer servers
    /// of every session.
    fn everything_stops(&self) {
        if let Some(mut supervisor) = self.supervisor.lock().ok().and_then(|mut held| held.take()) {
            let _ = supervisor.kill();
            let _ = supervisor.wait();
        }
        every_program_started_under_is_ended(&self.home);
    }

    fn the_supervisor_is_running(&self) -> bool {
        self.supervisor
            .lock()
            .ok()
            .and_then(|mut held| held.as_mut().map(|child| matches!(child.try_wait(), Ok(None))))
            .unwrap_or(false)
    }
}

impl Drop for TheMachinesOwnProgram {
    fn drop(&mut self) {
        self.everything_stops();
        let _ = std::fs::remove_file(&self.endpoint);
    }
}

/// One request under a name, freshly identified.
pub fn a_request(name: &str) -> Request {
    Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation).expect("randomness"),
        name,
    )
}

// ════════════════════════════════════════════════════════════════════════════
// The desk: the local record and the temporary area
// ════════════════════════════════════════════════════════════════════════════

/// **The application's own local record, on a throwaway file**, lent the way
/// the running application lends it: one act at a time.
///
/// **It is held inside the application's own start value** (`Started`), the
/// way the running application holds it, so an act that takes the start whole
/// — History's delete, the run's publisher of what `Servers` supplies — is
/// handed the same record every other act here writes. **One connection to the
/// file and no second**: two would meet each other's locks. The start carries
/// no diary, and declares the one condition `Servers` raises. Changed
/// 2026-10-24 by step 12's SC1 and SC8; until then the record stood alone.
pub struct TheDesksRecord {
    started: Arc<Mutex<Started>>,
}

/// **The record, held while this lives** — for a screen's reading, which
/// takes the record whole.
pub struct TheRecordHeld<'a>(std::sync::MutexGuard<'a, Started>);

impl std::ops::Deref for TheRecordHeld<'_> {
    type Target = LocalRecord;

    fn deref(&self) -> &LocalRecord {
        self.0.record.as_ref().expect("the desk's start holds its record")
    }
}

impl TheDesksRecord {
    fn over(record: LocalRecord) -> TheDesksRecord {
        let mut what_modules_declare = smoozy_desktop::presentation::declarations::TheDeclarations::new();
        what_modules_declare.module_declares(
            smoozy_desktop::servers::published::THE_MODULE,
            smoozy_desktop::servers::published::what_servers_declares(),
        );
        TheDesksRecord {
            started: Arc::new(Mutex::new(Started {
                the_sweep_left_behind: Vec::new(),
                record: Some(record),
                record_trouble: None,
                diary: None,
                declarations: Default::default(),
                what_modules_declare,
                said_once: smoozy_desktop::presentation::said_once::TheThingsSaidOnce::new(),
                run: Identity::issue(IdentityKind::Run).ok(),
                the_previous_run: smoozy_state::run_mark::WhatTheLaunchFound::NoPreviousRun,

            })),
        }
    }

    /// One act against the record.
    pub fn read<T>(&self, act: impl FnOnce(&rusqlite::Connection) -> T) -> T {
        let record = self.held();
        act(record.connection())
    }

    /// The record itself, held while the guard lives — for a screen's
    /// reading, which takes the record whole.
    pub fn held(&self) -> TheRecordHeld<'_> {
        TheRecordHeld(self.started.lock().expect("the record"))
    }

    /// **The application's start value holding this record**, for an act
    /// that takes it whole.
    pub fn the_start(&self) -> &Arc<Mutex<Started>> {
        &self.started
    }
}

impl smoozy_desktop::uploading::TheRecordAnUploadWrites for TheDesksRecord {
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
        let Ok(started) = self.started.lock() else {
            return false;
        };
        let Some(record) = started.record.as_ref() else {
            return false;
        };
        act(record.connection());
        true
    }
}

impl smoozy_desktop::downloading::TheRecordADownloadWrites for TheDesksRecord {
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
        <TheDesksRecord as smoozy_desktop::uploading::TheRecordAnUploadWrites>::with_the_record(self, act)
    }
}

impl smoozy_desktop::keys::LendsTheRecord for TheDesksRecord {
    fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
        <TheDesksRecord as smoozy_desktop::uploading::TheRecordAnUploadWrites>::with_the_record(self, act);
    }

    fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
}

/// **The user's computer, as far as a scenario is concerned**: a local record
/// and Smoozy's temporary area, in a throwaway folder removed whole. **It
/// outlives the machines stood up against it**, so a scenario can remove one
/// machine and add another to the same Smoozy.
pub struct TheDesk {
    record: Arc<TheDesksRecord>,
    temporary_area: PathBuf,
    /// Where the user's own files sit, for a scenario that sends one up or
    /// brings frames down.
    the_users_folder: PathBuf,
    folder: TheThrowawayFolder,
}

impl TheDesk {
    /// A fresh record, migrated as the application opens it, or why not.
    pub fn opened(named: &str) -> Result<TheDesk, String> {
        let folder = TheThrowawayFolder::made("desk", named)?;
        let record = LocalRecord::open_at(&folder.folder("record").join("smoozy.sqlite3"))
            .map_err(|why| format!("the record would not open: {why:?}"))?;
        Ok(TheDesk {
            record: Arc::new(TheDesksRecord::over(record)),
            temporary_area: folder.folder("temporary"),
            the_users_folder: folder.folder("the users own folder"),
            folder,
        })
    }

    pub fn record(&self) -> &Arc<TheDesksRecord> {
        &self.record
    }

    /// Smoozy's own temporary area on this computer.
    pub fn temporary_area(&self) -> &Path {
        &self.temporary_area
    }

    /// A folder of the user's own, on this computer.
    pub fn the_users_folder(&self) -> &Path {
        &self.the_users_folder
    }

    /// The throwaway folder everything of the desk's sits in.
    pub fn at(&self) -> &Path {
        &self.folder.0
    }
}

// ════════════════════════════════════════════════════════════════════════════
// A whole job's download, as an entry's *Download all to…* asks for it
// ════════════════════════════════════════════════════════════════════════════

/// **What a History or queue entry's *Download all to…* came to**, played the
/// way `a_whole_job_download_is_asked_for` in the desktop's
/// `src/surface/transfers.rs` composes it.
#[derive(Debug)]
pub struct TheWholeJobDownloaded {
    /// **`Download` M6's pre-flight**: what it found to put in front of the
    /// user before anything moved.
    pub the_pre_flight: smoozy_download::request::WhatThePreFlightHasToSay,
    /// Whether it stopped and asked, so the user answered with `Download`
    /// `Download` J10's go-ahead (*Download*), as nothing was already there.
    pub it_asked: bool,
    /// **What the request came to**, counted the way `Download` M16 counts.
    pub came_to: smoozy_desktop::downloading::WhatTheDownloadCameToInTheEnd,
    /// Every file now under the folder the user chose, by its path.
    pub landed: Vec<PathBuf>,
}

/// **`Download` J9's reading off the local job records alone**, mirrored from
/// the desktop's private `TheJobRecordsHere` in `src/surface/transfers.rs`,
/// which a test cannot name: the ending is the record's, the file's frame is
/// absent, and the picked folder is the job's own (`Download` M9, `Download` P10).
struct TheJobRecordsAtTheDesk<'a> {
    record: &'a TheDesksRecord,
    job: Identity,
    the_jobs_output_folder: Vec<u8>,
    label: Option<String>,
    smoozys_own_copies: Vec<Vec<u8>>,
}

impl smoozy_download::acting::enumerating::ReadsTheLocalJobRecords for TheJobRecordsAtTheDesk<'_> {
    fn is_a_copy_smoozy_placed(&self, source_path: &[u8]) -> bool {
        self.smoozys_own_copies.iter().any(|placed| placed.as_slice() == source_path)
    }

    fn what_the_local_record_says(&self, source_path: &[u8]) -> smoozy_download::vouching::WhatTheLocalRecordSays {
        let ending = self.record.read(|connection| {
            smoozy_desktop::record::job_records::JobRecords::over(connection)
                .read(&self.job)
                .ok()
                .flatten()
                .and_then(|held| held.the_ending_learned())
        });
        smoozy_download::vouching::WhatTheLocalRecordSays {
            the_file_is_inside_the_jobs_output_folder: source_path.starts_with(&self.the_jobs_output_folder),
            the_recorded_ending: ending,
            the_completeness_statement: None,
        }
    }

    fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
        None
    }

    fn the_job_whose_output_folder_this_is(&self, picked: &[u8]) -> Option<smoozy_download::acting::enumerating::AJobsOwnFolder> {
        (picked == self.the_jobs_output_folder.as_slice()).then(|| smoozy_download::acting::enumerating::AJobsOwnFolder {
            job: self.job.clone(),
            label: self.label.clone(),
        })
    }
}

/// **`Download` N5a's read of the job's record directory, over the server's
/// point** — the desktop's crate-private `TheRecordDirectorysReadingRoute`
/// in `src/servers/a_period.rs`, mirrored line for line.
struct TheRecordDirectoryOverThePoint<'a> {
    sessions: &'a ThisServersSessions,
}

impl smoozy_download::acting::record_directory::ReadsOneFileOnTheMachine for TheRecordDirectoryOverThePoint<'_> {
    async fn read_one_whole_file(
        &mut self,
        path: &[u8],
    ) -> Result<Vec<u8>, smoozy_server_protocol::subsystem::SubsystemFailure> {
        let path = path.to_vec();
        self.sessions
            .a_reading(false, move |subsystem| {
                Box::pin(async move {
                    smoozy_download::acting::wiring::TheReadingRoute::on(subsystem)
                        .read_one_whole_file(&path)
                        .await
                })
            })
            .await
            .unwrap_or(Err(smoozy_server_protocol::subsystem::SubsystemFailure::TheChannelIsGone))
    }
}

/// **A whole job brought down to `to`, the folder the user picked in the
/// folder panel** — an entry's *Download all to…*, in
/// `a_whole_job_download_is_asked_for`'s own order: the job read off the local
/// record, `Download` N5a's list read off its record directory, the request
/// counted and prepared on Transfers (`a_download_is_counted_and_prepared`),
/// `Download` M6's question answered with `Download` J10's *Download* where it asks, and the files
/// run down (`the_download_runs`). **What needs the window is left out**: the
/// folder panel (the user's choice is `to`) and the screens told again.
///
/// Added 2026-10-24 by step 12's SC3 and SC2, on the runner's ruling on SC1
/// and SC8's finding. **It is here rather than in each test** because three
/// scenarios bring frames down (SC1 step 7, SC8 step 4, SC2's situation).
pub async fn a_whole_job_is_downloaded_to(
    desk: &TheDesk,
    sessions: &ThisServersSessions,
    job: &Identity,
    to: &Path,
) -> Result<TheWholeJobDownloaded, String> {
    use smoozy_desktop::downloading as downloading;
    let (asked, named, records_directory, attempts, folder, label) = {
        let started = desk.record.the_start().lock().map_err(|_| "the start")?;
        let record = started.record.as_ref().ok_or("the start holds no record")?;
        let run = started.run.clone().ok_or("the start holds no run")?;
        let held = smoozy_desktop::record::job_records::JobRecords::over(record.connection())
            .read(job)
            .ok()
            .flatten()
            .ok_or("the job's record would not read")?;
        let row = smoozy_desktop::record::server_rows::one(record.connection(), &held.server.local_server.as_text())
            .ok()
            .flatten()
            .ok_or("the job's server row is gone")?;
        let layout = smoozy_desktop::servers::installation::the_layout_recorded_for(&row).ok_or("the row records no layout")?;
        let paths = smoozy_job_files::publication::AJobsPaths::composed(&layout, job, held.label.as_deref())
            .map_err(|why| format!("the job's paths: {why:?}"))?;
        let folder = held
            .the_job_output_folder(&layout)
            .ok_or("the folder is made when the render starts")?
            .into_bytes();
        (
            downloading::WhatIsBeingDownloaded {
                server_row: held.server.local_server.clone(),
                installation: row
                    .last_confirmed_installation_identity
                    .as_deref()
                    .and_then(|text| Identity::read_back(IdentityKind::Installation, text)),
                destination: to.as_os_str().as_encoded_bytes().to_vec(),
                picked: vec![folder.clone()],
                run,
            },
            smoozy_desktop::surface::the_queue::what_the_job_is_called(&held),
            paths.job_record_directory.into_bytes(),
            held.what_the_note_carried.attempts,
            folder,
            held.label.clone(),
        )
    };
    let smoozys_own_copies = match smoozy_download::acting::record_directory::read_what_smoozy_placed(
        &mut TheRecordDirectoryOverThePoint { sessions },
        &records_directory,
        attempts,
    )
    .await
    {
        Ok(Some(placed)) => placed.the_copies_still_as_placed(),
        Ok(None) | Err(()) => Vec::new(),
    };
    let records = TheJobRecordsAtTheDesk {
        record: &desk.record,
        job: job.clone(),
        the_jobs_output_folder: folder,
        label,
        smoozys_own_copies,
    };
    let the_run = downloading::TheRunsTransfers::for_this_run();
    let request = Identity::issue(IdentityKind::Operation).map_err(|_| "no randomness")?;
    let mut ready = downloading::a_download_is_counted_and_prepared(
        asked,
        downloading::TheCountIsShown { the_run: &the_run, request, named },
        sessions,
        desk.record.as_ref(),
        &records,
    )
    .await
    .map_err(|why| format!("the download did not start: {why:?}"))?;
    let the_pre_flight = ready.the_pre_flight.clone();
    let it_asked = the_pre_flight.the_request_stops_and_asks();
    if it_asked && the_pre_flight.the_headline() > 0 {
        return Err(format!("`Download` M6 asks about files already there, which a fresh folder holds none of: {the_pre_flight:?}"));
    }
    downloading::the_request_goes_without_a_question(&mut ready, desk.record.as_ref())
        .map_err(|why| format!("the record would not take the request: {why:?}"))?;
    let came_to = downloading::the_download_runs(&mut ready, sessions, &the_run, desk.record.as_ref()).await;
    let mut landed = Vec::new();
    let mut folders = vec![to.to_path_buf()];
    while let Some(folder) = folders.pop() {
        for one in std::fs::read_dir(&folder).into_iter().flatten().flatten() {
            let path = one.path();
            if path.is_dir() {
                folders.push(path);
            } else {
                landed.push(path);
            }
        }
    }
    landed.sort();
    Ok(TheWholeJobDownloaded {
        the_pre_flight,
        it_asked,
        came_to,
        landed,
    })
}

// ════════════════════════════════════════════════════════════════════════════
// The desk machine
// ════════════════════════════════════════════════════════════════════════════

/// **The rented machine, at the desk.** See the head of this file.
pub struct TheDeskMachine {
    line: TheLine,
    far_end: Arc<TheFarEnd>,
    program: TheMachinesOwnProgram,
    stand_in: TheStandInBlender,
    layout: TheLayout,
    installation: Identity,
    the_row: ServerRow,
    the_blend: PathBuf,
    home: PathBuf,
    _deadline: TheDeadline,
    /// **The small volume the machine's root sits on, where one was asked
    /// for** — dropped after the machine's programs are ended and before its
    /// folder goes.
    _volume: Option<TheSmallVolume>,
    folder: TheThrowawayFolder,
}

/// The host-trust decision a test makes: whatever key is offered is taken.
struct Proceeds;

impl DecideHostTrust for Proceeds {
    fn decide(&mut self, _offered: &[u8]) -> HostTrust {
        HostTrust::Proceed
    }
}

fn the_user() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .expect("the user running the test is named")
}

impl TheDeskMachine {
    /// **The machine stood up and its supervisor answering, its server row in
    /// the desk's record under `label`** — or nothing, with the reason
    /// printed as a skip, where this desk has not got what it needs.
    pub async fn stood_up(named: &str, desk: &TheDesk, label: &str) -> Option<TheDeskMachine> {
        Self::said_where_it_cannot_stand(Self::stood_up_or_why_not(named, desk, label, None).await)
    }

    /// **The same machine, with its root — the five areas, the package area
    /// and every record the program keeps — on a small volume of its own of
    /// `megabytes`**, so the machine's disk can be filled to nothing
    /// ([`TheSmallVolume`]). Added 2026-10-25 by step 12's SC4.
    pub async fn stood_up_with_its_root_on_a_small_volume(
        named: &str,
        desk: &TheDesk,
        label: &str,
        megabytes: u64,
    ) -> Option<TheDeskMachine> {
        Self::said_where_it_cannot_stand(Self::stood_up_or_why_not(named, desk, label, Some(megabytes)).await)
    }

    fn said_where_it_cannot_stand(stood: Result<TheDeskMachine, String>) -> Option<TheDeskMachine> {
        match stood {
            Ok(machine) => Some(machine),
            Err(why) => {
                println!("SKIPPED: no desk machine could stand here ({}): {why}.", std::env::consts::OS);
                None
            }
        }
    }

    async fn stood_up_or_why_not(
        named: &str,
        desk: &TheDesk,
        label: &str,
        on_a_volume_of: Option<u64>,
    ) -> Result<TheDeskMachine, String> {
        let has = what_the_desk_has()?;
        let folder = TheThrowawayFolder::made("machine", named)?;
        let deadline = TheDeadline::started(folder.0.clone());
        let home = folder.folder("home");
        let volume = match on_a_volume_of {
            Some(megabytes) => Some(TheSmallVolume::made(
                &folder.0,
                &folder.0.join(TheSmallVolume::WHERE_IT_IS_MOUNTED),
                megabytes,
            )?),
            None => None,
        };
        let bin = folder.folder("home/bin");
        let stand_in = TheStandInBlender::made(folder.folder("home/a-stand-in-for-blender"), &has.perl);
        // **What a rental's own tools answer, where this system's answer
        // differently.** `stat` in GNU's fields, which `Render Settings` Q14
        // item 2's reading asks for; `uname` naming Linux, which `Servers` Q13
        // reads; `systemd-detect-virt` saying no container; and `find`
        // standing in for a search of the whole disk, naming the one Blender
        // this machine holds.
        a_program(
            &bin.join("stat"),
            "#!/bin/sh\nif /usr/bin/stat --version >/dev/null 2>&1; then exec /usr/bin/stat \"$@\"; fi\nfor last; do :; done\nexec /usr/bin/stat -L -f '%z %i %m %c' \"$last\"\n",
        );
        a_program(&bin.join("uname"), "#!/bin/sh\necho Linux\n");
        a_program(&bin.join("systemd-detect-virt"), "#!/bin/sh\necho none\nexit 1\n");
        a_program(&bin.join("find"), &format!("#!/bin/sh\necho '{}'\n", stand_in.path().display()));

        let layout = TheLayout::of(
            MachineRoot::chosen(&home.join("root").display().to_string())
                .map_err(|why| format!("the machine's root is refused: {why:?}"))?,
            WhereThePackageAreaSits::UnderTheRoot,
        );
        // **The five areas exist before the first job** (`Job files and
        // outputs` N11): a placement makes them.
        for area in smoozy_job_files::layout::EVERY_AREA {
            std::fs::create_dir_all(layout.area(area)).map_err(|why| format!("an area: {why}"))?;
        }
        let the_blend = PathBuf::from(layout.area(Area::Asset)).join("shot_040.blend");
        std::fs::write(&the_blend, b"BLENDER-v502 a stand-in for a .blend").map_err(|why| format!("the .blend: {why}"))?;

        let far_end = Arc::new(TheFarEnd::made(&folder.0, &home, &bin, &has.sshd, &has.sftp_server)?);
        let installation = Identity::issue(IdentityKind::Installation).map_err(|_| "no randomness")?;
        let program = TheMachinesOwnProgram::placed(
            &has.the_program,
            &layout,
            &home,
            &bin,
            &installation,
            folder.0.join("the supervisor's complaints"),
        )?;
        program.the_supervisor_starts()?;
        let line = TheLine::to(Arc::clone(&far_end)).await.map_err(|why| format!("the line: {why}"))?;
        let the_row = Self::the_row_added(desk, &line, &far_end, &layout, &installation, label)?;
        Ok(TheDeskMachine {
            line,
            far_end,
            program,
            stand_in,
            layout,
            installation,
            the_row,
            the_blend,
            home,
            _deadline: deadline,
            _volume: volume,
            folder,
        })
    }

    /// **The server's row, as *Add* and a finished setup leave it**: the
    /// line's address, the user running the test, the client key's own
    /// fingerprint, the root the placement recorded and the installation
    /// read under it.
    fn the_row_added(
        desk: &TheDesk,
        line: &TheLine,
        far_end: &TheFarEnd,
        layout: &TheLayout,
        installation: &Identity,
        label: &str,
    ) -> Result<ServerRow, String> {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        use smoozy_servers::store::RecordedMachineRoot;
        let fingerprint = Command::new("ssh-keygen")
            .arg("-lf")
            .arg(far_end.client_key.with_extension("pub"))
            .output()
            .ok()
            .and_then(|said| {
                String::from_utf8_lossy(&said.stdout)
                    .split_whitespace()
                    .nth(1)
                    .map(str::to_string)
            })
            .ok_or("the client key's fingerprint would not read")?;
        let mut row = ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer).map_err(|_| "no randomness")?.as_text(),
            Host::what_the_user_typed("127.0.0.1").ok_or("the host is refused")?,
            Port::what_the_user_typed(line.address().port()).ok_or("the port is refused")?,
            Username::what_the_user_typed(&the_user()).ok_or("the user's name is refused")?,
            KeyReference {
                local_key_identifier: "the desk machine's key".into(),
                fingerprint,
            },
        );
        row.label = Some(label.to_string());
        row.recorded_machine_root = Some(RecordedMachineRoot {
            path: layout.root().as_path().to_string(),
            the_package_areas_own_location: None,
            read_under: installation.as_text(),
        });
        row.last_confirmed_installation_identity = Some(installation.as_text());
        desk.record.read(|c| {
            smoozy_desktop::record::server_rows::add(c, &row)
                .and_then(|()| smoozy_desktop::record::server_rows::write(c, &row))
                .map_err(|why| format!("the server's row: {why:?}"))
        })?;
        Ok(row)
    }

    // ------------------------------------------------------------ the desk side

    pub fn line(&self) -> &TheLine {
        &self.line
    }

    /// The server's row as it was added to the desk's record.
    pub fn the_row(&self) -> &ServerRow {
        &self.the_row
    }

    /// The machine's installation identity, which the row records.
    pub fn installation(&self) -> &Identity {
        &self.installation
    }

    pub fn layout(&self) -> &TheLayout {
        &self.layout
    }

    /// The package area, as the desktop names it to the point.
    pub fn package_area(&self) -> String {
        self.layout.area(Area::Package)
    }

    /// The `.blend` already on the machine, in its asset area.
    pub fn the_blend(&self) -> &Path {
        &self.the_blend
    }

    /// The machine's home folder, which its file-transfer sessions start in.
    pub fn home(&self) -> &Path {
        &self.home
    }

    pub fn the_stand_in(&self) -> &TheStandInBlender {
        &self.stand_in
    }

    /// **A real Blender installed on the machine, and the one a search of its
    /// disk finds** — for a scenario whose point is what the frames hold. Added
    /// 2026-10-25 by step 12's SC5, *The user adds a texture and renders
    /// again*.
    ///
    /// **The build is a two-line launcher in the machine's home** that starts
    /// `the_blender` — one of this Mac's own — with the factory-startup switch,
    /// because the Blenders at this desk are the Creator's and carry the
    /// Creator's settings and add-ons; a rental's Blender is launched without
    /// it (`Job files and outputs` R7), and only a rental proves that launch.
    /// The same wrapper step 10c's dependency wave piece 2 used
    /// (`a_real_blender_reads_a_real_file_through_the_real_program`). **It is
    /// named `blender`**, the name `Server Setup`'s search looks for, and the
    /// machine's `find` now names it instead of the stand-in. The stand-in is
    /// left where it is and nothing names it any more.
    pub fn a_real_blender_is_installed(&self, the_blender: &Path) -> PathBuf {
        let folder = self.home.join("blender-installed");
        std::fs::create_dir_all(&folder).expect("the Blender's folder on the machine");
        let at = folder.join("blender");
        a_program(
            &at,
            &format!("#!/bin/sh\nexec '{}' --factory-startup \"$@\"\n", the_blender.display()),
        );
        a_program(&self.home.join("bin").join("find"), &format!("#!/bin/sh\necho '{}'\n", at.display()));
        at
    }

    /// Where the desk connects.
    pub fn the_endpoint(&self) -> Endpoint {
        Endpoint {
            host: "127.0.0.1".to_string(),
            port: self.line.address().port(),
            username: the_user(),
        }
    }

    /// The desk's key, with no passphrase.
    pub fn the_credential(&self) -> Credential {
        Credential {
            private_key_file: self.far_end.client_key.clone(),
            passphrase: None,
        }
    }

    /// The machine's host key, as OpenSSH wrote its public half.
    pub fn the_host_public_key(&self) -> &Path {
        &self.far_end.host_public_key
    }

    /// **One connection through the line**, opened by the application's own
    /// transport with `figures`, or nothing where it does not open within
    /// `Servers` N2's twenty seconds.
    pub async fn a_connection(&self, figures: Figures) -> Option<Connection> {
        a_connection_to(self.the_endpoint(), self.the_credential(), figures).await
    }

    /// **The server's own point, lending one connection through the line**
    /// with the desk's figures, and an additional connection made the same
    /// way whenever the point asks for one (`Servers` S14).
    pub async fn a_point_lending(&self) -> Option<APointLending> {
        let mut connection = self.a_connection(THE_DESKS_FIGURES).await?;
        let sessions = ThisServersSessions::for_one_server();
        let (endpoint, key) = (self.the_endpoint(), self.far_end.client_key.clone());
        let additional: TheAdditionalConnection<Connection> = Arc::new(move || {
            let endpoint = endpoint.clone();
            let credential = Credential {
                private_key_file: key.clone(),
                passphrase: None,
            };
            Box::pin(async move { a_connection_to(endpoint, credential, THE_DESKS_FIGURES).await })
        });
        let lending = tokio::spawn({
            let sessions = sessions.clone();
            async move {
                let ending = lend_until_it_ends(&sessions, &mut connection, &additional).await;
                let _ = tokio::time::timeout(Duration::from_secs(5), connection.tear_down()).await;
                ending
            }
        });
        let began = Instant::now();
        while !sessions.a_connection_is_live() {
            if began.elapsed() > Duration::from_secs(20) || lending.is_finished() {
                lending.abort();
                return None;
            }
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
        Some(APointLending {
            sessions,
            lending: Some(lending),
        })
    }

    /// **The machine's supervisor, asked through a point** — the desktop's own
    /// seam, which plants the carriage link and runs the health exchange.
    pub fn the_supervisor_over(&self, point: &APointLending) -> OverThisServersPoint {
        OverThisServersPoint {
            sessions: point.sessions.clone(),
            package_area: self.package_area(),
        }
    }

    /// **What the Render Settings page hands the submit act for one job of
    /// this machine's `.blend` over `frames`**, rendered by the stand-in —
    /// and the stand-in's reading made to cover the same frames. The page's
    /// other choices are its defaults: no first-frame check, no download as
    /// it renders, every report on, nothing checked.
    pub fn what_the_page_hands_over(
        &self,
        desk: &TheDesk,
        frames: std::ops::RangeInclusive<i64>,
        label: &str,
    ) -> WhatThePageHandsOver {
        use smoozy_queue_history::record::{AnswerOrigin, DependencyCheckAtSubmission, Hue, TheServerAsItStood};
        use smoozy_render_settings::file_half::BlenderIdentity;
        use smoozy_render_settings::payload::{compose, Composition};
        use smoozy_render_settings::reporting::ReportingChoice;
        use smoozy_render_settings::store::SceneEntries;
        use smoozy_rendering::submission::{the_reporting_field, WhatTheDesktopHoldsAboutTheBuild};
        self.stand_in.prints_a_reading_of(frames.clone());
        let (major, minor, patch) = THE_STAND_INS_VERSION;
        let payload = compose(Composition {
            version_pick: &BlenderIdentity {
                path: self.stand_in.path().display().to_string(),
                version: smoozy_server_setup::blender::Version { major, minor, patch },
            },
            scene: None,
            answers: &SceneEntries {
                frame_range: Some(
                    smoozy_render_settings::frames::parse_the_frame_field(&format!(
                        "{}-{}",
                        frames.start(),
                        frames.end()
                    ))
                    .expect("a range"),
                ),
                ..Default::default()
            },
            standing: None,
            stale: false,
        });
        let issue = |kind| Identity::issue(kind).expect("randomness");
        WhatThePageHandsOver {
            server: TheServerAsItStood {
                local_server: Identity::read_back(IdentityKind::LocalServer, &self.the_row.local_server_identifier)
                    .expect("the row's own identity"),
                label: self.the_row.label.clone(),
                hue: Some(Hue(3)),
            },
            installation: Some(self.installation.clone()),
            layout: self.layout.clone(),
            package_area: self.package_area(),
            the_blend: self.the_blend.display().to_string(),
            label: Some(label.to_string()),
            reporting: the_reporting_field(None, ReportingChoice::Everything),
            the_version_origin: AnswerOrigin::ResolvedForThem,
            dependency_check: Some(DependencyCheckAtSubmission::NoStandingAtAll),
            what_the_desktop_holds_about_the_build: WhatTheDesktopHoldsAboutTheBuild::NothingButTheEntry,
            the_first_frame_check: false,
            the_download_as_it_renders_choice: false,
            where_the_downloads_go: None,
            the_formats_chosen: None,
            scene_jobs: vec![payload],
            the_reading_in_force: None,
            the_card_make: smoozy_job_files::manifest::TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
            run: issue(IdentityKind::Run),
            carries_on_from: None,
            the_temporary_area: Some(desk.temporary_area().to_path_buf()),
            the_servers_fallback_folder: None,
            the_driver_at_submission: Default::default(),
            the_typed_frames: None,
        }
    }

    // ------------------------------------------------------------ the two acts

    /// **The machine reboots**, abruptly: the line goes silent, every program
    /// on the machine is killed — the supervisor, every worker and the
    /// stand-in Blender among them — and after `down_for` the supervisor is
    /// started again from its package area, as boot registration starts it,
    /// and answers. **Then the line comes back**, ending every connection made
    /// before the reboot the way `old` says; a new connection reaches the
    /// machine that came back.
    ///
    /// **It wants a test runtime of more than one thread**, because the wait
    /// for the supervisor's answer is taken in place on it.
    pub async fn reboots(&self, down_for: Duration, old: HowTheOldConnectionsEnd) -> Result<(), String> {
        self.line.goes_silent();
        self.program.everything_stops();
        tokio::time::sleep(down_for).await;
        let program = &self.program;
        tokio::task::block_in_place(|| program.the_supervisor_starts())?;
        self.line.every_connection_ends(old);
        self.line.comes_back();
        Ok(())
    }

    /// **The machine is destroyed**: every program on it stops, and the line
    /// ends for good the way `how` says. Nothing brings it back.
    pub async fn is_destroyed(&self, how: HowTheMachineGoes) {
        if matches!(how, HowTheMachineGoes::WithARefusal) {
            self.line.stops_accepting().await;
        }
        self.program.everything_stops();
        self.line.is_gone(how);
    }

    /// Whether the supervisor this harness started is still running.
    pub fn the_supervisor_is_running(&self) -> bool {
        self.program.the_supervisor_is_running()
    }

    /// What the supervisor has said on its complaints stream, every start.
    pub fn the_supervisors_complaints(&self) -> String {
        String::from_utf8_lossy(&std::fs::read(&self.program.complaints).unwrap_or_default()).into_owned()
    }

    /// The throwaway folder everything of the machine's sits in.
    pub fn at(&self) -> &Path {
        &self.folder.0
    }
}

async fn a_connection_to(endpoint: Endpoint, credential: Credential, figures: Figures) -> Option<Connection> {
    tokio::time::timeout(
        Duration::from_secs(20),
        ssh::open(&endpoint, credential, figures, Box::new(Proceeds)),
    )
    .await
    .ok()?
    .ok()
}

/// **The server's own point, lending one connection** until it ends, then
/// tearing it down as `Servers`' driving does. Nothing lends a second: a
/// scenario that plays the repair drives `Servers`' own period instead.
pub struct APointLending {
    pub sessions: ThisServersSessions,
    lending: Option<tokio::task::JoinHandle<Ending>>,
}

impl APointLending {
    /// Whether the connection under the point has ended.
    pub fn has_ended(&self) -> bool {
        self.lending.as_ref().is_none_or(|lending| lending.is_finished())
    }

    /// **How the connection ended**, waited for no longer than `within`;
    /// nothing where it is still lent.
    pub async fn its_ending(&mut self, within: Duration) -> Option<Ending> {
        let lending = self.lending.as_mut()?;
        let ended = tokio::time::timeout(within, lending).await.ok()?.ok();
        self.lending = None;
        ended
    }
}

impl Drop for APointLending {
    fn drop(&mut self) {
        if let Some(lending) = self.lending.take() {
            lending.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The stand-in's frame is a PNG of one pixel**: the signature, the
    /// three chunks in their order, and the check is CRC-32 — the catalogue's
    /// own check value for the nine digits.
    #[test]
    fn the_stand_ins_frame_is_a_png_of_one_pixel() {
        assert_eq!(crc_32(b"123456789"), 0xCBF4_3926);
        let png = a_one_pixel_png();
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
        assert_eq!(&png[12..16], b"IHDR");
        assert_eq!(&png[16..24], &[0, 0, 0, 1, 0, 0, 0, 1], "one pixel by one");
        assert_eq!(&png[37..41], b"IDAT");
        assert_eq!(&png[png.len() - 8..png.len() - 4], b"IEND");
    }

    /// **The stand-in renders on its own clock, waits at a held frame, and
    /// goes on when it is let go** — what a scenario holding a render part
    /// way leans on. Run straight, against an inventory in the shape the
    /// render worker writes, because at this desk the machine's program
    /// undoes every render it starts (see the head of this file).
    #[test]
    fn the_stand_in_renders_on_its_clock_and_waits_at_a_held_frame() {
        let Some(perl) = the_first_of(&["/usr/bin/perl", "/usr/local/bin/perl"]) else {
            println!("SKIPPED: no Perl on this system for the stand-in Blender.");
            return;
        };
        let folder = TheThrowawayFolder::made("stand-in", "clock").expect("a throwaway folder");
        let stand_in = TheStandInBlender::made(folder.folder("stand-in"), &perl);
        stand_in.takes_this_long_a_frame(Duration::from_millis(50));
        let record = folder.folder("job-record");
        let frames: Vec<PathBuf> = (1..=3).map(|frame| folder.0.join(format!("out/f_{frame:04}.png"))).collect();
        // **Only the inventory's entries are read**, each a frame and its
        // predicted path, the path written as its bytes.
        let entries: Vec<serde_json::Value> = frames
            .iter()
            .zip(1..)
            .map(|(path, frame)| serde_json::json!({"frame": frame, "predicted_path": path.display().to_string().into_bytes()}))
            .collect();
        std::fs::write(
            record.join(smoozy_job_files::manifest::the_recorded_readings_file_name(1)),
            serde_json::json!({"inventory": {"It": {"entries": entries}}}).to_string(),
        )
        .expect("the recorded reading");
        stand_in.holds_before_frame(2);
        let render = Command::new(stand_in.path())
            .arg("--python")
            .arg(record.join("launch-material-1.py"))
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("the stand-in starts");
        let until = |seen: &dyn Fn() -> bool, said: &str| {
            let began = Instant::now();
            while !seen() {
                assert!(began.elapsed() < Duration::from_secs(10), "{said}: not seen within ten seconds");
                std::thread::sleep(Duration::from_millis(20));
            }
        };
        until(&|| stand_in.frames_written() == vec![1], "frame 1 written");
        std::thread::sleep(Duration::from_millis(300));
        assert_eq!(stand_in.frames_written(), vec![1], "it waits at the held frame");
        assert!(!frames[1].exists(), "frame 2 is not written while it is held");
        stand_in.lets_frame_go(2);
        let render = std::sync::Mutex::new(render);
        until(
            &|| matches!(render.lock().expect("the render").try_wait(), Ok(Some(_))),
            "the render ends once let go",
        );
        assert!(render.lock().expect("the render").wait().expect("it ended").success());
        assert_eq!(stand_in.frames_written(), vec![1, 2, 3]);
        for frame in &frames {
            assert!(std::fs::read(frame).is_ok_and(|bytes| bytes == a_one_pixel_png()), "{}", frame.display());
        }
    }
}

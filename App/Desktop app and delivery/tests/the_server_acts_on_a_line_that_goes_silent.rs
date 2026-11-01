//! **The server acts the dead-line sweep's desk coverage names, each run over a
//! real connection whose far end goes silent part-way, without closing
//! anything.** Written 2026-10-19 out of the Creator's ask: *"I think it's
//! prudent to test abrupt connection drops with other operations, not just
//! the one you accidentally caught."* The list is
//! `Audits/Build/Step 10c/A dead line under every server act — the sweep.md`,
//! *Desk coverage for the remaining acts*, second half: the whole-file write
//! at size, submission, Queue's acts, History's erase, setup's acts, and the
//! live reads.
//!
//! **What the far end is.** OpenSSH's own `sshd` in its one-connection mode,
//! as `tests/a_line_cut_silently_under_a_real_connection.rs` runs it, every
//! file-transfer session OpenSSH's own `sftp-server`, and the same kind of
//! relay between it and the desk: a line that can stop carrying bytes either
//! way, or only the machine's way back, and close nothing. **Where an act asks
//! the machine's supervisor, the supervisor is the machine's own program**,
//! the one this workspace built, started in the far end's package area and
//! reached through its real adapter over the line, as on a rental (the shape
//! `tests/a_job_is_submitted_through_the_servers_own_point.rs` stands it up
//! in, carried here over SSH). **Its line is held** by the setup-in-progress
//! state, so every job it takes waits and none starts a Blender this desk does
//! not have. **One stand-in, named where it is made**: a `stat` that answers
//! in GNU's fields where this system's own speaks BSD's.
//!
//! **What every test checks** is the sweep's four things: the act ends within
//! its own stated bound; what the user is shown says so plainly, and so do the
//! durable records; nothing is lost, and nothing is taken as whole that is
//! not; and the act works again once the line is back.
//!
//! **The figures.** The keepalive here is one second and three unanswered
//! (a three-to-four-second declaration window), not a rental's fifteen and
//! three (45 to 60 seconds); the mechanism is what is proved and the figures
//! are handed in exactly as a rental's are.
//!
//! **Nothing of the Creator's is touched.** The server's sessions start in a
//! throwaway home and its commands run with that home as theirs, the keys, the
//! record and every file are inside throwaway folders removed with the test,
//! and every server and supervisor is a child of the test that ends with it.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! **Every marker of another document is written out with that document at its
//! own site and this file keeps no header list.**

#[cfg(not(unix))]
#[test]
fn the_far_end_here_wants_openssh_and_a_unix_socket() {
    println!(
        "SKIPPED: the far end here is OpenSSH's own sshd and a Unix socket, which this \
         system ({}) has none of.",
        std::env::consts::OS
    );
}

#[cfg(unix)]
mod on_a_desk_with_openssh {
    use std::net::SocketAddr;
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::io::{AsRawFd, FromRawFd};
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::sync::watch;

    use smoozy_desktop::servers::sessions::{lend_until_it_ends, TheAdditionalConnection, ThisServersSessions};
    use smoozy_job_files::layout::{Area, MachineRoot, TheLayout, WhereThePackageAreaSits};
    use smoozy_server_protocol::transport::ssh::{self, Connection};
    use smoozy_server_protocol::transport::{Credential, DecideHostTrust, Ending, Endpoint, Figures, HostTrust};

    // ═══════════════════════════════════════════════════════════════════════
    // The far end
    // ═══════════════════════════════════════════════════════════════════════

    /// Where OpenSSH's own server and file-transfer server are.
    fn openssh() -> Option<(PathBuf, PathBuf)> {
        let sshd = ["/usr/sbin/sshd", "/usr/local/sbin/sshd"]
            .iter()
            .map(PathBuf::from)
            .find(|path| path.exists())?;
        let sftp = [
            "/usr/libexec/sftp-server",
            "/usr/lib/openssh/sftp-server",
            "/usr/libexec/openssh/sftp-server",
            "/usr/lib/ssh/sftp-server",
        ]
        .iter()
        .map(PathBuf::from)
        .find(|path| path.exists())?;
        Some((sshd, sftp))
    }

    /// A throwaway folder that takes itself away.
    pub(super) struct Scratch(pub PathBuf);

    impl Scratch {
        pub fn new(named: &str) -> Scratch {
            let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
                "smoozy-acts-{named}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|since| since.as_nanos())
                    .unwrap_or(0)
            ));
            std::fs::create_dir_all(&path).expect("the scratch folder");
            Scratch(path)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A file written and made runnable.
    fn a_program(at: &Path, text: &str) {
        std::fs::write(at, text).expect("a program is written");
        std::fs::set_permissions(at, std::fs::Permissions::from_mode(0o755)).expect("it runs");
    }

    /// **The machine the test stands up**: OpenSSH's own server, ready to take
    /// one connection at a time, its sessions and commands in a throwaway
    /// home, and the machine root inside that home with its five areas.
    pub(super) struct TheFarEnd {
        sshd: PathBuf,
        config: PathBuf,
        client_key: PathBuf,
        pub home: PathBuf,
        pub layout: TheLayout,
        pub the_blend: PathBuf,
        _scratch: Scratch,
    }

    impl TheFarEnd {
        /// **Nothing where this system has no OpenSSH this test can run.**
        pub fn made(named: &str) -> Option<TheFarEnd> {
            let (sshd, sftp) = openssh()?;
            let scratch = Scratch::new(named);
            let folder = scratch.0.clone();
            for key in ["host", "client"] {
                let made = std::process::Command::new("ssh-keygen")
                    .args(["-q", "-t", "ed25519", "-N", "", "-C", key, "-f"])
                    .arg(folder.join(key))
                    .status()
                    .ok()?;
                if !made.success() {
                    return None;
                }
            }
            std::fs::copy(folder.join("client.pub"), folder.join("authorized_keys")).ok()?;
            let home = folder.join("home");
            let bin = home.join("bin");
            std::fs::create_dir_all(&bin).ok()?;
            let layout = TheLayout::of(
                MachineRoot::chosen(&home.join("root").display().to_string()).expect("a root"),
                WhereThePackageAreaSits::UnderTheRoot,
            );
            for area in smoozy_job_files::layout::EVERY_AREA {
                std::fs::create_dir_all(layout.area(area)).ok()?;
            }
            let the_blend = PathBuf::from(layout.area(Area::Asset)).join("kitchen.blend");
            std::fs::write(&the_blend, b"BLENDER-v405 standing in for a scene").ok()?;
            // **A stand-in: `stat` in GNU's fields** (`Render Settings` Q14
            // item 2's reading), which a rental's Linux answers and this
            // system's own does not.
            a_program(
                &bin.join("stat"),
                "#!/bin/sh\n# Stands in for GNU stat's `-L -c '%s %i %Y %Z' FILE`.\n\
                 if /usr/bin/stat --version >/dev/null 2>&1; then exec /usr/bin/stat \"$@\"; fi\n\
                 exec /usr/bin/stat -L -f '%z %i %m %c' \"$4\"\n",
            );
            let config = folder.join("sshd_config");
            std::fs::write(
                &config,
                format!(
                    "HostKey \"{}\"\nAuthorizedKeysFile \"{}\"\nPidFile none\nUsePAM no\nStrictModes no\n\
                     PasswordAuthentication no\nKbdInteractiveAuthentication no\n\
                     Subsystem sftp {} -d \"{}\"\nSetEnv \"HOME={}\" \"PATH={}:/usr/bin:/bin\"\n",
                    folder.join("host").display(),
                    folder.join("authorized_keys").display(),
                    sftp.display(),
                    home.display(),
                    home.display(),
                    bin.display(),
                ),
            )
            .ok()?;
            let checked = std::process::Command::new(&sshd)
                .args(["-t", "-f"])
                .arg(&config)
                .output()
                .ok()?;
            if !checked.status.success() {
                println!(
                    "sshd will not run here: {}",
                    String::from_utf8_lossy(&checked.stderr).trim()
                );
                return None;
            }
            Some(TheFarEnd {
                sshd,
                config,
                client_key: folder.join("client"),
                home,
                layout,
                the_blend,
                _scratch: scratch,
            })
        }

        /// One connection's server: `sshd -i` on one end of a loopback pair
        /// (macOS's own `sshd` refuses a socket with no address).
        fn one_connection(&self) -> (std::process::Child, tokio::net::TcpStream) {
            let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("a listener");
            let ours = std::net::TcpStream::connect(listener.local_addr().expect("an address"))
                .expect("the pair connects");
            let (theirs, _) = listener.accept().expect("the pair is accepted");
            ours.set_nonblocking(true).expect("the end we drive");
            let reading = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let writing = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let child = std::process::Command::new(&self.sshd)
                .args(["-i", "-e", "-f"])
                .arg(&self.config)
                .stdin(reading)
                .stdout(writing)
                .stderr(
                    std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(self.config.with_file_name("sshd.log"))
                        .map(std::process::Stdio::from)
                        .unwrap_or_else(|_| std::process::Stdio::null()),
                )
                .spawn()
                .expect("OpenSSH's own server starts");
            drop(theirs);
            (child, tokio::net::TcpStream::from_std(ours).expect("the end we drive"))
        }

        /// What the server wrote about itself, for a failure to show.
        pub fn its_log(&self) -> String {
            std::fs::read_to_string(self.config.with_file_name("sshd.log")).unwrap_or_default()
        }
    }

    fn copy_of(descriptor: std::os::unix::io::RawFd) -> std::os::unix::io::RawFd {
        let copy = unsafe { libc::dup(descriptor) };
        assert!(copy >= 0, "the descriptor could not be copied");
        copy
    }

    // ═══════════════════════════════════════════════════════════════════════
    // The line
    // ═══════════════════════════════════════════════════════════════════════

    /// **The line between the desk and the machine.** Up, it carries every
    /// byte both ways. **Silent, it carries nothing and closes nothing**, which
    /// is what a connection left behind by a network change looks like from
    /// the desk. **Its two halves go silent separately where a test asks**: the
    /// machine's half alone is a request that arrives and an answer that never
    /// comes back. **It counts what it carried towards the machine**, and can
    /// go silent of itself once a given count has passed.
    pub(super) struct TheLine {
        address: SocketAddr,
        towards_the_machine: watch::Sender<bool>,
        towards_the_desk: watch::Sender<bool>,
        carried_towards_the_machine: Arc<AtomicU64>,
        silent_past: Arc<AtomicU64>,
        carried_towards_the_desk: Arc<AtomicU64>,
        silent_past_on_the_way_back: Arc<AtomicU64>,
        servers: Arc<Mutex<Vec<std::process::Child>>>,
        accepting: Option<tokio::task::JoinHandle<()>>,
    }

    impl Drop for TheLine {
        fn drop(&mut self) {
            if let Some(accepting) = self.accepting.take() {
                accepting.abort();
            }
            if let Ok(mut servers) = self.servers.lock() {
                for server in servers.iter_mut() {
                    let _ = server.kill();
                    let _ = server.wait();
                }
            }
        }
    }

    impl TheLine {
        pub async fn to(far_end: &Arc<TheFarEnd>) -> TheLine {
            let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
                .await
                .expect("the loopback address takes a listener");
            let address = listener.local_addr().expect("a bound listener has an address");
            let (towards_the_machine, _) = watch::channel(true);
            let (towards_the_desk, _) = watch::channel(true);
            let line = TheLine {
                address,
                towards_the_machine,
                towards_the_desk,
                carried_towards_the_machine: Arc::new(AtomicU64::new(0)),
                silent_past: Arc::new(AtomicU64::new(u64::MAX)),
                carried_towards_the_desk: Arc::new(AtomicU64::new(0)),
                silent_past_on_the_way_back: Arc::new(AtomicU64::new(u64::MAX)),
                servers: Arc::new(Mutex::new(Vec::new())),
                accepting: None,
            };
            let mut line = line;
            let far_end = Arc::clone(far_end);
            let (towards_the_machine, towards_the_desk) =
                (line.towards_the_machine.clone(), line.towards_the_desk.clone());
            let (carried, silent_past) = (
                Arc::clone(&line.carried_towards_the_machine),
                Arc::clone(&line.silent_past),
            );
            let (carried_back, silent_past_back) = (
                Arc::clone(&line.carried_towards_the_desk),
                Arc::clone(&line.silent_past_on_the_way_back),
            );
            let servers = Arc::clone(&line.servers);
            line.accepting = Some(tokio::spawn(async move {
                while let Ok((desk, _)) = listener.accept().await {
                    let (server, machine) = far_end.one_connection();
                    if let Ok(mut held) = servers.lock() {
                        held.push(server);
                    }
                    let (desk_reads, desk_writes) = tokio::io::split(desk);
                    let (machine_reads, machine_writes) = machine.into_split();
                    tokio::spawn(one_direction(
                        desk_reads,
                        machine_writes,
                        towards_the_machine.subscribe(),
                        Some(TheCount {
                            carried: Arc::clone(&carried),
                            silent_past: Arc::clone(&silent_past),
                            silences: vec![towards_the_machine.clone(), towards_the_desk.clone()],
                        }),
                    ));
                    tokio::spawn(one_direction(
                        machine_reads,
                        desk_writes,
                        towards_the_desk.subscribe(),
                        Some(TheCount {
                            carried: Arc::clone(&carried_back),
                            silent_past: Arc::clone(&silent_past_back),
                            silences: vec![towards_the_machine.clone(), towards_the_desk.clone()],
                        }),
                    ));
                }
            }));
            line
        }

        /// **Silent at once, both ways** — callable from inside an act.
        pub fn goes_silent(&self) {
            self.towards_the_machine.send_replace(false);
            self.towards_the_desk.send_replace(false);
        }

        /// **The machine's half alone**: what the desk sends still arrives,
        /// and nothing the machine answers comes back.
        pub fn the_answers_stop(&self) {
            self.towards_the_desk.send_replace(false);
        }

        pub fn comes_back(&self) {
            self.silent_past.store(u64::MAX, Ordering::SeqCst);
            self.silent_past_on_the_way_back.store(u64::MAX, Ordering::SeqCst);
            self.towards_the_machine.send_replace(true);
            self.towards_the_desk.send_replace(true);
        }

        /// **Silent both ways once `more` bytes past what it has carried so
        /// far have gone towards the machine.**
        pub fn goes_silent_after(&self, more: u64) {
            let now = self.carried_towards_the_machine.load(Ordering::SeqCst);
            self.silent_past.store(now.saturating_add(more), Ordering::SeqCst);
        }

        /// **Silent both ways once `more` bytes past what it has carried so
        /// far have come back towards the desk** — a download's cut.
        pub fn goes_silent_after_bringing(&self, more: u64) {
            let now = self.carried_towards_the_desk.load(Ordering::SeqCst);
            self.silent_past_on_the_way_back.store(now.saturating_add(more), Ordering::SeqCst);
        }

        pub fn is_up(&self) -> bool {
            *self.towards_the_machine.borrow() && *self.towards_the_desk.borrow()
        }
    }

    /// The desk-to-machine half's count, and the silence it can bring.
    struct TheCount {
        carried: Arc<AtomicU64>,
        silent_past: Arc<AtomicU64>,
        silences: Vec<watch::Sender<bool>>,
    }

    async fn until_the_line_is(up: &mut watch::Receiver<bool>, wanted: bool) {
        loop {
            if *up.borrow_and_update() == wanted {
                return;
            }
            if up.changed().await.is_err() {
                std::future::pending::<()>().await;
            }
        }
    }

    /// One direction of one connection, carried while its half is up.
    async fn one_direction<R, W>(mut from: R, mut to: W, mut up: watch::Receiver<bool>, count: Option<TheCount>)
    where
        R: tokio::io::AsyncRead + Unpin,
        W: tokio::io::AsyncWrite + Unpin,
    {
        let mut buffer = vec![0u8; 16 * 1024];
        loop {
            until_the_line_is(&mut up, true).await;
            let read = tokio::select! {
                read = from.read(&mut buffer) => read,
                () = until_the_line_is(&mut up, false) => continue,
            };
            let got = match read {
                Ok(0) | Err(_) => break,
                Ok(got) => got,
            };
            until_the_line_is(&mut up, true).await;
            if to.write_all(&buffer[..got]).await.is_err() {
                break;
            }
            if let Some(count) = &count {
                let carried = count.carried.fetch_add(got as u64, Ordering::SeqCst) + got as u64;
                if carried >= count.silent_past.load(Ordering::SeqCst) {
                    for silence in &count.silences {
                        silence.send_replace(false);
                    }
                }
            }
        }
        let _ = to.shutdown().await;
    }

    // ═══════════════════════════════════════════════════════════════════════
    // The desk side
    // ═══════════════════════════════════════════════════════════════════════

    struct Proceeds;
    impl DecideHostTrust for Proceeds {
        fn decide(&mut self, _offered: &[u8]) -> HostTrust {
            HostTrust::Proceed
        }
    }

    /// **The desk's figures**: `Servers` K3's mechanism at a one-second
    /// interval and three unanswered, a three-to-four-second window.
    pub(super) const AT_THE_DESK: Figures = Figures {
        interval: Duration::from_secs(1),
        unanswered_before_the_break: 3,
    };

    /// **How far past the window's close an act may end and still have ended
    /// with it**: the desk noticing the break, the one step under way failing,
    /// and the act writing its ending.
    pub(super) const ONE_STEP_AFTER_THE_BREAK: Duration = Duration::from_secs(5);

    pub(super) fn the_window_closes() -> Duration {
        AT_THE_DESK.the_declaration_window().1
    }

    async fn a_connection_through(line: &TheLine, far_end: &TheFarEnd) -> Connection {
        let user = std::env::var("USER")
            .or_else(|_| std::env::var("LOGNAME"))
            .expect("the user running the test is named");
        ssh::open(
            &Endpoint {
                host: "127.0.0.1".to_string(),
                port: line.address.port(),
                username: user,
            },
            Credential {
                private_key_file: far_end.client_key.clone(),
                passphrase: None,
            },
            AT_THE_DESK,
            Box::new(Proceeds),
        )
        .await
        .unwrap_or_else(|why| panic!("the connection opens through the line: {why:?}; {}", far_end.its_log()))
    }

    /// **The point, lending one real connection**, with no additional one —
    /// and nothing repairs it when it breaks, so the act meets the break the
    /// way it does between a break and `Servers`' repair.
    pub(super) struct ThePoint {
        pub sessions: ThisServersSessions,
        lending: Option<tokio::task::JoinHandle<(Ending, Instant)>>,
    }

    impl ThePoint {
        pub async fn over(line: &TheLine, far_end: &TheFarEnd) -> ThePoint {
            let mut connection = a_connection_through(line, far_end).await;
            let sessions = ThisServersSessions::for_one_server();
            let lending = tokio::spawn({
                let sessions = sessions.clone();
                async move {
                    let none: TheAdditionalConnection<Connection> = Arc::new(|| Box::pin(async { None }));
                    let ending = lend_until_it_ends(&sessions, &mut connection, &none).await;
                    (ending, Instant::now())
                }
            });
            while !sessions.a_connection_is_live() {
                tokio::task::yield_now().await;
            }
            ThePoint {
                sessions,
                lending: Some(lending),
            }
        }

        /// **The lending's own answer: how the connection ended and when** —
        /// the moment a server row stops reading *connected* (`Servers` K6).
        pub async fn its_ending(&mut self, within: Duration) -> (Ending, Instant) {
            let lending = self.lending.take().expect("the lending is held");
            tokio::time::timeout(within, lending)
                .await
                .expect("the lending answered")
                .expect("the lending ran")
        }
    }

    impl Drop for ThePoint {
        fn drop(&mut self) {
            if let Some(lending) = self.lending.take() {
                lending.abort();
            }
        }
    }

    /// **Says the far end cannot be stood up here and stops the test.**
    macro_rules! the_far_end_or_say_so {
        ($named:expr) => {
            match TheFarEnd::made($named) {
                Some(far_end) => Arc::new(far_end),
                None => {
                    println!(
                        "SKIPPED: no OpenSSH sshd, sftp-server and ssh-keygen this test can run on \
                         this system ({}).",
                        std::env::consts::OS
                    );
                    return;
                }
            }
        };
    }

    // ═══════════════════════════════════════════════════════════════════════
    // The whole-file write at size (K1's finding 1 in the dead-line account)
    // ═══════════════════════════════════════════════════════════════════════

    /// A pattern a short or shifted file cannot pass for.
    fn the_pattern(size: usize) -> Vec<u8> {
        (0..size).map(|at| (at as u64).wrapping_mul(2_654_435_761).to_le_bytes()[3]).collect()
    }

    /// **The material's write**, `Job files and outputs`' durable whole-file
    /// write, on one channel lent by the point — the call a submission's
    /// manifest, job record and confirming write make.
    async fn the_whole_file_is_written(
        sessions: &ThisServersSessions,
        path: &Path,
        bytes: Vec<u8>,
    ) -> Result<(), String> {
        use smoozy_job_files::acting::TheMachinesDirectories;
        let channel = sessions
            .a_transfer_channel()
            .await
            .map_err(|why| format!("no channel: {why:?}"))?;
        let mut subsystem = smoozy_server_protocol::subsystem::client::Subsystem::negotiate(channel)
            .await
            .map_err(|why| format!("no session: {why:?}"))?;
        subsystem
            .write_a_whole_file_durably(&path.to_string_lossy(), &bytes)
            .await
            .map_err(|why| format!("{why:?}"))
    }

    /// **A whole-file write of 1, 16 and 64 MB on a healthy line lands whole,
    /// and the connection under it stands.** The dead-line repairs' K1 finding
    /// 1: the write handed every piece over before it read one answer, and
    /// above six to nine megabytes the answers piled up unread until the
    /// connection stopped moving and its keepalive called a healthy line
    /// broken — at this desk 9, 12 and 16 MB all failed at four seconds.
    ///
    /// **What it bites on.** Before the repair the 16 and 64 MB writes failed
    /// with the connection declared gone; now the answers are collected as the
    /// byte loop collects them.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_whole_file_write_of_1_16_and_64_mb_lands_whole_on_a_healthy_line() {
        let far_end = the_far_end_or_say_so!("whole-write");
        let line = TheLine::to(&far_end).await;
        let point = ThePoint::over(&line, &far_end).await;
        let mut missed = Vec::new();
        for megabytes in [1usize, 16, 64] {
            let bytes = the_pattern(megabytes * 1024 * 1024);
            let at = far_end.home.join(format!("whole-{megabytes}.bin"));
            let began = Instant::now();
            let wrote = tokio::time::timeout(
                Duration::from_secs(120),
                the_whole_file_is_written(&point.sessions, &at, bytes.clone()),
            )
            .await;
            let took = began.elapsed();
            println!("{megabytes} MB: {wrote:?} in {took:?}");
            match wrote {
                Ok(Ok(())) => {
                    if std::fs::read(&at).ok().as_deref() != Some(bytes.as_slice()) {
                        missed.push(format!("{megabytes} MB: said written and did not land whole"));
                    }
                }
                other => missed.push(format!("{megabytes} MB: came to {other:?} after {took:?}")),
            }
            if !point.sessions.a_connection_is_live() {
                missed.push(format!("{megabytes} MB: the connection under the write was declared gone"));
                break;
            }
        }
        assert!(missed.is_empty(), "{missed:#?}");
    }

    /// **A whole-file write the line cuts part-way ends at the break, says it
    /// did not land, and leaves no name that reads as whole; once the line is
    /// back the same write on a fresh connection lands whole.** The write's
    /// own bound is the connection's break detection (`Server protocol and
    /// transfers` C18); its create is exclusive, so a cut write's name is left
    /// for the caller's own staging to sweep, never taken as the file.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_whole_file_write_cut_part_way_ends_at_the_break_and_lands_whole_once_the_line_is_back() {
        let far_end = the_far_end_or_say_so!("whole-write-cut");
        let line = TheLine::to(&far_end).await;
        let mut point = ThePoint::over(&line, &far_end).await;
        let bytes = the_pattern(32 * 1024 * 1024);
        let at = far_end.home.join("cut.bin");
        line.goes_silent_after(8 * 1024 * 1024);
        let began = Instant::now();
        let wrote = tokio::time::timeout(
            Duration::from_secs(60),
            the_whole_file_is_written(&point.sessions, &at, bytes.clone()),
        )
        .await
        .expect("the cut write waited past a minute");
        let took = began.elapsed();
        println!("the cut write came to {wrote:?} in {took:?}");
        assert!(!line.is_up(), "the line never went silent under the write");
        assert!(wrote.is_err(), "a write the line cut said it was written");
        assert!(
            took <= the_window_closes() + ONE_STEP_AFTER_THE_BREAK + Duration::from_secs(2),
            "the cut write took {took:?}"
        );
        let (ending, _) = point.its_ending(Duration::from_secs(10)).await;
        assert!(ending.is_a_break(), "the lending answered {ending:?}");
        let landed = std::fs::read(&at).unwrap_or_default();
        assert!(landed.len() < bytes.len(), "the cut write's file holds every byte");

        line.comes_back();
        let point = ThePoint::over(&line, &far_end).await;
        let again = far_end.home.join("again.bin");
        let wrote = tokio::time::timeout(
            Duration::from_secs(60),
            the_whole_file_is_written(&point.sessions, &again, bytes.clone()),
        )
        .await
        .expect("the write once the line was back waited past a minute");
        assert_eq!(wrote, Ok(()));
        assert!(std::fs::read(&again).ok().as_deref() == Some(bytes.as_slice()), "it did not land whole");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // The machine's own program, behind the line
    // ═══════════════════════════════════════════════════════════════════════

    use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
    use smoozy_server_protocol::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
    use smoozy_state::identities::{Identity, IdentityKind};

    /// **The machine's own program**, the one this workspace built, standing
    /// in the far end's package area: the active-package pointer naming it,
    /// its records, and a supervisor started there. **Every ask the desk makes
    /// over the line reaches it through its own adapter**, run by OpenSSH's
    /// server through the link the ask plants. **Its line is held**, so every
    /// job it takes waits (`Rented-server runtime` P1).
    pub(super) struct TheRealProgram {
        area: PathBuf,
        endpoint: PathBuf,
        pub installation: Identity,
        /// Where a test arms the hold on the next answer.
        flag: PathBuf,
        /// The far end's home, every program of the machine's started under it.
        home: PathBuf,
        supervisor: Mutex<Option<std::process::Child>>,
    }

    /// How long one asking of the program straight, with no line between, is
    /// given — a test's own bound; an answer takes well under a second.
    const HOW_LONG_ONE_ASKING_IS_GIVEN: Duration = Duration::from_secs(60);

    impl TheRealProgram {
        /// **The program this workspace built**, beside this test's own
        /// executable — present wherever the machine program's own tests were
        /// built. Nothing where it is not, and the test says so and stops.
        fn the_one_built() -> Option<PathBuf> {
            let here = std::env::current_exe().ok()?;
            let built = here
                .parent()?
                .parent()?
                .join(smoozy_runtime::the_program::THE_FIXED_ENTRY_POINT_NAME);
            built.is_file().then_some(built)
        }

        pub fn started_at(far_end: &TheFarEnd) -> Option<TheRealProgram> {
            TheRealProgram::started_at_holding(far_end, true)
        }

        /// **Started with its line held or not** — a setup's own run sets
        /// and clears the hold itself, so a setup's test starts it unheld.
        pub fn started_at_holding(far_end: &TheFarEnd, hold: bool) -> Option<TheRealProgram> {
            use smoozy_runtime::the_program::{
                records, the_active_package_pointer_at, the_record_at, THE_FIXED_ENTRY_POINT_NAME,
                THE_SUPERVISOR_WORD,
            };
            let built = TheRealProgram::the_one_built()?;
            let layout = &far_end.layout;
            let area = PathBuf::from(layout.area(Area::Package));
            let area_text = area.display().to_string();
            // **The active-package pointer is the program, run as itself** —
            // the name it was invoked by handed on, which is how the program
            // knows its package area. **One thing is added, and it is a
            // test's**: where a test has armed it, the adapter's answer is
            // held back a few seconds once the program has given it, and the
            // test is told — so a test can stop the machine's half of the line
            // after the machine acted and before its answer left. The
            // supervisor and every other invocation are the program alone.
            let perl = ["/usr/bin/perl", "/usr/local/bin/perl"].iter().map(PathBuf::from).find(|path| path.exists())?;
            let flag = far_end.home.join(".smoozy-test-holds-the-next-answer");
            a_program(
                Path::new(&the_active_package_pointer_at(&area_text)),
                &format!(
                    r#"#!{perl}
use strict;
my $real = '{built}';
my $flag = '{flag}';
if (@ARGV) {{ exec {{ $real }} $0, @ARGV; exit 127; }}
binmode STDIN;
my $in = do {{ local $/; <STDIN> }};
$in = '' unless defined $in;
pipe(my $from, my $to) or exit 126;
pipe(my $their_in, my $our_in) or exit 126;
my $child = fork();
exit 126 unless defined $child;
if ($child == 0) {{
    close $from; close $our_in;
    open(STDIN, '<&', $their_in) or exit 126;
    open(STDOUT, '>&', $to) or exit 126;
    exec {{ $real }} $0; exit 127;
}}
close $to; close $their_in;
binmode $our_in;
print $our_in $in;
close $our_in;
binmode $from;
my $out = do {{ local $/; <$from> }};
$out = '' unless defined $out;
waitpid($child, 0);
my $status = $? >> 8;
if (-e $flag) {{
    open(my $wanted, '<', $flag);
    my $word = do {{ local $/; <$wanted> }};
    close $wanted;
    $word = '' unless defined $word;
    if ($word eq '' || index($in, $word) >= 0) {{
        unlink $flag; open(my $told, '>', "$flag.answered"); close $told; sleep 5;
    }}
}}
binmode STDOUT;
print STDOUT $out;
exit $status;
"#,
                    perl = perl.display(),
                    built = built.display(),
                    flag = flag.display(),
                ),
            );
            // **A short endpoint path**: a socket's name has a length limit.
            let endpoint = std::env::temp_dir().join(format!(
                "smzy-{}.sock",
                &Identity::issue(IdentityKind::Operation).ok()?.as_text()[..12]
            ));
            std::fs::write(
                the_record_at(&area_text, records::THE_ENDPOINTS_RECORDED_PATH),
                endpoint.display().to_string(),
            )
            .ok()?;
            let installation = Identity::issue(IdentityKind::Installation).ok()?;
            std::fs::write(
                the_record_at(&area_text, records::THE_INSTALLATION_IDENTITY),
                installation.as_text(),
            )
            .ok()?;
            let phase = smoozy_runtime::package::phase::a_fresh_attempts_first_write(
                Identity::issue(IdentityKind::Operation).ok()?,
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
            std::fs::write(
                the_record_at(&area_text, records::THE_PHASE_RECORD),
                smoozy_runtime::written_form::write(&phase).ok()?,
            )
            .ok()?;
            std::fs::write(
                the_record_at(&area_text, smoozy_job_files::layout::THE_RECORD_OF_THE_ROOT),
                layout.root().as_path(),
            )
            .ok()?;
            let supervisor = std::process::Command::new(the_active_package_pointer_at(&area_text))
                .arg(THE_SUPERVISOR_WORD)
                .env("HOME", &far_end.home)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .ok()?;
            let program = TheRealProgram {
                area,
                endpoint,
                installation,
                flag,
                home: far_end.home.clone(),
                supervisor: Mutex::new(Some(supervisor)),
            };
            let health = Request::speaking(
                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                Identity::issue(IdentityKind::Operation).ok()?,
                smoozy_server_protocol::requests::THE_HEALTH_ANSWER_REQUEST,
            );
            let answered = smoozy_runtime::acting::endpoint::TheEndpointAsACaller::at(
                &program.endpoint.display().to_string(),
                Duration::from_secs(1),
            )
            .ask_waiting_for_a_supervisor(&health, HOW_LONG_ONE_ASKING_IS_GIVEN)
            .ok()?;
            assert!(answered.is_some(), "the machine's own supervisor never answered");
            if !hold {
                return Some(program);
            }
            // **The line held**, so every job written waits.
            let held = program.ask(
                &Request::speaking(
                    THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                    Identity::issue(IdentityKind::Operation).ok()?,
                    smoozy_server_protocol::requests::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
                )
                .carrying(Fields::none().and(
                    smoozy_server_protocol::requests::THE_SETUP_RUN_FIELD,
                    Identity::issue(IdentityKind::Operation).ok()?.as_text(),
                )),
            );
            assert!(
                matches!(&held, WhatCameBack::ItAnswered(answered)
                    if answered.the_request_name_it_says_it_does_not_answer().is_none()),
                "the line is held: {held:?}"
            );
            Some(program)
        }

        /// **Asked straight, through its own adapter, with no line between** —
        /// how a test reads what the machine holds.
        pub fn ask(&self, request: &Request) -> WhatCameBack {
            use std::io::Write;
            let pointer = smoozy_runtime::the_program::the_active_package_pointer_at(&self.area.display().to_string());
            let mut adapter = std::process::Command::new(pointer)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .expect("the adapter starts");
            adapter
                .stdin
                .take()
                .expect("a standard input")
                .write_all(&smoozy_server_protocol::framing::write_request_frame(request))
                .expect("the request goes in");
            let identifier = adapter.id();
            let (sent, heard) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let _ = sent.send(adapter.wait_with_output());
            });
            let Ok(Ok(said)) = heard.recv_timeout(HOW_LONG_ONE_ASKING_IS_GIVEN) else {
                let _ = std::process::Command::new("kill")
                    .args(["-KILL", &identifier.to_string()])
                    .status();
                panic!("DEADLINE MISSED: the machine's own program never answered one asking");
            };
            match smoozy_server_protocol::framing::read_one_result(&mut said.stdout.as_slice()) {
                Ok(answered) if answered.answers(request) => WhatCameBack::ItAnswered(answered),
                _ => WhatCameBack::NothingCorrelatedCameBack,
            }
        }

        /// **The next answer the adapter gives over the line is held back**
        /// for five seconds once the machine has acted on its request.
        pub fn holds_back_the_next_answer(&self) {
            self.holds_back_the_next_answer_to("");
        }

        /// **The same, for the next request whose frame carries `word`** —
        /// a request's name or a field's value, so one step of a run of many
        /// asks is the one cut.
        pub fn holds_back_the_next_answer_to(&self, word: &str) {
            let _ = std::fs::remove_file(PathBuf::from(format!("{}.answered", self.flag.display())));
            std::fs::write(&self.flag, word.as_bytes()).expect("the flag is armed");
        }

        /// **Nothing is held back any more.**
        pub fn holds_back_nothing(&self) {
            let _ = std::fs::remove_file(&self.flag);
        }

        /// **Waits until the machine has acted on the request and its answer
        /// is being held back**, within a test's own bound.
        pub async fn until_an_answer_is_held_back(&self) {
            let told = PathBuf::from(format!("{}.answered", self.flag.display()));
            let began = Instant::now();
            while !told.exists() {
                assert!(began.elapsed() < Duration::from_secs(30), "the machine never answered the request");
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
            let _ = std::fs::remove_file(&told);
        }

        /// **Whether the machine holds this job** — `State, identity and
        /// recovery` V1 step 2's ask, put to the program straight.
        pub fn holds(&self, job: &Identity) -> bool {
            use smoozy_server_protocol::requests::{
                IT_IS_PRESENT, ON_THIS_MACHINE_FIELD, THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST, THE_JOB_FIELD,
            };
            let asked = Request::speaking(
                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                Identity::issue(IdentityKind::Operation).expect("randomness"),
                THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
            )
            .carrying(Fields::none().and(THE_JOB_FIELD, job.as_text()));
            match self.ask(&asked) {
                WhatCameBack::ItAnswered(answered) => {
                    answered.fields.text(ON_THIS_MACHINE_FIELD) == Some(IT_IS_PRESENT)
                }
                other => panic!("the program did not say whether it holds {}: {other:?}", job.as_text()),
            }
        }

        /// **The supervisor asked through the far end's line**, as the
        /// running application asks it.
        pub fn over(&self, point: &ThePoint, far_end: &TheFarEnd) -> smoozy_desktop::servers::the_machines_work::OverThisServersPoint {
            let _ = self;
            smoozy_desktop::servers::the_machines_work::OverThisServersPoint {
                sessions: point.sessions.clone(),
                package_area: far_end.layout.area(Area::Package),
            }
        }
    }

    impl Drop for TheRealProgram {
        fn drop(&mut self) {
            if let Some(mut supervisor) = self.supervisor.lock().ok().and_then(|mut held| held.take()) {
                let _ = supervisor.kill();
                let _ = supervisor.wait();
            }
            // **Every program started through the far end's home** — through
            // the area, or through the link an ask plants in the home, which
            // names no area. An adapter left running through that link, its
            // supervisor gone, starts a fresh one, so this is done twice.
            for _ in 0..2 {
                let _ = std::process::Command::new("pkill")
                    .args(["-KILL", "-f", &self.home.display().to_string()])
                    .status();
                std::thread::sleep(Duration::from_millis(300));
            }
            let _ = std::fs::remove_file(&self.endpoint);
        }
    }

    /// **Says the machine's own program is not built here and stops the
    /// test.**
    macro_rules! the_program_or_say_so {
        ($far_end:expr) => {
            match TheRealProgram::started_at(&$far_end) {
                Some(program) => Arc::new(program),
                None => {
                    println!(
                        "SKIPPED: the machine's own program is not built beside this test; build the \
                         workspace's tests first."
                    );
                    return;
                }
            }
        };
    }

    // ═══════════════════════════════════════════════════════════════════════
    // The record
    // ═══════════════════════════════════════════════════════════════════════

    use smoozy_desktop::record::LocalRecord;
    use smoozy_desktop::uploading::TheRecordAnUploadWrites;

    /// **The application's own record, on a throwaway file**, lent one short
    /// act at a time as the running application lends it.
    pub(super) struct ARecordHere {
        record: Mutex<LocalRecord>,
        _folder: Scratch,
    }

    impl ARecordHere {
        pub fn new(named: &str) -> Arc<ARecordHere> {
            let folder = Scratch::new(named);
            Arc::new(ARecordHere {
                record: Mutex::new(LocalRecord::open_at(&folder.0.join("smoozy.sqlite3")).expect("the record opens")),
                _folder: folder,
            })
        }

        pub fn temporary(&self) -> PathBuf {
            self._folder.0.join("temporary")
        }

        /// **One question of the record**, as SQL.
        pub fn read<T>(&self, sql: &str, parameters: impl rusqlite::Params, row: impl FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>) -> Option<T> {
            let record = self.record.lock().expect("the record");
            record.connection().query_row(sql, parameters, row).ok()
        }

        /// **How a job stands in the local record**: *provisional*,
        /// *confirmed*, or nothing.
        pub fn standing_of(&self, job: &Identity) -> Option<String> {
            self.read("SELECT standing FROM job_record WHERE job = ?1", [job.as_text()], |row| row.get(0))
        }

        pub fn count(&self, table: &str) -> i64 {
            self.read(&format!("SELECT count(*) FROM {table}"), [], |row| row.get(0)).unwrap_or(-1)
        }
    }

    impl TheRecordAnUploadWrites for ARecordHere {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let Ok(record) = self.record.lock() else {
                return false;
            };
            act(record.connection());
            true
        }
    }

    impl smoozy_desktop::keys::LendsTheRecord for ARecordHere {
        fn write_a_diary_line(&self, _: &smoozy_keys::diary::Act) {}
        fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
            act(self.record.lock().expect("the record").connection());
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Submission and first-frame submission
    // ═══════════════════════════════════════════════════════════════════════

    use smoozy_desktop::submitting::{WhatBecameOfOneJob, WhatTheActCameTo, WhatThePageHandsOver};

    fn a_payload(frames: &str) -> smoozy_render_settings::payload::Payload {
        use smoozy_render_settings::file_half::BlenderIdentity;
        use smoozy_render_settings::payload::{compose, Composition};
        use smoozy_render_settings::store::SceneEntries;
        compose(Composition {
            version_pick: &BlenderIdentity {
                path: "/opt/blender-4.5.3/blender".to_string(),
                version: smoozy_server_setup::blender::Version {
                    major: 4,
                    minor: 5,
                    patch: 3,
                },
            },
            scene: None,
            answers: &SceneEntries {
                frame_range: Some(smoozy_render_settings::frames::parse_the_frame_field(frames).expect("a range")),
                sample_count: Some(64),
                ..Default::default()
            },
            standing: None,
            stale: false,
        })
    }

    /// **What the Render Settings page hands over** for one scene-job of the
    /// far end's file, with the first-frame check where `first_frame` says.
    pub(super) fn what_the_page_hands_over(
        far_end: &TheFarEnd,
        server: &Identity,
        installation: &Identity,
        temporary: &Path,
        first_frame: bool,
    ) -> WhatThePageHandsOver {
        use smoozy_queue_history::record::{AnswerOrigin, DependencyCheckAtSubmission, Hue, TheServerAsItStood};
        use smoozy_render_settings::reporting::ReportingChoice;
        use smoozy_rendering::submission::{the_reporting_field, WhatTheDesktopHoldsAboutTheBuild};
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        WhatThePageHandsOver {
            server: TheServerAsItStood {
                local_server: server.clone(),
                label: Some("the loopback box".to_string()),
                hue: Some(Hue(3)),
            },
            installation: Some(installation.clone()),
            layout: far_end.layout.clone(),
            package_area: far_end.layout.area(Area::Package),
            the_blend: far_end.the_blend.display().to_string(),
            label: Some("Kitchen, on the train".to_string()),
            reporting: the_reporting_field(None, ReportingChoice::Everything),
            the_version_origin: AnswerOrigin::ResolvedForThem,
            dependency_check: Some(DependencyCheckAtSubmission::NoStandingAtAll),
            what_the_desktop_holds_about_the_build: WhatTheDesktopHoldsAboutTheBuild::NothingButTheEntry,
            the_first_frame_check: first_frame,
            the_download_as_it_renders_choice: false,
            where_the_downloads_go: None,
            the_formats_chosen: None,
            scene_jobs: vec![a_payload("1-150")],
            the_reading_in_force: None,
            the_card_make: smoozy_job_files::manifest::TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
            run: issue(IdentityKind::Run),
            carries_on_from: None,
            the_temporary_area: Some(temporary.to_path_buf()),
            the_servers_fallback_folder: None,
            the_driver_at_submission: Default::default(),
            the_typed_frames: None,
        }
    }

    /// **What one act came to**: its ending, its row, its jobs, how long.
    pub(super) struct WhatTheSendCameTo {
        pub came_to: WhatTheActCameTo,
        pub which: u64,
        pub jobs: Vec<Identity>,
        pub took: Duration,
    }

    /// **One act, begun as the press begins it**, run off the test's runtime
    /// as the page runs it off the drawing thread.
    pub(super) async fn the_act(
        given: WhatThePageHandsOver,
        sessions: &ThisServersSessions,
        record: &Arc<ARecordHere>,
    ) -> WhatTheSendCameTo {
        use smoozy_desktop::downloading::TheRunsTransfers;
        use smoozy_desktop::submitting::{an_act_is_begun, the_act_is_performed_in_flight};
        let in_flight = an_act_is_begun(&given);
        let which = in_flight.which();
        let (sessions, record) = (sessions.clone(), Arc::clone(record));
        let began = Instant::now();
        let (came_to, named) = tokio::task::spawn_blocking(move || {
            let the_run = TheRunsTransfers::for_this_run();
            the_act_is_performed_in_flight(given, &sessions, &the_run, record.as_ref(), in_flight)
        })
        .await
        .expect("the act ran");
        WhatTheSendCameTo {
            came_to,
            which,
            jobs: named.into_iter().map(|job| job.job).collect(),
            took: began.elapsed(),
        }
    }

    /// **What the queue draws for an act's row now**: the row's sentence as
    /// the panel says it, where the row says *Not submitted.*
    pub(super) fn what_the_row_says(which: u64) -> Option<String> {
        use smoozy_desktop::submitting::the_acts_in_flight_to_draw;
        use smoozy_desktop::surface::render_settings::the_acts_answer;
        let nothing = |_: &str| -> Option<(std::time::SystemTime, Vec<Identity>)> { None };
        let act = the_acts_in_flight_to_draw(&nothing)
            .into_iter()
            .find(|act| act.which == which)?;
        let came_to = act.not_submitted?;
        let (said, _) = the_acts_answer(&came_to);
        Some(
            said.iter()
                .map(|part| {
                    let mut words = vec![part.lead.as_str().to_string()];
                    words.extend(part.values.iter().map(|value| value.as_str().to_string()));
                    words.extend(part.rest.iter().map(|rest| rest.as_str().to_string()));
                    words.retain(|word| !word.is_empty());
                    words.join(" ")
                })
                .collect::<Vec<_>>()
                .join(" "),
        )
    }

    /// **Where on the far end one job's send had got to**, read off the
    /// machine's own disk and its program: the remap material, the job's
    /// material, and the job record.
    fn how_far_it_got(far_end: &TheFarEnd, program: &TheRealProgram, job: &Identity) -> (bool, bool, bool) {
        use smoozy_dependencies::remap::RemapMaterial;
        let remap = PathBuf::from(String::from_utf8_lossy(
            RemapMaterial::location_in(&far_end.layout, job).as_bytes(),
        )
        .into_owned());
        let material = PathBuf::from(far_end.layout.area(Area::JobRecord))
            .join(job.as_text())
            .join(smoozy_job_files::manifest::MANIFEST_FILE_NAME);
        (remap.exists(), material.exists(), program.holds(job))
    }

    /// **What a cut send must come to**, whatever boundary the cut fell on:
    /// each job's ending agrees with the machine and the record; nothing the
    /// machine holds has gone from the record; the row says what the panel
    /// says; and the act ended within the break's window and one step.
    fn what_a_cut_send_must_come_to(
        said: &str,
        sent: &WhatTheSendCameTo,
        far_end: &TheFarEnd,
        program: &TheRealProgram,
        record: &ARecordHere,
        the_line_went: bool,
    ) -> Vec<String> {
        use smoozy_desktop::submitting::the_act_in_flight;
        let mut missed = Vec::new();
        let WhatTheActCameTo::TheseJobs(endings) = &sent.came_to else {
            return vec![format!("{said}: the act came to {:?}", sent.came_to)];
        };
        if endings.len() != sent.jobs.len() {
            missed.push(format!("{said}: {} endings for {} jobs", endings.len(), sent.jobs.len()));
        }
        if the_line_went && sent.took > the_window_closes() + ONE_STEP_AFTER_THE_BREAK {
            missed.push(format!("{said}: the act took {:?}, past the window and one step", sent.took));
        }
        let mut an_unaccounted_one = false;
        for (job, ending) in sent.jobs.iter().zip(endings) {
            let (remap, material, holds) = how_far_it_got(far_end, program, job);
            let standing = record.standing_of(job);
            println!(
                "{said}: {} came to {ending:?}; remap {remap}, material {material}, the machine holds it \
                 {holds}; the record says {standing:?}",
                &job.as_text()[..8]
            );
            match ending {
                // **In the line: the machine holds it and the record keeps it**,
                // provisional until the next connection's step 7 confirms it
                // against the machine's answer (`State, identity and recovery`
                // V1).
                WhatBecameOfOneJob::ItIsInTheLine => {
                    if !holds || standing.is_none() {
                        missed.push(format!("{said}: in the line, yet held {holds} and recorded {standing:?}"));
                    }
                }
                WhatBecameOfOneJob::NotKnown => {
                    an_unaccounted_one = true;
                    if standing.as_deref() != Some("provisional") {
                        missed.push(format!("{said}: not known, yet recorded {standing:?}"));
                    }
                }
                WhatBecameOfOneJob::NotAttemptedBehindAnUnaccountedOne => {
                    an_unaccounted_one = true;
                    if holds || standing.is_some() {
                        missed.push(format!("{said}: not attempted, yet held {holds} and recorded {standing:?}"));
                    }
                }
                WhatBecameOfOneJob::ItStoppedBeforeTheJobRecord { .. }
                | WhatBecameOfOneJob::TheFileOnTheServerCouldNotBeRead
                | WhatBecameOfOneJob::NotSentBehindAFirstFrameThatWasNot => {
                    if holds || standing.is_some() {
                        missed.push(format!("{said}: {ending:?}, yet held {holds} and recorded {standing:?}"));
                    }
                }
                other => missed.push(format!("{said}: an ending no cut gives: {other:?}")),
            }
            // **No job the machine took is gone from the record** — that is
            // the vanished job a later reading could never be matched to.
            if holds && standing.is_none() {
                missed.push(format!("{said}: the machine holds {} and the record has nothing", job.as_text()));
            }
        }
        let row = the_act_in_flight(sent.which);
        let all_in_the_line = endings.iter().all(|ending| *ending == WhatBecameOfOneJob::ItIsInTheLine);
        match (an_unaccounted_one, all_in_the_line) {
            // **Unaccounted: the queue's provisional entry takes over**
            // (`Queue & History` C10's sentence on it), and the row goes.
            (true, _) => {
                if row.is_some_and(|row| row.not_submitted.is_some()) {
                    missed.push(format!("{said}: an unaccounted job was drawn as not submitted"));
                }
            }
            (false, true) => {
                if !row.is_some_and(|row| row.ended_in_the_line) {
                    missed.push(format!("{said}: every job in the line, and the row did not wait for the machine's"));
                }
            }
            // **Not in the line: the row stays saying so, and why** — *Not
            // submitted.*, or for a first-frame check whose first job went,
            // *1 of 2 jobs submitted.* — then the reason (`Queue & History`
            // C10). The reason reads *Could not …* since step 13's words
            // sweep (2026-10-28); it read *Smoozy could not …* before.
            (false, false) => match what_the_row_says(sent.which).map(|words| words.replace('\u{a0}', " ")) {
                Some(words)
                    if (words.starts_with("Not submitted. ") || words.starts_with("1 of 2 jobs submitted. "))
                        && words.contains("Could not") =>
                {
                    println!("{said}: the row says “{words}”");
                }
                other => missed.push(format!("{said}: a job not in the line, and the row says {other:?}")),
            },
        }
        missed
    }

    /// **A submission cut at every point of its send — the remap material, the
    /// job's material and its publication, the job record request, its lost
    /// answer, and between a first-frame check's two jobs — ends within the
    /// break's window and one step, says what it came to, and leaves the
    /// record and the machine agreeing; once the line is back, an unaccounted
    /// job is resolved by the machine's own answer, never-taken material is
    /// cleared, and the same file sent again goes.** The dead-line sweep's
    /// desk coverage, *Submission and first-frame submission*.
    ///
    /// **How the cut points are reached.** One whole send is measured first:
    /// the bytes it carries towards the machine. Then the line is cut at each
    /// of a spread of fractions of that count, so the cuts fall on every step
    /// in turn, and the far end's own disk and program say where each one fell.
    /// The first-frame check is on, so each send is two jobs, the second held
    /// behind the first (`Rendering` S13).
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_submission_cut_anywhere_in_its_send_says_what_it_came_to_and_the_machine_and_record_agree() {

        let far_end = the_far_end_or_say_so!("send-cuts");
        let program = the_program_or_say_so!(far_end);

        let record = ARecordHere::new("send-cuts-record");
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let line = Arc::new(TheLine::to(&far_end).await);
        let given = || what_the_page_hands_over(&far_end, &server, &program.installation, &record.temporary(), true);

        // **One whole send, measured.**
        let point = ThePoint::over(&line, &far_end).await;
        let before = line.carried_towards_the_machine.load(Ordering::SeqCst);
        let whole = the_act(given(), &point.sessions, &record).await;
        let carried = line.carried_towards_the_machine.load(Ordering::SeqCst) - before;
        println!("a whole send: {:?} in {:?}, {carried} bytes towards the machine", whole.came_to, whole.took);
        assert_eq!(
            whole.came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine, WhatBecameOfOneJob::ItIsInTheLine]),
            "the whole send; the far end said: {}",
            far_end.its_log()
        );
        drop(point);

        let mut missed = Vec::new();
        let mut where_the_cuts_fell = Vec::new();
        for percent in [2u64, 8, 15, 25, 35, 45, 55, 62, 70, 78, 86, 93, 98] {
            let said = format!("cut at {percent}%");
            let mut point = ThePoint::over(&line, &far_end).await;
            line.goes_silent_after(carried * percent / 100);
            let sent = tokio::time::timeout(Duration::from_secs(60), the_act(given(), &point.sessions, &record))
                .await
                .unwrap_or_else(|_| panic!("{said}: the act waited past a minute"));
            let the_line_went = !line.is_up();
            if the_line_went {
                let (ending, _) = point.its_ending(Duration::from_secs(15)).await;
                if !ending.is_a_break() {
                    missed.push(format!("{said}: the lending answered {ending:?}"));
                }
            }
            missed.extend(what_a_cut_send_must_come_to(&said, &sent, &far_end, &program, &record, the_line_went));
            where_the_cuts_fell.push((
                said.clone(),
                sent.jobs.iter().map(|job| how_far_it_got(&far_end, &program, job)).collect::<Vec<_>>(),
                sent.came_to.clone(),
            ));
            drop(point);
            line.comes_back();

            // **The line back: the machine's own answer resolves an
            // unaccounted job** (`State, identity and recovery` V1 step 2), and
            // then the record and the machine agree on every job of the act.
            let point = ThePoint::over(&line, &far_end).await;
            {
                use smoozy_desktop::servers::the_machines_work::{every_provisional_record_is_resolved, WhatStepSevenDid};
                let (record, mut machine, server) = (Arc::clone(&record), program.over(&point, &far_end), server.clone());
                tokio::task::spawn_blocking(move || {
                    let mut did = WhatStepSevenDid::default();
                    every_provisional_record_is_resolved(&server, record.as_ref(), &mut machine, &mut did);
                })
                .await
                .expect("the resolution ran");
            }
            for job in &sent.jobs {
                let (holds, standing) = (program.holds(job), record.standing_of(job));
                match (holds, standing.as_deref()) {
                    (true, Some("confirmed")) | (false, None) => {}
                    other => missed.push(format!("{said}, the line back: {} held and recorded {other:?}", job.as_text())),
                }
            }
            // **Never-taken material is cleared on the matching installation,
            // and what the machine took keeps its own** (`Rendering` S4b).
            {
                let mut files = smoozy_desktop::submitting::OnThisServersPoint { sessions: &point.sessions };
                let _ = smoozy_desktop::submission_cleanup::pay(record.as_ref(), &mut files, &server, &program.installation).await;
            }
            for job in &sent.jobs {
                let (_, material, holds) = how_far_it_got(&far_end, &program, job);
                if material != holds {
                    missed.push(format!(
                        "{said}, the line back and the debt paid: {} has material {material} and is held {holds}",
                        job.as_text()
                    ));
                }
            }
            drop(point);
        }

        // **The same file sent again, the line up, goes whole.**
        let point = ThePoint::over(&line, &far_end).await;
        let again = the_act(given(), &point.sessions, &record).await;
        assert_eq!(
            again.came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine, WhatBecameOfOneJob::ItIsInTheLine]),
            "the send once the line was back"
        );
        drop(point);
        for (said, fell, came_to) in &where_the_cuts_fell {
            println!("{said}: (remap, material, record) per job {fell:?} → {came_to:?}");
        }
        assert!(missed.is_empty(), "{missed:#?}");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // One run of the application, as far as the queue is concerned
    // ═══════════════════════════════════════════════════════════════════════

    use smoozy_desktop::servers::stop_and_clear::TheWait;
    use smoozy_desktop::servers::the_machines_work::{OverThisServersPoint, WhatThisRunHolds};

    /// **The desk**: the record with a server row for the far end, the run's
    /// register of what each period holds, and the connection now lent.
    pub(super) struct TheDesk {
        pub record: Arc<ARecordHere>,
        pub server: Identity,
        pub holds: Arc<WhatThisRunHolds>,
        pub connection: u64,
    }

    impl TheDesk {
        /// **A record holding the far end's server row**, as *Add* and a
        /// finished setup leave one: the machine root recorded, and the
        /// installation last confirmed.
        pub fn for_the(far_end: &TheFarEnd, program: &TheRealProgram, named: &str) -> TheDesk {
            use smoozy_servers::identity::{Host, KeyReference, Port, Username};
            let record = ARecordHere::new(named);
            let server = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
            let row = smoozy_servers::store::ServerRow {
                local_server_identifier: server.as_text(),
                host: Host::what_the_user_typed("127.0.0.1").expect("a host"),
                port: Port::STANDARD_SSH,
                username: Username::what_the_user_typed("root").expect("a username"),
                key: KeyReference {
                    local_key_identifier: "key-1".into(),
                    fingerprint: "SHA256:aaa".into(),
                },
                label: Some("the loopback box".to_string()),
                colour: None,
                remembered_setup: None,
                remembered_hardware_reading: None,
                remembered_host_key: None,
                recorded_machine_root: Some(smoozy_servers::store::RecordedMachineRoot {
                    path: far_end.layout.root().as_path().to_string(),
                    the_package_areas_own_location: None,
                    read_under: program.installation.as_text(),
                }),
                last_confirmed_installation_identity: Some(program.installation.as_text()),
                event_cursor: None,
                machine_password_subject: None,
                chosen_fallback_folder: None,
                the_driver_missed_the_optix_bar: false,
            };
            {
                let held = record.record.lock().expect("the record");
                smoozy_desktop::record::server_rows::add(held.connection(), &row).expect("the row is added");
            }
            TheDesk {
                record,
                server,
                holds: Arc::new(WhatThisRunHolds::default()),
                connection: 0,
            }
        }

        /// ***Connect*: the point lent and `Servers` Q1 step 7 run on it**,
        /// through the point, as the running application runs it — the
        /// provisional records resolved, the journal read forward, the queue
        /// read and the offer composed.
        pub async fn connects(
            &mut self,
            line: &TheLine,
            far_end: &TheFarEnd,
            program: &TheRealProgram,
        ) -> (ThePoint, smoozy_desktop::servers::the_machines_work::WhatStepSevenDid) {
            let point = ThePoint::over(line, far_end).await;
            self.connection = self.holds.a_connection_begins(&self.server.as_text());
            let (record, holds, server, connection) =
                (Arc::clone(&self.record), Arc::clone(&self.holds), self.server.clone(), self.connection);
            let mut machine = program.over(&point, far_end);
            let did = tokio::task::spawn_blocking(move || {
                smoozy_desktop::servers::the_machines_work::the_step_seven(
                    &server,
                    connection,
                    record.as_ref(),
                    &mut machine,
                    &holds,
                    &|| true,
                )
            })
            .await
            .expect("step 7 ran");
            (point, did)
        }

        /// **`Servers` K3 and K5 heard: the break is declared**, so the reading
        /// reads as not updating (`Queue & History` C7).
        pub fn the_break_is_heard(&self) {
            self.holds.the_connection_ended(&self.server.as_text());
        }

        /// **The page hands over a submission to this desk's server.**
        pub fn given(&self, far_end: &TheFarEnd, program: &TheRealProgram, first_frame: bool, scenes: usize) -> WhatThePageHandsOver {
            let mut given = what_the_page_hands_over(
                far_end,
                &self.server,
                &program.installation,
                &self.record.temporary(),
                first_frame,
            );
            given.scene_jobs = vec![a_payload("1-150"); scenes];
            given
        }

        /// **Jobs sent on a healthy line**, every one in the machine's line.
        pub async fn sends(&self, point: &ThePoint, far_end: &TheFarEnd, program: &TheRealProgram, first_frame: bool, scenes: usize) -> Vec<Identity> {
            let sent = the_act(self.given(far_end, program, first_frame, scenes), &point.sessions, &self.record).await;
            let WhatTheActCameTo::TheseJobs(endings) = &sent.came_to else {
                panic!("the send came to {:?}", sent.came_to);
            };
            assert!(
                endings.iter().all(|ending| *ending == WhatBecameOfOneJob::ItIsInTheLine),
                "the send came to {endings:?}"
            );
            sent.jobs
        }

        /// **What the Queue's entry for this job says now** — its status line,
        /// as the screen composes it — or nothing where no entry is drawn
        /// for it.
        pub fn the_entry_says(&self, job: &Identity) -> Option<String> {
            let record = self.record.record.lock().expect("the record");
            let screen = smoozy_desktop::surface::the_queue::what_is_held(&self.holds, Some(&record), Some(&job.as_text()));
            let entry = screen.entry?;
            if entry.job != job.as_text() {
                return None;
            }
            Some(
                entry
                    .status
                    .iter()
                    .map(|part| format!("{}{}{}", part.lead, part.value, part.trail))
                    .collect::<Vec<_>>()
                    .join(" ")
                    .replace('\u{a0}', " "),
            )
        }

        /// The group a job was sent in.
        pub fn the_group_of(&self, job: &Identity) -> Option<Identity> {
            self.record
                .read("SELECT job_group FROM job_record WHERE job = ?1", [job.as_text()], |row| row.get::<_, Option<String>>(0))
                .flatten()
                .and_then(|text| Identity::read_back(IdentityKind::Group, &text))
        }
    }

    /// **The machine's queue, read straight**: every job it holds, in its
    /// order, and the hold each held job waits on.
    fn the_machines_queue(program: &Arc<TheRealProgram>) -> Vec<(Identity, Option<Identity>)> {
        use smoozy_desktop::servers::the_machines_work::{the_queue_reading_from, AsksTheSupervisor};
        struct Straight(Arc<TheRealProgram>);
        impl AsksTheSupervisor for Straight {
            fn ask(&mut self, request: &Request) -> WhatCameBack {
                self.0.ask(request)
            }
        }
        let reading = the_queue_reading_from(&mut Straight(Arc::clone(program))).expect("the program reads its queue");
        reading
            .jobs
            .iter()
            .map(|held| {
                let hold = match &held.state {
                    smoozy_queue_history::line::QueueState::Held { hold } => Some(hold.clone()),
                    _ => None,
                };
                (held.job.clone(), hold)
            })
            .collect()
    }

    /// **Where a cut falls on one of the queue's acts.**
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TheCut {
        /// **The line goes silent before the act asks anything**: the request
        /// never reaches the machine.
        BeforeItReaches,
        /// **The request reaches the machine, the machine acts and answers,
        /// and the answer never comes back**: the machine's half of the line
        /// goes silent the moment the answer is given.
        AfterTheMachineActs,
    }

    /// **One of the queue's acts, run through the server's own point with the
    /// cut where it says**, off the test's runtime as the surface runs it off
    /// the drawing thread. Answers what the act came to and how long it took.
    async fn one_queue_act<T: Send + 'static>(
        cut: TheCut,
        line: &Arc<TheLine>,
        program: &Arc<TheRealProgram>,
        point: &ThePoint,
        far_end: &TheFarEnd,
        act: impl FnOnce(&mut OverThisServersPoint) -> T + Send + 'static,
    ) -> (T, Duration) {
        let mut machine = program.over(point, far_end);
        let watching = match cut {
            TheCut::BeforeItReaches => {
                line.goes_silent();
                None
            }
            TheCut::AfterTheMachineActs => {
                program.holds_back_the_next_answer();
                let (line, program) = (Arc::clone(line), Arc::clone(program));
                Some(tokio::spawn(async move {
                    program.until_an_answer_is_held_back().await;
                    line.the_answers_stop();
                }))
            }
        };
        let began = Instant::now();
        let came_to = tokio::task::spawn_blocking(move || act(&mut machine)).await.expect("the act ran");
        let took = began.elapsed();
        if let Some(watching) = watching {
            let _ = tokio::time::timeout(Duration::from_secs(5), watching).await;
        }
        (came_to, took)
    }

    /// **A queue act's own wait at the desk**: `Rendering` K9's two minutes
    /// scaled to twelve seconds — past the desk's break window, so the
    /// waiter is still asking when the break is declared — with a short pause.
    const THE_WAIT_AT_THE_DESK: TheWait = TheWait {
        deadline: Duration::from_secs(12),
        pause: Duration::from_millis(250),
    };

    /// **How much past its own deadline a waiter may end**: one pause, and
    /// the desk noticing.
    const A_WAITERS_MARGIN: Duration = Duration::from_millis(1_500);

    // ═══════════════════════════════════════════════════════════════════════
    // Hold answers, Stop, Cancel, group Cancel, queue move, Track it, Stop
    // and clear
    // ═══════════════════════════════════════════════════════════════════════

    use smoozy_desktop::servers::the_queues_own_acts::{
        it_stands, the_cancel_is_sent, the_move_is_sent, the_stop_is_sent, TheQueuesOwnAct, WhatMoves,
        WhatTheMoveCameTo, WhereItGoes, WhereTheActStands,
    };

    /// **What every one of the queue's acts must come to, cut either way,
    /// and once the line is back.** Each act is run on jobs of its own, the
    /// line cut the way `cut` says; its ending, the entry's words, the local
    /// record and the machine are read; then the line is back, the next
    /// connection's step 7 runs, and the act is run again.
    async fn the_queues_acts_cut(cut: TheCut, named: &str) {

        use smoozy_desktop::servers::stop_and_clear::{the_pair_is_sent, WhatThePairCameTo, WhatThePairCarries};
        use smoozy_desktop::servers::the_machines_work::{
            the_held_jobs_answer_is_sent, the_offer_is_accepted, TheHeldJobsAnswer, WhatTheHeldJobsAnswerLeft,
            WhyNothingWasTracked,
        };
        let far_end = the_far_end_or_say_so!(named);
        let program = the_program_or_say_so!(far_end);

        let line = Arc::new(TheLine::to(&far_end).await);
        let mut desk = TheDesk::for_the(&far_end, &program, &format!("{named}-record"));
        let (point, _) = desk.connects(&line, &far_end, &program).await;

        // **The work the acts are run on**, sent on a healthy line: three
        // plain jobs, a group of two, and a first-frame check whose second job
        // is held behind its first.
        let plain = desk.sends(&point, &far_end, &program, false, 1).await;
        let to_cancel = desk.sends(&point, &far_end, &program, false, 1).await;
        let to_move = desk.sends(&point, &far_end, &program, false, 1).await;
        let group = desk.sends(&point, &far_end, &program, false, 2).await;
        let checked = desk.sends(&point, &far_end, &program, true, 1).await;
        drop(point);
        let (mut point, did) = desk.connects(&line, &far_end, &program).await;
        assert!(did.the_queue_was_read, "step 7 read the queue");
        let group_identity = desk.the_group_of(&group[0]).expect("a send of two scenes is one group");
        let the_rest = checked[1].clone();
        let hold = the_machines_queue(&program)
            .into_iter()
            .find_map(|(job, hold)| (job == the_rest).then_some(hold).flatten())
            .expect("the first-frame check's second job is held");

        let mut missed = Vec::new();
        let window = the_window_closes() + ONE_STEP_AFTER_THE_BREAK;
        let the_bound = |took: Duration| match cut {
            TheCut::BeforeItReaches => took <= window,
            TheCut::AfterTheMachineActs => {
                took + A_WAITERS_MARGIN >= THE_WAIT_AT_THE_DESK.deadline
                    && took <= THE_WAIT_AT_THE_DESK.deadline + A_WAITERS_MARGIN
            }
        };

        // ── Each act, cut. ────────────────────────────────────────────────
        // **Cancel**, on a waiting job.
        let job = to_cancel[0].clone();
        it_stands(&job.as_text(), Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheCancel)));
        let (stands, took) = one_queue_act(cut, &line, &program, &point, &far_end, {
            let job = job.clone();
            move |machine| the_cancel_is_sent(machine, &job, THE_WAIT_AT_THE_DESK)
        })
        .await;
        it_stands(&job.as_text(), stands.clone());
        let (ending, _) = point.its_ending(Duration::from_secs(15)).await;
        desk.the_break_is_heard();
        println!("{named}: Cancel came to {stands:?} in {took:?}; the line ended {ending:?}");
        let expected = match cut {
            TheCut::BeforeItReaches => WhereTheActStands::NotReached(TheQueuesOwnAct::TheCancel),
            TheCut::AfterTheMachineActs => WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheCancel),
        };
        if stands.as_ref() != Some(&expected) || !the_bound(took) {
            missed.push(format!("Cancel came to {stands:?} in {took:?}, not {expected:?} within its bound"));
        }
        // **While the line is down the entry says *Not updating* first**
        // (`Queue & History` C7), and beside it only a cancel sent and timed
        // out — never *Nothing was changed* about a request that may
        // have landed.
        let says = desk.the_entry_says(&job);
        let as_wanted = says.as_deref().is_some_and(|said| {
            said.starts_with("Not updating")
                && said.contains("Cancel sent. Timed out waiting for a reply.")
                    == (cut == TheCut::AfterTheMachineActs)
                && !said.contains("Nothing was changed")
        });
        if !as_wanted {
            missed.push(format!("Cancel: the entry says {says:?}"));
        }
        println!("{named}: the cancelled job's entry says {says:?}");
        // **Removed on the machine only where the request reached it; the
        // local record kept either way** — it goes only on a confirmed removal
        // (`Queue & History` L2).
        if program.holds(&job) != (cut == TheCut::BeforeItReaches) {
            missed.push(format!("Cancel: the machine holds the job {}", program.holds(&job)));
        }
        if desk.record.standing_of(&job).is_none() {
            missed.push("Cancel: the local record went on an unconfirmed removal".to_string());
        }
        line.comes_back();

        // **Stop**, naming a render the machine is not running: its own
        // outcome is *nothing was rendering*.
        let (mut point2, _) = desk.connects(&line, &far_end, &program).await;
        let render = Identity::issue(IdentityKind::Operation).expect("randomness");
        let (stands, took) = one_queue_act(cut, &line, &program, &point2, &far_end, {
            let render = render.clone();
            move |machine| the_stop_is_sent(machine, &render, THE_WAIT_AT_THE_DESK)
        })
        .await;
        let _ = point2.its_ending(Duration::from_secs(15)).await;
        desk.the_break_is_heard();
        println!("{named}: Stop came to {stands:?} in {took:?}");
        let expected = match cut {
            TheCut::BeforeItReaches => WhereTheActStands::NotReached(TheQueuesOwnAct::TheStop),
            TheCut::AfterTheMachineActs => WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheStop),
        };
        if stands.as_ref() != Some(&expected) || !the_bound(took) {
            missed.push(format!("Stop came to {stands:?} in {took:?}, not {expected:?} within its bound"));
        }
        line.comes_back();

        // **Move**, the third job to the front.
        let (mut point3, _) = desk.connects(&line, &far_end, &program).await;
        let order_before: Vec<Identity> = the_machines_queue(&program).into_iter().map(|(job, _)| job).collect();
        let (moved, took) = one_queue_act(cut, &line, &program, &point3, &far_end, {
            let job = to_move[0].clone();
            move |machine| the_move_is_sent(machine, &WhatMoves::OneJob(job), &WhereItGoes::AtTheFront, THE_WAIT_AT_THE_DESK)
        })
        .await;
        let _ = point3.its_ending(Duration::from_secs(15)).await;
        desk.the_break_is_heard();
        let order_after: Vec<Identity> = the_machines_queue(&program).into_iter().map(|(job, _)| job).collect();
        println!("{named}: Move came to {moved:?} in {took:?}");
        let expected = match cut {
            TheCut::BeforeItReaches => WhatTheMoveCameTo::NotReached,
            TheCut::AfterTheMachineActs => WhatTheMoveCameTo::TheWaiterGaveUp,
        };
        if moved != expected || !the_bound(took) {
            missed.push(format!("Move came to {moved:?} in {took:?}, not {expected:?} within its bound"));
        }
        if (order_after == order_before) != (cut == TheCut::BeforeItReaches) {
            missed.push(format!("Move: the machine's order moved {}", order_after != order_before));
        }
        line.comes_back();

        // **The group's Cancel** — the stop-and-clear pair naming the group.
        let (mut point4, _) = desk.connects(&line, &far_end, &program).await;
        for member in &group {
            it_stands(&member.as_text(), Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheCancel)));
        }
        let (pair, took) = one_queue_act(cut, &line, &program, &point4, &far_end, {
            let (record, group) = (Arc::clone(&desk.record), group_identity.clone());
            move |machine| {
                let held = record.record.lock().expect("the record");
                let mut keeper = smoozy_desktop::record::outcome_records::OutcomeRecords::over(held.connection());
                the_pair_is_sent(
                    &mut keeper,
                    machine,
                    &WhatThePairCarries {
                        group: Some(group),
                        the_render_the_desktop_last_saw: None,
                    },
                    &[],
                    THE_WAIT_AT_THE_DESK,
                )
            }
        })
        .await;
        for member in &group {
            it_stands(
                &member.as_text(),
                smoozy_desktop::servers::the_queues_own_acts::what_the_groups_cancel_leaves(pair.came_to, false),
            );
        }
        let _ = point4.its_ending(Duration::from_secs(15)).await;
        desk.the_break_is_heard();
        println!("{named}: the group's Cancel came to {:?} in {took:?}", pair.came_to);
        let expected = match cut {
            TheCut::BeforeItReaches => WhatThePairCameTo::NothingWasChanged,
            TheCut::AfterTheMachineActs => WhatThePairCameTo::NothingCameBack,
        };
        if pair.came_to != expected || !the_bound(took) {
            missed.push(format!("the group's Cancel came to {:?} in {took:?}, not {expected:?} within its bound", pair.came_to));
        }
        let gone = group.iter().filter(|member| !program.holds(member)).count();
        if (gone == 0) != (cut == TheCut::BeforeItReaches) {
            missed.push(format!("the group's Cancel: {gone} of its members gone from the machine"));
        }
        if group.iter().any(|member| desk.record.standing_of(member).is_none()) {
            missed.push("the group's Cancel: a member's local record went on an unconfirmed clear".to_string());
        }
        println!("{named}: a member's entry says {:?}", desk.the_entry_says(&group[0]));
        line.comes_back();

        // **The held job's answer: Render all.**
        let (mut point5, _) = desk.connects(&line, &far_end, &program).await;
        let (left, took) = one_queue_act(cut, &line, &program, &point5, &far_end, {
            let (job, hold) = (the_rest.clone(), hold.clone());
            move |machine| the_held_jobs_answer_is_sent(machine, &job, &hold, TheHeldJobsAnswer::RenderAll, THE_WAIT_AT_THE_DESK)
        })
        .await;
        let _ = point5.its_ending(Duration::from_secs(15)).await;
        desk.the_break_is_heard();
        println!("{named}: Render all came to {left:?} in {took:?}");
        let still_held = the_machines_queue(&program)
            .into_iter()
            .any(|(job, hold)| job == the_rest && hold.is_some());
        let answered_as_expected = match cut {
            TheCut::BeforeItReaches => matches!(
                left,
                WhatTheHeldJobsAnswerLeft::TheAnswer(smoozy_desktop::servers::the_machines_work::WhatTheAnswerCameTo::NotReached)
            ),
            TheCut::AfterTheMachineActs => matches!(
                left,
                WhatTheHeldJobsAnswerLeft::TheAnswer(smoozy_desktop::servers::the_machines_work::WhatTheAnswerCameTo::TheWaiterGaveUp)
            ),
        };
        if !answered_as_expected || !the_bound(took) {
            missed.push(format!("Render all came to {left:?} in {took:?}"));
        }
        if still_held != (cut == TheCut::BeforeItReaches) {
            missed.push(format!("Render all: the machine still holds the job back {still_held}"));
        }
        line.comes_back();

        // **Track it**, on a desk whose record does not know the work: the
        // offer composed at its step 7, then accepted with the cut under the
        // reads each entry is composed from.
        let mut stranger = TheDesk::for_the(&far_end, &program, &format!("{named}-stranger"));
        let (mut point6, did) = stranger.connects(&line, &far_end, &program).await;
        assert!(did.offered > 0, "a desk that sent nothing is offered the machine's work");
        let (tracked, took) = one_queue_act(cut, &line, &program, &point6, &far_end, {
            let (record, holds, server) = (Arc::clone(&stranger.record), Arc::clone(&stranger.holds), stranger.server.clone());
            move |machine| the_offer_is_accepted(&server, record.as_ref(), machine, &holds)
        })
        .await;
        let _ = point6.its_ending(Duration::from_secs(15)).await;
        stranger.the_break_is_heard();
        println!("{named}: Track it came to {tracked:?} in {took:?}");
        if tracked != Err(WhyNothingWasTracked::TheServerCouldNotBeReached) || took > window {
            missed.push(format!("Track it came to {tracked:?} in {took:?}"));
        }
        if stranger.record.count("job_record") != 0 {
            missed.push("Track it: a cut adoption wrote records".to_string());
        }
        line.comes_back();

        // **Stop and clear**, the whole queue — server removal's and *Update
        // now…*'s first act — on the stranger's desk, last, since it empties
        // the machine.
        let (mut point7, _) = stranger.connects(&line, &far_end, &program).await;
        let before = the_machines_queue(&program).len();
        let (pair, took) = one_queue_act(cut, &line, &program, &point7, &far_end, {
            let record = Arc::clone(&stranger.record);
            move |machine| {
                let held = record.record.lock().expect("the record");
                let mut keeper = smoozy_desktop::record::outcome_records::OutcomeRecords::over(held.connection());
                the_pair_is_sent(&mut keeper, machine, &WhatThePairCarries::default(), &[], THE_WAIT_AT_THE_DESK)
            }
        })
        .await;
        let _ = point7.its_ending(Duration::from_secs(15)).await;
        let after = the_machines_queue(&program).len();
        println!("{named}: Stop and clear came to {:?} in {took:?}; the machine held {before}, then {after}", pair.came_to);
        let expected = match cut {
            TheCut::BeforeItReaches => WhatThePairCameTo::NothingWasChanged,
            TheCut::AfterTheMachineActs => WhatThePairCameTo::NothingCameBack,
        };
        if pair.came_to != expected || !the_bound(took) {
            missed.push(format!("Stop and clear came to {:?} in {took:?}, not {expected:?} within its bound", pair.came_to));
        }
        let cleared = before > 0 && after < before;
        if cleared == (cut == TheCut::BeforeItReaches) {
            missed.push(format!("Stop and clear: the machine held {before}, then {after}"));
        }
        line.comes_back();

        // ── The line back. ────────────────────────────────────────────────
        // **The next connection's step 7 takes every late outcome off the
        // machine's journal**: a cancel or clear the machine carried out takes
        // its local records now, and the entry goes with them.
        let (point8, _) = desk.connects(&line, &far_end, &program).await;
        for job in to_cancel.iter().chain(&group) {
            let (holds, standing) = (program.holds(job), desk.record.standing_of(job));
            if !holds && standing.is_some() {
                missed.push(format!("the line back: {} is gone from the machine and still recorded {standing:?}", job.as_text()));
            }
        }
        // **And each act works again.**
        let again = plain[0].clone();
        let (stands, _) = one_queue_act_on_a_healthy_line(&program, &point8, &far_end, {
            let again = again.clone();
            move |machine| the_cancel_is_sent(machine, &again, THE_WAIT_AT_THE_DESK)
        })
        .await;
        if stands.is_some() || program.holds(&again) {
            missed.push(format!("the line back: Cancel came to {stands:?}"));
        }
        // **The stop reaches the machine again** — and, naming no render it
        // holds, is answered with the machine's own words.
        let (stands, _) = one_queue_act_on_a_healthy_line(&program, &point8, &far_end, move |machine| {
            the_stop_is_sent(machine, &render, THE_WAIT_AT_THE_DESK)
        })
        .await;
        if !matches!(stands, None | Some(WhereTheActStands::NotTaken(TheQueuesOwnAct::TheStop, Some(_)))) {
            missed.push(format!("the line back: Stop came to {stands:?}"));
        }
        let (tracked, _) = one_queue_act_on_a_healthy_line(&program, &point8, &far_end, {
            let (record, holds, server) = (Arc::clone(&stranger.record), Arc::clone(&stranger.holds), stranger.server.clone());
            move |machine| the_offer_is_accepted(&server, record.as_ref(), machine, &holds)
        })
        .await;
        println!("{named}: the line back, Track it came to {tracked:?}");
        if tracked.is_err() {
            missed.push(format!("the line back: Track it came to {tracked:?}"));
        }
        drop(point8);
        assert!(missed.is_empty(), "{missed:#?}");
    }

    /// One of the queue's acts on a healthy line.
    async fn one_queue_act_on_a_healthy_line<T: Send + 'static>(
        program: &Arc<TheRealProgram>,
        point: &ThePoint,
        far_end: &TheFarEnd,
        act: impl FnOnce(&mut OverThisServersPoint) -> T + Send + 'static,
    ) -> (T, Duration) {
        let mut machine = program.over(point, far_end);
        let began = Instant::now();
        let came_to = tokio::task::spawn_blocking(move || act(&mut machine)).await.expect("the act ran");
        (came_to, began.elapsed())
    }

    /// **Each of the queue's acts, its line gone silent before it asks
    /// anything: it ends at the break, says Smoozy could not reach the
    /// server, and the machine and the record are as they were.** The
    /// dead-line sweep's desk coverage, *Hold answers, Stop, Cancel, group
    /// Cancel, queue move, Track it, Stop and clear*: the cut before
    /// acceptance.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn the_queues_acts_cut_before_they_reach_the_machine_end_at_the_break_and_change_nothing() {
        the_queues_acts_cut(TheCut::BeforeItReaches, "queue-before").await;
    }

    /// **Each of the queue's acts, the machine's answer lost after it acted:
    /// the waiter ends at its own deadline, says the act was sent and timed
    /// out waiting, keeps the local record, and the next connection takes
    /// the late outcome.** The same coverage: the cut after acceptance.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn the_queues_acts_whose_answer_is_lost_say_so_and_the_late_outcome_is_taken() {
        the_queues_acts_cut(TheCut::AfterTheMachineActs, "queue-after").await;
    }

    /// **A waiter shorter than the break window ends at its own deadline,
    /// not at the break** — the dead-line sweep's finding 4, over the real
    /// line: the ask in hand when the deadline comes is given up then, and
    /// the cancel the machine carried out is still taken by the next
    /// connection.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_waiter_shorter_than_the_break_window_ends_at_its_own_deadline_over_a_real_line() {

        let far_end = the_far_end_or_say_so!("queue-deadline");
        let program = the_program_or_say_so!(far_end);

        let line = Arc::new(TheLine::to(&far_end).await);
        let mut desk = TheDesk::for_the(&far_end, &program, "queue-deadline-record");
        let (point, _) = desk.connects(&line, &far_end, &program).await;
        let job = desk.sends(&point, &far_end, &program, false, 1).await.remove(0);
        drop(point);
        let (point, _) = desk.connects(&line, &far_end, &program).await;
        let short = TheWait {
            deadline: Duration::from_secs(2),
            pause: Duration::from_millis(100),
        };
        let (stands, took) = one_queue_act(TheCut::AfterTheMachineActs, &line, &program, &point, &far_end, {
            let job = job.clone();
            move |machine| the_cancel_is_sent(machine, &job, short)
        })
        .await;
        println!("a two-second wait came to {stands:?} in {took:?}");
        assert_eq!(stands, Some(WhereTheActStands::TheWaiterGaveUp(TheQueuesOwnAct::TheCancel)));
        assert!(
            took <= short.deadline + Duration::from_millis(700),
            "the wait took {took:?}, past its own deadline of {:?}",
            short.deadline
        );
        assert!(desk.record.standing_of(&job).is_some(), "the local record went on an unconfirmed removal");
        drop(point);
        line.comes_back();
        let (_point, _) = desk.connects(&line, &far_end, &program).await;
        assert!(!program.holds(&job), "the machine never carried the cancel out");
        assert!(desk.record.standing_of(&job).is_none(), "the next connection did not take the late removal");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // History's Delete and Clear, and the server's half of them
    // ═══════════════════════════════════════════════════════════════════════

    /// **History's own deletion, performed on the local record** — `Queue &
    /// History` X1's one unit per entry, which is what *Delete* and each entry
    /// of *Clear* perform: the record goes and a tombstone owing the
    /// machine's half stands.
    fn history_deletes(record: &ARecordHere, jobs: &[Identity]) {
        use smoozy_desktop::record::deletions::TheDeletions;
        use smoozy_desktop::record::job_records::JobRecords;
        let held = record.record.lock().expect("the record");
        for job in jobs {
            let entry = JobRecords::over(held.connection())
                .read(job)
                .expect("the record reads")
                .expect("the entry stands");
            TheDeletions::over(held.connection())
                .perform(&smoozy_queue_history::deletion::ADeletion::of(&entry, std::time::SystemTime::now()))
                .expect("the deletion is performed");
        }
    }

    /// The jobs whose machine-side copy the record still owes.
    fn still_owed(record: &ARecordHere) -> Vec<Identity> {
        use smoozy_desktop::record::deletions::TheDeletions;
        let held = record.record.lock().expect("the record");
        TheDeletions::over(held.connection())
            .the_jobs_still_owing_their_erase()
            .expect("the debts read")
    }

    /// **The server's half, sent on this connection** — `Queue & History` X3
    /// and X4's erase for every deletion still owing it, as a confirmed
    /// connection sends it.
    async fn the_owed_erases_go(
        desk: &TheDesk,
        program: &TheRealProgram,
        point: &ThePoint,
        far_end: &TheFarEnd,
    ) -> (Vec<Identity>, Duration) {
        let (record, mut machine, sessions) = (Arc::clone(&desk.record), program.over(point, far_end), point.sessions.clone());
        let began = Instant::now();
        let paid = tokio::task::spawn_blocking(move || {
            smoozy_desktop::servers::a_period::the_owed_erases_are_sent(record.as_ref(), &mut machine, &|| {
                sessions.a_connection_is_live()
            })
        })
        .await
        .expect("the erases ran");
        (paid, began.elapsed())
    }

    /// **History's Delete and Clear stay done on this computer whatever the
    /// line does; the server's half, cut before it reaches the machine or
    /// with its answer lost, ends at the break and leaves every debt owed; a
    /// partial erase stays owed and is paid on a later connection; and no
    /// render is touched.** The dead-line sweep's desk coverage, *History
    /// Delete and Clear and the server cleanup*.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn historys_deletions_stand_and_the_servers_half_survives_every_cut_until_it_is_paid() {

        let far_end = the_far_end_or_say_so!("erase");
        let program = the_program_or_say_so!(far_end);

        let line = Arc::new(TheLine::to(&far_end).await);
        let mut desk = TheDesk::for_the(&far_end, &program, "erase-record");
        let (point, _) = desk.connects(&line, &far_end, &program).await;
        let mut jobs = Vec::new();
        for _ in 0..3 {
            jobs.extend(desk.sends(&point, &far_end, &program, false, 1).await);
        }
        drop(point);
        let (point, _) = desk.connects(&line, &far_end, &program).await;
        drop(point);
        // **A render beside them, and the file they rendered**: neither is
        // the erase's to touch.
        let render = PathBuf::from(far_end.layout.area(Area::Job)).join("an-earlier-job").join("frame-0001.png");
        std::fs::create_dir_all(render.parent().expect("a folder")).expect("the render's folder");
        std::fs::write(&render, b"a rendered frame").expect("a render");
        let the_blend_before = std::fs::read(&far_end.the_blend).expect("the blend");

        // **Delete one, then Clear the other two**: the local record goes at
        // once, the line being nothing to it.
        history_deletes(&desk.record, &jobs[..1]);
        history_deletes(&desk.record, &jobs[1..]);
        jobs.sort_by_key(|job| job.as_text());
        let mut missed = Vec::new();
        for job in &jobs {
            if desk.record.standing_of(job).is_some() {
                missed.push(format!("{} still stands in the local record", job.as_text()));
            }
        }
        assert_eq!(still_owed(&desk.record), jobs, "every deletion owes its machine half");
        let record_directory = |job: &Identity| PathBuf::from(far_end.layout.area(Area::JobRecord)).join(job.as_text());

        // **Cut before it reaches the machine.**
        let mut point = ThePoint::over(&line, &far_end).await;
        line.goes_silent();
        let (paid, took) = the_owed_erases_go(&desk, &program, &point, &far_end).await;
        let _ = point.its_ending(Duration::from_secs(15)).await;
        println!("erase, cut before: paid {paid:?} in {took:?}");
        if !paid.is_empty() || took > the_window_closes() + ONE_STEP_AFTER_THE_BREAK {
            missed.push(format!("erase, cut before: paid {paid:?} in {took:?}"));
        }
        if still_owed(&desk.record) != jobs || jobs.iter().any(|job| !record_directory(job).exists()) {
            missed.push("erase, cut before: a debt went, or the machine lost a record".to_string());
        }
        drop(point);
        line.comes_back();

        // **Cut after the machine erased the first, its answer lost.**
        let mut point = ThePoint::over(&line, &far_end).await;
        program.holds_back_the_next_answer();
        let stopping = {
            let (line, program) = (Arc::clone(&line), Arc::clone(&program));
            tokio::spawn(async move {
                program.until_an_answer_is_held_back().await;
                line.the_answers_stop();
            })
        };
        let (paid, took) = the_owed_erases_go(&desk, &program, &point, &far_end).await;
        let _ = tokio::time::timeout(Duration::from_secs(5), stopping).await;
        let _ = point.its_ending(Duration::from_secs(15)).await;
        println!(
            "erase, answer lost: paid {paid:?} in {took:?}; the first job's record directory stands {}",
            record_directory(&jobs[0]).exists()
        );
        if !paid.is_empty() || took > the_window_closes() + ONE_STEP_AFTER_THE_BREAK {
            missed.push(format!("erase, answer lost: paid {paid:?} in {took:?}"));
        }
        if record_directory(&jobs[0]).exists() {
            missed.push("erase, answer lost: the machine did not erase the first".to_string());
        }
        if still_owed(&desk.record) != jobs {
            missed.push("erase, answer lost: a debt was marked paid on an answer never heard".to_string());
        }
        drop(point);
        line.comes_back();

        // **A partial erase**: the second job's record directory will not
        // let its files go.
        std::fs::set_permissions(record_directory(&jobs[1]), std::fs::Permissions::from_mode(0o500))
            .expect("the folder is made read-only");
        let point = ThePoint::over(&line, &far_end).await;
        let (paid, _) = the_owed_erases_go(&desk, &program, &point, &far_end).await;
        std::fs::set_permissions(record_directory(&jobs[1]), std::fs::Permissions::from_mode(0o700))
            .expect("the folder is made writable again");
        let owed = still_owed(&desk.record);
        println!("erase, the line back with one folder held: paid {paid:?}; still owed {owed:?}");
        if !paid.contains(&jobs[2]) || paid.contains(&jobs[1]) || !owed.contains(&jobs[1]) {
            missed.push(format!("erase, a partial: paid {paid:?}, still owed {owed:?}"));
        }
        drop(point);

        // **The next connection retries what is still owed**, and the
        // partial is paid.
        let point = ThePoint::over(&line, &far_end).await;
        let (paid, _) = the_owed_erases_go(&desk, &program, &point, &far_end).await;
        let owed = still_owed(&desk.record);
        println!("erase, the next connection: paid {paid:?}; still owed {owed:?}");
        if !paid.contains(&jobs[1]) || owed.contains(&jobs[1]) || owed.contains(&jobs[2]) {
            missed.push(format!("erase, the next connection: paid {paid:?}, still owed {owed:?}"));
        }
        drop(point);

        if std::fs::read(&render).ok().as_deref() != Some(b"a rendered frame".as_slice()) {
            missed.push("an erase touched a render".to_string());
        }
        if std::fs::read(&far_end.the_blend).ok() != Some(the_blend_before) {
            missed.push("an erase touched the file the jobs rendered".to_string());
        }
        assert!(missed.is_empty(), "{missed:#?}");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Live figures, commentary, previews and captured text
    // ═══════════════════════════════════════════════════════════════════════

    /// The machine's own program, asked straight, as a supervisor seam.
    struct StraightToTheProgram(Arc<TheRealProgram>);

    impl smoozy_desktop::servers::the_machines_work::AsksTheSupervisor for StraightToTheProgram {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            self.0.ask(request)
        }
    }

    /// **The record, as a download writes it.**
    struct TheDoor(Arc<ARecordHere>);

    impl smoozy_desktop::downloading::TheRecordADownloadWrites for TheDoor {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            self.0.with_the_record(act)
        }
    }

    /// **A sink's whole bytes over the reading session**, as `Queue &
    /// History` E9's intake reads a finished job's captured text — the
    /// application's own reading route, behind its own seam.
    struct TheStreamsOverThePoint(ThisServersSessions);

    impl smoozy_desktop::servers::the_words_kept::ReadsAStreamBackWhole for TheStreamsOverThePoint {
        fn whole(&self, path: &str, length: u64) -> Result<Vec<u8>, String> {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|why| why.to_string())?;
            let path = path.as_bytes().to_vec();
            let read = runtime.block_on(self.0.a_reading(false, move |subsystem| {
                Box::pin(async move {
                    Ok(smoozy_dependencies::acting::checking::read_a_stream_back_over_the_file_session(
                        subsystem, &path, length,
                    )
                    .await)
                })
            }));
            match read {
                Ok(Ok(read)) => read,
                Ok(Err(failure)) => Err(format!("{failure:?}")),
                Err(not_lent) => Err(format!("{not_lent:?}")),
            }
        }
    }

    /// **A queue reading in hand with one job rendering and its two sinks
    /// named** — what the machine's own reading carries for a render this
    /// desk's machine cannot run (its render worker's process control is
    /// Linux's), so the sinks' bytes themselves are read over the real line.
    fn a_reading_with_a_render(
        rendering: &Identity,
        waiting: &Identity,
        ordinary: (&Path, u64),
        complaints: (&Path, u64),
    ) -> smoozy_queue_history::acting::adoption::TheMachinesWorkingState {
        use smoozy_queue_history::readings::{ASink, TheSinksOfARenderingJob};
        use smoozy_server_protocol::framing::Answered;
        use smoozy_server_protocol::requests::{
            the_field_of_item, HOW_MANY_JOBS_FIELD, IT_IS_RENDERING, IT_IS_WAITING, THE_JOB_FIELD,
            THE_QUEUE_STATE_FIELD, THE_RENDER_OPERATION_FIELD,
        };
        let item = |at: usize, name: &str| the_field_of_item(THE_JOB_FIELD, at, name);
        let sinks = TheSinksOfARenderingJob {
            ordinary: ASink {
                path: ordinary.0.display().to_string(),
                length: Some(ordinary.1),
            },
            complaints: ASink {
                path: complaints.0.display().to_string(),
                length: Some(complaints.1),
            },
        };
        let mut fields = Fields::none()
            .and(HOW_MANY_JOBS_FIELD, "2")
            .and(&item(0, THE_JOB_FIELD), rendering.as_text())
            .and(&item(0, THE_QUEUE_STATE_FIELD), IT_IS_RENDERING)
            .and(
                &item(0, THE_RENDER_OPERATION_FIELD),
                Identity::issue(IdentityKind::Operation).expect("randomness").as_text(),
            )
            .and(&item(1, THE_JOB_FIELD), waiting.as_text())
            .and(&item(1, THE_QUEUE_STATE_FIELD), IT_IS_WAITING);
        for (name, value) in sinks.as_pairs() {
            fields = fields.and(&item(0, name), value);
        }
        let request = smoozy_desktop::servers::the_machines_work::the_queue_reading_request().expect("a request");
        smoozy_desktop::servers::the_machines_work::the_queue_reading_in(WhatCameBack::ItAnswered(
            Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "test").carrying(fields),
        ))
        .expect("the reading reads")
    }

    fn a_reading_now<T>(value: T) -> smoozy_state::observed::Reading<T> {
        smoozy_state::observed::Reading::taken(value, std::time::SystemTime::now(), smoozy_state::observed::Clock::ThisComputer)
    }

    /// The server row's event cursor, as the record holds it.
    fn the_cursor(desk: &TheDesk) -> Option<u64> {
        let held = desk.record.record.lock().expect("the record");
        smoozy_desktop::record::server_rows::one(held.connection(), &desk.server.as_text())
            .expect("the row reads")
            .expect("the row stands")
            .event_cursor
    }

    /// **Every live read cut with its read outstanding keeps the last whole
    /// value, says it is not updating, moves no cursor and settles no fetch
    /// from what did not arrive; once the line is back each catches up once.**
    /// The dead-line sweep's desk coverage, *Live figures, commentary,
    /// previews and captured text*: the journal read forward, the queue
    /// reading, the running entry's words and its three lines, a preview's
    /// fetch, and a finished job's captured text.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn the_live_reads_cut_keep_the_last_whole_value_and_catch_up_once() {

        use smoozy_desktop::servers::the_machines_work::{
            a_read_forward_pass, the_entrys_lines_are_read,
            the_queue_reading_from, the_words_are_read, HowThePassEnded, TheSinksOverThisServersPoint,
        };
        let far_end = the_far_end_or_say_so!("live");
        let program = the_program_or_say_so!(far_end);

        let line = Arc::new(TheLine::to(&far_end).await);
        let mut desk = TheDesk::for_the(&far_end, &program, "live-record");
        let (point, _) = desk.connects(&line, &far_end, &program).await;
        let jobs = {
            let mut jobs = desk.sends(&point, &far_end, &program, false, 1).await;
            jobs.extend(desk.sends(&point, &far_end, &program, false, 1).await);
            jobs
        };
        drop(point);
        let (point, _) = desk.connects(&line, &far_end, &program).await;
        drop(point);
        let (cancelled, waiting) = (jobs[0].clone(), jobs[1].clone());
        let mut missed = Vec::new();
        let window = the_window_closes() + ONE_STEP_AFTER_THE_BREAK;

        // ── The journal, read forward. ───────────────────────────────────
        // **A cancel the machine carries out while nobody reads**: its event
        // waits in the journal, past the desk's cursor.
        let stands = tokio::task::spawn_blocking({
            let (program, job) = (Arc::clone(&program), cancelled.clone());
            move || the_cancel_is_sent(&mut StraightToTheProgram(program), &job, THE_WAIT_AT_THE_DESK)
        })
        .await
        .expect("the cancel ran");
        assert_eq!(stands, None, "the machine removed the job");
        let cursor_before = the_cursor(&desk);
        let mut point = ThePoint::over(&line, &far_end).await;
        program.holds_back_the_next_answer();
        let stopping = {
            let (line, program) = (Arc::clone(&line), Arc::clone(&program));
            tokio::spawn(async move {
                program.until_an_answer_is_held_back().await;
                line.the_answers_stop();
            })
        };
        let began = Instant::now();
        let ended = tokio::task::spawn_blocking({
            let (record, holds, server, mut machine) =
                (Arc::clone(&desk.record), Arc::clone(&desk.holds), desk.server.clone(), program.over(&point, &far_end));
            move || a_read_forward_pass(&server, record.as_ref(), &mut machine, &holds)
        })
        .await
        .expect("the pass ran");
        let took = began.elapsed();
        let _ = tokio::time::timeout(Duration::from_secs(5), stopping).await;
        println!("the read forward, its answer lost: {ended:?} in {took:?}");
        if !matches!(ended, HowThePassEnded::TheReadCouldNotComplete { applied: 0 }) || took > window {
            missed.push(format!("the read forward cut came to {ended:?} in {took:?}"));
        }
        if the_cursor(&desk) != cursor_before {
            missed.push(format!("the cursor moved on a read that never came back: {cursor_before:?} to {:?}", the_cursor(&desk)));
        }
        if desk.record.standing_of(&cancelled).is_none() {
            missed.push("the cancel's removal was applied from a read that never came back".to_string());
        }

        // ── The queue reading. ───────────────────────────────────────────
        // **Asked on the line gone silent, it answers nothing and the last
        // whole reading stands**; the break heard, it reads as not updating.
        let asked = tokio::task::spawn_blocking({
            let mut machine = program.over(&point, &far_end);
            move || the_queue_reading_from(&mut machine).is_ok()
        })
        .await
        .expect("the reading ran");
        if asked {
            missed.push("a queue reading came back over a silent line".to_string());
        }
        let _ = point.its_ending(Duration::from_secs(15)).await;
        desk.the_break_is_heard();
        let says = desk.the_entry_says(&waiting);
        println!("the waiting job's entry, the line down: {says:?}");
        if !says.as_deref().is_some_and(|said| said.starts_with("Not updating")) {
            missed.push(format!("the waiting job's entry says {says:?} with the line down"));
        }
        drop(point);
        line.comes_back();

        // **The line back: the read forward catches up once.**
        let (point, did) = desk.connects(&line, &far_end, &program).await;
        println!("the line back, step 7: {did:?}");
        if desk.record.standing_of(&cancelled).is_some() {
            missed.push("the line back: the late removal was not applied".to_string());
        }
        if the_cursor(&desk) == cursor_before {
            missed.push("the line back: the cursor did not move past what was applied".to_string());
        }
        let again = tokio::task::spawn_blocking({
            let (record, holds, server, mut machine) =
                (Arc::clone(&desk.record), Arc::clone(&desk.holds), desk.server.clone(), program.over(&point, &far_end));
            move || a_read_forward_pass(&server, record.as_ref(), &mut machine, &holds)
        })
        .await
        .expect("the pass ran");
        if again != (HowThePassEnded::ItReadToTheEnd { applied: 0 }) {
            missed.push(format!("the line back: a second pass came to {again:?}, not nothing left to apply"));
        }
        let says = desk.the_entry_says(&waiting);
        if says.as_deref().is_some_and(|said| said.starts_with("Not updating")) {
            missed.push(format!("the line back: the entry still says {says:?}"));
        }
        drop(point);

        // ── The running entry's words and its three lines. ───────────────
        let streams = far_end.home.join("streams");
        std::fs::create_dir_all(&streams).expect("the streams' folder");
        let (ordinary, complaints) = (streams.join("ordinary"), streams.join("complaints"));
        let printed: Vec<u8> = (1..=400)
            .flat_map(|at| format!("Fra:{at} Mem:12.0M | Time:00:0{} | Sample {at}/400\n", at % 10).into_bytes())
            .collect();
        std::fs::write(&ordinary, &printed).expect("the ordinary stream");
        std::fs::write(&complaints, b"").expect("the complaints stream");
        let rendering = Identity::issue(IdentityKind::Job).expect("randomness");
        let server = desk.server.as_text();
        let first_length = 1_000u64;
        let point = ThePoint::over(&line, &far_end).await;
        desk.connection = desk.holds.a_connection_begins(&server);
        desk.holds.a_reading_was_taken(
            &server,
            desk.connection,
            a_reading_now(a_reading_with_a_render(&rendering, &waiting, (&ordinary, first_length), (&complaints, 0))),
            None,
        );
        // **The Queue on screen, its entry the rendering job, and that
        // job's words open.**
        desk.holds.the_queue_is_shown(Some(&rendering.as_text()));
        desk.holds.the_words_are_opened(&server, &rendering.as_text(), "Kitchen");
        let _ = desk.holds.the_entry_is_drawn_for(Some((&server, &rendering.as_text())));
        let (first, lines_read) = tokio::task::spawn_blocking({
            let (holds, server, sessions) = (Arc::clone(&desk.holds), server.clone(), point.sessions.clone());
            move || {
                let mut reader = TheSinksOverThisServersPoint { sessions };
                (the_words_are_read(&holds, &server, &mut reader), the_entrys_lines_are_read(&holds, &server, &mut reader))
            }
        })
        .await
        .expect("the reads ran");
        let first = first.expect("the first words arrive");
        assert_eq!(first.new[0].as_bytes(), &printed[..first_length as usize], "the first words");
        assert!(lines_read, "the entry's lines are read");
        // **More printed, and the line cut with the read outstanding.**
        let mut point = point;
        desk.holds.a_reading_was_taken(
            &server,
            desk.connection,
            a_reading_now(a_reading_with_a_render(&rendering, &waiting, (&ordinary, printed.len() as u64), (&complaints, 0))),
            None,
        );
        line.goes_silent();
        let began = Instant::now();
        let (cut, lines_cut) = tokio::task::spawn_blocking({
            let (holds, server, sessions) = (Arc::clone(&desk.holds), server.clone(), point.sessions.clone());
            move || {
                let mut reader = TheSinksOverThisServersPoint { sessions };
                (the_words_are_read(&holds, &server, &mut reader), the_entrys_lines_are_read(&holds, &server, &mut reader))
            }
        })
        .await
        .expect("the reads ran");
        let took = began.elapsed();
        println!("the words, cut: {cut:?}, the lines changed {lines_cut}, in {took:?}");
        if cut.is_some() || lines_cut || took > window * 2 {
            missed.push(format!("the words cut came to {cut:?} and the lines {lines_cut} in {took:?}"));
        }
        let _ = point.its_ending(Duration::from_secs(15)).await;
        desk.the_break_is_heard();
        drop(point);
        line.comes_back();
        let point = ThePoint::over(&line, &far_end).await;
        desk.connection = desk.holds.a_connection_begins(&server);
        desk.holds.a_reading_was_taken(
            &server,
            desk.connection,
            a_reading_now(a_reading_with_a_render(&rendering, &waiting, (&ordinary, printed.len() as u64), (&complaints, 0))),
            None,
        );
        let (rest, lines_back) = tokio::task::spawn_blocking({
            let (holds, server, sessions) = (Arc::clone(&desk.holds), server.clone(), point.sessions.clone());
            move || {
                let mut reader = TheSinksOverThisServersPoint { sessions };
                (the_words_are_read(&holds, &server, &mut reader), the_entrys_lines_are_read(&holds, &server, &mut reader))
            }
        })
        .await
        .expect("the reads ran");
        match rest {
            Some(rest) => {
                let all = [first.new[0].as_bytes(), rest.new[0].as_bytes()].concat();
                if all != printed {
                    missed.push(format!(
                        "the line back: the words came to {} bytes in two arrivals, not the {} printed once",
                        all.len(),
                        printed.len()
                    ));
                }
            }
            None => missed.push("the line back: the rest of the words never arrived".to_string()),
        }
        let lines = desk.holds.the_entry_is_drawn_for(Some((&server, &rendering.as_text())));
        let every_line: Vec<String> = String::from_utf8_lossy(&printed).lines().map(str::to_string).collect();
        let wanted = every_line[every_line.len() - 3..].to_vec();
        if !lines_back || lines != wanted {
            missed.push(format!("the line back: the entry's lines read {lines:?}"));
        }
        drop(point);

        // ── A preview's fetch. ───────────────────────────────────────────
        let paths = smoozy_job_files::publication::AJobsPaths::composed(&far_end.layout, &waiting, Some("Kitchen, on the train"))
            .expect("the job's paths");
        let on_the_machine = PathBuf::from(
            smoozy_job_files::main_output::where_this_copy_is_written_in(&paths.job_record_directory, 1, true, 0).published_at,
        );
        std::fs::create_dir_all(on_the_machine.parent().expect("a folder")).expect("the derivative's folder");
        let picture = the_pattern(6 * 1024 * 1024);
        std::fs::write(&on_the_machine, &picture).expect("the derivative");
        {
            let held = desk.record.record.lock().expect("the record");
            smoozy_desktop::record::previews::a_derivative_is_owed(
                held.connection(),
                &waiting,
                1,
                &Identity::issue(IdentityKind::Operation).expect("randomness").as_text(),
                None,
                true,
                0,
                None,
            )
            .expect("the fetch is owed");
        }
        let area = Scratch::new("live-derivatives");
        let lands = smoozy_desktop::the_previews::WhereTheyLand {
            area: area.0.clone(),
            server: desk.server.clone(),
            installation: program.installation.clone(),
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            layout: far_end.layout.clone(),
        };
        let the_run = Arc::new(smoozy_desktop::downloading::TheRunsTransfers::for_this_run());
        let fetched = |point: &ThePoint| {
            let (record, lands, sessions, the_run) = (Arc::clone(&desk.record), lands.clone(), point.sessions.clone(), Arc::clone(&the_run));
            tokio::task::spawn_blocking(move || {
                let mut reaches = smoozy_desktop::the_previews::OverThisServersPoint {
                    record: TheDoor(Arc::clone(&record)),
                    sessions,
                    the_run,
                };
                smoozy_desktop::the_previews::the_owed_are_fetched(record.as_ref(), &mut reaches, &lands, &Default::default())
            })
        };
        let held_picture = || {
            let held = desk.record.record.lock().expect("the record");
            smoozy_desktop::the_previews::the_picture_held_for(held.connection(), &waiting)
        };
        let mut point = ThePoint::over(&line, &far_end).await;
        line.goes_silent_after_bringing(1024 * 1024);
        let began = Instant::now();
        let did = fetched(&point).await.expect("the fetch ran");
        let took = began.elapsed();
        println!("a preview, cut part-way: {did:?} in {took:?}");
        if did.published != 0 || did.still_owed != 1 || held_picture().is_some() {
            missed.push(format!("a preview cut part-way came to {did:?}, the picture held {:?}", held_picture()));
        }
        if took > window + Duration::from_secs(120) {
            missed.push(format!("a preview cut part-way took {took:?}"));
        }
        let _ = point.its_ending(Duration::from_secs(15)).await;
        drop(point);
        line.comes_back();
        let point = ThePoint::over(&line, &far_end).await;
        let did = fetched(&point).await.expect("the fetch ran");
        println!("a preview, the line back: {did:?}");
        match held_picture() {
            Some(held) if did.published == 1 && std::fs::read(&held.path).ok().as_deref() == Some(picture.as_slice()) => {}
            other => missed.push(format!("the line back: the preview came to {did:?}, the picture held {other:?}")),
        }
        drop(point);

        // ── A finished job's captured text. ──────────────────────────────
        // **Owed the way an ending's learning leaves it.** This desk's
        // machine renders nothing, so the job the intake asks about is one
        // the machine holds with no attempt: its debt, once the machine is
        // heard, settles with nothing — and until then it stands.
        {
            let held = desk.record.record.lock().expect("the record");
            held.connection()
                .execute("UPDATE job_record SET words_owed = 1 WHERE job = ?1", [waiting.as_text()])
                .expect("the words are owed");
        }
        let words_owed = || {
            desk.record
                .read("SELECT words_owed FROM job_record WHERE job = ?1", [waiting.as_text()], |row| row.get::<_, i64>(0))
                .unwrap_or(-1)
        };
        let taken_in = |point: &ThePoint| {
            let (record, server, mut machine, reads) = (
                Arc::clone(&desk.record),
                desk.server.clone(),
                program.over(point, &far_end),
                TheStreamsOverThePoint(point.sessions.clone()),
            );
            tokio::task::spawn_blocking(move || {
                smoozy_desktop::servers::the_words_kept::the_words_owed_are_taken_in(
                    &server,
                    record.as_ref(),
                    &mut machine,
                    &reads,
                    &Default::default(),
                )
                .0
            })
        };
        let mut point = ThePoint::over(&line, &far_end).await;
        line.goes_silent();
        let began = Instant::now();
        let did = taken_in(&point).await.expect("the intake ran");
        let took = began.elapsed();
        println!("the captured text, cut: {did:?} in {took:?}");
        if did.still_owed != 1 || did.settled_with_nothing != 0 || words_owed() != 1 || took > window {
            missed.push(format!("the captured text cut came to {did:?} in {took:?}, owed {}", words_owed()));
        }
        let _ = point.its_ending(Duration::from_secs(15)).await;
        drop(point);
        line.comes_back();
        // **The line back, the machine heard**: a job with no ending yet
        // keeps its debt and nothing is invented for it; once the machine no
        // longer holds the job, its answer settles the debt with nothing
        // (`Queue & History` E9's boundary).
        let point = ThePoint::over(&line, &far_end).await;
        let did = taken_in(&point).await.expect("the intake ran");
        let kept = desk.record.count("job_record_words");
        println!("the captured text, the line back, the job not ended: {did:?}; {kept} words kept");
        if did.still_owed != 1 || did.landed != 0 || words_owed() != 1 || kept > 0 {
            missed.push(format!("the line back: the captured text came to {did:?}, owed {}, {kept} kept", words_owed()));
        }
        let stands = tokio::task::spawn_blocking({
            let (program, job) = (Arc::clone(&program), waiting.clone());
            move || the_cancel_is_sent(&mut StraightToTheProgram(program), &job, THE_WAIT_AT_THE_DESK)
        })
        .await
        .expect("the cancel ran");
        assert_eq!(stands, None, "the machine removed the job");
        let did = taken_in(&point).await.expect("the intake ran");
        println!("the captured text, the job gone from the machine: {did:?}");
        if did.settled_with_nothing != 1 || words_owed() != 0 {
            missed.push(format!("the captured text, the job gone: {did:?}, owed {}", words_owed()));
        }
        drop(point);
        assert!(missed.is_empty(), "{missed:#?}");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Set up, Set up again, Stop and abandon, and Blender removal
    // ═══════════════════════════════════════════════════════════════════════

    use smoozy_desktop::setting_up::{TheRemovalAsked, WhereTheSetUpGot};

    /// **The two opening readings, answered as a rental's Ubuntu answers
    /// them** — `Server Setup` M1a reads absolute paths a desk has none of,
    /// so they cross the setup's own seam; everything after them goes over
    /// the line.
    struct AnUbuntuMachine;

    impl smoozy_desktop::setting_up::TakesTheOpeningReadings for AnUbuntuMachine {
        fn taken(
            &mut self,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<'_, smoozy_server_setup::readings::TheMachineBeforeAnythingIsTouched>
        {
            Box::pin(async {
                smoozy_server_setup::readings::TheMachineBeforeAnythingIsTouched::as_answered(
                    smoozy_server_setup::kind::MachineKind::ASystemdMachine,
                    Some("ID=ubuntu\nVERSION_ID=\"24.04\"\n".to_string()),
                    None,
                )
            })
        }
    }

    /// **The server's row, as a set-up server's stands**, read from and
    /// written to the desk's own record.
    struct TheRowOnTheDesk {
        record: Arc<ARecordHere>,
        server: Identity,
    }

    impl smoozy_desktop::setting_up::KeepsTheServersRow for TheRowOnTheDesk {
        fn the_row(&mut self) -> Option<smoozy_servers::store::ServerRow> {
            let held = self.record.record.lock().ok()?;
            smoozy_desktop::record::server_rows::one(held.connection(), &self.server.as_text()).ok().flatten()
        }
        fn record_the_root(&mut self, _recorded: smoozy_servers::store::RecordedMachineRoot) -> Result<(), String> {
            Err("a removal records no root".to_string())
        }
        fn record_the_installation_and_clear_the_cursor(&mut self, _identity: &Identity) -> Result<(), String> {
            Err("a removal records no installation".to_string())
        }
        fn clear_the_remembered_setup(&mut self) -> Result<(), String> {
            Ok(())
        }
    }

    /// **A Blender the machine holds**, published under its Blender area as
    /// a setup leaves one; its executable's path.
    fn a_blender_on_the_machine(far_end: &TheFarEnd, folder: &str) -> PathBuf {
        let at = PathBuf::from(far_end.layout.area(Area::Blender)).join(folder);
        std::fs::create_dir_all(at.join("lib")).expect("the build's folder");
        std::fs::write(at.join("lib").join("a-library.so"), vec![9u8; 64 * 1024]).expect("a library");
        a_program(&at.join("blender"), "#!/bin/sh\necho 'Blender 4.5.3'\n");
        at.join("blender")
    }

    /// **One removal run, driven over the line** — `Server Setup` B15's plain
    /// deletion, through the machine's own setup worker.
    async fn a_removal_over_the_line(desk: &TheDesk, point: &ThePoint, executable: &Path) -> (WhereTheSetUpGot, Duration, Vec<bool>) {
        let (sessions, record, server, executable) = (
            point.sessions.clone(),
            Arc::clone(&desk.record),
            desk.server.clone(),
            executable.display().to_string(),
        );
        let began = Instant::now();
        let (came_to, waiting) = tokio::task::spawn_blocking(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("a runtime for the run");
            let mut row = TheRowOnTheDesk { record, server };
            struct TheCard(Vec<bool>);
            impl smoozy_desktop::setting_up::AsksForThePassword for TheCard {
                fn the_password_for(&mut self, _: smoozy_server_setup::ready_action::Step) -> smoozy_desktop::servers::sessions::ItsOwnFuture<'_, Option<smoozy_security_and_licence::plaintext::Origin>> {
                    Box::pin(async { None })
                }
                fn the_cut_run_is_being_waited_for(&mut self, waiting: bool) {
                    self.0.push(waiting);
                    if waiting {
                        let panel = smoozy_desktop::surface::set_up::the_panel_while_the_cut_run_finishes(false);
                        let read = serde_json::to_string(&panel).expect("the running card");
                        assert_eq!(panel.title.as_str().replace('\u{a0}', " "), "Waiting for the interrupted setup to finish.");
                        assert!(panel.the_state_is_underway && panel.cancel.is_some());
                        assert!(panel.steps.is_empty() && panel.set_up_again.is_none());
                        assert!(!read.contains('`'));
                    }
                }
            }
            let mut card = TheCard(Vec::new());
            let came_to = runtime.block_on(smoozy_desktop::setting_up::a_blender_is_removed(
                &sessions,
                &mut AnUbuntuMachine,
                &mut row,
                TheRemovalAsked {
                    executable,
                    the_answered_version: "4.5.3".to_string(),
                    covered: Vec::new(),
                    the_managers_package: None,
                },
                &mut card,
                &smoozy_desktop::setting_up::NobodyAsked,
                &mut |_| {},
            ));
            (came_to, card.0)
        })
        .await
        .expect("the run ran");
        (came_to, began.elapsed(), waiting)
    }

    /// **A Blender's removal on a healthy line reaches *done* and the build
    /// is gone** — the far end's own setup worker, over the line. The shape
    /// every cut below is measured against.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_blender_removal_on_a_healthy_line_reaches_done() {
        let far_end = the_far_end_or_say_so!("removal-healthy");
        let program = match TheRealProgram::started_at_holding(&far_end, false) {
            Some(program) => Arc::new(program),
            None => {
                println!("SKIPPED: the machine's own program is not built beside this test");
                return;
            }
        };
        let line = Arc::new(TheLine::to(&far_end).await);
        let desk = TheDesk::for_the(&far_end, &program, "removal-healthy-record");
        let executable = a_blender_on_the_machine(&far_end, "blender-4.5.3-linux-x64");
        let point = ThePoint::over(&line, &far_end).await;
        let (came_to, took, _) = a_removal_over_the_line(&desk, &point, &executable).await;
        println!("a removal on a healthy line came to {came_to:?} in {took:?}; the build stands {}", executable.exists());
        assert!(
            matches!(&came_to, WhereTheSetUpGot::TheRunEnded { ending: smoozy_desktop::setting_up::TheRunsEnding::ItReachedDone { .. }, .. }),
            "{came_to:?}"
        );
        assert!(!executable.exists(), "the build was not removed");
    }

    /// **Where a cut falls on a setup's run.**
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ASetupsCut {
        /// The line silent before the run opens.
        BeforeTheRunOpens,
        /// **During a step**: the removal step is commanded, the machine's
        /// worker takes it, and the answer never comes back.
        DuringTheRemovalStep,
        /// **Just after the step's boundary**: the removal recorded, and the
        /// final check's command answered into a silent line.
        JustAfterItsBoundary,
    }

    /// Whether the machine's setup-in-progress state stands, and for which run.
    fn the_setup_state_on(program: &TheRealProgram) -> Option<String> {
        use smoozy_server_protocol::requests::{THE_HEALTH_ANSWER_REQUEST, THE_SETUP_IN_PROGRESS_FIELD};
        let asked = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            THE_HEALTH_ANSWER_REQUEST,
        );
        match program.ask(&asked) {
            WhatCameBack::ItAnswered(answered) => answered.fields.text(THE_SETUP_IN_PROGRESS_FIELD).map(str::to_string),
            other => panic!("the program's health answer: {other:?}"),
        }
    }

    /// **One cut on a removal run, on a machine of its own**, then *Set up
    /// again* once, then the abandon. What it missed, in words.
    async fn one_setup_cut(cut: ASetupsCut) -> Vec<String> {
        use smoozy_desktop::setting_up::{a_setup_is_abandoned, TheRunsEnding, WhatTheAbandonCameTo};
        use smoozy_desktop::surface::set_up::what_the_removal_came_to;
        use smoozy_server_protocol::requests::{THE_BUILD_IS_REMOVED_STEP, THE_FINAL_VERIFICATION_STEP};
        let named = format!("setup-{cut:?}");
        let Some(far_end) = TheFarEnd::made(&named) else {
            return Vec::new();
        };
        let far_end = Arc::new(far_end);
        let Some(program) = TheRealProgram::started_at_holding(&far_end, false) else {
            return Vec::new();
        };
        let program = Arc::new(program);
        let line = Arc::new(TheLine::to(&far_end).await);
        let desk = TheDesk::for_the(&far_end, &program, &format!("{named}-record"));
        let mut missed = Vec::new();
        let reached_done = |came_to: &WhereTheSetUpGot| {
            matches!(came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. })
        };
        let executable = a_blender_on_the_machine(&far_end, "blender-4.5.3-linux-x64");
        let mut point = ThePoint::over(&line, &far_end).await;
        let watching = match cut {
            ASetupsCut::BeforeTheRunOpens => {
                line.goes_silent();
                None
            }
            ASetupsCut::DuringTheRemovalStep | ASetupsCut::JustAfterItsBoundary => {
                program.holds_back_the_next_answer_to(if cut == ASetupsCut::DuringTheRemovalStep {
                    THE_BUILD_IS_REMOVED_STEP
                } else {
                    THE_FINAL_VERIFICATION_STEP
                });
                let (line, program) = (Arc::clone(&line), Arc::clone(&program));
                Some(tokio::spawn(async move {
                    program.until_an_answer_is_held_back().await;
                    line.the_answers_stop();
                    Instant::now()
                }))
            }
        };
        let began = Instant::now();
        let (came_to, _, _) = a_removal_over_the_line(&desk, &point, &executable).await;
        let ended = Instant::now();
        // **Measured from the cut**, which for a cut inside the run falls
        // part-way through it.
        let cut_at = match watching {
            Some(watching) => tokio::time::timeout(Duration::from_secs(10), watching)
                .await
                .ok()
                .and_then(Result::ok)
                .unwrap_or(began),
            None => began,
        };
        let took = ended.saturating_duration_since(cut_at);
        program.holds_back_nothing();
        let said = what_the_removal_came_to(&came_to);
        println!(
            "{cut:?}: came to {came_to:?}, {took:?} after the cut; the card says “{}” ({:?}); the build stands {}",
            said.said,
            said.the_machines_own_words,
            executable.exists()
        );
        if !line.is_up() {
            let _ = point.its_ending(Duration::from_secs(15)).await;
        } else {
            missed.push(format!("{cut:?}: the line never went silent under the run"));
        }
        if reached_done(&came_to) || said.said.is_empty() || said.said == "Blender removed." {
            missed.push(format!("{cut:?}: a cut run came to {came_to:?}, saying “{}”", said.said));
        }
        // **Its own bound**: the break, and the one step under way.
        if took > the_window_closes() + ONE_STEP_AFTER_THE_BREAK {
            missed.push(format!("{cut:?}: the cut run took {took:?}"));
        }
        if cut == ASetupsCut::BeforeTheRunOpens && !executable.exists() {
            missed.push("a run cut before it opened removed the build".to_string());
        }
        drop(point);
        line.comes_back();

        // **One press of Set up again waits for the cut run and finishes**
        // (`Server Setup` S5, `Server Setup` Y9). The user never has to keep pressing it.
        let point = ThePoint::over(&line, &far_end).await;
        let (again, took, waiting) = a_removal_over_the_line(&desk, &point, &executable).await;
        if cut != ASetupsCut::BeforeTheRunOpens && waiting != [true, false] {
            missed.push(format!("{cut:?}: the wait was not shown and cleared: {waiting:?}"));
        }
        let said = what_the_removal_came_to(&again);
        println!("{cut:?}: one Set up again came to {again:?} in {took:?}; the card says {said:?}");
        if said.the_machines_own_words.as_deref().is_some_and(|words| words.contains('`')) {
            missed.push(format!("{cut:?}: a plan marker reached the card"));
        }
        if !reached_done(&again) || executable.exists() {
            missed.push(format!("{cut:?}: set up again came to {again:?}; the build stands {}", executable.exists()));
        }
        if let Some(state) = the_setup_state_on(&program) {
            missed.push(format!("{cut:?}: set up again left the machine's setup state standing for {state}"));
        }

        // **Stop and abandon**: with no setup state standing — a removal is
        // not machine-changing — there is nothing to give up, and nothing is
        // sent.
        let abandoned = {
            let (sessions, area) = (point.sessions.clone(), far_end.layout.area(Area::Package));
            tokio::task::spawn_blocking(move || {
                let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("a runtime");
                let mut nobody = smoozy_desktop::setting_up::NobodyIsThereToAsk;
                runtime.block_on(a_setup_is_abandoned(&sessions, &area, &mut nobody))
            })
            .await
            .expect("the abandon ran")
        };
        println!("{cut:?}: the abandon came to {abandoned:?}");
        if abandoned != WhatTheAbandonCameTo::NothingStoodToAbandon {
            missed.push(format!("{cut:?}: the abandon came to {abandoned:?} over no setup state"));
        }
        drop(point);
        missed
    }

    /// **A setup's run cut at each of its points — before it opens, during a
    /// step, and just after the step's boundary — ends within the break's
    /// window and its step, says what it came to in the card's own words, and
    /// the next run (*Set up again*) reaches the end once the machine has let
    /// the cut run go.** The dead-line sweep's desk coverage, *Set up, Set up
    /// again, Stop and abandon, Blender removal*, over the machine's own setup
    /// worker: a Blender removal is the one setup run whose every step this
    /// desk's machine can perform (the driver, swap and install steps reach a
    /// Linux package system and the internet). Each cut has a machine of its
    /// own, run beside the others.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_setups_run_cut_at_each_point_says_so_and_the_next_run_picks_it_up() {
        if TheFarEnd::made("setup-probe").is_none() || TheRealProgram::the_one_built().is_none() {
            println!("SKIPPED: no OpenSSH, or the machine's own program is not built beside this test");
            return;
        }
        let (before, during, after) = tokio::join!(
            one_setup_cut(ASetupsCut::BeforeTheRunOpens),
            one_setup_cut(ASetupsCut::DuringTheRemovalStep),
            one_setup_cut(ASetupsCut::JustAfterItsBoundary),
        );
        let missed: Vec<String> = before.into_iter().chain(during).chain(after).collect();
        assert!(missed.is_empty(), "{missed:#?}");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // A silence briefly below the declaration window
    // ═══════════════════════════════════════════════════════════════════════

    /// **The line silent for `for_this_long`, beginning `after` from now**,
    /// while whatever the test runs goes on.
    fn a_brief_silence(line: &Arc<TheLine>, after: Duration, for_this_long: Duration) -> tokio::task::JoinHandle<()> {
        let line = Arc::clone(line);
        tokio::spawn(async move {
            tokio::time::sleep(after).await;
            line.goes_silent();
            tokio::time::sleep(for_this_long).await;
            line.comes_back();
        })
    }

    /// **A silence shorter than the declaration window is no break: every
    /// act under it carries on over the same connection and ends as it would
    /// on a healthy line.** The dead-line sweep's *run the primary-line cases
    /// briefly below the declaration window and past it* — the cases above
    /// are past it; these are below it, for a submission with its first-frame
    /// check, a queue act, the journal's read forward and a sixteen-megabyte
    /// whole-file write.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_silence_below_the_window_is_no_break_and_every_act_under_it_ends_as_on_a_healthy_line() {

        use smoozy_desktop::servers::the_machines_work::{a_read_forward_pass, HowThePassEnded};
        let far_end = the_far_end_or_say_so!("brief");
        let program = the_program_or_say_so!(far_end);

        let line = Arc::new(TheLine::to(&far_end).await);
        let mut desk = TheDesk::for_the(&far_end, &program, "brief-record");
        let (opens, _) = AT_THE_DESK.the_declaration_window();
        let brief = opens / 2;
        let (point, _) = desk.connects(&line, &far_end, &program).await;
        let mut missed = Vec::new();

        // **A submission with its first-frame check.**
        let silence = a_brief_silence(&line, Duration::from_millis(100), brief);
        let sent = the_act(desk.given(&far_end, &program, true, 1), &point.sessions, &desk.record).await;
        let _ = silence.await;
        println!("a send under a brief silence: {:?} in {:?}", sent.came_to, sent.took);
        if sent.came_to != WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine, WhatBecameOfOneJob::ItIsInTheLine]) {
            missed.push(format!("the send came to {:?}", sent.came_to));
        }

        // **A queue act: Cancel, on the first job.**
        let silence = a_brief_silence(&line, Duration::from_millis(50), brief);
        let (stands, took) = one_queue_act_on_a_healthy_line(&program, &point, &far_end, {
            let job = sent.jobs[0].clone();
            move |machine| the_cancel_is_sent(machine, &job, THE_WAIT_AT_THE_DESK)
        })
        .await;
        let _ = silence.await;
        println!("a cancel under a brief silence: {stands:?} in {took:?}");
        if stands.is_some() || program.holds(&sent.jobs[0]) {
            missed.push(format!("the cancel came to {stands:?}"));
        }

        // **The journal, read forward**, taking the cancel's removal.
        let silence = a_brief_silence(&line, Duration::from_millis(50), brief);
        let ended = tokio::task::spawn_blocking({
            let (record, holds, server, mut machine) =
                (Arc::clone(&desk.record), Arc::clone(&desk.holds), desk.server.clone(), program.over(&point, &far_end));
            move || a_read_forward_pass(&server, record.as_ref(), &mut machine, &holds)
        })
        .await
        .expect("the pass ran");
        let _ = silence.await;
        println!("a read forward under a brief silence: {ended:?}");
        if !matches!(ended, HowThePassEnded::ItReadToTheEnd { .. }) {
            missed.push(format!("the read forward came to {ended:?}"));
        }

        // **A whole-file write of sixteen megabytes.**
        let bytes = the_pattern(16 * 1024 * 1024);
        let at = far_end.home.join("brief.bin");
        let silence = a_brief_silence(&line, Duration::from_millis(20), brief);
        let wrote = the_whole_file_is_written(&point.sessions, &at, bytes.clone()).await;
        let _ = silence.await;
        if wrote.is_err() || std::fs::read(&at).ok().as_deref() != Some(bytes.as_slice()) {
            missed.push(format!("the whole-file write came to {wrote:?}"));
        }

        if !point.sessions.a_connection_is_live() {
            missed.push("a silence below the window was declared a break".to_string());
        }
        drop(point);
        assert!(missed.is_empty(), "{missed:#?}");
    }
}

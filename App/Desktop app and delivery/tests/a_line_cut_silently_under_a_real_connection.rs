//! **A real connection, a real OpenSSH server, and a line that goes silent
//! between them.** Written 2026-10-02 by step 10c's check-before-0.1.7 fix
//! piece 1, out of the check's first fault: after a minute's silence under an
//! upload on Hyperstack, the server row read *Idle · 1 connected* while the
//! copy held no network connection at all.
//!
//! **And a machine restarting under the connection**, added 2026-10-02 by the
//! fixes before 0.1.8, fault 1, out of the check before 0.1.8: after the
//! setup's own restart on Hyperstack the row read *Idle · 1 connected*, Files
//! *Reading /* and the setup *waiting for it to restart* for eleven minutes.
//! The line can now stand in for the machine going down and coming back, ending
//! the old connection politely or with a reset.
//!
//! **What the far end is.** OpenSSH's own `sshd`, run by this test as the
//! user running it, in its one-connection mode (`sshd -i`) on a socket pair —
//! so every byte the desk sends goes through the SSH transport Smoozy drives
//! (`Servers` K1a) to the program a rental runs, and every file-transfer session
//! is OpenSSH's own `sftp-server`. **What is between them is the line**: a
//! relay this test holds, which can go silent the way a provider's firewall
//! does — it stops carrying bytes either way and closes nothing, so what the
//! desk sends piles up unread and nothing comes back.
//!
//! **What the desk side is.** The application's own transport
//! (`smoozy_server_protocol::transport::ssh`), lent through the application's
//! own per-server point (`lend_until_it_ends`). **The point's lending is what
//! `Servers`' driving waits on**, and its answer is the moment the server row
//! stops reading *connected* (`Servers` K6) — so a lending that does not
//! answer is exactly a row reading *connected* over a connection that is gone.
//!
//! **Nothing of the Creator's is touched.** The keys, the server's
//! configuration and every file written are inside a throwaway folder made for
//! the test and deleted with it, and the server is a child of the test that
//! ends with it.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! **Every marker of another document is written out with that document at its
//! own site and this file keeps no header list.**

#[cfg(not(unix))]
#[test]
fn the_far_end_here_wants_a_unix_socket_pair() {
    println!(
        "SKIPPED: the far end here is OpenSSH's own sshd on a Unix socket pair, which this \
         system ({}) has none of.",
        std::env::consts::OS
    );
}

#[cfg(unix)]
mod on_a_desk_with_openssh {
    use std::net::SocketAddr;
    use std::os::unix::io::{AsRawFd, FromRawFd};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::sync::watch;

    use smoozy_desktop::servers::sessions::{
        lend_until_it_ends, TheAdditionalConnection, ThisServersSessions,
    };
    use smoozy_desktop::surface::files::{
        a_folder_is_read, the_tab_after_the_reading, TheListing, WhatTheReadingFound,
    };
    use smoozy_server_protocol::subsystem::client::Subsystem;
    use smoozy_server_protocol::transport::ssh::{self, Connection};
    use smoozy_server_protocol::transport::{
        Credential, DecideHostTrust, Ending, Endpoint, Figures, HostTrust,
    };
    use smoozy_servers::condition::ObservedValue;

    // ------------------------------------------------------------- the far end

    /// Where OpenSSH's own server and file-transfer server are, on the systems
    /// this runs on.
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
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let path = std::env::temp_dir().join(format!(
                "smoozy-silent-line-{named}-{}-{}",
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

    /// **OpenSSH's own server, ready to take one connection at a time on a
    /// socket it is handed**, with a host key and the client's key made by
    /// OpenSSH's own `ssh-keygen`.
    struct TheFarEnd {
        sshd: PathBuf,
        config: PathBuf,
        client_key: PathBuf,
        home: PathBuf,
        _scratch: Scratch,
    }

    impl TheFarEnd {
        fn made(named: &str) -> Option<TheFarEnd> {
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
            let config = folder.join("sshd_config");
            std::fs::write(
                &config,
                format!(
                    "HostKey {}\nAuthorizedKeysFile {}\nPidFile none\nUsePAM no\nStrictModes no\n\
                     PasswordAuthentication no\nKbdInteractiveAuthentication no\n\
                     Subsystem sftp {}\n",
                    folder.join("host").display(),
                    folder.join("authorized_keys").display(),
                    sftp.display()
                ),
            )
            .ok()?;
            let home = folder.join("home");
            std::fs::create_dir_all(&home).ok()?;
            // **The server's own check of its configuration**, so a system
            // whose `sshd` cannot run for this user — one wanting a folder only
            // its own service makes, say — is said and skipped rather than
            // failed.
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
                _scratch: scratch,
            })
        }

        /// One connection's server: `sshd -i`, its standard input and output a
        /// socket whose other end is handed back.
        fn one_connection(&self) -> (std::process::Child, tokio::net::TcpStream) {
            // **A loopback TCP pair rather than a Unix one**: macOS's own
            // `sshd` will not open an audit session on a socket with no
            // address, and refuses the login.
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
            (
                child,
                tokio::net::TcpStream::from_std(ours).expect("the end we drive"),
            )
        }
    }

    fn copy_of(descriptor: std::os::unix::io::RawFd) -> std::os::unix::io::RawFd {
        let copy = unsafe { libc::dup(descriptor) };
        assert!(copy >= 0, "the descriptor could not be copied");
        copy
    }

    // ---------------------------------------------------------------- the line

    /// **The line between the desk and the machine.** Up, it carries every byte
    /// both ways. **Silent, it carries nothing and closes nothing**: what the
    /// desk sends stays unread until the desk's own socket is full, and nothing
    /// comes back — which is what a provider's firewall dropping every packet
    /// looks like from the desk. **Each connection it carries is a server of
    /// its own**, ended with the line.
    struct TheLine {
        address: SocketAddr,
        up: watch::Sender<bool>,
        /// The desk-to-machine half alone, which goes first.
        towards_the_machine: watch::Sender<bool>,
        /// **How many times the machine has restarted, and how its old
        /// connections ended the last time** — see [`TheLine::the_machine_restarts`].
        /// Every connection carried before a restart ends with it; every one
        /// made after is carried as before.
        restarts: watch::Sender<(u64, HowTheOldConnectionEnds)>,
        servers: Arc<Mutex<Vec<std::process::Child>>>,
    }

    /// **How a machine that restarted treats a connection made before it went
    /// down.** Added 2026-10-02, the fixes before 0.1.8, fault 1.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum HowTheOldConnectionEnds {
        /// **Its service closed the socket as it shut down** — `Servers` K2's
        /// first bullet, and what the earlier passes on Hyperstack met: the
        /// desk reads the close and the connection is over.
        Politely,
        /// **The machine went down without closing anything, and came back on
        /// a fresh boot that knows nothing of the old connection**, so the
        /// desk's next packet on it is answered with a reset.
        WithAReset,
    }

    impl Drop for TheLine {
        fn drop(&mut self) {
            if let Ok(mut servers) = self.servers.lock() {
                for server in servers.iter_mut() {
                    let _ = server.kill();
                    let _ = server.wait();
                }
            }
        }
    }

    impl TheLine {
        async fn to(far_end: &Arc<TheFarEnd>) -> TheLine {
            let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
                .await
                .expect("the loopback address takes a listener");
            let address = listener
                .local_addr()
                .expect("a bound listener has an address");
            let (up, _) = watch::channel(true);
            let (towards_the_machine, _) = watch::channel(true);
            let (restarts, _) = watch::channel((0, HowTheOldConnectionEnds::Politely));
            let line = TheLine {
                address,
                up,
                towards_the_machine,
                restarts,
                servers: Arc::new(Mutex::new(Vec::new())),
            };
            let far_end = Arc::clone(far_end);
            let up = line.up.clone();
            let towards = line.towards_the_machine.clone();
            let restarts = line.restarts.clone();
            let servers = Arc::clone(&line.servers);
            tokio::spawn(async move {
                while let Ok((desk, _)) = listener.accept().await {
                    let (server, machine) = far_end.one_connection();
                    if let Ok(mut held) = servers.lock() {
                        held.push(server);
                    }
                    // **A socket closed with no linger is closed with a
                    // reset**, which is how the desk's half is ended when the
                    // machine comes back knowing nothing of it. Closed
                    // politely, its close is said first, as before.
                    let _ = desk.set_zero_linger();
                    let (desk_reads, desk_writes) = tokio::io::split(desk);
                    let (machine_reads, machine_writes) = machine.into_split();
                    let born = restarts.borrow().0;
                    tokio::spawn(one_direction(
                        desk_reads,
                        machine_writes,
                        towards.subscribe(),
                        (restarts.subscribe(), born),
                    ));
                    tokio::spawn(one_direction(
                        machine_reads,
                        desk_writes,
                        up.subscribe(),
                        (restarts.subscribe(), born),
                    ));
                }
            });
            line
        }

        /// **The machine restarts under the desk's connection**: it goes
        /// silent for `down_for` — the reboot — and comes back, ending every
        /// connection made before it the way `ends` says. **The line is up
        /// again at once**, so the next connection the desk makes reaches the
        /// machine that came back. Added 2026-10-02, the fixes before 0.1.8,
        /// fault 1.
        async fn the_machine_restarts(&self, down_for: Duration, ends: HowTheOldConnectionEnds) {
            self.goes_silent().await;
            tokio::time::sleep(down_for).await;
            let next = self.restarts.borrow().0 + 1;
            self.restarts.send_replace((next, ends));
            tokio::time::sleep(Duration::from_millis(50)).await;
            self.comes_back();
        }

        /// **The desk's half goes first, and the machine's a moment later** —
        /// as on a real line, where what was already on its way back still
        /// arrives after the desk's packets stop getting through. Those last
        /// answers open the machine's window once more, so the desk is caught
        /// sending into a line that takes nothing.
        async fn goes_silent(&self) {
            let _ = self.towards_the_machine.send(false);
            tokio::time::sleep(Duration::from_millis(300)).await;
            let _ = self.up.send(false);
        }

        /// **Written whether or not a connection is carried now** — a
        /// restart leaves none, and the next one must find the line up.
        fn comes_back(&self) {
            self.up.send_replace(true);
            self.towards_the_machine.send_replace(true);
        }
    }

    /// Waits until the line is in the state asked for.
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

    /// Waits until the machine has restarted since a connection was made, and
    /// answers with how that connection ends.
    async fn until_the_machine_restarts(
        restarts: &mut watch::Receiver<(u64, HowTheOldConnectionEnds)>,
        born: u64,
    ) -> HowTheOldConnectionEnds {
        loop {
            let (now, ends) = *restarts.borrow_and_update();
            if now != born {
                return ends;
            }
            if restarts.changed().await.is_err() {
                std::future::pending::<()>().await;
            }
        }
    }

    /// One direction of one connection, carried while the line is up — and
    /// ended the way the machine ends it, where it restarts.
    async fn one_direction<R, W>(
        mut from: R,
        mut to: W,
        mut up: watch::Receiver<bool>,
        (mut restarts, born): (watch::Receiver<(u64, HowTheOldConnectionEnds)>, u64),
    ) where
        R: tokio::io::AsyncRead + Unpin,
        W: tokio::io::AsyncWrite + Unpin,
    {
        let mut buffer = vec![0u8; 64 * 1024];
        let carried = async {
            loop {
                until_the_line_is(&mut up, true).await;
                // **A read is given up the moment the line goes silent**, and a
                // socket's read keeps what it has not handed over, so nothing
                // is lost and nothing is read while the line is silent.
                let read = tokio::select! {
                    read = from.read(&mut buffer) => read,
                    () = until_the_line_is(&mut up, false) => continue,
                };
                let got = match read {
                    Ok(0) | Err(_) => return,
                    Ok(got) => got,
                };
                until_the_line_is(&mut up, true).await;
                if to.write_all(&buffer[..got]).await.is_err() {
                    return;
                }
            }
        };
        let ends = tokio::select! {
            () = carried => HowTheOldConnectionEnds::Politely,
            ends = until_the_machine_restarts(&mut restarts, born) => ends,
        };
        // **Said politely, the close goes on to the other end**; ended with a
        // reset, nothing is said, and the desk's socket — dropped with no
        // linger once both directions have let it go — answers with a reset.
        if ends == HowTheOldConnectionEnds::Politely {
            let _ = to.shutdown().await;
        }
    }

    // ------------------------------------------------------------ the desk side

    struct Proceeds;
    impl DecideHostTrust for Proceeds {
        fn decide(&mut self, _offered: &[u8]) -> HostTrust {
            HostTrust::Proceed
        }
    }

    async fn a_connection_through(
        line: &TheLine,
        far_end: &TheFarEnd,
        figures: Figures,
    ) -> Connection {
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
            figures,
            Box::new(Proceeds),
        )
        .await
        .expect("the connection opens through the line")
    }

    /// **The point, lending one real connection**, with no additional one.
    fn lent(
        sessions: &ThisServersSessions,
        mut connection: Connection,
    ) -> tokio::task::JoinHandle<(Ending, Instant)> {
        let sessions = sessions.clone();
        tokio::spawn(async move {
            let none: TheAdditionalConnection<Connection> = Arc::new(|| Box::pin(async { None }));
            let ending = lend_until_it_ends(&sessions, &mut connection, &none).await;
            (ending, Instant::now())
        })
    }

    /// **One file's bytes, moving**: one file opened on a lent channel and
    /// written to without end, the writes pipelined the way the byte loop's are
    /// — sixty-four of the protocol's 32 KiB writes in flight, which is the
    /// machine's whole window. **The file itself goes round in its first 64
    /// MiB**, so a run that carries on at the loopback's pace does not fill the
    /// desk's disk; `acknowledged` adds every byte the machine answered.
    fn an_upload_moving(
        sessions: &ThisServersSessions,
        into: PathBuf,
        acknowledged: Arc<AtomicU64>,
    ) -> tokio::task::JoinHandle<Instant> {
        let sessions = sessions.clone();
        tokio::spawn(async move {
            let ended = async {
                let channel = sessions.a_transfer_channel().await.ok()?;
                let mut subsystem = Subsystem::negotiate(channel).await.ok()?;
                let path = into.to_string_lossy().into_owned();
                let handle = subsystem.create_exclusively(path.as_bytes()).await.ok()?;
                let chunk = vec![7u8; 32 * 1024];
                let mut sent = 0u64;
                loop {
                    let offset = sent % (64 * 1024 * 1024);
                    subsystem.send_a_write(&handle, offset, &chunk).await.ok()?;
                    sent += chunk.len() as u64;
                    if subsystem.writes_in_flight() >= 64 {
                        subsystem.settle_the_writes().await.ok()?;
                        acknowledged.fetch_add(64 * chunk.len() as u64, Ordering::SeqCst);
                    }
                }
                #[allow(unreachable_code)]
                Some(())
            };
            let _: Option<()> = ended.await;
            Instant::now()
        })
    }

    // ------------------------------------------------------------- the silence

    /// **How one silence went**, in seconds from the moment the line went
    /// silent.
    #[derive(Debug, Default)]
    struct WhatTheSilenceCameTo {
        /// When the lending answered, and with what — the moment the row
        /// stops reading *connected* (`Servers` K6).
        the_lending_answered: Option<(f64, Ending)>,
        /// When the upload's own wait ended.
        the_upload_ended: Option<f64>,
        /// When the Files page's reading answered, and what it found.
        the_reading_answered: Option<(f64, WhatTheReadingFound)>,
        /// The bytes the machine had acknowledged when the line came back, and
        /// when the watch ended.
        acknowledged: (u64, u64),
    }

    /// **One silence under a moving upload**, with the Files page asking for a
    /// folder a second into it, watched for `watched_for`. Nothing where this
    /// system has no OpenSSH server to run.
    async fn one_silence(
        named: &str,
        comes_back_after: Option<Duration>,
        watched_for: Duration,
    ) -> Option<WhatTheSilenceCameTo> {
        let Some(far_end) = TheFarEnd::made(named) else {
            println!(
                "SKIPPED: no OpenSSH sshd, sftp-server and ssh-keygen this test can run on this \
                 system ({}).",
                std::env::consts::OS
            );
            return None;
        };
        let far_end = Arc::new(far_end);
        let line = TheLine::to(&far_end).await;
        let connection = a_connection_through(&line, &far_end, AT_THE_DESK).await;
        let sessions = ThisServersSessions::for_one_server();
        let lending = lent(&sessions, connection);
        while !sessions.a_connection_is_live() {
            tokio::task::yield_now().await;
        }
        // **The reading session is opened first**, as the server's page opens
        // it before anything is uploaded.
        let first = a_folder_is_read(&sessions, far_end.home.to_string_lossy().as_bytes()).await;
        assert!(
            matches!(first, WhatTheReadingFound::TheFolder { .. }),
            "the first reading reads: {first:?}"
        );

        // **Four files at once**, as an upload of a folder sends them (`File
        // Browser` O1's four upward channels) — each channel holding the
        // machine's whole window, so the desk has more to send than its socket
        // holds, which is when the transport's loop is caught waiting.
        let acknowledged = Arc::new(AtomicU64::new(0));
        let mut uploads: Vec<tokio::task::JoinHandle<Instant>> = (0..FILES_AT_ONCE)
            .map(|which| {
                an_upload_moving(
                    &sessions,
                    far_end.home.join(format!("large-{which}.bin")),
                    Arc::clone(&acknowledged),
                )
            })
            .collect();
        let began = Instant::now();
        while acknowledged.load(Ordering::SeqCst) < 16 * 1024 * 1024 {
            assert!(
                began.elapsed() < Duration::from_secs(30),
                "the upload moves"
            );
            tokio::time::sleep(Duration::from_millis(20)).await;
        }

        let silent = Instant::now();
        line.goes_silent().await;
        let seconds = |at: Instant| at.saturating_duration_since(silent).as_secs_f64();
        let mut came = WhatTheSilenceCameTo::default();
        let mut reading: Option<tokio::task::JoinHandle<(WhatTheReadingFound, Instant)>> = None;
        let mut lending = Some(lending);
        while silent.elapsed() < watched_for {
            if let Some(after) = comes_back_after {
                if came.acknowledged.0 == 0 && silent.elapsed() >= after {
                    came.acknowledged.0 = acknowledged.load(Ordering::SeqCst).max(1);
                    line.comes_back();
                }
            }
            if reading.is_none()
                && came.the_reading_answered.is_none()
                && silent.elapsed() >= Duration::from_secs(1)
            {
                let sessions = sessions.clone();
                let folder = far_end.home.to_string_lossy().into_owned();
                reading = Some(tokio::spawn(async move {
                    let found = a_folder_is_read(&sessions, folder.as_bytes()).await;
                    (found, Instant::now())
                }));
            }
            if lending.as_ref().is_some_and(|it| it.is_finished()) {
                let (ending, at) = lending.take().expect("held").await.expect("the lending");
                came.the_lending_answered = Some((seconds(at), ending));
            }
            if !uploads.is_empty() && uploads.iter().all(|it| it.is_finished()) {
                let mut last = silent;
                for upload in uploads.drain(..) {
                    last = last.max(upload.await.expect("the upload"));
                }
                came.the_upload_ended = Some(seconds(last));
            }
            if reading.as_ref().is_some_and(|it| it.is_finished()) {
                let (found, at) = reading.take().expect("held").await.expect("the reading");
                came.the_reading_answered = Some((seconds(at), found));
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        came.acknowledged.1 = acknowledged.load(Ordering::SeqCst);
        if let Some(lending) = lending {
            lending.abort();
        }
        for upload in uploads {
            upload.abort();
        }
        if let Some(reading) = reading {
            reading.abort();
        }
        println!("{named}: {came:?}");
        Some(came)
    }

    /// **The desk's figures**: not `Servers` K3's 15 seconds and 3, which would
    /// make each silence a minute; the mechanism is what is proved, and the
    /// figures are handed in exactly as K3's are. **The declaration window at
    /// these is 3 to 4 seconds** (`Figures::the_declaration_window`).
    const AT_THE_DESK: Figures = Figures {
        interval: Duration::from_secs(1),
        unanswered_before_the_break: 3,
    };

    /// How many files move at once under the silence.
    const FILES_AT_ONCE: usize = 1;

    /// How far past the window's close a declaration may land and still be
    /// the window's: the time the desk takes to notice, on a loaded machine.
    const NOTICING: Duration = Duration::from_millis(1_500);

    // -------------------------------------------------------------- the tests

    /// **A line gone silent under an upload is a break declared inside
    /// `Servers` K3's window, every time** — and the upload's wait and the
    /// Files page's reading both end with it. Step 10c's check-before-0.1.7 fix
    /// piece 1, run at every desk run.
    ///
    /// **The case it bites on.** The transport sends from the loop that keeps
    /// K3's count, and that loop waits until the socket takes everything it
    /// was given; a silent line never takes it, so no keepalive was sent or
    /// counted and **the break was never declared** — five silences in six at
    /// this desk before the fix, and the first of two identical cuts on the
    /// Hyperstack check, which then read *Idle · 1 connected* with no
    /// connection under it and stood on *Reading…* for six minutes. **Three
    /// silences, because the fault was one of timing**: whether the socket was
    /// full at the moment of the cut.
    ///
    /// **What the page must then say** is the connection's case and never the
    /// folder's (`File Browser` P2): what it says where no connection is live
    /// (`File Browser` L14's last sentence).
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_line_gone_silent_under_an_upload_is_a_break_inside_k3s_window_every_time() {
        let (opens, closes) = AT_THE_DESK.the_declaration_window();
        let mut missed = Vec::new();
        for silence in 1..=3 {
            let named = format!("silent-{silence}");
            let Some(came) = one_silence(&named, None, closes * 3).await else {
                return;
            };
            let in_the_window = |at: f64| {
                at >= opens.as_secs_f64() - 0.05 && at <= (closes + NOTICING).as_secs_f64()
            };
            match came.the_lending_answered {
                Some((at, Ending::TheKeepaliveWentUnanswered)) if in_the_window(at) => {}
                other => missed.push(format!(
                    "silence {silence}: the lending answered {other:?}, not the keepalive's \
                     ending inside {opens:?} to {closes:?}"
                )),
            }
            if !came.the_upload_ended.is_some_and(in_the_window) {
                missed.push(format!(
                    "silence {silence}: the upload's wait ended at {:?}",
                    came.the_upload_ended
                ));
            }
            match &came.the_reading_answered {
                Some((at, WhatTheReadingFound::NothingWasLent))
                    if *at <= (closes + NOTICING).as_secs_f64() =>
                {
                    let tab = the_tab_after_the_reading(
                        Default::default(),
                        "a server",
                        far_end_folder(),
                        None,
                        WhatTheReadingFound::NothingWasLent,
                        Some(ObservedValue::Reconnecting),
                        false,
                    );
                    if !matches!(tab.listing, TheListing::NothingHasBeenListed { .. }) {
                        missed.push(format!(
                            "silence {silence}: the page drew {:?}",
                            tab.listing
                        ));
                    }
                }
                other => missed.push(format!(
                    "silence {silence}: the Files page's reading answered {other:?}, not \
                     *nothing lent* by the window's close"
                )),
            }
        }
        assert!(missed.is_empty(), "{missed:#?}");
    }

    /// **A line that comes back inside K3's window is no break, and the upload
    /// carries on over the same connection.** What the watch added must not
    /// declare a break the transport's own count would not: the count is
    /// measured from the last thing the line carried, either way.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_line_back_inside_k3s_window_is_no_break_and_the_upload_carries_on() {
        let (opens, closes) = AT_THE_DESK.the_declaration_window();
        let back_after = opens / 2;
        let Some(came) = one_silence("back", Some(back_after), closes * 2 + back_after).await
        else {
            return;
        };
        assert!(
            came.the_lending_answered.is_none(),
            "a break was declared: {came:?}"
        );
        assert!(
            came.the_upload_ended.is_none(),
            "the upload ended: {came:?}"
        );
        assert!(
            came.acknowledged.1 > came.acknowledged.0,
            "the upload did not carry on after the line came back: {came:?}"
        );
    }

    /// A folder for a composition that reads no folder.
    fn far_end_folder() -> &'static [u8] {
        b"/home/ubuntu"
    }

    // ------------------------------------------------------------ the restart

    /// **How one restart went**, in seconds from the moment the machine came
    /// back.
    #[derive(Debug, Default)]
    struct WhatTheRestartCameTo {
        /// When the lending answered, and with what — the moment the row stops
        /// reading *connected* (`Servers` K6) and `Servers`' repair, with the
        /// restart mark's policy (`Servers` M4), begins.
        the_lending_answered: Option<(f64, Ending)>,
        /// When the Files page's reading, asked just after the machine came
        /// back, answered, and what it found.
        the_reading_answered: Option<(f64, WhatTheReadingFound)>,
        /// **Whether a fresh connection — the repair's next attempt — reached
        /// the machine that came back** and read its folder through the point.
        a_fresh_connection_read_the_folder: bool,
    }

    /// **The machine restarts under a live connection**, the way `Server
    /// Setup`'s restart step makes it: the line silent for a moment, then the
    /// machine back, ending the old connection as `ends` says. **Watched for
    /// three of K3's windows from the moment it came back** — long past the
    /// moment any break should have been declared. Added 2026-10-02, the fixes
    /// before 0.1.8, fault 1.
    async fn one_restart(named: &str, ends: HowTheOldConnectionEnds) -> Option<WhatTheRestartCameTo> {
        let Some(far_end) = TheFarEnd::made(named) else {
            println!(
                "SKIPPED: no OpenSSH sshd, sftp-server and ssh-keygen this test can run on this \
                 system ({}).",
                std::env::consts::OS
            );
            return None;
        };
        let far_end = Arc::new(far_end);
        let line = TheLine::to(&far_end).await;
        let connection = a_connection_through(&line, &far_end, AT_THE_DESK).await;
        let sessions = ThisServersSessions::for_one_server();
        let lending = lent(&sessions, connection);
        while !sessions.a_connection_is_live() {
            tokio::task::yield_now().await;
        }
        let home = far_end.home.to_string_lossy().into_owned();
        let first = a_folder_is_read(&sessions, home.as_bytes()).await;
        assert!(
            matches!(first, WhatTheReadingFound::TheFolder { .. }),
            "the first reading reads: {first:?}"
        );

        // **Down for less than K3's window opens**, as the machine on the
        // check was back within half a minute of a 45-to-60-second window:
        // neither the keepalive count nor the line's own watch has anything
        // to declare before the machine is back.
        let (opens, closes) = AT_THE_DESK.the_declaration_window();
        line.the_machine_restarts(opens / 2, ends).await;
        let back = Instant::now();
        let seconds = |at: Instant| at.saturating_duration_since(back).as_secs_f64();

        let mut came = WhatTheRestartCameTo::default();
        let mut lending = Some(lending);
        let mut reading: Option<tokio::task::JoinHandle<(WhatTheReadingFound, Instant)>> = None;
        while back.elapsed() < closes * 3 {
            if reading.is_none()
                && came.the_reading_answered.is_none()
                && back.elapsed() >= Duration::from_millis(200)
            {
                let sessions = sessions.clone();
                let home = home.clone();
                reading = Some(tokio::spawn(async move {
                    let found = a_folder_is_read(&sessions, home.as_bytes()).await;
                    (found, Instant::now())
                }));
            }
            if lending.as_ref().is_some_and(|it| it.is_finished()) {
                let (ending, at) = lending.take().expect("held").await.expect("the lending");
                came.the_lending_answered = Some((seconds(at), ending));
            }
            if reading.as_ref().is_some_and(|it| it.is_finished()) {
                let (found, at) = reading.take().expect("held").await.expect("the reading");
                came.the_reading_answered = Some((seconds(at), found));
            }
            if came.the_lending_answered.is_some() && came.the_reading_answered.is_some() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        if let Some(lending) = lending {
            lending.abort();
        }
        if let Some(reading) = reading {
            reading.abort();
        }

        // **The repair's next attempt**: a fresh connection to the machine that
        // came back, lent through a fresh point the way a repaired period's
        // is, reading the same folder.
        let again = tokio::time::timeout(Duration::from_secs(20), async {
            let connection = a_connection_through(&line, &far_end, AT_THE_DESK).await;
            let sessions = ThisServersSessions::for_one_server();
            let lending = lent(&sessions, connection);
            while !sessions.a_connection_is_live() {
                tokio::task::yield_now().await;
            }
            let found = a_folder_is_read(&sessions, home.as_bytes()).await;
            lending.abort();
            matches!(found, WhatTheReadingFound::TheFolder { .. })
        })
        .await;
        came.a_fresh_connection_read_the_folder = again.unwrap_or(false);
        println!("{named}: {came:?}");
        Some(came)
    }

    /// What a restart must come to, whichever way the old connection ended:
    /// **the break declared within the desk's noticing of the machine coming
    /// back, the Files page told nothing is lent, and the machine reached
    /// again.**
    fn what_a_restart_must_come_to(came: &WhatTheRestartCameTo) -> Vec<String> {
        let mut missed = Vec::new();
        let noticed = NOTICING.as_secs_f64();
        match came.the_lending_answered {
            Some((at, ending)) if ending.is_a_break() && at <= noticed => {}
            other => missed.push(format!(
                "the lending answered {other:?}, not a break within {noticed} s of the machine \
                 coming back — the row would read *connected* with no connection under it"
            )),
        }
        match &came.the_reading_answered {
            Some((at, WhatTheReadingFound::NothingWasLent)) if *at <= noticed => {}
            other => missed.push(format!(
                "the Files page's reading answered {other:?}, not *nothing lent* within \
                 {noticed} s — the page would read *Reading* for as long as the run lasts"
            )),
        }
        if !came.a_fresh_connection_read_the_folder {
            missed.push("a fresh connection did not reach the machine that came back".to_string());
        }
        missed
    }

    /// **A machine that comes back from a restart knowing nothing of the old
    /// connection answers it with a reset, and that is a break declared at
    /// once.** The fixes before 0.1.8, fault 1, out of the check before 0.1.8
    /// on Hyperstack, where after the setup's own restart **the copy never
    /// heard its connection had gone** — the row read *Idle · 1 connected*,
    /// the setup *waiting for it to restart* and Files *Reading /* for eleven
    /// minutes.
    ///
    /// **It held before the fix and it holds after it**: the first thing
    /// suspected was that a reset made the transport leave without saying the
    /// connection ended, and this is the test that said it does not. It stays
    /// so that both ways a restart can end the old connection — this one and
    /// [`a_machine_back_from_a_restart_that_closed_politely_is_a_break_at_once_and_is_reached_again`]
    /// — are run at every desk run, with the machine reached again after each.
    /// The case that bites is
    /// [`a_restart_under_a_stopped_transport_is_heard_when_the_break_is_declared`].
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_machine_back_from_a_restart_with_a_reset_is_a_break_at_once_and_is_reached_again() {
        let Some(came) = one_restart("restart-reset", HowTheOldConnectionEnds::WithAReset).await
        else {
            return;
        };
        let missed = what_a_restart_must_come_to(&came);
        assert!(missed.is_empty(), "{missed:#?}");
    }

    /// **How the restart went over a transport that had stopped turning**, in
    /// seconds from the moment it stopped.
    #[derive(Debug, Default)]
    struct WhatTheStoppedTransportCameTo {
        the_lending_answered: Option<(f64, Ending)>,
        the_reading_answered: Option<(f64, WhatTheReadingFound)>,
        /// **Whether Smoozy's own tear-down of the connection returned** within
        /// five seconds — `None` where the lending never answered, so nothing
        /// was torn down.
        the_tear_down_returned: Option<bool>,
        a_fresh_connection_read_the_folder: bool,
    }

    /// **The machine restarts while the point is part-way through a reading
    /// and the transport under it has stopped turning.**
    ///
    /// **What stops it is real and is the transport's own**: every channel on a
    /// connection is fed from the one loop that reads the socket, and that loop
    /// waits whenever one channel's queue is full — a hundred messages
    /// (`russh` 0.63.1, `client/encrypted.rs`, the channel-data arm's
    /// `chan.send(…).await`). Here one transfer channel asks for far more than
    /// that and is never read, which is the plainest way to make it happen.
    /// **From then on nothing on the connection moves**: the Files reading
    /// asked next waits on an answer the loop will never hand it, and so does
    /// the point, which is doing that reading.
    ///
    /// **What the transport still does is declare the break** — `Servers` K3a's watch
    /// sees the line carry nothing for the whole window. **What it came to
    /// before the fix is the check's picture**: the declaration was written,
    /// and nobody heard it, because the point hears the connection's end only
    /// between two pieces of work and this piece never ended.
    async fn a_restart_over_a_stopped_transport(named: &str) -> Option<WhatTheStoppedTransportCameTo> {
        let Some(far_end) = TheFarEnd::made(named) else {
            println!(
                "SKIPPED: no OpenSSH sshd, sftp-server and ssh-keygen this test can run on this \
                 system ({}).",
                std::env::consts::OS
            );
            return None;
        };
        let far_end = Arc::new(far_end);
        let line = TheLine::to(&far_end).await;
        let connection = a_connection_through(&line, &far_end, AT_THE_DESK).await;
        let sessions = ThisServersSessions::for_one_server();
        // **The lending, and then Smoozy's own tear-down after it** — which is
        // what `Servers`' driving does with a connection whose break it has
        // heard, before the row can stop reading *connected*.
        let (torn_down, tear_down_returned) = tokio::sync::oneshot::channel();
        let lending = {
            let sessions = sessions.clone();
            let mut connection = connection;
            tokio::spawn(async move {
                let none: TheAdditionalConnection<Connection> =
                    Arc::new(|| Box::pin(async { None }));
                let ending = lend_until_it_ends(&sessions, &mut connection, &none).await;
                let at = Instant::now();
                let returned =
                    tokio::time::timeout(Duration::from_secs(5), connection.tear_down()).await;
                let _ = torn_down.send(returned.is_ok());
                (ending, at)
            })
        };
        while !sessions.a_connection_is_live() {
            tokio::task::yield_now().await;
        }
        let home = far_end.home.to_string_lossy().into_owned();
        let first = a_folder_is_read(&sessions, home.as_bytes()).await;
        assert!(
            matches!(first, WhatTheReadingFound::TheFolder { .. }),
            "the first reading reads: {first:?}"
        );

        // **One channel asks for 12 MiB in 32 KiB pieces and reads none of
        // it.**
        let large = far_end.home.join("large-to-read.bin");
        std::fs::write(&large, vec![7u8; 12 * 1024 * 1024]).expect("a file to read");
        let channel = sessions.a_transfer_channel().await.expect("a transfer channel");
        let mut unread = Subsystem::negotiate(channel).await.expect("the subsystem");
        let handle = unread
            .open_to_read(large.to_string_lossy().as_bytes())
            .await
            .expect("the file opens");
        for piece in 0..384u64 {
            unread
                .send_a_read(&handle, piece * 32 * 1024, 32 * 1024)
                .await
                .expect("a read goes out");
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
        // **And more asked of it after the loop has stopped**, which waits in
        // the transport's own queue of ten and then fills it — so anything else
        // that asks the stopped loop for something waits too, Smoozy's own
        // tear-down among them, as on a machine where readings and commands
        // went on being asked while nothing answered.
        let still_asking = tokio::spawn(async move {
            for piece in 384..416u64 {
                if unread.send_a_read(&handle, piece * 32 * 1024, 32 * 1024).await.is_err() {
                    break;
                }
            }
            unread
        });
        tokio::time::sleep(Duration::from_millis(200)).await;
        let stopped = Instant::now();
        let seconds = |at: Instant| at.saturating_duration_since(stopped).as_secs_f64();

        // **The Files page asks for the folder**, as it did on the check.
        let reading = {
            let sessions = sessions.clone();
            let home = home.clone();
            tokio::spawn(async move {
                let found = a_folder_is_read(&sessions, home.as_bytes()).await;
                (found, Instant::now())
            })
        };
        let (opens, closes) = AT_THE_DESK.the_declaration_window();
        line.the_machine_restarts(opens / 2, HowTheOldConnectionEnds::WithAReset)
            .await;

        let mut came = WhatTheStoppedTransportCameTo::default();
        let mut lending = Some(lending);
        let mut reading = Some(reading);
        while stopped.elapsed() < closes * 3 {
            if lending.as_ref().is_some_and(|it| it.is_finished()) {
                let (ending, at) = lending.take().expect("held").await.expect("the lending");
                came.the_lending_answered = Some((seconds(at), ending));
            }
            if reading.as_ref().is_some_and(|it| it.is_finished()) {
                let (found, at) = reading.take().expect("held").await.expect("the reading");
                came.the_reading_answered = Some((seconds(at), found));
            }
            if lending.is_none() && reading.is_none() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        if let Some(lending) = lending {
            lending.abort();
        }
        if let Some(reading) = reading {
            reading.abort();
        }
        still_asking.abort();
        came.the_tear_down_returned = tokio::time::timeout(Duration::from_secs(10), tear_down_returned)
            .await
            .ok()
            .and_then(Result::ok);

        let again = tokio::time::timeout(Duration::from_secs(20), async {
            let connection = a_connection_through(&line, &far_end, AT_THE_DESK).await;
            let sessions = ThisServersSessions::for_one_server();
            let lending = lent(&sessions, connection);
            while !sessions.a_connection_is_live() {
                tokio::task::yield_now().await;
            }
            let found = a_folder_is_read(&sessions, home.as_bytes()).await;
            lending.abort();
            matches!(found, WhatTheReadingFound::TheFolder { .. })
        })
        .await;
        came.a_fresh_connection_read_the_folder = again.unwrap_or(false);
        println!("{named}: {came:?}");
        Some(came)
    }

    /// **A machine restarting while the point is part-way through a reading on
    /// a transport that has stopped turning: the break is heard the moment the
    /// transport declares it, the reading is told nothing was lent, and the
    /// machine is reached again.** The fixes before 0.1.8, fault 1 — the
    /// check's picture exactly: *Idle · 1 connected*, Files on *Reading /*, and
    /// a setup waiting on a restart that had long finished.
    ///
    /// **The two cases it bites on.** The point waited on the reading with no
    /// ear for the connection's end, so the declaration `Servers` K3a's watch wrote
    /// reached nobody, and the row stood on *connected* for as long as the run
    /// lasted. **And once the point did hear it**, Smoozy's own tear-down sent
    /// its goodbye through the stopped loop's full queue and waited there for
    /// good — and `Servers`' driving waits on the tear-down before the row
    /// stops reading *connected*.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_restart_under_a_stopped_transport_is_heard_when_the_break_is_declared() {
        let Some(came) = a_restart_over_a_stopped_transport("restart-stopped").await else {
            return;
        };
        let (_, closes) = AT_THE_DESK.the_declaration_window();
        let by = (closes + NOTICING).as_secs_f64();
        let mut missed = Vec::new();
        match came.the_lending_answered {
            Some((at, ending)) if ending.is_a_break() && at <= by => {}
            other => missed.push(format!(
                "the lending answered {other:?}, not a break within {by} s of the transport \
                 stopping — the row would read *connected* with nothing under it"
            )),
        }
        match &came.the_reading_answered {
            Some((at, WhatTheReadingFound::NothingWasLent)) if *at <= by => {}
            other => missed.push(format!(
                "the Files page's reading answered {other:?}, not *nothing lent* within {by} s"
            )),
        }
        if came.the_tear_down_returned != Some(true) {
            missed.push(format!(
                "Smoozy's own tear-down came to {:?}, not returned within five seconds — the \
                 driving waits on it before the row can stop reading *connected*",
                came.the_tear_down_returned
            ));
        }
        if !came.a_fresh_connection_read_the_folder {
            missed.push("a fresh connection did not reach the machine that came back".to_string());
        }
        assert!(missed.is_empty(), "{missed:#?}");
    }

    /// **A machine whose service closed the socket as it went down is a break
    /// at once too** — `Servers` K2's first bullet, and the route the earlier
    /// passes on Hyperstack took, reconnecting about half a minute after the
    /// boot. Held before the fix and after it; it is here so the two ways a
    /// restart ends the old connection are both run at every desk run.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_machine_back_from_a_restart_that_closed_politely_is_a_break_at_once_and_is_reached_again(
    ) {
        let Some(came) = one_restart("restart-polite", HowTheOldConnectionEnds::Politely).await
        else {
            return;
        };
        let missed = what_a_restart_must_come_to(&came);
        assert!(missed.is_empty(), "{missed:#?}");
    }
}

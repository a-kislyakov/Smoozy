//! **Files after disconnecting and connecting again, timed at the desk — at no
//! added delay and at a far server's.** Written 2026-10-25 for the Creator's
//! eighteenth notes, note 14: *"I disconnected from a server using the
//! Connected green button while in the file browser. Then I connected using the
//! same button again and the file browser didn't show me anything for quite a
//! while."*
//!
//! **What the far end is.** OpenSSH's own `sshd` in its one-connection mode
//! (`sshd -i`), one per connection, its file-transfer sessions OpenSSH's own
//! `sftp-server`, in a throwaway home — the far end
//! `tests/a_dead_line_under_the_first_half_of_the_acts.rs` stands up, with the
//! same stand-ins for a rental's own programs: `uname` naming Linux,
//! `systemd-detect-virt`, an `nvidia-smi` naming one card, and the machine's
//! entry point as a short Perl script carrying each framed request to this
//! test's own supervisor. **Between it and the desk is the line**, a relay this
//! test holds that hands every byte on a fixed time after it arrived, each way
//! — nought, or a far server's quarter of a second.
//!
//! **The desk side is the application's own**: `Servers`' own driving and
//! sequence over `Servers`' own transport, the per-server point
//! (`lend_until_it_ends`), and Files' own composition and reading
//! (`the_tab_from`, `a_folder_is_read`). **The wait on a live connection is the
//! application's shape** (`crate::servers::a_period`'s
//! `TheTransportThroughThePoint`): the lending is begun first, and the
//! hand-over is made the moment it has.
//!
//! **What is stood in for, and named as such.** `Servers` Q1 step 7, which the
//! hand-over starts, needs the application's record and shell; here it is three
//! asks of the machine's supervisor, one after the other, which is what step 7
//! makes on a quiet machine (the read-forward, the queue reading, and one
//! more). **The page is stood in for by what it asks, and when**: Files asks at
//! every post `web/a-server.js` hears — the server's observed value moving, its
//! condition moving, and the work column moving after step 7 — and, since this
//! note, at the hand-over too (`web/files.js`, `THE_CONNECTION_IS_LENT`). The
//! page's own answer to that event is proved under Node in
//! `src/surface/files.rs`.
//!
//! **Nothing of the Creator's is touched.** Keys, the server's configuration and
//! every file are inside a throwaway folder deleted with the test, and every
//! server is a child of the test that ends with it.
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
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::io::{AsRawFd, FromRawFd};
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant, SystemTime};

    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    use smoozy_desktop::servers::installation::{OverTheConnection, TheMachinesInstallationIdentity};
    use smoozy_desktop::servers::sessions::{lend_until_it_ends, TheAdditionalConnection, ThisServersSessions};
    use smoozy_desktop::surface::files::{a_folder_is_read, the_tab_from, WhatThereIs, WhatTheReadingFound};
    use smoozy_server_protocol::framing::read_one_request;
    use smoozy_server_protocol::transport::ssh::Connection;
    use smoozy_server_protocol::transport::{Credential, DecideHostTrust, Ending, Endpoint, OpenFailure};
    use smoozy_servers::acting::{
        OpenConnections, SupplyTheCredential, TheActingHalf, TheServerBeingConnectedTo, Wait,
        WhatServersPublishes,
    };
    use smoozy_servers::condition::ObservedValue;
    use smoozy_servers::driving::{APeriod, TheDesktopsSide};
    use smoozy_servers::hardware::HardwareReading;
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use smoozy_servers::key_moments::{TheTwoAnswers, WhoStartedIt};
    use smoozy_servers::period::{OpeningAct, PeriodEnding, WhoTookIt};
    use smoozy_servers::repair::{NextAttemptDue, PermanentRefusal, WhatTheRecordSays};
    use smoozy_servers::store::{RememberedHostKey, ServerRow};
    use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};

    // ------------------------------------------------------------- the far end

    /// Where OpenSSH's own server and file-transfer server are, and Perl.
    fn what_the_far_end_needs() -> Option<(PathBuf, PathBuf, PathBuf)> {
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
        let perl = ["/usr/bin/perl", "/usr/local/bin/perl"]
            .iter()
            .map(PathBuf::from)
            .find(|path| path.exists())?;
        Some((sshd, sftp, perl))
    }

    /// A throwaway folder that takes itself away. **Short**, because a Unix
    /// socket's path has a short limit and one lives in here.
    struct Scratch(PathBuf);

    static SCRATCHES: AtomicUsize = AtomicUsize::new(0);

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let path = PathBuf::from("/tmp").join(format!(
                "smz-fb-{named}-{}-{}",
                std::process::id(),
                SCRATCHES.fetch_add(1, Ordering::SeqCst)
            ));
            let _ = std::fs::remove_dir_all(&path);
            std::fs::create_dir_all(&path).expect("the scratch folder");
            Scratch(std::fs::canonicalize(&path).expect("the scratch folder is there"))
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn a_program(at: &Path, text: &str) {
        std::fs::write(at, text).expect("a program is written");
        std::fs::set_permissions(at, std::fs::Permissions::from_mode(0o755)).expect("it runs");
    }

    /// **The machine the test stands up**: OpenSSH's own server, ready to take
    /// one connection at a time, its sessions in a throwaway home; the
    /// machine's package area inside that home; the stand-ins; and the socket
    /// the entry point reaches this test's supervisor on.
    struct TheFarEnd {
        sshd: PathBuf,
        config: PathBuf,
        client_key: PathBuf,
        home: PathBuf,
        package_area: PathBuf,
        the_supervisors_socket: PathBuf,
        _scratch: Scratch,
    }

    impl TheFarEnd {
        fn made(named: &str) -> Option<TheFarEnd> {
            let (sshd, sftp, perl) = what_the_far_end_needs()?;
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
            let package_area = home.join("area");
            std::fs::create_dir_all(&package_area).ok()?;
            // **What Files lists**: a folder of a render's frames and a scene.
            let renders = home.join("renders");
            std::fs::create_dir_all(&renders).ok()?;
            for n in 0..40 {
                std::fs::write(renders.join(format!("frame_{n:04}.png")), [n as u8; 64]).ok()?;
            }
            std::fs::write(home.join("scene.blend"), b"BLENDER-v405").ok()?;
            let the_supervisors_socket = folder.join("s.sock");

            // **`uname` naming Linux**, which `Servers` Q13's check reads; this
            // system's own names itself.
            a_program(&bin.join("uname"), "#!/bin/sh\necho Linux\n");
            // **`systemd-detect-virt`**, which the hardware reading asks
            // (`Rented-server runtime` K1's table) — not a container.
            a_program(&bin.join("systemd-detect-virt"), "#!/bin/sh\necho none\nexit 1\n");
            // **`nvidia-smi`, naming one card**, so the hardware reading asks
            // every question it asks of a rental with a card.
            a_program(
                &bin.join("nvidia-smi"),
                "#!/bin/sh\ncase \"$*\" in\n\
                 *uuid*) echo 'GPU-0000, 00000000:01:00.0, NVIDIA GeForce RTX 4090, 24564 MiB' ;;\n\
                 *driver_version*) echo '550.54.14' ;;\n\
                 *) echo 'NVIDIA-SMI 550.54.14   Driver Version: 550.54.14   CUDA Version: 12.4' ;;\n\
                 esac\n",
            );
            // **The machine's entry point** (`Rented-server runtime` G12),
            // reached through the planted link as on a rental: it carries the
            // one framed request to this test's supervisor and writes back what
            // that answers.
            a_program(
                &package_area.join("smoozy-server"),
                &format!(
                    r#"#!{perl}
use strict;
use Socket;
use IO::Socket::UNIX;
binmode STDIN;
binmode STDOUT;
my $frame = '';
sub take {{
    my ($wanted) = @_;
    my $start = length($frame);
    while (length($frame) - $start < $wanted) {{
        my $got = sysread(STDIN, $frame, $wanted - (length($frame) - $start), length($frame));
        exit 3 unless $got;
    }}
}}
take(4);
take(unpack('N', $frame));
my $supervisor = IO::Socket::UNIX->new(Type => SOCK_STREAM(), Peer => '{socket}') or exit 4;
binmode $supervisor;
syswrite($supervisor, $frame);
shutdown($supervisor, 1);
my $answer = '';
while (sysread($supervisor, my $piece, 65536)) {{ $answer .= $piece; }}
syswrite(STDOUT, $answer);
exit 0;
"#,
                    perl = perl.display(),
                    socket = the_supervisors_socket.display(),
                ),
            );

            let config = folder.join("sshd_config");
            std::fs::write(
                &config,
                format!(
                    "HostKey {}\nAuthorizedKeysFile {}\nPidFile none\nUsePAM no\nStrictModes no\n\
                     PasswordAuthentication no\nKbdInteractiveAuthentication no\n\
                     Subsystem sftp {} -d {}\nSetEnv HOME={} PATH={}:/usr/bin:/bin\n",
                    folder.join("host").display(),
                    folder.join("authorized_keys").display(),
                    sftp.display(),
                    home.display(),
                    home.display(),
                    bin.display(),
                ),
            )
            .ok()?;
            let checked = std::process::Command::new(&sshd).args(["-t", "-f"]).arg(&config).output().ok()?;
            if !checked.status.success() {
                println!("sshd will not run here: {}", String::from_utf8_lossy(&checked.stderr).trim());
                return None;
            }
            Some(TheFarEnd {
                sshd,
                config,
                client_key: folder.join("client"),
                home,
                package_area,
                the_supervisors_socket,
                _scratch: scratch,
            })
        }

        /// One connection's server: `sshd -i` on one end of a loopback pair.
        fn one_connection(&self) -> (std::process::Child, tokio::net::TcpStream) {
            let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("a listener");
            let ours = std::net::TcpStream::connect(listener.local_addr().expect("an address")).expect("the pair connects");
            let (theirs, _) = listener.accept().expect("the pair is accepted");
            ours.set_nonblocking(true).expect("the end we drive");
            let reading = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let writing = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let child = std::process::Command::new(&self.sshd)
                .args(["-i", "-e", "-f"])
                .arg(&self.config)
                .stdin(reading)
                .stdout(writing)
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("OpenSSH's own server starts");
            drop(theirs);
            (child, tokio::net::TcpStream::from_std(ours).expect("the end we drive"))
        }
    }

    fn copy_of(descriptor: std::os::unix::io::RawFd) -> std::os::unix::io::RawFd {
        let copy = unsafe { libc::dup(descriptor) };
        assert!(copy >= 0, "the descriptor could not be copied");
        copy
    }

    /// **This test's supervisor, standing in for the machine's own**: every
    /// request is answered *not a request this supervisor answers*, which is
    /// enough for an ask to have gone to the machine and come back.
    struct TheSupervisor {
        asked: Arc<AtomicUsize>,
        task: tokio::task::JoinHandle<()>,
    }

    impl TheSupervisor {
        fn at(far_end: &TheFarEnd) -> TheSupervisor {
            let socket = &far_end.the_supervisors_socket;
            let _ = std::fs::remove_file(socket);
            let listener = tokio::net::UnixListener::bind(socket).expect("the supervisor's socket");
            let asked = Arc::new(AtomicUsize::new(0));
            let task = tokio::spawn({
                let asked = Arc::clone(&asked);
                async move {
                    while let Ok((mut stream, _)) = listener.accept().await {
                        let mut frame = Vec::new();
                        if stream.read_to_end(&mut frame).await.is_err() {
                            continue;
                        }
                        let Ok(request) = read_one_request(&mut &frame[..]) else {
                            continue;
                        };
                        asked.fetch_add(1, Ordering::SeqCst);
                        let bytes = smoozy_server_protocol::framing::write_result_frame(
                            &smoozy_server_protocol::framing::Answered::to(
                                &request,
                                smoozy_server_protocol::framing::THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                                "0.1.0",
                            )
                            .saying_it_does_not_answer_that_request(&request.name),
                        );
                        let _ = stream.write_all(&bytes).await;
                        let _ = stream.shutdown().await;
                    }
                }
            });
            TheSupervisor { asked, task }
        }
    }

    impl Drop for TheSupervisor {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    // ---------------------------------------------------------------- the line

    /// **The line between the desk and the machine**: every byte handed on
    /// `late` after it arrived, each way, as a far server's line does. Nothing
    /// else about it is slow, so it stands in for distance and not for a thin
    /// line.
    struct TheLine {
        port: u16,
        servers: Arc<Mutex<Vec<std::process::Child>>>,
        task: tokio::task::JoinHandle<()>,
    }

    impl Drop for TheLine {
        fn drop(&mut self) {
            self.task.abort();
            if let Ok(mut servers) = self.servers.lock() {
                for server in servers.iter_mut() {
                    let _ = server.kill();
                    let _ = server.wait();
                }
            }
        }
    }

    impl TheLine {
        async fn to(far_end: &Arc<TheFarEnd>, late: Duration) -> TheLine {
            let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0)).await.expect("a listener");
            let port = listener.local_addr().expect("an address").port();
            let servers: Arc<Mutex<Vec<std::process::Child>>> = Arc::default();
            let task = tokio::spawn({
                let (far_end, servers) = (Arc::clone(far_end), Arc::clone(&servers));
                async move {
                    while let Ok((desk, _)) = listener.accept().await {
                        let (server, machine) = far_end.one_connection();
                        servers.lock().expect("the servers").push(server);
                        let (desk_reads, desk_writes) = desk.into_split();
                        let (machine_reads, machine_writes) = machine.into_split();
                        tokio::spawn(carried_late(desk_reads, machine_writes, late));
                        tokio::spawn(carried_late(machine_reads, desk_writes, late));
                    }
                }
            });
            TheLine { port, servers, task }
        }
    }

    /// **One direction of one connection, every byte handed on `late` after
    /// it arrived.** What arrives meanwhile is read and held, so the line's
    /// delay is paid once per crossing and never once per piece.
    async fn carried_late<R, W>(mut from: R, mut to: W, late: Duration)
    where
        R: tokio::io::AsyncRead + Unpin + Send + 'static,
        W: tokio::io::AsyncWrite + Unpin,
    {
        let (tell, mut told) = tokio::sync::mpsc::unbounded_channel::<(Instant, Vec<u8>)>();
        let reading = tokio::spawn(async move {
            let mut buffer = vec![0u8; 64 * 1024];
            loop {
                match from.read(&mut buffer).await {
                    Ok(0) | Err(_) => break,
                    Ok(got) => {
                        if tell.send((Instant::now() + late, buffer[..got].to_vec())).is_err() {
                            break;
                        }
                    }
                }
            }
        });
        while let Some((due, bytes)) = told.recv().await {
            tokio::time::sleep_until(tokio::time::Instant::from_std(due)).await;
            if to.write_all(&bytes).await.is_err() {
                break;
            }
        }
        let _ = to.shutdown().await;
        reading.abort();
    }

    // ------------------------------------------------------------ the desk side

    /// **What each moment of one connect came to**, measured from the press.
    #[derive(Debug, Default, Clone)]
    struct TheMoments {
        connected: Option<Instant>,
        /// Whether Files, asked as the row read *connected*, had a reading to
        /// follow — a live point to read on.
        a_reading_followed_at_connected: Option<bool>,
        lent: Option<Instant>,
        step_seven_done: Option<Instant>,
        listed: Option<Instant>,
        listed_names: usize,
    }

    /// **Where the page's next ask falls**, which is the one thing that
    /// differs between before this note and after it.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TheNextAsk {
        /// **Before**: the next post `web/a-server.js` hears after the row read
        /// *connected* is the work column moving, after step 7.
        AtTheWorkMoving,
        /// **After**: `web/files.js` hears the hand-over itself.
        AtTheHandOver,
    }

    /// **`Servers` K1's own transport, and the wait on a live connection made
    /// the point's lending with the hand-over beside it** — the shape of
    /// `TheTransportThroughThePoint` in `src/servers/a_period.rs`: the lending
    /// polled first, so the point is live before the hand-over runs.
    struct TheOpener {
        sessions: ThisServersSessions,
        when_it_is_lent: Arc<dyn Fn(&ThisServersSessions) + Send + Sync>,
    }

    impl OpenConnections for TheOpener {
        type Connection = Connection;

        async fn open(
            &mut self,
            endpoint: Endpoint,
            credential: Credential,
            decide: Box<dyn DecideHostTrust>,
            deadline: Duration,
        ) -> Result<Connection, OpenFailure> {
            smoozy_servers::wiring::TheTransport.open(endpoint, credential, decide, deadline).await
        }

        fn ended(&mut self, connection: &Connection) -> Option<Ending> {
            connection.ended()
        }

        async fn wait_until_it_ends(&mut self, connection: &mut Connection) -> Ending {
            let none: TheAdditionalConnection<Connection> = Arc::new(|| Box::pin(async { None }));
            let TheOpener { sessions, when_it_is_lent } = self;
            let lending = std::pin::pin!(lend_until_it_ends(sessions, connection, &none));
            let handed_over = async {
                when_it_is_lent(sessions);
                std::future::pending::<std::convert::Infallible>().await
            };
            tokio::select! {
                biased;
                ending = lending => ending,
                never = handed_over => match never {},
            }
        }

        async fn tear_down(&mut self, connection: Connection) -> Ending {
            connection.tear_down().await
        }
    }

    /// `Keys`' end, standing in: the key is usable and handed over.
    struct TheKey(PathBuf);

    impl SupplyTheCredential for TheKey {
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
                private_key_file: self.0.clone(),
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

    /// **What `Servers` publishes**, and Files asked the way the page asks at
    /// each post of the server's observed value: composed from the value and
    /// whether the point is live at that moment (`the_files_tab`).
    struct WhatIsPublished {
        row: ServerRow,
        sessions: ThisServersSessions,
        moments: Arc<Mutex<TheMoments>>,
    }

    impl WhatServersPublishes for WhatIsPublished {
        fn the_observed_value(&mut self, server: &str, value: ObservedValue) {
            if value != ObservedValue::Connected {
                return;
            }
            let mut moments = self.moments.lock().expect("the moments");
            if moments.connected.is_some() {
                return;
            }
            moments.connected = Some(Instant::now());
            let tab = the_tab_from(
                WhatThereIs::TheServer(Box::new(self.row.clone()), Some(value)),
                server,
                None,
                None,
                self.sessions.a_connection_is_live(),
                false,
            );
            moments.a_reading_followed_at_connected = Some(tab.a_reading_follows);
        }
        fn the_next_attempt_due(&mut self, _server: &str, _reading: NextAttemptDue) {}
        fn raise_the_connection_condition(&mut self, _server: &str, _says: WhatTheRecordSays) {}
        fn withdraw_the_connection_condition(&mut self, _server: &str) {}
        fn the_changed_identity_warning(&mut self, _server: &str) {}
        fn record_the_scanned_host_key(&mut self, _server: &str, _key: RememberedHostKey) {}
    }

    /// The desktop's side: the row as it stands.
    struct TheDesk(ServerRow);

    impl TheDesktopsSide for TheDesk {
        fn the_row_now(&mut self, _server: &str) -> Option<ServerRow> {
            Some(self.0.clone())
        }
        fn the_connection_read(&mut self, _server: &str, _check: InstallationCheck, _hardware: Option<HardwareReading>) {}
    }

    /// The desktop's clock; nothing here waits on the repair's pauses.
    struct TheClock;
    impl Wait for TheClock {
        async fn until(&mut self, how_long: Duration) {
            tokio::time::sleep(how_long).await;
        }
        fn now(&mut self) -> SystemTime {
            SystemTime::now()
        }
    }

    fn the_user() -> String {
        std::env::var("USER")
            .or_else(|_| std::env::var("LOGNAME"))
            .expect("the user running the test is named")
    }

    fn the_row(line: &TheLine) -> ServerRow {
        ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer).expect("random bytes").as_text(),
            Host::what_the_user_typed("127.0.0.1").expect("a host"),
            Port::what_the_user_typed(line.port).expect("a port"),
            Username::what_the_user_typed(&the_user()).expect("a username"),
            KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        )
    }

    /// **Step 7, stood in for**: three asks of the machine's supervisor, one
    /// after the other, on the point — a quiet machine's read-forward, queue
    /// reading and one more.
    async fn step_seven_stood_in_for(sessions: &ThisServersSessions, package_area: &str) {
        for _ in 0..3 {
            let Some(request) = smoozy_desktop::servers::the_machines_work::the_queue_reading_request() else {
                return;
            };
            let _ = sessions.ask_the_supervisor(package_area, &request).await;
        }
    }

    /// **Files' one reading**, the way `the_folder_is_read` takes it, with the
    /// moment its rows came back.
    async fn files_reads(sessions: ThisServersSessions, folder: Vec<u8>, moments: Arc<Mutex<TheMoments>>) {
        let found = a_folder_is_read(&sessions, &folder).await;
        let mut moments = moments.lock().expect("the moments");
        if let WhatTheReadingFound::TheFolder { entries, .. } = found {
            moments.listed = Some(Instant::now());
            moments.listed_names = entries.len();
        }
    }

    /// **One period, driven by `Servers`' own driving over the real
    /// transport**: connected, lent, its stood-in step 7 and, where `next`
    /// says, Files' reading — and ended at the user's own disconnect once
    /// Files has listed or a minute has gone. The moments are measured from
    /// the press.
    async fn one_period(
        far_end: &Arc<TheFarEnd>,
        row: &ServerRow,
        next: Option<TheNextAsk>,
    ) -> (Instant, TheMoments) {
        let moments = Arc::new(Mutex::new(TheMoments::default()));
        let sessions = ThisServersSessions::for_one_server();
        let folder = far_end.home.join("renders").as_os_str().as_bytes().to_vec();
        let area = far_end.package_area.to_string_lossy().into_owned();
        let when_it_is_lent: Arc<dyn Fn(&ThisServersSessions) + Send + Sync> = {
            let (moments, folder) = (Arc::clone(&moments), folder.clone());
            Arc::new(move |sessions: &ThisServersSessions| {
                moments.lock().expect("the moments").lent = Some(Instant::now());
                if next == Some(TheNextAsk::AtTheHandOver) {
                    tokio::spawn(files_reads(sessions.clone(), folder.clone(), Arc::clone(&moments)));
                }
                let (sessions, moments, folder, area) =
                    (sessions.clone(), Arc::clone(&moments), folder.clone(), area.clone());
                tokio::spawn(async move {
                    step_seven_stood_in_for(&sessions, &area).await;
                    moments.lock().expect("the moments").step_seven_done = Some(Instant::now());
                    if next == Some(TheNextAsk::AtTheWorkMoving) {
                        files_reads(sessions, folder, moments).await;
                    }
                });
            })
        };
        let installation = TheMachinesInstallationIdentity::for_this_server(row, OverTheConnection)
            .reading_the_row_as_it_stands({
                let row = row.clone();
                move || Some(row.clone())
            });
        let mut half = TheActingHalf {
            opener: TheOpener {
                sessions: sessions.clone(),
                when_it_is_lent,
            },
            credentials: TheKey(far_end.client_key.clone()),
            installation,
            publishes: WhatIsPublished {
                row: row.clone(),
                sessions: sessions.clone(),
                moments: Arc::clone(&moments),
            },
            wait: TheClock,
        };
        let pressed = Instant::now();
        let mut period = APeriod::begun(
            &row.local_server_identifier,
            OpeningAct::TheUsersConnectAction,
            WhoTookIt::TheUser,
            SystemTime::now(),
        );
        let ending = period.end_it();
        // **On a thread of its own, as the application drives a period**
        // (`crate::servers::a_period`), its sockets and clock the test's own
        // runtime's.
        let (driven, driving) = tokio::sync::oneshot::channel();
        let driver = {
            let (row, runtime) = (row.clone(), tokio::runtime::Handle::current());
            std::thread::spawn(move || {
                let mut desk = TheDesk(row.clone());
                let ended = runtime.block_on(period.drive(&row, &mut half, &mut desk));
                let _ = driven.send(ended);
            })
        };
        // **Held until what this period is for has happened**, or a minute.
        let _ = tokio::time::timeout(Duration::from_secs(60), async {
            loop {
                let now = moments.lock().expect("the moments").clone();
                let done = match next {
                    None => now.lent.is_some(),
                    Some(_) => now.listed.is_some(),
                };
                if done || driver.is_finished() {
                    return;
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await;
        // **The user disconnects** (`Servers` S10 item 1).
        ending.at(PeriodEnding::TheUserDisconnects);
        let _ = tokio::time::timeout(Duration::from_secs(30), driving)
            .await
            .expect("the driving ended at the disconnect");
        let moments = moments.lock().expect("the moments").clone();
        (pressed, moments)
    }

    fn since(pressed: Instant, at: Option<Instant>) -> String {
        at.map(|at| format!("{:.2} s", at.saturating_duration_since(pressed).as_secs_f64()))
            .unwrap_or_else(|| "never".to_string())
    }

    /// **The Creator's note 14, timed**: connected, disconnected through the
    /// pane's own act, connected again — and how long until Files shows its
    /// listing, at no added delay and at a quarter of a second each way, with
    /// the page's next ask where it fell before this note and where it falls
    /// after it.
    ///
    /// **What it holds, either way**: Files asked as the row reads *connected*
    /// has no live point to read on — `Servers` publishes *connected* before
    /// its own steps 5 and 6 and only then hands the connection over — so
    /// whatever lists the folder is an ask made later. **And the ask made at
    /// the hand-over lists the folder before the stood-in step 7 is through**
    /// at a far server's delay: the reading Files waits on goes first among
    /// readings and nothing queued ahead of it holds it back for long.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn files_lists_again_at_the_hand_over_rather_than_after_step_seven() {
        let Some(far_end) = TheFarEnd::made("again") else {
            println!(
                "SKIPPED: no OpenSSH sshd, sftp-server, ssh-keygen and Perl this test can run on \
                 this system ({}).",
                std::env::consts::OS
            );
            return;
        };
        let far_end = Arc::new(far_end);
        let supervisor = TheSupervisor::at(&far_end);
        let mut missed = Vec::new();
        for late in [Duration::ZERO, Duration::from_millis(250)] {
            for next in [TheNextAsk::AtTheWorkMoving, TheNextAsk::AtTheHandOver] {
                let said = format!("{} ms each way, the next ask {next:?}", late.as_millis());
                let line = TheLine::to(&far_end, late).await;
                let row = the_row(&line);
                // **Connected, then disconnected through the pane's own act.**
                let (_, first) = one_period(&far_end, &row, None).await;
                if first.lent.is_none() {
                    missed.push(format!("{said}: the first connection was never lent: {first:?}"));
                    continue;
                }
                // **Connected again**, and Files waiting on its folder.
                let (pressed, again) = one_period(&far_end, &row, Some(next)).await;
                println!(
                    "{said}: connected at {}, Files asked then had a reading to follow: {:?}; lent at {}; \
                     step 7 through at {}; Files listed {} names at {}",
                    since(pressed, again.connected),
                    again.a_reading_followed_at_connected,
                    since(pressed, again.lent),
                    since(pressed, again.step_seven_done),
                    again.listed_names,
                    since(pressed, again.listed),
                );
                if again.a_reading_followed_at_connected != Some(false) {
                    missed.push(format!(
                        "{said}: Files asked as the row read connected was {:?}, not a tab with no reading to follow",
                        again.a_reading_followed_at_connected
                    ));
                }
                if again.listed.is_none() || again.listed_names != 40 {
                    missed.push(format!("{said}: Files never listed the forty frames: {again:?}"));
                }
                // **At a far server's delay only**: with nothing added, the
                // listing and the three asks each take a few hundredths of a
                // second and either may end first.
                if next == TheNextAsk::AtTheHandOver && !late.is_zero() {
                    match (again.listed, again.step_seven_done) {
                        (Some(listed), Some(through)) if listed <= through => {}
                        (Some(_), None) => {}
                        _ => missed.push(format!(
                            "{said}: the listing asked at the hand-over waited behind step 7: {again:?}"
                        )),
                    }
                }
            }
        }
        println!("the supervisor was asked {} times in all", supervisor.asked.load(Ordering::SeqCst));
        assert!(missed.is_empty(), "{}", missed.join("\n"));
    }
}

//! **A dead line under every act of the first half** — connect, reconnect and
//! the first checks; reading the file and the dependency check; uploads and
//! downloads; and Files' own acts. Written 2026-10-19 from
//! `Audits/Build/Step 10c/Brief — a dead line, the desk coverage, first
//! half.md`, out of the Creator's ask: *"I think it's prudent to test abrupt
//! connection drops with other operations, not just the one you accidentally
//! caught."* The cut points are the sweep's (`Audits/Build/Step 10c/A dead line
//! under every server act — the sweep.md`, *Desk coverage for the remaining
//! acts*).
//!
//! **What the far end is.** OpenSSH's own `sshd` in its one-connection mode
//! (`sshd -i`) for every connection, its file-transfer sessions OpenSSH's own
//! `sftp-server`, started in a throwaway home. **Between it and the desk is the
//! line**: a relay this test holds, one per connection, whose two halves can
//! stop carrying bytes **without closing anything** — the way a provider's
//! firewall or a lost route goes — either both at once (the line goes silent)
//! or only the half coming back (the machine acts and its answer never
//! arrives).
//!
//! **Stand-ins for a rental's own programs, each named where it is made**: a
//! `stat` answering in GNU's fields, an `uname` naming Linux, a
//! `systemd-detect-virt` that tells the test it was asked, and the machine's
//! `smoozy-server` entry point as a short Perl script carrying the one framed
//! request to this test's own supervisor. **Every byte still crosses the
//! line.**
//!
//! **The desk side is the application's own**: the transport, the per-server
//! point (`lend_until_it_ends`), `Servers`' own driving and sequence, and each
//! act's own function. **A repair is stood in for** by lending a fresh
//! connection on the same point once the old one has ended, which is what
//! `Servers`' standing repair does without its fifteen-second pauses.
//!
//! **The figures are the desk's**: a one-second keepalive and three
//! unanswered, so the declaration window is three to four seconds rather than
//! `Servers` K3's 45 to 60. **Each cut is run briefly, under the window, and
//! past it.**
//!
//! **Nothing of the Creator's is touched.** Keys, the server's configuration,
//! the record and every file are inside throwaway folders deleted with the
//! test, and every server is a child of the test that ends with it.
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
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::io::{AsRawFd, FromRawFd};
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
    use tokio::sync::watch;

    use smoozy_desktop::servers::sessions::{
        lend_until_it_ends, TheAdditionalConnection, ThisServersSessions,
    };
    use smoozy_desktop::surface::files::{a_folder_is_read, WhatTheReadingFound, Words};
    use smoozy_desktop::uploading::TheRecordAnUploadWrites;
    use smoozy_desktop::record::LocalRecord;
    use smoozy_server_protocol::framing::{read_one_request, Request};
    use smoozy_server_protocol::transport::ssh::{self, Connection};
    use smoozy_server_protocol::transport::{
        Credential, DecideHostTrust, Ending, Endpoint, Figures, HostTrust,
    };
    use smoozy_state::identities::{Identity, IdentityKind};

    /// **One of the figures, held or missed** — a miss is kept rather than
    /// failing at once, so a run says every figure before it fails.
    macro_rules! a_figure {
        ($missed:ident, $held:expr, $($said:tt)+) => {
            if !$held {
                $missed.push(format!($($said)+));
            }
        };
    }

    // ------------------------------------------------------------- the figures

    /// **The desk's figures**: `Servers` K3's mechanism at a one-second
    /// interval and three unanswered, so the declaration window is 3 to 4
    /// seconds rather than 45 to 60.
    const AT_THE_DESK: Figures = Figures {
        interval: Duration::from_secs(1),
        unanswered_before_the_break: 3,
    };

    /// **How far past the window's close an act may end and still have ended
    /// with the break**: the desk noticing it, then the one step under way
    /// failing and the act saying so. A rental's bound is K3's 45 to 60
    /// seconds plus the same.
    const ONE_STEP_AFTER_THE_BREAK: Duration = Duration::from_secs(5);

    /// **A silence under the window**: long enough that the act is caught
    /// waiting, short enough that nothing should be declared.
    const A_BRIEF_SILENCE: Duration = Duration::from_millis(1_500);

    fn the_bound() -> Duration {
        AT_THE_DESK.the_declaration_window().1 + ONE_STEP_AFTER_THE_BREAK
    }

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
    /// socket's path has a short limit and two of them live in here.
    struct Scratch(PathBuf);

    static SCRATCHES: AtomicUsize = AtomicUsize::new(0);

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let path = PathBuf::from("/tmp").join(format!(
                "smz-dl-{named}-{}-{}",
                std::process::id(),
                SCRATCHES.fetch_add(1, Ordering::SeqCst)
            ));
            let _ = std::fs::remove_dir_all(&path);
            std::fs::create_dir_all(&path).expect("the scratch folder");
            Scratch(std::fs::canonicalize(&path).expect("the scratch folder is there"))
        }

        fn working(named: &str) -> Scratch {
            let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
                "smz-work-{named}-{}-{}", std::process::id(),
                SCRATCHES.fetch_add(1, Ordering::SeqCst),
            ));
            std::fs::create_dir_all(&path).expect("the working folder");
            Scratch(std::fs::canonicalize(path).expect("the working folder is there"))
        }

        fn folder(&self, named: &str) -> PathBuf {
            let folder = self.0.join(named);
            std::fs::create_dir_all(&folder).expect("a folder in the scratch");
            folder
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

    fn bytes_of(path: &Path) -> Vec<u8> {
        path.as_os_str().as_bytes().to_vec()
    }

    /// **How the far end is stood up.**
    #[derive(Clone, Copy, Default)]
    struct TheFarEndsShape {
        /// **OpenSSH's `MaxSessions`**: at one, the primary holds the reading
        /// session and every transfer rides `Servers` S14's additional
        /// connection.
        one_session_per_connection: bool,
        /// **The file-transfer server announces no copy-data**, so a copy on
        /// the machine is asked of Smoozy's own program instead.
        no_copy_data: bool,
    }

    /// **The machine the test stands up**: OpenSSH's own server, ready to take
    /// one connection at a time, its sessions in a throwaway home; the
    /// machine's package area inside that home; the stand-ins; and the two
    /// sockets the stand-ins reach this test on.
    struct TheFarEnd {
        sshd: PathBuf,
        config: PathBuf,
        client_key: PathBuf,
        home: PathBuf,
        package_area: PathBuf,
        the_supervisors_socket: PathBuf,
        the_signals_socket: PathBuf,
        scratch: Scratch,
        _working: Scratch,
    }

    impl TheFarEnd {
        fn made(named: &str, shape: TheFarEndsShape) -> Option<TheFarEnd> {
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
            // Keep the sockets short; the working root and package area must
            // stand outside shared temporary areas on Linux too.
            let working = Scratch::working(named);
            let home = working.0.join("home");
            let bin = home.join("bin");
            std::fs::create_dir_all(&bin).ok()?;
            let package_area = home.join("area");
            std::fs::create_dir_all(&package_area).ok()?;
            let the_supervisors_socket = folder.join("s.sock");
            let the_signals_socket = folder.join("g.sock");

            // **A stand-in: telling this test a program was asked for**, then
            // waiting for its word — so the test can cut the line at exactly
            // that moment. What the program then prints is the stand-in's own.
            let signalling = |name: &str, then: &str| {
                format!(
                    "#!{perl}\nuse strict;\nuse Socket;\nuse IO::Socket::UNIX;\n\
                     my $s = IO::Socket::UNIX->new(Type => SOCK_STREAM(), Peer => '{socket}');\n\
                     my $what = defined($ARGV[3]) ? (readlink($ARGV[3]) // $ARGV[3]) : '';\n\
                     if ($s) {{ print $s \"{name} $what\\n\"; my $go = <$s>; close($s); }}\n{then}\n",
                    perl = perl.display(),
                    socket = the_signals_socket.display(),
                )
            };
            // **`stat` in GNU's fields.** `Render Settings` Q14 item 2's
            // reading asks for `stat -L -c '%s %i %Y %Z' <file>`, which a
            // rental's Linux answers; only the Mac needs the translation.
            a_program(
                &bin.join("stat"),
                &signalling(
                    "stat",
                    if cfg!(target_os = "macos") {
                        "exec('/usr/bin/stat', '-L', '-f', '%z %i %m %c', $ARGV[3]);"
                    } else {
                        "exec('/usr/bin/stat', @ARGV);"
                    },
                ),
            );
            // **`uname` naming Linux**, which `Servers` Q13's check reads; this
            // system's own names itself.
            a_program(&bin.join("uname"), "#!/bin/sh\necho Linux\n");
            // **`find`, standing in for a rental's search of its own disk**
            // (`Server Setup` B1's `find / … -name blender`), which on this
            // desk would walk the whole of the computer: it names the one
            // build this home holds.
            let a_build = home.join("blender/5.2.0/blender");
            std::fs::create_dir_all(a_build.parent().expect("a folder")).ok()?;
            a_program(&a_build, "#!/bin/sh\necho 'Blender 5.2.0'\necho 'build date: 2026-10-01'\n");
            a_program(&bin.join("find"), &format!("#!/bin/sh\necho '{}'\n", a_build.display()));
            // **`systemd-detect-virt`**, which the hardware reading asks second
            // (`Rented-server runtime` K1's table) — not a container.
            a_program(
                &bin.join("systemd-detect-virt"),
                &signalling("systemd-detect-virt", "print \"none\\n\"; exit 1;"),
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
                    "HostKey \"{}\"\nAuthorizedKeysFile \"{}\"\nPidFile none\nUsePAM no\nStrictModes no\n\
                     PasswordAuthentication no\nKbdInteractiveAuthentication no\n\
                     Subsystem sftp {} -d \"{}\"{}\nSetEnv \"HOME={}\" \"PATH={}:/usr/bin:/bin\"\n{}",
                    folder.join("host").display(),
                    folder.join("authorized_keys").display(),
                    sftp.display(),
                    home.display(),
                    if shape.no_copy_data { " -P copy-data" } else { "" },
                    home.display(),
                    bin.display(),
                    if shape.one_session_per_connection { "MaxSessions 1\n" } else { "" },
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
                package_area,
                the_supervisors_socket,
                the_signals_socket,
                scratch,
                _working: working,
            })
        }

        /// One connection's server: `sshd -i` on one end of a loopback pair.
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

    }

    fn copy_of(descriptor: std::os::unix::io::RawFd) -> std::os::unix::io::RawFd {
        let copy = unsafe { libc::dup(descriptor) };
        assert!(copy >= 0, "the descriptor could not be copied");
        copy
    }

    /// The far end, or the test says why it is skipped and returns.
    macro_rules! a_far_end_or_skip {
        ($named:expr, $shape:expr) => {
            match TheFarEnd::made($named, $shape) {
                Some(far_end) => Arc::new(far_end),
                None => {
                    println!(
                        "SKIPPED: no OpenSSH sshd, sftp-server, ssh-keygen and Perl this test can \
                         run on this system ({}).",
                        std::env::consts::OS
                    );
                    return;
                }
            }
        };
    }

    // ------------------------------------------------------- the supervisor

    /// **What this test's supervisor does with one request.**
    enum Reply {
        /// The frame written back.
        Frame(Vec<u8>),
    }

    type Answering = Box<dyn FnMut(&Request) -> Reply + Send>;

    /// **This test's supervisor, standing in for the machine's own**: each
    /// request is handed to whatever the test has set it to answer with.
    struct TheSupervisor {
        answering: Arc<Mutex<Answering>>,
        asked: Arc<Mutex<Vec<String>>>,
        task: tokio::task::JoinHandle<()>,
    }

    impl TheSupervisor {
        fn at(far_end: &TheFarEnd) -> TheSupervisor {
            let socket = &far_end.the_supervisors_socket;
            let _ = std::fs::remove_file(socket);
            let listener = tokio::net::UnixListener::bind(socket).expect("the supervisor's socket");
            let answering: Arc<Mutex<Answering>> = Arc::new(Mutex::new(Box::new(|request: &Request| {
                Reply::Frame(smoozy_server_protocol::framing::write_result_frame(
                    &smoozy_server_protocol::framing::Answered::to(
                        request,
                        smoozy_server_protocol::framing::THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                        "0.1.0",
                    )
                    .saying_it_does_not_answer_that_request(&request.name),
                ))
            })));
            let asked: Arc<Mutex<Vec<String>>> = Arc::default();
            let task = tokio::spawn({
                let (answering, asked) = (Arc::clone(&answering), Arc::clone(&asked));
                async move {
                    while let Ok((mut stream, _)) = listener.accept().await {
                        let mut frame = Vec::new();
                        if stream.read_to_end(&mut frame).await.is_err() {
                            continue;
                        }
                        let Ok(request) = read_one_request(&mut &frame[..]) else {
                            continue;
                        };
                        asked.lock().expect("asked").push(request.name.clone());
                        let reply = (answering.lock().expect("the answering"))(&request);
                        let Reply::Frame(bytes) = reply;
                        let _ = stream.write_all(&bytes).await;
                        let _ = stream.shutdown().await;
                    }
                }
            });
            TheSupervisor { answering, asked, task }
        }

        fn answers_with(&self, answering: impl FnMut(&Request) -> Reply + Send + 'static) {
            *self.answering.lock().expect("the answering") = Box::new(answering);
        }
    }

    impl Drop for TheSupervisor {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    /// **What the stand-ins tell this test**: each one that signals names
    /// itself and waits for the word to go on, and whatever the test hung on
    /// that name runs first — the line cut at exactly that moment.
    type ASignalsHook = (Box<dyn Fn(&str) -> bool + Send>, Box<dyn FnMut() + Send>);

    struct TheSignals {
        hooks: Arc<Mutex<Vec<ASignalsHook>>>,
        heard: Arc<Mutex<Vec<String>>>,
        task: tokio::task::JoinHandle<()>,
    }

    impl TheSignals {
        fn at(far_end: &TheFarEnd) -> TheSignals {
            let socket = &far_end.the_signals_socket;
            let _ = std::fs::remove_file(socket);
            let listener = tokio::net::UnixListener::bind(socket).expect("the signals' socket");
            let hooks: Arc<Mutex<Vec<ASignalsHook>>> = Arc::default();
            let heard: Arc<Mutex<Vec<String>>> = Arc::default();
            let task = tokio::spawn({
                let (hooks, heard) = (Arc::clone(&hooks), Arc::clone(&heard));
                async move {
                    while let Ok((stream, _)) = listener.accept().await {
                        let (hooks, heard) = (Arc::clone(&hooks), Arc::clone(&heard));
                        tokio::spawn(async move {
                            let (reading, mut writing) = stream.into_split();
                            let mut line = String::new();
                            let mut reading = tokio::io::BufReader::new(reading);
                            if reading.read_line(&mut line).await.is_err() {
                                return;
                            }
                            let said = line.trim().to_string();
                            heard.lock().expect("heard").push(said.clone());
                            let hook = {
                                let mut hooks = hooks.lock().expect("the hooks");
                                hooks
                                    .iter()
                                    .position(|(matches, _)| matches(&said))
                                    .map(|at| hooks.remove(at).1)
                            };
                            if let Some(mut hook) = hook {
                                hook();
                            }
                            let _ = writing.write_all(b"go\n").await;
                        });
                    }
                }
            });
            TheSignals { hooks, heard, task }
        }

        /// **The next time `name` is asked for, `then` runs first.** Once.
        fn when(&self, name: &str, then: impl FnMut() + Send + 'static) {
            let name = name.to_string();
            self.when_it_says(move |said| said.split_whitespace().next() == Some(name.as_str()), then);
        }

        /// **The next time a stand-in says something `matches` takes**,
        /// `then` runs first — `stat` says the file it reads. Once.
        fn when_it_says(&self, matches: impl Fn(&str) -> bool + Send + 'static, then: impl FnMut() + Send + 'static) {
            self.hooks
                .lock()
                .expect("the hooks")
                .push((Box::new(matches), Box::new(then)));
        }

        fn heard(&self, name: &str) -> usize {
            self.heard
                .lock()
                .expect("heard")
                .iter()
                .filter(|heard| heard.split_whitespace().next() == Some(name))
                .count()
        }
    }

    impl Drop for TheSignals {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    // ---------------------------------------------------------------- the line

    /// **Which connections a cut is made on**, in the order they were made.
    #[derive(Debug, Clone, Copy)]
    enum Which {
        /// Every connection made so far, and every one made while it lasts.
        Every,
        /// The one made last.
        TheLast,
    }

    /// One connection's two halves.
    struct TheHalves {
        towards_the_machine: watch::Sender<bool>,
        back_to_the_desk: watch::Sender<bool>,
    }

    /// **The line between the desk and the machine, one under each
    /// connection.** Up, a half carries every byte. **Silent, it carries
    /// nothing and closes nothing**: what is sent stays unread, which is what a
    /// provider's firewall dropping every packet looks like from either end.
    struct TheLine {
        address: SocketAddr,
        /// **What a connection made now starts as**, both halves.
        the_route: watch::Sender<bool>,
        each: Arc<Mutex<Vec<TheHalves>>>,
        /// **How fast a connection made now carries the machine's bytes back**,
        /// in bytes a second; nought is as fast as it can.
        pace_back: Arc<AtomicU64>,
        servers: Arc<Mutex<Vec<std::process::Child>>>,
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
        async fn to(far_end: &Arc<TheFarEnd>) -> Arc<TheLine> {
            let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
                .await
                .expect("the loopback address takes a listener");
            let address = listener.local_addr().expect("a bound listener has an address");
            let (the_route, route) = watch::channel(true);
            let line = Arc::new(TheLine {
                address,
                the_route,
                each: Arc::default(),
                pace_back: Arc::default(),
                servers: Arc::default(),
            });
            let (far_end, each, pace, servers) = (
                Arc::clone(far_end),
                Arc::clone(&line.each),
                Arc::clone(&line.pace_back),
                Arc::clone(&line.servers),
            );
            tokio::spawn(async move {
                while let Ok((desk, _)) = listener.accept().await {
                    let (server, machine) = far_end.one_connection();
                    if let Ok(mut held) = servers.lock() {
                        held.push(server);
                    }
                    let carried = *route.borrow();
                    let (towards, _) = watch::channel(carried);
                    let (back, _) = watch::channel(carried);
                    let (desk_reads, desk_writes) = desk.into_split();
                    let (machine_reads, machine_writes) = machine.into_split();
                    tokio::spawn(carried_while_it_is_up(desk_reads, machine_writes, towards.subscribe(), 0));
                    tokio::spawn(carried_while_it_is_up(
                        machine_reads,
                        desk_writes,
                        back.subscribe(),
                        pace.load(Ordering::SeqCst),
                    ));
                    if let Ok(mut held) = each.lock() {
                        held.push(TheHalves {
                            towards_the_machine: towards,
                            back_to_the_desk: back,
                        });
                    }
                }
            });
            line
        }

        fn connections_made(&self) -> usize {
            self.each.lock().expect("the connections").len()
        }

        fn the_halves(&self, which: Which, mut each: impl FnMut(&TheHalves)) {
            let held = self.each.lock().expect("the connections");
            match which {
                Which::Every => held.iter().for_each(&mut each),
                Which::TheLast => {
                    if let Some(one) = held.last() {
                        each(one);
                    }
                }
            }
        }

        /// **The line goes silent, both ways at once** — callable from inside
        /// a hook. Every connection silenced, the route for a fresh one is down
        /// too.
        fn goes_silent(&self, which: Which) {
            if matches!(which, Which::Every) {
                self.the_route.send_replace(false);
            }
            self.the_halves(which, |halves| {
                halves.towards_the_machine.send_replace(false);
                halves.back_to_the_desk.send_replace(false);
            });
        }

        /// **Only the half coming back goes silent**: what the desk sends still
        /// reaches the machine, and the machine acts on it, but its answer
        /// never arrives — nor any answer to a keepalive.
        fn holds_the_answers(&self, which: Which) {
            if matches!(which, Which::Every) {
                self.the_route.send_replace(false);
            }
            self.the_halves(which, |halves| {
                halves.back_to_the_desk.send_replace(false);
            });
        }

        /// **The line comes back**, and with every connection the route for a
        /// fresh one.
        fn comes_back(&self, which: Which) {
            if matches!(which, Which::Every) {
                self.the_route.send_replace(true);
            }
            self.the_halves(which, |halves| {
                halves.towards_the_machine.send_replace(true);
                halves.back_to_the_desk.send_replace(true);
            });
        }

        fn the_route_is_up(&self, up: bool) {
            self.the_route.send_replace(up);
        }

        /// How fast a connection made from now on carries the machine's bytes
        /// back; nought is as fast as it can.
        fn carries_back_at(&self, bytes_a_second: u64) {
            self.pace_back.store(bytes_a_second, Ordering::SeqCst);
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

    /// **One direction of one connection, carried while its half is up** — at
    /// `pace` bytes a second where it is not nought. A read is given up the
    /// moment the half goes silent, and a socket keeps what it has not handed
    /// over, so nothing is lost and nothing is read while it is silent.
    async fn carried_while_it_is_up<R, W>(mut from: R, mut to: W, mut up: watch::Receiver<bool>, pace: u64)
    where
        R: tokio::io::AsyncRead + Unpin,
        W: tokio::io::AsyncWrite + Unpin,
    {
        let mut buffer = vec![0u8; if pace > 0 { 16 * 1024 } else { 64 * 1024 }];
        loop {
            until_it_is(&mut up, true).await;
            let read = tokio::select! {
                read = from.read(&mut buffer) => read,
                () = until_it_is(&mut up, false) => continue,
            };
            let got = match read {
                Ok(0) | Err(_) => break,
                Ok(got) => got,
            };
            until_it_is(&mut up, true).await;
            if to.write_all(&buffer[..got]).await.is_err() {
                break;
            }
            if pace > 0 {
                tokio::time::sleep(Duration::from_secs_f64(got as f64 / pace as f64)).await;
            }
        }
        let _ = to.shutdown().await;
    }

    // ------------------------------------------------------------ the desk side

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

    fn the_endpoint(line: &TheLine) -> Endpoint {
        Endpoint {
            host: "127.0.0.1".to_string(),
            port: line.address.port(),
            username: the_user(),
        }
    }

    fn the_credential(far_end: &TheFarEnd) -> Credential {
        Credential {
            private_key_file: far_end.client_key.clone(),
            passphrase: None,
        }
    }

    /// **One connection through the line**, or nothing where it does not open
    /// within three seconds.
    async fn a_connection_through(line: &TheLine, far_end: &TheFarEnd) -> Option<Connection> {
        tokio::time::timeout(
            Duration::from_secs(3),
            ssh::open(&the_endpoint(line), the_credential(far_end), AT_THE_DESK, Box::new(Proceeds)),
        )
        .await
        .ok()?
        .ok()
    }

    /// **The per-server point, lending one connection at a time, and lending
    /// a fresh one once the last has ended** — `Servers`' standing repair
    /// without its pauses: a fresh connection is tried every fifth of a second
    /// until one opens. Every ending is kept with its moment.
    struct ThePoint {
        sessions: ThisServersSessions,
        lent: Arc<AtomicUsize>,
        task: tokio::task::JoinHandle<()>,
    }

    impl ThePoint {
        async fn over(line: &Arc<TheLine>, far_end: &Arc<TheFarEnd>, with_an_additional: bool) -> ThePoint {
            let sessions = ThisServersSessions::for_one_server();
            let lent = Arc::new(AtomicUsize::new(0));
            let additional: TheAdditionalConnection<Connection> = if with_an_additional {
                let (line, far_end) = (Arc::clone(line), Arc::clone(far_end));
                Arc::new(move || {
                    let (line, far_end) = (Arc::clone(&line), Arc::clone(&far_end));
                    Box::pin(async move { a_connection_through(&line, &far_end).await })
                })
            } else {
                Arc::new(|| Box::pin(async { None }))
            };
            let task = tokio::spawn({
                let (sessions, lent, line, far_end) = (
                    sessions.clone(),
                    Arc::clone(&lent),
                    Arc::clone(line),
                    Arc::clone(far_end),
                );
                async move {
                    loop {
                        let Some(mut connection) = a_connection_through(&line, &far_end).await else {
                            tokio::time::sleep(Duration::from_millis(200)).await;
                            continue;
                        };
                        lent.fetch_add(1, Ordering::SeqCst);
                        let _ = lend_until_it_ends(&sessions, &mut connection, &additional).await;
                        let _ = connection.tear_down().await;
                    }
                }
            });
            let point = ThePoint { sessions, lent, task };
            point.is_lending("the first connection", 1).await;
            point
        }

        /// **Waits until the point has lent `times` connections in all and is
        /// lending now.**
        async fn is_lending(&self, said: &str, times: usize) {
            until(said, Duration::from_secs(20), || {
                self.lent.load(Ordering::SeqCst) >= times && self.sessions.a_connection_is_live()
            })
            .await;
        }

        fn lent(&self) -> usize {
            self.lent.load(Ordering::SeqCst)
        }
    }

    impl Drop for ThePoint {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    /// **A bounded wait on something the test is certain to see**, with what
    /// it was waiting for said where it is not seen.
    async fn until(said: &str, within: Duration, mut seen: impl FnMut() -> bool) {
        let began = Instant::now();
        while !seen() {
            assert!(began.elapsed() < within, "{said}: not seen within {within:?}");
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
    }

    /// **The words as a person reads them**, the typography point's
    /// non-breaking spaces taken out.
    fn as_read(said: &Words) -> String {
        said.pieces
            .iter()
            .filter_map(|piece| {
                piece
                    .said
                    .as_ref()
                    .map(|words| words.as_str().to_string())
                    .or_else(|| piece.value.as_ref().map(|value| value.as_str().to_string()))
            })
            .collect::<Vec<_>>()
            .join(" ")
            .replace('\u{a0}', " ")
    }

    fn the_names_in(found: &WhatTheReadingFound) -> Option<Vec<String>> {
        match found {
            WhatTheReadingFound::TheFolder { entries, .. } => Some(
                entries
                    .iter()
                    .map(|entry| String::from_utf8_lossy(entry.name.as_bytes()).into_owned())
                    .collect(),
            ),
            _ => None,
        }
    }

    // ------------------------------------------------------------- the record

    type AHook = Box<dyn FnMut(&rusqlite::Connection) -> bool + Send>;

    /// **The application's own record, on a throwaway file**, with a hook a
    /// test can hang on its writes: after each act on the record the hook is
    /// shown the record, and once it answers true it is spent.
    struct TheRecord {
        record: Mutex<LocalRecord>,
        hook: Mutex<Option<AHook>>,
    }

    impl TheRecord {
        fn at(path: &Path) -> Arc<TheRecord> {
            Arc::new(TheRecord {
                record: Mutex::new(LocalRecord::open_at(path).expect("the record opens")),
                hook: Mutex::new(None),
            })
        }

        fn when(&self, hook: impl FnMut(&rusqlite::Connection) -> bool + Send + 'static) {
            *self.hook.lock().expect("the hook") = Some(Box::new(hook));
        }

        fn held(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let Ok(record) = self.record.lock() else {
                return false;
            };
            act(record.connection());
            let mut hook = self.hook.lock().expect("the hook");
            if let Some(then) = hook.as_mut() {
                if then(record.connection()) {
                    *hook = None;
                }
            }
            true
        }

        fn read<T>(&self, act: impl FnOnce(&rusqlite::Connection) -> T) -> T {
            let record = self.record.lock().expect("the record");
            act(record.connection())
        }
    }

    impl TheRecordAnUploadWrites for TheRecord {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            self.held(act)
        }
    }

    impl smoozy_desktop::downloading::TheRecordADownloadWrites for TheRecord {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            self.held(act)
        }
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("random bytes")
    }

    /// **What one cut came to, against the bound**: when the act ended after
    /// the line went silent.
    fn ended_within(said: &str, missed: &mut Vec<String>, took: Duration, bound: Duration) {
        println!("{said}: ended {:.2} s after the cut (bound {:.1} s).", took.as_secs_f64(), bound.as_secs_f64());
        if took > bound {
            missed.push(format!(
                "{said}: the act ended {:.1} s after the cut, past its bound of {:.1} s",
                took.as_secs_f64(),
                bound.as_secs_f64()
            ));
        }
    }

    /// **A run can be narrowed to one cut** with `SMOOZY_ONLY_CUT` naming a
    /// part of its name — for looking at one case alone.
    fn only_this_cut(said: &str) -> bool {
        std::env::var("SMOOZY_ONLY_CUT").map_or(true, |only| said.contains(&only))
    }

    fn the_figures_held(said: &str, missed: Vec<String>) {
        assert!(missed.is_empty(), "{said}:\n{}", missed.join("\n"));
    }

    // =====================================================================
    // Files' own acts: a new folder, a rename, a delete, a move, and both
    // copy routes — the sweep's *cut after the server acts but before its
    // answer*.
    // =====================================================================

    mod files_acts {
        use super::*;
        use smoozy_desktop::managing_files::{
            a_copy_is_made, a_delete_is_made, a_folder_is_made, a_move_is_made, a_rename_is_made,
            ADeleteAsked, NothingIsMarked, WhatTheActCameTo, WhatTheCopyCameTo, WhatTheDeleteCameTo,
            WhatThePasteCameTo,
        };
        use smoozy_desktop::record::file_acts::FileActs;
        use smoozy_desktop::surface::files::{
            what_the_copy_says, what_the_delete_says, what_the_paste_says, what_the_row_says, AnActsAnswer,
        };
        use smoozy_file_browser::managing::{CutOrCopy, Pasteboard};

        fn the_answer_read(answer: &AnActsAnswer) -> String {
            match answer {
                AnActsAnswer::Done { said } => format!("DONE {}", said.as_ref().map(as_read).unwrap_or_default()),
                AnActsAnswer::Said { said } => as_read(said),
                AnActsAnswer::OnTheRow { editing, .. } => {
                    format!("ON THE ROW {}", editing.refused.as_ref().map(as_read).unwrap_or_default())
                }
                AnActsAnswer::Asks { .. } => "ASKS".to_string(),
            }
        }

        /// **The machine acts and its answer never comes**: the half coming
        /// back is held from just before the act, and the act is watched to
        /// its end; or, for a brief silence, the half comes back after
        /// [`A_BRIEF_SILENCE`] and the answer arrives late.
        async fn the_answer_is_held<T>(
            line: &Arc<TheLine>,
            brief: bool,
            act: impl std::future::Future<Output = T>,
        ) -> (T, Duration) {
            line.holds_the_answers(Which::Every);
            let cut = Instant::now();
            let line = Arc::clone(line);
            let back = tokio::spawn(async move {
                if brief {
                    tokio::time::sleep(A_BRIEF_SILENCE).await;
                    line.comes_back(Which::Every);
                }
            });
            let came_to = tokio::time::timeout(Duration::from_secs(60), act)
                .await
                .expect("the act ended within a minute");
            let took = cut.elapsed();
            let _ = back.await;
            (came_to, took)
        }

        /// **A new folder and a rename whose answers are lost**: the machine
        /// made the folder and renamed the file, the user is told the
        /// connection was lost before the server replied — never *done*, never
        /// *taken* — within the break's bound; once the line is back the
        /// folder read again shows what happened, and the same new folder
        /// asked again is refused as taken. **Under the window**, the late
        /// answer is taken and each reads done.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn a_new_folder_and_a_rename_whose_answers_are_lost_say_so_and_the_folder_shows_what_happened() {
            let far_end = a_far_end_or_skip!("folder", TheFarEndsShape::default());
            let line = TheLine::to(&far_end).await;
            let point = ThePoint::over(&line, &far_end, false).await;
            let record = TheRecord::at(&far_end.scratch.folder("desk").join("smoozy.sqlite3"));
            let work = far_end.home.join("work");
            std::fs::create_dir_all(&work).expect("the work folder");
            let mut missed = Vec::new();

            for brief in [true, false] {
                let said = if brief { "UNDER THE WINDOW" } else { "PAST THE WINDOW" };
                line.comes_back(Which::Every);
                point.is_lending(&format!("{said}: lending"), point.lent()).await;
                let before = point.lent();
                // **The reading session is open first**, as the page has it.
                let first = a_folder_is_read(&point.sessions, &bytes_of(&work)).await;
                assert!(the_names_in(&first).is_some(), "{said}: the first reading: {first:?}");
                let name = format!("renders-{brief}");

                // ---- the new folder
                let (came_to, took) = the_answer_is_held(
                    &line,
                    brief,
                    a_folder_is_made(&point.sessions, &bytes_of(&work), name.as_bytes(), &[]),
                )
                .await;
                let shown = the_answer_read(&what_the_row_says(
                    &came_to,
                    name.as_bytes(),
                    smoozy_desktop::surface::files::TheRefusedAct::MakingAFolder,
                    false,
                ));
                println!("{said}: the new folder came to {came_to:?} in {took:?}: {shown}");
                a_figure!(missed, work.join(&name).is_dir(), "{said}: the machine made no folder");
                if brief {
                    a_figure!(missed, came_to == WhatTheActCameTo::Done, "{said}: the new folder came to {came_to:?}");
                    a_figure!(missed, point.lent() == before, "{said}: a brief silence ended the connection");
                } else {
                    ended_within(&format!("{said}: the new folder"), &mut missed, took, the_bound());
                    a_figure!(
                        missed,
                        came_to == WhatTheActCameTo::TheConnectionWent,
                        "{said}: the new folder came to {came_to:?}, not the connection going"
                    );
                    a_figure!(
                        missed,
                        shown.contains("Connection lost before the server replied"),
                        "{said}: the page said {shown:?}"
                    );
                    line.comes_back(Which::Every);
                    point.is_lending(&format!("{said}: lent again"), before + 1).await;
                    let again = a_folder_is_read(&point.sessions, &bytes_of(&work)).await;
                    a_figure!(
                        missed,
                        the_names_in(&again).is_some_and(|names| names.contains(&name)),
                        "{said}: the folder read again does not show the new folder: {again:?}"
                    );
                    let repeated = a_folder_is_made(&point.sessions, &bytes_of(&work), name.as_bytes(), &[]).await;
                    a_figure!(
                        missed,
                        repeated == WhatTheActCameTo::TheNameIsTaken,
                        "{said}: the new folder asked again came to {repeated:?}"
                    );
                }

                // ---- the rename
                let before = point.lent();
                let from = work.join(format!("kitchen-{brief}.blend"));
                std::fs::write(&from, b"a scene").expect("the file to rename");
                let to_name = format!("kitchen-final-{brief}.blend");
                let (came_to, took) = the_answer_is_held(
                    &line,
                    brief,
                    a_rename_is_made(
                        &point.sessions,
                        record.as_ref(),
                        &NothingIsMarked,
                        &bytes_of(&from),
                        to_name.as_bytes(),
                        &[],
                    ),
                )
                .await;
                let shown = the_answer_read(&what_the_row_says(
                    &came_to,
                    to_name.as_bytes(),
                    smoozy_desktop::surface::files::TheRefusedAct::Renaming,
                    false,
                ));
                println!("{said}: the rename came to {came_to:?} in {took:?}: {shown}");
                a_figure!(
                    missed,
                    work.join(&to_name).is_file() && !from.exists(),
                    "{said}: the machine did not rename"
                );
                if brief {
                    a_figure!(missed, came_to == WhatTheActCameTo::Done, "{said}: the rename came to {came_to:?}");
                } else {
                    ended_within(&format!("{said}: the rename"), &mut missed, took, the_bound());
                    a_figure!(
                        missed,
                        came_to == WhatTheActCameTo::TheConnectionWent,
                        "{said}: the rename came to {came_to:?}, not the connection going"
                    );
                    a_figure!(
                        missed,
                        shown.contains("Connection lost before the server replied"),
                        "{said}: the page said {shown:?}"
                    );
                    line.comes_back(Which::Every);
                    point.is_lending(&format!("{said}: lent again"), before + 1).await;
                    let again = a_folder_is_read(&point.sessions, &bytes_of(&work)).await;
                    a_figure!(
                        missed,
                        the_names_in(&again).is_some_and(|names| names.contains(&to_name)),
                        "{said}: the folder read again does not show the renamed file: {again:?}"
                    );
                }
            }
            the_figures_held("A NEW FOLDER AND A RENAME", missed);
        }

        /// A folder of `files` files in `folders` folders under `root`.
        fn a_tree(root: &Path, folders: usize, files: usize) {
            for folder in 0..folders {
                let at = root.join(format!("f{folder:02}"));
                std::fs::create_dir_all(&at).expect("a folder in the tree");
                for file in 0..files {
                    std::fs::write(at.join(format!("{file:03}.exr")), b"frame").expect("a file in the tree");
                }
            }
        }

        fn the_files_under(root: &Path) -> usize {
            let mut count = 0;
            let mut pending = vec![root.to_path_buf()];
            while let Some(folder) = pending.pop() {
                let Ok(entries) = std::fs::read_dir(&folder) else { continue };
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        pending.push(entry.path());
                    }
                    count += 1;
                }
            }
            count
        }

        /// **A delete of a tree whose answers are lost part-way**: the half
        /// coming back is held the moment the first file goes, so the tree is
        /// left partly deleted. **What the user is told is honest**: every
        /// entry it counts deleted is gone, every one it counts not tried is
        /// still there, and the rest are said to be unanswered — never all
        /// deleted, never refused. The act's record keeps the same. Once the
        /// line is back, the folder read again shows what is left, and a
        /// second delete removes it.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn a_delete_whose_answers_are_lost_part_way_counts_honestly_and_a_second_delete_finishes_it() {
            let far_end = a_far_end_or_skip!("delete", TheFarEndsShape::default());
            let line = TheLine::to(&far_end).await;
            let point = ThePoint::over(&line, &far_end, false).await;
            let record = TheRecord::at(&far_end.scratch.folder("desk").join("smoozy.sqlite3"));
            let mut missed = Vec::new();
            for brief in [true, false] {
                let said = if brief { "UNDER THE WINDOW" } else { "PAST THE WINDOW" };
                line.comes_back(Which::Every);
                let before = point.lent();
                let work = far_end.home.join(format!("delete-{brief}"));
                let victim = work.join("cache");
                a_tree(&victim, 6, 150);
                let total = the_files_under(&victim) + 1;
                let first = a_folder_is_read(&point.sessions, &bytes_of(&work)).await;
                assert!(the_names_in(&first).is_some(), "{said}: the first reading: {first:?}");
                // **The cut, the moment the first entry is gone.**
                let watching = {
                    let (line, victim) = (Arc::clone(&line), victim.clone());
                    tokio::spawn(async move {
                        let began = Instant::now();
                        while the_files_under(&victim) + 1 >= total && began.elapsed() < Duration::from_secs(30) {
                            tokio::time::sleep(Duration::from_micros(200)).await;
                        }
                        line.holds_the_answers(Which::Every);
                        let cut = Instant::now();
                        if brief {
                            tokio::time::sleep(A_BRIEF_SILENCE).await;
                            line.comes_back(Which::Every);
                        }
                        cut
                    })
                };
                let asked = ADeleteAsked {
                    victims: vec![bytes_of(&victim)],
                    server_row: an_identity(IdentityKind::LocalServer),
                    installation: None,
                    run: an_identity(IdentityKind::Run),
                };
                let came_to = tokio::time::timeout(
                    Duration::from_secs(60),
                    a_delete_is_made(&point.sessions, record.as_ref(), &NothingIsMarked, &asked, &[], 8, &|_, _| {}),
                )
                .await
                .expect("the delete ended within a minute");
                let ended = Instant::now();
                let cut = watching.await.expect("the watch");
                let took = ended.saturating_duration_since(cut);
                let shown = the_answer_read(&what_the_delete_says(&came_to, &asked.victims));
                let left = if victim.exists() { the_files_under(&victim) + 1 } else { 0 };
                println!("{said}: the delete came to {came_to:?}, {left} of {total} left: {shown}");
                let WhatTheDeleteCameTo::Ran { record: the_act, done, refused, not_attempted, never_learned } =
                    came_to.clone()
                else {
                    missed.push(format!("{said}: the delete came to {came_to:?}"));
                    continue;
                };
                a_figure!(missed, refused == 0, "{said}: {refused} entries were said refused by the server");
                a_figure!(
                    missed,
                    done + refused + not_attempted + never_learned == total,
                    "{said}: the counts do not add up to the {total} entries"
                );
                // **The act's record, unit by unit, against the machine.**
                let read = record
                    .read(|connection| FileActs::over(connection).read(&the_act))
                    .expect("the act reads")
                    .expect("the act was written");
                for (unit, outcome) in &read.units {
                    let there = Path::new(std::ffi::OsStr::from_bytes(&unit.source)).exists();
                    match outcome {
                        Some(smoozy_state::outcomes::UnitOutcome::Done) => a_figure!(
                            missed,
                            !there,
                            "{said}: {} is recorded deleted and is still there",
                            String::from_utf8_lossy(&unit.source)
                        ),
                        Some(smoozy_state::outcomes::UnitOutcome::NotAttempted { .. }) => a_figure!(
                            missed,
                            there,
                            "{said}: {} is recorded not tried and is gone",
                            String::from_utf8_lossy(&unit.source)
                        ),
                        _ => {}
                    }
                }
                if brief {
                    a_figure!(
                        missed,
                        done == total && left == 0,
                        "{said}: a brief silence left the delete at {done} of {total}"
                    );
                    a_figure!(missed, point.lent() == before, "{said}: a brief silence ended the connection");
                    continue;
                }
                ended_within(&format!("{said}: the delete"), &mut missed, took, the_bound());
                a_figure!(missed, left > 0 && left < total, "{said}: the tree was not left partly deleted ({left} of {total})");
                a_figure!(missed, never_learned > 0, "{said}: nothing was said to be unanswered");
                a_figure!(
                    missed,
                    shown.contains("Connection lost before the server replied for some items"),
                    "{said}: the page said {shown:?}"
                );
                line.comes_back(Which::Every);
                point.is_lending(&format!("{said}: lent again"), before + 1).await;
                let again = a_folder_is_read(&point.sessions, &bytes_of(&work)).await;
                a_figure!(
                    missed,
                    the_names_in(&again).is_some_and(|names| names.contains(&"cache".to_string())),
                    "{said}: the folder read again does not show what is left: {again:?}"
                );
                let finished = a_delete_is_made(
                    &point.sessions,
                    record.as_ref(),
                    &NothingIsMarked,
                    &asked,
                    &[],
                    8,
                    &|_, _| {},
                )
                .await;
                a_figure!(
                    missed,
                    matches!(finished, WhatTheDeleteCameTo::Ran { refused: 0, not_attempted: 0, never_learned: 0, .. })
                        && !victim.exists(),
                    "{said}: the second delete came to {finished:?}"
                );
            }
            the_figures_held("A DELETE, PARTLY DONE", missed);
        }

        /// **A move whose answer is lost**: the machine moved the file, the
        /// user is told one result was never answered — never moved, never
        /// refused — and the act's record leaves its unit without an outcome.
        /// Once the line is back the destination shows the file, and the same
        /// paste asked again finds its source gone and moves nothing.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn a_move_whose_answer_is_lost_says_it_was_never_answered_and_the_destination_shows_it() {
            let far_end = a_far_end_or_skip!("move", TheFarEndsShape::default());
            let line = TheLine::to(&far_end).await;
            let point = ThePoint::over(&line, &far_end, false).await;
            let record = TheRecord::at(&far_end.scratch.folder("desk").join("smoozy.sqlite3"));
            let mut missed = Vec::new();
            for brief in [true, false] {
                let said = if brief { "UNDER THE WINDOW" } else { "PAST THE WINDOW" };
                line.comes_back(Which::Every);
                let before = point.lent();
                let from_folder = far_end.home.join(format!("from-{brief}"));
                let into = far_end.home.join(format!("into-{brief}"));
                std::fs::create_dir_all(&from_folder).expect("the folder");
                std::fs::create_dir_all(&into).expect("the folder");
                let source = from_folder.join("kitchen.blend");
                std::fs::write(&source, b"a scene").expect("the file");
                let first = a_folder_is_read(&point.sessions, &bytes_of(&into)).await;
                assert!(the_names_in(&first).is_some(), "{said}: the first reading: {first:?}");
                let pasteboard = Pasteboard {
                    server_row: an_identity(IdentityKind::LocalServer),
                    installation: None,
                    act: CutOrCopy::Cut,
                    entries: vec![bytes_of(&source)],
                };
                let run = an_identity(IdentityKind::Run);
                // **The answer held from the moment the act's record stands**,
                // after the plan was read and before the rename leaves.
                {
                    let line = Arc::clone(&line);
                    record.when(move |connection| {
                        let written: i64 = connection
                            .query_row("SELECT count(*) FROM file_act", [], |row| row.get(0))
                            .unwrap_or(0);
                        if written > 0 {
                            line.holds_the_answers(Which::Every);
                        }
                        written > 0
                    });
                }
                let cut = Arc::new(Mutex::new(None::<Instant>));
                let back = {
                    let (line, cut) = (Arc::clone(&line), Arc::clone(&cut));
                    tokio::spawn(async move {
                        until("the answer is held", Duration::from_secs(30), || {
                            line.each.lock().unwrap().iter().any(|halves| !*halves.back_to_the_desk.borrow())
                        })
                        .await;
                        *cut.lock().unwrap() = Some(Instant::now());
                        if brief {
                            tokio::time::sleep(A_BRIEF_SILENCE).await;
                            line.comes_back(Which::Every);
                        }
                    })
                };
                let came_to = tokio::time::timeout(
                    Duration::from_secs(60),
                    a_move_is_made(
                        &point.sessions,
                        record.as_ref(),
                        &NothingIsMarked,
                        &pasteboard,
                        &bytes_of(&into),
                        None,
                        &[],
                        &run,
                        None,
                    ),
                )
                .await
                .expect("the move ended within a minute");
                let ended = Instant::now();
                back.await.expect("the watch");
                let took = ended.saturating_duration_since(cut.lock().unwrap().unwrap_or(ended));
                let shown = the_answer_read(&what_the_paste_says(
                    &came_to,
                    &pasteboard.entries,
                    &bytes_of(&into),
                    &[],
                    &[],
                    smoozy_desktop::surface::files::TheCollidingAct::APasteAfterACut,
                ));
                println!("{said}: the move came to {came_to:?} in {took:?}: {shown}");
                a_figure!(
                    missed,
                    into.join("kitchen.blend").is_file() && !source.exists(),
                    "{said}: the machine did not move the file"
                );
                if brief {
                    a_figure!(
                        missed,
                        matches!(came_to, WhatThePasteCameTo::Ran { done: 1, never_learned: 0, .. }),
                        "{said}: the move came to {came_to:?}"
                    );
                    continue;
                }
                ended_within(&format!("{said}: the move"), &mut missed, took, the_bound());
                let WhatThePasteCameTo::Ran { record: the_act, done, refused, never_learned, .. } = came_to.clone()
                else {
                    missed.push(format!("{said}: the move came to {came_to:?}"));
                    continue;
                };
                a_figure!(
                    missed,
                    done == 0 && refused == 0 && never_learned == 1,
                    "{said}: the move counted {came_to:?}, not one never answered"
                );
                a_figure!(
                    missed,
                    shown.contains("Connection lost before the server replied"),
                    "{said}: the page said {shown:?}"
                );
                let read = record
                    .read(|connection| FileActs::over(connection).read(&the_act))
                    .expect("the act reads")
                    .expect("the act was written");
                a_figure!(
                    missed,
                    read.units.iter().all(|(_, outcome)| outcome.is_none()),
                    "{said}: the act's record holds an outcome nobody learned: {:?}",
                    read.units
                );
                line.comes_back(Which::Every);
                point.is_lending(&format!("{said}: lent again"), before + 1).await;
                let again = a_folder_is_read(&point.sessions, &bytes_of(&into)).await;
                a_figure!(
                    missed,
                    the_names_in(&again).is_some_and(|names| names.contains(&"kitchen.blend".to_string())),
                    "{said}: the destination read again does not show the file: {again:?}"
                );
                let repeated = a_move_is_made(
                    &point.sessions,
                    record.as_ref(),
                    &NothingIsMarked,
                    &pasteboard,
                    &bytes_of(&into),
                    None,
                    &[],
                    &run,
                    None,
                )
                .await;
                a_figure!(
                    missed,
                    matches!(repeated, WhatThePasteCameTo::Ran { done: 0, not_done: 1, .. }),
                    "{said}: the same paste asked again came to {repeated:?}"
                );
                a_figure!(missed, into.join("kitchen.blend").is_file(), "{said}: the moved file went");
            }
            the_figures_held("A MOVE", missed);
        }

        /// **Where a copy's cut falls.**
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum WhereTheCopyIsCut {
            /// **The moment the act's record stands**, before anything of the
            /// copy is asked of the machine: nothing is copied, and nothing
            /// may be said to be.
            AtItsRecord,
            /// **With the bytes**: the staging file standing (copy-data), or
            /// the program's copy made (Smoozy's own program) — the machine
            /// has acted and its answer never comes.
            WithTheBytes { brief: bool },
        }

        /// **A copy cut at its record and with its bytes**, by the file
        /// service's own copy-data and by Smoozy's own program on the machine:
        /// cut before anything is asked, it says the copy was not done; cut
        /// once the machine has acted, it says the copy was never answered —
        /// never copied, never refused by the server — and in both the source
        /// stays and nothing half-copied stands at the destination's name.
        /// Once the line is back the copy asked again lands. **Under the
        /// window**, the late answer is taken and the copy reads copied.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn a_copy_whose_answer_is_lost_says_so_by_either_route_and_leaves_the_source() {
            let mut missed = Vec::new();
            for no_copy_data in [false, true] {
                let route = if no_copy_data { "THE PROGRAM" } else { "COPY-DATA" };
                let far_end = a_far_end_or_skip!(
                    if no_copy_data { "copy-p" } else { "copy-d" },
                    TheFarEndsShape { no_copy_data, ..Default::default() }
                );
                let line = TheLine::to(&far_end).await;
                let supervisor = TheSupervisor::at(&far_end);
                // **Smoozy's own program copies the file** where it is asked,
                // as the machine's does, and answers it copied — with the line
                // holding its answers first where the test asks.
                let hold_after_copying = Arc::new(AtomicBool::new(false));
                {
                    let (line, hold) = (Arc::clone(&line), Arc::clone(&hold_after_copying));
                    supervisor.answers_with(move |request| {
                        use smoozy_server_protocol::framing::{
                            write_result_frame, Answered, Fields, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                        };
                        use smoozy_server_protocol::requests as asked;
                        let answered = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0");
                        if request.name != asked::THE_FILE_IS_COPIED_ON_THE_MACHINE_REQUEST {
                            return Reply::Frame(write_result_frame(
                                &answered.saying_it_does_not_answer_that_request(&request.name),
                            ));
                        }
                        let from = request.fields.value(asked::THE_COPYS_SOURCE_FIELD).unwrap_or_default().to_vec();
                        let to = request.fields.value(asked::THE_COPYS_STAGING_PATH_FIELD).unwrap_or_default().to_vec();
                        let copied = std::fs::copy(
                            std::ffi::OsStr::from_bytes(&from),
                            std::ffi::OsStr::from_bytes(&to),
                        )
                        .is_ok();
                        if hold.swap(false, Ordering::SeqCst) {
                            line.holds_the_answers(Which::Every);
                        }
                        Reply::Frame(write_result_frame(&answered.carrying(Fields::none().and(
                            asked::THE_COPYS_OUTCOME_FIELD,
                            if copied { asked::IT_WAS_COPIED } else { "it was not copied" },
                        ))))
                    });
                }
                let point = ThePoint::over(&line, &far_end, false).await;
                let record = TheRecord::at(&far_end.scratch.folder("desk").join("smoozy.sqlite3"));
                let package_area = far_end.package_area.to_string_lossy().into_owned();
                for (at, cut) in [
                    WhereTheCopyIsCut::AtItsRecord,
                    WhereTheCopyIsCut::WithTheBytes { brief: true },
                    WhereTheCopyIsCut::WithTheBytes { brief: false },
                ]
                .into_iter()
                .enumerate()
                {
                    let said = format!("{route}, {cut:?}");
                    let brief = cut == WhereTheCopyIsCut::WithTheBytes { brief: true };
                    line.comes_back(Which::Every);
                    let before = point.lent();
                    let from_folder = far_end.home.join(format!("src-{at}"));
                    let into = far_end.home.join(format!("dst-{at}"));
                    std::fs::create_dir_all(&from_folder).expect("the folder");
                    std::fs::create_dir_all(&into).expect("the folder");
                    let source = from_folder.join("plate.exr");
                    let size = 64 * 1024 * 1024;
                    std::fs::write(&source, vec![9u8; size]).expect("the file");
                    let first = a_folder_is_read(&point.sessions, &bytes_of(&into)).await;
                    assert!(the_names_in(&first).is_some(), "{said}: the first reading: {first:?}");
                    let pasteboard = Pasteboard {
                        server_row: an_identity(IdentityKind::LocalServer),
                        installation: None,
                        act: CutOrCopy::Copy,
                        entries: vec![bytes_of(&source)],
                    };
                    let run = an_identity(IdentityKind::Run);
                    let staging = into.join(".smoozy-staging");
                    match cut {
                        WhereTheCopyIsCut::AtItsRecord => {
                            let line = Arc::clone(&line);
                            record.when(move |connection| {
                                let written: i64 = connection
                                    .query_row("SELECT count(*) FROM file_act", [], |row| row.get(0))
                                    .unwrap_or(0);
                                if written > 0 {
                                    line.holds_the_answers(Which::Every);
                                }
                                written > 0
                            });
                        }
                        WhereTheCopyIsCut::WithTheBytes { .. } if no_copy_data => {
                            hold_after_copying.store(true, Ordering::SeqCst);
                        }
                        WhereTheCopyIsCut::WithTheBytes { .. } => {
                            let (line, staging) = (Arc::clone(&line), staging.clone());
                            tokio::spawn(async move {
                                let began = Instant::now();
                                while std::fs::read_dir(&staging).map(|mut held| held.next().is_none()).unwrap_or(true)
                                    && began.elapsed() < Duration::from_secs(30)
                                {
                                    tokio::time::sleep(Duration::from_micros(200)).await;
                                }
                                line.holds_the_answers(Which::Every);
                            });
                        }
                    }
                    let cut_at = Arc::new(Mutex::new(None::<Instant>));
                    let back = {
                        let (line, cut_at) = (Arc::clone(&line), Arc::clone(&cut_at));
                        tokio::spawn(async move {
                            until("the answer is held", Duration::from_secs(30), || {
                                line.each.lock().unwrap().iter().any(|halves| !*halves.back_to_the_desk.borrow())
                            })
                            .await;
                            *cut_at.lock().unwrap() = Some(Instant::now());
                            if brief {
                                tokio::time::sleep(A_BRIEF_SILENCE).await;
                                line.comes_back(Which::Every);
                            }
                        })
                    };
                    let came_to = tokio::time::timeout(
                        Duration::from_secs(60),
                        a_copy_is_made(
                            &point.sessions,
                            record.as_ref(),
                            &NothingIsMarked,
                            &pasteboard,
                            &bytes_of(&into),
                            None,
                            &[],
                            &run,
                            Some(&package_area),
                            &|_, _| {},
                        ),
                    )
                    .await
                    .expect("the copy ended within a minute");
                    let ended = Instant::now();
                    back.await.expect("the watch");
                    let took = ended.saturating_duration_since(cut_at.lock().unwrap().unwrap_or(ended));
                    let shown = the_answer_read(&what_the_copy_says(&came_to, &pasteboard.entries, &bytes_of(&into), &[], &[]));
                    let landed = into.join("plate.exr");
                    println!("{said}: the copy came to {came_to:?} in {took:?}: {shown}");
                    a_figure!(missed, source.is_file(), "{said}: the source went");
                    a_figure!(
                        missed,
                        !landed.exists() || std::fs::metadata(&landed).map(|held| held.len()).unwrap_or(0) == size as u64,
                        "{said}: a partial copy stands at the destination's name"
                    );
                    match cut {
                        WhereTheCopyIsCut::WithTheBytes { brief: true } => {
                            a_figure!(
                                missed,
                                matches!(came_to, WhatTheCopyCameTo::Ran { done: 1, never_learned: 0, .. }) && landed.is_file(),
                                "{said}: the copy came to {came_to:?}"
                            );
                            continue;
                        }
                        WhereTheCopyIsCut::AtItsRecord => {
                            ended_within(&format!("{said}: the copy"), &mut missed, took, the_bound());
                            a_figure!(
                                missed,
                                matches!(came_to, WhatTheCopyCameTo::Ran { done: 0, refused: 0, not_done: 1, never_learned: 0, .. })
                                    && !landed.exists(),
                                "{said}: the copy cut before it was asked counted {came_to:?}"
                            );
                        }
                        WhereTheCopyIsCut::WithTheBytes { brief: false } => {
                            ended_within(&format!("{said}: the copy"), &mut missed, took, the_bound());
                            a_figure!(
                                missed,
                                matches!(came_to, WhatTheCopyCameTo::Ran { done: 0, refused: 0, never_learned: 1, .. }),
                                "{said}: the copy counted {came_to:?}, not one never answered"
                            );
                            a_figure!(
                                missed,
                                shown.contains("Connection lost before the server replied"),
                                "{said}: the page said {shown:?}"
                            );
                        }
                    }
                    line.comes_back(Which::Every);
                    point.is_lending(&format!("{said}: lent again"), before + 1).await;
                    let answer = if landed.exists() {
                        Some(smoozy_file_browser::already_exists::CollisionAnswer::Replace)
                    } else {
                        None
                    };
                    let again = a_copy_is_made(
                        &point.sessions,
                        record.as_ref(),
                        &NothingIsMarked,
                        &pasteboard,
                        &bytes_of(&into),
                        answer,
                        &[],
                        &run,
                        Some(&package_area),
                        &|_, _| {},
                    )
                    .await;
                    a_figure!(
                        missed,
                        matches!(again, WhatTheCopyCameTo::Ran { done: 1, .. })
                            && std::fs::metadata(&landed).map(|held| held.len()).unwrap_or(0) == size as u64,
                        "{said}: the copy asked again came to {again:?}"
                    );
                }
                drop(supervisor);
            }
            the_figures_held("A COPY", missed);
        }

        /// **A copy whose extra connection alone goes silent**, at one session
        /// per connection: the copy's channel rides `Servers` S14's additional
        /// connection, the primary goes on answering, and the copy ends at that
        /// connection's own declaration, saying honestly what it did.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn a_copy_whose_additional_connection_alone_goes_silent_ends_at_its_declaration_and_says_so() {
            let far_end = a_far_end_or_skip!(
                "copy-alone",
                TheFarEndsShape { one_session_per_connection: true, no_copy_data: false }
            );
            let line = TheLine::to(&far_end).await;
            let point = ThePoint::over(&line, &far_end, true).await;
            let record = TheRecord::at(&far_end.scratch.folder("desk").join("smoozy.sqlite3"));
            let mut missed = Vec::new();
            let from_folder = far_end.home.join("src");
            let into = far_end.home.join("dst");
            std::fs::create_dir_all(&from_folder).expect("the folder");
            std::fs::create_dir_all(&into).expect("the folder");
            let sources: Vec<PathBuf> = (0..3)
                .map(|at| {
                    let source = from_folder.join(format!("plate-{at}.exr"));
                    std::fs::write(&source, vec![9u8; 1024 * 1024]).expect("the file");
                    source
                })
                .collect();
            let first = a_folder_is_read(&point.sessions, &bytes_of(&into)).await;
            assert!(the_names_in(&first).is_some(), "the first reading: {first:?}");
            let pasteboard = Pasteboard {
                server_row: an_identity(IdentityKind::LocalServer),
                installation: None,
                act: CutOrCopy::Copy,
                entries: sources.iter().map(|source| bytes_of(source)).collect(),
            };
            let run = an_identity(IdentityKind::Run);
            // **The additional connection's answers held at the first file's
            // outcome**, the primary carrying on.
            {
                let line = Arc::clone(&line);
                record.when(move |connection| {
                    let ended: i64 = connection
                        .query_row(
                            "SELECT count(*) FROM outcome_record_unit WHERE outcome IS NOT NULL",
                            [],
                            |row| row.get(0),
                        )
                        .unwrap_or(0);
                    if ended > 0 {
                        line.holds_the_answers(Which::TheLast);
                    }
                    ended > 0
                });
            }
            let lent_before = point.lent();
            let came_to = tokio::time::timeout(
                Duration::from_secs(60),
                a_copy_is_made(
                    &point.sessions,
                    record.as_ref(),
                    &NothingIsMarked,
                    &pasteboard,
                    &bytes_of(&into),
                    None,
                    &[],
                    &run,
                    None,
                    &|_, _| {},
                ),
            )
            .await
            .expect("the copy ended within a minute");
            let shown = the_answer_read(&what_the_copy_says(&came_to, &pasteboard.entries, &bytes_of(&into), &[], &[]));
            println!("ALONE: the copy came to {came_to:?}: {shown}; {} connections made", line.connections_made());
            a_figure!(missed, line.connections_made() >= 2, "the copy rode no additional connection");
            a_figure!(
                missed,
                point.lent() == lent_before && point.sessions.a_connection_is_live(),
                "the primary did not stand"
            );
            let read = a_folder_is_read(&point.sessions, &bytes_of(&into)).await;
            a_figure!(missed, the_names_in(&read).is_some(), "the primary read nothing: {read:?}");
            match &came_to {
                WhatTheCopyCameTo::Ran { done, refused, not_done, never_learned, .. } => {
                    let landed = sources
                        .iter()
                        .filter(|source| into.join(source.file_name().unwrap()).is_file())
                        .count();
                    a_figure!(missed, *refused == 0, "the copy said {refused} were refused by the server");
                    a_figure!(missed, *done <= landed, "the copy counted {done} copied and {landed} landed");
                    a_figure!(
                        missed,
                        done + not_done + never_learned + refused == 3,
                        "the copy's counts do not add up: {came_to:?}"
                    );
                }
                other => missed.push(format!("the copy came to {other:?}")),
            }
            for source in &sources {
                a_figure!(missed, source.is_file(), "a source went: {}", source.display());
            }
            the_figures_held("A COPY ON THE ADDITIONAL CONNECTION, CUT ALONE", missed);
        }
    }

    // =====================================================================
    // Uploads and downloads: before the transfer's setup, mid-file, at the
    // prefix's check, at publication and with publication's answer lost, and
    // over a file already there.
    // =====================================================================

    mod transfers {
        use super::*;
        use smoozy_desktop::downloading::{
            a_download_is_prepared, the_download_runs, the_request_goes_without_a_question, TheRunsTransfers,
            WhatIsBeingDownloaded,
        };
        use smoozy_desktop::record::server_rows;
        use smoozy_desktop::scheduling::{BytesPerSecond, TheCap};
        use smoozy_desktop::surface::transfers::{the_screen_the_record_answers, TheTransfersScreen};
        use smoozy_desktop::uploading::{
            an_upload_is_prepared, the_answer_is_applied, the_upload_runs, WhatIsBeingUploaded,
            WhatTheAnswerLeft, WhenAFileIsPublished,
        };
        use smoozy_download::acting::enumerating::{AJobsOwnFolder, ReadsTheLocalJobRecords};
        use smoozy_download::vouching::WhatTheLocalRecordSays;
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        use smoozy_servers::store::ServerRow;
        use smoozy_state::outcomes::UnitOutcome;

        struct NobodyIsTold;
        impl WhenAFileIsPublished for NobodyIsTold {
            fn a_file_was_published(&self, _landing_path: &[u8], _outcome: &UnitOutcome) {}
        }

        struct NoJobIsKnown;
        impl ReadsTheLocalJobRecords for NoJobIsKnown {
            fn what_the_local_record_says(&self, _source_path: &[u8]) -> WhatTheLocalRecordSays {
                WhatTheLocalRecordSays {
                    the_file_is_inside_the_jobs_output_folder: false,
                    the_recorded_ending: None,
                    the_completeness_statement: None,
                }
            }
            fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
                None
            }
            fn the_job_whose_output_folder_this_is(&self, _picked: &[u8]) -> Option<AJobsOwnFolder> {
                None
            }
        }

        /// **A server row in the record**, so the Transfers screen reads the
        /// requests made against it.
        fn a_server_row_in(record: &TheRecord) -> Identity {
            let row = an_identity(IdentityKind::LocalServer);
            let written = ServerRow {
                local_server_identifier: row.as_text(),
                host: Host::what_the_user_typed("198.51.100.10").expect("a host"),
                port: Port::STANDARD_SSH,
                username: Username::what_the_user_typed("root").expect("a username"),
                key: KeyReference {
                    local_key_identifier: "key-1".into(),
                    fingerprint: "SHA256:aaa".into(),
                },
                label: None,
                colour: None,
                remembered_setup: None,
                remembered_hardware_reading: None,
                remembered_host_key: None,
                recorded_machine_root: None,
                last_confirmed_installation_identity: None,
                event_cursor: None,
                machine_password_subject: None,
                chosen_fallback_folder: None,
                the_driver_missed_the_optix_bar: false,
            };
            record.read(|connection| server_rows::add(connection, &written).expect("the row is added"));
            row
        }

        /// A file of `size` bytes whose every byte says where it is.
        pub(super) fn a_file_at(path: &Path, size: usize, seed: u8) {
            let bytes: Vec<u8> = (0..size).map(|at| ((at / 4096) as u8).wrapping_add(seed)).collect();
            std::fs::write(path, bytes).expect("the file is written");
        }

        fn it_is_the_file(path: &Path, size: usize, seed: u8) -> bool {
            std::fs::read(path).is_ok_and(|bytes| {
                bytes.len() == size && bytes.iter().enumerate().all(|(at, byte)| *byte == ((at / 4096) as u8).wrapping_add(seed))
            })
        }

        /// Whatever is left in Smoozy's staging folders under `folder`.
        pub(super) fn left_staged(folder: &Path) -> Vec<String> {
            let mut left = Vec::new();
            let mut pending = vec![folder.to_path_buf()];
            while let Some(at) = pending.pop() {
                let Ok(entries) = std::fs::read_dir(&at) else { continue };
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        pending.push(path);
                    } else if path.to_string_lossy().contains(".smoozy-staging") {
                        left.push(path.to_string_lossy().into_owned());
                    }
                }
            }
            left
        }

        fn the_screen(record: &TheRecord, the_run: &TheRunsTransfers) -> TheTransfersScreen {
            let held = record.record.lock().expect("the record");
            the_screen_the_record_answers(Some(&held), the_run)
        }

        /// **What a person looking at the request's row reads**: whether it
        /// reads under way, and its first file's state and tail.
        fn the_row_reads(record: &TheRecord, the_run: &TheRunsTransfers, request: &str) -> String {
            let screen = the_screen(record, the_run);
            match screen.requests.iter().find(|row| row.id.as_str() == request) {
                None => "NO ROW".to_string(),
                Some(row) => format!(
                    "{} | {} | {} {}",
                    as_read(&row.name),
                    as_read(&row.count),
                    row.files.first().map(|file| file.state).unwrap_or_default(),
                    row.files
                        .first()
                        .and_then(|file| file.tail.as_ref().map(|tail| tail.as_str().to_string()))
                        .unwrap_or_default()
                ),
            }
        }

        fn waits(the_run: &TheRunsTransfers, request: &str) -> bool {
            the_run
                .what_is_under_way()
                .get(request)
                .is_some_and(|under_way| under_way.waits_for_the_connection)
        }

        fn staged(the_run: &TheRunsTransfers, request: &str) -> u64 {
            the_run
                .what_is_under_way()
                .get(request)
                .and_then(|under_way| under_way.moving.first().map(|file| file.report.bytes_staged))
                .unwrap_or(0)
        }

        fn verifying(the_run: &TheRunsTransfers, request: &str) -> bool {
            the_run
                .what_is_under_way()
                .get(request)
                .and_then(|under_way| under_way.moving.first().map(|file| file.report.verifying_a_prefix_of.is_some()))
                .unwrap_or(false)
        }

        /// **Where an upload's cut falls.**
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum WhereTheUploadIsCut {
            /// The line silent as the upload is prepared: the walk of the
            /// destination meets it.
            BeforeItsSetUp,
            /// Half the file staged.
            MidFile { brief: bool },
            /// **The carry-on's own check of the staged part**, after a first
            /// cut mid-file.
            AtThePrefixCheck,
            /// **The moment publication's pre-state is written**
            /// (`Server protocol and transfers` S15): the rename never leaves.
            AtPublication,
            /// **Publication's answer lost**: the rename reaches the machine
            /// and is made, and its answer never arrives.
            ThePublicationsAnswerLost { brief: bool },
            /// The same, publishing over a file already there, answered
            /// *Replace*.
            ThePublicationsAnswerLostOverAFileAlreadyThere,
        }

        /// **What one upload's cut came to.**
        struct WhatTheUploadsCutCameTo {
            /// From the cut to the row reading *waiting for the connection*,
            /// where it came to that.
            cut_to_waiting: Option<Duration>,
            /// What the row read while the line was down.
            while_down: String,
            came_to: Option<smoozy_desktop::uploading::WhatTheUploadCameTo>,
            /// How many fresh connections were lent across the upload.
            repairs: usize,
            /// Whether the file at its final name was ever anything but the
            /// file already there or the file sent, whole.
            the_final_name_was_ever_wrong: Option<String>,
        }

        /// **One upload of `size` bytes into `into`, cut where `cut` says**,
        /// the line brought back once the break is declared (or after a brief
        /// silence), and the point lending a fresh connection.
        #[allow(clippy::too_many_arguments)]
        async fn an_upload_cut(
            said: &str,
            cut: WhereTheUploadIsCut,
            line: &Arc<TheLine>,
            point: &ThePoint,
            record: &Arc<TheRecord>,
            server_row: &Identity,
            source: &Path,
            into: &Path,
            size: usize,
        ) -> WhatTheUploadsCutCameTo {
            let the_run = Arc::new(TheRunsTransfers::for_this_run());
            let capped = TheCap::Of(BytesPerSecond(8_000_000));
            the_run.the_caps_saved_are(capped, capped);
            let landed = into.join(source.file_name().expect("a name"));
            let already_there = cut == WhereTheUploadIsCut::ThePublicationsAnswerLostOverAFileAlreadyThere;
            if already_there {
                a_file_at(&landed, size / 3, 77);
            }
            let asked = WhatIsBeingUploaded {
                server_row: server_row.clone(),
                installation: Some(an_identity(IdentityKind::Installation)),
                destination: bytes_of(into),
                chosen: vec![source.to_path_buf()],
                run: an_identity(IdentityKind::Run),
            };
            let as_a_door: Arc<dyn TheRecordAnUploadWrites> = record.clone();
            let lent_at_first = point.lent();

            if cut == WhereTheUploadIsCut::BeforeItsSetUp {
                line.goes_silent(Which::Every);
                let began = Instant::now();
                let prepared = tokio::time::timeout(
                    Duration::from_secs(60),
                    an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref()),
                )
                .await
                .expect("the preparation ended within a minute");
                let took = began.elapsed();
                let refused = prepared.as_ref().err().cloned();
                println!("{said}: the preparation came to {refused:?} in {took:?}");
                let request = prepared.as_ref().ok().map(|ready| ready.request.record.as_text());
                let while_down = match &refused {
                    Some(why) => format!("REFUSED {why:?}"),
                    None => "IT WAS PREPARED".to_string(),
                };
                drop(prepared);
                // **What the screen shows of a request that never started.**
                let rows = the_screen(record, &the_run)
                    .requests
                    .iter()
                    .map(|row| format!("{} | {}", as_read(&row.name), as_read(&row.count)))
                    .collect::<Vec<_>>();
                println!("{said}: the screen holds {rows:?}");
                let _ = request;
                line.comes_back(Which::Every);
                point.is_lending(&format!("{said}: lent again"), lent_at_first + 1).await;
                let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
                    .await
                    .expect("the upload is prepared once the line is back");
                assert_eq!(
                    the_answer_is_applied(&mut ready, None, &point.sessions, as_a_door.as_ref()).await,
                    WhatTheAnswerLeft::ItGoes
                );
                let came_to = the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold).await;
                return WhatTheUploadsCutCameTo {
                    cut_to_waiting: Some(took),
                    while_down,
                    came_to: Some(came_to),
                    repairs: point.lent() - lent_at_first,
                    the_final_name_was_ever_wrong: None,
                };
            }

            let mut ready = an_upload_is_prepared(&asked, &point.sessions, as_a_door.as_ref())
                .await
                .expect("the upload is prepared");
            let answer = already_there.then_some(smoozy_file_browser::already_exists::CollisionAnswer::Replace);
            assert_eq!(
                the_answer_is_applied(&mut ready, answer, &point.sessions, as_a_door.as_ref()).await,
                WhatTheAnswerLeft::ItGoes
            );
            let request = ready.request.record.as_text();

            // **The final name, watched**: the file already there, or the file
            // sent, whole — never anything between, and never missing once
            // something stood there.
            let watching = Arc::new(AtomicBool::new(true));
            let ever_wrong = Arc::new(Mutex::new(None::<String>));
            let watcher = {
                let (watching, ever_wrong, landed) = (Arc::clone(&watching), Arc::clone(&ever_wrong), landed.clone());
                std::thread::spawn(move || {
                    let mut stood = already_there;
                    while watching.load(Ordering::SeqCst) {
                        match std::fs::metadata(&landed) {
                            Ok(held) => {
                                let whole_old = held.len() == (size / 3) as u64;
                                let whole_new = held.len() == size as u64;
                                if !whole_old && !whole_new {
                                    *ever_wrong.lock().unwrap() = Some(format!("{} bytes at the final name", held.len()));
                                }
                                stood = true;
                            }
                            Err(_) if stood => {
                                *ever_wrong.lock().unwrap() = Some("the final name stood empty".to_string());
                            }
                            Err(_) => {}
                        }
                        std::thread::sleep(Duration::from_micros(300));
                    }
                })
            };

            // **The cut.**
            let cut_at = Arc::new(Mutex::new(None::<Instant>));
            match cut {
                WhereTheUploadIsCut::AtPublication
                | WhereTheUploadIsCut::ThePublicationsAnswerLost { .. }
                | WhereTheUploadIsCut::ThePublicationsAnswerLostOverAFileAlreadyThere => {
                    let (line, cut_at) = (Arc::clone(line), Arc::clone(&cut_at));
                    record.when(move |connection| {
                        let pending: i64 = connection
                            .query_row(
                                "SELECT count(*) FROM transfer_record WHERE publication_staging_path IS NOT NULL",
                                [],
                                |row| row.get(0),
                            )
                            .unwrap_or(0);
                        if pending > 0 {
                            if cut == WhereTheUploadIsCut::AtPublication {
                                line.goes_silent(Which::Every);
                            } else {
                                line.holds_the_answers(Which::Every);
                            }
                            *cut_at.lock().unwrap() = Some(Instant::now());
                        }
                        pending > 0
                    });
                }
                _ => {}
            }
            let cutting = {
                let (line, the_run, request, cut_at) =
                    (Arc::clone(line), Arc::clone(&the_run), request.clone(), Arc::clone(&cut_at));
                async move {
                    match cut {
                        WhereTheUploadIsCut::MidFile { .. } | WhereTheUploadIsCut::AtThePrefixCheck => {
                            until("half the file staged", Duration::from_secs(60), || {
                                staged(&the_run, &request) >= (size / 2) as u64
                            })
                            .await;
                            line.goes_silent(Which::Every);
                            *cut_at.lock().unwrap() = Some(Instant::now());
                        }
                        _ => {
                            until("the cut", Duration::from_secs(60), || cut_at.lock().unwrap().is_some()).await;
                        }
                    }
                    let at = cut_at.lock().unwrap().expect("the cut was made");
                    if matches!(
                        cut,
                        WhereTheUploadIsCut::MidFile { brief: true } | WhereTheUploadIsCut::ThePublicationsAnswerLost { brief: true }
                    ) {
                        tokio::time::sleep(A_BRIEF_SILENCE).await;
                        line.comes_back(Which::Every);
                        return (None, String::new());
                    }
                    let waiting = tokio::time::timeout(Duration::from_secs(30), async {
                        until("the row waits for the connection", Duration::from_secs(30), || {
                            waits(&the_run, &request) || !the_run.what_is_under_way().contains_key(&request)
                        })
                        .await;
                    })
                    .await
                    .ok()
                    .map(|()| at.elapsed());
                    let while_down = String::new();
                    if cut == WhereTheUploadIsCut::AtThePrefixCheck {
                        // **The repair, slow coming back**, so the staged
                        // part's check is caught under way.
                        line.carries_back_at(2_000_000);
                        line.comes_back(Which::Every);
                        until("the staged part is checked", Duration::from_secs(60), || {
                            verifying(&the_run, &request)
                        })
                        .await;
                        line.goes_silent(Which::Every);
                        let again = Instant::now();
                        until("the row waits for the connection again", Duration::from_secs(30), || {
                            waits(&the_run, &request) || !the_run.what_is_under_way().contains_key(&request)
                        })
                        .await;
                        println!("the check's own cut: waiting {:.2} s after it", again.elapsed().as_secs_f64());
                        line.carries_back_at(0);
                    }
                    line.comes_back(Which::Every);
                    (waiting, while_down)
                }
            };
            let while_down_read = {
                let (record, the_run, request) = (Arc::clone(record), Arc::clone(&the_run), request.clone());
                move || the_row_reads(&record, &the_run, &request)
            };
            let (came_to, (cut_to_waiting, _)) = tokio::join!(
                tokio::time::timeout(
                    Duration::from_secs(180),
                    the_upload_runs(ready, &point.sessions, &the_run, Arc::clone(&as_a_door), &NobodyIsTold),
                ),
                async {
                    let cut = cutting.await;
                    cut
                }
            );
            watching.store(false, Ordering::SeqCst);
            let _ = watcher.join();
            let while_down = while_down_read();
            let the_final_name_was_ever_wrong = ever_wrong.lock().unwrap().clone();
            WhatTheUploadsCutCameTo {
                cut_to_waiting,
                while_down,
                came_to: came_to.ok(),
                repairs: point.lent() - lent_at_first,
                the_final_name_was_ever_wrong,
            }
        }

        /// **An upload cut at every point the sweep names but publication's
        /// own**, the line silent (not closed) and then back: before its setup,
        /// mid-file, at the carry-on's own check of the staged part, with
        /// publication's answer late, and publishing over a file already there
        /// with its answer lost. **Each must** wait visibly for the connection
        /// within the break's bound and never read *did not finish* or failed
        /// while it waits; land whole once the line is back, carried on or
        /// started over and said; leave nothing staged; and never let the final
        /// name hold anything but the file already there or the file sent,
        /// whole. **Under the window**, a brief silence ends nothing and the
        /// file carries on over the same connection.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn an_upload_cut_at_every_point_waits_for_the_line_and_lands_whole() {
            the_upload_is_cut_at(&[
                WhereTheUploadIsCut::BeforeItsSetUp,
                WhereTheUploadIsCut::MidFile { brief: true },
                WhereTheUploadIsCut::MidFile { brief: false },
                WhereTheUploadIsCut::AtThePrefixCheck,
                WhereTheUploadIsCut::ThePublicationsAnswerLost { brief: true },
                WhereTheUploadIsCut::ThePublicationsAnswerLostOverAFileAlreadyThere,
            ])
            .await;
        }

        /// **An upload whose publication reached the machine and whose answer
        /// did not** — the rename made, and the line gone before its answer
        /// came back; or the rename queued on the old connection when the line
        /// went, and delivered when it came back, after the desk had already
        /// let that connection go. **The file is whole at its final name, and
        /// the user must be told it uploaded.**
        ///
        /// `Server protocol and transfers` S15 reads the final file back and
        /// compares its whole content with the transfer's recorded proof.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn an_upload_whose_publication_answer_is_lost_reads_uploaded() {
            the_upload_is_cut_at(&[
                WhereTheUploadIsCut::AtPublication,
                WhereTheUploadIsCut::ThePublicationsAnswerLost { brief: false },
            ])
            .await;
        }

        async fn the_upload_is_cut_at(cuts: &[WhereTheUploadIsCut]) {

            let far_end = a_far_end_or_skip!("up", TheFarEndsShape::default());
            let line = TheLine::to(&far_end).await;
            let point = ThePoint::over(&line, &far_end, false).await;
            let record = TheRecord::at(&far_end.scratch.folder("desk").join("smoozy.sqlite3"));
            let server_row = a_server_row_in(&record);
            let here = far_end.scratch.folder("here");
            let size = 24_000_000;
            let mut missed = Vec::new();
            for (at, cut) in cuts.iter().copied().enumerate() {
                let said = format!("UP {cut:?}");
                if !only_this_cut(&said) {
                    continue;
                }
                line.comes_back(Which::Every);
                point.is_lending(&format!("{said}: lending"), point.lent()).await;
                let source = here.join(format!("scene-{at}.blend"));
                a_file_at(&source, size, at as u8);
                let into = far_end.home.join(format!("up-{at}"));
                std::fs::create_dir_all(&into).expect("the destination");
                let came = an_upload_cut(&said, cut, &line, &point, &record, &server_row, &source, &into, size).await;
                let landed = into.join(source.file_name().unwrap());
                println!(
                    "{said}: came to {:?}; waiting {:?} after the cut; {} repairs; the row reads {}",
                    came.came_to, came.cut_to_waiting, came.repairs, came.while_down
                );
                let brief = matches!(
                    cut,
                    WhereTheUploadIsCut::MidFile { brief: true } | WhereTheUploadIsCut::ThePublicationsAnswerLost { brief: true }
                );
                a_figure!(
                    missed,
                    came.came_to.as_ref().is_some_and(|came_to| came_to.uploaded == 1 && came_to.failed.is_empty()),
                    "{said}: the upload came to {:?}",
                    came.came_to
                );
                a_figure!(missed, it_is_the_file(&landed, size, at as u8), "{said}: the file on the machine is not the file sent");
                let left = left_staged(&into);
                a_figure!(missed, left.is_empty(), "{said}: left staged: {left:?}");
                a_figure!(
                    missed,
                    came.the_final_name_was_ever_wrong.is_none(),
                    "{said}: the final name held {:?}",
                    came.the_final_name_was_ever_wrong
                );
                if brief {
                    a_figure!(missed, came.repairs == 0, "{said}: a brief silence ended the connection");
                } else {
                    a_figure!(missed, came.repairs >= 1, "{said}: the line's cut ended no connection");
                    match came.cut_to_waiting {
                        Some(took) => ended_within(&format!("{said}: waiting for the connection"), &mut missed, took, the_bound()),
                        None => missed.push(format!("{said}: the row never read waiting for the connection")),
                    }
                }
            }
            the_figures_held("AN UPLOAD, CUT", missed);
        }
    
        /// **Where a download's cut falls.**
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum WhereTheDownloadIsCut {
            /// The line silent as the download is prepared: the enumeration
            /// meets it.
            BeforeItsSetUp,
            /// Half the file staged on this computer.
            MidFile { brief: bool },
            /// **The carry-on's own reading of the file on the server**
            /// (`Server protocol and transfers` S13's four values), after a
            /// first cut mid-file.
            AtTheSourcesReading,
            /// Mid-file, onto a file already on this computer, answered
            /// *Replace*: that file must stand whole until publication.
            MidFileOverAFileAlreadyThere,
        }

        /// **A download cut at every point the sweep names**, the line silent
        /// and then back: before its setup, mid-file, at the carry-on's own
        /// reading of the file on the server, and over a file already on this
        /// computer. Each must wait visibly within the break's bound, land
        /// whole once the line is back, leave nothing staged, and never let
        /// the final name on this computer hold a partial file. **Under the
        /// window**, a brief silence ends nothing.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn a_download_cut_at_every_point_waits_for_the_line_and_lands_whole() {

            let far_end = a_far_end_or_skip!("down", TheFarEndsShape::default());
            let signals = TheSignals::at(&far_end);
            let line = TheLine::to(&far_end).await;
            let point = ThePoint::over(&line, &far_end, false).await;
            let record = TheRecord::at(&far_end.scratch.folder("desk").join("smoozy.sqlite3"));
            let server_row = a_server_row_in(&record);
            let size = 24_000_000;
            let mut missed = Vec::new();
            for (at, cut) in [
                WhereTheDownloadIsCut::BeforeItsSetUp,
                WhereTheDownloadIsCut::MidFile { brief: true },
                WhereTheDownloadIsCut::MidFile { brief: false },
                WhereTheDownloadIsCut::AtTheSourcesReading,
                WhereTheDownloadIsCut::MidFileOverAFileAlreadyThere,
            ]
            .into_iter()
            .enumerate()
            {
                let said = format!("DOWN {cut:?}");
                if !only_this_cut(&said) {
                    continue;
                }
                line.comes_back(Which::Every);
                point.is_lending(&format!("{said}: lending"), point.lent()).await;
                let lent_at_first = point.lent();
                let on_the_machine = far_end.home.join(format!("down-{at}"));
                std::fs::create_dir_all(&on_the_machine).expect("the folder on the machine");
                let source = on_the_machine.join("frames.exr");
                a_file_at(&source, size, 40 + at as u8);
                let chosen = far_end.scratch.folder(&format!("chosen-{at}"));
                let landed = chosen.join("frames.exr");
                let already_there = cut == WhereTheDownloadIsCut::MidFileOverAFileAlreadyThere;
                if already_there {
                    a_file_at(&landed, size / 3, 99);
                }
                let the_run = Arc::new(TheRunsTransfers::for_this_run());
                let capped = TheCap::Of(BytesPerSecond(8_000_000));
                the_run.the_caps_saved_are(capped, capped);
                let asked = || WhatIsBeingDownloaded {
                    server_row: server_row.clone(),
                    installation: None,
                    destination: bytes_of(&chosen),
                    picked: vec![bytes_of(&source)],
                    run: an_identity(IdentityKind::Run),
                };
                let door: &dyn smoozy_desktop::downloading::TheRecordADownloadWrites = record.as_ref();
                if cut == WhereTheDownloadIsCut::BeforeItsSetUp {
                    line.goes_silent(Which::Every);
                    let began = Instant::now();
                    let prepared = tokio::time::timeout(
                        Duration::from_secs(60),
                        a_download_is_prepared(asked(), &point.sessions, door, &NoJobIsKnown),
                    )
                    .await
                    .expect("the preparation ended within a minute");
                    let took = began.elapsed();
                    println!("{said}: the preparation came to {:?} in {took:?}", prepared.as_ref().err());
                    ended_within(&format!("{said}: the preparation"), &mut missed, took, the_bound());
                    a_figure!(
                        missed,
                        matches!(
                            prepared,
                            Err(smoozy_desktop::downloading::WhyTheDownloadDidNotStart::TheServerLentNothing(_))
                        ),
                        "{said}: the preparation came to {:?}, not the server lending nothing",
                        prepared.as_ref().err()
                    );
                    a_figure!(missed, !landed.exists(), "{said}: something landed");
                    line.comes_back(Which::Every);
                    point.is_lending(&format!("{said}: lent again"), lent_at_first + 1).await;
                }
                let mut ready = a_download_is_prepared(asked(), &point.sessions, door, &NoJobIsKnown)
                    .await
                    .expect("the download is prepared");
                if already_there {
                    smoozy_desktop::downloading::the_answer_is_applied(
                        &mut ready,
                        door,
                        smoozy_file_browser::already_exists::CollisionAnswer::Replace,
                    )
                    .expect("the answer is written");
                } else {
                    the_request_goes_without_a_question(&mut ready, door).expect("the dispositions are written");
                }
                let request = ready.request.record.as_text();
                // **The final name on this computer, watched.**
                let watching = Arc::new(AtomicBool::new(true));
                let ever_wrong = Arc::new(Mutex::new(None::<String>));
                let watcher = {
                    let (watching, ever_wrong, landed) = (Arc::clone(&watching), Arc::clone(&ever_wrong), landed.clone());
                    std::thread::spawn(move || {
                        while watching.load(Ordering::SeqCst) {
                            if let Ok(held) = std::fs::metadata(&landed) {
                                // **Smoozy's own reservation**, an empty file
                                // at a name nothing stood at (`Desktop app and
                                // delivery` C10), is the one other thing that
                                // may stand there.
                                let the_reservation = held.len() == 0 && !already_there;
                                if held.len() != (size / 3) as u64 && held.len() != size as u64 && !the_reservation {
                                    *ever_wrong.lock().unwrap() = Some(format!("{} bytes at the final name", held.len()));
                                }
                            } else if already_there {
                                *ever_wrong.lock().unwrap() = Some("the file already there went".to_string());
                            }
                            std::thread::sleep(Duration::from_micros(300));
                        }
                    })
                };
                let cutting = {
                    let (line, the_run, request) = (Arc::clone(&line), Arc::clone(&the_run), request.clone());
                    let signals = &signals;
                    async move {
                        if cut == WhereTheDownloadIsCut::BeforeItsSetUp {
                            return None;
                        }
                        until("half the file staged", Duration::from_secs(60), || {
                            staged(&the_run, &request) >= (size / 2) as u64
                        })
                        .await;
                        line.goes_silent(Which::Every);
                        let at = Instant::now();
                        if cut == (WhereTheDownloadIsCut::MidFile { brief: true }) {
                            tokio::time::sleep(A_BRIEF_SILENCE).await;
                            line.comes_back(Which::Every);
                            return None;
                        }
                        until("the row waits for the connection", Duration::from_secs(30), || {
                            waits(&the_run, &request) || !the_run.what_is_under_way().contains_key(&request)
                        })
                        .await;
                        let waiting = at.elapsed();
                        if cut == WhereTheDownloadIsCut::AtTheSourcesReading {
                            // **The carry-on's reading of the server's file is
                            // where the line goes again.**
                            let silenced = Arc::clone(&line);
                            let asked_before = signals.heard("stat");
                            signals.when("stat", move || silenced.goes_silent(Which::Every));
                            line_back_after_a_break(&line).await;
                            until("the source is read again", Duration::from_secs(30), || {
                                signals.heard("stat") > asked_before
                            })
                            .await;
                            until("the row waits for the connection again", Duration::from_secs(30), || {
                                waits(&the_run, &request) || !the_run.what_is_under_way().contains_key(&request)
                            })
                            .await;
                        }
                        line.comes_back(Which::Every);
                        Some(waiting)
                    }
                };
                let (came_to, waiting) = tokio::join!(
                    tokio::time::timeout(
                        Duration::from_secs(180),
                        the_download_runs(&mut ready, &point.sessions, &the_run, door),
                    ),
                    cutting
                );
                watching.store(false, Ordering::SeqCst);
                let _ = watcher.join();
                let came_to = came_to.ok();
                let repairs = point.lent() - lent_at_first;
                println!("{said}: came to {came_to:?}; waiting {waiting:?} after the cut; {repairs} repairs");
                a_figure!(
                    missed,
                    came_to.as_ref().is_some_and(|came_to| came_to.fetched == 1 && came_to.failed.is_empty()),
                    "{said}: the download came to {came_to:?}"
                );
                a_figure!(missed, it_is_the_file(&landed, size, 40 + at as u8), "{said}: the file here is not the server's");
                let left = left_staged(&chosen);
                a_figure!(missed, left.is_empty(), "{said}: left staged: {left:?}");
                let wrong = ever_wrong.lock().unwrap().clone();
                a_figure!(missed, wrong.is_none(), "{said}: the final name held {wrong:?}");
                match cut {
                    WhereTheDownloadIsCut::MidFile { brief: true } => {
                        a_figure!(missed, repairs == 0, "{said}: a brief silence ended the connection")
                    }
                    WhereTheDownloadIsCut::BeforeItsSetUp => {}
                    _ => match waiting {
                        Some(took) => ended_within(&format!("{said}: waiting for the connection"), &mut missed, took, the_bound()),
                        None => missed.push(format!("{said}: the row never read waiting for the connection")),
                    },
                }
            }
            the_figures_held("A DOWNLOAD, CUT", missed);
        }

        /// **Every frame standing at its final name under `folder`**, wherever
        /// the arrangement lands them, Smoozy's staging folders left out.
        fn the_frames_under(folder: &Path) -> Vec<PathBuf> {
            let mut found = Vec::new();
            let mut pending = vec![folder.to_path_buf()];
            while let Some(at) = pending.pop() {
                let Ok(entries) = std::fs::read_dir(&at) else { continue };
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if !path.to_string_lossy().contains(".smoozy-staging") {
                            pending.push(path);
                        }
                    } else if path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.starts_with("frame_")) {
                        found.push(path);
                    }
                }
            }
            found
        }

        /// **Waits for the point's current connection to end, then brings the
        /// line back** — a repair's first attempt meeting a machine that
        /// answers again.
        async fn line_back_after_a_break(line: &Arc<TheLine>) {
            tokio::time::sleep(AT_THE_DESK.the_declaration_window().1 + Duration::from_millis(500)).await;
            line.comes_back(Which::Every);
        }

        /// **Frames coming down by auto-download, cut part-way** — the
        /// primary silenced, or `Servers` S14's additional connection alone at
        /// one session per connection — **and the ending walk cut before it
        /// has read the job's folder**. The frames already down are whole, the
        /// rest stay owed and no partial file stands at a frame's name; the
        /// pass ends within the break's bound; a cut walk changes nothing it
        /// has not read; and once the line is back the rest come down.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn frames_by_auto_download_cut_part_way_stay_owed_and_come_down_once_the_line_is_back() {
            use smoozy_desktop::downloading::{
                one_turn_of_an_arrangement, the_arrangement_is_made_at_submission, the_owed_fetches_come_down,
                AnArrangementOwingFetches, TheArrangementsKeeper, WhatTheSubmissionChose,
            };
            use smoozy_download::acting::fetching::TheFetchQueue;
            use smoozy_download::acting::probing::TheProbesCadence;
            use smoozy_download::acting::running::{TheArrangementsRun, WhatIsInHand, WhatTheTurnDid};
            use smoozy_download::arrangement::TheNarrowing;
            use smoozy_download::ending::ANarrowedEntry;
            use smoozy_download::vouching::{TheAttemptsOrder, TheOrder};
            use smoozy_download::watch::TheWindow;

            let mut missed = Vec::new();
            for alone in [false, true] {
                let said = if alone { "FRAMES, THE ADDITIONAL ALONE" } else { "FRAMES, THE LINE" };
                if !only_this_cut(said) {
                    continue;
                }
                let far_end = a_far_end_or_skip!(
                    if alone { "frames-a" } else { "frames" },
                    TheFarEndsShape { one_session_per_connection: alone, ..Default::default() }
                );
                let line = TheLine::to(&far_end).await;
                let point = ThePoint::over(&line, &far_end, alone).await;
                let record = TheRecord::at(&far_end.scratch.folder("desk").join("smoozy.sqlite3"));
                let server_row = a_server_row_in(&record);
                let frames = 120u32;
                let frame_size = 512 * 1024;
                let outputs = far_end.home.join("job/output");
                std::fs::create_dir_all(&outputs).expect("the job's output folder");
                for frame in 1..=frames {
                    a_file_at(&outputs.join(format!("frame_{frame:05}.png")), frame_size, frame as u8);
                }
                let chosen = far_end.scratch.folder("chosen");
                let the_run = Arc::new(TheRunsTransfers::for_this_run());
                let capped = TheCap::Of(BytesPerSecond(8_000_000));
                the_run.the_caps_saved_are(capped, capped);
                let run = an_identity(IdentityKind::Run);
                let job = an_identity(IdentityKind::Job);
                let door: &dyn smoozy_desktop::downloading::TheRecordADownloadWrites = record.as_ref();
                let mut arrangement = the_arrangement_is_made_at_submission(
                    &mut TheArrangementsKeeper { record: door },
                    true,
                    WhatTheSubmissionChose {
                        job: job.clone(),
                        server_row: server_row.clone(),
                        destination: bytes_of(&chosen),
                        narrowing: TheNarrowing::Everything,
                    },
                )
                .expect("the arrangement is made");
                let entries: Vec<ANarrowedEntry> = (1..=frames)
                    .map(|frame| ANarrowedEntry {
                        predicted_path: format!("frame_{frame:05}.png").into_bytes(),
                        frame: Some(frame as i64),
                    })
                    .collect();
                let order = TheOrder::The(TheAttemptsOrder::as_the_attempt_record_carries_it(
                    (1..=frames as i64).collect(),
                ));
                let folder = bytes_of(&outputs);
                let mut running = TheArrangementsRun::fresh();
                let mut queue = TheFetchQueue::at_a_server();
                let mut probing = TheProbesCadence::fresh();
                running.a_connection_began();
                running.the_ending_arrived(smoozy_queue_history::ending::Ending::Finished);
                let window = TheWindow {
                    the_desired_state_is_on: true,
                    a_connection_to_the_server_stands: true,
                    the_attempts_window_is_open: true,
                };
                let in_hand = || WhatIsInHand {
                    the_job_output_folder: &folder,
                    narrowed_entries: &entries,
                    the_whole_may_grow: false,
                    the_entries_the_narrowing_excludes: &[],
                    order: &order,
                    the_jobs_label: None,
                    the_first_frame_pair: None,
                    the_pairs_other_half_fetched: &[],
                    window,
                    the_placed_copies: None,
                    the_four_values_of_the_placed_copies: &[],
                    arrangements_walking_this_server: 1,
                };

                // ---- the ending walk, cut before it reads the job's folder
                if !alone {
                    line.goes_silent(Which::Every);
                    let began = Instant::now();
                    let walked = tokio::time::timeout(
                        Duration::from_secs(60),
                        one_turn_of_an_arrangement(&mut running, &point.sessions, door, &mut queue, &mut arrangement, in_hand()),
                    )
                    .await
                    .expect("the cut walk ended within a minute");
                    let took = began.elapsed();
                    println!("{said}: the cut walk came to {walked:?} in {took:?}");
                    ended_within(&format!("{said}: the cut walk"), &mut missed, took, the_bound());
                    a_figure!(
                        missed,
                        !matches!(walked, Ok(WhatTheTurnDid::ItWalked { .. })),
                        "{said}: the cut walk said it walked"
                    );
                    a_figure!(
                        missed,
                        queue.owed() == 0 && arrangement.reckoning.is_empty(),
                        "{said}: the cut walk changed the reckoning: {} owed, {} reckoned",
                        queue.owed(),
                        arrangement.reckoning.len()
                    );
                    line.comes_back(Which::Every);
                    point.is_lending(&format!("{said}: lent again"), 2).await;
                }
                let walked = one_turn_of_an_arrangement(&mut running, &point.sessions, door, &mut queue, &mut arrangement, in_hand())
                    .await
                    .expect("the ending walk ran");
                a_figure!(
                    missed,
                    queue.owed() == frames as usize,
                    "{said}: the walk once the line was back came to {walked:?}, {} owed",
                    queue.owed()
                );

                // ---- the frames, cut part-way
                let landed = |chosen: &Path| -> usize { the_frames_under(chosen).len() };
                let cut_at = Arc::new(Mutex::new(None::<Instant>));
                let watching = {
                    let (line, chosen, cut_at) = (Arc::clone(&line), chosen.clone(), Arc::clone(&cut_at));
                    tokio::spawn(async move {
                        let began = Instant::now();
                        while the_frames_under(&chosen).len() < 30 && began.elapsed() < Duration::from_secs(60)
                        {
                            tokio::time::sleep(Duration::from_millis(1)).await;
                        }
                        if alone {
                            line.goes_silent(Which::TheLast);
                        } else {
                            line.goes_silent(Which::Every);
                        }
                        *cut_at.lock().unwrap() = Some(Instant::now());
                        if alone {
                            // **The route for a fresh additional connection
                            // stays up**; the primary answers throughout.
                            line.the_route_is_up(true);
                        }
                    })
                };
                let lent_before = point.lent();
                let mut fetched = 0usize;
                let mut passes = 0;
                let mut first_pass_ended = None;
                let began = Instant::now();
                while fetched < frames as usize && began.elapsed() < Duration::from_secs(240) {
                    let mut owing = [AnArrangementOwingFetches {
                        running: &mut running,
                        queue: &mut queue,
                        probing: &mut probing,
                        arrangement: &mut arrangement,
                        in_hand: in_hand(),
                    }];
                    let came = the_owed_fetches_come_down(
                        &mut owing,
                        &point.sessions,
                        &the_run,
                        door,
                        &run,
                        Duration::from_secs(3_600),
                    )
                    .await;
                    passes += 1;
                    fetched += came.fetched;
                    if first_pass_ended.is_none() {
                        first_pass_ended = Some(Instant::now());
                        let here = landed(&chosen);
                        println!(
                            "{said}: the first pass fetched {} and {here} frames stand here; {} still owed",
                            came.fetched,
                            queue.owed()
                        );
                        a_figure!(missed, here >= came.fetched, "{said}: the pass counted {} and {here} stand", came.fetched);
                        if !alone {
                            a_figure!(
                                missed,
                                fetched < frames as usize && queue.owed() + fetched >= frames as usize,
                                "{said}: after the cut {fetched} fetched and {} owed of {frames}",
                                queue.owed()
                            );
                            line.comes_back(Which::Every);
                            point.is_lending(&format!("{said}: lent again"), lent_before + 1).await;
                        }
                    }
                    if came.fetched == 0 && passes > 3 {
                        break;
                    }
                }
                watching.await.expect("the watch");
                let cut = cut_at.lock().unwrap().expect("the cut was made");
                if let Some(ended) = first_pass_ended {
                    if !alone {
                        ended_within(&format!("{said}: the first pass"), &mut missed, ended.saturating_duration_since(cut), the_bound());
                    }
                }
                println!(
                    "{said}: {fetched} fetched in {passes} passes; {} lent in all; {} connections made",
                    point.lent(),
                    line.connections_made()
                );
                if alone {
                    a_figure!(
                        missed,
                        line.connections_made() >= 3,
                        "{said}: no fresh additional connection was made after the cut"
                    );
                }
                a_figure!(missed, fetched == frames as usize, "{said}: {fetched} of {frames} frames came down");
                for at in the_frames_under(&chosen) {
                    let frame: u32 = at
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .and_then(|stem| stem.strip_prefix("frame_"))
                        .and_then(|number| number.parse().ok())
                        .unwrap_or(0);
                    a_figure!(missed, it_is_the_file(&at, frame_size, frame as u8), "{said}: frame {frame} is not whole");
                }
                let left = left_staged(&chosen);
                a_figure!(missed, left.is_empty(), "{said}: left staged: {left:?}");
                if alone {
                    a_figure!(
                        missed,
                        point.lent() == lent_before,
                        "{said}: the primary was lent again, so it did not stand"
                    );
                }
            }
            the_figures_held("FRAMES BY AUTO-DOWNLOAD, CUT", missed);
        }
    }

    // =====================================================================
    // Connect, reconnect and the first checks — `Servers`' own driving and
    // sequence over the real transport — and the status reads a connected
    // server's pages take.
    // =====================================================================

    mod connecting {
        use super::*;
        use smoozy_desktop::servers::a_period::{what_the_driving_left, WhatTheDrivingLeft};
        use smoozy_desktop::servers::installation::{OverTheConnection, TheMachinesInstallationIdentity};
        use smoozy_server_protocol::transport::OpenFailure;
        use smoozy_servers::acting::{
            OpenConnections, SupplyTheCredential, TheActingHalf, TheServerBeingConnectedTo, Wait,
            WhatServersPublishes,
        };
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::{APeriod, HowTheDrivingEnded, TheDesktopsSide};
        use smoozy_servers::hardware::HardwareReading;
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        use smoozy_servers::key_moments::{TheTwoAnswers, WhoStartedIt};
        use smoozy_servers::period::{OpeningAct, WhoTookIt};
        use smoozy_servers::repair::{NextAttemptDue, PermanentRefusal, WhatTheRecordSays};
        use smoozy_servers::sequence::WhereItStopped;
        use smoozy_servers::store::{RecordedMachineRoot, RememberedHostKey, ServerRow};
        use smoozy_state::identities::InstallationCheck;
        use std::time::SystemTime;

        /// **`Servers` K1's transport, opened with the desk's keepalive
        /// figures, and `Servers` N2's deadline carried exactly as `Servers`' own wiring
        /// carries it** — and the live connection lent through a point of its
        /// own while the driving waits on it. **Where `real_figures`, it is
        /// `Servers`' own wiring itself**, K3's figures and all.
        struct TheOpener {
            sessions: ThisServersSessions,
            real_figures: bool,
            /// Called with the host-trust decision, inside the key exchange.
            during_the_exchange: Arc<Mutex<Option<Box<dyn FnMut() + Send>>>>,
        }

        struct TheExchangeWatched {
            inner: Box<dyn DecideHostTrust>,
            then: Arc<Mutex<Option<Box<dyn FnMut() + Send>>>>,
        }

        impl DecideHostTrust for TheExchangeWatched {
            fn decide(&mut self, offered: &[u8]) -> HostTrust {
                let decided = self.inner.decide(offered);
                if let Some(mut then) = self.then.lock().expect("the hook").take() {
                    then();
                }
                decided
            }
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
                let decide: Box<dyn DecideHostTrust> = Box::new(TheExchangeWatched {
                    inner: decide,
                    then: Arc::clone(&self.during_the_exchange),
                });
                if self.real_figures {
                    return smoozy_servers::wiring::TheTransport.open(endpoint, credential, decide, deadline).await;
                }
                match tokio::time::timeout(deadline, ssh::open(&endpoint, credential, AT_THE_DESK, decide)).await {
                    Ok(opened) => opened,
                    Err(_) => Err(OpenFailure::TheDeadlineExpired),
                }
            }

            fn ended(&mut self, connection: &Connection) -> Option<Ending> {
                connection.ended()
            }

            async fn wait_until_it_ends(&mut self, connection: &mut Connection) -> Ending {
                let none: TheAdditionalConnection<Connection> = Arc::new(|| Box::pin(async { None }));
                lend_until_it_ends(&self.sessions, connection, &none).await
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

        /// **What `Servers` published, each with its moment** — the server
        /// row's own value, the one the Servers page draws (`Servers` K6).
        #[derive(Clone, Default)]
        struct WhatWasPublished(Arc<Mutex<Vec<(Instant, ObservedValue)>>>, Arc<Mutex<Vec<RememberedHostKey>>>);

        impl WhatServersPublishes for WhatWasPublished {
            fn the_observed_value(&mut self, _server: &str, value: ObservedValue) {
                self.0.lock().expect("published").push((Instant::now(), value));
            }
            fn the_next_attempt_due(&mut self, _server: &str, _reading: NextAttemptDue) {}
            fn raise_the_connection_condition(&mut self, _server: &str, _says: WhatTheRecordSays) {}
            fn withdraw_the_connection_condition(&mut self, _server: &str) {}
            fn the_changed_identity_warning(&mut self, _server: &str) {}
            fn record_the_scanned_host_key(&mut self, _server: &str, key: RememberedHostKey) {
                self.1.lock().expect("published").push(key);
            }
        }

        impl WhatWasPublished {
            fn values(&self) -> Vec<(Instant, ObservedValue)> {
                self.0.lock().expect("published").clone()
            }

            /// The value published last.
            fn now(&self) -> Option<ObservedValue> {
                self.values().last().map(|(_, value)| *value)
            }

            /// When the value first left *connected* after `since`.
            fn left_connected_after(&self, since: Instant) -> Option<Instant> {
                self.values()
                    .into_iter()
                    .find(|(at, value)| *at >= since && *value != ObservedValue::Connected)
                    .map(|(at, _)| at)
            }
        }

        /// The desktop's side: the row as it stands, and what each primary
        /// connection read.
        #[derive(Clone)]
        struct TheDesk {
            row: ServerRow,
            read: Arc<Mutex<Vec<(InstallationCheck, Option<HardwareReading>)>>>,
        }

        impl TheDesktopsSide for TheDesk {
            fn the_row_now(&mut self, _server: &str) -> Option<ServerRow> {
                Some(self.row.clone())
            }
            fn the_connection_read(&mut self, _server: &str, check: InstallationCheck, hardware: Option<HardwareReading>) {
                self.read.lock().expect("read").push((check, hardware));
            }
        }

        /// **The desktop's clock, its pauses a twentieth as long** — the
        /// repair's fifteen seconds between attempts is not what is proved.
        struct AShortClock;
        impl Wait for AShortClock {
            async fn until(&mut self, how_long: Duration) {
                tokio::time::sleep(how_long / 20).await;
            }
            fn now(&mut self) -> SystemTime {
                SystemTime::now()
            }
        }

        fn the_row(line: &TheLine, far_end: &TheFarEnd, placed: bool) -> ServerRow {
            let mut row = ServerRow::saved(
                an_identity(IdentityKind::LocalServer).as_text(),
                Host::what_the_user_typed("127.0.0.1").expect("a host"),
                Port::what_the_user_typed(line.address.port()).expect("a port"),
                Username::what_the_user_typed(&the_user()).expect("a username"),
                KeyReference {
                    local_key_identifier: "key-1".into(),
                    fingerprint: "SHA256:aaa".into(),
                },
            );
            if placed {
                row.recorded_machine_root = Some(RecordedMachineRoot {
                    path: far_end.home.join("root").to_string_lossy().into_owned(),
                    the_package_areas_own_location: Some(far_end.package_area.to_string_lossy().into_owned()),
                    read_under: an_identity(IdentityKind::Installation).as_text(),
                });
                row.last_confirmed_installation_identity = Some(an_identity(IdentityKind::Installation).as_text());
            }
            row
        }

        /// **Where a connection's cut falls.**
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum WhereTheConnectIsCut {
            /// The line silent before the machine's banner: nothing answers.
            BeforeTheBanner,
            /// **Inside the key exchange**, at the host-trust decision, before
            /// authentication is asked.
            DuringTheExchange { brief: bool },
            /// **At `Servers` Q1 step 5's installation read**, the connection
            /// live and *connected* published.
            AtTheInstallationRead { brief: bool },
            /// **At step 6's hardware reading**, its second command.
            AtTheHardwareReading { brief: bool },
        }

        /// **What one period's cut came to.**
        struct WhatThePeriodCameTo {
            ended: Option<HowTheDrivingEnded>,
            /// From the cut to the period's end, or to the row leaving
            /// *connected*.
            took: Option<Duration>,
            published: WhatWasPublished,
            read: Vec<(InstallationCheck, Option<HardwareReading>)>,
            /// Whether the row reads *connected* again once the line is back.
            connected_again: bool,
        }

        /// **One period, driven by `Servers`' own driving over the real
        /// transport, cut where `cut` says**, the line back after a brief
        /// silence or once the break is declared.
        async fn a_period_cut(cut: WhereTheConnectIsCut) -> Option<WhatThePeriodCameTo> {
            let far_end = TheFarEnd::made("connect", TheFarEndsShape::default())?;
            let far_end = Arc::new(far_end);
            let signals = TheSignals::at(&far_end);
            let line = TheLine::to(&far_end).await;
            let placed = matches!(cut, WhereTheConnectIsCut::AtTheInstallationRead { .. });
            let row = the_row(&line, &far_end, placed);
            let published = WhatWasPublished::default();
            let desk = TheDesk {
                row: row.clone(),
                read: Arc::default(),
            };
            let during_the_exchange: Arc<Mutex<Option<Box<dyn FnMut() + Send>>>> = Arc::default();
            let cut_at = Arc::new(Mutex::new(None::<Instant>));
            let brief = matches!(
                cut,
                WhereTheConnectIsCut::DuringTheExchange { brief: true }
                    | WhereTheConnectIsCut::AtTheInstallationRead { brief: true }
                    | WhereTheConnectIsCut::AtTheHardwareReading { brief: true }
            );
            let the_cut = {
                let (line, cut_at) = (Arc::clone(&line), Arc::clone(&cut_at));
                move || {
                    line.goes_silent(Which::Every);
                    *cut_at.lock().unwrap() = Some(Instant::now());
                }
            };
            match cut {
                WhereTheConnectIsCut::BeforeTheBanner => the_cut.clone()(),
                WhereTheConnectIsCut::DuringTheExchange { .. } => {
                    *during_the_exchange.lock().unwrap() = Some(Box::new(the_cut.clone()));
                }
                WhereTheConnectIsCut::AtTheHardwareReading { .. } => {
                    signals.when("systemd-detect-virt", the_cut.clone());
                }
                WhereTheConnectIsCut::AtTheInstallationRead { .. } => {}
            }
            let reads = Arc::new(AtomicUsize::new(0));
            let installation = TheMachinesInstallationIdentity::for_this_server(&row, OverTheConnection)
                .reading_the_row_as_it_stands({
                    let (row, reads, the_cut) = (row.clone(), Arc::clone(&reads), the_cut.clone());
                    move || {
                        if placed && reads.fetch_add(1, Ordering::SeqCst) == 0 {
                            the_cut();
                        }
                        Some(row.clone())
                    }
                });
            let mut half = TheActingHalf {
                opener: TheOpener {
                    sessions: ThisServersSessions::for_one_server(),
                    real_figures: matches!(
                        cut,
                        WhereTheConnectIsCut::BeforeTheBanner | WhereTheConnectIsCut::DuringTheExchange { .. }
                    ),
                    during_the_exchange,
                },
                credentials: TheKey(far_end.client_key.clone()),
                installation,
                publishes: published.clone(),
                wait: AShortClock,
            };
            let mut period = APeriod::begun(
                &row.local_server_identifier,
                OpeningAct::TheUsersConnectAction,
                WhoTookIt::TheUser,
                SystemTime::now(),
            );
            let ending = period.end_it();
            let sessions = half.opener.sessions.clone();
            // **On a thread of its own, as the application drives a period**
            // (`crate::servers::a_period`), its sockets and clock the test's
            // own runtime's.
            let (driven, driving) = tokio::sync::oneshot::channel();
            let driver = {
                let (row, mut desk, runtime) = (row.clone(), desk.clone(), tokio::runtime::Handle::current());
                std::thread::spawn(move || {
                    let ended = runtime.block_on(period.drive(&row, &mut half, &mut desk));
                    let _ = driven.send((ended, Instant::now()));
                })
            };
            let mut driving = Box::pin(driving);
            // **The line back**: after a brief silence, or once the break or
            // the opening's end has been seen.
            let watching = {
                let (line, cut_at, published) = (Arc::clone(&line), Arc::clone(&cut_at), published.clone());
                tokio::spawn(async move {
                    let began = Instant::now();
                    loop {
                        let cut = *cut_at.lock().unwrap();
                        if let Some(at) = cut {
                            if brief {
                                tokio::time::sleep(A_BRIEF_SILENCE.saturating_sub(at.elapsed())).await;
                                line.comes_back(Which::Every);
                                return None;
                            }
                            // Past the window: wait for the row to leave
                            // *connected* (a break), or — a cut before the
                            // connection was made — for the opening to end.
                            let mut left = None;
                            while at.elapsed() < Duration::from_secs(40) {
                                let was_connected = published
                                    .values()
                                    .iter()
                                    .any(|(when, value)| *when <= at && *value == ObservedValue::Connected);
                                if was_connected {
                                    if let Some(when) = published.left_connected_after(at) {
                                        left = Some(when.saturating_duration_since(at));
                                        break;
                                    }
                                } else if published.now() == Some(ObservedValue::Disconnected) {
                                    break;
                                }
                                tokio::time::sleep(Duration::from_millis(5)).await;
                            }
                            line.comes_back(Which::Every);
                            return left;
                        }
                        if began.elapsed() > Duration::from_secs(60) {
                            return None;
                        }
                        tokio::time::sleep(Duration::from_millis(2)).await;
                    }
                })
            };
            let finished = tokio::time::timeout(Duration::from_secs(60), async {
                loop {
                    if driver.is_finished() {
                        return;
                    }
                    // End the period only after its expected check reading
                    // is back as well as the connection.
                    let cut_moment = *cut_at.lock().unwrap();
                    if let Some(at) = cut_moment {
                        let back = published
                            .values()
                            .iter()
                            .any(|(when, value)| *when > at && *value != ObservedValue::Connected)
                            || brief;
                        if back && at.elapsed() > A_BRIEF_SILENCE + Duration::from_millis(500)
                            && published.now() == Some(ObservedValue::Connected)
                            && sessions.a_connection_is_live()
                            && {
                                let reads = desk.read.lock().unwrap();
                                !reads.is_empty()
                                    && (matches!(cut, WhereTheConnectIsCut::AtTheInstallationRead { .. })
                                        || reads.last().is_some_and(|(_, hardware)| hardware.is_some()))
                                    && (brief || !matches!(cut, WhereTheConnectIsCut::AtTheHardwareReading { .. }) || reads.len() >= 2)
                            }
                        {
                            return;
                        }
                    }
                    tokio::time::sleep(Duration::from_millis(20)).await;
                }
            })
            .await;
            let left = watching.await.ok().flatten();
            let connected_again = published.now() == Some(ObservedValue::Connected);
            ending.at(smoozy_servers::period::PeriodEnding::TheUserDisconnects);
            let (ended, ended_at) = tokio::time::timeout(Duration::from_secs(30), &mut driving)
                .await
                .expect("the driving ended at the disconnect")
                .expect("the driving ran");
            let cut = *cut_at.lock().unwrap();
            let took = match (cut, &ended) {
                (Some(at), HowTheDrivingEnded::TheOpeningAttemptStopped(_)) => Some(ended_at.saturating_duration_since(at)),
                (Some(_), _) => left,
                _ => None,
            };
            if finished.is_err() {
                println!("{cut:?}: the period did not come to connected again within a minute");
            }
            let read = desk.read.lock().unwrap().clone();
            drop(signals);
            Some(WhatThePeriodCameTo {
                ended: Some(ended),
                took,
                published,
                read,
                connected_again,
            })
        }

        /// **A connect whose line goes silent before the machine's banner or
        /// inside the key exchange ends at `Servers` N2's twenty seconds, over
        /// `Servers`' own wiring** — *the connection was not made*, nothing
        /// claimed about why, no host key recorded, the row never reading
        /// *connected* — **and the same connect once the line is back
        /// connects.** Under the window, a brief silence in the exchange only
        /// delays the connection.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn a_connect_cut_before_it_is_authenticated_ends_at_its_twenty_seconds_and_connects_once_back() {
            let mut missed = Vec::new();
            for cut in [
                WhereTheConnectIsCut::BeforeTheBanner,
                WhereTheConnectIsCut::DuringTheExchange { brief: true },
                WhereTheConnectIsCut::DuringTheExchange { brief: false },
            ] {
                let said = format!("CONNECT {cut:?}");
                if !only_this_cut(&said) {
                    continue;
                }
                let Some(came) = a_period_cut(cut).await else {
                    println!("SKIPPED: no OpenSSH this test can run here");
                    return;
                };
                let values: Vec<ObservedValue> = came.published.values().iter().map(|(_, value)| *value).collect();
                println!("{said}: ended {:?} after {:?}; the row read {values:?}", came.ended, came.took);
                if cut == (WhereTheConnectIsCut::DuringTheExchange { brief: true }) {
                    a_figure!(missed, came.connected_again, "{said}: the connect did not connect: {values:?}");
                    continue;
                }
                a_figure!(
                    missed,
                    matches!(
                        came.ended,
                        Some(HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheConnectDeadlineExpired))
                    ),
                    "{said}: the connect ended {:?}, not at the connect deadline",
                    came.ended
                );
                if let Some(ended) = came.ended {
                    a_figure!(
                        missed,
                        what_the_driving_left(ended) == WhatTheDrivingLeft::TheConnectionWasNotMade,
                        "{said}: the page was left {:?}",
                        what_the_driving_left(ended)
                    );
                }
                match came.took {
                    Some(took) => ended_within(
                        &format!("{said}: the connect"),
                        &mut missed,
                        took,
                        smoozy_servers::repair::CONNECT_DEADLINE + Duration::from_secs(2),
                    ),
                    None => missed.push(format!("{said}: the connect never ended")),
                }
                a_figure!(
                    missed,
                    !values.contains(&ObservedValue::Connected),
                    "{said}: the row read connected over a connection never made: {values:?}"
                );
                a_figure!(
                    missed,
                    came.published.1.lock().unwrap().is_empty(),
                    "{said}: a host key was recorded for a connection never made"
                );
                a_figure!(missed, came.read.is_empty(), "{said}: something was read on a connection never made");
            }
            // **Once the line is back, the same connect connects** — every
            // case above ended with the line up, and a fresh period proves it.
            if only_this_cut("CONNECT ONCE BACK") {
                if let Some(came) = a_period_cut(WhereTheConnectIsCut::DuringTheExchange { brief: true }).await {
                    a_figure!(missed, came.connected_again, "a connect once the line is back did not connect");
                }
            }
            the_figures_held("A CONNECT, CUT BEFORE IT WAS MADE", missed);
        }

        /// **A connection whose line goes silent under its first checks** —
        /// `Servers` Q1 step 5's installation read, or step 6's hardware
        /// reading, the row already reading *connected* — **leaves
        /// *connected* within the break's bound**, never reading *connected*
        /// for good over a connection that is gone; claims nothing it did not
        /// read (no *absent* installation from a read that met the dead line,
        /// no hardware reading made of missing answers); and, on the repair,
        /// reads *connected* again with a whole reading. Under the window, a
        /// brief silence only delays the checks.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn the_first_checks_on_a_line_gone_silent_end_with_the_break_and_the_repair_reads_them_whole() {
            let mut missed = Vec::new();
            for cut in [
                WhereTheConnectIsCut::AtTheInstallationRead { brief: true },
                WhereTheConnectIsCut::AtTheInstallationRead { brief: false },
                WhereTheConnectIsCut::AtTheHardwareReading { brief: true },
                WhereTheConnectIsCut::AtTheHardwareReading { brief: false },
            ] {
                let said = format!("FIRST CHECKS {cut:?}");
                if !only_this_cut(&said) {
                    continue;
                }
                let Some(came) = a_period_cut(cut).await else {
                    println!("SKIPPED: no OpenSSH this test can run here");
                    return;
                };
                let values: Vec<ObservedValue> = came.published.values().iter().map(|(_, value)| *value).collect();
                let read: Vec<(InstallationCheck, bool)> =
                    came.read.iter().map(|(check, hardware)| (*check, hardware.is_some())).collect();
                println!(
                    "{said}: ended {:?}; left connected {:?} after the cut; the row read {values:?}; read {read:?}",
                    came.ended, came.took
                );
                let brief = matches!(
                    cut,
                    WhereTheConnectIsCut::AtTheInstallationRead { brief: true }
                        | WhereTheConnectIsCut::AtTheHardwareReading { brief: true }
                );
                if brief {
                    a_figure!(missed, came.connected_again, "{said}: the row did not stay connected: {values:?}");
                    a_figure!(
                        missed,
                        read.len() == 1 && (read[0].1 || matches!(cut, WhereTheConnectIsCut::AtTheInstallationRead { .. })),
                        "{said}: the checks read {read:?}"
                    );
                    continue;
                }
                match came.took {
                    Some(took) => ended_within(&format!("{said}: leaving connected"), &mut missed, took, the_bound()),
                    None => missed.push(format!("{said}: the row never left connected: {values:?}")),
                }
                // **Nothing claimed that was not read.** A read that met the
                // dead line is the fourth branch or nothing; a hardware
                // reading that met it is none.
                if matches!(cut, WhereTheConnectIsCut::AtTheInstallationRead { .. }) {
                    a_figure!(
                        missed,
                        came.read.iter().all(|(check, _)| *check != InstallationCheck::Absent)
                            || came.read.len() > 1 && came.read[0].0 != InstallationCheck::Absent,
                        "{said}: a read that met the dead line was taken as absent: {read:?}"
                    );
                    a_figure!(
                        missed,
                        matches!(
                            came.ended,
                            Some(HowTheDrivingEnded::TheOpeningAttemptStopped(_))
                                | Some(HowTheDrivingEnded::ThePeriodEnded(_))
                        ),
                        "{said}: the driving ended {:?}",
                        came.ended
                    );
                } else {
                    a_figure!(
                        missed,
                        came.read.first().is_some_and(|(_, hardware)| hardware.is_none()),
                        "{said}: the hardware reading cut part-way was kept: {read:?}"
                    );
                    a_figure!(
                        missed,
                        came.connected_again && came.read.len() >= 2 && came.read[1].1.is_some(),
                        "{said}: the repair did not read connected with a whole reading: {values:?}, {read:?}"
                    );
                }
            }
            the_figures_held("THE FIRST CHECKS, CUT", missed);
        }

        /// **The status reads a connected server's pages take** — the Blender
        /// versions installed, and the Set up card — **on a line gone silent**:
        /// each ends within the break's bound saying it could not ask, never
        /// *no Blender* or *no setup*; and once the line is back each answers.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn the_status_reads_on_a_line_gone_silent_say_they_could_not_ask_and_answer_once_back() {
            use smoozy_render_settings::acting::versions::TheVersionListAsk;
            let far_end = a_far_end_or_skip!("status", TheFarEndsShape::default());
            let supervisor = TheSupervisor::at(&far_end);
            let line = TheLine::to(&far_end).await;
            let point = ThePoint::over(&line, &far_end, false).await;
            let server = an_identity(IdentityKind::LocalServer).as_text();
            let area = far_end.package_area.to_string_lossy().into_owned();
            let mut missed = Vec::new();
            for brief in [true, false] {
                let said = if brief { "STATUS, UNDER THE WINDOW" } else { "STATUS, PAST THE WINDOW" };
                line.comes_back(Which::Every);
                point.is_lending(&format!("{said}: lending"), point.lent()).await;
                let before = point.lent();

                // ---- the Blender versions installed
                line.goes_silent(Which::Every);
                let cut = Instant::now();
                let back = {
                    let line = Arc::clone(&line);
                    tokio::spawn(async move {
                        if brief {
                            tokio::time::sleep(A_BRIEF_SILENCE).await;
                            line.comes_back(Which::Every);
                        }
                    })
                };
                let asked = tokio::time::timeout(
                    Duration::from_secs(60),
                    smoozy_desktop::submitting::the_blenders_on_the_machine(&point.sessions, &server, None),
                )
                .await
                .expect("the search ended within a minute");
                let took = cut.elapsed();
                let _ = back.await;
                println!("{said}: the search came to {asked:?} in {took:?}");
                if brief {
                    a_figure!(missed, matches!(asked, TheVersionListAsk::Answered(_)), "{said}: the search came to {asked:?}");
                    a_figure!(missed, point.lent() == before, "{said}: a brief silence ended the connection");
                } else {
                    ended_within(&format!("{said}: the search"), &mut missed, took, the_bound());
                    a_figure!(
                        missed,
                        matches!(asked, TheVersionListAsk::TheMachineCouldNotBeAsked),
                        "{said}: the search on a dead line came to {asked:?}, not *could not be asked*"
                    );
                    line.comes_back(Which::Every);
                    point.is_lending(&format!("{said}: lent again"), before + 1).await;
                    let again = smoozy_desktop::submitting::the_blenders_on_the_machine(&point.sessions, &server, None).await;
                    a_figure!(missed, matches!(again, TheVersionListAsk::Answered(_)), "{said}: the search once back came to {again:?}");
                }

                // ---- the Set up card
                let before = point.lent();
                let asked_before = supervisor.asked.lock().unwrap().len();
                line.goes_silent(Which::Every);
                let cut = Instant::now();
                let back = {
                    let line = Arc::clone(&line);
                    tokio::spawn(async move {
                        if brief {
                            tokio::time::sleep(A_BRIEF_SILENCE).await;
                            line.comes_back(Which::Every);
                        }
                    })
                };
                let card = tokio::time::timeout(
                    Duration::from_secs(60),
                    smoozy_desktop::setting_up::the_card_as_read(&point.sessions, &area, None),
                )
                .await
                .expect("the card's reading ended within a minute");
                let took = cut.elapsed();
                let _ = back.await;
                let asked_now = supervisor.asked.lock().unwrap().len() - asked_before;
                println!("{said}: the card came to {card:?} in {took:?}, the supervisor asked {asked_now} times");
                if brief {
                    a_figure!(missed, asked_now > 0, "{said}: the card's reading never reached the supervisor");
                    a_figure!(missed, point.lent() == before, "{said}: a brief silence ended the connection");
                } else {
                    ended_within(&format!("{said}: the card"), &mut missed, took, the_bound());
                    a_figure!(
                        missed,
                        card.the_run.is_none() && card.the_rest.is_none(),
                        "{said}: the card read on a dead line came to {card:?}"
                    );
                    line.comes_back(Which::Every);
                    point.is_lending(&format!("{said}: lent again"), before + 1).await;
                    let asked_before = supervisor.asked.lock().unwrap().len();
                    let _ = tokio::time::timeout(
                        Duration::from_secs(30),
                        smoozy_desktop::setting_up::the_card_as_read(&point.sessions, &area, None),
                    )
                    .await
                    .expect("the card once back answered within thirty seconds");
                    a_figure!(
                        missed,
                        supervisor.asked.lock().unwrap().len() > asked_before,
                        "{said}: the card once back never reached the supervisor"
                    );
                }
            }
            the_figures_held("THE STATUS READS", missed);
        }
    }

    // =====================================================================
    // Read the file, the dependency check, a row's confirmation, a linked
    // file's fixing, and the dependency upload — the sweep's *initial file
    // check, worker answer and returned streams*.
    // =====================================================================

    mod reading_and_checking {
        use super::*;
        use super::transfers::{a_file_at, left_staged};
        use smoozy_server_protocol::framing::{
            write_result_frame, Answered, Fields, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        };
        use smoozy_server_protocol::requests as asked;
        use std::collections::BTreeMap;

        /// **Where the machine's worker cuts the line**, once.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum TheWorkersCut {
            /// At the `nth` reading's first ask (counting from nought), before
            /// it is answered.
            AtTheAsk { nth: usize, brief: bool },
            /// **While the streams are read back**: the line goes `after` the
            /// recorded outcome is answered.
            WithTheStreams { after: Duration, brief: bool },
            /// **Once the fixed copy is made**, before the fixing is answered.
            AtTheFixing { brief: bool },
        }

        impl TheWorkersCut {
            fn brief(self) -> bool {
                match self {
                    TheWorkersCut::AtTheAsk { brief, .. }
                    | TheWorkersCut::WithTheStreams { brief, .. }
                    | TheWorkersCut::AtTheFixing { brief } => brief,
                }
            }
        }

        /// **The machine's file-inspection worker, standing in** — scripted:
        /// a check's reading is the document this test holds for that
        /// `.blend`; a file's reading is the printed reading this test holds;
        /// each answers *still in hand* at its first ask and its recorded
        /// outcome after, its two streams kept in the operation's folder.
        struct TheWorker {
            documents: BTreeMap<Vec<u8>, Vec<u8>>,
            the_files_reading: Vec<u8>,
            /// Bytes ahead of the document on the ordinary stream, so a
            /// stream's read-back can be caught under way.
            padding: usize,
            package_area: PathBuf,
            kept: BTreeMap<String, (Vec<u8>, usize, Vec<u8>, usize)>,
            readings: usize,
            cut: Option<TheWorkersCut>,
            line: Arc<TheLine>,
            /// When the cut was made.
            cut_at: Arc<Mutex<Option<Instant>>>,
        }

        impl TheWorker {
            fn the_cut_is_made(&mut self) {
                self.line.goes_silent(Which::Every);
                *self.cut_at.lock().unwrap() = Some(Instant::now());
                if self.cut.take().is_some_and(TheWorkersCut::brief) {
                    let line = Arc::clone(&self.line);
                    tokio::spawn(async move {
                        tokio::time::sleep(A_BRIEF_SILENCE).await;
                        line.comes_back(Which::Every);
                    });
                }
            }

            fn answers(&mut self, request: &Request) -> Reply {
                let answered = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0");
                let frame = |fields: Fields| Reply::Frame(write_result_frame(&answered.carrying(fields)));
                if request.name == asked::THE_DIGEST_OVER_A_BYTE_RANGE_REQUEST {
                    let file = request.fields.value(asked::THE_DIGESTED_FILE_FIELD).unwrap_or_default().to_vec();
                    return frame(match std::fs::read(std::ffi::OsStr::from_bytes(&file)) {
                        Ok(bytes) => Fields::none()
                            .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                            .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::IT_WAS_DIGESTED)
                            .and(
                                asked::THE_DIGEST_FIELD,
                                smoozy_server_protocol::transfers::integrity::Digest::of(&bytes).as_text(),
                            )
                            .and(asked::THE_BYTES_DIGESTED_FIELD, bytes.len().to_string()),
                        Err(_) => Fields::none()
                            .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                            .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::NOTHING_IS_THERE),
                    });
                }
                if request.name == asked::THE_FIXED_COPY_IS_PRODUCED_REQUEST {
                    let operation = request.identity.as_text();
                    if !self.kept.contains_key(&operation) {
                        let blend = request.fields.value(asked::THE_BLEND_FIELD).unwrap_or_default().to_vec();
                        let staging = request.fields.value(asked::THE_FIXINGS_STAGING_FIELD).unwrap_or_default().to_vec();
                        let pairs: Vec<smoozy_dependencies::remap::Pair> =
                            serde_json::from_slice(request.fields.value(asked::THE_FIXINGS_PAIRS_FIELD).unwrap_or(b"[]"))
                                .unwrap_or_default();
                        let _ = std::fs::copy(std::ffi::OsStr::from_bytes(&blend), std::ffi::OsStr::from_bytes(&staging));
                        let mut said =
                            format!("{}\n", smoozy_dependencies::acting::fixing_worker::THE_FIXINGS_START_MARK).into_bytes();
                        for pair in &pairs {
                            let line = serde_json::json!({
                                "kind": pair.kind, "setting": pair.setting, "match_text": pair.match_text,
                                "applied": true, "data_blocks": 1, "assignment_alone": "Established",
                                "reloaded": false, "values_found": []
                            });
                            said.extend_from_slice(
                                format!("{}{}\n", smoozy_dependencies::acting::material::THE_PER_PAIR_HEAD, line).as_bytes(),
                            );
                        }
                        let kept = self.keep(&operation, &said, b"");
                        self.kept.insert(operation.clone(), kept);
                        if matches!(self.cut, Some(TheWorkersCut::AtTheFixing { .. })) {
                            self.the_cut_is_made();
                        }
                    }
                    let (ordinary, ordinary_length, complaints, complaints_length) = self.kept[&operation].clone();
                    return frame(
                        Fields::none()
                            .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                            .and(asked::THE_ORDINARY_STREAM_FIELD, ordinary)
                            .and(asked::THE_ORDINARY_STREAM_LENGTH_FIELD, ordinary_length.to_string())
                            .and(asked::THE_COMPLAINTS_STREAM_FIELD, complaints)
                            .and(asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD, complaints_length.to_string())
                            .and(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD, asked::THE_ANSWER_IS_NO)
                            .and(asked::THE_EXIT_STATUS_FIELD, "0"),
                    );
                }
                if request.name == asked::THE_STOP_REQUEST {
                    return frame(Fields::none().and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND));
                }
                if request.name != asked::THE_READING_OF_A_BLEND_REQUEST {
                    return Reply::Frame(write_result_frame(
                        &Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0")
                            .saying_it_does_not_answer_that_request(&request.name),
                    ));
                }
                let operation = request.identity.as_text();
                if !self.kept.contains_key(&operation) {
                    // **The first ask starts the worker.**
                    let nth = self.readings;
                    self.readings += 1;
                    if matches!(self.cut, Some(TheWorkersCut::AtTheAsk { nth: at, .. }) if at == nth) {
                        self.the_cut_is_made();
                    }
                    let blend = request.fields.value(asked::THE_BLEND_FIELD).unwrap_or_default().to_vec();
                    let files = request.fields.text(asked::THE_WHICH_READING_FIELD) == Some(asked::THE_FILES_READING);
                    let (opening, closing, document) = if files {
                        (
                            smoozy_job_files::reading::THE_OPENING_SENTINEL,
                            smoozy_job_files::reading::THE_CLOSING_SENTINEL,
                            Some(self.the_files_reading.clone()),
                        )
                    } else {
                        (
                            smoozy_dependencies::acting::blender::THE_OPENING_SENTINEL,
                            smoozy_dependencies::acting::blender::THE_CLOSING_SENTINEL,
                            self.documents.get(&blend).cloned(),
                        )
                    };
                    let kept = match document {
                        Some(document) => {
                            let mut ordinary = vec![b' '; self.padding];
                            ordinary.extend_from_slice(b"\nBlender 5.2.2\n");
                            ordinary.extend_from_slice(opening);
                            ordinary.extend_from_slice(&document);
                            ordinary.extend_from_slice(closing);
                            ordinary.extend_from_slice(b"\nBlender quit\n");
                            self.keep(&operation, &ordinary, b"")
                        }
                        None => self.keep(&operation, b"", b"Error: cannot read file\n"),
                    };
                    self.kept.insert(operation, kept);
                    return frame(Fields::none().and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND));
                }
                if let Some(TheWorkersCut::WithTheStreams { after, .. }) = self.cut {
                    let (line, cut_at, brief) = (Arc::clone(&self.line), Arc::clone(&self.cut_at), self.cut.take().is_some_and(TheWorkersCut::brief));
                    tokio::spawn(async move {
                        tokio::time::sleep(after).await;
                        line.goes_silent(Which::Every);
                        *cut_at.lock().unwrap() = Some(Instant::now());
                        if brief {
                            tokio::time::sleep(A_BRIEF_SILENCE).await;
                            line.comes_back(Which::Every);
                        }
                    });
                }
                let (ordinary, ordinary_length, complaints, complaints_length) = self.kept[&operation].clone();
                let status = if ordinary_length > 0 { "0" } else { "1" };
                frame(
                    Fields::none()
                        .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                        .and(asked::THE_ORDINARY_STREAM_FIELD, ordinary)
                        .and(asked::THE_ORDINARY_STREAM_LENGTH_FIELD, ordinary_length.to_string())
                        .and(asked::THE_COMPLAINTS_STREAM_FIELD, complaints)
                        .and(asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD, complaints_length.to_string())
                        .and(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD, asked::THE_ANSWER_IS_NO)
                        .and(asked::THE_START_MARK_SEEN_FIELD, asked::THE_ANSWER_IS_YES)
                        .and(asked::THE_EXIT_STATUS_FIELD, status),
                )
            }

            /// Both streams kept in the operation's own folder.
            fn keep(&self, operation: &str, ordinary: &[u8], complaints: &[u8]) -> (Vec<u8>, usize, Vec<u8>, usize) {
                let folder = self.package_area.join("captured-streams").join(operation);
                std::fs::create_dir_all(&folder).expect("the operation's folder");
                let (at_ordinary, at_complaints) = (folder.join("ordinary"), folder.join("complaints"));
                std::fs::write(&at_ordinary, ordinary).expect("the ordinary stream is kept");
                std::fs::write(&at_complaints, complaints).expect("the complaints stream is kept");
                (bytes_of(&at_ordinary), ordinary.len(), bytes_of(&at_complaints), complaints.len())
            }
        }

        /// **The worker put to work**, answering through this test's
        /// supervisor.
        fn the_worker_answers(
            supervisor: &TheSupervisor,
            far_end: &TheFarEnd,
            line: &Arc<TheLine>,
            documents: BTreeMap<Vec<u8>, Vec<u8>>,
            the_files_reading: Vec<u8>,
        ) -> (Arc<Mutex<TheWorker>>, Arc<Mutex<Option<Instant>>>) {
            let cut_at = Arc::new(Mutex::new(None));
            let worker = Arc::new(Mutex::new(TheWorker {
                documents,
                the_files_reading,
                padding: 0,
                package_area: far_end.package_area.clone(),
                kept: BTreeMap::new(),
                readings: 0,
                cut: None,
                line: Arc::clone(line),
                cut_at: Arc::clone(&cut_at),
            }));
            let answering = Arc::clone(&worker);
            supervisor.answers_with(move |request| answering.lock().expect("the worker").answers(request));
            (worker, cut_at)
        }

        fn a_5_2() -> smoozy_render_settings::file_half::BlenderIdentity {
            smoozy_render_settings::file_half::BlenderIdentity {
                path: "/root/blender/blender-5.2.2/blender".to_string(),
                version: smoozy_server_setup::blender::Version { major: 5, minor: 2, patch: 2 },
            }
        }

        // ------------------------------------------------------ Read the file

        fn a_scene(name: &str) -> smoozy_render_settings::reading::SceneReading {
            use smoozy_render_settings::names::ExactName;
            use smoozy_render_settings::reading::*;
            let layer = |name: &str| ViewLayerReading {
                name: ExactName::as_the_reading_produced_it(name),
                renders_at: vec![(1, true)],
                animation: SwitchAnimation::None { value: true },
                own_sample_count: 1024,
                overrides_the_scenes_count: false,
            };
            SceneReading {
                name: ExactName::as_the_reading_produced_it(name),
                frame_start: 1,
                frame_end: 24,
                frame_step: 1,
                frame_count: 24,
                full_size: (1920, 1080),
                resolution_percentage: 100,
                size_at_the_files_percentage: (1920, 1080),
                engine_as_read: ExactName::as_the_reading_produced_it("CYCLES"),
                sample_counts: PerEngineCounts { cycles: 128, eevee: 64 },
                device: DeviceSetting::TheCards,
                early_stop: EarlyStop { adaptive_sampling_is_on: true, threshold: 0.01, minimum: 0 },
                time_limit: 0.0,
                layer_samples_rule: LayerSamplesRule::Use,
                saved_with: "5.2.2".to_string(),
                view_layers: vec![layer("ViewLayer")],
                pulls_in: Vec::new(),
                simplify: SimplifyReadBack {
                    use_simplify: false,
                    subdivision_render: 6,
                    child_particles_render: 1.0,
                    texture_limit_render: "OFF".to_string(),
                },
            }
        }

        /// **What a file's reading prints**, in the machine's own shape — the
        /// crate's own test's document, for one scene of 24 frames.
        fn a_printed_reading() -> Vec<u8> {
            use smoozy_job_files::manifest::CarriedAndNotInterpretedHere;
            use smoozy_job_files::reading::{
                ArtistsText, AtOneFrame, Destination, DestinationKind, Format, GatingConditions,
                HowManyViewsTheSceneWrites, RecordedLocation, TheVersionInHand, WhereItComesFrom, WhichScene,
            };
            use smoozy_job_files::reading_script::{TheObservedOutputs, ThePrintedReading};
            use smoozy_render_settings::names::ExactName;
            use smoozy_render_settings::reading::{OfferedEngineIdentifiers, SettingsHalf};
            let settings = SettingsHalf {
                scenes: vec![a_scene("Scene")],
                offered_engine_identifiers: OfferedEngineIdentifiers {
                    cycles: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                    eevee: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE")),
                },
                announced_add_on_case: None,
            };
            let layer = || ArtistsText::of(b"ViewLayer".to_vec());
            let gating = GatingConditions {
                compositing_switch: true,
                a_compositing_tree_is_present: false,
                a_live_final_output_node: false,
                the_final_output_node_is_muted: false,
                four_fives_use_nodes_switch: None,
                save_output_switch: None,
                the_render_goes_out_through_a_sequencer_holding_a_scene_strip: false,
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
                    the_view_transform: None,
                    the_tree_is_shared_with: None,
                    per_frame: (1..=24i64)
                        .map(|frame| AtOneFrame {
                            frame,
                            view_layers_that_render: vec![layer()],
                            gating: gating.clone(),
                            the_regular_output_paths_answered_path: Some(RecordedLocation::of(
                                format!("/ephemeral/out/f_{frame:04}.png").into_bytes(),
                            )),
                        })
                        .collect(),
                    recorded_locations: Vec::new(),
                    views: HowManyViewsTheSceneWrites::One,
                    the_compositing_tree: None,
                },
                settings: CarriedAndNotInterpretedHere(serde_json::to_value(&settings).expect("it renders")),
                dependencies: CarriedAndNotInterpretedHere(serde_json::json!({})),
                errors: std::collections::BTreeMap::new(),
                the_baked_caches_on_the_machine: Vec::new(),
                the_scene_holds_a_simulation_not_baked: false,
                the_scene_it_read: None,
                the_open_files_path: None,
                the_check_is_written_as_a_picture: false,
            };
            serde_json::to_vec(&printed).expect("it renders")
        }

        /// **Where a reading of the file's cut falls.**
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum WhereTheReadIsCut {
            /// **`Render Settings` Q14 item 2's reading of the four values**,
            /// which comes first.
            AtTheFourValues { brief: bool },
            AtTheWorkersAnswer { brief: bool },
            WhileTheStreamsComeBack { brief: bool },
        }

        /// **Read the file, cut at each of its three waits** — the four
        /// values, the worker's answer, and the streams' read-back — **ends
        /// within the break's bound with nothing stored**: the reading that
        /// stood before still stands and nothing half-read replaces it; the
        /// page is told the server could not be reached (or, at the four
        /// values, that the file could not be read). **Once the line is back
        /// the same press stores a fresh reading.** Under the window, each
        /// read only waits and stores.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn reading_the_file_cut_at_each_wait_stores_nothing_and_reads_once_back() {
            use smoozy_desktop::reading_the_file::{the_file_is_read, ThisServersMachine, WhatTheReadCameTo};
            use smoozy_desktop::surface::render_settings::{the_saved_with_version_lands, WhatTheRunHolds};
            use smoozy_render_settings::acting::pre_load::HowThePreLoadWent;
            use smoozy_render_settings::endings::Ending as ReadEnding;
            use smoozy_render_settings::file_half::{FileKey, Landed};
            let far_end = a_far_end_or_skip!("read", TheFarEndsShape::default());
            let supervisor = TheSupervisor::at(&far_end);
            let signals = TheSignals::at(&far_end);
            let line = TheLine::to(&far_end).await;
            let (worker, cut_at) = the_worker_answers(&supervisor, &far_end, &line, BTreeMap::new(), a_printed_reading());
            let mut point = ThePoint::over(&line, &far_end, false).await;
            let server = an_identity(IdentityKind::LocalServer).as_text();
            let the_blend = far_end.home.join("kitchen.blend");
            std::fs::write(&the_blend, b"BLENDER-v502 standing in for a scene").expect("the blend");
            let file = the_blend.to_string_lossy().into_owned();
            let key = FileKey {
                server: smoozy_desktop::surface::render_settings::the_servers_identity(&server),
                path: file.clone(),
            };
            let held = Mutex::new(WhatTheRunHolds::default());
            {
                let mut held = held.lock().unwrap();
                held.version_lists.insert(
                    server.clone(),
                    smoozy_server_setup::search::TheSearchResult {
                        entries: vec![smoozy_server_setup::search::ACandidate {
                            path: a_5_2().path,
                            answered: Some(a_5_2().version),
                        }],
                        answer: smoozy_server_setup::search::WhatTheAnswerSays {
                            the_search_is_by_name: false,
                            folders_refused: 0,
                            did_not_answer: Vec::new(),
                        },
                        at: std::time::SystemTime::now(),
                    },
                );
                the_saved_with_version_lands(&mut held, &server, &file, "5.2");
            }
            let area = far_end.package_area.to_string_lossy().into_owned();
            let the_standing = |held: &Mutex<WhatTheRunHolds>| {
                held.lock().unwrap().readings.standing(&key).map(|reading| reading.stored_at.taken_at())
            };
            let mut missed = Vec::new();

            // ---- a reading that stands before any cut
            let first = the_file_is_read(
                ThisServersMachine { sessions: &point.sessions, package_area: &area, server: &server },
                &held,
                &server,
                &file,
            )
            .await;
            println!("READ: the first reading came to {first:?}");
            assert!(
                matches!(first, WhatTheReadCameTo::ThePreLoad { went: HowThePreLoadWent::AReading { landed: Landed::Stored, .. }, .. }),
                "the first reading came to {first:?}"
            );

            for cut in [
                WhereTheReadIsCut::AtTheFourValues { brief: true },
                WhereTheReadIsCut::AtTheFourValues { brief: false },
                WhereTheReadIsCut::AtTheWorkersAnswer { brief: true },
                WhereTheReadIsCut::AtTheWorkersAnswer { brief: false },
                WhereTheReadIsCut::WhileTheStreamsComeBack { brief: true },
                WhereTheReadIsCut::WhileTheStreamsComeBack { brief: false },
            ] {
                let said = format!("READ {cut:?}");
                if !only_this_cut(&said) {
                    continue;
                }
                line.comes_back(Which::Every);
                point.is_lending(&format!("{said}: lending"), point.lent()).await;
                let brief = matches!(
                    cut,
                    WhereTheReadIsCut::AtTheFourValues { brief: true }
                        | WhereTheReadIsCut::AtTheWorkersAnswer { brief: true }
                        | WhereTheReadIsCut::WhileTheStreamsComeBack { brief: true }
                );
                *cut_at.lock().unwrap() = None;
                match cut {
                    WhereTheReadIsCut::AtTheFourValues { .. } => {
                        let (line, cut_at) = (Arc::clone(&line), Arc::clone(&cut_at));
                        signals.when("stat", move || {
                            line.goes_silent(Which::Every);
                            *cut_at.lock().unwrap() = Some(Instant::now());
                            if brief {
                                let line = Arc::clone(&line);
                                tokio::spawn(async move {
                                    tokio::time::sleep(A_BRIEF_SILENCE).await;
                                    line.comes_back(Which::Every);
                                });
                            }
                        });
                    }
                    WhereTheReadIsCut::AtTheWorkersAnswer { .. } => {
                        let mut worker = worker.lock().unwrap();
                        worker.cut = Some(TheWorkersCut::AtTheAsk { nth: worker.readings, brief });
                    }
                    WhereTheReadIsCut::WhileTheStreamsComeBack { .. } => {
                        // **A long stream on a slow line back**, so its read is
                        // caught under way: a fresh point over a line paced at
                        // a megabyte a second.
                        line.carries_back_at(1_000_000);
                        drop(point);
                        point = ThePoint::over(&line, &far_end, false).await;
                        line.carries_back_at(0);
                        let mut worker = worker.lock().unwrap();
                        worker.padding = 6_000_000;
                        worker.cut = Some(TheWorkersCut::WithTheStreams { after: Duration::from_millis(1_500), brief });
                    }
                }
                let before = point.lent();
                let standing_before = the_standing(&held);
                let came_to = tokio::time::timeout(
                    Duration::from_secs(90),
                    the_file_is_read(
                        ThisServersMachine { sessions: &point.sessions, package_area: &area, server: &server },
                        &held,
                        &server,
                        &file,
                    ),
                )
                .await
                .expect("the read ended within a minute and a half");
                let ended = Instant::now();
                worker.lock().unwrap().padding = 0;
                let took = cut_at.lock().unwrap().map(|at| ended.saturating_duration_since(at));
                let standing_after = the_standing(&held);
                println!("{said}: came to {came_to:?} {took:?} after the cut");
                if brief {
                    a_figure!(
                        missed,
                        matches!(came_to, WhatTheReadCameTo::ThePreLoad { went: HowThePreLoadWent::AReading { landed: Landed::Stored, .. }, .. }),
                        "{said}: the read came to {came_to:?}"
                    );
                    a_figure!(missed, point.lent() == before, "{said}: a brief silence ended the connection");
                    continue;
                }
                match took {
                    Some(took) => ended_within(&format!("{said}: the read"), &mut missed, took, the_bound()),
                    None => missed.push(format!("{said}: the cut was never made")),
                }
                let expected = match cut {
                    WhereTheReadIsCut::AtTheFourValues { .. } => matches!(came_to, WhatTheReadCameTo::TheFourValuesCouldNotBeRead),
                    _ => matches!(
                        came_to,
                        WhatTheReadCameTo::ThePreLoad { went: HowThePreLoadWent::NoReading(ReadEnding::TheMachineCouldNotBeReached), .. }
                    ),
                };
                a_figure!(missed, expected, "{said}: the read on a dead line came to {came_to:?}");
                a_figure!(
                    missed,
                    standing_after == standing_before,
                    "{said}: the reading that stood was replaced ({standing_before:?} then {standing_after:?})"
                );
                line.comes_back(Which::Every);
                point.is_lending(&format!("{said}: lent again"), before + 1).await;
                let again = the_file_is_read(
                    ThisServersMachine { sessions: &point.sessions, package_area: &area, server: &server },
                    &held,
                    &server,
                    &file,
                )
                .await;
                a_figure!(
                    missed,
                    matches!(again, WhatTheReadCameTo::ThePreLoad { went: HowThePreLoadWent::AReading { landed: Landed::Stored, .. }, .. }),
                    "{said}: the read once back came to {again:?}"
                );
            }
            the_figures_held("READ THE FILE, CUT", missed);
        }
    
        // ------------------------------------------------- the dependency check

        use smoozy_dependencies::bytes::{MachineLocation, StoredLocation};
        use smoozy_dependencies::record::{HoldingFile, MachineKey};
        use smoozy_dependencies::rows::RowState;
        use smoozy_dependencies::standing::Completion;
        use smoozy_desktop::checking_a_file::{the_check_runs, TheCheckCameTo, TheSittingIsOf, TheUsersOwnComputer};
        use smoozy_desktop::reading_the_file::ThisServersMachine;
        use smoozy_desktop::record::dependency_records::TheDependencyRecord;
        use smoozy_desktop::surface::the_dependency_check::WhatTheRunHolds as TheChecksRun;

        /// **One project on this computer, checked on the machine through the
        /// line**: the root, a linked library below it, and pictures.
        struct ACheckDesk {
            far_end: Arc<TheFarEnd>,
            _supervisor: TheSupervisor,
            signals: TheSignals,
            worker: Arc<Mutex<TheWorker>>,
            cut_at: Arc<Mutex<Option<Instant>>>,
            line: Arc<TheLine>,
            point: ThePoint,
            record: Arc<TheRecord>,
            held: Mutex<TheChecksRun>,
            machine_key: MachineKey,
            layout: smoozy_job_files::layout::TheLayout,
            the_run: Arc<smoozy_desktop::downloading::TheRunsTransfers>,
            run: Identity,
            root: PathBuf,
            project: PathBuf,
        }

        fn a_document_naming(references: serde_json::Value) -> Vec<u8> {
            serde_json::to_vec(&serde_json::json!({
                "references": references,
                "flat_list": [], "weak_references": [], "observed_stand_ins": []
            }))
            .expect("it writes")
        }

        fn an_image(recorded: &str) -> serde_json::Value {
            serde_json::json!({"kind": "Image", "datablock": recorded, "setting": "filepath",
                "recorded": recorded.as_bytes().to_vec(), "scenes": {"Established": ["Scene"]}, "source": "FILE"})
        }

        /// The desk, its project holding the root, `lib/props.blend` linked
        /// from it with a picture of its own, and `tex/wood.png` of `wood`
        /// bytes.
        async fn a_check_desk(named: &str, shape: TheFarEndsShape, wood: usize) -> Option<ACheckDesk> {
            use smoozy_job_files::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
            let far_end = Arc::new(TheFarEnd::made(named, shape)?);
            let project = far_end.scratch.folder("project");
            std::fs::create_dir_all(project.join("lib/tex")).expect("the project");
            std::fs::create_dir_all(project.join("tex")).expect("the project");
            let root = project.join("scene.blend");
            std::fs::write(&root, b"blend").expect("the root");
            let library = project.join("lib/props.blend");
            std::fs::write(&library, b"library").expect("the library");
            std::fs::write(project.join("lib/tex/brick.png"), b"brick").expect("a picture");
            a_file_at(&project.join("tex/wood.png"), wood, 11);
            let cache = project.join("blendcache_scene");
            std::fs::create_dir_all(&cache).expect("the cache folder");
            for frame in 1..=3 {
                a_file_at(&cache.join(format!("Cloth_00000{frame}_00.bphys")), 40_000, frame);
            }
            let machine_root = far_end.home.join("root");
            std::fs::create_dir_all(&machine_root).expect("the machine's root");
            let layout = TheLayout::of(
                MachineRoot::chosen(machine_root.to_str().expect("text")).expect("a root"),
                WhereThePackageAreaSits::UnderTheRoot,
            );
            let library_document = a_document_naming(serde_json::json!([an_image("//tex/brick.png")]));
            let mut documents = BTreeMap::new();
            documents.insert(
                bytes_of(&root),
                a_document_naming(serde_json::json!([
                    {"kind": "Library", "datablock": "props.blend", "setting": "filepath",
                     "recorded": b"//lib/props.blend".to_vec(), "scenes": {"Established": ["Scene"]}, "source": null},
                    an_image("//tex/wood.png"),
                    {"kind": "PhysicsPointCache", "datablock": "Cloth", "setting": "filepath",
                     "recorded": b"//blendcache_scene/".to_vec(), "scenes": {"Established": ["Scene"]}, "source": null}
                ])),
            );
            documents.insert(bytes_of(&library), library_document.clone());
            let mirror = smoozy_dependencies::landing::mirror_of(
                &layout,
                &StoredLocation::as_the_system_returned_it(&bytes_of(&library)),
            )
            .expect("the mirror fits");
            documents.insert(mirror.as_bytes().to_vec(), library_document);
            // **Smoozy's own program where the layout puts the package area**,
            // the entry point standing in as everywhere else.
            let package_area = PathBuf::from(layout.area(smoozy_job_files::layout::Area::Package));
            std::fs::create_dir_all(&package_area).expect("the package area");
            std::fs::copy(far_end.package_area.join("smoozy-server"), package_area.join("smoozy-server"))
                .expect("the entry point");
            std::fs::set_permissions(package_area.join("smoozy-server"), std::fs::Permissions::from_mode(0o755))
                .expect("it runs");
            let supervisor = TheSupervisor::at(&far_end);
            let signals = TheSignals::at(&far_end);
            let line = TheLine::to(&far_end).await;
            let (worker, cut_at) = the_worker_answers(&supervisor, &far_end, &line, documents, Vec::new());
            let point = ThePoint::over(&line, &far_end, shape.one_session_per_connection).await;
            let record = TheRecord::at(&far_end.scratch.folder("desk").join("smoozy.sqlite3"));
            let the_run = Arc::new(smoozy_desktop::downloading::TheRunsTransfers::for_this_run());
            let capped = smoozy_desktop::scheduling::TheCap::Of(smoozy_desktop::scheduling::BytesPerSecond(8_000_000));
            the_run.the_caps_saved_are(capped, capped);
            Some(ACheckDesk {
                far_end,
                _supervisor: supervisor,
                signals,
                worker,
                cut_at,
                line,
                point,
                record,
                held: Mutex::new(TheChecksRun::default()),
                machine_key: MachineKey::of(an_identity(IdentityKind::LocalServer), an_identity(IdentityKind::Installation)),
                layout,
                the_run,
                run: an_identity(IdentityKind::Run),
                root,
                project,
            })
        }

        impl ACheckDesk {
            fn the_sitting(&self) -> TheSittingIsOf {
                TheSittingIsOf {
                    server: self.machine_key.server_row.as_text(),
                    file: self.root.to_string_lossy().into_owned(),
                    root: MachineLocation::composed(bytes_of(&self.root)),
                    holding: HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(&bytes_of(
                        &self.root,
                    ))),
                    with: a_5_2(),
                    machine: self.machine_key.clone(),
                    layout: Some(self.layout.clone()),
                }
            }

            async fn a_check(&self) -> TheCheckCameTo {
                let sitting = self.the_sitting();
                let area = self.layout.area(smoozy_job_files::layout::Area::Package);
                let mut machine = ThisServersMachine {
                    sessions: &self.point.sessions,
                    package_area: &area,
                    server: &sitting.server,
                };
                tokio::time::timeout(
                    Duration::from_secs(90),
                    the_check_runs(&mut machine, self.record.as_ref(), &TheUsersOwnComputer, &self.held, &sitting, &|| {}),
                )
                .await
                .expect("the check ended within a minute and a half")
            }

            fn the_rows(&self) -> Vec<(RowState, String)> {
                let sitting = self.the_sitting();
                let key = format!("{}\u{1f}{}", sitting.server, sitting.file);
                self.held
                    .lock()
                    .unwrap()
                    .sittings
                    .get(&key)
                    .map(|sitting| {
                        sitting
                            .rows
                            .iter()
                            .map(|row| (row.state, String::from_utf8_lossy(row.row.ordering_key().as_bytes()).to_string()))
                            .collect()
                    })
                    .unwrap_or_default()
            }

            fn the_answers_kept(&self) -> usize {
                let sitting = self.the_sitting();
                self.record.read(|connection| {
                    TheDependencyRecord::over(connection)
                        .the_answers_of(&sitting.holding)
                        .map(|answers| answers.len())
                        .unwrap_or(0)
                })
            }

            fn the_standing(&self) -> Option<Completion> {
                let sitting = self.the_sitting();
                self.record.read(|connection| {
                    TheDependencyRecord::over(connection)
                        .the_standing(&sitting.root, &sitting.machine)
                        .ok()
                        .flatten()
                        .map(|standing| standing.completion)
                })
            }

            async fn a_send(&self, rows: Option<Vec<usize>>) -> smoozy_desktop::the_found_files_sent::WhatTheSendCameTo {
                use smoozy_desktop::the_found_files_sent::{the_found_files_go_up, the_rows_the_upload_sends, WhatIsSent};
                let of = self.the_sitting();
                let rows = rows.unwrap_or_else(|| {
                    let held = self.held.lock().unwrap();
                    let key = format!("{}\u{1f}{}", of.server, of.file);
                    the_rows_the_upload_sends(&held.sittings[&key].rows)
                });
                let asked = WhatIsSent { of, run: self.run.clone(), rows, replacing: false };
                let door: Arc<dyn TheRecordAnUploadWrites> = self.record.clone();
                tokio::time::timeout(
                    Duration::from_secs(180),
                    the_found_files_go_up(&asked, &self.point.sessions, &self.the_run, door, &self.held),
                )
                .await
                .expect("the send ended within three minutes")
            }

            fn the_mirror_of(&self, local: &Path) -> PathBuf {
                let mirror = smoozy_dependencies::landing::mirror_of(
                    &self.layout,
                    &StoredLocation::as_the_system_returned_it(&bytes_of(local)),
                )
                .expect("the mirror fits");
                PathBuf::from(std::ffi::OsStr::from_bytes(mirror.as_bytes()))
            }

            /// **The worker's cut, set; and the line back after it**, at once
            /// where it was brief or once the break has ended the lending.
            fn the_worker_cuts(&self, cut: TheWorkersCut) {
                *self.cut_at.lock().unwrap() = None;
                self.worker.lock().unwrap().cut = Some(cut);
            }

            fn took(&self, ended: Instant) -> Option<Duration> {
                self.cut_at.lock().unwrap().map(|at| ended.saturating_duration_since(at))
            }

            async fn back_and_lent(&self, said: &str, before: usize) {
                self.line.comes_back(Which::Every);
                self.point.is_lending(&format!("{said}: lent again"), before + 1).await;
            }
        }

        /// **The dependency check cut at each wait** — the root's worker
        /// answer, the root's streams coming back, and a linked library's
        /// reading below the root. **Cut at the root it gives nothing and
        /// keeps what an earlier check found; cut below the root the sitting
        /// stands with what was found and says it is incomplete**, the panel
        /// saying the connection dropped and the standing written so. Each
        /// ends within the break's bound, and once the line is back a check
        /// completes. Under the window, each only waits.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn the_dependency_check_cut_at_each_wait_keeps_its_findings_and_completes_once_back() {
            use smoozy_desktop::surface::the_dependency_check::the_panel;
            let Some(mut desk) = a_check_desk("check", TheFarEndsShape::default(), 5_000).await else {
                println!("SKIPPED: no OpenSSH this test can run here");
                return;
            };
            let mut missed = Vec::new();
            let first = desk.a_check().await;
            println!("CHECK: the first check came to {first:?}; rows {:?}", desk.the_rows());
            assert_eq!(first, TheCheckCameTo::TheSittingStands(Completion::Completed));
            let found_at_first = desk.the_answers_kept();
            for cut in [
                TheWorkersCut::AtTheAsk { nth: 0, brief: true },
                TheWorkersCut::AtTheAsk { nth: 0, brief: false },
                TheWorkersCut::WithTheStreams { after: Duration::from_millis(1_500), brief: true },
                TheWorkersCut::WithTheStreams { after: Duration::from_millis(1_500), brief: false },
                TheWorkersCut::AtTheAsk { nth: 1, brief: true },
                TheWorkersCut::AtTheAsk { nth: 1, brief: false },
            ] {
                let said = format!("CHECK {cut:?}");
                if !only_this_cut(&said) {
                    continue;
                }
                desk.line.comes_back(Which::Every);
                desk.point.is_lending(&format!("{said}: lending"), desk.point.lent()).await;
                if let TheWorkersCut::WithTheStreams { .. } = cut {
                    desk.line.carries_back_at(1_000_000);
                    desk.point = ThePoint::over(&desk.line, &desk.far_end, false).await;
                    desk.line.carries_back_at(0);
                    desk.worker.lock().unwrap().padding = 6_000_000;
                }
                // **Which reading the cut is on is counted afresh** — this
                // check's own first is the root's.
                let cut = match cut {
                    TheWorkersCut::AtTheAsk { nth, brief } => {
                        TheWorkersCut::AtTheAsk { nth: desk.worker.lock().unwrap().readings + nth, brief }
                    }
                    other => other,
                };
                let before = desk.point.lent();
                desk.the_worker_cuts(cut);
                let came_to = desk.a_check().await;
                let took = desk.took(Instant::now());
                desk.worker.lock().unwrap().padding = 0;
                let rows = desk.the_rows();
                println!("{said}: came to {came_to:?} {took:?} after the cut; rows {rows:?}");
                if cut.brief() {
                    a_figure!(
                        missed,
                        came_to == TheCheckCameTo::TheSittingStands(Completion::Completed),
                        "{said}: the check came to {came_to:?}"
                    );
                    a_figure!(missed, desk.point.lent() == before, "{said}: a brief silence ended the connection");
                    continue;
                }
                match took {
                    Some(took) => ended_within(&format!("{said}: the check"), &mut missed, took, the_bound()),
                    None => missed.push(format!("{said}: the cut was never made")),
                }
                a_figure!(
                    missed,
                    desk.the_answers_kept() >= found_at_first,
                    "{said}: the findings an earlier check kept went ({} of {found_at_first})",
                    desk.the_answers_kept()
                );
                match cut {
                    TheWorkersCut::AtTheAsk { nth, .. } if nth % 2 == 1 => {
                        // **Below the root.**
                        a_figure!(
                            missed,
                            came_to == TheCheckCameTo::TheSittingStands(Completion::TheConnectionDropped),
                            "{said}: the check cut below the root came to {came_to:?}"
                        );
                        a_figure!(
                            missed,
                            desk.the_standing() == Some(Completion::TheConnectionDropped),
                            "{said}: the standing reads {:?}",
                            desk.the_standing()
                        );
                        let sitting = desk.the_sitting();
                        let panel = the_panel(&desk.held.lock().unwrap(), &sitting.server, &sitting.file, None);
                        a_figure!(
                            missed,
                            panel.the_statements.iter().any(|statement| statement.which.as_str() == "the-connection-dropped"),
                            "{said}: the panel did not say the connection dropped"
                        );
                    }
                    _ => {
                        a_figure!(
                            missed,
                            matches!(
                                came_to,
                                TheCheckCameTo::ItGaveNothing(smoozy_render_settings::endings::Ending::TheMachineCouldNotBeReached)
                            ),
                            "{said}: the check cut at the root came to {came_to:?}"
                        );
                    }
                }
                desk.back_and_lent(&said, before).await;
                let again = desk.a_check().await;
                a_figure!(
                    missed,
                    again == TheCheckCameTo::TheSittingStands(Completion::Completed)
                        && desk.the_standing() == Some(Completion::Completed),
                    "{said}: the check once back came to {again:?}"
                );
            }
            the_figures_held("THE DEPENDENCY CHECK, CUT", missed);
        }

        /// **The found files sent up, the line cut mid-file** — the primary,
        /// or `Servers` S14's additional connection alone at one session per
        /// connection: the send waits for the connection and carries on, the
        /// picture lands whole at its mirror and reads *on the server*, and
        /// nothing of it is left staged.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn the_dependency_upload_cut_mid_file_carries_on_and_lands_whole() {

            let mut missed = Vec::new();
            for alone in [false, true] {
                let said = if alone { "SEND, THE ADDITIONAL ALONE" } else { "SEND, THE LINE" };
                if !only_this_cut(said) {
                    continue;
                }
                let size = 24_000_000;
                let Some(desk) = a_check_desk(
                    if alone { "send-a" } else { "send" },
                    TheFarEndsShape { one_session_per_connection: alone, ..Default::default() },
                    size,
                )
                .await
                else {
                    println!("SKIPPED: no OpenSSH this test can run here");
                    return;
                };
                let first = desk.a_check().await;
                println!("{said}: the check came to {first:?}; rows {:?}", desk.the_rows());
                let wood = desk.project.join("tex/wood.png");
                let mirror = desk.the_mirror_of(&wood);
                // **The cut mid-file**: once half the picture stands staged at
                // its mirror.
                let staging = mirror.parent().expect("a folder").join(".smoozy-staging");
                let cutting = {
                    let (line, staging) = (Arc::clone(&desk.line), staging.clone());
                    tokio::spawn(async move {
                        let began = Instant::now();
                        loop {
                            let staged: u64 = std::fs::read_dir(&staging)
                                .map(|entries| entries.flatten().filter_map(|entry| entry.metadata().ok()).map(|held| held.len()).max().unwrap_or(0))
                                .unwrap_or(0);
                            if staged >= (size / 2) as u64 || began.elapsed() > Duration::from_secs(60) {
                                break;
                            }
                            tokio::time::sleep(Duration::from_millis(1)).await;
                        }
                        if alone {
                            line.goes_silent(Which::TheLast);
                            line.the_route_is_up(true);
                        } else {
                            line.goes_silent(Which::Every);
                            tokio::time::sleep(AT_THE_DESK.the_declaration_window().1 + Duration::from_millis(500)).await;
                            line.comes_back(Which::Every);
                        }
                    })
                };
                let lent_before = desk.point.lent();
                let sent = desk.a_send(None).await;
                let _ = cutting.await;
                let rows = desk.the_rows();
                println!("{said}: the send came to {sent:?}; rows {rows:?}; {} connections made", desk.line.connections_made());
                a_figure!(missed, sent.failed == 0 && sent.did_not_finish == 0, "{said}: the send came to {sent:?}");
                a_figure!(
                    missed,
                    rows.iter().any(|row| row.1 == "//tex/wood.png" && row.0 == RowState::AlreadyOnTheMachine),
                    "{said}: the picture does not read on the server: {rows:?}"
                );
                a_figure!(
                    missed,
                    std::fs::read(&mirror).ok() == std::fs::read(&wood).ok(),
                    "{said}: the picture at its mirror is not the picture sent"
                );
                let left = left_staged(&desk.far_end.home.join("root"));
                a_figure!(missed, left.iter().all(|path| !path.contains("wood")), "{said}: left staged: {left:?}");
                if alone {
                    a_figure!(missed, desk.point.lent() == lent_before, "{said}: the primary did not stand");
                    a_figure!(missed, desk.line.connections_made() >= 3, "{said}: no fresh additional connection was made");
                } else {
                    a_figure!(missed, desk.point.lent() > lent_before, "{said}: the cut ended no connection");
                }
            }
            the_figures_held("THE DEPENDENCY UPLOAD, CUT", missed);
        }

        /// **A linked library's fixing cut once the fixed copy is made** —
        /// the check after the send fixes the library it reads at its mirror —
        /// **reads *could not be fixed*, never fixed, within the break's
        /// bound, and *Try again* once the line is back fixes it. And a row's
        /// confirmation cut at its reading of the copy on the server** keeps
        /// the row *sent before, not confirmed*, and Confirm once back
        /// confirms it. Under the window, each only waits.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn a_libraries_fixing_and_a_rows_confirmation_cut_say_so_and_finish_once_back() {

            let mut missed = Vec::new();
            for brief in [true, false] {
                let said = if brief { "FIXING, UNDER THE WINDOW" } else { "FIXING, PAST THE WINDOW" };
                if !only_this_cut(said) {
                    continue;
                }
                let Some(desk) = a_check_desk(if brief { "fix-b" } else { "fix" }, TheFarEndsShape::default(), 5_000).await
                else {
                    println!("SKIPPED: no OpenSSH this test can run here");
                    return;
                };
                let _ = desk.a_check().await;
                let sent = desk.a_send(None).await;
                println!("{said}: the send came to {sent:?}");
                // ---- the fixing, cut once the fixed copy is made
                let before = desk.point.lent();
                desk.the_worker_cuts(TheWorkersCut::AtTheFixing { brief });
                let line = Arc::clone(&desk.line);
                let back = tokio::spawn(async move {
                    // Past the window the line comes back once the break has
                    // had its time.
                    if !brief {
                        tokio::time::sleep(Duration::from_secs(30)).await;
                        line.comes_back(Which::Every);
                    }
                });
                let check = desk.a_check().await;
                let took = desk.took(Instant::now());
                let rows = desk.the_rows();
                println!("{said}: the check came to {check:?} {took:?} after the cut; rows {rows:?}");
                let at = rows.iter().position(|row| row.1 == "//lib/props.blend").expect("the library's row");
                if brief {
                    a_figure!(
                        missed,
                        rows[at].0 == RowState::AlreadyOnTheMachine,
                        "{said}: the library's fixing under a brief silence left it {:?}",
                        rows[at].0
                    );
                    back.abort();
                    continue;
                }
                match took {
                    Some(took) => ended_within(&format!("{said}: the check"), &mut missed, took, the_bound() + Duration::from_secs(2)),
                    None => missed.push(format!("{said}: the fixing was never cut")),
                }
                a_figure!(
                    missed,
                    rows[at].0 == RowState::CouldNotBeFixed,
                    "{said}: a library whose fixing was cut reads {:?}",
                    rows[at].0
                );
                back.abort();
                desk.back_and_lent(said, before).await;
                let again = desk.a_send(Some(vec![at])).await;
                println!("{said}: Try again came to {again:?}; rows {:?}", desk.the_rows());
                a_figure!(
                    missed,
                    desk.the_rows()[at].0 == RowState::AlreadyOnTheMachine,
                    "{said}: Try again once back did not fix it: {:?}",
                    desk.the_rows()
                );

                // ---- a row's confirmation, cut at its reading of the copy on
                // the server: the cache folder moved away on this computer
                // after it went up
                let cache = desk.project.join("blendcache_scene");
                let away = desk.far_end.scratch.0.join("moved-away");
                std::fs::rename(&cache, &away).expect("the cache folder is moved away");
                let _ = desk.a_check().await;
                let rows = desk.the_rows();
                let at = rows.iter().position(|row| row.1.contains("blendcache_scene")).expect("the cache's row");
                a_figure!(missed, rows[at].0 == RowState::SentBeforeNotConfirmed, "CONFIRM: the row reads {:?}", rows[at].0);
                let before = desk.point.lent();
                *desk.cut_at.lock().unwrap() = None;
                {
                    let (line, cut_at) = (Arc::clone(&desk.line), Arc::clone(&desk.cut_at));
                    desk.signals.when("stat", move || {
                        line.goes_silent(Which::Every);
                        *cut_at.lock().unwrap() = Some(Instant::now());
                    });
                }
                let of = desk.the_sitting();
                let confirmed = tokio::time::timeout(
                    Duration::from_secs(60),
                    smoozy_desktop::the_found_files_sent::a_row_is_confirmed(
                        &desk.point.sessions,
                        desk.record.as_ref(),
                        &desk.held,
                        &of,
                        at,
                    ),
                )
                .await
                .expect("the confirmation ended within a minute");
                let took = desk.took(Instant::now());
                println!("CONFIRM: cut at its reading came to {confirmed} {took:?} after the cut; row {:?}", desk.the_rows()[at]);
                match took {
                    Some(took) => ended_within("CONFIRM: the confirmation", &mut missed, took, the_bound()),
                    None => missed.push("CONFIRM: the cut was never made".to_string()),
                }
                // **Nothing learned, nothing claimed**: the row stays as it
                // was, and no copy is written down as gone from the server.
                a_figure!(
                    missed,
                    desk.the_rows()[at].0 == RowState::SentBeforeNotConfirmed,
                    "CONFIRM: a confirmation cut at its reading left the row {:?}",
                    desk.the_rows()[at].0
                );
                let gone = desk.record.read(|connection| {
                    TheDependencyRecord::over(connection)
                        .what_it_holds_for(&of.machine)
                        .map(|held| {
                            held.placements
                                .iter()
                                .filter(|placement| String::from_utf8_lossy(placement.mirrored_location.as_bytes()).contains("blendcache_scene"))
                                .map(|placement| format!("{:?}", placement))
                                .filter(|written| written.contains("Gone"))
                                .count()
                        })
                        .unwrap_or(0)
                });
                a_figure!(missed, gone == 0, "CONFIRM: {gone} copies were written down as gone from the server");
                desk.back_and_lent("CONFIRM", before).await;
                let confirmed = smoozy_desktop::the_found_files_sent::a_row_is_confirmed(
                    &desk.point.sessions,
                    desk.record.as_ref(),
                    &desk.held,
                    &of,
                    at,
                )
                .await;
                a_figure!(
                    missed,
                    confirmed && desk.the_rows()[at].0 == RowState::AlreadyOnTheMachine,
                    "CONFIRM: Confirm once back came to {confirmed}, the row {:?}",
                    desk.the_rows()[at].0
                );
                std::fs::rename(&away, &cache).expect("the cache folder is put back");
            }
            the_figures_held("A LIBRARY'S FIXING AND A ROW'S CONFIRMATION, CUT", missed);
        }

        /// **A linked library whose placement reading after its publication
        /// met the dead line** — the send said it uploaded, and the four
        /// values `Dependency Upload & Remap` K7's placement record needs
        /// could not be read, so no placement was written. **Once the line is
        /// back, the next check and its *Try again* must get the library
        /// fixed**, or say plainly what will.
        ///
        /// `Dependency Upload & Remap` Q5's read-through supplies the content
        /// again where `Dependency Upload & Remap` K7's placement reading was lost after publication.
        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn a_library_whose_placement_reading_met_the_dead_line_is_fixed_once_back() {

            let Some(desk) = a_check_desk("placement", TheFarEndsShape::default(), 5_000).await else {
                println!("SKIPPED: no OpenSSH this test can run here");
                return;
            };
            let _ = desk.a_check().await;
            let library = desk.project.join("lib/props.blend");
            let mirror = desk.the_mirror_of(&library).to_string_lossy().into_owned();
            {
                let line = Arc::clone(&desk.line);
                desk.signals.when_it_says(
                    move |said| said.starts_with("stat ") && said.ends_with(&mirror),
                    move || line.goes_silent(Which::Every),
                );
            }
            let before = desk.point.lent();
            let line = Arc::clone(&desk.line);
            let back = tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(8)).await;
                line.comes_back(Which::Every);
            });
            let sent = desk.a_send(None).await;
            let _ = back.await;
            desk.back_and_lent("PLACEMENT", before).await;
            println!("PLACEMENT: the send came to {sent:?}; rows {:?}", desk.the_rows());
            // Keep the placement missing, and cut the first fixing too, so
            // the actual Try again route must read the library and fix it.
            let before = desk.point.lent();
            desk.the_worker_cuts(TheWorkersCut::AtTheFixing { brief: false });
            let line = Arc::clone(&desk.line);
            let back = tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(8)).await;
                line.comes_back(Which::Every);
            });
            let check = desk.a_check().await;
            let rows = desk.the_rows();
            println!("PLACEMENT: the check once back came to {check:?}; rows {rows:?}");
            let at = rows.iter().position(|row| row.1 == "//lib/props.blend").expect("the library's row");
            assert_eq!(rows[at].0, RowState::CouldNotBeFixed);
            let _ = back.await;
            desk.back_and_lent("PLACEMENT, FIXING", before).await;
            let again = desk.a_send(Some(vec![at])).await;
            println!("PLACEMENT: Try again came to {again:?}; rows {:?}", desk.the_rows());
            assert_eq!(
                desk.the_rows()[at].0,
                RowState::AlreadyOnTheMachine,
                "the library is still not fixed: {:?}",
                desk.the_rows()
            );
        }
    }
}

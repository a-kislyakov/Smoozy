//! **A submission sent over a real connection whose far end stops answering
//! part-way through, without closing anything** — the shape of a network
//! change. Written 2026-10-19, out of the live pass on Vast for the thirteenth
//! and fourteenth notes, fault 1: the Creator's Mac moved to another WiFi
//! network while a job was being sent, the old connection went silently dead
//! rather than closed, the act ended *files not placed*, and the Queue row went
//! with no word anywhere the user was looking. `Queue & History` C10 as
//! changed that day: **the row stays in its server's line saying what the
//! panel says, *Not submitted.* and why, with a cross.**
//!
//! **What the far end is.** OpenSSH's own `sshd` in its one-connection mode,
//! as `tests/a_line_cut_silently_under_a_real_connection.rs` runs it, with the
//! same relay between it and the desk: a line that can stop carrying bytes
//! either way and close nothing. **Three things stand in for a rental's own
//! programs, and each is named where it is made**: a `stat` that answers in
//! GNU's fields where this system's own speaks BSD's, the machine's
//! `smoozy-server` entry point as a short Perl script carrying the one framed
//! request to this test, and this test's own supervisor answering it. **Every
//! byte of the submission still crosses the line** — the four-value read, the
//! remap material, the job's material and the job record request.
//!
//! **Nothing of the Creator's is touched.** The server's file-transfer
//! sessions start in a throwaway home (`sftp-server -d`), and its commands run
//! with that folder as their home (`SetEnv`), so the carriage links the act
//! plants land there and not in the Creator's own. The keys, the record and
//! every file are inside throwaway folders deleted with the test, and the
//! servers are children of the test that end with it.
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
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::sync::watch;

    use smoozy_desktop::downloading::TheRunsTransfers;
    use smoozy_desktop::record::LocalRecord;
    use smoozy_desktop::servers::sessions::{lend_until_it_ends, TheAdditionalConnection, ThisServersSessions};
    use smoozy_desktop::submitting::{
        a_not_submitted_row_is_cleared, an_act_is_begun, the_act_in_flight, the_act_is_performed_in_flight,
        the_acts_in_flight_to_draw, WhatBecameOfOneJob, WhatTheActCameTo, WhatThePageHandsOver,
    };
    use smoozy_desktop::surface::render_settings::the_acts_answer;
    use smoozy_desktop::uploading::TheRecordAnUploadWrites;
    use smoozy_job_files::layout::{Area, MachineRoot, TheLayout, WhereThePackageAreaSits};
    use smoozy_queue_history::record::{AnswerOrigin, DependencyCheckAtSubmission, Hue, TheServerAsItStood};
    use smoozy_render_settings::file_half::BlenderIdentity;
    use smoozy_render_settings::payload::{compose, Composition, Payload};
    use smoozy_render_settings::reporting::ReportingChoice;
    use smoozy_render_settings::store::SceneEntries;
    use smoozy_rendering::submission::{the_reporting_field, WhatTheDesktopHoldsAboutTheBuild};
    use smoozy_server_protocol::framing::{
        read_one_request, write_result_frame, Answered, Fields, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
    };
    use smoozy_server_protocol::requests::{
        IT_IS_ABSENT, IT_IS_PRESENT, ON_THIS_MACHINE_FIELD, THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
        THE_JOB_FIELD, THE_JOB_RECORD_IS_WRITTEN_REQUEST,
    };
    use smoozy_server_protocol::transport::ssh::{self, Connection};
    use smoozy_server_protocol::transport::{Credential, DecideHostTrust, Ending, Endpoint, Figures, HostTrust};
    use smoozy_state::identities::{Identity, IdentityKind};

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

    /// A throwaway folder that takes itself away.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
                "smoozy-silent-send-{named}-{}-{}",
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
    /// one connection at a time, its sessions in a throwaway home; the machine
    /// root inside that home; and the three stand-ins.
    struct TheFarEnd {
        sshd: PathBuf,
        config: PathBuf,
        client_key: PathBuf,
        layout: TheLayout,
        the_blend: PathBuf,
        _scratch: Scratch,
    }

    impl TheFarEnd {
        fn made(named: &str, the_supervisors_socket: &Path) -> Option<TheFarEnd> {
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

            // **The machine's root and its five areas, inside the home**, as a
            // placement leaves them (`Job files and outputs` N11).
            let layout = TheLayout::of(
                MachineRoot::chosen(&home.join("root").display().to_string()).expect("a root"),
                WhereThePackageAreaSits::UnderTheRoot,
            );
            for area in smoozy_job_files::layout::EVERY_AREA {
                std::fs::create_dir_all(layout.area(area)).ok()?;
            }
            let the_blend = PathBuf::from(layout.area(Area::Asset)).join("kitchen.blend");
            std::fs::write(&the_blend, b"BLENDER-v405 standing in for a scene").ok()?;

            // **A stand-in: `stat` in GNU's fields.** `Render Settings` Q14 item
            // 2's reading asks for `stat -L -c '%s %i %Y %Z' <file>`, which a
            // rental's Linux answers. Forward those arguments where supported;
            // otherwise ask for the same four values with the Mac's flags.
            a_program(
                &bin.join("stat"),
                "#!/bin/sh\n# Stands in for GNU stat's `-L -c '%s %i %Y %Z' FILE`.\nif /usr/bin/stat --version >/dev/null 2>&1; then exec /usr/bin/stat \"$@\"; fi\nexec /usr/bin/stat -L -f '%z %i %m %c' \"$4\"\n",
            );
            // **A stand-in: the machine's entry point** (`Rented-server
            // runtime` G12), reached through the planted link at
            // `~/.smoozy-carriage/package-area` as on a rental. **It carries
            // the one framed request on its standard input to this test's
            // supervisor and writes back what that answers**, so the request
            // and its answer cross the line as a rental's do.
            a_program(
                &PathBuf::from(layout.area(Area::Package)).join("smoozy-server"),
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
                layout,
                the_blend,
                _scratch: scratch,
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

        /// What the server wrote about itself, for a failure to show.
        fn its_log(&self) -> String {
            std::fs::read_to_string(self.config.with_file_name("sshd.log")).unwrap_or_default()
        }
    }

    fn copy_of(descriptor: std::os::unix::io::RawFd) -> std::os::unix::io::RawFd {
        let copy = unsafe { libc::dup(descriptor) };
        assert!(copy >= 0, "the descriptor could not be copied");
        copy
    }

    // ------------------------------------------------------- the supervisor

    /// **This test's supervisor, standing in for the machine's own**: it takes
    /// every job record it is asked to write, and says *present* for a job it
    /// took and *absent* for any other. Every other request it refuses by
    /// name, as the machine's program does a request it does not answer.
    struct TheSupervisor {
        taken: Arc<Mutex<Vec<String>>>,
        asked: Arc<Mutex<Vec<String>>>,
        task: tokio::task::JoinHandle<()>,
        cut: Arc<Mutex<Option<(bool, Box<dyn FnOnce() + Send>)>>>,
    }

    impl TheSupervisor {
        fn listening_at(socket: &Path) -> TheSupervisor {
            let _ = std::fs::remove_file(socket);
            let listener = tokio::net::UnixListener::bind(socket).expect("the supervisor's socket");
            let taken: Arc<Mutex<Vec<String>>> = Arc::default();
            let asked: Arc<Mutex<Vec<String>>> = Arc::default();
            let cut: Arc<Mutex<Option<(bool, Box<dyn FnOnce() + Send>)>>> = Arc::default();
            let task = tokio::spawn({
                let (taken, asked, cut) = (Arc::clone(&taken), Arc::clone(&asked), Arc::clone(&cut));
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
                        let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0");
                        let job = request.fields.text(THE_JOB_FIELD).unwrap_or_default().to_string();
                        let answer = if request.name == THE_JOB_RECORD_IS_WRITTEN_REQUEST {
                            let cutting = cut.lock().expect("cut").take();
                            if cutting.as_ref().is_none_or(|(accepted, _)| *accepted) {
                                taken.lock().expect("taken").push(job);
                            }
                            if let Some((_, now)) = cutting { now(); }
                            answered.carrying(Fields::none())
                        } else if request.name == THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST {
                            let word = if taken.lock().expect("taken").contains(&job) {
                                IT_IS_PRESENT
                            } else {
                                IT_IS_ABSENT
                            };
                            answered.carrying(Fields::none().and(ON_THIS_MACHINE_FIELD, word))
                        } else {
                            answered.saying_it_does_not_answer_that_request(&request.name)
                        };
                        let _ = stream.write_all(&write_result_frame(&answer)).await;
                        let _ = stream.shutdown().await;
                    }
                }
            });
            TheSupervisor { taken, asked, task, cut }
        }
    }

    impl Drop for TheSupervisor {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    // ---------------------------------------------------------------- the line

    /// **The line between the desk and the machine**, as the silent-line
    /// test's: up, it carries every byte both ways; **silent, it carries
    /// nothing and closes nothing**, which is what a connection left behind by
    /// a network change looks like from the desk.
    struct TheLine {
        address: SocketAddr,
        up: watch::Sender<bool>,
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
        async fn to(far_end: &Arc<TheFarEnd>) -> TheLine {
            let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
                .await
                .expect("the loopback address takes a listener");
            let address = listener.local_addr().expect("a bound listener has an address");
            let (up, _) = watch::channel(true);
            let line = TheLine {
                address,
                up,
                servers: Arc::new(Mutex::new(Vec::new())),
            };
            let far_end = Arc::clone(far_end);
            let up = line.up.clone();
            let servers = Arc::clone(&line.servers);
            tokio::spawn(async move {
                while let Ok((desk, _)) = listener.accept().await {
                    let (server, machine) = far_end.one_connection();
                    if let Ok(mut held) = servers.lock() {
                        held.push(server);
                    }
                    let (desk_reads, desk_writes) = tokio::io::split(desk);
                    let (machine_reads, machine_writes) = machine.into_split();
                    tokio::spawn(one_direction(desk_reads, machine_writes, up.subscribe()));
                    tokio::spawn(one_direction(machine_reads, desk_writes, up.subscribe()));
                }
            });
            line
        }

        /// **Silent at once, both ways** — callable from the record's own write.
        fn goes_silent(&self) {
            self.up.send_replace(false);
        }

        fn comes_back(&self) {
            self.up.send_replace(true);
        }
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

    /// One direction of one connection, carried while the line is up.
    async fn one_direction<R, W>(mut from: R, mut to: W, mut up: watch::Receiver<bool>)
    where
        R: tokio::io::AsyncRead + Unpin,
        W: tokio::io::AsyncWrite + Unpin,
    {
        let mut buffer = vec![0u8; 64 * 1024];
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

    /// **The desk's figures**: `Servers` K3's mechanism with a 1-second
    /// interval and 3 unanswered, as the silent-line test has them, so the
    /// declaration window is 3 to 4 seconds rather than K3's 45 to 60.
    const AT_THE_DESK: Figures = Figures {
        interval: Duration::from_secs(1),
        unanswered_before_the_break: 3,
    };

    /// **How far past the window's close the act may end and still have ended
    /// with it**: the desk noticing the break, then the one step under way
    /// failing and the act writing its ending. A rental's figure is K3's 45 to
    /// 60 seconds plus the same.
    const ONE_STEP_AFTER_THE_BREAK: Duration = Duration::from_secs(5);

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
    /// and nothing repairs it when it breaks, so the act meets the break.
    async fn lent(
        line: &TheLine,
        far_end: &TheFarEnd,
    ) -> (ThisServersSessions, tokio::task::JoinHandle<(Ending, Instant)>) {
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
        (sessions, lending)
    }

    // ------------------------------------------------------------- the record

    /// **The application's own record, on a throwaway file** — and, where the
    /// test asks, **the line goes silent the moment the provisional job record
    /// first stands**: `Queue & History` Y3 writes the record, then sends, so
    /// the silence falls before any of the job's material is placed, which is
    /// where the live pass's act was.
    struct ARecordHere {
        record: Mutex<LocalRecord>,
        when_the_job_record_stands: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    }

    impl ARecordHere {
        fn count(&self, table: &str) -> i64 {
            let record = self.record.lock().expect("the record");
            record
                .connection()
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| row.get::<_, i64>(0))
                .unwrap_or(-1)
        }
    }

    impl TheRecordAnUploadWrites for ARecordHere {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let Ok(record) = self.record.lock() else {
                return false;
            };
            act(record.connection());
            let jobs = record
                .connection()
                .query_row("SELECT count(*) FROM job_record", [], |row| row.get::<_, i64>(0))
                .unwrap_or(0);
            drop(record);
            if jobs > 0 {
                if let Some(then) = self.when_the_job_record_stands.lock().expect("held").take() {
                    then();
                }
            }
            true
        }
    }

    impl smoozy_desktop::keys::LendsTheRecord for ARecordHere {
        fn write_a_diary_line(&self, _: &smoozy_keys::diary::Act) {}
        fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
            act(self.record.lock().unwrap().connection());
        }
    }

    // ------------------------------------------------------------- the act

    fn a_payload(frames: &str) -> Payload {
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

    /// **What the Render Settings page hands over** for one scene-job of
    /// `the_blend` on `server` — as `tests/a_job_is_submitted_through_the_servers_own_point.rs`
    /// composes it.
    fn what_the_page_hands_over(
        far_end: &TheFarEnd,
        server: &Identity,
        installation: &Identity,
        temporary: &Path,
    ) -> WhatThePageHandsOver {
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
            the_first_frame_check: false,
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

    /// **One act, begun as the press begins it**, run off the test's runtime
    /// as the page runs it off the drawing thread. Answers what it came to,
    /// the act's row, and how long it took.
    async fn the_act(
        given: WhatThePageHandsOver,
        sessions: &ThisServersSessions,
        record: &Arc<ARecordHere>,
    ) -> (WhatTheActCameTo, u64, Duration) {
        let in_flight = an_act_is_begun(&given);
        let which = in_flight.which();
        let (sessions, record) = (sessions.clone(), Arc::clone(record));
        let began = Instant::now();
        let came_to = tokio::task::spawn_blocking(move || {
            let the_run = TheRunsTransfers::for_this_run();
            the_act_is_performed_in_flight(given, &sessions, &the_run, record.as_ref(), in_flight).0
        })
        .await
        .expect("the act ran");
        (came_to, which, began.elapsed())
    }

    /// **What the queue draws for the act now**: the row's sentence as the
    /// panel says it, in one string, where the act's row is still drawn.
    fn what_the_row_says(which: u64) -> Option<String> {
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

    // ---------------------------------------------------------------- the proof

    /// **A submission whose line goes silent before its material is placed
    /// ends within the break rule's window and one step, and its row stays in
    /// the queue saying *Not submitted.* and why; once the line is back,
    /// sending that file again goes, and takes the old row away.** The live
    /// pass on Vast for the thirteenth and fourteenth notes, fault 1;
    /// `Queue & History` C10 and `Design/Queue & History` line 25 as changed
    /// 2026-10-19.
    ///
    /// **What it bites on.** Before the change the act ended the same way,
    /// *files not placed*, and its row went with it: nothing in the queue said
    /// anything, and the one place that did was a Render Settings page the user
    /// had left. **What it also holds**: the act does not hang at
    /// *Submitting…* — the connection's own break detection ends it, with no
    /// deadline of the act's own (`Server protocol and transfers` C18) — and
    /// nothing of the job is left behind on this computer.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_submission_on_a_line_gone_silent_stays_in_the_queue_saying_so_and_goes_once_sent_again() {
        let socket = std::env::temp_dir().join(format!("smz-sup-{}.sock", std::process::id()));
        let Some(far_end) = TheFarEnd::made("cut", &socket) else {
            println!(
                "SKIPPED: no OpenSSH sshd, sftp-server, ssh-keygen and Perl this test can run on \
                 this system ({}).",
                std::env::consts::OS
            );
            return;
        };
        let far_end = Arc::new(far_end);
        let supervisor = TheSupervisor::listening_at(&socket);


        let desk = Scratch::new("desk");
        let record = Arc::new(ARecordHere {
            record: Mutex::new(LocalRecord::open_at(&desk.0.join("smoozy.sqlite3")).expect("the record opens")),
            when_the_job_record_stands: Mutex::new(None),
        });
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        let temporary = desk.0.join("desk-temporary");
        let line = Arc::new(TheLine::to(&far_end).await);

        // **The first send, and the line goes silent under it.**
        let (sessions, lending) = lent(&line, &far_end).await;
        {
            let line = Arc::clone(&line);
            *record.when_the_job_record_stands.lock().expect("held") = Some(Box::new(move || line.goes_silent()));
        }
        let (came_to, which, took) = the_act(
            what_the_page_hands_over(&far_end, &server, &installation, &temporary),
            &sessions,
            &record,
        )
        .await;
        let (_, closes) = AT_THE_DESK.the_declaration_window();
        println!("the first send came to {came_to:?} in {took:?}");
        assert!(
            matches!(&came_to, WhatTheActCameTo::TheseJobs(jobs)
                if matches!(jobs.as_slice(), [WhatBecameOfOneJob::ItStoppedBeforeTheJobRecord { .. }])),
            "the send on a silent line came to {came_to:?}; the far end said: {}",
            far_end.its_log()
        );
        assert!(
            took <= closes + ONE_STEP_AFTER_THE_BREAK,
            "the send on a silent line took {took:?}, past the window's close ({closes:?}) and one step"
        );
        let lending = tokio::time::timeout(Duration::from_secs(10), lending)
            .await
            .expect("the lending answered")
            .expect("the lending ran");
        assert!(lending.0.is_a_break(), "the lending answered {:?}", lending.0);
        assert!(supervisor.taken.lock().expect("taken").is_empty(), "the silent line carried a job record");

        // **The row stays, saying what the panel says.**
        let row = the_act_in_flight(which).expect("**the row went with the act, and nothing said why**");
        assert!(!row.ended_in_the_line);
        assert_eq!(row.not_submitted.as_ref(), Some(&came_to));
        assert_eq!(
            what_the_row_says(which).as_deref(),
            Some(
                ["Not submitted.", "Could not upload the job’s files to the server."]
                    .map(|words| smoozy_desktop::typography::Prose::new(words).as_str().to_string())
                    .join(" ")
                    .as_str()
            )
        );
        // **Nothing of the job is left on this computer**: the provisional
        // record went with the act (`Rendering` S4).
        assert_eq!(record.count("job_record"), 0, "the provisional record stood");

        // **The line is back: the same file sent again goes, and the old row
        // goes with it.**
        line.comes_back();
        let (sessions, again_lending) = lent(&line, &far_end).await;
        let (came_to, again, took) = the_act(
            what_the_page_hands_over(&far_end, &server, &installation, &temporary),
            &sessions,
            &record,
        )
        .await;
        println!("the second send came to {came_to:?} in {took:?}");
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]),
            "the send once the line was back; the supervisor was asked {:?}; the far end said: {}",
            supervisor.asked.lock().expect("asked"),
            far_end.its_log()
        );
        assert_eq!(supervisor.taken.lock().expect("taken").len(), 1);
        assert!(the_act_in_flight(which).is_none(), "the old row stood beside the new send");
        assert!(the_act_in_flight(again).is_some_and(|row| row.ended_in_the_line));
        assert!(!a_not_submitted_row_is_cleared(again));
        again_lending.abort();
        drop(sessions);
        let _ = std::fs::remove_file(&socket);
    }
    /// Cut after publication, at the record request: either never accepted, or
    /// accepted with its answer lost. Only the former ever becomes a debt.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn never_taken_material_is_cleared_after_reconciliation_but_lost_acceptance_survives() {
        use smoozy_desktop::servers::the_machines_work::{
            every_provisional_record_is_resolved, OverThisServersPoint, WhatStepSevenDid,
        };
        use smoozy_desktop::submitting::OnThisServersPoint;

        for accepted in [false, true] {
            let socket =
                std::env::temp_dir().join(format!("smz-k3-{}-{accepted}.sock", std::process::id()));
            let Some(far_end) = TheFarEnd::made(&format!("k3-{accepted}"), &socket) else {
                println!("SKIPPED: OpenSSH is unavailable");
                return;
            };
            let far_end = Arc::new(far_end);
            let supervisor = TheSupervisor::listening_at(&socket);
            let desk = Scratch::new("k3-record");
            let record = Arc::new(ARecordHere {
                record: Mutex::new(LocalRecord::open_at(&desk.0.join("record.sqlite3")).unwrap()),
                when_the_job_record_stands: Mutex::new(None),
            });
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();
            let installation = Identity::issue(IdentityKind::Installation).unwrap();
            let line = Arc::new(TheLine::to(&far_end).await);
            let (sessions, lending) = lent(&line, &far_end).await;
            let silence = Arc::clone(&line);
            *supervisor.cut.lock().unwrap() =
                Some((accepted, Box::new(move || silence.goes_silent())));
            let (came_to, which, _) = tokio::time::timeout(
                Duration::from_secs(20),
                the_act(
                    what_the_page_hands_over(
                        &far_end,
                        &server,
                        &installation,
                        &desk.0.join("temporary"),
                    ),
                    &sessions,
                    &record,
                ),
            )
            .await
            .expect("the cut ends the submission");
            assert_eq!(
                came_to,
                WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::NotKnown])
            );
            assert!(
                the_act_in_flight(which).is_none(),
                "unknown acceptance is not reported as a failed send"
            );
            tokio::time::timeout(Duration::from_secs(10), lending)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(
                record.count("job_record"),
                1,
                "unknown work remains provisional"
            );
            let material = {
                let record = record.record.lock().unwrap();
                record
                    .connection()
                    .query_row(
                        "SELECT material_folder, remap_path FROM submission_material",
                        [],
                        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
                    )
                    .unwrap()
            };
            assert!(Path::new(&material.0).join("manifest.json").exists());
            assert!(Path::new(&material.1).exists());
            line.comes_back();
            let (sessions, lending) = lent(&line, &far_end).await;
            let mut files = OnThisServersPoint {
                sessions: &sessions,
            };
            assert_eq!(
                smoozy_desktop::submission_cleanup::pay(
                    record.as_ref(),
                    &mut files,
                    &server,
                    &installation
                )
                .await,
                0,
                "no cleanup before a definitive answer"
            );
            let (for_record, for_sessions, for_server, package_area) = (
                Arc::clone(&record),
                sessions.clone(),
                server.clone(),
                far_end.layout.area(Area::Package),
            );
            let did = tokio::task::spawn_blocking(move || {
                let mut machine = OverThisServersPoint {
                    sessions: for_sessions,
                    package_area,
                };
                let mut did = WhatStepSevenDid::default();
                every_provisional_record_is_resolved(
                    &for_server,
                    for_record.as_ref(),
                    &mut machine,
                    &mut did,
                );
                did
            })
            .await
            .unwrap();
            if accepted {
                assert_eq!(did.confirmed, 1);
                assert_eq!(record.count("job_record"), 1);
            } else {
                assert_eq!(did.removed, 1);
                assert_eq!(record.count("job_record"), 0);
                assert_eq!(
                    record
                        .record
                        .lock()
                        .unwrap()
                        .connection()
                        .query_row("SELECT debt FROM submission_material", [], |row| row
                            .get::<_, i64>(0))
                        .unwrap(),
                    1,
                    "a never-taken submission owes its recorded material"
                );
                let other = Identity::issue(IdentityKind::Installation).unwrap();
                assert_eq!(
                    smoozy_desktop::submission_cleanup::pay(
                        record.as_ref(),
                        &mut files,
                        &server,
                        &other
                    )
                    .await,
                    0
                );
            }
            drop(files);
            lending.abort();
            let _ = lending.await;
            // The next connection pays the debt created by reconciliation.
            let (sessions, lending) = lent(&line, &far_end).await;
            let mut files = OnThisServersPoint {
                sessions: &sessions,
            };
            if !accepted {
                let unknown = Path::new(&material.0).join("unrecorded-file");
                std::fs::write(&unknown, b"leave this alone").unwrap();
                assert_eq!(
                    smoozy_desktop::submission_cleanup::pay(
                        record.as_ref(),
                        &mut files,
                        &server,
                        &installation
                    )
                    .await,
                    0
                );
                assert!(unknown.exists());
                assert!(
                    Path::new(&material.0).join("manifest.json").exists(),
                    "an unknown set is left alone"
                );
                std::fs::remove_file(unknown).unwrap();
                let mut cutting = CutTelegram {
                    sessions: &sessions,
                    line: &line,
                    cut: 4,
                    writes: 0,
                };
                assert_eq!(
                    smoozy_desktop::submission_cleanup::pay(
                        record.as_ref(),
                        &mut cutting,
                        &server,
                        &installation
                    )
                    .await,
                    0,
                    "a removed file with a lost answer leaves its unit unpaid"
                );
                tokio::time::timeout(Duration::from_secs(10), lending)
                    .await
                    .unwrap()
                    .unwrap();
                line.comes_back();
                let (repaired, next_lending) = lent(&line, &far_end).await;
                let mut files = OnThisServersPoint {
                    sessions: &repaired,
                };
                assert_eq!(
                    smoozy_desktop::submission_cleanup::pay(
                        record.as_ref(),
                        &mut files,
                        &server,
                        &installation
                    )
                    .await,
                    1
                );
                assert_eq!(
                    smoozy_desktop::submission_cleanup::pay(
                        record.as_ref(),
                        &mut files,
                        &server,
                        &installation
                    )
                    .await,
                    0
                );
                assert!(!Path::new(&material.0).exists());
                assert!(!Path::new(&material.1).exists());
                assert!(far_end.the_blend.exists());
                next_lending.abort();
                let _ = next_lending.await;
                let _ = std::fs::remove_file(&socket);
                continue;
            }
            let paid = smoozy_desktop::submission_cleanup::pay(
                record.as_ref(),
                &mut files,
                &server,
                &installation,
            )
            .await;
            assert_eq!(paid, 0);
            assert_eq!(Path::new(&material.0).exists(), accepted);
            assert_eq!(Path::new(&material.1).exists(), accepted);
            assert!(far_end.the_blend.exists(), "the source survives cleanup");
            assert_eq!(
                smoozy_desktop::submission_cleanup::pay(
                    record.as_ref(),
                    &mut files,
                    &server,
                    &installation
                )
                .await,
                0
            );
            lending.abort();
            let _ = lending.await;
            let _ = std::fs::remove_file(&socket);
        }
    }

    /// The cut uses the real file-transfer channel. Only its timing is supplied
    /// here: after open, after a partial write, or before the second file.
    struct CutTelegram<'a> {
        sessions: &'a ThisServersSessions,
        line: &'a TheLine,
        cut: u8,
        writes: usize,
    }
    use smoozy_server_setup::readings::{
        AListedThing, TheMachinesFileSurface, WhatTheMachineSaid as FileSaid,
    };
    impl TheMachinesFileSurface for CutTelegram<'_> {
        async fn read_whole(&mut self, path: &str) -> Result<Vec<u8>, FileSaid> {
            smoozy_desktop::submitting::OnThisServersPoint {
                sessions: self.sessions,
            }
            .read_whole(path)
            .await
        }
        async fn write_whole(&mut self, path: &str, bytes: &[u8]) -> Result<(), FileSaid> {
            self.writes += 1;
            if self.writes == 1 && (self.cut == 1 || self.cut == 2) {
                let (path, partial) = (path.as_bytes().to_vec(), self.cut == 2);
                self.sessions
                    .a_reading(false, move |subsystem| {
                        Box::pin(async move {
                            use smoozy_server_protocol::subsystem::{open_flag, Attributes};
                            let handle = subsystem
                                .open(
                                    &path,
                                    open_flag::WRITE | open_flag::CREATE | open_flag::TRUNCATE,
                                    Attributes::permissions(0o600),
                                )
                                .await
                                .unwrap();
                            if partial {
                                subsystem.send_a_write(&handle, 0, b"cut").await.unwrap();
                                subsystem.settle_the_writes().await.unwrap();
                            }
                            subsystem.close(&handle).await?;
                            Ok(())
                        })
                    })
                    .await
                    .unwrap()
                    .unwrap();
                self.line.goes_silent();
            }
            if self.writes == 2 && self.cut == 3 {
                self.line.goes_silent();
            }
            smoozy_desktop::submitting::OnThisServersPoint {
                sessions: self.sessions,
            }
            .write_whole(path, bytes)
            .await
        }
        async fn make_one_directory(&mut self, path: &str) -> Result<(), FileSaid> {
            smoozy_desktop::submitting::OnThisServersPoint {
                sessions: self.sessions,
            }
            .make_one_directory(path)
            .await
        }
        async fn remove_one_file(&mut self, path: &str) -> Result<(), FileSaid> {
            let removed = smoozy_desktop::submitting::OnThisServersPoint {
                sessions: self.sessions,
            }
            .remove_one_file(path)
            .await;
            if removed.is_ok() && self.cut == 4 && path.ends_with("/manifest.json") {
                self.line.goes_silent();
                return Err(FileSaid::TheChannelIsGone);
            }
            removed
        }
        async fn remove_one_empty_directory(&mut self, path: &str) -> Result<(), FileSaid> {
            smoozy_desktop::submitting::OnThisServersPoint {
                sessions: self.sessions,
            }
            .remove_one_empty_directory(path)
            .await
        }
        async fn list_one_directory(&mut self, path: &str) -> Result<Vec<AListedThing>, FileSaid> {
            smoozy_desktop::submitting::OnThisServersPoint {
                sessions: self.sessions,
            }
            .list_one_directory(path)
            .await
        }
        async fn plant_a_link(&mut self, target: &str, at: &str) -> Result<(), FileSaid> {
            smoozy_desktop::submitting::OnThisServersPoint {
                sessions: self.sessions,
            }
            .plant_a_link(target, at)
            .await
        }
        async fn read_link(&mut self, path: &str) -> Result<Vec<u8>, FileSaid> {
            smoozy_desktop::submitting::OnThisServersPoint {
                sessions: self.sessions,
            }
            .read_link(path)
            .await
        }
        async fn replace_by_rename(&mut self, from: &str, to: &str) -> Result<(), FileSaid> {
            smoozy_desktop::submitting::OnThisServersPoint {
                sessions: self.sessions,
            }
            .replace_by_rename(from, to)
            .await
        }
    }

    fn the_dummy_role_uses_the_saved_set(expected: &smoozy_telegram::material::TheMaterial) {
        for (path, bytes) in expected.the_files() {
            assert_eq!(&std::fs::read(path).unwrap(), bytes);
        }
        let configuration = smoozy_telegram::material::TheRoleConfiguration::read_back(
            &std::fs::read(expected.the_role_configurations_path()).unwrap(),
        )
        .unwrap();
        assert_eq!(&configuration, expected.the_configuration());
        let service = Arc::new(smoozy_telegram::transport::a_stand_in::AStandInService::new());
        let client = smoozy_telegram::transport::TheClient::over(service.clone());
        assert!(client
            .send_message(configuration.conversation.0, "Dummy report", None)
            .is_ok());
        assert_eq!(service.heard().len(), 1);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn telegram_cuts_keep_the_working_dummy_set_and_no_told_entry_moves_before_retry() {
        use smoozy_desktop::record::told_record;
        use smoozy_security_and_licence::plaintext::Plaintext;
        use smoozy_server_setup::acts::place_the_telegram_material;
        use smoozy_telegram::{
            material::{TheMaterial, TheRoleConfiguration},
            proof::TheConversation,
            told::TokenGeneration,
        };
        for cut in 1..=3 {
            let socket =
                std::env::temp_dir().join(format!("smz-k2-{}-{cut}.sock", std::process::id()));
            let Some(far_end) = TheFarEnd::made(&format!("k2-{cut}"), &socket) else {
                println!("SKIPPED: OpenSSH is unavailable");
                return;
            };
            let far_end = Arc::new(far_end);
            let area = far_end.layout.area(Area::Package);
            let installation = Identity::issue(IdentityKind::Installation).unwrap();
            let server = Identity::issue(IdentityKind::LocalServer)
                .unwrap()
                .as_text();
            let configuration = TheRoleConfiguration {
                conversation: TheConversation(1234),
                machine: installation.as_text(),
                colour: None,
                label: Some("dummy server".to_string()),
                reporting: Vec::new(),
            };
            let old = TheMaterial::composed(
                &area,
                Plaintext::new(b"old dummy token".to_vec()),
                TokenGeneration(1),
                configuration.clone(),
            )
            .unwrap();
            let mut replacement = configuration;
            replacement.conversation = TheConversation(1235);
            replacement.label = Some("replacement dummy server".to_string());
            let new = TheMaterial::composed(
                &area,
                Plaintext::new(b"new dummy token".to_vec()),
                TokenGeneration(2),
                replacement,
            )
            .unwrap();
            for (path, bytes) in old.the_files() {
                std::fs::write(path, bytes).unwrap();
            }
            let desk = Scratch::new("k2-record");
            let record = LocalRecord::open_at(&desk.0.join("record.sqlite3")).unwrap();
            let old_answer: Vec<_> = old
                .the_files()
                .iter()
                .map(|(path, _)| {
                    (
                        path.clone(),
                        smoozy_server_setup::acts::HowTheActWent::ItWasCarriedOut,
                    )
                })
                .collect();
            told_record::the_placement_was_answered(
                record.connection(),
                &server,
                &old,
                &old_answer,
                std::time::SystemTime::UNIX_EPOCH,
            )
            .unwrap();
            let before =
                told_record::what_this_server_was_told(record.connection(), &server).unwrap();
            let line = TheLine::to(&far_end).await;
            let (sessions, lending) = lent(&line, &far_end).await;
            let mut files = CutTelegram {
                sessions: &sessions,
                line: &line,
                cut,
                writes: 0,
            };
            let answer = tokio::time::timeout(
                Duration::from_secs(15),
                place_the_telegram_material(&mut files, new.the_files()),
            )
            .await
            .expect("the line's break ends placement");
            assert!(answer.iter().all(|(_, went)| !went.it_was_carried_out()));
            assert!(
                answer
                    .iter()
                    .all(|(_, went)| went.in_words().contains("connection")),
                "placement says why it failed"
            );
            told_record::the_placement_was_answered(
                record.connection(),
                &server,
                &new,
                &answer,
                std::time::SystemTime::now(),
            )
            .unwrap();
            assert_eq!(
                told_record::what_this_server_was_told(record.connection(), &server).unwrap(),
                before
            );
            the_dummy_role_uses_the_saved_set(&old);
            tokio::time::timeout(Duration::from_secs(10), lending)
                .await
                .unwrap()
                .unwrap();
            line.comes_back();
            let (sessions, lending) = lent(&line, &far_end).await;
            let mut files = smoozy_desktop::submitting::OnThisServersPoint {
                sessions: &sessions,
            };
            let answer = place_the_telegram_material(&mut files, new.the_files()).await;
            assert!(
                answer.iter().all(|(_, went)| went.it_was_carried_out()),
                "retry: {answer:?}"
            );
            told_record::the_placement_was_answered(
                record.connection(),
                &server,
                &new,
                &answer,
                std::time::SystemTime::now(),
            )
            .unwrap();
            assert_ne!(
                told_record::what_this_server_was_told(record.connection(), &server).unwrap(),
                before
            );
            the_dummy_role_uses_the_saved_set(&new);
            lending.abort();
            let _ = lending.await;
        }
    }
}

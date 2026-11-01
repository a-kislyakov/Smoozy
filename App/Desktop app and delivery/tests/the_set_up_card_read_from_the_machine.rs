//! **The 38's piece 6 at the desk: a copy of Smoozy opened after a server was
//! set up reads the setup off the machine — the machine's own program, asked
//! through the server's own point.**
//!
//! **What the far end is.** The machine program this workspace built
//! (`smoozy-server`), started as a supervisor in a throwaway package area, and
//! reached exactly as a rental reaches it: the point plants the carriage link
//! over **OpenSSH's own `sftp-server`** on a Unix socket pair, and the command
//! runs the entry point through that link with the request on its standard
//! input. **What it is not is an SSH connection**; the transport above is all
//! real.
//!
//! **Nothing of the Creator's is touched.** Every path is inside a throwaway
//! folder in the build folder's own temporary area, removed with the test, and
//! every program started under it is ended with it.
//!
//! **How a clause marker is written in this crate.** A marker with no document
//! beside it means a clause of `Preparation/Architecture/Desktop app and
//! delivery.md`; every other document's marker is written out with that
//! document at its site.

#[cfg(not(unix))]
#[test]
fn the_far_end_here_wants_a_unix_socket_pair() {
    println!(
        "SKIPPED: the far end here is OpenSSH's own sftp-server on a Unix socket pair, which \
         this system ({}) has none of.",
        std::env::consts::OS
    );
}

#[cfg(unix)]
mod on_a_desk_with_a_socket_pair {
    use std::ops::Deref;
    use std::os::fd::OwnedFd;
    use std::os::unix::process::CommandExt;
    use std::path::{Path, PathBuf};
    use std::pin::Pin;
    use std::process::{Child, Command, Stdio};
    use std::sync::{Arc, Mutex, Weak};
    use std::task::{Context, Poll};
    use std::time::{Duration, Instant};

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

    use smoozy_desktop::servers::sessions::{
        lend_until_it_ends, ACommandRunning, AHeldConnection, NoSession, TheAdditionalConnection,
        ThisServersSessions,
    };
    use smoozy_desktop::setting_up::{the_card_as_read, the_setup_as_read};
    use smoozy_desktop::surface::set_up::{the_tab, WhatTheMachineSaid};
    use smoozy_server_protocol::framing::{
        read_one_result, write_request_frame, Fields, Request,
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
    };
    use smoozy_server_protocol::placement::performing::THE_HEALTH_EXCHANGE_COMMAND;
    use smoozy_server_protocol::requests as asked;
    use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};
    use smoozy_state::identities::{Identity, IdentityKind};

    #[cfg(debug_assertions)]
    const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
        "/usr/libexec/sftp-server",
        "/usr/lib/openssh/sftp-server",
        "/usr/libexec/openssh/sftp-server",
        "/usr/lib/ssh/sftp-server",
    ];

    /// **How long one asking of the machine's program is given** — a test's
    /// own bound; an answer takes well under a second.
    const HOW_LONG_ONE_ASKING_IS_GIVEN: Duration = Duration::from_secs(60);

    /// **How long one machine may stand before it is ended**, pass or panic —
    /// a test's own bound, so a wait that never ends fails the run rather than
    /// holding it.
    #[cfg(debug_assertions)]
    const HOW_LONG_A_MACHINE_MAY_STAND: Duration = Duration::from_secs(600);

    /// **Every program whose command line names `under`, ended** — asked again
    /// until none is left, because a worker may start a child between one
    /// listing and the kill (`App/Scenarios/src/the_desk_machine.rs`).
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

    /// **The machine's own deadline**: dropped with [`TheStandingMachine`],
    /// pass or panic. Still standing at [`HOW_LONG_A_MACHINE_MAY_STAND`], a
    /// wait in the test never ended — so every program the machine started is
    /// ended and so is this binary (`App/Scenarios/src/the_desk_machine.rs`).
    struct TheDeadline {
        _held: std::sync::mpsc::Sender<()>,
    }

    impl TheDeadline {
        #[cfg(debug_assertions)]
        fn started(machine: &Arc<TheMachine>) -> TheDeadline {
            let (held, watched) = std::sync::mpsc::channel::<()>();
            let weak: Weak<TheMachine> = Arc::downgrade(machine);
            let folder = machine.at.clone();
            let endpoint = machine.endpoint.clone();
            let test = std::thread::current()
                .name()
                .unwrap_or("a set-up card test")
                .to_string();
            std::thread::spawn(move || {
                if watched.recv_timeout(HOW_LONG_A_MACHINE_MAY_STAND)
                    == Err(std::sync::mpsc::RecvTimeoutError::Timeout)
                {
                    let said = format!(
                        "DEADLINE MISSED: {test} still held its machine after {:?}. Every \
                         program it started is ended now, and so is this binary.\n",
                        HOW_LONG_A_MACHINE_MAY_STAND
                    );
                    let _ = std::io::Write::write_all(&mut std::io::stderr(), said.as_bytes());
                    if let Some(machine) = weak.upgrade() {
                        machine.end_every_process_and_clean_up();
                    }
                    every_program_started_under_is_ended(&folder);
                    let _ = std::fs::remove_file(&endpoint);
                    let _ = std::fs::remove_dir_all(&folder);
                    std::process::exit(101);
                }
            });
            TheDeadline { _held: held }
        }
    }

    /// **A machine standing for the length of one test**: dropped with the
    /// test, pass or panic, ending every process it started at once rather
    /// than waiting for a background task's handle to go first.
    struct TheStandingMachine {
        inner: Arc<TheMachine>,
        _deadline: TheDeadline,
    }

    impl Deref for TheStandingMachine {
        type Target = TheMachine;

        fn deref(&self) -> &TheMachine {
            &self.inner
        }
    }

    impl Drop for TheStandingMachine {
        fn drop(&mut self) {
            self.inner.end_every_process_and_clean_up();
        }
    }

    /// **The machine: a login's home the file-transfer server serves, a
    /// package area holding the machine's program, and its root.**
    struct TheMachine {
        sftp_server: PathBuf,
        at: PathBuf,
        home: PathBuf,
        area: PathBuf,
        endpoint: PathBuf,
        supervisor: Mutex<Option<Child>>,
        file_transfer_servers: Mutex<Vec<Arc<Mutex<Option<Child>>>>>,
        /// **Every *enabled* or *disabled* the desk sent this machine**, in
        /// order, as it arrived through the command route.
        the_role_was_asked: Mutex<Vec<TheRoleAsked>>,
    }

    /// **One *enabled* or *disabled* as the machine took it**: which, what
    /// the token file held at the moment it arrived, and what the machine's
    /// own supervisor said it did with the role worker.
    #[derive(Debug, Clone)]
    struct TheRoleAsked {
        enabled: bool,
        the_token_file_held: Option<Vec<u8>>,
        the_role_worker: Option<String>,
    }

    impl TheMachine {
        /// **The machine's program this workspace built**, beside this test's
        /// own executable, and OpenSSH's own file-transfer server — or nothing,
        /// and the test says so.
        fn standing(named: &str) -> Option<TheStandingMachine> {
            // **A release build of these tests stands no machine**: the
            // program beside it is a release build, which has no switch to
            // keep its Telegram worker off the line (see below).
            #[cfg(not(debug_assertions))]
            {
                let _ = named;
                println!(
                    "SKIPPED: a release build has no switch to keep the machine's Telegram \
                     worker from reaching Telegram, so no machine is started."
                );
                return None;
            }
            #[cfg(debug_assertions)]
            {
                Self::standing_in_a_development_build(named)
            }
        }

        #[cfg(debug_assertions)]
        fn standing_in_a_development_build(named: &str) -> Option<TheStandingMachine> {
            use smoozy_runtime::the_program::{
                records, the_active_package_pointer_at, the_record_at,
                THE_FIXED_ENTRY_POINT_NAME, THE_SUPERVISOR_WORD,
            };
            let sftp_server = WHERE_SFTP_SERVER_LIVES
                .iter()
                .map(PathBuf::from)
                .find(|path| path.exists())?;
            let built = std::env::current_exe()
                .ok()?
                .parent()?
                .parent()?
                .join(THE_FIXED_ENTRY_POINT_NAME);
            if !built.is_file() {
                return None;
            }
            let at = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
                .join(format!("smzy-card-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&at);
            let home = at.join("home");
            let area = at.join("package");
            let root = at.join("root");
            for folder in [&home, &area, &root] {
                std::fs::create_dir_all(folder).expect("a folder of the machine's");
            }
            let at = std::fs::canonicalize(&at).expect("the folder is there");
            let (home, area, root) = (at.join("home"), at.join("package"), at.join("root"));
            let area_text = area.display().to_string();
            std::os::unix::fs::symlink(&built, the_active_package_pointer_at(&area_text))
                .expect("the pointer is made");
            // **A short endpoint path**: a socket's name has a length limit.
            let endpoint = std::env::temp_dir().join(format!(
                "smzy-{}.sock",
                &Identity::issue(IdentityKind::Operation).expect("randomness").as_text()[..12]
            ));
            std::fs::write(
                the_record_at(&area_text, records::THE_ENDPOINTS_RECORDED_PATH),
                endpoint.display().to_string(),
            )
            .expect("the endpoint's path is recorded");
            std::fs::write(
                the_record_at(&area_text, records::THE_INSTALLATION_IDENTITY),
                Identity::issue(IdentityKind::Installation).expect("randomness").as_text(),
            )
            .expect("the installation is recorded");
            let phase = smoozy_runtime::package::phase::a_fresh_attempts_first_write(
                Identity::issue(IdentityKind::Operation).expect("randomness"),
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
                smoozy_runtime::written_form::write(&phase).expect("the record writes"),
            )
            .expect("the phase record is written");
            std::fs::write(
                the_record_at(&area_text, smoozy_job_files::layout::THE_RECORD_OF_THE_ROOT),
                root.display().to_string(),
            )
            .expect("the root is recorded");
            // **No test reaches Telegram.** The supervisor starts the real
            // Telegram worker, which on this machine finds made-up material
            // and would ask Telegram whether the token is good; with the
            // development switch on, every call it makes comes back at once
            // as a line that is down and nothing leaves this computer. The
            // supervisor's environment is what every worker it starts
            // inherits, so this one value reaches the worker.
            //
            // **Started in its own process group**, so every worker the
            // supervisor starts (`work telegram`, `work setup`, and any child
            // of theirs) shares that group and is ended by the kernel's own
            // group signal when this machine is dropped.
            let supervisor = Command::new(the_active_package_pointer_at(&area_text))
                .process_group(0)
                .arg(THE_SUPERVISOR_WORD)
                .env(smoozy_telegram::transport::THE_DEVELOPMENT_SWITCH_THAT_REACHES_NOTHING, "1")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("the supervisor starts");
            let machine = Arc::new(TheMachine {
                sftp_server,
                at,
                home,
                area,
                endpoint,
                supervisor: Mutex::new(Some(supervisor)),
                file_transfer_servers: Mutex::new(Vec::new()),
                the_role_was_asked: Mutex::new(Vec::new()),
            });
            let standing = TheStandingMachine {
                _deadline: TheDeadline::started(&machine),
                inner: machine,
            };
            let health = a_request(smoozy_server_protocol::requests::THE_HEALTH_ANSWER_REQUEST, Fields::none());
            let answered = smoozy_runtime::acting::endpoint::TheEndpointAsACaller::at(
                &standing.endpoint.display().to_string(),
                Duration::from_secs(1),
            )
            .ask_waiting_for_a_supervisor(&health, HOW_LONG_ONE_ASKING_IS_GIVEN)
            .expect("the endpoint is reachable at all");
            assert!(answered.is_some(), "the machine's own supervisor never answered");
            Some(standing)
        }

        /// **One framed request through the entry point found at `entry`**,
        /// its output as the command's.
        fn the_entry_point_answers(entry: &Path, frame: &[u8]) -> CommandResult {
            use std::io::Write;
            let mut adapter = Command::new(entry)
                .process_group(0)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("the adapter starts");
            let group = adapter.id() as libc::pid_t;
            if let Err(why) = adapter
                .stdin
                .take()
                .expect("a standard input")
                .write_all(frame)
            {
                unsafe {
                    libc::kill(-group, libc::SIGKILL);
                }
                let _ = adapter.kill();
                let _ = adapter.wait();
                panic!("the request could not go in: {why}");
            }
            let (sent, heard) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let _ = sent.send(adapter.wait_with_output());
            });
            let Ok(Ok(said)) = heard.recv_timeout(HOW_LONG_ONE_ASKING_IS_GIVEN) else {
                unsafe {
                    libc::kill(-group, libc::SIGKILL);
                }
                panic!("DEADLINE MISSED: the machine's own program never answered one asking");
            };
            CommandResult {
                output: said.stdout,
                complaint: said.stderr,
                status: said.status.code().and_then(|code| u32::try_from(code).ok()),
            }
        }

        /// **Every *enabled* or *disabled* that arrived after the first
        /// `since`**, in order.
        fn the_role_asked_since(&self, since: usize) -> Vec<TheRoleAsked> {
            self.the_role_was_asked.lock().expect("the list")[since..].to_vec()
        }

        fn how_often_the_role_was_asked(&self) -> usize {
            self.the_role_was_asked.lock().expect("the list").len()
        }

        /// **Asked straight, as a setup run's own driving asks** — the test's
        /// way of setting the machine up, which is not what is under test.
        fn ask(&self, request: &Request) -> smoozy_server_protocol::framing::Answered {
            let pointer = smoozy_runtime::the_program::the_active_package_pointer_at(
                &self.area.display().to_string(),
            );
            let said = Self::the_entry_point_answers(Path::new(&pointer), &write_request_frame(request));
            let answered =
                read_one_result(&mut said.output.as_slice()).expect("the program answered a frame");
            assert!(answered.answers(request), "the answer is to the request");
            answered
        }

        /// **What the machine does with a command**: the health exchange runs
        /// the entry point through the link the point planted in this home —
        /// the whole route a rental's command takes.
        fn the_command_runs(&self, command: &str, frame: Option<Vec<u8>>) -> CommandResult {
            if command == THE_HEALTH_EXCHANGE_COMMAND.as_str() {
                let entry = self.home.join(
                    command
                        .strip_prefix("~/")
                        .expect("the exchange is named from the login's home"),
                );
                let frame = frame.unwrap_or_default();
                // **The request's name, read off the frame the desk sent**, and
                // the token file as it stands when the request arrives.
                let enabled = smoozy_server_protocol::framing::read_one_request(&mut frame.as_slice())
                    .ok()
                    .and_then(|request| match request.name.as_str() {
                        asked::THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST => Some(true),
                        asked::THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST => Some(false),
                        _ => None,
                    });
                let the_token_file_held = std::fs::read(self.area.join("telegram-token")).ok();
                let said = Self::the_entry_point_answers(&entry, &frame);
                if let Some(enabled) = enabled {
                    let the_role_worker = read_one_result(&mut said.output.as_slice())
                        .ok()
                        .and_then(|answered| {
                            answered.fields.text(asked::WHAT_THE_ROLE_WORKER_DID_FIELD).map(str::to_string)
                        });
                    self.the_role_was_asked.lock().expect("the list").push(TheRoleAsked {
                        enabled,
                        the_token_file_held,
                        the_role_worker,
                    });
                }
                return said;
            }
            CommandResult {
                output: Vec::new(),
                complaint: b"no such command here".to_vec(),
                status: Some(127),
            }
        }

        /// **Ends every process started for this machine and removes its
        /// folder.**
        ///
        /// **Why both the process group and the path sweep are here**: a
        /// worker the supervisor starts (`smoozy-server work telegram`,
        /// `work setup`) outlives the supervisor itself (`Rented-server
        /// runtime` W6, `Rented-server runtime` W10), so ending the supervisor
        /// alone leaves the worker running. Putting the supervisor in its own
        /// process group makes a group signal from the kernel reach the
        /// supervisor and every worker it started even where `pkill` cannot
        /// inspect the process table; waiting until the group answers *no such
        /// process* sees every worker reaped; and
        /// [`every_program_started_under_is_ended`] catches any entry-point
        /// call started outside that group through the machine's folder.
        fn end_every_process_and_clean_up(&self) {
            let servers = self
                .file_transfer_servers
                .lock()
                .map(|mut held| std::mem::take(&mut *held))
                .unwrap_or_default();
            for server in servers {
                if let Some(mut child) = server.lock().ok().and_then(|mut held| held.take()) {
                    let _ = child.kill();
                    let _ = child.wait();
                }
            }
            if let Some(mut supervisor) = self.supervisor.lock().ok().and_then(|mut held| held.take()) {
                let group = supervisor.id() as libc::pid_t;
                unsafe {
                    libc::kill(-group, libc::SIGCONT);
                    libc::kill(-group, libc::SIGKILL);
                }
                let _ = supervisor.kill();
                let _ = supervisor.wait();
                for _ in 0..50 {
                    if unsafe { libc::kill(-group, libc::SIGKILL) } != 0 {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(20));
                }
            }
            every_program_started_under_is_ended(&self.at);
            let _ = std::fs::remove_file(&self.endpoint);
            let _ = std::fs::remove_dir_all(&self.at);
        }
    }

    impl Drop for TheMachine {
        fn drop(&mut self) {
            self.end_every_process_and_clean_up();
        }
    }

    fn a_request(name: &str, fields: Fields) -> Request {
        Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            name,
        )
        .carrying(fields)
    }

    struct ASession {
        stream: tokio::net::UnixStream,
        child: Arc<Mutex<Option<Child>>>,
    }

    impl Drop for ASession {
        fn drop(&mut self) {
            if let Some(mut child) = self.child.lock().ok().and_then(|mut held| held.take()) {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    impl AsyncRead for ASession {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for ASession {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.stream).poll_write(cx, buf)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.stream).poll_shutdown(cx)
        }
    }

    struct AConnectionToIt(Arc<TheMachine>);

    impl AHeldConnection for AConnectionToIt {
        type Channel = ASession;

        async fn open_a_session(&mut self) -> Result<ASession, NoSession> {
            let (ours, theirs) = std::os::unix::net::UnixStream::pair().expect("a socket pair");
            ours.set_nonblocking(true).expect("the end we drive");
            let reading: OwnedFd = theirs.try_clone().expect("its reading end").into();
            let writing: OwnedFd = theirs.into();
            let child = Command::new(&self.0.sftp_server)
                .current_dir(&self.0.home)
                .stdin(Stdio::from(reading))
                .stdout(Stdio::from(writing))
                .stderr(Stdio::null())
                .spawn()
                .expect("the machine's own file-transfer server starts");
            let child = Arc::new(Mutex::new(Some(child)));
            if let Ok(mut servers) = self.0.file_transfer_servers.lock() {
                servers.retain(|one| one.lock().ok().is_some_and(|held| held.is_some()));
                servers.push(Arc::clone(&child));
            }
            Ok(ASession {
                stream: tokio::net::UnixStream::from_std(ours).expect("the end we drive"),
                child,
            })
        }

        async fn a_command(
            &mut self,
            command: CommandString,
            frame: Option<Vec<u8>>,
        ) -> Result<CommandResult, NoSession> {
            let machine = Arc::clone(&self.0);
            let command = command.as_str().to_string();
            Ok(tokio::task::spawn_blocking(move || machine.the_command_runs(&command, frame))
                .await
                .expect("the command ran"))
        }

        async fn a_command_begun(
            &mut self,
            command: CommandString,
            frame: Option<Vec<u8>>,
        ) -> Result<ACommandRunning, NoSession> {
            let machine = Arc::clone(&self.0);
            let command = command.as_str().to_string();
            Ok(ACommandRunning::to_be_run(async move {
                Ok(tokio::task::spawn_blocking(move || machine.the_command_runs(&command, frame))
                    .await
                    .expect("the command ran"))
            }))
        }

        async fn until_it_ends(&mut self) -> Ending {
            std::future::pending().await
        }

        async fn torn_down(self) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    /// **The background lending of [`AConnectionToIt`]**, aborted on drop so a
    /// failing assertion ends the lending too rather than leaving it detached.
    struct TheLending(tokio::task::JoinHandle<Ending>);

    impl Drop for TheLending {
        fn drop(&mut self) {
            self.0.abort();
        }
    }

    /// **The machine, connected through its own point**, with the lending held
    /// for the length of the test.
    async fn connected(
        machine: &TheStandingMachine,
    ) -> (ThisServersSessions<AConnectionToIt>, TheLending) {
        let sessions = ThisServersSessions::<AConnectionToIt>::for_one_server();
        let more = Arc::clone(&machine.inner);
        let additional: TheAdditionalConnection<AConnectionToIt> = Arc::new(move || {
            let more = Arc::clone(&more);
            Box::pin(async move { Some(AConnectionToIt(more)) })
        });
        let mut primary = AConnectionToIt(Arc::clone(&machine.inner));
        let lending = tokio::spawn({
            let sessions = sessions.clone();
            async move { lend_until_it_ends(&sessions, &mut primary, &additional).await }
        });
        let started = Instant::now();
        while !sessions.a_connection_is_live() {
            assert!(
                started.elapsed() < HOW_LONG_ONE_ASKING_IS_GIVEN,
                "the lending never went live"
            );
            tokio::task::yield_now().await;
        }
        (sessions, TheLending(lending))
    }

    /// **A ready action, run to *done* on the machine** — opened with
    /// `Server Setup` Y4's two boundaries and *done* written from the desk, the
    /// way the desktop's own driving ends one.
    fn a_ready_action_run_to_done(machine: &TheMachine) -> Identity {
        let mut fields = Fields::none()
            .and(asked::THE_KIND_OF_SETUP_FIELD, asked::THE_READY_ACTION)
            .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "2");
        for (at, boundary) in [asked::THE_BOUNDARY_STARTED, asked::THE_BOUNDARY_DONE]
            .iter()
            .enumerate()
        {
            fields = fields.and(
                &asked::the_field_of_item(asked::A_COMPOSED_BOUNDARY, at, asked::THE_BOUNDARY_FIELD),
                *boundary,
            );
        }
        let opening = a_request(asked::THE_SETUP_RUN_IS_OPENED_REQUEST, fields);
        let run = opening.identity.clone();
        let opened = machine.ask(&opening);
        assert_eq!(
            opened.fields.text(asked::WHAT_BECAME_OF_IT_FIELD),
            Some(asked::IT_IS_STILL_IN_HAND),
            "{:?}",
            opened.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
        );
        let state = || {
            machine.ask(&a_request(
                asked::THE_SETUP_RUNS_STATE_REQUEST,
                Fields::none().and(asked::THE_SETUP_RUN_FIELD, run.as_text()),
            ))
        };
        let asked_at = Instant::now();
        while state().fields.text(asked::HOW_MANY_BOUNDARIES_REACHED_FIELD) != Some("1") {
            assert!(asked_at.elapsed() < HOW_LONG_ONE_ASKING_IS_GIVEN, "the run never started");
            std::thread::sleep(Duration::from_millis(50));
        }
        machine.ask(&a_request(
            asked::THE_BOUNDARY_IS_WRITTEN_REQUEST,
            Fields::none()
                .and(asked::THE_SETUP_RUN_FIELD, run.as_text())
                .and(asked::THE_BOUNDARY_FIELD, asked::THE_BOUNDARY_DONE)
                .and(asked::WHAT_THE_STEP_DID_FIELD, asked::IT_PERFORMED),
        ));
        assert_eq!(state().fields.text(asked::THE_RUNS_ENDING_FIELD), Some(asked::IT_COMPLETED));
        run
    }

    /// **A copy opened after the server was set up — so it drove no run there
    /// and no state stands — draws the setup's steps, never *No setup read
    /// from this server*** (`Server Setup` Y6a, `Server Setup` Y6b; `Open Threads`, *The Set
    /// up card says No setup read…*).
    ///
    /// **Undone, this fails at the run**: with nothing to name the ready
    /// action's retained run, the card is read from nothing and says no setup
    /// was read — which is the old route's answer, asserted first.
    #[tokio::test]
    async fn a_copy_opened_after_the_setup_draws_its_steps_and_not_no_setup_read() {
        let Some(machine) = TheMachine::standing("fresh-copy") else {
            println!(
                "SKIPPED: no OpenSSH sftp-server, or the machine's program is not built beside \
                 this test (`cargo build -p smoozy-server`). The card's reading is unproved here."
            );
            return;
        };
        let run = a_ready_action_run_to_done(&machine);

        let (sessions, lending) = connected(&machine).await;
        let area = machine.area.display().to_string();

        // **What a fresh copy had before this piece**: no state, no run it
        // drove, and so nothing read.
        assert!(
            the_setup_as_read(&sessions, &area, None).await.is_none(),
            "a fresh copy names no run by the old route"
        );

        let read = the_card_as_read(&sessions, &area, None).await;
        drop(lending);
        let rest = read.the_rest.clone().expect("the machine answered the card's reading");
        assert_eq!(rest.the_ready_actions_run.as_ref(), Some(&run));
        assert_eq!(rest.the_role_is_enabled, Some(false));
        let jobs = rest.the_jobs.clone().expect("the jobs were read");
        assert_eq!(jobs.the_most_recent, None, "nothing has ended on this machine");
        let the_run = read.the_run.clone().expect("the retained run is read");
        assert!(the_run.record.a_setup_finished());

        let tab = the_tab(
            WhatTheMachineSaid::ASetupRunAndTheMachine(Box::new(the_run), Box::new(rest)),
            "server-1",
        );
        let card = serde_json::to_value(&tab.the_card).expect("the card renders");
        // **The typography point binds short words with a non-breaking
        // space**, so the words are compared as a reader reads them.
        let read_as = |value: &serde_json::Value| {
            value.as_str().unwrap_or_default().replace('\u{a0}', " ")
        };
        // **`Server Setup` W1: a run every step of which had nothing to do
        // reads *Nothing to prepare***, a finished setup's own ending.
        assert_eq!(read_as(&card["state"]), "Nothing to prepare", "{card:#}");
        assert_eq!(card["dot"], "ok", "{card:#}");
        assert!(
            card["what_was_done"].as_array().is_some_and(|steps| !steps.is_empty()),
            "the setup's steps are listed: {card:#}"
        );
        assert_ne!(read_as(&card["state"]), "No setup found");
        // **`Server Setup` W3a's first line, read rather than left out.**
        assert_eq!(
            read_as(&card["the_render_device"]["lead"]),
            "Nothing rendered yet. GPU rendering not confirmed.",
            "{card:#}"
        );
        assert!(tab.the_card_was_last_seen.is_none());
    }

    // ════════════════════════════════════════════════════════════════════
    // The 38's piece 11: Telegram reaching a server set up before it
    // ════════════════════════════════════════════════════════════════════

    use smoozy_desktop::record::told_record;
    use smoozy_desktop::setting_up::{
        the_telegram_material_is_placed, what_the_server_was_not_told_is_told, NobodyAsked,
        NobodyIsThereToAsk, TakesTheOpeningReadings, TheMaterialFromTheRecord, TheServersRowHere,
        TheRunsEnding, WhatTheTellingCameTo, WhatThisServerIsOwed, WhereTheSetUpGot,
    };
    use smoozy_desktop::surface::set_up::{the_tab_with, TheBlenderSearch};
    use smoozy_security_and_licence::held::HeldSecret;
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::{GetFailure, ProbeOutcome, SecretStore, StoreFailure};
    use smoozy_telegram::proof::{KeepsTheProvedBot, TheBot, TheConversation};

    /// **The local record, lent the way the running application lends it.**
    struct ARecordHere(smoozy_desktop::record::LocalRecord);

    impl smoozy_desktop::keys::LendsTheRecord for ARecordHere {
        fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
            act(self.0.connection())
        }
        fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
    }

    /// **The bot token's one item, in memory, shared by every handle** — a
    /// made-up token, never a real one.
    #[derive(Clone, Default)]
    struct AStoreHere(Arc<Mutex<Option<Vec<u8>>>>);

    impl SecretStore for AStoreHere {
        fn put(&self, _: &HeldSecret, value: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            *self.0.lock().expect("the item") = Some(value.expose(|bytes| bytes.to_vec()));
            Ok(())
        }
        fn get(&self, _: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            self.0.lock().expect("the item").clone().map(Plaintext::new).ok_or(GetFailure::NotThere)
        }
        fn erase(&self, _: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
            *self.0.lock().expect("the item") = None;
            Ok(())
        }
        fn probe(&self, _: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    /// `Server Setup` M1 and `Server Setup` M1a's two readings, as a systemd Ubuntu machine
    /// answers them — the desk has no `/proc/1/comm` of a rental's to read.
    struct AnUbuntuMachine;

    impl TakesTheOpeningReadings for AnUbuntuMachine {
        fn taken(
            &mut self,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<
            '_,
            smoozy_server_setup::readings::TheMachineBeforeAnythingIsTouched,
        > {
            Box::pin(async {
                smoozy_server_setup::readings::TheMachineBeforeAnythingIsTouched::as_answered(
                    smoozy_server_setup::kind::MachineKind::ASystemdMachine,
                    Some("ID=ubuntu\nVERSION_ID=\"24.04\"\n".to_string()),
                    None,
                )
            })
        }
    }

    /// **A copy of Smoozy's own side for one server**: a fresh local record
    /// holding the server's row — its recorded root the machine's, its
    /// installation the machine's own — a bot proved, its made-up token stored,
    /// and Telegram switched on. **Told nothing yet.**
    struct OursHere {
        record: ARecordHere,
        store: AStoreHere,
        server: String,
        folder: PathBuf,
    }

    impl OursHere {
        fn for_the(machine: &TheMachine) -> OursHere {
            use smoozy_servers::identity::{Host, KeyReference, Port, Username};
            use smoozy_servers::store::{RecordedMachineRoot, ServerRow};
            let folder = machine.at.join("the-desk");
            std::fs::create_dir_all(&folder).expect("the desk's own folder");
            let record = smoozy_desktop::record::LocalRecord::open_at(&folder.join("local-record.sqlite3"))
                .expect("the record opens");
            let installation = std::fs::read_to_string(smoozy_runtime::the_program::the_record_at(
                &machine.area.display().to_string(),
                smoozy_runtime::the_program::records::THE_INSTALLATION_IDENTITY,
            ))
            .expect("the machine's installation reads");
            let c = record.connection();
            let mut row = ServerRow::saved(
                Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text(),
                Host::what_the_user_typed("127.0.0.1").expect("a host"),
                Port::what_the_user_typed(22).expect("a port"),
                Username::what_the_user_typed("root").expect("a username"),
                KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
            );
            row.label = Some("render box".into());
            row.last_confirmed_installation_identity = Some(installation.trim().to_string());
            // **The root the package area sits under**: `at`, so the package
            // area the record names is the machine's own (`Job files and
            // outputs` L4a's first arrangement).
            row.recorded_machine_root = Some(RecordedMachineRoot {
                path: machine.at.display().to_string(),
                the_package_areas_own_location: None,
                read_under: installation.trim().to_string(),
            });
            smoozy_desktop::record::server_rows::add(c, &row).expect("the row is added");
            smoozy_desktop::record::server_rows::write(c, &row).expect("the row is written");
            let bot = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
            let keeper = smoozy_desktop::record::telegram_bot::TheProvedBotRecord::over(c);
            keeper.the_proof_was_recorded(&bot, TheConversation(9001)).expect("the proof");
            let store = AStoreHere::default();
            store
                .put(
                    &smoozy_telegram::walkthrough::the_bot_tokens_item(),
                    Plaintext::new(b"7:the-first-made-up-token".to_vec()),
                    Duration::from_secs(1),
                )
                .expect("the token is stored");
            keeper.the_token_was_stored_for(&bot).expect("the stored fact");
            smoozy_desktop::record::settings::telegram_was_switched(c, true).expect("Telegram is on");
            OursHere { server: row.local_server_identifier, record: ARecordHere(record), store, folder }
        }

        fn the_material(&self) -> TheMaterialFromTheRecord<'_> {
            TheMaterialFromTheRecord {
                record: &self.record,
                server: self.server.clone(),
                store: Some(Box::new(self.store.clone())),
            }
        }

        fn c(&self) -> &rusqlite::Connection {
            self.record.0.connection()
        }

        /// **Settings' *not reached* list**, as the section draws it.
        fn not_reached(&self) -> Option<Vec<String>> {
            smoozy_desktop::surface::telegram::what_the_section_holds(self.c(), None).not_reached
        }

        fn told(&self, thing: smoozy_telegram::told::AToldThing) -> Option<String> {
            told_record::what_this_server_was_told(self.c(), &self.server)
                .expect("the told-record reads")
                .into_iter()
                .find(|entry| entry.thing == thing)
                .map(|entry| entry.value)
        }
    }

    impl Drop for OursHere {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.folder);
        }
    }

    /// **`Server Setup` G2's dedicated entry point, run from the desk** — what
    /// *Install it* runs.
    async fn install_it(
        sessions: &ThisServersSessions<AConnectionToIt>,
        ours: &OursHere,
    ) -> WhereTheSetUpGot {
        let mut row = TheServersRowHere { record: &ours.record, server: ours.server.clone() };
        let mut telegram = ours.the_material();
        the_telegram_material_is_placed(
            sessions,
            &mut AnUbuntuMachine,
            &mut row,
            &mut telegram,
            &mut NobodyIsThereToAsk,
            &NobodyAsked,
        )
        .await
    }

    /// **What a setup with Telegram on leaves** — `Server Setup` R1 item 8's
    /// own parts, in its order: the material composed from the record, placed
    /// through `Server Setup` G4's one vehicle with its answer handed to the
    /// told-record, then the role's state written on the ordinary path and
    /// acknowledged. (*Install it*'s own run is tested on its own above, so it
    /// does not seed these tests.)
    async fn as_a_setup_with_telegram_on_leaves_it(
        sessions: &ThisServersSessions<AConnectionToIt>,
        area: &str,
        ours: &OursHere,
    ) {
        use smoozy_desktop::setting_up::ComposesTheTelegramMaterial;
        let mut telegram = ours.the_material();
        let material = telegram.composed().expect("the material composes");
        let placed = smoozy_server_setup::acts::place_the_telegram_material(
            &mut smoozy_desktop::submitting::OnThisServersPoint { sessions },
            material.the_files(),
        )
        .await;
        assert!(placed.iter().all(|(_, went)| went.it_was_carried_out()), "{placed:?}");
        telegram.the_placement_was_answered(&material, &placed);
        smoozy_desktop::setting_up::the_roles_desired_state_is_written(sessions, area, true)
            .await
            .expect("the role is switched on");
        telegram.the_desired_state_was_acknowledged();
    }

    /// **The role's own state, as the machine reads it back**
    /// (`Server Setup` Y6b's Telegram line).
    async fn the_roles_state(sessions: &ThisServersSessions<AConnectionToIt>, area: &str) -> Option<bool> {
        the_card_as_read(sessions, area, None).await.the_rest.and_then(|rest| rest.the_role_is_enabled)
    }

    /// **A server set up with Telegram off, and Telegram on since: a
    /// connection owes it nothing** — it was never given Telegram, so the next
    /// connection does not bring it (the map's hole 14) — **and the card offers
    /// *Install it***, Settings' switch being on (`Design/Server Setup § Adding
    /// Telegram to a server already set up`).
    #[tokio::test]
    async fn a_server_never_given_telegram_is_offered_install_it_and_a_connection_brings_nothing() {
        let Some(machine) = TheMachine::standing("never-given") else {
            println!("SKIPPED: no OpenSSH sftp-server, or the machine's program is not built beside this test.");
            return;
        };
        let _ready = a_ready_action_run_to_done(&machine);
        let ours = OursHere::for_the(&machine);
        let (sessions, lending) = connected(&machine).await;
        let area = machine.area.display().to_string();
        assert_eq!(
            what_the_server_was_not_told_is_told(&sessions, &area, &mut ours.the_material()).await,
            WhatTheTellingCameTo::NothingWasOwed
        );
        assert!(!machine.area.join("telegram-token").exists(), "nothing was placed");
        let read = the_card_as_read(&sessions, &area, None).await;
        drop(lending);
        assert_eq!(read.the_rest.as_ref().and_then(|rest| rest.the_role_is_enabled), Some(false));
        let tab = the_tab_with(
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(read.the_run.expect("the ready run is read")),
                Box::new(read.the_rest.expect("the card's reading")),
            ),
            "server-1",
            TheBlenderSearch::ItWasNotTaken,
            true,
        );
        let card = serde_json::to_value(&tab.the_card).expect("the card renders");
        assert!(
            card["what_was_done"]
                .as_array()
                .is_some_and(|lines| lines.iter().any(|line| line["offer"]["act"] == "install-telegram")),
            "the card offers Install it: {card:#}"
        );
    }

    /// **`Server Setup` G2: *Install it* on a server already set up places the
    /// material and sets the role on, and does nothing else** — and it is
    /// placed once: the next connection owes it nothing (`Telegram` L2). **And
    /// `Server Setup` Y6b: the Telegram placement replaces nothing of the ready
    /// action's record**, so the card still reads the setup, now with
    /// *Telegram reporting installed* and no offer.
    ///
    /// **Ignored from the 38's piece 11 until 2026-10-04**: the machine's final
    /// check read a run whose marker stood at *telegram* as one that published
    /// a Blender, and ended it (`Server Setup — machine`'s
    /// `acting::worker::the_run_published_a_blender`, which now reads the run's
    /// composed list beside its marker).
    #[tokio::test]
    async fn install_it_on_a_set_up_server_places_telegram_once_and_replaces_nothing() {
        let Some(machine) = TheMachine::standing("install-it") else {
            println!("SKIPPED: no OpenSSH sftp-server, or the machine's program is not built beside this test.");
            return;
        };
        let ready = a_ready_action_run_to_done(&machine);
        let ours = OursHere::for_the(&machine);
        let (sessions, lending) = connected(&machine).await;
        let area = machine.area.display().to_string();

        assert_eq!(
            what_the_server_was_not_told_is_told(&sessions, &area, &mut ours.the_material()).await,
            WhatTheTellingCameTo::NothingWasOwed,
            "a server never given Telegram is not given it by a connection"
        );
        let read = the_card_as_read(&sessions, &area, None).await;
        assert_eq!(read.the_rest.as_ref().and_then(|rest| rest.the_role_is_enabled), Some(false));
        let card = |read: smoozy_desktop::setting_up::TheCardAsRead| {
            let tab = the_tab_with(
                WhatTheMachineSaid::ASetupRunAndTheMachine(
                    Box::new(read.the_run.expect("the ready run is read")),
                    Box::new(read.the_rest.expect("the card's reading")),
                ),
                "server-1",
                TheBlenderSearch::ItWasNotTaken,
                true,
            );
            serde_json::to_value(&tab.the_card).expect("the card renders")
        };
        let offers = |card: &serde_json::Value| {
            card["what_was_done"]
                .as_array()
                .is_some_and(|lines| lines.iter().any(|line| line["offer"]["act"] == "install-telegram"))
        };
        assert!(offers(&card(read)), "the card offers Install it");

        let came_to = install_it(&sessions, &ours).await;
        assert!(
            matches!(came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. }),
            "{came_to:?}"
        );
        assert_eq!(the_roles_state(&sessions, &area).await, Some(true), "the role is on");
        assert_eq!(
            std::fs::read(machine.area.join("telegram-token")).expect("the token file is placed"),
            b"7:the-first-made-up-token"
        );
        let read = the_card_as_read(&sessions, &area, None).await;
        assert_eq!(
            read.the_rest.as_ref().and_then(|rest| rest.the_ready_actions_run.clone()),
            Some(ready),
            "`Server Setup` Y6b: the Telegram placement replaced nothing of the ready action's record"
        );
        let drawn = card(read);
        assert!(!offers(&drawn), "nothing is offered once it is installed: {drawn:#}");
        // **Read as a person reads it**: the typography point binds a short
        // last word to the one before with a no-break space.
        assert!(
            format!("{drawn}").replace('\u{a0}', " ").contains("Telegram reporting installed"),
            "{drawn:#}"
        );

        // **`Telegram` L1: every told thing the placement carried is in the
        // record**, the role's state among them, so Settings names nobody.
        use smoozy_telegram::told::AToldThing;
        assert_eq!(ours.told(AToldThing::TheRolesDesiredState).as_deref(), Some("enabled"));
        assert_eq!(ours.told(AToldThing::TheTokenGeneration).as_deref(), Some("1"));
        assert_eq!(ours.not_reached(), Some(Vec::new()));
        assert_eq!(
            what_the_server_was_not_told_is_told(&sessions, &area, &mut ours.the_material()).await,
            WhatTheTellingCameTo::NothingWasOwed,
            "placed once: the next connection owes it nothing"
        );
        drop(lending);
    }

    /// **No release is read and nothing is placed on a server already set up**
    /// — `Server Setup` Y6b, *"On a machine prepared earlier, the placement
    /// replaces nothing"*: *Set up again* there runs `Server Setup` R1's run
    /// alone (step 10c's ninth-pass fix piece 3). Either one reached is a
    /// fault, so each says so.
    struct NoPlacementOnASetUpServer;

    impl smoozy_desktop::setting_up::PlacesTheVerifiedRelease<AConnectionToIt> for NoPlacementOnASetUpServer {
        fn place<'a>(
            &'a mut self,
            _connection: &'a mut AConnectionToIt,
            _needs: smoozy_server_protocol::placement::performing::WhatThePlacementNeeds<'a>,
        ) -> smoozy_desktop::servers::sessions::ItsOwnFuture<
            'a,
            Result<
                smoozy_server_protocol::placement::performing::ThePlacementsOutcome,
                smoozy_server_protocol::placement::performing::WhyThePlacementCouldNotBeCarriedOut,
            >,
        > {
            Box::pin(async { panic!("a server already set up is placed again") })
        }
    }

    struct NoReleaseOnASetUpServer;

    impl smoozy_desktop::setting_up::ReadsTheCurrentRelease for NoReleaseOnASetUpServer {
        fn the_current_release(
            &mut self,
            _detected: &smoozy_server_protocol::release::manifest::TargetTuple,
        ) -> Result<(smoozy_server_protocol::release::verifying::AVerifiedRelease, Vec<u8>), String> {
            panic!("a release is read for a server already set up")
        }
    }

    /// **An ordinary setup with Telegram on and no Blender asked for reaches
    /// *done*** — *Set up again* on a server already set up, a machine with no
    /// card established and no Blender, so `Server Setup` R1 composes the
    /// readings, item 8's Telegram material and item 9's *done*. **Item 9's
    /// final check has nothing of its own to read there**: no Blender was
    /// fetched or published and none was named, so `Server Setup` R4's *"where
    /// there was no Blender to read one against, the library half was not read
    /// and *ready* claims nothing about it"* is the whole answer, and the run
    /// ends at *done*.
    ///
    /// **Until 2026-10-04 the machine's program ended this run at the final
    /// check**, *"the final check was commanded over a Blender this run
    /// published"*: it read a marker standing at *telegram* as one past the
    /// publication. **This is the run the 38's piece 11 expected to fail and
    /// nobody had run** (`Audits/Build/Step 10c/The setup's final check and
    /// Install Telegram — what landed.md`).
    #[tokio::test]
    async fn set_up_again_with_telegram_on_and_no_blender_reaches_done() {
        let Some(machine) = TheMachine::standing("tg-no-blender") else {
            println!("SKIPPED: no OpenSSH sftp-server, or the machine's program is not built beside this test.");
            return;
        };
        let ours = OursHere::for_the(&machine);
        let (sessions, lending) = connected(&machine).await;
        let area = machine.area.display().to_string();
        let mut row = TheServersRowHere { record: &ours.record, server: ours.server.clone() };
        let mut telegram = ours.the_material();
        let came_to = smoozy_desktop::setting_up::a_machine_is_set_up(
            &sessions,
            &mut AnUbuntuMachine,
            &mut row,
            NoPlacementOnASetUpServer,
            &mut NoReleaseOnASetUpServer,
            &smoozy_desktop::setting_up::WhatWasAskedFor { install_this_blender: None, telegram_is_on: true, ..Default::default() },
            smoozy_server_setup::elevated::HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
            &mut telegram,
            &mut NobodyIsThereToAsk,
            &NobodyAsked,
            None,
            Some(smoozy_state::identities::InstallationCheck::PresentAndEqualConfirmed),
        )
        .await;
        assert!(
            matches!(came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. }),
            "{came_to:?}"
        );
        assert_eq!(the_roles_state(&sessions, &area).await, Some(true), "the role is on");
        assert_eq!(
            std::fs::read(machine.area.join("telegram-token")).expect("the token file is placed"),
            b"7:the-first-made-up-token"
        );
        drop(lending);
    }

    /// **`Telegram` L4 and L2: a server told an old token gets the new one on
    /// the next connection, through `Server Setup` G4's one vehicle, and the
    /// told-record says so** — Settings names the server until then, and nobody
    /// after.
    #[tokio::test]
    async fn a_server_told_an_old_token_gets_the_new_one_on_connect_and_the_record_says_so() {
        let Some(machine) = TheMachine::standing("new-token") else {
            println!("SKIPPED: no OpenSSH sftp-server, or the machine's program is not built beside this test.");
            return;
        };
        let ours = OursHere::for_the(&machine);
        let (sessions, lending) = connected(&machine).await;
        let area = machine.area.display().to_string();
        as_a_setup_with_telegram_on_leaves_it(&sessions, &area, &ours).await;
        assert_eq!(ours.not_reached(), Some(Vec::new()), "a setup leaves nobody unreached");

        // **A replacement token for the same bot, accepted the way Settings
        // accepts one** (`Telegram` L4): the generation moves on and the server
        // is left on the old one.
        let accepted = told_record::a_replacement_token_accepted(
            ours.c(),
            &ours.store,
            a_passed_check_for(7, "smoozy_a7k2_bot", "7:the-second-made-up-token"),
        )
        .expect("the replacement is accepted");
        assert_eq!(accepted.left_on_an_old_generation.len(), 1);
        assert_eq!(ours.not_reached(), Some(vec!["render box".to_string()]));
        assert_eq!(
            ours.the_material().what_this_server_is_owed(),
            WhatThisServerIsOwed { the_files: true, the_desired_state: None }
        );

        assert_eq!(
            what_the_server_was_not_told_is_told(&sessions, &area, &mut ours.the_material()).await,
            WhatTheTellingCameTo::Told(WhatThisServerIsOwed { the_files: true, the_desired_state: None })
        );
        assert_eq!(
            std::fs::read(machine.area.join("telegram-token")).expect("the token file reads"),
            b"7:the-second-made-up-token",
            "the machine holds the new token"
        );
        use smoozy_telegram::told::AToldThing;
        assert_eq!(ours.told(AToldThing::TheTokenGeneration).as_deref(), Some("2"));
        assert_eq!(ours.not_reached(), Some(Vec::new()), "Settings' list empties on the connection");
        assert_eq!(
            what_the_server_was_not_told_is_told(&sessions, &area, &mut ours.the_material()).await,
            WhatTheTellingCameTo::NothingWasOwed
        );
        drop(lending);
    }

    /// **`Specs/Telegram`: turning it off, each machine *"falls silent the next
    /// time Smoozy connects to it"*** — the role's state written *disabled* on
    /// the ordinary path (`Rented-server runtime` W8), and the told-record moved
    /// on the acknowledgement.
    #[tokio::test]
    async fn telegram_switched_off_reaches_the_server_on_the_next_connection() {
        let Some(machine) = TheMachine::standing("switched-off") else {
            println!("SKIPPED: no OpenSSH sftp-server, or the machine's program is not built beside this test.");
            return;
        };
        let ours = OursHere::for_the(&machine);
        let (sessions, lending) = connected(&machine).await;
        let area = machine.area.display().to_string();
        as_a_setup_with_telegram_on_leaves_it(&sessions, &area, &ours).await;
        assert_eq!(ours.not_reached(), Some(Vec::new()), "a setup leaves nobody unreached");
        assert_eq!(the_roles_state(&sessions, &area).await, Some(true));

        // **Switched off the way Settings switches it**: the switch alone. The
        // token's item is kept since 2026-10-31 (`Security and licence` E14,
        // the fifteenth notes, note 1).
        smoozy_desktop::record::settings::telegram_was_switched(ours.c(), false).expect("the switch");
        assert_eq!(ours.not_reached(), Some(vec!["render box".to_string()]));

        assert_eq!(
            what_the_server_was_not_told_is_told(&sessions, &area, &mut ours.the_material()).await,
            WhatTheTellingCameTo::Told(WhatThisServerIsOwed { the_files: false, the_desired_state: Some(false) })
        );
        assert_eq!(the_roles_state(&sessions, &area).await, Some(false), "the server falls silent");
        use smoozy_telegram::told::AToldThing;
        assert_eq!(ours.told(AToldThing::TheRolesDesiredState).as_deref(), Some("disabled"));
        assert_eq!(ours.not_reached(), Some(Vec::new()));
        drop(lending);
    }

    // ════════════════════════════════════════════════════════════════════
    // The Telegram worker's piece 7: new material reaches a running worker
    // ════════════════════════════════════════════════════════════════════

    /// The second made-up token, as a replacement leaves it on the machine.
    const THE_SECOND_TOKEN: &[u8] = b"7:the-second-made-up-token";

    /// **Which of the two the desk sent, in order.**
    fn which(asked: &[TheRoleAsked]) -> Vec<&'static str> {
        asked
            .iter()
            .map(|one| if one.enabled { "enabled" } else { "disabled" })
            .collect()
    }

    /// **`Telegram` L4's sentence of 2026-10-12 and §20: a replaced token
    /// on a server whose role is on reaches the running worker** — after the placement,
    /// *disabled* then *enabled* on the same connection, so the role worker is
    /// started again and reads the new token (`Telegram` K2 reads it at start
    /// only).
    ///
    /// **Undone, this fails at the pair**: the connection placed the new file
    /// and sent nothing else, so the token file held the new bytes and the
    /// worker running there went on with the old ones.
    #[tokio::test]
    async fn a_new_token_on_a_server_whose_role_is_on_starts_the_role_worker_again() {
        let Some(machine) = TheMachine::standing("h3-on") else {
            println!("SKIPPED: no OpenSSH sftp-server, or the machine's program is not built beside this test.");
            return;
        };
        let ours = OursHere::for_the(&machine);
        let (sessions, lending) = connected(&machine).await;
        let area = machine.area.display().to_string();
        as_a_setup_with_telegram_on_leaves_it(&sessions, &area, &ours).await;
        use smoozy_telegram::told::AToldThing;
        assert_eq!(ours.told(AToldThing::TheRolesDesiredState).as_deref(), Some("enabled"));
        let before = machine.how_often_the_role_was_asked();

        told_record::a_replacement_token_accepted(
            ours.c(),
            &ours.store,
            a_passed_check_for(7, "smoozy_a7k2_bot", "7:the-second-made-up-token"),
        )
        .expect("the replacement is accepted");
        assert_eq!(
            what_the_server_was_not_told_is_told(&sessions, &area, &mut ours.the_material()).await,
            WhatTheTellingCameTo::Told(WhatThisServerIsOwed { the_files: true, the_desired_state: None })
        );

        let asked = machine.the_role_asked_since(before);
        assert_eq!(which(&asked), vec!["disabled", "enabled"], "the pair, in its order: {asked:#?}");
        for one in &asked {
            assert_eq!(
                one.the_token_file_held.as_deref(),
                Some(THE_SECOND_TOKEN),
                "the pair follows the placement, so the worker started reads the new token: {asked:#?}"
            );
        }
        let disabled = asked[0].the_role_worker.clone().expect("the machine said what it did");
        let enabled = asked[1].the_role_worker.clone().expect("the machine said what it did");
        assert!(disabled.starts_with("the role is disabled"), "{disabled}");
        // **The role worker was started again, in the machine's own words.**
        // Where the kernel records a pair and the old worker was still up when
        // it was signalled, *enabled* finds it not yet ended and the
        // supervisor's own turn starts the new one when its ending is seen
        // (`Rented-server runtime` W10) — the only other answer that is right.
        // *(2026-10-24, the build runner, after step 12's SC0b: the desk's
        // development program now takes the pair from macOS's own process
        // table, so this branch is met at the desk too; neither branch asks
        // which system this is any more.)*
        let started_on_the_turn = disabled.contains("the worker was signalled")
            && enabled == "the role worker is already running";
        // **At the desk it is not started again, and the machine says why.**
        // Added 2026-10-16 by the Telegram worker's piece 2, when the worker
        // mode became the bot itself: until then it ended the instant it began,
        // so *enabled* always found nothing running and started one. **Now the
        // worker stays up**, and the desk publishes no per-process records, so
        // no pair was taken for it and none is invented to aim the stop by
        // (the Telegram worker's map, §6 item 2): *disabled* says it was not
        // signalled for that reason, and *enabled* finds it still running. On
        // a rented machine a pair is taken and the branch above is the one met.
        let not_signalled_at_the_desk = disabled.contains("was not signalled: no pair was taken")
            && enabled == "the role worker is already running";
        assert!(
            enabled.starts_with("started") || started_on_the_turn || not_signalled_at_the_desk,
            "the role worker is started again: {disabled} / {enabled}"
        );

        assert_eq!(the_roles_state(&sessions, &area).await, Some(true), "the role is on");
        assert_eq!(
            std::fs::read(machine.area.join("telegram-token")).expect("the token file reads"),
            THE_SECOND_TOKEN
        );
        assert_eq!(ours.told(AToldThing::TheRolesDesiredState).as_deref(), Some("enabled"));
        assert_eq!(ours.told(AToldThing::TheTokenGeneration).as_deref(), Some("2"));
        assert_eq!(ours.not_reached(), Some(Vec::new()));
        let before = machine.how_often_the_role_was_asked();
        assert_eq!(
            what_the_server_was_not_told_is_told(&sessions, &area, &mut ours.the_material()).await,
            WhatTheTellingCameTo::NothingWasOwed
        );
        assert!(machine.the_role_asked_since(before).is_empty(), "nothing owed, nothing restarted");
        drop(lending);
    }

    /// **A server whose role is off gets no *disabled* then *enabled***: there
    /// is no running worker to restart, so the new material is followed by the
    /// *enabled* the server is owed and by nothing else (`Telegram` L4's sentence
    /// of 2026-10-12 binds only a machine whose role is enabled).
    #[tokio::test]
    async fn a_new_token_on_a_server_whose_role_is_off_sends_no_disabled() {
        let Some(machine) = TheMachine::standing("h3-off") else {
            println!("SKIPPED: no OpenSSH sftp-server, or the machine's program is not built beside this test.");
            return;
        };
        let ours = OursHere::for_the(&machine);
        let (sessions, lending) = connected(&machine).await;
        let area = machine.area.display().to_string();
        as_a_setup_with_telegram_on_leaves_it(&sessions, &area, &ours).await;
        smoozy_desktop::record::settings::telegram_was_switched(ours.c(), false).expect("the switch");
        assert_eq!(
            what_the_server_was_not_told_is_told(&sessions, &area, &mut ours.the_material()).await,
            WhatTheTellingCameTo::Told(WhatThisServerIsOwed { the_files: false, the_desired_state: Some(false) })
        );
        assert_eq!(the_roles_state(&sessions, &area).await, Some(false), "the role is off");
        let before = machine.how_often_the_role_was_asked();

        // **A new token, and Telegram on again**: the files and the switch are
        // both owed.
        told_record::a_replacement_token_accepted(
            ours.c(),
            &ours.store,
            a_passed_check_for(7, "smoozy_a7k2_bot", "7:the-second-made-up-token"),
        )
        .expect("the replacement is accepted");
        smoozy_desktop::record::settings::telegram_was_switched(ours.c(), true).expect("the switch");
        let owed = WhatThisServerIsOwed { the_files: true, the_desired_state: Some(true) };
        assert_eq!(ours.the_material().what_this_server_is_owed(), owed);
        assert_eq!(
            what_the_server_was_not_told_is_told(&sessions, &area, &mut ours.the_material()).await,
            WhatTheTellingCameTo::Told(owed)
        );

        let asked = machine.the_role_asked_since(before);
        assert_eq!(which(&asked), vec!["enabled"], "no disabled to a role that is off: {asked:#?}");
        assert_eq!(asked[0].the_token_file_held.as_deref(), Some(THE_SECOND_TOKEN), "{asked:#?}");
        assert_eq!(the_roles_state(&sessions, &area).await, Some(true));
        assert_eq!(ours.not_reached(), Some(Vec::new()));
        drop(lending);
    }

    /// **The setup's Telegram step on a server whose role is on** — *Set up
    /// again* after a token replacement: the step places the new token and
    /// then sends *disabled* then *enabled* (`Telegram` §20's sentence of
    /// 2026-10-12, to `Server Setup`), and the run still reaches *done*.
    #[tokio::test]
    async fn set_up_again_on_a_server_whose_role_is_on_starts_the_role_worker_again() {
        let Some(machine) = TheMachine::standing("h3-setup") else {
            println!("SKIPPED: no OpenSSH sftp-server, or the machine's program is not built beside this test.");
            return;
        };
        let ours = OursHere::for_the(&machine);
        let (sessions, lending) = connected(&machine).await;
        let area = machine.area.display().to_string();
        as_a_setup_with_telegram_on_leaves_it(&sessions, &area, &ours).await;
        told_record::a_replacement_token_accepted(
            ours.c(),
            &ours.store,
            a_passed_check_for(7, "smoozy_a7k2_bot", "7:the-second-made-up-token"),
        )
        .expect("the replacement is accepted");
        let before = machine.how_often_the_role_was_asked();

        let mut row = TheServersRowHere { record: &ours.record, server: ours.server.clone() };
        let mut telegram = ours.the_material();
        let came_to = smoozy_desktop::setting_up::a_machine_is_set_up(
            &sessions,
            &mut AnUbuntuMachine,
            &mut row,
            NoPlacementOnASetUpServer,
            &mut NoReleaseOnASetUpServer,
            &smoozy_desktop::setting_up::WhatWasAskedFor { install_this_blender: None, telegram_is_on: true, ..Default::default() },
            smoozy_server_setup::elevated::HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
            &mut telegram,
            &mut NobodyIsThereToAsk,
            &NobodyAsked,
            None,
            Some(smoozy_state::identities::InstallationCheck::PresentAndEqualConfirmed),
        )
        .await;
        assert!(
            matches!(came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. }),
            "{came_to:?}"
        );
        let asked = machine.the_role_asked_since(before);
        assert_eq!(which(&asked), vec!["disabled", "enabled"], "the pair, in its order: {asked:#?}");
        for one in &asked {
            assert_eq!(one.the_token_file_held.as_deref(), Some(THE_SECOND_TOKEN), "{asked:#?}");
        }
        assert_eq!(the_roles_state(&sessions, &area).await, Some(true), "the role is on");
        use smoozy_telegram::told::AToldThing;
        assert_eq!(ours.told(AToldThing::TheRolesDesiredState).as_deref(), Some("enabled"));
        assert_eq!(ours.told(AToldThing::TheTokenGeneration).as_deref(), Some("2"));
        assert_eq!(ours.not_reached(), Some(Vec::new()));
        drop(lending);
    }

    /// A passed check for a bot, made through the walkthrough against the
    /// Telegram crate's stand-in service — nothing reaches Telegram.
    fn a_passed_check_for(
        id: i64,
        username: &str,
        token: &str,
    ) -> smoozy_telegram::walkthrough::TheCheckPassed {
        use smoozy_telegram::transport::a_stand_in::AStandInService;
        use smoozy_telegram::transport::TheClient;
        use smoozy_telegram::walkthrough::{check_the_bot_answers_through, ThePaste, WhatTheCheckFound};
        let service = Arc::new(AStandInService::default());
        service.then_answers(&format!(
            r#"{{"ok":true,"result":{{"id":{id},"is_bot":true,"first_name":"Smoozy","username":"{username}"}}}}"#
        ));
        let reach = move |_: &str| TheClient::over(service.clone());
        match check_the_bot_answers_through(&ThePaste::what_the_user_pasted(token), &reach) {
            WhatTheCheckFound::Answered(check) => check,
            other => panic!("the stand-in answers: {other:?}"),
        }
    }
}

//! **Step 10c piece 4's own proofs at the desk: a real folder listed, and a
//! real upload run to published bytes, through the server's own point — and the
//! installation check answering at a session limit of one.**
//!
//! **What the far end is.** A machine standing in for a rental, with a session
//! limit the way `MaxSessions` gives one: every session it opens is **OpenSSH's
//! own `sftp-server`**, spoken to over a Unix socket pair — the program a
//! rental's own file-transfer server is, and the one every observation behind
//! `Server protocol and transfers` §7 was taken against. **What it is not is an
//! SSH connection**: this crate carries no SSH server to stand one up, so the
//! loopback SSH server outside the repository is where that is run, and the
//! account says what it showed. What this file proves is everything above the
//! transport — the per-server point, the reading session, a transfer's seat,
//! the walk, the request, the byte loop and the publication.
//!
//! **The supervisor is answered by the stand-in**, which reads the framed
//! health request off the command's standard input and writes a framed answer
//! back — after checking that the carriage's link was planted on the machine
//! and that nothing else held a session when the command ran.
//!
//! **Nothing of the Creator's is touched.** Every path is inside a throwaway
//! folder made for the test and deleted with it.
//!
//! **Where `sftp-server` is not on this system the tests say so and stop**, and
//! **the socket pair is a Unix one**, so on Windows the whole file says that and
//! stops.
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
        "SKIPPED: the far end here is OpenSSH's own sftp-server on a Unix socket pair, which \
         this system ({}) has none of. The listing and the upload are unproved here.",
        std::env::consts::OS
    );
}

#[cfg(unix)]
mod on_a_desk_with_a_socket_pair {
    use std::path::{Path, PathBuf};
    use std::pin::Pin;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

    use smoozy_desktop::downloading::TheRunsTransfers;
    use smoozy_desktop::record::upload_requests::UploadRequests;
    use smoozy_desktop::record::LocalRecord;
    use smoozy_desktop::scheduling::{BytesPerSecond, TheCap};
    use smoozy_desktop::servers::installation::{
        the_check_through_a_point_of_its_own, where_the_package_area_is, OverTheConnection,
        TheMachinesInstallationIdentity, WhereThePackageAreaIs,
    };
    use smoozy_desktop::servers::sessions::{
        lend_until_it_ends, AHeldConnection, NoSession, TheAdditionalConnection,
        ThisServersSessions,
    };
    use smoozy_desktop::surface::files::{a_folder_is_read, WhatTheReadingFound};
    use smoozy_desktop::uploading::{
        an_upload_is_prepared, the_answer_is_applied, the_upload_runs, TheRecordAnUploadWrites,
        WhatIsBeingUploaded, WhatTheAnswerLeft, WhenAFileIsPublished,
    };
    use smoozy_file_browser::already_exists::CollisionAnswer;
    use smoozy_file_browser::request::KeepsTheRequest;
    use smoozy_server_protocol::framing::{
        read_one_request, write_result_frame, Answered, Fields,
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
    };
    use smoozy_server_protocol::placement::{
        THE_ENTRY_POINTS_CARRIAGE_SITE, THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR,
        THE_HEALTH_EXCHANGE_COMMAND,
    };
    use smoozy_server_protocol::reconnection::THE_RECORD_THE_IDENTITY_SITS_IN;
    use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};
    use smoozy_servers::acting::TheIdentityRead;
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use smoozy_servers::store::{RecordedMachineRoot, ServerRow};
    use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};
    use smoozy_state::outcomes::UnitOutcome;

    // --------------------------------------- where OpenSSH's own server is

    const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
        "/usr/libexec/sftp-server",
        "/usr/lib/openssh/sftp-server",
        "/usr/libexec/openssh/sftp-server",
        "/usr/lib/ssh/sftp-server",
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
                         The listing and the upload are unproved here.",
                        std::env::consts::OS
                    );
                    return;
                }
            }
        };
    }

    // --------------------------------------------------- the throwaway folder

    struct Scratch {
        at: PathBuf,
    }

    impl Scratch {
        fn new(named: &str) -> Scratch {
            Scratch::under(std::env::temp_dir(), named)
        }

        /// **A throwaway folder a machine root may be recorded in**: cargo's
        /// own folder for integration tests rather than the system's
        /// temporary one, because **`Job files and outputs` L3 refuses a root
        /// under a shared temporary area** and on Linux the system's temporary
        /// folder is `/tmp` itself. On a Mac it is under `/var/folders`, which
        /// the canonical form spells `/private/var/folders`, so L3 never saw
        /// it there — hosted run 35459394239 is where this was found.
        fn where_a_root_may_sit(named: &str) -> Scratch {
            Scratch::under(PathBuf::from(env!("CARGO_TARGET_TMPDIR")), named)
        }

        fn under(folder: PathBuf, named: &str) -> Scratch {
            let at = folder.join(format!("smzy-point-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&at);
            std::fs::create_dir_all(&at).expect("the throwaway folder is made");
            // **The canonical form**, so a path the machine reports and a path
            // composed here spell one folder the same way.
            let at = std::fs::canonicalize(&at).expect("the folder is there");
            Scratch { at }
        }

        fn folder(&self, named: &str) -> PathBuf {
            let at = self.at.join(named);
            std::fs::create_dir_all(&at).expect("the folder is made");
            at
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.at);
        }
    }

    fn the_bytes_of(path: &Path) -> Vec<u8> {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    }

    fn a_file_of(size: usize, seed: u8) -> Vec<u8> {
        (0..size)
            .map(|at| ((at % 251) as u8).wrapping_add(seed))
            .collect()
    }

    // ----------------------------------------------- a machine, standing in

    /// **What the stand-in's supervisor answers under.**
    struct TheSupervisor {
        installation: Identity,
        package_area: PathBuf,
        /// **Who answers a request other than health** — step 10c's Files'
        /// owed acts piece 7.
        answered_by: WhoAnswers,
    }

    /// **Who answers at the package area.**
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum WhoAnswers {
        /// The stand-in, with the health answer and nothing else.
        TheHealthStandIn,
        /// **The machine's own program this workspace built**, through its
        /// real adapter: the pointer in the package area names it.
        TheRealProgram,
        /// **A program older than the copy request**, which refuses it by
        /// name as every package does a name it does not know.
        AnOlderProgram,
    }

    /// **A machine with a session limit, whose every session is the machine's
    /// own `sftp-server`.**
    struct AMachine {
        program: PathBuf,
        home: PathBuf,
        limit: usize,
        supervisor: Option<TheSupervisor>,
        /// What happened, in order, for the assertions.
        log: Mutex<Vec<String>>,
        /// Transfer seats given on the additional connection, counted as they
        /// open.
        additional_sessions_opened: AtomicUsize,
    }

    impl AMachine {
        fn says(&self, what: String) {
            self.log.lock().expect("the log").push(what);
        }

        fn what_happened(&self) -> Vec<String> {
            self.log.lock().expect("the log").clone()
        }

        fn a_connection(self: &Arc<Self>, which: &'static str) -> AConnectionToIt {
            AConnectionToIt {
                machine: Arc::clone(self),
                open: Arc::new(AtomicUsize::new(0)),
                which,
            }
        }
    }

    /// One connection to the stand-in, **with its own session count**, the way
    /// `MaxSessions` is per connection.
    struct AConnectionToIt {
        machine: Arc<AMachine>,
        open: Arc<AtomicUsize>,
        which: &'static str,
    }

    /// One session's bytes, and the program at the far end of them. **The
    /// session is freed when this goes**, as a real server frees one.
    struct ASession {
        stream: tokio::net::UnixStream,
        child: std::process::Child,
        open: Arc<AtomicUsize>,
    }

    impl Drop for ASession {
        fn drop(&mut self) {
            let _ = self.child.kill();
            let _ = self.child.wait();
            self.open.fetch_sub(1, Ordering::SeqCst);
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

    fn copy_of(descriptor: std::os::unix::io::RawFd) -> std::os::unix::io::RawFd {
        let copy = unsafe { libc::dup(descriptor) };
        assert!(copy >= 0, "the descriptor could not be copied");
        copy
    }

    impl AHeldConnection for AConnectionToIt {
        type Channel = ASession;

        async fn open_a_session(&mut self) -> Result<ASession, NoSession> {
            use std::os::unix::io::{AsRawFd, FromRawFd};
            let open = self.open.load(Ordering::SeqCst);
            if open >= self.machine.limit {
                self.machine
                    .says(format!("{}: session refused ({open} open)", self.which));
                return Err(NoSession::Refused);
            }
            let (ours, theirs) = std::os::unix::net::UnixStream::pair().expect("a socket pair");
            ours.set_nonblocking(true).expect("the end we drive");
            let reading = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let writing = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let child = std::process::Command::new(&self.machine.program)
                .current_dir(&self.machine.home)
                .stdin(reading)
                .stdout(writing)
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("the machine's own file-transfer server starts");
            drop(theirs);
            let now = self.open.fetch_add(1, Ordering::SeqCst) + 1;
            if self.which == "additional" {
                self.machine
                    .additional_sessions_opened
                    .fetch_add(1, Ordering::SeqCst);
            }
            self.machine
                .says(format!("{}: session opened ({now} open)", self.which));
            Ok(ASession {
                stream: tokio::net::UnixStream::from_std(ours).expect("the end we drive"),
                child,
                open: Arc::clone(&self.open),
            })
        }

        async fn a_command(
            &mut self,
            command: CommandString,
            frame: Option<Vec<u8>>,
        ) -> Result<CommandResult, NoSession> {
            let open = self.open.load(Ordering::SeqCst);
            if open >= self.machine.limit {
                self.machine
                    .says(format!("{}: command refused ({open} open)", self.which));
                return Err(NoSession::Refused);
            }
            self.machine.says(format!(
                "{}: command `{}` runs with {} open",
                self.which,
                command.as_str(),
                open + 1
            ));
            Ok(self.the_command_is_answered(command, frame))
        }

        async fn until_it_ends(&mut self) -> Ending {
            std::future::pending().await
        }

        async fn torn_down(self) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    impl AConnectionToIt {
        /// **The health exchange, answered as the entry point would** — where
        /// the carriage's link points at the package area.
        fn the_command_is_answered(
            &self,
            command: CommandString,
            frame: Option<Vec<u8>>,
        ) -> CommandResult {
            let nothing = CommandResult {
                output: Vec::new(),
                complaint: b"no such command here".to_vec(),
                status: Some(127),
            };
            let Some(supervisor) = self.machine.supervisor.as_ref() else {
                return nothing;
            };
            if command.as_str() != THE_HEALTH_EXCHANGE_COMMAND.as_str() {
                return nothing;
            }
            let link = self
                .machine
                .home
                .join(THE_ENTRY_POINTS_CARRIAGE_SITE.link_path_on_the_subsystem);
            let points_at = std::fs::read_link(&link).ok();
            self.machine
                .says(format!("carriage points at {points_at:?}"));
            if points_at.as_deref() != Some(supervisor.package_area.as_path()) {
                return nothing;
            }
            let Some(frame) = frame else {
                return nothing;
            };
            let Ok(request) = read_one_request(&mut &frame[..]) else {
                return nothing;
            };
            self.machine.says(format!("the entry point was asked `{}`", request.name));
            match supervisor.answered_by {
                WhoAnswers::TheHealthStandIn => {}
                WhoAnswers::TheRealProgram => {
                    return the_real_adapter_answers(&supervisor.package_area, &frame);
                }
                WhoAnswers::AnOlderProgram => {
                    return CommandResult {
                        output: write_result_frame(
                            &Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.7")
                                .saying_it_does_not_answer_that_request(&request.name),
                        ),
                        complaint: Vec::new(),
                        status: Some(0),
                    };
                }
            }
            let [version, run, installation, phase] = THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR;
            let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0")
                .carrying(
                    Fields::none()
                        .and(version, "0.1.0")
                        .and(
                            run,
                            Identity::issue(IdentityKind::Run)
                                .expect("randomness")
                                .as_text(),
                        )
                        .and(installation, supervisor.installation.as_text())
                        .and(phase, "\"Inspecting\""),
                );
            CommandResult {
                output: write_result_frame(&answered),
                complaint: Vec::new(),
                status: Some(0),
            }
        }
    }

    /// **The program this workspace built**, beside this test's own
    /// executable — present wherever the machine program's own tests were
    /// built. Nothing where it is not, and the test says so and stops.
    fn the_program_this_workspace_built() -> Option<PathBuf> {
        let here = std::env::current_exe().ok()?;
        let built = here
            .parent()?
            .parent()?
            .join(smoozy_runtime::the_program::THE_FIXED_ENTRY_POINT_NAME);
        built.is_file().then_some(built)
    }

    /// **A package area holding the real program**: the active-package
    /// pointer naming it, and the endpoint's recorded path the adapter reads
    /// before it reads a request. **No supervisor is started** — the copy is
    /// the adapter's own to answer.
    fn a_package_area_with_the_real_program(scratch: &Scratch, built: &Path) -> PathBuf {
        use smoozy_runtime::the_program::{records, the_active_package_pointer_at, the_record_at};
        let area = scratch.folder("package");
        let text = area.display().to_string();
        std::os::unix::fs::symlink(built, the_active_package_pointer_at(&text)).expect("the pointer is made");
        std::fs::write(
            the_record_at(&text, records::THE_ENDPOINTS_RECORDED_PATH),
            area.join("no-supervisor.sock").display().to_string(),
        )
        .expect("the endpoint's path is recorded");
        area
    }

    /// **One framed request through the real adapter at `area`**, with a
    /// test's own bound of a minute, and the adapter's output as the
    /// command's.
    fn the_real_adapter_answers(area: &Path, frame: &[u8]) -> CommandResult {
        use std::io::Write;
        let pointer = smoozy_runtime::the_program::the_active_package_pointer_at(&area.display().to_string());
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
            .write_all(frame)
            .expect("the request goes in");
        let identifier = adapter.id();
        let (sent, heard) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = sent.send(adapter.wait_with_output());
        });
        let Ok(Ok(said)) = heard.recv_timeout(std::time::Duration::from_secs(60)) else {
            let _ = std::process::Command::new("kill").args(["-KILL", &identifier.to_string()]).status();
            panic!("DEADLINE MISSED: the machine's own program never answered one asking");
        };
        CommandResult {
            output: said.stdout,
            complaint: said.stderr,
            status: said.status.code().and_then(|code| u32::try_from(code).ok()),
        }
    }

    /// **The point, lending the stand-in's primary connection**, with `Servers`
    /// S14's additional connection made on asking.
    struct APointLending {
        sessions: ThisServersSessions<AConnectionToIt>,
        lending: tokio::task::JoinHandle<Ending>,
    }

    impl APointLending {
        async fn over(machine: &Arc<AMachine>) -> APointLending {
            let sessions = ThisServersSessions::<AConnectionToIt>::for_one_server();
            let more = Arc::clone(machine);
            let additional: TheAdditionalConnection<AConnectionToIt> = Arc::new(move || {
                let more = Arc::clone(&more);
                Box::pin(async move { Some(more.a_connection("additional")) })
            });
            let mut primary = machine.a_connection("primary");
            let lending = tokio::spawn({
                let sessions = sessions.clone();
                async move { lend_until_it_ends(&sessions, &mut primary, &additional).await }
            });
            while !sessions.a_connection_is_live() {
                tokio::task::yield_now().await;
            }
            APointLending { sessions, lending }
        }
    }

    impl Drop for APointLending {
        fn drop(&mut self) {
            self.lending.abort();
        }
    }

    fn a_machine(program: PathBuf, home: &Path, limit: usize) -> Arc<AMachine> {
        Arc::new(AMachine {
            program,
            home: home.to_path_buf(),
            limit,
            supervisor: None,
            log: Mutex::new(Vec::new()),
            additional_sessions_opened: AtomicUsize::new(0),
        })
    }

    // ------------------------------------------------------------- the record

    /// **The application's own record**, opened on a throwaway file, reached
    /// one short act at a time.
    struct ARecordHere(Mutex<LocalRecord>);

    impl TheRecordAnUploadWrites for ARecordHere {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            match self.0.lock() {
                Ok(record) => {
                    act(record.connection());
                    true
                }
                Err(_) => false,
            }
        }
    }

    fn a_record_file_in(scratch: &Scratch) -> PathBuf {
        scratch.folder("record").join("smoozy.sqlite3")
    }

    /// Every publication, written down.
    #[derive(Default)]
    struct WhatWasPublished(Mutex<Vec<Vec<u8>>>, std::sync::atomic::AtomicUsize, std::sync::atomic::AtomicUsize);

    impl WhenAFileIsPublished for WhatWasPublished {
        fn a_file_was_published(&self, landing_path: &[u8], outcome: &UnitOutcome) {
            assert_eq!(outcome, &UnitOutcome::Done);
            // **`Dependency Upload & Remap` G17b: each file was said to be going up,
            // and to have ended, before it is told here as published.**
            let mut list = self.0.lock().expect("the list");
            list.push(landing_path.to_vec());
            assert!(self.2.load(std::sync::atomic::Ordering::SeqCst) >= list.len(), "published with nothing said to have gone up");
            assert!(self.1.load(std::sync::atomic::Ordering::SeqCst) >= self.2.load(std::sync::atomic::Ordering::SeqCst));
        }

        fn a_file_begins_going_up(&self) {
            self.1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }

        fn a_file_stopped_going_up(&self) {
            self.2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    fn an_upload_of(chosen: Vec<PathBuf>, into: &Path) -> WhatIsBeingUploaded {
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        WhatIsBeingUploaded {
            server_row: issue(IdentityKind::LocalServer),
            installation: Some(issue(IdentityKind::Installation)),
            destination: the_bytes_of(into),
            chosen,
            run: issue(IdentityKind::Run),
        }
    }

    // ---------------------------------------------------------------- the proofs

    /// **`File Browser` L1 through the server's point, at a limit of one and
    /// above it**: one reading of a real folder on the lent reading session,
    /// the free-space figure beside it, and the entries the disk holds.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_real_folder_is_listed_through_the_servers_own_point() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        for limit in [1, 10] {
            let scratch = Scratch::new(&format!("list-{limit}"));
            let browsed = scratch.folder("kitchen");
            std::fs::create_dir_all(browsed.join("textures")).expect("made");
            std::fs::write(browsed.join("scene.blend"), a_file_of(4096, 1)).expect("written");
            std::fs::write(browsed.join("notes.txt"), b"three lines").expect("written");
            let machine = a_machine(program.clone(), &scratch.at, limit);
            let point = APointLending::over(&machine).await;

            let found = a_folder_is_read(&point.sessions, &the_bytes_of(&browsed)).await;
            let WhatTheReadingFound::TheFolder { entries, free } = found else {
                panic!("the folder listed at a limit of {limit}: {found:?}");
            };
            let mut names: Vec<String> = entries
                .iter()
                .map(|entry| entry.name.for_display())
                .collect();
            names.sort();
            assert_eq!(names, vec!["notes.txt", "scene.blend", "textures"]);
            let scene = entries
                .iter()
                .find(|entry| entry.name.as_bytes() == b"scene.blend")
                .expect("the .blend");
            assert_eq!(scene.size.bytes(), 4096, "the size is the disk's");
            assert!(
                free.is_some_and(|bytes| bytes > 0),
                "the free-space figure rode the reading"
            );
        }
    }

    /// **`File Browser` P1 and P2: a folder this login may not read is told on
    /// its own row in its parent's listing, and a folder that is not there is
    /// said to be not there** — two different answers.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_refused_folder_is_told_on_its_row_and_a_missing_one_is_said_to_be_missing() {
        let _deadline = TheTestsDeadline::started();
        use std::os::unix::fs::PermissionsExt;
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("refused");
        let parent = scratch.folder("jobs");
        let closed = parent.join("private");
        std::fs::create_dir_all(&closed).expect("made");
        std::fs::set_permissions(&closed, std::fs::Permissions::from_mode(0o000)).expect("closed");
        if std::fs::read_dir(&closed).is_ok() {
            println!("SKIPPED: this login reads a folder whose mode is zero, so nothing here can refuse it.");
            let _ = std::fs::set_permissions(&closed, std::fs::Permissions::from_mode(0o755));
            return;
        }
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;

        let found = a_folder_is_read(&point.sessions, &the_bytes_of(&closed)).await;
        let _ = std::fs::set_permissions(&closed, std::fs::Permissions::from_mode(0o755));
        let WhatTheReadingFound::ItsRowCarriesTheRefusal {
            parent: listed,
            entries,
            ..
        } = found
        else {
            panic!("the refusal is told on the folder's row: {found:?}");
        };
        assert_eq!(listed, the_bytes_of(&parent));
        assert!(entries
            .iter()
            .any(|entry| entry.name.as_bytes() == b"private"));

        let gone =
            a_folder_is_read(&point.sessions, &the_bytes_of(&parent.join("never-made"))).await;
        let WhatTheReadingFound::NotListed(said) = gone else {
            panic!("a folder that is not there is not listed: {gone:?}");
        };
        // **`File Browser` P3: where the answer names absence there is nothing
        // to walk** — the answer is already which of the two it was, and the
        // words say that one and never the other.
        // The words became *Not found.* in step 13's words sweep (2026-10-27).
        let shown = format!("{said:?}");
        assert!(
            shown.contains("found") && !shown.contains("permitted"),
            "{shown}"
        );
    }

    /// **The upload, run to published bytes on the machine, at a limit of one
    /// and above it** — a folder and a loose file, every file at its landing
    /// path with the bytes it left with, the request's every unit *done*, and
    /// nothing left in the reserved staging folder.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn an_upload_runs_to_published_bytes_through_the_servers_own_point() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        for limit in [1, 10] {
            let scratch = Scratch::new(&format!("up-{limit}"));
            let here = scratch.folder("here");
            std::fs::create_dir_all(here.join("kitchen/textures")).expect("made");
            let scene = a_file_of(3 * 1024 * 1024 + 17, 3);
            let wood = a_file_of(200_000, 5);
            let notes = a_file_of(11, 7);
            std::fs::write(here.join("kitchen/scene.blend"), &scene).expect("written");
            std::fs::write(here.join("kitchen/textures/wood.png"), &wood).expect("written");
            std::fs::write(here.join("notes.txt"), &notes).expect("written");
            let there = scratch.folder("machine/jobs");

            let record = ARecordHere(Mutex::new(
                LocalRecord::open_at(&a_record_file_in(&scratch)).expect("the record opens"),
            ));
            let record: Arc<dyn TheRecordAnUploadWrites> = Arc::new(record);
            let machine = a_machine(program.clone(), &scratch.at, limit);
            let point = APointLending::over(&machine).await;
            let the_run = TheRunsTransfers::for_this_run();
            let published = WhatWasPublished::default();

            let asked = an_upload_of(vec![here.join("kitchen"), here.join("notes.txt")], &there);
            let mut ready = an_upload_is_prepared(&asked, &point.sessions, record.as_ref())
                .await
                .expect("the upload is prepared");
            assert!(!ready.something_collides(), "nothing is there yet");
            assert_eq!(
                the_answer_is_applied(&mut ready, None, &point.sessions, record.as_ref()).await,
                WhatTheAnswerLeft::ItGoes
            );
            let request = ready.request.record.clone();
            let came_to = the_upload_runs(
                ready,
                &point.sessions,
                &the_run,
                Arc::clone(&record),
                &published,
            )
            .await;
            assert_eq!(came_to.uploaded, 3, "at a limit of {limit}: {came_to:?}");
            assert!(came_to.failed.is_empty());

            assert_eq!(
                std::fs::read(there.join("kitchen/scene.blend")).expect("landed"),
                scene
            );
            assert_eq!(
                std::fs::read(there.join("kitchen/textures/wood.png")).expect("landed"),
                wood
            );
            assert_eq!(
                std::fs::read(there.join("notes.txt")).expect("landed"),
                notes
            );
            for folder in [
                there.clone(),
                there.join("kitchen"),
                there.join("kitchen/textures"),
            ] {
                let staging = folder.join(".smoozy-staging");
                let left = std::fs::read_dir(&staging)
                    .map(|dir| dir.count())
                    .unwrap_or(0);
                assert_eq!(left, 0, "nothing is left staged under {staging:?}");
            }
            assert_eq!(
                published.0.lock().expect("the list").len(),
                3,
                "`File Browser` X7a's call, at each publication"
            );

            // **`File Browser` U1: the request is in the record with every unit
            // done**, each with its moment (`File Browser` U9a).
            let mut kept = Vec::new();
            record.with_the_record(&mut |connection| {
                kept = UploadRequests::over(connection)
                    .every_request_for(&asked.server_row)
                    .expect("the requests read back");
            });
            let kept = kept
                .into_iter()
                .find(|one| one.record == request)
                .expect("the request is in the record");
            assert!(kept
                .entries
                .iter()
                .all(|entry| entry.outcome == Some(UnitOutcome::Done)
                    && entry.outcome_written_at.is_some()));
            if limit == 1 {
                // **At a limit of one, every transfer rode `Servers` S14's
                // additional connection**, and the primary held nothing but
                // the reading session.
                assert_eq!(machine.additional_sessions_opened.load(Ordering::SeqCst), 3);
                assert!(!machine
                    .what_happened()
                    .iter()
                    .any(|line| line.starts_with("primary: session opened (2 open)")));
            }
        }
    }

    /// **A write the machine refuses for permission is a failure, told as one
    /// with the server's own words** — `File Browser` P1 and `File Browser` P6a — **and never
    /// *did not finish*.** The seventh pass on Hyperstack dragged a file onto
    /// `/` with the `ubuntu` login and was told only *1 file did not finish
    /// uploading.*, with nothing on Transfers. **The Transfers page is told the
    /// unit ended**, which is what moves its row.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_refused_upload_is_a_failure_with_the_servers_words() {
        let _deadline = TheTestsDeadline::started();
        use std::os::unix::fs::PermissionsExt;
        let program = the_server_or_say_so!();

        let scratch = Scratch::new("up-refused");
        let here = scratch.folder("here");
        std::fs::write(here.join("cube.blend"), a_file_of(93_790, 3)).expect("written");
        let there = scratch.folder("machine/root-owned");
        std::fs::set_permissions(&there, std::fs::Permissions::from_mode(0o555)).expect("closed");
        if std::fs::create_dir(there.join("probe")).is_ok() {
            println!("SKIPPED: this login writes into a folder whose mode forbids it, so nothing here can refuse it.");
            let _ = std::fs::set_permissions(&there, std::fs::Permissions::from_mode(0o755));
            return;
        }

        /// The record door, counting what the Transfers page would be told.
        struct ACountingDoor(ARecordHere, AtomicUsize);
        impl TheRecordAnUploadWrites for ACountingDoor {
            fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
                self.0.with_the_record(act)
            }
            fn a_transfer_moved(&self) {
                self.1.fetch_add(1, Ordering::SeqCst);
            }
            /// **Counts units' endings and nothing else**: bytes moving inside a file,
            /// and a request's run ending, are told to the page by their own call since
            /// step 10c's transfer wave piece 5, and are not what this test counts.
            fn a_transfer_is_moving(&self) {}
        }
        let door = Arc::new(ACountingDoor(
            ARecordHere(Mutex::new(
                LocalRecord::open_at(&a_record_file_in(&scratch)).expect("the record opens"),
            )),
            AtomicUsize::new(0),
        ));
        let record: Arc<dyn TheRecordAnUploadWrites> = door.clone();
        let machine = a_machine(program.clone(), &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        let the_run = TheRunsTransfers::for_this_run();
        let published = WhatWasPublished::default();

        let asked = an_upload_of(vec![here.join("cube.blend")], &there);
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, record.as_ref())
            .await
            .expect("the upload is prepared");
        assert_eq!(
            the_answer_is_applied(&mut ready, None, &point.sessions, record.as_ref()).await,
            WhatTheAnswerLeft::ItGoes
        );
        let request = ready.request.record.clone();
        let came_to = the_upload_runs(
            ready,
            &point.sessions,
            &the_run,
            Arc::clone(&record),
            &published,
        )
        .await;
        let _ = std::fs::set_permissions(&there, std::fs::Permissions::from_mode(0o755));

        assert_eq!(came_to.did_not_finish, 0, "a refusal was told as *did not finish*: {came_to:?}");
        assert_eq!(came_to.failed.len(), 1, "{came_to:?}");
        assert_eq!(came_to.refused.len(), 1, "{came_to:?}");
        assert_eq!(came_to.refused[0].0, came_to.failed[0]);
        assert!(
            came_to.refused[0].1.contains("permission was denied"),
            "not the server's own answer: {:?}",
            came_to.refused[0].1
        );
        assert_eq!(
            smoozy_desktop::uploading::what_the_server_said_refusing(&request, &came_to.failed[0]),
            Some(came_to.refused[0].1.clone()),
            "the words the Transfers row shows are not held for it"
        );
        assert_eq!(door.1.load(Ordering::SeqCst), 1, "the page was not told the unit ended");

        let mut kept = Vec::new();
        record.with_the_record(&mut |connection| {
            kept = UploadRequests::over(connection)
                .every_request_for(&asked.server_row)
                .expect("the requests read back");
        });
        let kept = kept
            .into_iter()
            .find(|one| one.record == request)
            .expect("the request is in the record");
        assert_eq!(kept.entries[0].outcome, Some(UnitOutcome::Failed));
    }

    /// **`File Browser` U4's question, and each answer doing what it says**:
    /// Skip existing leaves the file there and sends the rest; Keep both lands
    /// a second copy under `File Browser` X9's generated name; Replace
    /// replaces.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_already_exists_answers_each_do_what_they_say() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        for (answer, word) in [
            (CollisionAnswer::SkipExisting, "skip"),
            (CollisionAnswer::KeepBoth, "both"),
            (CollisionAnswer::Replace, "replace"),
        ] {
            let scratch = Scratch::new(&format!("ask-{word}"));
            let here = scratch.folder("here");
            let there = scratch.folder("machine");
            let new = a_file_of(5000, 9);
            let old = a_file_of(100, 1);
            std::fs::write(here.join("scene.blend"), &new).expect("written");
            std::fs::write(here.join("fresh.txt"), b"fresh").expect("written");
            std::fs::write(there.join("scene.blend"), &old).expect("already there");

            let record: Arc<dyn TheRecordAnUploadWrites> = Arc::new(ARecordHere(Mutex::new(
                LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens"),
            )));
            let machine = a_machine(program.clone(), &scratch.at, 1);
            let point = APointLending::over(&machine).await;
            let the_run = TheRunsTransfers::for_this_run();
            let published = WhatWasPublished::default();

            let asked = an_upload_of(
                vec![here.join("scene.blend"), here.join("fresh.txt")],
                &there,
            );
            let mut ready = an_upload_is_prepared(&asked, &point.sessions, record.as_ref())
                .await
                .expect("prepared");
            assert!(ready.something_collides());
            assert_eq!(ready.the_walk.colliding, 1, "the headline is one file");
            assert_eq!(
                the_answer_is_applied(&mut ready, Some(answer), &point.sessions, record.as_ref())
                    .await,
                WhatTheAnswerLeft::ItGoes
            );
            let came_to = the_upload_runs(
                ready,
                &point.sessions,
                &the_run,
                Arc::clone(&record),
                &published,
            )
            .await;
            assert_eq!(
                std::fs::read(there.join("fresh.txt")).expect("the fresh file"),
                b"fresh"
            );
            match answer {
                CollisionAnswer::SkipExisting => {
                    assert_eq!(came_to.skipped, 1);
                    assert_eq!(std::fs::read(there.join("scene.blend")).expect("kept"), old);
                }
                CollisionAnswer::KeepBoth => {
                    assert_eq!(came_to.uploaded, 2);
                    assert_eq!(std::fs::read(there.join("scene.blend")).expect("kept"), old);
                    assert_eq!(
                        std::fs::read(there.join("scene (2).blend")).expect("beside it"),
                        new
                    );
                }
                _ => {
                    assert_eq!(came_to.uploaded, 2);
                    assert_eq!(
                        std::fs::read(there.join("scene.blend")).expect("replaced"),
                        new
                    );
                }
            }
        }
    }

    /// **A job waiting on the stand-in machine**, as a submission writes its
    /// provisional record, naming its `.blend` there. Step 10c's Files' owed
    /// acts piece 2.
    fn a_waiting_job(
        job: &Identity,
        server: &Identity,
        blend: &Path,
    ) -> smoozy_queue_history::record::LocalJobRecord {
        use smoozy_queue_history::record::*;
        LocalJobRecord {
            job: job.clone(),
            group: None,
            server: TheServerAsItStood {
                local_server: server.clone(),
                label: None,
                hue: None,
            },
            submitted_at: None,
            label: Some("Kitchen".to_string()),
            blend_path_on_the_machine: Some(blend.to_str().expect("text").to_string()),
            scene_name: None,
            submitted_with_no_scene_named: false,
            resolution_percentage_typed: None,
            picture_size_from_the_read: None,
            main_output_from_the_read: None,
            blender: TheBlenderTheEntryKeeps {
                pick: smoozy_render_settings::payload::VersionPick::of(
                    &smoozy_render_settings::file_half::BlenderIdentity {
                        path: "/ephemeral/smoozy/blender/b".to_string(),
                        version: smoozy_server_setup::blender::Version { major: 4, minor: 5, patch: 0 },
                    },
                ),
                origin: AnswerOrigin::ResolvedForThem,
            },
            device_asked_for: TheDeviceAsk::TheFilesOwnUnread,
            the_files_values_beside_the_overrides: None,
            dependency_check: None,
            carries_on_from: None,
            disturbances: vec![],
            attempts_where_the_file_changed: vec![],
            the_frames: smoozy_queue_history::record::TheFramesTheReadFound::default(),
            standing: Standing::Provisional,
            the_render_start_has_been_learned: false,
            what_the_note_carried: WhatTheNoteCarried::default(),
            the_job_output_folder: Default::default(),
            the_driver_was_below_the_bar: None,
        }
    }

    /// The marks a job's record carries now.
    fn the_marks_of(
        record: &dyn TheRecordAnUploadWrites,
        job: &Identity,
    ) -> Vec<smoozy_queue_history::record::ADisturbance> {
        let mut marks = Vec::new();
        record.with_the_record(&mut |connection| {
            marks = smoozy_desktop::record::job_records::JobRecords::over(connection)
                .read(job)
                .expect("the record reads")
                .expect("the job's record stands")
                .disturbances;
        });
        marks
    }

    /// **`File Browser` W8 and `File Browser` W9 through the server's own point, onto the
    /// real record: an upload's *Replace* over a waiting job's `.blend` tells
    /// the job before anything moves, and the mark is finished from the
    /// unit.** Step 10c's Files' owed acts piece 2.
    ///
    /// - **The telling and the mark stand first**: once the telling is
    ///   written and before any file moves, the job carries one mark reading
    ///   *may have been disturbed*, naming the file — which is also what a run
    ///   killed at that moment leaves.
    /// - **A unit that published reads *disturbed***.
    /// - **A unit that took no effect removes the mark** — the file it was to
    ///   send was gone from this computer, so nothing was written over.
    /// - **A telling that cannot be written is the act not attempted**: the
    ///   marks' table refuses, so neither the telling nor any mark stands and
    ///   nothing is replaced.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_replace_over_a_live_jobs_file_tells_the_job_first_and_finishes_its_mark() {
        use smoozy_queue_history::record::DisturbanceReading;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        for case in ["landed", "took-no-effect", "telling-refused"] {
            let scratch = Scratch::new(&format!("tell-{case}"));
            let here = scratch.folder("here");
            let there = scratch.folder("machine");
            let new = a_file_of(5000, 9);
            let old = a_file_of(100, 1);
            std::fs::write(here.join("scene.blend"), &new).expect("written");
            std::fs::write(there.join("scene.blend"), &old).expect("already there");

            let record: Arc<dyn TheRecordAnUploadWrites> = Arc::new(ARecordHere(Mutex::new(
                LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens"),
            )));
            let asked = an_upload_of(vec![here.join("scene.blend")], &there);
            let job = Identity::issue(IdentityKind::Job).expect("randomness");
            record.with_the_record(&mut |connection| {
                smoozy_desktop::record::job_records::JobRecords::over(connection)
                    .write_the_provisional_record(&a_waiting_job(
                        &job,
                        &asked.server_row,
                        &there.join("scene.blend"),
                    ))
                    .expect("the job's record");
            });
            let machine = a_machine(program.clone(), &scratch.at, 1);
            let point = APointLending::over(&machine).await;
            let the_run = TheRunsTransfers::for_this_run();
            let published = WhatWasPublished::default();

            let mut ready = an_upload_is_prepared(&asked, &point.sessions, record.as_ref())
                .await
                .expect("prepared");
            assert!(ready.something_collides());

            // **Piece 1's reading names the job**, through the one composition.
            let mut holds = None;
            record.with_the_record(&mut |connection| {
                holds = smoozy_desktop::record::neighbouring_records::WhatTheLocalRecordHolds::read(
                    connection,
                    &smoozy_dependencies::record::MachineKey::of(
                        asked.server_row.clone(),
                        asked.installation.clone().expect("an installation"),
                    ),
                    smoozy_job_files::layout::TheLayout::of(
                        smoozy_job_files::layout::MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
                        smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
                    ),
                    None,
                )
                .ok();
            });
            let (_, named, _) =
                smoozy_desktop::surface::files::the_upload_warnings(&ready.the_walk, holds.as_ref());
            assert_eq!(named.len(), 1, "the waiting job was not named");

            if case == "telling-refused" {
                record.with_the_record(&mut |connection| {
                    connection
                        .execute_batch("DROP TABLE job_record_disturbance")
                        .expect("the marks' table goes");
                });
                assert!(
                    smoozy_desktop::uploading::the_telling_stands_first(&mut ready, &named, record.as_ref())
                        .is_err(),
                    "a telling the record refused was taken as written"
                );
                assert!(ready.request.telling.is_none());
                let mut told = 1;
                record.with_the_record(&mut |connection| {
                    told = connection
                        .query_row("SELECT count(*) FROM upload_request_telling_job", [], |row| {
                            row.get::<_, i64>(0)
                        })
                        .expect("the count");
                });
                assert_eq!(told, 0, "the telling stood without its marks");
                assert_eq!(
                    std::fs::read(there.join("scene.blend")).expect("untouched"),
                    old,
                    "nothing is replaced"
                );
                continue;
            }

            smoozy_desktop::uploading::the_telling_stands_first(&mut ready, &named, record.as_ref())
                .expect("the telling stands");
            // **The moment a run killed here leaves**: the mark, unfinished.
            let marks = the_marks_of(record.as_ref(), &job);
            assert_eq!(marks.len(), 1, "{marks:?}");
            assert_eq!(marks[0].reading, DisturbanceReading::MayHaveBeenDisturbed);
            assert_eq!(marks[0].the_act_intended, "replace");
            assert_eq!(marks[0].the_member, there.join("scene.blend").to_str().unwrap());

            assert_eq!(
                the_answer_is_applied(
                    &mut ready,
                    Some(CollisionAnswer::Replace),
                    &point.sessions,
                    record.as_ref()
                )
                .await,
                WhatTheAnswerLeft::ItGoes
            );
            if case == "took-no-effect" {
                std::fs::remove_file(here.join("scene.blend")).expect("the source goes");
            }
            let came_to = the_upload_runs(
                ready,
                &point.sessions,
                &the_run,
                Arc::clone(&record),
                &published,
            )
            .await;
            let marks = the_marks_of(record.as_ref(), &job);
            if case == "landed" {
                assert_eq!(came_to.uploaded, 1, "{came_to:?}");
                assert_eq!(std::fs::read(there.join("scene.blend")).expect("replaced"), new);
                assert_eq!(marks.len(), 1);
                assert_eq!(marks[0].reading, DisturbanceReading::Disturbed);
            } else {
                assert_eq!(came_to.uploaded, 0, "{came_to:?}");
                assert_eq!(std::fs::read(there.join("scene.blend")).expect("kept"), old);
                assert!(marks.is_empty(), "a mark stands over a file that never moved: {marks:?}");
            }
        }
    }

    // ── Step 10c's Files' owed acts piece 3: New folder and Rename ────────

    /// **What a unit of an act owed the check record**, written down so a test
    /// can read what the act asked for.
    #[derive(Default)]
    struct WhatWasMarked(Mutex<Vec<(Vec<u8>, smoozy_file_browser::managing::WhatTheUnitDid, UnitOutcome)>>);

    impl smoozy_desktop::managing_files::WhenAUnitOfAnActEnds for WhatWasMarked {
        fn a_unit_ended(
            &self,
            location: &[u8],
            did: smoozy_file_browser::managing::WhatTheUnitDid,
            outcome: &UnitOutcome,
        ) {
            self.0.lock().expect("the list").push((location.to_vec(), did, outcome.clone()));
        }
    }

    /// **`File Browser` M9 against OpenSSH's own `sftp-server`, through the
    /// server's own point: a new folder is the plain make-directory, and a
    /// name already there is refused — never merged — whether the listing the
    /// user is looking at showed it or only the machine knew.** A name the row
    /// refuses before trying sends nothing. Step 10c's Files' owed acts piece
    /// 3.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_new_folder_is_made_plainly_and_a_name_already_there_is_refused_never_merged() {
        use smoozy_desktop::managing_files::{a_folder_is_made, WhatTheActCameTo};
        use smoozy_file_browser::managing::WhyTheNameIsRefused;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("new-folder");
        let kitchen = scratch.folder("kitchen");
        std::fs::create_dir_all(kitchen.join("textures")).expect("made");
        std::fs::write(kitchen.join("textures").join("wood.png"), b"wood").expect("written");
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        let here = the_bytes_of(&kitchen);

        // A free name: made, and empty.
        assert_eq!(a_folder_is_made(&point.sessions, &here, b"caches", &[]).await, WhatTheActCameTo::Done);
        assert!(kitchen.join("caches").is_dir());
        assert_eq!(std::fs::read_dir(kitchen.join("caches")).expect("reads").count(), 0);

        // **The machine's own refusal**: the listing the user was looking at
        // was stale and did not show `textures`, and the plain form refused.
        assert_eq!(
            a_folder_is_made(&point.sessions, &here, b"textures", &[]).await,
            WhatTheActCameTo::TheNameIsTaken
        );
        assert_eq!(std::fs::read(kitchen.join("textures").join("wood.png")).expect("kept"), b"wood");
        assert_eq!(std::fs::read_dir(kitchen.join("textures")).expect("reads").count(), 1);
        // **And answered from the listing before any attempt** where it shows it.
        assert_eq!(
            a_folder_is_made(&point.sessions, &here, b"textures", &[b"textures".to_vec()]).await,
            WhatTheActCameTo::TheNameIsTaken
        );

        // **Refused on the row before anything is sent.**
        assert_eq!(
            a_folder_is_made(&point.sessions, &here, b"a/b", &[]).await,
            WhatTheActCameTo::TheNameIsRefused(WhyTheNameIsRefused::HoldsASlash)
        );
        assert!(!kitchen.join("a").exists(), "a name holding a slash reached the machine");
        assert_eq!(
            a_folder_is_made(&point.sessions, &here, b".smoozy-staging", &[]).await,
            WhatTheActCameTo::TheNameIsRefused(WhyTheNameIsRefused::IsTheStagingFoldersName)
        );
        assert!(!kitchen.join(".smoozy-staging").exists());

        // A folder that is not there to make one in.
        assert_eq!(
            a_folder_is_made(&point.sessions, &the_bytes_of(&kitchen.join("gone")), b"caches", &[]).await,
            WhatTheActCameTo::ItIsNotThere
        );
    }

    /// **`File Browser` M4 against OpenSSH's own `sftp-server`, through the
    /// server's own point: the base rename's own refusal is what reports
    /// *taken*, and nothing is replaced.** A rename onto an existing name
    /// leaves both files byte for byte; a rename onto a free name goes through
    /// whatever the listing on the page had shown, since the rename is always
    /// attempted (`File Browser` M4: the listing only hints). **The check
    /// record is asked for `File Browser` W11's mark at the place left only where the rename
    /// took effect** (`File Browser` W11 as widened 2026-10-02). Step 10c's
    /// Files' owed acts piece 3.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_rename_onto_a_name_already_there_is_refused_and_both_files_are_untouched() {
        use smoozy_desktop::managing_files::{a_rename_is_made, WhatTheActCameTo};
        use smoozy_file_browser::managing::WhatTheUnitDid;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("rename");
        let kitchen = scratch.folder("kitchen");
        let a = a_file_of(3000, 1);
        let b = a_file_of(2000, 2);
        std::fs::write(kitchen.join("a.png"), &a).expect("written");
        std::fs::write(kitchen.join("b.png"), &b).expect("written");
        std::fs::create_dir_all(kitchen.join("tex")).expect("made");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        let marked = WhatWasMarked::default();
        let path = |name: &str| the_bytes_of(&kitchen.join(name));

        // **Taken**: refused by the machine, and both files as they were.
        assert_eq!(
            a_rename_is_made(&point.sessions, &record, &marked, &path("a.png"), b"b.png", &[]).await,
            WhatTheActCameTo::TheNameIsTaken
        );
        assert_eq!(std::fs::read(kitchen.join("a.png")).expect("kept"), a);
        assert_eq!(std::fs::read(kitchen.join("b.png")).expect("kept"), b);
        // **A folder's name is taken the same way.**
        assert_eq!(
            a_rename_is_made(&point.sessions, &record, &marked, &path("a.png"), b"tex", &[]).await,
            WhatTheActCameTo::TheNameIsTaken
        );
        assert!(kitchen.join("tex").is_dir());
        assert_eq!(std::fs::read(kitchen.join("a.png")).expect("kept"), a);
        let refused_units = marked.0.lock().expect("the list").clone();
        assert!(
            refused_units.iter().all(|(_, _, outcome)| *outcome == UnitOutcome::Failed),
            "a refused rename was told to the check record as done: {refused_units:?}"
        );

        // **A free name goes through**, whatever the page's listing had hinted.
        assert_eq!(
            a_rename_is_made(&point.sessions, &record, &marked, &path("a.png"), b"c.png", &[]).await,
            WhatTheActCameTo::Done
        );
        assert!(!kitchen.join("a.png").exists());
        assert_eq!(std::fs::read(kitchen.join("c.png")).expect("renamed"), a);
        assert_eq!(
            marked.0.lock().expect("the list").last().cloned(),
            Some((path("a.png"), WhatTheUnitDid::MovedAwayFromIt, UnitOutcome::Done)),
            "`File Browser` W11's mark was not asked for at the place the rename left"
        );
        // A folder renamed.
        assert_eq!(
            a_rename_is_made(&point.sessions, &record, &marked, &path("tex"), b"textures", &[]).await,
            WhatTheActCameTo::Done
        );
        assert!(kitchen.join("textures").is_dir());

        // **Something no longer there** is said to be so.
        assert_eq!(
            a_rename_is_made(&point.sessions, &record, &marked, &path("a.png"), b"d.png", &[]).await,
            WhatTheActCameTo::ItIsNotThere
        );
        // **A name with a slash would move the file**, and is refused before
        // anything is sent.
        assert!(matches!(
            a_rename_is_made(&point.sessions, &record, &marked, &path("c.png"), b"tex/c.png", &[]).await,
            WhatTheActCameTo::TheNameIsRefused(_)
        ));
        assert_eq!(std::fs::read(kitchen.join("c.png")).expect("kept"), a);
    }

    /// **`File Browser` W8 and `File Browser` W9 for a rename, onto the real record**: a
    /// rename of a waiting job's `.blend` past the live-job warning writes the
    /// job's mark first, reading *may have been renamed* — finished as
    /// *renamed* once the machine took it, and **removed where the machine
    /// refused** (`File Browser` W9: a job none of whose members' units took
    /// effect carries no mark). **A mark that cannot be written is the rename
    /// not attempted**, and the file stays where it was. Step 10c's Files' owed
    /// acts piece 3.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_rename_of_a_live_jobs_file_tells_the_job_first_and_finishes_its_mark() {
        use smoozy_desktop::managing_files::{a_rename_is_made, WhatTheActCameTo};
        use smoozy_file_browser::warnings::{AJobNamed, HowTheJobStands};
        use smoozy_queue_history::record::DisturbanceReading;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        for case in ["renamed", "refused", "telling-refused"] {
            let scratch = Scratch::new(&format!("rename-tell-{case}"));
            let kitchen = scratch.folder("kitchen");
            std::fs::write(kitchen.join("scene.blend"), b"a scene").expect("written");
            std::fs::write(kitchen.join("taken.blend"), b"another").expect("written");
            let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
            let job = Identity::issue(IdentityKind::Job).expect("randomness");
            let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
            record.with_the_record(&mut |connection| {
                smoozy_desktop::record::job_records::JobRecords::over(connection)
                    .write_the_provisional_record(&a_waiting_job(&job, &server, &kitchen.join("scene.blend")))
                    .expect("the job's record");
            });
            let blend = the_bytes_of(&kitchen.join("scene.blend"));
            let named = vec![AJobNamed {
                job: job.clone(),
                stands: HowTheJobStands::Waiting,
                the_members_reached: vec![blend.clone()],
                the_set_was_the_widest_case: true,
            }];
            let machine = a_machine(program.clone(), &scratch.at, 1);
            let point = APointLending::over(&machine).await;
            let marked = WhatWasMarked::default();
            if case == "telling-refused" {
                record.with_the_record(&mut |connection| {
                    connection.execute_batch("DROP TABLE job_record_disturbance").expect("the marks' table goes");
                });
                assert_eq!(
                    a_rename_is_made(&point.sessions, &record, &marked, &blend, b"kitchen_v2.blend", &named).await,
                    WhatTheActCameTo::TheTellingWouldNotWrite
                );
                assert_eq!(std::fs::read(kitchen.join("scene.blend")).expect("not renamed"), b"a scene");
                assert!(!kitchen.join("kitchen_v2.blend").exists());
                assert!(marked.0.lock().expect("the list").is_empty(), "a rename not attempted was marked");
                continue;
            }
            let to: &[u8] = if case == "renamed" { b"kitchen_v2.blend" } else { b"taken.blend" };
            let came_to = a_rename_is_made(&point.sessions, &record, &marked, &blend, to, &named).await;
            let marks = the_marks_of(&record, &job);
            if case == "renamed" {
                assert_eq!(came_to, WhatTheActCameTo::Done);
                assert_eq!(marks.len(), 1, "{marks:?}");
                assert_eq!(marks[0].reading, DisturbanceReading::Disturbed);
                assert_eq!(marks[0].the_act_intended, "rename");
                assert_eq!(marks[0].the_member.as_bytes(), blend.as_slice());
            } else {
                assert_eq!(came_to, WhatTheActCameTo::TheNameIsTaken);
                assert!(marks.is_empty(), "a mark stands over a file the machine never renamed: {marks:?}");
                assert_eq!(std::fs::read(kitchen.join("taken.blend")).expect("kept"), b"another");
            }
        }
    }

    // ── Step 10c's Files' owed acts piece 4: Delete ─────────────────────────

    /// **What a delete is asked for, on a throwaway server row.**
    fn a_delete_of(victims: Vec<Vec<u8>>, installation: Option<Identity>) -> smoozy_desktop::managing_files::ADeleteAsked {
        smoozy_desktop::managing_files::ADeleteAsked {
            victims,
            server_row: Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            installation,
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
        }
    }

    /// **`File Browser` M2 against OpenSSH's own `sftp-server`, through the
    /// server's own point: a folder goes whole, bottom-up, its hidden names
    /// with it, and a link inside it is removed as itself — what it points at
    /// is untouched** (`File Browser` §7's head). The act's record holds every
    /// entry, deepest first, each *done*; and `File Browser` W11's mark is
    /// asked for every entry that went. Step 10c's Files' owed acts piece 4.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_folder_is_deleted_bottom_up_with_its_hidden_names_and_a_link_goes_as_itself() {
        use smoozy_desktop::managing_files::{a_delete_is_made, WhatTheDeleteCameTo, THE_REMOVALS_IN_FLIGHT};
        use smoozy_file_browser::managing::WhatTheUnitDid;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("delete-tree");
        let kitchen = scratch.folder("kitchen");
        let outside = scratch.folder("elsewhere");
        std::fs::write(outside.join("keep.png"), b"keep me").expect("written");
        let cache = kitchen.join("cache");
        std::fs::create_dir_all(cache.join("sub").join("deeper")).expect("made");
        std::fs::write(cache.join("a.bphys"), b"a").expect("written");
        std::fs::write(cache.join(".hidden"), b"h").expect("written");
        std::fs::write(cache.join("sub").join("b.bphys"), b"b").expect("written");
        std::fs::write(cache.join("sub").join("deeper").join(".c"), b"c").expect("written");
        std::os::unix::fs::symlink(outside.join("keep.png"), cache.join("sub").join("link.png")).expect("linked");
        std::os::unix::fs::symlink(&outside, cache.join("folder-link")).expect("linked");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        let marked = WhatWasMarked::default();
        let came_to = a_delete_is_made(
            &point.sessions,
            &record,
            &marked,
            &a_delete_of(vec![the_bytes_of(&cache)], None),
            &[],
            THE_REMOVALS_IN_FLIGHT,
            &|_, _| {},
        )
        .await;
        let WhatTheDeleteCameTo::Ran { record: id, done, refused, not_attempted, never_learned } = came_to else {
            panic!("the delete did not run: {came_to:?}");
        };
        // The folder, a sub-folder and a deeper one, a file and a hidden
        // one at each of the three, but for the top's second, and two links:
        // nine entries.
        assert_eq!((done, refused, not_attempted, never_learned), (9, 0, 0, 0));
        assert!(!cache.exists(), "the folder is still there");
        assert_eq!(std::fs::read(outside.join("keep.png")).expect("the link's target"), b"keep me");
        assert!(outside.is_dir(), "a link to a folder was followed");
        let mut read = None;
        record.with_the_record(&mut |connection| {
            read = smoozy_desktop::record::file_acts::FileActs::over(connection).read(&id).expect("reads");
        });
        let read = read.expect("the act's record");
        assert_eq!(read.units.len(), 9);
        assert!(read.units.iter().all(|(_, outcome)| *outcome == Some(UnitOutcome::Done)));
        let depths: Vec<usize> = read.units.iter().map(|(unit, _)| unit.source.split(|b| *b == b'/').count()).collect();
        assert!(depths.windows(2).all(|pair| pair[0] >= pair[1]), "not deepest first: {depths:?}");
        assert_eq!(read.units.last().expect("the folder").0.source, the_bytes_of(&cache));
        let marks = marked.0.lock().expect("the list").clone();
        assert_eq!(marks.len(), 9);
        assert!(marks.iter().all(|(_, did, outcome)| *did == WhatTheUnitDid::RemovedIt && *outcome == UnitOutcome::Done));
    }

    /// **`File Browser` M2's partial: an entry the login may not remove is
    /// refused, everything else that can go goes, and a folder still holding
    /// something is not attempted** — the report says so per entry, the same
    /// thing the listing then shows. Step 10c's Files' owed acts piece 4.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_delete_the_login_may_not_finish_is_partial_and_says_which_went() {
        use smoozy_desktop::managing_files::{a_delete_is_made, WhatTheDeleteCameTo, THE_REMOVALS_IN_FLIGHT};
        use std::os::unix::fs::PermissionsExt;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("delete-partial");
        let cache = scratch.folder("kitchen").join("cache");
        std::fs::create_dir_all(cache.join("locked")).expect("made");
        std::fs::write(cache.join("free.txt"), b"f").expect("written");
        std::fs::write(cache.join("locked").join("held.txt"), b"h").expect("written");
        std::fs::set_permissions(cache.join("locked"), std::fs::Permissions::from_mode(0o555)).expect("locked");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        let came_to = a_delete_is_made(
            &point.sessions,
            &record,
            &smoozy_desktop::managing_files::NothingIsMarked,
            &a_delete_of(vec![the_bytes_of(&cache)], None),
            &[],
            THE_REMOVALS_IN_FLIGHT,
            &|_, _| {},
        )
        .await;
        std::fs::set_permissions(cache.join("locked"), std::fs::Permissions::from_mode(0o755)).expect("unlocked");
        let WhatTheDeleteCameTo::Ran { done, refused, not_attempted, .. } = came_to else {
            panic!("the delete did not run: {came_to:?}");
        };
        assert_eq!((done, refused, not_attempted), (1, 1, 2), "free.txt; held.txt; locked and cache");
        assert!(!cache.join("free.txt").exists());
        assert!(cache.join("locked").join("held.txt").exists());
    }

    /// **`File Browser` X12 as ruled on 2026-10-02: a delete over a part-sent
    /// upload writes *stopped: staging removed by user* first, then removes
    /// the staging it named, and its owed clean-up is cleared** — while an
    /// entry in the staging folder that no record names is an unowned staging
    /// entry, left by a delete of a folder above it and removed only when it
    /// is the victim itself. Step 10c's Files' owed acts piece 4.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_delete_over_a_part_sent_upload_stops_its_record_first_and_leaves_an_unowned_entry() {
        use smoozy_desktop::managing_files::{a_delete_is_made, WhatTheDeleteCameTo, THE_REMOVALS_IN_FLIGHT};
        use smoozy_server_protocol::transfers::{Direction, Ending, StoppedReason, TransferRecord};
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("delete-staging");
        let textures = scratch.folder("kitchen").join("textures");
        let staging = textures.join(".smoozy-staging");
        std::fs::create_dir_all(&staging).expect("made");
        std::fs::write(textures.join("wood.png"), b"w").expect("written");
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        let mut part_sent = TransferRecord::arranged(
            Identity::issue(IdentityKind::Transfer).expect("randomness"),
            Direction::Up,
            Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            Some(installation.clone()),
            "/Users/a/big.blend".to_string(),
            textures.join("big.blend").to_str().expect("text").to_string(),
            staging.to_str().expect("text"),
            100,
            Identity::issue(IdentityKind::Run).expect("randomness"),
        );
        part_sent.ending = Some(Ending::Interrupted);
        let owned = part_sent.staging_path.as_bytes();
        std::fs::write(std::str::from_utf8(&owned).expect("text"), b"half").expect("staged");
        std::fs::write(staging.join("nobody-knows.smoozy-part"), b"orphan").expect("staged");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        record.with_the_record(&mut |connection| {
            smoozy_desktop::record::transfer_records::put(connection, &part_sent).expect("the record");
        });
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        let came_to = a_delete_is_made(
            &point.sessions,
            &record,
            &smoozy_desktop::managing_files::NothingIsMarked,
            &a_delete_of(vec![the_bytes_of(&textures)], Some(installation.clone())),
            &[],
            THE_REMOVALS_IN_FLIGHT,
            &|_, _| {},
        )
        .await;
        let WhatTheDeleteCameTo::Ran { done, refused, not_attempted, .. } = came_to else {
            panic!("the delete did not run: {came_to:?}");
        };
        // wood.png and the owned part went; the orphan, the staging folder and
        // textures were not attempted.
        assert_eq!((done, refused, not_attempted), (2, 0, 3));
        assert!(!std::path::Path::new(std::str::from_utf8(&owned).unwrap()).exists());
        assert!(staging.join("nobody-knows.smoozy-part").exists(), "an unowned staging entry was removed");
        let mut after = Vec::new();
        record.with_the_record(&mut |connection| {
            after = smoozy_desktop::record::transfer_records::all(connection).expect("reads");
        });
        assert_eq!(after.len(), 1);
        assert_eq!(after[0].ending, Some(Ending::Stopped(StoppedReason::StagingRemovedByUser)));
        assert!(!after[0].cleanup_is_owed, "the clean-up stayed owed after the staging went");

        // **The orphan as the victim itself is the explicit removal.**
        let orphan = the_bytes_of(&staging.join("nobody-knows.smoozy-part"));
        let came_to = a_delete_is_made(
            &point.sessions,
            &record,
            &smoozy_desktop::managing_files::NothingIsMarked,
            &a_delete_of(vec![orphan], Some(installation)),
            &[],
            THE_REMOVALS_IN_FLIGHT,
            &|_, _| {},
        )
        .await;
        assert!(matches!(came_to, WhatTheDeleteCameTo::Ran { done: 1, .. }), "{came_to:?}");
        assert!(!staging.join("nobody-knows.smoozy-part").exists());
    }

    /// **`File Browser` W8 and `File Browser` W9 for a delete, onto the real
    /// record**: a
    /// delete of a folder holding a waiting job's `.blend`, pushed past the
    /// live-job warning, tells the job in the act's own record and marks it
    /// first, and the mark reads *deleted* once the file went. **A telling
    /// that cannot be written is the delete not attempted.** Step 10c's Files'
    /// owed acts piece 4.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_delete_of_a_live_jobs_folder_tells_the_job_first_and_finishes_its_mark() {
        use smoozy_desktop::managing_files::{a_delete_is_made, WhatTheActCameTo, WhatTheDeleteCameTo, THE_REMOVALS_IN_FLIGHT};
        use smoozy_file_browser::warnings::{AJobNamed, HowTheJobStands};
        use smoozy_queue_history::record::DisturbanceReading;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        for case in ["deleted", "telling-refused"] {
            let scratch = Scratch::new(&format!("delete-tell-{case}"));
            let kitchen = scratch.folder("kitchen");
            std::fs::write(kitchen.join("scene.blend"), b"a scene").expect("written");
            let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
            let job = Identity::issue(IdentityKind::Job).expect("randomness");
            let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
            record.with_the_record(&mut |connection| {
                smoozy_desktop::record::job_records::JobRecords::over(connection)
                    .write_the_provisional_record(&a_waiting_job(&job, &server, &kitchen.join("scene.blend")))
                    .expect("the job's record");
            });
            let blend = the_bytes_of(&kitchen.join("scene.blend"));
            let named = vec![AJobNamed {
                job: job.clone(),
                stands: HowTheJobStands::Waiting,
                the_members_reached: vec![blend.clone()],
                the_set_was_the_widest_case: true,
            }];
            if case == "telling-refused" {
                record.with_the_record(&mut |connection| {
                    connection.execute_batch("DROP TABLE job_record_disturbance").expect("the marks' table goes");
                });
            }
            let machine = a_machine(program.clone(), &scratch.at, 1);
            let point = APointLending::over(&machine).await;
            let came_to = a_delete_is_made(
                &point.sessions,
                &record,
                &smoozy_desktop::managing_files::NothingIsMarked,
                &a_delete_of(vec![the_bytes_of(&kitchen)], None),
                &named,
                THE_REMOVALS_IN_FLIGHT,
                &|_, _| {},
            )
            .await;
            if case == "telling-refused" {
                assert_eq!(came_to, WhatTheDeleteCameTo::NothingWasRemoved(WhatTheActCameTo::TheTellingWouldNotWrite));
                assert!(kitchen.join("scene.blend").exists(), "a delete went on with no telling");
                continue;
            }
            let WhatTheDeleteCameTo::Ran { record: id, done: 2, .. } = came_to else {
                panic!("the delete did not run whole: {came_to:?}");
            };
            let marks = the_marks_of(&record, &job);
            assert_eq!(marks.len(), 1, "{marks:?}");
            assert_eq!(marks[0].reading, DisturbanceReading::Disturbed);
            assert_eq!(marks[0].the_act_intended, "delete");
            let mut told = Vec::new();
            record.with_the_record(&mut |connection| {
                told = smoozy_desktop::record::file_acts::FileActs::over(connection)
                    .read(&id)
                    .expect("reads")
                    .expect("held")
                    .the_jobs_told;
            });
            assert_eq!(told, vec![job]);
        }
    }

    /// **Piece 4's figure: a folder of 10,000 small files deleted through the
    /// server's own point, one removal at a time and then pipelined** — what
    /// the map asked to be brought back before any rental. Over a Unix socket
    /// pair the round trip is microseconds, so the gap here is the floor of
    /// what pipelining buys; on a rental a whole round trip is paid per
    /// removal that waits. Run by hand:
    /// `cargo test -p smoozy-desktop --test through_the_servers_own_point -- --ignored --nocapture ten_thousand`.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore = "a measurement, run by hand"]
    async fn ten_thousand_small_files_are_deleted_one_at_a_time_and_pipelined() {
        use smoozy_desktop::managing_files::{a_delete_is_made, WhatTheDeleteCameTo, THE_REMOVALS_IN_FLIGHT};
        let program = the_server_or_say_so!();
        for in_flight in [1, THE_REMOVALS_IN_FLIGHT] {
            let scratch = Scratch::new(&format!("delete-10000-{in_flight}"));
            let cache = scratch.folder("cache");
            for n in 0..10_000 {
                std::fs::write(cache.join(format!("f{n:05}.bphys")), b"x").expect("written");
            }
            let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
            let machine = a_machine(program.clone(), &scratch.at, 1);
            let point = APointLending::over(&machine).await;
            let started = std::time::Instant::now();
            let came_to = a_delete_is_made(
                &point.sessions,
                &record,
                &smoozy_desktop::managing_files::NothingIsMarked,
                &a_delete_of(vec![the_bytes_of(&cache)], None),
                &[],
                in_flight,
                &|_, _| {},
            )
            .await;
            let took = started.elapsed();
            assert!(matches!(came_to, WhatTheDeleteCameTo::Ran { done: 10_001, .. }), "{came_to:?}");
            println!("FIGURE: 10,000 files and their folder, {in_flight} in flight: {took:?}");
        }
    }

    // ── Step 10c's Files' owed acts piece 5: Cut, and Paste after a cut ─────

    /// **A pasteboard holding a cut**, on a throwaway server row.
    fn a_cut_of(entries: Vec<Vec<u8>>) -> smoozy_file_browser::managing::Pasteboard {
        smoozy_file_browser::managing::Pasteboard {
            server_row: Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            installation: None,
            act: smoozy_file_browser::managing::CutOrCopy::Cut,
            entries,
        }
    }

    /// **`File Browser` M5's merge and `File Browser` M8's question against OpenSSH's own
    /// `sftp-server`, through the server's own point**: a cut folder pasted
    /// onto a same-named folder goes down a level, **only the clashing file is
    /// asked about**, *Replace* replaces only that file, the rest move by their
    /// own renames, and the emptied source folders are removed. `File Browser`
    /// `File Browser` W11 and `File Browser` X7a are asked for the place each item left and where it
    /// landed. Step 10c's Files' owed acts piece 5.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_cut_folder_pasted_onto_a_same_named_one_merges_and_asks_only_about_the_clash() {
        use smoozy_desktop::managing_files::{a_move_is_made, WhatThePasteCameTo};
        use smoozy_file_browser::already_exists::{CollisionAnswer, Comparison};
        use smoozy_file_browser::managing::WhatTheUnitDid;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("paste-merge");
        let here = scratch.folder("here");
        let there = scratch.folder("there");
        let from = here.join("textures");
        std::fs::create_dir_all(from.join("sub")).expect("made");
        std::fs::write(from.join("a.png"), b"new a").expect("written");
        std::fs::write(from.join("b.png"), b"new b").expect("written");
        std::fs::write(from.join("sub").join("c.png"), b"new c").expect("written");
        let onto = there.join("textures");
        std::fs::create_dir_all(onto.join("sub")).expect("made");
        std::fs::write(onto.join("b.png"), b"old b").expect("written");
        std::fs::write(onto.join("keep.png"), b"kept").expect("written");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        let marked = WhatWasMarked::default();
        let cut = a_cut_of(vec![the_bytes_of(&from)]);
        let run = Identity::issue(IdentityKind::Run).expect("randomness");

        // **The question first, and only about b.png.**
        let asked = a_move_is_made(&point.sessions, &record, &marked, &cut, &the_bytes_of(&there), None, &[], &run, None).await;
        let WhatThePasteCameTo::AsksFirst(comparisons) = asked else {
            panic!("no question was asked: {asked:?}");
        };
        let clashing: Vec<&[u8]> = comparisons
            .iter()
            .filter(|one| one.what == Comparison::CollidesWithAFile)
            .map(|one| one.landing_path.as_slice())
            .collect();
        assert_eq!(clashing, vec![the_bytes_of(&onto.join("b.png")).as_slice()]);
        assert_eq!(std::fs::read(onto.join("b.png")).expect("untouched"), b"old b", "the question moved something");

        // **Replace.**
        let came_to = a_move_is_made(
            &point.sessions,
            &record,
            &marked,
            &cut,
            &the_bytes_of(&there),
            Some(CollisionAnswer::Replace),
            &[],
            &run,
            None,
        )
        .await;
        let WhatThePasteCameTo::Ran { done: 3, refused: 0, not_done: 0, .. } = came_to else {
            panic!("the paste did not move all three: {came_to:?}");
        };
        assert_eq!(std::fs::read(onto.join("a.png")).expect("moved"), b"new a");
        assert_eq!(std::fs::read(onto.join("b.png")).expect("replaced"), b"new b");
        assert_eq!(std::fs::read(onto.join("sub").join("c.png")).expect("moved"), b"new c");
        assert_eq!(std::fs::read(onto.join("keep.png")).expect("kept"), b"kept");
        assert!(!from.exists(), "the emptied source folder is still there");
        let marks = marked.0.lock().expect("the list").clone();
        assert!(marks.contains(&(the_bytes_of(&from.join("a.png")), WhatTheUnitDid::MovedAwayFromIt, UnitOutcome::Done)));
        assert!(marks.contains(&(the_bytes_of(&onto.join("a.png")), WhatTheUnitDid::PublishedOverIt, UnitOutcome::Done)));

        // **`File Browser` M14: the pasteboard is not cleared, and a cut pasted
        // twice finds its source gone** — said, and nothing else moves.
        let elsewhere = scratch.folder("elsewhere");
        let again = a_move_is_made(&point.sessions, &record, &marked, &cut, &the_bytes_of(&elsewhere), None, &[], &run, None).await;
        assert!(
            matches!(again, WhatThePasteCameTo::Ran { done: 0, not_done: 1, .. }),
            "a second paste of a cut whose source is gone: {again:?}"
        );
        assert_eq!(std::fs::read_dir(&elsewhere).expect("reads").count(), 0);

        // **Pasted back into the folder it was cut from, it is not there
        // either**, and that is said the same way — never *already in this
        // folder*. The pass before 0.1.9 on Vast, new fault 4.
        let back = a_move_is_made(&point.sessions, &record, &marked, &cut, &the_bytes_of(&here), None, &[], &run, None).await;
        assert!(
            matches!(back, WhatThePasteCameTo::Ran { done: 0, not_done: 1, .. }),
            "a cut whose source is gone, pasted back where it came from: {back:?}"
        );
    }

    /// **The refusals before anything is sent, and Replace where the kinds
    /// differ** — `Design/File Browser § What the screen says around an act`
    /// and `File Browser` M8's paragraph of 2026-10-02: a folder into itself
    /// or a folder inside it is refused; a cut pasted back where it came from
    /// moves nothing; and a file onto a same-named folder is not replaced
    /// even under *Replace*. Step 10c's Files' owed acts piece 5.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_paste_into_itself_is_refused_and_replace_never_swaps_a_file_for_a_folder() {
        use smoozy_desktop::managing_files::{a_move_is_made, WhatThePasteCameTo, WhyThePasteIsRefused};
        use smoozy_file_browser::already_exists::CollisionAnswer;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("paste-refusals");
        let here = scratch.folder("here");
        std::fs::create_dir_all(here.join("kitchen").join("inner")).expect("made");
        std::fs::write(here.join("wood.png"), b"a file").expect("written");
        let there = scratch.folder("there");
        std::fs::create_dir_all(there.join("wood.png")).expect("a folder of that name");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        let marked = WhatWasMarked::default();
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let kitchen = a_cut_of(vec![the_bytes_of(&here.join("kitchen"))]);
        for into in [here.join("kitchen"), here.join("kitchen").join("inner")] {
            assert_eq!(
                a_move_is_made(&point.sessions, &record, &marked, &kitchen, &the_bytes_of(&into), None, &[], &run, None).await,
                WhatThePasteCameTo::Refused(WhyThePasteIsRefused::AFolderIntoItself)
            );
        }
        assert_eq!(
            a_move_is_made(&point.sessions, &record, &marked, &kitchen, &the_bytes_of(&here), None, &[], &run, None).await,
            WhatThePasteCameTo::NothingToMove
        );
        let wood = a_cut_of(vec![the_bytes_of(&here.join("wood.png"))]);
        let came_to = a_move_is_made(
            &point.sessions,
            &record,
            &marked,
            &wood,
            &the_bytes_of(&there),
            Some(CollisionAnswer::Replace),
            &[],
            &run,
            None,
        )
        .await;
        // **Not done because the kinds differ** — never tried, so never read
        // as a crossing.
        assert!(
            matches!(came_to, WhatThePasteCameTo::Ran { done: 0, not_done: 1, crossings: 0, .. }),
            "{came_to:?}"
        );
        assert!(there.join("wood.png").is_dir(), "a folder was replaced by a file");
        assert_eq!(std::fs::read(here.join("wood.png")).expect("left"), b"a file");
        assert!(marked.0.lock().expect("the list").is_empty(), "something was marked for a move that did not happen");
    }

    /// **`File Browser` W8 and `File Browser` W9 for a move**: a cut of a
    /// waiting job's `.blend` pasted elsewhere tells the job first, and its
    /// mark reads *moved*. Step 10c's Files' owed acts piece 5.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_move_of_a_live_jobs_file_tells_the_job_first_and_finishes_its_mark() {
        use smoozy_desktop::managing_files::{a_move_is_made, WhatThePasteCameTo};
        use smoozy_file_browser::warnings::{AJobNamed, HowTheJobStands};
        use smoozy_queue_history::record::DisturbanceReading;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("paste-tell");
        let kitchen = scratch.folder("kitchen");
        let archive = scratch.folder("archive");
        std::fs::write(kitchen.join("scene.blend"), b"a scene").expect("written");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        record.with_the_record(&mut |connection| {
            smoozy_desktop::record::job_records::JobRecords::over(connection)
                .write_the_provisional_record(&a_waiting_job(&job, &server, &kitchen.join("scene.blend")))
                .expect("the job's record");
        });
        let blend = the_bytes_of(&kitchen.join("scene.blend"));
        let named = vec![AJobNamed {
            job: job.clone(),
            stands: HowTheJobStands::Waiting,
            the_members_reached: vec![blend.clone()],
            the_set_was_the_widest_case: true,
        }];
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        let came_to = a_move_is_made(
            &point.sessions,
            &record,
            &smoozy_desktop::managing_files::NothingIsMarked,
            &a_cut_of(vec![blend]),
            &the_bytes_of(&archive),
            None,
            &named,
            &Identity::issue(IdentityKind::Run).expect("randomness"),
            None,
        )
        .await;
        assert!(matches!(came_to, WhatThePasteCameTo::Ran { done: 1, .. }), "{came_to:?}");
        assert!(archive.join("scene.blend").exists());
        let marks = the_marks_of(&record, &job);
        assert_eq!(marks.len(), 1, "{marks:?}");
        assert_eq!(marks[0].reading, DisturbanceReading::Disturbed);
        assert_eq!(marks[0].the_act_intended, "move");
    }

    /// **`File Browser` M5's crossing and `File Browser` M6's pair, against a second
    /// filesystem attached at the desk** — a disk image, attached without
    /// administrator rights and detached at the end. A move whose rename the
    /// machine refuses with nothing at the destination name is a crossing; the
    /// file is copied onto the other disk and then removed where it was. **Run by hand on a Mac**:
    /// `cargo test -p smoozy-desktop --test through_the_servers_own_point -- --ignored --nocapture across_two_disks`.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore = "attaches a disk image, run by hand on a Mac"]
    async fn a_move_across_two_disks_is_read_as_a_crossing() {
        use smoozy_desktop::managing_files::{a_move_is_made, WhatThePasteCameTo};
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("paste-across");
        let here = scratch.folder("here");
        let mount = scratch.folder("mount");
        std::fs::write(here.join("wood.png"), b"wood").expect("written");
        let image = scratch.at.join("disk.dmg");
        let made = std::process::Command::new("hdiutil")
            .args(["create", "-size", "8m", "-fs", "HFS+", "-volname", "smzy", "-ov"])
            .arg(&image)
            .output();
        if !made.is_ok_and(|made| made.status.success()) {
            println!("SKIPPED: no disk image could be made here");
            return;
        }
        let attached = std::process::Command::new("hdiutil")
            .args(["attach", "-nobrowse", "-mountpoint"])
            .arg(&mount)
            .arg(&image)
            .status()
            .expect("hdiutil runs");
        assert!(attached.success());
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        let came_to = a_move_is_made(
            &point.sessions,
            &record,
            &smoozy_desktop::managing_files::NothingIsMarked,
            &a_cut_of(vec![the_bytes_of(&here.join("wood.png"))]),
            &the_bytes_of(&mount),
            None,
            &[],
            &Identity::issue(IdentityKind::Run).expect("randomness"),
            None,
        )
        .await;
        // **`File Browser` M6's pair, from piece 6**: copied onto the other
        // disk under `File Browser` M7's rule, then the source removed.
        let landed = std::fs::read(mount.join("wood.png")).ok();
        let left = here.join("wood.png").exists();
        let _ = std::process::Command::new("hdiutil").arg("detach").arg(&mount).status();
        println!("FIGURE: a move across two disks came to {came_to:?}");
        assert!(matches!(came_to, WhatThePasteCameTo::Ran { done: 1, crossings: 0, .. }), "{came_to:?}");
        assert_eq!(landed.as_deref(), Some(&b"wood"[..]), "the copy did not land on the other disk");
        assert!(!left, "the source stayed after its copy landed");
    }

    // ── Step 10c's Files' owed acts piece 6: Copy, and Paste after a copy ───

    /// **A pasteboard holding a copy.**
    fn a_copy_of(entries: Vec<Vec<u8>>) -> smoozy_file_browser::managing::Pasteboard {
        smoozy_file_browser::managing::Pasteboard {
            act: smoozy_file_browser::managing::CutOrCopy::Copy,
            ..a_cut_of(entries)
        }
    }

    /// **`File Browser` M7, `File Browser` M8 and `File Browser` X9 against OpenSSH's own `sftp-server`,
    /// through the server's own point**: a folder copied into a folder holding
    /// one of the same names merges, the clash is asked about, and *Keep both*
    /// lands the copy beside it as `a (2).png` with the original untouched;
    /// the source is untouched too, and no staging is left. **A file pasted
    /// into its own folder asks the same question, and Keep both is the
    /// duplicate.** Step 10c's Files' owed acts piece 6.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_copy_with_keep_both_lands_beside_and_leaves_the_original_and_the_source() {
        use smoozy_desktop::managing_files::{a_copy_is_made, WhatTheCopyCameTo};
        use smoozy_file_browser::already_exists::CollisionAnswer;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("copy-keep-both");
        let here = scratch.folder("here");
        let there = scratch.folder("there");
        let from = here.join("textures");
        std::fs::create_dir_all(from.join("sub")).expect("made");
        std::fs::write(from.join("a.png"), a_file_of(70_000, 3)).expect("written");
        std::fs::write(from.join("sub").join("b.png"), b"bee").expect("written");
        std::os::unix::fs::symlink("a.png", from.join("link.png")).expect("linked");
        std::fs::create_dir_all(there.join("textures")).expect("made");
        std::fs::write(there.join("textures").join("a.png"), b"the original").expect("written");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine(program, &scratch.at, 2);
        let point = APointLending::over(&machine).await;
        let marked = WhatWasMarked::default();
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let copy = a_copy_of(vec![the_bytes_of(&from)]);
        let asked = a_copy_is_made(&point.sessions, &record, &marked, &copy, &the_bytes_of(&there), None, &[], &run, None, &|_, _| {}).await;
        let WhatTheCopyCameTo::AsksFirst(comparisons) = asked else {
            panic!("no question: {asked:?}");
        };
        assert_eq!(comparisons.len(), 1, "{comparisons:?}");
        let came_to = a_copy_is_made(
            &point.sessions,
            &record,
            &marked,
            &copy,
            &the_bytes_of(&there),
            Some(CollisionAnswer::KeepBoth),
            &[],
            &run,
            None,
            &|_, _| {},
        )
        .await;
        assert!(matches!(came_to, WhatTheCopyCameTo::Ran { done: 3, refused: 0, not_done: 0, never_learned: 0, .. }), "{came_to:?}");
        let onto = there.join("textures");
        assert_eq!(std::fs::read(onto.join("a.png")).expect("kept"), b"the original");
        assert_eq!(std::fs::read(onto.join("a (2).png")).expect("beside"), a_file_of(70_000, 3));
        assert_eq!(std::fs::read(onto.join("sub").join("b.png")).expect("copied"), b"bee");
        assert_eq!(std::fs::read_link(onto.join("link.png")).expect("a link"), std::path::PathBuf::from("a.png"));
        assert!(!onto.join(".smoozy-staging").exists() && !onto.join("sub").join(".smoozy-staging").exists(), "staging was left");
        assert_eq!(std::fs::read(from.join("a.png")).expect("the source"), a_file_of(70_000, 3));
        assert_eq!(marked.0.lock().expect("the list").len(), 3, "`File Browser` X7a and `File Browser` W11 at each landing");

        // **Pasted into its own folder: the duplicate.**
        let wood = a_copy_of(vec![the_bytes_of(&from.join("sub").join("b.png"))]);
        let into = the_bytes_of(&from.join("sub"));
        assert!(matches!(
            a_copy_is_made(&point.sessions, &record, &marked, &wood, &into, None, &[], &run, None, &|_, _| {}).await,
            WhatTheCopyCameTo::AsksFirst(_)
        ));
        let came_to = a_copy_is_made(
            &point.sessions,
            &record,
            &marked,
            &wood,
            &into,
            Some(CollisionAnswer::KeepBoth),
            &[],
            &run,
            None,
            &|_, _| {},
        )
        .await;
        assert!(matches!(came_to, WhatTheCopyCameTo::Ran { done: 1, .. }), "{came_to:?}");
        assert_eq!(std::fs::read(from.join("sub").join("b (2).png")).expect("the duplicate"), b"bee");
    }

    /// **`File Browser` M7: a copy never writes at a final name**, so a copy
    /// that cannot finish leaves nothing there — here the destination folder
    /// will not take Smoozy's staging folder, and the unit is refused with
    /// nothing at the final name and no staging behind it. Step 10c's Files'
    /// owed acts piece 6.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_copy_that_cannot_finish_leaves_nothing_at_the_final_name() {
        use smoozy_desktop::managing_files::{a_copy_is_made, WhatTheCopyCameTo};
        use std::os::unix::fs::PermissionsExt;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("copy-refused");
        let here = scratch.folder("here");
        let shut = scratch.folder("shut");
        std::fs::write(here.join("big.blend"), a_file_of(10_000, 5)).expect("written");
        std::fs::set_permissions(&shut, std::fs::Permissions::from_mode(0o555)).expect("shut");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine(program, &scratch.at, 2);
        let point = APointLending::over(&machine).await;
        let came_to = a_copy_is_made(
            &point.sessions,
            &record,
            &smoozy_desktop::managing_files::NothingIsMarked,
            &a_copy_of(vec![the_bytes_of(&here.join("big.blend"))]),
            &the_bytes_of(&shut),
            None,
            &[],
            &Identity::issue(IdentityKind::Run).expect("randomness"),
            None,
            &|_, _| {},
        )
        .await;
        let anything = std::fs::read_dir(&shut).expect("reads").count();
        std::fs::set_permissions(&shut, std::fs::Permissions::from_mode(0o755)).expect("opened");
        assert!(matches!(came_to, WhatTheCopyCameTo::Ran { done: 0, refused: 1, .. }), "{came_to:?}");
        assert_eq!(anything, 0, "something was left in the destination");
    }

    /// **`File Browser` M7's paragraph of 2026-10-02: where copy-data is not
    /// announced, a copy is not attempted and the user is told why** — the
    /// server here is OpenSSH's own `sftp-server` with the request banned
    /// (`-P copy-data`), which takes the extension out of its announcement —
    /// the shape of an 8.9p1 server. Nothing is attempted and nothing lands.
    /// Step 10c's Files' owed acts piece 6.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_server_that_will_not_copy_lands_nothing() {
        use smoozy_desktop::managing_files::{a_copy_is_made, WhatTheCopyCameTo};
        use std::os::unix::fs::PermissionsExt;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("copy-banned");
        let here = scratch.folder("here");
        let there = scratch.folder("there");
        std::fs::write(here.join("big.blend"), b"bytes").expect("written");
        let banned = scratch.at.join("sftp-server-without-copy");
        std::fs::write(&banned, format!("#!/bin/sh\nexec '{}' -P copy-data\n", program.display())).expect("written");
        std::fs::set_permissions(&banned, std::fs::Permissions::from_mode(0o755)).expect("runnable");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine(banned, &scratch.at, 2);
        let point = APointLending::over(&machine).await;
        let came_to = a_copy_is_made(
            &point.sessions,
            &record,
            &smoozy_desktop::managing_files::NothingIsMarked,
            &a_copy_of(vec![the_bytes_of(&here.join("big.blend"))]),
            &the_bytes_of(&there),
            None,
            &[],
            &Identity::issue(IdentityKind::Run).expect("randomness"),
            None,
            &|_, _| {},
        )
        .await;
        println!("FIGURE: a copy against a server with copy-data banned came to {came_to:?}");
        // **Seen 2026-10-02: banning the request takes it out of the
        // announcement**, so nothing is attempted.
        assert_eq!(
            came_to,
            WhatTheCopyCameTo::TheServerCannotCopy(smoozy_desktop::managing_files::WhyTheServerCannotCopy::SmoozyIsNotSetUpThere)
        );
        assert!(!there.join("big.blend").exists());
    }

    /// **OpenSSH's own `sftp-server` with copy-data banned, the shape of an
    /// 8.9p1 server**, with Smoozy's own program placed at `area` and asked
    /// the way `who` says.
    fn a_machine_that_will_not_copy(
        program: &Path,
        scratch: &Scratch,
        area: &Path,
        who: WhoAnswers,
    ) -> Arc<AMachine> {
        use std::os::unix::fs::PermissionsExt;
        let banned = scratch.at.join("sftp-server-without-copy");
        std::fs::write(&banned, format!("#!/bin/sh\nexec '{}' -P copy-data\n", program.display())).expect("written");
        std::fs::set_permissions(&banned, std::fs::Permissions::from_mode(0o755)).expect("runnable");
        Arc::new(AMachine {
            program: banned,
            home: scratch.at.clone(),
            limit: 2,
            supervisor: Some(TheSupervisor {
                installation: Identity::issue(IdentityKind::Installation).expect("randomness"),
                package_area: area.to_path_buf(),
                answered_by: who,
            }),
            log: Mutex::new(Vec::new()),
            additional_sessions_opened: AtomicUsize::new(0),
        })
    }

    /// **`File Browser` M7's framed request, end to end: where the server does
    /// not announce copy-data, Smoozy's own program copies on the machine** —
    /// the real adapter this workspace built, answering
    /// `Server protocol and transfers` §1.1's *a file is copied on the machine*
    /// in its own invocation with no supervisor running at all, and the desktop
    /// publishing each staged copy by its own rename. A folder holding a file,
    /// a file in a sub-folder and a link lands whole; the link is still a link;
    /// no staging is left; `File Browser` X7a and `File Browser` W11 were asked
    /// at each landing; and a *Replace* over a file already there replaces it.
    /// Step 10c's Files' owed acts piece 7.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn where_the_server_will_not_copy_smoozys_own_program_copies_on_the_machine() {
        use smoozy_desktop::managing_files::{a_copy_is_made, WhatTheCopyCameTo};
        use smoozy_file_browser::already_exists::CollisionAnswer;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let Some(built) = the_program_this_workspace_built() else {
            println!("SKIPPED: the machine's own program is not built beside this test; build `smoozy-server` first.");
            return;
        };
        let scratch = Scratch::new("copy-by-the-program");
        let area = a_package_area_with_the_real_program(&scratch, &built);
        let here = scratch.folder("here");
        let there = scratch.folder("there");
        let from = here.join("textures");
        std::fs::create_dir_all(from.join("sub")).expect("made");
        std::fs::write(from.join("a.png"), a_file_of(300_000, 9)).expect("written");
        std::fs::write(from.join("sub").join("b.png"), b"bee").expect("written");
        std::os::unix::fs::symlink("a.png", from.join("link.png")).expect("linked");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine_that_will_not_copy(&program, &scratch, &area, WhoAnswers::TheRealProgram);
        let point = APointLending::over(&machine).await;
        let marked = WhatWasMarked::default();
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let area_text = area.display().to_string();
        let came_to = a_copy_is_made(
            &point.sessions,
            &record,
            &marked,
            &a_copy_of(vec![the_bytes_of(&from)]),
            &the_bytes_of(&there),
            None,
            &[],
            &run,
            Some(&area_text),
            &|_, _| {},
        )
        .await;
        let log = machine.what_happened();
        assert!(
            matches!(came_to, WhatTheCopyCameTo::Ran { done: 3, refused: 0, not_done: 0, never_learned: 0, .. }),
            "{came_to:?}\n{log:#?}"
        );
        let onto = there.join("textures");
        assert_eq!(std::fs::read(onto.join("a.png")).expect("copied"), a_file_of(300_000, 9));
        assert_eq!(std::fs::read(onto.join("sub").join("b.png")).expect("copied"), b"bee");
        assert_eq!(std::fs::read_link(onto.join("link.png")).expect("a link"), PathBuf::from("a.png"));
        assert!(!onto.join(".smoozy-staging").exists() && !onto.join("sub").join(".smoozy-staging").exists(), "staging was left");
        assert_eq!(std::fs::read(from.join("a.png")).expect("the source"), a_file_of(300_000, 9));
        assert_eq!(marked.0.lock().expect("the list").len(), 3, "`File Browser` X7a and `File Browser` W11 at each landing");
        let asked = log
            .iter()
            .filter(|line| line.contains("the entry point was asked `a file is copied on the machine`"))
            .count();
        assert_eq!(asked, 2, "the program was asked once per file, and never for the link: {log:#?}");

        // **Replace over a file already there.**
        std::fs::write(from.join("a.png"), b"newer").expect("written");
        let one = a_copy_of(vec![the_bytes_of(&from.join("a.png"))]);
        let asked = a_copy_is_made(&point.sessions, &record, &marked, &one, &the_bytes_of(&onto), None, &[], &run, Some(&area_text), &|_, _| {}).await;
        assert!(matches!(asked, WhatTheCopyCameTo::AsksFirst(_)), "{asked:?}");
        let came_to = a_copy_is_made(
            &point.sessions,
            &record,
            &marked,
            &one,
            &the_bytes_of(&onto),
            Some(CollisionAnswer::Replace),
            &[],
            &run,
            Some(&area_text),
            &|_, _| {},
        )
        .await;
        assert!(matches!(came_to, WhatTheCopyCameTo::Ran { done: 1, .. }), "{came_to:?}");
        assert_eq!(std::fs::read(onto.join("a.png")).expect("replaced"), b"newer");
        assert!(!onto.join(".smoozy-staging").exists(), "staging was left");
    }

    /// **Design's *Update* where Smoozy on the server is older than the copy
    /// request**: the program there refuses it by name, so nothing is copied,
    /// nothing lands and no staging is left — and the answer says which
    /// absence it was. Step 10c's Files' owed acts piece 7.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn where_smoozy_on_the_server_is_older_the_copy_says_so_and_lands_nothing() {
        use smoozy_desktop::managing_files::{a_copy_is_made, WhatTheCopyCameTo, WhyTheServerCannotCopy};
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("copy-by-an-older-program");
        let area = scratch.folder("package");
        let here = scratch.folder("here");
        let there = scratch.folder("there");
        std::fs::write(here.join("big.blend"), b"bytes").expect("written");
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine_that_will_not_copy(&program, &scratch, &area, WhoAnswers::AnOlderProgram);
        let point = APointLending::over(&machine).await;
        let came_to = a_copy_is_made(
            &point.sessions,
            &record,
            &smoozy_desktop::managing_files::NothingIsMarked,
            &a_copy_of(vec![the_bytes_of(&here.join("big.blend"))]),
            &the_bytes_of(&there),
            None,
            &[],
            &Identity::issue(IdentityKind::Run).expect("randomness"),
            Some(&area.display().to_string()),
            &|_, _| {},
        )
        .await;
        assert_eq!(came_to, WhatTheCopyCameTo::TheServerCannotCopy(WhyTheServerCannotCopy::SmoozyThereIsOlder));
        assert_eq!(std::fs::read_dir(&there).expect("reads").count(), 0, "something was left in the destination");
    }

    /// **Piece 6's figure: a large file copied on the machine, and whether the
    /// list stays live while it runs** — the bytes never cross the line, and
    /// the copy rides a transfer channel of its own, so a reading of the
    /// folder goes on beside it. The map asked for 4 GB; the desk had too
    /// little disk for two of them, so the figure is taken at 1 GB. Run by
    /// hand: `cargo test -p smoozy-desktop --test through_the_servers_own_point -- --ignored --nocapture a_large_file`.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "a measurement, run by hand"]
    async fn a_large_file_is_copied_on_the_machine_while_the_list_is_read() {
        use smoozy_desktop::managing_files::{a_copy_is_made, WhatTheCopyCameTo};
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("copy-large");
        let here = scratch.folder("here");
        let there = scratch.folder("there");
        {
            use std::io::Write;
            let mut file = std::fs::File::create(here.join("big.blend")).expect("made");
            let chunk = a_file_of(1 << 20, 7);
            for _ in 0..1024 {
                file.write_all(&chunk).expect("written");
            }
        }
        let record = ARecordHere(Mutex::new(LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens")));
        let machine = a_machine(program, &scratch.at, 10);
        let point = APointLending::over(&machine).await;
        let sessions = point.sessions.clone();
        let browsed = the_bytes_of(&here);
        let readings = tokio::spawn(async move {
            let mut slowest = std::time::Duration::ZERO;
            let mut taken = 0;
            for _ in 0..20 {
                let started = std::time::Instant::now();
                let _ = a_folder_is_read(&sessions, &browsed).await;
                slowest = slowest.max(started.elapsed());
                taken += 1;
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            (taken, slowest)
        });
        let started = std::time::Instant::now();
        let came_to = a_copy_is_made(
            &point.sessions,
            &record,
            &smoozy_desktop::managing_files::NothingIsMarked,
            &a_copy_of(vec![the_bytes_of(&here.join("big.blend"))]),
            &the_bytes_of(&there),
            None,
            &[],
            &Identity::issue(IdentityKind::Run).expect("randomness"),
            None,
            &|_, _| {},
        )
        .await;
        let took = started.elapsed();
        let (taken, slowest) = readings.await.expect("the readings");
        assert!(matches!(came_to, WhatTheCopyCameTo::Ran { done: 1, .. }), "{came_to:?}");
        println!("FIGURE: 1 GB copied on the machine in {took:?}; {taken} readings of the list beside it, the slowest {slowest:?}");
    }

    /// **R17 at the server's own point, by name: an upload does not hold the
    /// one seat a limit of one leaves for its whole request.**
    ///
    /// At a limit of one every transfer, in either direction, rides `Servers`
    /// S14's one additional connection one at a time. **The upload gives its
    /// seat back at each file's end and asks again behind whoever waited**, so
    /// a download that asks while the upload's first file is moving is seated
    /// long before the upload's last file; an upload that held its seat would
    /// seat the download after all six.
    ///
    /// **Since step 10c's eleventh-pass fix piece 4 an upload moves up to
    /// `File Browser` O1's four files at once**, so up to three more of its
    /// files have asked for a seat before the download does, and the point
    /// seats in the order asked: **the download is seated fifth at the
    /// latest.** That is inside `File Browser` O5's share — a class waiting
    /// through N grants takes the next, N being the count of declared classes
    /// — and it is the price of the upload's four channels on a machine that
    /// allows one session, where they buy nothing. It was second when an
    /// upload moved one file at a time.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn an_upload_does_not_hold_the_one_seat_a_limit_of_one_leaves() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let scratch = Scratch::new("seat");
        let here = scratch.folder("here");
        let files: Vec<PathBuf> = (0..6)
            .map(|at| {
                let file = here.join(format!("part_{at}.bin"));
                std::fs::write(&file, a_file_of(1024 * 1024, at as u8 + 1)).expect("written");
                file
            })
            .collect();
        let there = scratch.folder("machine");
        let record: Arc<dyn TheRecordAnUploadWrites> = Arc::new(ARecordHere(Mutex::new(
            LocalRecord::open_at(&a_record_file_in(&scratch)).expect("opens"),
        )));
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        // **The cap slows each file to about two seconds**, so the download has
        // a moment in which to ask while the first file moves.
        let the_run = TheRunsTransfers::for_this_run();
        the_run.a_cap_was_saved(
            smoozy_server_protocol::transfers::Direction::Up,
            TheCap::Of(BytesPerSecond(1024 * 1024)),
        );
        let published = WhatWasPublished::default();

        let asked = an_upload_of(files.clone(), &there);
        let mut ready = an_upload_is_prepared(&asked, &point.sessions, record.as_ref())
            .await
            .expect("prepared");
        the_answer_is_applied(&mut ready, None, &point.sessions, record.as_ref()).await;

        let the_downloads_seat = tokio::spawn({
            let sessions = point.sessions.clone();
            let machine = Arc::clone(&machine);
            async move {
                // Ask once the upload's first file has its seat.
                while machine.additional_sessions_opened.load(Ordering::SeqCst) < 1 {
                    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                }
                let seat = sessions
                    .a_transfer_channel()
                    .await
                    .expect("the download is seated");
                let seated_as = machine.additional_sessions_opened.load(Ordering::SeqCst);
                drop(seat);
                seated_as
            }
        });
        let came_to = the_upload_runs(
            ready,
            &point.sessions,
            &the_run,
            Arc::clone(&record),
            &published,
        )
        .await;
        assert_eq!(came_to.uploaded, files.len(), "{came_to:?}");
        let seated_as = the_downloads_seat.await.expect("the download's task");
        assert!(
            seated_as <= 1 + smoozy_file_browser::pool::THE_UPLOAD_DIRECTIONS_CEILING,
            "the download was seated as seat {seated_as} of {} — behind more of the upload's \
             files than its channels, so the upload held its seats past a file's end",
            files.len() + 1
        );
    }

    /// **R1, at a session limit of one: the installation check confirms a
    /// healthy supervisor** — the state area read on the point's reading
    /// session, then piece 2's supervisor ask through the same point, the
    /// command running with nothing else open.
    ///
    /// **Before step 10c piece 4 the second half held a session open across its
    /// command**, and at a limit of one the command was refused and every
    /// connection took the fourth branch — piece 2 watched it on the loopback
    /// SSH server, and the account says what that run shows now.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn at_a_limit_of_one_the_installation_check_confirms_a_healthy_supervisor() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::where_a_root_may_sit("check");
        let installation = Identity::issue(IdentityKind::Installation).expect("randomness");
        let row = ServerRow {
            local_server_identifier: "server-1".into(),
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
            remembered_host_key: None,
            remembered_hardware_reading: None,
            recorded_machine_root: Some(RecordedMachineRoot {
                path: scratch.at.join("root").to_string_lossy().into_owned(),
                the_package_areas_own_location: None,
                read_under: installation.as_text(),
            }),
            last_confirmed_installation_identity: Some(installation.as_text()),
            event_cursor: None,
            machine_password_subject: None,
            chosen_fallback_folder: None,
            the_driver_missed_the_optix_bar: false,
        };
        let area = match where_the_package_area_is(&row) {
            WhereThePackageAreaIs::At(area) => area,
            other => panic!(
                "the row records a placement, and {} was answered {other:?}",
                scratch.at.display()
            ),
        };
        std::fs::create_dir_all(&area).expect("the package area");
        std::fs::write(
            Path::new(&area).join(THE_RECORD_THE_IDENTITY_SITS_IN),
            installation.as_text(),
        )
        .expect("the state area holds the identity");

        let machine = Arc::new(AMachine {
            program,
            home: scratch.folder("home"),
            limit: 1,
            supervisor: Some(TheSupervisor {
                installation: installation.clone(),
                package_area: PathBuf::from(&area),
                answered_by: WhoAnswers::TheHealthStandIn,
            }),
            log: Mutex::new(Vec::new()),
            additional_sessions_opened: AtomicUsize::new(0),
        });
        let mut connection = machine.a_connection("primary");
        let mut reading = TheMachinesInstallationIdentity::for_this_server(&row, OverTheConnection);
        let read: TheIdentityRead =
            the_check_through_a_point_of_its_own(&mut reading, &mut connection).await;

        let happened = machine.what_happened();
        assert_eq!(
            read.check,
            InstallationCheck::PresentAndEqualConfirmed,
            "a healthy supervisor was not confirmed at a limit of one: {happened:#?}"
        );
        assert!(
            happened
                .iter()
                .any(|line| line == "primary: command `~/.smoozy-carriage/package-area/smoozy-server` runs with 1 open"),
            "the command did not run alone: {happened:#?}"
        );
        // ~~**The carriage's link was taken away** once the command had run.~~
        // **The carriage's link stands once the command has run**, for the
        // asks after it on this connection — `Server protocol and transfers`
        // §1.1 as changed on 2026-10-26 (the worldwide pass's setup faults,
        // fault 1). Until then every ask took it away, and this read that it
        // was gone.
        assert!(std::fs::symlink_metadata(
            machine
                .home
                .join(THE_ENTRY_POINTS_CARRIAGE_SITE.link_path_on_the_subsystem)
        )
        .is_ok());
    }


    // ------------------------------------- the `.blend` version, read on the machine

    /// **A `.blend` as Blender writes its opening text** — the observed shapes
    /// (`File Browser` L8), compressed the way Blender 5 saves by default where
    /// `compressed`, and plain otherwise.
    fn a_blend(opening: &[u8], compressed: bool) -> Vec<u8> {
        let mut bytes = opening.to_vec();
        bytes.extend(a_file_of(64 * 1024, 7));
        if compressed {
            zstd::stream::encode_all(&bytes[..], 3).expect("compressed")
        } else {
            bytes
        }
    }

    /// The four Blenders at the desk, as `Server Setup`'s search answers them.
    fn the_four_blenders_at_the_desk() -> smoozy_render_settings::version::TheListThisPageAsksFor {
        smoozy_server_setup::search::TheSearchResult {
            entries: [(4, 5, 12), (5, 0, 1), (5, 1, 2), (5, 2, 0)]
                .iter()
                .map(|(major, minor, patch)| smoozy_server_setup::search::ACandidate {
                    path: format!("/home/ubuntu/smoozy/blender/{major}.{minor}.{patch}/blender"),
                    answered: Some(smoozy_server_setup::blender::Version {
                        major: *major,
                        minor: *minor,
                        patch: *patch,
                    }),
                })
                .collect(),
            answer: smoozy_server_setup::search::WhatTheAnswerSays {
                the_search_is_by_name: false,
                folders_refused: 0,
                did_not_answer: Vec::new(),
            },
            at: std::time::SystemTime::now(),
        }
    }

    /// **`File Browser` L7 to L10 through the server's own point, and
    /// `Render Settings` B2's automatic match made on what it read.** Step
    /// 10c's twelfth-pass fix piece 6b: nothing in the application ever took
    /// this reading, so every listing row went without a tag, Render Settings
    /// matched no file without a pick, and the page said the file did not say
    /// which Blender saved it.
    ///
    /// - The rows come back first, each `.blend` marked as having its tag to
    ///   follow (`File Browser` L7's *the four fields show first*).
    /// - Each tag is read on the reading session: a compressed 5.1 save, a
    ///   plain 4.5 save, and a file whose opening is no Blender's, which is
    ///   blank and never a guess (`File Browser` L8).
    /// - The same folder listed again carries the tags this period read, and
    ///   asks for none (`File Browser` L10); the period's end drops them.
    /// - Render Settings, handed no version, takes the reading itself and
    ///   matches the 5.1 file to 5.1.2 with no pick.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_blends_saved_with_version_is_read_through_the_servers_own_point_and_matched() {
        use smoozy_desktop::surface::files::{
            the_saved_with_version_of, the_tab_after_the_reading, the_tags_period_ended, ARow,
            TheListing,
        };
        use smoozy_desktop::surface::render_settings::{
            the_path_as_it_is_keyed, the_saved_with_version_lands, the_saved_with_version_to_hold,
            the_version_resolved, WhatTheRunHolds,
        };
        use smoozy_render_settings::version::{HowItMatched, Resolution, WhyNone};

        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let scratch = Scratch::new("tags");
        let browsed = scratch.folder("battle");
        std::fs::write(browsed.join("lion.blend"), a_blend(b"BLENDER17-01v0501REND", true)).expect("written");
        std::fs::write(browsed.join("dog.blend"), a_blend(b"BLENDER-v405REND", false)).expect("written");
        std::fs::write(browsed.join("not-one.blend"), a_file_of(4096, 3)).expect("written");
        std::fs::write(browsed.join("notes.txt"), b"three lines").expect("written");
        let machine = a_machine(program, &scratch.at, 1);
        let point = APointLending::over(&machine).await;
        // **The tags are held per server for the run**, so this test's server
        // is its own.
        let server = format!("piece-6b-{}", std::process::id());

        let rows_of = |found| -> Vec<ARow> {
            match the_tab_after_the_reading(Default::default(), &server, &the_bytes_of(&browsed), None, found, None, false).listing {
                TheListing::Rows { rows, .. } => rows,
                other => panic!("the folder listed: {other:?}"),
            }
        };
        let named = |rows: &[ARow], name: &str| -> ARow {
            rows.iter()
                .find(|row| row.name.bytes == name.as_bytes())
                .cloned()
                .expect("the row")
        };

        // The rows first, every `.blend` with its tag to follow.
        let rows = rows_of(a_folder_is_read(&point.sessions, &the_bytes_of(&browsed)).await);
        for name in ["lion.blend", "dog.blend", "not-one.blend"] {
            let row = named(&rows, name);
            assert!(row.saved_with.is_none() && row.the_tag_follows, "{name}: {row:?}");
        }
        assert!(!named(&rows, "notes.txt").the_tag_follows, "only a .blend has a tag");

        // Each tag, read on the machine.
        let tag_of = |name: &str| the_bytes_of(&browsed.join(name));
        assert_eq!(
            the_saved_with_version_of(&point.sessions, &server, &tag_of("lion.blend"), false).await.as_deref(),
            Some("5.1"),
            "a compressed 5.1 save"
        );
        assert_eq!(
            the_saved_with_version_of(&point.sessions, &server, &tag_of("dog.blend"), false).await.as_deref(),
            Some("4.5"),
            "a plain 4.5 save"
        );
        assert_eq!(
            the_saved_with_version_of(&point.sessions, &server, &tag_of("not-one.blend"), false).await,
            None,
            "an opening that is no Blender's is blank, never a guess"
        );

        // Listed again: the tags this period read ride the rows.
        let rows = rows_of(a_folder_is_read(&point.sessions, &the_bytes_of(&browsed)).await);
        assert_eq!(named(&rows, "lion.blend").saved_with.map(|tag| tag.as_str().to_string()).as_deref(), Some("5.1"));
        assert_eq!(named(&rows, "dog.blend").saved_with.map(|tag| tag.as_str().to_string()).as_deref(), Some("4.5"));
        for name in ["lion.blend", "dog.blend", "not-one.blend"] {
            assert!(!named(&rows, name).the_tag_follows, "{name}: read once this period");
        }

        // Render Settings, handed no version, reads it and matches without a pick.
        let lion = tag_of("lion.blend");
        let file = the_path_as_it_is_keyed(&lion);
        let mut held = WhatTheRunHolds::default();
        held.version_lists.insert(server.clone(), the_four_blenders_at_the_desk());
        assert_eq!(
            the_version_resolved(&held, &server, &file),
            Resolution::NoVersionResolves(WhyNone::NoSavedWithVersionToMatchOn),
            "nothing held yet"
        );
        let to_hold = the_saved_with_version_to_hold(Some(&point.sessions), &server, &lion, "").await;
        assert_eq!(to_hold, "5.1");
        the_saved_with_version_lands(&mut held, &server, &file, &to_hold);
        match the_version_resolved(&held, &server, &file) {
            Resolution::Automatic { build, how } => {
                assert_eq!((build.answered.major, build.answered.minor, build.answered.patch), (5, 1, 2));
                assert_eq!(how, HowItMatched::TheExactLine);
            }
            other => panic!("the 5.1 file matched no Blender: {other:?}"),
        }
        // A version handed is taken as handed, and nothing is read for it.
        assert_eq!(the_saved_with_version_to_hold(Some(&point.sessions), &server, &lion, "4.5").await, "4.5");

        // The period's end drops the tags; a fresh period reads afresh.
        the_tags_period_ended(&server);
        let rows = rows_of(a_folder_is_read(&point.sessions, &the_bytes_of(&browsed)).await);
        assert!(named(&rows, "lion.blend").the_tag_follows && named(&rows, "lion.blend").saved_with.is_none());
    }

    // ------------------------------------------------------ this test's own deadline

    /// **What every test in this file waits on**, named in the message a missed
    /// deadline prints.
    const WHAT_THE_TESTS_HERE_WAIT_ON: &str =
        "the machine's own smoozy-server and OpenSSH's own sftp-server on socket pairs";

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
}

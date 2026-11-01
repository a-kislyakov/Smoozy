//! **Step 10c piece 6's own proofs at the desk: the Submit button's whole act,
//! run through the server's own point to a machine standing in for a rental.**
//!
//! **What the far end is.** The stand-in `tests/through_the_servers_own_point.rs`
//! built for step 10c piece 4, in the same shape: a session limit the way
//! `MaxSessions` gives one, and every session **OpenSSH's own `sftp-server`**
//! over a Unix socket pair — so the remap material's transfer and the
//! material's single-rename publication run against the program a rental's
//! own file-transfer server is. **What it is not is an SSH connection**; the
//! loopback run outside the repository is where that is shown.
//!
//! **The supervisor is a stand-in, and it is named as one wherever it
//! answers.** Step 10c piece 5 builds the machine program's answer to *the
//! job record is written*; until then the desk build refuses it by name. So the
//! stand-in answers the request three ways — it takes the job, it refuses the
//! request by name as the desk build does, or nothing that can be read comes
//! back — and each way is one test. **When it takes the job it first checks,
//! on the machine's own disk and in the local record, that everything the
//! unit's order promises is already true.** *(Piece 5 has since built the
//! machine's own answer; the stand-in stays, because it is what lets each
//! answer be chosen.)* **It answers *does the machine hold this job* the way a
//! test asks** — refused by name, one of `State, identity and recovery` V1
//! step 2's three words, or the truth about what it took — which is how step
//! 10c's eighth-pass fix piece 4 proves the resolution in the same act.
//!
//! **Nothing of the Creator's is touched.** Every path, the local record and
//! Smoozy's temporary area are inside a throwaway folder made for the test and
//! deleted with it.
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
         this system ({}) has none of. The submission is unproved here.",
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
    use smoozy_desktop::record::download_records::DownloadArrangements;
    use smoozy_desktop::record::job_records::{JobRecords, TheKeptSetup};
    use smoozy_desktop::record::outcome_records::OutcomeRecords;
    use smoozy_desktop::record::LocalRecord;
    use smoozy_desktop::servers::a_period::this_connections_arrangements_are_driven;
    use smoozy_desktop::servers::sessions::{
        lend_until_it_ends, ACommandRunning, AHeldConnection, NoSession,
        TheAdditionalConnection, ThisServersSessions,
    };
    use smoozy_desktop::submitting::{
        an_act_is_begun, the_act_in_flight, the_act_is_performed, the_act_is_performed_in_flight,
        WhatBecameOfOneJob, WhatTheActCameTo, WhatThePageHandsOver,
        ONE_SCENE_JOBS_SUBMISSION,
    };
    use smoozy_desktop::uploading::TheRecordAnUploadWrites;
    use smoozy_download::arrangement::WhatSmoozyDid;
    use smoozy_job_files::layout::{Area, MachineRoot, TheLayout, WhereThePackageAreaSits};
    use smoozy_job_files::manifest::{FrameRange, SubmissionManifest};
    use smoozy_job_files::publication::AJobsPaths;
    use smoozy_queue_history::record::{
        AnswerOrigin, DependencyCheckAtSubmission, Hue, Standing, TheServerAsItStood,
    };
    use smoozy_render_settings::file_half::BlenderIdentity;
    use smoozy_render_settings::payload::{compose, Composition, Payload};
    use smoozy_render_settings::reporting::ReportingChoice;
    use smoozy_render_settings::store::SceneEntries;
    use smoozy_rendering::submission::{the_reporting_field, WhatTheDesktopHoldsAboutTheBuild};
    use smoozy_server_protocol::framing::{
        read_one_request, write_result_frame, Answered, Fields,
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
    };
    use smoozy_server_protocol::placement::{
        THE_ENTRY_POINTS_CARRIAGE_SITE, THE_HEALTH_EXCHANGE_COMMAND,
    };
    use smoozy_desktop::servers::stop_and_clear::TheWait;
    use smoozy_desktop::servers::the_machines_work::{
        the_held_jobs_answer_is_sent, AsksTheSupervisor, TheHeldJobsAnswer,
        WhatTheHeldJobsAnswerLeft,
    };
    use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
    use smoozy_server_protocol::framing::Request;
    use smoozy_server_protocol::requests::{
        IT_HAS_A_RECORDED_OUTCOME, IT_WAS_REMOVED, THE_ANSWER_FIELD, THE_ANSWER_IS_NO,
        THE_ANSWER_IS_YES, THE_ANSWER_TO_A_HOLD_REQUEST, THE_CANCELS_OUTCOME_FIELD,
        THE_CANCEL_REQUEST, THE_HOLD_FIELD, THE_RELEASE_OUTCOME_FIELD, THE_ROUTE_FIELD,
        THE_ROUTE_IS_THE_DESK, WHAT_BECAME_OF_IT_FIELD,
    };
    use smoozy_server_protocol::requests::{
        IT_IS_ABSENT, IT_IS_PRESENT, ON_THIS_MACHINE_FIELD, THE_CURSOR_FIELD,
        THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST, THE_JOB_FIELD, THE_JOB_RECORD_IS_WRITTEN_REQUEST,
        THE_HELD_BEHIND_FIELD, THE_HOLD_REASON_FIELD, THE_LABEL_FIELD, THE_MACHINE_CANNOT_SAY,
        THE_READ_FORWARD_REQUEST,
    };
    use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_state::outcomes::{Keeps, UnitOutcome};

    // --------------------------------------- where OpenSSH's own server is

    const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
        "/usr/libexec/sftp-server",
        "/usr/lib/openssh/sftp-server",
        "/usr/libexec/openssh/sftp-server",
        "/usr/lib/ssh/sftp-server",
    ];

    macro_rules! the_server_or_say_so {
        () => {
            match WHERE_SFTP_SERVER_LIVES
                .iter()
                .map(PathBuf::from)
                .find(|path| path.exists())
            {
                Some(path) => path,
                None => {
                    println!(
                        "SKIPPED: no OpenSSH sftp-server on this system ({}). \
                         The submission is unproved here.",
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
        /// **In cargo's own folder for integration tests, not the system's
        /// temporary one**, because the stage records a machine root in here
        /// and **`Job files and outputs` L3 refuses a root under a shared
        /// temporary area**. On Linux the system's temporary folder is `/tmp`
        /// itself; on a Mac it is under `/var/folders`, which L3 never sees —
        /// hosted run 35459394239 is where this was found, in a sibling file.
        fn new(named: &str) -> Scratch {
            let at = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
                .join(format!("smzy-submit-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&at);
            std::fs::create_dir_all(&at).expect("the throwaway folder is made");
            let at = std::fs::canonicalize(&at).expect("the folder is there");
            Scratch { at }
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.at);
        }
    }

    // ----------------------------------------------- a machine, standing in

    /// **How the stand-in supervisor answers the job record request.**
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TheSupervisor {
        /// The job record written — **step 10c piece 5's answer, stood in for.**
        TakesTheJob,
        /// **What the desk build of the machine's program answers today**: the
        /// request refused by name.
        DoesNotTakeJobs,
        /// The command ends with nothing that can be read.
        SaysNothing,
        /// **The first job record request refused by name, every later one
        /// taken** — so an act's first job is not in the line and the rest
        /// could be. Step 10c's eleventh-pass fix piece 8.
        RefusesItsFirstJob,
    }

    /// **How the stand-in supervisor answers *does the machine hold this
    /// job*** — `State, identity and recovery` V1 step 2's three words, or the
    /// request refused by name.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TheHoldAsk {
        /// **Refused by name**, as every request the stand-in does not answer.
        IsRefusedByName,
        /// **One of the three words, whatever the machine took.**
        Says(&'static str),
        /// **The truth**: *present* for a job whose record request the
        /// stand-in took, *absent* for every other — which is what the
        /// machine's own program answers from its queue store.
        IsAnsweredFromWhatItTook,
    }

    struct AMachine {
        program: PathBuf,
        home: PathBuf,
        limit: usize,
        package_area: PathBuf,
        the_supervisor: TheSupervisor,
        the_hold_ask: TheHoldAsk,
        /// **How long one command takes on the machine** — nothing for the
        /// ordinary proofs, and a real machine's order of it for the stall's
        /// reproduction, where a command's length is the window another ask
        /// can arrive in.
        a_command_takes: std::time::Duration,
        /// The jobs whose record request the stand-in supervisor took.
        taken: Mutex<std::collections::BTreeSet<String>>,
        /// The local record's own file, which the stand-in supervisor reads to
        /// see what the desktop wrote before it asked.
        record_file: PathBuf,
        /// The layout the machine's areas sit in, which the stand-in supervisor
        /// reads to see what was published before it was asked.
        layout: TheLayout,
        log: Mutex<Vec<String>>,
        /// **The machine's own program, where a test stands one up behind
        /// this point** — step 10c's twelfth-pass fix piece 4. Every request
        /// the health exchange carries then goes to a real supervisor through
        /// the real adapter, and the stand-in answers none of them.
        the_real_program: Mutex<Option<Arc<TheRealProgram>>>,
        /// **What happens the moment a held job's request reaches the machine,
        /// before the machine acts on it** — the last instant of the window
        /// between the rest's local record and the machine taking the rest.
        /// Taken once.
        the_moment_the_rest_arrives: Mutex<Option<Box<dyn FnOnce() + Send>>>,
        /// **What happens the moment a job record request reaches the
        /// machine, before the machine acts on it** — any job's, the stand-in's
        /// or the real program's. Taken once. The Creator's seventh notes,
        /// piece Z7.
        the_moment_the_record_is_asked: Mutex<Option<Box<dyn FnOnce() + Send>>>,
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

    struct AConnectionToIt {
        machine: Arc<AMachine>,
        open: Arc<AtomicUsize>,
        which: &'static str,
    }

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
            // **The machine's answer is fixed when the command starts**, as a
            // real command resolves its path when it is run, and only then
            // does it take its time.
            let answered = self.the_command_is_answered(command, frame);
            if !self.machine.a_command_takes.is_zero() {
                tokio::time::sleep(self.machine.a_command_takes).await;
            }
            Ok(answered)
        }

        /// **Where a command takes time, it runs beside the point**, as a
        /// real connection's does since step 10c's eighth-pass fix piece 1 —
        /// its answer fixed when it starts, and its length spent away from the
        /// connection. **Where it takes none, it is the whole command here**,
        /// which is the seam's own provided answer and what every proof in
        /// this file before the stall's reproduction was written against.
        async fn a_command_begun(
            &mut self,
            command: CommandString,
            frame: Option<Vec<u8>>,
        ) -> Result<ACommandRunning, NoSession> {
            let takes = self.machine.a_command_takes;
            if takes.is_zero() {
                return self
                    .a_command(command, frame)
                    .await
                    .map(ACommandRunning::already_ended);
            }
            let open = self.open.load(Ordering::SeqCst);
            if open >= self.machine.limit {
                self.machine
                    .says(format!("{}: command refused ({open} open)", self.which));
                return Err(NoSession::Refused);
            }
            self.machine.says(format!(
                "{}: command `{}` begins with {} open",
                self.which,
                command.as_str(),
                open + 1
            ));
            // **A real machine's shell looks for the program a moment after
            // the request arrives**, not as it is sent: sshd starts a process,
            // and only that process follows the planted link. So the answer is
            // fixed a little into the command, and anything that reaches the
            // machine in that moment — another session's removal of the link —
            // is what the command meets.
            let starting = THE_MOMENT_A_SHELL_TAKES_TO_START.min(takes);
            let here = AConnectionToIt {
                machine: Arc::clone(&self.machine),
                open: Arc::clone(&self.open),
                which: self.which,
            };
            Ok(ACommandRunning::to_be_run(async move {
                tokio::time::sleep(starting).await;
                let answered = here.the_command_is_answered(command, frame);
                tokio::time::sleep(takes - starting).await;
                Ok(answered)
            }))
        }

        async fn until_it_ends(&mut self) -> Ending {
            std::future::pending().await
        }

        async fn torn_down(self) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    /// **How long a real machine takes to start the program a command
    /// names**, in order of magnitude only — a process started and a shell's
    /// search of a path. **No machine's measurement**; it is what places the
    /// command's reading of the link a moment after its request, as on a
    /// rental, rather than at the instant it is sent.
    const THE_MOMENT_A_SHELL_TAKES_TO_START: std::time::Duration =
        std::time::Duration::from_millis(10);

    const THE_FOUR_VALUES_READ: &str =
        "stat -L -c '%s %i %Y %Z' ~/.smoozy-carriage/settings-page-blend";

    impl AConnectionToIt {
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
            // **`Render Settings` Q14 item 2's reading**, answered from the file
            // the carriage points at, in GNU `stat`'s four fields.
            if command.as_str() == THE_FOUR_VALUES_READ {
                use std::os::unix::fs::MetadataExt;
                let link = self
                    .machine
                    .home
                    .join(".smoozy-carriage/settings-page-blend");
                let Ok(found) = std::fs::metadata(&link) else {
                    return nothing;
                };
                return CommandResult {
                    output: format!(
                        "{} {} {} {}\n",
                        found.size(),
                        found.ino(),
                        found.mtime(),
                        found.ctime()
                    )
                    .into_bytes(),
                    complaint: Vec::new(),
                    status: Some(0),
                };
            }
            if command.as_str() != THE_HEALTH_EXCHANGE_COMMAND.as_str() {
                return nothing;
            }
            let link = self
                .machine
                .home
                .join(THE_ENTRY_POINTS_CARRIAGE_SITE.link_path_on_the_subsystem);
            let Some(frame) = frame else {
                return nothing;
            };
            let Ok(request) = read_one_request(&mut &frame[..]) else {
                return nothing;
            };
            // **The command runs through the planted link or not at all** — a
            // real shell finds no `smoozy-server` behind a link that is gone —
            // and the request it would have carried is named here, so a log
            // says whose ask met it.
            if std::fs::read_link(&link).ok().as_deref()
                != Some(self.machine.package_area.as_path())
            {
                self.machine.says(format!(
                    "the entry point's carriage points elsewhere, so `{}` never reached the supervisor",
                    request.name
                ));
                return nothing;
            }
            // **The machine's own program answers, where one stands** — the
            // real adapter carries the very frame the desktop sent to a real
            // supervisor, and its output is the command's. A held job's
            // request meets the test's own moment first.
            if request.name == THE_JOB_RECORD_IS_WRITTEN_REQUEST {
                let moment = self.machine.the_moment_the_record_is_asked.lock().expect("the moment").take();
                if let Some(moment) = moment {
                    moment();
                }
            }
            let real = self.machine.the_real_program.lock().expect("the program").clone();
            if let Some(real) = real {
                if request.name == THE_JOB_RECORD_IS_WRITTEN_REQUEST
                    && request.fields.text(THE_HELD_BEHIND_FIELD).is_some()
                {
                    let moment = self.machine.the_moment_the_rest_arrives.lock().expect("the moment").take();
                    if let Some(moment) = moment {
                        self.machine
                            .says("the held job's request reached the machine, and the test's moment came first".to_string());
                        moment();
                    }
                }
                self.machine
                    .says(format!("the machine's own program was asked `{}`", request.name));
                return real.answers(&frame);
            }
            let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0");
            if request.name == THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST
                && self.machine.the_hold_ask != TheHoldAsk::IsRefusedByName
            {
                let job = request.fields.text(THE_JOB_FIELD).unwrap_or_default().to_string();
                let word = match self.machine.the_hold_ask {
                    TheHoldAsk::Says(word) => word,
                    _ if self.machine.taken.lock().expect("taken").contains(&job) => IT_IS_PRESENT,
                    _ => IT_IS_ABSENT,
                };
                self.machine
                    .says(format!("the supervisor was asked `{}` and said `{word}`", request.name));
                return CommandResult {
                    output: write_result_frame(
                        &answered.carrying(Fields::none().and(ON_THIS_MACHINE_FIELD, word)),
                    ),
                    complaint: Vec::new(),
                    status: Some(0),
                };
            }
            // **`Rented-server runtime` U9's answer to a hold, stood in for**:
            // said on the log with the hold, the answer and the route it
            // carried, and answered with the release it would write — *yes*
            // releases, *no* stops (`Rented-server runtime` U12). Step 10c's eleventh-pass fix piece 8.
            if request.name == THE_ANSWER_TO_A_HOLD_REQUEST {
                let hold = request.fields.text(THE_HOLD_FIELD).unwrap_or_default().to_string();
                let answer = request.fields.text(THE_ANSWER_FIELD).unwrap_or_default().to_string();
                let route = request.fields.text(THE_ROUTE_FIELD).unwrap_or_default().to_string();
                self.machine.says(format!(
                    "the supervisor was asked `{}` for hold {hold}: `{answer}`, by `{route}`",
                    request.name
                ));
                let release = if answer == THE_ANSWER_IS_YES { "released" } else { "stopped" };
                return CommandResult {
                    output: write_result_frame(
                        &answered.carrying(Fields::none().and(THE_RELEASE_OUTCOME_FIELD, release)),
                    ),
                    complaint: Vec::new(),
                    status: Some(0),
                };
            }
            // **`Queue & History` L1's cancel, stood in for**: said on the log
            // with the job it names, and answered *removed*.
            if request.name == THE_CANCEL_REQUEST {
                let job = request.fields.text(THE_JOB_FIELD).unwrap_or_default().to_string();
                self.machine
                    .says(format!("the supervisor was asked `{}` for job {job}", request.name));
                return CommandResult {
                    output: write_result_frame(
                        &answered.carrying(
                            Fields::none()
                                .and(WHAT_BECAME_OF_IT_FIELD, IT_HAS_A_RECORDED_OUTCOME)
                                .and(THE_CANCELS_OUTCOME_FIELD, IT_WAS_REMOVED),
                        ),
                    ),
                    complaint: Vec::new(),
                    status: Some(0),
                };
            }
            if request.name != THE_JOB_RECORD_IS_WRITTEN_REQUEST {
                // **Every other request is refused by name**, as the desk build
                // refuses every request piece 5 has not answered.
                self.machine
                    .says(format!("the supervisor was asked `{}`", request.name));
                return CommandResult {
                    output: write_result_frame(
                        &answered.saying_it_does_not_answer_that_request(&request.name),
                    ),
                    complaint: Vec::new(),
                    status: Some(0),
                };
            }
            let job = request
                .fields
                .text(THE_JOB_FIELD)
                .unwrap_or_default()
                .to_string();
            let label = request.fields.text(THE_LABEL_FIELD).map(str::to_string);
            let provisional = rusqlite::Connection::open(&self.machine.record_file)
                .ok()
                .and_then(|record| {
                    record
                        .query_row(
                            "SELECT standing FROM job_record WHERE job = ?1",
                            [&job],
                            |row| row.get::<_, String>(0),
                        )
                        .ok()
                });
            let (published, placed) = match Identity::read_back(IdentityKind::Job, &job) {
                Some(identity) => (
                    AJobsPaths::composed(&self.machine.layout, &identity, label.as_deref())
                        .map(|paths| {
                            Path::new(&paths.job_record_directory)
                                .join(smoozy_job_files::manifest::MANIFEST_FILE_NAME)
                                .is_file()
                        })
                        .unwrap_or(false),
                    Path::new(
                        String::from_utf8_lossy(
                            smoozy_dependencies::remap::RemapMaterial::location_in(
                                &self.machine.layout,
                                &identity,
                            )
                            .as_bytes(),
                        )
                        .as_ref(),
                    )
                    .is_file(),
                ),
                None => (false, false),
            };
            // **The hold instruction, where one rides** (`Rendering` S13) — the
            // two fields, both or neither, said on the log so a test reads
            // exactly what the second job carried. Step 10c's eleventh-pass fix
            // piece 8.
            let held_behind = request.fields.text(THE_HELD_BEHIND_FIELD).map(str::to_string);
            let hold_reason = request.fields.text(THE_HOLD_REASON_FIELD).map(str::to_string);
            self.machine.says(format!(
                "the stand-in supervisor was asked for the job record of {job}, labelled {label:?}; \
                 the remap material placed {placed}, the material published {published}; \
                 held behind {held_behind:?}, for {hold_reason:?}; \
                 the local record holds it {provisional:?}"
            ));
            let the_first_it_was_asked = self
                .machine
                .what_happened()
                .iter()
                .filter(|line| line.starts_with("the stand-in supervisor was asked for the job record"))
                .count()
                == 1;
            let it_takes_it = match self.machine.the_supervisor {
                TheSupervisor::TakesTheJob => true,
                TheSupervisor::RefusesItsFirstJob => !the_first_it_was_asked,
                TheSupervisor::DoesNotTakeJobs | TheSupervisor::SaysNothing => false,
            };
            if it_takes_it {
                self.machine.taken.lock().expect("taken").insert(job.clone());
            }
            match self.machine.the_supervisor {
                _ if it_takes_it => CommandResult {
                    output: write_result_frame(&answered.carrying(Fields::none())),
                    complaint: Vec::new(),
                    status: Some(0),
                },
                TheSupervisor::DoesNotTakeJobs
                | TheSupervisor::RefusesItsFirstJob
                | TheSupervisor::TakesTheJob => CommandResult {
                    output: write_result_frame(
                        &answered.saying_it_does_not_answer_that_request(&request.name),
                    ),
                    complaint: Vec::new(),
                    status: Some(0),
                },
                TheSupervisor::SaysNothing => CommandResult {
                    output: Vec::new(),
                    complaint: b"the supervisor did not answer".to_vec(),
                    status: Some(1),
                },
            }
        }
    }

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

    // ------------------------------------------------------------- the record

    /// **The application's own record**, on a throwaway file, reached one short
    /// act at a time — **and every act that makes the provisional job record or
    /// a transfer record stand for the first time is written into the
    /// machine's log**, so the order of the desk's writes and the machine's
    /// acts can be read off one list.
    struct ARecordHere {
        record: Mutex<LocalRecord>,
        machine: Arc<AMachine>,
        seen: Mutex<(i64, i64)>,
        /// **How many times the Transfers page would have been told a transfer
        /// moved** — the running application's doors post
        /// `TRANSFERS_MOVED` at exactly these calls.
        moved: AtomicUsize,
    }

    impl smoozy_desktop::downloading::TheRecordADownloadWrites for ARecordHere {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            <ARecordHere as TheRecordAnUploadWrites>::with_the_record(self, act)
        }

        fn a_transfer_moved(&self) {
            self.moved.fetch_add(1, Ordering::SeqCst);
        }
        /// **Counts units' endings and nothing else**: bytes moving inside a file,
        /// and a request's run ending, are told to the page by their own call since
        /// step 10c's transfer wave piece 5, and are not what this test counts.
        fn a_transfer_is_moving(&self) {}
    }

    impl TheRecordAnUploadWrites for ARecordHere {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let Ok(record) = self.record.lock() else {
                return false;
            };
            act(record.connection());
            let count = |table: &str| {
                record
                    .connection()
                    .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                        row.get::<_, i64>(0)
                    })
                    .unwrap_or(0)
            };
            let (jobs, transfers) = (count("job_record"), count("transfer_record"));
            let mut seen = self.seen.lock().expect("seen");
            if jobs > seen.0 {
                self.machine
                    .says("the desk: a provisional job record was written".to_string());
            }
            if transfers > seen.1 {
                self.machine
                    .says("the desk: a transfer record was written".to_string());
            }
            *seen = (jobs, transfers);
            true
        }
    }

    // ------------------------------------------------------------ the act

    struct TheStage {
        scratch: Scratch,
        machine: Arc<AMachine>,
        record: Arc<ARecordHere>,
        layout: TheLayout,
        the_blend: PathBuf,
    }

    fn the_stage(
        program: PathBuf,
        named: &str,
        limit: usize,
        the_supervisor: TheSupervisor,
    ) -> TheStage {
        the_stage_where(
            program,
            named,
            limit,
            the_supervisor,
            TheHoldAsk::IsRefusedByName,
            std::time::Duration::ZERO,
        )
    }

    fn the_stage_where(
        program: PathBuf,
        named: &str,
        limit: usize,
        the_supervisor: TheSupervisor,
        the_hold_ask: TheHoldAsk,
        a_command_takes: std::time::Duration,
    ) -> TheStage {
        let scratch = Scratch::new(named);
        let home = scratch.at.join("home");
        let layout = TheLayout::of(
            MachineRoot::chosen(&home.join("root").display().to_string()).expect("a root"),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        // **The five areas exist before the first job** (`Job files and
        // outputs` N11) — a placement makes them; the stand-in has them.
        for area in smoozy_job_files::layout::EVERY_AREA {
            std::fs::create_dir_all(layout.area(area)).expect("an area");
        }
        let the_blend = PathBuf::from(layout.area(Area::Asset)).join("kitchen.blend");
        std::fs::write(&the_blend, b"BLENDER-v405 standing in for a scene").expect("the .blend");
        let record_file = scratch.at.join("record").join("smoozy.sqlite3");
        std::fs::create_dir_all(record_file.parent().expect("a folder")).expect("made");
        let machine = Arc::new(AMachine {
            program,
            home,
            limit,
            package_area: PathBuf::from(layout.area(Area::Package)),
            the_supervisor,
            the_hold_ask,
            a_command_takes,
            taken: Mutex::new(Default::default()),
            record_file: record_file.clone(),
            layout: layout.clone(),
            log: Mutex::new(Vec::new()),
            the_real_program: Mutex::new(None),
            the_moment_the_rest_arrives: Mutex::new(None),
            the_moment_the_record_is_asked: Mutex::new(None),
        });
        let record = Arc::new(ARecordHere {
            record: Mutex::new(LocalRecord::open_at(&record_file).expect("the record opens")),
            machine: Arc::clone(&machine),
            seen: Mutex::new((0, 0)),
            moved: AtomicUsize::new(0),
        });
        TheStage {
            scratch,
            machine,
            record,
            layout,
            the_blend,
        }
    }

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
                frame_range: Some(
                    smoozy_render_settings::frames::parse_the_frame_field(frames).expect("a range"),
                ),
                sample_count: Some(64),
                ..Default::default()
            },
            standing: None,
            stale: false,
        })
    }

    fn what_the_page_hands_over(
        stage: &TheStage,
        scene_jobs: Vec<Payload>,
        the_first_frame_check: bool,
    ) -> WhatThePageHandsOver {
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        WhatThePageHandsOver {
            server: TheServerAsItStood {
                local_server: issue(IdentityKind::LocalServer),
                label: Some("the loopback box".to_string()),
                hue: Some(Hue(3)),
            },
            installation: Some(issue(IdentityKind::Installation)),
            layout: stage.layout.clone(),
            package_area: stage.layout.area(Area::Package),
            the_blend: stage.the_blend.display().to_string(),
            label: Some("Kitchen, first pass".to_string()),
            reporting: the_reporting_field(None, ReportingChoice::Everything),
            the_version_origin: AnswerOrigin::ResolvedForThem,
            dependency_check: Some(DependencyCheckAtSubmission::NoStandingAtAll),
            what_the_desktop_holds_about_the_build:
                WhatTheDesktopHoldsAboutTheBuild::NothingButTheEntry,
            the_first_frame_check,
            // **The submit strip's own three**, off by default here: the tests
            // that turn the switch on set them themselves.
            the_download_as_it_renders_choice: false,
            where_the_downloads_go: None,
            the_formats_chosen: None,
            scene_jobs,
            the_reading_in_force: None,
            // **`Job files and outputs` M1b's card make**, as the page composes
            // it from the hardware reading `Servers` holds — an ordinary rented
            // GPU machine's.
            the_card_make: smoozy_job_files::manifest::TheCardMakeAtSubmission::TheMakes(vec![
                "NVIDIA".to_string(),
            ]),
            run: issue(IdentityKind::Run),
            carries_on_from: None,
            the_temporary_area: Some(stage.scratch.at.join("desk-temporary")),
            // **No check of this file ran in this run**, so no pair is
            // composed (step 10c's dependency wave piece 7).
            the_servers_fallback_folder: None,
            // **Nothing of `Server Setup` R1c was taken**, so nothing is held
            // for the driver; the tests that pose it set it themselves.
            the_driver_at_submission: Default::default(),
            the_typed_frames: None,
        }
    }

    async fn the_act(
        stage: &TheStage,
        point: &APointLending,
        given: WhatThePageHandsOver,
    ) -> WhatTheActCameTo {
        let sessions = point.sessions.clone();
        let record = Arc::clone(&stage.record);
        // **The Creator's seventh notes, piece Z7: every act here is begun as
        // the press begins it**, and how it ended decides its *Submitting…*
        // row — kept for the machine's own row only where a job is in the
        // line and none is unaccounted; gone on every refusal and on *not
        // known*.
        let in_flight = an_act_is_begun(&given);
        let which = in_flight.which();
        let came_to = tokio::task::spawn_blocking(move || {
            let the_run = TheRunsTransfers::for_this_run();
            the_act_is_performed_in_flight(given, &sessions, &the_run, record.as_ref(), in_flight).0
        })
        .await
        .expect("the act ran");
        // **`Queue & History` C10 as changed 2026-10-19**: every job in the
        // line keeps the row for the machine's own; a job not in the line
        // keeps it saying *Not submitted.*; an act Smoozy could not account
        // for, and a lock, take it away.
        let (in_the_line, unaccounted) = match &came_to {
            WhatTheActCameTo::TheseJobs(jobs) => (
                !jobs.is_empty() && jobs.iter().all(|job| *job == WhatBecameOfOneJob::ItIsInTheLine),
                jobs.is_empty()
                    || jobs.iter().any(|job| matches!(
                        job,
                        WhatBecameOfOneJob::NotKnown | WhatBecameOfOneJob::NotAttemptedBehindAnUnaccountedOne
                    )),
            ),
            WhatTheActCameTo::TheFirstFrameIsNotKnown { .. } => (false, false),
        };
        let row = the_act_in_flight(which);
        let expected = if in_the_line {
            Some((true, None))
        } else if unaccounted {
            None
        } else {
            Some((false, Some(came_to.clone())))
        };
        assert_eq!(
            row.as_ref().map(|row| (row.ended_in_the_line, row.not_submitted.clone())),
            expected,
            "the Submitting… row after {came_to:?}"
        );
        if let Some(row) = row.filter(|row| row.ended_in_the_line) {
            assert!(!row.jobs.is_empty(), "a kept row knows its jobs, for the machine's row to replace it");
        }
        // **Cleared by its cross**, so the next act on the same file finds the
        // line as the test left it.
        smoozy_desktop::submitting::a_not_submitted_row_is_cleared(which);
        came_to
    }

    fn the_jobs_on_the_machine(layout: &TheLayout) -> Vec<(String, SubmissionManifest)> {
        let mut found = Vec::new();
        for entry in std::fs::read_dir(layout.area(Area::JobRecord))
            .expect("the area")
            .flatten()
        {
            let name = entry.file_name().to_string_lossy().into_owned();
            if let Ok(bytes) = std::fs::read(
                entry
                    .path()
                    .join(smoozy_job_files::manifest::MANIFEST_FILE_NAME),
            ) {
                found.push((
                    name,
                    SubmissionManifest::read_back(&bytes).expect("a manifest"),
                ));
            }
        }
        found
    }

    fn position_of(log: &[String], wanted: &str) -> usize {
        log.iter()
            .position(|line| line.contains(wanted))
            .unwrap_or_else(|| panic!("`{wanted}` never happened: {log:#?}"))
    }

    // ---------------------------------------------------------------- the proofs

    /// **The whole act, at a session limit of one and above it, to a stand-in
    /// supervisor that takes the job** — `Rendering` S3's order, S4's unit and
    /// `Queue & History` Y3, one scene-job.
    ///
    /// - **The provisional record, with its kept setup, stands before anything
    ///   of the submission reaches the machine** (`Rendering` S3, `State,
    ///   identity and recovery` §7 step 2): the desk's own write comes before
    ///   the first transfer record, and the stand-in supervisor finds it
    ///   *provisional* when it is asked.
    /// - **The remap material is placed, then the material published by one
    ///   rename, then the record asked for** (`Rendering` S4;
    ///   `Dependency Upload & Remap` M10; `Job files and outputs` M5): the
    ///   manifest is at the job record directory's own path, nothing is left
    ///   in staging, and the manifest says the job output folder is chosen at
    ///   render start, its name carrying the label (the Creator's second
    ///   notes, piece 4: it was made at submission until then).
    /// - **The request carries the job and the label** (step 10c piece 1).
    /// - **The outcome record holds all three units done.**
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_job_is_submitted_whole_through_the_servers_own_point() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        for limit in [1, 10] {
            let stage = the_stage(
                program.clone(),
                &format!("whole-{limit}"),
                limit,
                TheSupervisor::TakesTheJob,
            );
            let point = APointLending::over(&stage.machine).await;
            let given = what_the_page_hands_over(&stage, vec![a_payload("1-240")], false);
            let came_to = the_act(&stage, &point, given).await;
            let log = stage.machine.what_happened();
            assert_eq!(
                came_to,
                WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]),
                "at a limit of {limit}: {log:#?}"
            );

            // The order, off the one log.
            let written = position_of(&log, "the desk: a provisional job record was written");
            let transferred = position_of(&log, "the desk: a transfer record was written");
            let asked = position_of(&log, "the stand-in supervisor was asked for the job record");
            assert!(written < transferred && transferred < asked, "{log:#?}");
            assert!(
                log[asked].contains("labelled Some(\"Kitchen, first pass\")"),
                "{}",
                log[asked]
            );
            assert!(
                log[asked].ends_with("Some(\"provisional\")"),
                "{}",
                log[asked]
            );
            assert!(
                log[asked].contains("the remap material placed true, the material published true"),
                "the record was asked for before the unit's first two steps had landed: {}",
                log[asked]
            );

            // The machine's own disk.
            let jobs = the_jobs_on_the_machine(&stage.layout);
            assert_eq!(jobs.len(), 1, "one job's material, published");
            let (_, manifest) = &jobs[0];
            let job = Identity::read_back(IdentityKind::Job, &manifest.job).expect("the job");
            let paths = AJobsPaths::composed(&stage.layout, &job, Some("Kitchen, first pass"))
                .expect("paths");
            assert!(Path::new(&paths.job_record_directory).is_dir());
            assert!(
                !Path::new(&paths.staging_directory).exists(),
                "nothing left staged"
            );
            // **No job output folder is made at submission** (`Job files and
            // outputs` M7 as narrowed 2026-10-04): the manifest says the folder
            // is chosen at render start, with `Job files and outputs` N3's name carrying the label,
            // and the render worker makes it where the file points (`Job files
            // and outputs` P21). The Creator's second notes, piece 4.
            assert!(!Path::new(&paths.job_output_folder).exists(), "no folder is made at submission");
            assert_eq!(
                manifest.job_output_folder,
                smoozy_job_files::manifest::WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                    the_folders_name: smoozy_job_files::names::job_output_folder_name(&job, Some("Kitchen, first pass")),
                    the_servers_fallback_folder_at_submission: None,
                }
            );
            assert_eq!(
                manifest.frame_range,
                FrameRange::TheUsersParsedIntegers((1..=240).collect())
            );
            assert_eq!(
                manifest.blend_path_on_the_machine,
                stage.the_blend.display().to_string()
            );
            let remap = smoozy_dependencies::remap::RemapMaterial::location_in(&stage.layout, &job);
            let remap = PathBuf::from(String::from_utf8_lossy(remap.as_bytes()).into_owned());
            let placed = std::fs::read(&remap).expect("the remap material was placed");
            let placed = smoozy_dependencies::remap::RemapMaterial::read_back(&placed)
                .expect("it reads back");
            assert_eq!(placed.job, job.as_text());
            assert!(
                std::fs::read_dir(
                    remap
                        .parent()
                        .expect("the remaps folder")
                        .join(".smoozy-staging")
                )
                .map(|staged| staged.count() == 0)
                .unwrap_or(true),
                "nothing left in the remap material's staging"
            );

            // The local record.
            let record = stage.record.record.lock().expect("the record");
            let jobs = JobRecords::over(record.connection());
            let kept = jobs.read(&job).expect("read").expect("the record stands");
            assert_eq!(
                kept.standing,
                Standing::Provisional,
                "confirmation is the next connection's"
            );
            assert_eq!(kept.label.as_deref(), Some("Kitchen, first pass"));
            assert!(kept.submitted_with_no_scene_named);
            assert_eq!(
                jobs.the_kept_setup_of(&job).expect("read"),
                Some(TheKeptSetup::Kept(a_payload("1-240"))),
                "record version 25: the kept setup went down with the record"
            );
            let outcome = the_submissions_outcome_record(record.connection());
            assert!(
                outcome
                    .units
                    .iter()
                    .all(|(_, ended)| ended == &Some(UnitOutcome::Done)),
                "{outcome:?}"
            );
            drop(record);
            drop(point);
        }
    }

    /// **The settings half *Read the file* gives for a one-scene file**: the
    /// scene `Scene`, frames 1 to 250.
    fn the_settings_half_of_a_one_scene_file() -> smoozy_render_settings::reading::SettingsHalf {
        use smoozy_render_settings::names::ExactName;
        use smoozy_render_settings::reading::{
            DeviceSetting, EarlyStop, LayerSamplesRule, OfferedEngineIdentifiers,
            PerEngineCounts, SceneReading, SettingsHalf, SimplifyReadBack, SwitchAnimation,
            ViewLayerReading,
        };
        let name = |text: &str| ExactName::as_the_reading_produced_it(text);
        SettingsHalf {
            scenes: vec![SceneReading {
                name: name("Scene"),
                frame_start: 1,
                frame_end: 250,
                frame_step: 1,
                frame_count: 250,
                full_size: (1920, 1080),
                resolution_percentage: 100,
                size_at_the_files_percentage: (1920, 1080),
                engine_as_read: name("CYCLES"),
                sample_counts: PerEngineCounts {
                    cycles: 514,
                    eevee: 64,
                },
                device: DeviceSetting::TheCards,
                early_stop: EarlyStop {
                    adaptive_sampling_is_on: true,
                    threshold: 0.01,
                    minimum: 0,
                },
                time_limit: 0.0,
                layer_samples_rule: LayerSamplesRule::Use,
                saved_with: "4.5.3".to_string(),
                view_layers: vec![ViewLayerReading {
                    name: name("ViewLayer"),
                    renders_at: vec![(1, true)],
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
            }],
            offered_engine_identifiers: OfferedEngineIdentifiers {
                cycles: Some(name("CYCLES")),
                eevee: Some(name("BLENDER_EEVEE")),
            },
            announced_add_on_case: None,
        }
    }

    /// **A reading of one scene, as *Read the file* leaves it standing on the
    /// page**, and the payload the page composes under it with that scene
    /// ticked.
    fn a_file_read_on_the_page() -> (smoozy_render_settings::file_half::RecordedReading, Payload) {
        use smoozy_render_settings::file_half::{FileHalf, FileKey, StartedUnder};
        use smoozy_render_settings::names::{ExactName, NamedAnswer, Provenance};
        let picked = BlenderIdentity {
            path: "/opt/blender-4.5.3/blender".to_string(),
            version: smoozy_server_setup::blender::Version {
                major: 4,
                minor: 5,
                patch: 3,
            },
        };
        let four_values = smoozy_server_protocol::transfers::SourceReading {
            size: 4096,
            inode: "1".to_string(),
            modification_time: 1,
            change_time: 1,
        };
        let key = FileKey {
            server: Identity::issue(IdentityKind::LocalServer).expect("random bytes"),
            path: "/srv/smoozy/assets/kitchen.blend".to_string(),
        };
        let name = |text: &str| ExactName::as_the_reading_produced_it(text);
        let mut half = FileHalf::default();
        let _ = half.a_reading_landed(
            key.clone(),
            &StartedUnder {
                version_pick: picked.clone(),
                file_identity: four_values.clone(),
            },
            the_settings_half_of_a_one_scene_file(),
            Some(&picked),
            &four_values,
            std::time::SystemTime::now(),
        );
        let reading = half.standing(&key).expect("the reading stands").clone();
        let ticked = NamedAnswer {
            name: name("Scene"),
            provenance: Provenance::ARecordedReading(reading.identity()),
        };
        let payload = compose(Composition {
            version_pick: &picked,
            scene: Some(&ticked),
            answers: &SceneEntries {
                frame_range: Some(
                    smoozy_render_settings::frames::parse_the_frame_field("1-240").expect("a range"),
                ),
                sample_count: Some(64),
                ..Default::default()
            },
            standing: Some(&reading),
            stale: false,
        });
        (reading, payload)
    }

    /// **Read the file, then Render: the job is submitted, and its manifest
    /// says the file was not read** — step 10c's four small things after *Read
    /// the file*, item 1, the runner's ruling of 2026-09-29. **Until then a
    /// file read on the page was refused at Submit** (*"Smoozy cannot submit a
    /// file after reading it on this page."*), on the ground that nothing in
    /// the running application could take a reading, which dependency piece 3
    /// made untrue.
    ///
    /// - **`Job files and outputs` R7's second bullet**: the manifest says no
    ///   reading and carries no map, and the render worker takes the reading at
    ///   the render's start.
    /// - **The scene the page anchored to its reading is carried as it is**:
    ///   `Rendering` P1a validates the name against the reading in force at the
    ///   attempt, whatever its anchor.
    /// - **The local record carries what the reading said** — `Queue & History`
    ///   N14's file's value beside each override (`Render Settings` Y1a), and
    ///   `Queue & History` E12a's ask as the reading gave it.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_file_read_on_the_page_is_submitted_and_its_manifest_says_no_reading() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "read-then-render", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let (reading, payload) = a_file_read_on_the_page();
        let mut given = what_the_page_hands_over(&stage, vec![payload], false);
        given.the_reading_in_force = Some(reading);
        let came_to = the_act(&stage, &point, given).await;
        let log = stage.machine.what_happened();
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]),
            "{log:#?}"
        );

        let jobs = the_jobs_on_the_machine(&stage.layout);
        assert_eq!(jobs.len(), 1, "one job's material, published");
        let (_, manifest) = &jobs[0];
        assert_eq!(
            manifest.recorded_reading,
            smoozy_job_files::manifest::TheRecordedReading::ThereIsNone,
            "`Job files and outputs` R7's second bullet"
        );
        assert_eq!(manifest.scene.name, "Scene");

        let job = Identity::read_back(IdentityKind::Job, &manifest.job).expect("the job");
        let record = stage.record.record.lock().expect("the record");
        let kept = JobRecords::over(record.connection())
            .read(&job)
            .expect("read")
            .expect("the record stands");
        let companion = kept
            .the_files_values_beside_the_overrides
            .expect("`Queue & History` N14: the file's values stood at submission");
        assert_eq!(companion.sample_count, Some(514));
        assert_eq!(
            companion.frame_range.map(|range| (range.start, range.end)),
            Some((1, 250))
        );
        assert_eq!(
            kept.device_asked_for,
            smoozy_queue_history::record::TheDeviceAsk::TheFilesOwnAsTheReadingInForceGaveIt(
                smoozy_render_settings::device::DeviceChoice::TheCards
            )
        );
        drop(record);
        drop(point);
    }

    struct TheMaterialDebt {
        material_folder: String,
        remap_path: String,
        done: [bool; 3],
    }

    // The payable record includes the exact installation and contents. Read
    // its debt flag, including any wrongly marked accepted or unknown job.
    fn the_material_debts_of(connection: &rusqlite::Connection) -> Vec<TheMaterialDebt> {
        connection
            .prepare(
                "SELECT material_folder, remap_path, material_done, remap_done, folder_done
            FROM submission_material WHERE debt = 1",
            )
            .unwrap()
            .query_map([], |row| {
                Ok(TheMaterialDebt {
                    material_folder: row.get(0)?,
                    remap_path: row.get(1)?,
                    done: [row.get(2)?, row.get(3)?, row.get(4)?],
                })
            })
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    }

    fn the_outcome_records_of(
        connection: &rusqlite::Connection,
        action: &str,
    ) -> Vec<smoozy_state::outcomes::Recorded> {
        let identities: Vec<String> = connection
            .prepare("SELECT identity FROM outcome_record WHERE action = ?1")
            .expect("prepared")
            .query_map([action], |row| row.get(0))
            .expect("read")
            .collect::<Result<_, _>>()
            .expect("read");
        let keeper = OutcomeRecords::over(connection);
        identities
            .iter()
            .filter_map(|text| Identity::read_back(smoozy_state::outcomes::KIND, text))
            .filter_map(|identity| keeper.read(&identity).ok().flatten())
            .collect()
    }

    fn the_submissions_outcome_record(
        connection: &rusqlite::Connection,
    ) -> smoozy_state::outcomes::Recorded {
        let mut found = the_outcome_records_of(connection, ONE_SCENE_JOBS_SUBMISSION);
        assert_eq!(found.len(), 1, "one scene-job, one outcome record");
        found.remove(0)
    }

    /// **The Creator's seventh notes, piece Z7: a submitted job is in the
    /// queue at once.** Its *Submitting…* row stands from the press — before
    /// anything reaches the machine, so before the file is read there — and
    /// through the act, knowing its job by the time the job record is asked
    /// for. It ends in the line, so the row waits for the machine's own.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn z7_the_submitting_row_stands_from_the_press_through_the_act() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "z7-row", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let given = what_the_page_hands_over(&stage, vec![a_payload("1-3")], false);
        let server = given.server.local_server.as_text();

        let in_flight = an_act_is_begun(&given);
        let which = in_flight.which();
        let at_the_press = the_act_in_flight(which).expect("the row stands at the press");
        assert_eq!(at_the_press.server, server);
        assert_eq!(at_the_press.name, "Kitchen, first pass");
        assert!(at_the_press.jobs.is_empty() && !at_the_press.ended_in_the_line);
        assert!(
            stage.machine.what_happened().is_empty(),
            "nothing has reached the machine yet: {:#?}",
            stage.machine.what_happened()
        );

        let seen = Arc::new(Mutex::new(None));
        let seen_here = Arc::clone(&seen);
        *stage.machine.the_moment_the_record_is_asked.lock().expect("the moment") = Some(Box::new(move || {
            *seen_here.lock().expect("seen") = Some(the_act_in_flight(which));
        }));
        let sessions = point.sessions.clone();
        let record = Arc::clone(&stage.record);
        let (came_to, named) = tokio::task::spawn_blocking(move || {
            let the_run = TheRunsTransfers::for_this_run();
            the_act_is_performed_in_flight(given, &sessions, &the_run, record.as_ref(), in_flight)
        })
        .await
        .expect("the act ran");
        assert_eq!(came_to, WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]));
        let during = seen
            .lock()
            .expect("seen")
            .clone()
            .expect("the job record was asked for")
            .expect("the row stands while the act runs");
        assert_eq!(during.jobs, vec![named[0].job.clone()], "the row knows its job by then");
        assert!(!during.ended_in_the_line);
        let after = the_act_in_flight(which).expect("the row waits for the machine's own");
        assert!(after.ended_in_the_line);
    }

    /// A refused act releases its submission turn and its sessions: the next
    /// healthy act on the same point must reach the supervisor without a restart.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_healthy_retry_after_a_refusal_reaches_the_machine_on_the_same_point() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        for limit in [1, 2] {
            let stage = the_stage(program.clone(), &format!("retry-after-refusal-{limit}"), limit, TheSupervisor::RefusesItsFirstJob);
            let point = APointLending::over(&stage.machine).await;
            let given = what_the_page_hands_over(&stage, vec![a_payload("1-3")], false);
            let first = tokio::time::timeout(std::time::Duration::from_secs(20), the_act(&stage, &point, given.clone())).await.expect("first act ended");
            assert_eq!(first, WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::TheServerDoesNotTakeJobs]));
            let began = std::time::Instant::now();
            let retry = tokio::time::timeout(std::time::Duration::from_secs(20), the_act(&stage, &point, given)).await.expect("healthy retry stalled");
            assert_eq!(retry, WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]), "{:#?}", stage.machine.what_happened());
            assert_eq!(stage.machine.taken.lock().unwrap().len(), 1, "exactly the retry was accepted");
            println!("refusal then healthy retry, session limit {limit}: {:?}", began.elapsed());
        }
    }

    /// **What the desk build of the machine's program answers today**: the job
    /// record request refused by name. **The job is not in the line, the
    /// provisional record goes, and `Rendering` S4b's debt is written** — the
    /// material published and never a job, named from the outcome record and
    /// never from the machine.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_machine_that_does_not_take_jobs_leaves_no_record_and_a_named_debt() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "refused", 1, TheSupervisor::DoesNotTakeJobs);
        let point = APointLending::over(&stage.machine).await;
        let given = what_the_page_hands_over(&stage, vec![a_payload("1-3")], false);
        let came_to = the_act(&stage, &point, given).await;
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::TheServerDoesNotTakeJobs]),
            "{:#?}",
            stage.machine.what_happened()
        );
        let record = stage.record.record.lock().expect("the record");
        let left: i64 = record
            .connection()
            .query_row("SELECT count(*) FROM job_record", [], |row| row.get(0))
            .expect("counted");
        assert_eq!(
            left, 0,
            "the provisional record of a job the machine refused stands"
        );
        let outcome = the_submissions_outcome_record(record.connection());
        assert_eq!(outcome.units[2].1, Some(UnitOutcome::Failed), "{outcome:?}");
        let debts = the_material_debts_of(record.connection());
        assert_eq!(debts.len(), 1, "one debt, for the one unit that published");
        let jobs = the_jobs_on_the_machine(&stage.layout);
        assert_eq!(
            jobs.len(),
            1,
            "the material was published before the refusal"
        );
        assert_eq!(
            PathBuf::from(&debts[0].material_folder),
            PathBuf::from(stage.layout.area(Area::JobRecord)).join(&jobs[0].0),
            "the debt names the exact material set"
        );
        assert!(!debts[0].remap_path.is_empty());
        assert!(Path::new(&debts[0].remap_path).exists());
        assert!(debts[0].done[..2].iter().all(|done| !*done), "the material parts are owed, not taken");
    }

    /// **`Rendering` S9 and `Rendering` S4a: a unit whose record ask brought nothing back is
    /// not known, its provisional record stands, and the walk stops behind
    /// it** — the next scene-job is not attempted and nothing of it reaches the
    /// machine. The hold ask that follows is refused by name, which is *could
    /// not be asked* and never *does not hold*.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_unit_nobody_can_account_for_keeps_its_record_and_ends_the_walk() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "silence", 1, TheSupervisor::SaysNothing);
        let point = APointLending::over(&stage.machine).await;
        let given =
            what_the_page_hands_over(&stage, vec![a_payload("1-3"), a_payload("4-6")], false);
        let came_to = the_act(&stage, &point, given).await;
        let log = stage.machine.what_happened();
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![
                WhatBecameOfOneJob::NotKnown,
                WhatBecameOfOneJob::NotAttemptedBehindAnUnaccountedOne,
            ]),
            "{log:#?}"
        );
        assert_eq!(
            log.iter()
                .filter(|line| line.contains("asked for the job record"))
                .count(),
            1,
            "the second unit was attempted behind one nobody accounted for: {log:#?}"
        );
        assert!(
            log.iter()
                .any(|line| line.contains("does the machine hold this job")),
            "{log:#?}"
        );
        assert_eq!(the_jobs_on_the_machine(&stage.layout).len(), 1);
        let record = stage.record.record.lock().expect("the record");
        let standing: String = record
            .connection()
            .query_row("SELECT standing FROM job_record", [], |row| row.get(0))
            .expect("one record stands");
        assert_eq!(standing, "provisional");
        let outcome = the_submissions_outcome_record(record.connection());
        assert_eq!(
            outcome.units[2].1, None,
            "the third unit is unaccounted, not failed"
        );
        assert!(
            the_material_debts_of(record.connection())
                .is_empty(),
            "no debt is taken for a job the machine may hold"
        );
    }

    // ------------------------------------------ the resolution, in the same act

    /// **One lost acknowledgement, resolved against a machine that answers the
    /// hold ask with `word`** — step 10c's eighth-pass fix piece 4.
    async fn a_lost_acknowledgement_resolved_by(
        program: PathBuf,
        named: &str,
        word: &'static str,
    ) -> (TheStage, WhatTheActCameTo) {
        let stage = the_stage_where(
            program,
            named,
            1,
            TheSupervisor::SaysNothing,
            TheHoldAsk::Says(word),
            std::time::Duration::ZERO,
        );
        let point = APointLending::over(&stage.machine).await;
        let mut given = what_the_page_hands_over(&stage, vec![a_payload("1-3")], false);
        // **The strip's switch on**, so the arrangement written with the
        // provisional record is there to be kept or taken with it.
        given.the_download_as_it_renders_choice = true;
        given.where_the_downloads_go = Some(the_bytes_of(&stage.scratch.at.join("chosen")));
        let came_to = the_act(&stage, &point, given).await;
        drop(point);
        (stage, came_to)
    }

    fn how_many(connection: &rusqlite::Connection, table: &str) -> i64 {
        connection
            .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| row.get(0))
            .expect("counted")
    }

    /// **`Rendering` S9 and `State, identity and recovery` §12 rule 4, in the
    /// same act: the record ask brought nothing back, the machine is asked,
    /// and it says *present* — so the job is in the line.** Until step 10c's
    /// eighth-pass fix piece 4 the answer was never read and this ended *could
    /// not tell* (the eighth pass on Hyperstack, fault 2).
    ///
    /// **Nothing is removed and no debt is owed**: the record stands as an
    /// acknowledged one does, for the machine's own events to confirm, and the
    /// walk goes on to the next unit.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_lost_acknowledgement_the_machine_says_it_holds_is_in_the_line() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let (stage, came_to) =
            a_lost_acknowledgement_resolved_by(program, "held", IT_IS_PRESENT).await;
        let log = stage.machine.what_happened();
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]),
            "{log:#?}"
        );
        assert!(
            log.iter().any(|line| line.contains("said `present`")),
            "the machine was asked, in the same act: {log:#?}"
        );
        let record = stage.record.record.lock().expect("the record");
        assert_eq!(how_many(record.connection(), "job_record"), 1, "the record stands");
        assert_eq!(how_many(record.connection(), "download_arrangement"), 1);
        let outcome = the_submissions_outcome_record(record.connection());
        assert!(
            outcome.units.iter().all(|(_, ended)| *ended == Some(UnitOutcome::Done)),
            "{outcome:?}"
        );
        assert!(
            the_material_debts_of(record.connection())
                .is_empty(),
            "no debt is owed for a job the machine holds"
        );
    }

    /// **The same, and the machine says *absent*: the job is not submitted, its
    /// record and its arrangement go, and `Rendering` S4b's debt names what was
    /// published** — the material set, from the outcome record and never from
    /// the machine.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_lost_acknowledgement_the_machine_says_is_absent_is_not_submitted_and_owes_its_debt()
    {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let (stage, came_to) =
            a_lost_acknowledgement_resolved_by(program, "absent", IT_IS_ABSENT).await;
        let log = stage.machine.what_happened();
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItStoppedBeforeTheJobRecord {
                the_step: smoozy_rendering::submission::THE_ORDERED_UNIT[2],
            }]),
            "{log:#?}"
        );
        let record = stage.record.record.lock().expect("the record");
        assert_eq!(
            how_many(record.connection(), "job_record"),
            0,
            "the record of a job the machine does not hold stands"
        );
        assert_eq!(
            how_many(record.connection(), "download_arrangement"),
            0,
            "the arrangement goes with its record"
        );
        let outcome = the_submissions_outcome_record(record.connection());
        assert_eq!(outcome.units[2].1, Some(UnitOutcome::Failed), "{outcome:?}");
        let debts = the_material_debts_of(record.connection());
        assert_eq!(debts.len(), 1, "one debt, for the one unit that published");
        let jobs = the_jobs_on_the_machine(&stage.layout);
        assert_eq!(jobs.len(), 1, "the material was published before the ask");
        assert_eq!(
            PathBuf::from(&debts[0].material_folder),
            PathBuf::from(stage.layout.area(Area::JobRecord)).join(&jobs[0].0),
            "the debt names the exact material set"
        );
        assert!(Path::new(&debts[0].remap_path).exists());
        assert!(debts[0].done[..2].iter().all(|done| !*done));
    }

    /// **The same, and the machine says it cannot say: the one answer that is
    /// *could not tell*.** The record and its arrangement stand, no debt is
    /// taken, and the third unit keeps no outcome for a later connection's own
    /// ask (`State, identity and recovery` V1 step 2's third branch).
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_lost_acknowledgement_the_machine_cannot_say_about_keeps_its_record() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let (stage, came_to) =
            a_lost_acknowledgement_resolved_by(program, "cannot-say", THE_MACHINE_CANNOT_SAY)
                .await;
        let log = stage.machine.what_happened();
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::NotKnown]),
            "{log:#?}"
        );
        let record = stage.record.record.lock().expect("the record");
        assert_eq!(how_many(record.connection(), "job_record"), 1, "the record stands");
        assert_eq!(how_many(record.connection(), "download_arrangement"), 1);
        let outcome = the_submissions_outcome_record(record.connection());
        assert_eq!(outcome.units[2].1, None, "the third unit is unaccounted");
        assert!(
            the_material_debts_of(record.connection())
                .is_empty(),
            "no debt is taken for a job the machine may hold"
        );
    }

    // --------------------------------------- the stall after the manifest

    /// **The eighth pass's stall, at the desk** — step 10c's eighth-pass fix
    /// piece 4. `Audits/Build/Step 10c/Eighth-pass fix piece 4 — what
    /// landed.md` holds what it found.
    ///
    /// **What stands around the act, as it stood at 16:17:38 on Hyperstack.**
    /// The server's own point; `Desktop app and delivery` P10d's asks going to
    /// the supervisor through that same point from a thread of their own, as
    /// `the_machines_work.rs`'s passes do — **the same route, with the pause
    /// between passes taken out**, so what the pass met now and then is met
    /// every time; `Download` R1's arrangement written with the provisional
    /// record and `a_period.rs`'s driving of this connection's arrangements
    /// live beside it; and every command taking a real machine's order of
    /// time. **The stand-in supervisor answers the hold ask truthfully**, so a
    /// resolution cannot hide a record request that never arrived.
    ///
    /// **What it found**: every supervisor ask is three acts at the point —
    /// the link planted, the one command run through it, the link taken away —
    /// and the taking away removed whatever link stood there, another ask's
    /// included. So the submission's record request ran with no link, never
    /// reached the supervisor, and ended *could not tell*. **The point now
    /// takes one supervisor ask at a time**
    /// (`ThisServersSessions::ask_the_supervisor`).
    ///
    /// **Two shapes, because the point has two.** At the limit every rented
    /// machine has shown, a command runs beside the point (step 10c's
    /// eighth-pass fix piece 1), which made the race rare; at a learned limit
    /// of one it runs to its end inside the point, as before, and there every
    /// submission lost its request until the point's own fix.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn the_stall_after_the_manifest_does_not_come_beside_live_asks() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        for limit in [10, 1] {
            the_stall_is_looked_for(program.clone(), limit).await;
        }
    }

    async fn the_stall_is_looked_for(program: PathBuf, limit: usize) {
        const SUBMISSIONS: usize = 6;
        let stage = the_stage_where(
            program,
            &format!("stall-{limit}"),
            limit,
            TheSupervisor::TakesTheJob,
            TheHoldAsk::IsAnsweredFromWhatItTook,
            std::time::Duration::from_millis(250),
        );
        let point = APointLending::over(&stage.machine).await;
        let package_area = stage.layout.area(Area::Package);
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));

        // **P10d's asks, from a thread of their own.**
        let passes = {
            let sessions = point.sessions.clone();
            let stop = Arc::clone(&stop);
            let package_area = package_area.clone();
            std::thread::spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("a runtime for the passes");
                let mut asked = 0usize;
                while !stop.load(Ordering::SeqCst) {
                    let request = smoozy_server_protocol::framing::Request::speaking(
                        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                        Identity::issue(IdentityKind::Operation).expect("randomness"),
                        THE_READ_FORWARD_REQUEST,
                    )
                    .carrying(Fields::none().and(THE_CURSOR_FIELD, "0"));
                    runtime.block_on(sessions.ask_the_supervisor(&package_area, &request));
                    asked += 1;
                }
                asked
            })
        };

        // **This connection's arrangements, driven** — on a thread and a
        // runtime of its own, as the period's own driving is; the wait it ends
        // on is the pause `Download` W2 names.
        let the_run = Arc::new(TheRunsTransfers::for_this_run());
        let arrangements = {
            let sessions = point.sessions.clone();
            let record = Arc::clone(&stage.record);
            let layout = stage.layout.clone();
            let the_run = Arc::clone(&the_run);
            let stop = Arc::clone(&stop);
            let given = what_the_page_hands_over(&stage, Vec::new(), false);
            let (server_row, run) = (given.server.local_server.clone(), given.run.clone());
            std::thread::spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("a runtime for the arrangements");
                let it_is_live = || !stop.load(Ordering::SeqCst);
                runtime.block_on(this_connections_arrangements_are_driven(
                    &sessions,
                    &the_run,
                    record.as_ref(),
                    &server_row,
                    &layout,
                    &run,
                    &it_is_live,
                ));
            })
        };

        let mut outcomes = Vec::new();
        for _ in 0..SUBMISSIONS {
            let mut given = what_the_page_hands_over(&stage, vec![a_payload("1-3")], false);
            given.the_download_as_it_renders_choice = true;
            given.where_the_downloads_go = Some(the_bytes_of(&stage.scratch.at.join("chosen")));
            let started = std::time::Instant::now();
            let came_to = the_act(&stage, &point, given).await;
            outcomes.push((came_to, started.elapsed()));
        }
        stop.store(true, Ordering::SeqCst);
        let asked = passes.join().expect("the passes ended");
        arrangements.join().expect("the arrangements' driving ended");

        let log = stage.machine.what_happened();
        let lost: Vec<&String> = log
            .iter()
            .filter(|line| line.contains("never reached the supervisor"))
            .collect();
        println!(
            "at a limit of {limit}: P10d's stand-in asked the supervisor {asked} times beside \
             {SUBMISSIONS} submissions"
        );
        for (came_to, took) in &outcomes {
            println!("  {came_to:?} in {took:?}");
        }
        println!("  asks that met no planted link: {}", lost.len());
        for line in lost.iter().take(12) {
            println!("  {line}");
        }
        assert!(asked > 0, "P10d's stand-in never asked, so nothing was looked for");
        assert!(
            outcomes.iter().all(|(came_to, _)| *came_to
                == WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine])),
            "at a limit of {limit}, a submission did not reach the line beside live asks: \
             {outcomes:#?}\n{lost:#?}"
        );
        assert!(
            lost.is_empty(),
            "at a limit of {limit}, an ask met a link another ask had taken away: {lost:#?}"
        );
        drop(point);
    }

    /// **`Server Setup` R1c and `Rendering` S18a at the Submit button: a cards
    /// job on an NVIDIA machine, whose driver is below its Blender's OptiX
    /// bar on a server where a swap is owed, goes held for the driver** — the
    /// reason alone, no job to be held behind — **and each of the readings
    /// failing once sends it plain**: the processor asked for, the driver at
    /// the bar, no swap owed, a miss kept, and a program on the server that is
    /// not this application's own. Held-job piece A, 2026-10-12.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_job_that_would_meet_the_old_driver_goes_held_and_every_other_goes_plain() {
        use smoozy_desktop::servers::the_driver_swap::TheDriverAtSubmission;
        use smoozy_render_settings::device::DeviceChoice;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        // **4.5.3's bar is 535** (`Rendering` L23a), and the driver reads 530
        // on a machine of its own kernel.
        let held = TheDriverAtSubmission {
            the_driver_on_this_connection: Some(smoozy_servers::hardware::TheDriversOwnAnswer {
                version: smoozy_servers::hardware::DriverVersion::parse("530.30.02").expect("530"),
                cuda_version: None,
                where_k1_placed_the_machine: smoozy_servers::hardware::WhereK1PlacesIt::ItsOwnKernel,
            }),
            a_missed_swap_stands: false,
            a_swap_is_owed: true,
            the_program_is_this_applications_own: true,
        };
        let at_the_bar = TheDriverAtSubmission {
            the_driver_on_this_connection: Some(smoozy_servers::hardware::TheDriversOwnAnswer {
                version: smoozy_servers::hardware::DriverVersion::parse("550.163.01").expect("550"),
                ..held.the_driver_on_this_connection.clone().expect("a driver")
            }),
            ..held.clone()
        };
        let cases: Vec<(&str, Option<DeviceChoice>, TheDriverAtSubmission, bool)> = vec![
            ("r1c-held", Some(DeviceChoice::TheCards), held.clone(), true),
            ("r1c-processor", Some(DeviceChoice::TheProcessor), held.clone(), false),
            ("r1c-at-the-bar", Some(DeviceChoice::TheCards), at_the_bar, false),
            (
                "r1c-nothing-owed",
                Some(DeviceChoice::TheCards),
                TheDriverAtSubmission { a_swap_is_owed: false, ..held.clone() },
                false,
            ),
            (
                "r1c-a-miss-kept",
                Some(DeviceChoice::TheCards),
                TheDriverAtSubmission { a_missed_swap_stands: true, ..held.clone() },
                false,
            ),
            (
                "r1c-an-older-program",
                Some(DeviceChoice::TheCards),
                TheDriverAtSubmission { the_program_is_this_applications_own: false, ..held.clone() },
                false,
            ),
        ];
        for (name, device, at_submission, it_is_held) in cases {
            let stage = the_stage(program.clone(), name, 1, TheSupervisor::TakesTheJob);
            let point = APointLending::over(&stage.machine).await;
            let mut payload = a_payload("40-43");
            payload.device = device;
            let mut given = what_the_page_hands_over(&stage, vec![payload], false);
            given.the_driver_at_submission = at_submission;
            let came_to = the_act(&stage, &point, given).await;
            let log = stage.machine.what_happened();
            assert_eq!(
                came_to,
                WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]),
                "{name}: a held job is a submitted job: {log:#?}"
            );
            let asked: Vec<&String> = log
                .iter()
                .filter(|line| line.starts_with("the stand-in supervisor was asked for the job record"))
                .collect();
            assert_eq!(asked.len(), 1, "{name}: {log:#?}");
            let expected = if it_is_held {
                format!("held behind None, for Some({:?});", smoozy_rendering::submission::THE_DRIVER)
            } else {
                "held behind None, for None;".to_string()
            };
            assert!(asked[0].contains(&expected), "{name}: {}", asked[0]);
        }
    }

    /// **`Rendering` S18a: where the first-frame check meets the driver, the
    /// first job is held for the driver and the second keeps the check's hold,
    /// behind the first** — no job carries two. Held-job piece A, 2026-10-12.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_first_frame_check_meets_the_driver_and_each_job_carries_one_hold() {
        use smoozy_desktop::servers::the_driver_swap::TheDriverAtSubmission;
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "r1c-first-frame", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let mut payload = a_payload("40-43");
        payload.device = Some(smoozy_render_settings::device::DeviceChoice::TheCards);
        let mut given = what_the_page_hands_over(&stage, vec![payload], true);
        given.the_driver_at_submission = TheDriverAtSubmission {
            the_driver_on_this_connection: Some(smoozy_servers::hardware::TheDriversOwnAnswer {
                version: smoozy_servers::hardware::DriverVersion::parse("530.30.02").expect("530"),
                cuda_version: None,
                where_k1_placed_the_machine: smoozy_servers::hardware::WhereK1PlacesIt::ItsOwnKernel,
            }),
            a_missed_swap_stands: false,
            a_swap_is_owed: true,
            the_program_is_this_applications_own: true,
        };
        let came_to = the_act(&stage, &point, given).await;
        let log = stage.machine.what_happened();
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine, WhatBecameOfOneJob::ItIsInTheLine]),
            "{log:#?}"
        );
        let asked: Vec<&String> = log
            .iter()
            .filter(|line| line.starts_with("the stand-in supervisor was asked for the job record"))
            .collect();
        assert_eq!(asked.len(), 2, "{log:#?}");
        let first = asked[0]
            .split("the job record of ")
            .nth(1)
            .and_then(|rest| rest.split(',').next())
            .expect("the job is named")
            .to_string();
        assert!(
            asked[0].contains(&format!("held behind None, for Some({:?});", smoozy_rendering::submission::THE_DRIVER)),
            "the first frame's job is held for the driver: {}",
            asked[0]
        );
        assert!(
            asked[1].contains(&format!("held behind Some({first:?}), for Some(\"the first-frame check\");")),
            "the rest keeps the check's hold, behind the first: {}",
            asked[1]
        );
    }

    /// **`Rendering` S13: with *Render one frame first* on, two ordinary jobs
    /// go — the first frame as its own job, and the whole range, its first
    /// frame included, as a second job that carries the hold instruction.**
    /// The second job record request names the first job's identity as the job
    /// it is held behind and `Rendering`'s own reason; the first carries
    /// neither. **Each has its own manifest**, the first covering the one frame
    /// and the second the whole range. *(The second covered the rest of the
    /// range until decision C, Creator-stated 2026-10-02.)*
    ///
    /// *Until step 10c's eleventh-pass fix piece 8 this test asserted the
    /// fault: the rest was refused before anything of it was sent, with one
    /// manifest on the machine. The field the refusal waited for had been named
    /// by step 10c piece 5 two weeks before.*
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_first_frame_goes_and_the_rest_goes_held_behind_it() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "first-frame", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let given = what_the_page_hands_over(&stage, vec![a_payload("40-43")], true);
        let came_to = the_act(&stage, &point, given).await;
        let log = stage.machine.what_happened();
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![
                WhatBecameOfOneJob::ItIsInTheLine,
                WhatBecameOfOneJob::ItIsInTheLine,
            ]),
            "{log:#?}"
        );
        let asked: Vec<&String> = log
            .iter()
            .filter(|line| line.starts_with("the stand-in supervisor was asked for the job record"))
            .collect();
        assert_eq!(asked.len(), 2, "two job record requests: {log:#?}");
        let the_job_in = |line: &str| {
            line.split("the job record of ")
                .nth(1)
                .and_then(|rest| rest.split(',').next())
                .expect("the job is named")
                .to_string()
        };
        let first = the_job_in(asked[0]);
        assert!(
            asked[0].contains("held behind None, for None;"),
            "the first frame's job carries no hold: {}",
            asked[0]
        );
        assert!(
            asked[1].contains(&format!(
                "held behind Some({first:?}), for Some({:?});",
                smoozy_rendering::submission::the_hold_instruction(
                    &Identity::issue(IdentityKind::Job).unwrap(),
                    &Identity::issue(IdentityKind::Job).unwrap()
                )
                .reason
            )),
            "the rest carries both fields, naming the first job: {}",
            asked[1]
        );
        // **What the queue draws the held entry from**: this run knows the
        // second job is the first-frame check's held range, behind the first
        // job's frame 40, holding all four frames — the count *Render all*
        // names, the held job's own.
        let second = Identity::read_back(IdentityKind::Job, &the_job_in(asked[1])).expect("a job");
        assert_eq!(
            smoozy_desktop::submitting::the_first_frame_check_behind(&second),
            Some(smoozy_desktop::submitting::TheFirstFrameCheck {
                the_first_job: Identity::read_back(IdentityKind::Job, &first).expect("a job"),
                the_first_frame: 40,
                frames_held: 4,
            })
        );
        let mut jobs = the_jobs_on_the_machine(&stage.layout);
        jobs.sort_by_key(|(name, _)| !name.starts_with(&first));
        assert_eq!(jobs.len(), 2, "both jobs reached the machine");
        assert_eq!(jobs[0].1.frame_range, FrameRange::TheUsersParsedIntegers(vec![40]));
        assert_eq!(
            jobs[1].1.frame_range,
            FrameRange::TheUsersParsedIntegers(vec![40, 41, 42, 43]),
            "the second job renders the whole range, its first frame again included"
        );
        // **`Render Settings` Y1b, `Rendering` S13: the check alone carries
        // *the check is written as a picture***, and it reaches the machine in
        // the manifest; the range job carries nothing new. Added 2026-10-27,
        // the nineteenth notes, notes 3 and 9.
        let marked = |manifest: &SubmissionManifest| {
            smoozy_render_settings::payload::Payload::read_back(&manifest.settings_payload)
                .expect("the manifest carries a payload")
                .the_check_is_written_as_a_picture
        };
        assert!(marked(&jobs[0].1), "the check is marked");
        assert!(!marked(&jobs[1].1), "the range job is not");
        // **`Rendering` S13's *writes into*, `Job files and outputs` P18a: the
        // check's payload names the range job, and its manifest carries the
        // range job's folder name, so the two write into one folder; the
        // range job carries its own name and nothing new.** Added 2026-10-31,
        // the twenty-first notes, note 6.
        let writes_into = |manifest: &SubmissionManifest| {
            smoozy_render_settings::payload::Payload::read_back(&manifest.settings_payload)
                .expect("the manifest carries a payload")
                .the_check_writes_into
        };
        assert_eq!(writes_into(&jobs[0].1), Some(second.as_text()), "the check names its range job");
        assert_eq!(writes_into(&jobs[1].1), None, "the range job names nobody");
        let the_folders_name = |manifest: &SubmissionManifest| match &manifest.job_output_folder {
            smoozy_job_files::manifest::WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                the_folders_name,
                ..
            } => the_folders_name.clone(),
            other => panic!("the folder is chosen at render start: {other:?}"),
        };
        assert_eq!(the_folders_name(&jobs[0].1), the_folders_name(&jobs[1].1), "one folder for the pair");
        assert_eq!(
            the_folders_name(&jobs[1].1),
            smoozy_job_files::names::job_output_folder_name(&second, Some("Kitchen, first pass")),
            "the range job's own name"
        );
        let record = stage.record.record.lock().expect("the record");
        let held: i64 = record
            .connection()
            .query_row("SELECT count(*) FROM job_record", [], |row| row.get(0))
            .expect("counted");
        assert_eq!(held, 2, "both jobs have their local record");
    }

    /// **Decision C's own test (Creator-stated 2026-10-02; `Rendering` S13 as
    /// amended): *Render one frame first* over 1–240 sends frame 1, then 1–240
    /// held**, and the count the queue's *Render all* names is the held job's
    /// own, 240. **A 1–1 range sends one job and no hold.** Each read back from
    /// the manifests on the machine.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_first_frame_check_holds_the_whole_range_and_one_frame_is_one_job() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();


        let stage = the_stage(program.clone(), "render-all", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let given = what_the_page_hands_over(&stage, vec![a_payload("1-240")], true);
        let came_to = the_act(&stage, &point, given).await;
        let log = stage.machine.what_happened();
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine, WhatBecameOfOneJob::ItIsInTheLine]),
            "{log:#?}"
        );
        assert_eq!(
            the_frames_on(&stage),
            vec![
                FrameRange::TheUsersParsedIntegers(vec![1]),
                FrameRange::TheUsersParsedIntegers((1..=240).collect()),
            ],
            "frame 1, then all 240 held, frame 1 again included"
        );
        let held = log
            .iter()
            .filter(|line| line.starts_with("the stand-in supervisor was asked for the job record"))
            .nth(1)
            .and_then(|line| line.split("the job record of ").nth(1))
            .and_then(|rest| rest.split(',').next())
            .and_then(|job| Identity::read_back(IdentityKind::Job, job))
            .expect("the held job is named");
        let check = smoozy_desktop::submitting::the_first_frame_check_behind(&held).expect("this run sent the check");
        assert_eq!((check.the_first_frame, check.frames_held), (1, 240));
        let pair = smoozy_desktop::surface::the_queue::the_first_frame_is_held("1", check.frames_held, true);
        let label: Vec<String> = pair
            .first_answer
            .iter()
            .flat_map(|part| [part.lead.to_string(), part.value.clone(), part.trail.to_string()])
            .filter(|word| !word.is_empty())
            .collect();
        assert_eq!(label.join(" "), "Render all — 240 frames");
        drop(point);

        // **1–1: one job, no hold.**
        let stage = the_stage(program, "render-one", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let given = what_the_page_hands_over(&stage, vec![a_payload("1-1")], true);
        let came_to = the_act(&stage, &point, given).await;
        let log = stage.machine.what_happened();
        assert_eq!(came_to, WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]), "{log:#?}");
        let asked: Vec<&String> = log
            .iter()
            .filter(|line| line.starts_with("the stand-in supervisor was asked for the job record"))
            .collect();
        assert_eq!(asked.len(), 1, "{log:#?}");
        assert!(asked[0].contains("held behind None, for None;"), "no hold: {}", asked[0]);
        drop(point);
    }

    /// **`Rendering` S13: the rest is never sent behind a first job the machine
    /// did not take** — a hold whose dependency was never written would never
    /// be dropped (`Rented-server runtime` U21) and would stand for ever. The
    /// act says the first job's reason, once, and nothing of the rest's own.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_rest_is_not_sent_behind_a_first_frame_the_machine_did_not_take() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "first-refused", 1, TheSupervisor::RefusesItsFirstJob);
        let point = APointLending::over(&stage.machine).await;
        let given = what_the_page_hands_over(&stage, vec![a_payload("40-43")], true);
        let came_to = the_act(&stage, &point, given).await;
        let log = stage.machine.what_happened();
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![
                WhatBecameOfOneJob::TheServerDoesNotTakeJobs,
                WhatBecameOfOneJob::NotSentBehindAFirstFrameThatWasNot,
            ]),
            "{log:#?}"
        );
        assert_eq!(
            log.iter()
                .filter(|line| line.starts_with("the stand-in supervisor was asked for the job record"))
                .count(),
            1,
            "nothing of the rest was asked for: {log:#?}"
        );
        let record = stage.record.record.lock().expect("the record");
        let held: i64 = record
            .connection()
            .query_row("SELECT count(*) FROM job_record", [], |row| row.get(0))
            .expect("counted");
        assert_eq!(held, 0, "neither job has a local record");
    }

    /// **The page as *Read the file* leaves it** — `settings` read and
    /// standing, `Render Settings` B1's list answered, nothing ticked and nothing typed — and
    /// the server row the page composes against. The reading is keyed by the
    /// stage's own `.blend`, so the page and the act name one file.
    fn the_page_after_a_read(
        stage: &TheStage,
        settings: smoozy_render_settings::reading::SettingsHalf,
    ) -> (
        smoozy_desktop::surface::render_settings::WhatTheRunHolds,
        String,
        smoozy_servers::store::ServerRow,
    ) {
        use smoozy_render_settings::file_half::{FileKey, StartedUnder};
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        let server = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let file = stage.the_blend.display().to_string();
        let version = smoozy_server_setup::blender::Version {
            major: 4,
            minor: 5,
            patch: 3,
        };
        let picked = BlenderIdentity {
            path: "/opt/blender-4.5.3/blender".to_string(),
            version,
        };
        let four_values = smoozy_server_protocol::transfers::SourceReading {
            size: 4096,
            inode: "1".to_string(),
            modification_time: 1,
            change_time: 1,
        };
        let mut held = smoozy_desktop::surface::render_settings::WhatTheRunHolds::default();
        let _ = held.readings.a_reading_landed(
            FileKey {
                server: server.clone(),
                path: file.clone(),
            },
            &StartedUnder {
                version_pick: picked.clone(),
                file_identity: four_values.clone(),
            },
            settings,
            Some(&picked),
            &four_values,
            std::time::SystemTime::now(),
        );
        held.version_lists.insert(
            server.as_text(),
            smoozy_render_settings::version::TheListThisPageAsksFor {
                entries: vec![smoozy_server_setup::search::ACandidate {
                    path: picked.path.clone(),
                    answered: Some(version),
                }],
                answer: smoozy_server_setup::search::WhatTheAnswerSays {
                    the_search_is_by_name: false,
                    folders_refused: 0,
                    did_not_answer: Vec::new(),
                },
                at: std::time::SystemTime::now(),
            },
        );
        let row = smoozy_servers::store::ServerRow {
            local_server_identifier: server.as_text(),
            host: Host::what_the_user_typed("198.51.100.10").expect("a host"),
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
                path: "/ephemeral/smoozy".to_string(),
                the_package_areas_own_location: None,
                read_under: "b".repeat(32),
            }),
            // **A row whose setup committed**: the page asks for a confirmed
            // installation as well as a root (step 10c's RunPod-pass fix
            // piece 1).
            last_confirmed_installation_identity: Some("b".repeat(32)),
            event_cursor: None,
            machine_password_subject: None,
            chosen_fallback_folder: None,
            the_driver_missed_the_optix_bar: false,
        };
        (held, file, row)
    }

    /// **What the page hands over at *Render*, as the page composed it** —
    /// its scene-jobs and the reading in force — on the stage's own machine.
    fn what_the_page_composed(
        stage: &TheStage,
        held: &smoozy_desktop::surface::render_settings::WhatTheRunHolds,
        file: &str,
        row: &smoozy_servers::store::ServerRow,
        the_first_frame_check: bool,
    ) -> WhatThePageHandsOver {
        let composed = smoozy_desktop::surface::render_settings::the_act_as_the_page_composed_it(
            held,
            &row.local_server_identifier,
            file,
            &smoozy_desktop::surface::render_settings::WhatTheRecordHolds::default(),
            row,
            Identity::issue(IdentityKind::Run).expect("random bytes"),
        )
        .unwrap_or_else(|said| panic!("the page could not compose the act: {said:?}"));
        let mut given = what_the_page_hands_over(stage, composed.scene_jobs, the_first_frame_check);
        given.the_reading_in_force = composed.the_reading_in_force;
        given
    }

    /// **The frame-range field of every manifest on the machine**, in the
    /// order of their first frames, the file's own last.
    fn the_frames_on(stage: &TheStage) -> Vec<FrameRange> {
        let mut jobs = the_jobs_on_the_machine(&stage.layout)
            .into_iter()
            .map(|(_, manifest)| manifest.frame_range)
            .collect::<Vec<_>>();
        // **By first frame, then by length**: since decision C (2026-10-02)
        // a first-frame check's two jobs both start at the range's first
        // frame, and the machine's own order between them is the jobs'
        // random identities.
        jobs.sort_by_key(|range| match range {
            FrameRange::TheUsersParsedIntegers(frames) => (0, frames.first().copied(), frames.len()),
            FrameRange::TheFilesOwn => (1, None, 0),
        });
        jobs
    }

    /// **Step 10c's Sesterce-pass fix piece 1, at the desk, in the pass's own
    /// order** (`Audits/Build/Step 10c/The pass before Vast — Sesterce.md`,
    /// fault 1): *Read the file*, `1-20` typed with no scene ticked, *Render*,
    /// and **the range read back from the manifest on the machine** — where
    /// the pass found `TheFilesOwn` and Blender on 1..250.
    ///
    /// **`Render Settings` F5**: with a reading standing and nothing ticked,
    /// the job composes from the scene in hand's own entries — the entry the
    /// page wrote the range under and draws it from.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_file_read_on_the_page_takes_its_typed_range_to_the_machine() {
        use smoozy_desktop::surface::render_settings::{the_answer_is_taken, WhichSetting};
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();


        // **The pass's own act: read, type `1-20`, *Render*.**
        let stage = the_stage(program, "read-type-render", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let (mut held, file, row) = the_page_after_a_read(&stage, the_settings_half_of_a_one_scene_file());
        assert!(
            the_answer_is_taken(&mut held, &row.local_server_identifier, &file, WhichSetting::FrameRange, "1-20")
                .is_none()
        );
        let given = what_the_page_composed(&stage, &held, &file, &row, false);
        let came_to = the_act(&stage, &point, given).await;
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]),
            "{:#?}",
            stage.machine.what_happened()
        );
        assert_eq!(
            the_frames_on(&stage),
            vec![FrameRange::TheUsersParsedIntegers((1..=20).collect())],
            "the manifest carries the range the page drew, never the file's own"
        );
        drop(point);
    }

    /// **The same pass, its second fault: *Render one frame first* on a file
    /// that was read** — refused with *this file has not been read* after the
    /// read, with the file's own range and with a typed one. Each case is read
    /// back from the manifests on the machine.
    ///
    /// **`Rendering` S13**: the first frame of the range, which for an
    /// untouched range is the file's own frame start from the reading in force
    /// (`Render Settings` F4); both jobs then carry explicit frames (`Render
    /// Settings` Y9's first form).
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_file_read_on_the_page_takes_its_first_frame_check_to_the_machine() {
        use smoozy_desktop::surface::render_settings::{the_answer_is_taken, WhichSetting};
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();


        // ***Render one frame first* after the read, the file's own range.**
        let stage = the_stage(program.clone(), "read-first-frame", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let (held, file, row) = the_page_after_a_read(&stage, the_settings_half_of_a_one_scene_file());
        let given = what_the_page_composed(&stage, &held, &file, &row, true);
        let came_to = the_act(&stage, &point, given).await;
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![
                WhatBecameOfOneJob::ItIsInTheLine,
                WhatBecameOfOneJob::ItIsInTheLine,
            ]),
            "{:#?}",
            stage.machine.what_happened()
        );
        assert_eq!(
            the_frames_on(&stage),
            vec![
                FrameRange::TheUsersParsedIntegers(vec![1]),
                FrameRange::TheUsersParsedIntegers((1..=250).collect()),
            ],
            "the file's frame start, then the file's whole range"
        );
        drop(point);

        // ***Render one frame first* after the read, `1-2` typed.**
        let stage = the_stage(program, "read-type-first-frame", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let (mut held, file, row) = the_page_after_a_read(&stage, the_settings_half_of_a_one_scene_file());
        assert!(
            the_answer_is_taken(&mut held, &row.local_server_identifier, &file, WhichSetting::FrameRange, "1-2")
                .is_none()
        );
        let given = what_the_page_composed(&stage, &held, &file, &row, true);
        let came_to = the_act(&stage, &point, given).await;
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![
                WhatBecameOfOneJob::ItIsInTheLine,
                WhatBecameOfOneJob::ItIsInTheLine,
            ]),
            "{:#?}",
            stage.machine.what_happened()
        );
        assert_eq!(
            the_frames_on(&stage),
            vec![
                FrameRange::TheUsersParsedIntegers(vec![1]),
                FrameRange::TheUsersParsedIntegers(vec![1, 2]),
            ]
        );
        drop(point);
    }

    /// **Where the first frame cannot be named, the check says what would name
    /// it, and nothing is sent** (`Design/Render Settings § Submitting`,
    /// 2026-10-01): a file of two scenes read, none ticked, nothing typed —
    /// *Type the frames or tick a scene.*; nothing read and nothing typed —
    /// *Type the frames.*
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_first_frame_check_that_cannot_name_its_frame_says_what_would_and_sends_nothing() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();


        let stage = the_stage(program, "first-frame-not-known", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let mut two_scenes = the_settings_half_of_a_one_scene_file();
        let mut second = two_scenes.scenes[0].clone();
        second.name = smoozy_render_settings::names::ExactName::as_the_reading_produced_it("Exterior");
        two_scenes.scenes.push(second);
        let (held, file, row) = the_page_after_a_read(&stage, two_scenes);
        let given = what_the_page_composed(&stage, &held, &file, &row, true);
        assert!(given.scene_jobs[0].scene.is_none(), "several scenes, none ticked, none named");
        assert_eq!(
            the_act(&stage, &point, given).await,
            WhatTheActCameTo::TheFirstFrameIsNotKnown { a_scene_can_be_ticked: true }
        );

        let mut nothing_read = a_payload("1");
        nothing_read.frame_range = None;
        let given = what_the_page_hands_over(&stage, vec![nothing_read], true);
        assert_eq!(
            the_act(&stage, &point, given).await,
            WhatTheActCameTo::TheFirstFrameIsNotKnown { a_scene_can_be_ticked: false }
        );
        assert!(the_jobs_on_the_machine(&stage.layout).is_empty(), "nothing was sent");
        drop(point);
    }

    /// **The supervisor, asked through the server's own point** — the seam
    /// `src/servers/the_machines_work.rs` asks through, given this stand-in's
    /// point. Each ask is waited on from a thread of its own, as that file's
    /// own end does.
    struct OverThisPoint {
        sessions: ThisServersSessions<AConnectionToIt>,
        package_area: String,
    }

    impl AsksTheSupervisor for OverThisPoint {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            let (sessions, package_area) = (&self.sessions, self.package_area.as_str());
            std::thread::scope(|scope| {
                scope
                    .spawn(move || {
                        tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .expect("a runtime for one ask")
                            .block_on(sessions.ask_the_supervisor(package_area, request))
                    })
                    .join()
                    .expect("the ask did not panic")
            })
        }
    }

    /// **Step 10c's eleventh-pass fix piece 8, at the desk: *Render the rest*
    /// and *Cancel* each send one answer naming the hold**, over the
    /// server's own point — `Queue & History` H2, `Rented-server runtime` U9
    /// and `Rented-server runtime` U10: an ordinary command with its own identity, naming the hold,
    /// carrying the answer, by the desk's route. ***Cancel* then cancels the
    /// job** (`Queue & History` L5), because the machine's *no* removes
    /// nothing and `Specs/Queue & History` says saying no leaves nothing
    /// behind.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn render_the_rest_and_cancel_it_each_send_one_answer_naming_the_hold() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let stage = the_stage(program, "held-answers", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let (job, hold) = (
            Identity::issue(IdentityKind::Job).expect("random bytes"),
            Identity::issue(IdentityKind::Operation).expect("random bytes"),
        );
        let mut machine = OverThisPoint {
            sessions: point.sessions.clone(),
            package_area: stage.layout.area(Area::Package),
        };
        let wait = TheWait {
            deadline: std::time::Duration::from_secs(10),
            pause: std::time::Duration::from_millis(10),
        };
        let (job_sent, hold_sent) = (job.clone(), hold.clone());
        let (the_rest, cancel_it) = tokio::task::spawn_blocking(move || {
            (
                the_held_jobs_answer_is_sent(&mut machine, &job_sent, &hold_sent, TheHeldJobsAnswer::RenderAll, wait),
                the_held_jobs_answer_is_sent(&mut machine, &job_sent, &hold_sent, TheHeldJobsAnswer::CancelIt, wait),
            )
        })
        .await
        .expect("the answers ran");
        let log = stage.machine.what_happened();
        assert_eq!(the_rest, WhatTheHeldJobsAnswerLeft::ItIsInTheLine, "{log:#?}");
        assert_eq!(cancel_it, WhatTheHeldJobsAnswerLeft::ItWasRemoved, "{log:#?}");
        let asked: Vec<&String> = log
            .iter()
            .filter(|line| line.starts_with("the supervisor was asked"))
            .collect();
        assert_eq!(
            asked,
            vec![
                &format!(
                    "the supervisor was asked `{THE_ANSWER_TO_A_HOLD_REQUEST}` for hold {}: `{THE_ANSWER_IS_YES}`, by `{THE_ROUTE_IS_THE_DESK}`",
                    hold.as_text()
                ),
                &format!(
                    "the supervisor was asked `{THE_ANSWER_TO_A_HOLD_REQUEST}` for hold {}: `{THE_ANSWER_IS_NO}`, by `{THE_ROUTE_IS_THE_DESK}`",
                    hold.as_text()
                ),
                &format!("the supervisor was asked `{THE_CANCEL_REQUEST}` for job {}", job.as_text()),
            ],
            "one answer each, naming the hold, and the cancel after the no: {log:#?}"
        );
        drop(point);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 10c's twelfth-pass fix piece 4: the held rest shows in Queue — the
    // pass's order replayed against the machine's own program
    // ═══════════════════════════════════════════════════════════════════════

    /// **The machine's own program, standing behind the stand-in's point** —
    /// a real supervisor in the stage's own package area, asked through the
    /// real adapter, which is the whole route a desktop request takes on a
    /// rental (`Rented-server runtime — program/tests/the_program_runs.rs`
    /// stands one up the same way). **Its line is held** by the
    /// setup-in-progress state, so every job it takes waits and none starts a
    /// Blender this desk does not have.
    struct TheRealProgram {
        area: PathBuf,
        endpoint: PathBuf,
        supervisor: Mutex<Option<std::process::Child>>,
    }

    /// **How long one asking of the machine's own program is given** — a
    /// test's own bound; an answer takes well under a second.
    const HOW_LONG_ONE_ASKING_IS_GIVEN: std::time::Duration = std::time::Duration::from_secs(60);

    impl TheRealProgram {
        /// **The program this workspace built**, beside this test's own
        /// executable — present whenever the workspace's tests were built,
        /// since the program's own tests need it.
        fn the_one_built() -> Option<PathBuf> {
            let here = std::env::current_exe().ok()?;
            let built = here
                .parent()?
                .parent()?
                .join(smoozy_runtime::the_program::THE_FIXED_ENTRY_POINT_NAME);
            built.is_file().then_some(built)
        }

        fn started_in(layout: &TheLayout, built: &Path) -> TheRealProgram {
            use smoozy_runtime::the_program::{
                records, the_active_package_pointer_at, the_record_at, THE_FIXED_ENTRY_POINT_NAME,
                THE_SUPERVISOR_WORD,
            };
            let area = PathBuf::from(layout.area(Area::Package));
            let area_text = area.display().to_string();
            std::os::unix::fs::symlink(built, the_active_package_pointer_at(&area_text))
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
            // **The machine root the stage publishes under**, recorded where
            // the program reads it.
            std::fs::write(
                the_record_at(&area_text, smoozy_job_files::layout::THE_RECORD_OF_THE_ROOT),
                layout.root().as_path(),
            )
            .expect("the root is recorded");
            let supervisor = std::process::Command::new(the_active_package_pointer_at(&area_text))
                .arg(THE_SUPERVISOR_WORD)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("the supervisor starts");
            let program = TheRealProgram {
                area,
                endpoint,
                supervisor: Mutex::new(Some(supervisor)),
            };
            let health = Request::speaking(
                THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                Identity::issue(IdentityKind::Operation).expect("randomness"),
                smoozy_server_protocol::requests::THE_HEALTH_ANSWER_REQUEST,
            );
            let answered = smoozy_runtime::acting::endpoint::TheEndpointAsACaller::at(
                &program.endpoint.display().to_string(),
                std::time::Duration::from_secs(1),
            )
            .ask_waiting_for_a_supervisor(&health, HOW_LONG_ONE_ASKING_IS_GIVEN)
            .expect("the endpoint is reachable at all");
            assert!(answered.is_some(), "the machine's own supervisor never answered");
            // **The line held**, so every job written waits (`Rented-server
            // runtime` P1).
            let held = program.ask(
                &Request::speaking(
                    THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
                    Identity::issue(IdentityKind::Operation).expect("randomness"),
                    smoozy_server_protocol::requests::THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
                )
                .carrying(Fields::none().and(
                    smoozy_server_protocol::requests::THE_SETUP_RUN_FIELD,
                    Identity::issue(IdentityKind::Operation).expect("randomness").as_text(),
                )),
            );
            assert!(
                matches!(&held, WhatCameBack::ItAnswered(answered)
                    if answered.the_request_name_it_says_it_does_not_answer().is_none()),
                "the line is held: {held:?}"
            );
            program
        }

        /// **One framed request, through the real adapter**, and the adapter's
        /// output as the command's.
        fn answers(&self, frame: &[u8]) -> CommandResult {
            use std::io::Write;
            let pointer = smoozy_runtime::the_program::the_active_package_pointer_at(
                &self.area.display().to_string(),
            );
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
            let Ok(Ok(said)) = heard.recv_timeout(HOW_LONG_ONE_ASKING_IS_GIVEN) else {
                let _ = std::process::Command::new("kill")
                    .args(["-KILL", &identifier.to_string()])
                    .status();
                panic!("DEADLINE MISSED: the machine's own program never answered one asking");
            };
            CommandResult {
                output: said.stdout,
                complaint: said.stderr,
                status: said.status.code().and_then(|code| u32::try_from(code).ok()),
            }
        }

        /// **Asked straight, with no point between** — for the pass that lands
        /// while the point is itself inside a command.
        fn ask(&self, request: &Request) -> WhatCameBack {
            let said = self.answers(&smoozy_server_protocol::framing::write_request_frame(request));
            match smoozy_server_protocol::framing::read_one_result(&mut said.output.as_slice()) {
                Ok(answered) if answered.answers(request) => WhatCameBack::ItAnswered(answered),
                _ => WhatCameBack::NothingCorrelatedCameBack,
            }
        }
    }

    impl Drop for TheRealProgram {
        fn drop(&mut self) {
            if let Some(mut supervisor) = self.supervisor.lock().ok().and_then(|mut held| held.take()) {
                let _ = supervisor.kill();
                let _ = supervisor.wait();
            }
            // **Every program started through the area**, which the
            // supervisor's own death does not reach.
            let _ = std::process::Command::new("pkill")
                .args(["-KILL", "-f", &self.area.display().to_string()])
                .status();
            let _ = std::fs::remove_file(&self.endpoint);
        }
    }

    /// The program, asked straight.
    struct StraightToTheProgram(Arc<TheRealProgram>);

    impl AsksTheSupervisor for StraightToTheProgram {
        fn ask(&mut self, request: &Request) -> WhatCameBack {
            self.0.ask(request)
        }
    }

    /// **The record lent one read or one write at a time**, as the running
    /// application lends it to a connection's passes.
    impl smoozy_desktop::keys::LendsTheRecord for ARecordHere {
        fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
            if let Ok(record) = self.record.lock() {
                act(record.connection());
            }
        }

        fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
    }

    /// **A server row for the stage**, as *Add* leaves one.
    fn a_server_row_in(record: &ARecordHere) -> Identity {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        let row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let written = smoozy_servers::store::ServerRow {
            local_server_identifier: row.as_text(),
            host: Host::what_the_user_typed("198.51.100.10").expect("a host"),
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
            recorded_machine_root: None,
            last_confirmed_installation_identity: None,
            event_cursor: None,
            machine_password_subject: None,
            chosen_fallback_folder: None,
            the_driver_missed_the_optix_bar: false,
        };
        let held = record.record.lock().expect("the record");
        smoozy_desktop::record::server_rows::add(held.connection(), &written).expect("the row is added");
        row
    }

    /// **One run of the application, as far as the queue is concerned**: its
    /// record, its register of what each period holds, and its point.
    struct ARun {
        record: Arc<ARecordHere>,
        holds: Arc<smoozy_desktop::servers::the_machines_work::WhatThisRunHolds>,
        point: APointLending,
        connection: u64,
    }

    impl ARun {
        /// ***Connect*: the point lent and `Servers` Q1 step 7 run on it** —
        /// through the point, as the running application runs it.
        async fn connected(stage: &TheStage, record: Arc<ARecordHere>, server: &Identity) -> ARun {
            let holds = Arc::new(smoozy_desktop::servers::the_machines_work::WhatThisRunHolds::default());
            let point = APointLending::over(&stage.machine).await;
            let connection = holds.a_connection_begins(&server.as_text());
            let run = ARun {
                record,
                holds,
                point,
                connection,
            };
            let (record, holds, server) = (Arc::clone(&run.record), Arc::clone(&run.holds), server.clone());
            let mut machine = OverThisPoint {
                sessions: run.point.sessions.clone(),
                package_area: stage.layout.area(Area::Package),
            };
            tokio::task::spawn_blocking(move || {
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
            run
        }

        /// **One P10d pass with the Queue on screen**: the read forward from
        /// the cursor, then the queue reading — through the point.
        async fn a_pass(&self, stage: &TheStage, server: &Identity) {
            let (record, holds, server, connection) = (
                Arc::clone(&self.record),
                Arc::clone(&self.holds),
                server.clone(),
                self.connection,
            );
            let mut machine = OverThisPoint {
                sessions: self.point.sessions.clone(),
                package_area: stage.layout.area(Area::Package),
            };
            tokio::task::spawn_blocking(move || {
                smoozy_desktop::servers::the_machines_work::a_read_forward_pass(
                    &server,
                    record.as_ref(),
                    &mut machine,
                    &holds,
                );
                let reading = smoozy_desktop::servers::the_machines_work::the_queue_reading_from(&mut machine)
                    .expect("the machine's own program reads its queue");
                holds.a_reading_was_taken(
                    &server.as_text(),
                    connection,
                    smoozy_state::observed::Reading::taken(
                        reading,
                        std::time::SystemTime::now(),
                        smoozy_state::observed::Clock::ThisComputer,
                    ),
                    None,
                );
            })
            .await
            .expect("the pass ran");
        }

        /// ***Render one frame first* over four frames, pressed on this run** —
        /// and the rest's identity, read off the machine's own queue as the one
        /// held job that was not there before.
        async fn a_first_frame_check(&self, stage: &TheStage, server: &Identity, real: &Arc<TheRealProgram>) -> Identity {
            let held_before = the_held_jobs_on(real);
            let mut given = what_the_page_hands_over(stage, vec![a_payload("40-43")], true);
            given.server.local_server = server.clone();
            let (sessions, record) = (self.point.sessions.clone(), Arc::clone(&self.record));
            let came_to = tokio::task::spawn_blocking(move || {
                let the_run = TheRunsTransfers::for_this_run();
                the_act_is_performed(given, &sessions, &the_run, record.as_ref())
            })
            .await
            .expect("the act ran");
            assert_eq!(
                came_to,
                WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine, WhatBecameOfOneJob::ItIsInTheLine]),
                "both jobs of the check are in the machine's line: {:#?}",
                stage.machine.what_happened()
            );
            let mut new: Vec<Identity> = the_held_jobs_on(real)
                .into_iter()
                .filter(|job| !held_before.contains(job))
                .collect();
            assert_eq!(new.len(), 1, "the machine holds the check's rest");
            new.remove(0)
        }

        /// **The Queue as the screen composes it**, with the rest asked for,
        /// and the server row's work column beside it.
        fn the_queue(
            &self,
            chosen: &Identity,
        ) -> (
            smoozy_desktop::surface::the_queue::TheQueueOnScreen,
            Option<smoozy_desktop::surface::the_queue::WhatAServerIsDoing>,
        ) {
            let record = self.record.record.lock().expect("the record");
            let queue = smoozy_desktop::surface::the_queue::what_is_held(
                &self.holds,
                Some(&record),
                Some(&chosen.as_text()),
            );
            let column = smoozy_desktop::surface::the_queue::what_the_servers_are_doing(&self.holds, Some(&record))
                .into_values()
                .next();
            (queue, column)
        }
    }

    /// **Every job the machine's own program holds back**, off its own queue
    /// reading.
    fn the_held_jobs_on(real: &Arc<TheRealProgram>) -> Vec<Identity> {
        let reading =
            smoozy_desktop::servers::the_machines_work::the_queue_reading_from(&mut StraightToTheProgram(Arc::clone(real)))
                .expect("the machine's own program reads its queue");
        reading
            .jobs
            .into_iter()
            .filter(|job| matches!(job.state, smoozy_queue_history::line::QueueState::Held { .. }))
            .map(|job| job.job)
            .collect()
    }

    /// **What the Queue shows of one held rest**: a row in its server's line,
    /// and *Render all* and *Cancel* on its entry — **or, while its check
    /// is still in line, the pair's one row naming both parts, whose entry
    /// answers when the rest is asked for** (`Design/Queue & History § A
    /// first-frame check and its job`, the nineteenth notes, note 7). The
    /// row names the full range only where the rest's record is held and the
    /// machine holds the rest.
    fn the_queue_shows(queue: &smoozy_desktop::surface::the_queue::TheQueueOnScreen, rest: &Identity) -> bool {
        let rest = rest.as_text();
        let rows = || queue.lines.iter().flat_map(|line| &line.rows);
        let its_own = rows().any(|row| row.job == rest)
            && queue.entry.as_ref().is_some_and(|entry| entry.job == rest && !entry.held.is_empty());
        let on_its_checks_row = queue.entry.as_ref().is_some_and(|entry| {
            entry.the_check.as_ref().is_some_and(|the_check| the_check.job == entry.job)
                && rows().any(|row| row.job == entry.job && row.the_pair.len() == 2)
        });
        its_own || on_its_checks_row
    }

    /// **Step 10c's twelfth-pass fault 5, replayed against the machine's own
    /// program: a first-frame check's held rest is an entry in Queue whenever
    /// the machine holds it** — `Rendering` S13; `Queue & History` A1, C10,
    /// H7, H9; `State, identity and recovery` V1 step 2.
    ///
    /// **The pass's order first, as the map wrote it**: a check sent, the
    /// application ended and started again on the same record, *Connect*, a
    /// second check. Both rests are in Queue, before the reopen and after it —
    /// **so the reopen is not the cause.**
    ///
    /// **Then the cause.** A P10d pass reads forward every five seconds
    /// whatever is on screen, and a first-frame check writes the rest's local
    /// record *provisional* before the rest's request goes out. **A pass that
    /// lands between the two** reads the first job's event, an event about a
    /// provisional record, and until this piece asked the machine about
    /// *every* provisional record of the server — the rest's among them. The
    /// machine truly did not hold the rest yet, said *absent*, and `State,
    /// identity and recovery` V1 step 2's branch removed the rest's record
    /// leaving no trace. A moment later the machine took the rest. **From then
    /// on the machine held a job the desktop had no record of**: the Queue
    /// dropped it at its join, the work column counted it, and nothing brought
    /// it back. The third check here puts the pass at that moment.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_first_frame_checks_held_rest_is_in_the_queue_whenever_the_machine_holds_it() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let Some(built) = TheRealProgram::the_one_built() else {
            println!(
                "SKIPPED: the machine's own program is not built beside this test. Build the \
                 workspace's tests, which build it. The held rest is unproved here."
            );
            return;
        };

        let stage = the_stage(program, "held-rest", 4, TheSupervisor::TakesTheJob);
        let real = Arc::new(TheRealProgram::started_in(&stage.layout, &built));
        *stage.machine.the_real_program.lock().expect("the program") = Some(Arc::clone(&real));
        let server = a_server_row_in(&stage.record);

        // **The first run**: *Connect*, a check, a pass.
        let first_run = ARun::connected(&stage, Arc::clone(&stage.record), &server).await;
        let rest_before = first_run.a_first_frame_check(&stage, &server, &real).await;
        first_run.a_pass(&stage, &server).await;
        let (queue, _) = first_run.the_queue(&rest_before);
        assert!(the_queue_shows(&queue, &rest_before), "the rest sent before the reopen: {queue:#?}");

        // **The reopen**: the run ends, and a new one opens the same record.
        let ARun { point, record, .. } = first_run;
        drop(point);
        drop(record);
        let record_file = stage.scratch.at.join("record").join("smoozy.sqlite3");
        let reopened = Arc::new(ARecordHere {
            record: Mutex::new(LocalRecord::open_at(&record_file).expect("the record opens again")),
            machine: Arc::clone(&stage.machine),
            seen: Mutex::new((0, 0)),
            moved: AtomicUsize::new(0),
        });

        // **The second run**: *Connect*, a second check, a pass.
        let second_run = ARun::connected(&stage, Arc::clone(&reopened), &server).await;
        let rest_after = second_run.a_first_frame_check(&stage, &server, &real).await;
        second_run.a_pass(&stage, &server).await;
        for rest in [&rest_before, &rest_after] {
            let (queue, _) = second_run.the_queue(rest);
            assert!(the_queue_shows(&queue, rest), "a rest after the reopen: {queue:#?}");
        }

        // **A third check, with a pass landing as its rest is sent.**
        {
            let (record, holds, server) = (Arc::clone(&reopened), Arc::clone(&second_run.holds), server.clone());
            let straight = Arc::clone(&real);
            *stage.machine.the_moment_the_rest_arrives.lock().expect("the moment") = Some(Box::new(move || {
                smoozy_desktop::servers::the_machines_work::a_read_forward_pass(
                    &server,
                    record.as_ref(),
                    &mut StraightToTheProgram(straight),
                    &holds,
                );
            }));
        }
        let rest_during = second_run.a_first_frame_check(&stage, &server, &real).await;
        assert!(
            stage.machine.the_moment_the_rest_arrives.lock().expect("the moment").is_none(),
            "the pass landed while the rest was sent"
        );
        second_run.a_pass(&stage, &server).await;
        let held_on_the_machine = the_held_jobs_on(&real);
        assert!(held_on_the_machine.contains(&rest_during), "the machine holds the third rest");
        // **What the person meets, said first**: whether the Queue shows the
        // third rest, how many it counts waiting, and what the server row's
        // work column says — three waiting, the line held: three first frames
        // and three rests, **each pair counted once** (`Design/Queue & History
        // § A first-frame check and its job`, the nineteenth notes, note 7;
        // until then six).
        let three_waiting = Some(smoozy_desktop::surface::the_queue::WhatAServerIsDoing::Waiting { how_many: 3 });
        for rest in [&rest_during, &rest_before, &rest_after] {
            let (queue, column) = second_run.the_queue(rest);
            let waiting_in_the_queue: u32 = queue.lines.iter().map(|line| line.waiting).sum();
            let (shows, standing) = (the_queue_shows(&queue, rest), {
                let record = reopened.record.lock().expect("the record");
                JobRecords::over(record.connection()).read(rest).expect("it reads").is_some()
            });
            assert!(
                shows && waiting_in_the_queue == 3 && column == three_waiting && standing,
                "every rest the machine holds is in Queue, and the server row's count and the \
                 Queue's agree. For {}: the Queue shows it {shows}, the Queue counts \
                 {waiting_in_the_queue} waiting, the work column says {column:?}, and its local \
                 record stands {standing}",
                rest.as_text()
            );
        }
        drop(second_run);
        drop(real);
    }

    /// A paid lock left by an old copy cannot stop a valid submission.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_paid_lock_no_longer_stops_a_submission() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "locked", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let given = what_the_page_hands_over(&stage, vec![a_payload("1-3")], false);
        let came_to = the_act(&stage, &point, given).await;
        assert!(matches!(came_to, WhatTheActCameTo::TheseJobs(ref jobs)
            if jobs == &[WhatBecameOfOneJob::ItIsInTheLine]));
        assert!(!stage.machine.what_happened().is_empty());
        let record = stage.record.record.lock().expect("the record");
        let written: i64 = record
            .connection()
            .query_row("SELECT count(*) FROM outcome_record", [], |row| row.get(0))
            .expect("counted");
        assert!(written > 0);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 10c: the two callers, joined — a submission makes the arrangement
    // and a later connected period runs it
    // ═══════════════════════════════════════════════════════════════════════

    /// One path, as the bytes the adapters take.
    fn the_bytes_of(path: &Path) -> Vec<u8> {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    }

    /// **The whole of what step 8's milestone asks for at the desk: a job
    /// submitted with *Download as it renders* on, the local record closed and
    /// opened again, and a connected period that brings the frames home with
    /// nobody asking for them.**
    ///
    /// - **`Download` R1: the arrangement is made at submission**, with the
    ///   destination and the narrowing fixed at that moment — the caller
    ///   `src/submitting.rs` owed.
    /// - **`Download` R11 and R12: it survives the app closing and resumes on a
    ///   later connection.** The record is closed and a fresh one is opened over
    ///   the same file, which is what a later run opens, and **the period that
    ///   runs it is not the one the submission was made in**.
    /// - **`Download` E1: the ending arrives, the ending walk runs over the real
    ///   file-transfer server, and the queue drains** — each frame through
    ///   `one_file_comes_down`, landing inside the job's own folder under
    ///   `Download` P10's composed name.
    /// - **`Download` W2's cadence is the crate's own**: the pass answers with
    ///   the pause it computed and this file names no second figure.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn an_arrangement_made_at_submission_is_run_by_a_later_period_and_the_frames_land() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "arrangement", 4, TheSupervisor::TakesTheJob);
        let chosen = stage.scratch.at.join("chosen");
        std::fs::create_dir_all(&chosen).expect("the folder the user chose");

        // **The submission, with the strip's switch on and its own folder.**
        let point = APointLending::over(&stage.machine).await;
        let mut given = what_the_page_hands_over(&stage, vec![a_payload("1-3")], false);
        given.the_download_as_it_renders_choice = true;
        given.where_the_downloads_go = Some(the_bytes_of(&chosen));
        given.the_formats_chosen = None;
        let server_row = given.server.local_server.clone();
        let run = given.run.clone();
        let came_to = the_act(&stage, &point, given).await;
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]),
            "{:#?}",
            stage.machine.what_happened()
        );

        let jobs = the_jobs_on_the_machine(&stage.layout);
        assert_eq!(jobs.len(), 1);
        let job = Identity::read_back(IdentityKind::Job, &jobs[0].1.job).expect("the job");
        let paths =
            AJobsPaths::composed(&stage.layout, &job, Some("Kitchen, first pass")).expect("paths");
        the_render_worker_chose_its_folder(&paths);

        // **Three frames on the machine**, written in the attempt's own order,
        // and the machine's own note saying the render started and finished.
        let mut written: Vec<Vec<u8>> = Vec::new();
        for frame in 1..=3u32 {
            let bytes: Vec<u8> = (0..(1_024 * frame as usize))
                .map(|at| (at % 251) as u8)
                .collect();
            std::fs::write(
                Path::new(&paths.job_output_folder).join(format!("frame_{frame:04}.png")),
                &bytes,
            )
            .expect("a frame");
            written.push(bytes);
        }
        the_note_says_it_finished(&stage, &job);

        // **The app closes.** The record is dropped and the arrangement is read
        // back through a record opened afresh over the same file, which is what
        // a later run opens.
        let TheStage {
            scratch,
            machine,
            record,
            layout,
            ..
        } = stage;
        drop(point);
        drop(record);
        let record_file = scratch.at.join("record").join("smoozy.sqlite3");
        let reopened = ARecordHere {
            record: Mutex::new(LocalRecord::open_at(&record_file).expect("the record opens again")),
            machine: Arc::clone(&machine),
            seen: Mutex::new((0, 0)),
            moved: AtomicUsize::new(0),
        };
        {
            let held = reopened.record.lock().expect("the record");
            let read = DownloadArrangements::over(held.connection())
                .every_arrangement_for(&server_row)
                .expect("the arrangements read");
            assert_eq!(read.len(), 1, "one job, one arrangement");
            assert_eq!(read[0].job, job);
            assert_eq!(
                read[0].destination,
                the_bytes_of(&chosen),
                "the folder the submit strip named is the one the arrangement holds"
            );
            assert!(read[0].narrowing.is_no_filter(), "no formats were chosen");
            assert!(
                read[0].reckoning.is_empty(),
                "nothing has been acted on yet"
            );
        }

        // **A later connected period.** A fresh connection to the same machine,
        // and the driving that a period starts the moment one is lent.
        let later = APointLending::over(&machine).await;
        let passes = AtomicUsize::new(0);
        let it_is_live = || passes.fetch_add(1, Ordering::SeqCst) == 0;
        let the_run = TheRunsTransfers::for_this_run();
        this_connections_arrangements_are_driven(
            &later.sessions,
            &the_run,
            &reopened,
            &server_row,
            &layout,
            &run,
            &it_is_live,
        )
        .await;
        assert_eq!(passes.load(Ordering::SeqCst), 2, "one pass, then it ended");
        // **The Transfers page is told as each frame lands**, and once more
        // where the walk first wrote the three entries the row counts — so a
        // row that is showing moves with the files rather than sitting at
        // 0 / 3 (the seventh passes, Vast fault 7 and Hyperstack fault 8).
        assert!(
            reopened.moved.load(Ordering::SeqCst) >= 3,
            "the page was told {} times for three frames",
            reopened.moved.load(Ordering::SeqCst)
        );

        // **The frames are on the user's own computer, byte for byte, inside
        // the job's own folder** — `Download` P10 and M9.
        let landed: PathBuf = std::fs::read_dir(&chosen)
            .expect("the chosen folder")
            .flatten()
            .map(|entry| entry.path())
            // **`Server protocol and transfers` S1's staging sits inside the
            // user's own destination folder**, so the job's own folder is the
            // other one — and `Server protocol and transfers` S26 has the
            // staging empty at the end.
            .find(|path| {
                path.is_dir()
                    && path
                        .file_name()
                        .is_some_and(|name| !name.to_string_lossy().starts_with('.'))
            })
            .expect("the job's own folder sits between the destination and the frames");
        assert!(
            landed
                .file_name()
                .is_some_and(|name| { name.to_string_lossy().contains(&job.as_text()) }),
            "`Download` P10: the job's identity is in the folder's name — {landed:?}"
        );
        for (at, bytes) in written.iter().enumerate() {
            let frame = landed.join(format!("frame_{:04}.png", at + 1));
            assert_eq!(
                &std::fs::read(&frame).unwrap_or_else(|_| panic!("frame {} landed", at + 1)),
                bytes,
                "frame {} did not land whole",
                at + 1
            );
        }

        // **`Download` R4: the reckoning says what Smoozy did**, durably, and
        // `Download` R9 makes each of those final.
        let held = reopened.record.lock().expect("the record");
        let read = DownloadArrangements::over(held.connection())
            .every_arrangement_for(&server_row)
            .expect("the arrangements read");
        assert_eq!(read[0].reckoning.len(), 3);
        assert!(read[0]
            .reckoning
            .iter()
            .all(|entry| entry.account.state == WhatSmoozyDid::Fetched));
        drop(held);
        drop(later);
    }

    /// **Piece 10's sharpest finding, held from its first caller: a unit whose
    /// size cannot be established stays owed, and nothing is published over the
    /// frame's own name.**
    ///
    /// **What the finding was.** `Server protocol and transfers` S9's record
    /// carries the expected total and its loop treats a total of nothing as a
    /// transfer with nothing left to do — it publishes, so an empty file lands
    /// at the frame's own name, the ending reads *published*, and the reckoning
    /// writes *fetched*, which `Download` R9 makes final. The frame would never
    /// be fetched again and the user would keep an empty file where their
    /// render was.
    ///
    /// **This is that case reached from the running application.** The job's
    /// output folder holds a link pointing at nothing, which the walk lists as
    /// a file and whose attributes no reading can take. **Nothing lands, the
    /// reckoning stays empty, and the unit is still owed** — the direction this
    /// document fails in everywhere else: a fetch that waits, never one that
    /// lands wrong.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_unit_whose_size_cannot_be_established_stays_owed_and_nothing_is_published() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "no-size", 4, TheSupervisor::TakesTheJob);
        let chosen = stage.scratch.at.join("chosen");
        std::fs::create_dir_all(&chosen).expect("the folder the user chose");

        let point = APointLending::over(&stage.machine).await;
        let mut given = what_the_page_hands_over(&stage, vec![a_payload("1")], false);
        given.the_download_as_it_renders_choice = true;
        given.where_the_downloads_go = Some(the_bytes_of(&chosen));
        let server_row = given.server.local_server.clone();
        let run = given.run.clone();
        let came_to = the_act(&stage, &point, given).await;
        assert_eq!(
            came_to,
            WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine])
        );

        let jobs = the_jobs_on_the_machine(&stage.layout);
        let job = Identity::read_back(IdentityKind::Job, &jobs[0].1.job).expect("the job");
        let paths =
            AJobsPaths::composed(&stage.layout, &job, Some("Kitchen, first pass")).expect("paths");
        the_render_worker_chose_its_folder(&paths);
        // **One name in the output folder whose attributes cannot be read.**
        std::os::unix::fs::symlink(
            Path::new(&paths.job_output_folder).join("nothing-is-here.png"),
            Path::new(&paths.job_output_folder).join("frame_0001.png"),
        )
        .expect("the link is planted");

        the_note_says_it_finished(&stage, &job);

        let later_point = point;
        let passes = AtomicUsize::new(0);
        let it_is_live = || passes.fetch_add(1, Ordering::SeqCst) == 0;
        let the_run = TheRunsTransfers::for_this_run();
        this_connections_arrangements_are_driven(
            &later_point.sessions,
            &the_run,
            stage.record.as_ref(),
            &server_row,
            &stage.layout,
            &run,
            &it_is_live,
        )
        .await;

        // **Nothing was published over the frame's own name.**
        let landed: Vec<PathBuf> = walk_everything_under(&chosen);
        assert!(
            landed.iter().all(|path| !path.is_file()),
            "a file landed where no size could be established: {landed:#?}"
        );
        // **The unit stays owed**: no terminal state was written for it, so the
        // reckoning names it at all only when something became true of it.
        let record = stage.record.record.lock().expect("the record");
        let read = DownloadArrangements::over(record.connection())
            .every_arrangement_for(&server_row)
            .expect("the arrangements read");
        assert!(
            read[0]
                .reckoning
                .iter()
                .all(|entry| entry.account.state != WhatSmoozyDid::Fetched),
            "a unit with no size was written *fetched*: {:#?}",
            read[0].reckoning
        );
        drop(record);
        drop(later_point);
    }

    /// **`Download` J9's reading for a request that reaches no job at all.**
    struct NoJobIsKnown;

    impl smoozy_download::acting::enumerating::ReadsTheLocalJobRecords for NoJobIsKnown {
        fn what_the_local_record_says(
            &self,
            _source_path: &[u8],
        ) -> smoozy_download::vouching::WhatTheLocalRecordSays {
            smoozy_download::vouching::WhatTheLocalRecordSays {
                the_file_is_inside_the_jobs_output_folder: false,
                the_recorded_ending: None,
                the_completeness_statement: None,
            }
        }
        fn the_files_frame(&self, _source_path: &[u8]) -> Option<i64> {
            None
        }
        fn the_job_whose_output_folder_this_is(
            &self,
            _picked: &[u8],
        ) -> Option<smoozy_download::acting::enumerating::AJobsOwnFolder> {
            None
        }
    }

    /// **Files' own Download, over the server's own point, holding piece 10's
    /// sharpest finding: a unit whose size cannot be established stays owed and
    /// nothing is published.**
    ///
    /// **The manual route had no such guard.** `Server protocol and transfers`
    /// S9's record carries the expected total, and its loop treats a total of
    /// nothing as a transfer with nothing left to do — it publishes. A listing
    /// that carries no size reads as nought, which is the same value a file of
    /// nought bytes gives, so an empty file would land at the user's own file's
    /// name the moment they answered *Replace*.
    ///
    /// **What this test watches, said exactly.** The case piece 10 watched —
    /// a file that has bytes and whose size was not established — **cannot be
    /// staged against OpenSSH's own server**, which sizes every regular file in
    /// its listing; the hazard is a server that leaves the protocol's optional
    /// size field out, and there is none here to borrow. **So what is watched
    /// is the rule rather than the empty file**: a unit no reading can size
    /// takes no transfer identity, takes no outcome, and stays owed.
    ///
    /// **The other half is not vacuous and is the one that could break**: a
    /// file that really is nought bytes still comes home as a file of nought
    /// bytes, so the guard refuses an absence rather than refusing nought.
    #[tokio::test]
    async fn a_browser_download_leaves_a_unit_owed_where_no_size_can_be_established() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let stage = the_stage(program, "browser-size", 4, TheSupervisor::TakesTheJob);
        let chosen = stage.scratch.at.join("chosen");
        std::fs::create_dir_all(&chosen).expect("the folder the user chose");

        // **A folder on the machine with three files in it**: one with bytes,
        // one that really is nought bytes, and one that is taken away between
        // the walk and the fetch so that no reading can size it.
        let picked = stage.scratch.at.join("home/picked");
        std::fs::create_dir_all(&picked).expect("the folder on the machine");
        let full: Vec<u8> = (0..2_048).map(|at| (at % 251) as u8).collect();
        std::fs::write(picked.join("full.png"), &full).expect("the file with bytes");
        std::fs::write(picked.join("really-empty.png"), b"").expect("the empty file");
        std::fs::write(picked.join("goes-away.png"), b"").expect("the file that goes");

        let point = APointLending::over(&stage.machine).await;
        let server_row = Identity::issue(IdentityKind::LocalServer).expect("random bytes");
        let run = Identity::issue(IdentityKind::Run).expect("random bytes");
        let door: &dyn smoozy_desktop::downloading::TheRecordADownloadWrites =
            stage.record.as_ref();

        let mut ready = smoozy_desktop::downloading::a_download_is_prepared(
            smoozy_desktop::downloading::WhatIsBeingDownloaded {
                server_row: server_row.clone(),
                installation: None,
                destination: the_bytes_of(&chosen),
                picked: vec![the_bytes_of(&picked)],
                run: run.clone(),
            },
            &point.sessions,
            door,
            &NoJobIsKnown,
        )
        .await
        .expect("the request is prepared");
        assert_eq!(ready.request.entries.len(), 3, "the walk found all three");
        // **The page is told once as the request is written** — step 10c's
        // twelfth-pass fix piece 3 — and then once per unit, below.
        assert_eq!(
            stage.record.moved.load(Ordering::SeqCst),
            1,
            "the page was not told the request was written"
        );
        let told_before_the_files = stage.record.moved.load(Ordering::SeqCst);

        // **Taken away after the enumeration and before the fetch**, which is
        // what leaves a unit the walk sized at nought and no reading can size.
        std::fs::remove_file(picked.join("goes-away.png")).expect("the file goes");

        smoozy_desktop::downloading::the_request_goes_without_a_question(&mut ready, door)
            .expect("the dispositions are written");
        let the_run = TheRunsTransfers::for_this_run();
        let came_to = smoozy_desktop::downloading::the_download_runs(
            &mut ready,
            &point.sessions,
            &the_run,
            door,
        )
        .await;

        assert_eq!(came_to.fetched, 2, "the two that could be sized came home");
        // **The Transfers page is told once per unit whose outcome was
        // written**, and never for the one that stays owed.
        assert_eq!(
            stage.record.moved.load(Ordering::SeqCst) - told_before_the_files,
            2,
            "the page was not told as each file landed"
        );
        assert_eq!(
            came_to.did_not_finish, 1,
            "the one that could not stays owed"
        );
        assert!(came_to.failed.is_empty());

        let landed = chosen.join("picked");
        assert_eq!(
            std::fs::read(landed.join("full.png")).expect("the file"),
            full
        );
        assert_eq!(
            std::fs::read(landed.join("really-empty.png")).expect("the empty file"),
            Vec::<u8>::new(),
            "a file that really is nought bytes still comes home"
        );
        assert!(
            !landed.join("goes-away.png").exists(),
            "an empty file was published where no size could be established"
        );

        // **The unit stays owed**: no transfer was arranged for it and no
        // outcome was written, so a later sitting can still fetch it.
        let owed = ready
            .request
            .entries
            .iter()
            .find(|entry| entry.source_path == the_bytes_of(&picked.join("goes-away.png")))
            .expect("the entry is in the record");
        assert!(owed.outcome.is_none(), "an owed unit took an outcome");
        assert!(
            owed.transfer.is_none(),
            "an owed unit took a transfer identity"
        );
        drop(point);
    }

    /// Every path under a folder, in no particular order.
    fn walk_everything_under(at: &Path) -> Vec<PathBuf> {
        let mut found = Vec::new();
        if let Ok(read) = std::fs::read_dir(at) {
            for entry in read.flatten() {
                found.push(entry.path());
                found.extend(walk_everything_under(&entry.path()));
            }
        }
        found
    }

    /// **The machine's own note, saying the render started and finished** —
    /// `Queue & History` N1, carried into the local job record by the fill-in
    /// `State, identity and recovery` V1 step 4 performs.
    /// **What the render worker leaves at its step 4** (`Job files and
    /// outputs` P21): the job's folder, made where it chose, and the record of
    /// where it sits beside the manifest — which is what `Download` walks from
    /// 2026-10-04, the desktop making no folder at submission. **Placed here
    /// in Smoozy's job area**, the folder every job had before; `Download`
    /// walks whatever the record names. The Creator's second notes, piece 4.
    fn the_render_worker_chose_its_folder(paths: &AJobsPaths) {
        use smoozy_job_files::where_the_folder_sits::{WhereItIs, WhereTheJobOutputFolderSits, THE_RECORD_OF_WHERE_THE_FOLDER_SITS};
        std::fs::create_dir_all(&paths.job_output_folder).expect("the job's folder");
        let sits = WhereTheJobOutputFolderSits {
            path: paths.job_output_folder.clone(),
            where_it_is: WhereItIs::WhereTheFilePoints,
        };
        std::fs::write(
            Path::new(&paths.job_record_directory).join(THE_RECORD_OF_WHERE_THE_FOLDER_SITS),
            sits.as_written(),
        )
        .expect("where the job's folder sits is recorded");
    }

    fn the_note_says_it_finished(stage: &TheStage, job: &Identity) {
        let record = stage.record.record.lock().expect("the record");
        let on_the_machines_clock = |at| {
            smoozy_state::observed::Reading::taken(
                (),
                at,
                smoozy_state::observed::Clock::AMachine(smoozy_state::observed::MachineClock(
                    "the stand-in machine".to_string(),
                )),
            )
        };
        let note = smoozy_queue_history::acting::note::TheNote {
            started: Some(on_the_machines_clock(std::time::SystemTime::now())),
            ended: Some(on_the_machines_clock(std::time::SystemTime::now())),
            ending: Some(smoozy_queue_history::acting::note::TheEndingOnTheNote {
                outcome: smoozy_queue_history::acting::note::TheOutcomeAsWritten::Finished,
                exit_status: Some(0),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                the_device_line_as_observed: None,
            }),
            attempts: 1,
            ..smoozy_queue_history::acting::note::TheNote::created_with_the_job_record(
                "Kitchen, first pass",
            )
        };
        JobRecords::over(record.connection())
            .the_fill_in(job, &note)
            .expect("the note's half is carried over");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 10c's thirteenth-pass fix piece 2: a removed server's held jobs —
    // the pass's order replayed against the machine's own program
    // ═══════════════════════════════════════════════════════════════════════

    /// **The thirteenth pass on Hyperstack, at the desk, up to the card that
    /// *Set up this server* shows.** Two first-frame checks on one row, each
    /// rest held; the row removed with *Leave it running*, which sends
    /// nothing; the same point added again as a new row; and *Set up this
    /// server*'s step 4b asking the machine's own program what it holds.
    ///
    /// **The held rests block the commit, and nothing in this copy can answer
    /// them** (`Rented-server runtime` G21 and `Rented-server runtime` G23; `State, identity and
    /// recovery` Y9): the removal keeps their local records under the removed
    /// row, so the new row's offer never names them (the piece's proof, in
    /// `Audits/Build/Step 10c/Thirteenth-pass fix piece 2 — what landed.md`).
    /// **So the card says how many, and that Smoozy on this computer cannot
    /// answer them** — `Design/Server Setup § When work on the server holds the
    /// setup back`. Until this piece it said *work on the server still needs
    /// the one there now*, a reason that would never go.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_removed_rows_held_rests_are_counted_on_the_new_rows_setup_card() {
        use smoozy_runtime::package::retry::{whether_the_commit_may_proceed, AcceptedWorkStanding, WhetherTheCommitMayProceed};
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();
        let Some(built) = TheRealProgram::the_one_built() else {
            println!(
                "SKIPPED: the machine's own program is not built beside this test. Build the \
                 workspace's tests, which build it. The removed row's held rests are unproved here."
            );
            return;
        };

        let stage = the_stage(program, "removed-rests", 4, TheSupervisor::TakesTheJob);
        let real = Arc::new(TheRealProgram::started_in(&stage.layout, &built));
        *stage.machine.the_real_program.lock().expect("the program") = Some(Arc::clone(&real));
        let old_row = a_server_row_in(&stage.record);

        // **Two checks on the first row**, each rest held.
        let first_run = ARun::connected(&stage, Arc::clone(&stage.record), &old_row).await;
        let rest_d = first_run.a_first_frame_check(&stage, &old_row, &real).await;
        let rest_e = first_run.a_first_frame_check(&stage, &old_row, &real).await;
        let ARun { point, .. } = first_run;
        drop(point);

        // ***Remove*, *Leave it running*: the local forget, nothing sent.**
        {
            let record = stage.record.record.lock().expect("the record");
            smoozy_desktop::record::server_rows::remove_walking_the_enumeration(
                record.connection(),
                &old_row.as_text(),
            )
            .expect("the row is removed");
        }
        let held = the_held_jobs_on(&real);
        assert!(
            held.contains(&rest_d) && held.contains(&rest_e),
            "the machine still holds both rests after the removal"
        );

        // **The same point added again, and *Set up this server*'s step 4b.**
        let _new_row = a_server_row_in(&stage.record);
        let point = APointLending::over(&stage.machine).await;
        let read = smoozy_desktop::setting_up::what_the_program_there_has_accepted(
            &point.sessions,
            &stage.layout.area(Area::Package),
        )
        .await;
        drop(point);
        let held_read = read
            .iter()
            .filter(|(_, standing)| *standing == AcceptedWorkStanding::Held)
            .count();
        assert_eq!(held_read, 2, "the program's reading holds both rests: {read:?}");
        let WhetherTheCommitMayProceed::Deferred { because } = whether_the_commit_may_proceed(&read) else {
            panic!("the commit was not deferred over the held rests: {read:?}");
        };

        // **What the card says.**
        let came_to = smoozy_desktop::setting_up::what_a_placement_not_verified_came_to(
            smoozy_server_protocol::placement::performing::WhatThePlacementReached::Deferred { because },
            &read,
        );
        let said = smoozy_desktop::surface::set_up::what_it_came_to(&came_to).said;
        assert!(
            said.starts_with(&format!(
                "Setup stopped. Smoozy's server software cannot be replaced: {} from an earlier \
                 setup need an answer this computer cannot give",
                smoozy_desktop::typography::a_figure_with_its_unit(2, "jobs")
            )),
            "the card said: {said}"
        );
        drop(real);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Step 10c's RunPod-pass fix piece 1: a server whose first setup was
    // deferred is not set up, and the page says so before anything is sent
    // ═══════════════════════════════════════════════════════════════════════

    /// **The pass before Vast on RunPod, fault 1, at the desk.** *RunPod two*
    /// was added again over a machine holding an earlier setup's held job, and
    /// *Set up this server* was deferred. **Its hand-over had already recorded
    /// the root** (`Job files and outputs` L1, step 3), and the placement that
    /// confirms an installation (step 5) never committed — the pass's own
    /// local record holds exactly that row: a root, read under the deferred
    /// attempt's installation, and no confirmed installation. **Every
    /// *Render* then composed an act, and the act stopped at its first step**
    /// (the remap material's placement asks for a confirmed installation), so
    /// the page said *Smoozy could not put the job's files on the server.*
    /// about a server nothing had been sent to.
    ///
    /// **A server with no installation this copy can use is not set up**, and
    /// the page says the words it already says for one: *Not submitted.
    /// This server is not set up.* — before anything is sent.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_server_whose_first_setup_was_deferred_is_refused_before_anything_is_sent() {
        let _deadline = TheTestsDeadline::started();
        let program = the_server_or_say_so!();

        let stage = the_stage(program, "deferred-first-setup", 1, TheSupervisor::TakesTheJob);
        let point = APointLending::over(&stage.machine).await;
        let (held, file, mut row) = the_page_after_a_read(&stage, the_settings_half_of_a_one_scene_file());
        // **The row as the deferred setup left it**: the root recorded under
        // the attempt's installation, and that installation never confirmed.
        if let Some(root) = row.recorded_machine_root.as_mut() {
            root.read_under = Identity::issue(IdentityKind::Installation).expect("random bytes").as_text();
        }
        row.last_confirmed_installation_identity = None;
        let compose = |row: &smoozy_servers::store::ServerRow| {
            smoozy_desktop::surface::render_settings::the_act_as_the_page_composed_it(
                &held,
                &row.local_server_identifier,
                &file,
                &smoozy_desktop::surface::render_settings::WhatTheRecordHolds::default(),
                row,
                Identity::issue(IdentityKind::Run).expect("random bytes"),
            )
        };

        match compose(&row) {
            Ok(mut given) => {
                // **The fault, as the pass met it**: the act the page composed,
                // sent as it was composed.
                given.the_temporary_area = Some(stage.scratch.at.join("desk-temporary"));
                let came_to = the_act(&stage, &point, given).await;
                panic!(
                    "the page composed an act for a server with no installation confirmed; sent, it \
                     came to {came_to:?} and the page said {:?}",
                    smoozy_desktop::surface::render_settings::what_the_act_says(&came_to)
                );
            }
            Err(said) => {
                // **The words the page already says for a server never set
                // up** — no root recorded at all.
                let mut never = row.clone();
                never.recorded_machine_root = None;
                let never_set_up = compose(&never).err().expect("a server never set up is refused");
                assert_eq!(said, never_set_up, "the same words as for a server never set up");
            }
        }
        assert!(
            the_jobs_on_the_machine(&stage.layout).is_empty(),
            "nothing reached the machine"
        );
        drop(point);
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

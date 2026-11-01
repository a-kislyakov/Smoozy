//! **Step 10c's dependency wave piece 5 at the desk: *Check dependencies…* run
//! whole, through the server's own point, onto the real local record.**
//!
//! **What the far end is.** A machine standing in for a rental, in the shape
//! `tests/a_job_is_submitted_through_the_servers_own_point.rs` built: every
//! file session is **OpenSSH's own `sftp-server`** over a Unix socket pair, so
//! the entry point's link, the four values' link and every captured stream
//! read back run against the program a rental's own file-transfer server is.
//! **What it is not is an SSH connection.**
//!
//! **The supervisor is a stand-in for piece 2's file-inspection worker**, and
//! it is named as one wherever it answers: it takes *a reading of a `.blend`*,
//! answers *still in hand* at the first ask and the recorded outcome at the
//! next, and between the two it does what the worker does — writes the check's
//! own script into the operation's folder, starts Blender by the build's own
//! path with the check's own line (`smoozy_dependencies_machine::check_script::
//! the_words_of_the_check_reading`, written out below because this crate does
//! not reach that one), and keeps both streams. **The real program, the real
//! worker and real Blender were proved together by piece 2**
//! (`a_real_blender_reads_a_real_file_through_the_real_program`); what this
//! file proves is everything after the answer.
//!
//! **The record is the application's own**, opened at its real settings in a
//! throwaway folder.
//!
//! **Two tests.** The desk's three real files are ignored, because they need
//! the desk's Blenders and the splash tester's folder; run them with
//! `SMOOZY_DESK_WORK` set to that folder. The cut line runs everywhere
//! `sftp-server` does, from a scripted reading.
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
         this system ({}) has none of. The check at the desk is unproved here.",
        std::env::consts::OS
    );
}

#[cfg(unix)]
mod on_a_desk_with_a_socket_pair {
    use std::collections::BTreeMap;
    use std::os::unix::ffi::OsStrExt;
    use std::path::{Path, PathBuf};
    use std::pin::Pin;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

    use smoozy_dependencies::bytes::{MachineLocation, StoredLocation};
    use smoozy_dependencies::record::{HoldingFile, MachineKey};
    use smoozy_dependencies::rows::RowState;
    use smoozy_dependencies::search::SearchedFolderList;
    use smoozy_dependencies::standing::Completion;
    use smoozy_desktop::checking_a_file::{
        the_check_runs, TheCheckCameTo, TheSittingIsOf, TheUsersOwnComputer,
    };
    use smoozy_desktop::reading_the_file::ThisServersMachine;
    use smoozy_desktop::record::dependency_records::{
        the_searched_folder_list, TheDependencyRecord,
    };
    use smoozy_desktop::record::LocalRecord;
    use smoozy_desktop::servers::sessions::{
        lend_until_it_ends, AHeldConnection, NoSession, TheAdditionalConnection,
        ThisServersSessions,
    };
    use smoozy_desktop::surface::the_dependency_check::{the_panel, WhatTheRunHolds};
    use smoozy_render_settings::file_half::BlenderIdentity;
    use smoozy_server_protocol::framing::{
        read_one_request, write_result_frame, Answered, Fields,
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
    };
    use smoozy_server_protocol::placement::THE_HEALTH_EXCHANGE_COMMAND;
    use smoozy_server_protocol::requests as asked;
    use smoozy_server_protocol::transport::{CommandResult, CommandString, Ending};
    use smoozy_server_setup::blender::Version;
    use smoozy_state::identities::{Identity, IdentityKind};

    // --------------------------------------- where OpenSSH's own server is

    const WHERE_SFTP_SERVER_LIVES: &[&str] = &[
        "/usr/libexec/sftp-server",
        "/usr/lib/openssh/sftp-server",
        "/usr/libexec/openssh/sftp-server",
        "/usr/lib/ssh/sftp-server",
    ];

    fn the_sftp_server() -> Option<PathBuf> {
        WHERE_SFTP_SERVER_LIVES
            .iter()
            .map(PathBuf::from)
            .find(|path| path.exists())
    }

    // --------------------------------------------------- the throwaway folder

    struct Scratch {
        at: PathBuf,
    }

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let at = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
                .join(format!("smzy-check-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&at);
            std::fs::create_dir_all(&at).expect("the throwaway folder is made");
            Scratch {
                at: std::fs::canonicalize(&at).expect("the folder is there"),
            }
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.at);
        }
    }

    fn bytes_of(path: &Path) -> Vec<u8> {
        path.as_os_str().as_bytes().to_vec()
    }

    // ----------------------------------------------- a machine, standing in

    /// **How the stand-in worker takes a reading.**
    enum HowItReads {
        /// Real Blender, by the build's own path, with the check's own line.
        WithBlender,
        /// A document per `.blend`, printed between the check's two marks.
        FromAScript(BTreeMap<Vec<u8>, Vec<u8>>),
    }

    /// One reading as the stand-in worker kept it.
    struct AKeptReading {
        ordinary: Vec<u8>,
        ordinary_length: usize,
        complaints: Vec<u8>,
        complaints_length: usize,
        status: Option<i32>,
    }

    struct AMachine {
        sftp_server: PathBuf,
        home: PathBuf,
        package_area: PathBuf,
        how_it_reads: HowItReads,
        kept: Mutex<BTreeMap<String, AKeptReading>>,
        /// **After this many readings the line goes**, and nothing reaches the
        /// machine again.
        the_line_goes_after: Option<usize>,
        gone: AtomicBool,
        readings_taken: AtomicUsize,
        /// **Every folder asked to be made**, by its path — a `MKDIR`, read
        /// off the requests as they are sent. Added 2026-09-30, step 10c's
        /// eleventh-pass fix piece 4, so a test can say a send asks for each
        /// mirror folder once and not once per file.
        folders_asked: Mutex<Vec<Vec<u8>>>,
        /// **The contents readings asked of it**, counted — the 38's piece 7.
        digests_taken: AtomicUsize,
        /// **Every operation a stop named**, in order — the 38's piece 7.
        stops_asked: Mutex<Vec<String>>,
        /// **Every fixing fails while this holds** — the 38's piece 7, for
        /// `Dependency Upload & Remap` M16a's *Try again*.
        fixings_fail: AtomicBool,
        /// **Every byte written by a file session**, counted off the `WRITE`
        /// requests as they are sent — the 38's piece 7, so a send that
        /// carries on can be told from one that starts over.
        bytes_written: std::sync::atomic::AtomicU64,
    }

    impl AMachine {
        /// **One fixing, as the machine's worker runs it**: with real Blender,
        /// M14's own script and words; scripted, the file copied to the
        /// staging path and every pair reported applied.
        fn a_fixing_is_made(&self, operation: &str, fields: &Fields) -> AKeptReading {
            let build = fields.value(asked::THE_BUILD_FIELD).expect("the build").to_vec();
            let blend = fields.value(asked::THE_BLEND_FIELD).expect("the blend").to_vec();
            let staging = fields.value(asked::THE_FIXINGS_STAGING_FIELD).expect("the staging").to_vec();
            let pairs: Vec<smoozy_dependencies::remap::Pair> = serde_json::from_slice(
                fields.value(asked::THE_FIXINGS_PAIRS_FIELD).expect("the pairs"),
            )
            .expect("the pairs read");
            let folder = self.package_area.join("captured-streams").join(operation);
            std::fs::create_dir_all(&folder).expect("the operation's folder");
            let (ordinary, complaints, status) = match &self.how_it_reads {
                HowItReads::WithBlender => {
                    let script = folder.join("fixing.py");
                    std::fs::write(
                        &script,
                        smoozy_dependencies::acting::fixing_worker::the_fixing_python(&pairs, &staging),
                    )
                    .expect("the script is written");
                    let ran = std::process::Command::new(std::ffi::OsStr::from_bytes(&build))
                        .args(
                            smoozy_dependencies::acting::fixing_worker::the_words_of_the_fixing(
                                &blend,
                                &bytes_of(&script),
                            )
                            .iter()
                            .map(|word| std::ffi::OsStr::from_bytes(word)),
                        )
                        .output()
                        .expect("Blender starts by its own path");
                    (ran.stdout, ran.stderr, ran.status.code())
                }
                HowItReads::FromAScript(_) if self.fixings_fail.load(Ordering::SeqCst) => {
                    (Vec::new(), b"Error: cannot read file\n".to_vec(), Some(1))
                }
                HowItReads::FromAScript(_) => {
                    std::fs::copy(std::ffi::OsStr::from_bytes(&blend), std::ffi::OsStr::from_bytes(&staging))
                        .expect("the staged copy");
                    let mut said = format!(
                        "{}\n",
                        smoozy_dependencies::acting::fixing_worker::THE_FIXINGS_START_MARK
                    )
                    .into_bytes();
                    for pair in &pairs {
                        let line = serde_json::json!({
                            "kind": pair.kind, "setting": pair.setting, "match_text": pair.match_text,
                            "applied": true, "data_blocks": 1, "assignment_alone": "Established",
                            "reloaded": false, "values_found": []
                        });
                        said.extend_from_slice(
                            format!(
                                "{}{}\n",
                                smoozy_dependencies::acting::material::THE_PER_PAIR_HEAD,
                                line
                            )
                            .as_bytes(),
                        );
                    }
                    (said, Vec::new(), Some(0))
                }
            };
            let (at_ordinary, at_complaints) = (folder.join("ordinary"), folder.join("complaints"));
            std::fs::write(&at_ordinary, &ordinary).expect("the ordinary stream is kept");
            std::fs::write(&at_complaints, &complaints).expect("the complaints stream is kept");
            AKeptReading {
                ordinary: bytes_of(&at_ordinary),
                ordinary_length: ordinary.len(),
                complaints: bytes_of(&at_complaints),
                complaints_length: complaints.len(),
                status,
            }
        }

        fn a_connection(self: &Arc<Self>) -> AConnectionToIt {
            AConnectionToIt {
                machine: Arc::clone(self),
            }
        }

        /// **The stand-in worker's run**: the script into the operation's own
        /// folder, the build started by its own path, both streams kept.
        fn a_reading_is_taken(&self, operation: &str, build: &[u8], blend: &[u8]) -> AKeptReading {
            let folder = self.package_area.join("captured-streams").join(operation);
            std::fs::create_dir_all(&folder).expect("the operation's folder");
            let (ordinary, complaints, status) = match &self.how_it_reads {
                HowItReads::WithBlender => {
                    let script = folder.join("reading.py");
                    std::fs::write(
                        &script,
                        smoozy_dependencies::acting::reading_script::the_check_readings_script(),
                    )
                    .expect("the script is written");
                    let ran = std::process::Command::new(std::ffi::OsStr::from_bytes(build))
                        .arg("-b")
                        .arg("--factory-startup")
                        .arg(std::ffi::OsStr::from_bytes(blend))
                        .arg("--python-exit-code")
                        .arg("1")
                        .arg("--python")
                        .arg(&script)
                        .output()
                        .expect("Blender starts by its own path");
                    (ran.stdout, ran.stderr, ran.status.code())
                }
                HowItReads::FromAScript(documents) => match documents.get(blend) {
                    Some(document) => (
                        [
                            b"Blender quit\n".as_slice(),
                            smoozy_dependencies::acting::blender::THE_OPENING_SENTINEL,
                            document,
                            smoozy_dependencies::acting::blender::THE_CLOSING_SENTINEL,
                            b"\n",
                        ]
                        .concat(),
                        Vec::new(),
                        Some(0),
                    ),
                    None => (Vec::new(), b"Error: cannot read file\n".to_vec(), Some(1)),
                },
            };
            let (at_ordinary, at_complaints) = (folder.join("ordinary"), folder.join("complaints"));
            std::fs::write(&at_ordinary, &ordinary).expect("the ordinary stream is kept");
            std::fs::write(&at_complaints, &complaints).expect("the complaints stream is kept");
            AKeptReading {
                ordinary: bytes_of(&at_ordinary),
                ordinary_length: ordinary.len(),
                complaints: bytes_of(&at_complaints),
                complaints_length: complaints.len(),
                status,
            }
        }

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
            // **`Render Settings` Q14 item 2's reading of the four values**,
            // answered from the file the planted link names, in GNU `stat`'s
            // four fields.
            if let Some(link) = command.as_str().strip_prefix("stat -L -c '%s %i %Y %Z' ~/") {
                use std::os::unix::fs::MetadataExt;
                let Ok(found) = std::fs::metadata(self.home.join(link)) else {
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
            let Some(Ok(request)) = frame.map(|frame| read_one_request(&mut &frame[..])) else {
                return nothing;
            };
            let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0");
            // **The 38's piece 7: the contents reading**, digested here by the
            // desktop's own digest, which is the function the machine's own
            // program computes (`Server protocol and transfers` S23).
            if request.name == asked::THE_DIGEST_OVER_A_BYTE_RANGE_REQUEST {
                let file = request.fields.value(asked::THE_DIGESTED_FILE_FIELD).expect("the file");
                self.digests_taken.fetch_add(1, Ordering::SeqCst);
                let fields = match std::fs::read(std::ffi::OsStr::from_bytes(file)) {
                    Ok(bytes) => Fields::none()
                        .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                        .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::IT_WAS_DIGESTED)
                        .and(
                            asked::THE_DIGEST_FIELD,
                            smoozy_server_protocol::transfers::integrity::Digest::of(&bytes).as_text(),
                        )
                        .and(asked::THE_BYTES_DIGESTED_FIELD, bytes.len().to_string()),
                    Err(why) if why.kind() == std::io::ErrorKind::NotFound => Fields::none()
                        .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                        .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::NOTHING_IS_THERE),
                    Err(_) => Fields::none()
                        .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                        .and(asked::THE_DIGESTS_OUTCOME_FIELD, asked::IT_WAS_NOT_DIGESTED),
                };
                return CommandResult {
                    output: write_result_frame(&answered.carrying(fields)),
                    complaint: Vec::new(),
                    status: Some(0),
                };
            }
            // **The 38's piece 7: the fixing**, the way the machine's own
            // worker runs it — M14's script, composed by the same function
            // the program composes it with, and the build started by its own
            // path with M14's words. **A scripted machine copies the file to
            // the staging path and reports every pair applied.**
            if request.name == asked::THE_FIXED_COPY_IS_PRODUCED_REQUEST {
                let operation = request.identity.as_text();
                let mut kept = self.kept.lock().expect("the kept readings");
                if !kept.contains_key(&operation) {
                    let fixing = self.a_fixing_is_made(&operation, &request.fields);
                    kept.insert(operation.clone(), fixing);
                }
                let reading = &kept[&operation];
                let mut fields = Fields::none()
                    .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                    .and(asked::THE_ORDINARY_STREAM_FIELD, reading.ordinary.clone())
                    .and(asked::THE_ORDINARY_STREAM_LENGTH_FIELD, reading.ordinary_length.to_string())
                    .and(asked::THE_COMPLAINTS_STREAM_FIELD, reading.complaints.clone())
                    .and(asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD, reading.complaints_length.to_string())
                    .and(asked::THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD, asked::THE_ANSWER_IS_NO);
                if let Some(status) = reading.status {
                    fields = fields.and(asked::THE_EXIT_STATUS_FIELD, status.to_string());
                }
                return CommandResult {
                    output: write_result_frame(&answered.carrying(fields)),
                    complaint: Vec::new(),
                    status: Some(0),
                };
            }
            // **The 38's piece 7: a stop naming a reading**, taken and
            // written down; the stand-in's readings run to their end at the
            // first ask, so there is nothing left to interrupt.
            if request.name == asked::THE_STOP_REQUEST {
                self.stops_asked.lock().expect("the stops").push(
                    request
                        .fields
                        .text(asked::THE_OPERATION_TO_STOP_FIELD)
                        .unwrap_or_default()
                        .to_string(),
                );
                return CommandResult {
                    output: write_result_frame(&answered.carrying(
                        Fields::none().and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND),
                    )),
                    complaint: Vec::new(),
                    status: Some(0),
                };
            }
            if request.name != asked::THE_READING_OF_A_BLEND_REQUEST {
                return CommandResult {
                    output: write_result_frame(
                        &answered.saying_it_does_not_answer_that_request(&request.name),
                    ),
                    complaint: Vec::new(),
                    status: Some(0),
                };
            }
            let operation = request.identity.as_text();
            let mut kept = self.kept.lock().expect("the kept readings");
            let Some(reading) = kept.get(&operation) else {
                // **The first ask starts the worker**, and the line may go
                // with it.
                let taken = self.readings_taken.fetch_add(1, Ordering::SeqCst);
                if self.the_line_goes_after.is_some_and(|after| taken >= after) {
                    self.gone.store(true, Ordering::SeqCst);
                    return nothing;
                }
                let build = request
                    .fields
                    .value(asked::THE_BUILD_FIELD)
                    .expect("the build")
                    .to_vec();
                let blend = request
                    .fields
                    .value(asked::THE_BLEND_FIELD)
                    .expect("the blend")
                    .to_vec();
                kept.insert(
                    operation.clone(),
                    self.a_reading_is_taken(&operation, &build, &blend),
                );
                return CommandResult {
                    output: write_result_frame(
                        &answered.carrying(
                            Fields::none()
                                .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_IS_STILL_IN_HAND),
                        ),
                    ),
                    complaint: Vec::new(),
                    status: Some(0),
                };
            };
            let mut fields = Fields::none()
                .and(
                    asked::WHAT_BECAME_OF_IT_FIELD,
                    asked::IT_HAS_A_RECORDED_OUTCOME,
                )
                .and(asked::THE_ORDINARY_STREAM_FIELD, reading.ordinary.clone())
                .and(
                    asked::THE_ORDINARY_STREAM_LENGTH_FIELD,
                    reading.ordinary_length.to_string(),
                )
                .and(
                    asked::THE_COMPLAINTS_STREAM_FIELD,
                    reading.complaints.clone(),
                )
                .and(
                    asked::THE_COMPLAINTS_STREAM_LENGTH_FIELD,
                    reading.complaints_length.to_string(),
                );
            if let Some(status) = reading.status {
                fields = fields.and(asked::THE_EXIT_STATUS_FIELD, status.to_string());
            }
            CommandResult {
                output: write_result_frame(&answered.carrying(fields)),
                complaint: Vec::new(),
                status: Some(0),
            }
        }
    }

    struct AConnectionToIt {
        machine: Arc<AMachine>,
    }

    struct ASession {
        stream: tokio::net::UnixStream,
        child: std::process::Child,
        /// The machine this session is on, for its request count.
        machine: Arc<AMachine>,
        /// What was sent and not yet read as a whole packet.
        sent: Vec<u8>,
    }

    impl ASession {
        /// **The folders asked for in what was sent** — each packet a
        /// four-byte length, its type, its id, then a path's length and bytes.
        fn count_what_was_sent(&mut self, bytes: &[u8]) {
            const MKDIR: u8 = 14;
            const WRITE: u8 = 6;
            self.sent.extend_from_slice(bytes);
            let mut at = 0usize;
            while self.sent.len() >= at + 5 {
                let length =
                    u32::from_be_bytes(self.sent[at..at + 4].try_into().expect("four")) as usize;
                if length == 0 || self.sent.len() < at + 4 + length {
                    break;
                }
                // **A `WRITE`**: id, handle, offset, then the data's length.
                if self.sent[at + 4] == WRITE && length >= 13 {
                    let handle_length =
                        u32::from_be_bytes(self.sent[at + 9..at + 13].try_into().expect("four")) as usize;
                    let data_at = at + 13 + handle_length + 8;
                    if data_at + 4 <= at + 4 + length {
                        let data_length =
                            u32::from_be_bytes(self.sent[data_at..data_at + 4].try_into().expect("four"));
                        self.machine
                            .bytes_written
                            .fetch_add(u64::from(data_length), Ordering::SeqCst);
                    }
                }
                if self.sent[at + 4] == MKDIR && length >= 9 {
                    let path_length =
                        u32::from_be_bytes(self.sent[at + 9..at + 13].try_into().expect("four"))
                            as usize;
                    let path =
                        self.sent[at + 13..(at + 13 + path_length).min(at + 4 + length)].to_vec();
                    self.machine
                        .folders_asked
                        .lock()
                        .expect("the folders asked")
                        .push(path);
                }
                at += 4 + length;
            }
            self.sent.drain(..at);
        }
    }

    impl Drop for ASession {
        fn drop(&mut self) {
            let _ = self.child.kill();
            let _ = self.child.wait();
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
            let wrote = Pin::new(&mut self.stream).poll_write(cx, buf);
            if let Poll::Ready(Ok(written)) = wrote {
                self.count_what_was_sent(&buf[..written]);
            }
            wrote
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
            if self.machine.gone.load(Ordering::SeqCst) {
                return Err(NoSession::TheConnectionIsGone);
            }
            let (ours, theirs) = std::os::unix::net::UnixStream::pair().expect("a socket pair");
            ours.set_nonblocking(true).expect("the end we drive");
            let reading = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let writing = unsafe { std::process::Stdio::from_raw_fd(copy_of(theirs.as_raw_fd())) };
            let child = std::process::Command::new(&self.machine.sftp_server)
                .current_dir(&self.machine.home)
                .stdin(reading)
                .stdout(writing)
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("the machine's own file-transfer server starts");
            drop(theirs);
            Ok(ASession {
                stream: tokio::net::UnixStream::from_std(ours).expect("the end we drive"),
                child,
                machine: Arc::clone(&self.machine),
                sent: Vec::new(),
            })
        }

        async fn a_command(
            &mut self,
            command: CommandString,
            frame: Option<Vec<u8>>,
        ) -> Result<CommandResult, NoSession> {
            if self.machine.gone.load(Ordering::SeqCst) {
                return Err(NoSession::TheConnectionIsGone);
            }
            Ok(self.machine.the_command_is_answered(command, frame))
        }

        async fn until_it_ends(&mut self) -> Ending {
            std::future::pending().await
        }

        async fn torn_down(self) -> Ending {
            Ending::TornDownBySmoozy
        }
    }

    /// **The point, lending the stand-in's connection.**
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
                Box::pin(async move { Some(more.a_connection()) })
            });
            let mut primary = machine.a_connection();
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

    /// **The application's own record at its real settings**, one short act at
    /// a time.
    struct TheRecord(Mutex<LocalRecord>);

    impl smoozy_desktop::uploading::TheRecordAnUploadWrites for TheRecord {
        fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
            let record = self.0.lock().expect("the record");
            act(record.connection());
            true
        }
    }

    // ------------------------------------------------------------ one sitting

    struct WhatTheSittingLeft {
        came_to: TheCheckCameTo,
        held: WhatTheRunHolds,
        record: TheRecord,
        sitting: TheSittingIsOf,
        read: usize,
    }

    /// **One whole sitting over one `.blend`**, through the point, onto the
    /// record, with the user's search folders as given.
    fn a_sitting_over(
        scratch: &Scratch,
        blend: &Path,
        with: BlenderIdentity,
        how_it_reads: HowItReads,
        the_line_goes_after: Option<usize>,
        search_folders: Vec<Vec<u8>>,
    ) -> Option<WhatTheSittingLeft> {
        let sftp_server = the_sftp_server()?;
        let home = scratch.at.join("home");
        let package_area = home.join("area");
        std::fs::create_dir_all(&package_area).expect("the package area");
        let machine = Arc::new(AMachine {
            sftp_server,
            home,
            package_area: package_area.clone(),
            how_it_reads,
            kept: Mutex::new(BTreeMap::new()),
            the_line_goes_after,
            gone: AtomicBool::new(false),
            readings_taken: AtomicUsize::new(0),
            folders_asked: Mutex::new(Vec::new()),
            digests_taken: AtomicUsize::new(0),
            stops_asked: Mutex::new(Vec::new()),
            fixings_fail: AtomicBool::new(false),
            bytes_written: std::sync::atomic::AtomicU64::new(0),
        });
        let record = TheRecord(Mutex::new(
            LocalRecord::open_at(&scratch.at.join("record.sqlite3")).expect("the record opens"),
        ));
        {
            let opened = record.0.lock().expect("the record");
            the_searched_folder_list::write(
                opened.connection(),
                &SearchedFolderList {
                    folders: search_folders,
                },
            )
            .expect("the search folders are written");
        }
        let machine_key = MachineKey::of(
            Identity::issue(IdentityKind::LocalServer).expect("randomness"),
            Identity::issue(IdentityKind::Installation).expect("randomness"),
        );
        let sitting = TheSittingIsOf {
            server: machine_key.server_row.as_text(),
            file: String::from_utf8(bytes_of(blend)).expect("the path is text"),
            root: MachineLocation::composed(bytes_of(blend)),
            // **`File Browser` N6's answer, as an upload from this very place
            // would have written it** — the machine here is this Mac.
            holding: HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(
                &bytes_of(blend),
            )),
            with,
            machine: machine_key,
            layout: None,
        };
        let held = Mutex::new(WhatTheRunHolds::default());
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("a runtime");
        let came_to = runtime.block_on(async {
            let point = APointLending::over(&machine).await;
            let mut on_the_machine = ThisServersMachine {
                sessions: &point.sessions,
                package_area: package_area.to_str().expect("text"),
                server: &sitting.server,
            };
            the_check_runs(
                &mut on_the_machine,
                &record,
                &TheUsersOwnComputer,
                &held,
                &sitting,
                &|| {},
            )
            .await
        });
        Some(WhatTheSittingLeft {
            came_to,
            held: held.into_inner().expect("the run"),
            record,
            sitting,
            read: machine.readings_taken.load(Ordering::SeqCst),
        })
    }

    /// Every row, as (state, what the file records, where it was found).
    fn the_rows(left: &WhatTheSittingLeft) -> Vec<(RowState, String, Option<String>)> {
        let key = format!("{}\u{1f}{}", left.sitting.server, left.sitting.file);
        left.held.sittings[&key]
            .rows
            .iter()
            .map(|row| {
                (
                    row.state,
                    String::from_utf8_lossy(row.row.ordering_key().as_bytes()).to_string(),
                    row.the_answer()
                        .and_then(|answer| answer.resolution.as_ref())
                        .map(|resolution| {
                            String::from_utf8_lossy(resolution.to.location().as_bytes()).to_string()
                        }),
                )
            })
            .collect()
    }

    fn the_standing(left: &WhatTheSittingLeft) -> smoozy_dependencies::standing::CheckStanding {
        let record = left.record.0.lock().expect("the record");
        TheDependencyRecord::over(record.connection())
            .the_standing(&left.sitting.root, &left.sitting.machine)
            .expect("the standing reads")
            .expect("a standing was written")
    }

    // ------------------------------------------------------------ the cut line

    /// **`Dependency Upload & Remap` G16, through the server's own point: the
    /// line goes below the root, the sitting stands with what was found, the
    /// panel says it is incomplete, and the standing is written incomplete.**
    #[test]
    fn a_line_that_goes_mid_check_leaves_the_sitting_standing_and_says_it_is_incomplete() {
        let scratch = Scratch::new("cut");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("lib")).expect("the project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        std::fs::write(project.join("lib/props.blend"), b"library").expect("the library");
        std::fs::write(project.join("tex/wood.png"), b"wood").expect("a picture");
        let document = serde_json::json!({
            "references": [
                {"kind": "Library", "datablock": "props.blend", "setting": "filepath",
                 "recorded": b"//lib/props.blend".to_vec(), "scenes": {"Established": ["Scene"]}, "source": null},
                {"kind": "Image", "datablock": "wood.png", "setting": "filepath",
                 "recorded": b"//tex/wood.png".to_vec(), "scenes": {"Established": ["Scene"]}, "source": "FILE"},
                {"kind": "Image", "datablock": "gone.png", "setting": "filepath",
                 "recorded": b"//tex/gone.png".to_vec(), "scenes": {"Established": ["Scene"]}, "source": "FILE"}
            ],
            "flat_list": [], "weak_references": [], "observed_stand_ins": []
        });
        let mut documents = BTreeMap::new();
        documents.insert(
            bytes_of(&root),
            serde_json::to_vec(&document).expect("it writes"),
        );
        let Some(left) = a_sitting_over(
            &scratch,
            &root,
            BlenderIdentity {
                path: "/root/blender/blender-5.2.2/blender".to_string(),
                version: Version {
                    major: 5,
                    minor: 2,
                    patch: 2,
                },
            },
            HowItReads::FromAScript(documents),
            Some(1),
            Vec::new(),
        ) else {
            println!(
                "SKIPPED: no OpenSSH sftp-server on this system. The cut line is unproved here."
            );
            return;
        };
        assert_eq!(
            left.came_to,
            TheCheckCameTo::TheSittingStands(Completion::TheConnectionDropped)
        );
        assert_eq!(
            left.read, 2,
            "the root was read and the library's reading met the gone line"
        );
        let rows = the_rows(&left);
        assert_eq!(rows.len(), 3, "{rows:#?}");
        assert_eq!(
            rows.iter()
                .filter(|row| row.0 == RowState::NotFound)
                .count(),
            1
        );
        let standing = the_standing(&left);
        assert_eq!(standing.completion, Completion::TheConnectionDropped);
        let panel = the_panel(&left.held, &left.sitting.server, &left.sitting.file, None);
        assert!(panel
            .the_statements
            .iter()
            .any(|statement| statement.which.as_str() == "the-connection-dropped"));
        let foot = panel.the_foot.as_ref().expect("the foot");
        // **The total says it is a total so far** — the summary sentence's
        // ending since the dependency check's fresh look (2026-10-18); the
        // foot is only the button, *Upload the 1 found so far*.
        let summary = serde_json::to_string(&panel.the_summary)
            .expect("the summary sentence")
            .replace('\u{a0}', " ");
        assert!(
            foot.upload_is_unavailable && summary.contains("found so far"),
            "the total reads as final: {summary}"
        );
        let record = left.record.0.lock().expect("the record");
        assert_eq!(
            TheDependencyRecord::over(record.connection())
                .the_answers_of(&left.sitting.holding)
                .expect("they read")
                .len(),
            3,
            "the root's findings were not kept"
        );
    }

    // ------------------------------------------------------------ the desk's own files

    fn a_blender(path: &str, major: u32, minor: u32, patch: u32) -> BlenderIdentity {
        BlenderIdentity {
            path: path.to_string(),
            version: Version {
                major,
                minor,
                patch,
            },
        }
    }

    /// **The splash tester's three files, through the server's own point, with
    /// real Blender** — the three desk cases section 5 names. Run with
    /// `SMOOZY_DESK_WORK` set to the splash tester's `work` folder, and the
    /// desk's Blender 5.2.2 and 4.5.12 where piece 1's run found them.
    #[test]
    #[ignore = "needs the desk's Blenders and the splash tester's folder"]
    fn the_splash_and_battle_files_are_checked_whole_at_the_desk() {
        let work = PathBuf::from(std::env::var("SMOOZY_DESK_WORK").expect("SMOOZY_DESK_WORK"));
        let blender_52 = std::env::var("SMOOZY_DESK_BLENDER_52")
            .unwrap_or_else(|_| "/Applications/Blender.app/Contents/MacOS/Blender".to_string());
        let blender_45 = std::env::var("SMOOZY_DESK_BLENDER_45").unwrap_or_else(|_| {
            "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender".to_string()
        });

        // **5.2 as it comes: complete, nothing unfound, and the standing says so.**
        let scratch = Scratch::new("as-it-comes");
        let started = std::time::Instant::now();
        let left = a_sitting_over(
            &scratch,
            &work.join("as-they-come/5.2/blender-5.2-splash.blend"),
            a_blender(&blender_52, 5, 2, 2),
            HowItReads::WithBlender,
            None,
            Vec::new(),
        )
        .expect("an sftp-server");
        println!(
            "5.2 as it comes: {:?} in {:?}, rows {:#?}",
            left.came_to,
            started.elapsed(),
            the_rows(&left)
        );
        assert_eq!(
            left.came_to,
            TheCheckCameTo::TheSittingStands(Completion::Completed)
        );
        assert!(the_rows(&left).is_empty());
        let standing = the_standing(&left);
        assert_eq!(standing.completion, Completion::Completed);
        assert!(standing.every_row_confirmed_or_set_aside);
        assert_eq!(standing.counts.not_found, 0);
        assert_eq!(
            smoozy_rendering::warning::evaluate(
                &smoozy_dependencies::standing::WhatTheRecordSays::AStanding(Box::new(standing)),
                &a_blender(&blender_52, 5, 2, 2)
            ),
            smoozy_rendering::warning::TheWarningsCase::Absent
        );
        drop(left);
        drop(scratch);

        // **4.5 battle: only `treat_albedo` not found, from a scene two folders
        // deep in Chinese and Cyrillic folders.**
        let plan: serde_json::Value = serde_json::from_slice(
            &std::fs::read(work.join("battle/4.5/plan.json")).expect("the 4.5 plan"),
        )
        .expect("it reads");
        let scene = PathBuf::from(plan["scene"].as_str().expect("the scene"));
        assert!(scene.to_string_lossy().contains("镜头/сцена 1/"));
        let scratch = Scratch::new("battle-45");
        let started = std::time::Instant::now();
        let left = a_sitting_over(
            &scratch,
            &scene,
            a_blender(&blender_45, 4, 5, 12),
            HowItReads::WithBlender,
            None,
            Vec::new(),
        )
        .expect("an sftp-server");
        let rows = the_rows(&left);
        println!(
            "4.5 battle: {:?} in {:?}, {} rows",
            left.came_to,
            started.elapsed(),
            rows.len()
        );
        for row in &rows {
            println!("  {:?} {} -> {:?}", row.0, row.1, row.2);
        }
        assert_eq!(
            left.came_to,
            TheCheckCameTo::TheSittingStands(Completion::Completed)
        );
        let unfound: Vec<&str> = rows
            .iter()
            .filter(|row| row.0 == RowState::NotFound)
            .map(|row| row.1.as_str())
            .collect();
        assert_eq!(
            unfound,
            vec!["//../../assets/props/treat/textures/treat_albedo.png"]
        );
        assert_eq!(the_standing(&left).counts.not_found, 1);
        drop(left);
        drop(scratch);

        // **5.2 battle: only Mask4 and the decomposed name not found;
        // `normal.jpg` in two folders gives two rows; `Mask1` and `Мask1` give
        // two rows.** Searched over its own project folder too.
        let battle = work.join("battle/5.2");
        let plan: serde_json::Value =
            serde_json::from_slice(&std::fs::read(battle.join("plan.json")).expect("the 5.2 plan"))
                .expect("it reads");
        let scene = PathBuf::from(plan["scene"].as_str().expect("the scene"));
        let scratch = Scratch::new("battle-52");
        let started = std::time::Instant::now();
        let left = a_sitting_over(
            &scratch,
            &scene,
            a_blender(&blender_52, 5, 2, 2),
            HowItReads::WithBlender,
            None,
            vec![bytes_of(&battle)],
        )
        .expect("an sftp-server");
        let rows = the_rows(&left);
        println!(
            "5.2 battle: {:?} in {:?}, {} rows",
            left.came_to,
            started.elapsed(),
            rows.len()
        );
        for row in &rows {
            println!("  {:?} {} -> {:?}", row.0, row.1, row.2);
        }
        assert_eq!(
            left.came_to,
            TheCheckCameTo::TheSittingStands(Completion::Completed)
        );
        let mut unfound: Vec<String> = rows
            .iter()
            .filter(|row| row.0 == RowState::NotFound)
            .map(|row| row.1.rsplit('/').next().unwrap_or_default().to_string())
            .collect();
        unfound.sort();
        let decomposed_or_not = |name: &str| name.chars().filter(|c| !c.is_ascii()).count() > 0;
        assert_eq!(unfound.len(), 2, "{unfound:?}");
        assert!(
            unfound.iter().any(|name| name == "нет такого файла.png"),
            "{unfound:?}"
        );
        assert!(
            unfound
                .iter()
                .any(|name| name.ends_with("рассеивания.jpg") && decomposed_or_not(name)),
            "{unfound:?}"
        );
        let found_named = |name: &str| {
            rows.iter()
                .filter(|row| {
                    row.2
                        .as_deref()
                        .is_some_and(|at| at.rsplit('/').next() == Some(name))
                })
                .count()
        };
        assert_eq!(
            found_named("normal.jpg"),
            2,
            "normal.jpg in two folders is two rows"
        );
        assert_eq!(found_named("Mask1.png"), 1);
        assert_eq!(
            found_named("Мask1.png"),
            1,
            "the Cyrillic М is a file of its own"
        );
        assert!(
            found_named("лев's база #2.jpg") == 1,
            "the still picture with # in its name"
        );
        let standing = the_standing(&left);
        assert_eq!(standing.completion, Completion::Completed);
        assert_eq!(standing.counts.not_found, 2);
        assert!(!standing.every_row_confirmed_or_set_aside);
    }

    // ------------------------------------------------ piece 6: the found files sent up

    /// **One stand-in machine, one record and one machine key, kept across
    /// several checks and sends** — which is what a second check of the same
    /// file on the same server needs. Step 10c's dependency wave piece 6.
    struct ADesk {
        machine: Arc<AMachine>,
        record: Arc<TheRecord>,
        machine_key: MachineKey,
        layout: smoozy_job_files::layout::TheLayout,
        package_area: PathBuf,
        held: Mutex<WhatTheRunHolds>,
        the_run: smoozy_desktop::downloading::TheRunsTransfers,
        run: Identity,
    }

    fn a_desk(scratch: &Scratch, how_it_reads: HowItReads) -> Option<ADesk> {
        use smoozy_job_files::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
        let sftp_server = the_sftp_server()?;
        let home = scratch.at.join("home");
        let package_area = home.join("area");
        std::fs::create_dir_all(&package_area).expect("the package area");
        let root = home.join("root");
        std::fs::create_dir_all(&root).expect("the machine's root");
        let machine = Arc::new(AMachine {
            sftp_server,
            home,
            package_area: package_area.clone(),
            how_it_reads,
            kept: Mutex::new(BTreeMap::new()),
            the_line_goes_after: None,
            gone: AtomicBool::new(false),
            readings_taken: AtomicUsize::new(0),
            folders_asked: Mutex::new(Vec::new()),
            digests_taken: AtomicUsize::new(0),
            stops_asked: Mutex::new(Vec::new()),
            fixings_fail: AtomicBool::new(false),
            bytes_written: std::sync::atomic::AtomicU64::new(0),
        });
        let record = Arc::new(TheRecord(Mutex::new(
            LocalRecord::open_at(&scratch.at.join("record.sqlite3")).expect("the record opens"),
        )));
        Some(ADesk {
            machine,
            record,
            machine_key: MachineKey::of(
                Identity::issue(IdentityKind::LocalServer).expect("randomness"),
                Identity::issue(IdentityKind::Installation).expect("randomness"),
            ),
            layout: TheLayout::of(
                MachineRoot::chosen(root.to_str().expect("text")).expect("a root"),
                WhereThePackageAreaSits::UnderTheRoot,
            ),
            package_area,
            held: Mutex::new(WhatTheRunHolds::default()),
            the_run: smoozy_desktop::downloading::TheRunsTransfers::for_this_run(),
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
        })
    }

    fn a_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("a runtime")
    }

    /// **One check of `blend` on the desk's machine**, into the desk's own run
    /// memory — the root's holding file `File Browser` N6 would answer for an
    /// upload from this very place.
    fn a_check_on(
        desk: &ADesk,
        blend: &Path,
        with: BlenderIdentity,
        search_folders: Vec<Vec<u8>>,
    ) -> (TheCheckCameTo, TheSittingIsOf) {
        let holding = HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(
            &bytes_of(blend),
        ));
        a_check_holding(desk, blend, with, search_folders, holding)
    }

    /// **One check of `blend`, its answers keyed under `holding`** —
    /// `File Browser` N6's source path as [`a_check_on`] gives, or
    /// `Dependency Upload & Remap` K5's machine key for a file that reached the
    /// machine with no upload record behind it.
    fn a_check_holding(
        desk: &ADesk,
        blend: &Path,
        with: BlenderIdentity,
        search_folders: Vec<Vec<u8>>,
        holding: HoldingFile,
    ) -> (TheCheckCameTo, TheSittingIsOf) {
        {
            let opened = desk.record.0.lock().expect("the record");
            the_searched_folder_list::write(
                opened.connection(),
                &SearchedFolderList {
                    folders: search_folders,
                },
            )
            .expect("the search folders are written");
        }
        let sitting = TheSittingIsOf {
            server: desk.machine_key.server_row.as_text(),
            file: String::from_utf8(bytes_of(blend)).expect("the path is text"),
            root: MachineLocation::composed(bytes_of(blend)),
            holding,
            with,
            machine: desk.machine_key.clone(),
            layout: Some(desk.layout.clone()),
        };
        let came_to = a_runtime().block_on(async {
            let point = APointLending::over(&desk.machine).await;
            let mut on_the_machine = ThisServersMachine {
                sessions: &point.sessions,
                package_area: desk.package_area.to_str().expect("text"),
                server: &sitting.server,
            };
            the_check_runs(
                &mut on_the_machine,
                desk.record.as_ref(),
                &TheUsersOwnComputer,
                &desk.held,
                &sitting,
                &|| {},
            )
            .await
        });
        (came_to, sitting)
    }

    /// **The upload control pressed**, or one row's *Try again* or *Replace*.
    fn a_send_on(
        desk: &ADesk,
        of: &TheSittingIsOf,
        rows: Option<Vec<usize>>,
        replacing: bool,
    ) -> smoozy_desktop::the_found_files_sent::WhatTheSendCameTo {
        use smoozy_desktop::the_found_files_sent::{
            the_found_files_go_up, the_rows_the_upload_sends, WhatIsSent,
        };
        let rows = rows.unwrap_or_else(|| {
            let held = desk.held.lock().expect("the run");
            the_rows_the_upload_sends(&held.sittings[&keyed_as(of)].rows)
        });
        let asked = WhatIsSent {
            of: of.clone(),
            run: desk.run.clone(),
            rows,
            replacing,
        };
        let door: Arc<dyn smoozy_desktop::uploading::TheRecordAnUploadWrites> = desk.record.clone();
        a_runtime().block_on(async {
            let point = APointLending::over(&desk.machine).await;
            the_found_files_go_up(&asked, &point.sessions, &desk.the_run, door, &desk.held).await
        })
    }

    fn keyed_as(of: &TheSittingIsOf) -> String {
        format!("{}\u{1f}{}", of.server, of.file)
    }

    /// Every row of the desk's sitting over `of`, as (state, what the file
    /// records, where it was found).
    fn the_rows_on(desk: &ADesk, of: &TheSittingIsOf) -> Vec<(RowState, String, Option<String>)> {
        desk.held.lock().expect("the run").sittings[&keyed_as(of)]
            .rows
            .iter()
            .map(|row| {
                (
                    row.state,
                    String::from_utf8_lossy(row.row.ordering_key().as_bytes()).to_string(),
                    row.the_answer()
                        .and_then(|answer| answer.resolution.as_ref())
                        .map(|resolution| {
                            String::from_utf8_lossy(resolution.to.location().as_bytes()).to_string()
                        }),
                )
            })
            .collect()
    }

    fn the_standing_on(
        desk: &ADesk,
        of: &TheSittingIsOf,
    ) -> smoozy_dependencies::standing::CheckStanding {
        let record = desk.record.0.lock().expect("the record");
        TheDependencyRecord::over(record.connection())
            .the_standing(&of.root, &of.machine)
            .expect("the standing reads")
            .expect("a standing was written")
    }

    /// **Every transfer record the desk's record holds**, counted — so a send
    /// that sends nothing is seen to have sent nothing.
    fn the_transfers_written(desk: &ADesk) -> usize {
        let record = desk.record.0.lock().expect("the record");
        record
            .connection()
            .query_row("SELECT COUNT(*) FROM transfer_record", [], |row| {
                row.get::<_, i64>(0)
            })
            .expect("the transfers count") as usize
    }

    /// **A job waiting on the desk's machine**, as a submission writes its
    /// provisional record — no ending learned. Step 10c's Files' owed acts
    /// piece 1.
    fn a_waiting_job(
        job: &Identity,
        desk: &ADesk,
        with: &BlenderIdentity,
    ) -> smoozy_queue_history::record::LocalJobRecord {
        use smoozy_queue_history::record::*;
        LocalJobRecord {
            job: job.clone(),
            group: None,
            server: TheServerAsItStood {
                local_server: desk.machine_key.server_row.clone(),
                label: None,
                hue: None,
            },
            submitted_at: None,
            label: Some("Stone".to_string()),
            blend_path_on_the_machine: None,
            scene_name: None,
            submitted_with_no_scene_named: false,
            resolution_percentage_typed: None,
            picture_size_from_the_read: None,
            main_output_from_the_read: None,
            blender: TheBlenderTheEntryKeeps {
                pick: smoozy_render_settings::payload::VersionPick::of(with),
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

    fn the_mirror_of(desk: &ADesk, local: &Path) -> PathBuf {
        let mirror = smoozy_dependencies::landing::mirror_of(
            &desk.layout,
            &StoredLocation::as_the_system_returned_it(&bytes_of(local)),
        )
        .expect("the mirror fits");
        PathBuf::from(std::ffi::OsStr::from_bytes(mirror.as_bytes()))
    }

    /// **Step 10c's dependency wave piece 6, through the server's own point,
    /// onto the real record** — a check finds four files on this computer; the
    /// upload sends them; two land in their mirror folders byte for byte and
    /// read *on the server*, two meet a folder the login may not write and are
    /// said once above their rows; the standing counts what landed; **a second
    /// check sends nothing it already sent**; a third, with the folder
    /// writable, finds every row on the server and stops the warning; and a
    /// file changed on this computer asks *replace or stop*
    /// (`Dependency Upload & Remap` Q8) and is replaced only on *Replace*.
    #[test]
    fn the_found_files_land_in_their_mirrors_and_a_second_check_sends_nothing_again() {
        use smoozy_dependencies::rows::RowState::*;
        use std::os::unix::fs::PermissionsExt;
        // **The free app sends files up without an entitlement gate.**

        let scratch = Scratch::new("sent");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        std::fs::create_dir_all(project.join("ro")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        let wood: Vec<u8> = (0..300_000u32).map(|at| (at % 251) as u8).collect();
        std::fs::write(project.join("tex/wood.png"), &wood).expect("a picture");
        std::fs::write(project.join("tex/stone.png"), b"stone").expect("a picture");
        std::fs::write(project.join("ro/a.png"), b"a").expect("a picture");
        std::fs::write(project.join("ro/b.png"), b"b").expect("a picture");
        let reference = |name: &str, recorded: &str| {
            serde_json::json!({"kind": "Image", "datablock": name, "setting": "filepath",
                "recorded": recorded.as_bytes().to_vec(), "scenes": {"Established": ["Scene"]}, "source": "FILE"})
        };
        let document = serde_json::json!({
            "references": [
                reference("wood.png", "//tex/wood.png"),
                reference("stone.png", "//tex/stone.png"),
                reference("a.png", "//ro/a.png"),
                reference("b.png", "//ro/b.png")
            ],
            "flat_list": [], "weak_references": [], "observed_stand_ins": []
        });
        let mut documents = BTreeMap::new();
        documents.insert(
            bytes_of(&root),
            serde_json::to_vec(&document).expect("it writes"),
        );
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!(
                "SKIPPED: no OpenSSH sftp-server on this system. The sending is unproved here."
            );
            return;
        };
        let with = BlenderIdentity {
            path: "/root/blender/blender-5.2.2/blender".to_string(),
            version: Version {
                major: 5,
                minor: 2,
                patch: 2,
            },
        };

        // **The first check: four found on this computer.**
        let (came_to, of) = a_check_on(&desk, &root, with.clone(), Vec::new());
        assert_eq!(
            came_to,
            TheCheckCameTo::TheSittingStands(Completion::Completed)
        );
        assert!(the_rows_on(&desk, &of)
            .iter()
            .all(|row| row.0 == FoundOnYourComputer));

        // **A folder the login may not write**, already standing on the machine.
        let refused_folder = the_mirror_of(&desk, &project.join("ro"));
        std::fs::create_dir_all(&refused_folder).expect("the folder");
        std::fs::set_permissions(&refused_folder, std::fs::Permissions::from_mode(0o555))
            .expect("read only");

        // **The upload.**
        let sent = a_send_on(&desk, &of, None, false);
        println!("FIRST SEND: {sent:?}");
        assert_eq!(
            (sent.published, sent.failed, sent.did_not_finish),
            (2, 2, 0),
            "{sent:?}"
        );
        for name in ["tex/wood.png", "tex/stone.png"] {
            let local = project.join(name);
            assert_eq!(
                std::fs::read(the_mirror_of(&desk, &local)).expect("it landed at its mirror"),
                std::fs::read(&local).expect("the local file"),
                "{name} did not land byte for byte"
            );
        }
        let rows = the_rows_on(&desk, &of);
        for row in &rows {
            let expected = if row.1.starts_with("//ro/") {
                CouldNotBeSent
            } else {
                AlreadyOnTheMachine
            };
            assert_eq!(row.0, expected, "{row:?}");
        }
        // **One reason for several rows**, said once above them.
        let panel = the_panel(
            &desk.held.lock().expect("the run"),
            &of.server,
            &of.file,
            None,
        );
        let once = panel
            .the_once
            .expect("several refused for one reason are said once");
        assert_eq!(once.values[0].value.as_str(), "2");
        assert!(once
            .controls
            .iter()
            .all(|control| control.act.as_str() != "admin-access"));
        let table = panel.the_table.expect("the table");
        let states: Vec<String> = table
            .groups
            .iter()
            // **The group heading says the state, and a row whose state
            // changed after the check ran says its new one as its first
            // note** — since the dependency check's fresh look (2026-10-18);
            // no row carries a State column since the Creator's sixth notes,
            // note 4.
            .flat_map(|group| {
                group.rows.iter().map(move |row| {
                    let notes = serde_json::to_string(&row.notes)
                        .expect("the notes")
                        .replace('\u{a0}', " ");
                    if notes.contains("\"Already on the server\"") {
                        "Already on the server".to_string()
                    } else {
                        group.heading.as_str().replace('\u{a0}', " ")
                    }
                })
            })
            .collect();
        assert_eq!(
            states
                .iter()
                .filter(|state| *state == "Already on the server")
                .count(),
            2,
            "{states:?}"
        );
        // **K7's placement records, at publication.**
        let placements = {
            let record = desk.record.0.lock().expect("the record");
            TheDependencyRecord::over(record.connection())
                .the_placements_on(&desk.machine_key)
                .expect("the placements read")
        };
        assert_eq!(placements.len(), 2, "{placements:#?}");
        // **The standing counts what this sitting's uploads placed** (V11).
        let standing = the_standing_on(&desk, &of);
        assert_eq!(
            (standing.counts.confirmed, standing.counts.could_not_be_sent),
            (2, 2),
            "{:?}",
            standing.counts
        );
        assert!(!standing.every_row_confirmed_or_set_aside);
        // **Transfers draws the send** — the *Dependencies* tag, named after the
        // `.blend`, 2 of 4, and each refused file on its own row.
        let servers = vec![smoozy_desktop::surface::transfers::AServerKnown {
            row: of.server.clone(),
            name: "desk".to_string(),
            badge: None,
        }];
        let known = smoozy_desktop::surface::transfers::WhatIsKnown {
            servers: servers.clone(),
            requests: smoozy_desktop::surface::transfers::the_requests_the_sends_hold(&servers),
            ..Default::default()
        };
        let screen = smoozy_desktop::surface::transfers::the_screen_from(&known, None);
        let request = screen
            .requests
            .iter()
            .find(|one| one.tag.as_str() == "Dependencies")
            .expect("the send is on Transfers");
        let name = serde_json::to_string(&request.name).expect("it writes");
        assert!(
            name.contains("scene.blend") && !name.contains("/project/"),
            "{name}"
        );
        assert_eq!(
            request.control.as_ref().expect("the way back").act.as_str(),
            "open-the-check"
        );
        assert_eq!(request.files.len(), 2, "each refused file on its own row");

        // **A second check of the same file sends nothing it already sent.**
        std::fs::set_permissions(&refused_folder, std::fs::Permissions::from_mode(0o755))
            .expect("writable");
        let before = the_transfers_written(&desk);
        let (_, of) = a_check_on(&desk, &root, with.clone(), Vec::new());
        let rows = the_rows_on(&desk, &of);
        println!("SECOND CHECK: {rows:#?}");
        assert_eq!(
            rows.iter()
                .filter(|row| row.0 == AlreadyOnTheMachine)
                .count(),
            2,
            "{rows:#?}"
        );
        let sent = a_send_on(&desk, &of, None, false);
        assert_eq!(
            sent.published, 2,
            "only the two that did not land go: {sent:?}"
        );
        assert_eq!(the_transfers_written(&desk), before + 2);

        // **A third check: every row on the server, nothing sent, and the
        // warning stops.**
        let before = the_transfers_written(&desk);
        let (_, of) = a_check_on(&desk, &root, with.clone(), Vec::new());
        assert!(the_rows_on(&desk, &of)
            .iter()
            .all(|row| row.0 == AlreadyOnTheMachine));
        let sent = a_send_on(&desk, &of, None, false);
        assert_eq!(sent, Default::default(), "nothing is sent again");
        assert_eq!(the_transfers_written(&desk), before);
        let standing = the_standing_on(&desk, &of);
        assert!(standing.every_row_confirmed_or_set_aside);
        assert_eq!(
            smoozy_rendering::warning::evaluate(
                &smoozy_dependencies::standing::WhatTheRecordSays::AStanding(Box::new(standing)),
                &with
            ),
            smoozy_rendering::warning::TheWarningsCase::Absent
        );

        // **A file changed on this computer: a different file is at that
        // location** (`Dependency Upload & Remap` Q8) — asked, not sent, until
        // *Replace*.
        std::fs::write(project.join("tex/stone.png"), b"stone, painted again").expect("the change");
        let (_, of) = a_check_on(&desk, &root, with.clone(), Vec::new());
        let rows = the_rows_on(&desk, &of);
        let stone = rows
            .iter()
            .position(|row| row.1 == "//tex/stone.png")
            .expect("the stone row");
        assert_eq!(
            rows[stone].0, FoundOnYourComputer,
            "a changed file is not already on the server"
        );
        // **A waiting job reads that very file** (step 10c's Files' owed acts
        // piece 1): its record has no learned ending and its resolved set names
        // the mirror, so the overwrite box names it (`File Browser` W5).
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        {
            let record = desk.record.0.lock().expect("the record");
            let connection = record.connection();
            smoozy_desktop::record::job_records::JobRecords::over(connection)
                .write_the_provisional_record(&a_waiting_job(&job, &desk, &with))
                .expect("the job's record");
            TheDependencyRecord::over(connection)
                .a_jobs_resolved_set_is_written(
                    &job,
                    &desk.machine_key,
                    &MachineLocation::composed(bytes_of(&the_mirror_of(&desk, &root))),
                    &smoozy_dependencies::record::ResolvedSet {
                        locations: vec![MachineLocation::composed(bytes_of(&the_mirror_of(
                            &desk,
                            &project.join("tex/stone.png"),
                        )))],
                        remap_material: MachineLocation::composed(b"/nowhere/remap".to_vec()),
                    },
                )
                .expect("the job's set");
        }
        let before = the_transfers_written(&desk);
        let sent = a_send_on(&desk, &of, None, false);
        assert_eq!(sent.asks_to_replace, Some(stone), "{sent:?}");
        assert_eq!(
            the_transfers_written(&desk),
            before,
            "nothing is sent before Replace"
        );
        let overwrite = the_panel(
            &desk.held.lock().expect("the run"),
            &of.server,
            &of.file,
            None,
        )
        .the_overwrite
        .expect("the overwrite box");
        assert!(
            overwrite.live_jobs.is_some(),
            "the overwrite box named no job, though a waiting job reads the file"
        );
        let sent = a_send_on(&desk, &of, Some(vec![stone]), true);
        assert_eq!(sent.published, 1, "{sent:?}");
        assert_eq!(
            std::fs::read(the_mirror_of(&desk, &project.join("tex/stone.png"))).expect("replaced"),
            b"stone, painted again"
        );
        let (_, of) = a_check_on(&desk, &root, with, Vec::new());
        assert!(the_rows_on(&desk, &of)
            .iter()
            .all(|row| row.0 == AlreadyOnTheMachine));
    }

    /// **A send's found files go up four at once, and each mirror folder is
    /// asked for once in the send, not once per file** — step 10c's
    /// eleventh-pass fix piece 4 (`Dependency Upload & Remap` A7: *a directory
    /// seen standing stays standing*; `Dependency Upload & Remap` A9, `File
    /// Browser` O1). Run at every desk run.
    ///
    /// **The fault as a person met it**: the 4.5 battle project's 57 found
    /// files went up one at a time, and every one asked for every folder of
    /// its mirror again — a refusal and a read for each standing one — so the
    /// send cost about seven seconds a file on a line that could carry it in
    /// less than one.
    #[test]
    fn the_found_files_go_up_four_at_once_and_each_mirror_folder_is_asked_for_once() {

        let files = 12usize;
        let scratch = Scratch::new("many-sent");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        let mut references = Vec::new();
        for at in 0..files {
            let name = format!("tex_{at:02}.png");
            let bytes: Vec<u8> = (0..(200_000 + 997 * at) as u32)
                .map(|one| (one % 251) as u8)
                .collect();
            std::fs::write(project.join("tex").join(&name), &bytes).expect("a picture");
            references.push(serde_json::json!({"kind": "Image", "datablock": name, "setting": "filepath",
                "recorded": format!("//tex/{name}").into_bytes(), "scenes": {"Established": ["Scene"]}, "source": "FILE"}));
        }
        let document = serde_json::json!({
            "references": references,
            "flat_list": [], "weak_references": [], "observed_stand_ins": []
        });
        let mut documents = BTreeMap::new();
        documents.insert(
            bytes_of(&root),
            serde_json::to_vec(&document).expect("it writes"),
        );
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!(
                "SKIPPED: no OpenSSH sftp-server on this system. The sending is unproved here."
            );
            return;
        };
        let with = BlenderIdentity {
            path: "/root/blender/blender-5.2.2/blender".to_string(),
            version: Version {
                major: 5,
                minor: 2,
                patch: 2,
            },
        };
        let (_, of) = a_check_on(&desk, &root, with, Vec::new());
        assert_eq!(the_rows_on(&desk, &of).len(), files);
        // **The folders above one found file's own name, from `/` down** —
        // what `Dependency Upload & Remap` A7 makes, segment by segment.
        let the_folder = the_mirror_of(&desk, &project.join("tex/tex_00.png"))
            .parent()
            .expect("a folder")
            .to_path_buf();
        let folders = the_folder.components().count() - 1;
        let is_a_mirror_folder = |path: &[u8]| {
            let path = std::path::Path::new(std::ffi::OsStr::from_bytes(path));
            path != std::path::Path::new("/") && the_folder.starts_with(path)
        };
        let asked_before = desk
            .machine
            .folders_asked
            .lock()
            .expect("the folders")
            .len();
        let stop = AtomicBool::new(false);
        let (sent, at_once) = std::thread::scope(|scope| {
            let looking = scope.spawn(|| {
                let mut most = 0;
                while !stop.load(Ordering::SeqCst) {
                    for under_way in desk.the_run.what_is_under_way().values() {
                        most = most.max(under_way.moving.len());
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                most
            });
            let sent = a_send_on(&desk, &of, None, false);
            stop.store(true, Ordering::SeqCst);
            (sent, looking.join().expect("the looks"))
        });
        let asked = desk.machine.folders_asked.lock().expect("the folders")[asked_before..]
            .iter()
            .filter(|path| is_a_mirror_folder(path))
            .count();
        println!(
            "MANY SENT: {sent:?}; {at_once} at once; the mirror's folders asked for {asked} times \
             for {files} files under {folders} folders each."
        );
        assert_eq!(
            (sent.published, sent.failed, sent.did_not_finish),
            (files, 0, 0),
            "{sent:?}"
        );
        for at in 0..files {
            let local = project.join(format!("tex/tex_{at:02}.png"));
            assert_eq!(
                std::fs::read(the_mirror_of(&desk, &local)).expect("it landed at its mirror"),
                std::fs::read(&local).expect("the local file"),
                "tex_{at:02}.png did not land byte for byte"
            );
        }
        assert!(
            (2..=4).contains(&at_once),
            "{at_once} files of the send moved at once, not two to four"
        );
        assert!(
            asked <= folders,
            "the mirror's folders were asked for {asked} times for {files} files under {folders} \
             folders — more than once in the send"
        );
        // **Each publication's placement record carries its own file's four
        // values** (`Dependency Upload & Remap` K7): the readings go through one
        // planted link, and two at once would read each other's file. Every
        // file here has its own size, so a reading of another file shows.
        let placements = {
            let record = desk.record.0.lock().expect("the record");
            TheDependencyRecord::over(record.connection())
                .the_placements_on(&desk.machine_key)
                .expect("the placements read")
        };
        assert_eq!(placements.len(), files, "{placements:#?}");
        for placement in &placements {
            let landed = std::fs::metadata(std::path::Path::new(std::ffi::OsStr::from_bytes(
                placement.mirrored_location.as_bytes(),
            )))
            .expect("the file landed")
            .len();
            assert_eq!(
                placement.four_values_at_publication.size, landed,
                "a placement record carries another file's four values: {placement:#?}"
            );
        }
        // **And a second check finds every one already on the server.**
        let with = BlenderIdentity {
            path: "/root/blender/blender-5.2.2/blender".to_string(),
            version: Version {
                major: 5,
                minor: 2,
                patch: 2,
            },
        };
        let (_, of) = a_check_on(&desk, &root, with, Vec::new());
        assert!(
            the_rows_on(&desk, &of)
                .iter()
                .all(|row| row.0 == RowState::AlreadyOnTheMachine),
            "{:#?}",
            the_rows_on(&desk, &of)
        );
    }

    /// **A cache folder the check finds is sent whole** — after 0.1.8, the
    /// checks' leftovers, item 1. The check before 0.1.8 on Hyperstack found
    /// four cache folders and `bake_load`'s on the Mac, and every one failed
    /// to send with *Smoozy could not read this file on your computer*: the
    /// send read a folder as though it were one file. **A folder-shaped row's
    /// answer is a folder** (`Dependency Upload & Remap` F28), and every file
    /// under it lands at its own mirror (`Dependency Upload & Remap` A4),
    /// subfolders included, so the folder stands whole at the folder's mirror
    /// where the machine's copy reads it (`Dependency Upload & Remap` M22c).
    /// A second press sends nothing again.
    #[test]
    fn a_cache_folder_the_check_finds_is_sent_whole() {

        let scratch = Scratch::new("folder-sent");
        let project = scratch.at.join("project");
        let cache = project.join("blendcache_scene");
        let bake = project.join("bakes").join("1a2b");
        std::fs::create_dir_all(&cache).expect("the cache folder");
        std::fs::create_dir_all(bake.join("meta")).expect("the bake folder");
        std::fs::create_dir_all(bake.join("blobs")).expect("the bake folder");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        let mut made = Vec::new();
        for (at, name) in ["Cloth_000001_00.bphys", "Cloth_000002_00.bphys", "Cloth_000003_00.bphys"]
            .iter()
            .enumerate()
        {
            let bytes: Vec<u8> = (0..(40_000 + 313 * at) as u32).map(|one| (one % 241) as u8).collect();
            std::fs::write(cache.join(name), &bytes).expect("a cache file");
            made.push(cache.join(name));
        }
        std::fs::write(bake.join("meta").join("1.json"), b"{\"frame\": 1}").expect("a bake file");
        std::fs::write(bake.join("blobs").join("1_0.blob"), vec![7u8; 9_000]).expect("a bake file");
        made.push(bake.join("meta").join("1.json"));
        made.push(bake.join("blobs").join("1_0.blob"));
        // **Smoozy's own staging folder is not sent** (`File Browser` X12's
        // reserved name), as a download into the user's folder may leave one.
        let staging = cache.join(smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER);
        std::fs::create_dir_all(&staging).expect("a staging folder");
        std::fs::write(staging.join("x.smoozy-part"), b"part").expect("a staged part");
        let document = serde_json::json!({
            "references": [
                {"kind": "PhysicsPointCache", "datablock": "Cloth", "setting": "filepath",
                 "recorded": b"//blendcache_scene/".to_vec(), "scenes": {"Established": ["Scene"]}, "source": null},
                {"kind": "GeometryNodeBakeFolder", "datablock": "Bake", "setting": "directory",
                 "recorded": b"//bakes/1a2b/".to_vec(), "scenes": {"Established": ["Scene"]}, "source": null}
            ],
            "flat_list": [], "weak_references": [], "observed_stand_ins": []
        });
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&root), serde_json::to_vec(&document).expect("it writes"));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!(
                "SKIPPED: no OpenSSH sftp-server on this system. The sending is unproved here."
            );
            return;
        };

        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        let rows = the_rows_on(&desk, &of);
        assert_eq!(rows.len(), 2, "{rows:#?}");
        assert!(
            rows.iter().all(|row| row.0 == RowState::FoundOnYourComputer),
            "{rows:#?}"
        );

        let sent = a_send_on(&desk, &of, None, false);
        println!("FOLDERS SENT: {sent:?}");
        let rows = the_rows_on(&desk, &of);
        assert!(
            rows.iter().all(|row| row.0 == RowState::AlreadyOnTheMachine),
            "a folder row did not go up: {rows:#?}"
        );
        assert_eq!(
            (sent.published, sent.failed, sent.did_not_finish),
            (made.len(), 0, 0),
            "{sent:?}"
        );
        for local in &made {
            assert_eq!(
                std::fs::read(the_mirror_of(&desk, local)).expect("it landed at its mirror"),
                std::fs::read(local).expect("the local file"),
                "{} did not land byte for byte",
                local.display()
            );
        }
        // **The machine's own staging folder stands there, empty** — the
        // upload's own, beside what it published (`Server protocol and
        // transfers` S1) — **and nothing of this computer's was sent into it.**
        assert!(
            !the_mirror_of(&desk, &staging.join("x.smoozy-part")).exists(),
            "Smoozy's own staging folder was sent"
        );

        // **A second check reads both folders already on the server**, each
        // file established by its own placement record, and a press sends
        // nothing again.
        let written = the_transfers_written(&desk);
        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        let rows = the_rows_on(&desk, &of);
        assert!(
            rows.iter().all(|row| row.0 == RowState::AlreadyOnTheMachine),
            "a folder sent whole is not read as on the server: {rows:#?}"
        );
        let again = a_send_on(&desk, &of, Some(vec![0, 1]), false);
        assert_eq!(again.published, 0, "{again:?}");
        assert_eq!(the_transfers_written(&desk), written, "a file was sent again");

        // **The cache folder moved away on this computer after it went up**:
        // the check never reads it *found on your computer*. It was sent, so
        // it reads *uploaded before, not confirmed* (`Dependency Upload &
        // Remap` Q7), and Confirm reads the machine's copies and finds them.
        // The pass before 0.1.9 on Vast, new fault 5.
        let away = scratch.at.join("moved-away");
        std::fs::rename(&cache, &away).expect("the folder is moved away");
        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        let rows = the_rows_on(&desk, &of);
        let at = rows
            .iter()
            .position(|row| row.1.contains("blendcache_scene"))
            .expect("the cache row");
        assert_eq!(rows[at].0, RowState::SentBeforeNotConfirmed, "{rows:#?}");
        let confirmed = a_runtime().block_on(async {
            let point = APointLending::over(&desk.machine).await;
            smoozy_desktop::the_found_files_sent::a_row_is_confirmed(
                &point.sessions,
                desk.record.as_ref(),
                &desk.held,
                &of,
                at,
            )
            .await
        });
        assert!(confirmed, "Confirm did nothing on a folder row");
        assert_eq!(the_rows_on(&desk, &of)[at].0, RowState::AlreadyOnTheMachine);
        std::fs::rename(&away, &cache).expect("the folder is put back");
    }

    /// **A cache folder found and then moved away before it was ever sent
    /// reads *not found***, never *found on your computer* — the pass before
    /// 0.1.9 on Vast, new fault 5: the check went on reading the old place as
    /// found, and only the send said otherwise.
    #[test]
    fn a_cache_folder_moved_away_before_it_was_sent_reads_not_found() {

        let scratch = Scratch::new("folder-away");
        let project = scratch.at.join("project");
        let cache = project.join("blendcache_scene");
        std::fs::create_dir_all(&cache).expect("the cache folder");
        std::fs::write(cache.join("Cloth_000001_00.bphys"), vec![3u8; 4_000]).expect("a cache file");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        let document = serde_json::json!({
            "references": [
                {"kind": "PhysicsPointCache", "datablock": "Cloth", "setting": "filepath",
                 "recorded": b"//blendcache_scene/".to_vec(), "scenes": {"Established": ["Scene"]}, "source": null}
            ],
            "flat_list": [], "weak_references": [], "observed_stand_ins": []
        });
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&root), serde_json::to_vec(&document).expect("it writes"));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system.");
            return;
        };
        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        assert_eq!(the_rows_on(&desk, &of)[0].0, RowState::FoundOnYourComputer);
        std::fs::rename(&cache, scratch.at.join("moved-away")).expect("the folder is moved away");
        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        let rows = the_rows_on(&desk, &of);
        assert_eq!(rows[0].0, RowState::NotFound, "{rows:#?}");
    }

    /// **The 5.2 and 4.5 battle projects' found files, sent up at the desk with
    /// real Blender** — each found file lands in its mirror folder byte for
    /// byte, each of its rows reads *already on the server*, and a second check
    /// of the same file sends nothing again. Run with `SMOOZY_DESK_WORK` set to
    /// the splash tester's `work` folder, as the test above.
    #[test]
    #[ignore = "needs the desk's Blenders and the splash tester's folder"]
    fn the_battle_projects_found_files_land_in_their_mirrors_at_the_desk() {

        let work = PathBuf::from(std::env::var("SMOOZY_DESK_WORK").expect("SMOOZY_DESK_WORK"));
        let blender_52 = std::env::var("SMOOZY_DESK_BLENDER_52")
            .unwrap_or_else(|_| "/Applications/Blender.app/Contents/MacOS/Blender".to_string());
        let blender_45 = std::env::var("SMOOZY_DESK_BLENDER_45").unwrap_or_else(|_| {
            "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender".to_string()
        });
        for (battle, with, search) in [
            ("4.5", a_blender(&blender_45, 4, 5, 12), false),
            ("5.2", a_blender(&blender_52, 5, 2, 2), true),
        ] {
            let folder = work.join("battle").join(battle);
            let plan: serde_json::Value =
                serde_json::from_slice(&std::fs::read(folder.join("plan.json")).expect("the plan"))
                    .expect("it reads");
            let scene = PathBuf::from(plan["scene"].as_str().expect("the scene"));
            let scratch = Scratch::new(&format!("battle-sent-{battle}"));
            let desk = a_desk(&scratch, HowItReads::WithBlender).expect("an sftp-server");
            let search_folders = if search {
                vec![bytes_of(&folder)]
            } else {
                Vec::new()
            };
            let started = std::time::Instant::now();
            let (came_to, of) = a_check_on(&desk, &scene, with.clone(), search_folders.clone());
            assert_eq!(
                came_to,
                TheCheckCameTo::TheSittingStands(Completion::Completed)
            );
            let found: Vec<(RowState, String, Option<String>)> = the_rows_on(&desk, &of)
                .into_iter()
                .filter(|row| row.0 == RowState::FoundOnYourComputer)
                .collect();
            println!(
                "{battle} battle: checked in {:?}, {} found on this computer",
                started.elapsed(),
                found.len()
            );
            let started = std::time::Instant::now();
            let sent = a_send_on(&desk, &of, None, false);
            println!("{battle} battle: sent in {:?}: {sent:?}", started.elapsed());
            assert_eq!((sent.failed, sent.did_not_finish), (0, 0), "{sent:?}");
            let mut bytes = 0u64;
            for (_, records, at) in &found {
                let local = PathBuf::from(at.as_deref().expect("a found row has a location"));
                let landed =
                    std::fs::read(the_mirror_of(&desk, &local)).expect("it landed at its mirror");
                assert_eq!(
                    landed,
                    std::fs::read(&local).expect("the local file"),
                    "{records} did not land byte for byte"
                );
                bytes += landed.len() as u64;
            }
            println!(
                "{battle} battle: {} files, {bytes} bytes, byte for byte",
                found.len()
            );
            let rows = the_rows_on(&desk, &of);
            for (state, records, _) in rows
                .iter()
                .filter(|row| found.iter().any(|one| one.1 == row.1))
            {
                assert_eq!(*state, RowState::AlreadyOnTheMachine, "{records}");
            }
            // **A second check of the same file sends nothing again.**
            let before = the_transfers_written(&desk);
            let (_, of) = a_check_on(&desk, &scene, with, search_folders);
            let again = the_rows_on(&desk, &of);
            assert_eq!(
                again
                    .iter()
                    .filter(|row| row.0 == RowState::AlreadyOnTheMachine)
                    .count(),
                found.len(),
                "{again:#?}"
            );
            assert!(
                again
                    .iter()
                    .all(|row| row.0 != RowState::FoundOnYourComputer),
                "{again:#?}"
            );
            let sent = a_send_on(&desk, &of, None, false);
            assert_eq!(sent, Default::default(), "nothing is sent again");
            assert_eq!(the_transfers_written(&desk), before);
            let standing = the_standing_on(&desk, &of);
            println!("{battle} battle, second check: {:?}", standing.counts);
            assert_eq!(standing.counts.confirmed, found.len());
            assert_eq!(standing.counts.found_but_not_sent, 0);
        }
    }
    // ------------------------------- step 10c's dependency wave piece 7

    /// **What a submission of `of` placed on the desk's machine**: the act,
    /// through the server's own point, to the stand-in supervisor — which
    /// refuses the job record by name, as a machine that takes no job would,
    /// after the unit has placed the remap material and published the job's
    /// material — and every remap material then standing on the machine's
    /// disk.
    fn a_submission_on(
        desk: &ADesk,
        of: &TheSittingIsOf,
        with: &BlenderIdentity,
    ) -> (
        smoozy_desktop::submitting::WhatTheActCameTo,
        Vec<smoozy_dependencies::remap::RemapMaterial>,
        Vec<String>,
    ) {
        use smoozy_desktop::submitting::{the_act_is_performed, WhatThePageHandsOver};
        use smoozy_job_files::layout::Area;
        use smoozy_render_settings::payload::{compose, Composition};
        use smoozy_render_settings::store::SceneEntries;
        let payload = compose(Composition {
            version_pick: with,
            scene: None,
            answers: &SceneEntries {
                frame_range: Some(
                    smoozy_render_settings::frames::parse_the_frame_field("1").expect("a range"),
                ),
                ..Default::default()
            },
            standing: None,
            stale: false,
        });
        // **The five areas exist before the first job** (`Job files and
        // outputs` N11) — a placement makes them; the stand-in has them.
        for area in smoozy_job_files::layout::EVERY_AREA {
            std::fs::create_dir_all(desk.layout.area(area)).expect("an area");
        }
        let given = WhatThePageHandsOver {
            server: smoozy_queue_history::record::TheServerAsItStood {
                local_server: desk.machine_key.server_row.clone(),
                label: Some("desk".to_string()),
                hue: None,
            },
            installation: Some(desk.machine_key.installation.clone()),
            layout: desk.layout.clone(),
            package_area: desk.layout.area(Area::Package),
            the_blend: of.file.clone(),
            label: None,
            reporting: smoozy_rendering::submission::the_reporting_field(
                None,
                smoozy_render_settings::reporting::ReportingChoice::Everything,
            ),
            the_version_origin: smoozy_queue_history::record::AnswerOrigin::ResolvedForThem,
            dependency_check: Some(
                smoozy_queue_history::record::DependencyCheckAtSubmission::ACompleteStandingStood,
            ),
            what_the_desktop_holds_about_the_build:
                smoozy_rendering::submission::WhatTheDesktopHoldsAboutTheBuild::NothingButTheEntry,
            the_first_frame_check: false,
            the_download_as_it_renders_choice: false,
            where_the_downloads_go: None,
            the_formats_chosen: None,
            scene_jobs: vec![payload],
            the_reading_in_force: None,
            the_card_make: smoozy_job_files::manifest::TheCardMakeAtSubmission::TheMakes(vec![
                "NVIDIA".to_string(),
            ]),
            run: desk.run.clone(),
            carries_on_from: None,
            the_temporary_area: Some(desk.package_area.join("desk-temporary")),
            the_servers_fallback_folder: None,
            the_driver_at_submission: Default::default(),
            the_typed_frames: None,
        };
        let came_to = a_runtime().block_on(async {
            let point = APointLending::over(&desk.machine).await;
            let sessions = point.sessions.clone();
            let record = Arc::clone(&desk.record);
            let the_run = smoozy_desktop::downloading::TheRunsTransfers::for_this_run();
            tokio::task::spawn_blocking(move || {
                the_act_is_performed(given, &sessions, &the_run, record.as_ref())
            })
            .await
            .expect("the act ran")
        });
        let any_job = Identity::issue(IdentityKind::Job).expect("randomness");
        let at = smoozy_dependencies::remap::RemapMaterial::location_in(&desk.layout, &any_job);
        let folder = PathBuf::from(std::ffi::OsStr::from_bytes(at.as_bytes()))
            .parent()
            .expect("the remaps folder")
            .to_path_buf();
        let materials = std::fs::read_dir(&folder)
            .map(|entries| {
                entries
                    .flatten()
                    .filter_map(|entry| std::fs::read(entry.path()).ok())
                    .filter_map(|bytes| {
                        smoozy_dependencies::remap::RemapMaterial::read_back(&bytes)
                    })
                    .collect()
            })
            .unwrap_or_default();
        // **`Dependency Upload & Remap` K9's set**, as the record kept it.
        let resolved: Vec<String> = {
            let record = desk.record.0.lock().expect("the record");
            let mut read = record
                .connection()
                .prepare("SELECT machine_location FROM dependency_job_resolved_location ORDER BY job, position")
                .expect("it reads");
            read.query_map([], |row| row.get::<_, Vec<u8>>(0))
                .expect("it reads")
                .map(|at| String::from_utf8_lossy(&at.expect("a row")).to_string())
                .collect()
        };
        (came_to, materials, resolved)
    }

    /// **What the band reads for `of`**, from the desk's record alone.
    fn the_check_for(
        desk: &ADesk,
        of: &TheSittingIsOf,
    ) -> smoozy_desktop::surface::render_settings::WhatTheRecordHolds {
        let record = desk.record.0.lock().expect("the record");
        smoozy_desktop::surface::render_settings::WhatTheRecordHolds {
            the_check: smoozy_desktop::surface::render_settings::the_check_the_band_reads(
                record.connection(),
                &desk.machine_key,
                &of.file,
            )
            .expect("the record answers"),
            ..Default::default()
        }
    }

    /// **Each pair a material carries, as (kind, setting, what the file
    /// records, the new location)**, in text for reading.
    fn the_pairs_of(
        material: &smoozy_dependencies::remap::RemapMaterial,
    ) -> Vec<(String, String, String, String)> {
        let mut pairs: Vec<(String, String, String, String)> = material
            .pairs
            .iter()
            .map(|pair| {
                (
                    serde_json::to_string(&pair.kind)
                        .expect("a kind")
                        .trim_matches('"')
                        .to_string(),
                    pair.setting.clone(),
                    String::from_utf8_lossy(pair.expected.as_bytes()).to_string(),
                    String::from_utf8_lossy(pair.new_location.as_bytes()).to_string(),
                )
            })
            .collect();
        pairs.sort();
        pairs
    }

    /// **The Render Settings band for `of`, as the page draws it** from the
    /// run's memory `page` and the record's `record` — `None` where no band is
    /// drawn, else which of its statements.
    fn the_band_on(
        page: &mut smoozy_desktop::surface::render_settings::WhatTheRunHolds,
        of: &TheSittingIsOf,
        with: &BlenderIdentity,
        record: &smoozy_desktop::surface::render_settings::WhatTheRecordHolds,
    ) -> Option<String> {
        use smoozy_desktop::surface::render_settings::{
            the_answer_is_taken, the_panel_starting_from, WhichSetting,
        };
        page.version_lists.insert(
            of.server.clone(),
            smoozy_server_setup::search::TheSearchResult {
                entries: vec![smoozy_server_setup::search::ACandidate {
                    path: with.path.clone(),
                    answered: Some(with.version),
                }],
                answer: smoozy_server_setup::search::WhatTheAnswerSays {
                    the_search_is_by_name: false,
                    folders_refused: 0,
                    did_not_answer: Vec::new(),
                },
                at: std::time::SystemTime::now(),
            },
        );
        let _ = the_answer_is_taken(
            page,
            &of.server,
            &of.file,
            WhichSetting::VersionPick,
            &with.path,
        );
        let on = smoozy_desktop::surface::render_settings::WhatTheRecordHolds {
            the_dependency_warning:
                smoozy_desktop::surface::render_settings::TheSwitchReads::NotSwitchedOff,
            ..record.clone()
        };
        the_panel_starting_from(page, &of.server, &of.file, &on)
            .the_dependency_band
            .map(|band| band.which.as_str().to_string())
    }

    /// **A scripted `.blend` whose references are pictures**, recorded as
    /// given.
    fn a_document_of(pictures: &[&str]) -> Vec<u8> {
        let references: Vec<serde_json::Value> = pictures
            .iter()
            .map(|recorded| {
                serde_json::json!({"kind": "Image", "datablock": recorded, "setting": "filepath",
                    "recorded": recorded.as_bytes().to_vec(),
                    "scenes": {"Established": ["Scene"]}, "source": "FILE"})
            })
            .collect();
        serde_json::to_vec(&serde_json::json!({
            "references": references,
            "flat_list": [], "weak_references": [], "observed_stand_ins": []
        }))
        .expect("it writes")
    }

    fn a_5_2() -> BlenderIdentity {
        BlenderIdentity {
            path: "/root/blender/blender-5.2.2/blender".to_string(),
            version: Version {
                major: 5,
                minor: 2,
                patch: 2,
            },
        }
    }

    /// **Step 10c's dependency wave piece 7, the fault as a person meets it:
    /// a project checked and sent is submitted with no remap**, so the render
    /// looks for its pictures at the Mac's paths. The check finds two pictures
    /// and a third it cannot find; the upload sends the two; the submission
    /// places the material — and the material must point both pictures at
    /// their mirrors, and the third at nothing.
    #[test]
    fn a_project_checked_and_sent_is_submitted_with_its_remap() {

        let scratch = Scratch::new("remapped");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        std::fs::write(project.join("tex/wood.png"), b"wood").expect("a picture");
        std::fs::write(project.join("tex/stone.png"), b"stone").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(
            bytes_of(&root),
            a_document_of(&["//tex/wood.png", "//tex/stone.png", "//tex/gone.png"]),
        );
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system. The remap is unproved here.");
            return;
        };
        it_came_up_through_files(&desk, &root);
        let with = a_5_2();
        let (_, of) = a_check_on(&desk, &root, with.clone(), Vec::new());
        let sent = a_send_on(&desk, &of, None, false);
        assert_eq!(sent.published, 2, "{sent:?}");

        let (came_to, materials, resolved) = a_submission_on(&desk, &of, &with);
        println!("SUBMITTED: {came_to:?}");
        assert_eq!(materials.len(), 1, "one job, one material placed first");
        let pairs = the_pairs_of(&materials[0]);
        println!("PAIRS: {pairs:#?}");
        let mirror = |name: &str| {
            the_mirror_of(&desk, &project.join(name))
                .to_string_lossy()
                .to_string()
        };
        assert_eq!(
            pairs,
            vec![
                (
                    "Image".to_string(),
                    "filepath".to_string(),
                    "//tex/stone.png".to_string(),
                    mirror("tex/stone.png")
                ),
                (
                    "Image".to_string(),
                    "filepath".to_string(),
                    "//tex/wood.png".to_string(),
                    mirror("tex/wood.png")
                ),
            ],
            "the render would look for the pictures at the Mac's paths"
        );
        // **`Dependency Upload & Remap` K9: the set names both mirrors and the
        // material**, written with the provisional record.
        println!("RESOLVED SET: {resolved:#?}");
        assert!(resolved.contains(&mirror("tex/wood.png")), "{resolved:?}");
        assert!(resolved.contains(&mirror("tex/stone.png")), "{resolved:?}");
        let material_at = {
            let record = desk.record.0.lock().expect("the record");
            record
                .connection()
                .query_row(
                    "SELECT remap_material FROM dependency_job_resolved_set",
                    [],
                    |row| row.get::<_, Vec<u8>>(0),
                )
                .expect("the set was written")
        };
        assert!(String::from_utf8_lossy(&material_at).contains("/remaps/"));
    }

    /// **The fault as a person meets it: a complete check that left nothing
    /// unfound, and the band says nobody has checked** once the application
    /// has been closed and opened again — the page's memory is the run's, and
    /// the record holds the standing.
    #[test]
    fn a_complete_check_stops_the_warning_after_a_restart() {
        let scratch = Scratch::new("stops");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(&project).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&root), a_document_of(&[]));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system.");
            return;
        };
        let with = a_5_2();
        let (_, of) = a_check_on(&desk, &root, with.clone(), Vec::new());
        let standing = the_standing_on(&desk, &of);
        assert!(standing.every_row_confirmed_or_set_aside, "{standing:?}");
        // **A new run**: nothing in the page's memory and no check in hand.
        let mut page = smoozy_desktop::surface::render_settings::WhatTheRunHolds::default();
        let record = the_check_for(&desk, &of);
        let band = the_band_on(&mut page, &of, &with, &record);
        println!("AFTER A RESTART: {band:?}");
        assert_eq!(band, None, "the band stays after a complete check");

        // **A later change brings it back**: an upload published onto the
        // file's own path, which `File Browser` X7a marks in the record.
        {
            let opened = desk.record.0.lock().expect("the record");
            TheDependencyRecord::over(opened.connection())
                .a_standing_is_written(&standing.clone().replaced_since_check())
                .expect("the mark is written");
        }
        let record = the_check_for(&desk, &of);
        let band = the_band_on(&mut page, &of, &with, &record);
        println!("AFTER AN UPLOAD ONTO IT: {band:?}");
        assert_eq!(band.as_deref(), Some("replaced"));
    }

    /// **The safe side, after piece 7b: a standing whose answers sit under a
    /// key no reader finds any more does not stop the warning.** Two projects
    /// came up through Files and share a picture; both are checked and the
    /// picture sent; then the user removes the second's upload record from
    /// Transfers (`File Browser` N8). `File Browser` N6 now answers nothing for
    /// it, so the band, the composition and the walk all key it by its place
    /// on the server (`Dependency Upload & Remap` K5) and find none of the
    /// answers its check wrote. **The standing counted a picture confirmed on
    /// the machine, so the warning stands**, and a *Replace* over the picture
    /// cannot take it away.
    #[test]
    fn a_standing_whose_answers_no_reader_finds_does_not_stop_the_warning() {

        let scratch = Scratch::new("shared");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let first = project.join("first.blend");
        let second = project.join("second.blend");
        std::fs::write(&first, b"first").expect("a root");
        std::fs::write(&second, b"second").expect("a root");
        std::fs::write(project.join("tex/shared.png"), b"shared").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&first), a_document_of(&["//tex/shared.png"]));
        documents.insert(bytes_of(&second), a_document_of(&["//tex/shared.png"]));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system.");
            return;
        };
        it_came_up_through_files(&desk, &first);
        let seconds_upload = it_came_up_through_files(&desk, &second);
        let with = a_5_2();
        let (_, of_first) = a_check_on(&desk, &first, with.clone(), Vec::new());
        assert_eq!(a_send_on(&desk, &of_first, None, false).published, 1);
        let (_, of_second) = a_check_on(&desk, &second, with.clone(), Vec::new());
        assert!(the_rows_on(&desk, &of_second)
            .iter()
            .all(|row| row.0 == RowState::AlreadyOnTheMachine));
        assert!(the_standing_on(&desk, &of_second).every_row_confirmed_or_set_aside);
        let mut page = smoozy_desktop::surface::render_settings::WhatTheRunHolds::default();
        let record = the_check_for(&desk, &of_second);
        assert!(record.the_check.a_stop_can_be_kept);
        assert_eq!(the_band_on(&mut page, &of_second, &with, &record), None);

        {
            use smoozy_file_browser::request::KeepsTheRequest;
            let opened = desk.record.0.lock().expect("the record");
            smoozy_desktop::record::upload_requests::UploadRequests::over(opened.connection())
                .the_user_removed_it(&seconds_upload)
                .expect("the upload record is removed");
        }
        let record = the_check_for(&desk, &of_second);
        assert!(!record.the_check.a_stop_can_be_kept);
        let band = the_band_on(&mut page, &of_second, &with, &record);
        println!("THE OTHER PROJECT AFTER ITS UPLOAD RECORD WENT: {band:?}");
        assert_eq!(band.as_deref(), Some("never"));

        std::fs::write(project.join("tex/shared.png"), b"shared, painted again")
            .expect("the change");
        let (_, of_first) = a_check_on(&desk, &first, with.clone(), Vec::new());
        let sent = a_send_on(&desk, &of_first, Some(vec![0]), true);
        assert_eq!(sent.published, 1, "{sent:?}");
        let record = the_check_for(&desk, &of_second);
        let band = the_band_on(&mut page, &of_second, &with, &record);
        println!("THE OTHER PROJECT AFTER THE REPLACE: {band:?}");
        assert!(band.is_some(), "the other project's warning is gone");
    }

    /// **The same *Replace*, over two projects `Dependency Upload & Remap`
    /// V13a's walk can find** — each
    /// reached the machine with no upload record behind it, so each is keyed
    /// by its place on the server (`Dependency Upload & Remap` K5), as the walk
    /// keys it. **The second's warning stops after its check, and the
    /// *Replace* brings it back** — the mark written by the act, read from the
    /// record.
    #[test]
    fn a_replace_over_a_shared_picture_brings_back_the_warning_of_a_project_the_walk_can_find() {

        let scratch = Scratch::new("shared-k5");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let first = project.join("first.blend");
        let second = project.join("second.blend");
        std::fs::write(&first, b"first").expect("a root");
        std::fs::write(&second, b"second").expect("a root");
        std::fs::write(project.join("tex/shared.png"), b"shared").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&first), a_document_of(&["//tex/shared.png"]));
        documents.insert(bytes_of(&second), a_document_of(&["//tex/shared.png"]));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system.");
            return;
        };
        let with = a_5_2();
        let by_its_place = |blend: &Path| HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
            machine: desk.machine_key.clone(),
            location: MachineLocation::composed(bytes_of(blend)),
        };
        // **`Dependency Upload & Remap` K5: a relative location is not looked
        // for on this computer**, so
        // the project's folder is searched.
        let search = vec![bytes_of(&project)];
        let (_, of_first) = a_check_holding(
            &desk,
            &first,
            with.clone(),
            search.clone(),
            by_its_place(&first),
        );
        assert_eq!(a_send_on(&desk, &of_first, None, false).published, 1);
        let (_, of_second) = a_check_holding(
            &desk,
            &second,
            with.clone(),
            search.clone(),
            by_its_place(&second),
        );
        assert!(the_rows_on(&desk, &of_second)
            .iter()
            .all(|row| row.0 == RowState::AlreadyOnTheMachine));

        let mut page = smoozy_desktop::surface::render_settings::WhatTheRunHolds::default();
        let record = the_check_for(&desk, &of_second);
        assert!(record.the_check.a_stop_can_be_kept);
        let band = the_band_on(&mut page, &of_second, &with, &record);
        println!("THE OTHER PROJECT AFTER ITS CHECK: {band:?}");
        assert_eq!(band, None, "a complete check that left nothing unfound");

        // **After a restart**, the kinds its pair needs are in the record
        // (`Dependency Upload & Remap` K4's last limb, piece 7b), so the stop
        // stands with no check in hand.
        smoozy_is_closed_and_opened(&desk, &scratch);
        let record = the_check_for(&desk, &of_second);
        assert!(record.the_check.a_stop_can_be_kept);
        let band = the_band_on(&mut page, &of_second, &with, &record);
        println!("THE OTHER PROJECT AFTER A RESTART: {band:?}");
        assert_eq!(band, None);

        std::fs::write(project.join("tex/shared.png"), b"shared, painted again")
            .expect("the change");
        let (_, of_first) =
            a_check_holding(&desk, &first, with.clone(), search, by_its_place(&first));
        let sent = a_send_on(&desk, &of_first, Some(vec![0]), true);
        assert_eq!(sent.published, 1, "{sent:?}");
        assert!(the_standing_on(&desk, &of_second).disturbed);

        let record = the_check_for(&desk, &of_second);
        let band = the_band_on(&mut page, &of_second, &with, &record);
        println!("THE OTHER PROJECT AFTER THE REPLACE: {band:?}");
        assert_eq!(band.as_deref(), Some("replaced"));
    }

    // ----------------------------- piece 7b: the pairs kept, the warning that can stop

    /// **`File Browser` N6's record for `blend`, as an upload through Files
    /// writes it** — one request whose one file landed where it was sent, under
    /// the desk's server and installation. The machine here is this Mac, so the
    /// file lands at its own path, which is the landing the harness has always
    /// taken ([`a_check_on`]).
    fn it_came_up_through_files(desk: &ADesk, blend: &Path) -> Identity {
        use smoozy_file_browser::request::{
            Disposition, EnumerationEntry, KeepsTheRequest, UploadRequest,
        };
        let mut entry = EnumerationEntry::enumerated(bytes_of(blend), bytes_of(blend), 5);
        entry.disposition = Some(Disposition::ToSend);
        entry.outcome = Some(smoozy_state::outcomes::UnitOutcome::Done);
        entry.outcome_written_at = Some(std::time::SystemTime::now());
        let request = UploadRequest {
            record: Identity::issue(IdentityKind::Operation).expect("randomness"),
            server_row: desk.machine_key.server_row.clone(),
            installation: Some(desk.machine_key.installation.clone()),
            destination: bytes_of(blend.parent().expect("a folder")),
            entries: vec![entry],
            collision_answer: None,
            telling: None,
            opened_by_run: desk.run.clone(),
        };
        let opened = desk.record.0.lock().expect("the record");
        smoozy_desktop::record::upload_requests::UploadRequests::over(opened.connection())
            .write_it_down(&request)
            .expect("the upload is written down");
        request.record
    }

    /// **Smoozy closed and opened**: nothing of the run is kept — no check in
    /// hand, no page — and the record is opened again from its file.
    fn smoozy_is_closed_and_opened(desk: &ADesk, scratch: &Scratch) {
        *desk.held.lock().expect("the run") = WhatTheRunHolds::default();
        *desk.record.0.lock().expect("the record") =
            LocalRecord::open_at(&scratch.at.join("record.sqlite3")).expect("the record opens");
    }

    /// **Piece 7b, the first fault as a person meets it: a project checked
    /// and sent, Smoozy closed and opened, then submitted — with no pairs.**
    /// The kinds the pairs need were kept only in the run that checked, so
    /// the render looks for the pictures at the Mac's paths.
    #[test]
    fn a_project_checked_and_sent_is_submitted_with_its_remap_after_a_restart() {

        let scratch = Scratch::new("remapped-restart");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        std::fs::write(project.join("tex/wood.png"), b"wood").expect("a picture");
        std::fs::write(project.join("tex/stone.png"), b"stone").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(
            bytes_of(&root),
            a_document_of(&["//tex/wood.png", "//tex/stone.png", "//tex/gone.png"]),
        );
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system. The remap is unproved here.");
            return;
        };
        it_came_up_through_files(&desk, &root);
        let with = a_5_2();
        let (_, of) = a_check_on(&desk, &root, with.clone(), Vec::new());
        assert_eq!(a_send_on(&desk, &of, None, false).published, 2);

        smoozy_is_closed_and_opened(&desk, &scratch);
        let (came_to, materials, resolved) = a_submission_on(&desk, &of, &with);
        println!("SUBMITTED AFTER A RESTART: {came_to:?}");
        assert_eq!(materials.len(), 1, "one job, one material placed first");
        let pairs = the_pairs_of(&materials[0]);
        println!("PAIRS AFTER A RESTART: {pairs:#?}");
        let mirror = |name: &str| {
            the_mirror_of(&desk, &project.join(name))
                .to_string_lossy()
                .to_string()
        };
        assert_eq!(
            pairs,
            vec![
                (
                    "Image".to_string(),
                    "filepath".to_string(),
                    "//tex/stone.png".to_string(),
                    mirror("tex/stone.png")
                ),
                (
                    "Image".to_string(),
                    "filepath".to_string(),
                    "//tex/wood.png".to_string(),
                    mirror("tex/wood.png")
                ),
            ],
            "after a restart the render would look for the pictures at the Mac's paths"
        );
        assert!(resolved.contains(&mirror("tex/wood.png")), "{resolved:?}");
    }

    /// **Piece 7b, the second fault as a person meets it: a project uploaded
    /// through Files, checked and sent — and the band stays**, saying nobody
    /// has checked, in the run that checked and after a restart.
    #[test]
    fn a_project_uploaded_through_files_checked_and_sent_stops_the_warning() {

        let scratch = Scratch::new("through-files");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        std::fs::write(project.join("tex/wood.png"), b"wood").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&root), a_document_of(&["//tex/wood.png"]));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system.");
            return;
        };
        it_came_up_through_files(&desk, &root);
        let with = a_5_2();
        let (_, of) = a_check_on(&desk, &root, with.clone(), Vec::new());
        assert_eq!(a_send_on(&desk, &of, None, false).published, 1);
        let (_, of) = a_check_on(&desk, &root, with.clone(), Vec::new());
        assert!(the_standing_on(&desk, &of).every_row_confirmed_or_set_aside);

        let mut page = smoozy_desktop::surface::render_settings::WhatTheRunHolds::default();
        let record = the_check_for(&desk, &of);
        let in_this_run = the_band_on(&mut page, &of, &with, &record);
        println!("IN THE RUN THAT CHECKED: {in_this_run:?}");
        smoozy_is_closed_and_opened(&desk, &scratch);
        let mut page = smoozy_desktop::surface::render_settings::WhatTheRunHolds::default();
        let record = the_check_for(&desk, &of);
        let after_a_restart = the_band_on(&mut page, &of, &with, &record);
        println!("AFTER A RESTART: {after_a_restart:?}");
        assert_eq!(
            (in_this_run, after_a_restart),
            (None, None),
            "a complete check that left nothing unfound, and the band stays"
        );

        // **The safe side: an answer whose kind no reading wrote** (a record
        // written before version 29) gives no pair, so its stop is not let
        // stand.
        {
            let opened = desk.record.0.lock().expect("the record");
            let keeper = TheDependencyRecord::over(opened.connection());
            let answers: Vec<_> = keeper
                .the_answers_of(&of.holding)
                .expect("it reads")
                .into_iter()
                .map(|mut answer| {
                    answer.read_as.clear();
                    answer
                })
                .collect();
            keeper
                .the_answers_are_written(&answers)
                .expect("written again");
        }
        let record = the_check_for(&desk, &of);
        assert!(!record.the_check.a_stop_can_be_kept);
        let band = the_band_on(&mut page, &of, &with, &record);
        assert_eq!(band.as_deref(), Some("never"));
    }

    /// **Piece 7b, the third fault as a person meets it: two projects
    /// uploaded through Files share a picture; both are checked and the
    /// picture sent; the picture changes; a *Replace* from the first lays the
    /// new picture over it — and the second project's warning does not come
    /// back**, because nothing marks its standing.
    #[test]
    fn a_replace_over_a_picture_two_projects_from_files_share_brings_back_the_others_warning() {

        let scratch = Scratch::new("shared-files");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let first = project.join("first.blend");
        let second = project.join("second.blend");
        std::fs::write(&first, b"first").expect("a root");
        std::fs::write(&second, b"second").expect("a root");
        std::fs::write(project.join("tex/shared.png"), b"shared").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&first), a_document_of(&["//tex/shared.png"]));
        documents.insert(bytes_of(&second), a_document_of(&["//tex/shared.png"]));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system.");
            return;
        };
        it_came_up_through_files(&desk, &first);
        it_came_up_through_files(&desk, &second);
        let with = a_5_2();
        let (_, of_first) = a_check_on(&desk, &first, with.clone(), Vec::new());
        assert_eq!(a_send_on(&desk, &of_first, None, false).published, 1);
        let (_, of_second) = a_check_on(&desk, &second, with.clone(), Vec::new());
        assert!(the_standing_on(&desk, &of_second).every_row_confirmed_or_set_aside);
        let mut page = smoozy_desktop::surface::render_settings::WhatTheRunHolds::default();
        let record = the_check_for(&desk, &of_second);
        let after_its_check = the_band_on(&mut page, &of_second, &with, &record);
        println!("THE OTHER PROJECT AFTER ITS CHECK: {after_its_check:?}");

        std::fs::write(project.join("tex/shared.png"), b"shared, painted again")
            .expect("the change");
        let (_, of_first) = a_check_on(&desk, &first, with.clone(), Vec::new());
        assert_eq!(
            a_send_on(&desk, &of_first, Some(vec![0]), true).published,
            1
        );
        let marked = the_standing_on(&desk, &of_second).disturbed;
        println!("THE OTHER PROJECT'S STANDING MARKED DISTURBED: {marked}");
        let record = the_check_for(&desk, &of_second);
        let after_the_replace = the_band_on(&mut page, &of_second, &with, &record);
        println!("THE OTHER PROJECT AFTER THE REPLACE: {after_the_replace:?}");
        assert_eq!(
            (after_its_check, marked, after_the_replace.as_deref()),
            (None, true, Some("replaced")),
            "the band stopped after the check, and came back after the Replace"
        );
    }

    /// **Piece 7b: an upload through Files over a checked root, or over a
    /// picture a checked root's tree resolved, brings that root's warning
    /// back** — `File Browser` X7a and `File Browser` W11, performed on the local record
    /// (`smoozy_desktop::record::neighbouring_records::a_publication_through_files_is_marked`,
    /// which Files' upload calls at each publication). **Before 7b Files marked
    /// nothing**, and once a Files project's standing could stop the warning,
    /// the stop would have stood over a file the check no longer described.
    #[test]
    fn an_upload_through_files_over_a_root_or_its_picture_brings_the_warning_back() {
        use smoozy_desktop::record::neighbouring_records::a_publication_through_files_is_marked;

        let scratch = Scratch::new("files-marks");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let first = project.join("first.blend");
        let second = project.join("second.blend");
        std::fs::write(&first, b"first").expect("a root");
        std::fs::write(&second, b"second").expect("a root");
        std::fs::write(project.join("tex/shared.png"), b"shared").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&first), a_document_of(&["//tex/shared.png"]));
        documents.insert(bytes_of(&second), a_document_of(&["//tex/shared.png"]));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system.");
            return;
        };
        it_came_up_through_files(&desk, &first);
        it_came_up_through_files(&desk, &second);
        let with = a_5_2();
        let (_, of_first) = a_check_on(&desk, &first, with.clone(), Vec::new());
        assert_eq!(a_send_on(&desk, &of_first, None, false).published, 1);
        let (_, of_first) = a_check_on(&desk, &first, with.clone(), Vec::new());
        let (_, of_second) = a_check_on(&desk, &second, with.clone(), Vec::new());
        let mut page = smoozy_desktop::surface::render_settings::WhatTheRunHolds::default();
        for of in [&of_first, &of_second] {
            let record = the_check_for(&desk, of);
            assert_eq!(the_band_on(&mut page, of, &with, &record), None);
        }
        let mut readings = smoozy_render_settings::file_half::FileHalf::new();
        let published = |at: &[u8], readings: &mut smoozy_render_settings::file_half::FileHalf| {
            let record = desk.record.0.lock().expect("the record");
            a_publication_through_files_is_marked(
                record.connection(),
                &desk.machine_key,
                &desk.layout,
                readings,
                at,
                &smoozy_state::outcomes::UnitOutcome::Done,
            )
            .expect("the record answers")
        };

        // **The first root uploaded again through Files, edited.**
        let wrote = published(&bytes_of(&first), &mut readings);
        println!("AN UPLOAD OVER THE FIRST ROOT: {wrote:?}");
        let record = the_check_for(&desk, &of_first);
        assert_eq!(
            the_band_on(&mut page, &of_first, &with, &record).as_deref(),
            Some("replaced")
        );
        let record = the_check_for(&desk, &of_second);
        assert_eq!(the_band_on(&mut page, &of_second, &with, &record), None);

        // **The shared picture uploaded through Files over its mirror.**
        let wrote = published(
            &bytes_of(&the_mirror_of(&desk, &project.join("tex/shared.png"))),
            &mut readings,
        );
        println!("AN UPLOAD OVER THE SHARED PICTURE: {wrote:?}");
        assert!(the_standing_on(&desk, &of_second).disturbed);
        let record = the_check_for(&desk, &of_second);
        assert_eq!(
            the_band_on(&mut page, &of_second, &with, &record).as_deref(),
            Some("replaced")
        );
    }

    /// **The two battle projects, checked and sent with real Blender and then
    /// submitted: the pairs the placed material carries are the splash
    /// tester's own** (`…/scratchpad/results/remap-4.5-battle.py` and
    /// `remap-5.2-battle.py`), compared on what the file records and on where
    /// each lands under the asset area's `files` — the machine's root being
    /// the tester's in theirs and this desk's in ours.
    ///
    /// **Three differences are the rulings since the tester wrote them, and
    /// they are allowed by name**, and **Smoozy is closed and opened between
    /// the send and the Submit** (step 10c's dependency wave piece 7b): the bake output path `//` makes no pair
    /// (DW0-2, and `Dependency Upload & Remap` M22a); a reference recorded
    /// twice is one pair (`Dependency Upload & Remap` K3, M8); and a still picture with `#` in its name is
    /// a pair of its own rather than a set that found nothing (DW0-3). Run with
    /// `SMOOZY_DESK_WORK` set to the tester's `work` folder and
    /// `SMOOZY_DESK_RESULTS` to its `results` folder.
    #[test]
    #[ignore = "needs the desk's Blenders and the splash tester's folder"]
    fn the_battle_projects_are_submitted_with_the_testers_own_pairs() {

        let work = PathBuf::from(std::env::var("SMOOZY_DESK_WORK").expect("SMOOZY_DESK_WORK"));
        let results = std::env::var("SMOOZY_DESK_RESULTS")
            .map(PathBuf::from)
            .unwrap_or_else(|_| work.parent().expect("a parent").join("results"));
        let blender_52 = std::env::var("SMOOZY_DESK_BLENDER_52")
            .unwrap_or_else(|_| "/Applications/Blender.app/Contents/MacOS/Blender".to_string());
        let blender_45 = std::env::var("SMOOZY_DESK_BLENDER_45").unwrap_or_else(|_| {
            "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender".to_string()
        });
        let under_the_files_area = |at: &str| -> String {
            let from = at
                .find("/assets/files")
                .expect("under the asset area's files");
            at[from + "/assets/files".len()..].to_string()
        };
        let (opening, closing) = ("PAIRS = json.loads(r'''", "''')");
        for (battle, with, search) in [
            ("4.5", a_blender(&blender_45, 4, 5, 12), false),
            ("5.2", a_blender(&blender_52, 5, 2, 2), true),
        ] {
            let folder = work.join("battle").join(battle);
            let plan: serde_json::Value =
                serde_json::from_slice(&std::fs::read(folder.join("plan.json")).expect("the plan"))
                    .expect("it reads");
            let scene = PathBuf::from(plan["scene"].as_str().expect("the scene"));
            let scratch = Scratch::new(&format!("battle-remap-{battle}"));
            let desk = a_desk(&scratch, HowItReads::WithBlender).expect("an sftp-server");
            let search_folders = if search {
                vec![bytes_of(&folder)]
            } else {
                Vec::new()
            };
            // **Uploaded through Files, as the Creator's own projects are**,
            // so `File Browser` N6 names it (piece 7b).
            it_came_up_through_files(&desk, &scene);
            let (came_to, of) = a_check_on(&desk, &scene, with.clone(), search_folders);
            assert_eq!(
                came_to,
                TheCheckCameTo::TheSittingStands(Completion::Completed)
            );
            let sent = a_send_on(&desk, &of, None, false);
            assert_eq!((sent.failed, sent.did_not_finish), (0, 0), "{sent:?}");
            // **Smoozy closed and opened between the send and the Submit**
            // (piece 7b): the pairs come from the record alone.
            smoozy_is_closed_and_opened(&desk, &scratch);
            let started = std::time::Instant::now();
            let (came_to, materials, resolved) = a_submission_on(&desk, &of, &with);
            println!(
                "{battle} battle: submitted in {:?}: {came_to:?}",
                started.elapsed()
            );
            assert_eq!(materials.len(), 1, "one material, placed first");
            let ours: std::collections::BTreeSet<(String, String, String, String)> =
                the_pairs_of(&materials[0])
                    .into_iter()
                    .map(|(kind, setting, recorded, at)| {
                        (kind, setting, recorded, under_the_files_area(&at))
                    })
                    .collect();

            let written =
                std::fs::read_to_string(results.join(format!("remap-{battle}-battle.py")))
                    .expect("the tester's remap");
            let from = written.find(opening).expect("the pairs") + opening.len();
            let to = from + written[from..].find(closing).expect("the pairs' end");
            let theirs_as_written: Vec<serde_json::Value> =
                serde_json::from_str(&written[from..to]).expect("the pairs read");
            let as_text = |value: &serde_json::Value| {
                let bytes: Vec<u8> = value
                    .as_array()
                    .expect("bytes")
                    .iter()
                    .map(|byte| byte.as_u64().expect("a byte") as u8)
                    .collect();
                String::from_utf8_lossy(&bytes).to_string()
            };
            let mut the_bake_output_path = 0;
            let mut recorded_twice = 0;
            let mut theirs = std::collections::BTreeSet::new();
            for pair in &theirs_as_written {
                let kind = pair["kind"].as_str().expect("a kind").to_string();
                if kind == "BakeFolder" {
                    the_bake_output_path += 1;
                    continue;
                }
                if !theirs.insert((
                    kind,
                    pair["setting"].as_str().expect("a setting").to_string(),
                    as_text(&pair["expected"]),
                    under_the_files_area(&as_text(&pair["new_location"])),
                )) {
                    recorded_twice += 1;
                }
            }
            let only_ours: Vec<_> = ours.difference(&theirs).collect();
            let only_theirs: Vec<_> = theirs.difference(&ours).collect();
            println!(
                "{battle} battle: ours {}, theirs {} as written ({} distinct; the bake output \
                 path {}, recorded twice {}); only ours {only_ours:#?}; only theirs \
                 {only_theirs:#?}",
                ours.len(),
                theirs_as_written.len(),
                theirs.len(),
                the_bake_output_path,
                recorded_twice
            );
            assert!(only_theirs.is_empty(), "a pair the tester wrote is missing");
            assert!(
                only_ours
                    .iter()
                    .all(|(_, _, recorded, _)| recorded.contains('#')),
                "a pair the tester did not write, other than DW0-3's"
            );
            assert!(
                resolved.len() >= ours.len(),
                "the resolved set names every pair's mirror"
            );
        }
    }

    // ------------------------------- the 38's piece 7: item 27, the machine's digest

    /// **The 38's piece 7, item 27: a file standing at its mirror that no
    /// placement record names is read through on the server and not sent
    /// again** — `Dependency Upload & Remap` Q5's second route, the digest
    /// computed on the machine (`Dependency Upload & Remap` Q3). The way it
    /// meets a person: the record that would have said so is gone (a fresh
    /// install of Smoozy, another computer), or the file was put there through
    /// Files. **A different file at the mirror asks replace or stop**
    /// (`Dependency Upload & Remap` Q8) rather than being written over.
    #[test]
    fn a_file_already_at_its_mirror_that_no_record_names_is_not_sent_again() {

        let scratch = Scratch::new("read-through");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        let wood: Vec<u8> = (0..300_000u32).map(|at| (at % 251) as u8).collect();
        std::fs::write(project.join("tex/wood.png"), &wood).expect("a picture");
        std::fs::write(project.join("tex/stone.png"), b"stone").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(
            bytes_of(&root),
            a_document_of(&["//tex/wood.png", "//tex/stone.png"]),
        );
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system. The read-through is unproved here.");
            return;
        };
        // **Both already at their mirrors, with no record of either**: the
        // wood as it is, the stone as a different file.
        for (name, bytes) in [("tex/wood.png", wood.clone()), ("tex/stone.png", b"other".to_vec())] {
            let mirror = the_mirror_of(&desk, &project.join(name));
            std::fs::create_dir_all(mirror.parent().expect("a folder")).expect("the folder");
            std::fs::write(&mirror, bytes).expect("at its mirror");
        }
        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        let rows = the_rows_on(&desk, &of);
        println!("ROWS: {rows:#?}");
        let state_of = |recorded: &str| {
            rows.iter()
                .find(|row| row.1 == recorded)
                .map(|row| row.0)
                .expect("the row")
        };
        let before = the_transfers_written(&desk);
        let sent = a_send_on(&desk, &of, None, false);
        println!("SENT: {sent:?}");
        assert_eq!(
            state_of("//tex/wood.png"),
            RowState::AlreadyOnTheMachine,
            "the file at its mirror is the same file, and the check does not know it"
        );
        assert_eq!(sent.published, 0, "a file already on the server was sent again");
        assert!(
            sent.asks_to_replace.is_some(),
            "a different file at the stone's mirror is written over without asking"
        );
        assert_eq!(the_transfers_written(&desk), before, "nothing moved");
        assert_eq!(
            std::fs::read(the_mirror_of(&desk, &project.join("tex/stone.png"))).expect("there"),
            b"other",
            "the different file is left as it was"
        );
    }

    // ------------------------------- the 38's piece 7: item 7, could not be fixed

    /// **The 38's piece 7, `Dependency Upload & Remap` M16a: a linked `.blend`
    /// whose fixing could not be made reads *could not be fixed*, with its
    /// reason, and *Try again* makes it on its own** — no check run again,
    /// nothing sent again. The standing counts it, and nothing below the set is
    /// confirmed until it is fixed (V11).
    #[test]
    fn a_library_that_could_not_be_fixed_says_so_and_try_again_fixes_it_alone() {

        let scratch = Scratch::new("could-not-fix");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("lib/tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        let library = project.join("lib/props.blend");
        std::fs::write(&library, b"library").expect("the library");
        std::fs::write(project.join("lib/tex/wood.png"), b"wood").expect("a picture");
        let mut documents = BTreeMap::new();
        let library_document = serde_json::to_vec(&serde_json::json!({
            "references": [
                {"kind": "Image", "datablock": "wood.png", "setting": "filepath",
                 "recorded": b"//tex/wood.png".to_vec(), "scenes": {"Established": ["Scene"]}, "source": "FILE"}
            ],
            "flat_list": [], "weak_references": [], "observed_stand_ins": []
        }))
        .expect("it writes");
        documents.insert(
            bytes_of(&root),
            serde_json::to_vec(&serde_json::json!({
                "references": [
                    {"kind": "Library", "datablock": "props.blend", "setting": "filepath",
                     "recorded": b"//lib/props.blend".to_vec(), "scenes": {"Established": ["Scene"]}, "source": null}
                ],
                "flat_list": [], "weak_references": [], "observed_stand_ins": []
            }))
            .expect("it writes"),
        );
        let Some(probe) = a_desk(&scratch, HowItReads::FromAScript(BTreeMap::new())) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system. The fixing is unproved here.");
            return;
        };
        // **The library is read where this module places it, at its mirror.**
        documents.insert(bytes_of(&the_mirror_of(&probe, &library)), library_document);
        drop(probe);
        let desk = a_desk(&scratch, HowItReads::FromAScript(documents)).expect("an sftp-server");
        desk.machine.fixings_fail.store(true, Ordering::SeqCst);

        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        let sent = a_send_on(&desk, &of, None, false);
        assert_eq!((sent.published, sent.libraries_published), (1, 1), "{sent:?}");
        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        let rows = the_rows_on(&desk, &of);
        println!("ROWS: {rows:#?}");
        let at = rows
            .iter()
            .position(|row| row.1 == "//lib/props.blend")
            .expect("the library's row");
        assert_eq!(rows[at].0, RowState::CouldNotBeFixed, "{rows:#?}");
        assert!(rows.iter().any(|row| row.1 == "//tex/wood.png"), "the set was read below");
        let panel = the_panel(&desk.held.lock().expect("the run"), &of.server, &of.file, None);
        let drawn = format!("{:?}", panel.the_table.expect("a table"));
        assert!(drawn.contains("remap\\u{a0}failed"), "{drawn}");
        let standing = the_standing_on(&desk, &of);
        assert_eq!(standing.counts.could_not_be_fixed, 1);
        assert!(!standing.every_row_confirmed_or_set_aside);

        // **Try again**, on that row alone.
        desk.machine.fixings_fail.store(false, Ordering::SeqCst);
        let before = the_transfers_written(&desk);
        let again = a_send_on(&desk, &of, Some(vec![at]), false);
        println!("TRY AGAIN: {again:?}");
        assert_eq!(the_transfers_written(&desk), before, "nothing was sent again");
        assert_eq!(the_rows_on(&desk, &of)[at].0, RowState::AlreadyOnTheMachine);
        let fixings = {
            let record = desk.record.0.lock().expect("the record");
            TheDependencyRecord::over(record.connection())
                .the_fixings_on(&desk.machine_key)
                .expect("the fixings read")
        };
        assert_eq!(fixings.len(), 1);
        assert_eq!(
            fixings[0].points_at,
            vec![(
                smoozy_dependencies::bytes::RecordedLocation::as_the_reading_carried_it(b"//tex/wood.png"),
                MachineLocation::composed(bytes_of(&the_mirror_of(&desk, &project.join("lib/tex/wood.png")))),
            )],
            "the fixed copy points the set's own picture at its mirror"
        );
        assert!(Path::new(std::ffi::OsStr::from_bytes(fixings[0].fixed_copy.as_bytes())).is_file());
    }

    // ------------------- the tenth notes, note 4: the check runs again by itself

    /// **The Creator's tenth notes, note 4, on the stand-in machine: an upload
    /// of pictures alone ends with files arrived and starts one check, which
    /// reads the file once and finds every row on the server** —
    /// `Dependency Upload & Remap` G14 as widened 2026-10-17. Until then only
    /// a linked `.blend` going up started one, and this upload started none.
    #[test]
    fn an_upload_of_pictures_alone_starts_one_check() {
        use smoozy_desktop::surface::the_dependency_check::the_check_runs_again_after;

        let scratch = Scratch::new("checks-again-pictures");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        std::fs::write(project.join("tex/wood.png"), b"wood").expect("a picture");
        std::fs::write(project.join("tex/stone.png"), b"stone").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&root), a_document_of(&["//tex/wood.png", "//tex/stone.png"]));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system. The check after an upload is unproved here.");
            return;
        };
        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        let sent = a_send_on(&desk, &of, None, false);
        println!("SENT: {sent:?}");
        assert_eq!((sent.published, sent.libraries_published, sent.did_not_finish), (2, 0, 0));

        let read_before = desk.machine.readings_taken.load(Ordering::SeqCst);
        let mut checks = 0;
        the_check_runs_again_after(&sent, || {
            checks += 1;
            a_check_on(&desk, &root, a_5_2(), Vec::new());
        });
        assert_eq!(checks, 1, "an upload of pictures alone started no check");
        assert_eq!(
            desk.machine.readings_taken.load(Ordering::SeqCst) - read_before,
            1,
            "the check read the file once"
        );
        let rows = the_rows_on(&desk, &of);
        println!("ROWS: {rows:#?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.0 == RowState::AlreadyOnTheMachine), "{rows:#?}");
    }

    /// **The eighteenth notes' piece 3, `Dependency Upload & Remap` A9a: a
    /// press while a file is already on its way does not send it again** —
    /// the Creator's note 6: *"It has to know what is already en route via
    /// transfers, to avoid accidental duplicate uploads."* Another send of
    /// this run is carrying the stone to its landing place, as the run's
    /// register holds it while it runs. The press is handed both rows, as a
    /// panel drawn before that send began would hand them; it sends the wood
    /// alone, and the stone's landing place is not written. **Once the other
    /// send has ended, a press sends the stone.**
    #[test]
    fn a_press_while_a_file_is_on_its_way_does_not_send_it_again() {
        use smoozy_desktop::the_found_files_sent::{
            the_files_on_their_way_to, the_rows_left_to_upload, the_rows_the_upload_sends,
            the_sends_this_run, AFileSent, ASend, HowItEnded,
        };

        let scratch = Scratch::new("on-its-way");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        std::fs::write(project.join("tex/wood.png"), b"wood").expect("a picture");
        std::fs::write(project.join("tex/stone.png"), b"stone").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&root), a_document_of(&["//tex/wood.png", "//tex/stone.png"]));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system. A file on its way is unproved here.");
            return;
        };
        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        let stone = the_mirror_of(&desk, &project.join("tex/stone.png"));
        let wood = the_mirror_of(&desk, &project.join("tex/wood.png"));
        let request = Identity::issue(IdentityKind::Operation).expect("randomness");
        the_sends_this_run().lock().expect("the register").push(ASend {
            request: request.clone(),
            server: of.server.clone(),
            file: "/Users/someone/another.blend".to_string(),
            files: vec![AFileSent {
                landing: bytes_of(&stone),
                size: 5,
                transfer: None,
                ended: None,
            }],
        });

        // **The foot counts the wood alone; the press is still handed both.**
        let (left, handed) = {
            let held = desk.held.lock().expect("the run");
            let rows = &held.sittings[&keyed_as(&of)].rows;
            (
                the_rows_left_to_upload(rows, Some(&desk.layout), &the_files_on_their_way_to(&of.server)).len(),
                the_rows_the_upload_sends(rows).len(),
            )
        };
        assert_eq!((left, handed), (1, 2));
        let sent = a_send_on(&desk, &of, None, false);
        println!("SENT WHILE THE STONE IS ON ITS WAY: {sent:?}");
        assert_eq!((sent.published, sent.already_there, sent.failed, sent.did_not_finish), (1, 0, 0, 0));
        assert_eq!(std::fs::read(&wood).expect("the wood landed"), b"wood");
        assert!(!stone.exists(), "the stone was sent a second time while on its way");
        let rows = the_rows_on(&desk, &of);
        assert_eq!(rows.iter().find(|row| row.1 == "//tex/wood.png").expect("the wood").0, RowState::AlreadyOnTheMachine);
        assert_eq!(rows.iter().find(|row| row.1 == "//tex/stone.png").expect("the stone").0, RowState::FoundOnYourComputer);

        // **The other send ends — stopped — and the stone is free to go.**
        for one in the_sends_this_run().lock().expect("the register").iter_mut() {
            if one.request == request {
                one.files[0].ended = Some(HowItEnded::DidNotFinish);
            }
        }
        let sent = a_send_on(&desk, &of, None, false);
        println!("SENT ONCE THE OTHER SEND ENDED: {sent:?}");
        assert_eq!(sent.published, 1);
        assert_eq!(std::fs::read(&stone).expect("the stone landed"), b"stone");
        the_sends_this_run().lock().expect("the register").retain(|one| one.request != request);
    }

    /// **And an upload the user stopped starts none**, though a file of it
    /// arrived before the stop — the spec's *"an upload the user stopped … does
    /// not start it, since nothing resumes on its own"*. A small picture goes
    /// first and arrives; then *Stop* on the whole request, on Transfers' own
    /// act, while the large ones are under way.
    #[test]
    fn a_stopped_upload_starts_no_check() {
        use smoozy_desktop::surface::the_dependency_check::the_check_runs_again_after;
        use smoozy_desktop::the_found_files_sent::{
            the_found_files_go_up, the_rows_the_upload_sends, the_sends_this_run, HowItEnded,
            WhatIsSent,
        };

        let scratch = Scratch::new("checks-again-stopped");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        std::fs::write(project.join("tex/a_small.png"), b"small").expect("a small picture");
        let size: usize = 32 * 1024 * 1024;
        let large: Vec<u8> = (0..size).map(|at| (at % 251) as u8).collect();
        let mut recorded = vec!["//tex/a_small.png".to_string()];
        for at in 0..5 {
            let name = format!("tex/large_{at}.exr");
            std::fs::write(project.join(&name), &large).expect("a large picture");
            recorded.push(format!("//{name}"));
        }
        let recorded: Vec<&str> = recorded.iter().map(String::as_str).collect();
        let mut documents = BTreeMap::new();
        documents.insert(bytes_of(&root), a_document_of(&recorded));
        let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system. A stopped upload is unproved here.");
            return;
        };
        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        let rows = {
            let held = desk.held.lock().expect("the run");
            the_rows_the_upload_sends(&held.sittings[&keyed_as(&of)].rows)
        };
        let asked = WhatIsSent {
            of: of.clone(),
            run: desk.run.clone(),
            rows,
            replacing: false,
        };
        let door: Arc<dyn smoozy_desktop::uploading::TheRecordAnUploadWrites> = desk.record.clone();
        let sent = a_runtime().block_on(async {
            let point = APointLending::over(&desk.machine).await;
            let sending = the_found_files_go_up(&asked, &point.sessions, &desk.the_run, door, &desk.held);
            // **Stop, once the small picture has arrived** — the request
            // found in the run's register by its check, as Transfers finds it.
            let stopping = async {
                for _ in 0..20_000 {
                    let arrived = the_sends_this_run().lock().expect("the register").iter().find_map(|one| {
                        (one.server == of.server
                            && one.file == of.file
                            && one.files.iter().any(|file| file.ended == Some(HowItEnded::Published)))
                        .then(|| one.request.as_text())
                    });
                    if let Some(request) = arrived {
                        assert!(desk.the_run.the_user_stops(&request, None), "the request was under way");
                        return;
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                }
                panic!("no file of the upload arrived");
            };
            let (sent, ()) = tokio::join!(sending, stopping);
            sent
        });
        println!("SENT: {sent:?}");
        assert!(sent.published >= 1, "the small picture arrived before the stop: {sent:?}");
        assert!(sent.did_not_finish >= 1, "the stop landed on a file still to go: {sent:?}");

        let read_before = desk.machine.readings_taken.load(Ordering::SeqCst);
        let mut checks = 0;
        the_check_runs_again_after(&sent, || checks += 1);
        assert_eq!(checks, 0, "a stopped upload started a check");
        assert_eq!(desk.machine.readings_taken.load(Ordering::SeqCst), read_before);
    }

    /// **And a linked `.blend` the upload put on the machine still is read** —
    /// `Dependency Upload & Remap` G14's first case, now one case of the
    /// widened one: the check the upload starts reads the library at its
    /// mirror, its own reading, and what it points at appears alongside the
    /// rest.
    #[test]
    fn a_linked_blend_the_upload_put_up_is_read_by_the_check_it_starts() {
        use smoozy_desktop::surface::the_dependency_check::the_check_runs_again_after;

        let scratch = Scratch::new("checks-again-library");
        let project = scratch.at.join("project");
        std::fs::create_dir_all(project.join("lib/tex")).expect("the project");
        let root = project.join("scene.blend");
        std::fs::write(&root, b"blend").expect("the root");
        let library = project.join("lib/props.blend");
        std::fs::write(&library, b"library").expect("the library");
        std::fs::write(project.join("lib/tex/wood.png"), b"wood").expect("a picture");
        let mut documents = BTreeMap::new();
        documents.insert(
            bytes_of(&root),
            serde_json::to_vec(&serde_json::json!({
                "references": [
                    {"kind": "Library", "datablock": "props.blend", "setting": "filepath",
                     "recorded": b"//lib/props.blend".to_vec(), "scenes": {"Established": ["Scene"]}, "source": null}
                ],
                "flat_list": [], "weak_references": [], "observed_stand_ins": []
            }))
            .expect("it writes"),
        );
        let Some(probe) = a_desk(&scratch, HowItReads::FromAScript(BTreeMap::new())) else {
            println!("SKIPPED: no OpenSSH sftp-server on this system. The library's reading is unproved here.");
            return;
        };
        documents.insert(bytes_of(&the_mirror_of(&probe, &library)), a_document_of(&["//tex/wood.png"]));
        drop(probe);
        let desk = a_desk(&scratch, HowItReads::FromAScript(documents)).expect("an sftp-server");

        let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
        assert!(
            !the_rows_on(&desk, &of).iter().any(|row| row.1 == "//tex/wood.png"),
            "nothing below the library is read before it is on the machine"
        );
        let sent = a_send_on(&desk, &of, None, false);
        println!("SENT: {sent:?}");
        assert_eq!((sent.published, sent.libraries_published, sent.did_not_finish), (1, 1, 0));
        let read_before = desk.machine.readings_taken.load(Ordering::SeqCst);
        let mut checks = 0;
        the_check_runs_again_after(&sent, || {
            checks += 1;
            a_check_on(&desk, &root, a_5_2(), Vec::new());
        });
        assert_eq!(checks, 1);
        assert_eq!(
            desk.machine.readings_taken.load(Ordering::SeqCst) - read_before,
            2,
            "the file and the library it put up, each its own reading"
        );
        let rows = the_rows_on(&desk, &of);
        println!("ROWS: {rows:#?}");
        assert!(rows.iter().any(|row| row.1 == "//tex/wood.png"), "the library was not read below");
    }

    // ------------------------------- the 38's piece 7: item 37, carrying on

    /// **The 38's piece 7, item 37: a check's send cut by quitting carries on
    /// inside the part-sent file where the file has not changed** — `File
    /// Browser` K10 and K12, which say Files' own upload and the check's send
    /// are one act; **and starts over where it has.** The send is ended
    /// mid-file the way quitting ends it — the task dropped, the record left
    /// with no ending — Smoozy opened again on the same record, the check run
    /// again, and the upload pressed again.
    #[test]
    fn a_send_cut_by_quitting_carries_on_where_the_file_has_not_changed() {
        use smoozy_server_protocol::transfers::Ending;

        for changed in [false, true] {
            let scratch = Scratch::new(if changed { "carry-changed" } else { "carry-on" });
            let project = scratch.at.join("project");
            std::fs::create_dir_all(project.join("tex")).expect("the project");
            let root = project.join("scene.blend");
            std::fs::write(&root, b"blend").expect("the root");
            let big = project.join("tex/big.exr");
            let size: usize = 200 * 1024 * 1024;
            let bytes: Vec<u8> = (0..size).map(|at| (at % 253) as u8).collect();
            std::fs::write(&big, &bytes).expect("a large picture");
            let mut documents = BTreeMap::new();
            documents.insert(bytes_of(&root), a_document_of(&["//tex/big.exr"]));
            let Some(desk) = a_desk(&scratch, HowItReads::FromAScript(documents)) else {
                println!("SKIPPED: no OpenSSH sftp-server on this system. Carrying on is unproved here.");
                return;
            };
            let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());

            // **The first send, ended mid-file as quitting ends it.**
            let rows = {
                let held = desk.held.lock().expect("the run");
                smoozy_desktop::the_found_files_sent::the_rows_the_upload_sends(
                    &held.sittings[&keyed_as(&of)].rows,
                )
            };
            let asked = smoozy_desktop::the_found_files_sent::WhatIsSent {
                of: of.clone(),
                run: desk.run.clone(),
                rows,
                replacing: false,
            };
            let staged_when_quit = a_runtime().block_on(async {
                let point = APointLending::over(&desk.machine).await;
                let sessions = point.sessions.clone();
                let door: Arc<dyn smoozy_desktop::uploading::TheRecordAnUploadWrites> =
                    desk.record.clone();
                let held = Arc::new(Mutex::new(WhatTheRunHolds::default()));
                *held.lock().expect("the run") =
                    std::mem::take(&mut *desk.held.lock().expect("the run"));
                let the_run = Arc::new(smoozy_desktop::downloading::TheRunsTransfers::for_this_run());
                let sending = {
                    let (asked, held, the_run) = (asked.clone(), Arc::clone(&held), Arc::clone(&the_run));
                    tokio::spawn(async move {
                        smoozy_desktop::the_found_files_sent::the_found_files_go_up(
                            &asked, &sessions, &the_run, door, &held,
                        )
                        .await
                    })
                };
                // **Until a good part of the file is staged**, then quit.
                let staging = desk.layout.root().as_path().to_string();
                let mut staged = 0u64;
                for _ in 0..600 {
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    staged = a_staged_part_under(Path::new(&staging));
                    if staged > (size as u64) / 3 {
                        break;
                    }
                }
                sending.abort();
                let _ = sending.await;
                drop(point);
                *desk.held.lock().expect("the run") =
                    std::mem::take(&mut *held.lock().expect("the run"));
                staged
            });
            println!("STAGED WHEN QUIT: {staged_when_quit}");
            assert!(staged_when_quit > 0 && staged_when_quit < size as u64);
            if changed {
                let mut bytes = bytes.clone();
                bytes[0] ^= 0xff;
                std::fs::write(&big, &bytes).expect("the picture is saved again");
            }

            // **Smoozy opened again**, a new run on the same record.
            smoozy_is_closed_and_opened(&desk, &scratch);
            let desk = ADesk {
                run: Identity::issue(IdentityKind::Run).expect("randomness"),
                the_run: smoozy_desktop::downloading::TheRunsTransfers::for_this_run(),
                ..desk
            };
            let (_, of) = a_check_on(&desk, &root, a_5_2(), Vec::new());
            let written_before = desk.machine.bytes_written.load(Ordering::SeqCst);
            let sent = a_send_on(&desk, &of, None, false);
            let written = desk.machine.bytes_written.load(Ordering::SeqCst) - written_before;
            println!(
                "SECOND SEND ({}): {sent:?}, {written} bytes written of {size}",
                if changed { "changed" } else { "unchanged" }
            );
            assert_eq!(sent.published, 1, "{sent:?}");
            assert_eq!(
                std::fs::read(the_mirror_of(&desk, &big)).expect("it landed"),
                std::fs::read(&big).expect("the local file"),
                "the file that landed is not the file on this computer"
            );
            let records: Vec<(String, Option<Ending>)> = {
                let record = desk.record.0.lock().expect("the record");
                let mut read = record
                    .connection()
                    .prepare("SELECT transfer FROM transfer_record")
                    .expect("it reads");
                let ids: Vec<String> = read
                    .query_map([], |row| row.get::<_, String>(0))
                    .expect("it reads")
                    .map(|id| id.expect("an id"))
                    .collect();
                ids.into_iter()
                    .filter_map(|id| {
                        let transfer = Identity::read_back(IdentityKind::Transfer, &id)?;
                        smoozy_desktop::record::transfer_records::read(record.connection(), &transfer)
                            .ok()
                            .flatten()
                            .map(|one| (id, one.ending))
                    })
                    .collect()
            };
            println!("RECORDS: {records:#?}");
            if changed {
                assert!(written >= size as u64, "a changed file is sent again from its start");
            } else {
                // `Server protocol and transfers` S12 step 4 re-sends up to one checkpoint interval, which ends on the write that reaches 4 MiB, so up to one 261,120-byte write past it.
                const ONE_WRITE: u64 = 261_120;
                let one_interval =
                    smoozy_server_protocol::transfers::moving::THE_CHECKPOINT_INTERVAL + ONE_WRITE;
                assert!(
                    written <= size as u64 - staged_when_quit + one_interval,
                    "the part-sent file was sent again from the start rather than carried on"
                );
                assert_eq!(records.len(), 1, "the part was taken up rather than a fresh transfer arranged");
            }
        }
    }

    /// **The bytes of the largest staged part under the machine's root** —
    /// `Server protocol and transfers` S1's staging, in `File Browser` X12's
    /// reserved folder beside each landing.
    fn a_staged_part_under(root: &Path) -> u64 {
        let mut largest = 0u64;
        let mut folders = vec![root.to_path_buf()];
        while let Some(folder) = folders.pop() {
            let Ok(entries) = std::fs::read_dir(&folder) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    folders.push(path);
                } else if path
                    .components()
                    .any(|part| part.as_os_str() == smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER)
                {
                    largest = largest.max(entry.metadata().map(|meta| meta.len()).unwrap_or(0));
                }
            }
        }
        largest
    }

    // ------------------------------- the 38's piece 7: a set linked into a shot

    /// **The piece's fixture**, `App/Dependency Upload & Remap/tests/a linked
    /// set/`: `shots/shot.blend` links the collection `Set` from
    /// `//../assets/set.blend`, whose cube reads `//tex/checker.png`.
    fn the_linked_set() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../Dependency Upload & Remap/tests/a linked set")
    }

    fn a_folder_copied(from: &Path, to: &Path) {
        std::fs::create_dir_all(to).expect("the folder is made");
        for entry in std::fs::read_dir(from).expect("the folder reads").flatten() {
            let at = to.join(entry.file_name());
            if entry.path().is_dir() {
                a_folder_copied(&entry.path(), &at);
            } else {
                std::fs::copy(entry.path(), &at).expect("the file is copied");
            }
        }
    }

    /// **The shot uploaded through Files from `source` to `landed`**, as
    /// `File Browser` N6 then answers for it.
    fn it_came_up_through_files_from(desk: &ADesk, source: &Path, landed: &Path) {
        use smoozy_file_browser::request::{
            Disposition, EnumerationEntry, KeepsTheRequest, UploadRequest,
        };
        let size = std::fs::metadata(source).expect("the source").len();
        let mut entry = EnumerationEntry::enumerated(bytes_of(source), bytes_of(landed), size);
        entry.disposition = Some(Disposition::ToSend);
        entry.outcome = Some(smoozy_state::outcomes::UnitOutcome::Done);
        entry.outcome_written_at = Some(std::time::SystemTime::now());
        let request = UploadRequest {
            record: Identity::issue(IdentityKind::Operation).expect("randomness"),
            server_row: desk.machine_key.server_row.clone(),
            installation: Some(desk.machine_key.installation.clone()),
            destination: bytes_of(landed.parent().expect("a folder")),
            entries: vec![entry],
            collision_answer: None,
            telling: None,
            opened_by_run: desk.run.clone(),
        };
        let opened = desk.record.0.lock().expect("the record");
        smoozy_desktop::record::upload_requests::UploadRequests::over(opened.connection())
            .write_it_down(&request)
            .expect("the upload is written down");
    }

    /// **One check of the machine's copy `on_the_machine`**, its answers keyed
    /// under the source it came up from — which is what a check of a file
    /// uploaded through Files is.
    fn a_check_of_the_machine_copy(
        desk: &ADesk,
        on_the_machine: &Path,
        source: &Path,
        with: BlenderIdentity,
    ) -> (TheCheckCameTo, TheSittingIsOf) {
        let sitting = TheSittingIsOf {
            server: desk.machine_key.server_row.as_text(),
            file: String::from_utf8(bytes_of(on_the_machine)).expect("the path is text"),
            root: MachineLocation::composed(bytes_of(on_the_machine)),
            holding: HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(
                &bytes_of(source),
            )),
            with,
            machine: desk.machine_key.clone(),
            layout: Some(desk.layout.clone()),
        };
        let came_to = a_runtime().block_on(async {
            let point = APointLending::over(&desk.machine).await;
            let mut on_the_machine = ThisServersMachine {
                sessions: &point.sessions,
                package_area: desk.package_area.to_str().expect("text"),
                server: &sitting.server,
            };
            the_check_runs(
                &mut on_the_machine,
                desk.record.as_ref(),
                &TheUsersOwnComputer,
                &desk.held,
                &sitting,
                &|| {},
            )
            .await
        });
        (came_to, sitting)
    }

    /// **What one desk render of the machine's shot shows**, with the job's
    /// remap material applied first: whether the set's cube arrived with its
    /// eight corners, and whether the picture it reads is a file that is
    /// there. **A controlled test, labelled as one**: the render worker's
    /// launch is not run at the desk (macOS publishes no per-process record),
    /// so this runs the same Blender over the same file with the material's
    /// own fragment, which is the part of the launch the remap is.
    struct WhatTheRenderShowed {
        the_cube_has_its_corners: bool,
        the_picture_is_there: bool,
        said: String,
    }

    fn the_shot_rendered(
        blender: &str,
        shot: &Path,
        material: Option<&smoozy_dependencies::remap::RemapMaterial>,
        out: &Path,
    ) -> WhatTheRenderShowed {
        let folder = out.parent().expect("a folder");
        std::fs::create_dir_all(folder).expect("the folder");
        let material_script = folder.join("material.py");
        std::fs::write(
            &material_script,
            match material {
                Some(material) => smoozy_dependencies::acting::material::the_material_python(material),
                None => "pass\n".to_string(),
            },
        )
        .expect("the material is written");
        let probe = folder.join("probe.py");
        std::fs::write(
            &probe,
            "import bpy, os\n\
             crate = bpy.data.objects.get('Crate')\n\
             corners = len(crate.data.vertices) if crate is not None and crate.data is not None else 0\n\
             there = False\n\
             for image in bpy.data.images:\n    \
                 at = bpy.path.abspath(image.filepath, library=image.library)\n    \
                 print('SMOOZY-PROBE picture', image.filepath, '->', at, os.path.exists(at))\n    \
                 there = there or os.path.exists(at)\n\
             for library in bpy.data.libraries:\n    \
                 print('SMOOZY-PROBE library', library.filepath)\n\
             print('SMOOZY-PROBE corners', corners)\n\
             print('SMOOZY-PROBE there', there)\n",
        )
        .expect("the probe is written");
        let ran = std::process::Command::new(blender)
            .arg("-b")
            .arg("--factory-startup")
            .arg(shot)
            .arg("--python-exit-code")
            .arg("1")
            .arg("--python")
            .arg(&material_script)
            .arg("--python")
            .arg(&probe)
            .arg("-o")
            .arg(out)
            .arg("-F")
            .arg("PNG")
            .arg("-f")
            .arg("1")
            .output()
            .expect("Blender starts");
        let said = String::from_utf8_lossy(&ran.stdout).to_string();
        WhatTheRenderShowed {
            the_cube_has_its_corners: said.contains("SMOOZY-PROBE corners 8"),
            the_picture_is_there: said.contains("SMOOZY-PROBE there True"),
            said: said
                .lines()
                .filter(|line| line.contains("SMOOZY-") || line.contains("Saved"))
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }

    /// **The 38's piece 7, as a person meets item 7: a shot with a set linked
    /// into it, uploaded through Files, checked, sent and submitted.** The
    /// library has to reach the server and be fixed, the material has to point
    /// the shot at the fixed copy, and the render — with the artist's own
    /// computer out of reach, as it is from a rental — has to show the set's
    /// cube with its picture. Run by hand at the desk; `SMOOZY_DESK_BLENDER`
    /// names the Blender (5.2.2 by default) and `SMOOZY_DESK_RENDERS` a folder
    /// the two renders are left in for a person to look at.
    #[test]
    #[ignore = "needs a desk Blender"]
    fn a_set_linked_into_a_shot_is_sent_fixed_and_rendered_at_the_desk() {

        let blender = std::env::var("SMOOZY_DESK_BLENDER")
            .unwrap_or_else(|_| "/Applications/Blender.app/Contents/MacOS/Blender".to_string());
        let version = std::env::var("SMOOZY_DESK_VERSION").unwrap_or_else(|_| "5.2.2".to_string());
        let mut numbers = version.split('.').map(|part| part.parse::<u32>().expect("a number"));
        let with = a_blender(
            &blender,
            numbers.next().expect("major"),
            numbers.next().expect("minor"),
            numbers.next().expect("patch"),
        );
        let scratch = Scratch::new("linked-set");
        let user = scratch.at.join("user");
        let project = user.join("project");
        a_folder_copied(&the_linked_set(), &project);
        let source = project.join("shots/shot.blend");
        let desk = a_desk(&scratch, HowItReads::WithBlender).expect("an sftp-server");
        // **Uploaded through Files**, alone, to a folder of the user's choosing.
        // `a_desk`'s machine root.
        let on_the_machine = scratch.at.join("home/root/work/shot.blend");
        std::fs::create_dir_all(on_the_machine.parent().expect("a folder")).expect("the folder");
        std::fs::copy(&source, &on_the_machine).expect("the shot is on the machine");
        it_came_up_through_files_from(&desk, &source, &on_the_machine);

        let mut seen = Vec::new();
        let (came_to, of) = a_check_of_the_machine_copy(&desk, &on_the_machine, &source, with.clone());
        seen.push(format!("first check: {came_to:?}; rows {:#?}", the_rows_on(&desk, &of)));
        let sent = a_send_on(&desk, &of, None, false);
        seen.push(format!("first send: {sent:?}"));
        let (came_to, of) = a_check_of_the_machine_copy(&desk, &on_the_machine, &source, with.clone());
        seen.push(format!("second check: {came_to:?}; rows {:#?}", the_rows_on(&desk, &of)));
        let sent = a_send_on(&desk, &of, None, false);
        seen.push(format!("second send: {sent:?}"));
        let standing = the_standing_on(&desk, &of);
        seen.push(format!("standing after the second send: {:?}", standing.counts));

        let library = project.join("assets/set.blend");
        let the_library_is_at_its_mirror = std::fs::read(the_mirror_of(&desk, &library)).ok()
            == Some(std::fs::read(&library).expect("the library"));
        let fixings = {
            let record = desk.record.0.lock().expect("the record");
            TheDependencyRecord::over(record.connection())
                .the_fixings_on(&desk.machine_key)
                .expect("the fixings read")
        };
        seen.push(format!("fixings: {fixings:#?}"));

        let (came_to, materials, _) = a_submission_on(&desk, &of, &with);
        seen.push(format!("submitted: {came_to:?}"));
        let pairs = materials.first().map(the_pairs_of).unwrap_or_default();
        seen.push(format!("pairs: {pairs:#?}"));
        let the_library_pair_names_a_fixed_copy = pairs.iter().any(|(kind, _, recorded, at)| {
            kind == "Library" && recorded == "//../assets/set.blend" && at.contains("/fixed/")
        });

        // **The artist's computer is out of reach**, as it is from a rental.
        let away = scratch.at.join("user-away");
        std::fs::rename(&user, &away).expect("the user's folder is moved away");
        let renders = std::env::var("SMOOZY_DESK_RENDERS")
            .map(PathBuf::from)
            .unwrap_or_else(|_| scratch.at.join("renders"));
        let rendered = the_shot_rendered(
            &blender,
            &on_the_machine,
            materials.first(),
            &renders.join(format!("linked-set-{version}-#")),
        );
        std::fs::rename(&away, &user).expect("the user's folder is put back");
        seen.push(format!("render:\n{}", rendered.said));
        println!("{}", seen.join("\n\n"));

        assert!(the_library_is_at_its_mirror, "the library was never sent (and M2: the copy sent is the user's own, untouched)");
        assert_eq!(fixings.len(), 1, "the library was never fixed");
        assert!(
            std::path::Path::new(std::ffi::OsStr::from_bytes(fixings[0].fixed_copy.as_bytes())).is_file(),
            "the fixed copy is at its address"
        );
        assert!(
            standing.every_row_confirmed_or_set_aside,
            "every row, the one inside the set among them, is on the server: {:?}",
            standing.counts
        );
        assert!(
            the_library_pair_names_a_fixed_copy,
            "the material does not point the shot at the fixed copy"
        );
        assert!(rendered.the_cube_has_its_corners, "the render is missing the set");
        assert!(rendered.the_picture_is_there, "the set's picture is not found");
    }
}

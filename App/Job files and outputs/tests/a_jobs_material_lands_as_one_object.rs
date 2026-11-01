//! **The publication contract, run rather than argued.**
//!
//! **What the far end is.** A real SSH connection, opened by Smoozy's own
//! transport on the loopback address, carrying a **subsystem session served by
//! OpenSSH's own `sftp-server`** — the same program a rental's own file-transfer
//! server is. Nothing is rented; nothing on the Creator's machine outside one
//! temporary directory is touched.
//!
//! **What that is worth, and what it is not.** It is M5's single rename, M6's
//! base operation refusing an existing target, N9's plain create-directory
//! refusing an existing folder and making no parents, and K4's sweep — against a
//! server this project did not write. **It is not a rented Linux machine**, and
//! it says nothing about a provider's own image.
//!
//! **One request kind here is observed rather than read.** The subsystem's
//! directory removal is what K4's sweep needs, and no copy of the file-transfer
//! draft was on this machine to read its number out of. **So it is established
//! by running it**: OpenSSH's own `sftp-server -Q requests` lists `rmdir`
//! between `mkdir` and `realpath` (this Mac, 2026-09-03), and the test below
//! removes an empty directory with it, is refused where the directory still
//! holds a file, and is told *not there* where it does not exist. **If the
//! number were wrong every one of those would fail.**
//!
//! **Where `sftp-server` is not on this system the tests say so and stop.**
//! **And on Windows they say so and stop for a second reason**: the layout is a
//! Linux rule about the rented machine — *"Three operating systems on the
//! user's end, Linux alone on the rented end"* — so a Windows path is not a root
//! this document can hold.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan. Every other document's marker is written with that
//! document — at the site, or in the file's own header list of markers that
//! belong elsewhere. **Where a marker is a clause of this crate's own plan
//! *and* of the document meant, the header list is not enough and the document
//! is written out at the site**, because a bare one there does not fail to
//! resolve: it resolves to a real and wrong clause, which nobody reading it
//! can tell from a right one.
//!
//! **A section number is a marker for the whole of that rule.** `§1.3` and
//! `§9.2` are defined in as many documents as `M1` is, and this crate's plan has
//! a real `§1.3` and a real `§9.2` of its own — so a bare section number means a
//! section of this crate's plan, every other document's section is written
//! with that document, and where the number exists in both the document is
//! written out at the site.
//!
//! **The plan here is
//! `Preparation/Architecture/Job files and outputs.md`, and the words above
//! are the words `src/lib.rs` carries.** They are repeated because a file under
//! `tests/` is its own crate and never sees that one, and two of the three
//! sites the step 4 gate's fourth run caught were in a file under `tests/`.
//! Keep the wording identical wherever it appears.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use russh::server::{Auth, Msg, Server as _, Session};
use russh::{Channel, ChannelId};
use smoozy_job_files::acting::TheMachinesDirectories;
use smoozy_job_files::attempts::{AttemptRecord, TheMapInForce};
use smoozy_job_files::cleanup::{self, ASubmission, WhatTheDesktopRecordSays, WhatTheSweepDid};
use smoozy_job_files::layout::{Area, MachineRoot, TheLayout, WhereThePackageAreaSits, EVERY_AREA};
use smoozy_job_files::manifest::{
    AnchorState, CarriedAndNotInterpretedHere, FrameRange, SceneField, SubmissionManifest,
    TheBlenderToRenderBy, TheCardMakeAtSubmission, TheRecordedReading, MANIFEST_FILE_NAME,
};
use smoozy_job_files::publication::{
    place_a_jobs_material, write_an_attempt_record, AJobsPaths, HowThePlacementEnded,
};
use smoozy_server_protocol::subsystem::client::Subsystem;
use smoozy_server_protocol::transfers::SourceReading;
use smoozy_server_protocol::transport::ssh::{open, Connection};
use smoozy_server_protocol::transport::{
    Credential, DecideHostTrust, Endpoint, Figures, HostTrust,
};
use smoozy_state::identities::{Identity, IdentityKind};
use tokio::net::TcpListener;

// ------------------------------------------- where OpenSSH's own server is

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
    () => {{
        if cfg!(windows) {
            println!(
                "SKIPPED: the directory layout is a Linux rule about the rented \
                 machine, so a Windows path is no root of it. Unproved here."
            );
            return;
        }
        match the_machines_own_file_transfer_server() {
            Some(path) => path,
            None => {
                println!(
                    "SKIPPED: no OpenSSH sftp-server on this system ({}). \
                     The publication contract is unproved here.",
                    std::env::consts::OS
                );
                return;
            }
        }
    }};
}

// ------------------------------------------------------------- the machine

struct Machine {
    sftp_server: PathBuf,
    channels: std::collections::HashMap<ChannelId, Channel<Msg>>,
}

impl Clone for Machine {
    fn clone(&self) -> Machine {
        Machine {
            sftp_server: self.sftp_server.clone(),
            channels: std::collections::HashMap::new(),
        }
    }
}

impl russh::server::Server for Machine {
    type Handler = Self;
    fn new_client(&mut self, _address: Option<std::net::SocketAddr>) -> Self {
        self.clone()
    }
}

impl russh::server::Handler for Machine {
    type Error = russh::Error;

    async fn auth_publickey(
        &mut self,
        _user: &str,
        _key: &russh::keys::PublicKey,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        reply: russh::server::ChannelOpenHandle,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        reply.accept().await;
        self.channels.insert(channel.id(), channel);
        Ok(())
    }

    /// **The machine's own file-transfer server, on a subsystem session.**
    ///
    /// **The channel is held rather than dropped**, which is what keeps the
    /// flow control honest — the same lesson `Server protocol and transfers`'
    /// own test records.
    async fn subsystem_request(
        &mut self,
        channel: ChannelId,
        name: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        if name != "sftp" {
            session.channel_failure(channel)?;
            return Ok(());
        }
        session.channel_success(channel)?;
        let Some(taken) = self.channels.remove(&channel) else {
            return Ok(());
        };
        let program = self.sftp_server.clone();
        tokio::spawn(async move {
            let mut child = match tokio::process::Command::new(program)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
            {
                Ok(child) => child,
                Err(_) => return,
            };
            let mut to_the_server = child.stdin.take().expect("piped");
            let mut from_the_server = child.stdout.take().expect("piped");
            let stream = taken.into_stream();
            let (mut reading, mut writing) = tokio::io::split(stream);
            let inward = tokio::spawn(async move {
                let _ = tokio::io::copy(&mut reading, &mut to_the_server).await;
            });
            let _ = tokio::io::copy(&mut from_the_server, &mut writing).await;
            inward.abort();
            let _ = child.kill().await;
        });
        Ok(())
    }
}

struct RunningMachine {
    address: std::net::SocketAddr,
    _shut_down: russh::server::RunningServerHandle,
}

async fn a_machine(host_key_file: &Path, sftp_server: PathBuf) -> RunningMachine {
    let host_key = russh::keys::load_secret_key(host_key_file, None).expect("ssh-keygen wrote it");
    let config = Arc::new(russh::server::Config {
        keys: vec![host_key],
        auth_rejection_time: Duration::from_millis(0),
        auth_rejection_time_initial: Some(Duration::from_millis(0)),
        ..russh::server::Config::default()
    });
    let socket: &'static TcpListener = Box::leak(Box::new(
        TcpListener::bind(("127.0.0.1", 0)).await.expect("loopback"),
    ));
    let address = socket.local_addr().expect("bound");
    let machine: &'static mut Machine = Box::leak(Box::new(Machine {
        sftp_server,
        channels: std::collections::HashMap::new(),
    }));
    let running = machine.run_on_socket(config, socket);
    let shut_down = running.handle();
    tokio::spawn(running);
    RunningMachine {
        address,
        _shut_down: shut_down,
    }
}

// ---------------------------------------------------------------- the desk

/// The scratch sits under cargo's own directory for integration tests rather
/// than under the system temporary area, because **L3 refuses a root under a
/// shared temporary area** and on a Linux runner the system temporary area is
/// exactly `/tmp`.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
            "job-files-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("a clock")
                .as_nanos()
        ));
        std::fs::create_dir_all(&path).expect("a temporary directory can be made");
        Scratch(path)
    }

    fn a_key(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        let made = std::process::Command::new("ssh-keygen")
            .args(["-t", "ed25519", "-N", "", "-C", "smoozy job files test", "-f"])
            .arg(&path)
            .output()
            .expect("ssh-keygen is on the path");
        assert!(made.status.success());
        path
    }

    fn as_the_mount(&self) -> String {
        self.0.to_string_lossy().to_string()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct Approves;
impl DecideHostTrust for Approves {
    fn decide(&mut self, _offered: &[u8]) -> HostTrust {
        HostTrust::Proceed
    }
}

async fn a_connection(machine: &RunningMachine, key: &Path) -> Connection {
    open(
        &Endpoint {
            host: machine.address.ip().to_string(),
            port: machine.address.port(),
            username: "whoever".to_string(),
        },
        Credential {
            private_key_file: key.to_path_buf(),
            passphrase: None,
        },
        Figures {
            interval: Duration::from_secs(15),
            unanswered_before_the_break: 3,
        },
        Box::new(Approves),
    )
    .await
    .expect("the loopback machine takes the connection")
}

async fn a_subsystem(
    connection: &Connection,
) -> Subsystem<russh::ChannelStream<russh::client::Msg>> {
    let channel = connection
        .open_a_subsystem_channel()
        .await
        .expect("the machine serves the file-transfer subsystem");
    Subsystem::negotiate(channel)
        .await
        .expect("the subsystem negotiates")
}

// --------------------------------------------------------------- the pieces

fn a_layout(scratch: &Scratch) -> TheLayout {
    TheLayout::of(
        MachineRoot::chosen(&format!("{}/smoozy", scratch.as_the_mount())).expect("a root"),
        WhereThePackageAreaSits::UnderTheRoot,
    )
}

/// **N11: the five areas exist before the first job**, made one segment at a
/// time with N9's plain form, so no area is ever created deep.
async fn hand_over(machine: &mut impl TheMachinesDirectories, layout: &TheLayout) {
    machine
        .make_one_directory(layout.root().as_path())
        .await
        .expect("the root");
    for area in EVERY_AREA {
        machine
            .make_one_directory(&layout.area(area))
            .await
            .expect("an area");
    }
}

fn a_manifest(job: &Identity, paths: &AJobsPaths) -> SubmissionManifest {
    SubmissionManifest::composed(
        job,
        None,
        TheBlenderToRenderBy {
            version: "4.5.9".to_string(),
            build_path: "/ephemeral/smoozy/blender/blender-4.5.9/blender".to_string(),
        },
        SceneField {
            name: "Scene".to_string(),
            anchor: AnchorState::ToAnchor {
                provenance: "the user typed it".to_string(),
            },
        },
        FrameRange::TheUsersParsedIntegers(vec![1, 2, 3]),
        CarriedAndNotInterpretedHere(serde_json::json!({"samples": 128})),
        CarriedAndNotInterpretedHere(serde_json::json!({"remap": "asset-set-1"})),
        CarriedAndNotInterpretedHere(serde_json::json!({"reporting": "everything"})),
        "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
        SourceReading {
            size: 4096,
            inode: "912".to_string(),
            modification_time: 1_756_800_000,
            change_time: 1_756_800_001,
        },
        TheRecordedReading::ThereIsNone,
        // **M1b's card make**, as the desktop composes it from the hardware
        // reading `Servers` holds for an ordinary rented GPU machine.
        TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
        smoozy_job_files::manifest::WhereTheJobOutputFolderIs::named_by(paths),
    )
}

fn a_job() -> Identity {
    Identity::issue(IdentityKind::Job).expect("randomness")
}

// ----------------------------------------------------------------- the runs

#[tokio::test]
async fn a_jobs_material_lands_as_one_object_on_a_real_machine() {
    let _deadline = TheTestsDeadline::started();
    let server = the_server_or_say_so!();
    let scratch = Scratch::new("lands");
    let machine = a_machine(&scratch.a_key("host"), server).await;
    let connection = a_connection(&machine, &scratch.a_key("client")).await;
    let mut subsystem = a_subsystem(&connection).await;

    let layout = a_layout(&scratch);
    hand_over(&mut subsystem, &layout).await;

    // **L11 and L12 against the real machine**: a real write of at least one
    // byte at the exact path, read back, and nothing left behind. **Never a
    // writability test and never an empty file** — on a full filesystem both of
    // those succeed.
    assert_eq!(
        smoozy_job_files::placement::confirm_the_layout(&mut subsystem, &layout).await,
        vec![(
            layout.root().as_path().to_string(),
            smoozy_job_files::placement::WhatTheConfirmationFound::ItIsPlaced
        )]
    );

    let job = a_job();
    // **N13's own list, on a real filesystem, in a name Smoozy composed.** A
    // leading dash, a space, a colon, an emoji and letters outside the Latin
    // alphabet — every one of them reached disk unaltered on the rental, and
    // **§1.1 is why it is safe here: the path crosses as a protocol field and
    // no command string exists in this crate at all.**
    let paths = AJobsPaths::composed(&layout, &job, Some("-Кухня: 4k 🙂")).expect("paths");
    let manifest = a_manifest(&job, &paths);

    // **The final name does not exist before the publication**, checked from the
    // machine rather than from what the desk believes.
    assert!(subsystem
        .stat(paths.job_record_directory.as_bytes())
        .await
        .is_err());

    assert_eq!(
        place_a_jobs_material(&mut subsystem, &paths, &manifest).await,
        HowThePlacementEnded::ItLanded
    );

    // The record is there, the staging name is gone, and the manifest inside it
    // reads back as the one that was composed.
    assert!(subsystem
        .stat(paths.job_record_directory.as_bytes())
        .await
        .is_ok());
    assert!(
        subsystem
            .stat(paths.staging_directory.as_bytes())
            .await
            .is_err(),
        "the rename is the cleanup and the staging name must be gone"
    );
    let read = subsystem
        .read_a_whole_file(&format!(
            "{}/{MANIFEST_FILE_NAME}",
            paths.job_record_directory
        ))
        .await
        .expect("the manifest");
    assert_eq!(SubmissionManifest::read_back(&read).unwrap(), manifest);

    // **The job output folder is there and holds nothing** — L7, and it is what
    // *show outputs* opens on a job that has written nothing yet.
    assert!(subsystem
        .stat(paths.job_output_folder.as_bytes())
        .await
        .is_ok());
    assert!(subsystem
        .list_one_directory(&paths.job_output_folder)
        .await
        .expect("a listing")
        .is_empty());

    // **The name landed byte for byte**, read back off the machine's own
    // listing rather than off what the desk composed.
    let inside = subsystem
        .list_one_directory(&layout.area(Area::Job))
        .await
        .expect("a listing");
    let expected = paths
        .job_output_folder
        .rsplit('/')
        .next()
        .expect("a name")
        .as_bytes()
        .to_vec();
    assert!(inside.contains(&expected), "the name did not survive");

    // **M8's attempt record, durable before the render starts.**
    let record = AttemptRecord {
        number: 1,
        frame_list_in_the_order_it_was_written: vec![3, 1, 2],
        map_in_force: TheMapInForce::NoneWasComposedForThisAttempt,
        file_identity_at_this_attempt: manifest.file_identity_at_submission.clone(),
        the_file_changed_before_this_attempt: false,
        a_frame_an_earlier_attempt_wrote_was_kept: false,
    };
    write_an_attempt_record(&mut subsystem, &paths, &record)
        .await
        .expect("the attempt record");
    let read = subsystem
        .read_a_whole_file(&format!("{}/attempt-1.json", paths.job_record_directory))
        .await
        .expect("the attempt record back");
    assert_eq!(AttemptRecord::read_back(&read).unwrap(), record);
}

#[tokio::test]
async fn the_two_refusals_this_document_leans_on_are_the_machines_own() {
    let _deadline = TheTestsDeadline::started();
    let server = the_server_or_say_so!();
    let scratch = Scratch::new("refusals");
    let machine = a_machine(&scratch.a_key("host"), server).await;
    let connection = a_connection(&machine, &scratch.a_key("client")).await;
    let mut subsystem = a_subsystem(&connection).await;

    let layout = a_layout(&scratch);
    hand_over(&mut subsystem, &layout).await;

    // **N9: the plain create-directory operation refuses an existing folder.**
    // The make-parents form returns success against one, which is the silent
    // collision path — and there is no make-parents operation to reach for.
    let twice = format!("{}/twice", layout.area(Area::Job));
    subsystem.make_one_directory(&twice).await.expect("once");
    assert!(subsystem.make_one_directory(&twice).await.is_err());

    // **And it makes no parents.**
    let deep = format!("{}/not/there/yet", layout.area(Area::Job));
    assert!(subsystem.make_one_directory(&deep).await.is_err());

    // **M6: the base rename refuses to replace an existing target**, which is
    // why a job's material needs no extension to be published safely.
    let one = format!("{}/one", layout.area(Area::JobRecord));
    let other = format!("{}/other", layout.area(Area::JobRecord));
    subsystem.make_one_directory(&one).await.expect("one");
    subsystem.make_one_directory(&other).await.expect("other");
    assert!(subsystem.rename_without_replacing(&one, &other).await.is_err());
    // Both are still there — nothing was replaced.
    assert!(subsystem.stat(one.as_bytes()).await.is_ok());
    assert!(subsystem.stat(other.as_bytes()).await.is_ok());
}

#[tokio::test]
async fn the_abandoned_material_sweep_removes_the_two_directories_and_nothing_else() {
    let _deadline = TheTestsDeadline::started();
    let server = the_server_or_say_so!();
    let scratch = Scratch::new("sweep");
    let machine = a_machine(&scratch.a_key("host"), server).await;
    let connection = a_connection(&machine, &scratch.a_key("client")).await;
    let mut subsystem = a_subsystem(&connection).await;

    let layout = a_layout(&scratch);
    hand_over(&mut subsystem, &layout).await;

    let job = a_job();
    let paths = AJobsPaths::composed(&layout, &job, None).expect("paths");
    // A submission that made both directories and never published: the record
    // directory is made by hand first, so the rename is refused.
    subsystem
        .make_one_directory(&paths.job_record_directory)
        .await
        .expect("in the way");
    let ended = place_a_jobs_material(&mut subsystem, &paths, &a_manifest(&job, &paths)).await;
    assert!(matches!(ended, HowThePlacementEnded::ItStoppedAt(..)));
    assert!(subsystem
        .stat(paths.staging_directory.as_bytes())
        .await
        .is_ok());

    // Something else in the same area, which the sweep must not touch.
    let somebody_else = format!("{}/somebody-else", layout.area(Area::Job));
    subsystem
        .make_one_directory(&somebody_else)
        .await
        .expect("somebody else's");

    let submission = ASubmission {
        paths: paths.clone(),
        record: WhatTheDesktopRecordSays::ItStoppedBeforeThePublication,
    };
    let done = cleanup::sweep(&mut subsystem, &submission).await;
    assert_eq!(
        done,
        vec![
            (
                paths.staging_directory.clone(),
                WhatTheSweepDid::ItWasRemoved
            ),
            (
                paths.job_output_folder.clone(),
                WhatTheSweepDid::ItWasRemoved
            ),
        ]
    );
    assert!(subsystem
        .stat(paths.staging_directory.as_bytes())
        .await
        .is_err());
    assert!(subsystem
        .stat(paths.job_output_folder.as_bytes())
        .await
        .is_err());
    assert!(
        subsystem.stat(somebody_else.as_bytes()).await.is_ok(),
        "the sweep reached something it was not asked about"
    );

    // **The directory removal is refused where the directory still holds a
    // file, and says *not there* where it does not exist.** Those two answers
    // are what establish the request kind is the one the server means by
    // `rmdir` — the number is observed here rather than read.
    let holding = format!("{}/holding", layout.area(Area::JobRecord));
    subsystem.make_one_directory(&holding).await.expect("made");
    subsystem
        .write_a_whole_file_durably(&format!("{holding}/a-file"), b"one byte and more")
        .await
        .expect("written");
    assert!(subsystem.remove_one_empty_directory(&holding).await.is_err());
    assert_eq!(
        subsystem
            .remove_one_empty_directory(&format!("{}/never-made", layout.area(Area::JobRecord)))
            .await,
        Err(smoozy_job_files::acting::WhatTheMachineSaid::ThePathIsNotThere)
    );
    subsystem
        .remove_one_file(&format!("{holding}/a-file"))
        .await
        .expect("removed");
    assert!(subsystem.remove_one_empty_directory(&holding).await.is_ok());
}

// ------------------------------------------------------ this test's own deadline

/// **What every test in this file waits on**, named in the message a missed
/// deadline prints.
const WHAT_THE_TESTS_HERE_WAIT_ON: &str = "the loopback machine: OpenSSH's own sftp-server";

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

//! **The Telegram worker's piece 5 at the desk: a job the machine made for
//! itself, on a phone's *try it again* with Smoozy closed, is found on the
//! next connection and shown in Queue as work Smoozy is not tracking — offered
//! to be tracked, under the identity the machine issued.**
//!
//! `State, identity and recovery` Y7a: *"the desktop adopts such a job by the
//! identity the machine's record holds, exactly as it adopts any other"*. So
//! what is proved here is that the ordinary route already covers it: the
//! connection's step 7 (`Servers` Q1) reads the machine's journal forward past
//! the event the job's record was written under — which carries the machine's
//! own mark, a thing the desk reads by kind and never by its words — takes the
//! queue reading, and composes `Queue & History` A5's offer, and the Queue
//! screen draws the job as work the server is running that Smoozy is not
//! tracking, with *Track it*.
//!
//! **What the far end is.** A machine scripted to answer as the machine's own
//! program answers after the tap, field for field: the queue reading names the
//! failed job, ended, and the new job, waiting, first in the line; the read
//! forward carries the new job's *a job record was written* event, saying
//! `waiting: submitted again by this machine from <the failed job>`. **The
//! machine's half — that one tap makes that job, that record and that event —
//! is proved through the real program** in `App/Rented-server runtime —
//! program/tests/the_program_runs.rs`,
//! `try_it_again_makes_one_job_through_the_real_program_and_a_second_press_finds_it`.
//!
//! **And since owed piece 10, accepting the offer** (`Queue & History` A6 to
//! `Queue & History` A9): *Track it* enters the job in the local record, confirmed and under the
//! machine's identity, in one write, with what the machine's note and manifest
//! hold (A7). From then on it is in the queue as a tracked job; its row's
//! *Stop* reaches the machine; its render's start puts it in History, its
//! folder arrives by `Queue & History` N9's route, and its ending is learned
//! by the ordinary read forward. A second press adopts nothing, a deleted
//! entry is never adopted again (`Queue & History` A9), and a server that has gone or does not
//! answer adopts nothing (A6's all or nothing). Until that piece *Track it*
//! answered that it was not built.
//!
//! **Nothing of the Creator's is touched.** The local record is a throwaway
//! file in the build folder's own temporary area.
//!
//! **How a clause marker is written in this crate.** A marker with no document
//! beside it means a clause of `Preparation/Architecture/Desktop app and
//! delivery.md`; every other document's marker is written out with that
//! document at its site.

use std::path::PathBuf;

use smoozy_desktop::record::job_records::{JobRecords, TheKeptSetup};
use smoozy_desktop::record::LocalRecord;
use smoozy_desktop::servers::the_machines_work::{
    a_read_forward_pass, the_offer_is_accepted, the_queue_reading_from, the_step_seven, AsksTheSupervisor,
    WhatThisRunHolds, WhyNothingWasTracked,
};
use smoozy_desktop::surface::the_queue::{what_is_held, TheStopOrCancel};
use smoozy_job_files::manifest::{
    AnchorState, CarriedAndNotInterpretedHere, FrameRange, SceneField, SubmissionManifest, TheBlenderToRenderBy,
    TheCardMakeAtSubmission, TheRecordedReading, WhereTheJobOutputFolderIs, MANIFEST_FILE_NAME,
};
use smoozy_job_files::where_the_folder_sits::{WhereItIs, WhereTheJobOutputFolderSits, THE_RECORD_OF_WHERE_THE_FOLDER_SITS};
use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheNote, TheOutcomeAsWritten};
use smoozy_queue_history::ending::Ending;
use smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds;
use smoozy_render_settings::payload::Payload;
use smoozy_state::observed::MachineClock;
use smoozy_queue_history::record::{
    AnswerOrigin, Hue, LocalJobRecord, Standing, TheBlenderTheEntryKeeps, TheDeviceAsk,
    TheServerAsItStood, WhatTheNoteCarried,
};
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_server_protocol::requests as asked;
use smoozy_servers::identity::{Host, KeyReference, Port, Username};
use smoozy_servers::store::ServerRow;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, Reading};

/// **The local record, lent the way the running application lends it** — and
/// the diary lines another module wrote through it, kept with the module they
/// were written under (owed piece 11).
struct ARecordHere(LocalRecord, std::cell::RefCell<Vec<(&'static str, smoozy_reporting::lines::Line)>>);

impl smoozy_desktop::keys::LendsTheRecord for ARecordHere {
    fn lend(&self, act: &mut dyn FnMut(&rusqlite::Connection)) {
        act(self.0.connection())
    }
    fn write_a_diary_line(&self, _act: &smoozy_keys::diary::Act) {}
    fn write_a_modules_line(
        &self,
        module: &'static str,
        compose: &dyn Fn(Identity) -> smoozy_reporting::lines::Line,
    ) -> smoozy_desktop::diary_service::Wrote {
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        self.1.borrow_mut().push((module, compose(run)));
        smoozy_desktop::diary_service::Wrote::Yes
    }
}

/// **Where the made job stands on the machine**, moved by the test as the
/// machine's own program would move it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Waiting,
    Rendering,
    /// Stopped, its outcome recorded.
    Ended,
}

/// **The machine after the tap**, answering as its program does.
struct TheMachineAfterTheTap {
    failed: Identity,
    made: Identity,
    /// The render operation the failed job ended with.
    its_outcome: Identity,
    /// The made job's render operation, while it renders.
    the_render: Identity,
    /// The made job's recorded outcome, once it has ended.
    the_ending: Identity,
    stage: Stage,
    /// The made job's record directory, and its manifest's bytes there.
    record_directory: String,
    manifest: Vec<u8>,
    /// Where the render worker records the folder it chose at render start.
    folder_record: Vec<u8>,
    /// **Nothing answers at all** — a connection gone between the draw and
    /// the press.
    silent: bool,
    /// **The made job is gone from the machine between the reading and the
    /// press**, so its note reads as not held.
    the_job_is_gone: bool,
    asked: Vec<String>,
    /// The render operations a stop named.
    stops: Vec<String>,
}

impl TheMachineAfterTheTap {
    /// **What the new job's record was journalled under** — the kind every
    /// job's record is journalled under, saying the queue state, then the
    /// machine's mark and the job it was made from — **then, as the made job
    /// moves, its render's start and its recorded outcome.**
    fn the_journal(&self, from: u64) -> Fields {
        let mut events: Vec<(u64, &str, String, String)> = vec![(
            7,
            asked::THE_JOB_RECORD_WRITTEN_EVENT,
            self.made.as_text(),
            format!("{}: submitted again by this machine from {}", asked::IT_IS_WAITING, self.failed.as_text()),
        )];
        if self.stage != Stage::Waiting {
            events.push((8, asked::THE_RENDER_ATTEMPT_STARTED_EVENT, self.made.as_text(), "attempt 1".to_string()));
        }
        if self.stage == Stage::Ended {
            events.push((
                9,
                asked::THE_RECORDED_OUTCOME_WRITTEN_EVENT,
                self.made.as_text(),
                asked::IT_WAS_LEFT_STOPPED.to_string(),
            ));
        }
        let next = events.last().map(|(sequence, ..)| sequence + 1).unwrap_or(1);
        let carried: Vec<&(u64, &str, String, String)> = events.iter().filter(|(sequence, ..)| *sequence >= from).collect();
        let mut fields = Fields::none()
            .and(asked::HOW_MANY_EVENTS_FIELD, carried.len().to_string())
            .and(asked::THE_OLDEST_RETAINED_FIELD, "1")
            .and(asked::THE_NEXT_SEQUENCE_FIELD, next.to_string());
        for (at, (sequence, kind, about, says)) in carried.into_iter().enumerate() {
            let item = |name: &str| asked::the_field_of_item(asked::AN_EVENT, at, name);
            fields = fields
                .and(&item(asked::THE_SEQUENCE_FIELD), sequence.to_string())
                .and(
                    &item(asked::THE_EVENT_IDENTITY_FIELD),
                    Identity::issue(IdentityKind::Operation).expect("randomness").as_text(),
                )
                .and(&item(asked::THE_KIND_FIELD), *kind)
                .and(&item(asked::THE_ABOUT_FIELD), about.as_str())
                .and(&item(asked::THE_SAYS_FIELD), says.as_str());
        }
        fields
    }

    /// **`Queue & History` A1's reading** — the line first, then every other
    /// record by its identity, as the program orders it.
    fn the_queue(&self) -> Fields {
        let item = |at: usize, name: &str| asked::the_field_of_item(asked::THE_JOB_FIELD, at, name);
        let fields = Fields::none()
            .and(asked::HOW_MANY_JOBS_FIELD, "2")
            .and(asked::HOW_MANY_DID_NOT_READ_BACK_FIELD, "0")
            .and(&item(0, asked::THE_JOB_FIELD), self.made.as_text());
        let fields = match self.stage {
            Stage::Waiting => fields
                .and(&item(0, asked::THE_QUEUE_STATE_FIELD), asked::IT_IS_WAITING)
                .and(&item(0, asked::THE_PLACE_IN_THE_LINE_FIELD), "1")
                .and(asked::THE_JOB_THAT_STARTS_NEXT_FIELD, self.made.as_text()),
            Stage::Rendering => fields
                .and(&item(0, asked::THE_QUEUE_STATE_FIELD), asked::IT_IS_RENDERING)
                .and(&item(0, asked::THE_RENDER_OPERATION_FIELD), self.the_render.as_text()),
            Stage::Ended => fields
                .and(&item(0, asked::THE_QUEUE_STATE_FIELD), asked::IT_HAS_ENDED)
                .and(&item(0, asked::THE_OUTCOME_IT_ENDED_WITH_FIELD), self.the_ending.as_text()),
        };
        fields
            .and(&item(1, asked::THE_JOB_FIELD), self.failed.as_text())
            .and(&item(1, asked::THE_QUEUE_STATE_FIELD), asked::IT_HAS_ENDED)
            .and(&item(1, asked::THE_OUTCOME_IT_ENDED_WITH_FIELD), self.its_outcome.as_text())
    }

    /// **`Queue & History` N1's note for the made job**, as the program keeps
    /// it: the failed job's label, then its render's start and its ending as
    /// they are written. **Every other job's note reads as not held here.**
    fn the_note(&self, job: Option<&str>) -> Fields {
        if job != Some(self.made.as_text().as_str()) || self.the_job_is_gone {
            return Fields::none().and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_ABSENT);
        }
        let on_the_machine = |seconds: u64| {
            Reading::taken(
                (),
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds),
                Clock::AMachine(MachineClock("boot-1".to_string())),
            )
        };
        let mut note = TheNote::created_with_the_job_record("Hall");
        if self.stage != Stage::Waiting {
            note.started = Some(on_the_machine(1_760_000_100));
            note.attempts = 1;
        }
        if self.stage == Stage::Ended {
            note.ended = Some(on_the_machine(1_760_000_160));
            note.ending = Some(TheEndingOnTheNote {
                outcome: TheOutcomeAsWritten::LeftStopped,
                exit_status: None,
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                the_device_line_as_observed: None,
            });
        }
        note.as_the_answers_fields()
    }
}

impl AsksTheSupervisor for TheMachineAfterTheTap {
    fn ask(&mut self, request: &Request) -> WhatCameBack {
        if self.silent {
            return WhatCameBack::ItWasNeverAsked;
        }
        self.asked.push(request.name.clone());
        let answered = Answered::to(request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "the machine's program");
        let fields = match request.name.as_str() {
            asked::THE_READ_FORWARD_REQUEST => {
                let from = request.fields.text(asked::THE_CURSOR_FIELD).and_then(|text| text.parse().ok()).unwrap_or(0);
                self.the_journal(from)
            }
            asked::THE_CURSOR_MOVES_FORWARD_REQUEST => Fields::none(),
            asked::THE_QUEUE_READING_REQUEST => self.the_queue(),
            asked::THE_PER_JOB_NOTE_READING_REQUEST => self.the_note(request.fields.text(asked::THE_JOB_FIELD)),
            asked::THE_STOP_REQUEST => {
                self.stops.push(request.fields.text(asked::THE_OPERATION_TO_STOP_FIELD).unwrap_or_default().to_string());
                Fields::none()
                    .and(asked::WHAT_BECAME_OF_IT_FIELD, asked::IT_HAS_A_RECORDED_OUTCOME)
                    .and(asked::THE_STOPS_OUTCOME_FIELD, asked::IT_WAS_LEFT_STOPPED)
            }
            _ => return WhatCameBack::ItAnswered(answered.saying_it_does_not_answer_that_request(&request.name)),
        };
        WhatCameBack::ItAnswered(answered.carrying(fields))
    }

    /// **The made job's record directory, as the reading session reads it**:
    /// its manifest from the tap, and — once its render has started — the
    /// record of where its folder was chosen. Nothing else stands.
    fn a_file_of_the_job_record_directory(&mut self, path: &str) -> Option<Vec<u8>> {
        if self.silent {
            return None;
        }
        self.asked.push(format!("read {path}"));
        if path == format!("{}/{MANIFEST_FILE_NAME}", self.record_directory) {
            return Some(self.manifest.clone());
        }
        if self.stage != Stage::Waiting
            && path == format!("{}/{THE_RECORD_OF_WHERE_THE_FOLDER_SITS}", self.record_directory)
        {
            return Some(self.folder_record.clone());
        }
        None
    }
}

/// **The made job's manifest, as the tap published it**: the failed job's,
/// copied under the new identity (`Job files and outputs` M1,
/// `SubmissionManifest::copied_under`) — no group, the folder chosen at render
/// start under the new name.
fn the_manifest_the_tap_published(made: &Identity, kept: &Payload) -> SubmissionManifest {
    let failed = Identity::issue(IdentityKind::Job).expect("randomness");
    SubmissionManifest::composed(
        &failed,
        None,
        TheBlenderToRenderBy { version: "5.2.2".to_string(), build_path: "/root/smoozy/blender/5.2.2/blender".to_string() },
        SceneField { name: String::new(), anchor: AnchorState::ToAnchor { provenance: "none named".to_string() } },
        FrameRange::TheUsersParsedIntegers(vec![1, 2, 3]),
        kept.as_carried(),
        CarriedAndNotInterpretedHere(serde_json::Value::Null),
        CarriedAndNotInterpretedHere(serde_json::Value::Null),
        "/root/smoozy/assets/hall.blend".to_string(),
        smoozy_server_protocol::transfers::SourceReading {
            size: 10,
            inode: "1".to_string(),
            modification_time: 1,
            change_time: 1,
        },
        TheRecordedReading::ThereIsNone,
        TheCardMakeAtSubmission::NoReadingStood,
        WhereTheJobOutputFolderIs::ChosenAtRenderStart {
            the_folders_name: "unused".to_string(),
            the_servers_fallback_folder_at_submission: None,
        },
    )
    .copied_under(made, Some("Hall"))
    .expect("the copy's folder name fits")
}

/// **The settings the failed job was submitted with**, kept in its manifest
/// and so in the copy: a frame range, a typed percentage and the GPU.
fn the_settings_payload() -> Payload {
    use smoozy_render_settings::file_half::BlenderIdentity;
    use smoozy_render_settings::payload::{compose, Composition};
    use smoozy_render_settings::store::SceneEntries;
    let mut payload = compose(Composition {
        version_pick: &BlenderIdentity {
            path: "/root/smoozy/blender/5.2.2/blender".to_string(),
            version: smoozy_server_setup::blender::Version { major: 5, minor: 2, patch: 2 },
        },
        scene: None,
        answers: &SceneEntries {
            frame_range: Some(smoozy_render_settings::frames::parse_the_frame_field("1-3").expect("a range")),
            ..Default::default()
        },
        standing: None,
        stale: false,
    });
    payload.resolution_percentage = Some(50);
    payload
}

/// **The machine after the tap, set up for the desk it answers**: the made
/// job's record directory composed from the layout the row records.
fn the_machine_for(desk: &ARecordHere, server: &Identity, failed: &Identity, made: &Identity) -> TheMachineAfterTheTap {
    let row = smoozy_desktop::record::server_rows::one(desk.0.connection(), &server.as_text())
        .expect("the row reads")
        .expect("the row stands");
    let layout = smoozy_desktop::servers::installation::the_layout_recorded_for(&row).expect("a layout is recorded");
    let paths = smoozy_job_files::publication::AJobsPaths::composed(&layout, made, Some("Hall")).expect("the paths");
    let manifest = the_manifest_the_tap_published(made, &the_settings_payload());
    let folder = WhereTheJobOutputFolderSits {
        path: format!("{}/{}", layout.area(smoozy_job_files::layout::Area::Job), "made-hall"),
        where_it_is: WhereItIs::TheFallback { why: None, the_servers_fallback_folder_was_passed_over: false },
    };
    TheMachineAfterTheTap {
        failed: failed.clone(),
        made: made.clone(),
        its_outcome: Identity::issue(IdentityKind::Operation).expect("randomness"),
        the_render: Identity::issue(IdentityKind::Operation).expect("randomness"),
        the_ending: Identity::issue(IdentityKind::Operation).expect("randomness"),
        stage: Stage::Waiting,
        record_directory: paths.job_record_directory,
        manifest: manifest.as_written(),
        folder_record: folder.as_written(),
        silent: false,
        the_job_is_gone: false,
        asked: Vec::new(),
        stops: Vec::new(),
    }
}

/// How many local job records stand.
fn how_many_records(desk: &ARecordHere) -> i64 {
    desk.0
        .connection()
        .query_row("SELECT count(*) FROM job_record", [], |row| row.get(0))
        .expect("the record counts")
}

/// **A fresh reading of the machine, landed on the connection as P10d's pass
/// lands one** — the held offer untouched where `offer` is nothing.
fn a_fresh_reading(
    machine: &mut TheMachineAfterTheTap,
    holds: &WhatThisRunHolds,
    server: &Identity,
    connection: u64,
    offer: Option<Vec<Identity>>,
) {
    let state = the_queue_reading_from(machine).expect("the machine reads");
    holds.a_reading_was_taken(
        &server.as_text(),
        connection,
        Reading::taken(state, std::time::SystemTime::now(), Clock::ThisComputer),
        offer,
    );
}

/// **A fresh local record holding the server's row and the failed job** —
/// the job this desk sent, confirmed and in History, its ending learned.
fn the_desk(failed: &Identity) -> (ARecordHere, Identity) {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "smoozy-try-it-again-{}",
        Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
    ));
    std::fs::create_dir_all(&folder).expect("the desk's own folder");
    let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record opens");
    let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
    let row = ServerRow::saved(
        server.as_text(),
        Host::what_the_user_typed("127.0.0.1").expect("a host"),
        Port::what_the_user_typed(22).expect("a port"),
        Username::what_the_user_typed("root").expect("a username"),
        KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
    );
    let mut row = row;
    row.label = Some("Server A".to_string());
    // **The root `Job files and outputs` L1 chose at hand-over**, which every
    // job's record directory is composed under.
    row.recorded_machine_root = Some(smoozy_servers::store::RecordedMachineRoot {
        path: "/root/smoozy".into(),
        the_package_areas_own_location: None,
        read_under: "inst-1".into(),
    });
    smoozy_desktop::record::server_rows::add(record.connection(), &row).expect("the row is added");
    let sent = LocalJobRecord {
        job: failed.clone(),
        group: None,
        server: TheServerAsItStood { local_server: server.clone(), label: Some("Server A".to_string()), hue: Some(Hue(210)) },
        submitted_at: Some(Reading::taken(
            (),
            std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_760_000_000),
            Clock::ThisComputer,
        )),
        label: Some("Hall".to_string()),
        blend_path_on_the_machine: None,
        scene_name: None,
        submitted_with_no_scene_named: true,
        resolution_percentage_typed: None,
        picture_size_from_the_read: None,
        main_output_from_the_read: None,
        blender: TheBlenderTheEntryKeeps {
            pick: smoozy_render_settings::payload::VersionPick { build_path: "/b".to_string(), version: "5.2.2".to_string() },
            origin: AnswerOrigin::ResolvedForThem,
        },
        device_asked_for: TheDeviceAsk::TheFilesOwnUnread,
        the_files_values_beside_the_overrides: None,
        dependency_check: None,
        carries_on_from: None,
        disturbances: vec![],
        attempts_where_the_file_changed: vec![],
        the_frames: smoozy_queue_history::record::TheFramesTheReadFound::default(),
        standing: Standing::Confirmed,
        the_render_start_has_been_learned: true,
        what_the_note_carried: WhatTheNoteCarried::default(),
        the_job_output_folder: Default::default(),
        the_driver_was_below_the_bar: None,
    };
    JobRecords::over(record.connection()).write_the_provisional_record(&sent).expect("the job this desk sent");
    (ARecordHere(record, Default::default()), server)
}

/// **`Telegram` Y1 and `State, identity and recovery` Y7a at the desk: the job
/// the machine made is offered on the next connection, under the identity the
/// machine issued, and the Queue shows it as untracked work, waiting.**
///
/// The offer is `Queue & History` A5's — every job on the machine the local
/// record does not name — so it is the new job and not the failed one, which
/// this desk sent. **The event carrying the machine's mark is read past
/// without a word**: the desk reads the kind and the job it is about, the
/// cursor moves past it, and nothing in the local record changes until the
/// user tracks the job. **The Queue's header counts only the job this desk
/// tracks** — none in the line, the failed one having ended — and the banner
/// counts one waiting, with *Track it*.
#[test]
fn a_job_the_machine_made_on_a_phones_try_it_again_is_offered_and_shown_in_queue() {
    let failed = Identity::issue(IdentityKind::Job).expect("randomness");
    let made = Identity::issue(IdentityKind::Job).expect("randomness");
    let (desk, server) = the_desk(&failed);
    let mut machine = the_machine_for(&desk, &server, &failed, &made);
    let holds = WhatThisRunHolds::default();
    let connection = holds.a_connection_begins(&server.as_text());

    let did = the_step_seven(&server, connection, &desk, &mut machine, &holds, &|| true);
    assert!(did.the_queue_was_read, "the queue reading was taken: {:?}", machine.asked);
    assert_eq!(did.events_applied, 1, "the event the new job was journalled under was read: {:?}", machine.asked);
    assert_eq!(did.offered, 1, "`Queue & History` A5: one job offered");
    let period = holds
        .every_server()
        .into_iter()
        .find(|(held, _)| *held == server.as_text())
        .map(|(_, period)| period)
        .expect("the period holds the reading");
    assert_eq!(period.the_offer, vec![made.clone()], "the job the machine made, by the identity it issued");

    let records = JobRecords::over(desk.0.connection());
    assert!(records.read(&made).expect("it reads").is_none(), "nothing enters the record until the user tracks it");
    assert!(records.read(&failed).expect("it reads").is_some(), "and the job this desk sent stands");

    let screen = what_is_held(&holds, Some(&desk.0), None);
    let untracked = screen.untracked.expect("`Queue & History` A5: the offer is drawn in Queue");
    assert_eq!((untracked.rendering, untracked.waiting), (0, 1), "one job waiting that Smoozy is not tracking");
    assert_eq!(untracked.server_row, server.as_text());
    assert_eq!(untracked.cause, None, "no removal or address change is claimed for it");
}

/// **Owed piece 10: *Track it* on the job the machine made — it is tracked,
/// its row's *Stop* reaches the machine, it reaches History, its folder
/// arrives, and its ending is learned; a second press adopts nothing.**
///
/// - **`Queue & History` A6**: one write, the record **confirmed**, under the
///   identity the machine issued (Y4, `State, identity and recovery` Y7a).
/// - **A7**: the entry holds what the machine's records hold — the label off
///   the note, the `.blend`, the Blender pair and the typed percentage off the
///   manifest, the settings payload kept beside the record — and nothing this
///   desk never saw (no submitted moment).
/// - **A5 at the press**: a second press, and a press on an offer drawn
///   before the first went through, find nothing left and write nothing.
/// - **A8**: from then on it is an ordinary job — in the queue as a tracked
///   row with *Cancel* while it waits and *Stop* while it renders; in History
///   from its render's start (C9); its folder learned by `Queue & History` N9's route; its
///   ending learned by the read forward, and it leaves the queue (C11).
#[test]
fn track_it_adopts_the_job_the_machine_made_and_it_is_tracked_to_its_end() {
    let failed = Identity::issue(IdentityKind::Job).expect("randomness");
    let made = Identity::issue(IdentityKind::Job).expect("randomness");
    let (desk, server) = the_desk(&failed);
    let mut machine = the_machine_for(&desk, &server, &failed, &made);
    let holds = WhatThisRunHolds::default();
    let connection = holds.a_connection_begins(&server.as_text());
    let did = the_step_seven(&server, connection, &desk, &mut machine, &holds, &|| true);
    assert_eq!(did.offered, 1);

    // **Track it.**
    let adopted = the_offer_is_accepted(&server, &desk, &mut machine, &holds).expect("the offer is accepted");
    assert_eq!(adopted, vec![made.clone()], "`Queue & History` A6: the offer, under the machine's identity");
    let records = JobRecords::over(desk.0.connection());
    let held = records.read(&made).expect("it reads").expect("the job is in the local record");
    assert_eq!(held.standing, smoozy_queue_history::record::Standing::Confirmed, "A6: written confirmed");
    assert_eq!(held.server.local_server, server);
    assert_eq!(held.server.label.as_deref(), Some("Server A"), "`Queue & History` N12's copy of the server as its row stands");
    assert_eq!(held.label.as_deref(), Some("Hall"), "A7: the label off the machine's note");
    assert_eq!(held.blend_path_on_the_machine.as_deref(), Some("/root/smoozy/assets/hall.blend"), "A7: the file off the manifest");
    assert_eq!(held.blender.pick.version, "5.2.2", "A7: the Blender off the manifest");
    assert_eq!(held.resolution_percentage_typed, Some(50), "A7: the typed percentage off the settings payload");
    assert!(held.submitted_with_no_scene_named, "the payload named no scene");
    assert!(held.submitted_at.is_none(), "`Queue & History` N16: this desk never saw the submission, so no moment is made up");
    assert!(held.group.is_none(), "the tap's copy carries no group");
    assert_eq!(held.the_job_output_folder, TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart);
    assert!(!held.the_render_start_has_been_learned, "it waits, so it is not in History yet (C9)");
    assert!(
        matches!(records.the_kept_setup_of(&made), Ok(Some(TheKeptSetup::Kept(kept))) if kept == the_settings_payload()),
        "the settings payload is kept beside the record, as a submission's is"
    );

    // **The Queue: a tracked row, no banner.**
    let offer_held = holds
        .every_server()
        .into_iter()
        .find(|(held, _)| *held == server.as_text())
        .map(|(_, period)| period.the_offer)
        .expect("the period stands");
    assert!(offer_held.is_empty(), "the accepted jobs leave the held offer");
    let screen = what_is_held(&holds, Some(&desk.0), Some(&made.as_text()));
    assert!(screen.untracked.is_none(), "the offer is gone once accepted");
    assert_eq!(screen.jobs, 1, "the job is in its server's line");
    let entry = screen.entry.expect("the tracked job's entry");
    assert_eq!(entry.job, made.as_text());
    assert_eq!(entry.name, "Hall");
    assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Cancel), "a waiting row offers Cancel");

    // **`Queue & History` §17: one diary line for the act** — *untracked work
    // taken over*, the server's label and the count, under the module's own
    // name, and no job's identity or label (owed piece 11).
    {
        let lines = desk.1.borrow();
        assert_eq!(lines.len(), 1, "one line for the act: {lines:#?}");
        let (module, line) = &lines[0];
        assert_eq!(*module, smoozy_desktop::servers::the_machines_work::THE_QUEUES_MODULE);
        assert_eq!(line.kind, smoozy_reporting::lines::LineKind::Act);
        assert_eq!(line.text, "untracked work taken over\tServer A\t1");
        assert!(line.concerning.contains(&server), "it carries the server row");
        assert!(!line.concerning.contains(&made) && !line.text.contains("Hall"), "it names no job");
    }

    // **A second press adopts nothing, writes nothing and reads nothing.**
    let records_before = how_many_records(&desk);
    let asked_before = machine.asked.len();
    assert_eq!(the_offer_is_accepted(&server, &desk, &mut machine, &holds), Ok(Vec::new()));
    // **And a press on an offer drawn before the first press went through**:
    // the held offer names the job again, and the record already does.
    a_fresh_reading(&mut machine, &holds, &server, connection, Some(vec![made.clone()]));
    assert_eq!(the_offer_is_accepted(&server, &desk, &mut machine, &holds), Ok(Vec::new()));
    assert_eq!(how_many_records(&desk), records_before, "nothing adopted twice");
    assert_eq!(desk.1.borrow().len(), 1, "a press that adopts nothing writes no line");
    assert_eq!(machine.asked.len(), asked_before + 1, "only the fresh reading went to the machine");

    // **It starts rendering**: the read forward learns the start off the note
    // and it is in History at once (C9); its folder arrives by `Queue & History` N9's route.
    machine.stage = Stage::Rendering;
    a_read_forward_pass(&server, &desk, &mut machine, &holds);
    a_fresh_reading(&mut machine, &holds, &server, connection, None);
    let held = records.read(&made).expect("it reads").expect("it stands");
    assert!(held.the_render_start_has_been_learned, "the render's start is learned off the note");
    assert!(
        records.in_history().expect("history reads").entries.iter().any(|one| one.record.job == made),
        "`Queue & History` C9: in History from its render's start"
    );
    assert!(
        matches!(&held.the_job_output_folder, TheJobOutputFolderTheRecordHolds::It(sits) if sits.path.ends_with("/made-hall")),
        "`Queue & History` N9: the folder chosen at render start arrives, so Download has it: {:?}",
        held.the_job_output_folder
    );

    // **The row's Stop reaches the machine**, naming the render the reading
    // the row was drawn from shows (`Server protocol and transfers` C21).
    let screen = what_is_held(&holds, Some(&desk.0), Some(&made.as_text()));
    let entry = screen.entry.expect("the rendering job's entry");
    assert_eq!(entry.job, made.as_text());
    assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Stop), "a rendering row offers Stop");
    let the_render = holds
        .every_server()
        .into_iter()
        .find(|(held, _)| *held == server.as_text())
        .and_then(|(_, period)| {
            period.reading?.value().jobs.iter().find(|job| job.job == made).and_then(|job| match &job.state {
                smoozy_queue_history::line::QueueState::Rendering { operation } => Some(operation.clone()),
                _ => None,
            })
        })
        .expect("the reading shows it rendering");
    let stood = smoozy_desktop::servers::the_queues_own_acts::the_stop_is_sent(
        &mut machine,
        &the_render,
        smoozy_desktop::servers::stop_and_clear::TheWait {
            deadline: std::time::Duration::from_secs(5),
            pause: std::time::Duration::from_millis(10),
        },
    );
    assert_eq!(stood, None, "the stop is done: left stopped");
    assert_eq!(machine.stops, vec![machine.the_render.as_text()], "the stop named the made job's render");

    // **It ends**: the read forward learns the ending, History says it, and
    // it leaves the queue (C11).
    machine.stage = Stage::Ended;
    a_read_forward_pass(&server, &desk, &mut machine, &holds);
    a_fresh_reading(&mut machine, &holds, &server, connection, None);
    let held = records.read(&made).expect("it reads").expect("it stands");
    assert_eq!(held.the_ending_learned(), Some(Ending::Stopped), "the ending is learned");
    let history = smoozy_desktop::surface::the_history::what_is_kept(Some(&desk.0), Some(&made.as_text()));
    assert!(history.rows.iter().any(|row| row.job == made.as_text() && row.ending.is_some()), "History holds it, ended");
    let entry = history.entry.expect("its History entry");
    assert_eq!(
        entry.entry.the_folder,
        smoozy_desktop::surface::the_queue::TheFolderOnTheEntry::ItHasOne,
        "the entry has its folder, so its Download is drawn"
    );
    let screen = what_is_held(&holds, Some(&desk.0), None);
    assert!(screen.lines.iter().all(|line| line.rows.iter().all(|row| row.job != made.as_text())), "it has left the queue");
    assert!(screen.untracked.is_none());
}

/// **`Queue & History` A9 at the press: nothing about adoption restores a
/// deletion.** The adopted job's entry is deleted — its record and its
/// tombstone (X1) — and an offer drawn before the deletion names it again:
/// *Track it* adopts nothing.
#[test]
fn a_deleted_entry_is_never_adopted_again() {
    let failed = Identity::issue(IdentityKind::Job).expect("randomness");
    let made = Identity::issue(IdentityKind::Job).expect("randomness");
    let (desk, server) = the_desk(&failed);
    let mut machine = the_machine_for(&desk, &server, &failed, &made);
    let holds = WhatThisRunHolds::default();
    let connection = holds.a_connection_begins(&server.as_text());
    the_step_seven(&server, connection, &desk, &mut machine, &holds, &|| true);
    the_offer_is_accepted(&server, &desk, &mut machine, &holds).expect("adopted");
    let records = JobRecords::over(desk.0.connection());
    let held = records.read(&made).expect("it reads").expect("it stands");
    smoozy_desktop::record::deletions::TheDeletions::over(desk.0.connection())
        .perform_all(&[smoozy_queue_history::deletion::ADeletion::of(&held, std::time::SystemTime::now())])
        .expect("the entry is deleted");
    a_fresh_reading(&mut machine, &holds, &server, connection, Some(vec![made.clone()]));
    assert_eq!(the_offer_is_accepted(&server, &desk, &mut machine, &holds), Ok(Vec::new()));
    assert!(records.read(&made).expect("it reads").is_none(), "the deleted entry stays deleted");
}

/// **A6's all or nothing where the server has gone, or does not answer.**
///
/// - **The machine does not answer** the reads each entry is composed from
///   (A7): nothing is adopted, and the press says Smoozy could not reach the
///   server — a record written with nothing in it could never be filled in.
/// - **The server's period has ended**, its reading dropped (`Queue &
///   History` C6): there is no offer to accept, and the press says the same.
///
/// The offer still stands afterwards, so a later press on a live connection
/// adopts the job — declining or failing is never final (A5).
#[test]
fn a_server_that_has_gone_or_does_not_answer_adopts_nothing() {
    let failed = Identity::issue(IdentityKind::Job).expect("randomness");
    let made = Identity::issue(IdentityKind::Job).expect("randomness");
    let (desk, server) = the_desk(&failed);
    let mut machine = the_machine_for(&desk, &server, &failed, &made);
    let holds = WhatThisRunHolds::default();
    let connection = holds.a_connection_begins(&server.as_text());
    the_step_seven(&server, connection, &desk, &mut machine, &holds, &|| true);
    let records_before = how_many_records(&desk);

    machine.silent = true;
    assert_eq!(
        the_offer_is_accepted(&server, &desk, &mut machine, &holds),
        Err(WhyNothingWasTracked::TheServerCouldNotBeReached)
    );
    assert_eq!(how_many_records(&desk), records_before, "nothing adopted");
    assert!(desk.1.borrow().is_empty(), "nothing taken over, so no diary line");
    let screen = what_is_held(&holds, Some(&desk.0), None);
    assert!(screen.untracked.is_some(), "the offer still stands");

    machine.silent = false;
    assert_eq!(the_offer_is_accepted(&server, &desk, &mut machine, &holds), Ok(vec![made.clone()]));

    // **A second server's period ends before the press**: nothing to accept.
    let other_failed = Identity::issue(IdentityKind::Job).expect("randomness");
    let other_made = Identity::issue(IdentityKind::Job).expect("randomness");
    let (other_desk, other_server) = the_desk(&other_failed);
    let mut other = the_machine_for(&other_desk, &other_server, &other_failed, &other_made);
    let other_holds = WhatThisRunHolds::default();
    let other_connection = other_holds.a_connection_begins(&other_server.as_text());
    the_step_seven(&other_server, other_connection, &other_desk, &mut other, &other_holds, &|| true);
    other_holds.the_period_ended(&other_server.as_text());
    assert_eq!(
        the_offer_is_accepted(&other_server, &other_desk, &mut other, &other_holds),
        Err(WhyNothingWasTracked::TheServerCouldNotBeReached)
    );
    assert!(JobRecords::over(other_desk.0.connection()).read(&other_made).expect("it reads").is_none());
}

/// **`Queue & History` A7: where a record will not read, the entry states the
/// absence** — no label, no file, no Blender, the file's own device unread,
/// and the job area's folder, where `Queue & History` N9's route then reads. It is still
/// written confirmed, under the machine's identity, on this server (A6).
#[test]
fn an_adopted_job_whose_records_will_not_read_carries_their_absence() {
    let job = Identity::issue(IdentityKind::Job).expect("randomness");
    let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
    let stood = TheServerAsItStood { local_server: server.clone(), label: Some("Server A".to_string()), hue: None };
    let adopted = smoozy_queue_history::acting::adoption::AnAdoptedJob { job: job.clone(), server: server.clone(), the_note: None };
    let (record, kept) = smoozy_desktop::servers::the_machines_work::an_adopted_record(&adopted, &stood, None);
    assert_eq!(record.job, job);
    assert_eq!(record.server, stood);
    assert_eq!(record.standing, Standing::Confirmed);
    assert_eq!(record.label, None);
    assert_eq!(record.blend_path_on_the_machine, None);
    assert_eq!(record.scene_name, None);
    assert!(!record.submitted_with_no_scene_named, "no manifest read, so nothing says no scene was named");
    assert_eq!(record.blender.pick.version, "");
    assert_eq!(record.device_asked_for, TheDeviceAsk::TheFilesOwnUnread);
    assert_eq!(record.the_job_output_folder, TheJobOutputFolderTheRecordHolds::AsEveryJobBeforeItHadIt);
    assert!(!record.the_render_start_has_been_learned);
    assert!(kept.is_none());
}

/// **`Queue & History` A8 and C9: a job already rendering when it is adopted
/// is in History at once** — its note's start is the learning, carried by the
/// record's own fill-in at the write, with no pass in between — **and its
/// folder is read by `Queue & History` N9's route at the press.**
#[test]
fn a_job_adopted_while_it_renders_is_in_history_at_once() {
    let failed = Identity::issue(IdentityKind::Job).expect("randomness");
    let made = Identity::issue(IdentityKind::Job).expect("randomness");
    let (desk, server) = the_desk(&failed);
    let mut machine = the_machine_for(&desk, &server, &failed, &made);
    machine.stage = Stage::Rendering;
    let holds = WhatThisRunHolds::default();
    let connection = holds.a_connection_begins(&server.as_text());
    the_step_seven(&server, connection, &desk, &mut machine, &holds, &|| true);
    assert_eq!(the_offer_is_accepted(&server, &desk, &mut machine, &holds), Ok(vec![made.clone()]));
    let records = JobRecords::over(desk.0.connection());
    let held = records.read(&made).expect("it reads").expect("adopted");
    assert!(held.the_render_start_has_been_learned, "the note's start came with the write");
    assert_eq!(held.what_the_note_carried.attempts, 1);
    assert!(
        matches!(&held.the_job_output_folder, TheJobOutputFolderTheRecordHolds::It(_)),
        "`Queue & History` N9's route ran at the press, so its folder is known at once: {:?}",
        held.the_job_output_folder
    );
    assert!(
        records.in_history().expect("history reads").entries.iter().any(|one| one.record.job == made),
        "in History at once"
    );
    let screen = what_is_held(&holds, Some(&desk.0), Some(&made.as_text()));
    assert_eq!(screen.entry.expect("its entry").stop_or_cancel, Some(TheStopOrCancel::Stop));
}

/// **`Queue & History` A5's set, read at the press: a job the machine no
/// longer holds is not work on the machine**, and is not adopted — a record
/// confirmed for it would stand for a job nothing will ever fill in.
#[test]
fn a_job_gone_from_the_machine_before_the_press_is_not_adopted() {
    let failed = Identity::issue(IdentityKind::Job).expect("randomness");
    let made = Identity::issue(IdentityKind::Job).expect("randomness");
    let (desk, server) = the_desk(&failed);
    let mut machine = the_machine_for(&desk, &server, &failed, &made);
    let holds = WhatThisRunHolds::default();
    let connection = holds.a_connection_begins(&server.as_text());
    the_step_seven(&server, connection, &desk, &mut machine, &holds, &|| true);
    machine.the_job_is_gone = true;
    assert_eq!(the_offer_is_accepted(&server, &desk, &mut machine, &holds), Ok(Vec::new()));
    assert!(JobRecords::over(desk.0.connection()).read(&made).expect("it reads").is_none());
}

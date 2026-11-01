//! **`Preparation/Scenarios/The user stops a running job.md`, played at the
//! desk** — step 12's piece SC8.
//!
//! The scenario's situation and its six steps, unchanged, through the
//! application's own acts, against the desk machine. At each step the test
//! prints what the user did, what Smoozy hands the page to show, and which
//! clause says so, and asserts all three. **A step the desk cannot play is said
//! in the output and never skipped quietly.**
//!
//! **What is real here.** OpenSSH's own server behind the harness's line; the
//! application's own transport and the server's own point; `Servers` Q1 step 7
//! on the connection; the desktop's own submit act; the read forward and the
//! queue reading a pass takes; the queue's own *Stop* — `Server protocol and
//! transfers` C21's command, waited on by its own identity with `Rendering`
//! K9's deadline; History's own delete act, over the application's own start
//! value; the Queue and History screens' own readings; and the machine's own
//! program, its real supervisor, its real render worker and its real stop.
//! **What is stood in for**: Blender, by the harness's script, held before
//! frame 6 so that frame 6 is the frame in hand when the user stops.
//!
//! **Where the application's own wiring needs the window, it is mirrored and
//! said.** The connection here is the harness's point lending one connection,
//! not a period driven by `Servers` (that is SC1's subject); step 7 is run on
//! it as the hand-over runs it; a pass is the read forward and the queue
//! reading, as `ThePassesOverThisConnection` takes them; and the *Stop* is
//! `the_act_is_begun`'s own order — the job marked in hand, the stop sent, its
//! result kept, one read forward — with what needs the window left out (the
//! screens told again).
//!
//! **What the desk cannot do.**
//!
//! - **Twenty minutes into frame 6.** The stand-in waits before frame 6 for as
//!   long as the test holds it, which is what a long frame is from outside.
//! - **Step 4's folder panel.** *Download all to…* is played with the folder
//!   the user picks standing in for the panel; the rest of the act is the
//!   desktop's own (`a_whole_job_is_downloaded_to` in the harness says how).
//!   Until 2026-10-24 the download was not played at all, for want of
//!   `smoozy-download` in this crate's manifest; the runner's ruling added it.
//! - **A real render being stopped**, which the rented pass plays on every
//!   provider (the map's RP1 to RP4): what a real Blender leaves of frame 6 is
//!   not the stand-in's to say.
//!
//! **Nothing of the Creator's is touched**: see the head of
//! `src/the_desk_machine.rs`.

#![cfg(unix)]

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use smoozy_desktop::downloading::TheRunsTransfers;
use smoozy_desktop::record::deletions::TheDeletions;
use smoozy_desktop::record::job_records::JobRecords;
use smoozy_desktop::servers::stop_and_clear::TheWait;
use smoozy_desktop::servers::the_machines_work::{
    a_read_forward_pass, the_queue_reading_from, the_step_seven, AsksTheSupervisor, WhatThisRunHolds,
};
use smoozy_desktop::servers::the_queues_own_acts::{it_stands, the_stop_is_sent, where_the_act_stands, TheQueuesOwnAct, WhereTheActStands};
use smoozy_desktop::submitting::{the_act_is_performed_naming_its_jobs, WhatBecameOfOneJob, WhatTheActCameTo};
use smoozy_desktop::surface::the_history::{the_act_is_performed, the_history_as_asked, what_is_kept, AnEnding, TheHistoryAsk};
use smoozy_desktop::surface::the_queue::{what_is_held, TheActOnAJob, TheFolderOnTheEntry, TheStopOrCancel};
use smoozy_job_files::composing::TheReadingAndWhatFollows;
use smoozy_job_files::inventory::WhatTheInventoryIs;
use smoozy_job_files::publication::AJobsPaths;
use smoozy_queue_history::ending::Ending as AJobsEnding;
use smoozy_queue_history::line::QueueState;
use smoozy_scenarios::the_desk_machine::{a_request, a_whole_job_is_downloaded_to, APointLending, TheDesk, TheDeskMachine};
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::Fields;
use smoozy_server_protocol::requests as asked;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, Reading};

// ════════════════════════════════════════════════════════════════════════════
// One connection, and the passes on it
// ════════════════════════════════════════════════════════════════════════════

/// **The server's connection, as the run holds it**: the point lending it,
/// the run's own holding of the machine's work, and the connection's number.
struct TheConnection<'a> {
    machine: &'a TheDeskMachine,
    desk: &'a TheDesk,
    point: APointLending,
    holds: Arc<WhatThisRunHolds>,
    server: Identity,
    connection: u64,
}

impl TheConnection<'_> {
    /// **One pass of `Desktop app and delivery` P10d**: the read forward, then the queue reading, landed on
    /// this connection — off the test's own threads, because every ask blocks.
    async fn a_pass(&self) {
        let (mut machine, holds, server, connection, record) = (
            self.machine.the_supervisor_over(&self.point),
            Arc::clone(&self.holds),
            self.server.clone(),
            self.connection,
            Arc::clone(self.desk.record()),
        );
        tokio::task::spawn_blocking(move || {
            let _ = a_read_forward_pass(&server, record.as_ref(), &mut machine, &holds);
            if let Ok(state) = the_queue_reading_from(&mut machine) {
                holds.a_reading_was_taken(
                    &server.as_text(),
                    connection,
                    Reading::taken(state, SystemTime::now(), Clock::ThisComputer),
                    None,
                );
            }
        })
        .await
        .expect("the pass ran");
    }

    /// The job as the period-held reading last read it (`Queue & History` C6).
    fn the_job_as_last_read(&self, job: &Identity) -> Option<smoozy_queue_history::readings::AJobAsTheMachineReadsIt> {
        self.holds
            .every_server()
            .into_iter()
            .find(|(held, _)| *held == self.server.as_text())
            .and_then(|(_, period)| period.reading)
            .and_then(|reading| reading.value().jobs.iter().find(|one| &one.job == job).cloned())
    }

    /// **Passes, one after another, until `seen`**, each a fixed second apart,
    /// failing the test past `within`.
    async fn passes_until(&self, said: &str, within: Duration, seen: impl Fn(&Self) -> bool) {
        let began = Instant::now();
        loop {
            self.a_pass().await;
            if seen(self) {
                return;
            }
            assert!(began.elapsed() < within, "{said}: not seen within {within:?}");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

/// **The job's note, read through the point** (`Queue & History` N1).
async fn the_note_of(machine: &TheDeskMachine, point: &APointLending, job: &Identity) -> Option<Fields> {
    let mut supervisor = machine.the_supervisor_over(point);
    let request = a_request(asked::THE_PER_JOB_NOTE_READING_REQUEST).carrying(Fields::none().and(asked::THE_JOB_FIELD, job.as_text()));
    match tokio::task::spawn_blocking(move || supervisor.ask(&request)).await.expect("the ask ran") {
        WhatCameBack::ItAnswered(answered) => Some(answered.fields),
        _ => None,
    }
}

/// **Where each of the job's frames is owed a file**, by the attempt's own
/// inventory, as the render worker recorded it beside the launch material.
fn where_each_frame_goes(machine: &TheDeskMachine, job: &Identity) -> Vec<(i64, PathBuf)> {
    let record_directory = PathBuf::from(
        AJobsPaths::composed(machine.layout(), job, Some("Shot 40"))
            .expect("the job's paths compose")
            .job_record_directory,
    );
    let recorded = TheReadingAndWhatFollows::read_back(
        &std::fs::read(record_directory.join(smoozy_job_files::manifest::the_recorded_readings_file_name(1)))
            .expect("the render worker wrote its recorded reading"),
    )
    .expect("the recorded reading reads back");
    let WhatTheInventoryIs::It(inventory) = &recorded.inventory else {
        panic!("the recorded reading carries an inventory: {:?}", recorded.inventory);
    };
    inventory
        .entries
        .iter()
        .filter_map(|entry| {
            Some((entry.frame?, PathBuf::from(String::from_utf8(entry.predicted_path.clone()).ok()?)))
        })
        .collect()
}

/// One step's account line, printed for the output.
fn said(step: &str, what: impl AsRef<str>) {
    println!("SC8 {step}: {}", what.as_ref());
}

// ════════════════════════════════════════════════════════════════════════════
// The scenario
// ════════════════════════════════════════════════════════════════════════════

/// **The user stops a running job**, its situation and its six steps.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_user_stops_a_running_job() {
    let desk = TheDesk::opened("sc8").expect("the desk's record opens");
    let Some(machine) = TheDeskMachine::stood_up("sc8", &desk, "RunPod 4090").await else {
        return;
    };
    let server = Identity::read_back(IdentityKind::LocalServer, &machine.the_row().local_server_identifier)
        .expect("the row's own identity");

    // ── The situation ────────────────────────────────────────────────────
    // **"Smoozy is connected to it"**: a connection lent by the server's own
    // point, and step 7 run on it as the hand-over runs it.
    let point = machine.a_point_lending().await.expect("a connection through the line is lent");
    let holds = Arc::new(WhatThisRunHolds::default());
    let connection = holds.a_connection_begins(&server.as_text());
    {
        let (mut supervisor, holds, server, record, sessions) = (
            machine.the_supervisor_over(&point),
            Arc::clone(&holds),
            server.clone(),
            Arc::clone(desk.record()),
            point.sessions.clone(),
        );
        let did = tokio::task::spawn_blocking(move || {
            let is_live = || sessions.a_connection_is_live() && holds.is_current(&server.as_text(), connection);
            the_step_seven(&server, connection, record.as_ref(), &mut supervisor, &holds, &is_live)
        })
        .await
        .expect("step 7 ran");
        assert!(did.the_queue_was_read, "`State, identity and recovery` V1 step 5: the queue was read: {did:?}");
    }
    let the_connection = TheConnection {
        machine: &machine,
        desk: &desk,
        point,
        holds,
        server: server.clone(),
        connection,
    };

    // **"A 100-frame animation job is rendering … the first 5 frames have
    // finished. Frame 6 is about twenty minutes into rendering. Nothing else
    // is queued. Auto-download is off."** By the desktop's own submit act.
    machine.the_stand_in().takes_this_long_a_frame(Duration::from_millis(50));
    machine.the_stand_in().holds_before_frame(6);
    let given = machine.what_the_page_hands_over(&desk, 1..=100, "Shot 40");
    assert!(!given.the_download_as_it_renders_choice, "auto-download is off");
    let (sessions, record) = (the_connection.point.sessions.clone(), Arc::clone(desk.record()));
    let (came_to, jobs) = tokio::task::spawn_blocking(move || {
        the_act_is_performed_naming_its_jobs(given, &sessions, &TheRunsTransfers::for_this_run(), record.as_ref())
    })
    .await
    .expect("the act ran");
    assert_eq!(
        came_to,
        WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]),
        "the job is in the machine's line: {}",
        machine.the_supervisors_complaints()
    );
    let job = jobs[0].job.clone();
    let job_text = job.as_text();

    // ── Step 1: the user watches the first frames complete ───────────────
    the_connection
        .passes_until("the queue's reading shows frames 1 to 5 saved and frame 6 in hand", Duration::from_secs(90), |it| {
            it.the_job_as_last_read(&job).is_some_and(|read| {
                matches!(read.state, QueueState::Rendering { .. })
                    && read.live.the_figures().is_some_and(|figures| {
                        figures.newest_frame_saved == Some(5) && figures.readings.value().frame_in_hand == Some(6)
                    })
            })
        })
        .await;
    {
        let held = desk.record().held();
        let screen = what_is_held(&the_connection.holds, Some(&*held), Some(&job_text));
        let entry = screen.entry.as_ref().expect("the job's entry");
        assert_eq!(entry.name, "Shot 40");
        assert_eq!(entry.stop_or_cancel, Some(TheStopOrCancel::Stop), "a rendering row offers Stop");
        let progress = entry.progress.as_ref().expect("Design, *A rendering entry*: the progress is drawn");
        let resolution = entry
            .facts
            .iter()
            .find(|fact| fact.name == "Resolution")
            .expect("Design, *A rendering entry*: the entry carries the resolution");
        // **What the user sees is wrong is the picture size**: the machine's
        // own reading of the file, filled in by `Queue & History` N9's route
        // and drawn beside the percentage.
        assert_eq!(
            resolution.beside.as_deref(),
            Some("640 × 360"),
            "the entry draws the picture size the machine read: {resolution:?}"
        );
        said(
            "STEP 1 (played)",
            format!(
                "the queue shows Shot 40 rendering, frames 1 to 5 saved and frame 6 in hand; its *Rendering now* \
                 {:?}, its bars {:?} and {:?}; its Resolution reads {:?}{}",
                progress.now,
                progress.now_at,
                progress.whole,
                resolution.value,
                resolution.beside.as_deref().map(|beside| format!(" {beside}")).unwrap_or_default(),
            ),
        );
    }
    assert_eq!(machine.the_stand_in().frames_written(), vec![1, 2, 3, 4, 5]);

    // ── Step 2: the user stops the job ───────────────────────────────────
    {
        // **The act's own order** (`the_act_is_begun`): the render named from
        // the period-held reading, the job marked in hand, the stop sent and
        // waited on, what it came to kept, and one read forward.
        let the_render = match the_connection.the_job_as_last_read(&job).map(|read| read.state) {
            Some(QueueState::Rendering { operation }) => operation,
            other => panic!("the reading shows the job rendering: {other:?}"),
        };
        it_stands(&job_text, Some(WhereTheActStands::InHand(TheQueuesOwnAct::TheStop)));
        {
            let held = desk.record().held();
            let screen = what_is_held(&the_connection.holds, Some(&*held), Some(&job_text));
            let row = screen
                .lines
                .iter()
                .flat_map(|line| line.rows.iter())
                .find(|row| row.job == job_text)
                .expect("the job's row");
            said(
                "STEP 2 (played)",
                format!(
                    "while the stop is in hand the row reads {:?} ({:?}) — `Queue & History` L12a's *stopping*",
                    row.state, row.state_is
                ),
            );
        }
        let began = Instant::now();
        let mut supervisor = machine.the_supervisor_over(&the_connection.point);
        let stood = tokio::task::spawn_blocking(move || {
            let stood = the_stop_is_sent(&mut supervisor, &the_render, TheWait::THE_STOPS_OWN);
            (stood, supervisor)
        })
        .await
        .expect("the stop ran");
        let (stood, mut supervisor) = stood;
        assert_eq!(stood, None, "`Server protocol and transfers` C21: the stop is done, the render not running");
        it_stands(&job_text, stood);
        let took = began.elapsed();
        assert!(took < Duration::from_secs(120), "`Rendering` K9: within its waiter's deadline");
        let (holds, server_for_the_pass, record) = (Arc::clone(&the_connection.holds), server.clone(), Arc::clone(desk.record()));
        tokio::task::spawn_blocking(move || {
            let _ = a_read_forward_pass(&server_for_the_pass, record.as_ref(), &mut supervisor, &holds);
        })
        .await
        .expect("the read forward ran");
        the_connection.a_pass().await;
        assert_eq!(where_the_act_stands(&job_text), None, "an act that did what it was for leaves nothing to say");
        let note = the_note_of(&machine, &the_connection.point, &job).await.expect("the machine answers");
        assert_eq!(
            note.text(asked::THE_ENDING_FIELD),
            Some(asked::IT_WAS_LEFT_STOPPED),
            "the machine's note records the job left stopped: {note:?}"
        );
        assert_eq!(note.text(asked::THE_FILES_PRESENT_FIELD), Some("5"), "five files present: {note:?}");
        assert_eq!(machine.the_stand_in().frames_written(), vec![1, 2, 3, 4, 5], "frame 6 was never written");
        said(
            "STEP 2 (played)",
            format!(
                "the stop was answered done in {took:?}; the machine's note says {:?} with {:?} files present; the \
                 stand-in wrote frames 1 to 5 and nothing of frame 6",
                note.text(asked::THE_ENDING_FIELD),
                note.text(asked::THE_FILES_PRESENT_FIELD),
            ),
        );
    }

    // ── Step 3: the user looks at the queue and at the history ───────────
    {
        let held = desk.record().held();
        let screen = what_is_held(&the_connection.holds, Some(&*held), None);
        assert!(
            screen.lines.iter().all(|line| line.rows.iter().all(|row| row.job != job_text)),
            "`Queue & History` C11: a job whose ending is learned leaves the queue"
        );
        let history = what_is_kept(Some(&*held), Some(&job_text));
        let row = history.rows.iter().find(|row| row.job == job_text).expect("the job is in History");
        assert_eq!(row.ending, Some(AnEnding::Stopped), "History says it was stopped: {row:?}");
        let entry = history.entry.as_ref().expect("its History entry");
        let record = JobRecords::over(held.connection()).read(&job).expect("it reads").expect("it stands");
        assert_eq!(record.the_ending_learned(), Some(AJobsEnding::Stopped));
        said(
            "STEP 3 (played)",
            format!(
                "the queue holds {} jobs and not this one; History reads Shot 40 {:?}, frames {:?}; its entry's \
                 failure shape says {:?}",
                screen.jobs,
                row.ending,
                row.frames,
                entry.failure.as_ref().map(|failure| failure.said.clone()),
            ),
        );
    }

    // ── Step 4: the user looks for the 5 finished frames and brings them down
    {
        let held = desk.record().held();
        let history = what_is_kept(Some(&*held), Some(&job_text));
        let entry = history.entry.as_ref().expect("its History entry");
        assert_eq!(
            entry.entry.the_folder,
            TheFolderOnTheEntry::ItHasOne,
            "the entry has its folder, so its Download all is drawn"
        );
        drop(held);
        let frames = where_each_frame_goes(&machine, &job);
        let on_the_machine: Vec<i64> = frames.iter().filter(|(_, path)| path.is_file()).map(|(frame, _)| *frame).collect();
        assert_eq!(on_the_machine, vec![1, 2, 3, 4, 5], "the machine holds frames 1 to 5 and no other");
        let sixth = frames.iter().find(|(frame, _)| *frame == 6).map(|(_, path)| path.clone()).expect("frame 6 is owed a file");
        assert!(!sixth.exists(), "nothing stands where frame 6 would be: {}", sixth.display());
        // **Design, *The entry's controls*: *Download all to…*, the folder
        // picked in the panel** — a folder of the user's own on this
        // computer, never the system's own Downloads folder.
        let to = desk.the_users_folder().to_path_buf();
        let downloaded = a_whole_job_is_downloaded_to(&desk, &the_connection.point.sessions, &job, &to)
            .await
            .expect("`Download` M1: the whole job's request is made and run");
        assert_eq!(downloaded.came_to.fetched, 5, "`Download` M16: the five frames fetched: {downloaded:?}");
        assert!(downloaded.came_to.failed.is_empty() && downloaded.came_to.did_not_finish == 0, "{downloaded:?}");
        let names: Vec<String> = downloaded
            .landed
            .iter()
            .filter_map(|path| path.file_name().map(|name| name.to_string_lossy().into_owned()))
            .collect();
        let expected: Vec<String> = frames
            .iter()
            .filter(|(frame, _)| *frame <= 5)
            .filter_map(|(_, path)| path.file_name().map(|name| name.to_string_lossy().into_owned()))
            .collect();
        let mut sorted = names.clone();
        sorted.sort();
        let mut expected_sorted = expected.clone();
        expected_sorted.sort();
        assert_eq!(sorted, expected_sorted, "frames 1 to 5 came down, and nothing stands for frame 6: {:?}", downloaded.landed);
        for (frame, path) in frames.iter().filter(|(frame, _)| *frame <= 5) {
            let name = path.file_name().expect("a name");
            let here = downloaded.landed.iter().find(|landed| landed.file_name() == Some(name)).expect("it landed");
            assert_eq!(
                std::fs::read(here).expect("it reads"),
                std::fs::read(path).expect("the machine's reads"),
                "frame {frame} came down whole"
            );
        }
        let pre_flight = downloaded.the_pre_flight;
        said(
            "STEP 4 (played)",
            format!(
                "History's entry offers Download all, and the machine holds frames 1 to 5 and nothing where frame 6 \
                 would be. Download all to… the user's own folder: the pre-flight {} ({} already there, {} it cannot \
                 vouch for, {} it could not read){}; {} frames came down — {names:?} — and nothing for frame 6",
                if downloaded.it_asked { "asked first" } else { "asked nothing" },
                pre_flight.the_headline(),
                pre_flight.cannot_vouch_for,
                pre_flight.refusals,
                if downloaded.it_asked { ", answered with Download" } else { "" },
                downloaded.came_to.fetched,
            ),
        );
    }

    // ── Step 5: the user deletes the stopped job from the history ────────
    {
        let asked = {
            let held = desk.record().held();
            the_history_as_asked(what_is_kept(Some(&*held), Some(&job_text)), Some(&job_text), Some(TheHistoryAsk::Delete))
        };
        let the_box = asked.asking.as_ref().expect("Design, *Deleting an entry*: the delete box asks first");
        assert_eq!(the_box.go_ahead_act, TheActOnAJob::DeleteTheEntry);
        let done = {
            let started = desk.record().the_start().lock().expect("the start");
            the_act_is_performed(&started, TheActOnAJob::DeleteTheEntry, Some(&job_text))
        };
        assert_eq!(done, Some(Ok(())), "`Queue & History` X1: the entry is deleted");
        let owed = desk
            .record()
            .read(|connection| TheDeletions::over(connection).the_jobs_still_owing_their_erase())
            .expect("the tombstones read");
        assert!(owed.contains(&job), "`Queue & History` X4: the machine's half is owed, remembered as a deletion");
        said(
            "STEP 5 (played)",
            format!(
                "the delete box asked {:?} / {:?}, with *{}* on its go-ahead; the entry was deleted, and its erase on \
                 the machine is owed to the next connection",
                the_box.heading, the_box.said, the_box.go_ahead.iter().map(|part| part.lead).collect::<String>(),
            ),
        );
    }

    // ── Step 6: the user looks at the history again ──────────────────────
    {
        let held = desk.record().held();
        let history = what_is_kept(Some(&*held), None);
        assert!(history.rows.iter().all(|row| row.job != job_text), "the entry is gone from History");
        assert!(JobRecords::over(held.connection()).read(&job).expect("it reads").is_none(), "and its record");
        drop(held);
        let frames = where_each_frame_goes(&machine, &job);
        let still: Vec<i64> = frames.iter().filter(|(_, path)| path.is_file()).map(|(frame, _)| *frame).collect();
        assert_eq!(still, vec![1, 2, 3, 4, 5], "the delete box's word: the frames on the server are untouched");
        said(
            "STEP 6 (played)",
            format!(
                "History holds {} entries and not this one; the machine still holds frames {still:?}",
                history.jobs
            ),
        );
    }
}

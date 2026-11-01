//! **`Preparation/Scenarios/The server disk fills up during render.md`, played
//! at the desk** — step 12's piece SC4.
//!
//! The scenario's situation and its six steps, unchanged, through the
//! application's own acts, against the desk machine. At each step the test
//! prints what the user did, what Smoozy hands the page to show, and which
//! clause says so, and asserts all three. **A step the desk cannot play is said
//! in the output and never skipped quietly.**
//!
//! **How the full disk is made: a small throwaway volume, and not a stand-in.**
//! The machine's root — its five areas, its package area, and every record its
//! program keeps — sits on a 160 MB volume of its own, made and attached by
//! macOS's own `hdiutil` as the user running the test, with no administrator
//! (the harness's `TheSmallVolume` cites the manual page). So the machine's
//! program, its records and the job's frames share one disk, as on a rental
//! with one disk, and **the disk is filled to nothing for real**: every write
//! on it is refused by the system for want of room.
//!
//! **The scenario's sizes are scaled, and that is the one liberty.** 4 GB free
//! and 100 MB frames make the disk full at frame 40. Here the frames are the
//! stand-in's one-pixel picture, and the room frames 1 to 40 would have taken
//! is taken by **one filler file the test writes the moment frame 40 is saved**,
//! down to the last byte the volume will take — the same filler the rented pass
//! uses (the step 12 map, SC4). Step 5's *"clears 100GB"* is that file deleted.
//!
//! **What is real here.** OpenSSH's own server behind the harness's line; the
//! application's own transport and the server's own point; `Servers` Q1 step 7
//! on the connection; the desktop's own submit act; the read forward and the
//! queue reading a pass takes; the Files screen's own reading of a folder and
//! its free-space figure; the Queue and History screens' own readings; the
//! carry-on's own figure and the press composed from it; and the machine's own
//! program — its real supervisor, its real render worker — on a disk that is
//! really full. **What is stood in for**: Blender, by the harness's script. When
//! its write of frame 41 is refused it says so on its error stream and ends
//! with a failing status, which is the ending `What a job writes` (Specs) gives
//! a job with a frame still to come. **What Blender itself does on a full Linux
//! disk is not asserted here**, and what it prints about the disk is
//! unresolved in the Specs: the rented pass plays a real Blender.
//!
//! **Where the application's own wiring needs the window, it is mirrored and
//! said**, as in SC8 (`tests/the_user_stops_a_running_job.rs`, whose
//! connection and passes this file copies): the point lending one connection,
//! step 7 run on it, and a pass as the read forward and the queue reading. Two
//! private pieces of the *Carry on* press are mirrored at step 6, and say so
//! where they stand.
//!
//! **What the desk cannot do.**
//!
//! - **A Linux disk.** The volume is macOS's HFS+, and the program here is its
//!   development build; the refusal is the same `No space left on device`
//!   (errno 28 on both systems), and what a real Linux mount does around it is
//!   the rented pass's.
//! - **A real Blender's part-written frame.** The stand-in copies its picture
//!   with Perl's own copy; what it leaves at frame 41's name is read and said,
//!   and nothing about Blender is claimed from it.
//! - **The user freeing space with their own tools.** The filler is deleted by
//!   the test, which is what the user's own command would do.
//!
//! **Nothing of the Creator's is touched**: see the head of
//! `src/the_desk_machine.rs`.

#![cfg(unix)]

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use smoozy_desktop::a_new_job_from_an_old_one::{the_press_composed, TheWayIn, WhatTheBlendIs, WhatTheServerIs, WhereThePressLeads};
use smoozy_desktop::downloading::{TheRunsTransfers, TheWalksReadingRoute};
use smoozy_desktop::record::job_records::{JobRecords, TheKeptSetup};
use smoozy_desktop::servers::the_machines_work::{
    a_read_forward_pass, the_queue_reading_from, the_step_seven, AsksTheSupervisor, WhatThisRunHolds,
};
use smoozy_desktop::submitting::{the_act_is_performed_naming_its_jobs, WhatBecameOfOneJob, WhatTheActCameTo};
use smoozy_desktop::surface::files::{a_folder_is_read, the_tab_after_the_reading, TheListing, TheOrder, Words};
use smoozy_desktop::surface::render_settings::WhereItCameFrom;
use smoozy_desktop::surface::the_history::{the_endings_owed_in, what_is_kept, what_is_kept_beside, AnEnding};
use smoozy_desktop::surface::the_queue::{what_is_held, APart, ARowInTheQueue, TheQueueOnScreen};
use smoozy_job_files::composing::TheReadingAndWhatFollows;
use smoozy_job_files::publication::AJobsPaths;
use smoozy_queue_history::line::QueueState;
use smoozy_render_settings::frames::FrameForm;
use smoozy_rendering::again::{the_carry_ons_figure_over, TheCarryOnsFigure, WhatTheCarryOnRead};
use smoozy_scenarios::the_desk_machine::{a_request, APointLending, TheDesk, TheDeskMachine};
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::Fields;
use smoozy_server_protocol::requests as asked;
use smoozy_servers::condition::ObservedValue;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, Reading};

/// **The machine's disk**: big enough that `Rented-server runtime` M4a's
/// control-state reserve stands when the render starts, and small enough that
/// filling it costs this Mac nothing to speak of.
const THE_MACHINES_DISK_IN_MEGABYTES: u64 = 160;

/// The filler's name, on the machine's own disk beside its five areas.
const THE_FILLER: &str = "the room frames 1 to 40 took";

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
    /// **One pass of `Desktop app and delivery` P10d**: the read forward, then
    /// the queue reading, landed on this connection — off the test's own
    /// threads, because every ask blocks.
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
    async fn passes_until(&self, said: &str, within: Duration, seen: impl Fn(&Self) -> bool) -> Duration {
        let began = Instant::now();
        loop {
            self.a_pass().await;
            if seen(self) {
                return began.elapsed();
            }
            assert!(began.elapsed() < within, "{said}: not seen within {within:?}");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    /// **A fixed number of passes, a second apart**, for a step that looks
    /// at what stays the same.
    async fn passes(&self, how_many: usize) {
        for _ in 0..how_many {
            self.a_pass().await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    /// **The Queue screen, with the job's entry open.**
    fn the_queue(&self, job: &str) -> TheQueueOnScreen {
        let held = self.desk.record().held();
        what_is_held(&self.holds, Some(&*held), Some(job))
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

/// **The Files screen on one folder of the machine**, as `the_folder_is_read`
/// composes it: the folder read on the connection's reading session, with
/// `File Browser` L11's free-space figure against the same path, and the
/// browser composed from what the reading found.
async fn files_on(machine: &TheDeskMachine, point: &APointLending, folder: &str) -> smoozy_desktop::surface::files::TheBrowser {
    let found = a_folder_is_read(&point.sessions, folder.as_bytes()).await;
    the_tab_after_the_reading(
        TheOrder::default(),
        &machine.the_row().local_server_identifier,
        folder.as_bytes(),
        None,
        found,
        Some(ObservedValue::Connected),
        false,
    )
}

/// **Words as the page writes them**, prose and values in their order.
fn as_written(words: &Words) -> String {
    words
        .pieces
        .iter()
        .map(|piece| {
            piece
                .said
                .as_ref()
                .map(|said| said.as_str().to_string())
                .or_else(|| piece.value.as_ref().map(|value| value.as_str().to_string()))
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// **A line of parts as the page writes it.**
fn parts(parts: &[APart]) -> String {
    parts.iter().map(|part| format!("{}{}{}", part.lead, part.value, part.trail)).collect()
}

/// **The rows Files lists**, each its name and its size.
fn the_rows(browser: &smoozy_desktop::surface::files::TheBrowser) -> Vec<(String, Option<String>)> {
    match &browser.listing {
        TheListing::Rows { rows, .. } => rows
            .iter()
            .map(|row| {
                (
                    String::from_utf8_lossy(&row.name.bytes).into_owned(),
                    row.size.as_ref().map(|size| size.as_str().to_string()),
                )
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// The name the stand-in's frame `frame` lands under.
fn name_of(frame: i64) -> String {
    format!("f_{frame:04}.png")
}

fn the_row_of<'a>(screen: &'a TheQueueOnScreen, job: &str) -> Option<&'a ARowInTheQueue> {
    screen.lines.iter().flat_map(|line| line.rows.iter()).find(|row| row.job == job)
}

/// **The machine's disk, filled to the last byte it will take** — chunks of a
/// megabyte, then smaller ones, each until the system refuses the write. What
/// was written is handed back, and the refusal the last write met.
fn the_disk_is_filled(root: &Path) -> (u64, std::io::Error) {
    let mut filler = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(root.join(THE_FILLER))
        .expect("the filler is opened");
    let chunk = vec![0u8; 1 << 20];
    let mut written = 0u64;
    let mut refused = None;
    for size in [1 << 20, 1 << 16, 1 << 12, 1 << 9, 1] {
        loop {
            match filler.write(&chunk[..size]) {
                Ok(0) => break,
                Ok(wrote) => written += wrote as u64,
                Err(why) => {
                    refused = Some(why);
                    break;
                }
            }
        }
    }
    let _ = filler.sync_all();
    (written, refused.unwrap_or_else(|| std::io::Error::other("no write was refused")))
}

/// **How long the machine's work in hand at the fill is given to end before
/// the disk is filled again.** See step 2.
const THE_WORK_IN_HAND_ENDS_WITHIN: Duration = Duration::from_secs(2);

/// One step's account line, printed for the output.
fn said(step: &str, what: impl AsRef<str>) {
    println!("SC4 {step}: {}", what.as_ref());
}

// ════════════════════════════════════════════════════════════════════════════
// Step 6's press, and the two pieces of it that are mirrored
// ════════════════════════════════════════════════════════════════════════════

/// **`Rendering` A4's two readings of the machine, taken over the server's own
/// point as the press takes them**, and the figure `Rendering`'s own
/// `the_carry_ons_figure_over` reckons from them.
///
/// - **The walk** is the desktop's own route (`TheWalksReadingRoute`) and
///   `Download`'s own walk of the job output folder.
/// - **The map in force** is mirrored line for line from the desktop's
///   crate-private `the_map_in_force_on_the_machine` in
///   `src/servers/the_machines_work.rs`, which a test cannot name: the newest
///   attempt record carrying a frame list, and the inventory of the newest
///   recorded reading, each read through the supervisor's own seam for a file
///   of the job record directory.
async fn the_carry_on_is_read(
    machine: &TheDeskMachine,
    point: &APointLending,
    record_directory: String,
    attempts: u32,
    folder: String,
    kept: Option<i64>,
) -> (TheCarryOnsFigure, Option<Vec<i64>>) {
    use smoozy_job_files::reckoning::{AWalkEntry, WhatKindOfEntry, WhatTheWalkOfTheFolderIs};
    let mut supervisor = machine.the_supervisor_over(point);
    let map = tokio::task::spawn_blocking(move || {
        let under = |name: &str| format!("{record_directory}/{name}");
        let frame_list = (1..=attempts).rev().find_map(|attempt| {
            supervisor
                .a_file_of_the_job_record_directory(&under(&format!("attempt-{attempt}.json")))
                .and_then(|bytes| smoozy_job_files::attempts::AttemptRecord::read_back(&bytes))
                .map(|record| record.frame_list_in_the_order_it_was_written)
                .filter(|list| !list.is_empty())
        })?;
        let inventory = (1..=attempts).rev().find_map(|attempt| {
            supervisor
                .a_file_of_the_job_record_directory(&under(&smoozy_job_files::manifest::the_recorded_readings_file_name(attempt)))
                .and_then(|bytes| TheReadingAndWhatFollows::read_back(&bytes))
                .map(|reading| reading.inventory)
        })?;
        Some((frame_list, inventory))
    })
    .await
    .expect("the reading ran");
    let walked = {
        let mut route = TheWalksReadingRoute::on(&point.sessions);
        smoozy_download::acting::walking::walk_the_folder_and_everything_under_it(&mut route, folder.as_bytes()).await
    };
    let walk = match walked.walk {
        smoozy_download::watch::AWalk::Complete(entries) => WhatTheWalkOfTheFolderIs::ItCompleted(
            entries
                .into_iter()
                .map(|entry| AWalkEntry {
                    kind: if entry.is_a_folder {
                        WhatKindOfEntry::ADirectory
                    } else if walked.the_plain_files.contains(&entry.path) {
                        WhatKindOfEntry::AFile
                    } else {
                        WhatKindOfEntry::SomethingElse
                    },
                    path_under_the_job_output_folder: entry.path,
                    size: entry.size,
                })
                .collect(),
        ),
        smoozy_download::watch::AWalk::Discarded(_) => WhatTheWalkOfTheFolderIs::ItDidNotComplete,
    };
    let Some((frame_list, inventory)) = map else {
        return (
            TheCarryOnsFigure::TheMachineCouldNotBeChecked {
                why: smoozy_rendering::again::TheMachineCouldNotBeChecked::NoAttemptRecordOnTheMachineCarriesAFrameList,
                case: smoozy_rendering::again::the_could_not_check_case(kept),
            },
            None,
        );
    };
    let figure = the_carry_ons_figure_over(
        WhatTheCarryOnRead {
            the_walk: &walk,
            the_job_output_folder: &folder,
            the_inventory_for_the_map_in_force: &inventory,
            the_frame_list_asked_for: &frame_list,
        },
        kept,
    );
    (figure, Some(frame_list))
}

/// **`Rendering` A10's one attribute read of the `.blend`'s own path**, on the
/// file-transfer route — mirrored from the desktop's private
/// `the_blend_is_read` in `src/a_new_job_from_an_old_one.rs`.
async fn the_blend_is_read(point: &APointLending, path: &str) -> WhatTheBlendIs {
    use smoozy_server_protocol::subsystem::{StatusCode, SubsystemFailure};
    let path = path.as_bytes().to_vec();
    let answered = point
        .sessions
        .a_reading(true, move |subsystem| Box::pin(async move { subsystem.stat(&path).await.map(|_| ()) }))
        .await;
    match answered {
        Ok(Ok(())) => WhatTheBlendIs::There,
        Ok(Err(SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile))) => WhatTheBlendIs::Gone,
        Ok(Err(SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied))) => WhatTheBlendIs::MayNotBeRead,
        _ => WhatTheBlendIs::TheServerDidNotAnswer,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The scenario
// ════════════════════════════════════════════════════════════════════════════

/// **The server disk fills up during render**, its situation and its six
/// steps.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_server_disk_fills_up_during_render() {
    let desk = TheDesk::opened("sc4").expect("the desk's record opens");
    let Some(machine) =
        TheDeskMachine::stood_up_with_its_root_on_a_small_volume("sc4", &desk, "RunPod 4090", THE_MACHINES_DISK_IN_MEGABYTES).await
    else {
        println!("SC4 NOT PLAYED: no desk machine with its root on a small volume could stand here; nothing below ran.");
        return;
    };
    let server = Identity::read_back(IdentityKind::LocalServer, &machine.the_row().local_server_identifier)
        .expect("the row's own identity");
    let root = PathBuf::from(machine.layout().root().as_path());
    said(
        "HOW THE DISK IS MADE",
        format!(
            "the machine's root {} is a {THE_MACHINES_DISK_IN_MEGABYTES} MB volume of its own, made and attached by \
             hdiutil as this user; frames are the stand-in's one-pixel picture, and the room frames 1 to 40 would \
             take is taken by one filler file written once frame 40 is saved",
            root.display()
        ),
    );

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

    // **"A 100-frame animation job is rendering on it. Its `.blend` and
    // everything it needs went up in an earlier sitting. … Nothing else is
    // queued. Auto-download is off."** By the desktop's own submit act, with
    // the stand-in held before frame 41.
    machine.the_stand_in().takes_this_long_a_frame(Duration::from_millis(50));
    machine.the_stand_in().holds_before_frame(41);
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
    let record_directory = AJobsPaths::composed(machine.layout(), &job, Some("Shot 40"))
        .expect("the job's paths compose")
        .job_record_directory;

    // ── Step 1: the user watches the progress as frames complete ─────────
    the_connection
        .passes_until("the queue's reading shows frames 1 to 40 saved and frame 41 in hand", Duration::from_secs(90), |it| {
            it.the_job_as_last_read(&job).is_some_and(|read| {
                matches!(read.state, QueueState::Rendering { .. })
                    && read.live.the_figures().is_some_and(|figures| {
                        figures.newest_frame_saved == Some(40) && figures.readings.value().frame_in_hand == Some(41)
                    })
            })
        })
        .await;
    let folder = {
        let held = desk.record().held();
        JobRecords::over(held.connection())
            .read(&job)
            .expect("it reads")
            .expect("it stands")
            .the_job_output_folder(machine.layout())
            .expect("`Queue & History` N9: the folder chosen at render start is known")
    };
    let before = files_on(&machine, &the_connection.point, &folder).await;
    {
        let screen = the_connection.the_queue(&job_text);
        let row = the_row_of(&screen, &job_text).expect("the job's row");
        let entry = screen.entry.as_ref().expect("the job's entry");
        assert_eq!(row.state, Some("Rendering"), "`Queue & History` C6: the row reads rendering: {row:?}");
        let progress = entry.progress.as_ref().expect("Design, *A rendering entry*: the progress is drawn");
        said(
            "STEP 1 (played)",
            format!(
                "the queue shows Shot 40 {:?}, *Rendering now* {:?}, its bars {:?} and {:?}; Files on the job's folder \
                 lists {} files and says {:?}",
                row.state,
                progress.now,
                progress.now_at,
                progress.whole,
                the_rows(&before).len(),
                before.bottom.free_space.as_ref().map(as_written),
            ),
        );
    }
    assert_eq!(machine.the_stand_in().frames_written(), (1..=40).collect::<Vec<i64>>());

    // ── Step 2: frame 40 finishes, the disk is full, frame 41 begins ─────
    let the_whole_picture = std::fs::metadata(Path::new(&folder).join(name_of(1))).expect("frame 1 is on the machine").len();
    let (filled, refusal) = the_disk_is_filled(&root);
    assert_eq!(refusal.raw_os_error(), Some(libc::ENOSPC), "the system refused the last write for want of room: {refusal}");
    // **Room that comes back after the fill is taken too.** One run of this
    // test read *24 KB free* straight after a single fill (step 12's SC4
    // findings, 2026-10-25): room came back after the fill and before frame
    // 41 was let go, so the disk was not full at the moment the scenario says
    // it is. **Where it came from is not established**; the likeliest is the
    // machine's own work in hand at the fill — a preview of frame 40 being
    // made — giving back what it had staged when it ended on the full disk. A
    // real frame 41 of 100 MB would take that room anyway; the stand-in's 72
    // bytes would not. So the work in hand is given a fixed two seconds to
    // end, and the filler takes whatever came back.
    tokio::time::sleep(THE_WORK_IN_HAND_ENDS_WITHIN).await;
    let (topped_up, refusal) = the_disk_is_filled(&root);
    assert_eq!(refusal.raw_os_error(), Some(libc::ENOSPC), "the system refused the last write for want of room: {refusal}");
    let full = files_on(&machine, &the_connection.point, &folder).await;
    let free_when_full = full.bottom.free_space.as_ref().map(as_written).unwrap_or_default();
    // **`File Browser` L11's figure against the job's own folder, and
    // `Server protocol and transfers` F1a's "on the machine deliberately
    // filled, the reading read `0` free"** — the figure does not lag or round
    // at the boundary.
    assert!(
        free_when_full.starts_with("0\u{a0}bytes"),
        "`File Browser` L11: Files reads nothing free on the full disk: {free_when_full:?}"
    );
    machine.the_stand_in().lets_frame_go(41);
    said(
        "STEP 2 (played on a full volume)",
        format!(
            "the filler took {filled} bytes, and {topped_up} more two seconds later, and the next write was refused \
             ({refusal}); Files on the job's folder now says {free_when_full:?}; frame 41 is let go, and the \
             stand-in's write of it meets the full disk"
        ),
    );

    // ── Step 3: the user looks at the queue entry ────────────────────────
    // **What the test reads to know the render has ended, and Smoozy reads
    // none of it**: the supervisor's own complaints stream, which says when
    // the worker's report of the ending reached it and could not be written.
    let began = Instant::now();
    while !machine.the_supervisors_complaints().contains("a render's ending was reported and not written") {
        assert!(
            began.elapsed() < Duration::from_secs(60),
            "the render worker reported the ending within a minute: {}",
            machine.the_supervisors_complaints()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    let ended_after = began.elapsed();
    the_connection.passes(3).await;
    {
        let read = the_connection.the_job_as_last_read(&job);
        let screen = the_connection.the_queue(&job_text);
        let row = the_row_of(&screen, &job_text);
        let entry = screen.entry.as_ref();
        let note = the_note_of(&machine, &the_connection.point, &job).await;
        let what_it_showed = format!(
            "the machine's reading of the job {:?}; the row {:?}; the entry's status {:?}, its control {:?}, its \
             progress {:?}; the note's ending {:?}",
            read.as_ref().map(|read| &read.state),
            row.and_then(|row| row.state),
            entry.map(|entry| parts(&entry.status)),
            entry.and_then(|entry| entry.stop_or_cancel),
            entry.map(|entry| entry.progress.is_some()),
            note.as_ref().map(|note| note.text(asked::THE_ENDING_FIELD)),
        );
        said(
            "STEP 3 (played)",
            format!(
                "the render ended about {ended_after:?} after frame 41 was let go and its ending could not be written; \
                 {what_it_showed}"
            ),
        );
        // **`Rented-server runtime` S14's first bullet: the run does not end
        // for a refused write**, and the supervisor goes on serving.
        assert!(machine.the_supervisor_is_running(), "`Rented-server runtime` S14: the supervisor is still running");
        let note = note.expect("`Rented-server runtime` S14: the supervisor answers the note reading on a full disk");
        // **`Queue & History` (Specs), *The note can be read*: "a job that ends
        // in that window leaves the note still showing no ending until there
        // is room again. The note waits rather than guessing."**
        assert_eq!(note.text(asked::THE_ENDING_FIELD), None, "the note shows no ending while the disk is full: {note:?}");
        assert_eq!(machine.the_stand_in().frames_written(), (1..=40).collect::<Vec<i64>>(), "frame 41 was never saved");
        // **`Rented-server runtime` S14, *Readings stay truthful*: "the queue
        // reading's answer for a job whose group has ended carries the pending
        // recorded outcome as that job's state rather than *rendering* … so the
        // entry the user watches moves off *rendering* while the durable
        // account catches up at the landing"** — and `Rented-server runtime`
        // S14a carries the rest, `Rented-server runtime` V12's reading among
        // it, so the disk-full diagnosis reaches the entry inside the window.
        assert!(
            !matches!(read.as_ref().map(|read| &read.state), Some(QueueState::Rendering { .. })),
            "FINDING 1 — `Rented-server runtime` S14 (*Readings stay truthful*) and `Rented-server runtime` S14a: the render has ended and its \
             ending is owed to the full disk, so the queue reading carries the pending outcome as the job's state \
             rather than rendering. The app showed: {what_it_showed}"
        );
        assert_ne!(
            row.and_then(|row| row.state),
            Some("Rendering"),
            "`Rented-server runtime` S14: the entry moves off rendering: {what_it_showed}"
        );
        // **`Queue & History` C11: the job leaves the queue and the history
        // entry takes the ending Smoozy learns — from watching**; and
        // `Rented-server runtime` S14a: *"carrying it forward is what lets the
        // disk-full diagnosis reach the user inside the window"*. **It moves;
        // it does not vanish** (step 12's SC4 follow-up).
        let history = {
            let held = desk.record().held();
            what_is_kept_beside(
                Some(&*held),
                Some(&job_text),
                &|_| true,
                &the_endings_owed_in(&the_connection.holds),
            )
        };
        let in_history = history.rows.iter().find(|row| row.job == job_text);
        let failure = history
            .entry
            .as_ref()
            .filter(|entry| entry.entry.job == job_text)
            .and_then(|entry| entry.failure.as_ref());
        said(
            "STEP 3 (played), History",
            format!(
                "History lists Shot 40 {:?}, from the reading {:?}; its failure says {:?}, a second sentence {:?}, \
                 Delete offered {}",
                in_history.and_then(|row| row.ending),
                in_history.map(|row| row.the_ending_is_owed),
                failure.map(|failure| &failure.said),
                failure.and_then(|failure| failure.what_to_do_about_it.as_ref()),
                history.entry.as_ref().is_some_and(|entry| !entry.the_ending_is_owed),
            ),
        );
        assert_eq!(
            in_history.and_then(|row| row.ending),
            Some(AnEnding::Failed),
            "`Queue & History` C11 and `Rented-server runtime` S14: between the fill and the landing the job is in \
             History, failed, from the reading — on neither list it would be nowhere: {what_it_showed}"
        );
        assert!(in_history.is_some_and(|row| row.the_ending_is_owed), "the ending is the reading's until the landing");
        assert!(
            failure.is_some_and(|failure| failure.what_to_do_about_it.is_some()),
            "`Rented-server runtime` S14a and `Rented-server runtime` V12: the disk-full diagnosis reaches the user inside the window: {:?}",
            failure
        );
    }

    // ── Step 4: what is on the machine, and what came of frames 40 and 41
    {
        let browser = files_on(&machine, &the_connection.point, &folder).await;
        let rows = the_rows(&browser);
        let free = browser.bottom.free_space.as_ref().map(as_written).unwrap_or_default();
        let forty = rows.iter().find(|(name, _)| *name == name_of(40)).cloned();
        let forty_one = rows.iter().find(|(name, _)| *name == name_of(41)).cloned();
        let on_disk = |frame: i64| std::fs::metadata(Path::new(&folder).join(name_of(frame))).map(|meta| meta.len()).ok();
        said(
            "STEP 4 (played)",
            format!(
                "Files lists {} rows in the job's folder and says {free:?}; frame 40's row {forty:?} ({:?} bytes, a \
                 whole picture is {the_whole_picture}); frame 41's row {forty_one:?} ({:?} bytes on the disk)",
                rows.len(),
                on_disk(40),
                on_disk(41),
            ),
        );
        assert!(free.starts_with("0\u{a0}bytes"), "`File Browser` L11: still nothing free: {free:?}");
        let names: Vec<String> = rows.iter().map(|(name, _)| name.clone()).collect();
        assert_eq!(names, (1..=40).map(name_of).collect::<Vec<_>>(), "Files lists frames 1 to 40 and nothing else");
        assert!(forty.is_some(), "frame 40 is listed");
        assert_eq!(on_disk(40), Some(the_whole_picture), "frame 40 was saved before the disk filled and is whole");
        // **What came of frame 41 is what the full disk left**, and Smoozy
        // claims nothing about it either way (`What a job writes`, Specs: a
        // file left by a write that ran out of room is one Smoozy cannot treat
        // as finished). Here the refused write left no file at its name.
        assert_ne!(on_disk(41), Some(the_whole_picture), "frame 41 is not on the machine whole");
    }

    // ── Step 5: the user clears space on the machine ─────────────────────
    std::fs::remove_file(root.join(THE_FILLER)).expect("the filler is deleted");
    // **`Rented-server runtime` S14: "a bounded delay, never a loss"** — the
    // owed ending lands at the next occasion that something else causes, and
    // the user's own next connection is requests.
    let took = the_connection
        .passes_until("History learns the job's ending once there is room", Duration::from_secs(60), |it| {
            let held = it.desk.record().held();
            JobRecords::over(held.connection())
                .read(&job)
                .ok()
                .flatten()
                .and_then(|record| record.the_ending_learned())
                .is_some()
        })
        .await;
    {
        let freed = files_on(&machine, &the_connection.point, &folder).await;
        let free = freed.bottom.free_space.as_ref().map(as_written).unwrap_or_default();
        let note = the_note_of(&machine, &the_connection.point, &job).await.expect("the machine answers");
        let complaints = |field: &str| note.text(&format!("capture record 0: complaints stream{field}")).map(str::to_string);
        said(
            "STEP 5 (played)",
            format!(
                "the filler deleted; Files says {free:?}; the ending landed and History learned it {took:?} later; the \
                 note reads {:?}, exit status {:?}, last frame {:?}, {:?} files present and {:?} absent; Blender's \
                 error stream was {:?}, {:?} bytes kept and {:?} thrown away, first lost to {:?}",
                note.text(asked::THE_ENDING_FIELD),
                note.text("exit status"),
                note.text(asked::THE_LAST_FRAME_FIELD),
                note.text(asked::THE_FILES_PRESENT_FIELD),
                note.text("files absent"),
                complaints("'s outcome"),
                complaints(" retained"),
                complaints(" discarded"),
                complaints("'s first loss"),
            ),
        );
        assert!(!free.starts_with("0\u{a0}bytes"), "`File Browser` L11: room again: {free:?}");
        // **The landing replaces the reading's ending with the record's**
        // (step 12's SC4 follow-up): nothing is owed any more, and History's
        // entry is the record's own.
        let owed = the_endings_owed_in(&the_connection.holds);
        assert!(owed.is_empty(), "nothing is owed once the ending lands: {owed:?}");
        let history = {
            let held = desk.record().held();
            what_is_kept_beside(Some(&*held), Some(&job_text), &|_| true, &owed)
        };
        let row = history.rows.iter().find(|row| row.job == job_text).expect("the job is in History");
        assert!(!row.the_ending_is_owed, "the record's ending, after the landing");
        assert_eq!(row.ending, Some(AnEnding::Failed));
        assert_eq!(note.text(asked::THE_ENDING_FIELD), Some("failed"), "`Queue & History` N5: the note catches up: {note:?}");
        assert_eq!(note.text(asked::THE_FILES_PRESENT_FIELD), Some("40"), "`Job files and outputs` C27: forty present");
        // **`Rented-server runtime` M5: a sink write refused ends durable
        // writes for that stream and never its read**, the first reason
        // recorded once — so what the stand-in said about frame 41 is lost to
        // the full disk, and the record says so rather than presenting it as
        // collected.
        assert_eq!(complaints("'s outcome").as_deref(), Some("collection incomplete"), "`Rented-server runtime` M5: {note:?}");
        assert!(
            complaints("'s first loss").is_some_and(|why| why.contains("No space left on device")),
            "`Rented-server runtime` M5: the first loss names the full disk: {note:?}"
        );
    }

    // ── Step 6: the user looks for a way to get the rest rendered ────────
    {
        let (record, kept, history) = {
            let held = desk.record().held();
            let records = JobRecords::over(held.connection());
            let record = records.read(&job).expect("it reads").expect("it stands");
            let kept = match records.the_kept_setup_of(&job).expect("it reads") {
                Some(TheKeptSetup::Kept(payload)) => payload,
                other => panic!("`Queue & History` N10: the record kept the setup: {other:?}"),
            };
            (record, kept, what_is_kept(Some(&*held), Some(&job_text)))
        };
        let row = history.rows.iter().find(|row| row.job == job_text).expect("the job is in History");
        let entry = history.entry.as_ref().expect("its History entry");
        let failure = entry.failure.as_ref().expect("Design, the failure shape on a failed ending");
        let again = entry.again.as_ref().expect("Design, *The two ways to render it again*");
        let carry_on = again.carry_on.as_ref().expect("Design: Carry on is offered on a job that ended short");
        // **The *Carry on* press: `Rendering` A4's figure read from the machine
        // when the action is taken**, and the page composed from it.
        let kept_figure = record.what_the_note_carried.last_frame_figure.or(record.what_the_note_carried.given_up_where_it_was);
        let (figure, list) = the_carry_on_is_read(
            &machine,
            &the_connection.point,
            record_directory.clone(),
            record.what_the_note_carried.attempts,
            folder.clone(),
            kept_figure,
        )
        .await;
        let blend = the_blend_is_read(&the_connection.point, &machine.the_blend().display().to_string()).await;
        let pressed = the_press_composed(
            &record,
            &kept,
            &TheWayIn::CarryOn { typed: None },
            &WhatTheServerIs::Connected,
            blend,
            Some(&figure),
            list.as_deref(),
        )
        .expect("the press composes");
        said(
            "STEP 6 (played)",
            format!(
                "History reads Shot 40 {:?}, frames {:?}; its failure says {:?}, a second sentence {:?}, What the server \
                 said {:?}, Report a problem {}; it offers Submit again — {} frames and Resume — {} frames from frame \
                 {}; Resume pressed: the machine read {figure:?}, the .blend {blend:?}, and the page opens on {:?} \
                 with the band {:?}. Blender output is not drawn here: the words are fetched by a pass that needs the \
                 window, which this test does not mirror",
                row.ending,
                row.frames,
                failure.said,
                failure.what_to_do_about_it,
                failure.what_the_server_said,
                if failure.report_a_problem { "offered" } else { "not offered" },
                again.submit_again,
                carry_on.frames,
                carry_on.at,
                pressed.the_range,
                pressed.came_from.band,
            ),
        );
        assert_eq!(row.ending, Some(AnEnding::Failed), "History says it failed: {row:?}");
        assert!(failure.report_a_problem, "Design: *Report a problem…* is on every failed ending but a stop");
        assert_eq!(again.submit_again, 100, "`Rendering` A1: Submit again is the whole range");
        // **`Rendering` A4: the range opens at the last frame that counts as
        // already there, that frame re-rendered** — frame 40, which the full
        // disk's frame 41 leaves as the boundary.
        assert_eq!((carry_on.at, carry_on.frames), (40, 61), "`Rendering` A4: Carry on from frame 40, 61 frames");
        assert_eq!(figure, TheCarryOnsFigure::ReadFromTheMachine(Some(40)), "`Rendering` A4: read from the machine");
        assert!(
            matches!(pressed.leads, WhereThePressLeads::ThePageOpens { .. }),
            "`Rendering` A10: the .blend is there, so the page opens: {:?}",
            pressed.leads
        );
        assert_eq!(
            pressed.the_range,
            Some(FrameForm::Triple { first: 40, last: 100, step: 1 }),
            "`Rendering` A4: the page's range is frames 40 to 100"
        );
        assert!(
            matches!(pressed.came_from.band, WhereItCameFrom::CarryingOn { the_last_frame_written: 40, the_range_now_starts_at: 40, .. }),
            "`Rendering` A4: the band says the machine was checked: {:?}",
            pressed.came_from.band
        );
        // **The one cause Smoozy names on a failed job** (`Design/Queue &
        // History`: *"where Smoozy read the machine's free disk as none, the
        // failure carries a second sentence naming the full disk and saying to
        // free space"*; `Desktop app and delivery` M14 and `Desktop app and delivery` M15a; the reading
        // is `Rented-server runtime` V12's, taken in the act that first tried
        // to write the outcome — on the full disk — and riding the pending
        // outcome unchanged to its landing).
        assert!(
            failure.what_to_do_about_it.is_some(),
            "FINDING 2 — `Desktop app and delivery` M14 and `Desktop app and delivery` M15a, `Rented-server runtime` V12: the render's outcome was \
             first written on a disk with nothing free, so its carried reading reports none and the failure names the \
             full disk and says to free space. The app showed only {:?}, with no second sentence",
            failure.said
        );
    }
}

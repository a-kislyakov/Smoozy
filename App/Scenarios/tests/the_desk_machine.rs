//! **The desk machine, proved: it stands, a job is rendered on it, it
//! reboots, and it is destroyed.** Step 12's harness piece, written so every
//! scenario test can lean on what this one shows.
//!
//! **What is real here.** OpenSSH's own server behind the harness's line; the
//! application's own transport and per-server point; the desktop's own
//! submit act; the machine's own program, its real supervisor and its real
//! render worker. **What is stood in for**: Blender, by a script named as one.
//!
//! **The frame is rendered through the machine's own program at this desk
//! too.** The program takes a render's process record (`Rented-server
//! runtime` X1) from `/proc` on Linux and, in a development build, from
//! macOS's own process table at this Mac — step 12's piece SC0b, 2026-10-24.
//! Until then the program undid the render it started here, and this test
//! ran the stand-in itself and printed that the frame was unproved.
//!
//! **Nothing of the Creator's is touched**: see the head of
//! `src/the_desk_machine.rs`.

#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use smoozy_desktop::downloading::TheRunsTransfers;
use smoozy_desktop::servers::the_machines_work::{the_queue_is_read, AsksTheSupervisor, WhatThisRunHolds};
use smoozy_desktop::submitting::{the_act_is_performed_naming_its_jobs, WhatBecameOfOneJob, WhatTheActCameTo};
use smoozy_desktop::surface::the_history::{what_is_kept, AnEnding};
use smoozy_desktop::surface::the_queue::what_is_held;
use smoozy_job_files::composing::TheReadingAndWhatFollows;
use smoozy_job_files::inventory::WhatTheInventoryIs;
use smoozy_job_files::publication::AJobsPaths;
use smoozy_runtime::package::commit::HealthAnswer;
use smoozy_scenarios::the_desk_machine::{
    a_kernel_publishes_what_a_render_start_reads, a_request, APointLending, HowTheMachineGoes,
    HowTheOldConnectionsEnd, TheDesk, TheDeskMachine, THE_DESKS_FIGURES,
};
use smoozy_server_protocol::asking_the_supervisor::WhatCameBack;
use smoozy_server_protocol::framing::{Answered, Fields};
use smoozy_server_protocol::requests as asked;
use smoozy_state::identities::{Identity, IdentityKind};

/// **One ask of the machine's supervisor through the point** — the desktop's
/// own seam, so the carriage link is planted over the file-transfer session
/// and the entry point is run through it over SSH, as on a rental.
async fn asked_through(machine: &TheDeskMachine, point: &APointLending, request: smoozy_server_protocol::framing::Request) -> Option<Answered> {
    let mut supervisor = machine.the_supervisor_over(point);
    let came_back = tokio::task::spawn_blocking(move || supervisor.ask(&request))
        .await
        .expect("the ask ran");
    match came_back {
        WhatCameBack::ItAnswered(answered) => Some(answered),
        _ => None,
    }
}

/// **`Rented-server runtime` G12's four things, read through the point.**
async fn the_health(machine: &TheDeskMachine, point: &APointLending) -> Option<HealthAnswer> {
    let answered = asked_through(machine, point, a_request(asked::THE_HEALTH_ANSWER_REQUEST)).await?;
    smoozy_runtime::acting::package::the_health_answer_in(&answered)
}

/// **The job's note, read through the point once it carries an ending** —
/// with a test's own bound, so a line that never moves fails the test rather
/// than holding it.
async fn the_note_once_it_has_ended(machine: &TheDeskMachine, point: &APointLending, job: &Identity) -> Answered {
    let began = Instant::now();
    loop {
        let note = asked_through(
            machine,
            point,
            a_request(asked::THE_PER_JOB_NOTE_READING_REQUEST)
                .carrying(Fields::none().and(asked::THE_JOB_FIELD, job.as_text())),
        )
        .await;
        if let Some(note) = note.filter(|note| note.fields.text(asked::THE_ENDING_FIELD).is_some()) {
            return note;
        }
        assert!(
            began.elapsed() < Duration::from_secs(90),
            "the job never ended on the machine: {}",
            machine.the_supervisors_complaints()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

/// Whether a file is a PNG, by its signature.
fn is_a_png(path: &Path) -> bool {
    std::fs::read(path).is_ok_and(|bytes| bytes.starts_with(b"\x89PNG\r\n\x1a\n"))
}

/// **The desk machine stands, a one-frame job is submitted by the desktop's
/// own act and rendered on it, the machine reboots under the connection and
/// is reached again, and then it is destroyed.**
///
/// - **It stands**: a connection through the line is lent by the server's own
///   point, and the supervisor answers `Rented-server runtime` G12 through
///   the planted link, naming the installation the server's row records.
/// - **A frame is rendered**: the act puts the job in the machine's line, the
///   machine's own render worker takes it, asks the stand-in its version and
///   reads the file through it, and writes the recorded reading and the
///   attempt record. The render runs, the stand-in writes the frame where
///   the program's own inventory says, and the job finishes with one file
///   present.
/// - **It reboots**: the connection lent before is a break, a fresh connection
///   reaches a supervisor of a new run under the same installation, and the
///   job is still on the machine.
/// - **It is destroyed**: the connection lent is a break, no new connection
///   opens, and nothing of the machine's runs.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_desk_machine_stands_renders_a_frame_reboots_and_is_destroyed() {
    let desk = TheDesk::opened("proof").expect("the desk's record opens");
    let Some(machine) = TheDeskMachine::stood_up("proof", &desk, "RunPod 4090").await else {
        return;
    };

    // ---- it stands
    let mut point = machine.a_point_lending().await.expect("a connection through the line is lent");
    let before = the_health(&machine, &point)
        .await
        .unwrap_or_else(|| panic!("the supervisor answers through the point: {}", machine.the_supervisors_complaints()));
    assert_eq!(&before.installation, machine.installation(), "the installation the server's row records");

    // ---- a job of one frame, by the desktop's own act
    let given = machine.what_the_page_hands_over(&desk, 1..=1, "Shot 40");
    let (sessions, record) = (point.sessions.clone(), Arc::clone(desk.record()));
    let (came_to, jobs) = tokio::task::spawn_blocking(move || {
        let the_run = TheRunsTransfers::for_this_run();
        the_act_is_performed_naming_its_jobs(given, &sessions, &the_run, record.as_ref())
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

    // ---- the render
    let note = the_note_once_it_has_ended(&machine, &point, &job).await;
    let record_directory = PathBuf::from(
        AJobsPaths::composed(machine.layout(), &job, Some("Shot 40"))
            .expect("the job's paths compose")
            .job_record_directory,
    );
    let recorded = TheReadingAndWhatFollows::read_back(
        &std::fs::read(record_directory.join(smoozy_job_files::manifest::the_recorded_readings_file_name(1)))
            .unwrap_or_else(|why| {
                panic!(
                    "the render worker read the file through the stand-in and wrote its recorded reading: \
                     {why}; the note says {:?}; {}",
                    note.fields,
                    machine.the_supervisors_complaints()
                )
            }),
    )
    .expect("the recorded reading reads back");
    let WhatTheInventoryIs::It(inventory) = &recorded.inventory else {
        panic!("the recorded reading carries an inventory: {:?}", recorded.inventory);
    };
    let the_frame = PathBuf::from(
        String::from_utf8(
            inventory
                .entries
                .iter()
                .find(|entry| entry.frame == Some(1))
                .expect("frame 1 is owed a file")
                .predicted_path
                .clone(),
        )
        .expect("a path"),
    );
    assert!(
        record_directory.join("attempt-1.json").is_file(),
        "the attempt record is written before the render starts: {:?}",
        note.fields
    );
    // **The render through the machine's program.** A program whose reader
    // takes no process record here undoes the start, so that is said first
    // rather than met as a failed job.
    assert!(
        a_kernel_publishes_what_a_render_start_reads(),
        "the program's reader takes no process record on this system ({}), so the program would \
         undo the render it starts; a development build takes one at the desk",
        std::env::consts::OS
    );
    assert_eq!(note.fields.text(asked::THE_ENDING_FIELD), Some(asked::IT_FINISHED), "{:?}", note.fields);
    assert_eq!(note.fields.text(asked::THE_FILES_PRESENT_FIELD), Some("1"), "{:?}", note.fields);
    assert!(is_a_png(&the_frame), "the frame is where the inventory says: {}", the_frame.display());
    assert_eq!(machine.the_stand_in().frames_written(), vec![1]);

    // ---- the desk reads the queue once the job has ended
    // **A `Desktop app and delivery` P10d pass's queue reading that lands
    // after the ending and before a read forward has seen it** — step 10c's
    // worldwide setup pass on Vast, new fault 5, where a finished job was in
    // neither Queue nor History for at least 12 seconds. Taken by the
    // desktop's own route, over the point: the job is in exactly one list,
    // History, with its ending.
    let server = Identity::read_back(IdentityKind::LocalServer, &machine.the_row().local_server_identifier)
        .expect("the row's own identity");
    let holds = Arc::new(WhatThisRunHolds::default());
    let connection = holds.a_connection_begins(&server.as_text());
    {
        let (mut supervisor, holds, server, record) = (
            machine.the_supervisor_over(&point),
            Arc::clone(&holds),
            server.clone(),
            Arc::clone(desk.record()),
        );
        let read = tokio::task::spawn_blocking(move || {
            the_queue_is_read(&server, connection, record.as_ref(), &mut supervisor, &holds, &|| true, false)
        })
        .await
        .expect("the queue reading ran");
        assert!(read.gave.is_some(), "the queue reading is held: {}", machine.the_supervisors_complaints());
    }
    let (queue, history) = {
        let held = desk.record().held();
        (what_is_held(&holds, Some(&*held), None), what_is_kept(Some(&*held), None))
    };
    let job_text = job.as_text();
    let in_the_queue = queue.lines.iter().flat_map(|line| line.rows.iter()).any(|row| row.job == job_text);
    let in_history = history.rows.iter().find(|row| row.job == job_text).map(|row| row.ending);
    assert_eq!(
        (in_the_queue, in_history),
        (false, Some(Some(AnEnding::Finished))),
        "`Queue & History` C11: the job that ended has left the Queue and is in History with its ending"
    );

    // ---- it reboots
    machine
        .reboots(Duration::from_secs(1), HowTheOldConnectionsEnd::WithAReset)
        .await
        .expect("the machine comes back");
    let ending = point.its_ending(Duration::from_secs(15)).await;
    assert!(
        ending.is_some_and(|ending| ending.is_a_break()),
        "the connection lent before the reboot is a break: {ending:?}"
    );
    let again = machine
        .a_point_lending()
        .await
        .expect("a fresh connection reaches the machine that came back");
    let after = the_health(&machine, &again).await.expect("the supervisor that came back answers");
    assert_ne!(after.run, before.run, "a supervisor of a new run");
    assert_eq!(after.installation, before.installation, "on the same installation");
    let held = asked_through(
        &machine,
        &again,
        a_request(asked::THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST)
            .carrying(Fields::none().and(asked::THE_JOB_FIELD, job.as_text())),
    )
    .await
    .expect("the machine answers");
    assert_eq!(
        held.fields.text(asked::ON_THIS_MACHINE_FIELD),
        Some(asked::IT_IS_PRESENT),
        "the job is still on the machine"
    );

    // ---- it is destroyed
    let mut again = again;
    machine.is_destroyed(HowTheMachineGoes::WithARefusal).await;
    let ending = again.its_ending(Duration::from_secs(15)).await;
    assert!(
        ending.is_some_and(|ending| ending.is_a_break()),
        "the connection lent is a break: {ending:?}"
    );
    let began = Instant::now();
    assert!(
        machine.a_connection(THE_DESKS_FIGURES).await.is_none(),
        "no connection opens to a destroyed machine"
    );
    assert!(began.elapsed() < Duration::from_secs(5), "and it is refused, not left to time out");
    assert!(!machine.the_supervisor_is_running(), "nothing of the machine's runs");
}

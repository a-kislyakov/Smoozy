//! **The render worker — `The shape`'s third class, for the one kind whose
//! document this program can now reach.**
//!
//! # What it is
//!
//! **`Rendering` L4: the render is started by the render worker, a child of
//! the supervisor, against one durable operation record written before the
//! worker starts.** The supervisor's side of that start is §14's one unit and
//! lives beside the dispatch in `main.rs`; this file is the child. It is this
//! program's own path with the worker word and the render kind's word after
//! it, which is what `smoozy_runtime::acting::starting::how_the_program_invokes_a_worker`
//! composes. **What it does is `Rendering`'s**: §2's nine steps, walked by
//! that crate's own driver, with the steps' own work answered here the way the
//! machine answers it.
//!
//! # How it learns what it was started for
//!
//! **It is handed nothing but its kind.** The invocation carries the worker
//! word and the kind's word and no more, because the library that composes it
//! reads exactly those two operands back and refuses anything else. **So its
//! first act is to ask the supervisor that started it**, over the machine-local
//! endpoint at `Rented-server runtime` Z2's recorded path — which is that
//! document's M6, *one channel to the supervisor*. The supervisor answers from
//! the one render it started under this run: `Rendering` K1 makes the cards
//! exclusive, so there is one render operation at a time on a machine and the
//! question has one answer. **Where the supervisor recorded the worker's own
//! process identifier at the start, the ask carries it and a mismatch is
//! refused**, so a process that is not the one started gets nothing.
//!
//! **A worker that gets no answer does nothing.** Its operation record stays
//! open and the next reconciliation takes it, which is `State, identity and
//! recovery` §6's ordinary branch for a group that is gone.
//!
//! # The steps, as the machine answers them
//!
//! `smoozy_rendering_machine::acting::pipeline::TheWorkersOwnSteps` is the
//! seam, and [`TheMachineAnswersTheWorker`] is its implementation on the
//! machine:
//!
//! - **step 2** reads the remap material off the disk (`Rendering` L30);
//! - **step 3** reads the `.blend`'s four values off the disk
//!   (`Job files and outputs` R10);
//! - **step 4** is the render-start reading, **taken as one short headless run
//!   of the Blender the manifest names** (`Job files and outputs` R1), by
//!   `smoozy_rendering_machine::acting::reading`; what it composes from it —
//!   the recorded reading and everything derived — is written into the job
//!   record directory (`State, identity and recovery` §1.2) and the supervisor
//!   is asked to journal it (`Job files and outputs` H9);
//! - **step 5** asks the supervisor to place the hold (`Rented-server runtime`
//!   U6);
//! - **step 6** walks the job output folder (`Job files and outputs` C7);
//! - **step 7** copies any uploaded bake the remap material owes into the job's
//!   folder (`Dependency Upload & Remap` M22c) and writes down what it placed
//!   (`Dependency Upload & Remap` M22f), then writes the attempt record into
//!   the job record directory,
//!   which `State, identity and recovery` §1.3 rule 1 gives the render worker
//!   to write, and asks the supervisor to record the render process's pair
//!   into the operation record (`Rendering` L5).
//!
//! **Until step 10c piece 14 step 4 was refused outright** — no script for the
//! reading existed anywhere in this workspace, and the driver took the map, the
//! inventory and the hold answer before the step that produces them — so every
//! render attempt this program ever made ended there. Piece 13 wrote the script
//! and piece 14 made the step real.
//!
//! # After the launch
//!
//! **Steps 8 and 9 are this file's own**: both streams copied into the sinks
//! the supervisor made (`Rendering` L7), the render reaped, the one walk taken
//! and the completeness statement composed as a step of the ending
//! (`Rendering` K13), the ending classified by `Rendering` K14's own function,
//! and the report re-attempted against the endpoint until it is accepted
//! (`Rented-server runtime` W12). **The worker reports and the supervisor
//! writes** — `Rented-server runtime` S1 — so nothing below writes an outcome.
//!
//! **Markers used in this file that belong to another document are written
//! with their document at every site**, as the rest of this program does.

use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::os::unix::process::ExitStatusExt;
use std::time::Duration;

use smoozy_dependencies_machine::placed_copies::{
    the_placed_copies_after_the_render_file_name, the_placed_copies_file_name, APlacedFile,
    ThePlacedCopies,
};
use smoozy_dependencies_machine::remap::RemapMaterial;
use smoozy_framing::framing::{
    Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
};
use smoozy_framing::requests as asked;
use smoozy_framing::source_reading::SourceReading;
use smoozy_job_files_machine::attempts::AttemptRecord;
use smoozy_job_files_machine::composing::TheReadingAndWhatFollows;
use smoozy_job_files_machine::inventory::WhatTheInventoryIs;
// **The recorded reading's file name is the shared crate's**, beside the
// manifest's, because the desktop reads the file this worker writes (`Download`
// N2).
use smoozy_job_files_machine::manifest::{
    the_recorded_readings_file_name, FrameRange, SubmissionManifest,
    TheReadingAndEverythingDerivedFromIt, TheRecordedReading,
};
use smoozy_job_files_machine::reckoning::{
    AWalkEntry, TheCompletenessStatement, WhatKindOfEntry, WhatTheWalkOfTheFolderIs,
};
use smoozy_queue_history_machine::acting::note::WhatTheFigureIs;
use smoozy_queue_history_machine::ending::CollectionOutcome as TheEntrysCollectionOutcome;
use smoozy_render_settings_machine::payload::Payload;
use smoozy_queue_history_machine::readings::AWorkersReport;
use smoozy_rendering_machine::acting::copy_loop::{
    with_the_markers_observation, TheMarkersObservation, TheQueuesReaderAsATee,
    TheWorkersCopyLoop, THE_WORKERS_REPORTING_INTERVAL,
};
use smoozy_rendering_machine::acting::launch::{
    TheRenderAsItWasStarted, TheRenderStartedOnTheMachine,
    TheShortInvocationOnTheMachine,
};
use smoozy_rendering_machine::acting::material::TheWorkersOwnFilesystem;
use smoozy_rendering_machine::acting::pipeline::{
    the_manifests_scene_in, the_whole_range_of, TheAttempt, TheJobOutputFolderForThisAttempt, TheRenderWorkersPipeline,
    TheWorkersOwnSteps, WhereTheAttemptEnded,
};
use smoozy_job_files_machine::manifest::WhereTheJobOutputFolderIs;
use smoozy_job_files_machine::where_the_folder_sits::{
    the_job_output_folder_in_force, MakesFolders, WhatStands, WhereTheJobOutputFolderSits,
    THE_RECORD_OF_WHERE_THE_FOLDER_SITS,
};
use smoozy_rendering_machine::acting::reading::TheReadingOnTheMachine;
use smoozy_rendering_machine::acting::the_optix_library::TheLoadersOwnLookupOnTheMachine;
use smoozy_rendering_machine::endings::{
    classify, the_outcome_against_the_count, the_outcome_of, Cause, FailedReason, RecordedOutcome,
    WhatTheWorkerReaped,
};
use smoozy_runtime::acting::capture::TheCopyIntoTheSinks;
use smoozy_runtime::acting::endpoint::TheEndpointAsACaller;
use smoozy_runtime::acting::workers::{
    IT_WAS_ACCEPTED, IT_WAS_REFUSED_AS_STALE, THE_RUN_THE_WORKER_WAS_STARTED_UNDER,
    THE_WORKERS_REPORT_REQUEST, WHETHER_THE_REPORT_WAS_ACCEPTED,
};
use smoozy_runtime::capture::{
    collection_outcome, CaptureCeiling, CaptureRecord, CollectionOutcome, CopiesAStreamIntoItsSink,
    Stream, WhatTheWorkerObserved,
};
use smoozy_runtime::execution_group::RecordedPair;
use smoozy_server_setup_machine::blender::Version;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::Clock;

// ---------------------------------------------------------------------------
// The words the worker and the supervisor speak to each other
// ---------------------------------------------------------------------------

/// **The worker's first question: what was this render worker started for.**
///
/// **Neither end is the desktop**, so the name is this program's own and is
/// not in `smoozy_framing::requests`, which names what the two ends of the
/// connection speak. **A pure reading**: it writes nothing and journals
/// nothing.
pub const THE_RENDER_WORKERS_OWN_WORK_REQUEST: &str = "what the render worker was started for";

/// **`Rendering` L5: the render process's pair, recorded into the operation
/// record at the moment it was started.** The worker holds the pair and the
/// supervisor is the one writer of the operation record (`Rented-server
/// runtime` S1), so the worker asks.
pub const THE_RENDER_PROCESS_WAS_STARTED_REQUEST: &str = "the render process was started";

/// **`Rendering` P1 step 5 and `Rented-server runtime` U6: the supervisor
/// issues the hold identity, writes the hold record and journals it.** The
/// worker asks and never composes an identity (`Rendering` P3).
pub const THE_HOLD_IS_PLACED_AT_RENDER_START_REQUEST: &str = "a hold is placed at render start";

/// The worker's own process identifier, as a decimal number.
pub const THE_WORKERS_OWN_PROCESS_FIELD: &str = "worker process";
/// The render operation's identity.
pub const THE_OPERATION_FIELD: &str = "operation";
/// **The job record directory** `Job files and outputs` N3 composes, from the
/// job record's own reference (`Queue & History` Q2).
pub const THE_RECORD_DIRECTORY_FIELD: &str = "job record directory";
/// `Rented-server runtime` M1's two sinks.
pub const THE_ORDINARY_SINK_FIELD: &str = "ordinary sink";
pub const THE_COMPLAINTS_SINK_FIELD: &str = "complaints sink";
/// **`Rented-server runtime` M4a's ceiling, recorded by value before the
/// worker started**, as the available figure it was taken from.
pub const THE_AVAILABLE_BYTES_FIELD: &str = "available bytes";
/// **Why the supervisor has no work for the asker**, where it has none.
pub const WHY_THERE_IS_NO_WORK_FIELD: &str = "why there is no work";
/// **The machine's own clock's name** — the installation identity this package
/// area holds (`State, identity and recovery` D5, §2.3). **Absent where the
/// machine holds none**, which is the machine that journals nothing.
pub const THE_MACHINES_CLOCK_FIELD: &str = "machine clock";
/// The render process's pair, as `Rented-server runtime` X1 writes it.
pub const THE_RENDER_PROCESS_FIELD: &str = "render process";
/// Whether a write the worker asked for landed: yes or no.
pub const WHETHER_IT_WAS_RECORDED_FIELD: &str = "recorded";
/// Yes.
pub const IT_WAS_RECORDED: &str = "yes";

/// **The ending as the worker reports it**, riding `Rented-server runtime`
/// W12's own report. **The outcome words are the note's**
/// (`smoozy_framing::requests`), and *held for an answer* is added here
/// because it is an attempt's ending and never a job's (`Rendering` K14).
pub const IT_WAS_HELD_FOR_AN_ANSWER: &str = "held for an answer";
/// **`Rendering` K14a's cause-side value**, on a failed ending alone.
pub const THE_CAUSE_FIELD: &str = "cause";
pub const THE_MACHINES_OWN: &str = "the machine's own";
pub const THE_FILES_OWN: &str = "the file's own";
pub const UNDETERMINED: &str = "undetermined";
/// **The reason a failed ending records, verbatim** (`Rendering` K6: no
/// number names a cause).
pub const THE_REASON_FIELD: &str = "reason";
/// **`Rented-server runtime` M13's collection outcome, per stream**, with the
/// counts and the first loss reason M6 has the worker report.
pub const THE_ORDINARY_COLLECTION_FIELD: &str = "ordinary collection";
pub const THE_COMPLAINTS_COLLECTION_FIELD: &str = "complaints collection";
pub const THE_ORDINARY_RETAINED_FIELD: &str = "ordinary retained";
pub const THE_ORDINARY_DISCARDED_FIELD: &str = "ordinary discarded";
pub const THE_ORDINARY_FIRST_LOSS_FIELD: &str = "ordinary first loss";
pub const THE_COMPLAINTS_RETAINED_FIELD: &str = "complaints retained";
pub const THE_COMPLAINTS_DISCARDED_FIELD: &str = "complaints discarded";
pub const THE_COMPLAINTS_FIRST_LOSS_FIELD: &str = "complaints first loss";
/// **`Rented-server runtime` M28's two observations**, yes or no.
pub const THE_MARKER_PASSED_FIELD: &str = "marker passed";
pub const SOMETHING_FOLLOWED_THE_MARKER_FIELD: &str = "something followed the marker";
/// **The completeness statement, as `Job files and outputs`' own written
/// form**, or [`asked::WHY_NO_COMPLETENESS_STATEMENT_FIELD`] in its place.
pub const THE_COMPLETENESS_STATEMENT_FIELD: &str = "completeness statement";
/// **`Queue & History` N6, `Queue & History` N7 and `Queue & History` N8's
/// case**, where the figure itself is
/// absent: one of the three words below.
pub const THE_FIGURE_CASE_FIELD: &str = "figure case";
pub const NO_FRAME_COUNTS_AS_ALREADY_THERE: &str = "no frame counts as already there";
pub const NO_FIGURE_COULD_BE_STATED: &str = "no figure could be stated";
pub const THIS_IS_A_VIDEO_JOB: &str = "this is a video job";

/// **How long the worker waits for a supervisor to be holding the endpoint at
/// all, when it asks what it was started for.** A Build decision and nothing
/// is sized on it: the supervisor that started this worker is the one that
/// answers, within its own turn, and a worker that finds nothing holding the
/// endpoint for this long was started by a supervisor that has gone. Five
/// seconds.
pub const HOW_LONG_THE_WORKER_LOOKS_FOR_ITS_SUPERVISOR: Duration = Duration::from_secs(5);

/// **How long one exchange with the supervisor may take.** The supervisor
/// gives one caller's request thirty seconds and answers one caller at a
/// time, so a worker queued behind a slow caller waits for that one and for
/// its own. A Build decision; one minute.
pub const HOW_LONG_ONE_EXCHANGE_MAY_TAKE: Duration = Duration::from_secs(60);

/// **The pace between two attempts at the ending's report** — `Rented-server
/// runtime` W12's re-attempt, which goes on until the report is accepted. A
/// pace and not a deadline: nothing turns on it but how soon a supervisor that
/// comes back hears the ending. One second.
pub const BETWEEN_TWO_ATTEMPTS_AT_THE_REPORT: Duration = Duration::from_secs(1);

/// **The longest completeness statement one report carries**, in its written
/// form. `smoozy_framing::framing::THE_LARGEST_FRAME` bounds a whole request,
/// and a statement longer than half of it travels through the job record
/// directory (`TheEnding::as_fields_for`) rather than a frame that would never
/// be written. A Build figure. Sitting B's repair, 2026-10-21.
pub const THE_LONGEST_STATEMENT_ONE_REPORT_CARRIES: usize =
    smoozy_framing::framing::THE_LARGEST_FRAME / 2;

/// **`Job files and outputs` H9: the recorded reading is journalled as an
/// event when it is written.**
///
/// The worker writes the reading into the job record directory
/// (`State, identity and recovery` §1.2) and the supervisor is the one writer
/// of the journal (`Rented-server runtime` S1), so the worker asks. **Neither
/// end is the desktop**, so the request's own name is this program's; the event
/// kind it makes the supervisor journal is `smoozy_framing::requests`', because
/// the desktop reads that.
pub const THE_RECORDED_READING_WAS_WRITTEN_REQUEST: &str = "a recorded reading was written";

// ---------------------------------------------------------------------------
// The worker, whole
// ---------------------------------------------------------------------------

/// **What the supervisor says this worker was started for.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWork {
    pub operation: Identity,
    pub job: Identity,
    /// `Rented-server runtime` S2's run identifier the worker was started
    /// under, which its report carries (W12).
    pub run: Identity,
    pub attempt: u32,
    pub record_directory: String,
    pub capture: CaptureRecord,
    /// **The machine's own clock** (`State, identity and recovery` D5), named
    /// by the installation identity the package area holds. **Nothing where
    /// that identity is absent**, which is the same answer the supervisor gives
    /// about journalling on such a machine; the worker refuses the attempt
    /// rather than stamping a moment on a clock it cannot name.
    pub the_machines_clock: Option<Clock>,
    /// **The machine's layout** — `Job files and outputs` L1's recorded root,
    /// and the package area this program runs from
    /// (`Job files and outputs` L4a) — read by the worker itself and never carried by the supervisor.
    /// **Only a job output folder chosen at render start reads it** (`Job
    /// files and outputs` P21); `None` where no root is recorded, and such a
    /// job is refused rather than given a folder guessed at.
    pub the_layout: Option<smoozy_job_files_machine::layout::TheLayout>,
}

/// **`Job files and outputs` L1's root, as this machine recorded it, beside
/// the package area this program runs from** — the layout the job output
/// folder chosen at render start is kept out of. Nothing where no root is
/// recorded or it is not a root.
pub fn the_layout_this_machine_recorded(package_area: &str) -> Option<smoozy_job_files_machine::layout::TheLayout> {
    use smoozy_job_files_machine::layout::{Area, TheLayout, WhereThePackageAreaSits, THE_RECORD_OF_THE_ROOT};
    let written = std::fs::read(std::path::Path::new(package_area).join(THE_RECORD_OF_THE_ROOT)).ok()?;
    // **Judged by where it really leads on this machine as well as by its text**
    // (`Job files and outputs` L3; step 10c owed piece 7).
    let root = crate::a_root_on_this_machine(String::from_utf8_lossy(&written).trim()).ok()?;
    let under_the_root = TheLayout::of(root.clone(), WhereThePackageAreaSits::UnderTheRoot);
    let package_area = package_area.trim_end_matches('/');
    if package_area == under_the_root.area(Area::Package) {
        return Some(under_the_root);
    }
    // **`Job files and outputs` L4a's second arrangement**: the package area is its location's own
    // first segment, so the location is the folder holding it.
    let location = package_area.rsplit_once('/').map(|(location, _)| location)?;
    Some(TheLayout::of(
        root,
        WhereThePackageAreaSits::ApartAt(crate::a_root_on_this_machine(location).ok()?),
    ))
}

/// **The render worker's own run.**
///
/// **`package_area` is where this program was started from**, which is where
/// `Job files and outputs` L1's root is recorded and the package area's own
/// location (`Job files and outputs` L4a) — both of which the job output folder
/// chosen at render start is kept out of (`Job files and outputs` P19).
pub fn the_render_worker(package_area: &str, recorded_endpoint_path: &str) -> Result<(), String> {
    let supervisor = TheSupervisorAsTheWorkerReachesIt::at(recorded_endpoint_path);
    let mut work = supervisor.what_this_worker_was_started_for()?;
    work.the_layout = the_layout_this_machine_recorded(package_area);
    match one_attempt(&work, &supervisor) {
        Some(ending) => supervisor.report_the_ending(&work, &ending),
        // **`Rendering` P3**: the supervisor was not answering at the moment a
        // hold would have been placed. No hold, no outcome — the attempt ends
        // as an interrupted one and reconciliation takes it.
        None => Err(
            "smoozy-server: the render worker ended without an ending: the supervisor was not \
             answering when a hold was to be placed, so the attempt is reconciliation's"
                .to_string(),
        ),
    }
}

/// **One attempt, from the manifest to the ending the worker reports.**
///
/// `None` is `Rendering` P3's case and no other.
pub fn one_attempt(
    work: &TheWork,
    supervisor: &(impl AsksTheSupervisor + Sync),
) -> Option<TheEnding> {
    let mut composed = match TheAttemptAsComposed::from_the_record_directory(work) {
        Ok(composed) => composed,
        Err(why) => {
            // **The attempt could not be composed at all** — `Rendering` K14's
            // failed limb, undetermined (`Rendering` K14a). Nothing was launched.
            eprintln!("smoozy-server: the render attempt could not be composed: {why}");
            return Some(TheEnding::before_the_launch(
                classify(
                    Some(
                        smoozy_rendering_machine::endings::BeforeTheRenderInstruction::TheAttemptCouldNotBeComposed {
                            what_happened: why.clone(),
                        },
                    ),
                    false,
                    None,
                ),
                None,
                Err(format!("the attempt could not be composed, so no walk was taken: {why}")),
                WhatTheFigureIs::TheEndingsStepNeverRan,
            ));
        }
    };

    let mut steps = TheMachineAnswersTheWorker {
        supervisor,
        work,
        the_attempt_record: None,
        folders: TheMachinesOwnFolders,
        the_blend_path: composed.manifest.blend_path_on_the_machine.as_bytes().to_vec(),
        the_moment_before_the_launch: None,
        the_crash_file_before_the_launch: None,
        the_launch: None,
    };
    let start = TheRenderStartedOnTheMachine::on_the_rented_machine();
    let asks = TheShortInvocationOnTheMachine {
        the_scripts_path: format!(
            "{}/version-ask-{}.py",
            work.record_directory, work.attempt
        ),
    };
    let went = {
        let attempt = composed.as_the_driver_takes_it(work);
        let mut pipeline = TheRenderWorkersPipeline {
            start: &start,
            places: &TheWorkersOwnFilesystem,
            asks_the_version: &asks,
            takes_the_reading: &TheReadingOnTheMachine,
            steps: &mut steps,
            // **`Rendering` L24b**: before a job naming OptiX is launched, the
            // system's own loader is asked for the OptiX library, under this
            // worker's login and environment — which are the render's.
            looks_for_the_optix_library: &TheLoadersOwnLookupOnTheMachine::on_the_rented_machine(),
        };
        pipeline.run(attempt)
    };
    // **`Job files and outputs` P21: the folder step 4 chose is the job's from
    // here on**, so the ending step walks the one the render wrote into.
    if composed.the_job_output_folder.is_none() {
        composed.the_job_output_folder = went.the_job_output_folder.clone();
    }
    let list = steps
        .the_attempt_record
        .as_ref()
        .map(|record| record.frame_list_in_the_order_it_was_written.clone())
        .unwrap_or_default();
    // **`Job files and outputs` C7: the inventory is the one for the map in
    // force**, which is what step 4 composed — never one the manifest carried
    // and never one this file made up. Where the attempt ended before step 4
    // composed any, the statement says so rather than counting against
    // nothing (`Rented-server runtime` S13, `Job files and outputs` I9a).
    let inventory = went.inventory.clone();
    let whole_range = went.the_whole_range.clone();
    let the_plan = went.the_plan.clone();

    let ending = match went.ended {
        WhereTheAttemptEnded::TheRenderWasLaunched(started) => Some(after_the_launch(
            started,
            work,
            &composed,
            inventory.as_ref(),
            &whole_range,
            &list,
            TheFiguresGo {
                the_plan,
                to: &|report| supervisor.report_the_figures(work, report),
            },
        )),
        WhereTheAttemptEnded::NoHoldWasPlacedAndReconciliationTakesIt => None,
        WhereTheAttemptEnded::AHoldWasPlaced { hold } => Some(TheEnding {
            outcome: RecordedOutcome::HeldForAnAnswer,
            hold: Some(hold),
            exit_status: None,
            streams: None,
            // **`Rendering` K14: held for an answer carries no completeness
            // reading, because it ends no job.**
            statement: None,
            figure: WhatTheFigureIs::TheEndingsStepNeverRan,
            the_device_line: None,
        }),
        other => {
            // **An attempt that ended before or at its launch may still have
            // placed copies at step 7**, and nothing will write to them now —
            // so they are read again here too (the fixes before 0.1.8, fault 3).
            the_placed_copies_are_read_again(&work.record_directory, work.attempt);
            let outcome = the_outcome_of(&other)
                .expect("every ending but a launch and P3's classifies, and both are above");
            let (statement, figure) =
                the_ending_steps_walk(&composed, inventory.as_ref(), &whole_range);
            Some(TheEnding::before_the_launch(outcome, None, statement, figure))
        }
    };
    // **`Rendering` K13's crash-file bullet**, after the completeness reading
    // both arms above have taken.
    if let Some(ending) = &ending {
        the_crash_file_at_the_ending(
            work,
            steps.the_launch.as_ref(),
            ending,
            std::time::SystemTime::now(),
            std::path::Path::new(WHERE_BLENDER_WRITES_ITS_CRASH_FILE),
        );
    }
    ending
}

// ---------------------------------------------------------------------------
// The attempt, composed from what is durable on the machine (`Rendering` L29)
// ---------------------------------------------------------------------------

/// **Every input of `Rendering`'s driver, owned**, composed from the immutable
/// manifest, the job record directory and the asset area — `Rendering` L29's
/// chain and nothing from a connection.
pub struct TheAttemptAsComposed {
    /// **The immutable manifest, read and never recomposed** (`Job files and
    /// outputs` M2).
    ///
    /// **`Rendering` L23's card make is one of its fields** (`Job files and
    /// outputs` M1b), so this shape carries no cards of its own. **Until
    /// 2026-09-20 it carried a `Cards` value this worker filled with *the
    /// cards could not be established***, the hardware reading being
    /// `Servers`' and taken on the desktop — and every cards job therefore
    /// ended at `Rendering` L23's loud failure before the render.
    pub manifest: SubmissionManifest,
    pub payload: Payload,
    pub version: Version,
    pub remap_material_path: String,
    pub launch_material_path: String,
    /// **Where `Rendering` L8b's script before the file is placed.**
    pub load_time_script_path: String,
    /// Where the reading's own script is written (`Job files and outputs` R1).
    pub reading_script_path: String,
    /// **The folder the `.blend` sits in on the machine**, as bytes — what
    /// `Job files and outputs` P4's blend-file shorthand resolves against.
    pub the_folder_the_blend_sits_in: Vec<u8>,
    pub previous_attempts: Vec<AttemptRecord>,
    /// **The machine's own clock** (`State, identity and recovery` D5), as the
    /// supervisor named it when it answered what this worker was started for.
    pub the_machines_clock: Clock,
    /// **The manifest's recorded reading and everything derived from it**,
    /// where it carries one. **Step 3 decides whether it stands**
    /// (`Job files and outputs` R12 and `Job files and outputs` R13).
    pub the_manifests_reading: Option<TheReadingAndEverythingDerivedFromIt>,
    /// **The job output folder in force** — the one the manifest names, or the
    /// one an earlier attempt chose at render start and recorded (`Job files
    /// and outputs` P21) — and `None` where this attempt's step 4 is to
    /// choose it.
    pub the_job_output_folder: Option<String>,
    /// **The layout the choice is made against**, where it is to be made.
    pub the_layout: Option<smoozy_job_files_machine::layout::TheLayout>,
}

/// **Where an earlier attempt recorded the job output folder**, read off the
/// job record directory (`Job files and outputs` P21) — nothing where none
/// stands or it does not read back.
pub fn where_the_folder_was_recorded_to_sit(record_directory: &str) -> Option<WhereTheJobOutputFolderSits> {
    let at = std::path::Path::new(record_directory).join(THE_RECORD_OF_WHERE_THE_FOLDER_SITS);
    WhereTheJobOutputFolderSits::read_back(&std::fs::read(at).ok()?)
}

/// **The job output folder in force for a job, as the machine holds it** —
/// the manifest's, or the one recorded at render start — and nothing before
/// the render's step 4 has chosen one (`Job files and outputs` P21).
pub fn the_job_output_folder_on_this_machine(
    manifest: &SubmissionManifest,
    record_directory: &str,
) -> Option<String> {
    the_job_output_folder_in_force(
        manifest.job_output_folder.the_path_it_names(),
        where_the_folder_was_recorded_to_sit(record_directory).as_ref(),
    )
}

impl TheAttemptAsComposed {
    /// **`Job files and outputs` M2: a re-run reads the manifest; it never
    /// recomposes one.**
    pub fn from_the_record_directory(work: &TheWork) -> Result<TheAttemptAsComposed, String> {
        let at = format!(
            "{}/{}",
            work.record_directory,
            smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME
        );
        let bytes = std::fs::read(&at).map_err(|why| format!("{at}: {why}"))?;
        let manifest =
            SubmissionManifest::read_back(&bytes).ok_or_else(|| format!("{at}: not a manifest"))?;
        if manifest.job != work.job.as_text() {
            return Err(format!("{at}: the manifest names another job"));
        }
        let payload = Payload::read_back(&manifest.settings_payload)
            .ok_or_else(|| format!("{at}: the settings payload does not read back"))?;
        let version = Version::parse(&manifest.blender.version)
            .ok_or_else(|| format!("{at}: the version {} is not a version", manifest.blender.version))?;
        // **`Dependency Upload & Remap` M10: the manifest carries the
        // reference to the remap material**, as that document composed it.
        let remap_material_path = manifest
            .remap_material
            .0
            .get("remap_material")
            .and_then(|at| at.as_str())
            .ok_or_else(|| format!("{at}: the remap material's reference is not a location"))?
            .to_string();
        let previous_attempts = the_attempt_records_before(&work.record_directory, work.attempt);

        // **`Job files and outputs` R7's two bullets, and step 3 decides
        // between them.** Where the file was read before submitting the
        // manifest carries the reading and the map, and **nothing is
        // recomputed** (`Job files and outputs` R9); where it was not, the
        // manifest says so and the
        // driver's step 4 takes the reading after the load.
        //
        // **Nothing else is composed here any more.** The map in force, its
        // redirect pairs, the expected-output inventory, the whole asked range
        // and the hold answer were all composed in this function until step 10c
        // piece 14 — before step 1 of a pipeline whose step 4 produces them —
        // and each of the eight faults the step 10c reading found followed from
        // that.
        let the_manifests_reading = match &manifest.recorded_reading {
            TheRecordedReading::ItWasReadBeforeSubmitting(read) => Some((**read).clone()),
            TheRecordedReading::ThereIsNone => None,
        };

        // **`State, identity and recovery` D5**: a reading taken on this
        // machine is stamped on this machine's clock, which the supervisor
        // names from the installation identity the package area holds. **A
        // machine that names no clock journals nothing either**, so it cannot
        // keep `Job files and outputs` H9 and the attempt is refused here
        // rather than rendering something nothing can be recorded about.
        let the_machines_clock = work.the_machines_clock.clone().ok_or_else(|| {
            "this machine names no clock, so no reading taken on it could be recorded; no \
             installation identity is on this machine"
                .to_string()
        })?;

        // **`Job files and outputs` P21: chosen once per job and never
        // again.** A folder the manifest names, or one an earlier attempt
        // recorded, is this attempt's; otherwise step 4 chooses, and it needs
        // the layout to keep the folder out of Smoozy's own areas.
        let the_job_output_folder =
            the_job_output_folder_on_this_machine(&manifest, &work.record_directory);
        if the_job_output_folder.is_none() && work.the_layout.is_none() {
            return Err(
                "no root is recorded on this machine, so the job output folder cannot be chosen \
                 at render start"
                    .to_string(),
            );
        }

        Ok(TheAttemptAsComposed {
            the_job_output_folder,
            the_layout: work.the_layout.clone(),
            launch_material_path: format!(
                "{}/launch-material-{}.py",
                work.record_directory, work.attempt
            ),
            // **`Rendering` L8b: the script before the file**, beside the
            // material in the job's record directory.
            load_time_script_path: format!(
                "{}/load-time-{}.py",
                work.record_directory, work.attempt
            ),
            reading_script_path: format!(
                "{}/reading-{}.py",
                work.record_directory, work.attempt
            ),
            the_folder_the_blend_sits_in: the_folder_holding(
                manifest.blend_path_on_the_machine.as_bytes(),
            )
            .to_vec(),
            manifest,
            payload,
            version,
            remap_material_path,
            previous_attempts,
            the_machines_clock,
            the_manifests_reading,
        })
    }

    /// **The job output folder as the driver takes it** — known, or to be
    /// chosen at step 4 from what the manifest carried (`Job files and
    /// outputs` P21, `Job files and outputs` P23).
    fn the_folder_for_the_driver(&self) -> TheJobOutputFolderForThisAttempt<'_> {
        if let Some(folder) = &self.the_job_output_folder {
            return TheJobOutputFolderForThisAttempt::Known(folder);
        }
        match (&self.manifest.job_output_folder, &self.the_layout) {
            (
                WhereTheJobOutputFolderIs::ChosenAtRenderStart {
                    the_folders_name,
                    the_servers_fallback_folder_at_submission,
                },
                Some(layout),
            ) => TheJobOutputFolderForThisAttempt::ToBeChosen {
                layout,
                the_folders_name,
                the_servers_fallback_folder: the_servers_fallback_folder_at_submission.as_deref(),
            },
            _ => unreachable!("composed only where a folder stands or can be chosen"),
        }
    }

    /// The driver's own shape, borrowing from this.
    fn as_the_driver_takes_it<'a>(&'a self, work: &'a TheWork) -> TheAttempt<'a> {
        TheAttempt {
            job: &work.job,
            manifest: &self.manifest,
            payload: &self.payload,
            the_manifests_version: self.version,
            the_remap_materials_path: &self.remap_material_path,
            the_blender_build_path: &self.manifest.blender.build_path,
            the_launch_materials_path: &self.launch_material_path,
            the_load_time_scripts_path: &self.load_time_script_path,
            the_readings_script_path: &self.reading_script_path,
            the_job_output_folder: self.the_folder_for_the_driver(),
            the_folder_the_blend_sits_in: &self.the_folder_the_blend_sits_in,
            previous_attempts: &self.previous_attempts,
            number: work.attempt,
            the_machines_clock: &self.the_machines_clock,
            the_manifests_reading: self.the_manifests_reading.as_ref(),
            // **`Job files and outputs` M1 carries the outputs half of a
            // reading and not `Render Settings`' settings half**, so the
            // manifest never holds a settings half for `Rendering` P1a to
            // validate
            // against and step 4 reads the file even where the map stands.
            // **That is the code being narrower than the plan** — M1 and
            // `Render Settings` Q1 both make the recorded reading one reading
            // with both halves — and it is piece 15's to close.
            the_manifests_settings_half: None,
        }
    }
}

/// **The folder a path sits in, as bytes** — everything before the last
/// separator, and the separator itself where the path has only one.
///
/// **`Job files and outputs` R4a: the path is never decoded**, so this is a
/// split over bytes rather than over text.
fn the_folder_holding(path: &[u8]) -> &[u8] {
    const SEPARATOR: u8 = b'/';
    match path.iter().rposition(|byte| *byte == SEPARATOR) {
        Some(0) => &path[..1],
        Some(at) => &path[..at],
        None => b"",
    }
}

/// **`Job files and outputs` C18a: the attempt records already written,
/// oldest first.** A record that is not there or does not read back is left
/// out, and the reckoning walks back past it to the latest that does — or
/// covers the whole range where none does (`Job files and outputs` C20).
fn the_attempt_records_before(record_directory: &str, attempt: u32) -> Vec<AttemptRecord> {
    (1..attempt)
        .filter_map(|number| {
            let at = format!("{record_directory}/attempt-{number}.json");
            AttemptRecord::read_back(&std::fs::read(at).ok()?)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The steps' own work, as the machine answers it
// ---------------------------------------------------------------------------

/// **What the worker asks of the supervisor**, behind a trait so a test can
/// stand in for the supervisor and nothing else.
pub trait AsksTheSupervisor {
    /// `Rendering` P1 step 5. `None` where the supervisor is not answering.
    fn place_a_hold(&self, work: &TheWork, reason: &str) -> Option<Identity>;
    /// `Rendering` L5.
    fn record_the_render_process(&self, work: &TheWork, pair: RecordedPair) -> Result<(), String>;
    /// **`Job files and outputs` H9**: the recorded reading is journalled as an
    /// event when it is written, and the supervisor is the one writer of the
    /// journal (`Rented-server runtime` S1).
    fn journal_the_recorded_reading(&self, work: &TheWork) -> Result<(), String>;
    /// **`Queue & History` R5: the live figures, at the worker's interval.**
    /// Nothing comes back that the worker acts on: a report that does not
    /// land is followed by the next one, and the render never waits on it.
    /// Added 2026-10-03, the Creator's notes on 0.1.8, piece 6a.
    fn report_the_figures(&self, work: &TheWork, report: AWorkersReport);
}

/// **`Rendering`'s `TheWorkersOwnSteps`, as the machine answers them.** Each
/// method hands back what it found and takes no decision.
pub struct TheMachineAnswersTheWorker<'a, S: AsksTheSupervisor> {
    pub supervisor: &'a S,
    pub work: &'a TheWork,
    /// The attempt record step 7 wrote, for the copy loop's reader.
    pub the_attempt_record: Option<AttemptRecord>,
    /// The machine's own stat and make, for the job output folder chosen at
    /// render start (`Job files and outputs` P20).
    pub folders: TheMachinesOwnFolders,
    /// **The `.blend` the render loads**, as bytes — the name Blender gives its
    /// crash file comes from it (`Reporting a problem` G5a).
    pub the_blend_path: Vec<u8>,
    /// **The moment taken as step 7 writes the attempt record**, which is
    /// before the launch — the start of the window for other loaded files.
    pub the_moment_before_the_launch: Option<std::time::SystemTime>,
    /// Read before the render starts, never after it may have written a file.
    pub the_crash_file_before_the_launch: Option<Result<Option<TheCrashFileAsRead>, String>>,
    /// **What the launch marked**, where a render was started.
    pub the_launch: Option<TheLaunchAsMarked>,
}

/// **`Job files and outputs` P20 on the machine's own disk**: a stat, and the
/// plain make — `create_dir`, never the make-parents form.
pub struct TheMachinesOwnFolders;

impl MakesFolders for TheMachinesOwnFolders {
    fn what_stands_at(&mut self, path: &str) -> WhatStands {
        match std::fs::metadata(path) {
            Ok(found) if found.is_dir() => WhatStands::AFolder,
            Ok(_) => WhatStands::SomethingElse,
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => WhatStands::Nothing,
            Err(_) => WhatStands::SomethingElse,
        }
    }
    fn make_one_folder(&mut self, path: &str) -> Result<(), ()> {
        std::fs::create_dir(path).map_err(|_| ())
    }
}

impl<S: AsksTheSupervisor> TheWorkersOwnSteps for TheMachineAnswersTheWorker<'_, S> {
    fn the_machines_folders(&mut self) -> &mut dyn MakesFolders {
        &mut self.folders
    }

    /// **`Job files and outputs` P21: written once, durably, into the job
    /// record directory** — created only where nothing stands, the bytes
    /// flushed and the folder holding the name flushed after them, as the
    /// attempt record is (`State, identity and recovery` §1.3 rule 6).
    fn write_where_the_folder_sits(&mut self, sits: &WhereTheJobOutputFolderSits) -> Result<(), String> {
        use std::io::Write;
        let at = std::path::PathBuf::from(&self.work.record_directory).join(THE_RECORD_OF_WHERE_THE_FOLDER_SITS);
        let mut file = smoozy_runtime::acting::durable::create_only_if_absent(&at)
            .map_err(|why| why.as_text())?
            .ok_or_else(|| format!("{}: where the job's folder sits is already recorded", at.display()))?;
        file.write_all(&sits.as_written())
            .and_then(|()| file.sync_all())
            .map_err(|why| format!("{}: {why}", at.display()))?;
        smoozy_runtime::acting::durable::flush_the_directory_holding(&at).map_err(|why| why.as_text())
    }

    fn read_the_remap_material(&mut self, at: &str) -> Option<RemapMaterial> {
        RemapMaterial::read_back(&std::fs::read(at).ok()?)
    }

    fn the_four_values_now(&mut self, blend_path: &str) -> Option<SourceReading> {
        let read = std::fs::metadata(blend_path).ok()?;
        Some(SourceReading {
            size: read.size(),
            inode: read.ino().to_string(),
            modification_time: read.mtime(),
            change_time: read.ctime(),
        })
    }

    /// **`State, identity and recovery` §1.2: the job record directory holds
    /// the recorded reading, the expected-output inventory, the collision
    /// record and the two picks** — written here as one document per attempt,
    /// because `Job files and outputs` R13 keeps each attempt's own rather than
    /// overwriting the last. **Then `Job files and outputs` H9's journal
    /// line**, which the supervisor writes.
    ///
    /// **A write that fails is said and the render goes on.** The plan names no
    /// ending for it — M8's attempt record is the durable thing that must
    /// precede the launch — and what is lost is the telling
    /// `Job files and outputs` H8 names rather
    /// than the render the user paid for. **It is said loudly, in the machine's
    /// own words**, so nothing about it is quiet.
    fn write_the_recorded_reading(&mut self, composed: &TheReadingAndWhatFollows) {
        use std::io::Write;
        let at = std::path::PathBuf::from(&self.work.record_directory)
            .join(the_recorded_readings_file_name(self.work.attempt));
        let written = match serde_json::to_vec(composed) {
            Ok(written) => written,
            Err(why) => {
                eprintln!(
                    "smoozy-server: the recorded reading could not be written down: {why}; the \
                     render goes on and the job's entry will not carry what the reading found"
                );
                return;
            }
        };
        let wrote = smoozy_runtime::acting::durable::create_only_if_absent(&at)
            .map_err(|why| why.as_text())
            .and_then(|standing| match standing {
                Some(mut file) => file
                    .write_all(&written)
                    .and_then(|()| file.sync_all())
                    .map_err(|why| format!("{}: {why}", at.display())),
                None => Err(format!(
                    "{}: a recorded reading already stands at this attempt",
                    at.display()
                )),
            })
            .and_then(|()| {
                smoozy_runtime::acting::durable::flush_the_directory_holding(&at)
                    .map_err(|why| why.as_text())
            });
        if let Err(why) = wrote {
            eprintln!(
                "smoozy-server: the recorded reading was not written into the job record \
                 directory: {why}; the render goes on and the job's entry will not carry what \
                 the reading found"
            );
            return;
        }
        // **`Job files and outputs` H9: journalled as an event when it is
        // written** — and only once it
        // has been, so a reader that meets the event finds the record.
        if let Err(why) = self.supervisor.journal_the_recorded_reading(self.work) {
            eprintln!(
                "smoozy-server: the recorded reading was written and its event was not \
                 journalled: {why}"
            );
        }
    }

    fn ask_the_supervisor_to_place_the_hold(&mut self, reason: &str) -> Option<Identity> {
        self.supervisor.place_a_hold(self.work, reason)
    }

    fn walk_the_job_output_folder(&mut self, at: &str) -> WhatTheWalkOfTheFolderIs {
        the_walk_of(at)
    }

    /// **`Job files and outputs` M8: written once, durably, before the render
    /// starts, and never rewritten** — so the name is created only where
    /// nothing stands at it, the bytes flushed and the folder holding the name
    /// flushed after them (`State, identity and recovery` §1.3 rule 6).
    fn write_the_attempt_record(&mut self, record: &AttemptRecord) -> Result<(), String> {
        use std::io::Write;
        self.the_moment_before_the_launch = Some(std::time::SystemTime::now());
        self.the_crash_file_before_the_launch = Some(the_crash_file_before_the_launch(
            &self.the_blend_path, std::path::Path::new(WHERE_BLENDER_WRITES_ITS_CRASH_FILE),
        ));
        let at = std::path::PathBuf::from(&self.work.record_directory).join(record.file_name());
        let mut file = smoozy_runtime::acting::durable::create_only_if_absent(&at)
            .map_err(|why| why.as_text())?
            .ok_or_else(|| {
                format!(
                    "{}: an attempt record already stands at this number and is never rewritten",
                    at.display()
                )
            })?;
        file.write_all(&record.as_written())
            .and_then(|()| file.sync_all())
            .map_err(|why| format!("{}: {why}", at.display()))?;
        smoozy_runtime::acting::durable::flush_the_directory_holding(&at)
            .map_err(|why| why.as_text())?;
        self.the_attempt_record = Some(record.clone());
        Ok(())
    }

    /// **And this attempt's mark that it has its file loaded**, written first,
    /// so a render the supervisor then refuses to record — and which is ended
    /// here — still has its window for the crash-file step.
    fn record_the_render_process(&mut self, pair: RecordedPair) -> Result<(), String> {
        self.the_launch = Some(the_launch_is_marked(
            self.work,
            &self.the_blend_path,
            self.the_moment_before_the_launch
                .unwrap_or_else(std::time::SystemTime::now),
            self.the_crash_file_before_the_launch.take().unwrap_or_else(|| {
                Err("the crash file was not read before the render started".to_string())
            }),
            pair,
            &smoozy_runtime::acting::execution_group::ThePairAsTheKernelReportsIt::the_rented_machine(),
        ));
        self.supervisor.record_the_render_process(self.work, pair)
    }

    /// **`Dependency Upload & Remap` M22c, on the machine's own disk**: see
    /// [`the_copy_on_the_machine`].
    fn copy_into_the_job_folder(
        &mut self,
        from: &[u8],
        to: &[u8],
    ) -> Result<Vec<APlacedFile>, String> {
        use std::os::unix::ffi::OsStrExt;
        the_copy_on_the_machine(
            std::path::Path::new(std::ffi::OsStr::from_bytes(from)),
            std::path::Path::new(std::ffi::OsStr::from_bytes(to)),
        )
    }

    /// **`Dependency Upload & Remap` M8b, on the machine's own disk.**
    fn make_the_opening_folder(
        &mut self,
        folder: &smoozy_dependencies_machine::opening::TheOpeningFolder,
    ) -> Result<(), String> {
        smoozy_rendering_machine::acting::opening::make_the_opening_folder(folder)
    }

    /// **`Dependency Upload & Remap` M22f's list, written into the job record
    /// directory beside the recorded reading**, one per attempt, created only
    /// where nothing stands and flushed the way the attempt record is.
    ///
    /// **A write that fails is said and the render goes on**, for the reason
    /// the driver's own seam gives: what is lost is `Download` N5a's skip, and
    /// its failure direction is the safe one.
    fn write_the_placed_copies(&mut self, placed: &ThePlacedCopies) {
        use std::io::Write;
        let at = std::path::PathBuf::from(&self.work.record_directory)
            .join(the_placed_copies_file_name(self.work.attempt));
        let wrote = smoozy_runtime::acting::durable::create_only_if_absent(&at)
            .map_err(|why| why.as_text())
            .and_then(|standing| match standing {
                Some(mut file) => file
                    .write_all(&placed.as_written())
                    .and_then(|()| file.sync_all())
                    .map_err(|why| format!("{}: {why}", at.display())),
                None => Err(format!(
                    "{}: a placed-copy list already stands at this attempt",
                    at.display()
                )),
            })
            .and_then(|()| {
                smoozy_runtime::acting::durable::flush_the_directory_holding(&at)
                    .map_err(|why| why.as_text())
            });
        if let Err(why) = wrote {
            eprintln!(
                "smoozy-server: the uploaded bake was copied into the job's folder and the list of \
                 what was placed was not written down: {why}; the render goes on, and the copy \
                 will be downloaded with the job's output"
            );
        }
    }
}

/// **`Download` N5a's *now*, read on the machine the moment the attempt's render
/// has ended** — every file this attempt placed (`Dependency Upload & Remap`
/// M22f), with its four values as they stand, written beside the list under
/// [`the_placed_copies_after_the_render_file_name`]. Added 2026-10-02, the
/// fixes before 0.1.8, fault 3.
///
/// **Why here.** `Download` N5a skips a placed copy whose four values still match those
/// read when it was made, and brings home one the render wrote over. The
/// file-transfer surface the desktop reads with carries two of the four and
/// no more (`Server protocol and transfers` S13), so nothing on the desktop
/// could ever take the comparison's *now* — and the job's download brought home
/// every copy, the user's own baked cache, gigabytes of it. **After the render
/// has ended nothing writes into the job's folder**, so a reading taken now is
/// the one the ending walk would take.
///
/// **Nothing where this attempt placed nothing.** A file that is gone is left
/// out. **A reading that cannot be written is said and the attempt goes on**:
/// what is lost is the skip, whose failure direction is the safe one — the
/// copies come home.
fn the_placed_copies_are_read_again(record_directory: &str, attempt: u32) {
    use std::io::Write;
    let directory = std::path::Path::new(record_directory);
    let Ok(bytes) = std::fs::read(directory.join(the_placed_copies_file_name(attempt))) else {
        return;
    };
    let Some(placed) = ThePlacedCopies::read_back(&bytes) else {
        return;
    };
    let now = ThePlacedCopies {
        files: placed
            .files
            .iter()
            .filter_map(|file| {
                use std::os::unix::ffi::OsStrExt;
                let at = std::path::Path::new(std::ffi::OsStr::from_bytes(
                    file.job_folder_path.as_bytes(),
                ));
                let read = std::fs::symlink_metadata(at).ok()?;
                Some(APlacedFile {
                    job_folder_path: file.job_folder_path.clone(),
                    size: read.size(),
                    inode: read.ino().to_string(),
                    modification_time: read.mtime(),
                    change_time: read.ctime(),
                })
            })
            .collect(),
    };
    let at = directory.join(the_placed_copies_after_the_render_file_name(attempt));
    let wrote = smoozy_runtime::acting::durable::create_only_if_absent(&at)
        .map_err(|why| why.as_text())
        .and_then(|standing| match standing {
            Some(mut file) => file
                .write_all(&now.as_written())
                .and_then(|()| file.sync_all())
                .map_err(|why| format!("{}: {why}", at.display())),
            None => Err(format!(
                "{}: a reading after the render already stands at this attempt",
                at.display()
            )),
        })
        .and_then(|()| {
            smoozy_runtime::acting::durable::flush_the_directory_holding(&at)
                .map_err(|why| why.as_text())
        });
    if let Err(why) = wrote {
        eprintln!(
            "smoozy-server: the copies placed in the job's folder were not read again after the \
             render: {why}; they will be downloaded with the job's output"
        );
    }
}

/// **`Dependency Upload & Remap` M22c: an uploaded bake copied from its mirror
/// in the asset area into the job's folder, on the machine** — never a second
/// crossing of the connection, and **a copy and never a link**, so a render
/// that recomputes writes over the copy and the user's upload stands untouched.
///
/// **A folder is copied whole**: each file lands beneath `to` at the path it had
/// beneath `from`, and each folder is made. **A single file lands beneath `to`
/// under its own name**, because `to` is the folder the setting now names.
/// **A link inside the mirror is refused rather than followed**, because what
/// it points at was never sent.
///
/// **Each placed file's four values are read the moment it is made**
/// (`Dependency Upload & Remap` M22f) — size, inode, modification time and
/// change time, the shape `Server protocol and transfers` S13 uses. **A file
/// that could not be copied whole is taken away again**, so no part-file is
/// left in a folder the job brings home.
pub fn the_copy_on_the_machine(
    from: &std::path::Path,
    to: &std::path::Path,
) -> Result<Vec<APlacedFile>, String> {
    let read = std::fs::symlink_metadata(from).map_err(|why| format!("{}: {why}", from.display()))?;
    let mut placed = Vec::new();
    if read.is_dir() {
        copy_a_folder(from, to, &mut placed)?;
    } else if read.is_file() {
        let name = from
            .file_name()
            .ok_or_else(|| format!("{}: names no file", from.display()))?;
        std::fs::create_dir_all(to).map_err(|why| format!("{}: {why}", to.display()))?;
        placed.push(copy_one_file(from, &to.join(the_name_it_takes(name)))?);
    } else {
        return Err(format!("{}: is neither a file nor a folder", from.display()));
    }
    Ok(placed)
}

fn copy_a_folder(
    from: &std::path::Path,
    to: &std::path::Path,
    placed: &mut Vec<APlacedFile>,
) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|why| format!("{}: {why}", to.display()))?;
    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(from)
        .map_err(|why| format!("{}: {why}", from.display()))?
        .collect::<Result<_, _>>()
        .map_err(|why| format!("{}: {why}", from.display()))?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let here = entry.path();
        let kind = entry
            .file_type()
            .map_err(|why| format!("{}: {why}", here.display()))?;
        if kind.is_dir() {
            copy_a_folder(&here, &to.join(entry.file_name()), placed)?;
        } else if kind.is_file() {
            let there = to.join(the_name_it_takes(&entry.file_name()));
            placed.push(copy_one_file(&here, &there)?);
        } else {
            return Err(format!(
                "{}: is a link or a special file, and only files and folders are copied",
                here.display()
            ));
        }
    }
    Ok(())
}

/// **`Job files and outputs` D21b-i, resolved 2026-10-02: a baked cache file
/// whose stem is longer than a cache's name can hold is copied under a stem
/// it can hold**, which the material then writes to the cache's name. Every
/// other file keeps its name. The rule is `Job files and outputs — machine`'s
/// own, so the copy and the material cannot differ.
fn the_name_it_takes(name: &std::ffi::OsStr) -> std::ffi::OsString {
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    std::ffi::OsString::from_vec(
        smoozy_job_files_machine::mapping::the_name_a_copied_cache_file_takes(name.as_bytes()),
    )
}

fn copy_one_file(from: &std::path::Path, to: &std::path::Path) -> Result<APlacedFile, String> {
    use std::os::unix::ffi::OsStrExt;
    if let Err(why) = std::fs::copy(from, to) {
        let _ = std::fs::remove_file(to);
        return Err(format!("{} to {}: {why}", from.display(), to.display()));
    }
    let now = std::fs::metadata(to).map_err(|why| format!("{}: {why}", to.display()))?;
    Ok(APlacedFile {
        job_folder_path: smoozy_dependencies_machine::bytes::MachineLocation::composed(
            to.as_os_str().as_bytes().to_vec(),
        ),
        size: now.size(),
        inode: now.ino().to_string(),
        modification_time: now.mtime(),
        change_time: now.ctime(),
    })
}

/// **`Job files and outputs` C7's one walk of the job output folder**, every
/// entry beneath it with its size, relative to the folder and as bytes.
///
/// **`Job files and outputs` C9: a walk that did not complete is a failed
/// attempt and never a folder found empty**, so any entry that could not be
/// read ends the walk as not completed. **A folder that is not there is a walk
/// that did not complete**, for the same reason — the manifest names the
/// folder and the placement made it.
pub fn the_walk_of(folder: &str) -> WhatTheWalkOfTheFolderIs {
    fn walk(
        root: &std::path::Path,
        at: &std::path::Path,
        found: &mut Vec<AWalkEntry>,
    ) -> std::io::Result<()> {
        use std::os::unix::ffi::OsStrExt;
        for entry in std::fs::read_dir(at)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            let path = entry.path();
            let under = path
                .strip_prefix(root)
                .map(|under| under.as_os_str().as_bytes().to_vec())
                .unwrap_or_default();
            if kind.is_dir() {
                found.push(AWalkEntry {
                    path_under_the_job_output_folder: under,
                    kind: WhatKindOfEntry::ADirectory,
                    size: 0,
                });
                walk(root, &path, found)?;
            } else if kind.is_file() {
                found.push(AWalkEntry {
                    path_under_the_job_output_folder: under,
                    kind: WhatKindOfEntry::AFile,
                    size: entry.metadata()?.len(),
                });
            } else {
                found.push(AWalkEntry {
                    path_under_the_job_output_folder: under,
                    kind: WhatKindOfEntry::SomethingElse,
                    size: 0,
                });
            }
        }
        Ok(())
    }
    let root = std::path::Path::new(folder);
    let mut found = Vec::new();
    match walk(root, root, &mut found) {
        Ok(()) => WhatTheWalkOfTheFolderIs::ItCompleted(found),
        Err(_) => WhatTheWalkOfTheFolderIs::ItDidNotComplete,
    }
}

/// **`Rendering` K13's ending-step reading: the one walk, the completeness
/// statement against the inventory for the map in force, and `Queue & History`
/// N6's figure from the same walk.**
///
/// **The inventory is the one step 4 composed** (`Job files and outputs` C7),
/// handed back by the driver rather than read off the manifest — *"a job whose
/// file changed is reckoned against the arrangement it is about to write into
/// and not against one it wrote into before"*.
///
/// **An ending that can carry no statement says why** (`Rented-server
/// runtime` S13): the walk did not complete, the attempt ended before step 4
/// composed an inventory at all, or the inventory was withheld with its reason
/// (`Job files and outputs` I9a).
pub fn the_ending_steps_walk(
    composed: &TheAttemptAsComposed,
    inventory: Option<&WhatTheInventoryIs>,
    whole_range: &[i64],
) -> (Result<TheCompletenessStatement, String>, WhatTheFigureIs) {
    let Some(inventory) = inventory else {
        return (
            Err("the attempt ended before a reading stood, so no inventory was composed for \
                 the map in force and there is nothing to count against"
                .to_string()),
            WhatTheFigureIs::TheEndingsStepNeverRan,
        );
    };
    let Some(folder) = composed.the_job_output_folder.as_deref() else {
        return (
            Err("the attempt ended before its job output folder was chosen, so there is no \
                 folder to walk"
                .to_string()),
            WhatTheFigureIs::TheEndingsStepNeverRan,
        );
    };
    the_one_walk_against(inventory, folder, whole_range)
}

/// **`Rented-server runtime` S11's measurement, for a job whose ending
/// reconciliation writes with no worker alive** — the same one walk and the
/// same statement, **against the inventory for the map in force** (`Job files
/// and outputs` C7), which is S11's own *"the map in force"*.
///
/// **That inventory is the attempt's own recorded reading**, the one document
/// this worker writes into the job record directory at its step 4
/// ([`TheMachineAnswersTheWorker::write_the_recorded_reading`]) — **or, where
/// the job was read before submitting and step 3 kept the manifest's map, the
/// manifest's** (`Job files and outputs` R9: then step 4 writes none). **The
/// frames are the manifest's integers, or, for *the file's own*, the scene's
/// span in that reading**, by the one function the worker's own ending takes
/// them from ([`the_whole_range_of`]).
///
/// **Written for one case and now meeting another.** This read the manifest's
/// reading alone, which was every reading there was when it was written. Since
/// the runner's ruling of 2026-09-29 the desktop writes *there is none* into
/// every manifest and the worker reads the file at step 4, **so every ending
/// reconciliation wrote said *no inventory was composed***, and History read
/// *Last frame — none finished* for a job whose frame had been saved and
/// downloaded (step 10c's pass before Vast on Sesterce, *pbv lion2*). Moved to
/// the attempt's reading on 2026-10-01, step 10c's Sesterce-pass fix piece 2.
///
/// **It is taken after the disposition is decided and moves no branch of it**
/// (S12); the caller hands it the job record directory and the attempt the
/// operation record carries, and nothing else. **Where neither reading can be
/// read, the statement says why** (S13).
pub fn the_one_walk_for_the_job_in(
    record_directory: &str,
    attempt: u32,
) -> (Result<TheCompletenessStatement, String>, WhatTheFigureIs) {
    match the_reading_in_force_for_the_job_in(record_directory, attempt) {
        Ok(in_force) => the_one_walk_against(
            &in_force.inventory,
            &in_force.job_output_folder,
            &in_force.whole_range,
        ),
        Err(why) => (Err(why), WhatTheFigureIs::TheEndingsStepNeverRan),
    }
}

/// **The reading in force for one attempt of a job, as the machine holds it**
/// — the attempt's own recorded reading where its step 4 wrote one, or the
/// manifest's where the job was read before submitting and step 3 kept its
/// map (`Job files and outputs` R9) — **with the manifest it rides beside and
/// the job's whole range of frames.** The one place that loads it: the ending
/// step's walk above, the supervisor's preview catch-up (`Queue & History`
/// P1b) and the preview worker's choosing (`Job files and outputs` B16) all
/// read it here. Moved out of [`the_one_walk_for_the_job_in`] on 2026-10-03,
/// the 38's piece 3, rather than written a second time.
pub struct TheReadingInForce {
    pub manifest: SubmissionManifest,
    /// **The job output folder in force** — the manifest's, or the one chosen
    /// at render start and recorded (`Job files and outputs` P21).
    pub job_output_folder: String,
    /// The outputs half the inventory was composed from — the file's own; the
    /// inventory read it with [`TheReadingInForce::the_jobs_pins`] applied.
    pub outputs: smoozy_job_files_machine::reading::TheOutputsHalf,
    pub inventory: WhatTheInventoryIs,
    pub picks: smoozy_job_files_machine::main_output::ThePicks,
    /// **The job's pins, as the render applies them** — the manifest
    /// payload's (`Render Settings` V4), the one source both compositions of
    /// the inventory took them from — so the preview reads the frame's view
    /// layers with the job's choices (`Job files and outputs` R4 item 5). A
    /// payload that does not read back gives none: no choices. Added
    /// 2026-10-04 by the Creator's second notes, piece 1b, on the runner's
    /// word, as a pass-through only.
    pub the_jobs_pins: Vec<smoozy_job_files_machine::reading::AViewLayerTheJobPins>,
    /// **The job's scene's View Layers, in Blender's own order** — every one
    /// the scene holds, switched on or not, from the settings half of the
    /// attempt's own recorded reading, the job's scene found by the one rule
    /// for which scene a job renders. The preview's per-frame set puts a layer
    /// the job pins on that the file switched off at its place in this order
    /// (`Job files and outputs` R4 item 5, piece P8a of step 10c). **Empty
    /// where no settings half was recorded** — a job read before submitting,
    /// whose step 4 writes none — and such a layer then goes at the end, the
    /// answer before. Added 2026-10-18, piece P8b of step 10c.
    pub the_scenes_view_layers_in_order: Vec<smoozy_job_files_machine::reading::ArtistsText>,
    /// **The job's own asked order**: the manifest's integers, or, for *the
    /// file's own*, the scene's span in that reading — nothing where neither
    /// can be read, rather than a frame guessed.
    pub whole_range: Vec<i64>,
}

/// **The job's pins as the render applies them: the manifest payload's**
/// (`Render Settings` V4), turned into `Job files and outputs`' shape by the
/// render's own conversion, so there is one source and one conversion. **Not
/// the attempt's recorded reading**: a job read before submitting keeps the
/// manifest's reading at step 3 and writes none, and the payload is what both
/// compositions took their pins from. A payload that does not read back gives
/// none — no choices, the answer before 2026-10-04 (piece 1b).
pub fn the_pins_the_render_applies(
    payload: &smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere,
) -> Vec<smoozy_job_files_machine::reading::AViewLayerTheJobPins> {
    Payload::read_back(payload)
        .map(|payload| smoozy_rendering_machine::acting::pipeline::the_jobs_pins(&payload))
        .unwrap_or_default()
}

/// [`TheReadingInForce`], read off the job record directory, **or why none
/// stands**, in the machine's own words.
pub fn the_reading_in_force_for_the_job_in(
    record_directory: &str,
    attempt: u32,
) -> Result<TheReadingInForce, String> {
    let at = format!(
        "{record_directory}/{}",
        smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME
    );
    let Some(manifest) = std::fs::read(&at)
        .ok()
        .and_then(|bytes| SubmissionManifest::read_back(&bytes))
    else {
        return Err(format!("{at}: the manifest could not be read, so there is no inventory"));
    };
    let the_attempts_own = format!(
        "{record_directory}/{}",
        the_recorded_readings_file_name(attempt)
    );
    let recorded: Option<TheReadingAndWhatFollows> = std::fs::read(&the_attempts_own)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok());
    let (outputs, inventory, picks) = match (&recorded, &manifest.recorded_reading) {
        (Some(recorded), _) => (
            recorded.reading.outputs.clone(),
            recorded.inventory.clone(),
            recorded.picks.clone(),
        ),
        (None, TheRecordedReading::ItWasReadBeforeSubmitting(read)) => {
            (read.reading.clone(), read.inventory.clone(), read.picks.clone())
        }
        (None, TheRecordedReading::ThereIsNone) => {
            return Err(format!(
                "{the_attempts_own}: attempt {attempt}'s recorded reading could not be read, \
                 and the manifest carries none, so no inventory stands for the map in force"
            ))
        }
    };
    let settings = recorded.as_ref().and_then(|recorded| {
        serde_json::from_value::<smoozy_render_settings_machine::reading::SettingsHalf>(
            recorded.reading.settings.0.clone(),
        )
        .ok()
    });
    let whole_range = match (&manifest.frame_range, &settings) {
        (_, Some(settings)) => the_whole_range_of(&manifest, settings),
        (FrameRange::TheUsersParsedIntegers(frames), None) => frames.clone(),
        // **No reading to take the file's own span from**, so no frame is
        // named rather than one guessed.
        (FrameRange::TheFilesOwn, None) => Vec::new(),
    };
    let the_jobs_pins = the_pins_the_render_applies(&manifest.settings_payload);
    let the_scenes_view_layers_in_order = settings
        .as_ref()
        .and_then(|settings| the_manifests_scene_in(settings, &manifest))
        .map(|scene| {
            scene
                .view_layers
                .iter()
                .map(|layer| smoozy_job_files_machine::reading::ArtistsText::of(layer.name.as_bytes().to_vec()))
                .collect()
        })
        .unwrap_or_default();
    let Some(job_output_folder) = the_job_output_folder_on_this_machine(&manifest, record_directory) else {
        return Err(format!(
            "{record_directory}: no job output folder was chosen for this job yet, so there is \
             nothing to count in"
        ));
    };
    Ok(TheReadingInForce {
        manifest,
        job_output_folder,
        outputs,
        inventory,
        picks,
        the_jobs_pins,
        the_scenes_view_layers_in_order,
        whole_range,
    })
}

/// The walk and the statement, against one inventory.
fn the_one_walk_against(
    inventory: &WhatTheInventoryIs,
    folder: &str,
    whole_range: &[i64],
) -> (Result<TheCompletenessStatement, String>, WhatTheFigureIs) {
    let inventory = match inventory {
        WhatTheInventoryIs::It(inventory) => inventory,
        WhatTheInventoryIs::TheFrameSetIsNotPredictable(why) => {
            return (
                Err(format!(
                    "no inventory was composed for the map in force, so there is nothing to \
                     count against: {why:?}"
                )),
                WhatTheFigureIs::TheEndingsStepNeverRan,
            )
        }
    };
    let WhatTheWalkOfTheFolderIs::ItCompleted(walk) = the_walk_of(folder) else {
        return (
            Err(format!("the walk of {folder} did not complete")),
            WhatTheFigureIs::TheEndingsStepNeverRan,
        );
    };
    let statement =
        smoozy_job_files_machine::reckoning::the_completeness_statement(inventory, &walk, folder);
    let figure = if inventory.a_destination_is_video_shaped {
        WhatTheFigureIs::ThisIsAVideoJob
    } else {
        smoozy_queue_history_machine::acting::note::the_last_frame_figure(
            whole_range,
            |frame| {
                smoozy_job_files_machine::reckoning::the_frame_is_already_there(
                    inventory, frame, &walk, folder,
                )
            },
        )
    };
    (Ok(statement), figure)
}

// ---------------------------------------------------------------------------
// Steps 8 and 9: the copy loop, the reap and the ending
// ---------------------------------------------------------------------------

/// **What one stream came to**, as `Rented-server runtime` M6 has the worker
/// report it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatOneStreamCameTo {
    pub observed: WhatTheWorkerObserved,
    pub outcome: CollectionOutcome,
}

/// **The ending the worker reports.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheEnding {
    /// `Rendering` K14 and `Rendering` K14a, classified by that crate's own function.
    pub outcome: RecordedOutcome,
    /// The hold, where the attempt ended at one.
    pub hold: Option<Identity>,
    /// **`Rented-server runtime` M22: verbatim.** A process ended by a signal
    /// has no status number, and its signal's number is written negative —
    /// Build's written form, and nothing reads a cause out of it (K6).
    pub exit_status: Option<i32>,
    /// The two streams, where a launch was made.
    pub streams: Option<(WhatOneStreamCameTo, WhatOneStreamCameTo)>,
    /// **The completeness statement, or why there is none.** `None` on an
    /// ending that carries no reading at all, which is *held for an answer*
    /// alone (`Rendering` K14).
    pub statement: Option<Result<TheCompletenessStatement, String>>,
    pub figure: WhatTheFigureIs,
    /// **`Queue & History` E13's device line, as this worker observed it at
    /// its own copy loop**, verbatim — the tee's first observed line, and
    /// absent where none passed or no launch was made. **Reported on whatever
    /// ending it passed on and never read here**: `Queue & History` E12 says
    /// what it is worth, where the entry is drawn (the paragraph after
    /// `Queue & History` N3). Until step 10c's fix piece 2 the tee observed it
    /// and the loop threw it away.
    pub the_device_line: Option<String>,
}

impl TheEnding {
    fn before_the_launch(
        outcome: RecordedOutcome,
        hold: Option<Identity>,
        statement: Result<TheCompletenessStatement, String>,
        figure: WhatTheFigureIs,
    ) -> TheEnding {
        TheEnding {
            outcome,
            hold,
            exit_status: None,
            streams: None,
            statement: Some(statement),
            figure,
            the_device_line: None,
        }
    }
}

/// **A sink the worker could not open**, which refuses every write in the
/// machine's own words — so the copy loop still drains both pipes to their
/// end (`Rented-server runtime` M5: never closing the reader) and the stream
/// comes to *collection incomplete* with the reason, rather than the render
/// being stalled or the reason lost.
struct ASinkThatCouldNotBeOpened(String);

impl CopiesAStreamIntoItsSink for ASinkThatCouldNotBeOpened {
    fn write(&self, _stream: Stream, _bytes: &[u8], _may_write: u64) -> Result<u64, String> {
        Err(self.0.clone())
    }
}

/// **Where the live figures go while the render runs** — `Queue & History`
/// R5's report, sent to the supervisor at the worker's interval
/// ([`smoozy_rendering_machine::acting::copy_loop::THE_WORKERS_REPORTING_INTERVAL`]),
/// with `Queue & History` B1's total in every one, and since piece 6b
/// `Queue & History` B6's finished part beside it, both summed from the plan
/// carried here. Added 2026-10-03, the Creator's notes on 0.1.8, piece 6a.
pub struct TheFiguresGo<'a> {
    pub the_plan: Option<Vec<smoozy_queue_history_machine::plan::Renderlet>>,
    pub to: &'a (dyn Fn(AWorkersReport) + Sync),
}

/// **Steps 8 and 9.**
pub fn after_the_launch(
    started: TheRenderAsItWasStarted,
    work: &TheWork,
    composed: &TheAttemptAsComposed,
    inventory: Option<&WhatTheInventoryIs>,
    whole_range: &[i64],
    the_attempts_list: &[i64],
    figures: TheFiguresGo<'_>,
) -> TheEnding {
    let TheRenderAsItWasStarted {
        ordinary,
        complaints,
        mut child,
        ..
    } = started;
    let observers = (
        TheMarkersObservation::default(),
        TheQueuesReaderAsATee::for_the_attempt(
            composed.version.major,
            work.attempt,
            the_attempts_list,
        )
        .carrying_the_plan(figures.the_plan)
        // **The scene the manifest names**, so the line naming a view layer is
        // split after it (`Queue & History` R10; the Creator's second notes,
        // piece 3c, on the build runner's word). An unnamed scene is none.
        .knowing_the_scene(Some(composed.manifest.scene.name.clone())),
    );
    // **`Queue & History` R5: the worker reports its current readings at its
    // own interval**, stamped on the machine's own clock (`State, identity and
    // recovery` D5). Until the Creator's notes on 0.1.8, piece 6a, the tee
    // folded every line and nothing ever sent what it held.
    let clock = composed.the_machines_clock.clone();
    let report = |held: &(TheMarkersObservation, TheQueuesReaderAsATee)| {
        held.1.the_report(std::time::SystemTime::now(), clock.clone())
    };
    let (ordinary_observed, complaints_observed, markers, the_device_line, the_ceiling_was_reached) =
        match TheCopyIntoTheSinks::opening(&work.capture) {
            Ok(copy) => the_loop(
                &copy,
                work.capture.ceiling,
                observers,
                ordinary,
                complaints,
                (&report, figures.to),
            ),
            Err(why) => the_loop(
                &ASinkThatCouldNotBeOpened(why),
                work.capture.ceiling,
                observers,
                ordinary,
                complaints,
                (&report, figures.to),
            ),
        };

    // **The ending step's own reap** (`Rendering` P1 step 9), verbatim.
    let exit_status = match child.wait() {
        Ok(status) => status.code().or_else(|| status.signal().map(|signal| -signal)),
        Err(_) => None,
    };
    // **The render has ended, so what Smoozy placed is read again now** —
    // before the ending is recorded, so the desktop that learns of the ending
    // finds the reading there. The fixes before 0.1.8, fault 3.
    the_placed_copies_are_read_again(&work.record_directory, work.attempt);

    let ordinary_observed = with_the_markers_observation(ordinary_observed, &markers);
    let complaints_observed = with_the_markers_observation(complaints_observed, &markers);
    let streams = (
        WhatOneStreamCameTo {
            outcome: collection_outcome(true, false, &ordinary_observed, the_ceiling_was_reached),
            observed: ordinary_observed,
        },
        WhatOneStreamCameTo {
            outcome: collection_outcome(true, false, &complaints_observed, the_ceiling_was_reached),
            observed: complaints_observed,
        },
    );
    let outcome = classify(
        None,
        false,
        Some(&WhatTheWorkerReaped {
            exit_status,
            the_marker_was_observed: streams.0.observed.the_marker_passed == Some(true),
            the_ordinary_streams_collection: as_the_entry_reads_it(streams.0.outcome),
        }),
    );
    let (statement, figure) = the_ending_steps_walk(composed, inventory, whole_range);
    // **`Rendering` K14b: the walk comes before the outcome is final**, because a
    // render that ended at 0 having written none of what the job owed is failed.
    // The one place it is decided: every later reader reads the word recorded
    // here, across a supervisor's restart too, and reconciliation never writes
    // *finished* (`Rented-server runtime` S11).
    let outcome = the_outcome_against_the_count(outcome, statement.as_ref().ok());
    TheEnding {
        outcome,
        hold: None,
        exit_status,
        streams: Some(streams),
        statement: Some(statement),
        figure,
        the_device_line,
    }
}

/// What the loop's readings are taken with, and where each report goes.
type TheReporting<'a> = (
    &'a (dyn Fn(&(TheMarkersObservation, TheQueuesReaderAsATee)) -> AWorkersReport + Sync),
    &'a (dyn Fn(AWorkersReport) + Sync),
);

fn the_loop<C, R1, R2>(
    copy: &C,
    ceiling: CaptureCeiling,
    observers: (TheMarkersObservation, TheQueuesReaderAsATee),
    ordinary: R1,
    complaints: R2,
    (take, send): TheReporting<'_>,
) -> (
    WhatTheWorkerObserved,
    WhatTheWorkerObserved,
    TheMarkersObservation,
    Option<String>,
    bool,
)
where
    C: CopiesAStreamIntoItsSink + Sync,
    R1: Read + Send,
    R2: Read + Send,
{
    let the_loop = TheWorkersCopyLoop::holding(copy, ceiling, observers);
    let (ordinary, complaints) = the_loop.run_reporting_every(
        THE_WORKERS_REPORTING_INTERVAL,
        ordinary,
        complaints,
        take,
        send,
    );
    let reached = the_loop
        .ceiling
        .lock()
        .map(|ceiling| ceiling.is_spent())
        .unwrap_or(false);
    // **The tee's device line is kept, verbatim** — `Queue & History` E13's
    // observation, taken at this loop and handed on with the ending
    // (the paragraph after `Queue & History` N3). **Only the line as
    // printed**: nothing here reads what it names.
    let (markers, the_device_line) = the_loop
        .observers
        .into_inner()
        .map(|(markers, tee)| {
            (
                markers,
                tee.the_device_line()
                    .map(|line| line.the_line_as_printed.clone()),
            )
        })
        .unwrap_or_default();
    (ordinary, complaints, markers, the_device_line, reached)
}

/// **`Rented-server runtime` M13's four, in `Queue & History`'s own type**,
/// which is the one `Rendering` K14 classifies against. The two types are one
/// closed set of four spelled in two crates.
fn as_the_entry_reads_it(outcome: CollectionOutcome) -> TheEntrysCollectionOutcome {
    match outcome {
        CollectionOutcome::Collected => TheEntrysCollectionOutcome::Collected,
        CollectionOutcome::CollectedAndTheStreamWasEmpty => {
            TheEntrysCollectionOutcome::CollectedAndTheStreamWasEmpty
        }
        CollectionOutcome::CollectionIncomplete => TheEntrysCollectionOutcome::CollectionIncomplete,
        CollectionOutcome::NeverEstablished => TheEntrysCollectionOutcome::NeverEstablished,
    }
}

/// **A collection outcome's own words**, for the report and for the
/// operation record's per-kind field. **Their one home is
/// `smoozy_runtime::capture::CollectionOutcome::in_words`**, which the note
/// reading's answer reads them back with (`Queue & History` E9) — moved there
/// on 2026-10-03, the Creator's notes on 0.1.8, piece 6d.
pub fn the_words_for(outcome: CollectionOutcome) -> &'static str {
    outcome.in_words()
}

// ---------------------------------------------------------------------------
// The ending, as fields — the worker writes them and the supervisor reads them
// ---------------------------------------------------------------------------

/// **The ending's own word**, in the note's words and *held for an answer*.
pub fn the_ending_word(outcome: &RecordedOutcome) -> &'static str {
    match outcome {
        RecordedOutcome::Finished => asked::IT_FINISHED,
        RecordedOutcome::Failed {
            reason: FailedReason::NoStatusWasReaped,
            ..
        } => asked::NO_STATUS_WAS_REAPED,
        RecordedOutcome::Failed { .. } => asked::IT_FAILED,
        RecordedOutcome::HeldForAnAnswer => IT_WAS_HELD_FOR_AN_ANSWER,
    }
}

/// `Rendering` K14a's value, in words.
pub fn the_cause_word(cause: Cause) -> &'static str {
    match cause {
        Cause::TheMachinesOwn => THE_MACHINES_OWN,
        Cause::TheFilesOwn => THE_FILES_OWN,
        Cause::Undetermined => UNDETERMINED,
    }
}

/// The worker and supervisor share a record directory on this machine.
/// Only the supervisor records the ending (`Rendering` P1 step 9).
pub const THE_STATEMENT_IS_IN_THE_RECORD_DIRECTORY_FIELD: &str = "the completeness statement is in the record directory";

fn the_large_statements_path(directory: &str, attempt: u32) -> std::path::PathBuf {
    std::path::Path::new(directory).join(format!("attempt-{attempt}-completeness.json"))
}

fn keep_the_large_statement(directory: &str, attempt: u32, written: &[u8]) -> Result<(), String> {
    use std::io::Write;
    let at = the_large_statements_path(directory, attempt);
    let same = || std::fs::read(&at).map_err(|why| why.to_string()).and_then(|held| {
        if held == written { Ok(()) } else { Err("this attempt already holds a different completeness statement".to_string()) }
    });
    if at.exists() {
        same()?;
        return smoozy_runtime::acting::durable::flush_the_directory_holding(&at).map_err(|why| why.as_text());
    }
    let unique = Identity::issue(IdentityKind::Operation).map_err(|_| "no identity for the statement's publication".to_string())?;
    let staged = at.with_extension(format!("{}.part", unique.as_text()));
    let kept = (|| {
        let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&staged).map_err(|why| why.to_string())?;
        file.write_all(written).and_then(|_| file.sync_all()).map_err(|why| why.to_string())?;
        // Publish whole, without ever replacing an earlier attempt's statement.
        match std::fs::hard_link(&staged, &at) {
            Ok(()) => Ok(()),
            Err(why) if why.kind() == std::io::ErrorKind::AlreadyExists => same(),
            Err(why) => Err(why.to_string()),
        }
    })();
    let _ = std::fs::remove_file(staged);
    kept?;
    smoozy_runtime::acting::durable::flush_the_directory_holding(&at).map_err(|why| why.as_text())
}

/// Read the worker's large statement from the supervisor's own job and
/// attempt. A missing or unreadable publication is retried, never counted as zero.
pub fn the_fields_with_the_recorded_statement(fields: &Fields, directory: &str, attempt: u32) -> Result<Fields, String> {
    if fields.text(THE_STATEMENT_IS_IN_THE_RECORD_DIRECTORY_FIELD) != Some("yes") {
        return Ok(fields.clone());
    }
    let written = std::fs::read(the_large_statements_path(directory, attempt)).map_err(|why| why.to_string())?;
    serde_json::from_slice::<TheCompletenessStatement>(&written).map_err(|why| why.to_string())?;
    Ok(fields.clone().and(THE_COMPLETENESS_STATEMENT_FIELD, written))
}

impl TheEnding {
    /// Keep a large statement whole beside this attempt's reading. The
    /// report names this route, never a caller-supplied path. The supervisor
    /// reads the same statement before recording the ending; no second walk.
    pub fn as_fields_for(&self, fields: Fields, directory: &str, attempt: u32) -> Result<Fields, String> {
        if let Some(Ok(statement)) = &self.statement {
            let written = serde_json::to_vec(statement).map_err(|why| why.to_string())?;
            if written.len() > THE_LONGEST_STATEMENT_ONE_REPORT_CARRIES {
                keep_the_large_statement(directory, attempt, &written)?;
                let mut without = self.clone();
                without.statement = None;
                return Ok(without.as_fields(fields).and(THE_STATEMENT_IS_IN_THE_RECORD_DIRECTORY_FIELD, "yes"));
            }
        }
        Ok(self.as_fields(fields))
    }

    /// **The report's fields.**
    pub fn as_fields(&self, mut fields: Fields) -> Fields {
        fields = fields.and(asked::THE_ENDING_FIELD, the_ending_word(&self.outcome));
        if let RecordedOutcome::Failed { reason, cause } = &self.outcome {
            // **The reason is written as a sentence, never as the type's
            // debugging rendering.** Step 10c's sixth pass on Vast read
            // `BeforeTheRenderInstruction(TheAttemptCouldNotBeComposed {
            // what_happened: "…" })` here on a real rented machine, the words
            // already inside it and the type names around them.
            fields = fields
                .and(THE_CAUSE_FIELD, the_cause_word(*cause))
                .and(THE_REASON_FIELD, reason.in_words());
            // **`Design/Queue & History`: *Failed before rendering.*** The machine
            // knows which side of the render instruction this ended on.
            if self.outcome.no_render_began() {
                fields = fields.and(asked::NO_RENDER_BEGAN_FIELD, asked::IT_IS_YES);
            }
        }
        if let Some(hold) = &self.hold {
            fields = fields.and(asked::THE_HOLD_IT_IS_HELD_UNDER_FIELD, hold.as_text());
        }
        if let Some(status) = self.exit_status {
            fields = fields.and(asked::THE_EXIT_STATUS_FIELD, status.to_string());
        }
        if let Some((ordinary, complaints)) = &self.streams {
            fields = fields
                .and(THE_ORDINARY_COLLECTION_FIELD, the_words_for(ordinary.outcome))
                .and(THE_ORDINARY_RETAINED_FIELD, ordinary.observed.retained.to_string())
                .and(THE_ORDINARY_DISCARDED_FIELD, ordinary.observed.discarded.to_string())
                .and(THE_COMPLAINTS_COLLECTION_FIELD, the_words_for(complaints.outcome))
                .and(THE_COMPLAINTS_RETAINED_FIELD, complaints.observed.retained.to_string())
                .and(
                    THE_COMPLAINTS_DISCARDED_FIELD,
                    complaints.observed.discarded.to_string(),
                )
                .and(
                    THE_MARKER_PASSED_FIELD,
                    a_yes_or_no(ordinary.observed.the_marker_passed == Some(true)),
                )
                .and(
                    SOMETHING_FOLLOWED_THE_MARKER_FIELD,
                    a_yes_or_no(ordinary.observed.something_followed_the_marker == Some(true)),
                );
            if let Some(why) = &ordinary.observed.first_loss_reason {
                fields = fields.and(THE_ORDINARY_FIRST_LOSS_FIELD, why.as_str());
            }
            if let Some(why) = &complaints.observed.first_loss_reason {
                fields = fields.and(THE_COMPLAINTS_FIRST_LOSS_FIELD, why.as_str());
            }
        }
        if let Some(line) = self.the_device_line.as_deref().filter(|line| !line.is_empty()) {
            fields = fields.and(asked::THE_DEVICE_LINE_FIELD, line);
        }
        match &self.statement {
            None => {}
            Some(Ok(statement)) => {
                let written = serde_json::to_vec(statement).unwrap_or_default();
                if written.len() > THE_LONGEST_STATEMENT_ONE_REPORT_CARRIES {
                    fields = fields.and(
                        asked::WHY_NO_COMPLETENESS_STATEMENT_FIELD,
                        format!(
                            "the statement was composed and is {} bytes long, more than one \
                             report carries",
                            written.len()
                        ),
                    );
                } else {
                    fields = fields.and(THE_COMPLETENESS_STATEMENT_FIELD, written);
                }
            }
            Some(Err(why)) => {
                fields = fields.and(asked::WHY_NO_COMPLETENESS_STATEMENT_FIELD, why.as_str());
            }
        }
        match self.figure {
            WhatTheFigureIs::It(frame) => {
                fields = fields.and(asked::THE_LAST_FRAME_FIELD, frame.to_string());
            }
            WhatTheFigureIs::NoFrameCountsAsAlreadyThere => {
                fields = fields.and(THE_FIGURE_CASE_FIELD, NO_FRAME_COUNTS_AS_ALREADY_THERE);
            }
            WhatTheFigureIs::ThisIsAVideoJob => {
                fields = fields.and(THE_FIGURE_CASE_FIELD, THIS_IS_A_VIDEO_JOB);
            }
            WhatTheFigureIs::TheEndingsStepNeverRan => {
                fields = fields.and(THE_FIGURE_CASE_FIELD, NO_FIGURE_COULD_BE_STATED);
            }
        }
        fields
    }
}

fn a_yes_or_no(it_is: bool) -> &'static str {
    if it_is {
        "yes"
    } else {
        "no"
    }
}

// ---------------------------------------------------------------------------
// Blender's crash file, at a failed ending (`Reporting a problem` G5 and
// `Reporting a problem` G5a; `Rendering` K13's crash-file bullet)
// ---------------------------------------------------------------------------
//
// **Step 11's R3b, machine half, 2026-10-24.** At every failed ending the
// worker takes, after its completeness reading, it looks for Blender's crash
// file where the research says Blender writes it, takes it only where it is
// this attempt's, and keeps a copy for the attempt; a pure reading serves what
// it kept (`Server protocol and transfers` C6, answered in `main.rs`).
//
// **Where it is kept, and why that is not beside the note.**
// `Reporting a problem` G5a says *"copied whole beside the job's per-job note
// … and shares the note's lifetime"*, and `Rendering` K13 has the worker keep
// it. **The note's store has one writer, the supervisor** (`State, identity
// and recovery` §1.3 rule 1, `Rented-server runtime` S1), and the store the
// render worker writes is the job record directory (the same rule's second
// paragraph). **Every route that removes the note removes the job record
// directory in the same cleanup** — `Queue & History` L1's (a cancel, a clear,
// a drop at the admission point) and `Queue & History` X2's erase — so a copy kept there shares the note's lifetime
// exactly, which is what the clause is for, and no second writer touches the
// note's store. Named in this piece's account for the decisions record.

/// **Where Blender writes its crash file on the machines Smoozy renders on.**
///
/// **Documented in the research, by the Method's route**: the joined answer
/// `Preparation/Blender Research/Across topics/When a render ends.md`,
/// *Whether a crash file is written*: *"in all four versions on Linux, in the
/// temporary folder under the loaded file's name"*; and the two comparison
/// questions it names, read for that question alone — *Reports 1*: *"With a
/// file loaded it printed `Writing: /tmp/simple.crash.txt`"*, and *The machine
/// 4*: *"the file went into `/tmp`, named after the loaded blend-file … It did
/// not sit beside the blend-file."* Label: **observed in a test**, on one
/// rented Linux machine. **Whether a machine whose environment names another
/// temporary folder moves it is not established**, and a pass on a rental is
/// what would show it (this piece's account).
pub const WHERE_BLENDER_WRITES_ITS_CRASH_FILE: &str = "/tmp";

/// **The ending the loaded file's name loses, and what takes its place** —
/// `Reporting a problem` G5's first bound, **observed in a test** (`Facts.md`
/// Fact 131, the Pile 3 rental): *"with `heavy.blend` open Blender wrote
/// `heavy.crash.txt`"*. **A file whose name does not end so has no crash-file
/// name on record**, and none is guessed for it.
pub const THE_LOADED_FILES_OWN_ENDING: &[u8] = b".blend";
pub const WHAT_ENDS_THE_CRASH_FILES_NAME: &[u8] = b".crash.txt";

/// **The largest crash file a copy is kept of.** A Build figure: a quarter of
/// one frame, so a kept file always travels whole in the reading's answer. The
/// two files the research holds are 1775 bytes each (Fact 131). **A larger file
/// is not taken, and the statement says so**, since `Reporting a problem` G5's
/// take is whole or not at all.
pub const THE_LARGEST_CRASH_FILE_KEPT: u64 = (smoozy_framing::framing::THE_LARGEST_FRAME / 4) as u64;

/// **The name Blender gives its crash file for the `.blend` at `blend_path`**,
/// or why it is not established. **The path is never decoded** (`Job files and
/// outputs` R4a); the name is bytes.
pub fn the_crash_files_name_for(blend_path: &[u8]) -> Result<Vec<u8>, String> {
    let name = match blend_path.iter().rposition(|byte| *byte == b'/') {
        Some(at) => &blend_path[at + 1..],
        None => blend_path,
    };
    match name.strip_suffix(THE_LOADED_FILES_OWN_ENDING) {
        Some(stem) if !stem.is_empty() => Ok([stem, WHAT_ENDS_THE_CRASH_FILES_NAME].concat()),
        _ => Err(format!(
            "the loaded file's name, {}, does not end in .blend, and the name Blender gives its \
             crash file for such a file is not on record",
            String::from_utf8_lossy(name)
        )),
    }
}

/// **This attempt's mark that it had a file loaded**, written into its own job
/// record directory at the launch and closed at the ending — so another
/// attempt on the machine can tell whether a crash file of the same name could
/// have been this one's (`Reporting a problem` G5a: *"while no other attempt on
/// the machine had a file of the same name loaded"*). The shape is Build's.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TheFileThisAttemptLoaded {
    pub job: String,
    pub attempt: u32,
    /// The name Blender would give its crash file, where it is established.
    pub the_crash_files_name: Option<Vec<u8>>,
    /// **The moment before the launch, on the machine's clock**, as a whole
    /// second and the nanoseconds after it.
    pub launched_at: (i64, u32),
    /// **The render process's pair** (`Rendering` L5), as `Rented-server
    /// runtime` X1 writes it — what tells a mark whose attempt is still
    /// running from one whose worker was ended before it could close it.
    pub render_process: String,
    /// The moment of the ending, written when the attempt's ending step runs.
    #[serde(default)]
    pub ended_at: Option<(i64, u32)>,
}

/// The mark's own file name inside the job record directory.
pub fn the_file_loaded_marks_name(attempt: u32) -> String {
    format!("attempt-{attempt}-the-file-loaded.json")
}

/// **The attempt's crash-file statement's** own file name.
pub fn the_crash_file_statements_name(attempt: u32) -> String {
    format!("attempt-{attempt}-crash-file.json")
}

/// **The kept copy's** own file name. The name Blender gave it rides the
/// statement, not this name, so a scene's name never becomes a path here.
pub fn the_kept_crash_files_name(attempt: u32) -> String {
    format!("attempt-{attempt}-crash-file.txt")
}

/// A moment as a whole second and the nanoseconds after it.
fn a_moment_as_a_pair(at: std::time::SystemTime) -> (i64, u32) {
    smoozy_runtime::written_form::the_moment_as_a_pair(at)
}

/// The file's own state, compared before and after a render. Identity also
/// distinguishes a replacement whose size and recorded times are unchanged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheCrashFileAsRead {
    device: u64,
    inode: u64,
    length: u64,
    modified: (i64, i64),
    changed: (i64, i64),
}

impl TheCrashFileAsRead {
    fn from(metadata: &std::fs::Metadata) -> Self {
        Self {
            device: metadata.dev(),
            inode: metadata.ino(),
            length: metadata.len(),
            modified: (metadata.mtime(), metadata.mtime_nsec()),
            changed: (metadata.ctime(), metadata.ctime_nsec()),
        }
    }
}

/// Read the candidate before starting the render. An unreadable earlier state
/// cannot establish that the file at the ending belongs to this attempt.
pub fn the_crash_file_before_the_launch(
    blend_path: &[u8],
    where_blender_writes: &std::path::Path,
) -> Result<Option<TheCrashFileAsRead>, String> {
    use std::os::unix::ffi::OsStrExt;
    let name = the_crash_files_name_for(blend_path)?;
    let at = where_blender_writes.join(std::ffi::OsStr::from_bytes(&name));
    match std::fs::symlink_metadata(&at) {
        Ok(metadata) => Ok(Some(TheCrashFileAsRead::from(&metadata))),
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(why) => Err(format!("{} could not be read before the render started: {why}", at.display())),
    }
}

/// **What the launch left for the ending's crash-file step**: the moment, the
/// name, this attempt's own mark, and the other attempts' marks that were
/// already over when this one launched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheLaunchAsMarked {
    pub launched_at: (i64, u32),
    pub the_crash_files_name: Result<Vec<u8>, String>,
    pub the_crash_file_before_the_launch: Result<Option<TheCrashFileAsRead>, String>,
    pub the_mark: std::path::PathBuf,
    /// **Marks still open at the launch whose render process was gone by
    /// then** — their worker ended before it closed them, so their file was
    /// not loaded while this attempt ran. Every other open mark counts as
    /// loaded.
    pub over_before_the_launch: Vec<std::path::PathBuf>,
}

/// **Every attempt's mark on this machine**, read from every job record
/// directory beside this one. A mark that does not read back is left out.
fn the_marks_on_this_machine(record_directory: &str) -> Vec<(std::path::PathBuf, TheFileThisAttemptLoaded)> {
    let Some(area) = std::path::Path::new(record_directory).parent() else {
        return Vec::new();
    };
    let mut marks = Vec::new();
    for directory in std::fs::read_dir(area).into_iter().flatten().flatten() {
        for entry in std::fs::read_dir(directory.path()).into_iter().flatten().flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !(name.starts_with("attempt-") && name.ends_with("-the-file-loaded.json")) {
                continue;
            }
            let at = entry.path();
            if let Some(mark) = std::fs::read(&at)
                .ok()
                .and_then(|bytes| serde_json::from_slice::<TheFileThisAttemptLoaded>(&bytes).ok())
            {
                marks.push((at, mark));
            }
        }
    }
    marks
}

/// **At the launch: this attempt's mark is written, and every other open mark
/// of the same name is judged by its render process.** Called with the pair
/// the moment the render was started. **A mark that could not be written is
/// said and the render goes on** — what is lost is another attempt's ability
/// to tell this one was running, and that attempt then takes a file it should
/// not only if both ran at once, which `Queue & History` Q4 does not let the
/// line do.
pub fn the_launch_is_marked(
    work: &TheWork,
    blend_path: &[u8],
    launched_at: std::time::SystemTime,
    the_crash_file_before_the_launch: Result<Option<TheCrashFileAsRead>, String>,
    pair: RecordedPair,
    kernel: &dyn smoozy_runtime::execution_group::ValidatesTheRecordedPair,
) -> TheLaunchAsMarked {
    let the_crash_files_name = the_crash_files_name_for(blend_path);
    let launched_at = a_moment_as_a_pair(launched_at);
    let the_mark = std::path::Path::new(&work.record_directory).join(the_file_loaded_marks_name(work.attempt));
    let mut over_before_the_launch = Vec::new();
    if let Ok(name) = &the_crash_files_name {
        for (at, mark) in the_marks_on_this_machine(&work.record_directory) {
            if at == the_mark || mark.ended_at.is_some() || mark.the_crash_files_name.as_ref() != Some(name) {
                continue;
            }
            let gone = RecordedPair::recorded_in(&mark.render_process).is_some_and(|recorded| {
                kernel
                    .what_is_at(recorded.identifier)
                    .is_ok_and(|there| recorded.validates_against(there).the_group_is_gone())
            });
            if gone {
                over_before_the_launch.push(at);
            }
        }
    }
    let mark = TheFileThisAttemptLoaded {
        job: work.job.as_text(),
        attempt: work.attempt,
        the_crash_files_name: the_crash_files_name.clone().ok(),
        launched_at,
        render_process: pair.as_this_document_records_it(),
        ended_at: None,
    };
    if let Err(why) = serde_json::to_vec(&mark)
        .map_err(|why| why.to_string())
        .and_then(|written| {
            smoozy_runtime::acting::durable::publish(&the_mark, &written).map_err(|why| why.as_text())
        })
    {
        eprintln!(
            "smoozy-server: the mark that this attempt has its file loaded was not written: {why}; \
             the render goes on"
        );
    }
    TheLaunchAsMarked {
        launched_at,
        the_crash_files_name,
        the_crash_file_before_the_launch,
        the_mark,
        over_before_the_launch,
    }
}

/// **At the ending: this attempt's mark is closed with the moment**, so a
/// later attempt of the same name knows the file is no longer loaded. Said
/// and not fatal where it cannot be written.
pub fn the_ending_is_marked(launch: &TheLaunchAsMarked, ended_at: std::time::SystemTime) {
    let closed = std::fs::read(&launch.the_mark)
        .map_err(|why| why.to_string())
        .and_then(|bytes| {
            serde_json::from_slice::<TheFileThisAttemptLoaded>(&bytes).map_err(|why| why.to_string())
        })
        .and_then(|mut mark| {
            mark.ended_at = Some(a_moment_as_a_pair(ended_at));
            serde_json::to_vec(&mark).map_err(|why| why.to_string())
        })
        .and_then(|written| {
            smoozy_runtime::acting::durable::publish(&launch.the_mark, &written).map_err(|why| why.as_text())
        });
    if let Err(why) = closed {
        eprintln!("smoozy-server: the mark that this attempt had its file loaded was not closed: {why}");
    }
}

/// **One attempt's crash-file statement**, written into its job record
/// directory by the ending step. The shape is Build's.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TheCrashFileStatement {
    pub attempt: u32,
    /// **The name Blender gives the file**, as bytes, where it is established
    /// — `Reporting a problem` G5's first bound has it travel with the take.
    pub the_crash_files_own_name: Option<Vec<u8>>,
    /// **The kept copy's length**, where a copy was kept.
    pub kept: Option<u64>,
    /// **Why none was kept**, in the machine's words, where none was.
    pub why_none_was_kept: Option<String>,
}

/// **`Reporting a problem` G5a's *which file is this attempt's***, judged and
/// read: the file's whole bytes, or why it is not taken.
///
/// - **Where**: `where_blender_writes`, under the name the loaded file gives.
/// - **A link is never followed, and only a plain file of this login's is
///   read** — the folder is shared by every login on the machine, and a link
///   or another login's file there is not Blender's for this attempt. The file
///   opened is held to the one judged, so nothing swapped in between is read.
/// - **When**: new or changed since the reading before the render started,
///   with a recorded write time no later than the ending.
/// - **No other attempt had a file of the same name loaded** in that time.
pub fn the_file_this_attempt_left(
    work: &TheWork,
    launch: &TheLaunchAsMarked,
    ended_at: std::time::SystemTime,
    where_blender_writes: &std::path::Path,
) -> Result<Vec<u8>, String> {
    use std::os::unix::ffi::OsStrExt;
    let name = launch.the_crash_files_name.clone()?;
    let at = where_blender_writes.join(std::ffi::OsStr::from_bytes(&name));
    let shown = at.display().to_string();
    let judged = match std::fs::symlink_metadata(&at) {
        Ok(judged) => judged,
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => {
            return Err(format!("nothing stands at {shown}"))
        }
        Err(why) => return Err(format!("{shown} could not be read: {why}")),
    };
    if judged.file_type().is_symlink() {
        return Err(format!("{shown} is a link, and a link is never followed"));
    }
    if !judged.is_file() {
        return Err(format!("{shown} is not a plain file"));
    }
    let this_login = std::fs::metadata(&work.record_directory)
        .map(|directory| directory.uid())
        .map_err(|why| format!("the job record directory could not be read: {why}"))?;
    if judged.uid() != this_login {
        return Err(format!("{shown} belongs to another user on this machine"));
    }
    let written = judged
        .modified()
        .map(a_moment_as_a_pair)
        .map_err(|why| format!("{shown}'s moment could not be read: {why}"))?;
    let ended = a_moment_as_a_pair(ended_at);
    let before = launch.the_crash_file_before_the_launch.as_ref().map_err(Clone::clone)?;
    if before.as_ref() == Some(&TheCrashFileAsRead::from(&judged)) {
        return Err(format!("{shown} is unchanged since before this attempt's launch"));
    }
    if written > ended {
        return Err(format!("{shown} was written after this attempt's ending"));
    }
    for (other, mark) in the_marks_on_this_machine(&work.record_directory) {
        if other == launch.the_mark
            || mark.the_crash_files_name.as_deref() != Some(name.as_slice())
            || mark.launched_at > ended
            || mark.ended_at.is_some_and(|over| over < launch.launched_at)
            || launch.over_before_the_launch.contains(&other)
        {
            continue;
        }
        return Err(format!(
            "another attempt on this machine had a file of the same name loaded while this one ran \
             (job {}, attempt {}), so {shown} may be that attempt's",
            mark.job, mark.attempt
        ));
    }
    if judged.len() > THE_LARGEST_CRASH_FILE_KEPT {
        return Err(format!(
            "{shown} is {} bytes, more than the {THE_LARGEST_CRASH_FILE_KEPT} a copy is kept of, and \
             it is taken whole or not at all",
            judged.len()
        ));
    }
    let mut file = std::fs::File::open(&at).map_err(|why| format!("{shown} could not be opened: {why}"))?;
    let opened = file.metadata().map_err(|why| format!("{shown} could not be read: {why}"))?;
    if TheCrashFileAsRead::from(&opened) != TheCrashFileAsRead::from(&judged) {
        return Err(format!("{shown} changed between being judged and being opened"));
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|why| format!("{shown} could not be read: {why}"))?;
    if bytes.len() as u64 != judged.len()
        || TheCrashFileAsRead::from(&file.metadata().map_err(|why| format!("{shown} could not be read: {why}"))?)
            != TheCrashFileAsRead::from(&judged)
    {
        return Err(format!("{shown} changed while it was being read"));
    }
    Ok(bytes)
}

/// **`Rendering` K13's crash-file bullet: the step itself, at a failed
/// ending, after the completeness reading.** It reads no exit number
/// (`Rendering` K6) and moves no classification (`Rendering` K14): what it
/// writes is the statement and, where the file is this attempt's, the copy.
/// **The original is left where Blender wrote it** (`Reporting a problem` G5a).
///
/// **A write that fails is said and the ending goes on** — the copy is first,
/// so a statement saying *kept* never stands without the copy it names; a
/// statement that cannot be written leaves the attempt with none, which the
/// reading answers as no statement rather than as a file.
pub fn the_crash_file_step(
    work: &TheWork,
    launch: Option<&TheLaunchAsMarked>,
    ended_at: std::time::SystemTime,
    where_blender_writes: &std::path::Path,
) -> TheCrashFileStatement {
    let directory = std::path::Path::new(&work.record_directory);
    let (name, taken) = match launch {
        None => (
            None,
            Err("no render was launched in this attempt, so no crash file can be this attempt's".to_string()),
        ),
        Some(launch) => (
            launch.the_crash_files_name.clone().ok(),
            the_file_this_attempt_left(work, launch, ended_at, where_blender_writes),
        ),
    };
    let kept = taken.and_then(|bytes| {
        smoozy_runtime::acting::durable::publish(&directory.join(the_kept_crash_files_name(work.attempt)), &bytes)
            .map(|()| bytes.len() as u64)
            .map_err(|why| format!("the crash file was found and its copy could not be kept: {}", why.as_text()))
    });
    let statement = TheCrashFileStatement {
        attempt: work.attempt,
        the_crash_files_own_name: name,
        kept: kept.as_ref().ok().copied(),
        why_none_was_kept: kept.err(),
    };
    if let Err(why) = serde_json::to_vec(&statement)
        .map_err(|why| why.to_string())
        .and_then(|written| {
            smoozy_runtime::acting::durable::publish(
                &directory.join(the_crash_file_statements_name(work.attempt)),
                &written,
            )
            .map_err(|why| why.as_text())
        })
    {
        eprintln!("smoozy-server: the crash-file statement for this attempt was not written: {why}");
    }
    statement
}

/// **The ending's last step: the launch's mark is closed at every ending a
/// launch was marked for, and the crash-file step runs at every *failed*
/// one** — `Reporting a problem` G5a's *"At every failed ending the render
/// worker takes"*, and never at a finished one or at *held for an answer*.
/// **Which failed ending it is never matters**: the step is not keyed to an
/// exit number (`Rendering` K6). Nothing where the step did not run.
pub fn the_crash_file_at_the_ending(
    work: &TheWork,
    launch: Option<&TheLaunchAsMarked>,
    ending: &TheEnding,
    ended_at: std::time::SystemTime,
    where_blender_writes: &std::path::Path,
) -> Option<TheCrashFileStatement> {
    if let Some(launch) = launch {
        the_ending_is_marked(launch, ended_at);
    }
    match ending.outcome {
        RecordedOutcome::Failed { .. } => Some(the_crash_file_step(work, launch, ended_at, where_blender_writes)),
        RecordedOutcome::Finished | RecordedOutcome::HeldForAnAnswer => None,
    }
}

/// **One attempt's kept crash file, as the reading serves it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AKeptCrashFile {
    pub attempt: u32,
    pub the_crash_files_own_name: Option<Vec<u8>>,
    pub what: WhatWasKept,
}

/// What the statement and the copy beside it come to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatWasKept {
    /// The copy, whole.
    TheFile(Vec<u8>),
    /// None was kept, and why.
    None(String),
    /// **A copy was kept and does not read back whole**, in the machine's
    /// words. Never read as none kept.
    ItDoesNotReadBack(String),
}

/// **Every crash-file statement in a job record directory, newest attempt
/// first**, each with its copy read whole — what `main.rs`'s pure reading
/// answers from. **Nothing where the step never ran**; a directory that cannot
/// be read is the machine's own words.
pub fn the_kept_crash_files_in(record_directory: &str) -> Result<Vec<AKeptCrashFile>, String> {
    let directory = std::path::Path::new(record_directory);
    let entries = match std::fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(why) => return Err(format!("{record_directory}: {why}")),
    };
    let mut statements = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let Some(number) = name
            .strip_prefix("attempt-")
            .and_then(|rest| rest.strip_suffix("-crash-file.json"))
            .and_then(|number| number.parse::<u32>().ok())
        else {
            continue;
        };
        let read = std::fs::read(entry.path())
            .map_err(|why| why.to_string())
            .and_then(|bytes| serde_json::from_slice::<TheCrashFileStatement>(&bytes).map_err(|why| why.to_string()));
        statements.push(match read {
            Ok(statement) => {
                let what = match (statement.kept, statement.why_none_was_kept) {
                    (Some(length), _) => {
                        let at = directory.join(the_kept_crash_files_name(statement.attempt));
                        match std::fs::read(&at) {
                            Ok(bytes) if bytes.len() as u64 == length => WhatWasKept::TheFile(bytes),
                            Ok(bytes) => WhatWasKept::ItDoesNotReadBack(format!(
                                "{}: {} bytes stand where {length} were kept",
                                at.display(),
                                bytes.len()
                            )),
                            Err(why) => WhatWasKept::ItDoesNotReadBack(format!("{}: {why}", at.display())),
                        }
                    }
                    (None, why) => WhatWasKept::None(why.unwrap_or_default()),
                };
                AKeptCrashFile {
                    attempt: statement.attempt,
                    the_crash_files_own_name: statement.the_crash_files_own_name,
                    what,
                }
            }
            Err(why) => AKeptCrashFile {
                attempt: number,
                the_crash_files_own_name: None,
                what: WhatWasKept::ItDoesNotReadBack(format!("the statement does not read back: {why}")),
            },
        });
    }
    statements.sort_by(|a, b| b.attempt.cmp(&a.attempt));
    Ok(statements)
}

// ---------------------------------------------------------------------------
// The one channel to the supervisor (`Rented-server runtime` M6, W12)
// ---------------------------------------------------------------------------

/// The supervisor, as this worker reaches it: the machine-local endpoint at
/// `Rented-server runtime` Z2's recorded path.
pub struct TheSupervisorAsTheWorkerReachesIt {
    endpoint: TheEndpointAsACaller,
    /// **The same endpoint, for the live figures alone**, with the interval as
    /// its bound: a report not answered by the time the next is due is worth
    /// nothing, and the reporting must never queue up behind a busy
    /// supervisor.
    figures: TheEndpointAsACaller,
}

impl TheSupervisorAsTheWorkerReachesIt {
    pub fn at(recorded_endpoint_path: &str) -> Self {
        TheSupervisorAsTheWorkerReachesIt {
            endpoint: TheEndpointAsACaller::at(recorded_endpoint_path, HOW_LONG_ONE_EXCHANGE_MAY_TAKE),
            figures: TheEndpointAsACaller::at(recorded_endpoint_path, THE_WORKERS_REPORTING_INTERVAL),
        }
    }

    fn a_request(name: &str, identity: Identity, fields: Fields) -> Request {
        Request::speaking(THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, identity, name).carrying(fields)
    }

    fn a_fresh_identity() -> Result<Identity, String> {
        Identity::issue(IdentityKind::Operation)
            .map_err(|_| "smoozy-server: the system would not supply random bytes".to_string())
    }

    /// **The first question**, asked with this process's own identifier.
    pub fn what_this_worker_was_started_for(&self) -> Result<TheWork, String> {
        let request = Self::a_request(
            THE_RENDER_WORKERS_OWN_WORK_REQUEST,
            Self::a_fresh_identity()?,
            Fields::none().and(THE_WORKERS_OWN_PROCESS_FIELD, std::process::id().to_string()),
        );
        let answered = self
            .endpoint
            .ask_waiting_for_a_supervisor(&request, HOW_LONG_THE_WORKER_LOOKS_FOR_ITS_SUPERVISOR)
            .map_err(|why| format!("smoozy-server: the render worker could not ask: {why}"))?
            .ok_or_else(|| {
                "smoozy-server: the render worker (`Rendering`'s kind) asked what it was started \
                 for and no supervisor answered, so it does nothing; its operation record is \
                 reconciliation's"
                    .to_string()
            })?;
        the_work_in(&answered)
    }

    /// **`Rented-server runtime` W12: re-attempted until it is accepted.** A
    /// supervisor that refuses it as stale has answered, and asking again
    /// would get the same answer; anything else is tried again at the pace.
    pub fn report_the_ending(&self, work: &TheWork, ending: &TheEnding) -> Result<(), String> {
        loop {
            let fields = match ending.as_fields_for(
                Fields::none().and(THE_RUN_THE_WORKER_WAS_STARTED_UNDER, work.run.as_text()),
                &work.record_directory, work.attempt,
            ) {
                Ok(fields) => fields,
                Err(why) => {
                    eprintln!("smoozy-server: the ending's statement was not published yet: {why}");
                    std::thread::sleep(BETWEEN_TWO_ATTEMPTS_AT_THE_REPORT);
                    continue;
                }
            };
            let request = Self::a_request(THE_WORKERS_REPORT_REQUEST, work.operation.clone(), fields);
            if let Ok(Some(answered)) = self.endpoint.ask(&request) {
                match answered.fields.text(WHETHER_THE_REPORT_WAS_ACCEPTED) {
                    Some(IT_WAS_ACCEPTED) => return Ok(()),
                    Some(IT_WAS_REFUSED_AS_STALE) => {
                        return Err(
                            "smoozy-server: the render worker's ending was refused as stale"
                                .to_string(),
                        )
                    }
                    _ => {
                        if let Some(why) = answered.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                        {
                            eprintln!(
                                "smoozy-server: the render worker's ending was not written yet: \
                                 {why}"
                            );
                        }
                    }
                }
            }
            std::thread::sleep(BETWEEN_TWO_ATTEMPTS_AT_THE_REPORT);
        }
    }
}

impl AsksTheSupervisor for TheSupervisorAsTheWorkerReachesIt {
    /// **`Rented-server runtime` W12's own report, carrying no ending** — so the
    /// supervisor judges it by W12's rule like any other (a report from an
    /// earlier run is refused unless *resumed* was written) and holds the
    /// figures it carries only where it is accepted. **The correlation identity
    /// is the operation's own** (`Server protocol and transfers` C1).
    fn report_the_figures(&self, work: &TheWork, report: AWorkersReport) {
        // **What came back is not read**: accepted, refused as stale or not
        // answered, the next report goes at the next interval all the same.
        let _ = self
            .figures
            .ask(&the_figures_report(&work.operation, &work.run, &report));
    }

    fn place_a_hold(&self, work: &TheWork, reason: &str) -> Option<Identity> {
        let request = Self::a_request(
            THE_HOLD_IS_PLACED_AT_RENDER_START_REQUEST,
            Self::a_fresh_identity().ok()?,
            Fields::none()
                .and(THE_OPERATION_FIELD, work.operation.as_text())
                .and(asked::THE_HOLD_REASON_FIELD, reason),
        );
        let answered = self.endpoint.ask(&request).ok()??;
        Identity::read_back(
            IdentityKind::Operation,
            answered.fields.text(asked::THE_HOLD_IT_IS_HELD_UNDER_FIELD)?,
        )
    }

    fn journal_the_recorded_reading(&self, work: &TheWork) -> Result<(), String> {
        let request = Self::a_request(
            THE_RECORDED_READING_WAS_WRITTEN_REQUEST,
            Self::a_fresh_identity()?,
            Fields::none()
                .and(THE_OPERATION_FIELD, work.operation.as_text())
                .and(asked::THE_JOB_FIELD, work.job.as_text())
                .and(asked::THE_ATTEMPT_FIELD, work.attempt.to_string()),
        );
        match self.endpoint.ask(&request)? {
            Some(answered)
                if answered.fields.text(WHETHER_IT_WAS_RECORDED_FIELD) == Some(IT_WAS_RECORDED) =>
            {
                Ok(())
            }
            Some(answered) => Err(answered
                .fields
                .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                .unwrap_or("the supervisor did not journal the recorded reading")
                .to_string()),
            None => Err("the supervisor did not answer".to_string()),
        }
    }

    fn record_the_render_process(&self, work: &TheWork, pair: RecordedPair) -> Result<(), String> {
        let request = Self::a_request(
            THE_RENDER_PROCESS_WAS_STARTED_REQUEST,
            Self::a_fresh_identity()?,
            Fields::none()
                .and(THE_OPERATION_FIELD, work.operation.as_text())
                .and(THE_RENDER_PROCESS_FIELD, pair.as_this_document_records_it()),
        );
        match self.endpoint.ask(&request)? {
            Some(answered) if answered.fields.text(WHETHER_IT_WAS_RECORDED_FIELD) == Some(IT_WAS_RECORDED) => {
                Ok(())
            }
            Some(answered) => Err(answered
                .fields
                .text(asked::THE_MACHINES_OWN_WORDS_FIELD)
                .unwrap_or("the supervisor did not record the render process")
                .to_string()),
            None => Err("the supervisor did not answer".to_string()),
        }
    }
}

/// **`Queue & History` R5's report as the worker sends it**: `Rented-server
/// runtime` W12's own request under the operation's identity, carrying the run
/// the worker was started under and the figures, and no ending.
pub fn the_figures_report(operation: &Identity, run: &Identity, report: &AWorkersReport) -> Request {
    let fields = report.as_pairs().into_iter().fold(
        Fields::none().and(THE_RUN_THE_WORKER_WAS_STARTED_UNDER, run.as_text()),
        |fields, (name, value)| fields.and(name, value),
    );
    Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        operation.clone(),
        THE_WORKERS_REPORT_REQUEST,
    )
    .carrying(fields)
}

/// **The supervisor's answer, read back**, and anything short of the whole of
/// it is no work at all.
fn the_work_in(answered: &Answered) -> Result<TheWork, String> {
    let fields = &answered.fields;
    let Some(operation) = fields.text(THE_OPERATION_FIELD) else {
        return Err(format!(
            "smoozy-server: the render worker has no work: {}",
            fields.text(WHY_THERE_IS_NO_WORK_FIELD).unwrap_or("the supervisor said nothing more")
        ));
    };
    let identity = |kind, text: Option<&str>| {
        text.and_then(|text| Identity::read_back(kind, text))
            .ok_or_else(|| "smoozy-server: the render worker's work did not read back".to_string())
    };
    let job = identity(IdentityKind::Job, fields.text(asked::THE_JOB_FIELD))?;
    let text = |name: &str| {
        fields
            .text(name)
            .map(str::to_string)
            .ok_or_else(|| format!("smoozy-server: the render worker's work carried no {name}"))
    };
    let attempt: u32 = text(asked::THE_ATTEMPT_FIELD)?
        .parse()
        .map_err(|_| "smoozy-server: the attempt is not a number".to_string())?;
    let available: u64 = text(THE_AVAILABLE_BYTES_FIELD)?
        .parse()
        .map_err(|_| "smoozy-server: the available figure is not a number".to_string())?;
    Ok(TheWork {
        operation: identity(IdentityKind::Operation, Some(operation))?,
        run: identity(IdentityKind::Run, fields.text(THE_RUN_THE_WORKER_WAS_STARTED_UNDER))?,
        attempt,
        record_directory: text(THE_RECORD_DIRECTORY_FIELD)?,
        // **A machine that names no clock answers nothing here**, and the
        // worker refuses the attempt rather than inventing a name for one.
        the_machines_clock: fields
            .text(THE_MACHINES_CLOCK_FIELD)
            .map(|name| Clock::AMachine(smoozy_state::observed::MachineClock(name.to_string()))),
        capture: CaptureRecord {
            job: job.clone(),
            attempt,
            ordinary_sink: text(THE_ORDINARY_SINK_FIELD)?,
            complaints_sink: text(THE_COMPLAINTS_SINK_FIELD)?,
            ceiling: CaptureCeiling::from(available),
        },
        job,
        the_layout: None,
    })
}

/// **The work as the supervisor answers it** — the same fields
/// [`the_work_in`] reads, composed in one place so the two cannot drift.
pub fn the_work_as_fields(work: &TheWork, available_bytes: u64) -> Fields {
    let fields = match &work.the_machines_clock {
        Some(Clock::AMachine(named)) => {
            Fields::none().and(THE_MACHINES_CLOCK_FIELD, named.0.as_str())
        }
        // **This computer's clock is never a machine's** (`State, identity and
        // recovery` D5), and a machine holding no installation identity names
        // none at all.
        Some(Clock::ThisComputer) | None => Fields::none(),
    };
    fields
        .and(THE_OPERATION_FIELD, work.operation.as_text())
        .and(asked::THE_JOB_FIELD, work.job.as_text())
        .and(THE_RUN_THE_WORKER_WAS_STARTED_UNDER, work.run.as_text())
        .and(asked::THE_ATTEMPT_FIELD, work.attempt.to_string())
        .and(THE_RECORD_DIRECTORY_FIELD, work.record_directory.as_str())
        .and(THE_ORDINARY_SINK_FIELD, work.capture.ordinary_sink.as_str())
        .and(THE_COMPLAINTS_SINK_FIELD, work.capture.complaints_sink.as_str())
        .and(THE_AVAILABLE_BYTES_FIELD, available_bytes.to_string())
}

// ---------------------------------------------------------------------------
// The render kind's own field of the operation record (`Rented-server runtime` W4)
// ---------------------------------------------------------------------------

/// **The render kind's per-kind field**, in its written form.
///
/// `Rented-server runtime` W4 makes the field the kind's document's and says it
/// is where §11.3's capture record lives for a render; M1 records both sinks'
/// paths in it before the worker starts; M6 has the supervisor write the
/// counts, the collection outcomes, the marker's observations and the exit into
/// the operation record; and `Rendering` L5 records the render process's pair
/// there. **One entry per attempt**, because `Rented-server runtime` M19 makes
/// a re-run a new attempt with its own two sinks and its own two outcomes, and
/// M18 writes each once and never again.
///
/// **The shape is Build's**, one JSON document, the form every other record on
/// the machine takes. It is written by the supervisor alone (S1).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TheRenderKindsOwnField {
    pub job: String,
    pub attempts: Vec<OneAttemptsCapture>,
}

/// One attempt's capture record, and what its ending brought.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OneAttemptsCapture {
    pub attempt: u32,
    pub ordinary_sink: String,
    pub complaints_sink: String,
    /// M4a's figure, recorded by value before the worker started.
    pub available_bytes: u64,
    /// `Rendering` L5's pair, as X1 writes it.
    #[serde(default)]
    pub render_process: Option<String>,
    /// **What the ending brought**, written once when it is reported.
    #[serde(default)]
    pub ending: Option<TheEndingAsRecorded>,
    /// **`Rented-server runtime` M17: *collection incomplete*, for both
    /// streams, written by the reconciliation that found this attempt's group
    /// gone and no ending reported.** Without it an interrupted capture would
    /// read as an empty one.
    #[serde(default)]
    pub collection_incomplete_by_reconciliation: bool,
    /// **`State, identity and recovery` §5.3 step 7's one boundary, the
    /// issuance, made durable**: whether the termination has been issued on
    /// this attempt. **Written in the act that issues it, and before the
    /// interrupt is sent** (the runner's ruling of 2026-10-01): a supervisor
    /// that ends between the two leaves *issued* with nothing sent, which a
    /// person's fresh *Stop* answers, and never *not issued* with an interrupt
    /// out, which the next run would follow with a second of its own — the
    /// quiet escalation `Rendering` K10 forbids. **A record written before step
    /// 10c's Sesterce-pass fix piece 2 reads back as not issued**, and none can
    /// carry a stop: Smoozy does not update while jobs run (Rows 518, 519).
    #[serde(default)]
    pub the_termination_was_issued: bool,
    /// **Every stop command taken on this attempt, and whether its own
    /// interrupt has gone out** — `Rendering` K10's *a repeat of one stop's
    /// identity sends nothing more, and a fresh stop sends its own*, kept
    /// across a supervisor restart; and the stops an ending closes
    /// (`State, identity and recovery` §5.3 step 6), whichever run took them.
    /// Each is written in the same act as the desired state *should be
    /// stopped* it rides with.
    #[serde(default)]
    pub the_stops_on_it: Vec<AStopOnThisAttempt>,
}

/// **One stop command on a render attempt**, as the render kind's own field
/// keeps it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AStopOnThisAttempt {
    /// The stop command's own operation identity (`Server protocol and
    /// transfers` C21).
    pub stop: String,
    /// **Whether this stop's interrupt has gone out**, written before it is
    /// sent, as the issuance is.
    pub its_interrupt_went_out: bool,
}

/// **`Rented-server runtime` M6's report, and `Rendering` K14a's cause**,
/// as the operation record keeps them.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TheEndingAsRecorded {
    pub ending: String,
    pub cause: Option<String>,
    pub reason: Option<String>,
    pub exit_status: Option<i32>,
    pub ordinary: Option<String>,
    pub complaints: Option<String>,
    pub ordinary_retained: Option<u64>,
    pub ordinary_discarded: Option<u64>,
    pub ordinary_first_loss: Option<String>,
    pub complaints_retained: Option<u64>,
    pub complaints_discarded: Option<u64>,
    pub complaints_first_loss: Option<String>,
    pub marker_passed: Option<bool>,
    pub something_followed_the_marker: Option<bool>,
    /// **`Queue & History` E13's device line, as the worker reported it**,
    /// verbatim. Absent where none passed, and on a record written before
    /// step 10c's fix piece 2, which reads back as no line observed.
    #[serde(default)]
    pub device_line: Option<String>,
    /// **Whether the attempt ended before any render began** (`Rendering` K14),
    /// as the worker reported it. No on a record written before 2026-11-04.
    #[serde(default)]
    pub no_render_began: bool,
}

impl TheEndingAsRecorded {
    /// **From the report's fields**, taking what they carry and inventing
    /// nothing they do not.
    pub fn from_the_report(fields: &Fields) -> Option<TheEndingAsRecorded> {
        let text = |name: &str| fields.text(name).map(str::to_string);
        let number = |name: &str| fields.text(name).and_then(|text| text.parse::<u64>().ok());
        let yes = |name: &str| fields.text(name).map(|text| text == "yes");
        Some(TheEndingAsRecorded {
            ending: text(asked::THE_ENDING_FIELD)?,
            cause: text(THE_CAUSE_FIELD),
            reason: text(THE_REASON_FIELD),
            exit_status: fields
                .text(asked::THE_EXIT_STATUS_FIELD)
                .and_then(|text| text.parse::<i32>().ok()),
            ordinary: text(THE_ORDINARY_COLLECTION_FIELD),
            complaints: text(THE_COMPLAINTS_COLLECTION_FIELD),
            ordinary_retained: number(THE_ORDINARY_RETAINED_FIELD),
            ordinary_discarded: number(THE_ORDINARY_DISCARDED_FIELD),
            ordinary_first_loss: text(THE_ORDINARY_FIRST_LOSS_FIELD),
            complaints_retained: number(THE_COMPLAINTS_RETAINED_FIELD),
            complaints_discarded: number(THE_COMPLAINTS_DISCARDED_FIELD),
            complaints_first_loss: text(THE_COMPLAINTS_FIRST_LOSS_FIELD),
            marker_passed: yes(THE_MARKER_PASSED_FIELD),
            something_followed_the_marker: yes(SOMETHING_FOLLOWED_THE_MARKER_FIELD),
            device_line: text(asked::THE_DEVICE_LINE_FIELD),
            no_render_began: fields.text(asked::NO_RENDER_BEGAN_FIELD) == Some(asked::IT_IS_YES),
        })
    }
}

impl TheRenderKindsOwnField {
    pub fn as_written(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("the field is composed of values that always render")
    }

    pub fn read_back(bytes: &[u8]) -> Option<TheRenderKindsOwnField> {
        serde_json::from_slice(bytes).ok()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// **The reason an attempt failed is recorded as a sentence**, and what
    /// the supervisor reads back is that sentence. Step 10c's sixth pass on
    /// Vast recorded `BeforeTheRenderInstruction(TheAttemptCouldNotBeComposed
    /// { what_happened: "…" })` on a real rented machine.
    #[test]
    fn a_failed_attempts_reason_is_recorded_in_words_and_read_back_as_them() {
        use smoozy_rendering_machine::endings::BeforeTheRenderInstruction;
        let reason = FailedReason::BeforeTheRenderInstruction(
            BeforeTheRenderInstruction::TheAttemptCouldNotBeComposed {
                what_happened: "the frame list named no frame".to_string(),
            },
        );
        let ending = TheEnding::before_the_launch(
            RecordedOutcome::Failed {
                reason: reason.clone(),
                cause: Cause::Undetermined,
            },
            None,
            Err("no inventory".to_string()),
            WhatTheFigureIs::TheEndingsStepNeverRan,
        );
        let fields = ending.as_fields(Fields::none());
        let recorded = TheEndingAsRecorded::from_the_report(&fields).expect("an ending");
        assert!(recorded.no_render_began, "`Rendering` K14: it ended before the render instruction");
        let words = recorded.reason.expect("a failed ending carries its reason");
        assert_eq!(words, reason.in_words());
        for type_name in ["BeforeTheRenderInstruction", "TheAttemptCouldNotBeComposed", "{"] {
            assert!(!words.contains(type_name), "{words}");
        }
    }

    /// **A render that began and ended badly never says no render began.**
    #[test]
    fn a_render_that_began_and_ended_badly_does_not_say_none_began() {
        for reason in [
            FailedReason::ANonZeroStatus { exit_status: 1 },
            FailedReason::NoStatusWasReaped,
            FailedReason::TheMarkerWasNeverObserved { exit_status: 1 },
            FailedReason::TheRecordedPairCouldNotBeWritten { why: "no".to_string() },
            FailedReason::NothingTheJobOwedWasWritten,
        ] {
            let ending = TheEnding::before_the_launch(
                RecordedOutcome::Failed { reason: reason.clone(), cause: Cause::Undetermined },
                None,
                Err("no inventory".to_string()),
                WhatTheFigureIs::TheEndingsStepNeverRan,
            );
            let recorded = TheEndingAsRecorded::from_the_report(&ending.as_fields(Fields::none())).expect("an ending");
            assert!(!recorded.no_render_began, "{reason:?}");
        }
    }

    /// **The preview's pins are the payload's, and a payload with none, or
    /// one that does not read back, gives none** — no choices, today's answer
    /// (piece 1b, 2026-10-04).
    #[test]
    fn the_previews_pins_are_the_payloads_and_none_reads_as_no_choices() {
        use smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere;
        use smoozy_render_settings_machine::names::{AnchorState, ExactName, Provenance};
        use smoozy_render_settings_machine::view_layers::PinEntry;
        let mut payload = smoozy_rendering_machine::material::testing::an_untouched_payload();
        assert!(the_pins_the_render_applies(&payload.as_carried()).is_empty());
        payload.view_layer_pins.push(PinEntry {
            layer: ExactName::as_the_reading_produced_it("Box AO"),
            anchor: AnchorState::ToAnchor {
                provenance: Provenance::AKeptSetup { the_old_job: "job-0".to_string() },
            },
            on: false,
        });
        let pins = the_pins_the_render_applies(&payload.as_carried());
        assert_eq!(pins.len(), 1);
        assert_eq!(pins[0].layer.as_bytes(), b"Box AO");
        assert!(!pins[0].on);
        assert!(the_pins_the_render_applies(&CarriedAndNotInterpretedHere(serde_json::json!(
            "not a payload"
        )))
        .is_empty());
    }

    /// **The work reads back as it was answered**, field for field.
    #[test]
    fn the_work_the_supervisor_answers_is_the_work_the_worker_reads() {
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let work = TheWork {
            operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
            job: job.clone(),
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            attempt: 2,
            record_directory: "/root/job-records/j".to_string(),
            capture: CaptureRecord {
                job,
                attempt: 2,
                ordinary_sink: "/area/streams/j/attempt-2/ordinary".to_string(),
                complaints_sink: "/area/streams/j/attempt-2/complaints".to_string(),
                ceiling: CaptureCeiling::from(smoozy_runtime::capture::THE_CONTROL_STATE_RESERVE + 7),
            },
            the_machines_clock: Some(Clock::AMachine(
                smoozy_state::observed::MachineClock("installation-1".to_string()),
            )),
            the_layout: None,
        };
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            THE_RENDER_WORKERS_OWN_WORK_REQUEST,
        );
        let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0")
            .carrying(the_work_as_fields(
                &work,
                smoozy_runtime::capture::THE_CONTROL_STATE_RESERVE + 7,
            ));
        assert_eq!(the_work_in(&answered), Ok(work));

        let none = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0")
            .carrying(Fields::none().and(WHY_THERE_IS_NO_WORK_FIELD, "no render was started"));
        assert!(the_work_in(&none)
            .expect_err("no work")
            .contains("no render was started"));
    }

    /// **`Dependency Upload & Remap` M22c on the disk: a single uploaded file
    /// lands under its own name in the folder the setting names, as a copy with
    /// its own four values; a link in the mirror is refused, not followed; and
    /// a mirror that is not there says so.** The folder case is proved through
    /// the real program in `tests/the_program_runs.rs`.
    #[test]
    fn a_file_is_copied_under_its_own_name_and_a_link_is_never_placed() {
        use std::os::unix::fs::MetadataExt;
        let root = std::env::temp_dir().join(format!(
            "smoozy-the-copy-{}",
            Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
        ));
        std::fs::create_dir_all(root.join("mirror")).unwrap();
        std::fs::write(root.join("mirror/cache.vdb"), b"a baked volume").unwrap();
        let to = root.join("job/bake");

        let placed = the_copy_on_the_machine(&root.join("mirror/cache.vdb"), &to)
            .expect("the copy is made");
        assert_eq!(std::fs::read(to.join("cache.vdb")).unwrap(), b"a baked volume");
        assert_eq!(placed.len(), 1);
        let now = std::fs::metadata(to.join("cache.vdb")).unwrap();
        let master = std::fs::metadata(root.join("mirror/cache.vdb")).unwrap();
        assert_eq!(placed[0].inode, now.ino().to_string());
        assert_ne!(now.ino(), master.ino(), "a copy, never a link to the master");

        std::os::unix::fs::symlink("/etc/hosts", root.join("mirror/elsewhere")).unwrap();
        let refused = the_copy_on_the_machine(&root.join("mirror"), &root.join("job/again"))
            .expect_err("a link is refused");
        assert!(refused.contains("elsewhere"), "{refused}");
        assert!(!root.join("job/again/elsewhere").exists());

        let missing = the_copy_on_the_machine(&root.join("no-such-mirror"), &to)
            .expect_err("nothing to copy");
        assert!(missing.contains("no-such-mirror"), "{missing}");
        std::fs::remove_dir_all(&root).ok();
    }

    /// **`Job files and outputs` D21b-i, resolved 2026-10-02: a baked cache
    /// file whose stem is longer than a cache's name can hold is copied under
    /// a stem it can hold**, and its placed record names the new path; every
    /// other file keeps its name. At the desk a cloth on an object named
    /// `Занавеска_левая_окна` was copied under its 80-character stem and never
    /// read; under the shorter stem all 24 frames matched the plain render.
    #[test]
    fn a_cache_file_whose_stem_a_name_cannot_hold_is_copied_under_one_it_can() {
        use std::os::unix::ffi::OsStrExt;
        let root = std::env::temp_dir().join(format!(
            "smoozy-the-long-stem-{}",
            Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
        ));
        let long: String = "Занавеска_левая_окна"
            .bytes()
            .map(|byte| format!("{byte:02X}"))
            .collect();
        let short = String::from_utf8(
            smoozy_job_files_machine::mapping::the_stem_a_cache_name_can_hold(long.as_bytes()),
        )
        .unwrap();
        std::fs::create_dir_all(root.join("mirror")).unwrap();
        std::fs::write(root.join(format!("mirror/{long}_000001_00.bphys")), b"frame one").unwrap();
        std::fs::write(root.join("mirror/436C6F746841_000001_00.bphys"), b"a short one").unwrap();
        let to = root.join("job/pointcache");
        let placed = the_copy_on_the_machine(&root.join("mirror"), &to).expect("the copy is made");
        assert_eq!(
            std::fs::read(to.join(format!("{short}_000001_00.bphys"))).unwrap(),
            b"frame one"
        );
        assert!(!to.join(format!("{long}_000001_00.bphys")).exists());
        assert_eq!(std::fs::read(to.join("436C6F746841_000001_00.bphys")).unwrap(), b"a short one");
        assert!(placed.iter().any(|one| one.job_folder_path.as_bytes()
            == to.join(format!("{short}_000001_00.bphys")).as_os_str().as_bytes()));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **`Job files and outputs` C7's walk, and `Job files and outputs` C9's rule that a folder not
    /// there is a walk that did not complete.**
    /// **`Job files and outputs` P21 on the machine: the job output folder in
    /// force is the manifest's where it names one, the one recorded at render
    /// start where it does not, and nothing before a render chose it** — and
    /// **the layout the choice is kept out of is the recorded root beside the
    /// package area this program runs from** (`Job files and outputs` L4a's two
    /// arrangements). The Creator's second notes, piece 4.
    #[test]
    fn the_folder_in_force_is_the_manifests_or_the_one_recorded_at_render_start() {
        use smoozy_job_files_machine::layout::{Area, WhereThePackageAreaSits};
        // **The build folder's own temporary area, not the system's**: a root
        // is recorded under this folder below, `Job files and outputs` L3
        // refuses a root under `/tmp`, and on Linux the system's temporary
        // folder is `/tmp` — hosted run 37519118385 is where this was found.
        // A unit test has no `CARGO_TARGET_TMPDIR`; three folders up from this
        // test program is the build folder, as in `the_setup_worker`'s
        // `the_build_folders_own_temporary_area`.
        let at = std::env::current_exe()
            .expect("this test knows where it is")
            .ancestors()
            .nth(3)
            .expect("a test program sits three folders inside the build folder")
            .join("tmp")
            .join(format!(
                "smoozy-the-folder-in-force-{}",
                Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
            ));
        std::fs::create_dir_all(&at).unwrap();
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let mut manifest = smoozy_rendering_machine::composition::testing::a_manifest(&job, "5.2", "Scene");
        let directory = at.display().to_string();
        let named = manifest.job_output_folder.the_path_it_names().unwrap().to_string();
        assert_eq!(the_job_output_folder_on_this_machine(&manifest, &directory), Some(named));
        manifest.job_output_folder = WhereTheJobOutputFolderIs::ChosenAtRenderStart {
            the_folders_name: "job-Kitchen".into(),
            the_servers_fallback_folder_at_submission: None,
        };
        assert_eq!(the_job_output_folder_on_this_machine(&manifest, &directory), None, "nothing chosen yet");
        let sits = WhereTheJobOutputFolderSits {
            path: "/home/ubuntu/Render/job-Kitchen".into(),
            where_it_is: smoozy_job_files_machine::where_the_folder_sits::WhereItIs::WhereTheFilePoints,
        };
        std::fs::write(at.join(THE_RECORD_OF_WHERE_THE_FOLDER_SITS), sits.as_written()).unwrap();
        assert_eq!(
            the_job_output_folder_on_this_machine(&manifest, &directory).as_deref(),
            Some("/home/ubuntu/Render/job-Kitchen")
        );

        // **The layout**: the root recorded in the package area, and the
        // package area under it or apart.
        let root = at.join("ephemeral/smoozy");
        let under = root.join("package");
        std::fs::create_dir_all(&under).unwrap();
        std::fs::write(under.join(smoozy_job_files_machine::layout::THE_RECORD_OF_THE_ROOT), root.display().to_string()).unwrap();
        let layout = the_layout_this_machine_recorded(&under.display().to_string()).expect("a layout");
        assert_eq!(layout.where_the_package_area_sits(), &WhereThePackageAreaSits::UnderTheRoot);
        assert_eq!(layout.area(Area::Package), under.display().to_string());
        let apart = at.join("data/package");
        std::fs::create_dir_all(&apart).unwrap();
        std::fs::write(apart.join(smoozy_job_files_machine::layout::THE_RECORD_OF_THE_ROOT), root.display().to_string()).unwrap();
        let layout = the_layout_this_machine_recorded(&apart.display().to_string()).expect("a layout");
        assert_eq!(layout.area(Area::Package), apart.display().to_string());
        assert!(matches!(layout.where_the_package_area_sits(), WhereThePackageAreaSits::ApartAt(_)));
        assert!(the_layout_this_machine_recorded(&at.join("nowhere").display().to_string()).is_none());
        std::fs::remove_dir_all(&at).ok();
    }

    /// **`Job files and outputs` L3, judged by where the recorded root really
    /// leads on this machine** (step 10c owed piece 7). A root
    /// written through a link into an area wiped at every boot gives no
    /// layout, so no job is given a folder under it; a root through a link to
    /// a good place gives the layout, with the root as written.
    #[cfg(unix)]
    #[test]
    fn a_recorded_root_that_leads_into_a_shared_temporary_area_gives_no_layout() {
        use smoozy_job_files_machine::layout::THE_RECORD_OF_THE_ROOT;
        // **Beside this test's own program, in the build folder**, which is
        // under no shared temporary area here or on a hosted machine. **It
        // goes when the test ends, whichever way it ends**, its links with it
        // (piece P8b of step 10c): a failed run used to leave its link into
        // `/tmp` in the build folder.
        struct GoesWithTheTest(std::path::PathBuf);
        impl Drop for GoesWithTheTest {
            fn drop(&mut self) {
                std::fs::remove_dir_all(&self.0).ok();
            }
        }
        let at = std::env::current_exe()
            .expect("this test's own program")
            .parent()
            .expect("its folder")
            .join(format!("smoozy-a-recorded-root-that-is-a-link-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&at);
        let _goes = GoesWithTheTest(at.clone());
        let package_area = at.join("state/package");
        std::fs::create_dir_all(&package_area).unwrap();
        let package_area_text = package_area.display().to_string();

        let into_the_wiped_area = at.join("data");
        std::os::unix::fs::symlink("/tmp", &into_the_wiped_area).unwrap();
        std::fs::write(
            package_area.join(THE_RECORD_OF_THE_ROOT),
            format!("{}/smoozy", into_the_wiped_area.display()),
        )
        .unwrap();
        assert!(
            the_layout_this_machine_recorded(&package_area_text).is_none(),
            "a root through a link into /tmp gave a layout"
        );

        let volume = at.join("the-volume");
        std::fs::create_dir_all(&volume).unwrap();
        let to_a_good_place = at.join("mnt");
        std::os::unix::fs::symlink(&volume, &to_a_good_place).unwrap();
        let root = format!("{}/smoozy", to_a_good_place.display());
        std::fs::write(package_area.join(THE_RECORD_OF_THE_ROOT), &root).unwrap();
        let layout = the_layout_this_machine_recorded(&package_area_text).expect("a layout");
        assert_eq!(layout.root().as_path(), root);
    }

    #[test]
    fn the_walk_returns_every_entry_with_its_size_and_a_missing_folder_did_not_complete() {
        let root = std::env::temp_dir().join(format!(
            "smoozy-the-walk-{}",
            Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
        ));
        std::fs::create_dir_all(root.join("beauty")).unwrap();
        std::fs::write(root.join("beauty/0001.png"), b"12345").unwrap();
        std::fs::write(root.join("0002.png"), b"").unwrap();
        let WhatTheWalkOfTheFolderIs::ItCompleted(mut found) =
            the_walk_of(&root.display().to_string())
        else {
            panic!("the walk completed");
        };
        found.sort_by(|a, b| a.path_under_the_job_output_folder.cmp(&b.path_under_the_job_output_folder));
        let seen: Vec<(Vec<u8>, u64)> = found
            .iter()
            .map(|entry| (entry.path_under_the_job_output_folder.clone(), entry.size))
            .collect();
        assert_eq!(
            seen,
            vec![
                (b"0002.png".to_vec(), 0),
                (b"beauty".to_vec(), 0),
                (b"beauty/0001.png".to_vec(), 5),
            ]
        );
        std::fs::remove_dir_all(&root).ok();
        assert_eq!(
            the_walk_of(&root.display().to_string()),
            WhatTheWalkOfTheFolderIs::ItDidNotComplete
        );
    }

    #[test]
    fn a_large_statement_stays_whole_and_a_missing_or_changed_one_is_not_accepted() {
        let directory = std::env::temp_dir().join(format!("smoozy-large-statement-{}", Identity::issue(IdentityKind::Operation).unwrap().as_text()));
        std::fs::create_dir_all(&directory).unwrap();
        let directory_text = directory.to_str().unwrap();
        let statement = TheCompletenessStatement {
            expected: 1,
            present: vec![vec![b'x'; THE_LONGEST_STATEMENT_ONE_REPORT_CARRIES]],
            absent: Vec::new(), present_at_zero_bytes: Vec::new(), what_the_added_write_places_produced: Vec::new(),
        };
        let mut ending = TheEnding::before_the_launch(RecordedOutcome::Finished, None, Ok(statement.clone()), WhatTheFigureIs::It(1));
        let fields = ending.as_fields_for(Fields::none(), directory_text, 1).unwrap();
        assert!(fields.value(THE_COMPLETENESS_STATEMENT_FIELD).is_none());
        assert_eq!(fields.text(THE_STATEMENT_IS_IN_THE_RECORD_DIRECTORY_FIELD), Some("yes"));
        let read = the_fields_with_the_recorded_statement(&fields, directory_text, 1).unwrap();
        assert_eq!(serde_json::from_slice::<TheCompletenessStatement>(read.value(THE_COMPLETENESS_STATEMENT_FIELD).unwrap()).unwrap(), statement);
        let before = std::fs::read(the_large_statements_path(directory_text, 1)).unwrap();
        assert_eq!(ending.as_fields_for(Fields::none(), directory_text, 1).unwrap(), fields, "a repeated report reuses the publication");
        assert!(the_fields_with_the_recorded_statement(&fields, directory_text, 2).is_err(), "another attempt cannot borrow this statement");
        ending.statement.as_mut().unwrap().as_mut().unwrap().expected += 1;
        assert!(ending.as_fields_for(Fields::none(), directory_text, 1).is_err(), "never replace a statement");
        assert_eq!(std::fs::read(the_large_statements_path(directory_text, 1)).unwrap(), before);
        std::fs::write(the_large_statements_path(directory_text, 2), b"incomplete").unwrap();
        assert!(the_fields_with_the_recorded_statement(&fields, directory_text, 2).is_err(), "an unreadable statement is not an empty one");
        std::fs::remove_dir_all(directory).unwrap();
    }

    /// **The ending's fields read back into the record the operation record
    /// keeps**, and an ending that carries no statement says why.
    #[test]
    fn an_ending_reported_reads_back_as_the_record_keeps_it() {
        let ending = TheEnding::before_the_launch(
            RecordedOutcome::Failed {
                reason: FailedReason::BeforeTheRenderInstruction(
                    smoozy_rendering_machine::endings::BeforeTheRenderInstruction::AReadingThatDidNotEndSuccessfully,
                ),
                cause: Cause::Undetermined,
            },
            None,
            Err("no inventory".to_string()),
            WhatTheFigureIs::TheEndingsStepNeverRan,
        );
        let fields = ending.as_fields(Fields::none());
        assert_eq!(fields.text(asked::THE_ENDING_FIELD), Some(asked::IT_FAILED));
        assert_eq!(fields.text(THE_CAUSE_FIELD), Some(UNDETERMINED));
        assert_eq!(
            fields.text(asked::WHY_NO_COMPLETENESS_STATEMENT_FIELD),
            Some("no inventory")
        );
        assert_eq!(fields.text(THE_COMPLETENESS_STATEMENT_FIELD), None);
        let recorded = TheEndingAsRecorded::from_the_report(&fields).expect("it reads");
        assert_eq!(recorded.ending, asked::IT_FAILED);
        assert_eq!(recorded.cause.as_deref(), Some(UNDETERMINED));
        assert_eq!(recorded.exit_status, None);
        assert_eq!(recorded.ordinary, None, "no launch, so no stream came to anything");
    }

    /// **A stand-in for the kernel's per-process record**, so a render can be
    /// started at a desk whose kernel publishes none. It stands in for the
    /// reading and never for a judgement.
    struct AKernelThatAnswers;

    impl smoozy_runtime::execution_group::ValidatesTheRecordedPair for AKernelThatAnswers {
        fn what_is_at(&self, identifier: u32) -> Result<Option<RecordedPair>, String> {
            Ok(Some(RecordedPair {
                identifier,
                start_time: 7,
            }))
        }
    }

    /// **Steps 8 and 9 run for real against a stand-in for Blender**, which
    /// is a shell script named as one: both streams copied into the sinks the
    /// supervisor would have made, the render reaped, the marker observed as
    /// the bytes passed, and the ending classified by `Rendering` K14's own
    /// function. **Nothing here says what a real Blender prints**: the
    /// stand-in prints the material's marker head because that is what
    /// `Rented-server runtime` M28 observes, and the ending turns on it.
    ///
    /// **Reached from the supervisor's own tests too** (step 10c's fix piece
    /// 2), so a finished render can be played through the supervisor's writes
    /// at a desk whose kernel publishes no per-process record.
    pub(crate) fn after_a_stand_in_prints(script: &str) -> (TheEnding, String, String) {
        let (ending, ordinary, complaints, _) = after_a_stand_in_prints_with(script, |_| {});
        (ending, ordinary, complaints)
    }

    /// **The same, with something put in the record directory before the
    /// stand-in starts** — `beside` is handed that folder, and `{root}` in the
    /// script is replaced by it — **and what the attempt left there under
    /// [`the_placed_copies_after_the_render_file_name`] handed back.** Added
    /// 2026-10-02, the fixes before 0.1.8, fault 3.
    fn after_a_stand_in_prints_with(
        script: &str,
        beside: impl FnOnce(&std::path::Path),
    ) -> (TheEnding, String, String, Option<Vec<u8>>) {
        a_stand_in_render(
            script,
            beside,
            None,
            TheFiguresGo {
                the_plan: None,
                to: &|_| {},
            },
        )
    }

    /// **The same, under the operation and run named, with the live figures
    /// sent where `figures` says** — so a supervisor's own tests can play the
    /// whole report through (the Creator's notes on 0.1.8, piece 6a).
    pub(crate) fn after_a_stand_in_prints_reporting(
        script: &str,
        under: (Identity, Identity),
        figures: TheFiguresGo<'_>,
    ) -> TheEnding {
        a_stand_in_render(script, |_| {}, Some(under), figures).0
    }

    /// **A plan of one view layer at `samples` on each frame, one tile each** —
    /// `Queue & History` B1's renderlets as the driver hands them over, for the
    /// tests that run the worker's own loop without a reading. Piece 6b.
    pub(crate) fn a_plan_of(
        frames: &[i64],
        samples: u64,
    ) -> Vec<smoozy_queue_history_machine::plan::Renderlet> {
        frames
            .iter()
            .map(|frame| smoozy_queue_history_machine::plan::Renderlet {
                frame: *frame,
                view_layer: "ViewLayer".to_string(),
                planned_samples: samples,
                tiles: 1,
            })
            .collect()
    }

    fn a_stand_in_render(
        script: &str,
        beside: impl FnOnce(&std::path::Path),
        under: Option<(Identity, Identity)>,
        figures: TheFiguresGo<'_>,
    ) -> (TheEnding, String, String, Option<Vec<u8>>) {
        use smoozy_rendering_machine::acting::launch::StartsTheRender as _;
        use smoozy_runtime::capture::MakesTheSinks;
        use std::os::unix::fs::PermissionsExt;
        let root = std::env::temp_dir().join(format!(
            "smoozy-after-the-launch-{}",
            Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
        ));
        std::fs::create_dir_all(&root).unwrap();
        let stand_in = root.join("a-stand-in-for-blender");
        beside(&root);
        std::fs::write(&stand_in, script.replace("{root}", &root.display().to_string())).unwrap();
        std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755)).unwrap();

        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let capture = CaptureRecord {
            job: job.clone(),
            attempt: 1,
            ordinary_sink: root.join("streams/ordinary").display().to_string(),
            complaints_sink: root.join("streams/complaints").display().to_string(),
            ceiling: CaptureCeiling::from(
                smoozy_runtime::capture::THE_CONTROL_STATE_RESERVE + 1024 * 1024,
            ),
        };
        smoozy_runtime::acting::capture::TheSinksOnTheMachine
            .make(&capture)
            .expect("the sinks are made");
        let (operation, run) = under.unwrap_or_else(|| {
            (
                Identity::issue(IdentityKind::Operation).expect("randomness"),
                Identity::issue(IdentityKind::Run).expect("randomness"),
            )
        });
        let work = TheWork {
            operation,
            job: job.clone(),
            run,
            attempt: 1,
            record_directory: root.display().to_string(),
            capture: capture.clone(),
            the_machines_clock: Some(Clock::AMachine(
                smoozy_state::observed::MachineClock("installation-1".to_string()),
            )),
            the_layout: None,
        };
        let composed = TheAttemptAsComposed {
            manifest: smoozy_rendering_machine::composition::testing::a_manifest(&job, "5.2", "Scene"),
            payload: smoozy_rendering_machine::material::testing::an_untouched_payload(),
            version: Version { major: 5, minor: 2, patch: 0 },
            remap_material_path: String::new(),
            launch_material_path: String::new(),
            load_time_script_path: String::new(),
            reading_script_path: String::new(),
            the_folder_the_blend_sits_in: b"/ephemeral/smoozy/assets".to_vec(),
            previous_attempts: Vec::new(),
            the_machines_clock: Clock::AMachine(smoozy_state::observed::MachineClock(
                "installation-1".to_string(),
            )),
            the_manifests_reading: None,
            the_job_output_folder: Some("/ephemeral/smoozy/jobs/job".to_string()),
            the_layout: None,
        };
        let started = TheRenderStartedOnTheMachine {
            kernel: AKernelThatAnswers,
        }
        .start(&smoozy_rendering_machine::acting::launch::TheLaunch {
            the_blender_build_path: stand_in.display().to_string(),
            words: Vec::new(),
        })
        .expect("the stand-in starts");
        // **No reading stood in this stand-in**, so the ending says why it can
        // state no completeness rather than counting against nothing.
        let ending = after_the_launch(started, &work, &composed, None, &[1, 2, 3], &[1, 2, 3], figures);
        let ordinary = std::fs::read_to_string(&capture.ordinary_sink).unwrap();
        let complaints = std::fs::read_to_string(&capture.complaints_sink).unwrap();
        let after = std::fs::read(root.join(the_placed_copies_after_the_render_file_name(1))).ok();
        std::fs::remove_dir_all(&root).ok();
        (ending, ordinary, complaints, after)
    }

    /// **`Download` N5a's *now*: the moment a launched render has ended, every
    /// file the attempt placed is read again, beside the list** — and a file
    /// the render wrote over reads as written over, while one it left alone
    /// reads as it was made. The fixes before 0.1.8, fault 3.
    ///
    /// **The case it bites on**: nothing read them again, so the desktop had
    /// no *now* to compare with and brought every placed copy home.
    #[test]
    fn the_placed_copies_are_read_again_the_moment_the_render_has_ended() {
        let (ending, _, _, after) = after_a_stand_in_prints_with(
            "#!/bin/sh\nprintf 'the render wrote here' >> '{root}/pointcache/b.bphys'\n\
             echo 'SMOOZY-MATERIAL {\"job\":\"a stand-in\"}'\nexit 0\n",
            |root| {
                use std::os::unix::ffi::OsStrExt;
                let cache = root.join("pointcache");
                std::fs::create_dir_all(&cache).unwrap();
                let mut placed = ThePlacedCopies::default();
                for name in ["a.bphys", "b.bphys"] {
                    let at = cache.join(name);
                    std::fs::write(&at, b"a baked frame").unwrap();
                    let read = std::fs::metadata(&at).unwrap();
                    placed.files.push(APlacedFile {
                        job_folder_path: smoozy_dependencies_machine::bytes::MachineLocation::composed(
                            at.as_os_str().as_bytes().to_vec(),
                        ),
                        size: read.size(),
                        inode: read.ino().to_string(),
                        modification_time: read.mtime(),
                        change_time: read.ctime(),
                    });
                }
                std::fs::write(root.join(the_placed_copies_file_name(1)), placed.as_written()).unwrap();
            },
        );
        assert_eq!(ending.outcome, RecordedOutcome::Finished);
        let after = ThePlacedCopies::read_back(&after.expect("the reading after the render is written"))
            .expect("it reads");
        assert_eq!(after.files.len(), 2, "{after:?}");
        let size_of = |name: &str| {
            after
                .files
                .iter()
                .find(|file| file.job_folder_path.as_bytes().ends_with(name.as_bytes()))
                .map(|file| file.size)
        };
        assert_eq!(size_of("a.bphys"), Some(13), "the file the render left alone");
        assert_eq!(
            size_of("b.bphys"),
            Some(13 + 21),
            "the file the render wrote over is read as it stands after the render"
        );
    }

    #[test]
    fn a_render_that_prints_the_marker_and_ends_at_nought_is_finished_and_both_streams_are_kept() {
        let (ending, ordinary, complaints) = after_a_stand_in_prints(
            "#!/bin/sh\necho 'SMOOZY-MATERIAL {\"job\":\"a stand-in\"}'\necho 'Fra:1 a line'\necho 'a complaint' >&2\nexit 0\n",
        );
        assert_eq!(ending.outcome, RecordedOutcome::Finished);
        assert_eq!(ending.exit_status, Some(0));
        let (on_ordinary, on_complaints) = ending.streams.expect("a launch was made");
        assert_eq!(on_ordinary.outcome, CollectionOutcome::Collected);
        assert_eq!(on_complaints.outcome, CollectionOutcome::Collected);
        assert_eq!(on_ordinary.observed.the_marker_passed, Some(true));
        assert_eq!(on_ordinary.observed.retained, ordinary.len() as u64);
        assert!(ordinary.contains("Fra:1 a line"));
        assert_eq!(complaints, "a complaint\n", "`Rented-server runtime` M7: never merged");
        // **No inventory, so the ending says why it carries no statement.**
        assert!(matches!(ending.statement, Some(Err(_))));
    }

    /// **What a 5.x render prints for its devices, as this test's stand-in
    /// prints it** — the head the reader matches and the machine's own words
    /// after it, which are illustrative: nothing turns on them, and they are
    /// carried as printed.
    pub(crate) const A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT: &str =
        "Path tracing on: NVIDIA GeForce RTX 4090 (CUDA)";

    /// A stand-in's script that prints the device line in the 5.x shape, the
    /// material's marker where `marker` says, and ends at `status`.
    pub(crate) fn a_stand_in_that_prints_a_device_line(marker: bool, status: i32) -> String {
        format!(
            "#!/bin/sh\n{}echo '00:00.600  cycles           | {A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT}'\necho 'Fra:1 a line'\nexit {status}\n",
            if marker {
                "echo 'SMOOZY-MATERIAL {\"job\":\"a stand-in\"}'\n"
            } else {
                ""
            }
        )
    }

    /// **`Queue & History` E13, kept: the device line the tee observed at this
    /// worker's own loop rides the ending, verbatim, and the report carries it
    /// to the supervisor** (the paragraph after `Queue & History` N3).
    ///
    /// **The fault this catches is the seventh passes' *Ran on — not known***:
    /// the tee observed the line on every provider and the loop threw it away
    /// with the rest of the observers.
    #[test]
    fn a_finished_render_carries_the_device_line_it_printed_into_the_report_verbatim() {
        let (ending, ordinary, _) =
            after_a_stand_in_prints(&a_stand_in_that_prints_a_device_line(true, 0));
        assert_eq!(ending.outcome, RecordedOutcome::Finished);
        assert_eq!(
            ending.the_device_line.as_deref(),
            Some(A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT),
            "the line as printed, with nothing read out of it"
        );
        assert!(
            ordinary.contains(A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT),
            "and it is still in the sink, which is the record (`Queue & History` E7)"
        );
        let fields = ending.as_fields(Fields::none());
        assert_eq!(
            fields.text(asked::THE_DEVICE_LINE_FIELD),
            Some(A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT)
        );
        assert_eq!(fields.text(THE_REASON_FIELD), None, "a finished ending records no reason");
        let recorded = TheEndingAsRecorded::from_the_report(&fields).expect("it reads");
        assert_eq!(
            recorded.device_line.as_deref(),
            Some(A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT)
        );
    }

    /// **The line is carried on a failed ending too, and nothing here judges
    /// it** — `Queue & History` E12's *not known* for a run that did not
    /// finish is the entry's to say. The reason rides beside it, in words.
    #[test]
    fn a_failed_render_carries_its_reason_in_words_and_the_line_as_observed() {
        let (ending, _, _) =
            after_a_stand_in_prints(&a_stand_in_that_prints_a_device_line(true, 3));
        assert!(matches!(ending.outcome, RecordedOutcome::Failed { .. }));
        assert_eq!(
            ending.the_device_line.as_deref(),
            Some(A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT)
        );
        let fields = ending.as_fields(Fields::none());
        let reason = fields.text(THE_REASON_FIELD).expect("a failed ending carries its reason");
        assert!(!reason.is_empty());
        assert!(
            !reason.contains('{') && !reason.contains("ANonZeroStatus"),
            "the reason is words, never the type's debugging form: {reason}"
        );
        let recorded = TheEndingAsRecorded::from_the_report(&fields).expect("it reads");
        assert_eq!(recorded.reason.as_deref(), Some(reason));
        assert_eq!(
            recorded.device_line.as_deref(),
            Some(A_DEVICE_LINE_AS_A_STAND_IN_PRINTS_IT)
        );
    }

    /// **An operation record written before fix piece 2 still reads back**,
    /// with no line observed rather than a record that will not read.
    #[test]
    fn a_recorded_ending_written_before_the_device_line_reads_back_with_none() {
        let field: TheRenderKindsOwnField = serde_json::from_str(
            r#"{"job":"j","attempts":[{"attempt":1,"ordinary_sink":"o","complaints_sink":"c","available_bytes":1,"ending":{"ending":"finished","cause":null,"reason":null,"exit_status":0,"ordinary":null,"complaints":null,"ordinary_retained":null,"ordinary_discarded":null,"ordinary_first_loss":null,"complaints_retained":null,"complaints_discarded":null,"complaints_first_loss":null,"marker_passed":true,"something_followed_the_marker":null}}]}"#,
        )
        .expect("an older record reads back");
        assert_eq!(
            field.attempts[0].ending.as_ref().expect("the ending").device_line,
            None
        );
    }

    #[test]
    fn a_render_that_never_reached_the_material_and_ended_at_three_is_failed_undetermined() {
        let (ending, ordinary, _) =
            after_a_stand_in_prints("#!/bin/sh\necho 'no marker here'\nexit 3\n");
        assert_eq!(
            ending.outcome,
            RecordedOutcome::Failed {
                reason: FailedReason::ANonZeroStatus { exit_status: 3 },
                cause: Cause::Undetermined,
            }
        );
        let (on_ordinary, _) = ending.streams.clone().expect("a launch was made");
        assert_eq!(
            on_ordinary.outcome,
            CollectionOutcome::NeverEstablished,
            "`Rented-server runtime` M15: no marker at all is a run that did not reach the material"
        );
        assert_eq!(ordinary, "no marker here\n");
        let fields = ending.as_fields(Fields::none());
        assert_eq!(fields.text(asked::THE_EXIT_STATUS_FIELD), Some("3"));
        assert_eq!(fields.text(THE_ORDINARY_COLLECTION_FIELD), Some("never established"));
    }

    /// **The Creator's notes on 0.1.8, piece 6a, at the desk.** A real Blender
    /// renders a real ten-frame file through the worker's own steps 8 and 9,
    /// with the desk's stand-in for the kernel's per-process record, and every
    /// report that left the worker during the render is printed as it went.
    /// **Before the piece nothing left it**: the ending's report carried no
    /// figure, and the reader, read again off the same bytes, held frame 10 and
    /// 256 of 256 samples that nothing ever sent.
    #[test]
    #[ignore = "run by hand at the desk with a real Blender and a real .blend"]
    fn a_real_blender_renders_through_the_workers_own_loop_and_its_figures_leave_it() {
        use smoozy_rendering_machine::acting::launch::StartsTheRender as _;
        use smoozy_runtime::capture::MakesTheSinks;
        let blender = std::env::var("SMOOZY_DESK_BLENDER").expect("SMOOZY_DESK_BLENDER");
        let blend = std::env::var("SMOOZY_DESK_BLEND").expect("SMOOZY_DESK_BLEND");
        let root = std::env::temp_dir().join(format!(
            "smoozy-a-real-render-{}",
            Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
        ));
        std::fs::create_dir_all(&root).unwrap();
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let capture = CaptureRecord {
            job: job.clone(),
            attempt: 1,
            ordinary_sink: root.join("streams/ordinary").display().to_string(),
            complaints_sink: root.join("streams/complaints").display().to_string(),
            ceiling: CaptureCeiling::from(
                smoozy_runtime::capture::THE_CONTROL_STATE_RESERVE + 64 * 1024 * 1024,
            ),
        };
        smoozy_runtime::acting::capture::TheSinksOnTheMachine
            .make(&capture)
            .expect("the sinks are made");
        let work = TheWork {
            operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
            job: job.clone(),
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            attempt: 1,
            record_directory: root.display().to_string(),
            capture: capture.clone(),
            the_machines_clock: Some(Clock::AMachine(
                smoozy_state::observed::MachineClock("installation-1".to_string()),
            )),
            the_layout: None,
        };
        let composed = TheAttemptAsComposed {
            manifest: smoozy_rendering_machine::composition::testing::a_manifest(&job, "5.2", "Scene"),
            payload: smoozy_rendering_machine::material::testing::an_untouched_payload(),
            version: Version { major: 5, minor: 2, patch: 2 },
            remap_material_path: String::new(),
            launch_material_path: String::new(),
            load_time_script_path: String::new(),
            reading_script_path: String::new(),
            the_folder_the_blend_sits_in: b"/ephemeral/smoozy/assets".to_vec(),
            previous_attempts: Vec::new(),
            the_machines_clock: Clock::AMachine(smoozy_state::observed::MachineClock(
                "installation-1".to_string(),
            )),
            the_manifests_reading: None,
            the_job_output_folder: Some("/ephemeral/smoozy/jobs/job".to_string()),
            the_layout: None,
        };
        let frames: Vec<i64> = (1..=10).collect();
        let started = TheRenderStartedOnTheMachine {
            kernel: AKernelThatAnswers,
        }
        .start(&smoozy_rendering_machine::acting::launch::TheLaunch {
            the_blender_build_path: blender,
            words: vec![
                "--factory-startup".to_string(),
                "-b".to_string(),
                blend,
                "-o".to_string(),
                root.join("out/f_").display().to_string(),
                "-a".to_string(),
            ],
        })
        .expect("the real Blender starts");
        let began = std::time::Instant::now();
        let sent: std::sync::Mutex<Vec<AWorkersReport>> = std::sync::Mutex::new(Vec::new());
        let ending = after_the_launch(
            started,
            &work,
            &composed,
            None,
            &frames,
            &frames,
            TheFiguresGo {
                the_plan: Some(a_plan_of(&frames, 256)),
                to: &|report| {
                    let read = report.readings.value();
                    println!(
                        "{:>6.1}s sent: frame {:?}, samples {:?}, frames counted {}, newest saved {:?}, \
                         Blender's estimate {:?}, plan total {:?}, plan units finished {:?}",
                        began.elapsed().as_secs_f64(),
                        read.frame_in_hand,
                        read.samples.map(|pair| (pair.seen, pair.of)),
                        read.frames_completed,
                        report.newest_frame_saved,
                        read.blenders_own_estimate_for_the_frame,
                        report.the_plan_total,
                        report.the_units_finished,
                    );
                    sent.lock().unwrap().push(report);
                },
            },
        );
        let took = began.elapsed();
        let ordinary = std::fs::read(&capture.ordinary_sink).unwrap();
        println!(
            "the render ended {:?} after {took:?}, {} bytes on the ordinary stream",
            ending.outcome,
            ordinary.len()
        );
        let sent = sent.into_inner().unwrap();
        println!("{} reports left the worker", sent.len());
        let frames_read: Vec<i64> = sent
            .iter()
            .filter_map(|report| report.readings.value().frame_in_hand)
            .collect();
        assert!(
            sent.len() as u64 >= took.as_secs() / 2 - 2,
            "a report every two seconds while it rendered"
        );
        assert_eq!(frames_read.first(), Some(&1));
        assert_eq!(frames_read.iter().max(), Some(&10));
        assert!(frames_read.windows(2).all(|two| two[0] <= two[1]), "{frames_read:?}");
        std::fs::remove_dir_all(&root).ok();
    }

    // -----------------------------------------------------------------------
    // Blender's crash file at a failed ending — step 11's R3b, machine half
    // -----------------------------------------------------------------------

    /// **A kernel that knows no process at all**, so every recorded pair reads
    /// as gone — a worker that was ended before it could close its mark.
    struct AKernelThatKnowsNothing;

    impl smoozy_runtime::execution_group::ValidatesTheRecordedPair for AKernelThatKnowsNothing {
        fn what_is_at(&self, _identifier: u32) -> Result<Option<RecordedPair>, String> {
            Ok(None)
        }
    }

    /// **One machine's area for these tests**: the job-record folder every job
    /// record directory sits in, side by side as on a machine, and a folder
    /// standing in for the one Blender writes its crash file to.
    struct AnArea {
        root: std::path::PathBuf,
    }

    impl AnArea {
        fn made() -> AnArea {
            let root = std::env::temp_dir().join(format!(
                "smoozy-the-crash-file-{}",
                Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
            ));
            std::fs::create_dir_all(root.join("job-records")).unwrap();
            std::fs::create_dir_all(root.join("tmp")).unwrap();
            AnArea { root }
        }

        /// The folder standing in for [`WHERE_BLENDER_WRITES_ITS_CRASH_FILE`].
        fn where_blender_writes(&self) -> std::path::PathBuf {
            self.root.join("tmp")
        }

        /// A job's record directory in the area, and the work at its attempt.
        fn a_job(&self, attempt: u32) -> TheWork {
            let job = Identity::issue(IdentityKind::Job).expect("randomness");
            let directory = self.root.join("job-records").join(job.as_text());
            std::fs::create_dir_all(&directory).unwrap();
            a_work_for(&job, attempt, &directory.display().to_string())
        }
    }

    impl Drop for AnArea {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.root).ok();
        }
    }

    /// The work a supervisor would answer for one attempt of `job`.
    pub(crate) fn a_work_for(job: &Identity, attempt: u32, record_directory: &str) -> TheWork {
        TheWork {
            operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
            job: job.clone(),
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            attempt,
            record_directory: record_directory.to_string(),
            capture: CaptureRecord {
                job: job.clone(),
                attempt,
                ordinary_sink: format!("{record_directory}/streams-{attempt}/ordinary"),
                complaints_sink: format!("{record_directory}/streams-{attempt}/complaints"),
                ceiling: CaptureCeiling::from(
                    smoozy_runtime::capture::THE_CONTROL_STATE_RESERVE + 1024 * 1024,
                ),
            },
            the_machines_clock: Some(Clock::AMachine(
                smoozy_state::observed::MachineClock("installation-1".to_string()),
            )),
            the_layout: None,
        }
    }

    /// **What a crash file of the research's shape holds** — a version line, a
    /// backtrace section and a Python backtrace section (Fact 131). The words
    /// are this test's own; nothing reads them.
    const A_CRASH_FILE: &str = "# Blender 5.2.0, a stand-in\n# backtrace\n./blender(+0x1) [0x1]\n\n# Python backtrace\n";

    /// **One attempt launched against a stand-in for Blender and taken to its
    /// ending, then the ending's last step** — the stand-in is a shell script
    /// named as one, and `{tmp}` in it is the folder standing in for the one
    /// Blender writes its crash file to. **Nothing here says what a real
    /// Blender does**: the stand-in writes a file where the research says
    /// Blender writes one, and ends as the script says.
    fn an_attempt_that_ends(
        area: &AnArea,
        work: &TheWork,
        blend: &str,
        script: &str,
        kernel: &dyn smoozy_runtime::execution_group::ValidatesTheRecordedPair,
    ) -> (TheEnding, Option<TheCrashFileStatement>) {
        use smoozy_rendering_machine::acting::launch::StartsTheRender as _;
        use smoozy_runtime::capture::MakesTheSinks;
        use std::os::unix::fs::PermissionsExt;
        smoozy_runtime::acting::capture::TheSinksOnTheMachine
            .make(&work.capture)
            .expect("the sinks are made");
        let stand_in = std::path::Path::new(&work.record_directory).join("a-stand-in-for-blender");
        std::fs::write(
            &stand_in,
            script.replace("{tmp}", &area.where_blender_writes().display().to_string()),
        )
        .unwrap();
        std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755)).unwrap();
        let mut composed = a_composed_attempt(&work.job);
        composed.manifest.blend_path_on_the_machine = blend.to_string();

        let launched_at = std::time::SystemTime::now();
        let before = the_crash_file_before_the_launch(blend.as_bytes(), &area.where_blender_writes());
        let started = TheRenderStartedOnTheMachine {
            kernel: AKernelThatAnswers,
        }
        .start(&smoozy_rendering_machine::acting::launch::TheLaunch {
            the_blender_build_path: stand_in.display().to_string(),
            words: Vec::new(),
        })
        .expect("the stand-in starts");
        let launch = the_launch_is_marked(work, blend.as_bytes(), launched_at, before, started.recorded, kernel);
        let ending = after_the_launch(
            started,
            work,
            &composed,
            None,
            &[1, 2, 3],
            &[1, 2, 3],
            TheFiguresGo {
                the_plan: None,
                to: &|_| {},
            },
        );
        let statement = the_crash_file_at_the_ending(
            work,
            Some(&launch),
            &ending,
            std::time::SystemTime::now(),
            &area.where_blender_writes(),
        );
        (ending, statement)
    }

    // -----------------------------------------------------------------------
    // `Rendering` K14b: a render that ends at 0 having written none of what the
    // job owed is failed. Reports 1 and 12, the 5.3 integration's piece 2.
    // -----------------------------------------------------------------------

    /// **An attempt launched against a stand-in, against a real inventory and a
    /// real job output folder.** `{out}` in the script is that folder. **Nothing
    /// here says what a real Blender does**: *Reports 1* is what says a scene
    /// with no camera ends at 0 with nothing written in 5.3, and this plays that
    /// ending through the worker's own ending step.
    fn an_attempt_against_an_inventory(
        names_owed: &[(&str, Option<i64>)],
        a_video: bool,
        script: &str,
    ) -> TheEnding {
        use smoozy_job_files_machine::inventory::{AnEntry, TheInventory, WhichRouteProducedTheName};
        use smoozy_rendering_machine::acting::launch::StartsTheRender as _;
        use smoozy_runtime::capture::MakesTheSinks;
        use std::os::unix::fs::PermissionsExt;
        let area = AnArea::made();
        let work = area.a_job(1);
        let folder = area.root.join("out");
        std::fs::create_dir_all(&folder).unwrap();
        smoozy_runtime::acting::capture::TheSinksOnTheMachine
            .make(&work.capture)
            .expect("the sinks are made");
        let stand_in = std::path::Path::new(&work.record_directory).join("a-stand-in-for-blender");
        std::fs::write(&stand_in, script.replace("{out}", &folder.display().to_string())).unwrap();
        std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755)).unwrap();
        let mut composed = a_composed_attempt(&work.job);
        composed.the_job_output_folder = Some(folder.display().to_string());
        let inventory = WhatTheInventoryIs::It(TheInventory {
            entries: names_owed
                .iter()
                .map(|(name, frame)| AnEntry {
                    predicted_path: folder.join(name).display().to_string().into_bytes(),
                    frame: *frame,
                    the_route_that_produced_the_name: WhichRouteProducedTheName::ItWasAskedOfBlender,
                    the_destinations_aiming_at_it: vec![0],
                    may_grow: a_video,
                })
                .collect(),
            added_write_place_folders: Vec::new(),
            collision_record: Vec::new(),
            a_destination_is_video_shaped: a_video,
            why_it_owes_nothing: None,
        });
        let started = TheRenderStartedOnTheMachine {
            kernel: AKernelThatAnswers,
        }
        .start(&smoozy_rendering_machine::acting::launch::TheLaunch {
            the_blender_build_path: stand_in.display().to_string(),
            words: Vec::new(),
        })
        .expect("the stand-in starts");
        after_the_launch(
            started,
            &work,
            &composed,
            Some(&inventory),
            &[1, 2],
            &[1, 2],
            TheFiguresGo {
                the_plan: None,
                to: &|_| {},
            },
        )
    }

    const THE_MARKER_AND_EXIT_NOUGHT: &str =
        "echo 'SMOOZY-MATERIAL {\"job\":\"a stand-in\"}'\nexit 0\n";

    fn at_nought_having_run(before: &str) -> String {
        format!("#!/bin/sh\n{before}\n{THE_MARKER_AND_EXIT_NOUGHT}")
    }

    const THE_REASON_IN_WORDS: &str = "Blender exited normally but wrote none of the job's files";

    fn it_is_the_count_that_failed_it(ending: &TheEnding) -> bool {
        matches!(
            &ending.outcome,
            RecordedOutcome::Failed { reason, cause: Cause::Undetermined }
                if reason.in_words() == THE_REASON_IN_WORDS
        )
    }

    const TWO_FRAMES: [(&str, Option<i64>); 2] = [("f_0001.png", Some(1)), ("f_0002.png", Some(2))];

    /// **The ending *Reports 1* records for 5.3: status 0, the marker passed,
    /// nothing written.** Failed, undetermined, its reason in words, and the
    /// statement still beside it — and **the report says so in the words the
    /// supervisor keeps**, so History, the Queue, Telegram and a hold behind it
    /// all read an ordinary *failed*.
    #[test]
    fn a_render_ending_at_nought_having_written_nothing_it_owed_is_failed() {
        let ending = an_attempt_against_an_inventory(&TWO_FRAMES, false, &at_nought_having_run(""));
        assert!(it_is_the_count_that_failed_it(&ending), "{:?}", ending.outcome);
        assert_eq!(ending.exit_status, Some(0), "the status is kept verbatim");
        let Some(Ok(statement)) = &ending.statement else {
            panic!("the statement stands beside it: {:?}", ending.statement);
        };
        assert_eq!((statement.expected, statement.present.len(), statement.absent.len()), (2, 0, 2));
        assert_eq!(ending.figure, WhatTheFigureIs::NoFrameCountsAsAlreadyThere);
        let fields = ending.as_fields(Fields::none());
        assert_eq!(fields.text(asked::THE_ENDING_FIELD), Some(asked::IT_FAILED));
        assert_eq!(fields.text(THE_CAUSE_FIELD), Some(UNDETERMINED));
        assert_eq!(fields.text(THE_REASON_FIELD), Some(THE_REASON_IN_WORDS));
        assert_eq!(fields.text(asked::THE_EXIT_STATUS_FIELD), Some("0"));
    }

    /// **A file at zero bytes is not written** — `Job files and outputs` C27's
    /// own fourth set, and a single still on a full disk can leave exactly that
    /// on Linux.
    #[test]
    fn files_left_at_zero_bytes_are_not_written() {
        let ending = an_attempt_against_an_inventory(
            &TWO_FRAMES,
            false,
            &at_nought_having_run(": > '{out}/f_0001.png'\n: > '{out}/f_0002.png'"),
        );
        assert!(it_is_the_count_that_failed_it(&ending), "{:?}", ending.outcome);
    }

    /// **The ruling's other half: a partly written job stays as it is**, the
    /// count short beside *finished* (`Queue & History` E3); so does a whole one.
    #[test]
    fn a_job_with_anything_of_what_it_owed_written_stays_finished() {
        let one = an_attempt_against_an_inventory(
            &TWO_FRAMES,
            false,
            &at_nought_having_run("printf frame > '{out}/f_0001.png'"),
        );
        assert_eq!(one.outcome, RecordedOutcome::Finished);
        let Some(Ok(statement)) = &one.statement else {
            panic!("{:?}", one.statement);
        };
        assert!(statement.the_count_came_up_short());
        let both = an_attempt_against_an_inventory(
            &TWO_FRAMES,
            false,
            &at_nought_having_run("printf frame > '{out}/f_0001.png'\nprintf frame > '{out}/f_0002.png'"),
        );
        assert_eq!(both.outcome, RecordedOutcome::Finished);
    }

    /// **A video job prints no `Saved:` line and is judged by its one file**: the
    /// count reads the disk, never the stream.
    #[test]
    fn a_video_job_is_judged_by_its_one_file_and_needs_no_saved_line() {
        let the_video = [("f_0001-0002.mkv", None)];
        let written = an_attempt_against_an_inventory(
            &the_video,
            true,
            &at_nought_having_run("printf video > '{out}/f_0001-0002.mkv'"),
        );
        assert_eq!(written.outcome, RecordedOutcome::Finished);
        let nothing = an_attempt_against_an_inventory(&the_video, true, &at_nought_having_run(""));
        assert!(it_is_the_count_that_failed_it(&nothing), "{:?}", nothing.outcome);
    }

    /// **A job that owes nothing is not a failure** (`Specs/What a job writes`),
    /// and it stays *finished* with nothing written.
    #[test]
    fn a_job_that_owed_nothing_stays_finished() {
        let ending = an_attempt_against_an_inventory(&[], false, &at_nought_having_run(""));
        assert_eq!(ending.outcome, RecordedOutcome::Finished);
    }

    /// **It moves *finished* and nothing else**: a non-zero status and a marker
    /// never seen keep their own reasons, whatever the count says.
    #[test]
    fn the_count_moves_no_other_ending() {
        let at_three = an_attempt_against_an_inventory(
            &TWO_FRAMES,
            false,
            "#!/bin/sh\necho 'SMOOZY-MATERIAL {\"job\":\"a stand-in\"}'\nexit 3\n",
        );
        assert!(
            matches!(
                at_three.outcome,
                RecordedOutcome::Failed { reason: FailedReason::ANonZeroStatus { exit_status: 3 }, .. }
            ),
            "{:?}",
            at_three.outcome
        );
        let no_marker = an_attempt_against_an_inventory(&TWO_FRAMES, false, "#!/bin/sh\nexit 0\n");
        assert!(
            matches!(
                no_marker.outcome,
                RecordedOutcome::Failed { reason: FailedReason::TheMarkerWasNeverObserved { .. }, .. }
            ),
            "{:?}",
            no_marker.outcome
        );
    }

    /// The attempt as `after_the_launch` takes it, for a stand-in.
    fn a_composed_attempt(job: &Identity) -> TheAttemptAsComposed {
        TheAttemptAsComposed {
            manifest: smoozy_rendering_machine::composition::testing::a_manifest(job, "5.2", "Scene"),
            payload: smoozy_rendering_machine::material::testing::an_untouched_payload(),
            version: Version { major: 5, minor: 2, patch: 0 },
            remap_material_path: String::new(),
            launch_material_path: String::new(),
            load_time_script_path: String::new(),
            reading_script_path: String::new(),
            the_folder_the_blend_sits_in: b"/ephemeral/smoozy/assets".to_vec(),
            previous_attempts: Vec::new(),
            the_machines_clock: Clock::AMachine(smoozy_state::observed::MachineClock(
                "installation-1".to_string(),
            )),
            the_manifests_reading: None,
            the_job_output_folder: Some("/ephemeral/smoozy/jobs/job".to_string()),
            the_layout: None,
        }
    }

    /// A stand-in that writes a crash file under `name` and ends at `status`.
    fn a_stand_in_that_crashes(name: &str, status: i32) -> String {
        format!("#!/bin/sh\nprintf '{A_CRASH_FILE}' > '{{tmp}}/{name}'\nexit {status}\n")
    }

    /// **`Reporting a problem` G5's first bound, as the research observed it**:
    /// `heavy.blend` gives `heavy.crash.txt`; the folder never enters the name;
    /// and a file whose name does not end in `.blend` has no name on record.
    #[test]
    fn the_crash_files_name_is_the_loaded_files_with_its_ending_replaced() {
        assert_eq!(the_crash_files_name_for(b"heavy.blend").unwrap(), b"heavy.crash.txt");
        assert_eq!(
            the_crash_files_name_for(b"/ephemeral/smoozy/assets/a b/Kitchen v2.blend").unwrap(),
            b"Kitchen v2.crash.txt"
        );
        assert!(the_crash_files_name_for(b"/assets/kitchen").is_err());
        assert!(the_crash_files_name_for(b"/assets/.blend").is_err());
        assert!(the_crash_files_name_for(b"/assets/kitchen.blend1").is_err());
    }

    /// **A render that crashes and leaves its file has a copy kept whole for
    /// the attempt, and the reading serves it under the name Blender gave
    /// it** — at a crash's status and at an ordinary failed one alike, since
    /// the step is never keyed to an exit number (`Rendering` K6). **The
    /// original is left where Blender wrote it** (`Reporting a problem` G5a).
    ///
    /// **The fault it bites on**: nothing looked for the file at all, so a
    /// report could only ever say the part was absent.
    #[test]
    fn a_render_that_crashes_and_leaves_its_file_has_a_copy_kept_whole_for_the_attempt() {
        for status in [139, 3] {
            let area = AnArea::made();
            let work = area.a_job(1);
            let (ending, statement) = an_attempt_that_ends(
                &area,
                &work,
                "/ephemeral/smoozy/assets/kitchen.blend",
                &a_stand_in_that_crashes("kitchen.crash.txt", status),
                &AKernelThatAnswers,
            );
            assert!(matches!(ending.outcome, RecordedOutcome::Failed { .. }), "{:?}", ending.outcome);
            let statement = statement.expect("the step runs at a failed ending");
            let written = A_CRASH_FILE.to_string();
            assert_eq!(statement.kept, Some(written.len() as u64), "{statement:?}");
            assert_eq!(statement.why_none_was_kept, None);
            assert_eq!(statement.the_crash_files_own_name.as_deref(), Some(&b"kitchen.crash.txt"[..]));

            let kept = the_kept_crash_files_in(&work.record_directory).expect("it reads");
            assert_eq!(
                kept,
                vec![AKeptCrashFile {
                    attempt: 1,
                    the_crash_files_own_name: Some(b"kitchen.crash.txt".to_vec()),
                    what: WhatWasKept::TheFile(written.clone().into_bytes()),
                }]
            );
            assert_eq!(
                std::fs::read_to_string(area.where_blender_writes().join("kitchen.crash.txt")).unwrap(),
                written,
                "the original is left where Blender wrote it"
            );
        }
    }

    /// A new file and an overwritten file remain this attempt's even when
    /// their recorded write time is earlier than the launch clock. An
    /// unchanged file from attempt 1 must never be taken by attempt 2.
    #[test]
    fn a_new_or_changed_crash_file_with_a_time_before_the_launch_is_kept() {
        for already_there in [false, true] {
            let area = AnArea::made();
            let work = area.a_job(1);
            let at = area.where_blender_writes().join("kitchen.crash.txt");
            let earlier = std::time::SystemTime::now() - Duration::from_secs(60);
            if already_there {
                std::fs::write(&at, b"another attempt's file").unwrap();
                std::fs::File::options().write(true).open(&at).unwrap()
                    .set_times(std::fs::FileTimes::new().set_modified(earlier)).unwrap();
            }
            let before = the_crash_file_before_the_launch(b"/assets/kitchen.blend", &area.where_blender_writes());
            let launch = the_launch_is_marked(
                &work, b"/assets/kitchen.blend", std::time::SystemTime::now(), before,
                RecordedPair { identifier: 4242, start_time: 7 }, &AKernelThatAnswers,
            );
            std::fs::write(&at, A_CRASH_FILE).unwrap();
            std::fs::File::options().write(true).open(&at).unwrap()
                .set_times(std::fs::FileTimes::new().set_modified(earlier)).unwrap();
            assert!(a_moment_as_a_pair(std::fs::metadata(&at).unwrap().modified().unwrap()) < launch.launched_at);
            let statement = the_crash_file_step(&work, Some(&launch), std::time::SystemTime::now(), &area.where_blender_writes());
            assert_eq!(statement.kept, Some(A_CRASH_FILE.len() as u64), "{statement:?}");
            the_ending_is_marked(&launch, std::time::SystemTime::now());

            let next = a_work_for(&work.job, 2, &work.record_directory);
            let before = the_crash_file_before_the_launch(b"/assets/kitchen.blend", &area.where_blender_writes());
            let launch = the_launch_is_marked(
                &next, b"/assets/kitchen.blend", std::time::SystemTime::now(), before,
                RecordedPair { identifier: 4242, start_time: 7 }, &AKernelThatAnswers,
            );
            let statement = the_crash_file_step(&next, Some(&launch), std::time::SystemTime::now(), &area.where_blender_writes());
            assert_eq!(statement.kept, None, "an unchanged earlier attempt's file is refused");
            assert!(statement.why_none_was_kept.unwrap().contains("is unchanged since before"));
        }
    }

    /// **A render that fails and leaves no file says none was kept, and why**
    /// — `Reporting a problem` S5's *absent*, with the reason stated; and **a
    /// render that finishes takes no step at all**, while its mark is closed.
    /// **An attempt that ended before any launch says so**, rather than looking.
    #[test]
    fn a_render_that_leaves_no_file_says_none_was_kept_and_a_finished_one_takes_no_step() {
        let area = AnArea::made();
        let work = area.a_job(1);
        let (_, statement) = an_attempt_that_ends(
            &area,
            &work,
            "/assets/kitchen.blend",
            "#!/bin/sh\necho 'a failure line' >&2\nexit 1\n",
            &AKernelThatAnswers,
        );
        let statement = statement.expect("the step runs at a failed ending");
        assert_eq!(statement.kept, None);
        let why = statement.why_none_was_kept.expect("the reason is stated");
        assert!(why.starts_with("nothing stands at "), "{why}");
        assert!(why.ends_with("kitchen.crash.txt"), "{why}");
        assert_eq!(
            the_kept_crash_files_in(&work.record_directory).unwrap()[0].what,
            WhatWasKept::None(why.clone())
        );

        let finished = area.a_job(1);
        let (ending, statement) = an_attempt_that_ends(
            &area,
            &finished,
            "/assets/garden.blend",
            "#!/bin/sh\necho 'SMOOZY-MATERIAL {\"job\":\"a stand-in\"}'\nprintf 'x' > '{tmp}/garden.crash.txt'\nexit 0\n",
            &AKernelThatAnswers,
        );
        assert_eq!(ending.outcome, RecordedOutcome::Finished);
        assert_eq!(statement, None, "no step at a finished ending");
        assert_eq!(the_kept_crash_files_in(&finished.record_directory).unwrap(), Vec::new());
        let mark: TheFileThisAttemptLoaded = serde_json::from_slice(
            &std::fs::read(std::path::Path::new(&finished.record_directory).join(the_file_loaded_marks_name(1)))
                .expect("the mark"),
        )
        .unwrap();
        assert!(mark.ended_at.is_some(), "the mark is closed at every ending");

        let never_launched = area.a_job(2);
        let statement = the_crash_file_step(
            &never_launched,
            None,
            std::time::SystemTime::now(),
            &area.where_blender_writes(),
        );
        assert_eq!(statement.attempt, 2);
        assert!(
            statement.why_none_was_kept.as_deref().is_some_and(|why| why.starts_with("no render was launched")),
            "{statement:?}"
        );
    }

    /// **Two jobs of the same file** — `Reporting a problem` G5a's *which file
    /// is this attempt's*, both ways it can go wrong:
    ///
    /// - **one after the other**: the first crashes and its file is kept; the
    ///   second, with a file of the same name in another folder, fails without
    ///   crashing, and the file the first left is **not** taken — it was
    ///   unchanged since before the second's launch;
    /// - **at once**: while one attempt has the file loaded and is running,
    ///   another's crash file of that name is not taken, naming the attempt it
    ///   may be; **and once that attempt's render process is gone**, its mark
    ///   left open by a worker that was ended, a later attempt takes its own.
    #[test]
    fn two_jobs_of_the_same_file_never_take_each_others_crash_file() {
        let area = AnArea::made();
        let first = area.a_job(1);
        let (_, statement) = an_attempt_that_ends(
            &area,
            &first,
            "/assets/one/kitchen.blend",
            &a_stand_in_that_crashes("kitchen.crash.txt", 139),
            &AKernelThatAnswers,
        );
        assert!(statement.expect("a statement").kept.is_some());
        let second = area.a_job(1);
        let (_, statement) = an_attempt_that_ends(
            &area,
            &second,
            "/assets/two/kitchen.blend",
            "#!/bin/sh\nexit 1\n",
            &AKernelThatAnswers,
        );
        let why = statement.expect("a statement").why_none_was_kept.expect("none kept");
        assert!(why.ends_with("is unchanged since before this attempt's launch"), "{why}");

        // **At once.** An attempt of another job launched with the same file
        // name and still running, its render process standing.
        let running = area.a_job(1);
        let _its_launch = the_launch_is_marked(
            &running,
            b"/assets/three/kitchen.blend",
            std::time::SystemTime::now(),
            the_crash_file_before_the_launch(b"/assets/three/kitchen.blend", &area.where_blender_writes()),
            RecordedPair {
                identifier: 4242,
                start_time: 7,
            },
            &AKernelThatAnswers,
        );
        // Make the overwrite distinguishable by size, without depending on
        // the interval between two file timestamps.
        std::fs::write(area.where_blender_writes().join("kitchen.crash.txt"), b"the earlier file").unwrap();
        let beside_it = area.a_job(1);
        let (_, statement) = an_attempt_that_ends(
            &area,
            &beside_it,
            "/assets/four/kitchen.blend",
            &a_stand_in_that_crashes("kitchen.crash.txt", 139),
            &AKernelThatAnswers,
        );
        let why = statement.expect("a statement").why_none_was_kept.expect("none kept");
        assert!(why.starts_with("another attempt on this machine had a file of the same name loaded"), "{why}");
        assert!(why.contains(&running.job.as_text()), "the attempt it may be is named: {why}");

        // **A file of another name is no reason**, with that mark still open.
        let another_name = area.a_job(1);
        let (_, statement) = an_attempt_that_ends(
            &area,
            &another_name,
            "/assets/garden.blend",
            &a_stand_in_that_crashes("garden.crash.txt", 139),
            &AKernelThatAnswers,
        );
        assert!(statement.expect("a statement").kept.is_some());

        // **The open mark's render process is gone** by this launch, so it was
        // not loaded while this attempt ran.
        std::fs::write(area.where_blender_writes().join("kitchen.crash.txt"), b"the earlier file").unwrap();
        let later = area.a_job(1);
        let (_, statement) = an_attempt_that_ends(
            &area,
            &later,
            "/assets/five/kitchen.blend",
            &a_stand_in_that_crashes("kitchen.crash.txt", 139),
            &AKernelThatKnowsNothing,
        );
        let statement = statement.expect("a statement");
        assert!(statement.kept.is_some(), "{statement:?}");
    }

    /// **A link at the name is never followed, and another login's file is
    /// not this attempt's** — the folder is shared by every login on the
    /// machine. Here the link points at a file the attempt's own login can
    /// read, which is the case the rule exists for.
    #[test]
    fn a_link_at_the_crash_files_name_is_never_followed() {
        let area = AnArea::made();
        let work = area.a_job(1);
        let elsewhere = area.root.join("something-private");
        std::fs::write(&elsewhere, b"not Blender's").unwrap();
        let script = format!(
            "#!/bin/sh\nln -s '{}' '{{tmp}}/kitchen.crash.txt'\nexit 139\n",
            elsewhere.display()
        );
        let (_, statement) = an_attempt_that_ends(&area, &work, "/assets/kitchen.blend", &script, &AKernelThatAnswers);
        let why = statement.expect("a statement").why_none_was_kept.expect("none kept");
        assert!(why.ends_with("is a link, and a link is never followed"), "{why}");
        assert!(!std::path::Path::new(&work.record_directory).join(the_kept_crash_files_name(1)).exists());
    }

    /// **A kept copy that no longer reads back whole is said to be so**, never
    /// served short and never read as none kept.
    #[test]
    fn a_kept_copy_that_does_not_read_back_whole_is_never_served_short() {
        let area = AnArea::made();
        let work = area.a_job(1);
        an_attempt_that_ends(
            &area,
            &work,
            "/assets/kitchen.blend",
            &a_stand_in_that_crashes("kitchen.crash.txt", 139),
            &AKernelThatAnswers,
        );
        std::fs::write(
            std::path::Path::new(&work.record_directory).join(the_kept_crash_files_name(1)),
            b"short",
        )
        .unwrap();
        let kept = the_kept_crash_files_in(&work.record_directory).unwrap();
        assert!(matches!(&kept[0].what, WhatWasKept::ItDoesNotReadBack(_)), "{kept:?}");
    }
}

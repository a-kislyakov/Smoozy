//! **The preview worker — `Rented-server runtime` W1's preview kind, *"one
//! job's still copies"*, one derivative at a time.** Added 2026-10-03, the
//! 38's piece 3.
//!
//! # What it is
//!
//! **`Job files and outputs` B22 to `Job files and outputs` B29**: one still
//! copy of one frame, made
//! once, on the machine, by the Blender the job's manifest names, headless and
//! needing no connection (`Job files and outputs` B23), and published into the job record directory
//! by a rename (`Job files and outputs` B29, `Job files and outputs` B30) — the
//! one set `Queue & History` fetches copies of and
//! `Telegram` reads. **The supervisor's side is beside the dispatch in
//! `main.rs`**: it queues the work on the render worker's report and at a
//! job's ending (`Queue & History` P1, `Queue & History` P1a, `Queue & History`
//! P1b) and starts this worker
//! through §14's one unit, one production at a time; this file is the child,
//! what it is told and what it reports.
//!
//! **What it does, in order.** It asks what it was started for. It reads the
//! reading in force for the job's attempt and chooses the frame's source and
//! treatment — `Job files and outputs — machine`'s
//! `derivatives::what_one_frame_is_made_of`, which is the candidate list, the
//! rungs, the picks, the route and the view layer's record, all built before
//! this file. Where nothing can be made it says why and starts nothing.
//! Otherwise it writes the script, starts the Blender by the build's own path,
//! **hears the source's size and tells it the target**
//! (`smoozy_job_files_machine::main_output::the_target_size`, `Job files and
//! outputs` B26a), waits for it, measures what it wrote against that clause's
//! byte cap, **makes it again smaller where it is over**
//! ([`the_lightest_of_its_writes`]), renames it into place,
//! and reports.
//!
//! # `Queue & History` §16's termination contract, kept
//!
//! 1. **The termination is delivered to the recorded external Blender process,
//!    at any moment** — and nothing in this build issues one: the preview kind
//!    holds no resource exclusively, so `Rented-server runtime` A6 never
//!    displaces it, and no request stops a production. **The one ending this
//!    worker gives its own Blender is
//!    [`HOW_LONG_ONE_PRODUCTION_IS_GIVEN`]'s**, a Build bound, so a Blender
//!    that never ends cannot hold every later production behind it. All the
//!    writes of one copy share it.
//! 2. **The worker outlives every member of its group**: it holds both streams
//!    to their end and reaps the Blender before it reports.
//! 3. **The cleanup owed is the sweep of the derivative's staging path**, so a
//!    production that did not finish never publishes a partial — the staged
//!    name is removed and nothing is renamed. A heavier write held aside
//!    ([`the_lightest_of_its_writes`]) sits under the same staging prefix.
//! 4. **A displaced operation's outcome is its own**, never a failure of the
//!    work that displaced it — moot while nothing displaces one.
//!
//! **Markers used in this file that belong to another document are written
//! with their document at every site**, as the rest of this program does.

use std::ffi::OsStr;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use smoozy_framing::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_framing::requests as asked;
use smoozy_job_files_machine::derivatives::{
    it_was_made, the_derivatives_script, the_size_in, the_size_told, the_words_of_a_derivative,
    what_one_frame_is_made_of, ADerivativeToMake, WhyNothingIsMade, THE_SCRIPTS_FILE_NAME,
};
use smoozy_job_files_machine::main_output::{
    the_size_to_write_it_at_next, the_target_size, where_this_copy_is_written_in, ATargetSize,
    ADerivativesRecord, WhereADerivativeIsWritten, THE_LARGEST_A_DERIVATIVE_MAY_BE_IN_BYTES,
    THE_MOST_WRITES_OF_ONE_COPY,
};
use smoozy_runtime::acting::capture::TheCopyIntoTheSinks;
use smoozy_runtime::acting::endpoint::TheEndpointAsACaller;
use smoozy_runtime::acting::workers::{
    IT_WAS_ACCEPTED, IT_WAS_REFUSED_AS_STALE, THE_RUN_THE_WORKER_WAS_STARTED_UNDER,
    THE_WORKERS_REPORT_REQUEST, WHETHER_THE_REPORT_WAS_ACCEPTED,
};
use smoozy_runtime::capture::{CaptureCeiling, CaptureRecord, Stream};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::the_file_inspection_worker::{copy_to_its_end, copy_to_its_end_watching, Copied};
use crate::the_render_worker::{
    THE_AVAILABLE_BYTES_FIELD, THE_COMPLAINTS_SINK_FIELD, THE_OPERATION_FIELD,
    THE_ORDINARY_SINK_FIELD, THE_WORKERS_OWN_PROCESS_FIELD, WHY_THERE_IS_NO_WORK_FIELD,
};

// ---------------------------------------------------------------------------
// The words the worker and the supervisor speak to each other
// ---------------------------------------------------------------------------

/// **The worker's first question: what was this preview worker started for.**
/// Neither end is the desktop, so the name is this program's own — the same
/// standing the render and file-inspection workers' first questions have.
pub const THE_PREVIEW_WORKERS_OWN_WORK_REQUEST: &str = "what the preview worker was started for";

/// The frame the production is of.
pub const THE_FRAME_FIELD: &str = "frame";
/// The job record directory the supervisor holds for the job (`Queue &
/// History` Q2's record), where the reading in force and the derivative sit.
pub const THE_RECORD_DIRECTORY_FIELD: &str = "record directory";
/// **Whether the job's ending was not *finished*** — `Job files and outputs`
/// `Job files and outputs` B20's second exclusion is worked out only then.
pub const THE_JOB_WAS_INTERRUPTED_FIELD: &str = "the job was interrupted";
/// Where the worker writes the production's script, beside its two sinks.
pub const THE_SCRIPT_FIELD: &str = "script";

/// **The production's ending, on `Rented-server runtime` W12's report** — one
/// document, [`TheProductionsEnding`], whose presence is what makes a report a
/// production's.
pub const THE_PRODUCTION_FIELD: &str = "the production";

/// **How long one production's Blender is given before the worker ends it.**
/// A Build bound and nothing of the plan's: `Job files and outputs` B24
/// measured a derivative at about 2.4 seconds with every core taken by a
/// render, and this is two hundred and fifty times that, so it bites only on
/// a Blender that will never end — which would otherwise hold every later
/// production on the machine behind it, one being made at a time. **It is one
/// bound for a copy, however many times `Job files and outputs` B26a writes
/// it**, so three writes hold later productions no longer than one could; the
/// copy already written stands if a later write runs into it.
pub const HOW_LONG_ONE_PRODUCTION_IS_GIVEN: Duration = Duration::from_secs(600);

/// How often the worker looks whether its Blender has ended.
const BETWEEN_TWO_LOOKS_AT_THE_BLENDER: Duration = Duration::from_millis(50);

// ---------------------------------------------------------------------------
// What the worker is told, and the kind's own field
// ---------------------------------------------------------------------------

/// **What the supervisor says this worker was started for.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePreviewsWork {
    pub operation: Identity,
    /// `Rented-server runtime` S2's run identifier, which the report carries
    /// (W12).
    pub run: Identity,
    pub job: Identity,
    pub frame: i64,
    /// The attempt whose recorded reading is the reading in force.
    pub attempt: u32,
    pub record_directory: String,
    pub the_job_was_interrupted: bool,
    pub script: String,
    /// `Rented-server runtime` M1's two sinks and M4a's ceiling.
    pub capture: CaptureRecord,
}

/// **What one production came to**, the kind's own outcome word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatTheProductionCameTo {
    /// The derivative was written and renamed into place.
    Published,
    /// **Nothing can be made of this frame** — a fact about the job's files or
    /// its reading, which would come out the same every time
    /// ([`WhyNothingIsMade`]). **Not a production that failed**, so `Queue &
    /// History` P1a's *queued again at the next report* does not reach it.
    NothingToMake,
    /// **The production failed** — the Blender did not write it, or the
    /// rename did not land. The frame is owed again at the next report and at
    /// the ending's write (`Queue & History` P1a, `Queue & History` P1b).
    NotMade,
}

impl WhatTheProductionCameTo {
    /// The kind's own word, written as the record's outcome.
    pub fn the_word(self) -> &'static str {
        match self {
            WhatTheProductionCameTo::Published => "published",
            WhatTheProductionCameTo::NothingToMake => "nothing to make",
            WhatTheProductionCameTo::NotMade => "not made",
        }
    }

    /// The outcome a record carries, read back.
    pub fn of_the_word(word: &str) -> Option<WhatTheProductionCameTo> {
        [
            WhatTheProductionCameTo::Published,
            WhatTheProductionCameTo::NothingToMake,
            WhatTheProductionCameTo::NotMade,
        ]
        .into_iter()
        .find(|one| one.the_word() == word)
    }
}

/// **What one production came to, as the worker reports it and the record
/// keeps it.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheProductionsEnding {
    pub came_to: WhatTheProductionCameTo,
    /// **`Job files and outputs` B31a's record**: the one named source file,
    /// the frame, and the view layer's name where one is established.
    pub the_record: Option<ADerivativesRecord>,
    /// **Where it was published** — the job record directory's
    /// `preview-<frame>.png` (`Job files and outputs` B29, `Job files and
    /// outputs` B30).
    pub published_at: Option<String>,
    /// Its length in bytes, read once it was written.
    pub bytes: Option<u64>,
    /// The source's size, as its Blender said it, and the size it was told.
    pub the_sources_size: Option<(u32, u32)>,
    pub the_size_it_was_written_at: Option<(u32, u32)>,
    /// **`Job files and outputs` B26a's last sentence**: *"the fact is
    /// recorded with it, so a consumer refuses knowingly"* — a ratio past
    /// twenty, or a file past the byte cap.
    pub it_could_not_be_brought_under_every_cap: bool,
    /// **`Rented-server runtime` M22's exit status, verbatim**, and the
    /// signal where one ended it.
    pub exit_status: Option<i32>,
    pub signal: Option<i32>,
    /// The machine's own words, where nothing was published.
    pub words: Option<String>,
    /// **The frame's other View Layers' copies, each ending on its own** —
    /// `Queue & History` P1's *owed* read per copy. Empty where the frame has
    /// one copy, and on every ending written before 2026-10-10 (the Creator's
    /// seventh notes, note 4). This ending's own fields are the main copy's.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub the_other_layers: Vec<TheProductionsEnding>,
}

impl TheProductionsEnding {
    /// **What the frame's production came to, every copy counted** — the
    /// record's outcome, which is what `Queue & History` P1a's *published* reads. **A frame is
    /// published only where every copy it owed is**: the main copy published
    /// and each other layer's published or with nothing to make. A layer copy
    /// that failed leaves the frame *not made*, so it is owed again at the next
    /// report, exactly as a missing frame is.
    pub fn the_frames_outcome(&self) -> WhatTheProductionCameTo {
        if self.came_to == WhatTheProductionCameTo::Published
            && self
                .the_other_layers
                .iter()
                .any(|other| other.came_to == WhatTheProductionCameTo::NotMade)
        {
            return WhatTheProductionCameTo::NotMade;
        }
        self.came_to
    }

    /// **Every copy this production published, the main one first** — what is
    /// journalled, one event each (`Queue & History` P3).
    pub fn every_copy_published(&self) -> Vec<&TheProductionsEnding> {
        std::iter::once(self)
            .chain(self.the_other_layers.iter())
            .filter(|one| one.came_to == WhatTheProductionCameTo::Published)
            .collect()
    }
}

impl TheProductionsEnding {
    fn nothing_to_make(why: &WhyNothingIsMade, the_record: Option<ADerivativesRecord>) -> Self {
        TheProductionsEnding {
            came_to: WhatTheProductionCameTo::NothingToMake,
            the_record,
            published_at: None,
            bytes: None,
            the_sources_size: None,
            the_size_it_was_written_at: None,
            it_could_not_be_brought_under_every_cap: false,
            exit_status: None,
            signal: None,
            words: Some(why.in_words()),
            the_other_layers: Vec::new(),
        }
    }

    fn not_made(words: String, the_record: Option<ADerivativesRecord>) -> Self {
        TheProductionsEnding {
            came_to: WhatTheProductionCameTo::NotMade,
            words: Some(words),
            ..Self::nothing_to_make(&WhyNothingIsMade::ThereIsNoCandidate, the_record)
        }
    }
}

/// **`Rented-server runtime` W4: the preview kind's own field of the
/// operation record**, which `Job files and outputs` shapes (W1 gives it the
/// kind's declarations). **It names both sinks before the worker starts**
/// (M1), and the ending is written once, from the worker's report, with the
/// record's outcome in the same write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThePreviewKindsOwnField {
    pub job: String,
    pub frame: i64,
    pub attempt: u32,
    pub record_directory: String,
    pub the_job_was_interrupted: bool,
    pub ordinary_sink: String,
    pub complaints_sink: String,
    pub script: String,
    pub ending: Option<TheProductionsEnding>,
}

impl ThePreviewKindsOwnField {
    pub fn as_written(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("the field renders")
    }

    pub fn read_back(bytes: &[u8]) -> Option<ThePreviewKindsOwnField> {
        serde_json::from_slice(bytes).ok()
    }
}

/// **The work, as the supervisor answers the worker's first question.**
pub fn the_work_as_fields(work: &ThePreviewsWork, available_bytes: u64) -> Fields {
    Fields::none()
        .and(THE_OPERATION_FIELD, work.operation.as_text())
        .and(THE_RUN_THE_WORKER_WAS_STARTED_UNDER, work.run.as_text())
        .and(asked::THE_JOB_FIELD, work.job.as_text())
        .and(THE_FRAME_FIELD, work.frame.to_string())
        .and(asked::THE_ATTEMPT_FIELD, work.attempt.to_string())
        .and(THE_RECORD_DIRECTORY_FIELD, work.record_directory.as_str())
        .and(
            THE_JOB_WAS_INTERRUPTED_FIELD,
            if work.the_job_was_interrupted {
                asked::THE_ANSWER_IS_YES
            } else {
                asked::THE_ANSWER_IS_NO
            },
        )
        .and(THE_SCRIPT_FIELD, work.script.as_str())
        .and(THE_ORDINARY_SINK_FIELD, work.capture.ordinary_sink.as_str())
        .and(THE_COMPLAINTS_SINK_FIELD, work.capture.complaints_sink.as_str())
        .and(THE_AVAILABLE_BYTES_FIELD, available_bytes.to_string())
}

/// The work, read back by the worker.
pub fn the_work_in(answered: &Answered) -> Result<ThePreviewsWork, String> {
    let fields = &answered.fields;
    if let Some(why) = fields.text(WHY_THERE_IS_NO_WORK_FIELD) {
        return Err(format!("smoozy-server: the preview worker has no work: {why}"));
    }
    let text = |name: &str| {
        fields
            .text(name)
            .ok_or_else(|| format!("smoozy-server: the preview worker was not told {name}"))
    };
    let number = |name: &str| {
        text(name)?
            .parse::<i64>()
            .map_err(|_| format!("smoozy-server: the preview worker's {name} did not read back"))
    };
    let operation = Identity::read_back(IdentityKind::Operation, text(THE_OPERATION_FIELD)?)
        .ok_or("smoozy-server: the operation identity did not read back")?;
    let available: u64 = text(THE_AVAILABLE_BYTES_FIELD)?
        .parse()
        .map_err(|_| "smoozy-server: the available figure did not read back".to_string())?;
    Ok(ThePreviewsWork {
        capture: CaptureRecord {
            job: operation.clone(),
            attempt: 1,
            ordinary_sink: text(THE_ORDINARY_SINK_FIELD)?.to_string(),
            complaints_sink: text(THE_COMPLAINTS_SINK_FIELD)?.to_string(),
            ceiling: CaptureCeiling::from(available),
        },
        operation,
        run: Identity::read_back(IdentityKind::Run, text(THE_RUN_THE_WORKER_WAS_STARTED_UNDER)?)
            .ok_or("smoozy-server: the run identifier did not read back")?,
        job: Identity::read_back(IdentityKind::Job, text(asked::THE_JOB_FIELD)?)
            .ok_or("smoozy-server: the job identity did not read back")?,
        frame: number(THE_FRAME_FIELD)?,
        attempt: u32::try_from(number(asked::THE_ATTEMPT_FIELD)?)
            .map_err(|_| "smoozy-server: the attempt did not read back".to_string())?,
        record_directory: text(THE_RECORD_DIRECTORY_FIELD)?.to_string(),
        the_job_was_interrupted: text(THE_JOB_WAS_INTERRUPTED_FIELD)? == asked::THE_ANSWER_IS_YES,
        script: text(THE_SCRIPT_FIELD)?.to_string(),
    })
}

/// **The ending a report carries, where it is a production's**, read back by
/// the supervisor. `None` where the report carries none.
pub fn the_ending_in(fields: &Fields) -> Option<TheProductionsEnding> {
    serde_json::from_str(fields.text(THE_PRODUCTION_FIELD)?).ok()
}

// ---------------------------------------------------------------------------
// The worker, whole
// ---------------------------------------------------------------------------

/// **The preview worker's own run**: ask, make, report.
///
/// **A worker that gets no answer does nothing**, and says so naming the
/// document `Rented-server runtime` W1 gives this kind's declarations to; its
/// operation record is reconciliation's.
pub fn the_preview_worker(recorded_endpoint_path: &str) -> Result<(), String> {
    let supervisor = TheEndpointAsACaller::at(
        recorded_endpoint_path,
        crate::the_render_worker::HOW_LONG_ONE_EXCHANGE_MAY_TAKE,
    );
    let asked_for_work = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        Identity::issue(IdentityKind::Operation)
            .map_err(|_| "smoozy-server: the system would not supply random bytes".to_string())?,
        THE_PREVIEW_WORKERS_OWN_WORK_REQUEST,
    )
    .carrying(Fields::none().and(THE_WORKERS_OWN_PROCESS_FIELD, std::process::id().to_string()));
    let answered = supervisor
        .ask_waiting_for_a_supervisor(
            &asked_for_work,
            crate::the_render_worker::HOW_LONG_THE_WORKER_LOOKS_FOR_ITS_SUPERVISOR,
        )
        .map_err(|why| format!("smoozy-server: the preview worker could not ask: {why}"))?
        .ok_or_else(|| {
            "smoozy-server: the preview worker (`Job files and outputs`' kind) asked what it was \
             started for and no supervisor answered, so it does nothing; its operation record is \
             reconciliation's"
                .to_string()
        })?;
    let work = the_work_in(&answered)?;
    let ending = one_production(&work);
    report_the_ending(&supervisor, &work, &ending)
}

/// **`Rented-server runtime` W12: re-attempted until it is accepted**, at the
/// render worker's pace. A supervisor that refuses it as stale has answered.
fn report_the_ending(
    supervisor: &TheEndpointAsACaller,
    work: &ThePreviewsWork,
    ending: &TheProductionsEnding,
) -> Result<(), String> {
    let request = Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        work.operation.clone(),
        THE_WORKERS_REPORT_REQUEST,
    )
    .carrying(
        Fields::none()
            .and(THE_RUN_THE_WORKER_WAS_STARTED_UNDER, work.run.as_text())
            .and(
                THE_PRODUCTION_FIELD,
                serde_json::to_string(ending).expect("the ending renders"),
            ),
    );
    loop {
        if let Ok(Some(answered)) = supervisor.ask(&request) {
            match answered.fields.text(WHETHER_THE_REPORT_WAS_ACCEPTED) {
                Some(IT_WAS_ACCEPTED) => return Ok(()),
                Some(IT_WAS_REFUSED_AS_STALE) => {
                    return Err(
                        "smoozy-server: the preview worker's ending was refused as stale".to_string()
                    )
                }
                _ => {
                    if let Some(why) = answered.fields.text(asked::THE_MACHINES_OWN_WORDS_FIELD) {
                        eprintln!(
                            "smoozy-server: the preview worker's ending was not written yet: {why}"
                        );
                    }
                }
            }
        }
        std::thread::sleep(crate::the_render_worker::BETWEEN_TWO_ATTEMPTS_AT_THE_REPORT);
    }
}

/// **The size the Blender said, and what the worker told it**, held while
/// its standard output passes.
#[derive(Debug, Default)]
struct TheHandshake {
    /// The bytes not yet matched against a mark — at most a mark's length
    /// past the newest chunk, so a mark split across two chunks is still
    /// heard.
    held: Vec<u8>,
    the_sources_size: Option<(u32, u32)>,
    told: Option<(u32, u32)>,
    made: bool,
}

/// **One production, from the reading to the ending the worker reports.**
pub fn one_production(work: &ThePreviewsWork) -> TheProductionsEnding {
    match what_this_frame_is_made_of(work) {
        // **`Job files and outputs` B23: the Blender the manifest names**, by
        // the build's own path.
        Ok((build, made_of)) => the_derivative_is_made(work, &build, &made_of),
        Err(nothing) => nothing,
    }
}

/// **The production's choosing, before any Blender starts**: the reading in
/// force for the work's attempt, and the frame's copies chosen from it — with
/// the build the manifest names — or the ending that says why nothing is made.
///
/// **The frame's View Layers are read with the job's pins and in the scene's
/// own order** (`Job files and outputs` R4 item 5): the scene's list rides in
/// the reading in force, so a layer the job pins on that the file switched
/// off has its copy at its place in the scene, and the preview's dropdown
/// lists it there (`Queue & History` P1, P4). Split out of
/// [`one_production`] on 2026-10-18, piece P8b of step 10c, so the choosing
/// is proved through the same path without a Blender.
pub fn what_this_frame_is_made_of(
    work: &ThePreviewsWork,
) -> Result<(String, smoozy_job_files_machine::derivatives::WhatOneFrameIsMadeOf), TheProductionsEnding> {
    let in_force = crate::the_render_worker::the_reading_in_force_for_the_job_in(
        &work.record_directory,
        work.attempt,
    )
    .map_err(|why| TheProductionsEnding::nothing_to_make(&WhyNothingIsMade::NoReadingStands(why), None))?;
    // **The one look at the disk a derivative's choosing takes** (`Job files
    // and outputs` B16): a predicted path's size, where a file is there.
    let size_of = |path: &[u8]| {
        std::fs::metadata(OsStr::from_bytes(path))
            .ok()
            .filter(|metadata| metadata.is_file())
            .map(|metadata| metadata.len())
    };
    let made_of = what_one_frame_is_made_of(
        work.frame,
        &in_force.outputs,
        &in_force.inventory,
        &in_force.picks,
        &in_force.the_jobs_pins,
        &in_force.the_scenes_view_layers_in_order,
        &in_force.whole_range,
        work.the_job_was_interrupted,
        &size_of,
    )
    .map_err(|why| TheProductionsEnding::nothing_to_make(&why, None))?;
    Ok((in_force.manifest.blender.build_path, made_of))
}

/// **The derivative of one frame made, once its source and treatment are
/// chosen**: the script written, the Blender started by the build's own path
/// and told the size, its streams held to their end and it reaped, the copy
/// measured and renamed into place — or why not.
///
/// **One copy per View Layer** (`Queue & History` P1's new bullet, the
/// Creator's seventh notes, note 4): the main copy first, at the frame's own
/// name; then, only once it is published, each other layer's copy in the
/// frame's set's order, each its own Blender start by `Job files and outputs` B26's same four steps
/// with that layer's name in step 1, written at its place
/// ([`where_this_copy_is_written_in`]) and ending on its own
/// ([`TheProductionsEnding::the_other_layers`]).
pub fn the_derivative_is_made(
    work: &ThePreviewsWork,
    build: &str,
    made_of: &smoozy_job_files_machine::derivatives::WhatOneFrameIsMadeOf,
) -> TheProductionsEnding {
    let mut main = one_copy_is_made(
        work,
        build,
        &made_of.treatment,
        made_of.the_picture_inside.as_ref(),
        made_of.the_composited_picture.as_ref(),
        &made_of.record,
    );
    if main.came_to != WhatTheProductionCameTo::Published {
        return main;
    }
    // **Each other layer's copy is made of its own record's source** — the
    // main copy's file on the multi-layer route, or the file of a destination
    // that layer alone feeds (`Job files and outputs` B31a's last bullet, the
    // Creator's ninth notes, note 2, built 2026-10-16) — in its own treatment.
    for other in &made_of.the_other_layers {
        let ending = one_copy_is_made(
            work,
            build,
            &other.treatment,
            other.the_picture_inside.as_ref(),
            None,
            &other.record,
        );
        main.the_other_layers.push(ending);
    }
    main
}

/// **One copy of the frame**, the main one or one other layer's: the script
/// written, the copy written and measured, made again smaller where
/// `Job files and outputs` B26a's byte cap asks it, and renamed into place —
/// or why not. **The file it is made of is its record's own** (`Job files and
/// outputs` B31).
fn one_copy_is_made(
    work: &ThePreviewsWork,
    build: &str,
    treatment: &smoozy_job_files_machine::derivatives::TheTreatment,
    the_picture_inside: Option<&smoozy_job_files_machine::reading::ArtistsText>,
    the_composited_picture: Option<&smoozy_job_files_machine::reading::ArtistsText>,
    the_record: &ADerivativesRecord,
) -> TheProductionsEnding {
    let record = Some(the_record.clone());
    let at = where_this_copy_is_written_in(
        &work.record_directory,
        work.frame,
        the_record.it_is_the_main_copy,
        the_record.its_place_in_the_frames_set,
    );
    let script = the_derivatives_script(&ADerivativeToMake {
        source: the_record.the_source_file.clone(),
        staged_at: at.staged_at.clone(),
        treatment: treatment.clone(),
        the_picture_inside: the_picture_inside.map(|picture| picture.as_bytes().to_vec()),
        the_composited_picture: the_composited_picture.map(|picture| picture.as_bytes().to_vec()),
    });
    if let Err(why) = std::fs::write(&work.script, script) {
        return TheProductionsEnding::not_made(format!("{}: {why}", work.script), record);
    }
    let began = Instant::now();
    let first = match one_write(work, build, &at, None, began, record.clone()) {
        Ok(first) => first,
        Err(ending) => return ending,
    };
    let written = the_lightest_of_its_writes(work, build, &at, first, began);
    let the_written = |mut ending: TheProductionsEnding| {
        ending.the_sources_size = written.the_sources_size;
        ending.the_size_it_was_written_at = Some(written.at);
        ending.exit_status = written.exit_status;
        ending.signal = written.signal;
        ending
    };
    // **`Job files and outputs` B29: published by a rename**, which is the one
    // unit in that document that publishes over an existing name.
    if let Err(why) = std::fs::rename(&at.staged_at, &at.published_at) {
        std::fs::remove_file(&at.staged_at).ok();
        return the_written(TheProductionsEnding::not_made(
            format!("{} could not be renamed into place: {why}", at.staged_at),
            record,
        ));
    }
    if let Err(why) =
        smoozy_runtime::acting::durable::flush_the_directory_holding(std::path::Path::new(&at.published_at))
    {
        eprintln!(
            "smoozy-server: a derivative was published and its directory was not flushed: {}",
            why.as_text()
        );
    }
    let ratio_past_the_cap = written
        .the_sources_size
        .is_some_and(|(width, height)| the_target_size(width, height).it_could_not_be_brought_under_every_cap);
    the_written(TheProductionsEnding {
        came_to: WhatTheProductionCameTo::Published,
        the_record: record,
        published_at: Some(at.published_at.clone()),
        bytes: Some(written.bytes),
        the_sources_size: None,
        the_size_it_was_written_at: None,
        it_could_not_be_brought_under_every_cap: ratio_past_the_cap
            || written.bytes > THE_LARGEST_A_DERIVATIVE_MAY_BE_IN_BYTES,
        exit_status: None,
        signal: None,
        words: None,
        the_other_layers: Vec::new(),
    })
}

/// **What one write left at the staged name.**
#[derive(Debug, Clone, Copy)]
struct AWrite {
    bytes: u64,
    /// The size it was told, which is the size it was written at.
    at: (u32, u32),
    the_sources_size: Option<(u32, u32)>,
    exit_status: Option<i32>,
    signal: Option<i32>,
}

/// **`Job files and outputs` B26a, *how the production comes under the byte
/// cap*.** A write at or under the cap is kept as it is and nothing more is
/// started. **Over it, the same script is started again and told a smaller
/// size**, so the copy is made from the source again and not from the heavier
/// copy. That heavier copy is held aside under the staging prefix meanwhile, and
/// comes back to the staged name where a later write fails or weighs no less:
/// **nothing is published until the last write has ended**, so no consumer
/// meets a copy about to be replaced, and a frame keeps the copy it has. What
/// comes back is at the staged name; whether it is under the cap is the caller's
/// to record.
fn the_lightest_of_its_writes(
    work: &ThePreviewsWork,
    build: &str,
    at: &WhereADerivativeIsWritten,
    first: AWrite,
    began: Instant,
) -> AWrite {
    let held_at = format!("{}.heavier", at.staged_at);
    let mut lightest = first;
    let mut writes = 1;
    while writes < THE_MOST_WRITES_OF_ONE_COPY {
        let Some(smaller) = the_size_to_write_it_at_next(lightest.at, lightest.bytes) else {
            break;
        };
        if std::fs::rename(&at.staged_at, &held_at).is_err() {
            break;
        }
        writes += 1;
        eprintln!(
            "smoozy-server: a derivative of frame {} weighed {} bytes at {} by {} and is made again at {} by {}",
            work.frame, lightest.bytes, lightest.at.0, lightest.at.1, smaller.width, smaller.height
        );
        let lighter = match one_write(work, build, at, Some(smaller), began, None) {
            Ok(write) if write.bytes < lightest.bytes => Some(write),
            Ok(_) => None,
            Err(ending) => {
                eprintln!(
                    "smoozy-server: a derivative of frame {} was not made again: {}",
                    work.frame,
                    ending.words.unwrap_or_default()
                );
                None
            }
        };
        match lighter {
            Some(write) => {
                std::fs::remove_file(&held_at).ok();
                lightest = write;
            }
            None => {
                // A rename over the name, whatever the later write left there.
                std::fs::rename(&held_at, &at.staged_at).ok();
                break;
            }
        }
    }
    lightest
}

/// **One start of the Blender for one copy**: started by the build's own path and
/// told the size — `told`, or `Job files and outputs` B26a's target where this is
/// the copy's first write — its streams held to their end and it reaped, and what
/// it left at the staged name measured. **`began` is when the copy's first write
/// started**: [`HOW_LONG_ONE_PRODUCTION_IS_GIVEN`] is one bound for them all. The
/// ending it gives back where nothing was left is the copy's, with the staged
/// name swept.
fn one_write(
    work: &ThePreviewsWork,
    build: &str,
    at: &WhereADerivativeIsWritten,
    told: Option<ATargetSize>,
    began: Instant,
    record: Option<ADerivativesRecord>,
) -> Result<AWrite, TheProductionsEnding> {
    let sinks = match TheCopyIntoTheSinks::opening(&work.capture) {
        Ok(sinks) => Arc::new(sinks),
        Err(why) => {
            return Err(TheProductionsEnding::not_made(
                format!("the two sinks could not be opened: {why}"),
                record,
            ))
        }
    };
    // Handed to the kernel as the program to run, never through a shell.
    let mut started = match Command::new(build)
        .args(the_words_of_a_derivative(&work.script).iter().map(|word| OsStr::from_bytes(word)))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(started) => started,
        Err(why) => {
            return Err(TheProductionsEnding::not_made(
                format!("the production could not be started: {build} could not be run: {why}"),
                record,
            ))
        }
    };

    let ceiling = Arc::new(Mutex::new(work.capture.ceiling));
    let handshake = Arc::new(Mutex::new(TheHandshake::default()));
    let mut its_input = started.stdin.take();
    let ordinary = started.stdout.take().map(|out| {
        let sinks = Arc::clone(&sinks);
        let ceiling = Arc::clone(&ceiling);
        let handshake = Arc::clone(&handshake);
        std::thread::spawn(move || {
            copy_to_its_end_watching(
                Stream::Ordinary,
                Box::new(out),
                sinks.as_ref(),
                &ceiling,
                &mut |chunk| {
                    let mut heard = handshake.lock().expect("the handshake is not poisoned");
                    heard.held.extend_from_slice(chunk);
                    if heard.the_sources_size.is_none() {
                        if let Some(size) = the_size_in(&heard.held) {
                            heard.the_sources_size = Some(size);
                            // **`Job files and outputs` B26a's target is
                            // that crate's arithmetic**, told
                            // back on the Blender's standard input. A source
                            // with no pixels is told nothing, and its script
                            // ends at the failure status.
                            if size.0 > 0 && size.1 > 0 {
                                let target = told.unwrap_or_else(|| the_target_size(size.0, size.1));
                                if let Some(input) = its_input.as_mut() {
                                    if input.write_all(the_size_told(target).as_bytes()).is_ok()
                                        && input.flush().is_ok()
                                    {
                                        heard.told = Some((target.width, target.height));
                                    }
                                }
                            }
                            its_input = None;
                        }
                    }
                    if it_was_made(&heard.held) {
                        heard.made = true;
                    }
                    let keep = 128.min(heard.held.len());
                    let from = heard.held.len() - keep;
                    heard.held.drain(..from);
                },
            )
        })
    });
    let complaints = started.stderr.take().map(|err| {
        let sinks = Arc::clone(&sinks);
        let ceiling = Arc::clone(&ceiling);
        std::thread::spawn(move || {
            copy_to_its_end(Stream::Complaints, Box::new(err), sinks.as_ref(), &ceiling)
        })
    });

    // **Reaped, so the production leaves no process behind**, within
    // [`HOW_LONG_ONE_PRODUCTION_IS_GIVEN`].
    let mut given_up = false;
    let ended = loop {
        match started.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if began.elapsed() >= HOW_LONG_ONE_PRODUCTION_IS_GIVEN => {
                given_up = true;
                started.kill().ok();
                break started.wait().ok();
            }
            Ok(None) => std::thread::sleep(BETWEEN_TWO_LOOKS_AT_THE_BLENDER),
            Err(_) => break started.wait().ok(),
        }
    };
    let joined = |one: Option<std::thread::JoinHandle<Copied>>| {
        one.and_then(|handle| handle.join().ok()).unwrap_or_default()
    };
    let _ = joined(ordinary);
    let _ = joined(complaints);
    let heard = handshake.lock().expect("the handshake is not poisoned");
    let exit_status = ended.and_then(|status| status.code());
    let signal = ended.and_then(|status| status.signal());

    let sweep = || {
        std::fs::remove_file(&at.staged_at).ok();
    };
    let the_written = |mut ending: TheProductionsEnding| {
        ending.the_sources_size = heard.the_sources_size;
        ending.the_size_it_was_written_at = heard.told;
        ending.exit_status = exit_status;
        ending.signal = signal;
        ending
    };
    if heard.the_sources_size == Some((0, 0)) || heard.the_sources_size.is_some_and(|size| size.0 == 0 || size.1 == 0) {
        sweep();
        return Err(the_written(TheProductionsEnding::nothing_to_make(
            &WhyNothingIsMade::TheSourceHasNoPixels,
            record,
        )));
    }
    if given_up {
        sweep();
        return Err(the_written(TheProductionsEnding::not_made(
            format!(
                "the production's Blender had not ended after {} seconds and was ended",
                HOW_LONG_ONE_PRODUCTION_IS_GIVEN.as_secs()
            ),
            record,
        )));
    }
    let staged_length = std::fs::metadata(&at.staged_at)
        .ok()
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len());
    let ran_to_its_end = exit_status == Some(0) && heard.made;
    let (Some(bytes), Some(written_at)) = (staged_length.filter(|_| ran_to_its_end), heard.told) else {
        sweep();
        return Err(the_written(TheProductionsEnding::not_made(
            format!(
                "the production's Blender ended at {} and {}",
                match (exit_status, signal) {
                    (Some(code), _) => format!("status {code}"),
                    (None, Some(signal)) => format!("signal {signal}"),
                    (None, None) => "no status that could be read".to_string(),
                },
                if heard.the_sources_size.is_none() {
                    "never said the source's size"
                } else if !heard.made {
                    "never said it had written the copy"
                } else {
                    "left no copy at the staged name"
                }
            ),
            record,
        )));
    };
    Ok(AWrite {
        bytes,
        at: written_at,
        the_sources_size: heard.the_sources_size,
        exit_status,
        signal,
    })
}

/// The script's own path, for a production's operation folder.
pub fn the_scripts_path_in(folder: &str) -> String {
    format!("{folder}/{THE_SCRIPTS_FILE_NAME}")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **Piece P8b of step 10c, through the preview worker's own choosing**: a
    /// five-layer job whose third layer, *Kitchen*, the file switches off and
    /// the job pins on. The record directory holds the manifest, carrying the
    /// pin in its settings payload, and attempt 1's recorded reading, composed
    /// by `Job files and outputs`' one composition: the outputs half lists the
    /// four layers the file renders, the settings half the scene's five in
    /// Blender's order. One multi-layer file sits at frame 1's predicted path.
    ///
    /// **The frame's copies are published in the scene's order, 1, 2, 3, 4,
    /// 5**: the main copy the first layer's at place nought, then the other
    /// four at places one to four, *Kitchen* third. Nothing else moves. Until
    /// P8b the preview path handed no order down, and *Kitchen* was last.
    #[test]
    fn p8b_a_layer_pinned_on_is_published_at_its_place_in_the_scenes_order() {
        use smoozy_job_files_machine::composing::{
            the_reading_and_everything_derived_from, WhatTheReadingIsComposedAgainst,
        };
        use smoozy_job_files_machine::main_output::WhereTheRenderEnters;
        use smoozy_job_files_machine::reading::{
            ArtistsText, AtOneFrame, Destination, DestinationKind, Format, GatingConditions,
            HowManyViewsTheSceneWrites, RecordedLocation, TheVersionInHand, WhereItComesFrom,
            WhichScene,
        };
        use smoozy_job_files_machine::reading_script::{TheObservedOutputs, ThePrintedReading};
        use smoozy_render_settings_machine::names::{AnchorState, ExactName};
        use smoozy_render_settings_machine::reading::{SwitchAnimation, ViewLayerReading};
        use smoozy_render_settings_machine::view_layers::PinEntry;
        use smoozy_rendering_machine::material::testing::{a_reading, a_scene, an_untouched_payload};

        // **A folder that goes with the test, in the build folder's own
        // temporary area**, as the supervisor's tests keep theirs.
        struct GoesWithTheTest(std::path::PathBuf);
        impl Drop for GoesWithTheTest {
            fn drop(&mut self) {
                std::fs::remove_dir_all(&self.0).ok();
            }
        }
        let at = GoesWithTheTest(
            std::env::current_exe()
                .expect("this test knows where it is")
                .ancestors()
                .nth(3)
                .expect("a test program sits three folders inside the build folder")
                .join("tmp")
                .join(format!(
                    "smoozy-p8b-{}",
                    Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
                )),
        );
        let records = at.0.join("record");
        let outputs = at.0.join("out");
        std::fs::create_dir_all(&records).expect("the record directory");
        std::fs::create_dir_all(&outputs).expect("the job output folder");

        let the_scene = ["Mid", "Aardvark", "Kitchen", "Alpha", "Beta"];
        let the_files_own = ["Mid", "Aardvark", "Alpha", "Beta"];
        let layer = |name: &str| ArtistsText::of(name.as_bytes().to_vec());

        // **The manifest, the pin in its payload.**
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let mut manifest = smoozy_rendering_machine::composition::testing::a_manifest(&job, "5.2.0", "Scene");
        let mut payload = an_untouched_payload();
        payload.view_layer_pins.push(PinEntry {
            layer: ExactName::as_the_reading_produced_it("Kitchen"),
            anchor: AnchorState::Anchored {
                the_reading_it_came_from: a_reading().identity(),
            },
            on: true,
        });
        manifest.settings_payload = payload.as_carried();
        std::fs::write(
            records.join(smoozy_job_files_machine::manifest::MANIFEST_FILE_NAME),
            manifest.as_written(),
        )
        .expect("the manifest is written");

        // **Attempt 1's recorded reading**, through the one composition.
        let gating = GatingConditions {
            the_effects_on_for_render: None,
            compositing_switch: true,
            a_compositing_tree_is_present: false,
            a_live_final_output_node: false,
            the_final_output_node_is_muted: false,
            four_fives_use_nodes_switch: None,
            save_output_switch: None,
            the_render_goes_out_through_a_sequencer_holding_a_scene_strip: false,
        };
        let mut scene = a_scene("Scene");
        scene.view_layers = the_scene
            .iter()
            .map(|name| {
                let on = *name != "Kitchen";
                ViewLayerReading {
                    name: ExactName::as_the_reading_produced_it(name),
                    renders_at: (1..=10).map(|frame| (frame, on)).collect(),
                    animation: SwitchAnimation::None { value: on },
                    own_sample_count: 0,
                    overrides_the_scenes_count: false,
                }
            })
            .collect();
        let mut settings = a_reading().settings;
        settings.scenes = vec![scene];
        let printed = ThePrintedReading {
            outputs: TheObservedOutputs {
                taken_by: TheVersionInHand::of(5, 2),
                gating: gating.clone(),
                destinations: vec![Destination {
                    kind: DestinationKind::TheRegularOutputPath,
                    recorded_location: Some(RecordedLocation::of(b"//out/f_".to_vec())),
                    format: Some(Format {
                        as_the_file_records_it: ArtistsText::of(b"OPEN_EXR_MULTILAYER".to_vec()),
                        the_ending_it_writes: Some(ArtistsText::of(b".exr".to_vec())),
                        is_video_shaped: false,
                    }),
                    origin: WhereItComesFrom::TheScenesOutputPath,
                    scene: WhichScene::TheJobsOwnScene,
                    the_view_layers_that_feed_it: the_files_own.iter().map(|name| layer(name)).collect(),
                    the_location_with_its_folder_replaced: None,
                }],
                scenes_reached_through_a_sequencer_strip: Vec::new(),
                the_tree_is_shared_with: None,
                per_frame: (1..=3)
                    .map(|frame| AtOneFrame {
                        frame,
                        view_layers_that_render: the_files_own.iter().map(|name| layer(name)).collect(),
                        gating: gating.clone(),
                        the_regular_output_paths_answered_path: Some(RecordedLocation::of(
                            format!("/artist/out/f_{frame:04}.exr").into_bytes(),
                        )),
                    })
                    .collect(),
                recorded_locations: Vec::new(),
                views: HowManyViewsTheSceneWrites::One,
                the_compositing_tree: None,
                the_view_transform: Some(layer("AgX")),
            },
            settings: smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere(
                serde_json::to_value(&settings).expect("the settings half renders"),
            ),
            dependencies: smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere(
                serde_json::json!({}),
            ),
            errors: std::collections::BTreeMap::new(),
            the_baked_caches_on_the_machine: Vec::new(),
            the_scene_holds_a_simulation_not_baked: false,
            the_scene_it_read: None,
            the_open_files_path: None,
            the_check_is_written_as_a_picture: false,
        };
        let job_output_folder = outputs.display().to_string();
        let pins = crate::the_render_worker::the_pins_the_render_applies(&manifest.settings_payload);
        assert_eq!(pins.len(), 1, "the pin reads back");
        let composed = the_reading_and_everything_derived_from(
            printed,
            WhatTheReadingIsComposedAgainst {
                the_folder_the_blend_sits_in: b"/artist",
                job_output_folder: &job_output_folder,
                enters_at: &WhereTheRenderEnters { scene: layer("Scene"), view_layer: layer("Mid") },
                the_jobs_pins: &pins,
            },
        )
        .expect("the reading composes");
        std::fs::write(
            records.join(smoozy_job_files_machine::manifest::the_recorded_readings_file_name(1)),
            serde_json::to_vec(&composed).expect("the reading renders"),
        )
        .expect("the recorded reading is written");

        // **Frame 1's one multi-layer file, where the inventory predicts it.**
        let smoozy_job_files_machine::inventory::WhatTheInventoryIs::It(inventory) = &composed.inventory else {
            panic!("an inventory is composed");
        };
        let at_frame_one = inventory
            .entries
            .iter()
            .find(|entry| entry.frame == Some(1))
            .expect("frame 1 is owed a file");
        let file = std::path::PathBuf::from(OsStr::from_bytes(&at_frame_one.predicted_path));
        std::fs::create_dir_all(file.parent().expect("a folder")).expect("its folder");
        std::fs::write(&file, b"a multi-layer frame").expect("the frame is there");

        let streams = at.0.join("streams");
        let work = ThePreviewsWork {
            operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            job,
            frame: 1,
            attempt: 1,
            record_directory: records.display().to_string(),
            the_job_was_interrupted: false,
            script: streams.join("derivative.py").display().to_string(),
            capture: CaptureRecord {
                job: Identity::issue(IdentityKind::Job).expect("randomness"),
                attempt: 1,
                ordinary_sink: streams.join("ordinary").display().to_string(),
                complaints_sink: streams.join("complaints").display().to_string(),
                ceiling: CaptureCeiling::from(1 << 30),
            },
        };
        let (build, made_of) = what_this_frame_is_made_of(&work).expect("the frame's copies are chosen");
        assert_eq!(build, manifest.blender.build_path);
        assert_eq!(made_of.the_picture_inside, Some(layer("Mid.Combined")), "the multi-layer route");
        let published: Vec<(String, u32, bool)> = std::iter::once(&made_of.record)
            .chain(made_of.the_other_layers.iter().map(|other| &other.record))
            .map(|record| {
                (
                    String::from_utf8_lossy(
                        record.the_view_layer_its_picture_came_from.as_ref().expect("named").as_bytes(),
                    )
                    .to_string(),
                    record.its_place_in_the_frames_set,
                    record.it_is_the_main_copy,
                )
            })
            .collect();
        assert_eq!(
            published,
            vec![
                ("Mid".to_string(), 0, true),
                ("Aardvark".to_string(), 1, false),
                ("Kitchen".to_string(), 2, false),
                ("Alpha".to_string(), 3, false),
                ("Beta".to_string(), 4, false),
            ],
            "the frame's copies in the scene's order, Kitchen third"
        );
        for other in &made_of.the_other_layers {
            assert_eq!(other.record.the_source_file, made_of.record.the_source_file, "the one file");
        }
    }

    /// **The field names both sinks, which `Rented-server runtime` M1 and
    /// §14's unit require**, and reads back whole with its ending.
    #[test]
    fn the_kinds_own_field_names_both_sinks_and_reads_back_whole() {
        let mut field = ThePreviewKindsOwnField {
            job: "a1".to_string(),
            frame: 147,
            attempt: 2,
            record_directory: "/root/smoozy/job-records/a1".to_string(),
            the_job_was_interrupted: false,
            ordinary_sink: "/pkg/captured-streams/op/ordinary".to_string(),
            complaints_sink: "/pkg/captured-streams/op/complaints".to_string(),
            script: "/pkg/captured-streams/op/derivative.py".to_string(),
            ending: None,
        };
        let written = field.as_written();
        let text = String::from_utf8_lossy(&written);
        assert!(text.contains(&field.ordinary_sink) && text.contains(&field.complaints_sink));
        assert_eq!(ThePreviewKindsOwnField::read_back(&written), Some(field.clone()));
        field.ending = Some(TheProductionsEnding {
            came_to: WhatTheProductionCameTo::Published,
            the_record: Some(ADerivativesRecord {
                the_source_file: vec![b'/', 0xff],
                frame: 147,
                the_view_layer_its_picture_came_from: None,
                it_is_the_main_copy: true,
                its_place_in_the_frames_set: 0,
                where_it_came_from: None,
            }),
            published_at: Some("/r/preview-147.png".to_string()),
            bytes: Some(10),
            the_sources_size: Some((64, 48)),
            the_size_it_was_written_at: Some((64, 48)),
            it_could_not_be_brought_under_every_cap: false,
            exit_status: Some(0),
            signal: None,
            words: None,
            the_other_layers: Vec::new(),
        });
        assert_eq!(ThePreviewKindsOwnField::read_back(&field.as_written()), Some(field));
    }

    /// **The work crosses whole**, and the outcome words read back.
    #[test]
    fn the_work_crosses_whole_and_the_outcome_words_read_back() {
        let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
        let work = ThePreviewsWork {
            operation: operation.clone(),
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            job: Identity::issue(IdentityKind::Job).expect("randomness"),
            frame: -3,
            attempt: 2,
            record_directory: "/root/smoozy/job-records/a1".to_string(),
            the_job_was_interrupted: true,
            script: "/pkg/captured-streams/op/derivative.py".to_string(),
            capture: CaptureRecord {
                job: operation,
                attempt: 1,
                ordinary_sink: "/pkg/o".to_string(),
                complaints_sink: "/pkg/c".to_string(),
                ceiling: CaptureCeiling::from(1 << 30),
            },
        };
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            THE_PREVIEW_WORKERS_OWN_WORK_REQUEST,
        );
        let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0")
            .carrying(the_work_as_fields(&work, 1 << 30));
        assert_eq!(the_work_in(&answered).expect("it reads back"), work);
        for came_to in [
            WhatTheProductionCameTo::Published,
            WhatTheProductionCameTo::NothingToMake,
            WhatTheProductionCameTo::NotMade,
        ] {
            assert_eq!(WhatTheProductionCameTo::of_the_word(came_to.the_word()), Some(came_to));
        }
    }

    /// **The derivative made by real Blenders, the way `Job files and
    /// outputs` B25, `Job files and outputs` B26 and `Job files and outputs`
    /// B26a say** — run by hand at the desk, one
    /// Blender at a time, never beside a build:
    ///
    /// ```text
    /// SMOOZY_DESK_OUT=<a scratch folder> cargo test -p smoozy-server \
    ///     a_real_blender_makes_each_derivative -- --ignored --nocapture
    /// ```
    ///
    /// Each of the four Blenders renders the factory scene at 320 by 180 as
    /// a PNG, a single-layer OpenEXR and a multi-layer one, and a wide PNG at
    /// 9000 by 3000; then this worker's own [`the_derivative_is_made`] makes
    /// a copy of each, with that Blender, through the size handshake. **What
    /// it shows**: a PNG's copy is the PNG, its treatment not applied twice;
    /// an OpenEXR's copy in the job's own view transform matches the same
    /// frame Blender itself wrote as a PNG, and the plain one does not, so the
    /// treatment is what makes it right; a multi-layer file loads with no
    /// pixels and has nothing made; and the wide frame is written at the
    /// target `Job files and outputs` B26a sets. Pictures are compared by that same Blender's own Python.
    #[test]
    #[ignore = "run by hand at the desk with the four Blenders"]
    fn a_real_blender_makes_each_derivative_the_way_the_plan_says() {
        use smoozy_job_files_machine::derivatives::{
            APresentFileAndItsDestination, TheTreatment, WhatOneFrameIsMadeOf,
        };
        use smoozy_job_files_machine::main_output::APresentFile;
        use std::path::{Path, PathBuf};
        const THE_FRAMES: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
scene = bpy.context.scene
scene.render.engine = "CYCLES"
scene.cycles.samples = 8
scene.cycles.device = "CPU"
scene.render.resolution_x = 320
scene.render.resolution_y = 180
scene.render.resolution_percentage = 100
settings = scene.render.image_settings
def write(name, the_format, multi_layer=False):
    if hasattr(settings, "media_type"):
        settings.media_type = "MULTI_LAYER_IMAGE" if multi_layer else "IMAGE"
    if not (multi_layer and hasattr(settings, "media_type")):
        settings.file_format = the_format
    if the_format == "PNG":
        # An OpenEXR's depth carries over to the next PNG otherwise, and
        # Blender reads a 16-bit PNG back as linear.
        settings.color_depth = "8"
    scene.render.filepath = out + "/" + name
    bpy.ops.render.render(write_still=True)
write("frame.png", "PNG")
write("frame.exr", "OPEN_EXR")
write("frame-ml.exr", "OPEN_EXR_MULTILAYER", multi_layer=True)
# Two view layers, the second drawing every surface flat red: 4.5 writes the
# second's channels first, so only a choice by name finds the first's.
second = scene.view_layers.new("Second")
red = bpy.data.materials.new("red")
red.use_nodes = True
for node in list(red.node_tree.nodes):
    red.node_tree.nodes.remove(node)
glow = red.node_tree.nodes.new("ShaderNodeEmission")
glow.inputs[0].default_value = (1, 0, 0, 1)
end = red.node_tree.nodes.new("ShaderNodeOutputMaterial")
red.node_tree.links.new(glow.outputs[0], end.inputs[0])
second.material_override = red
write("two-ml.exr", "OPEN_EXR_MULTILAYER", multi_layer=True)
second.use = False
write("two-first.png", "PNG")
scene.view_layers.remove(second)
scene.render.engine = "BLENDER_WORKBENCH"
scene.render.resolution_x = 9000
scene.render.resolution_y = 3000
write("wide.png", "PNG")
write("wide-ml.exr", "OPEN_EXR_MULTILAYER", multi_layer=True)
print("SMOOZY-DESK-VIEW", scene.view_settings.view_transform, flush=True)
"#;
        const THE_COMPARISON: &str = r#"
import bpy, sys
import numpy
a, b = sys.argv[sys.argv.index("--") + 1:][:2]
one = bpy.data.images.load(a)
two = bpy.data.images.load(b)
print("SMOOZY-DESK-SIZE", one.size[0], one.size[1], two.size[0], two.size[1], flush=True)
if tuple(one.size) == tuple(two.size):
    p = numpy.array(one.pixels[:])
    q = numpy.array(two.pixels[:])
    print("SMOOZY-DESK-MEAN %.6f" % float(numpy.abs(p - q).mean()), flush=True)
"#;
        let out = PathBuf::from(std::env::var("SMOOZY_DESK_OUT").expect("SMOOZY_DESK_OUT"));
        let run = |blender: &str, arguments: &[&str]| -> String {
            let said = Command::new(blender)
                .args(arguments)
                .stdin(Stdio::null())
                .output()
                .expect("Blender runs");
            String::from_utf8_lossy(&said.stdout).to_string()
        };
        let the_word_after = |said: &str, mark: &str| -> Vec<String> {
            said.lines()
                .find(|line| line.starts_with(mark))
                .map(|line| line.split_whitespace().skip(1).map(str::to_string).collect())
                .unwrap_or_default()
        };
        for (version, blender) in [
            ("4.5", "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"),
            ("5.0", "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"),
            ("5.1", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
            ("5.2", "/Applications/Blender.app/Contents/MacOS/Blender"),
        ] {
            let folder = out.join(version);
            std::fs::create_dir_all(&folder).expect("the folder");
            let frames_script = folder.join("frames.py");
            std::fs::write(&frames_script, THE_FRAMES).expect("the script");
            let said = run(blender, &[
                "-b", "--factory-startup", "--python-exit-code", "1", "--python",
                frames_script.to_str().unwrap(), "--", folder.to_str().unwrap(),
            ]);
            let view = the_word_after(&said, "SMOOZY-DESK-VIEW");
            println!("{version}: the factory scene's view transform {view:?}");
            let compare = |a: &Path, b: &Path| -> (Vec<String>, Option<f64>) {
                let script = folder.join("compare.py");
                std::fs::write(&script, THE_COMPARISON).expect("the script");
                let said = run(blender, &[
                    "-b", "--factory-startup", "--python", script.to_str().unwrap(), "--",
                    a.to_str().unwrap(), b.to_str().unwrap(),
                ]);
                (
                    the_word_after(&said, "SMOOZY-DESK-SIZE"),
                    the_word_after(&said, "SMOOZY-DESK-MEAN").first().and_then(|mean| mean.parse().ok()),
                )
            };
            let make_inside = |frame: i64,
                               name: &str,
                               format: &[u8],
                               treatment: TheTreatment,
                               picture: Option<&[u8]>| {
                let source = folder.join(name);
                let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
                let streams = folder.join(format!("streams-{frame}"));
                std::fs::create_dir_all(&streams).expect("the streams' folder");
                let capture = CaptureRecord {
                    job: operation.clone(),
                    attempt: 1,
                    ordinary_sink: streams.join("ordinary").display().to_string(),
                    complaints_sink: streams.join("complaints").display().to_string(),
                    ceiling: CaptureCeiling::from(1 << 30),
                };
                smoozy_runtime::capture::MakesTheSinks::make(
                    &smoozy_runtime::acting::capture::TheSinksOnTheMachine,
                    &capture,
                )
                .expect("the sinks");
                let work = ThePreviewsWork {
                    operation,
                    run: Identity::issue(IdentityKind::Run).expect("randomness"),
                    job: Identity::issue(IdentityKind::Job).expect("randomness"),
                    frame,
                    attempt: 1,
                    record_directory: folder.display().to_string(),
                    the_job_was_interrupted: false,
                    script: streams.join("derivative.py").display().to_string(),
                    capture,
                };
                let path = source.display().to_string().into_bytes();
                let made_of = WhatOneFrameIsMadeOf {
                    the_composited_picture: None,
                    the_source: APresentFileAndItsDestination {
                        file: APresentFile {
                            path: path.clone(),
                            frame: Some(frame),
                            the_picture_it_holds: None,
                            format: format.to_vec(),
                            bytes: std::fs::metadata(&source).map(|m| m.len()).unwrap_or(0),
                            is_video_shaped: false,
                            it_sits_in_the_main_outputs_folder: true,
                        },
                        multi_layer: picture.is_some(),
                        the_destination_it_came_from: smoozy_job_files_machine::main_output::WhereThePictureCameFrom::TheOutputPath,
                        the_view_layers_that_feed_it: Vec::new(),
                    },
                    treatment,
                    the_picture_inside: picture.map(|name| {
                        smoozy_job_files_machine::reading::ArtistsText::of(name.to_vec())
                    }),
                    record: ADerivativesRecord {
                        the_source_file: path,
                        frame,
                        the_view_layer_its_picture_came_from: None,
                        it_is_the_main_copy: true,
                        its_place_in_the_frames_set: 0,
                        where_it_came_from: None,
                    },
                    the_other_layers: Vec::new(),
                };
                let ending = the_derivative_is_made(&work, blender, &made_of);
                println!("{version}: {name} -> {ending:?}");
                ending
            };
            let make = |frame: i64, name: &str, format: &[u8], treatment: TheTreatment| {
                make_inside(frame, name, format, treatment, None)
            };
            let agx = || TheTreatment::TheJobsOwnViewTransform(b"AgX".to_vec());
            let the_copy = |frame: i64| folder.join(format!("preview-{frame}.png"));

            // **A PNG's copy is the PNG**: no treatment applied twice.
            let png = make(1, "frame.png", b"PNG", TheTreatment::ItAlreadyCarriesItsTreatment);
            assert_eq!(png.came_to, WhatTheProductionCameTo::Published, "{version}");
            assert_eq!(png.the_size_it_was_written_at, Some((320, 180)), "{version}");
            let (size, mean) = compare(&the_copy(1), &folder.join("frame.png"));
            println!("{version}: the PNG's copy against the PNG: {size:?}, mean {mean:?}");
            assert!(mean.expect("compared") < 0.004, "{version}: {mean:?}");

            // **An OpenEXR's copy in the job's own view transform is the
            // frame Blender wrote as a PNG**, and a plain one is not.
            let exr = make(2, "frame.exr", b"OPEN_EXR", agx());
            assert_eq!(exr.came_to, WhatTheProductionCameTo::Published, "{version}");
            let (_, treated) = compare(&the_copy(2), &folder.join("frame.png"));
            let plain = make(3, "frame.exr", b"OPEN_EXR", TheTreatment::ItAlreadyCarriesItsTreatment);
            assert_eq!(plain.came_to, WhatTheProductionCameTo::Published, "{version}");
            let (_, untreated) = compare(&the_copy(3), &folder.join("frame.png"));
            println!("{version}: the OpenEXR's copy against the PNG: AgX {treated:?}, Standard {untreated:?}");
            assert!(treated.expect("compared") < 0.01, "{version}: {treated:?}");
            assert!(untreated.expect("compared") > 2.0 * treated.expect("compared"), "{version}");

            // **A multi-layer file sent down Blender's own route loads with no
            // pixels and has nothing made.**
            let multi = make(4, "frame-ml.exr", b"OPEN_EXR", agx());
            assert_eq!(multi.came_to, WhatTheProductionCameTo::NothingToMake, "{version}");
            assert!(!the_copy(4).exists());

            // **`Job files and outputs` B26's second route: the bundled
            // library makes the multi-layer file's beauty, in the job's own
            // view transform, and it is the frame Blender wrote as a PNG.**
            let library = make_inside(
                6,
                "frame-ml.exr",
                b"OPEN_EXR_MULTILAYER",
                agx(),
                Some(b"ViewLayer.Combined"),
            );
            assert_eq!(library.came_to, WhatTheProductionCameTo::Published, "{version}: {library:?}");
            assert_eq!(library.the_size_it_was_written_at, Some((320, 180)), "{version}");
            let (size, mean) = compare(&the_copy(6), &folder.join("frame.png"));
            println!("{version}: the multi-layer copy against the PNG: {size:?}, mean {mean:?}");
            assert!(mean.expect("compared") < 0.004, "{version}: {mean:?}");

            // **`Job files and outputs` B19a: the picture is found by its
            // name** — of two view layers, the first's, whatever order the
            // file holds them in.
            let named = make_inside(
                9,
                "two-ml.exr",
                b"OPEN_EXR_MULTILAYER",
                agx(),
                Some(b"ViewLayer.Combined"),
            );
            assert_eq!(named.came_to, WhatTheProductionCameTo::Published, "{version}: {named:?}");
            let (size, mean) = compare(&the_copy(9), &folder.join("two-first.png"));
            println!("{version}: the first view layer's copy against its own PNG: {size:?}, mean {mean:?}");
            assert!(mean.expect("compared") < 0.004, "{version}: {mean:?}");

            // **A picture not in the file has nothing made**, and leaves no
            // copy behind.
            let absent = make_inside(
                7,
                "frame-ml.exr",
                b"OPEN_EXR_MULTILAYER",
                agx(),
                Some(b"Nope.Combined"),
            );
            assert_eq!(absent.came_to, WhatTheProductionCameTo::NothingToMake, "{version}");
            assert!(!the_copy(7).exists());

            // **`Job files and outputs` B26a on the second route**: the wide multi-layer frame is
            // written at the target.
            let wide_multi = make_inside(
                8,
                "wide-ml.exr",
                b"OPEN_EXR_MULTILAYER",
                agx(),
                Some(b"ViewLayer.Combined"),
            );
            assert_eq!(wide_multi.came_to, WhatTheProductionCameTo::Published, "{version}");
            assert_eq!(wide_multi.the_sources_size, Some((9000, 3000)), "{version}");
            assert_eq!(wide_multi.the_size_it_was_written_at, Some((7500, 2500)), "{version}");
            let (size, _) = compare(&the_copy(8), &the_copy(8));
            assert_eq!(size.first().map(String::as_str), Some("7500"), "{version}: {size:?}");

            // **`Job files and outputs` B26a: the wide frame is written at
            // the target.**
            let wide = make(5, "wide.png", b"PNG", TheTreatment::ItAlreadyCarriesItsTreatment);
            assert_eq!(wide.came_to, WhatTheProductionCameTo::Published, "{version}");
            assert_eq!(wide.the_sources_size, Some((9000, 3000)), "{version}");
            assert_eq!(wide.the_size_it_was_written_at, Some((7500, 2500)), "{version}");
            let (size, _) = compare(&the_copy(5), &the_copy(5));
            assert_eq!(size.first().map(String::as_str), Some("7500"), "{version}: {size:?}");
            assert_eq!(size.get(1).map(String::as_str), Some("2500"), "{version}: {size:?}");
            println!("{version}: the wide copy is {:?} bytes, under every cap: {}", wide.bytes, !wide.it_could_not_be_brought_under_every_cap);
        }
    }

    /// **A stand-in for a Blender, for `Job files and outputs` B26a's measuring:
    /// it speaks the size handshake and writes a file of the weight `weighs`
    /// works out from the size it was told** (`w` and `h`). It says each start
    /// in `starts` as the name it wrote and the size. `before_it_writes` may end
    /// it first. It reads where to write from the worker's own script, the last
    /// word it is started with.
    struct AStandIn {
        folder: std::path::PathBuf,
        program: String,
        starts: std::path::PathBuf,
    }

    impl Drop for AStandIn {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.folder).ok();
        }
    }

    impl AStandIn {
        fn weighing(source: (u32, u32), weighs: &str, before_it_writes: &str) -> AStandIn {
            use std::os::unix::fs::PermissionsExt;
            let folder = std::env::current_exe()
                .expect("this test knows where it is")
                .ancestors()
                .nth(3)
                .expect("a test program sits three folders inside the build folder")
                .join("tmp")
                .join(format!(
                    "smoozy-b26a-{}",
                    Identity::issue(IdentityKind::Operation).expect("randomness").as_text()
                ));
            std::fs::create_dir_all(&folder).expect("the folder");
            let starts = folder.join("starts");
            let opens = String::from_utf8_lossy(smoozy_job_files_machine::derivatives::THE_SIZE_OPENS);
            let closes = String::from_utf8_lossy(smoozy_job_files_machine::derivatives::THE_SIZE_CLOSES);
            let made = String::from_utf8_lossy(smoozy_job_files_machine::derivatives::THE_MADE_MARK);
            let (width, height) = source;
            let text = format!(
                "#!/bin/sh\n\
                 for word in \"$@\"; do script=\"$word\"; done\n\
                 staged=$(sed -n 's/.*\\\\\"staged_at\\\\\":\\\\\"\\([^\\\\]*\\)\\\\\".*/\\1/p' \"$script\")\n\
                 echo '{opens}{width} {height}{closes}'\n\
                 read w h || exit 1\n\
                 echo \"${{staged##*/}} $w $h\" >> '{starts}'\n\
                 {before_it_writes}\n\
                 head -c $(({weighs})) /dev/zero > \"$staged\"\n\
                 echo '{made}'\n",
                starts = starts.display(),
            );
            let program = folder.join("stand-in");
            std::fs::write(&program, text).expect("the stand-in");
            std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).expect("runnable");
            AStandIn { program: program.display().to_string(), folder, starts }
        }

        fn starts(&self) -> Vec<String> {
            std::fs::read_to_string(&self.starts)
                .unwrap_or_default()
                .lines()
                .map(str::to_string)
                .collect()
        }

        /// One frame's copies made by the worker's own route, the frame's other
        /// View Layers' among them.
        fn makes(&self, other_layers: u32) -> TheProductionsEnding {
            use smoozy_job_files_machine::derivatives::{
                APresentFileAndItsDestination, AnotherLayersCopy, TheTreatment, WhatOneFrameIsMadeOf,
            };
            use smoozy_job_files_machine::main_output::{APresentFile, WhereThePictureCameFrom};
            let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
            let capture = CaptureRecord {
                job: operation.clone(),
                attempt: 1,
                ordinary_sink: self.folder.join("ordinary").display().to_string(),
                complaints_sink: self.folder.join("complaints").display().to_string(),
                ceiling: CaptureCeiling::from(1 << 30),
            };
            smoozy_runtime::capture::MakesTheSinks::make(
                &smoozy_runtime::acting::capture::TheSinksOnTheMachine,
                &capture,
            )
            .expect("the sinks");
            let work = ThePreviewsWork {
                operation,
                run: Identity::issue(IdentityKind::Run).expect("randomness"),
                job: Identity::issue(IdentityKind::Job).expect("randomness"),
                frame: 1,
                attempt: 1,
                record_directory: self.folder.display().to_string(),
                the_job_was_interrupted: false,
                script: self.folder.join("derivative.py").display().to_string(),
                capture,
            };
            let record = |main: bool, place: u32| ADerivativesRecord {
                the_source_file: b"/j/f_0001.png".to_vec(),
                frame: 1,
                the_view_layer_its_picture_came_from: None,
                it_is_the_main_copy: main,
                its_place_in_the_frames_set: place,
                where_it_came_from: None,
            };
            let made_of = WhatOneFrameIsMadeOf {
                the_composited_picture: None,
                the_source: APresentFileAndItsDestination {
                    file: APresentFile {
                        path: b"/j/f_0001.png".to_vec(),
                        frame: Some(1),
                        the_picture_it_holds: None,
                        format: b"PNG".to_vec(),
                        bytes: 1,
                        is_video_shaped: false,
                        it_sits_in_the_main_outputs_folder: true,
                    },
                    multi_layer: false,
                    the_destination_it_came_from: WhereThePictureCameFrom::TheOutputPath,
                    the_view_layers_that_feed_it: Vec::new(),
                },
                treatment: TheTreatment::ItAlreadyCarriesItsTreatment,
                the_picture_inside: None,
                record: record(true, 0),
                the_other_layers: (1..=other_layers)
                    .map(|place| AnotherLayersCopy {
                        treatment: TheTreatment::ItAlreadyCarriesItsTreatment,
                        the_picture_inside: None,
                        record: record(false, place),
                    })
                    .collect(),
            };
            the_derivative_is_made(&work, &self.program, &made_of)
        }

        /// What sits in the record directory besides the stand-in's own files:
        /// no staged name is left behind.
        fn staged_names_left(&self) -> Vec<String> {
            std::fs::read_dir(&self.folder)
                .expect("the folder")
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .filter(|name| name.starts_with(smoozy_job_files_machine::publication::THE_STAGING_PREFIX))
                .collect()
        }

        fn weight_of(&self, name: &str) -> u64 {
            std::fs::metadata(self.folder.join(name)).expect("published").len()
        }
    }

    const THE_CAP: u64 = THE_LARGEST_A_DERIVATIVE_MAY_BE_IN_BYTES;
    /// A weight that follows the area, as a frame of noise's does.
    const NOISE: &str = "w * h * 3";

    /// **`Job files and outputs` B26a: a copy under the cap is written once**, at
    /// the size `the_target_size` set, and is what was written.
    #[test]
    fn b26a_a_copy_under_the_byte_cap_is_written_once() {
        let blender = AStandIn::weighing((3840, 2160), "w * h / 10", "");
        let ending = blender.makes(0);
        assert_eq!(ending.came_to, WhatTheProductionCameTo::Published, "{ending:?}");
        assert_eq!(blender.starts(), ["staging-preview-1.png 3840 2160"]);
        assert_eq!(ending.the_size_it_was_written_at, Some((3840, 2160)));
        assert_eq!(ending.the_sources_size, Some((3840, 2160)));
        assert_eq!(ending.bytes, Some(3840 * 2160 / 10));
        assert_eq!(blender.weight_of("preview-1.png"), 3840 * 2160 / 10);
        assert!(!ending.it_could_not_be_brought_under_every_cap);
        assert!(blender.staged_names_left().is_empty(), "{:?}", blender.staged_names_left());
    }

    /// **`Job files and outputs` B26a: a copy over the cap is made again smaller,
    /// keeping the ratio, and the smaller is what is published.**
    #[test]
    fn b26a_a_copy_over_the_byte_cap_is_written_again_smaller() {
        let blender = AStandIn::weighing((3840, 2160), NOISE, "");
        let ending = blender.makes(0);
        assert_eq!(ending.came_to, WhatTheProductionCameTo::Published, "{ending:?}");
        assert_eq!(blender.starts().len(), 2, "{:?}", blender.starts());
        let (width, height) = ending.the_size_it_was_written_at.expect("the size it went out at");
        assert!(width < 3840, "{width}");
        assert!((f64::from(width) / f64::from(height) - 16.0 / 9.0).abs() < 0.01, "{width} {height}");
        assert_eq!(ending.the_sources_size, Some((3840, 2160)), "the source's own size stands");
        let weighs = blender.weight_of("preview-1.png");
        assert!(weighs <= THE_CAP, "{weighs}");
        assert_eq!(ending.bytes, Some(weighs));
        assert!(!ending.it_could_not_be_brought_under_every_cap, "{ending:?}");
        assert!(blender.staged_names_left().is_empty(), "{:?}", blender.staged_names_left());
    }

    /// **Three writes at most**, and a copy that never comes under the cap is
    /// published at its lightest with the fact on the record.
    #[test]
    fn b26a_three_writes_then_the_lightest_is_published_with_the_fact() {
        let blender = AStandIn::weighing((3840, 2160), "11000000 + w * h / 1000", "");
        let ending = blender.makes(0);
        assert_eq!(ending.came_to, WhatTheProductionCameTo::Published, "{ending:?}");
        let starts = blender.starts();
        assert_eq!(starts.len() as u32, smoozy_job_files_machine::main_output::THE_MOST_WRITES_OF_ONE_COPY);
        assert!(ending.it_could_not_be_brought_under_every_cap, "{ending:?}");
        let (width, height) = ending.the_size_it_was_written_at.expect("the size it went out at");
        assert_eq!(starts.last().map(String::as_str), Some(format!("staging-preview-1.png {width} {height}").as_str()));
        assert_eq!(ending.bytes, Some(11_000_000 + u64::from(width) * u64::from(height) / 1000));
        assert_eq!(ending.bytes, Some(blender.weight_of("preview-1.png")));
        assert!(blender.staged_names_left().is_empty(), "{:?}", blender.staged_names_left());
    }

    /// **A smaller write that weighs no less ends the tries**: the first stands.
    #[test]
    fn b26a_a_smaller_write_that_weighs_no_less_leaves_the_first() {
        let blender = AStandIn::weighing(
            (3840, 2160),
            "w == 3840 ? 11000000 : 12000000",
            "",
        );
        let ending = blender.makes(0);
        assert_eq!(ending.came_to, WhatTheProductionCameTo::Published, "{ending:?}");
        assert_eq!(blender.starts().len(), 2, "{:?}", blender.starts());
        assert_eq!(ending.the_size_it_was_written_at, Some((3840, 2160)));
        assert_eq!(ending.bytes, Some(11_000_000));
        assert_eq!(blender.weight_of("preview-1.png"), 11_000_000);
        assert!(ending.it_could_not_be_brought_under_every_cap);
        assert!(blender.staged_names_left().is_empty(), "{:?}", blender.staged_names_left());
    }

    /// **A later write that fails costs nothing**: the copy already written is
    /// published with the fact, rather than the frame losing its preview.
    #[test]
    fn b26a_a_later_write_that_fails_leaves_the_copy_already_written() {
        let blender = AStandIn::weighing((3840, 2160), NOISE, "[ \"$w\" = 3840 ] || exit 3");
        let ending = blender.makes(0);
        assert_eq!(ending.came_to, WhatTheProductionCameTo::Published, "{ending:?}");
        assert_eq!(blender.starts().len(), 2, "{:?}", blender.starts());
        assert_eq!(ending.the_size_it_was_written_at, Some((3840, 2160)));
        assert_eq!(blender.weight_of("preview-1.png"), 3840 * 2160 * 3);
        assert!(ending.it_could_not_be_brought_under_every_cap);
        assert_eq!(ending.exit_status, Some(0), "the ending speaks for the copy it published");
        assert!(blender.staged_names_left().is_empty(), "{:?}", blender.staged_names_left());
    }

    /// **A first write that fails is what it always was**: nothing published, so
    /// the frame stays owed (`Queue & History` P1a), and one start.
    #[test]
    fn b26a_a_first_write_that_fails_publishes_nothing() {
        let blender = AStandIn::weighing((3840, 2160), NOISE, "exit 3");
        let ending = blender.makes(0);
        assert_eq!(ending.came_to, WhatTheProductionCameTo::NotMade, "{ending:?}");
        assert_eq!(blender.starts().len(), 1);
        assert!(!blender.folder.join("preview-1.png").exists());
    }

    /// **Each View Layer's copy is measured on its own.** The main copy is light
    /// and is written once; the other layer's is heavy and is written twice.
    #[test]
    fn b26a_each_view_layers_copy_comes_under_the_byte_cap_on_its_own() {
        let blender = AStandIn::weighing(
            (3840, 2160),
            "w * h * 3 * heavy + w * h / 10 * (1 - heavy)",
            "case \"${staged##*/}\" in *layer*) heavy=1;; *) heavy=0;; esac",
        );
        let ending = blender.makes(1);
        assert_eq!(ending.came_to, WhatTheProductionCameTo::Published, "{ending:?}");
        let starts = blender.starts();
        assert_eq!(starts.len(), 3, "{starts:?}");
        assert_eq!(starts[0], "staging-preview-1.png 3840 2160");
        assert_eq!(ending.the_size_it_was_written_at, Some((3840, 2160)), "the main copy is untouched");
        let other = &ending.the_other_layers[0];
        assert_eq!(other.came_to, WhatTheProductionCameTo::Published, "{other:?}");
        assert!(other.the_size_it_was_written_at.expect("written").0 < 3840, "{other:?}");
        assert!(blender.weight_of("preview-1-layer-1.png") <= THE_CAP);
        assert!(!other.it_could_not_be_brought_under_every_cap, "{other:?}");
        assert!(blender.staged_names_left().is_empty(), "{:?}", blender.staged_names_left());
    }

    /// **`Job files and outputs` B26a on real Blenders, through this worker's
    /// own route**, 5.2.3 and the 5.3 beta, one at a time. Run by hand at the
    /// desk, never beside a build:
    ///
    /// ```text
    /// SMOOZY_DESK_OUT=<an empty folder> cargo test -p smoozy-server \
    ///     a_frame_too_heavy_comes_under_the_byte_cap_at_the_desk -- --ignored --nocapture
    /// ```
    ///
    /// **On both of `Job files and outputs` B26's routes**: a 4K frame of pure noise comes out under the
    /// cap, the shape it went in and the right picture, and Telegram's own check
    /// says it fits; a 4K plain gradient comes out at 4K, from a multi-layer
    /// file with no compression too, its pixels those of the copy made without
    /// `Job files and outputs` B26 step 4's naming of the compression. Telling the noise
    /// apart: each half of the frame is noise over its own range, so a copy's two
    /// halves say whether it is this picture, and the multi-layer file's second
    /// View Layer has its halves the other way round, so its copy says it is that
    /// layer's.
    #[test]
    #[ignore = "run by hand at the desk with Blender 5.2.3 and the 5.3 beta"]
    fn a_frame_too_heavy_comes_under_the_byte_cap_at_the_desk() {
        use smoozy_job_files_machine::derivatives::{
            APresentFileAndItsDestination, AnotherLayersCopy, TheTreatment, WhatOneFrameIsMadeOf,
        };
        use smoozy_job_files_machine::main_output::{APresentFile, WhereThePictureCameFrom};
        use smoozy_job_files_machine::reading::ArtistsText;
        use smoozy_telegram::pictures::{whether_it_fits, WhetherItFits};
        use std::path::{Path, PathBuf};

        const THE_FRAMES: &str = r#"
import sys, numpy
import OpenImageIO as oiio
out = sys.argv[sys.argv.index("--") + 1]
W, H = 3840, 2160
rng = numpy.random.default_rng(7)
def noise(low_on_the_left):
    a = rng.random((H, W, 3), dtype=numpy.float32) * 0.5
    a[:, W // 2:] += 0.5
    return a if low_on_the_left else a[:, ::-1]
ramp = numpy.repeat(numpy.linspace(0, 1, W, dtype=numpy.float32)[None, :, None], H, axis=0).repeat(3, axis=2)
def png(name, a):
    spec = oiio.ImageSpec(W, H, 3, oiio.UINT8)
    buf = oiio.ImageBuf(spec)
    assert buf.set_pixels(spec.roi, a), buf.geterror()
    assert buf.write(out + "/" + name, oiio.TypeUInt8, "png"), buf.geterror()
def multi_layer(name, layers, compression="zip"):
    names = tuple(layer + ".Combined." + letter for layer, _ in layers for letter in "RGBA")
    spec = oiio.ImageSpec(W, H, len(names), oiio.HALF)
    spec.channelnames = names
    spec.attribute("compression", compression)
    whole = numpy.concatenate(
        [numpy.dstack((a, numpy.ones((H, W, 1), dtype=numpy.float32))) for _, a in layers], axis=2
    )
    buf = oiio.ImageBuf(spec)
    assert buf.set_pixels(spec.roi, whole), buf.geterror()
    assert buf.write(out + "/" + name, oiio.HALF, "exr"), buf.geterror()
png("noise.png", noise(True))
png("gradient.png", ramp)
multi_layer("noise-ml.exr", [("ViewLayer", noise(True)), ("Second", noise(False))])
multi_layer("gradient-ml.exr", [("ViewLayer", ramp)])
multi_layer("gradient-none.exr", [("ViewLayer", ramp)], "none")
print("SMOOZY-DESK-FRAMES", flush=True)
"#;
        const THE_LOOK: &str = r#"
import sys, numpy
import OpenImageIO as oiio
path = sys.argv[sys.argv.index("--") + 1]
buf = oiio.ImageBuf(path)
a = buf.get_pixels(oiio.FLOAT)
w = a.shape[1]
print("SMOOZY-DESK-LOOK", buf.spec().width, buf.spec().height, buf.spec().nchannels,
      "%.4f" % float(a[:, :w // 2].mean()), "%.4f" % float(a[:, w // 2:].mean()), flush=True)
"#;
        const THE_COMPARISON_OF_PIXELS: &str = r#"
import sys, numpy
import OpenImageIO as oiio
one, two = sys.argv[sys.argv.index("--") + 1:][:2]
a = oiio.ImageBuf(one)
b = oiio.ImageBuf(two)
print("SMOOZY-DESK-SAME", a.spec().width, a.spec().height,
      bool(numpy.array_equal(a.get_pixels(oiio.UINT8), b.get_pixels(oiio.UINT8))), flush=True)
"#;
        let out = PathBuf::from(std::env::var("SMOOZY_DESK_OUT").expect("SMOOZY_DESK_OUT"));
        for (version, blender) in [
            ("5.2.3", "/Applications/Blender.app/Contents/MacOS/Blender"),
            ("5.3b", "/Applications/Blender 5.3b.app/Contents/MacOS/Blender"),
        ] {
            let folder = out.join(version);
            std::fs::create_dir_all(&folder).expect("the folder");
            let run = |script: &str, name: &str, argument: &Path| -> String {
                let at = folder.join(name);
                std::fs::write(&at, script).expect("the script");
                let said = Command::new(blender)
                    .args(["-b", "--factory-startup", "--python-exit-code", "1", "--python"])
                    .arg(&at)
                    .arg("--")
                    .arg(argument)
                    .stdin(Stdio::null())
                    .output()
                    .expect("Blender runs");
                let text = String::from_utf8_lossy(&said.stdout).to_string();
                assert!(said.status.success(), "{text}\n{}", String::from_utf8_lossy(&said.stderr));
                text
            };
            assert!(run(THE_FRAMES, "frames.py", &folder).contains("SMOOZY-DESK-FRAMES"), "{version}");
            for name in ["noise.png", "gradient.png"] {
                let weighs = std::fs::metadata(folder.join(name)).expect("written").len();
                println!("{version}: the source {name} weighs {weighs} bytes");
            }
            assert!(
                std::fs::metadata(folder.join("noise.png")).expect("written").len() > THE_CAP,
                "the noise is over the cap to begin with"
            );

            // One copy looked at: its size, and its two halves' means.
            let look = |path: &Path| -> (u32, u32, u32, f64, f64) {
                let said = run(THE_LOOK, "look.py", path);
                let line = said
                    .lines()
                    .find(|line| line.starts_with("SMOOZY-DESK-LOOK"))
                    .expect("it looked")
                    .split_whitespace()
                    .skip(1)
                    .map(str::to_string)
                    .collect::<Vec<_>>();
                (
                    line[0].parse().unwrap(),
                    line[1].parse().unwrap(),
                    line[2].parse().unwrap(),
                    line[3].parse().unwrap(),
                    line[4].parse().unwrap(),
                )
            };
            let run_on_two = |script: &str, name: &str, one: &Path, two: &Path| -> String {
                let at = folder.join(name);
                std::fs::write(&at, script).expect("the script");
                let said = Command::new(blender)
                    .args(["-b", "--factory-startup", "--python-exit-code", "1", "--python"])
                    .arg(&at)
                    .arg("--")
                    .args([one, two])
                    .stdin(Stdio::null())
                    .output()
                    .expect("Blender runs");
                assert!(said.status.success(), "{}", String::from_utf8_lossy(&said.stderr));
                String::from_utf8_lossy(&said.stdout).to_string()
            };
            // The multi-layer copy as it was before `Job files and outputs` B26 step 4 named the
            // compression: the worker's own script at the source's own size, with
            // that one line taken out.
            let the_copy_with_no_name = |frame: i64, name: &str| -> PathBuf {
                let at = folder.join(format!("no-name-{frame}.png"));
                let script = the_derivatives_script(&ADerivativeToMake {
                    source: folder.join(name).display().to_string().into_bytes(),
                    staged_at: at.display().to_string(),
                    treatment: TheTreatment::TheJobsOwnViewTransform(b"Standard".to_vec()),
                    the_picture_inside: Some(b"ViewLayer.Combined".to_vec()),
                    the_composited_picture: None,
                });
                let the_line = "flat.specmod().attribute(\"compression\", \"zip\")\n";
                assert!(script.contains(the_line), "the script names the compression");
                let script_at = folder.join(format!("no-name-{frame}.py"));
                std::fs::write(&script_at, script.replace(the_line, "")).expect("the script");
                let mut started = Command::new(blender)
                    .args(the_words_of_a_derivative(script_at.to_str().unwrap()).iter().map(|word| OsStr::from_bytes(word)))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .expect("Blender runs");
                started.stdin.take().expect("its input").write_all(b"3840 2160\n").expect("told");
                assert!(started.wait().expect("it ended").success(), "{version}");
                at
            };
            let make = |frame: i64, name: &str, inside: &[&str]| {
                let source = folder.join(name);
                let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
                let streams = folder.join(format!("streams-{frame}"));
                std::fs::create_dir_all(&streams).expect("the streams' folder");
                let capture = CaptureRecord {
                    job: operation.clone(),
                    attempt: 1,
                    ordinary_sink: streams.join("ordinary").display().to_string(),
                    complaints_sink: streams.join("complaints").display().to_string(),
                    ceiling: CaptureCeiling::from(1 << 30),
                };
                smoozy_runtime::capture::MakesTheSinks::make(
                    &smoozy_runtime::acting::capture::TheSinksOnTheMachine,
                    &capture,
                )
                .expect("the sinks");
                let work = ThePreviewsWork {
                    operation,
                    run: Identity::issue(IdentityKind::Run).expect("randomness"),
                    job: Identity::issue(IdentityKind::Job).expect("randomness"),
                    frame,
                    attempt: 1,
                    record_directory: folder.display().to_string(),
                    the_job_was_interrupted: false,
                    script: streams.join("derivative.py").display().to_string(),
                    capture,
                };
                let path = source.display().to_string().into_bytes();
                let a_layer = |layer: &str| ArtistsText::of(format!("{layer}.Combined").into_bytes());
                let treatment = if inside.is_empty() {
                    TheTreatment::ItAlreadyCarriesItsTreatment
                } else {
                    TheTreatment::TheJobsOwnViewTransform(b"Standard".to_vec())
                };
                let record = |main: bool, place: u32| ADerivativesRecord {
                    the_source_file: path.clone(),
                    frame,
                    the_view_layer_its_picture_came_from: None,
                    it_is_the_main_copy: main,
                    its_place_in_the_frames_set: place,
                    where_it_came_from: None,
                };
                let made_of = WhatOneFrameIsMadeOf {
                    the_composited_picture: None,
                    the_source: APresentFileAndItsDestination {
                        file: APresentFile {
                            path: path.clone(),
                            frame: Some(frame),
                            the_picture_it_holds: None,
                            format: if inside.is_empty() { b"PNG".to_vec() } else { b"OPEN_EXR_MULTILAYER".to_vec() },
                            bytes: std::fs::metadata(&source).map(|m| m.len()).unwrap_or(0),
                            is_video_shaped: false,
                            it_sits_in_the_main_outputs_folder: true,
                        },
                        multi_layer: !inside.is_empty(),
                        the_destination_it_came_from: WhereThePictureCameFrom::TheOutputPath,
                        the_view_layers_that_feed_it: Vec::new(),
                    },
                    treatment: treatment.clone(),
                    the_picture_inside: inside.first().map(|layer| a_layer(layer)),
                    record: record(true, 0),
                    the_other_layers: inside
                        .iter()
                        .enumerate()
                        .skip(1)
                        .map(|(place, layer)| AnotherLayersCopy {
                            treatment: treatment.clone(),
                            the_picture_inside: Some(a_layer(layer)),
                            record: record(false, place as u32),
                        })
                        .collect(),
                };
                let began = Instant::now();
                let ending = the_derivative_is_made(&work, blender, &made_of);
                println!(
                    "{version}: {name}: {:?} of {:?} at {:?}, {:?} bytes, over a cap: {}, in {:.1} seconds",
                    ending.came_to,
                    ending.the_sources_size,
                    ending.the_size_it_was_written_at,
                    ending.bytes,
                    ending.it_could_not_be_brought_under_every_cap,
                    began.elapsed().as_secs_f64(),
                );
                ending
            };
            let it_fits = |ending: &TheProductionsEnding| {
                assert_eq!(ending.came_to, WhatTheProductionCameTo::Published, "{version}: {ending:?}");
                assert!(!ending.it_could_not_be_brought_under_every_cap, "{version}: {ending:?}");
                let at = ending.published_at.as_ref().expect("published");
                let bytes = std::fs::read(at).expect("it reads");
                assert!(bytes.len() as u64 <= THE_CAP, "{version}: {}", bytes.len());
                assert_eq!(ending.bytes, Some(bytes.len() as u64), "{version}");
                assert_eq!(whether_it_fits(&bytes), WhetherItFits::Fits, "{version}");
                println!("{version}: {at} weighs {} bytes and Telegram's check says it fits", bytes.len());
                look(Path::new(at))
            };
            let a_smaller_right_picture = |ending: &TheProductionsEnding, left_is_lower: bool| {
                let (width, height, channels, left, right) = it_fits(ending);
                assert!(width < 3840 && height < 2160, "{version}: {width} {height}");
                assert_eq!(ending.the_size_it_was_written_at, Some((width, height)), "{version}");
                assert_eq!(ending.the_sources_size, Some((3840, 2160)), "{version}");
                assert!((f64::from(width) / f64::from(height) - 16.0 / 9.0).abs() < 0.01, "{version}");
                assert_eq!(channels, 3, "{version}: the copy carries no transparency (`Job files and outputs` B28)");
                println!("{version}: the halves' means {left} and {right}");
                assert_eq!(left < right, left_is_lower, "{version}: {left} {right}");
                assert!((left - right).abs() > 0.2, "{version}: {left} {right}");
            };
            let at_4k_as_before = |ending: &TheProductionsEnding| {
                let (width, height, channels, left, right) = it_fits(ending);
                assert_eq!((width, height), (3840, 2160), "{version}");
                assert_eq!(channels, 3, "{version}");
                assert!(left < right, "{version}: {left} {right}");
            };

            // **Blender's own route.**
            a_smaller_right_picture(&make(1, "noise.png", &[]), true);
            at_4k_as_before(&make(2, "gradient.png", &[]));
            // **The bundled library's route, and a View Layer's own copy.**
            let two_layers = make(3, "noise-ml.exr", &["ViewLayer", "Second"]);
            a_smaller_right_picture(&two_layers, true);
            a_smaller_right_picture(&two_layers.the_other_layers[0], false);
            at_4k_as_before(&make(4, "gradient-ml.exr", &["ViewLayer"]));
            // **`Job files and outputs` B26 step 4: a multi-layer file with no
            // compression comes out at 4K**, its PNG's compression named. Without
            // the name the bundled library carries the source's over and the PNG
            // weighs its pixels. Its pixels are those of the copy made without
            // the name; a file with ZIP comes out byte for byte as it did.
            let none = make(5, "gradient-none.exr", &["ViewLayer"]);
            at_4k_as_before(&none);
            let unnamed = the_copy_with_no_name(5, "gradient-none.exr");
            let unnamed_weighs = std::fs::metadata(&unnamed).expect("written").len();
            println!("{version}: without the name it weighed {unnamed_weighs} bytes, with it {:?}", none.bytes);
            assert!(unnamed_weighs > THE_CAP, "{version}: {unnamed_weighs}");
            let published = PathBuf::from(none.published_at.as_ref().expect("published"));
            let said = run_on_two(THE_COMPARISON_OF_PIXELS, "same.py", &published, &unnamed);
            println!("{version}: {}", said.lines().find(|line| line.starts_with("SMOOZY-DESK-SAME")).unwrap_or("no comparison"));
            assert!(said.contains("SMOOZY-DESK-SAME 3840 2160 True"), "{version}: {said}");
            let zipped = the_copy_with_no_name(4, "gradient-ml.exr");
            assert_eq!(
                std::fs::read(&zipped).expect("written"),
                std::fs::read(folder.join("preview-4.png")).expect("published"),
                "{version}: a file with ZIP comes out byte for byte as it did"
            );
        }
    }

    /// **The Creator's seventh notes, note 4 — a frame is published only where
    /// every copy it owed is**: a layer copy that failed leaves the frame *not
    /// made*, owed again (`Queue & History` P1a read per copy); one with nothing to make does
    /// not; and every copy published is journalled, the main one first.
    #[test]
    fn a_frame_is_published_only_where_every_copy_it_owed_is() {
        let a_copy = |came_to: WhatTheProductionCameTo, main: bool, place: u32| TheProductionsEnding {
            came_to,
            the_record: Some(ADerivativesRecord {
                the_source_file: b"/j/f_0001.exr".to_vec(),
                frame: 1,
                the_view_layer_its_picture_came_from: None,
                it_is_the_main_copy: main,
                its_place_in_the_frames_set: place,
                where_it_came_from: None,
            }),
            published_at: None,
            bytes: None,
            the_sources_size: None,
            the_size_it_was_written_at: None,
            it_could_not_be_brought_under_every_cap: false,
            exit_status: None,
            signal: None,
            words: None,
            the_other_layers: Vec::new(),
        };
        let mut frame = a_copy(WhatTheProductionCameTo::Published, true, 0);
        frame.the_other_layers.push(a_copy(WhatTheProductionCameTo::Published, false, 1));
        frame.the_other_layers.push(a_copy(WhatTheProductionCameTo::NothingToMake, false, 2));
        assert_eq!(frame.the_frames_outcome(), WhatTheProductionCameTo::Published);
        let published = frame.every_copy_published();
        assert_eq!(published.len(), 2);
        assert!(published[0].the_record.as_ref().unwrap().it_is_the_main_copy, "the main copy first");
        frame.the_other_layers.push(a_copy(WhatTheProductionCameTo::NotMade, false, 3));
        assert_eq!(frame.the_frames_outcome(), WhatTheProductionCameTo::NotMade, "owed again");
        assert_eq!(frame.every_copy_published().len(), 2, "what was published is still journalled");
        // An ending written before the layers reads back with none.
        let written = serde_json::to_string(&a_copy(WhatTheProductionCameTo::Published, true, 0)).unwrap();
        assert!(!written.contains("the_other_layers"), "{written}");
        let read: TheProductionsEnding = serde_json::from_str(&written).unwrap();
        assert!(read.the_other_layers.is_empty());
    }

    /// **The Creator's seventh notes, note 4, at the desk, with one Blender and
    /// small files**: a two-layer multi-layer frame gives two copies — the main
    /// one first, at the frame's own name, and the other layer's at its place;
    /// a frame where the second layer did not render gives
    /// one; and a layer the frame's set names but the file does not hold has
    /// nothing made and leaves the frame published. Run by hand:
    /// `SMOOZY_DESK_OUT=<an empty folder> cargo test -- --ignored
    /// one_copy_per_view_layer_at_the_desk`.
    #[test]
    #[ignore = "run by hand at the desk with one Blender"]
    fn one_copy_per_view_layer_at_the_desk() {
        use smoozy_job_files_machine::derivatives::{
            the_other_layers_copies, APresentFileAndItsDestination, TheTreatment, WhatOneFrameIsMadeOf,
        };
        use smoozy_job_files_machine::main_output::APresentFile;
        use smoozy_job_files_machine::reading::ArtistsText;
        use std::path::PathBuf;
        const THE_FRAMES: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
scene = bpy.context.scene
scene.render.engine = "BLENDER_WORKBENCH"
scene.render.resolution_x = 64
scene.render.resolution_y = 36
scene.render.resolution_percentage = 100
settings = scene.render.image_settings
if hasattr(settings, "media_type"):
    settings.media_type = "MULTI_LAYER_IMAGE"
else:
    settings.file_format = "OPEN_EXR_MULTILAYER"
second = scene.view_layers.new("Second")
scene.render.filepath = out + "/two-ml.exr"
bpy.ops.render.render(write_still=True)
second.use = False
scene.render.filepath = out + "/one-ml.exr"
bpy.ops.render.render(write_still=True)
"#;
        let blender = "/Applications/Blender.app/Contents/MacOS/Blender";
        let folder = PathBuf::from(std::env::var("SMOOZY_DESK_OUT").expect("SMOOZY_DESK_OUT"));
        std::fs::create_dir_all(&folder).expect("the folder");
        let script = folder.join("frames.py");
        std::fs::write(&script, THE_FRAMES).expect("the script");
        let rendered = Command::new(blender)
            .args(["-b", "--factory-startup", "--python-exit-code", "1", "--python"])
            .arg(&script)
            .arg("--")
            .arg(&folder)
            .stdin(Stdio::null())
            .output()
            .expect("Blender runs");
        assert!(rendered.status.success(), "{}", String::from_utf8_lossy(&rendered.stderr));
        let layer = |name: &str| ArtistsText::of(name.as_bytes().to_vec());
        let make = |frame: i64, name: &str, the_frames_layers: &[ArtistsText]| {
            let source = folder.join(name);
            let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
            let streams = folder.join(format!("streams-{frame}"));
            std::fs::create_dir_all(&streams).expect("the streams' folder");
            let capture = CaptureRecord {
                job: operation.clone(),
                attempt: 1,
                ordinary_sink: streams.join("ordinary").display().to_string(),
                complaints_sink: streams.join("complaints").display().to_string(),
                ceiling: CaptureCeiling::from(1 << 30),
            };
            smoozy_runtime::capture::MakesTheSinks::make(&smoozy_runtime::acting::capture::TheSinksOnTheMachine, &capture)
                .expect("the sinks");
            let work = ThePreviewsWork {
                operation,
                run: Identity::issue(IdentityKind::Run).expect("randomness"),
                job: Identity::issue(IdentityKind::Job).expect("randomness"),
                frame,
                attempt: 1,
                record_directory: folder.display().to_string(),
                the_job_was_interrupted: false,
                script: streams.join("derivative.py").display().to_string(),
                capture,
            };
            let path = source.display().to_string().into_bytes();
            let record = ADerivativesRecord {
                the_source_file: path.clone(),
                frame,
                the_view_layer_its_picture_came_from: (the_frames_layers.len() > 1).then(|| layer("ViewLayer")),
                it_is_the_main_copy: true,
                its_place_in_the_frames_set: 0,
                where_it_came_from: None,
            };
            let main_inside = layer("ViewLayer.Combined");
            let agx = TheTreatment::TheJobsOwnViewTransform(b"AgX".to_vec());
            let the_other_layers = the_other_layers_copies(
                &ADerivativesRecord {
                    the_view_layer_its_picture_came_from: Some(layer("ViewLayer")),
                    ..record.clone()
                },
                &agx,
                Some(&main_inside),
                the_frames_layers,
            );
            let made_of = WhatOneFrameIsMadeOf {
                the_composited_picture: None,
                the_source: APresentFileAndItsDestination {
                    file: APresentFile {
                        path,
                        frame: Some(frame),
                        the_picture_it_holds: None,
                        format: b"OPEN_EXR_MULTILAYER".to_vec(),
                        bytes: std::fs::metadata(&source).map(|m| m.len()).unwrap_or(0),
                        is_video_shaped: false,
                        it_sits_in_the_main_outputs_folder: true,
                    },
                    multi_layer: true,
                    the_destination_it_came_from: smoozy_job_files_machine::main_output::WhereThePictureCameFrom::TheOutputPath,
                    the_view_layers_that_feed_it: Vec::new(),
                },
                treatment: agx,
                the_picture_inside: Some(main_inside),
                record,
                the_other_layers,
            };
            let ending = the_derivative_is_made(&work, blender, &made_of);
            println!("frame {frame}: {ending:?}");
            ending
        };
        let both = [layer("ViewLayer"), layer("Second")];

        // **Two layers rendered: two copies, the main one first.**
        let two = make(1, "two-ml.exr", &both);
        assert_eq!(two.came_to, WhatTheProductionCameTo::Published, "{two:?}");
        assert!(two.the_record.as_ref().unwrap().it_is_the_main_copy);
        assert_eq!(two.the_other_layers.len(), 1);
        assert_eq!(two.the_other_layers[0].came_to, WhatTheProductionCameTo::Published, "{two:?}");
        assert_eq!(two.the_frames_outcome(), WhatTheProductionCameTo::Published);
        let main = folder.join("preview-1.png");
        let other = folder.join("preview-1-layer-1.png");
        assert!(main.exists() && other.exists());
        assert_eq!(two.the_other_layers[0].published_at.as_deref(), other.to_str());

        // **The second layer did not render at this frame: one copy.**
        let one = make(2, "one-ml.exr", &both[..1]);
        assert_eq!(one.came_to, WhatTheProductionCameTo::Published, "{one:?}");
        assert!(one.the_other_layers.is_empty());
        assert!(folder.join("preview-2.png").exists());
        assert!(!folder.join("preview-2-layer-1.png").exists());

        // **A layer the set names and the file does not hold: nothing made,
        // and the frame is still published.**
        let short = make(3, "one-ml.exr", &both);
        assert_eq!(short.the_other_layers.len(), 1);
        assert_eq!(short.the_other_layers[0].came_to, WhatTheProductionCameTo::NothingToMake, "{short:?}");
        assert_eq!(short.the_frames_outcome(), WhatTheProductionCameTo::Published);
        assert!(!folder.join("preview-3-layer-1.png").exists());
    }

    /// **The Creator's ninth notes, note 2, at the desk, with one Blender and
    /// small files**: where the main copy is not made by the multi-layer route,
    /// the other layer's copy is made of the file its own destination wrote
    /// (`Job files and outputs` B31a's last bullet). Two view layers' beauties
    /// are written as two PNGs of different sizes — standing in for two file
    /// output nodes, one per layer — so the copy's source size says which file
    /// it was made of. Run by hand: `SMOOZY_DESK_OUT=<an empty folder> cargo
    /// test -- --ignored one_copy_per_view_layer_from_its_own_file_at_the_desk`.
    #[test]
    #[ignore = "run by hand at the desk with one Blender"]
    fn one_copy_per_view_layer_from_its_own_file_at_the_desk() {
        use smoozy_job_files_machine::derivatives::{
            the_derivatives_record, the_other_layers_own_copies, the_rungs_are_told,
            APresentFileAndItsDestination, TheTreatment, WhatOneFrameIsMadeOf,
        };
        use smoozy_job_files_machine::main_output::{APresentFile, ThePicks};
        use smoozy_job_files_machine::reading::{ArtistsText, TheVersionInHand};
        use std::path::PathBuf;
        const THE_FRAMES: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
scene = bpy.context.scene
scene.render.engine = "BLENDER_WORKBENCH"
scene.render.resolution_percentage = 100
scene.render.image_settings.file_format = "PNG"
scene.render.resolution_x = 64
scene.render.resolution_y = 36
scene.render.filepath = out + "/box/box_0001.png"
bpy.ops.render.render(write_still=True)
scene.render.resolution_x = 48
scene.render.resolution_y = 27
scene.render.filepath = out + "/mattress/mattress_0001.png"
bpy.ops.render.render(write_still=True)
"#;
        let blender = "/Applications/Blender.app/Contents/MacOS/Blender";
        let folder = PathBuf::from(std::env::var("SMOOZY_DESK_OUT").expect("SMOOZY_DESK_OUT"));
        std::fs::create_dir_all(&folder).expect("the folder");
        let script = folder.join("frames.py");
        std::fs::write(&script, THE_FRAMES).expect("the script");
        let rendered = Command::new(blender)
            .args(["-b", "--factory-startup", "--python-exit-code", "1", "--python"])
            .arg(&script)
            .arg("--")
            .arg(&folder)
            .stdin(Stdio::null())
            .output()
            .expect("Blender runs");
        assert!(rendered.status.success(), "{}", String::from_utf8_lossy(&rendered.stderr));
        let layer = |name: &str| ArtistsText::of(name.as_bytes().to_vec());
        let a_node_file = |name: &str, fed_by: &str, sits_with_the_main_output: bool| {
            let path = folder.join(format!("{name}/{name}_0001.png"));
            APresentFileAndItsDestination {
                file: APresentFile {
                    path: path.display().to_string().into_bytes(),
                    frame: Some(1),
                    the_picture_it_holds: Some(layer("Image")),
                    format: b"PNG".to_vec(),
                    bytes: std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0),
                    is_video_shaped: false,
                    it_sits_in_the_main_outputs_folder: sits_with_the_main_output,
                },
                multi_layer: false,
                the_destination_it_came_from: smoozy_job_files_machine::main_output::WhereThePictureCameFrom::AFileOutputNode(
                    layer(name),
                ),
                the_view_layers_that_feed_it: vec![layer(fed_by)],
            }
        };
        let present = vec![a_node_file("box", "Box", true), a_node_file("mattress", "Mattress", false)];
        let both = [layer("Box"), layer("Mattress")];
        let record = the_derivatives_record(&present[0], 1, &both);
        assert_eq!(record.the_view_layer_its_picture_came_from, Some(layer("Box")));
        let the_other_layers = the_other_layers_own_copies(
            &record,
            &present,
            &the_rungs_are_told(TheVersionInHand::of(5, 2), None),
            &ThePicks::made_once(None, None),
            None,
            &both,
        );
        assert_eq!(the_other_layers.len(), 1);
        let operation = Identity::issue(IdentityKind::Operation).expect("randomness");
        let streams = folder.join("streams-1");
        std::fs::create_dir_all(&streams).expect("the streams' folder");
        let capture = CaptureRecord {
            job: operation.clone(),
            attempt: 1,
            ordinary_sink: streams.join("ordinary").display().to_string(),
            complaints_sink: streams.join("complaints").display().to_string(),
            ceiling: CaptureCeiling::from(1 << 30),
        };
        smoozy_runtime::capture::MakesTheSinks::make(&smoozy_runtime::acting::capture::TheSinksOnTheMachine, &capture)
            .expect("the sinks");
        let work = ThePreviewsWork {
            operation,
            run: Identity::issue(IdentityKind::Run).expect("randomness"),
            job: Identity::issue(IdentityKind::Job).expect("randomness"),
            frame: 1,
            attempt: 1,
            record_directory: folder.display().to_string(),
            the_job_was_interrupted: false,
            script: streams.join("derivative.py").display().to_string(),
            capture,
        };
        let made_of = WhatOneFrameIsMadeOf {
            the_composited_picture: None,
            the_source: present[0].clone(),
            treatment: TheTreatment::ItAlreadyCarriesItsTreatment,
            the_picture_inside: None,
            record,
            the_other_layers,
        };
        let ending = the_derivative_is_made(&work, blender, &made_of);
        println!("{ending:?}");
        assert_eq!(ending.came_to, WhatTheProductionCameTo::Published, "{ending:?}");
        assert_eq!(ending.the_sources_size, Some((64, 36)), "the main copy is the first layer's file");
        assert_eq!(ending.the_other_layers.len(), 1);
        let other = &ending.the_other_layers[0];
        assert_eq!(other.came_to, WhatTheProductionCameTo::Published, "{other:?}");
        assert_eq!(other.the_sources_size, Some((48, 27)), "the other copy is its own layer's file");
        let named = other.the_record.as_ref().expect("a record");
        assert_eq!(named.the_view_layer_its_picture_came_from, Some(layer("Mattress")));
        assert!(!named.it_is_the_main_copy);
        assert_eq!(named.its_place_in_the_frames_set, 1);
        assert_eq!(ending.the_frames_outcome(), WhatTheProductionCameTo::Published);
        assert!(folder.join("preview-1.png").exists());
        assert_eq!(other.published_at.as_deref(), folder.join("preview-1-layer-1.png").to_str());
    }
}

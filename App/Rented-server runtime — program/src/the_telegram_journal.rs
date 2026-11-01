//! **The Telegram worker reads the machine's journal and tells the phone about
//! jobs** — the third piece of the Telegram worker's map, added 2026-10-16.
//!
//! **What it is.** On every turn of the run, after the routing and the
//! acknowledgement, [`TheJournalReader::a_turn`] reads the journal forward as
//! the Telegram worker (`Rented-server runtime` T2, U27; `State, identity and
//! recovery` E2) over piece 1's own read, does the work each event asks for,
//! and only then moves its own cursor (`Telegram` F3; `State, identity and
//! recovery` E3). Then it looks at whether the machine has had nothing to do
//! for half an hour (`Telegram` V3).
//!
//! **The work, by event** — everything composed by `smoozy-telegram`'s rules,
//! from records other documents own (`Telegram` §2's last paragraph):
//!
//! - **a render's recorded outcome**: finished (`Telegram` E1), failed with
//!   `Rendering` K14a's cause deciding *try it again* (`Telegram` Y4a), given
//!   up, or left stopped — the last through [`TheOwnersActs::report`]
//!   (`Telegram` D2); then the group's one message where the job is a member
//!   (`Telegram` G1, G2); then the waiting message of any hold this job was the
//!   first frame of (`Telegram` A1, A3);
//! - **a hold placed**: all available first-frame copies, followed by the
//!   separate waiting question (`Telegram` A3a), sent once that frame is back — **and nothing for a
//!   hold for the driver** (`Telegram` A3, held-job piece B);
//! - **a hold's release**: which answer was applied, through
//!   [`TheOwnersActs::report`] (`Telegram` A3); a drop, told with its reason
//!   (`Telegram` A4);
//! - **a job record removed**: where the machine dropped a waiting job because
//!   its Blender was removed, told (`Queue & History` L22); a cancel or a clear
//!   is the user's own act and is told nothing;
//! - **a job record written**: where this machine made the job for itself on
//!   a phone's *try it again*, *… was resubmitted* (`Telegram` Y1), from the
//!   map's piece 5;
//! - **a render attempt started**: a separate Started message with Stop,
//!   under every reporting choice, whoever submitted the job (`Telegram` C1a);
//! - **a preview published**, from the map's piece 6: the frame's picture
//!   (`Telegram` I2) where the job's reporting choice sends pictures
//!   (`Telegram` V1) or the user asked for that frame (`Telegram` I4); one
//!   outside Telegram's limits said once for the job (`Telegram` I3); its
//!   caption names the View Layer, or the source, the event carries (`Telegram`
//!   I2, the Creator's tenth notes, note 2). ~~**The main copy only** — never
//!   another View Layer's copy (`Queue & History` P10)~~ **Since 2026-10-18
//!   (the Creator's fourteenth notes, note 7) a frame of several copies goes
//!   as one album, every copy named, on the main copy's event**, from the
//!   copies its production published; the other copies' events send nothing;
//!   an album carries its Stop on a separate companion (`Telegram` C1b);
//! - **a picture every few minutes** (`Telegram` I2a, from 2026-10-19, the
//!   Creator's want of that day): where the job's choice is `Telegram` V1a's fourth
//!   member, its first frame picture goes at once, and then at most one frame's
//!   pictures every `minutes`, always the newest, on the worker's own clock and
//!   in its memory alone ([`ThePacingOfAJob`]); a held frame is dropped at the
//!   job's ending, and *send a frame* is never paced;
//! - **a finished job's own message waits for its last frame's pictures**
//!   (the runner's ruling of 2026-10-19, the live pass's §7), within the wait
//!   the contact sheet already keeps, so the album never arrives after
//!   *finished* ([`TheJournalReader::the_last_frames_picture_is_awaited`]);
//! - **a finished job's contact sheet**, from the map's piece 6: up to ten of
//!   its frames in frame order, ~~its main copies, as one album~~ **since
//!   2026-10-31 (the Creator's seventeenth notes, note 10) one album per View
//!   Layer those frames hold, main copy's first, each naming its layer**
//!   (`Telegram` M2);
//! - **every other event**: nothing, and done.
//!
//! **At least once, never twice in one run.** `State, identity and recovery`
//! `State, identity and recovery` E4: an event is applied by its identity, so one this run has done is never
//! done again however often it is read; a worker killed between a send and the
//! cursor's move sends again when it is next started (`Telegram` G3's standing:
//! never silence, at worst a repeat).
//!
//! **A gap is said** (`State, identity and recovery` E5): where the journal no
//! longer holds what the cursor points at, the phone is told before anything
//! after the gap is.
//!
//! **What is not here, and whose it is**: *try it again*, the stop and the
//! release as commands, which are the command path's
//! ([`crate::the_telegram_worker::TheCommandPathOverTheSupervisor`], pieces 4
//! and 5); the status message, which is its own clock's
//! ([`crate::the_telegram_worker::TheStatusOnItsClock`], piece 6); the moving
//! picture (piece 8).
//!
//! **A drop is told off its own entry from piece 4** (`Rented-server runtime`
//! U20 as dated 2026-10-16), which carries the reason, the held count, the
//! first frame and the held job's label ([`the_drops_entry`]), so a worker
//! started after the hold was placed tells it too.
//!
//! **Nothing here writes a store**: every record is read straight off the
//! package area, read-only, by the crates that own it, and the one durable fact
//! this reader holds — its cursor — is written by the supervisor
//! (`Telegram` F2).

use smoozy_telegram::job_messages::AServerLabel;
use smoozy_telegram::job_messages::{AMarkedText, a_user_message};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use smoozy_framing::framing::Fields;
use smoozy_framing::requests as asked;
use smoozy_job_files_machine::manifest::{FrameRange, SubmissionManifest, MANIFEST_FILE_NAME};
use smoozy_queue_history_machine::acting::note::{
    TheNote, TheOutcomeAsWritten, ThePerJobNotesOnTheMachine, TheStatementAtTheEnding,
    THE_AREA_THE_NOTES_SIT_IN,
};
use smoozy_queue_history_machine::acting::queue_store::{TheQueueOnTheMachine, THE_AREA_THE_QUEUE_SITS_IN};
use smoozy_queue_history_machine::line::{JobRecord, QueueState};
use smoozy_runtime::acting::operation_records::{
    TheOperationRecordsOnTheMachine, THE_AREA_THE_TWO_RECORDS_SIT_IN,
};
use smoozy_runtime::acting::stores::TheHoldRecordsOnTheMachine;
use smoozy_runtime::capture::CollectionOutcome;
use smoozy_runtime::holds::{HoldRecord, KeepsHoldRecords, Subject};
use smoozy_runtime::the_program::the_record_at;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::machine_operations::{Keeps, WhatTheMachineHoldsUnderAnIdentity};
use smoozy_telegram::actions::{
    AJournalledOutcome, AReleasesOutcome, ReadsTheWordsBack, TheJobMessages, TheOwnersActs,
    TheSubject, WhyAMessageWasNotSent,
};
use smoozy_telegram::job_messages::{
    some_events_were_missed, the_drop_for_a_removed_blender, the_drop_message, the_failure_message,
    the_finishing_message, the_given_up_message, the_job_reports, the_start_message,
    the_stops_outcome, what_the_group_says, ACollectionOutcome, AFailedJob, AFinishedJob,
    AGivenUpJob, AKindOfReport, AMember, AMembersEnding, AStopsOutcome, AStream,
    ASubmissionsOutcome, AWaitingJob, TheAnswer, TheCause, TheDependencyEnded, TheIdleMessage,
    TheReportingChoice, WhatTheHoldWaitsFor, ANSWER_A_WAITING_JOB,
};
use smoozy_telegram::job_messages::{a_finished_video_fits, a_size_in_words};
use smoozy_telegram::media::{
    offer_the_contact_sheet, offer_the_finished_video, the_contact_sheet, the_finished_video_is_too_big,
    the_finished_videos_caption,
};
use smoozy_telegram::pacing::{AClassOfSend, HowASendEnded};
use smoozy_telegram::pictures::{
    a_frames_picture_could_not_be_made, the_work_is_done, whether_it_fits, ADerivative, APictureSent, ThePictures,
    WhetherItFits, WhichPictureItIs,
};
use smoozy_telegram::listener::ReadsItsOwnClock;
use smoozy_telegram::transport::WhyTheCallFailed;
use smoozy_telegram::worker_state::TheTelegramWorkersState;

use crate::the_render_worker::{
    OneAttemptsCapture, TheRenderKindsOwnField, THE_FILES_OWN, THE_MACHINES_OWN,
};
use crate::the_telegram_worker::{
    asked as one_request, OneStore, SaysOnItsComplaintsStream, TheFramesAsked, WhyItWasNotAnswered,
    THE_TELEGRAM_WORKERS_READ_FORWARD_REQUEST,
};

// ---------------------------------------------------------------------------
// The records, read off the package area
// ---------------------------------------------------------------------------

/// **`Rendering` S18a's reason for a hold for the driver**, spelt as
/// `smoozy_rendering::submission::THE_DRIVER` spells it. That crate reaches
/// the SSH client and this program may not take it (`Cargo.toml`'s note), so
/// the word is written again here and a test holds the two together.
pub const THE_DRIVERS_REASON: &str = "the driver";

/// **Every record the reader reads, read straight off the package area by the
/// crate that owns it** — the queue, the per-job notes, the hold records, the
/// operation records' capture fields, a job's manifest and its published
/// derivatives. **Read-only**: nothing here makes an area, writes a file or
/// sets a permission (`Telegram` F2; `State, identity and recovery` §1.3 rule
/// 1). A record that cannot be read is nothing, and the message that needed it
/// says what it can or is not sent.
#[derive(Debug, Clone)]
pub struct TheRecordsOnThisMachine {
    package_area: String,
}

/// What the ending actually establishes for a video-only job. The saved
/// statement counts files, not the frames inside a video (`Job files and
/// outputs` I20 and `Job files and outputs` C27; `Queue & History` D10). Its requested range is kept
/// separate from that unknown count, including after a stop.
struct AVideoAtTheEnding {
    /// **The video files the statement found present and not empty**, each
    /// with its size — `Telegram` M10's, added 2026-10-27. `None` where the
    /// ending holds no statement.
    files: Option<Vec<AFinishedVideoFile>>,
    requested: Vec<i64>,
}

/// **`Telegram` M10: one finished video file of a job**, as its ending found
/// it — present and not empty in the completeness statement (`Job files and
/// outputs` C27), named by the inventory's video entry (`Job files and outputs` I19,
/// `Job files and outputs` I20), its size
/// read off the disk when the ending is composed. Added 2026-10-27, the
/// nineteenth notes, note 4.
#[derive(Debug, Clone, PartialEq, Eq)]
struct AFinishedVideoFile {
    path: PathBuf,
    /// The file's own name, as the caption and the messages say it.
    name: String,
    /// `None` where the size could not be read: the file is then neither sent
    /// nor said to be too big.
    bytes: Option<u64>,
}

impl AFinishedVideoFile {
    fn is_too_big(&self) -> bool {
        self.bytes.is_some_and(|bytes| !a_finished_video_fits(bytes))
    }
}

impl AVideoAtTheEnding {
    /// **The finished video's words** — `Telegram` M10, said as a finished
    /// job of pictures says its count and time (*Finished. 240 frames in 6
    /// hours 2 minutes.*), then the video: *Finished. 240 frames in 6 hours 2
    /// minutes. Video on the server (38.2 MB).* `took` is the render time, or
    /// nothing where the note lacks either moment. *(The second words sweep,
    /// 2026-11-01: it read *Finished. Video file on the server (38.2 MB). Job
    /// range: 1–240 (240 frames). Render time: 6 hours 2 minutes.*)*
    fn finished_in_words(&self, took: Option<String>) -> String {
        let frames = self.requested.len() as u64;
        let counted = (frames > 0).then(|| {
            format!(
                "{} {}",
                smoozy_telegram::job_messages::with_thousands(frames),
                if frames == 1 { "frame" } else { "frames" }
            )
        });
        let head = match (counted, took) {
            (Some(counted), Some(took)) => format!("Finished. {counted} in {took}."),
            (Some(counted), None) => format!("Finished. {counted}."),
            (None, Some(took)) => format!("Finished in {took}."),
            (None, None) => "Finished.".to_string(),
        };
        format!("{head} {}", self.files_in_words(true))
    }

    /// **The stopped video's words.** A stopped job sends no video, so its
    /// words say nothing about sending, and only they give the job's range and
    /// add that the video's frame count is unknown.
    fn stopped_in_words(&self) -> String {
        let range = match (self.requested.iter().min(), self.requested.iter().max()) {
            (Some(first), Some(last)) if first == last => format!(" Job range: {first} (1 frame)."),
            (Some(first), Some(last)) => format!(" Job range: {first}–{last} ({} frames).", self.requested.len()),
            _ => " Job range unknown.".to_string(),
        };
        format!("{}{range} Video frame count unknown.", self.files_in_words(false))
    }

    /// **The video files, in words.** `Telegram` M10 adds each file's size and,
    /// at a finished ending — `the_video_goes` — whether a file is too big to
    /// send.
    fn files_in_words(&self, the_video_goes: bool) -> String {
        match &self.files {
            None => "Video file status unknown.".to_string(),
            Some(files) if files.is_empty() => "Video file missing or empty.".to_string(),
            Some(files) => {
                let sizes: Option<Vec<u64>> = files.iter().map(|file| file.bytes).collect();
                let too_big = if the_video_goes {
                    files.iter().filter(|file| file.is_too_big()).count()
                } else {
                    0
                };
                match (files.len(), sizes) {
                    (1, Some(sizes)) if too_big == 1 => format!(
                        "Video on the server ({}), too large to send.",
                        a_size_in_words(sizes[0])
                    ),
                    (1, Some(sizes)) => format!("Video on the server ({}).", a_size_in_words(sizes[0])),
                    (1, None) => "Video on the server.".to_string(),
                    // The inventory names the expected files; video splitting
                    // can leave more (`Job files and outputs` I20), never
                    // fewer than these.
                    (n, sizes) => {
                        let in_all = sizes
                            .map(|sizes| format!(" ({} in all)", a_size_in_words(sizes.iter().sum())))
                            .unwrap_or_default();
                        let big = match too_big {
                            0 => String::new(),
                            1 => " 1 too large to send.".to_string(),
                            k => format!(" {k} too large to send."),
                        };
                        format!("At least {n} video files on the server{in_all}.{big}")
                    }
                }
            }
        }
    }
}

impl TheRecordsOnThisMachine {
    pub fn in_the_package_area(package_area: &Path) -> TheRecordsOnThisMachine {
        TheRecordsOnThisMachine { package_area: package_area.display().to_string() }
    }

    fn at(&self, name: &str) -> PathBuf {
        PathBuf::from(the_record_at(&self.package_area, name))
    }

    fn queue(&self) -> TheQueueOnTheMachine {
        TheQueueOnTheMachine::at(self.at(THE_AREA_THE_QUEUE_SITS_IN))
    }

    fn notes(&self) -> ThePerJobNotesOnTheMachine {
        ThePerJobNotesOnTheMachine::at(self.at(THE_AREA_THE_NOTES_SIT_IN))
    }

    /// The job record, or nothing where none stands.
    pub fn the_job_record(&self, job: &Identity) -> Option<JobRecord> {
        self.queue().one_record(job)
    }

    /// Every job record, or nothing where the queue cannot be read.
    pub fn every_job_record(&self) -> Option<Vec<JobRecord>> {
        self.queue().every_record().ok().map(|(records, _)| records)
    }

    /// The per-job note (`Queue & History` N1).
    pub fn the_note(&self, job: &Identity) -> Option<TheNote> {
        self.notes().read(job).ok().flatten()
    }

    /// **The latest moment any note says a job ended**, in whole seconds on
    /// the machine's clock — where a reader started after the last ending
    /// finds `Queue & History` W2's quiet moment.
    pub fn the_latest_ending_on_any_note(&self) -> Option<u64> {
        let notes = self.notes();
        notes
            .every_job()
            .ok()?
            .iter()
            .filter_map(|job| notes.read(job).ok().flatten()?.ended)
            .filter_map(|ended| seconds_of(ended.taken_at()))
            .max()
    }

    /// **`Telegram` R13a and `Telegram` C1c — the newest frame the job's derivative set
    /// holds** (`Telegram` I2): the last of the job's frames, in its own order, whose
    /// picture is published in its record directory. Nothing where the record,
    /// its manifest or every picture is gone. Added 2026-10-31, the Telegram
    /// discussion, the server half.
    pub fn the_newest_frame_held(&self, job: &Identity) -> Option<i64> {
        let record = self.the_job_record(job)?;
        let manifest = self.the_manifest(&record)?;
        self.the_frames_of(&record, &manifest).into_iter().rev().find(|frame| {
            Path::new(
                &smoozy_job_files_machine::main_output::where_a_derivative_is_written_in(&record.record_directory, *frame)
                    .published_at,
            )
            .is_file()
        })
    }

    /// **`Telegram` R1b and R1 — whether this machine still holds a job's
    /// record folder**, which is how long the job's register lines and
    /// action-context records are kept.
    pub fn holds_the_record_folder_of(&self, job: &str) -> bool {
        Identity::read_back(IdentityKind::Job, job)
            .and_then(|job| self.the_job_record(&job))
            .is_some_and(|record| Path::new(&record.record_directory).is_dir())
    }

    /// The job's manifest (`Job files and outputs` M1), in its record directory.
    pub fn the_manifest(&self, record: &JobRecord) -> Option<SubmissionManifest> {
        let bytes = std::fs::read(format!("{}/{MANIFEST_FILE_NAME}", record.record_directory)).ok()?;
        SubmissionManifest::read_back(&bytes)
    }

    /// **`Telegram` V1c: the job's choice in force** — the job record's current
    /// choice where the desk changed it, and the immutable manifest's field
    /// where it never did. Read at every event, so a change counts from the
    /// first event read after it. One that cannot be read is the narrowest
    /// (`Telegram` V1a).
    pub fn the_choice_in_force(&self, record: &JobRecord) -> TheReportingChoice {
        let manifest = self.the_manifest(record);
        TheReportingChoice::in_force(
            record.reporting_choice_now.as_deref(),
            manifest.as_ref().map_or(&serde_json::Value::Null, |manifest| &manifest.reporting_settings.0),
        )
    }

    /// `Telegram` V1b: silence, read from the choice in force (`Telegram` V1c).
    pub fn the_record_reports_nothing(&self, record: &JobRecord) -> bool {
        self.the_choice_in_force(record) == TheReportingChoice::Nothing
    }

    /// Either half of a first-frame check silences its automatic messages.
    pub fn the_hold_reports_nothing(&self, hold: &HoldRecord, held: &JobRecord) -> bool {
        self.the_record_reports_nothing(held) || hold.dependency.as_ref()
            .and_then(|dependency| self.the_job_record(&dependency.work))
            .is_some_and(|first| self.the_record_reports_nothing(&first))
    }

    pub fn the_job_reports_nothing(&self, record: &JobRecord) -> bool {
        if self.the_record_reports_nothing(record) { return true; }
        if let QueueState::Held { hold } = &record.state {
            if self.the_hold(hold).is_some_and(|hold| self.the_hold_reports_nothing(&hold, record)) {
                return true;
            }
        }
        self.every_job_record().is_some_and(|every| every.iter().any(|held| {
            let QueueState::Held { hold } = &held.state else { return false };
            self.the_hold(hold).is_some_and(|hold| {
                hold.dependency.as_ref().is_some_and(|dependency| dependency.work == record.job)
                    && self.the_hold_reports_nothing(&hold, held)
            })
        }))
    }

    /// The hold record (`Rented-server runtime` U2), reached by its identity.
    pub fn the_hold(&self, hold: &Identity) -> Option<HoldRecord> {
        TheHoldRecordsOnTheMachine {
            area: self.at(smoozy_runtime::the_program::records::THE_AREA_THE_HOLD_RECORDS_SIT_IN),
        }
        .read(hold)
        .ok()
        .flatten()
    }

    /// **Every attempt's capture record of the job's render**, reached through
    /// the identity its queue state carries — the render while it runs, the
    /// recorded outcome once it has ended (`Queue & History` Q2) — and the
    /// render kind's own field on that operation record, where the worker's
    /// report and `Rendering` K14a's cause are kept.
    pub fn the_attempts(&self, job: &Identity) -> Option<Vec<OneAttemptsCapture>> {
        let operation = match self.the_job_record(job)?.state {
            QueueState::Ended { outcome } => outcome,
            QueueState::Rendering { operation } => operation,
            _ => return None,
        };
        let records = TheOperationRecordsOnTheMachine { area: self.at(THE_AREA_THE_TWO_RECORDS_SIT_IN) };
        match records.what_is_held_under(&operation).ok()? {
            WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record) => Some(
                record
                    .the_kinds_safe_boundary_marker
                    .as_deref()
                    .and_then(TheRenderKindsOwnField::read_back)?
                    .attempts,
            ),
            _ => None,
        }
    }

    /// `Telegram` R1 and `Telegram` D0: a re-run may share its operation identity. Its attempt number
    /// therefore belongs in the context generation as well.
    pub fn action_generation(&self, record: &JobRecord) -> String {
        match record.state {
            QueueState::Rendering { .. } | QueueState::Ended { .. } => format!(
                "{}:{}",
                the_generation_of(record),
                self.the_note(&record.job).map_or(0, |note| note.attempts)
            ),
            _ => the_generation_of(record),
        }
    }

    /// **The frames a job covers**, in order: the user's own integers, or —
    /// where the manifest carries the file's own range — the reading in force
    /// for its first attempt. Empty where neither can be read, and no frame is
    /// then guessed.
    pub fn the_frames_of(&self, record: &JobRecord, manifest: &SubmissionManifest) -> Vec<i64> {
        match &manifest.frame_range {
            FrameRange::TheUsersParsedIntegers(frames) => frames.clone(),
            FrameRange::TheFilesOwn => {
                crate::the_render_worker::the_reading_in_force_for_the_job_in(&record.record_directory, 1)
                    .map(|in_force| in_force.whole_range)
                    .unwrap_or_default()
            }
        }
    }

    /// **The frames on the server, and the frames not rendered, at an
    /// ending** — the frames `Job files and outputs` C27's statement counts
    /// already there against the reading in force, and the rest of the job's
    /// range. Where the reading cannot be read, the statement's present and
    /// absent files stand in, one file to a frame.
    fn the_frames_at_the_ending(&self, record: &JobRecord, note: &TheNote) -> (u64, u64) {
        let Some(TheStatementAtTheEnding::It(statement)) = &note.the_completeness_statement else {
            return (0, 0);
        };
        let attempt = note.ending.as_ref().map_or(1, |ending| ending.the_attempt_whose_capture_record_holds_the_words.max(1));
        if let Ok(in_force) =
            crate::the_render_worker::the_reading_in_force_for_the_job_in(&record.record_directory, attempt)
        {
            if let smoozy_job_files_machine::inventory::WhatTheInventoryIs::It(inventory) = &in_force.inventory {
                let there = smoozy_job_files_machine::reckoning::the_frames_the_statement_counts_already_there(
                    inventory,
                    statement,
                    &in_force.whole_range,
                    &in_force.job_output_folder,
                )
                .len() as u64;
                let all = in_force.whole_range.len() as u64;
                return (there, all.saturating_sub(there));
            }
        }
        (statement.present.len() as u64, statement.absent.len() as u64)
    }

    /// **The sixteenth notes, piece 1: a video-only job owes no frame
    /// pictures.** Use the ending attempt's inventory, the same one the
    /// supervisor uses for `Queue & History` P1b. A video beside image outputs
    /// still owes pictures. An unreadable inventory proves neither case.
    fn the_video_at_the_ending(&self, record: &JobRecord) -> Option<AVideoAtTheEnding> {
        let note = self.the_note(&record.job)?;
        let attempt = note.ending.as_ref()?.the_attempt_whose_capture_record_holds_the_words.max(1);
        let in_force = crate::the_render_worker::the_reading_in_force_for_the_job_in(&record.record_directory, attempt).ok()?;
        let smoozy_job_files_machine::inventory::WhatTheInventoryIs::It(inventory) = in_force.inventory else {
            return None;
        };
        if !inventory.a_destination_is_video_shaped || inventory.entries.iter().any(|entry| entry.frame.is_some()) {
            return None;
        }
        let files = Self::the_video_files_in(&inventory, &note);
        Some(AVideoAtTheEnding { files, requested: in_force.whole_range })
    }

    /// **`Telegram` M10: the finished video files of any job that writes a
    /// video** — a video-only job's, or the video of a job that writes
    /// pictures as well — read as [`Self::the_video_at_the_ending`] reads
    /// them. Empty where the job writes no video; `None` where nothing can be
    /// read. Added 2026-10-27, the nineteenth notes, note 4.
    fn the_finished_videos(&self, record: &JobRecord) -> Option<Vec<AFinishedVideoFile>> {
        let note = self.the_note(&record.job)?;
        let attempt = note.ending.as_ref()?.the_attempt_whose_capture_record_holds_the_words.max(1);
        let in_force = crate::the_render_worker::the_reading_in_force_for_the_job_in(&record.record_directory, attempt).ok()?;
        let smoozy_job_files_machine::inventory::WhatTheInventoryIs::It(inventory) = in_force.inventory else {
            return None;
        };
        if !inventory.a_destination_is_video_shaped {
            return Some(Vec::new());
        }
        Self::the_video_files_in(&inventory, &note)
    }

    /// **The inventory's video entries — the ones carrying no frame (`Job
    /// files and outputs` I20) — that the completeness statement found present
    /// and not empty**, each once, in name order, with its size.
    fn the_video_files_in(
        inventory: &smoozy_job_files_machine::inventory::TheInventory,
        note: &TheNote,
    ) -> Option<Vec<AFinishedVideoFile>> {
        use std::os::unix::ffi::OsStrExt;
        let Some(TheStatementAtTheEnding::It(statement)) = &note.the_completeness_statement else {
            return None;
        };
        let mut paths: Vec<&Vec<u8>> = inventory
            .entries
            .iter()
            .filter(|entry| entry.frame.is_none())
            .map(|entry| &entry.predicted_path)
            .filter(|path| statement.present.contains(path) && !statement.present_at_zero_bytes.contains(path))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        paths.sort();
        Some(
            paths
                .into_iter()
                .map(|bytes| {
                    let path = PathBuf::from(std::ffi::OsStr::from_bytes(bytes));
                    AFinishedVideoFile {
                        name: path
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_default(),
                        bytes: std::fs::metadata(&path).ok().map(|found| found.len()),
                        path,
                    }
                })
                .collect(),
        )
    }

    /// **A held job's name and its count of frames**, read off its records —
    /// the note's naming, the `.blend`'s where the note has none, and the
    /// frames its manifest covers. Empty and nought where neither reads.
    pub fn the_label_and_count_of(&self, hold: &HoldRecord) -> (String, u64) {
        let Subject::Job(job) = &hold.subject else {
            return (String::new(), 0);
        };
        let Some(record) = self.the_job_record(job) else {
            return (String::new(), 0);
        };
        let manifest = self.the_manifest(&record);
        let label = match (self.the_note(job), manifest.as_ref()) {
            (Some(note), manifest) => note.what_the_job_is_called(manifest),
            (None, Some(manifest)) => smoozy_queue_history_machine::naming::the_files_name(&manifest.blend_path_on_the_machine),
            (None, None) => String::new(),
        };
        let held = manifest.as_ref().map_or(0, |manifest| self.the_frames_of(&record, manifest).len() as u64);
        (label, held)
    }

    /// **The first frame a hold waits on** — its dependency's first frame,
    /// or failing one, the held job's own.
    pub fn the_first_frame_of(&self, hold: &HoldRecord) -> Option<i64> {
        let of = |job: &Identity| {
            let record = self.the_job_record(job)?;
            let manifest = self.the_manifest(&record)?;
            self.the_frames_of(&record, &manifest).first().copied()
        };
        hold.dependency
            .as_ref()
            .and_then(|dependency| of(&dependency.work))
            .or_else(|| match &hold.subject {
                Subject::Job(job) => of(job),
                Subject::CommandAwaitingAPerson(_) => None,
            })
    }

    /// **What a job is called**, the way every message names it: the note's
    /// label, or the `.blend`'s name where the note has none.
    pub fn what_the_job_is_called(&self, job: &Identity) -> String {
        let record = self.the_job_record(job);
        let manifest = record.as_ref().and_then(|record| self.the_manifest(record));
        match (self.the_note(job), manifest.as_ref()) {
            (Some(note), manifest) => note.what_the_job_is_called(manifest),
            (None, Some(manifest)) => smoozy_queue_history_machine::naming::the_files_name(&manifest.blend_path_on_the_machine),
            (None, None) => String::new(),
        }
    }

    /// **Which of the frame's pictures its published main copy is, as the
    /// preview record it was published from says** (`Telegram` I2, the
    /// Creator's tenth notes, note 2; piece T2c of step 10c) — for a picture
    /// sent straight from the set, where no event is read. The record keeps
    /// the main copy's own `Job files and outputs` B31a record, which is what
    /// the event was composed from, so the caption reads as one sent off the
    /// event does. Read from every preview operation record of the job
    /// ([`which_picture_the_records_name`]); nothing where none reads.
    pub fn which_picture_the_preview_record_names(&self, job: &Identity, frame: i64) -> Option<WhichPictureItIs> {
        let records = TheOperationRecordsOnTheMachine { area: self.at(THE_AREA_THE_TWO_RECORDS_SIT_IN) };
        let productions = records
            .every_operation_record_of_the_kind(smoozy_runtime::the_program::the_word_for(
                smoozy_runtime::workers::WorkerKind::Preview,
            ))
            .ok()?;
        which_picture_the_records_name(&productions, job, frame)
    }

    /// **`Telegram` I2 as written 2026-10-18 — every copy of a frame the set
    /// holds, in its place in the frame's set**: the main copy first, named
    /// `main_named`, then each other View Layer's copy in place order (`Queue &
    /// History` P1, P4), named as its preview record names it
    /// ([`the_other_copies_the_records_name`]), read at its published name
    /// (`Job files and outputs`' `where_a_layers_copy_is_written_in`).
    /// `production` is the production the album goes for — the main copy's
    /// event's operation — and none for *send a frame*, which reads the set.
    /// **Nothing where the main copy is not published**: that is *cleared*,
    /// and the route that makes it again is the caller's (`Telegram` I4). An
    /// other copy whose file is gone leaves the whole frame cleared, so a
    /// requested album waits for the preview route (`Telegram` I4). A frame
    /// of one copy, or one whose production has no record here, gives the
    /// main copy alone, as before.
    pub fn the_frames_copies(
        &self,
        record: &JobRecord,
        job: &Identity,
        label: &str,
        frame: i64,
        main_named: Option<WhichPictureItIs>,
        production: Option<&str>,
    ) -> Option<Vec<ADerivative>> {
        let mut main = self.the_published_picture(record, job, label, frame)?;
        if main.bytes.is_empty() {
            return None;
        }
        main.which_picture = main_named;
        let mut copies = vec![main];
        let records = TheOperationRecordsOnTheMachine { area: self.at(THE_AREA_THE_TWO_RECORDS_SIT_IN) };
        let productions = records
            .every_operation_record_of_the_kind(smoozy_runtime::the_program::the_word_for(
                smoozy_runtime::workers::WorkerKind::Preview,
            ))
            .ok()?;
        for (place, which_picture) in the_other_copies_the_records_name(&productions, job, frame, production) {
            let at = smoozy_job_files_machine::main_output::where_a_layers_copy_is_written_in(
                &record.record_directory,
                frame,
                place,
            )
            .published_at;
            let bytes = std::fs::read(&at).ok()?;
            if bytes.is_empty() {
                return None;
            }
            copies.push(ADerivative {
                job: job.as_text(),
                job_label: label.to_string(),
                frame: u64::try_from(frame).unwrap_or(0),
                file_name: format!("{frame}-layer-{place}.png"),
                bytes,
                which_picture,
            });
        }
        Some(copies)
    }

    /// `Telegram` A3a sends every available copy, even if another copy was cleared.
    /// The ordinary requested-frame path still requires its whole recorded set.
    fn available_check_copies(
        &self,
        record: &JobRecord,
        job: &Identity,
        label: &str,
        frame: i64,
    ) -> Vec<ADerivative> {
        let records = TheOperationRecordsOnTheMachine {
            area: self.at(THE_AREA_THE_TWO_RECORDS_SIT_IN),
        };
        let productions = records
            .every_operation_record_of_the_kind(smoozy_runtime::the_program::the_word_for(
                smoozy_runtime::workers::WorkerKind::Preview,
            ))
            .unwrap_or_default();
        let mut copies = Vec::new();
        if let Some(mut main) = self.the_published_picture(record, job, label, frame) {
            if !main.bytes.is_empty() {
                main.which_picture = the_main_copy_named(&productions, job, frame, true);
                copies.push(main);
            }
        }
        for (place, which_picture) in
            the_other_copies_named(&productions, job, frame, None, true)
        {
            let at = smoozy_job_files_machine::main_output::where_a_layers_copy_is_written_in(
                &record.record_directory,
                frame,
                place,
            )
            .published_at;
            if let Ok(bytes) = std::fs::read(at) {
                if !bytes.is_empty() {
                    copies.push(ADerivative {
                        job: job.as_text(),
                        job_label: label.into(),
                        frame: u64::try_from(frame).unwrap_or(0),
                        file_name: format!("{frame}-layer-{place}.png"),
                        bytes,
                        which_picture,
                    });
                }
            }
        }
        copies
    }

    /// **`Telegram` M2 as changed 2026-10-31 — every copy of each frame the
    /// contact sheet chose**, in frame order, read as `available_check_copies`
    /// reads a check's: the main copy where it is published, then each other
    /// View Layer's published copy in place order, each with the name its
    /// preview records give it (`Job files and outputs` B31a; no source file
    /// name, which differs frame by frame and would match nothing). Only
    /// where each file sits is read here, not its bytes. With each frame,
    /// whether every copy it owes is in ([`every_copy_of_the_frame_is_in`]),
    /// which the sheet's wait reads.
    pub fn the_copies_of_the_sheets_frames(
        &self,
        record: &JobRecord,
        job: &Identity,
        frames: &[i64],
    ) -> Vec<TheCopiesOfAFrameOnTheSheet> {
        let records = TheOperationRecordsOnTheMachine {
            area: self.at(THE_AREA_THE_TWO_RECORDS_SIT_IN),
        };
        let productions = records
            .every_operation_record_of_the_kind(smoozy_runtime::the_program::the_word_for(
                smoozy_runtime::workers::WorkerKind::Preview,
            ))
            .unwrap_or_default();
        let there = |at: &str| std::fs::metadata(at).is_ok_and(|file| file.is_file() && file.len() > 0);
        frames
            .iter()
            .map(|&frame| {
                let mut copies = Vec::new();
                let main_at = smoozy_job_files_machine::main_output::where_a_derivative_is_written_in(
                    &record.record_directory,
                    frame,
                )
                .published_at;
                let its_main_copy_is_published = there(&main_at);
                if its_main_copy_is_published {
                    copies.push(ACopyOnTheSheet {
                        frame,
                        place: None,
                        layer: the_main_copy_named(&productions, job, frame, false),
                        at: main_at,
                        file_name: format!("{frame}.png"),
                    });
                }
                for (place, layer) in the_other_copies_named(&productions, job, frame, None, false) {
                    let at = smoozy_job_files_machine::main_output::where_a_layers_copy_is_written_in(
                        &record.record_directory,
                        frame,
                        place,
                    )
                    .published_at;
                    if there(&at) {
                        copies.push(ACopyOnTheSheet {
                            frame,
                            place: Some(place),
                            layer,
                            at,
                            file_name: format!("{frame}-layer-{place}.png"),
                        });
                    }
                }
                let its_pictures_are_in =
                    every_copy_of_the_frame_is_in(&productions, job, frame, its_main_copy_is_published);
                TheCopiesOfAFrameOnTheSheet { copies, its_pictures_are_in }
            })
            .collect()
    }

    /// **Whether every copy one frame of a job owes is in**, off the preview
    /// operations' own records and the main copy's published file
    /// ([`every_copy_of_the_frame_is_in`]) — what a first-frame check's
    /// question waits on (`Telegram` A3a; the live Telegram pass's fault 5).
    pub fn every_copy_of_the_frame_is_in_for(&self, record: &JobRecord, job: &Identity, frame: i64) -> bool {
        let records = TheOperationRecordsOnTheMachine { area: self.at(THE_AREA_THE_TWO_RECORDS_SIT_IN) };
        let productions = records
            .every_operation_record_of_the_kind(smoozy_runtime::the_program::the_word_for(
                smoozy_runtime::workers::WorkerKind::Preview,
            ))
            .unwrap_or_default();
        let main_at =
            smoozy_job_files_machine::main_output::where_a_derivative_is_written_in(&record.record_directory, frame)
                .published_at;
        let its_main_copy_is_published = std::fs::metadata(&main_at).is_ok_and(|file| file.is_file() && file.len() > 0);
        every_copy_of_the_frame_is_in(&productions, job, frame, its_main_copy_is_published)
    }

    /// **What each production of one frame of a job came to**, off the
    /// preview operations' own records: `None` for one still standing, and
    /// its outcome for one that ended. Empty where none is recorded, or the
    /// records cannot be read. For a finished job's own message, which waits
    /// for its last frame's pictures only where one is coming.
    pub fn what_the_productions_of_a_frame_came_to(
        &self,
        job: &Identity,
        frame: i64,
    ) -> Vec<Option<crate::the_preview_worker::WhatTheProductionCameTo>> {
        let records = TheOperationRecordsOnTheMachine { area: self.at(THE_AREA_THE_TWO_RECORDS_SIT_IN) };
        records
            .every_operation_record_of_the_kind(smoozy_runtime::the_program::the_word_for(
                smoozy_runtime::workers::WorkerKind::Preview,
            ))
            .unwrap_or_default()
            .iter()
            .filter(|record| &record.subject == job)
            .filter_map(|record| {
                crate::the_preview_worker::ThePreviewKindsOwnField::read_back(
                    record.the_kinds_safe_boundary_marker.as_deref()?,
                )
            })
            .filter(|field| field.frame == frame)
            .map(|field| field.ending.map(|ending| ending.came_to))
            .collect()
    }

    /// **A frame's published derivative** (`Job files and outputs` B22), or
    /// nothing where none is published yet. Which of the frame's pictures it
    /// is, is not read here: a caller holding the published event hands it on
    /// ([`which_picture_the_event_names`]), and a caller reading the set reads
    /// the preview record ([`Self::which_picture_the_preview_record_names`]).
    pub fn the_published_picture(&self, record: &JobRecord, job: &Identity, label: &str, frame: i64) -> Option<ADerivative> {
        let at = smoozy_job_files_machine::main_output::where_a_derivative_is_written_in(&record.record_directory, frame)
            .published_at;
        let bytes = std::fs::read(&at).ok()?;
        Some(ADerivative {
            job: job.as_text(),
            job_label: label.to_string(),
            frame: u64::try_from(frame).unwrap_or(0),
            file_name: format!("{}.png", frame),
            bytes,
            which_picture: None,
        })
    }
}

/// **`Rented-server runtime` M11 — a render's captured stream, read back
/// whole or not at all**, from the sink its attempt's capture record names.
/// Nothing is trimmed, prefixed or redacted (M8) and nothing is mined (M24).
impl ReadsTheWordsBack for TheRecordsOnThisMachine {
    fn read_whole(&self, job: &str, attempt: u32, stream: AStream) -> Result<Vec<u8>, String> {
        let job = Identity::read_back(IdentityKind::Job, job).ok_or("the button names no job")?;
        let attempts = self.the_attempts(&job).ok_or("no capture record is held for that job")?;
        let one = attempts
            .iter()
            .find(|one| one.attempt == attempt)
            .ok_or_else(|| format!("no capture record is held for attempt {attempt}"))?;
        let sink = match stream {
            AStream::TheOrdinaryOutput => &one.ordinary_sink,
            AStream::TheComplaints => &one.complaints_sink,
        };
        std::fs::read(sink).map_err(|why| format!("the captured stream could not be read whole: {:?}", why.kind()))
    }
}

fn seconds_of(moment: SystemTime) -> Option<u64> {
    moment.duration_since(UNIX_EPOCH).ok().map(|since| since.as_secs())
}

/// **`Rented-server runtime` M13's outcome for one stream of one attempt**,
/// as the crate's own value: what the worker reported, *collection incomplete*
/// where reconciliation found the attempt cut (M17), and *never established*
/// where nothing was recorded at all.
fn what_the_stream_came_to(attempt: Option<&OneAttemptsCapture>, stream: AStream) -> ACollectionOutcome {
    let Some(attempt) = attempt else {
        return ACollectionOutcome::NeverEstablished;
    };
    if attempt.collection_incomplete_by_reconciliation {
        return ACollectionOutcome::CollectionIncomplete;
    }
    let words = attempt.ending.as_ref().and_then(|ending| match stream {
        AStream::TheOrdinaryOutput => ending.ordinary.as_deref(),
        AStream::TheComplaints => ending.complaints.as_deref(),
    });
    match words.and_then(CollectionOutcome::from_words) {
        Some(CollectionOutcome::Collected) => ACollectionOutcome::Collected,
        Some(CollectionOutcome::CollectedAndTheStreamWasEmpty) => ACollectionOutcome::CollectedAndTheStreamWasEmpty,
        Some(CollectionOutcome::CollectionIncomplete) => ACollectionOutcome::CollectionIncomplete,
        _ => ACollectionOutcome::NeverEstablished,
    }
}

/// **`Rendering` K14a's cause, read where the worker's report put it** — the
/// render kind's own field on the operation record. **Where none is recorded
/// it is *undetermined***, which takes no *try it again* (`Telegram` Y4a).
fn the_cause_of(attempt: Option<&OneAttemptsCapture>) -> TheCause {
    match attempt.and_then(|attempt| attempt.ending.as_ref()?.cause.as_deref()) {
        Some(THE_MACHINES_OWN) => TheCause::TheMachinesOwn,
        Some(THE_FILES_OWN) => TheCause::TheFilesOwn,
        _ => TheCause::Undetermined,
    }
}

// ---------------------------------------------------------------------------
// One read forward
// ---------------------------------------------------------------------------

/// **One event, as the read forward carries it** (`State, identity and
/// recovery` E1, `State, identity and recovery` E7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnEventRead {
    pub sequence: u64,
    /// `State, identity and recovery` E4's key.
    pub event: String,
    pub kind: String,
    pub about: Option<String>,
    pub says: String,
    /// Whole seconds on the machine's clock.
    pub happened_at: Option<u64>,
}

/// **One read forward**, in the desktop's own words, which piece 1's request
/// answers in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheReadOffered {
    pub events: Vec<AnEventRead>,
    /// `State, identity and recovery` E5's number.
    pub oldest_retained: Option<u64>,
    pub next_sequence: u64,
}

/// The read forward's answer, read back. **Nothing where it is not one.**
pub fn what_the_read_offered(fields: &Fields) -> Option<WhatTheReadOffered> {
    let number = |name: &str| fields.text(name).and_then(|text| text.parse::<u64>().ok());
    let how_many = number(asked::HOW_MANY_EVENTS_FIELD)?;
    let next_sequence = number(asked::THE_NEXT_SEQUENCE_FIELD)?;
    let mut events = Vec::new();
    for at in 0..how_many as usize {
        let item = |name: &str| fields.text(&asked::the_field_of_item(asked::AN_EVENT, at, name)).map(str::to_string);
        events.push(AnEventRead {
            sequence: item(asked::THE_SEQUENCE_FIELD)?.parse().ok()?,
            event: item(asked::THE_EVENT_IDENTITY_FIELD)?,
            kind: item(asked::THE_KIND_FIELD)?,
            about: item(asked::THE_ABOUT_FIELD),
            says: item(asked::THE_SAYS_FIELD).unwrap_or_default(),
            happened_at: item(asked::THE_HAPPENED_AT_FIELD).and_then(|text| text.parse().ok()),
        });
    }
    Some(WhatTheReadOffered { events, oldest_retained: number(asked::THE_OLDEST_RETAINED_FIELD), next_sequence })
}

// ---------------------------------------------------------------------------
// The reader
// ---------------------------------------------------------------------------

/// **How long a waiting message waits for the first frame's picture** once
/// that frame's job has finished — the preview is made after the ending
/// (`Queue & History` P1b), so it is usually not there yet when the ending is
/// read. Past this the question goes as text with its buttons, which the
/// crate's own rule already allows, because the answer is owed either way. **A
/// Build figure and nothing is sized on it**; later events are not held for
/// it — only the cursor is.
pub const HOW_LONG_A_WAITING_MESSAGE_WAITS_FOR_ITS_PICTURE: Duration = Duration::from_secs(120);

/// **How long a finished job's contact sheet waits for its pictures** once the
/// job has ended — the previews of its last frames are made after the ending
/// (`Queue & History` P1b), one at a time. Past this the sheet goes with the
/// pictures that are there. **A Build figure, the map's piece 6, and nothing
/// is sized on it**; later events are not held for it — only the cursor is.
pub const HOW_LONG_A_CONTACT_SHEET_WAITS_FOR_ITS_PICTURES: Duration = Duration::from_secs(10 * 60);

/// **`Telegram` I2a — one paced job, as this run holds it**: the moment its
/// last frame picture was sent, on the worker's own clock (the listener's
/// monotonic one, never the wall clock and never the desktop's: `State,
/// identity and recovery` D5), and the one frame held since. **Memory only**:
/// nothing about either is written to the worker's state store (`Telegram`
/// F2), so a restart forgets both and the job's next frame goes at once.
#[derive(Debug, Clone, Default)]
struct ThePacingOfAJob {
    /// None before the job's first picture went in this run.
    last_sent: Option<Duration>,
    held: Option<APacedFrame>,
}

/// **A frame whose pictures wait for their turn** — what `Telegram` I2 reads to send
/// them: the frame, the production its main copy's event named, and which of
/// the frame's pictures that copy is.
#[derive(Debug, Clone, PartialEq)]
struct APacedFrame {
    frame: i64,
    operation: String,
    which_picture: Option<WhichPictureItIs>,
}

/// **`Queue & History` W3's half hour.**
const THE_HALF_HOUR: u64 = 30 * 60;

/// **V2's switch for the idle message**, by the name the desktop stores it
/// under (`Desktop app and delivery`'s `record::settings`, *a machine has been
/// idle*) and the told-record carries in the role configuration.
pub const THE_IDLE_SWITCH: &str = "a-server-has-had-nothing-to-do";

/// **What doing one event's work came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
enum TheWork {
    /// Done: its messages went, or it asked for none.
    Done,
    /// Not yet: the line reached nothing. `Telegram` F3a keeps this event owed while
    /// later events go on, and bounds its tries only while other calls reach.
    TheLineReachedNothing,
    /// Not yet, but later events go on: a waiting message waits for its
    /// picture. The cursor stays at this event.
    NotYet,
}

/// `Telegram` F3a/`Telegram` R7a's notice, with offsets counted as Telegram's entity calls require.
fn the_unsent_notice(server: &(impl smoozy_telegram::job_messages::TheServersName + ?Sized), job: &str) -> (String, serde_json::Value) {
    let text = a_user_message(server, job, "", "A message about this job could not be sent.");
    (text.text.clone(), text.entities())
}

/// `Telegram` F3a: a failed send counts only after another call reached Telegram.
#[derive(Default)]
struct TheTriesOfASend {
    counted: u8,
    last_reach: Option<u64>,
}
impl TheTriesOfASend {
    fn after_a_failure(&mut self, reached: u64) -> bool {
        if self.last_reach.is_some_and(|last| reached > last) { self.counted += 1; }
        self.last_reach = Some(reached);
        self.counted >= 3
    }
}

struct AnUnsentNotice {
    event: AnEventRead,
    tries: TheTriesOfASend,
}

/// **The Telegram worker's journal reader.** One per run.
pub struct TheJournalReader {
    supervisor: Arc<dyn crate::the_telegram_worker::AsksTheSupervisor>,
    refused: Arc<AtomicBool>,
    store: OneStore,
    messages: TheJobMessages,
    /// **`TheOwnersActs::report`'s home for the outcomes it tells** — the
    /// stop's, the release's and, from piece 5, the resubmission's.
    reporter: TheOwnersActs,
    records: TheRecordsOnThisMachine,
    /// The machine's identity, and the label it is named by in the chat.
    machine: String,
    server: AServerLabel,
    the_idle_switch: bool,
    the_machines_now: Arc<dyn Fn() -> SystemTime + Send + Sync>,
    /// **`State, identity and recovery` E4: every part of every event this run has done**, by the event's
    /// identity and the part's number, so nothing is said twice in one run.
    done: HashSet<String>,
    /// **Where the next read starts** — past every event read, done or owed;
    /// the durable cursor is the store's and moves only to the first event
    /// not yet done.
    read_from: Option<u64>,
    /// Events owed their work, by sequence — read before and not done yet.
    owed: BTreeMap<u64, AnEventRead>,
    idle: TheIdleMessage,
    /// `Queue & History` W2's quiet moment, in seconds on the machine's clock.
    quiet_since: Option<u64>,
    /// **What this run read of each hold awaiting the user when it was
    /// placed** — the held job's name, its count and the first frame — because
    /// a *no* and a drop remove the held job's records and its hold record in
    /// the same act as the release (`Rented-server runtime` U12a; `Queue &
    /// History` H9, `Queue & History` H11). Memory only: a worker started
    /// between the two has nothing of it. **A drop's own entry is preferred
    /// since piece 4** ([`what_a_drops_entry_says`]); this fills what an entry
    /// an older program wrote leaves out, and a *no*'s release, which carries
    /// nothing.
    remembered: HashMap<String, (String, u64, Option<i64>)>,
    /// **The map's piece 6: frame pictures through one keeper of the once
    /// per job of `Telegram` I3**, for the whole run.
    pictures: ThePictures,
    requested_pictures: HashMap<(String, i64), ThePictures>,
    held_subjects: HashMap<String, TheSubject>,
    last_drop: Option<(u64, String)>,
    /// **The frames the user asked for whose picture is being made again**
    /// (`Telegram` I4), shared with the owner's path.
    frames: TheFramesAsked,
    /// **A contact sheet offered to the upload lane and not ended yet**, by
    /// its event — offered and not waited for, since a turn held for an upload
    /// is a heartbeat held too (`Telegram` C6b).
    sheets_in_flight: HashMap<String, std::sync::mpsc::Receiver<HowASendEnded>>,
    /// **The jobs one of whose contact sheets Telegram refused**, by the
    /// job's identity, so the chat is told once for the job (`Telegram` M2,
    /// `Telegram` I3; the live Telegram pass's fault 3). Memory only: a restart may say
    /// it once more, never nothing.
    sheets_refused: HashSet<String>,
    /// **The worker's own clock** — the listener's, monotonic — which
    /// `Telegram` I2a counts its minutes on.
    clock: Arc<dyn ReadsItsOwnClock>,
    /// **`Telegram` I2a: every paced job this run has sent or held a frame
    /// of**, by the job's identity. Memory only.
    paced: HashMap<String, ThePacingOfAJob>,
    /// **The frames whose main copy's event this run has done**, sent, held
    /// or not sent, by job and frame — what a finished job's own message waits
    /// on for its last frame ([`Self::the_last_frames_picture_is_awaited`]).
    pictures_done: HashSet<(String, i64)>,
    tries: HashMap<u64, TheTriesOfASend>,
    unsent_notices: BTreeMap<u64, AnUnsentNotice>,
    holds_failed_this_turn: HashSet<String>,
    notes: SaysOnItsComplaintsStream,
    /// **A test's way to pose a worker killed between its sends and its
    /// cursor's move**: the next turn sends and then moves nothing.
    #[cfg(test)]
    pub the_next_cursor_move_is_lost: bool,
}

/// What the reader is handed, by the assembly.
pub struct WhatTheReaderIsHanded {
    pub supervisor: Arc<dyn crate::the_telegram_worker::AsksTheSupervisor>,
    pub refused: Arc<AtomicBool>,
    pub store: OneStore,
    pub messages: TheJobMessages,
    pub reporter: TheOwnersActs,
    pub records: TheRecordsOnThisMachine,
    pub machine: String,
    pub server: AServerLabel,
    pub reporting: Vec<(String, bool)>,
    pub the_machines_now: Arc<dyn Fn() -> SystemTime + Send + Sync>,
    /// The frames asked for, shared with the owner's path (the map's piece 6).
    pub frames: TheFramesAsked,
    /// **The worker's own clock**, the listener's — a clock a test moves on
    /// itself in the tests — for `Telegram` I2a's minutes.
    pub clock: Arc<dyn ReadsItsOwnClock>,
    pub notes: SaysOnItsComplaintsStream,
}

impl TheJournalReader {
    pub fn new(handed: WhatTheReaderIsHanded) -> TheJournalReader {
        let the_idle_switch = handed.reporting.iter().any(|(name, on)| name == THE_IDLE_SWITCH && *on);
        let quiet_since = handed.records.the_latest_ending_on_any_note();
        let pictures = ThePictures::new(handed.messages.conversation, &handed.server);
        TheJournalReader {
            supervisor: handed.supervisor,
            refused: handed.refused,
            store: handed.store,
            messages: handed.messages,
            reporter: handed.reporter,
            records: handed.records,
            machine: handed.machine,
            server: handed.server,
            the_idle_switch,
            the_machines_now: handed.the_machines_now,
            done: HashSet::new(),
            read_from: None,
            owed: BTreeMap::new(),
            idle: TheIdleMessage::default(),
            quiet_since,
            remembered: HashMap::new(),
            pictures,
            requested_pictures: HashMap::new(),
            held_subjects: HashMap::new(),
            last_drop: None,
            frames: handed.frames,
            sheets_in_flight: HashMap::new(),
            sheets_refused: HashSet::new(),
            clock: handed.clock,
            paced: HashMap::new(),
            pictures_done: HashSet::new(),
            tries: HashMap::new(),
            unsent_notices: BTreeMap::new(),
            holds_failed_this_turn: HashSet::new(),
            notes: handed.notes,
            #[cfg(test)]
            the_next_cursor_move_is_lost: false,
        }
    }

    #[cfg(test)]
    pub fn record_notes_with(&mut self, notes: SaysOnItsComplaintsStream) {
        self.messages.queue.note_failed_sends_with(notes.clone());
        self.notes = notes;
    }

    fn note(&self, what: &str) {
        (self.notes)(&format!("the journal: {what}"));
    }

    fn now(&self) -> u64 {
        seconds_of((self.the_machines_now)()).unwrap_or(0)
    }

    /// **Piece 1's read forward**, from `cursor` where one is given.
    fn read(&self, cursor: Option<u64>) -> Option<WhatTheReadOffered> {
        let fields = match cursor {
            Some(cursor) => Fields::none().and(asked::THE_CURSOR_FIELD, cursor.to_string()),
            None => Fields::none(),
        };
        match one_request(self.supervisor.as_ref(), THE_TELEGRAM_WORKERS_READ_FORWARD_REQUEST, fields) {
            Ok(answered) => {
                let read = what_the_read_offered(&answered.fields);
                if read.is_none() {
                    self.note("the journal could not be read forward this turn");
                }
                read
            }
            Err(why) => {
                if let WhyItWasNotAnswered::ItIsToEndItself(_) = why {
                    self.refused.store(true, Ordering::SeqCst);
                }
                self.note(&format!("the journal was not read: {}", why.in_words()));
                None
            }
        }
    }

    /// **The cursor moved** — `State, identity and recovery` E3, after the work.
    fn the_cursor_moves_to(&mut self, next: u64) {
        #[cfg(test)]
        if std::mem::take(&mut self.the_next_cursor_move_is_lost) {
            return;
        }
        if let Err(why) = self.store.record_the_journal_cursor(next) {
            self.note(&format!("the cursor did not move to {next}: {}", why.0));
        }
    }

    /// **`Telegram` R1b and R1 — what leaves with a job's record folder**: its
    /// register lines and this machine's action-context records for it, kept
    /// until then so a late press still answers *This job has already ended.*
    /// (`Telegram` C1b). **A record issued here for another machine's job, or for no
    /// job** — the listener's *Which job?* and its ask — **leaves with its
    /// message's line**, which `Telegram` R1b's bound ages out. A record for a hold whose
    /// job is gone is the hold's and is left. Every turn, as `Telegram` C1b's sweep is;
    /// a store that cannot be read is tried at the next.
    fn forget_what_has_no_folder_left(&mut self) {
        let (records, machine) = (&self.records, self.machine.clone());
        let gone = |job: &str| !records.holds_the_record_folder_of(job);
        let lines = match self.store.the_sent_messages() {
            Ok(lines) => lines,
            Err(why) => return self.note(&format!("the register could not be read: {}", why.0)),
        };
        let registered: HashSet<i64> = lines.iter().map(|line| line.message).collect();
        match self.store.forget_sent_messages_where(&|line| {
            matches!(&line.about, smoozy_telegram::worker_state::WhatAMessageIsAbout::AJob { job, .. } if gone(job))
        }) {
            Ok(0) => {}
            Ok(how_many) => self.note(&format!("forgot {how_many} register lines whose job's folder is gone")),
            Err(why) => self.note(&format!("register lines were not forgotten: {}", why.0)),
        }
        match self.store.remove_action_contexts_where(&|record| match (&record.job, &record.hold) {
            (Some(job), _) if record.owning_machine == machine => gone(job),
            (_, Some(_)) if record.owning_machine == machine => false,
            _ => record.source_message.is_none_or(|message| !registered.contains(&message)),
        }) {
            Ok(0) => {}
            Ok(how_many) => self.note(&format!("removed {how_many} action-context records whose job's folder or message line is gone")),
            Err(why) => self.note(&format!("action-context records were not removed: {}", why.0)),
        }
    }

    /// **One turn**: read forward, do each event's work, move the cursor to
    /// the first event not done, and then the idle message.
    pub fn a_turn(&mut self) {
        if self.refused.load(Ordering::SeqCst) {
            return;
        }
        self.holds_failed_this_turn.clear();
        // Durable contexts also cover a restart, a journal gap, and jobs
        // removed before their ending was read. No new timer or wait is used.
        let records = &self.records;
        let machine = &self.machine;
        if let Err(why) = self.messages.remove_the_stops_where(&|context| {
            context.owning_machine == *machine && context.job.as_deref().is_some_and(|job| {
                let Some(job) = Identity::read_back(IdentityKind::Job, job) else { return false };
                match records.queue().read_one(&job) {
                    Ok(None) => true,
                    Ok(Some(job)) => matches!(job.state, QueueState::Ended { .. }),
                    Err(_) => false, // An unreadable record is not an ended job.
                }
            })
        }) {
            self.note(&why);
        }
        self.forget_what_has_no_folder_left();
        let cursor = match self.store.the_journal_cursor() {
            Ok(cursor) => cursor,
            Err(why) => return self.note(&format!("the cursor could not be read: {}", why.0)),
        };
        let Some(cursor) = cursor else {
            // **A first worker starts at the read's next sequence** (piece 1's
            // note): what the machine journalled before the bot was on is not
            // the phone's to be told now.
            if let Some(read) = self.read(None) {
                self.the_cursor_moves_to(read.next_sequence);
            }
            return;
        };
        let from = self.read_from.unwrap_or(cursor).max(cursor);
        let Some(read) = self.read(Some(from)) else {
            return;
        };
        // **`State, identity and recovery` E5: a gap is said rather than skipped**, once, before anything
        // past it; what the journal no longer holds is gone, and the cursor
        // is free to pass it once the gap is said.
        let mut the_floor = cursor;
        if let Some(oldest) = read.oldest_retained.filter(|oldest| cursor < *oldest) {
            let key = format!("the gap below {oldest}");
            if !self.done.contains(&key) {
                match self.send_text(some_events_were_missed(&self.server)) {
                    TheWork::TheLineReachedNothing => return,
                    _ => {
                        self.done.insert(key);
                    }
                }
            }
            the_floor = oldest;
            self.owed.retain(|sequence, _| *sequence >= oldest);
            self.tries.retain(|sequence, _| *sequence >= oldest);
        }
        for event in &read.events {
            if let Some((sequence, hold)) = self.last_drop.take() {
                if event.sequence == sequence + 1 && event.kind == asked::THE_JOB_RECORD_REMOVED_EVENT {
                    if let Some(job) = &event.about {
                        self.held_subjects.insert(hold.clone(), self.subject_named(job, &hold));
                    }
                }
            }
            if event.kind == asked::THE_RELEASE_EVENT && event.says.ends_with(": dropped") {
                if let Some(hold) = &event.about {
                    self.last_drop = Some((event.sequence, hold.clone()));
                }
            }
        }
        for event in read.events.iter().filter(|event| event.sequence >= from) {
            self.owed.insert(event.sequence, event.clone());
        }
        let past_the_read = read.events.last().map_or(from.max(the_floor), |last| last.sequence + 1);
        let owed: Vec<AnEventRead> = self.owed.values().cloned().collect();
        let mut the_line_reached_nothing = false;
        for event in owed {
            let count = self.tries.get(&event.sequence).map_or(0, |tries| tries.counted);
            self.the_sends_context(&event, "event", count);
            match self.the_work_for(&event) {
                TheWork::Done => {
                    self.owed.remove(&event.sequence);
                    self.tries.remove(&event.sequence);
                }
                TheWork::NotYet => {}
                TheWork::TheLineReachedNothing => {
                    the_line_reached_nothing = true;
                    let after = self.messages.queue.how_many_calls_reached_telegram();
                    let passed = self.tries.entry(event.sequence).or_default().after_a_failure(after);
                    self.note(&format!("event {} job {} kind {} not reached; counted tries {}",
                        event.sequence, event.about.as_deref().unwrap_or("none"), event.kind, self.tries[&event.sequence].counted));
                    if passed {
                        self.note(&format!("event {} job {} kind {} passed after 3 counted tries; one unsent notice is owed",
                            event.sequence, event.about.as_deref().unwrap_or("none"), event.kind));
                        self.owed.remove(&event.sequence);
                        self.tries.remove(&event.sequence);
                        self.unsent_notices.insert(event.sequence, AnUnsentNotice { event, tries: TheTriesOfASend::default() });
                    }
                    // `Telegram` F3a: later jobs go on. A down line advances no failed
                    // event and spends no further tries without another reach.
                }
            }
        }
        self.the_unsent_notices();
        self.messages.queue.the_event_being_sent("outside a journal event".into());
        self.read_from = Some(past_the_read);
        let reached = self.owed.keys().chain(self.unsent_notices.keys()).min().copied()
            .unwrap_or(past_the_read).max(the_floor);
        if reached > cursor {
            self.the_cursor_moves_to(reached);
        }
        // **`Telegram` I2a: a held frame whose minutes have passed is sent**,
        // after the turn's events, so a frame that came in this turn replaces
        // it first and the newest goes.
        if the_line_reached_nothing || self.the_held_frames_whose_minutes_have_passed() == TheWork::TheLineReachedNothing {
            return;
        }
        // **Owed piece 11: a picture asked for and not back in time is said to
        // have failed**, after the turn's events, so one published in this
        // turn is sent rather than given up on.
        if self.the_pictures_not_back_in_time() != TheWork::TheLineReachedNothing {
            self.the_idle_message();
        }
    }

    fn the_job_of_event(&self, event: &AnEventRead) -> Option<Identity> {
        let about = event.about.as_deref()?;
        if let Some(job) = Identity::read_back(IdentityKind::Job, about) { return Some(job); }
        let hold = Identity::read_back(IdentityKind::Operation, about)?;
        match self.records.the_hold(&hold)?.subject {
            smoozy_runtime::holds::Subject::Job(job) => Some(job),
            _ => None,
        }
    }

    fn the_sends_context(&self, event: &AnEventRead, kind: &str, counted: u8) {
        self.messages.queue.the_event_being_sent(format!(
            "event {} subject {} job {} kind {} / {kind}, counted tries before send {counted}",
            event.sequence, event.about.as_deref().unwrap_or("none"),
            self.the_job_of_event(event).map(|job| job.as_text()).unwrap_or_else(|| "none".into()), event.kind,
        ));
    }

    /// `Telegram` F3a: the notice completes this event's work before its cursor passes.
    /// Later events still run; a notice never creates a replacement notice.
    fn the_unsent_notices(&mut self) {
        let notices = std::mem::take(&mut self.unsent_notices);
        for (sequence, mut notice) in notices {
            self.the_sends_context(&notice.event, "unsent notice", notice.tries.counted);
            let label = self.the_job_of_event(&notice.event)
                .map(|job| self.records.what_the_job_is_called(&job)).unwrap_or_default();
            // `Telegram` R7a: the explicit colour and the marked names; no markup.
            let (text, entities) = the_unsent_notice(&self.server, &label);
            let conversation = self.messages.conversation;
            let queue = self.messages.queue.clone();
            let receipt = queue.offer(AClassOfSend::EndingsHoldsAndDrops,
                Box::new(move |client| client.send_message_with_entities(conversation, &text, entities.clone())));
            let work = self.sent(smoozy_telegram::actions::wait_for(&queue, receipt));
            if work == TheWork::TheLineReachedNothing {
                let after = self.messages.queue.how_many_calls_reached_telegram();
                let passed = notice.tries.after_a_failure(after);
                self.note(&format!("unsent notice for event {sequence} not reached; counted tries {}", notice.tries.counted));
                if passed {
                    self.note(&format!("unsent notice for event {sequence} passed after 3 counted tries; no further notice"));
                } else {
                    self.unsent_notices.insert(sequence, notice);
                }
            }
        }
    }

    /// **`Telegram` I2a — at the worker's first turn after `minutes` have
    /// passed since a paced job's last picture, its held frame is sent**, from
    /// the set as `Telegram` I2 reads it, and that send's moment is the job's new last
    /// moment. **A held frame of a job that has ended, of whatever kind, is
    /// dropped and never sent** (`Telegram` I2a, `Telegram` E1): the ending, the moving picture and
    /// the contact sheet go as V1 lets them. One whose copy was cleared before
    /// its turn is nothing to send. A line that reached nothing keeps the frame
    /// held, to be sent on a later turn.
    fn the_held_frames_whose_minutes_have_passed(&mut self) -> TheWork {
        let now = self.clock.now();
        let held: Vec<String> =
            self.paced.iter().filter(|(_, pacing)| pacing.held.is_some()).map(|(job, _)| job.clone()).collect();
        for job_text in held {
            let record = Identity::read_back(IdentityKind::Job, &job_text)
                .and_then(|job| Some((self.records.the_job_record(&job)?, job)));
            let Some((record, job)) = record.filter(|(record, _)| !matches!(record.state, QueueState::Ended { .. })) else {
                self.paced.remove(&job_text);
                continue;
            };
            let Some(interval) = self.records.the_choice_in_force(&record).the_interval_between_pictures() else {
                self.paced.remove(&job_text);
                continue;
            };
            let Some(pacing) = self.paced.get(&job_text) else { continue };
            if pacing.last_sent.is_some_and(|last| now.saturating_sub(last) < interval) {
                continue;
            }
            let Some(frame) = pacing.held.clone() else { continue };
            let (came_to, sent) = self.a_paced_frame_sent(&job, &record, &frame);
            if came_to == TheWork::TheLineReachedNothing {
                return came_to;
            }
            if let Some(pacing) = self.paced.get_mut(&job_text) {
                pacing.held = None;
                if sent {
                    pacing.last_sent = Some(now);
                }
            }
        }
        TheWork::Done
    }

    /// **One frame's pictures sent as `Telegram` I2 sends them**, read from the set under
    /// the production its main copy's event named — one album, each picture
    /// named, or one picture — with *Stop this job* on a lone picture or on
    /// an album’s separate companion while the job is live. What it came to, and
    /// whether a picture went.
    fn a_paced_frame_sent(&mut self, job: &Identity, record: &JobRecord, frame: &APacedFrame) -> (TheWork, bool) {
        let label = self.records.what_the_job_is_called(job);
        let Some(copies) = self.records.the_frames_copies(
            record,
            job,
            &label,
            frame.frame,
            frame.which_picture.clone(),
            Some(&frame.operation),
        ) else {
            self.note(&format!("frame {} of {} was cleared before its picture was read", frame.frame, job.as_text()));
            return (TheWork::Done, false);
        };
        // **`Telegram` C1b: Stop only while the job is live** — a last frame's
        // picture read after the ending goes with none, and no companion.
        let stop = self.it_is_live(job).then(|| self.the_subject_of_a_job(job, record));
        match self.pictures.send_a_frame_with_its_stop(
            &mut self.messages,
            copies,
            AClassOfSend::FramePicturesAndMedia,
            stop.as_ref(),
        ) {
            APictureSent::Sent { .. } => (TheWork::Done, true),
            APictureSent::NotSentAndSaid | APictureSent::NotSentAlreadySaid => (TheWork::Done, false),
            APictureSent::NotAccepted(ended) if the_work_is_done(&ended) => (TheWork::Done, false),
            APictureSent::NotAccepted(_) => (TheWork::TheLineReachedNothing, false),
        }
    }

    /// **`Telegram` I4's answer, ended where the picture never came — owed
    /// piece 11, on the build runner's ruling of 2026-10-16.** A frame the
    /// user was told is *being made again* whose picture is not back
    /// [`crate::the_telegram_worker::A_PICTURE_IS_WAITED_FOR`] after it was
    /// asked for is said, once, to have failed, in the first class of
    /// `Telegram` C6 since it ends an answer. **Nothing is journalled for it**:
    /// the supervisor journals nothing for a production that made nothing,
    /// which is why the worker keeps the limit itself. A line that reached
    /// nothing leaves the frame asked for, to be said on a later turn.
    fn the_pictures_not_back_in_time(&mut self) -> TheWork {
        for (job, frame) in self.frames.waited_for_too_long() {
            let label = Identity::read_back(IdentityKind::Job, &job)
                .map(|identity| self.records.what_the_job_is_called(&identity))
                .unwrap_or_default();
            let text = a_frames_picture_could_not_be_made(
                &self.server,
                &label,
                u64::try_from(frame).unwrap_or(0),
            );
            let subject = self.subject_named(&job, &job);
            let live = Identity::read_back(IdentityKind::Job, &job).is_some_and(|job| self.it_is_live(&job));
            let ended = smoozy_telegram::actions::message_result(self.messages.send_about_while(
                live,
                &smoozy_telegram::job_messages::AComposedMessage {
                    text,
                    buttons: vec![],
                },
                &subject,
                &self.server,
                &label,
                AClassOfSend::RoutedAnswersAndAcknowledgements,
            ));
            match self.sent(ended) {
                TheWork::TheLineReachedNothing => return TheWork::TheLineReachedNothing,
                _ => {
                    self.frames.answered(&job, frame);
                }
            }
        }
        TheWork::Done
    }

    /// **What one send came to**, for the event that made it: accepted is
    /// done; a line that reached nothing is tried again; **a refusal is done**
    /// and said on the complaints stream, because sending the same message
    /// again would be refused the same way and hold every event behind it.
    fn sent(&self, ended: HowASendEnded) -> TheWork {
        match ended {
            HowASendEnded::Accepted(_) | HowASendEnded::DroppedAsSuperseded => TheWork::Done,
            HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_)) => TheWork::TheLineReachedNothing,
            HowASendEnded::HandedBack(why) => {
                self.note(&format!("a job message was refused and is not sent again: {why:?}"));
                TheWork::Done
            }
        }
    }

    /// **A message with no buttons, sent** (`Telegram` C6's class for
    /// endings, holds and drops), and what it came to.
    fn send_text(&mut self, text: AMarkedText) -> TheWork {
        let ended = self.messages.send(text);
        self.sent(ended)
    }

    /// **The same, for a message about one job with no buttons** — an
    /// ending — registered as about that job (`Telegram` R1b), so a reply to
    /// it reaches it.
    fn send_text_about(&mut self, text: AMarkedText, job: &Identity) -> TheWork {
        let ended = self.messages.send_about_a_job(text, &job.as_text());
        self.sent(ended)
    }

    /// The same, for a message sent with its buttons.
    fn sent_with_buttons(&self, sent: Result<i64, WhyAMessageWasNotSent>) -> TheWork {
        match sent {
            Ok(_) => TheWork::Done,
            Err(WhyAMessageWasNotSent::NotSent(ended)) => self.sent(ended),
            // **The message went**: sending it again would say it twice.
            Err(why) => {
                self.note(&format!("a job message went without its buttons: {why:?}"));
                TheWork::Done
            }
        }
    }

    /// **One part of an event's work, once in this run** (`State, identity and recovery` E4): `part` names it
    /// within the event, so an event of several messages never repeats one
    /// that went.
    fn once(&mut self, event: &AnEventRead, part: &str, work: impl FnOnce(&mut Self) -> TheWork) -> TheWork {
        let key = format!("{} {part}", event.event);
        if self.done.contains(&key) {
            return TheWork::Done;
        }
        let counted = self.tries.get(&event.sequence).map_or(0, |tries| tries.counted);
        self.the_sends_context(event, part, counted);
        let came_to = work(self);
        if came_to == TheWork::Done {
            self.done.insert(key);
        }
        came_to
    }

    fn the_work_for(&mut self, event: &AnEventRead) -> TheWork {
        if [
            asked::THE_RECORDED_OUTCOME_WRITTEN_EVENT,
            asked::THE_JOB_RECORD_REMOVED_EVENT,
            asked::THE_RELEASE_EVENT,
            asked::THE_HOLD_PLACED_EVENT,
        ]
        .contains(&event.kind.as_str())
        {
            if let Some(at) = event.happened_at {
                self.quiet_since = Some(self.quiet_since.map_or(at, |quiet| quiet.max(at)));
            }
        }
        match event.kind.as_str() {
            asked::THE_RECORDED_OUTCOME_WRITTEN_EVENT => self.an_ending(event),
            asked::THE_HOLD_PLACED_EVENT => self.a_hold_placed(event),
            asked::THE_RELEASE_EVENT => self.a_release(event),
            asked::THE_JOB_RECORD_REMOVED_EVENT => self.a_removal(event),
            asked::THE_JOB_RECORD_WRITTEN_EVENT => self.a_job_written(event),
            asked::THE_RENDER_ATTEMPT_STARTED_EVENT => self.a_started_job(event),
            asked::THE_PREVIEW_PUBLISHED_EVENT => self.a_preview_published(event),
            asked::THE_TELEGRAM_CHOICE_CHANGED_EVENT => self.a_choice_changed(event),
            _ => TheWork::Done,
        }
    }

    /// **`Telegram` Y1's report: a job this machine made for itself on a
    /// phone's *try it again* is told once it stands** — *… was resubmitted* —
    /// through [`TheOwnersActs::report`], the way a stop's outcome is told.
    /// The command's own answer told nothing where it made the job, so this
    /// is the one telling (piece 4's rule: the journal tells the rest).
    /// **A job the desktop sent says nothing here**: its event carries no
    /// mark ([`the_job_a_resubmission_was_made_from`]).
    fn a_job_written(&mut self, event: &AnEventRead) -> TheWork {
        if the_job_a_resubmission_was_made_from(&event.says).is_none() {
            return TheWork::Done;
        }
        let Some(job) = event.about.as_deref().and_then(|about| Identity::read_back(IdentityKind::Job, about)) else {
            return TheWork::Done;
        };
        let label = self.records.what_the_job_is_called(&job);
        let server = self.server.clone();
        let subject = self.subject_named(&job.as_text(), &job.as_text());
        self.once(event, "the resubmission", |reader| {
            let reported = reader.reporter.report_about(
                AJournalledOutcome::ASubmission {
                    server: server.clone(),
                    label: label.clone(),
                    outcome: ASubmissionsOutcome::Submitted,
                },
                &subject,
                &server,
                &label,
            );
            reported.map_or(TheWork::Done, |ended| reader.sent(ended))
        })
    }

    /// `Telegram` C1a: journal starts are sent under every reporting choice, for both
    /// desktop submissions and phone resubmissions. Reconnect is not a start.
    fn a_started_job(&mut self, event: &AnEventRead) -> TheWork {
        let Some(job) = event
            .about
            .as_deref()
            .and_then(|id| Identity::read_back(IdentityKind::Job, id))
        else {
            return TheWork::Done;
        };
        let Some(record) = self.records.the_job_record(&job) else {
            return TheWork::Done;
        };
        if self.records.the_job_reports_nothing(&record) { return TheWork::Done; }
        let label = self.records.what_the_job_is_called(&job);
        let mut subject = self.the_subject_of_a_job(&job, &record);
        if let Some(attempt) = event
            .says
            .strip_prefix("attempt ")
            .and_then(|n| n.parse::<u32>().ok())
        {
            subject.generation = format!("{}:{attempt}", the_generation_of(&record));
        }
        // **`Telegram` C1b: a start read after its job ended carries no Stop.**
        let live = self.it_is_live(&job);
        self.once(event, "the start", |reader| {
            let sent = reader.messages.send_about_while(
                live,
                &smoozy_telegram::job_messages::AComposedMessage {
                    text: the_start_message(&reader.server, &label),
                    buttons: vec![],
                },
                &subject,
                &reader.server,
                &label,
                AClassOfSend::EndingsHoldsAndDrops,
            );
            reader.sent_with_buttons(sent)
        })
    }

    fn subject_named(&self, job: &str, fallback_generation: &str) -> TheSubject {
        if let Some(record) = Identity::read_back(IdentityKind::Job, job)
            .and_then(|id| self.records.the_job_record(&id))
        {
            return self.the_subject_of_a_job(&record.job, &record);
        }
        TheSubject {
            owning_machine: self.machine.clone(),
            job: Some(job.into()),
            hold: None,
            generation: fallback_generation.into(),
        }
    }

    fn subject_of_hold(&self, hold: &Identity) -> Option<TheSubject> {
        if let Some(subject) = self.held_subjects.get(&hold.as_text()) {
            return Some(subject.clone());
        }
        if let Some(standing) = self.records.the_hold(hold) {
            if let Subject::Job(job) = standing.subject {
                return Some(self.subject_named(&job.as_text(), &hold.as_text()));
            }
        }
        let found = Mutex::new(None);
        let mut store = self.store.clone();
        let _ = store.remove_action_contexts_where(&|record| {
            if record.hold.as_deref() == Some(hold.as_text().as_str()) {
                if let Some(job) = record.payload["job"].as_str().or(record.job.as_deref()) {
                    *found.lock().unwrap_or_else(|p| p.into_inner()) =
                        Some(self.subject_named(job, &hold.as_text()));
                }
            }
            false
        });
        found.into_inner().unwrap_or_else(|p| p.into_inner())
    }

    /// **`Telegram` C1b — whether a job is live now**: waiting, rendering or
    /// held, as the journal's records read while a message is composed. Only
    /// a message composed while it is live carries Stop.
    fn it_is_live(&self, job: &Identity) -> bool {
        self.records
            .the_job_record(job)
            .is_some_and(|record| !matches!(record.state, QueueState::Ended { .. }))
    }

    // ------------------------------------------------------------ endings

    fn the_subject_of_a_job(&self, job: &Identity, record: &JobRecord) -> TheSubject {
        TheSubject {
            owning_machine: self.machine.clone(),
            job: Some(job.as_text()),
            hold: None,
            generation: self.records.action_generation(record),
        }
    }

    /// **`Telegram` E1, `Telegram` Y4a, `Telegram` D2 and `Telegram` G1 — a render's recorded outcome.**
    fn an_ending(&mut self, event: &AnEventRead) -> TheWork {
        let Some(job) = event.about.as_deref().and_then(|about| Identity::read_back(IdentityKind::Job, about)) else {
            return TheWork::Done;
        };
        let (Some(record), Some(note)) = (self.records.the_job_record(&job), self.records.the_note(&job)) else {
            // **The job went before its ending was read** — removed by a
            // clear or an erase — and nothing of it is left to compose from.
            self.note(&format!("{} ended and its records were gone before its ending was read", job.as_text()));
            return TheWork::Done;
        };
        let manifest = self.records.the_manifest(&record);
        let label = note.what_the_job_is_called(manifest.as_ref());
        let choice = if event.says.starts_with("Telegram off: ") || self.records.the_job_reports_nothing(&record) {
            TheReportingChoice::Nothing
        } else {
            self.records.the_choice_in_force(&record)
        };
        let outcome = note.ending.as_ref().map(|ending| ending.outcome);
        // **`Telegram` I2a: a frame still held when the job ends is not
        // sent**, whatever the ending; the finishing message and the contact
        // sheet come instead.
        if let Some(pacing) = self.paced.get_mut(&job.as_text()) {
            pacing.held = None;
        }
        let ended_at = note.ended.as_ref().and_then(|ended| seconds_of(ended.taken_at())).or(event.happened_at);
        // **The finished message waits for the job's last frame's pictures**
        // (the runner's ruling of 2026-10-19): the ending is journalled before
        // the last frame's preview is made (`Queue & History` P1b), so without
        // this the album arrived after *finished* on the live pass. Within the
        // wait the contact sheet keeps; later events go on meanwhile.
        if outcome == Some(TheOutcomeAsWritten::Finished)
            && !self.done.contains(&format!("{} its own message", event.event))
            && self.the_last_frames_picture_is_awaited(&job, &record, manifest.as_ref(), ended_at)
        {
            return TheWork::NotYet;
        }
        let theirs = self.once(event, "its own message", |reader| match outcome {
            _ if choice == TheReportingChoice::Nothing => TheWork::Done,
            Some(TheOutcomeAsWritten::Finished) => reader.finished(&record, &note, &label, choice),
            Some(TheOutcomeAsWritten::Failed) | Some(TheOutcomeAsWritten::NoStatusWasReaped) => {
                reader.failed(&job, &record, &note, &label, choice)
            }
            Some(TheOutcomeAsWritten::GivenUp) => reader.given_up(&job, &record, &note, &label, choice),
            Some(TheOutcomeAsWritten::LeftStopped) => reader.left_stopped(&record, &note, &label),
            None => TheWork::Done,
        });
        if theirs != TheWork::Done {
            return theirs;
        }
        let the_groups = self.once(event, "its group's message", |reader| reader.the_group(&record, &manifest));
        if the_groups != TheWork::Done {
            return the_groups;
        }
        if outcome != Some(TheOutcomeAsWritten::Finished) {
            return TheWork::Done;
        }
        let the_holds = self.the_holds_waiting_on(&job, event.happened_at);
        if the_holds == TheWork::TheLineReachedNothing {
            return the_holds;
        }
        // **`Telegram` M2 — the contact sheet at the job's end**, the map's
        // piece 6, after the job's own message and its group's.
        // Since 2026-10-31 one album per View Layer, each its own once-only
        // part of the event (the seventeenth notes, note 10).
        let the_sheet = self.the_contact_sheets(event, &job, &record, manifest.as_ref(), &label, choice, ended_at);
        // **`Telegram` M10 — the finished video, after the contact sheet**,
        // each file one more part of the same event, so none goes twice.
        let the_video = if the_sheet == TheWork::Done {
            self.the_finished_videos(event, &job, &record, manifest.as_ref(), &label, choice)
        } else {
            the_sheet
        };
        match (the_holds, the_video) {
            (_, TheWork::TheLineReachedNothing) => TheWork::TheLineReachedNothing,
            (TheWork::NotYet, _) | (_, TheWork::NotYet) => TheWork::NotYet,
            _ => TheWork::Done,
        }
    }

    fn finished(&mut self, record: &JobRecord, note: &TheNote, label: &str, choice: TheReportingChoice) -> TheWork {
        if record.group.is_some() || !the_job_reports(choice, AKindOfReport::Finished) {
            return TheWork::Done;
        }
        if let Some(video) = self.records.the_video_at_the_ending(record) {
            let took = note
                .started
                .as_ref()
                .zip(note.ended.as_ref())
                .and_then(|(started, ended)| {
                    ended.taken_at().duration_since(started.taken_at()).ok()
                })
                .map(smoozy_telegram::job_messages::how_long_in_words);
            return self.send_text_about(
                a_user_message(&self.server, label, "", &video.finished_in_words(took)),
                &record.job,
            );
        }
        let (there, _) = self.records.the_frames_at_the_ending(record, note);
        // **Where the note lacks either moment the length is left out**, never
        // made up: the crate leaves it out wherever the two do not subtract,
        // and an end before its start is that case.
        let (started, ended) = match (note.started.as_ref(), note.ended.as_ref()) {
            (Some(started), Some(ended)) => (started.taken_at(), ended.taken_at()),
            _ => (UNIX_EPOCH + Duration::from_secs(1), UNIX_EPOCH),
        };
        let finished = AFinishedJob {
            server: self.server.clone(),
            label: label.to_string(),
            frames: there,
            started,
            ended,
            group: None,
        };
        match the_finishing_message(&finished) {
            Some(text) => self.send_text_about(text, &record.job),
            None => TheWork::Done,
        }
    }

    fn failed(&mut self, job: &Identity, record: &JobRecord, note: &TheNote, label: &str, choice: TheReportingChoice) -> TheWork {
        if !the_job_reports(choice, AKindOfReport::Failed) {
            return TheWork::Done;
        }
        let ending = note.ending.as_ref();
        let attempt_number = ending.map_or(1, |ending| ending.the_attempt_whose_capture_record_holds_the_words);
        let attempts = self.records.the_attempts(job).unwrap_or_default();
        let attempt = attempts.iter().find(|one| one.attempt == attempt_number);
        let (there, not_there) = self.records.the_frames_at_the_ending(record, note);
        let at_frame = note
            .last_frame_figure
            .and_then(|frame| u64::try_from(frame).ok())
            .map(|frame| (frame, there + not_there))
            .filter(|(_, of)| *of > 0);
        let failed = AFailedJob {
            server: self.server.clone(),
            job: job.as_text(),
            label: label.to_string(),
            generation: self.records.action_generation(record),
            attempt: attempt_number,
            at_frame,
            exit_status: ending.and_then(|ending| ending.exit_status),
            frames_on_the_server: there,
            ordinary_output: what_the_stream_came_to(attempt, AStream::TheOrdinaryOutput),
            complaints: what_the_stream_came_to(attempt, AStream::TheComplaints),
            cause: the_cause_of(attempt),
            // **`Rented-server runtime` V12's reading, carried on the note's
            // ending**: the figure against the job's own output folder at the
            // act that first tried to write the outcome. **No reading is not
            // a reading of nought**, so an absent one says nothing.
            disk_free_at_the_ending: note
                .the_free_space_at_the_ending
                .as_ref()
                .and_then(|free| free.the_reading())
                .map(|read| read.available_bytes),
        };
        // **`Telegram` C1b: an ending carries no Stop**; the words and *try it
        // again* stay.
        let subject = self.the_subject_of_a_job(job, record);
        let sent = self.messages.send_with_buttons(&the_failure_message(&failed), &subject);
        self.sent_with_buttons(sent)
    }

    fn given_up(&mut self, job: &Identity, record: &JobRecord, note: &TheNote, label: &str, choice: TheReportingChoice) -> TheWork {
        if !the_job_reports(choice, AKindOfReport::Failed) {
            return TheWork::Done;
        }
        let attempt_number = note.ending.as_ref().map_or(1, |ending| ending.the_attempt_whose_capture_record_holds_the_words);
        let attempts = self.records.the_attempts(job).unwrap_or_default();
        let attempt = attempts.iter().find(|one| one.attempt == attempt_number);
        let given_up = AGivenUpJob {
            server: self.server.clone(),
            job: job.as_text(),
            label: label.to_string(),
            generation: self.records.action_generation(record),
            attempt: attempt_number,
            frames_on_the_server: self.records.the_frames_at_the_ending(record, note).0,
            ordinary_output: what_the_stream_came_to(attempt, AStream::TheOrdinaryOutput),
            complaints: what_the_stream_came_to(attempt, AStream::TheComplaints),
        };
        let subject = self.the_subject_of_a_job(job, record);
        let sent = self.messages.send_with_buttons(&the_given_up_message(&given_up), &subject);
        self.sent_with_buttons(sent)
    }

    /// **`Telegram` D2's second half — the stop's recorded outcome, with the
    /// counts and what remains**, through [`TheOwnersActs::report`]. Told
    /// whichever route stopped it, as A3 tells a release whichever route
    /// answered: the phone carries what the machine recorded.
    fn left_stopped(&mut self, record: &JobRecord, note: &TheNote, label: &str) -> TheWork {
        if let Some(video) = self.records.the_video_at_the_ending(record) {
            return self.send_text_about(
                a_user_message(&self.server, label, "", &format!("Stopped. {}", video.stopped_in_words())),
                &record.job,
            );
        }
        let (there, not_there) = self.records.the_frames_at_the_ending(record, note);
        self.send_text_about(
            the_stops_outcome(
                &self.server,
                label,
                &AStopsOutcome::Stopped {
                    frames_on_the_server: there,
                    frames_not_rendered: not_there,
                },
            ),
            &record.job,
        )
    }

    /// **`Telegram` G1 and G2 — the group's one message**, composed from every
    /// member record this machine holds with the group's identity, sent once
    /// the last member has a recorded ending and at least one finished.
    fn the_group(&mut self, record: &JobRecord, manifest: &Option<SubmissionManifest>) -> TheWork {
        let Some(group) = &record.group else {
            return TheWork::Done;
        };
        let Some(every) = self.records.every_job_record() else {
            return TheWork::Done;
        };
        let members: Vec<AMember> = every
            .iter()
            .filter(|one| one.group.as_ref() == Some(group))
            .filter(|one| !self.records.the_record_reports_nothing(one))
            .map(|one| {
                let note = self.records.the_note(&one.job);
                let ending = note.as_ref().and_then(|note| note.ending.as_ref()).map(|ending| match ending.outcome {
                    TheOutcomeAsWritten::Finished => {
                        let there = note.as_ref().map_or(0, |note| self.records.the_frames_at_the_ending(one, note).0);
                        AMembersEnding::Finished { frames: there }
                    }
                    TheOutcomeAsWritten::Failed | TheOutcomeAsWritten::NoStatusWasReaped => AMembersEnding::Failed,
                    TheOutcomeAsWritten::GivenUp => AMembersEnding::GivenUp,
                    TheOutcomeAsWritten::LeftStopped => AMembersEnding::LeftStopped,
                });
                AMember {
                    job: one.job.as_text(),
                    ending,
                    started: note.as_ref().and_then(|note| note.started.as_ref()).map(|at| at.taken_at()),
                    ended: note.as_ref().and_then(|note| note.ended.as_ref()).map(|at| at.taken_at()),
                }
            })
            .collect();
        // **The group is named by its file**: every member is one scene of the
        // same `.blend`, and each member's own label names its scene.
        let the_groups_label = manifest
            .as_ref()
            .map(|manifest| smoozy_queue_history_machine::naming::the_files_name(&manifest.blend_path_on_the_machine))
            .unwrap_or_default();
        // **`Telegram` G1: the group's one message goes once, whichever
        // member's ending is read last.** Where the last two endings are read
        // in one turn (after a restart, say), both members' records already
        // read *ended* at the first, and each would send it. **The mark is
        // kept with every other part this run has done** (`State, identity and
        // recovery` E4), under the group's identity rather than the event's.
        // Added 2026-10-25, the small owed fixes after the eighteenth notes.
        let the_groups_own = format!("the group {} its message", group.as_text());
        if self.done.contains(&the_groups_own) {
            return TheWork::Done;
        }
        // **`Telegram` C1b: a group's ending carries no Stop** — the labelled
        // scene Stops of 2026-10-23 went on 2026-10-25.
        match what_the_group_says(&self.server, &the_groups_label, &members) {
            Some(text) => {
                let came_to = self.send_text(text);
                if came_to == TheWork::Done {
                    self.done.insert(the_groups_own);
                }
                came_to
            }
            None => TheWork::Done,
        }
    }

    // ------------------------------------------------------------ pictures

    /// The first-frame check sends its pictures before the separate approval
    /// question, so ordinary frame reports and contact sheets do not repeat them.
    fn it_is_a_first_frame_check(&self, job: &Identity) -> bool {
        self.records.every_job_record().is_some_and(|every| {
            every.iter().any(|one| {
                let QueueState::Held { hold } = &one.state else { return false };
                self.records.the_hold(hold).is_some_and(|record| {
                    record.release.is_none()
                        && Self::what_it_waits_for(&record).the_phone_is_told()
                        && record.dependency.as_ref().map(|dependency| &dependency.work) == Some(job)
                })
            })
        })
    }

    /// **`Telegram` I2 and `Telegram` I3 — a preview published is a picture to send**,
    /// the map's piece 6. ~~**The main copy only** (`Queue & History` P10: the
    /// other View Layers' copies are the desktop's alone).~~ **Since 2026-10-18
    /// every copy of the frame goes, on the main copy's event** (`Telegram`
    /// I2, `Queue & History` P10 as written that day, the Creator's fourteenth
    /// notes, note 7): the supervisor journals all of a production's copies in
    /// one go, so the main copy's event sends the frame's set as its production
    /// published it ([`TheRecordsOnThisMachine::the_frames_copies`]) — one
    /// album, each picture named, or one picture for a frame of one copy — and
    /// **the other copies' events send nothing**. Sent where the
    /// job's reporting choice sends pictures (`Telegram` V1), with *Stop this
    /// job* while it renders (`Telegram` D1); **sent as an answer where the user
    /// asked for this frame** (`Telegram` I4), whatever the choice; and not repeated where it
    /// is a check's first frame, which its waiting message carries. **The
    /// derivative is read from the set, never remade** (`Telegram` I2); one cleared
    /// before it was read is nothing to send. **Its caption names the View
    /// Layer, or the source, the event carries** (`Telegram` I2, added
    /// 2026-10-17, the Creator's tenth notes, note 2), sent or answered alike.
    ///
    /// **`Telegram` I2a, from 2026-10-19: where the job's choice is `Telegram`
    /// V1a's fourth member, a frame not asked for is paced.** The job's first frame
    /// picture goes at once. After a picture is sent, a frame whose event
    /// arrives less than `minutes` after it is **held, not sent** — the job's
    /// one held frame, the newest by frame number replacing an older one — and
    /// the event's work is done (`Telegram` F3), so the cursor moves on. A frame whose
    /// event arrives once `minutes` have passed goes at once, or the held
    /// frame does where that is newer. **An asked frame is never paced and
    /// never moves the moment** (`Telegram` I4).
    ///
    /// `Telegram` C1b: while the job is live, a lone picture gets Stop and an
    /// album a separate companion; ~~including when the job ended while its
    /// picture was being produced~~ a picture read once the job has ended gets
    /// neither (eighteenth notes, note 16, 2026-10-25).
    fn a_preview_published(&mut self, event: &AnEventRead) -> TheWork {
        let came_to = self.a_preview_published_now_or_held(event);
        if came_to == TheWork::Done {
            let main = smoozy_queue_history_machine::previews::ThePublishedPreview::read_back(&event.says)
                .filter(|published| published.main)
                .zip(event.about.as_deref().and_then(|about| Identity::read_back(IdentityKind::Job, about)));
            if let Some((published, job)) = main {
                self.pictures_done.insert((job.as_text(), published.frame));
            }
        }
        came_to
    }

    fn a_preview_published_now_or_held(&mut self, event: &AnEventRead) -> TheWork {
        let Some(published) = smoozy_queue_history_machine::previews::ThePublishedPreview::read_back(&event.says) else {
            return TheWork::Done;
        };
        if !published.main {
            return TheWork::Done;
        }
        let Some(job) = event.about.as_deref().and_then(|about| Identity::read_back(IdentityKind::Job, about)) else {
            return TheWork::Done;
        };
        let Some(record) = self.records.the_job_record(&job) else {
            return TheWork::Done;
        };
        let asked = self.frames.is_asked(&job.as_text(), published.frame);
        let choice = self.records.the_choice_in_force(&record);
        let pictures_are_sent =
            the_job_reports(choice, AKindOfReport::AFramePicture)
                && !self.records.the_job_reports_nothing(&record)
                && !self.it_is_a_first_frame_check(&job);
        if !asked && !pictures_are_sent {
            return TheWork::Done;
        }
        let this_frame = APacedFrame {
            frame: published.frame,
            operation: published.operation.clone(),
            which_picture: which_picture_the_event_names(&published),
        };
        if asked {
            let label = self.records.what_the_job_is_called(&job);
            let Some(copies) = self.records.the_frames_copies(
                &record,
                &job,
                &label,
                this_frame.frame,
                this_frame.which_picture.clone(),
                // An earlier production can name a cleared copy that this
                // production failed to restore. An asked album needs the
                // whole set, as the supervisor's `Telegram` I4 check did.
                None,
            ) else {
                // **A frame asked for stays asked** (owed piece 11): the limit
                // then says the picture could not be made, rather than nothing.
                self.note(&format!("frame {} of {} was cleared before its picture was read", published.frame, job.as_text()));
                return TheWork::Done;
            };
            let server = self.server.clone();
            return self.once(event, "its picture", |reader| {
                // **`Telegram` C1b: an asked frame of a job that has ended is
                // answered with no Stop.**
                let subject = reader.it_is_live(&job).then(|| reader.the_subject_of_a_job(&job, &record));
                let key = (job.as_text(), published.frame);
                let pictures = reader
                    .requested_pictures
                    .entry(key.clone())
                    .or_insert_with(|| {
                        ThePictures::for_a_request(reader.messages.conversation, &server)
                    });
                let result = pictures.send_a_frame_with_its_stop(
                    &mut reader.messages,
                    copies,
                    AClassOfSend::RoutedAnswersAndAcknowledgements,
                    subject.as_ref(),
                );
                if let APictureSent::NotAccepted(ended) = result {
                    if !the_work_is_done(&ended) {
                        return TheWork::TheLineReachedNothing;
                    }
                }
                reader.requested_pictures.remove(&key);
                reader.frames.answered(&job.as_text(), published.frame);
                TheWork::Done
            });
        }

        // **`Telegram` I2a: held, or the newest sent at once.**
        let interval = choice.the_interval_between_pictures();
        let now = self.clock.now();
        let mut sending = this_frame.clone();
        if let Some(interval) = interval {
            let pacing = self.paced.entry(job.as_text()).or_default();
            if let Some(held) = pacing.held.take().filter(|held| held.frame > this_frame.frame) {
                sending = held;
            }
            if pacing.last_sent.is_some_and(|last| now.saturating_sub(last) < interval) {
                pacing.held = Some(sending.clone());
                let said = format!(
                    "frame {} of {} is held for its turn, {} seconds apart",
                    sending.frame,
                    job.as_text(),
                    interval.as_secs()
                );
                self.note(&said);
                return TheWork::Done;
            }
        }
        let job_text = job.as_text();
        let came_to = self.once(event, "its picture", |reader| {
            let (came_to, sent) = reader.a_paced_frame_sent(&job, &record, &sending);
            if interval.is_some() {
                let pacing = reader.paced.entry(job_text.clone()).or_default();
                if sent {
                    pacing.last_sent = Some(now);
                }
                // **A line that reached nothing keeps a newer held frame
                // held**: this event comes back and is weighed again.
                if came_to == TheWork::TheLineReachedNothing && sending.frame != this_frame.frame {
                    pacing.held = Some(sending.clone());
                }
            }
            came_to
        });
        came_to
    }

    /// **Whether a finished job's own message waits for its last frame's
    /// pictures** — the runner's ruling of 2026-10-19 on the live pass's §7:
    /// *the job's finished message waits for its last frame's pictures, within
    /// the picture wait the worker already keeps, so the album never arrives
    /// after it.* It waits only where the job's pictures are sent at all
    /// (`Telegram` V1, and not for a check's first frame, which its waiting
    /// message shows), and only until this run has done the last frame's main
    /// copy's event — sent, held under `Telegram` I2a, or not sent — or
    /// [`HOW_LONG_A_CONTACT_SHEET_WAITS_FOR_ITS_PICTURES`] has passed since the
    /// ending.
    ///
    /// **It does not wait where no picture is coming**, from the frame's own
    /// inventory for a video-only job (the sixteenth notes, piece 1), or its
    /// production records (`Queue & History` P1b queues one for every image frame at
    /// the ending): every production of that frame ended and none published, or
    /// none recorded and the frame's copy already on the set. A production
    /// standing, or none recorded and no copy yet, is a picture coming; so is a
    /// copy published whose event this run has not done.
    ///
    /// **What it costs**: a worker restarted between sending the last frame's
    /// album and reading the ending finds the album's event behind its cursor
    /// and waits the whole wait before *finished* — late, never lost.
    fn the_last_frames_picture_is_awaited(
        &self,
        job: &Identity,
        record: &JobRecord,
        manifest: Option<&SubmissionManifest>,
        ended_at: Option<u64>,
    ) -> bool {
        use crate::the_preview_worker::WhatTheProductionCameTo;
        let Some(manifest) = manifest else {
            return false;
        };
        if !the_job_reports(self.records.the_choice_in_force(record), AKindOfReport::AFramePicture)
            || self.it_is_a_first_frame_check(job)
            || self.records.the_video_at_the_ending(record).is_some()
        {
            return false;
        }
        let Some(last) = self.records.the_frames_of(record, manifest).last().copied() else {
            return false;
        };
        if self.pictures_done.contains(&(job.as_text(), last)) {
            return false;
        }
        let waited = ended_at.map_or(u64::MAX, |ended| self.now().saturating_sub(ended));
        if waited >= HOW_LONG_A_CONTACT_SHEET_WAITS_FOR_ITS_PICTURES.as_secs() {
            return false;
        }
        let productions = self.records.what_the_productions_of_a_frame_came_to(job, last);
        if productions.is_empty() {
            return self.records.the_published_picture(record, job, "", last).is_none();
        }
        productions
            .iter()
            .any(|came_to| matches!(came_to, None | Some(WhatTheProductionCameTo::Published)))
    }

    /// **`Telegram` M2 — a finished job's contact sheet**: a selection of at
    /// most ten of its frames, in frame order, as the design draws it — the
    /// first frame, the last and eight spread evenly between
    /// (`smoozy_telegram::media::the_contact_sheet`) — sent as an album at the
    /// job's end. **No image is composed and no Blender runs.** Sent where the
    /// job's reporting choice sends it (`Telegram` V1), and not for a check's
    /// first frame, which its waiting message shows.
    ///
    /// **One album per View Layer the chosen frames' sets hold** (`Telegram`
    /// M2 as changed 2026-10-31, the Creator's seventeenth notes, note 10;
    /// Specs `Telegram`, *A contact sheet at the end*): the frames are chosen
    /// once, every copy of each is gathered (as `available_check_copies` does
    /// for the first-frame check), and the copies are matched across frames by
    /// the layer's recorded name (`Job files and outputs` B31a), never by
    /// place ([`the_sheets_of`]). The main copy's album goes first, then each
    /// other layer's in the set's order (`Queue & History` P1, `Queue & History` P4,
    /// `Queue & History` P10); a
    /// frame with no copy for a layer is left out of that layer's album. Each
    /// album names its layer; a job whose chosen frames hold one copy each
    /// sends one album with no name, as before. **The albums go one after
    /// another, each its own once-only part of the event** (`Telegram` F3), so
    /// none goes twice and a refusal is written to the server's record once
    /// per album ([`Self::sent`]). No album carries Stop (`Telegram` C1b), and
    /// their number has no cap.
    ///
    /// **It waits for its pictures**: the previews of a job's last frames are
    /// made after its ending (`Queue & History` P1b), so the sheets wait until
    /// every copy each chosen frame owes is in — no production of it still
    /// standing and one ended with every copy published, not the main one
    /// alone ([`every_copy_of_the_frame_is_in`], the live Telegram pass's
    /// fault 2) — or [`HOW_LONG_A_CONTACT_SHEET_WAITS_FOR_ITS_PICTURES`] has passed, and
    /// then go with what is there. A picture outside Telegram's limits is left
    /// out, since one would make Telegram refuse the album. **Each album goes
    /// in the upload lane and is not waited for** (`Telegram` C6b): its ending
    /// is read on later turns, and only then does the next go.
    /// A video-only job has no sheet and owes no wait (the sixteenth notes,
    /// piece 1); an image output beside a video still keeps this wait.
    #[allow(clippy::too_many_arguments)]
    fn the_contact_sheets(
        &mut self,
        event: &AnEventRead,
        job: &Identity,
        record: &JobRecord,
        manifest: Option<&SubmissionManifest>,
        label: &str,
        choice: TheReportingChoice,
        ended_at: Option<u64>,
    ) -> TheWork {
        if !the_job_reports(choice, AKindOfReport::AContactSheet) || self.it_is_a_first_frame_check(job) {
            return TheWork::Done;
        }
        if self.records.the_video_at_the_ending(record).is_some() {
            return TheWork::Done;
        }
        let Some(manifest) = manifest else {
            return TheWork::Done;
        };
        let frames: Vec<u64> = self
            .records
            .the_frames_of(record, manifest)
            .into_iter()
            .filter_map(|frame| u64::try_from(frame).ok())
            .collect();
        let chosen: Vec<i64> = the_contact_sheet(&frames).into_iter().filter_map(|frame| i64::try_from(frame).ok()).collect();
        let copies = self.records.the_copies_of_the_sheets_frames(record, job, &chosen);
        let sheets = the_sheets_of(&copies);
        let named = sheets.len() > 1;
        let parts: Vec<String> = sheets
            .iter()
            .map(|sheet| match &sheet.layer {
                Some(layer) if named => format!("its contact sheet · {}", layer.as_said()),
                _ => "its contact sheet".to_string(),
            })
            .collect();
        let begun = parts.iter().any(|part| {
            let key = format!("{} {part}", event.event);
            self.done.contains(&key) || self.sheets_in_flight.contains_key(&key)
        });
        let waited = ended_at.map_or(u64::MAX, |ended| self.now().saturating_sub(ended));
        let all_in = copies.iter().all(|frame| frame.its_pictures_are_in);
        if !begun && !all_in && waited < HOW_LONG_A_CONTACT_SHEET_WAITS_FOR_ITS_PICTURES.as_secs() {
            return TheWork::NotYet;
        }
        for (sheet, part) in sheets.into_iter().zip(parts) {
            let key = format!("{} {part}", event.event);
            let layer = if named { sheet.layer } else { None };
            let came_to = self.once(event, &part, |reader| {
                reader.one_contact_sheet(&key, job, label, layer, frames.len(), &sheet.pictures)
            });
            if came_to != TheWork::Done {
                return came_to;
            }
        }
        // **A sheet Telegram refused is said once in the chat**, once every
        // album has gone or been refused (`Telegram` M2, as `Telegram` I3 says for a
        // picture; the live Telegram pass's fault 3: a 413 was written to the
        // server's record and the user told nothing).
        if self.sheets_refused.contains(&job.as_text()) {
            let text = a_user_message(&self.server, label, "", "Telegram rejected the contact sheet.");
            return self.once(event, "its contact sheet refused", |reader| reader.send_text(text));
        }
        TheWork::Done
    }

    /// **One View Layer's album of the contact sheet** (`Telegram` M2), offered
    /// to the upload lane or, where it is already there, its ending read
    /// (`Telegram` C6b). The pictures are read only when it is offered.
    fn one_contact_sheet(
        &mut self,
        key: &str,
        job: &Identity,
        label: &str,
        layer: Option<WhichPictureItIs>,
        of: usize,
        pictures: &[ACopyOnTheSheet],
    ) -> TheWork {
        // **`Telegram` C1b: the contact sheet comes after the ending, so it
        // carries no Stop and has no companion to carry one.**
        if let Some(told) = self.sheets_in_flight.remove(key) {
            return match told.try_recv() {
                Ok(ended) => {
                    if matches!(&ended, HowASendEnded::HandedBack(why) if !matches!(why, WhyTheCallFailed::NotReached(_))) {
                        self.sheets_refused.insert(job.as_text());
                    }
                    self.sent(ended)
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    self.sheets_in_flight.insert(key.to_string(), told);
                    TheWork::NotYet
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => TheWork::TheLineReachedNothing,
            };
        }
        let pictures: Vec<ADerivative> = pictures
            .iter()
            .filter_map(|copy| {
                let bytes = std::fs::read(&copy.at).ok().filter(|bytes| !bytes.is_empty())?;
                Some(ADerivative {
                    job: job.as_text(),
                    job_label: label.to_string(),
                    frame: u64::try_from(copy.frame).unwrap_or(0),
                    file_name: copy.file_name.clone(),
                    bytes,
                    which_picture: copy.layer.clone(),
                })
            })
            .filter(|picture| whether_it_fits(&picture.bytes) != WhetherItFits::OutsideTheLimits)
            .collect();
        let Some(told) = offer_the_contact_sheet(
            &self.messages.queue,
            self.messages.conversation,
            &self.server,
            label,
            layer,
            of,
            pictures,
        ) else {
            return TheWork::Done;
        };
        self.sheets_in_flight.insert(key.to_string(), told);
        TheWork::NotYet
    }

    /// **`Telegram` M10 — a job that writes a video sends the finished video
    /// at a finished ending**, where its choice would send its contact sheet:
    /// each file at most the cap goes as one video message, in name order,
    /// the next only once the last has ended. A file over the cap is not sent;
    /// where the job's own finishing message did not already say so — a job
    /// in a group, or one that writes pictures as well — one short message
    /// says it. **A first-frame check sends none**, as it sends no contact
    /// sheet. Added 2026-10-27, the nineteenth notes, note 4.
    fn the_finished_videos(
        &mut self,
        event: &AnEventRead,
        job: &Identity,
        record: &JobRecord,
        manifest: Option<&SubmissionManifest>,
        label: &str,
        choice: TheReportingChoice,
    ) -> TheWork {
        if !the_job_reports(choice, AKindOfReport::AFinishedVideo) || self.it_is_a_first_frame_check(job) {
            return TheWork::Done;
        }
        let marked_a_check = manifest
            .and_then(|manifest| {
                smoozy_render_settings_machine::payload::Payload::read_back(&manifest.settings_payload)
            })
            .is_some_and(|payload| payload.the_check_is_written_as_a_picture);
        if marked_a_check {
            return TheWork::Done;
        }
        let Some(files) = self.records.the_finished_videos(record) else {
            return TheWork::Done;
        };
        let the_finishing_message_said_it =
            record.group.is_none() && self.records.the_video_at_the_ending(record).is_some();
        for file in files {
            let part = format!("its video {}", file.path.display());
            let key = format!("{} {part}", event.event);
            let came_to = self.once(event, &part, |reader| {
                reader.one_finished_video(&key, &file, label, the_finishing_message_said_it)
            });
            if came_to != TheWork::Done {
                return came_to;
            }
        }
        TheWork::Done
    }

    /// **One finished video file**, sent, said too big, or waited on while its
    /// upload is in the lane (`Telegram` C6b).
    fn one_finished_video(&mut self, key: &str, file: &AFinishedVideoFile, label: &str, said: bool) -> TheWork {
        if let Some(told) = self.sheets_in_flight.remove(key) {
            return match told.try_recv() {
                Ok(ended) => self.sent(ended),
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    self.sheets_in_flight.insert(key.to_string(), told);
                    TheWork::NotYet
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => TheWork::TheLineReachedNothing,
            };
        }
        let Some(bytes) = file.bytes else {
            self.note(&format!("a finished video's size could not be read and it is not sent: {}", file.name));
            return TheWork::Done;
        };
        if !a_finished_video_fits(bytes) {
            if said {
                return TheWork::Done;
            }
            return self.send_text(the_finished_video_is_too_big(&self.server, label, &file.name, bytes));
        }
        let contents = match std::fs::read(&file.path) {
            Ok(contents) => contents,
            Err(why) => {
                self.note(&format!("a finished video could not be read and is not sent: {}: {why}", file.name));
                return TheWork::Done;
            }
        };
        let told = offer_the_finished_video(
            &self.messages.queue,
            self.messages.conversation,
            the_finished_videos_caption(&self.server, label, &file.name),
            file.name.clone(),
            contents,
        );
        self.sheets_in_flight.insert(key.to_string(), told);
        TheWork::NotYet
    }

    // ------------------------------------------------------------ holds

    /// **What a hold waits for**, read off its reason (`Rented-server
    /// runtime` U2; `Telegram` A3).
    fn what_it_waits_for(hold: &HoldRecord) -> WhatTheHoldWaitsFor {
        if hold.reason == THE_DRIVERS_REASON {
            WhatTheHoldWaitsFor::TheDriverUpdate
        } else {
            WhatTheHoldWaitsFor::TheUsersAnswer
        }
    }

    /// **`Telegram` A1 and A3 — a hold placed.** A hold for the driver is told
    /// nothing. A hold whose first frame is still to come is told when that
    /// frame's job finishes ([`Self::the_holds_waiting_on`]); otherwise now.
    fn a_hold_placed(&mut self, event: &AnEventRead) -> TheWork {
        let Some(hold) = event.about.as_deref().and_then(|about| Identity::read_back(IdentityKind::Operation, about)) else {
            return TheWork::Done;
        };
        self.a_hold_is_told(hold)
    }

    /// **A hold's waiting message, where it is owed now** — from its placing,
    /// or from `Telegram` V1c's change that turned a silent check on.
    fn a_hold_is_told(&mut self, hold: Identity) -> TheWork {
        let Some(record) = self.records.the_hold(&hold) else {
            return TheWork::Done;
        };
        if !Self::what_it_waits_for(&record).the_phone_is_told() || record.release.is_some() {
            return TheWork::Done;
        }
        if let Subject::Job(job) = &record.subject {
            if self.records.the_job_record(job)
                .is_some_and(|held| self.records.the_hold_reports_nothing(&record, &held)) {
                return TheWork::Done;
            }
        }
        let (label, held) = self.the_label_and_count_of(&record);
        let the_first_frame = self.the_first_frame_of(&record);
        self.remembered.insert(hold.as_text(), (label, held, the_first_frame));
        if let Some(dependency) = &record.dependency {
            let finished = self
                .records
                .the_note(&dependency.work)
                .and_then(|note| note.ending)
                .is_some_and(|ending| ending.outcome == TheOutcomeAsWritten::Finished);
            if !finished {
                // **Its frame is not back**: the dependency's own ending says
                // it, or drops the hold (`Telegram` A4).
                return TheWork::Done;
            }
        }
        let the_frames_end = record
            .dependency
            .as_ref()
            .and_then(|dependency| self.records.the_note(&dependency.work)?.ended)
            .and_then(|ended| seconds_of(ended.taken_at()));
        // **One waiting message per hold, whichever event sends it** — the
        // placing, or the first frame's ending (`Self::the_holds_waiting_on`).
        // Where both are read in one turn, as after a restart, it was said
        // twice until 2026-10-16 (found by the map's piece 4).
        let key = format!("the waiting message for {}", hold.as_text());
        if self.done.contains(&key) {
            return TheWork::Done;
        }
        let came_to = self.the_waiting_message(&record, the_frames_end);
        if came_to == TheWork::Done {
            self.done.insert(key);
        }
        came_to
    }

    /// **The waiting message of every unanswered hold whose first frame this
    /// job was** — read off the queue's held records, since a hold record is
    /// reached by its identity and never by listing (`Rented-server runtime`
    /// U2's store).
    fn the_holds_waiting_on(&mut self, job: &Identity, ended_at: Option<u64>) -> TheWork {
        let Some(every) = self.records.every_job_record() else {
            return TheWork::Done;
        };
        let mut came_to = TheWork::Done;
        for one in every {
            let QueueState::Held { hold } = &one.state else { continue };
            let Some(record) = self.records.the_hold(hold) else { continue };
            if record.release.is_some()
                || !Self::what_it_waits_for(&record).the_phone_is_told()
                || record.dependency.as_ref().map(|dependency| &dependency.work) != Some(job)
            {
                continue;
            }
            let key = format!("the waiting message for {}", hold.as_text());
            if self.done.contains(&key) {
                continue;
            }
            match self.the_waiting_message(&record, ended_at) {
                TheWork::Done => {
                    self.done.insert(key);
                }
                TheWork::NotYet => came_to = TheWork::NotYet,
                TheWork::TheLineReachedNothing => return TheWork::TheLineReachedNothing,
            }
        }
        came_to
    }

    /// `Telegram` A3a: send every available check copy, then a separate question for the
    /// held range. Re-read the hold before sending and retain completed batches
    /// when the question reaches nothing.
    fn the_waiting_message(&mut self, hold: &HoldRecord, the_frames_end: Option<u64>) -> TheWork {
        let key = hold.hold.as_text();
        if self.holds_failed_this_turn.contains(&key) { return TheWork::NotYet; }
        let work = self.the_waiting_message_now(hold, the_frames_end);
        if work == TheWork::TheLineReachedNothing { self.holds_failed_this_turn.insert(key); }
        work
    }

    fn the_waiting_message_now(&mut self, hold: &HoldRecord, the_frames_end: Option<u64>) -> TheWork {
        if self
            .records
            .the_hold(&hold.hold)
            .is_some_and(|now| now.release.is_some())
        {
            return TheWork::Done;
        }
        let Subject::Job(held_job) = &hold.subject else {
            return TheWork::Done;
        };
        let Some(held) = self.records.the_job_record(held_job) else {
            return TheWork::Done;
        };
        if self.records.the_hold_reports_nothing(hold, &held) { return TheWork::Done; }
        let manifest = self.records.the_manifest(&held);
        let label = match (self.records.the_note(held_job), manifest.as_ref()) {
            (Some(note), manifest) => note.what_the_job_is_called(manifest),
            (None, Some(manifest)) => smoozy_queue_history_machine::naming::the_files_name(&manifest.blend_path_on_the_machine),
            (None, None) => String::new(),
        };
        let the_held_frames = manifest
            .as_ref()
            .map(|manifest| self.records.the_frames_of(&held, manifest))
            .unwrap_or_default();
        let frames_held = the_held_frames.len() as u64;
        let the_frames = smoozy_telegram::job_messages::the_frames_in_words(
            the_held_frames.iter().map(|frame| *frame as i64),
        );
        // **The first frame, and its pictures where they are published** —
        // with whether every copy that frame owes is in, which the wait below
        // reads (`Telegram` A3a; the live Telegram pass's fault 5).
        let (the_frame_shown, pictures, its_pictures_are_in) = match &hold.dependency {
            Some(dependency) => match self.records.the_job_record(&dependency.work) {
                Some(first) => {
                    let frame = self.records.the_manifest(&first).and_then(|manifest| {
                        self.records
                            .the_frames_of(&first, &manifest)
                            .first()
                            .copied()
                    });
                    let check_label = self.records.what_the_job_is_called(&dependency.work);
                    let pictures = frame
                        .map(|frame| {
                            self.records.available_check_copies(
                                &first,
                                &dependency.work,
                                &check_label,
                                frame,
                            )
                        })
                        .filter(|copies| !copies.is_empty());
                    let in_ = frame.is_some_and(|frame| {
                        self.records.every_copy_of_the_frame_is_in_for(&first, &dependency.work, frame)
                    });
                    (frame, pictures, in_)
                }
                None => (None, None, false),
            },
            None => (None, None, false),
        };
        let the_frame_shown = the_frame_shown.or_else(|| {
            manifest
                .as_ref()
                .and_then(|manifest| self.records.the_frames_of(&held, manifest).first().copied())
        });
        // **The question waits for the check's frame's every copy**, not its
        // first: the frame's other layers are made after its main copy in the
        // same production, and their names come only from its ending, so a
        // question sent once the main copy alone was there carried one
        // picture, named nothing (the live Telegram pass's fault 5). Within
        // the same wait as before, then what is there goes.
        if hold.dependency.is_some() && (pictures.is_none() || !its_pictures_are_in) {
            let waited = the_frames_end.map_or(u64::MAX, |end| self.now().saturating_sub(end));
            if waited < HOW_LONG_A_WAITING_MESSAGE_WAITS_FOR_ITS_PICTURE.as_secs() {
                return TheWork::NotYet;
            }
        }
        let waiting = AWaitingJob {
            server: self.server.clone(),
            label,
            hold: hold.hold.as_text(),
            generation: hold.hold.as_text(),
            waits_for: Self::what_it_waits_for(hold),
            the_frame_shown: the_frame_shown.and_then(|frame| u64::try_from(frame).ok()).unwrap_or(1),
            frames_held,
            the_frames,
        };
        let subject = TheSubject {
            owning_machine: self.machine.clone(),
            job: Some(held_job.as_text()),
            hold: Some(hold.hold.as_text()),
            generation: hold.hold.as_text(),
        };
        let copies = pictures.unwrap_or_default();
        let no_picture = copies.is_empty();
        for (part, batch) in smoozy_telegram::pictures::the_sends_of_a_frame(copies)
            .into_iter()
            .enumerate()
        {
            let key = format!("hold {} picture batch {part}", hold.hold.as_text());
            if self.done.contains(&key) {
                continue;
            }
            match self.pictures.send_a_frame_with_its_stop(
                &mut self.messages,
                batch,
                AClassOfSend::EndingsHoldsAndDrops,
                None,
            ) {
                APictureSent::NotAccepted(ended) if !the_work_is_done(&ended) => {
                    return TheWork::TheLineReachedNothing
                }
                _ => {
                    self.done.insert(key);
                }
            }
        }
        // Picture batches already accepted stay done when this text reaches nothing.
        match self
            .messages
            .send_the_waiting_question(&waiting, no_picture, &subject)
        {
            Ok(_) => TheWork::Done,
            Err(why) => self.sent_with_buttons(Err(why)),
        }
    }

    /// **What the phone was shown of a hold** — the label and the held count
    /// its waiting message's buttons carry, read off their action-context
    /// records in the worker's own store. **Found by the store's one way of
    /// going through its records**, a removal whose test removes nothing: no
    /// record goes and no request is made.
    fn what_the_phone_was_shown_of(&self, hold: &Identity) -> Option<(String, u64)> {
        let found: Mutex<Option<(String, u64)>> = Mutex::new(None);
        let mut store = self.store.clone();
        let _ = store.remove_action_contexts_where(&|record| {
            if record.hold.as_deref() == Some(hold.as_text().as_str()) && record.action == ANSWER_A_WAITING_JOB {
                let label = record.payload["labels"]["label"].as_str().unwrap_or_default().to_string();
                let held = record.payload["frames_held"].as_u64().unwrap_or(0);
                *found.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some((label, held));
            }
            false
        });
        found.into_inner().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// **`Telegram` A3 and A4 — a hold's release.** An answer: which was
    /// applied, through [`TheOwnersActs::report`]. A drop: why, as its own
    /// message, told from the drop's own entry where it carries one.
    fn a_release(&mut self, event: &AnEventRead) -> TheWork {
        let Some(hold) = event.about.as_deref().and_then(|about| Identity::read_back(IdentityKind::Operation, about)) else {
            return TheWork::Done;
        };
        let word = event.says.rsplit(": ").next().unwrap_or_default();
        let record = self.records.the_hold(&hold);
        if record.as_ref().is_some_and(|hold| {
            let Subject::Job(job) = &hold.subject else { return false };
            self.records.the_job_record(job)
                .is_some_and(|held| self.records.the_hold_reports_nothing(hold, &held))
        }) { return TheWork::Done; }
        // **Its question's buttons all go first**, whoever answered.
        let came_to = self.once(event, "the question's buttons", |reader| reader.the_questions_buttons_go(&hold));
        if came_to != TheWork::Done {
            return came_to;
        }
        let subject = Some(self.subject_of_hold(&hold).unwrap_or_else(|| TheSubject {
            owning_machine: self.machine.clone(),
            job: None,
            hold: Some(hold.as_text()),
            generation: hold.as_text(),
        }));
        // **A drop's own entry first** (`Rented-server runtime` U20 as dated
        // 2026-10-16): the machine's words at the drop, which a worker started
        // after the placing still has. What this run remembers, or what any
        // record still says, fills only what the entry leaves out.
        if word == "dropped" {
            if let Some(entry) = what_a_drops_entry_says(&event.says) {
                if entry.the_phone_is_silent { return TheWork::Done; }
                let remembered = self.remembered.get(&hold.as_text()).cloned();
                let label = if entry.label.is_empty() {
                    remembered.as_ref().map(|(label, _, _)| label.clone()).unwrap_or_default()
                } else {
                    entry.label.clone()
                };
                let frames_held = entry.frames_held.or(remembered.as_ref().map(|(_, held, _)| *held)).unwrap_or(0);
                let the_frame_shown = entry
                    .the_first_frame
                    .or(remembered.as_ref().and_then(|(_, _, frame)| *frame))
                    .and_then(|frame| u64::try_from(frame).ok())
                    .unwrap_or(1);
                let text = the_drop_message(
                    &self.server,
                    &label,
                    frames_held,
                    the_frame_shown,
                    why_it_was_dropped(&entry.reason),
                );
                return self.once(event, "the drop", |reader| reader.send_text(text));
            }
        }
        // **What the phone was shown first**: a *no* removes its subject and
        // the hold record with it in the same act (`Rented-server runtime`
        // U12a), so the records are usually gone by now.
        let remembered = self.remembered.get(&hold.as_text()).cloned();
        let shown = self
            .what_the_phone_was_shown_of(&hold)
            .or_else(|| remembered.as_ref().map(|(label, held, _)| (label.clone(), *held)));
        let (label, frames_held, waits_for) = match (&shown, &record) {
            (Some((label, held)), record) => (
                label.clone(),
                *held,
                record.as_ref().map_or(WhatTheHoldWaitsFor::TheUsersAnswer, Self::what_it_waits_for),
            ),
            (None, Some(record)) => {
                let (label, held) = self.the_label_and_count_of(record);
                (label, held, Self::what_it_waits_for(record))
            }
            // **Nothing was shown and nothing is left**: the phone never heard
            // of this hold, and there is nothing true to compose.
            (None, None) => {
                self.note(&format!(
                    "{} was released and nothing of it is left to tell: this run did not read its placing",
                    hold.as_text()
                ));
                return TheWork::Done;
            }
        };
        if word == "dropped" {
            // **The reason**: the hold record's, where it stands; where it went
            // with its subject, the one drop that removes both — `Queue &
            // History` H9's, the first frame's job cancelled before it ran.
            let why = why_it_was_dropped(
                record.as_ref().and_then(|record| record.release.as_ref()).map_or("", |release| release.produced_by.as_str()),
            );
            let the_frame_shown = remembered
                .as_ref()
                .and_then(|(_, _, frame)| *frame)
                .or_else(|| record.as_ref().and_then(|record| self.the_first_frame_of(record)))
                .and_then(|frame| u64::try_from(frame).ok())
                .unwrap_or(1);
            let text = the_drop_message(&self.server, &label, frames_held, the_frame_shown, why);
            return self.once(event, "the drop", |reader| reader.send_text(text));
        }
        let answer = match word {
            "released" => TheAnswer::GoOn,
            "stopped" => TheAnswer::Dont,
            _ => return TheWork::Done,
        };
        self.once(event, "the answer applied", |reader| {
            let report = AJournalledOutcome::ARelease {
                server: reader.server.clone(),
                label: label.clone(),
                answer,
                frames_held,
                waits_for,
                outcome: AReleasesOutcome::Applied,
            };
            let reported = match &subject {
                Some(subject) => {
                    reader
                        .reporter
                        .report_about(report, subject, &reader.server, &label)
                }
                None => reader.reporter.report(report),
            };
            reported.map_or(TheWork::Done, |ended| reader.sent(ended))
        })
    }

    /// **The first frame a hold waits on** — its dependency's first frame,
    /// or failing one, the held job's own.
    fn the_first_frame_of(&self, hold: &HoldRecord) -> Option<i64> {
        self.records.the_first_frame_of(hold)
    }

    /// **`Telegram` A3 and `Telegram` C1b — a released hold's question keeps no button**:
    /// both answers and its Stop go from the *Needs your answer* message,
    /// whether the desk or the phone answered, or the hold was dropped, and
    /// every record on it is marked, so nothing rebuilds it; the *Answered …*
    /// message carries the Stop from then on (the live Telegram pass's fault
    /// 6: the question kept all three after a desk answer). The question is
    /// found by its answers' records in the worker's own store, which bind
    /// the hold. A line that reaches nothing is tried again with the event;
    /// any other failure is noted and passed, as the ending's own removal of
    /// Stops is.
    fn the_questions_buttons_go(&mut self, hold: &Identity) -> TheWork {
        let records = match self.messages.store.the_action_contexts() {
            Ok(records) => records,
            Err(why) => {
                self.note(&format!("the question of {} kept its buttons: {}", hold.as_text(), why.0));
                return TheWork::Done;
            }
        };
        let questions: std::collections::BTreeSet<i64> = records
            .iter()
            .filter(|record| {
                record.conversation == self.messages.conversation
                    && record.action == ANSWER_A_WAITING_JOB
                    && record.hold.as_deref() == Some(hold.as_text().as_str())
            })
            .filter_map(|record| record.source_message)
            .collect();
        for message in questions {
            match self.messages.remove_every_button_on(message, AClassOfSend::EndingsHoldsAndDrops) {
                Ok(_) => {}
                Err(why) if why.the_line_reached_nothing() => return TheWork::TheLineReachedNothing,
                Err(why) => self.note(&format!("the question of {} kept its buttons: {why:?}", hold.as_text())),
            }
        }
        TheWork::Done
    }

    /// A held job's name and count, read off its records.
    fn the_label_and_count_of(&self, hold: &HoldRecord) -> (String, u64) {
        self.records.the_label_and_count_of(hold)
    }

    // ------------------------------------------------------------ a changed choice

    /// **`Telegram` V1c — a live job's reporting choice changed.** Nothing is
    /// said about the change itself, and nothing passed over before it is sent:
    /// every later event is read under the choice in force when it is read.
    /// Two things move here.
    ///
    /// - **`Telegram` I2a's clock starts again.** The job's pacing is forgotten, with any
    ///   frame held under the old choice, so under `APictureEveryFewMinutes`
    ///   the next finished frame goes at once and counts from there.
    /// - **A held check turned on gets `Telegram` A2's question now**, with its pictures,
    ///   where its first frame is back. A question already sent is not sent
    ///   again: its buttons' records in the worker's own store say it went,
    ///   across restarts too. A check whose frame is not back yet is told at
    ///   that frame's ending, as any check is.
    fn a_choice_changed(&mut self, event: &AnEventRead) -> TheWork {
        let Some(job) = event.about.as_deref().and_then(|about| Identity::read_back(IdentityKind::Job, about)) else {
            return TheWork::Done;
        };
        self.paced.remove(&job.as_text());
        let Some(record) = self.records.the_job_record(&job) else {
            return TheWork::Done;
        };
        let QueueState::Held { hold } = &record.state else {
            return TheWork::Done;
        };
        if self.records.the_job_reports_nothing(&record) || self.what_the_phone_was_shown_of(hold).is_some() {
            return TheWork::Done;
        }
        // **A silent check's ending marked its waiting message done with
        // nothing sent**; the button records above are what say one went.
        self.done.remove(&format!("the waiting message for {}", hold.as_text()));
        self.a_hold_is_told(hold.clone())
    }

    // ------------------------------------------------------------ removals

    /// **`Queue & History` L22 — a waiting job dropped because its Blender was
    /// removed**, told off the event alone. A cancel or a clear is the user's
    /// own act, and nothing in `Telegram` tells the phone of it.
    fn a_removal(&mut self, event: &AnEventRead) -> TheWork {
        let Some((version, label)) = asked::what_a_drop_says(&event.says) else {
            return TheWork::Done;
        };
        // **`Telegram` C1b: a drop is an ending, and carries no Stop.**
        let text = the_drop_for_a_removed_blender(&self.server, &label, &version);
        self.once(event, "the drop", |reader| reader.send_text(text))
    }

    // ------------------------------------------------------------ idle

    /// **`Telegram` V3 — the idle message, once per idle stretch**, where V2's
    /// switch is on: the machine idle by `Queue & History` W1 (nothing
    /// rendering, nothing in the line that can start), the quiet moment W2's
    /// (the latest event that left the line, or failing any this run read, the
    /// latest ending on a note), and the half hour W3's, all on this machine's
    /// own clock. **No spoken mark is written** (G3): a restart may say it again.
    fn the_idle_message(&mut self) {
        if !self.the_idle_switch {
            return;
        }
        let Some(quiet) = self.quiet_since else { return };
        let Some(every) = self.records.every_job_record() else { return };
        let idle = !every.iter().any(|one| matches!(one.state, QueueState::Rendering { .. } | QueueState::Waiting));
        let past = self.now().saturating_sub(quiet) >= THE_HALF_HOUR;
        let stretch = quiet.to_string();
        let Some(text) = self.idle.what_to_say(&self.server, true, idle, Some(&stretch), past) else {
            return;
        };
        if self.send_text(text) != TheWork::Done {
            // **Never silence**: the stretch is forgotten so the next turn
            // says it again.
            self.idle = TheIdleMessage::default();
        }
    }
}

// ---------------------------------------------------------------------------
// What a drop's journal entry carries
// ---------------------------------------------------------------------------

/// **What the release event of a drop says after everything it carries** —
/// the supervisor's own words for any release (`Rented-server runtime` U26),
/// unchanged, so the release outcome stays the last thing the entry says and
/// the desktop reads it as it always has.
pub const THE_TAIL_OF_A_DROPS_ENTRY: &str = "the hold's release outcome was written: dropped";

/// **The bound on a dropped job's label in the entry**, the one `Queue &
/// History` L22's drop uses, so the entry stays inside `State, identity and
/// recovery` E6a's per-entry budget.
pub const THE_BOUND_ON_A_DROPPED_LABEL: usize = 160;

/// **A drop's journal entry, as the Telegram worker reads it back** —
/// `Rented-server runtime` U20 as dated 2026-10-16: the reason, the held
/// count, the first frame and a bounded copy of the held job's label, because
/// the drop removes every record that held them in the same act
/// (`Queue & History` H11), and a worker started between the placing and the
/// drop has nothing else to tell the phone from. Added 2026-10-16, the
/// Telegram worker's piece 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADropAsJournalled {
    /// `Telegram` V1b's silence, saved before the held job's records are removed.
    pub the_phone_is_silent: bool,
    /// The release's own reason — *the dependency failed*, *… was stopped*,
    /// *… was given up*, *… was cancelled*.
    pub reason: String,
    /// The frames the held job covered, where its manifest named them.
    pub frames_held: Option<u64>,
    /// The first frame's number, where its job's manifest named it.
    pub the_first_frame: Option<i64>,
    pub label: String,
}

/// **A drop's journal entry, composed** — the supervisor's half. The reason
/// first and the label last before the tail, so a label holding anything
/// reads back whole; numbers nobody could read are written empty, never as
/// a nought.
pub fn the_drops_entry(drop: &ADropAsJournalled) -> String {
    let mut end = drop.label.len().min(THE_BOUND_ON_A_DROPPED_LABEL);
    while !drop.label.is_char_boundary(end) {
        end -= 1;
    }
    format!(
        "{}{}: {}: {}: {}: {THE_TAIL_OF_A_DROPS_ENTRY}",
        if drop.the_phone_is_silent { "Telegram off: " } else { "" },
        drop.reason,
        drop.frames_held.map(|n| n.to_string()).unwrap_or_default(),
        drop.the_first_frame.map(|n| n.to_string()).unwrap_or_default(),
        &drop.label[..end],
    )
}

/// **[`the_drops_entry`], read back** — or nothing where the entry is any
/// other release, or one an older program wrote with nothing before its tail.
pub fn what_a_drops_entry_says(says: &str) -> Option<ADropAsJournalled> {
    let carried = says.strip_suffix(THE_TAIL_OF_A_DROPS_ENTRY)?
        .strip_suffix(": ")?;
    let (the_phone_is_silent, carried) = match carried.strip_prefix("Telegram off: ") {
        Some(rest) => (true, rest),
        None => (false, carried),
    };
    let (reason, rest) = carried.split_once(": ")?;
    let (held, rest) = rest.split_once(": ")?;
    let (first, label) = rest.split_once(": ")?;
    Some(ADropAsJournalled {
        the_phone_is_silent,
        reason: reason.to_string(),
        frames_held: held.parse().ok(),
        the_first_frame: first.parse().ok(),
        label: label.to_string(),
    })
}

/// **Why a drop happened, read off the release's own reason** — the words
/// `smoozy_queue_history_machine::acting::held` writes. Anything else is the
/// one drop whose reason is not an ending: the first frame's job cancelled
/// before it ran (`Queue & History` H9).
fn why_it_was_dropped(reason: &str) -> TheDependencyEnded {
    if reason.ends_with("was stopped") {
        TheDependencyEnded::Stopped
    } else if reason.ends_with("was given up") {
        TheDependencyEnded::GivenUp
    } else if reason.ends_with("failed") {
        TheDependencyEnded::Failed
    } else {
        TheDependencyEnded::Cancelled
    }
}

/// **The words between the queue state and the failed job in the event a
/// machine-made job's record is journalled under** — the smallest mark that
/// tells a job this machine made for itself, on a phone's *try it again*
/// (`Telegram` Y1), from one the desktop sent. Added 2026-10-16, the Telegram
/// worker's piece 5.
///
/// **Why the event, and why its words.** The resubmission is told to the phone
/// off the journal, like every other outcome (`Telegram` F3), so a worker
/// started after the job was made still tells it. The event is the one every
/// job's record is journalled under (`Queue & History` Q3), so every reader
/// that counts the line's moves goes on counting this one. **What it says is
/// Build's own words** — `State, identity and recovery` E1's *what happened*,
/// which no plan words for this kind — and the desktop reads this kind by its
/// name and the job it is about, never by what it says. **It says the queue
/// state first, as every such event does**, then this, then the failed job.
/// Nothing in the job record, the note or the manifest carries the fact, and
/// each of those is a plan's own list.
pub const SUBMITTED_AGAIN_BY_THIS_MACHINE_FROM: &str = "submitted again by this machine from";

/// **What a machine-made job's event says**: its queue state, the mark, and
/// the job it was made from.
pub fn what_a_resubmissions_event_says(state: &str, failed: &Identity) -> String {
    format!("{state}: {SUBMITTED_AGAIN_BY_THIS_MACHINE_FROM} {}", failed.as_text())
}

/// **[`what_a_resubmissions_event_says`], read back**: the job it was made
/// from, or nothing where the event says anything else — which is what every
/// job the desktop sent says.
pub fn the_job_a_resubmission_was_made_from(says: &str) -> Option<Identity> {
    let (_, after) = says.split_once(&format!(": {SUBMITTED_AGAIN_BY_THIS_MACHINE_FROM} "))?;
    Identity::read_back(IdentityKind::Job, after)
}

/// **The base generation a button about a job is valid in** (`Telegram` R1): the
/// identity the job's queue state carries — the render while it runs, the
/// recorded outcome once ended, the hold while held. **A hold's own buttons
/// carry the hold's identity.** Piece 4's reader of what this machine holds
/// answers by the same rule, so a button stays live exactly as long as the
/// state it was sent about. `action_generation` adds the note's attempt number.
pub fn the_generation_of(record: &JobRecord) -> String {
    match &record.state {
        QueueState::Waiting => record.job.as_text(),
        QueueState::Held { hold } => hold.as_text(),
        QueueState::Rendering { operation } => operation.as_text(),
        QueueState::Ended { outcome } => outcome.as_text(),
    }
}

/// **Which of the frame's pictures a published copy is, as its event says**
/// (`Telegram` I2, added 2026-10-17, the Creator's tenth notes, note 2): the
/// View Layer's name where the event carries one, or else the source
/// `Job files and outputs` B31a recorded. **Both are carried only for a frame
/// of more than one View Layer**, so a frame of one names nothing; nor does an
/// event from before, or one whose name was left to the record for length
/// (`Queue & History` P3). A name is the artist's bytes, said as they read.
pub fn which_picture_the_event_names(
    published: &smoozy_queue_history_machine::previews::ThePublishedPreview,
) -> Option<WhichPictureItIs> {
    which_picture_is_named(published.view_layer.as_deref(), published.source.as_ref())
}

/// **The View Layer's name where there is one, or else the source** — one
/// wording for the event and the record alike. A name is the artist's bytes,
/// said as they read.
fn which_picture_is_named(
    view_layer: Option<&[u8]>,
    source: Option<&smoozy_job_files_machine::main_output::WhereThePictureCameFrom>,
) -> Option<WhichPictureItIs> {
    use smoozy_job_files_machine::main_output::WhereThePictureCameFrom;
    let said = |bytes: &[u8]| String::from_utf8_lossy(bytes).into_owned();
    if let Some(name) = view_layer {
        return Some(WhichPictureItIs::AViewLayer(said(name)));
    }
    match source? {
        WhereThePictureCameFrom::TheOutputPath => Some(WhichPictureItIs::TheOutputPath),
        WhereThePictureCameFrom::AFileOutputNode(name) => Some(WhichPictureItIs::AFileOutputNode(said(name.as_bytes()))),
    }
}

/// **`Telegram` I2 as written 2026-10-18 — a frame's other copies, as its
/// preview records name them** (the Creator's fourteenth notes, note 7): every
/// copy beside the main one that a production of `frame` of `job` published,
/// by its place in the frame's set and in place order (`Queue & History` P1,
/// P4), with which picture it is — its View Layer, or its source. `production`
/// names the one production an album goes for, the main copy's event's
/// operation, so the album is made from the copies published under it. Without
/// one — *send a frame*, read from the set — every published production of
/// the frame is read, and a place is named only where every record names it
/// alike, as [`which_picture_the_records_name`] does for the main copy.
pub fn the_other_copies_the_records_name(
    productions: &[smoozy_state::machine_operations::OperationRecord],
    job: &Identity,
    frame: i64,
    production: Option<&str>,
) -> Vec<(u32, Option<WhichPictureItIs>)> {
    the_other_copies_named(productions, job, frame, production, false)
}

fn the_other_copies_named(
    productions: &[smoozy_state::machine_operations::OperationRecord],
    job: &Identity,
    frame: i64,
    production: Option<&str>,
    name_source_file: bool,
) -> Vec<(u32, Option<WhichPictureItIs>)> {
    use crate::the_preview_worker::{ThePreviewKindsOwnField, WhatTheProductionCameTo};
    let mut by_place: BTreeMap<u32, Vec<Option<WhichPictureItIs>>> = BTreeMap::new();
    let ours = productions
        .iter()
        .filter(|record| &record.subject == job)
        .filter(|record| production.is_none_or(|operation| record.operation.as_text() == operation))
        .filter_map(|record| ThePreviewKindsOwnField::read_back(record.the_kinds_safe_boundary_marker.as_deref()?))
        .filter(|field| field.frame == frame)
        .filter_map(|field| field.ending);
    for ending in ours {
        let copies = std::iter::once(&ending)
            .chain(ending.the_other_layers.iter())
            .filter(|copy| copy.came_to == WhatTheProductionCameTo::Published)
            .filter_map(|copy| copy.the_record.as_ref())
            .filter(|copy| !copy.it_is_the_main_copy);
        for copy in copies {
            by_place.entry(copy.its_place_in_the_frames_set).or_default().push(the_copy_named(copy, name_source_file));
        }
    }
    by_place
        .into_iter()
        .map(|(place, names)| {
            let first = names[0].clone();
            let alike = names.iter().all(|name| *name == first);
            (place, if alike { first } else { None })
        })
        .collect()
}

/// **Which of a frame's pictures its main copy is, read off the preview
/// records of its job** (piece T2c of step 10c): the records of `job` whose
/// production is of `frame` and whose main copy was published — the main
/// copy's own ending, so a frame whose other layer's copy failed still
/// counts. **A frame published more than once** (made again after it was
/// cleared, `Telegram` I4) **is named only where every such record names it
/// alike**: the records carry no moment to tell which wrote the file there
/// now, so a disagreement names nothing rather than guess. A frame of one
/// View Layer, whose record carries neither name nor source, names nothing.
pub fn which_picture_the_records_name(
    productions: &[smoozy_state::machine_operations::OperationRecord],
    job: &Identity,
    frame: i64,
) -> Option<WhichPictureItIs> {
    the_main_copy_named(productions, job, frame, false)
}

/// `Telegram` A3a still names a single check copy when its producer omitted the layer
/// and source kind: the recorded source filename is evidence, not a guess.
/// Ordinary frame captions keep their existing single-layer wording.
fn the_copy_named(
    record: &smoozy_job_files_machine::main_output::ADerivativesRecord,
    name_source_file: bool,
) -> Option<WhichPictureItIs> {
    which_picture_is_named(
        record.the_view_layer_its_picture_came_from.as_ref().map(|name| name.as_bytes()),
        record.where_it_came_from.as_ref(),
    ).or_else(|| {
        if !name_source_file { return None; }
        let name = record.the_source_file.rsplit(|byte| *byte == b'/').next()?;
        (!name.is_empty()).then(|| WhichPictureItIs::ASourceFile(String::from_utf8_lossy(name).into_owned()))
    })
}

fn the_main_copy_named(
    productions: &[smoozy_state::machine_operations::OperationRecord],
    job: &Identity,
    frame: i64,
    name_source_file: bool,
) -> Option<WhichPictureItIs> {
    use crate::the_preview_worker::{ThePreviewKindsOwnField, WhatTheProductionCameTo};
    let mut named = productions
        .iter()
        .filter(|record| &record.subject == job)
        .filter_map(|record| ThePreviewKindsOwnField::read_back(record.the_kinds_safe_boundary_marker.as_deref()?))
        .filter(|field| field.frame == frame)
        .filter_map(|field| field.ending)
        .filter(|ending| ending.came_to == WhatTheProductionCameTo::Published)
        .map(|ending| {
            let record = ending.the_record?;
            the_copy_named(&record, name_source_file)
        });
    let first = named.next()?;
    named.all(|one| one == first).then_some(first).flatten()
}

/// **One copy of a frame the contact sheet chose** (`Telegram` M2): where its
/// file sits, its place in the frame's set (none for the main copy), and the
/// name its preview records give it (`Job files and outputs` B31a).
#[derive(Debug, Clone, PartialEq)]
pub struct ACopyOnTheSheet {
    pub frame: i64,
    pub place: Option<u32>,
    pub layer: Option<WhichPictureItIs>,
    pub at: String,
    pub file_name: String,
}

/// **Every copy of one frame the contact sheet chose**, main copy first, and
/// what the sheet's wait reads of it.
#[derive(Debug, Clone, PartialEq)]
pub struct TheCopiesOfAFrameOnTheSheet {
    pub copies: Vec<ACopyOnTheSheet>,
    /// [`every_copy_of_the_frame_is_in`].
    pub its_pictures_are_in: bool,
}

/// **Whether every copy a frame owes is in** — `Telegram` M2's *the wait …
/// covers every copy* and `Telegram` A3a's *complete published set*, read off the
/// frame's preview records (`Queue & History` P1a): **no production of the
/// frame is still standing, one of them ended with every copy it owed
/// published** (`TheProductionsEnding::the_frames_outcome`), and its main copy
/// is there now, as the wait always asked. A layer copy
/// that failed leaves its frame *not made*, owed again at a later production
/// (`Queue & History` P1a), so the main copy alone being there is not enough:
/// between the two productions nothing stands and the main copy is
/// published, which is where the live Telegram pass's *backdrop* sheet went
/// with 3 of 4 frames (its fault 2), and where a check's question went with
/// its main copy alone, named nothing (fault 5). A frame no production is
/// recorded for — records an older program wrote, or none readable — is in
/// where its main copy is published, as before.
pub fn every_copy_of_the_frame_is_in(
    productions: &[smoozy_state::machine_operations::OperationRecord],
    job: &Identity,
    frame: i64,
    its_main_copy_is_published: bool,
) -> bool {
    use crate::the_preview_worker::{ThePreviewKindsOwnField, WhatTheProductionCameTo};
    let fields: Vec<ThePreviewKindsOwnField> = productions
        .iter()
        .filter(|record| &record.subject == job)
        .filter_map(|record| ThePreviewKindsOwnField::read_back(record.the_kinds_safe_boundary_marker.as_deref()?))
        .filter(|field| field.frame == frame)
        .collect();
    if fields.is_empty() {
        return its_main_copy_is_published;
    }
    its_main_copy_is_published
        && fields.iter().all(|field| field.ending.is_some())
        && fields.iter().any(|field| {
            field.ending.as_ref().is_some_and(|ending| ending.the_frames_outcome() == WhatTheProductionCameTo::Published)
        })
}

/// **One View Layer's album of the contact sheet**: the layer's recorded name
/// and its pictures, in frame order.
#[derive(Debug, Clone, PartialEq)]
pub struct OneLayersSheet {
    pub layer: Option<WhichPictureItIs>,
    pub pictures: Vec<ACopyOnTheSheet>,
}

/// **`Telegram` M2 as changed 2026-10-31 — the chosen frames' copies, one album
/// per View Layer.** Copies are matched across frames by the layer's recorded
/// name (`Job files and outputs` B31a), never by place, since a layer can be
/// missing at some frames: a frame with no copy for a layer is left out of
/// that layer's album. **The main copy's album first** — the layer the first
/// published main copy is named — **then each other layer's in the set's
/// order** (`Queue & History` P1, `Queue & History` P4,
/// `Queue & History` P10), read as the lowest place it holds
/// in any chosen frame, ties in the order first met. At most one picture per
/// frame in each album. One album means the job's chosen frames hold one
/// layer, and the caller sends it with no name.
pub fn the_sheets_of(frames: &[TheCopiesOfAFrameOnTheSheet]) -> Vec<OneLayersSheet> {
    let the_main_layer = frames
        .iter()
        .flat_map(|frame| frame.copies.iter())
        .find(|copy| copy.place.is_none())
        .map(|copy| copy.layer.clone());
    let mut sheets: Vec<(u32, OneLayersSheet)> = Vec::new();
    for copy in frames.iter().flat_map(|frame| frame.copies.iter()) {
        let place = copy.place.unwrap_or(0);
        match sheets.iter_mut().find(|(_, sheet)| sheet.layer == copy.layer) {
            Some((lowest, sheet)) => {
                *lowest = (*lowest).min(place);
                if sheet.pictures.iter().all(|one| one.frame != copy.frame) {
                    sheet.pictures.push(copy.clone());
                }
            }
            None => sheets.push((place, OneLayersSheet { layer: copy.layer.clone(), pictures: vec![copy.clone()] })),
        }
    }
    let the_main_ones = the_main_layer
        .and_then(|layer| sheets.iter().position(|(_, sheet)| sheet.layer == layer))
        .map(|at| sheets.remove(at));
    // A stable sort keeps the order first met among equal places.
    sheets.sort_by_key(|(lowest, _)| *lowest);
    the_main_ones.into_iter().chain(sheets).map(|(_, sheet)| sheet).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **`Telegram` M2 as changed 2026-10-31: copies are matched across
    /// frames by the layer's recorded name, never by place.** Frame 1 holds
    /// *Interior* (main), *Reflections* at place 1 and *Shadows* at 2; frame 2
    /// lost *Reflections*, so *Shadows* sits at place 1 there. Three sheets,
    /// the main copy's first, then the set's order; *Reflections* has frame 1
    /// alone, *Shadows* both frames.
    #[test]
    fn seventeenth_10_the_sheets_match_copies_by_name_not_by_place() {
        let copy = |frame: i64, place: Option<u32>, name: &str| ACopyOnTheSheet {
            frame,
            place,
            layer: Some(WhichPictureItIs::AViewLayer(name.into())),
            at: format!("{frame}-{place:?}"),
            file_name: String::new(),
        };
        let frame = |copies: Vec<ACopyOnTheSheet>| TheCopiesOfAFrameOnTheSheet {
            copies,
            its_pictures_are_in: true,
        };
        let sheets = the_sheets_of(&[
            frame(vec![copy(1, None, "Interior"), copy(1, Some(1), "Reflections"), copy(1, Some(2), "Shadows")]),
            frame(vec![copy(2, None, "Interior"), copy(2, Some(1), "Shadows")]),
        ]);
        let said: Vec<(String, Vec<i64>)> = sheets
            .iter()
            .map(|sheet| {
                (
                    sheet.layer.as_ref().map(WhichPictureItIs::as_said).unwrap_or_default(),
                    sheet.pictures.iter().map(|one| one.frame).collect(),
                )
            })
            .collect();
        assert_eq!(
            said,
            [
                ("Interior".to_string(), vec![1, 2]),
                ("Reflections".to_string(), vec![1]),
                ("Shadows".to_string(), vec![1, 2]),
            ]
        );
        // One copy each, named nothing: one sheet.
        let one = |frame: i64| TheCopiesOfAFrameOnTheSheet {
            copies: vec![ACopyOnTheSheet { frame, place: None, layer: None, at: String::new(), file_name: String::new() }],
            its_pictures_are_in: true,
        };
        assert_eq!(the_sheets_of(&[one(1), one(2)]).len(), 1);
        assert!(the_sheets_of(&[]).is_empty());
    }

    #[test]
    fn twenty_first_only_a_new_reach_spends_a_try() {
        let mut first = TheTriesOfASend::default();
        assert!(!first.after_a_failure(80));
        assert_eq!(first.counted, 0, "old registration calls do not spend a first try");
        let mut tries = TheTriesOfASend::default();
        for _ in 0..20 { assert!(!tries.after_a_failure(0)); }
        assert_eq!(tries.counted, 0);
        assert!(!tries.after_a_failure(1));
        for _ in 0..20 { assert!(!tries.after_a_failure(1)); }
        assert_eq!(tries.counted, 1);
        assert!(!tries.after_a_failure(2));
        assert!(tries.after_a_failure(3));
    }

    #[test]
    fn twenty_first_the_unsent_notice_keeps_names_and_counts_utf16() {
        let (text, entities) = the_unsent_notice("Server 🟧", "Pipe <&> 🎬");
        assert_eq!(text, "Server 🟧\nPipe <&> 🎬\nA message about this job could not be sent.");
        assert_eq!(entities[0]["length"], 9);
        assert_eq!(entities[1]["offset"], 10);
        assert_eq!(entities[1]["length"], 11);
    }

    /// **`Telegram` I2, the Creator's tenth notes, note 2: the event's View
    /// Layer, or its source, is handed on**, read back from the event's own
    /// words; a frame of one View Layer, whose event carries neither, names
    /// nothing.
    #[test]
    fn tenth_notes_2_the_event_hands_on_its_layer_or_its_source() {
        use smoozy_job_files_machine::main_output::WhereThePictureCameFrom;
        use smoozy_job_files_machine::reading::ArtistsText;
        use smoozy_queue_history_machine::previews::ThePublishedPreview;
        let one = ThePublishedPreview {
            frame: 140,
            operation: "o".into(),
            view_layer: None,
            main: true,
            place: 0,
            source: None,
        };
        let read = |published: ThePublishedPreview| {
            which_picture_the_event_names(&ThePublishedPreview::read_back(&published.as_says()).expect("it reads"))
        };
        assert_eq!(
            read(ThePublishedPreview { view_layer: Some(b"Reflections".to_vec()), ..one.clone() }),
            Some(WhichPictureItIs::AViewLayer("Reflections".into()))
        );
        assert_eq!(
            read(ThePublishedPreview { source: Some(WhereThePictureCameFrom::TheOutputPath), ..one.clone() }),
            Some(WhichPictureItIs::TheOutputPath)
        );
        assert_eq!(
            read(ThePublishedPreview {
                source: Some(WhereThePictureCameFrom::AFileOutputNode(ArtistsText::of(b"Beauty".to_vec()))),
                ..one.clone()
            }),
            Some(WhichPictureItIs::AFileOutputNode("Beauty".into()))
        );
        assert_eq!(read(one), None, "a frame of one View Layer names none");
    }

    /// **Piece T2c: the preview record a frame was published from names
    /// its picture** — the main copy's layer, or its source, or none for a
    /// frame of one View Layer. Another job's record, another frame's, and
    /// a production whose main copy was not published are not read; a frame
    /// published twice is named only where both records agree.
    #[test]
    fn t2c_the_preview_record_names_its_layer_or_its_source() {
        use crate::the_preview_worker::{ThePreviewKindsOwnField, TheProductionsEnding, WhatTheProductionCameTo};
        use smoozy_job_files_machine::main_output::{ADerivativesRecord, WhereThePictureCameFrom};
        use smoozy_job_files_machine::reading::ArtistsText;
        use smoozy_state::machine_operations::OperationRecord;
        let (kitchen, hall) = (
            Identity::issue(IdentityKind::Job).expect("randomness"),
            Identity::issue(IdentityKind::Job).expect("randomness"),
        );
        let production = |job: &Identity, frame: i64, came_to: WhatTheProductionCameTo, layer: Option<&str>, source: Option<WhereThePictureCameFrom>| {
            let mut record = OperationRecord::opened(
                Identity::issue(IdentityKind::Operation).expect("randomness"),
                job.clone(),
                "preview",
            );
            record.the_kinds_safe_boundary_marker = Some(
                ThePreviewKindsOwnField {
                    job: job.as_text(),
                    frame,
                    attempt: 1,
                    record_directory: "/r".into(),
                    the_job_was_interrupted: false,
                    ordinary_sink: "/r/o".into(),
                    complaints_sink: "/r/c".into(),
                    script: "/r/s.py".into(),
                    ending: Some(TheProductionsEnding {
                        came_to,
                        the_record: Some(ADerivativesRecord {
                            the_source_file: b"/r/x.exr".to_vec(),
                            frame,
                            the_view_layer_its_picture_came_from: layer.map(|name| ArtistsText::of(name.as_bytes().to_vec())),
                            it_is_the_main_copy: true,
                            its_place_in_the_frames_set: 0,
                            where_it_came_from: source,
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
                    }),
                }
                .as_written(),
            );
            record
        };
        use WhatTheProductionCameTo::{NotMade, Published};
        let node = || WhereThePictureCameFrom::AFileOutputNode(ArtistsText::of(b"Beauty".to_vec()));
        let records = [
            production(&hall, 1, Published, Some("Hall's own"), None),
            production(&kitchen, 2, Published, Some("Elsewhere"), None),
            production(&kitchen, 1, NotMade, Some("Not made"), None),
            production(&kitchen, 1, Published, Some("Reflections"), None),
            production(&kitchen, 3, Published, None, Some(WhereThePictureCameFrom::TheOutputPath)),
            production(&kitchen, 4, Published, None, Some(node())),
            production(&kitchen, 4, Published, None, Some(node())),
            production(&kitchen, 5, Published, None, None),
            production(&kitchen, 6, Published, Some("Reflections"), None),
            production(&kitchen, 6, Published, None, Some(WhereThePictureCameFrom::TheOutputPath)),
        ];
        let named = |frame| which_picture_the_records_name(&records, &kitchen, frame);
        assert_eq!(named(1), Some(WhichPictureItIs::AViewLayer("Reflections".into())), "its own job's, its own frame's, published");
        assert_eq!(named(3), Some(WhichPictureItIs::TheOutputPath));
        assert_eq!(named(4), Some(WhichPictureItIs::AFileOutputNode("Beauty".into())), "published twice, named alike");
        assert_eq!(named(5), None, "a frame of one View Layer names none");
        let check_name = the_main_copy_named(&records, &kitchen, 5, true);
        assert_eq!(check_name, Some(WhichPictureItIs::ASourceFile("x.exr".into())));
        assert_eq!(smoozy_telegram::pictures::the_caption("Server", "Check", 5, check_name.as_ref()), "Server\nCheck · Frame 5 · Source: x.exr · Not colour-accurate");
        assert_eq!(named(6), None, "published twice, named apart: nothing is guessed");
        assert_eq!(named(7), None, "no record");
    }

    /// **The read forward's answer reads back as the supervisor composed it.**
    #[test]
    fn a_read_forward_reads_back() {
        let fields = Fields::none()
            .and(asked::HOW_MANY_EVENTS_FIELD, "1")
            .and(asked::THE_NEXT_SEQUENCE_FIELD, "8")
            .and(asked::THE_OLDEST_RETAINED_FIELD, "3")
            .and(&asked::the_field_of_item(asked::AN_EVENT, 0, asked::THE_SEQUENCE_FIELD), "7")
            .and(&asked::the_field_of_item(asked::AN_EVENT, 0, asked::THE_EVENT_IDENTITY_FIELD), "e")
            .and(&asked::the_field_of_item(asked::AN_EVENT, 0, asked::THE_KIND_FIELD), "k")
            .and(&asked::the_field_of_item(asked::AN_EVENT, 0, asked::THE_SAYS_FIELD), "s")
            .and(&asked::the_field_of_item(asked::AN_EVENT, 0, asked::THE_HAPPENED_AT_FIELD), "100");
        let read = what_the_read_offered(&fields).expect("it reads");
        assert_eq!(read.next_sequence, 8);
        assert_eq!(read.oldest_retained, Some(3));
        assert_eq!(read.events[0].sequence, 7);
        assert_eq!(read.events[0].about, None);
        assert_eq!(read.events[0].happened_at, Some(100));
        assert!(what_the_read_offered(&Fields::none()).is_none(), "not a read is nothing");
    }

    /// **`Rendering` S18a's word is spelt as that crate spells it**, which this
    /// program cannot reach, so the spelling is read off its source.
    #[test]
    fn the_drivers_reason_is_spelt_as_rendering_spells_it() {
        let source = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../Rendering/src/submission.rs"),
        )
        .expect("Rendering's submission is readable");
        assert!(
            source.contains(&format!("pub const THE_DRIVER: &str = \"{THE_DRIVERS_REASON}\";")),
            "the reason for a hold for the driver is spelt differently there"
        );
    }

    /// **`Rendering` K14a's cause and `Rented-server runtime` M13's outcomes,
    /// read off a capture record**: nothing recorded is *undetermined* and
    /// *never established*; a cut attempt is *collection incomplete*.
    #[test]
    fn the_cause_and_the_outcomes_are_read_as_recorded_and_nothing_is_guessed() {
        assert_eq!(the_cause_of(None), TheCause::Undetermined);
        assert_eq!(what_the_stream_came_to(None, AStream::TheComplaints), ACollectionOutcome::NeverEstablished);
        let mut one = OneAttemptsCapture {
            attempt: 1,
            ordinary_sink: String::new(),
            complaints_sink: String::new(),
            available_bytes: 0,
            render_process: None,
            ending: None,
            collection_incomplete_by_reconciliation: true,
            the_termination_was_issued: false,
            the_stops_on_it: Vec::new(),
        };
        assert_eq!(what_the_stream_came_to(Some(&one), AStream::TheOrdinaryOutput), ACollectionOutcome::CollectionIncomplete);
        one.collection_incomplete_by_reconciliation = false;
        one.ending = Some(crate::the_render_worker::TheEndingAsRecorded {
            ending: asked::IT_FAILED.to_string(),
            cause: Some(THE_MACHINES_OWN.to_string()),
            reason: None,
            exit_status: Some(1),
            ordinary: Some(CollectionOutcome::Collected.in_words().to_string()),
            complaints: Some(CollectionOutcome::CollectedAndTheStreamWasEmpty.in_words().to_string()),
            ordinary_retained: None,
            ordinary_discarded: None,
            ordinary_first_loss: None,
            complaints_retained: None,
            complaints_discarded: None,
            complaints_first_loss: None,
            marker_passed: None,
            something_followed_the_marker: None,
            device_line: None,
        });
        assert_eq!(the_cause_of(Some(&one)), TheCause::TheMachinesOwn);
        assert_eq!(what_the_stream_came_to(Some(&one), AStream::TheOrdinaryOutput), ACollectionOutcome::Collected);
        assert_eq!(
            what_the_stream_came_to(Some(&one), AStream::TheComplaints),
            ACollectionOutcome::CollectedAndTheStreamWasEmpty
        );
    }
}

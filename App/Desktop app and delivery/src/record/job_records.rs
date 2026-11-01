//! The local job record in the local record — `Desktop app and delivery` §5,
//! and the first store Smoozy has ever had for one.
//!
//! **The shape is `Queue & History`'s and the store is this one's**, which is
//! the same split `outcome_records` and `key_rows` already stand on. Nothing
//! here decides what a job record holds, what a mark says, or what an ending
//! is: every one of those is that module's, in `App/Queue & History/`.
//!
//! # What this ends
//!
//! **Nothing anywhere in Smoozy wrote a job record before this file**, which is
//! why the Queue and History screens were built and permanently empty. What is
//! here is the table, the writer that puts a provisional record down at
//! submission, the resolution that confirms or removes it, and the fill-in that
//! completes it from the machine's own note.
//!
//! # Three rules that are kept here and are easy to break
//!
//! **No absence, of any length, removes a provisional record.** `State,
//! identity and recovery` V1 step 2, at that document's §7, has three branches
//! and the third — *absent, and the machine cannot say* — leaves the record
//! provisional and asks again next time. `Preparation/Specs/Whole app.md`
//! forbids the determination a timeout would need: Smoozy cannot tell a network
//! problem from a switched-off machine from a machine that no longer exists.
//! **The branch is not decided here at all** — [`the_resolution`] hands the
//! machine's answer to `Queue & History`'s own `what_step_2_does` and performs
//! what comes back, so a timeout has nowhere in this file to be written.
//!
//! **The fill-in never overwrites what `Queue & History` N16 protects.** The
//! submitted moment is the one figure on the entry taken from the desktop's
//! clock; the note's two moments are the machine's. The fill-in is
//! `LocalJobRecord::the_fill_in_from_the_note` and this file writes back the
//! carried half's columns alone — the submitted moment is not among them.
//!
//! **A history tombstone refuses the fill-in unconditionally.** `State,
//! identity and recovery` H4, at that document's §10.1, and it is the whole
//! reason [`TheTombstones`] is read before a note is carried over.
//!
//! # Where a moment goes
//!
//! **The submitted moment is written only where it is this computer's clock**,
//! and the note's two only where they are a machine's, with that machine
//! beside them. A moment that will not read back as the clock it was written on
//! is not returned at all, which is the rule `key_rows` already states for an
//! identifier: **nothing is returned where a column cannot be read back into
//! the shape it was written as.**
//!
//! # How a clause marker is written in this crate
//!
//! **This file keeps no header list**, and writes every other document's name
//! out at its own site. `Queue & History` and this crate's own plan collide on
//! nearly every letter they use — an N, a C, an L, an X, a Q and a P each — and
//! a header list cannot tell two clauses of one name apart. The shape is
//! `src/servers/a_period.rs`'s, taken unchanged. **A bare marker below means a
//! clause of this crate's own plan, `Preparation/Architecture/Desktop app and
//! delivery.md`.**

use smoozy_queue_history::acting::note::{
    AnAttemptMark, TheEndingOnTheNote, TheNote, TheOutcomeAsWritten,
};
use smoozy_queue_history::acting::submission::{AProvisionalRecord, WritesTheProvisionalRecord};
use smoozy_queue_history::record::{
    ADisturbance, AnAttemptWhereTheFileChanged, AnswerOrigin, CarriesOnFrom,
    DependencyCheckAtSubmission, DisturbanceReading, Hue, LocalJobRecord, Standing,
    TheBlenderTheEntryKeeps, TheDeviceAsk, TheServerAsItStood, WhatAnAbsenceDoes,
    WhatBecameOfTheFramesAlreadyFinished, WhatTheMachineSaidAboutTheJob, WhatTheNoteCarried,
};
use smoozy_render_settings::device::DeviceChoice;
use smoozy_render_settings::names::ExactName;
use smoozy_job_files::manifest::CarriedAndNotInterpretedHere;
use smoozy_render_settings::payload::{
    Companion, Payload, TheFilesFrameRange, TheFilesSimplify, TheFilesSwitch, VersionPick,
};
use smoozy_render_settings::reading::DeviceSetting;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, MachineClock, Reading};
use smoozy_state::tombstones::{HistoryTombstone, HistoryTombstones, MachineSideCopy};

use super::{moment, nanoseconds, RecordFailure};
use rusqlite::OptionalExtension;

/// **What record version 25's column holds for one job.**
#[derive(Debug, Clone, PartialEq)]
pub enum TheKeptSetup {
    /// The payload the job was submitted with, read back whole.
    Kept(Payload),
    /// **No setup was kept with this record** — it was written before record
    /// version 25, when nothing kept one, or by something other than a
    /// submission.
    NoneWasKept,
    /// Something stands in the column and will not read back as a payload.
    ItWillNotReadBack,
}

/// `Queue & History` P5a: the descriptive pair, retained on both jobs.
/// This grants no approval authority; the machine's hold supplies that.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstFramePurpose {
    pub check_job: String,
    pub range_job: String,
    pub first_frame: i64,
    pub submitted_range: smoozy_render_settings::frames::FrameForm,
}

/// The keeper, over one open local record.
pub struct JobRecords<'a> {
    connection: &'a rusqlite::Connection,
}

impl<'a> JobRecords<'a> {
    pub fn over(connection: &'a rusqlite::Connection) -> Self {
        JobRecords { connection }
    }
}

/// **What `State, identity and recovery` V1 step 2, at that document's §7, did
/// to one record**, with the store's own two extra answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheResolutionDid {
    /// The three branches, as `Queue & History` decided them.
    TheStepsOwnAnswer(WhatAnAbsenceDoes),
    /// **No record of that job stands here**, which is not one of the three
    /// branches: step 2 runs over the provisional records the store holds, and
    /// a job it does not hold was never one of them.
    NoSuchRecordStands,
}

/// **What the fill-in did**, and the refusal is its own answer rather than a
/// silence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheFillInDid {
    ItCarriedTheNotesHalfOver,
    /// **`State, identity and recovery` H4, at that document's §10.1: a
    /// history tombstone refuses the fill for the job identities it names, and
    /// that refusal is unconditional** — whatever the machine still holds, and
    /// whether or not the machine-side copy was ever removed.
    ATombstoneRefusedIt,
    NoSuchRecordStands,
}

impl JobRecords<'_> {
    /// **The write `State, identity and recovery` V1 step 2, at that
    /// document's §7, asks for: durable, at submission, before the submission
    /// is sent.**
    ///
    /// **The whole record goes down in one transaction** — the row and its four
    /// sets. A row standing without its disturbance marks would be a record
    /// saying no file of the job's was touched, which is the one thing
    /// `Queue & History` N21 exists to prevent.
    ///
    /// **It is an insert and never an update.** A job identity is issued once
    /// and never reused, so a second write under one identity is a fault rather
    /// than a second submission.
    pub fn write_the_provisional_record(
        &self,
        record: &LocalJobRecord,
    ) -> Result<(), RecordFailure> {
        self.write_it_down(record, None)
    }

    /// **The same write, with the kept setup beside the record, in the same
    /// transaction** — record version 25, step 10c piece 6.
    ///
    /// `Queue & History` N10 and `Queue & History` N17 put the kept setup on the local job
    /// record, and `Rendering` A1 and `Render Settings` Y15 say it is the
    /// payload the job was submitted with. **The payload's type is `Render
    /// Settings`' and `Queue & History`'s record has no field for it**, so it is
    /// written here, beside the record, and read by
    /// [`JobRecords::the_kept_setup_of`]. **It goes down in the record's own
    /// transaction**: a record standing without its kept setup would be an
    /// entry that can never be submitted again, and nothing would say why.
    pub fn write_the_provisional_record_and_its_kept_setup(
        &self,
        record: &LocalJobRecord,
        kept_setup: &Payload,
    ) -> Result<(), RecordFailure> {
        self.write_it_down(record, Some(kept_setup))
    }

    /// **The job whose folder a job's files land in — its range job, for a
    /// first-frame check; itself, for every other job** (`Download` P10's
    /// dated note: the check writes into its range job's folder on the machine,
    /// `Job files and outputs` P18a). A record that will not read answers the
    /// job itself. Added 2026-10-31, the twenty-first notes, note 6.
    pub fn the_job_whose_folder_it_lands_in(&self, job: &Identity) -> Identity {
        self.first_frame_purpose(job)
            .ok()
            .flatten()
            .filter(|purpose| purpose.check_job == job.as_text())
            .and_then(|purpose| {
                Identity::read_back(smoozy_state::identities::IdentityKind::Job, &purpose.range_job)
            })
            .unwrap_or_else(|| job.clone())
    }

    /// The saved purpose is absent for legacy jobs and ordinary single frames.
    pub fn first_frame_purpose(&self, job: &Identity) -> Result<Option<FirstFramePurpose>, RecordFailure> {
        let values = self.connection.query_row(
            "SELECT first_frame_purpose, first_frame_pair, first_frame_number, first_frame_range FROM job_record WHERE job = ?1",
            [job.as_text()], |row| Ok((row.get::<_, Option<String>>(0)?, row.get::<_, Option<String>>(1)?, row.get::<_, Option<i64>>(2)?, row.get::<_, Option<String>>(3)?)),
        ).optional()?;
        let Some((Some(role), Some(pair), Some(first_frame), Some(range))) = values else { return Ok(None); };
        let submitted_range = smoozy_render_settings::frames::parse_the_frame_field(&range)
            .map_err(|_| RecordFailure::Engine { reported: "the saved first-frame range will not read back".into() })?;
        let (check_job, range_job) = match role.as_str() {
            "check" => (job.as_text(), pair),
            "range" => (pair, job.as_text()),
            _ => return Err(RecordFailure::Engine { reported: "the saved first-frame purpose will not read back".into() }),
        };
        Ok(Some(FirstFramePurpose {check_job, range_job, first_frame, submitted_range}))
    }

    /// **Record version 51: the frame text the user typed for this job**, as
    /// Render Settings accepted it — `Queue & History` P5b. **For the screen
    /// alone**: nothing reads a number out of it. `None` for a record before the
    /// version, a job whose range was the file's own, a first-frame check, and a
    /// job taken over from a server — each of which draws the numbers form.
    /// **Record version 55: the job's reporting choice as the server holds
    /// it** (`Telegram` V1c) — the `Telegram` V1a value in the manifest's own form, as
    /// JSON text — or nothing where none was kept.
    pub fn the_reporting_choice_of(&self, job: &Identity) -> Result<Option<String>, RecordFailure> {
        let found: Option<Option<String>> = self
            .connection
            .query_row(
                "SELECT reporting_choice FROM job_record WHERE job = ?1",
                [job.as_text()],
                |row| row.get(0),
            )
            .optional()?;
        Ok(found.flatten())
    }

    /// **Record version 55: the choice written** — at submission from the
    /// manifest's field, and after a change from the value the server's
    /// acknowledgement carries, so the entry shows what the server holds.
    pub fn the_reporting_choice_is_written(&self, job: &Identity, choice: &str) -> Result<(), RecordFailure> {
        self.connection.execute(
            "UPDATE job_record SET reporting_choice = ?1 WHERE job = ?2",
            rusqlite::params![choice, job.as_text()],
        )?;
        Ok(())
    }

    pub fn the_typed_frames_of(&self, job: &Identity) -> Result<Option<String>, RecordFailure> {
        let text = self
            .connection
            .query_row(
                "SELECT typed_frames FROM job_record WHERE job = ?1",
                [job.as_text()],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?
            .flatten();
        Ok(text.filter(|text| !text.trim().is_empty()))
    }

    /// **`Queue & History` A6's one durable write: every job of one accepted
    /// offer enters the local record together, or none does** — *"All or
    /// nothing is the offer's own shape"*, and `Queue & History` §16
    /// declares adoption accept atomic entire. Owed piece 10.
    ///
    /// **Each record is written confirmed and under the identity the machine
    /// carries** (A6, `Queue & History` Y4); the caller composes it, and
    /// this writes it as it is. **It is an insert, as the submission's is**:
    /// a record already standing under one of the identities refuses the
    /// whole write, so an offer accepted twice adopts nothing twice. The kept
    /// setup goes beside a record where the machine's manifest gave one.
    pub fn the_adopted_are_written(
        &self,
        adopted: &[(LocalJobRecord, Option<Payload>)],
    ) -> Result<(), RecordFailure> {
        debug_assert!(adopted.iter().all(|(record, _)| record.standing == Standing::Confirmed));
        let records: Vec<(&LocalJobRecord, Option<&Payload>)> =
            adopted.iter().map(|(record, kept)| (record, kept.as_ref())).collect();
        self.write_them_down(&records)
    }

    fn write_it_down(
        &self,
        record: &LocalJobRecord,
        kept_setup: Option<&Payload>,
    ) -> Result<(), RecordFailure> {
        self.write_them_down(&[(record, kept_setup)])
    }

    /// **Every record handed in goes down in one transaction, or none does.**
    /// One record at submission; every record of one accepted offer at
    /// adoption (`Queue & History` A6: *"every offered job enters the local
    /// record in one durable write"*). Owed piece 10.
    fn write_them_down(
        &self,
        records: &[(&LocalJobRecord, Option<&Payload>)],
    ) -> Result<(), RecordFailure> {
        self.write_them_down_with_purpose(records, None, None)
    }

    fn write_them_down_with_purpose(
        &self,
        records: &[(&LocalJobRecord, Option<&Payload>)],
        purpose: Option<&FirstFramePurpose>,
        typed_frames: Option<&str>,
    ) -> Result<(), RecordFailure> {
        self.connection.execute("BEGIN IMMEDIATE", [])?;
        let written = records
            .iter()
            .try_for_each(|(record, kept_setup)| {
                self.one_is_put_down(record, *kept_setup)?;
                // **Record version 51: the frame text the user typed**, in the
                // record's own transaction (`Queue & History` P5b). Nothing is
                // written where nothing was typed.
                if let Some(text) = typed_frames.map(str::trim).filter(|text| !text.is_empty()) {
                    self.connection.execute(
                        "UPDATE job_record SET typed_frames = ?1 WHERE job = ?2",
                        rusqlite::params![text, record.job.as_text()],
                    )?;
                }
                if let Some(purpose) = purpose {
                    let (role, pair) = if record.job.as_text() == purpose.check_job {
                        ("check", &purpose.range_job)
                    } else if record.job.as_text() == purpose.range_job {
                        ("range", &purpose.check_job)
                    } else { return Err(RecordFailure::Engine { reported: "the first-frame pair names another job".into() }); };
                    let range = crate::surface::render_settings::written_out(&purpose.submitted_range);
                    self.connection.execute("UPDATE job_record SET first_frame_purpose = ?1, first_frame_pair = ?2, first_frame_number = ?3, first_frame_range = ?4 WHERE job = ?5",
                        rusqlite::params![role, pair, purpose.first_frame, range, record.job.as_text()])?;
                }
                Ok(())
            });
        match written {
            Ok(()) => {
                self.connection.execute("COMMIT", [])?;
                Ok(())
            }
            Err(failure) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(failure)
            }
        }
    }

    /// **One record and its sets, inside a transaction the caller opened.**
    fn one_is_put_down(
        &self,
        record: &LocalJobRecord,
        kept_setup: Option<&Payload>,
    ) -> Result<(), RecordFailure> {
        let submitted_at = match record.submitted_at.as_ref() {
            None => None,
            Some(reading) => {
                // **`Queue & History` N16: the submitted moment is on the
                // desktop's clock.** A reading on any other clock is refused
                // rather than written, because a machine's moment in this
                // column is a moment the entry would present as the
                // submission's.
                if *reading.clock() != Clock::ThisComputer {
                    return Err(RecordFailure::AMomentTheRecordCannotHold {
                        column: "submitted_at",
                        reported: "the submitted moment is not on this computer's clock".into(),
                    });
                }
                Some(a_moment("submitted_at", reading.taken_at())?)
            }
        };

        let job = record.job.as_text();
        self.connection.execute(
            "INSERT INTO job_record (
                job, server, job_group, standing, render_start_learned,
                server_label, server_hue, submitted_at, label, blend_path,
                scene_name, submitted_with_no_scene_named,
                resolution_percentage_typed, picture_width, picture_height,
                main_output, blender_build_path, blender_version,
                blender_origin, device_ask, device_choice, dependency_check,
                carries_on_from_job, carries_on_from_label,
                note_started, note_started_machine, note_ended,
                note_ended_machine, note_ending_outcome,
                note_ending_exit_status, note_ending_capture_attempt,
                note_attempts, note_given_up_where_it_was,
                note_last_frame_figure, file_values_stood,
                file_values_frame_start, file_values_frame_end,
                file_values_frame_step, file_values_sample_count,
                file_values_resolution_percentage, file_values_engine,
                file_values_device, file_values_use_simplify,
                file_values_subdivision_render,
                file_values_child_particles_render,
                file_values_texture_limit_render,
                the_reason_the_machine_recorded, the_device_line_as_observed
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30,
                ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40,
                ?41, ?42, ?43, ?44, ?45, ?46, ?47, ?48
             )",
            rusqlite::params![
                job,
                record.server.local_server.as_text(),
                record.group.as_ref().map(Identity::as_text),
                standing_as_text(record.standing),
                record.the_render_start_has_been_learned as i64,
                record.server.label,
                record.server.hue.map(|hue| hue.0 as i64),
                submitted_at,
                record.label,
                record
                    .blend_path_on_the_machine
                    .as_ref()
                    .map(|path| path.as_bytes()),
                record.scene_name,
                record.submitted_with_no_scene_named as i64,
                record.resolution_percentage_typed.map(|it| it as i64),
                record
                    .picture_size_from_the_read
                    .map(|(wide, _)| wide as i64),
                record
                    .picture_size_from_the_read
                    .map(|(_, high)| high as i64),
                record.main_output_from_the_read,
                record.blender.pick.build_path,
                record.blender.pick.version,
                origin_as_text(record.blender.origin),
                device_ask_as_text(record.device_asked_for).0,
                device_ask_as_text(record.device_asked_for).1,
                record.dependency_check.map(dependency_check_as_text),
                record
                    .carries_on_from
                    .as_ref()
                    .map(|it| it.earlier_job.as_text()),
                record
                    .carries_on_from
                    .as_ref()
                    .map(|it| it.the_label_as_it_stood.clone()),
                machine_moment(
                    "note_started",
                    record.what_the_note_carried.started.as_ref()
                )?
                .0,
                machine_moment(
                    "note_started",
                    record.what_the_note_carried.started.as_ref()
                )?
                .1,
                machine_moment("note_ended", record.what_the_note_carried.ended.as_ref())?.0,
                machine_moment("note_ended", record.what_the_note_carried.ended.as_ref())?.1,
                record
                    .what_the_note_carried
                    .ending
                    .as_ref()
                    .map(|ending| outcome_as_text(ending.outcome)),
                record
                    .what_the_note_carried
                    .ending
                    .as_ref()
                    .and_then(|ending| ending.exit_status),
                record.what_the_note_carried.ending.as_ref().map(|ending| {
                    ending.the_attempt_whose_capture_record_holds_the_words as i64
                }),
                record.what_the_note_carried.attempts as i64,
                record.what_the_note_carried.given_up_where_it_was,
                record.what_the_note_carried.last_frame_figure,
                record.the_files_values_beside_the_overrides.is_some() as i64,
                file_values(record)
                    .and_then(|it| it.frame_range)
                    .map(|it| it.start),
                file_values(record)
                    .and_then(|it| it.frame_range)
                    .map(|it| it.end),
                file_values(record)
                    .and_then(|it| it.frame_range)
                    .map(|it| it.step),
                file_values(record)
                    .and_then(|it| it.sample_count)
                    .map(|it| it as i64),
                file_values(record)
                    .and_then(|it| it.resolution_percentage)
                    .map(|it| it as i64),
                file_values(record)
                    .and_then(|it| it.engine.as_ref())
                    .map(|name| name.as_text().to_string()),
                file_values(record)
                    .and_then(|it| it.device)
                    .map(device_setting_as_text),
                file_values(record)
                    .and_then(|it| it.simplify.use_simplify)
                    .map(|on| on as i64),
                file_values(record)
                    .and_then(|it| it.simplify.subdivision_render)
                    .map(|it| it as i64),
                file_values(record).and_then(|it| it.simplify.child_particles_render),
                file_values(record).and_then(|it| it.simplify.texture_limit_render.clone()),
                // **Record versions 27 and 28, written with the rest of
                // the note's carried half**, so a record written whole —
                // an adopted job's, whose note already ended — keeps what
                // its ending carried (the paragraph after
                // `Queue & History` N3). At submission both are absent.
                record
                    .what_the_note_carried
                    .ending
                    .as_ref()
                    .and_then(|ending| ending.the_reason_the_machine_recorded.clone())
                    .filter(|reason| !reason.is_empty()),
                record
                    .what_the_note_carried
                    .ending
                    .as_ref()
                    .and_then(|ending| ending.the_device_line_as_observed.clone())
                    .filter(|line| !line.is_empty()),
            ],
        )?;

        for (position, disturbance) in record.disturbances.iter().enumerate() {
            self.connection.execute(
                "INSERT INTO job_record_disturbance (job, position, member, act_intended, reading, how_many)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![
                    job,
                    position as i64,
                    disturbance.the_member,
                    disturbance.the_act_intended,
                    reading_as_text(disturbance.reading),
                    disturbance.how_many,
                ],
            )?;
        }
        for attempt in &record.attempts_where_the_file_changed {
            self.write_one_file_changed(&job, attempt)?;
        }
        for mark in &record.what_the_note_carried.re_run_marks {
            self.write_one_re_run_mark(&job, mark)?;
        }
        if let Some(companion) = record.the_files_values_beside_the_overrides.as_ref() {
            for (position, (layer, switch)) in companion.view_layer_pins.iter().enumerate() {
                let (switch, value) = switch_as_text(*switch);
                self.connection.execute(
                    "INSERT INTO job_record_file_value_pin
                     (job, position, view_layer, switch, switch_value)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    rusqlite::params![job, position as i64, layer.as_text(), switch, value],
                )?;
            }
        }
        // **Record version 39: where the job's output folder is**, written
        // with the record and in the same transaction. Nothing is written
        // for the case every record before the version holds.
        if record.the_job_output_folder
            != smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds::AsEveryJobBeforeItHadIt
        {
            self.connection.execute(
                "UPDATE job_record SET job_output_folder = ?2 WHERE job = ?1",
                rusqlite::params![
                    job,
                    record.the_job_output_folder.as_written(),
                ],
            )?;
        }
        // **Record version 50: `Rented-server runtime` V12's reading, where
        // the ending carried one** — so a record written whole, an adopted
        // job's whose note already ended, keeps it.
        self.the_free_space_at_the_ending_is_kept(&record.job, &record.what_the_note_carried)?;
        if let Some(kept) = kept_setup {
            // **The payload in the form `Render Settings` carries it**, the
            // form the manifest's own settings-payload field holds on the
            // machine (`Job files and outputs` M1).
            self.connection.execute(
                "UPDATE job_record SET kept_setup = ?2 WHERE job = ?1",
                rusqlite::params![job, kept.as_carried().0.to_string()],
            )?;
        }
        Ok(())
    }

    fn write_one_file_changed(
        &self,
        job: &str,
        attempt: &AnAttemptWhereTheFileChanged,
    ) -> Result<(), RecordFailure> {
        self.connection.execute(
            "INSERT OR REPLACE INTO job_record_file_changed
             (job, attempt, a_frame_an_earlier_attempt_wrote_was_kept)
             VALUES (?1, ?2, ?3)",
            rusqlite::params![
                job,
                attempt.attempt as i64,
                matches!(
                    attempt.what_became_of_the_frames_already_finished,
                    WhatBecameOfTheFramesAlreadyFinished::TheyWereReused
                ) as i64,
            ],
        )?;
        Ok(())
    }

    /// **A known frame is never written back to absent.** The machine's note
    /// writes a re-run's mark with the frame absent (the attempt's list is
    /// the render worker's, composed after the start), and every fill-in
    /// carries the note's mark again; the frame the desktop read off the
    /// attempt's own record ([`JobRecords::the_re_runs_frame_is_known`]) stays.
    /// Corrected 2026-10-24, on the second finding of step 12's reboot
    /// scenario: this used to replace the row, frame and all.
    fn write_one_re_run_mark(&self, job: &str, mark: &AnAttemptMark) -> Result<(), RecordFailure> {
        self.connection.execute(
            "INSERT INTO job_record_re_run_mark (job, attempt, proceeded_from)
             VALUES (?1, ?2, ?3)
             ON CONFLICT (job, attempt) DO UPDATE SET
                 proceeded_from = COALESCE(excluded.proceeded_from, job_record_re_run_mark.proceeded_from)",
            rusqlite::params![job, mark.attempt as i64, mark.the_re_run_proceeded_from],
        )?;
        Ok(())
    }

    /// **`Queue & History` N1's re-run marks on one server's records whose
    /// frame is not known yet**, as (job, attempt), in the record's order.
    ///
    /// The frame is the first of the re-run attempt's own list, which the
    /// render worker writes into that attempt's record after the machine has
    /// written the mark; the desktop reads it there
    /// (`crate::servers::the_machines_work`) and lands it with
    /// [`JobRecords::the_re_runs_frame_is_known`].
    pub fn the_re_runs_whose_frame_is_not_known(
        &self,
        server: &Identity,
    ) -> Result<Vec<(Identity, u32)>, RecordFailure> {
        let mut statement = self.connection.prepare(
            "SELECT mark.job, mark.attempt FROM job_record_re_run_mark AS mark
             JOIN job_record AS record ON record.job = mark.job
             WHERE record.server = ?1 AND mark.proceeded_from IS NULL
             ORDER BY mark.job, mark.attempt",
        )?;
        let rows = statement.query_map([server.as_text()], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (job, attempt) = row?;
            if let Some(job) = Identity::read_back(IdentityKind::Job, &job) {
                out.push((job, attempt as u32));
            }
        }
        Ok(out)
    }

    /// **The frame a re-run proceeded from, landed on the job record's own
    /// mark** — the mark Queue and History draw (`Design/Queue & History`,
    /// *The marks*, the fifth). It is written only where the mark stands with
    /// no frame, so a frame once known is never changed. True where it landed.
    pub fn the_re_runs_frame_is_known(
        &self,
        job: &Identity,
        attempt: u32,
        frame: i64,
    ) -> Result<bool, RecordFailure> {
        let changed = self.connection.execute(
            "UPDATE job_record_re_run_mark SET proceeded_from = ?3
             WHERE job = ?1 AND attempt = ?2 AND proceeded_from IS NULL",
            rusqlite::params![job.as_text(), attempt as i64, frame],
        )?;
        Ok(changed > 0)
    }

    /// One record, whole, or nothing where none stands.
    pub fn read(&self, job: &Identity) -> Result<Option<LocalJobRecord>, RecordFailure> {
        let mut found = self.read_where("job = ?1", [job.as_text()])?.records();
        Ok(found.pop())
    }

    /// **Record version 27: the reason the machine recorded for this job's
    /// ending, kept beside the record** — step 10c's sixth pass, fault 3.
    ///
    /// **Written whole and never edited** (`Desktop app and delivery` M9), and
    /// **only onto a record that stands**: a job whose record is gone — deleted,
    /// with its tombstone written — has no row, so nothing is written for it,
    /// which is `State, identity and recovery` H4's refusal holding by the shape
    /// of the statement rather than by a check. **An empty reason is no reason**
    /// and writes nothing.
    ///
    /// **Its caller is the connection's fill-in**, inside that fill-in's own
    /// transaction, since the note's ending carries the reason (step 10c's fix
    /// piece 2; the paragraph after `Queue & History` N3).
    pub fn the_reason_the_machine_recorded_is_kept(
        &self,
        job: &Identity,
        reason: &str,
    ) -> Result<bool, RecordFailure> {
        if reason.is_empty() {
            return Ok(false);
        }
        let written = self.connection.execute(
            "UPDATE job_record SET the_reason_the_machine_recorded = ?2 WHERE job = ?1",
            rusqlite::params![job.as_text(), reason],
        )?;
        Ok(written > 0)
    }

    /// **The reason record version 27 keeps beside one job's record**, or
    /// nothing where none was carried in or no record of that job stands.
    pub fn the_reason_the_machine_recorded(
        &self,
        job: &Identity,
    ) -> Result<Option<String>, RecordFailure> {
        use rusqlite::OptionalExtension;
        let found: Option<Option<String>> = self
            .connection
            .query_row(
                "SELECT the_reason_the_machine_recorded FROM job_record WHERE job = ?1",
                [job.as_text()],
                |row| row.get(0),
            )
            .optional()?;
        Ok(found.flatten().filter(|reason| !reason.is_empty()))
    }

    /// **Record version 28: the device line the machine observed for this
    /// job's ending, kept beside the record** — step 10c's fix piece 2
    /// (`Queue & History` E13, and the paragraph after `Queue & History` N3).
    ///
    /// **Written whole and verbatim, and never read for what it names.**
    /// Whether it says what did the work is `Queue & History` E12's, decided
    /// where the entry is drawn and only on a finished ending. The same two
    /// rules as the reason's: **only onto a record that stands**, and **an
    /// empty line is no line** and writes nothing.
    pub fn the_device_line_as_observed_is_kept(
        &self,
        job: &Identity,
        line: &str,
    ) -> Result<bool, RecordFailure> {
        if line.is_empty() {
            return Ok(false);
        }
        let written = self.connection.execute(
            "UPDATE job_record SET the_device_line_as_observed = ?2 WHERE job = ?1",
            rusqlite::params![job.as_text(), line],
        )?;
        Ok(written > 0)
    }

    /// **`Queue & History` N21's mark, appended onto one job's record** — the
    /// write `File Browser` W8 lands with the telling, reading *may have been
    /// disturbed* until the act's outcome is learned. Step 10c's Files' owed
    /// acts piece 2.
    ///
    /// **Appended after any the job already carries and never replacing one**
    /// (the table's own rule: one job's files may be touched more than once and
    /// by more than one act). **Only onto a record that stands**: a job whose
    /// record is gone has nothing to carry a mark, so nothing is written and
    /// the answer says so.
    ///
    /// **It opens no transaction of its own**, because `File Browser` W8 makes it part of the
    /// telling's one write and the caller holds that transaction.
    pub fn a_mark_is_appended(
        &self,
        job: &Identity,
        mark: &ADisturbance,
    ) -> Result<bool, RecordFailure> {
        Ok(a_mark_is_appended_to(self.connection, job, mark)?)
    }

    /// **`File Browser` W9's finish: the act's outcome is learned and a unit
    /// touching this member took effect**, so the marks this act left reading
    /// *may have been disturbed* now read *disturbed*. A mark already finished,
    /// or left by another act, is not touched.
    pub fn a_mark_is_finished(
        &self,
        job: &Identity,
        member: &str,
        act_intended: &str,
    ) -> Result<usize, RecordFailure> {
        Ok(self.connection.execute(
            "UPDATE job_record_disturbance SET reading = ?4
             WHERE job = ?1 AND member = ?2 AND act_intended = ?3 AND reading = ?5",
            rusqlite::params![
                job.as_text(),
                member,
                act_intended,
                reading_as_text(DisturbanceReading::Disturbed),
                reading_as_text(DisturbanceReading::MayHaveBeenDisturbed),
            ],
        )?)
    }

    /// **A delete inside a job's folder finishes its one mark as what went** —
    /// the build runner's ruling (d) of 2026-10-03, `Design/Queue & History §
    /// The marks`: the mark the delete left on `member` before its first
    /// removal now names `named` — the one file that went, or the folder more
    /// went from with `how_many` — and reads `reading`. Only a mark still
    /// reading *may have been disturbed* is finished, as
    /// [`Self::a_mark_is_finished`] does.
    pub fn a_deletes_mark_is_finished_as(
        &self,
        job: &Identity,
        member: &str,
        named: &str,
        how_many: Option<u32>,
        reading: DisturbanceReading,
    ) -> Result<usize, RecordFailure> {
        Ok(self.connection.execute(
            "UPDATE job_record_disturbance SET member = ?4, how_many = ?5, reading = ?6
             WHERE job = ?1 AND member = ?2 AND act_intended = ?3 AND reading = ?7",
            rusqlite::params![
                job.as_text(),
                member,
                crate::managing_files::A_DELETE,
                named,
                how_many,
                reading_as_text(reading),
                reading_as_text(DisturbanceReading::MayHaveBeenDisturbed),
            ],
        )?)
    }

    /// **`File Browser` W9: no unit touching this member took effect**, so the
    /// mark this act left is removed — a mark pointing at a file that never
    /// moved is the one thing it exists not to say (`Queue & History` N21:
    /// *removed where none of that job's own units took effect*). Only marks
    /// still reading *may have been disturbed* are removed.
    pub fn a_mark_is_removed(
        &self,
        job: &Identity,
        member: &str,
        act_intended: &str,
    ) -> Result<usize, RecordFailure> {
        Ok(self.connection.execute(
            "DELETE FROM job_record_disturbance
             WHERE job = ?1 AND member = ?2 AND act_intended = ?3 AND reading = ?4",
            rusqlite::params![
                job.as_text(),
                member,
                act_intended,
                reading_as_text(DisturbanceReading::MayHaveBeenDisturbed),
            ],
        )?)
    }

    /// **Record version 42: `Queue & History` N23's mark, written onto one
    /// job's record before that job's release is sent** (`Server Setup` R1c).
    /// Held-job piece A, 2026-10-12.
    ///
    /// **One durable write**: the driver's version as handed in, and the job's
    /// Blender version copied from the record itself, in one statement. **The
    /// answer is whether it landed**: false where no record of that job stands
    /// — nothing carries the mark, so the caller sends no release (`Queue & History` N23: *"no
    /// job is released without its mark"*) — and the store's own refusal comes
    /// back as one. **Written whole, and written again the same way** where a
    /// release is decided a second time.
    pub fn the_driver_mark_is_written(
        &self,
        job: &Identity,
        the_drivers_version: Option<&str>,
    ) -> Result<bool, RecordFailure> {
        let written = self.connection.execute(
            "UPDATE job_record SET driver_mark_blender = blender_version,
                                   driver_mark_driver_version = ?2
             WHERE job = ?1",
            rusqlite::params![job.as_text(), the_drivers_version],
        )?;
        Ok(written > 0)
    }

    /// **The kept setup written beside one job's record** — record version
    /// 25's own reader. **Nothing** where no record of that job stands.
    ///
    /// **A column that will not read back as a payload is said to, and no
    /// payload is made up for it** — the rule this file already keeps for every
    /// column: nothing is returned where a value cannot be read back into the
    /// shape it was written as.
    pub fn the_kept_setup_of(&self, job: &Identity) -> Result<Option<TheKeptSetup>, RecordFailure> {
        use rusqlite::OptionalExtension;
        let found: Option<Option<String>> = self
            .connection
            .query_row(
                "SELECT kept_setup FROM job_record WHERE job = ?1",
                [job.as_text()],
                |row| row.get(0),
            )
            .optional()?;
        Ok(found.map(|column| match column {
            None => TheKeptSetup::NoneWasKept,
            Some(text) => match text.parse() {
                Ok(value) => match Payload::read_back(&CarriedAndNotInterpretedHere(value)) {
                    Some(payload) => TheKeptSetup::Kept(payload),
                    None => TheKeptSetup::ItWillNotReadBack,
                },
                Err(_) => TheKeptSetup::ItWillNotReadBack,
            },
        }))
    }

    /// **Every provisional record for one server** — what a connection's
    /// `State, identity and recovery` V1 step 2, at that document's §7, runs
    /// over.
    ///
    /// **A record marked *start never heard* is not among them** (`Queue &
    /// History` C9's second way in, item C1): it is a history record from that
    /// mark on, and its server can no longer be asked.
    pub fn provisional_for(&self, server: &Identity) -> Result<Vec<LocalJobRecord>, RecordFailure> {
        Ok(self
            .read_where(
                "server = ?1 AND standing = 'provisional' AND start_never_heard = 0",
                [server.as_text()],
            )?
            .records())
    }

    /// **Every job record carrying one group identity** — `Desktop app and
    /// delivery` T11: *"have all of this group's jobs reached a recorded
    /// ending"* is answered from the job records this desktop already holds,
    /// with no group record and no count kept beside them. Added 2026-10-03,
    /// the 38's piece 1, as that question's one reader.
    pub fn in_the_group(&self, group: &Identity) -> Result<Vec<LocalJobRecord>, RecordFailure> {
        Ok(self.read_where("job_group = ?1", [group.as_text()])?.records())
    }

    /// **Every job this desktop tracks on one server for which it has learned
    /// no ending** — `Servers` W31's second source, the durable local record
    /// alone, which the Edit dialog's warning is composed from where no queue
    /// reading is held.
    ///
    /// **The ending is `Queue & History`'s own reading of the record**
    /// (`LocalJobRecord::the_ending_learned`), not a second one here. **A
    /// record marked *start never heard* is not among them**: its installation
    /// is gone, it is a history record from that mark on (`Queue & History`
    /// C9's second way in), and nothing on that server is tracked for it any
    /// more. Added 2026-09-19, the Gemini catch-up of step 10a, Area 5 (F-1).
    pub fn with_no_learned_ending_for(
        &self,
        server: &Identity,
    ) -> Result<Vec<LocalJobRecord>, RecordFailure> {
        Ok(self
            .read_where(
                "server = ?1 AND start_never_heard = 0",
                [server.as_text()],
            )?
            .records()
            .into_iter()
            .filter(|record| record.the_ending_learned().is_none())
            .collect())
    }

    /// **Every record that is in history** — `Queue & History` C9: a job is in
    /// history from the moment Smoozy learns it started rendering, and from
    /// that learning the local record is what history draws.
    ///
    /// **And by C9's second way in, which is not a learned start** — item C1,
    /// *Creator-stated, 2026-09-18*: a record whose start was never learned,
    /// whose server is gone, is marked *start never heard* by
    /// [`JobRecords::the_server_is_gone`] and is in history from that mark.
    /// Each entry says which way it came in, because the entry reads its
    /// ending from that.
    ///
    /// **A provisional record is otherwise never among them**, which is
    /// `State, identity and recovery` V1 step 2's own sentence, at that
    /// document's §7: *"It becomes a history entry only when Smoozy learns the
    /// job started rendering, or when its server is gone in one of the two ways
    /// step 1 and the bullet below name."* The learning or the mark is the
    /// test and the standing is not, so a record can be confirmed and in no
    /// list at all.
    ///
    /// **A row in history that will not read back is counted rather than
    /// dropped without a word** — step 10b round 1 of *jobs, history and
    /// downloads*, must-do 3(c) — and History says how many.
    pub fn in_history(&self) -> Result<WhatHistoryHolds, RecordFailure> {
        let found = self.read_where("render_start_learned = 1 OR start_never_heard = 1", [])?;
        Ok(WhatHistoryHolds {
            entries: found
                .read
                .into_iter()
                .map(|(record, marked)| AnEntryHistoryHolds {
                    // **The mark means something only where the start was
                    // never learned**: a start learned afterwards is the
                    // stronger fact and the entry reads its ending from it.
                    start_never_heard: marked && !record.the_render_start_has_been_learned,
                    record,
                })
                .collect(),
            could_not_be_read: found.could_not_be_read,
        })
    }

    /// **`Queue & History` C9's second way in, performed: every record of this
    /// server whose render start was never learned is marked *start never
    /// heard*, and history holds it from then on.**
    ///
    /// Item C1 of step 10b round 1 of *jobs, history and downloads* —
    /// *Creator-stated, 2026-09-18:* *"a job Smoozy submitted and never learned
    /// had started, whose server is then destroyed or removed, is kept and
    /// shown in History, marked that Smoozy never heard whether it started."*
    ///
    /// **Two acts call for it and they are the only two** (`State, identity and
    /// recovery` V1 steps 1 and 2, at that document's §7): the server row's
    /// removal, which performs [`THE_START_WAS_NEVER_HEARD`] inside its own
    /// walk (`src/record/server_rows.rs`), and step 1's absent or different
    /// branch, which calls this. **No absence, of any length, calls it** —
    /// a server that simply does not answer leaves its records exactly as step
    /// 2's third branch leaves them.
    ///
    /// **Nothing is removed and nothing is invented**: the render-start column
    /// stays nought, no started moment is written, and the sets beside
    /// the row stay with it, `Queue & History` N21's disturbance marks among
    /// them.
    pub fn the_server_is_gone(&self, server: &Identity) -> Result<usize, RecordFailure> {
        Ok(self
            .connection
            .execute(THE_START_WAS_NEVER_HEARD, [server.as_text()])?)
    }

    fn read_where<P: rusqlite::Params>(
        &self,
        clause: &str,
        parameters: P,
    ) -> Result<WhatTheReadFound, RecordFailure> {
        let mut statement = self.connection.prepare(&format!(
            "SELECT job, server, job_group, standing, render_start_learned,
                    server_label, server_hue, submitted_at, label, blend_path,
                    scene_name, submitted_with_no_scene_named,
                    resolution_percentage_typed, picture_width, picture_height,
                    main_output, blender_build_path, blender_version,
                    blender_origin, device_ask, device_choice, dependency_check,
                    carries_on_from_job, carries_on_from_label,
                    note_started, note_started_machine, note_ended,
                    note_ended_machine, note_ending_outcome,
                    note_ending_exit_status, note_ending_capture_attempt,
                    note_attempts, note_given_up_where_it_was,
                    note_last_frame_figure, file_values_stood,
                    file_values_frame_start, file_values_frame_end,
                    file_values_frame_step, file_values_sample_count,
                    file_values_resolution_percentage, file_values_engine,
                    file_values_device, file_values_use_simplify,
                    file_values_subdivision_render,
                    file_values_child_particles_render,
                    file_values_texture_limit_render, start_never_heard,
                    the_reason_the_machine_recorded, the_device_line_as_observed,
                    frames_asked, frames_cannot_be_counted, job_output_folder,
                    driver_mark_blender, driver_mark_driver_version
             FROM job_record WHERE {clause}"
        ))?;
        let rows = statement.query_map(parameters, |row| {
            Ok(ARowAsRead {
                job: row.get(0)?,
                server: row.get(1)?,
                group: row.get(2)?,
                standing: row.get(3)?,
                render_start_learned: row.get::<_, i64>(4)? != 0,
                server_label: row.get(5)?,
                server_hue: row.get(6)?,
                submitted_at: row.get(7)?,
                label: row.get(8)?,
                blend_path: row.get(9)?,
                scene_name: row.get(10)?,
                submitted_with_no_scene_named: row.get::<_, i64>(11)? != 0,
                resolution_percentage_typed: row.get(12)?,
                picture_width: row.get(13)?,
                picture_height: row.get(14)?,
                main_output: row.get(15)?,
                blender_build_path: row.get(16)?,
                blender_version: row.get(17)?,
                blender_origin: row.get(18)?,
                device_ask: row.get(19)?,
                device_choice: row.get(20)?,
                dependency_check: row.get(21)?,
                carries_on_from_job: row.get(22)?,
                carries_on_from_label: row.get(23)?,
                note_started: row.get(24)?,
                note_started_machine: row.get(25)?,
                note_ended: row.get(26)?,
                note_ended_machine: row.get(27)?,
                note_ending_outcome: row.get(28)?,
                note_ending_exit_status: row.get(29)?,
                note_ending_capture_attempt: row.get(30)?,
                note_attempts: row.get(31)?,
                note_given_up_where_it_was: row.get(32)?,
                note_last_frame_figure: row.get(33)?,
                file_values_stood: row.get::<_, i64>(34)? != 0,
                frame_start: row.get(35)?,
                frame_end: row.get(36)?,
                frame_step: row.get(37)?,
                sample_count: row.get(38)?,
                resolution_percentage: row.get(39)?,
                engine: row.get(40)?,
                device: row.get(41)?,
                use_simplify: row.get(42)?,
                subdivision_render: row.get(43)?,
                child_particles_render: row.get(44)?,
                texture_limit_render: row.get(45)?,
                start_never_heard: row.get::<_, i64>(46)? != 0,
                the_reason_the_machine_recorded: row.get(47)?,
                the_device_line_as_observed: row.get(48)?,
                frames_asked: row.get(49)?,
                frames_cannot_be_counted: row.get::<_, i64>(50)? != 0,
                job_output_folder: row.get(51)?,
                driver_mark_blender: row.get(52)?,
                driver_mark_driver_version: row.get(53)?,
            })
        })?;

        let mut found = WhatTheReadFound::default();
        for row in rows {
            let row = row?;
            let marked = row.start_never_heard;
            match self.compose(row)? {
                Some(record) => found.read.push((record, marked)),
                // **Counted rather than dropped without a word** — must-do
                // 3(c). The row is still not returned: a record drawn from a
                // guess is the app being wrong on screen about its own state.
                None => found.could_not_be_read += 1,
            }
        }
        Ok(found)
    }

    /// **Nothing is returned where a column cannot be read back into the shape
    /// it was written as**, which is the rule `key_rows` already states.
    ///
    /// **An identity, a standing or an origin that will not read back makes the
    /// whole row unreadable rather than a record with a guess in it.** A record
    /// drawn from a guess is the app being wrong on screen about its own state,
    /// which L8a and `Preparation/Specs/Whole app.md` both close.
    ///
    /// **So does any other word of the record's own that will not read back** —
    /// a disturbance mark, a view-layer pin, the dependency check's state, the
    /// file's device, the ending the note carried (step 10b round 1 of *jobs,
    /// history and downloads*, must-do 3(b)). Each was read with its failure
    /// dropped, so a record lost a mark without a word and was drawn as one
    /// whose files nobody touched — the one thing `Queue & History` N21 exists
    /// to prevent — or with an ending of *not known* in place of the one it
    /// had. **Found by reading every `_from_text` in this file**, not only the
    /// two the audit named.
    fn compose(&self, row: ARowAsRead) -> Result<Option<LocalJobRecord>, RecordFailure> {
        let Some(job) = Identity::read_back(IdentityKind::Job, &row.job) else {
            return Ok(None);
        };
        let Some(server) = Identity::read_back(IdentityKind::LocalServer, &row.server) else {
            return Ok(None);
        };
        let Some(standing) = standing_from_text(&row.standing) else {
            return Ok(None);
        };
        let Some(origin) = origin_from_text(&row.blender_origin) else {
            return Ok(None);
        };
        let Some(device_asked_for) =
            device_ask_from_text(&row.device_ask, row.device_choice.as_deref())
        else {
            return Ok(None);
        };
        let group = match &row.group {
            None => None,
            Some(text) => match Identity::read_back(IdentityKind::Group, text) {
                Some(group) => Some(group),
                None => return Ok(None),
            },
        };
        let dependency_check = match row.dependency_check.as_deref() {
            None => None,
            Some(text) => match dependency_check_from_text(text) {
                Some(state) => Some(state),
                None => return Ok(None),
            },
        };
        let device = match row.device.as_deref() {
            None => None,
            Some(text) => match device_setting_from_text(text) {
                Some(device) => Some(device),
                None => return Ok(None),
            },
        };
        let ending = match row.note_ending_outcome.as_deref() {
            None => None,
            Some(text) => {
                let Some(outcome) = outcome_from_text(text) else {
                    return Ok(None);
                };
                // **An ending with no attempt to find its words under is not
                // an ending this record can hold**, and it is not drawn as
                // *not known* either — the row will not read back.
                let Some(attempt) = row.note_ending_capture_attempt else {
                    return Ok(None);
                };
                // **The two the ending carries beyond `Queue & History` N3 are read back from
                // the two columns kept beside the record** — record versions
                // 27 and 28 — so an ending read off the record is the ending
                // the note carried in, whole (the paragraph after
                // `Queue & History` N3). An empty column is no value.
                Some(TheEndingOnTheNote {
                    outcome,
                    exit_status: row.note_ending_exit_status,
                    the_attempt_whose_capture_record_holds_the_words: attempt as u32,
                    the_reason_the_machine_recorded: row
                        .the_reason_the_machine_recorded
                        .clone()
                        .filter(|reason| !reason.is_empty()),
                    the_device_line_as_observed: row
                        .the_device_line_as_observed
                        .clone()
                        .filter(|line| !line.is_empty()),
                })
            }
        };
        let Some(disturbances) = self.the_disturbances_of(&row.job)? else {
            return Ok(None);
        };
        let Some(view_layer_pins) = self.the_pins_of(&row.job)? else {
            return Ok(None);
        };
        let carries_on_from = match (&row.carries_on_from_job, &row.carries_on_from_label) {
            (Some(job), Some(label)) => match Identity::read_back(IdentityKind::Job, job) {
                Some(earlier_job) => Some(CarriesOnFrom {
                    earlier_job,
                    the_label_as_it_stood: label.clone(),
                }),
                None => return Ok(None),
            },
            _ => None,
        };

        let key = row.job.clone();
        Ok(Some(LocalJobRecord {
            job,
            group,
            server: TheServerAsItStood {
                local_server: server,
                label: row.server_label,
                hue: row.server_hue.map(|hue| Hue(hue as u16)),
            },
            submitted_at: row
                .submitted_at
                .map(|at| Reading::taken((), moment(at), Clock::ThisComputer)),
            label: row.label,
            blend_path_on_the_machine: row
                .blend_path
                .map(|path| String::from_utf8_lossy(&path).into_owned()),
            scene_name: row.scene_name,
            submitted_with_no_scene_named: row.submitted_with_no_scene_named,
            resolution_percentage_typed: row.resolution_percentage_typed.map(|it| it as u32),
            picture_size_from_the_read: match (row.picture_width, row.picture_height) {
                (Some(wide), Some(high)) => Some((wide as u32, high as u32)),
                _ => None,
            },
            main_output_from_the_read: row.main_output,
            blender: TheBlenderTheEntryKeeps {
                pick: VersionPick {
                    build_path: row.blender_build_path,
                    version: row.blender_version,
                },
                origin,
            },
            device_asked_for,
            the_files_values_beside_the_overrides: if row.file_values_stood {
                Some(Companion {
                    frame_range: match (row.frame_start, row.frame_end, row.frame_step) {
                        (Some(start), Some(end), Some(step)) => {
                            Some(TheFilesFrameRange { start, end, step })
                        }
                        _ => None,
                    },
                    sample_count: row.sample_count.map(|it| it as u64),
                    resolution_percentage: row.resolution_percentage.map(|it| it as u32),
                    engine: row
                        .engine
                        .as_deref()
                        .map(ExactName::as_the_reading_produced_it),
                    device,
                    view_layer_pins,
                    simplify: TheFilesSimplify {
                        use_simplify: row.use_simplify.map(|on| on != 0),
                        subdivision_render: row.subdivision_render.map(|it| it as u32),
                        child_particles_render: row.child_particles_render,
                        texture_limit_render: row.texture_limit_render,
                    },
                })
            } else {
                None
            },
            dependency_check,
            carries_on_from,
            disturbances,
            attempts_where_the_file_changed: self.the_file_changed_of(&key)?,
            the_frames: smoozy_queue_history::record::TheFramesTheReadFound {
                asked: row.frames_asked.as_deref().and_then(the_frames_asked_from_text),
                they_cannot_be_counted: row.frames_cannot_be_counted,
            },
            standing,
            the_render_start_has_been_learned: row.render_start_learned,
            what_the_note_carried: WhatTheNoteCarried {
                started: a_machine_reading(row.note_started, row.note_started_machine),
                ended: a_machine_reading(row.note_ended, row.note_ended_machine),
                ending,
                attempts: row.note_attempts as u32,
                re_run_marks: self.the_re_run_marks_of(&key)?,
                given_up_where_it_was: row.note_given_up_where_it_was,
                last_frame_figure: row.note_last_frame_figure,
                the_free_space_at_the_ending: self.the_free_space_at_the_ending_of(&key)?,
            },
            // **A record before version 39 holds nothing here**, and is the
            // job area as every job had it; **a value that will not read back
            // is taken the same way** rather than invented.
            the_job_output_folder: row
                .job_output_folder
                .as_deref()
                .and_then(smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds::read_back)
                .unwrap_or_default(),
            // **Record version 42: `Queue & History` N23's mark, present
            // exactly where its Blender column is.** A record before that
            // version holds nothing here.
            the_driver_was_below_the_bar: row.driver_mark_blender.map(|the_jobs_blender| {
                smoozy_queue_history::record::TheDriverWasBelowTheBar {
                    the_drivers_version: row.driver_mark_driver_version,
                    the_jobs_blender,
                }
            }),
        }))
    }

    /// **`None` where a mark's word will not read back**, and the caller then
    /// takes the whole row as unreadable (must-do 3(b)).
    fn the_disturbances_of(&self, job: &str) -> Result<Option<Vec<ADisturbance>>, RecordFailure> {
        let mut statement = self.connection.prepare(
            "SELECT member, act_intended, reading, how_many FROM job_record_disturbance
             WHERE job = ?1 ORDER BY position",
        )?;
        let rows = statement.query_map([job], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<u32>>(3)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (the_member, the_act_intended, reading, how_many) = row?;
            let Some(reading) = reading_from_text(&reading) else {
                return Ok(None);
            };
            out.push(ADisturbance {
                the_member,
                the_act_intended,
                reading,
                how_many,
            });
        }
        Ok(Some(out))
    }

    fn the_file_changed_of(
        &self,
        job: &str,
    ) -> Result<Vec<AnAttemptWhereTheFileChanged>, RecordFailure> {
        let mut statement = self.connection.prepare(
            "SELECT attempt, a_frame_an_earlier_attempt_wrote_was_kept
             FROM job_record_file_changed WHERE job = ?1 ORDER BY attempt",
        )?;
        let rows = statement.query_map([job], |row| {
            Ok(AnAttemptWhereTheFileChanged {
                attempt: row.get::<_, i64>(0)? as u32,
                what_became_of_the_frames_already_finished: if row.get::<_, i64>(1)? != 0 {
                    WhatBecameOfTheFramesAlreadyFinished::TheyWereReused
                } else {
                    WhatBecameOfTheFramesAlreadyFinished::TheyWereRenderedAgain
                },
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// **Record version 50: `Rented-server runtime` V12's reading, written
    /// where the carried half holds one and never over one already kept** —
    /// the reading is immutable with the ending it rides (`State, identity and
    /// recovery` §8.6). Its moment is a machine's and is refused on any other
    /// clock, as the note's other two are.
    fn the_free_space_at_the_ending_is_kept(
        &self,
        job: &Identity,
        carried: &WhatTheNoteCarried,
    ) -> Result<(), RecordFailure> {
        let Some(read) = carried.the_free_space_at_the_ending.as_ref() else {
            return Ok(());
        };
        let (at, machine) = machine_moment("note_free_space_taken_at", Some(&read.taken_at))?;
        let available = i64::try_from(read.available_bytes).map_err(|_| {
            RecordFailure::AMomentTheRecordCannotHold {
                column: "note_free_space_available",
                reported: "the figure is larger than this record's own spelling holds".into(),
            }
        })?;
        self.connection.execute(
            "UPDATE job_record SET
                note_free_space_available = ?2, note_free_space_folder = ?3,
                note_free_space_taken_at = ?4, note_free_space_machine = ?5
             WHERE job = ?1 AND note_free_space_available IS NULL",
            rusqlite::params![
                job.as_text(),
                available,
                read.the_folder_it_was_taken_against.as_bytes(),
                at,
                machine,
            ],
        )?;
        Ok(())
    }

    /// **Record version 50's reading, read back whole or not at all**: a
    /// figure with no folder or no moment is not a reading, and reads as none.
    fn the_free_space_at_the_ending_of(
        &self,
        job: &str,
    ) -> Result<Option<smoozy_queue_history::acting::note::TheFreeSpaceReadAtTheEnding>, RecordFailure> {
        use rusqlite::OptionalExtension;
        let found: Option<(Option<i64>, Option<Vec<u8>>, Option<i64>, Option<String>)> = self
            .connection
            .query_row(
                "SELECT note_free_space_available, note_free_space_folder,
                        note_free_space_taken_at, note_free_space_machine
                 FROM job_record WHERE job = ?1",
                [job],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .optional()?;
        let Some((Some(available), Some(folder), at, machine)) = found else {
            return Ok(None);
        };
        Ok((|| {
            Some(smoozy_queue_history::acting::note::TheFreeSpaceReadAtTheEnding {
                available_bytes: u64::try_from(available).ok()?,
                the_folder_it_was_taken_against: String::from_utf8(folder).ok()?,
                taken_at: a_machine_reading(at, machine)?,
            })
        })())
    }

    fn the_re_run_marks_of(&self, job: &str) -> Result<Vec<AnAttemptMark>, RecordFailure> {
        let mut statement = self.connection.prepare(
            "SELECT attempt, proceeded_from FROM job_record_re_run_mark
             WHERE job = ?1 ORDER BY attempt",
        )?;
        let rows = statement.query_map([job], |row| {
            Ok(AnAttemptMark {
                attempt: row.get::<_, i64>(0)? as u32,
                the_re_run_proceeded_from: row.get(1)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// **`None` where a pin's word will not read back** (must-do 3(b)).
    fn the_pins_of(
        &self,
        job: &str,
    ) -> Result<Option<Vec<(ExactName, TheFilesSwitch)>>, RecordFailure> {
        let mut statement = self.connection.prepare(
            "SELECT view_layer, switch, switch_value FROM job_record_file_value_pin
             WHERE job = ?1 ORDER BY position",
        )?;
        let rows = statement.query_map([job], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<i64>>(2)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (layer, switch, value) = row?;
            let Some(switch) = switch_from_text(&switch, value) else {
                return Ok(None);
            };
            out.push((ExactName::as_the_reading_produced_it(&layer), switch));
        }
        Ok(Some(out))
    }

    /// **`State, identity and recovery` V1 step 2, at that document's §7,
    /// performed against the store.**
    ///
    /// **The branch is `Queue & History`'s and is not decided here.** This hands
    /// the machine's answer to that crate's `what_step_2_does` and performs what
    /// comes back — so the third branch's *no absence, of any length, removes a
    /// provisional record* is kept where it is stated, and there is no clock, no
    /// count and no age anywhere in this function to build a timeout out of.
    pub fn the_resolution(
        &self,
        job: &Identity,
        said: WhatTheMachineSaidAboutTheJob,
    ) -> Result<WhatTheResolutionDid, RecordFailure> {
        let stands: bool = self.connection.query_row(
            "SELECT count(*) > 0 FROM job_record WHERE job = ?1 AND standing = 'provisional'",
            [job.as_text()],
            |row| row.get(0),
        )?;
        if !stands {
            return Ok(WhatTheResolutionDid::NoSuchRecordStands);
        }
        let did = smoozy_queue_history::record::what_step_2_does(said);
        match did {
            WhatAnAbsenceDoes::TheRecordBecomesConfirmed => {
                self.connection.execute(
                    "UPDATE job_record SET standing = 'confirmed' WHERE job = ?1",
                    [job.as_text()],
                )?;
            }
            WhatAnAbsenceDoes::TheRecordIsRemovedLeavingNoTrace => {
                self.remove_a_never_taken_submission(job)?;
            }
            // **Nothing is written at all.** The record stays exactly as it is
            // and the question is asked again on the next connection.
            WhatAnAbsenceDoes::TheRecordStaysProvisionalAndIsAskedAgainNextTime => {}
        }
        Ok(WhatTheResolutionDid::TheStepsOwnAnswer(did))
    }

    /// **`State, identity and recovery` V1 step 4, at that document's §7,
    /// performed: fill in what the local record does not hold, from the
    /// machine's own note.**
    ///
    /// **The tombstone is asked first and its refusal is unconditional**
    /// (`State, identity and recovery` H4, at that document's §10.1) — whatever
    /// the machine still holds, and whether or not the machine-side copy was
    /// ever removed.
    ///
    /// **What is carried over is `Queue & History`'s own decision**, in
    /// `LocalJobRecord::the_fill_in_from_the_note`; this writes back the carried
    /// half's columns and the re-run marks, and touches no other column. **The
    /// submitted moment is not among them**, which is `Queue & History` N16
    /// kept by the shape of
    /// the statement rather than by care.
    pub fn the_fill_in(
        &self,
        job: &Identity,
        note: &TheNote,
    ) -> Result<WhatTheFillInDid, RecordFailure> {
        if !self.tombstones()?.may_be_filled_in(job) {
            return Ok(WhatTheFillInDid::ATombstoneRefusedIt);
        }
        let Some(mut record) = self.read(job)? else {
            return Ok(WhatTheFillInDid::NoSuchRecordStands);
        };
        let the_ending_was_learned_before = record.the_ending_learned().is_some();
        record.the_fill_in_from_the_note(note);
        // **`Queue & History` E9: the connection that learns the ending owes
        // the words**, and the debt is written in the act that writes the
        // ending, so a quit between the two cannot lose it. Record version 34,
        // the Creator's notes on 0.1.8, piece 6d.
        let the_words_are_owed_now =
            !the_ending_was_learned_before && record.the_ending_learned().is_some();

        let carried = &record.what_the_note_carried;
        let started = machine_moment("note_started", carried.started.as_ref())?;
        let ended = machine_moment("note_ended", carried.ended.as_ref())?;
        self.connection.execute("BEGIN IMMEDIATE", [])?;
        let written = (|| -> Result<(), RecordFailure> {
            self.connection.execute(
                "UPDATE job_record SET
                    render_start_learned = ?2,
                    note_started = ?3, note_started_machine = ?4,
                    note_ended = ?5, note_ended_machine = ?6,
                    note_ending_outcome = ?7, note_ending_exit_status = ?8,
                    note_ending_capture_attempt = ?9, note_attempts = ?10,
                    note_given_up_where_it_was = ?11, note_last_frame_figure = ?12
                 WHERE job = ?1",
                rusqlite::params![
                    job.as_text(),
                    record.the_render_start_has_been_learned as i64,
                    started.0,
                    started.1,
                    ended.0,
                    ended.1,
                    carried
                        .ending
                        .as_ref()
                        .map(|it| outcome_as_text(it.outcome)),
                    carried.ending.as_ref().and_then(|it| it.exit_status),
                    carried
                        .ending
                        .as_ref()
                        .map(|it| it.the_attempt_whose_capture_record_holds_the_words as i64),
                    carried.attempts as i64,
                    carried.given_up_where_it_was,
                    carried.last_frame_figure,
                ],
            )?;
            for mark in &carried.re_run_marks {
                self.write_one_re_run_mark(&job.as_text(), mark)?;
            }
            if the_words_are_owed_now {
                self.connection.execute(
                    "UPDATE job_record SET words_owed = 1 WHERE job = ?1",
                    [job.as_text()],
                )?;
            }
            // **the paragraph after `Queue & History` N3: the two the
            // note's ending carries beyond `Queue & History` N3 are kept beside the record, in
            // this same transaction** — the reason (record version 27) and the
            // device line as observed (record version 28), each whole and
            // verbatim. **Neither is read here**: the device line is kept on
            // whatever ending it came with, and `Queue & History` E12's
            // judgement of it is the entry's.
            // **Record version 50: `Rented-server runtime` V12's reading rides
            // the ending into this same transaction.**
            self.the_free_space_at_the_ending_is_kept(job, carried)?;
            if let Some(ending) = carried.ending.as_ref() {
                if let Some(reason) = ending.the_reason_the_machine_recorded.as_deref() {
                    self.the_reason_the_machine_recorded_is_kept(job, reason)?;
                }
                if let Some(line) = ending.the_device_line_as_observed.as_deref() {
                    self.the_device_line_as_observed_is_kept(job, line)?;
                }
            }
            Ok(())
        })();
        match written {
            Ok(()) => {
                self.connection.execute("COMMIT", [])?;
                Ok(WhatTheFillInDid::ItCarriedTheNotesHalfOver)
            }
            Err(failure) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(failure)
            }
        }
    }

    /// **`Queue & History` N9's recorded reading, landed by the same step 4.**
    ///
    /// **The merge rule is that module's and is not applied twice**: this reads
    /// the record, hands it to `LocalJobRecord::the_read_fills_what_it_lacks`,
    /// and writes back the fields that call can move — four, and the frames'
    /// two since record version 35 (the 38's piece 10).
    ///
    /// **Called by `crate::servers::the_machines_work`'s reading fill-in**, on
    /// `Queue & History` N9's two routes, since the 38's piece 10; until then
    /// only tests called it.
    pub fn the_read_is_filled_in(
        &self,
        job: &Identity,
        read: &smoozy_queue_history::record::WhatTheReadProduced,
    ) -> Result<WhatTheFillInDid, RecordFailure> {
        if !self.tombstones()?.may_be_filled_in(job) {
            return Ok(WhatTheFillInDid::ATombstoneRefusedIt);
        }
        let Some(mut record) = self.read(job)? else {
            return Ok(WhatTheFillInDid::NoSuchRecordStands);
        };
        record.the_read_fills_what_it_lacks(read);
        self.connection.execute("BEGIN IMMEDIATE", [])?;
        let written = (|| -> Result<(), RecordFailure> {
            self.connection.execute(
                "UPDATE job_record SET scene_name = ?2, picture_width = ?3,
                    picture_height = ?4, main_output = ?5, frames_asked = ?6,
                    frames_cannot_be_counted = ?7,
                    job_output_folder = CASE WHEN ?8 IS NULL THEN job_output_folder ELSE ?8 END
                 WHERE job = ?1",
                rusqlite::params![
                    job.as_text(),
                    record.scene_name,
                    record
                        .picture_size_from_the_read
                        .map(|(wide, _)| wide as i64),
                    record
                        .picture_size_from_the_read
                        .map(|(_, high)| high as i64),
                    record.main_output_from_the_read,
                    record.the_frames.asked.as_ref().map(the_frames_asked_as_text),
                    record.the_frames.they_cannot_be_counted as i64,
                    // **Record version 39**: only a folder learned now is
                    // written; the case every older record holds writes
                    // nothing over it.
                    match &record.the_job_output_folder {
                        smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds::AsEveryJobBeforeItHadIt => None,
                        held => Some(held.as_written()),
                    },
                ],
            )?;
            for attempt in &record.attempts_where_the_file_changed {
                self.write_one_file_changed(&job.as_text(), attempt)?;
            }
            Ok(())
        })();
        match written {
            Ok(()) => {
                self.connection.execute("COMMIT", [])?;
                Ok(WhatTheFillInDid::ItCarriedTheNotesHalfOver)
            }
            Err(failure) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(failure)
            }
        }
    }

    /// Keep the exact material debt and remove the never-taken job together.
    /// A failed write leaves the provisional job available for another check.
    pub fn remove_a_never_taken_submission(&self, job: &Identity) -> Result<(), RecordFailure> {
        self.connection
            .execute("SAVEPOINT never_taken_submission", [])?;
        let removed = (|| {
            // Only this definitive branch makes a candidate into a debt.
            // Older, hand-made test records may predate the material table.
            let has_material_table: bool = self.connection.query_row(
                "SELECT count(*) > 0 FROM sqlite_schema WHERE name = 'submission_material'",
                [],
                |row| row.get(0),
            )?;
            if has_material_table {
                self.connection.execute("UPDATE submission_material SET debt = 1 WHERE job = ?1
                    AND EXISTS (SELECT 1 FROM job_record WHERE job = ?1 AND standing = 'provisional')",
                    [job.as_text()])?;
            }
            self.remove(job)
        })();
        match removed {
            Ok(()) => {
                self.connection
                    .execute("RELEASE never_taken_submission", [])?;
                Ok(())
            }
            Err(failure) => {
                let _ = self.connection.execute_batch(
                    "ROLLBACK TO never_taken_submission; RELEASE never_taken_submission",
                );
                Err(failure)
            }
        }
    }

    /// **The record and every set beside it, together** — six since record version 37.
    ///
    /// **This is the removal and never a deletion of history**: `Queue &
    /// History` X1's deletion is the record *and* a tombstone in one act, and
    /// the tombstone is the caller's half.
    ///
    /// **`Download` R1's arrangement goes with it, in the same transaction.**
    /// `Download` R3: *the arrangement's lifetime is user-held, riding the
    /// job's entry* — removed when the user removes the entry, and *a cancelled
    /// job's provisional record removal takes the arrangement with it*. Since
    /// 2026-09-27 the arrangement is written with the provisional record,
    /// before the send (`src/submitting.rs`), so every removal here — a
    /// submission that never reached the machine, `State, identity and
    /// recovery` V1 step 2's *removed leaving no trace*, and `Queue & History`
    /// X1's deletion — would otherwise leave one held against a job that is
    /// gone, and `Download` R12 would resume it on every later connection.
    pub fn remove(&self, job: &Identity) -> Result<(), RecordFailure> {
        let text = job.as_text();
        // **The row and its sets go together or not at all**, and this is
        // also called from inside a transaction somebody else opened —
        // `State, identity and recovery` V1 step 2's removal, at that
        // document's §7. The engine has no nested transaction, so one is opened
        // here only where none is standing.
        let it_opens_its_own = self.connection.is_autocommit();
        // **`Queue & History` P4: the fetched copies are subject-held on the
        // record, so the act that removes the record removes them** (`State,
        // identity and recovery` R18) — read before the rows go, taken off
        // this computer once they have. The 38's piece 3.
        let the_copies = super::previews::the_copies_held_for(self.connection, job).unwrap_or_default();
        if it_opens_its_own {
            self.connection.execute("BEGIN IMMEDIATE", [])?;
        }
        let removed = (|| -> Result<(), RecordFailure> {
            for table in EVERY_SET_BESIDE_THE_ROW
                .iter()
                .chain(super::download_records::THE_ARRANGEMENTS_TABLES.iter())
            {
                self.connection
                    .execute(&format!("DELETE FROM {table} WHERE job = ?1"), [&text])?;
            }
            self.connection
                .execute("DELETE FROM job_record WHERE job = ?1", [&text])?;
            Ok(())
        })();
        match (removed, it_opens_its_own) {
            (Ok(()), true) => {
                self.connection.execute("COMMIT", [])?;
                super::previews::the_copies_go(&the_copies);
                Ok(())
            }
            (Ok(()), false) => {
                super::previews::the_copies_go(&the_copies);
                Ok(())
            }
            (Err(failure), true) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(failure)
            }
            (Err(failure), false) => Err(failure),
        }
    }

    /// The tombstones this record holds, read whole.
    ///
    /// **They are few and one identity each** (`State, identity and recovery`
    /// H3, at that document's §10.1), and the trait they answer hands back a
    /// reference, so they are read rather than queried one at a time.
    pub fn tombstones(&self) -> Result<TheTombstones, RecordFailure> {
        let mut statement = self
            .connection
            .prepare("SELECT job, deleted_at, machine_side_copy FROM history_tombstone")?;
        let rows = statement.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        let mut held = Vec::new();
        for row in rows {
            let (job, deleted_at, copy) = row?;
            let Some(job) = Identity::read_back(IdentityKind::Job, &job) else {
                continue;
            };
            held.push(HistoryTombstone {
                job,
                deleted_at: Reading::taken((), moment(deleted_at), Clock::ThisComputer),
                machine_side_copy: if copy == "removed" {
                    MachineSideCopy::Removed
                } else {
                    MachineSideCopy::StillOwed
                },
            });
        }
        Ok(TheTombstones { held })
    }
}

/// **Every set beside the row**, named once so a removal and the schema cannot
/// drift apart.
pub const EVERY_SET_BESIDE_THE_ROW: [&str; 6] = [
    "job_record_disturbance",
    "job_record_file_changed",
    "job_record_re_run_mark",
    "job_record_file_value_pin",
    // **`Queue & History` E9's kept words**, record version 34 — they live
    // as long as the entry and go with it (`Queue & History` X1).
    "job_record_words",
    // **`Queue & History` P4's fetched derivatives**, record version 37 —
    // subject-held on the job record (`State, identity and recovery` R23).
    // The 38's piece 3.
    super::previews::TABLE,
];

/// **`Queue & History` C9's second way in, as the one statement that writes
/// it** — item C1. The removal walk in `src/record/server_rows.rs` runs it
/// inside its own transaction and [`JobRecords::the_server_is_gone`] runs it
/// for §7 step 1's branch, so the two cannot mark different records.
pub const THE_START_WAS_NEVER_HEARD: &str =
    "UPDATE job_record SET start_never_heard = 1 WHERE server = ?1 AND render_start_learned = 0";

/// **What History reads from this store** — the records it holds, and how
/// many rows it holds that would not read back.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WhatHistoryHolds {
    pub entries: Vec<AnEntryHistoryHolds>,
    /// **Rows in history that would not read back into the shape they were
    /// written as** (must-do 3(c)). They are not drawn, and History says how
    /// many there are.
    pub could_not_be_read: u32,
}

/// One record history holds, and which of `Queue & History` C9's two ways it
/// came in by.
#[derive(Debug, Clone, PartialEq)]
pub struct AnEntryHistoryHolds {
    pub record: LocalJobRecord,
    /// **Item C1: Smoozy never heard whether this job started**, and its
    /// server is gone. False wherever the start was learned.
    pub start_never_heard: bool,
}

/// What one read of the table found, before anybody chose what to keep.
#[derive(Default)]
struct WhatTheReadFound {
    read: Vec<(LocalJobRecord, bool)>,
    could_not_be_read: u32,
}

impl WhatTheReadFound {
    fn records(self) -> Vec<LocalJobRecord> {
        self.read.into_iter().map(|(record, _)| record).collect()
    }
}

/// The tombstones, as `State, identity and recovery` §10.1's own trait wants
/// them.
pub struct TheTombstones {
    held: Vec<HistoryTombstone>,
}

impl HistoryTombstones for TheTombstones {
    fn tombstone_for(&self, job: &Identity) -> Option<&HistoryTombstone> {
        self.held.iter().find(|tombstone| &tombstone.job == job)
    }
}

/// **`Queue & History`'s own submission seam, answered by this store.**
///
/// **It carries the whole record and the trait carries four fields of it**, and
/// the difference is the reason this type exists. `submit` hands its writer an
/// `AProvisionalRecord` — the job, the server, the group and the label — while
/// the record `Queue & History` N10 to `Queue & History` N22 describes is
/// written at that same
/// moment and holds two dozen more facts, every one of them composed by the
/// page that submitted. **So the caller hands the whole record over here and
/// the trait's four fields are checked against it**, rather than a record being
/// written with everything but those four missing.
///
/// **And it carries the kept setup**, written beside the record in the same
/// transaction (record version 25, step 10c piece 6) — the submission is the
/// one writer that holds it, so a writer handed no kept setup would be the gap
/// step 10b piece 6a handed up, arriving by another road.
pub struct TheSubmissionsWriter<'a> {
    records: JobRecords<'a>,
    the_whole_record: LocalJobRecord,
    the_kept_setup: Payload,
    first_frame_purpose: Option<FirstFramePurpose>,
    /// **`Queue & History` P5b: the frame text the user typed**, written beside
    /// the kept setup in the same transaction (record version 51). Never in
    /// the payload: the kept setup is the payload, and it carries numbers only.
    typed_frames: Option<String>,
}

impl<'a> TheSubmissionsWriter<'a> {
    pub fn of(
        connection: &'a rusqlite::Connection,
        the_whole_record: LocalJobRecord,
        the_kept_setup: Payload,
    ) -> Self {
        TheSubmissionsWriter {
            records: JobRecords::over(connection),
            the_whole_record,
            the_kept_setup,
            first_frame_purpose: None,
            typed_frames: None,
        }
    }

    pub fn with_first_frame_purpose(mut self, purpose: Option<FirstFramePurpose>) -> Self {
        self.first_frame_purpose = purpose;
        self
    }

    /// **`Queue & History` P5b: the accepted frame text goes down with the
    /// record**, or nothing where the range was not typed.
    pub fn with_the_typed_frames(mut self, typed: Option<String>) -> Self {
        self.typed_frames = typed;
        self
    }
}

impl WritesTheProvisionalRecord for TheSubmissionsWriter<'_> {
    fn write_it(&mut self, record: &AProvisionalRecord) -> Result<(), String> {
        // **All four of the trait's fields**, the label with the other three
        // (must-do 5): a record written under a label the submission did not
        // carry is an entry named for a different job.
        if record.job != self.the_whole_record.job
            || record.server != self.the_whole_record.server.local_server
            || record.group != self.the_whole_record.group
            // **The trait carries the label as typed and the record keeps it
            // as an option**; a record with none is drawn with an empty name,
            // so none and an empty label are the one label.
            || record.label.as_str() != self.the_whole_record.label.as_deref().unwrap_or("")
        {
            return Err(
                "the submission names a job this writer was not given the record for".to_string(),
            );
        }
        self.records
            .write_them_down_with_purpose(
                &[(&self.the_whole_record, Some(&self.the_kept_setup))],
                self.first_frame_purpose.as_ref(),
                self.typed_frames.as_deref(),
            )
            .map_err(|failure| format!("{failure:?}"))
    }

    fn remove_it(&mut self, job: &Identity) -> Result<(), String> {
        self.records
            .remove_a_never_taken_submission(job)
            .map_err(|failure| format!("{failure:?}"))
    }
}

// ───────────────────────────────────────────────────────────────────────────
// How each closed set is written down, and read back
// ───────────────────────────────────────────────────────────────────────────

fn standing_as_text(standing: Standing) -> &'static str {
    match standing {
        Standing::Provisional => "provisional",
        Standing::Confirmed => "confirmed",
    }
}

fn standing_from_text(text: &str) -> Option<Standing> {
    match text {
        "provisional" => Some(Standing::Provisional),
        "confirmed" => Some(Standing::Confirmed),
        _ => None,
    }
}

fn origin_as_text(origin: AnswerOrigin) -> &'static str {
    match origin {
        AnswerOrigin::TheUsersOwnOverride => "the user's own override",
        AnswerOrigin::ResolvedForThem => "resolved for them",
    }
}

fn origin_from_text(text: &str) -> Option<AnswerOrigin> {
    match text {
        "the user's own override" => Some(AnswerOrigin::TheUsersOwnOverride),
        "resolved for them" => Some(AnswerOrigin::ResolvedForThem),
        _ => None,
    }
}

/// **`Queue & History` E12a's third value is a state rather than a default**,
/// so it is written as a word of its own and never as a missing choice.
fn device_ask_as_text(ask: TheDeviceAsk) -> (&'static str, Option<&'static str>) {
    match ask {
        TheDeviceAsk::TheUsersOwn(choice) => {
            ("the user's own", Some(device_choice_as_text(choice)))
        }
        TheDeviceAsk::TheFilesOwnAsTheReadingInForceGaveIt(choice) => (
            "the file's own as the reading in force gave it",
            Some(device_choice_as_text(choice)),
        ),
        TheDeviceAsk::TheFilesOwnUnread => ("the file's own, unread", None),
    }
}

fn device_ask_from_text(ask: &str, choice: Option<&str>) -> Option<TheDeviceAsk> {
    match ask {
        "the user's own" => Some(TheDeviceAsk::TheUsersOwn(device_choice_from_text(choice?)?)),
        "the file's own as the reading in force gave it" => Some(
            TheDeviceAsk::TheFilesOwnAsTheReadingInForceGaveIt(device_choice_from_text(choice?)?),
        ),
        "the file's own, unread" => Some(TheDeviceAsk::TheFilesOwnUnread),
        _ => None,
    }
}

fn device_choice_as_text(choice: DeviceChoice) -> &'static str {
    match choice {
        DeviceChoice::TheCards => "the cards",
        DeviceChoice::TheProcessor => "the processor",
    }
}

fn device_choice_from_text(text: &str) -> Option<DeviceChoice> {
    match text {
        "the cards" => Some(DeviceChoice::TheCards),
        "the processor" => Some(DeviceChoice::TheProcessor),
        _ => None,
    }
}

fn device_setting_as_text(setting: DeviceSetting) -> &'static str {
    match setting {
        DeviceSetting::TheCards => "the cards",
        DeviceSetting::TheProcessor => "the processor",
    }
}

fn device_setting_from_text(text: &str) -> Option<DeviceSetting> {
    match text {
        "the cards" => Some(DeviceSetting::TheCards),
        "the processor" => Some(DeviceSetting::TheProcessor),
        _ => None,
    }
}

/// **`Queue & History` N15's first mark is one field wide enough for the
/// state and never a boolean**, so each of the four is a word of its own.
fn dependency_check_as_text(state: DependencyCheckAtSubmission) -> &'static str {
    match state {
        DependencyCheckAtSubmission::NoStandingAtAll => "no standing at all",
        DependencyCheckAtSubmission::AStandingCarryingTheReplacedSinceCheckOrDisturbedMark => {
            "a standing carrying the replaced-since-check or disturbed mark"
        }
        DependencyCheckAtSubmission::AStandingKeyedToAnotherBlenderVersion => {
            "a standing keyed to another Blender version"
        }
        DependencyCheckAtSubmission::ACompleteStandingStood => "a complete standing stood",
    }
}

fn dependency_check_from_text(text: &str) -> Option<DependencyCheckAtSubmission> {
    match text {
        "no standing at all" => Some(DependencyCheckAtSubmission::NoStandingAtAll),
        "a standing carrying the replaced-since-check or disturbed mark" => {
            Some(DependencyCheckAtSubmission::AStandingCarryingTheReplacedSinceCheckOrDisturbedMark)
        }
        "a standing keyed to another Blender version" => {
            Some(DependencyCheckAtSubmission::AStandingKeyedToAnotherBlenderVersion)
        }
        "a complete standing stood" => Some(DependencyCheckAtSubmission::ACompleteStandingStood),
        _ => None,
    }
}

/// [`JobRecords::a_mark_is_appended`]'s one statement, for a caller already
/// inside a transaction of its own that answers in the engine's own error.
pub(crate) fn a_mark_is_appended_to(
    connection: &rusqlite::Connection,
    job: &Identity,
    mark: &ADisturbance,
) -> rusqlite::Result<bool> {
    let written = connection.execute(
        "INSERT INTO job_record_disturbance (job, position, member, act_intended, reading, how_many)
         SELECT ?1,
                (SELECT COALESCE(MAX(position) + 1, 0) FROM job_record_disturbance WHERE job = ?1),
                ?2, ?3, ?4, ?5
         WHERE EXISTS (SELECT 1 FROM job_record WHERE job = ?1)",
        rusqlite::params![
            job.as_text(),
            mark.the_member,
            mark.the_act_intended,
            reading_as_text(mark.reading),
            mark.how_many,
        ],
    )?;
    Ok(written > 0)
}

fn reading_as_text(reading: DisturbanceReading) -> &'static str {
    match reading {
        DisturbanceReading::Disturbed => "disturbed",
        DisturbanceReading::MayHaveBeenDisturbed => "may have been disturbed",
    }
}

fn reading_from_text(text: &str) -> Option<DisturbanceReading> {
    match text {
        "disturbed" => Some(DisturbanceReading::Disturbed),
        "may have been disturbed" => Some(DisturbanceReading::MayHaveBeenDisturbed),
        _ => None,
    }
}

fn outcome_as_text(outcome: TheOutcomeAsWritten) -> &'static str {
    match outcome {
        TheOutcomeAsWritten::Finished => "finished",
        TheOutcomeAsWritten::Failed => "failed",
        TheOutcomeAsWritten::NoStatusWasReaped => "no status was reaped",
        TheOutcomeAsWritten::LeftStopped => "left stopped",
        TheOutcomeAsWritten::GivenUp => "given up",
    }
}

fn outcome_from_text(text: &str) -> Option<TheOutcomeAsWritten> {
    match text {
        "finished" => Some(TheOutcomeAsWritten::Finished),
        "failed" => Some(TheOutcomeAsWritten::Failed),
        "no status was reaped" => Some(TheOutcomeAsWritten::NoStatusWasReaped),
        "left stopped" => Some(TheOutcomeAsWritten::LeftStopped),
        "given up" => Some(TheOutcomeAsWritten::GivenUp),
        _ => None,
    }
}

fn switch_as_text(switch: TheFilesSwitch) -> (&'static str, Option<i64>) {
    match switch {
        TheFilesSwitch::Static(on) => ("static", Some(on as i64)),
        TheFilesSwitch::Animated => ("animated", None),
    }
}

fn switch_from_text(text: &str, value: Option<i64>) -> Option<TheFilesSwitch> {
    match text {
        "static" => Some(TheFilesSwitch::Static(value? != 0)),
        "animated" => Some(TheFilesSwitch::Animated),
        _ => None,
    }
}

fn file_values(record: &LocalJobRecord) -> Option<&Companion> {
    record.the_files_values_beside_the_overrides.as_ref()
}

/// A moment this record can hold, or the refusal naming the column.
fn a_moment(column: &'static str, at: std::time::SystemTime) -> Result<i64, RecordFailure> {
    nanoseconds(at).ok_or(RecordFailure::AMomentTheRecordCannotHold {
        column,
        reported: "the moment is outside what this record's own spelling holds".into(),
    })
}

/// **A moment of the note's, with the machine whose clock it was taken on.**
///
/// **A note's moment is never on this computer's clock** (`State, identity and
/// recovery` D5, and `Queue & History` N4's *never across clocks*), so one that
/// is is refused rather than written with the machine column empty — which
/// would read back as a moment on a clock nothing names.
fn machine_moment(
    column: &'static str,
    reading: Option<&Reading<()>>,
) -> Result<(Option<i64>, Option<String>), RecordFailure> {
    let Some(reading) = reading else {
        return Ok((None, None));
    };
    let Clock::AMachine(MachineClock(machine)) = reading.clock() else {
        return Err(RecordFailure::AMomentTheRecordCannotHold {
            column,
            reported: "a moment of the machine's note is not on a machine's clock".into(),
        });
    };
    Ok((
        Some(a_moment(column, reading.taken_at())?),
        Some(machine.clone()),
    ))
}

fn a_machine_reading(at: Option<i64>, machine: Option<String>) -> Option<Reading<()>> {
    Some(Reading::taken(
        (),
        moment(at?),
        Clock::AMachine(MachineClock(machine?)),
    ))
}

/// One row of the table, before anything has been read back into a shape.
struct ARowAsRead {
    job: String,
    server: String,
    group: Option<String>,
    standing: String,
    render_start_learned: bool,
    server_label: Option<String>,
    server_hue: Option<i64>,
    submitted_at: Option<i64>,
    label: Option<String>,
    blend_path: Option<Vec<u8>>,
    scene_name: Option<String>,
    submitted_with_no_scene_named: bool,
    resolution_percentage_typed: Option<i64>,
    picture_width: Option<i64>,
    picture_height: Option<i64>,
    main_output: Option<String>,
    blender_build_path: String,
    blender_version: String,
    blender_origin: String,
    device_ask: String,
    device_choice: Option<String>,
    dependency_check: Option<String>,
    carries_on_from_job: Option<String>,
    carries_on_from_label: Option<String>,
    note_started: Option<i64>,
    note_started_machine: Option<String>,
    note_ended: Option<i64>,
    note_ended_machine: Option<String>,
    note_ending_outcome: Option<String>,
    note_ending_exit_status: Option<i32>,
    note_ending_capture_attempt: Option<i64>,
    note_attempts: i64,
    note_given_up_where_it_was: Option<i64>,
    note_last_frame_figure: Option<i64>,
    file_values_stood: bool,
    frame_start: Option<i64>,
    frame_end: Option<i64>,
    frame_step: Option<i64>,
    sample_count: Option<i64>,
    resolution_percentage: Option<i64>,
    engine: Option<String>,
    device: Option<String>,
    use_simplify: Option<i64>,
    subdivision_render: Option<i64>,
    child_particles_render: Option<f64>,
    texture_limit_render: Option<String>,
    start_never_heard: bool,
    /// Record version 27's column.
    the_reason_the_machine_recorded: Option<String>,
    /// Record version 28's column.
    the_device_line_as_observed: Option<String>,
    /// Record version 35's two columns.
    frames_asked: Option<String>,
    frames_cannot_be_counted: bool,
    /// **Record version 39's column**: where the job's output folder is, in
    /// `Queue & History`'s own shape as JSON, and nothing for a record written
    /// before it.
    job_output_folder: Option<String>,
    driver_mark_blender: Option<String>,
    driver_mark_driver_version: Option<String>,
}

/// **Record version 35's asked order, in this crate's own words**: one run a
/// line-free item, `first step how-many`, the runs joined by `;`. A column
/// that will not read back reads as no order, which draws as a dash — never as
/// a count invented from half a column.
fn the_frames_asked_as_text(asked: &smoozy_queue_history::record::TheFramesAsked) -> String {
    asked
        .runs
        .iter()
        .map(|run| format!("{} {} {}", run.first, run.step, run.how_many))
        .collect::<Vec<_>>()
        .join(";")
}

fn the_frames_asked_from_text(text: &str) -> Option<smoozy_queue_history::record::TheFramesAsked> {
    use smoozy_queue_history::record::{ARunOfFrames, TheFramesAsked};
    let mut runs = Vec::new();
    for run in text.split(';') {
        let mut parts = run.split(' ');
        let first = parts.next()?.parse().ok()?;
        let step = parts.next()?.parse().ok()?;
        let how_many = parts.next()?.parse().ok()?;
        if parts.next().is_some() {
            return None;
        }
        runs.push(ARunOfFrames { first, step, how_many });
    }
    (!runs.is_empty()).then_some(TheFramesAsked { runs })
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_queue_history::ending::Ending;
    use std::time::{Duration, SystemTime};

    /// **Record version 51: the frame text the user typed goes down with the
    /// record, in its transaction, and never into the kept setup** — `Queue &
    /// History` P5b, `Render Settings` F1, `State, identity and recovery` I10;
    /// the eighteenth notes, piece 7. It reads back after reopening, a record
    /// written with none reads none, a refused write takes the whole record
    /// with it, and deleting the record deletes the text.
    #[test]
    fn the_typed_frames_go_down_with_the_record_and_never_into_the_kept_setup() {
        let folder = std::env::temp_dir().join(format!("smoozy-typed-frames-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join("record");
        let record = crate::record::LocalRecord::open_at(&path).unwrap();
        let server = a_server();
        let typed = a_full_record(&a_job(), &server);
        let untyped = a_full_record(&a_job(), &server);
        let provisional = |job: &LocalJobRecord| AProvisionalRecord {
            job: job.job.clone(),
            server: server.clone(),
            group: job.group.clone(),
            label: job.label.clone().unwrap_or_default(),
        };
        TheSubmissionsWriter::of(record.connection(), typed.clone(), a_kept_setup())
            .with_the_typed_frames(Some(" 1,2-4 ".to_string()))
            .write_it(&provisional(&typed))
            .unwrap();
        TheSubmissionsWriter::of(record.connection(), untyped.clone(), a_kept_setup())
            .with_the_typed_frames(None)
            .write_it(&provisional(&untyped))
            .unwrap();
        let check = |record: &crate::record::LocalRecord| {
            let records = JobRecords::over(record.connection());
            assert_eq!(records.the_typed_frames_of(&typed.job).unwrap().as_deref(), Some("1,2-4"));
            assert_eq!(records.the_typed_frames_of(&untyped.job).unwrap(), None);
            // **The kept setup is the payload, and it carries numbers only.**
            let Some(TheKeptSetup::Kept(kept)) = records.the_kept_setup_of(&typed.job).unwrap() else {
                panic!("the kept setup reads back");
            };
            assert_eq!(kept, a_kept_setup());
            assert!(!kept.as_carried().0.to_string().contains("2-4"));
            assert_eq!(records.read(&typed.job).unwrap().unwrap(), typed);
        };
        check(&record);
        drop(record);
        let record = crate::record::LocalRecord::open_at(&path).unwrap();
        check(&record);
        // **All or nothing**: a refused text takes the record with it.
        record
            .connection()
            .execute_batch(
                "CREATE TRIGGER refuse_typed BEFORE UPDATE OF typed_frames ON job_record \
                 BEGIN SELECT RAISE(ABORT, 'refused'); END",
            )
            .unwrap();
        let another = a_full_record(&a_job(), &server);
        let refused = TheSubmissionsWriter::of(record.connection(), another.clone(), a_kept_setup())
            .with_the_typed_frames(Some("7,9".to_string()))
            .write_it(&provisional(&another))
            .unwrap_err();
        assert!(refused.contains("refused"), "{refused}");
        assert_eq!(JobRecords::over(record.connection()).read(&another.job).unwrap(), None);
        // **Deleting the record deletes the text with it** (`Queue & History` X1).
        JobRecords::over(record.connection()).remove(&typed.job).unwrap();
        assert_eq!(JobRecords::over(record.connection()).the_typed_frames_of(&typed.job).unwrap(), None);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **Record version 55 (`Telegram` V1c): the job's reporting choice is
    /// kept beside the record**, replaced by a later write (the value a
    /// change's acknowledgement carries), read back after reopening, absent
    /// for a job never given one, and gone with the record.
    #[test]
    fn twenty_first_7_the_reporting_choice_follows_the_acknowledgement() {
        let folder = std::env::temp_dir().join(format!("smoozy-reporting-choice-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join("record");
        let record = crate::record::LocalRecord::open_at(&path).unwrap();
        let server = a_server();
        let job = a_full_record(&a_job(), &server);
        let none = a_full_record(&a_job(), &server);
        for one in [&job, &none] {
            TheSubmissionsWriter::of(record.connection(), one.clone(), a_kept_setup())
                .write_it(&AProvisionalRecord {
                    job: one.job.clone(),
                    server: server.clone(),
                    group: one.group.clone(),
                    label: one.label.clone().unwrap_or_default(),
                })
                .unwrap();
        }
        let records = JobRecords::over(record.connection());
        records.the_reporting_choice_is_written(&job.job, "\"Everything\"").unwrap();
        assert_eq!(records.the_reporting_choice_of(&job.job).unwrap().as_deref(), Some("\"Everything\""));
        records.the_reporting_choice_is_written(&job.job, "\"Nothing\"").unwrap();
        assert_eq!(records.the_reporting_choice_of(&none.job).unwrap(), None);
        drop(record);
        let record = crate::record::LocalRecord::open_at(&path).unwrap();
        let records = JobRecords::over(record.connection());
        assert_eq!(records.the_reporting_choice_of(&job.job).unwrap().as_deref(), Some("\"Nothing\""));
        records.remove(&job.job).unwrap();
        assert_eq!(records.the_reporting_choice_of(&job.job).unwrap(), None);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    #[test]
    fn seventeenth_piece_2_again_purpose_pair_and_range_survive_reopening_and_failed_writes() {
        use smoozy_render_settings::frames::FrameForm;
        let folder = std::env::temp_dir().join(format!("smoozy-check-purpose-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join("record");
        let record = crate::record::LocalRecord::open_at(&path).unwrap();
        let server = a_server();
        let first = a_full_record(&a_job(), &server);
        let held = a_full_record(&a_job(), &server);
        let purpose = FirstFramePurpose { check_job:first.job.as_text(), range_job:held.job.as_text(),
            first_frame:-3, submitted_range:FrameForm::List(vec![-3, 0, 12]) };
        for job in [&first, &held] {
            let provisional = AProvisionalRecord { job:job.job.clone(), server:server.clone(), group:job.group.clone(), label:job.label.clone().unwrap_or_default() };
            TheSubmissionsWriter::of(record.connection(), job.clone(), a_kept_setup())
                .with_first_frame_purpose(Some(purpose.clone())).write_it(&provisional).unwrap();
        }
        let check = |record:&crate::record::LocalRecord| {
            let records = JobRecords::over(record.connection());
            for job in [&first, &held] { assert_eq!(records.first_frame_purpose(&job.job).unwrap(), Some(purpose.clone())); }
            // **Either part names the one pair** (the nineteenth notes, note 7).
            for job in [&first, &held] {
                let pair = crate::surface::the_queue::ThePair::of(&records, &job.job).unwrap();
                assert_eq!((&pair.check, &pair.range), (&first.job, &held.job));
                assert_eq!(pair.the_checks_line().lead, "First-frame check · frame");
                assert_eq!(pair.the_checks_line().value, "-3");
                assert_eq!(pair.the_frames, "-3,0,12");
            }
            assert_eq!(records.read(&held.job).unwrap().unwrap(), held);
        };
        check(&record);drop(record);
        let record = crate::record::LocalRecord::open_at(&path).unwrap();check(&record);
        record.connection().execute_batch("CREATE TRIGGER refuse_purpose BEFORE UPDATE OF first_frame_purpose ON job_record BEGIN SELECT RAISE(ABORT, 'refused'); END").unwrap();
        let another = a_full_record(&a_job(), &server);
        let provisional = AProvisionalRecord { job:another.job.clone(), server, group:another.group.clone(), label:another.label.clone().unwrap_or_default() };
        let refused = TheSubmissionsWriter::of(record.connection(), another.clone(), a_kept_setup())
            .with_first_frame_purpose(Some(FirstFramePurpose {range_job:another.job.as_text(), ..purpose}))
            .write_it(&provisional).unwrap_err();
        assert!(refused.contains("refused"), "the store rejected the purpose write: {refused}");
        assert_eq!(JobRecords::over(record.connection()).read(&another.job).unwrap(), None, "a purpose failure rolls back the entire provisional record");
        drop(record);let _ = std::fs::remove_dir_all(folder);
    }

    fn a_record_store() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        for migration in super::super::migrations::ALL {
            if let super::super::migrations::WhatItRuns::TheseStatements(statements) =
                migration.what_it_runs
            {
                for statement in statements {
                    connection.execute(statement, []).unwrap();
                }
            }
        }
        connection
    }

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).unwrap()
    }

    fn a_server() -> Identity {
        Identity::issue(IdentityKind::LocalServer).unwrap()
    }

    /// A record with every field filled, so a round trip has something to lose.
    fn a_full_record(job: &Identity, server: &Identity) -> LocalJobRecord {
        LocalJobRecord {
            job: job.clone(),
            group: Some(Identity::issue(IdentityKind::Group).unwrap()),
            server: TheServerAsItStood {
                local_server: server.clone(),
                label: Some("the big box".to_string()),
                hue: Some(Hue(210)),
            },
            submitted_at: Some(Reading::taken(
                (),
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000),
                Clock::ThisComputer,
            )),
            label: Some("Kitchen, whole animation".to_string()),
            blend_path_on_the_machine: Some(
                "/ephemeral/smoozy/assets/files/Users/a/kitchen.blend".to_string(),
            ),
            scene_name: None,
            submitted_with_no_scene_named: true,
            resolution_percentage_typed: Some(50),
            picture_size_from_the_read: None,
            main_output_from_the_read: None,
            blender: TheBlenderTheEntryKeeps {
                pick: VersionPick {
                    build_path: "/ephemeral/smoozy/blender/blender-4.5.12/blender".to_string(),
                    version: "4.5.12".to_string(),
                },
                origin: AnswerOrigin::TheUsersOwnOverride,
            },
            device_asked_for: TheDeviceAsk::TheUsersOwn(DeviceChoice::TheCards),
            the_files_values_beside_the_overrides: Some(Companion {
                frame_range: Some(TheFilesFrameRange {
                    start: 1,
                    end: 240,
                    step: 1,
                }),
                sample_count: Some(128),
                resolution_percentage: Some(100),
                engine: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                device: Some(DeviceSetting::TheProcessor),
                view_layer_pins: vec![
                    (
                        ExactName::as_the_reading_produced_it("ViewLayer"),
                        TheFilesSwitch::Static(true),
                    ),
                    (
                        ExactName::as_the_reading_produced_it("Behind"),
                        TheFilesSwitch::Animated,
                    ),
                ],
                simplify: TheFilesSimplify {
                    use_simplify: Some(false),
                    subdivision_render: Some(3),
                    child_particles_render: Some(0.5),
                    texture_limit_render: Some("2048".to_string()),
                },
            }),
            dependency_check: Some(DependencyCheckAtSubmission::NoStandingAtAll),
            carries_on_from: Some(CarriesOnFrom {
                earlier_job: a_job(),
                the_label_as_it_stood: "Kitchen, first pass".to_string(),
            }),
            disturbances: vec![ADisturbance {
                the_member: "/ephemeral/smoozy/assets/files/Users/a/tex/wood.png".to_string(),
                the_act_intended: "replace".to_string(),
                reading: DisturbanceReading::MayHaveBeenDisturbed,
                how_many: None,
            }],
            attempts_where_the_file_changed: vec![],
            the_frames: smoozy_queue_history::record::TheFramesTheReadFound::default(),
            standing: Standing::Provisional,
            the_render_start_has_been_learned: false,
            what_the_note_carried: WhatTheNoteCarried::default(),
            the_job_output_folder: Default::default(),
            the_driver_was_below_the_bar: None,
        }
    }

    fn a_note(server: &Identity) -> TheNote {
        let machine = Clock::AMachine(MachineClock(server.as_text()));
        let started = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_600);
        let mut note = TheNote::created_with_the_job_record("Kitchen, whole animation");
        note.started = Some(Reading::taken((), started, machine.clone()));
        note.ended = Some(Reading::taken(
            (),
            started + Duration::from_secs(3_600),
            machine,
        ));
        note.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::GivenUp,
            exit_status: Some(137),
            the_attempt_whose_capture_record_holds_the_words: 2,
            the_reason_the_machine_recorded: None,
            the_device_line_as_observed: None,
        });
        note.attempts = 2;
        note.re_run_marks = vec![AnAttemptMark::for_a_re_run(2, &[91, 92])];
        note.given_up_where_it_was = Some(90);
        note.last_frame_figure = Some(90);
        note
    }

    /// **The whole record goes down at submission and comes back whole**, sets
    /// and all — which is what makes an entry the same after a restart.
    #[test]
    fn the_record_written_at_submission_reads_back_field_for_field() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        let written = a_full_record(&job, &server);
        records.write_the_provisional_record(&written).unwrap();

        let read = records.read(&job).unwrap().expect("the record stands");
        assert_eq!(read, written);
    }

    /// **Record version 39: where the job's output folder is goes down with the
    /// record and comes back** — *chosen at render start* at submission, then
    /// the machine's choice once `Queue & History` N9 brings it, with its case
    /// and the passed-over mark; a record that holds nothing reads as the job
    /// area every older job had. The Creator's second notes, piece 4.
    #[test]
    fn where_the_jobs_folder_is_goes_down_with_the_record_and_is_filled_in_once() {
        use smoozy_job_files::where_the_folder_sits::{WhereItIs, WhereTheJobOutputFolderSits};
        use smoozy_queue_history::record::TheJobOutputFolderTheRecordHolds;
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        let written = LocalJobRecord {
            the_job_output_folder: TheJobOutputFolderTheRecordHolds::ChosenAtRenderStart,
            ..a_full_record(&job, &server)
        };
        records.write_the_provisional_record(&written).unwrap();
        assert_eq!(records.read(&job).unwrap().unwrap(), written);

        let chosen = WhereTheJobOutputFolderSits {
            path: "/home/ubuntu/Render/job-Kitchen".into(),
            where_it_is: WhereItIs::TheFallback {
                why: Some(smoozy_job_files::where_the_folder_sits::WhyTheFilesOwnPlaceWasNotUsed::ItIsAFolderSmoozyCannotUse),
                the_servers_fallback_folder_was_passed_over: true,
            },
        };
        records
            .the_read_is_filled_in(
                &job,
                &smoozy_queue_history::record::WhatTheReadProduced {
                    the_job_output_folder: Some(chosen.clone()),
                    ..Default::default()
                },
            )
            .unwrap();
        assert_eq!(records.read(&job).unwrap().unwrap().the_job_output_folder, TheJobOutputFolderTheRecordHolds::It(chosen));

        // **An older record holds nothing in the column**, and reads so.
        let older = a_job();
        records.write_the_provisional_record(&a_full_record(&older, &server)).unwrap();
        connection.execute("UPDATE job_record SET job_output_folder = NULL WHERE job = ?1", [older.as_text()]).unwrap();
        assert_eq!(
            records.read(&older).unwrap().unwrap().the_job_output_folder,
            TheJobOutputFolderTheRecordHolds::AsEveryJobBeforeItHadIt
        );
    }

    /// **`Queue & History` N16: a submitted moment that is not this computer's
    /// is refused rather than written.**
    #[test]
    fn a_submitted_moment_on_a_machines_clock_is_refused() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        let mut record = a_full_record(&job, &server);
        record.submitted_at = Some(Reading::taken(
            (),
            SystemTime::UNIX_EPOCH,
            Clock::AMachine(MachineClock(server.as_text())),
        ));
        assert!(matches!(
            records.write_the_provisional_record(&record),
            Err(RecordFailure::AMomentTheRecordCannotHold { column, .. }) if column == "submitted_at"
        ));
        assert!(records.read(&job).unwrap().is_none());
    }

    /// **No absence, of any length, removes a provisional record.**
    ///
    /// **This is the first thing a later agent will break.** The third branch
    /// is asked a thousand times and the record is there every time, and — the
    /// half that matters — the store's own text carries nothing to build a
    /// timeout out of.
    #[test]
    fn no_absence_of_any_length_removes_a_provisional_record() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        records
            .write_the_provisional_record(&a_full_record(&job, &server))
            .unwrap();

        for _ in 0..1_000 {
            assert_eq!(
                records
                    .the_resolution(&job, WhatTheMachineSaidAboutTheJob::TheMachineCannotSay)
                    .unwrap(),
                WhatTheResolutionDid::TheStepsOwnAnswer(
                    WhatAnAbsenceDoes::TheRecordStaysProvisionalAndIsAskedAgainNextTime
                )
            );
            let still = records.read(&job).unwrap().expect("it is still there");
            assert_eq!(still.standing, Standing::Provisional);
        }
        assert_eq!(records.provisional_for(&server).unwrap().len(), 1);

        // And the one machine answer that does remove it is the machine's own
        // operation record, never an absence.
        assert_eq!(
            records
                .the_resolution(
                    &job,
                    WhatTheMachineSaidAboutTheJob::ItsOperationRecordShowsTheSubmissionWasRefusedOrNeverTaken
                )
                .unwrap(),
            WhatTheResolutionDid::TheStepsOwnAnswer(
                WhatAnAbsenceDoes::TheRecordIsRemovedLeavingNoTrace
            )
        );
        assert!(records.read(&job).unwrap().is_none());
        for table in EVERY_SET_BESIDE_THE_ROW {
            let left: i64 = connection
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                    row.get(0)
                })
                .unwrap();
            assert_eq!(
                left, 0,
                "{table} was left behind, so the record left a trace"
            );
        }
    }

    /// **The store has nothing to measure an absence with**, which is what
    /// makes the rule above hold against a line somebody adds later. The scan
    /// is of this file's own working half.
    #[test]
    fn nothing_in_this_store_can_tell_how_long_a_machine_has_been_quiet() {
        let source = include_str!("job_records.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let resolution = working_half
            .split("pub fn the_resolution")
            .nth(1)
            .expect("the resolution is in the working half");
        let body = resolution
            .split("\n    }")
            .next()
            .expect("the function has a body");
        for forbidden in [
            "elapsed",
            "Instant",
            "SystemTime",
            "Duration",
            "timeout",
            "deadline",
        ] {
            assert!(
                !body.contains(forbidden),
                "{forbidden} in the resolution is the determination \
                 `Preparation/Specs/Whole app.md` forbids"
            );
        }
    }

    /// **A machine that holds the job confirms the record, and confirmation is
    /// not the render having started** — so the record is still in no list.
    #[test]
    fn a_confirmed_record_is_not_yet_a_history_entry() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        records
            .write_the_provisional_record(&a_full_record(&job, &server))
            .unwrap();
        records
            .the_resolution(&job, WhatTheMachineSaidAboutTheJob::ItHoldsTheJob)
            .unwrap();

        let read = records.read(&job).unwrap().unwrap();
        assert_eq!(read.standing, Standing::Confirmed);
        assert!(!read.the_render_start_has_been_learned);
        assert!(records.in_history().unwrap().entries.is_empty());
        assert!(records.provisional_for(&server).unwrap().is_empty());
    }

    /// **The fill-in carries the note's half over, puts the job into history,
    /// and never overwrites what `Queue & History` N16 protects.**
    #[test]
    fn the_fill_in_carries_the_note_and_leaves_the_submitted_moment_alone() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        let written = a_full_record(&job, &server);
        records.write_the_provisional_record(&written).unwrap();

        assert_eq!(
            records.the_fill_in(&job, &a_note(&server)).unwrap(),
            WhatTheFillInDid::ItCarriedTheNotesHalfOver
        );
        let read = records.read(&job).unwrap().unwrap();

        // The carried half is there, across a read from the store.
        assert_eq!(read.the_ending_learned(), Some(Ending::GivenUp));
        assert_eq!(read.what_the_note_carried.attempts, 2);
        assert_eq!(read.what_the_note_carried.last_frame_figure, Some(90));
        assert_eq!(read.what_the_note_carried.given_up_where_it_was, Some(90));
        assert_eq!(
            read.what_the_note_carried.re_run_marks,
            vec![AnAttemptMark {
                attempt: 2,
                the_re_run_proceeded_from: Some(91),
            }]
        );
        assert_eq!(
            read.the_four_moments().exactly_one_duration(),
            Some(Duration::from_secs(3_600))
        );
        // **`Queue & History` C9: it is in history from the render start.**
        assert!(read.the_render_start_has_been_learned);
        assert_eq!(records.in_history().unwrap().entries.len(), 1);

        // **`Queue & History` N16 is untouched, and it is still this
        // computer's clock.**
        assert_eq!(read.submitted_at, written.submitted_at);
        assert_eq!(
            read.the_four_moments().the_submitted_moment(),
            written.the_four_moments().the_submitted_moment()
        );
        // And every other field Smoozy's own record carries.
        assert_eq!(read.label, written.label);
        assert_eq!(read.server, written.server);
        assert_eq!(read.blender, written.blender);
        assert_eq!(read.device_asked_for, written.device_asked_for);
        assert_eq!(read.disturbances, written.disturbances);
        assert_eq!(read.carries_on_from, written.carries_on_from);
        assert_eq!(
            read.the_files_values_beside_the_overrides,
            written.the_files_values_beside_the_overrides
        );
    }

    /// **A history tombstone refuses the fill-in unconditionally** — whatever
    /// the machine still holds, and whether or not the machine-side copy was
    /// ever removed.
    #[test]
    fn a_tombstone_refuses_the_fill_in_and_the_machine_side_copy_does_not_lift_it() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        records
            .write_the_provisional_record(&a_full_record(&job, &server))
            .unwrap();
        for copy in ["still owed", "removed"] {
            connection
                .execute(
                    "INSERT OR REPLACE INTO history_tombstone (job, deleted_at, machine_side_copy)
                     VALUES (?1, 1, ?2)",
                    rusqlite::params![job.as_text(), copy],
                )
                .unwrap();
            assert_eq!(
                records.the_fill_in(&job, &a_note(&server)).unwrap(),
                WhatTheFillInDid::ATombstoneRefusedIt
            );
            assert_eq!(
                records
                    .the_read_is_filled_in(&job, &Default::default())
                    .unwrap(),
                WhatTheFillInDid::ATombstoneRefusedIt
            );
            let read = records.read(&job).unwrap().unwrap();
            assert!(read.what_the_note_carried.ending.is_none());
            assert!(!read.the_render_start_has_been_learned);
        }
    }

    /// **`Queue & History` N9's read fills what the record lacks and never what
    /// it carries**, through that module's own merge rule.
    #[test]
    fn the_read_fills_the_scene_name_and_leaves_the_no_scene_mark_standing() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        records
            .write_the_provisional_record(&a_full_record(&job, &server))
            .unwrap();
        records
            .the_read_is_filled_in(
                &job,
                &smoozy_queue_history::record::WhatTheReadProduced {
                    the_job_output_folder: None,
                    scene_name: Some("Interior".to_string()),
                    picture_size: Some((1920, 1080)),
                    main_output_or_the_jobs_folder: Some("/ephemeral/x/frame_####.png".to_string()),
                    attempts_where_the_file_changed: vec![AnAttemptWhereTheFileChanged {
                        attempt: 2,
                        what_became_of_the_frames_already_finished:
                            WhatBecameOfTheFramesAlreadyFinished::TheyWereReused,
                    }],
                    the_frames: smoozy_queue_history::record::TheFramesTheReadFound {
                        asked: smoozy_queue_history::record::TheFramesAsked::from_the_order(
                            &[1001, 1002, 1003, 7, 9, 11],
                        ),
                        they_cannot_be_counted: true,
                    },
                },
            )
            .unwrap();
        let read = records.read(&job).unwrap().unwrap();
        assert_eq!(read.scene_name.as_deref(), Some("Interior"));
        assert_eq!(read.picture_size_from_the_read, Some((1920, 1080)));
        assert!(read.submitted_with_no_scene_named);
        assert_eq!(read.attempts_where_the_file_changed.len(), 1);
        // **Record version 35's two columns read back as written** — the
        // asked order as two runs, and the two-view mark (the 38's piece 10).
        assert_eq!(
            read.the_frames.asked,
            smoozy_queue_history::record::TheFramesAsked::from_the_order(&[1001, 1002, 1003, 7, 9, 11])
        );
        assert_eq!(read.the_frames.asked.as_ref().map(|asked| asked.runs.len()), Some(2));
        assert!(read.the_frames.they_cannot_be_counted);
    }

    /// **The submission seam writes the whole record, and it refuses a
    /// submission it was not given the record for.**
    #[test]
    fn the_submission_seam_writes_the_whole_record_and_refuses_a_stranger() {
        let connection = a_record_store();
        let job = a_job();
        let server = a_server();
        let record = a_full_record(&job, &server);
        let group = record.group.clone();
        let mut writer = TheSubmissionsWriter::of(&connection, record, a_kept_setup());

        assert!(writer
            .write_it(&AProvisionalRecord {
                job: a_job(),
                server: server.clone(),
                group: group.clone(),
                label: "Kitchen, whole animation".to_string(),
            })
            .is_err());
        assert!(JobRecords::over(&connection).read(&job).unwrap().is_none());
        // **The fourth field is checked with the other three** (must-do 5): the
        // right job, server and group under another label is still a stranger.
        assert!(writer
            .write_it(&AProvisionalRecord {
                job: job.clone(),
                server: server.clone(),
                group: group.clone(),
                label: "Bedroom, one frame".to_string(),
            })
            .is_err());
        assert!(JobRecords::over(&connection).read(&job).unwrap().is_none());

        writer
            .write_it(&AProvisionalRecord {
                job: job.clone(),
                server,
                group,
                label: "Kitchen, whole animation".to_string(),
            })
            .unwrap();
        let read = JobRecords::over(&connection).read(&job).unwrap().unwrap();
        assert_eq!(read.label.as_deref(), Some("Kitchen, whole animation"));

        // **Record version 25: the kept setup went down with the record.**
        assert_eq!(
            JobRecords::over(&connection).the_kept_setup_of(&job).unwrap(),
            Some(TheKeptSetup::Kept(a_kept_setup()))
        );

        writer.remove_it(&job).unwrap();
        assert!(JobRecords::over(&connection).read(&job).unwrap().is_none());
        assert_eq!(JobRecords::over(&connection).the_kept_setup_of(&job).unwrap(), None);
    }

    /// **The payload a job is submitted with**, with a frame range, a sample
    /// count and an engine pick — the three facts version 23 had no column for.
    fn a_kept_setup() -> Payload {
        use smoozy_render_settings::engine::OfferedEngine;
        use smoozy_render_settings::file_half::BlenderIdentity;
        use smoozy_render_settings::payload::{compose, Composition};
        use smoozy_render_settings::store::SceneEntries;
        let answers = SceneEntries {
            frame_range: Some(
                smoozy_render_settings::frames::parse_the_frame_field("1-240").unwrap(),
            ),
            sample_count: Some(64),
            engine_pick: Some(OfferedEngine::Cycles),
            ..Default::default()
        };
        compose(Composition {
            version_pick: &BlenderIdentity {
                path: "/ephemeral/smoozy/blender/blender-4.5.12/blender".to_string(),
                version: smoozy_server_setup::blender::Version {
                    major: 4,
                    minor: 5,
                    patch: 12,
                },
            },
            scene: None,
            answers: &answers,
            standing: None,
            stale: false,
        })
    }

    /// **Record version 25, and the gap step 10b piece 6a handed up: the frame
    /// range, the sample count and the engine the job was submitted with come
    /// back from the record**, whole — and a record written without a kept
    /// setup says so rather than inventing one.
    #[test]
    fn the_kept_setup_is_written_with_the_record_and_read_back_whole() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let server = a_server();

        let kept = a_job();
        records
            .write_the_provisional_record_and_its_kept_setup(
                &a_full_record(&kept, &server),
                &a_kept_setup(),
            )
            .unwrap();
        let Some(TheKeptSetup::Kept(payload)) = records.the_kept_setup_of(&kept).unwrap() else {
            panic!("the kept setup did not come back");
        };
        assert_eq!(payload, a_kept_setup());
        assert_eq!(payload.sample_count, Some(64));
        assert!(payload.frame_range.is_some());
        assert!(payload.engine.is_some());

        let before = a_job();
        records
            .write_the_provisional_record(&a_full_record(&before, &server))
            .unwrap();
        assert_eq!(
            records.the_kept_setup_of(&before).unwrap(),
            Some(TheKeptSetup::NoneWasKept)
        );

        connection
            .execute(
                "UPDATE job_record SET kept_setup = 'not a payload' WHERE job = ?1",
                [before.as_text()],
            )
            .unwrap();
        assert_eq!(
            records.the_kept_setup_of(&before).unwrap(),
            Some(TheKeptSetup::ItWillNotReadBack)
        );
        assert_eq!(records.the_kept_setup_of(&a_job()).unwrap(), None);
    }

    /// **Record version 42: `Queue & History` N23's mark lands on a record that
    /// stands, copies the job's own Blender version, survives the fill-in, and
    /// says it did not land where no record stands.** Held-job piece A.
    #[test]
    fn the_driver_mark_is_written_onto_a_standing_record_and_read_back() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        let written = a_full_record(&job, &server);
        records.write_the_provisional_record(&written).unwrap();
        assert_eq!(records.read(&job).unwrap().unwrap().the_driver_was_below_the_bar, None);

        assert!(records.the_driver_mark_is_written(&job, Some("550.163.01")).unwrap());
        let read = records.read(&job).unwrap().unwrap();
        assert_eq!(
            read.the_driver_was_below_the_bar,
            Some(smoozy_queue_history::record::TheDriverWasBelowTheBar {
                the_drivers_version: Some("550.163.01".to_string()),
                the_jobs_blender: written.blender.pick.version.clone(),
            })
        );
        // **The note's fill-in leaves it where it is** (`Queue & History` N10).
        records.the_fill_in(&job, &a_note(&server)).unwrap();
        assert!(records.read(&job).unwrap().unwrap().the_driver_was_below_the_bar.is_some());

        // **A driver that gave no version is written as none.**
        assert!(records.the_driver_mark_is_written(&job, None).unwrap());
        assert_eq!(
            records
                .read(&job)
                .unwrap()
                .unwrap()
                .the_driver_was_below_the_bar
                .and_then(|mark| mark.the_drivers_version),
            None
        );
        // **No record, no mark, and the answer says so.**
        assert!(!records.the_driver_mark_is_written(&a_job(), Some("550.163.01")).unwrap());
    }

    /// **Record version 27: the machine's recorded reason is kept whole beside
    /// a record that stands, and nowhere else.** Step 10c's sixth pass, fault 3.
    /// **Record version 50: `Rented-server runtime` V12's reading, landed by
    /// the fill-in with the ending, comes back whole — and a later note never
    /// replaces it.** Before step 12's SC4 findings the record had nowhere to
    /// keep one, so History could not name a full disk after the connection.
    #[test]
    fn the_free_space_at_the_ending_is_kept_with_the_ending_and_never_replaced() {
        use smoozy_queue_history::acting::note::{TheFreeSpaceAtTheEnding, TheFreeSpaceReadAtTheEnding};
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        records.write_the_provisional_record(&a_full_record(&job, &server)).unwrap();
        let mut note = a_note(&server);
        let reading = TheFreeSpaceReadAtTheEnding {
            available_bytes: 0,
            the_folder_it_was_taken_against: "/root/jobs/kitchen/output".to_string(),
            taken_at: Reading::taken(
                (),
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_004_200),
                Clock::AMachine(MachineClock(server.as_text())),
            ),
        };
        note.the_free_space_at_the_ending = Some(TheFreeSpaceAtTheEnding::It(reading.clone()));
        records.the_fill_in(&job, &note).unwrap();
        let read = records.read(&job).unwrap().unwrap();
        assert_eq!(read.what_the_note_carried.the_free_space_at_the_ending, Some(reading.clone()));
        let mut later = note.clone();
        later.the_free_space_at_the_ending = Some(TheFreeSpaceAtTheEnding::It(TheFreeSpaceReadAtTheEnding {
            available_bytes: 155_000_000,
            ..reading.clone()
        }));
        records.the_fill_in(&job, &later).unwrap();
        assert_eq!(
            records.read(&job).unwrap().unwrap().what_the_note_carried.the_free_space_at_the_ending,
            Some(reading),
            "the reading is immutable with the ending it rides"
        );
        let other = a_job();
        records.write_the_provisional_record(&a_full_record(&other, &server)).unwrap();
        records.the_fill_in(&other, &a_note(&server)).unwrap();
        assert_eq!(
            records.read(&other).unwrap().unwrap().what_the_note_carried.the_free_space_at_the_ending,
            None,
            "an ending that carried none keeps none"
        );
    }

    #[test]
    fn the_reason_the_machine_recorded_is_kept_whole_beside_a_standing_record() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        records
            .write_the_provisional_record(&a_full_record(&job, &a_server()))
            .unwrap();
        assert_eq!(records.the_reason_the_machine_recorded(&job).unwrap(), None);

        let reason = "Smoozy does not know how to render on a NVIDIA GeForce GTX 1650";
        assert!(records.the_reason_the_machine_recorded_is_kept(&job, reason).unwrap());
        assert_eq!(
            records.the_reason_the_machine_recorded(&job).unwrap().as_deref(),
            Some(reason)
        );
        // **An empty reason is no reason**, and the one kept stands.
        assert!(!records.the_reason_the_machine_recorded_is_kept(&job, "").unwrap());
        assert_eq!(
            records.the_reason_the_machine_recorded(&job).unwrap().as_deref(),
            Some(reason)
        );
        // **A job with no record gets nothing written for it.**
        let gone = a_job();
        assert!(!records.the_reason_the_machine_recorded_is_kept(&gone, reason).unwrap());
        assert_eq!(records.the_reason_the_machine_recorded(&gone).unwrap(), None);
    }

    /// **Record versions 27 and 28, filled in from the note: the reason the
    /// machine recorded and the device line as observed are kept beside the
    /// record by the fill-in's own transaction, and the ending read back
    /// carries both, verbatim** — step 10c's fix piece 2, the paragraph
    /// after `Queue & History` N3.
    ///
    /// **Undone, this fails at the reason**: the writer was built and nothing
    /// called it, so no reason ever reached a History entry.
    #[test]
    fn the_fill_in_keeps_the_reason_and_the_device_line_and_the_ending_reads_back_whole() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        records
            .write_the_provisional_record(&a_full_record(&job, &server))
            .unwrap();
        let reason = "the render ended with status 3";
        let line = "Path tracing on: NVIDIA GeForce RTX 4090 (CUDA)";
        let mut note = a_note(&server);
        note.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Failed,
            exit_status: Some(3),
            the_attempt_whose_capture_record_holds_the_words: 2,
            the_reason_the_machine_recorded: Some(reason.to_string()),
            the_device_line_as_observed: Some(line.to_string()),
        });
        assert!(matches!(
            records.the_fill_in(&job, &note).unwrap(),
            WhatTheFillInDid::ItCarriedTheNotesHalfOver
        ));
        assert_eq!(
            records.the_reason_the_machine_recorded(&job).unwrap().as_deref(),
            Some(reason)
        );
        let read = records.read(&job).unwrap().expect("it stands");
        assert_eq!(
            read.what_the_note_carried.ending, note.ending,
            "the ending read off the record is the ending the note carried"
        );

        // **A note whose ending carries neither leaves both columns empty**,
        // and the ending reads back with neither.
        let other = a_job();
        records
            .write_the_provisional_record(&a_full_record(&other, &server))
            .unwrap();
        records.the_fill_in(&other, &a_note(&server)).unwrap();
        let read = records.read(&other).unwrap().expect("it stands");
        let ending = read.what_the_note_carried.ending.expect("an ending");
        assert_eq!(ending.the_reason_the_machine_recorded, None);
        assert_eq!(ending.the_device_line_as_observed, None);
        assert_eq!(records.the_reason_the_machine_recorded(&other).unwrap(), None);
    }

    /// **The kept setup goes down in the record's own transaction**: where the
    /// record is refused, no kept setup stands for it either.
    #[test]
    fn a_refused_record_leaves_no_kept_setup_behind() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let mut record = a_full_record(&job, &a_server());
        record.submitted_at = Some(Reading::taken(
            (),
            std::time::SystemTime::UNIX_EPOCH,
            Clock::AMachine(MachineClock("elsewhere".to_string())),
        ));
        assert!(records
            .write_the_provisional_record_and_its_kept_setup(&record, &a_kept_setup())
            .is_err());
        assert_eq!(records.the_kept_setup_of(&job).unwrap(), None);
    }

    /// **`Queue & History` A6: an accepted offer goes down in one write, or
    /// none of it does.** Two adopted records, the second under an identity a
    /// record already stands for: the write is refused whole, and the first
    /// is not left behind with its kept setup. The same two, with nothing in
    /// the way, both stand, confirmed, each with its kept setup. Owed piece 10.
    #[test]
    fn an_accepted_offer_is_written_whole_or_not_at_all() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let server = a_server();
        let adopted = |job: &Identity| {
            let mut record = a_full_record(job, &server);
            record.standing = Standing::Confirmed;
            record.submitted_at = None;
            (record, Some(a_kept_setup()))
        };
        let (first, second) = (a_job(), a_job());
        records.write_the_provisional_record(&a_full_record(&second, &server)).unwrap();
        assert!(records.the_adopted_are_written(&[adopted(&first), adopted(&second)]).is_err());
        assert_eq!(records.read(&first).unwrap(), None, "nothing of the offer was adopted");
        assert_eq!(records.the_kept_setup_of(&first).unwrap(), None);

        let offer = [adopted(&a_job()), adopted(&a_job())];
        records.the_adopted_are_written(&offer).unwrap();
        for (written, _) in &offer {
            let read = records.read(&written.job).unwrap().expect("adopted");
            assert_eq!(read.standing, Standing::Confirmed);
            assert_eq!(&read, written);
            assert!(matches!(records.the_kept_setup_of(&written.job).unwrap(), Some(TheKeptSetup::Kept(_))));
        }
    }

    /// A record in history, written whole and then learned to have started.
    fn a_record_in_history(connection: &rusqlite::Connection) -> Identity {
        let records = JobRecords::over(connection);
        let job = a_job();
        records
            .write_the_provisional_record(&a_full_record(&job, &a_server()))
            .unwrap();
        connection
            .execute(
                "UPDATE job_record SET render_start_learned = 1 WHERE job = ?1",
                [job.as_text()],
            )
            .unwrap();
        job
    }

    /// **Must-do 3(b) and 3(c): a record with a word that will not read back is
    /// a row that will not read back, and History is told how many.**
    ///
    /// **Each of these failed before the fix**: the disturbance and the pin
    /// were skipped with `continue`, so the record came back with the mark
    /// missing and was drawn as one nobody had touched; the dependency check's
    /// state and the ending were dropped the same way; and a whole row that
    /// would not read back vanished with nothing counted.
    #[test]
    fn a_row_with_a_word_that_will_not_read_back_is_counted_and_not_drawn() {
        for (what, spoil) in [
            (
                "a disturbance mark",
                "UPDATE job_record_disturbance SET reading = 'smudged' WHERE job = ?1",
            ),
            (
                "a view-layer pin",
                "UPDATE job_record_file_value_pin SET switch = 'sideways' WHERE job = ?1",
            ),
            (
                "the dependency check's state",
                "UPDATE job_record SET dependency_check = 'half checked' WHERE job = ?1",
            ),
            (
                "the file's device",
                "UPDATE job_record SET file_values_device = 'the toaster' WHERE job = ?1",
            ),
            // **The control**: a statement that changes nothing, so both
            // records read and nothing is counted.
            (
                "nothing at all",
                "UPDATE job_record SET job = job WHERE job = ?1 AND 0",
            ),
        ] {
            let connection = a_record_store();
            let records = JobRecords::over(&connection);
            let good = a_record_in_history(&connection);
            let spoiled = a_record_in_history(&connection);
            let changed = connection.execute(spoil, [spoiled.as_text()]).unwrap();
            let held = records.in_history().unwrap();
            if changed == 0 {
                // The control: nothing spoiled, so both read and none is counted.
                assert_eq!(held.entries.len(), 2, "{what}");
                assert_eq!(held.could_not_be_read, 0, "{what}");
                continue;
            }
            assert_eq!(
                held.entries.iter().map(|one| one.record.job.clone()).collect::<Vec<_>>(),
                vec![good.clone()],
                "a record with {what} that would not read back was drawn"
            );
            assert_eq!(held.could_not_be_read, 1, "{what} was dropped without a word");
            assert!(records.read(&spoiled).unwrap().is_none(), "{what}");
        }
    }

    /// **A delete inside a job's folder leaves one mark per delete act, saying
    /// what went** — the build runner's ruling (d) of 2026-10-03,
    /// `Design/Queue & History § The marks`. Out of the Vast pass before
    /// 0.1.9, where deleting one frame left a mark naming the job's whole
    /// folder. Each case starts from the mark the delete writes before its
    /// first removal, on the member the job's set was reached at:
    ///
    /// - one frame deleted names that frame, with no count;
    /// - a folder of twelve frames deleted names the folder with the count —
    ///   one mark, not twelve;
    /// - twelve frames none of which answered reads *may have been*, with the
    ///   count of those never answered;
    /// - a delete of a folder holding the job's `.blend` still names the
    ///   member, as every mark did before.
    #[test]
    fn a_delete_inside_a_jobs_folder_leaves_one_mark_saying_what_went() {
        use crate::managing_files::{a_mark, the_deletes_marks_are_finished, A_DELETE};
        use smoozy_file_browser::warnings::{AJobNamed, HowTheJobStands};
        use smoozy_state::outcomes::UnitOutcome;
        struct Here(std::sync::Mutex<rusqlite::Connection>);
        impl crate::uploading::TheRecordAnUploadWrites for Here {
            fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
                act(&self.0.lock().unwrap_or_else(|poisoned| poisoned.into_inner()));
                true
            }
        }
        let frames = b"/root/job/frames".to_vec();
        let frame = |at: u32| format!("/root/job/frames/{at:04}.png").into_bytes();
        let twelve_done: Vec<(Vec<u8>, bool, Option<UnitOutcome>)> = (1..=12)
            .map(|at| (frame(at), false, Some(UnitOutcome::Done)))
            .chain(std::iter::once((frames.clone(), true, Some(UnitOutcome::Done))))
            .collect();
        let twelve_unanswered: Vec<(Vec<u8>, bool, Option<UnitOutcome>)> =
            (1..=12).map(|at| (frame(at), false, None)).collect();
        for (case, member, entries, victims, wanted) in [
            (
                "one frame",
                b"/root/job".to_vec(),
                vec![(frame(1), false, Some(UnitOutcome::Done))],
                vec![frame(1)],
                ("/root/job/frames/0001.png", None, DisturbanceReading::Disturbed),
            ),
            (
                "a folder of twelve",
                b"/root/job".to_vec(),
                twelve_done.clone(),
                vec![frames.clone()],
                ("/root/job/frames/", Some(12), DisturbanceReading::Disturbed),
            ),
            (
                "twelve never answered",
                b"/root/job".to_vec(),
                twelve_unanswered.clone(),
                (1..=12).map(frame).collect(),
                ("/root/job/frames/", Some(12), DisturbanceReading::MayHaveBeenDisturbed),
            ),
            (
                "a folder above the job's file",
                b"/root/job/scene.blend".to_vec(),
                vec![(b"/root/job/scene.blend".to_vec(), false, Some(UnitOutcome::Done)), (b"/root/job".to_vec(), true, Some(UnitOutcome::Done))],
                vec![b"/root/job".to_vec()],
                ("/root/job/scene.blend", None, DisturbanceReading::Disturbed),
            ),
        ] {
            let connection = a_record_store();
            let job = a_record_in_history(&connection);
            assert!(a_mark_is_appended_to(&connection, &job, &a_mark(&member, A_DELETE)).unwrap());
            let named = vec![AJobNamed {
                job: job.clone(),
                stands: HowTheJobStands::Rendering,
                the_members_reached: vec![member.clone()],
                the_set_was_the_widest_case: true,
            }];
            let here = Here(std::sync::Mutex::new(connection));
            the_deletes_marks_are_finished(&here, &named, &entries, &victims);
            let connection = here.0.into_inner().unwrap();
            // **The record carries a replace mark of its own** (`a_full_record`);
            // the delete's marks are the ones counted.
            let marks: Vec<ADisturbance> = JobRecords::over(&connection)
                .the_disturbances_of(&job.as_text())
                .unwrap()
                .unwrap()
                .into_iter()
                .filter(|one| one.the_act_intended == A_DELETE)
                .collect();
            assert_eq!(marks.len(), 1, "{case}: one mark per delete act");
            assert_eq!(
                (marks[0].the_member.as_str(), marks[0].how_many, marks[0].reading),
                wanted,
                "{case}"
            );
        }
    }

    /// **The ending the note carried, spoiled, is a row that will not read
    /// back — never a record drawn as *ending not known*.**
    #[test]
    fn an_ending_that_will_not_read_back_is_not_drawn_as_not_known() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let job = a_job();
        let server = a_server();
        records
            .write_the_provisional_record(&a_full_record(&job, &server))
            .unwrap();
        records.the_fill_in(&job, &a_note(&server)).unwrap();
        assert_eq!(records.in_history().unwrap().entries.len(), 1);
        connection
            .execute(
                "UPDATE job_record SET note_ending_outcome = 'wandered off' WHERE job = ?1",
                [job.as_text()],
            )
            .unwrap();
        let held = records.in_history().unwrap();
        assert!(held.entries.is_empty());
        assert_eq!(held.could_not_be_read, 1);
    }

    /// **Item C1: a server that is gone turns every record whose start was
    /// never learned into a history entry marked *start never heard*** — and
    /// removes nothing, invents no start, and leaves a record whose start was
    /// learned exactly as it was.
    #[test]
    fn a_server_that_is_gone_keeps_the_jobs_whose_start_was_never_heard() {
        let connection = a_record_store();
        let records = JobRecords::over(&connection);
        let server = a_server();
        let elsewhere = a_server();
        let provisional = a_job();
        let confirmed = a_job();
        let rendered = a_job();
        let on_another_server = a_job();
        for (job, on) in [
            (&provisional, &server),
            (&confirmed, &server),
            (&rendered, &server),
            (&on_another_server, &elsewhere),
        ] {
            records
                .write_the_provisional_record(&a_full_record(job, on))
                .unwrap();
        }
        records
            .the_resolution(&confirmed, WhatTheMachineSaidAboutTheJob::ItHoldsTheJob)
            .unwrap();
        records.the_fill_in(&rendered, &a_note(&server)).unwrap();
        assert_eq!(records.in_history().unwrap().entries.len(), 1);

        assert_eq!(records.the_server_is_gone(&server).unwrap(), 2);

        let held = records.in_history().unwrap();
        let mut in_history: Vec<(Identity, bool)> = held
            .entries
            .iter()
            .map(|one| (one.record.job.clone(), one.start_never_heard))
            .collect();
        in_history.sort_by_key(|(job, _)| job.as_text());
        let mut expected = vec![
            (provisional.clone(), true),
            (confirmed.clone(), true),
            (rendered.clone(), false),
        ];
        expected.sort_by_key(|(job, _)| job.as_text());
        assert_eq!(in_history, expected);

        // **Nothing invented**: the start is still not learned, no started
        // moment is written, and the marks stay with the record.
        let kept = records.read(&provisional).unwrap().unwrap();
        assert!(!kept.the_render_start_has_been_learned);
        assert!(kept.what_the_note_carried.started.is_none());
        assert_eq!(kept.disturbances.len(), 1);
        // **Step 2 no longer asks about the marked ones**, their server being
        // gone; the record whose start was learned is untouched by the mark.
        assert_eq!(
            records
                .provisional_for(&server)
                .unwrap()
                .into_iter()
                .map(|one| one.job)
                .collect::<Vec<_>>(),
            vec![rendered.clone()]
        );
        // **Another server's waiting job is untouched.**
        assert_eq!(records.provisional_for(&elsewhere).unwrap().len(), 1);
    }
}

//! **§10 N1 to N9 performed — the per-job note, one durable record per job in
//! the package area, written by the supervisor in the same act as the record
//! that makes each fact true.**
//!
//! # Why one fact never has two disagreeing homes
//!
//! **N2 is the whole reason this module has no *update the note* call of its
//! own.** Every write below names the event that makes its fact true — the
//! attempt starting, the recorded outcome being written, a disposition of §6
//! landing — and there is no function here that takes a note and a value and
//! writes them without one. **The started time lands when the render
//! operation's attempt starts; the ending and its time land when the recorded
//! outcome is written; the re-run marks land with the dispositions.**
//!
//! # The null ending, and why it is a variant rather than a default
//!
//! **N1: a null ending for a job that has not ended, never an invented one.**
//! [`TheNote::ending`] is [`None`] until a recorded outcome is written, and
//! nothing here fills it from a reading, a stream or a walk.
//!
//! **N5 is why that matters most exactly when it matters most.** With the
//! filesystem at a genuine zero every *read* answered normally and truthfully,
//! before the refused writes and after them, and no read reported success from
//! nothing. The limit: while the state mount is full the *writes* refuse, so
//! an ending that occurs then is an ending the note learns only after space
//! returns. **The refused write leaves the null ending standing, which is the
//! truthful state** — and this module has no other answer for a refused write.
//!
//! # N3: a pointer, never a diagnosis
//!
//! **The note carries the ending's classification, the exit status verbatim
//! and a reference to the attempt's capture record with its two collection
//! outcomes.** The words themselves stay in the sinks (E7). **Nothing in the
//! note is mined from a stream**, and there is no field here for a cause, a
//! summary or a first line.
//!
//! # Two things more than N3 names, and neither is a diagnosis
//!
//! **`Queue & History` §10, the paragraph after N3 (added 2026-09-27): the
//! note's ending carries, and the answer that reads the note sends, the reason
//! the machine recorded with the outcome and the device line as the worker
//! observed it at its own loop, both verbatim.** The reason is `Rendering`
//! K14's, in the words the machine wrote at the ending — the machine's own
//! record of why the ending was classified as it was, composed when it was
//! classified and never read back out of a stream. The line is E13's
//! observation of bytes in passing (E10), **carried as observed and never
//! mined for a cause**: whether it says what did the work is E12's judgement,
//! made where the entry is drawn and only on a *finished* ending. **N3's rule
//! binds both**, and the words themselves still stay in the sinks.
//!
//! **Until step 10c's fix piece 2 neither travelled**, so History read *Ran on
//! — not known* on every job on all four providers, and a failed job never
//! said why.
//!
//! # The answer the note is read in, and its one home
//!
//! **The note reading's answer is composed and read here, both directions in
//! one file** — [`TheNote::as_the_answers_fields`], which the machine's program
//! sends, and [`TheNote::read_from_the_answer`], which the desktop reads. Until
//! fix piece 2 the two halves were written apart, one in each program, joined
//! only by the field names; that is how the reason and the line could be
//! missing from the one and nobody notice at the other. **A round trip here is
//! the proof that what one end sends is what the other reads.**
//!
//! # N6's figure, and the one thing it is not
//!
//! **The figure is the last frame, in the job's own asked order as the
//! manifest records it, that counts as already there** — across attempts, and
//! whichever attempt asked for it. **That is not `Job files and outputs` C17's
//! last frame asked for**, which reads one attempt's own list; a re-run's list
//! is the complement of what was already done, and reading the figure off it
//! would narrow the figure to that complement. [`the_last_frame_figure`] takes
//! the manifest's own order and no list.
//!
//! **N7: where the ending's own step never ran there is no figure**, which is
//! [`None`] and never a zero.
//!
//! **N8: a video job has no figure at all** (D10), and the caller says so by
//! not asking for one — [`the_last_frame_figure`] is not reached for a
//! video-shaped job and [`WhyThereIsNoFigure`] names that case.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` S1, S14, M21, M22.
//! `Job files and outputs` M5. `State, identity and recovery` §1.2, §1.3.
//!
//! **Four of `Job files and outputs`' markers are deliberately not on that
//! list and are written out at every site instead** — `Job files and outputs`
//! C1, `Job files and outputs` C7, `Job files and outputs` C12 and
//! `Job files and outputs` C15 — because this plan defines a clause of each of
//! those names too.

// The store below is on a unix only, and it is the only thing here that
// names a path.
#[cfg(unix)]
use std::path::PathBuf;
// Named by the written form below, which is on a unix only.
#[cfg(unix)]
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
// The note reading's answer, composed and read here on every system — the
// machine sends it and the desktop reads it.
use smoozy_framing::framing::Fields;
use smoozy_framing::requests as asked;
// Named by the store and by its tests, both on a unix only.
#[cfg(unix)]
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::Reading;
#[cfg(unix)]
use smoozy_state::observed::{Clock, MachineClock};

#[cfg(unix)]
use crate::acting::queue_store::WhatTheStoreSaid;
use crate::ending::RecordedOutcome;

/// **N1's ending half: the outcome and its detail.**
///
/// **The exit status is carried verbatim and is never read as a cause**
/// (`Rented-server runtime` M22, and `Queue & History` E2's rule that the
/// number never names one). The capture reference is a pointer into the attempt's own record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheEndingOnTheNote {
    /// E2's four, as the note records them.
    pub outcome: TheOutcomeAsWritten,
    /// **`Rented-server runtime` M22: recorded, handed on, never a claim.**
    /// Absent where the group
    /// was gone with no reaped status, which is an ending with no status.
    pub exit_status: Option<i32>,
    /// **N3's pointer**: the attempt whose capture record holds the words.
    /// **The words are not here.**
    pub the_attempt_whose_capture_record_holds_the_words: u32,
    /// **The reason the machine recorded with the outcome** — `Rendering`
    /// K14's, in the words the machine wrote at the ending (`Queue & History`
    /// §10, the paragraph after N3). **Absent where the ending recorded none**,
    /// which is every ending but a failure, and on a note written before fix
    /// piece 2 — which reads back as no reason recorded, never as an empty one.
    #[serde(default)]
    pub the_reason_the_machine_recorded: Option<String>,
    /// **E13's device line, as the worker observed it at its own loop,
    /// verbatim** (`Queue & History` §10, the paragraph after N3). **Carried
    /// on any ending where it passed, and read on none but a finished one**:
    /// that is E12's, decided by `crate::ending::the_device_field` where the
    /// entry is drawn, and never here. Absent where no line passed.
    #[serde(default)]
    pub the_device_line_as_observed: Option<String>,
}

/// The recorded outcomes, in the note's written form.
///
/// **The mapping to an entry's ending is [`crate::ending::the_ending`]'s and
/// is not decided a second time here** — this type exists only because the
/// rules half's own [`RecordedOutcome`] is a judgement's shape rather than a
/// written one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheOutcomeAsWritten {
    /// **The render kind's own ending**, whose classification is `Rendering`'s
    /// to define (E2).
    Finished,
    Failed,
    /// **`Rented-server runtime` M21**, kept as its own value so the mapping
    /// keeps the rule rather than the caller.
    NoStatusWasReaped,
    LeftStopped,
    GivenUp,
}

impl TheOutcomeAsWritten {
    /// The entry's own reading of it.
    pub fn as_a_recorded_outcome(self) -> RecordedOutcome {
        use crate::ending::TheRenderKindsOwnEnding;
        match self {
            TheOutcomeAsWritten::LeftStopped => RecordedOutcome::LeftStopped,
            TheOutcomeAsWritten::GivenUp => RecordedOutcome::GivenUp,
            TheOutcomeAsWritten::Finished => {
                RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Finished)
            }
            TheOutcomeAsWritten::Failed => {
                RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::Failed)
            }
            TheOutcomeAsWritten::NoStatusWasReaped => {
                RecordedOutcome::TheRenderKindsOwn(TheRenderKindsOwnEnding::NoStatusWasReaped)
            }
        }
    }
}

/// **N1's attempt marks — interrupted and re-run, and from which frame the
/// re-run proceeded.**
///
/// *"From which frame the re-run proceeded, which is the first frame of the
/// re-run attempt's list."* **It is read off the list rather than computed**,
/// so a scattered list's first entry is the mark whatever its number is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnAttemptMark {
    pub attempt: u32,
    /// **Absent where the attempt's list was empty**, which is not a zero.
    pub the_re_run_proceeded_from: Option<i64>,
}

impl AnAttemptMark {
    /// **The mark for one re-run, from that attempt's own frame list.**
    pub fn for_a_re_run(attempt: u32, the_attempts_list: &[i64]) -> AnAttemptMark {
        AnAttemptMark {
            attempt,
            the_re_run_proceeded_from: the_attempts_list.first().copied(),
        }
    }
}

/// **The completeness statement the ending carries, or the stated reason
/// there is none** — `Rendering` K13, `Rendering` K19, `Rented-server
/// runtime` S11 and `Rented-server runtime` S13.
///
/// **An ending that carries no statement is not an ending that carries an
/// empty one**, which is the distinction H8 turns on, so the absence is a
/// variant with its reason rather than a default. **The statement is
/// `Job files and outputs` C27's four sets**, composed against the ending
/// step's one walk (`Job files and outputs` C7), and nothing here reads
/// inside it.
///
/// **Why it is on the note.** `Rendering` K19 makes it *"the statement
/// carried with the recorded outcome"*, and the note is the read surface an
/// entry's lasting facts come from (N1); the operation record, where the
/// outcome itself is written, is read by nothing on the desktop. Added at step
/// 10c piece 5's second sitting, which is when an ending first carried one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheStatementAtTheEnding {
    It(smoozy_job_files_machine::reckoning::TheCompletenessStatement),
    NoneWasTaken { why: String },
}

/// **`Rented-server runtime` V12's free-space reading, carried with the
/// recorded outcome it was taken for, or the stated reason none was taken.**
///
/// **Taken once, on the machine, against that job's own output folder, in the
/// act that first tries to write the render's recorded outcome**, and never
/// taken again: where that write is refused for want of room, the reading
/// rides the pending outcome unchanged to its landing (`Rented-server runtime`
/// S14), so a disk freed before the landing cannot silence the diagnosis.
///
/// **An ending carrying no reading is not one carrying a zero**
/// (`Rented-server runtime` V12, and `Server protocol and transfers` F6a item
/// 6), so the absence is a variant with its reason, as
/// [`TheStatementAtTheEnding`]'s is.
///
/// **Why it is on the note.** `Rented-server runtime` V12 has it reach a
/// reader *"by the route the ending does"*, and the note is the route the
/// desktop learns an ending by — the same ground the statement stands on. Added 2026-10-25, step 12's SC4
/// findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheFreeSpaceAtTheEnding {
    It(TheFreeSpaceReadAtTheEnding),
    NoneWasTaken { why: String },
}

/// **One reading, per path** — `Server protocol and transfers` F1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheFreeSpaceReadAtTheEnding {
    /// **`Server protocol and transfers` F3's available figure**, never the
    /// free figure.
    pub available_bytes: u64,
    /// **The exact folder it was taken against**, carried so that nothing
    /// downstream widens a per-path answer into a per-machine one
    /// (`Server protocol and transfers` F16).
    pub the_folder_it_was_taken_against: String,
    /// **The moment it was taken, on the machine's own clock** (`State,
    /// identity and recovery` D5): the act's first attempt, never the landing.
    pub taken_at: Reading<()>,
}

/// The note reading's names for `Rented-server runtime` V12's reading.
/// **Declared here, beside the one composer and the one reader**, as the rest
/// of the note's are read and composed in this file.
pub const THE_FREE_SPACE_AT_THE_ENDING_FIELD: &str = "free space at the ending";
pub const THE_FOLDER_THE_FREE_SPACE_WAS_READ_AGAINST_FIELD: &str = "free space at the ending, folder";
pub const WHEN_THE_FREE_SPACE_WAS_READ_FIELD: &str = "free space at the ending, taken at";
pub const THE_CLOCK_THE_FREE_SPACE_WAS_READ_ON_FIELD: &str = "free space at the ending, clock";
pub const WHY_NO_FREE_SPACE_AT_THE_ENDING_FIELD: &str = "why no free space at the ending";

impl TheFreeSpaceAtTheEnding {
    /// **As an answer carries it**: the figure, the folder and the moment with
    /// its clock — or the reason there is none, which is never a figure of
    /// nought.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        match self {
            TheFreeSpaceAtTheEnding::It(read) => {
                let (at, clock) = a_moment_as_the_answer_carries_it(&read.taken_at);
                vec![
                    (THE_FREE_SPACE_AT_THE_ENDING_FIELD, read.available_bytes.to_string()),
                    (
                        THE_FOLDER_THE_FREE_SPACE_WAS_READ_AGAINST_FIELD,
                        read.the_folder_it_was_taken_against.clone(),
                    ),
                    (WHEN_THE_FREE_SPACE_WAS_READ_FIELD, at),
                    (THE_CLOCK_THE_FREE_SPACE_WAS_READ_ON_FIELD, clock),
                ]
            }
            TheFreeSpaceAtTheEnding::NoneWasTaken { why } => {
                vec![(WHY_NO_FREE_SPACE_AT_THE_ENDING_FIELD, why.clone())]
            }
        }
    }

    /// **Read back off an answer**, or nothing where the answer carries
    /// neither — and **a figure with no folder or no moment is not a
    /// reading**, so it reads as none rather than as half of one.
    pub fn read_back<'a>(text: impl Fn(&str) -> Option<&'a str>) -> Option<TheFreeSpaceAtTheEnding> {
        if let Some(bytes) = text(THE_FREE_SPACE_AT_THE_ENDING_FIELD) {
            use smoozy_state::observed::{Clock, MachineClock};
            let seconds: u64 = text(WHEN_THE_FREE_SPACE_WAS_READ_FIELD)?.parse().ok()?;
            return Some(TheFreeSpaceAtTheEnding::It(TheFreeSpaceReadAtTheEnding {
                available_bytes: bytes.parse().ok()?,
                the_folder_it_was_taken_against: text(THE_FOLDER_THE_FREE_SPACE_WAS_READ_AGAINST_FIELD)?
                    .to_string(),
                taken_at: Reading::taken(
                    (),
                    std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds),
                    Clock::AMachine(MachineClock(
                        text(THE_CLOCK_THE_FREE_SPACE_WAS_READ_ON_FIELD)?.to_string(),
                    )),
                ),
            }));
        }
        text(WHY_NO_FREE_SPACE_AT_THE_ENDING_FIELD).map(|why| TheFreeSpaceAtTheEnding::NoneWasTaken {
            why: why.to_string(),
        })
    }

    /// The reading itself, where one was taken.
    pub fn the_reading(&self) -> Option<&TheFreeSpaceReadAtTheEnding> {
        match self {
            TheFreeSpaceAtTheEnding::It(read) => Some(read),
            TheFreeSpaceAtTheEnding::NoneWasTaken { .. } => None,
        }
    }
}

/// **The note's ending while it is owed to a full store, as the queue reading
/// carries it** — `Rented-server runtime` S14, *Readings stay truthful*, and
/// `Rented-server runtime` S14a. The supervisor holds the ending it could not
/// write; the queue reading gives the job as *ended* and carries, beside that
/// state, the outcome the supervisor classified, the mark that it is **owed to
/// the durable store rather than written to it**, and `Rented-server runtime`
/// V12's reading from the act's first attempt.
///
/// **A reading is never evidence that a record landed** (`Rented-server
/// runtime` S14a): the note keeps its null ending until the landing, and
/// nothing that turns on the write having landed reads this. The desktop draws
/// the job's ending from it until the landing replaces it with the record
/// (`Queue & History` C11: *the history entry takes the ending Smoozy learns —
/// from watching*). Added 2026-10-25, step 12's SC4 follow-up.
///
/// **Composed and read here, both directions in one file**, as the note
/// reading's own answer is: the machine's program sends it, and the desktop
/// and the phone's worker read it back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheEndingOwed {
    /// **The outcome the supervisor classified** (`Rented-server runtime`
    /// M20); its mapping to an entry's ending is
    /// [`crate::ending::the_ending`]'s, as for a written one.
    pub outcome: TheOutcomeAsWritten,
    /// **`Rented-server runtime` V12's reading, the first attempt's**, or none
    /// where the answer carried none.
    pub the_free_space_at_the_ending: Option<TheFreeSpaceAtTheEnding>,
}

/// The queue reading's names for an ending owed: the mark, and the outcome in
/// the note's own word.
pub const THE_OUTCOME_IS_OWED_TO_THE_DURABLE_STORE_FIELD: &str = "owed to the durable store";
pub const THE_PENDING_OUTCOME_FIELD: &str = "the pending outcome";

impl TheEndingOwed {
    /// **As the queue reading's item carries it, beside the job's *ended***:
    /// the outcome, the owed mark, and `Rented-server runtime` V12's reading
    /// where there is one.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![
            (THE_PENDING_OUTCOME_FIELD, self.outcome.the_answers_word().to_string()),
            (THE_OUTCOME_IS_OWED_TO_THE_DURABLE_STORE_FIELD, asked::THE_ANSWER_IS_YES.to_string()),
        ];
        if let Some(free) = &self.the_free_space_at_the_ending {
            pairs.extend(free.as_pairs());
        }
        pairs
    }

    /// **Read back off the item**, or nothing where the item is not marked
    /// owed or names an outcome this build does not know — an ending that
    /// landed carries no mark, and is the record's.
    pub fn read_back<'a>(text: impl Fn(&str) -> Option<&'a str> + Copy) -> Option<TheEndingOwed> {
        if text(THE_OUTCOME_IS_OWED_TO_THE_DURABLE_STORE_FIELD) != Some(asked::THE_ANSWER_IS_YES) {
            return None;
        }
        Some(TheEndingOwed {
            outcome: TheOutcomeAsWritten::read_from_the_answers_word(text(THE_PENDING_OUTCOME_FIELD)?)?,
            the_free_space_at_the_ending: TheFreeSpaceAtTheEnding::read_back(text),
        })
    }
}

/// **N1's note, whole.**
///
/// **It holds no observed reading of a machine other than the machine's own
/// two moments**, which are `State, identity and recovery` D5's readings on
/// that machine's own clock and carry the clock they were taken on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheNote {
    /// **The label, whole and authoritative** (Y8). The folder's bounded
    /// rendering is not here and is never re-derived from this.
    pub label: String,
    /// **The moment the render started, on the machine's clock**, written when
    /// the render operation's attempt starts.
    pub started: Option<Reading<()>>,
    /// **The moment it ended, on the same clock**, written when the recorded
    /// outcome is written.
    pub ended: Option<Reading<()>>,
    /// **N1: a null ending for a job that has not ended, never an invented
    /// one.**
    pub ending: Option<TheEndingOnTheNote>,
    /// **The attempt count and the re-run marks.**
    pub attempts: u32,
    pub re_run_marks: Vec<AnAttemptMark>,
    /// **Given up, where it was** — the figure at the moment the job was
    /// given up, which is the same figure N6 computes.
    pub given_up_where_it_was: Option<i64>,
    /// **N6's last-frame figure.**
    pub last_frame_figure: Option<i64>,
    /// **The completeness statement the ending carried**, written in the same
    /// act as the ending, or its stated absence. `None` until an ending is
    /// written — and on a note written before this field, which reads back as
    /// no statement recorded rather than as an empty one.
    pub the_completeness_statement: Option<TheStatementAtTheEnding>,
    /// **`Rented-server runtime` V12's reading, written in the same act as the
    /// ending**, or its stated absence. `None` until an ending is written, and
    /// on a note written before this field — which reads back as no reading
    /// recorded, never as a reading of nought.
    pub the_free_space_at_the_ending: Option<TheFreeSpaceAtTheEnding>,
}

impl TheNote {
    /// **N1: it is created in the same act as the job record (Q3), with the
    /// label and a null ending.**
    ///
    /// **There is no other constructor**, so a note with an ending on it
    /// cannot be made without an ending having been written.
    pub fn created_with_the_job_record(label: impl Into<String>) -> TheNote {
        TheNote {
            label: label.into(),
            started: None,
            ended: None,
            ending: None,
            attempts: 0,
            re_run_marks: Vec::new(),
            given_up_where_it_was: None,
            last_frame_figure: None,
            the_completeness_statement: None,
            the_free_space_at_the_ending: None,
        }
    }

    /// **What a message on the phone names this job by** — `Telegram` R7 and
    /// `Telegram` R7a: this note's label, whole; where it is empty or only spaces, the
    /// file's name, the last part of the `.blend`'s path the job's manifest
    /// holds (`Job files and outputs` M1). **The rule is
    /// [`crate::naming::what_a_job_is_called`]**, the one the desktop names the
    /// same job by, so a phone and a screen cannot call one job two things.
    ///
    /// **The note's label is never rewritten to the file's name**: it is the
    /// user's, and N1 and Y8 make it authoritative. The manifest is optional
    /// because a manifest that cannot be read must not cost a named job its
    /// name; an unnamed job then has none, which is what the desktop shows
    /// where it holds no path either.
    ///
    /// **Written for the role worker, which is not built** (Vast-pass fix
    /// piece 3, 2026-10-01): every value the phone's words take a job's name
    /// from is to be handed this, never the note's label alone.
    pub fn what_the_job_is_called(
        &self,
        manifest: Option<&smoozy_job_files_machine::manifest::SubmissionManifest>,
    ) -> String {
        crate::naming::what_a_job_is_called(
            Some(&self.label),
            manifest.map(|manifest| manifest.blend_path_on_the_machine.as_str()),
        )
    }

    /// **N1's note, as the note reading's answer carries it** — what the
    /// machine's program sends when it holds the note.
    ///
    /// **N1's null ending is the field's absence**, never an invented one; so
    /// are the reason and the device line where the ending recorded none
    /// (`Queue & History` §10, the paragraph after N3). **Both are sent
    /// verbatim**, and nothing here reads either.
    ///
    /// **`Rendering` K19's statement rides as four counts, or its stated
    /// absence**, which is never four noughts (`Rented-server runtime` S13);
    /// nothing is said of it while the ending is null. Moved here from the
    /// machine's program at step 10c's fix piece 2, unchanged but for the two
    /// fields.
    pub fn as_the_answers_fields(&self) -> Fields {
        let mut fields = Fields::none()
            .and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_PRESENT)
            .and(asked::THE_LABEL_FIELD, self.label.as_str())
            .and(asked::THE_ATTEMPTS_FIELD, self.attempts.to_string())
            .and(asked::HOW_MANY_RE_RUNS_FIELD, self.re_run_marks.len().to_string());
        if let Some(started) = &self.started {
            let (at, clock) = a_moment_as_the_answer_carries_it(started);
            fields = fields
                .and(asked::THE_STARTED_FIELD, at)
                .and(asked::THE_STARTED_CLOCK_FIELD, clock);
        }
        if let Some(ended) = &self.ended {
            let (at, clock) = a_moment_as_the_answer_carries_it(ended);
            fields = fields
                .and(asked::THE_ENDED_FIELD, at)
                .and(asked::THE_ENDED_CLOCK_FIELD, clock);
        }
        if let Some(ending) = &self.ending {
            fields = fields
                .and(asked::THE_ENDING_FIELD, ending.outcome.the_answers_word())
                .and(
                    asked::THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD,
                    ending
                        .the_attempt_whose_capture_record_holds_the_words
                        .to_string(),
                );
            if let Some(status) = ending.exit_status {
                fields = fields.and(asked::THE_EXIT_STATUS_FIELD, status.to_string());
            }
            if let Some(reason) = ending
                .the_reason_the_machine_recorded
                .as_deref()
                .filter(|reason| !reason.is_empty())
            {
                fields = fields.and(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD, reason);
            }
            if let Some(line) = ending
                .the_device_line_as_observed
                .as_deref()
                .filter(|line| !line.is_empty())
            {
                fields = fields.and(asked::THE_DEVICE_LINE_FIELD, line);
            }
        }
        for (at, mark) in self.re_run_marks.iter().enumerate() {
            fields = fields.and(
                &asked::the_field_of_item(asked::A_RE_RUN, at, asked::THE_ATTEMPT_FIELD),
                mark.attempt.to_string(),
            );
            if let Some(frame) = mark.the_re_run_proceeded_from {
                fields = fields.and(
                    &asked::the_field_of_item(
                        asked::A_RE_RUN,
                        at,
                        asked::THE_FRAME_IT_PROCEEDED_FROM_FIELD,
                    ),
                    frame.to_string(),
                );
            }
        }
        if let Some(frame) = self.given_up_where_it_was {
            fields = fields.and(asked::THE_FRAME_IT_WAS_GIVEN_UP_AT_FIELD, frame.to_string());
        }
        if let Some(frame) = self.last_frame_figure {
            fields = fields.and(asked::THE_LAST_FRAME_FIELD, frame.to_string());
        }
        match &self.the_completeness_statement {
            None => {}
            Some(TheStatementAtTheEnding::It(statement)) => {
                fields = fields
                    .and(asked::THE_FILES_EXPECTED_FIELD, statement.expected.to_string())
                    .and(asked::THE_FILES_PRESENT_FIELD, statement.present.len().to_string())
                    .and(asked::THE_FILES_ABSENT_FIELD, statement.absent.len().to_string())
                    .and(
                        asked::THE_FILES_PRESENT_AT_ZERO_BYTES_FIELD,
                        statement.present_at_zero_bytes.len().to_string(),
                    );
            }
            Some(TheStatementAtTheEnding::NoneWasTaken { why }) => {
                fields = fields.and(asked::WHY_NO_COMPLETENESS_STATEMENT_FIELD, why.as_str());
            }
        }
        // **`Rented-server runtime` V12's reading rides the ending to the
        // desktop**, or its stated absence; nothing while the ending is null.
        if let Some(free) = &self.the_free_space_at_the_ending {
            for (name, value) in free.as_pairs() {
                fields = fields.and(name, value);
            }
        }
        fields
    }

    /// **The note, read off the note reading's answer** — what the desktop
    /// reads once the machine has said it holds the note.
    ///
    /// **Nothing where the answer does not say the note is there, or cannot be
    /// read whole.** A note is never half-read: an ending whose outcome word
    /// this build does not know, or one with no pointer to its words, is a
    /// note this cannot read, and the fill-in is then not made at all rather
    /// than made with the ending left out.
    ///
    /// **What the answer does not carry, this does not invent**: the
    /// completeness statement rides as four counts and not as its sets, so it
    /// is not read back into [`TheNote::the_completeness_statement`], which
    /// stays absent here. Moved here from the desktop at step 10c's fix piece 2,
    /// unchanged but for the two fields of `Queue & History` §10's paragraph
    /// after N3.
    pub fn read_from_the_answer(fields: &Fields) -> Option<TheNote> {
        if fields.text(asked::ON_THIS_MACHINE_FIELD) != Some(asked::IT_IS_PRESENT) {
            return None;
        }
        let a_number = |name: &str| fields.text(name).and_then(|text| text.parse::<i64>().ok());
        let a_count = |name: &str| fields.text(name).and_then(|text| text.parse::<u32>().ok());
        let mut note =
            TheNote::created_with_the_job_record(fields.text(asked::THE_LABEL_FIELD).unwrap_or(""));
        note.started = a_moment_read_off_the_answer(
            fields,
            asked::THE_STARTED_FIELD,
            asked::THE_STARTED_CLOCK_FIELD,
        );
        note.ended =
            a_moment_read_off_the_answer(fields, asked::THE_ENDED_FIELD, asked::THE_ENDED_CLOCK_FIELD);
        note.ending = match fields.text(asked::THE_ENDING_FIELD) {
            None => None,
            Some(word) => Some(TheEndingOnTheNote {
                outcome: TheOutcomeAsWritten::read_from_the_answers_word(word)?,
                exit_status: fields
                    .text(asked::THE_EXIT_STATUS_FIELD)
                    .and_then(|text| text.parse::<i32>().ok()),
                the_attempt_whose_capture_record_holds_the_words: a_count(
                    asked::THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD,
                )?,
                the_reason_the_machine_recorded: fields
                    .text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD)
                    .filter(|reason| !reason.is_empty())
                    .map(str::to_string),
                the_device_line_as_observed: fields
                    .text(asked::THE_DEVICE_LINE_FIELD)
                    .filter(|line| !line.is_empty())
                    .map(str::to_string),
            }),
        };
        note.attempts = a_count(asked::THE_ATTEMPTS_FIELD).unwrap_or(0);
        let re_runs = fields
            .text(asked::HOW_MANY_RE_RUNS_FIELD)
            .and_then(|text| text.parse::<usize>().ok())
            .unwrap_or(0);
        for at in 0..re_runs {
            let of_the_item = |name: &str| {
                fields
                    .text(&asked::the_field_of_item(asked::A_RE_RUN, at, name))
                    .map(str::to_string)
            };
            note.re_run_marks.push(AnAttemptMark {
                attempt: of_the_item(asked::THE_ATTEMPT_FIELD)?.parse().ok()?,
                the_re_run_proceeded_from: of_the_item(asked::THE_FRAME_IT_PROCEEDED_FROM_FIELD)
                    .and_then(|text| text.parse().ok()),
            });
        }
        note.given_up_where_it_was = a_number(asked::THE_FRAME_IT_WAS_GIVEN_UP_AT_FIELD);
        note.last_frame_figure = a_number(asked::THE_LAST_FRAME_FIELD);
        note.the_free_space_at_the_ending = TheFreeSpaceAtTheEnding::read_back(|name| fields.text(name));
        Some(note)
    }
}

impl TheOutcomeAsWritten {
    /// The outcome's word on the note reading's answer — the note's own words
    /// in `smoozy_framing::requests`, one for each of the five.
    pub fn the_answers_word(self) -> &'static str {
        match self {
            TheOutcomeAsWritten::Finished => asked::IT_FINISHED,
            TheOutcomeAsWritten::Failed => asked::IT_FAILED,
            TheOutcomeAsWritten::NoStatusWasReaped => asked::NO_STATUS_WAS_REAPED,
            TheOutcomeAsWritten::LeftStopped => asked::IT_WAS_LEFT_STOPPED,
            TheOutcomeAsWritten::GivenUp => asked::IT_WAS_GIVEN_UP,
        }
    }

    /// The outcome an answer's word names, or nothing for a word this build
    /// does not know.
    pub fn read_from_the_answers_word(word: &str) -> Option<TheOutcomeAsWritten> {
        [
            TheOutcomeAsWritten::Finished,
            TheOutcomeAsWritten::Failed,
            TheOutcomeAsWritten::NoStatusWasReaped,
            TheOutcomeAsWritten::LeftStopped,
            TheOutcomeAsWritten::GivenUp,
        ]
        .into_iter()
        .find(|outcome| outcome.the_answers_word() == word)
    }
}

/// **A moment as the answer carries it**: whole seconds since 1970, and the
/// name of the clock it was taken on (`State, identity and recovery` D5).
fn a_moment_as_the_answer_carries_it(reading: &Reading<()>) -> (String, String) {
    use smoozy_state::observed::{Clock, MachineClock};
    let seconds = reading
        .taken_at()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or(0);
    let clock = match reading.clock() {
        Clock::AMachine(MachineClock(name)) => name.clone(),
        Clock::ThisComputer => "this computer".to_string(),
    };
    (seconds.to_string(), clock)
}

/// **A moment read off the answer, on the clock the answer names** — a
/// machine's own, which is the only clock a note's moment is ever on (D5).
fn a_moment_read_off_the_answer(fields: &Fields, at: &str, clock: &str) -> Option<Reading<()>> {
    use smoozy_state::observed::{Clock, MachineClock};
    let seconds: u64 = fields.text(at)?.parse().ok()?;
    let clock = fields.text(clock)?;
    Some(Reading::taken(
        (),
        std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds),
        Clock::AMachine(MachineClock(clock.to_string())),
    ))
}

/// **The note's written form, and it is Build's own choice** — the same shape
/// `Job files and outputs` settled for the manifest and the attempt record,
/// one JSON document per note.
///
/// **The two moments are written with the clock they were taken on**, because
/// a moment read back without its clock is a moment something could subtract
/// across clocks, which D5 forbids.
// The written form the store publishes, so it is on a unix only with it.
#[cfg(unix)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TheNoteAsWritten {
    label: String,
    started: Option<AMomentAsWritten>,
    ended: Option<AMomentAsWritten>,
    ending: Option<TheEndingOnTheNote>,
    attempts: u32,
    re_run_marks: Vec<AnAttemptMark>,
    given_up_where_it_was: Option<i64>,
    last_frame_figure: Option<i64>,
    #[serde(default)]
    the_completeness_statement: Option<TheStatementAtTheEnding>,
    #[serde(default)]
    the_free_space_at_the_ending: Option<TheFreeSpaceAsWritten>,
}

/// **`Rented-server runtime` V12's reading as the note writes it**, its moment with the clock it was
/// taken on, as the note's other two moments are.
#[cfg(unix)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum TheFreeSpaceAsWritten {
    It {
        available_bytes: u64,
        the_folder_it_was_taken_against: String,
        taken_at: AMomentAsWritten,
    },
    NoneWasTaken {
        why: String,
    },
}

#[cfg(unix)]
impl TheFreeSpaceAsWritten {
    fn of(free: &TheFreeSpaceAtTheEnding) -> Result<TheFreeSpaceAsWritten, WhatTheStoreSaid> {
        Ok(match free {
            TheFreeSpaceAtTheEnding::It(read) => TheFreeSpaceAsWritten::It {
                available_bytes: read.available_bytes,
                the_folder_it_was_taken_against: read.the_folder_it_was_taken_against.clone(),
                taken_at: AMomentAsWritten::of(&read.taken_at)?,
            },
            TheFreeSpaceAtTheEnding::NoneWasTaken { why } => {
                TheFreeSpaceAsWritten::NoneWasTaken { why: why.clone() }
            }
        })
    }

    fn read_back(self) -> TheFreeSpaceAtTheEnding {
        match self {
            TheFreeSpaceAsWritten::It {
                available_bytes,
                the_folder_it_was_taken_against,
                taken_at,
            } => TheFreeSpaceAtTheEnding::It(TheFreeSpaceReadAtTheEnding {
                available_bytes,
                the_folder_it_was_taken_against,
                taken_at: taken_at.read_back(),
            }),
            TheFreeSpaceAsWritten::NoneWasTaken { why } => TheFreeSpaceAtTheEnding::NoneWasTaken { why },
        }
    }
}

#[cfg(unix)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AMomentAsWritten {
    seconds_since_the_epoch: u64,
    /// The machine's own clock, named by the server row it was read from.
    /// **A note on a machine never carries the desktop's clock**, and a
    /// moment that reads back as the desktop's is not a moment of this note's.
    machine: String,
}

#[cfg(unix)]
impl AMomentAsWritten {
    /// **Q8's rule, kept here too: a write is refused loudly and nothing moves
    /// behind it.**
    ///
    /// **A moment that cannot be written is a refusal and never an absence.**
    /// Two things make one — a reading that is not on a machine clock, which
    /// a note on a machine never carries (D5), and a subtraction from 1970
    /// that fails. Dropped silently, either left an entry with **no start
    /// time, no duration and no reason**, which is the shape N7 reserves for
    /// an ending Smoozy never learned — so the user read one condition as
    /// another.
    fn of(reading: &Reading<()>) -> Result<AMomentAsWritten, WhatTheStoreSaid> {
        let Clock::AMachine(MachineClock(machine)) = reading.clock() else {
            return Err(WhatTheStoreSaid::ItWasRefused {
                why: "a moment on this note is not on a machine clock".to_string(),
            });
        };
        let since_the_epoch = reading
            .taken_at()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| WhatTheStoreSaid::ItWasRefused {
                why: "a moment on this note is before 1970 and cannot be written".to_string(),
            })?;
        Ok(AMomentAsWritten {
            seconds_since_the_epoch: since_the_epoch.as_secs(),
            machine: machine.clone(),
        })
    }

    fn read_back(self) -> Reading<()> {
        Reading::taken(
            (),
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(self.seconds_since_the_epoch),
            Clock::AMachine(MachineClock(self.machine)),
        )
    }
}

#[cfg(unix)]
impl TheNoteAsWritten {
    /// **A note whose moment cannot be written is not written at all.** The
    /// refusal is handed back and the note that stood before it still stands
    /// — which for an ending is the null ending, the truthful state.
    fn of(note: &TheNote) -> Result<TheNoteAsWritten, WhatTheStoreSaid> {
        Ok(TheNoteAsWritten {
            label: note.label.clone(),
            started: note
                .started
                .as_ref()
                .map(AMomentAsWritten::of)
                .transpose()?,
            ended: note.ended.as_ref().map(AMomentAsWritten::of).transpose()?,
            ending: note.ending.clone(),
            attempts: note.attempts,
            re_run_marks: note.re_run_marks.clone(),
            given_up_where_it_was: note.given_up_where_it_was,
            last_frame_figure: note.last_frame_figure,
            the_completeness_statement: note.the_completeness_statement.clone(),
            the_free_space_at_the_ending: note
                .the_free_space_at_the_ending
                .as_ref()
                .map(TheFreeSpaceAsWritten::of)
                .transpose()?,
        })
    }

    fn read_back(self) -> TheNote {
        TheNote {
            label: self.label,
            started: self.started.map(AMomentAsWritten::read_back),
            ended: self.ended.map(AMomentAsWritten::read_back),
            ending: self.ending,
            attempts: self.attempts,
            re_run_marks: self.re_run_marks,
            given_up_where_it_was: self.given_up_where_it_was,
            last_frame_figure: self.last_frame_figure,
            the_completeness_statement: self.the_completeness_statement,
            the_free_space_at_the_ending: self
                .the_free_space_at_the_ending
                .map(TheFreeSpaceAsWritten::read_back),
        }
    }
}

/// **N1's store: one durable record per job, in the package area, written by
/// the supervisor.**
///
/// **`State, identity and recovery` §1.2's own inventory row, owner
/// `Queue & History`** — held by the supervisor and by nothing else, which is
/// what `Rented-server runtime` S1 makes true of every durable store on the
/// machine.
///
/// **On a unix only, because it publishes through
/// `smoozy_runtime::acting::durable` and that crate gates its whole `acting`
/// module to a unix.** The inventory row above is `State, identity and
/// recovery` J11's, the stores on a rented server, and that document's §1.3
/// rule 6 says a rented machine is a Linux machine. [`crate::acting`] states
/// the arrangement once for all four of this crate's such stores. **The note
/// itself is not gated** — [`TheNote`] is what `smoozy-queue-history`'s `acting::adoption`
/// reads on the user's own computer.
#[cfg(unix)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePerJobNotesOnTheMachine {
    at: PathBuf,
}

/// What every published note's name ends in.
pub const THE_NOTES_SUFFIX: &str = ".note.json";

/// **Where the per-job notes sit under the package area**, and the name is
/// Build's own. N1 puts the store in the package area and names no segment.
/// Added at step 10c piece 5, when the supervisor first made the store.
pub const THE_AREA_THE_NOTES_SIT_IN: &str = "per-job-notes";

/// **Q3's one act: the per-job note and the job record, written by the
/// supervisor together** — after the job's material is published, and before
/// the event is journalled, which is the supervisor's own write and comes
/// after this returns.
///
/// # The note goes first, and the record's write is the one that decides
///
/// **Q3 makes the record's existence the fact**: *"A job is in the line
/// exactly when its job record exists."* So the record is written last, and
/// **a record never stands without its note** — which is what N1's *"created
/// in the same act as the job record"* has to mean on a machine with two files
/// and no transaction across them. **A record written without its note would
/// leave `State, identity and recovery` V1 step 4's fill-in with nothing to
/// read, and nothing anywhere would fail.**
///
/// **Where the record's write is refused, the note this act wrote is removed**,
/// so a refused act leaves neither behind, and the refusal is handed back in
/// the machine's own words (Q8). **Where even that removal is refused**, the
/// note stands with no record: by Q3 that is not a job, nothing reads a note to
/// decide what is in the line, and it is said rather than hidden — the caller
/// is handed the record's refusal, which is what decided.
///
/// **A process killed between the two writes** leaves the same thing, a note
/// with no record, and for the same reason it is not a job.
///
/// Added at step 10c piece 5.
#[cfg(unix)]
pub fn the_job_record_and_its_note_in_one_act(
    queue: &mut crate::acting::queue_store::TheQueueOnTheMachine,
    notes: &mut ThePerJobNotesOnTheMachine,
    record: &crate::line::JobRecord,
    label: &str,
) -> Result<(), WhatTheStoreSaid> {
    notes.create(&record.job, label)?;
    if let Err(why) = queue.write(record) {
        let _ = notes.remove(&record.job);
        return Err(why);
    }
    Ok(())
}

#[cfg(unix)]
impl ThePerJobNotesOnTheMachine {
    /// The store, at the directory the package area's own layout names.
    ///
    /// **The directory is not created here**, for the reason the queue store
    /// states: a store that made its own directory would make a missing
    /// package area look like a job with no note.
    pub fn at(directory: impl Into<PathBuf>) -> ThePerJobNotesOnTheMachine {
        ThePerJobNotesOnTheMachine {
            at: directory.into(),
        }
    }

    fn published(&self, job: &Identity) -> PathBuf {
        self.at.join(format!("{}{THE_NOTES_SUFFIX}", job.as_text()))
    }

    /// **Where one job's note sits**, for `Queue & History` L1's material
    /// cleanup, which removes it as its third unit. Added 2026-09-29, step
    /// 10c's first stop-and-clear piece, so the program names the note's path through the store that
    /// writes it rather than composing it a second time.
    pub fn where_one_sits(&self, job: &Identity) -> PathBuf {
        self.published(job)
    }

    /// **One write, published by the rename `Job files and outputs` M5 already
    /// uses**, so a note read while it was being written is never a note with
    /// a fact missing from it.
    ///
    /// **Every write is refused loudly and nothing moves behind it** (Q8, N5).
    /// The refusal is the machine's own words, and the note that stood before
    /// it still stands — which for an ending is the null ending, the truthful
    /// state.
    fn write(&mut self, job: &Identity, note: &TheNote) -> Result<(), WhatTheStoreSaid> {
        let written = serde_json::to_vec(&TheNoteAsWritten::of(note)?).map_err(|why| {
            WhatTheStoreSaid::ItWasRefused {
                why: why.to_string(),
            }
        })?;
        smoozy_runtime::acting::durable::publish(&self.published(job), &written)
            .map_err(|why| WhatTheStoreSaid::ItWasRefused { why: why.as_text() })
    }

    /// **N1: created in the same act as the job record, with the label and a
    /// null ending.**
    pub fn create(&mut self, job: &Identity, label: &str) -> Result<TheNote, WhatTheStoreSaid> {
        let note = TheNote::created_with_the_job_record(label);
        self.write(job, &note)?;
        Ok(note)
    }

    /// The note, or nothing where none stands.
    ///
    /// **A note that does not read back whole is not a note** and is reported
    /// as one rather than answered as a job with no facts.
    pub fn read(&self, job: &Identity) -> Result<Option<TheNote>, WhatTheStoreSaid> {
        let at = self.published(job);
        let Some(bytes) = smoozy_runtime::acting::durable::read(&at)
            .map_err(|why| WhatTheStoreSaid::ItWasRefused { why: why.as_text() })?
        else {
            return Ok(None);
        };
        match serde_json::from_slice::<TheNoteAsWritten>(&bytes) {
            Ok(written) => Ok(Some(written.read_back())),
            Err(_) => Err(WhatTheStoreSaid::ItIsNotAJobRecord {
                at: at.display().to_string(),
            }),
        }
    }

    /// **N2: the write that rides the attempt starting.** The started moment,
    /// on the machine's own clock, and the attempt count.
    pub fn the_attempt_started(
        &mut self,
        job: &Identity,
        attempt: u32,
        the_attempts_list: &[i64],
        at: Reading<()>,
    ) -> Result<TheNote, WhatTheStoreSaid> {
        let mut note = self.read(job)?.ok_or(WhatTheStoreSaid::ItIsNotAJobRecord {
            at: self.published(job).display().to_string(),
        })?;
        if note.started.is_none() {
            note.started = Some(at);
        }
        note.attempts = attempt;
        if attempt > 1 {
            note.re_run_marks
                .push(AnAttemptMark::for_a_re_run(attempt, the_attempts_list));
        }
        self.write(job, &note)?;
        Ok(note)
    }

    /// **N2: the write that rides the recorded outcome.** The ending, its
    /// moment, N6's figure where the ending's own step produced one, the
    /// completeness statement the ending carries or the stated reason it
    /// carries none, and `Rented-server runtime` V12's free-space reading or
    /// the stated reason it carries none.
    ///
    /// **The figure and the statement are handed in rather than computed
    /// here**, because both are the ending step's own walk — `Job files and
    /// outputs` C1's reckoning, taken once — and a second walk would be a
    /// second answer.
    pub fn the_outcome_was_written(
        &mut self,
        job: &Identity,
        ending: TheEndingOnTheNote,
        at: Reading<()>,
        figure: WhatTheFigureIs,
        statement: TheStatementAtTheEnding,
        free_space: TheFreeSpaceAtTheEnding,
    ) -> Result<TheNote, WhatTheStoreSaid> {
        let mut note = self.read(job)?.ok_or(WhatTheStoreSaid::ItIsNotAJobRecord {
            at: self.published(job).display().to_string(),
        })?;
        let given_up = ending.outcome == TheOutcomeAsWritten::GivenUp;
        note.ending = Some(ending);
        note.ended = Some(at);
        note.last_frame_figure = figure.the_figure();
        note.the_completeness_statement = Some(statement);
        note.the_free_space_at_the_ending = Some(free_space);
        if given_up {
            note.given_up_where_it_was = figure.the_figure();
        }
        self.write(job, &note)?;
        Ok(note)
    }

    /// **X2's unit: the note removed with the job's machine-side record.**
    /// **An already-absent note is *done*, never *failed***.
    pub fn remove(&mut self, job: &Identity) -> Result<(), WhatTheStoreSaid> {
        match std::fs::remove_file(self.published(job)) {
            Ok(()) => Ok(()),
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(why) => Err(WhatTheStoreSaid::ItWasRefused {
                why: why.to_string(),
            }),
        }
    }

    /// Every identity this store holds a note for.
    ///
    /// **Used by the erase and by the machine's own reading, never to decide
    /// what is in the line** — that is the queue record's, by Q3.
    pub fn every_job(&self) -> Result<Vec<Identity>, WhatTheStoreSaid> {
        let entries =
            std::fs::read_dir(&self.at).map_err(|why| WhatTheStoreSaid::ItWasRefused {
                why: format!("{}: {why}", self.at.display()),
            })?;
        let mut out = Vec::new();
        for entry in entries {
            let path = entry
                .map_err(|why| WhatTheStoreSaid::ItWasRefused {
                    why: why.to_string(),
                })?
                .path();
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            let Some(text) = name.strip_suffix(THE_NOTES_SUFFIX) else {
                continue;
            };
            if let Some(job) = Identity::read_back(IdentityKind::Job, text) {
                out.push(job);
            }
        }
        Ok(out)
    }
}

/// **N6, N7 and N8 in one answer, so a caller cannot get a figure without
/// saying which case it is in.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheFigureIs {
    /// **N6: the last frame, in the job's own asked order, that counts as
    /// already there.**
    It(i64),
    /// **N6: nothing in the job's own order counts as already there yet.**
    NoFrameCountsAsAlreadyThere,
    /// **N7: the ending's own step never ran, so there is no figure**, and the
    /// actions that read the figure are not offered.
    TheEndingsStepNeverRan,
    /// **N8 and D10: a video job has no figure at all.**
    ThisIsAVideoJob,
}

// Read where a note is written, which is on a unix only; its own test
// exercises it on every system.
#[cfg_attr(not(unix), allow(dead_code))]
impl WhatTheFigureIs {
    fn the_figure(self) -> Option<i64> {
        match self {
            WhatTheFigureIs::It(frame) => Some(frame),
            _ => None,
        }
    }
}

/// **Why an entry shows no figure, stated rather than left blank.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyThereIsNoFigure {
    NoFrameCountsAsAlreadyThere,
    TheEndingsStepNeverRan,
    ThisIsAVideoJob,
}

/// **N6, computed at the job's ending, on the machine, by the ending's own
/// step.**
///
/// **The order is the job's own asked order as the manifest records it, and
/// the walk is the ending step's one walk** — `Job files and outputs` C7's,
/// already taken for the completeness statement (H7). **`Job files and
/// outputs` C12's already-there test is handed in** rather than repeated, so
/// this function reads no file and knows nothing about a name.
///
/// **It is across attempts and whichever attempt asked for the frame**, which
/// is why it takes the manifest's order and never an attempt's list — *"so a
/// re-run's own frame list never narrows the figure to that list's
/// complement."*
///
/// **`Job files and outputs` C17's consequence is kept explicit**: the last
/// frame asked for on the previous attempt is re-rendered and is short or
/// absent while it is, so it may drop out of the figure. **That is the rule
/// working rather than a loss**, and nothing here holds the figure up against
/// it.
///
/// **`Job files and outputs` C15's limit rides the figure rather than hiding
/// under it**: a file short but not empty still counts, so the figure can
/// vouch for a frame with a hole in it. **No size floor and no trailing-byte
/// check is built for that** — it is the limit stated, and N8 names the two
/// signals where it bites.
pub fn the_last_frame_figure(
    the_jobs_own_asked_order: &[i64],
    already_there: impl Fn(i64) -> bool,
) -> WhatTheFigureIs {
    match the_jobs_own_asked_order
        .iter()
        .rev()
        .copied()
        .find(|frame| already_there(*frame))
    {
        Some(frame) => WhatTheFigureIs::It(frame),
        None => WhatTheFigureIs::NoFrameCountsAsAlreadyThere,
    }
}

/// **N9, stated as a value: what the render's file read produces reaches the
/// entry by the two standing routes and no third one.**
///
/// The recorded reading is composed on the machine, journalled as an event
/// when written, and sits in the job record directory for §7 step 4's fill-in.
/// **It is `Job files and outputs`' store, not a second one of this
/// document's**, and nothing here copies a field of it into the note.
pub const THE_RECORDED_READING_IS_NOT_COPIED_INTO_THE_NOTE: bool = true;

/// **N5, stated as a value: nothing here holds a guessed ending while a write
/// is refused.**
///
/// The refused write leaves the null ending standing. What could not be
/// written is held pending and re-attempted until it lands under
/// `Rented-server runtime` S14 — **that document's machinery and not a second
/// one here.**
pub const A_REFUSED_WRITE_LEAVES_THE_NULL_ENDING_STANDING: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    #[cfg(unix)]
    fn a_root(named: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("smoozy-queue-note-{}-{named}", std::process::id()));
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(&root).expect("the place is made");
        root
    }

    #[cfg(unix)]
    fn on_the_machine(seconds: u64) -> Reading<()> {
        Reading::taken(
            (),
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds),
            Clock::AMachine(MachineClock("server-1".to_string())),
        )
    }

    /// **N1: created with the label and a null ending, and the ending stays
    /// null until an outcome is written.**
    #[cfg(unix)]
    #[test]
    fn a_new_note_carries_the_label_and_a_null_ending() {
        let root = a_root("new");
        let mut notes = ThePerJobNotesOnTheMachine::at(&root);
        let job = a_job();
        let note = notes.create(&job, "Shot 12 — hero pass").expect("written");
        assert_eq!(note.label, "Shot 12 — hero pass");
        assert_eq!(note.ending, None, "never an invented ending");
        assert_eq!(note.started, None);
        assert_eq!(notes.read(&job).expect("read"), Some(note));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **Q3's one act: the note and the record together, the record last, and
    /// a refusal of either leaves neither.**
    ///
    /// **Each half is broken on purpose**: a note store that refuses leaves no
    /// record, and a queue that refuses takes the note back out. **A record
    /// standing without its note is the failure this act exists to prevent**,
    /// because nothing anywhere would notice it.
    #[cfg(unix)]
    #[test]
    fn the_record_and_its_note_are_one_act_and_neither_stands_alone() {
        use crate::acting::queue_store::TheQueueOnTheMachine;
        use crate::line::JobRecord;

        let root = a_root("one-act");
        std::fs::create_dir_all(root.join("queue")).unwrap();
        std::fs::create_dir_all(root.join("notes")).unwrap();
        let job = a_job();
        let record = JobRecord::submitted(job.clone(), None, "/r".to_string(), &[]);

        // **The note refused: no record is written.**
        let mut queue = TheQueueOnTheMachine::at(root.join("queue"));
        let mut no_notes = ThePerJobNotesOnTheMachine::at(root.join("no notes here"));
        assert!(the_job_record_and_its_note_in_one_act(&mut queue, &mut no_notes, &record, "a")
            .is_err());
        assert_eq!(queue.read_one(&job), Ok(None), "no record without its note");

        // **The record refused: the note this act wrote is taken back.**
        let mut no_queue = TheQueueOnTheMachine::at(root.join("no queue here"));
        let mut notes = ThePerJobNotesOnTheMachine::at(root.join("notes"));
        assert!(the_job_record_and_its_note_in_one_act(&mut no_queue, &mut notes, &record, "a")
            .is_err());
        assert_eq!(notes.read(&job), Ok(None), "no note left from a refused act");

        // **Both there: both written, the note with the label and a null
        // ending.**
        the_job_record_and_its_note_in_one_act(&mut queue, &mut notes, &record, "Shot 12")
            .expect("written");
        assert!(queue.read_one(&job).expect("read").is_some());
        let note = notes.read(&job).expect("read").expect("there");
        assert_eq!(note.label, "Shot 12");
        assert_eq!(note.ending, None);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **N2 and N4: the started moment rides the attempt starting, on the
    /// machine's own clock, and the duration comes out of the two readings.**
    #[cfg(unix)]
    #[test]
    fn the_two_moments_are_the_machines_own_and_the_duration_is_theirs() {
        let root = a_root("moments");
        let mut notes = ThePerJobNotesOnTheMachine::at(&root);
        let job = a_job();
        notes.create(&job, "a label").expect("written");
        notes
            .the_attempt_started(&job, 1, &[1, 2, 3], on_the_machine(100))
            .expect("written");
        let note = notes
            .the_outcome_was_written(
                &job,
                TheEndingOnTheNote {
                    outcome: TheOutcomeAsWritten::Finished,
                    exit_status: Some(0),
                    the_attempt_whose_capture_record_holds_the_words: 1,
                    the_reason_the_machine_recorded: None,
                    the_device_line_as_observed: None,
                },
                on_the_machine(160),
                WhatTheFigureIs::It(3),
                TheStatementAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
                TheFreeSpaceAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
            )
            .expect("written");
        let started = note.started.expect("there");
        let ended = note.ended.expect("there");
        assert_eq!(ended.since(&started).map(|d| d.as_secs()), Some(60));
        assert!(matches!(started.clock(), Clock::AMachine(_)));
        assert_eq!(note.last_frame_figure, Some(3));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **Q8: a moment that cannot be written is refused loudly and nothing
    /// moves behind it.**
    ///
    /// **The fault this catches is a silently dropped moment.** A reading on
    /// the desktop's clock, or one before 1970, used to write a note with the
    /// moment simply absent — leaving an entry with no start time, no duration
    /// and no reason, which is the shape N7 reserves for an ending Smoozy
    /// never learned. The user read one condition as another, and nothing
    /// anywhere said so.
    #[cfg(unix)]
    #[test]
    fn a_moment_that_cannot_be_written_refuses_the_note_rather_than_going_missing() {
        let root = a_root("refused-moment");
        let mut notes = ThePerJobNotesOnTheMachine::at(&root);
        let job = a_job();
        notes.create(&job, "a label").expect("written");
        let on_the_desktop = Reading::taken(
            (),
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(100),
            Clock::ThisComputer,
        );
        assert!(
            notes
                .the_attempt_started(&job, 1, &[1, 2, 3], on_the_desktop)
                .is_err(),
            "a moment that is not on a machine clock refuses the write"
        );
        let standing = notes.read(&job).expect("read").expect("there");
        assert_eq!(standing.started, None, "and nothing moved behind it");
        assert_eq!(standing.ending, None, "the null ending still stands");

        let before_1970 = Reading::taken(
            (),
            SystemTime::UNIX_EPOCH - std::time::Duration::from_secs(1),
            Clock::AMachine(MachineClock("server-1".to_string())),
        );
        assert!(notes
            .the_attempt_started(&job, 1, &[1, 2, 3], before_1970)
            .is_err());
        std::fs::remove_dir_all(&root).ok();
    }

    /// **N1: the re-run mark says which frame the re-run proceeded from, and
    /// it is the first frame of that attempt's own list.**
    #[cfg(unix)]
    #[test]
    fn a_re_run_marks_the_frame_it_proceeded_from_off_the_attempts_own_list() {
        let root = a_root("rerun");
        let mut notes = ThePerJobNotesOnTheMachine::at(&root);
        let job = a_job();
        notes.create(&job, "a label").expect("written");
        notes
            .the_attempt_started(&job, 1, &[1, 2, 3, 4], on_the_machine(10))
            .expect("written");
        let note = notes
            .the_attempt_started(&job, 2, &[3, 4], on_the_machine(20))
            .expect("written");
        assert_eq!(note.attempts, 2);
        assert_eq!(
            note.re_run_marks,
            vec![AnAttemptMark {
                attempt: 2,
                the_re_run_proceeded_from: Some(3)
            }]
        );
        // **The started moment is the first attempt's and does not move.**
        assert_eq!(note.started, Some(on_the_machine(10)));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **N6: the figure is read in the job's own asked order, across
    /// attempts** — never narrowed to a re-run's own list.
    #[test]
    fn the_figure_reads_the_jobs_own_order_and_is_not_narrowed_by_a_re_runs_list() {
        // The job asked for 1 to 6. Frames 1, 2, 3 and 6 count as already
        // there; the second attempt's own list was [4, 5, 6] alone.
        let order = [1, 2, 3, 4, 5, 6];
        let there = |frame: i64| matches!(frame, 1 | 2 | 3 | 6);
        assert_eq!(the_last_frame_figure(&order, there), WhatTheFigureIs::It(6));
        // Reading it off the re-run's complement would answer 3, which is the
        // fault this clause names.
        let complement = [1, 2, 3];
        assert_eq!(
            the_last_frame_figure(&complement, there),
            WhatTheFigureIs::It(3),
            "the complement gives the narrower and wrong answer"
        );
    }

    /// **N6 and N7: no frame already there is not the same as no ending
    /// step**, and neither is a zero.
    #[test]
    fn nothing_already_there_and_no_ending_step_are_two_different_answers() {
        assert_eq!(
            the_last_frame_figure(&[1, 2, 3], |_| false),
            WhatTheFigureIs::NoFrameCountsAsAlreadyThere
        );
        assert_eq!(WhatTheFigureIs::TheEndingsStepNeverRan.the_figure(), None);
        assert_eq!(WhatTheFigureIs::ThisIsAVideoJob.the_figure(), None);
        assert_eq!(
            WhatTheFigureIs::NoFrameCountsAsAlreadyThere.the_figure(),
            None
        );
    }

    /// **N1 and N8: a video job's note carries no figure at all**, and *given
    /// up* records where it was by the same figure.
    #[cfg(unix)]
    #[test]
    fn a_video_jobs_note_has_no_figure_and_given_up_records_where_it_was() {
        let root = a_root("video");
        let mut notes = ThePerJobNotesOnTheMachine::at(&root);
        let video = a_job();
        notes.create(&video, "a video job").expect("written");
        let note = notes
            .the_outcome_was_written(
                &video,
                TheEndingOnTheNote {
                    outcome: TheOutcomeAsWritten::GivenUp,
                    exit_status: None,
                    the_attempt_whose_capture_record_holds_the_words: 2,
                    the_reason_the_machine_recorded: None,
                    the_device_line_as_observed: None,
                },
                on_the_machine(5),
                WhatTheFigureIs::ThisIsAVideoJob,
                TheStatementAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
                TheFreeSpaceAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
            )
            .expect("written");
        assert_eq!(note.last_frame_figure, None);
        assert_eq!(note.given_up_where_it_was, None);

        let frames = a_job();
        notes.create(&frames, "a frame job").expect("written");
        let note = notes
            .the_outcome_was_written(
                &frames,
                TheEndingOnTheNote {
                    outcome: TheOutcomeAsWritten::GivenUp,
                    exit_status: None,
                    the_attempt_whose_capture_record_holds_the_words: 2,
                    the_reason_the_machine_recorded: None,
                    the_device_line_as_observed: None,
                },
                on_the_machine(5),
                WhatTheFigureIs::It(11),
                TheStatementAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
                TheFreeSpaceAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
            )
            .expect("written");
        assert_eq!(note.given_up_where_it_was, Some(11));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **N5 and Q8: a refused write leaves the note that stood, and for an
    /// ending that is the null ending — the truthful state.**
    ///
    /// **The fault is introduced the way Fact 40 met it: the reads go on
    /// answering and the writes refuse.** The store's directory is made
    /// unwritable while it stays readable, which is what a full state mount
    /// looks like from here.
    #[cfg(unix)]
    #[test]
    fn a_refused_write_leaves_the_null_ending_standing_and_reads_still_answer() {
        use std::os::unix::fs::PermissionsExt;
        let root = a_root("refused");
        let mut notes = ThePerJobNotesOnTheMachine::at(&root);
        let job = a_job();
        notes.create(&job, "a label").expect("written");
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o500))
            .expect("the writes are refused from here on");
        let refused = notes
            .the_outcome_was_written(
                &job,
                TheEndingOnTheNote {
                    outcome: TheOutcomeAsWritten::LeftStopped,
                    exit_status: None,
                    the_attempt_whose_capture_record_holds_the_words: 1,
                    the_reason_the_machine_recorded: None,
                    the_device_line_as_observed: None,
                },
                on_the_machine(9),
                WhatTheFigureIs::It(1),
                TheStatementAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
                TheFreeSpaceAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
            )
            .expect_err("refused");
        assert!(matches!(refused, WhatTheStoreSaid::ItWasRefused { .. }));
        // **N5's own half: the read answers truthfully, and the ending it
        // answers with is still null.**
        let standing = notes.read(&job).expect("read").expect("there");
        assert_eq!(standing.ending, None);
        assert_eq!(standing.ended, None);
        const _: () = assert!(A_REFUSED_WRITE_LEAVES_THE_NULL_ENDING_STANDING);
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).ok();
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A note that does not read back whole is reported and never answered
    /// as a job with no facts.**
    #[cfg(unix)]
    #[test]
    fn something_that_is_not_a_note_is_reported_rather_than_read_as_an_empty_one() {
        let root = a_root("stranger");
        let notes = ThePerJobNotesOnTheMachine::at(&root);
        let job = a_job();
        std::fs::write(
            root.join(format!("{}{THE_NOTES_SUFFIX}", job.as_text())),
            b"not a note",
        )
        .unwrap();
        assert!(matches!(
            notes.read(&job),
            Err(WhatTheStoreSaid::ItIsNotAJobRecord { .. })
        ));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **N3: the note points at the attempt's capture record and holds no
    /// word of the machine's.** The guard is a scan of this file's working
    /// half, comments taken off, for a field that would hold one.
    #[test]
    fn the_note_holds_a_pointer_and_never_the_words() {
        let source = include_str!("note.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let code: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in ["stderr", "stdout", "sink", "complaints", "captured_text"] {
            assert!(
                !code.contains(forbidden),
                "{forbidden} would put a stream's words in the note, which N3 forbids"
            );
        }
        const _: () = assert!(THE_RECORDED_READING_IS_NOT_COPIED_INTO_THE_NOTE);
    }

    /// A note with a started and an ended moment and the given ending, as the
    /// supervisor's two writes leave it.
    fn a_note_that_ended(ending: TheEndingOnTheNote) -> TheNote {
        let on_a_machine = |seconds| {
            Reading::taken(
                (),
                std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds),
                smoozy_state::observed::Clock::AMachine(smoozy_state::observed::MachineClock(
                    "server-1".to_string(),
                )),
            )
        };
        TheNote {
            started: Some(on_a_machine(100)),
            ended: Some(on_a_machine(160)),
            ending: Some(ending),
            attempts: 2,
            re_run_marks: vec![AnAttemptMark {
                attempt: 2,
                the_re_run_proceeded_from: Some(3),
            }],
            last_frame_figure: Some(3),
            ..TheNote::created_with_the_job_record("Kitchen, first pass")
        }
    }

    /// **`Queue & History` §10, the paragraph after N3, both ends at once: what
    /// the machine's program sends is what the desktop reads** — the reason
    /// the machine recorded on a failed ending, and the device line on a
    /// finished one, each verbatim.
    ///
    /// **The fault this catches is the one the seventh passes met**: the
    /// answer carried the outcome, the times, the exit status and the attempt
    /// and nothing else, so History read *Ran on — not known* on every job and
    /// a failed job never said why. Composing and reading in one file is what
    /// makes the round trip testable at all.
    #[test]
    fn the_answer_carries_the_reason_and_the_device_line_and_reads_back_as_them() {
        let reason = "Smoozy does not know how to render on a NVIDIA GeForce GTX 1650";
        let failed = a_note_that_ended(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Failed,
            exit_status: Some(3),
            the_attempt_whose_capture_record_holds_the_words: 2,
            the_reason_the_machine_recorded: Some(reason.to_string()),
            the_device_line_as_observed: None,
        });
        let answered = failed.as_the_answers_fields();
        assert_eq!(
            answered.text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD),
            Some(reason),
            "the reason travels, in the machine's own words"
        );
        assert_eq!(answered.text(asked::THE_DEVICE_LINE_FIELD), None);
        let read = TheNote::read_from_the_answer(&answered).expect("the note reads back");
        assert_eq!(read.ending, failed.ending, "the ending reads back whole");
        assert_eq!(read.started, failed.started);
        assert_eq!(read.ended, failed.ended);
        assert_eq!(read.re_run_marks, failed.re_run_marks);
        assert_eq!(read.last_frame_figure, Some(3));

        let line = "Device: NVIDIA GeForce RTX 4090 (CUDA)";
        let finished = a_note_that_ended(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Finished,
            exit_status: Some(0),
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            the_device_line_as_observed: Some(line.to_string()),
        });
        let answered = finished.as_the_answers_fields();
        assert_eq!(answered.text(asked::THE_DEVICE_LINE_FIELD), Some(line));
        assert_eq!(answered.text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD), None);
        let read = TheNote::read_from_the_answer(&answered).expect("the note reads back");
        assert_eq!(read.ending, finished.ending);
    }

    /// **N1's null ending travels as an absence, and so do the two things the
    /// ending carries** — a note whose job has not ended sends neither, and an
    /// empty value is never sent as one.
    #[test]
    fn a_note_with_no_ending_sends_no_reason_and_no_line_and_an_empty_one_is_none() {
        let waiting = TheNote::created_with_the_job_record("a label");
        let answered = waiting.as_the_answers_fields();
        assert_eq!(answered.text(asked::THE_ENDING_FIELD), None);
        assert_eq!(answered.text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD), None);
        assert_eq!(answered.text(asked::THE_DEVICE_LINE_FIELD), None);
        assert_eq!(
            TheNote::read_from_the_answer(&answered).expect("it reads").ending,
            None
        );

        let empty = a_note_that_ended(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Failed,
            exit_status: None,
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: Some(String::new()),
            the_device_line_as_observed: Some(String::new()),
        });
        let answered = empty.as_the_answers_fields();
        assert_eq!(answered.text(asked::THE_REASON_THE_MACHINE_RECORDED_FIELD), None);
        assert_eq!(answered.text(asked::THE_DEVICE_LINE_FIELD), None);
    }

    /// **The desktop reads nothing it was not sent**: an answer that does not
    /// say the note is there, or an ending word this build does not know, is
    /// no note at all rather than half of one.
    #[test]
    fn an_answer_that_is_not_a_whole_note_reads_as_none() {
        assert_eq!(
            TheNote::read_from_the_answer(
                &Fields::none().and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_ABSENT)
            ),
            None
        );
        let unknown = Fields::none()
            .and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_PRESENT)
            .and(asked::THE_ENDING_FIELD, "a word from a later build")
            .and(asked::THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD, "1");
        assert_eq!(TheNote::read_from_the_answer(&unknown), None);
        let no_pointer = Fields::none()
            .and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_PRESENT)
            .and(asked::THE_ENDING_FIELD, asked::IT_FAILED);
        assert_eq!(TheNote::read_from_the_answer(&no_pointer), None);
    }

    /// **A note written before step 10c's fix piece 2 still reads back**, with
    /// no reason and no line recorded — never an empty one, and never a note
    /// that fails to read.
    #[cfg(unix)]
    #[test]
    fn a_note_written_before_the_two_fields_reads_back_with_neither() {
        let root = a_root("before-the-two");
        let notes = ThePerJobNotesOnTheMachine::at(&root);
        let job = a_job();
        std::fs::write(
            root.join(format!("{}{THE_NOTES_SUFFIX}", job.as_text())),
            br#"{"label":"old","started":null,"ended":null,"ending":{"outcome":"Failed","exit_status":3,"the_attempt_whose_capture_record_holds_the_words":1},"attempts":1,"re_run_marks":[],"given_up_where_it_was":null,"last_frame_figure":null}"#,
        )
        .unwrap();
        let read = notes.read(&job).expect("it reads").expect("it is there");
        let ending = read.ending.expect("the ending");
        assert_eq!(ending.the_reason_the_machine_recorded, None);
        assert_eq!(ending.the_device_line_as_observed, None);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **N2: the two ride the recorded outcome's own write** and are on the
    /// store afterwards, verbatim.
    #[cfg(unix)]
    #[test]
    fn the_outcomes_write_keeps_the_reason_and_the_line_on_the_store() {
        let root = a_root("the-two-kept");
        let mut notes = ThePerJobNotesOnTheMachine::at(&root);
        let job = a_job();
        notes.create(&job, "a label").expect("written");
        let ending = TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Failed,
            exit_status: Some(1),
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: Some("the render ended with status 1".to_string()),
            the_device_line_as_observed: Some("Device: CPU".to_string()),
        };
        notes
            .the_outcome_was_written(
                &job,
                ending.clone(),
                on_the_machine(9),
                WhatTheFigureIs::NoFrameCountsAsAlreadyThere,
                TheStatementAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
                TheFreeSpaceAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
            )
            .expect("written");
        assert_eq!(
            notes.read(&job).expect("read").expect("there").ending,
            Some(ending)
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **`Rented-server runtime` V12, both ends at once: the reading the
    /// supervisor took rides the ending onto the store and onto the answer,
    /// and reads back as it was taken** — figure, folder, moment and clock.
    /// Before step 12's SC4 findings no note carried one, so the desktop had
    /// nothing to name a full disk from.
    #[cfg(unix)]
    #[test]
    fn the_free_space_at_the_ending_rides_the_note_and_the_answer_unchanged() {
        let root = a_root("free-space");
        let mut notes = ThePerJobNotesOnTheMachine::at(&root);
        let job = a_job();
        notes.create(&job, "a label").expect("written");
        let reading = TheFreeSpaceAtTheEnding::It(TheFreeSpaceReadAtTheEnding {
            available_bytes: 0,
            the_folder_it_was_taken_against: "/root/smoozy/jobs/shot-40/output".to_string(),
            taken_at: on_the_machine(1_700_000_000),
        });
        notes
            .the_outcome_was_written(
                &job,
                TheEndingOnTheNote {
                    outcome: TheOutcomeAsWritten::Failed,
                    exit_status: Some(28),
                    the_attempt_whose_capture_record_holds_the_words: 1,
                    the_reason_the_machine_recorded: None,
                    the_device_line_as_observed: None,
                },
                on_the_machine(1_700_000_001),
                WhatTheFigureIs::It(40),
                TheStatementAtTheEnding::NoneWasTaken {
                    why: "a test's ending".to_string(),
                },
                reading.clone(),
            )
            .expect("written");
        let stored = notes.read(&job).expect("read").expect("there");
        assert_eq!(stored.the_free_space_at_the_ending, Some(reading.clone()));
        let answered = stored.as_the_answers_fields();
        assert_eq!(answered.text(THE_FREE_SPACE_AT_THE_ENDING_FIELD), Some("0"));
        let read = TheNote::read_from_the_answer(&answered).expect("the note reads back");
        assert_eq!(read.the_free_space_at_the_ending, Some(reading));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **`Rented-server runtime` V12: an ending carrying no reading is not one
    /// carrying a zero** — the
    /// absence travels with its reason and reads back as an absence; a note
    /// with no ending, or written before the field, carries none at all.
    #[test]
    fn no_free_space_reading_is_never_a_reading_of_nought() {
        let mut note = TheNote::created_with_the_job_record("a label");
        let answered = note.as_the_answers_fields();
        assert_eq!(answered.text(THE_FREE_SPACE_AT_THE_ENDING_FIELD), None);
        assert_eq!(TheNote::read_from_the_answer(&answered).unwrap().the_free_space_at_the_ending, None);
        note.the_free_space_at_the_ending = Some(TheFreeSpaceAtTheEnding::NoneWasTaken {
            why: "the job's output folder is gone".to_string(),
        });
        let answered = note.as_the_answers_fields();
        assert_eq!(answered.text(THE_FREE_SPACE_AT_THE_ENDING_FIELD), None);
        assert_eq!(
            TheNote::read_from_the_answer(&answered).unwrap().the_free_space_at_the_ending,
            note.the_free_space_at_the_ending
        );
        // **A figure with no folder is no reading.**
        let half = Fields::none().and(THE_FREE_SPACE_AT_THE_ENDING_FIELD, "0");
        assert_eq!(TheFreeSpaceAtTheEnding::read_back(|name| half.text(name)), None);
    }

    /// **X2's unit: an already-absent note is done rather than failed.**
    #[cfg(unix)]
    #[test]
    fn removing_a_note_twice_is_done() {
        let root = a_root("remove");
        let mut notes = ThePerJobNotesOnTheMachine::at(&root);
        let job = a_job();
        notes.create(&job, "a label").expect("written");
        assert_eq!(notes.every_job().expect("read"), vec![job.clone()]);
        notes.remove(&job).expect("removed");
        notes.remove(&job).expect("an absent note is done");
        assert_eq!(notes.read(&job).expect("read"), None);
        assert!(notes.every_job().expect("read").is_empty());
        std::fs::remove_dir_all(&root).ok();
    }
}

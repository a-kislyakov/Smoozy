//! **`Queue & History` E9's kept words, in the local record** — the table
//! record version 34 made, out of the Creator's notes on 0.1.8, piece 6d
//! (2026-10-03).
//!
//! **The rule is `Queue & History`'s and the store is this document's**, the
//! split every row module here draws. E9: *"on a connection that learns a job's
//! ending, the desktop fetches both captured streams and the per-stream
//! collection outcomes into the job's history record, whole or not at all
//! (M11), kept apart, unedited."* **What is here is the read and the write and
//! nothing else**: the fetch is `smoozy_queue_history::acting::words::fetch_the_words`,
//! driven by `src/servers/the_words_kept.rs`, and the panel is
//! `src/presentation/the_machines_own_words.rs`'s.
//!
//! **Kept apart**: one row per stream per attempt, never a joined record
//! (`Rented-server runtime` M7 and M19). **Unedited**: the bytes land as they
//! arrived and a row that holds an outcome is never written again; the one
//! row that may be written over is one that holds no outcome yet
//! (`Desktop app and delivery` M10a), which holds no words either.
//!
//! **The entry's lifetime is the words' lifetime**: the table is one of
//! [`super::job_records::EVERY_SET_BESIDE_THE_ROW`], so a deletion takes them
//! with the entry (`Queue & History` X1) and a migration may never empty it
//! (L12a). Nothing else expires them — E9's own cost note.

use smoozy_runtime::capture::CollectionOutcome as TheRuntimesOutcome;
use smoozy_state::identities::{Identity, IdentityKind};

use crate::presentation::the_machines_own_words::Stream;

use super::RecordFailure;

/// The one table, named once.
pub const TABLE: &str = "job_record_words";

/// **One stream of one attempt, as the record keeps it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStreamKept {
    pub attempt: u32,
    pub stream: Stream,
    /// `Rented-server runtime` M13's outcome. **`None` is M10a's outcome not
    /// yet written**, and such a row holds no words.
    pub outcome: Option<TheRuntimesOutcome>,
    /// **The bytes, whole and unedited** (`Rented-server runtime` M8, M11).
    pub the_words: Vec<u8>,
    /// `Rented-server runtime` M5's counts and first loss reason, where the
    /// machine reported them.
    pub retained: Option<u64>,
    pub discarded: Option<u64>,
    pub why_the_rest_went: Option<String>,
}

fn the_streams_word(stream: Stream) -> &'static str {
    match stream {
        Stream::TheOrdinaryOutput => "ordinary",
        Stream::TheComplaints => "complaints",
    }
}

fn the_stream_named(word: &str) -> Option<Stream> {
    Stream::BOTH
        .into_iter()
        .find(|stream| the_streams_word(*stream) == word)
}

/// **The jobs of this server whose words are owed** — `words_owed`, set by
/// the fill-in that learned the ending (`State, identity and recovery` V1 step
/// 4, `JobRecords::the_fill_in`).
pub fn the_jobs_owed_their_words(
    connection: &rusqlite::Connection,
    server: &Identity,
) -> Result<Vec<Identity>, RecordFailure> {
    let mut asking =
        connection.prepare("SELECT job FROM job_record WHERE server = ?1 AND words_owed = 1")?;
    let rows = asking.query_map([server.as_text()], |row| row.get::<_, String>(0))?;
    let mut out = Vec::new();
    for row in rows {
        if let Some(job) = Identity::read_back(IdentityKind::Job, &row?) {
            out.push(job);
        }
    }
    Ok(out)
}

/// **What a landing did.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheLandingDid {
    /// The rows are written, and the debt cleared where it was settled.
    ItLanded,
    /// **The entry is gone** — deleted while the words were on their way.
    /// Nothing is written, so no word outlives its entry.
    NoSuchRecordStands,
}

/// **Every stream that arrived, written in one act, and the debt cleared in
/// the same act where nothing more is owed.**
///
/// **A row holding an outcome is never written again** — a second landing of
/// the same attempt leaves the first as it is. A row holding none (M10a) is
/// the one row a later landing fills.
pub fn the_words_land(
    connection: &rusqlite::Connection,
    job: &Identity,
    arrived: &[AStreamKept],
    nothing_more_is_owed: bool,
) -> Result<WhatTheLandingDid, RecordFailure> {
    connection.execute_batch("BEGIN IMMEDIATE")?;
    let written = (|| -> Result<WhatTheLandingDid, RecordFailure> {
        let stands: bool = connection.query_row(
            "SELECT count(*) > 0 FROM job_record WHERE job = ?1",
            [job.as_text()],
            |row| row.get(0),
        )?;
        if !stands {
            return Ok(WhatTheLandingDid::NoSuchRecordStands);
        }
        for one in arrived {
            connection.execute(
                "INSERT INTO job_record_words
                    (job, attempt, stream, outcome, the_words, retained, discarded, why_the_rest_went)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                 ON CONFLICT (job, attempt, stream) DO UPDATE SET
                    outcome = excluded.outcome,
                    the_words = excluded.the_words,
                    retained = excluded.retained,
                    discarded = excluded.discarded,
                    why_the_rest_went = excluded.why_the_rest_went
                 WHERE job_record_words.outcome IS NULL",
                rusqlite::params![
                    job.as_text(),
                    one.attempt as i64,
                    the_streams_word(one.stream),
                    one.outcome.map(TheRuntimesOutcome::in_words),
                    one.the_words,
                    one.retained.map(|count| count as i64),
                    one.discarded.map(|count| count as i64),
                    one.why_the_rest_went,
                ],
            )?;
        }
        if nothing_more_is_owed {
            connection.execute(
                "UPDATE job_record SET words_owed = 0 WHERE job = ?1",
                [job.as_text()],
            )?;
        }
        Ok(WhatTheLandingDid::ItLanded)
    })();
    match written {
        Ok(WhatTheLandingDid::ItLanded) => {
            connection.execute_batch("COMMIT")?;
            Ok(WhatTheLandingDid::ItLanded)
        }
        Ok(nothing) => {
            let _ = connection.execute_batch("ROLLBACK");
            Ok(nothing)
        }
        Err(failure) => {
            let _ = connection.execute_batch("ROLLBACK");
            Err(failure)
        }
    }
}

/// **Every stream kept for a job**, in attempt order and, within an attempt,
/// the ordinary output before the complaints. **A row that will not read back
/// is left out** — an outcome in words that name none, or a stream that is
/// neither — and a panel never shows a stream it cannot name.
pub fn the_words_kept_for(
    connection: &rusqlite::Connection,
    job: &Identity,
) -> Result<Vec<AStreamKept>, RecordFailure> {
    let mut asking = connection.prepare(
        "SELECT attempt, stream, outcome, the_words, retained, discarded, why_the_rest_went
         FROM job_record_words WHERE job = ?1
         ORDER BY attempt, CASE stream WHEN 'ordinary' THEN 0 ELSE 1 END",
    )?;
    let rows = asking.query_map([job.as_text()], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, Vec<u8>>(3)?,
            row.get::<_, Option<i64>>(4)?,
            row.get::<_, Option<i64>>(5)?,
            row.get::<_, Option<String>>(6)?,
        ))
    })?;
    let mut out = Vec::new();
    for row in rows {
        let (attempt, stream, outcome, the_words, retained, discarded, why_the_rest_went) = row?;
        let Some(stream) = the_stream_named(&stream) else {
            continue;
        };
        let outcome = match outcome {
            None => None,
            Some(words) => match TheRuntimesOutcome::from_words(&words) {
                Some(outcome) => Some(outcome),
                None => continue,
            },
        };
        out.push(AStreamKept {
            attempt: attempt as u32,
            stream,
            outcome,
            the_words,
            retained: retained.map(|count| count as u64),
            discarded: discarded.map(|count| count as u64),
            why_the_rest_went,
        });
    }
    Ok(out)
}

/// **What each kept stream came to, without its words** — what History reads
/// every time it draws an entry, so the bytes are read only when the panel is
/// opened. In the same order as [`the_words_kept_for`]; a row that will not
/// read back is left out the same way.
pub fn the_outcomes_kept_for(
    connection: &rusqlite::Connection,
    job: &Identity,
) -> Result<Vec<(u32, Stream, Option<TheRuntimesOutcome>)>, RecordFailure> {
    let mut asking = connection.prepare(
        "SELECT attempt, stream, outcome FROM job_record_words WHERE job = ?1
         ORDER BY attempt, CASE stream WHEN 'ordinary' THEN 0 ELSE 1 END",
    )?;
    let rows = asking.query_map([job.as_text()], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
        ))
    })?;
    let mut out = Vec::new();
    for row in rows {
        let (attempt, stream, outcome) = row?;
        let Some(stream) = the_stream_named(&stream) else {
            continue;
        };
        let outcome = match outcome {
            None => None,
            Some(words) => match TheRuntimesOutcome::from_words(&words) {
                Some(outcome) => Some(outcome),
                None => continue,
            },
        };
        out.push((attempt as u32, stream, outcome));
    }
    Ok(out)
}

/// **Whether any word is kept for a job** — what decides whether History
/// offers *Blender output* at all. **No words kept is no control**: a panel
/// with nothing in it is never what the user meets (M10).
pub fn any_words_are_kept_for(
    connection: &rusqlite::Connection,
    job: &Identity,
) -> Result<bool, RecordFailure> {
    Ok(connection.query_row(
        "SELECT count(*) > 0 FROM job_record_words WHERE job = ?1",
        [job.as_text()],
        |row| row.get(0),
    )?)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::record::job_records::JobRecords;
    use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheNote, TheOutcomeAsWritten};

    pub(crate) fn a_record_store() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().expect("an in-memory record");
        for migration in crate::record::migrations::ALL {
            crate::record::migrations::run_one_for_tests(&connection, migration)
                .expect("every migration runs");
        }
        connection
    }

    /// **A job in history on one server, its ending not yet learned.**
    pub(crate) fn a_job_on(connection: &rusqlite::Connection, server: &Identity) -> Identity {
        let record = crate::record::deletions::tests::a_record_in_history(server, "Kitchen");
        let mut provisional = record.clone();
        provisional.what_the_note_carried.ending = None;
        JobRecords::over(connection)
            .write_the_provisional_record(&provisional)
            .expect("written");
        record.job
    }

    /// The note a finished job's machine sends.
    pub(crate) fn a_finished_note() -> TheNote {
        let mut note = TheNote::created_with_the_job_record("Kitchen");
        note.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Finished,
            exit_status: Some(0),
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            no_render_began: false,
            the_device_line_as_observed: None,
        });
        note.attempts = 1;
        note
    }

    pub(crate) fn a_stream(attempt: u32, stream: Stream, words: &[u8]) -> AStreamKept {
        AStreamKept {
            attempt,
            stream,
            outcome: Some(if words.is_empty() {
                TheRuntimesOutcome::CollectedAndTheStreamWasEmpty
            } else {
                TheRuntimesOutcome::Collected
            }),
            the_words: words.to_vec(),
            retained: Some(words.len() as u64),
            discarded: Some(0),
            why_the_rest_went: None,
        }
    }

    fn owed(connection: &rusqlite::Connection, server: &Identity) -> Vec<Identity> {
        the_jobs_owed_their_words(connection, server).expect("it reads")
    }

    /// **E9: the connection that learns the ending owes the words, and only
    /// that one.** The debt is written by the fill-in that learns the ending,
    /// in the same act; a fill-in that learns nothing new owes nothing; a
    /// landing that settles it clears it.
    #[test]
    fn learning_the_ending_owes_the_words_and_their_landing_settles_the_debt() {
        let connection = a_record_store();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let job = a_job_on(&connection, &server);
        assert!(owed(&connection, &server).is_empty(), "nothing is owed before the ending");

        JobRecords::over(&connection)
            .the_fill_in(&job, &a_finished_note())
            .expect("filled in");
        assert_eq!(owed(&connection, &server), vec![job.clone()]);

        assert_eq!(
            the_words_land(&connection, &job, &[a_stream(1, Stream::TheOrdinaryOutput, b"Fra:1")], false)
                .unwrap(),
            WhatTheLandingDid::ItLanded
        );
        assert_eq!(owed(&connection, &server), vec![job.clone()], "not settled, still owed");
        the_words_land(&connection, &job, &[], true).unwrap();
        assert!(owed(&connection, &server).is_empty());

        // **A second fill-in of the same ending owes nothing again.**
        JobRecords::over(&connection)
            .the_fill_in(&job, &a_finished_note())
            .expect("filled in");
        assert!(owed(&connection, &server).is_empty());
    }

    /// **Kept apart and unedited**: each stream of each attempt is its own
    /// row, read back byte for byte in attempt order; a second landing of a
    /// row that holds an outcome changes nothing; a row whose outcome was not
    /// yet written is the one a later landing fills.
    #[test]
    fn the_words_are_kept_apart_unedited_and_only_an_unwritten_outcome_is_filled_later() {
        let connection = a_record_store();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let job = a_job_on(&connection, &server);
        let bytes = b"Fra:1 Mem:9.84G | Sample 96/2048\n\xff\xfe not text".to_vec();
        let mut pending = a_stream(2, Stream::TheComplaints, b"");
        pending.outcome = None;
        the_words_land(
            &connection,
            &job,
            &[
                a_stream(2, Stream::TheOrdinaryOutput, b"second"),
                a_stream(1, Stream::TheComplaints, b""),
                a_stream(1, Stream::TheOrdinaryOutput, &bytes),
                pending,
            ],
            false,
        )
        .unwrap();
        let kept = the_words_kept_for(&connection, &job).unwrap();
        assert_eq!(
            kept.iter().map(|one| (one.attempt, one.stream)).collect::<Vec<_>>(),
            vec![
                (1, Stream::TheOrdinaryOutput),
                (1, Stream::TheComplaints),
                (2, Stream::TheOrdinaryOutput),
                (2, Stream::TheComplaints),
            ]
        );
        assert_eq!(kept[0].the_words, bytes, "byte for byte");
        assert_eq!(kept[3].outcome, None, "M10a, kept as itself");

        // **A second landing never edits a row that holds an outcome.**
        the_words_land(&connection, &job, &[a_stream(1, Stream::TheOrdinaryOutput, b"other")], false)
            .unwrap();
        // **An unwritten outcome is filled by the landing that brings it.**
        the_words_land(&connection, &job, &[a_stream(2, Stream::TheComplaints, b"late")], true)
            .unwrap();
        let again = the_words_kept_for(&connection, &job).unwrap();
        assert_eq!(again[0].the_words, bytes, "unedited");
        assert_eq!(again[3].the_words, b"late");
        assert_eq!(again[3].outcome, Some(TheRuntimesOutcome::Collected));
        assert!(any_words_are_kept_for(&connection, &job).unwrap());
    }

    /// **No word outlives its entry**: words arriving for an entry deleted on
    /// the way land nothing, and deleting an entry takes its words with it
    /// (`Queue & History` X1, through the one walk every set beside the row
    /// is named in).
    #[test]
    fn no_word_outlives_its_entry() {
        let connection = a_record_store();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let job = a_job_on(&connection, &server);
        the_words_land(&connection, &job, &[a_stream(1, Stream::TheOrdinaryOutput, b"Fra:1")], true)
            .unwrap();
        JobRecords::over(&connection).remove(&job).expect("removed");
        assert!(!any_words_are_kept_for(&connection, &job).unwrap());
        assert_eq!(
            the_words_land(&connection, &job, &[a_stream(1, Stream::TheOrdinaryOutput, b"Fra:1")], true)
                .unwrap(),
            WhatTheLandingDid::NoSuchRecordStands
        );
        assert!(!any_words_are_kept_for(&connection, &job).unwrap());
    }
}

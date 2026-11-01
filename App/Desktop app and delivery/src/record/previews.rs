//! **`Queue & History` P4's fetched derivatives, held on the job record** — the
//! table record version 37 made, out of the 38's piece 3, desktop half
//! (2026-10-03).
//!
//! **The rules are `Queue & History` §15's and `State, identity and recovery`
//! §11's, and the store is this document's**, the split every row module here
//! draws. What is here is the read and the write and nothing else:
//!
//! - **P3: a journalled production puts a fetch on the job** —
//!   [`a_derivative_is_owed`], written by the read forward before the cursor
//!   moves past the event (`State, identity and recovery` E3);
//! - **P4: the fetch lands, and the copy is held with its one moment** —
//!   [`the_copy_was_published`], the moment `State, identity and recovery` R13
//!   names, on this computer's clock;
//! - **P2 and P10a: what the entry draws** — [`the_newest_held`];
//! - **`State, identity and recovery` R18 to `State, identity and recovery` R20: what the age sweep reads and clears** — [`every_copy_held`]
//!   and [`the_copy_is_cleared`]. The comparison is `smoozy_state::lifetimes`'
//!   and is not made here.
//!
//! **Subject-held on the job record** (`State, identity and recovery` R23):
//! the table is one of [`super::job_records::EVERY_SET_BESIDE_THE_ROW`], and
//! [`super::job_records::JobRecords::remove`] takes the copies off this computer
//! with the rows ([`the_copies_held_for`], [`the_copies_go`]).
//!
//! **Every path here is the copy's own, recorded when it was published**, and
//! nothing composes one from a field of the record (`Desktop app and delivery`
//! L4): the name is issued fresh for each fetch by the one who fetches.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use smoozy_job_files::main_output::WhereThePictureCameFrom;
use smoozy_state::identities::{Identity, IdentityKind};

use super::RecordFailure;

/// The one table, named once.
pub const TABLE: &str = "job_record_preview";

/// **One frame's derivative, as this computer holds it** — owed, held, or
/// held with a newer one owed (`Job files and outputs` B29 publishes over the
/// name, and `Queue & History` P4 re-fetches it, replacing the held copy).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFrameOfTheSet {
    pub job: Identity,
    pub frame: i64,
    /// **The production record on the machine the newest event named.**
    pub operation: String,
    /// **`Queue & History` P10a's name, as the bytes it is**, absent where the
    /// event carried none — **and part of the key** (P4: job, frame, View
    /// Layer), so two layers' copies of one frame never replace each other.
    pub view_layer: Option<Vec<u8>>,
    /// **Whether it is the frame's main copy** (P10): what the live fetch and
    /// the notification read. Every row held before record version 40 is.
    pub the_main_copy: bool,
    /// **Its place in the frame's set** (P4), the order a frame's layers are
    /// listed in. Nought on every row held before record version 40.
    pub place: i64,
    /// **Where its picture came from, for a copy that names no View Layer in
    /// a frame of more than one** — the output path, or a File Output node by
    /// name (`Job files and outputs` B31a, `Queue & History` P3), as the event
    /// carried it. **Such a copy is listed first and is no layer's**, so it is
    /// never what a layer's place asks for ([`the_copy_of`]). Absent on every
    /// other row, and on every row held before record version 44. The
    /// Creator's tenth notes, note 2.
    pub source: Option<WhereThePictureCameFrom>,
    /// **A fetch is owed for it**, and none has published since the event.
    pub owed: bool,
    /// **The held copy**, where a fetch has published one.
    pub copy: Option<PathBuf>,
    /// **`State, identity and recovery` R13's one moment**: when that copy was
    /// published on this computer, by this computer's clock.
    pub published_on_this_computer_at: Option<SystemTime>,
}

impl AFrameOfTheSet {
    /// **A copy is held** — published, with its moment.
    pub fn is_held(&self) -> bool {
        self.copy.is_some() && self.published_on_this_computer_at.is_some()
    }
}

/// **The moment, in the record's own spelling**: milliseconds since the
/// epoch. A moment before the epoch is not one this computer publishes at.
fn in_milliseconds(at: SystemTime) -> Option<i64> {
    at.duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|since| i64::try_from(since.as_millis()).ok())
}

fn from_milliseconds(at: i64) -> Option<SystemTime> {
    u64::try_from(at)
        .ok()
        .map(|at| UNIX_EPOCH + Duration::from_millis(at))
}

const EVERY_COLUMN: &str =
    "job, frame, operation, view_layer, owed, copy, published_on_this_computer_at, main_copy, place, source";

type ARow = (String, i64, String, Option<Vec<u8>>, i64, Option<String>, Option<i64>, i64, i64, Option<String>);

fn a_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ARow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
        row.get(9)?,
    ))
}

/// **The key's View Layer part, matched whether it is named or absent** —
/// `IS` rather than `=`, since an absent layer is a key too (P4).
const THE_KEY: &str = "job = ?1 AND frame = ?2 AND view_layer IS ?3";

/// **A row whose job will not read back is left out**: it names nothing an
/// entry can draw.
fn composed((job, frame, operation, view_layer, owed, copy, at, main, place, source): ARow) -> Option<AFrameOfTheSet> {
    Some(AFrameOfTheSet {
        job: Identity::read_back(IdentityKind::Job, &job)?,
        frame,
        operation,
        view_layer,
        the_main_copy: main == 1,
        place,
        // **Words that will not read back are no source**: the copy is then
        // drawn as one carrying none, as every copy before version 44 is.
        source: source.as_deref().and_then(WhereThePictureCameFrom::read_back),
        owed: owed == 1,
        copy: copy.map(PathBuf::from),
        published_on_this_computer_at: at.and_then(from_milliseconds),
    })
}

fn every_row(
    connection: &rusqlite::Connection,
    sql: &str,
    parameters: impl rusqlite::Params,
) -> Result<Vec<AFrameOfTheSet>, RecordFailure> {
    let mut asking = connection.prepare(sql)?;
    let rows = asking.query_map(parameters, a_row)?;
    let mut out = Vec::new();
    for row in rows {
        if let Some(one) = composed(row?) {
            out.push(one);
        }
    }
    Ok(out)
}

/// **What writing an owed fetch did.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheEventPut {
    /// **A fetch is owed** — for a frame not held, or for one whose held copy
    /// a newer production replaces.
    AFetchIsOwed,
    /// **The same production read again** (`State, identity and recovery` E4):
    /// nothing changes.
    NothingNew,
    /// **No local record of the job stands** — deleted, or never this
    /// desktop's. Nothing is written, so no copy outlives its entry.
    NoSuchRecordStands,
}

/// **`Queue & History` P3: a production the read forward applied puts a fetch
/// on the job.** One small write, atomic by itself.
///
/// **Idempotent** (`State, identity and recovery` E4): the event read again
/// names the production already written and changes nothing. **A newer
/// production of a frame already held** (`Job files and outputs` B29) owes a
/// fetch again and keeps the held copy until that fetch replaces it.
///
/// **Keyed by (job, frame, View Layer)** (P4, the Creator's seventh notes,
/// note 4): each layer's copy is owed on its own row, carrying whether it is
/// the frame's main copy and its place — **and, for a copy that names no View
/// Layer in a frame of more than one, where its picture came from** (P3, the
/// Creator's tenth notes, note 2), kept as the event said it.
#[allow(clippy::too_many_arguments)]
pub fn a_derivative_is_owed(
    connection: &rusqlite::Connection,
    job: &Identity,
    frame: i64,
    operation: &str,
    view_layer: Option<&[u8]>,
    the_main_copy: bool,
    place: i64,
    source: Option<&WhereThePictureCameFrom>,
) -> Result<WhatTheEventPut, RecordFailure> {
    let source = source.map(WhereThePictureCameFrom::as_written);
    let stands: bool = connection.query_row(
        "SELECT count(*) > 0 FROM job_record WHERE job = ?1",
        [job.as_text()],
        |row| row.get(0),
    )?;
    if !stands {
        return Ok(WhatTheEventPut::NoSuchRecordStands);
    }
    let already: Option<String> = connection
        .query_row(
            &format!("SELECT operation FROM job_record_preview WHERE {THE_KEY}"),
            rusqlite::params![job.as_text(), frame, view_layer],
            |row| row.get(0),
        )
        .map(Some)
        .or_else(|failure| match failure {
            rusqlite::Error::QueryReturnedNoRows => Ok(None),
            other => Err(other),
        })?;
    match already {
        Some(standing) if standing == operation => Ok(WhatTheEventPut::NothingNew),
        Some(_) => {
            connection.execute(
                &format!(
                    "UPDATE job_record_preview SET operation = ?4, owed = 1, main_copy = ?5, place = ?6, source = ?7
                     WHERE {THE_KEY}"
                ),
                rusqlite::params![job.as_text(), frame, view_layer, operation, i64::from(the_main_copy), place, source],
            )?;
            Ok(WhatTheEventPut::AFetchIsOwed)
        }
        None => {
            connection.execute(
                "INSERT INTO job_record_preview (job, frame, operation, view_layer, owed, main_copy, place, source)
                 VALUES (?1, ?2, ?3, ?4, 1, ?5, ?6, ?7)",
                rusqlite::params![job.as_text(), frame, operation, view_layer, i64::from(the_main_copy), place, source],
            )?;
            Ok(WhatTheEventPut::AFetchIsOwed)
        }
    }
}

/// **The fetches owed for this server's jobs** — `Queue & History` P2 and
/// `Queue & History` P4: the running job's newest main copy first, each ended
/// job's newest main copy next, and every other View Layer copy and
/// passed-over frame behind them, newest frame first.
pub fn the_owed_on(
    connection: &rusqlite::Connection,
    server: &Identity,
) -> Result<Vec<AFrameOfTheSet>, RecordFailure> {
    every_row(
        connection,
        "SELECT job_record_preview.job, frame, operation, view_layer, owed, copy,
                published_on_this_computer_at, main_copy, place, source
         FROM job_record_preview JOIN job_record ON job_record.job = job_record_preview.job
         WHERE job_record.server = ?1 AND owed = 1
         ORDER BY
             CASE
                 WHEN job_record_preview.main_copy = 1
                  AND job_record_preview.frame = (
                      SELECT MAX(latest.frame)
                      FROM job_record_preview AS latest
                      WHERE latest.job = job_record_preview.job AND latest.main_copy = 1
                  )
                 THEN CASE WHEN job_record.note_ending_outcome IS NULL THEN 2 ELSE 1 END
                 ELSE 0
             END DESC,
             frame DESC,
             main_copy DESC,
             place ASC,
             job_record_preview.rowid DESC",
        [server.as_text()],
    )
}

/// **What landing a fetched copy did.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThePublicationDid {
    /// **The copy is held**, and any copy it replaced is handed back to be
    /// taken off this computer.
    ItIsHeld { replaced: Option<PathBuf> },
    /// **The frame is no longer the record's** — the entry was deleted, or the
    /// age sweep cleared it, while the fetch was on its way. The fetched copy
    /// is handed back to be taken off this computer, so it outlives nothing.
    NothingHoldsIt,
}

/// **`Queue & History` P4: the fetch published, and the copy is held with its
/// one moment** (`State, identity and recovery` R13).
///
/// **Where a newer production was journalled while this one was on its way**,
/// the copy is held and the fetch stays owed, so the newer one replaces it in
/// turn. One small write.
pub fn the_copy_was_published(
    connection: &rusqlite::Connection,
    job: &Identity,
    frame: i64,
    view_layer: Option<&[u8]>,
    operation: &str,
    copy: &Path,
    at: SystemTime,
) -> Result<WhatThePublicationDid, RecordFailure> {
    let Some(text) = copy.to_str() else {
        return Ok(WhatThePublicationDid::NothingHoldsIt);
    };
    let Some(at) = in_milliseconds(at) else {
        return Ok(WhatThePublicationDid::NothingHoldsIt);
    };
    let standing: Option<(String, Option<String>)> = connection
        .query_row(
            &format!("SELECT operation, copy FROM job_record_preview WHERE {THE_KEY}"),
            rusqlite::params![job.as_text(), frame, view_layer],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map(Some)
        .or_else(|failure| match failure {
            rusqlite::Error::QueryReturnedNoRows => Ok(None),
            other => Err(other),
        })?;
    let Some((the_newest_named, replaced)) = standing else {
        return Ok(WhatThePublicationDid::NothingHoldsIt);
    };
    let still_owed = the_newest_named != operation;
    connection.execute(
        &format!(
            "UPDATE job_record_preview SET copy = ?4, published_on_this_computer_at = ?5, owed = ?6
             WHERE {THE_KEY}"
        ),
        rusqlite::params![job.as_text(), frame, view_layer, text, at, i64::from(still_owed)],
    )?;
    Ok(WhatThePublicationDid::ItIsHeld {
        replaced: replaced.map(PathBuf::from).filter(|old| old != copy),
    })
}

/// **The machine no longer holds this production** — it said so when the
/// fetch asked for it. **The debt is settled with nothing**: a frame never
/// held goes from the set, and a held copy stays as it is.
pub fn the_production_is_gone(
    connection: &rusqlite::Connection,
    job: &Identity,
    frame: i64,
    view_layer: Option<&[u8]>,
    operation: &str,
) -> Result<(), RecordFailure> {
    connection.execute(
        &format!("DELETE FROM job_record_preview WHERE {THE_KEY} AND operation = ?4 AND copy IS NULL"),
        rusqlite::params![job.as_text(), frame, view_layer, operation],
    )?;
    connection.execute(
        &format!("UPDATE job_record_preview SET owed = 0 WHERE {THE_KEY} AND operation = ?4"),
        rusqlite::params![job.as_text(), frame, view_layer, operation],
    )?;
    Ok(())
}

/// **`Queue & History` P2: the newest derivative the set holds** — the
/// highest frame with a copy on this computer. What the entry draws.
///
/// **The main copy only** (P10, the Creator's seventh notes, note 4): what the
/// entry opens on and the finished notification attaches is never another
/// View Layer's copy.
pub fn the_newest_held(
    connection: &rusqlite::Connection,
    job: &Identity,
) -> Result<Option<AFrameOfTheSet>, RecordFailure> {
    Ok(every_row(
        connection,
        &format!(
            "SELECT {EVERY_COLUMN} FROM job_record_preview
             WHERE job = ?1 AND copy IS NOT NULL AND published_on_this_computer_at IS NOT NULL
               AND main_copy = 1
             ORDER BY frame DESC LIMIT 1"
        ),
        [job.as_text()],
    )?
    .into_iter()
    .next())
}

/// **One frame's held copy**, for the page that asks for its bytes: the copy
/// at `place` in the frame's set, **or the frame's main copy where no place is
/// asked** (P10).
///
/// **A copy that carries a source is no layer's**, so a place never asks for
/// it, and it is reached as the frame's main copy (the Creator's tenth notes,
/// note 2): it sits at place nought beside the first layer's own copy, which a
/// place of nought must still find.
pub fn the_copy_of(
    connection: &rusqlite::Connection,
    job: &Identity,
    frame: i64,
    place: Option<i64>,
) -> Result<Option<AFrameOfTheSet>, RecordFailure> {
    Ok(every_row(
        connection,
        &format!(
            "SELECT {EVERY_COLUMN} FROM job_record_preview
             WHERE job = ?1 AND frame = ?2 AND copy IS NOT NULL
               AND (CASE WHEN ?3 IS NULL THEN main_copy = 1 ELSE place = ?3 AND source IS NULL END)
             ORDER BY main_copy DESC, rowid DESC"
        ),
        rusqlite::params![job.as_text(), frame, place],
    )?
    .into_iter()
    .next())
}

/// **Every copy the job's record holds, frame by frame and in each frame's
/// own order** — what the preview's surface hands the page, so it can list a
/// frame's View Layers and find its main copy (P4, P10a). Added 2026-10-10,
/// the Creator's seventh notes, note 4. **A copy that carries a source comes
/// first in its frame, then the layers** (`Design/Queue & History` line 201,
/// the Creator's tenth notes, note 2): it is always the frame's main copy, at
/// place nought, so the main copy's coming first at a place puts it there.
pub fn every_copy_held_for(
    connection: &rusqlite::Connection,
    job: &Identity,
) -> Result<Vec<AFrameOfTheSet>, RecordFailure> {
    every_row(
        connection,
        &format!(
            "SELECT {EVERY_COLUMN} FROM job_record_preview
             WHERE job = ?1 AND copy IS NOT NULL AND published_on_this_computer_at IS NOT NULL
             ORDER BY frame ASC, place ASC, main_copy DESC"
        ),
        [job.as_text()],
    )
}

/// **Every copy held, for the age sweep** (`State, identity and recovery`
/// `State, identity and recovery` R18).
pub fn every_copy_held(connection: &rusqlite::Connection) -> Result<Vec<AFrameOfTheSet>, RecordFailure> {
    every_row(
        connection,
        &format!(
            "SELECT {EVERY_COLUMN} FROM job_record_preview
             WHERE copy IS NOT NULL AND published_on_this_computer_at IS NOT NULL
             ORDER BY rowid"
        ),
        [],
    )
}

/// **`State, identity and recovery` R20: the copy alone.** The frame goes from
/// the set — the job record, its ending, its times and its figures are
/// untouched — **and nothing is written in its place**: a pass writes no
/// durable state of its own. Only the copy the sweep judged is cleared, so a
/// copy a fetch replaced meanwhile is judged at the next pass.
pub fn the_copy_is_cleared(
    connection: &rusqlite::Connection,
    job: &Identity,
    frame: i64,
    copy: &Path,
) -> Result<(), RecordFailure> {
    let Some(text) = copy.to_str() else {
        return Ok(());
    };
    connection.execute(
        "DELETE FROM job_record_preview WHERE job = ?1 AND frame = ?2 AND copy = ?3",
        rusqlite::params![job.as_text(), frame, text],
    )?;
    Ok(())
}

/// **Every copy this job's record holds**, read before the record goes.
pub fn the_copies_held_for(
    connection: &rusqlite::Connection,
    job: &Identity,
) -> Result<Vec<PathBuf>, RecordFailure> {
    Ok(every_row(
        connection,
        &format!("SELECT {EVERY_COLUMN} FROM job_record_preview WHERE job = ?1 AND copy IS NOT NULL"),
        [job.as_text()],
    )?
    .into_iter()
    .filter_map(|frame| frame.copy)
    .collect())
}

/// **The copies are taken off this computer** — exactly the files the record
/// named, and nothing beside them. **A removal that did not complete is not
/// retried here**: the record no longer names the file, and that is said
/// where the record and the world stop agreeing.
pub fn the_copies_go(copies: &[PathBuf]) {
    for copy in copies {
        let _ = std::fs::remove_file(copy);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn a_record() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        for migration in crate::record::migrations::ALL {
            if let crate::record::migrations::WhatItRuns::TheseStatements(statements) =
                migration.what_it_runs
            {
                for statement in statements {
                    connection.execute(statement, []).unwrap();
                }
            }
        }
        connection
    }

    /// **Record version 40 (the Creator's seventh notes, note 4): a row held
    /// before it reads with its layer as it was, as the frame's main copy at
    /// place nought; the key is (job, frame, View Layer)**, so two layers of a
    /// frame stand side by side while two unnamed copies of one frame cannot.
    /// A throwaway record in memory — nothing of the Creator's is opened.
    #[test]
    fn version_40_keys_the_copies_by_view_layer_and_an_older_row_reads_as_the_main_copy() {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        for migration in crate::record::migrations::ALL.iter().filter(|migration| migration.produces < 40) {
            if let crate::record::migrations::WhatItRuns::TheseStatements(statements) = migration.what_it_runs {
                for statement in statements {
                    connection.execute(statement, []).unwrap();
                }
            }
        }
        let job = Identity::issue(IdentityKind::Job).unwrap();
        connection
            .execute(
                "INSERT INTO job_record_preview VALUES (?1, 5, 'op', NULL, 0, '/p/a.png', 1759500000000)",
                [job.as_text()],
            )
            .unwrap();
        for statement in crate::record::schema::VERSION_40 {
            connection.execute(statement, []).unwrap();
        }
        // **Read by today's reader, so brought to today's shape first**: the
        // versions after 40 add to the row and carry nothing (version 44's
        // source among them, the Creator's tenth notes, note 2).
        for migration in crate::record::migrations::ALL.iter().filter(|migration| migration.produces > 40) {
            if let crate::record::migrations::WhatItRuns::TheseStatements(statements) = migration.what_it_runs {
                for statement in statements {
                    connection.execute(statement, []).unwrap();
                }
            }
        }
        let held = every_copy_held_for(&connection, &job).unwrap();
        assert_eq!(held.len(), 1, "the row was carried over whole");
        assert_eq!(held[0].view_layer, None);
        assert!(held[0].the_main_copy);
        assert_eq!(held[0].place, 0);
        assert_eq!(held[0].copy.as_deref(), Some(Path::new("/p/a.png")));

        let insert = |layer: Option<&[u8]>, main: i64, place: i64| {
            connection.execute(
                "INSERT INTO job_record_preview (job, frame, operation, view_layer, owed, main_copy, place)
                 VALUES (?1, 6, 'op-6', ?2, 1, ?3, ?4)",
                rusqlite::params![job.as_text(), layer, main, place],
            )
        };
        insert(Some(b"Box"), 1, 0).unwrap();
        insert(Some(b"Mattress"), 0, 1).expect("a second layer of the frame stands beside the first");
        assert!(insert(Some(b"Box"), 1, 0).is_err(), "one row per (job, frame, View Layer)");
        connection
            .execute("INSERT INTO job_record_preview (job, frame, operation, owed) VALUES (?1, 7, 'op-7', 1)", [job.as_text()])
            .unwrap();
        assert!(
            connection
                .execute("INSERT INTO job_record_preview (job, frame, operation, owed) VALUES (?1, 7, 'op-8', 1)", [job.as_text()])
                .is_err(),
            "one unnamed copy per frame"
        );
    }

    fn a_job_in(connection: &rusqlite::Connection, server: &Identity) -> Identity {
        let made = crate::record::deletions::tests::a_record_in_history(server, "Kitchen");
        crate::record::job_records::JobRecords::over(connection)
            .write_the_provisional_record(&made)
            .unwrap();
        made.job
    }

    fn a_server() -> Identity {
        Identity::issue(IdentityKind::LocalServer).unwrap()
    }

    /// **P3 and E4: an event owes a fetch once, the same event again owes
    /// nothing, and a newer production of a held frame owes it again** —
    /// keeping the held copy until the newer one lands.
    #[test]
    fn an_event_owes_one_fetch_and_a_newer_production_owes_it_again() {
        let connection = a_record();
        let server = a_server();
        let job = a_job_in(&connection, &server);
        assert_eq!(
            a_derivative_is_owed(&connection, &job, 5, "op-1", None, true, 0, None).unwrap(),
            WhatTheEventPut::AFetchIsOwed
        );
        assert_eq!(
            a_derivative_is_owed(&connection, &job, 5, "op-1", None, true, 0, None).unwrap(),
            WhatTheEventPut::NothingNew
        );
        assert_eq!(the_owed_on(&connection, &server).unwrap().len(), 1);

        let at = SystemTime::now();
        let copy = PathBuf::from("/previews/a.png");
        assert_eq!(
            the_copy_was_published(&connection, &job, 5, None, "op-1", &copy, at).unwrap(),
            WhatThePublicationDid::ItIsHeld { replaced: None }
        );
        assert!(the_owed_on(&connection, &server).unwrap().is_empty());
        let held = the_newest_held(&connection, &job).unwrap().unwrap();
        assert_eq!(held.copy.as_deref(), Some(copy.as_path()));
        assert_eq!(
            held.published_on_this_computer_at.map(in_milliseconds),
            Some(in_milliseconds(at))
        );

        assert_eq!(
            a_derivative_is_owed(&connection, &job, 5, "op-2", None, true, 0, None).unwrap(),
            WhatTheEventPut::AFetchIsOwed
        );
        assert_eq!(
            the_newest_held(&connection, &job).unwrap().unwrap().copy.as_deref(),
            Some(copy.as_path()),
            "the held copy stands until the newer one lands"
        );
        let newer = PathBuf::from("/previews/b.png");
        assert_eq!(
            the_copy_was_published(&connection, &job, 5, None, "op-2", &newer, at).unwrap(),
            WhatThePublicationDid::ItIsHeld { replaced: Some(copy) }
        );
        // **The View Layer is the key's** (P4, the Creator's seventh notes,
        // note 4): a newer production of the same layer replaces it in place.
        let held = the_newest_held(&connection, &job).unwrap().unwrap();
        assert_eq!((held.view_layer, held.operation.as_str()), (None, "op-2"));
    }

    /// **A production journalled while an older fetch was on its way keeps
    /// the frame owed**, so the newer one is fetched in turn.
    #[test]
    fn an_older_fetch_landing_after_a_newer_event_leaves_the_frame_owed() {
        let connection = a_record();
        let server = a_server();
        let job = a_job_in(&connection, &server);
        a_derivative_is_owed(&connection, &job, 5, "op-1", None, true, 0, None).unwrap();
        a_derivative_is_owed(&connection, &job, 5, "op-2", None, true, 0, None).unwrap();
        the_copy_was_published(&connection, &job, 5, None, "op-1", Path::new("/p/a.png"), SystemTime::now())
            .unwrap();
        let owed = the_owed_on(&connection, &server).unwrap();
        assert_eq!(owed.len(), 1);
        assert_eq!(owed[0].operation, "op-2");
    }

    /// **P2: newest frame first**, and the entry draws the highest frame held.
    #[test]
    fn the_owed_come_newest_first_and_the_entry_draws_the_highest_held() {
        let connection = a_record();
        let server = a_server();
        let job = a_job_in(&connection, &server);
        for frame in [1, 3, 2] {
            a_derivative_is_owed(&connection, &job, frame, &format!("op-{frame}"), None, true, 0, None).unwrap();
        }
        let order: Vec<i64> = the_owed_on(&connection, &server)
            .unwrap()
            .iter()
            .map(|one| one.frame)
            .collect();
        assert_eq!(order, vec![3, 2, 1]);
        for frame in [1, 2] {
            the_copy_was_published(
                &connection,
                &job,
                frame,
                None,
                &format!("op-{frame}"),
                Path::new(&format!("/p/{frame}.png")),
                SystemTime::now(),
            )
            .unwrap();
        }
        assert_eq!(the_newest_held(&connection, &job).unwrap().unwrap().frame, 2);
        assert!(the_owed_on(&connection, &a_server()).unwrap().is_empty(), "another server owes nothing");
    }

    /// **A job with no record owes nothing**, and a fetch landing after the
    /// entry went holds nothing.
    #[test]
    fn nothing_is_held_for_a_record_that_does_not_stand() {
        let connection = a_record();
        let job = Identity::issue(IdentityKind::Job).unwrap();
        assert_eq!(
            a_derivative_is_owed(&connection, &job, 1, "op", None, true, 0, None).unwrap(),
            WhatTheEventPut::NoSuchRecordStands
        );
        assert_eq!(
            the_copy_was_published(&connection, &job, 1, None, "op", Path::new("/p/x.png"), SystemTime::now())
                .unwrap(),
            WhatThePublicationDid::NothingHoldsIt
        );
    }

    /// **`State, identity and recovery` R23 and R18: removing the record takes its copies off this computer**,
    /// and only the files the record named.
    #[test]
    fn removing_the_record_takes_its_copies_and_nothing_beside_them() {
        let folder = std::env::temp_dir().join(format!(
            "smoozy-previews-{}",
            Identity::issue(IdentityKind::Operation).unwrap().as_text()
        ));
        std::fs::create_dir_all(&folder).unwrap();
        let copy = folder.join("held.png");
        let beside = folder.join("beside.png");
        std::fs::write(&copy, b"png").unwrap();
        std::fs::write(&beside, b"png").unwrap();

        let connection = a_record();
        let server = a_server();
        let job = a_job_in(&connection, &server);
        a_derivative_is_owed(&connection, &job, 1, "op", None, true, 0, None).unwrap();
        the_copy_was_published(&connection, &job, 1, None, "op", &copy, SystemTime::now()).unwrap();
        crate::record::job_records::JobRecords::over(&connection).remove(&job).unwrap();

        assert!(!copy.exists(), "the copy went with the record");
        assert!(beside.exists(), "nothing the record did not name was touched");
        assert!(every_copy_held(&connection).unwrap().is_empty());
        std::fs::remove_file(&beside).unwrap();
        std::fs::remove_dir(&folder).unwrap();
    }

    /// **The Creator's shape, held** (the tenth notes, note 2): the main copy
    /// from the output path keeps its source and comes first in its frame;
    /// the first layer's own copy at place nought stands beside it, and a
    /// place of nought reaches that layer and never the main copy, which no
    /// place asks for. A node's source keeps its name's bytes. **What it bites
    /// on**: the source not kept; the main copy listed after the layers; the
    /// layer at place nought shadowed by the main copy.
    #[test]
    fn tenth_notes_2_a_sourced_main_copy_is_kept_first_and_never_shadows_a_layer() {
        let connection = a_record();
        let server = a_server();
        let job = a_job_in(&connection, &server);
        let path = WhereThePictureCameFrom::TheOutputPath;
        a_derivative_is_owed(&connection, &job, 4, "op", Some(b"Box"), false, 0, None).unwrap();
        a_derivative_is_owed(&connection, &job, 4, "op", Some(b"Mattress"), false, 1, None).unwrap();
        a_derivative_is_owed(&connection, &job, 4, "op", None, true, 0, Some(&path)).unwrap();
        let owed = the_owed_on(&connection, &server).unwrap();
        assert_eq!(owed.len(), 3);
        assert_eq!((owed[0].the_main_copy, owed[0].source.clone()), (true, Some(path.clone())), "the main copy is owed first");
        for (layer, copy) in [(None, "/p/main.png"), (Some(&b"Box"[..]), "/p/box.png"), (Some(&b"Mattress"[..]), "/p/mattress.png")] {
            the_copy_was_published(&connection, &job, 4, layer, "op", Path::new(copy), SystemTime::now()).unwrap();
        }
        let held = every_copy_held_for(&connection, &job).unwrap();
        assert_eq!(
            held.iter().map(|one| (one.view_layer.clone(), one.source.clone())).collect::<Vec<_>>(),
            vec![(None, Some(path)), (Some(b"Box".to_vec()), None), (Some(b"Mattress".to_vec()), None)],
            "the source first, then the layers in their order"
        );
        let copy = |place: Option<i64>| the_copy_of(&connection, &job, 4, place).unwrap().unwrap().copy.unwrap();
        assert_eq!(copy(None), PathBuf::from("/p/main.png"));
        assert_eq!(copy(Some(0)), PathBuf::from("/p/box.png"), "place nought is the first layer's");
        assert_eq!(copy(Some(1)), PathBuf::from("/p/mattress.png"));
        assert_eq!(the_newest_held(&connection, &job).unwrap().unwrap().copy, Some(PathBuf::from("/p/main.png")));

        let node = WhereThePictureCameFrom::AFileOutputNode(smoozy_job_files::reading::ArtistsText::of(vec![b'B', 0xff]));
        a_derivative_is_owed(&connection, &job, 5, "op-5", None, true, 0, Some(&node)).unwrap();
        assert_eq!(the_owed_on(&connection, &server).unwrap()[0].source, Some(node));
    }

    /// **A production the machine no longer holds settles the debt**: a frame
    /// never held goes, a held one keeps its copy.
    #[test]
    fn a_production_gone_from_the_machine_settles_the_debt() {
        let connection = a_record();
        let server = a_server();
        let job = a_job_in(&connection, &server);
        a_derivative_is_owed(&connection, &job, 1, "op-1", None, true, 0, None).unwrap();
        a_derivative_is_owed(&connection, &job, 2, "op-2", None, true, 0, None).unwrap();
        the_copy_was_published(&connection, &job, 2, None, "op-2", Path::new("/p/2.png"), SystemTime::now())
            .unwrap();
        a_derivative_is_owed(&connection, &job, 2, "op-3", None, true, 0, None).unwrap();
        the_production_is_gone(&connection, &job, 1, None, "op-1").unwrap();
        the_production_is_gone(&connection, &job, 2, None, "op-3").unwrap();
        assert!(the_owed_on(&connection, &server).unwrap().is_empty());
        assert_eq!(the_newest_held(&connection, &job).unwrap().unwrap().frame, 2);
        assert!(the_copy_of(&connection, &job, 1, None).unwrap().is_none());
    }

    /// **The Creator's nineteenth notes, note 10 — `Queue & History` P2 and
    /// `Queue & History` P4**: the running job's newest main copy comes first
    /// even at a lower frame number, each ended job's newest main copy comes
    /// next, and publishing the running job's newest main copy does not promote
    /// its passed-over frames ahead of an ended job's one picture.
    #[test]
    fn nineteenth_notes_10_each_jobs_newest_main_copy_comes_before_passed_over_frames() {
        use smoozy_queue_history::acting::note::{TheEndingOnTheNote, TheOutcomeAsWritten};

        let connection = a_record();
        let server = a_server();

        let ended_in = |label: &str| {
            let mut made = crate::record::deletions::tests::a_record_in_history(&server, label);
            made.what_the_note_carried.ending = Some(TheEndingOnTheNote {
                outcome: TheOutcomeAsWritten::Finished,
                exit_status: Some(0),
                the_attempt_whose_capture_record_holds_the_words: 1,
                the_reason_the_machine_recorded: None,
                the_device_line_as_observed: None,
            });
            crate::record::job_records::JobRecords::over(&connection)
                .write_the_provisional_record(&made)
                .unwrap();
            made.job
        };

        let the_check = ended_in("Check");
        let the_earlier_job = ended_in("Earlier");
        let the_running_job = a_job_in(&connection, &server);

        a_derivative_is_owed(&connection, &the_check, 1, "check-1", None, true, 0, None).unwrap();
        for frame in [1, 10, 50] {
            a_derivative_is_owed(
                &connection,
                &the_earlier_job,
                frame,
                &format!("earlier-{frame}"),
                None,
                true,
                0,
                None,
            )
            .unwrap();
        }
        for frame in 1..=5 {
            a_derivative_is_owed(
                &connection,
                &the_running_job,
                frame,
                &format!("running-{frame}"),
                None,
                true,
                0,
                None,
            )
            .unwrap();
        }
        a_derivative_is_owed(&connection, &the_running_job, 5, "running-5", Some(b"Mist"), false, 1, None)
            .unwrap();

        let owed: Vec<(Identity, i64, bool)> = the_owed_on(&connection, &server)
            .unwrap()
            .into_iter()
            .map(|one| (one.job, one.frame, one.the_main_copy))
            .collect();
        assert_eq!(
            &owed[..3],
            &[
                (the_running_job.clone(), 5, true),
                (the_earlier_job.clone(), 50, true),
                (the_check.clone(), 1, true),
            ],
            "running newest main copy first, then each ended job's newest main copy"
        );

        the_copy_was_published(
            &connection,
            &the_running_job,
            5,
            None,
            "running-5",
            Path::new("/p/r5.png"),
            SystemTime::now(),
        )
        .unwrap();
        let after_running_newest: Vec<(Identity, i64, bool)> = the_owed_on(&connection, &server)
            .unwrap()
            .into_iter()
            .map(|one| (one.job, one.frame, one.the_main_copy))
            .collect();
        assert_eq!(
            &after_running_newest[..2],
            &[
                (the_earlier_job, 50, true),
                (the_check, 1, true),
            ],
            "a held newest main copy keeps the running job's passed-over frames behind ended jobs' newest main copies"
        );
    }
}

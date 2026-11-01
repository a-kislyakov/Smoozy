//! **`Queue & History` C2's catch-up notice record, in the local record** —
//! the two tables record version 38 made, out of the 38's piece 15
//! (2026-10-04).
//!
//! **The rule is `Queue & History`'s and the store is this document's**, the
//! split every row module here draws. `Queue & History` C2: *"Applying step 3 writes the notice's
//! content into the local record as part of the application, before the cursor
//! advances past the events it names"*; *"A crash between advancing and showing
//! loses the showing and not the record; the next run shows the standing
//! notice."* **What is here is the read, the write and the removal, and nothing
//! else**: the composition is `smoozy_queue_history::catch_up`'s, and what
//! opening History and a dropped line's *Dismiss* take is
//! `smoozy_queue_history::acting::catching_up`'s, reached through
//! [`TheCatchUpHalf`], that crate's own seam onto this record.
//!
//! **One standing record per machine** (`Queue & History` C1a), keyed by the
//! local server identifier, and **subject-held on the server row** (`Queue &
//! History` C2): the row's removal
//! walk takes it, its lines first, under
//! `smoozy_servers::store::KeyedByTheServer::CatchUpNotices`.
//!
//! **A line that will not read back is left out of the answer** — an identity
//! the record cannot spell, a kind or an ending this binary does not know.
//! That costs one line of the account, never the account.

use smoozy_queue_history::acting::catching_up::KeepsTheCatchUpHalf;
use smoozy_queue_history::catch_up::{ANoticeEntry, EventCursor, NoticeRecord};
use smoozy_queue_history::ending::Ending;
use smoozy_state::identities::{Identity, IdentityKind};

use super::RecordFailure;

/// The record's own table, named once.
pub const TABLE: &str = "catch_up_notice";
/// Its lines' table, named once.
pub const ENTRY_TABLE: &str = "catch_up_notice_entry";

/// The four kinds of line, as the table spells them — `Queue & History` C1's
/// list.
const ENDED: &str = "ended";
const RE_RUN: &str = "re-run";
const HELD_JOB_DROPPED: &str = "held job dropped";
const DROPPED_FOR_A_REMOVED_BLENDER: &str = "dropped for a removed blender";

/// `Queue & History` E2's four endings, as the table spells them.
fn the_ending_as_written(ending: Ending) -> &'static str {
    match ending {
        Ending::Finished => "finished",
        Ending::Failed => "failed",
        Ending::Stopped => "stopped",
        Ending::GivenUp => "given up",
    }
}

fn the_ending_read_back(text: &str) -> Option<Ending> {
    Ending::ALL
        .into_iter()
        .find(|ending| the_ending_as_written(*ending) == text)
}

/// **The record standing for one machine, or none.**
pub fn the_standing_notice(
    connection: &rusqlite::Connection,
    server: &Identity,
) -> Result<Option<NoticeRecord>, RecordFailure> {
    let incomplete: Option<bool> = {
        let mut asking = connection
            .prepare("SELECT the_account_is_incomplete FROM catch_up_notice WHERE server = ?1")?;
        let mut rows = asking.query([server.as_text()])?;
        match rows.next()? {
            Some(row) => Some(row.get::<_, i64>(0)? == 1),
            None => None,
        }
    };
    let Some(the_account_is_incomplete) = incomplete else {
        return Ok(None);
    };
    let mut asking = connection.prepare(
        "SELECT job, sequence, kind, ending, attempt, frame, why, name, version
         FROM catch_up_notice_entry WHERE server = ?1 ORDER BY place, sequence",
    )?;
    let rows = asking.query_map([server.as_text()], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<String>>(3)?,
            row.get::<_, Option<i64>>(4)?,
            row.get::<_, Option<i64>>(5)?,
            row.get::<_, Option<String>>(6)?,
            row.get::<_, Option<String>>(7)?,
            row.get::<_, Option<String>>(8)?,
        ))
    })?;
    let mut entries = Vec::new();
    for row in rows {
        let (job, sequence, kind, ending, attempt, frame, why, name, version) = row?;
        let Some(job) = Identity::read_back(IdentityKind::Job, &job) else {
            continue;
        };
        let sequence = sequence as u64;
        let entry = match kind.as_str() {
            ENDED => match ending.as_deref().and_then(the_ending_read_back) {
                Some(ending) => ANoticeEntry::Ended { job, ending, sequence },
                None => continue,
            },
            RE_RUN => ANoticeEntry::InterruptedAndReRunFrom {
                job,
                attempt: attempt.unwrap_or(0) as u32,
                frame,
                sequence,
            },
            HELD_JOB_DROPPED => ANoticeEntry::HeldJobDropped {
                job,
                name: name.unwrap_or_default(),
                why: why.unwrap_or_default(),
                sequence,
            },
            DROPPED_FOR_A_REMOVED_BLENDER => ANoticeEntry::WaitingJobDroppedForARemovedBlender {
                job,
                a_bounded_rendering_of_the_label: name.unwrap_or_default(),
                the_version: version.unwrap_or_default(),
                sequence,
            },
            _ => continue,
        };
        entries.push(entry);
    }
    Ok(Some(NoticeRecord {
        server: server.clone(),
        entries,
        the_account_is_incomplete,
    }))
}

/// **Every standing, undismissed record at this moment**, in one order the
/// draw can rely on — what `Queue & History` C2a's one message is composed
/// over.
pub fn every_standing_notice(
    connection: &rusqlite::Connection,
) -> Result<Vec<NoticeRecord>, RecordFailure> {
    let servers: Vec<String> = {
        let mut asking = connection.prepare("SELECT server FROM catch_up_notice ORDER BY server")?;
        let rows = asking.query_map([], |row| row.get::<_, String>(0))?;
        rows.collect::<Result<_, _>>()?
    };
    let mut out = Vec::new();
    for server in servers {
        let Some(server) = Identity::read_back(IdentityKind::LocalServer, &server) else {
            continue;
        };
        if let Some(record) = the_standing_notice(connection, &server)? {
            out.push(record);
        }
    }
    Ok(out)
}

/// **`Queue & History` C2: the record is written whole, in one act** — the record and every line
/// of it, or nothing. A write that is refused leaves what stood before, and
/// the caller moves no cursor.
pub fn the_notice_is_written(
    connection: &rusqlite::Connection,
    record: &NoticeRecord,
) -> Result<(), RecordFailure> {
    connection.execute_batch("SAVEPOINT the_catch_up_notice")?;
    let attempt = (|| -> Result<(), RecordFailure> {
        let server = record.server.as_text();
        connection.execute("DELETE FROM catch_up_notice_entry WHERE server = ?1", [&server])?;
        connection.execute(
            "INSERT INTO catch_up_notice (server, the_account_is_incomplete) VALUES (?1, ?2)
             ON CONFLICT (server) DO UPDATE SET the_account_is_incomplete = excluded.the_account_is_incomplete",
            rusqlite::params![server, record.the_account_is_incomplete as i64],
        )?;
        for (place, entry) in record.entries.iter().enumerate() {
            let (job, sequence) = entry.recognised_by();
            let (kind, ending, attempt, frame, why, name, version): (
                &str,
                Option<&str>,
                Option<i64>,
                Option<i64>,
                Option<&str>,
                Option<&str>,
                Option<&str>,
            ) = match entry {
                ANoticeEntry::Ended { ending, .. } => {
                    (ENDED, Some(the_ending_as_written(*ending)), None, None, None, None, None)
                }
                ANoticeEntry::InterruptedAndReRunFrom { attempt, frame, .. } => {
                    (RE_RUN, None, Some(*attempt as i64), *frame, None, None, None)
                }
                ANoticeEntry::HeldJobDropped { name, why, .. } => {
                    (HELD_JOB_DROPPED, None, None, None, Some(why), Some(name), None)
                }
                ANoticeEntry::WaitingJobDroppedForARemovedBlender {
                    a_bounded_rendering_of_the_label,
                    the_version,
                    ..
                } => (
                    DROPPED_FOR_A_REMOVED_BLENDER,
                    None,
                    None,
                    None,
                    None,
                    Some(a_bounded_rendering_of_the_label),
                    Some(the_version),
                ),
            };
            connection.execute(
                "INSERT INTO catch_up_notice_entry
                     (server, job, sequence, place, kind, ending, attempt, frame, why, name, version)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                rusqlite::params![
                    server,
                    job.as_text(),
                    sequence as i64,
                    place as i64,
                    kind,
                    ending,
                    attempt,
                    frame,
                    why,
                    name,
                    version
                ],
            )?;
        }
        Ok(())
    })();
    match attempt {
        Ok(()) => {
            connection.execute_batch("RELEASE the_catch_up_notice")?;
            Ok(())
        }
        Err(failure) => {
            let _ = connection.execute_batch("ROLLBACK TO the_catch_up_notice; RELEASE the_catch_up_notice");
            Err(failure)
        }
    }
}

/// **`Queue & History` C2's lifetime, and the user's own act**: the record and its lines go,
/// in one act.
pub fn the_notice_is_removed(
    connection: &rusqlite::Connection,
    server: &Identity,
) -> Result<(), RecordFailure> {
    connection.execute_batch("SAVEPOINT the_catch_up_notice_goes")?;
    let attempt = (|| -> Result<(), RecordFailure> {
        connection.execute("DELETE FROM catch_up_notice_entry WHERE server = ?1", [server.as_text()])?;
        connection.execute("DELETE FROM catch_up_notice WHERE server = ?1", [server.as_text()])?;
        Ok(())
    })();
    match attempt {
        Ok(()) => {
            connection.execute_batch("RELEASE the_catch_up_notice_goes")?;
            Ok(())
        }
        Err(failure) => {
            let _ = connection
                .execute_batch("ROLLBACK TO the_catch_up_notice_goes; RELEASE the_catch_up_notice_goes");
            Err(failure)
        }
    }
}

/// **`Queue & History`'s own seam onto this record** —
/// `smoozy_queue_history::acting::catching_up::KeepsTheCatchUpHalf`, so the
/// dismissal that crate already performs is the one performed here.
///
/// **The cursor half is the server row's own column**, which `State, identity
/// and recovery` E2's cursor already lives in (the desktop's *next to read*,
/// so *past* is one less), **taken under the installation identity the row
/// last confirmed**. Nothing on the desktop advances the cursor through this
/// seam — step 3's own pass does, in `src/servers/the_machines_work.rs` — and
/// the two halves are here because the seam is one trait.
pub struct TheCatchUpHalf<'c> {
    connection: &'c rusqlite::Connection,
}

impl<'c> TheCatchUpHalf<'c> {
    pub fn over(connection: &'c rusqlite::Connection) -> Self {
        TheCatchUpHalf { connection }
    }
}

fn said(failure: RecordFailure) -> String {
    format!("{failure:?}")
}

impl KeepsTheCatchUpHalf for TheCatchUpHalf<'_> {
    fn the_standing_notice(&self, server: &Identity) -> Option<NoticeRecord> {
        the_standing_notice(self.connection, server).ok().flatten()
    }

    fn write_the_notice(&mut self, record: &NoticeRecord) -> Result<(), String> {
        the_notice_is_written(self.connection, record).map_err(said)
    }

    fn remove_the_notice(&mut self, server: &Identity) -> Result<(), String> {
        the_notice_is_removed(self.connection, server).map_err(said)
    }

    fn every_standing_notice(&self) -> Vec<NoticeRecord> {
        every_standing_notice(self.connection).unwrap_or_default()
    }

    fn the_cursor(&self, server: &Identity) -> Option<EventCursor> {
        let row = super::server_rows::one(self.connection, &server.as_text()).ok()??;
        let next = row.event_cursor.filter(|next| *next > 0)?;
        let taken_under = Identity::read_back(
            IdentityKind::Installation,
            row.last_confirmed_installation_identity.as_deref()?,
        )?;
        Some(EventCursor {
            taken_under,
            past_sequence_number: next - 1,
        })
    }

    fn advance_the_cursor(&mut self, server: &Identity, cursor: &EventCursor) -> Result<(), String> {
        let mut row = super::server_rows::one(self.connection, &server.as_text())
            .map_err(said)?
            .ok_or_else(|| "no such server row".to_string())?;
        row.event_cursor = Some(cursor.past_sequence_number.saturating_add(1));
        super::server_rows::write(self.connection, &row).map_err(said)
    }
}

/// **Opening History takes every ending off every standing record**
/// (`Queue & History` C2a as changed 2026-10-12, the Creator's ninth notes,
/// note 3), through `Queue & History`'s own act: a record left naming nothing
/// is dismissed, and one naming a dropped job stands with that line alone.
/// *(Until then this file's one dismissal was the whole message, every record
/// behind the banner's *Dismiss*; the message as one dismissable whole is gone
/// with the banner.)*
pub fn history_is_opened(connection: &rusqlite::Connection) -> Result<(), String> {
    smoozy_queue_history::acting::catching_up::history_was_opened(&mut TheCatchUpHalf::over(connection))
}

/// **The user's *Dismiss* on one dropped job's line** takes that one name off
/// the record that holds it (`Queue & History` C2a as changed 2026-10-12),
/// through `Queue & History`'s own act.
pub fn the_dropped_line_is_dismissed(connection: &rusqlite::Connection, job: &Identity) -> Result<(), String> {
    smoozy_queue_history::acting::catching_up::the_dropped_line_is_dismissed(&mut TheCatchUpHalf::over(connection), job)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_record() -> rusqlite::Connection {
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

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).unwrap()
    }

    /// **`Queue & History` C2: every kind of line is written and read back as it was composed**,
    /// in the order it was composed, a re-run with no frame yet included; the
    /// gap's statement stands on a record with no line; and a second write of
    /// the record replaces the first rather than adding to it.
    #[test]
    fn a_notice_is_read_back_as_it_was_written_in_its_order() {
        let connection = a_record();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let other = Identity::issue(IdentityKind::LocalServer).unwrap();
        assert_eq!(the_standing_notice(&connection, &server).unwrap(), None);
        let record = NoticeRecord {
            server: server.clone(),
            entries: vec![
                ANoticeEntry::InterruptedAndReRunFrom { job: a_job(), attempt: 2, frame: None, sequence: 9 },
                ANoticeEntry::Ended { job: a_job(), ending: Ending::GivenUp, sequence: 3 },
                ANoticeEntry::HeldJobDropped {
                    job: a_job(),
                    name: "Kitchen — final v4".into(),
                    why: "the hold's release outcome was written: dropped".into(),
                    sequence: 4,
                },
                ANoticeEntry::WaitingJobDroppedForARemovedBlender {
                    job: a_job(),
                    a_bounded_rendering_of_the_label: "Attic".into(),
                    the_version: "4.5.3".into(),
                    sequence: 5,
                },
                ANoticeEntry::InterruptedAndReRunFrom { job: a_job(), attempt: 2, frame: Some(61), sequence: 6 },
            ],
            the_account_is_incomplete: false,
        };
        the_notice_is_written(&connection, &record).unwrap();
        let gap = NoticeRecord { server: other.clone(), entries: Vec::new(), the_account_is_incomplete: true };
        the_notice_is_written(&connection, &gap).unwrap();
        assert_eq!(the_standing_notice(&connection, &server).unwrap().as_ref(), Some(&record));
        assert_eq!(the_standing_notice(&connection, &other).unwrap().as_ref(), Some(&gap));
        assert_eq!(every_standing_notice(&connection).unwrap().len(), 2);

        let mut grown = record.clone();
        grown.entries.truncate(1);
        the_notice_is_written(&connection, &grown).unwrap();
        assert_eq!(the_standing_notice(&connection, &server).unwrap(), Some(grown));
    }

    /// **`Queue & History` C2a as changed 2026-10-12: opening History takes
    /// the endings off every record and dismisses one left naming nothing; a
    /// dropped job's line goes only by its own *Dismiss*; and a removal takes
    /// one record alone** — over the real tables, so a line never outlives
    /// its record.
    #[test]
    fn opening_history_and_a_dropped_lines_dismiss_each_take_their_own_and_a_removal_takes_one() {
        let connection = a_record();
        let one = Identity::issue(IdentityKind::LocalServer).unwrap();
        let two = Identity::issue(IdentityKind::LocalServer).unwrap();
        let three = Identity::issue(IdentityKind::LocalServer).unwrap();
        let kitchen = a_job();
        let dropped = ANoticeEntry::HeldJobDropped {
            job: kitchen.clone(),
            name: "Kitchen — final v4".into(),
            why: "the hold's release outcome was written: dropped".into(),
            sequence: 2,
        };
        for (server, entries) in [
            (&one, vec![ANoticeEntry::Ended { job: a_job(), ending: Ending::Finished, sequence: 1 }, dropped.clone()]),
            (&two, vec![ANoticeEntry::Ended { job: a_job(), ending: Ending::Failed, sequence: 1 }]),
            (&three, vec![ANoticeEntry::Ended { job: a_job(), ending: Ending::Finished, sequence: 1 }]),
        ] {
            the_notice_is_written(
                &connection,
                &NoticeRecord { server: server.clone(), entries, the_account_is_incomplete: false },
            )
            .unwrap();
        }
        the_notice_is_removed(&connection, &three).unwrap();
        assert_eq!(the_standing_notice(&connection, &three).unwrap(), None);
        assert!(the_standing_notice(&connection, &two).unwrap().is_some());

        history_is_opened(&connection).unwrap();
        assert_eq!(the_standing_notice(&connection, &two).unwrap(), None, "a record naming nothing stands");
        assert_eq!(
            the_standing_notice(&connection, &one).unwrap().map(|record| record.entries),
            Some(vec![dropped]),
            "the ending stayed, or the dropped job's line went with History"
        );

        the_dropped_line_is_dismissed(&connection, &kitchen).unwrap();
        assert!(every_standing_notice(&connection).unwrap().is_empty());
        let lines: i64 = connection
            .query_row("SELECT count(*) FROM catch_up_notice_entry", [], |row| row.get(0))
            .unwrap();
        assert_eq!(lines, 0, "a line outlived its record");
    }
}

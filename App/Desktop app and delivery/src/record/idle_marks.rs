//! **`Queue & History` W5's idle-spoken mark, in the local record** — the
//! table record version 33 made, out of the 38's piece 1 (2026-10-03).
//!
//! **The rule is `Queue & History`'s and the store is this document's**, the
//! split every row module here draws. `Queue & History` W5: *"Before speaking, the mark is looked
//! up; present for this stretch means already spoken."* **What is here is the
//! read and the write and nothing else** — the judgement is
//! `smoozy_queue_history::idle::may_speak`, and the one act that writes a mark
//! is `smoozy_queue_history::acting::idle_recheck::the_recheck`.
//!
//! **Subject-held on the server row** (`Queue & History` W5): the row's removal
//! walk takes every mark of the server with it, under
//! `smoozy_servers::store::KeyedByTheServer::IdleSpokenMarks`.
//!
//! **`State, identity and recovery`'s own markers are written out with that
//! document at every site below.** `Queue & History` W2 and W5 are that
//! document's and are written with it.

use smoozy_queue_history::idle::{IdleSpokenMark, StretchKey};
use smoozy_state::identities::{Identity, IdentityKind};

use super::RecordFailure;

/// The one table, named once.
pub const TABLE: &str = "idle_spoken_mark";

/// **Every mark this server holds.**
///
/// **A mark whose identities will not read back is left out of the answer**,
/// and that costs at most one repeated telling — the cheaper of `Queue & History` W5's two
/// failures, the other being a machine billing for nothing that is never told
/// about.
pub fn the_marks_for(
    connection: &rusqlite::Connection,
    server: &Identity,
) -> Result<Vec<IdleSpokenMark>, RecordFailure> {
    let mut asking = connection
        .prepare("SELECT installation, opening_event FROM idle_spoken_mark WHERE server = ?1")?;
    let rows = asking.query_map([server.as_text()], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?))
    })?;
    let mut out = Vec::new();
    for row in rows {
        let (installation, opening_event) = row?;
        let Some(installation) = Identity::read_back(IdentityKind::Installation, &installation)
        else {
            continue;
        };
        let opening_event = match opening_event {
            None => None,
            Some(text) => match Identity::read_back(IdentityKind::Event, &text) {
                Some(event) => Some(event),
                None => continue,
            },
        };
        out.push(IdleSpokenMark {
            server: server.clone(),
            stretch: StretchKey {
                installation,
                opening_event,
            },
        });
    }
    Ok(out)
}

/// **The mark is written** — by the act of speaking and by nothing else
/// (`Queue & History` W5). **One small write**, atomic by itself (`Queue &
/// History`'s own table of writes). A mark already standing for the stretch is
/// not written twice.
pub fn the_mark_is_written(
    connection: &rusqlite::Connection,
    mark: &IdleSpokenMark,
) -> Result<(), RecordFailure> {
    let opening = mark.stretch.opening_event.as_ref().map(Identity::as_text);
    connection.execute(
        "INSERT INTO idle_spoken_mark (server, installation, opening_event)
         SELECT ?1, ?2, ?3
         WHERE NOT EXISTS (
             SELECT 1 FROM idle_spoken_mark
             WHERE server = ?1 AND installation = ?2 AND opening_event IS ?3
         )",
        rusqlite::params![
            mark.server.as_text(),
            mark.stretch.installation.as_text(),
            opening
        ],
    )?;
    Ok(())
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

    fn a_mark(server: &Identity, opening: Option<Identity>) -> IdleSpokenMark {
        IdleSpokenMark {
            server: server.clone(),
            stretch: StretchKey {
                installation: Identity::issue(IdentityKind::Installation).unwrap(),
                opening_event: opening,
            },
        }
    }

    /// **`Queue & History` W5: a mark written is read back for its server, and only its
    /// server**, the floor's absent opening event included — and a second write
    /// of the same stretch adds nothing.
    #[test]
    fn a_mark_is_read_back_for_its_server_once() {
        let connection = a_record();
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let other = Identity::issue(IdentityKind::LocalServer).unwrap();
        let opened = a_mark(&server, Some(Identity::issue(IdentityKind::Event).unwrap()));
        let floor = a_mark(&server, None);
        for mark in [&opened, &floor, &opened, &floor] {
            the_mark_is_written(&connection, mark).unwrap();
        }
        the_mark_is_written(&connection, &a_mark(&other, None)).unwrap();

        let read = the_marks_for(&connection, &server).unwrap();
        assert_eq!(read.len(), 2, "one mark per stretch: {read:?}");
        assert!(read.contains(&opened));
        assert!(read.contains(&floor));
        assert_eq!(the_marks_for(&connection, &other).unwrap().len(), 1);
    }
}

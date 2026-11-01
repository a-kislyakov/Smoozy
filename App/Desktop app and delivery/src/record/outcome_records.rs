//! The outcome record in the local record — `Desktop app and delivery` §5 and
//! `State, identity and recovery` §9.4.
//!
//! **The shape is that document's and the store is this one's**, the same
//! split `Security and licence` §1.4 B30 draws for the secret adapter and the
//! same one `key_rows` already stands on. Nothing here decides what an outcome
//! record holds.
//!
//! **This is a shared record rather than any module's.** Every action whose
//! partition units are smaller than itself writes one — `Keys`' three at step
//! 3, `File Browser` U1's upload request, `Download` M1's request, and the
//! rest as they arrive. Each of those documents adds its own fields to its own
//! record; what is here is the part §9.4 fixes for all of them, which is the
//! enumeration and the outcomes.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below. **Where a marker is a clause of this crate's
//! own plan *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there resolves to a
//! real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Security and licence` B30. `File Browser` U1.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **This crate's own plan defines an F19, a P5, a P7 and an M1 of its own**,
//! so none of those is listed above. **That the document is written out
//! includes the P7 inside a test's own assertion message**, written out there
//! on 2026-09-04 so the message a failing test prints names the document as
//! well as the marker.
//!
//! **A2 holds: no module has a durable store of its own.**

use rusqlite::OptionalExtension;
use smoozy_state::identities::Identity;
use smoozy_state::outcomes::{self, Keeps, Recorded, Refused, UnitOutcome};

/// The keeper, over one open local record.
pub struct OutcomeRecords<'a> {
    connection: &'a rusqlite::Connection,
}

impl<'a> OutcomeRecords<'a> {
    pub fn over(connection: &'a rusqlite::Connection) -> Self {
        OutcomeRecords { connection }
    }
}

/// The engine's own words, wrapped as the store's trouble. **Nothing here
/// turns an engine failure into a rule of §9.4** — the two are separate
/// members of [`Refused`] for exactly that reason.
pub(super) fn the_store(error: rusqlite::Error) -> Refused {
    Refused::TheStore(error.to_string())
}

/// How an outcome is written down. **Three words and no more**, matching
/// §9.4's closed list of three.
pub(super) fn as_text(outcome: &UnitOutcome) -> (&'static str, Option<String>) {
    match outcome {
        UnitOutcome::Done => ("done", None),
        UnitOutcome::Failed => ("failed", None),
        UnitOutcome::NotAttempted { stopped_by } => ("not attempted", Some(stopped_by.clone())),
    }
}

/// Read one back. **A row whose outcome text is none of the three is read as
/// no outcome at all** rather than guessed at — the same refusal
/// [`super::key_rows`] makes for an identifier that will not read back.
pub(super) fn from_text(outcome: Option<String>, stopped_by: Option<String>) -> Option<UnitOutcome> {
    match outcome.as_deref() {
        Some("done") => Some(UnitOutcome::Done),
        Some("failed") => Some(UnitOutcome::Failed),
        Some("not attempted") => Some(UnitOutcome::NotAttempted {
            stopped_by: stopped_by.unwrap_or_default(),
        }),
        _ => None,
    }
}

impl Keeps for OutcomeRecords<'_> {
    /// **`State, identity and recovery` P5 item 1, at that document's §9.4:
    /// the enumeration,
    /// durable before the first unit is attempted.**
    ///
    /// **The identity and every unit go down in one transaction**, so an
    /// interruption inside this call leaves either the whole enumeration or
    /// none of it. A record holding half its units would say a unit was never
    /// owed, which is the one thing item 1 exists to prevent.
    fn enumerate(&mut self, action: &str, units: &[&str]) -> Result<Identity, Refused> {
        let identity = outcomes::issue().map_err(|_| {
            Refused::TheStore("this computer offered no randomness for an identity".into())
        })?;
        let text = identity.as_text();

        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let written = (|| -> Result<(), rusqlite::Error> {
            self.connection.execute(
                "INSERT INTO outcome_record (identity, action) VALUES (?1, ?2)",
                rusqlite::params![text, action],
            )?;
            for (position, role) in units.iter().enumerate() {
                self.connection.execute(
                    "INSERT INTO outcome_record_unit (record, position, role, outcome, stopped_by)
                     VALUES (?1, ?2, ?3, NULL, NULL)",
                    rusqlite::params![text, position as i64, role],
                )?;
            }
            Ok(())
        })();
        match written {
            Ok(()) => {
                self.connection.execute("COMMIT", []).map_err(the_store)?;
                Ok(identity)
            }
            Err(error) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(the_store(error))
            }
        }
    }

    /// **`State, identity and recovery` P5 item 2, at that document's §9.4,
    /// and `State, identity and recovery` F19's immutability at its §8.6.**
    ///
    /// The update is conditional on the outcome still being absent, and the
    /// count it returns is what says which of the two refusals applies. **The
    /// check and the write are one statement** rather than a read followed by
    /// a write, because two statements could be interleaved with another
    /// caller's and let a second outcome land.
    fn a_unit_ended(
        &mut self,
        record: &Identity,
        unit: &str,
        outcome: UnitOutcome,
    ) -> Result<(), Refused> {
        let (outcome, stopped_by) = as_text(&outcome);
        let changed = self
            .connection
            .execute(
                "UPDATE outcome_record_unit SET outcome = ?1, stopped_by = ?2
                 WHERE record = ?3 AND role = ?4 AND outcome IS NULL",
                rusqlite::params![outcome, stopped_by, record.as_text(), unit],
            )
            .map_err(the_store)?;
        if changed > 0 {
            return Ok(());
        }
        // Nothing moved. Either the unit was never owed, or its ending has
        // already been written and is not this write's to change.
        let owed: bool = self
            .connection
            .query_row(
                "SELECT count(*) > 0 FROM outcome_record_unit WHERE record = ?1 AND role = ?2",
                rusqlite::params![record.as_text(), unit],
                |row| row.get(0),
            )
            .map_err(the_store)?;
        Err(if owed {
            Refused::ThatUnitHasAlreadyEnded
        } else {
            Refused::ThatUnitWasNotOwed
        })
    }

    fn read(&self, record: &Identity) -> Result<Option<Recorded>, Refused> {
        let text = record.as_text();
        let action: Option<String> = self
            .connection
            .query_row(
                "SELECT action FROM outcome_record WHERE identity = ?1",
                [&text],
                |row| row.get(0),
            )
            .optional()
            .map_err(the_store)?;
        let Some(action) = action else {
            return Ok(None);
        };

        let mut statement = self
            .connection
            .prepare(
                "SELECT role, outcome, stopped_by FROM outcome_record_unit
                 WHERE record = ?1 ORDER BY position",
            )
            .map_err(the_store)?;
        let rows = statement
            .query_map([&text], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                ))
            })
            .map_err(the_store)?;

        let mut units = Vec::new();
        for row in rows {
            let (role, outcome, stopped_by) = row.map_err(the_store)?;
            units.push((role, from_text(outcome, stopped_by)));
        }
        Ok(Some(Recorded {
            identity: record.clone(),
            action,
            units,
        }))
    }

    /// **§11.1: user-held.** The record and its units go together, in one
    /// transaction — a record whose units outlived it would be an enumeration
    /// belonging to nothing.
    fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
        let text = record.as_text();
        self.connection
            .execute("BEGIN IMMEDIATE", [])
            .map_err(the_store)?;
        let removed = (|| -> Result<(), rusqlite::Error> {
            self.connection
                .execute("DELETE FROM outcome_record_unit WHERE record = ?1", [&text])?;
            self.connection
                .execute("DELETE FROM outcome_record WHERE identity = ?1", [&text])?;
            Ok(())
        })();
        match removed {
            Ok(()) => {
                self.connection.execute("COMMIT", []).map_err(the_store)?;
                Ok(())
            }
            Err(error) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(the_store(error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::outcomes::{ActionOutcome, Writing};

    fn a_record() -> rusqlite::Connection {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        for statement in super::super::schema::VERSION_1
            .iter()
            .chain(super::super::schema::VERSION_2)
            .chain(super::super::schema::VERSION_3)
        {
            connection.execute(statement, []).unwrap();
        }
        connection
    }

    #[test]
    fn the_enumeration_lands_before_a_single_outcome_does() {
        let connection = a_record();
        let mut keeper = OutcomeRecords::over(&connection);
        let identity = keeper
            .enumerate("delete a managed key", &["the private half", "the erasure"])
            .unwrap();

        let read = keeper.read(&identity).unwrap().unwrap();
        assert_eq!(read.action, "delete a managed key");
        assert_eq!(
            read.units
                .iter()
                .map(|(role, _)| role.as_str())
                .collect::<Vec<_>>(),
            vec!["the private half", "the erasure"]
        );
        assert!(read.units.iter().all(|(_, outcome)| outcome.is_none()));
        assert_eq!(read.the_actions_outcome(), None);
    }

    #[test]
    fn an_outcome_is_written_once_and_the_store_refuses_the_second() {
        let connection = a_record();
        let mut keeper = OutcomeRecords::over(&connection);
        let identity = keeper.enumerate("generation", &["the key pair"]).unwrap();

        keeper
            .a_unit_ended(&identity, "the key pair", UnitOutcome::Done)
            .unwrap();
        assert_eq!(
            keeper.a_unit_ended(&identity, "the key pair", UnitOutcome::Failed),
            Err(Refused::ThatUnitHasAlreadyEnded)
        );
        assert_eq!(
            keeper.a_unit_ended(&identity, "a unit nobody owed", UnitOutcome::Done),
            Err(Refused::ThatUnitWasNotOwed)
        );
        // And the value that stands is the first one written.
        let read = keeper.read(&identity).unwrap().unwrap();
        assert_eq!(read.units[0].1, Some(UnitOutcome::Done));
    }

    #[test]
    fn the_whole_act_reads_back_across_a_reopening() {
        let connection = a_record();
        let identity = {
            let mut keeper = OutcomeRecords::over(&connection);
            let mut writing = Writing::open(
                &mut keeper,
                "delete a managed key",
                &["the private half", "the public half", "the erasure"],
            )
            .unwrap();
            writing
                .a_unit_ended("the private half", UnitOutcome::Failed)
                .unwrap();
            writing
                .the_rest_were_not_attempted("the private half")
                .unwrap();
            writing.identity().clone()
        };

        // A fresh keeper over the same record — which is what the next
        // presentation is, after an application run has ended.
        let keeper = OutcomeRecords::over(&connection);
        let read = keeper.read(&identity).unwrap().unwrap();
        assert_eq!(read.the_actions_outcome(), Some(ActionOutcome::Nothing));
        assert_eq!(
            read.what_could_not_be_done()
                .iter()
                .map(|(role, _)| *role)
                .collect::<Vec<_>>(),
            vec!["the private half", "the public half", "the erasure"]
        );
        assert_eq!(
            read.units[2].1,
            Some(UnitOutcome::NotAttempted {
                stopped_by: "the private half".to_string()
            })
        );
    }

    #[test]
    fn a_unit_whose_ending_was_lost_reads_as_unfinished_and_not_as_failed() {
        let connection = a_record();
        let mut keeper = OutcomeRecords::over(&connection);
        let identity = keeper
            .enumerate("generation", &["the key pair", "the row", "the store put"])
            .unwrap();
        keeper
            .a_unit_ended(&identity, "the key pair", UnitOutcome::Done)
            .unwrap();
        // The application stops here.
        let read = keeper.read(&identity).unwrap().unwrap();
        assert_eq!(read.unfinished().collect::<Vec<_>>(), vec!["the row", "the store put"]);
        assert!(read.what_could_not_be_done().is_empty());
        assert_eq!(read.the_actions_outcome(), None);
    }

    #[test]
    fn the_user_removing_it_takes_the_units_with_it() {
        let connection = a_record();
        let mut keeper = OutcomeRecords::over(&connection);
        let identity = keeper.enumerate("an act", &["a", "b"]).unwrap();
        keeper.the_user_removed_it(&identity).unwrap();

        assert!(keeper.read(&identity).unwrap().is_none());
        let left: i64 = connection
            .query_row("SELECT count(*) FROM outcome_record_unit", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(left, 0);
    }

    #[test]
    fn two_records_of_one_action_kind_do_not_touch_each_other() {
        let connection = a_record();
        let mut keeper = OutcomeRecords::over(&connection);
        let first = keeper.enumerate("generation", &["the key pair"]).unwrap();
        let second = keeper.enumerate("generation", &["the key pair"]).unwrap();
        keeper
            .a_unit_ended(&first, "the key pair", UnitOutcome::Failed)
            .unwrap();

        assert_eq!(
            keeper.read(&second).unwrap().unwrap().units[0].1,
            None,
            "a retry is a new action with a new record \
             (`State, identity and recovery` P7, at that document's §9.5)"
        );
        keeper
            .a_unit_ended(&second, "the key pair", UnitOutcome::Done)
            .unwrap();
        assert_eq!(
            keeper.read(&first).unwrap().unwrap().units[0].1,
            Some(UnitOutcome::Failed)
        );
    }
}

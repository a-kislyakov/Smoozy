//! **`File Browser` §11's record for a delete, a move and a copy** — the
//! module's own fields on the `State, identity and recovery` §9.4 outcome
//! record, as `File Browser` U1's are for an upload. Step 10c's Files' owed
//! acts piece 4 (record version 31).
//!
//! **One enumeration, joined rather than copied.** The shared
//! `outcome_record` and `outcome_record_unit` hold the act and each unit's
//! outcome by position; the tables here hold, at the same position, the
//! unit's source, its destination and — for a copy — its staging path, which
//! is what `File Browser` §11 means by *the entry at that position already
//! holds them*. **The act's own outcome is in neither**: it is derived from
//! the units at every reading (`State, identity and recovery` P5 item 3).
//!
//! **The telling is rows and its emptiness is its absence** (`File Browser`
//! W8): the affected jobs are fields of this record and never of a second one,
//! and an act that disturbed nothing carries none.
//!
//! **User-held** (`State, identity and recovery` §11.1): nothing here removes
//! a record by age.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `File Browser` U1, W8.
//!
//! **This crate's own plan defines a P5, an M7 and an F19 of its own**, so
//! those are written out with their document at every site instead.

use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::outcomes::{Refused, UnitOutcome};

use super::outcome_records::{as_text, from_text, the_store};

/// **Which act**, from `File Browser` §11.1 `File Browser` B3's closed set, as the record
/// and the shared outcome record's action name keep it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichAct {
    Delete,
    Move,
    Copy,
}

impl WhichAct {
    pub fn as_text(self) -> &'static str {
        match self {
            WhichAct::Delete => "delete",
            WhichAct::Move => "move",
            WhichAct::Copy => "copy",
        }
    }

    fn from_text(text: &str) -> Option<WhichAct> {
        match text {
            "delete" => Some(WhichAct::Delete),
            "move" => Some(WhichAct::Move),
            "copy" => Some(WhichAct::Copy),
            _ => None,
        }
    }
}

/// **One unit, at its position** — `File Browser` §11's role in the module's
/// own words, and the paths the entry at that position holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AUnit {
    /// `File Browser` §11's partition unit, word for word: *one entry*, *one
    /// top-level item*, *one file's (copy, delete) pair*, *one file*.
    pub role: &'static str,
    pub source: Vec<u8>,
    pub destination: Option<Vec<u8>>,
    /// `File Browser` M7's copy staging path, for a copy.
    pub staging: Option<Vec<u8>>,
}

/// **The act as it is written before its first machine effect.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAct {
    pub act: WhichAct,
    pub server_row: Identity,
    pub installation: Option<Identity>,
    pub opened_by_run: Identity,
    pub units: Vec<AUnit>,
    /// `File Browser` W8's telling: the affected jobs, none where no live-job
    /// warning was pushed past.
    pub the_jobs_told: Vec<Identity>,
}

/// **An act read back**, with each unit's outcome where it was written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnActRead {
    pub record: Identity,
    pub act: WhichAct,
    pub server_row: Identity,
    pub units: Vec<(AUnit, Option<UnitOutcome>)>,
    pub the_jobs_told: Vec<Identity>,
}

/// The keeper, over one open local record.
pub struct FileActs<'a> {
    connection: &'a rusqlite::Connection,
}

impl<'a> FileActs<'a> {
    pub fn over(connection: &'a rusqlite::Connection) -> Self {
        FileActs { connection }
    }

    /// **`State, identity and recovery` P5 item 1 and `File Browser` W8, in
    /// one write**: the act's identity, its whole enumeration, its telling,
    /// and each affected job's mark (`marks`, reading *may have been
    /// disturbed*) — before the act's first machine effect. **Both or
    /// neither**: a refused write is the act not attempted.
    pub fn the_act_is_written(
        &self,
        act: &AnAct,
        marks: &[(Identity, smoozy_queue_history::record::ADisturbance)],
    ) -> Result<Identity, Refused> {
        let record = Identity::issue(IdentityKind::Operation)
            .map_err(|_| Refused::TheStore("this computer offered no randomness for an identity".into()))?;
        let text = record.as_text();
        self.connection
            .execute_batch("SAVEPOINT a_file_act_is_written")
            .map_err(the_store)?;
        let written = (|| -> rusqlite::Result<()> {
            self.connection.execute(
                "INSERT INTO outcome_record (identity, action) VALUES (?1, ?2)",
                rusqlite::params![text, act.act.as_text()],
            )?;
            self.connection.execute(
                "INSERT INTO file_act (record, act, server_row, installation, opened_by_run)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![
                    text,
                    act.act.as_text(),
                    act.server_row.as_text(),
                    act.installation.as_ref().map(Identity::as_text),
                    act.opened_by_run.as_text(),
                ],
            )?;
            let mut unit = self.connection.prepare_cached(
                "INSERT INTO outcome_record_unit (record, position, role, outcome, stopped_by)
                 VALUES (?1, ?2, ?3, NULL, NULL)",
            )?;
            let mut entry = self.connection.prepare_cached(
                "INSERT INTO file_act_unit (record, position, source, destination, staging)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
            )?;
            for (position, one) in act.units.iter().enumerate() {
                unit.execute(rusqlite::params![text, position as i64, one.role])?;
                entry.execute(rusqlite::params![
                    text,
                    position as i64,
                    one.source,
                    one.destination,
                    one.staging
                ])?;
            }
            let mut told = self.connection.prepare_cached(
                "INSERT INTO file_act_telling_job (record, position, job) VALUES (?1, ?2, ?3)",
            )?;
            for (position, job) in act.the_jobs_told.iter().enumerate() {
                told.execute(rusqlite::params![text, position as i64, job.as_text()])?;
            }
            for (job, mark) in marks {
                super::job_records::a_mark_is_appended_to(self.connection, job, mark)?;
            }
            Ok(())
        })();
        match written {
            Ok(()) => {
                self.connection
                    .execute_batch("RELEASE a_file_act_is_written")
                    .map_err(the_store)?;
                Ok(record)
            }
            Err(error) => {
                let _ = self
                    .connection
                    .execute_batch("ROLLBACK TO a_file_act_is_written; RELEASE a_file_act_is_written");
                Err(the_store(error))
            }
        }
    }

    /// **`State, identity and recovery` P5 item 2: the units that ended, at the
    /// moment they ended, in one save.** An outcome already written is never
    /// overwritten (`State, identity and recovery` F19): each write carries
    /// `outcome IS NULL`.
    pub fn these_units_ended(
        &self,
        record: &Identity,
        ended: &[(usize, UnitOutcome)],
    ) -> Result<(), Refused> {
        if ended.is_empty() {
            return Ok(());
        }
        let text = record.as_text();
        self.connection
            .execute_batch("SAVEPOINT file_act_units_ended")
            .map_err(the_store)?;
        let written = (|| -> rusqlite::Result<()> {
            let mut update = self.connection.prepare_cached(
                "UPDATE outcome_record_unit SET outcome = ?1, stopped_by = ?2
                 WHERE record = ?3 AND position = ?4 AND outcome IS NULL",
            )?;
            for (position, outcome) in ended {
                let (outcome, stopped_by) = as_text(outcome);
                update.execute(rusqlite::params![outcome, stopped_by, text, *position as i64])?;
            }
            Ok(())
        })();
        match written {
            Ok(()) => self
                .connection
                .execute_batch("RELEASE file_act_units_ended")
                .map_err(the_store),
            Err(error) => {
                let _ = self
                    .connection
                    .execute_batch("ROLLBACK TO file_act_units_ended; RELEASE file_act_units_ended");
                Err(the_store(error))
            }
        }
    }

    /// One act read back, or nothing where the record holds none under that
    /// identity. **A record whose identities will not read back is read as
    /// none** rather than given invented ones.
    pub fn read(&self, record: &Identity) -> Result<Option<AnActRead>, Refused> {
        use rusqlite::OptionalExtension;
        let text = record.as_text();
        let head: Option<(String, String)> = self
            .connection
            .query_row(
                "SELECT act, server_row FROM file_act WHERE record = ?1",
                [&text],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(the_store)?;
        let Some((act, server_row)) = head else {
            return Ok(None);
        };
        let (Some(act), Some(server_row)) = (
            WhichAct::from_text(&act),
            Identity::read_back(IdentityKind::LocalServer, &server_row),
        ) else {
            return Ok(None);
        };
        let mut statement = self
            .connection
            .prepare(
                "SELECT u.role, e.source, e.destination, e.staging, u.outcome, u.stopped_by
                 FROM file_act_unit e JOIN outcome_record_unit u
                   ON u.record = e.record AND u.position = e.position
                 WHERE e.record = ?1 ORDER BY e.position",
            )
            .map_err(the_store)?;
        let rows = statement
            .query_map([&text], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Vec<u8>>(1)?,
                    row.get::<_, Option<Vec<u8>>>(2)?,
                    row.get::<_, Option<Vec<u8>>>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                ))
            })
            .map_err(the_store)?;
        let mut units = Vec::new();
        for row in rows {
            let (role, source, destination, staging, outcome, stopped_by) = row.map_err(the_store)?;
            let role = THE_ROLES.iter().find(|one| **one == role).copied().unwrap_or("one entry");
            units.push((
                AUnit {
                    role,
                    source,
                    destination,
                    staging,
                },
                from_text(outcome, stopped_by),
            ));
        }
        let mut statement = self
            .connection
            .prepare("SELECT job FROM file_act_telling_job WHERE record = ?1 ORDER BY position")
            .map_err(the_store)?;
        let the_jobs_told = statement
            .query_map([&text], |row| row.get::<_, String>(0))
            .map_err(the_store)?
            .filter_map(|job| job.ok())
            .filter_map(|job| Identity::read_back(IdentityKind::Job, &job))
            .collect();
        Ok(Some(AnActRead {
            record: record.clone(),
            act,
            server_row,
            units,
            the_jobs_told,
        }))
    }
}

/// **`File Browser` §11's partition units, word for word**, as the role every
/// unit of one action carries.
pub const ONE_ENTRY: &str = "one entry";
pub const ONE_TOP_LEVEL_ITEM: &str = "one top-level item";
pub const ONE_FILES_COPY_DELETE_PAIR: &str = "one file's (copy, delete) pair";
pub const ONE_FILE: &str = "one file";
const THE_ROLES: [&str; 4] = [ONE_ENTRY, ONE_TOP_LEVEL_ITEM, ONE_FILES_COPY_DELETE_PAIR, ONE_FILE];

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_queue_history::record::{ADisturbance, DisturbanceReading};

    fn a_record(name: &str) -> (std::path::PathBuf, crate::record::LocalRecord) {
        let folder = std::env::temp_dir().join(format!("smoozy-file-acts-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder");
        let record =
            crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record opens");
        (folder, record)
    }

    fn an_act(jobs: Vec<Identity>) -> AnAct {
        let issue = |kind| Identity::issue(kind).expect("randomness");
        AnAct {
            act: WhichAct::Delete,
            server_row: issue(IdentityKind::LocalServer),
            installation: Some(issue(IdentityKind::Installation)),
            opened_by_run: issue(IdentityKind::Run),
            units: vec![
                AUnit { role: ONE_ENTRY, source: b"/home/ubuntu/cache/a.bphys".to_vec(), destination: None, staging: None },
                AUnit { role: ONE_ENTRY, source: b"/home/ubuntu/cache".to_vec(), destination: None, staging: None },
            ],
            the_jobs_told: jobs,
        }
    }

    /// **The enumeration lands whole before any outcome, the outcomes land
    /// once each, and the act reads back across a reopening** — with its
    /// telling.
    #[test]
    fn a_delete_is_written_whole_before_it_runs_and_reads_back_with_its_outcomes() {
        let (folder, record) = a_record("whole");
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let keeper = FileActs::over(record.connection());
        let id = keeper.the_act_is_written(&an_act(vec![job.clone()]), &[]).expect("written");
        let read = keeper.read(&id).expect("reads").expect("held");
        assert_eq!(read.act, WhichAct::Delete);
        assert_eq!(read.units.len(), 2);
        assert!(read.units.iter().all(|(_, outcome)| outcome.is_none()), "an outcome before the act ran");
        assert_eq!(read.the_jobs_told, vec![job]);
        keeper
            .these_units_ended(&id, &[(0, UnitOutcome::Done), (1, UnitOutcome::Failed)])
            .expect("the outcomes");
        // **Never overwritten.**
        keeper.these_units_ended(&id, &[(0, UnitOutcome::Failed)]).expect("a second write moves nothing");
        drop(record);
        let record =
            crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("reopens");
        let read = FileActs::over(record.connection()).read(&id).expect("reads").expect("held");
        assert_eq!(read.units[0].1, Some(UnitOutcome::Done));
        assert_eq!(read.units[1].1, Some(UnitOutcome::Failed));
        assert_eq!(read.units[0].0.source, b"/home/ubuntu/cache/a.bphys".to_vec());
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **`File Browser` W8: the telling and the marks are one write**, so a
    /// mark that cannot be written leaves no act behind it either.
    #[test]
    fn an_act_whose_marks_cannot_be_written_is_not_written_at_all() {
        let (folder, record) = a_record("both-or-neither");
        let connection = record.connection();
        connection.execute_batch("DROP TABLE job_record_disturbance").expect("the marks' table goes");
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let mark = ADisturbance {
            the_member: "/home/ubuntu/cache".to_string(),
            the_act_intended: "delete".to_string(),
            reading: DisturbanceReading::MayHaveBeenDisturbed,
            how_many: None,
        };
        assert!(FileActs::over(connection).the_act_is_written(&an_act(vec![job.clone()]), &[(job, mark)]).is_err());
        let acts: i64 = connection.query_row("SELECT count(*) FROM file_act", [], |row| row.get(0)).unwrap();
        let outcomes: i64 =
            connection.query_row("SELECT count(*) FROM outcome_record", [], |row| row.get(0)).unwrap();
        assert_eq!((acts, outcomes), (0, 0), "an act stood without its telling");
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
    }
}

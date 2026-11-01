//! The local record's database and its migrations — `Desktop app and delivery`
//! §5.
//!
//! **L1: there is one durable local record and this document builds it to the
//! inventory of `State, identity and recovery` §1.1. No row is added to that
//! inventory here.** What this module owns is the store: where it sits, how it
//! is opened, how it carries its own version, and how a version change is
//! published.
//!
//! **`Security and licence` §9 is the constraint**: the record holds no secret
//! value, is **not encrypted**, and is *"written in the per-user
//! application-data location the platform adapter names, created for the
//! owning user only, and protected by the operating system's own file
//! permissions and nothing further"*.
//!
//! **L4: no field of the record is ever placed in a filename.**

pub mod catch_up_notices;
pub mod deletions;
pub mod dependency_records;
pub mod download_records;
pub mod failure_records;
pub mod file_acts;
pub mod idle_marks;
pub mod installs_not_begun;
pub mod job_records;
pub mod kept_words;
pub mod key_rows;
pub mod migrations;
pub mod neighbouring_records;
pub mod outcome_records;
pub mod pane_widths;
pub mod previews;
pub mod schema;
pub mod server_package_trust;
pub mod server_rows;
pub mod settings;
pub mod suppressed_warnings;
pub mod submission_material;
pub mod telegram_bot;
pub mod told_record;
pub mod transfer_records;
// **The update route's three rows** — record version 49, step 11's
// update-reading piece.
pub mod update_trust;
pub mod upload_requests;

use std::path::{Path, PathBuf};

use crate::platform::files::{self, Durability};
use crate::platform::locations::{self, Location};

/// The engine, named here because a builder meeting this file should not have
/// to go looking, and pinned in `App/Cargo.toml` the way the shell is.
///
/// **A Build decision, taken at step 2**, on the properties §5 needs rather
/// than on preference. **The record at rest is one file** — its own
/// publication: *"A database in SQLite is a single disk file"*, with
/// *"Temporary journal files are created as part of transaction control, but
/// those extra files are not part of the steady-state database."* That is what
/// L7 and L8 both need: a copy is a file copy, and a migration publishes with
/// [`files::publish`]'s atomic rename.
///
/// **It is compiled in rather than linked to whatever the machine has**,
/// because the application is a plain downloaded binary on three systems and
/// nothing may be assumed present on any of them.
pub const ENGINE: &str = "SQLite, compiled into the application";

/// Why the record could not be opened, used, or migrated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordFailure {
    /// The application-data location has no path on this computer — W5's own
    /// case, arriving here.
    NoLocation { reported: String },
    /// A local write or read the adapter classified.
    File(files::Failure),
    /// **L9: the record's schema version is newer than this binary's.** It is
    /// not opened, and the fields this binary recognises are not read.
    RecordIsNewerThanTheBinary { record: u32, binary: u32 },
    /// **A moment the record's own spelling cannot hold, refused rather than
    /// written as something else.** Added 2026-09-07 from round 1 of `Access`'s
    /// code audit, finding F-9.
    ///
    /// **The column is named because the refusal is a thing a person reads.**
    /// [`nanoseconds`] is the store's one spelling of a moment and the failure
    /// is its own, not the engine's — which is why this is not
    /// [`RecordFailure::Engine`].
    AMomentTheRecordCannotHold {
        column: &'static str,
        reported: String,
    },
    /// The engine reported something.
    Engine { reported: String },
}

impl From<rusqlite::Error> for RecordFailure {
    fn from(error: rusqlite::Error) -> Self {
        RecordFailure::Engine {
            reported: error.to_string(),
        }
    }
}

/// The one durable local record.
pub struct LocalRecord {
    connection: rusqlite::Connection,
    path: PathBuf,
}

impl LocalRecord {
    /// Open the record at the location the adapter names, migrating it first.
    ///
    /// **L6: a migration runs at application start, before any module reads the
    /// record**, which is why the migration is inside opening rather than
    /// beside it — there is no way to hold an open record that has not been
    /// brought up to date.
    pub fn open() -> Result<Self, RecordFailure> {
        let path = locations::resolve(Location::LocalRecord).map_err(|not_named| {
            RecordFailure::NoLocation {
                reported: not_named.reported,
            }
        })?;
        let holder = locations::resolve(Location::PerUserApplicationData).map_err(|not_named| {
            RecordFailure::NoLocation {
                reported: not_named.reported,
            }
        })?;
        files::ensure_directory(&holder).map_err(RecordFailure::File)?;
        Self::open_at(&path)
    }

    /// The same, at a named path. **This exists so the record can be tested at
    /// all**, and every rule below is exercised through it.
    pub fn open_at(path: &Path) -> Result<Self, RecordFailure> {
        // **Whether the record already existed is asked before it is opened**,
        // because opening creates it. L8's copy is of a record that was there,
        // and *there* is not the same question as *what version it reads*: a
        // record written before this store carried a version at all reads 0,
        // exactly as a record this call just made does.
        let was_already_there = matches!(
            files::outside_program_wrote(path),
            files::Recheck::Present { .. }
        );
        let connection = rusqlite::Connection::open(path)?;
        settle_the_durability(&connection)?;
        migrations::bring_up_to_date(&connection, path, was_already_there)?;
        Ok(LocalRecord {
            connection,
            path: path.to_path_buf(),
        })
    }

    /// The record's own file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The version the record carries — **L5, and it is read from the record
    /// rather than inferred from which fields are present**, because inferring
    /// it is reading a store's shape as state.
    pub fn schema_version(&self) -> Result<u32, RecordFailure> {
        schema::version_in(&self.connection)
    }

    /// The engine, for anything that has to ask.
    pub fn connection(&self) -> &rusqlite::Connection {
        &self.connection
    }
}

/// The three settings `State, identity and recovery` §1.3 rule 6 forces, each
/// with the engine's own words for why.
///
/// **`synchronous = EXTRA`.** The engine's own page: *"EXTRA synchronous is
/// like FULL with the addition that the directory containing a rollback
/// journal is synced after that journal is unlinked to commit a transaction in
/// DELETE mode"*, and *"FULL is not necessarily durable across a power loss in
/// rollback mode, so if durability is desired, it is best to set the
/// synchronous mode to EXTRA."* **That is rule 6's last clause** — *the
/// directory holding that name is flushed too* — which the rule itself calls
/// not optional and the one that is ordinarily missed.
///
/// **`fullfsync = ON`, and only this system needs it.** The engine's own page:
/// *"This flag determines whether or not the F_FULLFSYNC syncing method is used
/// on systems that support it. The default value of the fullfsync flag is off.
/// Only Mac OS X supports F_FULLFSYNC."* **The default is off**, and rule 6
/// says in as many words that on the user's own Mac a durable write is the
/// full-flush form — so leaving the default would have left every *written
/// durably* in two documents resting on a flush that Apple's own manual says
/// does not reach the platters.
///
/// **`journal_mode = DELETE`**, which is the engine's own default and is
/// stated rather than assumed: it is what makes the record one file at rest,
/// which is what L7's publication and L8's copy both stand on.
fn settle_the_durability(connection: &rusqlite::Connection) -> Result<(), RecordFailure> {
    connection.pragma_update(None, "journal_mode", "DELETE")?;
    connection.pragma_update(None, "synchronous", "EXTRA")?;
    if cfg!(target_os = "macos") {
        connection.pragma_update(None, "fullfsync", true)?;
    }
    Ok(())
}

/// **L8: a copy of the record is taken before a migration runs and kept until
/// the new version has started successfully.**
///
/// This is not caution, it is forced by §20: macOS's supported replacement API
/// carries a named backup for rollback, the Windows update is side-by-side,
/// and the Linux update retains the previous verified package — **so on all
/// three systems a previous binary can be run again after a migration has
/// happened**, and without the copy a rollback would meet a record it cannot
/// read.
///
/// **L10 is what makes it cost nothing**: a migration writes no secret value
/// into any table, backup or crash record, so the record and its copies are
/// safe to write, copy and back up.
/// **A moment in this record is nanoseconds since the epoch, and not seconds.**
///
/// It has one home because more than one table holds a moment and they must not
/// disagree: `Servers` H2a's *last seen* mark, drawn from the desktop's own
/// moment, and `File Browser` U9a's moment of a unit outcome, drawn from the
/// same clock at the durable write. **A store that rounded either would move
/// the value every time it was written and read back**, and `Servers` H2a's
/// mark and `File Browser` U9a's moment are both compared for order.
///
/// **This is the store's spelling of a moment and never a clock of its own.**
/// `State, identity and recovery` D5 governs what may be done with the value;
/// nothing here reads a clock.
///
/// **It answers a failure rather than zero, and the caller decides what to do
/// with it. Corrected 2026-09-07** from round 1 of `Access`'s code audit,
/// finding F-9. **Nanoseconds in a signed 64-bit number run out around 2262**,
/// and a moment past that — or before 1970 — used to be written as **zero** and
/// read back as the Unix epoch, with nothing refused, nothing recorded and
/// nothing said. The read side already refused unrepresentable moments;
/// the write side now applies the same rule.
///
/// **The other row modules' behaviour does not move**, which was ruling 6 of
/// that round: each of the three other call sites was read and each supplies
/// the zero it was already writing, explicitly and with the reason at the site.
pub(crate) fn nanoseconds(at: std::time::SystemTime) -> Option<i64> {
    at.duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|since| i64::try_from(since.as_nanos()).ok())
}

pub(crate) fn moment(nanoseconds: i64) -> std::time::SystemTime {
    std::time::UNIX_EPOCH + std::time::Duration::from_nanos(nanoseconds.max(0) as u64)
}

pub fn copy_beside(path: &Path) -> PathBuf {
    let mut copy = path.as_os_str().to_os_string();
    copy.push(".before-migration");
    PathBuf::from(copy)
}

/// Take L8's copy.
pub fn take_the_copy(path: &Path) -> Result<PathBuf, RecordFailure> {
    let copy = copy_beside(path);
    let bytes = std::fs::read(path).map_err(|error| RecordFailure::Engine {
        reported: format!("the record could not be read to copy it: {error}"),
    })?;
    let written = files::write_whole_file(&copy, &bytes, Durability::ToTheDevice);
    if !written.wrote_the_file() {
        return Err(match written.first_error {
            Some(failure) => RecordFailure::File(failure),
            None => RecordFailure::Engine {
                reported: "the copy was not at its path after it was closed".into(),
            },
        });
    }
    Ok(copy)
}

/// **L13: the copy is spent by being restored, and the act is named here rather
/// than assumed.**
///
/// **Restoring it over the record is the first act of any rollback to a
/// previous binary, and it happens before that binary reads the record.** A
/// rollback that put a previous binary in front of a migrated record would meet
/// a newer schema and L9 would refuse to open it — **so L9's refusal is what
/// the user meets only where the copy is gone**, and it is not the ordinary
/// outcome of a rollback.
pub fn restore_the_copy(path: &Path) -> Result<(), RecordFailure> {
    let copy = copy_beside(path);
    files::publish(&copy, path).map_err(RecordFailure::File)
}

/// **L13's other half: the copy is discarded only after the new version's own
/// start has been recorded as successful**, which is what L8 means by *started
/// successfully* and which §20.1 Z21 defines.
pub fn discard_the_copy(path: &Path) -> Result<(), RecordFailure> {
    files::remove_file(&copy_beside(path)).map_err(RecordFailure::File)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::files::Recheck;

    fn scratch(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-record-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        path.join("local-record.sqlite3")
    }

    /// L5 and L6: a fresh record is created, migrated to what this binary
    /// knows, and carries that version in itself.
    #[test]
    fn a_fresh_record_opens_at_the_version_this_binary_knows() {
        let path = scratch("fresh");
        let record = LocalRecord::open_at(&path).expect("a fresh record opens");
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);

        // L6: opening again runs nothing and changes nothing.
        drop(record);
        let again = LocalRecord::open_at(&path).expect("it opens again");
        assert_eq!(again.schema_version().unwrap(), schema::BINARY_KNOWS);

        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// The engine property L7 and L8 both stand on: **the record at rest is one
    /// file.** Nothing beside it belongs to the steady-state database.
    #[test]
    fn the_record_at_rest_is_one_file() {
        let path = scratch("one-file");
        let record = LocalRecord::open_at(&path).unwrap();
        drop(record);

        let holder = path.parent().unwrap();
        let beside: Vec<String> = std::fs::read_dir(holder)
            .unwrap()
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            beside,
            vec!["local-record.sqlite3".to_string()],
            "{beside:?}"
        );

        let _ = std::fs::remove_dir_all(holder);
    }

    /// The three settings `State, identity and recovery` §1.3 rule 6 forces are
    /// actually in force, read back off the engine rather than assumed from
    /// having been set.
    #[test]
    fn the_record_is_opened_for_a_durable_write() {
        let path = scratch("durability");
        let record = LocalRecord::open_at(&path).unwrap();

        let journal: String = record
            .connection()
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .unwrap();
        assert_eq!(journal.to_uppercase(), "DELETE");

        // EXTRA is 3 in the engine's own numbering.
        let synchronous: i64 = record
            .connection()
            .pragma_query_value(None, "synchronous", |row| row.get(0))
            .unwrap();
        assert_eq!(synchronous, 3);

        // The default is off and Apple's own manual is why it may not be left
        // there on this system.
        if cfg!(target_os = "macos") {
            let full: i64 = record
                .connection()
                .pragma_query_value(None, "fullfsync", |row| row.get(0))
                .unwrap();
            assert_eq!(full, 1);
        }

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **L9: a binary that meets a record newer than its own schema version
    /// does not open it and says so.** It does not read the fields it
    /// recognises and ignore the rest.
    #[test]
    fn a_record_newer_than_the_binary_is_not_opened() {
        let path = scratch("newer");
        {
            let record = LocalRecord::open_at(&path).unwrap();
            schema::set_version(record.connection(), schema::BINARY_KNOWS + 5).unwrap();
        }
        match LocalRecord::open_at(&path) {
            Err(RecordFailure::RecordIsNewerThanTheBinary { record, binary }) => {
                assert_eq!(record, schema::BINARY_KNOWS + 5);
                assert_eq!(binary, schema::BINARY_KNOWS);
            }
            other => panic!(
                "the record was opened anyway: {other:?}",
                other = other.err()
            ),
        }
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **L7: the unit is the whole migration.** A migration whose statements
    /// fail part-way leaves the record at the version it had, with nothing the
    /// migration made.
    #[test]
    fn a_migration_is_atomic_entire() {
        let path = scratch("atomic");
        let record = LocalRecord::open_at(&path).unwrap();
        let before = record.schema_version().unwrap();

        // A migration whose second statement cannot run.
        let broken = migrations::Migration {
            produces: before + 1,
            what_it_runs: migrations::WhatItRuns::TheseStatements(&[
                "CREATE TABLE half_made (a INTEGER)",
                "CREATE TABLE half_made (a INTEGER)",
            ]),
        };
        assert!(migrations::run_one_for_tests(record.connection(), &broken).is_err());

        assert_eq!(
            record.schema_version().unwrap(),
            before,
            "the version moved"
        );
        let made: bool = record
            .connection()
            .query_row(
                "SELECT count(*) > 0 FROM sqlite_schema WHERE name = 'half_made'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(!made, "half of the migration survived");

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **L8: the copy is taken before a migration runs**, and a record that
    /// does not exist yet has nothing to copy — the fresh-installation case,
    /// which is not a failure to protect against.
    #[test]
    fn the_copy_is_taken_before_a_migration_and_not_before_a_creation() {
        let path = scratch("copy");

        // A fresh record: nothing to copy.
        let record = LocalRecord::open_at(&path).unwrap();
        drop(record);
        assert!(!migrations::the_copy_is_there(&path));

        // Now a record that already exists and has to move — a version 0
        // record, which is what any earlier binary's would be. **L8's copy is
        // taken**, and it holds what was there before the migration ran.
        let older = path.parent().unwrap().join("older-record.sqlite3");
        {
            let connection = rusqlite::Connection::open(&older).unwrap();
            connection
                .execute_batch("CREATE TABLE something_old (a INTEGER)")
                .unwrap();
        }
        let record = LocalRecord::open_at(&older).unwrap();
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);
        drop(record);
        assert!(
            migrations::the_copy_is_there(&older),
            "L8's copy was not taken"
        );

        // The copy holds the record as it stood before, not after.
        let copy = rusqlite::Connection::open(copy_beside(&older)).unwrap();
        let versioned: bool = copy
            .query_row(
                "SELECT count(*) > 0 FROM sqlite_schema WHERE name = 'smoozy_schema_version'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(
            !versioned,
            "the copy was taken after the migration, not before"
        );

        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **L13: the copy is spent by being restored**, and it is discarded only
    /// once the new version has started successfully.
    #[test]
    fn the_copy_is_spent_by_being_restored() {
        let path = scratch("restore");
        {
            let record = LocalRecord::open_at(&path).unwrap();
            record
                .connection()
                .execute_batch("CREATE TABLE the_old_shape (a INTEGER)")
                .unwrap();
        }
        take_the_copy(&path).expect("the copy is taken");

        // The record moves on.
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            connection
                .execute_batch("DROP TABLE the_old_shape; CREATE TABLE the_new_shape (a INTEGER)")
                .unwrap();
        }

        restore_the_copy(&path).expect("the copy is restored over the record");
        let connection = rusqlite::Connection::open(&path).unwrap();
        let old: bool = connection
            .query_row(
                "SELECT count(*) > 0 FROM sqlite_schema WHERE name = 'the_old_shape'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(old, "the restore did not put the previous record back");
        drop(connection);

        // Spent: restoring published it, so nothing is left at the copy's name.
        assert!(!migrations::the_copy_is_there(&path));

        // And discarding one that is there removes it.
        take_the_copy(&path).unwrap();
        assert!(migrations::the_copy_is_there(&path));
        discard_the_copy(&path).unwrap();
        assert!(!migrations::the_copy_is_there(&path));

        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **L11 and L12**: a migration that empties a protected table is caught
    /// rather than passing silently. Row 290's failure is invisible until a
    /// user presses the control and one warning does not come back.
    #[test]
    fn a_migration_that_empties_a_protected_table_is_caught() {
        let path = scratch("protected");
        let record = LocalRecord::open_at(&path).unwrap();
        record
            .connection()
            .execute_batch(
                // **One column since version 15** — `Desktop app and delivery`
                // §15.2 G4 forbids the moment version 1 wrote here by name.
                "INSERT INTO suppressed_warning VALUES ('file-browser.delete');
                 INSERT INTO history_tombstone VALUES ('a-job', 1, 'removed')",
            )
            .unwrap();

        let before = migrations::protected_counts(record.connection()).unwrap();
        // **Three names and the job record's six sets** — must-do 4 of step
        // 10b round 1 of *jobs, history and downloads*, `Queue & History`
        // E9's kept words since record version 34 (piece 6d), and P4's
        // fetched derivatives since record version 37 (the 38's piece 3).
        assert_eq!(before.len(), schema::TABLES_A_MIGRATION_NEVER_EMPTIES.len());
        assert_eq!(before.len(), 9);
        migrations::nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing has moved yet");

        record
            .connection()
            .execute_batch("DELETE FROM suppressed_warning")
            .unwrap();
        assert!(
            migrations::nothing_protected_was_lost(record.connection(), &before).is_err(),
            "a removed suppression row was not caught"
        );

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **L12a: a migration that empties the local job record is caught.**
    ///
    /// **The same shape as the test above, for the same reason and a worse
    /// cost.** L11's own failure is invisible until a user presses a control;
    /// this one is invisible full stop — nothing is pressed, nothing fails, and
    /// a person opens History to find their whole render history gone. The
    /// table has held it since version 23 and the list that protects it named
    /// two tables until 2026-09-18.
    #[test]
    fn a_migration_that_empties_the_job_record_is_caught() {
        let path = scratch("protected-job-record");
        let record = LocalRecord::open_at(&path).unwrap();
        record
            .connection()
            .execute(
                "INSERT INTO job_record (
                     job, server, standing, render_start_learned,
                     submitted_with_no_scene_named, blender_build_path,
                     blender_version, blender_origin, device_ask, file_values_stood
                 ) VALUES ('a-job', 'a-server', 'confirmed', 1, 0, '/blender', '4.2.0', 'fetched', 'cpu', 0)",
                [],
            )
            .unwrap();

        let before = migrations::protected_counts(record.connection()).unwrap();
        assert!(
            before.iter().any(|(table, held)| table == "job_record" && *held == 1),
            "the job record is not one of the tables a migration never empties"
        );
        migrations::nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing has moved yet");

        record
            .connection()
            .execute_batch("DELETE FROM job_record")
            .unwrap();
        assert!(
            migrations::nothing_protected_was_lost(record.connection(), &before).is_err(),
            "a migration emptied the local job record and nothing failed — the user's \
             whole render history would have gone with it"
        );

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **L12a covers the whole job record, and the record is six tables** —
    /// must-do 4 of step 10b round 1 of *jobs, history and downloads*.
    ///
    /// **This failed before the fix**: the list named the row's table alone, so
    /// a migration that emptied the disturbances table — `Queue & History`
    /// N21's marks — or any of the other three sets passed with nothing failing.
    #[test]
    fn a_migration_that_empties_any_set_beside_the_job_record_is_caught() {
        for table in job_records::EVERY_SET_BESIDE_THE_ROW {
            let path = scratch(&format!("protected-{table}"));
            let record = LocalRecord::open_at(&path).unwrap();
            let one_row = match table {
                "job_record_disturbance" => {
                    "INSERT INTO job_record_disturbance VALUES ('a-job', 0, '/a.png', 'replace', 'disturbed', NULL)"
                }
                "job_record_file_changed" => "INSERT INTO job_record_file_changed VALUES ('a-job', 2, 1)",
                "job_record_re_run_mark" => "INSERT INTO job_record_re_run_mark VALUES ('a-job', 2, 61)",
                "job_record_file_value_pin" => {
                    "INSERT INTO job_record_file_value_pin VALUES ('a-job', 0, 'ViewLayer', 'static', 1)"
                }
                "job_record_words" => {
                    "INSERT INTO job_record_words VALUES ('a-job', 1, 'ordinary', 'collected', x'4672613a31', 5, 0, NULL)"
                }
                "job_record_preview" => {
                    "INSERT INTO job_record_preview VALUES ('a-job', 5, 'op', NULL, 0, '/p/a.png', 1759500000000, 1, 0, NULL)"
                }
                other => panic!("{other} is a set beside the job record with no row written here"),
            };
            record.connection().execute_batch(one_row).unwrap();

            let before = migrations::protected_counts(record.connection()).unwrap();
            assert!(
                before.iter().any(|(held, count)| held == table && *count == 1),
                "{table} is not one of the tables a migration never empties"
            );
            record
                .connection()
                .execute_batch(&format!("DELETE FROM {table}"))
                .unwrap();
            assert!(
                migrations::nothing_protected_was_lost(record.connection(), &before).is_err(),
                "a migration emptied {table} and nothing failed"
            );

            drop(record);
            let _ = std::fs::remove_dir_all(path.parent().unwrap());
        }
    }

    /// **Version 24 carries every job record across and marks none of them** —
    /// item C1's column arrives as an addition, so a record made at version 23
    /// keeps its whole history and reads nought for the new mark.
    #[test]
    fn version_24_carries_every_job_record_across_unmarked() {
        let path = scratch("version-24");
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in migrations::ALL.iter().filter(|one| one.produces <= 23) {
                migrations::run_one_for_tests(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 23);
            connection
                .execute(
                    "INSERT INTO job_record (
                         job, server, standing, render_start_learned,
                         submitted_with_no_scene_named, blender_build_path,
                         blender_version, blender_origin, device_ask, file_values_stood
                     ) VALUES ('a-job', 'a-server', 'confirmed', 1, 0, '/blender', '4.2.0',
                               'resolved for them', 'the file''s own, unread', 0)",
                    [],
                )
                .unwrap();
        }
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            migrations::protected_counts(&connection).unwrap()
        };
        let record = LocalRecord::open_at(&path).expect("the version-23 record opens");
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);
        migrations::nothing_protected_was_lost(record.connection(), &before)
            .expect("the job record came across whole");
        let marked: i64 = record
            .connection()
            .query_row("SELECT start_never_heard FROM job_record", [], |row| row.get(0))
            .unwrap();
        assert_eq!(marked, 0);

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn seventeenth_piece_2_again_old_jobs_remain_unknown_after_the_forward_migration() {
        let path = scratch("seventeenth-check-purpose");
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in migrations::ALL.iter().filter(|one| one.produces <= 46) {
                migrations::run_one_for_tests(&connection, migration).unwrap();
            }
            connection.execute("INSERT INTO job_record (job, server, standing, render_start_learned,
                submitted_with_no_scene_named, blender_build_path, blender_version, blender_origin,
                device_ask, file_values_stood) VALUES ('old-job', 'old-server', 'confirmed', 0, 0,
                '/blender', '5.2.0', 'resolved for them', 'the file''s own, unread', 0)", []).unwrap();
        }
        let record = LocalRecord::open_at(&path).unwrap();
        let saved: Option<String> = record.connection().query_row(
            "SELECT first_frame_purpose FROM job_record WHERE job = 'old-job'", [], |row| row.get(0),
        ).expect("the forward migration adds optional purpose without inventing an old check");
        assert_eq!(saved, None);
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);
        drop(record);
        let reopened = LocalRecord::open_at(&path).unwrap();
        assert_eq!(reopened.connection().query_row("SELECT count(*) FROM job_record WHERE job = 'old-job'", [], |row| row.get::<_, u32>(0)).unwrap(), 1);
        drop(reopened);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn seventeenth_v48_keeps_legacy_starts_unknown_and_the_v47_job_fields() {
        let path = scratch("piece3-v48");
        {
            let c = rusqlite::Connection::open(&path).unwrap();
            for migration in migrations::ALL.iter().filter(|m| m.produces <= 47) {
                migrations::run_one_for_tests(&c, migration).unwrap();
            }
            c.execute("INSERT INTO upload_request (record,server_row,destination,opened_by_run) VALUES ('old-up','old-server',x'2f','old-run')", []).unwrap();
            c.execute("INSERT INTO download_request (record,server_row,destination,opened_by_run) VALUES ('old-down','old-server',x'2f','old-run')", []).unwrap();
            c.execute("INSERT INTO download_arrangement (job,server_row,destination,narrowing,desired_state) VALUES ('old-job','old-server',x'2f','everything','on')", []).unwrap();
            c.execute("INSERT INTO job_record (job, server, standing, render_start_learned,
                submitted_with_no_scene_named, blender_build_path, blender_version, blender_origin,
                device_ask, file_values_stood, first_frame_purpose, first_frame_pair, first_frame_number, first_frame_range)
                VALUES ('old-job', 'old-server', 'confirmed', 0, 0, '/blender', '5.2.0', 'resolved for them',
                'the file''s own, unread', 0, 'check', 'paired-job', 1, '1-90')", []).unwrap();
        }
        for _ in 0..2 {
            let record = LocalRecord::open_at(&path).unwrap();
            // **The version this binary knows, not 48 by name** — 48 was the
            // newest when this was written, and version 49 (the update route's
            // rows, step 11) carries a version-48 record forward untouched.
            assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);
            assert!(schema::BINARY_KNOWS >= 48);
            let c = record.connection();
            for (table, key, id) in [("upload_request", "record", "old-up"), ("download_request", "record", "old-down"), ("download_arrangement", "job", "old-job")] {
                assert!(transfer_records::parent_start(c, table, key, id).unwrap().is_none());
                c.execute_batch("SAVEPOINT legacy_update").unwrap();
                assert!(transfer_records::start_for_parent(c, table, key, id, false).unwrap().is_none());
                c.execute_batch("RELEASE legacy_update").unwrap();
            }
            let held: (String, String, i64, String) = c.query_row("SELECT first_frame_purpose,first_frame_pair,first_frame_number,first_frame_range FROM job_record WHERE job='old-job'", [], |r| Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?))).unwrap();
            assert_eq!(held, ("check".into(), "paired-job".into(), 1, "1-90".into()));
            assert_eq!(c.query_row("SELECT count(*) FROM transfer_start_order", [], |r| r.get::<_, i64>(0)).unwrap(), 0);
        }
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **L10: a migration writes no secret value**, and the way that is kept is
    /// that no migration statement carries a value at all — every one of them
    /// is a shape.
    #[test]
    fn no_migration_statement_carries_a_value() {
        // **Narrowed 2026-09-03, and it was this test that was too strict
        // rather than the migration.** L11 and L12 forbid a migration
        // *removing* what the record holds; nothing forbids one carrying
        // existing rows forward, and `Security and licence` E3a's, at that
        // document's §5.1,
        // register needs exactly that — a migration that left the marks behind
        // would destroy the debts the register exists to keep. **What stays
        // forbidden is a migration writing state the record did not already
        // hold**, which an `INSERT ... SELECT` over the record's own rows
        // cannot do.
        // **A migration that decides from the record is asked over a record**,
        // and over the one shape that makes it produce every statement it has —
        // otherwise the test reads an empty list and passes for the wrong
        // reason. Added 2026-09-13 with version 16.
        let path = scratch("no-values");
        let connection = rusqlite::Connection::open(&path).unwrap();
        for migration in migrations::ALL.iter().filter(|one| one.produces <= 15) {
            migrations::run_one_for_tests(&connection, migration).expect("it migrates");
        }
        the_value_set_is_put_back_to_three_columns(&connection);
        connection
            .execute_batch(
                "ALTER TABLE dependency_check_standing
                 DROP COLUMN every_row_confirmed_or_set_aside",
            )
            .unwrap();

        let mut looked_at = 0;
        for migration in migrations::ALL {
            let statements: Vec<String> = match migration.what_it_runs {
                migrations::WhatItRuns::TheseStatements(statements) => {
                    statements.iter().map(|one| one.to_string()).collect()
                }
                migrations::WhatItRuns::WhateverTheRecordAsksFor(decide) => {
                    decide(&connection).expect("it decides")
                }
                migrations::WhatItRuns::WithTheRecordsPreviousExistence(decide) => {
                    let mut statements = decide(&connection, true).expect("existing record");
                    statements.extend(decide(&connection, false).expect("new record"));
                    statements
                }
            };
            for statement in statements {
                looked_at += 1;
                let head = statement.trim_start().to_uppercase();
                let a_shape = head.starts_with("CREATE")
                    || head.starts_with("ALTER")
                    || head.starts_with("DROP");
                let carries_the_record_forward =
                    head.starts_with("INSERT") && head.contains("SELECT");
                // Only version 53 may create the fixed legacy erase debt or
                // delete the withdrawn told kind. Neither holds a secret.
                let the_paid_cleanup = migration.produces == 53
                    && (statement == format!("INSERT OR IGNORE INTO owed_erase (kind, subject) VALUES ('licence key', '{}')", key_rows::LEGACY_LICENCE_KEY_SUBJECT)
                        || statement == "DELETE FROM told_entry WHERE thing = 'the lock policy'");
                assert!(
                    a_shape || carries_the_record_forward || the_paid_cleanup,
                    "a migration statement is neither a shape nor a carry: {statement}"
                );
            }
        }
        assert!(looked_at > 0, "no statement was read at all");

        drop(connection);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// Put `failure_record_raised_over` back to the three columns a record that
    /// crossed version 14 before 2026-09-12 is carrying — the shape version 16
    /// exists to repair. **Written out rather than taken from git**, so the test
    /// says what it is standing on.
    fn the_value_set_is_put_back_to_three_columns(connection: &rusqlite::Connection) {
        connection
            .execute_batch(
                "DROP TABLE failure_record_raised_over;
                 CREATE TABLE failure_record_raised_over (
        durable_subject TEXT NOT NULL,
        identity_or_mark TEXT NOT NULL,
        value TEXT NOT NULL,
        PRIMARY KEY (durable_subject, identity_or_mark, value)
     )",
            )
            .unwrap();
    }

    /// The record's own account of how a table is written, which is what a
    /// repaired shape and a fresh one have to agree about.
    fn the_tables_own_statement(connection: &rusqlite::Connection, table: &str) -> String {
        connection
            .query_row(
                "SELECT sql FROM sqlite_schema WHERE type = 'table' AND name = ?1",
                [table],
                |row| row.get::<_, String>(0),
            )
            .unwrap()
    }

    /// A record left exactly as a binary that knew only version 15 would have
    /// left it.
    fn a_version_fifteen_record(path: &std::path::Path) -> rusqlite::Connection {
        let connection = rusqlite::Connection::open(path).unwrap();
        for migration in migrations::ALL.iter().filter(|one| one.produces <= 15) {
            migrations::run_one_for_tests(&connection, migration).expect("it migrates");
        }
        assert_eq!(schema::version_in(&connection).unwrap(), 15);
        connection
    }

    /// Two rows in the tables L11 and L12 protect, so a crossing can be shown
    /// not to have emptied them.
    fn the_protected_tables_are_given_a_row(connection: &rusqlite::Connection) {
        connection
            .execute_batch(
                "INSERT INTO history_tombstone (job, deleted_at, machine_side_copy)
                 VALUES ('a-job', 1, 'gone');
                 INSERT INTO suppressed_warning (warning_identity)
                 VALUES ('file-browser.delete')",
            )
            .unwrap();
    }

    /// **The first crossing: a record made from nothing.**
    ///
    /// Version 14 makes the four-column table, so version 16 has nothing to do
    /// and does nothing — it produces no statement at all and the record comes
    /// out at 16 with the table version 14 wrote.
    #[test]
    fn a_fresh_record_crosses_version_sixteen_with_no_statement_at_all() {
        let path = scratch("sixteen-fresh");
        let record = LocalRecord::open_at(&path).expect("a fresh record opens");
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);

        assert!(
            schema::version_16(record.connection())
                .expect("it decides")
                .is_empty(),
            "version 16 had work to do on a record version 14 had just made"
        );
        assert!(schema::the_table_has_the_column(
            record.connection(),
            "failure_record_raised_over",
            "durable_subject_kind"
        )
        .unwrap());

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **The second crossing, and it is the one this migration exists for: the
    /// real three-column record, with rows in it.**
    ///
    /// The table is put back to the three columns the Creator's own record was
    /// carrying, a condition record and two values it was raised over are
    /// written by hand the way a binary of that day would have written them,
    /// and this binary opens the record. **What is asserted is that the rows
    /// came across with the kind their own record carries, that the repaired
    /// table is written exactly the way a fresh one is, and that the reader
    /// which used to answer *no such column* answers the record.**
    #[test]
    fn a_three_column_record_crosses_version_sixteen_and_keeps_its_rows() {
        use crate::record::failure_records::{DurableSubject, FailureRecords, Kind};
        use smoozy_state::identities::{Identity, IdentityKind};

        let path = scratch("sixteen-three-columns");
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let server = issue(IdentityKind::LocalServer);
        let condition = issue(IdentityKind::Condition);

        let fresh_statement = {
            let other = scratch("sixteen-three-columns-fresh");
            let record = LocalRecord::open_at(&other).unwrap();
            let statement =
                the_tables_own_statement(record.connection(), "failure_record_raised_over");
            drop(record);
            let _ = std::fs::remove_dir_all(other.parent().unwrap());
            statement
        };

        {
            let connection = a_version_fifteen_record(&path);
            the_protected_tables_are_given_a_row(&connection);
            the_value_set_is_put_back_to_three_columns(&connection);

            connection
                .execute(
                    "INSERT INTO failure_record
                        (kind, durable_subject, durable_subject_kind, identity_or_mark,
                         condition_key, the_diary_holds_the_sentence)
                     VALUES ('condition', ?1, 'server row', ?2, 'a-key', 0)",
                    rusqlite::params![server.as_text(), condition.as_text()],
                )
                .unwrap();
            for value in ["disconnected", "reconnecting"] {
                connection
                    .execute(
                        "INSERT INTO failure_record_raised_over
                            (durable_subject, identity_or_mark, value)
                         VALUES (?1, ?2, ?3)",
                        rusqlite::params![server.as_text(), condition.as_text(), value],
                    )
                    .unwrap();
            }
            // **A value row whose record is gone**, so the carry can be shown
            // not to drop one.
            connection
                .execute(
                    "INSERT INTO failure_record_raised_over
                        (durable_subject, identity_or_mark, value)
                     VALUES ('an orphan', 'a mark', 'a value')",
                    [],
                )
                .unwrap();
        }

        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            migrations::protected_counts(&connection).unwrap()
        };

        // This binary opens it. L6 runs the one migration that is missing.
        let record = LocalRecord::open_at(&path).expect("the three-column record opens");
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);

        // **Written exactly the way a fresh record writes it**, which is what
        // stops a repaired record and a new one being two shapes again.
        assert_eq!(
            the_tables_own_statement(record.connection(), "failure_record_raised_over"),
            fresh_statement
        );

        // **Every row came across, and each carries its own record's kind.**
        let carried: Vec<(String, String, String, String)> = record
            .connection()
            .prepare(
                "SELECT durable_subject, durable_subject_kind, identity_or_mark, value
                 FROM failure_record_raised_over ORDER BY value",
            )
            .unwrap()
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(
            carried,
            vec![
                (
                    "an orphan".to_string(),
                    String::new(),
                    "a mark".to_string(),
                    "a value".to_string()
                ),
                (
                    server.as_text(),
                    "server row".to_string(),
                    condition.as_text(),
                    "disconnected".to_string()
                ),
                (
                    server.as_text(),
                    "server row".to_string(),
                    condition.as_text(),
                    "reconnecting".to_string()
                ),
            ]
        );

        // **The read that used to answer *no such column* answers the record.**
        let subject = DurableSubject::ServerRow(server.clone());
        let standing = FailureRecords::over(record.connection())
            .standing_against(&subject)
            .expect("the read answers");
        assert_eq!(standing.len(), 1);
        assert_eq!(standing[0].kind, Kind::Condition);
        assert_eq!(
            standing[0].the_values_it_was_raised_over,
            vec!["disconnected".to_string(), "reconnecting".to_string()]
        );

        // L11 and L12: nothing protected was lost by the crossing.
        migrations::nothing_protected_was_lost(record.connection(), &before)
            .expect("the protected tables are as they were");

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **The third crossing: a record already at four columns, which must cross
    /// untouched.**
    ///
    /// The rows are written with a kind of their own that no join could have
    /// produced, so a migration that rebuilt the table anyway would show up as
    /// a changed value rather than as nothing.
    #[test]
    fn a_four_column_record_crosses_version_sixteen_untouched() {
        let path = scratch("sixteen-four-columns");

        {
            let connection = a_version_fifteen_record(&path);
            connection
                .execute(
                    "INSERT INTO failure_record_raised_over
                        (durable_subject, durable_subject_kind, identity_or_mark, value)
                     VALUES ('a subject', 'history record', 'a mark', 'a value')",
                    [],
                )
                .unwrap();
        }

        let statement_before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            the_tables_own_statement(&connection, "failure_record_raised_over")
        };

        let record = LocalRecord::open_at(&path).expect("the four-column record opens");
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);
        assert_eq!(
            the_tables_own_statement(record.connection(), "failure_record_raised_over"),
            statement_before,
            "a record already at four columns was rebuilt"
        );

        let kind: String = record
            .connection()
            .query_row(
                "SELECT durable_subject_kind FROM failure_record_raised_over",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(
            kind, "history record",
            "the kind the record already held was replaced"
        );

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **The other half of version 16: the column [`schema::VERSION_10`]'s
    /// in-place edit left off any record that crossed version 10 on
    /// 2026-09-04.**
    ///
    /// A row is put in the table first, because the engine will not add a
    /// `NOT NULL` column to a table holding rows without a default — which is
    /// why the repaired column carries one.
    #[test]
    fn a_check_standing_missing_its_column_crosses_version_sixteen_and_keeps_its_row() {
        let path = scratch("sixteen-check-standing");

        {
            let connection = a_version_fifteen_record(&path);
            connection
                .execute_batch(
                    "ALTER TABLE dependency_check_standing
                     DROP COLUMN every_row_confirmed_or_set_aside",
                )
                .unwrap();
            connection
                .execute_batch(
                    "INSERT INTO dependency_check_standing
                        (server_row, installation, root, read_with_build_path,
                         read_with_version, completion, count_not_found,
                         count_could_not_be_sent, count_could_not_be_fixed,
                         count_found_but_not_sent, count_set_aside, count_confirmed,
                         root_size, root_inode, root_modification_time,
                         root_change_time, replaced_since_check, disturbed)
                     VALUES ('a server', 'an installation', X'2f612e626c656e64',
                             '/blender', '4.5', 'complete', 1, 0, 0, 0, 0, 0,
                             10, 'inode-1', 2, 3, 0, 0)",
                )
                .unwrap();
        }

        let record = LocalRecord::open_at(&path).expect("the record opens");
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);

        // **The row is still there and the column reads the safe direction.**
        // `VERSION_10`'s own note: the plausible guess stops a warning on a file
        // that still has unconfirmed rows.
        let (rows, confirmed): (i64, i64) = record
            .connection()
            .query_row(
                "SELECT count(*), max(every_row_confirmed_or_set_aside)
                 FROM dependency_check_standing",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(rows, 1);
        assert_eq!(confirmed, 0);

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **L5, L6 and L9 over version 8** — `Server protocol and transfers` S9's
    /// per-attempt stable file identity.
    ///
    /// A record left at version 7 by an earlier binary is opened by this one,
    /// migrated, and read back through the transfer record's own reader. **This
    /// is the proof the migration is the mechanism the existing ones use** and
    /// not a shape only a fresh record ever gets: version 7's rows are written
    /// here with the old table's own columns, because a version 7 binary had no
    /// others.
    #[test]
    fn a_version_seven_record_opens_at_version_eight_and_its_transfers_read_back() {
        use smoozy_state::identities::{Identity, IdentityKind};

        let path = scratch("version-eight");
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let transfer = issue(IdentityKind::Transfer);
        let server = issue(IdentityKind::LocalServer);
        let run = issue(IdentityKind::Run);

        // A record as a binary that knew only version 7 would have left it.
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in migrations::ALL.iter().filter(|one| one.produces <= 7) {
                migrations::run_one_for_tests(&connection, migration).expect("it migrates");
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 7);
            connection
                .execute(
                    "INSERT INTO transfer_record
                     (transfer, attempt, direction, server, source_path, final_path,
                      staging_directory, total_size_expected, written_by_run, cleanup_is_owed)
                     VALUES (?1, 2, 'down', ?2, '/machine/frame.exr',
                      '/home/user/Downloads/frame.exr', '/home/user/Downloads/.smoozy-staging',
                      1024, ?3, 0)",
                    rusqlite::params![transfer.as_text(), server.as_text(), run.as_text()],
                )
                .unwrap();
            connection
                .execute(
                    "INSERT INTO transfer_superseded_attempt (transfer, attempt) VALUES (?1, 1)",
                    [transfer.as_text()],
                )
                .unwrap();
        }

        // This binary opens it. L6 runs the one migration that is missing.
        let record = LocalRecord::open_at(&path).expect("the version 7 record opens");
        // **L6 runs every migration that is missing, one at a time in order**,
        // so a version 7 record comes out at whatever this binary knows rather
        // than at the next number up. The version 8 column this test is about
        // is read below.
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);

        let back = transfer_records::read(record.connection(), &transfer)
            .expect("it reads")
            .expect("the transfer is still there");
        assert_eq!(back.attempt, 2);
        assert_eq!(back.superseded_attempts.len(), 1);
        assert_eq!(back.superseded_attempts[0].attempt, 1);
        // **Nothing was invented for it.** The old record never held an
        // identity for that attempt, and a null is read as *nothing was
        // recorded* rather than as a supplier that could not answer — so
        // `Server protocol and transfers` S28 leaves that path alone, which is
        // exactly what the record said before the migration.
        assert_eq!(back.superseded_attempts[0].staging_identity, None);

        // And a record written by this binary carries the identity through.
        let mut written = back.clone();
        written.superseded_attempts[0].staging_identity = Some(
            smoozy_server_protocol::transfers::StableFileIdentity::The("volume-1:file-7".into()),
        );
        transfer_records::put(record.connection(), &written).expect("written");
        let again = transfer_records::read(record.connection(), &transfer)
            .unwrap()
            .unwrap();
        assert_eq!(again, written);

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **Version 9 over a real version 8 record, and every field `File
    /// Browser` U1 adds has a home.**
    ///
    /// The record is left as a binary that knew only version 8 would have left
    /// it, with an outcome record and its enumeration already in it, and this
    /// binary opens it. **What the test asks is `File Browser` U1's own
    /// list**, field by
    /// field, written and read back — including the two paths as bytes that no
    /// text column could hold, and the join to §9.4's enumeration by position,
    /// which is what keeps one enumeration rather than two.
    #[test]
    fn a_version_eight_record_opens_at_version_nine_and_takes_u1s_own_fields() {
        use smoozy_state::identities::{Identity, IdentityKind};
        use smoozy_state::outcomes::{Keeps, UnitOutcome};

        let path = scratch("version-nine");
        let issue = |kind| Identity::issue(kind).expect("random bytes");
        let server = issue(IdentityKind::LocalServer);
        let run = issue(IdentityKind::Run);
        let transfer = issue(IdentityKind::Transfer);
        let job = issue(IdentityKind::Job);

        // A record as a binary that knew only version 8 would have left it,
        // carrying a §9.4 outcome record and its enumeration.
        let record_identity = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in migrations::ALL.iter().filter(|one| one.produces <= 8) {
                migrations::run_one_for_tests(&connection, migration).expect("it migrates");
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 8);
            let mut keeper = outcome_records::OutcomeRecords::over(&connection);
            keeper
                .enumerate("an upload request", &["the first file", "the second file"])
                .expect("the enumeration lands")
        };

        // This binary opens it. L6 runs every migration that is missing.
        //
        // **The version asserted is [`schema::BINARY_KNOWS`] and not a
        // number**, so that a module arriving with its own migration moves one
        // constant rather than this test. It read `9` twice until 2026-09-04,
        // when the first later version landed; what the test is about is
        // `File Browser` U1's fields surviving a migration, and that is
        // unchanged.
        let record = LocalRecord::open_at(&path).expect("the version 8 record opens");
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);
        assert!(schema::BINARY_KNOWS >= 9);

        // **Nothing of §9.4's half moved.** The enumeration written before the
        // migration is the enumeration after it, which is L11 and L12's rule
        // holding over the table the units live in.
        let keeper = outcome_records::OutcomeRecords::over(record.connection());
        let read = keeper
            .read(&record_identity)
            .expect("it reads")
            .expect("the record is still there");
        assert_eq!(read.action, "an upload request");
        assert_eq!(read.units.len(), 2);

        // **`File Browser` U1's record-level fields.**
        let connection = record.connection();
        connection
            .execute(
                "INSERT INTO upload_request
                 (record, server_row, installation, destination, collision_answer, opened_by_run)
                 VALUES (?1, ?2, NULL, ?3, 'keep both', ?4)",
                rusqlite::params![
                    record_identity.as_text(),
                    server.as_text(),
                    // A destination that is not valid text, which is why the
                    // column is a blob: `File Browser` N2 carries names as
                    // stored on disk and nothing re-encodes one.
                    &b"/jobs/\xff\xfe scenes"[..],
                    run.as_text()
                ],
            )
            .expect("the request's own fields go down");

        // **`File Browser` U1's enumeration entries, joined to §9.4's own
        // positions.**
        connection
            .execute(
                "INSERT INTO upload_request_entry
                 (record, position, source_path, landing_path, size_as_enumerated,
                  disposition, keep_both_landing_path, handed_to_act, transfer,
                  outcome_written_at)
                 VALUES (?1, 0, ?2, ?3, 4096, 'to send', NULL, NULL, ?4, NULL)",
                rusqlite::params![
                    record_identity.as_text(),
                    &b"/Users/somebody/\xff scene.blend"[..],
                    &b"/jobs/\xff scene.blend"[..],
                    transfer.as_text()
                ],
            )
            .expect("the first entry goes down");
        connection
            .execute(
                "INSERT INTO upload_request_entry
                 (record, position, source_path, landing_path, size_as_enumerated,
                  disposition, keep_both_landing_path, handed_to_act, transfer,
                  outcome_written_at)
                 VALUES (?1, 1, ?2, ?3, 8192, 'renamed under keep both', ?4, NULL, NULL, NULL)",
                rusqlite::params![
                    record_identity.as_text(),
                    &b"/Users/somebody/texture.png"[..],
                    &b"/jobs/texture.png"[..],
                    &b"/jobs/texture (2).png"[..]
                ],
            )
            .expect("the second entry goes down");

        // **`File Browser` U1's telling**, whose rows hold the jobs alone.
        connection
            .execute(
                "INSERT INTO upload_request_telling_job (record, position, job)
                 VALUES (?1, 0, ?2)",
                rusqlite::params![record_identity.as_text(), job.as_text()],
            )
            .expect("the telling goes down");

        // **`File Browser` U9a's moment lands on the unit whose outcome
        // landed**, which is what the join is for: the outcome is §9.4's and
        // the moment is `File Browser` U1's,
        // and they are one unit.
        let mut keeper = outcome_records::OutcomeRecords::over(connection);
        keeper
            .a_unit_ended(&record_identity, "the first file", UnitOutcome::Done)
            .expect("the unit ends");
        connection
            .execute(
                "UPDATE upload_request_entry SET outcome_written_at = 1770000000000000000
                 WHERE record = ?1 AND position = 0",
                [record_identity.as_text()],
            )
            .expect("the moment goes down beside it");

        let (source, landing, size, disposition, transfer_back, moment, outcome): (
            Vec<u8>,
            Vec<u8>,
            i64,
            String,
            String,
            i64,
            String,
        ) = connection
            .query_row(
                "SELECT e.source_path, e.landing_path, e.size_as_enumerated, e.disposition,
                        e.transfer, e.outcome_written_at, u.outcome
                 FROM upload_request_entry e
                 JOIN outcome_record_unit u
                   ON u.record = e.record AND u.position = e.position
                 WHERE e.record = ?1 AND e.position = 0",
                [record_identity.as_text()],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                        row.get(6)?,
                    ))
                },
            )
            .expect("the entry and its unit come back as one");
        assert_eq!(source, b"/Users/somebody/\xff scene.blend");
        assert_eq!(landing, b"/jobs/\xff scene.blend");
        assert_eq!(size, 4096);
        assert_eq!(disposition, "to send");
        assert_eq!(transfer_back, transfer.as_text());
        assert_eq!(moment, 1_770_000_000_000_000_000);
        assert_eq!(outcome, "done");

        // The destination came back as the bytes that went down.
        let destination: Vec<u8> = connection
            .query_row(
                "SELECT destination FROM upload_request WHERE record = ?1",
                [record_identity.as_text()],
                |row| row.get(0),
            )
            .expect("the destination comes back");
        assert_eq!(destination, b"/jobs/\xff\xfe scenes");

        // `File Browser` X9's generated name is on the entry it belongs to and
        // nowhere else.
        let keep_both: Option<Vec<u8>> = connection
            .query_row(
                "SELECT keep_both_landing_path FROM upload_request_entry
                 WHERE record = ?1 AND position = 1",
                [record_identity.as_text()],
                |row| row.get(0),
            )
            .expect("it comes back");
        assert_eq!(keep_both.as_deref(), Some(&b"/jobs/texture (2).png"[..]));

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// L8's copy sits beside the record and is a file like any other, so the
    /// adapter is what looks for it.
    #[test]
    fn the_copy_sits_beside_the_record() {
        let path = PathBuf::from("/somewhere/local-record.sqlite3");
        assert_eq!(
            copy_beside(&path),
            PathBuf::from("/somewhere/local-record.sqlite3.before-migration")
        );
        assert!(matches!(
            crate::platform::files::outside_program_wrote(&copy_beside(&path)),
            Recheck::Absent
        ));
    }
}

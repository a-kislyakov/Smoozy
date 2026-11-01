//! Migrations — `Desktop app and delivery` §5.2.
//!
//! **L6: a migration runs at application start, before any module reads the
//! record, and one at a time in order. Forward only.** No module sees a partly
//! migrated record, because none has read it yet.

use std::path::Path;

use super::schema::{self, BINARY_KNOWS};
use super::{copy_beside, take_the_copy, RecordFailure};
use crate::platform::files;

/// One migration: the version it produces and what it runs to produce it.
pub struct Migration {
    pub produces: u32,
    pub what_it_runs: WhatItRuns,
}

/// What a migration runs: fixed statements, a decision from the record, or
/// a decision that also distinguishes an existing record from a new one.
///
/// **Added 2026-09-13, out of the repair of the failure-record store.** Every
/// migration before it ran the same statements on every record, which is right
/// wherever the shape a migration meets is the shape the version before it
/// made. **It stops being right the moment a version has been produced by two
/// different sets of statements** — which is what an in-place edit of a shipped
/// version leaves behind, and what the record on the Creator's own desk was
/// carrying.
///
/// **The second kind is not a licence to edit a shipped version.** A migration
/// is still forward only and a version is still produced once. What this adds
/// is the ability to repair a divergence that already exists, **from the table
/// as it actually is rather than from an argument about which records could
/// exist** — and that argument is what caused the divergence.
pub enum WhatItRuns {
    /// The same statements on every record.
    TheseStatements(&'static [&'static str]),
    /// **Statements decided from the record in front of it.** The decision runs
    /// **inside the migration's own transaction** (see [`run_one`]), so the
    /// shape it read is the shape it acts on and nothing can move between the
    /// two.
    WhateverTheRecordAsksFor(fn(&rusqlite::Connection) -> Result<Vec<String>, RecordFailure>),
    /// A new installation never held the withdrawn key. Existing records may
    /// have stopped after storing it and before writing its reference.
    WithTheRecordsPreviousExistence(
        fn(&rusqlite::Connection, bool) -> Result<Vec<String>, RecordFailure>,
    ),
}

/// Every migration this binary carries, in order. **Forward only**: nothing
/// here undoes anything, and a version is never produced twice.
pub const ALL: &[Migration] = &[
    Migration {
        produces: 1,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_1),
    },
    Migration {
        produces: 2,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_2),
    },
    Migration {
        produces: 3,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_3),
    },
    Migration {
        produces: 4,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_4),
    },
    Migration {
        produces: 5,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_5),
    },
    Migration {
        produces: 6,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_6),
    },
    Migration {
        produces: 7,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_7),
    },
    Migration {
        produces: 8,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_8),
    },
    Migration {
        produces: 9,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_9),
    },
    Migration {
        produces: 10,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_10),
    },
    Migration {
        produces: 11,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_11),
    },
    Migration {
        produces: 12,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_12),
    },
    Migration {
        produces: 13,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_13),
    },
    Migration {
        produces: 14,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_14),
    },
    Migration {
        produces: 15,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_15),
    },
    Migration {
        produces: 16,
        what_it_runs: WhatItRuns::WhateverTheRecordAsksFor(schema::version_16),
    },
    Migration {
        produces: 17,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_17),
    },
    Migration {
        produces: 18,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_18),
    },
    Migration {
        produces: 19,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_19),
    },
    Migration {
        produces: 20,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_20),
    },
    Migration {
        produces: 21,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_21),
    },
    Migration {
        produces: 22,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_22),
    },
    Migration {
        produces: 23,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_23),
    },
    Migration {
        produces: 24,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_24),
    },
    Migration {
        produces: 25,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_25),
    },
    Migration {
        produces: 26,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_26),
    },
    Migration {
        produces: 27,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_27),
    },
    Migration {
        produces: 28,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_28),
    },
    Migration {
        produces: 29,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_29),
    },
    Migration {
        produces: 30,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_30),
    },    Migration {
        produces: 31,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_31),
    },
    Migration {
        produces: 32,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_32),
    },
    Migration {
        produces: 33,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_33),
    },
    Migration {
        produces: 34,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_34),
    },
    Migration {
        produces: 35,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_35),
    },
    Migration {
        produces: 36,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_36),
    },
    Migration {
        produces: 37,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_37),
    },
    Migration {
        produces: 38,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_38),
    },
    Migration {
        produces: 39,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_39),
    },
    Migration {
        produces: 40,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_40),
    },
    Migration {
        produces: 41,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_41),
    },
    Migration {
        produces: 42,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_42),
    },
    Migration {
        produces: 43,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_43),
    },
    Migration {
        produces: 44,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_44),
    },
    Migration {
        produces: 45,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_45),
    },
    Migration {
        produces: 46,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_46),
    },
    Migration {
        produces: 47,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_47),
    },
    Migration {
        produces: 48,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_48),
    },
    Migration {
        produces: 49,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_49),
    },
    Migration {
        produces: 50,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_50),
    },
    Migration {
        produces: 51,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_51),
    },
    Migration {
        produces: 52,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_52),
    },
    Migration {
        produces: 53,
        what_it_runs: WhatItRuns::WithTheRecordsPreviousExistence(the_paid_record_is_withdrawn),
    },
    Migration {
        produces: 54,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_54),
    },
    Migration {
        produces: 55,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_55),
    },
    Migration {
        produces: 56,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_56),
    },
    Migration {
        produces: 57,
        what_it_runs: WhatItRuns::TheseStatements(schema::VERSION_57),
    },
];

/// Free-and-open-source migration. The fixed subject also covers a crash
/// after the old key was stored but before its reference was written.
fn the_paid_record_is_withdrawn(_: &rusqlite::Connection, existed: bool) -> Result<Vec<String>, RecordFailure> {
    let mut statements = Vec::new();
    if existed {
        statements.push(format!("INSERT OR IGNORE INTO owed_erase (kind, subject) VALUES ('licence key', '{}')", super::key_rows::LEGACY_LICENCE_KEY_SUBJECT));
    }
    statements.extend([
        "INSERT OR IGNORE INTO owed_erase (kind, subject) SELECT 'licence key', licence_key_subject FROM licence_record WHERE licence_key_subject IS NOT NULL".into(),
        "DROP TABLE licence_slot".into(),
        "DROP TABLE licence_record".into(),
        "DELETE FROM told_entry WHERE thing = 'the lock policy'".into(),
    ]);
    Ok(statements)
}

/// Bring the record at `path` up to [`BINARY_KNOWS`].
///
/// **L9: a binary that meets a record newer than its own schema version does
/// not open it and says so.** It does not read the fields it recognises and
/// ignore the rest — a partial read would be the app being wrong on screen
/// about its own state.
pub fn bring_up_to_date(
    connection: &rusqlite::Connection,
    path: &Path,
    the_record_was_already_there: bool,
) -> Result<(), RecordFailure> {
    let found = schema::version_in(connection)?;

    if found > BINARY_KNOWS {
        return Err(RecordFailure::RecordIsNewerThanTheBinary {
            record: found,
            binary: BINARY_KNOWS,
        });
    }
    if found == BINARY_KNOWS {
        return Ok(());
    }

    // **L8: the copy is taken before a migration runs.** A record that did not
    // exist until this call made it has nothing to copy, and that is the
    // fresh-installation case rather than a failure to protect against.
    //
    // **The question is whether the record was there, not what version it
    // reads.** A record written by a binary from before this store carried a
    // version reads 0, which is the same number a record made a moment ago
    // reads — so taking the version as the test would migrate a real user's
    // record with no copy behind it, and a rollback would then meet L9's
    // refusal with nothing to restore.
    if the_record_was_already_there {
        take_the_copy(path)?;
    }

    for migration in ALL.iter().filter(|migration| migration.produces > found) {
        run_one_for_record(connection, migration, the_record_was_already_there)?;
    }
    Ok(())
}

/// **L7: a migration's partition unit is the whole migration.**
///
/// §9.2 test 1: nothing promises an outcome about half a migration, so the
/// action is **atomic entire**. Test 2 then applies with force, because a
/// half-migrated record is precisely a half-finished unit that could be
/// mistaken for a finished one.
///
/// **How that is kept, and it is worth being exact about.** §5.2 says the
/// migration is *"staged and published in one step, by the platform file
/// adapter's publication operation"*. The engine's own transaction is what
/// stages and publishes it: the statements and the version are one transaction,
/// and the engine's own page says the commit *"will not corrupt the
/// database"* and, at the setting this record is opened with, is durable across
/// a power loss. **The record at rest is one file either way** — its own
/// publication's *"A database in SQLite is a single disk file"* — so there is
/// never a moment where a reader can see half of it. **The file-level staging
/// and rename the clause describes is what [`super::restore_the_copy`] uses for
/// L13's rollback**, which is the other direction of the same act.
///
/// **L10: a migration writes no secret value**, into no table, no backup and no
/// crash record. That is kept by there being nothing to write one from: no
/// migration statement carries a value at all, and the record holds no secret
/// (`Security and licence` §9).
fn run_one_for_record(connection: &rusqlite::Connection, migration: &Migration, existed: bool) -> Result<(), RecordFailure> {
    connection.execute_batch("BEGIN IMMEDIATE")?;

    let attempt = (|| -> Result<(), RecordFailure> {
        match migration.what_it_runs {
            WhatItRuns::TheseStatements(statements) => {
                for statement in statements {
                    connection.execute_batch(statement)?;
                }
            }
            // **The shape is read here, inside the transaction that acts on
            // it.** `BEGIN IMMEDIATE` is already held, so the table this asked
            // about is the table the statements meet. Deciding outside would be
            // a reading and an act with a gap between them, which is the shape
            // of every store fault this stage has met.
            WhatItRuns::WhateverTheRecordAsksFor(decide) => {
                for statement in decide(connection)? {
                    connection.execute_batch(&statement)?;
                }
            }
            WhatItRuns::WithTheRecordsPreviousExistence(decide) => {
                for statement in decide(connection, existed)? {
                    connection.execute_batch(&statement)?;
                }
            }
        }
        schema::set_version(connection, migration.produces)?;
        Ok(())
    })();

    match attempt {
        Ok(()) => {
            connection.execute_batch("COMMIT")?;
            Ok(())
        }
        Err(failure) => {
            // The whole migration or none of it. A rollback that itself fails
            // leaves the original failure standing, which is the one the caller
            // has to act on.
            let _ = connection.execute_batch("ROLLBACK");
            Err(failure)
        }
    }
}

#[cfg(test)]
fn run_one(connection: &rusqlite::Connection, migration: &Migration) -> Result<(), RecordFailure> {
    run_one_for_record(connection, migration, true)
}

/// **L11, L12 and L12a, kept as a check a migration cannot pass without
/// meeting.**
///
/// L11: a migration never removes a suppression row and never rewrites a
/// warning identity. **Row 290's failure is invisible until a user presses the
/// control and one warning does not come back**, and a migration is the one
/// place that failure could be introduced silently.
///
/// L12: a migration never removes a history tombstone. §10.1 keeps a tombstone
/// *"for the life of the local record"* and it is cleared by nothing; a
/// migration is part of that life.
///
/// L12a: a migration never empties the local job record. **It joins the other
/// two on the same terms and is the worst of the three to lose**, the user's
/// whole render history living in that table since version 23; the failure it
/// guards against is the one this family of clauses exists for, which is
/// silent.
///
/// This counts the protected tables before and after, so a migration that
/// empties one is a failure rather than a silence.
pub fn nothing_protected_was_lost(
    connection: &rusqlite::Connection,
    before: &[(String, i64)],
) -> Result<(), RecordFailure> {
    for (table, was) in before {
        let now = count_in(connection, table)?;
        if now < *was {
            return Err(RecordFailure::Engine {
                reported: format!(
                    "a migration removed rows from {table}, which L11, L12 and L12a forbid: {was} became {now}"
                ),
            });
        }
    }
    Ok(())
}

/// The counts L11, L12 and L12a are checked against.
pub fn protected_counts(
    connection: &rusqlite::Connection,
) -> Result<Vec<(String, i64)>, RecordFailure> {
    let mut counts = Vec::new();
    for table in schema::TABLES_A_MIGRATION_NEVER_EMPTIES {
        let exists: bool = connection.query_row(
            "SELECT count(*) > 0 FROM sqlite_schema WHERE type = 'table' AND name = ?1",
            [table],
            |row| row.get(0),
        )?;
        if exists {
            counts.push((table.to_string(), count_in(connection, table)?));
        }
    }
    Ok(counts)
}

fn count_in(connection: &rusqlite::Connection, table: &str) -> Result<i64, RecordFailure> {
    Ok(
        connection.query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
            row.get(0)
        })?,
    )
}

/// Whether L8's copy is still beside the record.
pub fn the_copy_is_there(path: &Path) -> bool {
    matches!(
        files::outside_program_wrote(&copy_beside(path)),
        files::Recheck::Present { .. }
    )
}

/// The one migration step, reachable from the record's own tests so that L7's
/// *atomic entire* is proved against a migration that fails rather than argued
/// about one that cannot.
#[cfg(test)]
pub fn run_one_for_tests(
    connection: &rusqlite::Connection,
    migration: &Migration,
) -> Result<(), RecordFailure> {
    // Callers build a fresh throwaway record one version at a time. Tests of
    // an upgrade open that record through LocalRecord after closing it.
    run_one_for_record(connection, migration, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;

    fn scratch(name: &str) -> std::path::PathBuf {
        crate::scratch_folders::a_folder(format!("smoozy-migration-test-{name}-{}", std::process::id()))
            .join("local-record.sqlite3")
    }

    /// **Record version 25 carries every job record across, and L12a holds** —
    /// step 10c piece 6. A record at version 24 holding step 10b's job rows,
    /// one of them marked that its start was never heard, is opened by this
    /// binary: every row is still there, the mark with it, and each reads no
    /// kept setup, which is what a record from before the writer holds.
    #[test]
    fn version_25_carries_every_job_record_across_with_no_kept_setup() {
        let path = scratch("version-25");
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 24) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 24);
            for (job, standing, heard, never_heard) in [
                ("a-job", "confirmed", 1, 0),
                ("b-job", "provisional", 0, 0),
                ("c-job", "provisional", 0, 1),
            ] {
                connection
                    .execute(
                        "INSERT INTO job_record (
                             job, server, standing, render_start_learned,
                             submitted_with_no_scene_named, blender_build_path,
                             blender_version, blender_origin, device_ask, file_values_stood,
                             start_never_heard
                         ) VALUES (?1, 'a-server', ?2, ?3, 0, '/blender', '4.5.0',
                                   'resolved for them', 'the file''s own, unread', 0, ?4)",
                        rusqlite::params![job, standing, heard, never_heard],
                    )
                    .unwrap();
            }
        }
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };
        assert!(before.iter().any(|(table, count)| table == "job_record" && *count == 3));

        let record = LocalRecord::open_at(&path).expect("the version-24 record opens");
        // **The binary migrates to whatever it knows and this test is about
        // version 25's own column**, so it reads the version against
        // [`BINARY_KNOWS`] rather than against a number written here. *(It read
        // 25 twice until step 10c piece 10 added version 26, which is exactly
        // the fact-in-two-places shape this stage keeps paying for.)*
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(
            ALL.iter().any(|one| one.produces == 25),
            "version 25 is the kept setup's and this binary carries it"
        );
        nothing_protected_was_lost(record.connection(), &before)
            .expect("the job record came across whole");
        let rows: Vec<(String, i64, Option<String>)> = record
            .connection()
            .prepare("SELECT job, start_never_heard, kept_setup FROM job_record ORDER BY job")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(
            rows,
            vec![
                ("a-job".to_string(), 0, None),
                ("b-job".to_string(), 0, None),
                ("c-job".to_string(), 1, None),
            ]
        );
        assert!(
            the_copy_is_there(&path),
            "L8: the copy is taken beside the record before the migration runs"
        );

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }
    /// **Record version 26 carries an existing record forward, and the two
    /// `Download` stores arrive empty** — step 10c piece 10, and this is the
    /// dangerous half of that piece.
    ///
    /// The record is built to version 25 by hand, given job rows, an upload
    /// request and a settings row, and then opened by this binary. **Every
    /// protected table comes across**, the kept-setup column is untouched, and
    /// the five new tables exist with nothing in them — which is what a record
    /// written before this store holds.
    #[test]
    fn version_26_carries_a_version_25_record_forward_and_adds_the_two_download_stores() {
        let path = scratch("version-26");
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 25) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 25);
            for job in ["a-job", "b-job"] {
                connection
                    .execute(
                        "INSERT INTO job_record (
                             job, server, standing, render_start_learned,
                             submitted_with_no_scene_named, blender_build_path,
                             blender_version, blender_origin, device_ask, file_values_stood,
                             start_never_heard, kept_setup
                         ) VALUES (?1, 'a-server', 'confirmed', 1, 0, '/blender', '4.5.0',
                                   'resolved for them', 'the file''s own, unread', 0, 0,
                                   'a payload')",
                        [job],
                    )
                    .unwrap();
            }
            connection
                .execute(
                    "INSERT INTO upload_request
                       (record, server_row, destination, opened_by_run)
                     VALUES ('a-record', 'a-server', X'2f7570', 'a-run')",
                    [],
                )
                .unwrap();
            // **The five tables do not exist before the migration**, which is
            // what makes the assertion below about this migration rather than
            // about the record's shape in general.
            for table in THE_TWO_DOWNLOAD_STORES_TABLES {
                let exists: bool = connection
                    .query_row(
                        "SELECT count(*) > 0 FROM sqlite_schema WHERE type = 'table' AND name = ?1",
                        [table],
                        |row| row.get(0),
                    )
                    .unwrap();
                assert!(!exists, "{table} exists at version 25");
            }
        }
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };

        let record = LocalRecord::open_at(&path).expect("the version-25 record opens");
        // **It is carried to whatever this binary knows**, which version 27
        // moved past 26 (step 10c's sixth pass).
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(BINARY_KNOWS >= 26);
        nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing protected was lost bringing 25 forward to 26");

        // **What stood at version 25 still stands**, the kept setup with it.
        let jobs: Vec<(String, Option<String>)> = record
            .connection()
            .prepare("SELECT job, kept_setup FROM job_record ORDER BY job")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(
            jobs,
            vec![
                ("a-job".to_string(), Some("a payload".to_string())),
                ("b-job".to_string(), Some("a payload".to_string())),
            ]
        );
        let uploads: i64 = record
            .connection()
            .query_row("SELECT count(*) FROM upload_request", [], |row| row.get(0))
            .unwrap();
        assert_eq!(uploads, 1, "File Browser's own request came across");

        // **The two stores are there and empty.** `State, identity and
        // recovery` §11.1 is user-held, so nothing is invented for a record
        // that never had one.
        for table in THE_TWO_DOWNLOAD_STORES_TABLES {
            let count: i64 = record
                .connection()
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| row.get(0))
                .unwrap();
            assert_eq!(count, 0, "{table} arrived with rows in it");
        }
        assert!(
            the_copy_is_there(&path),
            "L8: the copy is taken beside the record before the migration runs"
        );

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **Record version 30 carries a version-29 record forward and the
    /// *Show hidden files* choice arrives off** — `File Browser` L20, out of
    /// Row 520. The record is built to version 29 by hand, given a job row, an
    /// upload request and a settings row holding a folder and a cap, and then
    /// opened by this binary: every protected table comes across, the settings
    /// row keeps what it held, the new column reads as nothing — which is off
    /// — and the tick then writes on the migrated record.
    #[test]
    fn version_30_carries_a_version_29_record_forward_with_hidden_files_off() {
        let path = scratch("version-30");
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 29) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 29);
            connection
                .execute(
                    "INSERT INTO job_record (
                         job, server, standing, render_start_learned,
                         submitted_with_no_scene_named, blender_build_path,
                         blender_version, blender_origin, device_ask, file_values_stood,
                         start_never_heard, kept_setup
                     ) VALUES ('a-job', 'a-server', 'confirmed', 1, 0, '/blender', '4.5.0',
                               'resolved for them', 'the file''s own, unread', 0, 0, 'a payload')",
                    [],
                )
                .unwrap();
            connection
                .execute(
                    "INSERT INTO upload_request
                       (record, server_row, destination, opened_by_run)
                     VALUES ('a-record', 'a-server', X'2f7570', 'a-run')",
                    [],
                )
                .unwrap();
            connection
                .execute(
                    "INSERT INTO setting (the_one_row, download_folder, upload_cap_bytes_per_second)
                     VALUES (1, X'2f52656e64657273', 12500000)",
                    [],
                )
                .unwrap();
            // **The column does not exist before the migration**, which is
            // what makes the assertions below about this migration.
            let has_it: bool = connection
                .query_row(
                    "SELECT count(*) > 0 FROM pragma_table_info('setting') WHERE name = 'hidden_files_shown'",
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            assert!(!has_it, "the column exists at version 29");
        }
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };

        let record = LocalRecord::open_at(&path).expect("the version-29 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(BINARY_KNOWS >= 30);
        nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing protected was lost bringing 29 forward to 30");
        let c = record.connection();
        let settings = crate::record::settings::read(c).expect("the settings read");
        assert_eq!(settings.the_chosen_download_folder.as_deref(), Some(&b"/Renders"[..]));
        assert_eq!(
            settings.the_upload_cap,
            crate::scheduling::TheCap::Of(crate::scheduling::BytesPerSecond(12_500_000))
        );
        assert!(
            !crate::record::settings::hidden_files_are_shown(c).expect("the choice reads"),
            "a record from before the choice shows hidden files"
        );
        crate::record::settings::hidden_files_were_switched(c, true).expect("the tick is written");
        assert!(crate::record::settings::hidden_files_are_shown(c).unwrap());
        let jobs: i64 = c.query_row("SELECT count(*) FROM job_record", [], |row| row.get(0)).unwrap();
        assert_eq!(jobs, 1);
        assert!(
            the_copy_is_there(&path),
            "L8: the copy is taken beside the record before the migration runs"
        );
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **Record version 31 carries a version-30 record forward and the file
    /// acts' tables arrive empty** — `File Browser` §11's record for a delete,
    /// a move and a copy, step 10c's Files' owed acts piece 4. The record is
    /// built to version 30 by hand, given a job row, an upload request and an
    /// outcome record, and then opened by this binary: every protected table
    /// comes across, the shared outcome record keeps what it held, and a
    /// delete is then written and read back on the migrated record.
    #[test]
    fn version_31_carries_a_version_30_record_forward_with_the_file_acts_tables_empty() {
        let path = scratch("version-31");
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 30) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 30);
            connection
                .execute(
                    "INSERT INTO job_record (
                         job, server, standing, render_start_learned,
                         submitted_with_no_scene_named, blender_build_path,
                         blender_version, blender_origin, device_ask, file_values_stood,
                         start_never_heard, kept_setup
                     ) VALUES ('a-job', 'a-server', 'confirmed', 1, 0, '/blender', '4.5.0',
                               'resolved for them', 'the file''s own, unread', 0, 0, 'a payload')",
                    [],
                )
                .unwrap();
            connection
                .execute("INSERT INTO outcome_record (identity, action) VALUES ('an-upload', 'upload')", [])
                .unwrap();
            let has_it: bool = connection
                .query_row(
                    "SELECT count(*) > 0 FROM sqlite_schema WHERE type = 'table' AND name = 'file_act'",
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            assert!(!has_it, "the table exists at version 30");
        }
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };

        let record = LocalRecord::open_at(&path).expect("the version-30 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(BINARY_KNOWS >= 31);
        nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing protected was lost bringing 30 forward to 31");
        let c = record.connection();
        let outcomes: i64 = c.query_row("SELECT count(*) FROM outcome_record", [], |row| row.get(0)).unwrap();
        assert_eq!(outcomes, 1, "the shared outcome record lost what it held");
        let acts: i64 = c.query_row("SELECT count(*) FROM file_act", [], |row| row.get(0)).unwrap();
        assert_eq!(acts, 0);
        use smoozy_state::identities::{Identity, IdentityKind};
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let keeper = crate::record::file_acts::FileActs::over(c);
        let id = keeper
            .the_act_is_written(
                &crate::record::file_acts::AnAct {
                    act: crate::record::file_acts::WhichAct::Delete,
                    server_row: issue(IdentityKind::LocalServer),
                    installation: None,
                    opened_by_run: issue(IdentityKind::Run),
                    units: vec![crate::record::file_acts::AUnit {
                        role: crate::record::file_acts::ONE_ENTRY,
                        source: b"/home/ubuntu/old.png".to_vec(),
                        destination: None,
                        staging: None,
                    }],
                    the_jobs_told: Vec::new(),
                },
                &[],
            )
            .expect("a delete is written on the migrated record");
        assert_eq!(keeper.read(&id).unwrap().expect("held").units.len(), 1);
        assert!(
            the_copy_is_there(&path),
            "L8: the copy is taken beside the record before the migration runs"
        );
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **Record version 32 carries a version-31 record forward with every
    /// mark as it was, and a delete's count then reads back** — the build
    /// runner's ruling (d) of 2026-10-03. A count of one is refused: one file
    /// is named, never counted.
    #[test]
    fn version_32_carries_the_marks_forward_and_holds_a_deletes_count() {
        let path = scratch("version-32");
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 31) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 31);
            connection
                .execute(
                    "INSERT INTO job_record_disturbance VALUES ('a-job', 0, '/a.png', 'replace', 'disturbed')",
                    [],
                )
                .unwrap();
        }
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };
        let record = LocalRecord::open_at(&path).expect("the version-31 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(BINARY_KNOWS >= 32);
        nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing protected was lost bringing 31 forward to 32");
        let c = record.connection();
        let kept: Option<i64> = c
            .query_row("SELECT how_many FROM job_record_disturbance WHERE job = 'a-job'", [], |row| row.get(0))
            .unwrap();
        assert_eq!(kept, None, "a mark written before version 32 counts nothing");
        c.execute(
            "INSERT INTO job_record_disturbance VALUES ('a-job', 1, '/root/job/frames', 'delete', 'disturbed', 12)",
            [],
        )
        .unwrap();
        assert!(
            c.execute(
                "INSERT INTO job_record_disturbance VALUES ('a-job', 2, '/root/job/frames', 'delete', 'disturbed', 1)",
                [],
            )
            .is_err(),
            "a count of one was taken"
        );
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **Record version 43 carries a version-42 record forward, and the
    /// installs still waiting on a server can then be kept and read back** —
    /// `Server Setup` Y9a, step 10c's owed piece 4. The record is built to
    /// version 42 by hand, given a server row, and opened by this binary: the
    /// row comes across, the new table arrives empty, and a list is written
    /// and read back on the migrated record. **Without version 43 the table is
    /// not there and this fails.**
    #[test]
    fn version_43_carries_a_version_42_record_forward_with_nothing_waiting() {
        let path = scratch("version-43");
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 42) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 42);
            connection
                .execute(
                    "INSERT INTO server_row (
                         identifier, host, port, username, key_identifier, key_fingerprint,
                         remembered_setup
                     ) VALUES ('a-server', 'h', 22, 'root', 'k', 'f', 0)",
                    [],
                )
                .unwrap();
            let has_it: bool = connection
                .query_row(
                    "SELECT count(*) > 0 FROM sqlite_schema WHERE type = 'table' AND name = 'server_install_not_begun'",
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            assert!(!has_it, "the table exists at version 42");
        }
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };
        let record = LocalRecord::open_at(&path).expect("the version-42 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(BINARY_KNOWS >= 43);
        nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing protected was lost bringing 42 forward to 43");
        let c = record.connection();
        let rows: i64 = c.query_row("SELECT count(*) FROM server_row", [], |row| row.get(0)).unwrap();
        assert_eq!(rows, 1, "the server row did not come across");
        use crate::record::installs_not_begun::{the_ones_waiting_are_kept, the_ones_waiting_on};
        assert!(the_ones_waiting_on(c, "a-server").unwrap().is_empty(), "something was waiting before anybody kept it");
        the_ones_waiting_are_kept(c, "a-server", &["5.2.2".to_string(), "4.5.14".to_string()]).unwrap();
        assert_eq!(the_ones_waiting_on(c, "a-server").unwrap(), ["5.2.2", "4.5.14"]);
        assert!(the_copy_is_there(&path), "L8: the copy is taken beside the record before the migration runs");
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **Record version 54 carries a version-53 record forward, and the
    /// installs a closed Smoozy left waiting read as installs from the list,
    /// while acts of every kind can then be kept** — `Server Setup` Y9a item
    /// 10, the nineteenth notes, note 2. The record is built to version 53 by
    /// hand, given a server row and two waiting installs in version 43's shape,
    /// and opened by this binary. Nothing protected is lost, the two rows come
    /// across in order as installs, and a mixed line is written and read back.
    /// **Without version 54 the column is not there and this fails.**
    #[test]
    fn version_54_carries_a_version_53_record_forward_with_its_installs_as_installs() {
        use crate::record::installs_not_begun::{
            the_acts_waiting_are_kept, the_acts_waiting_on, the_ones_waiting_on, AnActKept, TheActsKind,
        };
        let path = scratch("version-54");
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 53) {
                run_one_for_tests(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 53);
            connection
                .execute(
                    "INSERT INTO server_row (
                         identifier, host, port, username, key_identifier, key_fingerprint,
                         remembered_setup
                     ) VALUES ('a-server', 'h', 22, 'root', 'k', 'f', 0)",
                    [],
                )
                .unwrap();
            connection
                .execute_batch(
                    "INSERT INTO server_install_not_begun (server, position, version) VALUES ('a-server', 0, '5.2.3');
                     INSERT INTO server_install_not_begun (server, position, version) VALUES ('a-server', 1, '4.5.15');",
                )
                .unwrap();
            assert!(
                !schema::the_table_has_the_column(&connection, "server_install_not_begun", "act").unwrap(),
                "the column exists at version 53"
            );
        }
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };
        let record = LocalRecord::open_at(&path).expect("the version-53 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(BINARY_KNOWS >= 54);
        nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing protected was lost bringing 53 forward to 54");
        let c = record.connection();
        assert_eq!(
            the_acts_waiting_on(c, "a-server").unwrap(),
            [AnActKept::an_install("5.2.3"), AnActKept::an_install("4.5.15")],
            "the installs a closed Smoozy left did not come across as installs, in order"
        );
        assert_eq!(the_ones_waiting_on(c, "a-server").unwrap(), ["5.2.3", "4.5.15"]);
        let mixed = vec![
            AnActKept { kind: TheActsKind::ARemoval, named: "5.2.2".to_string() },
            AnActKept { kind: TheActsKind::TheReadyAction, named: String::new() },
        ];
        the_acts_waiting_are_kept(c, "a-server", &mixed).unwrap();
        assert_eq!(the_acts_waiting_on(c, "a-server").unwrap(), mixed);
        assert!(the_copy_is_there(&path), "L8: the copy is taken beside the record before the migration runs");
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **Record version 44 carries a version-43 record forward, and a
    /// preview's source can then be kept and read back** — the Creator's
    /// tenth notes, note 2. The record is built to version 43 by hand, in a
    /// throwaway folder, and given a job and two held copies of one frame —
    /// the shape version 40 left; it is then opened by this binary. The rows
    /// come across untouched and read with no source, nothing protected is
    /// lost, and a copy owed with the output path as its source is kept and
    /// read back first in its frame. **Without version 44 the column is not
    /// there and this fails.** Nothing of the Creator's is opened.
    #[test]
    fn version_44_carries_a_version_43_record_forward_with_no_source_on_any_copy() {
        use crate::record::previews::{a_derivative_is_owed, every_copy_held_for};
        use smoozy_job_files::main_output::WhereThePictureCameFrom;
        let path = scratch("version-44");
        let server = smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::LocalServer).unwrap();
        let made = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        let job = made.job.clone();
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 43) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 43);
            crate::record::job_records::JobRecords::over(&connection)
                .write_the_provisional_record(&made)
                .unwrap();
            for (layer, main, place, copy) in [(Some(&b"Box"[..]), 1, 0, "/p/box.png"), (Some(&b"Mattress"[..]), 0, 1, "/p/m.png")] {
                connection
                    .execute(
                        "INSERT INTO job_record_preview
                            (job, frame, operation, view_layer, owed, copy, published_on_this_computer_at, main_copy, place)
                         VALUES (?1, 7, 'op', ?2, 0, ?3, 1760000000000, ?4, ?5)",
                        rusqlite::params![job.as_text(), layer, copy, main, place],
                    )
                    .unwrap();
            }
            let has_it: bool = connection
                .query_row(
                    "SELECT count(*) > 0 FROM pragma_table_info('job_record_preview') WHERE name = 'source'",
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            assert!(!has_it, "the column exists at version 43");
        }
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };
        let record = LocalRecord::open_at(&path).expect("the version-43 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(BINARY_KNOWS >= 44);
        nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing protected was lost bringing 43 forward to 44");
        let c = record.connection();
        let held = every_copy_held_for(c, &job).unwrap();
        assert_eq!(
            held.iter().map(|one| (one.view_layer.clone(), one.the_main_copy, one.place, one.source.clone())).collect::<Vec<_>>(),
            vec![(Some(b"Box".to_vec()), true, 0, None), (Some(b"Mattress".to_vec()), false, 1, None)],
            "the rows came across as they were, with no source"
        );
        a_derivative_is_owed(c, &job, 8, "op-8", None, true, 0, Some(&WhereThePictureCameFrom::TheOutputPath)).unwrap();
        a_derivative_is_owed(c, &job, 8, "op-8", Some(b"Box"), false, 0, None).unwrap();
        crate::record::previews::the_copy_was_published(c, &job, 8, Some(b"Box"), "op-8", std::path::Path::new("/p/b8.png"), std::time::SystemTime::now()).unwrap();
        crate::record::previews::the_copy_was_published(c, &job, 8, None, "op-8", std::path::Path::new("/p/8.png"), std::time::SystemTime::now()).unwrap();
        let eight: Vec<_> = every_copy_held_for(c, &job).unwrap().into_iter().filter(|one| one.frame == 8).collect();
        assert_eq!(eight[0].source, Some(WhereThePictureCameFrom::TheOutputPath), "kept, and first in its frame");
        assert_eq!(eight[1].source, None);
        assert!(the_copy_is_there(&path), "L8: the copy is taken beside the record before the migration runs");
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// Every row of one table, every column, in a stable order — so a
    /// migration that rewrites a stored row is caught, not just one that
    /// drops it.
    fn every_row_of(connection: &rusqlite::Connection, table: &str) -> Vec<Vec<rusqlite::types::Value>> {
        let mut statement = connection.prepare(&format!("SELECT * FROM {table} ORDER BY rowid")).unwrap();
        let width = statement.column_count();
        statement
            .query_map([], |row| (0..width).map(|at| row.get::<_, rusqlite::types::Value>(at)).collect())
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    #[test]
    fn version_46_keeps_a_legacy_bot_key_and_its_removal_debt() {
        use smoozy_telegram::proof::{KeepsTheProvedBot, TheBot, TheConversation};
        use crate::record::telegram_bot::{TheProvedBotRecord, the_current_bot_tokens_item};
        use smoozy_telegram::walkthrough::the_bot_tokens_item;
        let path = scratch("version-46");
        {
            let c = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 45) {
                run_one(&c, migration).unwrap();
            }
            let keeper = TheProvedBotRecord::over(&c);
            let bot = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
            keeper.the_proof_was_recorded(&bot, TheConversation(9001)).unwrap();
            keeper.the_token_was_stored_for(&bot).unwrap();
            c.execute("INSERT INTO owed_erase (kind, subject) VALUES ('bot token', 'telegram-bot')", []).unwrap();
        }
        let record = LocalRecord::open_at(&path).unwrap();
        let c = record.connection();
        let keeper = TheProvedBotRecord::over(c);
        assert_eq!(keeper.the_bot_whose_token_is_stored().unwrap(), Some(7));
        assert_eq!(the_current_bot_tokens_item(c).unwrap(), the_bot_tokens_item());
        let (bot, _) = keeper.the_proved_bot().unwrap().unwrap();
        let new_item = keeper.the_item_for_saving(&bot).unwrap();
        assert_ne!(new_item, the_bot_tokens_item(), "the owed item cannot be reused");
        assert_eq!(keeper.the_bot_whose_token_is_stored().unwrap(), None);
        assert_eq!(c.query_row("SELECT COUNT(*) FROM owed_erase WHERE kind = 'bot token' AND subject = 'telegram-bot'", [], |r| r.get::<_, i64>(0)).unwrap(), 1);
        drop(record);
        let reopened = LocalRecord::open_at(&path).unwrap();
        assert_eq!(the_current_bot_tokens_item(reopened.connection()).unwrap(), new_item, "a retry uses the same reserved item");
    }

    /// **Record version 45 carries a version-44 record forward, and a
    /// submission's exact material can then be kept and read back** —
    /// `Rendering` S4b and S9, and the check of 2026-10-19 that found this
    /// test missing. The record is built to version 44 by hand, in a
    /// throwaway folder, and given a job, a preview copy and a server row; it
    /// is then opened by this binary. Nothing protected is lost, the three
    /// tables' rows come across with every column as it was, the new table
    /// arrives empty, and a kept material writes and reads back. **Without
    /// version 45 the table is not there and this fails.** Nothing of the
    /// Creator's is opened.
    #[test]
    fn version_45_carries_a_version_44_record_forward_with_no_material_kept() {
        let path = scratch("version-45");
        let server = smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::LocalServer).unwrap();
        let made = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        let job = made.job.clone();
        const THE_TABLES: [&str; 3] = ["job_record", "job_record_preview", "server_row"];
        let rows_before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 44) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 44);
            crate::record::job_records::JobRecords::over(&connection)
                .write_the_provisional_record(&made)
                .unwrap();
            connection
                .execute(
                    "INSERT INTO job_record_preview
                        (job, frame, operation, view_layer, owed, copy, published_on_this_computer_at, main_copy, place, source)
                     VALUES (?1, 7, 'op', NULL, 0, '/p/7.png', 1760000000000, 1, 0, NULL)",
                    [job.as_text()],
                )
                .unwrap();
            connection
                .execute(
                    "INSERT INTO server_row (
                         identifier, host, port, username, key_identifier, key_fingerprint,
                         remembered_setup
                     ) VALUES (?1, 'h', 22, 'root', 'k', 'f', 0)",
                    [server.as_text()],
                )
                .unwrap();
            let has_it: bool = connection
                .query_row(
                    "SELECT count(*) > 0 FROM sqlite_schema WHERE type = 'table' AND name = 'submission_material'",
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            assert!(!has_it, "the table exists at version 44");
            THE_TABLES.map(|table| {
                let rows = every_row_of(&connection, table);
                assert_eq!(rows.len(), 1, "{table} was given one row");
                rows
            })
        };
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };
        let record = LocalRecord::open_at(&path).expect("the version-44 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(BINARY_KNOWS >= 45);
        nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing protected was lost bringing 44 forward to 45");
        let c = record.connection();
        for (table, was) in THE_TABLES.iter().zip(rows_before.iter()) {
            let mut now = every_row_of(c, table);
            if *table == "job_record" {
                // Version 47 appends descriptive fields and version 50 the
                // free space at the ending; legacy jobs receive four NULLs
                // from each, and one more from version 51's typed frames,
                // and one more from version 55's reporting choice, while
                // every pre-existing value stays identical.
                for row in &mut now {
                    assert!(row[row.len()-10..].iter().all(|value| *value == rusqlite::types::Value::Null));
                    row.truncate(row.len()-10);
                }
            }
            assert_eq!(&now, was, "{table}'s existing fields did not come across as they were");
        }
        let kept: i64 = c.query_row("SELECT count(*) FROM submission_material", [], |row| row.get(0)).unwrap();
        assert_eq!(kept, 0, "the new table arrived with rows in it");
        let installation =
            smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::Installation).unwrap();
        crate::record::submission_material::Material {
            job: job.clone(),
            server: server.clone(),
            installation: installation.clone(),
            material_folder: "/root/smoozy/jobs/a-job".to_string(),
            material: b"the manifest, exactly".to_vec(),
            remap_path: "/root/smoozy/remap/a-job.json".to_string(),
            remap: b"the remap, exactly".to_vec(),
            output_folder: Some("/root/out/Kitchen".to_string()),
            done: [false; 3],
        }
        .keep(c)
        .expect("a material is kept on the migrated record");
        let read: (String, String, String, String, Vec<u8>, String, Vec<u8>, Option<String>, i64, i64, i64, i64) = c
            .query_row(
                "SELECT job, server, installation, material_folder, material, remap_path, remap,
                        output_folder, debt, material_done, remap_done, folder_done
                 FROM submission_material",
                [],
                |row| {
                    Ok((
                        row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?,
                        row.get(6)?, row.get(7)?, row.get(8)?, row.get(9)?, row.get(10)?, row.get(11)?,
                    ))
                },
            )
            .unwrap();
        assert_eq!(
            read,
            (
                job.as_text(),
                server.as_text(),
                installation.as_text(),
                "/root/smoozy/jobs/a-job".to_string(),
                b"the manifest, exactly".to_vec(),
                "/root/smoozy/remap/a-job.json".to_string(),
                b"the remap, exactly".to_vec(),
                Some("/root/out/Kitchen".to_string()),
                0,
                0,
                0,
                0,
            ),
            "the material read back as it was kept, owing nothing yet"
        );
        assert!(the_copy_is_there(&path), "L8: the copy is taken beside the record before the migration runs");
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **Record version 49 carries a version-48 record forward, and the update
    /// route's three rows can then be kept** — step 11's update-reading piece.
    /// The record is built to version 48 by hand, in a throwaway folder, and
    /// given a server row and a server-package trust record; it is then opened
    /// by this binary. **Both old rows come across byte for byte and still read
    /// back through their own readers**, nothing protected is lost, L8's copy is
    /// beside the record, the three new tables arrive empty, and each takes a
    /// row and gives it back. **Without version 49 the tables are not there and
    /// this fails.** Nothing of the Creator's is opened.
    #[test]
    fn version_49_carries_a_version_48_record_forward_with_the_update_rows_empty() {
        use crate::record::update_trust;
        use smoozy_server_protocol::release::trust::TheTrustRecord;
        let path = scratch("version-49");
        let the_server_packages = TheTrustRecord {
            greatest_accepted_release_sequence: 1766,
            greatest_accepted_key_set_generation: 1,
            the_statement_accepted_at_that_generation: b"a server-package statement".to_vec(),
            its_root_signature: [9; 64],
        };
        let (servers_before, trust_before) = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 48) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 48);
            connection
                .execute(
                    "INSERT INTO server_row (
                         identifier, host, port, username, key_identifier, key_fingerprint,
                         remembered_setup
                     ) VALUES ('a-server', 'h', 22, 'root', 'k', 'f', 0)",
                    [],
                )
                .unwrap();
            crate::record::server_package_trust::write(&connection, &the_server_packages).unwrap();
            for table in ["update_trust", "update_check", "update_in_probe"] {
                let has_it: bool = connection
                    .query_row(
                        "SELECT count(*) > 0 FROM sqlite_schema WHERE type = 'table' AND name = ?1",
                        [table],
                        |row| row.get(0),
                    )
                    .unwrap();
                assert!(!has_it, "{table} exists at version 48");
            }
            (every_row_of(&connection, "server_row"), every_row_of(&connection, "server_package_trust"))
        };
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };
        let record = LocalRecord::open_at(&path).expect("the version-48 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(BINARY_KNOWS >= 49);
        nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing protected was lost bringing 48 forward to 49");
        let c = record.connection();
        assert_eq!(every_row_of(c, "server_row"), servers_before, "a server row was rewritten");
        assert_eq!(every_row_of(c, "server_package_trust"), trust_before, "the server-package trust record was rewritten");
        assert_eq!(
            crate::record::server_package_trust::read(c).unwrap(),
            Some(the_server_packages),
            "the old trust record does not read back"
        );
        for table in ["update_trust", "update_check", "update_in_probe"] {
            let rows: i64 = c.query_row(&format!("SELECT count(*) FROM {table}"), [], |row| row.get(0)).unwrap();
            assert_eq!(rows, 0, "{table} arrived with rows in it");
        }
        assert_eq!(update_trust::read_the_trust_record(c).unwrap(), None);
        assert_eq!(update_trust::the_last_completed_check(c).unwrap(), None);
        assert_eq!(update_trust::the_declined_version(c).unwrap(), None);
        assert_eq!(update_trust::the_update_in_probe(c).unwrap(), None);
        let kept = crate::updating::trust::TheUpdateTrustRecord {
            greatest_accepted_release_sequence: 3,
            greatest_accepted_trust_sequence: 1,
            the_statement_accepted_at_that_trust_sequence: b"an update statement".to_vec(),
            its_recovery_signature: [4; 64],
        };
        update_trust::write_the_trust_record(c, &kept).unwrap();
        assert_eq!(update_trust::read_the_trust_record(c).unwrap(), Some(kept));
        update_trust::this_version_was_declined(c, "0.2.0").unwrap();
        assert_eq!(update_trust::the_declined_version(c).unwrap().as_deref(), Some("0.2.0"));
        assert_eq!(
            crate::record::server_package_trust::read(c).unwrap().map(|it| it.greatest_accepted_release_sequence),
            Some(1766),
            "the update record and the server-package record share a field"
        );
        assert!(the_copy_is_there(&path), "L8: the copy is taken beside the record before the migration runs");
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **Record version 51 carries a version-50 record forward, and the frame
    /// text the user typed can then be kept and read back** — `Queue & History`
    /// P5b, the eighteenth notes, piece 7. The record is built to version 50 by
    /// hand, in a throwaway folder, and given a job; it is then opened by this
    /// binary. Nothing protected is lost, the old row comes across with every
    /// column as it was and the new one empty, the old record still reads back
    /// whole and draws no typed text, and a new submission keeps its text.
    /// **Without version 51 the column is not there and this fails.** Nothing of
    /// the Creator's is opened.
    /// **Version 55 (`Telegram` V1c, the twenty-first notes, note 7): a
    /// version-54 job comes across whole with no reporting choice**, and the
    /// column takes one afterwards.
    #[test]
    fn version_55_carries_a_version_54_record_forward_with_no_reporting_choice() {
        use crate::record::job_records::JobRecords;
        let path = scratch("version-55");
        let server = smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::LocalServer).unwrap();
        let old = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        let rows_before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 54) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 54);
            JobRecords::over(&connection).write_the_provisional_record(&old).unwrap();
            assert!(!schema::the_table_has_the_column(&connection, "job_record", "reporting_choice").unwrap());
            every_row_of(&connection, "job_record")
        };
        let record = LocalRecord::open_at(&path).expect("the version-54 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        let c = record.connection();
        let mut now = every_row_of(c, "job_record");
        for row in &mut now {
            assert_eq!(row.last(), Some(&rusqlite::types::Value::Null), "an old job arrived with a reporting choice");
            row.pop();
        }
        assert_eq!(now, rows_before, "the old job's fields did not come across as they were");
        let records = JobRecords::over(c);
        assert_eq!(records.read(&old.job).unwrap().as_ref(), Some(&old));
        assert_eq!(records.the_reporting_choice_of(&old.job).unwrap(), None);
        records.the_reporting_choice_is_written(&old.job, "\"Everything\"").unwrap();
        assert_eq!(records.the_reporting_choice_of(&old.job).unwrap().as_deref(), Some("\"Everything\""));
    }

    /// **Version 56 (`Telegram` V1a as amended, `Telegram` V1d): a version-55
    /// Telegram setting comes across whole**, its three new fields empty, and
    /// takes the default's minutes and a phone's choice afterwards.
    #[test]
    fn version_56_carries_a_version_55_telegram_setting_forward_with_no_minutes_and_no_phones_choice() {
        use crate::record::settings::{the_phones_choice_is_taken_where_newer, the_telegram_settings};
        let path = scratch("version-56");
        let rows_before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 55) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 55);
            connection
                .execute(
                    "INSERT INTO telegram_setting (the_one_row, switched_on, reporting_default) VALUES (1, 1, 'pictures-only')",
                    [],
                )
                .unwrap();
            every_row_of(&connection, "telegram_setting")
        };
        let record = LocalRecord::open_at(&path).expect("the version-55 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        let c = record.connection();
        let mut now = every_row_of(c, "telegram_setting");
        for row in &mut now {
            assert!(row[row.len() - 3..].iter().all(|value| *value == rusqlite::types::Value::Null));
            row.truncate(row.len() - 3);
        }
        assert_eq!(now, rows_before, "the setting did not come across as it was");
        let held = the_telegram_settings(c).unwrap();
        assert!(held.switched_on);
        assert_eq!(held.the_reporting_default.as_deref(), Some("pictures-only"));
        assert_eq!((held.the_default_minutes, held.the_phones_choice_taken), (None, None));
        assert!(the_phones_choice_is_taken_where_newer(c, "a-picture-every-few-minutes", Some(10), "op", 1).unwrap());
        assert_eq!(the_telegram_settings(c).unwrap().the_default_minutes, Some(10));
    }

    /// **Version 57 (`Download` R5's dated note of 2026-10-31): a version-56
    /// arrangement comes across whole and reads as not yet walked**, which is
    /// how it read before; the completion is written afterwards against its
    /// own start, and a new start reads as not walked again. The twenty-sixth
    /// notes, note 4.
    #[test]
    fn version_57_carries_a_version_56_arrangement_forward_as_not_yet_walked() {
        use crate::record::download_records::DownloadArrangements;
        let path = scratch("version-57");
        let job = smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::Job).unwrap();
        let rows_before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 56) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 56);
            connection
                .execute(
                    "INSERT INTO download_arrangement (job, server_row, destination, narrowing, desired_state, start_order)
                     VALUES (?1, 'row', x'2f64', 'everything', 'on', 4)",
                    [job.as_text()],
                )
                .unwrap();
            every_row_of(&connection, "download_arrangement")
        };
        let record = LocalRecord::open_at(&path).expect("the version-56 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        let c = record.connection();
        let mut now = every_row_of(c, "download_arrangement");
        for row in &mut now {
            assert_eq!(row.last(), Some(&rusqlite::types::Value::Null), "an old arrangement arrived walked");
            row.pop();
        }
        assert_eq!(now, rows_before, "the arrangement did not come across as it was");
        let stores = DownloadArrangements::over(c);
        assert!(!stores.the_ending_walk_is_written_down(&job).unwrap());
        stores.the_ending_walk_completed(&job).unwrap();
        assert!(stores.the_ending_walk_is_written_down(&job).unwrap());
        c.execute("UPDATE download_arrangement SET start_order = 5", []).unwrap();
        assert!(!stores.the_ending_walk_is_written_down(&job).unwrap(), "a new start inherited the old walk");
    }

    #[test]
    fn version_51_carries_a_version_50_record_forward_with_no_typed_frames() {
        use crate::record::job_records::JobRecords;
        let path = scratch("version-51");
        let server = smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::LocalServer).unwrap();
        let old = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen");
        let rows_before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= 50) {
                run_one(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 50);
            JobRecords::over(&connection).write_the_provisional_record(&old).unwrap();
            assert!(
                !schema::the_table_has_the_column(&connection, "job_record", "typed_frames").unwrap(),
                "the column exists at version 50"
            );
            every_row_of(&connection, "job_record")
        };
        let before = {
            let connection = rusqlite::Connection::open(&path).unwrap();
            protected_counts(&connection).unwrap()
        };
        let record = LocalRecord::open_at(&path).expect("the version-50 record opens");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(BINARY_KNOWS >= 51);
        nothing_protected_was_lost(record.connection(), &before)
            .expect("nothing protected was lost bringing 50 forward to 51");
        let c = record.connection();
        let mut now = every_row_of(c, "job_record");
        for row in &mut now {
            // **Version 55's reporting choice comes after it**, also absent.
            assert_eq!(row.last(), Some(&rusqlite::types::Value::Null), "an old job arrived with a reporting choice");
            row.pop();
            assert_eq!(row.last(), Some(&rusqlite::types::Value::Null), "an old job arrived with typed text");
            row.pop();
        }
        assert_eq!(now, rows_before, "the old job's fields did not come across as they were");
        let records = JobRecords::over(c);
        assert_eq!(records.read(&old.job).unwrap().as_ref(), Some(&old), "the old record no longer reads back");
        assert_eq!(records.the_typed_frames_of(&old.job).unwrap(), None);
        let typed = crate::record::deletions::tests::a_record_in_history(&server, "Hall");
        records.write_the_provisional_record(&typed).unwrap();
        c.execute(
            "UPDATE job_record SET typed_frames = '1,2-4' WHERE job = ?1",
            [typed.job.as_text()],
        )
        .unwrap();
        assert_eq!(records.the_typed_frames_of(&typed.job).unwrap().as_deref(), Some("1,2-4"));
        assert!(the_copy_is_there(&path), "L8: the copy is taken beside the record before the migration runs");
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **A fresh record arrives at version 31 with the file acts' tables.**
    #[test]
    fn a_fresh_record_has_the_file_acts_tables() {
        let path = scratch("fresh-31");
        let record = LocalRecord::open_at(&path).expect("a fresh record is made");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        for table in ["file_act", "file_act_unit", "file_act_telling_job"] {
            let count: i64 = record
                .connection()
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| row.get(0))
                .unwrap();
            assert_eq!(count, 0, "{table}");
        }
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **A fresh record arrives at version 30 with the choice off.**
    #[test]
    fn a_fresh_record_has_hidden_files_off() {
        let path = scratch("fresh-30");
        let record = LocalRecord::open_at(&path).expect("a fresh record is made");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        assert!(!crate::record::settings::hidden_files_are_shown(record.connection()).unwrap());
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    /// **A fresh record is made right** — the other half of the dangerous part.
    ///
    /// A record that never existed runs every migration in order and comes out
    /// at [`BINARY_KNOWS`] with the two stores present and empty, **and with no
    /// copy beside it**, because L8's copy is of a record that was there and a
    /// fresh installation has nothing to copy.
    #[test]
    fn a_fresh_record_is_made_at_the_version_this_binary_knows_with_both_stores() {
        let path = scratch("fresh-26");
        let record = LocalRecord::open_at(&path).expect("a fresh record is made");
        assert_eq!(record.schema_version().unwrap(), BINARY_KNOWS);
        for table in THE_TWO_DOWNLOAD_STORES_TABLES {
            let count: i64 = record
                .connection()
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| row.get(0))
                .unwrap();
            assert_eq!(count, 0, "{table} is not there, or arrived with rows in it");
        }
        assert!(
            !the_copy_is_there(&path),
            "a record that did not exist has nothing to copy"
        );
        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    fn a_paid_record(name: &str) -> std::path::PathBuf {
        let path = scratch(name);
        let connection = rusqlite::Connection::open(&path).unwrap();
        // v0.1.8's shipped schema version is 30 (read from that tag).
        for migration in ALL.iter().filter(|one| one.produces <= 30) {
            run_one_for_tests(&connection, migration).unwrap();
        }
        connection.execute_batch("INSERT INTO licence_record (only_row, state, grant_expiry_kind, licence_key_subject) VALUES (1, 'locked', 'not held', 'licence-record');
            INSERT INTO licence_slot VALUES (0, 'slot', 'computer', 1, 2, 'this computer');
            INSERT INTO told_entry (server, thing, value, acknowledged_at, installation) VALUES ('server', 'the lock policy', 'locked', 0, 'installation');
            INSERT INTO told_entry (server, thing, value, acknowledged_at, installation) VALUES ('server', 'role', 'on', 0, 'installation');
            INSERT INTO owed_erase VALUES ('bot token', 'another-secret');").unwrap();
        path
    }

    #[test]
    fn free_software_migrates_the_paid_record_and_keeps_other_told_rows() {
        let path = a_paid_record("free-software-migration");
        let record = LocalRecord::open_at(&path).unwrap();
        let connection = record.connection();
        let paid_tables: i64 = connection.query_row("SELECT count(*) FROM sqlite_schema WHERE name IN ('licence_record', 'licence_slot')", [], |row| row.get(0)).unwrap();
        assert_eq!(paid_tables, 0);
        let things: Vec<String> = connection.prepare("SELECT thing FROM told_entry ORDER BY thing").unwrap().query_map([], |row| row.get(0)).unwrap().collect::<Result<_, _>>().unwrap();
        assert_eq!(things, vec!["role"]);
        use smoozy_security_and_licence::erasure::EraseMarks;
        let marks = crate::record::key_rows::KeyRowMarks(connection).marks().unwrap();
        assert_eq!(connection.query_row("SELECT subject FROM owed_erase WHERE kind = 'licence key'", [], |row| row.get::<_, String>(0)).unwrap(), "licence-record");
        assert!(marks.iter().any(|item| item.subject.as_str() == "another-secret"));
    }

    #[test]
    fn free_software_only_an_existing_record_can_owe_the_fixed_key_erase() {
        use smoozy_security_and_licence::erasure::EraseMarks;
        let path = scratch("free-software-fresh");
        let fresh = LocalRecord::open_at(&path).unwrap();
        assert!(crate::record::key_rows::KeyRowMarks(fresh.connection()).marks().unwrap().is_empty());
        crate::record::key_rows::with_legacy_licence_erases_owed(fresh.connection(), || panic!("a fresh record reached the store"));

        // The old put could complete before its reference was written.
        let path = a_paid_record("free-software-orphan");
        rusqlite::Connection::open(&path).unwrap().execute("DELETE FROM licence_record", []).unwrap();
        let orphan = LocalRecord::open_at(&path).unwrap();
        assert_eq!(orphan.connection().query_row("SELECT subject FROM owed_erase WHERE kind = 'licence key'", [], |row| row.get::<_, String>(0)).unwrap(), crate::record::key_rows::LEGACY_LICENCE_KEY_SUBJECT);
    }

    #[test]
    fn free_software_erases_the_legacy_key_once_and_retries_only_unconfirmed_erases() {
        use smoozy_security_and_licence::{held::*, plaintext::Plaintext, secret_store::*};
        struct Store { held: std::cell::Cell<bool>, erases: std::cell::Cell<usize>, refuses: std::cell::Cell<bool> }
        impl SecretStore for Store {
            fn put(&self, _: &HeldSecret, _: Plaintext, _: std::time::Duration) -> Result<(), StoreFailure> { unreachable!() }
            fn get(&self, _: &HeldSecret, _: std::time::Duration) -> Result<Plaintext, GetFailure> { unreachable!() }
            fn erase(&self, _: &HeldSecret, _: std::time::Duration) -> Result<(), StoreFailure> { panic!("the retired item is not an ordinary kind") }
            fn erase_legacy_licence_key(&self, subject: &Subject, deadline: std::time::Duration) -> Result<(), StoreFailure> {
                assert_eq!(subject.as_str(), "licence-record");
                assert_eq!(deadline, std::time::Duration::from_secs(10));
                self.erases.set(self.erases.get()+1);
                if self.refuses.get() { return Err(StoreFailure::LockedOrRefused); }
                self.held.set(false); Ok(())
            }
            fn probe(&self, _: std::time::Duration) -> ProbeOutcome { unreachable!() }
            fn enumerate(&self, _: std::time::Duration) -> Result<Vec<HeldSecret>, StoreFailure> { unreachable!() }
        }
        let path = a_paid_record("free-software-erase");
        let store = Store { held: true.into(), erases: 0.into(), refuses: true.into() };
        let record = LocalRecord::open_at(&path).unwrap();
        crate::record::key_rows::retry_the_legacy_licence_erases(record.connection(), &store);
        assert_eq!(store.erases.get(), 1);
        assert!(store.held.get());
        store.refuses.set(false);
        crate::record::key_rows::retry_the_legacy_licence_erases(record.connection(), &store);
        assert!(!store.held.get());
        assert_eq!(store.erases.get(), 2);
        drop(record);
        let record = LocalRecord::open_at(&path).unwrap();
        crate::record::key_rows::retry_the_legacy_licence_erases(record.connection(), &store);
        assert_eq!(store.erases.get(), 2, "a confirmed erase was repeated at the next launch");
        crate::record::key_rows::with_legacy_licence_erases_owed(record.connection(), || panic!("a confirmed erase opened the store again"));
        assert_eq!(record.connection().query_row("SELECT count(*) FROM owed_erase WHERE kind = 'bot token'", [], |r| r.get::<_, i64>(0)).unwrap(), 1);
    }

    /// **The migration list ends at the version the binary knows**, so a
    /// migration added without the number moving fails here rather than at a
    /// user's record. `schema::BINARY_KNOWS`'s own doc comment states the rule;
    /// this is the check.
    #[test]
    fn the_last_migration_produces_the_version_this_binary_knows() {
        let highest = ALL.iter().map(|one| one.produces).max().expect("migrations");
        assert_eq!(highest, BINARY_KNOWS);
        let mut expected = 0;
        for migration in ALL {
            expected += 1;
            assert_eq!(
                migration.produces, expected,
                "the migrations are one per version, in order, with no gap"
            );
        }
    }

    /// The five tables version 26 creates, named once.
    const THE_TWO_DOWNLOAD_STORES_TABLES: [&str; 6] = [
        "download_arrangement",
        "download_arrangement_format",
        "download_arrangement_entry",
        "download_request",
        "download_request_entry",
        "download_request_refusal",
    ];
}

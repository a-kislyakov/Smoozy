//! **The warnings the user has switched off, and the one control that brings
//! them all back** — `Desktop app and delivery` §15 G1 to G10, and
//! `State, identity and recovery` §10.3, which supplies the one table.
//!
//! **The rule is §15's; the store is this document's**, which is the split
//! every other row module here draws. **A2 holds and it is the whole of G1: no
//! module has a durable store of its own**, there is one local record, and this
//! is the only place in it a suppressed warning can be written. A module that
//! wanted to remember its own in its own way has nowhere to put it.
//!
//! # The row is one column, and that is a ruling rather than an omission
//!
//! **G4: *"A row is the module and the warning identity, and nothing else. No
//! wording, no timestamp, no count, no per-subject qualifier — a row carrying
//! anything more would be a suppression two rows could disagree about."***
//! `State, identity and recovery` §10.3 says the same from the other side.
//!
//! **Both halves are kept by the one column.** The module is carried by the
//! identity's own declared spelling — `File Browser` M3 declares
//! `file-browser.delete`, `Rendering` W4a declares `rendering.missing-assets` —
//! and the binding between a module and its identities lives on §4 A7's
//! declaration route at [`crate::presentation::declarations`], which is the one
//! place it can be checked. **A module column beside it would be the same fact
//! in two places**, with nothing reading it: G7's restore is one delete over
//! the whole table and walks no modules, and a suppression check is asked by
//! the owning module with its own identity already in hand.
//!
//! **The `suppressed_at` moment [`crate::record::schema::VERSION_1`] made is
//! gone at [`crate::record::schema::VERSION_15`].** G4 forbids a timestamp by
//! name, and G9's *no age, no cap, no sweep* is what makes it hazardous rather
//! than harmless: a stored moment is the exact shape of the sweep that clause
//! forbids, sitting in the table waiting for somebody to write it.
//!
//! # What must never be added here
//!
//! - **No age, no cap and no sweep** (G9). Suppressions are user-held
//!   (`State, identity and recovery` §11.1): they end when the user removes
//!   them and never otherwise, and **nothing restores a warning because time
//!   passed**. There is no clock in this file and
//!   [`tests::nothing_here_reads_a_clock_or_ages_a_row`] is what says so.
//! - **No second delete.** [`THE_ONE_DELETE`] is the whole of G7 and it carries
//!   no `WHERE`: the control does not ask each module to restore its own, does
//!   not enumerate warnings, and does not need to know which modules exist. **A
//!   warning belonging to a module no longer in the application is restored by
//!   the same act as every other.**
//! - **No tombstone** (G10, and `State, identity and recovery` §10.3): nothing
//!   is being undone by a later reading, so a restored warning stays restored
//!   because there is no row left to find.
//! - **Nothing about a unit outcome** (§14 Q6). What is said once is the
//!   reason, never the tally, and there is nowhere in a one-column table an
//!   outcome record or a unit could travel.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` W4a.
//!
//! **`File Browser` M3 and `State, identity and recovery`'s own markers are
//! written out with their document at every site below and are deliberately not
//! listed here** — this plan defines an M3 of its own and clauses of the same
//! short names, so a bare one would resolve to a real and wrong clause.

use super::RecordFailure;

/// The one table. Named here so this file, the schema and the guard tests read
/// from one place.
pub const TABLE: &str = "suppressed_warning";

/// **G7: the control is one delete over the table and it walks no modules.**
///
/// **It is a constant so that *there is one* is a thing a test can ask.** The
/// clause's teeth are in what the statement does not carry: no `WHERE`, no
/// module, no identity, no age. G8's partition unit is the whole table, so this
/// is the whole act — it restores all of them or it leaves the table as it was.
pub const THE_ONE_DELETE: &str = "DELETE FROM suppressed_warning";

/// **Whether the user has switched this warning off.**
///
/// Asked by the owning module with its own identity already in hand, which is
/// why nothing here enumerates and nothing here takes a module. An identity the
/// table does not hold is not switched off, which is the ordinary state of
/// every warning in a fresh installation.
pub fn it_is_switched_off(
    connection: &rusqlite::Connection,
    warning_identity: &str,
) -> Result<bool, RecordFailure> {
    Ok(connection.query_row(
        "SELECT count(*) > 0 FROM suppressed_warning WHERE warning_identity = ?1",
        [warning_identity],
        |row| row.get(0),
    )?)
}

/// **The user switched this warning off** — the one write there is.
///
/// **Switching the same warning off twice is one suppression**, not two: the
/// identity is the primary key and a second write is the same row. G4's *a row
/// carrying anything more would be a suppression two rows could disagree about*
/// is kept by there being nothing in the row for two writes to differ over.
///
/// **The caller checks the declaration first** — G5, and §4 A7's hosting rule
/// that a module with no declaration answers *no* to everything.
/// [`crate::surface::said_once::a_warning_was_switched_off`] is where the two
/// are put together, so the check has one home rather than one per caller.
pub fn the_user_switched_it_off(
    connection: &rusqlite::Connection,
    warning_identity: &str,
) -> Result<(), RecordFailure> {
    connection.execute(
        "INSERT OR IGNORE INTO suppressed_warning (warning_identity) VALUES (?1)",
        [warning_identity],
    )?;
    Ok(())
}

/// **How many warnings are switched off.**
///
/// Design's own rule for the control: *"The restore control says how many are
/// switched off, and says there is no way back one at a time … without it the
/// user cannot tell a control that restores nine warnings from one that
/// restores none."* **It is a reading of what the control will do and never a
/// stored number** — a count in a column would be the fact in two places, and
/// G4 forbids a count in the row besides.
pub fn how_many_are_switched_off(
    connection: &rusqlite::Connection,
) -> Result<u32, RecordFailure> {
    let counted: i64 =
        connection.query_row("SELECT count(*) FROM suppressed_warning", [], |row| {
            row.get(0)
        })?;
    Ok(u32::try_from(counted).unwrap_or(u32::MAX))
}

/// **The control was pressed: every warning the user ever switched off comes
/// back** — G7 and G8.
///
/// **Atomic entire** (G8, and `State, identity and recovery` §9.2 test 1): the
/// smallest thing the spec promises an outcome about is *every warning the user
/// has switched off, all at once*, so **a partly-restored table is never a
/// state the user can be left in**. The count and the delete are one
/// transaction for that reason — not because one statement could half-run, but
/// because the number the user is told is then the number that was restored.
///
/// **It walks no modules** (G7). Nothing is asked of a module, nothing is
/// enumerated, and a row belonging to a module that is no longer in the
/// application goes with the rest.
///
/// The answer is how many were restored, for the control's own line.
pub fn every_switched_off_warning_is_restored(
    connection: &rusqlite::Connection,
) -> Result<u32, RecordFailure> {
    connection.execute_batch("BEGIN IMMEDIATE")?;
    let done = (|| -> Result<u32, RecordFailure> {
        let were = how_many_are_switched_off(connection)?;
        connection.execute_batch(THE_ONE_DELETE)?;
        Ok(were)
    })();
    match done {
        Ok(were) => {
            connection.execute_batch("COMMIT")?;
            Ok(were)
        }
        Err(failure) => {
            // The whole table or none of it. A rollback that itself fails
            // leaves the original failure standing, which is the one the caller
            // has to act on — the same shape a migration takes at
            // [`super::migrations`].
            let _ = connection.execute_batch("ROLLBACK");
            Err(failure)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;

    /// A record of its own, in a folder of its own, the way every other row
    /// module here makes one.
    fn a_record(name: &str) -> (std::path::PathBuf, LocalRecord) {
        let mut folder = std::env::temp_dir();
        folder.push(format!(
            "smoozy-suppressed-warning-test-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder to put the record in");
        let record =
            LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record opens");
        (folder, record)
    }

    fn done(folder: std::path::PathBuf) {
        let _ = std::fs::remove_dir_all(folder);
    }

    /// The columns of the table as the record actually makes it.
    fn the_columns(connection: &rusqlite::Connection) -> Vec<String> {
        let mut asking = connection
            .prepare(&format!("PRAGMA table_info({TABLE})"))
            .expect("the table is there");
        let rows = asking
            .query_map([], |row| row.get::<_, String>(1))
            .expect("the pragma answers");
        rows.map(|row| row.expect("a column name")).collect()
    }

    /// **G4: a row is the module and the warning identity, and nothing else.**
    ///
    /// **This is the assertion the ruling of this piece rests on**, and it
    /// catches both halves of what the table as built got wrong: the
    /// `suppressed_at` moment G4 forbids by name and G9 makes hazardous, and
    /// any module column somebody adds later on the reading that *the module
    /// and the warning identity* means two fields. **The module is in the
    /// identity's own spelling** and the binding is checked on the declaration
    /// route, which is the one place it can be.
    ///
    /// **§14 Q6 rides on the same line**: there is nowhere in a one-column
    /// table for a unit outcome or a tally to travel.
    #[test]
    fn the_row_is_the_warning_identity_and_nothing_else() {
        let (folder, record) = a_record("one-column");
        assert_eq!(
            the_columns(record.connection()),
            vec!["warning_identity".to_string()],
            "the suppression row is not one column. **G4: *no wording, no \
             timestamp, no count, no per-subject qualifier — a row carrying \
             anything more would be a suppression two rows could disagree \
             about*.** The module lives in the identity's own declared spelling \
             and its binding is checked on §4 A7's declaration route; a second \
             column would be the same fact in two places, with nothing reading \
             it"
        );
        drop(record);
        done(folder);
    }

    /// **A hundred switchings-off of one warning are one row.**
    #[test]
    fn switching_the_same_warning_off_twice_is_one_suppression() {
        let (folder, record) = a_record("twice");
        for _ in 0..100 {
            the_user_switched_it_off(record.connection(), "file-browser.delete")
                .expect("the row goes down");
        }
        assert_eq!(
            how_many_are_switched_off(record.connection()).expect("the count answers"),
            1
        );
        assert!(it_is_switched_off(record.connection(), "file-browser.delete")
            .expect("the read answers"));
        drop(record);
        done(folder);
    }

    /// **A warning nobody has switched off is not switched off**, and that is
    /// the ordinary state of a fresh installation rather than a gap.
    #[test]
    fn a_warning_nobody_switched_off_reads_as_on() {
        let (folder, record) = a_record("fresh");
        assert!(!it_is_switched_off(record.connection(), "file-browser.delete")
            .expect("the read answers"));
        assert_eq!(
            how_many_are_switched_off(record.connection()).expect("the count answers"),
            0
        );
        drop(record);
        done(folder);
    }

    /// **G7 and G8: the control is one delete over the table, and it takes
    /// every row whatever module wrote it.**
    ///
    /// The second identity is a module that is not in this application at all,
    /// which is G7's own case: *"A warning belonging to a module that is no
    /// longer in the application is restored by the same act as every other."*
    #[test]
    fn the_control_restores_every_warning_and_walks_no_modules() {
        let (folder, record) = a_record("restore");
        for identity in [
            "file-browser.delete",
            "rendering.missing-assets",
            "a-module-nobody-ships-any-more.something",
        ] {
            the_user_switched_it_off(record.connection(), identity).expect("the row goes down");
        }
        assert_eq!(
            every_switched_off_warning_is_restored(record.connection())
                .expect("the delete goes down"),
            3
        );
        assert_eq!(
            how_many_are_switched_off(record.connection()).expect("the count answers"),
            0
        );
        drop(record);
        done(folder);
    }

    /// **Pressing the control with nothing switched off restores none and is
    /// not a failure.**
    ///
    /// Design: *"Where none is switched off, the control is there and reads
    /// *none switched off* rather than going away — a control that appears only
    /// when it has work is a control nobody knows exists."*
    #[test]
    fn the_control_pressed_with_nothing_switched_off_restores_none() {
        let (folder, record) = a_record("none");
        assert_eq!(
            every_switched_off_warning_is_restored(record.connection())
                .expect("the delete goes down"),
            0
        );
        drop(record);
        done(folder);
    }

    /// **G7's teeth: the one delete carries no `WHERE` and names no module.**
    ///
    /// A delete narrowed to a module, or to an identity, or to an age, would be
    /// a different act wearing the same name — and the failure would be G2's
    /// invisible one, met by a user who presses the control and finds one
    /// warning has not come back.
    #[test]
    fn the_one_delete_is_over_the_whole_table() {
        let statement = THE_ONE_DELETE.to_uppercase();
        assert!(!statement.contains("WHERE"), "{THE_ONE_DELETE} is narrowed");
        assert!(!statement.contains("LIMIT"), "{THE_ONE_DELETE} is narrowed");
        assert!(
            !statement.contains("WARNING_IDENTITY"),
            "{THE_ONE_DELETE} names a row"
        );
        assert_eq!(THE_ONE_DELETE, format!("DELETE FROM {TABLE}"));
    }

    /// **G9: no age, no cap, no sweep, and nothing restores a warning because
    /// time passed.**
    ///
    /// Held by reading this file's own working half rather than by remembering
    /// it. **A clock here is the first line of the sweep the clause forbids**,
    /// and the `suppressed_at` column that was in the table until version 15 is
    /// what the clause was written about.
    #[test]
    fn nothing_here_reads_a_clock_or_ages_a_row() {
        let source = include_str!("suppressed_warnings.rs");
        let working = source.split("#[cfg(test)]").next().unwrap_or_default();
        let working = past_the_comments(working);
        for forbidden in [
            "SystemTime",
            "Instant",
            "Duration",
            "suppressed_at",
            "elapsed",
            "now()",
            "ORDER BY",
            "LIMIT",
        ] {
            assert!(
                !working.contains(forbidden),
                "src/record/suppressed_warnings.rs names {forbidden}. **G9: \
                 suppressions are user-held — no age, no cap, no sweep — and \
                 nothing restores a warning because time passed.** The moment \
                 version 1 stored is gone at version 15 for exactly this reason"
            );
        }
    }

    /// The file's text with its comment lines read past, so a prohibition can
    /// be written beside the code that keeps it. The same shape
    /// `crate::presentation` uses, and for the same reason.
    fn past_the_comments(text: &str) -> String {
        text.lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with('*') || line.starts_with("/*"))
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// **Nothing else in this crate writes the suppression table.**
    ///
    /// **This is G1 and G2 together, held over the crate rather than over this
    /// file.** G2's failure is invisible — *"nobody finds out until a user
    /// presses the control and one warning does not come back"* — and a second
    /// writer somewhere else in this crate is one of the two ways that happens.
    /// The other is a module remembering its own elsewhere, which the test
    /// below is for.
    #[test]
    fn nothing_else_in_this_crate_writes_a_suppression() {
        let mut writers = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
            &mut writers,
        );
        assert_eq!(
            writers,
            vec!["suppressed_warnings.rs".to_string()],
            "something besides this module writes the suppression table. \
             **G1: nothing else may hold a suppression, and it is kept \
             structurally**; **G2: the failure is invisible** — nobody finds \
             out until a user presses the control and one warning does not \
             come back"
        );
    }

    /// Every `.rs` file under `src` whose working half writes the table.
    fn walk(folder: &std::path::Path, found: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(folder) else {
            return;
        };
        let mut paths: Vec<_> = entries
            .map(|entry| entry.expect("the entry is readable").path())
            .collect();
        paths.sort();
        for path in paths {
            if path.is_dir() {
                walk(&path, found);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("the file is readable");
            let working = text.split("#[cfg(test)]").next().unwrap_or_default();
            let writes = [
                "INSERT INTO suppressed_warning",
                "INSERT OR IGNORE INTO suppressed_warning",
                "UPDATE suppressed_warning",
                "DELETE FROM suppressed_warning",
            ]
            .iter()
            .any(|statement| working.contains(statement));
            if writes {
                found.push(
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .expect("the file has a name")
                        .to_string(),
                );
            }
        }
    }

    /// **G1, checked across the whole application rather than assumed.**
    ///
    /// A2 gives no module a durable store of its own and this table is the only
    /// row of `State, identity and recovery` §1.1's inventory that holds a
    /// suppressed warning — so **a module that wanted to remember its own has
    /// nowhere to write it.** That is a claim about every crate in the
    /// workspace and not only about this one, and the brief for this piece asks
    /// for it to be checked rather than taken on trust.
    ///
    /// **What the check covers, said exactly.** It reads every `.rs` file under
    /// `App/` and fails on a table creation naming a suppression anywhere but
    /// in this crate's schema. **It does not and cannot cover a module
    /// remembering a suppression under some other name**, in a settings value
    /// or a file of its own; what covers that is A2 itself, which gives such a
    /// module no store to write in.
    #[test]
    fn no_other_crate_in_the_application_makes_a_suppression_store() {
        let app = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("this crate sits inside App/")
            .to_path_buf();
        let mut makers = Vec::new();
        walk_for_stores(&app, &mut makers);
        assert_eq!(
            makers,
            vec!["schema.rs".to_string()],
            "a second suppression store exists in this workspace. **G1: there \
             is one local record and this document builds it; the suppression \
             table is the only row of the inventory that holds a suppressed \
             warning, so a module that wanted to remember its own has nowhere \
             to put one.** **A2 is what makes that structural**, and the \
             failure it prevents is G2's invisible one"
        );
    }

    /// Every `.rs` file under `App/` whose working half creates a table naming
    /// a suppression.
    ///
    /// **Every folder whose name starts with `target` is skipped** — `target`,
    /// `target-a` and any other build folder: they hold built copies of these
    /// same sources, and other tests leave stray links in them. **A link is
    /// never followed**, to a folder or to a file, so nothing outside `App/` is
    /// ever read; a source the workspace builds is a real file under `App/`
    /// and is reached without one.
    fn walk_for_stores(folder: &std::path::Path, found: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(folder) else {
            return;
        };
        let mut paths: Vec<_> = entries
            .map(|entry| entry.expect("the entry is readable").path())
            .collect();
        paths.sort();
        for path in paths {
            let Ok(what) = std::fs::symlink_metadata(&path) else {
                continue;
            };
            if what.file_type().is_symlink() {
                continue;
            }
            if what.is_dir() {
                if path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|name| name.starts_with("target"))
                {
                    continue;
                }
                walk_for_stores(&path, found);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("the file is readable");
            let working = text.split("#[cfg(test)]").next().unwrap_or_default();
            let makes = working.split("CREATE TABLE").skip(1).any(|after| {
                let name: String = after
                    .chars()
                    .skip_while(|c| c.is_whitespace())
                    .take_while(|c| !c.is_whitespace() && *c != '(')
                    .collect();
                name.to_lowercase().contains("suppress")
            });
            if makes {
                found.push(
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .expect("the file has a name")
                        .to_string(),
                );
            }
        }
    }

    /// **The walk above reads `App/`'s own sources and nothing else** — proved
    /// in a throwaway folder rather than on the real tree, where the stray
    /// links come and go with other tests and the old walk failed at random.
    ///
    /// The folder holds one real store maker, `schema.rs`, and three strays a
    /// walk must not read: a copy inside `target-x/`, and — where the system
    /// makes links without asking — a link to a folder outside holding a copy,
    /// and a link to that copy itself. **The walk finds `schema.rs` alone.**
    /// Before the fix it read `target-x/` and followed both links.
    #[test]
    fn the_walk_skips_every_build_folder_and_follows_no_link_out() {
        let root = std::env::temp_dir().join(format!(
            "smoozy-suppression-walk-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        let app = root.join("App");
        let outside = root.join("somewhere-else");
        let maker = "pub const S: &str = \"CREATE TABLE suppressed_warning (k TEXT)\";\n";
        std::fs::create_dir_all(app.join("A crate/src")).unwrap();
        std::fs::write(app.join("A crate/src/schema.rs"), maker).unwrap();
        std::fs::create_dir_all(app.join("target-x/debug/deps")).unwrap();
        std::fs::write(app.join("target-x/debug/deps/stray_copy.rs"), maker).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(outside.join("linked_copy.rs"), maker).unwrap();
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&outside, app.join("A crate/a-link-out")).unwrap();
            std::os::unix::fs::symlink(
                outside.join("linked_copy.rs"),
                app.join("A crate/src/a_linked_file.rs"),
            )
            .unwrap();
        }
        let mut makers = Vec::new();
        walk_for_stores(&app, &mut makers);
        let _ = std::fs::remove_dir_all(&root);
        assert_eq!(
            makers,
            vec!["schema.rs".to_string()],
            "the walk read a build folder's stray copy or followed a link out \
             of App/, so the check above fails on whatever other tests left \
             lying in App/target-a"
        );
    }

    /// The table's name is the one the schema makes.
    #[test]
    fn the_table_is_the_one_the_schema_makes() {
        assert!(
            crate::record::schema::VERSION_1
                .iter()
                .any(|statement| statement.contains(&format!("CREATE TABLE {TABLE}"))),
            "the migration no longer makes the table this file writes"
        );
        assert!(
            crate::record::schema::TABLES_A_MIGRATION_NEVER_EMPTIES.contains(&TABLE),
            "**L11 and G6: no migration ever removes a suppression row.** The \
             table has left the list the migration check reads"
        );
    }

    /// **G6 and L11, demonstrated against a table with rows in it rather than
    /// argued about an empty one.**
    ///
    /// The record holds no suppression anywhere yet — nothing in Smoozy draws a
    /// warning with a switch-off on it — so a migration that dropped every row
    /// would pass every other test in this crate. **This one puts two rows in a
    /// version-14 record, runs the version-15 migration over them, and asks
    /// what came out.** A column goes; both rows stay; the count the protected
    /// check reads does not move.
    #[test]
    fn the_migration_carries_every_suppression_across() {
        let mut folder = std::env::temp_dir();
        folder.push(format!(
            "smoozy-suppression-migration-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder to put the record in");
        let path = folder.join("local-record.sqlite3");
        let connection = rusqlite::Connection::open(&path).expect("the record opens");

        // A record as a binary before this piece would have left it.
        for migration in crate::record::migrations::ALL
            .iter()
            .filter(|migration| migration.produces <= 14)
        {
            crate::record::migrations::run_one_for_tests(&connection, migration)
                .expect("the migration runs");
        }
        connection
            .execute_batch(
                "INSERT INTO suppressed_warning VALUES ('file-browser.delete', 1725840000);
                 INSERT INTO suppressed_warning VALUES ('rendering.missing-assets', 1725840001)",
            )
            .expect("the two rows go down");

        let before = crate::record::migrations::protected_counts(&connection)
            .expect("the protected counts read");

        let fifteen = crate::record::migrations::ALL
            .iter()
            .find(|migration| migration.produces == 15)
            .expect("version 15 is carried");
        crate::record::migrations::run_one_for_tests(&connection, fifteen)
            .expect("the migration runs");

        assert_eq!(
            the_columns(&connection),
            vec!["warning_identity".to_string()],
            "version 15 did not take the moment off the row"
        );
        assert_eq!(
            how_many_are_switched_off(&connection).expect("the count answers"),
            2,
            "**G6 and L11: a migration never removes a suppression row.** Row \
             290's failure is invisible until a user presses the control and \
             one warning does not come back, and a migration is the one place \
             it could be introduced silently"
        );
        crate::record::migrations::nothing_protected_was_lost(&connection, &before)
            .expect("nothing protected was lost");
        assert!(it_is_switched_off(&connection, "file-browser.delete").expect("the read answers"));
        assert!(
            it_is_switched_off(&connection, "rendering.missing-assets").expect("the read answers")
        );

        drop(connection);
        done(folder);
    }
}

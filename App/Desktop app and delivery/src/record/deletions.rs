//! **Deleting a history entry, which is two writes and one unit** —
//! `Queue & History` X1, and the clearing acts `Queue & History` X6 and
//! `Queue & History` X6a over it.
//!
//! # Why this is a file of its own rather than a method on the store
//!
//! **`src/record/job_records.rs`'s `remove` is half of a deletion and says so.**
//! It is the removal `State, identity and recovery` V1 step 2 makes when a
//! machine says it never took the job, and that removal writes no tombstone
//! because nothing was deleted. **`Queue & History` X1's deletion is the record
//! *and* the tombstone in one atomic local act**, and a record gone with no
//! tombstone is the one state that clause forbids: the entry would come back at
//! the next fill-in.
//!
//! **So the two halves are joined here, in one transaction, and the store below
//! is left as it is.** Step 10b piece 6 landed the store and named this as what
//! piece 8 owed it.
//!
//! # What a deletion does not reach
//!
//! **Nothing on a machine, and nothing the user owns.** `Queue & History` X2's
//! erase of Smoozy's own record on the machine is owed at the next connection
//! and is carried as a debt on the tombstone itself — `State, identity and
//! recovery` §10.1's `machine_side_copy`, written *still owed* here and never
//! anything else. The renders on the machine and whatever was downloaded stay,
//! which is what the box the user read said.
//!
//! # How a clause marker is written in this file
//!
//! **This file keeps no header list** and writes every other document's name
//! out at its own site, which is `src/servers/a_period.rs`'s shape and the
//! store's beside it. A bare marker below means a clause of this crate's own
//! plan, `Preparation/Architecture/Desktop app and delivery.md`.

use smoozy_queue_history::deletion::ADeletion;
use smoozy_state::tombstones::MachineSideCopy;

use super::job_records::JobRecords;
use super::{nanoseconds, RecordFailure};

/// **The two spellings the machine-side debt is written and read as**, named
/// once. `JobRecords::tombstones` reads them and this writes them, so a test
/// below writes one and reads it back rather than either side trusting the
/// other.
const STILL_OWED: &str = "still owed";
const REMOVED: &str = "removed";

/// The performer, over one open local record.
pub struct TheDeletions<'a> {
    connection: &'a rusqlite::Connection,
}

impl<'a> TheDeletions<'a> {
    pub fn over(connection: &'a rusqlite::Connection) -> Self {
        TheDeletions { connection }
    }

    /// **One deletion, whole or not at all** — `Queue & History` X1, and
    /// `State, identity and recovery` §9.3's rule that the two writes are one
    /// unit.
    ///
    /// **The deletion value itself is `Queue & History`'s**, composed by
    /// `smoozy_queue_history::deletion::ADeletion::of`. Nothing here decides
    /// what a tombstone holds or which entries a clearing covers.
    pub fn perform(&self, deletion: &ADeletion) -> Result<(), RecordFailure> {
        debug_assert!(deletion.is_one_unit());
        let at = nanoseconds(deletion.write_the_tombstone.deleted_at.taken_at()).ok_or(
            RecordFailure::AMomentTheRecordCannotHold {
                column: "history_tombstone.deleted_at",
                reported: "the deletion moment is outside the record's own spelling".to_string(),
            },
        )?;
        let copy = match deletion.write_the_tombstone.machine_side_copy {
            MachineSideCopy::StillOwed => STILL_OWED,
            MachineSideCopy::Removed => REMOVED,
        };
        self.connection.execute("BEGIN IMMEDIATE", [])?;
        let done = (|| -> Result<(), RecordFailure> {
            // **The kept submission material, where it owes nothing**, in
            // this same saved change — read before the row goes, because
            // whether the job was accepted is the row's own standing.
            super::submission_material::pruned_with_its_entry(
                self.connection,
                &deletion.remove_the_record_of,
            )?;
            // **The row and its sets**, through the store's own removal,
            // which takes the transaction already standing here rather than
            // opening a second one.
            JobRecords::over(self.connection).remove(&deletion.remove_the_record_of)?;
            self.connection.execute(
                "INSERT OR REPLACE INTO history_tombstone (job, deleted_at, machine_side_copy)
                 VALUES (?1, ?2, ?3)",
                rusqlite::params![deletion.write_the_tombstone.job.as_text(), at, copy],
            )?;
            Ok(())
        })();
        match done {
            Ok(()) => {
                self.connection.execute("COMMIT", [])?;
                Ok(())
            }
            Err(failure) => {
                let _ = self.connection.execute("ROLLBACK", []);
                Err(failure)
            }
        }
    }

    /// **A clearing act, which is the per-entry deletion over every entry the
    /// scope covers** — `Queue & History` X6 and `Queue & History` X6a. **One
    /// user act, partial across entries and atomic within each**: an
    /// interrupted clear leaves the
    /// remaining entries standing and drawn, and the act is the user's to take
    /// again. That is that document's own sentence, so this stops at the first
    /// write that would not go through and reports it, leaving what was already
    /// deleted deleted.
    pub fn perform_all(&self, deletions: &[ADeletion]) -> Result<(), RecordFailure> {
        for deletion in deletions {
            self.perform(deletion)?;
        }
        Ok(())
    }

    /// **Every deleted job whose machine-side copy is still owed** —
    /// `State, identity and recovery` H5's debt, read at a connection. The
    /// 38's piece 16.
    pub fn the_jobs_still_owing_their_erase(
        &self,
    ) -> Result<Vec<smoozy_state::identities::Identity>, RecordFailure> {
        use smoozy_state::identities::{Identity, IdentityKind};
        let mut statement = self
            .connection
            .prepare("SELECT job FROM history_tombstone WHERE machine_side_copy = ?1 ORDER BY job")?;
        let jobs = statement
            .query_map([STILL_OWED], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(jobs
            .iter()
            .filter_map(|job| Identity::read_back(IdentityKind::Job, job))
            .collect())
    }

    /// **The debt paid: the flag set, and only this** — `Queue & History` X3,
    /// set only when every unit of the erase is done. The tombstone itself
    /// stays (`State, identity and recovery` H4), so the entry never comes
    /// back.
    pub fn the_machine_side_copy_was_removed(
        &self,
        job: &smoozy_state::identities::Identity,
    ) -> Result<(), RecordFailure> {
        self.connection.execute(
            "UPDATE history_tombstone SET machine_side_copy = ?1 WHERE job = ?2",
            rusqlite::params![REMOVED, job.as_text()],
        )?;
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::time::{Duration, SystemTime};

    use smoozy_queue_history::deletion::{a_clearing, AnEntryTheListDraws, ClearingScope};
    use smoozy_queue_history::record::{
        AnswerOrigin, Hue, LocalJobRecord, Standing, TheBlenderTheEntryKeeps, TheDeviceAsk,
        TheServerAsItStood, WhatTheNoteCarried,
    };
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_state::observed::{Clock, Reading};
    use smoozy_state::tombstones::HistoryTombstones;

    use super::*;

    /// A record store with every migration run, in memory.
    pub(crate) fn a_record_store() -> rusqlite::Connection {
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

    /// **One record whose render start has been learned**, which is the only
    /// kind either act below covers — `Queue & History` C9.
    pub(crate) fn a_record_in_history(server: &Identity, label: &str) -> LocalJobRecord {
        LocalJobRecord {
            job: Identity::issue(IdentityKind::Job).unwrap(),
            group: None,
            server: TheServerAsItStood {
                local_server: server.clone(),
                label: Some("the big box".to_string()),
                hue: Some(Hue(210)),
            },
            submitted_at: Some(Reading::taken(
                (),
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_754_600_000),
                Clock::ThisComputer,
            )),
            label: Some(label.to_string()),
            blend_path_on_the_machine: None,
            scene_name: None,
            submitted_with_no_scene_named: true,
            resolution_percentage_typed: None,
            picture_size_from_the_read: None,
            main_output_from_the_read: None,
            blender: TheBlenderTheEntryKeeps {
                pick: smoozy_render_settings::payload::VersionPick {
                    build_path: "/b".to_string(),
                    version: "4.5.12".to_string(),
                },
                origin: AnswerOrigin::ResolvedForThem,
            },
            device_asked_for: TheDeviceAsk::TheFilesOwnUnread,
            the_files_values_beside_the_overrides: None,
            dependency_check: None,
            carries_on_from: None,
            disturbances: vec![],
            attempts_where_the_file_changed: vec![],
            the_frames: smoozy_queue_history::record::TheFramesTheReadFound::default(),
            standing: Standing::Confirmed,
            the_render_start_has_been_learned: true,
            what_the_note_carried: WhatTheNoteCarried::default(),
            the_job_output_folder: Default::default(),
            the_driver_was_below_the_bar: None,
        }
    }

    fn a_moment() -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_754_700_000)
    }

    /// **`Queue & History` X1: the record goes and the tombstone arrives, in
    /// one act.**
    #[test]
    fn a_deletion_removes_the_record_and_writes_the_tombstone_together() {
        let connection = a_record_store();
        let store = JobRecords::over(&connection);
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let record = a_record_in_history(&server, "Kitchen");
        store.write_the_provisional_record(&record).unwrap();
        assert_eq!(store.in_history().unwrap().entries.len(), 1);

        let deletion = ADeletion::of(&record, a_moment());
        TheDeletions::over(&connection).perform(&deletion).unwrap();

        assert!(store.read(&record.job).unwrap().is_none());
        assert!(store.in_history().unwrap().entries.is_empty());
        // **The debt written here is the one the reader reads back**, which is
        // what keeps the two spellings from drifting apart.
        let tombstones = store.tombstones().unwrap();
        let held = tombstones
            .tombstone_for(&record.job)
            .expect("the tombstone is there");
        assert_eq!(held.machine_side_copy, MachineSideCopy::StillOwed);
        assert_eq!(held.deleted_at.taken_at(), a_moment());
        assert!(!tombstones.may_be_filled_in(&record.job));
    }

    /// **`Queue & History` X6 and `Queue & History` X6a: a clearing act is the
    /// per-entry deletion over its scope**,
    /// and every entry outside the scope is left standing.
    #[test]
    fn a_clearing_deletes_every_entry_its_scope_covers_and_leaves_the_rest() {
        let connection = a_record_store();
        let store = JobRecords::over(&connection);
        let one = Identity::issue(IdentityKind::LocalServer).unwrap();
        let two = Identity::issue(IdentityKind::LocalServer).unwrap();
        let records: Vec<LocalJobRecord> =
            [(&one, "Kitchen"), (&one, "Bedroom"), (&two, "Hallway")]
                .into_iter()
                .map(|(server, label)| {
                    let record = a_record_in_history(server, label);
                    store.write_the_provisional_record(&record).unwrap();
                    record
                })
                .collect();
        let entries: Vec<AnEntryTheListDraws<'_>> = records
            .iter()
            .map(|record| AnEntryTheListDraws {
                record,
                the_render_start_has_been_learned: true,
            })
            .collect();
        let deletions = a_clearing(
            &entries,
            &ClearingScope::OneMachines(one.clone()),
            a_moment(),
        );
        assert_eq!(deletions.len(), 2);
        TheDeletions::over(&connection)
            .perform_all(&deletions)
            .unwrap();
        let left = store.in_history().unwrap().entries;
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].record.server.local_server, two);
        let tombstones = store.tombstones().unwrap();
        assert!(tombstones.tombstone_for(&records[0].job).is_some());
        assert!(tombstones.tombstone_for(&records[1].job).is_some());
        assert!(tombstones.tombstone_for(&records[2].job).is_none());

        // **`Queue & History` X6a: the whole of history takes the one left.**
        let left_entry = [AnEntryTheListDraws {
            record: &records[2],
            the_render_start_has_been_learned: true,
        }];
        let all = a_clearing(&left_entry, &ClearingScope::TheWholeOfHistory, a_moment());
        TheDeletions::over(&connection).perform_all(&all).unwrap();
        assert!(store.in_history().unwrap().entries.is_empty());
    }

    /// One record written with its kept submission material, the material's
    /// debt and done units set by hand. Hands back the record.
    fn a_record_with_kept_material(
        connection: &rusqlite::Connection,
        label: &str,
        standing: Standing,
        debt: bool,
        done: [bool; 3],
    ) -> LocalJobRecord {
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let mut record = a_record_in_history(&server, label);
        record.standing = standing;
        JobRecords::over(connection)
            .write_the_provisional_record(&record)
            .unwrap();
        crate::record::submission_material::Material {
            job: record.job.clone(),
            server,
            installation: Identity::issue(IdentityKind::Installation).unwrap(),
            material_folder: format!("/root/smoozy/jobs/{label}"),
            material: b"a manifest".to_vec(),
            remap_path: format!("/root/smoozy/remap/{label}.json"),
            remap: b"a remap".to_vec(),
            output_folder: None,
            done: [false; 3],
        }
        .keep(connection)
        .unwrap();
        connection
            .execute(
                "UPDATE submission_material SET debt = ?2, material_done = ?3,
                    remap_done = ?4, folder_done = ?5 WHERE job = ?1",
                rusqlite::params![record.job.as_text(), debt, done[0], done[1], done[2]],
            )
            .unwrap();
        let written: String = connection
            .query_row(
                "SELECT standing FROM job_record WHERE job = ?1",
                [record.job.as_text()],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(
            written == "confirmed",
            standing == Standing::Confirmed,
            "the record was written with the standing the test asked for"
        );
        record
    }

    fn the_material_is_kept_for(connection: &rusqlite::Connection, job: &Identity) -> bool {
        connection
            .query_row(
                "SELECT count(*) > 0 FROM submission_material WHERE job = ?1",
                [job.as_text()],
                |row| row.get(0),
            )
            .unwrap()
    }

    /// **The kept material goes in the same saved change as its History
    /// entry, when its job was accepted or its cleanup debt is settled** —
    /// the check of record version 45, so the table does not grow without
    /// end. **Without the prune both rows stay and this fails.**
    #[test]
    fn kept_material_goes_with_its_entry_when_the_job_was_accepted_or_its_debt_is_settled() {
        let connection = a_record_store();
        let store = JobRecords::over(&connection);
        let accepted =
            a_record_with_kept_material(&connection, "Kitchen", Standing::Confirmed, false, [false; 3]);
        let settled =
            a_record_with_kept_material(&connection, "Bedroom", Standing::Provisional, true, [true; 3]);
        for record in [&accepted, &settled] {
            assert!(the_material_is_kept_for(&connection, &record.job));
            TheDeletions::over(&connection)
                .perform(&ADeletion::of(record, a_moment()))
                .unwrap();
            assert!(store.read(&record.job).unwrap().is_none(), "the entry went");
            assert!(store.tombstones().unwrap().tombstone_for(&record.job).is_some());
            assert!(
                !the_material_is_kept_for(&connection, &record.job),
                "{:?}: the material owed nothing and stayed",
                record.label
            );
        }
    }

    /// **A row whose debt is unpaid, or whose job is unconfirmed, is never
    /// removed** — the entry still goes, and the material stays as the only
    /// record of what to clean off the server.
    #[test]
    fn kept_material_stays_when_its_debt_is_unpaid_or_its_job_is_unconfirmed() {
        let connection = a_record_store();
        let store = JobRecords::over(&connection);
        let unconfirmed =
            a_record_with_kept_material(&connection, "Hallway", Standing::Provisional, false, [false; 3]);
        let unpaid_though_accepted =
            a_record_with_kept_material(&connection, "Attic", Standing::Confirmed, true, [true, true, false]);
        let unpaid_and_unconfirmed =
            a_record_with_kept_material(&connection, "Cellar", Standing::Provisional, true, [false; 3]);
        for record in [&unconfirmed, &unpaid_though_accepted, &unpaid_and_unconfirmed] {
            TheDeletions::over(&connection)
                .perform(&ADeletion::of(record, a_moment()))
                .unwrap();
            assert!(store.read(&record.job).unwrap().is_none(), "the entry went");
            assert!(
                the_material_is_kept_for(&connection, &record.job),
                "{:?}: the material was removed while it still owed",
                record.label
            );
        }
    }
}

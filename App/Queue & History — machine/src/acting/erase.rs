//! **§14 X2 and X3 performed — the owed machine-side erase: its enumerated
//! units removed at the next connection, its outcome record in the local
//! record, and the tombstone's flag set only on all units done.**
//!
//! # Why it sits in this half
//!
//! **The machine's own program runs the units, and the desktop writes what
//! they came to**, so both ends read the one enumeration. Moved here from
//! `App/Queue & History/src/acting/erase.rs` by the 38's piece 16, as step 10c
//! moved the other machine halves; that file re-exports this one at the path
//! it always had. **The machine half is [`erase_the_units`]**; the desktop half
//! is [`open_the_record`] and [`write_what_the_machine_did`], and between them
//! the answer carries [`the_units_not_done_as_written`] and is read back by
//! [`what_the_answer_says`].
//!
//! # What it reaches, and the one thing it never touches
//!
//! **X2 enumerates it exactly, and it is the record — never the renders.**
//! [`AUnitOfTheErase::THE_ENUMERATION`] is that list in the order the erase
//! runs it. **It does not touch the job output folder or anything in it**: the
//! rendered frames are files of the user's, and the spec says in as many words
//! that deleting history destroys the record and not the renders. Where the
//! frames go is the user's own act through `File Browser`, or the
//! machine-reaching server removal, or the machine's destruction.
//!
//! # Why the hold record is its own unit
//!
//! **A hold record left with no release outcome written reads as unreleased to
//! `Rented-server runtime` G21's commit check, and G23 then blocks every
//! package commit on that machine.** So the hold record naming the deleted
//! identity as its subject is accounted as its own unit, and its outcome gates
//! the tombstone's flag exactly as the others do.
//!
//! # Why the erase's own command record is excepted
//!
//! **It is the record of the act and is never among the records the act
//! removes.** An erase that removed its own command record would be an act
//! that could not report what it had done.
//!
//! # X3's shape, and why it is a §9.4 record rather than a growing tombstone
//!
//! **The enumeration is durable on the desktop before the first unit is
//! attempted, and each unit's outcome lands as it becomes true** — partial
//! across the material kinds, atomic within each directory removal.
//! **Removal of a unit already absent is recorded *done*, never *failed***,
//! because the erase must be able to run against a machine where earlier units
//! already went.
//!
//! **The tombstone keeps its one flag and grows no enumeration of its own**
//! (`State, identity and recovery` §10.1), and the flag is set only when every
//! unit is done. **The remainder is taken by a new action with a new operation
//! identity, composed from the outcome record's enumeration** (§9.5 — no unit
//! is retried inside its action).
//!
//! # What is unresolved here, recorded rather than rescued
//!
//! **That the widened erase completes as specified has not been observed** —
//! one removal per box, not repeated, streams not included. The plan's §18
//! item 5 names what breaks. **No verification pass, no re-read and no second
//! sweep is built for it**: each would be a rescue for a known hole, and what
//! holds is the outcome record, which says per unit what was done and what was
//! not.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §9.4, §9.5, §10.1.
//! `Rented-server runtime` U22, G21, G23. `Dependency Upload & Remap` U2.
//! `Job files and outputs` §11.
//!
//! **`Rented-server runtime` R1 and `State, identity and recovery` F19 are
//! deliberately not on that list and are written out at every site instead**,
//! because this plan defines a clause of each of those names too.

use std::path::{Path, PathBuf};

use smoozy_state::identities::Identity;
use smoozy_state::outcomes::{Keeps, Recorded, Refused, UnitOutcome, Writing};
use smoozy_state::tombstones::MachineSideCopy;

/// **X2's enumeration, exactly, and the job output folder is not in it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AUnitOfTheErase {
    /// The machine's own history copy.
    TheMachinesHistoryCopy,
    /// The per-job note.
    ThePerJobNote,
    /// The queue job record where one still stands.
    TheQueueJobRecord,
    /// **The hold record naming that identity as its subject** —
    /// `Rented-server runtime` U22's third ending, and `Rented-server
    /// runtime` R1's retention row.
    TheHoldRecordNamingIt,
    /// **The operation and command records** — `Rented-server runtime` R1's
    /// rule, they end with their subject's machine-side record — **the erase's
    /// own command record excepted.**
    TheOperationAndCommandRecords,
    /// The capture records **and both captured streams**.
    TheCaptureRecordsAndBothStreams,
    /// The job record directory entire — manifest, recorded reading, attempt
    /// records, launch material, machine-side derivatives (`Job files and
    /// outputs` §11).
    TheJobRecordDirectory,
    /// The remap material at `remaps/<job identity>`
    /// (`Dependency Upload & Remap` U2).
    TheRemapMaterial,
}

impl AUnitOfTheErase {
    /// **The enumeration, in the order the erase runs it.**
    pub const THE_ENUMERATION: [AUnitOfTheErase; 8] = [
        AUnitOfTheErase::TheMachinesHistoryCopy,
        AUnitOfTheErase::ThePerJobNote,
        AUnitOfTheErase::TheQueueJobRecord,
        AUnitOfTheErase::TheHoldRecordNamingIt,
        AUnitOfTheErase::TheOperationAndCommandRecords,
        AUnitOfTheErase::TheCaptureRecordsAndBothStreams,
        AUnitOfTheErase::TheJobRecordDirectory,
        AUnitOfTheErase::TheRemapMaterial,
    ];

    /// The unit's own name in the §9.4 record.
    pub fn name(self) -> &'static str {
        match self {
            AUnitOfTheErase::TheMachinesHistoryCopy => "the machine's history copy",
            AUnitOfTheErase::ThePerJobNote => "the per-job note",
            AUnitOfTheErase::TheQueueJobRecord => "the queue job record",
            AUnitOfTheErase::TheHoldRecordNamingIt => "the hold record naming it",
            AUnitOfTheErase::TheOperationAndCommandRecords => "the operation and command records",
            AUnitOfTheErase::TheCaptureRecordsAndBothStreams => {
                "the capture records and both streams"
            }
            AUnitOfTheErase::TheJobRecordDirectory => "the job record directory",
            AUnitOfTheErase::TheRemapMaterial => "the remap material",
        }
    }
}

/// The action's own name in the §9.4 record.
pub const THE_ACTIONS_NAME: &str = "the machine-side erase";

/// **X2, stated as a value: the erase does not touch the job output folder or
/// anything in it.**
///
/// *"The rendered frames are files of the user's, and the spec says in as many
/// words that deleting history destroys the record and not the renders."*
pub const THE_ERASE_NEVER_REACHES_THE_JOB_OUTPUT_FOLDER: bool = true;

/// **Where each unit sits on the machine.**
///
/// **A unit may be several paths** — the operation and command records, and
/// the capture records with both streams, each cover more than one file — and
/// the unit is done when every path under it is gone. **Atomic within each
/// directory removal** is what makes a directory unit one act.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhereTheUnitsSit {
    pub the_machines_history_copy: Vec<PathBuf>,
    pub the_per_job_note: Vec<PathBuf>,
    pub the_queue_job_record: Vec<PathBuf>,
    pub the_hold_record_naming_it: Vec<PathBuf>,
    /// **The erase's own command record is not in this list**, and the
    /// composer is what keeps it out.
    pub the_operation_and_command_records: Vec<PathBuf>,
    pub the_capture_records_and_both_streams: Vec<PathBuf>,
    pub the_job_record_directory: Vec<PathBuf>,
    pub the_remap_material: Vec<PathBuf>,
}

impl WhereTheUnitsSit {
    pub fn of(&self, unit: AUnitOfTheErase) -> &[PathBuf] {
        match unit {
            AUnitOfTheErase::TheMachinesHistoryCopy => &self.the_machines_history_copy,
            AUnitOfTheErase::ThePerJobNote => &self.the_per_job_note,
            AUnitOfTheErase::TheQueueJobRecord => &self.the_queue_job_record,
            AUnitOfTheErase::TheHoldRecordNamingIt => &self.the_hold_record_naming_it,
            AUnitOfTheErase::TheOperationAndCommandRecords => {
                &self.the_operation_and_command_records
            }
            AUnitOfTheErase::TheCaptureRecordsAndBothStreams => {
                &self.the_capture_records_and_both_streams
            }
            AUnitOfTheErase::TheJobRecordDirectory => &self.the_job_record_directory,
            AUnitOfTheErase::TheRemapMaterial => &self.the_remap_material,
        }
    }

    /// **The erase's own command record, excepted from the units by
    /// construction.**
    ///
    /// Anything equal to it is dropped from the operation-and-command unit, so
    /// an act cannot remove the record of itself even if a composer hands it
    /// in.
    pub fn excepting_the_erases_own_command_record(mut self, its_own: &Path) -> WhereTheUnitsSit {
        self.the_operation_and_command_records
            .retain(|at| at != its_own);
        self
    }
}

/// **X3's record, opened before the first unit is attempted.**
///
/// **It is `State, identity and recovery` §9.4's record in the local record**,
/// under a freshly issued operation identity, and it is not a store of this
/// document's own.
pub fn open_the_record<'a>(keeper: &'a mut dyn Keeps) -> Result<Writing<'a>, Refused> {
    let units: Vec<&str> = AUnitOfTheErase::THE_ENUMERATION
        .iter()
        .map(|unit| unit.name())
        .collect();
    Writing::open(keeper, THE_ACTIONS_NAME, &units)
}

/// What the erase left.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheEraseLeft {
    /// The record this run wrote into.
    pub record: Identity,
    /// **The tombstone's one flag** — set only when every unit is done.
    pub machine_side_copy: MachineSideCopy,
}

/// **X3, performed: every unit attempted, each outcome written as it becomes
/// true, and the flag set only on all units done.**
///
/// **A unit already absent is *done*, never *failed***, which is what lets the
/// erase run again against a machine where earlier units already went.
///
/// **No unit is retried inside this action** (`State, identity and recovery`
/// §9.5): a failed unit is recorded and the remainder is a new action, which
/// is [`the_remainder`].
pub fn run_the_erase(
    record: &mut Writing<'_>,
    where_they_sit: &WhereTheUnitsSit,
    remove: impl Fn(&Path) -> Result<(), String>,
) -> Result<MachineSideCopy, Refused> {
    let did = erase_the_units(where_they_sit, remove);
    for (unit, outcome) in &did {
        record.a_unit_ended(unit.name(), outcome.clone())?;
    }
    Ok(the_flag_after(&did))
}

/// **The machine's half: every unit attempted once, in the enumeration's
/// order, and what each came to.** Nothing is written here; the machine's
/// answer carries it to the desktop's record.
///
/// **A unit is *done* when every path under it is gone**, an absent one
/// included; **every path is tried even after one refuses**, so a failed unit
/// leaves as little behind as it can.
pub fn erase_the_units(
    where_they_sit: &WhereTheUnitsSit,
    remove: impl Fn(&Path) -> Result<(), String>,
) -> Vec<(AUnitOfTheErase, UnitOutcome)> {
    AUnitOfTheErase::THE_ENUMERATION
        .into_iter()
        .map(|unit| {
            let refused = where_they_sit
                .of(unit)
                .iter()
                .filter(|at| remove(at).is_err())
                .count();
            let outcome = if refused == 0 { UnitOutcome::Done } else { UnitOutcome::Failed };
            (unit, outcome)
        })
        .collect()
}

/// **Whether anything of the job stands under any unit** — the machine's
/// reading of *a machine holding that identity* (`Queue & History` X4,
/// `State, identity and recovery` H5). **A machine where nothing stands is not
/// one the debt is owed to**, so it is told apart from one where every unit
/// was just removed.
pub fn anything_stands(where_they_sit: &WhereTheUnitsSit) -> bool {
    AUnitOfTheErase::THE_ENUMERATION
        .iter()
        .flat_map(|unit| where_they_sit.of(*unit))
        .any(|at| std::fs::symlink_metadata(at).is_ok())
}

/// **The tombstone's one flag after one action**: removed only when every
/// unit is done.
pub fn the_flag_after(did: &[(AUnitOfTheErase, UnitOutcome)]) -> MachineSideCopy {
    let every_unit_is_done = did.len() == AUnitOfTheErase::THE_ENUMERATION.len()
        && did.iter().all(|(_, outcome)| *outcome == UnitOutcome::Done);
    if every_unit_is_done {
        MachineSideCopy::Removed
    } else {
        MachineSideCopy::StillOwed
    }
}

/// **The one way the answer names the units not done** — their §9.4 names,
/// joined. Both ends read it through this file, so neither spells it.
const BETWEEN_TWO_UNITS: &str = "; ";

/// The units not done, as the answer carries them. Empty where every unit is
/// done.
pub fn the_units_not_done_as_written(did: &[(AUnitOfTheErase, UnitOutcome)]) -> String {
    did.iter()
        .filter(|(_, outcome)| *outcome != UnitOutcome::Done)
        .map(|(unit, _)| unit.name())
        .collect::<Vec<_>>()
        .join(BETWEEN_TWO_UNITS)
}

/// **The desktop's reading of the machine's answer**, unit by unit: every unit
/// it names as not done is *failed*, and every other is *done*. **Only an
/// answer that says the erase ran is read this way** — one that never came
/// back leaves the record's units with no outcome, which is the truth.
pub fn what_the_answer_says(units_not_done: &str) -> Vec<(AUnitOfTheErase, UnitOutcome)> {
    let named: Vec<&str> = units_not_done
        .split(BETWEEN_TWO_UNITS)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .collect();
    AUnitOfTheErase::THE_ENUMERATION
        .into_iter()
        .map(|unit| {
            let outcome = if named.contains(&unit.name()) {
                UnitOutcome::Failed
            } else {
                UnitOutcome::Done
            };
            (unit, outcome)
        })
        .collect()
}

/// **The desktop's half: what the machine did, written into the record the
/// desktop opened before asking**, by that record's identity — so the local
/// record is never held across the ask.
pub fn write_what_the_machine_did(
    keeper: &mut dyn Keeps,
    record: &Identity,
    did: &[(AUnitOfTheErase, UnitOutcome)],
) -> Result<MachineSideCopy, Refused> {
    for (unit, outcome) in did {
        keeper.a_unit_ended(record, unit.name(), outcome.clone())?;
    }
    Ok(the_flag_after(did))
}

/// **One path removed, whatever shape it is, and an absent one is done.**
///
/// **Atomic within each directory removal**: a directory goes whole.
pub fn remove_whatever_is_there(at: &Path) -> Result<(), String> {
    let outcome = if at.is_dir() {
        std::fs::remove_dir_all(at)
    } else {
        std::fs::remove_file(at)
    };
    match outcome {
        Ok(()) => Ok(()),
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(why) => Err(format!("{}: {why}", at.display())),
    }
}

/// **X3's remainder: a new action with a new operation identity, composed from
/// the outcome record's enumeration.**
///
/// **It is composed from what the record says was not done and from nothing
/// else** — never from a fresh look at the machine, which would be a second
/// answer to a question the record already answers.
///
/// **A unit whose outcome never landed is owed too**, because a record caught
/// between doing a part and saying so cannot tell a builder which it was, and
/// re-running a unit already done is *done* by the absent-is-done rule.
pub fn the_remainder(recorded: &Recorded) -> Vec<AUnitOfTheErase> {
    AUnitOfTheErase::THE_ENUMERATION
        .into_iter()
        .filter(|unit| {
            recorded
                .units
                .iter()
                .find(|(named, _)| named == unit.name())
                .map(|(_, outcome)| !matches!(outcome, Some(UnitOutcome::Done)))
                .unwrap_or(false)
        })
        .collect()
}

/// **X3's own unresolved half, recorded beside the code.**
///
/// *"That the widened erase completes as specified has not been observed —
/// one removal per box, not repeated, streams not included."* **What holds is
/// the outcome record**, which says per unit what was done and what was not,
/// and the tombstone's flag, which is set only on all units done. **Nothing
/// verifies the removal afterwards**, because a verification pass would be a
/// rescue for a known hole.
pub const THE_WIDENED_ERASE_HAS_NOT_BEEN_OBSERVED_TO_COMPLETE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;
    use std::collections::HashMap;

    /// A §9.4 keeper in hand. **`State, identity and recovery` §8.6
    /// `State, identity and recovery` F19's rule — a unit outcome is
    /// immutable — is kept here too**, so a test cannot overwrite one.
    #[derive(Default)]
    struct RecordsInHand(HashMap<String, Recorded>);

    impl Keeps for RecordsInHand {
        fn enumerate(&mut self, action: &str, units: &[&str]) -> Result<Identity, Refused> {
            let identity = Identity::issue(IdentityKind::Operation).expect("randomness");
            self.0.insert(
                identity.as_text(),
                Recorded {
                    identity: identity.clone(),
                    action: action.to_string(),
                    units: units.iter().map(|unit| (unit.to_string(), None)).collect(),
                },
            );
            Ok(identity)
        }

        fn a_unit_ended(
            &mut self,
            record: &Identity,
            unit: &str,
            outcome: UnitOutcome,
        ) -> Result<(), Refused> {
            let recorded = self
                .0
                .get_mut(&record.as_text())
                .expect("the record was opened");
            for (named, standing) in recorded.units.iter_mut() {
                if named == unit {
                    assert!(standing.is_none(), "a unit outcome is immutable");
                    *standing = Some(outcome);
                    return Ok(());
                }
            }
            panic!("{unit} is not in the enumeration")
        }

        fn read(&self, record: &Identity) -> Result<Option<Recorded>, Refused> {
            Ok(self.0.get(&record.as_text()).cloned())
        }

        fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
            self.0.remove(&record.as_text());
            Ok(())
        }
    }

    fn a_root(named: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("smoozy-queue-erase-{}-{named}", std::process::id()));
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(&root).expect("the place is made");
        root
    }

    fn lay_out(root: &Path) -> WhereTheUnitsSit {
        let one = |name: &str| {
            let at = root.join(name);
            std::fs::write(&at, b"{}").unwrap();
            vec![at]
        };
        let directory = |name: &str| {
            let at = root.join(name);
            std::fs::create_dir_all(at.join("inside")).unwrap();
            std::fs::write(at.join("inside/file"), b"x").unwrap();
            vec![at]
        };
        WhereTheUnitsSit {
            the_machines_history_copy: one("history.json"),
            the_per_job_note: one("note.json"),
            the_queue_job_record: one("record.json"),
            the_hold_record_naming_it: one("hold.hold"),
            the_operation_and_command_records: vec![root.join("op.json"), root.join("cmd.json")]
                .into_iter()
                .inspect(|at| std::fs::write(at, b"{}").unwrap())
                .collect(),
            the_capture_records_and_both_streams: vec![
                root.join("capture.json"),
                root.join("ordinary.log"),
                root.join("complaints.log"),
            ]
            .into_iter()
            .inspect(|at| std::fs::write(at, b"{}").unwrap())
            .collect(),
            the_job_record_directory: directory("job-record"),
            the_remap_material: directory("remaps"),
        }
    }

    /// **X2 and X3: every unit goes, each outcome lands, and the flag is set
    /// only when all eight are done.**
    #[test]
    fn a_whole_erase_removes_all_eight_units_and_sets_the_flag() {
        let root = a_root("whole");
        let where_they_sit = lay_out(&root);
        let output_folder = root.join("outputs");
        std::fs::create_dir_all(&output_folder).unwrap();
        std::fs::write(output_folder.join("f_0001.png"), b"a frame").unwrap();

        let mut keeper = RecordsInHand::default();
        let mut record = open_the_record(&mut keeper).expect("opened");
        let flag =
            run_the_erase(&mut record, &where_they_sit, remove_whatever_is_there).expect("run");
        assert_eq!(flag, MachineSideCopy::Removed);
        let recorded = record.read().expect("read").expect("there");
        assert_eq!(recorded.units.len(), 8);
        assert!(recorded
            .units
            .iter()
            .all(|(_, outcome)| *outcome == Some(UnitOutcome::Done)));
        assert!(the_remainder(&recorded).is_empty());

        // **The renders are untouched.**
        assert!(output_folder.join("f_0001.png").exists());
        const _: () = assert!(THE_ERASE_NEVER_REACHES_THE_JOB_OUTPUT_FOLDER);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A unit already absent is *done*, never *failed***, so the erase can
    /// run against a machine where earlier units already went.
    #[test]
    fn an_erase_over_units_already_gone_records_every_one_done() {
        let root = a_root("absent");
        let where_they_sit = lay_out(&root);
        for unit in AUnitOfTheErase::THE_ENUMERATION {
            for at in where_they_sit.of(unit) {
                remove_whatever_is_there(at).unwrap();
            }
        }
        let mut keeper = RecordsInHand::default();
        let mut record = open_the_record(&mut keeper).expect("opened");
        let flag =
            run_the_erase(&mut record, &where_they_sit, remove_whatever_is_there).expect("run");
        assert_eq!(flag, MachineSideCopy::Removed);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **X3: the flag is set only when every unit is done, and the remainder
    /// is composed from the record's own enumeration.**
    #[test]
    fn one_failed_unit_leaves_the_debt_standing_and_names_itself_in_the_remainder() {
        let root = a_root("partial");
        let where_they_sit = lay_out(&root);
        let mut keeper = RecordsInHand::default();
        let mut record = open_the_record(&mut keeper).expect("opened");
        let flag = run_the_erase(&mut record, &where_they_sit, |at| {
            if at.ends_with("complaints.log") {
                Err("refused".to_string())
            } else {
                remove_whatever_is_there(at)
            }
        })
        .expect("run");
        assert_eq!(
            flag,
            MachineSideCopy::StillOwed,
            "the debt stands until every unit is done"
        );
        let recorded = record.read().expect("read").expect("there");
        assert_eq!(
            the_remainder(&recorded),
            vec![AUnitOfTheErase::TheCaptureRecordsAndBothStreams]
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **The erase's own command record is never among the records the act
    /// removes.**
    #[test]
    fn the_erases_own_command_record_is_kept_out_of_the_units() {
        let root = a_root("own");
        let its_own = root.join("cmd.json");
        let where_they_sit = lay_out(&root).excepting_the_erases_own_command_record(&its_own);
        assert!(!where_they_sit
            .the_operation_and_command_records
            .contains(&its_own));
        let mut keeper = RecordsInHand::default();
        let mut record = open_the_record(&mut keeper).expect("opened");
        run_the_erase(&mut record, &where_they_sit, remove_whatever_is_there).expect("run");
        assert!(its_own.exists(), "the record of the act survives the act");
        std::fs::remove_dir_all(&root).ok();
    }

    /// **The hold record is its own unit**, because a hold left unreleased
    /// blocks every package commit on that machine.
    #[test]
    fn the_hold_record_is_its_own_unit_and_gates_the_flag_like_any_other() {
        assert!(AUnitOfTheErase::THE_ENUMERATION.contains(&AUnitOfTheErase::TheHoldRecordNamingIt));
        let root = a_root("hold");
        let where_they_sit = lay_out(&root);
        let mut keeper = RecordsInHand::default();
        let mut record = open_the_record(&mut keeper).expect("opened");
        let flag = run_the_erase(&mut record, &where_they_sit, |at| {
            if at.ends_with("hold.hold") {
                Err("refused".to_string())
            } else {
                remove_whatever_is_there(at)
            }
        })
        .expect("run");
        assert_eq!(flag, MachineSideCopy::StillOwed);
        const _: () = assert!(THE_WIDENED_ERASE_HAS_NOT_BEEN_OBSERVED_TO_COMPLETE);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **The 38's piece 16: what the machine did crosses as words and is
    /// written on the desktop unit by unit**, and a unit it names as not done
    /// keeps the debt standing.
    #[test]
    fn the_machines_answer_is_read_back_unit_by_unit_and_gates_the_flag() {
        let root = a_root("across");
        let where_they_sit = lay_out(&root);
        assert!(anything_stands(&where_they_sit));
        let did = erase_the_units(&where_they_sit, |at| {
            if at.ends_with("hold.hold") {
                Err("refused".to_string())
            } else {
                remove_whatever_is_there(at)
            }
        });
        let said = the_units_not_done_as_written(&did);
        assert_eq!(said, "the hold record naming it");
        assert_eq!(what_the_answer_says(&said), did);

        let mut keeper = RecordsInHand::default();
        let opened = open_the_record(&mut keeper).expect("opened");
        let record = opened.identity().clone();
        let flag = write_what_the_machine_did(&mut keeper, &record, &what_the_answer_says(&said))
            .expect("written");
        assert_eq!(flag, MachineSideCopy::StillOwed);
        let recorded = keeper.read(&record).expect("read").expect("there");
        assert_eq!(the_remainder(&recorded), vec![AUnitOfTheErase::TheHoldRecordNamingIt]);

        // **The remainder, as a new action**: only the hold stands now.
        let did = erase_the_units(&where_they_sit, remove_whatever_is_there);
        assert_eq!(the_units_not_done_as_written(&did), "");
        assert_eq!(the_flag_after(&what_the_answer_says("")), MachineSideCopy::Removed);
        assert!(!anything_stands(&where_they_sit), "nothing of the job stands now");
        std::fs::remove_dir_all(&root).ok();
    }
}

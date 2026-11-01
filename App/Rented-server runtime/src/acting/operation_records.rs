//! The operation records and the command records in the package area —
//! `State, identity and recovery` §1.2's two machine-side stores whose content
//! is that document's.
//!
//! **[`smoozy_state::machine_operations`] holds the rules.** This module is the
//! write and the read.
//!
//! # Why these two are written here at all
//!
//! **The rule is one document's and the store is another's, which is the split
//! every row module in this stage already draws.** `State, identity and
//! recovery` §1.2's *content owned by* column gives both records to that
//! document, and its §1.3 rule 1 makes **the supervisor the only writer of every
//! durable store in that inventory** — and the supervisor is this crate's
//! program. **So the form is that document's and the bytes are this one's**;
//! [`crate::written_form::a_machine_side_record`] is where the bytes are settled
//! and says so.
//!
//! **This is what `Preparation/Build/Plan.md` step 4 assigned to step 6 and step
//! 6 did not build.** Until it landed, `Server protocol and transfers` C7 had
//! nothing to be true of — an acknowledgement says the request is recorded, and
//! nothing on any machine recorded one — and that document's §10 floor rested on
//! it through its own G2.
//!
//! # What each write is
//!
//! **Every write here is [`crate::acting::durable::publish`]**, which is
//! `State, identity and recovery` §1.3 rule 6's four steps: write, flush the
//! file, rename, flush the directory. **The ordering that matters is the one
//! outside this file** — the record is written durably **before** the act it
//! names, which is `Server protocol and transfers` C4 and that document's G1,
//! and the trait's own shape is what keeps it.
//!
//! # Two rules kept at the writer rather than left to a caller
//!
//! **An identity already recorded is refused rather than replaced**
//! (`Server protocol and transfers` C4), and the refusal looks in **both**
//! stores: an identity is an operation record's or a command record's and never
//! both, so a caller that recorded a stop and then tried to record a worker
//! under the same identity is refused here.
//!
//! **A record that is there and will not read back is a refusal and never a
//! skip.** §6 step 2 selects *each* operation record with no recorded outcome,
//! so a reconciliation that quietly passed over an unreadable one would leave an
//! open operation nobody ever takes a branch for — a job that is neither resumed
//! nor re-run nor given up, on a machine that is being paid for.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C4.
//! `State, identity and recovery` J11.
//!
//! **Two of `State, identity and recovery`'s are deliberately not listed there**
//! — **G1** and **G4** — because this document defines a G-series of its own, so
//! a header list could not tell the two apart. Each is written out with its
//! document at the site.

use std::path::{Path, PathBuf};

use smoozy_state::identities::Identity;
use smoozy_state::machine_operations::{
    CommandRecord, Keeps, OperationRecord, Refused, WhatTheMachineHoldsUnderAnIdentity,
};

use crate::acting::durable;
use crate::written_form::a_machine_side_record::{
    ACommandRecordAsWritten, AnOperationRecordAsWritten,
};
use crate::written_form::{self, TheRecordCouldNotBeRead};

/// **What a record's own name ends with**, and the two are different so that a
/// reader of the area can tell which store a name belongs to without opening it.
///
/// **A publication in flight is `<name>.writing`** — [`durable::the_staging_name_for`]'s
/// own suffix — so neither of these ever matches one, and the enumeration below
/// cannot pick up a half-written record.
const AN_OPERATION_RECORDS_SUFFIX: &str = ".operation";
const A_COMMAND_RECORDS_SUFFIX: &str = ".command";

/// **Where the two stores sit under the package area**, and the name is a Build
/// decision.
///
/// **No clause fixes it.** `State, identity and recovery` §1.2 puts both records
/// in the package area and `State, identity and recovery` J11 puts every store
/// there **outside every job folder**; neither that document nor this one names
/// a segment, exactly as
/// neither names one for the five records `crate::the_program::records` carries.
///
/// **It is named here rather than beside those five** because that module says
/// in its own words that only this plan's own records are named there, and these
/// two are `State, identity and recovery`'s content. **The store is this
/// crate's, so its area's name is too.**
pub const THE_AREA_THE_TWO_RECORDS_SIT_IN: &str = "operation-records";

/// The two record stores, in the package area.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheOperationRecordsOnTheMachine {
    /// Where the records sit, inside the package area.
    pub area: PathBuf,
}

impl TheOperationRecordsOnTheMachine {
    /// In an area, made with the permission the package area's own stores carry.
    pub fn in_the_area(area: &Path) -> Result<Self, String> {
        durable::make_the_area(area).map_err(|why| why.as_text())?;
        Ok(TheOperationRecordsOnTheMachine {
            area: area.to_path_buf(),
        })
    }

    /// **A record is reached by its identity**, which is what the whole store is
    /// for: `Server protocol and transfers` C8's question is *what became of
    /// this identity*.
    fn where_an_operation_sits(&self, operation: &Identity) -> PathBuf {
        self.area
            .join(format!("{}{AN_OPERATION_RECORDS_SUFFIX}", operation.as_text()))
    }

    fn where_a_command_sits(&self, operation: &Identity) -> PathBuf {
        self.area
            .join(format!("{}{A_COMMAND_RECORDS_SUFFIX}", operation.as_text()))
    }

    fn read_one_operation(&self, at: &Path) -> Result<Option<OperationRecord>, Refused> {
        let Some(bytes) = durable::read(at).map_err(|why| Refused::TheStore(why.as_text()))? else {
            return Ok(None);
        };
        let written: AnOperationRecordAsWritten =
            written_form::read(&bytes).map_err(|why| the_store_said(at, why))?;
        written
            .back()
            .map(Some)
            .map_err(|why| the_store_said(at, why))
    }

    /// **Every operation record of one kind, open or closed** — the same
    /// listing `State, identity and recovery` §6 step 2's selection takes
    /// below, and the same discipline: **every fact comes from inside a
    /// record**, the listing supplies only which records the supervisor has
    /// written, and a name that matches and does not read back refuses the
    /// whole enumeration rather than being passed over.
    ///
    /// **Its one reader is `Queue & History` P1a's published set**, read once
    /// in a supervisor run: the frames of each job whose derivative an
    /// operation of the preview kind published, which are *"the derivative
    /// operations' own records"*. Added 2026-10-03, the 38's piece 3.
    pub fn every_operation_record_of_the_kind(
        &self,
        kind: &str,
    ) -> Result<Vec<OperationRecord>, Refused> {
        let entries = std::fs::read_dir(&self.area).map_err(|why| {
            Refused::TheStore(format!("{}: {why}", self.area.display()))
        })?;
        let mut out = Vec::new();
        for entry in entries {
            let at = entry
                .map_err(|why| Refused::TheStore(format!("{}: {why}", self.area.display())))?
                .path();
            let is_one = at
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(AN_OPERATION_RECORDS_SUFFIX));
            if !is_one {
                continue;
            }
            let Some(record) = self.read_one_operation(&at)? else {
                return Err(Refused::TheStore(format!(
                    "{}: an operation record was listed and was not there when it was read",
                    at.display()
                )));
            };
            if record.kind == kind {
                out.push(record);
            }
        }
        Ok(out)
    }

    fn read_one_command(&self, at: &Path) -> Result<Option<CommandRecord>, Refused> {
        let Some(bytes) = durable::read(at).map_err(|why| Refused::TheStore(why.as_text()))? else {
            return Ok(None);
        };
        let written: ACommandRecordAsWritten =
            written_form::read(&bytes).map_err(|why| the_store_said(at, why))?;
        written
            .back()
            .map(Some)
            .map_err(|why| the_store_said(at, why))
    }
}

fn the_store_said(at: &Path, why: TheRecordCouldNotBeRead) -> Refused {
    Refused::TheStore(format!("{}: {}", at.display(), why.what_the_reader_said))
}

impl Keeps for TheOperationRecordsOnTheMachine {
    /// **`Server protocol and transfers` C4 and `State, identity and recovery`
    /// G1: written durably before the act, and an identity already recorded is
    /// refused rather than replaced.**
    fn record_before_acting(&mut self, record: &OperationRecord) -> Result<(), Refused> {
        if self.what_is_held_under(&record.operation)?
            != WhatTheMachineHoldsUnderAnIdentity::Nothing
        {
            return Err(Refused::ThatIdentityIsAlreadyRecorded);
        }
        self.write_it_back(record)
    }

    /// **`State, identity and recovery` G4: the same ordering for a command that
    /// starts no worker.**
    fn record_a_command_before_acting(&mut self, record: &CommandRecord) -> Result<(), Refused> {
        if self.what_is_held_under(&record.operation)?
            != WhatTheMachineHoldsUnderAnIdentity::Nothing
        {
            return Err(Refused::ThatIdentityIsAlreadyRecorded);
        }
        self.write_a_command_back(record)
    }

    /// **Both stores are looked in**, because an identity is in one or the other
    /// and never both.
    fn what_is_held_under(
        &self,
        operation: &Identity,
    ) -> Result<WhatTheMachineHoldsUnderAnIdentity, Refused> {
        if let Some(record) = self.read_one_operation(&self.where_an_operation_sits(operation))? {
            return Ok(WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record));
        }
        if let Some(record) = self.read_one_command(&self.where_a_command_sits(operation))? {
            return Ok(WhatTheMachineHoldsUnderAnIdentity::ACommandRecord(record));
        }
        Ok(WhatTheMachineHoldsUnderAnIdentity::Nothing)
    }

    fn write_it_back(&mut self, record: &OperationRecord) -> Result<(), Refused> {
        let bytes = written_form::write(&AnOperationRecordAsWritten::of(record))
            .map_err(|why| Refused::TheStore(why.what_the_reader_said))?;
        durable::publish(&self.where_an_operation_sits(&record.operation), &bytes)
            .map_err(|why| Refused::TheStore(why.as_text()))
    }

    fn write_a_command_back(&mut self, record: &CommandRecord) -> Result<(), Refused> {
        let bytes = written_form::write(&ACommandRecordAsWritten::of(record))
            .map_err(|why| Refused::TheStore(why.what_the_reader_said))?;
        durable::publish(&self.where_a_command_sits(&record.operation), &bytes)
            .map_err(|why| Refused::TheStore(why.as_text()))
    }

    /// **`State, identity and recovery` §6 step 2's selection: every operation
    /// record with no recorded outcome.**
    ///
    /// **This is the one place in this crate that reads a directory, and it is
    /// worth saying why that is not the thing rule 5 forbids.** That rule bars
    /// reading a file's presence, name or size **as state** — *"Every state fact
    /// comes from a record Smoozy or the supervisor wrote."* **Every state fact
    /// here comes from inside a record**: whether an operation is open is its
    /// own recorded outcome, read out of the file. What the listing supplies is
    /// only which records the supervisor has written, and §6 gives no other
    /// route to them — `State, identity and recovery` §1.2's inventory holds one
    /// row for the operation records and no index beside it.
    ///
    /// **Two hazards are closed rather than hoped about.** A publication in
    /// flight is named `<record>.writing` and matches neither suffix, so a
    /// half-written record is never opened. And **a name that matches and does
    /// not read back refuses the whole enumeration**: skipping it would leave an
    /// open operation that no run ever takes a branch for, which is a job
    /// neither resumed nor re-run nor given up on a machine being paid for.
    ///
    /// **Command records are not in this loop at all**, which their suffix keeps
    /// as well as their shape.
    fn every_record_eligible_for_reconciliation(&self) -> Result<Vec<OperationRecord>, Refused> {
        let entries = std::fs::read_dir(&self.area).map_err(|why| {
            Refused::TheStore(format!("{}: {why}", self.area.display()))
        })?;
        let mut open = Vec::new();
        for entry in entries {
            let at = entry
                .map_err(|why| Refused::TheStore(format!("{}: {why}", self.area.display())))?
                .path();
            let is_one = at
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(AN_OPERATION_RECORDS_SUFFIX));
            if !is_one {
                continue;
            }
            let Some(record) = self.read_one_operation(&at)? else {
                // The name was there when the directory was read and the file
                // was gone when it was opened. Nothing on this machine removes
                // an operation record, so this is not an ordinary case; it is
                // reported rather than passed over.
                return Err(Refused::TheStore(format!(
                    "{}: an operation record was listed and was not there when it was read",
                    at.display()
                )));
            };
            if record.is_eligible_for_reconciliation() {
                open.push(record);
            }
        }
        Ok(open)
    }

    /// **§13 R1's one term for a command record of its own: the command record
    /// of the desktop's cursor move ends as the command closes.**
    ///
    /// **It is the only thing on this machine that removes a record**, and its
    /// two refusals are what keep it that narrow. **An open command record is
    /// refused**, so nothing ends a command whose outcome was never written —
    /// that record is S14's pending write's, not this. **An operation record is
    /// refused**, because no term of R1's ends one this way and the enumeration
    /// above rests on nothing removing one.
    ///
    /// **An identity nothing is held under is not a refusal.** Ending is safe
    /// to repeat, which is what lets the caller answer the move and report a
    /// refusal on the complaints stream without the two getting tangled.
    fn end_a_closed_command_record(&mut self, operation: &Identity) -> Result<(), Refused> {
        match self.what_is_held_under(operation)? {
            WhatTheMachineHoldsUnderAnIdentity::Nothing => return Ok(()),
            WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(_) => {
                return Err(Refused::TheStore(format!(
                    "{}: that identity holds an operation record, which no term of §13 R1 ends \
                     this way",
                    operation.as_text()
                )));
            }
            WhatTheMachineHoldsUnderAnIdentity::ACommandRecord(record) => {
                if record.recorded_outcome().is_none() {
                    return Err(Refused::TheStore(format!(
                        "{}: that command record is still open and is not ended here",
                        operation.as_text()
                    )));
                }
            }
        }
        let at = self.where_a_command_sits(operation);
        match std::fs::remove_file(&at) {
            Ok(()) => Ok(()),
            // Gone between the read and the removal. The record is ended either
            // way, which is what the caller asked for.
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(why) => Err(Refused::TheStore(format!("{}: {why}", at.display()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::machine_operations::{
        Disposition, RecordedOutcome, WhichOperationItActsOn,
    };

    fn an_area() -> PathBuf {
        std::env::temp_dir().join(format!(
            "smoozy-runtime-operation-records-{}-{}",
            std::process::id(),
            {
                static NEXT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            }
        ))
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    fn a_record() -> OperationRecord {
        OperationRecord::opened(
            an_identity(IdentityKind::Operation),
            an_identity(IdentityKind::Job),
            "render",
        )
    }

    /// **A record is written and read back whole**, through the store a
    /// supervisor would use.
    #[test]
    fn a_record_is_written_and_read_back_by_its_identity() {
        let area = an_area();
        let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();
        let record = a_record();
        store.record_before_acting(&record).unwrap();
        assert_eq!(
            store.what_is_held_under(&record.operation).unwrap(),
            WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record)
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **An identity nothing was recorded under answers *nothing*.**
    #[test]
    fn an_identity_the_machine_never_recorded_reads_as_nothing() {
        let area = an_area();
        let store = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();
        assert_eq!(
            store
                .what_is_held_under(&an_identity(IdentityKind::Operation))
                .unwrap(),
            WhatTheMachineHoldsUnderAnIdentity::Nothing
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **`Server protocol and transfers` C4 at the store: a request repeating an
    /// identity already recorded never acts again**, and the first record
    /// stands.
    #[test]
    fn a_second_record_under_one_identity_is_refused_and_the_first_stands() {
        let area = an_area();
        let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();
        let record = a_record();
        store.record_before_acting(&record).unwrap();
        let mut a_repeat = record.clone();
        a_repeat.kind = "preview".into();
        assert_eq!(
            store.record_before_acting(&a_repeat),
            Err(Refused::ThatIdentityIsAlreadyRecorded)
        );
        let held = store.what_is_held_under(&record.operation).unwrap();
        assert_eq!(
            held,
            WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record)
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **The refusal looks in both stores**: an identity is one store's or the
    /// other's and never both.
    #[test]
    fn an_identity_recorded_as_a_command_is_refused_as_an_operation() {
        let area = an_area();
        let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();
        let command = CommandRecord::submitted(
            an_identity(IdentityKind::Operation),
            WhichOperationItActsOn::TheCallerNamedIt(an_identity(IdentityKind::Operation)),
        );
        store.record_a_command_before_acting(&command).unwrap();
        let mut as_an_operation = a_record();
        as_an_operation.operation = command.operation.clone();
        assert_eq!(
            store.record_before_acting(&as_an_operation),
            Err(Refused::ThatIdentityIsAlreadyRecorded)
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A record survives every restart the machine can have**: a reader that
    /// did not write it finds it exactly as it was, with its disposition and its
    /// attempt count.
    #[test]
    fn a_record_is_read_back_by_a_supervisor_that_did_not_write_it() {
        let area = an_area();
        let run = an_identity(IdentityKind::Run);
        let mut record = a_record();
        {
            let mut writing = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();
            writing.record_before_acting(&record).unwrap();
            record
                .a_run_took_this_branch(Disposition::ReRunOnce, run.clone())
                .unwrap();
            writing.write_it_back(&record).unwrap();
        }
        let a_later_run = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();
        let held = a_later_run.what_is_held_under(&record.operation).unwrap();
        assert_eq!(
            held,
            WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record)
        );
        assert_eq!(
            held.what_an_open_record_carries(),
            Some((Disposition::ReRunOnce, 2))
        );
        std::fs::remove_dir_all(&area).ok();
    }

    /// **`State, identity and recovery` §6 step 2: the open operation records
    /// and nothing else** — not a closed one, and not a command record.
    #[test]
    fn reconciliation_selects_the_open_operation_records_and_neither_of_the_other_two() {
        let area = an_area();
        let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();

        let open = a_record();
        store.record_before_acting(&open).unwrap();

        let mut closed = a_record();
        store.record_before_acting(&closed).unwrap();
        closed.close(RecordedOutcome::GivenUp, None).unwrap();
        store.write_it_back(&closed).unwrap();

        store
            .record_a_command_before_acting(&CommandRecord::submitted(
                an_identity(IdentityKind::Operation),
                WhichOperationItActsOn::TheCallerNamedIt(an_identity(IdentityKind::Operation)),
            ))
            .unwrap();

        let selected = store.every_record_eligible_for_reconciliation().unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].operation, open.operation);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **Every record of one kind, open and closed, and none of another kind
    /// or a command** — `Queue & History` P1a's published set's one reading.
    #[test]
    fn every_record_of_one_kind_is_listed_open_or_closed_and_nothing_else() {
        let area = an_area();
        let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();
        let a_preview = || {
            OperationRecord::opened(
                an_identity(IdentityKind::Operation),
                an_identity(IdentityKind::Job),
                "preview",
            )
        };
        let open = a_preview();
        store.record_before_acting(&open).unwrap();
        let mut closed = a_preview();
        store.record_before_acting(&closed).unwrap();
        closed
            .close(RecordedOutcome::TheKindsOwn("published".to_string()), None)
            .unwrap();
        store.write_it_back(&closed).unwrap();
        store.record_before_acting(&a_record()).unwrap();
        store
            .record_a_command_before_acting(&CommandRecord::submitted(
                an_identity(IdentityKind::Operation),
                WhichOperationItActsOn::TheCallerNamedIt(an_identity(IdentityKind::Operation)),
            ))
            .unwrap();
        let mut listed: Vec<Identity> = store
            .every_operation_record_of_the_kind("preview")
            .unwrap()
            .into_iter()
            .map(|record| record.operation)
            .collect();
        listed.sort_by_key(|operation| operation.as_text());
        let mut wanted = vec![open.operation.clone(), closed.operation.clone()];
        wanted.sort_by_key(|operation| operation.as_text());
        assert_eq!(listed, wanted);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A publication in flight is never picked up.** A file at the staging
    /// name is left in the area and the enumeration does not open it, which is
    /// what its suffix is for.
    #[test]
    fn a_half_written_record_is_never_read_as_one() {
        let area = an_area();
        let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();
        let record = a_record();
        store.record_before_acting(&record).unwrap();

        let interrupted = durable::the_staging_name_for(&store.where_an_operation_sits(
            &an_identity(IdentityKind::Operation),
        ));
        std::fs::write(&interrupted, b"half of a record\n").unwrap();

        let selected = store.every_record_eligible_for_reconciliation().unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].operation, record.operation);
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A record that is there and will not read back refuses the whole
    /// enumeration rather than being passed over**, because a skipped open
    /// record is an operation nothing ever takes a branch for.
    #[test]
    fn an_unreadable_record_refuses_the_enumeration_rather_than_being_skipped() {
        let area = an_area();
        let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();
        store.record_before_acting(&a_record()).unwrap();
        let damaged = store.where_an_operation_sits(&an_identity(IdentityKind::Operation));
        std::fs::write(&damaged, b"this is not a record\n").unwrap();

        let why = store
            .every_record_eligible_for_reconciliation()
            .expect_err("an unreadable record is a refusal");
        assert!(matches!(why, Refused::TheStore(_)));
        std::fs::remove_dir_all(&area).ok();
    }

    /// **A command record is written and read back too**, with the subject the
    /// supervisor resolved.
    #[test]
    fn a_command_record_carries_the_subject_the_supervisor_resolved() {
        let area = an_area();
        let mut store = TheOperationRecordsOnTheMachine::in_the_area(&area).unwrap();
        let mut command = CommandRecord::submitted(
            an_identity(IdentityKind::Operation),
            WhichOperationItActsOn::ARuleTheSupervisorResolves {
                the_calling_documents_rule: "whatever is rendering on this machine".into(),
                resolved_to: None,
            },
        );
        store.record_a_command_before_acting(&command).unwrap();
        let target = an_identity(IdentityKind::Operation);
        command
            .the_supervisor_resolved_the_rule(target.clone())
            .unwrap();
        store.write_a_command_back(&command).unwrap();

        let held = store.what_is_held_under(&command.operation).unwrap();
        assert_eq!(
            held,
            WhatTheMachineHoldsUnderAnIdentity::ACommandRecord(command)
        );
        std::fs::remove_dir_all(&area).ok();
    }
}

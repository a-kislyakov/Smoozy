//! The operation records and the command records on a rented machine —
//! `State, identity and recovery` §1.2 and §5.1.
//!
//! # What this is, and why it is here rather than beside the supervisor
//!
//! **§1.2 names two of the machine's durable stores as this document's own**:
//! *the operation records*, one per operation that starts a worker, and *the
//! command records*, one per command that starts none. The table's own
//! *content owned by* column says **this document; per-kind fields are the
//! kind's document's** for the first and **this document** for the second. So
//! what a record holds, what may be written into it and in what order, is
//! settled here.
//!
//! **What is not here is the store.** §1.3 rule 1 makes the supervisor the only
//! writer of every durable store in §1.2, and the supervisor is
//! `Rented-server runtime`'s program. That crate's `acting` half is the write
//! and the read; **the bytes it lays down are a Build decision recorded there**,
//! because no clause anywhere fixes a written form for these two any more than
//! it does for the four stores that document owns.
//!
//! **Nothing in this file opens a file, and that is what makes §5.1 and §6
//! testable at the desk** — the same split every other member of this stage
//! draws.
//!
//! # Two values this file carries without reading
//!
//! **The operation's kind** is `Rented-server runtime` W1's closed set of
//! seven, and **the recorded execution group** is that document's X1 pair. This
//! file cannot name either type: that crate reads this one and not the other
//! way round. **Both are carried opaquely and neither is parsed here**, which is
//! the same shape `Rented-server runtime`'s own `PerKindField` uses in the
//! other direction — *"Opaque here. The owning document reads it; this one
//! carries it."*
//!
//! # What is deliberately absent
//!
//! **The event journal**, which is §4's and is the other machine-side store
//! this document owns. §6 step 4 journals every disposition written, so a
//! supervisor that reconciles owes the journal as well — and **nothing here
//! stands in for it**. [`WHAT_A_RECORD_DOES_NOT_DO`] says so where a reader of
//! this file meets it.
//!
//! **Reconciliation itself**, which is §6's table over these records. What is
//! here is the record it reads and writes; the loop is the supervisor's.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` X1, S2.
//! `Server protocol and transfers` C4.
//!
//! **Two more of `Rented-server runtime`'s are deliberately not listed there** —
//! **W1** and **W4** — because this document defines a W-series of its own, so a
//! header list could not tell the two apart. Each is written out with its
//! document at the site.
//!
//! **A marker this document also defines is never listed there and is written
//! out with its document at the site**, because a header list cannot tell the
//! two apart and a bare one there would resolve to a real and wrong clause.

use crate::identities::Identity;

/// **G1's third field: the desired state.**
///
/// **G3: *ended* is written only in the same durable write as the recorded
/// outcome**, so the two can never disagree and reconciliation can never meet a
/// record that is *ended* and open. [`OperationRecord::close`] is the only way
/// to write either, which is how that ordering is kept by the shape rather than
/// by a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesiredState {
    ShouldRun,
    ShouldBeStopped,
    /// Written only with the recorded outcome.
    Ended,
}

/// **G2's first half: a reconciliation disposition is what one supervisor run
/// did with this record.**
///
/// The four names are `The shape`'s. **It is written at every run that takes a
/// branch for the record, it replaces the one before it, and it never closes the
/// record.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    Resumed,
    ReRunOnce,
    LeftStopped,
    GivenUp,
}

impl Disposition {
    pub const ALL: [Disposition; 4] = [
        Disposition::Resumed,
        Disposition::ReRunOnce,
        Disposition::LeftStopped,
        Disposition::GivenUp,
    ];

    /// **G2's second half: two of the four dispositions are terminal results as
    /// well, and where reconciliation writes one it writes the recorded outcome
    /// in the same act.** *Resumed* and *re-run once* write no outcome.
    pub fn the_outcome_it_writes_with_itself(self) -> Option<RecordedOutcome> {
        match self {
            Disposition::LeftStopped => Some(RecordedOutcome::LeftStopped),
            Disposition::GivenUp => Some(RecordedOutcome::GivenUp),
            Disposition::Resumed | Disposition::ReRunOnce => None,
        }
    }
}

/// **G2: the operation's one terminal result, and only a terminal result may set
/// it.**
///
/// *Left stopped*, *given up*, **or the kind's own ending, which includes the
/// ordinary successful one** — and that third is the kind's document's word
/// rather than this one's, so it is carried and not read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordedOutcome {
    LeftStopped,
    GivenUp,
    /// **The kind's own ending**, in the owning document's own words.
    TheKindsOwn(String),
}

/// **§5.2's pair, carried rather than composed here.**
///
/// `Rented-server runtime` X1 records the group leader's identity together with
/// the value a later process cannot present, and that document owns both halves.
/// **This file holds what a record carries and never validates one**: G6 puts
/// every validation at the stop and the re-attach, which are that document's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRecordedExecutionGroup {
    /// The pair as the document that recorded it writes it down. Opaque here.
    pub as_that_document_recorded_it: String,
    /// **`Rented-server runtime` S2's run identifier** — the supervisor run that
    /// started this attempt. G9: a group recorded under a previous run
    /// identifier is treated as gone unless it validates.
    pub started_under_run: Identity,
}

/// **G1. One durable operation record per operation that starts a worker,
/// written by the supervisor before any worker is started.**
///
/// The eight things it holds are the eight fields below, in G1's own order.
///
/// **`Server protocol and transfers` C4 is the ordering this record exists to
/// keep**: the machine records the operation identity durably **before acting on
/// it**, and a request repeating an identity already recorded never acts again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationRecord {
    /// The operation identity, and **the subject identity it acts on** — a job,
    /// a server-setup run, a transfer, a preview, or another operation, which is
    /// what a stop acts on.
    pub operation: Identity,
    pub subject: Identity,
    /// **The operation's kind.** `Rented-server runtime` W1's set of seven,
    /// carried as that document writes it and not parsed here.
    pub kind: String,
    desired_state: DesiredState,
    /// **The attempt count.** §6's table reads 1 and 2 and nothing else, the
    /// re-run-once rule being what bounds it.
    pub attempt_count: u32,
    /// **The recorded execution group of the current attempt**, paired per §5.2,
    /// **and the run identifier of the supervisor run that started it.**
    pub execution_group: Option<TheRecordedExecutionGroup>,
    /// **The kind's own safe-boundary marker**, whose meaning belongs to the
    /// document that defines the kind — `Rented-server runtime` W4's per-kind
    /// field. **This document only requires it to exist and to be durable**, so
    /// it is bytes here and is never read.
    pub the_kinds_safe_boundary_marker: Option<Vec<u8>>,
    /// **The latest reconciliation disposition and the run identifier that wrote
    /// it.** Absent until a run has taken a branch for this record.
    latest_disposition: Option<(Disposition, Identity)>,
    /// **The recorded outcome, written once and never rewritten.**
    recorded_outcome: Option<RecordedOutcome>,
}

/// Why a write into a record was refused. **Each is a rule of §5.1 kept rather
/// than a fault of a store's**; a store's own troubles are its keeper's to
/// report in its own words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// **G2: the recorded outcome is written once and never rewritten**, and
    /// G3 makes a record carrying one closed.
    ItAlreadyCarriesARecordedOutcome,
    /// **G3: a record with a recorded outcome is closed**, and §6 step 2 does
    /// not select it — so nothing writes a disposition over one either.
    ItIsClosedAndTakesNoFurtherDisposition,
    /// **C4's ordering, from the other side**: an identity already recorded
    /// never acts again, so a second record under one is refused rather than
    /// replacing the first.
    ThatIdentityIsAlreadyRecorded,
    /// The store said no. **Its own words**, because what a mount or a
    /// filesystem said is not this file's to paraphrase.
    TheStore(String),
}

impl Refused {
    /// **The refusal in plain words**, for a message a person may read. Added
    /// 2026-09-21, step 10c: the machine's own answers carried this type's
    /// debugging rendering behind *What the server said*.
    pub fn in_words(&self) -> String {
        match self {
            Refused::ItAlreadyCarriesARecordedOutcome => {
                "that operation has already ended, and its ending is not rewritten".to_string()
            }
            Refused::ItIsClosedAndTakesNoFurtherDisposition => {
                "that operation has already ended, so nothing more is done to it".to_string()
            }
            Refused::ThatIdentityIsAlreadyRecorded => {
                "an operation under that identity is already recorded".to_string()
            }
            Refused::TheStore(said) => said.clone(),
        }
    }
}

impl OperationRecord {
    /// **G1's own ordering: the record is opened before any worker is started**,
    /// which is the whole of what makes `Server protocol and transfers` C4
    /// implementable. A fresh record is **open** — no disposition, no outcome —
    /// and its desired state is *should run*.
    pub fn opened(operation: Identity, subject: Identity, kind: &str) -> Self {
        OperationRecord {
            operation,
            subject,
            kind: kind.to_string(),
            desired_state: DesiredState::ShouldRun,
            attempt_count: 1,
            execution_group: None,
            the_kinds_safe_boundary_marker: None,
            latest_disposition: None,
            recorded_outcome: None,
        }
    }

    /// **The one route a store reads a record back through.**
    ///
    /// **The four fields the rules above guard are private on purpose**, so that
    /// the only ways to set them are the acts that are allowed to — the stop,
    /// the disposition, the close. **A store reading its own bytes back is the
    /// one caller that legitimately sets all four at once**, and it gets a named
    /// route rather than public fields, the same shape `Identity::read_back`
    /// already uses for the same reason.
    ///
    /// **It restores and never repairs.** Where the bytes carry a state the
    /// rules above would refuse — an outcome without *ended*, or the two
    /// disagreeing — that is a record somebody wrote wrongly, and it is read
    /// back as it stands so a reader meets it rather than a tidied copy.
    #[allow(clippy::too_many_arguments)]
    pub fn read_back(
        operation: Identity,
        subject: Identity,
        kind: String,
        desired_state: DesiredState,
        attempt_count: u32,
        execution_group: Option<TheRecordedExecutionGroup>,
        the_kinds_safe_boundary_marker: Option<Vec<u8>>,
        latest_disposition: Option<(Disposition, Identity)>,
        recorded_outcome: Option<RecordedOutcome>,
    ) -> Self {
        OperationRecord {
            operation,
            subject,
            kind,
            desired_state,
            attempt_count,
            execution_group,
            the_kinds_safe_boundary_marker,
            latest_disposition,
            recorded_outcome,
        }
    }

    pub fn desired_state(&self) -> DesiredState {
        self.desired_state
    }

    pub fn latest_disposition(&self) -> Option<&(Disposition, Identity)> {
        self.latest_disposition.as_ref()
    }

    pub fn recorded_outcome(&self) -> Option<&RecordedOutcome> {
        self.recorded_outcome.as_ref()
    }

    /// **G3. A record with no recorded outcome is eligible for reconciliation,
    /// at this run and at every later one**, however many dispositions it has
    /// already carried. **A record with a recorded outcome is closed.**
    pub fn is_eligible_for_reconciliation(&self) -> bool {
        self.recorded_outcome.is_none()
    }

    /// **§5.3 step 2: the target's desired state is written to *should be
    /// stopped*, durably, before anything else.**
    ///
    /// It is not an ending: the record stays open and eligible, and what closes
    /// it is the wait at §5.3 step 6 or §6's table.
    pub fn the_stop_was_asked_for(&mut self) -> Result<(), Refused> {
        if self.recorded_outcome.is_some() {
            return Err(Refused::ItIsClosedAndTakesNoFurtherDisposition);
        }
        self.desired_state = DesiredState::ShouldBeStopped;
        Ok(())
    }

    /// **G2: a disposition that does not close the record.** *Resumed* and
    /// *re-run once*, each **written durably before the act it names** — before
    /// the supervisor re-attaches, and before a second attempt is started.
    ///
    /// **It replaces the one before it and it never closes the record.** A
    /// disposition that is one of the two terminal ones is refused here, because
    /// those two write the outcome in the same act and [`OperationRecord::close`]
    /// is that act.
    pub fn a_run_took_this_branch(
        &mut self,
        disposition: Disposition,
        run: Identity,
    ) -> Result<(), Refused> {
        if self.recorded_outcome.is_some() {
            return Err(Refused::ItIsClosedAndTakesNoFurtherDisposition);
        }
        if disposition.the_outcome_it_writes_with_itself().is_some() {
            return Err(Refused::ItAlreadyCarriesARecordedOutcome);
        }
        if disposition == Disposition::ReRunOnce {
            self.attempt_count = 2;
        }
        self.latest_disposition = Some((disposition, run));
        Ok(())
    }

    /// **G2 and G3 in one act: the recorded outcome, the desired state *ended*,
    /// and — where reconciliation is what closed it — the terminal disposition,
    /// all in the same durable write.**
    ///
    /// **This is the only way either of those two fields is ever set**, which is
    /// what makes G3's *the loop can never meet a record that is ended and open*
    /// true by construction rather than by a rule somebody keeps.
    ///
    /// **A second close is refused**: the outcome is written once and never
    /// rewritten.
    pub fn close(
        &mut self,
        outcome: RecordedOutcome,
        by_reconciliation: Option<(Disposition, Identity)>,
    ) -> Result<(), Refused> {
        if self.recorded_outcome.is_some() {
            return Err(Refused::ItAlreadyCarriesARecordedOutcome);
        }
        if let Some((disposition, run)) = by_reconciliation {
            if disposition.the_outcome_it_writes_with_itself() != Some(outcome.clone()) {
                return Err(Refused::ItIsClosedAndTakesNoFurtherDisposition);
            }
            self.latest_disposition = Some((disposition, run));
        }
        self.recorded_outcome = Some(outcome);
        self.desired_state = DesiredState::Ended;
        Ok(())
    }
}

/// **G4. A command that starts no worker gets a command record instead.**
///
/// A stop, a deletion, a query, an acknowledged submission. **It holds the
/// operation identity, the subject identity it acts on, and the recorded
/// outcome, and nothing else**: no desired state, no attempt count, no execution
/// group, no safe-boundary marker, no disposition.
///
/// **§6 step 2 never selects one**, which is what the absence of a desired state
/// makes true rather than a rule anybody keeps: there is no field for a branch
/// of that table to read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandRecord {
    pub operation: Identity,
    /// **Which operation this command acts on.** G4: for a stop it is the
    /// operation identity the caller named, **or the one the supervisor
    /// resolved a selection rule to** — one field with one meaning, and the only
    /// thing that moves is when it is filled.
    subject: WhichOperationItActsOn,
    recorded_outcome: Option<RecordedOutcome>,
}

/// **G4's subject field, and its two ways of being filled.**
///
/// **What is not admitted is a stop with neither a named target nor a selection
/// rule**, so there is no third member here, and **a subject identity is never
/// rewritten once it has been resolved.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhichOperationItActsOn {
    /// Filled at submission, by a caller that can name the target.
    TheCallerNamedIt(Identity),
    /// Filled at resolution, by the supervisor, from the rule the calling
    /// document stated. **The rule is never invented by the supervisor.**
    ARuleTheSupervisorResolves {
        the_calling_documents_rule: String,
        resolved_to: Option<Identity>,
    },
}

impl CommandRecord {
    /// **Written at submission, whichever way the subject is filled.**
    pub fn submitted(operation: Identity, subject: WhichOperationItActsOn) -> Self {
        CommandRecord {
            operation,
            subject,
            recorded_outcome: None,
        }
    }

    /// **The one route a store reads a command record back through**, for the
    /// reason [`OperationRecord::read_back`] gives.
    pub fn read_back(
        operation: Identity,
        subject: WhichOperationItActsOn,
        recorded_outcome: Option<RecordedOutcome>,
    ) -> Self {
        CommandRecord {
            operation,
            subject,
            recorded_outcome,
        }
    }

    pub fn subject(&self) -> &WhichOperationItActsOn {
        &self.subject
    }

    pub fn recorded_outcome(&self) -> Option<&RecordedOutcome> {
        self.recorded_outcome.as_ref()
    }

    /// **G4: the supervisor's resolution completes the field durably**, once,
    /// and it is never rewritten. **A rule that resolves to nothing is a
    /// recorded outcome and not an unresolved record left open**, so the caller
    /// closes the record with *nothing was rendering* in its own words rather
    /// than leaving the field empty.
    pub fn the_supervisor_resolved_the_rule(&mut self, to: Identity) -> Result<(), Refused> {
        match &mut self.subject {
            WhichOperationItActsOn::TheCallerNamedIt(_) => {
                Err(Refused::ThatIdentityIsAlreadyRecorded)
            }
            WhichOperationItActsOn::ARuleTheSupervisorResolves { resolved_to, .. } => {
                if resolved_to.is_some() {
                    return Err(Refused::ThatIdentityIsAlreadyRecorded);
                }
                *resolved_to = Some(to);
                Ok(())
            }
        }
    }

    /// **Written once and never rewritten**, the same rule the operation
    /// record's outcome carries.
    pub fn close(&mut self, outcome: RecordedOutcome) -> Result<(), Refused> {
        if self.recorded_outcome.is_some() {
            return Err(Refused::ItAlreadyCarriesARecordedOutcome);
        }
        self.recorded_outcome = Some(outcome);
        Ok(())
    }
}

/// The two records a machine holds under one operation identity, as whatever
/// asks the machine about an identity finds them.
///
/// **An identity is in one store or the other and never in both**, which is
/// §5.1's own split: a command that starts a worker gets an operation record and
/// one that starts none gets a command record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheMachineHoldsUnderAnIdentity {
    AnOperationRecord(OperationRecord),
    ACommandRecord(CommandRecord),
    /// **Nothing was ever recorded under it.**
    Nothing,
}

impl WhatTheMachineHoldsUnderAnIdentity {
    /// The recorded outcome, whichever store the identity sits in, or nothing
    /// where the record is open or absent.
    pub fn recorded_outcome(&self) -> Option<&RecordedOutcome> {
        match self {
            WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record) => {
                record.recorded_outcome()
            }
            WhatTheMachineHoldsUnderAnIdentity::ACommandRecord(record) => record.recorded_outcome(),
            WhatTheMachineHoldsUnderAnIdentity::Nothing => None,
        }
    }

    /// **The latest reconciliation disposition and the attempt count**, which is
    /// what an open operation record carries back to a caller that asks about a
    /// running operation. A command record carries neither, having no field for
    /// either.
    pub fn what_an_open_record_carries(&self) -> Option<(Disposition, u32)> {
        match self {
            WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record) => record
                .latest_disposition()
                .map(|(disposition, _)| (*disposition, record.attempt_count)),
            _ => None,
        }
    }
}

/// The two stores, behind a trait.
///
/// **The ordering is the whole of `Server protocol and transfers` C4 and of
/// G1**, and it is the trait's shape rather than a note: there is no method
/// that starts anything, so the only thing a caller can do first is write the
/// record. **The supervisor is the only implementor on a machine** (§1.3 rule
/// 1); a test implements it over a list.
pub trait Keeps {
    /// **C4 and G1: write the record durably, before acting on the identity.**
    ///
    /// **An identity already recorded is refused rather than replaced**, which
    /// is C4's *a request repeating an identity already recorded never acts
    /// again* kept at the store instead of left to a caller.
    fn record_before_acting(
        &mut self,
        record: &OperationRecord,
    ) -> Result<(), Refused>;

    /// **G4: the same ordering for a command that starts no worker.**
    fn record_a_command_before_acting(&mut self, record: &CommandRecord) -> Result<(), Refused>;

    /// What is held under one identity, in either store.
    fn what_is_held_under(
        &self,
        operation: &Identity,
    ) -> Result<WhatTheMachineHoldsUnderAnIdentity, Refused>;

    /// Write a record back after a rule above has changed it. **Every rule that
    /// may change one is a method on the record itself**, so a keeper never
    /// decides what a record now says — it writes down what the record already
    /// says.
    fn write_it_back(&mut self, record: &OperationRecord) -> Result<(), Refused>;

    /// The same, for a command record.
    fn write_a_command_back(&mut self, record: &CommandRecord) -> Result<(), Refused>;

    /// **§6 step 2's selection: every operation record with no recorded
    /// outcome.** Command records are not in this loop at all.
    fn every_record_eligible_for_reconciliation(&self) -> Result<Vec<OperationRecord>, Refused>;

    /// **End a closed command record, by its identity** — the one term
    /// `Rented-server runtime` §13 R1 gives a command record of its own, for
    /// the desktop's cursor move: *the command record of the desktop's cursor
    /// move ends as the command closes*.
    ///
    /// **It refuses a record that is still open**, so nothing can end a command
    /// whose outcome was never written, **and it refuses an operation record**,
    /// which no term of R1's ends this way. **An identity nothing is held
    /// under is not an error**: ending is safe to repeat, which is what lets
    /// the caller report a refusal and carry on.
    ///
    /// **Why the trait carries it rather than a caller deleting a file.** The
    /// store is the only thing that knows where a command record sits, and
    /// `State, identity and recovery` §1.3's rules bind whoever writes there.
    fn end_a_closed_command_record(&mut self, operation: &Identity) -> Result<(), Refused>;
}

/// **What a record does not do, said here so nobody reads the store as more than
/// it is.**
///
/// **A record is not the event journal.** §4 is a separate store of this
/// document's and §6 step 4 journals every disposition written and every outcome
/// written with one, *"so a reader learns of them under Section 4 rather than by
/// comparing two readings"*. **Nothing here writes an event, and nothing here
/// stands in for one**: a reader that learned of a disposition by reading this
/// store twice would be doing the exact thing that step exists to prevent.
///
/// **And a record is not reconciliation.** §6's table is the loop that reads
/// these records and writes their dispositions; what is here is the record.
pub const WHAT_A_RECORD_DOES_NOT_DO: &str =
    "the event journal is §4's separate store and no disposition written here reaches a reader \
     until it is journalled; §6's reconciliation is the loop over these records and is not this \
     file's";

#[cfg(test)]
pub mod a_list {
    //! A keeper that holds its records in two lists — the whole of §5.1's
    //! rules with none of a machine, so the tests below can run at the desk.

    use super::*;

    #[derive(Default)]
    pub struct ALists {
        pub operations: Vec<OperationRecord>,
        pub commands: Vec<CommandRecord>,
    }

    impl Keeps for ALists {
        fn record_before_acting(&mut self, record: &OperationRecord) -> Result<(), Refused> {
            if self.what_is_held_under(&record.operation)?
                != WhatTheMachineHoldsUnderAnIdentity::Nothing
            {
                return Err(Refused::ThatIdentityIsAlreadyRecorded);
            }
            self.operations.push(record.clone());
            Ok(())
        }

        fn record_a_command_before_acting(&mut self, record: &CommandRecord) -> Result<(), Refused> {
            if self.what_is_held_under(&record.operation)?
                != WhatTheMachineHoldsUnderAnIdentity::Nothing
            {
                return Err(Refused::ThatIdentityIsAlreadyRecorded);
            }
            self.commands.push(record.clone());
            Ok(())
        }

        fn what_is_held_under(
            &self,
            operation: &Identity,
        ) -> Result<WhatTheMachineHoldsUnderAnIdentity, Refused> {
            if let Some(held) = self
                .operations
                .iter()
                .find(|held| held.operation == *operation)
            {
                return Ok(WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(
                    held.clone(),
                ));
            }
            if let Some(held) = self.commands.iter().find(|held| held.operation == *operation) {
                return Ok(WhatTheMachineHoldsUnderAnIdentity::ACommandRecord(
                    held.clone(),
                ));
            }
            Ok(WhatTheMachineHoldsUnderAnIdentity::Nothing)
        }

        fn write_it_back(&mut self, record: &OperationRecord) -> Result<(), Refused> {
            let Some(held) = self
                .operations
                .iter_mut()
                .find(|held| held.operation == record.operation)
            else {
                return Err(Refused::TheStore("no such record".into()));
            };
            *held = record.clone();
            Ok(())
        }

        fn write_a_command_back(&mut self, record: &CommandRecord) -> Result<(), Refused> {
            let Some(held) = self
                .commands
                .iter_mut()
                .find(|held| held.operation == record.operation)
            else {
                return Err(Refused::TheStore("no such record".into()));
            };
            *held = record.clone();
            Ok(())
        }

        fn every_record_eligible_for_reconciliation(
            &self,
        ) -> Result<Vec<OperationRecord>, Refused> {
            Ok(self
                .operations
                .iter()
                .filter(|held| held.is_eligible_for_reconciliation())
                .cloned()
                .collect())
        }

        fn end_a_closed_command_record(&mut self, operation: &Identity) -> Result<(), Refused> {
            if self
                .operations
                .iter()
                .any(|held| held.operation == *operation)
            {
                return Err(Refused::TheStore(
                    "that identity holds an operation record, which no term ends this way".into(),
                ));
            }
            let Some(at) = self
                .commands
                .iter()
                .position(|held| held.operation == *operation)
            else {
                return Ok(());
            };
            if self.commands[at].recorded_outcome().is_none() {
                return Err(Refused::TheStore(
                    "that command record is still open and is not ended here".into(),
                ));
            }
            self.commands.remove(at);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::a_list::ALists;
    use super::*;
    use crate::identities::IdentityKind;

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

    /// **G1: a fresh record is open, at its first attempt, with no disposition
    /// and no outcome**, and its desired state is *should run*.
    #[test]
    fn a_fresh_record_is_open_and_says_the_work_should_run() {
        let record = a_record();
        assert_eq!(record.desired_state(), DesiredState::ShouldRun);
        assert_eq!(record.attempt_count, 1);
        assert!(record.latest_disposition().is_none());
        assert!(record.recorded_outcome().is_none());
        assert!(record.is_eligible_for_reconciliation());
    }

    /// **G2: a disposition and an outcome are two different things and they are
    /// never one field.** Two of the four write an outcome with themselves and
    /// two write none.
    #[test]
    fn two_of_the_four_dispositions_write_an_outcome_and_two_write_none() {
        for disposition in Disposition::ALL {
            let writes = disposition.the_outcome_it_writes_with_itself();
            match disposition {
                Disposition::LeftStopped => assert_eq!(writes, Some(RecordedOutcome::LeftStopped)),
                Disposition::GivenUp => assert_eq!(writes, Some(RecordedOutcome::GivenUp)),
                Disposition::Resumed | Disposition::ReRunOnce => assert_eq!(writes, None),
            }
        }
    }

    /// **G3: a record stays open and eligible however many dispositions it has
    /// carried**, which is the sentence that makes a resumed operation reachable
    /// at every later run.
    #[test]
    fn a_record_that_has_carried_two_dispositions_is_still_eligible() {
        let mut record = a_record();
        record
            .a_run_took_this_branch(Disposition::Resumed, an_identity(IdentityKind::Run))
            .unwrap();
        record
            .a_run_took_this_branch(Disposition::ReRunOnce, an_identity(IdentityKind::Run))
            .unwrap();
        assert!(record.is_eligible_for_reconciliation());
        assert!(record.recorded_outcome().is_none());
        // The later disposition replaces the earlier one.
        assert_eq!(
            record.latest_disposition().map(|(which, _)| *which),
            Some(Disposition::ReRunOnce)
        );
        // **§6's table writes the count of 2 with the disposition**, so the
        // re-run-once branch cannot be taken a second time.
        assert_eq!(record.attempt_count, 2);
    }

    /// **G3: desired state *ended* is written only in the same durable write as
    /// the recorded outcome**, so the loop can never meet a record that is
    /// *ended* and open. **Held by the shape**: `close` is the only route to
    /// either field.
    #[test]
    fn ended_and_the_recorded_outcome_are_written_in_one_act_and_never_apart() {
        let mut record = a_record();
        assert_ne!(record.desired_state(), DesiredState::Ended);
        record
            .close(
                RecordedOutcome::TheKindsOwn("the render finished".into()),
                None,
            )
            .unwrap();
        assert_eq!(record.desired_state(), DesiredState::Ended);
        assert!(record.recorded_outcome().is_some());
        assert!(!record.is_eligible_for_reconciliation());
    }

    /// **G2: the recorded outcome is written once and never rewritten**, and a
    /// closed record takes no further disposition either.
    #[test]
    fn a_closed_record_is_closed_against_both_a_second_outcome_and_a_later_disposition() {
        let mut record = a_record();
        record.close(RecordedOutcome::GivenUp, None).unwrap();
        assert_eq!(
            record.close(RecordedOutcome::LeftStopped, None),
            Err(Refused::ItAlreadyCarriesARecordedOutcome)
        );
        assert_eq!(
            record.a_run_took_this_branch(Disposition::Resumed, an_identity(IdentityKind::Run)),
            Err(Refused::ItIsClosedAndTakesNoFurtherDisposition)
        );
        assert_eq!(record.recorded_outcome(), Some(&RecordedOutcome::GivenUp));
    }

    /// **The two terminal dispositions write the outcome in the same act**, and
    /// neither can be written as an ordinary disposition — which is what keeps
    /// them from closing a record without an outcome.
    #[test]
    fn a_terminal_disposition_is_written_only_with_the_outcome_it_carries() {
        let run = an_identity(IdentityKind::Run);
        for disposition in [Disposition::LeftStopped, Disposition::GivenUp] {
            let mut record = a_record();
            assert!(record
                .a_run_took_this_branch(disposition, run.clone())
                .is_err());
            let outcome = disposition.the_outcome_it_writes_with_itself().unwrap();
            record
                .close(outcome.clone(), Some((disposition, run.clone())))
                .unwrap();
            assert_eq!(record.recorded_outcome(), Some(&outcome));
            assert_eq!(
                record.latest_disposition().map(|(which, _)| *which),
                Some(disposition)
            );
        }
    }

    /// **A close by reconciliation whose disposition does not carry that outcome
    /// is refused**, so the pair cannot be written disagreeing.
    #[test]
    fn a_terminal_disposition_cannot_be_written_beside_an_outcome_it_does_not_carry() {
        let mut record = a_record();
        assert_eq!(
            record.close(
                RecordedOutcome::GivenUp,
                Some((Disposition::LeftStopped, an_identity(IdentityKind::Run)))
            ),
            Err(Refused::ItIsClosedAndTakesNoFurtherDisposition)
        );
        assert!(record.is_eligible_for_reconciliation());
    }

    /// **§5.3 step 2: the stop writes the desired state and does not end the
    /// operation.**
    #[test]
    fn asking_for_a_stop_writes_the_desired_state_and_leaves_the_record_open() {
        let mut record = a_record();
        record.the_stop_was_asked_for().unwrap();
        assert_eq!(record.desired_state(), DesiredState::ShouldBeStopped);
        assert!(record.is_eligible_for_reconciliation());
    }

    /// **`Server protocol and transfers` C4 at the store: an identity already
    /// recorded never acts again**, so a second record under one is refused.
    #[test]
    fn a_second_record_under_one_identity_is_refused_rather_than_replacing_the_first() {
        let mut keeper = ALists::default();
        let record = a_record();
        keeper.record_before_acting(&record).unwrap();
        let mut a_repeat = record.clone();
        a_repeat.kind = "preview".into();
        assert_eq!(
            keeper.record_before_acting(&a_repeat),
            Err(Refused::ThatIdentityIsAlreadyRecorded)
        );
        let held = keeper.what_is_held_under(&record.operation).unwrap();
        assert_eq!(
            held,
            WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record)
        );
    }

    /// **An identity is in one store or the other and never in both.**
    #[test]
    fn an_identity_recorded_as_a_command_is_refused_as_an_operation_and_the_other_way_round() {
        let mut keeper = ALists::default();
        let command = CommandRecord::submitted(
            an_identity(IdentityKind::Operation),
            WhichOperationItActsOn::TheCallerNamedIt(an_identity(IdentityKind::Operation)),
        );
        keeper.record_a_command_before_acting(&command).unwrap();
        let mut as_an_operation = a_record();
        as_an_operation.operation = command.operation.clone();
        assert_eq!(
            keeper.record_before_acting(&as_an_operation),
            Err(Refused::ThatIdentityIsAlreadyRecorded)
        );
    }

    /// **§6 step 2: command records are not in the loop at all**, which is what
    /// having no desired state makes true — there is no field for a branch to
    /// read.
    #[test]
    fn reconciliation_selects_open_operation_records_and_no_command_record() {
        let mut keeper = ALists::default();
        let open = a_record();
        let mut closed = a_record();
        closed.close(RecordedOutcome::GivenUp, None).unwrap();
        keeper.record_before_acting(&open).unwrap();
        keeper.record_before_acting(&closed).unwrap();
        keeper.write_it_back(&closed).unwrap();
        keeper
            .record_a_command_before_acting(&CommandRecord::submitted(
                an_identity(IdentityKind::Operation),
                WhichOperationItActsOn::TheCallerNamedIt(an_identity(IdentityKind::Operation)),
            ))
            .unwrap();

        let selected = keeper.every_record_eligible_for_reconciliation().unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].operation, open.operation);
    }

    /// **G4: the subject field is one field with one meaning and the only thing
    /// that moves is when it is filled** — and a resolved one is never
    /// rewritten.
    #[test]
    fn a_stops_subject_is_filled_once_whichever_way_it_was_filled() {
        let named = CommandRecord::submitted(
            an_identity(IdentityKind::Operation),
            WhichOperationItActsOn::TheCallerNamedIt(an_identity(IdentityKind::Operation)),
        );
        let mut named = named;
        assert_eq!(
            named.the_supervisor_resolved_the_rule(an_identity(IdentityKind::Operation)),
            Err(Refused::ThatIdentityIsAlreadyRecorded)
        );

        let mut by_rule = CommandRecord::submitted(
            an_identity(IdentityKind::Operation),
            WhichOperationItActsOn::ARuleTheSupervisorResolves {
                the_calling_documents_rule: "whatever is rendering on this machine".into(),
                resolved_to: None,
            },
        );
        let target = an_identity(IdentityKind::Operation);
        by_rule
            .the_supervisor_resolved_the_rule(target.clone())
            .unwrap();
        assert_eq!(
            by_rule.the_supervisor_resolved_the_rule(an_identity(IdentityKind::Operation)),
            Err(Refused::ThatIdentityIsAlreadyRecorded)
        );
        assert_eq!(
            by_rule.subject(),
            &WhichOperationItActsOn::ARuleTheSupervisorResolves {
                the_calling_documents_rule: "whatever is rendering on this machine".into(),
                resolved_to: Some(target),
            }
        );
    }

    /// **What an open record carries back to a caller asking about it** — the
    /// latest disposition and the attempt count — and a command record carries
    /// neither, having no field for either.
    #[test]
    fn an_open_record_carries_its_disposition_and_attempt_count_and_a_command_record_carries_none() {
        let mut record = a_record();
        record
            .a_run_took_this_branch(Disposition::ReRunOnce, an_identity(IdentityKind::Run))
            .unwrap();
        let held = WhatTheMachineHoldsUnderAnIdentity::AnOperationRecord(record);
        assert_eq!(
            held.what_an_open_record_carries(),
            Some((Disposition::ReRunOnce, 2))
        );
        assert!(held.recorded_outcome().is_none());

        let command = WhatTheMachineHoldsUnderAnIdentity::ACommandRecord(CommandRecord::submitted(
            an_identity(IdentityKind::Operation),
            WhichOperationItActsOn::TheCallerNamedIt(an_identity(IdentityKind::Operation)),
        ));
        assert_eq!(command.what_an_open_record_carries(), None);
    }

    /// **The journal is not this store**, and the sentence saying so names both
    /// halves of what is absent.
    #[test]
    fn what_a_record_does_not_do_names_the_journal_and_reconciliation() {
        assert!(WHAT_A_RECORD_DOES_NOT_DO.contains("event journal"));
        assert!(WHAT_A_RECORD_DOES_NOT_DO.contains("reconciliation"));
    }
}

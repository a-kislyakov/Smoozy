//! **§4 performed — the command records written durably before acting, their
//! travel, the supervisor's one serialized write of each rewrite, the removal
//! of a record and the material cleanup that follows, the mark's write riding
//! the removal's outcome, and the drop at the admission point.**
//!
//! # Nothing here decides anything
//!
//! **Every rewrite is computed as a pure function over the records as they
//! stand** — [`crate::line::reorder`], [`crate::line::block_move`],
//! [`crate::line::the_admission_point`], and `stop_and_clear` in
//! `smoozy-queue-history` — and this module reads the records, hands them to those functions, and writes back
//! what they answer. **A branch taken here that is not one of theirs would be
//! a second answer to a question already answered.**
//!
//! # Why the record's removal is the deciding write
//!
//! **L1: the deciding write is the job record's removal — one durable write,
//! after which the job cannot start**, because the admission point no longer
//! sees it (Q3, Q4). **The material cleanup follows it and can never
//! resurrect the job**, which is why a unit of it failing is recorded and
//! retried by the ordinary owed-work route rather than rolled back.
//!
//! **The order is never the other way round.** Cleaning the material first
//! and then failing to remove the record would leave a startable job whose
//! material is gone.
//!
//! # The command record, and whose rule it keeps
//!
//! **`Server protocol and transfers` C4 is the rule**: the machine records the
//! operation identity durably before acting on it, in a command record where
//! the request starts no worker, and a re-sent command carrying the same
//! identity is answered from the record and never acted a second time
//! (`State, identity and recovery` §5.1, §5.4).
//!
//! **The record is the machine's one command store, and it is not here.**
//! ~~The store below is this document's own commands and no others~~ — **there
//! is no store below any longer. Removed 2026-09-29, step 10c's first stop-and-clear piece**, when the
//! program first answered these commands: `smoozy-state`'s command record,
//! kept by `smoozy-runtime`'s operation-record store, is where every other
//! command the machine answers is recorded and where C8's answer about an
//! identity is read, and its `WhichOperationItActsOn::ARuleTheSupervisorResolves`
//! is exactly L13's resolved subject. **A second store here would have been a
//! second place to ask what became of an identity**, which this file's own
//! sentence — *no second command store is opened for anything already owning
//! one* — forbade. The ruling is in this module's Build decisions record.
//!
//! # What calls these, and what does not yet
//!
//! **The program a rented machine runs — `smoozy-server` — calls the cancel,
//! the clear, the material cleanup and D14's finish from step 10c's first stop-and-clear piece**, and
//! the started record's state and the ended record's since step 10c piece 5's
//! second sitting. ~~**The reorder and the block move it still refuses by name**,
//! because nothing on the desktop sends either, so [`apply_a_rewrite`] has no
//! caller outside its tests.~~ **It calls [`apply_a_rewrite`] from 2026-10-04,
//! the 38's piece 8**, for both the reorder and the block move, which is when
//! the desktop first sent either. **Nothing here invents a program and nothing
//! here has a `main`.**
//!
//! **The journal is not written here.** Each act names what is journalled and
//! hands it back; the journal is `Rented-server runtime`'s store under its own
//! single writer.
//!
//! **The stop itself is `Rendering`'s to send** (L10) and its sequence is
//! `State, identity and recovery` §5.3's; L13's resolution of the paired stop
//! is the program's, over [`crate::line::resolve_the_stops_subject`]. What is
//! here is the queue's side: the waiter with its own deadline, and what the
//! entry reads meanwhile, which is [`crate::acts`]'s.
//!
//! # L23 and L24's act record, and why the clear is not one write
//!
//! **The clear spans several job records and this store is one file per
//! record**, so the single write §16's table used to declare cannot exist:
//! `State, identity and recovery` §9.6 P8 settles that no single filesystem
//! operation updates two independent files atomically. **So the clear is
//! partial across records, atomic within each removal, and made whole by a
//! durable record of the act** — L23's, written before the first removal,
//! naming every identity the clear covers, each outcome written as it becomes
//! true, and cleared only when the last covered record is gone. **A reorder
//! and a block move carry the same record** (L24), holding the whole intended
//! key assignment, because the rewrite writes back only the records whose key
//! actually moved and a refusal part-way would otherwise leave an order
//! nothing could recompute.
//!
//! **A refusal part-way leaves the record standing rather than swallowing
//! it**, and [`crate::acting::reconciliation::finish_the_outstanding_acts`] is
//! what finishes it — before the admission point may start anything (D14).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C16, C17.
//! `State, identity and recovery` §5.1, §5.3, §5.4, §9.3, §9.5.
//! `Rented-server runtime` S1. `Dependency Upload & Remap` U2.
//!
//! **`Server protocol and transfers` C4 is deliberately not on that list and
//! is written out at every site instead**, because this plan defines a C4 of
//! its own.

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use smoozy_framing::framing::{Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
use smoozy_framing::requests as asked;
use smoozy_state::identities::{Identity, IdentityKind};

use crate::acting::queue_store::{TheQueueOnTheMachine, WhatTheStoreSaid};
use crate::acts::{AWaitersEnd, MaterialACancelRemoves, WhatTheStopLeft};
use crate::line::JobRecord;
// The rewrite is only shaped here for the act record's own write, which is
// on a unix only.
#[cfg(unix)]
use crate::line::Rewrite;

/// **The queue commands this document sends, and the list is closed.**
///
/// **The name is what travels**, and each is one request of the machine-local
/// endpoint's own framing. **No path and no machine-origin value is composed
/// into a name**, which is `Server protocol and transfers` §1.1's rule.
///
/// **Each name is read from `smoozy_framing::requests`, where both ends read
/// it**, from 2026-09-29. **Until then they were spelled here** as
/// `queue.cancel`, `queue.clear` and the rest, which no program answered — and
/// the answer to a hold as `queue.answer-a-hold`, which is not the name the
/// machine answers that request by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AQueueCommand {
    /// L1.
    CancelAWaitingJob,
    /// L6.
    Reorder,
    /// L7.
    MoveAGroupAsOneBlock,
    /// L13 and L14 — the clear half of the ordered pair.
    Clear,
    /// L13's stop half. **It names no target**; its subject is resolved on the
    /// machine at the clear's write.
    TheStopPairedWithAClear,
    /// H2 — the desk's answer to a hold.
    AnswerAHold,
}

impl AQueueCommand {
    /// The request name, which is a compile-time constant per command.
    pub fn name(self) -> &'static str {
        match self {
            AQueueCommand::CancelAWaitingJob => asked::THE_CANCEL_REQUEST,
            AQueueCommand::Reorder => asked::THE_REORDER_REQUEST,
            AQueueCommand::MoveAGroupAsOneBlock => asked::THE_GROUP_BLOCK_MOVE_REQUEST,
            AQueueCommand::Clear => asked::THE_CLEAR_REQUEST,
            AQueueCommand::TheStopPairedWithAClear => asked::THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST,
            AQueueCommand::AnswerAHold => asked::THE_ANSWER_TO_A_HOLD_REQUEST,
        }
    }
}

/// **How one identity of a clear ended — L23's three, and the list is closed
/// at three.** It is `State, identity and recovery` §9.4's own set of unit
/// outcomes, in the written form this store keeps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HowAnIdentityEnded {
    /// **A record already absent is *done*, never *failed***, the same rule
    /// every removal in this crate keeps.
    Done,
    Failed {
        why: String,
    },
    /// **Never tried, naming what stopped it.**
    NotAttempted {
        stopped_by: String,
    },
}

/// **L23 and L24's record of one act on the line, durable on the machine.**
///
/// **It is the thing that makes the act whole**, in place of the one write
/// §16's table used to declare and `State, identity and recovery` §9.6 P8
/// rules out. A record still standing is an outstanding act and nothing else.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheActsRecord {
    /// **L23: every identity the clear covers, with what became of each.**
    /// The set is L14's, composed at the moment of the clear with the running
    /// one exempted, and written before the first removal.
    AClear {
        covered: Vec<(String, Option<HowAnIdentityEnded>)>,
    },
    /// **L24: the whole intended key assignment, written before the first key
    /// moves** — a reorder's and a block move's alike, they being one act
    /// shape with one record.
    ARewrite { keys: Vec<(String, u64)> },
}

impl TheActsRecord {
    /// **L23: the record is cleared only when the last covered record is
    /// gone.** Until then the act is outstanding.
    pub fn every_identity_is_done(&self) -> bool {
        match self {
            TheActsRecord::AClear { covered } => covered
                .iter()
                .all(|(_, outcome)| outcome == &Some(HowAnIdentityEnded::Done)),
            // A rewrite's record carries the intended assignment and no
            // per-record outcome: it is cleared when the last moved key is
            // written, which is the caller's own answer.
            TheActsRecord::ARewrite { .. } => false,
        }
    }
}

/// **L23 and L24's store on the machine, one durable record per act
/// identity**, beside the command records and written the same way.
///
/// **On a unix only, for the reason the command records above carry**:
/// `State, identity and recovery` J11's inventory of the stores on a rented
/// server, and that document's §1.3 rule 6, which says a rented machine is a
/// Linux machine.
#[cfg(unix)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheActRecordsOnTheMachine {
    at: PathBuf,
}

/// What every published act record's name ends in.
pub const THE_ACT_RECORDS_SUFFIX: &str = ".act.json";

/// **Where L23 and L24's act records sit in the package area**, beside the
/// machine-local queue and never inside it: a clear refused part-way by a store
/// that will not remove from the queue's own directory must still be able to
/// write down what happened. The name is Build's. Added 2026-09-29, step 10c's
/// first stop-and-clear piece, when the program first made this store.
pub const THE_AREA_THE_ACT_RECORDS_SIT_IN: &str = "machine-local-queue-acts";

#[cfg(unix)]
impl TheActRecordsOnTheMachine {
    pub fn at(directory: impl Into<PathBuf>) -> TheActRecordsOnTheMachine {
        TheActRecordsOnTheMachine {
            at: directory.into(),
        }
    }

    fn published(&self, act: &Identity) -> PathBuf {
        self.at
            .join(format!("{}{THE_ACT_RECORDS_SUFFIX}", act.as_text()))
    }

    /// **Written durably before the act's first effect.**
    pub fn record(
        &mut self,
        act: &Identity,
        record: &TheActsRecord,
    ) -> Result<(), WhatTheStoreSaid> {
        let written = serde_json::to_vec(record).map_err(|why| WhatTheStoreSaid::ItWasRefused {
            why: why.to_string(),
        })?;
        smoozy_runtime::acting::durable::publish(&self.published(act), &written)
            .map_err(|why| WhatTheStoreSaid::ItWasRefused { why: why.as_text() })
    }

    pub fn read(&self, act: &Identity) -> Result<Option<TheActsRecord>, WhatTheStoreSaid> {
        let at = self.published(act);
        let Some(bytes) = smoozy_runtime::acting::durable::read(&at)
            .map_err(|why| WhatTheStoreSaid::ItWasRefused { why: why.as_text() })?
        else {
            return Ok(None);
        };
        serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|_| WhatTheStoreSaid::ItIsNotAJobRecord {
                at: at.display().to_string(),
            })
    }

    /// **The record cleared, which is what says the act is no longer
    /// outstanding.** An already-absent record is done.
    pub fn cleared(&mut self, act: &Identity) -> Result<(), WhatTheStoreSaid> {
        match std::fs::remove_file(self.published(act)) {
            Ok(()) => Ok(()),
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(why) => Err(WhatTheStoreSaid::ItWasRefused {
                why: why.to_string(),
            }),
        }
    }

    /// **Every act record still standing — which is every act D14 has to
    /// finish**, in no order of its own.
    pub fn every_outstanding(&self) -> Result<Vec<(Identity, TheActsRecord)>, WhatTheStoreSaid> {
        let entries =
            std::fs::read_dir(&self.at).map_err(|why| WhatTheStoreSaid::ItWasRefused {
                why: format!("{}: {why}", self.at.display()),
            })?;
        let mut out = Vec::new();
        for entry in entries {
            let path = entry
                .map_err(|why| WhatTheStoreSaid::ItWasRefused {
                    why: why.to_string(),
                })?
                .path();
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            let Some(text) = name.strip_suffix(THE_ACT_RECORDS_SUFFIX) else {
                continue;
            };
            let Some(act) = Identity::read_back(IdentityKind::Operation, text) else {
                continue;
            };
            if let Some(record) = self.read(&act)? {
                out.push((act, record));
            }
        }
        Ok(out)
    }
}

/// **The command's travel, composed as one request of the endpoint's own
/// framing and nothing else.**
///
/// **No transport is written here and no command string is composed here.**
/// The frame is `Server protocol and transfers`' own; what this function adds
/// is the name and the fields this document's commands carry.
pub fn the_request_for(command: AQueueCommand, identity: Identity, fields: Fields) -> Request {
    Request::speaking(
        THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
        identity,
        command.name(),
    )
    .carrying(fields)
}

/// One unit of the material cleanup, with what became of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AUnitsOutcome {
    pub unit: MaterialACancelRemoves,
    /// **Absent where the unit is done** — and **a unit already absent is
    /// done, never failed**, because the cleanup must be able to run against a
    /// machine where earlier units already went.
    pub refused: Option<String>,
}

/// **L1's outcome: the deciding write, then the cleanup.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACancelsOutcome {
    /// **The one durable write that decides it.** Where it is refused, nothing
    /// else was attempted and the job is still in the line.
    pub the_record_was_removed: bool,
    pub cleanup: Vec<AUnitsOutcome>,
}

/// Where each of L1's four units sits on the machine.
///
/// **The remap material is composed by `Dependency Upload & Remap` U2's own
/// rule** through [`crate::acts::the_remap_material_a_cancel_removes`] and
/// never by a path made here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereTheMaterialSits {
    pub job_record_directory: PathBuf,
    pub remap_material: PathBuf,
    pub per_job_note: PathBuf,
    /// **`Job files and outputs` P21: absent where no render has chosen the
    /// job's folder yet** — a job whose folder is chosen at render start and
    /// that never reached its render's step 4 has none, and that unit is done.
    pub job_output_folder: Option<PathBuf>,
    /// **`Telegram` V1c's current choice, read off the job record before it
    /// goes**, so what follows a removal hears the choice in force and not only
    /// the manifest's. Not a unit of material: nothing removes it. Absent where
    /// the choice was never changed. Added 2026-10-30, the twenty-first notes,
    /// note 7.
    pub reporting_choice_now: Option<String>,
}

impl WhereTheMaterialSits {
    fn of(&self, unit: MaterialACancelRemoves) -> Option<&PathBuf> {
        match unit {
            MaterialACancelRemoves::TheJobRecordDirectory => Some(&self.job_record_directory),
            MaterialACancelRemoves::TheRemapMaterial => Some(&self.remap_material),
            MaterialACancelRemoves::ThePerJobNote => Some(&self.per_job_note),
            MaterialACancelRemoves::TheEmptyJobOutputFolder => self.job_output_folder.as_ref(),
        }
    }
}

/// **L1, performed: the job record removed first, then the material.**
///
/// **The order is the promise.** After the removal the admission point no
/// longer sees the job, so nothing the cleanup does or fails to do can start
/// it; before the removal nothing of the material is touched, so a refused
/// removal leaves a whole job.
///
/// **Partial across directories and atomic within each**, per `Job files and
/// outputs`' own declaration: each unit is its own removal and a unit that
/// fails is recorded and left to the ordinary owed-work route (`State,
/// identity and recovery` §9.5 — no unit is retried inside its action).
pub fn apply_a_cancel(
    store: &mut TheQueueOnTheMachine,
    job: &Identity,
    where_it_sits: &WhereTheMaterialSits,
) -> Result<ACancelsOutcome, WhatTheStoreSaid> {
    store.remove(job)?;
    Ok(ACancelsOutcome {
        the_record_was_removed: true,
        cleanup: the_material_after_a_removal(where_it_sits),
    })
}

/// **L1's material cleanup, the four units in the enumeration's order** — run
/// after a record's removal and never before it, by a cancel, by L14's clear
/// for each job it removed, and by L22's drop.
///
/// **Partial across directories and atomic within each**: each unit is its
/// own removal, and a unit that fails is recorded and left to the ordinary
/// owed-work route (`State, identity and recovery` §9.5).
pub fn the_material_after_a_removal(where_it_sits: &WhereTheMaterialSits) -> Vec<AUnitsOutcome> {
    MaterialACancelRemoves::IN_ORDER
        .into_iter()
        .map(|unit| AUnitsOutcome {
            unit,
            refused: match (unit, where_it_sits.of(unit)) {
                // **Nothing stands to remove**, and a unit already absent is
                // done, never failed.
                (_, None) => None,
                (MaterialACancelRemoves::TheEmptyJobOutputFolder, Some(at)) => {
                    remove_the_folder_where_it_is_empty(at).err()
                }
                (_, Some(at)) => remove_whatever_is_there(at).err(),
            },
        })
        .collect()
}

/// **L1's fourth unit is *the empty job output folder*, and only an empty one
/// is removed. Added 2026-09-29, step 10c's first stop-and-clear piece.**
///
/// **Until then it went the way the other three go — whole, whatever it
/// held.** For a cancel of a job that never rendered that is the same thing.
/// **L14's clear is not that case**: it covers a record left reading
/// *rendering* by an interruption, and that record's folder can hold frames
/// the user rendered — which are the user's (`Rendering` K7), and which no
/// clause of this plan says a clear deletes. **So a folder with anything in it
/// is left, and the unit says why.**
fn remove_the_folder_where_it_is_empty(at: &Path) -> Result<(), String> {
    match std::fs::remove_dir(at) {
        Ok(()) => Ok(()),
        Err(why) if why.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(why) if at.is_dir() => Err(format!(
            "{}: the job output folder is not empty, so it is left ({why})",
            at.display()
        )),
        Err(why) => Err(format!("{}: {why}", at.display())),
    }
}

/// One unit removed, whatever shape it is.
///
/// **A unit already absent is *done*, never *failed***, and a directory is
/// removed whole — which is the *atomic within each* half of the declaration.
fn remove_whatever_is_there(at: &Path) -> Result<(), String> {
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

/// **L6, L7, L8 and L24, performed: the whole intended key assignment
/// recorded first, then the keys that actually move.**
///
/// **The record comes before the first key moves and is L24's**, because the
/// pass below writes back only the records whose key actually moved — a
/// refusal part-way would otherwise leave an order that is neither the one the
/// line had nor the one the user asked for, with nothing left to recompute it
/// from. **A refusal leaves the record standing**, and
/// [`crate::acting::reconciliation::finish_the_outstanding_acts`] finishes it
/// before the admission point may start anything (D14).
///
/// **The act is partial across records and atomic within each rewrite**, which
/// is §16's re-declared row: the block cannot be one filesystem write, because
/// `State, identity and recovery` §9.6 P8 settles that no single operation
/// updates two independent files atomically.
///
/// **Nothing here re-decides an order.** The keys are [`Rewrite::keys`]'s and
/// this only writes them.
///
/// **On a unix only, because it writes the act record**, which is a store on a
/// rented server — `State, identity and recovery` J11 — and a rented machine
/// is a Linux machine (that document's §1.3 rule 6).
#[cfg(unix)]
pub fn apply_a_rewrite(
    store: &mut TheQueueOnTheMachine,
    acts: &mut TheActRecordsOnTheMachine,
    act: &Identity,
    records: &[JobRecord],
    rewrite: &Rewrite,
) -> Result<(), WhatTheStoreSaid> {
    acts.record(
        act,
        &TheActsRecord::ARewrite {
            keys: rewrite
                .keys
                .iter()
                .map(|(job, key)| (job.as_text(), key.as_written()))
                .collect(),
        },
    )?;
    write_the_keys_that_move(store, records, &rewrite.keys)?;
    acts.cleared(act)
}

/// The pass both [`apply_a_rewrite`] and D14's completion run: each record
/// whose key actually moves, written as its own durable write.
///
/// **On a unix only**: both callers are.
#[cfg(unix)]
fn write_the_keys_that_move(
    store: &mut TheQueueOnTheMachine,
    records: &[JobRecord],
    keys: &[(Identity, crate::line::SequenceKey)],
) -> Result<(), WhatTheStoreSaid> {
    for (job, key) in keys {
        let Some(mut record) = records.iter().find(|record| &record.job == job).cloned() else {
            continue;
        };
        if record.sequence_key == Some(*key) {
            continue;
        }
        record.sequence_key = Some(*key);
        store.write(&record)?;
    }
    Ok(())
}

/// **L24's record finished: the intended assignment applied to the records as
/// they stand.** Read back from the record and never recomposed — the order
/// the user asked for is what the record holds, and nothing else can supply
/// it.
///
/// **On a unix only**: its only caller finishes an act record.
#[cfg(unix)]
pub(crate) fn finish_a_recorded_rewrite(
    store: &mut TheQueueOnTheMachine,
    records: &[JobRecord],
    keys: &[(String, u64)],
) -> Result<(), WhatTheStoreSaid> {
    let intended: Vec<(Identity, crate::line::SequenceKey)> = keys
        .iter()
        .filter_map(|(job, key)| {
            Identity::read_back(IdentityKind::Job, job)
                .map(|job| (job, crate::line::SequenceKey::read_back(*key)))
        })
        .collect();
    write_the_keys_that_move(store, records, &intended)
}

/// **L14, performed: one write removing every job record the clear covers.**
///
/// **What it covers is the rule of `stop_and_clear`, in
/// `smoozy-queue-history`** — every job record on that machine that is not the running one, waiting in the line and held
/// outside it alike. **The running one has one reading and only one: the
/// operation holding the exclusive render at the moment of this write**, which
/// is the same operation L13 resolves as the stop's subject and never the one
/// the desktop's reading last showed.
///
/// **L16: where the clear's write is refused, the caller does not attempt the
/// stop** — the refusal comes back and the pair's second half is recorded
/// *not attempted*, naming the clear.
///
/// **L23: the covered identities are written durably before the first removal,
/// each outcome as it becomes true, and the record is cleared only when the
/// last is gone.** That record is what shuts Row 59's window, in place of the
/// one write §16's table used to declare and `State, identity and recovery`
/// §9.6 P8 rules out. **A refusal part-way leaves the record standing rather
/// than swallowing it**, and a supervisor that comes back finishes the clear
/// from it before the admission point may start anything (D14) — which is the
/// case that matters most, a machine killed between two removals starting a
/// job the user asked to have cleared.
///
/// **On a unix only, because it writes the act record** — the same reason
/// [`apply_a_rewrite`] carries.
#[cfg(unix)]
pub fn apply_the_clear(
    store: &mut TheQueueOnTheMachine,
    acts: &mut TheActRecordsOnTheMachine,
    clear: &Identity,
    covered: &[Identity],
    after_each_removal: &mut dyn FnMut(&Identity),
) -> Result<(), TheClearWasRefused> {
    let mut standing: Vec<(String, Option<HowAnIdentityEnded>)> =
        covered.iter().map(|job| (job.as_text(), None)).collect();
    acts.record(
        clear,
        &TheActsRecord::AClear {
            covered: standing.clone(),
        },
    )
    .map_err(TheClearWasRefused::ItsRecordWasNotWritten)?;
    remove_each_covered_record(store, acts, clear, &mut standing, after_each_removal)
        .map_err(TheClearWasRefused::PartWay)?;
    let finished = TheActsRecord::AClear { covered: standing };
    if finished.every_identity_is_done() {
        acts.cleared(clear).map_err(TheClearWasRefused::PartWay)?;
    }
    Ok(())
}

/// **Why a clear did not end whole, and the two are not the same thing.
/// Added 2026-09-29, step 10c's first stop-and-clear piece.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheClearWasRefused {
    /// **L16's refused write**: L23's record could not be written, so nothing
    /// was removed and the clear never began. The paired stop is not
    /// attempted, naming the clear.
    ItsRecordWasNotWritten(WhatTheStoreSaid),
    /// **L23's refusal part-way**: the record stands with what happened on it,
    /// and D14 finishes it. **The clear is outstanding, not ended.**
    PartWay(WhatTheStoreSaid),
}

impl TheClearWasRefused {
    /// The store's own words.
    pub fn why(&self) -> String {
        match self {
            TheClearWasRefused::ItsRecordWasNotWritten(said) | TheClearWasRefused::PartWay(said) => {
                match said {
                    WhatTheStoreSaid::ItWasRefused { why } => why.clone(),
                    WhatTheStoreSaid::ItIsNotAJobRecord { at } => {
                        format!("{at}: not a record this store can read")
                    }
                }
            }
        }
    }
}

/// **L23's removals, and the one place an outcome is written as it becomes
/// true.** Shared with D14's completion, so a clear finished after a crash is
/// finished by the same code that began it.
///
/// **A refusal stops the pass**: the failing identity is recorded *failed*,
/// every identity not yet reached is recorded *not attempted* naming it, and
/// the refusal is handed back with the record still standing.
///
/// **`after_each_removal` is called for each record this pass removed, after
/// the removal and before its *done* is written** — which is where the
/// program journals the removal (L23's last bullet) and runs L1's material
/// cleanup. **Before the *done*, so a pass cut between the two repeats the
/// call rather than losing it**: the next pass finds the record already absent,
/// which is *done*, and calls again. Journalling a removal twice is safe — the
/// desktop removing a local record that is already gone removes nothing — and
/// never journalling one would leave that record on the desktop for good.
/// Added 2026-09-29, step 10c's first stop-and-clear piece.
///
/// **On a unix only, because it writes the act record** — the same reason
/// [`apply_a_rewrite`] carries.
#[cfg(unix)]
pub(crate) fn remove_each_covered_record(
    store: &mut TheQueueOnTheMachine,
    acts: &mut TheActRecordsOnTheMachine,
    clear: &Identity,
    standing: &mut [(String, Option<HowAnIdentityEnded>)],
    after_each_removal: &mut dyn FnMut(&Identity),
) -> Result<(), WhatTheStoreSaid> {
    for at in 0..standing.len() {
        if standing[at].1 == Some(HowAnIdentityEnded::Done) {
            continue;
        }
        let Some(job) = Identity::read_back(IdentityKind::Job, &standing[at].0) else {
            standing[at].1 = Some(HowAnIdentityEnded::Failed {
                why: "the record names an identity that does not read back".to_string(),
            });
            continue;
        };
        match store.remove(&job) {
            Ok(()) => {
                after_each_removal(&job);
                standing[at].1 = Some(HowAnIdentityEnded::Done);
            }
            Err(refused) => {
                let why = match &refused {
                    WhatTheStoreSaid::ItWasRefused { why } => why.clone(),
                    WhatTheStoreSaid::ItIsNotAJobRecord { at } => at.clone(),
                };
                standing[at].1 = Some(HowAnIdentityEnded::Failed { why });
                let stopped_by = standing[at].0.clone();
                for (_, outcome) in standing.iter_mut().skip(at + 1) {
                    if outcome.is_none() {
                        *outcome = Some(HowAnIdentityEnded::NotAttempted {
                            stopped_by: stopped_by.clone(),
                        });
                    }
                }
                // **The record stands, and it stands with what happened on
                // it.** Where even this write is refused the record still
                // stands, with the outcomes it had — outstanding either way.
                let _ = acts.record(
                    clear,
                    &TheActsRecord::AClear {
                        covered: standing.to_vec(),
                    },
                );
                return Err(refused);
            }
        }
        acts.record(
            clear,
            &TheActsRecord::AClear {
                covered: standing.to_vec(),
            },
        )?;
    }
    Ok(())
}

/// **Q2's two queue-state writes, named so the caller journals what the write
/// made true** — *written at the event that makes it true and journalled*.
/// The journal itself is `Rented-server runtime`'s store under its own single
/// writer and is not written here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AQueueStateWasWritten {
    ItStarted { job: Identity, operation: Identity },
    ItEnded { job: Identity, outcome: Identity },
}

/// **Q2, performed: the started record's state written with the render
/// operation's identity, in the same act the admission point names it
/// started.**
///
/// **Without it the record still reads *waiting* after the job has started**,
/// and three things quietly stop working: the admission point takes the same
/// job again on its next pass, the idle judgement reads the line as holding
/// startable work, and the clear covers a record whose worker is running,
/// which L14 says in as many words never happens.
pub fn the_record_started(
    store: &mut TheQueueOnTheMachine,
    job: &Identity,
    operation: &Identity,
) -> Result<AQueueStateWasWritten, WhatTheStoreSaid> {
    debug_assert_eq!(operation.kind(), IdentityKind::Operation);
    let mut record = store
        .one_record(job)
        .ok_or_else(|| WhatTheStoreSaid::ItIsNotAJobRecord { at: job.as_text() })?;
    record.state = crate::line::QueueState::Rendering {
        operation: operation.clone(),
    };
    store.write(&record)?;
    Ok(AQueueStateWasWritten::ItStarted {
        job: job.clone(),
        operation: operation.clone(),
    })
}

/// **Q2, performed: the ended record's state written with the recorded
/// outcome's identity, in the same act
/// [`crate::acting::note::ThePerJobNotesOnTheMachine::the_outcome_was_written`]
/// runs.**
pub fn the_record_ended(
    store: &mut TheQueueOnTheMachine,
    job: &Identity,
    outcome: &Identity,
) -> Result<AQueueStateWasWritten, WhatTheStoreSaid> {
    debug_assert_eq!(outcome.kind(), IdentityKind::Operation);
    let mut record = store
        .one_record(job)
        .ok_or_else(|| WhatTheStoreSaid::ItIsNotAJobRecord { at: job.as_text() })?;
    record.state = crate::line::QueueState::Ended {
        outcome: outcome.clone(),
    };
    store.write(&record)?;
    Ok(AQueueStateWasWritten::ItEnded {
        job: job.clone(),
        outcome: outcome.clone(),
    })
}

/// **L21, performed: the mark written on every covered live record, in the
/// same act in which the removal's recorded outcome is written.**
///
/// **The covered set is `Server Setup` B14's own predicate's, composed once
/// before anything is removed, and handed in.** **No clause here reads the
/// machine for a build**: no path is stat'd, run or asked its version.
///
/// **A mark whose write is refused is the disk-full condition and nothing
/// else**: the write refuses loudly, no mark stands, and that job takes L22's
/// last path — it starts, and `Rendering` L14's version confirmation fails the
/// attempt before the render instruction.
pub fn the_mark_riding_the_removals_outcome(
    store: &mut TheQueueOnTheMachine,
    records: &mut [JobRecord],
    covered: &[Identity],
    removal: &Identity,
    the_answered_version: &str,
) -> Result<Vec<Identity>, WhatTheStoreSaid> {
    crate::line::mark_the_removed_build(records, covered, removal, the_answered_version);
    let mut marked = Vec::new();
    for record in records.iter() {
        if record.removed_build.is_some() && covered.contains(&record.job) {
            store.write(record)?;
            marked.push(record.job.clone());
        }
    }
    Ok(marked)
}

/// **L22, performed: the drop at the admission point, in one serialized
/// act.**
///
/// **The whole act is supervisor-local, with no connection and no Smoozy in
/// it.** Each marked record it reaches is removed, the drop is journalled by
/// the caller from what comes back, and the material cleanup follows per L1's
/// enumeration — **until a startable record or an empty line.**
///
/// **Nothing is written to history and no tombstone is written**, which is
/// [`crate::line::A_DROPPED_JOB_LEAVES_NO_HISTORY_ENTRY_AND_NO_TOMBSTONE`].
pub fn the_admission_point_performed(
    store: &mut TheQueueOnTheMachine,
    records: &[JobRecord],
    a_bounded_label_for: impl Fn(&Identity) -> String,
    where_each_jobs_material_sits: impl Fn(&Identity) -> WhereTheMaterialSits,
) -> Result<crate::line::AtTheAdmissionPoint, WhatTheStoreSaid> {
    let at = crate::line::the_admission_point(records, a_bounded_label_for);
    for dropped in &at.dropped {
        store.remove(&dropped.job)?;
        let _ = the_material_after_a_removal(&where_each_jobs_material_sits(&dropped.job));
    }
    Ok(at)
}

/// **L12, performed: a waiter with a deadline of its own, and expiry is the
/// waiter giving up.**
///
/// **Observed 1–3 seconds on the virtual machine and 1.9 to 14 on the
/// container**, so nothing here waits without a bound. The deadline is the
/// caller's — `Server protocol and transfers` C16 and C17's per-waiter
/// deadline — and **expiry is never read as the stop having failed or
/// succeeded**: [`AWaitersEnd::TheWaiterGaveUp`] carries no statement about
/// the stop at all.
///
/// **Removing the deadline fails rather than hangs**: the outcome is asked for
/// once per interval and the loop ends at the deadline whether or not one has
/// arrived.
pub fn wait_for_the_stops_outcome(
    deadline: Duration,
    between_two_asks: Duration,
    now: impl Fn() -> std::time::Instant,
    mut ask: impl FnMut() -> Option<WhatTheStopLeft>,
    mut rest: impl FnMut(Duration),
) -> AWaitersEnd {
    let opened = now();
    loop {
        if let Some(left) = ask() {
            return AWaitersEnd::TheOutcomeWasLearned(left);
        }
        if now().duration_since(opened) >= deadline {
            return AWaitersEnd::TheWaiterGaveUp;
        }
        rest(between_two_asks);
    }
}

/// **L2's own rule, stated as a value: a cancel is reported done only when the
/// machine's copy is gone**, which is why the removal above is what the
/// desktop's local half waits on.
pub const THE_MACHINES_COPY_GOES_FIRST: bool = true;

/// The kind an identity issued for one of these commands carries.
pub const A_COMMAND_IDENTITY_IS_AN_OPERATION_IDENTITY: IdentityKind = IdentityKind::Operation;

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use crate::line::Position;
    use crate::line::{QueueState, SequenceKey};
    use std::time::Instant;

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    fn an_operation() -> Identity {
        Identity::issue(IdentityKind::Operation).expect("randomness")
    }

    fn a_root(named: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "smoozy-queue-commands-{}-{named}",
            std::process::id()
        ));
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(&root).expect("the place is made");
        root
    }

    fn a_record(job: &Identity, key: u64) -> JobRecord {
        let mut record = JobRecord::submitted(
            job.clone(),
            None,
            format!("/ephemeral/smoozy/job-records/{}", job.as_text()),
            &[],
        );
        record.sequence_key = Some(SequenceKey::read_back(key));
        record
    }

    fn material(root: &Path, job: &Identity) -> WhereTheMaterialSits {
        WhereTheMaterialSits {
            job_record_directory: root.join(format!("records/{}", job.as_text())),
            remap_material: root.join(format!("remaps/{}", job.as_text())),
            per_job_note: root.join(format!("{}.note.json", job.as_text())),
            job_output_folder: Some(root.join(format!("outputs/{}", job.as_text()))),
            reporting_choice_now: None,
        }
    }

    fn lay_the_material_out(where_it_sits: &WhereTheMaterialSits) {
        std::fs::create_dir_all(&where_it_sits.job_record_directory).unwrap();
        std::fs::write(
            where_it_sits.job_record_directory.join("manifest.json"),
            b"{}",
        )
        .unwrap();
        std::fs::create_dir_all(&where_it_sits.remap_material).unwrap();
        std::fs::write(&where_it_sits.per_job_note, b"{}").unwrap();
        std::fs::create_dir_all(where_it_sits.job_output_folder.as_ref().unwrap()).unwrap();
    }

    /// **L1: the record's removal is the deciding write, and the material
    /// cleanup follows it** — all four units, in the enumeration's order.
    #[test]
    fn a_cancel_removes_the_record_first_and_then_all_four_units() {
        let root = a_root("cancel");
        let mut store = TheQueueOnTheMachine::at(&root);
        let job = a_job();
        store.write(&a_record(&job, 1)).expect("written");
        let where_it_sits = material(&root, &job);
        lay_the_material_out(&where_it_sits);

        let outcome = apply_a_cancel(&mut store, &job, &where_it_sits).expect("cancelled");
        assert!(outcome.the_record_was_removed);
        assert_eq!(outcome.cleanup.len(), 4);
        assert!(outcome.cleanup.iter().all(|unit| unit.refused.is_none()));
        assert_eq!(store.one_record(&job), None, "the job cannot start");
        assert!(!where_it_sits.job_record_directory.exists());
        assert!(!where_it_sits.remap_material.exists());
        assert!(!where_it_sits.per_job_note.exists());
        assert!(!where_it_sits.job_output_folder.as_ref().unwrap().exists());
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A unit already absent is *done*, never *failed*** — the cleanup must
    /// be able to run against a machine where earlier units already went.
    #[test]
    fn a_cancel_over_material_that_is_already_gone_reports_every_unit_done() {
        let root = a_root("absent");
        let mut store = TheQueueOnTheMachine::at(&root);
        let job = a_job();
        store.write(&a_record(&job, 1)).expect("written");
        let outcome = apply_a_cancel(&mut store, &job, &material(&root, &job)).expect("cancelled");
        assert!(outcome.cleanup.iter().all(|unit| unit.refused.is_none()));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **L1's order: a refused removal leaves a whole job, material and
    /// all.** Cleaning the material first and then failing to remove the
    /// record would leave a startable job whose material is gone, which is
    /// exactly what this order prevents.
    ///
    /// **The refusal is forced the one way every system agrees on** —
    /// [`crate::acting::queue_store::a_store_whose_record_cannot_be_removed`],
    /// which says why.
    #[test]
    fn a_refused_removal_touches_no_material_at_all() {
        let root = a_root("refused");
        let job = a_job();
        let where_it_sits = material(&root, &job);
        lay_the_material_out(&where_it_sits);
        let mut store =
            crate::acting::queue_store::a_store_whose_record_cannot_be_removed(&root, &job);
        let refused = apply_a_cancel(&mut store, &job, &where_it_sits);
        assert!(refused.is_err(), "the removal refused");
        assert!(
            where_it_sits.job_record_directory.exists(),
            "and nothing of the material was touched"
        );
        assert!(where_it_sits.per_job_note.exists());
        std::fs::remove_dir_all(&root).ok();
    }

    /// **L6 and L8: the rewrite is written back in one pass and the line
    /// reads in the new order.**
    #[cfg(unix)]
    #[test]
    fn a_reorders_rewrite_is_written_back_and_the_line_reads_in_the_new_order() {
        let root = a_root("reorder");
        let mut store = TheQueueOnTheMachine::at(&root);
        let first = a_job();
        let second = a_job();
        let third = a_job();
        for (job, key) in [(&first, 10), (&second, 20), (&third, 30)] {
            store.write(&a_record(job, key)).expect("written");
        }
        let (records, _) = store.every_record().expect("read");
        let rewrite = crate::line::reorder(&records, &third, &Position::AtTheFrontOfTheWaitingLine);
        let mut acts = TheActRecordsOnTheMachine::at(&root);
        let act = an_operation();
        apply_a_rewrite(&mut store, &mut acts, &act, &records, &rewrite).expect("written");
        assert_eq!(
            acts.read(&act).expect("read"),
            None,
            "the record is cleared when the last moved key is written"
        );
        let (records, _) = store.every_record().expect("read");
        let line: Vec<Identity> = crate::line::the_line(&records)
            .into_iter()
            .map(|record| record.job.clone())
            .collect();
        assert_eq!(line, vec![third, first, second]);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **L14: the clear removes every covered record and leaves the running
    /// one**, which is the record the caller kept out of the covered set.
    #[cfg(unix)]
    #[test]
    fn the_clear_removes_what_it_covers_and_leaves_the_running_one() {
        let root = a_root("clear");
        let mut store = TheQueueOnTheMachine::at(&root);
        let waiting = a_job();
        let held = a_job();
        let running = a_job();
        store.write(&a_record(&waiting, 10)).expect("written");
        let mut held_record = a_record(&held, 20);
        held_record.state = QueueState::Held {
            hold: an_operation(),
        };
        held_record.sequence_key = None;
        store.write(&held_record).expect("written");
        let mut running_record = a_record(&running, 5);
        running_record.state = QueueState::Rendering {
            operation: an_operation(),
        };
        store.write(&running_record).expect("written");

        let mut acts = TheActRecordsOnTheMachine::at(&root);
        let clear = an_operation();
        let mut told = Vec::new();
        apply_the_clear(
            &mut store,
            &mut acts,
            &clear,
            &[waiting.clone(), held.clone()],
            &mut |job| told.push(job.clone()),
        )
        .expect("cleared");
        assert_eq!(
            told,
            vec![waiting.clone(), held.clone()],
            "L23's last bullet: each removal is handed back to be journalled"
        );
        assert_eq!(
            acts.read(&clear).expect("read"),
            None,
            "L23: the record is cleared when the last covered record is gone"
        );
        let (records, _) = store.every_record().expect("read");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].job, running);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **L23: the covered identities are written durably before the first
    /// removal, and a refusal part-way leaves the record standing rather than
    /// swallowing it.**
    ///
    /// **The fault is introduced where a full mount puts it**: the store's own
    /// directory is a plain file, so every removal in it refuses. The act
    /// record is kept in a directory that still works, which is what a real
    /// machine's package area gives — and what the test needs, because a
    /// record that could not be written at all would prove nothing about the
    /// record standing.
    #[cfg(unix)]
    #[test]
    fn a_clear_refused_part_way_leaves_its_record_standing_with_what_happened_on_it() {
        let root = a_root("clear-refused");
        std::fs::write(root.join("gone"), b"not a directory").unwrap();
        let mut store = TheQueueOnTheMachine::at(root.join("gone"));
        let mut acts = TheActRecordsOnTheMachine::at(&root);
        let clear = an_operation();
        let first = a_job();
        let second = a_job();
        let refused = apply_the_clear(
            &mut store,
            &mut acts,
            &clear,
            &[first.clone(), second.clone()],
            &mut |_| panic!("nothing was removed, so nothing is journalled"),
        );
        assert!(
            matches!(refused, Err(TheClearWasRefused::PartWay(_))),
            "the removal refused: {refused:?}"
        );
        let standing = acts.read(&clear).expect("read").expect("the record stands");
        let TheActsRecord::AClear { covered } = standing else {
            panic!("a clear's record")
        };
        assert_eq!(covered[0].0, first.as_text());
        assert!(matches!(
            covered[0].1,
            Some(HowAnIdentityEnded::Failed { .. })
        ));
        assert_eq!(
            covered[1].1,
            Some(HowAnIdentityEnded::NotAttempted {
                stopped_by: first.as_text()
            }),
            "a unit never tried names what stopped it"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **L24: the whole intended key assignment is written before the first
    /// key moves**, because the pass writes back only the keys that actually
    /// move and a refusal would otherwise leave an order nothing could
    /// recompute.
    #[cfg(unix)]
    #[test]
    fn a_rewrites_record_holds_the_whole_intended_assignment_before_a_single_key_moves() {
        let root = a_root("rewrite-record");
        let mut store = TheQueueOnTheMachine::at(&root);
        let mut acts = TheActRecordsOnTheMachine::at(&root);
        let first = a_job();
        let second = a_job();
        for (job, key) in [(&first, 10), (&second, 20)] {
            store.write(&a_record(job, key)).expect("written");
        }
        let (records, _) = store.every_record().expect("read");
        let rewrite =
            crate::line::reorder(&records, &second, &Position::AtTheFrontOfTheWaitingLine);
        let act = an_operation();
        // The record is written first, so it holds the assignment even for the
        // records whose key does not move.
        acts.record(
            &act,
            &TheActsRecord::ARewrite {
                keys: rewrite
                    .keys
                    .iter()
                    .map(|(job, key)| (job.as_text(), key.as_written()))
                    .collect(),
            },
        )
        .expect("recorded");
        let TheActsRecord::ARewrite { keys } = acts.read(&act).expect("read").expect("there")
        else {
            panic!("a rewrite's record")
        };
        assert_eq!(
            keys.len(),
            2,
            "every waiting record's key, not only the moved"
        );
        assert_eq!(keys[0].0, second.as_text());
        apply_a_rewrite(&mut store, &mut acts, &act, &records, &rewrite).expect("written");
        assert_eq!(acts.read(&act).expect("read"), None);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **Q2: the two queue-state writes, each in the act that makes its fact
    /// true, and each naming what is journalled.**
    ///
    /// **Without the first the record still reads *waiting* after the job has
    /// started**, so the admission point takes it again, the idle judgement
    /// reads the line as holding startable work, and the clear covers a record
    /// whose worker is running.
    #[test]
    fn the_two_queue_states_are_written_and_each_names_what_is_journalled() {
        let root = a_root("states");
        let mut store = TheQueueOnTheMachine::at(&root);
        let job = a_job();
        store.write(&a_record(&job, 1)).expect("written");
        let operation = an_operation();
        assert_eq!(
            the_record_started(&mut store, &job, &operation).expect("written"),
            AQueueStateWasWritten::ItStarted {
                job: job.clone(),
                operation: operation.clone()
            }
        );
        let back = store.one_record(&job).expect("there");
        assert_eq!(
            back.state,
            QueueState::Rendering {
                operation: operation.clone()
            }
        );
        // **The admission point does not take it again**, and the line no
        // longer holds startable work.
        let (records, _) = store.every_record().expect("read");
        assert_eq!(crate::line::the_line(&records).len(), 0);

        let outcome = an_operation();
        assert_eq!(
            the_record_ended(&mut store, &job, &outcome).expect("written"),
            AQueueStateWasWritten::ItEnded {
                job: job.clone(),
                outcome: outcome.clone()
            }
        );
        assert_eq!(
            store.one_record(&job).expect("there").state,
            QueueState::Ended { outcome }
        );
        // **A record that is not there is not a state to write**, and the
        // refusal says so rather than writing one.
        assert!(the_record_started(&mut store, &a_job(), &an_operation()).is_err());
        std::fs::remove_dir_all(&root).ok();
    }

    /// **L21 and L22: the mark is written on the covered live records, and the
    /// admission point drops each marked record it reaches and starts the
    /// first startable one.**
    #[test]
    fn the_mark_is_written_and_the_admission_point_drops_every_marked_record() {
        let root = a_root("mark");
        let mut store = TheQueueOnTheMachine::at(&root);
        let marked_one = a_job();
        let marked_two = a_job();
        let startable = a_job();
        for (job, key) in [(&marked_one, 10), (&marked_two, 20), (&startable, 30)] {
            store.write(&a_record(job, key)).expect("written");
        }
        let (mut records, _) = store.every_record().expect("read");
        let removal = an_operation();
        let written = the_mark_riding_the_removals_outcome(
            &mut store,
            &mut records,
            &[marked_one.clone(), marked_two.clone()],
            &removal,
            "4.5.3",
        )
        .expect("written");
        assert_eq!(written.len(), 2);

        let (records, _) = store.every_record().expect("read");
        let at = the_admission_point_performed(
            &mut store,
            &records,
            |_| "a label".to_string(),
            |job| material(&root, job),
        )
        .expect("applied");
        assert_eq!(at.started, Some(startable.clone()));
        assert_eq!(at.dropped.len(), 2);
        assert!(at
            .dropped
            .iter()
            .all(|dropped| dropped.the_answered_version_the_job_was_submitted_with == "4.5.3"));
        // **The dropped records are gone from the line in the same act.**
        let (records, _) = store.every_record().expect("read");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].job, startable);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **L23, part-way past the first removal: the first record goes and is
    /// handed back to be journalled, the second's refusal stops the pass, and
    /// D14's finish takes the rest once the store will remove again — handing
    /// back only what it removed.** Added 2026-09-29, step 10c's first stop-and-clear piece.
    ///
    /// **The second record's removal is refused the way every system agrees
    /// on**: its name is taken by a directory with something in it
    /// ([`crate::acting::queue_store::a_store_whose_record_cannot_be_removed`]).
    #[cfg(unix)]
    #[test]
    fn a_clear_refused_at_its_second_removal_hands_back_the_first_and_is_finished_later() {
        let root = a_root("clear-second");
        let first = a_job();
        let second = a_job();
        let mut store =
            crate::acting::queue_store::a_store_whose_record_cannot_be_removed(&root, &second);
        store.write(&a_record(&first, 10)).expect("written");
        let mut acts = TheActRecordsOnTheMachine::at(&root);
        let clear = an_operation();
        let mut told = Vec::new();
        let refused = apply_the_clear(
            &mut store,
            &mut acts,
            &clear,
            &[first.clone(), second.clone()],
            &mut |job| told.push(job.clone()),
        );
        assert!(matches!(refused, Err(TheClearWasRefused::PartWay(_))));
        assert_eq!(told, vec![first.clone()]);
        let TheActsRecord::AClear { covered } =
            acts.read(&clear).expect("read").expect("the record stands")
        else {
            panic!("a clear's record")
        };
        assert_eq!(covered[0].1, Some(HowAnIdentityEnded::Done));
        assert!(matches!(covered[1].1, Some(HowAnIdentityEnded::Failed { .. })));

        // **The store removes again, and D14 finishes the clear.**
        let in_the_way = root
            .join("store")
            .join(format!("{}{}", second.as_text(), crate::acting::queue_store::THE_RECORDS_SUFFIX));
        std::fs::remove_dir_all(&in_the_way).expect("the way is cleared");
        store.write(&a_record(&second, 20)).expect("written");
        let mut finished_now = Vec::new();
        let (_, finished) = crate::acting::reconciliation::finish_the_outstanding_acts(
            &mut store,
            &mut acts,
            &mut |job| finished_now.push(job.clone()),
        )
        .expect("finished");
        assert_eq!(finished_now, vec![second.clone()], "only what this pass removed");
        assert_eq!(finished, vec![clear.clone()]);
        assert_eq!(acts.read(&clear).expect("read"), None);
        assert!(store.every_record().expect("read").0.is_empty());
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A job whose folder was never chosen** — `Job files and outputs` P21's
    /// render-start case, cancelled before its render's step 4 — **has no
    /// folder to remove, and that unit is done**, never failed; the other
    /// three go as they always did. The Creator's second notes, piece 4.
    #[test]
    fn a_job_whose_folder_was_never_chosen_has_its_folder_unit_done() {
        let root = a_root("never-chosen");
        let job = a_job();
        let mut where_it_sits = material(&root, &job);
        lay_the_material_out(&where_it_sits);
        where_it_sits.job_output_folder = None;
        let cleanup = the_material_after_a_removal(&where_it_sits);
        assert!(cleanup.iter().all(|unit| unit.refused.is_none()), "{cleanup:?}");
        assert!(!where_it_sits.job_record_directory.exists());
        std::fs::remove_dir_all(&root).ok();
    }

    /// **L1's fourth unit is the *empty* job output folder**: a folder holding
    /// frames is left and the unit says why, while the other three go whole.
    /// Added 2026-09-29, step 10c's first stop-and-clear piece — a record left *rendering* by an
    /// interruption is one L14's clear covers, and its frames are the user's.
    #[test]
    fn a_job_output_folder_with_frames_in_it_is_left_by_the_cleanup() {
        let root = a_root("frames");
        let job = a_job();
        let where_it_sits = material(&root, &job);
        lay_the_material_out(&where_it_sits);
        std::fs::write(where_it_sits.job_output_folder.as_ref().unwrap().join("0001.png"), b"a frame").unwrap();
        let cleanup = the_material_after_a_removal(&where_it_sits);
        let folder = cleanup
            .iter()
            .find(|unit| unit.unit == MaterialACancelRemoves::TheEmptyJobOutputFolder)
            .expect("the unit is there");
        assert!(folder.refused.as_deref().is_some_and(|why| why.contains("not empty")));
        assert!(where_it_sits.job_output_folder.as_ref().unwrap().join("0001.png").exists());
        assert!(!where_it_sits.job_record_directory.exists());
        assert!(!where_it_sits.per_job_note.exists());
        std::fs::remove_dir_all(&root).ok();
    }

    /// **L12: the waiter has a deadline of its own, and its expiry says
    /// nothing about the stop.**
    ///
    /// **Removing the deadline fails rather than hangs**, which is what the
    /// second half of this test proves: with the deadline at zero the waiter
    /// gives up on its first pass rather than resting for ever.
    #[test]
    fn the_waiter_gives_up_at_its_own_deadline_and_concludes_nothing() {
        let ticks = std::cell::Cell::new(0u64);
        let opened = Instant::now();
        let now = || opened + Duration::from_secs(1);
        let end = wait_for_the_stops_outcome(
            Duration::from_secs(20),
            Duration::from_millis(1),
            // A clock that never advances past the deadline would hang, so it
            // moves with each ask and the outcome arrives on the fourth.
            || opened + Duration::from_millis(ticks.get()),
            || {
                ticks.set(ticks.get() + 1);
                (ticks.get() >= 4).then_some(WhatTheStopLeft::LeftStopped)
            },
            |_| {},
        );
        assert_eq!(
            end,
            AWaitersEnd::TheOutcomeWasLearned(WhatTheStopLeft::LeftStopped)
        );

        let mut asks = 0;
        let end = wait_for_the_stops_outcome(
            Duration::from_secs(0),
            Duration::from_millis(1),
            now,
            || {
                asks += 1;
                None
            },
            |_| panic!("a waiter at its deadline never rests again"),
        );
        assert_eq!(end, AWaitersEnd::TheWaiterGaveUp);
        assert_eq!(asks, 1);
    }

    /// **The travel is one request of the endpoint's own framing, and the
    /// name is a constant per command.**
    #[test]
    fn every_command_travels_as_one_named_request_and_no_name_is_composed() {
        let identity = an_operation();
        let job = a_job();
        for command in [
            AQueueCommand::CancelAWaitingJob,
            AQueueCommand::Reorder,
            AQueueCommand::MoveAGroupAsOneBlock,
            AQueueCommand::Clear,
            AQueueCommand::TheStopPairedWithAClear,
            AQueueCommand::AnswerAHold,
        ] {
            let request = the_request_for(
                command,
                identity.clone(),
                Fields::none().and("job", job.as_text().into_bytes()),
            );
            assert_eq!(request.name, command.name());
            assert!(
                !request.name.contains('/'),
                "no path is composed into a name"
            );
            assert_eq!(request.fields.text("job"), Some(job.as_text().as_str()));
        }
        const _: () = assert!(THE_MACHINES_COPY_GOES_FIRST);
        assert_eq!(
            A_COMMAND_IDENTITY_IS_AN_OPERATION_IDENTITY,
            IdentityKind::Operation
        );
    }
}

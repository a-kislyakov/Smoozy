//! **§1 performed — the machine-local queue as one durable store in the
//! package area, written only by the supervisor, with Q8's disk-full answer
//! kept rather than worked around.**
//!
//! # Q1, and the three things that make it a rule rather than a hope
//!
//! **One durable store, in the package area, holding one job record per job,
//! written only by the supervisor.** That is `State, identity and recovery`
//! §1.2's own inventory row and `Job files and outputs` L6's placement —
//! **outside every job folder, so nothing the user browses, uploads,
//! downloads or renders reaches it and no job cleanup removes it.**
//!
//! **The single writer is what everything below rests on** (`Rented-server
//! runtime` S1). [`TheQueueOnTheMachine`] is held by the supervisor and there
//! is no second way to open one: nothing in this module reads a record and
//! writes it back from anywhere else, so the race L14 names is closed by
//! construction and not by a lock.
//!
//! # One file per record, and why not one file for the line
//!
//! **Each job record is its own file, named by the job identity, published by
//! a write into a staged name and one rename.** Three clauses force it and no
//! preference is involved.
//!
//! - **Q3 makes the record's existence the fact**: a job is in the line
//!   exactly when its job record exists with state *waiting* or *rendering*.
//!   A single file holding the whole line would make *the record exists* a
//!   property of a parse rather than of the filesystem, and a half-written
//!   line would take every job with it.
//! - **Q6 puts untracked records in the same store and the same line**, so
//!   the store has to hold records the desktop never named — which a
//!   per-record file does and a desktop-driven index does not.
//! - **`State, identity and recovery` §1.3 rule 5 forbids reading a file's
//!   size as state**, and nothing here does: the answer to *is this job in
//!   the line* is the record's own state field, read out of a file that was
//!   published whole.
//!
//! **The staged name and the rename are `Job files and outputs` M5's own
//! shape**, taken because a record read while it was being written would be a
//! line with a job missing from it.
//!
//! # Q8's disk-full answer, kept exactly
//!
//! **Every write is refused loudly with the machine's own words and nothing
//! moves behind it.** There is no retry, no partial write, no dropped field
//! and no in-memory line that carries on as though the write had landed:
//! [`WhatTheStoreSaid::ItWasRefused`] carries the refusal verbatim and the
//! caller's act does not happen.
//!
//! **What is not here, and it is a clause rather than an omission.** *"The
//! repair is `Server Setup`'s and `Desktop app and delivery`'s automatic
//! supervisor-unavailable trigger, and nothing in this document is that
//! trigger."* So there is no free-space check before a write, no reserve, no
//! sweep and no schedule in this module — **a check before the write would be
//! a second answer to a question the write already answers, and a wrong one
//! the moment the space goes between the two.**
//!
//! **A supervisor already running when the mount fills goes on answering**,
//! every durable write refusing loudly, and what it could not write about an
//! ending is held pending and re-attempted until it lands under
//! `Rented-server runtime` S14 — **which is that document's machinery and not
//! a second one here.**
//!
//! # What is deliberately not decided here
//!
//! **The line's order, the admission point, every rewrite and the released
//! placement are [`crate::line`]'s**, computed as pure functions over the
//! records as they stand. This module reads the records, hands them to those
//! functions, and writes back what they answer. **It decides nothing.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` S1, S14. `Job files and
//! outputs` M5. `State, identity and recovery` §1.2, §1.3.
//!
//! **`Job files and outputs` L6 is deliberately not on that list and is
//! written out at every site instead**, because this plan defines an L6 of
//! its own.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::line::{JobRecord, QueueState, RemovedBuildMark, SequenceKey};

/// **The record's written form, and it is Build's own choice.**
///
/// **The rules half's [`JobRecord`] is untouched by it**: that type is the
/// shape every judgement in this crate is written over, and giving it a
/// written form of its own would put a storage concern inside the one place
/// the line's rules live. So the form is here, beside the store that uses
/// it, in the shape `Job files and outputs` already settled for the manifest
/// and the attempt record — one JSON document per record.
///
/// **Every identity is written as `State, identity and recovery`'s own text
/// with the kind beside it**, because that document's `read_back` takes the
/// kind as an argument: an identity read back under the wrong kind is a
/// comparison that can never match, and this is what makes reading one back
/// under the wrong kind impossible rather than merely unlikely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TheRecordAsWritten {
    job: String,
    group: Option<String>,
    sequence_key: Option<u64>,
    state: TheStateAsWritten,
    record_directory: String,
    removed_build: Option<TheMarkAsWritten>,
    /// **`Telegram` V1c's current choice**, absent on every record written
    /// before 2026-10-30 and on every job never changed — so an older record
    /// reads back as it always did, and an older program reading a newer
    /// record ignores the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reporting_choice_now: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum TheStateAsWritten {
    Waiting,
    Held { hold: String },
    Rendering { operation: String },
    Ended { outcome: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TheMarkAsWritten {
    /// **L21's mark holds the removal operation's own identity**, which is
    /// an operation identity like every other.
    removal: String,
    the_answered_version_the_job_was_submitted_with: String,
}

impl TheRecordAsWritten {
    fn of(record: &JobRecord) -> TheRecordAsWritten {
        TheRecordAsWritten {
            job: record.job.as_text(),
            group: record.group.as_ref().map(Identity::as_text),
            sequence_key: record.sequence_key.map(SequenceKey::as_written),
            state: match &record.state {
                QueueState::Waiting => TheStateAsWritten::Waiting,
                QueueState::Held { hold } => TheStateAsWritten::Held {
                    hold: hold.as_text(),
                },
                QueueState::Rendering { operation } => TheStateAsWritten::Rendering {
                    operation: operation.as_text(),
                },
                QueueState::Ended { outcome } => TheStateAsWritten::Ended {
                    outcome: outcome.as_text(),
                },
            },
            record_directory: record.record_directory.clone(),
            removed_build: record.removed_build.as_ref().map(|mark| TheMarkAsWritten {
                removal: mark.removal.as_text(),
                the_answered_version_the_job_was_submitted_with: mark
                    .the_answered_version_the_job_was_submitted_with
                    .clone(),
            }),
            reporting_choice_now: record.reporting_choice_now.clone(),
        }
    }

    /// **A record that does not read back whole is not a record.** Every
    /// identity has to read back under its own kind, and one that does not is
    /// `None` — never a record with a field guessed at.
    fn read_back(self) -> Option<JobRecord> {
        Some(JobRecord {
            job: Identity::read_back(IdentityKind::Job, &self.job)?,
            group: match self.group {
                Some(text) => Some(Identity::read_back(IdentityKind::Group, &text)?),
                None => None,
            },
            sequence_key: self.sequence_key.map(SequenceKey::read_back),
            state: match self.state {
                TheStateAsWritten::Waiting => QueueState::Waiting,
                // **The hold identity is the supervisor's own operation
                // identity** (`Rented-server runtime` U2), which is the kind
                // that crate's own hold record carries.
                TheStateAsWritten::Held { hold } => QueueState::Held {
                    hold: Identity::read_back(IdentityKind::Operation, &hold)?,
                },
                TheStateAsWritten::Rendering { operation } => QueueState::Rendering {
                    operation: Identity::read_back(IdentityKind::Operation, &operation)?,
                },
                TheStateAsWritten::Ended { outcome } => QueueState::Ended {
                    outcome: Identity::read_back(IdentityKind::Operation, &outcome)?,
                },
            },
            record_directory: self.record_directory,
            removed_build: match self.removed_build {
                Some(mark) => Some(RemovedBuildMark {
                    removal: Identity::read_back(IdentityKind::Operation, &mark.removal)?,
                    the_answered_version_the_job_was_submitted_with: mark
                        .the_answered_version_the_job_was_submitted_with,
                }),
                None => None,
            },
            reporting_choice_now: self.reporting_choice_now,
        })
    }
}

/// What the store said about one act.
///
/// **Two answers and they are not the same kind of thing**: what the machine
/// refused, carried in the machine's own words, and what this module read as
/// not a record. Neither is ever turned into an empty line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheStoreSaid {
    /// **Q8: the write was refused, loudly, and nothing moved behind it.**
    /// The machine's own words, carried verbatim — on a full state mount they
    /// are the loud errno 28 ones, uniformly.
    ItWasRefused { why: String },
    /// A file in the store's directory that is not a job record. **It is not
    /// read as an absent job and it is not removed**: `Server protocol and
    /// transfers` S28's rule against acting on the strength of no record
    /// naming a path binds here too.
    ItIsNotAJobRecord { at: String },
}

/// **Q1's store: one directory in the package area, one file per job
/// record.**
///
/// **Held by the supervisor and by nothing else** — there is no second
/// constructor, and every method takes `&mut self`, so two writers cannot
/// exist inside one process either.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheQueueOnTheMachine {
    at: PathBuf,
}

/// What a staged record's name is prefixed with while it is being written.
///
/// **A prefix rather than a separate directory**, so the rename that
/// publishes it is within one directory and is the plain rename `Job files
/// and outputs` M5 already uses.
pub const THE_STAGING_PREFIX: &str = ".staging-";

/// The suffix every published record carries.
pub const THE_RECORDS_SUFFIX: &str = ".json";

/// **Where the machine-local queue sits under the package area**, and the name
/// is Build's own. **Q1 puts the store in the package area and names no
/// segment**, the way `Rented-server runtime` names its own areas there
/// (`hold-records`, `operation-records`, `event-journal`). Added at step 10c
/// piece 5, when the supervisor first made the store.
pub const THE_AREA_THE_QUEUE_SITS_IN: &str = "machine-local-queue";

impl TheQueueOnTheMachine {
    /// The store, at the directory the package area's own layout names.
    ///
    /// **The directory is not created here.** The package area's five areas
    /// exist before the first job (`Job files and outputs` L6), and a store
    /// that made its own directory would make a missing package area look
    /// like an empty line.
    pub fn at(directory: impl Into<PathBuf>) -> TheQueueOnTheMachine {
        TheQueueOnTheMachine {
            at: directory.into(),
        }
    }

    fn published(&self, job: &Identity) -> PathBuf {
        self.at
            .join(format!("{}{THE_RECORDS_SUFFIX}", job.as_text()))
    }

    /// **Where one job's record sits**, published — for `Queue & History`
    /// X2's erase, which names each unit by its path. The 38's piece 16.
    pub fn where_one_sits(&self, job: &Identity) -> PathBuf {
        self.published(job)
    }

    fn staged(&self, job: &Identity) -> PathBuf {
        self.at.join(format!(
            "{THE_STAGING_PREFIX}{}{THE_RECORDS_SUFFIX}",
            job.as_text()
        ))
    }

    /// **Q3: the job record, written by the supervisor after the job's
    /// material is published.**
    ///
    /// **Written into a staged name, made durable, then published by one
    /// rename.** A reader that meets the staged name meets a name no reader
    /// looks for; a reader that meets the published name meets a whole
    /// record.
    ///
    /// **Q8: a refusal at any of the three ends the act, loudly, and nothing
    /// moves behind it.** The staged file a refused publication leaves is
    /// removed where it can be — and where even that is refused it is left,
    /// because a store that cannot be written to is not a store that can be
    /// tidied.
    pub fn write(&mut self, record: &JobRecord) -> Result<(), WhatTheStoreSaid> {
        let staged = self.staged(&record.job);
        let published = self.published(&record.job);
        let written = serde_json::to_vec(&TheRecordAsWritten::of(record)).map_err(|why| {
            WhatTheStoreSaid::ItWasRefused {
                why: why.to_string(),
            }
        })?;
        write_durably(&staged, &written).map_err(|why| WhatTheStoreSaid::ItWasRefused { why })?;
        if let Err(why) = std::fs::rename(&staged, &published) {
            let _ = std::fs::remove_file(&staged);
            return Err(WhatTheStoreSaid::ItWasRefused {
                why: format!("{}: {why}", published.display()),
            });
        }
        Ok(())
    }

    /// **Every record in the store, tracked and untracked alike** (Q6).
    ///
    /// **A file that is not a job record is reported and never read as an
    /// absent job**, and the staged names are skipped: a staged file is a
    /// write in flight and reading one would be reading a record that is not
    /// yet a record.
    ///
    /// **The order the filesystem hands them back means nothing** — the line
    /// is [`crate::line::the_line`]'s, in ascending sequence-key order, and
    /// this answer is a set.
    pub fn every_record(
        &self,
    ) -> Result<(Vec<JobRecord>, Vec<WhatTheStoreSaid>), WhatTheStoreSaid> {
        let entries =
            std::fs::read_dir(&self.at).map_err(|why| WhatTheStoreSaid::ItWasRefused {
                why: format!("{}: {why}", self.at.display()),
            })?;
        let mut records = Vec::new();
        let mut not_records = Vec::new();
        for entry in entries {
            let path = match entry {
                Ok(entry) => entry.path(),
                Err(why) => {
                    return Err(WhatTheStoreSaid::ItWasRefused {
                        why: why.to_string(),
                    })
                }
            };
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                not_records.push(WhatTheStoreSaid::ItIsNotAJobRecord {
                    at: path.display().to_string(),
                });
                continue;
            };
            if name.starts_with(THE_STAGING_PREFIX) || !name.ends_with(THE_RECORDS_SUFFIX) {
                // A write in flight, or something that is not this store's.
                continue;
            }
            match std::fs::read(&path)
                .ok()
                .and_then(|bytes| serde_json::from_slice::<TheRecordAsWritten>(&bytes).ok())
                .and_then(TheRecordAsWritten::read_back)
            {
                Some(record) => records.push(record),
                None => not_records.push(WhatTheStoreSaid::ItIsNotAJobRecord {
                    at: path.display().to_string(),
                }),
            }
        }
        Ok((records, not_records))
    }

    /// One record, or nothing where none stands under that identity.
    ///
    /// **An absence is an absence and never a failure**: a job whose record
    /// is not there is a job that is not in the line, which is Q3 read from
    /// the other side.
    pub fn one_record(&self, job: &Identity) -> Option<JobRecord> {
        let bytes = std::fs::read(self.published(job)).ok()?;
        serde_json::from_slice::<TheRecordAsWritten>(&bytes)
            .ok()
            .and_then(TheRecordAsWritten::read_back)
    }

    /// **One record, and a read that could not be taken said so** — unlike
    /// [`TheQueueOnTheMachine::one_record`], which reads a refusal as nothing.
    ///
    /// **Three answers and they are not the same thing**: the record; no
    /// record standing under that identity; and the store refusing the read or
    /// holding a file under that name that is not a record. **The third is
    /// never folded into the second**, because *absent* is what the ask after a
    /// lost acknowledgement turns into the desktop removing its own record
    /// (`State, identity and recovery` V1 step 2), and a machine that could not
    /// read its own store has not found the job absent. Added at step 10c piece
    /// 5, for that ask.
    pub fn read_one(&self, job: &Identity) -> Result<Option<JobRecord>, WhatTheStoreSaid> {
        let at = self.published(job);
        let bytes = match std::fs::read(&at) {
            Ok(bytes) => bytes,
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(why) => {
                return Err(WhatTheStoreSaid::ItWasRefused {
                    why: format!("{}: {why}", at.display()),
                })
            }
        };
        serde_json::from_slice::<TheRecordAsWritten>(&bytes)
            .ok()
            .and_then(TheRecordAsWritten::read_back)
            .map(Some)
            .ok_or(WhatTheStoreSaid::ItIsNotAJobRecord {
                at: at.display().to_string(),
            })
    }

    /// **The record removed, which is what takes a job out of the line.**
    ///
    /// **An already-absent record is *done* and never a failure** — the same
    /// rule `smoozy-queue-history`'s `deletion` applies to every unit of the machine-side
    /// erase.
    pub fn remove(&mut self, job: &Identity) -> Result<(), WhatTheStoreSaid> {
        match std::fs::remove_file(self.published(job)) {
            Ok(()) => Ok(()),
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(why) => Err(WhatTheStoreSaid::ItWasRefused {
                why: why.to_string(),
            }),
        }
    }
}

/// One file, written and made durable before the call returns.
///
/// **`State, identity and recovery` §1.3 rule 6's flush**, taken on the file
/// itself. **The directory entry is not flushed**, which is the same bound
/// `Job files and outputs` M8's own write records and states in full — the
/// rename is ordered after the file's flush and the directory that afterwards
/// holds the published name is never flushed. **No route to it is invented
/// here.**
fn write_durably(at: &Path, bytes: &[u8]) -> Result<(), String> {
    use std::io::Write;
    let mut file = std::fs::File::create(at).map_err(|why| format!("{}: {why}", at.display()))?;
    file.write_all(bytes)
        .map_err(|why| format!("{}: {why}", at.display()))?;
    file.sync_all()
        .map_err(|why| format!("{}: {why}", at.display()))?;
    Ok(())
}

/// **A store whose one record cannot be removed, for the tests that have to
/// force [`TheQueueOnTheMachine::remove`] to refuse.**
///
/// Returns the store, with the record's own name already taken by a directory
/// that has something in it. The caller's root must exist.
///
/// **Why it is shaped this way, and why the shape it replaced was wrong.** The
/// shape it replaced made the store's directory a plain file, so the record's
/// path led through something that is not a directory. On a unix that is
/// `ENOTDIR` and the removal refuses. On Windows the record simply does not
/// exist, and the standard library documents `NotFound` as an answer a removal
/// may give for a path that does not exist — *"This function will only ever
/// return an error of kind `NotFound` if the given path does not exist"*
/// (`std::fs::remove_file`, the shipped documentation of the toolchain in use).
/// `NotFound` is the one answer [`TheQueueOnTheMachine::remove`] reads as
/// *already gone* and reports done, so the removal did not refuse there and
/// two tests that force a refusal failed on Windows alone.
///
/// **Here the record's name exists**, so that sentence rules `NotFound` out on
/// every system, and the same documentation promises the removal errors when
/// the path is a directory. The refusal no longer depends on which of two
/// allowed answers a system picks.
#[cfg(test)]
pub(crate) fn a_store_whose_record_cannot_be_removed(
    root: &Path,
    job: &Identity,
) -> TheQueueOnTheMachine {
    let store = TheQueueOnTheMachine::at(root.join("store"));
    let in_the_way = store.published(job);
    std::fs::create_dir_all(&in_the_way).expect("the record's name is taken");
    std::fs::write(in_the_way.join("in the way"), b"{}").expect("and it is not empty");
    store
}

/// **Q8, stated as a value: nothing in this module is the supervisor-
/// unavailable repair trigger, and nothing in it checks free space.**
///
/// The repair is `Server Setup`'s and `Desktop app and delivery`'s, and a
/// check before a write would be a second answer to a question the write
/// already answers — wrong the moment the space goes between the two.
pub const NOTHING_HERE_CHECKS_FREE_SPACE_OR_REPAIRS_A_FULL_MOUNT: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::{the_admission_point, the_line, JobRecord, QueueState, SequenceKey};
    use smoozy_state::identities::IdentityKind;

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    fn a_root(named: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("smoozy-queue-store-{}-{named}", std::process::id()));
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
        // **Q7: the supervisor writes the key**, and these tests set it
        // directly so the line's order is the one under test rather than the
        // order the records happen to be made in.
        record.sequence_key = Some(SequenceKey::read_back(key));
        record
    }

    /// **Q1 and Q3: a record written is a record read back**, and the line is
    /// what the records say rather than what the filesystem's order says.
    #[test]
    fn a_record_is_written_and_read_back_and_the_line_is_the_sequence_keys() {
        let root = a_root("write");
        let mut store = TheQueueOnTheMachine::at(&root);
        let first = a_job();
        let second = a_job();
        // Written out of order on purpose: the filesystem's order means
        // nothing and the sequence key is the line.
        store.write(&a_record(&second, 20)).expect("written");
        store.write(&a_record(&first, 10)).expect("written");
        let (records, not_records) = store.every_record().expect("read");
        assert_eq!(records.len(), 2);
        assert!(not_records.is_empty());
        let line = the_line(&records);
        assert_eq!(line.len(), 2);
        assert_eq!(line[0].job, first);
        assert_eq!(line[1].job, second);
        assert_eq!(store.one_record(&first).map(|r| r.job), Some(first));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **The record is published by a rename and a staged name is never read
    /// as a record.** A reader that took the staged file would read a line
    /// with a half-written job in it.
    #[test]
    fn a_staged_name_is_never_read_as_a_record_and_the_published_one_is() {
        let root = a_root("staged");
        let store = TheQueueOnTheMachine::at(&root);
        let job = a_job();
        // A write caught in flight: the staged name with a whole record's
        // bytes in it.
        std::fs::write(
            root.join(format!(
                "{THE_STAGING_PREFIX}{}{THE_RECORDS_SUFFIX}",
                job.as_text()
            )),
            serde_json::to_vec(&super::TheRecordAsWritten::of(&a_record(&job, 1))).unwrap(),
        )
        .unwrap();
        let (records, not_records) = store.every_record().expect("read");
        assert!(records.is_empty(), "a staged name is not a record");
        assert!(not_records.is_empty(), "and it is not a fault either");
        assert_eq!(store.one_record(&job), None);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **Q6: an untracked record is a record**, and the store answers with it
    /// like any other — the machine's own order decides what runs next,
    /// tracked or not.
    #[test]
    fn a_record_the_desktop_never_named_is_read_back_like_any_other() {
        let root = a_root("untracked");
        let mut store = TheQueueOnTheMachine::at(&root);
        let untracked = a_job();
        store.write(&a_record(&untracked, 1)).expect("written");
        let (records, _) = store.every_record().expect("read");
        assert_eq!(records.len(), 1);
        // The admission point takes it exactly as it would a tracked one.
        let taken = the_admission_point(&records, |_| "a label".to_string());
        assert_eq!(taken.started, Some(untracked));
        assert!(taken.dropped.is_empty());
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A file that is not a job record is reported and never read as an
    /// absent job, and never removed.**
    #[test]
    fn something_that_is_not_a_record_is_reported_and_left_alone() {
        let root = a_root("stranger");
        let mut store = TheQueueOnTheMachine::at(&root);
        let job = a_job();
        store.write(&a_record(&job, 1)).expect("written");
        let stranger = root.join("not-a-record.json");
        std::fs::write(&stranger, b"{}").unwrap();
        let (records, not_records) = store.every_record().expect("read");
        assert_eq!(records.len(), 1, "the real record is still read");
        assert_eq!(not_records.len(), 1);
        assert!(stranger.exists(), "and nothing removed it");
        std::fs::remove_dir_all(&root).ok();
    }

    /// **Q8: a write that cannot be made is refused loudly with the machine's
    /// own words, and nothing moves behind it.** The fault is introduced by
    /// taking the store's directory away, which is what a full mount and a
    /// missing package area both look like from here.
    #[test]
    fn a_write_that_cannot_be_made_is_refused_loudly_and_nothing_moves() {
        let root = a_root("refused");
        let mut store = TheQueueOnTheMachine::at(root.join("nothing-here"));
        let job = a_job();
        let refused = store.write(&a_record(&job, 1)).expect_err("refused");
        let WhatTheStoreSaid::ItWasRefused { why } = &refused else {
            panic!("{refused:?}");
        };
        assert!(!why.is_empty(), "the machine's own words");
        // **Nothing moved behind it**: no record stands and the read of the
        // store answers with the refusal rather than an empty line.
        assert!(store.every_record().is_err(), "never a guessed empty line");
        const _: () = assert!(NOTHING_HERE_CHECKS_FREE_SPACE_OR_REPAIRS_A_FULL_MOUNT);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A store that cannot be read answers with the refusal and never with
    /// an empty line** — which is the one silent wrong answer this module
    /// could give, since an empty line reads as *the machine has nothing to
    /// do*.
    #[test]
    fn a_store_that_cannot_be_read_never_answers_with_an_empty_line() {
        let store = TheQueueOnTheMachine::at("/no/such/package/area/queue");
        assert!(matches!(
            store.every_record(),
            Err(WhatTheStoreSaid::ItWasRefused { .. })
        ));
    }

    /// **A record removed takes the job out of the line, and an already-absent
    /// one is done rather than a failure.**
    #[test]
    fn a_removed_record_is_out_of_the_line_and_removing_it_twice_is_done() {
        let root = a_root("removed");
        let mut store = TheQueueOnTheMachine::at(&root);
        let job = a_job();
        store.write(&a_record(&job, 1)).expect("written");
        store.remove(&job).expect("removed");
        assert_eq!(store.one_record(&job), None);
        let (records, _) = store.every_record().expect("read");
        assert!(records.is_empty());
        store
            .remove(&job)
            .expect("an already-absent record is done");
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A read that could not be taken is never an absence**, and a file
    /// under a record's name that is not a record is not one either.
    #[test]
    fn reading_one_record_tells_absent_from_unreadable() {
        let root = a_root("read-one");
        let mut store = TheQueueOnTheMachine::at(&root);
        let job = a_job();
        assert_eq!(store.read_one(&job), Ok(None), "nothing written is absent");
        store.write(&a_record(&job, 1)).expect("written");
        assert_eq!(store.read_one(&job).expect("read").map(|r| r.job), Some(job.clone()));
        let other = a_job();
        std::fs::write(store.published(&other), b"not a record").unwrap();
        assert!(matches!(
            store.read_one(&other),
            Err(WhatTheStoreSaid::ItIsNotAJobRecord { .. })
        ));
        // **A read the system refuses**: the record's own name taken by a
        // directory, which every system refuses to read as a file and none
        // reports as not found — the shape `a_store_whose_record_cannot_be_removed`
        // settled for the same reason.
        let refused = a_store_whose_record_cannot_be_removed(&root, &job);
        assert!(matches!(
            refused.read_one(&job),
            Err(WhatTheStoreSaid::ItWasRefused { .. })
        ));
        std::fs::remove_dir_all(&root).ok();
    }

    /// **A record rewritten in place replaces the one before it**, which is
    /// what a state change is: each state is written at the event that makes
    /// it true, and the record is the one durable answer.
    #[test]
    fn a_state_change_is_a_rewrite_of_the_one_record() {
        let root = a_root("state");
        let mut store = TheQueueOnTheMachine::at(&root);
        let job = a_job();
        let mut record = a_record(&job, 1);
        store.write(&record).expect("written");
        assert!(store.one_record(&job).expect("there").is_waiting());
        record.state = QueueState::Rendering {
            operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
        };
        record.sequence_key = None;
        store.write(&record).expect("written");
        let back = store.one_record(&job).expect("there");
        assert!(!back.is_waiting());
        assert!(back.is_live());
        let (records, _) = store.every_record().expect("read");
        assert_eq!(records.len(), 1, "one record per job, always");
        std::fs::remove_dir_all(&root).ok();
    }
}

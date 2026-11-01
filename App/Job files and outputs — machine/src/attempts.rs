//! §3.4 — the attempt records beside the manifest.
//!
//! **M8: each render attempt writes one attempt record in the job record
//! directory, durably, before the render starts** — and *durably* on this route
//! is **three of the four parts** `State, identity and recovery` §1.3 rule 6
//! names, not all four. **What a write over the file-transfer connection
//! keeps:** the bytes are written, the file itself is flushed to the storage
//! device before the write is reported done, and where the write is published
//! by a name the rename follows that flush. **What it does not keep is rule 6's
//! last clause** — the flush of the directory that afterwards holds the
//! published name — **and nothing has established that it can be**, so no route
//! to it is written here. **What that costs**, in M8's own words: an attempt
//! record published and reported durable *"can be absent after a power cut"*,
//! and one lost that way leaves the reckoning nothing to read back, so C20
//! covers the whole range and the job is re-rendered whole at the user's
//! expense. `Server protocol and transfers` S20a is the transfer contract's own
//! side of the same limit. The ordering in the clause is still worth nothing
//! without the durability there is, *"since a record still in a cache when the
//! machine stops never existed"*. **It is written once and never rewritten; a
//! later attempt writes a new one and no attempt record is ever removed while
//! the job's record exists.**
//!
//! **M9: attempt records are what makes Row 134 answerable at all** — the frame
//! list has to survive an interruption, a reboot and Smoozy being closed, and
//! it is durable on the machine before the render begins. **And they are what
//! keeps Row 137's two arrangements separable**: where the file changed and the
//! map moved, both maps are on the machine, **each named by the attempt that
//! used it**.
//!
//! **M8's two further fields, added 2026-09-04 out of a code round.** The
//! statement that the file changed before this attempt (R13), and the
//! statement of whether this attempt kept any frame an earlier one had written
//! (C12a). Neither is read by the render. They are the two facts the telling
//! `Queue & History` N9a composes is read from, and they live here because the
//! record is the durable thing that survives the machine.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` S15a and S20a.

use serde::{Deserialize, Serialize};
use smoozy_framing::source_reading::SourceReading;

/// **The map in force for one attempt** (R7, R13).
///
/// **R13 keeps the previous attempt's map**, so one job's folder holding two
/// arrangements of subfolders is a recorded fact with each arrangement named by
/// the attempt that made it, rather than a layout nobody can account for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheMapInForce {
    /// **No reading stood at this attempt, so no map was composed for it.**
    /// R7's second bullet: where the file was not read before submitting, the
    /// render worker takes the reading after the load and composes the map
    /// then. **The record says so rather than carrying an empty map that would
    /// read as a map of nothing.**
    NoneWasComposedForThisAttempt,
    /// The map itself (§6).
    It(crate::mapping::TheMap),
}

/// **M8's record.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttemptRecord {
    /// The attempt's number.
    pub number: u32,
    /// **The frame list handed to the render in the order it was written.**
    ///
    /// **The order is the record**, because C18's last-frame-asked-for rule
    /// reads it back and *"a half-written frame list read by a later re-run
    /// would name a last frame that was never asked for"* (§12). It is
    /// integers, never text — C3.
    pub frame_list_in_the_order_it_was_written: Vec<i64>,
    /// **The map in force for that attempt.**
    pub map_in_force: TheMapInForce,
    /// **The file identity read at that moment** (R10, R12's first of two
    /// moments).
    #[serde(with = "crate::manifest::the_four_values")]
    pub file_identity_at_this_attempt: SourceReading,
    /// **R13's statement that the file changed before this attempt.** It is
    /// written here beside the fresh reading and the fresh map, and it is one
    /// of the two facts `Queue & History` N9a's telling is read from. It is a
    /// plain yes or no with no count beside it.
    ///
    /// **Absent on a record written before this field**, which
    /// [`serde`]'s default reads as *no*, exactly as N13's rule draws an
    /// absent fact as absent rather than as a claim.
    #[serde(default)]
    pub the_file_changed_before_this_attempt: bool,
    /// **C12a's statement of whether this attempt kept any frame an earlier
    /// one had written**, and the second of the two facts `Queue & History`
    /// N9a is read from.
    ///
    /// It is [`crate::reckoning::any_frame_an_earlier_attempt_wrote_was_kept`]
    /// over the list this record carries, taken before the render starts and
    /// never re-derived afterwards. **A plain yes or no**: a count would want a
    /// walk of the earlier attempt's map, which no clause takes.
    #[serde(default)]
    pub a_frame_an_earlier_attempt_wrote_was_kept: bool,
}

impl AttemptRecord {
    /// The record's own file name inside the job record directory.
    ///
    /// **Build's own choice**, and it carries the attempt's number because M9
    /// requires each map to be *"named by the attempt that used it"*.
    pub fn file_name(&self) -> String {
        format!("attempt-{}.json", self.number)
    }

    pub fn as_written(&self) -> Vec<u8> {
        serde_json::to_vec_pretty(self)
            .expect("an attempt record is composed of values that always render")
    }

    pub fn read_back(bytes: &[u8]) -> Option<AttemptRecord> {
        serde_json::from_slice(bytes).ok()
    }
}

/// **M8's *never rewritten*, kept where it can be seen.**
///
/// A later attempt writes a **new** record, so nothing anywhere opens an
/// existing attempt record for writing. That is why the publication of one is
/// the create-intended kind (`Server protocol and transfers` S15a) rather than
/// a replacement, and why the file name carries the number.
pub const NO_ATTEMPT_RECORD_IS_EVER_REWRITTEN_OR_REMOVED_WHILE_THE_JOBS_RECORD_EXISTS: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    fn a_record(number: u32, frames: Vec<i64>) -> AttemptRecord {
        AttemptRecord {
            number,
            frame_list_in_the_order_it_was_written: frames,
            map_in_force: TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: SourceReading {
                size: 4096,
                inode: "912".to_string(),
                modification_time: 1_756_800_000,
                change_time: 1_756_800_001,
            },
            the_file_changed_before_this_attempt: false,
            a_frame_an_earlier_attempt_wrote_was_kept: false,
        }
    }

    #[test]
    fn the_frame_lists_order_survives_the_round_trip() {
        // C18 reads the list back for **the order Smoozy asked for**, so an
        // order that did not survive being written would name a last frame that
        // was never asked for.
        let record = a_record(1, vec![10, 4, 7, 1]);
        let again = AttemptRecord::read_back(&record.as_written()).unwrap();
        assert_eq!(again.frame_list_in_the_order_it_was_written, vec![10, 4, 7, 1]);
    }

    #[test]
    fn two_attempts_write_two_records_and_neither_names_the_other() {
        // M9: both maps are on the machine, each named by the attempt that used
        // it.
        let first = a_record(1, vec![1, 2]);
        let second = a_record(2, vec![2]);
        assert_ne!(first.file_name(), second.file_name());
        assert_eq!(first.file_name(), "attempt-1.json");
        assert_eq!(second.file_name(), "attempt-2.json");
    }

    #[test]
    fn a_frame_list_is_integers_and_never_text() {
        // C3's requirement, kept by the type: there is no route by which a
        // frame reaches this record as a string for a machine to parse.
        let record = a_record(1, vec![1, 2, 3]);
        let written = String::from_utf8(record.as_written()).unwrap();
        assert!(!written.contains("\"1\""), "a frame was written as text: {written}");
    }
}

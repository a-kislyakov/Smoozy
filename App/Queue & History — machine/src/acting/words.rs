//! **§8 E4, E7, E8, E9, E13 and E13a performed — the capture into the sinks,
//! the live commentary read over the connection, the fetch of both streams
//! into the history record at reconciliation, and the two observations
//! recorded and carried to the entry.**
//!
//! # E7: the sinks are the record and this module makes none of them
//!
//! **`Rented-server runtime` M1 to M8, M4a and M13 are taken entire** — two
//! sinks per attempt made by the supervisor, the worker's own copy into them,
//! M4a's finite ceiling, and M13's four outcomes.
//! [`smoozy_runtime::acting::capture`] is where all of that already is, and
//! **this module opens no second sink, writes no second ceiling and invents no
//! second outcome.**
//!
//! **The record is the stream and not a digest of it**: the marker is part of
//! the raw record and the shell's own lines on Linux are in it too, unedited.
//!
//! # E4: which silence it is comes from the collection outcome
//!
//! ***The render printed nothing* and *nothing of what the render printed
//! reached Smoozy* are the same on a naive entry and are not the same event.**
//! **They are separated by two different records rather than by two readings
//! of one thing** (`Rented-server runtime` M14), and this module carries that
//! crate's answer into this one's shape without deciding anything.
//!
//! # E8: one capture serving two promises
//!
//! **While the job runs, the desktop reads the sink's current bytes over the
//! connection, on request, as a period-held reading shown on the entry** —
//! reading material for display, never state. **The read recurs while the
//! commentary is on screen, on `Desktop app and delivery` P10d's live-reading
//! sequence, and its repetition stops when the commentary is not.**
//! **After the ending, the same sinks are what history carries.**
//!
//! # E9: the fetch covers every attempt that carries a capture record
//!
//! **Never the ending attempt alone.** Two sinks and two outcomes per attempt,
//! **each attempt's pair fetched whole or not at all** (`Rented-server
//! runtime` M11), so a re-run job's entry holds what the interrupted attempt
//! said up to the interruption beside the ending attempt's own words —
//! **the record of whether the render was healthy when the machine went
//! down**, which would otherwise be readable only while connected to that
//! machine and die with the rental.
//!
//! **Where the machine is gone before any connection learned the ending, the
//! words are gone with it and the entry says it learned nothing** — the spec's
//! own boundary, and nothing here fills it.
//!
//! # E10: nothing reads the words
//!
//! **Not sorted, not grouped, not mined for a cause.** The three things
//! observed *at the loop* rather than read from the sink are E13's device
//! line, E13a's colour-substitution line and P1a's frame-save announcement,
//! **each an observation of bytes in passing, not a reading of the record.**
//! **No function in this module opens a sink to look for one.**
//!
//! # Where the two observations are made, and where they are not
//!
//! **The observation is `Rendering`'s copy loop's and the grammar is
//! [`crate::acting::reader`]'s** — R1 makes this crate's reader a tee on that
//! loop rather than a second read of anything. **What is here is the third
//! step: the observation recorded in the attempt's record, and step 4's
//! fill-in carrying it to the entry and into history.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` M1, M3, M4a, M8, M11,
//! M13, M14, M19, M28. `State, identity and recovery` §7, §9.3.
//! `Desktop app and delivery` P10d.
//!
//! **`Rented-server runtime` R1 is deliberately not on that list and is
//! written out at its site instead**, because this plan defines an R1 of its
//! own.

// Named by the store below, which is on a unix only.
#[cfg(unix)]
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use smoozy_runtime::capture::CollectionOutcome as TheRuntimesOutcome;
use smoozy_state::identities::{Identity, IdentityKind};

#[cfg(unix)]
use crate::acting::queue_store::WhatTheStoreSaid;
use crate::ending::{AnAttemptsWords, CollectionOutcome};
use crate::readings::{ObservedColourSubstitutionLine, ObservedDeviceLine, WhichStream};

/// **E4, performed: `Rented-server runtime` M13's four carried into this
/// document's shape, one for one.**
///
/// **Nothing is collapsed and nothing is defaulted.** Each of the four is a
/// sentence of its own on the entry, and an empty panel is never what the user
/// meets.
pub fn which_silence_it_was(outcome: TheRuntimesOutcome) -> CollectionOutcome {
    match outcome {
        TheRuntimesOutcome::Collected => CollectionOutcome::Collected,
        TheRuntimesOutcome::CollectedAndTheStreamWasEmpty => {
            CollectionOutcome::CollectedAndTheStreamWasEmpty
        }
        TheRuntimesOutcome::CollectionIncomplete => CollectionOutcome::CollectionIncomplete,
        TheRuntimesOutcome::NeverEstablished => CollectionOutcome::NeverEstablished,
    }
}

/// **E4 and E11: one attempt's pair of outcomes, composed from that crate's
/// own two answers.**
pub fn an_attempts_words(
    attempt: u32,
    ordinary_output: TheRuntimesOutcome,
    complaints: TheRuntimesOutcome,
) -> AnAttemptsWords {
    AnAttemptsWords {
        attempt,
        ordinary_output: which_silence_it_was(ordinary_output),
        complaints: which_silence_it_was(complaints),
    }
}

/// **E13 and E13a's third step: what the attempt's record carries about the
/// two lines.**
///
/// **The written form is Build's own choice**, in the shape `Job files and
/// outputs` already settled for the manifest and the attempt record.
/// **It holds no reading of the sink**: each value is an observation the copy
/// loop made in passing and handed over.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhatTheAttemptsRecordCarries {
    pub job: String,
    pub attempt: u32,
    /// **E13: the device line, as printed, and the stream the version's route
    /// printed it on.** Absent where none passed.
    pub device_line: Option<ALineAsWritten>,
    /// **E13a: the colour-substitution line.** **Absent is never evidence that
    /// no substitution happened.**
    pub colour_line: Option<AColourLineAsWritten>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ALineAsWritten {
    pub the_line_as_printed: String,
    pub on_stream: TheStreamAsWritten,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AColourLineAsWritten {
    /// **The substance, as one of the three kinds** — the exact printed text
    /// of the three is on record in no version, so nothing here writes one.
    pub kind: TheColourKindAsWritten,
    pub the_line_as_printed: String,
    pub on_stream: TheStreamAsWritten,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheStreamAsWritten {
    TheOrdinaryOutput,
    TheComplaints,
}

impl TheStreamAsWritten {
    fn of(stream: WhichStream) -> TheStreamAsWritten {
        match stream {
            WhichStream::TheOrdinaryOutput => TheStreamAsWritten::TheOrdinaryOutput,
            WhichStream::TheComplaints => TheStreamAsWritten::TheComplaints,
        }
    }

    fn read_back(self) -> WhichStream {
        match self {
            TheStreamAsWritten::TheOrdinaryOutput => WhichStream::TheOrdinaryOutput,
            TheStreamAsWritten::TheComplaints => WhichStream::TheComplaints,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheColourKindAsWritten {
    TheSceneViewNotFoundAndADefaultSet,
    TheImageColourSpaceNotFoundAndADefaultUsed,
    TheCurrentValueMatchesNoEntryInTheList,
}

impl TheColourKindAsWritten {
    fn of(kind: crate::ending::ColourSubstitutionKind) -> TheColourKindAsWritten {
        use crate::ending::ColourSubstitutionKind as K;
        match kind {
            K::TheSceneViewNotFoundAndADefaultSet => {
                TheColourKindAsWritten::TheSceneViewNotFoundAndADefaultSet
            }
            K::TheImageColourSpaceNotFoundAndADefaultUsed => {
                TheColourKindAsWritten::TheImageColourSpaceNotFoundAndADefaultUsed
            }
            K::TheCurrentValueMatchesNoEntryInTheList => {
                TheColourKindAsWritten::TheCurrentValueMatchesNoEntryInTheList
            }
        }
    }

    fn read_back(self) -> crate::ending::ColourSubstitutionKind {
        use crate::ending::ColourSubstitutionKind as K;
        match self {
            TheColourKindAsWritten::TheSceneViewNotFoundAndADefaultSet => {
                K::TheSceneViewNotFoundAndADefaultSet
            }
            TheColourKindAsWritten::TheImageColourSpaceNotFoundAndADefaultUsed => {
                K::TheImageColourSpaceNotFoundAndADefaultUsed
            }
            TheColourKindAsWritten::TheCurrentValueMatchesNoEntryInTheList => {
                K::TheCurrentValueMatchesNoEntryInTheList
            }
        }
    }
}

impl WhatTheAttemptsRecordCarries {
    /// **Composed from the two observations the copy loop handed over**, and
    /// from nothing else.
    pub fn from_the_observations(
        job: &Identity,
        attempt: u32,
        device_line: Option<&ObservedDeviceLine>,
        colour_line: Option<&ObservedColourSubstitutionLine>,
    ) -> WhatTheAttemptsRecordCarries {
        WhatTheAttemptsRecordCarries {
            job: job.as_text(),
            attempt,
            device_line: device_line.map(|line| ALineAsWritten {
                the_line_as_printed: line.the_line_as_printed.clone(),
                on_stream: TheStreamAsWritten::of(line.on_stream),
            }),
            colour_line: colour_line.map(|line| AColourLineAsWritten {
                kind: TheColourKindAsWritten::of(line.kind),
                the_line_as_printed: line.the_line_as_printed.clone(),
                on_stream: TheStreamAsWritten::of(line.on_stream),
            }),
        }
    }

    /// **Step 4's fill-in, read back into the shapes the entry judges over.**
    ///
    /// **What each means is [`crate::ending`]'s and is not decided here**:
    /// the device field turns on the recorded outcome and never on the line's
    /// presence (E12), and the colour statement infers nothing from absence.
    pub fn read_back(
        &self,
    ) -> (
        Option<ObservedDeviceLine>,
        Option<ObservedColourSubstitutionLine>,
    ) {
        (
            self.device_line.as_ref().map(|line| ObservedDeviceLine {
                attempt: self.attempt,
                the_line_as_printed: line.the_line_as_printed.clone(),
                on_stream: line.on_stream.read_back(),
            }),
            self.colour_line
                .as_ref()
                .map(|line| ObservedColourSubstitutionLine {
                    attempt: self.attempt,
                    kind: line.kind.read_back(),
                    the_line_as_printed: line.the_line_as_printed.clone(),
                    on_stream: line.on_stream.read_back(),
                }),
        )
    }
}

/// **The attempt's own records, in the job record directory.**
///
/// **`Job files and outputs` owns that directory** and this is one more file
/// in it, named by the attempt, in the shape that document already settled.
///
/// **On a unix only, because it publishes through
/// `smoozy_runtime::acting::durable` and that crate gates its whole `acting`
/// module to a unix.** The job record directory is a store on a rented server
/// — `State, identity and recovery` J11 — and that document's §1.3 rule 6
/// says a rented machine is a Linux machine. [`crate::acting`] states the
/// arrangement once for all four of this crate's such stores. **The fetch half
/// of this file is not gated** — [`fetch_the_words`] runs on the user's own
/// computer.
#[cfg(unix)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheObservationsOnTheMachine {
    at: PathBuf,
}

/// What every published observations record's name ends in.
pub const THE_OBSERVATIONS_SUFFIX: &str = ".observations.json";

#[cfg(unix)]
impl TheObservationsOnTheMachine {
    pub fn in_the_job_record_directory(
        directory: impl Into<PathBuf>,
    ) -> TheObservationsOnTheMachine {
        TheObservationsOnTheMachine {
            at: directory.into(),
        }
    }

    fn published(&self, attempt: u32) -> PathBuf {
        self.at
            .join(format!("attempt-{attempt}{THE_OBSERVATIONS_SUFFIX}"))
    }

    /// **Written once, in the act that ends the attempt.** Never rewritten,
    /// because an attempt's record is never rewritten.
    pub fn write(
        &mut self,
        carries: &WhatTheAttemptsRecordCarries,
    ) -> Result<(), WhatTheStoreSaid> {
        let written =
            serde_json::to_vec(carries).map_err(|why| WhatTheStoreSaid::ItWasRefused {
                why: why.to_string(),
            })?;
        smoozy_runtime::acting::durable::publish(&self.published(carries.attempt), &written)
            .map_err(|why| WhatTheStoreSaid::ItWasRefused { why: why.as_text() })
    }

    pub fn read(
        &self,
        attempt: u32,
    ) -> Result<Option<WhatTheAttemptsRecordCarries>, WhatTheStoreSaid> {
        let at = self.published(attempt);
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
}

/// **E8: the live commentary, read over the connection on request.**
///
/// **The sink's current bytes, as a period-held reading shown on the entry** —
/// reading material for display, never state. **Nothing durable is written
/// from it**, and this type has no route to a store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheLiveCommentary {
    pub attempt: u32,
    pub stream: WhichStream,
    /// The bytes as they stand, unedited. **The shell's own lines on Linux are
    /// in it too**, because the record is the stream and not a digest of it.
    pub bytes: Vec<u8>,
}

/// **The machine, on the far side of one trait, because it genuinely is.**
pub trait ReadsTheSinkOverTheConnection {
    /// **On request, and the request recurs only while the commentary is on
    /// screen** — `Desktop app and delivery` P10d's live-reading sequence.
    /// **This module holds no schedule**, and there is no timer here.
    fn the_sinks_current_bytes(
        &self,
        job: &Identity,
        attempt: u32,
        stream: WhichStream,
    ) -> Result<TheLiveCommentary, String>;

    /// **E9: both captured streams of one attempt, fetched whole or not at
    /// all** (`Rented-server runtime` M11).
    fn both_streams_of(
        &self,
        job: &Identity,
        attempt: u32,
    ) -> Result<(TheLiveCommentary, TheLiveCommentary), String>;
}

/// **What one attempt's fetch put into the history record.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneAttemptsWordsFetched {
    pub words: AnAttemptsWords,
    /// **Kept apart, unedited.** Two sinks per attempt, never merged.
    pub ordinary_output: Vec<u8>,
    pub complaints: Vec<u8>,
}

/// **What the fetch left, attempt by attempt.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheFetchLeft {
    /// Each attempt's pair, in attempt order. **An attempt whose pair could
    /// not be fetched is absent rather than half-present.**
    pub fetched: Vec<OneAttemptsWordsFetched>,
    /// The attempts whose pair could not be fetched, with what the machine
    /// said. **Partial across attempts, atomic within one.**
    pub could_not_be_fetched: Vec<(u32, String)>,
}

/// **E9, performed: the history record takes the words in at reconciliation,
/// and from then on they survive the machine.**
///
/// **The fetch covers every attempt of the job that carries a capture record,
/// never the ending attempt alone**, and each attempt's pair is fetched whole
/// or not at all.
///
/// **The cost is stated rather than hidden**: about 5 kB per frame on the
/// ordinary stream under 4.5 and about 3.7 kB under 5.0, with a measured zero
/// on the complaints stream — so a ten-thousand-frame job is roughly 50 MB, on
/// the user's own history under their own control, and **nothing expires it.**
pub fn fetch_the_words(
    machine: &dyn ReadsTheSinkOverTheConnection,
    job: &Identity,
    every_attempt_carrying_a_capture_record: &[(u32, TheRuntimesOutcome, TheRuntimesOutcome)],
) -> WhatTheFetchLeft {
    let mut fetched = Vec::new();
    let mut could_not_be_fetched = Vec::new();
    for (attempt, ordinary, complaints) in every_attempt_carrying_a_capture_record {
        match machine.both_streams_of(job, *attempt) {
            Ok((one, two)) => fetched.push(OneAttemptsWordsFetched {
                words: an_attempts_words(*attempt, *ordinary, *complaints),
                ordinary_output: one.bytes,
                complaints: two.bytes,
            }),
            Err(why) => could_not_be_fetched.push((*attempt, why)),
        }
    }
    WhatTheFetchLeft {
        fetched,
        could_not_be_fetched,
    }
}

/// **One attempt's capture record, as the note reading's answer carries it** —
/// E9's *every attempt of the job that carries a capture record*, with where
/// its two sinks are and what each stream came to. Added 2026-10-03, the
/// Creator's notes on 0.1.8, piece 6d.
///
/// **The desktop composes no machine path** (E8's rule, and hole 10 of the
/// map of those notes): the sinks are named here, by the machine, from
/// `Rented-server runtime` M1's capture record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACaptureRecordAsCarried {
    pub attempt: u32,
    /// Each sink's path and **the bytes standing in it when the answer was
    /// composed** — how far a whole read goes (`Rented-server runtime` M11).
    pub sinks: crate::readings::TheSinksOfARenderingJob,
    pub ordinary: WhatAStreamCameTo,
    pub complaints: WhatAStreamCameTo,
}

/// **What one stream came to**, as the machine recorded it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatAStreamCameTo {
    /// **`Rented-server runtime` M13's outcome. `None` is
    /// `Desktop app and delivery` M10a**: an outcome not yet written, which is
    /// none of the four and is never read as one.
    pub outcome: Option<TheRuntimesOutcome>,
    /// `Rented-server runtime` M5's retained count, where the worker reported
    /// one.
    pub retained: Option<u64>,
    /// `Rented-server runtime` M5's discarded count, where the worker
    /// reported one.
    pub discarded: Option<u64>,
    /// `Rented-server runtime` M5's first loss reason, in the machine's own
    /// words.
    pub first_loss: Option<String>,
}

/// The names one stream's record travels under, and no other stream's.
struct TheStreamsNames {
    outcome: &'static str,
    retained: &'static str,
    discarded: &'static str,
    first_loss: &'static str,
}

const THE_ORDINARY_STREAMS_NAMES: TheStreamsNames = TheStreamsNames {
    outcome: smoozy_framing::requests::THE_ORDINARY_OUTCOME_FIELD,
    retained: smoozy_framing::requests::THE_ORDINARY_RETAINED_COUNT_FIELD,
    discarded: smoozy_framing::requests::THE_ORDINARY_DISCARDED_COUNT_FIELD,
    first_loss: smoozy_framing::requests::THE_ORDINARY_FIRST_LOSS_FIELD,
};

const THE_COMPLAINTS_STREAMS_NAMES: TheStreamsNames = TheStreamsNames {
    outcome: smoozy_framing::requests::THE_COMPLAINTS_OUTCOME_FIELD,
    retained: smoozy_framing::requests::THE_COMPLAINTS_RETAINED_COUNT_FIELD,
    discarded: smoozy_framing::requests::THE_COMPLAINTS_DISCARDED_COUNT_FIELD,
    first_loss: smoozy_framing::requests::THE_COMPLAINTS_FIRST_LOSS_FIELD,
};

/// **The capture records, added to the note reading's answer** — what the
/// machine's program sends with a note whose ending is written. **One
/// composition and one reading back** ([`the_capture_records_read_from`]), so
/// the two ends spell every name once.
pub fn the_capture_records_as_fields(
    fields: smoozy_framing::framing::Fields,
    records: &[ACaptureRecordAsCarried],
) -> smoozy_framing::framing::Fields {
    use smoozy_framing::requests as asked;
    let item = |at: usize, name: &str| asked::the_field_of_item(asked::A_CAPTURE_RECORD, at, name);
    let mut fields = fields.and(asked::HOW_MANY_CAPTURE_RECORDS_FIELD, records.len().to_string());
    for (at, record) in records.iter().enumerate() {
        fields = fields.and(&item(at, asked::THE_ATTEMPT_FIELD), record.attempt.to_string());
        for (name, value) in record.sinks.as_pairs() {
            fields = fields.and(&item(at, name), value);
        }
        for (came_to, names) in [
            (&record.ordinary, THE_ORDINARY_STREAMS_NAMES),
            (&record.complaints, THE_COMPLAINTS_STREAMS_NAMES),
        ] {
            if let Some(outcome) = came_to.outcome {
                fields = fields.and(&item(at, names.outcome), outcome.in_words());
            }
            if let Some(count) = came_to.retained {
                fields = fields.and(&item(at, names.retained), count.to_string());
            }
            if let Some(count) = came_to.discarded {
                fields = fields.and(&item(at, names.discarded), count.to_string());
            }
            if let Some(why) = came_to.first_loss.as_deref() {
                fields = fields.and(&item(at, names.first_loss), why);
            }
        }
    }
    fields
}

/// **The capture records, read off the note reading's answer — whole or not
/// at all.**
///
/// **`None` where the answer carries no count** — a program older than piece
/// 6d, or a note with no ending — **and `None` where any record does not read
/// whole**: an attempt with no number, a sink with no path, or an outcome in
/// words that name none. A record left out would be an attempt E9's fetch
/// passed over in silence.
pub fn the_capture_records_read_from(
    fields: &smoozy_framing::framing::Fields,
) -> Option<Vec<ACaptureRecordAsCarried>> {
    use smoozy_framing::requests as asked;
    let how_many: usize = fields
        .text(asked::HOW_MANY_CAPTURE_RECORDS_FIELD)?
        .parse()
        .ok()?;
    let mut records = Vec::with_capacity(how_many);
    for at in 0..how_many {
        let text = |name: &str| {
            fields.text(&asked::the_field_of_item(asked::A_CAPTURE_RECORD, at, name))
        };
        let a_count = |name: &str| -> Result<Option<u64>, ()> {
            match text(name) {
                None => Ok(None),
                Some(written) => written.parse().map(Some).map_err(|_| ()),
            }
        };
        let came_to = |names: &TheStreamsNames| -> Option<WhatAStreamCameTo> {
            let outcome = match text(names.outcome) {
                None => None,
                Some(words) => Some(TheRuntimesOutcome::from_words(words)?),
            };
            Some(WhatAStreamCameTo {
                outcome,
                retained: a_count(names.retained).ok()?,
                discarded: a_count(names.discarded).ok()?,
                first_loss: text(names.first_loss).map(str::to_string),
            })
        };
        records.push(ACaptureRecordAsCarried {
            attempt: text(asked::THE_ATTEMPT_FIELD)?.parse().ok()?,
            sinks: crate::readings::TheSinksOfARenderingJob::read_back(text)?,
            ordinary: came_to(&THE_ORDINARY_STREAMS_NAMES)?,
            complaints: came_to(&THE_COMPLAINTS_STREAMS_NAMES)?,
        });
    }
    Some(records)
}

/// **The attempts [`fetch_the_words`] is handed: those whose two outcomes are
/// both written.** An attempt with an outcome not yet written is not fetched —
/// its record has nothing to say yet that `Desktop app and delivery` M10a's
/// own sentence does not —
/// and the caller keeps it as that sentence.
pub fn the_attempts_with_both_outcomes(
    records: &[ACaptureRecordAsCarried],
) -> Vec<(u32, TheRuntimesOutcome, TheRuntimesOutcome)> {
    records
        .iter()
        .filter_map(|record| {
            Some((
                record.attempt,
                record.ordinary.outcome?,
                record.complaints.outcome?,
            ))
        })
        .collect()
}

/// **E9's boundary, stated as a value: where the machine is gone before any
/// connection learned the ending, the words are gone with it and the entry
/// says it learned nothing.**
///
/// **Nothing here invents a word, a first line or a summary to fill that
/// silence.**
pub const NOTHING_FILLS_THE_SILENCE_OF_A_MACHINE_THAT_WENT_AWAY: bool = true;

/// **E10, stated as a value: nothing in this module reads the words.**
///
/// Not sorted, not grouped, not mined for a cause — no version hands out a
/// failure in a form a program could act on, and four wordings exist for one
/// failure. **Smoozy keeps them and shows them.**
pub const NOTHING_HERE_READS_THE_WORDS: bool = true;

/// The kind an identity naming a job in an observations record carries.
pub const A_JOB_IDENTITY_IS_A_JOB_IDENTITY: IdentityKind = IdentityKind::Job;

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use crate::ending::ColourSubstitutionKind;
    use std::collections::HashMap;

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    #[cfg(unix)]
    fn a_root(named: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("smoozy-queue-words-{}-{named}", std::process::id()));
        std::fs::remove_dir_all(&root).ok();
        std::fs::create_dir_all(&root).expect("the place is made");
        root
    }

    /// **E4: the four outcomes carry one for one, and the two silences stay
    /// two different events.**
    #[test]
    fn the_four_outcomes_carry_one_for_one_and_the_two_silences_stay_apart() {
        assert_eq!(
            which_silence_it_was(TheRuntimesOutcome::CollectedAndTheStreamWasEmpty),
            CollectionOutcome::CollectedAndTheStreamWasEmpty
        );
        assert_eq!(
            which_silence_it_was(TheRuntimesOutcome::NeverEstablished),
            CollectionOutcome::NeverEstablished
        );
        assert_ne!(
            which_silence_it_was(TheRuntimesOutcome::CollectedAndTheStreamWasEmpty),
            which_silence_it_was(TheRuntimesOutcome::NeverEstablished),
            "the render printing nothing is not Smoozy having established nothing"
        );
        assert_eq!(
            which_silence_it_was(TheRuntimesOutcome::Collected),
            CollectionOutcome::Collected
        );
        assert_eq!(
            which_silence_it_was(TheRuntimesOutcome::CollectionIncomplete),
            CollectionOutcome::CollectionIncomplete
        );
    }

    /// **E4 and E11: the outcomes are carried per stream per attempt.**
    #[test]
    fn each_attempt_carries_its_own_two_outcomes() {
        let words = an_attempts_words(
            2,
            TheRuntimesOutcome::CollectionIncomplete,
            TheRuntimesOutcome::CollectedAndTheStreamWasEmpty,
        );
        assert_eq!(words.attempt, 2);
        assert_eq!(
            words.ordinary_output,
            CollectionOutcome::CollectionIncomplete
        );
        assert_eq!(
            words.complaints,
            CollectionOutcome::CollectedAndTheStreamWasEmpty
        );
    }

    /// **E13 and E13a: both observations round-trip through the attempt's
    /// record, and an absent one reads back absent.**
    #[cfg(unix)]
    #[test]
    fn both_observations_round_trip_and_an_absent_one_reads_back_absent() {
        let root = a_root("observations");
        let mut store = TheObservationsOnTheMachine::in_the_job_record_directory(&root);
        let job = a_job();
        let device = ObservedDeviceLine {
            attempt: 1,
            the_line_as_printed: "Path tracing on: card 0".to_string(),
            on_stream: WhichStream::TheOrdinaryOutput,
        };
        let colour = ObservedColourSubstitutionLine {
            attempt: 1,
            kind: ColourSubstitutionKind::TheImageColourSpaceNotFoundAndADefaultUsed,
            the_line_as_printed: "whatever the version printed".to_string(),
            on_stream: WhichStream::TheComplaints,
        };
        store
            .write(&WhatTheAttemptsRecordCarries::from_the_observations(
                &job,
                1,
                Some(&device),
                Some(&colour),
            ))
            .expect("written");
        let back = store.read(1).expect("read").expect("there");
        let (read_device, read_colour) = back.read_back();
        assert_eq!(read_device.as_ref(), Some(&device));
        assert_eq!(read_colour.as_ref(), Some(&colour));

        // An attempt where neither line passed.
        store
            .write(&WhatTheAttemptsRecordCarries::from_the_observations(
                &job, 2, None, None,
            ))
            .expect("written");
        let (none_device, none_colour) = store.read(2).expect("read").expect("there").read_back();
        assert_eq!(none_device, None);
        assert_eq!(none_colour, None);
        // **E13a: absence is never evidence that no substitution happened.**
        assert_eq!(
            crate::ending::what_the_entry_says_about_colour(none_colour.as_ref()),
            crate::ending::WhatTheEntrySaysAboutColour::NothingAndNoClaim
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **E12 is kept by the ending and never by the line's presence** — a
    /// device line read back off a stopped job's record still gives *not
    /// known*.
    #[test]
    fn a_device_line_on_a_stopped_job_still_reads_not_known() {
        let line = ObservedDeviceLine {
            attempt: 1,
            the_line_as_printed: "Path tracing on: card 0".to_string(),
            on_stream: WhichStream::TheOrdinaryOutput,
        };
        assert_eq!(
            crate::ending::the_device_field(
                crate::ending::RecordedOutcome::LeftStopped,
                Some(&line)
            ),
            crate::ending::DevicesThatDidTheWork::NotKnown
        );
    }

    struct AMachineInHand {
        streams: HashMap<u32, (Vec<u8>, Vec<u8>)>,
        refuses: Vec<u32>,
    }

    impl ReadsTheSinkOverTheConnection for AMachineInHand {
        fn the_sinks_current_bytes(
            &self,
            _job: &Identity,
            attempt: u32,
            stream: WhichStream,
        ) -> Result<TheLiveCommentary, String> {
            let (one, two) = self
                .streams
                .get(&attempt)
                .ok_or_else(|| "no capture record".to_string())?;
            Ok(TheLiveCommentary {
                attempt,
                stream,
                bytes: match stream {
                    WhichStream::TheOrdinaryOutput => one.clone(),
                    WhichStream::TheComplaints => two.clone(),
                },
            })
        }

        fn both_streams_of(
            &self,
            job: &Identity,
            attempt: u32,
        ) -> Result<(TheLiveCommentary, TheLiveCommentary), String> {
            if self.refuses.contains(&attempt) {
                return Err("the connection dropped".to_string());
            }
            Ok((
                self.the_sinks_current_bytes(job, attempt, WhichStream::TheOrdinaryOutput)?,
                self.the_sinks_current_bytes(job, attempt, WhichStream::TheComplaints)?,
            ))
        }
    }

    fn a_machine(refuses: Vec<u32>) -> AMachineInHand {
        AMachineInHand {
            streams: HashMap::from([
                (1, (b"the first attempt".to_vec(), Vec::new())),
                (2, (b"the second attempt".to_vec(), b"a complaint".to_vec())),
            ]),
            refuses,
        }
    }

    /// **E9: the fetch covers every attempt that carries a capture record,
    /// never the ending attempt alone**, and each pair is kept apart.
    #[test]
    fn the_fetch_covers_every_attempt_and_keeps_the_two_streams_apart() {
        let job = a_job();
        let left = fetch_the_words(
            &a_machine(Vec::new()),
            &job,
            &[
                (
                    1,
                    TheRuntimesOutcome::CollectionIncomplete,
                    TheRuntimesOutcome::CollectedAndTheStreamWasEmpty,
                ),
                (
                    2,
                    TheRuntimesOutcome::Collected,
                    TheRuntimesOutcome::Collected,
                ),
            ],
        );
        assert_eq!(left.fetched.len(), 2);
        assert_eq!(left.fetched[0].words.attempt, 1);
        assert_eq!(left.fetched[0].ordinary_output, b"the first attempt");
        assert!(left.fetched[0].complaints.is_empty());
        assert_eq!(left.fetched[1].complaints, b"a complaint");
        assert!(left.could_not_be_fetched.is_empty());
    }

    /// **E9: whole or not at all, per attempt** — an attempt whose pair could
    /// not be fetched is absent rather than half-present, and the others
    /// still land.
    #[test]
    fn an_attempt_whose_pair_could_not_be_fetched_is_absent_rather_than_half_present() {
        let job = a_job();
        let left = fetch_the_words(
            &a_machine(vec![1]),
            &job,
            &[
                (
                    1,
                    TheRuntimesOutcome::Collected,
                    TheRuntimesOutcome::Collected,
                ),
                (
                    2,
                    TheRuntimesOutcome::Collected,
                    TheRuntimesOutcome::Collected,
                ),
            ],
        );
        assert_eq!(left.fetched.len(), 1);
        assert_eq!(left.fetched[0].words.attempt, 2);
        assert_eq!(left.could_not_be_fetched.len(), 1);
        assert_eq!(left.could_not_be_fetched[0].0, 1);
        const _: () = assert!(NOTHING_FILLS_THE_SILENCE_OF_A_MACHINE_THAT_WENT_AWAY);
    }

    fn a_capture_record(attempt: u32) -> ACaptureRecordAsCarried {
        use crate::readings::{ASink, TheSinksOfARenderingJob};
        ACaptureRecordAsCarried {
            attempt,
            sinks: TheSinksOfARenderingJob {
                ordinary: ASink {
                    path: format!("/area/streams/j/attempt-{attempt}/ordinary"),
                    length: Some(40),
                },
                complaints: ASink {
                    path: format!("/area/streams/j/attempt-{attempt}/complaints"),
                    length: Some(0),
                },
            },
            ordinary: WhatAStreamCameTo {
                outcome: Some(TheRuntimesOutcome::CollectionIncomplete),
                retained: Some(40),
                discarded: Some(9000),
                first_loss: Some("No space left on device".to_string()),
            },
            complaints: WhatAStreamCameTo {
                outcome: Some(TheRuntimesOutcome::CollectedAndTheStreamWasEmpty),
                retained: Some(0),
                discarded: Some(0),
                first_loss: None,
            },
        }
    }

    /// **E9: every attempt's capture record travels on the note reading's
    /// answer and is read back whole**, each stream's outcome and counts under
    /// its own names — and **an outcome not yet written travels as an absence
    /// and reads back as one** (`Desktop app and delivery` M10a), never as one
    /// of the four. The Creator's notes on 0.1.8, piece 6d.
    #[test]
    fn every_capture_record_travels_on_the_answer_and_reads_back_whole() {
        let mut pending = a_capture_record(2);
        pending.ordinary = WhatAStreamCameTo::default();
        pending.complaints = WhatAStreamCameTo::default();
        pending.sinks.ordinary.length = None;
        let records = vec![a_capture_record(1), pending];
        let fields = the_capture_records_as_fields(
            smoozy_framing::framing::Fields::none(),
            &records,
        );
        assert_eq!(the_capture_records_read_from(&fields), Some(records.clone()));
        assert_eq!(
            the_attempts_with_both_outcomes(&records),
            vec![(
                1,
                TheRuntimesOutcome::CollectionIncomplete,
                TheRuntimesOutcome::CollectedAndTheStreamWasEmpty
            )],
            "the attempt whose outcomes are not yet written is not fetched"
        );
    }

    /// **Whole or not at all**: an answer from an older program carries no
    /// count and reads as nothing, and a record that does not read whole —
    /// here an outcome in words that name none — makes the whole answer
    /// nothing rather than a list one attempt short.
    #[test]
    fn an_answer_with_no_count_or_a_record_that_does_not_read_whole_reads_as_nothing() {
        use smoozy_framing::requests as asked;
        assert_eq!(
            the_capture_records_read_from(&smoozy_framing::framing::Fields::none()),
            None
        );
        let mut unwritten = a_capture_record(1);
        unwritten.ordinary.outcome = None;
        let fields = the_capture_records_as_fields(
            smoozy_framing::framing::Fields::none(),
            &[unwritten],
        )
        .and(
            &asked::the_field_of_item(asked::A_CAPTURE_RECORD, 0, asked::THE_ORDINARY_OUTCOME_FIELD),
            "printed something",
        );
        assert_eq!(the_capture_records_read_from(&fields), None);
    }

    /// **E8: the live read is bytes as they stand, unedited**, and nothing
    /// durable is written from it.
    #[test]
    fn the_live_commentary_is_the_bytes_as_they_stand() {
        let job = a_job();
        let live = a_machine(Vec::new())
            .the_sinks_current_bytes(&job, 2, WhichStream::TheOrdinaryOutput)
            .expect("read");
        assert_eq!(live.bytes, b"the second attempt");
        assert_eq!(live.stream, WhichStream::TheOrdinaryOutput);
    }

    /// **E10: nothing in this module reads the words.** The guard is a scan
    /// of the working half for anything that would sort, group or mine them.
    #[test]
    fn nothing_here_sorts_groups_or_mines_the_words() {
        let source = include_str!("words.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let code: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in [
            ".sort",
            ".contains(\"",
            "starts_with(\"",
            "cause",
            "summary",
        ] {
            assert!(
                !code.contains(forbidden),
                "{forbidden} would be a reading of the words, which E10 forbids"
            );
        }
        const _: () = assert!(NOTHING_HERE_READS_THE_WORDS);
        assert_eq!(A_JOB_IDENTITY_IS_A_JOB_IDENTITY, IdentityKind::Job);
    }
}

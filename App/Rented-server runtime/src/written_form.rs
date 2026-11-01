//! The written form of this document's own durable stores in the package area.
//!
//! # Which stores these are, and why the form is decided here
//!
//! **Four durable things in the package area are this document's own**, and
//! each says so at itself: the package-phase record (G1, G2), the hold record —
//! U2's own words are *"a durable store of this document's, in the package
//! area"* — the machine-level setup-in-progress state (P1), and the
//! boot-registration record (K16). **The claim file is a fifth** (G1b), and it
//! holds the claiming installation identity and G2's transaction-owner fields.
//!
//! **What the stores hold is the plan's; how the bytes are laid out is not.**
//! No clause anywhere fixes a written form for them, so it is a Build decision
//! and it is recorded in this module's decisions record rather than being
//! chosen silently here.
//!
//! ~~**Two machine-side stores are deliberately absent from this file** and are
//! `State, identity and recovery` §1.2's rather than this document's: the
//! operation record on the machine, and the event journal. This crate reads
//! both and writes neither, and nothing here invents a form for either.~~
//!
//! **CORRECTED 2026-09-06, and only half of it was ever right.** ~~The **event
//! journal** is still absent from this file and nothing here invents a form for
//! it.~~ **The operation records and the command records are written here from
//! that date**, because §1.3 rule 1 makes the supervisor the only writer of
//! every durable store in `State, identity and recovery` §1.2 and the
//! supervisor is this crate's program — so *this crate writes neither* was
//! false the moment either store existed. **What is still that document's is
//! what a record holds and what may be written into it**, which is
//! `smoozy_state::machine_operations` and is not restated here; **what is this
//! crate's is the bytes**, on the same ground as the four stores above: no
//! clause anywhere fixes a written form for them either.
//!
//! **AND THE EVENT JOURNAL JOINED THEM LATER THE SAME DAY, for the same reason
//! and by the same split.** `State, identity and recovery` §4 is that document's
//! entire — what an event is, what a sequence promises, the cursor and
//! acknowledgement rules, and `State, identity and recovery` E6a's cap and
//! per-entry budget are all
//! `smoozy_state::journal`'s and none of them is restated here. **What is here
//! is the bytes of one entry**, because no clause anywhere fixes a written form
//! for that store either, and because §1.3 rule 1 makes the same supervisor its
//! only writer. [`an_event_as_written`] is that form; the append and the read
//! are [`crate::acting::journal`]'s.
//!
//! **Nothing reaches inside a record.** Both types keep the four fields their
//! own rules guard private, and this file goes through their published routes —
//! the accessors on the way out and `read_back` on the way in — exactly as it
//! does for an identity and a reading.
//!
//! # Two values this crate does not own, and how each is written
//!
//! An identity and a reading are both `State, identity and recovery`'s types,
//! and neither carries a written form of its own. **Each is written here
//! through that type's own published route and never by reaching inside it.**
//!
//! - **An identity** is written as its kind and its text — `Identity::as_text`
//!   and `Identity::read_back`, which is the pair that document offers. The kind
//!   is carried because `read_back` takes one; **it is carried rather than
//!   assumed per field**, so a record cannot be read back as an identity of the
//!   wrong kind by a caller who guessed.
//! - **A reading** is written as its moment and its clock —
//!   `Reading::taken_at`, `Reading::clock` and `Reading::taken`. The moment is
//!   written as a signed count of seconds from the usual epoch with its
//!   nanoseconds beside it, **signed because a machine's clock is its own** and
//!   nothing here corrects one: V11 records that a container's clock is the
//!   host's and that nothing inside can discipline it, so a moment before the
//!   epoch is a reading this form must be able to carry rather than refuse.

use serde::{Deserialize, Serialize};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, MachineClock, Reading};

/// An identity as it is written down: its kind, and its own text.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnIdentityAsWritten {
    kind: WhichKind,
    text: String,
}

/// The identity kinds `State, identity and recovery` §2.2 registers a kind for,
/// named in a form that survives being written down.
///
/// **It is a total match in both directions**, so a kind added to that document
/// stops this crate compiling rather than being written as something else —
/// which is what happened on 2026-09-06, when the event journal added the
/// eleventh.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum WhichKind {
    LocalServer,
    LocalKey,
    SecretStoreSubject,
    Job,
    Group,
    Installation,
    Run,
    Condition,
    Transfer,
    Operation,
    Event,
}

impl WhichKind {
    fn of(kind: IdentityKind) -> Self {
        match kind {
            IdentityKind::LocalServer => WhichKind::LocalServer,
            IdentityKind::LocalKey => WhichKind::LocalKey,
            IdentityKind::SecretStoreSubject => WhichKind::SecretStoreSubject,
            IdentityKind::Job => WhichKind::Job,
            IdentityKind::Group => WhichKind::Group,
            IdentityKind::Installation => WhichKind::Installation,
            IdentityKind::Run => WhichKind::Run,
            IdentityKind::Condition => WhichKind::Condition,
            IdentityKind::Transfer => WhichKind::Transfer,
            IdentityKind::Operation => WhichKind::Operation,
            IdentityKind::Event => WhichKind::Event,
        }
    }

    fn back(self) -> IdentityKind {
        match self {
            WhichKind::LocalServer => IdentityKind::LocalServer,
            WhichKind::LocalKey => IdentityKind::LocalKey,
            WhichKind::SecretStoreSubject => IdentityKind::SecretStoreSubject,
            WhichKind::Job => IdentityKind::Job,
            WhichKind::Group => IdentityKind::Group,
            WhichKind::Installation => IdentityKind::Installation,
            WhichKind::Run => IdentityKind::Run,
            WhichKind::Condition => IdentityKind::Condition,
            WhichKind::Transfer => IdentityKind::Transfer,
            WhichKind::Operation => IdentityKind::Operation,
            WhichKind::Event => IdentityKind::Event,
        }
    }
}

/// How an identity is written into one of this document's stores.
pub mod an_identity {
    use super::*;

    /// Write it as its kind and its own text.
    pub fn serialize<S>(identity: &Identity, out: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        AnIdentityAsWritten {
            kind: WhichKind::of(identity.kind()),
            text: identity.as_text(),
        }
        .serialize(out)
    }

    /// Read it back through that document's own route.
    ///
    /// **A text the route refuses is a refusal here too**, and never an
    /// identity made up to fill the field.
    pub fn deserialize<'a, D>(from: D) -> Result<Identity, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let written = AnIdentityAsWritten::deserialize(from)?;
        Identity::read_back(written.kind.back(), &written.text).ok_or_else(|| {
            serde::de::Error::custom("the recorded text is not an identity this route reads back")
        })
    }
}

/// A moment and the clock it was taken on, as they are written down.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AReadingAsWritten {
    seconds_from_the_usual_epoch: i64,
    nanoseconds: u32,
    clock: WhichClock,
}

/// Which clock a reading was taken on — `State, identity and recovery` D5's own division.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum WhichClock {
    ThisComputer,
    AMachine(String),
}

/// **A moment as a whole second and the nanoseconds after it**, which is how
/// this file writes every moment down.
///
/// **It is public because U14 puts the same moment on the wire**, and one
/// derivation is what keeps the two from drifting: the release's moment in the
/// hold record and the release's moment in the command's result are the same
/// figure taken the same way. `Rented-server runtime`'s
/// [`crate::acting::stores::the_release_moment_as_text`] is the other caller.
///
/// **The nanoseconds are the part after the second and are never negative**, so
/// a moment before the usual epoch has a negative second and a positive
/// remainder. A machine's clock is its own and nothing here corrects one, so
/// such a moment is carried rather than refused.
pub fn the_moment_as_a_pair(taken_at: std::time::SystemTime) -> (i64, u32) {
    match taken_at.duration_since(std::time::UNIX_EPOCH) {
        Ok(since) => (since.as_secs() as i64, since.subsec_nanos()),
        Err(before) => {
            let back = before.duration();
            if back.subsec_nanos() == 0 {
                (-(back.as_secs() as i64), 0)
            } else {
                (
                    -(back.as_secs() as i64) - 1,
                    1_000_000_000 - back.subsec_nanos(),
                )
            }
        }
    }
}

/// How a reading with no value of its own is written — the moment a hold's
/// release outcome was produced on the machine's own clock (U2).
pub mod a_reading_of_nothing {
    use super::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(reading: &Reading<()>, out: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (seconds, nanoseconds) = super::the_moment_as_a_pair(reading.taken_at());
        AReadingAsWritten {
            seconds_from_the_usual_epoch: seconds,
            nanoseconds,
            clock: match reading.clock() {
                Clock::ThisComputer => WhichClock::ThisComputer,
                Clock::AMachine(MachineClock(named)) => WhichClock::AMachine(named.clone()),
            },
        }
        .serialize(out)
    }

    /// **A moment that does not rebuild is a refusal, and never a stopped
    /// program.**
    ///
    /// This reads a hold record's release moment at a supervisor start, and
    /// **S14 and the whole of Section 16 item 12 are about a supervisor that
    /// keeps answering and refuses loudly.** Both the addition and the
    /// subtraction can run past what a moment can hold, so both are taken in
    /// their checked forms and a figure that does not rebuild is the reader's own
    /// refusal — the same shape [`an_identity`]'s own reader already takes, where
    /// a text the route refuses is a refusal and never a value made up to fill
    /// the field.
    pub fn deserialize<'a, D>(from: D) -> Result<Reading<()>, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let written = AReadingAsWritten::deserialize(from)?;
        let taken_at = the_moment_in(written.seconds_from_the_usual_epoch, written.nanoseconds)
            .ok_or_else(|| {
                serde::de::Error::custom(
                    "the recorded moment does not rebuild as a moment on this machine",
                )
            })?;
        let clock = match written.clock {
            WhichClock::ThisComputer => Clock::ThisComputer,
            WhichClock::AMachine(named) => Clock::AMachine(MachineClock(named)),
        };
        let _: SystemTime = taken_at;
        Ok(Reading::taken((), taken_at, clock))
    }

    /// The moment a written seconds-and-nanoseconds pair names, or nothing where
    /// it names none.
    fn the_moment_in(seconds: i64, nanoseconds: u32) -> Option<SystemTime> {
        // A nanoseconds field past a whole second is not a figure this form ever
        // writes, and the subtraction below would run past zero on one.
        if nanoseconds >= 1_000_000_000 {
            return None;
        }
        if seconds >= 0 {
            let whole = Duration::from_secs(seconds as u64)
                .checked_add(Duration::from_nanos(u64::from(nanoseconds)))?;
            return UNIX_EPOCH.checked_add(whole);
        }
        // `seconds` is at most -1 here, so its own negation is a figure.
        let back = seconds.unsigned_abs();
        let before = if nanoseconds == 0 {
            Duration::from_secs(back)
        } else {
            Duration::from_secs(back.checked_sub(1)?)
                .checked_add(Duration::from_nanos(u64::from(
                    1_000_000_000 - nanoseconds,
                )))?
        };
        UNIX_EPOCH.checked_sub(before)
    }
}

/// **The written form of `State, identity and recovery` §5.1's two machine-side
/// records.**
///
/// **The two types are that document's and keep four fields private**, so this
/// goes through their own published routes rather than deriving on them: the
/// accessors on the way out, `read_back` on the way in. **A field this crate
/// does not understand is carried and never read** — the operation's kind, the
/// recorded execution group's pair and the kind's own safe-boundary marker are
/// all opaque there and stay opaque here.
pub mod a_machine_side_record {
    use super::*;
    use smoozy_state::machine_operations::{
        CommandRecord, DesiredState, Disposition, OperationRecord, RecordedOutcome,
        TheRecordedExecutionGroup, WhichOperationItActsOn,
    };

    #[derive(Debug, Clone, Serialize, Deserialize)]
    enum WhichDesiredState {
        ShouldRun,
        ShouldBeStopped,
        Ended,
    }

    impl WhichDesiredState {
        fn of(state: DesiredState) -> Self {
            match state {
                DesiredState::ShouldRun => WhichDesiredState::ShouldRun,
                DesiredState::ShouldBeStopped => WhichDesiredState::ShouldBeStopped,
                DesiredState::Ended => WhichDesiredState::Ended,
            }
        }

        fn back(&self) -> DesiredState {
            match self {
                WhichDesiredState::ShouldRun => DesiredState::ShouldRun,
                WhichDesiredState::ShouldBeStopped => DesiredState::ShouldBeStopped,
                WhichDesiredState::Ended => DesiredState::Ended,
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    enum WhichDisposition {
        Resumed,
        ReRunOnce,
        LeftStopped,
        GivenUp,
    }

    impl WhichDisposition {
        fn of(disposition: Disposition) -> Self {
            match disposition {
                Disposition::Resumed => WhichDisposition::Resumed,
                Disposition::ReRunOnce => WhichDisposition::ReRunOnce,
                Disposition::LeftStopped => WhichDisposition::LeftStopped,
                Disposition::GivenUp => WhichDisposition::GivenUp,
            }
        }

        fn back(&self) -> Disposition {
            match self {
                WhichDisposition::Resumed => Disposition::Resumed,
                WhichDisposition::ReRunOnce => Disposition::ReRunOnce,
                WhichDisposition::LeftStopped => Disposition::LeftStopped,
                WhichDisposition::GivenUp => Disposition::GivenUp,
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    enum WhichOutcome {
        LeftStopped,
        GivenUp,
        TheKindsOwn(String),
    }

    impl WhichOutcome {
        fn of(outcome: &RecordedOutcome) -> Self {
            match outcome {
                RecordedOutcome::LeftStopped => WhichOutcome::LeftStopped,
                RecordedOutcome::GivenUp => WhichOutcome::GivenUp,
                RecordedOutcome::TheKindsOwn(words) => WhichOutcome::TheKindsOwn(words.clone()),
            }
        }

        fn back(&self) -> RecordedOutcome {
            match self {
                WhichOutcome::LeftStopped => RecordedOutcome::LeftStopped,
                WhichOutcome::GivenUp => RecordedOutcome::GivenUp,
                WhichOutcome::TheKindsOwn(words) => RecordedOutcome::TheKindsOwn(words.clone()),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct AGroupAsWritten {
        as_that_document_recorded_it: String,
        #[serde(with = "an_identity")]
        started_under_run: Identity,
    }

    /// An operation record as it is written down — `State, identity and
    /// recovery` G1's eight things, in that clause's own order.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnOperationRecordAsWritten {
        #[serde(with = "an_identity")]
        operation: Identity,
        #[serde(with = "an_identity")]
        subject: Identity,
        kind: String,
        desired_state: WhichDesiredState,
        attempt_count: u32,
        execution_group: Option<AGroupAsWritten>,
        the_kinds_safe_boundary_marker: Option<Vec<u8>>,
        latest_disposition: Option<(WhichDisposition, AnIdentityAsWritten)>,
        recorded_outcome: Option<WhichOutcome>,
    }

    impl AnOperationRecordAsWritten {
        pub fn of(record: &OperationRecord) -> Self {
            AnOperationRecordAsWritten {
                operation: record.operation.clone(),
                subject: record.subject.clone(),
                kind: record.kind.clone(),
                desired_state: WhichDesiredState::of(record.desired_state()),
                attempt_count: record.attempt_count,
                execution_group: record.execution_group.as_ref().map(|group| AGroupAsWritten {
                    as_that_document_recorded_it: group.as_that_document_recorded_it.clone(),
                    started_under_run: group.started_under_run.clone(),
                }),
                the_kinds_safe_boundary_marker: record.the_kinds_safe_boundary_marker.clone(),
                latest_disposition: record.latest_disposition().map(|(which, run)| {
                    (
                        WhichDisposition::of(*which),
                        AnIdentityAsWritten {
                            kind: WhichKind::of(run.kind()),
                            text: run.as_text(),
                        },
                    )
                }),
                recorded_outcome: record.recorded_outcome().map(WhichOutcome::of),
            }
        }

        /// **Read back through that document's own route.** A run identifier
        /// that will not read back refuses the whole record rather than being
        /// dropped, because a disposition with no run identifier is a field
        /// `State, identity and recovery` G1 says the record holds.
        pub fn back(&self) -> Result<OperationRecord, TheRecordCouldNotBeRead> {
            let latest_disposition = match &self.latest_disposition {
                None => None,
                Some((which, run)) => {
                    let run = Identity::read_back(run.kind.back(), &run.text).ok_or_else(|| {
                        TheRecordCouldNotBeRead {
                            what_the_reader_said:
                                "the run identifier beside the disposition does not read back"
                                    .to_string(),
                        }
                    })?;
                    Some((which.back(), run))
                }
            };
            Ok(OperationRecord::read_back(
                self.operation.clone(),
                self.subject.clone(),
                self.kind.clone(),
                self.desired_state.back(),
                self.attempt_count,
                self.execution_group
                    .as_ref()
                    .map(|group| TheRecordedExecutionGroup {
                        as_that_document_recorded_it: group.as_that_document_recorded_it.clone(),
                        started_under_run: group.started_under_run.clone(),
                    }),
                self.the_kinds_safe_boundary_marker.clone(),
                latest_disposition,
                self.recorded_outcome.as_ref().map(|outcome| outcome.back()),
            ))
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    enum WhichSubject {
        TheCallerNamedIt(AnIdentityAsWritten),
        ARuleTheSupervisorResolves {
            the_calling_documents_rule: String,
            resolved_to: Option<AnIdentityAsWritten>,
        },
    }

    /// A command record as it is written down — `State, identity and recovery`
    /// G4's three things and nothing else.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ACommandRecordAsWritten {
        #[serde(with = "an_identity")]
        operation: Identity,
        subject: WhichSubject,
        recorded_outcome: Option<WhichOutcome>,
    }

    fn written(identity: &Identity) -> AnIdentityAsWritten {
        AnIdentityAsWritten {
            kind: WhichKind::of(identity.kind()),
            text: identity.as_text(),
        }
    }

    fn read_one_back(written: &AnIdentityAsWritten) -> Result<Identity, TheRecordCouldNotBeRead> {
        Identity::read_back(written.kind.back(), &written.text).ok_or_else(|| {
            TheRecordCouldNotBeRead {
                what_the_reader_said: "the recorded text is not an identity this route reads back"
                    .to_string(),
            }
        })
    }

    impl ACommandRecordAsWritten {
        pub fn of(record: &CommandRecord) -> Self {
            ACommandRecordAsWritten {
                operation: record.operation.clone(),
                subject: match record.subject() {
                    WhichOperationItActsOn::TheCallerNamedIt(target) => {
                        WhichSubject::TheCallerNamedIt(written(target))
                    }
                    WhichOperationItActsOn::ARuleTheSupervisorResolves {
                        the_calling_documents_rule,
                        resolved_to,
                    } => WhichSubject::ARuleTheSupervisorResolves {
                        the_calling_documents_rule: the_calling_documents_rule.clone(),
                        resolved_to: resolved_to.as_ref().map(written),
                    },
                },
                recorded_outcome: record.recorded_outcome().map(WhichOutcome::of),
            }
        }

        pub fn back(&self) -> Result<CommandRecord, TheRecordCouldNotBeRead> {
            let subject = match &self.subject {
                WhichSubject::TheCallerNamedIt(target) => {
                    WhichOperationItActsOn::TheCallerNamedIt(read_one_back(target)?)
                }
                WhichSubject::ARuleTheSupervisorResolves {
                    the_calling_documents_rule,
                    resolved_to,
                } => WhichOperationItActsOn::ARuleTheSupervisorResolves {
                    the_calling_documents_rule: the_calling_documents_rule.clone(),
                    resolved_to: match resolved_to {
                        None => None,
                        Some(target) => Some(read_one_back(target)?),
                    },
                },
            };
            Ok(CommandRecord::read_back(
                self.operation.clone(),
                subject,
                self.recorded_outcome.as_ref().map(|outcome| outcome.back()),
            ))
        }
    }
}

/// **The written form of one entry of `State, identity and recovery` §4's event
/// journal.**
///
/// **The type is that document's and keeps its three fields private**, so this
/// goes through its own published routes rather than deriving on it: the
/// accessors on the way out, `AnEvent::journalled` on the way in. **A field this
/// crate does not understand is carried and never read** — the event's kind and
/// what it says are the owning document's own words and stay opaque here,
/// exactly as an operation's kind does.
///
/// **The event identity is written with its kind like every other identity**, so
/// an entry cannot be read back as an event under an operation's identity: the
/// route on the way in refuses any identity that is not an event's.
///
/// **One entry is one line.** [`write`] ends every record with a newline, and
/// the journal is appended to rather than published by a name — which is the
/// whole of why an append can outlive the moment the record store starts
/// refusing (Section 16 item 26). A line that has no newline behind it never
/// completed, and [`crate::acting::journal`] is where that is acted on.
pub mod an_event_as_written {
    use super::*;
    use smoozy_state::journal::{AnEvent, SequenceNumber, WhatHappened};

    /// One journalled event, as it is written down.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnEventAsWritten {
        #[serde(with = "an_identity")]
        event: Identity,
        sequence: SequenceNumber,
        kind: String,
        about: Option<AnIdentityAsWritten>,
        says: String,
        #[serde(with = "a_reading_of_nothing")]
        happened_at: Reading<()>,
    }

    impl AnEventAsWritten {
        pub fn of(event: &AnEvent) -> Self {
            let what = event.what();
            AnEventAsWritten {
                event: event.event().clone(),
                sequence: event.sequence(),
                kind: what.kind.clone(),
                about: what.about.as_ref().map(|subject| AnIdentityAsWritten {
                    kind: WhichKind::of(subject.kind()),
                    text: subject.as_text(),
                }),
                says: what.says.clone(),
                happened_at: what.happened_at.clone(),
            }
        }

        /// **Read back through that document's own route.**
        ///
        /// **A subject that will not read back refuses the whole entry rather
        /// than being dropped**, because a reader finds its own work by that
        /// field and an entry that quietly lost it would be an event about
        /// nothing.
        pub fn back(&self) -> Result<AnEvent, TheRecordCouldNotBeRead> {
            let about = match &self.about {
                None => None,
                Some(subject) => Some(
                    Identity::read_back(subject.kind.back(), &subject.text).ok_or_else(|| {
                        TheRecordCouldNotBeRead {
                            what_the_reader_said:
                                "the subject beside the event does not read back".to_string(),
                        }
                    })?,
                ),
            };
            AnEvent::journalled(
                self.event.clone(),
                self.sequence,
                WhatHappened {
                    kind: self.kind.clone(),
                    about,
                    says: self.says.clone(),
                    happened_at: self.happened_at.clone(),
                },
            )
            .map_err(|_| TheRecordCouldNotBeRead {
                what_the_reader_said: "the journalled identity is not an event identity".to_string(),
            })
        }

        /// The sequence number this entry says it carries, **read without
        /// rebuilding the whole event**, so a damaged entry can be reported at
        /// the sequence it claims.
        pub fn the_sequence_it_claims(&self) -> SequenceNumber {
            self.sequence
        }
    }
}

/// What went wrong reading a record back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheRecordCouldNotBeRead {
    /// What the reader said, verbatim.
    pub what_the_reader_said: String,
}

/// Write a record down.
pub fn write<T: Serialize>(record: &T) -> Result<Vec<u8>, TheRecordCouldNotBeRead> {
    serde_json::to_vec(record).map(|mut bytes| {
        bytes.push(b'\n');
        bytes
    })
    .map_err(|why| TheRecordCouldNotBeRead {
        what_the_reader_said: why.to_string(),
    })
}

/// Read a record back.
///
/// **A record that does not read back is a refusal and never a default.** G31a's
/// never-held answer is an *absent* record, which the store above answers with
/// nothing at all; a record that is there and unreadable is a different thing
/// and says so.
pub fn read<T: for<'a> Deserialize<'a>>(bytes: &[u8]) -> Result<T, TheRecordCouldNotBeRead> {
    serde_json::from_slice(bytes).map_err(|why| TheRecordCouldNotBeRead {
        what_the_reader_said: why.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holds::{Dependency, HoldRecord, Origin, Release, ReleaseOutcome, Subject};
    use crate::machine_kind::MachineKind;
    use crate::package::phase::{
        a_fresh_attempts_first_write, Phase, PhaseRecord, PreviousVersion, Route, TransactionOwner,
    };
    use crate::package::release::{RetainedAdapter, VerifiedReleaseDescriptor};
    use crate::setup_state::SetupInProgressState;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the machine has randomness")
    }

    fn a_descriptor() -> VerifiedReleaseDescriptor {
        VerifiedReleaseDescriptor {
            manifest_format: "the one format".into(),
            verified_release_sequence: 41,
            requested_package_version: "0.1.0".into(),
            manifest_digest: [7u8; 32],
            artifact_identity: "the exact artifact".into(),
            artifact_byte_length: 12_345,
            artifact_digest: [9u8; 32],
        }
    }

    #[test]
    fn an_identity_comes_back_the_same_identity_with_the_same_kind() {
        let issued = an_identity(IdentityKind::Operation);
        let state = SetupInProgressState {
            the_run_that_set_it: issued.clone(),
        };
        let back: SetupInProgressState = read(&write(&state).unwrap()).unwrap();
        assert_eq!(back.the_run_that_set_it, issued);
        assert_eq!(back.the_run_that_set_it.kind(), IdentityKind::Operation);
    }

    /// **The kind is carried rather than assumed.** Two identities with the
    /// same bytes and different kinds are different identities, and a form that
    /// dropped the kind would read one back as the other.
    #[test]
    fn the_kind_is_carried_so_two_identities_are_not_confused() {
        let job = an_identity(IdentityKind::Job);
        let written = write(&SetupInProgressState {
            the_run_that_set_it: job.clone(),
        })
        .unwrap();
        let back: SetupInProgressState = read(&written).unwrap();
        assert_eq!(back.the_run_that_set_it.kind(), IdentityKind::Job);
        assert_ne!(
            back.the_run_that_set_it.kind(),
            IdentityKind::Operation,
            "the kind must not be assumed from the field"
        );
    }

    #[test]
    fn a_text_that_is_not_an_identity_is_refused_rather_than_made_up() {
        let broken = br#"{"the_run_that_set_it":{"kind":"Run","text":"not sixteen bytes"}}"#;
        let refused = read::<SetupInProgressState>(broken).expect_err("it must not read back");
        assert!(refused.what_the_reader_said.contains("identity"));
    }

    /// **A recorded moment that does not rebuild is a refusal rather than the
    /// end of the supervisor.**
    ///
    /// A hold record's release moment is read at a supervisor start, and a
    /// figure that runs past what a moment can hold used to stop the program
    /// there. S14 is about a supervisor that keeps answering and refuses loudly.
    ///
    /// **The figures below are the ones a desk can pose.** A nanoseconds field
    /// past a whole second is refused outright, and it is the case that used to
    /// take the subtraction past zero and stop the program. **The addition and
    /// the subtraction themselves are in their checked forms too**, and whether
    /// either can be made to overflow depends on how wide the machine's own
    /// moment is — which the desk cannot pose and this test does not claim to.
    #[test]
    fn a_recorded_moment_that_does_not_rebuild_is_refused_rather_than_ending_the_supervisor() {
        for (seconds, nanoseconds) in [
            (-1i64, 4_000_000_000u32),
            (0, 4_000_000_000),
            (i64::MIN, 1_000_000_000),
            (i64::MAX, u32::MAX),
        ] {
            let written = format!(
                r#"{{"hold":{{"kind":"Operation","text":"{}"}},"subject":{{"Job":{{"kind":"Job","text":"{}"}}}},"reason":"a reason","origin":"Submission","dependency":null,"release":{{"outcome":"Released","produced_by":"an answer","at":{{"seconds_from_the_usual_epoch":{seconds},"nanoseconds":{nanoseconds},"clock":"ThisComputer"}}}}}}"#,
                an_identity(IdentityKind::Operation).as_text(),
                an_identity(IdentityKind::Job).as_text(),
            );
            let refused = read::<HoldRecord>(written.as_bytes())
                .expect_err("the moment does not rebuild, so it is refused");
            assert!(
                !refused.what_the_reader_said.is_empty(),
                "the reader says what it refused, at {seconds}.{nanoseconds}"
            );
        }
    }

    #[test]
    fn a_whole_phase_record_comes_back_field_for_field() {
        let mut record = a_fresh_attempts_first_write(
            an_identity(IdentityKind::Operation),
            a_descriptor(),
            Some(PreviousVersion {
                descriptor: a_descriptor(),
                retained_adapter: RetainedAdapter {
                    declared_interval: (2, 3),
                    declared_request_set: vec!["inspect".into(), "stop".into()],
                },
                location: "/smoozy/package/previous".into(),
                previous_boot_registration_content: b"the previous hook".to_vec(),
            }),
            MachineKind::Container,
            "/smoozy/package/staging".into(),
        );
        record.owner = Some(TransactionOwner {
            invocation: an_identity(IdentityKind::Operation),
            route: Route::SetupPlacement,
            process: (1493, 19186),
            installation: an_identity(IdentityKind::Installation),
        });
        record.phase = Phase::CommittedUnverified;
        record.staged_packages_retained_adapters = vec![RetainedAdapter {
            declared_interval: (3, 4),
            declared_request_set: vec!["release".into()],
        }];
        record.deferred_because = Some("a hold stands".into());

        let back: PhaseRecord = read(&write(&record).unwrap()).unwrap();
        assert_eq!(back, record);
    }

    #[test]
    fn a_hold_record_comes_back_with_its_release_moment_and_clock() {
        let record = HoldRecord {
            hold: an_identity(IdentityKind::Operation),
            subject: Subject::Job(an_identity(IdentityKind::Job)),
            reason: "the first frame has not arrived".into(),
            origin: Origin::Submission,
            dependency: Some(Dependency {
                work: an_identity(IdentityKind::Job),
                the_predicate_the_placing_document_declared: "Row 54".into(),
            }),
            release: Some(Release {
                outcome: ReleaseOutcome::Released,
                produced_by: "an answer, over ThePhone".into(),
                at: Reading::taken(
                    (),
                    UNIX_EPOCH + Duration::new(1_777_000_000, 123_456_789),
                    Clock::AMachine(MachineClock("that machine".into())),
                ),
            }),
        };
        let back: HoldRecord = read(&write(&record).unwrap()).unwrap();
        assert_eq!(back, record);
    }

    /// **A machine's clock is its own and nothing here corrects one**, so a
    /// moment before the usual epoch is carried rather than refused.
    #[test]
    fn a_moment_before_the_epoch_is_carried_rather_than_refused() {
        for moment in [
            UNIX_EPOCH - Duration::new(5, 0),
            UNIX_EPOCH - Duration::new(5, 400_000_000),
            UNIX_EPOCH,
            UNIX_EPOCH + Duration::new(1, 1),
        ] {
            let record = HoldRecord {
                hold: an_identity(IdentityKind::Operation),
                subject: Subject::CommandAwaitingAPerson(an_identity(IdentityKind::Operation)),
                reason: "a reason the placing document declared".into(),
                origin: Origin::RenderStart,
                dependency: None,
                release: Some(Release {
                    outcome: ReleaseOutcome::Dropped,
                    produced_by: "its dependency ended".into(),
                    at: Reading::taken((), moment, Clock::ThisComputer),
                }),
            };
            let back: HoldRecord = read(&write(&record).unwrap()).unwrap();
            let came_back: SystemTime = back.release.as_ref().unwrap().at.taken_at();
            assert_eq!(came_back, moment, "the moment {moment:?} did not come back");
        }
    }
}

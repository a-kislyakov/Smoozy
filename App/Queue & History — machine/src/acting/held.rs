//! **§3 H2, H4 and H11 performed — the desk's answer as a command over the
//! connection, the machine's serialization of two answers, and the drop
//! event's journalling and application.**
//!
//! # The mechanism is `Rented-server runtime` §8's and none of it is written
//! again
//!
//! **There is one hold mechanism and no second waiting mechanism.** The hold
//! record, the two origins, the one release path, first-recorded-wins, the
//! loser being told what happened, and a *no* after a *yes* never becoming a
//! stop are all [`smoozy_runtime::holds`]'s and are called rather than
//! restated. What this module adds is three things and no more:
//!
//! - **the desk's answer composed as one ordinary command** with its own
//!   operation identity, naming the hold identity and carrying the answer;
//! - **the serialized read-apply-write on the machine**, with H3's placement
//!   written in the same act as the release;
//! - **the release predicate**, which is this document's own and is supplied
//!   into [`smoozy_runtime::holds::evaluate`] rather than decided there.
//!
//! # The predicate's reading, and why it is decoded here
//!
//! **`Rented-server runtime` U30 carries the reading opaquely** — *"nothing in
//! this crate reads inside it"* — and Row 54's own shape is this document's.
//! So the bytes carried with the dependency's outcome are decoded here into
//! [`crate::waiting::CompletenessReading`], and [`crate::waiting::the_predicate`]
//! is what answers. **A reading that does not decode is an absent reading and
//! never an empty one**, which is exactly the difference H8 turns on: no drop
//! is written against absent evidence, because a dependency that ran and was
//! given up may have written its first frame and the drop is irreversible
//! under `Rented-server runtime` U2's write-once rule.
//!
//! # What is not here
//!
//! **The phone's route is `Telegram`'s** and arrives at the same supervisor
//! command path; the two differ in how they reach the machine and in nothing
//! after that. **Nothing here is a second command path.**
//!
//! **The journal is not written here.** Each act hands back what is
//! journalled, and the journal is `Rented-server runtime`'s store under its
//! own single writer.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` U2, U9, U10, U12, U13,
//! U14, U15, U19, U19a, U20, U22, U30, S1. `State, identity and recovery` §7.
//!
//! **`Server protocol and transfers` C4 is deliberately not on that list and
//! is written out at its site instead**, because this plan defines a C4 of its
//! own.

use std::time::SystemTime;

use smoozy_runtime::holds::{
    self, Answer, AnswerRoute, HoldRecord, KeepsHoldRecords, PredicateMoment, Release,
    ReleaseOutcome, Subject, TheReadingCarriedWithTheEnding, WhatAnAnswerProduces,
    WhatTheAnswerSays,
};
use smoozy_framing::framing::{Fields, Request};
use smoozy_state::identities::Identity;
use smoozy_state::observed::Clock;

use crate::acting::commands::{the_request_for, AQueueCommand};
use crate::acting::queue_store::{TheQueueOnTheMachine, WhatTheStoreSaid};
use crate::waiting::{CompletenessReading, HowTheDependencyEnded, TheMoment, WhyDropped};

/// **H2: the desk's answer, composed as one ordinary command with its own
/// operation identity, naming the hold identity and carrying the answer.**
///
/// **It is subject to `Server protocol and transfers` C4 like any other**, so
/// the caller records the identity durably before it acts and a repeat of that
/// identity never acts again — on the machine, in the one command store the
/// program keeps (`smoozy-state`'s command record).
///
/// **The phone's route is `Rented-server runtime` U10's and reaches the same
/// supervisor command path.** Nothing here composes a second one.
pub fn the_desks_answer(
    operation: Identity,
    hold: Identity,
    says: WhatTheAnswerSays,
) -> (Answer, Request) {
    let request = the_request_for(
        AQueueCommand::AnswerAHold,
        operation.clone(),
        Fields::none().and("hold", hold.as_text().into_bytes()).and(
            "answer",
            match says {
                WhatTheAnswerSays::Yes => b"yes".to_vec(),
                WhatTheAnswerSays::No => b"no".to_vec(),
            },
        ),
    );
    (
        Answer {
            operation,
            hold,
            says,
            route: AnswerRoute::TheDesk,
        },
        request,
    )
}

/// What applying an answer left behind, from the queue's side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheAnswerLeft {
    /// **U12: the release outcome was written, once, and the write is what
    /// releases the subject.** Where the answer was *yes*, H3's placement was
    /// written in the same act and the job is at the end of the line.
    TheReleaseWasWritten {
        release: Release,
        /// **H3: the released job's new place**, written in the same act as
        /// the release. Absent where the answer was *no*.
        placed_in_the_line: bool,
    },
    /// **U13 and U14: the first answer recorded won and this one acts on
    /// nothing.** The caller is told which answer was applied and when — a
    /// recorded outcome rather than a failure.
    ///
    /// **U15: a *no* arriving after a *yes* is not converted into a stop.**
    /// Nothing here turns this into one, and the ordinary stop stays offered
    /// on the now-running entry.
    ItActedOnNothingAndCarriesTheRecordedOutcome(Release),
}

/// **H4, performed: read, apply, write — serialized on the machine that holds
/// the hold record, because that is where both routes end.**
///
/// **The single writer is what makes first-recorded-wins true rather than
/// hoped for** (`Rented-server runtime` S1): this function takes `&mut` of
/// both stores, so two answers cannot be applied at once inside one process
/// either.
///
/// **H3's placement rides the release's own write, and inside that act the
/// placement goes first.** *"The released subject enters the line behind
/// whatever is waiting at the moment the release outcome is written, and the
/// supervisor writes that placement in the same act as the release."* **The
/// release outcome is written last** because it is write-once (`Rented-server
/// runtime` U2) and is what stops the entry offering the answers: a refusal
/// then leaves the hold unreleased with both answers standing, rather than a
/// job the entry will no longer offer and the admission point will never
/// consider. **The place is not the front and not its old one**, and the
/// moment is the release outcome's write, on the machine, so it is right
/// whether the answer came from the desk, from a phone, or hours after Smoozy
/// closed.
pub fn apply_the_answer(
    holds_on_the_machine: &dyn KeepsHoldRecords,
    queue: &mut TheQueueOnTheMachine,
    hold: &Identity,
    answer: &Answer,
    at: SystemTime,
    on: Clock,
) -> Result<WhatTheAnswerLeft, WhatTheStoreSaid> {
    let record = holds_on_the_machine
        .read(hold)
        .map_err(|why| WhatTheStoreSaid::ItWasRefused { why })?
        .ok_or_else(|| WhatTheStoreSaid::ItIsNotAJobRecord { at: hold.as_text() })?;
    match holds::apply(&record, answer, at, on) {
        WhatAnAnswerProduces::ActsOnNothingAndCarriesTheRecordedOutcome(already) => {
            Ok(WhatTheAnswerLeft::ItActedOnNothingAndCarriesTheRecordedOutcome(already))
        }
        WhatAnAnswerProduces::TheReleaseOutcomeIsWritten(release) => {
            // **H3's placement runs first and the release outcome is written
            // last.** The release outcome is write-once (`Rented-server
            // runtime` U2) and is what the desktop reads to stop offering the
            // answers, so a refusal of the placement has to leave the hold
            // unreleased with both answers still on the entry — something the
            // user can act on. Written the other way round, a refused
            // placement left a job the entry would no longer offer and the
            // admission point would never consider: silent, and the machine
            // goes quiet with the job sitting in it.
            let placed = match (release.outcome, subject_of(&record)) {
                (ReleaseOutcome::Released, Some(job)) => place_the_released_subject(queue, &job)?,
                _ => false,
            };
            holds_on_the_machine
                .write_the_release(hold, &release)
                .map_err(|why| WhatTheStoreSaid::ItWasRefused { why })?;
            Ok(WhatTheAnswerLeft::TheReleaseWasWritten {
                release,
                placed_in_the_line: placed,
            })
        }
    }
}

/// The job a hold record's subject names.
///
/// **`Rented-server runtime` U2's subject kind admits a command as well as a
/// job, and no document today places one.** A subject that is not a job takes
/// no place in this document's line, so there is nothing here to place — which
/// is the honest answer rather than a case invented for it.
fn subject_of(record: &HoldRecord) -> Option<Identity> {
    match &record.subject {
        Subject::Job(job) => Some(job.clone()),
        Subject::CommandAwaitingAPerson(_) => None,
    }
}

/// **H3, performed: the released record takes state *waiting* and a sequence
/// key greater than every key then in that machine's line.**
///
/// **The line is read at this moment and never earlier**, which is what makes
/// *behind whatever is waiting at the moment the answer comes* true.
fn place_the_released_subject(
    queue: &mut TheQueueOnTheMachine,
    job: &Identity,
) -> Result<bool, WhatTheStoreSaid> {
    let (records, _) = queue.every_record()?;
    let Some(mut released) = queue.one_record(job) else {
        return Ok(false);
    };
    // **A record already in the line is already placed.** The placement now
    // runs before the release is written, so a refusal of that write leaves
    // both answers standing and a second answer reaches here again; placing a
    // record twice would move it behind whatever joined in between.
    if !matches!(released.state, crate::line::QueueState::Held { .. }) {
        return Ok(true);
    }
    crate::line::the_released_placement(&mut released, &records);
    queue.write(&released)?;
    Ok(true)
}

/// **The reading carried with a dependency's ending, decoded into this
/// document's own shape.**
///
/// **A reading that does not decode is an absent reading**, never an empty
/// one — H8 turns on the difference and four empty sets would destroy it.
pub fn decode_the_carried_reading(
    carried: &TheReadingCarriedWithTheEnding,
) -> Option<CompletenessReading> {
    let written: TheReadingAsWritten =
        serde_json::from_slice(&carried.what_the_dependencys_own_worker_took).ok()?;
    Some(CompletenessReading {
        frames_already_there: written.frames_already_there,
        the_shape: if written.the_inventory_is_video_shaped {
            crate::waiting::TheInventorysShape::Video
        } else {
            crate::waiting::TheInventorysShape::Frames
        },
    })
}

/// **The written form the reading travels in, and it is Build's own choice.**
///
/// **The frames are in the job's own asked order as the manifest records
/// them** (N6's order), which is the order this document's shape is defined
/// over — so the order is part of the value rather than something a reader
/// re-imposes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TheReadingAsWritten {
    pub frames_already_there: Vec<i64>,
    pub the_inventory_is_video_shaped: bool,
}

impl TheReadingAsWritten {
    /// The reading, ready to be carried with a recorded outcome.
    pub fn carried(&self) -> TheReadingCarriedWithTheEnding {
        TheReadingCarriedWithTheEnding {
            what_the_dependencys_own_worker_took: serde_json::to_vec(self)
                .expect("a reading is composed of values that always render"),
        }
    }
}

/// **What a hold's evaluation left, from the queue's side.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheEvaluationLeft {
    /// **U20 and H11: *dropped* was written with the dependency's ending as
    /// the reason, and the drop is journalled.**
    ItWasDropped { release: Release, why: WhyDropped },
    /// The hold stands with its own release path — the user's answer — still
    /// the route.
    TheHoldStands,
    /// **H8: no reading was carried and the dependency did run, so the
    /// predicate was not evaluated.** No drop is written.
    ItWasNotEvaluated { ending: HowTheDependencyEnded },
}

/// **H7 to H9 performed at `Rented-server runtime` U19's and U19a's two
/// moments, with this document's own predicate supplied into that crate's
/// evaluation.**
///
/// **The moment is that crate's and the answer is this one's.** `evaluate`
/// decides *whether the predicate runs at all* — a hold with no dependency, or
/// one already released, never reaches it — and the predicate handed in is
/// [`crate::waiting::the_predicate`] over the decoded reading.
pub fn evaluate_the_predicate(
    record: &HoldRecord,
    moment: PredicateMoment,
    ending: HowTheDependencyEnded,
    carried: Option<&TheReadingCarriedWithTheEnding>,
    at: SystemTime,
    on: Clock,
) -> WhatTheEvaluationLeft {
    evaluated_with(record, moment, ending, carried, at, on, crate::waiting::the_predicate)
}

/// **The same, with the predicate handed in** — so the desk can prove which
/// holds it is never asked for.
///
/// **A hold with no dependency, or one already released, never reaches the
/// predicate at all**, and that is asked here before the predicate is run
/// rather than after. *Until held-job piece A, 2026-10-12, the predicate was
/// run first and its answer thrown away for such a hold*: harmless while every
/// hold placed at submission named a dependency, and not what `Rendering`
/// S18a's hold for the driver — no dependency and no release predicate — may
/// rest on.
fn evaluated_with(
    record: &HoldRecord,
    moment: PredicateMoment,
    ending: HowTheDependencyEnded,
    carried: Option<&TheReadingCarriedWithTheEnding>,
    at: SystemTime,
    on: Clock,
    the_predicate: impl FnOnce(&TheMoment) -> crate::waiting::WhatThePredicateGives,
) -> WhatTheEvaluationLeft {
    if record.dependency.is_none() || record.release.is_some() {
        return WhatTheEvaluationLeft::TheHoldStands;
    }
    // **The moment gate is `Rented-server runtime` U19's**, and it is asked
    // first so a hold with no dependency is never evaluated at all.
    let ours = match moment {
        PredicateMoment::WhenTheDependencysJobRecordIsRemoved => {
            TheMoment::TheDependencysJobRecordIsRemoved
        }
        PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten => {
            TheMoment::TheDependencysRecordedOutcomeIsWritten {
                ending,
                carried: carried.and_then(decode_the_carried_reading),
            }
        }
    };
    let ours = the_predicate(&ours);
    let theirs = holds::evaluate(record, moment, carried, |_| {
        matches!(ours, crate::waiting::WhatThePredicateGives::Drop(_))
    });
    match theirs {
        holds::WhatThePredicateGives::Drop => match ours {
            crate::waiting::WhatThePredicateGives::Drop(why) => {
                WhatTheEvaluationLeft::ItWasDropped {
                    release: Release {
                        outcome: ReleaseOutcome::Dropped,
                        produced_by: match why {
                            WhyDropped::TheDependencyWasCancelled => {
                                "the dependency was cancelled".to_string()
                            }
                            // **The ending in its own four words and never
                            // this type's rendering** — step 10c's fifth pass
                            // on Vast, fault 5.
                            WhyDropped::TheDependencysEnding(ending) => {
                                format!("the dependency {}", ending.in_words())
                            }
                        },
                        at: smoozy_state::observed::Reading::taken((), at, on),
                    },
                    why,
                }
            }
            // **Unreachable by construction**: the predicate handed in is the
            // one asked above, so a drop there is a drop here.
            _ => WhatTheEvaluationLeft::TheHoldStands,
        },
        // **H8's *not evaluated* is a statement about why the hold stands, and
        // it is only that where the hold has a live dependency**: a record with
        // none, or one already released, never reached the predicate at all.
        holds::WhatThePredicateGives::TheHoldStands => match ours {
            crate::waiting::WhatThePredicateGives::NotEvaluatedNoReadingWasCarried { ending }
                if record.dependency.is_some() && record.release.is_none() =>
            {
                WhatTheEvaluationLeft::ItWasNotEvaluated { ending }
            }
            _ => WhatTheEvaluationLeft::TheHoldStands,
        },
    }
}

/// **H11, performed: the dropped waiter leaves nothing anywhere.**
///
/// **The subject's machine-side record and its material go first, by the same
/// enumeration as a cancel (L1), and the release outcome is written last.**
/// Inside the cancel the order is L1's for L1's reason: the record's removal
/// is the deciding write, and after it the job cannot start. **Across the two
/// it is the release that goes last**, because the release outcome is
/// write-once (`Rented-server runtime` U2) and is what the desktop reads to
/// stop offering the answers — so a refusal leaves the hold unreleased with
/// both answers still on the entry, which the user can act on, rather than a
/// released hold over a job nothing will ever consider.
///
/// **The user is told why, once**, and the telling is the catch-up notice or a
/// *now* notification per `Desktop app and delivery` T18's boundary — composed
/// from what this function hands back, never from a store of its own. **On the
/// desktop the provisional record is removed when the event is applied**
/// (`State, identity and recovery` §7 steps 2 and 3), so a job that never
/// rendered leaves no trace.
pub fn apply_the_drop(
    holds_on_the_machine: &dyn KeepsHoldRecords,
    queue: &mut TheQueueOnTheMachine,
    hold: &Identity,
    subject: &Identity,
    release: &Release,
    where_it_sits: &crate::acting::commands::WhereTheMaterialSits,
) -> Result<crate::acting::commands::ACancelsOutcome, WhatTheStoreSaid> {
    // **The cancel runs first and the release outcome is written last**, for
    // the reason [`apply_the_answer`] gives: the release outcome is write-once
    // and is what stops the entry offering the answers, so a refusal has to
    // leave the hold unreleased with both answers still on the entry. **The
    // cancel treats an already-absent record as done**, so a second answer
    // after a refusal is safe.
    let outcome = crate::acting::commands::apply_a_cancel(queue, subject, where_it_sits)?;
    holds_on_the_machine
        .write_the_release(hold, release)
        .map_err(|why| WhatTheStoreSaid::ItWasRefused { why })?;
    Ok(outcome)
}

/// **`Rented-server runtime` U22's third ending, kept as a value: a hold ends
/// when its subject is removed, and the record goes with it.**
///
/// **So L5's cancel of a held job needs no second mechanism**, and X2's
/// enumeration accounts the hold record as its own unit.
pub const A_HOLD_ENDS_WITH_ITS_SUBJECT: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::{JobRecord, QueueState, SequenceKey};
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::observed::MachineClock;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn an_operation() -> Identity {
        Identity::issue(IdentityKind::Operation).expect("randomness")
    }

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    fn a_clock() -> Clock {
        Clock::AMachine(MachineClock("server-1".to_string()))
    }

    /// A hold-record keeper in hand, so what is under test is the queue's own
    /// side rather than that crate's store.
    ///
    /// **`Rented-server runtime` U2's write-once rule is kept here too**, so a
    /// second release cannot be written past the first even in a test.
    #[derive(Default)]
    struct HoldsInHand(std::cell::RefCell<HashMap<String, HoldRecord>>);

    impl KeepsHoldRecords for HoldsInHand {
        fn place(&self, record: &HoldRecord) -> Result<(), String> {
            self.0
                .borrow_mut()
                .insert(record.hold.as_text(), record.clone());
            Ok(())
        }

        fn write_the_release(&self, hold: &Identity, release: &Release) -> Result<(), String> {
            let mut held = self.0.borrow_mut();
            let record = held
                .get_mut(&hold.as_text())
                .ok_or_else(|| "no hold record stands".to_string())?;
            if record.release.is_some() {
                return Err("the release outcome is already written".to_string());
            }
            record.release = Some(release.clone());
            Ok(())
        }

        fn read(&self, hold: &Identity) -> Result<Option<HoldRecord>, String> {
            Ok(self.0.borrow().get(&hold.as_text()).cloned())
        }
    }

    fn a_root(named: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("smoozy-queue-held-{}-{named}", std::process::id()));
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

    /// **H2: the answer is one ordinary command with its own operation
    /// identity, naming the hold and carrying the answer.**
    #[test]
    fn the_desks_answer_is_one_command_naming_the_hold_and_carrying_the_answer() {
        let operation = an_operation();
        let hold = an_operation();
        let (answer, request) =
            the_desks_answer(operation.clone(), hold.clone(), WhatTheAnswerSays::Yes);
        assert_eq!(answer.operation, operation);
        assert_eq!(answer.hold, hold);
        assert_eq!(answer.route, AnswerRoute::TheDesk);
        assert_eq!(request.name, smoozy_framing::requests::THE_ANSWER_TO_A_HOLD_REQUEST);
        assert_eq!(request.fields.text("hold"), Some(hold.as_text().as_str()));
        assert_eq!(request.fields.text("answer"), Some("yes"));
        let (_, no) = the_desks_answer(an_operation(), hold, WhatTheAnswerSays::No);
        assert_eq!(no.fields.text("answer"), Some("no"));
    }

    /// **The reading round-trips into this document's own shape, and a reading
    /// that does not decode is absent rather than empty.**
    #[test]
    fn the_carried_reading_decodes_into_this_documents_shape_and_a_bad_one_is_absent() {
        let written = TheReadingAsWritten {
            frames_already_there: vec![1, 2, 3],
            the_inventory_is_video_shaped: false,
        };
        let back = decode_the_carried_reading(&written.carried()).expect("decoded");
        assert_eq!(back.frames_already_there, vec![1, 2, 3]);
        assert!(!back.no_frame_counts_as_already_there());
        assert_eq!(
            decode_the_carried_reading(&TheReadingCarriedWithTheEnding {
                what_the_dependencys_own_worker_took: b"not a reading".to_vec()
            }),
            None,
            "an unreadable reading is absent, never four empty sets"
        );
    }

    fn a_hold_with_a_dependency(subject: &Identity, dependency: &Identity) -> HoldRecord {
        HoldRecord {
            hold: an_operation(),
            subject: Subject::Job(subject.clone()),
            reason: "the first-frame check".to_string(),
            origin: smoozy_runtime::holds::Origin::Submission,
            dependency: Some(smoozy_runtime::holds::Dependency {
                work: dependency.clone(),
                the_predicate_the_placing_document_declared:
                    crate::waiting::THE_PREDICATE_H7_DECLARES.to_string(),
            }),
            release: None,
        }
    }

    /// **H7 and H8: an ending with no frame in its carried reading drops the
    /// waiter; a reading showing a frame leaves the hold standing; and an
    /// outcome carrying no reading is not evaluated at all.**
    #[test]
    fn the_predicate_drops_on_no_frame_stands_on_a_frame_and_is_not_evaluated_on_no_reading() {
        let subject = a_job();
        let dependency = a_job();
        let record = a_hold_with_a_dependency(&subject, &dependency);
        let none = TheReadingAsWritten {
            frames_already_there: Vec::new(),
            the_inventory_is_video_shaped: false,
        }
        .carried();
        let some = TheReadingAsWritten {
            frames_already_there: vec![7],
            the_inventory_is_video_shaped: false,
        }
        .carried();
        let at = SystemTime::UNIX_EPOCH;

        let dropped = evaluate_the_predicate(
            &record,
            PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten,
            HowTheDependencyEnded::GivenUp,
            Some(&none),
            at,
            a_clock(),
        );
        assert!(matches!(
            dropped,
            WhatTheEvaluationLeft::ItWasDropped {
                why: WhyDropped::TheDependencysEnding(HowTheDependencyEnded::GivenUp),
                ..
            }
        ));

        assert_eq!(
            evaluate_the_predicate(
                &record,
                PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten,
                HowTheDependencyEnded::Finished,
                Some(&some),
                at,
                a_clock(),
            ),
            WhatTheEvaluationLeft::TheHoldStands
        );

        // **H8: no reading carried and the dependency did run.**
        assert_eq!(
            evaluate_the_predicate(
                &record,
                PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten,
                HowTheDependencyEnded::GivenUp,
                None,
                at,
                a_clock(),
            ),
            WhatTheEvaluationLeft::ItWasNotEvaluated {
                ending: HowTheDependencyEnded::GivenUp
            }
        );
    }

    /// **H9: a dependency that never ran is a dependency that produced no
    /// frame, and the drop fires at the removal of its job record.**
    #[test]
    fn a_cancelled_dependency_drops_the_waiter_at_the_removal_of_its_record() {
        let record = a_hold_with_a_dependency(&a_job(), &a_job());
        let left = evaluate_the_predicate(
            &record,
            PredicateMoment::WhenTheDependencysJobRecordIsRemoved,
            HowTheDependencyEnded::Failed,
            None,
            SystemTime::UNIX_EPOCH,
            a_clock(),
        );
        assert!(matches!(
            left,
            WhatTheEvaluationLeft::ItWasDropped {
                why: WhyDropped::TheDependencyWasCancelled,
                ..
            }
        ));
    }

    /// **`Rendering` S18a's hold for the driver names no dependency, and no
    /// predicate is ever run for it** — at either of `Rented-server runtime`
    /// U19's moments, with or without a reading carried. The predicate handed
    /// in here would fail the test if it were run. Held-job piece A,
    /// 2026-10-12.
    #[test]
    fn a_hold_with_no_dependency_never_has_a_predicate_run_for_it() {
        let mut record = a_hold_with_a_dependency(&a_job(), &a_job());
        record.dependency = None;
        record.reason = "the driver".to_string();
        let carried = TheReadingAsWritten {
            frames_already_there: Vec::new(),
            the_inventory_is_video_shaped: false,
        }
        .carried();
        for moment in [
            PredicateMoment::WhenTheDependencysJobRecordIsRemoved,
            PredicateMoment::WhenTheDependencysRecordedOutcomeIsWritten,
        ] {
            for reading in [None, Some(&carried)] {
                let left = evaluated_with(
                    &record,
                    moment,
                    HowTheDependencyEnded::Failed,
                    reading,
                    SystemTime::UNIX_EPOCH,
                    a_clock(),
                    |_| panic!("a predicate was run for a hold with no dependency"),
                );
                assert_eq!(left, WhatTheEvaluationLeft::TheHoldStands);
            }
        }
        // **And the public route gives the same.**
        assert_eq!(
            evaluate_the_predicate(
                &record,
                PredicateMoment::WhenTheDependencysJobRecordIsRemoved,
                HowTheDependencyEnded::Failed,
                None,
                SystemTime::UNIX_EPOCH,
                a_clock(),
            ),
            WhatTheEvaluationLeft::TheHoldStands
        );
    }

    /// **H11: the dropped waiter leaves nothing anywhere** — the record goes
    /// first, then all four units of L1's enumeration.
    #[test]
    fn a_dropped_waiter_leaves_no_record_and_no_material() {
        let root = a_root("drop");
        let mut queue = TheQueueOnTheMachine::at(&root);
        let subject = a_job();
        let mut held = a_record(&subject, 10);
        held.state = QueueState::Held {
            hold: an_operation(),
        };
        held.sequence_key = None;
        queue.write(&held).expect("written");

        let where_it_sits = crate::acting::commands::WhereTheMaterialSits {
            job_record_directory: root.join("records"),
            remap_material: root.join("remaps"),
            per_job_note: root.join("note.json"),
            job_output_folder: Some(root.join("outputs")),
            reporting_choice_now: None,
        };
        std::fs::create_dir_all(&where_it_sits.job_record_directory).unwrap();
        std::fs::write(&where_it_sits.per_job_note, b"{}").unwrap();

        let outcome = crate::acting::commands::apply_a_cancel(&mut queue, &subject, &where_it_sits)
            .expect("dropped");
        assert!(outcome.the_record_was_removed);
        assert_eq!(queue.one_record(&subject), None);
        assert!(!where_it_sits.job_record_directory.exists());
        assert!(!where_it_sits.per_job_note.exists());
        const _: () = assert!(A_HOLD_ENDS_WITH_ITS_SUBJECT);
        std::fs::remove_dir_all(&root).ok();
    }

    /// **H4: the first answer recorded wins, the second acts on nothing, and
    /// a *no* after a *yes* is not converted into a stop.**
    #[test]
    fn the_first_answer_recorded_wins_and_the_second_acts_on_nothing() {
        let root = a_root("two-answers");
        let mut queue = TheQueueOnTheMachine::at(&root);
        let subject = a_job();
        let holds_on_the_machine = HoldsInHand::default();
        let record = a_hold_with_a_dependency(&subject, &a_job());
        let hold = record.hold.clone();
        holds_on_the_machine.place(&record).expect("placed");

        let mut held = a_record(&subject, 1);
        held.state = QueueState::Held { hold: hold.clone() };
        held.sequence_key = None;
        queue.write(&held).expect("written");

        let (yes, _) = the_desks_answer(an_operation(), hold.clone(), WhatTheAnswerSays::Yes);
        let first = apply_the_answer(
            &holds_on_the_machine,
            &mut queue,
            &hold,
            &yes,
            SystemTime::UNIX_EPOCH,
            a_clock(),
        )
        .expect("applied");
        let WhatTheAnswerLeft::TheReleaseWasWritten {
            release,
            placed_in_the_line,
        } = first
        else {
            panic!("the first answer wins")
        };
        assert_eq!(release.outcome, ReleaseOutcome::Released);
        assert!(placed_in_the_line, "H3's placement rides the same act");
        assert!(queue.one_record(&subject).expect("there").is_waiting());

        let (no, _) = the_desks_answer(an_operation(), hold.clone(), WhatTheAnswerSays::No);
        let second = apply_the_answer(
            &holds_on_the_machine,
            &mut queue,
            &hold,
            &no,
            SystemTime::UNIX_EPOCH,
            a_clock(),
        )
        .expect("applied");
        let WhatTheAnswerLeft::ItActedOnNothingAndCarriesTheRecordedOutcome(already) = second
        else {
            panic!("the second acts on nothing")
        };
        // **The loser is told which answer was applied and when.**
        assert_eq!(already.outcome, ReleaseOutcome::Released);
        // **U15: the *no* is not converted into a stop** — the job is still
        // waiting and nothing wrote *stopped*.
        assert!(queue.one_record(&subject).expect("there").is_waiting());
        std::fs::remove_dir_all(&root).ok();
    }

    /// **The release outcome is written last, so a refused placement leaves
    /// the hold unreleased with both answers still on the entry.**
    ///
    /// **The fault is introduced where a full mount puts it**: the queue
    /// store's own directory is a plain file, so every write in it refuses.
    /// Written the other way round the hold would read released — the entry
    /// would stop offering the answers — over a job still held, which the
    /// admission point never considers and nothing anywhere can act on.
    #[test]
    fn a_refused_placement_leaves_the_hold_unreleased_and_both_answers_standing() {
        let root = a_root("placement-refused");
        std::fs::write(root.join("gone"), b"not a directory").unwrap();
        let mut queue = TheQueueOnTheMachine::at(root.join("gone"));
        let subject = a_job();
        let holds_on_the_machine = HoldsInHand::default();
        let record = a_hold_with_a_dependency(&subject, &a_job());
        let hold = record.hold.clone();
        holds_on_the_machine.place(&record).expect("placed");

        let (yes, _) = the_desks_answer(an_operation(), hold.clone(), WhatTheAnswerSays::Yes);
        let refused = apply_the_answer(
            &holds_on_the_machine,
            &mut queue,
            &hold,
            &yes,
            SystemTime::UNIX_EPOCH,
            a_clock(),
        );
        assert!(refused.is_err(), "the placement refused");
        assert!(
            holds_on_the_machine
                .read(&hold)
                .expect("read")
                .expect("there")
                .release
                .is_none(),
            "the hold is unreleased and both answers are still offered"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **H11's drop writes the release last too**, so a refused cancel leaves
    /// the hold unreleased rather than a released hold over a job that is
    /// still there.
    ///
    /// **The cancel is made to refuse the one way every system agrees on** —
    /// [`crate::acting::queue_store::a_store_whose_record_cannot_be_removed`],
    /// which says why.
    #[test]
    fn a_refused_cancel_inside_a_drop_leaves_the_hold_unreleased() {
        let root = a_root("drop-refused");
        let subject = a_job();
        let mut queue =
            crate::acting::queue_store::a_store_whose_record_cannot_be_removed(&root, &subject);
        let holds_on_the_machine = HoldsInHand::default();
        let record = a_hold_with_a_dependency(&subject, &a_job());
        let hold = record.hold.clone();
        holds_on_the_machine.place(&record).expect("placed");
        let where_it_sits = crate::acting::commands::WhereTheMaterialSits {
            job_record_directory: root.join("records"),
            remap_material: root.join("remaps"),
            per_job_note: root.join("note.json"),
            job_output_folder: Some(root.join("outputs")),
            reporting_choice_now: None,
        };
        std::fs::create_dir_all(&where_it_sits.job_record_directory).unwrap();
        let release = Release {
            outcome: ReleaseOutcome::Dropped,
            produced_by: "the dependency was cancelled".to_string(),
            at: smoozy_state::observed::Reading::taken((), SystemTime::UNIX_EPOCH, a_clock()),
        };
        assert!(apply_the_drop(
            &holds_on_the_machine,
            &mut queue,
            &hold,
            &subject,
            &release,
            &where_it_sits,
        )
        .is_err());
        assert!(
            holds_on_the_machine
                .read(&hold)
                .expect("read")
                .expect("there")
                .release
                .is_none(),
            "the hold is unreleased"
        );
        assert!(
            where_it_sits.job_record_directory.exists(),
            "and the cancel's own order held: nothing of the material was touched"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// **H3: the released subject enters the line behind whatever is waiting
    /// at the moment the release outcome is written** — not the front, and
    /// not its old place.
    #[test]
    fn a_released_subject_joins_the_end_of_the_line_as_it_stands_at_the_release() {
        let root = a_root("release");
        let mut queue = TheQueueOnTheMachine::at(&root);
        let released = a_job();
        let mut held = a_record(&released, 1);
        held.state = QueueState::Held {
            hold: an_operation(),
        };
        held.sequence_key = None;
        queue.write(&held).expect("written");
        // Two jobs joined the line while it waited.
        let first = a_job();
        let second = a_job();
        queue.write(&a_record(&first, 100)).expect("written");
        queue.write(&a_record(&second, 200)).expect("written");

        assert!(place_the_released_subject(&mut queue, &released).expect("placed"));
        let (records, _) = queue.every_record().expect("read");
        let line: Vec<Identity> = crate::line::the_line(&records)
            .into_iter()
            .map(|record| record.job.clone())
            .collect();
        assert_eq!(line, vec![first, second, released]);
        std::fs::remove_dir_all(&root).ok();
    }
}

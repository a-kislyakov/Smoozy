//! Row 54's release predicate — `Queue & History` §3 H7–H10 — and what the
//! entry says about a held job (H1, H4–H6, as far as they are judgements).
//!
//! **Constraint:** the hold mechanism is `Rented-server runtime` §8's and is
//! not this document's — one hold record, two origins, one release path, no
//! second waiting mechanism. What is this document's is the predicate, the
//! moment it is evaluated, and what it reads.
//!
//! **What the predicate reads is behind [`CompletenessReading`]**, this
//! crate's own shape for what a completeness statement says about frames:
//! `Job files and outputs` C27's four sets are that document's, composed at
//! step 7 by its own reckoning, and the acting half hands the frames that
//! count as already there — under C12–C13's rule, a zero-byte file never
//! counting — into this shape. **Nothing here walks a folder.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` U2, U5, U13, U14, U15,
//! U19, U19a, U20, U21, U30, S13. `Job files and outputs` C12, C13, C15,
//! C27.

use smoozy_state::identities::Identity;

/// **H7's reading: the completeness statement's answer for the frames — which
/// count as already there.** *Already there* is `Job files and outputs` C12
/// and C13: every file the frame owes present and none of them empty; a file
/// of zero bytes never counts, wherever it sits. **The exit status decides
/// nothing here** — twenty separate endings report success, six of them
/// ending at 0 with an empty output folder — and the frame in flight is never
/// counted.
///
/// **This is a reading Smoozy's own reckoning wrote, carried with the
/// outcome, and never a reading of the folder by this crate.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletenessReading {
    /// The frames that count as already there, in the job's own asked order
    /// as the manifest records it (N6's order).
    pub frames_already_there: Vec<i64>,
    /// **H10: a video-shaped inventory is one file.** It counts only where
    /// present and not empty, and `Job files and outputs` C15's limit — a
    /// file short but not empty still counts — is carried, named, and not
    /// repaired.
    pub the_shape: TheInventorysShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheInventorysShape {
    Frames,
    /// One file. `frames_already_there` then holds the one frame the file
    /// stands for where it counts, and nothing where it does not.
    Video,
}

impl CompletenessReading {
    /// **H7's predicate, declared in full: no frame counts as already
    /// there.**
    pub fn no_frame_counts_as_already_there(&self) -> bool {
        self.frames_already_there.is_empty()
    }
}

/// How the dependency ended, as the recorded outcome says it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheDependencyEnded {
    Finished,
    Failed,
    LeftStopped,
    GivenUp,
}

impl HowTheDependencyEnded {
    /// **How the dependency ended, in standard software words** (Rule 9) —
    /// beside the type, so a reason recorded about it is a sentence and never
    /// this type's debugging rendering.
    ///
    /// **Added 2026-09-23**, out of step 10c's fifth pass on Vast, fault 5: a
    /// dropped hold's recorded reason read *the dependency's ending: `Failed`*.
    pub fn in_words(self) -> &'static str {
        match self {
            HowTheDependencyEnded::Finished => "finished",
            HowTheDependencyEnded::Failed => "failed",
            HowTheDependencyEnded::LeftStopped => "was stopped",
            HowTheDependencyEnded::GivenUp => "was given up",
        }
    }
}

/// **H8: the predicate is evaluated at `Rented-server runtime` U19's one
/// moment — when the dependency's recorded outcome is written — against the
/// reading carried with that outcome, and at no other moment and against no
/// other reading. H9: a dependency that never ran is a dependency that
/// produced no frame, and the drop fires at the moment its job record is
/// removed** (`Rented-server runtime` U19a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheMoment {
    TheDependencysRecordedOutcomeIsWritten {
        ending: HowTheDependencyEnded,
        /// **Carried with that outcome, or not** — `Rented-server runtime`
        /// U30's reading, and S13's explicit absence where the walk could
        /// not be taken.
        carried: Option<CompletenessReading>,
    },
    /// H9: the job record of a dependency that never ran is removed.
    TheDependencysJobRecordIsRemoved,
}

/// What the predicate gives, and the reason a drop carries
/// (`Rented-server runtime` U20).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThePredicateGives {
    /// The release outcome *dropped*, with the reason, journalled.
    Drop(WhyDropped),
    /// **A dependency whose ending is *finished with its frame already
    /// there* releases nothing and drops nothing**: the hold's own release
    /// path — the user's answer — is what acts on it.
    TheHoldStands,
    /// **H8: where the recorded outcome carries no completeness reading and
    /// the dependency did run, the predicate is not evaluated.** No drop is
    /// written; the hold stands with its own release path still the route,
    /// H5's offer unchanged and H6's stated cause naming the dependency's
    /// ending. Dropping the waiter against absent evidence is irreversible
    /// under `Rented-server runtime` U2's write-once rule.
    NotEvaluatedNoReadingWasCarried { ending: HowTheDependencyEnded },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyDropped {
    /// The dependency's ending, with no frame in its carried reading.
    TheDependencysEnding(HowTheDependencyEnded),
    /// H9: *the dependency was cancelled*.
    TheDependencyWasCancelled,
}

/// **H7–H9, performed.**
pub fn the_predicate(moment: &TheMoment) -> WhatThePredicateGives {
    match moment {
        // H9: no carried reading, and no reading is no frame — a fact here,
        // where the dependency never ran.
        TheMoment::TheDependencysJobRecordIsRemoved => {
            WhatThePredicateGives::Drop(WhyDropped::TheDependencyWasCancelled)
        }
        TheMoment::TheDependencysRecordedOutcomeIsWritten { ending, carried } => match carried {
            None => WhatThePredicateGives::NotEvaluatedNoReadingWasCarried { ending: *ending },
            Some(reading) => {
                if reading.no_frame_counts_as_already_there() {
                    WhatThePredicateGives::Drop(WhyDropped::TheDependencysEnding(*ending))
                } else {
                    WhatThePredicateGives::TheHoldStands
                }
            }
        },
    }
}

/// **H7's predicate, by the name a hold record carries it under** —
/// `Rented-server runtime` U18: *"The predicate is declared by the document
/// that placed the hold and evaluated by the supervisor"*, and `Rented-server runtime`
/// U2 carries it as the declaring document's own name for it. **This is the
/// one spelling**: the supervisor writes it into the hold record the
/// first-frame check's second job arrives under (`Rendering` S13), and nothing
/// reads it back to decide anything — [`the_predicate`] is what is evaluated.
/// Added at step 10c piece 5, when the supervisor first placed such a hold.
pub const THE_PREDICATE_H7_DECLARES: &str = "no frame counts as already there";

/// **H9's *no reading is no frame* does not extend to a dependency that ran
/// and was given up**, which may have written its first frame — that case
/// is H8's *not evaluated*, never a drop. Kept by [`the_predicate`]'s two
/// arms being two, and asserted here so a reader finds the rule beside the
/// code.
pub const NO_READING_IS_NO_FRAME_ONLY_WHERE_THE_DEPENDENCY_NEVER_RAN: bool = true;

/// **H1: a held job's entry says it is waiting on an answer, and offers both
/// answers, and that is the whole of what a waiting entry adds.** The state
/// derives from the hold record; both kinds of hold read identically apart
/// from the stated reason, which the hold record carries and this shows
/// without interpreting (`Rented-server runtime` U2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AWaitingEntry {
    /// The reason, as the hold record carries it — shown, not interpreted.
    pub the_stated_reason: String,
    /// **H6: a held job with a dependency shows what it waits on** — the
    /// first-frame job it waits behind; where the dependency has ended and
    /// H8 left the hold standing, the stated cause is that ending.
    pub waits_on: Option<WhatItWaitsOn>,
    /// **H5: the two answers are offered while, and only while, the hold's
    /// release outcome is unwritten as far as the desktop knows.** A tap
    /// racing that knowledge is safe by H4; nothing withdraws the offer on a
    /// guess.
    pub both_answers_offered: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatItWaitsOn {
    TheFirstFrameJob(Identity),
    /// The dependency ended, and H8 left the hold standing.
    TheDependencysEnding {
        dependency: Identity,
        ending: HowTheDependencyEnded,
    },
}

pub fn a_waiting_entry(
    the_stated_reason: &str,
    dependency: Option<(&Identity, Option<HowTheDependencyEnded>)>,
    the_release_outcome_is_known_to_the_desktop: bool,
) -> AWaitingEntry {
    AWaitingEntry {
        the_stated_reason: the_stated_reason.to_string(),
        waits_on: dependency.map(|(job, ending)| match ending {
            None => WhatItWaitsOn::TheFirstFrameJob(job.clone()),
            Some(ending) => WhatItWaitsOn::TheDependencysEnding {
                dependency: job.clone(),
                ending,
            },
        }),
        both_answers_offered: !the_release_outcome_is_known_to_the_desktop,
    }
}

/// **H4: two answers to one hold are settled by the machine's
/// serialization** — the first recorded wins and the second acts on nothing
/// (`Rented-server runtime` U13); the loser is told which answer was applied
/// and when (U14); a *no* after a *yes* is not converted into a stop (U15).
/// What this crate adds is only presentation ground: the entry shows the
/// recorded release outcome from the moment it is learned.
pub const A_NO_AFTER_A_YES_IS_NEVER_CONVERTED_INTO_A_STOP: bool = true;

/// **H11: the dropped waiter leaves nothing anywhere, and the user is told
/// why, once.** Applying the drop event removes the machine-side job record
/// and its material by the same enumeration as a cancel (L1), and on the
/// desktop the provisional record is removed when the event is applied — a
/// job that never rendered leaves no trace.
pub const A_DROPPED_WAITER_LEAVES_NOTHING_ANYWHERE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::tests::a_job;

    fn a_reading(frames: &[i64]) -> CompletenessReading {
        CompletenessReading {
            frames_already_there: frames.to_vec(),
            the_shape: TheInventorysShape::Frames,
        }
    }

    #[test]
    fn a_dependency_that_ended_with_no_frame_already_there_drops_the_waiter() {
        for ending in [
            HowTheDependencyEnded::Failed,
            HowTheDependencyEnded::LeftStopped,
            HowTheDependencyEnded::GivenUp,
        ] {
            let moment = TheMoment::TheDependencysRecordedOutcomeIsWritten {
                ending,
                carried: Some(a_reading(&[])),
            };
            assert_eq!(
                the_predicate(&moment),
                WhatThePredicateGives::Drop(WhyDropped::TheDependencysEnding(ending))
            );
        }
    }

    #[test]
    fn a_dependency_finished_with_its_frame_already_there_releases_nothing_and_drops_nothing() {
        let moment = TheMoment::TheDependencysRecordedOutcomeIsWritten {
            ending: HowTheDependencyEnded::Finished,
            carried: Some(a_reading(&[1])),
        };
        assert_eq!(the_predicate(&moment), WhatThePredicateGives::TheHoldStands);
    }

    #[test]
    fn the_exit_status_decides_nothing_a_finished_ending_with_no_frame_drops() {
        // Six endings at 0 with an empty output folder: *finished* with no
        // frame already there is a drop like any other.
        let moment = TheMoment::TheDependencysRecordedOutcomeIsWritten {
            ending: HowTheDependencyEnded::Finished,
            carried: Some(a_reading(&[])),
        };
        assert_eq!(
            the_predicate(&moment),
            WhatThePredicateGives::Drop(WhyDropped::TheDependencysEnding(
                HowTheDependencyEnded::Finished
            ))
        );
    }

    #[test]
    fn an_outcome_carrying_no_reading_after_the_dependency_ran_is_not_evaluated() {
        let moment = TheMoment::TheDependencysRecordedOutcomeIsWritten {
            ending: HowTheDependencyEnded::GivenUp,
            carried: None,
        };
        assert_eq!(
            the_predicate(&moment),
            WhatThePredicateGives::NotEvaluatedNoReadingWasCarried {
                ending: HowTheDependencyEnded::GivenUp
            }
        );
    }

    #[test]
    fn a_dependency_that_never_ran_drops_the_waiter_at_the_removal_of_its_record() {
        assert_eq!(
            the_predicate(&TheMoment::TheDependencysJobRecordIsRemoved),
            WhatThePredicateGives::Drop(WhyDropped::TheDependencyWasCancelled)
        );
    }

    #[test]
    fn a_video_shaped_dependency_reads_the_same_statement_at_its_one_limit() {
        let present = CompletenessReading {
            frames_already_there: vec![1],
            the_shape: TheInventorysShape::Video,
        };
        let absent = CompletenessReading {
            frames_already_there: Vec::new(),
            the_shape: TheInventorysShape::Video,
        };
        assert!(!present.no_frame_counts_as_already_there());
        assert!(absent.no_frame_counts_as_already_there());
    }

    #[test]
    fn a_waiting_entry_offers_both_answers_until_the_release_outcome_is_known() {
        let first_frame = a_job();
        let entry = a_waiting_entry("the first frame check", Some((&first_frame, None)), false);
        assert!(entry.both_answers_offered);
        assert_eq!(
            entry.waits_on,
            Some(WhatItWaitsOn::TheFirstFrameJob(first_frame.clone()))
        );
        let after = a_waiting_entry(
            "the first frame check",
            Some((&first_frame, Some(HowTheDependencyEnded::GivenUp))),
            true,
        );
        assert!(!after.both_answers_offered);
        assert!(matches!(
            after.waits_on,
            Some(WhatItWaitsOn::TheDependencysEnding {
                ending: HowTheDependencyEnded::GivenUp,
                ..
            })
        ));
    }
}

//! **§15's machine half — the preview worker's queueing per finished frame and
//! the ending's catch-up write.** The desktop's half — the trigger, the one
//! fetch in flight, the fetch arranged through the transfer contract and the
//! heading over the picture — is in `smoozy-queue-history`'s own
//! `acting::previews`, which re-exports everything here at the path it has
//! always been reached at.
//!
//! # What is on the machine and what is on the desk
//!
//! **P1 and P1b are the supervisor's**: the queueing of preview work, in frame
//! order, over records Smoozy wrote. **P2 to P5 and P10a are the desktop's**:
//! the trigger, the fetch, the holding and the heading. **The two never meet
//! in one process**, and since 2026-09-19 they are not in one folder either:
//! this half is in the crate the rented machine's own program may take.
//!
//! # P1a's trigger is a record of Smoozy's own observation
//!
//! **The render worker observes each frame's save announcement as the bytes
//! pass its own copy loop** and reports **one figure** — the newest frame of
//! the attempt's list whose save has been announced — on R5's bounded
//! reporting. **On a report, the supervisor queues preview work, in frame
//! order, for every frame of that prefix whose chosen candidate has no
//! published derivative and no production standing.**
//!
//! **Nothing here walks a folder, reads a file's presence or reads a
//! free-space figure** (`State, identity and recovery` §1.3 rule 5, §11.4).
//! The prefix arithmetic is [`crate::previews::the_frames_owed_a_preview`]'s
//! and this module only performs what it answers.
//!
//! # P1b is what makes *no finished frame is passed over* true
//!
//! **In the same pass in which a recorded outcome that ends a job is written,
//! the supervisor reads the completeness reading carried with that outcome and
//! queues preview work for every frame it counts already there** whose chosen
//! candidate has no published derivative and no production standing.
//!
//! **It is a read of records Smoozy wrote** — the carried reading, and the
//! derivative operations' own records — never a walk of its own: the one walk
//! behind it is the ending step's, already taken (H7). **It recovers the
//! frames of a stream that printed nothing, the frames whose reports a
//! supervisor restart lost, an interrupted first attempt's frames, and every
//! production that failed for want of room.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` B20.
//! `Rented-server runtime` S14. `State, identity and recovery` §1.3, §9.5,
//! §11.4.

use smoozy_state::identities::Identity;

use crate::previews::the_frames_owed_a_preview;

/// **What the supervisor queues, for one job, at one trigger.**
///
/// **In frame order, and one entry per frame** — never a batch with an order
/// of its own, because the spec's promise is that the user moves through a
/// job's finished frames and a set with holes does not let them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatIsQueued {
    pub job: Identity,
    pub frames: Vec<i64>,
}

/// **What the supervisor already holds about a job's derivatives, read off
/// records Smoozy wrote.**
///
/// **Not one of these is a look at the folder.** The published set is the
/// derivative operations' own outcomes; the standing set is the productions
/// queued or running; the excluded set is
/// `Job files and outputs` B20's candidate rules, applied by that document.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatTheRecordsSayAboutTheDerivatives {
    pub published: Vec<i64>,
    pub standing_queued_or_running: Vec<i64>,
    pub excluded_by_the_candidate_rules: Vec<i64>,
}

/// **P1 and P1a performed: on a report, the supervisor queues preview work for
/// the completed prefix.**
///
/// **A missed or delayed report costs a delay, never a hole** — P1b's write is
/// the catch-up — and **a production that failed leaves its frame owed and
/// queued again at the next report**, never retried inside its own action
/// (`State, identity and recovery` §9.5), never on a timer, and never on a
/// free-space reading.
///
/// **The backlog is bounded by the frame count and by nothing else, and it is
/// worked off rather than dropped.**
pub fn queue_on_a_report(
    job: &Identity,
    the_attempts_list_in_order: &[i64],
    newest_frame_saved: Option<i64>,
    records: &WhatTheRecordsSayAboutTheDerivatives,
) -> WhatIsQueued {
    WhatIsQueued {
        job: job.clone(),
        frames: the_frames_owed_a_preview(
            the_attempts_list_in_order,
            newest_frame_saved,
            &records.published,
            &records.standing_queued_or_running,
            &records.excluded_by_the_candidate_rules,
        ),
    }
}

/// **P1b performed: the ending's write is the catch-up.**
///
/// **In the same pass in which a recorded outcome that ends a job is written**
/// — whoever writes it, the worker path or reconciliation, which is H7's own
/// shape — **the supervisor reads the completeness reading carried with that
/// outcome and queues preview work, in frame order, for every frame it counts
/// already there** whose chosen candidate has no published derivative and no
/// production standing, under P1a's same exclusions.
///
/// **Where the outcome carries no reading, nothing is queued**, because there
/// is no set to queue from — the same absence H8 turns on, never four empty
/// sets.
pub fn queue_at_the_endings_write(
    job: &Identity,
    carried: Option<&crate::waiting::CompletenessReading>,
    records: &WhatTheRecordsSayAboutTheDerivatives,
) -> WhatIsQueued {
    let frames = match carried {
        None => Vec::new(),
        Some(reading) => reading
            .frames_already_there
            .iter()
            .copied()
            .filter(|frame| {
                !records.published.contains(frame)
                    && !records.standing_queued_or_running.contains(frame)
                    && !records.excluded_by_the_candidate_rules.contains(frame)
            })
            .collect(),
    };
    WhatIsQueued {
        job: job.clone(),
        frames,
    }
}

/// **P1b, stated as a value: where even the ending's write is refused for
/// space, nothing here fires until that write lands.**
///
/// When it lands is Q8's and N5's ground — after space returns, by the
/// supervisor's restart or by `Rented-server runtime` S14's pending-write
/// landing — **and the catch-up rides the write itself**, so the backlog's
/// recovery needs no trigger of its own beyond the ending's.
pub const THE_CATCH_UP_RIDES_THE_ENDINGS_OWN_WRITE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::waiting::{CompletenessReading, TheInventorysShape};
    use smoozy_state::identities::IdentityKind;

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    /// **P1a: the report queues the completed prefix, in frame order, less
    /// what is published, standing or excluded.**
    #[test]
    fn a_report_queues_the_completed_prefix_in_frame_order() {
        let job = a_job();
        let queued = queue_on_a_report(
            &job,
            &[10, 11, 12, 13],
            Some(12),
            &WhatTheRecordsSayAboutTheDerivatives {
                published: vec![10],
                standing_queued_or_running: vec![11],
                excluded_by_the_candidate_rules: Vec::new(),
            },
        );
        assert_eq!(queued.job, job);
        assert_eq!(queued.frames, vec![12]);
    }

    /// **No report, nothing queued** — and that is R4's stream that printed
    /// nothing, where P1b is the route the set is produced by.
    #[test]
    fn no_report_queues_nothing_and_the_ending_is_the_route_instead() {
        let job = a_job();
        assert!(queue_on_a_report(
            &job,
            &[1, 2, 3],
            None,
            &WhatTheRecordsSayAboutTheDerivatives::default()
        )
        .frames
        .is_empty());
        let at_the_end = queue_at_the_endings_write(
            &job,
            Some(&CompletenessReading {
                frames_already_there: vec![1, 2, 3],
                the_shape: TheInventorysShape::Frames,
            }),
            &WhatTheRecordsSayAboutTheDerivatives::default(),
        );
        assert_eq!(at_the_end.frames, vec![1, 2, 3]);
    }

    /// **P1b: a production that failed leaves its frame owed and is queued
    /// again at the ending's write** — the condition reads what stands, not
    /// what failed.
    #[test]
    fn the_endings_write_queues_every_frame_with_no_derivative_standing() {
        let job = a_job();
        let queued = queue_at_the_endings_write(
            &job,
            Some(&CompletenessReading {
                frames_already_there: vec![1, 2, 3, 4],
                the_shape: TheInventorysShape::Frames,
            }),
            &WhatTheRecordsSayAboutTheDerivatives {
                published: vec![1],
                standing_queued_or_running: vec![2],
                // A video destination, and an interrupted job's last frame
                // asked for.
                excluded_by_the_candidate_rules: vec![4],
            },
        );
        assert_eq!(queued.frames, vec![3]);
        const _: () = assert!(THE_CATCH_UP_RIDES_THE_ENDINGS_OWN_WRITE);
    }

    /// **An outcome carrying no reading queues nothing**, which is the absence
    /// H8 turns on rather than four empty sets.
    #[test]
    fn an_outcome_carrying_no_reading_queues_nothing() {
        assert!(queue_at_the_endings_write(
            &a_job(),
            None,
            &WhatTheRecordsSayAboutTheDerivatives::default()
        )
        .frames
        .is_empty());
    }

    /// **P4a: nothing in this module takes a held copy and arranges anything,
    /// and nothing clears one.** The guard is a scan of the working half.
    /// **It stood over the whole file before the file was split on
    /// 2026-09-19, so it stands over each half**; the desktop half keeps its
    /// own copy, with P7's constant beside it.
    #[test]
    fn nothing_here_re_fetches_a_cleared_copy_or_clears_one() {
        let source = include_str!("previews.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let code: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in ["age", "sweep", "clear", "expire"] {
            assert!(
                !code.contains(forbidden),
                "{forbidden} would be a clearing this module does not own"
            );
        }
    }

    /// **P1: nothing here reads a folder, a file's presence or a free-space
    /// figure.**
    /// Over each half of the split file, for the same reason as the guard
    /// above.
    #[test]
    fn nothing_here_looks_at_the_filesystem_at_all() {
        let source = include_str!("previews.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let code: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in ["std::fs", "read_dir", "metadata", "free_space"] {
            assert!(!code.contains(forbidden), "{forbidden}");
        }
    }
}

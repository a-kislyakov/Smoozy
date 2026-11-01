//! Retention and cleanup, the local lifetimes — `State, identity and
//! recovery` §11.
//!
//! §11.1's five classes are the complete list of local lifetimes `Desktop app
//! and delivery` is handed. §11.2 says where each is enforced, and there is
//! exactly one sweep in the whole of it: the age ending on a job's fetched
//! derivatives, which is the one lifetime here measured in elapsed time on the
//! user's own computer.

use std::time::{Duration, SystemTime};

/// §11.1's five classes.
///
/// **user-held** and **subject-held** things are removed by the act that
/// removes them and by no sweep, with the one exception named at
/// [`PREVIEW_AGE_PERIOD`]. **period-held** and **run-held** things are in
/// process memory or in a swept area. **action-held** things end with the
/// action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lifetime {
    /// Ends when the user removes it, and never otherwise. History records and
    /// everything they carry; server rows; key rows; settings; suppressed
    /// warnings; the store's standing, which no control removes; transfer
    /// arrangements; the three trust records; secret-store subject references;
    /// the run-mark, the
    /// remembered pane widths, the last update-check moment and the
    /// declined-update version mark.
    UserHeld,
    /// Ends when its subject is removed, and never otherwise — with the two
    /// endings §8.8 names for a failure record, and **the age ending below,
    /// which belongs to a job's fetched derivatives alone**.
    SubjectHeld,
    /// Ends when the connected period ends, however it ends. The last-known
    /// queue reading; every live reading. **The machine password held for a
    /// connection is not in this class** — it is connection-held, and R2 names
    /// it as the one place the project said otherwise.
    PeriodHeld,
    /// Ends when the application run ends, however it ends. The Linux store
    /// probe result; the
    /// throwaway known-hosts files and the throwaway key pairs, each removed
    /// by its area's own start-of-run sweep where a process exited first.
    /// **Nothing is ever trusted from one.**
    RunHeld,
    /// Ends when the action ends, success or failure. The do-not-remember
    /// machine password; staged transfer material until publication.
    ActionHeld,
}

/// R16: **thirty days**, and it is a constant of the application. Ruled
/// 2026-09-02 under the Creator's standing delegation of that date, the
/// Creator having ruled the age ending itself on 2026-08-22 without setting a
/// number.
///
/// **Nothing reads the number from anywhere** — no setting exposes it, no
/// reading raises or lowers it, and no machine supplies it. A lifetime that a
/// reading could move is a lifetime nobody stated.
pub const PREVIEW_AGE_PERIOD: Duration = Duration::from_secs(30 * 24 * 60 * 60);

/// R18's two moments, and there is no third. **Nothing polls a clock on a
/// schedule and nothing reaches a machine.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SweepMoment {
    /// The moment a running application is certain to reach without a timer,
    /// and what clears the copies where the application is never left running
    /// long enough to cross the period.
    ApplicationStart,
    /// The moment the held set grows.
    APreviewFetchPublishedACopy,
}

/// One fetched derivative as the sweep sees it: the copy, and the one moment
/// recorded beside it.
///
/// R13: the moment is **when that copy was published on the user's computer**,
/// on that computer's own clock — never the frame's render moment, never the
/// job's ending and never the machine's note, because a lifetime taken from a
/// machine reading would be arithmetic across two clocks, which D5 forbids.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchedDerivative<Copy> {
    pub copy: Copy,
    pub published_on_this_computer_at: SystemTime,
}

/// R19's comparison, stated so it can be built.
///
/// **A sweep reads the clock once, at the start of the pass, and judges every
/// copy in the pass against that one reading**, so no two copies in one pass
/// are compared against different nows. A copy is past its period when that
/// reading minus its recorded moment is **greater than** thirty days, and at
/// nothing less — thirty days exactly is kept.
///
/// **A difference that is negative — a clock now standing earlier than the
/// moment the copy was published — is not greater than thirty days and clears
/// nothing**, and there is no second rule for it: the copy is judged again at
/// the next pass.
pub fn past_its_period(reading: SystemTime, published_at: SystemTime) -> bool {
    match reading.duration_since(published_at) {
        Ok(held_for) => held_for > PREVIEW_AGE_PERIOD,
        // The clock now stands earlier than the moment the copy was published.
        Err(_) => false,
    }
}

/// One pass of the sweep.
///
/// R20: **the copy alone.** The job record, its ending, its times, its figures
/// and every other field it carries are untouched, and the entry then shows no
/// picture, which is the ruling's own accepted cost. **A pass writes no
/// durable state of its own** — no record of having run, no next-due moment,
/// no count — so nothing schedules the next pass and nothing is recovered
/// after a crash. A copy whose removal did not complete is met again at the
/// next pass and removed then.
///
/// Returns the copies to clear, in the order they were handed in.
pub fn a_pass<Copy: Clone>(
    _at: SweepMoment,
    reading: SystemTime,
    held: &[FetchedDerivative<Copy>],
) -> Vec<Copy> {
    held.iter()
        .filter(|derivative| past_its_period(reading, derivative.published_on_this_computer_at))
        .map(|derivative| derivative.copy.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn moment_before(reading: SystemTime, ago: Duration) -> SystemTime {
        reading - ago
    }

    /// R21's test, written in the plan's own words: *"Record a copy's
    /// publication moment at thirty days and one second before the pass's own
    /// clock reading and run a pass: the copy is gone and the job record is
    /// whole. Record another at thirty days exactly before that reading: the
    /// copy stands."*
    #[test]
    fn thirty_days_and_one_second_goes_and_thirty_days_exactly_stands() {
        let reading = SystemTime::now();
        let held = vec![
            FetchedDerivative {
                copy: "a day past",
                published_on_this_computer_at: moment_before(
                    reading,
                    PREVIEW_AGE_PERIOD + Duration::from_secs(1),
                ),
            },
            FetchedDerivative {
                copy: "thirty days exactly",
                published_on_this_computer_at: moment_before(reading, PREVIEW_AGE_PERIOD),
            },
        ];

        for at in [
            SweepMoment::ApplicationStart,
            SweepMoment::APreviewFetchPublishedACopy,
        ] {
            assert_eq!(a_pass(at, reading, &held), vec!["a day past"]);
        }
    }

    #[test]
    fn a_clock_standing_earlier_clears_nothing() {
        let reading = SystemTime::now();
        let held = vec![FetchedDerivative {
            copy: "published in the future",
            published_on_this_computer_at: reading + Duration::from_secs(60 * 60 * 24 * 365),
        }];
        assert!(a_pass(SweepMoment::ApplicationStart, reading, &held).is_empty());
    }

    #[test]
    fn every_copy_in_one_pass_is_judged_against_one_reading() {
        // Two copies published a second apart, either side of the boundary,
        // come out one cleared and one kept — which is only true if both were
        // measured against the same now.
        let reading = SystemTime::now();
        let held = vec![
            FetchedDerivative {
                copy: "older",
                published_on_this_computer_at: moment_before(
                    reading,
                    PREVIEW_AGE_PERIOD + Duration::from_secs(1),
                ),
            },
            FetchedDerivative {
                copy: "younger",
                published_on_this_computer_at: moment_before(
                    reading,
                    PREVIEW_AGE_PERIOD - Duration::from_secs(1),
                ),
            },
        ];
        assert_eq!(
            a_pass(SweepMoment::ApplicationStart, reading, &held),
            vec!["older"]
        );
    }

    #[test]
    fn a_pass_over_nothing_does_nothing() {
        let held: Vec<FetchedDerivative<&str>> = Vec::new();
        assert!(a_pass(SweepMoment::ApplicationStart, SystemTime::now(), &held).is_empty());
    }

    #[test]
    fn the_period_is_thirty_days_and_is_read_from_nowhere() {
        assert_eq!(PREVIEW_AGE_PERIOD, Duration::from_secs(2_592_000));
    }
}

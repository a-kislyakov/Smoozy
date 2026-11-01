//! The restart `Server Setup` caused — `Servers` §4.
//!
//! **M1: the mark is `Server Setup`'s and this document takes it.**
//! *Restarting, caused by Smoozy* sits on the connected period, is **set before
//! the restart command leaves**, and is cleared **by exactly two things and
//! nothing else: the machine answering, or this document's reconnect policy
//! giving up.** A user who stops waiting **clears nothing** — the mark and the
//! machine-level state both stand and the policy goes on running.
//!
//! **M2: *this drop is mine* is a state set before the act and never a reading
//! taken after it.** That is `Server Setup` T4's own sentence — **that
//! document's T4 and not this one's**, which is about a resume resting on a
//! staging path Smoozy names. The plan's own M2 writes it bare and the
//! ambiguity is inherited rather than invented here. Fact 57 is its reason: a
//! machine closing politely and a machine powered off are detected in the same
//! second, so **nothing in the moment tells a restart from a destruction**.
//! Smoozy knows because it caused it.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` T13.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::repair::{ATTEMPTS_IN_A_RUN, CONNECT_DEADLINE, PAUSE_BETWEEN_ATTEMPTS, RUN_SPAN,
    STANDING_PAUSE};

/// **M4: the policy is four runs of §3.1's shape, and then it gives up.**
pub const RUNS_BEFORE_GIVING_UP: u32 = 4;

/// **M4: eight attempts per run, 15-second pauses, a 20-second connect
/// deadline, N2a's 105-second run span, and a 60-second standing pause between
/// runs. Its wait is therefore exactly 4 × 105 + 3 × 60 = 600 seconds — ten
/// minutes.**
///
/// **It was a floor until 2026-08-22**, when N2a bounded the run's span and made
/// it a length.
pub const THE_WAIT: Duration = Duration::from_secs(600);

/// **M5: ten minutes is derived from the spread and not from any machine.**
///
/// `Server Setup` T6 measured return at **9 to 90 seconds across three machines
/// and about 90 for a container recycle**, and says outright that *"anything
/// budgeting a fixed wait is wrong on most machines"*. **Ten minutes is more
/// than six times the top of that spread**, so a machine merely slow to return
/// is never declared gone, and **it is the run structure of §3.1 repeated
/// rather than a fresh constant**.
pub fn the_wait_is_composed_of_the_repairs_own_figures() -> Duration {
    Duration::from_secs(
        u64::from(RUNS_BEFORE_GIVING_UP) * RUN_SPAN.as_secs()
            + u64::from(RUNS_BEFORE_GIVING_UP - 1) * STANDING_PAUSE.as_secs(),
    )
}

/// **M6: the three ways the wait ends, and only two clear the mark.** This is
/// `Server Setup` T6's closed set, performed — **that document's T6 and not
/// this one's**, which is the label every handed-over connection carries and
/// says something else entirely. The plan's own M6 writes it bare and the
/// ambiguity is inherited rather than invented here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheWaitEnds {
    /// **1. The machine answers.** The mark clears, the period's observed value
    /// reaches *connected*, and `Server Setup`'s recovery run takes the setup
    /// over from the recorded boundary.
    TheMachineAnswers,
    /// **2. The user stops waiting.** The mark does **not** clear, this policy
    /// goes on running, and the machine-level state stands. **What ends is the
    /// user's waiting and nothing else.**
    TheUserStopsWaiting,
    /// **3. This policy gives up.** The mark clears. `Server Setup` reports the
    /// setup cut off part-way, and Smoozy says only what it knows — the
    /// connection is down and could not be restored — **with no claim to know
    /// why, because a machine still starting cannot be told from one that will
    /// never answer.**
    ThePolicyGivesUp,
}

impl HowTheWaitEnds {
    pub const ALL: [HowTheWaitEnds; 3] = [
        HowTheWaitEnds::TheMachineAnswers,
        HowTheWaitEnds::TheUserStopsWaiting,
        HowTheWaitEnds::ThePolicyGivesUp,
    ];

    /// **M1 and M6: exactly two clear it.**
    pub fn clears_the_mark(self) -> bool {
        matches!(
            self,
            HowTheWaitEnds::TheMachineAnswers | HowTheWaitEnds::ThePolicyGivesUp
        )
    }

    /// **M6 item 2: the policy goes on running after the user stops waiting.**
    pub fn the_policy_goes_on_running(self) -> bool {
        matches!(self, HowTheWaitEnds::TheUserStopsWaiting)
    }

    /// **M9: so the two clearing events land differently and both land
    /// correctly.** Where the machine answers, the observed value reaches
    /// *connected* and **there was never a record to withdraw**. Where the
    /// policy gives up, **the condition is raised at that moment** and the
    /// notification fires then — **the first moment Smoozy has anything to tell
    /// the user that it did not already cause.**
    pub fn raises_the_condition_record(self) -> bool {
        matches!(self, HowTheWaitEnds::ThePolicyGivesUp)
    }
}

/// **M1, as the one place the mark lives: a handle on the period, set from
/// outside it and read live by the driving.**
///
/// **Why a handle rather than a field the period is built with.** M1 puts the
/// mark on the period and has it **set before the restart command leaves** —
/// which is in the middle of a period's life, from the thread driving
/// `Server Setup` R1's run, while [`crate::driving`] is waiting on the
/// connection the restart is about to kill. A value fixed when the period
/// began could never be that. **It is the shape [`crate::driving::EndThePeriod`]
/// already uses** for the three of S10's endings that arrive from another
/// thread, and for the same reason.
///
/// **One copy of the fact and no second anywhere.** The period holds this and
/// the driving reads it; what the desktop publishes for the row is composed
/// from it through [`crate::condition::observed_value`] ([`K8`] is the
/// composition). **Nobody keeps a second bool beside it.**
///
/// [`K8`]: crate::condition::observed_value
#[derive(Clone)]
pub struct TheRestartMark(Arc<TheMarksOwnState>);

struct TheMarksOwnState {
    held: Mutex<WhereTheMarkStands>,
    wakes: tokio::sync::Notify,
}

/// **Where the mark stands, and the third state is what M1's *cleared by
/// exactly two things* needs in order to be checkable.**
///
/// A cleared mark carries **which of [`HowTheWaitEnds`]' two clearing endings
/// cleared it**, because `Server Setup` T5 and T6 do completely different
/// things with the two: the machine answering is picked up by a recovery run,
/// and the policy giving up is reported as a setup cut off part-way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereTheMarkStands {
    /// No restart of Smoozy's has been issued under this period.
    NoMarkWasEverSet,
    /// **Set before the restart command left**, and standing.
    ItStands,
    /// Cleared, by one of M6's two clearing endings and by nothing else.
    ItWasClearedBy(HowTheWaitEnds),
    /// **S11: every period-held thing ends the moment the period ends**, and
    /// the mark is one of them.
    ///
    /// **This is not a third clearing event and M1's closed set of two is
    /// untouched**: a period that has ended holds no mark because it holds
    /// nothing, and the server reads *disconnected* off the period rather than
    /// off anything here. It is written down so that a caller waiting on one of
    /// the two endings has a condition it can certainly reach — the period ends
    /// at the latest when the application run does (S10 item 2) — rather than a
    /// wait nothing would ever end.
    ThePeriodEndedUnderIt,
}

impl TheRestartMark {
    /// A period with no restart of Smoozy's under it, which is every period
    /// until one is issued.
    pub fn none_is_set() -> TheRestartMark {
        TheRestartMark(Arc::new(TheMarksOwnState {
            held: Mutex::new(WhereTheMarkStands::NoMarkWasEverSet),
            wakes: tokio::sync::Notify::new(),
        }))
    }

    /// **M1: set before the restart command leaves.**
    ///
    /// **Setting it a second time is not a second mark**, and a mark already
    /// cleared is not set again by it: a period sees one restart of Smoozy's,
    /// because `Server Setup` R1 item 6 is one step of one run and
    /// `Server Setup` Y9 allows one run at a time.
    pub fn set(&self) {
        let mut held = self.0.held.lock().expect("the mark");
        if matches!(*held, WhereTheMarkStands::NoMarkWasEverSet) {
            *held = WhereTheMarkStands::ItStands;
        }
    }

    /// **S11: the period ended, so nothing holds a mark any more.**
    ///
    /// **It clears nothing and answers nothing about the machine**; what it
    /// does is end every wait on this value, because the thing being waited on
    /// has gone.
    pub fn the_period_ended(&self) {
        {
            let mut held = self.0.held.lock().expect("the mark");
            // **A recorded clearing is not rubbed out by the period ending
            // after it.** The two endings are what `Server Setup` T5 and T6
            // act on, and a period whose machine answered ends later like any
            // other — so the ending that happened stays readable.
            if matches!(*held, WhereTheMarkStands::ItWasClearedBy(_)) {
                return;
            }
            *held = WhereTheMarkStands::ThePeriodEndedUnderIt;
        }
        self.0.wakes.notify_waiters();
    }

    /// **M3's condition, and K8's**: whether the mark stands now.
    pub fn stands(&self) -> bool {
        matches!(self.where_it_stands(), WhereTheMarkStands::ItStands)
    }

    pub fn where_it_stands(&self) -> WhereTheMarkStands {
        *self.0.held.lock().expect("the mark")
    }

    /// **M1 and M6: the closed set of two, performed in one place.**
    ///
    /// **An ending that does not clear the mark does not clear it here
    /// either** — [`HowTheWaitEnds::TheUserStopsWaiting`] answers `false` and
    /// leaves the mark exactly where it was, which is T6's *the mark stands,
    /// the machine-level state stands, and the policy goes on running*. **A
    /// mark that is not standing is not cleared twice.**
    pub fn cleared_by(&self, how: HowTheWaitEnds) -> bool {
        if !how.clears_the_mark() {
            return false;
        }
        let cleared = {
            let mut held = self.0.held.lock().expect("the mark");
            if matches!(*held, WhereTheMarkStands::ItStands) {
                *held = WhereTheMarkStands::ItWasClearedBy(how);
                true
            } else {
                false
            }
        };
        if cleared {
            self.0.wakes.notify_waiters();
        }
        cleared
    }

    /// **T6's wait, as the side that issued the restart reads it: which of the
    /// two clearing endings happened.**
    ///
    /// **It answers at once where the mark has already cleared**, and the
    /// waiter is registered before the mark is looked at, so a clear landing
    /// between the look and the wait is never missed — [`crate::driving`]'s own
    /// ending does the same dance for the same reason. **The third ending never
    /// arrives here**: the user stopping the wait clears nothing, so it is the
    /// waiting caller's to end and not this value's.
    ///
    /// **Nothing where the period ended under it** ([`WhereTheMarkStands::ThePeriodEndedUnderIt`]),
    /// which is what makes this a wait with a condition it can certainly reach.
    pub async fn how_the_wait_ended(&self) -> Option<HowTheWaitEnds> {
        loop {
            let notified = self.0.wakes.notified();
            let mut notified = std::pin::pin!(notified);
            notified.as_mut().enable();
            match self.where_it_stands() {
                WhereTheMarkStands::ItWasClearedBy(how) => return Some(how),
                WhereTheMarkStands::ThePeriodEndedUnderIt => return None,
                _ => {}
            }
            notified.await;
        }
    }
}

impl std::fmt::Debug for TheRestartMark {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("TheRestartMark")
            .field(&self.where_it_stands())
            .finish()
    }
}

/// **Two marks are the same mark when they stand the same way.** The period
/// this sits on is compared by what it holds, and a handle is not a thing the
/// comparison is about.
impl PartialEq for TheRestartMark {
    fn eq(&self, other: &TheRestartMark) -> bool {
        self.where_it_stands() == other.where_it_stands()
    }
}

impl Eq for TheRestartMark {}

/// **M3: while the mark stands, the dropped-connection repair and the
/// could-not-restore notification are suppressed**, and the server reads as
/// **restarting**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhileTheMarkStands {
    pub the_ordinary_repair_is_suppressed: bool,
    pub the_could_not_restore_notification_is_suppressed: bool,
    /// **M8: no connection condition record is raised at all.** The observed
    /// value still moves — Smoozy knows perfectly well the connection is down,
    /// because it took it down — but the record `Desktop app and delivery` T13
    /// raises the notification from **is not raised**. That is the suppression
    /// expressed in the one mechanism that presents it, rather than a second
    /// suppression rule sitting beside the record.
    pub no_condition_record_is_raised: bool,
}

impl WhileTheMarkStands {
    pub fn what_is_suppressed() -> WhileTheMarkStands {
        WhileTheMarkStands {
            the_ordinary_repair_is_suppressed: true,
            the_could_not_restore_notification_is_suppressed: true,
            no_condition_record_is_raised: true,
        }
    }
}

/// **M7: giving up hands the period back to the ordinary repair rather than
/// ending it.**
///
/// The spec's own sentence is *"Only if the machine never comes back does it
/// become an ordinary failure, reported as one"*, and an ordinary failure is one
/// **the standing repair goes on working at for as long as the app is open**.
/// So the period stands, the suppressions lift, and **the machine comes back on
/// its own if it ever does**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AfterGivingUp {
    pub the_period_stands: bool,
    pub the_suppressions_lift: bool,
    pub the_ordinary_repair_resumes: bool,
}

impl AfterGivingUp {
    pub fn what_happens() -> AfterGivingUp {
        AfterGivingUp {
            the_period_stands: true,
            the_suppressions_lift: true,
            the_ordinary_repair_resumes: true,
        }
    }
}

/// The policy's own bookkeeping — runs done, and whether it has given up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MarkedPeriodPolicy {
    runs_failed: u32,
}

impl MarkedPeriodPolicy {
    pub fn fresh() -> MarkedPeriodPolicy {
        MarkedPeriodPolicy::default()
    }

    pub fn a_run_failed(&mut self) {
        self.runs_failed += 1;
    }

    /// **M4: four runs, and then it gives up.**
    pub fn has_given_up(self) -> bool {
        self.runs_failed >= RUNS_BEFORE_GIVING_UP
    }

    /// Each attempt carries the same connect deadline the ordinary repair's
    /// does — **the run structure repeated rather than a fresh constant**.
    pub fn attempts_per_run(self) -> u32 {
        ATTEMPTS_IN_A_RUN
    }

    pub fn connect_deadline(self) -> Duration {
        CONNECT_DEADLINE
    }

    pub fn pause_between_attempts(self) -> Duration {
        PAUSE_BETWEEN_ATTEMPTS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_wait_is_the_repairs_own_figures_and_not_a_fresh_constant() {
        assert_eq!(the_wait_is_composed_of_the_repairs_own_figures(), THE_WAIT);
        assert_eq!(THE_WAIT, Duration::from_secs(600));
    }

    #[test]
    fn ten_minutes_is_more_than_six_times_the_top_of_the_observed_spread() {
        let (_, top) = crate::repair::OBSERVED_RETURN_SPREAD;
        assert!(THE_WAIT > top * 6);
    }

    #[test]
    fn exactly_two_of_the_three_endings_clear_the_mark() {
        assert_eq!(HowTheWaitEnds::ALL.len(), 3);
        let clearing: Vec<_> = HowTheWaitEnds::ALL
            .into_iter()
            .filter(|ending| ending.clears_the_mark())
            .collect();
        assert_eq!(
            clearing,
            vec![HowTheWaitEnds::TheMachineAnswers, HowTheWaitEnds::ThePolicyGivesUp]
        );
    }

    #[test]
    fn a_user_who_stops_waiting_clears_nothing_and_the_policy_runs_on() {
        let ending = HowTheWaitEnds::TheUserStopsWaiting;
        assert!(!ending.clears_the_mark());
        assert!(ending.the_policy_goes_on_running());
        assert!(!ending.raises_the_condition_record());
    }

    #[test]
    fn the_condition_is_raised_only_where_the_policy_gives_up() {
        // M9: where the machine answers there was never a record to withdraw.
        assert!(!HowTheWaitEnds::TheMachineAnswers.raises_the_condition_record());
        assert!(HowTheWaitEnds::ThePolicyGivesUp.raises_the_condition_record());
    }

    #[test]
    fn the_policy_gives_up_after_four_runs_and_not_before() {
        let mut policy = MarkedPeriodPolicy::fresh();
        for _ in 0..RUNS_BEFORE_GIVING_UP - 1 {
            policy.a_run_failed();
            assert!(!policy.has_given_up());
        }
        policy.a_run_failed();
        assert!(policy.has_given_up());
    }

    #[test]
    fn giving_up_hands_the_period_back_rather_than_ending_it() {
        let after = AfterGivingUp::what_happens();
        assert!(after.the_period_stands);
        assert!(after.the_suppressions_lift);
        assert!(after.the_ordinary_repair_resumes);
    }

    #[test]
    fn the_mark_is_set_once_and_cleared_by_exactly_the_two() {
        // M1: set before the restart command leaves, cleared by the machine
        // answering or by the policy giving up, and by nothing else.
        let mark = TheRestartMark::none_is_set();
        assert_eq!(mark.where_it_stands(), WhereTheMarkStands::NoMarkWasEverSet);
        assert!(!mark.stands());
        mark.set();
        assert!(mark.stands());

        // **The user stopping the wait clears nothing** — T6 in as many words.
        assert!(!mark.cleared_by(HowTheWaitEnds::TheUserStopsWaiting));
        assert!(mark.stands());

        assert!(mark.cleared_by(HowTheWaitEnds::TheMachineAnswers));
        assert_eq!(
            mark.where_it_stands(),
            WhereTheMarkStands::ItWasClearedBy(HowTheWaitEnds::TheMachineAnswers)
        );
        // A mark already cleared is not cleared twice, and setting it again is
        // not a second mark.
        assert!(!mark.cleared_by(HowTheWaitEnds::ThePolicyGivesUp));
        mark.set();
        assert!(!mark.stands());
    }

    #[test]
    fn the_handle_is_one_fact_and_not_a_copy_per_holder() {
        let held_by_the_period = TheRestartMark::none_is_set();
        let held_by_the_setup = held_by_the_period.clone();
        held_by_the_setup.set();
        assert!(held_by_the_period.stands());
        held_by_the_period.cleared_by(HowTheWaitEnds::ThePolicyGivesUp);
        assert_eq!(
            held_by_the_setup.where_it_stands(),
            WhereTheMarkStands::ItWasClearedBy(HowTheWaitEnds::ThePolicyGivesUp)
        );
    }

    #[tokio::test]
    async fn the_wait_answers_with_whichever_of_the_two_cleared_it() {
        // T6, from the side that issued the restart: it waits, and what it is
        // told is which ending happened rather than that something happened.
        let mark = TheRestartMark::none_is_set();
        mark.set();
        let waiting = mark.clone();
        let told = tokio::spawn(async move { waiting.how_the_wait_ended().await });
        // The user stopping the wait wakes nobody, because it clears nothing.
        mark.cleared_by(HowTheWaitEnds::TheUserStopsWaiting);
        assert!(!told.is_finished());
        mark.cleared_by(HowTheWaitEnds::TheMachineAnswers);
        assert_eq!(
            told.await.expect("the wait"),
            Some(HowTheWaitEnds::TheMachineAnswers)
        );

        // **A clear that landed before anybody waited is answered at once.**
        assert_eq!(
            mark.how_the_wait_ended().await,
            Some(HowTheWaitEnds::TheMachineAnswers)
        );
    }

    #[tokio::test]
    async fn a_period_that_ends_under_the_mark_ends_the_wait_and_clears_nothing() {
        // S11: the mark is period-held, so a period that ends holds none — and
        // the wait ends with nothing to report rather than never ending.
        let mark = TheRestartMark::none_is_set();
        mark.set();
        mark.the_period_ended();
        assert!(!mark.stands());
        assert_eq!(mark.where_it_stands(), WhereTheMarkStands::ThePeriodEndedUnderIt);
        assert_eq!(mark.how_the_wait_ended().await, None);
        // **M1's closed set is untouched**: nothing here is a clearing event,
        // so a mark that ended with its period cannot be read as one that the
        // machine answered under.
        assert!(!mark.cleared_by(HowTheWaitEnds::TheMachineAnswers));
    }

    #[test]
    fn the_mark_suppresses_all_three_things_and_not_two() {
        let while_it_stands = WhileTheMarkStands::what_is_suppressed();
        assert!(while_it_stands.the_ordinary_repair_is_suppressed);
        assert!(while_it_stands.the_could_not_restore_notification_is_suppressed);
        assert!(while_it_stands.no_condition_record_is_raised);
    }
}

//! Repair — `Servers` §3.
//!
//! **N1: repair starts the moment a break is declared inside a standing period,
//! with the user doing nothing**, and every attempt belongs to the period the
//! user opened rather than starting a new one.
//!
//! **The constraint** is Row 31's two clauses, the Creator's ruling of
//! 2026-08-08 that repair does not stop for good, `Server Setup` T6's
//! requirement that **no figure may be derived from any one machine's observed
//! return cost** — against a measured spread of **9 to 90 seconds across three
//! machines and a container recycle at about 90** — and Fact 57's ten-minute
//! finding, which forbids resurrecting a connection.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C16.
//! `Desktop app and delivery` T13.

use std::time::{Duration, SystemTime};

/// **N2: a run is eight attempts.**
///
/// **One consequence is named rather than hidden: *eight* is the count in the
/// refused-at-once case and an upper bound in every other.** Where attempts run
/// to their deadline, **fewer than eight fall inside the span**. The spec
/// already carries the bound in its own words — *"a run is **up to** eight
/// attempts"*.
pub const ATTEMPTS_IN_A_RUN: u32 = 8;

/// **N2: between one attempt and the next the pause is 15 seconds, measured
/// from the *end* of the previous attempt and never from its start.**
///
/// **N3: measuring from the end is the load-bearing half of that shape.** An
/// attempt against an unreachable machine takes time to fail, and a cadence
/// measured from the start would put attempts in flight against a connection
/// nobody has declared dead yet. Fact 57's own numbers are why the two cannot
/// be separated: **the detection of a break is itself tens of seconds**, so a
/// fresh attempt made before the previous one has failed is an attempt made in
/// the dark.
pub const PAUSE_BETWEEN_ATTEMPTS: Duration = Duration::from_secs(15);

/// **N2: each attempt carries its own connect deadline of 20 seconds**, which
/// is a waiter's deadline under `Server protocol and transfers` C16 and
/// **changes no durable state when it expires**.
pub const CONNECT_DEADLINE: Duration = Duration::from_secs(20);

/// **N2a: a run's span is fixed at 105 seconds, so a run's failure lands at a
/// moment nobody has to compute from what its attempts happened to cost.**
///
/// **No attempt begins once 105 seconds have elapsed from the run's start**,
/// and an attempt still in flight at that moment is **abandoned there** —
/// abandonment being that attempt's own waiter deadline arriving early, and
/// changing no durable state.
///
/// **What forced the bound.** N5 raises the connection condition when a run
/// fails and `Desktop app and delivery` T13 posts the could-not-restore
/// notification from that record. **Unbounded, run 1 fails anywhere between 105
/// and 265 seconds**, on how a failed attempt against that particular machine
/// happens to fail — a figure nothing in this project has observed and nobody
/// chose. For one and the same three-minute outage Smoozy would then either
/// notify the user shortly before the machine came back or reconnect silently,
/// **with nothing written down deciding which**. **Bounded, the outcome is one
/// comparison.**
pub const RUN_SPAN: Duration = Duration::from_secs(105);

/// **N6: after a run fails, a further run begins after a standing pause of 60
/// seconds.** Runs continue for as long as the period stands, which is for as
/// long as the app is open unless the user disconnects.
pub const STANDING_PAUSE: Duration = Duration::from_secs(60);

/// **N4: what the numbers are derived from, and it is a spread and a floor
/// rather than any machine's cost.**
///
/// - **The pause is at or above the shortest observed return.** The spread's
///   floor is **9 seconds**; a pause under it spends attempts before the fastest
///   machine on record could have answered. Fifteen sits above it, and it is the
///   Creator's own rough shape.
/// - **A run's span sits above the spread's top.** Eight attempts and seven
///   pauses of 15 seconds are **105 seconds**, and the top of the observed
///   spread is **90 seconds**, a container recycle. So a machine returning at
///   the slowest cost anyone has watched is still inside its first run.
/// - **No figure is any one machine's measurement.** 15 is a rough shape
///   bounded below by the spread's floor; 8 is chosen so that 7 × 15 clears the
///   spread's top; 105 is that same product taken as the run's bound; 20 is a
///   waiter's own limit and not a return cost.
pub const OBSERVED_RETURN_SPREAD: (Duration, Duration) =
    (Duration::from_secs(9), Duration::from_secs(90));

/// One run of attempts, and whether another attempt may begin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run {
    /// The moment the run started, on the desktop's clock.
    pub started_at: SystemTime,
    /// How many attempts have begun in this run.
    pub attempts_begun: u32,
}

impl Run {
    pub fn begun(at: SystemTime) -> Run {
        Run {
            started_at: at,
            attempts_begun: 0,
        }
    }

    /// **N2a: no attempt begins once 105 seconds have elapsed from the run's
    /// start.** With every attempt refused at once the eight attempts fill the
    /// span exactly (7 × 15 = 105); with attempts running to their 20-second
    /// deadline, fewer fall inside it.
    ///
    /// **The boundary is inclusive, and N2a's own two sentences are what pin
    /// it.** Read *once 105 seconds have elapsed* as excluding the instant
    /// itself and the refused-at-once case gives **seven** attempts, not eight:
    /// attempt 8 falls at exactly 7 × 15 = 105. N2a states in the same
    /// paragraph that **the eight attempts fill the span exactly**, and N4
    /// derives the span as *"eight attempts and seven pauses of 15 seconds"* —
    /// **the span is measured to the start of the eighth attempt, so that
    /// attempt begins.** Nothing else moves: the run still fails at a
    /// determinate 105 seconds, and a ninth attempt is refused by the count
    /// rather than by the clock.
    pub fn another_attempt_may_begin(&self, now: SystemTime) -> bool {
        if self.attempts_begun >= ATTEMPTS_IN_A_RUN {
            return false;
        }
        match now.duration_since(self.started_at) {
            Ok(elapsed) => elapsed <= RUN_SPAN,
            // A desktop clock that moved backwards is not a reason to start an
            // attempt the span forbids.
            Err(_) => false,
        }
    }

    /// **N2a: an attempt still in flight when the span closes is abandoned
    /// there**, which changes no durable state exactly as C16's expiry does
    /// not.
    ///
    /// **It is the attempt's own start that decides**, not the run's: an
    /// attempt that began before the span closed and is still running when it
    /// does is abandoned, while the eighth attempt beginning at the boundary
    /// has not been in flight for any of it.
    pub fn an_attempt_in_flight_is_abandoned(
        &self,
        the_attempt_began_at: SystemTime,
        now: SystemTime,
    ) -> bool {
        let began_before_the_span_closed =
            matches!(the_attempt_began_at.duration_since(self.started_at), Ok(elapsed) if elapsed < RUN_SPAN);
        let the_span_has_closed =
            matches!(now.duration_since(self.started_at), Ok(elapsed) if elapsed > RUN_SPAN);
        began_before_the_span_closed && the_span_has_closed
    }

    /// **N2a's abandonment, as the deadline one attempt is given.**
    ///
    /// An attempt is raced against **the run's own remaining time** as well as
    /// against **its own connect deadline**, and is abandoned at whichever
    /// comes first. That is the rule [`Run::an_attempt_in_flight_is_abandoned`]
    /// states, expressed as a figure the attempt can be handed: a deadline that
    /// expires at [`Run::fails_at`] is an attempt abandoned exactly where the
    /// span closes, and one that expires at [`CONNECT_DEADLINE`] is the
    /// ordinary attempt N2 already bounds.
    ///
    /// **The eighth attempt beginning at the boundary is given nothing**, which
    /// is the inclusive boundary kept rather than contradicted: it may begin —
    /// `another_attempt_may_begin` says so — and it has no span left to run in,
    /// so the run still fails at a determinate 105 seconds.
    ///
    /// **A desktop clock that moved backwards leaves no remaining time**, for
    /// the same reason `another_attempt_may_begin` refuses an attempt on one:
    /// the span is not extended by arithmetic across a clock nothing defends.
    pub fn the_deadline_for_an_attempt(&self, beginning_at: SystemTime) -> Duration {
        let the_span_leaves = self
            .fails_at()
            .duration_since(beginning_at)
            .unwrap_or(Duration::ZERO);
        CONNECT_DEADLINE.min(the_span_leaves)
    }

    /// The moment this run fails, and it is determinate.
    ///
    /// **N5: the record is raised at the run's failure and not at the break
    /// declaration** — which N2a puts at a determinate 105 seconds after the
    /// declaration. **A break repaired within 105 seconds of its declaration
    /// raises nothing.**
    pub fn fails_at(&self) -> SystemTime {
        self.started_at + RUN_SPAN
    }

    /// **When the next attempt of this run begins, read at the moment the last
    /// one ended — or `None`, and the run has failed.** Corrected 2026-10-24,
    /// on the first finding of step 12's reboot and destroy scenarios, whose
    /// account is in `Audits/Build/Step 12/`.
    ///
    /// - **N2: the next attempt falls 15 seconds after the last one ended.**
    /// - **N2a: the span is measured to the start of the last attempt, so where
    ///   those 15 seconds would carry it past the span's close, it begins at
    ///   the close instead** — and is given none of the span
    ///   ([`Run::the_deadline_for_an_attempt`]). On a clock where a refusal
    ///   takes any time at all, the eighth attempt of the refused-at-once case
    ///   would otherwise fall a few milliseconds past 105 seconds and never
    ///   begin, and the run would fail at 90 — N2a's *the eight attempts fill
    ///   the span exactly* and N5's *a determinate 105 seconds* both broken by
    ///   what the attempts happened to cost, which is the figure N2a exists to
    ///   keep out. **The pause is cut only at the span's close, and the attempt
    ///   that begins there cannot reach the machine**, so what the cut moves is
    ///   the count and never whether a machine coming back is found.
    /// - **Once the span has closed, or the eighth attempt has begun, there is
    ///   no next attempt**: the run fails. An attempt abandoned at the close
    ///   ends exactly there, so the attempt at the boundary is the last.
    ///
    /// **The decision is taken here, at the attempt's end, and not again when
    /// the pause is over**: a real wait wakes a little late, and a boundary
    /// re-read on the clock after it would refuse the very attempt this
    /// granted.
    pub fn the_next_attempt_begins(&self, the_last_ended_at: SystemTime) -> Option<SystemTime> {
        if self.attempts_begun >= ATTEMPTS_IN_A_RUN {
            return None;
        }
        let closes = self.fails_at();
        if the_last_ended_at < self.started_at || the_last_ended_at >= closes {
            // A desktop clock that moved backwards is not a reason to start an
            // attempt the span forbids, as at `another_attempt_may_begin`.
            return None;
        }
        Some((the_last_ended_at + PAUSE_BETWEEN_ATTEMPTS).min(closes))
    }

    /// **How long is left before the run fails, from `now`** — what the
    /// driving waits out when the run has no next attempt before its span
    /// closes. **N2a and N5: the failure lands at [`Run::fails_at`] and never
    /// earlier**, whatever the attempts happened to cost; zero where the span
    /// has already closed.
    pub fn left_before_it_fails(&self, now: SystemTime) -> Duration {
        self.fails_at().duration_since(now).unwrap_or(Duration::ZERO)
    }
}

/// **N5: where a run fails, the server is left disconnected, carrying what
/// Smoozy said about the run that failed, and Smoozy says so.**
///
/// The failure record is `State, identity and recovery` §8's, **of kind
/// condition** — the connection condition, which is live and can stop being
/// true — and **it says only that the connection is down**. **It carries no
/// claim about why**, because Smoozy cannot tell a network problem from a
/// machine switched off from a machine that no longer exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheRecordSays {
    pub the_connection_is_down: bool,
    /// Always false. **Kept as a field rather than left out**, so a builder
    /// adding a cause finds the clause rather than an empty struct.
    pub any_claim_about_why: bool,
}

impl WhatTheRecordSays {
    pub fn raised() -> WhatTheRecordSays {
        WhatTheRecordSays {
            the_connection_is_down: true,
            any_claim_about_why: false,
        }
    }
}

/// **N8: nothing escalates, backs off further, or gives up.** There is no
/// growing pause, no attempt ceiling and no terminal state.
///
/// *"An hour with the user's own line down costs them that hour"* — with a
/// 60-second standing pause and a 105-second run, **a machine that returns is
/// picked up within about three minutes of returning**, and the hour costs the
/// hour and nothing more.
pub const LONGEST_WAIT_AFTER_A_MACHINE_RETURNS: Duration =
    Duration::from_secs(STANDING_PAUSE.as_secs() + RUN_SPAN.as_secs());

/// **N8b: one refusal is permanent and the repair stops on it rather than
/// retrying for ever.**
///
/// Where the key check's second answer is *no* because `Security and licence`
/// §3.3's store standing reads **never answered** and the row's key is
/// protected, **no run of attempts can ever change it** — the answer is a
/// property of the installation and not of the moment. The server is left
/// disconnected and marked with the permanent condition; **the standing pause
/// does not restart for it**.
///
/// **N8's *no terminal state* is untouched for every other cause**, and a
/// user-started connect still works and still asks. **The refusal stops being
/// permanent the moment the standing does**, so nothing is written that would
/// have to be cleared by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PermanentRefusal {
    pub the_store_standing_reads_never_answered: bool,
    pub the_rows_key_is_protected: bool,
}

impl PermanentRefusal {
    pub fn stops_the_repair(self) -> bool {
        self.the_store_standing_reads_never_answered && self.the_rows_key_is_protected
    }
}

/// **N10a: the repair publishes one further reading and it is *the moment the
/// next attempt is due*.**
///
/// - **What it is.** One moment on the desktop's own clock, and nothing else —
///   no attempt count, no run number, no schedule. **There is exactly one such
///   value per server row.**
/// - **When it is written, and it cannot be written earlier.** At the instant an
///   attempt ends. **N2's rule is what forces that**: the pause is measured from
///   the **end** of the previous attempt, so **the moment the next attempt falls
///   is unknown until the current one has failed**. A value published when an
///   attempt began would be wrong for that attempt's whole life.
/// - **What it reads while an attempt is in flight: absent.** There is no next
///   attempt while one is running, and **absent is a value rather than a gap**.
/// - **It covers both pauses and does not distinguish them.** N2's 15 seconds
///   inside a run and N6's 60 seconds between runs are both *when the next
///   attempt falls*. Whoever draws it never learns which pause it came from, and
///   does not need to.
/// - **Where it lives: run-held, beside the condition record and never inside
///   it.** §8.2 fixes what a condition record holds — the condition identity and
///   the set of observed values it was raised over — and **a moment is
///   neither**.
/// - **N8b's permanent refusal publishes nothing.** Where the repair has stopped
///   for good there is no next attempt, so the reading is absent and stays
///   absent — **which is the correct answer and not a missing one**.
///
/// **§16 item 20: nothing in this is observed, and the reading is a schedule
/// rather than a measurement.** It is what the repair **intends**, not what it
/// will do; a machine under load, a suspended computer or a slow failure can
/// make the attempt land later than the moment said. **§16 item 21: a computer
/// that sleeps is not handled here and no document handles it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextAttemptDue {
    /// **Absent, and it is a value.** An attempt is in flight, or N8b has
    /// stopped the repair for good, or no period stands.
    Absent,
    At(SystemTime),
}

/// Which pause the moment is computed from — used to compute it, never
/// published. **N10a: whoever draws it never learns which pause it came from.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pause {
    /// N2's 15 seconds inside a run.
    InsideARun,
    /// N6's 60 seconds between runs.
    BetweenRuns,
    /// **4.2's four runs and ten-minute wait are pauses of the same kind**, so
    /// a restart-marked period publishes the same reading. **What it does not
    /// publish is a moment for giving up**, because M7's giving up is an event
    /// rather than a schedule the user is shown.
    UnderTheRestartPolicy(bool),
}

impl Pause {
    pub fn length(self) -> Duration {
        match self {
            Pause::InsideARun => PAUSE_BETWEEN_ATTEMPTS,
            Pause::BetweenRuns => STANDING_PAUSE,
            Pause::UnderTheRestartPolicy(between_runs) => {
                if between_runs {
                    STANDING_PAUSE
                } else {
                    PAUSE_BETWEEN_ATTEMPTS
                }
            }
        }
    }
}

/// **N10a: published at the instant an attempt ends, and never earlier.**
pub fn next_attempt_due(the_attempt_ended_at: SystemTime, after: Pause) -> NextAttemptDue {
    NextAttemptDue::At(the_attempt_ended_at + after.length())
}

/// **N11 to N13: the three places repair does not run at all.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairDoesNotRun {
    /// **N11: where the period carries the restart mark**, the ordinary repair
    /// is suppressed and [`crate::restart`]'s policy runs instead. `Server
    /// Setup` §12 requires the suppression and this document performs it.
    ThePeriodCarriesTheRestartMark,
    /// **N12: where the server's key is not usable**, no attempt is made — Row
    /// 33 — and S10 item 4 ends the period.
    TheKeyIsNotUsable,
    /// **N13: where no period stands.** A server the user has not connected, a
    /// server whose put-back failed, and a server the user disconnected are all
    /// **repaired by nothing**, because repair is a property of a standing
    /// period and of nothing else.
    NoPeriodStands,
}

impl RepairDoesNotRun {
    pub const ALL: [RepairDoesNotRun; 3] = [
        RepairDoesNotRun::ThePeriodCarriesTheRestartMark,
        RepairDoesNotRun::TheKeyIsNotUsable,
        RepairDoesNotRun::NoPeriodStands,
    ];
}

/// **N9: every attempt is a fresh connection.**
///
/// K5's finding forbids anything else, and it is also what makes a repair
/// capable of meeting **a changed identity** and **a key that has gone away** —
/// **both of which are checked at every attempt and not once at the start of
/// the repair**.
pub const EVERY_ATTEMPT_IS_A_FRESH_CONNECTION: bool = true;

/// **N10: a repair that succeeds asks nothing of the user and does not need
/// their connect action again.** The condition record is withdrawn by the
/// observed value reaching *connected*, which is §8.2's derivation and needs no
/// act of this document's.
pub const A_SUCCEEDED_REPAIR_ASKS_NOTHING: bool = true;

/// **A failed run is not on S10's list of endings**, and that is Row 31's
/// *repair does not stop for good* kept by construction: **nothing ends a
/// period on a count.**
pub fn a_failed_run_ends_the_period() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn start() -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(2_000_000)
    }

    #[test]
    fn seven_pauses_of_fifteen_seconds_are_the_runs_span() {
        // N4's own derivation, and N2a takes that product as the bound.
        assert_eq!(PAUSE_BETWEEN_ATTEMPTS * (ATTEMPTS_IN_A_RUN - 1), RUN_SPAN);
    }

    #[test]
    fn the_runs_span_clears_the_top_of_the_observed_return_spread() {
        let (floor, top) = OBSERVED_RETURN_SPREAD;
        assert!(PAUSE_BETWEEN_ATTEMPTS >= floor);
        assert!(RUN_SPAN > top);
    }

    #[test]
    fn eight_attempts_fill_the_span_exactly_when_every_one_is_refused_at_once() {
        let mut run = Run::begun(start());
        let mut now = start();
        let mut begun = 0;
        while run.another_attempt_may_begin(now) {
            run.attempts_begun += 1;
            begun += 1;
            now += PAUSE_BETWEEN_ATTEMPTS;
        }
        assert_eq!(begun, ATTEMPTS_IN_A_RUN);
    }

    #[test]
    fn attempts_running_to_their_deadline_give_fewer_than_eight() {
        // N2a's own consequence: eight is the refused-at-once count and an
        // upper bound in every other case.
        let mut run = Run::begun(start());
        let mut now = start();
        let mut begun = 0;
        while run.another_attempt_may_begin(now) {
            run.attempts_begun += 1;
            begun += 1;
            now += CONNECT_DEADLINE + PAUSE_BETWEEN_ATTEMPTS;
        }
        assert!(begun < ATTEMPTS_IN_A_RUN);
        // 0, 35, 70 and 105 — the last at the boundary the eighth attempt
        // would otherwise have taken.
        assert_eq!(begun, 4);
    }

    #[test]
    fn the_eighth_attempt_begins_at_the_boundary_and_a_ninth_never_does() {
        // N2a's two sentences pin the boundary: read exclusively, the
        // refused-at-once case gives seven attempts and not the eight the same
        // paragraph states.
        let mut run = Run::begun(start());
        run.attempts_begun = ATTEMPTS_IN_A_RUN - 1;
        assert!(run.another_attempt_may_begin(start() + RUN_SPAN));
        run.attempts_begun = ATTEMPTS_IN_A_RUN;
        assert!(!run.another_attempt_may_begin(start() + RUN_SPAN));
        // Past the boundary the clock refuses it whatever the count says.
        let fresh = Run::begun(start());
        assert!(!fresh.another_attempt_may_begin(start() + RUN_SPAN + Duration::from_secs(1)));
    }

    #[test]
    fn an_attempt_that_began_inside_the_span_and_outlives_it_is_abandoned() {
        let run = Run::begun(start());
        let began = start() + Duration::from_secs(90);
        assert!(!run.an_attempt_in_flight_is_abandoned(began, start() + RUN_SPAN));
        assert!(run.an_attempt_in_flight_is_abandoned(
            began,
            start() + RUN_SPAN + Duration::from_secs(1)
        ));
        // The eighth attempt, beginning at the boundary, has not been in
        // flight for any of the span.
        assert!(!run.an_attempt_in_flight_is_abandoned(
            start() + RUN_SPAN,
            start() + RUN_SPAN + Duration::from_secs(1)
        ));
    }

    #[test]
    fn an_attempt_is_raced_against_the_span_as_well_as_against_its_own_limit() {
        // N2a: abandoned at whichever comes first. The deadline is what carries
        // the rule the predicate above states, so the two are pinned together
        // here rather than each being right on its own.
        let run = Run::begun(start());
        // Early in the span its own limit is the shorter of the two.
        assert_eq!(
            run.the_deadline_for_an_attempt(start()),
            CONNECT_DEADLINE
        );
        // Late in it the span is, and the attempt ends where the span closes
        // rather than 5 seconds past it.
        let late = start() + Duration::from_secs(90);
        assert_eq!(run.the_deadline_for_an_attempt(late), Duration::from_secs(15));
        assert!(!run.an_attempt_in_flight_is_abandoned(
            late,
            late + run.the_deadline_for_an_attempt(late)
        ));
        assert!(run.an_attempt_in_flight_is_abandoned(late, late + CONNECT_DEADLINE));
        // The eighth attempt begins at the boundary with none of the span left.
        assert_eq!(
            run.the_deadline_for_an_attempt(start() + RUN_SPAN),
            Duration::ZERO
        );
        // A desktop clock that moved backwards does not lengthen the span.
        assert_eq!(
            run.the_deadline_for_an_attempt(start() + RUN_SPAN + Duration::from_secs(1)),
            Duration::ZERO
        );
    }

    #[test]
    fn the_next_attempt_lands_on_the_close_where_its_pause_would_pass_it() {
        // Step 12's scenario finding 1: on a clock where a refusal takes time,
        // the eighth attempt's 15 seconds carried it past 105 and the run
        // failed at 90.
        let mut run = Run::begun(start());
        run.attempts_begun = 7;
        let ended = start() + Duration::from_millis(90_140);
        assert_eq!(run.the_next_attempt_begins(ended), Some(run.fails_at()));
        // Earlier in the run the pause is N2's own 15 seconds.
        run.attempts_begun = 1;
        let early = start() + Duration::from_millis(20);
        assert_eq!(
            run.the_next_attempt_begins(early),
            Some(early + PAUSE_BETWEEN_ATTEMPTS)
        );
        // An attempt that ended at the close, abandoned there, is the last.
        assert_eq!(run.the_next_attempt_begins(run.fails_at()), None);
        // The eighth attempt is the last whatever the clock says.
        run.attempts_begun = ATTEMPTS_IN_A_RUN;
        assert_eq!(run.the_next_attempt_begins(early), None);
        // A clock that moved backwards starts nothing.
        let fresh = Run::begun(start());
        assert_eq!(
            fresh.the_next_attempt_begins(start() - Duration::from_secs(1)),
            None
        );
    }

    #[test]
    fn what_is_left_before_the_run_fails_is_never_negative() {
        let run = Run::begun(start());
        assert_eq!(
            run.left_before_it_fails(start() + Duration::from_millis(90_010)),
            Duration::from_millis(14_990)
        );
        assert_eq!(run.left_before_it_fails(run.fails_at()), Duration::ZERO);
        assert_eq!(
            run.left_before_it_fails(run.fails_at() + Duration::from_millis(3)),
            Duration::ZERO
        );
    }

    #[test]
    fn a_runs_failure_lands_at_a_determinate_moment() {
        // Unbounded it would land anywhere between 105 and 265 seconds.
        assert_eq!(Run::begun(start()).fails_at(), start() + RUN_SPAN);
    }

    #[test]
    fn the_record_says_the_connection_is_down_and_never_why() {
        let said = WhatTheRecordSays::raised();
        assert!(said.the_connection_is_down);
        assert!(!said.any_claim_about_why);
    }

    #[test]
    fn a_machine_that_returns_is_picked_up_within_about_three_minutes() {
        assert_eq!(LONGEST_WAIT_AFTER_A_MACHINE_RETURNS, Duration::from_secs(165));
    }

    #[test]
    fn nothing_ends_a_period_on_a_count() {
        assert!(!a_failed_run_ends_the_period());
    }

    #[test]
    fn the_permanent_refusal_needs_both_halves() {
        assert!(PermanentRefusal {
            the_store_standing_reads_never_answered: true,
            the_rows_key_is_protected: true,
        }
        .stops_the_repair());
        // A protected key on a computer whose store answers is an ordinary
        // ask, and N8's no-terminal-state is untouched for it.
        assert!(!PermanentRefusal {
            the_store_standing_reads_never_answered: false,
            the_rows_key_is_protected: true,
        }
        .stops_the_repair());
        assert!(!PermanentRefusal {
            the_store_standing_reads_never_answered: true,
            the_rows_key_is_protected: false,
        }
        .stops_the_repair());
    }

    #[test]
    fn the_next_attempt_moment_covers_both_pauses_and_says_which_neither_time() {
        let ended = start() + Duration::from_secs(20);
        assert_eq!(
            next_attempt_due(ended, Pause::InsideARun),
            NextAttemptDue::At(ended + Duration::from_secs(15))
        );
        assert_eq!(
            next_attempt_due(ended, Pause::BetweenRuns),
            NextAttemptDue::At(ended + Duration::from_secs(60))
        );
    }

    #[test]
    fn the_restart_policys_pauses_publish_the_same_reading() {
        let ended = start();
        assert_eq!(
            next_attempt_due(ended, Pause::UnderTheRestartPolicy(false)),
            next_attempt_due(ended, Pause::InsideARun)
        );
        assert_eq!(
            next_attempt_due(ended, Pause::UnderTheRestartPolicy(true)),
            next_attempt_due(ended, Pause::BetweenRuns)
        );
    }

    #[test]
    fn absent_is_a_value_and_not_a_gap() {
        // While an attempt is in flight, and for good under N8b.
        assert_ne!(NextAttemptDue::Absent, next_attempt_due(start(), Pause::InsideARun));
    }

    #[test]
    fn the_three_places_repair_does_not_run_are_three() {
        assert_eq!(RepairDoesNotRun::ALL.len(), 3);
    }
}

//! The period and the repair, driven — `Servers` §1, §3 and §4.
//!
//! **[`crate::period`], [`crate::repair`], [`crate::restart`] and
//! [`crate::condition`] are the halves that decide, and this is the half that
//! runs them.** Every figure below is the repair's own constant, every closed
//! list is that module's enum, and **nothing here composes a third of anything**.
//!
//! **S5 is the shape of this file.** A period begins **at the act** and never
//! when the connection becomes live, so [`APeriod::begun`] is called before an
//! attempt is made and the connection is an observed value of a period that
//! already stands. That is `State, identity and recovery` §3's D2 — *"No
//! desired state is ever inferred from an observed reading"* — kept by
//! construction rather than by care.
//!
//! **K5 is the other shape.** A break declared is a break, and the connection
//! under it is torn down rather than kept for later: Fact 57's ten-minute
//! silent gap left a connection unrecoverable **even after the path was
//! restored**. So every attempt is a fresh connection (N9), and nothing here
//! ever resurrects one.
//!
//! **An ending from outside wakes the driving wherever it waits.** S10 items 1,
//! 3 and 6 arrive from another thread, and S11 ends every period-held thing the
//! moment the period ends. So every wait below — the live connection, the
//! repair's two pauses, and an attempt in flight with Q19's re-take inside it —
//! is raced against the ending, and the ending wins. *(Added 2026-09-18, step
//! 10b's first audit round: the ending only set a flag, and a healthy
//! connection went on being waited on until it ended by itself — so
//! *Disconnect*, an edit's save and a removal did nothing to it. The stand-in
//! connection ends by itself, which is what hid it.)*
//!
//! **The row is read at every attempt and not once per period.** D8 has the
//! remembered host key *"readable and writable at the moment a connection is
//! attempted"*, so each attempt asks [`TheDesktopsSide`] for the row as the
//! local record holds it then. *(Added the same day: the period used to keep
//! the copy it began with, so a repair after a first connection compared
//! against no key at all, and an accepted changed identity never took effect.)*
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` T13. `Server protocol
//! and transfers` C16. `State, identity and recovery` Y9, V1.

use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use smoozy_server_protocol::transport::Ending;

use crate::acting::{
    OpenConnections, SupplyTheCredential, TheActingHalf, TheInstallationIdentityRead, Wait,
    WhatServersPublishes,
};
use crate::condition::{observed_value, ObservedValue, WhatIsTrueOfTheServer};
use crate::first_connection::ChangedIdentityWarning;
use crate::hardware::HardwareReading;
use crate::period::{ConnectedPeriod, OpeningAct, PeriodEnding, WhoTookIt};
use crate::repair::{
    self, NextAttemptDue, PermanentRefusal, Run, WhatTheRecordSays, ATTEMPTS_IN_A_RUN,
};
use crate::restart::MarkedPeriodPolicy;
use crate::sequence::{self, AnAttempt, Attempt, ConnectionRole, WhereItStopped};
use crate::store::ServerRow;
use smoozy_state::identities::InstallationCheck;

/// **What the desktop hands the driving besides the five seams, and what the
/// driving hands back.** It is this module's own signature and not a sixth
/// seam of [`crate::acting`], whose five traits are unchanged.
///
/// - **D8: the row as the local record holds it at the moment of an attempt.**
///   The remembered host key is *"readable and writable at the moment a
///   connection is attempted"*, and an accept clears it between two attempts
///   of one repair. **Where the record answers nothing**, the attempt is made
///   on the row as it was last read — never on nothing.
/// - **Q1 steps 5 and 6: what a primary connection read**, handed back the
///   moment the connection is live and its sequence has run. `State, identity
///   and recovery` Y9 and V1 step 1 say what the desktop does with the
///   branch. What becomes of the fresh reading is this plan's H18; **nothing here does
///   either**, and nothing here drops them.
pub trait TheDesktopsSide {
    fn the_row_now(&mut self, server: &str) -> Option<ServerRow>;

    fn the_connection_read(
        &mut self,
        server: &str,
        check: InstallationCheck,
        hardware: Option<HardwareReading>,
    );
}

/// One connected period, and everything that happens under it for its life.
pub struct APeriod {
    period: ConnectedPeriod,
    /// **S10's six moments arrive here.** Two of them — the user's disconnect
    /// and the row's removal — come from outside this task, which is what the
    /// shared handle is for.
    ended: EndThePeriod,
    /// **Q17: bounded to the declared break and not to the attempt.** It lives
    /// on the period because a repair is the period's, and it is reset when the
    /// user accepts.
    warning: ChangedIdentityWarning,
    /// **E6 and E7: who took the opening act**, carried from the act rather
    /// than derived from whether the attempt is the period's first. **The
    /// put-back's own opening attempt is the period's first**, so a route that
    /// read the first attempt as the user's would raise a passphrase box on a
    /// connection Smoozy makes on its own.
    opened_by_whom: WhoTookIt,
    /// **Step 4's *Runs on* column.** True until a connection to this server has
    /// completed the sequence once.
    the_first_connection_to_this_server: bool,
    /// **N8b: one refusal is permanent and the repair stops on it.** Where it
    /// stands, there is no next attempt and the reading stays absent.
    stopped_for_good: bool,
}

/// What ends a period from outside the task driving it.
///
/// **S10 item 1's disconnect is available for as long as a period stands** —
/// including while the server reads as reconnecting, and including while it
/// reads as disconnected between runs — and **taking it stops the repair**.
/// Item 6's removal arrives the same way, at the local forget.
///
/// **Setting it wakes the driving**, wherever it is waiting — S11 is *the
/// moment the period ends*, and a flag nobody reads until the connection ends
/// by itself is not that moment.
#[derive(Clone)]
pub struct EndThePeriod(Arc<TheEnding>);

struct TheEnding {
    held: Mutex<Option<PeriodEnding>>,
    wakes: tokio::sync::Notify,
}

impl EndThePeriod {
    fn afresh() -> EndThePeriod {
        EndThePeriod(Arc::new(TheEnding {
            held: Mutex::new(None),
            wakes: tokio::sync::Notify::new(),
        }))
    }

    /// **The first ending wins.** A period ends once, and a second caller
    /// arriving after it is not a second ending.
    pub fn at(&self, ending: PeriodEnding) {
        let first = {
            let mut held = self.0.held.lock().expect("the ending");
            let first = held.is_none();
            if first {
                *held = Some(ending);
            }
            first
        };
        if first {
            self.0.wakes.notify_waiters();
        }
    }

    pub fn has_ended(&self) -> Option<PeriodEnding> {
        *self.0.held.lock().expect("the ending")
    }

    /// **Waits for an ending, and answers at once where one has arrived.** The
    /// waiter is registered before the ending is looked at, so an ending that
    /// lands between the look and the wait is never missed.
    pub async fn arrives(&self) -> PeriodEnding {
        loop {
            let notified = self.0.wakes.notified();
            let mut notified = std::pin::pin!(notified);
            notified.as_mut().enable();
            if let Some(ending) = self.has_ended() {
                return ending;
            }
            notified.await;
        }
    }
}

/// What a wait raced against the ending came to.
enum Raced<T> {
    Done(T),
    Ended(PeriodEnding),
}

/// **Any wait, raced against the ending, and the ending wins.** An ending that
/// has already arrived is answered without the work being started.
async fn unless_it_ends<T>(
    ending: &EndThePeriod,
    work: impl std::future::Future<Output = T>,
) -> Raced<T> {
    tokio::select! {
        biased;
        ended = ending.arrives() => Raced::Ended(ended),
        done = work => Raced::Done(done),
    }
}

/// How one attempt came out, as the driving reads it.
enum AttemptEnded<C> {
    Live(C),
    Stopped(WhereItStopped),
    /// **An ending from outside arrived while the attempt was in flight.** The
    /// attempt is abandoned there — its future dropped, which is how Q19's
    /// re-take and every step under it end at *"the period ending"* — and the
    /// ending is the one that happened.
    ThePeriodEnded(PeriodEnding),
}

impl APeriod {
    /// **S5: a period begins at the act.** Nothing about the connection is
    /// consulted here, and the connection this act is about has not been
    /// attempted yet.
    ///
    /// **Three values and not two**: which act opened it, and **who took that
    /// act** ([`WhoTookIt`]). S8 makes the edit put-back O1 performed by Smoozy
    /// inside the user's own save, so the act alone cannot say whether P4's ask
    /// is available.
    pub fn begun(server: &str, by: OpeningAct, taken_by: WhoTookIt, at: SystemTime) -> APeriod {
        APeriod {
            period: ConnectedPeriod::begun(server, by, at),
            ended: EndThePeriod::afresh(),
            warning: ChangedIdentityWarning::none_yet(),
            opened_by_whom: taken_by,
            the_first_connection_to_this_server: true,
            stopped_for_good: false,
        }
    }

    /// **Which act opened this period**, so a caller that has to say which it
    /// was — `Reporting a problem` J2's act line, whose fixed form names it —
    /// reads it off the period rather than carrying a second copy beside it.
    /// Added 2026-09-22.
    pub fn opened_by(&self) -> OpeningAct {
        self.period.opened_by
    }

    /// **E2: the edit's put-back, begun.** It is O1
    /// ([`crate::period::THE_PUT_BACK_IS_O1`]) taken by Smoozy inside the
    /// user's own save ([`crate::period::THE_PUT_BACK_IS_TAKEN_BY`]), **so P3
    /// binds it entire and no passphrase box can appear on it**: where the new
    /// key needs a passphrase Smoozy is not holding, no attempt is made at all
    /// and the period ends at S10 item 5 with the new details saved.
    ///
    /// **Nothing calls this yet and that is the point of building it now.** The
    /// screens arrive at a later step; the rule was stated in this module with
    /// nothing driving it, so whoever wires the put-back has a route that
    /// cannot be wired wrongly rather than a rule to remember.
    pub fn begun_as_the_edits_put_back(server: &str, at: SystemTime) -> APeriod {
        APeriod::begun(
            server,
            crate::period::THE_PUT_BACK_IS_O1,
            crate::period::THE_PUT_BACK_IS_TAKEN_BY,
            at,
        )
    }

    /// **M1: the mark is `Server Setup`'s and this document takes it.** Set
    /// before the restart command leaves, and cleared by exactly two things.
    pub fn carrying_the_restart_mark(self) -> APeriod {
        self.period.restart_mark.set();
        self
    }

    /// **The handle M1's *set before the restart command leaves* is set
    /// through**, and it is the same shape as [`APeriod::end_it`]: the act
    /// arrives from another thread while this one waits on a connection.
    ///
    /// **`Server Setup` T4 supplies the mark, the moment it is set and the
    /// moment it clears; this document holds it and acts on it.** So the caller
    /// that issues the restart takes this before the command leaves, and what
    /// clears it is [`crate::restart::HowTheWaitEnds`]' closed set of two —
    /// both of which are performed below and neither of which is the caller's.
    pub fn the_restart_mark(&self) -> crate::restart::TheRestartMark {
        self.period.restart_mark.clone()
    }

    /// The handle S10 items 1 and 6 arrive on.
    pub fn end_it(&self) -> EndThePeriod {
        self.ended.clone()
    }

    /// The whole life of one period: the opening act's connection, the stretch
    /// it is up, and every repair under it — until one of S10's six moments.
    ///
    /// **`row` is the row as the act read it, and each attempt reads it again**
    /// through `desk` (D8). **What each primary connection read is handed back
    /// through `desk` too** — Q1 steps 5 and 6 — so the desktop can do what
    /// `State, identity and recovery` Y9 says with the branch, and what this
    /// plan's H18 says with the reading.
    pub async fn drive<O, K, I, P, W, D>(
        &mut self,
        row: &ServerRow,
        half: &mut TheActingHalf<O, K, I, P, W>,
        desk: &mut D,
    ) -> HowTheDrivingEnded
    where
        O: OpenConnections,
        K: SupplyTheCredential,
        I: TheInstallationIdentityRead<O::Connection>,
        P: WhatServersPublishes,
        W: Wait,
        D: TheDesktopsSide,
    {
        let mut row = row.clone();
        // The opening act's own connection. **S10 item 5 is written against
        // this connection and reaches no replacement**: a repair attempt that
        // fails is an attempt under a period that already stands.
        //
        // **It carries N2's own connect deadline and nothing shorter**: it
        // belongs to no run, so there is no run span to race it against.
        let mut connection = match self
            .one_attempt(&mut row, half, desk, true, repair::CONNECT_DEADLINE)
            .await
        {
            AttemptEnded::Live(connection) => connection,
            AttemptEnded::ThePeriodEnded(ending) => {
                return HowTheDrivingEnded::ThePeriodEnded(self.ends(half, &row, ending));
            }
            AttemptEnded::Stopped(stopped) => {
                let ending =
                    self.ends(half, &row, PeriodEnding::TheOpeningConnectionNeverBecameLive);
                // **The step travels only where the ending is the attempt's
                // own.** `ends` hands back the ending that actually happened,
                // and the first ending wins: where the user disconnected or the
                // row was removed while this attempt was in flight, that is the
                // ending, and the step the attempt stopped at claims nothing
                // about it.
                return if ending == PeriodEnding::TheOpeningConnectionNeverBecameLive {
                    HowTheDrivingEnded::TheOpeningAttemptStopped(stopped)
                } else {
                    HowTheDrivingEnded::ThePeriodEnded(ending)
                };
            }
        };

        loop {
            // **§8.5's requirement, and K10's prohibition**: the break is
            // reported by the transport on its own and nothing is sent to learn
            // of it. **Raced against the ending** — S10 items 1, 3 and 6 arrive
            // while a healthy connection is being waited on, and that is the
            // ordinary case of all three.
            let ending = self.ended.clone();
            let raced =
                unless_it_ends(&ending, half.opener.wait_until_it_ends(&mut connection)).await;
            let ending = match raced {
                Raced::Done(ending) => ending,
                Raced::Ended(ended) => {
                    // **S11: the connection is closed and the period ends the
                    // way every other ending does.**
                    half.opener.tear_down(connection).await;
                    return HowTheDrivingEnded::ThePeriodEnded(self.ends(half, &row, ended));
                }
            };
            // **K5: a break declared is a break.** The connection under it is
            // torn down rather than kept, whichever way it ended.
            half.opener.tear_down(connection).await;
            // **The primary connection is no longer live, and the observed
            // value has to move with it.** Without this the first repair
            // attempt publishes *connected* off a connection that has just been
            // torn down — which is K6's first row read off a stale fact rather
            // than off S14 condition 6's own.
            self.period.connections.primary_is_live = false;

            if !ending.is_a_break() {
                // Smoozy's own tear-down, which is not a break and reports none.
                let ended = self
                    .the_period_has_ended()
                    .unwrap_or(PeriodEnding::TheUserDisconnects);
                return HowTheDrivingEnded::ThePeriodEnded(self.ends(half, &row, ended));
            }
            if let Some(ending) = self.the_period_has_ended() {
                return HowTheDrivingEnded::ThePeriodEnded(self.ends(half, &row, ending));
            }

            // **N12 and S10 item 4: a break declared while the server's key is
            // not usable ends the period**, because the repair Row 33 forbids
            // could never run and a period whose repair cannot run promises what
            // Smoozy cannot deliver.
            if !half.credentials.the_two_answers(&row.key).the_key_is_usable {
                return HowTheDrivingEnded::ThePeriodEnded(self.ends(
                    half,
                    &row,
                    PeriodEnding::ABreakDeclaredWithTheKeyNotUsable,
                ));
            }

            match self.repair(&mut row, half, desk).await {
                Repaired::OnANewConnection(fresh) => connection = fresh,
                Repaired::ThePeriodEnded(ending) => {
                    return HowTheDrivingEnded::ThePeriodEnded(self.ends(half, &row, ending))
                }
                // **N8b: the period still stands and nothing is attempted under
                // it.** A user-started connect still works and still asks, so
                // what the caller holds is a standing period with no driving
                // under it rather than an ended one.
                Repaired::StoppedForGood => return HowTheDrivingEnded::TheRepairStoppedForGood,
            }
        }
    }

    /// **N1: repair starts the moment a break is declared inside a standing
    /// period, with the user doing nothing**, and every attempt belongs to the
    /// period the user opened rather than starting a new one.
    ///
    /// **N6: a failed run is not the end of it.** After a run fails the period
    /// stands, the server reads disconnected, and a further run begins after the
    /// standing pause — **for as long as the app is open unless the user
    /// disconnects**. **N8: nothing escalates, backs off further, or gives up.**
    ///
    /// **Both pauses are raced against the ending**, so a disconnect taken
    /// during one ends the period at once rather than after up to sixty
    /// seconds of saying nothing.
    async fn repair<O, K, I, P, W, D>(
        &mut self,
        row: &mut ServerRow,
        half: &mut TheActingHalf<O, K, I, P, W>,
        desk: &mut D,
    ) -> Repaired<O::Connection>
    where
        O: OpenConnections,
        K: SupplyTheCredential,
        I: TheInstallationIdentityRead<O::Connection>,
        P: WhatServersPublishes,
        W: Wait,
        D: TheDesktopsSide,
    {
        // **N11: where the period carries the restart mark, the ordinary repair
        // is suppressed and §4.2's policy runs instead.**
        let mut marked = MarkedPeriodPolicy::fresh();

        loop {
            if let Some(ending) = self.the_period_has_ended() {
                return Repaired::ThePeriodEnded(ending);
            }
            let started_at = half.wait.now();
            let mut run = Run::begun(started_at);
            // **Whether the next attempt begins is decided once, at the end of
            // the attempt before it** ([`Run::the_next_attempt_begins`]), and
            // not read again off the clock after the pause: a real wait wakes
            // a little late, and the attempt N2a puts at the span's close
            // would be refused by the very lateness of the wait that led to
            // it.
            let mut another_attempt = run.another_attempt_may_begin(started_at);

            while another_attempt {
                if let Some(ending) = self.the_period_has_ended() {
                    return Repaired::ThePeriodEnded(ending);
                }
                run.attempts_begun += 1;
                // **N2a's other half, and it is what makes the run's failure
                // land at 105 seconds rather than at whatever its attempts
                // happened to cost.** The attempt is raced against the run's
                // own remaining time as well as against N2's connect deadline,
                // and abandoned at whichever comes first — abandonment being
                // that attempt's own waiter deadline arriving early, which
                // changes no durable state exactly as `Server protocol and
                // transfers` C16's expiry does not.
                //
                // Without it, against a machine that swallows connections
                // rather than refusing them the run fails at 125 seconds and
                // against one that refuses at once at 105, so a machine coming
                // back at 115 is silently reconnected on one kind of outage and
                // complained about on the other.
                let deadline = run.the_deadline_for_an_attempt(half.wait.now());
                match self.one_attempt(row, half, desk, false, deadline).await {
                    AttemptEnded::Live(connection) => {
                        // **M6 item 1: the machine answers.** The mark clears,
                        // the period's observed value reaches *connected*, and
                        // `Server Setup`'s recovery run takes the setup over
                        // from the recorded boundary — which is what the side
                        // that issued the restart is waiting on.
                        //
                        // **It is cleared here and not at the period's next
                        // publish**, because the clear is what wakes that
                        // waiter, and a machine that answers and is not told of
                        // leaves a setup waiting for a machine that is already
                        // back.
                        self.period
                            .restart_mark
                            .cleared_by(crate::restart::HowTheWaitEnds::TheMachineAnswers);
                        // **N10: a repair that succeeds asks nothing of the
                        // user and does not need their connect action again.**
                        return Repaired::OnANewConnection(connection);
                    }
                    AttemptEnded::ThePeriodEnded(ending) => {
                        return Repaired::ThePeriodEnded(ending);
                    }
                    AttemptEnded::Stopped(stopped) => {
                        if self.stopped_for_good {
                            // **N8b: the standing pause does not restart for
                            // it**, and there is no next attempt to publish.
                            return self.stand_still(row, half, stopped);
                        }
                        // **N10a: published at the instant an attempt ends, and
                        // never earlier** — the pause is measured from the end
                        // of the previous attempt, so the moment is unknown
                        // until the current one has failed.
                        let ended_at = half.wait.now();
                        match run.the_next_attempt_begins(ended_at) {
                            Some(begins_at) => {
                                // **N2's 15 seconds, or less only where N2a's
                                // close cuts them** — the moment published is
                                // the moment the attempt is made.
                                half.publishes.the_next_attempt_due(
                                    &row.local_server_identifier,
                                    NextAttemptDue::At(begins_at),
                                );
                                let pause = begins_at
                                    .duration_since(ended_at)
                                    .unwrap_or(std::time::Duration::ZERO);
                                let ending = self.ended.clone();
                                if let Raced::Ended(ended) =
                                    unless_it_ends(&ending, half.wait.until(pause)).await
                                {
                                    return Repaired::ThePeriodEnded(ended);
                                }
                            }
                            None => {
                                // **N2a and N5: the run fails at its 105 seconds
                                // and never earlier.** With the last pause cut
                                // at the close, the last attempt ends there in
                                // every ordinary case; where one did not (a
                                // desktop clock that moved backwards), the rest
                                // of the span is waited out, so the record
                                // still lands at one moment. Corrected
                                // 2026-10-24: the run used to fail the moment
                                // a full pause no longer fitted, which on a
                                // real clock was 90 seconds in both shapes of
                                // outage.
                                let left = run.left_before_it_fails(ended_at);
                                if !left.is_zero() {
                                    half.publishes.the_next_attempt_due(
                                        &row.local_server_identifier,
                                        NextAttemptDue::At(
                                            run.fails_at() + repair::STANDING_PAUSE,
                                        ),
                                    );
                                    let ending = self.ended.clone();
                                    if let Raced::Ended(ended) =
                                        unless_it_ends(&ending, half.wait.until(left)).await
                                    {
                                        return Repaired::ThePeriodEnded(ended);
                                    }
                                }
                                another_attempt = false;
                            }
                        }
                    }
                }
            }

            // The run failed.
            if self.period.restart_mark.stands() {
                marked.a_run_failed();
                // **M8: while the mark stands, no connection condition record is
                // raised at all** — the observed value still moves, because
                // Smoozy knows perfectly well the connection is down, but the
                // record `Desktop app and delivery` T13 raises the notification
                // from is not raised.
                if marked.has_given_up() {
                    // **M6 item 3 and M9: the mark clears, and the condition is
                    // raised at that moment** — the first moment Smoozy has
                    // anything to tell the user that it did not already cause.
                    self.period
                        .restart_mark
                        .cleared_by(crate::restart::HowTheWaitEnds::ThePolicyGivesUp);
                    half.publishes.raise_the_connection_condition(
                        &row.local_server_identifier,
                        WhatTheRecordSays::raised(),
                    );
                    // **M7: giving up hands the period back to the ordinary
                    // repair rather than ending it.** The period stands, the
                    // suppressions lift, and the machine comes back on its own
                    // if it ever does.
                }
            } else {
                // **N5: where a run fails, the server is left disconnected,
                // carrying what Smoozy said about the run that failed.** The
                // record is raised at the run's failure and not at the break
                // declaration.
                half.publishes.raise_the_connection_condition(
                    &row.local_server_identifier,
                    WhatTheRecordSays::raised(),
                );
            }

            self.publish_the_observed_value(half, row, false);
            let due = repair::next_attempt_due(half.wait.now(), repair::Pause::BetweenRuns);
            half.publishes
                .the_next_attempt_due(&row.local_server_identifier, due);
            let ending = self.ended.clone();
            if let Raced::Ended(ended) =
                unless_it_ends(&ending, half.wait.until(repair::STANDING_PAUSE)).await
            {
                return Repaired::ThePeriodEnded(ended);
            }
        }
    }

    /// **N8b: the server is left disconnected and marked with the permanent
    /// condition, and the standing pause does not restart for it.**
    ///
    /// **The refusal stops being permanent the moment the standing does**, so
    /// nothing durable is written that would have to be cleared by hand — this
    /// leaves the period standing and stops attempting, and a user-started
    /// connect still works and still asks.
    fn stand_still<O, K, I, P, W>(
        &mut self,
        row: &ServerRow,
        half: &mut TheActingHalf<O, K, I, P, W>,
        _stopped: WhereItStopped,
    ) -> Repaired<O::Connection>
    where
        O: OpenConnections,
        K: SupplyTheCredential,
        I: TheInstallationIdentityRead<O::Connection>,
        P: WhatServersPublishes,
        W: Wait,
    {
        half.publishes.raise_the_connection_condition(
            &row.local_server_identifier,
            WhatTheRecordSays::raised(),
        );
        self.publish_the_observed_value(half, row, false);
        // **N8b publishes nothing**: there is no next attempt, so the reading is
        // absent and stays absent — the correct answer and not a missing one.
        half.publishes
            .the_next_attempt_due(&row.local_server_identifier, NextAttemptDue::Absent);
        Repaired::StoppedForGood
    }

    /// One attempt: Q1's sequence, with the observed value moving around it.
    ///
    /// **D8: the row is read afresh first**, so the comparison is made against
    /// the host key the record holds now — the one the first connection wrote,
    /// or none after an accept cleared it — and never against the copy the
    /// period began with. **The whole attempt is raced against the ending.**
    async fn one_attempt<O, K, I, P, W, D>(
        &mut self,
        row: &mut ServerRow,
        half: &mut TheActingHalf<O, K, I, P, W>,
        desk: &mut D,
        the_opening_act: bool,
        deadline: std::time::Duration,
    ) -> AttemptEnded<O::Connection>
    where
        O: OpenConnections,
        K: SupplyTheCredential,
        I: TheInstallationIdentityRead<O::Connection>,
        P: WhatServersPublishes,
        W: Wait,
        D: TheDesktopsSide,
    {
        if let Some(fresh) = desk.the_row_now(&row.local_server_identifier) {
            *row = fresh;
        }
        let row: &ServerRow = row;

        // **N10a: what the reading says while an attempt is in flight is
        // absent**, and absent is a value rather than a gap.
        half.publishes
            .the_next_attempt_due(&row.local_server_identifier, NextAttemptDue::Absent);
        self.publish_the_observed_value(half, row, true);

        let attempt = AnAttempt {
            row,
            who: if the_opening_act {
                // **E6 and E7: who took the opening act, carried rather than
                // derived.** The put-back's own opening attempt is the period's
                // first, so reading *first attempt* as *the user's* would put a
                // passphrase box on screen at a moment the user is looking
                // elsewhere.
                self.opened_by_whom.as_the_attempt_reads_it()
            } else {
                // **P3 binds every connection Smoozy makes on its own**, which
                // is what makes *an automatic attempt never puts a passphrase
                // prompt on screen* true by not making the attempt.
                crate::key_moments::WhoStartedIt::SmoozyOnItsOwn
            },
            the_first_connection_to_this_server: self.the_first_connection_to_this_server,
            role: ConnectionRole::Primary,
            deadline,
        };

        // **K6's *connected* is published the moment the transport reports the
        // connection authenticated and usable**, which is Q1 step 3 — never at
        // the end of the whole sequence of readings.
        //
        // **K12, W16 and Q19 are what force the moment.** K12: a supervisor
        // that does not answer *"is never read as a broken connection… The
        // server connection stays up."* W16: after a username edit the server
        // connection is *"**not** marked down"*. Q19 re-takes step 5 *"on the
        // primary connection that is already live"*, which says in its own
        // words that the connection is live while the reading runs. Published
        // at the end of the sequence instead, the row read *reconnecting* for
        // six to fifteen seconds after every repair, and **for the rest of the
        // run after a username edit**, over a connection Smoozy was running
        // requests on.
        let a_period_stands = self.the_period_has_ended().is_none();
        let the_restart_mark_stands = self.period.restart_mark.stands();
        let server = row.local_server_identifier.as_str();
        let the_ending = self.ended.clone();
        let raced = {
            let APeriod {
                period, warning, ..
            } = &mut *self;
            let primary_is_live = &mut period.connections.primary_is_live;
            let mut it_is_authenticated_and_usable = |publishes: &mut P| {
                *primary_is_live = true;
                publishes.the_observed_value(
                    server,
                    observed_value(WhatIsTrueOfTheServer {
                        a_period_stands,
                        the_primary_connection_is_live: true,
                        a_run_of_attempts_is_in_flight: true,
                        the_restart_mark_stands,
                    }),
                );
            };
            unless_it_ends(
                &the_ending,
                sequence::run(attempt, half, warning, &mut it_is_authenticated_and_usable),
            )
            .await
        };

        match raced {
            Raced::Ended(ending) => {
                // **The attempt was abandoned and nothing it held is live.**
                self.period.connections.primary_is_live = false;
                AttemptEnded::ThePeriodEnded(ending)
            }
            Raced::Done(Attempt::ItIsLive(gave)) => {
                self.the_first_connection_to_this_server = false;
                // **The value was published at authentication and is not
                // published a second time here**: nothing about the server has
                // moved since, and K6's *connected* is one value rather than
                // one per step that finished.
                debug_assert!(self.period.connections.primary_is_live);
                // **N10: the condition record is withdrawn by the observed
                // value reaching *connected***, which is §8.2's derivation and
                // needs no act of this document's.
                half.publishes
                    .withdraw_the_connection_condition(&row.local_server_identifier);
                // **Q1 steps 5 and 6 leave the driving.** The branch and the
                // fresh reading are the desktop's to act on — this plan's H18,
                // and `State, identity and recovery` Y9; a primary connection always carries a
                // branch, and the reading is absent on the fourth.
                if let Some(check) = gave.installation {
                    desk.the_connection_read(&row.local_server_identifier, check, gave.hardware);
                }
                AttemptEnded::Live(gave.connection)
            }
            Raced::Done(Attempt::ItStopped(stopped)) => {
                // **A sequence abandoned after authentication puts the value
                // back.** Step 4's not-Linux outcomes 2 and 3 tear the
                // connection down, and Q19's re-take ends where there is no
                // connection left to read on; in both, S10 item 5 says the
                // connection the act was to make never became live.
                self.period.connections.primary_is_live = false;
                if let WhereItStopped::TheKeyCheck(answers) = stopped {
                    // **N8b: asked rather than composed.** Neither half of the
                    // permanence is derivable from the two answers — it is the
                    // store standing and whether the row's key is protected —
                    // and a repair that guessed would either retry for ever
                    // against an answer that cannot change or stop for good on
                    // one that can.
                    let refusal: PermanentRefusal =
                        half.credentials.the_refusal_is_permanent(&row.key);
                    if !answers.a_passphrase_is_available_without_asking
                        && refusal.stops_the_repair()
                        && !the_opening_act
                    {
                        self.stopped_for_good = true;
                    }
                }
                AttemptEnded::Stopped(stopped)
            }
        }
    }

    /// **K6's value, derived from the two facts K8a keys it on**, and never
    /// composed here.
    fn publish_the_observed_value<O, K, I, P, W>(
        &self,
        half: &mut TheActingHalf<O, K, I, P, W>,
        row: &ServerRow,
        a_run_of_attempts_is_in_flight: bool,
    ) where
        P: WhatServersPublishes,
    {
        let value = observed_value(WhatIsTrueOfTheServer {
            a_period_stands: self.the_period_has_ended().is_none(),
            the_primary_connection_is_live: self.period.connections.primary_is_live,
            a_run_of_attempts_is_in_flight,
            the_restart_mark_stands: self.period.restart_mark.stands(),
        });
        half.publishes
            .the_observed_value(&row.local_server_identifier, value);
    }

    fn the_period_has_ended(&self) -> Option<PeriodEnding> {
        self.ended.has_ended()
    }

    /// **S11: every period-held thing ends the moment the period ends, however
    /// it ends**, and the observed value goes to *disconnected* because no
    /// period stands.
    fn ends<O, K, I, P, W>(
        &mut self,
        half: &mut TheActingHalf<O, K, I, P, W>,
        row: &ServerRow,
        ending: PeriodEnding,
    ) -> PeriodEnding
    where
        P: WhatServersPublishes,
    {
        self.ended.at(ending);
        self.period.connections.primary_is_live = false;
        // **S11: every period-held thing ends the moment the period ends**, and
        // the restart mark is one of them. It clears nothing — M1's set of two
        // is untouched — and what it does is end the wait `Server Setup` T6
        // holds on this period, which would otherwise wait on a period that has
        // gone.
        self.period.restart_mark.the_period_ended();
        half.publishes
            .the_observed_value(&row.local_server_identifier, ObservedValue::Disconnected);
        half.publishes
            .the_next_attempt_due(&row.local_server_identifier, NextAttemptDue::Absent);
        self.the_period_has_ended().unwrap_or(ending)
    }

    /// What a caller reads of the period itself. **S4: it holds what it holds
    /// and nothing else.**
    pub fn as_it_stands(&self) -> &ConnectedPeriod {
        &self.period
    }

    /// **Q6's accept path**: the recorded value is replaced and the next attempt
    /// goes ahead, so the warning may be raised again in a later repair.
    pub fn the_user_accepted_the_changed_identity(&mut self) {
        self.warning.the_user_accepted();
    }
}

/// How a repair ended.
enum Repaired<C> {
    /// **N9: every attempt is a fresh connection.**
    OnANewConnection(C),
    ThePeriodEnded(PeriodEnding),
    /// **N8b**, and the period stands through it.
    StoppedForGood,
}

/// How the driving of one period ended.
///
/// **Three outcomes and not one, because N8b leaves the period standing and
/// because Q13's outcome 2 is a sentence about the machine.** A period that
/// ends is S10's; a repair that stops for good is not an ending at all — *"the
/// refusal stops being permanent the moment the standing does"*, and **a
/// user-started connect still works and still asks**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheDrivingEnded {
    /// **S10 item 5, reached by the opening attempt stopping, with the step of
    /// Q1's sequence that stopped it.**
    ///
    /// **Why the step travels and the other endings carry nothing.** For eight
    /// of the nine steps this value is not read and must not be: Smoozy cannot
    /// tell a network problem from a machine switched off from a machine that
    /// no longer exists, so it says the connection was not made and claims
    /// nothing about why. **Q13's outcome 2 is the ninth and it is a different
    /// promise** — *"Smoozy says plainly that it is not a Linux machine"* —
    /// and outcome 3 beside it is the one that claims nothing. A caller that
    /// cannot tell the two apart cannot keep the clause, and a caller that
    /// composed the sentence from an ending carrying no reason would be
    /// claiming what it does not know.
    ///
    /// **Only the opening attempt can stop here and that is Q11's doing**: the
    /// not-Linux check runs on the first connection to a server, and a period
    /// whose opening attempt fails ends at once, so no repair attempt ever
    /// reaches step 4.
    TheOpeningAttemptStopped(WhereItStopped),
    /// **One of S10's six endings**, and nothing about any attempt. Reached by
    /// item 5 as well, where something outside the driving ended the period
    /// while the opening attempt was still in flight: that ending is the one
    /// that happened, and the step the attempt stopped at says nothing about
    /// it.
    ThePeriodEnded(PeriodEnding),
    TheRepairStoppedForGood,
}

/// **N2's count, restated nowhere.** Named here so a reader of this file sees
/// that the loop above takes it from the repair rather than holding an eight of
/// its own.
pub const A_RUN_IS: u32 = ATTEMPTS_IN_A_RUN;

/// Whether an ending the transport reported is a break. **K5's question**, asked
/// of the transport's own value.
pub fn is_a_break(ending: Ending) -> bool {
    ending.is_a_break()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::first_connection::NotLinuxOutcome;
    use crate::identity::{Host, KeyReference, Port, Username};
    use crate::testing::{AMachine, TheDesk};
    use crate::hardware::HardwareReading;
    use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};
    use std::time::Duration;

    fn a_row() -> ServerRow {
        ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer)
                .expect("randomness")
                .as_text(),
            Host::what_the_user_typed("5.180.183.77").expect("a host"),
            Port::what_the_user_typed(33510).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        )
    }

    /// **The desktop's side, as a test holds it**: the row the record hands
    /// back at each attempt, and what each live connection handed back.
    ///
    /// **A record that answers nothing** is what every test above the new ones
    /// holds, and each attempt is then made on the row as it was last read —
    /// the row the act was handed.
    struct TheRecord {
        /// What each read answers, in turn; **the last stands for every read
        /// after it**.
        answers: Vec<Option<ServerRow>>,
        reads: usize,
        handed_back: Vec<(InstallationCheck, Option<HardwareReading>)>,
    }

    impl TheRecord {
        fn answering_nothing() -> TheRecord {
            TheRecord::answering(vec![None])
        }

        fn answering(answers: Vec<Option<ServerRow>>) -> TheRecord {
            TheRecord {
                answers,
                reads: 0,
                handed_back: Vec::new(),
            }
        }
    }

    impl TheDesktopsSide for TheRecord {
        fn the_row_now(&mut self, _server: &str) -> Option<ServerRow> {
            let answer = self
                .answers
                .get(self.reads)
                .or_else(|| self.answers.last())
                .cloned()
                .flatten();
            self.reads += 1;
            answer
        }

        fn the_connection_read(
            &mut self,
            _server: &str,
            check: InstallationCheck,
            hardware: Option<HardwareReading>,
        ) {
            self.handed_back.push((check, hardware));
        }
    }

    fn a_period(desk: &TheDesk) -> APeriod {
        APeriod::begun(
            "srv-1",
            OpeningAct::TheUsersConnectAction,
            WhoTookIt::TheUser,
            desk.clock.now(),
        )
    }

    /// A period wired so that a connection nothing breaks ends the application
    /// run rather than hanging the test.
    fn driven(desk: &mut TheDesk) -> APeriod {
        let period = a_period(desk);
        desk.half.opener.when_nothing_more_breaks(period.end_it());
        period
    }

    /// The user disconnects after this many pauses, which is what stopping a
    /// standing repair by hand looks like from in here.
    fn the_user_disconnects_after(desk: &mut TheDesk, period: &APeriod, pauses: usize) {
        desk.half.wait.ends_the_period_after =
            Some((pauses, period.end_it(), PeriodEnding::TheUserDisconnects));
    }

    #[tokio::test]
    async fn a_period_begins_at_the_act_and_not_at_the_connection_becoming_live() {
        // S5 and `State, identity and recovery` D2: the period stands before
        // anything has been attempted.
        let desk = TheDesk::with(AMachine::answering());
        let period = a_period(&desk);
        assert!(!period.as_it_stands().connections.primary_is_live);
        assert_eq!(
            period.as_it_stands().opened_by,
            OpeningAct::TheUsersConnectAction
        );
    }

    #[tokio::test]
    async fn an_opening_connection_that_never_becomes_live_ends_the_period() {
        // S10 item 5, and it is written against the opening act's own
        // connection: no repair runs under it, because a repair is a
        // replacement under a period that already has a connection.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().refusing_for_ever());
        let mut period = driven(&mut desk);
        let ended = period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(
            ended,
            HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheAddressDidNotAnswer)
        );
        assert_eq!(desk.half.opener.attempts_made(), 1);
    }

    #[tokio::test]
    async fn a_run_is_eight_attempts_and_fills_the_span_exactly_when_each_is_refused_at_once() {
        // N2, N2a and N4: 7 × 15 = 105, and the span is measured to the start
        // of the eighth attempt, so that attempt begins.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(8));
        let mut period = driven(&mut desk);
        let started = desk.clock.now();
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        // The opening attempt, then a run of eight, then the tenth succeeds
        // after the standing pause.
        assert_eq!(desk.half.opener.attempts_made(), 10);
        assert_eq!(
            desk.clock.now().duration_since(started).expect("forward"),
            repair::RUN_SPAN + repair::STANDING_PAUSE
        );
    }

    #[tokio::test]
    async fn the_pause_is_measured_from_the_end_of_the_previous_attempt() {
        // N3: an attempt against an unreachable machine takes time to fail, and
        // a cadence measured from the start would put attempts in flight
        // against a connection nobody has declared dead yet.
        let mut machine = AMachine::answering().opening_then_refusing(2);
        machine.each_attempt_costs = Duration::from_secs(20);
        let row = a_row();
        let mut desk = TheDesk::with(machine);
        let mut period = driven(&mut desk);
        let started = desk.clock.now();
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(
            desk.clock.now().duration_since(started).expect("forward"),
            Duration::from_secs(20 + 20 + 15 + 20 + 15 + 20)
        );
    }

    #[tokio::test]
    async fn every_attempt_carries_the_connect_deadline() {
        // N2's third figure, which is a waiter's deadline and changes no
        // durable state when it expires.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(2));
        let mut period = driven(&mut desk);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert!(desk
            .half
            .opener
            .deadlines
            .iter()
            .all(|deadline| *deadline == repair::CONNECT_DEADLINE));
    }

    #[tokio::test]
    async fn a_run_that_fails_raises_the_condition_and_says_nothing_about_why() {
        // N5: raised at the run's failure and not at the break declaration, and
        // it carries no claim about why — Smoozy cannot tell a network problem
        // from a machine switched off from a machine that no longer exists.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(8));
        let mut period = driven(&mut desk);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(desk.half.publishes.conditions_raised.len(), 1);
        assert!(desk.half.publishes.conditions_raised[0].the_connection_is_down);
        assert!(!desk.half.publishes.conditions_raised[0].any_claim_about_why);
    }

    #[tokio::test]
    async fn a_break_repaired_inside_the_first_run_raises_nothing() {
        // N2a's whole point: a break repaired within 105 seconds of its
        // declaration raises nothing, and one that is not raises the condition
        // at 105 seconds.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(3));
        let mut period = driven(&mut desk);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert!(desk.half.publishes.conditions_raised.is_empty());
    }

    #[tokio::test]
    async fn nothing_escalates_backs_off_further_or_gives_up() {
        // N8: there is no growing pause, no attempt ceiling and no terminal
        // state — every pause is one of the repair's own two figures.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(25));
        let mut period = driven(&mut desk);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        let standing = desk
            .half
            .wait
            .waited_for
            .iter()
            .filter(|waited| **waited == repair::STANDING_PAUSE)
            .count();
        assert_eq!(standing, 3);
        assert!(desk.half.wait.waited_for.iter().all(|waited| *waited
            == repair::STANDING_PAUSE
            || *waited == repair::PAUSE_BETWEEN_ATTEMPTS));
    }

    #[tokio::test]
    async fn a_repair_that_succeeds_withdraws_the_condition_and_asks_nothing() {
        // N10, and the withdrawal is the observed value reaching *connected*.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(3));
        let mut period = driven(&mut desk);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(desk.half.publishes.conditions_withdrawn, 2);
        assert!(desk
            .half
            .publishes
            .observed_values
            .contains(&ObservedValue::Connected));
    }

    #[tokio::test]
    async fn the_next_attempt_due_is_absent_while_an_attempt_is_in_flight() {
        // N10a: absent is a value rather than a gap, and the moment is written
        // at the instant an attempt ends — never when one begins.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(2));
        let mut period = driven(&mut desk);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        let published = &desk.half.publishes.next_attempt_due;
        assert!(matches!(published[0], NextAttemptDue::Absent));
        assert!(published
            .iter()
            .any(|reading| matches!(reading, NextAttemptDue::At(_))));
    }

    #[tokio::test]
    async fn the_user_disconnecting_stops_the_repair() {
        // S10 item 1: available for as long as a period stands, including while
        // the server reads disconnected between runs, and taking it stops the
        // repair.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing_for_ever());
        let mut period = driven(&mut desk);
        the_user_disconnects_after(&mut desk, &period, 3);
        let ended = period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(
            ended,
            HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheUserDisconnects)
        );
        assert_eq!(desk.half.opener.attempts_made(), 4);
    }

    #[tokio::test]
    async fn a_break_with_the_key_gone_ends_the_period_rather_than_repairing() {
        // S10 item 4 and N12: the repair Row 33 forbids could never run, and a
        // period whose repair cannot run promises what Smoozy cannot deliver.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing_for_ever());
        desk.half.credentials.usable_for_the_first = Some(1);
        let mut period = driven(&mut desk);
        let ended = period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(
            ended,
            HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::ABreakDeclaredWithTheKeyNotUsable)
        );
        // Row 33: no attempt was made after the key went.
        assert_eq!(desk.half.opener.attempts_made(), 1);
    }

    #[tokio::test]
    async fn the_permanent_refusal_stops_the_repair_and_leaves_the_period_standing() {
        // N8b, and the two halves of it are asked rather than composed. The
        // period stands, so a user-started connect still works and still asks.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing_for_ever());
        let mut period = driven(&mut desk);
        // The opening act is the user's and may ask; everything after it is
        // Smoozy's own and may not, so the repair meets a *no* that no run of
        // attempts can ever change.
        desk.half.credentials.permanent = true;
        desk.half.credentials.a_passphrase_is_available = false;
        let ended = period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(ended, HowTheDrivingEnded::TheRepairStoppedForGood);
        // **N8b publishes nothing**: the reading is absent and stays absent.
        assert!(matches!(
            desk.half.publishes.next_attempt_due.last(),
            Some(NextAttemptDue::Absent)
        ));
    }

    #[tokio::test]
    async fn a_permanent_refusal_needs_the_passphrase_answer_and_not_the_flag_alone() {
        // The flag says the installation could never answer; it stops nothing
        // while a passphrase is available, which is what keeps N8's *no
        // terminal state* untouched for every other cause.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(3));
        desk.half.credentials.permanent = true;
        let mut period = driven(&mut desk);
        let ended = period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(
            ended,
            HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheApplicationRunEnds)
        );
    }

    #[tokio::test]
    async fn no_condition_record_is_raised_while_the_restart_mark_stands() {
        // M8: the observed value still moves — Smoozy knows perfectly well the
        // connection is down, because it took it down — but the record the
        // could-not-restore notification is posted from is not raised.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing_for_ever());
        let mut period = driven(&mut desk).carrying_the_restart_mark();
        the_user_disconnects_after(&mut desk, &period, 24);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert!(desk.half.publishes.conditions_raised.is_empty());
    }

    #[tokio::test]
    async fn the_marked_policy_gives_up_after_four_runs_and_raises_the_condition_then() {
        // M4, M6 item 3 and M9: four runs of the ordinary shape, and the
        // condition is raised at the moment it gives up — the first moment
        // Smoozy has anything to tell the user that it did not already cause.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing_for_ever());
        let mut period = driven(&mut desk).carrying_the_restart_mark();
        the_user_disconnects_after(&mut desk, &period, 32);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(desk.half.publishes.conditions_raised.len(), 1);
        // **M7: giving up hands the period back to the ordinary repair rather
        // than ending it**, so the mark is gone and the period stood on.
        assert!(!period.as_it_stands().restart_mark.stands());
    }

    #[tokio::test]
    async fn the_machine_answering_clears_the_mark_and_says_which_ending_it_was() {
        // M6 item 1, which is the ending `Server Setup` T5's recovery run hangs
        // on: the machine answers, the mark clears, and the side that issued
        // the restart is told which of the two endings happened.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(3));
        let mut period = driven(&mut desk).carrying_the_restart_mark();
        let mark = period.the_restart_mark();
        assert!(mark.stands());
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert!(!mark.stands());
        assert_eq!(
            mark.how_the_wait_ended().await,
            Some(crate::restart::HowTheWaitEnds::TheMachineAnswers)
        );
        // **M8 and M9: nothing was said to the user about a connection Smoozy
        // took down itself**, the run that failed before the machine answered
        // included.
        assert!(desk.half.publishes.conditions_raised.is_empty());
    }

    #[tokio::test]
    async fn the_mark_is_set_on_a_period_already_running_and_not_only_at_its_act() {
        // M1: *set before the restart command leaves*, which is the middle of a
        // period's life — the driving is waiting on the connection the restart
        // is about to kill. The handle is what makes that reachable at all.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing_for_ever());
        let mut period = driven(&mut desk);
        let mark = period.the_restart_mark();
        assert!(!mark.stands());
        mark.set();
        the_user_disconnects_after(&mut desk, &period, 24);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        // The driving read the mark that was set from outside it, so the runs
        // it allows ran with nothing said to the user.
        assert!(desk.half.publishes.conditions_raised.is_empty());
        // **Neither of M1's two cleared it** — the machine never answered and
        // the policy had not given up — and the period ending under it is S11
        // rather than a third clearing event.
        assert_eq!(
            mark.where_it_stands(),
            crate::restart::WhereTheMarkStands::ThePeriodEndedUnderIt
        );
    }

    #[tokio::test]
    async fn a_connection_nothing_breaks_stands_until_the_run_ends() {
        // S10 item 2, and S12 from the other side: nothing under a period that
        // is never broken is repaired, because there is nothing to repair.
        let row = a_row();
        let mut desk =
            TheDesk::with(AMachine::answering().whose_connection_never_breaks());
        let mut period = driven(&mut desk);
        let ended = period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(
            ended,
            HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheApplicationRunEnds)
        );
        assert_eq!(desk.half.opener.attempts_made(), 1);
        assert!(desk.half.publishes.conditions_raised.is_empty());
    }

    #[tokio::test]
    async fn every_attempt_is_a_fresh_connection_and_none_is_resurrected() {
        // K5 and N9: the connection under a declared break is torn down rather
        // than kept, so the tear-downs match the connections that were live.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(3));
        let mut period = driven(&mut desk);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(desk.half.opener.torn_down(), 2);
        assert!(repair::EVERY_ATTEMPT_IS_A_FRESH_CONNECTION);
    }

    #[tokio::test]
    async fn a_run_fails_at_the_span_even_when_its_attempts_run_to_their_full_limit() {
        // **N2a's other half, which nothing drove.** Every other driving test
        // here uses attempts that fail instantly, so the run filled its span by
        // its pauses alone and the missing abandonment never showed. Against a
        // machine that swallows connections rather than refusing them, the run
        // used to fail at 125 seconds — so a machine coming back at 115 was
        // silently reconnected on one kind of outage and complained about on
        // the other, on nothing but how the far end happens to fail.
        let mut machine = AMachine::answering().opening_then_refusing_for_ever();
        machine.each_attempt_costs = repair::CONNECT_DEADLINE;
        let row = a_row();
        let mut desk = TheDesk::with(machine);
        let started = desk.clock.now();
        let mut period = driven(&mut desk);
        // The user disconnects at the standing pause, which is the first wait
        // after the run has failed.
        the_user_disconnects_after(&mut desk, &period, 4);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;

        // The opening attempt cost the machine's own 20 seconds and the run
        // began when it broke, so the run's failure lands at 20 + 105.
        assert_eq!(
            desk.half.publishes.conditions_raised_at,
            vec![started + repair::CONNECT_DEADLINE + repair::RUN_SPAN]
        );
        // **Every attempt inside the run was raced against the span**: none was
        // given more than the span had left.
        let inside_the_run = &desk.half.opener.deadlines[1..];
        assert!(inside_the_run
            .iter()
            .all(|deadline| *deadline <= repair::CONNECT_DEADLINE));
        assert_eq!(
            inside_the_run.last(),
            Some(&Duration::ZERO),
            "the attempt that begins at the boundary has none of the span left"
        );
    }

    /// **A run of attempts on a clock where they take time**: each refusal
    /// costs `each_attempt_costs`, and every wait wakes a little late, as a
    /// real one does. Returns when the run began — the break's declaration —
    /// and the desk after the user disconnected at the standing pause.
    async fn a_run_on_a_clock_where_attempts_take_time(
        each_attempt_costs: Duration,
        pauses_in_the_run: usize,
    ) -> (SystemTime, TheDesk) {
        let mut machine = AMachine::answering().opening_then_refusing_for_ever();
        machine.each_attempt_costs = each_attempt_costs;
        let row = a_row();
        let mut desk = TheDesk::with(machine);
        desk.half.wait.wakes_late_by = Duration::from_millis(5);
        let started = desk.clock.now();
        let mut period = driven(&mut desk);
        // The standing pause is the first wait after the run has failed.
        the_user_disconnects_after(&mut desk, &period, pauses_in_the_run + 1);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        // The opening attempt cost the machine's own time, and the break was
        // declared the moment it was live.
        (started + each_attempt_costs, desk)
    }

    #[tokio::test]
    async fn refused_at_once_on_a_real_clock_the_run_is_eight_attempts_and_fails_at_105() {
        // **Step 12's scenario finding 1, the refusal shape.** A refusal of 20
        // milliseconds and waits that wake 5 milliseconds late carried the
        // eighth attempt's 15 seconds past the span's close, so the run was
        // seven attempts and failed at 90.17 seconds. N2a: the eight attempts
        // fill the span exactly; N5: the record lands at 105.
        let (declared, desk) =
            a_run_on_a_clock_where_attempts_take_time(Duration::from_millis(20), 7).await;
        assert_eq!(
            desk.half.opener.attempts_made(),
            1 + repair::ATTEMPTS_IN_A_RUN as usize,
            "the opening attempt, then a run of eight"
        );
        assert_eq!(desk.half.publishes.conditions_raised_at.len(), 1);
        let after = desk.half.publishes.conditions_raised_at[0]
            .duration_since(declared)
            .expect("forward");
        assert!(
            after >= repair::RUN_SPAN && after <= repair::RUN_SPAN + Duration::from_millis(5),
            "raised {after:?} after the declaration"
        );
        // The eighth attempt begins at the close, with none of the span left.
        assert_eq!(desk.half.opener.deadlines.last(), Some(&Duration::ZERO));
        // **N10a: the moment published for it is the close itself.**
        assert!(desk
            .half
            .publishes
            .next_attempt_due
            .contains(&NextAttemptDue::At(declared + repair::RUN_SPAN)));
    }

    #[tokio::test]
    async fn attempts_running_to_their_deadline_on_a_real_clock_fail_the_run_at_105() {
        // **Step 12's scenario finding 1, the silence shape.** Attempts at 0,
        // 35 and 70 each ran their 20 seconds, the waits woke late, and the
        // fourth attempt's moment fell at 105.01: the run failed at 90.01.
        let (declared, desk) =
            a_run_on_a_clock_where_attempts_take_time(repair::CONNECT_DEADLINE, 3).await;
        assert_eq!(
            desk.half.opener.attempts_made(),
            1 + 4,
            "the opening attempt, then 0, 35, 70 and the one at the close"
        );
        assert_eq!(desk.half.publishes.conditions_raised_at.len(), 1);
        let after = desk.half.publishes.conditions_raised_at[0]
            .duration_since(declared)
            .expect("forward");
        assert!(
            after >= repair::RUN_SPAN && after <= repair::RUN_SPAN + Duration::from_millis(5),
            "raised {after:?} after the declaration"
        );
        assert_eq!(desk.half.opener.deadlines.last(), Some(&Duration::ZERO));
    }

    #[tokio::test]
    async fn a_slow_refusal_running_into_the_close_is_abandoned_there_and_the_run_fails_at_105() {
        // **N2a and N5, with refusals that each take 10 seconds**: the fifth
        // attempt's pause is not cut, it begins at 100 with 5 seconds of the
        // span left, and is abandoned at the close — so the run fails at 105
        // and not at the 110 the refusal would have taken.
        let mut machine = AMachine::answering().opening_then_refusing_for_ever();
        machine.each_attempt_costs = Duration::from_secs(10);
        let row = a_row();
        let mut desk = TheDesk::with(machine);
        let started = desk.clock.now();
        let mut period = driven(&mut desk);
        // 0–10, 25–35, 50–60, 75–85, 100–105; the standing pause is the
        // fifth wait.
        the_user_disconnects_after(&mut desk, &period, 5);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        let declared = started + Duration::from_secs(10);
        assert_eq!(desk.half.opener.attempts_made(), 1 + 5);
        assert_eq!(
            desk.half.publishes.conditions_raised_at,
            vec![declared + repair::RUN_SPAN]
        );
    }

    #[tokio::test]
    async fn a_reconnection_after_an_edit_makes_no_attempt_where_no_passphrase_is_available() {
        // **E6, E7 and P3.** The put-back is a connection Smoozy makes on its
        // own, so where the new key needs a passphrase Smoozy is not holding
        // the put-back is not attempted at all: the server is left disconnected
        // with the new details saved. **It is the period's first attempt**, so
        // deriving *who started it* from that would put a passphrase box on
        // screen at a moment the user is looking elsewhere.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering());
        desk.half.credentials.a_passphrase_is_available = false;
        let mut period = APeriod::begun_as_the_edits_put_back("srv-1", desk.clock.now());
        desk.half.opener.when_nothing_more_breaks(period.end_it());

        let ended = period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(
            ended,
            HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheKeyCheck(
                desk.half.credentials.the_two_answers(&row.key)
            ))
        );
        assert_eq!(desk.half.opener.attempts_made(), 0, "no attempt is made");
        assert_eq!(desk.half.credentials.asks, 0, "and nobody is asked");
        assert_eq!(
            desk.half.publishes.observed_values.last(),
            Some(&ObservedValue::Disconnected)
        );
    }

    #[tokio::test]
    async fn the_same_period_opened_by_the_user_does_make_the_attempt_and_asks() {
        // The other side of the same value: P4 lets a connect the user started
        // ask, so the first answer alone gates it.
        //
        // **The connection is one nothing breaks**, so the opening act's own
        // attempt is the only attempt there is. A machine whose live connection
        // breaks would open a standing repair here that no attempt can ever
        // finish — P3 refuses every automatic attempt while no passphrase is
        // available, and N9's *nothing gives up* means it would run for ever —
        // which is this test's own wiring rather than anything about the rule
        // it is here to hold.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().whose_connection_never_breaks());
        desk.half.credentials.a_passphrase_is_available = false;
        let mut period = driven(&mut desk);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(desk.half.opener.attempts_made(), 1);
        assert_eq!(desk.half.credentials.asks, 1);
    }

    #[tokio::test]
    async fn the_value_published_while_the_identity_read_is_re_taken_is_connected() {
        // **F-1's whole point: K12, W16 and Q19.** The connection is live the
        // moment the transport reports it authenticated and usable, so the
        // re-taken reading runs under a server that reads *connected*. Marked
        // at the end of the sequence instead, the row read *reconnecting* for
        // six to fifteen seconds after every repair and, after a username edit,
        // for the rest of the run over a connection that works.
        //
        // **One connection and one sequence**: the connection is one nothing
        // breaks, so the three reads counted below are the opening sequence's
        // own re-take and not a repair's sequence counted in with it.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().whose_connection_never_breaks());
        desk.half.installation.answers = vec![
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer,
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer,
            InstallationCheck::PresentAndEqualConfirmed,
        ];
        let mut period = driven(&mut desk);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;

        assert_eq!(desk.half.installation.reads_taken, 3);
        assert_eq!(
            desk.half.installation.value_at_each_read,
            vec![Some(ObservedValue::Connected); 3],
            "the reading is re-taken on a primary connection that is already live"
        );
    }

    #[tokio::test]
    async fn a_sequence_abandoned_after_authentication_puts_the_value_back() {
        // Step 4's outcome 2 tears the connection down, and S10 item 5 ends the
        // period: the value is live while the transport says it is, and
        // disconnected the moment the sequence gives it up.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().naming_the_operating_system("Darwin"));
        let mut period = driven(&mut desk);
        let ended = period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(
            ended,
            HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheNotLinuxCheck(
                NotLinuxOutcome::ItNamedSomethingElse
            )),
            "**Q13's outcome 2 leaves this crate carrying which outcome it was.** \
             The clause is a sentence about the machine, and a caller handed an \
             ending that says only that the connection was not made cannot say it."
        );
        assert_eq!(
            desk.half.publishes.observed_values.last(),
            Some(&ObservedValue::Disconnected)
        );
        assert!(!period.as_it_stands().connections.primary_is_live);
    }

    /// **Q13's outcome 3 leaves the crate too, and it is a different value** —
    /// which is the whole reason the step travels at all. Outcome 2 says what
    /// the machine is; outcome 3 claims nothing, and a caller that could not
    /// tell them apart would either say nothing where the clause promises a
    /// sentence or claim what Smoozy does not know.
    #[tokio::test]
    async fn a_machine_that_answers_nothing_is_a_different_outcome_from_one_that_names_something() {
        let row = a_row();
        let mut desk =
            TheDesk::with(AMachine::answering().saying_nothing_about_its_operating_system());
        let mut period = driven(&mut desk);
        let ended = period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(
            ended,
            HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheNotLinuxCheck(
                NotLinuxOutcome::NothingSmoozyCouldRead
            ))
        );
    }

    /// **The step travels only where the ending is the attempt's own.** Where
    /// something outside the driving ends the period while the opening attempt
    /// is still in flight, that is the ending that happened — S10 item 1 here —
    /// and the caller is told nothing about any step.
    #[tokio::test]
    async fn an_ending_from_outside_beats_the_step_the_attempt_stopped_at() {
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().naming_the_operating_system("Darwin"));
        let mut period = driven(&mut desk);
        period.end_it().at(PeriodEnding::TheUserDisconnects);
        let ended = period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        assert_eq!(
            ended,
            HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheUserDisconnects)
        );
    }

    // ------------------------------------------ the row, read at every attempt

    /// **D8, and the first half of the fault step 10b's audit found.** A first
    /// connection records the machine's key; the period's own copy of the row
    /// still remembers none. **Every repair attempt used to compare against
    /// that copy**, take the *nothing remembered* path, trust whatever key was
    /// offered and record it again — so the host-key check was skipped on
    /// every repair of the period in which a server was first reached.
    #[tokio::test]
    async fn a_repair_after_a_first_connection_compares_against_the_key_it_recorded() {
        let row = a_row();
        assert!(row.remembered_host_key.is_none(), "a server never reached");
        let mut recorded = row.clone();
        recorded.remembered_host_key =
            Some(crate::store::RememberedHostKey::approved(AMachine::THE_HOST_KEY.to_vec()));
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(1));
        let mut period = driven(&mut desk);
        // The opening attempt reads the row as the act read it; every attempt
        // after it reads the key the opening connection wrote.
        let mut record = TheRecord::answering(vec![Some(row.clone()), Some(recorded)]);
        period.drive(&row, &mut desk.half, &mut record).await;

        assert_eq!(desk.half.opener.attempts_made(), 3);
        assert_eq!(
            desk.half.publishes.host_keys_recorded, 1,
            "**only the first connection records a key**: a repair that recorded \
             again took the nothing-remembered path and trusted what it was offered"
        );
        assert_eq!(record.reads, 3, "every attempt reads the row");
    }

    /// **The same fault's other half: accepting a changed identity during a
    /// repair never took effect.** The accept clears the stored key; the next
    /// attempt compared against the key in the period's copy, failed again,
    /// and — Q17 warning once per repair — the server repaired for ever after
    /// the user said yes.
    #[tokio::test]
    async fn an_accepted_identity_takes_effect_at_the_next_attempt_of_the_same_repair() {
        let mut row = a_row();
        row.remembered_host_key =
            Some(crate::store::RememberedHostKey::approved(AMachine::THE_HOST_KEY.to_vec()));
        let mut differs = row.clone();
        differs.remembered_host_key = Some(crate::store::RememberedHostKey::approved(
            b"the key the record held before the machine was rebuilt".to_vec(),
        ));
        let mut accepted = row.clone();
        accepted.remembered_host_key = None;

        let mut desk = TheDesk::with(AMachine::answering());
        let mut period = driven(&mut desk);
        // The opening attempt matches; two repair attempts meet a record whose
        // key differs; then the user accepts and the record holds none.
        let mut record = TheRecord::answering(vec![
            Some(row.clone()),
            Some(differs.clone()),
            Some(differs),
            Some(accepted),
        ]);
        let ended = period.drive(&row, &mut desk.half, &mut record).await;

        assert_eq!(desk.half.publishes.changed_identity_warnings, 1, "Q17: once per repair");
        assert_eq!(
            desk.half.publishes.host_keys_recorded, 1,
            "**the attempt after the accept went ahead** and recorded the key it met"
        );
        assert_eq!(desk.half.opener.attempts_made(), 4);
        assert_eq!(
            ended,
            HowTheDrivingEnded::ThePeriodEnded(PeriodEnding::TheApplicationRunEnds)
        );
    }

    /// **A record that answers nothing costs the attempt nothing**: it is made
    /// on the row as it was last read, and never on nothing.
    #[tokio::test]
    async fn a_record_that_answers_nothing_leaves_the_attempt_on_the_row_last_read() {
        let mut row = a_row();
        row.remembered_host_key = Some(crate::store::RememberedHostKey::approved(
            b"not the machine's".to_vec(),
        ));
        let mut desk = TheDesk::with(AMachine::answering());
        let mut period = driven(&mut desk);
        let ended = period
            .drive(&row, &mut desk.half, &mut TheRecord::answering_nothing())
            .await;
        assert_eq!(
            ended,
            HowTheDrivingEnded::TheOpeningAttemptStopped(WhereItStopped::TheIdentityDiffers)
        );
    }

    // --------------------------------- what a connection read, handed back

    /// **Q1 steps 5 and 6 leave the driving.** `drive` used to hand back only
    /// the connection, dropping the installation check's branch and the fresh
    /// hardware reading at the edge of this module — so nothing took `State,
    /// identity and recovery` Y9's branch against the row, and a recycled
    /// machine went on showing the old machine's hardware.
    #[tokio::test]
    async fn every_primary_connection_hands_back_its_branch_and_its_reading() {
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(1));
        desk.half.installation.answers = vec![InstallationCheck::Absent];
        let mut period = driven(&mut desk);
        let mut record = TheRecord::answering_nothing();
        period.drive(&row, &mut desk.half, &mut record).await;

        assert_eq!(record.handed_back.len(), 2, "the opening connection and the repaired one");
        for (check, hardware) in &record.handed_back {
            assert_eq!(*check, InstallationCheck::Absent);
            assert!(hardware.is_some(), "H18: the absent branch takes a fresh reading");
        }
    }

    /// **H21: the fourth branch hands back its branch and no reading**, because
    /// none is taken there; Q19's re-take settles first, and the branch handed
    /// back is the one it settled on.
    #[tokio::test]
    async fn a_connection_that_settles_after_the_fourth_branch_hands_back_where_it_settled() {
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().whose_connection_never_breaks());
        desk.half.installation.answers = vec![
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer,
            InstallationCheck::PresentAndEqualConfirmed,
        ];
        let mut period = driven(&mut desk);
        let mut record = TheRecord::answering_nothing();
        period.drive(&row, &mut desk.half, &mut record).await;
        assert_eq!(record.handed_back.len(), 1);
        assert_eq!(record.handed_back[0].0, InstallationCheck::PresentAndEqualConfirmed);
        assert!(record.handed_back[0].1.is_some());
    }

    // **An ending from outside, waking the driving, is proved against a real
    // connection** in `tests/against_a_real_machine.rs`: the stand-in
    // connection here ends by itself, which is exactly what hid the fault.

    #[tokio::test]
    async fn the_observed_value_never_reads_connected_without_a_live_connection() {
        // K6's table, over a whole period: *connected* is published only where
        // the primary connection is live.
        let row = a_row();
        let mut desk = TheDesk::with(AMachine::answering().opening_then_refusing(9));
        let mut period = driven(&mut desk);
        period.drive(&row, &mut desk.half, &mut TheRecord::answering_nothing()).await;
        let values = &desk.half.publishes.observed_values;
        assert_eq!(
            values.iter().filter(|value| **value == ObservedValue::Connected).count(),
            2,
            "the opening connection and the repaired one"
        );
        assert_eq!(values.last(), Some(&ObservedValue::Disconnected));
    }
}

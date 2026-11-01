//! **Telling now from catching up, without a clock** — `Desktop app and
//! delivery` §13.5, T18, T18a, T19, T20 and T21.
//!
//! **The boundary is the desktop's own reconciliation read and not an age.**
//! Events applied at `State, identity and recovery` §7 step 3 — read forward
//! from the cursor on a connection — **are the catch-up account** and produce
//! `Queue & History`'s one catch-up notice and no notifications. Events that
//! arrive **after** that step has completed for that server, while the
//! connected period is live, are *now* and produce notifications under the
//! four kinds.
//!
//! **No clock is involved and none is needed** (T19). `State, identity and
//! recovery` D5 forbids subtracting readings taken on different clocks; the
//! rented machine's clock and the user's own are each undefended in their own
//! way, and an age comparison would be arithmetic across two of them.
//! **There is no moment, no duration and no comparison anywhere in this
//! file**, which is the strongest form the clause can take: a builder cannot
//! reach for a clock that is not there.
//!
//! **T21: nothing about presentation is written into the journal.** What this
//! module holds is process memory for the length of a connected period and is
//! written nowhere — `State, identity and recovery` E7 forbids a presented
//! flag in the journal and gives the reason, which is that a single flag
//! cannot serve two independent readers and an event the Telegram worker
//! presented would read as presented to the desktop.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` D5, F25.
//! `Rented-server runtime` S14, U26.

use std::collections::HashSet;

use smoozy_state::identities::Identity;

/// **The two routes an event reaches this desktop by, and there is no third**
/// — T18.
///
/// **`Rented-server runtime` U26: there is no push**, so every event is read
/// forward from the cursor. What differs is *which* read-forward, and that is
/// the whole boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowItArrived {
    /// **`State, identity and recovery` §7 step 3, on a connection.** The
    /// catch-up account.
    AtTheReconciliationRead,
    /// **§3.3 P10d's live-reading sequence, inside a standing connected
    /// period.** *"What makes an event arrive then is P10d's live-reading
    /// sequence — its recurring read-forward is the only *now* route"* — and
    /// T18 says outright that without it every ending would be catch-up and
    /// this boundary would classify nothing.
    OnTheLiveReading,
}

/// **What the boundary decides.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NowOrCatchingUp {
    /// Produces a notification under the four kinds.
    Now,
    /// Produces `Queue & History`'s one catch-up notice and **no
    /// notification** — T18, and `Queue & History` C1.
    CatchingUp,
}

/// **Which servers' reconciliation read has completed in the connected period
/// now standing** — the whole of what the boundary is made of.
///
/// **It is run-held and per period.** T20: *"a reconnection after a drop
/// re-runs step 3, so everything that happened during the outage is
/// catch-up"* — which is the burst Row 284 exists to forbid — so a period
/// ending takes the server out again and the next connection starts from
/// catching up.
///
/// **Nothing durable is written here and nothing is read back from a store.**
/// A fresh launch holds none of it, which is correct: the first connection of
/// a run runs step 3 like any other.
#[derive(Debug, Default)]
pub struct WhereTheEventsAreComingFrom {
    /// Held by the identity's own written form, so that the set is one thing
    /// rather than a lifetime tangled into the boundary.
    step_three_has_completed: HashSet<String>,
}

impl WhereTheEventsAreComingFrom {
    pub fn new() -> Self {
        Self::default()
    }

    /// **`State, identity and recovery` §7 step 3 has completed for this
    /// server on this connection.** Everything read forward after this, on the
    /// live reading, is *now*.
    pub fn the_reconciliation_read_completed(&mut self, server: &Identity) {
        self.step_three_has_completed.insert(server.as_text());
    }

    /// **The connected period ended** — however it ended. T20's re-run is what
    /// follows, so the server goes back to having no completed read and the
    /// next connection's step 3 is catch-up again.
    pub fn the_connected_period_ended(&mut self, server: &Identity) {
        self.step_three_has_completed.remove(&server.as_text());
    }

    /// **T18, performed.**
    ///
    /// Two facts and no third: which read the event arrived on, and whether
    /// this server's reconciliation read has completed in the period now
    /// standing. **No moment of any kind reaches this function**, which is
    /// T19 kept by the signature rather than by care.
    pub fn how_this_event_arrived(
        &self,
        server: &Identity,
        arrived: HowItArrived,
    ) -> NowOrCatchingUp {
        match arrived {
            // **T20's first consequence, correct rather than tolerated**: an
            // event that happened seconds before a connection completed step 3
            // is still catch-up, and the notice names it, so the user is not
            // told twice.
            HowItArrived::AtTheReconciliationRead => NowOrCatchingUp::CatchingUp,
            HowItArrived::OnTheLiveReading => {
                if self.step_three_has_completed.contains(&server.as_text()) {
                    NowOrCatchingUp::Now
                } else {
                    NowOrCatchingUp::CatchingUp
                }
            }
        }
    }
}

/// **T18a: an ending learned from a landing of pending writes is classified by
/// when its events arrive, and the landing is not a second route.**
///
/// `Rented-server runtime` S14 holds a full mount's journal events **pending**
/// — the write is refused and re-attempted — so at the moment the job fails
/// there is no event to read forward and T18's boundary has nothing to
/// classify. **When those writes land, the events they carry are read forward
/// from the cursor like any others** and [`WhereTheEventsAreComingFrom`]
/// decides them with no new rule and no new state.
///
/// **This constant is what a reader looking for the second route finds instead
/// of the second route.** It exists because the absence is the clause: a
/// builder meeting a landing naturally reaches for a special case, and T18a
/// says outright that the landing *"gets no rule of its own"*.
pub const A_LANDING_OF_PENDING_WRITES_IS_NOT_A_SECOND_ROUTE: bool = true;

/// **T19: no clock is involved, deliberately.**
///
/// **`State, identity and recovery` D5 forbids subtracting readings taken on
/// different clocks.** The event's own moment on the machine may be hours
/// older than its arrival and decides nothing here; the reader's own cursor is
/// a record Smoozy wrote, and that is what the boundary is made of.
///
/// The value is the statement. **Nothing in this module takes a moment, a
/// duration or an age**, and a builder who adds one has to delete this line to
/// make the module read consistently, which is the point of it.
pub const NO_CLOCK_IS_INVOLVED: bool = true;

/// **T21: nothing about presentation is written into the journal.**
///
/// What is supplied to the presenter is the event moment and the cursor, and
/// **the decision is made here** — `State, identity and recovery` E7's reason
/// being that a single flag cannot serve two independent readers, so an event
/// the Telegram worker presented would read as presented to the desktop.
pub const NOTHING_ABOUT_PRESENTATION_IS_WRITTEN_INTO_THE_JOURNAL: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;

    fn a_server() -> Identity {
        Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness")
    }

    /// **T18, both halves.**
    #[test]
    fn the_reconciliation_read_is_catching_up_and_the_live_reading_after_it_is_now() {
        let server = a_server();
        let mut where_from = WhereTheEventsAreComingFrom::new();

        assert_eq!(
            where_from.how_this_event_arrived(&server, HowItArrived::AtTheReconciliationRead),
            NowOrCatchingUp::CatchingUp
        );
        // Before step 3 completes, even the live reading is catch-up: T18's
        // *now* is defined as after that step has completed for that server.
        assert_eq!(
            where_from.how_this_event_arrived(&server, HowItArrived::OnTheLiveReading),
            NowOrCatchingUp::CatchingUp
        );

        where_from.the_reconciliation_read_completed(&server);
        assert_eq!(
            where_from.how_this_event_arrived(&server, HowItArrived::OnTheLiveReading),
            NowOrCatchingUp::Now
        );
        // **And step 3 is still catch-up afterwards**, which is what makes a
        // reconnection's re-run safe.
        assert_eq!(
            where_from.how_this_event_arrived(&server, HowItArrived::AtTheReconciliationRead),
            NowOrCatchingUp::CatchingUp
        );
    }

    /// **T20: a reconnection re-runs step 3, so everything that happened
    /// during the outage is catch-up** — the burst Row 284 exists to forbid.
    ///
    /// This is the line that fails if somebody keeps the completed mark across
    /// a period, which would turn a two-hour outage's whole backlog into a
    /// burst of banners the moment the connection came back.
    #[test]
    fn a_reconnection_puts_the_server_back_to_catching_up() {
        let server = a_server();
        let mut where_from = WhereTheEventsAreComingFrom::new();
        where_from.the_reconciliation_read_completed(&server);
        assert_eq!(
            where_from.how_this_event_arrived(&server, HowItArrived::OnTheLiveReading),
            NowOrCatchingUp::Now
        );

        where_from.the_connected_period_ended(&server);
        assert_eq!(
            where_from.how_this_event_arrived(&server, HowItArrived::OnTheLiveReading),
            NowOrCatchingUp::CatchingUp
        );
    }

    /// **The boundary is per server**, which T18 says in its own words —
    /// *"after that step has completed for that server"*. One machine's
    /// reconciliation says nothing about another's.
    #[test]
    fn one_servers_completed_read_says_nothing_about_another() {
        let one = a_server();
        let other = a_server();
        let mut where_from = WhereTheEventsAreComingFrom::new();
        where_from.the_reconciliation_read_completed(&one);
        assert_eq!(
            where_from.how_this_event_arrived(&other, HowItArrived::OnTheLiveReading),
            NowOrCatchingUp::CatchingUp
        );
    }

    /// **T18a, both endings of it, and it needs no rule of its own.**
    ///
    /// A full mount holds the ending's journal events pending
    /// (`Rented-server runtime` S14). When they land **while the connected
    /// period is live**, they arrive on the live reading and the failed job
    /// speaks for itself under T12. When they land at a later reconciliation's
    /// step 3 — the user having quit before the space was freed — they are
    /// catch-up and there is no notification of that ending at all, which T18a
    /// names as the cost rather than leaving it to be found.
    #[test]
    fn a_landing_of_pending_writes_is_classified_by_when_its_events_arrive() {
        let server = a_server();
        let mut where_from = WhereTheEventsAreComingFrom::new();
        where_from.the_reconciliation_read_completed(&server);

        assert_eq!(
            where_from.how_this_event_arrived(&server, HowItArrived::OnTheLiveReading),
            NowOrCatchingUp::Now
        );

        // The user quit and came back: the period ended and step 3 runs again.
        where_from.the_connected_period_ended(&server);
        assert_eq!(
            where_from.how_this_event_arrived(&server, HowItArrived::AtTheReconciliationRead),
            NowOrCatchingUp::CatchingUp
        );
        assert!(A_LANDING_OF_PENDING_WRITES_IS_NOT_A_SECOND_ROUTE);
    }

    /// **T19, held by reading the file rather than by asserting a value.**
    ///
    /// The clause is that no clock is involved *deliberately*, and the way to
    /// hold that is to fail when one appears. Every name a moment could arrive
    /// under is looked for in this module's own source.
    #[test]
    fn no_clock_reaches_this_module() {
        let source = include_str!("when.rs");
        let (body, _tests) = source
            .split_once("#[cfg(test)]")
            .expect("this module has tests");
        for forbidden in [
            "SystemTime",
            "Instant",
            "Duration",
            "Reading<",
            "elapsed",
            "now()",
            "timestamp",
        ] {
            assert!(
                !body.contains(forbidden),
                "T19: no clock is involved in telling now from catching up, and `{forbidden}` \
                 has appeared in the module that decides it — State, identity and recovery D5 \
                 forbids arithmetic across two clocks neither of which is trusted"
            );
        }
        assert!(NO_CLOCK_IS_INVOLVED);
        assert!(NOTHING_ABOUT_PRESENTATION_IS_WRITTEN_INTO_THE_JOURNAL);
    }
}

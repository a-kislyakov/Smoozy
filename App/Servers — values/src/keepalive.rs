//! Holding the connection up, and learning it has ended — `Servers` §2.1 and
//! §2.3.
//!
//! **The constraint is `State, identity and recovery` §8.5**, which is a
//! requirement placed on this document rather than a decision it retakes:
//! *"the connection's observed value must come from a mechanism that reports a
//! break without the caller having to use the connection first."*
//!
//! **K1: the connection is held by the SSH transport Smoozy itself drives, and
//! that transport is the supplier of the connection's observed value.**
//!
//! **K1a: no command-line SSH client is invoked for this connection, so no
//! option handed to one governs it.** Smoozy opens, authenticates, holds up and
//! tears down this connection **inside its own process**. **No `ssh`, `sftp` or
//! `scp` process is run for it at any moment**, so `IdentityFile`, `BatchMode`,
//! `SSH_ASKPASS_REQUIRE`, `ServerAliveInterval` and every other client option
//! are not mechanisms of this document — each configures a program this
//! architecture does not run, and **a builder reaching for one has found this
//! clause rather than a gap**. What the connection is given instead is a value
//! on a call: the private key file's path at authentication, and K3's interval
//! and count as figures of Smoozy's own driving.
//!
//! **What this does not reach**: `Keys`' local readings genuinely do run
//! `ssh-keygen` as a subprocess and their flags are that document's. Nothing
//! here says a tool is never run — only that none is run to make **this**
//! connection.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C18, G8.

use std::time::Duration;

/// **K3: the keepalive interval is 15 seconds.**
///
/// Three constraints fix the figures below and **none is any one machine's
/// measured cost**.
///
/// - **It must be far below the silent case's cost.** Twenty-one minutes and
///   forty-five seconds of a caller believing a dead connection is a slow one
///   is the harm §8.5 exists to prevent, and one minute is not it.
/// - **It must be far above what one exchange on the user's own line costs.**
///   Fact 25's own words: *"most of the gain is hiding a flat 1.0–1.4 s
///   connection setup rather than adding bandwidth."* **Connection setup is the
///   upper bound and it contains a round trip rather than being one**, so a
///   keepalive exchange costs less than 1.4 s — more than ten times inside a
///   15-second interval.
/// - **It must not be so tight that a busy connection declares a break under
///   its own traffic.** The count of three is what absorbs a single lost
///   keepalive on a link carrying a transfer at the negotiated maximum write of
///   261,120 bytes per operation.
pub const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(15);

/// **K3: the unanswered count is 3.**
pub const UNANSWERED_COUNT: u32 = 3;

/// **K3's third figure, stated because the two above do not fix the bound.**
///
/// **The break is declared when three consecutive keepalive requests have gone
/// unanswered *and one further interval has elapsed*** — so the declaration
/// lands **between 45 and 60 seconds** after a break that fell anywhere in an
/// interval. Without this rule, whether the elapsed time since the break is
/// 30-to-45 seconds or 45-to-60 turns on a timing rule nobody wrote down.
pub const ONE_FURTHER_INTERVAL_ELAPSES_BEFORE_THE_DECLARATION: bool = true;

/// The earliest a break can be declared after it fell — 45 seconds.
pub const EARLIEST_DECLARATION: Duration = Duration::from_secs(45);

/// The latest a break can be declared after it fell — 60 seconds.
///
/// **K6b: the observed value lags a break by this bound**, so a server can read
/// **connected** for up to one interval-and-count over a connection that is
/// already dead. **Nothing shortens that** — tightening the figures is what
/// K3's own constraints forbid — and **nothing anywhere reads the value as a
/// statement about a reading's freshness**: a reading's currency comes from its
/// own moment, never from the observed value standing beside it.
pub const LATEST_DECLARATION: Duration = Duration::from_secs(60);

/// **K2: two things report a break and neither requires the caller to have work
/// to do.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakRoute {
    /// **An announced ending.** The far end closing the connection, or its own
    /// service closing the socket as it shuts down, arrives on the transport as
    /// a close. Observed — a machine closing politely and a machine powered off
    /// from a provider's console were **both told to the caller in the same
    /// second**, keepalives making no difference, because *"A provider's stop is
    /// an orderly shutdown, not a power cut"* (Fact 57).
    ///
    /// **§16 item 27: which of the two routes a machine being *destroyed* takes
    /// has never been posed, and under the ruling of 2026-08-31 it is
    /// permanently unresolved.** The evidence behind this route is a provider
    /// *stop*; a destroy is a different act and nothing in this project poses
    /// it. **Nothing here leaves a space for the answer.**
    AnAnnouncedEnding,
    /// **A caller-side keepalive that goes unanswered.** This is what covers
    /// the case the announced ending does not, and **it is the case that
    /// matters: the caller's own line failing.** Observed — the same event was
    /// detected in **56 seconds** with keepalives on and **not detected at all
    /// for 21 minutes 45 seconds** with them off (Fact 57). The machine will
    /// not do it: `clientaliveinterval 0`, which is OpenSSH's shipped default.
    ACallerSideKeepaliveUnanswered,
}

impl BreakRoute {
    pub const ALL: [BreakRoute; 2] = [
        BreakRoute::AnAnnouncedEnding,
        BreakRoute::ACallerSideKeepaliveUnanswered,
    ];

    /// **§8.5's constraint, kept: neither route requires the caller to use the
    /// connection first.**
    pub fn requires_the_caller_to_use_the_connection_first(self) -> bool {
        false
    }
}

/// **K4: a keepalive is a request on the connection and never a ping.**
///
/// `Server protocol and transfers` C18 is exact — *"no health check in this
/// document pings"* — and Fact 57 is why: **one provider answers no ping at
/// all** while its connection works perfectly, so **any ping-based health check
/// would report it dead**. The keepalive here rides inside the connection
/// Smoozy holds and is answered by the same transport that carries the work.
pub const NOTHING_HERE_PINGS: bool = true;

/// Counts the unanswered keepalives on one connection and says when the break
/// is declared. **The whole of K3's rule, in one place**, so no caller composes
/// the bound out of the two figures and gets the other window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeepaliveCount {
    unanswered: u32,
    intervals_since_the_count_filled: u32,
}

impl KeepaliveCount {
    pub fn fresh() -> KeepaliveCount {
        KeepaliveCount::default()
    }

    /// One interval elapsed with the keepalive unanswered.
    pub fn went_unanswered(&mut self) {
        if self.unanswered < UNANSWERED_COUNT {
            self.unanswered += 1;
        } else {
            self.intervals_since_the_count_filled += 1;
        }
    }

    /// An answer arrived. **The count is consecutive**, so one answer resets
    /// it — which is what makes three the figure that absorbs a single lost
    /// keepalive on a busy link.
    pub fn was_answered(&mut self) {
        self.unanswered = 0;
        self.intervals_since_the_count_filled = 0;
    }

    /// **K3: three consecutive unanswered requests *and one further interval*.**
    pub fn a_break_is_declared(self) -> bool {
        self.unanswered >= UNANSWERED_COUNT && self.intervals_since_the_count_filled >= 1
    }
}

/// **K5: a break declared is a break, and the connection under it is torn down
/// rather than kept for later.**
///
/// Observed — a ten-minute silent gap left a connection **unrecoverable even
/// after the path was restored**, almost certainly because the provider's
/// address translation forgets faster than the protocol does, *"though the
/// actual timeout was not established"* (Fact 57). So the repair **always
/// builds a new connection** and never resurrects one, and **nothing anywhere
/// waits on an old connection coming back to life**.
pub fn a_declared_break_may_be_resurrected() -> bool {
    false
}

/// **K5a: the address the new connection is built to survives an hour of
/// complete silence**, which is the half K5 could not speak to.
///
/// **Observed** — a rental left running and touched by nothing at all for
/// **3623 seconds** answered on the same host and the same port on the first
/// packet, and it was the same machine: the marker written before the silence,
/// the boot identifier unchanged, and a process started before the silence
/// still alive with an unbroken heartbeat.
///
/// **The two observations are about different things and do not disagree.**
/// K5's ten minutes is about **one held connection** dying and not coming back;
/// this is about **the provider's address mapping** still routing a fresh
/// connection to the same machine.
///
/// **The bound is exactly one hour, on one provider, on one rental.** It sets
/// no upper limit, says nothing about a rental that stops and restarts, and
/// does not touch §16 item 5's remaining half — whether a seller later hands
/// the same address to a **different** machine. Label: **observed in a test**.
pub const THE_ADDRESS_SURVIVED_ONE_HOUR_OF_SILENCE: Duration = Duration::from_secs(3_623);

/// **§2.3: what is never used to learn whether the connection is alive.**
///
/// Each of these is a route a builder would otherwise reach for, and each is
/// refused with the clause that refuses it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeverReadAsABrokenConnection {
    /// **K10: no request is made to learn whether the connection is alive.**
    /// §8.5 forbids a mechanism that needs the caller to use the connection
    /// first, and a request made to find out is exactly that.
    ARequestMadeToFindOut,
    /// **K11: a stalled transfer channel is never read as a broken
    /// connection**, which is `Server protocol and transfers` G8's rule from
    /// the other side.
    AStalledTransferChannel,
    /// **K12: a supervisor that does not answer is never read as a broken
    /// connection.** The supervisor-unavailable condition's observed value is
    /// whether the supervisor answered a request on the command path and is
    /// nothing else, and *"the two are never inferred from one another"*. **The
    /// server connection stays up.**
    ASupervisorThatDoesNotAnswer,
    /// **K13: nothing about the connection is inferred from a machine being
    /// reachable by any other route.** Reachability is not the check; the
    /// installation identity is, and it is a check about Smoozy's own state
    /// rather than about the connection.
    ReachabilityByAnyOtherRoute,
}

impl NeverReadAsABrokenConnection {
    pub const ALL: [NeverReadAsABrokenConnection; 4] = [
        NeverReadAsABrokenConnection::ARequestMadeToFindOut,
        NeverReadAsABrokenConnection::AStalledTransferChannel,
        NeverReadAsABrokenConnection::ASupervisorThatDoesNotAnswer,
        NeverReadAsABrokenConnection::ReachabilityByAnyOtherRoute,
    ];
}

/// **§2.3, kept by one function**: nothing on that list ever declares a break.
pub fn declares_a_break(_never: NeverReadAsABrokenConnection) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_declaration_window_is_forty_five_to_sixty_seconds() {
        // K3's own arithmetic: three unanswered intervals, then one more.
        assert_eq!(KEEPALIVE_INTERVAL * UNANSWERED_COUNT, EARLIEST_DECLARATION);
        assert_eq!(
            KEEPALIVE_INTERVAL * (UNANSWERED_COUNT + 1),
            LATEST_DECLARATION
        );
    }

    #[test]
    fn three_unanswered_keepalives_alone_do_not_declare_a_break() {
        // Without K3's third figure the bound would be 30-to-45 seconds.
        let mut count = KeepaliveCount::fresh();
        for _ in 0..UNANSWERED_COUNT {
            count.went_unanswered();
        }
        assert!(!count.a_break_is_declared());
        count.went_unanswered();
        assert!(count.a_break_is_declared());
    }

    #[test]
    fn one_answer_resets_the_count_because_it_is_consecutive() {
        let mut count = KeepaliveCount::fresh();
        count.went_unanswered();
        count.went_unanswered();
        count.was_answered();
        count.went_unanswered();
        count.went_unanswered();
        count.went_unanswered();
        count.went_unanswered();
        assert!(count.a_break_is_declared());

        let mut absorbed = KeepaliveCount::fresh();
        absorbed.went_unanswered();
        absorbed.was_answered();
        assert!(!absorbed.a_break_is_declared());
    }

    #[test]
    fn neither_break_route_needs_the_caller_to_use_the_connection_first() {
        assert_eq!(BreakRoute::ALL.len(), 2);
        for route in BreakRoute::ALL {
            assert!(!route.requires_the_caller_to_use_the_connection_first());
        }
    }

    #[test]
    fn nothing_on_the_never_list_ever_declares_a_break() {
        assert_eq!(NeverReadAsABrokenConnection::ALL.len(), 4);
        for never in NeverReadAsABrokenConnection::ALL {
            assert!(!declares_a_break(never));
        }
    }

    #[test]
    fn a_declared_break_is_never_resurrected() {
        assert!(!a_declared_break_may_be_resurrected());
    }

    #[test]
    fn the_hour_of_silence_bound_is_the_observed_figure_and_not_a_round_one() {
        // Stated as what was watched — 3623 seconds — rather than as "an hour",
        // because the bound is one provider on one rental.
        assert_eq!(THE_ADDRESS_SURVIVED_ONE_HOUR_OF_SILENCE.as_secs(), 3_623);
    }
}

//! Where the interrupt does not take — §3.3 K8–K11: what *not taking* is,
//! the waiter's deadline, and what the user is told when it expires.
//!
//! **K8: what counts as *not taking* is defined, because nobody upstream
//! has.** The stop's caller waits on the stop command's own operation
//! identity — the identity `Server protocol and transfers` C21 gives it,
//! naming the target's — and *not taking* is **that identity having no
//! recorded outcome when the caller's deadline expires, with nothing
//! recorded saying the group has ended.** Both halves are needed: no
//! recorded outcome alone is a stop still landing (1 to 3 seconds on the
//! virtual machine, 1.9 to 14 seconds on the container, Fact 2), and a
//! group gone without an outcome is the supervisor's to record, not the
//! waiter's to conclude.
//!
//! **K9: the deadline is the waiter's own, 120 seconds, and it is an
//! assumption bounded from above by two sets of measurements that do not
//! agree with each other.** Fact 106: on a frame Blender estimated at ten
//! and a half hours, four interrupts came back at 2.7, 8.1, 13.4 and 29.9
//! seconds, the latency being the time left in the current sample batch —
//! 37.3 seconds on that scene. *Frames 15*: the longest recorded waits are
//! **65 to 73 seconds**, in all four versions, on one scene at 320 by 180.
//! **The two are not reconciled and this crate does not choose between
//! them** (§9 item 7, permanently unresolved on that half). Sixty is below
//! an observed value; one hundred and twenty clears the highest observation
//! with room, and the costs are not symmetric — a long deadline costs a
//! slower honest message, a short one a false *not stopped* on an ordinary
//! render. **A builder moves the constant on a measurement of their own and
//! never on either of these two.**
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Server protocol and transfers` C8, C16,
//! C17, C21. `Job files and outputs` C17.

use std::time::Duration;

/// **K9's value, and the two measurements it is bounded by.**
pub const THE_WAITERS_DEADLINE: Duration = Duration::from_secs(120);

/// The first set — Fact 106's four interrupts and the batch they were
/// measured against, in seconds.
pub const THE_FIRST_SET_FACT_106: ([f64; 4], f64) = ([2.666, 8.142, 13.437, 29.946], 37.3);

/// The second set — *Frames 15*'s longest recorded waits, in seconds, in
/// all four versions on one scene at 320 by 180.
pub const THE_SECOND_SET_FRAMES_15: (u64, u64) = (65, 73);

/// What the waiter holds at its deadline's expiry — C8's answer to the
/// stop command's own identity, and nothing the desktop read for itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheWaiterHolds {
    /// Whether the stop command's identity has a recorded outcome.
    pub the_stop_has_a_recorded_outcome: bool,
    /// Whether anything recorded says the group has ended — the
    /// supervisor's record, never a live validation the desktop took.
    pub the_records_say_the_group_has_ended: bool,
}

/// **K8, performed: *not taking* is both halves at once.**
pub fn the_interrupt_did_not_take(held: WhatTheWaiterHolds) -> bool {
    !held.the_stop_has_a_recorded_outcome && !held.the_records_say_the_group_has_ended
}

/// **K10: what the user is told when the deadline expires is the waiter's
/// truth and nothing stronger** — the stop was sent, and the render had
/// not ended when Smoozy stopped waiting. **K11: Smoozy cannot tell a stop
/// that failed from a render that is slow to honour it**, and the
/// classification refuses to manufacture the difference. The words are
/// Design's; what they may claim is bounded here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheUserIsTold {
    /// The stop was sent, and the render had not ended when Smoozy stopped
    /// waiting. **Never** that the render could not be stopped.
    TheStopWasSentAndTheRenderHadNotEndedWhenSmoozyStoppedWaiting,
}

/// **K10's fixed rule**: no kill is reached for, nothing re-signals without
/// a fresh command, the entry goes on showing the records' state, and a
/// render that honours the interrupt after the deadline shows *stopped*
/// when its outcome lands, the expiry having changed nothing (`Server
/// protocol and transfers` C17). A fresh stop command is a new command
/// with a new identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatExpiryDoes {
    pub reaches_for_the_kill: bool,
    pub re_signals_without_a_fresh_command: bool,
    pub changes_any_durable_state: bool,
    pub a_fresh_stop_is_a_new_command_with_a_new_identity: bool,
}

pub const WHAT_EXPIRY_DOES: WhatExpiryDoes = WhatExpiryDoes {
    reaches_for_the_kill: false,
    re_signals_without_a_fresh_command: false,
    changes_any_durable_state: false,
    a_fresh_stop_is_a_new_command_with_a_new_identity: true,
};

/// **K10 and K11, performed**: the waiter's truth, on expiry with the
/// interrupt not taken; nothing at all where it took.
pub fn what_the_user_is_told(held: WhatTheWaiterHolds) -> Option<WhatTheUserIsTold> {
    the_interrupt_did_not_take(held)
        .then_some(WhatTheUserIsTold::TheStopWasSentAndTheRenderHadNotEndedWhenSmoozyStoppedWaiting)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_deadline_is_one_hundred_and_twenty_seconds_and_clears_both_measurements_with_room() {
        assert_eq!(THE_WAITERS_DEADLINE, Duration::from_secs(120));
        let (interrupts, batch) = THE_FIRST_SET_FACT_106;
        for wait in interrupts {
            assert!(wait < batch);
        }
        assert!(batch < THE_WAITERS_DEADLINE.as_secs_f64());
        let (_, longest) = THE_SECOND_SET_FRAMES_15;
        assert!(longest < THE_WAITERS_DEADLINE.as_secs());
        // Sixty was below an observed value — which is the reason the
        // constant is not sixty.
        assert!(longest > 60);
    }

    #[test]
    fn not_taking_needs_both_halves() {
        assert!(the_interrupt_did_not_take(WhatTheWaiterHolds {
            the_stop_has_a_recorded_outcome: false,
            the_records_say_the_group_has_ended: false,
        }));
        // A stop still landing.
        assert!(!the_interrupt_did_not_take(WhatTheWaiterHolds {
            the_stop_has_a_recorded_outcome: true,
            the_records_say_the_group_has_ended: false,
        }));
        // A group gone, recorded by the supervisor.
        assert!(!the_interrupt_did_not_take(WhatTheWaiterHolds {
            the_stop_has_a_recorded_outcome: false,
            the_records_say_the_group_has_ended: true,
        }));
    }

    #[test]
    fn the_user_meets_the_waiters_truth_and_nothing_stronger() {
        let not_taken = WhatTheWaiterHolds {
            the_stop_has_a_recorded_outcome: false,
            the_records_say_the_group_has_ended: false,
        };
        assert_eq!(
            what_the_user_is_told(not_taken),
            Some(WhatTheUserIsTold::TheStopWasSentAndTheRenderHadNotEndedWhenSmoozyStoppedWaiting)
        );
        let taken = WhatTheWaiterHolds {
            the_stop_has_a_recorded_outcome: true,
            the_records_say_the_group_has_ended: true,
        };
        assert_eq!(what_the_user_is_told(taken), None);
        const _: () = assert!(!WHAT_EXPIRY_DOES.reaches_for_the_kill);
        const _: () = assert!(!WHAT_EXPIRY_DOES.re_signals_without_a_fresh_command);
        const _: () = assert!(!WHAT_EXPIRY_DOES.changes_any_durable_state);
        const _: () = assert!(WHAT_EXPIRY_DOES.a_fresh_stop_is_a_new_command_with_a_new_identity);
    }
}

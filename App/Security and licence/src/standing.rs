//! The store's standing — `Security and licence` §3.3 S32 to S36, answering
//! Row 316.
//!
//! S25's conservative rule reads across runs — *"a probe that has **never
//! succeeded in any run**"* — while the probe result itself is run-held and is
//! re-taken every run. What makes that rule executable is one durable value in
//! the local record with **three readings and one writer**. S33: it is a note
//! Smoozy wrote about its own history and not a cached store reading, so
//! nothing ever reads it in place of taking the probe.

use crate::secret_store::ProbeOutcome;

/// The three readings of §3.3 S32's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreStanding {
    /// The value's absence at first launch: no probe has been taken on this
    /// installation.
    NeverProbed,
    /// The first probe of this installation failed: a computer with nowhere to
    /// remember, until something proves otherwise.
    NeverAnswered,
    /// Any probe succeeded, ever. The store exists here, and every later
    /// failure is an ordinary failure under §3.0 rule 4. **It never returns to
    /// `NeverAnswered`.**
    HasAnswered,
}

impl Default for StoreStanding {
    /// The absence of the value is the first reading, which is what S32's
    /// table says in its own words. A record holding no standing is a fresh
    /// installation and not a fault.
    fn default() -> Self {
        StoreStanding::NeverProbed
    }
}

/// What a probe's outcome does to the standing, and what Smoozy says because
/// of it. The one writer of the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StandingWrite {
    /// What to write durably. Where it equals what was already there, nothing
    /// is written.
    pub standing: StoreStanding,
    /// S34: **the one-sentence notice is shown at the moment the standing goes
    /// from *never probed* to *never answered*, once for the life of the
    /// installation** — not once per run, which is what the promise's own
    /// subordinate clause exists to prevent.
    ///
    /// It is Smoozy's own notice rather than a warning the user suppressed, so
    /// it takes no row in the suppression table.
    pub say_it_once: bool,
}

impl StoreStanding {
    /// Applies one probe outcome. This is the whole of the value's writing
    /// rule, in one place, so no caller can invent a fourth transition.
    pub fn after(self, outcome: ProbeOutcome) -> StandingWrite {
        match (self, outcome) {
            // Any probe succeeding, ever, moves the standing here for good.
            (_, ProbeOutcome::RoundTripWorked) => StandingWrite {
                standing: StoreStanding::HasAnswered,
                say_it_once: false,
            },
            // The first probe of this installation failing.
            (StoreStanding::NeverProbed, ProbeOutcome::RoundTripFailed) => StandingWrite {
                standing: StoreStanding::NeverAnswered,
                say_it_once: true,
            },
            // A later failure on a computer already known to have nowhere to
            // remember. The notice was said once and is not said again.
            (StoreStanding::NeverAnswered, ProbeOutcome::RoundTripFailed) => StandingWrite {
                standing: StoreStanding::NeverAnswered,
                say_it_once: false,
            },
            // S25 and S35: a store that answered once is **not** re-classified
            // as nowhere by any number of failures. The failure is reported as
            // an ordinary failure under §3.0 rule 4 and the standing holds.
            //
            // S35 names the cost rather than repairing it: a user who had a
            // keyring and removes it leaves the standing here for ever, so the
            // once-only notice never appears for them. That is rule 4 working
            // as written, and the alternative reopens the first of the two
            // opposite lies §3.3 exists to prevent.
            (StoreStanding::HasAnswered, ProbeOutcome::RoundTripFailed) => StandingWrite {
                standing: StoreStanding::HasAnswered,
                say_it_once: false,
            },
        }
    }

    /// S36. On a computer with nowhere to remember, *Remember it* would behave
    /// exactly as *Do not remember it*, and offering it is **the pretence the
    /// promise forbids in as many words**. So the offer is composed after the
    /// probe, from the standing: two choices where it reads *never answered*,
    /// three otherwise.
    ///
    /// The middle choice is unaffected in every case, being process memory
    /// (§2 item 3b) and needing no store at all.
    pub fn machine_password_choices(self) -> &'static [MachinePasswordChoice] {
        match self {
            StoreStanding::NeverAnswered => &[
                MachinePasswordChoice::DoNotRememberIt,
                MachinePasswordChoice::WhileThisConnectionLasts,
            ],
            StoreStanding::NeverProbed | StoreStanding::HasAnswered => &[
                MachinePasswordChoice::DoNotRememberIt,
                MachinePasswordChoice::WhileThisConnectionLasts,
                MachinePasswordChoice::RememberIt,
            ],
        }
    }
}

/// The three choices `Whole app`'s Security concern offers for a machine
/// password, which are §2 items 3c, 3b and 3 in that order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachinePasswordChoice {
    /// §2 item 3c — process memory, for the one action.
    DoNotRememberIt,
    /// §2 item 3b — process memory, keyed by the connection.
    WhileThisConnectionLasts,
    /// §2 item 3 — the secret store, across restarts.
    RememberIt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_absent_value_reads_as_never_probed() {
        assert_eq!(StoreStanding::default(), StoreStanding::NeverProbed);
    }

    #[test]
    fn the_notice_is_said_at_the_first_failure_and_only_there() {
        let first = StoreStanding::NeverProbed.after(ProbeOutcome::RoundTripFailed);
        assert_eq!(first.standing, StoreStanding::NeverAnswered);
        assert!(first.say_it_once);

        let again = first.standing.after(ProbeOutcome::RoundTripFailed);
        assert_eq!(again.standing, StoreStanding::NeverAnswered);
        assert!(!again.say_it_once);
    }

    #[test]
    fn a_probe_that_works_moves_the_standing_and_says_nothing() {
        let written = StoreStanding::NeverAnswered.after(ProbeOutcome::RoundTripWorked);
        assert_eq!(written.standing, StoreStanding::HasAnswered);
        assert!(!written.say_it_once);
    }

    #[test]
    fn a_store_that_answered_once_never_returns_to_never_answered() {
        let mut standing = StoreStanding::HasAnswered;
        for _ in 0..20 {
            standing = standing.after(ProbeOutcome::RoundTripFailed).standing;
        }
        assert_eq!(standing, StoreStanding::HasAnswered);
    }

    #[test]
    fn a_first_probe_that_works_never_says_the_notice() {
        let written = StoreStanding::NeverProbed.after(ProbeOutcome::RoundTripWorked);
        assert_eq!(written.standing, StoreStanding::HasAnswered);
        assert!(!written.say_it_once);
    }

    #[test]
    fn the_offer_is_two_choices_where_there_is_nowhere_to_remember() {
        let offer = StoreStanding::NeverAnswered.machine_password_choices();
        assert_eq!(offer.len(), 2);
        assert!(!offer.contains(&MachinePasswordChoice::RememberIt));
        assert!(offer.contains(&MachinePasswordChoice::WhileThisConnectionLasts));
    }

    #[test]
    fn the_offer_is_three_choices_everywhere_else() {
        for standing in [StoreStanding::NeverProbed, StoreStanding::HasAnswered] {
            assert_eq!(standing.machine_password_choices().len(), 3);
        }
    }
}

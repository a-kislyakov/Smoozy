//! The three observed values, and what Servers supplies — `Servers` §2.2.
//!
//! **K6: the connection's observed value has exactly three values and the set
//! is closed: connected, reconnecting, disconnected.** They are what `State,
//! identity and recovery` §8.3's table already names against this document, and
//! **this document does not add a fourth**.
//!
//! **K7: Servers supplies that value and the transitions, and presents
//! neither.** The condition record is §8.2's derivation; it holds over
//! *disconnected* and *reconnecting* and is **withdrawn on *connected***.
//! `Desktop app and delivery` T13 presents it and raises the could-not-restore
//! notification **from that record and not from a request failing**, which this
//! document keeps by supplying a value that moves on the transport's own report
//! rather than on any request's outcome.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` T13.

/// **K6's closed set of three.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservedValue {
    /// A period stands and its primary connection is live.
    Connected,
    /// A period stands, its primary connection is not live, and **an attempt is
    /// in flight** — including while a connection is being opened or closed by
    /// hand.
    Reconnecting,
    /// No period stands; **or** a period stands with **no run of attempts in
    /// flight**, which is the stretch between one run and the next.
    Disconnected,
}

impl ObservedValue {
    pub const ALL: [ObservedValue; 3] = [
        ObservedValue::Connected,
        ObservedValue::Reconnecting,
        ObservedValue::Disconnected,
    ];
}

/// The two facts every value and every presentation below is composed from.
///
/// **W19's table is keyed on these two and never on the three-value
/// shorthand** — K6's three values are compositions of them, and a warning is
/// not cut on a composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatIsTrueOfTheServer {
    /// **K8a's boolean**: whether a connected period stands. One per server
    /// row, run-held, taken from the period itself and derived from nothing.
    pub a_period_stands: bool,
    /// Whether the **primary** connection is live. **S14 condition 6**: the
    /// condition is derived from the primary alone.
    pub the_primary_connection_is_live: bool,
    /// **A run of attempts is in flight.** *A run is in flight for its whole
    /// length, its in-run pauses included* — which is K6a's correction of one
    /// word, the 15-second pause inside a run being **not** the stretch between
    /// runs.
    pub a_run_of_attempts_is_in_flight: bool,
    /// The period's *restarting, caused by Smoozy* mark, where `Server Setup`
    /// has set one.
    pub the_restart_mark_stands: bool,
}

/// K6's table, performed.
///
/// **K6a records that this table's second value has been read wrongly once and
/// promised wrongly once, and both are recorded because the same sentence
/// caused both.** The *disconnected* row used to read *"no attempt in flight"*
/// and then gloss itself *"the stretch between one run of attempts and the
/// next"* — **and those are not the same stretch**. N7 had already ruled it the
/// other way in plain words, so **a run is in flight for its whole length**.
pub fn observed_value(of: WhatIsTrueOfTheServer) -> ObservedValue {
    if !of.a_period_stands {
        return ObservedValue::Disconnected;
    }
    if of.the_primary_connection_is_live {
        return ObservedValue::Connected;
    }
    if of.a_run_of_attempts_is_in_flight {
        return ObservedValue::Reconnecting;
    }
    ObservedValue::Disconnected
}

/// **K8 and K8a: two presentations composed beside the observed value, and
/// neither is a fourth value.**
///
/// **K8a's argument for the fifth is K8's exactly**: K8 already composes a
/// presentation out of the period's restart mark standing beside the observed
/// value; **a period standing beside the observed value is the same shape**,
/// and it is what was missing. K6's set is still three.
///
/// **What each presentation is called on screen is Design's.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presentation {
    Connected,
    /// **K8: composed from the period's restart mark standing while the
    /// observed value is *reconnecting* or *disconnected*.** `Server Setup` T6
    /// supplies it *"as a state, not a wording"*.
    Restarting,
    /// **K8a: a period stands while the observed value is *reconnecting* or
    /// *disconnected*, and no restart mark stands.** It takes the **connected**
    /// menu whose first slot reads *disconnect* — which is S10 item 1's
    /// promise, available for as long as a period stands.
    Repairing,
    /// **A server whose period does not stand is repaired by nothing** (N13)
    /// and takes the disconnected menu.
    Disconnected,
}

/// **K8a: the precedence between the two presentations is stated so nobody
/// derives it.** Where the restart mark stands, K8's *restarting* wins;
/// *repairing* is what a standing period reads as when no mark stands.
pub fn presentation(of: WhatIsTrueOfTheServer) -> Presentation {
    match observed_value(of) {
        ObservedValue::Connected => Presentation::Connected,
        ObservedValue::Reconnecting | ObservedValue::Disconnected => {
            if !of.a_period_stands {
                Presentation::Disconnected
            } else if of.the_restart_mark_stands {
                Presentation::Restarting
            } else {
                Presentation::Repairing
            }
        }
    }
}

/// Which menu the row is given. **K8a: the two situations K6's *disconnected*
/// covers take opposite first slots**, which is why the boolean exists at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    /// First slot reads *disconnect*.
    TheConnectedMenu,
    TheDisconnectedMenu,
}

pub fn menu(of: WhatIsTrueOfTheServer) -> Menu {
    if of.a_period_stands {
        Menu::TheConnectedMenu
    } else {
        Menu::TheDisconnectedMenu
    }
}

/// **K7: the condition record holds over *disconnected* and *reconnecting* and
/// is withdrawn on *connected*.**
///
/// **M8 is the one exception and it is the restart mark's**: while the mark
/// stands, **no connection condition record is raised at all**. The observed
/// value still moves — Smoozy knows perfectly well the connection is down,
/// because it took it down — but the record `Desktop app and delivery` T13
/// raises the could-not-restore notification from is not raised. That is the
/// suppression `Server Setup` requires, **expressed in the one mechanism that
/// presents it** rather than as a second suppression rule sitting beside the
/// record.
pub fn a_condition_record_stands(of: WhatIsTrueOfTheServer, a_run_has_failed: bool) -> bool {
    if of.the_restart_mark_stands {
        return false;
    }
    if of.the_primary_connection_is_live && of.a_period_stands {
        return false;
    }
    // N5: the record is raised at the run's failure and not at the break
    // declaration.
    a_run_has_failed
}

/// **K9: *connected* and *needing a new key* are two independent readings of one
/// server and never one state machine.**
///
/// The first is this section's; the second is computed at each list draw from a
/// local read ([`crate::key_moments`]). **Both can be true at once**, which is
/// what Row 34 requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwoIndependentReadings {
    pub observed: ObservedValue,
    pub needing_a_new_key: bool,
}

/// **N7b: which vocabulary these three words are in is fixed for the whole of
/// this document.**
///
/// Wherever the plan says a server *reads* or *is left* **connected**,
/// **reconnecting** or **disconnected**, **it names K6's observed value and
/// never a word on a screen.** K7 already says this document presents none of
/// them; this says which vocabulary the sentences are written in, so that no
/// builder reads one of them as the label a row carries.
///
/// **W19's table was in that list until 2026-08-22 and is not any more**: it is
/// keyed on [`WhatIsTrueOfTheServer`]'s two facts and uses none of the three
/// words.
pub const THESE_THREE_WORDS_ARE_NEVER_A_LABEL: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    fn nothing() -> WhatIsTrueOfTheServer {
        WhatIsTrueOfTheServer {
            a_period_stands: false,
            the_primary_connection_is_live: false,
            a_run_of_attempts_is_in_flight: false,
            the_restart_mark_stands: false,
        }
    }

    fn connected() -> WhatIsTrueOfTheServer {
        WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: true,
            ..nothing()
        }
    }

    fn repairing_mid_run() -> WhatIsTrueOfTheServer {
        WhatIsTrueOfTheServer {
            a_period_stands: true,
            a_run_of_attempts_is_in_flight: true,
            ..nothing()
        }
    }

    fn between_runs() -> WhatIsTrueOfTheServer {
        WhatIsTrueOfTheServer {
            a_period_stands: true,
            ..nothing()
        }
    }

    #[test]
    fn the_set_is_three_and_stays_three() {
        assert_eq!(ObservedValue::ALL.len(), 3);
    }

    #[test]
    fn a_run_is_in_flight_for_its_whole_length_including_its_own_pauses() {
        // K6a's one-word correction. An in-run pause is not the stretch
        // between runs, so it reads reconnecting rather than disconnected.
        assert_eq!(observed_value(repairing_mid_run()), ObservedValue::Reconnecting);
        assert_eq!(observed_value(between_runs()), ObservedValue::Disconnected);
    }

    #[test]
    fn no_period_reads_disconnected_and_a_live_primary_reads_connected() {
        assert_eq!(observed_value(nothing()), ObservedValue::Disconnected);
        assert_eq!(observed_value(connected()), ObservedValue::Connected);
    }

    #[test]
    fn a_repairing_server_and_a_disconnected_one_read_alike_and_are_told_apart() {
        // Both read *disconnected* between runs; the boolean is what separates
        // them, and it decides which menu the row is given.
        assert_eq!(observed_value(between_runs()), observed_value(nothing()));
        assert_eq!(presentation(between_runs()), Presentation::Repairing);
        assert_eq!(presentation(nothing()), Presentation::Disconnected);
        assert_eq!(menu(between_runs()), Menu::TheConnectedMenu);
        assert_eq!(menu(nothing()), Menu::TheDisconnectedMenu);
    }

    #[test]
    fn the_restart_mark_wins_over_repairing() {
        let marked = WhatIsTrueOfTheServer {
            the_restart_mark_stands: true,
            ..repairing_mid_run()
        };
        assert_eq!(presentation(marked), Presentation::Restarting);
    }

    #[test]
    fn no_condition_record_stands_while_the_restart_mark_does() {
        // M8, and it is where T13's could-not-restore suppression comes from.
        let marked = WhatIsTrueOfTheServer {
            the_restart_mark_stands: true,
            ..between_runs()
        };
        assert!(!a_condition_record_stands(marked, true));
    }

    #[test]
    fn the_record_is_raised_at_the_runs_failure_and_not_at_the_break() {
        // N5: not at the break declaration.
        assert!(!a_condition_record_stands(repairing_mid_run(), false));
        assert!(a_condition_record_stands(between_runs(), true));
    }

    #[test]
    fn the_record_is_withdrawn_on_connected() {
        assert!(!a_condition_record_stands(connected(), true));
    }

    #[test]
    fn connected_and_needing_a_new_key_can_both_be_true() {
        // Row 34's own requirement.
        let both = TwoIndependentReadings {
            observed: observed_value(connected()),
            needing_a_new_key: true,
        };
        assert_eq!(both.observed, ObservedValue::Connected);
        assert!(both.needing_a_new_key);
    }
}

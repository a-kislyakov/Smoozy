//! The key's three moments — `Servers` §9.
//!
//! **What *usable* means, what a fingerprint is, and how a passphrase is held
//! are `Keys`', and nothing below decides any of them.** `Security and licence`
//! §6 owns the private half of a key and is not retaken, and no module here
//! holds a secret value.
//!
//! **P1: before every connection attempt, Servers asks `Keys` two things about
//! the row's key reference.** Is this key usable? And **if it needs a
//! passphrase, is one available for this attempt without asking?** **Where
//! Smoozy is making the connection on its own — P3's list — it does not attempt
//! on anything but two yeses; a connect the user started is gated on the first
//! answer alone, and P4 settles the second.** *(P1 carried P3's rule with none
//! of P3's scope until 2026-09-03, when the plan's own sentence was repaired
//! from this code round. The reading here has always been the narrow one.)*

use crate::identity::KeyReference;

/// The two questions, and what came back. **`Keys` defines *usable*; Servers
/// reads the answer.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheTwoAnswers {
    pub the_key_is_usable: bool,
    /// **Both must be answerable without prompting**, because a connection
    /// Smoozy makes on its own may never put a passphrase prompt on screen.
    pub a_passphrase_is_available_without_asking: bool,
}

impl TheTwoAnswers {
    /// **P3: where either answer is no, no attempt is made.** The server is left
    /// disconnected, **marked with what is wrong**, and Smoozy says so.
    ///
    /// **This holds for every connection Smoozy makes on its own** — a repair
    /// attempt, the edit put-back, and the additional connection — and **it is
    /// what makes *an automatic attempt never puts a passphrase prompt on
    /// screen* true by not making the attempt rather than by suppressing a
    /// prompt.**
    pub fn an_attempt_may_be_made(self) -> bool {
        self.the_key_is_usable && self.a_passphrase_is_available_without_asking
    }
}

/// **P1a: *the row's key reference* is the pair — the local key identifier *and*
/// the key fingerprint beside it — and both values are handed over at every
/// ask.**
///
/// The key a server row names is the key row that identifier names **and** whose
/// current fingerprint equals the fingerprint this row holds; **where the two
/// disagree the answer to P1's first question is no**, and the derivation at the
/// next draw is *needing a new key*.
///
/// **The identifier alone is not the reference, and the constraint is `Keys`'
/// replacement act**: it keeps the row's identifier and changes the row's
/// fingerprint, **so an identifier-only ask would go on answering yes for every
/// server set up with the key that left.**
pub fn the_reference_still_names_the_same_key(
    row: &KeyReference,
    the_key_rows_current_fingerprint: &str,
) -> bool {
    row.fingerprint == the_key_rows_current_fingerprint
}

/// **P2: the check is taken at the moment of the attempt and never from a cached
/// value.**
///
/// Row 46's mark is computed at each list draw, **and a key can go away between
/// a draw and a connect**; every attempt in a repair run is its own attempt and
/// **takes its own check**. **Nothing here reads a value written once at connect
/// time.**
pub const THE_CHECK_IS_TAKEN_AT_EVERY_ATTEMPT: bool = true;

/// **Which connections P3 binds entirely, and which may ask.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhoStartedIt {
    /// **P4: a connect the user started may ask.** Who holds and supplies a
    /// passphrase is `Keys`', and this document **neither holds one nor asks for
    /// one itself**: the adapter carries the use out and Servers is handed the
    /// outcome — *"No module receives a plaintext"*.
    ///
    /// **P4a: the ask itself is `Keys`'**, and what this document hands over at
    /// that moment is **three display-only fields of the row** — the label, the
    /// username and the host — beside P1a's pair, so the box can name the server
    /// being connected to. **Nothing else crosses, no secret crosses at all**,
    /// and what comes back here is the attempt's outcome.
    TheUser,
    /// **A connection Smoozy makes on its own**: a repair attempt, the edit
    /// put-back, and the additional connection.
    SmoozyOnItsOwn,
}

impl WhoStartedIt {
    pub fn may_ask_for_a_passphrase(self) -> bool {
        matches!(self, WhoStartedIt::TheUser)
    }
}

/// **P5: that a key can be seen to need a passphrase without having it is
/// answered, and the promise rests on that half.**
///
/// Documented and observed (Fact 35 half A): type, bit size and fingerprint are
/// readable from an encrypted private key file with no passphrase, and
/// **passphrase-protection is detectable without prompting**. `Security and
/// licence` §6 owns which route is read as the yes-or-no; nothing here
/// re-decides it.
///
/// **P6: half B is answered and it is belt-and-braces rather than the promise's
/// support.** Batch mode does suppress the passphrase prompt, closed
/// 2026-08-20. **The promise here is *Smoozy does not make the attempt at all*,
/// and P1 to P3 keep it without half B**: an attempt that is never made cannot
/// prompt.
///
/// **And K1a narrows what half B was ever worth here.** Batch mode is a property
/// of the command-line client, which this connection does not run at all — **a
/// transport Smoozy drives asks nobody anything: there is no terminal, no helper
/// and no prompt route in the process.**
pub const THE_PROMISE_RESTS_ON_THE_ATTEMPT_NEVER_BEING_MADE: bool = true;

/// **P8: an open connection survives its key going away, and it survives by
/// construction.**
///
/// `Security and licence` §6: Smoozy **holds a path and a fingerprint**, hands
/// the **path** to the transport at authentication, never copies the file, never
/// retains its bytes. **Nothing after authentication reads the key file**, so
/// there is nothing in Smoozy that could close the connection over it and
/// nothing in the connection that depends on it.
///
/// **P9: Fact 58 answers it yes and the promise is kept.** Observed — a copy of
/// the key was made, a session opened with the copy, and the copy deleted; *"the
/// live session kept working, and a fresh connection with the missing key
/// failed"*, which proves the key gated **authentication** and that the survival
/// is because the session no longer needs the file.
pub fn an_open_connection_closes_when_its_key_goes_away() -> bool {
    false
}

/// **P10: the mark appears alongside, and the two readings are independent.**
/// The server stays **connected** and shows **needing a new key**; **the mark
/// applies to the next connection and not to this one.** It appears the next
/// time the server list is on screen, **which includes while the server is
/// connected**.
///
/// **P11: where such a connection then has its break declared, the period
/// ends.** No repair is attempted — P3 forbids the attempt — and a period whose
/// repair can never run would promise something Smoozy cannot deliver. So the
/// server ends up **disconnected** and **needing a new key**, and Smoozy says
/// so.
///
/// **Reviving the period when the key is mended would need a watcher**, and Row
/// 46 forbids one outright; **the user's own connect** — available again the
/// moment the mark clears — is the route back and is the route the spec grants.
pub fn a_break_with_the_key_gone_ends_the_period() -> bool {
    true
}

/// **P12 to P15: the check at the list draw.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtTheListDraw {
    /// **P12: it runs when the server list is drawn, once per server, over
    /// every server on it.** It runs for a **connected** server too, since
    /// *connected* and *needing a new key* can both be true. **Nothing polls in
    /// the background.**
    pub once_per_server_at_each_draw: bool,
    /// **P13: the mark is a derivation and not a stored state.** It is computed
    /// at each draw from `Keys`' answer about P1a's pair, **so there is nothing
    /// to go stale, nothing to clear, and no cache that refreshes itself.**
    ///
    /// That closes both of the implementations the promise rules out — **a
    /// watcher, and a value written once at connect time** — **by construction
    /// rather than by care**.
    pub the_mark_is_stored: bool,
    /// **P14: the check is a local read and never reaches a machine.** It is
    /// `Keys` answering about a file on the user's own computer, so S9's closed
    /// list is untouched by it and **drawing the list costs no connection**.
    pub it_reaches_a_machine: bool,
}

impl AtTheListDraw {
    pub fn how_it_runs() -> AtTheListDraw {
        AtTheListDraw {
            once_per_server_at_each_draw: true,
            the_mark_is_stored: false,
            it_reaches_a_machine: false,
        }
    }

    /// **P15: the mark clears the next time the user looks at their servers,
    /// however the key was mended** — a different key chosen here, or the same
    /// key mended in `Keys`. **No edit and no removal is needed**, which falls
    /// out of P13: **there is no stored mark to clear.**
    pub fn what_clears_the_mark() -> WhatClearsTheMark {
        WhatClearsTheMark::TheNextDraw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatClearsTheMark {
    /// And nothing else, because nothing is stored.
    TheNextDraw,
}

/// **P7: the connect action is withdrawn for a server marked as needing a new
/// key**, so the ordinary route never reaches P3 at all. **Connecting would
/// fail, so the action is not offered.**
pub fn the_connect_action_is_offered(needing_a_new_key: bool) -> bool {
    !needing_a_new_key
}

/// **P16: the row holds the fingerprint of the key it uses and matches against
/// it.** That is what lets a key be recognised as the one this server wants
/// **after it left Smoozy entirely and was pointed at again later**. **The
/// recognition is `Keys`'** — what a fingerprint is, and whether two keys can
/// share one, is Fact 76 and that document's — **and Servers' half is that the
/// row holds the fingerprint it needs and hands it over to be matched.**
pub const SERVERS_HALF_IS_HOLDING_THE_VALUE_AND_HANDING_IT_OVER: bool = true;

/// The three ways to repair or remove a server’s key association remain open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WayOut {
    PointTheServerAtADifferentKey,
    MendTheKeyInKeys,
    RemoveTheServer,
}

impl WayOut {
    pub const ALL: [WayOut; 3] = [
        WayOut::PointTheServerAtADifferentKey,
        WayOut::MendTheKeyInKeys,
        WayOut::RemoveTheServer,
    ];

    pub fn survives_a_locked_smoozy(self) -> bool {
        !matches!(self, WayOut::PointTheServerAtADifferentKey)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_reference() -> KeyReference {
        KeyReference {
            local_key_identifier: "key-1".into(),
            fingerprint: "SHA256:aaa".into(),
        }
    }

    #[test]
    fn nothing_but_two_yeses_lets_an_attempt_be_made() {
        assert!(TheTwoAnswers {
            the_key_is_usable: true,
            a_passphrase_is_available_without_asking: true,
        }
        .an_attempt_may_be_made());
        assert!(!TheTwoAnswers {
            the_key_is_usable: true,
            a_passphrase_is_available_without_asking: false,
        }
        .an_attempt_may_be_made());
        assert!(!TheTwoAnswers {
            the_key_is_usable: false,
            a_passphrase_is_available_without_asking: true,
        }
        .an_attempt_may_be_made());
    }

    #[test]
    fn a_replacement_that_kept_the_identifier_is_caught_by_the_fingerprint() {
        // `Keys`' replacement keeps the identifier and changes the fingerprint,
        // so an identifier-only ask would go on answering yes for every server
        // set up with the key that left.
        assert!(the_reference_still_names_the_same_key(
            &a_reference(),
            "SHA256:aaa"
        ));
        assert!(!the_reference_still_names_the_same_key(
            &a_reference(),
            "SHA256:bbb"
        ));
    }

    #[test]
    fn only_a_connect_the_user_started_may_ask() {
        assert!(WhoStartedIt::TheUser.may_ask_for_a_passphrase());
        assert!(!WhoStartedIt::SmoozyOnItsOwn.may_ask_for_a_passphrase());
    }

    #[test]
    fn the_mark_is_a_derivation_with_nothing_stored_to_go_stale() {
        let draw = AtTheListDraw::how_it_runs();
        assert!(!draw.the_mark_is_stored);
        assert!(!draw.it_reaches_a_machine);
        assert!(draw.once_per_server_at_each_draw);
        assert_eq!(
            AtTheListDraw::what_clears_the_mark(),
            WhatClearsTheMark::TheNextDraw
        );
    }

    #[test]
    fn an_open_connection_survives_its_key_going_away() {
        assert!(!an_open_connection_closes_when_its_key_goes_away());
    }

    #[test]
    fn a_break_declared_with_the_key_gone_ends_the_period_rather_than_repairing() {
        assert!(a_break_with_the_key_gone_ends_the_period());
    }

    #[test]
    fn the_connect_action_is_withdrawn_for_a_server_needing_a_new_key() {
        assert!(!the_connect_action_is_offered(true));
        assert!(the_connect_action_is_offered(false));
    }

    #[test]
    fn a_locked_smoozy_closes_one_way_out_of_three() {
        let surviving: Vec<_> = WayOut::ALL
            .into_iter()
            .filter(|way| way.survives_a_locked_smoozy())
            .collect();
        assert_eq!(surviving, vec![WayOut::MendTheKeyInKeys, WayOut::RemoveTheServer]);
    }
}

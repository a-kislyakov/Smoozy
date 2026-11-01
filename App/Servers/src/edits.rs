//! The edit put-back and what an edit throws away — `Servers` §5 and §6.4.
//!
//! **E1: saving an edit to a connected server's host, port, username or key
//! ends the period.** The spec's own verb is the reason — the save
//! *"disconnects a live server so the change can be applied"* — and a
//! disconnect is how a period ends. **The user's act of saving is what ends it,
//! exactly as their disconnect would.**
//!
//! **E2: the put-back is O1 performed by Smoozy as part of that same act.**
//! Three things follow and each is a promise **kept by construction rather than
//! by an exception**: a failed put-back is left alone, there being no period, so
//! N13 applies and no repair runs; the replacement list stays exactly two; and
//! editing a disconnected server leaves it disconnected.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C14.
//! `Job files and outputs` L1.

use crate::period::{OpeningAct, PeriodEnding};

/// **D12: the rules are cut on the field that changed, not on *the connection
/// details changed*. An answer that treats the edit as one event gets three of
/// the four wrong.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatChanged {
    /// **D13: host and port together name what Smoozy is reaching**, so
    /// changing either points the entry at a different machine. The hardware
    /// read from the old one is the wrong answer, the event cursor was taken
    /// against a journal on the old one, and the confirmed installation
    /// identity belonged to it, and **the recorded machine root was chosen on
    /// it** (`Job files and outputs` L1). **All four go in the same write.**
    HostOrPort,
    /// **D14: changing the username clears the remembered machine password
    /// without asking, because a password belongs to a login and the login has
    /// changed.** Label: **Creator-stated** (2026-08-10).
    Username,
    /// **D15: changing the key clears nothing.** The row's key reference and
    /// fingerprint change, and that is all. **In particular it does not touch
    /// the remembered host key**, since a key change does not change which
    /// machine is reached.
    TheKey,
    /// **E3: a renaming changes no connection detail, so it ends no period and
    /// opens nothing. Neither does a colour change.**
    TheLabelOrTheColour,
    /// **The server's chosen fallback folder** (D3's row, added 2026-10-04 with
    /// `Job files and outputs` P23). **E3 as widened that day: changing it ends
    /// no period and carries no warning** — it is the user's own instruction
    /// about where a job that cannot use its file's folder goes, and no
    /// connection is made on it.
    TheFallbackFolder,
}

/// D12's table, as four fates rather than one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheEditThrowsAway {
    /// The remembered hardware reading, **the recorded machine root**, the
    /// event cursor and the last confirmed installation identity, which move
    /// together and never separately.
    ///
    /// **Four, not three, since 2026-09-03.** `Job files and outputs` L1's
    /// recorded root is machine-derived and carries an installation identity,
    /// so D13's own reason reaches it exactly as it reaches the other three: a
    /// host or port edit points the entry at a different machine, and a root
    /// chosen on the old one is the wrong answer. **D3 and D13 were repaired
    /// where this builder stood**; the rule itself does not move.
    pub the_machine_derived_values: Fate,
    pub the_remembered_host_key: Fate,
    pub the_remembered_machine_password: Fate,
    /// **B7: the remembered setup survives every edit**, which is what it is
    /// for — an add whose connection failed on a wrong address is put right by
    /// editing the address and connecting.
    pub the_remembered_setup: Fate,
    /// **D3's row for the chosen fallback folder: kept by every D12 edit, a
    /// host or port edit included** (added 2026-10-04). It is the user's own
    /// instruction and not a value read from a machine, so pointing the entry
    /// at a different machine does not make it wrong.
    pub the_chosen_fallback_folder: Fate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fate {
    Kept,
    Dropped,
    /// **The field is cleared, so the next connection is a first connection and
    /// trusts silently** (D12's host-or-port row for the host key).
    StartedOver,
    /// **D14's erase, without asking.** It is an erase through the secret
    /// adapter against the row's subject reference, and where the store does not
    /// confirm it, `Security and licence` §5.1's **recorded debt** is written
    /// and the sweep clears it later. **This is the one place a stored secret is
    /// destroyed without being asked about**, so a miss leaves a password behind
    /// against a login that no longer exists — **which is why it is an
    /// enumerated step of the edit's own write and not a side effect of
    /// anything.**
    ClearedWithoutAsking,
}

/// **D12's table, performed.**
pub fn what_the_edit_throws_away(changed: WhatChanged) -> WhatTheEditThrowsAway {
    match changed {
        WhatChanged::HostOrPort => WhatTheEditThrowsAway {
            the_machine_derived_values: Fate::Dropped,
            the_remembered_host_key: Fate::StartedOver,
            the_remembered_machine_password: Fate::Kept,
            the_remembered_setup: Fate::Kept,
            the_chosen_fallback_folder: Fate::Kept,
        },
        WhatChanged::Username => WhatTheEditThrowsAway {
            the_machine_derived_values: Fate::Kept,
            the_remembered_host_key: Fate::Kept,
            the_remembered_machine_password: Fate::ClearedWithoutAsking,
            the_remembered_setup: Fate::Kept,
            the_chosen_fallback_folder: Fate::Kept,
        },
        WhatChanged::TheKey | WhatChanged::TheLabelOrTheColour | WhatChanged::TheFallbackFolder => {
            WhatTheEditThrowsAway {
                the_machine_derived_values: Fate::Kept,
                the_remembered_host_key: Fate::Kept,
                the_remembered_machine_password: Fate::Kept,
                the_remembered_setup: Fate::Kept,
                the_chosen_fallback_folder: Fate::Kept,
            }
        }
    }
}

/// **E1 and E3: which edits end a standing period.**
pub fn ends_the_period(changed: WhatChanged) -> Option<PeriodEnding> {
    match changed {
        WhatChanged::HostOrPort | WhatChanged::Username | WhatChanged::TheKey => {
            Some(PeriodEnding::TheEditsSave)
        }
        WhatChanged::TheLabelOrTheColour | WhatChanged::TheFallbackFolder => None,
    }
}

/// **E2: the put-back is O1**, and **S8** is why the replacement list stays at
/// two.
pub const THE_PUT_BACK_IS: OpeningAct = OpeningAct::TheUsersConnectAction;

/// **E2's three consequences, each kept by construction.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatFollowsFromTheEdit {
    /// **A failed put-back is left alone.** There is no period, so N13 applies
    /// and no repair runs. The server is left disconnected with the new details
    /// saved and Smoozy says so.
    pub a_failed_put_back_is_repaired_by_nothing: bool,
    /// **Editing a disconnected server leaves it disconnected**, because there
    /// was no period to end and none is opened.
    pub editing_a_disconnected_server_opens_nothing: bool,
}

pub fn what_follows(a_period_stood: bool, changed: WhatChanged) -> WhatFollowsFromTheEdit {
    let _ = changed;
    WhatFollowsFromTheEdit {
        a_failed_put_back_is_repaired_by_nothing: true,
        editing_a_disconnected_server_opens_nothing: !a_period_stood,
    }
}

/// **E4: what the ended period drops is what §1.3 says it drops** — the
/// last-known queue reading, every live reading, and any machine password held
/// for the connection. **The last two are re-taken on the new period**; the
/// machine password is the user's own choice and is asked for again where it is
/// needed, **which is `Security and licence` §2's arrangement and not a new
/// cost**.
///
/// **E5: transfers are not period-held and do not go with it.** Their durable
/// record is `Server protocol and transfers` S9's, in the local record and keyed
/// to the server row and the installation identity. **A transfer whose channel
/// ends at the edit's disconnect ends as *interrupted*, never as *stopped*** —
/// Smoozy did not stop the user's transfer, it replaced the machine underneath
/// it — so the staged material is kept and the user is told it is there.
///
/// **What becomes of it on the new period** is §12.2's label and C14's gate: on
/// a host or port edit **the installation identity read on the new connection is
/// absent or different, and every transfer record held against the row is
/// disqualified**, which is correct — the entry now points at a different
/// machine.
pub const TRANSFERS_ARE_NOT_PERIOD_HELD: bool = true;

/// **E6: the put-back may meet a changed identity, and it may never meet a
/// passphrase.** These are two different kinds of interruption on one automatic
/// path and the document tells them apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptionOnThePutBack {
    /// **A changed identity puts a question on screen and that is permitted.**
    /// It arises on a username or key edit, where the remembered identity is
    /// **kept** rather than started over, and the machine's identity may have
    /// changed for its own reasons in the meantime. **The user decides**;
    /// declining leaves the server disconnected with the new details saved,
    /// which is the same outcome as any failed put-back.
    AChangedIdentity,
    /// **A key passphrase never does.** Row 33 binds every connection Smoozy
    /// makes on its own, and the put-back is one. Where the new key needs a
    /// passphrase Smoozy is not holding, **the put-back is not attempted at
    /// all**: the server is left disconnected with the new details saved and
    /// **marked with what is wrong**.
    AKeyPassphrase,
}

impl InterruptionOnThePutBack {
    /// **E7: the difference between them is what each question is about.** The
    /// changed-identity question is about **the machine** and only the user can
    /// answer it, and it is a question the spec puts on screen by name. The
    /// passphrase question is about **a secret**, and putting one on screen at a
    /// moment the user is looking elsewhere is what the promise forbids
    /// outright.
    pub fn may_put_a_question_on_screen(self) -> bool {
        matches!(self, InterruptionOnThePutBack::AChangedIdentity)
    }

    /// Where the passphrase is the interruption, **no attempt is made**.
    pub fn the_put_back_is_attempted(self) -> bool {
        matches!(self, InterruptionOnThePutBack::AChangedIdentity)
    }
}

/// **E8: a put-back never carries a remembered setup.**
///
/// A setup is remembered only where the connection failed, so a server holding
/// one **has never been reached and is disconnected** — and editing a
/// disconnected server leaves it disconnected, so **there is no put-back to
/// carry one**. [`crate::setup_bit`] is the mechanism that makes this true
/// rather than merely stated.
pub fn a_put_back_can_carry_a_remembered_setup() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_four_rules_are_cut_on_the_field_and_treating_it_as_one_event_gets_three_wrong() {
        let host = what_the_edit_throws_away(WhatChanged::HostOrPort);
        let username = what_the_edit_throws_away(WhatChanged::Username);
        let key = what_the_edit_throws_away(WhatChanged::TheKey);
        assert_ne!(host, username);
        assert_ne!(username, key);
        assert_ne!(host, key);
    }

    #[test]
    fn a_host_or_port_edit_drops_the_machine_derived_values_together() {
        let fates = what_the_edit_throws_away(WhatChanged::HostOrPort);
        assert_eq!(fates.the_machine_derived_values, Fate::Dropped);
        assert_eq!(fates.the_remembered_host_key, Fate::StartedOver);
        // The password belongs to a login, and the login has not changed.
        assert_eq!(fates.the_remembered_machine_password, Fate::Kept);
    }

    #[test]
    fn a_username_edit_clears_the_machine_password_without_asking_and_nothing_else() {
        let fates = what_the_edit_throws_away(WhatChanged::Username);
        assert_eq!(
            fates.the_remembered_machine_password,
            Fate::ClearedWithoutAsking
        );
        assert_eq!(fates.the_machine_derived_values, Fate::Kept);
        assert_eq!(fates.the_remembered_host_key, Fate::Kept);
    }

    #[test]
    fn a_key_edit_clears_nothing_at_all() {
        let fates = what_the_edit_throws_away(WhatChanged::TheKey);
        assert_eq!(fates.the_machine_derived_values, Fate::Kept);
        assert_eq!(fates.the_remembered_host_key, Fate::Kept);
        assert_eq!(fates.the_remembered_machine_password, Fate::Kept);
    }

    #[test]
    fn the_remembered_setup_survives_every_edit() {
        for changed in [
            WhatChanged::HostOrPort,
            WhatChanged::Username,
            WhatChanged::TheKey,
            WhatChanged::TheLabelOrTheColour,
        ] {
            assert_eq!(
                what_the_edit_throws_away(changed).the_remembered_setup,
                Fate::Kept
            );
        }
    }

    /// **D3 and E3 for the chosen fallback folder (2026-10-04, `Job files
    /// and outputs` P23)**: every edit keeps it, a host or port edit included,
    /// and changing it alone ends no period.
    #[test]
    fn the_chosen_fallback_folder_survives_every_edit_and_changing_it_ends_no_period() {
        for changed in [
            WhatChanged::HostOrPort,
            WhatChanged::Username,
            WhatChanged::TheKey,
            WhatChanged::TheLabelOrTheColour,
            WhatChanged::TheFallbackFolder,
        ] {
            assert_eq!(what_the_edit_throws_away(changed).the_chosen_fallback_folder, Fate::Kept);
        }
        assert_eq!(ends_the_period(WhatChanged::TheFallbackFolder), None);
    }

    #[test]
    fn a_rename_ends_no_period_and_opens_nothing() {
        assert_eq!(ends_the_period(WhatChanged::TheLabelOrTheColour), None);
        assert_eq!(
            ends_the_period(WhatChanged::Username),
            Some(PeriodEnding::TheEditsSave)
        );
    }

    #[test]
    fn editing_a_disconnected_server_leaves_it_disconnected() {
        let follows = what_follows(false, WhatChanged::HostOrPort);
        assert!(follows.editing_a_disconnected_server_opens_nothing);
        assert!(follows.a_failed_put_back_is_repaired_by_nothing);
    }

    #[test]
    fn a_changed_identity_may_ask_and_a_passphrase_may_never() {
        assert!(InterruptionOnThePutBack::AChangedIdentity.may_put_a_question_on_screen());
        assert!(!InterruptionOnThePutBack::AKeyPassphrase.may_put_a_question_on_screen());
        assert!(!InterruptionOnThePutBack::AKeyPassphrase.the_put_back_is_attempted());
    }

    #[test]
    fn the_put_back_is_o1_and_carries_no_remembered_setup() {
        assert_eq!(THE_PUT_BACK_IS, OpeningAct::TheUsersConnectAction);
        assert!(!a_put_back_can_carry_a_remembered_setup());
    }
}

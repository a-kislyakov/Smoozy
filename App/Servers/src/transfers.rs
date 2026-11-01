//! Transfers across a repair — `Servers` §12.
//!
//! **The constraint** is `Server protocol and transfers` C11 — *"A replaced
//! connection replaces the channels and nothing else"* — and §7's
//! staging-and-publication contract, **neither of which is re-solved here**.
//!
//! **T1: the period owes a transfer a connection and nothing else.** What
//! survives a replacement is C11's, resume is `Server protocol and transfers`
//! S12's, and publication is `Server protocol and transfers` S15's. **Both are
//! that document's and not this one's** — this plan's S12 is *a drop does not
//! end a period* and its S15 is *where the cap is 0, no session is permitted*.
//! The plan writes both markers as links into the other document; this
//! sentence was copied from it and the links were dropped.
//! **This document does not re-solve resumption**; it decides **when a
//! connection is a replacement** and **what a break does to a transfer's
//! record**.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` C11, C13, C14,
//! S26.

use crate::period::{OpeningAct, Replacement};

/// **T6: every connection this document hands over is labelled as one of two
/// things, and the label is what C11 reads.**
///
/// **T9: this is the point where the period stops being bookkeeping.** A
/// repaired connection is handed to `File Browser` and to `Download` mid-work,
/// **and both read the same two labels. Neither is told anything about the
/// period beyond them.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionLabel {
    /// **R1 and R2.** What follows is **C11's four steps in order**: the
    /// installation-identity gate, capabilities negotiated afresh, outstanding
    /// pending-command records resolved, and **step 4's re-establishment of
    /// transfer channels — whose scope is C11 step 4's own and is not restated
    /// here**.
    Replacement,
    /// **O1 and O2.** The same gate and the same fresh negotiation, **and no
    /// transfer is resumed**.
    PeriodOpening,
}

impl ConnectionLabel {
    /// **T7: that label is why *the user disconnecting is the opposite case*
    /// holds.** A disconnect ends the period, **so the user's later connect is
    /// period-opening**, C11 step 4 does not fire, and **nothing resumes on its
    /// own** — *"there is no connection left and none was asked for"*. What is
    /// kept is C13's rule that **nothing starts work a user did not ask for on a
    /// new connection**, and Row 197's promise that **the leftover is kept and
    /// the user is told it is there**.
    pub fn c11_step_four_fires(self) -> bool {
        matches!(self, ConnectionLabel::Replacement)
    }
}

/// The label a replacement carries.
pub fn label_for_a_replacement(_which: Replacement) -> ConnectionLabel {
    ConnectionLabel::Replacement
}

/// The label a period-opening act carries.
pub fn label_for_an_opening_act(_which: OpeningAct) -> ConnectionLabel {
    ConnectionLabel::PeriodOpening
}

/// **T8: the additional connection carries the primary connection's label.**
///
/// It is opened inside a period and **its own opening is never a replacement
/// event**: transfers are resumed **once, on the primary connection's label**,
/// and seated on whichever connection admission gives them.
pub fn label_for_the_additional_connection(primary: ConnectionLabel) -> ConnectionLabel {
    primary
}

/// **T2: a break declared under a standing period records every transfer in
/// flight as *interrupted*, never as *stopped*.**
///
/// That is S26's third row: **the staged material is kept** so the transfer can
/// be carried on, and the record names the staging path and the last checkpoint.
/// **Smoozy did not stop the user's transfer; its path failed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowATransferEnded {
    Interrupted,
    Stopped,
}

pub fn how_a_break_ends_a_transfer() -> HowATransferEnded {
    HowATransferEnded::Interrupted
}

/// **T3: the repaired connection is a new transport and always a new one**, per
/// K5. C11 does not need it to be the same socket — *"the identity is the
/// caller's and the record is the machine's; neither belongs to a connection"* —
/// **so correlation and every transfer record survive by construction.**
pub const A_REPAIRED_CONNECTION_IS_ALWAYS_A_NEW_TRANSPORT: bool = true;

/// **T4: resume rests on a staging path Smoozy names and never on a tool's own
/// partial.**
///
/// Fact 17's two conditions are why this matters and **they are already absorbed
/// by the contract**: on one box the sending direction *"accepted the flag and
/// resumed nothing"* and re-sent in full, and a hard kill on another left rsync's
/// own *"randomly named temporary file"*, *"a name Smoozy cannot predict"*.
/// `Server protocol and transfers` S4 and S5 **put the name inside Smoozy, so the
/// case does not arise**, and Row 45 promises no more than the verified resume
/// of `Server protocol and transfers` S12 delivers — that document's S12 and
/// not this one's.
///
/// **§16 item 14 names these so nobody reads Row 45 as resting on a tool's own
/// resume.**
pub fn resume_ever_reads_a_tools_own_partial() -> bool {
    false
}

/// **T5: where the machine turns out to be a different one, the transfers go
/// with it.**
///
/// C14's gate runs **before anything is resumed**: **on *absent* or *present and
/// different*, every transfer record and staged partial held against this server
/// row is disqualified.** That is correct for a repaired connection to a recycled
/// container **and** correct for the put-back after a host or port edit.
pub fn transfers_are_disqualified(
    check: smoozy_state::identities::InstallationCheck,
) -> bool {
    use smoozy_state::identities::MachineDerivedValue;
    check.what_becomes_of_machine_derived_values() == MachineDerivedValue::Disqualified
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::InstallationCheck;

    #[test]
    fn a_repair_and_a_restart_return_are_both_replacements() {
        for which in Replacement::ALL {
            assert_eq!(label_for_a_replacement(which), ConnectionLabel::Replacement);
        }
    }

    #[test]
    fn both_opening_acts_are_period_opening_and_resume_nothing() {
        for which in OpeningAct::ALL {
            let label = label_for_an_opening_act(which);
            assert_eq!(label, ConnectionLabel::PeriodOpening);
            assert!(!label.c11_step_four_fires());
        }
    }

    #[test]
    fn a_connect_after_a_disconnect_resumes_nothing_on_its_own() {
        // T7: the disconnect ended the period, so the later connect is
        // period-opening and C11 step 4 does not fire.
        assert!(!label_for_an_opening_act(OpeningAct::TheUsersConnectAction).c11_step_four_fires());
        assert!(label_for_a_replacement(Replacement::RepairAfterAnUnintendedDrop)
            .c11_step_four_fires());
    }

    #[test]
    fn the_additional_connection_carries_the_primarys_label_and_is_never_its_own_event() {
        assert_eq!(
            label_for_the_additional_connection(ConnectionLabel::Replacement),
            ConnectionLabel::Replacement
        );
        assert_eq!(
            label_for_the_additional_connection(ConnectionLabel::PeriodOpening),
            ConnectionLabel::PeriodOpening
        );
    }

    #[test]
    fn a_break_records_a_transfer_as_interrupted_and_never_as_stopped() {
        assert_eq!(how_a_break_ends_a_transfer(), HowATransferEnded::Interrupted);
    }

    #[test]
    fn a_recycled_container_disqualifies_every_transfer_record() {
        assert!(transfers_are_disqualified(InstallationCheck::Absent));
        assert!(transfers_are_disqualified(InstallationCheck::PresentAndDifferent));
        assert!(!transfers_are_disqualified(
            InstallationCheck::PresentAndEqualConfirmed
        ));
        // The fourth branch discards nothing.
        assert!(!transfers_are_disqualified(
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer
        ));
    }

    #[test]
    fn resume_never_reads_a_tools_own_partial() {
        assert!(!resume_ever_reads_a_tools_own_partial());
    }
}

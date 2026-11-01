//! The remembered setup bit — `Servers` §7.
//!
//! **The constraint** is Row 26 and its dependency on Row 32: **the rule turns
//! on *reached the machine or not*, and a restart mid-setup filed on the wrong
//! side either forgets a setup it should keep or keeps one it should forget.**
//!
//! **B1: the bit is written at exactly one moment — an add carrying a ticked
//! setup option whose period never gets a live connection.** It records which
//! options were ticked, and it is written **as part of the same durable write
//! that saves the server**.

use smoozy_state::lifetimes::Lifetime;

/// **B2: *reached the machine* is a single instant and it is named exactly: the
/// moment the SSH transport reports the connection authenticated and usable.**
///
/// Before that instant the connection failed and the bit is written. **From that
/// instant on, the bit is not written, whatever happens next** — not by a setup
/// step failing, not by a drop mid-setup, not by a restart Smoozy caused, and
/// not by a half-finished add where one option succeeded and the other did not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereTheAddGotTo {
    /// The connection failed before the transport reported it authenticated and
    /// usable.
    TheConnectionNeverReachedTheMachine,
    /// **B3: a restart is filed on the *reached* side and the question never
    /// arises for it.** A restart is not a case for this bit at all: the
    /// connection was established, the setup is not abandoned, and `Server
    /// Setup`'s recovery run carries it on from its recorded boundary. **The bit
    /// exists only for a setup that never started.**
    TheTransportReportedItAuthenticatedAndUsable,
}

/// **B1 and B2, as one function**, so no path writes the bit on the *reached*
/// side by reasoning about what happened afterwards.
pub fn the_bit_is_written(
    a_setup_option_was_ticked: bool,
    got_to: WhereTheAddGotTo,
) -> bool {
    a_setup_option_was_ticked
        && matches!(got_to, WhereTheAddGotTo::TheConnectionNeverReachedTheMachine)
}

/// **B4: the instant is after step 3 of the first-connection sequence and
/// before steps 4 to 8**, so **a machine that authenticates and is then found
/// not to be Linux is on the *reached* side.**
///
/// The user is told plainly it is not a Linux machine and **nothing about the
/// setup is remembered**, which is right — a remembered setup would only run
/// again on the same machine and fail the same way.
pub const AFTER_STEP: u8 = 3;
pub const BEFORE_STEP: u8 = 4;

/// **B5: three things clear it and the list is closed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatClearsTheBit {
    /// **1. A connection to that server is established**, whatever the setup
    /// then does.
    ///
    /// **The bit is cleared in the same durable write as the moment B2 names**
    /// — the transport reporting the connection authenticated and usable — **and
    /// the remembered options are handed to `Server Setup` from what that write
    /// carried**, so the clearing does not depend on the hand-over happening and
    /// **no path that is established and then goes no further leaves the bit
    /// standing**. The setup runs exactly once from the bit and the bit is gone
    /// whether the setup succeeds, partly succeeds or fails. That is the spec's
    /// rule kept literally: *"Once Smoozy has reached the machine, nothing about
    /// the setup is remembered."*
    AConnectionIsEstablished,
    /// **2. The user drops it** without carrying it out.
    TheUserDropsIt,
    /// **3. The server is removed.**
    TheServerIsRemoved,
}

impl WhatClearsTheBit {
    pub const ALL: [WhatClearsTheBit; 3] = [
        WhatClearsTheBit::AConnectionIsEstablished,
        WhatClearsTheBit::TheUserDropsIt,
        WhatClearsTheBit::TheServerIsRemoved,
    ];
}

/// **B6: the connect action carries it.** A server holding the bit reads as
/// needing both, and its connect action says it will do both. **What that action
/// is called is Design's.**
pub const THE_CONNECT_ACTION_CARRIES_IT: bool = true;

/// **B8: the bit is the user's own instruction and nothing was read from a
/// machine to produce it**, so **it carries no installation identity and no
/// installation-identity branch disqualifies it**.
///
/// §11.1 puts *"a server's remembered setup state"* in the **subject-held**
/// class beside machine-derived values, and **this clause is what tells the two
/// apart inside that class**. §16 item 9 carries it with the host key's, and
/// closed in this document's favour on 2026-08-22: §2.3 names both exceptions
/// and their **different** reasons.
pub const THE_BIT: Lifetime = Lifetime::SubjectHeld;

/// **B8, stated as the function a builder would otherwise write wrongly.**
pub fn is_machine_derived() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edits::{what_the_edit_throws_away, Fate, WhatChanged};

    #[test]
    fn the_bit_is_written_only_where_the_connection_never_reached_the_machine() {
        assert!(the_bit_is_written(
            true,
            WhereTheAddGotTo::TheConnectionNeverReachedTheMachine
        ));
        assert!(!the_bit_is_written(
            true,
            WhereTheAddGotTo::TheTransportReportedItAuthenticatedAndUsable
        ));
        assert!(!the_bit_is_written(
            false,
            WhereTheAddGotTo::TheConnectionNeverReachedTheMachine
        ));
    }

    #[test]
    fn a_restart_is_filed_on_the_reached_side_so_the_question_never_arises() {
        // B3. A restart mid-setup filed the other way would keep a setup that
        // `Server Setup`'s recovery run is already carrying on.
        assert!(!the_bit_is_written(
            true,
            WhereTheAddGotTo::TheTransportReportedItAuthenticatedAndUsable
        ));
    }

    #[test]
    fn a_machine_that_authenticates_and_is_not_linux_is_on_the_reached_side() {
        // B4: the instant sits after step 3 and before the not-Linux check at
        // step 4, so nothing about the setup is remembered there.
        assert!(!the_bit_is_written(
            true,
            WhereTheAddGotTo::TheTransportReportedItAuthenticatedAndUsable
        ));
    }

    #[test]
    fn three_things_clear_it_and_the_list_is_closed() {
        assert_eq!(WhatClearsTheBit::ALL.len(), 3);
    }

    #[test]
    fn it_survives_every_edit_which_is_what_it_is_for() {
        // B7: an add whose connection failed on a wrong address is put right by
        // editing the address and connecting.
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

    #[test]
    fn it_is_subject_held_and_carries_no_installation_identity() {
        assert_eq!(THE_BIT, Lifetime::SubjectHeld);
        assert!(!is_machine_derived());
    }
}

//! **Permission, and what a refusal is** — `Desktop app and delivery` §13.2,
//! T6, T7 and T8.
//!
//! **Three models rather than one, and the adapter holds three.** That is T6
//! in its own words, and it is why this file's one type has values no single
//! system can produce all of: **macOS asks and reads the state back and a
//! denial is silent; Windows never asks and only reads; Linux has no
//! permission model at all.** A shape that pretended the three were one would
//! have to invent either a prompt Windows does not draw or a denied state
//! Linux cannot have.
//!
//! **Nothing here ever tells the user anything.** T8: a denied state is
//! treated exactly as T5's Linux case — the kinds do not arrive, **no failure
//! record is raised**, and nothing else changes. The per-kind switches in
//! Settings stay the user's own record of what they want whether or not the
//! system will deliver any of it.

/// **What this system says about permission to post** — T6.
///
/// **A denial is silent on macOS** and the system *"ignores any attempts to
/// schedule local notifications"*, so the state being **readable back** is the
/// whole of how Smoozy can know; nothing about a refusal ever arrives at a
/// post.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheSystemSays {
    /// **macOS, after a granted request; Windows, where the read-only setting
    /// is enabled.** Posts are attempted.
    ItIsGranted,
    /// **macOS, where the user refused.** T6: readable back, and silent.
    ItWasDenied,
    /// **macOS, before the first request.** T7 is what turns this into an ask,
    /// and it does so at the first moment a post would happen and never at
    /// application start.
    NothingHasBeenAskedYet,
    /// **Windows: the read-only setting, in one of its ways of being switched
    /// off.** T6: *"no prompt and no request interface found, only a read-only
    /// setting whose every non-enabled value is a way of being switched off"*
    /// — so the values are not told apart here, there being nothing different
    /// to do about any of them.
    TheSystemHasItSwitchedOff,
    /// **Linux: there is no permission model in the specification at all**, so
    /// there is no denied state to read (T6). It is not the same as granted:
    /// nothing was granted, and nothing could be.
    ThereIsNoPermissionModel,
}

impl WhatTheSystemSays {
    /// **Whether a post is attempted at all.**
    ///
    /// **Attempted, never delivered** — T23: delivery is never confirmed on
    /// any of the three, so this answers what Smoozy does and never what the
    /// user gets.
    ///
    /// [`WhatTheSystemSays::NothingHasBeenAskedYet`] is **not** a yes: T7
    /// makes the asking part of the first post, so a caller that met this
    /// value asks first and reads again. [`the_asking_comes_first`] is the
    /// line that says so.
    pub fn a_post_is_attempted(self) -> bool {
        match self {
            WhatTheSystemSays::ItIsGranted | WhatTheSystemSays::ThereIsNoPermissionModel => true,
            WhatTheSystemSays::ItWasDenied
            | WhatTheSystemSays::TheSystemHasItSwitchedOff
            | WhatTheSystemSays::NothingHasBeenAskedYet => false,
        }
    }

    /// **T7: this state is the one the request is made from**, and the request
    /// is made at the first moment a notification would actually be posted.
    pub fn the_asking_comes_first(self) -> bool {
        matches!(self, WhatTheSystemSays::NothingHasBeenAskedYet)
    }

    /// **T8: a refusal is not a failure record.** No value of this type ever
    /// produces one, and this constant is what a reader looking for the
    /// missing branch finds instead of a missing branch.
    pub const A_REFUSAL_RAISES_NO_FAILURE_RECORD: bool = true;
}

/// **Which of the three models a system has**, so that the shape of the
/// answer can be checked at this desk for all three rather than only for the
/// one this machine is.
///
/// **The mapping is T6's and is read from each vendor's own publication**; no
/// system's own code is reached from here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheModel {
    /// macOS: **request, read back, and a denial is silent.**
    AskAndReadBack,
    /// Windows: **no prompt and no request interface, only a read-only
    /// setting.**
    ReadOnly,
    /// Linux: **none at all.**
    None,
}

/// **Which model a named system has** — T6.
///
/// A system this document does not name has no model here and is treated as
/// having none, which is the honest answer for a system §1's three-system
/// boundary was never written for.
pub fn the_model_of(system: &str) -> TheModel {
    match system {
        "macos" => TheModel::AskAndReadBack,
        "windows" => TheModel::ReadOnly,
        _ => TheModel::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **T6: three models, not one.** The line that fails if somebody
    /// collapses them.
    #[test]
    fn the_three_systems_have_three_permission_models() {
        assert_eq!(the_model_of("macos"), TheModel::AskAndReadBack);
        assert_eq!(the_model_of("windows"), TheModel::ReadOnly);
        assert_eq!(the_model_of("linux"), TheModel::None);
    }

    /// **T8: a denial and a switched-off setting both stop the posting and
    /// neither says anything.**
    ///
    /// There is no branch anywhere in this crate that turns either into a
    /// telling, and the two constants below are what a reader meets where they
    /// went looking for one.
    #[test]
    fn a_refusal_stops_the_post_and_says_nothing() {
        assert!(!WhatTheSystemSays::ItWasDenied.a_post_is_attempted());
        assert!(!WhatTheSystemSays::TheSystemHasItSwitchedOff.a_post_is_attempted());
        assert!(WhatTheSystemSays::A_REFUSAL_RAISES_NO_FAILURE_RECORD);
    }

    /// **T6: Linux having no model is not the same as being granted one**, and
    /// the post is attempted anyway — which is the whole of what *no
    /// permission model* means for behaviour.
    #[test]
    fn linux_has_nothing_to_refuse_and_the_post_is_attempted() {
        assert!(WhatTheSystemSays::ThereIsNoPermissionModel.a_post_is_attempted());
        assert!(!WhatTheSystemSays::ThereIsNoPermissionModel.the_asking_comes_first());
    }

    /// **T7: the unasked state is an ask and not a refusal.**
    ///
    /// This is the line that fails if somebody makes *nothing has been asked
    /// yet* mean *go ahead*, which would post nothing on a fresh Mac for ever
    /// and look exactly like a denial.
    #[test]
    fn the_unasked_state_asks_rather_than_refusing() {
        assert!(WhatTheSystemSays::NothingHasBeenAskedYet.the_asking_comes_first());
        assert!(!WhatTheSystemSays::NothingHasBeenAskedYet.a_post_is_attempted());
    }
}

//! The Telegram role's material, and its two entry points — `Server Setup` §7.
//!
//! **The constraint** is Row 91; `Telegram` §20's material list, **which this
//! document places and does not decide**; `Rented-server runtime` §5.3 and W7
//! to W9; `Security and licence` §10.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Telegram` §20, L4. `Rented-server runtime` §2, §5.3,
//! K11, K19, W7, W8, W9. `Security and licence` §10. `Server protocol and
//! transfers` §5.

/// **G1: the material is `Telegram` §20's list whole**, and this document
/// places it rather than deciding it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatIsPlaced {
    /// **The token file**, placed per `Security and licence` §10 — inside the
    /// authenticated SSH connection as part of this placement, **never a command
    /// argument**, a file owned by the supervisor's login permitting that login
    /// only, **in the package state area, outside every job folder, never under
    /// a shared temporary area**.
    TheTokenFile,
    /// **The role configuration** — the proved conversation identity, the
    /// machine's own identity and display label, and the machine-level
    /// reporting settings.
    TheRoleConfiguration,
    /// **The durable enabled-or-disabled desired state**, set through the
    /// ordinary command path (`Rented-server runtime` W8), **with the supervisor
    /// doing every start and stop**.
    TheDesiredState,
}

impl WhatIsPlaced {
    pub const THE_LIST: [WhatIsPlaced; 3] = [
        WhatIsPlaced::TheTokenFile,
        WhatIsPlaced::TheRoleConfiguration,
        WhatIsPlaced::TheDesiredState,
    ];
}

/// **G2: the Row 91 decision — material is placed by whichever entry point runs
/// while Telegram is on, and is not placed in advance.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryPoint {
    /// **At first setup with Telegram on, placement is
    /// [`crate::ready_action`] R1 step 8 of the ready action's run.**
    TheReadyActionsOwnStep,
    /// **The dedicated entry point places the same material on a machine
    /// prepared earlier and sets the desired state to enabled, and does nothing
    /// else**: no other setup step re-runs, **the machine is not restarted**,
    /// and a change of the desired state restarts nothing but the role worker
    /// (`Rented-server runtime` W9).
    TheDedicatedEntryPoint,
}

/// **G2, performed: what a machine gets, given whether Telegram is on.**
///
/// **Where Telegram is off at setup, nothing of it is placed.**
pub fn what_the_ready_action_places(telegram_is_on: bool) -> &'static [WhatIsPlaced] {
    if telegram_is_on {
        &WhatIsPlaced::THE_LIST
    } else {
        &[]
    }
}

/// **G2, performed per entry point: what each one places.**
///
/// *"Material is placed by whichever entry point runs while Telegram is on"*,
/// and it is `Telegram` §20's list whole either way — **the dedicated entry
/// point places the same material and sets the desired state to enabled**, so
/// it is never reached with Telegram off.
pub fn what_an_entry_point_places(
    entry: EntryPoint,
    telegram_is_on: bool,
) -> &'static [WhatIsPlaced] {
    match entry {
        EntryPoint::TheReadyActionsOwnStep => what_the_ready_action_places(telegram_is_on),
        EntryPoint::TheDedicatedEntryPoint => &WhatIsPlaced::THE_LIST,
    }
}

/// **G2: the no-restart promise is kept by a mechanism already built, not by
/// care here.**
pub fn the_dedicated_entry_point_restarts_the_machine(_it_would_be_simpler: bool) -> bool {
    false
}

/// **G2: no other setup step re-runs at the dedicated entry point.**
pub fn what_else_the_dedicated_entry_point_runs() -> &'static [crate::ready_action::Step] {
    &[]
}

/// **G3: the dedicated entry point on a machine that holds no package places
/// the package first and still restarts nothing.**
///
/// The role worker is supervisor-hosted; **without the runtime there is nothing
/// to host it.** The placement is the ordinary transaction (`Rented-server
/// runtime` §2) through the bootstrap-and-repair request, **which restarts no
/// machine** — on a container its registration step needs the elevated hook
/// install (`Rented-server runtime` K11, K19), and [`crate::elevation`] E2's flow
/// applies where the login must be asked for a password. **The rest of the setup
/// — the ready action's machine preparation — is not run.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatTheDedicatedEntryPointDoes {
    pub places_the_package_first: bool,
    pub restarts_the_machine: bool,
    pub runs_the_ready_actions_machine_preparation: bool,
}

/// **G3, performed.**
pub fn what_the_dedicated_entry_point_does(
    the_machine_holds_no_package: bool,
) -> WhatTheDedicatedEntryPointDoes {
    WhatTheDedicatedEntryPointDoes {
        places_the_package_first: the_machine_holds_no_package,
        restarts_the_machine: false,
        runs_the_ready_actions_machine_preparation: false,
    }
}

/// **G4: replacement deliveries reuse the same placement vehicle**
/// (`Telegram` L4): the new token file replaces the old over the same route,
/// **and the told-record's bookkeeping is that document's.**
pub fn a_replacement_uses_a_second_vehicle(_it_is_a_replacement: bool) -> bool {
    false
}

/// **§13 item 10, recorded rather than repaired.**
///
/// **The dedicated entry point on a never-prepared machine places the package
/// (G3), and whether that placement's elevated container step belongs inside an
/// action the spec describes as *"adds Telegram's piece and does nothing else"*
/// is a reading this document took rather than found written.** **Breaks:**
/// nothing mechanical — the alternative is an entry point that refuses
/// unprepared machines, which the spec does not say either. **The no-restart and
/// no-other-setup promises hold on both readings**, which is why this is
/// recorded for the runner and built as written.
pub const THE_CONTAINER_HOOK_STEP_INSIDE_THE_DEDICATED_ENTRY_POINT_IS_A_READING: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_of_telegram_is_placed_where_it_is_off_at_setup() {
        assert!(what_the_ready_action_places(false).is_empty());
        assert_eq!(what_the_ready_action_places(true).len(), 3);
        assert_eq!(WhatIsPlaced::THE_LIST.len(), 3);
    }

    #[test]
    fn the_dedicated_entry_point_places_and_does_nothing_else() {
        let bare = what_the_dedicated_entry_point_does(true);
        assert!(bare.places_the_package_first);
        assert!(!bare.restarts_the_machine);
        assert!(!bare.runs_the_ready_actions_machine_preparation);

        let prepared = what_the_dedicated_entry_point_does(false);
        assert!(!prepared.places_the_package_first);
        assert!(!prepared.restarts_the_machine);

        assert!(what_else_the_dedicated_entry_point_runs().is_empty());
        assert!(!the_dedicated_entry_point_restarts_the_machine(true));
    }

    #[test]
    fn a_replacement_delivery_reuses_the_same_vehicle() {
        assert!(!a_replacement_uses_a_second_vehicle(true));
        const { assert!(THE_CONTAINER_HOOK_STEP_INSIDE_THE_DEDICATED_ENTRY_POINT_IS_A_READING) };
    }

    #[test]
    fn there_are_two_entry_points_and_both_set_the_same_desired_state() {
        // **The fault this test used to have.** Its loop asserted that an array
        // of two values matched those same two values, and it never tested that
        // either entry point set the desired state at all.
        //
        // G2 and Y6b: one source whichever entry point placed the material.
        for entry in [
            EntryPoint::TheReadyActionsOwnStep,
            EntryPoint::TheDedicatedEntryPoint,
        ] {
            let placed = what_an_entry_point_places(entry, true);
            assert!(
                placed.contains(&WhatIsPlaced::TheDesiredState),
                "{entry:?} places no desired state"
            );
            assert_eq!(placed.len(), WhatIsPlaced::THE_LIST.len());
        }
        // **And nothing of it is placed by the ready action where Telegram is
        // off at setup**, which is the one difference between the two.
        assert!(what_an_entry_point_places(EntryPoint::TheReadyActionsOwnStep, false).is_empty());
        assert!(WhatIsPlaced::THE_LIST.contains(&WhatIsPlaced::TheDesiredState));
    }
}

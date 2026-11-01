//! **B2's second performer: the desktop carrying out the package transaction
//! through the pre-package surface.**
//!
//! # What this is, and why nothing did it before
//!
//! B2 divides the transaction between two performers. *"Where a verified
//! runtime is alive, `Rented-server runtime` may perform its own half of the
//! transaction; where none is, the desktop performs it through the surface, and
//! the transaction's shape is that document's either way."* **The machine-side
//! half has been built since step 8a and the desktop's did not exist at all**,
//! so on a bare rented machine — where there is never a live runtime — nothing
//! anywhere performed the placement. `Rented-server runtime` G1b says the same
//! thing from the other side: *"Every bare machine, and every machine whose
//! supervisor is dead — which is the whole case the bootstrap-and-repair
//! request exists for."*
//!
//! **The rules are not here.** Every decision this module reaches for is
//! `Rented-server runtime`'s, in `smoozy_runtime::package` — the closed phase
//! set, what a claim that is already held means, the retry table, what the
//! health exchange produces, what a rollback restores, what the sweep may
//! reach. **This module is the acts**, and it holds no judgement of its own: a
//! second copy of the transaction's shape on the desktop would drift from the
//! machine's, and the two would then disagree about a machine neither could
//! read.
//!
//! # The one thing that is genuinely this side's
//!
//! **The machine's own acting half opens files, takes a kernel lock, sends a
//! signal and renames a path. This one has neither a filesystem nor a
//! process.** So every act below is one of §1.1's two mechanisms and nothing
//! else: a subsystem operation, or one constant command on the command channel.
//! `Rented-server runtime` G1b is exact about what that costs and what stands in
//! its place — *"the surface has no route to a kernel lock"*, and the claim is
//! the exclusive create of the claim file itself.
//!
//! # What the traits next door are, and why this module does not implement them
//!
//! `smoozy_runtime::package` publishes five traits — the claim, the phase
//! record, the two reads the commit rests on, the commit's own three acts, and
//! the health answer. **They are the machine-side performer's shape and every
//! one of them is synchronous**, because that performer is a program on the
//! machine with the machine's own filesystem under it. **Every act on this side
//! is a round trip over a connection**, so implementing those traits here would
//! mean blocking a thread inside an asynchronous runtime on every one — which
//! is not a style preference but a stall of every other transfer on the same
//! runtime. **So the traits are left where they are and this module performs the
//! same acts with the same rules**, and the two performers are held together by
//! taking every decision from the same functions rather than by sharing a trait.
//!
//! # What is not performed here, and it is one thing
//!
//! **The root step on a container is `Server Setup`'s** and always was:
//! `Rented-server runtime` K11 makes installing the provider's hook an elevated
//! step, and K19 puts a second obligation on the same step. What this module
//! does on that machine kind is write K16's record and hand the hook over,
//! which is `Rented-server runtime` K16a's own ending and not a refusal.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rented-server runtime` G1a, G1b, G10, G11, G12,
//! G13, G14, G14a, G15, G16, G17, G18, G19, G20, G21, G22, G25, G26, G29, G31a,
//! K11, K15, K16, K16a, K16b, K19.
//!
//! **A marker this plan also defines is never listed there and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause.
//! `Rented-server runtime` G1 to G9 are all of that kind here.

pub mod claim;
pub mod performing;
pub mod records;
pub mod registration;

pub use claim::{
    release_the_claim_at_the_terminal_write, take_the_claim, TheClaimsOwnPath,
    WhatTakingTheClaimReached,
};
pub use performing::{
    ask_for_the_health_answer, ask_the_running_supervisor_to_end, complete_the_rollback,
    place_a_verified_release, what_the_placement_reached, ThePlacementsOutcome,
    ThePlacementsSurface, WhatThePlacementReached, WhetherTheRunningSupervisorEnded,
    WhyThePlacementCouldNotBeCarriedOut, THE_ENTRY_POINTS_CARRIAGE_SITE,
    THE_FOUR_THINGS_THE_HEALTH_EXCHANGE_ASKS_FOR, THE_HEALTH_ANSWER_REQUEST,
    THE_HEALTH_EXCHANGE_COMMAND,
};
pub use records::{
    make_the_installation_identity_durable, publish_durably, read_the_boot_registration_record,
    read_the_phase_record, record_the_endpoints_path, rename_the_pointer_to,
    the_active_pointers_value, the_record_at, the_staging_location_for,
    whether_the_durability_ordering_applies, write_the_boot_registration_record,
    write_the_phase_record, WhetherTheDurabilityOrderingApplied,
};
/// **`Rented-server runtime` K10's path**, re-exported so the crate that owns
/// the root step's commands names the one constant rather than a copy.
pub use smoozy_runtime::registration_content::THE_PROVIDERS_HOOK_PATH;
pub use registration::{
    confirm_the_registration, enable_the_unit, grant_lingering, install_the_hook,
    place_the_registration, read_the_linger_flag_back, start_the_supervisor_the_pointer_names,
    the_container_hooks_content, the_registration_this_placement_would_make,
    the_user_manager_answers, TheCommandAndItsElevatedForm, TheRootStepOnAContainer, WhereAUserUnitSitsOnTheSubsystem,
    THE_KEEPERS_START, THE_KEEPERS_START_FILE_ON_THE_SUBSYSTEM, THE_LINGERING_GRANT,
    THE_LINGER_READ_BACK, THE_LOGINS_OWN_NAME, THE_SUPERVISOR_START, THE_UNIT_ENABLE,
    THE_USER_MANAGER_ASKED,
};

/// **The whole set of acts this module performs, in the order the transaction
/// takes them.**
///
/// **It is the plan's own order and not a shape invented here.**
/// `Rented-server runtime` G3's phase table fixes what is written before what,
/// `Rented-server runtime` G9 puts the registration before the *committing*
/// phase, and that document's G25 fixes the last three. **The value exists so
/// that the order is a thing a test can read** rather than the order two calls
/// happen to sit in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnActOfThePlacement {
    /// `Rented-server runtime` G1b's exclusive create. **Nothing else happens
    /// until it is held.**
    TheClaimIsTaken,
    /// `Rented-server runtime` G4's one write: the fresh attempt's identity, the
    /// carried-forward previous values and the *inspecting* phase together.
    TheFreshAttemptsFirstWriteIsMade,
    /// The artifact's bytes are placed at the staging location the phase record
    /// names, under the *staging* phase.
    TheArtifactIsStaged,
    /// `Rented-server runtime` G29's at-rest comparison, and only then the mode
    /// that makes the artifact runnable. The *staged and validated* phase
    /// follows it.
    TheStagedArtifactIsReadBackAndMadeExecutable,
    /// `Rented-server runtime` K16's record, then the registration itself, then
    /// K15's read-back — all before the *committing* phase, which is
    /// `Rented-server runtime` G9's own ordering.
    TheRegistrationIsPlacedAndConfirmed,
    /// `Rented-server runtime` G10's identity, then `Rented-server runtime`
    /// G7's single rename, then the activation and the health exchange — that
    /// document's own six steps, which are
    /// `smoozy_runtime::package::commit::CommitStep::IN_ORDER`.
    TheCommitIsPerformed,
}

impl AnActOfThePlacement {
    /// The order, and there is no other.
    pub const IN_ORDER: [AnActOfThePlacement; 6] = [
        AnActOfThePlacement::TheClaimIsTaken,
        AnActOfThePlacement::TheFreshAttemptsFirstWriteIsMade,
        AnActOfThePlacement::TheArtifactIsStaged,
        AnActOfThePlacement::TheStagedArtifactIsReadBackAndMadeExecutable,
        AnActOfThePlacement::TheRegistrationIsPlacedAndConfirmed,
        AnActOfThePlacement::TheCommitIsPerformed,
    ];
}

/// The package area as text, with a trailing separator taken off.
///
/// **One area written two ways gives one set of paths**, which matters because
/// the same area reaches this module from a caller and reaches
/// [`crate::bootstrap::inspect`] from another.
pub fn the_package_area_as_text(area: &[u8]) -> String {
    let text = String::from_utf8_lossy(area).into_owned();
    text.strip_suffix('/').map(str::to_string).unwrap_or(text)
}

/// **`Rented-server runtime` G20's sweep and nothing wider, performed.**
///
/// That clause bounds the sweep to the locations the phase record names, and
/// **the bound is not re-decided here**: the list of paths comes from
/// `smoozy_runtime::package::retry::what_the_sweep_may_reach`, which is the one
/// place that judgement lives. There is no folder walk and no widening
/// argument.
///
/// **A path that is not there is swept**, because the sweep's purpose is that
/// nothing of an abandoned attempt is left rather than that something was
/// removed.
pub async fn sweep_the_abandoned_staging<S>(
    subsystem: &mut crate::subsystem::client::Subsystem<S>,
    record: &smoozy_runtime::package::phase::PhaseRecord,
) -> Result<(), crate::subsystem::SubsystemFailure>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    for path in smoozy_runtime::package::retry::what_the_sweep_may_reach(record) {
        match subsystem.remove(path.as_bytes()).await {
            Ok(()) => {}
            Err(why) if why.is_the_path_not_being_there() => {}
            Err(why) => return Err(why),
        }
    }
    Ok(())
}

/// **`Rented-server runtime` G15's removal, and `Rented-server runtime` G5's at
/// a fresh replacement's first act — the same act, performed.**
///
/// **It is not the sweep**, and the two have separate names for the reason the
/// rules half gives them separate functions: at the two phases the sweep runs
/// at, the previous package is the one the machine is running. **Nothing where
/// there is no previous version.**
pub async fn remove_the_previous_package<S>(
    subsystem: &mut crate::subsystem::client::Subsystem<S>,
    record: &smoozy_runtime::package::phase::PhaseRecord,
) -> Result<(), crate::subsystem::SubsystemFailure>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    for path in
        smoozy_runtime::package::retry::what_a_removal_of_the_previous_version_reaches(record)
    {
        match subsystem.remove(path.as_bytes()).await {
            Ok(()) => {}
            Err(why) if why.is_the_path_not_being_there() => {}
            Err(why) => return Err(why),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_six_acts_are_in_the_plans_own_order_and_the_claim_is_first() {
        // **The set is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so an act added to the
        // enum has none and whoever added it is sent to this list.
        for (at, act) in AnActOfThePlacement::IN_ORDER.iter().enumerate() {
            let where_the_list_puts_it = match act {
                AnActOfThePlacement::TheClaimIsTaken => 0,
                AnActOfThePlacement::TheFreshAttemptsFirstWriteIsMade => 1,
                AnActOfThePlacement::TheArtifactIsStaged => 2,
                AnActOfThePlacement::TheStagedArtifactIsReadBackAndMadeExecutable => 3,
                AnActOfThePlacement::TheRegistrationIsPlacedAndConfirmed => 4,
                AnActOfThePlacement::TheCommitIsPerformed => 5,
            };
            assert_eq!(at, where_the_list_puts_it, "{act:?} is not where the list puts it");
        }
    }

    #[test]
    fn one_area_written_two_ways_gives_one_set_of_paths() {
        assert_eq!(
            the_package_area_as_text(b"/opt/smoozy/package/"),
            the_package_area_as_text(b"/opt/smoozy/package")
        );
    }
}

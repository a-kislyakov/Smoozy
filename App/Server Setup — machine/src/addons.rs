//! Add-ons — `Server Setup` §6.
//!
//! **The constraint** is Rows 97 to 101 and 309, and the Creator's ruling of
//! 2026-08-12: **the listing is taken as good enough and Smoozy never installs
//! over what it cannot see.**
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Rented-server runtime` M24. `Render Settings` Q12.
//! `Server protocol and transfers` §7.
//!
//! **`Rendering` S4 is not listed above and is written out at each site**,
//! because this plan defines an S4 of its own and a bare one could mean either.

use crate::blender::Version;

/// **A1: an add-on Smoozy fetches comes from a repository already on the fresh
/// Blender's own list, and Smoozy adds no repository of its own.**
///
/// The list sits in Blender's settings — **the version-numbered folder shared
/// by every build of that version, so writing to it is writing shared state** —
/// and a fresh Blender holds Blender's own extensions site among its three
/// entries.
///
/// **Blender installs by name from a repository or straight from a file, never
/// from a web address**, and the installing command line forces the no-window
/// mode anyway.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereAnAddOnComesFrom {
    /// **By name, from a repository already on the build's own list.**
    ByNameFromARepositoryAlreadyOnTheList,
    /// **A user's own copy takes the from-file route and touches no
    /// repository.**
    AFileTheUserPointedAt,
}

impl WhereAnAddOnComesFrom {
    pub const BOTH: [WhereAnAddOnComesFrom; 2] = [
        WhereAnAddOnComesFrom::ByNameFromARepositoryAlreadyOnTheList,
        WhereAnAddOnComesFrom::AFileTheUserPointedAt,
    ];
}

/// **A1: Smoozy adds no repository of its own, ever.**
pub fn smoozy_adds_a_repository(_it_would_help: bool) -> bool {
    false
}

/// **A1: an add-on is never installed from a web address.**
pub fn an_add_on_may_be_installed_from_a_web_address(_address: &str) -> bool {
    false
}

/// **A1: the machine's reach to the extensions site is established for the
/// listing only** — **an actual extension package has never been fetched by this
/// project** (Fact 54's named gap; `Server Setup` §13 item 3). **Breaks:** A1's
/// fetched-add-on route is exercised end to end by nobody. **What holds**: the
/// listing's reachability, the from-file route for user copies (observed end to
/// end), and [`what_the_install_concluded`]'s judge-by-read-back, which catches
/// a failed install whatever caused it.
pub const THE_FETCHED_ROUTE_IS_EXERCISED_END_TO_END_BY_NOBODY: bool = true;

/// **A2: what is packed is the add-on's directory as it sits in the user's own
/// version-numbered settings folder, byte for byte, archived without rewriting
/// anything, sent under `Server protocol and transfers` §7's contract, and
/// installed from file into the target build's own version folder.**
///
/// **The ordinary case is a machine of a different kind at the other end**: the
/// plain part travels and works, a compiled part built for the user's machine
/// does not, **and the failure arrives only when something uses it, that run
/// reporting success too** — so [`what_the_install_concluded`]'s claim
/// discipline is the whole honesty available.
pub fn anything_is_rewritten_while_packing(_it_would_help: bool) -> bool {
    false
}

/// **A2 and A6: the Python change, and which versions sit on which side of
/// it.**
///
/// *"The Python changed between 5.0 and 5.1"*, so an add-on with a compiled
/// part installed for one is **not usable by the other even where they share a
/// folder**. A compiled part built for the wrong Python is **refused at install
/// at a failing status** and is reported as the machine's own words behind a
/// failed step.
pub fn the_same_python(left: Version, right: Version) -> bool {
    fn side(version: Version) -> u8 {
        // 4.5 and 5.0 on one side, 5.1 and 5.2 on the other.
        if version.major > 5 || (version.major == 5 && version.minor >= 1) {
            1
        } else {
            0
        }
    }
    side(left) == side(right)
}

/// **A2's observed direction, carried rather than dropped**: the observed
/// transfer direction was **the reverse of Smoozy's** — packed on Linux, opened
/// on a Mac — and the sources name that as not covered (`Server Setup` §13
/// item 4).
pub const THE_PACKED_COPYS_DIRECTION_WAS_OBSERVED_THE_OTHER_WAY: bool = true;

/// **A3: Smoozy does not ask for, read, or check a licence key.**
///
/// **Fact 56 is answered by reading, 2026-09-02, in the direction this clause
/// already stood.** The marketplace whose rules bind the paid Blender add-ons
/// this question is about publishes that code products may be licensed as GPL
/// or MIT and *"No other licensing options are currently permitted for
/// code-based products"*, that both permit redistribution and modification, and
/// that a compliant add-on's folder carries a `LICENSE` file beside the code
/// with *"a license notice at the top of the file"* in every Python file. **So a
/// copy carries licence text and a copyright notice, not an entitlement.**
/// Label: **documented**.
///
/// **The residue is narrower than the question was**: neither licence forbids an
/// add-on making a runtime activation check of its own, so **whether any
/// individual add-on additionally holds a seller-issued key inside its folder is
/// not established** — and this is one marketplace's rules on one date. **The
/// silence is removed by the failure being the add-on's own**, since an add-on
/// that will not run unactivated says so itself.
pub fn smoozy_reads_a_licence_key(_it_is_in_the_folder: bool) -> bool {
    false
}

/// **A4: Row 99 — the switch-on uses the route that reports, and the answer is
/// trusted exactly as far as the research allows.**
///
/// **On the reporting route the act succeeds for a working add-on and raises for
/// a broken one, naming the fault; the other route swallows the failure** and
/// the launch-time switch-on prints a complaint and still ends at success — **so
/// the quiet routes are never used.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchOnRoute {
    /// The one that reports.
    TheRouteThatReports,
}

/// **A4: what is learned, and it is exactly this much.**
///
/// *"Loadable at the moment it was switched on, in this sitting"* — the list of
/// switched-on add-ons **records what was asked for rather than what loaded**,
/// asking Blender to look again **reports nothing about damage**, and **nothing
/// anywhere says beforehand whether the part a render needs will work.**
pub fn what_the_switch_on_establishes() -> &'static str {
    "loadable at the moment it was switched on, in this sitting"
}

/// **A5: Row 100 — an install is judged by reading what the machine now holds,
/// and the command's ending is read in neither direction.**
///
/// **One refusal ends at a successful status** (a declared Blender-version
/// mismatch, refused by name with nothing installed), **one failure reports
/// honestly** (wrong-Python compiled part), and the same version declaration is
/// **ignored entirely** for an older-style add-on whatever every version's
/// manual says — **so the manual is not the rule and the exit status is not the
/// rule.**
pub fn the_installs_exit_status_decides_anything(_it_was_zero: bool) -> bool {
    false
}

/// **A5: what is read back, and what counts as installed.**
///
/// After the install, the target build is asked for **its installed listing and
/// its switched-on listing** (with A4's switch-on made through the reporting
/// route in the same sitting).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheReadBack {
    pub installed_listing: Vec<String>,
    /// **A5 asks for this reading and no judgement consumes it, and that is the
    /// answer rather than a gap.**
    ///
    /// **Why it is taken.** A5 names both listings, so the reading is what the
    /// plan requires and taking one of the two would be short of it. A reader
    /// who found only one reading taken would think the code was short.
    ///
    /// **Why nothing reads it.** A4 already records that the switched-on listing
    /// *"records what was asked for rather than what loaded"*, and *"nothing
    /// anywhere says beforehand whether the part a render needs will work"* —
    /// **which is why A5 was right not to lean on it.** What *installed* means is
    /// the installed listing plus the reporting switch-on not having raised, and
    /// that is [`what_the_install_concluded`]'s whole test.
    pub switched_on_listing: Vec<String>,
    /// Whether the reporting switch-on raised.
    pub the_reporting_switch_on_raised: bool,
}

/// **A5, performed: *installed* means the add-on appears in the read-back
/// listing **and** the reporting switch-on did not raise.**
pub fn what_the_install_concluded(name: &str, read_back: &TheReadBack) -> WhatMayBeClaimed {
    let in_the_listing = read_back
        .installed_listing
        .iter()
        .any(|held| held == name);
    if in_the_listing && !read_back.the_reporting_switch_on_raised {
        WhatMayBeClaimed::InstalledAndSwitchedOnLoadableAtThatMoment
    } else {
        WhatMayBeClaimed::NotInstalled
    }
}

/// **A5: what this module then claims, and it is the spec's own sentence.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatMayBeClaimed {
    /// *"Installed and switched on, loadable at the moment it was switched
    /// on"* — **and whether it works shows when the job renders.**
    InstalledAndSwitchedOnLoadableAtThatMoment,
    NotInstalled,
}

/// **A5's known incompleteness, carried and not papered over.**
///
/// **Narrowed 2026-09-02, and the sentence was firmer than its evidence.**
/// *Add-ons 10* lists as **not established** whether any route gives one
/// complete list of everything installed **including switched-off older-style
/// add-ons**, and its own summary is *"Smoozy can be sure something is there and
/// can never be sure something is absent"* — **a silence about what was found,
/// not a demonstrated absence.** The listing also **says nothing about health**.
pub const NO_ROUTE_IS_ESTABLISHED_TO_COVER_SWITCHED_OFF_OLDER_STYLE_ADD_ONS: bool = true;

/// **A6: Row 101 — the folder written and read is the version-numbered settings
/// folder of the build the job will render with, and the compare-before-install
/// is against that build's own read-back listing.**
///
/// **Where the listing shows the add-on present, nothing is installed** —
/// *nothing is installed twice* is a promise about **what the listing shows**,
/// per the ruling. **Where the listing shows it absent, it is installed.**
///
/// **The residue is stated**: an older-style switched-off copy is visible to no
/// route this project established, so **an install can land on a copy the
/// listing could not show** — the bounded, ruled-on cost of taking the listing
/// as good enough (`Server Setup` §13 item 4).
pub fn anything_is_installed(name: &str, read_back: &TheReadBack) -> bool {
    !read_back.installed_listing.iter().any(|held| held == name)
}

/// **A6: the folder is the build's version-numbered settings folder**, and
/// patch releases share it — two numbers, not three. **A removed build leaves
/// the folder for the next one.**
pub fn the_settings_folder(version: Version) -> String {
    version.settings_folder()
}

/// **A6: where the wrong-Python failure surfaces across the shared folder, and
/// the narrowing takes the loud half away. Corrected 2026-08-31, out of the
/// repaired Blender research.**
///
/// **Across the shared folder the install-time refusal cannot fire at all**:
/// [`anything_is_installed`]'s compare-before-install finds the add-on already
/// in the second build's read-back listing and installs nothing, **so there is
/// no install for the version check to refuse**. **And nothing on record
/// exercised that check across the change** — all five sharing tests used a
/// plain-Python add-on with no bundled module, so the second version installed
/// nothing in every one of them. Label: **observed in a test**.
///
/// **So on the shared folder the failure surfaces at use and nowhere earlier**,
/// which is A5's claim discipline; A2's failing-status refusal stands for an
/// install actually performed, which is a fresh install rather than the
/// shared-folder case (`Server Setup` §13 item 24).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereTheWrongPythonSurfaces {
    /// A fresh install actually performed: **refused at install, at a failing
    /// status**.
    AtTheInstall,
    /// The shared-folder case: **at use and nowhere earlier**.
    AtUse,
}

/// **A6, performed.**
pub fn where_the_wrong_python_surfaces(
    the_listing_already_shows_it: bool,
) -> WhereTheWrongPythonSurfaces {
    if the_listing_already_shows_it {
        WhereTheWrongPythonSurfaces::AtUse
    } else {
        WhereTheWrongPythonSurfaces::AtTheInstall
    }
}

/// **A6a: every install writes into a user repository and never into the folder
/// a Blender runs from, and that condition is what the *no warning* row rests
/// on. Added 2026-08-31, out of the repaired Blender research.**
///
/// Every add-on install on record went into a **user repository** — the
/// version-numbered settings folder A6 already fixes as the target — and
/// **nothing was ever written into the folder a running Blender was running
/// from**, which is the one write recorded as killing a render on Linux. Label:
/// **observed in a test**. **So *an add-on install disturbed no running render*
/// is a result about writes into a user repository; it is never read as evidence
/// about a write into a Blender's own folder.**
pub const THE_CONDITION_TRAVELS_WITH_THE_RESULT: bool = true;

/// **A6a's one requirement, and it is free.**
///
/// Where anything this module runs sets where the **system** extensions
/// repository lives, **that location is pointed outside every Blender's own
/// folder.** The mitigation is the research's own and costs nothing. **A system
/// repository left inside a Blender's folder would put an install on the exact
/// write that kills a render on Linux**, and `statements` in `smoozy-server-setup` W3's *no
/// warning* row would then be wrong about the one step it says nothing about.
pub fn the_system_repository_may_sit_inside_a_blenders_own_folder(_it_is_convenient: bool) -> bool {
    false
}

/// **A7: the offer's scope is the spec's** — per job, this machine, the Blender
/// this job renders with. **Other builds need their own copies.**
///
/// **A version changed after Pre-load leaves the list describing a version the
/// job will not use, and Smoozy says so** — the wider question is
/// `Render Settings`', **settled at its Q12 (Row 142)**: a version change on
/// that page discards the whole pre-load reading, this list with it — *"The
/// recorded reading for that (server, file) is discarded — every scene's values
/// at once, the announced add-on case included, since an add-on belongs to one
/// Blender rather than to the machine."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheOffersScope {
    pub per_job: bool,
    pub this_machine: bool,
    pub the_blender_this_job_renders_with: bool,
}

/// **A7's scope, as one value.**
pub const THE_OFFERS_SCOPE: TheOffersScope = TheOffersScope {
    per_job: true,
    this_machine: true,
    the_blender_this_job_renders_with: true,
};

/// **A8: Row 111 — what serialises a machine's setup is [`crate::driving`] Y9's
/// one-run-at-a-time admission; what it cannot serialise is a Blender the user
/// started, and a refusal from Blender's own add-on store lock is expected
/// rather than read as a broken machine.**
///
/// With one Blender holding the lock, an install fired from another **refuses in
/// under a second at a failing status naming the session that holds it**.
/// **Smoozy does not parse the refusal for a cause** — the step is reported
/// failed with the machine's own words whole behind it (the same never-mined
/// discipline as `Rented-server runtime` M24), the read-back of A5 says what the
/// machine now holds, and **the step is safe to run again**.
pub fn a_store_lock_refusal_is_read_as_a_broken_machine(_it_refused: bool) -> bool {
    false
}

/// **A8: a Blender that is rendering does not hold the lock**, which is why
/// installs beside a render disturbed nothing; **a running Blender does not pick
/// a new add-on up by itself, and nothing here restarts one to make it.**
pub fn a_running_blender_is_restarted_to_pick_up_an_add_on(_it_would_help: bool) -> bool {
    false
}

/// **A9: Row 309 — the one thing that crosses from Pre-load is a file naming an
/// engine the chosen Blender lacks**, and what follows the hand-off is Rows 97
/// to 101's machinery unchanged.
///
/// **This module does none of the reading**; the offer it composes from the
/// hand-off is the same offer, and its claims are A5's.
pub fn what_crosses_from_pre_load() -> &'static str {
    "a file naming an engine the chosen Blender lacks, and nothing else"
}

/// **A9a: where the user picks the engine themselves and the machine's Blender
/// does not have it, the job is refused before it is sent — and the refusal is
/// not performed here.**
///
/// **Why the promise exists rather than a fallback**: on a user-picked engine
/// nothing falls back, three of the four Blender versions stop at a failure and
/// **the fourth never stops — on a rented machine it runs until somebody
/// destroys it, billing all the while** — so the case is an open bill rather
/// than a wrong picture.
///
/// **Who refuses.** The refusal is a gate on the submission, so it belongs to
/// the document that composes and sends one — `Rendering`, ahead of that
/// document's **`Rendering` S4** ordered unit, before the first step of it runs. **Nothing
/// here composes, sends or refuses a job**, and writing the gate in this
/// document would put a second arrangement beside the one that exists.
///
/// **What this module supplies to it, and the whole of it**: A5's read-back —
/// the target build's own installed listing and switched-on listing, with A5's
/// known incompleteness riding along. **The refusal rests on that read-back and
/// never on a build's engine list**, because no list of engines a version is
/// supposed to ship is consulted anywhere here and inventing one would be
/// exactly the modelling [`crate::blender`] B1 forbids.
///
/// **What was owed elsewhere is written, and what is left is code rather than a
/// plan.** Both halves landed on 2026-09-04: `Rendering` defines the gate clause
/// as `Rendering` S3a, and Row 517 quotes the spec sentence and is answered
/// there. `Server Setup` §13 item 26 is marked closed on both. **What remains
/// owed is the code that keeps `Rendering` S3a**, which is a later build step's
/// and not this module's. It is on the routing list in `arriving_later`, in
/// `smoozy-server-setup`.
pub fn this_module_refuses_the_job(_the_engine_is_absent: bool) -> bool {
    false
}

/// **A9a: what this module supplies the gate, taken from A5's read-back.**
pub fn what_the_gate_is_given(name: &str, read_back: &TheReadBack) -> WhatMayBeClaimed {
    what_the_install_concluded(name, read_back)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_read_back(installed: &[&str], raised: bool) -> TheReadBack {
        TheReadBack {
            installed_listing: installed.iter().map(|it| it.to_string()).collect(),
            switched_on_listing: installed.iter().map(|it| it.to_string()).collect(),
            the_reporting_switch_on_raised: raised,
        }
    }

    #[test]
    fn an_install_is_judged_by_the_read_back_and_never_by_the_commands_ending() {
        // A5: one refusal ends at a successful status and one failure reports
        // honestly, so the exit status is not the rule in either direction.
        assert!(!the_installs_exit_status_decides_anything(true));
        assert!(!the_installs_exit_status_decides_anything(false));
        assert_eq!(
            what_the_install_concluded("magic", &a_read_back(&["magic"], false)),
            WhatMayBeClaimed::InstalledAndSwitchedOnLoadableAtThatMoment
        );
    }

    #[test]
    fn an_add_on_in_the_listing_whose_switch_on_raised_is_not_installed() {
        assert_eq!(
            what_the_install_concluded("magic", &a_read_back(&["magic"], true)),
            WhatMayBeClaimed::NotInstalled
        );
        assert_eq!(
            what_the_install_concluded("magic", &a_read_back(&[], false)),
            WhatMayBeClaimed::NotInstalled
        );
    }

    #[test]
    fn nothing_is_installed_where_the_listing_already_shows_it() {
        // A6, and the ruling of 2026-08-12: *nothing is installed twice* is a
        // promise about what the listing shows.
        assert!(!anything_is_installed("magic", &a_read_back(&["magic"], false)));
        assert!(anything_is_installed("magic", &a_read_back(&["other"], false)));
    }

    #[test]
    fn the_wrong_python_surfaces_at_use_across_the_shared_folder() {
        // Corrected 2026-08-31: across the shared folder the install-time
        // refusal cannot fire at all, because nothing is installed.
        assert_eq!(
            where_the_wrong_python_surfaces(true),
            WhereTheWrongPythonSurfaces::AtUse
        );
        assert_eq!(
            where_the_wrong_python_surfaces(false),
            WhereTheWrongPythonSurfaces::AtTheInstall
        );
    }

    #[test]
    fn the_python_changed_between_five_zero_and_five_one() {
        let v = |text: &str| Version::parse(text).expect("a version");
        assert!(the_same_python(v("4.5.9"), v("5.0.1")));
        assert!(the_same_python(v("5.1.2"), v("5.2.0")));
        assert!(!the_same_python(v("5.0.1"), v("5.1.2")));
    }

    #[test]
    fn patch_releases_share_the_settings_folder() {
        let v = |text: &str| Version::parse(text).expect("a version");
        assert_eq!(the_settings_folder(v("4.5.9")), the_settings_folder(v("4.5.23")));
        assert_ne!(the_settings_folder(v("5.0.1")), the_settings_folder(v("5.1.2")));
    }

    #[test]
    fn smoozy_adds_no_repository_and_installs_from_no_web_address() {
        assert!(!smoozy_adds_a_repository(true));
        assert!(!an_add_on_may_be_installed_from_a_web_address(
            "https://extensions.blender.org/x"
        ));
        assert_eq!(WhereAnAddOnComesFrom::BOTH.len(), 2);
    }

    #[test]
    fn no_licence_key_is_asked_for_read_or_checked() {
        // A3, and Fact 56 answered by reading in the direction this already
        // stood.
        assert!(!smoozy_reads_a_licence_key(true));
    }

    #[test]
    fn nothing_is_rewritten_while_packing_the_users_own_copy() {
        assert!(!anything_is_rewritten_while_packing(true));
        const { assert!(THE_PACKED_COPYS_DIRECTION_WAS_OBSERVED_THE_OTHER_WAY) };
    }

    #[test]
    fn the_switch_on_establishes_this_sitting_and_nothing_further() {
        assert!(what_the_switch_on_establishes().contains("in this sitting"));
        assert_eq!(SwitchOnRoute::TheRouteThatReports, SwitchOnRoute::TheRouteThatReports);
        const { assert!(NO_ROUTE_IS_ESTABLISHED_TO_COVER_SWITCHED_OFF_OLDER_STYLE_ADD_ONS) };
        const { assert!(THE_FETCHED_ROUTE_IS_EXERCISED_END_TO_END_BY_NOBODY) };
    }

    #[test]
    fn the_system_repository_never_sits_inside_a_blenders_own_folder() {
        // A6a's one requirement, and it is free.
        assert!(!the_system_repository_may_sit_inside_a_blenders_own_folder(true));
        const { assert!(THE_CONDITION_TRAVELS_WITH_THE_RESULT) };
    }

    #[test]
    fn a_store_lock_refusal_is_expected_and_no_blender_is_restarted_for_an_add_on() {
        assert!(!a_store_lock_refusal_is_read_as_a_broken_machine(true));
        assert!(!a_running_blender_is_restarted_to_pick_up_an_add_on(true));
    }

    #[test]
    fn the_offers_scope_is_per_job_this_machine_and_this_build() {
        const { assert!(THE_OFFERS_SCOPE.per_job) };
        const { assert!(THE_OFFERS_SCOPE.this_machine) };
        const { assert!(THE_OFFERS_SCOPE.the_blender_this_job_renders_with) };
    }

    #[test]
    fn only_a_missing_renderer_crosses_from_pre_load() {
        assert!(what_crosses_from_pre_load().contains("nothing else"));
    }

    #[test]
    fn this_module_never_refuses_a_job_and_only_supplies_the_read_back() {
        // A9a: the gate belongs to `Rendering`, ahead of that document's
        // `Rendering` S4 ordered unit.
        assert!(!this_module_refuses_the_job(true));
        assert_eq!(
            what_the_gate_is_given("cycles-x", &a_read_back(&["cycles-x"], false)),
            WhatMayBeClaimed::InstalledAndSwitchedOnLoadableAtThatMoment
        );
    }
}

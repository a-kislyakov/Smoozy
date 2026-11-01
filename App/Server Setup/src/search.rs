//! The search, and taking a Blender off — `Server Setup` §5.4 and §5.5.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Queue & History` §17, N10, N12. `Rendering` L14.
//! `File Browser` N1, P3, §12. `State, identity and recovery` D4.
//!
//! **`Job files and outputs` M1 and M1a are not listed above**, because this plan
//! defines an M1 and an M1a of its own and a bare one could mean either;
//! where this file cites one, the document is written out at the site.

use std::time::{Duration, SystemTime};

use crate::blender::Version;

/// **B10: Row 103 — the search is a whole-disk file search for Blender's
/// executable name, crossing filesystems, plus one asking-run per candidate
/// under [`crate::blender`] B1's discipline.**
pub const THE_EXECUTABLES_NAME: &str = "blender";

/// **B10: the search is never restricted to one filesystem**, which silently
/// loses a snap mounted from a loop device.
pub fn the_search_is_restricted_to_one_filesystem() -> bool {
    false
}

/// **B10: one path the search returns is not a candidate — Smoozy's own Blender
/// area, at the exact path B9 composes.**
///
/// **The area is named `blender` and the search is by name, so on every machine
/// this module has prepared the search finds it.** **Observed in a controlled
/// test** on Hyperstack `Debian 12 Plain`, 2026-09-04: the whole-disk search
/// returned five paths, three of them directories, and one of those was the area
/// this module's own placement had made. Left in, it costs an asking-run that
/// cannot answer and puts Smoozy's own storage folder in front of the user on
/// B11's did-not-answer list, named as a Blender that would not say what it was.
///
/// **What is dropped is that one exact path and nothing else.** Not a prefix,
/// not a rule about directories: **builds Smoozy placed live inside the area**,
/// each at its own deeper path, and dropping the area itself is what keeps them.
///
/// **This rests on a decision and not on a fact about the world.** Smoozy
/// composes that path ([`crate::placement::the_blender_area`]) and a build is
/// never at it, so nothing outside Smoozy has to behave in any particular way
/// for this to hold. **Whether any other path the search returns is a directory
/// is not established here and is not decided here** — such a path stays a
/// candidate, is asked, does not answer, and B11 names it, which is what the
/// removal of the file-kind restriction chose deliberately.
///
/// **Nothing is dropped where no area is known**, which is a machine this module
/// has not placed a root on.
pub fn the_candidates(found: Vec<String>, smoozys_own_blender_area: Option<&str>) -> Vec<String> {
    let Some(area) = smoozys_own_blender_area else {
        return found;
    };
    found.into_iter().filter(|path| path != area).collect()
}

/// One candidate the search found, with what it answered.
///
/// **B10's second trap: deduplication is never by resolved path** — a snap's
/// path resolves to the snap launcher — **so the path-and-answer pair is the
/// entry and the version each path reports is the discriminator.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACandidate {
    /// The executable's own path, as the search found it. **Never resolved.**
    pub path: String,
    /// What it answered when asked, or nothing where it did not answer.
    pub answered: Option<Version>,
}

/// **B10, performed: the entries the search produces from what it found.**
///
/// **The version is parsed from the `Blender ` line and never by position**
/// (Fact 45's first trap), which is [`crate::blender`]'s reading and is not
/// retaken here.
///
/// **Nothing is filtered out here, deliberately.** [`what_the_answer_says`]
/// reads this same set to compose B11's did-not-answer list, so a candidate
/// dropped here would be dropped from the naming as well — **and B11's two
/// halves would come apart, the machine falling silent about which candidates
/// did not answer.** The half that must not list them is
/// [`the_listing`].
pub fn the_entries(candidates: Vec<ACandidate>) -> Vec<ACandidate> {
    let mut entries: Vec<ACandidate> = Vec::new();
    for candidate in candidates {
        // The pair is the entry: two paths that resolve to one launcher are two
        // entries, and two answers at one path are one.
        if entries.iter().any(|held| held.path == candidate.path) {
            continue;
        }
        entries.push(candidate);
    }
    entries
}

/// **B11: a candidate killed at its time limit is not listed** — *"and is named
/// as one that did not answer"*.
///
/// **The filter is here and never in [`the_entries`]**, which is the same slice
/// [`what_the_answer_says`] composes the naming half from: filtering there would
/// take B11's other half with it. **The naming keeps reading the unfiltered
/// set.**
///
/// **Until 2026-09-04 nothing filtered anywhere**, so an unanswering binary
/// reached the offered list — the naming half was built and the listing half was
/// not.
///
/// **The tie to B1's bound is worth naming.** Until the version ask was
/// bounded, nothing was ever killed at a limit, so a candidate answering nothing
/// answered nothing for some other reason; the rule is the same either way and
/// this is the site that keeps it.
pub fn the_listing(entries: &[ACandidate]) -> Vec<ACandidate> {
    entries
        .iter()
        .filter(|entry| entry.answered.is_some())
        .cloned()
        .collect()
}

/// **B10's observed cost**: 5.3 seconds on a 126 GB disk 12% full, **plus a run
/// per candidate**.
pub const THE_OBSERVED_SEARCH_COST: Duration = Duration::from_millis(5_300);

/// **B11: what the search honestly cannot do is kept in its answer.**
///
/// **Since 2026-10-02 only the last of the three is drawn** — the Creator's
/// note 13 on 0.1.7, *"All this should just go — it's unnecessary noise."* The
/// by-name mark and the refusal count stay here and in the diary, and nothing
/// on a screen reads them (`Render Settings`' header and Set up's table).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheAnswerSays {
    /// **A renamed binary defeats the file search** and is found only by
    /// running every file, which is not done — **a build the user renamed is
    /// invisible and the answer says the search is by name.**
    pub the_search_is_by_name: bool,
    /// **A folder the login may not read is skipped and counted**; where
    /// refusals occurred, the answer says **the machine could not be searched
    /// completely**. A search run as root could not meet a refusal at all,
    /// **which is why the count exists**.
    pub folders_refused: usize,
    /// **A candidate killed at its time limit is not listed and is named as one
    /// that did not answer** ([`crate::blender`] B1).
    pub did_not_answer: Vec<String>,
}

/// **B11, performed.**
pub fn what_the_answer_says(entries: &[ACandidate], folders_refused: usize) -> WhatTheAnswerSays {
    WhatTheAnswerSays {
        the_search_is_by_name: true,
        folders_refused,
        did_not_answer: entries
            .iter()
            .filter(|entry| entry.answered.is_none())
            .map(|entry| entry.path.clone())
            .collect(),
    }
}

/// **B11: the machine could not be searched completely where any folder was
/// refused.** *(The elevated retry this used to open under [`crate::elevation`]
/// E2's flow was withdrawn by B11 on 2026-10-02: the count is not drawn, so no
/// line offers it.)*
pub fn the_machine_was_searched_completely(answer: &WhatTheAnswerSays) -> bool {
    answer.folders_refused == 0
}

/// **B12: Row 104 — the search runs on demand and its result is period-held,
/// never durable.**
///
/// **This is `State, identity and recovery` D4 applied, not an exception
/// invented**: the reading is taken fresh at the moment of use, carries its
/// moment, and **nothing durable is written from it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenTheSearchRuns {
    /// When Server Setup's own view is opened.
    TheModulesOwnViewIsOpened,
    /// When `Render Settings` asks for the version list.
    TheVersionListIsAskedFor,
    /// Before a removal is offered.
    BeforeARemovalIsOffered,
}

impl WhenTheSearchRuns {
    /// **The three moments, and never on connecting** — it costs a whole-disk
    /// pass and a run per build on a machine billed by the hour, and nothing
    /// needs it until one of these.
    pub const THE_THREE: [WhenTheSearchRuns; 3] = [
        WhenTheSearchRuns::TheModulesOwnViewIsOpened,
        WhenTheSearchRuns::TheVersionListIsAskedFor,
        WhenTheSearchRuns::BeforeARemovalIsOffered,
    ];
}

/// **B12: the reading carries its own moment and the lifetime is period-held.**
pub const THE_SEARCHES_LIFETIME: smoozy_state::lifetimes::Lifetime =
    smoozy_state::lifetimes::Lifetime::PeriodHeld;

/// **B12: it never runs on connecting.**
pub fn the_search_runs_on_connecting() -> bool {
    false
}

/// One search result, with the moment it was taken.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSearchResult {
    pub entries: Vec<ACandidate>,
    pub answer: WhatTheAnswerSays,
    /// The moment on the machine's own clock, carried per `State, identity and
    /// recovery` D4.
    pub at: SystemTime,
}

/// **B12a: the folder the Blender table shows, and the link it carries into
/// `File Browser`.**
///
/// **The value is per build, and it comes from the reading that drew the row.**
/// [`the_entries`] returns a path and an answered version per candidate, and
/// [`the_matching_rule`] makes the executable's path the build's identifier —
/// **so the folder is the directory containing that path, derived at the moment
/// the table is drawn and never remembered.**
///
/// **Never per version**: two builds answering one version have two paths and
/// two folders.
pub fn the_folder_for(path: &str) -> &str {
    match path.rfind('/') {
        Some(0) => "/",
        Some(at) => &path[..at],
        None => ".",
    }
}

/// **B12a: what crosses into `File Browser`, and no machine is chosen.**
///
/// `File Browser` is the same server's other tab, so the server row is the one
/// whose screen this is; **the path is the position the browser opens at**,
/// which is that document's own run-held browsing state (`File Browser` N1).
/// **Nothing is picked and nothing comes back** — this is not the entry point
/// `File Browser` §12 supplies to `Rendering`.
///
/// **So this document composes the path and claims nothing about it**: the
/// reading behind it is period-held, the folder may be gone or unreadable by the
/// time the browser asks, and what is then said is `File Browser` P3's answer
/// under that document's own separation.
pub fn what_crosses_to_the_browser(path: &str) -> &str {
    the_folder_for(path)
}

/// **B12a: the link is offered on every row all the same.**
///
/// On a build Smoozy did not place, the folder is the machine's own — a snap's
/// found path resolves to the snap launcher and the snap mount is read-only
/// squashfs, **so the column can link at a folder nothing may be written
/// into**. **Withholding it for some builds would be this document deciding
/// which folders the user may look at, which no spec grants it.**
pub fn the_link_is_withheld_on_some_rows(_it_is_read_only: bool) -> bool {
    false
}

/// **B12b: each Blender in the table carries a note where the machine's NVIDIA
/// driver is below that Blender's OptiX bar.** Added 2026-10-11, the fourth
/// piece of the OptiX and driver work.
///
/// **The bar is `Rendering` L23a's, for the version the row's build answered**,
/// and the driver's version is the reading `Servers` H11b holds for this
/// connection. **Both are read, not remembered**, like the rest of the row
/// (B12): this is handed the reading the table is drawn beside and keeps
/// nothing. **The comparison is not made here a second time** — it is
/// `Servers` H11b's own judgement (`smoozy_servers::hardware::what_the_bar_says`),
/// and this only says which of the builds the table holds it falls on.
///
/// **Where no NVIDIA driver answers, no row carries the note**, so the caller
/// passes no driver and nothing comes back. **Whatever the machine kind**: on a
/// machine Smoozy can manage the note stands while `Server Setup` R1b's swap
/// waits, and it goes when the next reading finds the driver at the bar.
pub fn the_builds_the_optix_note_falls_on(
    the_driver: Option<&smoozy_servers::hardware::TheDriversOwnAnswer>,
    builds: &[Version],
) -> Vec<Version> {
    let Some(the_driver) = the_driver else {
        return Vec::new();
    };
    let numbers: Vec<smoozy_servers::hardware::BlenderNumbers> = builds
        .iter()
        .map(|version| (version.major, version.minor, version.patch))
        .collect();
    // **The third input is the statement's and plays no part in the note**,
    // which is drawn whatever the machine kind and whatever a swap came to.
    let below = smoozy_servers::hardware::what_the_bar_says(the_driver, &numbers, false)
        .blenders_below_their_bar;
    builds
        .iter()
        .copied()
        .filter(|version| below.contains(&(version.major, version.minor, version.patch)))
        .collect()
}

/// **B13: the matching rule Row 96 needs, and nobody else can supply — a
/// Blender on the machine is identified by its executable's path; a job's
/// recorded need carries that path beside the version.**
///
/// **A version number does not identify a build** — two builds of one version
/// share a settings folder and both answer the same version — so the submission
/// payload records the picked build's **path and its answered version**, the
/// manifest carries both (`Job files and outputs` M1,
/// `Job files and outputs` M1a), and the launch uses
/// the path with `Rendering` L14's version confirmation guarding substitution.
///
/// **`Rendering` L14 and `Queue & History` N10 both stand unchanged**: the
/// version is on the manifest and the local record exactly as N10 says, and L14
/// still confirms the version by asking the binary. **What is new is the path
/// beside it.**
pub fn the_matching_rule() -> &'static str {
    "a Blender on the machine is identified by its executable's path, and a job's recorded need carries that path beside the version"
}

/// **B13, as the question a builder would otherwise answer wrongly.**
pub fn a_version_identifies_a_build(_version: Version) -> bool {
    false
}

/// **B14: what a live job is, in `Queue & History` §17's terms.**
///
/// *"A job is live from its record's publication until its recorded outcome is
/// written, held jobs included; the live set per machine is the job records with
/// state waiting, held or rendering."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveState {
    Waiting,
    Held,
    Rendering,
}

/// One live job, as the warning reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ALiveJob {
    pub state: LiveState,
    /// **The recorded build path** — B13's identifier, and what the warning is
    /// composed from. **Never the version.**
    pub recorded_build_path: String,
}

/// **B15: the two removal routes, and the rule is the same for both while the
/// reason is not.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovalRoute {
    /// **The plain deletion of the install's folder.**
    TheFolderDeletion,
    /// **A build the machine's own software manager installed, removed through
    /// that manager.** A path under the system package areas is the manager's;
    /// **deleting its files by hand leaves the manager's records lying about the
    /// machine**, which is exactly the kind of stale reading Row 104 exists to
    /// forbid. **A path under the snap mount cannot be deleted by hand at all**
    /// — Canonical's own *System snap directory*, read 2026-08-20, gives the
    /// mount as `squashfs (ro,nodev,relatime,x-gdu.hide)`, and a read-only mount
    /// refuses the deletion rather than leaving records behind.
    TheMachinesOwnSoftwareManager,
}

/// **B14: the warning names every live job whose recorded build path is the
/// path of the build being removed, or sits under it where the removal is a
/// folder deletion.**
///
/// **One predicate covering both of B15's routes**, because a build the
/// machine's own software manager installed has no folder of Smoozy's to be
/// under and **its recorded path may be a launcher rather than a folder at
/// all**.
///
/// **On the manager route the build is the package, and one package can be
/// found at two paths.** B10's search finds the snap's launcher and its own
/// folder's executable alike, and the manager removes both together — so a job
/// submitted with the other path loses its Blender too, and naming only the
/// exact path would remove that one quietly. **So on that route every live job
/// whose recorded path the same package holds is named**
/// ([`crate::blender::the_managed_package_at`]). Added 2026-10-16, step 10c's
/// owed piece 6.
pub fn the_warning_names(
    live: &[ALiveJob],
    the_build_being_removed: &str,
    route: RemovalRoute,
) -> Vec<ALiveJob> {
    let the_package = crate::blender::the_managed_package_at(the_build_being_removed);
    live.iter()
        .filter(|job| {
            job.recorded_build_path == the_build_being_removed
                || match route {
                    RemovalRoute::TheFolderDeletion => job
                        .recorded_build_path
                        .starts_with(&format!("{the_build_being_removed}/")),
                    RemovalRoute::TheMachinesOwnSoftwareManager => {
                        the_package.is_some()
                            && crate::blender::the_managed_package_at(&job.recorded_build_path)
                                == the_package
                    }
                }
        })
        .cloned()
        .collect()
}

/// **B14: what the warning tells each named job about itself, per route.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheWarningSays {
    /// **On the folder-deletion route**: a job already rendering **will
    /// finish** — deleting the folder a render was running from was permitted at
    /// once and the render carried on to a normal end, all four versions,
    /// **observed on Linux and on a Mac; Windows untried and the deletion may be
    /// refused outright there**.
    ARenderingJobWillFinish,
    /// **On the manager route that is not established and is not claimed**:
    /// what a removal performed by the machine's own software manager does to a
    /// render already running from that build **was never observed**, and this
    /// document does not carry one route's observation onto the other in the
    /// sentence the user reads before deciding. Label: **not established**;
    /// `Server Setup` §13 item 17.
    ARenderingJobsFateIsNotKnownOnThisRoute,
    /// **On either route**: a waiting job was submitted with that build and
    /// **will have nothing to render with**. What becomes of it is
    /// `Queue & History`'s.
    AWaitingJobWillHaveNothingToRenderWith,
}

/// **B14, performed.**
pub fn what_the_warning_says(job: &ALiveJob, route: RemovalRoute) -> WhatTheWarningSays {
    match (job.state, route) {
        (LiveState::Rendering, RemovalRoute::TheFolderDeletion) => {
            WhatTheWarningSays::ARenderingJobWillFinish
        }
        (LiveState::Rendering, RemovalRoute::TheMachinesOwnSoftwareManager) => {
            WhatTheWarningSays::ARenderingJobsFateIsNotKnownOnThisRoute
        }
        (LiveState::Waiting | LiveState::Held, _) => {
            WhatTheWarningSays::AWaitingJobWillHaveNothingToRenderWith
        }
    }
}

/// **B14: Smoozy never refuses and never removes quietly; the last-Blender case
/// adds nothing on its own account** (the spec's rule).
pub fn a_removal_is_refused_because_it_is_the_last_blender(_it_is: bool) -> bool {
    false
}

/// **B15: the version-numbered settings folder is left behind in every case.**
///
/// It is shared by every build of that version and **holds add-ons the user may
/// have placed** (the Creator's never-install-over ruling reaches it in spirit),
/// and a build removed leaving it behind is **Blender's own documented
/// behaviour**.
pub fn the_settings_folder_is_removed_with_the_build(_route: RemovalRoute) -> bool {
    false
}

/// **B15: the manager's removal is an elevated act** and
/// [`crate::elevation`] E2's flow applies.
pub fn the_removal_is_an_elevated_act(route: RemovalRoute) -> bool {
    matches!(route, RemovalRoute::TheMachinesOwnSoftwareManager)
}

/// **B15's manager route, as a manager paired with the package name it holds
/// Blender under and the constant command that removes it.**
///
/// **The three used to be apart and could come apart.** The fixed set was one
/// bare package name, the command was a separate constant naming one manager,
/// and **nothing anywhere read which manager put a found build on the machine**
/// — so an apt-installed `/usr/bin/blender`, which Fact 45 records as a real
/// case, passed the set and got the snap command.
///
/// **Deciding a manager from a path is not done here and cannot be.** B15 names
/// the two classes — *"A path under the system package areas is the manager's"*
/// and *"A path under the snap mount cannot be deleted by hand at all"* — and
/// gives no path for either; the only thing it cites about the snap mount is
/// that it is read-only. **Writing path facts in from memory is what Rule 2
/// forbids outright**, so the manager is paired with the name rather than
/// derived from a reading nobody has.
///
/// *(Added 2026-10-16, step 10c's owed piece 6. The paragraph above is kept as
/// written. **The snap's two paths are not from memory**: both were observed —
/// the launcher in the six-machine batch's Fact 45 and in the Blender
/// research's hand test of 2026-08-06, and the snap's own folder in that same
/// test — so [`crate::blender::the_managed_package_at`] reads a found path
/// against those two shapes, and a row at either is removed through this
/// pairing. **The distribution's package still has no entry**: no plan names
/// its removal command, and its `/usr/bin/blender` cannot be told from a link
/// at that path.)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AManagerAndItsPackage {
    /// The manager, by its own name.
    pub manager: &'static str,
    /// **The package name that manager holds Blender under**, a constant fixed
    /// at build time because `Server protocol and transfers` 1.1 forbids
    /// echoing a machine-read name into a command string in either phase, and
    /// **none of that rule's carriages carries a non-path machine-origin
    /// identifier into a command.** *(The fifth carriage, added 2026-09-23 for
    /// R1 item 5's Ubuntu arm, carries a name the machine's own detection tool
    /// answers, composed on the machine. This name is one Smoozy read off the
    /// machine at B10's search and would have to echo back, and no tool on the
    /// machine answers which package holds it — so nothing here moves.)*
    pub package_name: &'static str,
    /// **That manager's own removal command**, so the command and the name
    /// cannot come apart.
    pub removal: crate::elevated::AnElevatedStep,
}

/// **§13 item 21: the managers this module may remove a build from, and the one
/// entry is the Snap Store's own.**
///
/// **The one name established anywhere in this plan is the store's own** — the
/// Snap Store's package-information endpoint for the Blender Foundation's
/// package is `https://api.snapcraft.io/v2/snaps/info/blender`, read 2026-08-19
/// (`Preparation/Architecture/Sources/Server Setup.sources.md`, shape 2), and
/// that is the name [`crate::blender`] B2 reads the store by.
///
/// **The distribution package Fact 45's apt build carries is named nowhere this
/// document may read, so no second entry is written from memory** — and a
/// managed build whose manager is not established is reported as one this module
/// cannot remove, which is §13 item 21's own answer. **Row 96's warning is
/// unaffected**: such a build is still found by [`the_entries`] and still warned
/// about under [`the_warning_names`], and the user's own manager is the route.
pub const THE_MANAGERS_SMOOZY_MAY_REMOVE_FROM: [AManagerAndItsPackage; 1] =
    [AManagerAndItsPackage {
        manager: "the Snap Store",
        package_name: crate::blender::THE_SNAPS_PACKAGE,
        removal: crate::elevated::THE_MANAGER_ROUTE_REMOVAL,
    }];

/// **§13 item 21, performed: the pairing for a package name, or nothing.**
pub fn the_manager_route_for(package_name: &str) -> Option<AManagerAndItsPackage> {
    THE_MANAGERS_SMOOZY_MAY_REMOVE_FROM
        .into_iter()
        .find(|pairing| pairing.package_name == package_name)
}

/// **§13 item 21, as the question a caller asks.**
pub fn this_module_can_remove_the_managed_build(package_name: &str) -> bool {
    the_manager_route_for(package_name).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_candidate(path: &str, answered: Option<&str>) -> ACandidate {
        ACandidate {
            path: path.to_string(),
            answered: answered.and_then(Version::parse),
        }
    }

    /// **B12b: the note falls on each build whose bar the driver is below, by
    /// the version it answered** — on a 570 driver, 5.2.1 and a 5.3 build and
    /// nothing earlier (`Design/Server Setup § Blender`'s own example), and on
    /// no build at all where no driver answered.
    #[test]
    fn the_optix_note_falls_on_the_builds_whose_bar_the_driver_is_below() {
        use smoozy_servers::hardware::{DriverVersion, TheDriversOwnAnswer, WhereK1PlacesIt};
        let builds = [
            Version::parse("4.5.3").expect("a version"),
            Version::parse("5.1.1").expect("a version"),
            Version::parse("5.2.1").expect("a version"),
            Version::parse("5.3.0").expect("a version"),
        ];
        let a_driver = |version: &str| TheDriversOwnAnswer {
            version: DriverVersion::parse(version).expect("a driver version"),
            cuda_version: Some("12.8".into()),
            where_k1_placed_the_machine: WhereK1PlacesIt::ItsOwnKernel,
        };
        assert_eq!(
            the_builds_the_optix_note_falls_on(Some(&a_driver("570.211.01")), &builds),
            vec![builds[2], builds[3]]
        );
        assert!(the_builds_the_optix_note_falls_on(Some(&a_driver("595.91.07")), &builds).is_empty());
        assert!(the_builds_the_optix_note_falls_on(None, &builds).is_empty());
    }

    #[test]
    fn two_paths_that_resolve_to_one_launcher_are_two_entries() {
        // B10's second trap: deduplication is never by resolved path.
        let entries = the_entries(vec![
            a_candidate("/snap/bin/blender", Some("4.5.9")),
            a_candidate("/snap/blender/current/blender", Some("4.5.9")),
        ]);
        assert_eq!(entries.len(), 2);
        assert!(!the_search_is_restricted_to_one_filesystem());
    }

    #[test]
    fn a_candidate_that_did_not_answer_is_named_rather_than_listed() {
        // **B11's two halves, and the fault this test exists for.** Nothing
        // filtered anywhere until 2026-09-04, so an unanswering binary reached
        // the offered list; and a filter put inside `the_entries` would have
        // taken the naming half with it, because the naming reads the same
        // slice.
        let entries = the_entries(vec![
            a_candidate("/usr/bin/blender", Some("4.5.9")),
            a_candidate("/opt/odd/blender", None),
        ]);
        let answer = what_the_answer_says(&entries, 0);
        // **Named.**
        assert_eq!(answer.did_not_answer, vec!["/opt/odd/blender".to_string()]);
        // **And not listed.**
        let listed = the_listing(&entries);
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].path, "/usr/bin/blender");
        assert!(!listed.iter().any(|entry| entry.answered.is_none()));
        assert!(answer.the_search_is_by_name);
        assert!(the_machine_was_searched_completely(&answer));
    }

    #[test]
    fn a_refused_folder_makes_the_answer_say_the_machine_was_not_searched_completely() {
        let answer = what_the_answer_says(&[], 3);
        assert!(!the_machine_was_searched_completely(&answer));
        assert_eq!(answer.folders_refused, 3);
    }

    #[test]
    fn the_search_runs_at_three_moments_and_never_on_connecting() {
        assert_eq!(WhenTheSearchRuns::THE_THREE.len(), 3);
        assert!(!the_search_runs_on_connecting());
        assert_eq!(
            THE_SEARCHES_LIFETIME,
            smoozy_state::lifetimes::Lifetime::PeriodHeld
        );
        assert_eq!(THE_OBSERVED_SEARCH_COST, Duration::from_millis(5_300));
    }

    #[test]
    fn the_folder_is_derived_per_build_and_never_per_version() {
        // B12a: two builds answering one version have two paths and two
        // folders.
        assert_eq!(
            the_folder_for("/smoozy/blender/i-7-blender-5.2.0/blender"),
            "/smoozy/blender/i-7-blender-5.2.0"
        );
        assert_eq!(
            what_crosses_to_the_browser("/opt/other/blender"),
            "/opt/other"
        );
        assert_eq!(the_folder_for("/blender"), "/");
        assert!(!the_link_is_withheld_on_some_rows(true));
    }

    #[test]
    fn a_version_never_identifies_a_build() {
        // B13. Two builds of one version share a settings folder and both
        // answer the same version.
        assert!(!a_version_identifies_a_build(Version::parse("4.5.9").unwrap()));
        assert!(the_matching_rule().contains("executable's path"));
    }

    #[test]
    fn the_warning_reaches_a_job_under_the_folder_only_on_the_folder_deletion_route() {
        let live = [
            ALiveJob {
                state: LiveState::Rendering,
                recorded_build_path: "/smoozy/blender/i-7/blender".to_string(),
            },
            ALiveJob {
                state: LiveState::Waiting,
                recorded_build_path: "/snap/bin/blender".to_string(),
            },
        ];
        let named = the_warning_names(&live, "/smoozy/blender/i-7", RemovalRoute::TheFolderDeletion);
        assert_eq!(named.len(), 1);
        assert_eq!(named[0].state, LiveState::Rendering);

        // **The half this test used to skip.** On the manager route it named a
        // path that exactly matched the recorded one, so it never proved that a
        // job *under* the removed path is excluded there — which is the whole
        // difference between the two routes, since a managed build's recorded
        // path may be a launcher rather than a folder at all.
        let under_it = [ALiveJob {
            state: LiveState::Rendering,
            recorded_build_path: "/smoozy/blender/i-7/blender".to_string(),
        }];
        assert_eq!(
            the_warning_names(&under_it, "/smoozy/blender/i-7", RemovalRoute::TheFolderDeletion)
                .len(),
            1
        );
        assert!(the_warning_names(
            &under_it,
            "/smoozy/blender/i-7",
            RemovalRoute::TheMachinesOwnSoftwareManager
        )
        .is_empty());

        // And an exact match is named on either route.
        let named = the_warning_names(
            &live,
            "/snap/bin/blender",
            RemovalRoute::TheMachinesOwnSoftwareManager,
        );
        assert_eq!(named.len(), 1);
        assert_eq!(named[0].state, LiveState::Waiting);
    }

    /// **Step 10c's owed piece 6: on the manager route the build is the
    /// package**, so a job submitted with the snap's other path is named too —
    /// and a job at a path no package holds is not.
    #[test]
    fn the_manager_route_names_every_job_at_a_path_the_same_package_holds() {
        let at = |path: &str| ALiveJob {
            state: LiveState::Waiting,
            recorded_build_path: path.to_string(),
        };
        let live = [
            at("/snap/bin/blender"),
            at("/snap/blender/7599/blender"),
            at("/usr/bin/blender"),
            at("/smoozy/blender/i-7/blender"),
        ];
        let named = the_warning_names(
            &live,
            "/snap/bin/blender",
            RemovalRoute::TheMachinesOwnSoftwareManager,
        );
        assert_eq!(named, live[..2].to_vec());
        // **The folder route is unchanged**: the exact path, and what sits
        // under it.
        assert_eq!(
            the_warning_names(&live, "/snap/bin/blender", RemovalRoute::TheFolderDeletion),
            live[..1].to_vec()
        );
    }

    #[test]
    fn the_manager_route_never_borrows_the_folder_routes_observation() {
        // §13 item 17: what a manager removal does to a render already running
        // from that build was never observed, and the sentence the user reads
        // says so.
        let rendering = ALiveJob {
            state: LiveState::Rendering,
            recorded_build_path: "/snap/bin/blender".to_string(),
        };
        assert_eq!(
            what_the_warning_says(&rendering, RemovalRoute::TheMachinesOwnSoftwareManager),
            WhatTheWarningSays::ARenderingJobsFateIsNotKnownOnThisRoute
        );
        assert_eq!(
            what_the_warning_says(&rendering, RemovalRoute::TheFolderDeletion),
            WhatTheWarningSays::ARenderingJobWillFinish
        );
    }

    #[test]
    fn a_waiting_job_is_told_the_same_truth_on_either_route() {
        let waiting = ALiveJob {
            state: LiveState::Waiting,
            recorded_build_path: "/x/blender".to_string(),
        };
        for route in [
            RemovalRoute::TheFolderDeletion,
            RemovalRoute::TheMachinesOwnSoftwareManager,
        ] {
            assert_eq!(
                what_the_warning_says(&waiting, route),
                WhatTheWarningSays::AWaitingJobWillHaveNothingToRenderWith
            );
        }
    }

    #[test]
    fn the_settings_folder_survives_every_removal_and_no_removal_is_refused() {
        for route in [
            RemovalRoute::TheFolderDeletion,
            RemovalRoute::TheMachinesOwnSoftwareManager,
        ] {
            assert!(!the_settings_folder_is_removed_with_the_build(route));
        }
        assert!(!a_removal_is_refused_because_it_is_the_last_blender(true));
        assert!(the_removal_is_an_elevated_act(
            RemovalRoute::TheMachinesOwnSoftwareManager
        ));
        assert!(!the_removal_is_an_elevated_act(RemovalRoute::TheFolderDeletion));
    }

    #[test]
    fn a_managed_build_under_an_unfixed_name_cannot_be_removed_by_this_module() {
        // §13 item 21: no carriage carries a non-path machine-origin identifier
        // into a command, so the package name is a constant or there is no
        // removal.
        assert!(this_module_can_remove_the_managed_build("blender"));
        assert!(!this_module_can_remove_the_managed_build("blender-git-nightly"));
    }

    #[test]
    fn the_manager_and_its_package_name_and_its_command_are_one_pairing() {
        // **The fault this test exists for.** The set knew one package name and
        // the command knew one manager, and nothing tied them: an apt-installed
        // `/usr/bin/blender` — Fact 45's real case — passed the set and got the
        // snap command.
        assert_eq!(THE_MANAGERS_SMOOZY_MAY_REMOVE_FROM.len(), 1);
        let pairing = the_manager_route_for("blender").expect("the store's own name");
        assert_eq!(pairing.manager, "the Snap Store");
        assert_eq!(pairing.package_name, "blender");
        // The command is that manager's own and names that package.
        assert!(pairing.removal.plain.as_str().starts_with("snap "));
        assert!(pairing
            .removal
            .plain
            .as_str()
            .ends_with(pairing.package_name));
        // And a name outside the pairing selects no command at all.
        assert!(the_manager_route_for("blender-git-nightly").is_none());
    }
}

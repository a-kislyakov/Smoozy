//! §1.1 — choosing the root, and choosing the package area's mount beside it.
//!
//! **The readings arrive as values.** Each one is `Server protocol and
//! transfers` §9's own per-path reading, which carries the path it was taken
//! against, so **L2's *against each candidate's own path* is checked here
//! rather than trusted** and **F16's prohibition on widening a per-path answer
//! into a per-machine one has something to hold on to**.
//!
//! **L5: the user is not asked which mount.** `Open Threads` parks *where a
//! job's folder actually sits on the machine* and says it becomes Specs' only
//! if the user should be offered the choice. They are not.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` F1, F16.
//! `Rented-server runtime` M4a. `Server Setup` R7b.
//!
//! **A marker this document also defines is never listed there.** One below is
//! of that kind, and it writes the document meant out where it stands.

use smoozy_framing::free_space::FreeSpaceReading;

use crate::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits, WhyThatIsNotARoot};

/// The root's own segment under the chosen mount.
///
/// **Build's choice, and N7 is the constraint it is made against**: the prefix
/// is three segments under the mount and *"is kept short deliberately"*,
/// because §6 deepens paths by construction.
pub const THE_ROOTS_SEGMENT: &str = "smoozy";

/// **`Rented-server runtime` M4a's control-state reserve, at its one
/// declaration point.**
///
/// M4a's own words: the supervisor *"subtracts a **64 MiB** control-state
/// reserve"*, and *"The `64 MiB` value is a Build constant at one declaration
/// point and not a user setting."* **L4a is the other reader of it** — the
/// package area's remaining candidate must clear it — so it is declared once,
/// here, and the supervisor's own use at step 6 takes this one rather than
/// writing a second.
pub const THE_CONTROL_STATE_RESERVE_BYTES: u64 = 64 * 1024 * 1024;

/// One writable location `Server Setup` was given, with its reading.
///
/// **L2: the candidate set is the writable locations `Server Setup` was
/// given.** Nothing here enumerates a machine's filesystems to find them, which
/// is Fact 28's trap: a plain reading on one container returned **51
/// filesystems**, 48 memory-backed, one of them *"the **host's** disk,
/// bind-mounted in to deliver the driver files"* showing 20 GiB free the rental
/// may not touch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    /// The writable location itself. The root is composed one segment under it.
    pub writable_location: String,
    /// **The per-path reading taken against that location.** Absent where the
    /// reading could not be taken — and **an absent reading is never stood in
    /// for**, which is what `Server protocol and transfers` F16 and `Servers`
    /// H10 require. **`Servers` H10 and not this document's H10**, which is
    /// §9.3's queue entry and says something else entirely.
    pub reading: Option<FreeSpaceReading>,
}

/// What the choice produced. **Made once per installation at hand-over and
/// never re-chosen** (L1, L4a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheChoice {
    pub layout: TheLayout,
    /// The reading the root was chosen on, kept so that the choice can be
    /// accounted for afterwards.
    pub the_root_was_chosen_on: FreeSpaceReading,
    /// **Absent where L4a's second choice did not fire**, which is `Server
    /// Setup` R7b's *"nothing second is chosen, recorded or handed over"*.
    pub the_package_area_was_chosen_on: Option<FreeSpaceReading>,
}

/// Why no root could be chosen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyNoRootCouldBeChosen {
    /// **L2 has nothing to choose between.** No candidate was given at all.
    ThereWereNoCandidates,
    /// **Every candidate either had no reading, or its reading was taken
    /// against some other path, or the root under it is not a root Smoozy may
    /// use** (L3, N5). **Nothing is substituted for a missing figure**, so the
    /// choice does not happen and says so.
    NoCandidateOfferedAFigureThisDocumentMayChooseOn,
}

/// **L2 and L4a, performed as one choice.**
///
/// **L2:** *"The candidate set is the writable locations `Server Setup` was
/// given, and the choice is made by a per-path free-space reading against each
/// candidate's own path … reading the **available** figure and never the free
/// one. The largest available figure wins."*
///
/// **L4:** *"A rental's second filesystem may be a quota or may be a real second
/// device, and nothing here assumes either."* **So size alone chooses, and no
/// expectation about a second filesystem enters in either direction** — not
/// that it is faster, separate or independently full, and not that it is not.
/// **On a machine of the second shape the conventional root is the wrong one by
/// a factor of eight**, which is what makes this reading load-bearing rather
/// than tidy.
///
/// **L4a:** *"The package area takes the remaining candidate with the largest
/// available figure, by L2's own per-path reading, provided that figure clears
/// the 64 MiB control-state reserve — and where no remaining candidate stands
/// or clears it, the package area sits under the root as a sibling of the
/// others."*
///
/// **Where two candidates read the same figure the first given wins.** That is
/// Build's own tie-break and the plan states none; it is here rather than left
/// to a sort's stability so that one installation's choice is reproducible from
/// the candidate list.
pub fn choose(candidates: &[Candidate]) -> Result<TheChoice, WhyNoRootCouldBeChosen> {
    if candidates.is_empty() {
        return Err(WhyNoRootCouldBeChosen::ThereWereNoCandidates);
    }

    let mut usable: Vec<(MachineRoot, FreeSpaceReading)> = Vec::new();
    for candidate in candidates {
        if let Some(pair) = what_this_candidate_offers(candidate) {
            usable.push(pair);
        }
    }
    if usable.is_empty() {
        return Err(WhyNoRootCouldBeChosen::NoCandidateOfferedAFigureThisDocumentMayChooseOn);
    }

    // **The largest available figure wins**, with the first given winning a tie.
    let mut best = 0;
    for (at, (_, reading)) in usable.iter().enumerate() {
        if reading.available_bytes > usable[best].1.available_bytes {
            best = at;
        }
    }
    let (root, the_root_was_chosen_on) = usable.remove(best);

    // **L4a, over what is left of the same list.**
    //
    // **A candidate whose root path equals the one already chosen is passed
    // over**, because L4a places the package area *"on a writable mount chosen
    // apart from the job area's, where L2's candidate set offers more than
    // one"*, and **two identical paths are not more than one mount**. Two
    // candidates written `/mnt/data` and `/mnt/data/` compose the same root,
    // and without this the second is recorded as the package area's own
    // location, apart, at the identical place — after which the desktop's
    // record says a second mount was chosen when none was and `Server Setup`
    // R7b's second-mount hand-over runs against the same mount.
    //
    // **Plain equality of the recorded path and nothing more.** No normalising
    // and no resolving: working out whether two differently written machine
    // paths are the same place is a claim about the machine, and nothing here
    // makes one.
    let mut second: Option<usize> = None;
    for (at, (candidate_root, reading)) in usable.iter().enumerate() {
        if candidate_root.as_path() == root.as_path() {
            continue;
        }
        if reading.available_bytes < THE_CONTROL_STATE_RESERVE_BYTES {
            continue;
        }
        match second {
            Some(so_far) if reading.available_bytes <= usable[so_far].1.available_bytes => {}
            _ => second = Some(at),
        }
    }

    let (package_area, the_package_area_was_chosen_on) = match second {
        Some(at) => {
            let (apart, reading) = usable.remove(at);
            (WhereThePackageAreaSits::ApartAt(apart), Some(reading))
        }
        None => (WhereThePackageAreaSits::UnderTheRoot, None),
    };

    Ok(TheChoice {
        layout: TheLayout::of(root, package_area),
        the_root_was_chosen_on,
        the_package_area_was_chosen_on,
    })
}

/// One candidate's root and reading, or nothing.
///
/// **Three ways a candidate offers nothing, and each is a refusal rather than a
/// substitution.** It has no reading; its reading was taken against some other
/// path, which would make it a figure about a mount nobody asked about; or the
/// root under it fails L3 or N5.
fn what_this_candidate_offers(candidate: &Candidate) -> Option<(MachineRoot, FreeSpaceReading)> {
    let reading = candidate.reading.as_ref()?;
    if reading.the_path_it_was_taken_against != candidate.writable_location {
        return None;
    }
    let path = format!(
        "{}/{}",
        candidate.writable_location.trim_end_matches('/'),
        THE_ROOTS_SEGMENT
    );
    match MachineRoot::chosen(&path) {
        Ok(root) => Some((root, reading.clone())),
        Err(WhyThatIsNotARoot::ItIsNotAnAbsolutePath)
        | Err(WhyThatIsNotARoot::ItIsUnderASharedTemporaryArea)
        | Err(WhyThatIsNotARoot::ItHasARelativeSegment)
        | Err(WhyThatIsNotARoot::ItDoesNotFitTheBudget(_)) => None,
        // **Never returned by the text tests above** — they are the machine's
        // refusals, made where it reads where the root really leads. Listed so
        // that a candidate refused for either is refused here too. *Added
        // 2026-10-16, step 10c owed piece 7.*
        Err(WhyThatIsNotARoot::ItLeadsUnderASharedTemporaryArea)
        | Err(WhyThatIsNotARoot::WhereItLeadsCouldNotBeRead) => None,
    }
}

/// **What L1 records, and it is recorded in two places.**
///
/// *"Its path is recorded in the machine's state area at hand-over, beside the
/// installation identity, and it is recorded against the server row on the
/// desktop side as a machine-derived value carrying that identity."* **The
/// desktop half is `Servers`' server row** — a column that document's D3 did not
/// name until it was repaired at Build.
///
/// **L4a's second location travels in the same act** (`Server Setup` R7b): *"a
/// reading that finds one finds both"*, and where L4a's second choice did not
/// fire **nothing second is chosen, recorded or handed over**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatIsRecorded {
    pub root_path: String,
    pub the_package_areas_own_location: Option<String>,
}

impl TheChoice {
    /// The two values the hand-over writes down, taken from the choice itself
    /// so the record and the choice cannot drift.
    pub fn what_is_recorded(&self) -> WhatIsRecorded {
        WhatIsRecorded {
            root_path: self.layout.root().as_path().to_string(),
            the_package_areas_own_location: match self.layout.where_the_package_area_sits() {
                WhereThePackageAreaSits::UnderTheRoot => None,
                WhereThePackageAreaSits::ApartAt(apart) => Some(apart.as_path().to_string()),
            },
        }
    }
}

/// **The layout again, from what was written down.**
///
/// **The root is chosen once per installation and never re-chosen** (L1), so
/// every later sitting rebuilds the layout from the record rather than choosing
/// again. A record whose paths no longer pass L3 or N5 rebuilds nothing — which
/// is the same refusal [`choose`] makes, rather than a root nobody may use being
/// resurrected because it was once written down.
pub fn the_layout_again(recorded: &WhatIsRecorded) -> Option<TheLayout> {
    let root = MachineRoot::chosen(&recorded.root_path).ok()?;
    let package_area = match &recorded.the_package_areas_own_location {
        None => WhereThePackageAreaSits::UnderTheRoot,
        Some(path) => WhereThePackageAreaSits::ApartAt(MachineRoot::chosen(path).ok()?),
    };
    Some(TheLayout::of(root, package_area))
}

/// **L5, written down rather than left as the absence of a screen.**
pub const THE_USER_IS_NOT_ASKED_WHICH_MOUNT: bool = true;

/// **L4's caution, carried so that nobody reaches for the large mount by
/// name.**
///
/// *"The large mount's name on that seller is `/ephemeral`, **nothing was
/// tested about whether it survives a stop, a start or a restart**, and the name
/// is the seller's own claim rather than an observation — so size alone never
/// chooses it."* **Nothing in [`choose`] reads a candidate's name at all**, and
/// this constant is what says that is deliberate.
pub const NO_CANDIDATE_IS_EVER_CHOSEN_OR_REFUSED_BY_ITS_NAME: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_framing::free_space::WhichRoute;

    fn a_reading(path: &str, available: u64) -> FreeSpaceReading {
        FreeSpaceReading {
            available_bytes: available,
            the_path_it_was_taken_against: path.to_string(),
            route: WhichRoute::TheStatvfsExtension,
        }
    }

    fn a_candidate(path: &str, available: u64) -> Candidate {
        Candidate {
            writable_location: path.to_string(),
            reading: Some(a_reading(path, available)),
        }
    }

    #[test]
    fn the_largest_available_figure_wins_and_the_conventional_root_loses() {
        // L4's real-device shape, observed: `/dev/vda1` at 97 GB on `/` and
        // `/dev/vdb` at 787 GB on `/ephemeral`. **The conventional root is the
        // wrong one by a factor of eight.**
        let choice = choose(&[
            a_candidate("/", 97_000_000_000),
            a_candidate("/ephemeral", 787_000_000_000),
        ])
        .unwrap();
        assert_eq!(choice.layout.root().as_path(), "/ephemeral/smoozy");
        assert_eq!(
            choice.the_root_was_chosen_on.available_bytes,
            787_000_000_000
        );
    }

    #[test]
    fn l4a_places_the_package_area_apart_where_a_second_candidate_clears_the_reserve() {
        let choice = choose(&[
            a_candidate("/ephemeral", 787_000_000_000),
            a_candidate("/", 97_000_000_000),
        ])
        .unwrap();
        match choice.layout.where_the_package_area_sits() {
            WhereThePackageAreaSits::ApartAt(apart) => assert_eq!(apart.as_path(), "/smoozy"),
            other => panic!("L4a did not place the package area apart: {other:?}"),
        }
        assert!(choice.the_package_area_was_chosen_on.is_some());
        // **On either arrangement the areas are still siblings of nothing** —
        // the package area on its own mount sits under no shared root at all.
        assert_eq!(
            choice.layout.area(crate::layout::Area::Package),
            "/smoozy/package"
        );
    }

    #[test]
    fn l4a_leaves_the_package_area_under_the_root_where_nothing_clears_the_reserve() {
        // A second mount with less than the 64 MiB reserve on it is not a place
        // for the package area, and **nothing second is chosen or recorded**.
        let choice = choose(&[
            a_candidate("/ephemeral", 787_000_000_000),
            a_candidate("/scratch", 1_000_000),
        ])
        .unwrap();
        assert_eq!(
            choice.layout.where_the_package_area_sits(),
            &WhereThePackageAreaSits::UnderTheRoot
        );
        assert!(choice.the_package_area_was_chosen_on.is_none());
    }

    #[test]
    fn one_writable_candidate_leaves_the_package_area_under_the_root() {
        let choice = choose(&[a_candidate("/", 500_000_000_000)]).unwrap();
        assert_eq!(
            choice.layout.where_the_package_area_sits(),
            &WhereThePackageAreaSits::UnderTheRoot
        );
        assert!(choice.the_package_area_was_chosen_on.is_none());
    }

    #[test]
    fn a_candidate_with_no_reading_is_not_chosen_and_nothing_is_substituted() {
        let refused = choose(&[Candidate {
            writable_location: "/ephemeral".to_string(),
            reading: None,
        }]);
        assert_eq!(
            refused,
            Err(WhyNoRootCouldBeChosen::NoCandidateOfferedAFigureThisDocumentMayChooseOn)
        );
    }

    #[test]
    fn a_reading_taken_against_some_other_path_does_not_choose_this_candidate() {
        // F1's requirement is that a reading names the path it was taken
        // against; F16 forbids widening it. A figure from another mount is
        // exactly the widening, so it chooses nothing.
        let refused = choose(&[Candidate {
            writable_location: "/ephemeral".to_string(),
            reading: Some(a_reading("/", 787_000_000_000)),
        }]);
        assert_eq!(
            refused,
            Err(WhyNoRootCouldBeChosen::NoCandidateOfferedAFigureThisDocumentMayChooseOn)
        );
    }

    #[test]
    fn l3_takes_a_shared_temporary_area_out_of_the_candidate_set_however_big_it_reads() {
        let choice = choose(&[
            a_candidate("/var/tmp", 999_000_000_000),
            a_candidate("/", 97_000_000_000),
        ])
        .unwrap();
        assert_eq!(choice.layout.root().as_path(), "/smoozy");
    }

    #[test]
    fn no_candidate_at_all_is_its_own_answer() {
        assert_eq!(
            choose(&[]),
            Err(WhyNoRootCouldBeChosen::ThereWereNoCandidates)
        );
    }

    #[test]
    fn the_layout_survives_being_written_down_and_read_back_on_both_arrangements() {
        // **L1: the root is chosen once per installation and never re-chosen**,
        // so a later sitting has to rebuild the same layout out of the two
        // values the hand-over wrote against the server row.
        for candidates in [
            vec![a_candidate("/ephemeral", 787_000_000_000)],
            vec![
                a_candidate("/ephemeral", 787_000_000_000),
                a_candidate("/", 97_000_000_000),
            ],
        ] {
            let choice = choose(&candidates).unwrap();
            let again = the_layout_again(&choice.what_is_recorded()).expect("read back");
            assert_eq!(again, choice.layout);
        }
    }

    #[test]
    fn a_recorded_root_that_no_longer_passes_l3_rebuilds_nothing() {
        let refused = the_layout_again(&WhatIsRecorded {
            root_path: "/var/tmp/smoozy".to_string(),
            the_package_areas_own_location: None,
        });
        assert!(refused.is_none());
    }

    #[test]
    fn two_candidates_at_one_path_are_not_two_mounts_and_nothing_second_is_recorded() {
        // **L4a: *where L2's candidate set offers more than one*.** The root
        // trims a trailing slash, so these two compose the same path — and
        // recording the second as the package area's own location, apart, would
        // tell the desktop a second mount was chosen when none was, and send
        // `Server Setup` R7b's second-mount hand-over at the same mount.
        let choice = choose(&[
            a_candidate("/mnt/data", 500_000_000_000),
            a_candidate("/mnt/data/", 500_000_000_000),
        ])
        .unwrap();
        assert_eq!(choice.layout.root().as_path(), "/mnt/data/smoozy");
        assert_eq!(
            choice.layout.where_the_package_area_sits(),
            &WhereThePackageAreaSits::UnderTheRoot
        );
        assert!(choice.the_package_area_was_chosen_on.is_none());
        assert!(choice
            .what_is_recorded()
            .the_package_areas_own_location
            .is_none());
    }

    #[test]
    fn a_second_mount_at_a_different_path_is_still_chosen() {
        // The pass-over above is plain equality and nothing more, so a real
        // second mount is untouched by it.
        let choice = choose(&[
            a_candidate("/mnt/data", 500_000_000_000),
            a_candidate("/mnt/data/", 500_000_000_000),
            a_candidate("/state", 100_000_000_000),
        ])
        .unwrap();
        match choice.layout.where_the_package_area_sits() {
            WhereThePackageAreaSits::ApartAt(apart) => assert_eq!(apart.as_path(), "/state/smoozy"),
            other => panic!("L4a did not place the package area apart: {other:?}"),
        }
    }

    #[test]
    fn a_tie_is_settled_by_the_order_the_candidates_were_given() {
        let choice = choose(&[a_candidate("/first", 10_000), a_candidate("/second", 10_000)])
            .unwrap();
        assert_eq!(choice.layout.root().as_path(), "/first/smoozy");
    }
}

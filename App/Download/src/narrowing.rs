//! Narrowing, the count and the formats — `Download` §6, N1 to N8 with N4b,
//! N4c, N4d and N5a; and §13, Row 214 closed unresolved. Rows 213, 215, 298,
//! 314 and 412.
//!
//! **Constraint:** the spec's two promises the narrowing is held to —
//! *"everything written while the arrangement was off comes back"* and
//! *"changing their mind about the narrowing costs the user nothing"*.
//!
//! **The count is `Job files and outputs` §7's and is consumed, never derived
//! a second way.** Entries only, files expected on disk, frames producing
//! nothing counting as nothing, view layers multiplying nothing, added write
//! places in no count. **Nothing in this file counts a file.**
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` I13, I14, I19, I20, I21,
//! L7a. `Render Settings` Y7. `Dependency Upload & Remap` M22a, M22b, M22f.
//! `File Browser` U11.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Job files and outputs` D21e and
//! `Job files and outputs` P10 are the two that arise here.

use smoozy_dependencies::remap::PlacedCopyList;
use smoozy_job_files::inventory::TheInventory;
use smoozy_job_files::reading::{Destination, TheOutputsHalf};
use smoozy_server_protocol::transfers::SourceReading;

use crate::arrangement::TheNarrowing;

/// **N1: the list of formats offered is read off the recorded reading's
/// destination set — every destination's format, deduplicated.**
///
/// **It exists at the moments `Render Settings` Q4 and `Render Settings` Y7
/// fix**: after a pre-load, or from render start otherwise, never before. **It is a product of the file
/// and the version rendering it, so it is never a fixed list Smoozy carries.**
/// Where a node gathers outputs into a multi-layer file, the format that
/// counts is that file's — already inside the recorded reading.
pub fn the_formats_offered(reading: &TheOutputsHalf) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    for destination in &reading.destinations {
        let Some(format) = &destination.format else {
            continue;
        };
        let name = format.as_the_file_records_it.as_bytes().to_vec();
        if !out.contains(&name) {
            out.push(name);
        }
    }
    out
}

/// **N2: the list reaches the desktop as a reading of the job's record
/// directory, and until it lands there is no list and nothing to narrow
/// with.**
///
/// Where the reading ran at pre-load, the material is already on the desktop
/// side from that moment. Where the job was submitted unread, the machine-side
/// reading lands when the job starts, and the desktop reads the recorded
/// reading and the inventory from the job's record directory over the
/// connection, at or after render-start — **a reading with its moment.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheNarrowingMaterial {
    /// It is on the desktop side. The formats it offers are N1's.
    OnTheDesktop(Box<TheOutputsHalf>),
    /// **It has not landed.** There is no list, the narrowing control has
    /// nothing to offer, and the arrangement covers everything — which is what
    /// makes a list arriving later invalidate nothing.
    ItHasNotLanded,
}

impl TheNarrowingMaterial {
    /// The formats the control may offer, which is none until the material
    /// lands.
    pub fn the_formats_it_offers(&self) -> Vec<Vec<u8>> {
        match self {
            TheNarrowingMaterial::OnTheDesktop(reading) => the_formats_offered(reading),
            TheNarrowingMaterial::ItHasNotLanded => Vec::new(),
        }
    }

    /// **N3: an arrangement made with no list covers everything.**
    pub fn a_narrowing_may_be_offered(&self) -> bool {
        matches!(self, TheNarrowingMaterial::OnTheDesktop(_))
    }
}

/// **N3: *everything* is no filter rather than a frozen list, so the list
/// arriving later invalidates nothing.**
///
/// From the moment the narrowing material is on the desktop, the job's queue
/// entry offers the narrowing; **a narrowing the user applies takes effect on
/// the arrangement from that moment forward.** Files already fetched outside
/// the new narrowing stay — *what has already come home stays* — and files not
/// yet fetched outside it stop being owed.
pub const EVERYTHING_IS_NO_FILTER_RATHER_THAN_A_FROZEN_LIST: bool = true;

/// **N4: narrowing is by format and reaches only the inventory's per-file
/// entries.**
///
/// A file with no format in the reading — an added write place's file, an
/// unmatched earlier-attempt file — **carries nothing for a format cut to
/// select against**. Such a file is **outside the narrowing**, which means it
/// is **not filtered out by it**, and not *outside the arrangement*.
///
/// **It comes at the ending walk under every arrangement, narrowed or not.**
/// Whatever a narrowing passed over — a file that **has** a format and was not
/// one of the chosen ones — is still on the server and fetchable by hand,
/// which is what makes that choice safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheNarrowingSaysAbout {
    /// It has a format and the narrowing chose it. Owed like anything else.
    ItIsOneOfTheChosenFormats,
    /// It has a format and the narrowing did not choose it. **Out of the
    /// whole, silently correct**, and re-openable where the narrowing widens.
    ItIsNotOneOfTheChosenFormats,
    /// **It carries no format at all**, so the cut cannot speak about it. Not
    /// filtered out — it comes at the ending walk.
    TheCutCannotSpeakAboutIt,
}

/// **N4 and N5, as one judgement over an inventory entry.**
///
/// **N5: a file is one of the narrowed formats when its inventory entry's
/// destination carries one of the chosen formats in the recorded reading. The
/// match is against the inventory by predicted path, never against the file's
/// name, and nothing is opened and nothing is changed.**
///
/// **A name-suffix rule is refused outright**: an output's name is the
/// artist's and carries no promise about its format, a multi-layer file's
/// format is the file's rather than its outputs', and the inventory already
/// holds the one authoritative mapping (`Job files and outputs` I14) —
/// deriving a second one from spelling would be a second count by another
/// name.
pub fn what_the_narrowing_says_about(
    narrowing: &TheNarrowing,
    destinations: &[Destination],
    the_entrys_destinations: &[usize],
) -> WhatTheNarrowingSaysAbout {
    let formats: Vec<&[u8]> = the_entrys_destinations
        .iter()
        .filter_map(|at| destinations.get(*at))
        .filter_map(|destination| destination.format.as_ref())
        .map(|format| format.as_the_file_records_it.as_bytes())
        .collect();
    if formats.is_empty() {
        return WhatTheNarrowingSaysAbout::TheCutCannotSpeakAboutIt;
    }
    match narrowing {
        TheNarrowing::Everything => WhatTheNarrowingSaysAbout::ItIsOneOfTheChosenFormats,
        TheNarrowing::TheseFormats(chosen) => {
            if formats
                .iter()
                .any(|format| chosen.iter().any(|one| one.as_slice() == *format))
            {
                WhatTheNarrowingSaysAbout::ItIsOneOfTheChosenFormats
            } else {
                WhatTheNarrowingSaysAbout::ItIsNotOneOfTheChosenFormats
            }
        }
    }
}

/// **N5: a walk entry matching no inventory entry has no format and falls
/// under N4.**
///
/// The match is by predicted path and by nothing else.
pub fn the_inventory_entry_for<'a>(
    inventory: &'a TheInventory,
    predicted_path: &[u8],
) -> Option<&'a smoozy_job_files::inventory::AnEntry> {
    inventory
        .entries
        .iter()
        .find(|entry| entry.predicted_path == predicted_path)
}

/// **N4b: why the old reading could not stand, in one line each.**
///
/// `What a job writes` promises unconditionally that a bake and a cache *"are
/// brought home with it, and are not left on a machine that is about to be
/// destroyed"*, and `Dependency Upload & Remap`'s spec repeats it. **The user
/// cannot ask for them back either**: the choice is *"by file format, and goes
/// no finer"* and `Job files and outputs` I14 gives an added write place no
/// per-file entries and so no format, **so nothing a bake writes can ever
/// appear in that list.**
///
/// **N4c: this is not a finer cut and does not touch the Creator's ruling.**
/// *By file format, and goes no finer* fixes what the **cut** is. A file with
/// no format is not a finer cut of the same set; it is a file the cut cannot
/// speak about, and reading the cut's silence as exclusion was an inference
/// rather than any spec's promise. **No new control, no new choice, and
/// nothing new on screen.**
///
/// **N4d: what it costs, stated rather than absorbed.** A narrowed
/// arrangement now also brings back whatever the render wrote into an added
/// write place, which can be far larger than the frames the user narrowed to.
/// **The arrangement's own reading is what makes that visible** — such files
/// are counted separately and reported separately, never folded into the
/// narrowed count — and the words are Design's. **Nothing here lets a user
/// decline them, because both specs say they come home.**
pub const A_FILE_WITH_NO_FORMAT_IS_OUTSIDE_THE_NARROWING_AND_NOT_OUTSIDE_THE_ARRANGEMENT: bool =
    true;

/// **N5a: one class of file under the job folder is skipped on the way home —
/// a bake or cache Smoozy itself placed there before the render.**
///
/// `Dependency Upload & Remap` M22a places a bake the user uploaded at the
/// redirect's own job-folder target, so the render finds it, and
/// `Job files and outputs` L7a carries the one exception to *nothing but what
/// a render writes sits inside a job output folder* that this needs.
/// **Without this rule the user would download the bake they had just
/// uploaded**, which on a simulation cache is not a rounding error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherItIsSkippedOnTheWayHome {
    /// **A path in the placed-copy list whose four values still match is the
    /// copy Smoozy placed, and is skipped.**
    ItIsTheCopySmoozyPlaced,
    /// **A path whose values have moved is not that copy any more — the render
    /// wrote there — and it comes home like everything else.**
    ItComesHomeLikeEverythingElse,
}

/// **N5a: the test is the placement record and never the file, and it is a
/// comparison against what the record says was placed.**
///
/// Smoozy placed it, so Smoozy knows the exact paths it wrote **and what it
/// wrote at each of them** — `Dependency Upload & Remap` M22f's placed-copy
/// list, a field of the job's own local record, holding each copy's exact
/// job-folder path and the four-value identity read the moment the copy was
/// made.
///
/// **Comparing against another *file* — by name, or by looking like a bake —
/// would indeed get it wrong. Comparing against Smoozy's own *record* is the
/// opposite thing**, and without it the Creator's own *both* case
/// (`Dependency Upload & Remap` M22b) lost the recompute silently.
///
/// **Its failure direction is the safe one.** A copy whose four values are
/// unchanged is skipped, which costs the user nothing — an identical result is
/// not a result lost. A copy that was touched comes home, which costs at worst
/// a re-download of something the user already has.
///
/// **The skip is by path and by the four values at it, and stops there.**
/// Anything the render wrote **beside** a placed file, under the same added
/// write place, is not a path Smoozy placed and comes home like everything
/// else — as does anything it wrote **over** one. So a job whose render did
/// recompute into that folder brings its results home, and a job whose render
/// read the placed bake and wrote nothing new brings home an
/// empty-of-new folder, which is `Job files and outputs` D21e's own *reported
/// as empty rather than as a loss*.
///
/// **Where the record is gone, nothing is skipped**: a machine whose placement
/// record cannot be read brings the folder home whole — the safe direction.
pub fn whether_it_is_skipped_on_the_way_home(
    placed: Option<&PlacedCopyList>,
    path_on_the_machine: &[u8],
    four_values_now: &SourceReading,
) -> WhetherItIsSkippedOnTheWayHome {
    let Some(placed) = placed else {
        return WhetherItIsSkippedOnTheWayHome::ItComesHomeLikeEverythingElse;
    };
    let matched = placed
        .copies
        .iter()
        .find(|copy| copy.job_folder_path.as_bytes() == path_on_the_machine);
    match matched {
        Some(copy) if !copy.was_written_over(four_values_now) => {
            WhetherItIsSkippedOnTheWayHome::ItIsTheCopySmoozyPlaced
        }
        _ => WhetherItIsSkippedOnTheWayHome::ItComesHomeLikeEverythingElse,
    }
}

/// **N5a as the ending walk needs it: the placement record, the folder the
/// walk's paths are relative to, and whatever four-value readings are in
/// hand.**
///
/// **The four-value readings are empty today and that is a recorded absence,
/// not a gap here.** `Server protocol and transfers` S13's reading is taken
/// for a download's own resume (the 38's piece 9,
/// [`crate::acting::fetching::TheFourValues`]) and by nothing for a placed
/// copy, so no placed copy
/// can be judged and **nothing is skipped** — which is the plan's own safe
/// direction: a copy that comes home costs at worst a re-download of something
/// the user already has. **What the walk does not do is let that pass
/// silently**: every placed path it could not judge is named on the verdict,
/// so the arrangement's own reading can say so.
pub struct ThePlacedCopies<'a> {
    /// **`Dependency Upload & Remap` M22f's placed-copy list**, a field of the
    /// job's own local record. **Absent is a value**: where the placement
    /// record cannot be read, nothing is skipped and the folder comes home
    /// whole.
    pub list: Option<&'a PlacedCopyList>,
    /// The job output folder, because the list holds whole machine paths and
    /// the walk holds paths relative to it.
    pub the_job_output_folder: &'a [u8],
    /// **`Server protocol and transfers` S13's four values for a placed path
    /// as they read now**, by whole machine path. A path with no reading here
    /// cannot be judged.
    pub the_four_values_now: &'a [(Vec<u8>, SourceReading)],
}

/// **What N5a says about one file the ending walk is about to bring home.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatN5aSays {
    /// **Nothing at all**: the path is not one Smoozy placed, so N5a does not
    /// reach it and it comes home like everything else.
    ItIsNotAPlacedCopy,
    /// **It is the copy Smoozy placed and its four values still match**, so it
    /// is skipped on the way home.
    ItIsTheCopySmoozyPlacedAndIsSkipped,
    /// **It is a path Smoozy placed and the four values could not be read**,
    /// so it comes home — the safe direction — **and the arrangement says
    /// so.**
    ItIsAPlacedPathThatCouldNotBeJudged,
}

impl ThePlacedCopies<'_> {
    /// **N5a asked of one walk path**, which is relative to the job output
    /// folder.
    ///
    /// **The comparison is against Smoozy's own placement record and never
    /// against the file** — not by name and not by looking like a bake — which
    /// is what makes the skip safe at all.
    pub fn what_it_says_about(&self, relative_to_the_job_output_folder: &[u8]) -> WhatN5aSays {
        let Some(list) = self.list else {
            return WhatN5aSays::ItIsNotAPlacedCopy;
        };
        let mut whole = self.the_job_output_folder.to_vec();
        if !whole.ends_with(b"/") {
            whole.push(b'/');
        }
        whole.extend_from_slice(relative_to_the_job_output_folder);
        if !list
            .copies
            .iter()
            .any(|copy| copy.job_folder_path.as_bytes() == whole)
        {
            return WhatN5aSays::ItIsNotAPlacedCopy;
        }
        let Some((_, now)) = self
            .the_four_values_now
            .iter()
            .find(|(path, _)| *path == whole)
        else {
            return WhatN5aSays::ItIsAPlacedPathThatCouldNotBeJudged;
        };
        match whether_it_is_skipped_on_the_way_home(self.list, &whole, now) {
            WhetherItIsSkippedOnTheWayHome::ItIsTheCopySmoozyPlaced => {
                WhatN5aSays::ItIsTheCopySmoozyPlacedAndIsSkipped
            }
            WhetherItIsSkippedOnTheWayHome::ItComesHomeLikeEverythingElse => {
                WhatN5aSays::ItIsNotAPlacedCopy
            }
        }
    }
}

/// **N5a: no count moves.**
///
/// A placed bake was never an inventory entry — `Job files and outputs` I14
/// gives an added write place a folder with no per-file entries and no
/// contribution to the count — so skipping it removes nothing from any figure,
/// and the whole is untouched. **This is a rule about what is fetched, never
/// about what is counted.**
pub const THE_SKIP_IS_A_RULE_ABOUT_WHAT_IS_FETCHED_AND_NEVER_ABOUT_WHAT_IS_COUNTED: bool = true;

/// **N5a: it never reaches an ordinary output.**
///
/// The skip is confined to paths under `Job files and outputs` 5.5's four
/// added write places, which is where L7a confines the placement in the first
/// place. **The regular output path and every file output node's folder are
/// outside it by construction, so no frame can ever be skipped by this rule.**
pub const THE_SKIP_NEVER_REACHES_AN_ORDINARY_OUTPUT: bool = true;

/// **N6: the whole an arrangement reads against is the number of the narrowed
/// inventory entries — every entry where the arrangement is unnarrowed — plus
/// the video statement.**
///
/// **The count is `Job files and outputs`' and is consumed, never derived a
/// second way**: entries only (I14), files expected on disk (I13), frames
/// producing nothing counting as nothing, view layers multiplying nothing,
/// added write places in no count. **A video destination's entry counts one
/// and the whole is stated as possibly growing** (I19, I20); further split
/// files arriving under the folder are fetched at the ending and counted as
/// the growth the statement allowed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheWhole {
    /// The number of narrowed inventory entries.
    pub entries: usize,
    /// **The video statement**, where any of the job's destinations is
    /// video-shaped: the whole is stated as possibly growing rather than as a
    /// number that will have to be broken.
    pub it_may_grow: bool,
}

/// **N6, computed over the inventory and the narrowing.**
///
/// **The video statement is read over the same narrowed entries the count
/// walks, and not off the raw inventory.** A whole that counted only the
/// narrowed entries and then said *it may grow* because some entry the
/// narrowing cut out is video-shaped would tell the user their total may grow
/// when nothing they are waiting for can.
pub fn the_whole(
    inventory: &TheInventory,
    destinations: &[Destination],
    narrowing: &TheNarrowing,
) -> TheWhole {
    let it_is_in_the_whole = |entry: &smoozy_job_files::inventory::AnEntry| {
        !matches!(
            what_the_narrowing_says_about(
                narrowing,
                destinations,
                &entry.the_destinations_aiming_at_it
            ),
            WhatTheNarrowingSaysAbout::ItIsNotOneOfTheChosenFormats
        )
    };
    TheWhole {
        entries: inventory
            .entries
            .iter()
            .filter(|entry| it_is_in_the_whole(entry))
            .count(),
        it_may_grow: inventory
            .entries
            .iter()
            .filter(|entry| it_is_in_the_whole(entry))
            .any(|entry| entry.may_grow),
    }
}

/// **N7: what leaves the whole, and what stays in it.**
///
/// Units leave the whole **only where the promise itself says they are not
/// coming through this arrangement**, and everything that leaves is reported
/// separately — `File Browser` U11's discipline, agreed with rather than
/// re-derived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherItLeavesTheWhole {
    /// **left alone** — out of the whole, reported with the amber line's list.
    LeftAlone,
    /// **cannot land** — out, reported with the refusal.
    CannotLand,
    /// **skipped by narrowing** — out, silently correct, the whole recomputed.
    SkippedByNarrowing,
    /// **newer version on the server** — the original stays counted as
    /// fetched; the telling rides beside it.
    ItStaysCountedAsFetched,
    /// **never coming** — a narrowed entry absent at the ending walk, or
    /// present at zero bytes — **stays in the whole and is the shortfall.**
    ///
    /// **That is deliberate: the short count is the one saying the spec
    /// promises for what nobody could vouch for, and moving these out would
    /// round the arrangement up to done over a hole.**
    ItStaysInTheWholeAndIsTheShortfall,
}

impl WhetherItLeavesTheWhole {
    pub fn leaves_the_whole(self) -> bool {
        matches!(
            self,
            WhetherItLeavesTheWhole::LeftAlone
                | WhetherItLeavesTheWhole::CannotLand
                | WhetherItLeavesTheWhole::SkippedByNarrowing
        )
    }
}

/// **N7, read off a reckoning entry's terminal state.**
pub fn whether_it_leaves_the_whole(
    state: &crate::arrangement::WhatSmoozyDid,
) -> Option<WhetherItLeavesTheWhole> {
    use crate::arrangement::WhatSmoozyDid;
    match state {
        WhatSmoozyDid::LeftAlone => Some(WhetherItLeavesTheWhole::LeftAlone),
        WhatSmoozyDid::CannotLand => Some(WhetherItLeavesTheWhole::CannotLand),
        WhatSmoozyDid::SkippedByNarrowing => Some(WhetherItLeavesTheWhole::SkippedByNarrowing),
        WhatSmoozyDid::NeverComing => {
            Some(WhetherItLeavesTheWhole::ItStaysInTheWholeAndIsTheShortfall)
        }
        WhatSmoozyDid::Fetched => Some(WhetherItLeavesTheWhole::ItStaysCountedAsFetched),
        WhatSmoozyDid::Owed => None,
    }
}

/// **N8: the arrangement's whole has no byte bar, and this is the one declared
/// divergence from `File Browser` U11's pair, with its reason.**
///
/// **A byte total over files not yet written does not exist.** The manual
/// whole is counted in files and drawn in bytes, both totals existing before
/// the first transfer starts; the arrangement's whole is counted in files
/// alone.
pub const AN_ARRANGEMENT_HAS_NO_BYTE_TOTAL_BECAUSE_ONE_DOES_NOT_EXIST: bool = true;

/// **§13: a format the version rendering it cannot deliver is permanently
/// unresolved, and no answer is owed here or anywhere.**
///
/// It is a Blender question, checked 2026-08-12 and standing: **every optional
/// format is compiled into every official build on both machines the project
/// could ask, so the case was never produced.** Producing it wants a Blender
/// built without one of the optional formats, which is a further Blender
/// question, and the Creator ruled on 2026-08-31 that there will be no further
/// Blender research and no further rented machine for answering a question.
/// **So this is not a pending item and no space is left in this crate or in
/// its record for the answer.**
///
/// **What breaks if it comes out the wrong way**: N1's format list would name
/// a format that never arrives, and a narrowed arrangement's whole would carry
/// entries that can never be fetched.
///
/// **What bounds the damage**: the three settled neighbouring cases, and this
/// document's own ending discipline — the absent files become *never coming*
/// at the ending walk, the arrangement ends and reads **ended, short of its
/// whole**, saying so.
pub const ROW_214_IS_PERMANENTLY_UNRESOLVED_AND_IS_NOT_A_PENDING_ITEM: bool = true;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use smoozy_dependencies::remap::{PlacedCopy, PlacedCopyList};
    use smoozy_job_files::inventory::{AnEntry, WhichRouteProducedTheName};
    use smoozy_job_files::reading::{
        ArtistsText, Destination, DestinationKind, Format, WhereItComesFrom, WhichScene,
    };

    pub(crate) fn a_destination(format: Option<&str>) -> Destination {
        Destination {
            kind: DestinationKind::TheRegularOutputPath,
            recorded_location: None,
            format: format.map(|name| Format {
                as_the_file_records_it: ArtistsText::of(name),
                the_ending_it_writes: None,
                is_video_shaped: false,
            }),
            origin: WhereItComesFrom::TheScenesOutputPath,
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        }
    }

    pub(crate) fn an_entry(path: &[u8], frame: Option<i64>, destination: usize) -> AnEntry {
        AnEntry {
            predicted_path: path.to_vec(),
            frame,
            the_route_that_produced_the_name: WhichRouteProducedTheName::ItWasAskedOfBlender,
            the_destinations_aiming_at_it: vec![destination],
            may_grow: false,
        }
    }

    pub(crate) fn an_inventory(entries: Vec<AnEntry>) -> TheInventory {
        TheInventory {
            entries,
            added_write_place_folders: Vec::new(),
            collision_record: Vec::new(),
            a_destination_is_video_shaped: false,
            why_it_owes_nothing: None,
        }
    }

    fn four_values(size: u64, inode: &str, modified: i64, changed: i64) -> SourceReading {
        SourceReading {
            size,
            inode: inode.to_string(),
            modification_time: modified,
            change_time: changed,
        }
    }

    #[test]
    fn the_formats_offered_are_the_destinations_own_deduplicated() {
        let reading = TheOutputsHalf {
            destinations: vec![
                a_destination(Some("PNG")),
                a_destination(Some("OPEN_EXR")),
                a_destination(Some("PNG")),
                a_destination(None),
            ],
            ..a_reading()
        };
        assert_eq!(
            the_formats_offered(&reading),
            vec![b"PNG".to_vec(), b"OPEN_EXR".to_vec()]
        );
    }

    fn a_reading() -> TheOutputsHalf {
        TheOutputsHalf {
            taken_by: smoozy_job_files::reading::TheVersionInHand::of(4, 5),
            gating: smoozy_job_files::reading::GatingConditions {
                the_effects_on_for_render: None,
                compositing_switch: false,
                a_compositing_tree_is_present: false,
                a_live_final_output_node: false,
                the_final_output_node_is_muted: false,
                four_fives_use_nodes_switch: None,
                save_output_switch: None,
                the_render_goes_out_through_a_sequencer_holding_a_scene_strip: false,
            },
            destinations: Vec::new(),
            scenes_reached_through_a_sequencer_strip: Vec::new(),
            the_tree_is_shared_with: None,
            per_frame: Vec::new(),
            recorded_locations: Vec::new(),
            views: smoozy_job_files::reading::HowManyViewsTheSceneWrites::One,
            the_beauty_walk: smoozy_job_files::main_output::WhatTheWalkFound::Reached {
                destinations: Vec::new(),
                it_reached_the_final_output_node: false,
            },
            the_view_transform: None,
        }
    }

    #[test]
    fn until_the_material_lands_there_is_no_list_and_the_arrangement_covers_everything() {
        let none = TheNarrowingMaterial::ItHasNotLanded;
        assert!(none.the_formats_it_offers().is_empty());
        assert!(!none.a_narrowing_may_be_offered());
    }

    #[test]
    fn a_file_with_no_format_is_never_filtered_out_by_a_narrowing() {
        let destinations = vec![a_destination(None)];
        assert_eq!(
            what_the_narrowing_says_about(
                &TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]),
                &destinations,
                &[0]
            ),
            WhatTheNarrowingSaysAbout::TheCutCannotSpeakAboutIt
        );
        // And an entry naming no destination at all is the same case.
        assert_eq!(
            what_the_narrowing_says_about(
                &TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]),
                &destinations,
                &[]
            ),
            WhatTheNarrowingSaysAbout::TheCutCannotSpeakAboutIt
        );
    }

    #[test]
    fn the_cut_selects_on_the_inventorys_destination_and_never_on_a_name() {
        let destinations = vec![a_destination(Some("PNG")), a_destination(Some("OPEN_EXR"))];
        let narrowing = TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]);
        assert_eq!(
            what_the_narrowing_says_about(&narrowing, &destinations, &[0]),
            WhatTheNarrowingSaysAbout::ItIsOneOfTheChosenFormats
        );
        assert_eq!(
            what_the_narrowing_says_about(&narrowing, &destinations, &[1]),
            WhatTheNarrowingSaysAbout::ItIsNotOneOfTheChosenFormats
        );
        // Everything is no filter, so it chooses whatever has a format.
        assert_eq!(
            what_the_narrowing_says_about(&TheNarrowing::Everything, &destinations, &[1]),
            WhatTheNarrowingSaysAbout::ItIsOneOfTheChosenFormats
        );
    }

    #[test]
    fn the_whole_is_the_narrowed_entries_and_a_file_with_no_format_stays_in_it() {
        let destinations = vec![
            a_destination(Some("PNG")),
            a_destination(Some("OPEN_EXR")),
            a_destination(None),
        ];
        let inventory = an_inventory(vec![
            an_entry(b"a.png", Some(1), 0),
            an_entry(b"b.exr", Some(1), 1),
            an_entry(b"c.abc", None, 2),
        ]);
        let narrowed = the_whole(
            &inventory,
            &destinations,
            &TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]),
        );
        // The EXR leaves; the one the cut cannot speak about does not.
        assert_eq!(narrowed.entries, 2);
        assert!(!narrowed.it_may_grow);
        assert_eq!(
            the_whole(&inventory, &destinations, &TheNarrowing::Everything).entries,
            3
        );
    }

    #[test]
    fn a_video_entry_makes_the_whole_say_it_may_grow() {
        let destinations = vec![a_destination(Some("FFMPEG"))];
        let mut entry = an_entry(b"anim.mp4", None, 0);
        entry.may_grow = true;
        let inventory = an_inventory(vec![entry]);
        let whole = the_whole(&inventory, &destinations, &TheNarrowing::Everything);
        assert_eq!(whole.entries, 1);
        assert!(whole.it_may_grow);
    }

    /// **N6: the video statement is read over the entries the count counts.**
    ///
    /// A narrowing that cuts the video destination out leaves a whole with
    /// nothing in it that can grow, so the statement must not stand — the user
    /// would otherwise be told the total of what they are waiting for may
    /// change when every entry in it is a fixed frame.
    #[test]
    fn a_narrowed_whole_does_not_say_it_may_grow_where_the_video_entry_was_cut_out() {
        let destinations = vec![a_destination(Some("PNG")), a_destination(Some("FFMPEG"))];
        let mut video = an_entry(b"anim.mp4", None, 1);
        video.may_grow = true;
        let inventory = an_inventory(vec![an_entry(b"a.png", Some(1), 0), video]);

        let narrowed = the_whole(
            &inventory,
            &destinations,
            &TheNarrowing::TheseFormats(vec![b"PNG".to_vec()]),
        );
        assert_eq!(narrowed.entries, 1);
        assert!(!narrowed.it_may_grow);

        // Unnarrowed, the same inventory says it may grow.
        assert!(the_whole(&inventory, &destinations, &TheNarrowing::Everything).it_may_grow);
    }

    #[test]
    fn never_coming_stays_in_the_whole_and_the_three_that_leave_are_the_three_n7_names() {
        use crate::arrangement::WhatSmoozyDid;
        assert!(whether_it_leaves_the_whole(&WhatSmoozyDid::LeftAlone)
            .unwrap()
            .leaves_the_whole());
        assert!(whether_it_leaves_the_whole(&WhatSmoozyDid::CannotLand)
            .unwrap()
            .leaves_the_whole());
        assert!(
            whether_it_leaves_the_whole(&WhatSmoozyDid::SkippedByNarrowing)
                .unwrap()
                .leaves_the_whole()
        );
        assert!(!whether_it_leaves_the_whole(&WhatSmoozyDid::NeverComing)
            .unwrap()
            .leaves_the_whole());
        assert!(!whether_it_leaves_the_whole(&WhatSmoozyDid::Fetched)
            .unwrap()
            .leaves_the_whole());
        assert!(whether_it_leaves_the_whole(&WhatSmoozyDid::Owed).is_none());
    }

    #[test]
    fn the_placed_copy_is_skipped_only_where_its_four_values_have_not_moved() {
        let at_placement = four_values(4096, "77", 100, 100);
        let placed = PlacedCopyList {
            copies: vec![PlacedCopy {
                job_folder_path: smoozy_dependencies::bytes::MachineLocation::composed(
                    b"/smoozy/jobs/j/outputs/cache/fluid".to_vec(),
                ),
                four_values_at_placement: at_placement.clone(),
            }],
        };
        assert_eq!(
            whether_it_is_skipped_on_the_way_home(
                Some(&placed),
                b"/smoozy/jobs/j/outputs/cache/fluid",
                &at_placement
            ),
            WhetherItIsSkippedOnTheWayHome::ItIsTheCopySmoozyPlaced
        );
        // Written over: the render wrote there, so it comes home.
        assert_eq!(
            whether_it_is_skipped_on_the_way_home(
                Some(&placed),
                b"/smoozy/jobs/j/outputs/cache/fluid",
                &four_values(9000, "77", 300, 300)
            ),
            WhetherItIsSkippedOnTheWayHome::ItComesHomeLikeEverythingElse
        );
        // Written beside: not a path Smoozy placed.
        assert_eq!(
            whether_it_is_skipped_on_the_way_home(
                Some(&placed),
                b"/smoozy/jobs/j/outputs/cache/fluid_0002",
                &at_placement
            ),
            WhetherItIsSkippedOnTheWayHome::ItComesHomeLikeEverythingElse
        );
    }

    #[test]
    fn where_the_placement_record_is_gone_nothing_is_skipped() {
        assert_eq!(
            whether_it_is_skipped_on_the_way_home(
                None,
                b"/smoozy/jobs/j/outputs/cache/fluid",
                &four_values(4096, "77", 100, 100)
            ),
            WhetherItIsSkippedOnTheWayHome::ItComesHomeLikeEverythingElse
        );
    }
}

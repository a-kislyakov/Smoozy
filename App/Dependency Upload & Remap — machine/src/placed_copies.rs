//! **§6.7 on the machine — M22a, M22c and M22f.** An uploaded cache or bake is
//! copied from the asset area to the job-folder place `Job files and outputs`
//! D20 gives its write place, before the render starts, and each copy's four
//! values are read the moment it is made.
//!
//! # What is here, and what is not
//!
//! **The instruction, where it goes, and the record of what was placed** — all
//! of them values. **The copy itself is the render worker's**, because this
//! crate reads no disk (its own `Cargo.toml`), and the render worker is the
//! one that already holds the map in force at the moment the copy is owed.
//!
//! # Why the instruction names a write place and not a folder
//!
//! **The job-folder value D20 assigns is composed on the machine**, by the
//! render worker's step 4, out of the reading in force — or taken from the
//! manifest where the desktop read the file first and nothing changed since.
//! So the desktop cannot know it at submission, and **the instruction says
//! which write place the bake belongs to and what that place recorded when the
//! check read it**, and the machine finds the place's folder in the map in
//! force (`Job files and outputs` P10: a pair for one of the four is keyed to
//! the named setting). **Until 2026-09-29 the instruction carried the
//! job-folder path itself**, written before step 4 moved the map to the
//! machine, and nothing had ever composed it.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` D20, L7a, P10.
//! `Download` N5a. `Server protocol and transfers` S13.

use serde::{Deserialize, Serialize};
use smoozy_job_files_machine::mapping::{HowItMapped, TheMap};
use smoozy_job_files_machine::reading::AnAddedWritePlace;

use crate::bytes::{MachineLocation, RecordedLocation};
use crate::check::Kind;

/// **Which of D20's four write places a kind of reference is**, where it is
/// one — the four kinds `crate::remap`'s own list says the redirect assigns
/// (M22a), and nothing else.
pub fn the_write_place_a_kind_is(kind: Kind) -> Option<AnAddedWritePlace> {
    match kind {
        Kind::BakeFolder => Some(AnAddedWritePlace::TheBakeOutputPath),
        Kind::PhysicsPointCache => Some(AnAddedWritePlace::ThePointCache),
        Kind::GeometryNodeBakeFolder => Some(AnAddedWritePlace::TheGeometryNodesBakeFolder),
        Kind::ProxyFolder => Some(AnAddedWritePlace::TheSequencerProxyFolder),
        _ => None,
    }
}

/// **M22c's instruction: an uploaded bake, and the write place its copy goes
/// to, before the render starts.**
///
/// **It rides the remap material** (`crate::remap::RemapMaterial`), which
/// the render worker already reads at its step 2 — so the instruction is
/// immutable with the job and needs no connection, the same as the pairs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheCopyTheMachineOwes {
    /// **The mirror the bake was sent to** (A4): the upload still lands in the
    /// asset area first, immutable and shared. A file or a folder.
    pub from: MachineLocation,
    /// The write place whose job-folder value the copy goes to (M22a).
    pub place: AnAddedWritePlace,
    /// **What that place recorded when the check read it**, which is how the
    /// place is found in the map in force: the reading and the check read the
    /// same setting's bytes (`Job files and outputs` P10).
    pub recorded: RecordedLocation,
}

impl TheCopyTheMachineOwes {
    /// **M22b: where no bake was sent, nothing changes at all** — D20's value
    /// stands, the render computes, and the result comes home — so no copy is
    /// owed and none is composed.
    pub fn where_a_bake_was_sent(
        the_mirror_the_bake_went_to: Option<&MachineLocation>,
        place: AnAddedWritePlace,
        recorded: &RecordedLocation,
    ) -> Option<TheCopyTheMachineOwes> {
        Some(TheCopyTheMachineOwes {
            from: the_mirror_the_bake_went_to?.clone(),
            place,
            recorded: recorded.clone(),
        })
    }

    /// **Where the copy goes: the folder the map in force gives this write
    /// place**, for the destination whose recorded location is the one the
    /// check read.
    ///
    /// **Every folder is answered once**, since two scenes recording the same
    /// place the same way resolve to one folder. **None at all is an answer and
    /// not a gap**: the file changed after the check, or the place was found
    /// not mappable — and the attempt then ends naming the copy, because a bake
    /// sent and never placed is a bake the render never reads, which nothing
    /// else would say.
    pub fn where_it_goes(&self, map: &TheMap) -> Vec<Vec<u8>> {
        let found = self.the_folders_for(map, Some(self.recorded.as_bytes()));
        // **G6, added 2026-10-02: a physics cache the check found at its
        // default folder beside the `.blend`** was listed as `//blendcache_…/`
        // because the cache itself records nothing; on the machine its place
        // is the point cache's folder for the caches that record no location
        // (`Job files and outputs` P7a).
        if found.is_empty()
            && self.place == AnAddedWritePlace::ThePointCache
            && is_a_default_cache_folder(self.recorded.as_bytes())
        {
            return self.the_folders_for(map, None);
        }
        found
    }

    fn the_folders_for(&self, map: &TheMap, wanted: Option<&[u8]>) -> Vec<Vec<u8>> {
        let mut folders: Vec<Vec<u8>> = Vec::new();
        for pair in &map.pairs {
            if pair.destination.as_an_added_write_place() != Some(self.place) {
                continue;
            }
            let recorded = pair
                .destination
                .recorded_location
                .as_ref()
                .map(|location| location.as_bytes());
            if recorded != wanted {
                continue;
            }
            if let HowItMapped::UnderTheJobsFolder { new_folder, .. } = &pair.outcome {
                if !folders.contains(new_folder) {
                    folders.push(new_folder.clone());
                }
            }
        }
        folders
    }

    /// **The copy, in standard software words**, for the attempt's reason
    /// where it could not be made: the place, and the mirror it comes from.
    ///
    /// **The place's own words begin with *the***, so this sentence does not
    /// add one. Until 2026-10-02 it read *"the uploaded the geometry nodes bake
    /// folder"* (Rule 9).
    ///
    /// **Shortened 2026-11-04** (`Design/Queue & History`): it read *the files
    /// uploaded for the geometry nodes bake folder (/ephemeral/…)*. A user reads it
    /// at once now, so it names the folder as their file names it, and never
    /// Smoozy's own copy of it. A geometry-nodes bake is just *the bake folder*.
    pub fn in_words(&self) -> String {
        let place = match self.place {
            AnAddedWritePlace::TheGeometryNodesBakeFolder => "the bake folder",
            other => other.in_words(),
        };
        format!("{place} {}", String::from_utf8_lossy(self.recorded.as_bytes()))
    }
}

/// **`Job files and outputs` D21b-i, added 2026-10-02: a baked physics cache
/// whose files are on the machine where the file records them — uploaded
/// beside the `.blend` — copied to its job-folder value before the render, as
/// a sent bake is (M22c), and recorded the same way (M22f).**
///
/// **Composed by the machine from its own reading**, never sent: the reading
/// at the render's start lists every disk cache the file marks baked whose
/// folder is on this machine (`Job files and outputs — machine`'s
/// `ABakedCacheOnTheMachine`). A cache marked baked whose files are not where
/// its setting then looks gives the object as it starts, unmoving, on every
/// frame — so without this copy a baked cache uploaded with the project is
/// never read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheCopyOfABakeOnTheMachine {
    /// Where the cache's files sit on the machine.
    pub from: MachineLocation,
    /// What the cache's setting records, absent where it records none — which
    /// is how its job-folder value is found in the map in force.
    pub recorded: Option<Vec<u8>>,
}

impl TheCopyOfABakeOnTheMachine {
    /// **One copy per folder the reading found**, in the reading's order; a
    /// folder two caches share is copied once.
    pub fn the_copies_of(
        found: &[smoozy_job_files_machine::reading::ABakedCacheOnTheMachine],
    ) -> Vec<TheCopyOfABakeOnTheMachine> {
        let mut copies: Vec<TheCopyOfABakeOnTheMachine> = Vec::new();
        for cache in found {
            let copy = TheCopyOfABakeOnTheMachine {
                from: MachineLocation::composed(cache.its_files_sit_in.as_bytes().to_vec()),
                recorded: cache
                    .recorded_location
                    .as_ref()
                    .map(|location| location.as_bytes().to_vec()),
            };
            if !copies.contains(&copy) {
                copies.push(copy);
            }
        }
        copies
    }

    /// **The folder the map in force gives the point cache for this cache's
    /// recorded location** — the same lookup a sent copy makes, with the
    /// location absent where the cache records none (`Job files and outputs`
    /// R4 item 2). None at all is an answer and not a gap, as for a sent copy.
    pub fn where_it_goes(&self, map: &TheMap) -> Vec<Vec<u8>> {
        let mut folders: Vec<Vec<u8>> = Vec::new();
        for pair in &map.pairs {
            if pair.destination.as_an_added_write_place() != Some(AnAddedWritePlace::ThePointCache) {
                continue;
            }
            let recorded = pair
                .destination
                .recorded_location
                .as_ref()
                .map(|location| location.as_bytes());
            if recorded != self.recorded.as_deref() {
                continue;
            }
            if let HowItMapped::UnderTheJobsFolder { new_folder, .. } = &pair.outcome {
                if !folders.contains(new_folder) {
                    folders.push(new_folder.clone());
                }
            }
        }
        folders
    }

    /// **The copy, in standard software words**, for the attempt's reason
    /// where it could not be made.
    ///
    /// **Shortened 2026-11-04, the same kind of sentence as the sent copy's**: the
    /// folder as the file records it, or Smoozy's own path only where it records none.
    pub fn in_words(&self) -> String {
        match &self.recorded {
            Some(recorded) => format!(
                "{} {}",
                AnAddedWritePlace::ThePointCache.in_words(),
                String::from_utf8_lossy(recorded)
            ),
            None => format!(
                "{} at {}",
                AnAddedWritePlace::ThePointCache.in_words(),
                String::from_utf8_lossy(self.from.as_bytes())
            ),
        }
    }
}

/// **G6, added 2026-10-02: the form the check gives a physics cache found at
/// its default folder** — `//blendcache_` and a name, one folder beside the
/// `.blend` (`Job files and outputs` D21c) — and nothing deeper.
fn is_a_default_cache_folder(recorded: &[u8]) -> bool {
    let Some(rest) = recorded.strip_prefix(b"//blendcache_") else {
        return false;
    };
    let name = rest.strip_suffix(b"/").unwrap_or(rest);
    !name.is_empty() && !name.contains(&b'/')
}

/// **One file the copy placed, as M22f records it**: its exact job-folder path
/// and the four values read the moment it was made — size, inode, modification
/// time and change time, the one shape `Server protocol and transfers` S13
/// already uses.
///
/// **Per file and not per bake**, because `Download` N5a skips by path and by
/// the four values at it, and a baked folder is many files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APlacedFile {
    pub job_folder_path: MachineLocation,
    pub size: u64,
    pub inode: String,
    pub modification_time: i64,
    pub change_time: i64,
}

/// **M22f's placed-copy list, as the machine writes it for one attempt** —
/// into the job record directory, beside the recorded reading, for the
/// desktop to take into the job's own local record.
///
/// **Each attempt copies again**, because each attempt is a render start and
/// M22a places the bake *"before the render starts"*: a copy an earlier
/// attempt's render wrote over is placed afresh, and its four values are new.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ThePlacedCopies {
    pub files: Vec<APlacedFile>,
}

impl ThePlacedCopies {
    pub fn as_written(&self) -> Vec<u8> {
        serde_json::to_vec_pretty(self).expect("the list is composed of values that always render")
    }

    pub fn read_back(bytes: &[u8]) -> Option<ThePlacedCopies> {
        serde_json::from_slice(bytes).ok()
    }
}

/// Where one attempt's placed-copy list is written, inside the job record
/// directory.
pub fn the_placed_copies_file_name(attempt: u32) -> String {
    format!("placed-copies-{attempt}.json")
}

/// **The same list read again the moment the attempt's render ended**, with
/// each placed file's four values as they stand then — written beside the
/// list, in the same shape. Added 2026-10-02, the fixes before 0.1.8, fault 3.
///
/// **It is what `Download` N5a compares against.** N5a skips a placed copy
/// whose four values still match the ones read when it was made, and brings
/// home one the render wrote over; the file-transfer surface carries two of
/// the four and no more (`Server protocol and transfers` S13), so the
/// comparison's *now* is read here, on the machine, by Smoozy's own program.
/// **After the render has ended nothing writes into a job's folder**, so a
/// reading taken then is the reading the ending walk would take.
///
/// **A file the render took away is left out**, and the list then names a path
/// the walk never meets.
pub fn the_placed_copies_after_the_render_file_name(attempt: u32) -> String {
    format!("placed-copies-{attempt}-after-the-render.json")
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_job_files_machine::mapping::APair;
    use smoozy_job_files_machine::reading::{
        Destination, DestinationKind, RecordedLocation as TheReadingsLocation, WhereItComesFrom,
        WhichScene,
    };

    fn a_place(place: AnAddedWritePlace, recorded: Option<&[u8]>, new_folder: &[u8]) -> APair {
        APair {
            destination: Destination {
                kind: DestinationKind::AnAddedWritePlace(place),
                recorded_location: recorded.map(|bytes| TheReadingsLocation::of(bytes.to_vec())),
                format: None,
                origin: WhereItComesFrom::AnAddedWritePlaceNamed(place),
                scene: WhichScene::TheJobsOwnScene,
                the_view_layers_that_feed_it: Vec::new(),
                the_location_with_its_folder_replaced: None,
            },
            outcome: HowItMapped::UnderTheJobsFolder {
                new_folder: new_folder.to_vec(),
                further_values: Vec::new(),
            },
        }
    }

    fn owed(place: AnAddedWritePlace, recorded: &[u8]) -> TheCopyTheMachineOwes {
        TheCopyTheMachineOwes::where_a_bake_was_sent(
            Some(&MachineLocation::composed(b"/root/assets/files/a/bake".to_vec())),
            place,
            &RecordedLocation::as_the_reading_carried_it(recorded),
        )
        .expect("a bake was sent")
    }

    /// **Four bakes sent from four folders land in four folders** — `Job
    /// files and outputs` R4 item 2, corrected 2026-10-02: the map holds one
    /// destination per recorded location, and each copy finds the folder of
    /// its own. Until then the map held one per place, and the second copy
    /// found none: *"this job's folder has no place for //bakePath"*.
    #[test]
    fn four_bakes_sent_from_four_folders_land_in_four_folders() {
        let place = AnAddedWritePlace::TheGeometryNodesBakeFolder;
        let recorded: [&[u8]; 4] =
            [b"//config-on-sim-node", b"//bakePath", b"//set-on-node", b"//only-set-on-modifier"];
        let mut pairs: Vec<APair> = recorded
            .iter()
            .map(|one| {
                let mut folder = b"/job/".to_vec();
                folder.extend_from_slice(&one[2..]);
                a_place(place, Some(one), &folder)
            })
            .collect();
        pairs.push(a_place(place, None, b"/job/geometry-nodes-bake"));
        let map = TheMap { pairs };
        let mut landed = Vec::new();
        for one in recorded {
            let copy = TheCopyTheMachineOwes {
                from: MachineLocation::composed(b"/root/assets/files/a".to_vec()),
                place,
                recorded: RecordedLocation::as_the_reading_carried_it(one),
            };
            let folders = copy.where_it_goes(&map);
            assert_eq!(folders.len(), 1, "{}", String::from_utf8_lossy(one));
            landed.push(folders[0].clone());
        }
        landed.dedup();
        assert_eq!(landed.len(), 4);
        assert!(!landed.contains(&b"/job/geometry-nodes-bake".to_vec()));
    }

    /// **`Job files and outputs` D21b-i, added 2026-10-02: a baked cache found
    /// on the machine goes to the point cache's folder for its own recorded
    /// location** — the folder for the caches that record none where it
    /// records none — and a folder two caches share is copied once.
    #[test]
    fn a_baked_cache_found_on_the_machine_goes_to_its_own_locations_folder() {
        use smoozy_job_files_machine::reading::ABakedCacheOnTheMachine;
        let place = AnAddedWritePlace::ThePointCache;
        let map = TheMap {
            pairs: vec![
                a_place(place, Some(b"//cloth_cache/"), b"/job/cloth_cache"),
                a_place(place, None, b"/job/pointcache"),
                a_place(AnAddedWritePlace::TheBakeOutputPath, None, b"/job/bake"),
            ],
        };
        let beside = ABakedCacheOnTheMachine {
            recorded_location: None,
            its_files_sit_in: TheReadingsLocation::of(b"/up/blendcache_x".to_vec()),
        };
        let external = ABakedCacheOnTheMachine {
            recorded_location: Some(TheReadingsLocation::of(b"//cloth_cache/".to_vec())),
            its_files_sit_in: TheReadingsLocation::of(b"/up/cloth_cache".to_vec()),
        };
        let copies = TheCopyOfABakeOnTheMachine::the_copies_of(&[
            beside.clone(),
            external,
            beside,
        ]);
        assert_eq!(copies.len(), 2, "a folder two caches share is copied once");
        assert_eq!(copies[0].where_it_goes(&map), vec![b"/job/pointcache".to_vec()]);
        assert_eq!(copies[1].where_it_goes(&map), vec![b"/job/cloth_cache".to_vec()]);
    }

    /// **G6, added 2026-10-02: a physics cache the check found at its default
    /// folder goes to the point cache's folder for the caches that record no
    /// location** — the cache itself records nothing, so the map holds no
    /// destination for `//blendcache_…/`. Before, the copy found no folder and
    /// the attempt would have ended: the check never listed such a cache.
    #[test]
    fn a_cache_sent_from_its_default_folder_goes_to_the_folder_for_caches_that_record_none() {
        let map = TheMap {
            pairs: vec![
                a_place(AnAddedWritePlace::ThePointCache, Some(b"//cloth_cache/"), b"/job/cloth_cache"),
                a_place(AnAddedWritePlace::ThePointCache, None, b"/job/pointcache"),
            ],
        };
        let sent = owed(AnAddedWritePlace::ThePointCache, b"//blendcache_scene/");
        assert_eq!(sent.where_it_goes(&map), vec![b"/job/pointcache".to_vec()]);
        // **Only the default form, and only for the point cache.**
        let elsewhere = owed(AnAddedWritePlace::ThePointCache, b"//blendcache_scene/deeper/");
        assert!(elsewhere.where_it_goes(&map).is_empty());
        let not_a_cache = owed(AnAddedWritePlace::TheGeometryNodesBakeFolder, b"//blendcache_scene/");
        assert!(not_a_cache.where_it_goes(&map).is_empty());
        // **A cache that records its own folder still goes to that folder.**
        let own = owed(AnAddedWritePlace::ThePointCache, b"//cloth_cache/");
        assert_eq!(own.where_it_goes(&map), vec![b"/job/cloth_cache".to_vec()]);
    }

    /// **The copy's words name the place once, with one *the*** (Rule 9).
    #[test]
    fn the_copys_words_say_the_place_once() {
        let copy = TheCopyTheMachineOwes {
            from: MachineLocation::composed(b"/root/assets/files/a/bakePath".to_vec()),
            place: AnAddedWritePlace::TheGeometryNodesBakeFolder,
            recorded: RecordedLocation::as_the_reading_carried_it(b"//bakePath"),
        };
        assert_eq!(copy.in_words(), "the bake folder //bakePath");
        assert!(!copy.in_words().contains("the the"));
    }

    /// **The folder as the file records it, and Smoozy's own path only where the
    /// file records none** (`Design/Queue & History`, 2026-11-04).
    #[test]
    fn a_copys_words_name_the_folder_as_the_file_does() {
        let sent = TheCopyTheMachineOwes {
            from: MachineLocation::composed(b"/ephemeral/assets/x".to_vec()),
            place: AnAddedWritePlace::TheBakeOutputPath,
            recorded: RecordedLocation::as_the_reading_carried_it(b"//bake/"),
        };
        assert_eq!(sent.in_words(), "the bake output folder //bake/");
        let recorded = TheCopyOfABakeOnTheMachine {
            from: MachineLocation::composed(b"/ephemeral/x/cloth".to_vec()),
            recorded: Some(b"//cloth/".to_vec()),
        };
        assert_eq!(recorded.in_words(), "the physics cache folder //cloth/");
        let by_default = TheCopyOfABakeOnTheMachine {
            from: MachineLocation::composed(b"/ephemeral/x/blendcache_t".to_vec()),
            recorded: None,
        };
        assert_eq!(by_default.in_words(), "the physics cache folder at /ephemeral/x/blendcache_t");
    }

    /// **M22a: the four kinds the redirect assigns, and only those, are a write
    /// place a copy can go to.**
    #[test]
    fn the_four_kinds_the_redirect_assigns_are_the_four_write_places() {
        let places: Vec<AnAddedWritePlace> = crate::check::Kind::ALL
            .iter()
            .filter_map(|kind| the_write_place_a_kind_is(*kind))
            .collect();
        assert_eq!(places.len(), 4);
        for place in smoozy_job_files_machine::reading::EVERY_ADDED_WRITE_PLACE {
            // **A Replay fluid domain's cache folder takes no copy**:
            // `Job files and outputs` D20 adds it as a fifth write place and
            // says no copy is placed at it, so a fluid cache the check finds
            // is a remap pair, as before.
            if place == AnAddedWritePlace::TheReplayFluidCacheFolder {
                assert!(!places.contains(&place));
                continue;
            }
            assert!(places.contains(&place), "{place:?}");
        }
    }

    /// **M22b: no bake sent, no copy owed.**
    #[test]
    fn no_copy_is_owed_where_no_bake_was_sent() {
        assert_eq!(
            TheCopyTheMachineOwes::where_a_bake_was_sent(
                None,
                AnAddedWritePlace::TheBakeOutputPath,
                &RecordedLocation::as_the_reading_carried_it(b"//bake/"),
            ),
            None
        );
    }

    /// **The copy goes to the map's folder for its own place and its own
    /// recorded location**, once however many scenes name it, and to nothing
    /// where the map has no such place.
    #[test]
    fn the_copy_goes_to_the_folder_the_map_gives_its_own_place() {
        let map = TheMap {
            pairs: vec![
                a_place(AnAddedWritePlace::ThePointCache, Some(b"//bake/"), b"/j/out/pointcache"),
                a_place(AnAddedWritePlace::TheBakeOutputPath, Some(b"//other/"), b"/j/out/other"),
                a_place(AnAddedWritePlace::TheBakeOutputPath, Some(b"//bake/"), b"/j/out/bake"),
                a_place(AnAddedWritePlace::TheBakeOutputPath, Some(b"//bake/"), b"/j/out/bake"),
                a_place(AnAddedWritePlace::TheBakeOutputPath, None, b"/j/out/bake-unread"),
            ],
        };
        assert_eq!(
            owed(AnAddedWritePlace::TheBakeOutputPath, b"//bake/").where_it_goes(&map),
            vec![b"/j/out/bake".to_vec()]
        );
        assert!(owed(AnAddedWritePlace::TheGeometryNodesBakeFolder, b"//bake/")
            .where_it_goes(&map)
            .is_empty());
        assert!(owed(AnAddedWritePlace::TheBakeOutputPath, b"//gone/")
            .where_it_goes(&map)
            .is_empty());
    }

    /// **The instruction and the list both cross the disk whole.**
    #[test]
    fn the_instruction_and_the_list_read_back_as_written() {
        let copy = owed(AnAddedWritePlace::TheBakeOutputPath, b"//bake/");
        let text = serde_json::to_vec(&copy).expect("it renders");
        assert_eq!(serde_json::from_slice::<TheCopyTheMachineOwes>(&text).ok(), Some(copy));
        let list = ThePlacedCopies {
            files: vec![APlacedFile {
                job_folder_path: MachineLocation::composed(b"/j/out/bake/l.png".to_vec()),
                size: 3,
                inode: "7".to_string(),
                modification_time: 1,
                change_time: 2,
            }],
        };
        assert_eq!(ThePlacedCopies::read_back(&list.as_written()), Some(list));
        assert_eq!(the_placed_copies_file_name(2), "placed-copies-2.json");
    }
}

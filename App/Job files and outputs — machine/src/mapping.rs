//! §6 — bringing a destination under the job's folder.
//!
//! **P1: the mechanism the spec offered is taken as it stands** — take the
//! folder every destination writes into, throw away the run of folders they all
//! share, and keep what is left under the job's folder. **P2: no replacement
//! considered keeps the second promise as well.** Giving every destination a
//! folder named for its position loses the artist's arrangement entirely;
//! keeping the whole of every path buries the ordinary case. *"The shared-run
//! rule is the only one weighed that gives the ordinary file a flat folder and
//! still keeps two different places different."*
//!
//! **Everything here works on bytes.** A recorded location crosses as bytes
//! under R4a, real disks hold names that are not text, and a rule that decoded
//! one on the way through would key the wrong record and match no pair at the
//! remap.
//!
//! **P3: what the mechanism gains that is new here is the failure it did not
//! have.** The rule deepens paths by construction and the machine refuses a
//! name it will not take for its length **at open, before any byte is sent**, so
//! the check has to be Smoozy's own and has to happen before the render. **And
//! the added write places are part of what deepens them**: the shared run is
//! worked out over a larger set, so it can be shorter, so what is left under the
//! job's folder is deeper. P16 is unaffected by the larger set — only a prefix
//! identical to every destination is ever removed. ~~And the added write places
//! are part of what deepens them~~ — **no longer so from 2026-10-04** (D20, P6,
//! P9): the added places take no part in the shared run, so a default `//` on
//! the bake output path no longer deepens every job by one folder.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Dependency Upload & Remap` M4a.

use serde::{Deserialize, Serialize};

use crate::names::{a_name_fits, a_path_fits, WhyThePathIsRefused};
use crate::reading::{
    AnAddedWritePlace, Destination, DestinationKind, TheVersionInHand, WhichBranch, WhichScene,
};

/// The one separator every path below is split and joined on.
pub const SEPARATOR: u8 = b'/';

/// **The blend-file shorthand that opens a path written relative to the
/// `.blend`.**
///
/// The research names it under that name and `Dependency Upload & Remap` M4a
/// compares it verbatim — *"the leading `//` blend-file shorthand stays"*.
pub const THE_BLEND_FILE_SHORTHAND: &[u8] = b"//";

/// What a resolved folder is anchored to.
///
/// **P4's three forms collapse to two anchors**, because a path written
/// relative to the `.blend` is *"an ordinary path from then on"* once it is
/// resolved against the folder the `.blend` sits in on the rented machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Anchor {
    /// An ordinary absolute path on the rented machine.
    TheMachinesOwnRoot,
    /// **P4's foreign form.** A path written on another kind of computer *"is
    /// kept exactly as written and resolves to nothing on Linux"* — Blender
    /// *"does not translate it, does not rewrite it and does not refuse it"*,
    /// and a drive letter is an ordinary place name that happens to hold a
    /// colon. **Such a path is taken as its own tree, sharing at most the
    /// topmost folder and often nothing at all, and it keeps the whole of
    /// itself beneath.**
    ItsOwnTree,
}

/// **P4: every destination's folder resolved to a single absolute form before
/// the rule sees it, and the rule works on nothing else.**
///
/// **P5: after resolution no destination folder climbs, and the rule never has
/// to reason about one.** That is why P4 comes first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedFolder {
    pub anchor: Anchor,
    /// The whole folder names, in order. **Never letters** — P6 measures the
    /// shared run in whole names.
    pub segments: Vec<Vec<u8>>,
}

impl ResolvedFolder {
    /// The folder written back out as one path.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        if self.anchor == Anchor::TheMachinesOwnRoot {
            out.push(SEPARATOR);
        }
        for (at, segment) in self.segments.iter().enumerate() {
            if at > 0 {
                out.push(SEPARATOR);
            }
            out.extend_from_slice(segment);
        }
        out
    }
}

/// Split a path's bytes into whole names, dropping the empty runs a doubled
/// separator and a trailing separator leave.
fn whole_names(path: &[u8]) -> Vec<Vec<u8>> {
    path.split(|byte| *byte == SEPARATOR)
        .filter(|segment| !segment.is_empty())
        .map(<[u8]>::to_vec)
        .collect()
}

/// Take the `.` and `..` steps out, clamping at the anchor.
///
/// **P5 is what this is for.** A path written relative to the `.blend` *"may
/// climb above that folder and the resolution settles that too"*, and after
/// resolution nothing climbs — there is nowhere above the anchor to climb to,
/// so a step that would is dropped. **Build's own rule for the clamp**, taken
/// because P5 states the outcome and names no other treatment.
fn without_the_steps(segments: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    for segment in segments {
        if segment == b"." {
            continue;
        }
        if segment == b".." {
            out.pop();
            continue;
        }
        out.push(segment);
    }
    out
}

/// **P4, performed, for a folder.**
///
/// `the_folder_the_blend_sits_in` is that folder **on the rented machine**,
/// which is what a blend-file-shorthand path is resolved against.
pub fn resolve_a_folder(
    written: &[u8],
    the_folder_the_blend_sits_in: &[u8],
) -> ResolvedFolder {
    if written.starts_with(THE_BLEND_FILE_SHORTHAND) {
        let beneath = &written[THE_BLEND_FILE_SHORTHAND.len()..];
        let mut segments = whole_names(the_folder_the_blend_sits_in);
        segments.extend(whole_names(beneath));
        return ResolvedFolder {
            anchor: if the_folder_the_blend_sits_in.starts_with(&[SEPARATOR]) {
                Anchor::TheMachinesOwnRoot
            } else {
                Anchor::ItsOwnTree
            },
            segments: without_the_steps(segments),
        };
    }
    if written.starts_with(&[SEPARATOR]) {
        return ResolvedFolder {
            anchor: Anchor::TheMachinesOwnRoot,
            segments: without_the_steps(whole_names(written)),
        };
    }
    // **P4's foreign form, kept exactly as written.** Nothing is resolved,
    // nothing is tidied, and no step is taken out: a `..` inside a path that
    // means nothing on this machine is part of the name of its own tree.
    ResolvedFolder {
        anchor: Anchor::ItsOwnTree,
        segments: whole_names(written),
    }
}

/// **P4's third bullet: the regular output path is one piece of text holding
/// the folder and the front of the name together, and Blender itself splits it
/// at the last separator.**
///
/// **The rule works on the part before that separator.** *"A path ending in a
/// separator has no name front at all."*
///
/// **A separator inside the blend-file shorthand is not the last separator.**
/// `//f_` is the front `f_` in the `.blend`'s own folder, `//`, and never the
/// machine's root. Corrected 2026-10-04 by piece 1b: it split as `/` and `f_`,
/// and the folder resolved to the root of the machine.
pub fn split_the_regular_output_path(written: &[u8]) -> (&[u8], &[u8]) {
    match written.iter().rposition(|byte| *byte == SEPARATOR) {
        Some(at) if at < THE_BLEND_FILE_SHORTHAND.len()
            && written.starts_with(THE_BLEND_FILE_SHORTHAND) =>
        {
            (THE_BLEND_FILE_SHORTHAND, &written[THE_BLEND_FILE_SHORTHAND.len()..])
        }
        Some(at) => (&written[..at], &written[at + 1..]),
        // No separator at all: the whole text is the front of the name and the
        // folder is whatever the render is run from. Nothing here invents one.
        None => (&[], written),
    }
}

/// **Whether a destination's recorded location holds the folder and the front
/// of the file name together**, so Blender splits it at the last separator
/// and the rule works on the part before (P4's third bullet).
///
/// - **The regular output path**, in every version.
/// - **In 4.5, a file output node gathering into one multi-layer file.** Its
///   folder property is *"used as the start of the file name and not only as a
///   folder"* (D8). Added 2026-10-04 by piece 1b: until then the whole
///   property was taken for a folder and D8's separator was written after it,
///   so the artist's `//../Render/EXR/### - Hero - Box` wrote
///   `EXR/### - Hero - Box/0001.exr` where the file left alone writes
///   `EXR/001 - Hero - Box.exr` (observed at the desk in 4.5.12). From 5.0
///   that node's folder is a folder and its name is its base file name.
///
/// **The one place this is decided**: the map's folder ([`compose_the_map`]),
/// the name I2's recipe predicts ([`crate::inventory`]) and the value the
/// redirect writes ([`crate::redirect::compose_the_launch_material`]) all ask
/// [`the_name_front_of`], so the three cannot disagree.
pub fn it_holds_a_name_front(kind: &DestinationKind, version: TheVersionInHand) -> bool {
    match kind {
        DestinationKind::TheRegularOutputPath => true,
        DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { .. } => {
            version.branch() == WhichBranch::FourFive
        }
        DestinationKind::AnOutputOfAFileOutputNode { .. }
        | DestinationKind::AnAddedWritePlace(_) => false,
    }
}

/// **Whether a written folder opens with a place**: the blend-file shorthand or
/// a separator. Without one P4 calls it another computer's.
pub fn it_opens_with_a_place(written: &[u8]) -> bool {
    written.starts_with(THE_BLEND_FILE_SHORTHAND) || written.starts_with(&[SEPARATOR])
}

/// **The front of the file name a destination's recorded location holds**,
/// where [`it_holds_a_name_front`] says it holds one; nothing where the whole
/// location is a folder, or where none was recorded.
///
/// **Read off the location Blender writes into** — for 4.5's gathering node,
/// its folder property with its curly-brace expressions put in, where the
/// reading replaced them (`Job files and outputs` I2, added 2026-10-18;
/// [`Destination::the_location_blender_writes_into`]).
pub fn the_name_front_of(destination: &Destination, version: TheVersionInHand) -> Option<&[u8]> {
    if !it_holds_a_name_front(&destination.kind, version) {
        return None;
    }
    let location = destination.the_location_blender_writes_into()?;
    Some(split_the_regular_output_path(location).1)
}

/// **The mapped folder joined to the name front the artist wrote** — the value
/// a setting that holds both takes. The front is not touched (N12: Smoozy
/// writes no output name), and an empty front leaves the folder ended with a
/// separator, which is D8's own value.
pub fn the_mapped_folder_and_its_name_front(new_folder: &[u8], front: &[u8]) -> Vec<u8> {
    let mut out = new_folder.to_vec();
    out.push(SEPARATOR);
    out.extend_from_slice(front);
    out
}

/// **P6: the shared run is the longest run of whole folder names common to
/// every resolved destination folder in the set, measured in whole names and
/// never in letters.**
///
/// `/home/a/out` and `/home/a/output2` share `/home/a`. **Two folders with
/// different anchors share nothing**, which is what makes P4's foreign form
/// keep the whole of itself beneath.
pub fn the_shared_run(folders: &[ResolvedFolder]) -> Vec<Vec<u8>> {
    let Some(first) = folders.first() else {
        return Vec::new();
    };
    if folders.iter().any(|folder| folder.anchor != first.anchor) {
        return Vec::new();
    }
    let mut run = first.segments.clone();
    for folder in &folders[1..] {
        let common = run
            .iter()
            .zip(folder.segments.iter())
            .take_while(|(one, other)| one == other)
            .count();
        run.truncate(common);
    }
    run
}

/// **P7: each destination's new folder is the job output folder joined to what
/// is left of that destination's folder after the shared run is removed.**
///
/// *"A destination with nothing left lands at the job output folder itself."*
pub fn the_new_folder(
    job_output_folder: &str,
    folder: &ResolvedFolder,
    shared_run: &[Vec<u8>],
) -> Vec<u8> {
    let mut out = job_output_folder.as_bytes().to_vec();
    for segment in folder.segments.iter().skip(shared_run.len()) {
        out.push(SEPARATOR);
        out.extend_from_slice(segment);
    }
    out
}

/// **P12: why a destination the rule covers is recorded in the map as *not
/// mappable*. There is no third case, and the reading names which it is.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhyItIsNotMappable {
    /// **Case 1.** Its predicted deepest file path exceeds the whole-path
    /// budget, or one of its segments exceeds the name budget (N5, N8).
    ItsPredictedDeepestFilePathWillNotFit(WhyThePathIsRefused),
    /// **Case 2, and it is 4.5's alone.** An output's name climbs above the
    /// node's folder. *"Moving the folder does not move that output with it,
    /// and Smoozy does not touch output names, so the redirect cannot carry
    /// it."* In 5.0 every separator in a name becomes an underscore and every
    /// file lands flat in the node's folder, and in 5.1 and 5.2 a name cannot
    /// climb out.
    AnOutputNameClimbsAboveItsNodesFolderInFourFive,
}

/// **P10's *further values*: where a place takes more than a folder to be set,
/// the further values that place requires go with its folder and are applied
/// with it.**
///
/// **D21b-i, and it was corrected when it was run.** For the physics point
/// cache it is three values together, the folder among them. *"`use_external`
/// is the value that decides it, and without it the assignment is a silent
/// no-op"* — assigning the folder alone with the disk-cache switch on was read
/// back exactly as assigned, the bake ran, **the render finished and reported
/// success**, and the cache was written beside the `.blend`: 250 files, outside
/// the job's folder, uncounted, destroyed with the machine.
///
/// **It was four until 2026-10-02, the disk-cache switch first** — and writing
/// that switch on a cache where it is already on deletes every file the cache
/// holds (D21b-i's correction of that date).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FurtherValue {
    /// The setting's own name, as the place's own vocabulary spells it.
    pub setting: String,
    /// What it is set to.
    pub value: FurtherValueIs,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FurtherValueIs {
    On,
    /// **A text value.** Until 2026-10-02 this carried the one stem every
    /// cache file was given (`smoozy`). Nothing composes it now — each cache
    /// keeps its own stem, written by the material from the cache itself
    /// ([`the_stem_setting_for`]) — and it stays so that a map recorded before
    /// that date still reads.
    NonEmptyText(String),
}

/// The values D21b-i names for the physics point cache, less the folder,
/// which is the pair's own, and less the name, which is each cache's own
/// ([`the_stem_setting_for`]): *external* on.
///
/// **The spellings are the research's own**, quoted at D21b-i, and not composed
/// here: `use_external`, `filepath` and `name`.
///
/// **The disk-cache switch, `use_disk_cache`, is never among them. Corrected
/// 2026-10-02 (D21b-i).** Writing it on a cache where it is already on deletes
/// every file that cache holds — 48 baked files beside the `.blend` became none
/// at that step, at the desk in 4.5.12 and 5.2.2 — and writing it on a cache
/// the file keeps in memory drops that cache's in-file bake. The material sets
/// these values only on a cache whose switch the file already has on, so the
/// switch is the file's own and nothing here touches it.
///
/// **And the name is not one stem for every cache. Corrected 2026-10-02
/// (D21b-i).** A baked cache copied into the job's folder is read under its
/// own stem and not under another: under `smoozy` the cloth's frame 24 was
/// wrong on 35% of pixels, with Blender's cache line reading *"No valid data
/// to read!"*; and two caches given one stem write into one another's files.
pub fn the_further_values_for(place: AnAddedWritePlace) -> Vec<FurtherValue> {
    match place {
        AnAddedWritePlace::ThePointCache => vec![FurtherValue {
            setting: "point_cache.use_external".to_string(),
            value: FurtherValueIs::On,
        }],
        // **The other three take a folder and nothing else that this project
        // has established.** D21d records the attempts rather than a
        // conclusion, and §14 item 22 closes the fact permanently unresolved,
        // so nothing is invented for them here.
        AnAddedWritePlace::TheBakeOutputPath
        | AnAddedWritePlace::TheGeometryNodesBakeFolder
        | AnAddedWritePlace::TheSequencerProxyFolder => Vec::new(),
        // **The Replay fluid cache folder takes its folder alone** (D20's
        // fifth place, D1a): set as the file loads, the folder was read and
        // written with nothing beside it, at the desk on 2026-10-02.
        AnAddedWritePlace::TheReplayFluidCacheFolder => Vec::new(),
    }
}

/// **The longest stem a point cache's name can hold, in bytes** — its `name`
/// property's own `length_max` read 64 at the desk, 2026-10-02, the ending
/// mark included (`Job files and outputs` D21b-i, sources entry 38d).
pub const THE_LONGEST_STEM_A_CACHE_NAME_HOLDS: usize = 63;

/// How much of a stem too long for a cache's name is kept before the mark.
const THE_PART_OF_A_LONG_STEM_KEPT: usize = 46;

/// **D21b-i, resolved 2026-10-02 on the build runner's ruling: a stem a
/// cache's name can hold.** Blender's own stem is its object's name in
/// hexadecimal, two characters a byte, so an object named longer than 31
/// bytes has a stem no cache name can hold, and a baked cache copied into
/// the job's folder under it is never read. **A stem that fits is returned
/// as it is.** One that does not becomes its first 46 bytes, an underscore,
/// and the 64-bit FNV-1a of the whole stem as 16 capital hexadecimal digits:
/// 63 bytes, the same for the same stem every time, and never a stem Blender
/// writes, since Blender's carry no underscore.
///
/// **The material computes the same in Python** (`Rendering — machine`'s
/// `smoozy_a_stem_a_name_can_hold`), and the copy renames each copied file by
/// it ([`the_name_a_copied_cache_file_takes`]), so the two agree by
/// construction and the desk proves it.
pub fn the_stem_a_cache_name_can_hold(stem: &[u8]) -> Vec<u8> {
    if stem.len() <= THE_LONGEST_STEM_A_CACHE_NAME_HOLDS {
        return stem.to_vec();
    }
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in stem {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let mut short = stem[..THE_PART_OF_A_LONG_STEM_KEPT].to_vec();
    short.push(b'_');
    short.extend_from_slice(format!("{hash:016X}").as_bytes());
    short
}

/// **D21b-i, resolved 2026-10-02: the name a file takes as a baked cache is
/// copied into the job's folder.** A point cache file is named
/// `<stem>_<six-digit frame>_<two-digit index>.bphys`; where its stem is longer
/// than a cache's name can hold, the copy takes
/// [`the_stem_a_cache_name_can_hold`]'s stem instead. **Every other name is
/// copied as it is**, so nothing else any copy places is touched.
pub fn the_name_a_copied_cache_file_takes(file_name: &[u8]) -> Vec<u8> {
    let Some(rest) = file_name.strip_suffix(b".bphys") else {
        return file_name.to_vec();
    };
    // `_` + 6 digits + `_` + 2 digits, from the right.
    const TAIL: usize = 1 + 6 + 1 + 2;
    if rest.len() <= TAIL {
        return file_name.to_vec();
    }
    let (stem, tail) = rest.split_at(rest.len() - TAIL);
    let shaped = tail[0] == b'_'
        && tail[1..7].iter().all(u8::is_ascii_digit)
        && tail[7] == b'_'
        && tail[8..].iter().all(u8::is_ascii_digit);
    if !shaped || stem.len() <= THE_LONGEST_STEM_A_CACHE_NAME_HOLDS {
        return file_name.to_vec();
    }
    let mut renamed = the_stem_a_cache_name_can_hold(stem);
    renamed.extend_from_slice(tail);
    renamed.extend_from_slice(b".bphys");
    renamed
}

/// **D21b-i, corrected 2026-10-02: the setting each cache's own file stem is
/// written to, for the place that has one** — the point cache's `name`, after
/// *external* and the folder.
///
/// **The stem is the cache's own**: its name where it has one, else the stem
/// Blender gives it, which is its object's name written as two capital
/// hexadecimal digits per byte of the name's UTF-8 — read off three caches on
/// three objects at the desk on 2026-10-02, a cloth, a cloth on an object with
/// a name that is not ASCII, and a particle system, in 4.5.12 and 5.2.2
/// (`Audits/Build/Step 10c/Simulation fix piece 3 — what landed.md`). The
/// material reads both off the cache itself, so nothing here carries a stem.
pub fn the_stem_setting_for(place: AnAddedWritePlace) -> Option<&'static str> {
    match place {
        AnAddedWritePlace::ThePointCache => Some("point_cache.name"),
        AnAddedWritePlace::TheBakeOutputPath
        | AnAddedWritePlace::TheGeometryNodesBakeFolder
        | AnAddedWritePlace::TheSequencerProxyFolder
        | AnAddedWritePlace::TheReplayFluidCacheFolder => None,
    }
}

/// **P7a's names: the folder each added write place takes under the job's
/// folder** — from 2026-10-04 whether or not the walk read a value back from
/// it (D20, P7a widened); before then only where it read none.
///
/// **D21b is why there has to be one at all**: each of the four *"enters the
/// destination set whether or not the walk read a value back from it, and the
/// redirect sets each one under the job's folder unconditionally"*. There is no
/// folder of the artist's own to keep the tail of, so the folder is named for
/// the setting — which is what P10 means by a pair *"keyed to the named
/// setting"*.
///
/// **The names are Build's own and the plan fixes none of them** — P7a says so
/// in as many words — which is N3's own standing for every name this document
/// synthesises. **What they must
/// be is distinct from each other**, because D21f reads *"whether that walk
/// returned any entry at all under that place's mapped folder"* — four places
/// sharing one folder would make that reading say the same thing about all four.
///
/// **`pointcache` is the one that is not a free choice.** D21b-i observed both
/// versions writing `pointcache/<name>_000001_00.bphys` under the job's folder
/// once the four values were set, so the point cache makes that folder itself
/// beneath whatever it is given; the name is kept the same so that a reader of
/// the job's folder does not meet `pointcache/pointcache/`.
pub fn the_folder_name_for(place: AnAddedWritePlace) -> &'static str {
    match place {
        AnAddedWritePlace::TheBakeOutputPath => "bake",
        AnAddedWritePlace::ThePointCache => "pointcache",
        AnAddedWritePlace::TheGeometryNodesBakeFolder => "geometry-nodes-bake",
        AnAddedWritePlace::TheSequencerProxyFolder => "sequencer-proxy",
        AnAddedWritePlace::TheReplayFluidCacheFolder => "fluid-cache",
    }
}

/// How one destination mapped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HowItMapped {
    UnderTheJobsFolder {
        /// The new folder's whole path on the machine.
        new_folder: Vec<u8>,
        /// P10's further values, applied with the folder.
        further_values: Vec<FurtherValue>,
    },
    /// **P12, with the reason.** D21 carries it in the recorded reading and it
    /// reaches `Queue & History` as a field of it, *"so nothing the reading
    /// found is hidden"*.
    NotMappable(WhyItIsNotMappable),
}

/// **P10: the map is a record of pairs — the destination the reading found, and
/// the new folder — and the render applies pairs. It is not an instruction to
/// recompute.**
///
/// **A pair for one of D20's four is keyed to the named setting** rather than to
/// a value the reading found, and it is in the map whether or not the walk read
/// a value back (D21b).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APair {
    pub destination: Destination,
    pub outcome: HowItMapped,
}

/// The map, computed once per job (R7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheMap {
    pub pairs: Vec<APair>,
}

impl TheMap {
    /// **P13: a job with any destination recorded as not mappable is a job
    /// whose paths Smoozy cannot bring under its folder**, and it is held under
    /// §9.1.
    ///
    /// **There is no partial redirect that proceeds quietly** over a
    /// destination the rule covers: the whole point of the promise is that
    /// `Download` is told it has one folder to watch.
    pub fn a_destination_is_not_mappable(&self) -> bool {
        self.pairs
            .iter()
            .any(|pair| matches!(pair.outcome, HowItMapped::NotMappable(_)))
    }

    /// **P11 as corrected 2026-10-04, out of the Creator's note 14 on the test
    /// copy: the folders shown are the new folders of the destinations the
    /// render asked for, relative to the job output folder** — the added write
    /// places are not among them. They are still in the map, still redirected
    /// and still come home; listing them put four or five cache folders on
    /// every job whether or not anything wrote there.
    ///
    /// **That is Row 122's second clause and it is the same object the render
    /// applies.**
    pub fn the_folders_the_job_will_write_into(
        &self,
        job_output_folder: &str,
    ) -> Vec<Vec<u8>> {
        self.the_folders_of(job_output_folder, |destination| {
            destination.is_one_the_render_asked_for()
        })
    }

    /// The new folders of the destinations `keep` answers yes for, relative to
    /// the job output folder, each once, in the map's order.
    pub fn the_folders_of(
        &self,
        job_output_folder: &str,
        keep: impl Fn(&Destination) -> bool,
    ) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        for pair in &self.pairs {
            if !keep(&pair.destination) {
                continue;
            }
            if let HowItMapped::UnderTheJobsFolder { new_folder, .. } = &pair.outcome {
                let beneath = new_folder
                    .strip_prefix(job_output_folder.as_bytes())
                    .unwrap_or(new_folder);
                let beneath = beneath.strip_prefix(&[SEPARATOR]).unwrap_or(beneath);
                let beneath = beneath.to_vec();
                if !out.contains(&beneath) {
                    out.push(beneath);
                }
            }
        }
        out
    }

    /// The pair for one destination, by the destination the reading found.
    pub fn the_pair_for(&self, destination: &Destination) -> Option<&APair> {
        self.pairs
            .iter()
            .find(|pair| pair.destination == *destination)
    }
}

/// One destination handed to the rule, with what §7's recipes say it will
/// write.
pub struct ADestinationToMap<'a> {
    pub destination: &'a Destination,
    /// **N8: the budget is measured against the predicted deepest file path and
    /// not against the folder.** *"Checking the folder alone would pass a
    /// destination whose files then fail at open."* So this is the longest name
    /// §7's recipes predict for that destination over the frames the job
    /// covers.
    ///
    /// **Absent for an added write place**, which writes files this stage has
    /// no recipe to name (I14) — the budget is then measured against the folder
    /// alone, which is all there is to measure.
    pub the_longest_name_it_will_write: Option<Vec<u8>>,
}

/// **P9: the set the map covers is the destination set of R4 item 2 entire**
/// — including every scene a sequencer strip reaches, and including the added
/// write places, every one of which has a pair.
///
/// **P6 and P9 as corrected 2026-10-04, out of the Creator's notes 14 and 18 on
/// the test copy: the shared run is worked out over the render's own output
/// destinations alone, and the added write places take no part in it, with or
/// without a value.** A bake output path left at `//`, the `.blend`'s own
/// folder — which a freshly read file records — had sat in the set on every
/// job, so the run came out one folder shorter and every folder the render
/// wrote gained a level. **D20 and P7a as widened the same day: every added
/// place takes its own named folder under the job's folder** ([`the_folder_name_for`]),
/// distinct per place and per recorded location.
///
/// **P8: the regular output path takes part in working out the shared run
/// alongside the nodes, and is not treated as the root of anything.**
///
/// **R9: this is the one place in this crate that runs the shared-run rule.**
/// A second evaluation of it is the fault Row 122 exists to prevent.
pub fn compose_the_map(
    destinations: &[ADestinationToMap<'_>],
    the_folder_the_blend_sits_in: &[u8],
    job_output_folder: &str,
    version: TheVersionInHand,
) -> TheMap {
    // **P6's set: the render's own output destinations, resolved.** An added
    // write place offers it nothing, value or none (D20, 2026-10-04), and nor
    // does a folder P7b takes out of the run.
    let folders: Vec<Option<ResolvedFolder>> = destinations
        .iter()
        .map(|to_map| {
            if to_map.destination.is_one_the_render_asked_for() {
                the_folder_of(to_map.destination, the_folder_the_blend_sits_in, version)
            } else {
                None
            }
        })
        .collect();
    let the_resolved_ones: Vec<ResolvedFolder> = folders.iter().flatten().cloned().collect();
    let shared_run = the_shared_run(&the_resolved_ones);

    let mut new_folders: Vec<Vec<u8>> = Vec::with_capacity(destinations.len());
    let mut places_named_so_far: Vec<AnAddedWritePlace> = Vec::new();
    for (to_map, folder) in destinations.iter().zip(folders.iter()) {
        new_folders.push(match (to_map.destination.as_an_added_write_place(), folder) {
            // **P7a, widened 2026-10-04: an added place's folder is named for
            // its setting, whether or not a location was recorded for it.** A
            // place with more than one destination — one per distinct recorded
            // location (R4 item 2) — numbers the second and later from 2, in
            // the reading's order, so each stays its own folder for D21f.
            (Some(place), _) => {
                let before = places_named_so_far.iter().filter(|seen| **seen == place).count();
                places_named_so_far.push(place);
                a_named_folder_under(job_output_folder, place, before + 1)
            }
            (None, Some(folder)) => the_new_folder(job_output_folder, folder, &shared_run),
            // **P7b: a folder that opens with no place keeps the whole of what
            // it says under the job's folder.**
            (None, None) => match the_folder_with_no_place_in_the_run(to_map.destination, version) {
                Some(written) => {
                    the_new_folder(job_output_folder, &ResolvedFolder { anchor: Anchor::ItsOwnTree, segments: without_the_steps(whole_names(written)) }, &[])
                }
                // A destination the reading found and did not say where, which
                // R4 item 2 does not produce: it lands at the job output folder
                // itself rather than being invented a name.
                None => job_output_folder.as_bytes().to_vec(),
            },
        });
    }
    keep_the_folders_that_held_an_expression_apart(destinations, &mut new_folders, version);

    let mut pairs = Vec::with_capacity(destinations.len());
    for (to_map, new_folder) in destinations.iter().zip(new_folders) {
        let outcome = match why_it_is_not_mappable(to_map, &new_folder, version) {
            Some(why) => HowItMapped::NotMappable(why),
            None => HowItMapped::UnderTheJobsFolder {
                new_folder,
                further_values: to_map
                    .destination
                    .as_an_added_write_place()
                    .map(the_further_values_for)
                    .unwrap_or_default(),
            },
        };
        pairs.push(APair {
            destination: to_map.destination.clone(),
            outcome,
        });
    }
    TheMap { pairs }
}

/// **§6.6 P18's set: the folders of the render's own output destinations,
/// resolved on the machine** — P6's set exactly as [`compose_the_map`] works
/// out its shared run over it, by the same per-destination resolution, so the
/// run the job's folder is placed in and the run the map removes are one
/// rule. D20's added write places take no part (P9). A destination with no
/// recorded folder offers nothing. Added 2026-10-04, the Creator's second
/// notes, piece 4.
pub fn the_renders_own_folders_resolved(
    destinations: &[Destination],
    the_folder_the_blend_sits_in: &[u8],
    version: TheVersionInHand,
) -> Vec<ResolvedFolder> {
    destinations
        .iter()
        .filter(|destination| destination.is_one_the_render_asked_for())
        .filter_map(|destination| the_folder_of(destination, the_folder_the_blend_sits_in, version))
        .collect()
}

/// **The folder one of the render's own destinations is written as**, split from
/// the front of the name where its location holds one.
fn the_folder_written(destination: &Destination, version: TheVersionInHand) -> Option<&[u8]> {
    let location = destination.the_location_blender_writes_into()?;
    Some(if it_holds_a_name_front(&destination.kind, version) {
        split_the_regular_output_path(location).0
    } else {
        location
    })
}

/// **The folder as the file records it**, split the same way.
fn the_folder_recorded(destination: &Destination, version: TheVersionInHand) -> Option<&[u8]> {
    let location = destination.recorded_location.as_ref()?.as_bytes();
    Some(if it_holds_a_name_front(&destination.kind, version) {
        split_the_regular_output_path(location).0
    } else {
        location
    })
}

fn it_holds_a_brace(text: &[u8]) -> bool {
    text.iter().any(|byte| *byte == b'{' || *byte == b'}')
}

/// **P7b: the folder of a destination that takes no part in the shared run**: it
/// held an expression and, read, opens with no place. So it is no folder of
/// another computer (P4), which is what it read as until 2026-11-02: that took
/// away the shared run, pushed every other folder down the whole of its path,
/// and sent the job to the fallback as *another computer's*.
fn the_folder_with_no_place_in_the_run(destination: &Destination, version: TheVersionInHand) -> Option<&[u8]> {
    if !destination.is_one_the_render_asked_for() {
        return None;
    }
    let recorded = the_folder_recorded(destination, version)?;
    let written = the_folder_written(destination, version)?;
    (it_holds_a_brace(recorded) && !it_opens_with_a_place(written)).then_some(written)
}

/// **P7b: a folder that held an expression never lands on another destination's
/// folder**, since naming it may have made it the same text. Each takes the
/// number that keeps it apart, as P7a's second place does. **Two with the same
/// recorded text are one folder, as P17 keeps them**, unless that text names the
/// node or the scene, which differs between them where Blender works it out.
fn keep_the_folders_that_held_an_expression_apart(
    destinations: &[ADestinationToMap<'_>],
    new_folders: &mut [Vec<u8>],
    version: TheVersionInHand,
) {
    let it_is_kept_apart = |destination: &Destination| {
        let (Some(recorded), Some(written)) =
            (the_folder_recorded(destination, version), the_folder_written(destination, version))
        else {
            return false;
        };
        destination.is_one_the_render_asked_for()
            && it_holds_a_brace(recorded)
            && (!it_opens_with_a_place(written)
                || crate::reading::a_folder_with_its_expressions_named(recorded) == written)
    };
    let mut taken: Vec<Vec<u8>> = destinations
        .iter()
        .zip(new_folders.iter())
        .filter(|(to_map, _)| !it_is_kept_apart(to_map.destination))
        .map(|(_, folder)| folder.clone())
        .collect();
    let mut seen: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for (to_map, folder) in destinations.iter().zip(new_folders.iter_mut()) {
        if !it_is_kept_apart(to_map.destination) {
            continue;
        }
        let key = the_key_of_a_folder_kept_apart(to_map.destination, version);
        if let Some((_, given)) = seen.iter().find(|(one, _)| *one == key) {
            *folder = given.clone();
            continue;
        }
        let mut candidate = folder.clone();
        let mut number = 2;
        while taken.contains(&candidate) {
            candidate = [folder.as_slice(), format!("-{number}").as_bytes()].concat();
            number += 1;
        }
        *folder = candidate.clone();
        taken.push(candidate.clone());
        seen.push((key, candidate));
    }
}

/// What makes two such folders one place.
fn the_key_of_a_folder_kept_apart(destination: &Destination, version: TheVersionInHand) -> Vec<u8> {
    let recorded = the_folder_recorded(destination, version).unwrap_or_default();
    let holds = |what: &[u8]| recorded.windows(what.len()).any(|window| window == what);
    let mut key = recorded.to_vec();
    if holds(b"{node_name}") {
        key.push(0);
        if let DestinationKind::AnOutputOfAFileOutputNode { node, .. }
        | DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { node, .. } = &destination.kind
        {
            key.extend_from_slice(node.as_bytes());
        }
    }
    if holds(b"{scene_name}") {
        key.push(0);
        if let WhichScene::ReachedThroughASequencerStrip { scene, .. } = &destination.scene {
            key.extend_from_slice(scene.as_bytes());
        }
    }
    key
}

/// The folder one of the render's own destinations writes into, resolved — or
/// nothing, where there is no recorded folder to resolve.
///
/// **A location holding a name front is split first** (P4's third bullet:
/// the regular output path, and since 2026-10-04 4.5's gathering node —
/// [`it_holds_a_name_front`]); every other destination's recorded location is
/// a folder already. **An added write place is never asked**: it takes
/// [`a_named_folder_under`], value or none (P7a). **Nor is a folder of P7b's**
/// ([`the_folder_with_no_place_in_the_run`]).
///
/// **A node's folder is read with its curly-brace expressions put in** where
/// the reading replaced them all, so the folder the redirect maps and hands
/// Blender holds no expression (`Job files and outputs` I2's *A node's folder
/// is read the same way*, added 2026-10-18); one left unresolved, which from
/// 2026-11-02 is one Blender works out in a place the rule can measure (P7b),
/// is mapped as recorded.
fn the_folder_of(
    destination: &Destination,
    the_folder_the_blend_sits_in: &[u8],
    version: TheVersionInHand,
) -> Option<ResolvedFolder> {
    if the_folder_with_no_place_in_the_run(destination, version).is_some() {
        return None;
    }
    Some(resolve_a_folder(the_folder_written(destination, version)?, the_folder_the_blend_sits_in))
}

/// **P7a's named folder**: the job output folder joined to the name for the
/// place's setting — and, for the place's second destination and later, the
/// number that keeps it distinct (`pointcache-2`).
fn a_named_folder_under(job_output_folder: &str, place: AnAddedWritePlace, nth: usize) -> Vec<u8> {
    let mut out = job_output_folder.as_bytes().to_vec();
    out.push(SEPARATOR);
    out.extend_from_slice(the_folder_name_for(place).as_bytes());
    if nth > 1 {
        out.extend_from_slice(format!("-{nth}").as_bytes());
    }
    out
}

/// **P12, performed.** Case 1 first, then 4.5's own case 2.
fn why_it_is_not_mappable(
    to_map: &ADestinationToMap<'_>,
    new_folder: &[u8],
    version: TheVersionInHand,
) -> Option<WhyItIsNotMappable> {
    // **P12 case 2, and P14 is why case 1 is checked at all for an added write
    // place**: *"where one of them is not mappable it is P12's case 1 — a
    // length that will not fit."*
    if version.branch() == WhichBranch::FourFive {
        if let DestinationKind::AnOutputOfAFileOutputNode { output, .. } =
            &to_map.destination.kind
        {
            if the_name_climbs(output.as_bytes()) {
                return Some(WhyItIsNotMappable::AnOutputNameClimbsAboveItsNodesFolderInFourFive);
            }
        }
    }

    // **N8: against the predicted deepest file path.** Every segment of the
    // folder is checked as well as the whole path, which is what
    // [`a_path_fits`] does.
    let mut deepest = new_folder.to_vec();
    if let Some(name) = to_map.the_longest_name_it_will_write.as_ref() {
        deepest.push(SEPARATOR);
        deepest.extend_from_slice(name);
    }
    // The budget is a count of bytes, so a location that is not text is
    // measured exactly as the machine will measure it.
    if let Err(why) = fits(&deepest) {
        return Some(WhyItIsNotMappable::ItsPredictedDeepestFilePathWillNotFit(why));
    }
    None
}

/// N5's budget over bytes that need not be text.
///
/// [`a_path_fits`] takes a `&str` because every path this crate composes for
/// itself is one; a destination's own bytes need not be, and R4a forbids
/// decoding them to find out. **The arithmetic is the same arithmetic** — the
/// budget counts bytes — so this measures the bytes directly and defers to that
/// function wherever the path is text, so there is one rule rather than two.
fn fits(path: &[u8]) -> Result<(), WhyThePathIsRefused> {
    match std::str::from_utf8(path) {
        Ok(text) => a_path_fits(text),
        Err(_) => {
            if path.is_empty() {
                return Err(WhyThePathIsRefused::ItIsEmpty);
            }
            if path.len() > crate::names::PATH_BUDGET_BYTES {
                return Err(WhyThePathIsRefused::TheWholePathIsOverTheBudget {
                    it_is_this_many_bytes: path.len(),
                });
            }
            for segment in path.split(|byte| *byte == SEPARATOR) {
                if segment.is_empty() {
                    continue;
                }
                if segment.len() > crate::names::NAME_BUDGET_BYTES {
                    return Err(WhyThePathIsRefused::ASingleNameIsOverTheBudget {
                        it_is_this_many_bytes: segment.len(),
                    });
                }
            }
            // A name a checker cannot read is still a name of the right length,
            // and [`a_name_fits`] is the rule it is measured by wherever it is
            // text.
            let _ = a_name_fits;
            Ok(())
        }
    }
}

/// Whether an output's own piece of path climbs above the node's folder.
///
/// **4.5's own case**, P12 case 2: the recipe joins the node's folder, the
/// output's own piece of path, the frame number and the ending, so a piece of
/// path whose steps take it above the folder writes outside it.
fn the_name_climbs(name: &[u8]) -> bool {
    let mut depth: i64 = 0;
    for segment in name.split(|byte| *byte == SEPARATOR) {
        if segment == b"." || segment.is_empty() {
            continue;
        }
        if segment == b".." {
            depth -= 1;
            if depth < 0 {
                return true;
            }
            continue;
        }
        depth += 1;
    }
    false
}

/// **P16: two destinations that resolved to different folders always land in
/// different folders under the job's folder**, because only a prefix identical
/// to every destination is removed and the remainders therefore still differ.
///
/// **So the collision set after redirecting is exactly the collision set before
/// it**, which is what makes the collision record computable before the render.
///
/// **P17: and two destinations that were the same place stay the same place**,
/// which P7's *nothing left* case covers without inventing anything to tell them
/// apart with.
pub const THE_RULE_NEVER_MERGES_TWO_DESTINATIONS_THAT_WERE_DISTINCT: bool = true;

/// **P15: every other form of path is mappable, which is what lets the folders
/// be stated firmly.**
///
/// §6.2 covers the relative and the foreign forms, and P7 covers the case where
/// nothing is left. **P14: there is no class of write place the rule covers but
/// does not map** — the one write place that is not here at all is D21's
/// custom-property residue, which the reading never finds, *"so there is nothing
/// to record and nothing to hold on"*.
pub const EVERY_OTHER_FORM_OF_PATH_IS_MAPPABLE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reading::tests::{a_destination, a_location};
    use crate::reading::{ArtistsText, WhereItComesFrom, WhichScene};

    const BLEND_SITS_IN: &[u8] = b"/ephemeral/smoozy/assets/set-1";
    const JOB_FOLDER: &str = "/ephemeral/smoozy/jobs/job-7";

    fn a_node_output(node: &str, output: &str, at: &str) -> Destination {
        a_destination(
            DestinationKind::AnOutputOfAFileOutputNode {
                the_effect: None,
                node: ArtistsText::of(node.as_bytes().to_vec()),
                output: ArtistsText::of(output.as_bytes().to_vec()),
                the_nodes_base_file_name: None,
                replaced: None,
            },
            at,
            WhereItComesFrom::AFileOutputNodeNamed(ArtistsText::of(node.as_bytes().to_vec())),
        )
    }

    fn to_map<'a>(destinations: &'a [Destination]) -> Vec<ADestinationToMap<'a>> {
        destinations
            .iter()
            .map(|destination| ADestinationToMap {
                destination,
                the_longest_name_it_will_write: Some(b"beauty0010.png".to_vec()),
            })
            .collect()
    }

    #[test]
    fn p6_measures_the_shared_run_in_whole_names_and_never_in_letters() {
        // `/home/a/out` and `/home/a/output2` share `/home/a` — a rule
        // measured in letters would take `/home/a/out` and put the second
        // destination at `put2`.
        let one = resolve_a_folder(b"/home/a/out", BLEND_SITS_IN);
        let other = resolve_a_folder(b"/home/a/output2", BLEND_SITS_IN);
        let run = the_shared_run(&[one.clone(), other.clone()]);
        assert_eq!(run, vec![b"home".to_vec(), b"a".to_vec()]);
        assert_eq!(
            the_new_folder(JOB_FOLDER, &one, &run),
            format!("{JOB_FOLDER}/out").into_bytes()
        );
        assert_eq!(
            the_new_folder(JOB_FOLDER, &other, &run),
            format!("{JOB_FOLDER}/output2").into_bytes()
        );
    }

    #[test]
    fn a_foreign_path_keeps_the_whole_of_itself_beneath() {
        // P4. A path written on another kind of computer resolves to nothing on
        // Linux and is its own tree, so it shares no run with a machine path.
        let linux = resolve_a_folder(b"/home/a/out", BLEND_SITS_IN);
        let windows = resolve_a_folder(b"C:/Users/artist/out", BLEND_SITS_IN);
        assert_eq!(windows.anchor, Anchor::ItsOwnTree);
        let run = the_shared_run(&[linux, windows.clone()]);
        assert!(run.is_empty(), "two anchors cannot share a run");
        assert_eq!(
            the_new_folder(JOB_FOLDER, &windows, &run),
            format!("{JOB_FOLDER}/C:/Users/artist/out").into_bytes()
        );
    }

    #[test]
    fn a_blend_file_shorthand_path_resolves_against_the_blends_folder_and_may_climb() {
        // P4's first bullet, and P5: after resolution nothing climbs.
        let beneath = resolve_a_folder(b"//renders/beauty", BLEND_SITS_IN);
        assert_eq!(beneath.anchor, Anchor::TheMachinesOwnRoot);
        assert_eq!(
            beneath.as_bytes(),
            b"/ephemeral/smoozy/assets/set-1/renders/beauty".to_vec()
        );
        let climbing = resolve_a_folder(b"//../out", BLEND_SITS_IN);
        assert_eq!(
            climbing.as_bytes(),
            b"/ephemeral/smoozy/assets/out".to_vec()
        );
        assert!(!climbing.segments.iter().any(|segment| segment == b".."));
    }

    #[test]
    fn the_regular_output_path_is_split_at_the_last_separator() {
        // P4's third bullet. The rule works on the part before it, and a path
        // ending in a separator has no name front at all.
        assert_eq!(
            split_the_regular_output_path(b"/home/a/out/beauty"),
            (&b"/home/a/out"[..], &b"beauty"[..])
        );
        assert_eq!(
            split_the_regular_output_path(b"/home/a/out/"),
            (&b"/home/a/out"[..], &b""[..])
        );
        // **The blend-file shorthand's own separators are not the last
        // separator** (piece 1b, 2026-10-04): `//f_` is `f_` in the `.blend`'s
        // own folder, never at the machine's root.
        assert_eq!(split_the_regular_output_path(b"//f_"), (&b"//"[..], &b"f_"[..]));
        assert_eq!(split_the_regular_output_path(b"//"), (&b"//"[..], &b""[..]));
        assert_eq!(
            resolve_a_folder(split_the_regular_output_path(b"//f_").0, BLEND_SITS_IN),
            resolve_a_folder(BLEND_SITS_IN, b"/")
        );
    }

    #[test]
    fn p8_puts_the_regular_output_path_into_the_shared_run_and_not_at_the_root() {
        // A rule that treated the regular output path as the root would put
        // every node's folder under it. Here both are ordinary members.
        let destinations = vec![
            a_destination(
                DestinationKind::TheRegularOutputPath,
                "/home/a/renders/frames/f_",
                WhereItComesFrom::TheScenesOutputPath,
            ),
            a_node_output("Beauty", "Image", "/home/a/renders/passes"),
        ];
        let map = compose_the_map(
            &to_map(&destinations),
            BLEND_SITS_IN,
            JOB_FOLDER,
            TheVersionInHand::of(5, 2),);
        let folders = map.the_folders_the_job_will_write_into(JOB_FOLDER);
        assert_eq!(folders, vec![b"frames".to_vec(), b"passes".to_vec()]);
    }

    #[test]
    fn an_added_write_place_takes_no_part_in_the_shared_run_and_is_not_listed() {
        // **P6, P9 and D20 as corrected 2026-10-04**, out of the Creator's notes
        // 14 and 18 on the test copy. Until then a cache with a value sat in
        // the shared run, so the run came out shorter and every folder the
        // render wrote gained a level (`renders/frames`, `renders/passes`).
        // Now the render's own destinations decide the run alone, the cache
        // takes its own named folder (P7a), and it is not among the folders
        // shown (P11).
        let regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/renders/frames/f_",
            WhereItComesFrom::TheScenesOutputPath,
        );
        let node = a_node_output("Beauty", "Image", "/home/a/renders/passes");
        let cache = Destination {
            kind: DestinationKind::AnAddedWritePlace(AnAddedWritePlace::ThePointCache),
            recorded_location: Some(a_location("/home/a/sim/cache")),
            format: None,
            origin: WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::ThePointCache),
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };

        let without = vec![regular.clone(), node.clone()];
        let with = vec![regular.clone(), node.clone(), cache.clone()];
        let alone = compose_the_map(
            &to_map(&without),
            BLEND_SITS_IN,
            JOB_FOLDER,
            TheVersionInHand::of(5, 2),);
        let beside = compose_the_map(
            &to_map(&with),
            BLEND_SITS_IN,
            JOB_FOLDER,
            TheVersionInHand::of(5, 2),);
        assert_eq!(
            alone.the_folders_the_job_will_write_into(JOB_FOLDER),
            vec![b"frames".to_vec(), b"passes".to_vec()]
        );
        assert_eq!(
            beside.the_folders_the_job_will_write_into(JOB_FOLDER),
            vec![b"frames".to_vec(), b"passes".to_vec()],
            "the cache moves no folder and is not shown"
        );
        // **It is still in the map, under its own name**, value and all.
        assert_eq!(
            beside.the_folders_of(JOB_FOLDER, |_| true),
            vec![b"frames".to_vec(), b"passes".to_vec(), b"pointcache".to_vec()]
        );
    }

    #[test]
    fn a_bake_output_path_left_at_the_blends_own_folder_moves_nothing() {
        // **The Creator's own case** (note 14): a fresh file records the bake
        // output path as `//`, the `.blend`'s own folder. With the file one
        // folder below its outputs, that `//` had made the run `/home/ubuntu`
        // instead of `/home/ubuntu/Render`, so every folder gained `Render/`
        // and `rendering` was listed. Now the ordinary file's frames land at
        // the top of the job's folder, as the Specs' worked example says, and
        // the bake takes `bake`.
        let regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "//../Render/Animation",
            WhereItComesFrom::TheScenesOutputPath,
        );
        let png = a_node_output("PNG", "### - Box", "//../Render/PNG");
        let exr = a_node_output("EXR", "Image", "//../Render/EXR");
        let bake = Destination {
            kind: DestinationKind::AnAddedWritePlace(AnAddedWritePlace::TheBakeOutputPath),
            recorded_location: Some(a_location("//")),
            format: None,
            origin: WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::TheBakeOutputPath),
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };
        let everything = vec![regular, png, exr, bake];
        let map = compose_the_map(
            &to_map(&everything),
            b"/home/ubuntu/rendering",
            JOB_FOLDER,
            TheVersionInHand::of(5, 2),);
        assert_eq!(
            map.the_folders_the_job_will_write_into(JOB_FOLDER),
            vec![Vec::new(), b"PNG".to_vec(), b"EXR".to_vec()]
        );
        assert_eq!(
            map.the_folders_of(JOB_FOLDER, |destination| destination
                .as_an_added_write_place()
                .is_some()),
            vec![b"bake".to_vec()]
        );
    }

    #[test]
    fn two_destinations_of_one_place_take_two_folders() {
        // **P7a, widened 2026-10-04: distinct per place and per recorded
        // location.** R4 item 2 gives a place one destination per distinct
        // location; the second takes the name with a number, so D21f's walk
        // still reads each on its own.
        let a_cache = |at: &str| Destination {
            kind: DestinationKind::AnAddedWritePlace(AnAddedWritePlace::ThePointCache),
            recorded_location: Some(a_location(at)),
            format: None,
            origin: WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::ThePointCache),
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };
        let both = vec![a_cache("/home/a/sim/one"), a_cache("/home/a/sim/two")];
        let map = compose_the_map(&to_map(&both), BLEND_SITS_IN, JOB_FOLDER, TheVersionInHand::of(5, 2));
        assert_eq!(
            map.the_folders_of(JOB_FOLDER, |_| true),
            vec![b"pointcache".to_vec(), b"pointcache-2".to_vec()]
        );
    }

    /// **A node destination whose folder the reading has read**, from 5.0.
    fn a_node_read(node: &str, at: &str) -> Destination {
        let mut one = a_node_output(node, "Image", at);
        one.put_in_the_replaced_text(Some(b"job"), TheVersionInHand::of(5, 2));
        one
    }

    fn a_regular_output_read(at: &str) -> Destination {
        let mut one = Destination {
            kind: DestinationKind::TheRegularOutputPath,
            recorded_location: Some(crate::reading::RecordedLocation::of(at.as_bytes().to_vec())),
            format: None,
            origin: WhereItComesFrom::TheScenesOutputPath,
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };
        one.put_in_the_replaced_text(Some(b"job"), TheVersionInHand::of(5, 2));
        one
    }

    fn the_folders(map: &TheMap) -> Vec<String> {
        map.pairs
            .iter()
            .map(|pair| match &pair.outcome {
                HowItMapped::UnderTheJobsFolder { new_folder, .. } => String::from_utf8_lossy(new_folder).into_owned(),
                HowItMapped::NotMappable(_) => "not mappable".to_string(),
            })
            .collect()
    }

    /// **P7b: why the regular output went deep.** A folder opening with an
    /// expression read as another computer's own tree, so no run was shared and
    /// the regular output kept the whole of its path under the job's folder. It
    /// now takes no part in the run, and the regular output is where it would be.
    #[test]
    fn p7b_a_folder_opening_with_an_expression_takes_no_part_in_the_run() {
        let version = TheVersionInHand::of(5, 2);
        let destinations = [a_regular_output_read("//render/f_"), a_node_read("A", "{project_root}/out")];
        let map = compose_the_map(&to_map(&destinations), b"/home/ubuntu/rendering", JOB_FOLDER, version);
        assert_eq!(the_folders(&map), [JOB_FOLDER.to_string(), format!("{JOB_FOLDER}/project_root/out")]);
        // And a folder written from the file's own place stays in the run.
        let destinations = [a_regular_output_read("//render/f_"), a_node_read("A", "//renders/{project_root}/out")];
        let map = compose_the_map(&to_map(&destinations), b"/home/ubuntu/rendering", JOB_FOLDER, version);
        assert_eq!(
            the_folders(&map),
            [format!("{JOB_FOLDER}/render"), format!("{JOB_FOLDER}/renders/project_root/out")]
        );
        // `{blend_name}` is replaced by its value, and opens with no place either.
        let destinations = [a_regular_output_read("//render/f_"), a_node_read("A", "{blend_name}/out")];
        let map = compose_the_map(&to_map(&destinations), b"/home/ubuntu/rendering", JOB_FOLDER, version);
        assert_eq!(the_folders(&map), [JOB_FOLDER.to_string(), format!("{JOB_FOLDER}/job/out")]);
        // A step in a named folder climbs nowhere (P5).
        let destinations = [a_regular_output_read("//render/f_"), a_node_read("A", "{project_root}/../../out")];
        let map = compose_the_map(&to_map(&destinations), b"/home/ubuntu/rendering", JOB_FOLDER, version);
        assert_eq!(the_folders(&map)[1], format!("{JOB_FOLDER}/out"));
    }

    /// **P7b and P16: a folder that held an expression does not land on another
    /// destination's**, and P17 keeps one recorded text one folder, unless it
    /// names the node.
    #[test]
    fn p7b_folders_that_held_expressions_keep_apart_and_one_text_stays_one_folder() {
        let version = TheVersionInHand::of(5, 2);
        let destinations = [
            a_regular_output_read("//render/f_"),
            a_node_read("A", "{project_root}/out"),
            a_node_read("B", "{project_root}/out"),
            a_node_read("C", "{unknown}/out"),
            a_node_read("D", "//render/project_root/out"),
            a_node_read("E", "{node_name}/out"),
            a_node_read("F", "{node_name}/out"),
        ];
        let map = compose_the_map(&to_map(&destinations), b"/home/ubuntu/rendering", JOB_FOLDER, version);
        assert_eq!(
            the_folders(&map),
            [
                JOB_FOLDER.to_string(),
                format!("{JOB_FOLDER}/project_root/out-2"),
                format!("{JOB_FOLDER}/project_root/out-2"),
                format!("{JOB_FOLDER}/unknown/out"),
                format!("{JOB_FOLDER}/project_root/out"),
                format!("{JOB_FOLDER}/node_name/out"),
                format!("{JOB_FOLDER}/node_name/out-2"),
            ]
        );
    }

    /// **P7b leaves alone what works.** A node's folder with no brace, and one
    /// Blender works out from the file's own place, map exactly as they did.
    #[test]
    fn p7b_changes_nothing_for_a_folder_blender_works_out_or_one_with_no_expression() {
        let version = TheVersionInHand::of(5, 2);
        let destinations = [
            a_regular_output_read("//render/f_"),
            a_node_read("A", "//{scene_name}/out"),
            a_node_read("B", "//plain"),
        ];
        let map = compose_the_map(&to_map(&destinations), b"/home/ubuntu/rendering", JOB_FOLDER, version);
        assert_eq!(
            the_folders(&map),
            [
                format!("{JOB_FOLDER}/render"),
                format!("{JOB_FOLDER}/{{scene_name}}/out"),
                format!("{JOB_FOLDER}/plain"),
            ]
        );
    }

    #[test]
    fn p7s_nothing_left_case_lands_at_the_job_output_folder_itself() {
        let only = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/f_",
            WhereItComesFrom::TheScenesOutputPath,
        );
        let destinations = vec![only];
        let map = compose_the_map(
            &to_map(&destinations),
            BLEND_SITS_IN,
            JOB_FOLDER,
            TheVersionInHand::of(5, 2),);
        assert_eq!(
            map.the_folders_the_job_will_write_into(JOB_FOLDER),
            vec![Vec::<u8>::new()]
        );
    }

    #[test]
    fn p17_keeps_two_destinations_that_were_the_same_place_in_the_same_place() {
        let one = a_node_output("Beauty", "Image", "/home/a/out");
        let other = a_node_output("Other", "Image", "/home/a/out");
        let destinations = vec![one, other];
        let map = compose_the_map(
            &to_map(&destinations),
            BLEND_SITS_IN,
            JOB_FOLDER,
            TheVersionInHand::of(5, 2),);
        assert_eq!(map.the_folders_the_job_will_write_into(JOB_FOLDER).len(), 1);
    }

    #[test]
    fn p12_case_one_fires_on_the_predicted_file_path_and_not_on_the_folder() {
        // N8: checking the folder alone would pass a destination whose files
        // then fail at open. The folder here fits and the file does not.
        let long = format!("/home/a/{}", "n".repeat(250));
        let destination = a_destination(
            DestinationKind::TheRegularOutputPath,
            &format!("{long}/f_"),
            WhereItComesFrom::TheScenesOutputPath,
        );
        let destinations = [destination];
        let held = [ADestinationToMap {
            destination: &destinations[0],
            the_longest_name_it_will_write: Some(b"x".repeat(300)),
        }];
        let map = compose_the_map(
            &held,
            BLEND_SITS_IN,
            JOB_FOLDER,
            TheVersionInHand::of(5, 2),);
        assert!(map.a_destination_is_not_mappable());
        assert!(matches!(
            map.pairs[0].outcome,
            HowItMapped::NotMappable(WhyItIsNotMappable::ItsPredictedDeepestFilePathWillNotFit(_))
        ));

        // And with a name that fits, the same folder maps.
        let fitting = vec![ADestinationToMap {
            destination: &destinations[0],
            the_longest_name_it_will_write: Some(b"f_0001.png".to_vec()),
        }];
        assert!(!compose_the_map(
            &fitting,
            BLEND_SITS_IN,
            JOB_FOLDER,
            TheVersionInHand::of(5, 2),)
        .a_destination_is_not_mappable());
    }

    #[test]
    fn p12_case_two_is_four_fives_alone() {
        let destinations = vec![a_node_output("Beauty", "../outside", "/home/a/out")];
        for (version, expected) in [
            (TheVersionInHand::of(4, 5), true),
            (TheVersionInHand::of(5, 0), false),
            (TheVersionInHand::of(5, 2), false),
        ] {
            let map = compose_the_map(
                &to_map(&destinations),
                BLEND_SITS_IN,
                JOB_FOLDER,
                version,);
            assert_eq!(
                map.a_destination_is_not_mappable(),
                expected,
                "{version:?} decided the climbing name wrongly"
            );
        }
    }

    #[test]
    fn a_name_that_steps_down_and_back_up_does_not_climb() {
        assert!(!the_name_climbs(b"passes/../beauty"));
        assert!(the_name_climbs(b"../beauty"));
        assert!(the_name_climbs(b"passes/../../beauty"));
    }

    #[test]
    fn the_point_cache_pair_carries_its_further_values_and_the_other_three_carry_none() {
        // D21b-i: `use_external` is the value that decides it, and without it
        // the assignment is a silent no-op with the render reporting success.
        let cache = the_further_values_for(AnAddedWritePlace::ThePointCache);
        assert!(cache.iter().any(|value| value.setting.ends_with("use_external")));
        for place in [
            AnAddedWritePlace::TheBakeOutputPath,
            AnAddedWritePlace::TheGeometryNodesBakeFolder,
            AnAddedWritePlace::TheSequencerProxyFolder,
        ] {
            assert!(the_further_values_for(place).is_empty());
        }
    }

    /// **D21b-i, corrected 2026-10-02: the disk-cache switch is never
    /// written.** Writing it on a cache where it is already on deleted every
    /// file the cache held — 48 baked files beside the `.blend` became none, in
    /// 4.5.12 and 5.2.2 at the desk. So no place's further values may name it,
    /// and the point cache's are *external* and nothing else — the name being
    /// each cache's own since the same day ([`the_stem_setting_for`]).
    #[test]
    fn no_place_writes_the_disk_cache_switch() {
        use crate::reading::EVERY_ADDED_WRITE_PLACE;
        for place in EVERY_ADDED_WRITE_PLACE {
            for value in the_further_values_for(place) {
                assert!(
                    !value.setting.contains("use_disk_cache"),
                    "{place:?} writes the disk-cache switch, which deletes the cache's files"
                );
            }
        }
        let settings: Vec<String> = the_further_values_for(AnAddedWritePlace::ThePointCache)
            .into_iter()
            .map(|value| value.setting)
            .collect();
        assert_eq!(settings, ["point_cache.use_external"]);
    }

    /// **D21b-i, corrected 2026-10-02: no cache is given one stem for every
    /// cache.** Under the stem `smoozy` a baked cloth copied into the job's
    /// folder was not read — frame 24 wrong on 35% of pixels, *"No valid data
    /// to read!"* — so no further value carries a name, and the point cache's
    /// own stem is written to its `name` by the material, read off the cache.
    #[test]
    fn no_cache_is_given_one_stem_for_every_cache() {
        use crate::reading::EVERY_ADDED_WRITE_PLACE;
        for place in EVERY_ADDED_WRITE_PLACE {
            assert!(the_further_values_for(place)
                .iter()
                .all(|value| !matches!(value.value, FurtherValueIs::NonEmptyText(_))));
        }
        assert_eq!(
            the_stem_setting_for(AnAddedWritePlace::ThePointCache),
            Some("point_cache.name")
        );
        assert_eq!(the_stem_setting_for(AnAddedWritePlace::TheGeometryNodesBakeFolder), None);
    }

    /// **D21b-i, resolved 2026-10-02: a stem too long for a cache's name is
    /// made one it can hold, and a copied cache file is renamed by it.** At the
    /// desk a cloth on an object named `Cloth_Simulation_Curtain_Left_Window_001`
    /// (an 80-character stem), and one named `Занавеска_левая_окна` (also 80),
    /// were copied and never read: frame 24 wrong on 35% of pixels.
    #[test]
    fn a_stem_too_long_for_a_caches_name_is_made_one_it_can_hold() {
        let long: Vec<u8> = "Cloth_Simulation_Curtain_Left_Window_001"
            .bytes()
            .flat_map(|byte| format!("{byte:02X}").into_bytes())
            .collect();
        assert_eq!(long.len(), 80);
        let short = the_stem_a_cache_name_can_hold(&long);
        assert_eq!(short.len(), THE_LONGEST_STEM_A_CACHE_NAME_HOLDS);
        assert_eq!(&short[..46], &long[..46]);
        assert_eq!(short[46], b'_');
        // **The same stem gives the same short stem**, and the material's
        // Python must give exactly this one: FNV-1a, 64 bits, capital hex.
        assert_eq!(short, the_stem_a_cache_name_can_hold(&long));
        assert_eq!(&short[47..], b"F3D2745F4CE4F1E2");
        // **A stem that fits is left alone**, a user's name among them.
        assert_eq!(the_stem_a_cache_name_can_hold(b"436C6F746841"), b"436C6F746841");
        // **The copy's names.**
        let mut file = long.clone();
        file.extend_from_slice(b"_000024_00.bphys");
        let mut wanted = short.clone();
        wanted.extend_from_slice(b"_000024_00.bphys");
        assert_eq!(the_name_a_copied_cache_file_takes(&file), wanted);
        for untouched in [
            &b"436C6F746841_000024_00.bphys"[..],
            b"00001_00000.blob",
            b"meta/00001_00000.json",
            b"lightmap.png",
        ] {
            assert_eq!(the_name_a_copied_cache_file_takes(untouched), untouched);
        }
        let mut odd = long.clone();
        odd.extend_from_slice(b"_24_00.bphys");
        assert_eq!(the_name_a_copied_cache_file_takes(&odd), odd, "not a cache file's shape");
    }

    #[test]
    fn an_added_write_place_the_walk_read_nothing_back_from_is_still_in_the_map() {
        // D21b: each of the five enters the destination set whether or not the
        // walk read a value back, and the redirect sets each one under the
        // job's folder unconditionally (the fifth, a Replay fluid cache's
        // folder, since 2026-10-02).
        let places: Vec<Destination> = crate::reading::EVERY_ADDED_WRITE_PLACE
            .iter()
            .map(|place| Destination {
                kind: DestinationKind::AnAddedWritePlace(*place),
                recorded_location: None,
                format: None,
                origin: WhereItComesFrom::AnAddedWritePlaceNamed(*place),
                scene: WhichScene::TheJobsOwnScene,
                the_view_layers_that_feed_it: Vec::new(),
                the_location_with_its_folder_replaced: None,
            })
            .collect();
        let held: Vec<ADestinationToMap<'_>> = places
            .iter()
            .map(|destination| ADestinationToMap {
                destination,
                the_longest_name_it_will_write: None,
            })
            .collect();
        let map = compose_the_map(
            &held,
            BLEND_SITS_IN,
            JOB_FOLDER,
            TheVersionInHand::of(5, 2),);
        assert_eq!(map.pairs.len(), 5);
        assert!(!map.a_destination_is_not_mappable());
        // **P7a: five places, five folders, each named for its own setting.**
        // D21f reads whether the ending walk
        // returned anything under *that place's* mapped folder, so four places
        // sharing one folder would make that reading say the same thing about
        // all four. **None of them is shown** (P11, 2026-10-04).
        assert!(map.the_folders_the_job_will_write_into(JOB_FOLDER).is_empty());
        let folders = map.the_folders_of(JOB_FOLDER, |_| true);
        assert_eq!(folders.len(), 5);
        assert!(folders.contains(&b"pointcache".to_vec()));
        assert!(folders.contains(&b"fluid-cache".to_vec()));
    }

    #[test]
    fn a_place_with_no_folder_shares_no_run_and_the_ordinary_file_stays_flat() {
        // **P7a, and D21c's own case**: the artist who recorded no path for any
        // of the four, *"the common case, not a corner"*. A rule that gave those four
        // the machine's root as their folder would find no run common to them
        // and to the render's own destination, and P2's rejected outcome would
        // arrive on almost every job — the artist's whole chain of folders to
        // walk down before reaching the frames.
        let regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/renders/frames/f_",
            WhereItComesFrom::TheScenesOutputPath,
        );
        let mut everything = vec![regular];
        everything.extend(crate::reading::EVERY_ADDED_WRITE_PLACE.iter().map(|place| {
            Destination {
                kind: DestinationKind::AnAddedWritePlace(*place),
                recorded_location: None,
                format: None,
                origin: WhereItComesFrom::AnAddedWritePlaceNamed(*place),
                scene: WhichScene::TheJobsOwnScene,
                the_view_layers_that_feed_it: Vec::new(),
                the_location_with_its_folder_replaced: None,
            }
        }));
        let held: Vec<ADestinationToMap<'_>> = everything
            .iter()
            .map(|destination| ADestinationToMap {
                destination,
                the_longest_name_it_will_write: match destination.kind {
                    DestinationKind::AnAddedWritePlace(_) => None,
                    _ => Some(b"f_0001.png".to_vec()),
                },
            })
            .collect();
        let map = compose_the_map(
            &held,
            BLEND_SITS_IN,
            JOB_FOLDER,
            TheVersionInHand::of(5, 2),);
        let folders = map.the_folders_the_job_will_write_into(JOB_FOLDER);
        // The one destination that has a folder keeps P7's *nothing left* case:
        // a folder of frames, in a folder named for the job that made them —
        // and it is the only folder shown (P11, 2026-10-04).
        assert_eq!(folders, vec![Vec::<u8>::new()]);
        assert_eq!(map.the_folders_of(JOB_FOLDER, |_| true).len(), 6);
    }
}

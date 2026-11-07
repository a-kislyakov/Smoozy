//! §7 — the expected-output inventory, the count and the collision record.
//!
//! **I12: the job's count is the owed files summed over its frames, and it is
//! never a multiplication.** **I13: the count is of the files the job is expected
//! to leave on disk, not of the writes it asks for** — a collision is two writes
//! and one file, so it is counted once. *"`Download` measures arrivals against
//! this number, so a number that can never be reached would leave a job short of
//! done for good."*
//!
//! **I14: the expected-output inventory is the count's own working, kept rather
//! than thrown away.** One entry per expected file, and **the count is the number
//! of those entries**. Nothing derives a count a second way, and nothing keys the
//! inventory per destination, which would count a collision twice against I13.
//!
//! **I5: the two recipes together were checked against a real render and were
//! right** — R6's 105 predicted against 105 written, on one fixture, in two
//! versions.
//!
//! **I24: the finding under the video half rests on one container and one
//! codec**, and the spec says outright that none of it should be read as a fact
//! about video output in general. **Nothing below keys on a container, a codec or
//! a size.**

use serde::{Deserialize, Serialize};

use crate::mapping::{HowItMapped, TheMap, SEPARATOR};
use crate::reading::{
    AnAddedWritePlace, ArtistsText, Destination, DestinationKind, TheOutputsHalf,
    TheVersionInHand, WhereItComesFrom, WhichBranch, WhichScene,
};

/// **I1: the split is forced rather than chosen, and the reading records which
/// of the two produced each name** so a mismatch found later says where to look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhichRouteProducedTheName {
    /// **Asked of the Blender that will render.** The regular output path's
    /// route, and the one that doubles as a check on a broken template.
    ItWasAskedOfBlender,
    /// **Built from the version's own recipe**, because there is no route to
    /// ask: from 5.0 a node displays one assembled name for the whole node
    /// before anything renders, and **no way was found in any version for a
    /// script to read that assembled name**, so the displayed name is not a
    /// route.
    ItWasBuiltFromTheVersionsRecipe,
}

/// **The frame number's width in a name a node's recipe builds, where the
/// file's own name holds no hash mark.**
///
/// **The plan gave this by observation rather than by rule, and the first
/// observation was one frame.** I2's two names on disk are `beauty0010.png` in
/// 4.5.9 and `fixture_52beauty0010.png` in 5.2.0, both at frame 10, both four
/// digits wide. **Observed again at the desk on 2026-10-04, in 4.5.12, 5.0.1,
/// 5.1.2 and 5.2.2** (`Audits/Build/Step 10c/The Creator's second notes, piece
/// 1 — what landed.md`, section 1): `beauty0001.png` to `beauty0003.png`, and
/// `beauty1000.png` at frame 1000 — four is a floor, and a wider number is
/// written whole.
///
/// **Where the file's own name holds hash marks this width is not used**:
/// [`the_name_a_node_writes`] puts the frame at the last run of them instead,
/// padded to the run's length — the rule the same desk test is the source of.
/// Until 2026-10-04 nothing carried I3's hash rule across to a node, and the
/// Creator's own job predicted `### - … Mattress0001.png` where Blender wrote
/// `001 - … Mattress.png`.
pub const THE_FRAME_NUMBERS_WIDTH_IN_A_NODES_NAME: usize = 4;

/// **I4: for a node in the job's own scene, the frame number is present in every
/// version, because every job renders as an animation.**
///
/// *"From 5.0 a node stamps the frame number only on an animation render; 4.5
/// stamps it every time."* **The one observation of a scene reached through a
/// sequencer strip goes the other way** — 4.5.9 wrote `sec0001.png` with a
/// number while 5.2.0 wrote `fixture_52sec.png` with none — and **that
/// observation is carried with its source's own limit and with no gloss on it**:
/// *"stated narrowly — one fixture, one scene reached through a strip, and the
/// general rule was not established"*.
///
/// **So the sentence is stated for the job's own scene and is not extended to a
/// sequencer-reached one, and this function is that boundary.** Whether a node
/// in a scene reached through a sequencer strip stamps the frame number from 5.0
/// is a question owed to Blender Research (§14 item 18), so **the answer for
/// that shape is not established and is not decided here** — the one observation
/// is applied as an observation of exactly the case it covers.
pub fn a_node_stamps_the_frame_number(scene: &WhichScene, version: TheVersionInHand) -> bool {
    match scene {
        // I4's own sentence, for the shape it is stated for.
        WhichScene::TheJobsOwnScene => true,
        // The one observation, applied to the one shape it was taken on and
        // never widened into a rule: 4.5 stamped a number, 5.2 did not.
        WhichScene::ReachedThroughASequencerStrip { .. } => {
            version.branch() == WhichBranch::FourFive
        }
    }
}

/// **I2, performed: the node recipe, which differs across the version
/// boundary.**
///
/// *"4.5 joins the node's folder, the output's own piece of path, the frame
/// number and the ending; 5.0, 5.1 and 5.2 put the node's own base file name
/// between the folder and the output's name."*
///
/// **Both halves of this recipe are the node's folder joined to something, and
/// in 4.5's gathering mode the folder is not a folder** — D8's separator rule is
/// what makes this recipe predict the name that is actually written, *"and
/// without it the prediction and the file part company by one path level with
/// nothing raised"*. **So the folder handed in here is always the mapped folder
/// the redirect set**, and D8's separator is the redirect's own act rather than
/// this recipe's. **Since 2026-10-04 (piece 1b) 4.5's gathering node keeps the
/// front of its name after that folder**, and it is handed in here as the base
/// file name ([`crate::mapping::the_name_front_of`]).
///
/// **Where the frame number goes — the hash-mark rule, observed at the desk on
/// 2026-10-04 in all four versions** (`Audits/Build/Step 10c/The Creator's
/// second notes, piece 1 — what landed.md`, section 1; the plan's sentence is
/// I2's, owed by the runner as P2 (c)). The assembled name is the base file
/// name and the output's own piece of path together, and **the file's own name
/// is the part of it after its last separator**:
///
/// - **where the file's own name holds a run of `#`, the last run takes the
///   frame number, padded with zeros to the run's length, and nothing is added
///   at the end** — `### - name` at frame 1 is `001 - name`, `name####` is
///   `name0001`, and frame 1000 in `###` is `1000 - name`, written whole;
/// - **an earlier run stays as written** — `##_##` at frame 1 is `##_01`, and
///   so is a run split between the base file name and the output's name;
/// - **a run in a folder part does not count** — `d##/x` is `d##/x0001`;
/// - **with no run in the file's own name, four digits go on the end**, as
///   before ([`THE_FRAME_NUMBERS_WIDTH_IN_A_NODES_NAME`]).
pub fn the_name_a_node_writes(
    the_nodes_base_file_name: Option<&ArtistsText>,
    the_outputs_own_piece_of_path: Option<&ArtistsText>,
    frame: Option<i64>,
    the_ending: Option<&ArtistsText>,
) -> Vec<u8> {
    let mut out = Vec::new();
    // **From 5.0 the node's own base file name sits between the folder and the
    // output's name.** In 4.5 there is none — the folder property is the base
    // path, and D8's separator rule has already ended it — so nothing goes here.
    if let Some(base) = the_nodes_base_file_name {
        out.extend_from_slice(base.as_bytes());
    }
    if let Some(piece) = the_outputs_own_piece_of_path {
        out.extend_from_slice(piece.as_bytes());
    }
    if let Some(frame) = frame {
        let the_files_own_name_starts = out
            .iter()
            .rposition(|byte| *byte == SEPARATOR)
            .map_or(0, |at| at + 1);
        match the_last_run_of_hash_marks(&out[the_files_own_name_starts..]) {
            Some((from, length)) => {
                let at = the_files_own_name_starts + from;
                out.splice(
                    at..at + length,
                    format!("{frame:0length$}").into_bytes(),
                );
            }
            None => out.extend_from_slice(
                format!("{frame:0width$}", width = THE_FRAME_NUMBERS_WIDTH_IN_A_NODES_NAME)
                    .as_bytes(),
            ),
        }
    }
    // **The format's ending last unless the artist switched it off** (I3), which
    // is an absence on the destination's own format and never an empty string.
    if let Some(ending) = the_ending {
        out.extend_from_slice(ending.as_bytes());
    }
    out
}

/// Where the last run of `#` in a name starts, and how long it is.
fn the_last_run_of_hash_marks(name: &[u8]) -> Option<(usize, usize)> {
    let ends = name.iter().rposition(|byte| *byte == b'#')? + 1;
    let starts = name[..ends]
        .iter()
        .rposition(|byte| *byte != b'#')
        .map_or(0, |at| at + 1);
    Some((starts, ends - starts))
}

/// One entry of the inventory: **one expected file** (I14).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnEntry {
    /// **The predicted path**, whole, on the machine — the destination's mapped
    /// folder joined to the predicted name.
    pub predicted_path: Vec<u8>,
    /// **The frame it belongs to.** Absent for a video destination's one entry,
    /// which carries no per-frame entries (I20).
    pub frame: Option<i64>,
    /// **Which route produced the name** (I1), so a mismatch found later says
    /// where to look.
    pub the_route_that_produced_the_name: WhichRouteProducedTheName,
    /// **The destination that aims at it — or both destinations, where two of
    /// them aimed at that one path** (I14, §7.4). They are places in the
    /// reading's own destination set, so an entry names a destination rather
    /// than copying its fields (I14a: *"no format is copied onto an inventory
    /// entry"*).
    pub the_destinations_aiming_at_it: Vec<usize>,
    /// **I20: a video destination's inventory entry is one file, marked as *may
    /// grow*.** *"The file is named for the whole range before Blender knows the
    /// render will finish; where the artist has Blender's own splitting on, a
    /// second file appears under a name of its own. So Smoozy states one and says
    /// plainly that a long job may leave more, rather than promising a number it
    /// will have to break."*
    pub may_grow: bool,
}

/// **The added write places carried in the same inventory as folders** (I14).
///
/// *"Each with its mapped folder, no predicted per-file entries, and no
/// contribution to the count. A bake or a cache writes files this stage has no
/// recipe to name and no method to count."*
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnAddedWritePlacesFolder {
    pub place: AnAddedWritePlace,
    pub mapped_folder: Vec<u8>,
    /// **Whether the mapped folder is the place's own**, which is what makes
    /// D21f's reading sound. Added 2026-09-05, following the plan's repair out
    /// of a code round.
    ///
    /// ~~A place the walk read a value back from goes through the ordinary
    /// shared-run rule~~ — **not from 2026-10-04**: every place takes its own
    /// named folder (P7a widened). **One shape can still share it**: a render
    /// destination whose folder, after the shared run, is that same name or
    /// sits beneath it (`<job>/bake` for an artist's own `…/bake` folder).
    /// **D21f reads whether the ending walk returned any entry under that
    /// place's mapped folder** — shared, that is a fixed **yes** where the
    /// cache wrote nothing. **So the reading is withheld and stated as withheld
    /// where this is false.**
    ///
    /// **It is computed once, here, off the map** — the one place that holds
    /// every destination's folder — rather than derived a second way later.
    pub the_folder_is_its_own: bool,
}

/// One member of a colliding pair, with where it comes from.
///
/// **I16a: each member of a colliding pair carries where it comes from, by
/// name, and the origin is a field of the destination in the recorded
/// reading.** *"The path alone is the same string twice, and the naming is what
/// sends the artist to the right two places in their own file."*
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AColliderInTheRecord {
    /// The destination's place in the reading's own set.
    pub destination: usize,
    /// **The origin, by name** — *the scene's output path*, *File Output node
    /// Beauty*. It is the destination's own field and is not composed here.
    pub origin: WhereItComesFrom,
}

/// **I15's first set: the collision record** — what the user is warned about,
/// and what holds a job under H1's third condition.
///
/// *"The spec's own case: two destinations **in different nodes** aimed at the
/// same name in the same place. Two outputs on one node cannot collide, because
/// Blender adds a number to a repeated name as it is typed; a node group used
/// twice in one tree is the same case as two nodes."*
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ACollidingPair {
    pub predicted_path: Vec<u8>,
    /// **Both members are recorded, because the warning names two
    /// destinations** (I16).
    pub members: Vec<AColliderInTheRecord>,
}

/// The inventory, the count's own working kept (I14).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheInventory {
    pub entries: Vec<AnEntry>,
    pub added_write_place_folders: Vec<AnAddedWritePlacesFolder>,
    /// **I17: the collision record is computed once, over the map's new
    /// folders**, and P16 is why that is the same answer as computing it over
    /// the original folders.
    pub collision_record: Vec<ACollidingPair>,
    /// **I22: where any of a job's destinations is video-shaped, the
    /// already-there machinery is switched off for the whole job.**
    pub a_destination_is_video_shaped: bool,
    /// **H1a's reason, where the count is zero**, composed with the count and
    /// from the same per-frame set — **so with the job's pins applied** (R4
    /// item 5). Added 2026-10-04 by piece 1b, on the runner's ruling: H1a's
    /// reason had been read again off the recorded reading, which stays the
    /// file's, so a job whose every layer is pinned off was told *nothing was
    /// aimed anywhere* instead of *no frame has an active view layer*.
    ///
    /// **Absent on an inventory recorded before that date**, and then
    /// [`crate::holds::the_six_things`] reads the reason off the recorded
    /// reading as it always did — no choices, today's answer.
    #[serde(default)]
    pub why_it_owes_nothing: Option<crate::holds::WhyItWillWriteNothing>,
}

impl TheInventory {
    /// **I12 and I14: the count is the number of entries, and nothing derives a
    /// count a second way.**
    pub fn the_count(&self) -> usize {
        self.entries.len()
    }

    /// **I14a's per-destination figure: the number of entries naming that
    /// destination.**
    ///
    /// **It is not the total's working.** *"An entry naming two destinations is
    /// one file and appears in both figures, so the per-destination figures may
    /// sum above the count, and the count is never derived by adding them."*
    pub fn the_figure_for(&self, destination: usize) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.the_destinations_aiming_at_it.contains(&destination))
            .count()
    }

    /// **I14a's split by format: a count of entries, grouped by the format the
    /// destination each entry names carries.**
    ///
    /// *"Every entry falls in exactly one group and the groups sum to the count,
    /// so a collision is counted once in the split exactly as it is counted once
    /// in the total."*
    ///
    /// **A colliding entry whose two destinations carry different formats is
    /// grouped under the format of the first destination the entry records**,
    /// deterministically — *"and which format the file left on disk actually
    /// carries is not decidable before the render"* (§14 item 20).
    /// **An entry whose format is not recorded falls in a group of its own that
    /// says so, rather than dropping out of the split.** Corrected 2026-09-05,
    /// out of a code round: two silent skips — an entry naming no destination,
    /// and a destination carrying no format — took entries out of the split
    /// altogether, so **I14a's invariant broke with nothing said** and the page
    /// drawing the split showed groups adding to less than the count, which a
    /// reader could not tell from I14a's other figure, the per-destination one
    /// that may legitimately sum above it. **The key is an optional format name
    /// and absent means *not recorded*** — a value no real format name can be
    /// confused with.
    pub fn the_split_by_format(
        &self,
        destinations: &[Destination],
    ) -> Vec<(Option<Vec<u8>>, usize)> {
        let mut out: Vec<(Option<Vec<u8>>, usize)> = Vec::new();
        for entry in &self.entries {
            let name = entry
                .the_destinations_aiming_at_it
                .first()
                .and_then(|first| destinations.get(*first))
                .and_then(|destination| destination.format.as_ref())
                .map(|format| format.as_the_file_records_it.as_bytes().to_vec());
            match out.iter_mut().find(|(already, _)| *already == name) {
                Some((_, how_many)) => *how_many += 1,
                None => out.push((name, 1)),
            }
        }
        out
    }
}

/// **I9a: why an inventory was not composed at all.**
///
/// *"Where the reading finds the scene writing more than one view, the
/// expected-output inventory is not composed for that job: the reckoning records
/// the frame set as **not predictable, with the reason named**."* **Nothing then
/// reads absence as evidence** — no frame is called missing, no carry-on
/// re-renders anything, and the count is withheld rather than being wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhyTheFrameSetIsNotPredictable {
    /// **The scene writes more than one view, written separately.** With two
    /// views switched on, no file with the predicted name is written at all —
    /// the files carry a per-view suffix, **and this document composes no
    /// per-view names, deliberately**.
    TheSceneWritesMoreThanOneViewSeparately,
    /// **A file output node in a scene reached through a sequencer strip, from
    /// 5.0 on: the name the recipe builds carries no frame number, so every
    /// frame of that destination predicts one path.** Added 2026-09-05, out of a
    /// code round.
    ///
    /// I4 declines to decide the shape — the one observation of a
    /// sequencer-reached scene is *"neither read as a general rule nor
    /// denied"*, and §14 item 18 is permanently unresolved — **and a name
    /// built either way is a guess at a closed Blender fact.** Stamping the
    /// number predicts a name that may never be written, so every frame reads
    /// as absent and an interrupted job re-renders its whole range at the
    /// user's expense. Not stamping it, which is what the recipe did, gives
    /// **five entries at one path over five frames**: I14's *one entry per
    /// expected file* and I13's *of the files the job is expected to leave on
    /// disk* both break, the completeness statement matches each entry
    /// separately against the walk and **nothing consumes a walk entry**, so
    /// one file on disk answers all five and **Smoozy reports a job complete
    /// that left one file where it promised a hundred**.
    ///
    /// **So no count is composed for that job, which is I9a's own answer to
    /// its nearest question**: the names cannot be predicted, the reason is
    /// named, no frame is called missing and nothing reads absence as
    /// evidence. **No count is honest; a wrong count is not.** The cost is
    /// stated and is real — such a job re-renders whole on a carry-on, which
    /// is what a job with no already-there test has always cost.
    ///
    /// **4.5 is unchanged and keeps predicting**: there the node stamps the
    /// number, which is what I4 states for the job's own scene as well, so
    /// 4.5's answer is not a second guess.
    AFileOutputNodeInASequencerReachedSceneFromFiveZero,
    /// **A destination the map recorded as not mappable.** Added 2026-09-05,
    /// out of a code round.
    ///
    /// §6.4's cases — a length that will not fit, or a 4.5 output name that
    /// climbs out of its node's folder, **which P3 says the rule makes more
    /// likely by construction**. Nothing is predicted for such a destination,
    /// and until this it was dropped without trace: where every one of the
    /// render's own destinations went that way the count came to zero, and
    /// **three things read the zero as a fact** — H1's second condition said
    /// *nothing was aimed anywhere* about a job that plainly aimed somewhere,
    /// C27's completeness statement said nought expected and nought present so
    /// a full range read as complete, and C14 counted every frame already there
    /// so a re-run handed back one frame and called the job done.
    ///
    /// **The first hold already fires and is true.** What was missing is that
    /// nothing carried it into the completeness statement or the reckoning, and
    /// **withholding is what carries it**.
    ADestinationIsNotMappable,
    /// **A frame's regular output name was not answered.** Added 2026-09-05,
    /// out of a code round.
    ///
    /// I1 asks the regular output path's name of the Blender that will render.
    /// **An unanswered name means the name is not known**, and it is not the
    /// same thing as [`crate::reading::GatingConditions::the_scenes_own_frames_are_written`]
    /// coming back false, which means *this destination writes nothing at this
    /// frame* and is right. Until this both left the same absence.
    TheRegularOutputPathsNameWasNotAnswered,
    /// **A frame owes no file the reading can name, because every file it
    /// would owe comes from a node whose names hold a curly-brace expression
    /// Smoozy does not replace.** Added 2026-10-18, `Architecture/Job files
    /// and outputs` I2's *What an unresolved name does* (the Creator's
    /// thirteenth notes, notes 5, 9 and 10): I9a's discipline, never frames
    /// that write nothing (I10). The expression itself is named in the reading,
    /// beside the node's raw text ([`crate::reading::TheNodesTextReplaced`]).
    AFramesFilesAreNamedByAnExpressionThatIsNotReplaced,
}

/// The inventory, or the statement that it was not composed (I9a).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatTheInventoryIs {
    It(TheInventory),
    /// **Not composed, with the reason named.** *"That is the same discipline
    /// C23 already applies to a status that cannot be trusted, applied to a name
    /// that cannot be predicted."*
    TheFrameSetIsNotPredictable(WhyTheFrameSetIsNotPredictable),
}

/// **I6: for each frame the job covers, the file is asked six things and the
/// frame's owed files follow from them.**
///
/// 1. which view layers render at that frame;
/// 2. **how many views the scene writes** (I9a);
/// 3. what the regular output path writes;
/// 4. which file output nodes will run and what is connected to each;
/// 5. **whether the job's scene renders through its own sequencer holding a
///    Scene strip, in which case its file output nodes write nothing at all**
///    (§14 item 13);
/// 6. and where two of those land on one name.
///
/// **It was four until 2026-09-01**, when the repaired Blender research settled
/// the two that were missing; **both additions take files out of the count**, and
/// a count that is too high reads as a job with missing frames for ever.
///
/// **I11: what a node writes need not be the render's own picture, and the count
/// is unchanged by that.** It is still one file per frame, so nothing here looks
/// at what a node is fed.
pub fn compose_the_inventory(reading: &TheOutputsHalf, map: &TheMap) -> WhatTheInventoryIs {
    // **Question 2, and it is asked before anything is composed.** I9a: where
    // the scene writes more than one view separately, the inventory is not
    // composed for that job at all.
    if !reading.views.the_names_are_predictable() {
        return WhatTheInventoryIs::TheFrameSetIsNotPredictable(
            WhyTheFrameSetIsNotPredictable::TheSceneWritesMoreThanOneViewSeparately,
        );
    }
    // **Asked before anything is composed too**: a name built with no frame
    // number in it predicts one path for every frame of the destination.
    if reading
        .destinations
        .iter()
        .any(|destination| the_name_would_carry_no_frame_number(destination, reading.taken_by))
    {
        return WhatTheInventoryIs::TheFrameSetIsNotPredictable(
            WhyTheFrameSetIsNotPredictable::AFileOutputNodeInASequencerReachedSceneFromFiveZero,
        );
    }

    let mut entries: Vec<AnEntry> = Vec::new();
    let mut added_write_place_folders = Vec::new();
    let mut a_destination_is_video_shaped = false;
    // **The frames at which an output whose name holds an expression Smoozy
    // does not replace would write** — it owes no entry, and a frame left
    // owing nothing nameable is withheld below.
    let mut frames_owing_an_unresolved_name: Vec<i64> = Vec::new();

    for (at, destination) in reading.destinations.iter().enumerate() {
        let mapped = match map.the_pair_for(destination) {
            // **A destination the map recorded as *not mappable* holds the job
            // under H1's first condition, and the inventory is withheld with
            // its own reason** rather than the destination being dropped out of
            // a count that then reads as a fact.
            Some(pair) => match &pair.outcome {
                HowItMapped::UnderTheJobsFolder { new_folder, .. } => new_folder.as_slice(),
                HowItMapped::NotMappable(_) => {
                    return WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                        WhyTheFrameSetIsNotPredictable::ADestinationIsNotMappable,
                    )
                }
            },
            // The map is composed over every destination in the set (P10), so a
            // destination with no pair at all does not arise.
            None => continue,
        };
        if let Some(place) = destination.as_an_added_write_place() {
            // **I14: an added write place is carried as a folder, with no
            // per-file entries and no contribution to the count.**
            added_write_place_folders.push(AnAddedWritePlacesFolder {
                place,
                the_folder_is_its_own: no_other_destination_writes_at_or_under(map, mapped),
                mapped_folder: mapped.to_vec(),
            });
            continue;
        }
        let video = destination
            .format
            .as_ref()
            .is_some_and(|format| format.is_video_shaped);
        if video && destination.its_folder_is_unresolved() {
            // **The video's folder holds an expression Smoozy does not
            // replace** (I2, 2026-10-18): no entry, and every frame it would
            // have named owes a file the reading cannot name.
            frames_owing_an_unresolved_name.extend(reading.per_frame.iter().map(|at| at.frame));
            continue;
        }
        if video {
            // **I19: a video destination can only ever be the regular output
            // path**, and I20 gives it one entry marked *may grow* with no
            // per-frame entries.
            a_destination_is_video_shaped = true;
            entries.push(AnEntry {
                predicted_path: the_video_files_path(reading, mapped),
                frame: None,
                the_route_that_produced_the_name: WhichRouteProducedTheName::ItWasAskedOfBlender,
                the_destinations_aiming_at_it: vec![at],
                may_grow: true,
            });
            continue;
        }
        for frame in &reading.per_frame {
            let name = match the_name_owed_at(reading, destination, frame) {
                WhatIsOwedAt::AName(name) => name,
                WhatIsOwedAt::NothingIsOwed => continue,
                // **I2's *What an unresolved name does*: no inventory entry**,
                // so it adds nothing to the count and holds no frame back; its
                // file comes down at the ending walk (`Download` J8).
                WhatIsOwedAt::TheNameIsUnresolved => {
                    frames_owing_an_unresolved_name.push(frame.frame);
                    continue;
                }
                // **The name is not known, which is not the same as nothing
                // being owed.** Withheld rather than left as an absence three
                // things downstream would read as a fact.
                WhatIsOwedAt::TheNameIsNotKnown => {
                    return WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                        WhyTheFrameSetIsNotPredictable::TheRegularOutputPathsNameWasNotAnswered,
                    )
                }
            };
            let mut predicted_path = mapped.to_vec();
            predicted_path.push(SEPARATOR);
            predicted_path.extend_from_slice(&name);
            let route = match destination.kind {
                DestinationKind::TheRegularOutputPath => {
                    WhichRouteProducedTheName::ItWasAskedOfBlender
                }
                _ => WhichRouteProducedTheName::ItWasBuiltFromTheVersionsRecipe,
            };
            // **I13 and I16: a collision is two writes and one file, so it is
            // counted once** — the second destination joins the entry the first
            // made rather than adding one.
            match entries
                .iter_mut()
                .find(|entry| entry.predicted_path == predicted_path && entry.frame == Some(frame.frame))
            {
                Some(already) => {
                    if !already.the_destinations_aiming_at_it.contains(&at) {
                        already.the_destinations_aiming_at_it.push(at);
                    }
                }
                None => entries.push(AnEntry {
                    predicted_path,
                    frame: Some(frame.frame),
                    the_route_that_produced_the_name: route,
                    the_destinations_aiming_at_it: vec![at],
                    may_grow: false,
                }),
            }
        }
    }

    // **A frame left owing no file the reading can name is not predictable,
    // with the reason named** (I2's *What an unresolved name does*, I9a's
    // discipline) — never a frame that writes nothing (I10). A video's one
    // entry names the file every frame goes into.
    let a_frame_owes_nothing_nameable = !a_destination_is_video_shaped
        && frames_owing_an_unresolved_name
            .iter()
            .any(|frame| !entries.iter().any(|entry| entry.frame == Some(*frame)));
    if a_frame_owes_nothing_nameable {
        return WhatTheInventoryIs::TheFrameSetIsNotPredictable(
            WhyTheFrameSetIsNotPredictable::AFramesFilesAreNamedByAnExpressionThatIsNotReplaced,
        );
    }

    let collision_record = the_collision_record(&entries, &reading.destinations);
    // **H1a's reason, read off the very per-frame set this count was made
    // from** — the job's pins applied where the composition applied them
    // (R4 item 5's one rule, [`crate::reading::what_the_jobs_pin_says`]).
    let why_it_owes_nothing = entries
        .is_empty()
        .then(|| crate::holds::why_it_will_write_nothing(reading));
    WhatTheInventoryIs::It(TheInventory {
        entries,
        added_write_place_folders,
        collision_record,
        a_destination_is_video_shaped,
        why_it_owes_nothing,
    })
}

/// **Whether a destination will write something the reading cannot name** (I2's
/// *What an unresolved name does*). It owes no entry, so its figure is nought,
/// and it is not a destination that writes nothing: the page keeps its row.
pub fn it_writes_what_the_reading_cannot_name(reading: &TheOutputsHalf, destination: &Destination) -> bool {
    if destination.format.as_ref().is_some_and(|format| format.is_video_shaped) {
        return destination.its_folder_is_unresolved();
    }
    reading
        .per_frame
        .iter()
        .any(|frame| matches!(the_name_owed_at(reading, destination, frame), WhatIsOwedAt::TheNameIsUnresolved))
}

/// **Whether an added write place's mapped folder is its own** — D21f's
/// condition, read off the map.
///
/// **The walk D21f reads returns every entry under the folder**, so the reading
/// is sound only where nothing else in the map writes at that folder or beneath
/// it. That covers both shapes the plan names: **shared with a render
/// destination** (P17 keeps two destinations that were the same place in the
/// same place), and **the job output folder itself** (P7's *nothing left*),
/// where every other destination sits beneath it.
fn no_other_destination_writes_at_or_under(map: &TheMap, folder: &[u8]) -> bool {
    let beneath = [folder, &[SEPARATOR]].concat();
    let mut at_this_folder = 0;
    for pair in &map.pairs {
        let HowItMapped::UnderTheJobsFolder { new_folder, .. } = &pair.outcome else {
            continue;
        };
        if new_folder.as_slice() == folder {
            at_this_folder += 1;
            continue;
        }
        if new_folder.starts_with(&beneath) {
            return false;
        }
    }
    // The place's own pair is one of them; a second is another destination
    // sharing the folder.
    at_this_folder == 1
}

/// **Whether I2's recipe would build this destination's name with no frame
/// number in it**, which predicts one path for every frame of it.
///
/// It is a file output node's destination in a scene reached through a
/// sequencer strip, read by a version [`a_node_stamps_the_frame_number`] answers
/// **no** for — which is 5.0 and after. **That function and the test holding its
/// boundary are unchanged**; what changed on 2026-09-05 is that its *no* stops
/// being used to build a name and withholds the whole inventory instead.
fn the_name_would_carry_no_frame_number(
    destination: &Destination,
    version: TheVersionInHand,
) -> bool {
    let it_is_a_nodes_destination = matches!(
        destination.kind,
        DestinationKind::AnOutputOfAFileOutputNode { .. }
            | DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { .. }
    );
    it_is_a_nodes_destination && !a_node_stamps_the_frame_number(&destination.scene, version)
}

/// **The one file a video destination is expected to leave** (I20).
///
/// The name is Blender's own answer for the range, taken from the reading's
/// first frame answer, because **the file is named for the whole range before
/// Blender knows the render will finish** — so there is one name and it is not
/// per frame.
fn the_video_files_path(reading: &TheOutputsHalf, mapped: &[u8]) -> Vec<u8> {
    let mut out = mapped.to_vec();
    if let Some(answered) = reading
        .per_frame
        .iter()
        .find_map(|frame| frame.the_regular_output_paths_answered_path.as_ref())
    {
        out.push(SEPARATOR);
        out.extend_from_slice(the_part_after_the_last_separator(answered.as_bytes()));
    }
    out
}

/// What a destination owes at one frame — and **an absence and an unknown are
/// two answers here rather than one**.
///
/// Added 2026-09-05, out of a code round: the two used to be the same `None`,
/// so a name Blender never answered read exactly like a destination that owes
/// nothing at that frame.
pub(crate) enum WhatIsOwedAt {
    /// The predicted name, without its folder.
    AName(Vec<u8>),
    /// **Nothing is owed and that is right** — I8's view layer that does not
    /// render, I10's frame with none active, D12's switch off.
    NothingIsOwed,
    /// **The name is not known.** Only the regular output path reaches this,
    /// because it is the one route asked of Blender (I1).
    TheNameIsNotKnown,
    /// **A node's folder, base file name or output's own name holds a
    /// curly-brace expression Smoozy does not replace** (I2, added
    /// 2026-10-18). Blender writes a file, under a name the reading cannot
    /// give, so the output owes no entry.
    TheNameIsUnresolved,
}

/// **I6 to I10, for one destination at one frame: what it owes, if anything.**
///
/// **Visible inside this crate since 2026-09-20**, so that N8's budget is
/// measured against the name this recipe predicts rather than against a second
/// copy of it: [`crate::composing`] hands the longest of these to
/// [`crate::mapping::compose_the_map`]. **It stays private to the crate** —
/// what a destination owes is read through the inventory everywhere else.
pub(crate) fn the_name_owed_at(
    reading: &TheOutputsHalf,
    destination: &Destination,
    frame: &crate::reading::AtOneFrame,
) -> WhatIsOwedAt {
    // **I10: frames that produce nothing are counted as nothing.** A frame with
    // no view layer active on it renders nothing and writes nothing, and a job
    // can carry a stretch of those mid-range. *"A count that multiplied frames
    // by outputs would promise files that are never coming."*
    if frame.view_layers_that_render.is_empty() {
        return WhatIsOwedAt::NothingIsOwed;
    }
    // **I8: a view layer that does not render at a frame writes nothing at that
    // frame, so a node input fed only by it owes no file.**
    if !destination.the_view_layers_that_feed_it.is_empty()
        && !destination
            .the_view_layers_that_feed_it
            .iter()
            .any(|fed_by| {
                frame
                    .view_layers_that_render
                    .iter()
                    .any(|rendering| rendering.as_bytes() == fed_by.as_bytes())
            })
    {
        return WhatIsOwedAt::NothingIsOwed;
    }
    match &destination.kind {
        DestinationKind::TheRegularOutputPath => {
            // **D12: from 5.2 a switch inside the file stops the scene's own
            // frames being written even when a path is handed in on the command
            // line, while a node in the same tree keeps writing.** The count and
            // the inventory are composed from what it says, *"so the redirect
            // never promises a file that switch will prevent"*.
            if !frame.gating.the_scenes_own_frames_are_written() {
                return WhatIsOwedAt::NothingIsOwed;
            }
            // **I2's *The regular output path's folder is read the same way*
            // (2026-10-18)**: a folder holding an expression Smoozy does not
            // replace leaves its entries unresolved, as a node's are.
            if destination.its_folder_is_unresolved() {
                return WhatIsOwedAt::TheNameIsUnresolved;
            }
            // **I9: view layers never multiply the count.** A frame with several
            // active view layers writes **one** set of files from the regular
            // output path, never one per view layer; no view layer's name goes
            // into a file name; and only a multi-layer file carries them all,
            // inside the one file.
            // **An unanswered name is not an absence.** The switch above says
            // this destination writes nothing at this frame and is right; this
            // says the name is not known, and the two used to leave the same
            // answer.
            let Some(answered) = frame.the_regular_output_paths_answered_path.as_ref() else {
                return WhatIsOwedAt::TheNameIsNotKnown;
            };
            WhatIsOwedAt::AName(the_part_after_the_last_separator(answered.as_bytes()).to_vec())
        }
        DestinationKind::AnOutputOfAFileOutputNode {
            output,
            the_nodes_base_file_name,
            ..
        } => {
            if !the_scenes_nodes_will_write(reading, destination, frame) {
                return WhatIsOwedAt::NothingIsOwed;
            }
            // **I2: the recipe joins the replaced text and never the raw
            // text**; a reading taken before 2026-10-18 is taken as raw.
            let (base, output) = match destination.the_replaced_text() {
                None => (the_nodes_base_file_name.as_ref(), Some(output)),
                Some(replaced) => {
                    if replaced.the_unresolved_expression().is_some() {
                        return WhatIsOwedAt::TheNameIsUnresolved;
                    }
                    (
                        replaced.the_base_file_name.as_ref().and_then(|base| base.the_text()),
                        replaced.the_outputs_own_name.as_ref().and_then(|output| output.the_text()),
                    )
                }
            };
            WhatIsOwedAt::AName(the_name_a_node_writes(
                base,
                output,
                a_node_stamps_the_frame_number(&destination.scene, reading.taken_by)
                    .then_some(frame.frame),
                the_ending_of(destination),
            ))
        }
        DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile {
            the_nodes_base_file_name,
            ..
        } => {
            if !the_scenes_nodes_will_write(reading, destination, frame) {
                return WhatIsOwedAt::NothingIsOwed;
            }
            // **I2: the replaced text**, as above; in 4.5 the front is read off
            // the replaced folder property ([`crate::mapping::the_name_front_of`]).
            let base = match destination.the_replaced_text() {
                None => the_nodes_base_file_name.as_ref(),
                Some(replaced) => {
                    if replaced.the_unresolved_expression().is_some() {
                        return WhatIsOwedAt::TheNameIsUnresolved;
                    }
                    replaced.the_base_file_name.as_ref().and_then(|base| base.the_text())
                }
            };
            // **I7: one file for the whole node, whatever the number of
            // outputs**, so no output's own piece of path goes into the name.
            //
            // **In 4.5 there is no base file name and the node's folder property
            // is the start of the file name** (D8). ~~The redirect ends that
            // property with a separator, always, so nothing precedes the frame
            // number.~~ **Corrected 2026-10-04 by piece 1b: the redirect keeps
            // the part after the property's last separator as the start of the
            // name** ([`crate::mapping::the_name_front_of`]), so it stands where
            // 5.0's base file name stands and the hash rule fills it.
            let the_front = crate::mapping::the_name_front_of(destination, reading.taken_by)
                .map(|front| ArtistsText::of(front.to_vec()));
            WhatIsOwedAt::AName(the_name_a_node_writes(
                base.or(the_front.as_ref()),
                None,
                a_node_stamps_the_frame_number(&destination.scene, reading.taken_by)
                    .then_some(frame.frame),
                the_ending_of(destination),
            ))
        }
        DestinationKind::AnAddedWritePlace(_) => WhatIsOwedAt::NothingIsOwed,
    }
}

/// **I6's fifth question, and R4 item 1's second thing that reaches a file
/// output node.**
///
/// A destination in the job's own scene is read against the frame's own gating;
/// **a scene reached through a sequencer strip contributes its own destinations,
/// read against its own gating conditions rather than the job's** (R4 item 3).
fn the_scenes_nodes_will_write(
    reading: &TheOutputsHalf,
    destination: &Destination,
    frame: &crate::reading::AtOneFrame,
) -> bool {
    match &destination.scene {
        // **I8's second paragraph: from 5.3 a node writes only at a frame its
        // own effect is on for render**, set, keyed or driven.
        WhichScene::TheJobsOwnScene => {
            frame.gating.a_file_output_node_will_write()
                && frame.gating.a_nodes_effect_is_on(destination.the_effect())
        }
        WhichScene::ReachedThroughASequencerStrip { scene, .. } => reading
            .scenes_reached_through_a_sequencer_strip
            .iter()
            .find(|reached| reached.scene.as_bytes() == scene.as_bytes())
            .is_some_and(|reached| {
                reached.gating.a_file_output_node_will_write()
                    && reached.gating.a_nodes_effect_is_on(destination.the_effect())
            }),
    }
}

/// The ending a destination's format writes, where the artist has not switched
/// it off (I3).
fn the_ending_of(destination: &Destination) -> Option<&ArtistsText> {
    destination
        .format
        .as_ref()?
        .the_ending_it_writes
        .as_ref()
}

/// The part of a path after its last separator — which is what Blender itself
/// splits the regular output path at (P4's third bullet).
fn the_part_after_the_last_separator(path: &[u8]) -> &[u8] {
    match path.iter().rposition(|byte| *byte == SEPARATOR) {
        Some(at) => &path[at + 1..],
        None => path,
    }
}

/// **I15's first set, computed: the collision record.**
///
/// **Two destinations in different nodes aimed at the same name in the same
/// place.** *"Two outputs on one node cannot collide, because Blender adds a
/// number to a repeated name as it is typed; a node group used twice in one tree
/// is the same case as two nodes."*
///
/// **I15's second set — the count's own test — is wider and is not this**: any
/// two destinations in the map predicting one path in one folder, which takes in
/// a node destination landing on the regular output path's own predicted name.
/// **The two sets can differ, and where they do the count is right and no warning
/// is raised**; H1's third condition reads this set and never the count's test.
fn the_collision_record(entries: &[AnEntry], destinations: &[Destination]) -> Vec<ACollidingPair> {
    let mut out = Vec::new();
    for entry in entries {
        if entry.the_destinations_aiming_at_it.len() < 2 {
            continue;
        }
        let members: Vec<AColliderInTheRecord> = entry
            .the_destinations_aiming_at_it
            .iter()
            .filter_map(|at| {
                destinations.get(*at).map(|destination| AColliderInTheRecord {
                    destination: *at,
                    origin: destination.origin.clone(),
                })
            })
            .collect();
        // **Node to node, and never a node against the regular output path.**
        if !two_or_more_are_a_file_output_nodes(&members) {
            continue;
        }
        // **A path already recorded is not recorded twice**: the record is about
        // the pair of destinations, and a pair that collides at every frame
        // would otherwise be one warning per frame.
        if out
            .iter()
            .any(|already: &ACollidingPair| already.members == members)
        {
            continue;
        }
        out.push(ACollidingPair {
            predicted_path: entry.predicted_path.clone(),
            members,
        });
    }
    out
}

/// Whether **two or more of the members are destinations of the file output node
/// kind**, which is the record's own condition (I15).
///
/// **The members are told apart by the destination the map found and never by
/// the node name it carries.** Corrected 2026-09-05, out of a code round: the
/// test was two distinct node **names**, and **a file output node inside a group
/// used twice puts one node in two places**, so both members carried the same
/// name, the test said one node, and the pair was dropped — no warning raised,
/// H1's third condition not fired, and one picture silently overwritten, on the
/// exact case I15 names as belonging in the record: *"a node group used twice in
/// one tree is the same case as two nodes."* Nothing else caught it, because the
/// count's wider test holds nothing and warns nobody by its own declaration
/// (I18).
///
/// **The record's real boundary is kept** — node to node, and never a node
/// against the regular output path, since only file-output-node members are
/// counted. **And nothing the plan claims is lost**: I15 says outright that two
/// outputs on one node cannot collide, because Blender numbers a repeated name
/// as it is typed.
///
/// **The Blender half is parked and not answered** — §14 item 29. This is right
/// wherever the reading produces two destinations for a doubly-used group; where
/// it produces one, nothing in the code can see the collision.
fn two_or_more_are_a_file_output_nodes(members: &[AColliderInTheRecord]) -> bool {
    let mut destinations: Vec<usize> = Vec::new();
    for member in members {
        if !matches!(member.origin, WhereItComesFrom::AFileOutputNodeNamed(_)) {
            continue;
        }
        if destinations.contains(&member.destination) {
            continue;
        }
        destinations.push(member.destination);
    }
    destinations.len() >= 2
}

/// **I18: the collision record is one of the three conditions that hold a job;
/// the count's wider test holds nothing and warns nobody.**
pub const THE_COUNTS_WIDER_TEST_HOLDS_NOTHING_AND_WARNS_NOBODY: bool = true;

/// **I21: nothing judges a video destination's progress by watching the file.**
///
/// Its size *"does not move while the render runs — it appears within moments at
/// a token size and stays there, reaching its real size only when the file is
/// closed"*. **A size that does not move is not the same as nothing being
/// written**, and neither reading is taken: `State, identity and recovery` §1.3
/// rule 5 forbids it anyway.
pub const NOTHING_WATCHES_A_VIDEO_FILE_TO_JUDGE_PROGRESS: bool = true;

/// **I23: the branch is taken before submitting and the warning is worth nothing
/// afterwards**, which is why the shape is in the recorded reading and therefore
/// in the manifest or in the first attempt's record.
pub const THE_VIDEO_SHAPE_IS_A_FIELD_OF_THE_READING_AND_NOT_A_LATER_TEST: bool = true;

/// **I22: running a video job again replaces what was left rather than adding to
/// it, so nothing has to be cleared first.**
pub const A_VIDEO_JOB_RUN_AGAIN_REPLACES_WHAT_WAS_LEFT: bool = true;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::main_output::WhatTheWalkFound;
    use crate::mapping::{compose_the_map, ADestinationToMap};
    use crate::reading::tests::{a_destination, a_location, plain_gating};
    use crate::reading::{AtOneFrame, Format, GatingConditions, HowManyViewsTheSceneWrites};

    pub(crate) const BLEND_SITS_IN: &[u8] = b"/ephemeral/smoozy/assets/set-1";
    pub(crate) const JOB_FOLDER: &str = "/ephemeral/smoozy/jobs/job-7";

    pub(crate) fn text(of: &str) -> ArtistsText {
        ArtistsText::of(of.as_bytes().to_vec())
    }

    pub(crate) fn a_frame(frame: i64, view_layers: &[&str], at: Option<&str>) -> AtOneFrame {
        AtOneFrame {
            frame,
            view_layers_that_render: view_layers.iter().map(|name| text(name)).collect(),
            gating: plain_gating(),
            the_regular_output_paths_answered_path: at.map(a_location),
        }
    }

    pub(crate) fn a_reading(
        destinations: Vec<Destination>,
        per_frame: Vec<AtOneFrame>,
        version: TheVersionInHand,
    ) -> TheOutputsHalf {
        TheOutputsHalf {
            taken_by: version,
            gating: plain_gating(),
            destinations,
            scenes_reached_through_a_sequencer_strip: Vec::new(),
            the_tree_is_shared_with: None,
            per_frame,
            recorded_locations: Vec::new(),
            views: HowManyViewsTheSceneWrites::One,
            the_beauty_walk: WhatTheWalkFound::NoEntry,
            the_view_transform: None,
        }
    }

    pub(crate) fn a_node_destination(node: &str, output: &str, at: &str, base: Option<&str>) -> Destination {
        let mut destination = a_destination(
            DestinationKind::AnOutputOfAFileOutputNode {
                the_effect: None,
                node: text(node),
                output: text(output),
                the_nodes_base_file_name: base.map(text),
                replaced: None,
            },
            at,
            WhereItComesFrom::AFileOutputNodeNamed(text(node)),
        );
        destination.format = Some(Format {
            as_the_file_records_it: text("PNG"),
            the_ending_it_writes: Some(text(".png")),
            is_video_shaped: false,
        });
        destination
    }

    pub(crate) fn mapped(reading: &TheOutputsHalf, version: TheVersionInHand) -> TheMap {
        let held: Vec<ADestinationToMap<'_>> = reading
            .destinations
            .iter()
            .map(|destination| ADestinationToMap {
                destination,
                the_longest_name_it_will_write: Some(b"placeholder0001.png".to_vec()),
            })
            .collect();
        compose_the_map(&held, BLEND_SITS_IN, JOB_FOLDER, version)
    }

    fn composed(reading: &TheOutputsHalf, version: TheVersionInHand) -> TheInventory {
        match compose_the_inventory(reading, &mapped(reading, version)) {
            WhatTheInventoryIs::It(inventory) => inventory,
            other => panic!("the inventory was withheld: {other:?}"),
        }
    }

    #[test]
    fn i2s_recipe_differs_across_the_version_boundary_exactly_as_it_was_observed() {
        // Observed on disk on 2026-08-17: 4.5.9 wrote `beauty0010.png` and 5.2.0
        // wrote `fixture_52beauty0010.png`.
        assert_eq!(
            the_name_a_node_writes(None, Some(&text("beauty")), Some(10), Some(&text(".png"))),
            b"beauty0010.png".to_vec()
        );
        assert_eq!(
            the_name_a_node_writes(
                Some(&text("fixture_52")),
                Some(&text("beauty")),
                Some(10),
                Some(&text(".png"))
            ),
            b"fixture_52beauty0010.png".to_vec()
        );
    }

    #[test]
    fn a_run_of_hash_marks_takes_the_frame_as_blender_wrote_it_in_all_four() {
        // **The desk test of 2026-10-04 in 4.5.12, 5.0.1, 5.1.2 and 5.2.2**
        // (`Audits/Build/Step 10c/The Creator's second notes, piece 1 — what
        // landed.md`, section 1), every name as Blender wrote it, the four
        // versions alike.
        let png = Some(text(".png"));
        let name = |base: Option<&str>, output: &str, frame: i64| {
            String::from_utf8(the_name_a_node_writes(
                base.map(text).as_ref(),
                Some(&text(output)),
                Some(frame),
                png.as_ref(),
            ))
            .unwrap()
        };
        // The Creator's own shape: `### - …` wrote `001 - …`, nothing at the end.
        assert_eq!(name(None, "### - Box", 1), "001 - Box.png");
        assert_eq!(name(None, "name####", 3), "name0003.png");
        // A number wider than its run is written whole.
        assert_eq!(name(None, "### - name", 1000), "1000 - name.png");
        // The last run takes it; an earlier run stays as written, also when
        // the two runs are split between the base file name and the output.
        assert_eq!(name(None, "##_##", 1), "##_01.png");
        assert_eq!(name(Some("a##_"), "b##", 2), "a##_b02.png");
        // A run in the base file name counts like one in the output's name.
        assert_eq!(name(Some("shot_###_"), "beauty", 1), "shot_001_beauty.png");
        // A run in a folder part does not count, and four digits go on the end.
        assert_eq!(name(None, "d##/x", 1), "d##/x0001.png");
        assert_eq!(name(None, "d##/x###", 1), "d##/x001.png");
        // No run at all: four digits on the end, as before, wider written whole.
        assert_eq!(name(None, "beauty", 1), "beauty0001.png");
        assert_eq!(name(None, "beauty", 1000), "beauty1000.png");
    }

    #[test]
    fn an_ending_switched_off_writes_no_ending_rather_than_an_empty_one() {
        // I3's *unless the artist switched it off*, and §14 item 20's shape.
        assert_eq!(
            the_name_a_node_writes(None, Some(&text("beauty")), Some(10), None),
            b"beauty0010".to_vec()
        );
    }

    #[test]
    fn i4_is_stated_for_the_jobs_own_scene_and_the_strip_observation_is_not_widened() {
        // The one observation of a scene reached through a strip: 4.5.9 wrote
        // `sec0001.png` with a number and 5.2.0 wrote `fixture_52sec.png` with
        // none. It is applied to that shape and to no other.
        for version in [TheVersionInHand::of(4, 5), TheVersionInHand::of(5, 2)] {
            assert!(a_node_stamps_the_frame_number(
                &WhichScene::TheJobsOwnScene,
                version
            ));
        }
        let strip = WhichScene::ReachedThroughASequencerStrip {
            scene: text("Second"),
            strip: text("to_second"),
        };
        assert!(a_node_stamps_the_frame_number(&strip, TheVersionInHand::of(4, 5)));
        assert!(!a_node_stamps_the_frame_number(&strip, TheVersionInHand::of(5, 2)));
    }

    #[test]
    fn the_count_is_worked_out_frame_by_frame_and_never_by_multiplying() {
        // I12 and I8: a view layer that does not render at a frame writes
        // nothing at that frame, so a node input fed only by it owes no file. A
        // multiplication would promise files that are never coming.
        let mut sometimes = a_node_destination("Sometimes", "beauty", "/home/a/out", None);
        sometimes.the_view_layers_that_feed_it = vec![text("VL_Sometimes")];
        let always = a_node_destination("Always", "beauty", "/home/a/out2", None);
        let reading = a_reading(
            vec![always, sometimes],
            vec![
                a_frame(10, &["VL_Always", "VL_Sometimes"], None),
                a_frame(22, &["VL_Always"], None),
                a_frame(50, &["VL_Always"], None),
            ],
            TheVersionInHand::of(4, 5),
        );
        let inventory = composed(&reading, TheVersionInHand::of(4, 5));
        // Three frames times two destinations would be six.
        assert_eq!(inventory.the_count(), 4);
        assert_eq!(inventory.the_figure_for(0), 3);
        assert_eq!(inventory.the_figure_for(1), 1);
    }

    #[test]
    fn a_frame_with_no_active_view_layer_owes_nothing() {
        // I10: a frame with no view layer active on it renders nothing and
        // writes nothing, and a job can carry a stretch of those mid-range.
        let reading = a_reading(
            vec![a_node_destination("Beauty", "beauty", "/home/a/out", None)],
            vec![
                a_frame(1, &["VL"], None),
                a_frame(2, &[], None),
                a_frame(3, &["VL"], None),
            ],
            TheVersionInHand::of(4, 5),
        );
        assert_eq!(composed(&reading, TheVersionInHand::of(4, 5)).the_count(), 2);
    }

    #[test]
    fn a_collision_is_two_writes_and_one_file_and_is_counted_once() {
        // I13 and I16. Two nodes with the same output name in the same folder
        // predict one path.
        let one = a_node_destination("Beauty", "shared", "/home/a/out", None);
        let other = a_node_destination("Other", "shared", "/home/a/out", None);
        let reading = a_reading(
            vec![one, other],
            vec![a_frame(1, &["VL"], None)],
            TheVersionInHand::of(4, 5),
        );
        let inventory = composed(&reading, TheVersionInHand::of(4, 5));
        assert_eq!(inventory.the_count(), 1);
        // I14a: the per-destination figures sum above the count.
        assert_eq!(inventory.the_figure_for(0) + inventory.the_figure_for(1), 2);
        // I15's collision record: node to node, both members named.
        assert_eq!(inventory.collision_record.len(), 1);
        assert_eq!(inventory.collision_record[0].members.len(), 2);
        assert_eq!(
            inventory.collision_record[0].members[0].origin,
            WhereItComesFrom::AFileOutputNodeNamed(text("Beauty"))
        );
    }

    #[test]
    fn the_counts_wider_test_takes_in_a_case_the_record_does_not() {
        // I15: a node destination landing on the regular output path's own
        // predicted name. The count is right — one file — and no warning is
        // raised, because the record is node to node.
        let regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/beauty",
            WhereItComesFrom::TheScenesOutputPath,
        );
        let node = a_node_destination("Beauty", "beauty", "/home/a/out", None);
        let reading = a_reading(
            vec![regular, node],
            vec![a_frame(10, &["VL"], Some("/home/a/out/beauty0010.png"))],
            TheVersionInHand::of(4, 5),
        );
        let inventory = composed(&reading, TheVersionInHand::of(4, 5));
        assert_eq!(inventory.the_count(), 1, "{:#?}", inventory.entries);
        assert!(
            inventory.collision_record.is_empty(),
            "the record is node to node and this pair is not"
        );
    }

    #[test]
    fn the_save_output_switch_takes_the_scenes_own_frames_out_of_the_count_and_leaves_a_node_in() {
        // D12, and I6's *both additions take files out of the count*.
        let regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/f_",
            WhereItComesFrom::TheScenesOutputPath,
        );
        let node = a_node_destination("Beauty", "beauty", "/home/a/passes", None);
        let mut off: GatingConditions = plain_gating();
        off.save_output_switch = Some(false);
        let mut frame = a_frame(1, &["VL"], Some("/home/a/out/f_0001.png"));
        frame.gating = off;
        let reading = a_reading(vec![regular, node], vec![frame], TheVersionInHand::of(5, 2));
        let inventory = composed(&reading, TheVersionInHand::of(5, 2));
        assert_eq!(inventory.the_count(), 1);
        assert_eq!(inventory.the_figure_for(0), 0);
        assert_eq!(inventory.the_figure_for(1), 1);
    }

    #[test]
    fn a_scene_strip_stops_a_nodes_files_without_stopping_the_scenes_own() {
        // §14 item 13, R4 item 1's second condition.
        let node = a_node_destination("Beauty", "beauty", "/home/a/out", None);
        let mut with_a_strip = plain_gating();
        with_a_strip.the_render_goes_out_through_a_sequencer_holding_a_scene_strip = true;
        let mut frame = a_frame(1, &["VL"], None);
        frame.gating = with_a_strip;
        let reading = a_reading(vec![node], vec![frame], TheVersionInHand::of(5, 2));
        assert_eq!(composed(&reading, TheVersionInHand::of(5, 2)).the_count(), 0);
    }

    #[test]
    fn a_two_view_scene_withholds_the_inventory_with_the_reason_named() {
        // I9a: nothing then reads absence as evidence — no frame is called
        // missing, no carry-on re-renders anything, and the count is withheld
        // rather than being wrong.
        let mut reading = a_reading(
            vec![a_node_destination("Beauty", "beauty", "/home/a/out", None)],
            vec![a_frame(1, &["VL"], None)],
            TheVersionInHand::of(5, 2),
        );
        reading.views = HowManyViewsTheSceneWrites::MoreThanOneWrittenSeparately { how_many: 2 };
        assert_eq!(
            compose_the_inventory(&reading, &mapped(&reading, TheVersionInHand::of(5, 2))),
            WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                WhyTheFrameSetIsNotPredictable::TheSceneWritesMoreThanOneViewSeparately
            )
        );
        // And a job told to put both views in one file writes the ordinary name,
        // so it is not withheld.
        reading.views = HowManyViewsTheSceneWrites::MoreThanOneGatheredIntoOneFile;
        assert!(matches!(
            compose_the_inventory(&reading, &mapped(&reading, TheVersionInHand::of(5, 2))),
            WhatTheInventoryIs::It(_)
        ));
    }

    /// **I2's *What an unresolved name does*** (added 2026-10-18): a node
    /// output whose name holds an expression Smoozy does not replace owes no
    /// entry and holds no frame back — the frame counts on the files it can
    /// name; where it can name none, the frame set is **not predictable, with
    /// the reason named**, never frames that write nothing. **The replaced
    /// text is what the recipe joins**, and a reading with none is taken raw.
    #[test]
    fn an_unresolved_node_name_owes_no_entry_and_a_frame_owing_nothing_nameable_is_withheld() {
        let version = TheVersionInHand::of(5, 3);
        let read = |base: &str| {
            let mut one = a_node_destination(base, "Image", "/home/a/out", Some(base));
            one.put_in_the_replaced_text(Some(b"standin"), TheVersionInHand::of(5, 2));
            one
        };
        let (fps, blend_name) = (read("{fps} - a"), read("{blend_name} - b"));
        let names = |inventory: &TheInventory| -> Vec<String> {
            inventory.entries.iter().map(|entry| String::from_utf8_lossy(&entry.predicted_path).into_owned()).collect()
        };

        let both = a_reading(vec![fps.clone(), blend_name.clone()], vec![a_frame(1, &["VL"], None)], version);
        let inventory = composed(&both, version);
        assert_eq!(names(&inventory), [format!("{JOB_FOLDER}/standin - bImage0001.png")]);
        assert_eq!(inventory.the_count(), 1, "the unresolved output adds nothing to the count");

        let alone = a_reading(vec![fps], vec![a_frame(1, &["VL"], None)], version);
        assert_eq!(
            compose_the_inventory(&alone, &mapped(&alone, version)),
            WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                WhyTheFrameSetIsNotPredictable::AFramesFilesAreNamedByAnExpressionThatIsNotReplaced
            )
        );

        // **A frame its layer does not render at owes nothing**, unresolved or
        // not, and is not withheld for it (I8 before I2).
        let mut elsewhere = read("{fps} - c");
        elsewhere.the_view_layers_that_feed_it = vec![text("Other")];
        let quiet = a_reading(vec![elsewhere, blend_name.clone()], vec![a_frame(1, &["VL"], None)], version);
        assert_eq!(composed(&quiet, version).the_count(), 1);

        // **A reading taken before the replacement existed is taken raw.**
        let raw = a_node_destination("B", "Image", "/home/a/out", Some("{blend_name} - b"));
        let older = a_reading(vec![raw], vec![a_frame(1, &["VL"], None)], version);
        assert_eq!(names(&composed(&older, version)), [format!("{JOB_FOLDER}/{{blend_name}} - bImage0001.png")]);
    }

    #[test]
    fn a_node_in_a_sequencer_reached_scene_withholds_the_inventory_from_five_zero() {
        // Without this the recipe built a name with no frame number in it, so
        // every frame of that destination predicted the same path: five entries
        // at one path over five frames, breaking I14's *one entry per expected
        // file*. The completeness statement matches each entry against the walk
        // separately and nothing consumes a walk entry, so one file on disk
        // answered all five and Smoozy reported a job complete that left one
        // file where it promised a hundred.
        let strip = WhichScene::ReachedThroughASequencerStrip {
            scene: text("Second"),
            strip: text("to_second"),
        };
        let mut node = a_node_destination("Beauty", "beauty", "/home/a/out", None);
        node.scene = strip.clone();
        let mut reading = a_reading(
            vec![node],
            (1..=5).map(|frame| a_frame(frame, &["VL"], None)).collect(),
            TheVersionInHand::of(5, 2),
        );
        reading.scenes_reached_through_a_sequencer_strip = vec![crate::reading::SceneReachedThroughAStrip {
            scene: text("Second"),
            strip: text("to_second"),
            gating: plain_gating(),
        }];
        assert_eq!(
            compose_the_inventory(&reading, &mapped(&reading, TheVersionInHand::of(5, 2))),
            WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                WhyTheFrameSetIsNotPredictable::AFileOutputNodeInASequencerReachedSceneFromFiveZero
            )
        );

        // **4.5 is unchanged and keeps predicting**: there the node stamps the
        // number, which is what I4 states for the job's own scene as well.
        reading.taken_by = TheVersionInHand::of(4, 5);
        let inventory = composed(&reading, TheVersionInHand::of(4, 5));
        assert_eq!(inventory.the_count(), 5);
        let mut paths: Vec<&Vec<u8>> = inventory
            .entries
            .iter()
            .map(|entry| &entry.predicted_path)
            .collect();
        paths.sort();
        paths.dedup();
        assert_eq!(paths.len(), 5, "five frames, five distinct paths");
    }

    #[test]
    fn a_destination_the_map_could_not_bring_under_withholds_the_inventory() {
        // It used to be dropped without trace, and where every one of the
        // render's own destinations went that way the count came to zero —
        // which H1's second condition read as *nothing was aimed anywhere*, the
        // completeness statement read as nought expected and nought present so
        // a full range read as complete, and C14 read as every frame already
        // there, handing a re-run back one frame and calling the job done.
        let climbs = a_node_destination("Beauty", "../../climb", "/home/a/out", None);
        let reading = a_reading(
            vec![climbs],
            vec![a_frame(1, &["VL"], None)],
            TheVersionInHand::of(4, 5),
        );
        let map = mapped(&reading, TheVersionInHand::of(4, 5));
        assert!(map.a_destination_is_not_mappable());
        assert_eq!(
            compose_the_inventory(&reading, &map),
            WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                WhyTheFrameSetIsNotPredictable::ADestinationIsNotMappable
            )
        );
    }

    #[test]
    fn an_unanswered_regular_output_name_withholds_and_the_switch_being_off_does_not() {
        // The two used to leave the same absence. An unanswered name means the
        // name is not known; D12's switch off means this destination writes
        // nothing at this frame, and that one is right.
        let regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/f_",
            WhereItComesFrom::TheScenesOutputPath,
        );
        let unanswered = a_reading(
            vec![regular.clone()],
            vec![a_frame(1, &["VL"], None)],
            TheVersionInHand::of(5, 2),
        );
        assert_eq!(
            compose_the_inventory(&unanswered, &mapped(&unanswered, TheVersionInHand::of(5, 2))),
            WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                WhyTheFrameSetIsNotPredictable::TheRegularOutputPathsNameWasNotAnswered
            )
        );

        // The switch off, with a node still writing: the inventory is composed
        // and the regular output path simply owes nothing.
        let node = a_node_destination("Beauty", "beauty", "/home/a/passes", None);
        let mut off = plain_gating();
        off.save_output_switch = Some(false);
        let mut frame = a_frame(1, &["VL"], None);
        frame.gating = off;
        let switched_off = a_reading(vec![regular, node], vec![frame], TheVersionInHand::of(5, 2));
        let inventory = composed(&switched_off, TheVersionInHand::of(5, 2));
        assert_eq!(inventory.the_count(), 1);
        assert_eq!(inventory.the_figure_for(0), 0);
    }

    #[test]
    fn a_node_group_used_twice_is_in_the_collision_record() {
        // I15: *"a node group used twice in one tree is the same case as two
        // nodes."* A file output node inside a group has one name, so both
        // destinations carried it, the record's test said one node, and the
        // pair was dropped — no warning, H1's third condition not fired, one
        // picture silently overwritten, and nothing else catching it because
        // the count's wider test holds nothing and warns nobody (I18).
        let one = a_node_destination("Inner", "beauty", "/home/a/out", None);
        let twice = a_node_destination("Inner", "beauty", "/home/a/out", None);
        let reading = a_reading(
            vec![one, twice],
            vec![a_frame(1, &["VL"], None)],
            TheVersionInHand::of(4, 5),
        );
        let inventory = composed(&reading, TheVersionInHand::of(4, 5));
        assert_eq!(inventory.the_count(), 1);
        assert_eq!(inventory.collision_record.len(), 1);
        assert_eq!(inventory.collision_record[0].members.len(), 2);

        // And the record's real boundary is kept: a node against the regular
        // output path is still not in it.
        let regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/beauty",
            WhereItComesFrom::TheScenesOutputPath,
        );
        let node = a_node_destination("Beauty", "beauty", "/home/a/out", None);
        let not_a_pair = a_reading(
            vec![regular, node],
            vec![a_frame(10, &["VL"], Some("/home/a/out/beauty0010.png"))],
            TheVersionInHand::of(4, 5),
        );
        assert!(composed(&not_a_pair, TheVersionInHand::of(4, 5))
            .collision_record
            .is_empty());
    }

    #[test]
    fn an_entry_whose_format_is_not_recorded_stays_in_the_split() {
        // I14a: *"every entry falls in exactly one group and the groups sum to
        // the count."* Two silent skips used to take such an entry out of the
        // split altogether, so the page showed groups adding to less than the
        // count with nothing said — and a reader could not tell it from I14a's
        // other figure, which may legitimately sum above the count.
        let with_a_format = a_node_destination("Beauty", "beauty", "/home/a/out", None);
        let mut without = a_node_destination("Plain", "plain", "/home/a/out", None);
        without.format = None;
        let reading = a_reading(
            vec![with_a_format, without],
            vec![a_frame(1, &["VL"], None)],
            TheVersionInHand::of(4, 5),
        );
        let inventory = composed(&reading, TheVersionInHand::of(4, 5));
        let split = inventory.the_split_by_format(&reading.destinations);
        assert_eq!(
            split.iter().map(|(_, how_many)| how_many).sum::<usize>(),
            inventory.the_count()
        );
        assert!(split.iter().any(|(format, _)| format.is_none()));
    }

    #[test]
    fn a_video_destination_is_one_entry_marked_may_grow_and_carries_no_per_frame_entries() {
        // I19, I20 and I22.
        let mut regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/stereo_",
            WhereItComesFrom::TheScenesOutputPath,
        );
        regular.format = Some(Format {
            as_the_file_records_it: text("FFMPEG"),
            the_ending_it_writes: Some(text(".mp4")),
            is_video_shaped: true,
        });
        let reading = a_reading(
            vec![regular],
            vec![
                a_frame(7, &["VL"], Some("/home/a/out/stereo_0007-0009.mp4")),
                a_frame(8, &["VL"], Some("/home/a/out/stereo_0007-0009.mp4")),
                a_frame(9, &["VL"], Some("/home/a/out/stereo_0007-0009.mp4")),
            ],
            TheVersionInHand::of(5, 2),
        );
        let inventory = composed(&reading, TheVersionInHand::of(5, 2));
        assert_eq!(inventory.the_count(), 1);
        assert!(inventory.entries[0].may_grow);
        assert_eq!(inventory.entries[0].frame, None);
        assert!(inventory.a_destination_is_video_shaped);
    }

    #[test]
    fn an_added_write_place_is_a_folder_in_the_inventory_and_in_no_count() {
        // I14 and D21e: no count ever vouches for a bake or a cache.
        let node = a_node_destination("Beauty", "beauty", "/home/a/out", None);
        let cache = Destination {
            kind: DestinationKind::AnAddedWritePlace(AnAddedWritePlace::ThePointCache),
            recorded_location: None,
            format: None,
            origin: WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::ThePointCache),
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };
        let reading = a_reading(
            vec![node, cache],
            vec![a_frame(1, &["VL"], None)],
            TheVersionInHand::of(5, 2),
        );
        let inventory = composed(&reading, TheVersionInHand::of(5, 2));
        assert_eq!(inventory.the_count(), 1);
        assert_eq!(inventory.added_write_place_folders.len(), 1);
        assert_eq!(
            inventory.added_write_place_folders[0].place,
            AnAddedWritePlace::ThePointCache
        );
    }

    #[test]
    fn the_split_by_format_sums_to_the_count_and_a_collision_falls_in_one_group() {
        // I14a: every entry falls in exactly one group and the groups sum to the
        // count.
        let one = a_node_destination("Beauty", "shared", "/home/a/out", None);
        let mut other = a_node_destination("Other", "shared", "/home/a/out", None);
        other.format = Some(Format {
            as_the_file_records_it: text("JPEG"),
            the_ending_it_writes: Some(text(".png")),
            is_video_shaped: false,
        });
        let third = a_node_destination("Third", "alone", "/home/a/out", None);
        let reading = a_reading(
            vec![one, other, third],
            vec![a_frame(1, &["VL"], None)],
            TheVersionInHand::of(4, 5),
        );
        let inventory = composed(&reading, TheVersionInHand::of(4, 5));
        let split = inventory.the_split_by_format(&reading.destinations);
        assert_eq!(
            split.iter().map(|(_, how_many)| how_many).sum::<usize>(),
            inventory.the_count()
        );
        // The colliding entry is grouped under the first destination's format.
        assert_eq!(split, vec![(Some(b"PNG".to_vec()), 2)]);
    }

    /// **I8's second paragraph: from 5.3 a node owes only at the frames its own
    /// effect is on**, and a node of another effect by the same name is not held
    /// by that.
    #[test]
    fn a_node_in_an_effect_off_at_a_frame_owes_nothing_at_that_frame() {
        let version = TheVersionInHand::of(5, 3);
        let in_the_effect = |effect: &str, node: &str| {
            let mut one = a_node_destination(node, "Image", effect, Some("x_"));
            let DestinationKind::AnOutputOfAFileOutputNode { the_effect, .. } = &mut one.kind else {
                panic!("a node's destination");
            };
            *the_effect = Some(text(effect));
            one
        };
        let at = |frame: i64, on: &[&str]| {
            let mut one = a_frame(frame, &["ViewLayer"], None);
            one.gating.the_effects_on_for_render = Some(on.iter().map(|name| text(name)).collect());
            one
        };
        let reading = a_reading(
            vec![in_the_effect("One", "File Output"), in_the_effect("Two", "File Output")],
            vec![at(1, &["One", "Two"]), at(2, &["One"]), at(3, &[])],
            version,
        );
        assert_eq!(composed(&reading, version).the_count(), 3);
        // **A reading with no stack is held by nothing here**, which is 4.5 to 5.2.
        let mut older = a_node_destination("File Output", "Image", "//passes/", Some("x_"));
        older.the_view_layers_that_feed_it = Vec::new();
        let reading = a_reading(vec![older], vec![a_frame(1, &["ViewLayer"], None)], TheVersionInHand::of(5, 2));
        assert_eq!(composed(&reading, TheVersionInHand::of(5, 2)).the_count(), 1);
    }
}

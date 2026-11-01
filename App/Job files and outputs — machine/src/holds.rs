//! §9 — the three holds.
//!
//! **H3: this document names the three conditions; `Rendering` owns the
//! mechanism.** Its Row 23 quotes the spec saying *"This module carries that out
//! and decides none of it"*, and **the release of a held job is `Rented-server
//! runtime`'s machine-held answer-release path**, which `Rendering`'s Row 4
//! already needs for the first-frame check. **Nothing here invents a second
//! waiting mechanism** — there is no hold record, no answer, no release and no
//! waiting anywhere in this file, and there must not be.
//!
//! **H2: all three are read off the recorded reading and none needs a second
//! evaluation.** That is why they are here and not in three places.
//!
//! **H7: what the answer does is release or stop, and neither writes anything
//! this document owns beyond the attempt record.** `Rendering` and
//! `Rented-server runtime` own the release; `Queue & History` owns the entry.

use serde::{Deserialize, Serialize};

use crate::inventory::{TheInventory, WhatTheInventoryIs};
use crate::main_output::{
    the_beauty_reaches_more_than_one_destination, the_places_the_beauty_is_written,
    the_render_reaches_frames_and_a_video, ThePlacesTheBeautyIsWritten,
};
use crate::mapping::TheMap;
use crate::reading::{DestinationKind, TheOutputsHalf, WhichScene};

/// **H1: three conditions hold a job before its render starts, and the list is
/// closed.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatHoldsTheJob {
    /// **A destination Smoozy cannot bring under the job's folder**, established
    /// by the map (§6.4) — a length that will not fit, or a 4.5 output name that
    /// climbs out of its node's folder.
    ///
    /// **It reads the whole destination set** of R4 item 2, the added write
    /// places included, *"because the spec grants this hold unconditionally for
    /// any path Smoozy cannot bring under the job's folder"*.
    ADestinationCannotBeBroughtUnderTheJobsFolder,
    /// **The job will write nothing at all**, established by the
    /// expected-output inventory holding no entry — **the count at zero** — and
    /// never the added write places, which are in no count.
    ///
    /// **H1a: the condition consumes the count rather than the destination
    /// set.** As first written it read R4 item 2's destination set alone, and a
    /// job whose every frame has no active view layer has destinations that are
    /// not empty — *"so the condition never fired for it, while I10 counted its
    /// every frame as nothing and C14 called the same job complete: the reading
    /// already produced the fact and no condition consumed it."*
    TheJobWillWriteNothingAtAll,
    /// **Two destinations aimed at one name in one place**, established by the
    /// **collision record** of §7.4 — *"which is the node-to-node set and not the
    /// count's wider test"*.
    TwoDestinationsAimedAtOneNameInOnePlace,
}

/// The three, as one list. **The list is closed** (H1), and D21g says outright
/// that it *"is not widened"* by an added write place that produced nothing.
pub const THE_THREE_CONDITIONS: [WhatHoldsTheJob; 3] = [
    WhatHoldsTheJob::ADestinationCannotBeBroughtUnderTheJobsFolder,
    WhatHoldsTheJob::TheJobWillWriteNothingAtAll,
    WhatHoldsTheJob::TwoDestinationsAimedAtOneNameInOnePlace,
];

/// **H1a's second half: the reason named inside the one statement, read off the
/// reading's standing fields and never a new one.**
///
/// *"Item 2's destination set says whether nothing was aimed anywhere, item 5's
/// per-frame set says whether nothing renders at any frame, and the words are
/// Design's."* **A job that renders nothing at all is not a seventh statement**
/// — it is the same one as a job that writes nothing, with a different reason
/// named in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhyItWillWriteNothing {
    /// **The render's own output destinations are empty across every frame** —
    /// in every version a tree whose nodes are all muted, unconnected, or reached
    /// by a compositing switch that is off, and from **5.2** the Save Output
    /// switch off with no node writing anything.
    NothingWasAimedAnywhere,
    /// **Destinations standing while no frame the job covers has an active view
    /// layer**, every frame owing nothing (I6, I8, I10).
    NoFrameHasAnActiveViewLayer,
}

/// **`Rented-server runtime` U2: the hold record carries *the reason, from the
/// closed set the placing document declares*** — and H1 is that set, so this
/// is where each of its three is spelled for the record. **One spelling, read
/// back by the same table** ([`the_conditions_a_hold_reason_names`]), so the
/// machine that writes the reason and the desktop that draws the entry from
/// it can never spell a condition two ways. *(The 38's piece 10, item 26:
/// until 2026-10-03 the render worker wrote the conditions as Rust's own
/// debug print of the list, which no reader could name.)*
///
/// **The words are a record's, not the user's**: the entry says each
/// condition in Design's words (`Queue & History` H1, *"shows the stated
/// reason without interpreting it"* — the reason is told apart, never
/// reworded into another fact).
const THE_REASONS_AS_THE_HOLD_RECORD_CARRIES_THEM: [(WhatHoldsTheJob, &str); 3] = [
    (
        WhatHoldsTheJob::ADestinationCannotBeBroughtUnderTheJobsFolder,
        "a destination cannot be brought under the job's folder",
    ),
    (
        WhatHoldsTheJob::TheJobWillWriteNothingAtAll,
        "the job will write nothing at all",
    ),
    (
        WhatHoldsTheJob::TwoDestinationsAimedAtOneNameInOnePlace,
        "two destinations aimed at one name in one place",
    ),
];

/// **Between two conditions in one reason.** A hold can stand on more than
/// one of the three at once, and the record carries one reason.
const BETWEEN_TWO_CONDITIONS: &str = "; ";

impl WhatHoldsTheJob {
    /// **This condition as the hold record carries it** — see
    /// [`THE_REASONS_AS_THE_HOLD_RECORD_CARRIES_THEM`].
    pub fn as_the_hold_record_carries_it(self) -> &'static str {
        THE_REASONS_AS_THE_HOLD_RECORD_CARRIES_THEM
            .iter()
            .find(|(condition, _)| *condition == self)
            .map(|(_, said)| *said)
            .expect("each of the three has its spelling")
    }
}

/// **The reason a hold placed at a render's start carries** (`Rented-server
/// runtime` U2 and `Rented-server runtime` U6): every condition that held
/// the job, in H1's own order.
pub fn the_hold_reason_for(conditions: &[WhatHoldsTheJob]) -> String {
    THE_THREE_CONDITIONS
        .iter()
        .filter(|condition| conditions.contains(condition))
        .map(|condition| condition.as_the_hold_record_carries_it())
        .collect::<Vec<_>>()
        .join(BETWEEN_TWO_CONDITIONS)
}

/// **The conditions a hold's reason names, read back** — or nothing where the
/// reason is not one this set spells: a first-frame check's, or one from a
/// program older than 2026-10-03. **A reason with any part this set does not
/// spell names none**, so nothing is ever drawn from half a reason.
pub fn the_conditions_a_hold_reason_names(reason: &str) -> Option<Vec<WhatHoldsTheJob>> {
    let mut named = Vec::new();
    for part in reason.split(BETWEEN_TWO_CONDITIONS) {
        let (condition, _) = THE_REASONS_AS_THE_HOLD_RECORD_CARRIES_THEM
            .iter()
            .find(|(_, said)| *said == part)?;
        if !named.contains(condition) {
            named.push(*condition);
        }
    }
    (!named.is_empty()).then_some(named)
}

/// **H5's two triggers read the same fact R7 records, and there is one of it.**
///
/// **R7 and H5 are the same fact read for two purposes** — which side composed
/// the map — so it has one home, [`crate::reading::WhichSideComposedTheMap`],
/// and this is a re-export rather than a second shape. **This file used to
/// define its own, spelling one variant `TheMachineAtRenderStart` against
/// R7's `TheRenderWorkerAtRenderStart`**, so a caller holding one had to
/// translate before asking the other, and a file importing both could not name
/// either. Removed 2026-09-05, out of a code round; **R9's discipline is that
/// the folders shown and the folders written into are the same object, and two
/// shapes to keep in step by hand is that fault in miniature.**
pub use crate::reading::WhichSideComposedTheMap;

impl WhatHoldsTheJob {
    /// **H5, performed.**
    ///
    /// - **The cannot-bring-under condition holds the job whichever side computed
    ///   the map**, because the spec states it unconditionally. *"Where the map
    ///   came from the manifest, the queue entry has carried it since submission
    ///   and the hold is where the user's answer is taken."*
    /// - **The writes-nothing and colliding-pair conditions hold the job only
    ///   where the map was computed on the machine**, because where the file was
    ///   read before submitting the user was already shown those two and answered
    ///   by submitting. The spec's own wording is *"Where that read finds…"*, and
    ///   *that read* is the one at job start.
    ///
    /// **H6: the user is not asked the same question twice**, which is the whole
    /// reason H5 is written as two triggers rather than one.
    pub fn it_fires_when_the_map_came_from(self, side: WhichSideComposedTheMap) -> bool {
        match self {
            WhatHoldsTheJob::ADestinationCannotBeBroughtUnderTheJobsFolder => true,
            WhatHoldsTheJob::TheJobWillWriteNothingAtAll
            | WhatHoldsTheJob::TwoDestinationsAimedAtOneNameInOnePlace => {
                side == WhichSideComposedTheMap::TheRenderWorkerAtRenderStart
            }
        }
    }
}

/// **H1 and H2, performed: what holds this job, read off the recorded reading,
/// the map and the inventory, and off nothing else.**
///
/// **H4: the hold is placed on the machine, after the file is loaded and before
/// the render begins.** It has to be: the answer may arrive from a phone with
/// nobody connected and Smoozy closed. **And from 5.2 it has to be before Blender
/// is asked to render at all** — with no live node at all that version refuses
/// the render outright before any frame is started, *"so a writes-nothing job left
/// to the run's own complaint would never reach a hold"*. **That is a reason for
/// where the hold sits and not a clause of the condition it holds on.**
pub fn what_holds_this_job(
    map: &TheMap,
    inventory: &WhatTheInventoryIs,
    side: WhichSideComposedTheMap,
) -> Vec<WhatHoldsTheJob> {
    let mut out = Vec::new();
    if map.a_destination_is_not_mappable() {
        out.push(WhatHoldsTheJob::ADestinationCannotBeBroughtUnderTheJobsFolder);
    }
    // **I9a: where the inventory was not composed, no frame is called missing
    // and nothing reads absence as evidence** — so a withheld inventory holds
    // nothing on a count it does not have.
    if let WhatTheInventoryIs::It(inventory) = inventory {
        if inventory.the_count() == 0 {
            out.push(WhatHoldsTheJob::TheJobWillWriteNothingAtAll);
        }
        if !inventory.collision_record.is_empty() {
            out.push(WhatHoldsTheJob::TwoDestinationsAimedAtOneNameInOnePlace);
        }
    }
    out.retain(|condition| condition.it_fires_when_the_map_came_from(side));
    out
}

/// **H1a's reason, read off the reading's standing fields** — **the per-frame
/// set the count was composed from, so with the job's pins applied** (R4 item
/// 5; 2026-10-04, piece 1b). [`crate::inventory::compose_the_inventory`] asks
/// this over that set and carries the answer with the count.
pub fn why_it_will_write_nothing(reading: &TheOutputsHalf) -> WhyItWillWriteNothing {
    // **Item 2's destination set says whether nothing was aimed anywhere.**
    if !reading
        .destinations
        .iter()
        .any(crate::reading::Destination::is_one_the_render_asked_for)
    {
        return WhyItWillWriteNothing::NothingWasAimedAnywhere;
    }
    // **Item 5's per-frame set says whether nothing renders at any frame.**
    if reading
        .per_frame
        .iter()
        .all(|frame| frame.view_layers_that_render.is_empty())
    {
        return WhyItWillWriteNothing::NoFrameHasAnActiveViewLayer;
    }
    // Destinations stood and frames rendered, and the count still came to zero —
    // the gating took every file out. **That is the first reason and not a
    // third**: nothing the render was asked for is aimed anywhere that will
    // write.
    WhyItWillWriteNothing::NothingWasAimedAnywhere
}

/// **H8: the six things the machine produces for a person who is not there.**
///
/// *"Where the file was not read before submitting, the machine produces the six
/// things and they reach a queue entry that may be read much later, from a
/// different computer, with nobody connected at the moment they were worked
/// out."* **All six are fields of the recorded reading**, so nothing here is a
/// second evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheSixThings {
    /// **The folders it is writing into**, as they appear under the job's folder
    /// (P11): the folders of the files counted, and never an added write place's.
    pub the_folders_it_is_writing_into: Vec<Vec<u8>>,
    /// **That it is writing nothing where that is so**, with H1a's reason named
    /// inside the one statement.
    pub it_is_writing_nothing: Option<WhyItWillWriteNothing>,
    /// **That two of its destinations collide where they do** — the collision
    /// record, each member carrying where it comes from (I16a).
    pub the_colliding_pairs: Vec<crate::inventory::ACollidingPair>,
    /// **That the same picture is being written to more than one place**, read
    /// off the beauty walk and off nothing else (B9). **Only places writing
    /// images count**: a video at the output path is not one (B9's dated
    /// paragraph of 2026-10-12).
    pub the_same_picture_reaches_more_than_one_place: bool,
    /// **Which places those are**, so the line names what the walk found —
    /// the runner's ruling of 2026-10-12, *The duplicate-picture warning names
    /// what it found*. Empty where the picture reaches one place or none.
    #[serde(default)]
    pub where_the_same_picture_is_written: ThePlacesTheBeautyIsWritten,
    /// **That the render reaches frames and a video**: the beauty reaches a
    /// File Output node and the output path writes a video, two outputs told
    /// as what they are (B9's dated paragraph of 2026-10-12; the Creator's
    /// eighth notes, note 4).
    #[serde(default)]
    pub the_render_reaches_frames_and_a_video: bool,
    /// **That its compositor is shared with another scene**, and which scene
    /// (R4 item 4). Arises from 5.0 on and never in 4.5.
    pub the_tree_is_shared_with: Option<crate::reading::ArtistsText>,
    /// **That its output is a single video file** (§7.5).
    pub its_output_is_a_single_video_file: bool,
}

/// **H8, composed off the recorded reading, the map and the inventory.**
///
/// **H10: the queue entry is `Queue & History`'s and what it says here is this
/// document's**, which is the spec's own division and is kept — so this composes
/// the six facts and no wording.
pub fn the_six_things(
    reading: &TheOutputsHalf,
    map: &TheMap,
    inventory: &WhatTheInventoryIs,
    job_output_folder: &str,
) -> TheSixThings {
    let (colliding, writes_nothing, video) = match inventory {
        WhatTheInventoryIs::It(it) => (
            it.collision_record.clone(),
            // **H1a's reason with the job's choices**: the one the count was
            // composed with; an inventory recorded before it carried one
            // reads the recorded reading, as before (2026-10-04, piece 1b).
            (it.the_count() == 0)
                .then(|| it.why_it_owes_nothing.unwrap_or_else(|| why_it_will_write_nothing(reading))),
            the_output_is_a_single_video_file(it),
        ),
        // **The inventory was withheld** (I9a), so there is no count to read a
        // writes-nothing statement off and no predicted name to collide.
        WhatTheInventoryIs::TheFrameSetIsNotPredictable(_) => (Vec::new(), None, false),
    };
    // **P11 and hole 4 of the Creator's second notes (2026-10-04): the folders
    // of the files counted.** Where the inventory stands, a destination the
    // render asked for that owes no file — a node fed only by a layer the job
    // switched off — names no folder; where it was withheld (I9a), the map's
    // own list stands.
    let the_folders_it_is_writing_into = match inventory {
        WhatTheInventoryIs::It(it) => map.the_folders_of(job_output_folder, |destination| {
            destination.is_one_the_render_asked_for()
                && reading
                    .destinations
                    .iter()
                    .position(|held| held == destination)
                    .is_some_and(|at| it.the_figure_for(at) > 0)
        }),
        WhatTheInventoryIs::TheFrameSetIsNotPredictable(_) => {
            map.the_folders_the_job_will_write_into(job_output_folder)
        }
    };
    // **I19: only the regular output path can write a video**, and this is the
    // field the inventory reads to say so.
    let the_output_path_writes_a_video = reading.destinations.iter().any(|destination| {
        destination.kind == DestinationKind::TheRegularOutputPath
            && destination.scene == WhichScene::TheJobsOwnScene
            && destination.format.as_ref().is_some_and(|format| format.is_video_shaped)
    });
    let walk = &reading.the_beauty_walk;
    let twice = the_beauty_reaches_more_than_one_destination(walk, the_output_path_writes_a_video);
    TheSixThings {
        the_folders_it_is_writing_into,
        it_is_writing_nothing: writes_nothing,
        the_colliding_pairs: colliding,
        the_same_picture_reaches_more_than_one_place: twice,
        where_the_same_picture_is_written: if twice {
            the_places_the_beauty_is_written(walk, the_output_path_writes_a_video)
        } else {
            ThePlacesTheBeautyIsWritten::default()
        },
        the_render_reaches_frames_and_a_video: the_render_reaches_frames_and_a_video(
            walk,
            the_output_path_writes_a_video,
        ),
        the_tree_is_shared_with: reading.the_tree_is_shared_with.clone(),
        its_output_is_a_single_video_file: video,
    }
}

/// **§7.5's shape, read for H8's sixth thing.**
///
/// **I19: a video destination can only ever be the regular output path, and a job
/// can carry a video destination and frame-shaped node destinations at once** —
/// so *its output is a single video file* is true only where the video entry is
/// the whole of what the job writes.
fn the_output_is_a_single_video_file(inventory: &TheInventory) -> bool {
    inventory.a_destination_is_video_shaped
        && inventory.entries.iter().all(|entry| entry.may_grow)
}

/// **H9: they reach the desktop by the two routes that already exist and by no
/// third one.**
///
/// *"The recorded reading is journalled as an event when it is written, so a
/// reader learns of it by reading forward from its own durable cursor (`State,
/// identity and recovery` §4); and it is in the job record for step 4 of `State,
/// identity and recovery` §7 to fill in."*
///
/// **Nothing polls and nothing is pushed**, and nothing in this crate composes a
/// third route.
pub const THE_SIX_THINGS_REACH_THE_DESKTOP_BY_THE_TWO_ROUTES_THAT_ALREADY_EXIST: bool = true;

/// **H3, held where it can be checked: no waiting mechanism is written here.**
///
/// The hold's record, its answer and its release are `Rented-server runtime`'s
/// machine-held answer-release path, and the act of holding is `Rendering`'s.
/// **This file has no hold record, no answer, no release and no wait.**
pub const NOTHING_HERE_IS_A_SECOND_WAITING_MECHANISM: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::compose_the_inventory;
    use crate::inventory::tests::{a_frame, a_node_destination, a_reading, mapped, JOB_FOLDER};
    use crate::main_output::{ADestinationTheBeautyReached, WhatTheWalkFound};
    use crate::mapping::{compose_the_map, ADestinationToMap};
    use crate::reading::tests::a_destination;
    use crate::reading::{
        ArtistsText, DestinationKind, TheVersionInHand, WhereItComesFrom,
    };

    const VERSION: TheVersionInHand = TheVersionInHand { major: 5, minor: 2 };

    fn text(of: &str) -> ArtistsText {
        ArtistsText::of(of.as_bytes().to_vec())
    }

    /// **H1a's reason is read with the job's choices** (R4 item 5; piece 1b,
    /// 2026-10-04, on the runner's ruling). A job whose every layer is pinned
    /// off renders nothing at any frame, and is told so, though the recorded
    /// reading — which stays the file's — has its layers rendering. **An
    /// inventory recorded before the reason travelled with the count has
    /// none, and reads the recorded reading as before**: no choices, today's
    /// answer.
    #[test]
    fn a_job_with_every_layer_pinned_off_is_told_no_frame_has_an_active_view_layer() {
        let mut node = a_node_destination("Beauty", "b_", "/home/a/out", None);
        node.the_view_layers_that_feed_it = vec![text("Box")];
        let the_files_own = a_reading(
            vec![node],
            vec![a_frame(1, &["Box"], None), a_frame(2, &["Box"], None)],
            VERSION,
        );
        let pins = [crate::reading::AViewLayerTheJobPins { layer: text("Box"), on: false }];
        let as_the_job_renders_it = crate::reading::TheOutputsHalf {
            per_frame: crate::reading::the_per_frame_set_with_the_jobs_pins(
                &the_files_own.per_frame,
                &pins,
            ),
            ..the_files_own.clone()
        };
        let map = mapped(&as_the_job_renders_it, VERSION);
        let inventory = compose_the_inventory(&as_the_job_renders_it, &map);
        let six = the_six_things(&the_files_own, &map, &inventory, JOB_FOLDER);
        assert_eq!(
            six.it_is_writing_nothing,
            Some(WhyItWillWriteNothing::NoFrameHasAnActiveViewLayer)
        );

        // **An inventory recorded before 2026-10-04** carries no reason.
        let mut recorded = serde_json::to_value(&inventory).expect("it writes");
        recorded["It"]
            .as_object_mut()
            .expect("the inventory")
            .remove("why_it_owes_nothing")
            .expect("the field is written");
        let old: WhatTheInventoryIs = serde_json::from_value(recorded).expect("an old record reads");
        let WhatTheInventoryIs::It(it) = &old else { panic!("composed") };
        assert_eq!(it.why_it_owes_nothing, None);
        assert_eq!(
            the_six_things(&the_files_own, &map, &old, JOB_FOLDER).it_is_writing_nothing,
            Some(WhyItWillWriteNothing::NothingWasAimedAnywhere)
        );
    }

    /// **`Rented-server runtime` U2's reason, written and read back by one
    /// table** — the 38's piece 10, item 26. Every combination of the three
    /// comes back as itself, in H1's order; a first-frame check's reason, a
    /// debug print and half a reason name nothing.
    #[test]
    fn a_hold_reason_spells_h1s_conditions_and_reads_back_as_them() {
        for mask in 1u8..8 {
            let conditions: Vec<WhatHoldsTheJob> = THE_THREE_CONDITIONS
                .iter()
                .enumerate()
                .filter(|(at, _)| mask & (1 << at) != 0)
                .map(|(_, condition)| *condition)
                .collect();
            let reason = the_hold_reason_for(&conditions);
            assert!(!reason.contains("Will"), "never a debug print: {reason}");
            assert_eq!(the_conditions_a_hold_reason_names(&reason), Some(conditions));
        }
        assert_eq!(the_conditions_a_hold_reason_names("the first-frame check"), None);
        assert_eq!(
            the_conditions_a_hold_reason_names("[TheJobWillWriteNothingAtAll]"),
            None
        );
        assert_eq!(
            the_conditions_a_hold_reason_names("the job will write nothing at all; something else"),
            None
        );
        assert_eq!(the_conditions_a_hold_reason_names(""), None);
    }

    #[test]
    fn the_list_of_conditions_is_closed_at_three() {
        // H1, and D21g: an added write place that produced nothing holds no job
        // and does not widen this list.
        assert_eq!(THE_THREE_CONDITIONS.len(), 3);
    }

    #[test]
    fn a_job_whose_every_frame_has_no_active_view_layer_is_held() {
        // **H1a's own case, and it is the fault that clause was written for**:
        // as first written the condition read the destination set alone, and
        // such a job has destinations that are not empty — so the condition
        // never fired for it, while I10 counted its every frame as nothing and
        // C14 called the same job complete.
        let reading = a_reading(
            vec![a_node_destination("Beauty", "beauty", "/home/a/out", None)],
            vec![a_frame(1, &[], None), a_frame(2, &[], None)],
            VERSION,
        );
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        assert!(what_holds_this_job(
            &map,
            &inventory,
            WhichSideComposedTheMap::TheRenderWorkerAtRenderStart
        )
        .contains(&WhatHoldsTheJob::TheJobWillWriteNothingAtAll));
        // And the reason named inside the one statement is the right one.
        assert_eq!(
            why_it_will_write_nothing(&reading),
            WhyItWillWriteNothing::NoFrameHasAnActiveViewLayer
        );
    }

    #[test]
    fn nothing_aimed_anywhere_is_the_other_reason_and_not_a_seventh_statement() {
        let reading = a_reading(Vec::new(), vec![a_frame(1, &["VL"], None)], VERSION);
        assert_eq!(
            why_it_will_write_nothing(&reading),
            WhyItWillWriteNothing::NothingWasAimedAnywhere
        );
    }

    #[test]
    fn a_cache_folder_alone_does_not_stop_a_job_being_held_for_writing_nothing() {
        // H1's second cell: **never the added write places**, which are in no
        // count, *"because the spec's case is a job that writes nothing the user
        // asked the render for and a cache folder is not that"*.
        let cache = crate::reading::Destination {
            kind: DestinationKind::AnAddedWritePlace(
                crate::reading::AnAddedWritePlace::ThePointCache,
            ),
            recorded_location: None,
            format: None,
            origin: WhereItComesFrom::AnAddedWritePlaceNamed(
                crate::reading::AnAddedWritePlace::ThePointCache,
            ),
            scene: crate::reading::WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };
        let reading = a_reading(vec![cache], vec![a_frame(1, &["VL"], None)], VERSION);
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        assert!(what_holds_this_job(
            &map,
            &inventory,
            WhichSideComposedTheMap::TheRenderWorkerAtRenderStart
        )
        .contains(&WhatHoldsTheJob::TheJobWillWriteNothingAtAll));
    }

    #[test]
    fn the_two_triggers_differ_and_the_user_is_not_asked_the_same_question_twice() {
        // H5 and H6. A user who pre-loaded and submitted has already answered
        // the writes-nothing and colliding-pair questions.
        let one = a_node_destination("Beauty", "shared", "/home/a/out", None);
        let other = a_node_destination("Other", "shared", "/home/a/out", None);
        let reading = a_reading(vec![one, other], vec![a_frame(1, &["VL"], None)], VERSION);
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        assert_eq!(
            what_holds_this_job(
                &map,
                &inventory,
                WhichSideComposedTheMap::TheRenderWorkerAtRenderStart
            ),
            vec![WhatHoldsTheJob::TwoDestinationsAimedAtOneNameInOnePlace]
        );
        assert!(what_holds_this_job(
            &map,
            &inventory,
            WhichSideComposedTheMap::TheDesktopAtSubmission
        )
        .is_empty());
    }

    #[test]
    fn the_cannot_bring_under_condition_holds_whichever_side_composed_the_map() {
        // H5's first bullet: the spec states it unconditionally.
        let long = format!("/home/a/{}", "n".repeat(250));
        let destinations = vec![a_destination(
            DestinationKind::TheRegularOutputPath,
            &format!("{long}/f_"),
            WhereItComesFrom::TheScenesOutputPath,
        )];
        let held = vec![ADestinationToMap {
            destination: &destinations[0],
            the_longest_name_it_will_write: Some(b"x".repeat(300)),
        }];
        let map = compose_the_map(
            &held,
            b"/ephemeral/smoozy/assets/set-1",
            JOB_FOLDER,
            VERSION,);
        let reading = a_reading(destinations, vec![a_frame(1, &["VL"], None)], VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        for side in [
            WhichSideComposedTheMap::TheDesktopAtSubmission,
            WhichSideComposedTheMap::TheRenderWorkerAtRenderStart,
        ] {
            assert!(what_holds_this_job(&map, &inventory, side)
                .contains(&WhatHoldsTheJob::ADestinationCannotBeBroughtUnderTheJobsFolder));
        }
    }

    #[test]
    fn the_third_condition_reads_the_collision_record_and_never_the_counts_wider_test() {
        // I15 and I18. A node destination landing on the regular output path's
        // own predicted name is the count's test and holds nothing.
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
        let map = mapped(&reading, TheVersionInHand::of(4, 5));
        let inventory = compose_the_inventory(&reading, &map);
        assert!(!what_holds_this_job(
            &map,
            &inventory,
            WhichSideComposedTheMap::TheRenderWorkerAtRenderStart
        )
        .contains(&WhatHoldsTheJob::TwoDestinationsAimedAtOneNameInOnePlace));
    }

    #[test]
    fn the_six_things_are_all_fields_of_the_recorded_reading() {
        // H8 and H2: none of the six needs a second evaluation.
        let node = a_node_destination("Beauty", "beauty", "/home/a/out", None);
        let mut reading = a_reading(vec![node], vec![a_frame(1, &["VL"], None)], VERSION);
        reading.the_tree_is_shared_with = Some(text("Second"));
        reading.the_beauty_walk = WhatTheWalkFound::Reached {
            destinations: vec![ADestinationTheBeautyReached {
                node: text("Beauty"),
                output: Some(text("beauty")),
            }],
            it_reached_the_final_output_node: true,
        };
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        let six = the_six_things(&reading, &map, &inventory, JOB_FOLDER);
        assert!(!six.the_folders_it_is_writing_into.is_empty());
        assert!(six.it_is_writing_nothing.is_none());
        assert!(six.the_colliding_pairs.is_empty());
        assert!(six.the_same_picture_reaches_more_than_one_place);
        assert!(!six.the_render_reaches_frames_and_a_video);
        assert_eq!(six.the_tree_is_shared_with, Some(text("Second")));
        assert!(!six.its_output_is_a_single_video_file);
    }

    /// **A reading whose beauty reaches these File Output node inputs, and the
    /// output path where `the_output_path` says what it writes** (`Some(true)`
    /// a video, `Some(false)` images, `None` the walk did not reach it).
    fn the_six_where_the_beauty_reaches(
        inputs: &[(&str, &str)],
        the_output_path: Option<bool>,
    ) -> TheSixThings {
        let mut destinations: Vec<crate::reading::Destination> = inputs
            .iter()
            .map(|(node, output)| a_node_destination(node, output, "/home/a/out/frames", None))
            .collect();
        let mut at = None;
        if let Some(video) = the_output_path {
            let mut regular = a_destination(
                DestinationKind::TheRegularOutputPath,
                "/home/a/out/render_",
                WhereItComesFrom::TheScenesOutputPath,
            );
            if video {
                regular.format = Some(crate::reading::Format {
                    as_the_file_records_it: text("FFMPEG"),
                    the_ending_it_writes: Some(text(".mp4")),
                    is_video_shaped: true,
                });
                at = Some("/home/a/out/render_0001-0002.mp4");
            } else {
                at = Some("/home/a/out/render_0001.png");
            }
            destinations.push(regular);
        }
        let mut reading = a_reading(
            destinations,
            vec![a_frame(1, &["VL"], at), a_frame(2, &["VL"], at)],
            VERSION,
        );
        reading.the_beauty_walk = WhatTheWalkFound::Reached {
            destinations: inputs
                .iter()
                .map(|(node, output)| ADestinationTheBeautyReached {
                    node: text(node),
                    output: Some(text(output)),
                })
                .collect(),
            it_reached_the_final_output_node: the_output_path.is_some(),
        };
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        the_six_things(&reading, &map, &inventory, JOB_FOLDER)
    }

    fn places(nodes: usize, inputs: usize, the_output_path: bool) -> ThePlacesTheBeautyIsWritten {
        ThePlacesTheBeautyIsWritten {
            file_output_nodes: nodes,
            their_inputs: inputs,
            the_output_path,
        }
    }

    /// **The Creator's eighth notes, note 4, and their answer of 2026-10-12:
    /// a video is not a pass at all.** A File Output node and a video at the
    /// output path are two outputs, named as what they are, and never the
    /// same picture written twice. The single-video line stays off too: the
    /// video is not all the job writes.
    #[test]
    fn frames_from_a_node_and_a_video_from_the_output_path_are_not_written_twice() {
        let six = the_six_where_the_beauty_reaches(&[("Beauty", "beauty")], Some(true));
        assert!(!six.the_same_picture_reaches_more_than_one_place);
        assert_eq!(six.where_the_same_picture_is_written, places(0, 0, false));
        assert!(six.the_render_reaches_frames_and_a_video);
        assert!(!six.its_output_is_a_single_video_file);
    }

    /// **Where both write images the twice line stands**, naming a File
    /// Output node and the output path, and the two outputs are not told.
    #[test]
    fn a_node_and_an_image_output_path_are_the_same_picture_written_twice() {
        let six = the_six_where_the_beauty_reaches(&[("Beauty", "beauty")], Some(false));
        assert!(six.the_same_picture_reaches_more_than_one_place);
        assert_eq!(six.where_the_same_picture_is_written, places(1, 1, true));
        assert!(!six.the_render_reaches_frames_and_a_video);
    }

    /// **Two File Output nodes and a video**: the twice line names the two
    /// nodes and never the output path, and the two outputs are told as well.
    #[test]
    fn two_nodes_and_a_video_are_written_twice_and_are_frames_and_a_video() {
        let six = the_six_where_the_beauty_reaches(
            &[("Beauty", "beauty"), ("Beauty.001", "beauty")],
            Some(true),
        );
        assert!(six.the_same_picture_reaches_more_than_one_place);
        assert_eq!(six.where_the_same_picture_is_written, places(2, 2, false));
        assert!(six.the_render_reaches_frames_and_a_video);
    }

    /// **The runner's ruling of 2026-10-12: the line names what the walk
    /// found.** One node through two inputs is one node, two inputs; two nodes
    /// and an image output path are three places; one place is no line.
    #[test]
    fn the_places_named_are_the_places_the_walk_found() {
        let one_node_two_inputs =
            the_six_where_the_beauty_reaches(&[("Beauty", "exr"), ("Beauty", "png")], None);
        assert!(one_node_two_inputs.the_same_picture_reaches_more_than_one_place);
        assert_eq!(one_node_two_inputs.where_the_same_picture_is_written, places(1, 2, false));
        let two_nodes_and_the_path = the_six_where_the_beauty_reaches(
            &[("Beauty", "beauty"), ("Beauty.001", "beauty")],
            Some(false),
        );
        assert_eq!(two_nodes_and_the_path.where_the_same_picture_is_written, places(2, 2, true));
        let one = the_six_where_the_beauty_reaches(&[("Beauty", "beauty")], None);
        assert!(!one.the_same_picture_reaches_more_than_one_place);
        assert_eq!(one.where_the_same_picture_is_written, places(0, 0, false));
    }

    /// **A record of the six written before 2026-10-12 still reads**, with
    /// neither new field: nothing named, nothing told.
    #[test]
    fn six_things_recorded_before_the_places_were_named_still_read() {
        let six = the_six_where_the_beauty_reaches(&[("Beauty", "beauty")], Some(false));
        let mut recorded = serde_json::to_value(&six).expect("it writes");
        let fields = recorded.as_object_mut().expect("the six");
        fields.remove("where_the_same_picture_is_written").expect("written");
        fields.remove("the_render_reaches_frames_and_a_video").expect("written");
        let old: TheSixThings = serde_json::from_value(recorded).expect("an old record reads");
        assert!(old.the_same_picture_reaches_more_than_one_place);
        assert_eq!(old.where_the_same_picture_is_written, places(0, 0, false));
        assert!(!old.the_render_reaches_frames_and_a_video);
    }

    #[test]
    fn the_folders_shown_are_the_folders_of_the_files_counted() {
        // **P11 and hole 4 of the Creator's second notes, 2026-10-04.** Three
        // destinations in three folders: a node fed by a layer that renders, a
        // node fed only by a layer that renders at no frame (as one the job
        // pinned off does), and a cache. Only the first names a folder — the
        // second owes no file, and the cache is never shown.
        let mut beauty = a_node_destination("Beauty", "beauty", "/home/a/out/beauty", None);
        beauty.the_view_layers_that_feed_it = vec![text("VL")];
        let mut ao = a_node_destination("AO", "ao", "/home/a/out/ao", None);
        ao.the_view_layers_that_feed_it = vec![text("AO")];
        let mut bake = a_destination(
            DestinationKind::AnAddedWritePlace(crate::reading::AnAddedWritePlace::TheBakeOutputPath),
            "//",
            WhereItComesFrom::AnAddedWritePlaceNamed(
                crate::reading::AnAddedWritePlace::TheBakeOutputPath,
            ),
        );
        bake.format = None;
        let reading = a_reading(vec![beauty, ao, bake], vec![a_frame(1, &["VL"], None)], VERSION);
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        let six = the_six_things(&reading, &map, &inventory, JOB_FOLDER);
        assert_eq!(six.the_folders_it_is_writing_into, vec![b"beauty".to_vec()]);
        // The map's own list still names both of the render's destinations.
        assert_eq!(
            map.the_folders_the_job_will_write_into(JOB_FOLDER),
            vec![b"beauty".to_vec(), b"ao".to_vec()]
        );
    }

    #[test]
    fn a_withheld_inventory_holds_no_job_on_a_count_it_does_not_have() {
        // I9a: nothing then reads absence as evidence.
        let mut reading = a_reading(
            vec![a_node_destination("Beauty", "beauty", "/home/a/out", None)],
            vec![a_frame(1, &["VL"], None)],
            VERSION,
        );
        reading.views =
            crate::reading::HowManyViewsTheSceneWrites::MoreThanOneWrittenSeparately { how_many: 2 };
        let map = mapped(&reading, VERSION);
        let inventory = compose_the_inventory(&reading, &map);
        assert!(what_holds_this_job(
            &map,
            &inventory,
            WhichSideComposedTheMap::TheRenderWorkerAtRenderStart
        )
        .is_empty());
    }
}

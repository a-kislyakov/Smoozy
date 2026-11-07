//! **R4 item 7, composed once: the recorded reading and everything derived
//! from it, by one function both sides call.**
//!
//! # Why this is one function rather than two
//!
//! **R7: the destination map is computed once per job, on whichever side ran
//! the reading, and the render applies it rather than deriving one.** R9 puts
//! it in the sharpest form — *"nothing anywhere recomputes a map that
//! exists"*, and *"a builder that finds a second evaluation of the shared-run
//! rule has found the fault Row 122 exists to prevent"*.
//!
//! **Two sides run the reading and only one composition may exist.** The
//! render worker runs it at `Rendering` P1 step 4, where the manifest carries
//! no map or the file has changed under it (R13); the desktop runs it at
//! pre-load and composes the manifest's own copy at submission (R7's first
//! bullet). **Both call the function below.** A second arrangement of the same
//! four calls would be the one-fact-in-two-places fault this stage keeps
//! paying for, and it would show as the user agreeing to one set of folders
//! and the render writing into another.
//!
//! # What it does, and the four things it does not decide
//!
//! It runs, in order, [`the_destinations_that_will_be_written`],
//! [`the_one_document_from`], [`compose_the_map`], [`compose_the_inventory`]
//! and the two picks. **Every one of those already existed and none of them is
//! re-decided here**: this file is the order and the shape, in the way
//! `Rendering`'s own pipeline is the order of the nine steps.
//!
//! **It takes no reading and starts no process.** Performing the reading is
//! `Rendering`'s `acting::reading` on the machine and the file-inspection
//! worker's at pre-load; what arrives here is what the script printed.
//!
//! **It judges no hold.** [`crate::holds::what_holds_this_job`] is H1's and is
//! called by the side that has an answer to take — which is `Rendering` P1
//! step 5 on the machine, with H5's two triggers.
//!
//! **Every marker of another document's is written with that document at the
//! site**, because this plan defines a `P1` of its own and a header list
//! naming it would resolve to the wrong clause.

use crate::inventory::{compose_the_inventory, the_name_owed_at, WhatIsOwedAt, WhatTheInventoryIs};
use crate::main_output::{
    what_the_main_output_is, ADestinationTheBeautyReached, ThePicks, WhatTheMainOutputIs,
    WhatTheWalkFound, WhereTheRenderEnters,
};
use crate::mapping::{compose_the_map, ADestinationToMap, TheMap};
use crate::manifest::TheReadingAndEverythingDerivedFromIt;
use crate::reading::{
    the_per_frame_set_with_the_jobs_pins, AViewLayerTheJobPins, Destination, DestinationKind,
    TheOneDocument, TheOutputsHalf,
};
use crate::reading_script::{
    the_destinations_that_will_be_written, the_one_document_from, ThePrintedReading,
    WhyItIsNoReading,
};

/// What the composition needs beside what the script printed.
///
/// **Every field is the caller's own fact and none of them is in the reading**:
/// where the file sits, where the job writes, and where the beauty walk enters.
pub struct WhatTheReadingIsComposedAgainst<'a> {
    /// **The folder the `.blend` sits in on the machine**, as bytes — what
    /// P4's blend-file shorthand resolves against. **R4a: never decoded.**
    pub the_folder_the_blend_sits_in: &'a [u8],
    /// **The job output folder** (M7), as the manifest names it.
    pub job_output_folder: &'a str,
    /// **B1: where the beauty walk enters** — the job's scene and the view
    /// layer it is followed from.
    pub enters_at: &'a WhereTheRenderEnters,
    /// **R4 item 5: the view layers the job pins**, applied to the per-frame
    /// set before the map, the inventory and the picks are composed from it
    /// ([`crate::reading::the_per_frame_set_with_the_jobs_pins`]). Empty where
    /// the job pins none — and where there is no job yet, as at a read before
    /// submitting.
    pub the_jobs_pins: &'a [AViewLayerTheJobPins],
}

/// **The recorded reading and everything derived from it**, as the side that
/// ran the reading holds it.
///
/// **The reading is both halves** (R3, `Render Settings` Q1): the document is
/// what goes into the job record and into a manifest, and the settings half is
/// what `Rendering` P1a validates names against.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TheReadingAndWhatFollows {
    /// R1's one document, with R4 item 7's beauty walk already derived on its
    /// outputs half.
    pub reading: TheOneDocument,
    /// **The map, computed once** (R7).
    pub map: TheMap,
    /// The expected-output inventory, which carries the collision record and
    /// the predicted names (I14).
    pub inventory: WhatTheInventoryIs,
    /// **B13: each pick is made once, at the moment the recorded reading is
    /// composed**, and B15 forbids recomputing either.
    pub picks: ThePicks,
}

impl TheReadingAndWhatFollows {
    /// **The file the render worker wrote, read back** (`Job files and
    /// outputs` H9) — the same shape the attempt record and the manifest read
    /// back by, so the desktop's fill-in reads it as the machine's own shape
    /// (`Queue & History` N9; the 38's piece 10). Nothing where the bytes are
    /// not that shape.
    pub fn read_back(bytes: &[u8]) -> Option<TheReadingAndWhatFollows> {
        serde_json::from_slice(bytes).ok()
    }

    /// **The shape M1's recorded-reading field carries**, for the side that
    /// writes a manifest (R7's first bullet).
    ///
    /// **It is a rendering of what this already holds and never a second
    /// composition.**
    pub fn as_the_manifest_carries_it(&self) -> TheReadingAndEverythingDerivedFromIt {
        TheReadingAndEverythingDerivedFromIt {
            reading: self.reading.outputs.clone(),
            map: self.map.clone(),
            inventory: self.inventory.clone(),
            picks: self.picks.clone(),
        }
    }
}

/// **R4 item 7, performed: the reading, the map, the inventory and the two
/// picks, out of one printed reading.**
///
/// **`Render Settings` Q7 is applied first** by
/// [`the_one_document_from`]: a document whose error map is not empty is no
/// reading, and nothing of it is carried forward. `Job files and outputs` C9
/// makes that a failed attempt at the render's step 4.
pub fn the_reading_and_everything_derived_from(
    printed: ThePrintedReading,
    against: WhatTheReadingIsComposedAgainst<'_>,
) -> Result<TheReadingAndWhatFollows, WhyItIsNoReading> {
    // **R4 item 2: a file output node's outputs are destinations only where the
    // node will run**, and the rule lives on
    // [`crate::reading::GatingConditions::a_file_output_node_will_write`]. The
    // script records every node's outputs with the gating beside them, so the
    // filtering happens here, once, **before the map is composed** — a map
    // composed over a wider set than the inventory reads would leave the
    // inventory dropping destinations with no pair, which is a count that then
    // reads as a fact.
    let mut printed = printed;
    printed.outputs.destinations = the_destinations_that_will_be_written(&printed.outputs);

    let reading = the_one_document_from(printed, against.enters_at)?;

    // **R4 item 5, 2026-10-04: the job's own choices apply before anything
    // reads the per-frame set**, so the map, the inventory and the picks are
    // composed from the set as the job will render it. **The reading that is
    // kept stays what the file says** — this is a working copy, never written.
    let as_the_job_renders_it = TheOutputsHalf {
        per_frame: the_per_frame_set_with_the_jobs_pins(
            &reading.outputs.per_frame,
            against.the_jobs_pins,
        ),
        ..reading.outputs.clone()
    };

    // **N8: the budget is measured against the predicted deepest file path and
    // not against the folder.** *"Checking the folder alone would pass a
    // destination whose files then fail at open."* The recipe that predicts a
    // name is §7's and is [`the_name_owed_at`]'s; what is handed to the rule is
    // the longest it predicts over the frames the job covers.
    let longest: Vec<Option<Vec<u8>>> = as_the_job_renders_it
        .destinations
        .iter()
        .map(|destination| the_longest_name_it_will_write(&as_the_job_renders_it, destination))
        .collect();
    let to_map: Vec<ADestinationToMap<'_>> = as_the_job_renders_it
        .destinations
        .iter()
        .zip(longest.iter())
        .map(|(destination, longest)| ADestinationToMap {
            destination,
            the_longest_name_it_will_write: longest.clone(),
        })
        .collect();
    let map = compose_the_map(
        &to_map,
        against.the_folder_the_blend_sits_in,
        against.job_output_folder,
        as_the_job_renders_it.taken_by,
    );
    drop(to_map);

    let inventory = compose_the_inventory(&as_the_job_renders_it, &map);
    let picks = the_two_picks(&as_the_job_renders_it, &inventory);

    Ok(TheReadingAndWhatFollows {
        reading,
        map,
        inventory,
        picks,
    })
}

/// **The longest name §7's recipes predict for one destination over the frames
/// the job covers**, or nothing where it owes no file at any of them.
///
/// **An added write place has none**, which is I14 — it writes files this stage
/// has no recipe to name — and the budget is then measured against the folder
/// alone, which is all there is to measure.
fn the_longest_name_it_will_write(
    reading: &TheOutputsHalf,
    destination: &Destination,
) -> Option<Vec<u8>> {
    if destination.as_an_added_write_place().is_some() {
        return None;
    }
    let mut longest: Option<Vec<u8>> = None;
    for frame in &reading.per_frame {
        let WhatIsOwedAt::AName(name) = the_name_owed_at(reading, destination, frame) else {
            continue;
        };
        if longest.as_ref().is_none_or(|held| held.len() < name.len()) {
            longest = Some(name);
        }
    }
    longest
}

/// **Which of B4's outcomes a reading's main output is**, off its gating
/// conditions and the walk it carries — the one composition of
/// [`what_the_main_output_is`]'s inputs, read by the picks below and by a
/// derivative's choosing ([`crate::derivatives::the_present_files_at`]).
pub fn the_main_output_of(reading: &TheOutputsHalf) -> WhatTheMainOutputIs {
    what_the_main_output_is(
        reading.the_compositor_runs(),
        reading.gating.the_scenes_own_frames_are_written(),
        reading
            .gating
            .the_render_goes_out_through_a_sequencer_holding_a_scene_strip,
        &reading.the_beauty_walk,
    )
}

/// **B11 and B13: the two picks, made once, at the moment the recorded reading
/// is composed.**
///
/// **B11: arbitrary is allowed and unstable is not.** Where two or more
/// candidates are left the pick takes the first the reading holds, which is the
/// file's own order; what matters is that it is taken once (B15).
fn the_two_picks(reading: &TheOutputsHalf, inventory: &WhatTheInventoryIs) -> ThePicks {
    let which = the_main_output_of(reading);
    // **B7: where the beauty reaches a destination inside a file output node,
    // that destination wins over the regular output path.** Which of two or
    // more is B11's arbitrary-but-once pick, and it is the first the walk
    // recorded, which is the file's own order.
    let the_main_output = match which {
        WhatTheMainOutputIs::ADestinationInsideAFileOutputNode => {
            match &reading.the_beauty_walk {
                WhatTheWalkFound::Reached { destinations, .. } => destinations.first().cloned(),
                // B7's case cannot arise from a walk that found nothing, and
                // saying so is cheaper than a branch that pretends it could.
                WhatTheWalkFound::Blocked { .. } | WhatTheWalkFound::NoEntry => None,
            }
        }
        // **The regular output path is the main output and it is not a
        // destination inside a node**, which is the only shape
        // [`ADestinationTheBeautyReached`] has. B8's *no main output* and this
        // case are told apart by the folder *show outputs* opens, which is the
        // job's own folder in both.
        WhatTheMainOutputIs::TheRegularOutputPath | WhatTheMainOutputIs::ThereIsNone => None,
    };
    let the_preview_source = the_predicted_path_of(&the_main_output, which, reading, inventory);
    ThePicks::made_once(the_main_output, the_preview_source)
}

/// **The preview source, as its predicted path under the job output folder**
/// (B13).
///
/// **It is the first entry the inventory holds for the main output's own
/// destination**, which is B11's *the first the reading holds*. **Nothing
/// present is read here and nothing could be**: the pick is made at the moment
/// the recorded reading is composed, before the job has rendered a byte, which
/// is exactly what B15 asks for — *"a pick recomputed at each opening satisfies
/// every other sentence in the spec's section and breaks this one"*.
///
/// **Absent where the job has no main output, or where the inventory was
/// withheld** (I9a), in which case
/// [`crate::main_output::which_file_the_preview_reads`] falls to its first
/// candidate, which is what B18's own last line does.
fn the_predicted_path_of(
    the_main_output: &Option<ADestinationTheBeautyReached>,
    which: WhatTheMainOutputIs,
    reading: &TheOutputsHalf,
    inventory: &WhatTheInventoryIs,
) -> Option<Vec<u8>> {
    let WhatTheInventoryIs::It(inventory) = inventory else {
        return None;
    };
    let at = match which {
        WhatTheMainOutputIs::ThereIsNone => return None,
        WhatTheMainOutputIs::TheRegularOutputPath => reading
            .destinations
            .iter()
            .position(|destination| destination.kind == DestinationKind::TheRegularOutputPath)?,
        WhatTheMainOutputIs::ADestinationInsideAFileOutputNode => {
            let reached = the_main_output.as_ref()?;
            reading
                .destinations
                .iter()
                .position(|destination| it_is_the_one_the_beauty_reached(destination, reached))?
        }
    };
    inventory
        .entries
        .iter()
        .find(|entry| entry.the_destinations_aiming_at_it.contains(&at))
        .map(|entry| entry.predicted_path.clone())
}

/// Whether a destination is the one the walk named — **by the node's own name
/// and the output's, as bytes**, which is `Rendering` L15's rule and this
/// crate's for every name a reading produced.
fn it_is_the_one_the_beauty_reached(
    destination: &Destination,
    reached: &ADestinationTheBeautyReached,
) -> bool {
    // **From 5.3 by its effect as well** (R4 item 2, B4), since two effects'
    // groups can hold nodes of one name.
    if destination.the_effect() != reached.the_effect.as_ref() {
        return false;
    }
    match &destination.kind {
        DestinationKind::AnOutputOfAFileOutputNode { node, output, .. } => {
            node.as_bytes() == reached.node.as_bytes()
                && reached
                    .output
                    .as_ref()
                    .is_some_and(|wanted| wanted.as_bytes() == output.as_bytes())
        }
        // **I7: where the node gathers its outputs into one file the
        // destination is the whole node**, and the walk records no output.
        DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { node, .. } => {
            node.as_bytes() == reached.node.as_bytes() && reached.output.is_none()
        }
        DestinationKind::TheRegularOutputPath | DestinationKind::AnAddedWritePlace(_) => false,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::inventory::tests::{a_frame, a_node_destination, text};
    use crate::main_output::{Link, Node, NodeKind, Socket, TheCompositingTree};
    use crate::reading::tests::{a_destination, plain_gating};
    use crate::reading::{
        AnAddedWritePlace, ArtistsText, HowManyViewsTheSceneWrites, TheVersionInHand, WhereItComesFrom,
    };
    use crate::reading_script::TheObservedOutputs;

    const BLEND_SITS_IN: &[u8] = b"/ephemeral/smoozy/assets";
    const JOB_FOLDER: &str = "/ephemeral/smoozy/jobs/j1";

    fn enters_at() -> WhereTheRenderEnters {
        WhereTheRenderEnters {
            scene: text("Kitchen"),
            view_layer: text("ViewLayer"),
        }
    }

    fn against(enters_at: &WhereTheRenderEnters) -> WhatTheReadingIsComposedAgainst<'_> {
        WhatTheReadingIsComposedAgainst {
            the_folder_the_blend_sits_in: BLEND_SITS_IN,
            job_output_folder: JOB_FOLDER,
            enters_at,
            the_jobs_pins: &[],
        }
    }

    /// **The tree the beauty is walked over**: the render's entry point, a file
    /// output node, and the final output node, with the beauty wired to both.
    fn a_tree() -> TheCompositingTree {
        TheCompositingTree {
            nodes: vec![
                Node {
                    name: text("Render Layers"),
                    kind: NodeKind::RenderLayers {
                        scene: text("Kitchen"),
                        view_layer: text("ViewLayer"),
                        // **The beauty's socket is not first**, so a walk
                        // reading by position takes the wrong picture (B19a).
                        // The name is `Image` and not the pass's own name,
                        // which B1 was corrected to on 2026-09-20.
                        outputs: vec![text("Depth"), text("Image")],
                    },
                },
                Node {
                    name: text("Beauty"),
                    kind: NodeKind::AFileOutputNode {
                        inputs: vec![text("Image")],
                        gathering_into_one_file: false,
                        muted: false,
                    },
                },
                Node {
                    name: text("Composite"),
                    kind: NodeKind::TheFinalOutputNode,
                },
            ],
            links: vec![
                Link {
                    from: Socket { node: 0, index: 1 },
                    to: Socket { node: 1, index: 0 },
                },
                Link {
                    from: Socket { node: 0, index: 1 },
                    to: Socket { node: 2, index: 0 },
                },
            ],
        }
    }

    /// One added write place, which is a folder and carries no format (I14).
    fn a_bake_folder() -> Destination {
        let mut destination = a_destination(
            DestinationKind::AnAddedWritePlace(AnAddedWritePlace::TheBakeOutputPath),
            "//",
            WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::TheBakeOutputPath),
        );
        destination.format = None;
        destination
    }

    /// **What a printed reading of a two-destination file looks like** — the
    /// regular output path, one file output node's output, and one added write
    /// place, over three frames.
    pub(crate) fn a_printed_reading() -> ThePrintedReading {
        let mut regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "//out/f_",
            WhereItComesFrom::TheScenesOutputPath,
        );
        regular.the_view_layers_that_feed_it = vec![text("ViewLayer")];
        let mut node = a_node_destination("Beauty", "Image", "//nodes/beauty/", None);
        node.the_view_layers_that_feed_it = vec![text("ViewLayer")];
        ThePrintedReading {
            outputs: TheObservedOutputs {
                taken_by: TheVersionInHand::of(4, 5),
                gating: plain_gating(),
                destinations: vec![regular, node, a_bake_folder()],
                scenes_reached_through_a_sequencer_strip: Vec::new(),
                the_tree_is_shared_with: None,
                per_frame: (1..=3)
                    .map(|frame| {
                        a_frame(
                            frame,
                            &["ViewLayer"],
                            Some(&format!("/home/artist/out/f_{frame:04}.png")),
                        )
                    })
                    .collect(),
                recorded_locations: Vec::new(),
                views: HowManyViewsTheSceneWrites::One,
                the_compositing_tree: Some(a_tree()),
                the_view_transform: None,
            },
            settings: crate::manifest::CarriedAndNotInterpretedHere(serde_json::json!({})),
            dependencies: crate::manifest::CarriedAndNotInterpretedHere(serde_json::json!({})),
            errors: std::collections::BTreeMap::new(),
            the_baked_caches_on_the_machine: Vec::new(),
            the_scene_holds_a_simulation_not_baked: false,
            the_scene_it_read: None,
            the_open_files_path: None,
            the_check_is_written_as_a_picture: false,
        }
    }

    /// **One call composes the map, the inventory and the two picks.**
    #[test]
    fn one_call_composes_the_map_the_inventory_and_the_picks() {
        let enters = enters_at();
        let composed = the_reading_and_everything_derived_from(a_printed_reading(), against(&enters))
            .expect("the error map is empty, so it is a reading");
        // **Every destination the reading kept has a pair** (P10), so nothing
        // the inventory reads can be missing one.
        assert_eq!(
            composed.map.pairs.len(),
            composed.reading.outputs.destinations.len()
        );
        let WhatTheInventoryIs::It(inventory) = &composed.inventory else {
            panic!("this file's names are predictable: {:?}", composed.inventory);
        };
        // Three frames, two destinations the render asked for, so six entries.
        assert_eq!(inventory.the_count(), 6);
        // **The added write place is a folder and in no count** (I14).
        assert_eq!(inventory.added_write_place_folders.len(), 1);
        // **R4 item 7's beauty walk is derived rather than read** and reaches
        // both the node and the final output node.
        assert!(matches!(
            composed.reading.outputs.the_beauty_walk,
            WhatTheWalkFound::Reached {
                it_reached_the_final_output_node: true,
                ..
            }
        ));
    }

    /// **R4 item 5, 2026-10-04: the job's pins apply before the map and the
    /// inventory are composed, and the reading kept is the file's own.** The
    /// Creator's note 14: a layer the job switched off owed files, so the count
    /// read 5 of 7 for good. Here a second layer feeds a node of its own; pinned
    /// off it owes nothing and the count drops by one per frame, and pinned on
    /// it owes a file at a frame the file itself switches it off at.
    #[test]
    fn a_pinned_layer_renders_at_every_frame_or_at_none_and_the_reading_stays_the_files() {
        let with_ao = || {
            let mut printed = a_printed_reading();
            let mut ao = a_node_destination("AO", "AO", "//nodes/beauty/", None);
            ao.the_view_layers_that_feed_it = vec![text("AO")];
            printed.outputs.destinations.insert(2, ao);
            for frame in &mut printed.outputs.per_frame {
                if frame.frame != 2 {
                    frame.view_layers_that_render.push(text("AO"));
                }
            }
            printed
        };
        let count = |composed: &TheReadingAndWhatFollows| match &composed.inventory {
            WhatTheInventoryIs::It(inventory) => inventory.the_count(),
            other => panic!("withheld: {other:?}"),
        };
        let enters = enters_at();
        let pinned = |on: bool| {
            vec![AViewLayerTheJobPins {
                layer: ArtistsText::of(b"AO".to_vec()),
                on,
            }]
        };

        // The file's own: AO renders at frames 1 and 3, so 6 + 2.
        let own = the_reading_and_everything_derived_from(with_ao(), against(&enters)).expect("a reading");
        assert_eq!(count(&own), 8);

        let off = pinned(false);
        let composed = the_reading_and_everything_derived_from(
            with_ao(),
            WhatTheReadingIsComposedAgainst {
                the_jobs_pins: &off,
                ..against(&enters)
            },
        )
        .expect("a reading");
        assert_eq!(count(&composed), 6, "pinned off, AO owes nothing at any frame");
        // **The recorded reading stays what the file says.**
        assert_eq!(composed.reading, own.reading);

        let on = pinned(true);
        let composed = the_reading_and_everything_derived_from(
            with_ao(),
            WhatTheReadingIsComposedAgainst {
                the_jobs_pins: &on,
                ..against(&enters)
            },
        )
        .expect("a reading");
        assert_eq!(count(&composed), 9, "pinned on, AO owes a file at every frame");
    }

    /// **B7 and B13: the beauty reaches a file output node's output, so that
    /// destination is the main output and the preview source is its own
    /// predicted path** — a path under the job output folder and never one of
    /// the artist's own.
    #[test]
    fn the_picks_are_made_once_and_the_preview_source_is_a_predicted_path() {
        let enters = enters_at();
        let composed = the_reading_and_everything_derived_from(a_printed_reading(), against(&enters))
            .expect("a reading");
        let main = composed
            .picks
            .the_main_output
            .as_ref()
            .expect("the beauty reached the Beauty node");
        assert_eq!(main.node.as_bytes(), b"Beauty");
        let source = composed
            .picks
            .the_preview_source
            .as_ref()
            .expect("the main output owes a file at every frame");
        assert!(
            source.starts_with(JOB_FOLDER.as_bytes()),
            "{}",
            String::from_utf8_lossy(source)
        );
        // **Made once**: the same reading composed again gives the same pick,
        // which is what B15 asks of the pair.
        let again = the_reading_and_everything_derived_from(a_printed_reading(), against(&enters))
            .expect("a reading");
        assert_eq!(again.picks, composed.picks);
    }

    /// **`Render Settings` Q7: a document whose error map is not empty is no
    /// reading**, and nothing of it is carried forward — no map, no inventory,
    /// no pick.
    #[test]
    fn a_document_with_a_complaint_in_it_composes_nothing() {
        let mut printed = a_printed_reading();
        printed.errors.insert(
            "the regular output path at frame 1".to_string(),
            "Unknown variable referenced in template expression".to_string(),
        );
        let enters = enters_at();
        let why = the_reading_and_everything_derived_from(printed, against(&enters))
            .expect_err("an error map that is not empty is no reading");
        assert!(matches!(why, WhyItIsNoReading::TheErrorMapIsNotEmpty(_)));
    }

    /// **R4 item 2: a node's outputs are destinations only where the node will
    /// run.** With the compositing switch off the node writes nothing, so its
    /// destination leaves the set before the map is composed — and the count
    /// falls with it rather than the inventory dropping a destination the map
    /// never paired.
    #[test]
    fn a_node_that_will_not_run_leaves_the_set_before_the_map_is_composed() {
        let mut printed = a_printed_reading();
        printed.outputs.gating.compositing_switch = false;
        for frame in &mut printed.outputs.per_frame {
            frame.gating.compositing_switch = false;
        }
        let enters = enters_at();
        let composed =
            the_reading_and_everything_derived_from(printed, against(&enters)).expect("a reading");
        assert!(
            !composed
                .reading
                .outputs
                .destinations
                .iter()
                .any(|destination| matches!(
                    destination.kind,
                    DestinationKind::AnOutputOfAFileOutputNode { .. }
                )),
            "the node will not write, so it is no destination"
        );
        assert_eq!(
            composed.map.pairs.len(),
            composed.reading.outputs.destinations.len()
        );
    }

    /// **N8's budget is measured against the predicted deepest file path**, so
    /// the rule is handed the longest name §7's recipes predict rather than
    /// nothing. Without it a destination whose folder fits and whose files do
    /// not passes the map and fails at open.
    #[test]
    fn the_budget_is_handed_the_longest_name_the_recipes_predict() {
        let enters = enters_at();
        let reading = the_one_document_from(a_printed_reading(), &enters).expect("a reading");
        let node = reading
            .outputs
            .destinations
            .iter()
            .find(|destination| {
                matches!(
                    destination.kind,
                    DestinationKind::AnOutputOfAFileOutputNode { .. }
                )
            })
            .expect("the fixture holds one");
        assert_eq!(
            the_longest_name_it_will_write(&reading.outputs, node),
            Some(b"Image0001.png".to_vec()),
            "4.5 joins the output's own piece of path, the frame number and the ending"
        );
        // **An added write place has none** (I14).
        let place = reading
            .outputs
            .destinations
            .iter()
            .find(|destination| destination.as_an_added_write_place().is_some())
            .expect("the fixture holds one");
        assert_eq!(the_longest_name_it_will_write(&reading.outputs, place), None);
    }

    /// **A name that will not fit is caught because the budget measured the
    /// file and not the folder.** The folder fits and the files do not, which
    /// is the exact case N8 names — and the job is then held under H1's first
    /// condition rather than failing at open.
    #[test]
    fn a_destination_whose_files_will_not_fit_is_not_mappable() {
        let mut printed = a_printed_reading();
        let long = "o".repeat(250);
        printed.outputs.destinations[1] =
            a_node_destination("Beauty", &long, "//nodes/beauty/", None);
        printed.outputs.destinations[1].the_view_layers_that_feed_it = vec![text("ViewLayer")];
        let enters = enters_at();
        let composed =
            the_reading_and_everything_derived_from(printed, against(&enters)).expect("a reading");
        assert!(
            composed.map.a_destination_is_not_mappable(),
            "N8 measures the predicted deepest file path"
        );
    }

    /// **R9, held where it can be checked**: this crate declares the shared-run
    /// rule in one module and calls it from one other, and a second call site
    /// is the fault Row 122 exists to prevent.
    #[test]
    fn nothing_else_in_this_crate_composes_a_map_outside_its_own_tests() {
        let at = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut callers = Vec::new();
        for file in std::fs::read_dir(&at)
            .and_then(|at| at.collect::<Result<Vec<_>, _>>())
            .expect("the source folder")
        {
            let path = file.path();
            if path.extension().and_then(|it| it.to_str()) != Some("rs") {
                continue;
            }
            let held = std::fs::read_to_string(&path).expect("a source file");
            // A module's own tests exercising the rule is not a second
            // evaluation in the application.
            let before_the_tests = held.split("#[cfg(test)]").next().unwrap_or_default();
            if before_the_tests.contains("compose_the_map(") {
                callers.push(path.file_name().unwrap().to_string_lossy().to_string());
            }
        }
        callers.sort();
        assert_eq!(
            callers,
            vec!["composing.rs".to_string(), "mapping.rs".to_string()],
            "R9: the rule is declared in mapping.rs and called from here alone"
        );
    }
}

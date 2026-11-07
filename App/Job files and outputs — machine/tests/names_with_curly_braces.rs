//! **A File Output node's names with curly braces, over what Blender really
//! printed and really wrote** — the Creator's thirteenth notes, piece P
//! (notes 5, 9 and 10; `Architecture/Job files and outputs` I2's *A
//! curly-brace expression in a node's names*).
//!
//! **The reading is real**: `the_curly_brace_stand_in_as_blender_5_3_printed_it.json`
//! is what Smoozy's own reading script printed between its two marks in
//! Blender 5.3 beta, headless, on the stand-in the ignored desk test
//! `names_with_curly_braces_at_the_desk` makes — two View Layers, *a* and *b*,
//! each Render Layers node into its own File Output node with the base file
//! names `{blend_name} - a` and `{blend_name} - b`, no Group Output, saved as
//! `standin.blend`. Blender 5.1.2 printed the same document but for its paths.
//!
//! **The names are real**: [`WHAT_BLENDER_WROTE`] is the listing of what the
//! same Blender's render of frame 1 wrote, left alone and through Smoozy's
//! redirect alike, with each file's size (2026-10-18, the build's Mac).

use smoozy_job_files_machine::composing::{
    the_reading_and_everything_derived_from, TheReadingAndWhatFollows, WhatTheReadingIsComposedAgainst,
};
use smoozy_job_files_machine::derivatives::{the_present_files_at, the_rungs_are_told, what_one_frame_is_made_of};
use smoozy_job_files_machine::holds::the_six_things;
use smoozy_job_files_machine::inventory::WhatTheInventoryIs;
use smoozy_job_files_machine::main_output::{
    the_candidates, ADestinationTheBeautyReached, WhatTheMainOutputIs, WhereThePictureCameFrom, WhereTheRenderEnters,
};
use smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere;
use smoozy_job_files_machine::reading::{ArtistsText, TheVersionInHand, THE_CLOSING_SENTINEL, THE_OPENING_SENTINEL};
use smoozy_job_files_machine::reading_script::the_printed_reading_in;
use smoozy_job_files_machine::redirect::compose_the_launch_material;
use smoozy_job_files_machine::reckoning::{
    the_completeness_statement, the_frames_the_statement_counts_already_there, AWalkEntry, WhatKindOfEntry,
};

/// What Smoozy's reading printed, in Blender 5.3 beta.
const WHAT_THE_READING_PRINTED: &str = include_str!("the_curly_brace_stand_in_as_blender_5_3_printed_it.json");

/// **What the render of frame 1 wrote under the job's folder**, by name and
/// size — `_preview0001.png` the regular output path's, the other two the
/// nodes'.
const WHAT_BLENDER_WROTE: [(&str, u64); 3] = [
    ("_preview/standin - aImage0001.png", 13452),
    ("_preview/standin - bImage0001.png", 13452),
    ("_preview0001.png", 10940),
];

/// The folder the stand-in sat in when it was read, as its printed path says.
const THE_FOLDER_THE_BLEND_SAT_IN: &str =
    "/private/tmp/smoozy-test/desk-53/rendering";

const THE_JOB_OUTPUT_FOLDER: &str = "/home/ubuntu/Render/standin job";

fn composed() -> TheReadingAndWhatFollows {
    composed_from(&the_document())
}

/// The real printed document, to be changed for one case.
fn the_document() -> serde_json::Value {
    serde_json::from_str(WHAT_THE_READING_PRINTED).expect("the real document parses")
}

/// A printed document, taken in between its marks and composed as the
/// machine composes it.
fn composed_from(document: &serde_json::Value) -> TheReadingAndWhatFollows {
    let mut stream = THE_OPENING_SENTINEL.to_vec();
    stream.extend_from_slice(&serde_json::to_vec(document).expect("it writes"));
    stream.extend_from_slice(THE_CLOSING_SENTINEL);
    let printed = the_printed_reading_in(&stream).expect("the document is between its marks");
    the_reading_and_everything_derived_from(
        printed,
        WhatTheReadingIsComposedAgainst {
            the_folder_the_blend_sits_in: THE_FOLDER_THE_BLEND_SAT_IN.as_bytes(),
            job_output_folder: THE_JOB_OUTPUT_FOLDER,
            enters_at: &WhereTheRenderEnters {
                scene: ArtistsText::of(b"Scene".to_vec()),
                view_layer: ArtistsText::of(b"a".to_vec()),
            },
            the_jobs_pins: &[],
        },
    )
    .expect("a reading")
}

fn under_the_job(name: &str) -> Vec<u8> {
    format!("{THE_JOB_OUTPUT_FOLDER}/{name}").into_bytes()
}

fn the_size_blender_wrote(path: &[u8]) -> Option<u64> {
    WHAT_BLENDER_WROTE
        .iter()
        .find(|(name, _)| under_the_job(name) == path)
        .map(|(_, size)| *size)
}

/// **The reading predicts exactly the names Blender wrote** — `{blend_name}`
/// put in as the file's own name, `standin`, never left raw — and the count is
/// the three files.
#[test]
fn the_reading_predicts_the_names_blender_wrote_for_curly_brace_base_file_names() {
    let composed = composed();
    let WhatTheInventoryIs::It(inventory) = &composed.inventory else {
        panic!("the inventory is composed: {:?}", composed.inventory);
    };
    let mut predicted: Vec<Vec<u8>> = inventory.entries.iter().map(|entry| entry.predicted_path.clone()).collect();
    predicted.sort();
    let written: Vec<Vec<u8>> = WHAT_BLENDER_WROTE.iter().map(|(name, _)| under_the_job(name)).collect();
    assert_eq!(
        predicted.iter().map(|path| String::from_utf8_lossy(path).into_owned()).collect::<Vec<_>>(),
        written.iter().map(|path| String::from_utf8_lossy(path).into_owned()).collect::<Vec<_>>(),
    );
    assert_eq!(inventory.the_count(), 3);
    assert_eq!(
        composed.map.the_folders_the_job_will_write_into(THE_JOB_OUTPUT_FOLDER),
        vec![b"".to_vec(), b"_preview".to_vec()],
        "the job's folder and the nodes' folder, as the file arranges them"
    );
}

/// The listing [`WHAT_BLENDER_WROTE`] as the ending's walk reads it.
fn the_walk() -> Vec<AWalkEntry> {
    WHAT_BLENDER_WROTE
        .iter()
        .map(|(name, size)| AWalkEntry {
            path_under_the_job_output_folder: name.as_bytes().to_vec(),
            kind: WhatKindOfEntry::AFile,
            size: *size,
        })
        .collect()
}

fn said(path: &[u8]) -> String {
    String::from_utf8_lossy(path).into_owned()
}

/// **Over what Blender wrote: frame 1 counts as there, so the ending's
/// catch-up queues it (`Queue & History` P1b) and the finishing message
/// counts it** — the very call `the_telegram_journal`'s `the_frames_at_the_ending`
/// makes. **The preview's main copy is layer *a*'s own picture, `standin -
/// aImage0001.png`, named after *a***: the tree has no Group Output, but a
/// live File Output node is in it, so the compositor runs (B5 narrowed,
/// 2026-10-18) and the walk from *a* — the first layer that renders — reaches
/// *File Output a*, which B7 makes the main output. ***b*'s copy is made from
/// its own node's file**, named after it (B31a's last bullet). **The regular
/// output path's picture is offered after them**: a candidate below *a*'s
/// beauty, and no copy of its own, being fed by both layers.
#[test]
fn over_what_blender_wrote_frame_one_is_there_and_each_layer_has_its_own_copy() {
    let composed = composed();
    let WhatTheInventoryIs::It(inventory) = &composed.inventory else {
        panic!("the inventory is composed: {:?}", composed.inventory);
    };
    let walk = the_walk();
    let statement = the_completeness_statement(inventory, &walk, THE_JOB_OUTPUT_FOLDER);
    assert!(statement.absent.is_empty(), "nothing predicted is missing: {:?}", statement.absent);
    assert_eq!(statement.present.len(), 3);
    let there = the_frames_the_statement_counts_already_there(inventory, &statement, &[1], THE_JOB_OUTPUT_FOLDER);
    assert_eq!(there, vec![1], "frame 1 is there");
    assert_eq!(there.len(), 1, "the finishing message's count: 1 frame");

    // **B13's two picks, made once**: *File Output a*'s `Image` is the main
    // output, and the preview reads its file.
    assert_eq!(
        composed.picks.the_main_output,
        Some(ADestinationTheBeautyReached {
            the_effect: None,
            node: ArtistsText::of(b"File Output a".to_vec()),
            output: Some(ArtistsText::of(b"Image".to_vec())),
        }),
        "B7: the node destination the beauty reaches is the main output"
    );
    assert_eq!(
        composed.picks.the_preview_source.as_deref().map(said),
        Some(said(&under_the_job("_preview/standin - aImage0001.png")))
    );

    let layers = [ArtistsText::of(b"a".to_vec()), ArtistsText::of(b"b".to_vec())];
    let made = what_one_frame_is_made_of(
        1,
        &composed.reading.outputs,
        &composed.inventory,
        &composed.picks,
        &[],
        &layers,
        &[1],
        false,
        &the_size_blender_wrote,
    )
    .expect("frame 1's copy is made");
    assert_eq!(
        said(&made.record.the_source_file),
        said(&under_the_job("_preview/standin - aImage0001.png")),
        "the main copy is layer a's own picture"
    );
    assert!(made.record.it_is_the_main_copy);
    assert_eq!(made.record.the_view_layer_its_picture_came_from, Some(layers[0].clone()), "named after a");
    assert_eq!(made.record.its_place_in_the_frames_set, 0);
    assert_eq!(made.record.where_it_came_from, None, "a layer's name, so no source is recorded");
    // **Each other layer's own copy comes from the node file it alone feeds**,
    // named after it, at its place in the frame's set.
    let copies: Vec<(String, Option<ArtistsText>, u32)> = made
        .the_other_layers
        .iter()
        .map(|other| {
            (
                said(&other.record.the_source_file),
                other.record.the_view_layer_its_picture_came_from.clone(),
                other.record.its_place_in_the_frames_set,
            )
        })
        .collect();
    assert_eq!(
        copies,
        vec![(said(&under_the_job("_preview/standin - bImage0001.png")), Some(layers[1].clone()), 1)],
        "b's copy from b's own file, and no other"
    );

    // **The regular output path's picture is offered after them**: the
    // candidates in the rungs' order put *a*'s beauty first and the regular
    // output path's picture below it.
    let present = the_present_files_at(
        1,
        &composed.reading.outputs,
        inventory,
        WhatTheMainOutputIs::ADestinationInsideAFileOutputNode,
        &the_size_blender_wrote,
    );
    let files: Vec<_> = present.iter().map(|one| one.file.clone()).collect();
    let candidates: Vec<String> = the_candidates(&files, &the_rungs_are_told(TheVersionInHand::of(5, 3), None))
        .iter()
        .map(|candidate| said(&candidate.file.path))
        .collect();
    assert_eq!(candidates.first(), Some(&said(&under_the_job("_preview/standin - aImage0001.png"))));
    let the_output_paths = said(&under_the_job("_preview0001.png"));
    assert!(
        candidates.iter().position(|one| *one == the_output_paths)
            > candidates.iter().position(|one| *one == said(&under_the_job("_preview/standin - aImage0001.png"))),
        "the regular output path's picture is offered after a's: {candidates:?}"
    );
    assert_eq!(candidates.len(), 3, "all three files are candidates: {candidates:?}");

    // **B9's duplicate-picture warning and the frames-and-video line are not
    // raised for this shape**: the beauty reaches one place, and the walk never
    // reaches a final output node.
    let six = the_six_things(&composed.reading.outputs, &composed.map, &composed.inventory, THE_JOB_OUTPUT_FOLDER);
    assert!(!six.the_same_picture_reaches_more_than_one_place);
    assert!(!six.the_render_reaches_frames_and_a_video);
}

/// **A tree with a Group Output behaves as before**: the same stand-in with a
/// live Group Output fed by *a*'s beauty beside *File Output a*. The compositor
/// ran before this piece and runs now; B7 still makes *File Output a*'s
/// `Image` the main output, and the beauty reaching two places is still told
/// as the duplicate picture. With nothing from *a* reaching a File Output
/// node, the walk reaches only the Group Output and the regular output path is
/// the main output, as B4's table has always said.
#[test]
fn a_tree_with_a_group_output_behaves_as_before() {
    let with_a_group_output = |also_into_the_node: bool| {
        let mut document = the_document();
        let outputs = &mut document["outputs"];
        outputs["gating"]["a_live_final_output_node"] = serde_json::json!(true);
        outputs["per_frame"][0]["gating"]["a_live_final_output_node"] = serde_json::json!(true);
        let tree = &mut outputs["the_compositing_tree"];
        tree["nodes"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({"name": b"Group Output".to_vec(), "kind": "TheFinalOutputNode"}));
        let links = tree["links"].as_array_mut().unwrap();
        if !also_into_the_node {
            links.retain(|link| link["from"]["node"] != serde_json::json!(0));
        }
        links.push(serde_json::json!({"from": {"node": 0, "index": 0}, "to": {"node": 4, "index": 0}}));
        composed_from(&document)
    };

    let composed = with_a_group_output(true);
    assert_eq!(
        composed.picks.the_main_output.as_ref().map(|one| said(one.node.as_bytes())),
        Some("File Output a".to_string())
    );
    let six = the_six_things(&composed.reading.outputs, &composed.map, &composed.inventory, THE_JOB_OUTPUT_FOLDER);
    assert!(six.the_same_picture_reaches_more_than_one_place, "File Output a and the output path");
    assert!(!six.the_render_reaches_frames_and_a_video);

    let composed = with_a_group_output(false);
    assert_eq!(composed.picks.the_main_output, None, "the regular output path is no node destination");
    assert_eq!(
        composed.picks.the_preview_source.as_deref().map(said),
        Some(said(&under_the_job("_preview0001.png"))),
        "B4's table: the walk reached the Group Output alone"
    );
    let made = what_one_frame_is_made_of(
        1,
        &composed.reading.outputs,
        &composed.inventory,
        &composed.picks,
        &[],
        &[ArtistsText::of(b"a".to_vec()), ArtistsText::of(b"b".to_vec())],
        &[1],
        false,
        &the_size_blender_wrote,
    )
    .expect("frame 1's copy is made");
    assert_eq!(said(&made.record.the_source_file), said(&under_the_job("_preview0001.png")));
    assert_eq!(made.record.where_it_came_from, Some(WhereThePictureCameFrom::TheOutputPath));
}

/// **The regular output path's folder is read the same way** (I2, added
/// 2026-10-18): the stand-in's regular output path set to
/// `//../Render/{blend_name}/x_` is predicted under `standin/` and handed to
/// Blender as `<job>/standin/x_`, with no brace. The name after the folder is
/// still the one Blender answered — here, as Blender would answer for that
/// path, `x_0001.png`.
#[test]
fn a_regular_output_path_folder_holding_blend_name_is_predicted_and_handed_on_without_braces() {
    let mut document = the_document();
    let outputs = &mut document["outputs"];
    outputs["destinations"][0]["recorded_location"] = serde_json::json!(b"//../Render/{blend_name}/x_".to_vec());
    let answered = format!("{}/../Render/standin/x_0001.png", THE_FOLDER_THE_BLEND_SAT_IN);
    outputs["per_frame"][0]["the_regular_output_paths_answered_path"] = serde_json::json!(answered.as_bytes());
    let composed = composed_from(&document);
    let WhatTheInventoryIs::It(inventory) = &composed.inventory else {
        panic!("the inventory is composed: {:?}", composed.inventory);
    };
    let mut predicted: Vec<String> = inventory.entries.iter().map(|entry| said(&entry.predicted_path)).collect();
    predicted.sort();
    assert_eq!(
        predicted,
        [
            said(&under_the_job("_preview/standin - aImage0001.png")),
            said(&under_the_job("_preview/standin - bImage0001.png")),
            said(&under_the_job("standin/x_0001.png")),
        ]
    );
    let material = compose_the_launch_material(
        &composed.map,
        TheVersionInHand::of(5, 3),
        "a job",
        CarriedAndNotInterpretedHere(serde_json::json!({})),
    );
    assert_eq!(
        material.the_regular_output_path.as_ref().map(|argument| said(&argument.value)),
        Some(said(&under_the_job("standin/x_"))),
        "handed to Blender with the name put in and no brace"
    );
}

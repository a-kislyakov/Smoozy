//! **A File Output node's names with curly braces, predicted and then
//! rendered — proved with real Blenders, kept in the tree and ignored.** The
//! Creator's thirteenth notes, piece P (notes 5, 9 and 10). Run by hand at the
//! desk, one Blender at a time:
//!
//! ```text
//! SMOOZY_DESK_AT=<scratch folder> SMOOZY_DESK_ONLY=53 cargo test -p smoozy-job-files-machine --test names_with_curly_braces_at_the_desk -- --ignored --nocapture
//! SMOOZY_DESK_AT=<scratch folder> SMOOZY_DESK_ONLY=51 cargo test -p smoozy-job-files-machine --test names_with_curly_braces_at_the_desk -- --ignored --nocapture
//! ```
//!
//! **The regular output path's folder with braces** (piece P2, `Job files and
//! outputs` I2's *The regular output path's folder is read the same way*):
//! the same run with `SMOOZY_DESK_REGULAR_OUTPUT_PATH='//../Render/{blend_name}/x_'`
//! makes the stand-in with that regular output path instead of
//! `//../Render/_preview`, and proves the same three things of it.
//!
//! The two Blenders are found at their usual places on this Mac —
//! `/Applications/Blender 5.3b.app` and `/Applications/Blender 5.1.2.app` — or
//! at `SMOOZY_DESK_BLENDER_53` and `SMOOZY_DESK_BLENDER_51`. **This crate's own
//! code starts no process**; this file is a test of it, not part of it.
//!
//! **The file is a stand-in for the Creator's `suzanne.blend`, made here**
//! (his own file is never used): two View Layers, *a* and *b*, each Render
//! Layers node into its own File Output node, base file names
//! `{blend_name} - a` and `{blend_name} - b`, one input `Image` each, both
//! nodes and the regular output path at `//../Render/_preview`, PNG, **no
//! Group Output**, saved one folder below as `standin.blend`. Small, and few
//! samples: this proof is about names, not progress.
//!
//! **What it proves**: the file left alone writes
//! `_preview/standin - aImage0001.png`, `_preview/standin - bImage0001.png`
//! and `_preview0001.png`; Smoozy's own reading of it predicts exactly those
//! three under the job's folder; and the same file rendered through this
//! crate's own redirect writes exactly those three there. Each run's printed
//! reading is kept at `<SMOOZY_DESK_AT>/<version>/printed.json`.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use smoozy_job_files_machine::composing::{
    the_reading_and_everything_derived_from, WhatTheReadingIsComposedAgainst,
};
use smoozy_job_files_machine::inventory::WhatTheInventoryIs;
use smoozy_job_files_machine::main_output::WhereTheRenderEnters;
use smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere;
use smoozy_job_files_machine::reading::{
    ArtistsText, DestinationKind, RecordedLocation, TheInvocation, TheVersionInHand,
    THE_CLOSING_SENTINEL, THE_OPENING_SENTINEL,
};
use smoozy_job_files_machine::reading_script::{the_printed_reading_in, the_reading_script};
use smoozy_job_files_machine::redirect::compose_the_launch_material;

/// **No run of Blender here is waited on past this.**
const ONE_RUN_AT_MOST: Duration = Duration::from_secs(600);

const THE_STAND_IN: &str = r####"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
scene = bpy.context.scene
scene.frame_start = 1
scene.frame_end = 1
scene.render.engine = "CYCLES"
scene.cycles.device = "CPU"
scene.cycles.samples = 4
scene.cycles.use_adaptive_sampling = False
scene.cycles.use_denoising = False
scene.render.resolution_x = 160
scene.render.resolution_y = 90
scene.render.resolution_percentage = 100
scene.render.filepath = sys.argv[sys.argv.index("--") + 2]
scene.render.image_settings.file_format = "PNG"
scene.view_layers[0].name = "a"
scene.view_layers.new("b")
tree = bpy.data.node_groups.new("Compositor", "CompositorNodeTree")
scene.compositing_node_group = tree
for layer in ("a", "b"):
    rl = tree.nodes.new("CompositorNodeRLayers")
    rl.name = "Render Layers " + layer
    rl.scene = scene
    rl.layer = layer
    fo = tree.nodes.new("CompositorNodeOutputFile")
    fo.name = "File Output " + layer
    fo.directory = "//../Render/_preview"
    fo.file_name = "{blend_name} - " + layer
    fo.format.media_type = "IMAGE"
    fo.format.file_format = "PNG"
    fo.file_output_items.clear()
    fo.file_output_items.new("RGBA", "Image")
    tree.links.new(rl.outputs[0], fo.inputs[0])
bpy.ops.wm.save_as_mainfile(filepath=out)
"####;

/// **The redirect's node values, set as the launch material sets them**
/// (`Rendering` L10's `Redirect` instruction: the node looked up by name, the
/// property this crate names, the folder exactly as this crate composed it).
const THE_REDIRECT_APPLIED: &str = r#"
import bpy, os
tree = bpy.context.scene.compositing_node_group
for one in smoozy_redirect:
    node = tree.nodes[os.fsdecode(bytes(one["node"]))]
    setattr(node, one["property"], os.fsdecode(bytes(one["folder"])))
    print("smoozy-desk: set", node.name, one["property"], getattr(node, one["property"]))
"#;

fn the_two() -> Vec<(u16, u16, PathBuf)> {
    [
        (5u16, 3u16, "SMOOZY_DESK_BLENDER_53", "/Applications/Blender 5.3b.app/Contents/MacOS/Blender"),
        (5, 1, "SMOOZY_DESK_BLENDER_51", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
    ]
    .into_iter()
    .filter(|(major, minor, _, _)| {
        std::env::var("SMOOZY_DESK_ONLY").map_or(true, |only| only == format!("{major}{minor}"))
    })
    .map(|(major, minor, named, usual)| {
        (major, minor, PathBuf::from(std::env::var(named).unwrap_or_else(|_| usual.to_string())))
    })
    .collect()
}

/// One run of Blender, waited on until it ends or [`ONE_RUN_AT_MOST`] passes.
fn run(blender: &Path, arguments: &[&str]) -> (Option<i32>, Vec<u8>) {
    let mut child = Command::new(blender)
        .args(arguments)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Blender starts");
    let mut stdout = child.stdout.take().expect("its stream");
    let reader = std::thread::spawn(move || {
        let mut all = Vec::new();
        std::io::Read::read_to_end(&mut stdout, &mut all).expect("the stream reads");
        all
    });
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().expect("it can be asked") {
            break status;
        }
        if started.elapsed() > ONE_RUN_AT_MOST {
            let _ = child.kill();
            let _ = child.wait();
            panic!("{} {arguments:?} ran past {ONE_RUN_AT_MOST:?}", blender.display());
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    (status.code(), reader.join().expect("the reader ends"))
}

/// Every file under a folder, as paths relative to it.
fn every_file_under(folder: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut waiting = vec![folder.to_path_buf()];
    while let Some(at) = waiting.pop() {
        for entry in std::fs::read_dir(&at).expect("the folder reads") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                waiting.push(path);
            } else {
                out.push(path.strip_prefix(folder).unwrap().to_string_lossy().into_owned());
            }
        }
    }
    out.sort();
    out
}

/// What sits between the two marks, as the script printed it.
fn between_the_marks(stream: &[u8]) -> Vec<u8> {
    let find = |haystack: &[u8], needle: &[u8]| {
        haystack.windows(needle.len()).position(|window| window == needle)
    };
    let from = find(stream, THE_OPENING_SENTINEL).expect("the opening mark") + THE_OPENING_SENTINEL.len();
    let to = find(&stream[from..], THE_CLOSING_SENTINEL).expect("the closing mark");
    stream[from..from + to].to_vec()
}

#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta and 5.1.2, one at a time"]
fn the_stand_ins_curly_brace_names_are_predicted_as_blender_writes_them() {
    let at = std::env::var("SMOOZY_DESK_AT").map_or_else(
        |_| std::env::temp_dir().join(format!("smoozy-curly-braces-{}", std::process::id())),
        PathBuf::from,
    );
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_STAND_IN).expect("written");
    let mut faults: Vec<String> = Vec::new();
    let the_regular_output_path =
        std::env::var("SMOOZY_DESK_REGULAR_OUTPUT_PATH").unwrap_or_else(|_| "//../Render/_preview".to_string());
    println!("the stand-in's regular output path: {the_regular_output_path}");

    for (major, minor, blender) in the_two() {
        let named = format!("{major}.{minor}");
        let root = at.join(format!("desk-{major}{minor}"));
        if root.exists() {
            std::fs::remove_dir_all(&root).expect("the last run's folder is cleared");
        }
        let rendering = root.join("rendering");
        std::fs::create_dir_all(&rendering).expect("a folder");
        let file = rendering.join("standin.blend");
        let (made, _) = run(
            &blender,
            &[
                "--factory-startup", "--background", "--python-exit-code", "63", "--python", maker.to_str().unwrap(), "--",
                file.to_str().unwrap(), &the_regular_output_path,
            ],
        );
        assert_eq!(made, Some(0), "{named}: the file is made");

        // **The file left alone**, its own paths untouched, frame 1 by the
        // frame list Smoozy's line uses (`Rendering` L8 item 8).
        let (alone, _) = run(
            &blender,
            &["--factory-startup", "--background", "--python-exit-code", "63", file.to_str().unwrap(), "--render-frame", "1"],
        );
        assert_eq!(alone, Some(0), "{named}: the file renders left alone");
        let left_alone = every_file_under(&root.join("Render"));
        println!("{named}: written left alone under Render/ {left_alone:?}");

        // **Smoozy's own reading**, composed for this version.
        let script = the_reading_script(
            &TheInvocation {
                the_blender_build_path: blender.display().to_string(),
                the_blend_path_on_the_machine: RecordedLocation::of(file.to_str().unwrap().as_bytes().to_vec()),
                the_version_it_will_render_by: TheVersionInHand::of(major, minor),
                the_frames_the_job_covers: vec![1],
                the_check_is_written_as_a_picture: false,
                the_check_is_marked: false,
            },
            None,
        )
        .expect("every name is on record");
        let reading = root.join("reading.py");
        std::fs::write(&reading, script).expect("written");
        let (status, stream) = run(
            &blender,
            &["--factory-startup", "--background", "--python-exit-code", "63", file.to_str().unwrap(), "--python", reading.to_str().unwrap()],
        );
        assert_eq!(status, Some(0), "{named}: the reading ends at success");
        std::fs::write(root.join("printed.json"), between_the_marks(&stream)).expect("kept");
        let printed = the_printed_reading_in(&stream).expect("the document is between its marks");
        for destination in &printed.outputs.destinations {
            if let DestinationKind::AnOutputOfAFileOutputNode { node, the_nodes_base_file_name, replaced, .. } = &destination.kind {
                let said = |text: Option<&ArtistsText>| text.map(|text| String::from_utf8_lossy(text.as_bytes()).into_owned());
                let replaced = replaced.as_ref().expect("the reading replaced the node's text");
                println!(
                    "{named}: {:?} base file name {:?} read as {:?}, folder read as {:?}, output read as {:?}",
                    String::from_utf8_lossy(node.as_bytes()),
                    said(the_nodes_base_file_name.as_ref()),
                    replaced.the_base_file_name.as_ref().map(|one| said(one.the_text()).unwrap_or_else(|| format!("unresolved {:?}", said(one.the_expression())))),
                    said(replaced.the_folder.the_text()),
                    replaced.the_outputs_own_name.as_ref().and_then(|one| said(one.the_text())),
                );
            }
        }

        let job = root.join("job");
        let job_folder = job.to_str().unwrap().to_string();
        let composed = the_reading_and_everything_derived_from(
            printed,
            WhatTheReadingIsComposedAgainst {
                the_folder_the_blend_sits_in: rendering.to_str().unwrap().as_bytes(),
                job_output_folder: &job_folder,
                enters_at: &WhereTheRenderEnters {
                    scene: ArtistsText::of(b"Scene".to_vec()),
                    view_layer: ArtistsText::of(b"a".to_vec()),
                },
                the_jobs_pins: &[],
            },
        )
        .expect("a reading");
        let WhatTheInventoryIs::It(inventory) = &composed.inventory else {
            panic!("{named}: the inventory is composed: {:?}", composed.inventory);
        };
        let mut predicted: Vec<String> = inventory
            .entries
            .iter()
            .map(|entry| {
                let path = String::from_utf8_lossy(&entry.predicted_path).into_owned();
                path.strip_prefix(&format!("{job_folder}/")).unwrap_or(&path).to_string()
            })
            .collect();
        predicted.sort();
        println!("{named}: predicted under the job's folder {predicted:?}");

        // **Through Smoozy's own redirect**: the launch material this crate
        // composes from the map, its node values set by name, and the regular
        // output path by its own argument (D10).
        let version = TheVersionInHand::of(major, minor);
        let material = compose_the_launch_material(&composed.map, version, "desk", CarriedAndNotInterpretedHere(serde_json::json!({})));
        let nodes: Vec<serde_json::Value> = material
            .assignments
            .iter()
            .filter_map(|assignment| {
                let node = match &assignment.destination.kind {
                    DestinationKind::AnOutputOfAFileOutputNode { node, .. }
                    | DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { node, .. } => node,
                    _ => return None,
                };
                Some(serde_json::json!({"node": node.as_bytes(), "property": assignment.property.as_ref()?, "folder": assignment.folder}))
            })
            .collect();
        let redirect = root.join("redirect.py");
        std::fs::write(&redirect, format!("smoozy_redirect = {}\n{THE_REDIRECT_APPLIED}", serde_json::Value::Array(nodes)))
            .expect("written");
        let regular = String::from_utf8(
            material.the_regular_output_path.as_ref().expect("the stand-in has a regular output path").value.clone(),
        )
        .expect("text");
        println!("{named}: the regular output path handed to Blender {regular:?}");
        let (rendered, stream) = run(
            &blender,
            &[
                "--factory-startup", "--background", "--python-exit-code", "63", file.to_str().unwrap(),
                "--render-output", &regular, "--python", redirect.to_str().unwrap(), "--render-frame", "1",
            ],
        );
        for line in String::from_utf8_lossy(&stream).lines() {
            if line.starts_with("smoozy-desk:") {
                println!("{named}: {line}");
            }
        }
        assert_eq!(rendered, Some(0), "{named}: the file renders through the redirect");
        let landed = every_file_under(&job);
        println!("{named}: written through the redirect under the job's folder {landed:?}");

        if predicted != landed {
            faults.push(format!("{named}: predicted {predicted:?}, written {landed:?}"));
        }
        if landed != left_alone {
            faults.push(format!("{named}: through the redirect {landed:?}, left alone {left_alone:?}"));
        }
        if inventory.the_count() != 3 {
            faults.push(format!("{named}: the count is {}", inventory.the_count()));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

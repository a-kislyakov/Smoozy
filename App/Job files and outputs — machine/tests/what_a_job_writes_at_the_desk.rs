//! **What a job writes, predicted and then rendered — proved with real
//! Blenders, kept in the tree and ignored.** The Creator's second notes on the
//! test copy, piece 1 (notes 14 and 18). Run by hand at the desk:
//!
//! ```text
//! cargo test -p smoozy-job-files-machine --test what_a_job_writes_at_the_desk -- --ignored --nocapture
//! ```
//!
//! Each of the four Blenders is found at its usual place on this Mac, or at
//! `SMOOZY_DESK_BLENDER_45`, `_50`, `_51` and `_52`. **This crate's own code
//! starts no process**; this file is a test of it, not part of it.
//!
//! **The file is a stand-in for the Creator's own, made here** (the Creator's
//! own file is never used): a video at `//../Render/Animation`, a PNG and a
//! multi-layer EXR File Output node for each of two view layers with `### - `
//! names under `//../Render/`, two more view layers (*Box AO*, *Mattress AO*)
//! feeding two more PNG outputs, the bake output path left as a fresh file
//! records it, and the file saved one folder below.
//!
//! **What it proves, in every version**: the reading of that file, composed
//! with both AO layers pinned off, predicts the job's folders as *the job
//! folder, PNG, EXR*, a count of five, and exactly the names Blender then
//! writes when the same file renders frame 1 **through this crate's own
//! redirect** (the launch material's node values and the regular output
//! path's argument) with those two layers switched off — read off the disk,
//! not off this crate. **And those names are the names the same file writes
//! left alone**, its own paths untouched: Smoozy moves where files go, never
//! what they are called. Added 2026-10-04 by piece 1b, which found 4.5's
//! gathering node turning the artist's file name into a folder (D8).
//!
//! **Every render takes the Creator's test shape** (piece 1b's brief): Cycles,
//! 3840×2160, 1000 samples, the noise threshold off, on this Mac's graphics
//! card. `SMOOZY_DESK_ONLY=45` (or `50`, `51`, `52`) runs one version alone.

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
    ArtistsText, AViewLayerTheJobPins, DestinationKind, RecordedLocation, TheInvocation,
    TheVersionInHand,
};
use smoozy_job_files_machine::redirect::compose_the_launch_material;
use smoozy_job_files_machine::reading_script::{the_printed_reading_in, the_reading_script};

/// **No run of Blender here is waited on past this** — one 4K frame of two
/// view layers at 1000 samples, on the slowest of the four.
const ONE_RUN_AT_MOST: Duration = Duration::from_secs(1800);

const THE_STAND_IN: &str = r####"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
major = bpy.app.version[0]
scene = bpy.context.scene
scene.frame_start = 1
scene.frame_end = 1
scene.render.engine = "CYCLES"
scene.render.resolution_x = 3840
scene.render.resolution_y = 2160
scene.render.resolution_percentage = 100
scene.cycles.samples = 1000
scene.cycles.use_adaptive_sampling = False
scene.cycles.device = "GPU"
scene.render.filepath = "//../Render/Animation"
if major >= 5:
    scene.render.image_settings.media_type = "VIDEO"
scene.render.image_settings.file_format = "FFMPEG"
scene.view_layers[0].name = "Box"
for name in ("Mattress", "Box AO", "Mattress AO"):
    scene.view_layers.new(name)
if major >= 5:
    tree = bpy.data.node_groups.new("Compositor", "CompositorNodeTree")
    scene.compositing_node_group = tree
else:
    scene.use_nodes = True
    tree = scene.node_tree
    for n in list(tree.nodes):
        tree.nodes.remove(n)
def a_node(name, folder, exr, item, base):
    fo = tree.nodes.new("CompositorNodeOutputFile")
    fo.name = name
    if major >= 5:
        fo.directory = folder
        fo.file_name = base
        fo.format.media_type = "MULTI_LAYER_IMAGE" if exr else "IMAGE"
        fo.format.file_format = "OPEN_EXR_MULTILAYER" if exr else "PNG"
        fo.file_output_items.clear()
        fo.file_output_items.new("RGBA", item)
    else:
        fo.base_path = folder + ("/" + base if base else "")
        fo.format.file_format = "OPEN_EXR_MULTILAYER" if exr else "PNG"
        if exr:
            fo.layer_slots.clear()
            fo.layer_slots.new(item)
        else:
            fo.file_slots.clear()
            fo.file_slots.new(item)
    return fo
for layer in ("Box", "Mattress", "Box AO", "Mattress AO"):
    rl = tree.nodes.new("CompositorNodeRLayers")
    rl.name = "RL " + layer
    rl.scene = scene
    rl.layer = layer
    png = a_node("PNG " + layer, "//../Render/PNG", False, "### - Hero - " + layer, "")
    tree.links.new(rl.outputs[0], png.inputs[0])
    if "AO" not in layer:
        exr = a_node("EXR " + layer, "//../Render/EXR", True, "Image", "### - Hero - " + layer)
        tree.links.new(rl.outputs[0], exr.inputs[0])
bpy.ops.wm.save_as_mainfile(filepath=out)
"####;

/// **The two layers the job pins off, as the render's own launch material
/// would** (`Render Settings` V4), **and this Mac's graphics card**, which a
/// factory start does not remember. Run before every render, Smoozy's and the
/// file's own alike.
const THE_PINS_AND_THE_CARD: &str = r#"
import bpy
scene = bpy.context.scene
for name in ("Box AO", "Mattress AO"):
    scene.view_layers[name].use = False
cycles = bpy.context.preferences.addons["cycles"].preferences
cycles.compute_device_type = "METAL"
cycles.get_devices()
for device in cycles.devices:
    device.use = device.type == "METAL"
scene.cycles.device = "GPU"
"#;

/// **The redirect's node values, set as the launch material sets them**
/// (`Rendering` L10's `Redirect` instruction: the node looked up by name, the
/// property this crate names, the folder exactly as this crate composed it).
/// The values ride in `smoozy_redirect`, written above this text.
const THE_REDIRECT_APPLIED: &str = r#"
import bpy, os
scene = bpy.context.scene
tree = scene.compositing_node_group if bpy.app.version[0] >= 5 else scene.node_tree
for one in smoozy_redirect:
    node = tree.nodes[os.fsdecode(bytes(one["node"]))]
    setattr(node, one["property"], os.fsdecode(bytes(one["folder"])))
    print("smoozy-desk: set", node.name, one["property"], getattr(node, one["property"]))
"#;

fn the_four() -> Vec<(u16, u16, PathBuf)> {
    [
        (4u16, 5u16, "SMOOZY_DESK_BLENDER_45", "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"),
        (5, 0, "SMOOZY_DESK_BLENDER_50", "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"),
        (5, 1, "SMOOZY_DESK_BLENDER_51", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
        (5, 2, "SMOOZY_DESK_BLENDER_52", "/Applications/Blender.app/Contents/MacOS/Blender"),
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

#[test]
#[ignore = "run by hand at the desk with the four Blenders"]
fn the_stand_in_predicts_the_job_folder_png_exr_and_five_files_and_blender_writes_those_five_under_their_own_names() {
    let at = std::env::temp_dir().join(format!("smoozy-what-a-job-writes-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_STAND_IN).expect("written");
    let pins = at.join("pins.py");
    std::fs::write(&pins, THE_PINS_AND_THE_CARD).expect("written");
    let mut faults: Vec<String> = Vec::new();

    for (major, minor, blender) in the_four() {
        let named = format!("{major}.{minor}");
        let root = at.join(format!("{major}{minor}"));
        let rendering = root.join("rendering");
        std::fs::create_dir_all(&rendering).expect("a folder");
        let file = rendering.join("standin.blend");
        let (made, _) = run(
            &blender,
            &[
                "--factory-startup",
                "--background",
                "--python-exit-code",
                "63",
                "--python",
                maker.to_str().unwrap(),
                "--",
                file.to_str().unwrap(),
            ],
        );
        assert_eq!(made, Some(0), "{named}: the file is made");

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
            &[
                "--factory-startup",
                "--background",
                "--python-exit-code",
                "63",
                file.to_str().unwrap(),
                "--python",
                reading.to_str().unwrap(),
            ],
        );
        assert_eq!(status, Some(0), "{named}: the reading ends at success");
        let printed = the_printed_reading_in(&stream).expect("the document is between its marks");

        let job = root.join("job");
        let job_folder = job.to_str().unwrap().to_string();
        let enters_at = WhereTheRenderEnters {
            scene: ArtistsText::of(b"Scene".to_vec()),
            view_layer: ArtistsText::of(b"Box".to_vec()),
        };
        let the_pins: Vec<AViewLayerTheJobPins> = ["Box AO", "Mattress AO"]
            .into_iter()
            .map(|layer| AViewLayerTheJobPins {
                layer: ArtistsText::of(layer.as_bytes().to_vec()),
                on: false,
            })
            .collect();
        let composed = the_reading_and_everything_derived_from(
            printed,
            WhatTheReadingIsComposedAgainst {
                the_folder_the_blend_sits_in: rendering.to_str().unwrap().as_bytes(),
                job_output_folder: &job_folder,
                enters_at: &enters_at,
                the_jobs_pins: &the_pins,
            },
        )
        .expect("a reading");

        let folders: Vec<String> = composed
            .map
            .the_folders_the_job_will_write_into(&job_folder)
            .iter()
            .map(|folder| String::from_utf8_lossy(folder).into_owned())
            .collect();
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
        println!("{named}: folders {folders:?}");
        println!("{named}: count {}", inventory.the_count());
        println!("{named}: predicted {predicted:?}");

        // **Through Smoozy's own redirect**: the launch material this crate
        // composes from the map, its node values set by name as `Rendering`
        // sets them, and the regular output path by its own argument (D10).
        let version = TheVersionInHand::of(major, minor);
        let material = compose_the_launch_material(
            &composed.map,
            version,
            "desk",
            CarriedAndNotInterpretedHere(serde_json::json!({})),
        );
        let nodes: Vec<serde_json::Value> = material
            .assignments
            .iter()
            .filter_map(|assignment| {
                let node = match &assignment.destination.kind {
                    DestinationKind::AnOutputOfAFileOutputNode { node, .. }
                    | DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { node, .. } => node,
                    _ => return None,
                };
                Some(serde_json::json!({
                    "node": node.as_bytes(),
                    "property": assignment.property.as_ref()?,
                    "folder": assignment.folder,
                }))
            })
            .collect();
        let redirect = root.join("redirect.py");
        std::fs::write(
            &redirect,
            format!(
                "smoozy_redirect = {}\n{THE_REDIRECT_APPLIED}",
                serde_json::Value::Array(nodes)
            ),
        )
        .expect("written");
        let regular = String::from_utf8(
            material
                .the_regular_output_path
                .as_ref()
                .expect("the stand-in has a regular output path")
                .value
                .clone(),
        )
        .expect("text");
        let (rendered, stream) = run(
            &blender,
            &[
                "--factory-startup",
                "--background",
                "--python-exit-code",
                "63",
                file.to_str().unwrap(),
                "-o",
                &regular,
                "--python",
                pins.to_str().unwrap(),
                "--python",
                redirect.to_str().unwrap(),
                "-a",
            ],
        );
        for line in String::from_utf8_lossy(&stream).lines() {
            if line.starts_with("smoozy-desk:") {
                println!("{named}: {line}");
            }
        }
        assert_eq!(rendered, Some(0), "{named}: the file renders through the redirect");
        let landed = every_file_under(&job);
        println!("{named}: landed through the redirect {landed:?}");

        // **The same file left alone**, its own paths untouched, the same two
        // layers off: everything lands under `<root>/Render`, the run the
        // render's own destinations share, so its names relative to that
        // folder are the names the artist's own render would write there.
        let (alone, _) = run(
            &blender,
            &[
                "--factory-startup",
                "--background",
                "--python-exit-code",
                "63",
                file.to_str().unwrap(),
                "--python",
                pins.to_str().unwrap(),
                "-a",
            ],
        );
        assert_eq!(alone, Some(0), "{named}: the file renders left alone");
        let left_alone = every_file_under(&root.join("Render"));
        println!("{named}: landed left alone {left_alone:?}");

        // **Every version alike**, 4.5 included since piece 1b: the job
        // folder, PNG and EXR, and the names the artist gave.
        let the_folders_owed = vec!["", "PNG", "EXR"];
        if folders != the_folders_owed {
            faults.push(format!("{named}: the folders shown are {folders:?}"));
        }
        if inventory.the_count() != 5 {
            faults.push(format!("{named}: the count is {}", inventory.the_count()));
        }
        if predicted != landed {
            faults.push(format!("{named}: predicted {predicted:?}, written {landed:?}"));
        }
        if landed != left_alone {
            faults.push(format!(
                "{named}: through the redirect {landed:?}, left alone {left_alone:?}"
            ));
        }
        // The frames are large; each version's are gone before the next.
        std::fs::remove_dir_all(&root).expect("the version's folder is removed");
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **`Job files and outputs` P20 on this Mac's own disk** — a stat and the
/// plain make, as the render worker's are on a rented machine.
struct TheDesksOwnFolders;

impl smoozy_job_files_machine::where_the_folder_sits::MakesFolders for TheDesksOwnFolders {
    fn what_stands_at(&mut self, path: &str) -> smoozy_job_files_machine::where_the_folder_sits::WhatStands {
        use smoozy_job_files_machine::where_the_folder_sits::WhatStands;
        match std::fs::metadata(path) {
            Ok(found) if found.is_dir() => WhatStands::AFolder,
            Ok(_) => WhatStands::SomethingElse,
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => WhatStands::Nothing,
            Err(_) => WhatStands::SomethingElse,
        }
    }
    fn make_one_folder(&mut self, path: &str) -> Result<(), ()> {
        std::fs::create_dir(path).map_err(|_| ())
    }
}

/// **`Job files and outputs` §6.6 at the desk, with the frames landing** —
/// the Creator's second notes, piece 4 (note 18's want). The stand-in above,
/// saved in a stand-in for `/home/ubuntu/rendering`, is read with this
/// crate's own reading script; **the job's folder is chosen by this crate's
/// own choice** (`Job files and outputs` P18 to `Job files and outputs` P23),
/// made on this Mac's disk, and the map composed under it; then frame 1
/// renders through this crate's own redirect, AO pinned off, in the Creator's
/// test shape (4K, 1000 samples, the noise threshold off, this Mac's card) —
/// and **what landed is listed under the folder where the file points**:
/// `<home>/Render/<job>/`, the video at its top, `PNG/` and `EXR/` beneath.
/// `SMOOZY_DESK_ONLY` picks the version (5.2 where unset); the job's name is
/// N3's, a stand-in identity and the label *Kitchen*.
#[test]
#[ignore = "run by hand at the desk with a real Blender"]
fn the_jobs_folder_is_made_where_the_file_points_and_the_frames_land_in_it() {
    use smoozy_job_files_machine::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
    use smoozy_job_files_machine::where_the_folder_sits::{
        the_folder_is_chosen_and_made, the_run_of, WhatTheChoiceIsMadeFrom,
    };
    // **The build folder's own temporary area, not the system's**: a root is
    // chosen under this folder below, and `Job files and outputs` L3 refuses
    // one under `/tmp`, which is the system's temporary folder on Linux.
    let at = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("smoozy-where-the-folder-sits-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_STAND_IN).expect("written");
    let pins = at.join("pins.py");
    std::fs::write(&pins, THE_PINS_AND_THE_CARD).expect("written");
    let only = std::env::var("SMOOZY_DESK_ONLY").unwrap_or_else(|_| "52".to_string());
    let (major, minor, blender) = the_four()
        .into_iter()
        .find(|(major, minor, _)| format!("{major}{minor}") == only)
        .expect("the version asked for");
    let home = at.join("home/ubuntu");
    let rendering = home.join("rendering");
    std::fs::create_dir_all(&rendering).expect("a folder");
    let file = rendering.join("standin.blend");
    let (made, _) = run(
        &blender,
        &["--factory-startup", "--background", "--python-exit-code", "63", "--python", maker.to_str().unwrap(), "--", file.to_str().unwrap()],
    );
    assert_eq!(made, Some(0), "the file is made");
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
    let reading = at.join("reading.py");
    std::fs::write(&reading, script).expect("written");
    let (status, stream) = run(
        &blender,
        &["--factory-startup", "--background", "--python-exit-code", "63", file.to_str().unwrap(), "--python", reading.to_str().unwrap()],
    );
    assert_eq!(status, Some(0), "the reading ends at success");
    let printed = the_printed_reading_in(&stream).expect("the document is between its marks");

    // **Smoozy's own root is elsewhere on the machine**, as on a rental.
    let layout = TheLayout::of(
        MachineRoot::chosen(at.join("ephemeral/smoozy").to_str().unwrap()).expect("a root"),
        WhereThePackageAreaSits::UnderTheRoot,
    );
    let job = smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::Job).expect("randomness");
    let name = smoozy_job_files_machine::names::job_output_folder_name(&job, Some("Kitchen"));
    let enters_at = WhereTheRenderEnters {
        scene: ArtistsText::of(b"Scene".to_vec()),
        view_layer: ArtistsText::of(b"Box".to_vec()),
    };
    let the_pins: Vec<AViewLayerTheJobPins> = ["Box AO", "Mattress AO"]
        .into_iter()
        .map(|layer| AViewLayerTheJobPins { layer: ArtistsText::of(layer.as_bytes().to_vec()), on: false })
        .collect();
    let destinations =
        smoozy_job_files_machine::reading_script::the_destinations_that_will_be_written(&printed.outputs);
    let run_of = the_run_of(&destinations, rendering.to_str().unwrap().as_bytes(), printed.outputs.taken_by);
    println!("the run: {run_of:?}");
    let sits = the_folder_is_chosen_and_made(
        &WhatTheChoiceIsMadeFrom {
            the_run: &run_of,
            layout: &layout,
            the_job_output_folders_name: &name,
            the_servers_fallback_folder: None,
        },
        &mut |_| true,
        &mut TheDesksOwnFolders,
    )
    .expect("a folder");
    println!("where the job's folder sits: {sits:?}");
    let composed = the_reading_and_everything_derived_from(
        printed,
        WhatTheReadingIsComposedAgainst {
            the_folder_the_blend_sits_in: rendering.to_str().unwrap().as_bytes(),
            job_output_folder: &sits.path,
            enters_at: &enters_at,
            the_jobs_pins: &the_pins,
        },
    )
    .expect("a reading");
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
    let redirect = at.join("redirect.py");
    std::fs::write(&redirect, format!("smoozy_redirect = {}\n{THE_REDIRECT_APPLIED}", serde_json::Value::Array(nodes))).expect("written");
    let regular = String::from_utf8(material.the_regular_output_path.as_ref().expect("a regular output path").value.clone()).expect("text");
    let began = Instant::now();
    let (rendered, _) = run(
        &blender,
        &["--factory-startup", "--background", "--python-exit-code", "63", file.to_str().unwrap(), "-o", &regular, "--python", pins.to_str().unwrap(), "--python", redirect.to_str().unwrap(), "-a"],
    );
    println!("rendered in {:?}", began.elapsed());
    assert_eq!(rendered, Some(0), "the file renders through the redirect");
    let landed = every_file_under(Path::new(&sits.path));
    println!("landed under {}: {landed:?}", sits.path);
    println!("Smoozy's own job area holds: {:?}", std::fs::read_dir(layout.area(smoozy_job_files_machine::layout::Area::Job)).map(|read| read.count()).ok());
    let expected = home.join("Render").join(&name);
    std::fs::remove_dir_all(&at).expect("the folder is removed");
    assert_eq!(sits.path, expected.to_str().unwrap());
    assert_eq!(sits.where_it_is, smoozy_job_files_machine::where_the_folder_sits::WhereItIs::WhereTheFilePoints);
    assert_eq!(landed.len(), 5, "{landed:?}");
    assert!(landed.iter().any(|one| one.starts_with("PNG/001 - Hero - Box")), "{landed:?}");
    assert!(landed.iter().any(|one| one.starts_with("EXR/001 - Hero - Box")), "{landed:?}");
}

//! **The reading moves only the scenes that move — proved with real Blenders,
//! kept in the tree and ignored.** The Creator's notes on 0.1.8, piece 2. Run
//! by hand at the desk:
//!
//! ```text
//! cargo test -p smoozy-job-files-machine --test stepping_at_the_desk -- --ignored --nocapture
//! ```
//!
//! Each of the four Blenders is found at its usual place on this Mac, or at
//! `SMOOZY_DESK_BLENDER_45`, `_50`, `_51` and `_52`. **This crate's own code
//! starts no process**; this file is a test of it, not part of it.
//!
//! **What it proves**, in every version: one file with four scenes is read by
//! the script this crate composes, with a frame-change handler of the test's
//! own loaded before the file that prints a line each time a scene is moved to
//! a frame.
//!
//! - **A scene whose view-layer switch is keyed is moved**, and its table
//!   reads the switch off and on again across its frames.
//! - **A scene with nothing animated is never moved**, and its table still
//!   covers every frame of its span with the value it holds.
//! - **A scene whose markers name cameras is moved**, though it carries no
//!   animation at all: a marker changes the camera at its frame.
//! - **A scene whose compositing tree keys a node's value is not moved**
//!   (since 2026-11-13, the Creator's notes on the test copy, second round,
//!   note 15): the reading never reads a node input's value. Until then any
//!   keyed tree moved its scene. (A node's mute cannot be keyed — 4.5 refuses
//!   it, *"property "mute" not animatable"*.)

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use smoozy_job_files_machine::reading::{RecordedLocation, TheInvocation, TheVersionInHand};
use smoozy_job_files_machine::reading_script::{the_printed_reading_in, the_reading_script};

/// **No run of Blender here is waited on past this.**
const ONE_RUN_AT_MOST: Duration = Duration::from_secs(300);

const THE_FILE_MADE: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
bpy.ops.wm.read_factory_settings(use_empty=False)
moves = bpy.context.scene
moves.name = "Moves"
moves.frame_start = 1
moves.frame_end = 12
second = moves.view_layers.new("Second")
second.use = True
second.keyframe_insert("use", frame=1)
second.use = False
second.keyframe_insert("use", frame=6)
second.use = True
second.keyframe_insert("use", frame=10)
still = bpy.data.scenes.new("Still")
still.frame_start = 1
still.frame_end = 12
still.view_layers.new("Other")
marked = bpy.data.scenes.new("Marked")
marked.frame_start = 1
marked.frame_end = 6
for name, frame in (("A", 1), ("B", 4)):
    camera = bpy.data.objects.new(name, bpy.data.cameras.new(name))
    marked.collection.objects.link(camera)
    marker = marked.timeline_markers.new(name, frame=frame)
    marker.camera = camera
keyed = bpy.data.scenes.new("Tree")
keyed.frame_start = 1
keyed.frame_end = 6
if bpy.app.version[:2] == (4, 5):
    keyed.use_nodes = True
    tree = keyed.node_tree
else:
    tree = bpy.data.node_groups.new("Compositor", "CompositorNodeTree")
    keyed.compositing_node_group = tree
blur = tree.nodes.new("CompositorNodeBlur")
values = [socket for socket in blur.inputs if socket.type in ("VALUE", "INT", "VECTOR")]
if values:
    values[0].keyframe_insert("default_value", frame=1)
else:
    blur.keyframe_insert("size_x", frame=1)
assert keyed.animation_data is None and tree.animation_data is not None
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

const THE_STEPS_COUNTED: &str = r#"
import bpy
from bpy.app.handlers import persistent

@persistent
def smoozy_desk_counts_a_step(scene, *_):
    print("SMOOZY-DESK-STEPPED", scene.name, flush=True)

bpy.app.handlers.frame_change_post.append(smoozy_desk_counts_a_step)
"#;

fn the_four() -> Vec<(u16, u16, PathBuf)> {
    [
        (4u16, 5u16, "SMOOZY_DESK_BLENDER_45", "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"),
        (5, 0, "SMOOZY_DESK_BLENDER_50", "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"),
        (5, 1, "SMOOZY_DESK_BLENDER_51", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
        (5, 2, "SMOOZY_DESK_BLENDER_52", "/Applications/Blender 5.2.4.app/Contents/MacOS/Blender"),
    ]
    .into_iter()
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

#[test]
#[ignore = "run by hand at the desk with the four Blenders"]
fn the_reading_moves_only_the_scenes_that_move_in_all_four() {
    let at = std::env::temp_dir().join(format!("smoozy-stepping-at-the-desk-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_FILE_MADE).expect("written");
    let counter = at.join("count.py");
    std::fs::write(&counter, THE_STEPS_COUNTED).expect("written");

    for (major, minor, blender) in the_four() {
        let named = format!("{major}.{minor}");
        let file = at.join(format!("four-scenes-{major}{minor}.blend"));
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
                the_frames_the_job_covers: Vec::new(),
                the_check_is_written_as_a_picture: false,
                the_check_is_marked: false,
            },
            None,
        )
        .expect("every name is on record");
        let reading = at.join(format!("reading-{major}{minor}.py"));
        std::fs::write(&reading, script).expect("written");

        let (status, stream) = run(
            &blender,
            &[
                "--factory-startup",
                "--background",
                "--python-exit-code",
                "63",
                "--python",
                counter.to_str().unwrap(),
                file.to_str().unwrap(),
                "--python",
                reading.to_str().unwrap(),
            ],
        );
        assert_eq!(status, Some(0), "{named}: the reading ends at success");
        let printed = the_printed_reading_in(&stream).expect("the document is between its marks");
        let text = String::from_utf8_lossy(&stream);
        let stepped = |scene: &str| {
            text.lines()
                .filter(|line| *line == format!("SMOOZY-DESK-STEPPED {scene}"))
                .count()
        };
        let table = |scene: &str, layer: &str| -> Vec<(i64, bool)> {
            let scenes = printed.settings.0["scenes"].as_array().expect("the scenes");
            let one = scenes.iter().find(|s| s["name"] == scene).expect("the scene is read");
            let layers = one["view_layers"].as_array().expect("its layers");
            let it = layers.iter().find(|l| l["name"] == layer).expect("the layer is read");
            it["renders_at"]
                .as_array()
                .expect("its table")
                .iter()
                .map(|row| (row[0].as_i64().unwrap(), row[1].as_bool().unwrap()))
                .collect()
        };

        // **The keyed switch: moved, and its table moves with it.**
        assert!(stepped("Moves") > 0, "{named}: the scene with a keyed switch is moved");
        let second = table("Moves", "Second");
        assert!(second.contains(&(1, true)), "{named}: {second:?}");
        assert!(second.contains(&(6, false)), "{named}: {second:?}");
        assert!(second.contains(&(10, true)), "{named}: {second:?}");

        // **Nothing animated: never moved, and the table still covers the span.**
        assert_eq!(stepped("Still"), 0, "{named}: the still scene is never moved");
        let other = table("Still", "Other");
        assert_eq!(
            other,
            (1..=12).map(|frame| (frame, true)).collect::<Vec<_>>(),
            "{named}: the value read once stands for every frame"
        );

        // **Markers that name cameras: moved, with no animation on the scene.**
        assert!(stepped("Marked") > 0, "{named}: a scene whose markers name cameras is moved");

        // **A compositing tree keyed on a node's value: not moved since
        // 2026-11-13** (the Creator's notes on the test copy, second round,
        // note 15). The reading never reads a node input's value; a node's
        // mute, which it does read, cannot be keyed. Before, it was moved.
        assert_eq!(stepped("Tree"), 0, "{named}: a tree keyed on a value the reading never reads moves nothing");
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

/// **The 38's piece 10, item 25: the reading names the scene it read**, in all
/// four Blenders. A file of three scenes is saved showing *Interior*, the
/// second of them; read with no scene named, the printed reading names
/// *Interior* — the scene its outputs half was read from — and not the file's
/// first scene. Run by hand at the desk:
///
/// ```text
/// cargo test -p smoozy-job-files-machine --test stepping_at_the_desk the_reading_names -- --ignored --nocapture
/// ```
///
/// **What it does not prove**: a file saved from Blender's interface. This
/// file is saved by a Blender with no window, as every file the research
/// tried was (`Rendering` L16's untested half).
const THE_FILE_OF_THREE_SCENES: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
bpy.ops.wm.read_factory_settings(use_empty=False)
first = bpy.context.scene
first.name = "Exterior"
interior = bpy.data.scenes.new("Interior")
interior.view_layers.new("ViewLayer")
kitchen = bpy.data.scenes.new("Kitchen")
if bpy.context.window is not None:
    bpy.context.window.scene = interior
else:
    for window in bpy.context.window_manager.windows:
        window.scene = interior
print("SMOOZY-DESK-SHOWING", bpy.context.scene.name, flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

#[test]
#[ignore = "run by hand at the desk with the four Blenders"]
fn the_reading_names_the_scene_it_read_in_all_four() {
    let at = std::env::temp_dir().join(format!("smoozy-scene-it-read-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_FILE_OF_THREE_SCENES).expect("written");
    for (major, minor, blender) in the_four() {
        let named = format!("{major}.{minor}");
        let file = at.join(format!("three-scenes-{major}{minor}.blend"));
        let (made, said) = run(
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
        println!("{named}: {}", String::from_utf8_lossy(&said).lines().find(|line| line.starts_with("SMOOZY-DESK-SHOWING")).unwrap_or("no showing line"));
        let script = the_reading_script(
            &TheInvocation {
                the_blender_build_path: blender.display().to_string(),
                the_blend_path_on_the_machine: RecordedLocation::of(file.to_str().unwrap().as_bytes().to_vec()),
                the_version_it_will_render_by: TheVersionInHand::of(major, minor),
                the_frames_the_job_covers: Vec::new(),
                the_check_is_written_as_a_picture: false,
                the_check_is_marked: false,
            },
            None,
        )
        .expect("every name is on record");
        let reading = at.join(format!("reading-{major}{minor}.py"));
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
        let read = printed.the_scene_it_read.as_ref().map(|name| name.as_bytes().to_vec());
        println!("{named}: the reading names {:?}", read.as_deref().map(String::from_utf8_lossy));
        assert_eq!(read.as_deref(), Some(&b"Interior"[..]), "{named}: the scene it read");
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

/// **The 38's piece 3, `Job files and outputs` B25: the reading carries the
/// job's own view transform**, in all four Blenders. A file whose scene is set
/// to *Standard* — not the factory scene's own — is read by the script this
/// crate composes, and the printed reading's outputs half names *Standard*,
/// which is the half the manifest carries. Run by hand at the desk:
///
/// ```text
/// cargo test -p smoozy-job-files-machine --test stepping_at_the_desk the_reading_carries -- --ignored --nocapture
/// ```
const THE_FILE_WITH_ITS_OWN_TREATMENT: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
bpy.ops.wm.read_factory_settings(use_empty=False)
bpy.context.scene.view_settings.view_transform = "Standard"
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

#[test]
#[ignore = "run by hand at the desk with the four Blenders"]
fn the_reading_carries_the_jobs_own_view_transform_in_all_four() {
    let at = std::env::temp_dir().join(format!("smoozy-view-transform-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_FILE_WITH_ITS_OWN_TREATMENT).expect("written");
    for (major, minor, blender) in the_four() {
        let named = format!("{major}.{minor}");
        let file = at.join(format!("standard-{major}{minor}.blend"));
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
                the_frames_the_job_covers: Vec::new(),
                the_check_is_written_as_a_picture: false,
                the_check_is_marked: false,
            },
            None,
        )
        .expect("every name is on record");
        let reading = at.join(format!("reading-{major}{minor}.py"));
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
        let read = printed.outputs.the_view_transform.as_ref().map(|name| name.as_bytes().to_vec());
        println!("{named}: the reading carries {:?}", read.as_deref().map(String::from_utf8_lossy));
        assert_eq!(read.as_deref(), Some(&b"Standard"[..]), "{named}: the job's own view transform");
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

/// **`Job files and outputs` B26's multi-layer route, from a real reading**:
/// a file whose regular output path writes multi-layer OpenEXR with two view
/// layers is read by this crate's own script in each of the four; its
/// destination records `OPEN_EXR_MULTILAYER` (from 5.0 through the separate
/// media setting), is told apart as multi-layer, and its picture inside is the
/// first view layer's `Combined`, which the record carries. Added 2026-10-03,
/// a picture for a multi-layer OpenEXR job.
///
/// ```text
/// cargo test -p smoozy-job-files-machine --test stepping_at_the_desk a_multi_layer -- --ignored --nocapture
/// ```
const THE_FILE_WRITING_MULTI_LAYER: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
bpy.ops.wm.read_factory_settings(use_empty=False)
scene = bpy.context.scene
settings = scene.render.image_settings
if hasattr(settings, "media_type"):
    settings.media_type = "MULTI_LAYER_IMAGE"
else:
    settings.file_format = "OPEN_EXR_MULTILAYER"
scene.view_layers.new("Second")
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

#[test]
#[ignore = "run by hand at the desk with the four Blenders"]
fn a_multi_layer_regular_output_is_read_as_one_and_named_by_its_first_view_layer_in_all_four() {
    use smoozy_job_files_machine::derivatives::{
        how_it_is_made, it_writes_multi_layer_files, the_derivatives_record,
        APresentFileAndItsDestination, TheTreatment, THE_MULTI_LAYER_FORMAT,
    };
    use smoozy_job_files_machine::main_output::APresentFile;
    use smoozy_job_files_machine::reading::{ArtistsText, DestinationKind};
    let at = std::env::temp_dir().join(format!("smoozy-multi-layer-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_FILE_WRITING_MULTI_LAYER).expect("written");
    for (major, minor, blender) in the_four() {
        let named = format!("{major}.{minor}");
        let file = at.join(format!("multi-{major}{minor}.blend"));
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
                the_frames_the_job_covers: Vec::new(),
                the_check_is_written_as_a_picture: false,
                the_check_is_marked: false,
            },
            None,
        )
        .expect("every name is on record");
        let reading = at.join(format!("reading-{major}{minor}.py"));
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
        let regular = printed
            .outputs
            .destinations
            .iter()
            .find(|one| one.kind == DestinationKind::TheRegularOutputPath)
            .expect("the regular output path");
        let format = regular
            .format
            .as_ref()
            .map(|format| format.as_the_file_records_it.as_bytes().to_vec())
            .unwrap_or_default();
        println!(
            "{named}: recorded {:?}, fed by {:?}",
            String::from_utf8_lossy(&format),
            regular.the_view_layers_that_feed_it
        );
        assert_eq!(format, THE_MULTI_LAYER_FORMAT, "{named}");
        assert!(it_writes_multi_layer_files(&regular.kind, &format), "{named}");
        let chosen = APresentFileAndItsDestination {
            file: APresentFile {
                path: b"/j/out/0001.exr".to_vec(),
                frame: Some(1),
                the_picture_it_holds: Some(ArtistsText::of(b"Combined".to_vec())),
                format: format.clone(),
                bytes: 10,
                is_video_shaped: false,
                it_sits_in_the_main_outputs_folder: true,
            },
            multi_layer: true,
            the_destination_it_came_from: smoozy_job_files_machine::main_output::WhereThePictureCameFrom::TheOutputPath,
            the_view_layers_that_feed_it: regular.the_view_layers_that_feed_it.clone(),
        };
        let frame_layers: Vec<ArtistsText> = printed
            .outputs
            .per_frame
            .first()
            .map(|at| at.view_layers_that_render.clone())
            .unwrap_or_default();
        let (treatment, picture) = how_it_is_made(
            &chosen,
            printed.outputs.the_view_transform.as_ref(),
            &frame_layers,
        )
        .expect("made");
        assert_eq!(treatment, TheTreatment::TheJobsOwnViewTransform(b"AgX".to_vec()), "{named}");
        assert_eq!(
            picture.map(|name| name.as_bytes().to_vec()),
            Some(b"ViewLayer.Combined".to_vec()),
            "{named}"
        );
        let record = the_derivatives_record(&chosen, 1, &frame_layers);
        assert_eq!(
            record.the_view_layer_its_picture_came_from.map(|name| name.as_bytes().to_vec()),
            Some(b"ViewLayer".to_vec()),
            "{named}: two view layers render, so the record names the one"
        );
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

/// **`Job files and outputs` §14 item 30, for R4b: a scene's half read beside
/// every other scene's in one opening is the same as a reading naming that
/// scene alone** — in all four Blenders, on a file of six scenes that each
/// write something different. The Creator's seventh notes, note 1.
/// Run by hand at the desk:
///
/// ```text
/// cargo test -p smoozy-job-files-machine --test stepping_at_the_desk every_scene -- --ignored --nocapture
/// ```
///
/// The file is read once by the pre-load reading, which names no scene, and
/// then once for each scene by the reading that names it, with no frames
/// handed in — the only difference between the two being the naming. **Each
/// scene's half, made into the reading naming it, must print byte for byte
/// what the reading naming it printed**, and so must the scene the file was
/// saved showing, whose half is the reading's own.
const THE_FILE_OF_SIX_SCENES: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
major = bpy.app.version[0]
bpy.ops.wm.read_factory_settings(use_empty=False)


def a_tree(scene):
    if major >= 5:
        tree = bpy.data.node_groups.new("Compositor " + scene.name, "CompositorNodeTree")
        scene.compositing_node_group = tree
    else:
        scene.use_nodes = True
        tree = scene.node_tree
        for n in list(tree.nodes):
            tree.nodes.remove(n)
    return tree


def a_node(tree, name, folder, exr, item, base):
    fo = tree.nodes.new("CompositorNodeOutputFile")
    fo.name = name
    if major >= 5:
        fo.directory = folder
        fo.file_name = base
        fo.format.media_type = "MULTI_LAYER_IMAGE" if exr else "IMAGE"
        fo.format.file_format = "OPEN_EXR_MULTILAYER" if exr else "JPEG"
        fo.file_output_items.clear()
        fo.file_output_items.new("RGBA", item)
    else:
        fo.base_path = folder + ("/" + base if base else "")
        fo.format.file_format = "OPEN_EXR_MULTILAYER" if exr else "JPEG"
        if exr:
            fo.layer_slots.clear()
            fo.layer_slots.new(item)
        else:
            fo.file_slots.clear()
            fo.file_slots.new(item)
    return fo


saved = bpy.context.scene
saved.name = "Saved"
saved.frame_start = 1
saved.frame_end = 4
saved.render.filepath = "//saved/beauty_####"
saved.render.image_settings.file_format = "PNG"

exr = bpy.data.scenes.new("Exr")
exr.frame_start = 1
exr.frame_end = 8
exr.render.filepath = "//exr/beauty_####"
exr.render.image_settings.file_format = "OPEN_EXR"
exr.view_layers.new("Fg")
second = exr.view_layers.new("Keyed")
second.use = True
second.keyframe_insert("use", frame=1)
second.use = False
second.keyframe_insert("use", frame=5)
tree = a_tree(exr)
for layer in ("ViewLayer", "Fg"):
    if layer not in [one.name for one in exr.view_layers]:
        continue
    rl = tree.nodes.new("CompositorNodeRLayers")
    rl.scene = exr
    rl.layer = layer
    jpg = a_node(tree, "JPG " + layer, "//exr/jpg", False, layer + "_####", "")
    tree.links.new(rl.outputs[0], jpg.inputs[0])
    multi = a_node(tree, "Multi " + layer, "//exr/multi", True, "Image", layer + "_####")
    tree.links.new(rl.outputs[0], multi.inputs[0])

edit = bpy.data.scenes.new("Edit")
edit.frame_start = 1
edit.frame_end = 3
edit.render.filepath = "//edit/cut_####"
edit.render.image_settings.file_format = "TIFF"
editor = edit.sequence_editor_create()
holder = editor.strips if hasattr(editor, "strips") else editor.sequences
holder.new_scene("to_exr", exr, 1, 1)
edit.render.use_sequencer = True
for name, frame in (("A", 1), ("B", 2)):
    camera = bpy.data.objects.new(name, bpy.data.cameras.new(name))
    edit.collection.objects.link(camera)
    edit.timeline_markers.new(name, frame=frame).camera = camera

stereo = bpy.data.scenes.new("Stereo")
stereo.frame_start = 2
stereo.frame_end = 3
stereo.render.filepath = "//stereo/eye_####"
stereo.render.image_settings.file_format = "BMP"
try:
    stereo.render.use_multiview = True
    stereo.render.views_format = "STEREO_3D"
    stereo.render.image_settings.views_format = "INDIVIDUAL"
except Exception as why:
    print("SMOOZY-DESK-NO-STEREO", why, flush=True)

shared = bpy.data.scenes.new("Shared")
shared.render.filepath = "//shared/s_####"
shared.render.image_settings.file_format = "TARGA"
if major >= 5:
    shared.compositing_node_group = tree
else:
    own = a_tree(shared)
    rl = own.nodes.new("CompositorNodeRLayers")
    rl.scene = shared
    muted = a_node(own, "Muted", "//shared/muted", False, "m_####", "")
    own.links.new(rl.outputs[0], muted.inputs[0])
    muted.mute = True

sim = bpy.data.scenes.new("Sim")
sim.render.filepath = "//sim/s_####"
try:
    with bpy.context.temp_override(scene=sim):
        bpy.ops.rigidbody.world_add()
    print("SMOOZY-DESK-RIGID", sim.rigidbody_world is not None, flush=True)
except Exception as why:
    print("SMOOZY-DESK-NO-RIGID", why, flush=True)

if bpy.context.window is not None:
    bpy.context.window.scene = saved
else:
    for window in bpy.context.window_manager.windows:
        window.scene = saved
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

#[test]
#[ignore = "run by hand at the desk with the four Blenders"]
fn every_scene_read_in_one_opening_is_the_same_as_a_reading_naming_it_in_all_four() {
    use smoozy_job_files_machine::reading::ArtistsText;
    use smoozy_job_files_machine::reading_script::{the_pre_load_reading_script, TheOtherScenes};

    let at = std::env::temp_dir().join(format!("smoozy-every-scene-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_FILE_OF_SIX_SCENES).expect("written");
    let the_scenes = ["Saved", "Exr", "Edit", "Stereo", "Shared", "Sim"];
    for (major, minor, blender) in the_four() {
        let named = format!("{major}.{minor}");
        let file = at.join(format!("six-scenes-{major}{minor}.blend"));
        let (made, said) = run(
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
        for line in String::from_utf8_lossy(&said).lines().filter(|line| line.starts_with("SMOOZY-DESK")) {
            println!("{named}: {line}");
        }
        let invocation = TheInvocation {
            the_blender_build_path: blender.display().to_string(),
            the_blend_path_on_the_machine: RecordedLocation::of(file.to_str().unwrap().as_bytes().to_vec()),
            the_version_it_will_render_by: TheVersionInHand::of(major, minor),
            the_frames_the_job_covers: Vec::new(),
            the_check_is_written_as_a_picture: false,
            the_check_is_marked: false,
        };
        let read_with = |script: String, label: &str| -> Vec<u8> {
            let path = at.join(format!("reading-{major}{minor}-{label}.py"));
            std::fs::write(&path, script).expect("written");
            let (status, stream) = run(
                &blender,
                &[
                    "--factory-startup",
                    "--background",
                    "--python-exit-code",
                    "63",
                    file.to_str().unwrap(),
                    "--python",
                    path.to_str().unwrap(),
                ],
            );
            assert_eq!(status, Some(0), "{named} {label}: the reading ends at success");
            stream
        };

        let stream = read_with(the_pre_load_reading_script(&invocation).expect("every name is on record"), "every");
        let every = the_printed_reading_in(&stream).expect("the document is between its marks");
        let others = TheOtherScenes::in_the_stream(&stream).the_other_scenes.expect("R4b: the other scenes' halves");
        assert_eq!(every.the_scene_it_read.as_ref().map(|name| name.as_bytes().to_vec()), Some(b"Saved".to_vec()));
        assert_eq!(
            others.iter().map(|half| String::from_utf8_lossy(half.scene.as_bytes()).into_owned()).collect::<std::collections::BTreeSet<_>>(),
            the_scenes[1..].iter().map(|name| name.to_string()).collect::<std::collections::BTreeSet<_>>(),
            "{named}: every other scene, once"
        );
        assert_eq!(others.len(), the_scenes.len() - 1, "{named}: each once");

        let mut kinds = std::collections::BTreeSet::new();
        for scene in the_scenes {
            let one = read_with(
                the_reading_script(&invocation, Some(&ArtistsText::of(scene.as_bytes().to_vec())))
                    .expect("every name is on record"),
                scene,
            );
            let alone = the_printed_reading_in(&one).expect("the document is between its marks");
            assert!(
                TheOtherScenes::in_the_stream(&one).the_other_scenes.is_none(),
                "{named} {scene}: a reading naming its scene holds only that scene"
            );
            let beside = if scene == "Saved" {
                every.clone()
            } else {
                let half = others.iter().find(|half| half.scene.as_bytes() == scene.as_bytes()).expect("its half");
                assert!(half.errors.is_empty(), "{named} {scene}: {:?}", half.errors);
                every.as_a_reading_naming(half).expect("its half was read")
            };
            let alone_bytes = serde_json::to_vec(&alone).expect("it renders");
            let beside_bytes = serde_json::to_vec(&beside).expect("it renders");
            let outputs = serde_json::to_vec(&alone.outputs).expect("it renders");
            kinds.insert(outputs.clone());
            println!(
                "{named} {scene}: {} destinations, {} frames, {} bytes alone, identical: {}",
                alone.outputs.destinations.len(),
                alone.outputs.per_frame.len(),
                alone_bytes.len(),
                alone_bytes == beside_bytes
            );
            assert!(alone_bytes == beside_bytes, "{named} {scene}: the half read beside the others differs from the reading naming it");
        }
        assert_eq!(kinds.len(), the_scenes.len(), "{named}: every scene writes something different");
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

/// **The Creator's notes on the test copy, second round, note 15: a scene
/// whose keys reach nothing the reading reads is not moved, and the reading
/// is the same as when every scene is moved** — in the four Blenders and 5.3.
/// Run by hand at the desk:
///
/// ```text
/// cargo test -p smoozy-job-files-machine --test stepping_at_the_desk keys_that_reach_nothing -- --ignored --nocapture
/// ```
///
/// One file of seven scenes over frames 1 to 12. **Not moved**: a scene with
/// nothing animated; one whose sequencer strip's opacity is keyed; one whose
/// view exposure is keyed; one carrying animation data with nothing in it; one
/// whose compositing tree keys a node's value. **Moved, as before**: one whose
/// Cycles sample count is keyed (the reading reads it), and one whose strip's
/// opacity is driven (what a driver resolves to is not in the research).
///
/// **The reading is the same both ways**: the document between the marks is
/// compared byte for byte against a reading of the same file by the same
/// script with every scene moved at every frame of its span, which is what
/// the reading did before 2026-10-03.
const THE_FILE_OF_KEYS_THAT_REACH_NOTHING: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
bpy.ops.wm.read_factory_settings(use_empty=False)
first = bpy.context.scene
first.name = "Plain"
first.frame_start = 1
first.frame_end = 12

def a_scene(name):
    scene = bpy.data.scenes.new(name)
    scene.frame_start = 1
    scene.frame_end = 12
    scene.view_layers.new("Layer")
    return scene

def a_strip(scene):
    editor = scene.sequence_editor_create()
    strips = getattr(editor, "strips", None)
    if strips is None:
        strips = editor.sequences
    try:
        return strips.new_effect(name="Fade", type="COLOR", channel=1, frame_start=1, length=12)
    except TypeError:
        return strips.new_effect(name="Fade", type="COLOR", channel=1, frame_start=1, frame_end=13)

strips = a_scene("Strips")
fade = a_strip(strips)
fade.blend_alpha = 1.0
fade.keyframe_insert("blend_alpha", frame=1)
fade.blend_alpha = 0.0
fade.keyframe_insert("blend_alpha", frame=12)

exposure = a_scene("Exposure")
exposure.view_settings.exposure = 0.0
exposure.view_settings.keyframe_insert("exposure", frame=1)
exposure.view_settings.exposure = 2.0
exposure.view_settings.keyframe_insert("exposure", frame=12)

empty = a_scene("Empty")
empty.animation_data_create()

tree_scene = a_scene("TreeValue")
if bpy.app.version[:2] == (4, 5):
    tree_scene.use_nodes = True
    tree = tree_scene.node_tree
else:
    tree = bpy.data.node_groups.new("Compositor", "CompositorNodeTree")
    tree_scene.compositing_node_group = tree
blur = tree.nodes.new("CompositorNodeBlur")
values = [socket for socket in blur.inputs if socket.type in ("VALUE", "INT", "VECTOR")]
if values:
    values[0].keyframe_insert("default_value", frame=1)
else:
    blur.keyframe_insert("size_x", frame=1)

samples = a_scene("Samples")
samples.cycles.samples = 16
samples.cycles.keyframe_insert("samples", frame=1)
samples.cycles.samples = 64
samples.cycles.keyframe_insert("samples", frame=12)

driven = a_scene("Driven")
dimmed = a_strip(driven)
curve = dimmed.driver_add("blend_alpha")
curve.driver.type = "AVERAGE"

for scene in bpy.data.scenes:
    data = scene.animation_data
    paths = []
    if data is not None and data.action is not None:
        action = data.action
        for one in getattr(action, "fcurves", ()):
            paths.append(one.data_path)
        for layer in getattr(action, "layers", ()):
            for strip in layer.strips:
                for bag in getattr(strip, "channelbags", ()):
                    paths.extend(one.data_path for one in bag.fcurves)
    print("SMOOZY-DESK-PATHS", scene.name, sorted(set(paths)), flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

fn the_four_and_five_three() -> Vec<(u16, u16, PathBuf)> {
    let mut all = the_four();
    all.push((
        5,
        3,
        PathBuf::from(
            std::env::var("SMOOZY_DESK_BLENDER_53")
                .unwrap_or_else(|_| "/Applications/Blender 5.3.app/Contents/MacOS/Blender".to_string()),
        ),
    ));
    all
}

/// What sits between the document's two marks, byte for byte.
fn between_the_marks(stream: &[u8]) -> Vec<u8> {
    use smoozy_job_files_machine::reading::{THE_CLOSING_SENTINEL, THE_OPENING_SENTINEL};
    let find = |hay: &[u8], needle: &[u8]| hay.windows(needle.len()).position(|w| w == needle);
    let opens = find(stream, THE_OPENING_SENTINEL).expect("the opening mark") + THE_OPENING_SENTINEL.len();
    let closes = find(&stream[opens..], THE_CLOSING_SENTINEL).expect("the closing mark");
    stream[opens..opens + closes].to_vec()
}

/// The same script with every scene moved at every frame of its span.
fn every_scene_moved(script: &str) -> String {
    let mut moved = script.to_string();
    for rule in ["    moves = the_walk_must_step(scene)\n", "    moves = something_on_it_moves(scene)\n"] {
        moved = moved.replace(rule, "    moves = True\n");
    }
    assert!(moved != script, "the walk's rule is where the test looks for it");
    // Moved at every frame: the reading off the curves (2026-11-13) is taken
    // out too, or a scene of plain keys would be moved once per run.
    todays_walk(&moved)
}

/// Reads `file` with `script` and the step counter loaded before it; gives
/// back the stream.
fn read_counting_steps(blender: &Path, at: &Path, counter: &Path, file: &Path, script: &str, called: &str) -> Vec<u8> {
    let reading = at.join(called);
    std::fs::write(&reading, script).expect("written");
    let (status, stream) = run(
        blender,
        &[
            "--factory-startup",
            "--background",
            "--python-exit-code",
            "63",
            "--python",
            counter.to_str().unwrap(),
            file.to_str().unwrap(),
            "--python",
            reading.to_str().unwrap(),
        ],
    );
    assert_eq!(status, Some(0), "{called}: the reading ends at success");
    stream
}

fn steps_of(stream: &[u8], scene: &str) -> usize {
    String::from_utf8_lossy(stream)
        .lines()
        .filter(|line| *line == format!("SMOOZY-DESK-STEPPED {scene}"))
        .count()
}

/// Makes the file of keys that reach nothing in one Blender, and reads it
/// both ways; gives back (the narrowed stream, the every-scene stream).
fn the_file_of_keys_read_both_ways(at: &Path, major: u16, minor: u16, blender: &Path) -> (Vec<u8>, Vec<u8>) {
    let named = format!("{major}.{minor}");
    let maker = at.join("make-keys.py");
    std::fs::write(&maker, THE_FILE_OF_KEYS_THAT_REACH_NOTHING).expect("written");
    let counter = at.join("count.py");
    std::fs::write(&counter, THE_STEPS_COUNTED).expect("written");
    let file = at.join(format!("keys-{major}{minor}.blend"));
    let (made, said) = run(
        blender,
        &["--factory-startup", "--background", "--python-exit-code", "63", "--python", maker.to_str().unwrap(), "--", file.to_str().unwrap()],
    );
    for line in String::from_utf8_lossy(&said).lines().filter(|line| line.starts_with("SMOOZY-DESK-PATHS")) {
        println!("{named}: {line}");
    }
    assert_eq!(made, Some(0), "{named}: the file is made");
    let script = the_reading_script(
        &TheInvocation {
            the_blender_build_path: blender.display().to_string(),
            the_blend_path_on_the_machine: RecordedLocation::of(file.to_str().unwrap().as_bytes().to_vec()),
            the_version_it_will_render_by: TheVersionInHand::of(major, minor),
            the_frames_the_job_covers: Vec::new(),
            the_check_is_written_as_a_picture: false,
            the_check_is_marked: false,
        },
        None,
    )
    .expect("every name is on record");
    let narrowed = read_counting_steps(blender, at, &counter, &file, &script, &format!("narrowed-{major}{minor}.py"));
    let every = read_counting_steps(blender, at, &counter, &file, &every_scene_moved(&script), &format!("every-{major}{minor}.py"));
    (narrowed, every)
}

#[test]
#[ignore = "run by hand at the desk with the four Blenders and 5.3"]
fn keys_that_reach_nothing_the_reading_reads_move_no_scene_and_change_no_answer() {
    let at = std::env::temp_dir().join(format!("smoozy-keys-that-reach-nothing-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    for (major, minor, blender) in the_four_and_five_three() {
        let named = format!("{major}.{minor}");
        let (narrowed, every) = the_file_of_keys_read_both_ways(&at, major, minor, &blender);
        for scene in ["Plain", "Strips", "Exposure", "Empty", "TreeValue", "Samples", "Driven"] {
            println!("{named}: {scene} moved {} times, {} with every scene moved", steps_of(&narrowed, scene), steps_of(&every, scene));
            assert!(steps_of(&every, scene) > 0, "{named}: {scene} is moved when every scene is");
        }
        for scene in ["Plain", "Strips", "Exposure", "Empty", "TreeValue"] {
            assert_eq!(steps_of(&narrowed, scene), 0, "{named}: {scene}'s keys reach nothing the reading reads, so it is not moved");
        }
        for scene in ["Samples", "Driven"] {
            assert!(steps_of(&narrowed, scene) > 0, "{named}: {scene} is still moved");
        }
        assert!(
            between_the_marks(&narrowed) == between_the_marks(&every),
            "{named}: the reading is the same as with every scene moved"
        );
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

/// **`Render Settings` Q5's build detail of 2026-11-13: the walk says where it
/// is.** The same file: two scenes are moved, twelve frames each, so the
/// walk's last line says 24 of 24; a reading that moves nothing writes none.
#[test]
#[ignore = "run by hand at the desk with the four Blenders and 5.3"]
fn the_walk_says_where_it_is_in_all_five() {
    use smoozy_job_files_machine::reading::{the_walks_last_step_in, THE_WALKS_MARK};
    let at = std::env::temp_dir().join(format!("smoozy-the-walk-says-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    for (major, minor, blender) in the_four_and_five_three() {
        let named = format!("{major}.{minor}");
        let (narrowed, every) = the_file_of_keys_read_both_ways(&at, major, minor, &blender);
        let lines = narrowed.windows(THE_WALKS_MARK.len()).filter(|w| *w == THE_WALKS_MARK).count();
        println!("{named}: {lines} step lines, the last {:?}; every scene moved: the last {:?}", the_walks_last_step_in(&narrowed), the_walks_last_step_in(&every));
        assert_eq!(the_walks_last_step_in(&narrowed), Some((24, 24)), "{named}");
        assert!(lines >= 2, "{named}: the first step and the last");
        assert_eq!(the_walks_last_step_in(&every), Some((84, 84)), "{named}: seven scenes of twelve frames");
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

/// **`Job files and outputs` R4 item 5, read off the curves (2026-11-13, the
/// Blender research *Reading at a frame*): a scene whose animation is plain
/// keys is read off its curves, moved once per run of frames with the same
/// key, and the reading is the same as today's walk** — in the four Blenders
/// and 5.3. Run by hand at the desk, one Blender at a time:
///
/// ```text
/// cargo test -p smoozy-job-files-machine --test stepping_at_the_desk read_off_its_curves -- --ignored --nocapture --test-threads=1
/// ```
///
/// One file of ten scenes over frames 1 to 24, saved by Blender's own save.
/// **Read off the curves**: a view-layer switch keyed on, off, on (three
/// runs, so three moves); Cycles' sample count and a view layer's own sample
/// count keyed (read only after the walk, so one move); markers naming two
/// cameras with a marker naming none between them, the output path naming
/// the camera from 5.0 (two runs); a muted switch (one move); nothing
/// animated (never moved). **Walked as before, every frame**: a driven
/// sample count, an NLA track, Time Stretching 200 to 100, and two markers on
/// one frame naming different cameras. Each file is read twice by each
/// reading — the pre-load reading of every scene, and the render-start
/// reading naming *Markers* with frames 3, 15 and 30 — once as built and once
/// with the reading off the curves taken out, which is today's walk; the
/// documents between the marks must be byte-identical. Each move is counted
/// by the test's own wrap of the script's one step, so no frame-change
/// handler is loaded (one would make every scene walk).
const THE_FILE_READ_OFF_ITS_CURVES: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
bpy.ops.wm.read_factory_settings(use_empty=False)


def frames(scene):
    scene.frame_start = 1
    scene.frame_end = 24
    return scene


def a_scene(name):
    scene = frames(bpy.data.scenes.new(name))
    scene.view_layers.new("Second")
    return scene


def cameras(scene, marks):
    for name, frame in marks:
        camera = bpy.data.objects.new(scene.name + name, bpy.data.cameras.new(scene.name + name))
        scene.collection.objects.link(camera)
        scene.timeline_markers.new(name, frame=frame).camera = camera


def the_curves_of(scene):
    action = scene.animation_data.action
    curves = list(getattr(action, "fcurves", ()))
    for layer in getattr(action, "layers", ()):
        for strip in layer.strips:
            for bag in getattr(strip, "channelbags", ()):
                curves.extend(bag.fcurves)
    return curves


def switched(scene):
    layer = scene.view_layers["Second"]
    for value, frame in ((True, 1), (False, 8), (True, 16)):
        layer.use = value
        layer.keyframe_insert("use", frame=frame)


first = frames(bpy.context.scene)
first.name = "Switch"
first.view_layers.new("Second")
switched(first)

samples = a_scene("Samples")
samples.cycles.samples = 16
samples.cycles.keyframe_insert("samples", frame=1)
samples.cycles.samples = 64
samples.cycles.keyframe_insert("samples", frame=24)
layer = samples.view_layers["Second"]
layer.samples = 4
layer.keyframe_insert("samples", frame=1)
layer.samples = 40
layer.keyframe_insert("samples", frame=24)

markers = a_scene("Markers")
cameras(markers, (("A", 5), ("B", 15)))
markers.timeline_markers.new("Note", frame=10)
markers.render.filepath = "//markers/{camera_name}_####" if bpy.app.version >= (5, 0, 0) else "//markers/m_####"

muted = a_scene("Muted")
switched(muted)
for curve in the_curves_of(muted):
    curve.mute = True

a_scene("Plain")

driven = a_scene("Driven")
driven.cycles.driver_add("samples").driver.type = "AVERAGE"

stacked = a_scene("Stacked")
stacked.cycles.samples = 8
stacked.cycles.keyframe_insert("samples", frame=1)
stacked.animation_data.nla_tracks.new()

stretched = a_scene("Stretched")
switched(stretched)
stretched.render.frame_map_old = 200
stretched.render.frame_map_new = 100

tie = a_scene("Tie")
cameras(tie, (("A", 5), ("B", 5)))

for scene in bpy.data.scenes:
    data = scene.animation_data
    paths = []
    if data is not None and data.action is not None:
        paths = sorted(set((one.data_path, one.mute) for one in the_curves_of(scene)))
    print("SMOOZY-DESK-PATHS", scene.name, paths, [(m.name, m.frame, m.camera.name if m.camera else None) for m in scene.timeline_markers], flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

/// The script as built with the reading off the curves taken out: today's
/// walk. A script that has no reading off the curves is today's walk already.
fn todays_walk(script: &str) -> String {
    script.replace(
        "    stands = the_frames_the_scene_stands_at(scene, span) if moves else None\n",
        "    stands = None\n",
    )
}

/// The script with its one step wrapped, so each move prints the scene's name.
fn counting_each_move(script: &str) -> String {
    let wrapped = script.replace("scene.frame_set(", "smoozy_desk_step(scene, ");
    assert!(wrapped != script, "the script's one step is where the test looks for it");
    wrapped.replacen(
        "import bpy, json, os, sys, time\n",
        "import bpy, json, os, sys, time\n\n\ndef smoozy_desk_step(scene, frame):\n    print(\"\\nSMOOZY-DESK-STEPPED\", scene.name, flush=True)\n    scene.frame_set(frame)\n\n",
        1,
    )
}

/// Reads `file` with `script`, no handler loaded; gives back the stream and
/// how long the whole run took.
fn read_with_no_handler(blender: &Path, at: &Path, file: &Path, script: &str, called: &str) -> (Vec<u8>, Duration) {
    let reading = at.join(called);
    std::fs::write(&reading, script).expect("written");
    let started = Instant::now();
    let (status, stream) = run(
        blender,
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
    let took = started.elapsed();
    assert_eq!(status, Some(0), "{called}: the reading ends at success");
    (stream, took)
}

#[test]
#[ignore = "run by hand at the desk with the four Blenders and 5.3"]
fn a_scene_read_off_its_curves_reads_as_the_walk_does_in_all_five() {
    use smoozy_job_files_machine::reading::{the_walks_last_step_in, ArtistsText};
    use smoozy_job_files_machine::reading_script::the_pre_load_reading_script;
    let at = std::env::temp_dir().join(format!("smoozy-read-off-its-curves-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_FILE_READ_OFF_ITS_CURVES).expect("written");
    for (major, minor, blender) in the_four_and_five_three() {
        let named = format!("{major}.{minor}");
        let file = at.join(format!("curves-{major}{minor}.blend"));
        let (made, said) = run(
            &blender,
            &["--factory-startup", "--background", "--python-exit-code", "63", "--python", maker.to_str().unwrap(), "--", file.to_str().unwrap()],
        );
        for line in String::from_utf8_lossy(&said).lines().filter(|line| line.starts_with("SMOOZY-DESK")) {
            println!("{named}: {line}");
        }
        assert_eq!(made, Some(0), "{named}: the file is made");
        let invocation = |frames: Vec<i64>| TheInvocation {
            the_blender_build_path: blender.display().to_string(),
            the_blend_path_on_the_machine: RecordedLocation::of(file.to_str().unwrap().as_bytes().to_vec()),
            the_version_it_will_render_by: TheVersionInHand::of(major, minor),
            the_frames_the_job_covers: frames,
            the_check_is_written_as_a_picture: false,
            the_check_is_marked: false,
        };
        let readings = [
            ("pre-load", the_pre_load_reading_script(&invocation(Vec::new())).expect("every name is on record")),
            (
                "render-start",
                the_reading_script(&invocation(vec![3, 15, 30]), Some(&ArtistsText::of(b"Markers".to_vec())))
                    .expect("every name is on record"),
            ),
        ];
        for (label, script) in readings {
            let (read, read_took) = read_with_no_handler(&blender, &at, &file, &counting_each_move(&script), &format!("read-{label}-{major}{minor}.py"));
            let (walked, walk_took) = read_with_no_handler(&blender, &at, &file, &counting_each_move(&todays_walk(&script)), &format!("walk-{label}-{major}{minor}.py"));
            let scenes = ["Switch", "Samples", "Markers", "Muted", "Plain", "Driven", "Stacked", "Stretched", "Tie"];
            for scene in scenes {
                println!("{named} {label}: {scene} moved {} times, {} by today's walk", steps_of(&read, scene), steps_of(&walked, scene));
            }
            println!(
                "{named} {label}: the walk's last line {:?}, today's {:?}; {:.2} s against {:.2} s",
                the_walks_last_step_in(&read),
                the_walks_last_step_in(&walked),
                read_took.as_secs_f64(),
                walk_took.as_secs_f64()
            );
            let identical = between_the_marks(&read) == between_the_marks(&walked);
            println!("{named} {label}: identical: {identical}");
            assert!(identical, "{named} {label}: the reading off the curves differs from today's walk");
            assert_eq!(steps_of(&read, "Plain"), 0, "{named} {label}: nothing animated, never moved");
            if label == "render-start" {
                // **The camera reaches the reading**: from 5.0 the output path
                // names the camera, so each frame's name is the camera the
                // markers give it, read off the curves as by the walk.
                let printed = the_printed_reading_in(&read).expect("the document is between its marks");
                let names: Vec<(i64, String)> = printed
                    .outputs
                    .per_frame
                    .iter()
                    .map(|at| {
                        let path = at.the_regular_output_paths_answered_path.as_ref().map(|one| one.as_bytes().to_vec()).unwrap_or_default();
                        let path = String::from_utf8_lossy(&path).into_owned();
                        (at.frame, path.rsplit('/').next().unwrap_or_default().to_string())
                    })
                    .collect();
                println!("{named} {label}: the names at the job's frames {names:?}");
                if major >= 5 {
                    assert!(names.iter().any(|(frame, name)| *frame == 3 && name.starts_with("MarkersA")), "{named}: {names:?}");
                    assert!(names.iter().any(|(frame, name)| *frame == 15 && name.starts_with("MarkersB")), "{named}: {names:?}");
                    assert!(names.iter().any(|(frame, name)| *frame == 30 && name.starts_with("MarkersB")), "{named}: {names:?}");
                }
            }
            if label == "pre-load" {
                assert_eq!(steps_of(&read, "Switch"), 3, "{named}: a keyed switch is moved once per run");
                assert_eq!(steps_of(&read, "Samples"), 1, "{named}: keys read only after the walk move once");
                assert_eq!(steps_of(&read, "Markers"), 2, "{named}: markers move once per camera");
                assert_eq!(steps_of(&read, "Muted"), 1, "{named}: a muted curve moves nothing");
                // A span is the scene's frames, 1 to 24, and where a view
                // layer's switch is keyed the frames beside its keys: 0 too.
                let span = |scene: &str| if ["Switch", "Muted", "Stretched"].contains(&scene) { 25 } else { 24 };
                for scene in ["Switch", "Samples", "Markers", "Muted"] {
                    assert_eq!(steps_of(&walked, scene), span(scene), "{named}: today's walk moves {scene} at every frame");
                }
                // **From 2026-11-14 the driven scene's driver is followed
                // without a move** (`Job files and outputs` R4 item 5, drivers
                // and cheap moves): an Averaged Value driver with no variable
                // gives the same at every frame, so the scene is moved once.
                assert_eq!(steps_of(&read, "Driven"), 1, "{named}: a driver with nothing to follow moves once");
                assert_eq!(steps_of(&walked, "Driven"), span("Driven"), "{named}: today's walk moves Driven at every frame");
                for scene in ["Stacked", "Stretched", "Tie"] {
                    assert_eq!(steps_of(&read, scene), span(scene), "{named}: {scene} is walked as before");
                    assert_eq!(steps_of(&walked, scene), span(scene), "{named}: {scene} is walked as before");
                }
                assert_eq!(the_walks_last_step_in(&read), Some((73, 73)), "{named}: only walked scenes count");
                assert_eq!(the_walks_last_step_in(&walked), Some((195, 195)), "{named}: today's walk counts all eight");
            }
        }
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

/// **The same, timed, on a file heavy enough that each move costs what a
/// production scene's does**: three geometry-node grids of 2.25 million
/// points each, a noise's scale keyed on their node tree so every move works
/// them out again, and the scene's Cycles sample count keyed over 30 frames. The pre-load reading as built
/// against today's walk, whole runs timed from start to Blender quitting, the
/// two documents byte-identical. (Timed until 2026-11-14; since then every
/// move leaves the grids out, and the times are printed, not compared.) Run
/// by hand at the desk, one Blender at a time:
///
/// ```text
/// cargo test -p smoozy-job-files-machine --test stepping_at_the_desk a_heavy_file_read_off -- --ignored --nocapture --test-threads=1
/// ```
const THE_HEAVY_FILE: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
bpy.ops.wm.read_factory_settings(use_empty=False)
scene = bpy.context.scene
scene.frame_start = 1
scene.frame_end = 30
scene.cycles.samples = 16
scene.cycles.keyframe_insert("samples", frame=1)
scene.cycles.samples = 64
scene.cycles.keyframe_insert("samples", frame=30)
group = bpy.data.node_groups.new("Heavy", "GeometryNodeTree")
group.interface.new_socket(name="Geometry", in_out="OUTPUT", socket_type="NodeSocketGeometry")
grid = group.nodes.new("GeometryNodeMeshGrid")
grid.inputs["Vertices X"].default_value = 1500
grid.inputs["Vertices Y"].default_value = 1500
noise = group.nodes.new("ShaderNodeTexNoise")
noise.inputs["Scale"].default_value = 1.0
noise.inputs["Scale"].keyframe_insert("default_value", frame=1)
noise.inputs["Scale"].default_value = 5.0
noise.inputs["Scale"].keyframe_insert("default_value", frame=30)
moved = group.nodes.new("GeometryNodeSetPosition")
ends = group.nodes.new("NodeGroupOutput")
group.links.new(grid.outputs["Mesh"], moved.inputs["Geometry"])
group.links.new(noise.outputs["Color"], moved.inputs["Offset"])
group.links.new(moved.outputs["Geometry"], ends.inputs[0])
for at in range(3):
    holder = bpy.data.objects.new("Grid" + str(at), bpy.data.meshes.new("Grid" + str(at)))
    scene.collection.objects.link(holder)
    holder.location.x = at * 3
    holder.modifiers.new("Heavy", "NODES").node_group = group
print("SMOOZY-DESK-GRIDS", [one.name for one in scene.objects], flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

#[test]
#[ignore = "run by hand at the desk with the four Blenders and 5.3"]
fn a_heavy_file_read_off_its_curves_is_faster_and_the_same_in_all_five() {
    use smoozy_job_files_machine::reading_script::the_pre_load_reading_script;
    let at = std::env::temp_dir().join(format!("smoozy-heavy-read-off-its-curves-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make-heavy.py");
    std::fs::write(&maker, THE_HEAVY_FILE).expect("written");
    for (major, minor, blender) in the_four_and_five_three() {
        let named = format!("{major}.{minor}");
        let file = at.join(format!("heavy-{major}{minor}.blend"));
        let (made, said) = run(
            &blender,
            &["--factory-startup", "--background", "--python-exit-code", "63", "--python", maker.to_str().unwrap(), "--", file.to_str().unwrap()],
        );
        for line in String::from_utf8_lossy(&said).lines().filter(|line| line.starts_with("SMOOZY-DESK")) {
            println!("{named}: {line}");
        }
        assert_eq!(made, Some(0), "{named}: the file is made");
        let script = the_pre_load_reading_script(&TheInvocation {
            the_blender_build_path: blender.display().to_string(),
            the_blend_path_on_the_machine: RecordedLocation::of(file.to_str().unwrap().as_bytes().to_vec()),
            the_version_it_will_render_by: TheVersionInHand::of(major, minor),
            the_frames_the_job_covers: Vec::new(),
            the_check_is_written_as_a_picture: false,
            the_check_is_marked: false,
        })
        .expect("every name is on record");
        let (walked, walk_took) = read_with_no_handler(&blender, &at, &file, &counting_each_move(&todays_walk(&script)), &format!("walk-heavy-{major}{minor}.py"));
        let (read, read_took) = read_with_no_handler(&blender, &at, &file, &counting_each_move(&script), &format!("read-heavy-{major}{minor}.py"));
        let identical = between_the_marks(&read) == between_the_marks(&walked);
        println!(
            "{named}: today's walk {:.2} s, {} moves; read off the curves {:.2} s, {} moves; identical: {identical}",
            walk_took.as_secs_f64(),
            steps_of(&walked, "Scene"),
            read_took.as_secs_f64(),
            steps_of(&read, "Scene")
        );
        assert!(identical, "{named}: the reading off the curves differs from today's walk");
        assert_eq!(steps_of(&read, "Scene"), 1, "{named}: one move");
        // **No timing here from 2026-11-14**: every move now leaves the grids
        // out (`Job files and outputs` R4 item 5, cheap moves), so the walk
        // costs about what one move does. The speed against the body as it
        // stood is proved by `drivers_and_cheap_moves_at_the_desk.rs`.
        let _ = (read_took, walk_took);
        std::fs::remove_file(&file).expect("the heavy file is removed");
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

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
//! - **A scene whose compositing tree is keyed is moved**, though the scene
//!   itself carries no animation: a node's value is keyed, and the keys live
//!   on the tree rather than on the scene. (A node's mute cannot be keyed —
//!   4.5 refuses it, *"property "mute" not animatable"* — so the test keys a
//!   value.)

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
        (5, 2, "SMOOZY_DESK_BLENDER_52", "/Applications/Blender.app/Contents/MacOS/Blender"),
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

        // **A keyed compositing tree: moved, with no animation on the scene.**
        assert!(stepped("Tree") > 0, "{named}: a scene whose compositing tree is keyed is moved");
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

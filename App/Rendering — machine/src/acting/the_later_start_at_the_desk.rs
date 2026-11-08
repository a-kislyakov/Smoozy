//! **`Rendering` L10b's later start and `Job files and outputs` R4 item 8's
//! *Where the simulations begin*, with real Blenders** — kept in the tree and
//! ignored. Added 2026-11-08, the Creator's twenty-seventh notes, note 6. Run by
//! hand at the desk, one Blender at a time:
//!
//! ```text
//! SMOOZY_DESK_OUT=<scratch folder> SMOOZY_DESK_BLENDER_53=<5.3 RC> cargo test -p smoozy-rendering-machine the_later_start_at_the_desk -- --ignored --nocapture --test-threads=1
//! ```
//!
//! **What runs is the render worker's driver, whole**, through
//! [`super::the_5_3_readings_at_the_desk`]'s harness: the real reading, the
//! material and the line. **Today's start is had from the same file with one
//! thing added that the reading cannot account for** — an armature in iTaSC's
//! Simulation mode, in a scene of its own, which nothing in the job's scene
//! sees — so the two jobs differ in where the playing starts and nothing else.
//! That is also the iTaSC case of the fallbacks. The account is
//! `Audits/Build/Twenty-seventh notes/Playing from the simulation's own start —
//! what landed.md`.
//!
//! **What this does not prove**: Linux or a graphics card. Every run is on the
//! processor, on this Mac, with files saved by a Blender with no window.

use std::path::{Path, PathBuf};

use smoozy_job_files_machine::reading::WhereTheSimulationsBegin;

use super::the_5_3_readings_at_the_desk::{make, one_job_whole, the_blenders, the_folder, WhatCameBack};
use crate::material::tests::an_untouched_payload;

/// Workbench, 64 pixels square, BMP so a picture's pixels are its bytes after
/// the header; the factory cube kept as a collider, the light and camera kept.
const THE_SCENE: &str = r#"
import bpy, sys, os
from mathutils import Vector
out = sys.argv[sys.argv.index("--") + 1]
scene = bpy.context.scene
scene.name = "Scene"
scene.render.engine = "BLENDER_WORKBENCH"
scene.render.resolution_x = 64
scene.render.resolution_y = 64
scene.render.resolution_percentage = 100
scene.render.filepath = "//render/f_"
scene.render.image_settings.file_format = "BMP"
scene.render.image_settings.color_mode = "RGB"
scene.frame_start = 1
scene.frame_end = 60
camera = bpy.data.objects["Camera"]
camera.location = (0, -12, 6)
camera.rotation_euler = (Vector((0, 0, 1)) - camera.location).to_track_quat("-Z", "Y").to_euler()
cube = bpy.data.objects["Cube"]
cube.modifiers.new("Collision", "COLLISION")

def a_cloth(name, start, end=60, z=4.0):
    bpy.ops.mesh.primitive_grid_add(x_subdivisions=12, y_subdivisions=12, size=4, location=(0, 0, z))
    one = bpy.context.active_object
    one.name = name
    modifier = one.modifiers.new("Cloth", "CLOTH")
    modifier.point_cache.frame_start = start
    modifier.point_cache.frame_end = end
    return one

def an_itasc_armature_elsewhere():
    elsewhere = bpy.data.scenes.new("Elsewhere")
    data = bpy.data.armatures.new("Arm")
    arm = bpy.data.objects.new("Arm", data)
    elsewhere.collection.objects.link(arm)
    scene.collection.objects.link(arm)
    bpy.context.view_layer.objects.active = arm
    with bpy.context.temp_override(object=arm, active_object=arm, selected_objects=[arm], selected_editable_objects=[arm]):
        bpy.ops.object.mode_set(mode="EDIT")
    bone = data.edit_bones.new("B")
    bone.head = (0, 0, 0)
    bone.tail = (0, 0, 1)
    with bpy.context.temp_override(object=arm, active_object=arm, selected_objects=[arm], selected_editable_objects=[arm]):
        bpy.ops.object.mode_set(mode="OBJECT")
    scene.collection.objects.unlink(arm)
    arm.pose.ik_solver = "ITASC"
    arm.pose.ik_param.mode = "SIMULATION"
    return arm
"#;

/// A file made from the scene above and `what`, saved at frame 1.
fn a_file(blender: &Path, here: &Path, name: &str, what: &str) -> PathBuf {
    make(
        blender,
        &format!(
            "{THE_SCENE}\n{what}\nscene.frame_current = 1\nprint(\"SMOOZY-DESK-MADE\", bpy.app.version_string, flush=True)\nbpy.ops.wm.save_as_mainfile(filepath=out)\n"
        ),
        here,
        name,
    )
}

/// One job through the driver, the scene `Scene`, the frames asked.
fn a_job(blender: &Path, file: &Path, root: &Path, frames: Vec<i64>) -> WhatCameBack {
    one_job_whole(blender, file, root, "Scene", &an_untouched_payload(), frames, Vec::new(), None, Vec::new())
}

/// What the reading in force said, and the frames the material played: the
/// first and last `SMOOZY-SIMULATION` frame, or none.
fn what_it_did(came: &WhatCameBack) -> (Option<WhereTheSimulationsBegin>, Option<(i64, i64)>) {
    let said = came.the_composed.as_ref().and_then(|one| one.reading.where_the_simulations_begin.clone());
    let frames: Vec<i64> = came
        .the_renders_output
        .lines()
        .filter_map(|line| line.strip_prefix("SMOOZY-SIMULATION "))
        .filter_map(|json| serde_json::from_str::<serde_json::Value>(json).ok())
        .filter_map(|value| value.get("frame").and_then(|frame| frame.as_i64()))
        .collect();
    (said, frames.first().zip(frames.last()).map(|(a, b)| (*a, *b)))
}

fn the_pixels(path: &Path) -> Vec<u8> {
    let raw = std::fs::read(path).unwrap_or_else(|_| panic!("{} was written", path.display()));
    raw[u32::from_le_bytes(raw[10..14].try_into().unwrap()) as usize..].to_vec()
}

/// Every frame each job wrote, by frame number: its pixels.
fn the_frames(came: &WhatCameBack, root: &Path) -> Vec<(String, Vec<u8>)> {
    came.in_the_jobs_folder
        .iter()
        .filter(|name| name.ends_with(".bmp"))
        .map(|name| (name.clone(), the_pixels(&root.join("job").join(name))))
        .collect()
}

fn say(named: &str, what: &str, came: &WhatCameBack) {
    let (said, played) = what_it_did(came);
    println!(
        "    {named} {what}: launched {:?}, status {:?}, item 8 {said:?}, played {played:?}, {} frames written",
        came.not_launched,
        came.the_renders_status,
        came.in_the_jobs_folder.iter().filter(|name| name.ends_with(".bmp")).count()
    );
}

/// How many frames differ between two jobs, and which.
fn the_frames_that_differ(late: &[(String, Vec<u8>)], old: &[(String, Vec<u8>)]) -> Vec<String> {
    assert_eq!(late.len(), old.len(), "the same frames written");
    late.iter()
        .zip(old)
        .filter(|((a, pa), (b, pb))| {
            assert_eq!(a, b);
            pa != pb
        })
        .map(|((a, _), _)| a.clone())
        .collect()
}

/// **The Creator's shape**: a cloth starting at 20, frames 1–60 asked. The late
/// job plays 20–60, today's start plays 1–60, and every one of the sixty frames
/// is the same, pixel for pixel. **Dynamic Paint likewise**, on vertices,
/// starting at 20, frames 1–40: frames 1–19 come before its start and are
/// rendered after the playing, as before.
#[test]
#[ignore = "run by hand at the desk with Blender 4.5 to 5.3 RC, one at a time"]
fn the_creators_shape_plays_from_the_cloths_start_and_matches_todays_frames() {
    let out = the_folder().join("later-start");
    for (named, blender) in the_blenders(&["4.5", "5.0", "5.1", "5.2", "5.3"]) {
        println!("{named}:");
        let here = out.join(named);
        for (kind, what, frames, begins) in [
            ("cloth", "a_cloth(\"Cloth\", 20)", (1..=60).collect::<Vec<i64>>(), 20),
            ("paint", THE_PAINT, (1..=40).collect(), 20),
        ] {
            let late_file = a_file(&blender, &here, &format!("{kind}-late"), what);
            let old_file = a_file(&blender, &here, &format!("{kind}-old"), &format!("{what}\nan_itasc_armature_elsewhere()"));
            let late = a_job(&blender, &late_file, &here.join(format!("{kind}-late-run")), frames.clone());
            let old = a_job(&blender, &old_file, &here.join(format!("{kind}-old-run")), frames.clone());
            say(named, &format!("{kind} late"), &late);
            say(named, &format!("{kind} old"), &old);
            let highest = *frames.last().unwrap();
            assert_eq!(what_it_did(&late), (Some(WhereTheSimulationsBegin::AtFrame(begins)), Some((begins, highest))), "{named} {kind}");
            // `Render Settings` Q2b: the settings half, which the page reads,
            // carries the same first frame, and nothing where it cannot account.
            let settings = |came: &WhatCameBack| came.the_composed.as_ref().expect("a reading").reading.settings.0["scenes"][0]["simulations_begin"].clone();
            assert_eq!(settings(&late), serde_json::json!(begins), "{named} {kind}");
            assert_eq!(settings(&old), serde_json::Value::Null, "{named} {kind}");
            let (said, played) = what_it_did(&old);
            assert!(matches!(said, Some(WhereTheSimulationsBegin::CannotAccount(_))), "{named} {kind}: {said:?}");
            assert_eq!(played, Some((1, highest)), "{named} {kind}");
            let differ = the_frames_that_differ(
                &the_frames(&late, &here.join(format!("{kind}-late-run"))),
                &the_frames(&old, &here.join(format!("{kind}-old-run"))),
            );
            println!("    {named} {kind}: {} frames compared, {} differ {differ:?}", frames.len(), differ.len());
            assert!(differ.is_empty(), "{named} {kind}: {differ:?}");
        }
    }
}

/// Dynamic Paint on vertices, a moving brush painting the canvas from frame 20,
/// shown as vertex colour (the Earlier frames hand tests' own scene, shrunk).
const THE_PAINT: &str = r#"
scene.display.shading.light = "FLAT"
bpy.ops.mesh.primitive_grid_add(x_subdivisions=30, y_subdivisions=30, size=6, location=(0, 0, 2.5))
canvas = bpy.context.active_object
canvas.modifiers.new("Dynamic Paint", "DYNAMIC_PAINT")
with bpy.context.temp_override(object=canvas, active_object=canvas):
    bpy.ops.dpaint.type_toggle(type="CANVAS")
surface = canvas.modifiers["Dynamic Paint"].canvas_settings.canvas_surfaces[0]
with bpy.context.temp_override(object=canvas, active_object=canvas):
    bpy.ops.dpaint.output_toggle(output="A")
colours = canvas.data.color_attributes
colours.active_color = colours["dp_paintmap"]
if hasattr(colours, "render_color_index"):
    colours.render_color_index = colours.active_color_index
kinds = [one.identifier for one in scene.display.shading.bl_rna.properties["color_type"].enum_items]
scene.display.shading.color_type = "VERTEX" if "VERTEX" in kinds else "ATTRIBUTE"
surface.frame_start = 20
surface.frame_end = 60
bpy.ops.mesh.primitive_uv_sphere_add(radius=0.7, location=(-3, 0, 2.8))
brush = bpy.context.active_object
brush.modifiers.new("Dynamic Paint", "DYNAMIC_PAINT")
with bpy.context.temp_override(object=brush, active_object=brush):
    bpy.ops.dpaint.type_toggle(type="BRUSH")
brush.modifiers["Dynamic Paint"].ui_type = "BRUSH"
brush.modifiers["Dynamic Paint"].brush_settings.paint_source = "DISTANCE"
brush.modifiers["Dynamic Paint"].brush_settings.paint_distance = 0.5
brush.keyframe_insert("location", frame=1)
brush.location.x = 3
brush.keyframe_insert("location", frame=60)
brush.hide_render = True
"#;

/// **A thousand-frame scene whose only simulation starts at 900** plays from
/// 900, and its last frame is today's.
#[test]
#[ignore = "run by hand at the desk with Blender 4.5 to 5.3 RC, one at a time"]
fn a_simulation_at_nine_hundred_plays_from_nine_hundred() {
    let out = the_folder().join("later-start-900");
    for (named, blender) in the_blenders(&["4.5", "5.0", "5.1", "5.2", "5.3"]) {
        println!("{named}:");
        let here = out.join(named);
        let what = "scene.frame_end = 1000\na_cloth(\"Cloth\", 900, 1000)";
        let late_file = a_file(&blender, &here, "late", what);
        let old_file = a_file(&blender, &here, "old", &format!("{what}\nan_itasc_armature_elsewhere()"));
        let late = a_job(&blender, &late_file, &here.join("late-run"), vec![998, 999, 1000]);
        let old = a_job(&blender, &old_file, &here.join("old-run"), vec![998, 999, 1000]);
        say(named, "late", &late);
        say(named, "old", &old);
        assert_eq!(what_it_did(&late), (Some(WhereTheSimulationsBegin::AtFrame(900)), Some((900, 1000))), "{named}");
        assert_eq!(what_it_did(&old).1, Some((1, 1000)), "{named}");
        let differ = the_frames_that_differ(&the_frames(&late, &here.join("late-run")), &the_frames(&old, &here.join("old-run")));
        println!("    {named}: 3 frames compared, {} differ {differ:?}", differ.len());
        assert!(differ.is_empty(), "{named}: {differ:?}");
    }
}

/// **One case the reading cannot account for, from each of the brief's
/// bullets, each playing from today's start** — a simulation zone with its
/// Cache off; a Scene strip showing another scene; a missing library (the
/// iTaSC armature is the old-start job above). **And one place outside the
/// scene that is read, not fallen back on**: a background scene whose cloth
/// starts at 5 brings the start down to 5.
#[test]
#[ignore = "run by hand at the desk with Blender 4.5 to 5.3 RC, one at a time"]
fn what_it_cannot_account_for_plays_from_todays_start() {
    let out = the_folder().join("later-start-fallbacks");
    for (named, blender) in the_blenders(&["4.5", "5.0", "5.1", "5.2", "5.3"]) {
        println!("{named}:");
        let here = out.join(named);
        let library = a_file(&blender, &here.join("assets"), "library", "a_cloth(\"Linked\", 30)");
        let cases: Vec<(&str, String, Option<i64>)> = vec![
            ("zone-cache-off", format!("a_cloth(\"Cloth\", 20)\n{THE_ZONE}\nholder.use_simulation_cache = False"), None),
            ("zone-cache-on", format!("a_cloth(\"Cloth\", 20)\n{THE_ZONE}"), Some(20)),
            ("scene-strip", format!("a_cloth(\"Cloth\", 20)\n{THE_STRIP}"), None),
            ("missing-library", format!("a_cloth(\"Cloth\", 20)\n{}", the_link(&library)), None),
            ("background-scene", format!("a_cloth(\"Cloth\", 20)\n{THE_BACKGROUND}"), Some(5)),
        ];
        for (case, what, begins) in cases {
            let file = a_file(&blender, &here, case, &what);
            if case == "missing-library" {
                std::fs::rename(&library, here.join("assets").join("moved-away.blend")).expect("the library is moved away");
            }
            let came = a_job(&blender, &file, &here.join(format!("{case}-run")), vec![25, 26]);
            say(named, case, &came);
            if case == "missing-library" {
                std::fs::rename(here.join("assets").join("moved-away.blend"), &library).expect("put back");
            }
            let (said, played) = what_it_did(&came);
            match begins {
                Some(frame) => {
                    assert_eq!(said, Some(WhereTheSimulationsBegin::AtFrame(frame)), "{named} {case}");
                    assert_eq!(played.map(|(from, _)| from), Some(frame), "{named} {case}");
                }
                None => {
                    assert!(matches!(said, Some(WhereTheSimulationsBegin::CannotAccount(_))), "{named} {case}: {said:?}");
                    assert_eq!(played.map(|(from, _)| from), Some(1), "{named} {case}");
                }
            }
        }
    }
}

/// A cube whose simulation zone has its own range from 20, on the object
/// `holder`.
const THE_ZONE: &str = r#"
bpy.ops.mesh.primitive_cube_add(size=0.5, location=(3, 0, 0))
holder = bpy.context.active_object
tree = bpy.data.node_groups.new("Zone", "GeometryNodeTree")
tree.interface.new_socket("Geometry", in_out="INPUT", socket_type="NodeSocketGeometry")
tree.interface.new_socket("Geometry", in_out="OUTPUT", socket_type="NodeSocketGeometry")
into = tree.nodes.new("GeometryNodeSimulationInput")
leaving = tree.nodes.new("GeometryNodeSimulationOutput")
into.pair_with_output(leaving)
source = tree.nodes.new("NodeGroupInput")
end = tree.nodes.new("NodeGroupOutput")
tree.links.new(source.outputs[0], into.inputs["Geometry"])
tree.links.new(into.outputs["Geometry"], leaving.inputs["Geometry"])
tree.links.new(leaving.outputs["Geometry"], end.inputs[0])
modifier = holder.modifiers.new("Zone", "NODES")
modifier.node_group = tree
modifier.bakes[0].use_custom_simulation_frame_range = True
modifier.bakes[0].frame_start = 20
modifier.bakes[0].frame_end = 60
"#;

/// The job's scene's sequencer holding a Scene strip of another scene.
const THE_STRIP: &str = r#"
other = bpy.data.scenes.new("Shown")
editor = scene.sequence_editor_create()
strips = editor.strips if hasattr(editor, "strips") else editor.sequences
strips.new_scene("Shown", other, 1, 1)
"#;

/// A background scene holding a cloth that starts at 5.
const THE_BACKGROUND: &str = r#"
behind = bpy.data.scenes.new("Behind")
behind.frame_start = 1
behind.frame_end = 60
bpy.ops.mesh.primitive_grid_add(x_subdivisions=6, y_subdivisions=6, size=2, location=(4, 0, 4))
early = bpy.context.active_object
early.name = "Early"
for one in list(early.users_collection):
    one.objects.unlink(early)
behind.collection.objects.link(early)
early.modifiers.new("Cloth", "CLOTH").point_cache.frame_start = 5
scene.background_set = behind
"#;

/// A cloth linked from `library`, by a relative path.
fn the_link(library: &Path) -> String {
    format!(
        r#"
bpy.ops.wm.save_as_mainfile(filepath=out)
with bpy.data.libraries.load({library:?}, link=True, relative=True) as (there, here):
    here.objects = ["Linked"]
for one in here.objects:
    scene.collection.objects.link(one)
"#,
        library = library.display().to_string(),
    )
}

/// **The shutter at the near end** (`Rendering` L10b): a shutter reaching back
/// before the later start, Cycles with motion blur on, the cloth starting at 20
/// and frames 20–22 asked. CENTER reaches half a frame back, END its whole
/// length. The late job and today's give the same pictures.
#[test]
#[ignore = "run by hand at the desk with Blender 4.5 to 5.3 RC, one at a time"]
fn the_shutter_reaching_before_the_later_start_needs_nothing_more() {
    let out = the_folder().join("later-start-shutter");
    for (named, blender) in the_blenders(&["4.5", "5.0", "5.1", "5.2", "5.3"]) {
        println!("{named}:");
        let here = out.join(named);
        for (position, length) in [("CENTER", 1.0), ("END", 2.0)] {
            let what = format!(
                "a_cloth(\"Cloth\", 20)\nscene.render.engine = \"CYCLES\"\nscene.cycles.device = \"CPU\"\nscene.cycles.samples = 4\nscene.cycles.use_denoising = False\nscene.cycles.use_adaptive_sampling = False\nscene.render.use_motion_blur = True\nscene.render.motion_blur_position = \"{position}\"\nscene.render.motion_blur_shutter = {length}"
            );
            let case = position.to_lowercase();
            let late_file = a_file(&blender, &here, &format!("{case}-late"), &what);
            let old_file = a_file(&blender, &here, &format!("{case}-old"), &format!("{what}\nan_itasc_armature_elsewhere()"));
            let late = a_job(&blender, &late_file, &here.join(format!("{case}-late-run")), vec![20, 21, 22]);
            let old = a_job(&blender, &old_file, &here.join(format!("{case}-old-run")), vec![20, 21, 22]);
            say(named, &format!("{case} late"), &late);
            say(named, &format!("{case} old"), &old);
            assert_eq!(what_it_did(&late).1.map(|(from, _)| from), Some(20), "{named} {case}");
            assert_eq!(what_it_did(&old).1.map(|(from, _)| from), Some(1), "{named} {case}");
            let differ = the_frames_that_differ(
                &the_frames(&late, &here.join(format!("{case}-late-run"))),
                &the_frames(&old, &here.join(format!("{case}-old-run"))),
            );
            println!("    {named} {case}: 3 frames compared, {} differ {differ:?}", differ.len());
            assert!(differ.is_empty(), "{named} {case}: {differ:?}");
        }
    }
}

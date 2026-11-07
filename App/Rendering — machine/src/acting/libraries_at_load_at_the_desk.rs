//! **`Dependency Upload & Remap` M8b and `Dependency Upload & Remap` M4b with real
//! Blenders: a file's start scripts and its libraries, and a set of tiles** — kept
//! in the tree and ignored.
//! Run by hand at the desk, one Blender at a time:
//!
//! ```text
//! SMOOZY_DESK_OUT=<scratch folder> cargo test -p smoozy-rendering-machine libraries_at_load_at_the_desk -- --ignored --nocapture
//! ```
//!
//! `SMOOZY_DESK_ONLY=53` narrows a run to one Blender. **What runs is the render
//! worker's driver, whole**, as in [`super::the_5_3_readings_at_the_desk`], whose
//! harness this borrows, and the fixed copies are made by `Dependency Upload &
//! Remap` M14's own script and words.
//!
//! **The files**, made here, with no production content. A top file, a library in
//! the middle and a library below it, the shape a real shot has, 64 pixels square,
//! Cycles on the processor:
//!
//! - **red, on the left: the top file's start script.** The panel saves far out of
//!   the picture. The script moves it in, naming it by the library's recorded text,
//!   as a real shot's lighting script does.
//! - **green, on the right: a start script in the library below.** It registers the
//!   setting a driver on the panel's strength reads. With no setting registered the
//!   panel stays at what the fixing saved, which is dark.
//!
//! **So the picture shows the fault plainly.** The whole folder by hand gives red
//! and green. A job whose library arrives by `Dependency Upload & Remap` M8's reload
//! gives black and black, with Blender ending at 0 and nothing said: the case the
//! whole-location test below keeps, because the opening folder cannot reach it. A job
//! through the opening folder gives red and green.
//!
//! **What it does not prove**: Linux, a card, or a file saved from Blender's window.
//! The real shot's picture is `Audits/Build/Blender 5.3/Why the shot's picture
//! differs — what landed.md`.

use std::path::{Path, PathBuf};

use smoozy_dependencies_machine::bytes::{MachineLocation, RecordedLocation};
use smoozy_dependencies_machine::check::Kind;
use smoozy_dependencies_machine::fixing_script::{the_fixing_python, the_words_of_the_fixing};
use smoozy_dependencies_machine::remap::{compose_pair, NewLocation, Pair};

use super::the_5_3_readings_at_the_desk::{one_job_with_pairs, run, the_blenders, the_folder};
use crate::material::tests::an_untouched_payload;

const THE_FILES: &str = r#"
import bpy, os, sys, warnings
warnings.simplefilter("ignore", DeprecationWarning)
args = sys.argv[sys.argv.index("--") + 1:]
what, out = args[0], args[1]
bpy.ops.wm.read_factory_settings(use_empty=True)

def a_panel(name, x, colour):
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata([(-1, 0, -1), (1, 0, -1), (1, 0, 1), (-1, 0, 1)], [], [(0, 1, 2, 3)])
    mesh.update()
    panel = bpy.data.objects.new(name, mesh)
    panel.location = (x, 0, 0)
    material = bpy.data.materials.new(name)
    material.use_nodes = True
    tree = material.node_tree
    tree.nodes.clear()
    leaving = tree.nodes.new("ShaderNodeOutputMaterial")
    emission = tree.nodes.new("ShaderNodeEmission")
    emission.inputs["Color"].default_value = colour
    value = tree.nodes.new("ShaderNodeValue")
    value.name = "Glow"
    value.outputs[0].default_value = 1.0
    tree.links.new(value.outputs[0], emission.inputs["Strength"])
    tree.links.new(emission.outputs[0], leaving.inputs[0])
    mesh.materials.append(material)
    return panel, material

if what == "below":
    panel, material = a_panel("GlowPanel", 1.2, (0, 1, 0, 1))
    collection = bpy.data.collections.new("Below")
    collection.objects.link(panel)
    bpy.context.scene.collection.children.link(collection)
    material.node_tree.nodes["Glow"].outputs[0].default_value = 0.0
    script = bpy.data.texts.new("below_script.py")
    script.from_string(
        "import bpy\n"
        "bpy.types.Scene.desk_glow = bpy.props.FloatProperty(name='Glow', default=1.0)\n"
    )
    script.use_module = True
    # A library's text comes along only where something linked names it, as a rig
    # names its own script.
    panel["start_script"] = script
    curve = material.node_tree.driver_add('nodes["Glow"].outputs[0].default_value')
    curve.driver.type = "SCRIPTED"
    variable = curve.driver.variables.new()
    variable.name = "var"
    variable.type = "CONTEXT_PROP"
    variable.targets[0].context_property = "ACTIVE_SCENE"
    variable.targets[0].data_path = "desk_glow"
    curve.driver.expression = "var"
elif what == "middle":
    panel, material = a_panel("MovedPanel", 100.0, (1, 0, 0, 1))
    collection = bpy.data.collections.new("Stage")
    collection.objects.link(panel)
    bpy.context.scene.collection.children.link(collection)
    with bpy.data.libraries.load(args[2], link=True, relative=False) as (data_from, data_to):
        data_to.collections = ["Below"]
    collection.children.link(data_to.collections[0])
    for library in bpy.data.libraries:
        library.filepath = "//below.blend"
elif what == "top":
    recorded = args[3]
    scene = bpy.context.scene
    with bpy.data.libraries.load(args[2], link=True, relative=False) as (data_from, data_to):
        data_to.collections = ["Stage"]
    scene.collection.children.link(data_to.collections[0])
    for library in bpy.data.libraries:
        if library.filepath.endswith("middle.blend"):
            library.filepath = recorded
    camera_data = bpy.data.cameras.new("Camera")
    camera_data.type = "ORTHO"
    camera_data.ortho_scale = 5.0
    camera = bpy.data.objects.new("Camera", camera_data)
    camera.location = (0, -10, 0)
    camera.rotation_euler = (1.5707963, 0, 0)
    scene.collection.objects.link(camera)
    scene.camera = camera
    script = bpy.data.texts.new("top_script.py")
    script.from_string(
        "import bpy\n"
        "try:\n"
        "    bpy.data.objects['MovedPanel', " + repr(recorded) + "].location = (-1.2, 0, 0)\n"
        "except Exception:\n"
        "    print('SMOOZY-DESK the start script could not reach the panel')\n"
    )
    script.use_module = True
    scene.name = "Scene"
    scene.render.engine = "CYCLES"
    scene.cycles.device = "CPU"
    scene.cycles.samples = 1
    scene.cycles.use_adaptive_sampling = False
    scene.cycles.use_denoising = False
    scene.render.resolution_x = 64
    scene.render.resolution_y = 64
    scene.render.resolution_percentage = 100
    scene.render.filepath = "//render/f_"
    scene.render.image_settings.file_format = "PNG"
    scene.view_settings.view_transform = "Standard"
    scene.frame_start = 1
    scene.frame_end = 1
print("SMOOZY-DESK-MADE", what, bpy.app.version_string, bpy.app.build_hash.decode(), flush=True)
os.makedirs(os.path.dirname(out), exist_ok=True)
bpy.ops.wm.save_as_mainfile(filepath=out, relative_remap=False)
"#;

/// **A set of two tiles beside a top file**: the panel's first tile green, its
/// second, which the panel's corners do not reach, blue. Saved as a set, recorded
/// `//maps/set/tile_<UDIM>.png`.
const THE_TILED_FILE: &str = r#"
import bpy, os, sys, warnings
warnings.simplefilter("ignore", DeprecationWarning)
args = sys.argv[sys.argv.index("--") + 1:]
out = args[0]
folder = os.path.join(os.path.dirname(out), "maps", "set")
os.makedirs(folder, exist_ok=True)
for number, colour in ((1001, (0, 1, 0, 1)), (1002, (0, 0, 1, 1))):
    one = bpy.data.images.new("t%d" % number, 4, 4)
    one.pixels = list(colour) * 16
    one.filepath_raw = os.path.join(folder, "tile_%d.png" % number)
    one.file_format = "PNG"
    one.save()
    bpy.data.images.remove(one)
bpy.ops.wm.read_factory_settings(use_empty=True)
scene = bpy.context.scene
image = bpy.data.images.load(os.path.join(folder, "tile_1001.png"))
image.source = "TILED"
image.tiles.new(tile_number=1002)
image.filepath = "//maps/set/tile_<UDIM>.png"
mesh = bpy.data.meshes.new("Panel")
mesh.from_pydata([(-2, 0, -2), (2, 0, -2), (2, 0, 2), (-2, 0, 2)], [], [(0, 1, 2, 3)])
mesh.update()
uv = mesh.uv_layers.new(name="UVMap")
for at, corner in enumerate(((0.1, 0.1), (0.9, 0.1), (0.9, 0.9), (0.1, 0.9))):
    uv.data[at].uv = corner
panel = bpy.data.objects.new("Panel", mesh)
scene.collection.objects.link(panel)
material = bpy.data.materials.new("Panel")
material.use_nodes = True
tree = material.node_tree
tree.nodes.clear()
leaving = tree.nodes.new("ShaderNodeOutputMaterial")
emission = tree.nodes.new("ShaderNodeEmission")
texture = tree.nodes.new("ShaderNodeTexImage")
texture.image = image
tree.links.new(texture.outputs[0], emission.inputs["Color"])
tree.links.new(emission.outputs[0], leaving.inputs[0])
mesh.materials.append(material)
camera_data = bpy.data.cameras.new("Camera")
camera_data.type = "ORTHO"
camera_data.ortho_scale = 5.0
camera = bpy.data.objects.new("Camera", camera_data)
camera.location = (0, -10, 0)
camera.rotation_euler = (1.5707963, 0, 0)
scene.collection.objects.link(camera)
scene.camera = camera
scene.name = "Scene"
scene.render.engine = "CYCLES"
scene.cycles.device = "CPU"
scene.cycles.samples = 1
scene.cycles.use_adaptive_sampling = False
scene.cycles.use_denoising = False
scene.render.resolution_x = 64
scene.render.resolution_y = 64
scene.render.resolution_percentage = 100
scene.render.filepath = "//render/f_"
scene.render.image_settings.file_format = "PNG"
scene.view_settings.view_transform = "Standard"
scene.frame_start = 1
scene.frame_end = 1
print("SMOOZY-DESK-MADE", image.source, image.filepath, bpy.app.version_string, bpy.app.build_hash.decode(), flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out, relative_remap=False)
"#;

const THE_PIXELS: &str = r#"
import bpy, sys
for one in sys.argv[sys.argv.index("--") + 1:]:
    image = bpy.data.images.load(one)
    w, h = image.size
    p = image.pixels[:]
    def at(x, y):
        i = 4 * (y * w + x)
        return [round(v, 1) for v in p[i:i + 3]]
    print("SMOOZY-DESK-PIXELS", at(w // 2 - 16, h // 2), at(w // 2 + 16, h // 2), flush=True)
"#;

fn the_words(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

fn path_text(path: &Path) -> String {
    path.display().to_string()
}

fn a_made_file(blender: &Path, here: &Path, script: &str, arguments: &[&str]) {
    std::fs::create_dir_all(here).expect("a folder");
    let maker = here.join("make.py");
    std::fs::write(&maker, script).expect("written");
    let mut words = the_words(&["--factory-startup", "--background", "--python-exit-code", "63", "--python"]);
    words.push(path_text(&maker));
    words.push("--".to_string());
    words.extend(the_words(arguments));
    let (status, out, err) = run(blender, &words);
    let said = String::from_utf8_lossy(&out);
    println!("    {}", said.lines().find(|line| line.starts_with("SMOOZY-DESK-MADE")).unwrap_or("no made line"));
    assert_eq!(status, Some(0), "made: {said} {}", String::from_utf8_lossy(&err));
}

fn a_library_pair(recorded: &str, fixed: &Path) -> Pair {
    compose_pair(
        Kind::Library,
        "filepath",
        &RecordedLocation::as_the_reading_carried_it(recorded.as_bytes()),
        NewLocation::AFixedCopy(MachineLocation::composed(path_text(fixed).into_bytes())),
    )
    .expect("a library pair names a fixed copy")
}

/// **`Dependency Upload & Remap` M14's own script and words**: the copy written to
/// the fixed copy's place.
fn fixed(blender: &Path, from: &Path, to: &Path, pairs: &[Pair], here: &Path) {
    std::fs::create_dir_all(to.parent().unwrap()).expect("a folder");
    let script = here.join(format!("fix-{}.py", to.file_stem().unwrap().to_string_lossy()));
    std::fs::write(&script, the_fixing_python(pairs, path_text(to).as_bytes())).expect("written");
    let words: Vec<String> = the_words_of_the_fixing(path_text(from).as_bytes(), path_text(&script).as_bytes())
        .into_iter()
        .map(|word| String::from_utf8(word).unwrap())
        .collect();
    let (status, out, err) = run(blender, &words);
    assert_eq!(status, Some(0), "fixed: {} {}", String::from_utf8_lossy(&out), String::from_utf8_lossy(&err));
}

/// The colours left and right of the middle of each picture.
fn the_pixels(blender: &Path, here: &Path, pictures: &[PathBuf]) -> Vec<String> {
    let script = here.join("pixels.py");
    std::fs::write(&script, THE_PIXELS).expect("written");
    let mut words = the_words(&["--factory-startup", "--background", "--python"]);
    words.push(path_text(&script));
    words.push("--".to_string());
    words.extend(pictures.iter().map(|one| path_text(one)));
    let (status, out, _) = run(blender, &words);
    assert_eq!(status, Some(0));
    String::from_utf8_lossy(&out)
        .lines()
        .filter_map(|line| line.strip_prefix("SMOOZY-DESK-PIXELS ").map(str::to_string))
        .collect()
}

/// The whole folder, by hand: what the file makes on the artist's computer.
fn the_whole_folder_by_hand(blender: &Path, file: &Path, out: &Path) -> PathBuf {
    let mut words = the_words(&["--factory-startup", "--background", "--enable-autoexec"]);
    words.push(path_text(file));
    words.push("--render-output".to_string());
    words.push(format!("{}/hand_", path_text(out)));
    words.push("--render-frame".to_string());
    words.push("1".to_string());
    let (status, said, err) = run(blender, &words);
    assert_eq!(status, Some(0), "{} {}", String::from_utf8_lossy(&said), String::from_utf8_lossy(&err));
    out.join("hand_0001.png")
}

const RED_AND_GREEN: &str = "[1.0, 0.0, 0.0] [0.0, 1.0, 0.0]";

/// **`Dependency Upload & Remap` M8b: a job whose libraries are recorded relative to
/// the file renders what the whole folder renders**, its start scripts having seen
/// them. **And a job recorded by whole location, which the folder cannot reach, keeps
/// the fault**, which is what shows this test can see it.
#[test]
#[ignore = "run by hand at the desk with the six Blenders, one at a time"]
fn a_files_start_scripts_see_its_libraries() {
    let out = the_folder().join("start-scripts");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["4.5", "5.0", "5.1", "5.2", "5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let pack = here.join("pack");
        let lib = pack.join("lib");
        let below = lib.join("below.blend");
        let middle = lib.join("middle.blend");
        let top = pack.join("top.blend");
        a_made_file(&blender, &here.join("m1"), THE_FILES, &["below", below.to_str().unwrap()]);
        a_made_file(&blender, &here.join("m2"), THE_FILES, &["middle", middle.to_str().unwrap(), below.to_str().unwrap()]);
        a_made_file(&blender, &here.join("m3"), THE_FILES, &["top", top.to_str().unwrap(), middle.to_str().unwrap(), "//lib/middle.blend"]);
        let by_hand = the_whole_folder_by_hand(&blender, &top, &here);

        let copies = here.join("fixed");
        let below_fixed = copies.join("below.blend");
        let middle_fixed = copies.join("middle.blend");
        fixed(&blender, &below, &below_fixed, &[], &here);
        fixed(&blender, &middle, &middle_fixed, &[a_library_pair("//below.blend", &below_fixed)], &here);

        // **The opening folder.**
        let came = one_job_with_pairs(
            &blender,
            &top,
            &here.join("through-the-folder"),
            &an_untouched_payload(),
            vec![1],
            vec![a_library_pair("//lib/middle.blend", &middle_fixed)],
        );
        println!("    the line: {}", came.the_line);
        println!("    not launched: {:?}; the render ended at {:?}", came.not_launched, came.the_renders_status);
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        let picture = here.join("through-the-folder/job").join(
            came.in_the_jobs_folder.iter().find(|name| name.ends_with(".png")).expect("a picture"),
        );

        // **By whole location: the top file recording the library by a place that
        // is gone, so assignment and reload is all there is.**
        let whole_top = here.join("whole/top.blend");
        a_made_file(&blender, &here.join("m4"), THE_FILES, &["top", whole_top.to_str().unwrap(), middle.to_str().unwrap(), "/nowhere/lib/middle.blend"]);
        let old = one_job_with_pairs(
            &blender,
            &whole_top,
            &here.join("by-whole-location"),
            &an_untouched_payload(),
            vec![1],
            vec![a_library_pair("/nowhere/lib/middle.blend", &middle_fixed)],
        );
        let old_picture = here.join("by-whole-location/job").join(
            old.in_the_jobs_folder.iter().find(|name| name.ends_with(".png")).expect("a picture"),
        );

        let pixels = the_pixels(&blender, &here, &[by_hand, picture, old_picture]);
        println!("    the whole folder by hand, through the folder, by whole location: {pixels:?}");
        let mut one_fault = |what: String| {
            println!("    FAULT {what}");
            faults.push(format!("{named}: {what}"));
        };
        if pixels.first().map(String::as_str) != Some(RED_AND_GREEN) {
            one_fault(format!("the whole folder by hand gave {:?}", pixels.first()));
        }
        if pixels.get(1).map(String::as_str) != Some(RED_AND_GREEN) {
            one_fault(format!("through the folder gave {:?}", pixels.get(1)));
        }
        if pixels.get(2).map(String::as_str) == Some(RED_AND_GREEN) {
            one_fault("by whole location showed no fault, so this test cannot see one".to_string());
        }
        if came.the_renders_output.contains("Cannot find lib") {
            one_fault("the first load still could not find a library".to_string());
        }
        if !came.the_renders_output.contains(r#""applied": true, "data_blocks": 1, "assignment_alone": "Established", "reloaded": false"#) {
            one_fault("the library pair did not report applied with no reload".to_string());
        }
        if !came.the_line.contains("opening-1/top.blend") {
            one_fault(format!("the line opened {}", came.the_line));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **`Dependency Upload & Remap` M4b: Blender reads a set of tiles from a location
/// that keeps the file's name**, assigned with no reload, as an image is, and **draws
/// it as missing from the folder alone**, which is what a pair used to name.
#[test]
#[ignore = "run by hand at the desk with the six Blenders, one at a time"]
fn a_set_of_tiles_is_read_from_a_location_that_keeps_its_name() {
    let out = the_folder().join("tiles");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["4.5", "5.0", "5.1", "5.2", "5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let pack = here.join("pack");
        let top = pack.join("top.blend");
        a_made_file(&blender, &here.join("m"), THE_TILED_FILE, &[top.to_str().unwrap()]);
        let by_hand = the_whole_folder_by_hand(&blender, &top, &here);

        let mirror = here.join("mirror/maps/set");
        std::fs::create_dir_all(&mirror).expect("a folder");
        for name in ["tile_1001.png", "tile_1002.png"] {
            std::fs::copy(pack.join("maps/set").join(name), mirror.join(name)).expect("copied");
        }
        let an_image_pair = |to: &str| {
            compose_pair(
                Kind::Image,
                "filepath",
                &RecordedLocation::as_the_reading_carried_it(b"//maps/set/tile_<UDIM>.png"),
                NewLocation::AMirror(MachineLocation::composed(to.as_bytes().to_vec())),
            )
            .expect("an image pair")
        };
        let mut pictures = vec![by_hand];
        for (name, to) in [
            ("keeps-its-name", format!("{}/tile_<UDIM>.png", path_text(&mirror))),
            ("the-folder-alone", path_text(&mirror)),
        ] {
            let came = one_job_with_pairs(
                &blender,
                &top,
                &here.join(name),
                &an_untouched_payload(),
                vec![1],
                vec![an_image_pair(&to)],
            );
            println!("    {name}: not launched {:?}; ended at {:?}", came.not_launched, came.the_renders_status);
            pictures.push(here.join(name).join("job").join(
                came.in_the_jobs_folder.iter().find(|one| one.ends_with(".png")).expect("a picture"),
            ));
        }
        let pixels = the_pixels(&blender, &here, &pictures);
        println!("    by hand, keeping its name, the folder alone: {pixels:?}");
        if pixels.first() != pixels.get(1) || !pixels[0].contains("[0.0, 1.0, 0.0]") {
            faults.push(format!("{named}: keeping its name gave {pixels:?}"));
        }
        if pixels.get(2) == pixels.first() {
            faults.push(format!("{named}: the folder alone showed no fault, so this test cannot see one"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

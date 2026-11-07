//! **Packed linked data through the fixing's save, rendered before and after —
//! the 5.3 integration's piece 1 (Reading a file 8, Dependencies 13, Node setups
//! 2), kept in the tree and ignored.** 5.3's release notes change which packed
//! linked items count as direct and say only direct ones are written, and M14's
//! save writes a copy. Run by hand at the desk, one Blender at a time:
//!
//! ```text
//! SMOOZY_DESK_OUT=<scratch folder> cargo test -p smoozy-dependencies-machine --test packed_linked_data_at_the_desk -- --ignored --nocapture
//! ```
//!
//! `SMOOZY_DESK_ONLY=53` narrows it to one Blender. Each is found at its usual
//! place, or at `SMOOZY_DESK_BLENDER_52`, `_53` and `_54`. **This crate's own
//! code starts no process**; this file is a test of it, not part of it.
//!
//! **The file**, three planes side by side, each showing one colour from
//! somewhere else, 64 pixels square, Cycles on the processor:
//!
//! - **red, direct**: a local plane using a packed linked material;
//! - **green, through another packed item**: a local plane using a packed linked
//!   material whose colour comes from a packed linked node group inside it;
//! - **blue, through a regular link**: a plane linked, not packed, from
//!   `middle.blend`, whose material that file packed from `base.blend` — the
//!   release notes' *indirectly linked* case.
//!
//! **What it does**: makes the files, moves the two libraries that were packed
//! out of reach, renders the file, runs [`the_fixing_python`] with the launch
//! line M14 uses, first on `middle.blend` with no pair and then on the file with
//! one real pair (the regular link, to that fixed copy), takes `middle.blend`
//! away from its old place so only the pair can find it, and renders the copy. **The two pictures must match byte for byte and each must
//! show all three colours**, so a missing item, which draws pink, cannot pass.
//!
//! **What it does not prove**: Linux, a card, or a file saved from Blender's
//! interface. Every run is on the processor, on this Mac, with no window.
//!
//! **Names used**, read off `bpy.types.BlendDataLibraries` in the project's
//! copy of 5.3's reference (`load(filepath, *, link, pack, relative, …)`) and off
//! the three Blenders' own property lists at the desk on 2026-11-02: `ID.library`,
//! `ID.is_linked_packed`, `ID.is_library_indirect`, `Library.filepath`,
//! `Library.is_archive`. The account is `Audits/Build/Blender 5.3/Piece 1, the
//! desk readings — what landed.md`.

// Mac and Linux only: it reads a path's bytes the Unix way, and runs at the desk.
#![cfg(unix)]

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use smoozy_dependencies_machine::bytes::{MachineLocation, RecordedLocation};
use smoozy_dependencies_machine::check::Kind;
use smoozy_dependencies_machine::fixing_script::{the_fixing_python, the_words_of_the_fixing};
use smoozy_dependencies_machine::remap::{compose_pair, NewLocation};

/// **No run of Blender here is waited on past this.**
const ONE_RUN_AT_MOST: Duration = Duration::from_secs(300);

const THE_LIBRARIES_AND_THE_FILE: &str = r#"
import bpy, os, sys
args = sys.argv[sys.argv.index("--") + 1:]
what, out = args[0], args[1]
bpy.ops.wm.read_factory_settings(use_empty=True)

def an_emission(name, colour):
    material = bpy.data.materials.new(name)
    material.use_nodes = True
    tree = material.node_tree
    tree.nodes.clear()
    emission = tree.nodes.new("ShaderNodeEmission")
    output = tree.nodes.new("ShaderNodeOutputMaterial")
    tree.links.new(emission.outputs[0], output.inputs[0])
    emission.inputs[0].default_value = colour
    return material

def a_plane(name, x, material, scene):
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata([(x - 1, -1, 0), (x + 1, -1, 0), (x + 1, 1, 0), (x - 1, 1, 0)], [], [(0, 1, 2, 3)])
    mesh.materials.append(material)
    plane = bpy.data.objects.new(name, mesh)
    scene.collection.objects.link(plane)
    return plane

def describe(label):
    print("SMOOZY-DESK-HELD", label, bpy.app.version_string, flush=True)
    for one in list(bpy.data.materials) + list(bpy.data.node_groups) + list(bpy.data.objects):
        print("SMOOZY-DESK-HELD ", type(one).__name__, one.name, "library", one.library.filepath if one.library else None,
              "packed", one.is_linked_packed, "indirect", one.is_library_indirect, flush=True)
    for one in bpy.data.libraries:
        print("SMOOZY-DESK-HELD  Library", one.name, one.filepath, "archive", one.is_archive, flush=True)

scene = bpy.context.scene
if what == "base":
    an_emission("BaseBlue", (0, 0, 1, 1)).use_fake_user = True
elif what == "lib":
    an_emission("DirectRed", (1, 0, 0, 1)).use_fake_user = True
    group = bpy.data.node_groups.new("InnerGroup", "ShaderNodeTree")
    group.interface.new_socket("Colour", in_out="OUTPUT", socket_type="NodeSocketColor")
    rgb = group.nodes.new("ShaderNodeRGB")
    rgb.outputs[0].default_value = (0, 1, 0, 1)
    leaving = group.nodes.new("NodeGroupOutput")
    group.links.new(rgb.outputs[0], leaving.inputs[0])
    via = an_emission("ViaInner", (0, 0, 0, 1))
    node = via.node_tree.nodes.new("ShaderNodeGroup")
    node.node_tree = group
    via.node_tree.links.new(node.outputs[0], via.node_tree.nodes["Emission"].inputs[0])
    via.use_fake_user = True
elif what == "middle":
    base = args[2]
    with bpy.data.libraries.load(base, link=True, pack=True, relative=True) as (data_from, data_to):
        data_to.materials = ["BaseBlue"]
    plane = a_plane("MiddlePlane", 2.0, data_to.materials[0], scene)
    plane.use_fake_user = True
    describe("middle")
elif what == "main":
    lib, middle = args[2], args[3]
    with bpy.data.libraries.load(lib, link=True, pack=True, relative=True) as (data_from, data_to):
        data_to.materials = ["DirectRed", "ViaInner"]
    direct, through = data_to.materials
    a_plane("Red", -2.0, direct, scene)
    a_plane("Green", 0.0, through, scene)
    with bpy.data.libraries.load(middle, link=True, relative=True) as (data_from, data_to):
        data_to.objects = ["MiddlePlane"]
    scene.collection.objects.link(data_to.objects[0])
    camera_data = bpy.data.cameras.new("Camera")
    camera_data.type = "ORTHO"
    camera_data.ortho_scale = 6.0
    camera = bpy.data.objects.new("Camera", camera_data)
    camera.location = (0, 0, 5)
    scene.collection.objects.link(camera)
    scene.camera = camera
    scene.render.engine = "CYCLES"
    scene.cycles.device = "CPU"
    scene.cycles.samples = 2
    scene.cycles.use_adaptive_sampling = False
    scene.cycles.use_denoising = False
    scene.render.resolution_x = 64
    scene.render.resolution_y = 64
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "BMP"
    scene.render.image_settings.color_mode = "RGB"
    scene.view_settings.view_transform = "Standard"
    scene.frame_start = 1
    scene.frame_end = 1
    describe("main as made")
bpy.ops.wm.save_as_mainfile(filepath=out)
print("SMOOZY-DESK-MADE", what, os.path.getsize(out), flush=True)
"#;

/// **What a saved file holds, read after opening it.**
const WHAT_THE_FILE_HOLDS: &str = r#"
import bpy
print("SMOOZY-DESK-HELD", bpy.data.filepath, bpy.app.version_string, flush=True)
for one in list(bpy.data.materials) + list(bpy.data.node_groups) + list(bpy.data.objects):
    print("SMOOZY-DESK-HELD ", type(one).__name__, one.name, "library", one.library.filepath if one.library else None,
          "packed", one.is_linked_packed, "indirect", one.is_library_indirect, "missing", one.is_missing, flush=True)
for one in bpy.data.libraries:
    print("SMOOZY-DESK-HELD  Library", one.name, one.filepath, "archive", one.is_archive, flush=True)
"#;

fn the_blenders() -> Vec<(&'static str, PathBuf)> {
    [
        ("5.2", "52", "SMOOZY_DESK_BLENDER_52", "/Applications/Blender.app/Contents/MacOS/Blender"),
        ("5.3", "53", "SMOOZY_DESK_BLENDER_53", "/Applications/Blender 5.3b.app/Contents/MacOS/Blender"),
        ("5.4", "54", "SMOOZY_DESK_BLENDER_54", "/Applications/Blender 5.4a.app/Contents/MacOS/Blender"),
    ]
    .into_iter()
    .filter(|(_, short, _, _)| std::env::var("SMOOZY_DESK_ONLY").map_or(true, |only| only == *short))
    .map(|(named, _, variable, usual)| {
        (named, PathBuf::from(std::env::var(variable).unwrap_or_else(|_| usual.to_string())))
    })
    .collect()
}

/// One run of Blender, both streams together, waited on until it ends or
/// [`ONE_RUN_AT_MOST`] passes.
fn run<S: AsRef<OsStr>>(blender: &Path, arguments: &[S]) -> (Option<i32>, String) {
    let mut child = Command::new(blender)
        .args(arguments)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Blender starts");
    let mut stdout = child.stdout.take().expect("its stream");
    let mut stderr = child.stderr.take().expect("its other stream");
    let out = std::thread::spawn(move || {
        let mut all = Vec::new();
        std::io::Read::read_to_end(&mut stdout, &mut all).ok();
        all
    });
    let err = std::thread::spawn(move || {
        let mut all = Vec::new();
        std::io::Read::read_to_end(&mut stderr, &mut all).ok();
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
            panic!("{} ran past {ONE_RUN_AT_MOST:?}", blender.display());
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    let mut said = out.join().expect("the reader ends");
    said.extend(err.join().expect("the reader ends"));
    (status.code(), String::from_utf8_lossy(&said).to_string())
}

fn make(blender: &Path, maker: &Path, what: &str, file: &Path, more: &[&Path]) -> String {
    let mut arguments: Vec<String> = [
        "--factory-startup", "--background", "--python-exit-code", "63", "--python",
    ]
    .iter()
    .map(|one| one.to_string())
    .collect();
    arguments.push(maker.display().to_string());
    arguments.push("--".to_string());
    arguments.push(what.to_string());
    arguments.push(file.display().to_string());
    arguments.extend(more.iter().map(|one| one.display().to_string()));
    let (status, said) = run(blender, &arguments);
    assert_eq!(status, Some(0), "{what} is made: {said}");
    said
}

fn the_held_lines(said: &str) -> Vec<String> {
    said.lines()
        .filter(|line| line.starts_with("SMOOZY-DESK-HELD") || line.starts_with("SMOOZY-DESK-MADE"))
        .map(str::to_string)
        .collect()
}

fn what_it_holds(blender: &Path, lister: &Path, file: &Path) -> Vec<String> {
    let arguments = [
        "--factory-startup".to_string(),
        "--background".to_string(),
        file.display().to_string(),
        "--python".to_string(),
        lister.display().to_string(),
    ];
    let (_, said) = run(blender, &arguments);
    the_held_lines(&said)
}

/// One frame of a file, written as a BMP, its bytes handed back.
fn a_picture_of(blender: &Path, file: &Path, out: &Path) -> Vec<u8> {
    std::fs::create_dir_all(out).expect("a folder");
    let arguments = [
        "--factory-startup".to_string(),
        "--background".to_string(),
        "--python-exit-code".to_string(),
        "63".to_string(),
        file.display().to_string(),
        "--render-output".to_string(),
        out.join("f_").display().to_string(),
        "--render-frame".to_string(),
        "1".to_string(),
    ];
    let (status, said) = run(blender, &arguments);
    std::fs::write(out.join("render.log"), &said).ok();
    assert_eq!(status, Some(0), "{}: {said}", file.display());
    std::fs::read(out.join("f_0001.bmp")).unwrap_or_else(|why| panic!("{}: {why}: {said}", file.display()))
}

/// **Each third of the frame, its mean red, green and blue**, from a BMP Blender
/// wrote uncompressed, rows padded to four bytes.
fn the_three_thirds(bytes: &[u8]) -> Vec<[f64; 3]> {
    let word = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
    let start = word(10) as usize;
    let width = (word(18) as i32).unsigned_abs() as usize;
    let rows = (word(22) as i32).unsigned_abs() as usize;
    let each = u16::from_le_bytes(bytes[28..30].try_into().unwrap()) as usize / 8;
    let stride = (width * each).div_ceil(4) * 4;
    let mut thirds = vec![[0.0f64; 3]; 3];
    let mut counts = [0.0f64; 3];
    for row in rows / 4..rows * 3 / 4 {
        for x in 0..width {
            let third = x * 3 / width;
            // A narrow middle of each third, so an edge between planes counts for nothing.
            let within = (x * 3) % width;
            if within < width / 4 || within > width * 3 / 4 {
                continue;
            }
            let pixel = &bytes[start + row * stride + x * each..];
            thirds[third][0] += pixel[2] as f64;
            thirds[third][1] += pixel[1] as f64;
            thirds[third][2] += pixel[0] as f64;
            counts[third] += 1.0;
        }
    }
    for (third, count) in thirds.iter_mut().zip(counts) {
        for value in third.iter_mut() {
            *value /= count;
        }
    }
    thirds
}

#[test]
#[ignore = "run by hand at the desk with Blender 5.2, 5.3 beta and 5.4 alpha, one at a time"]
fn packed_linked_data_comes_through_the_fixings_save_the_same_picture() {
    let at = std::env::var("SMOOZY_DESK_OUT").map_or_else(
        |_| std::env::temp_dir().join(format!("smoozy-packed-linked-{}", std::process::id())),
        PathBuf::from,
    );
    let at = at.join("packed-linked");
    let mut faults: Vec<String> = Vec::new();
    for (named, blender) in the_blenders() {
        println!("{named}:");
        let here = at.join(named);
        std::fs::create_dir_all(&here).expect("a folder");
        let maker = here.join("make.py");
        std::fs::write(&maker, THE_LIBRARIES_AND_THE_FILE).expect("written");
        let lister = here.join("list.py");
        std::fs::write(&lister, WHAT_THE_FILE_HOLDS).expect("written");
        let stage = here.join("stage");
        let the_files_own = here.join("file");
        let mirror = here.join("mirror");
        let staging = here.join("staging");
        for folder in [&stage, &the_files_own, &mirror, &staging] {
            std::fs::create_dir_all(folder).expect("a folder");
        }
        let base = stage.join("base.blend");
        let lib = stage.join("lib.blend");
        let middle = the_files_own.join("middle.blend");
        let main = the_files_own.join("main.blend");
        make(&blender, &maker, "base", &base, &[]);
        make(&blender, &maker, "lib", &lib, &[]);
        let said = make(&blender, &maker, "middle", &middle, &[&base]);
        for line in the_held_lines(&said) {
            println!("    {line}");
        }
        let said = make(&blender, &maker, "main", &main, &[&lib, &middle]);
        for line in the_held_lines(&said) {
            println!("    {line}");
        }

        // **The libraries that were packed go out of reach**, as they are on a
        // rented machine: from here on only what the files hold can answer.
        std::fs::rename(&stage, here.join("stage-out-of-reach")).expect("moved away");

        // **`middle.blend` gets its own fixed copy, as M14 does from the bottom
        // up**, no pair needed, so the blue plane reaches the picture only
        // through the save under test.
        let script = here.join("fixing-middle.py");
        let the_fixed_middle = mirror.join("middle.blend");
        std::fs::write(&script, the_fixing_python(&[], the_fixed_middle.as_os_str().as_bytes())).expect("written");
        let words = the_words_of_the_fixing(middle.as_os_str().as_bytes(), script.as_os_str().as_bytes());
        let words: Vec<&OsStr> = words.iter().map(|word| OsStr::from_bytes(word)).collect();
        let (status, said) = run(&blender, &words);
        std::fs::write(here.join("fixing-middle.log"), &said).ok();
        println!("    the fixing of middle.blend ended at {status:?}");
        if status != Some(0) {
            faults.push(format!("{named}: the fixing of middle.blend ended at {status:?}: {said}"));
            continue;
        }
        println!("    middle.blend's fixed copy, reopened:");
        for line in what_it_holds(&blender, &lister, &the_fixed_middle) {
            println!("    {line}");
        }

        println!("    the file, reopened:");
        for line in what_it_holds(&blender, &lister, &main) {
            println!("    {line}");
        }
        let before = a_picture_of(&blender, &main, &here.join("before"));

        // **M14's script and launch line, with one real pair.**
        let pair = compose_pair(
            Kind::Library,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(b"//middle.blend"),
            NewLocation::AFixedCopy(MachineLocation::composed(
                mirror.join("middle.blend").as_os_str().as_bytes().to_vec(),
            )),
        )
        .expect("the pair composes");
        let the_copy = staging.join("main.blend");
        let script = here.join("fixing.py");
        std::fs::write(&script, the_fixing_python(&[pair], the_copy.as_os_str().as_bytes())).expect("written");
        let words = the_words_of_the_fixing(main.as_os_str().as_bytes(), script.as_os_str().as_bytes());
        let words: Vec<&OsStr> = words.iter().map(|word| OsStr::from_bytes(word)).collect();
        let (status, said) = run(&blender, &words);
        std::fs::write(here.join("fixing.log"), &said).ok();
        println!("    the fixing ended at {status:?}");
        for line in said.lines().filter(|line| line.starts_with("SMOOZY-REMAP-PAIR")) {
            println!("    {}", line.chars().take(240).collect::<String>());
        }
        if status != Some(0) {
            faults.push(format!("{named}: the fixing ended at {status:?}: {said}"));
            continue;
        }
        println!(
            "    sizes: the file {} bytes, the copy {} bytes",
            std::fs::metadata(&main).expect("there").len(),
            std::fs::metadata(&the_copy).expect("there").len()
        );

        // **Only the pair can find the regular link now.**
        std::fs::rename(&middle, the_files_own.join("middle.blend.moved")).expect("moved away");
        println!("    the copy, reopened:");
        for line in what_it_holds(&blender, &lister, &the_copy) {
            println!("    {line}");
        }
        let after = a_picture_of(&blender, &the_copy, &here.join("after"));

        // **What the copy carries by itself**, every library out of reach. This
        // is the release note's own question; no render hangs on it.
        std::fs::rename(&mirror, here.join("mirror-out-of-reach")).expect("moved away");
        println!("    the copy, reopened with no library in reach:");
        for line in what_it_holds(&blender, &lister, &the_copy) {
            println!("    {line}");
        }

        let thirds = the_three_thirds(&before);
        println!("    the file's picture, each third's mean red, green, blue: {thirds:?}");
        println!("    the copy's picture: {:?}", the_three_thirds(&after));
        for (third, (colour, channel)) in [("red", 0usize), ("green", 1), ("blue", 2)].into_iter().enumerate() {
            let shown = thirds[third];
            let others = (0..3).filter(|one| *one != channel).map(|one| shown[one]).fold(0.0, f64::max);
            if shown[channel] < 100.0 || others > 30.0 {
                faults.push(format!("{named}: the {colour} plane drew {shown:?}"));
            }
        }
        let same = before == after;
        println!("    the two pictures are the same, byte for byte: {same}");
        if !same {
            faults.push(format!("{named}: the fixing changed the picture"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **The 5.3 integration's piece 3: the check, the fixing and the remap on a file
/// with two effects.** Each reaches every effect's group and none of them puts a
/// line of Smoozy's own making on either stream (`Job files and outputs` R4 item
/// 1). 5.2 has no stack, so it is left out. The remap is run on its own, with no
/// pairs, as the render's material runs it; the check's document names the
/// scene as the owner of the second effect's group's import node.
/// **Every Blender the check runs in**, for the two tests of the live pass on
/// real production files.
fn every_blender() -> Vec<(&'static str, PathBuf)> {
    [
        ("4.5", "45", "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"),
        ("5.0", "50", "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"),
        ("5.1", "51", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
        ("5.2", "52", "/Applications/Blender.app/Contents/MacOS/Blender"),
        ("5.3", "53", "/Applications/Blender 5.3b.app/Contents/MacOS/Blender"),
        ("5.4", "54", "/Applications/Blender 5.4a.app/Contents/MacOS/Blender"),
    ]
    .into_iter()
    .filter(|(_, short, _)| std::env::var("SMOOZY_DESK_ONLY").map_or(true, |only| only == *short))
    .map(|(named, _, usual)| (named, PathBuf::from(usual)))
    .collect()
}

/// **What the check's own script reads of a file**: each reference as its kind,
/// what Blender read, and the file's own text where one was found.
fn the_references_of(blender: &Path, file: &Path, here: &Path) -> Vec<(String, String, Option<String>)> {
    use smoozy_dependencies_machine::check_script::{the_check_readings_script, the_words_of_the_check_reading};
    let empty = here.join("nothing.py");
    std::fs::write(&empty, "").expect("written");
    let script = here.join("check.py");
    std::fs::write(&script, the_check_readings_script()).expect("written");
    let words = the_words_of_the_check_reading(file.as_os_str().as_bytes(), empty.as_os_str().as_bytes(), script.as_os_str().as_bytes());
    let words: Vec<&OsStr> = words.iter().map(|word| OsStr::from_bytes(word)).collect();
    let (status, said) = run(blender, &words);
    assert_eq!(status, Some(0), "the check: {said}");
    let opens = said.find("<<<SMOOZY-DEPENDENCIES>>>").expect("the document") + "<<<SMOOZY-DEPENDENCIES>>>".len();
    let closes = said[opens..].find("<<<END-SMOOZY-DEPENDENCIES>>>").expect("its end") + opens;
    let document: serde_json::Value = serde_json::from_str(&said[opens..closes]).expect("it reads");
    let as_text = |value: &serde_json::Value| {
        String::from_utf8_lossy(&value.as_array().expect("bytes").iter().map(|one| one.as_u64().unwrap() as u8).collect::<Vec<u8>>()).to_string()
    };
    document["references"]
        .as_array()
        .expect("the references")
        .iter()
        .map(|one| {
            (
                one["kind"].as_str().unwrap_or("").to_string(),
                as_text(&one["recorded"]),
                one.get("the_files_own_text").filter(|own| !own.is_null()).map(as_text),
            )
        })
        .collect()
}

const A_LIBRARY_AND_A_FILE_LINKING_IT: &str = r#"
import bpy, sys
args = sys.argv[sys.argv.index("--") + 1:]
what, out = args[0], args[1]
bpy.ops.wm.read_factory_settings(use_empty=True)
if what == "lib":
    # **An object using both**, so linking it brings them, as a rig brings its
    # script on the real file: a linked item with no user is not kept.
    mesh = bpy.data.meshes.new("M")
    one = bpy.data.objects.new("O", mesh)
    bpy.context.scene.collection.objects.link(one)
    text = bpy.data.texts.new("rig.py")
    one["its_script"] = text
    image = bpy.data.images.new("Pic", 4, 4)
    image.source = "FILE"
    image.filepath = "//its_own_picture.png"
    material = bpy.data.materials.new("Mat")
    material.use_nodes = True
    material.node_tree.nodes.new("ShaderNodeTexImage").image = image
    mesh.materials.append(material)
    bpy.ops.wm.save_as_mainfile(filepath=out)
else:
    bpy.ops.wm.save_as_mainfile(filepath=out)
    with bpy.data.libraries.load(args[2], link=True, relative=True) as (data_from, data_to):
        data_to.objects = ["O"]
    bpy.context.scene.collection.objects.link(data_to.objects[0])
    if what == "packed":
        print("SMOOZY-DESK-PACKED", bpy.ops.file.pack_libraries(), flush=True)
    compress = what != "plain"
    bpy.ops.wm.save_as_mainfile(filepath=out, compress=compress, relative_remap=False)
    print("SMOOZY-DESK-HELD", bpy.app.version_string, [(one.name, one.filepath, bool(one.packed_file)) for one in bpy.data.libraries], flush=True)
"#;

/// **`Dependency Upload & Remap` G6c, in every version**: a packed library's text
/// block is no row and its own location is no row, while the linked picture's
/// own outside file still is. **The control**: not packed, the library is its
/// row, of its own kind, and the text block is no row. Read through the check's
/// own script. **And the real file**, where `SMOOZY_DESK_REAL_FILE` names it.
/// **`Dependency Upload & Remap` G6e, on every Blender.** Text in Blender's own
/// font is no reference; text in a font of its own still is.
#[test]
#[ignore = "run by hand at the desk with every Blender, one at a time"]
fn blenders_own_font_is_no_row_at_the_desk() {
    let out = PathBuf::from(std::env::var("SMOOZY_DESK_OUT").expect("SMOOZY_DESK_OUT names a scratch folder"));
    for (name, blender) in every_blender() {
        let here = out.join(format!("font-{name}"));
        std::fs::create_dir_all(&here).expect("the folder");
        let file = here.join("text.blend");
        let maker = here.join("make.py");
        std::fs::write(
            &maker,
            format!(
                "import bpy\nc = bpy.data.curves.new('T', type='FONT')\nbpy.data.objects.new('T', c)\n\
                 bpy.context.scene.collection.objects.link(bpy.data.objects['T'])\n\
                 print('FONT', c.font.filepath)\nbpy.ops.wm.save_as_mainfile(filepath={:?})\n",
                file.to_str().expect("text")
            ),
        )
        .expect("the maker");
        let (status, said) = run(&blender, &["-b", "--factory-startup", "--python-exit-code", "1", "--python", maker.to_str().expect("text")]);
        assert_eq!(status, Some(0), "{said}");
        assert!(said.contains("FONT <builtin>"), "{name}: how the font records itself: {said}");
        let references = the_references_of(&blender, &file, &here);
        println!("{name}: {references:?}");
        assert!(
            references.iter().all(|one| one.1 != "<builtin>" && one.2.as_deref() != Some("<builtin>") && one.0 != "Font"),
            "{name}: {references:?}"
        );
    }
}

#[test]
#[ignore = "run by hand at the desk with every Blender, one at a time"]
fn a_packed_librarys_text_block_is_no_row_at_the_desk() {
    let out = std::env::var("SMOOZY_DESK_OUT").map(PathBuf::from).unwrap_or_else(|_| std::env::temp_dir().join("smoozy-packed-text"));
    let mut faults = Vec::new();
    for (named, blender) in every_blender() {
        println!("{named}:");
        let here = out.join("packed-text").join(named);
        std::fs::create_dir_all(&here).expect("a folder");
        let maker = here.join("make.py");
        std::fs::write(&maker, A_LIBRARY_AND_A_FILE_LINKING_IT).expect("written");
        let lib = here.join("lib.blend");
        make(&blender, &maker, "lib", &lib, &[]);
        let packed = here.join("packed.blend");
        let said = make(&blender, &maker, "packed", &packed, &[&lib]);
        println!("    {}", said.lines().filter(|line| line.starts_with("SMOOZY-DESK")).collect::<Vec<_>>().join(" | "));
        let linked = here.join("linked.blend");
        make(&blender, &maker, "linked", &linked, &[&lib]);
        let the_packed = the_references_of(&blender, &packed, &here);
        let the_linked = the_references_of(&blender, &linked, &here);
        println!("    packed: {the_packed:?}");
        println!("    not packed: {the_linked:?}");
        if the_packed.iter().any(|(_, recorded, _)| recorded.ends_with("lib.blend")) {
            faults.push(format!("{named}: a packed library's location is a row: {the_packed:?}"));
        }
        if !the_packed.iter().any(|(kind, recorded, _)| kind == "Image" && recorded.ends_with("its_own_picture.png")) {
            faults.push(format!("{named}: the linked picture's own file is gone: {the_packed:?}"));
        }
        if !the_linked.iter().any(|(kind, recorded, _)| kind == "Library" && recorded.ends_with("lib.blend")) {
            faults.push(format!("{named}: a library not packed is no row: {the_linked:?}"));
        }
        if the_linked.iter().any(|(kind, _, _)| kind == "Text") {
            faults.push(format!("{named}: a text row: {the_linked:?}"));
        }
    }
    if let Ok(real) = std::env::var("SMOOZY_DESK_REAL_FILE") {
        let (_, blender) = every_blender().into_iter().find(|(named, _)| *named == "5.3").expect("5.3");
        let here = out.join("packed-text").join("real");
        std::fs::create_dir_all(&here).expect("a folder");
        let the_real = the_references_of(&blender, Path::new(&real), &here);
        println!("the real file: {the_real:?}");
        if the_real.iter().any(|(kind, _, _)| kind == "Text" || kind == "Library") {
            faults.push(format!("the real file: {the_real:?}"));
        }
        if !the_real.iter().any(|(kind, recorded, _)| kind == "Image" && recorded.ends_with("cowboi_0001.jpg")) {
            faults.push(format!("the real file's truly missing picture is gone: {the_real:?}"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **`Dependency Upload & Remap` G6d, in every version, compressed and not.** A
/// file saved at a depth holding `//../../../lib.blend`, opened from a folder
/// two names under the root: Blender reads a shorter location, the file's own
/// text comes back beside it, and from a deep folder Blender reads what the file
/// holds, so nothing is added. Needs `TMPDIR=/tmp`, the folder's depth being the
/// case.
#[test]
#[ignore = "run by hand at the desk with every Blender, one at a time, with TMPDIR=/tmp"]
fn a_librarys_location_is_read_as_the_file_writes_it() {
    let the_top = std::env::temp_dir().join(format!("smoozy-l-{}", std::process::id()));
    assert_eq!(the_top.components().count(), 3, "two names under the root: {}", the_top.display());
    let mut faults = Vec::new();
    for (named, blender) in every_blender() {
        println!("{named}:");
        for form in ["linked", "plain"] {
            let deep = the_top.join("a").join("b").join("c");
            std::fs::create_dir_all(&deep).expect("a folder");
            let maker = deep.join("make.py");
            std::fs::write(&maker, A_LIBRARY_AND_A_FILE_LINKING_IT).expect("written");
            let lib = the_top.join("lib.blend");
            make(&blender, &maker, "lib", &lib, &[]);
            let saved = deep.join("job.blend");
            let said = make(&blender, &maker, form, &saved, &[&lib]);
            println!("    {form}: {}", said.lines().filter(|line| line.starts_with("SMOOZY-DESK")).collect::<Vec<_>>().join(" | "));
            let every = the_references_of(&blender, &saved, &deep);
            println!("    {form}: every reference: {every:?}");
            let from_deep: Vec<_> = every
                .into_iter()
                .filter(|(kind, _, _)| kind == "Library")
                .collect();
            println!("    {form}, from its own folder: {from_deep:?}");
            if from_deep != vec![("Library".to_string(), "//../../../lib.blend".to_string(), None)] {
                faults.push(format!("{named}, {form}, from its own folder: {from_deep:?}"));
            }
            let shallow = the_top.join(format!("shallow-{named}-{form}.blend"));
            std::fs::copy(&saved, &shallow).expect("a copy");
            let from_shallow: Vec<_> = the_references_of(&blender, &shallow, &the_top)
                .into_iter()
                .filter(|(kind, _, _)| kind == "Library")
                .collect();
            println!("    {form}, two names under the root: {from_shallow:?}");
            if from_shallow
                != vec![(
                    "Library".to_string(),
                    "//../../lib.blend".to_string(),
                    Some("//../../../lib.blend".to_string()),
                )]
            {
                faults.push(format!("{named}, {form}, two names under the root: {from_shallow:?}"));
            }
        }
    }
    let _ = std::fs::remove_dir_all(&the_top);
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **The chain below.** `c` holds a material naming a picture, `b` a mesh using
/// that material, `a` an object using that mesh, and the top file links the
/// object, so opening the top file reaches all three.
const THREE_LEVELS_OF_LIBRARIES: &str = r#"
import bpy, sys
args = sys.argv[sys.argv.index("--") + 1:]
what, out = args[0], args[1]
bpy.ops.wm.read_factory_settings(use_empty=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
scene = bpy.context.scene
if what == "c":
    image = bpy.data.images.new("Pic", 4, 4)
    image.source = "FILE"
    image.filepath = "//c_picture.png"
    material = bpy.data.materials.new("Mc")
    material.use_nodes = True
    material.node_tree.nodes.new("ShaderNodeTexImage").image = image
    material.use_fake_user = True
elif what == "b":
    with bpy.data.libraries.load(args[2], link=True, relative=True) as (data_from, data_to):
        data_to.materials = ["Mc"]
    mesh = bpy.data.meshes.new("Mb")
    mesh.materials.append(data_to.materials[0])
    mesh.use_fake_user = True
elif what == "a":
    with bpy.data.libraries.load(args[2], link=True, relative=True) as (data_from, data_to):
        data_to.meshes = ["Mb"]
    one = bpy.data.objects.new("Oa", data_to.meshes[0])
    scene.collection.objects.link(one)
else:
    with bpy.data.libraries.load(args[2], link=True, relative=True) as (data_from, data_to):
        data_to.objects = ["Oa"]
    scene.collection.objects.link(data_to.objects[0])
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

/// **Where a location read in `file` is, measured from that file's own folder**:
/// the rule the desktop's `where_the_render_finds_it` keeps.
fn where_it_is_measured_from(file: &Path, recorded: &str) -> PathBuf {
    let mut at: Vec<std::path::Component> = Vec::new();
    let relative = recorded.strip_prefix("//").expect("a relative location");
    let folder = file.parent().expect("a folder");
    for part in folder.components().chain(Path::new(relative).components()) {
        match part {
            std::path::Component::ParentDir => {
                at.pop();
            }
            std::path::Component::CurDir => {}
            other => at.push(other),
        }
    }
    at.iter().collect()
}

/// **`Dependency Upload & Remap` G17b's ground, in every version.** A top file,
/// and three levels of libraries sent one at a time, the top one first. Read
/// through the check's own script each time: every library that is there brings
/// the next one's Library reference into the top file's reading, an absent one
/// is named and nothing below it is, and each location, measured from the top
/// file's own folder, is where that library really sits. Needs `TMPDIR=/tmp`.
#[test]
#[ignore = "run by hand at the desk with every Blender, one at a time"]
fn a_files_reading_grows_as_its_libraries_land_at_the_desk() {
    let out = std::env::var("SMOOZY_DESK_OUT").map(PathBuf::from).unwrap_or_else(|_| std::env::temp_dir().join("smoozy-three-levels"));
    let mut faults = Vec::new();
    for (named, blender) in every_blender() {
        println!("{named}:");
        let here = out.join("three-levels").join(named);
        let _ = std::fs::remove_dir_all(&here);
        let (shot, libs, deep, other, away) =
            (here.join("shot"), here.join("shot/libs"), here.join("shot/libs/deep"), here.join("other"), here.join("away"));
        for folder in [&deep, &other, &away] {
            std::fs::create_dir_all(folder).expect("a folder");
        }
        let here = std::fs::canonicalize(&here).expect("the folder");
        let (shot, libs, deep, other, away) = (
            std::fs::canonicalize(&shot).unwrap(),
            std::fs::canonicalize(&libs).unwrap(),
            std::fs::canonicalize(&deep).unwrap(),
            std::fs::canonicalize(&other).unwrap(),
            std::fs::canonicalize(&away).unwrap(),
        );
        let maker = here.join("make.py");
        std::fs::write(&maker, THREE_LEVELS_OF_LIBRARIES).expect("written");
        let (top, a, b, c) = (shot.join("top.blend"), libs.join("a.blend"), deep.join("b.blend"), other.join("c.blend"));
        make(&blender, &maker, "c", &c, &[]);
        make(&blender, &maker, "b", &b, &[&c]);
        make(&blender, &maker, "a", &a, &[&b]);
        make(&blender, &maker, "top", &top, &[&a]);
        let the_three = [&a, &b, &c];
        for (at, one) in the_three.iter().enumerate() {
            std::fs::rename(one, away.join(format!("{at}.blend"))).expect("taken away");
        }
        // **Read with none there, then as each lands.**
        for landed in 0..=3 {
            if landed > 0 {
                std::fs::rename(away.join(format!("{}.blend", landed - 1)), the_three[landed - 1]).expect("it lands");
            }
            let references = the_references_of(&blender, &top, &here);
            println!("    {landed} there: {references:?}");
            let libraries: Vec<PathBuf> = references
                .iter()
                .filter(|(kind, _, _)| kind == "Library")
                .map(|(_, recorded, _)| where_it_is_measured_from(&top, recorded))
                .collect();
            let wanted: Vec<PathBuf> = the_three[..(landed + 1).min(3)].iter().map(|one| (*one).clone()).collect();
            let mut sorted = libraries.clone();
            sorted.sort();
            let mut wanted_sorted = wanted.clone();
            wanted_sorted.sort();
            if sorted != wanted_sorted {
                faults.push(format!("{named}, {landed} there: named {libraries:?}, wanted {wanted:?}"));
            }
            let the_picture = references.iter().any(|(kind, recorded, _)| kind == "Image" && recorded.ends_with("c_picture.png"));
            if the_picture != (landed == 3) {
                faults.push(format!("{named}, {landed} there: the third's picture named: {the_picture}"));
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta and 5.4 alpha, one at a time"]
fn the_check_the_fixing_and_the_remap_read_every_effect_and_warn_of_nothing() {
    use smoozy_dependencies_machine::check_script::{the_check_readings_script, the_words_of_the_check_reading};
    use smoozy_dependencies_machine::material::the_material_python;
    use smoozy_dependencies_machine::remap::RemapMaterial;
    use smoozy_render_settings_machine::file_half::BlenderIdentity;
    use smoozy_server_setup_machine::blender::Version;
    use smoozy_state::identities::{Identity, IdentityKind};

    const TWO_EFFECTS: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
scene = bpy.context.scene
for named in ("First", "Second"):
    effect = scene.compositor_effects.new(named)
    tree = bpy.data.node_groups.new("G " + named, "CompositorNodeTree")
    tree.nodes.new("CompositorNodeRLayers")
    effect.node_group = tree
scene.compositor_effects.active = scene.compositor_effects[0]
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;
    let out = std::env::var("SMOOZY_DESK_OUT").map(PathBuf::from).unwrap_or_else(|_| std::env::temp_dir().join("smoozy-effects-check"));
    let mut faults = Vec::new();
    for (named, blender) in the_blenders() {
        if named == "5.2" {
            continue;
        }
        println!("{named}:");
        let here = out.join("effects").join(named);
        std::fs::create_dir_all(&here).expect("a folder");
        let maker = here.join("make.py");
        std::fs::write(&maker, TWO_EFFECTS).expect("written");
        let file = here.join("job.blend");
        let (status, said) = run(&blender, &[
            OsStr::new("-b"),
            OsStr::new("--factory-startup"),
            OsStr::new("--python-exit-code"),
            OsStr::new("63"),
            OsStr::new("--python"),
            maker.as_os_str(),
            OsStr::new("--"),
            file.as_os_str(),
        ]);
        assert_eq!(status, Some(0), "{said}");

        let empty = here.join("nothing.py");
        std::fs::write(&empty, "").expect("written");
        let script = here.join("check.py");
        std::fs::write(&script, the_check_readings_script()).expect("written");
        let words = the_words_of_the_check_reading(file.as_os_str().as_bytes(), empty.as_os_str().as_bytes(), script.as_os_str().as_bytes());
        let words: Vec<&OsStr> = words.iter().map(|word| OsStr::from_bytes(word)).collect();
        let (status, check_said) = run(&blender, &words);
        std::fs::write(here.join("check.log"), &check_said).ok();
        println!("    the check ended at {status:?}");

        let script = here.join("fixing.py");
        std::fs::write(&script, the_fixing_python(&[], here.join("fixed.blend").as_os_str().as_bytes())).expect("written");
        let words = the_words_of_the_fixing(file.as_os_str().as_bytes(), script.as_os_str().as_bytes());
        let words: Vec<&OsStr> = words.iter().map(|word| OsStr::from_bytes(word)).collect();
        let (fixing_status, fixing_said) = run(&blender, &words);
        std::fs::write(here.join("fixing.log"), &fixing_said).ok();
        println!("    the fixing ended at {fixing_status:?}");

        let material = RemapMaterial::composed(
            &Identity::issue(IdentityKind::Job).expect("randomness"),
            &BlenderIdentity {
                path: blender.display().to_string(),
                version: Version { major: 5, minor: if named == "5.3" { 3 } else { 4 }, patch: 0 },
            },
            Vec::new(),
        );
        let script = here.join("remap.py");
        std::fs::write(&script, format!("import bpy, json, os, sys\n{}", the_material_python(&material))).expect("written");
        let (remap_status, remap_said) = run(&blender, &[
            OsStr::new("-b"),
            OsStr::new("--factory-startup"),
            file.as_os_str(),
            OsStr::new("--python-exit-code"),
            OsStr::new("1"),
            OsStr::new("--python"),
            script.as_os_str(),
        ]);
        std::fs::write(here.join("remap.log"), &remap_said).ok();
        println!("    the remap ended at {remap_status:?}");
        for (what, ended, said) in [("the check", status, &check_said), ("the fixing", fixing_status, &fixing_said), ("the remap", remap_status, &remap_said)] {
            if ended != Some(0) {
                faults.push(format!("{named}: {what} ended at {ended:?}: {said}"));
            }
            if said.contains("DeprecationWarning") || said.contains("compositing_node_group") {
                faults.push(format!("{named}: {what} warned: {said}"));
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

const A_FILE_NAMED_BY_WHAT_IT_LINKS: &str = r#"
import bpy, os, sys
args = sys.argv[sys.argv.index("--") + 1:]
what, out = args[0], args[1]
if what == "top-first":
    bpy.ops.wm.read_factory_settings(use_empty=True)
    mesh = bpy.data.meshes.new("TopM")
    one = bpy.data.objects.new("TopCube", mesh)
    bpy.context.scene.collection.objects.link(one)
    image = bpy.data.images.new("TopPic", 4, 4)
    image.source = "FILE"
    image.filepath = "//tex/top.png"
    image.use_fake_user = True
    bpy.ops.wm.save_as_mainfile(filepath=out)
elif what == "lib":
    bpy.ops.wm.read_factory_settings(use_empty=True)
    mesh = bpy.data.meshes.new("LibM")
    one = bpy.data.objects.new("LibSphere", mesh)
    bpy.context.scene.collection.objects.link(one)
    bpy.ops.wm.save_as_mainfile(filepath=out)
    with bpy.data.libraries.load(args[2], link=True, relative=True) as (data_from, data_to):
        data_to.objects = ["TopCube"]
    bpy.context.scene.collection.objects.link(data_to.objects[0])
    bpy.ops.wm.save_mainfile()
else:
    bpy.ops.wm.open_mainfile(filepath=out)
    with bpy.data.libraries.load(args[2], link=True, relative=True) as (data_from, data_to):
        data_to.objects = ["LibSphere"]
    bpy.context.scene.collection.objects.link(data_to.objects[0])
    bpy.ops.wm.save_mainfile()
print("SMOOZY-DESK-MADE", what, [(l.name, l.filepath) for l in bpy.data.libraries], flush=True)
"#;

/// **The second Sesterce pass's *Uploaded; remap failed*: a file named by what it
/// links.** `top.blend` links `lib/a.blend`, and `a.blend` links `top.blend`. Read
/// through the check's own script, and fixed through M14's own script and launch
/// line, with the pairs M10 would give. Prints what each said; the account says
/// what it found. Needs `TMPDIR=/tmp`.
#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta"]
fn a_file_named_by_what_it_links_is_fixed_at_the_desk() {
    let out = std::env::var("SMOOZY_DESK_OUT").map(PathBuf::from).unwrap_or_else(|_| std::env::temp_dir().join("smoozy-a-loop"));
    let blender = PathBuf::from("/Applications/Blender 5.3b.app/Contents/MacOS/Blender");
    let here = out.join("a-loop");
    let _ = std::fs::remove_dir_all(&here);
    std::fs::create_dir_all(here.join("shot/lib")).expect("a folder");
    std::fs::create_dir_all(here.join("shot/tex")).expect("a folder");
    std::fs::create_dir_all(here.join("fixed")).expect("a folder");
    let here = std::fs::canonicalize(&here).expect("the folder");
    std::fs::write(here.join("shot/tex/top.png"), b"not a picture").expect("written");
    let maker = here.join("make.py");
    std::fs::write(&maker, A_FILE_NAMED_BY_WHAT_IT_LINKS).expect("written");
    let (top, a) = (here.join("shot/top.blend"), here.join("shot/lib/a.blend"));
    println!("{}", make(&blender, &maker, "top-first", &top, &[]).lines().filter(|l| l.contains("SMOOZY-DESK")).collect::<String>());
    println!("{}", make(&blender, &maker, "lib", &a, &[&top]).lines().filter(|l| l.contains("SMOOZY-DESK")).collect::<String>());
    println!("{}", make(&blender, &maker, "top", &top, &[&a]).lines().filter(|l| l.contains("SMOOZY-DESK")).collect::<String>());
    let of_top = the_references_of(&blender, &top, &here);
    println!("top's reading: {of_top:?}");
    let of_a = the_references_of(&blender, &a, &here);
    println!("a's reading: {of_a:?}");
    for (named, file, references) in [("top", &top, &of_top), ("a", &a, &of_a)] {
        let pairs: Vec<_> = references
            .iter()
            .filter(|(kind, recorded, _)| kind == "Image" || (kind == "Library" && recorded.contains(if named == "top" { "a.blend" } else { "top.blend" })))
            .map(|(kind, recorded, _)| {
                let new = here.join("fixed").join(recorded.rsplit('/').next().unwrap());
                let new = MachineLocation::composed(new.as_os_str().as_bytes().to_vec());
                compose_pair(
                    if kind == "Image" { Kind::Image } else { Kind::Library },
                    "filepath",
                    &RecordedLocation::as_the_reading_carried_it(recorded.as_bytes()),
                    if kind == "Image" { NewLocation::AMirror(new) } else { NewLocation::AFixedCopy(new) },
                )
                .expect("the pair composes")
            })
            .collect();
        let the_copy = here.join("fixed").join(format!("{named}.blend"));
        let script = here.join(format!("fixing-{named}.py"));
        std::fs::write(&script, the_fixing_python(&pairs, the_copy.as_os_str().as_bytes())).expect("written");
        let words = the_words_of_the_fixing(file.as_os_str().as_bytes(), script.as_os_str().as_bytes());
        let words: Vec<&OsStr> = words.iter().map(|word| OsStr::from_bytes(word)).collect();
        let (status, said) = run(&blender, &words);
        println!("the fixing of {named} ended at {status:?}, {} pairs", pairs.len());
        for line in said.lines() {
            println!("    {}", line.chars().take(220).collect::<String>());
        }
        println!("    the copy is there: {}", the_copy.exists());
    }
}

/// **The same on a real file**, its pairs from a list of kind, setting and the
/// reference in hexadecimal, a line each: `SMOOZY_DESK_REAL_FILE` and
/// `SMOOZY_DESK_REAL_PAIRS`. Says which pairs did not apply.
#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta, on a real file"]
fn a_real_files_pairs_are_tried_at_the_desk() {
    let file = PathBuf::from(std::env::var("SMOOZY_DESK_REAL_FILE").expect("the file"));
    let list = std::fs::read_to_string(std::env::var("SMOOZY_DESK_REAL_PAIRS").expect("the pairs")).expect("it reads");
    let here = PathBuf::from(std::env::var("SMOOZY_DESK_OUT").expect("a scratch folder"));
    std::fs::create_dir_all(&here).expect("a folder");
    let blender = PathBuf::from("/Applications/Blender 5.3b.app/Contents/MacOS/Blender");
    let pairs: Vec<_> = list
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            let (kind, setting, reference) = (parts.next()?, parts.next()?, parts.next()?);
            let reference: Vec<u8> = (0..reference.len())
                .step_by(2)
                .map(|at| u8::from_str_radix(&reference[at..at + 2], 16).expect("hexadecimal"))
                .collect();
            let kind: Kind = serde_json::from_value(serde_json::Value::String(kind.to_string())).expect("a kind");
            let new = MachineLocation::composed(here.join("nowhere.bin").as_os_str().as_bytes().to_vec());
            let new = if kind == Kind::Library { NewLocation::AFixedCopy(new) } else { NewLocation::AMirror(new) };
            compose_pair(kind, setting, &RecordedLocation::as_the_reading_carried_it(&reference), new).ok()
        })
        .collect();
    println!("{} pairs", pairs.len());
    let the_copy = here.join("the-copy.blend");
    let script = here.join("fixing.py");
    std::fs::write(&script, the_fixing_python(&pairs, the_copy.as_os_str().as_bytes())).expect("written");
    let words = the_words_of_the_fixing(file.as_os_str().as_bytes(), script.as_os_str().as_bytes());
    let words: Vec<&OsStr> = words.iter().map(|word| OsStr::from_bytes(word)).collect();
    let began = Instant::now();
    let (status, said) = run(&blender, &words);
    std::fs::write(here.join("fixing.log"), &said).ok();
    println!("ended at {status:?} in {:?}", began.elapsed());
    let applied = said.lines().filter(|line| line.starts_with("SMOOZY-REMAP-PAIR") && line.contains("\"applied\": true")).count();
    let not = said.lines().filter(|line| line.starts_with("SMOOZY-REMAP-PAIR") && line.contains("\"applied\": false")).count();
    println!("applied {applied}, not applied {not}, the copy is there: {}", the_copy.exists());
    for line in said.lines().filter(|line| line.contains("Traceback") || line.contains("Error")).take(20) {
        println!("    {}", line.chars().take(200).collect::<String>());
    }
}

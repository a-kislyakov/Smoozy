//! **A picture stored as a sequence, and a colour name the reading Blender
//! lacks, through the check's own script — `Dependency Upload & Remap`,
//! *Colour spaces and image sequences*, kept in the tree and ignored.** Run by
//! hand at the desk, one Blender at a time:
//!
//! ```text
//! SMOOZY_DESK_OUT=<scratch folder> cargo test -p smoozy-dependencies-machine --test colour_and_sequences_at_the_desk -- --ignored --nocapture
//! ```
//!
//! `SMOOZY_DESK_ONLY=45` (or `50`, `51`, `52`, `53`) narrows it to one Blender.
//! **Blender's own settings folders, its temporary folder and the two colour
//! variables are pointed into the scratch folder or cleared for every run**, so
//! nothing of the Mac's own is read or written. **This crate's own code starts
//! no process**; this file is a test of it, not part of it.
//!
//! **The sequence.** A file made in each Blender holds a picture `q_0001.png`
//! stored as `SEQUENCE`, shown by two Image Texture nodes (Start 5, Offset 2,
//! Frames 6, the second Cyclic) and by a compositor Image node (Start 1,
//! Offset 5, Frames 20) — the research's `k1`, `k2` and `k9`'s right half — and
//! a second picture shown by a world's Environment Texture node alone. **The
//! check's own script** must carry the three users and their settings for the
//! first and *another node* for the second; the files they need over frames 1
//! to 15 must be test 5's (`q_0002` to `q_0020`); and the name of each number
//! must be the name Blender's own `filepath_from_user` gives for it.
//!
//! **The colour names.** A copy of each Blender's own colour configuration with
//! a colour space `Smoozy Orphan`, a view `Smoozy View` and a display `Smoozy
//! Display` added, and `Linear Rec.2020` renamed `Smoozy Renamed Rec.2020` (the
//! research's `make_ocio.py`, cut down). Files made under it (through
//! `BLENDER_OCIO`, or `OCIO` in 4.5, which does not act on the first — test 1)
//! are read by the check's own script under Blender's own configuration, and
//! [`the_colour_names_not_found`] over the two streams must name what each
//! lacks. The renamed space keeps its interop ID where the file has one, so 5.3
//! matches it and nothing is said; below 5.3 it is said.
//!
//! **What it does not prove**: Linux, Windows, a window, a render. The account is
//! `Audits/Build/What if — the answers/Colour spaces and image sequences, built —
//! what landed.md`.

// Mac and Linux only: it reads a path's bytes the Unix way, and runs at the desk.
#![cfg(unix)]

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use smoozy_dependencies_machine::check::{
    the_colour_names_not_found, the_file_a_number_reads, the_files_a_sequence_needs, ASequencesUser,
    TheSequencesUsers, WhichColourName,
};
use smoozy_dependencies_machine::check_script::{the_check_readings_script, the_words_of_the_check_reading};

fn the_blenders() -> Vec<(&'static str, PathBuf)> {
    [
        ("4.5", "45", "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"),
        ("5.0", "50", "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"),
        ("5.1", "51", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
        ("5.2", "52", "/Applications/Blender 5.2.4.app/Contents/MacOS/Blender"),
        ("5.3", "53", "/Applications/Blender 5.3.app/Contents/MacOS/Blender"),
    ]
    .into_iter()
    .filter(|(_, short, _)| std::env::var("SMOOZY_DESK_ONLY").map_or(true, |only| only == *short))
    .map(|(named, _, usual)| (named, PathBuf::from(usual)))
    .collect()
}

/// One run of Blender, its two streams kept apart, with its settings folders in
/// the scratch folder and the colour variables cleared but for `colour`.
fn run<S: AsRef<OsStr>>(blender: &Path, here: &Path, colour: Option<(&str, &Path)>, arguments: &[S]) -> (Option<i32>, String, String) {
    let mut command = Command::new(blender);
    command
        .args(arguments)
        .stdin(std::process::Stdio::null())
        .env_remove("OCIO")
        .env_remove("BLENDER_OCIO")
        .env("TMPDIR", here.join("tmp"))
        .env("BLENDER_USER_CONFIG", here.join("config"))
        .env("BLENDER_USER_SCRIPTS", here.join("scripts_user"))
        .env("BLENDER_USER_EXTENSIONS", here.join("extensions"))
        .env("BLENDER_USER_DATAFILES", here.join("datafiles"));
    if let Some((variable, config)) = colour {
        command.env(variable, config);
    }
    let output = command.output().expect("Blender starts");
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

fn make(blender: &Path, here: &Path, colour: Option<(&str, &Path)>, what: &str, out: &Path) -> String {
    let maker = here.join("make.py");
    std::fs::write(&maker, THE_FILES).expect("written");
    let arguments = vec![
        "--factory-startup".to_string(),
        "--background".to_string(),
        "--python-exit-code".to_string(),
        "63".to_string(),
        "--python".to_string(),
        maker.display().to_string(),
        "--".to_string(),
        what.to_string(),
        out.display().to_string(),
    ];
    let (status, said, complaints) = run(blender, here, colour, &arguments);
    assert_eq!(status, Some(0), "{what} is made: {said}{complaints}");
    said
}

/// **The check's own reading of a file**, under Blender's own configuration:
/// the document, and the two streams.
fn the_check_reads(blender: &Path, here: &Path, file: &Path) -> (serde_json::Value, String, String) {
    let empty = here.join("nothing.py");
    std::fs::write(&empty, "").expect("written");
    let script = here.join("check.py");
    std::fs::write(&script, the_check_readings_script()).expect("written");
    let words = the_words_of_the_check_reading(file.as_os_str().as_bytes(), empty.as_os_str().as_bytes(), script.as_os_str().as_bytes());
    let words: Vec<&OsStr> = words.iter().map(|word| OsStr::from_bytes(word)).collect();
    let (status, ordinary, complaints) = run(blender, here, None, &words);
    assert_eq!(status, Some(0), "the check: {ordinary}{complaints}");
    let opens = ordinary.find("<<<SMOOZY-DEPENDENCIES>>>").expect("the document") + "<<<SMOOZY-DEPENDENCIES>>>".len();
    let closes = ordinary[opens..].find("<<<END-SMOOZY-DEPENDENCIES>>>").expect("its end") + opens;
    (serde_json::from_str(&ordinary[opens..closes]).expect("it reads"), ordinary, complaints)
}

const THE_FILES: &str = r#"
import bpy, os, sys
args = sys.argv[sys.argv.index("--") + 1:]
what, out = args[0], args[1]
bpy.ops.wm.read_factory_settings(use_empty=True)
scene = bpy.context.scene
folder = os.path.dirname(out)

def a_picture(name):
    path = os.path.join(folder, name)
    made = bpy.data.images.new("made", 8, 8)
    made.filepath_raw = path
    made.file_format = "PNG"
    made.save()
    bpy.data.images.remove(made)
    return bpy.data.images.load(path)

def a_material(name, image):
    material = bpy.data.materials.new(name)
    material.use_nodes = True
    node = material.node_tree.nodes.new("ShaderNodeTexImage")
    node.image = image
    return material, node

def settings(user, start, offset, frames, cyclic):
    user.frame_start = start
    user.frame_offset = offset
    user.frame_duration = frames
    user.use_cyclic = cyclic

if what == "sequence":
    seq = a_picture("q_0001.png")
    seq.source = "SEQUENCE"
    seq.name = "Seq"
    material, one = a_material("K1", seq)
    material.use_fake_user = True
    settings(one.image_user, 5, 2, 6, False)
    material, two = a_material("K2", seq)
    material.use_fake_user = True
    settings(two.image_user, 5, 2, 6, True)
    scene.render.use_compositing = True
    if hasattr(scene, "compositing_node_group"):
        tree = bpy.data.node_groups.new("Comp", "CompositorNodeTree")
        scene.compositing_node_group = tree
    else:
        scene.use_nodes = True
        tree = scene.node_tree
    image_node = tree.nodes.new("CompositorNodeImage")
    image_node.image = seq
    settings(image_node, 1, 5, 20, False)
    env = a_picture("e_0001.png")
    env.source = "SEQUENCE"
    env.name = "EnvSeq"
    world = bpy.data.worlds.new("W")
    world.use_nodes = True
    world.node_tree.nodes.new("ShaderNodeTexEnvironment").image = env
    scene.world = world
    # Blender's own file name for each number, through the first node's image user.
    for number in (0, 2, 20, 1000):
        one.image_user.frame_current = number
        print("SMOOZY-DESK-NAME", number, os.path.basename(seq.filepath_from_user(image_user=one.image_user)), flush=True)
    one.image_user.frame_current = 0
else:
    image = a_picture("swatch.png")
    image.name = "Swatch"
    material, node = a_material("M", image)
    material.use_fake_user = True
    if what == "col_orphan":
        image.colorspace_settings.name = "Smoozy Orphan"
    elif what == "col_renamed":
        image.colorspace_settings.name = "Smoozy Renamed Rec.2020"
    elif what == "col_view":
        scene.view_settings.view_transform = "Smoozy View"
    elif what == "col_display":
        scene.display_settings.display_device = "Smoozy Display"
    print("SMOOZY-DESK-MADE", what, repr(image.colorspace_settings.name), repr(scene.view_settings.view_transform),
          repr(scene.display_settings.display_device), flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

/// The research's `make_ocio.py`, cut down to what this needs: a colour space
/// `Smoozy Orphan` (a copy of `Linear Rec.2020` with no other names and no
/// interop ID), `Linear Rec.2020` renamed `Smoozy Renamed Rec.2020` with its
/// interop ID kept, a view `Smoozy View` on the `sRGB` display and a display
/// `Smoozy Display`.
const MAKE_OCIO: &str = r#"
import os, re, shutil, sys
src, out = sys.argv[1], sys.argv[2]
if os.path.exists(out):
    shutil.rmtree(out)
shutil.copytree(src, out)
text = open(os.path.join(src, "config.ocio"), encoding="utf-8").read()
text = re.sub(r"Linear Rec\.2020(?=[\s,\]\}]|$)", "Smoozy Renamed Rec.2020", text, flags=re.M)
lines = text.split("\n")
start = next(i for i, l in enumerate(lines) if l.strip() == "name: Smoozy Renamed Rec.2020")
b0 = start - 1
b1 = b0 + 1
while b1 < len(lines) and lines[b1].strip() != "- !<ColorSpace>" and not re.match(r"^\S", lines[b1]):
    b1 += 1
block = lines[b0:b1]
while block and block[-1].strip() == "":
    block.pop()
orphan = []
for l in block:
    s = l.strip()
    indent = l[:len(l) - len(l.lstrip())]
    if s.startswith("name:"):
        l = indent + "name: Smoozy Orphan"
    elif s.startswith("aliases:"):
        l = indent + "aliases: []"
    elif s.startswith("interop_id:"):
        continue
    orphan.append(l)
ci = next(i for i, l in enumerate(lines) if l.rstrip() == "colorspaces:")
lines[ci + 1:ci + 1] = orphan + [""]
in_srgb = False
for i, l in enumerate(lines):
    if l.rstrip() == "  sRGB:":
        in_srgb = True
        continue
    if in_srgb and "{name: False Color," in l:
        lines.insert(i + 1, l.replace("{name: False Color,", "{name: Smoozy View,"))
        break
di = next(i for i, l in enumerate(lines) if l.rstrip() == "  sRGB:")
dj = di + 1
while lines[dj].startswith("    - !<View>"):
    dj += 1
lines[dj:dj] = ["  Smoozy Display:"] + lines[di + 1:dj]
text = "\n".join(lines)
text = re.sub(r"^(active_views: \[.*)\]$", r"\1, Smoozy View]", text, flags=re.M)
text = re.sub(r"^(active_displays: \[.*)\]$", r"\1, Smoozy Display]", text, flags=re.M)
open(os.path.join(out, "config.ocio"), "w", encoding="utf-8").write(text)
"#;

fn bytes_of(value: &serde_json::Value) -> Vec<u8> {
    value.as_array().expect("bytes").iter().map(|one| one.as_u64().unwrap() as u8).collect()
}

#[test]
#[ignore = "runs every Blender at the desk; by hand"]
fn a_sequence_and_a_colour_name_through_the_checks_own_script() {
    let out = PathBuf::from(std::env::var("SMOOZY_DESK_OUT").expect("SMOOZY_DESK_OUT names a scratch folder"));
    for (version, blender) in the_blenders() {
        let here = out.join(version);
        let _ = std::fs::remove_dir_all(&here);
        for folder in ["tmp", "config", "scripts_user", "extensions", "datafiles", "files"] {
            std::fs::create_dir_all(here.join(folder)).expect("made");
        }
        println!("SMOOZY-DESK {version} {}", blender.display());

        // ---------------------------------------------------------- the sequence
        let file = here.join("files/sequence.blend");
        let said = make(&blender, &here, None, "sequence", &file);
        let blenders_names: Vec<(i64, String)> = said
            .lines()
            .filter_map(|line| line.strip_prefix("SMOOZY-DESK-NAME "))
            .map(|rest| {
                let (number, name) = rest.split_once(' ').expect("two words");
                (number.parse().expect("a number"), name.to_string())
            })
            .collect();
        assert_eq!(blenders_names.len(), 4, "{said}");
        for (number, name) in &blenders_names {
            let ours = String::from_utf8(the_file_a_number_reads(b"q_0001.png", *number).expect("worked out")).unwrap();
            println!("SMOOZY-DESK {version} number {number}: Blender {name}, Smoozy {ours}");
            assert_eq!(&ours, name, "{version}: the name of {number}");
        }
        let (document, _, _) = the_check_reads(&blender, &here, &file);
        let pictures: Vec<&serde_json::Value> = document["references"]
            .as_array()
            .expect("the references")
            .iter()
            .filter(|one| one["kind"] == "Image")
            .collect();
        let named = |ends: &str| {
            *pictures
                .iter()
                .find(|one| String::from_utf8_lossy(&bytes_of(&one["recorded"])).ends_with(ends))
                .unwrap_or_else(|| panic!("{version}: {ends} in {document}"))
        };
        let seq: TheSequencesUsers = serde_json::from_value(named("q_0001.png")["sequence"].clone()).expect("its users");
        let env: TheSequencesUsers = serde_json::from_value(named("e_0001.png")["sequence"].clone()).expect("its users");
        println!("SMOOZY-DESK {version} q_0001.png: {seq:?}");
        println!("SMOOZY-DESK {version} e_0001.png: {env:?}");
        let mut users = seq.users.clone();
        users.sort_by_key(|one| (one.start, one.offset, one.frames, one.cyclic));
        assert_eq!(
            users,
            vec![
                ASequencesUser { start: 1, offset: 5, frames: 20, cyclic: false },
                ASequencesUser { start: 5, offset: 2, frames: 6, cyclic: false },
                ASequencesUser { start: 5, offset: 2, frames: 6, cyclic: true },
            ],
            "{version}"
        );
        assert!(!seq.other_users, "{version}");
        assert!(env.users.is_empty() && env.other_users, "{version}: {env:?}");
        let frames: Vec<i64> = (1..=15).collect();
        let needed: Vec<String> = the_files_a_sequence_needs(b"q_0001.png", &seq, &frames)
            .expect("worked out")
            .into_iter()
            .map(|one| String::from_utf8(one).unwrap())
            .collect();
        let expected: Vec<String> = (2..=20).map(|number| format!("q_{number:04}.png")).collect();
        println!("SMOOZY-DESK {version} needed over 1 to 15: {} files, {} to {}", needed.len(), needed[0], needed[needed.len() - 1]);
        assert_eq!(needed, expected, "{version}");

        // ------------------------------------------------------ the colour names
        let own = blender
            .parent()
            .and_then(Path::parent)
            .expect("the bundle")
            .join("Resources")
            .join(version)
            .join("datafiles/colormanagement");
        let custom = here.join("ocio");
        let maker = here.join("make_ocio.py");
        std::fs::write(&maker, MAKE_OCIO).expect("written");
        let made = Command::new("python3")
            .arg("-I")
            .arg(&maker)
            .arg(&own)
            .arg(&custom)
            .output()
            .expect("python runs");
        assert!(made.status.success(), "{}", String::from_utf8_lossy(&made.stderr));
        let config = custom.join("config.ocio");
        let variable = if version == "4.5" { "OCIO" } else { "BLENDER_OCIO" };
        for (what, expected) in [
            ("col_orphan", vec![(WhichColourName::ColourSpace, "Smoozy Orphan")]),
            ("col_view", vec![(WhichColourName::ViewTransform, "Smoozy View")]),
            ("col_display", vec![(WhichColourName::Display, "Smoozy Display")]),
            (
                "col_renamed",
                if version == "5.3" { vec![] } else { vec![(WhichColourName::ColourSpace, "Smoozy Renamed Rec.2020")] },
            ),
        ] {
            let file = here.join(format!("files/{what}.blend"));
            let made = make(&blender, &here, Some((variable, &config)), what, &file);
            let made = made.lines().find(|line| line.starts_with("SMOOZY-DESK-MADE")).unwrap_or("").to_string();
            let (_, ordinary, complaints) = the_check_reads(&blender, &here, &file);
            let said = the_colour_names_not_found(&[ordinary.as_bytes(), complaints.as_bytes()]);
            let on_which: Vec<&str> = said
                .iter()
                .map(|one| {
                    match (ordinary.contains(&format!("\"{}\"", one.name)), complaints.contains(&format!("\"{}\"", one.name))) {
                        (true, true) => "both",
                        (true, false) => "ordinary",
                        (false, true) => "complaints",
                        (false, false) => "neither",
                    }
                })
                .collect();
            println!("SMOOZY-DESK {version} {what}: made {made} | said {said:?} | on {on_which:?}");
            let said: Vec<(WhichColourName, &str)> = said.iter().map(|one| (one.which, one.name.as_str())).collect();
            assert_eq!(said, expected, "{version} {what}:\n{ordinary}\n{complaints}");
        }
    }
}

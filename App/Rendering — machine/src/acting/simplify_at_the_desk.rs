//! **Owed piece 8, the first check: Simplify's texture cap and child-particle
//! cap, imposed at a render's start, reach the picture** — kept in the tree and
//! ignored. `Rendering` §9 item 11 left exactly this inferred: *"the launch
//! route for Texture Limit and Child Particles — a value assigned by the launch
//! material rather than saved in the file"*. Run by hand at the desk:
//!
//! ```text
//! cargo test -p smoozy-rendering-machine simplify_at_the_desk -- --ignored --nocapture
//! ```
//!
//! Each of the five Blenders on the Mac is found at its usual place, or at
//! `SMOOZY_DESK_BLENDER_45`, `_50`, `_51`, `_52` and `_53`; the runs go to a
//! folder under the system's temporary folder, kept so the pictures can be
//! looked at, or under `SMOOZY_DESK_OUT`.
//!
//! **What runs is the render worker's own driver, whole**, as
//! `pipeline.rs`'s eleventh-notes desk test runs it: the real version ask, the
//! real reading of the file, everything composed from it, the script before
//! the file and the launch material placed and read back on disk, and the
//! render launched with the line's own words. Only the machine's own records
//! are stand-ins. The build path is a two-line wrapper adding the
//! factory-startup switch at the head, as the desk's other real-Blender tests
//! do, so the Creator's own preferences and add-ons play no part.
//!
//! **Two small Cycles scenes, each rendered three times in each Blender**:
//! as saved (no Simplify entry in the payload), with the switch alone, and with
//! the switch and the cap. Both files are saved with the switch off, the
//! texture limit *OFF* and child particles at 1, so the switch alone changes
//! nothing either cap would.
//!
//! - **The texture cap.** A plane fills the frame and shows, through an
//!   emission shader with no filtering, one eighth of a packed 1024-pixel
//!   picture of black and white stripes four pixels wide — one texel per
//!   pixel of the 128-pixel frame. **At a 128-pixel cap the whole picture is
//!   128 pixels across, so a stripe pair is less than one texel and the
//!   stripes cannot survive.** How it is told: along each row of the middle,
//!   the mean difference between a pixel and the one four to its right —
//!   about full scale with the stripes, near nought without.
//! - **The child-particle cap.** Ten hair strands grow from a ground the
//!   camera sees edge on, with fifty interpolated children each at render,
//!   white against the dark. **At a child-particle fraction of nought only the
//!   ten parents are left.** How it is told: the share of the frame's pixels
//!   that are bright.
//!
//! **What it does not prove**: a graphics card, Windows, or a file saved from
//! Blender's interface. Every run here is on the processor, on this Mac, with
//! files saved by a Blender with no window.
//!
//! **Names used**, each read off all five Blenders' own property lists at the
//! desk on 2026-10-17 (`Audits/Build/Step 10c/Owed piece 8 — what landed.md`
//! lists them with where). The node sockets the files link are checked by
//! name inside the making script itself, so a socket in another place fails
//! the file's making rather than linking the wrong thing.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use smoozy_dependencies_machine::placed_copies::{APlacedFile, ThePlacedCopies};
use smoozy_dependencies_machine::remap::RemapMaterial;
use smoozy_framing::source_reading::SourceReading;
use smoozy_job_files_machine::attempts::AttemptRecord;
use smoozy_job_files_machine::composing::TheReadingAndWhatFollows;
use smoozy_job_files_machine::manifest::FrameRange;
use smoozy_job_files_machine::reckoning::WhatTheWalkOfTheFolderIs;
use smoozy_job_files_machine::where_the_folder_sits::{MakesFolders, WhatStands, WhereTheJobOutputFolderSits};
use smoozy_render_settings_machine::payload::Payload;
use smoozy_server_setup_machine::the_optix_library::WhatTheLoaderFound;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, MachineClock};

use crate::acting::launch::tests::AKernelThat;
use crate::acting::launch::{AsksABlenderToNameItsVersion, TheRenderStartedOnTheMachine, TheShortInvocationOnTheMachine};
use crate::acting::material::TheWorkersOwnFilesystem;
use crate::acting::pipeline::{
    TheAttempt, TheJobOutputFolderForThisAttempt, TheRenderWorkersPipeline, TheWorkersOwnSteps,
    WhereTheAttemptEnded,
};
use crate::acting::reading::TheReadingOnTheMachine;
use crate::acting::the_optix_library::LooksForTheOptixLibrary;
use crate::composition::tests::a_manifest;
use crate::material::tests::{a_blender, a_job, an_untouched_payload, four_values};

/// **No run of Blender here is waited on past this.**
const ONE_RUN_AT_MOST: Duration = Duration::from_secs(300);

/// What both files share: an empty Cycles scene on the processor, 128 pixels
/// square, one frame, written as an uncompressed BMP so the test reads the
/// picture itself, and Simplify off with neither cap in force.
const THE_SCENE_BOTH_FILES_SHARE: &str = r#"
import bpy, sys, math
out = sys.argv[sys.argv.index("--") + 1]
bpy.ops.wm.read_factory_settings(use_empty=True)
scene = bpy.context.scene
scene.name = "Scene"
scene.render.engine = "CYCLES"
assert scene.render.engine == "CYCLES", scene.render.engine
scene.cycles.device = "CPU"
scene.cycles.samples = 16
scene.cycles.use_adaptive_sampling = False
scene.cycles.use_denoising = False
scene.render.resolution_x = 128
scene.render.resolution_y = 128
scene.render.resolution_percentage = 100
scene.render.image_settings.file_format = "BMP"
scene.render.image_settings.color_mode = "RGB"
scene.render.filepath = "//out/f_"
scene.frame_start = 1
scene.frame_end = 1
scene.render.use_simplify = False
scene.render.simplify_child_particles_render = 1.0
scene.cycles.texture_limit_render = "OFF"

def an_emission(name, picture=None):
    material = bpy.data.materials.new(name)
    material.use_nodes = True
    tree = material.node_tree
    tree.nodes.clear()
    emission = tree.nodes.new("ShaderNodeEmission")
    output = tree.nodes.new("ShaderNodeOutputMaterial")
    assert emission.inputs[0].name == "Color", [s.name for s in emission.inputs]
    assert emission.outputs[0].name == "Emission", [s.name for s in emission.outputs]
    assert output.inputs[0].name == "Surface", [s.name for s in output.inputs]
    tree.links.new(emission.outputs[0], output.inputs[0])
    if picture is not None:
        texture = tree.nodes.new("ShaderNodeTexImage")
        texture.image = picture
        texture.interpolation = "Closest"
        assert texture.outputs[0].name == "Color", [s.name for s in texture.outputs]
        tree.links.new(texture.outputs[0], emission.inputs[0])
    else:
        emission.inputs[0].default_value = (1.0, 1.0, 1.0, 1.0)
    return material

def a_camera(scale, location, rotation):
    data = bpy.data.cameras.new("Camera")
    data.type = "ORTHO"
    data.ortho_scale = scale
    camera = bpy.data.objects.new("Camera", data)
    camera.location = location
    camera.rotation_euler = rotation
    scene.collection.objects.link(camera)
    scene.camera = camera
"#;

/// **The texture file**: one eighth of a 1024-pixel striped picture across a
/// 128-pixel frame.
const THE_TEXTURE_FILE: &str = r#"
size = 1024
picture = bpy.data.images.new("Stripes", size, size)
row = []
for x in range(size):
    value = 1.0 if (x // 4) % 2 == 0 else 0.0
    row += [value, value, value, 1.0]
picture.pixels[:] = row * size
picture.pack()
mesh = bpy.data.meshes.new("Plane")
mesh.from_pydata([(-1, -1, 0), (1, -1, 0), (1, 1, 0), (-1, 1, 0)], [], [(0, 1, 2, 3)])
layer = mesh.uv_layers.new(name="UVMap")
for loop, uv in zip(layer.data, [(0, 0), (0.125, 0), (0.125, 0.125), (0, 0.125)]):
    loop.uv = uv
mesh.materials.append(an_emission("Stripes", picture))
plane = bpy.data.objects.new("Plane", mesh)
scene.collection.objects.link(plane)
a_camera(2.0, (0, 0, 5), (0, 0, 0))
print("SMOOZY-DESK-MADE texture", picture.size[0], picture.size[1], picture.packed_file is not None, flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

/// **The hair file**: ten parents and fifty interpolated children each, seen
/// from the side.
const THE_HAIR_FILE: &str = r#"
mesh = bpy.data.meshes.new("Ground")
mesh.from_pydata([(-1, -1, 0), (1, -1, 0), (1, 1, 0), (-1, 1, 0)], [], [(0, 1, 2, 3)])
mesh.materials.append(an_emission("White"))
ground = bpy.data.objects.new("Ground", mesh)
scene.collection.objects.link(ground)
ground.show_instancer_for_render = False
ground.modifiers.new("Hair", "PARTICLE_SYSTEM")
settings = ground.particle_systems[0].settings
settings.type = "HAIR"
settings.count = 10
settings.hair_length = 1.0
settings.emit_from = "FACE"
settings.render_type = "PATH"
settings.child_type = "INTERPOLATED"
settings.rendered_child_count = 50
settings.child_percent = 50
settings.material = 1
settings.root_radius = 1.0
settings.radius_scale = 0.02
a_camera(2.5, (0, -5, 0.5), (math.pi / 2, 0, 0))
print("SMOOZY-DESK-MADE hair", settings.count, settings.child_type, settings.rendered_child_count, flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

fn the_five() -> Vec<(&'static str, PathBuf)> {
    [
        ("4.5", "SMOOZY_DESK_BLENDER_45", "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"),
        ("5.0", "SMOOZY_DESK_BLENDER_50", "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"),
        ("5.1", "SMOOZY_DESK_BLENDER_51", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
        ("5.2", "SMOOZY_DESK_BLENDER_52", "/Applications/Blender.app/Contents/MacOS/Blender"),
        ("5.3", "SMOOZY_DESK_BLENDER_53", "/Applications/Blender 5.3b.app/Contents/MacOS/Blender"),
    ]
    .into_iter()
    .map(|(named, variable, usual)| {
        (named, PathBuf::from(std::env::var(variable).unwrap_or_else(|_| usual.to_string())))
    })
    .collect()
}

/// A child waited on until it ends or [`ONE_RUN_AT_MOST`] passes; it is
/// killed and reaped past that, so nothing is left running.
fn wait_for(child: &mut std::process::Child, what: &str) -> Option<i32> {
    let began = Instant::now();
    loop {
        if let Some(status) = child.try_wait().expect("it can be asked") {
            return status.code();
        }
        if began.elapsed() > ONE_RUN_AT_MOST {
            let _ = child.kill();
            let _ = child.wait();
            panic!("{what} ran past {ONE_RUN_AT_MOST:?}");
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// One run of Blender that makes a file, its ordinary output kept.
fn make(blender: &Path, script: &Path, file: &Path) -> (Option<i32>, String) {
    let mut child = std::process::Command::new(blender)
        .args(["--factory-startup", "--background", "--python-exit-code", "63", "--python"])
        .arg(script)
        .arg("--")
        .arg(file)
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
    let status = wait_for(&mut child, "the file's making");
    let mut said = out.join().expect("the reader ends");
    said.extend(err.join().expect("the reader ends"));
    (status, String::from_utf8_lossy(&said).to_string())
}

/// **The machine's own records, as stand-ins that answer and never judge**,
/// and folders made for real.
struct TheDesksMachine {
    remap: RemapMaterial,
}

impl MakesFolders for TheDesksMachine {
    fn what_stands_at(&mut self, path: &str) -> WhatStands {
        if Path::new(path).is_dir() {
            WhatStands::AFolder
        } else {
            WhatStands::Nothing
        }
    }
    fn make_one_folder(&mut self, path: &str) -> Result<(), ()> {
        std::fs::create_dir(path).map_err(|_| ())
    }
}

impl TheWorkersOwnSteps for TheDesksMachine {
    fn read_the_remap_material(&mut self, _at: &str) -> Option<RemapMaterial> {
        Some(self.remap.clone())
    }
    fn the_four_values_now(&mut self, _blend_path: &str) -> Option<SourceReading> {
        Some(four_values())
    }
    fn write_the_recorded_reading(&mut self, _composed: &TheReadingAndWhatFollows) {}
    fn ask_the_supervisor_to_place_the_hold(&mut self, reason: &str) -> Option<Identity> {
        panic!("the attempt asked for a hold: {reason}");
    }
    fn walk_the_job_output_folder(&mut self, _at: &str) -> WhatTheWalkOfTheFolderIs {
        WhatTheWalkOfTheFolderIs::ItCompleted(Vec::new())
    }
    fn the_machines_folders(&mut self) -> &mut dyn MakesFolders {
        self
    }
    fn write_where_the_folder_sits(&mut self, _sits: &WhereTheJobOutputFolderSits) -> Result<(), String> {
        Ok(())
    }
    fn write_the_attempt_record(&mut self, _record: &AttemptRecord) -> Result<(), String> {
        Ok(())
    }
    fn copy_into_the_job_folder(&mut self, from: &[u8], _to: &[u8]) -> Result<Vec<APlacedFile>, String> {
        Err(format!("nothing is copied at this desk: {}", String::from_utf8_lossy(from)))
    }
    fn write_the_placed_copies(&mut self, _placed: &ThePlacedCopies) {}
    fn record_the_render_process(
        &mut self,
        _pair: smoozy_runtime::execution_group::RecordedPair,
    ) -> Result<(), String> {
        Ok(())
    }
}

/// L24b's lookup is asked only where the job names `OPTIX`; these jobs name
/// none.
struct NoCards;
impl LooksForTheOptixLibrary for NoCards {
    fn what_the_loader_finds(&self) -> WhatTheLoaderFound {
        WhatTheLoaderFound::TheLibrary
    }
}

/// **A BMP read back**: its width, its height and each pixel's green value,
/// top row first. Blender writes it uncompressed; three and four bytes a pixel
/// are both read, rows padded to four bytes, bottom row first on disk where the
/// height is positive.
fn a_bmps_green(bytes: &[u8]) -> (usize, usize, Vec<u8>) {
    let word = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
    assert_eq!(&bytes[0..2], b"BM", "a BMP");
    let start = word(10) as usize;
    let width = word(18) as i32;
    let height = word(22) as i32;
    let bits = u16::from_le_bytes(bytes[28..30].try_into().unwrap()) as usize;
    let compression = word(30);
    assert!(bits == 24 || bits == 32, "{bits} bits a pixel");
    assert!(compression == 0 || compression == 3, "compression {compression}");
    let (width, rows) = (width.unsigned_abs() as usize, height.unsigned_abs() as usize);
    let each = bits / 8;
    let stride = (width * each).div_ceil(4) * 4;
    let mut green = Vec::with_capacity(width * rows);
    for row in 0..rows {
        let on_disk = if height > 0 { rows - 1 - row } else { row };
        let line = &bytes[start + on_disk * stride..start + on_disk * stride + width * each];
        for pixel in line.chunks(each) {
            green.push(pixel[1]);
        }
    }
    (width, rows, green)
}

/// **The stripes' measure**: over the middle rows, the mean difference between
/// a pixel and the one four to its right, from 0 to 255.
fn the_stripes(width: usize, rows: usize, green: &[u8]) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0;
    for row in rows / 4..rows * 3 / 4 {
        for x in width / 8..width * 7 / 8 - 4 {
            let here = green[row * width + x] as f64;
            let there = green[row * width + x + 4] as f64;
            sum += (here - there).abs();
            count += 1.0;
        }
    }
    sum / count
}

/// **The hair's measure**: the share of the frame's pixels that are bright.
fn the_bright_share(green: &[u8]) -> f64 {
    green.iter().filter(|value| **value > 128).count() as f64 / green.len() as f64
}

fn every_bmp_under(folder: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(folder).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(every_bmp_under(&path));
        } else if path.extension().is_some_and(|ending| ending.eq_ignore_ascii_case("bmp")) {
            found.push(path);
        }
    }
    found.sort();
    found
}

/// **One job through the render worker's driver**: the file copied into its
/// own folder, the render launched with the line's own words and waited on,
/// and the one picture it wrote read back. Hands back the picture's green
/// values, its size, and the material's Simplify lines from the render's
/// ordinary output — the marker's own lines for the Simplify family.
fn one_job(
    blender: &Path,
    file: &Path,
    root: &Path,
    payload: &Payload,
) -> Result<((usize, usize, Vec<u8>), Vec<String>), String> {
    std::fs::create_dir_all(root).expect("the job's folder");
    let wrapper = root.join("blender-with-factory-startup");
    std::fs::write(&wrapper, format!("#!/bin/sh\nexec '{}' --factory-startup \"$@\"\n", blender.display()))
        .expect("the wrapper");
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).expect("runnable");
    let blend = root.join(file.file_name().expect("a name"));
    std::fs::copy(file, &blend).expect("the file is put in place");
    let job_folder = root.join("job");
    let at = |name: &str| root.join(name).display().to_string();

    // **L14's ask, for real**, so the manifest names what the binary names.
    let ask = TheShortInvocationOnTheMachine {
        the_scripts_path: at("ask.py"),
    };
    let version = ask
        .what_version_it_names(&wrapper.display().to_string())
        .ok_or("the Blender named no version")?;
    let version_text = format!("{}.{}.{}", version.major, version.minor, version.patch);

    let job = a_job();
    let mut manifest = a_manifest(&job, &version_text, "Scene");
    manifest.frame_range = FrameRange::TheUsersParsedIntegers(vec![1]);
    manifest.blend_path_on_the_machine = blend.display().to_string();
    let mut machine = TheDesksMachine {
        remap: RemapMaterial::composed(&job, &a_blender(), Vec::new()),
    };
    let clock = Clock::AMachine(MachineClock("the-desk".to_string()));
    let start = TheRenderStartedOnTheMachine {
        kernel: AKernelThat(Some(19186)),
    };
    let went = {
        let mut pipeline = TheRenderWorkersPipeline {
            start: &start,
            places: &TheWorkersOwnFilesystem,
            asks_the_version: &ask,
            takes_the_reading: &TheReadingOnTheMachine,
            steps: &mut machine,
            looks_for_the_optix_library: &NoCards,
        };
        pipeline.run(TheAttempt {
            job: &job,
            manifest: &manifest,
            payload,
            the_manifests_version: version,
            the_remap_materials_path: &at("remap"),
            the_blender_build_path: &wrapper.display().to_string(),
            the_launch_materials_path: &at("material.py"),
            the_load_time_scripts_path: &at("load-time.py"),
            the_readings_script_path: &at("reading.py"),
            the_job_output_folder: TheJobOutputFolderForThisAttempt::Known(&job_folder.display().to_string()),
            the_folder_the_blend_sits_in: root.display().to_string().as_bytes(),
            previous_attempts: &[],
            number: 1,
            the_machines_clock: &clock,
            the_manifests_reading: None,
            the_manifests_settings_half: None,
        })
    };
    let WhereTheAttemptEnded::TheRenderWasLaunched(mut started) = went.ended else {
        return Err(format!("not launched: {:?}", went.ended));
    };
    let take = |mut from: Box<dyn std::io::Read + Send>| {
        std::thread::spawn(move || {
            let mut all = Vec::new();
            from.read_to_end(&mut all).ok();
            all
        })
    };
    let ordinary = take(Box::new(started.ordinary));
    let complaints = take(Box::new(started.complaints));
    let status = wait_for(&mut started.child, "the render");
    let ordinary = ordinary.join().expect("the reader ends");
    let complaints = complaints.join().expect("the reader ends");
    std::fs::write(root.join("render.log"), &ordinary).ok();
    std::fs::write(root.join("render-complaints.log"), &complaints).ok();
    if status != Some(0) {
        return Err(format!("the render ended at {status:?}; see {}", root.display()));
    }
    let simplify_lines: Vec<String> = String::from_utf8_lossy(&ordinary)
        .lines()
        .filter(|line| line.starts_with("SMOOZY-MATERIAL") && line.contains("\"simplify\""))
        .map(|line| line.chars().take(400).collect())
        .collect();
    let written = every_bmp_under(&job_folder);
    if written.len() != 1 {
        return Err(format!("{} pictures written: {written:?}", written.len()));
    }
    let bytes = std::fs::read(&written[0]).expect("the picture reads");
    Ok((a_bmps_green(&bytes), simplify_lines))
}

#[test]
#[ignore = "run by hand at the desk with the five Blenders"]
fn simplifys_texture_cap_and_child_particle_cap_imposed_at_the_start_reach_the_picture_in_all_five() {
    let out = std::env::var("SMOOZY_DESK_OUT").map(PathBuf::from).unwrap_or_else(|_| {
        std::env::temp_dir().join(format!(
            "smoozy-simplify-at-the-desk-{}",
            Identity::issue(IdentityKind::Operation).unwrap().as_text()
        ))
    });
    std::fs::create_dir_all(&out).expect("a folder");
    println!("the runs are kept under {}", out.display());
    let texture_maker = out.join("make-texture.py");
    std::fs::write(&texture_maker, format!("{THE_SCENE_BOTH_FILES_SHARE}{THE_TEXTURE_FILE}")).expect("written");
    let hair_maker = out.join("make-hair.py");
    std::fs::write(&hair_maker, format!("{THE_SCENE_BOTH_FILES_SHARE}{THE_HAIR_FILE}")).expect("written");

    let as_saved = an_untouched_payload();
    let mut switch_only = an_untouched_payload();
    switch_only.simplify.use_simplify = Some(true);
    let mut texture_capped = switch_only.clone();
    texture_capped.simplify.texture_limit_render = Some("128".to_string());
    let mut children_capped = switch_only.clone();
    children_capped.simplify.child_particles_render = Some(0.0);

    let mut faults = Vec::new();
    for (named, blender) in the_five() {
        let here = out.join(named);
        std::fs::create_dir_all(&here).expect("a folder");

        // **The texture cap.**
        let texture = here.join("stripes.blend");
        let (made, said) = make(&blender, &texture_maker, &texture);
        println!("{named}: {}", said.lines().find(|line| line.starts_with("SMOOZY-DESK-MADE")).unwrap_or("no made line"));
        if made != Some(0) {
            faults.push(format!("{named}: the texture file was not made: {said}"));
            continue;
        }
        let mut stripes = Vec::new();
        for (case, payload) in [("as saved", &as_saved), ("switch only", &switch_only), ("capped at 128", &texture_capped)] {
            match one_job(&blender, &texture, &here.join(format!("stripes {case}")), payload) {
                Ok(((width, rows, green), lines)) => {
                    let measure = the_stripes(width, rows, &green);
                    println!("{named}: stripes, {case}: {width}x{rows}, mean difference four pixels apart {measure:.1} of 255");
                    for line in lines {
                        println!("{named}:     {line}");
                    }
                    stripes.push(measure);
                }
                Err(why) => {
                    faults.push(format!("{named}: stripes, {case}: {why}"));
                    stripes.push(f64::NAN);
                }
            }
        }
        if !(stripes[0] > 100.0 && stripes[1] > 0.75 * stripes[0] && stripes[2] < 0.25 * stripes[0]) {
            faults.push(format!("{named}: the texture cap did not show: as saved, switch only, capped = {stripes:?}"));
        }

        // **The child-particle cap.**
        let hair = here.join("hair.blend");
        let (made, said) = make(&blender, &hair_maker, &hair);
        println!("{named}: {}", said.lines().find(|line| line.starts_with("SMOOZY-DESK-MADE")).unwrap_or("no made line"));
        if made != Some(0) {
            faults.push(format!("{named}: the hair file was not made: {said}"));
            continue;
        }
        let mut shares = Vec::new();
        for (case, payload) in [("as saved", &as_saved), ("switch only", &switch_only), ("children at 0", &children_capped)] {
            match one_job(&blender, &hair, &here.join(format!("hair {case}")), payload) {
                Ok(((width, rows, green), lines)) => {
                    let share = the_bright_share(&green);
                    println!("{named}: hair, {case}: {width}x{rows}, bright share {:.4}", share);
                    for line in lines {
                        println!("{named}:     {line}");
                    }
                    shares.push(share);
                }
                Err(why) => {
                    faults.push(format!("{named}: hair, {case}: {why}"));
                    shares.push(f64::NAN);
                }
            }
        }
        if !(shares[0] > 0.02
            && (shares[1] - shares[0]).abs() < 0.25 * shares[0]
            && shares[2] < 0.5 * shares[0])
        {
            faults.push(format!("{named}: the child-particle cap did not show: as saved, switch only, capped = {shares:?}"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

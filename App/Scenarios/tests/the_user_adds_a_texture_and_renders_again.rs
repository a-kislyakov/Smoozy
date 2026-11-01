//! **`Preparation/Scenarios/The user adds a texture and renders again.md`,
//! played at the desk with a real Blender** — step 12's piece SC5.
//!
//! The scenario's situation and its nine steps, unchanged, through the
//! application's own acts, against the desk machine. At each step the test
//! prints what the user did, what Smoozy hands the page to show, and which
//! clause says so, and asserts all three. **A step the desk cannot play is said
//! in the output and never skipped quietly.**
//!
//! **Ignored, and run on purpose with the Blender's path set**, the way
//! `Job files and outputs — machine/tests/stepping_at_the_desk.rs` is:
//!
//! ```text
//! SMOOZY_DESK_BLENDER='/Applications/Blender 5.1.2.app/Contents/MacOS/Blender' \
//! cargo test -p smoozy-scenarios --test the_user_adds_a_texture_and_renders_again \
//!   -- --ignored --nocapture
//! ```
//!
//! **The sizes, the runner's ruling on the map's open question.** 200 frames,
//! as the scenario says; 160 × 90 pixels, 8 samples, Cycles on the CPU, no
//! denoising, so the whole test, both days' renders among it, takes about a
//! minute and a quarter on this Mac. **The scenario's 200 MB displacement map
//! is a 1024 × 1024 PNG of ripples, about 34 KB**: what the scenario turns on
//! is that the map was never sent up, and nothing sends it, so its size
//! changes nothing a step checks.
//!
//! **What the frames hold, and what the test asserts about it.** The plans say
//! a texture linked for the first time is not on the machine, nothing checks
//! at a submission, and the render will not find it (`Specs/Dependency Upload
//! & Remap`, *Iterating*); what stands in for that is the warning at
//! submission. **So step 9 asserts that today's frames do not carry the map**:
//! they are compared, frame for frame, with yesterday's, and frame 1 is
//! rendered once more on the user's own computer, where the map is, to show
//! what a floor carrying it looks like. A test asserting that the new frames
//! show the map would be asserting against the plans.
//!
//! **What is real here.** OpenSSH's own server behind the harness's line; the
//! application's own transport and the server's own point; `Servers` Q1 step 7
//! on the connection; Files' own upload — the walk, the already-exists box, the
//! answer written, the files sent and each publication marked; Render
//! Settings' own page — the saved-with tag, the machine's Blenders searched
//! and asked their version, *Read the file* through the machine's file-reading
//! worker, the answers typed and the act composed — and the desktop's own
//! submit act; the queue and History screens' own readings; a whole job's
//! *Download all to…*; and the machine's own program, its real supervisor and
//! render worker. **And Blender itself**: one of this Mac's own, started on the
//! machine by the machine's program, and on the user's computer by the test to
//! make, change and open the user's files.
//!
//! **Where the application's own wiring needs the window, it is mirrored and
//! said**, as in SC8 (`tests/the_user_stops_a_running_job.rs`, whose
//! connection and passes this file copies): the point lending one connection,
//! step 7 run on it, a pass as the read forward and the queue reading. Four
//! private pieces of the desktop are mirrored and say so where they stand —
//! what Render Settings reads of the record before it draws, the door each
//! upload's publication goes through, the page's opening reads, and the read
//! of a captured stream that brings Blender's words into History.
//!
//! **What the desk cannot do.**
//!
//! - **A Linux machine and the user's computer apart.** The machine and the
//!   user's computer are one Mac here, so a file the machine's Blender looks
//!   for by an absolute path on the user's computer would be found. **The
//!   user's file records its textures relative to itself**, as a project does,
//!   and the machine's Blender looks for them beside the `.blend` on the
//!   machine; what it finds there is only what Files sent up.
//! - **The machine's Blender is started through a launcher that adds the
//!   factory-startup switch**, because this Mac's Blenders are the Creator's
//!   (the harness's `a_real_blender_is_installed` says why); a rental's is
//!   launched without it.
//! - **Yesterday is under a minute ago**, in the same sitting of Smoozy.
//!
//! **Nothing of the Creator's is touched**: see the head of
//! `src/the_desk_machine.rs`. Every Blender this test starts on the user's
//! computer runs with the factory-startup switch and a throwaway home, and
//! writes only inside the desk's throwaway folder.

#![cfg(unix)]

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use smoozy_dependencies::record::MachineKey;
use smoozy_desktop::downloading::TheRunsTransfers;
use smoozy_desktop::reading_the_file::{the_file_is_read, ThisServersMachine, WhatTheReadCameTo};
use smoozy_desktop::record::job_records::JobRecords;
use smoozy_desktop::servers::the_machines_work::{a_read_forward_pass, the_queue_reading_from, the_step_seven, WhatThisRunHolds};
use smoozy_desktop::servers::the_words_kept::{the_words_owed_are_taken_in, ReadsAStreamBackWhole};
use smoozy_desktop::submitting::{the_act_is_performed_naming_its_jobs, the_blenders_on_the_machine, WhatBecameOfOneJob, WhatTheActCameTo};
use smoozy_desktop::surface::files::{
    a_folder_is_read, the_collision_question, the_tab_after_the_reading, the_upload_warnings, what_the_record_holds_for_the_row,
    TheCollidingAct, TheCollisionQuestion, TheListing, TheOrder,
};
use smoozy_desktop::surface::render_settings::{
    the_act_as_the_page_composed_it, the_answer_is_taken, the_check_the_band_reads, the_panel_starting_from, the_run_holds,
    the_saved_with_version_lands, the_saved_with_version_to_hold, the_version_list_came_back, what_the_act_says, ASentence,
    TheRenderSettings, TheSwitchReads, WhatTheRecordHolds, WhichSetting,
};
use smoozy_desktop::surface::the_history::{what_is_kept, AnEnding};
use smoozy_desktop::surface::the_queue::{what_is_held, APart};
use smoozy_desktop::uploading::{
    an_upload_is_prepared, the_answer_is_applied, the_telling_stands_first, the_upload_runs, TheRecordAnUploadWrites,
    WhatIsBeingUploaded, WhatTheAnswerLeft, WhatTheUploadCameTo, WhenAFileIsPublished,
};
use smoozy_file_browser::already_exists::CollisionAnswer;
use smoozy_job_files::layout::{Area, TheLayout};
use smoozy_queue_history::ending::Ending as AJobsEnding;
use smoozy_queue_history::line::QueueState;
use smoozy_scenarios::the_desk_machine::{a_whole_job_is_downloaded_to, APointLending, TheDesk, TheDeskMachine, TheDesksRecord};
use smoozy_servers::condition::ObservedValue;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, Reading};

// ════════════════════════════════════════════════════════════════════════════
// The sizes
// ════════════════════════════════════════════════════════════════════════════

/// **The scenario's own 200 frames.**
const THE_FRAMES: i64 = 200;

/// **The picture, and how hard Cycles works on it** — the runner's ruling:
/// small, few samples, so both days render in well under twenty minutes.
const THE_WIDTH: u32 = 160;
const THE_HEIGHT: u32 = 90;
const THE_SAMPLES: u32 = 8;

/// **The map's side, in pixels** — a PNG of about 34 KB standing in for the
/// scenario's 200 MB; see the head of this file.
const THE_MAPS_SIDE: u32 = 1024;

/// **How long one Blender on the user's computer may take** — a test's own
/// bound, so a Blender that never ends fails the run rather than holding it.
const ONE_BLENDER_AT_MOST: Duration = Duration::from_secs(300);

/// The job's name, as the user types it on both days.
const THE_JOBS_NAME: &str = "Shot 40";

// ════════════════════════════════════════════════════════════════════════════
// Blender on the user's own computer
// ════════════════════════════════════════════════════════════════════════════

/// **What the user does in Blender on their own computer, as one script**:
/// make yesterday's file, add today's map, render one frame where the map is,
/// and open frames to look at them. Each act prints one line, `SC5-SAID`
/// followed by what it found, which the test reads.
///
/// **The two pictures are written by the script itself**, as plain PNG files
/// from Python's own `zlib`, so nothing about Blender's image saving stands
/// between the test and the pixels it chose.
const THE_USERS_BLENDER_SCRIPT: &str = r#"
import bpy, sys, os, json, math, zlib, struct
import numpy as np

argv = sys.argv[sys.argv.index("--") + 1:]
mode = argv[0]

def said(what):
    print("SC5-SAID " + json.dumps(what), flush=True)

def a_png(path, rgb):
    height, width, _ = rgb.shape
    raw = b"".join(b"\x00" + rgb[row].tobytes() for row in range(height))
    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "wb") as out:
        out.write(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0))
                  + chunk(b"IDAT", zlib.compress(raw, 6)) + chunk(b"IEND", b""))

def nodes_of(material):
    if getattr(material, "node_tree", None) is None:
        try:
            material.use_nodes = True
        except Exception:
            pass
    return material.node_tree

def the_node(tree, kind):
    return next(node for node in tree.nodes if node.type == kind)

def an_image_beside_the_file(path, relative, non_colour):
    image = bpy.data.images.load(path)
    image.filepath = relative
    if non_colour:
        try:
            image.colorspace_settings.name = "Non-Color"
        except Exception:
            pass
    return image

def the_images():
    return {image.name: image.filepath for image in bpy.data.images if image.source == "FILE"}

if mode == "yesterday":
    blend, frames, width, height, samples = argv[1], int(argv[2]), int(argv[3]), int(argv[4]), int(argv[5])
    project = os.path.dirname(blend)
    planks = np.zeros((256, 256, 3), dtype=np.uint8)
    rows = np.arange(256)[:, None]
    grain = (np.sin(np.arange(256)[None, :] * 0.35 + rows * 0.05) * 18).astype(np.int16)
    base = np.where((rows // 32) % 2 == 0, 150, 115).astype(np.int16) + grain
    planks[..., 0] = np.clip(base + 40, 0, 255)
    planks[..., 1] = np.clip(base - 30, 0, 255)
    planks[..., 2] = np.clip(base - 80, 0, 255)
    a_png(os.path.join(project, "textures", "crate.png"), planks)

    bpy.ops.wm.read_factory_settings(use_empty=True)
    scene = bpy.context.scene
    scene.render.engine = "CYCLES"
    scene.cycles.device = "CPU"
    scene.cycles.samples = samples
    scene.cycles.use_adaptive_sampling = False
    scene.cycles.use_denoising = False
    scene.render.resolution_x = width
    scene.render.resolution_y = height
    scene.render.resolution_percentage = 100
    scene.frame_start = 1
    scene.frame_end = frames
    scene.render.image_settings.file_format = "PNG"
    scene.render.image_settings.color_mode = "RGB"
    scene.render.filepath = "//render/shot_040_"
    world = bpy.data.worlds.new("World")
    scene.world = world
    world.color = (0.03, 0.03, 0.04)

    bpy.ops.mesh.primitive_plane_add(size=30, location=(0, 0, 0))
    floor = bpy.context.active_object
    floor.name = "Floor"
    floor_material = bpy.data.materials.new("Floor")
    tree = nodes_of(floor_material)
    the_node(tree, "BSDF_PRINCIPLED").inputs["Base Color"].default_value = (0.5, 0.5, 0.5, 1)
    the_node(tree, "BSDF_PRINCIPLED").inputs["Roughness"].default_value = 0.5
    floor.data.materials.append(floor_material)

    bpy.ops.mesh.primitive_cube_add(size=2, location=(0, 0, 1))
    crate = bpy.context.active_object
    crate.name = "Crate"
    crate_material = bpy.data.materials.new("Crate")
    crate.data.materials.append(crate_material)
    crate.rotation_euler = (0, 0, 0)
    crate.keyframe_insert("rotation_euler", index=2, frame=1)
    crate.rotation_euler = (0, 0, math.radians(90))
    crate.keyframe_insert("rotation_euler", index=2, frame=frames)

    camera = bpy.data.objects.new("Camera", bpy.data.cameras.new("Camera"))
    scene.collection.objects.link(camera)
    camera.location = (0, -10, 4)
    camera.rotation_euler = (math.radians(70), 0, 0)
    scene.camera = camera
    sun = bpy.data.objects.new("Sun", bpy.data.lights.new("Sun", "SUN"))
    sun.data.energy = 4
    sun.rotation_euler = (math.radians(80), 0, math.radians(20))
    scene.collection.objects.link(sun)

    bpy.ops.wm.save_as_mainfile(filepath=blend)
    tree = nodes_of(crate_material)
    texture = tree.nodes.new("ShaderNodeTexImage")
    texture.image = an_image_beside_the_file(os.path.join(project, "textures", "crate.png"), "//textures/crate.png", False)
    tree.links.new(texture.outputs["Color"], the_node(tree, "BSDF_PRINCIPLED").inputs["Base Color"])
    bpy.ops.wm.save_mainfile()
    said({"saved": blend, "version": list(bpy.app.version), "images": the_images()})

elif mode == "today":
    blend, side = argv[1], int(argv[2])
    project = os.path.dirname(blend)
    the_map = os.path.join(project, "textures", "floor_displacement.png")
    x = np.arange(side)[None, :]
    y = np.arange(side)[:, None]
    height = (127.5 + 127.5 * np.sin(2 * math.pi * x / 32) * np.sin(2 * math.pi * y / 32)).astype(np.uint8)
    a_png(the_map, np.repeat(height[..., None], 3, axis=2))

    bpy.ops.wm.open_mainfile(filepath=blend)
    material = bpy.data.materials["Floor"]
    tree = nodes_of(material)
    texture = tree.nodes.new("ShaderNodeTexImage")
    texture.image = an_image_beside_the_file(the_map, "//textures/floor_displacement.png", True)
    displacement = tree.nodes.new("ShaderNodeDisplacement")
    displacement.inputs["Scale"].default_value = 0.6
    tree.links.new(texture.outputs["Color"], displacement.inputs["Height"])
    tree.links.new(displacement.outputs["Displacement"], the_node(tree, "OUTPUT_MATERIAL").inputs["Displacement"])
    for holder in (material, getattr(material, "cycles", None)):
        try:
            holder.displacement_method = "BUMP"
        except Exception:
            pass
    bpy.ops.wm.save_mainfile()
    said({"saved": blend, "map": the_map, "map_bytes": os.path.getsize(the_map), "images": the_images()})

elif mode == "one-frame":
    blend, frame, out = argv[1], int(argv[2]), argv[3]
    bpy.ops.wm.open_mainfile(filepath=blend)
    scene = bpy.context.scene
    scene.frame_set(frame)
    bpy.ops.render.render()
    bpy.data.images["Render Result"].save_render(filepath=out)
    said({"rendered": out, "frame": frame})

elif mode == "open":
    asked = json.load(open(argv[1]))
    def pixels(path):
        image = bpy.data.images.load(path)
        width, height = image.size
        values = np.empty(width * height * image.channels, dtype=np.float32)
        image.pixels.foreach_get(values)
        values = values.reshape(height, width, image.channels)[..., :3]
        bpy.data.images.remove(image)
        return (width, height), values
    def magenta(values):
        return float(np.mean((values[..., 0] > 0.8) & (values[..., 1] < 0.2) & (values[..., 2] > 0.8)))
    opened = [pixels(path) for path in asked["frames"]]
    against = [pixels(path) for path in asked.get("against", [])]
    found = {
        "opened": len(opened),
        "sizes": sorted({size for size, _ in opened}),
        "magenta_at_most": max(magenta(values) for _, values in opened),
        "mean_brightness": float(np.mean([np.mean(values) for _, values in opened])),
    }
    if against:
        found["each_against_its_pair"] = [float(np.mean(np.abs(a - b))) for (_, a), (_, b) in zip(opened, against)]
    if asked.get("reference"):
        _, reference = pixels(asked["reference"])
        found["the_reference_against_the_first"] = float(np.mean(np.abs(reference - opened[0][1])))
        if against:
            found["the_reference_against_the_pairs_first"] = float(np.mean(np.abs(reference - against[0][1])))
    said(found)
"#;

/// **Blender, on the user's own computer**: the user's own act in `mode`, with
/// the factory-startup switch and a throwaway home so nothing of the
/// Creator's is read or written, and what it found handed back.
fn the_users_blender(blender: &Path, desk: &TheDesk, args: &[String]) -> serde_json::Value {
    let script = desk.at().join("what the user does in blender.py");
    std::fs::write(&script, THE_USERS_BLENDER_SCRIPT).expect("the script is written");
    let home = desk.at().join("the user's home");
    std::fs::create_dir_all(&home).expect("a throwaway home");
    let began = Instant::now();
    let child = Command::new(blender)
        .args(["-b", "--factory-startup", "--python-exit-code", "1", "--python"])
        .arg(&script)
        .arg("--")
        .args(args)
        .env("HOME", &home)
        .current_dir(desk.at())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Blender starts on the user's computer");
    // **A fixed deadline, not a wait**: the run is ended if it is still going
    // when the bound passes.
    let pid = child.id();
    let (ended, watched) = std::sync::mpsc::channel::<()>();
    let watchdog = std::thread::spawn(move || {
        if watched.recv_timeout(ONE_BLENDER_AT_MOST) == Err(std::sync::mpsc::RecvTimeoutError::Timeout) {
            let _ = Command::new("kill").args(["-KILL", &pid.to_string()]).status();
        }
    });
    let output = child.wait_with_output().expect("Blender ends");
    let _ = ended.send(());
    let _ = watchdog.join();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "Blender on the user's computer ({:?}) ended {:?} after {:?}:\n{stdout}\n{}",
        args.first(),
        output.status,
        began.elapsed(),
        String::from_utf8_lossy(&output.stderr)
    );
    let line = stdout
        .lines()
        .rev()
        .find_map(|line| line.strip_prefix("SC5-SAID "))
        .unwrap_or_else(|| panic!("Blender said nothing the test reads:\n{stdout}"));
    serde_json::from_str(line).expect("what Blender said reads")
}

// ════════════════════════════════════════════════════════════════════════════
// One connection, and the passes on it
// ════════════════════════════════════════════════════════════════════════════

/// **The server's connection, as the run holds it**: the point lending it,
/// the run's own holding of the machine's work, and the connection's number.
struct TheConnection<'a> {
    machine: &'a TheDeskMachine,
    desk: &'a TheDesk,
    point: APointLending,
    holds: Arc<WhatThisRunHolds>,
    server: Identity,
    connection: u64,
}

impl TheConnection<'_> {
    /// **One pass of `Desktop app and delivery` P10d**: the read forward, then
    /// the queue reading, landed on this connection — off the test's own
    /// threads, because every ask blocks.
    async fn a_pass(&self) {
        let (mut machine, holds, server, connection, record) = (
            self.machine.the_supervisor_over(&self.point),
            Arc::clone(&self.holds),
            self.server.clone(),
            self.connection,
            Arc::clone(self.desk.record()),
        );
        tokio::task::spawn_blocking(move || {
            let _ = a_read_forward_pass(&server, record.as_ref(), &mut machine, &holds);
            if let Ok(state) = the_queue_reading_from(&mut machine) {
                holds.a_reading_was_taken(&server.as_text(), connection, Reading::taken(state, SystemTime::now(), Clock::ThisComputer), None);
            }
        })
        .await
        .expect("the pass ran");
    }

    /// The job as the period-held reading last read it (`Queue & History` C6).
    fn the_job_as_last_read(&self, job: &Identity) -> Option<smoozy_queue_history::readings::AJobAsTheMachineReadsIt> {
        self.the_last_queue_reading()
            .and_then(|reading| reading.jobs.iter().find(|one| &one.job == job).cloned())
    }

    /// The machine's last queue reading this run holds (`Queue & History` C6).
    fn the_last_queue_reading(&self) -> Option<smoozy_queue_history::acting::adoption::TheMachinesWorkingState> {
        self.holds
            .every_server()
            .into_iter()
            .find(|(held, _)| *held == self.server.as_text())
            .and_then(|(_, period)| period.reading)
            .map(|reading| reading.value().clone())
    }

    /// **Passes, one after another, until `seen`**, each a fixed second apart,
    /// failing the test past `within`.
    async fn passes_until(&self, said: &str, within: Duration, seen: impl Fn(&Self) -> bool) {
        let began = Instant::now();
        loop {
            self.a_pass().await;
            if seen(self) {
                return;
            }
            assert!(
                began.elapsed() < within,
                "{said}: not seen within {within:?}: {}",
                self.machine.the_supervisors_complaints()
            );
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    fn the_row(&self) -> &smoozy_servers::store::ServerRow {
        self.machine.the_row()
    }

    fn server_text(&self) -> String {
        self.server.as_text()
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Files
// ════════════════════════════════════════════════════════════════════════════

/// **The Files screen on one folder of the machine**, as `the_folder_is_read`
/// composes it: the folder read on the connection's reading session, with
/// `File Browser` L11's free-space figure against the same path. Copied from
/// SC4.
async fn files_on(connection: &TheConnection<'_>, folder: &str) -> smoozy_desktop::surface::files::TheBrowser {
    let found = a_folder_is_read(&connection.point.sessions, folder.as_bytes()).await;
    the_tab_after_the_reading(
        TheOrder::default(),
        &connection.the_row().local_server_identifier,
        folder.as_bytes(),
        None,
        found,
        Some(ObservedValue::Connected),
        false,
    )
}

/// **The names Files lists**, each with its size as drawn — **with *Show
/// hidden files* unticked**, as it is until the user ticks it: a row the
/// composition marks hidden is left out of the list by the page, which is
/// where `File Browser` L22's untick happens, unless it carries a refusal.
/// Smoozy's own staging folder is one (`Specs/File Browser`: *"hidden like any
/// other"*).
fn the_rows(browser: &smoozy_desktop::surface::files::TheBrowser) -> Vec<(String, Option<String>)> {
    match &browser.listing {
        TheListing::Rows { rows, .. } => rows
            .iter()
            .filter(|row| !row.hidden || row.refusal.is_some())
            .map(|row| {
                (
                    String::from_utf8_lossy(&row.name.bytes).into_owned(),
                    row.size.as_ref().map(|size| size.as_str().to_string()),
                )
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// **Files' words as the page writes them**, prose and values in their order.
fn as_written(words: &smoozy_desktop::surface::files::Words) -> String {
    words
        .pieces
        .iter()
        .map(|piece| {
            piece
                .said
                .as_ref()
                .map(|said| said.as_str().to_string())
                .or_else(|| piece.value.as_ref().map(|value| value.as_str().to_string()))
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// **The door each upload's publication goes through** — mirrored from the
/// desktop's private `TheNeighboursOfThisMachine` in `src/surface/files.rs`,
/// line for line, over the desk's record and Render Settings' own run-held
/// memory: `File Browser` X7a's writes onto the local record, and the
/// recorded reading of a file published over dropped (`Render Settings` J10).
struct ThePublicationsMarked {
    record: Arc<TheDesksRecord>,
    machine: MachineKey,
    layout: TheLayout,
}

impl WhenAFileIsPublished for ThePublicationsMarked {
    fn a_file_was_published(&self, landing_path: &[u8], outcome: &smoozy_state::outcomes::UnitOutcome) {
        let started = self.record.the_start().lock().ok();
        let Ok(mut held) = the_run_holds().lock() else {
            return;
        };
        let marked = started.as_ref().and_then(|started| started.record.as_ref()).map(|record| {
            smoozy_desktop::record::neighbouring_records::a_publication_through_files_is_marked(
                record.connection(),
                &self.machine,
                &self.layout,
                &mut held.readings,
                landing_path,
                outcome,
            )
        });
        if matches!(marked, Some(Ok(_))) {
            return;
        }
        let mut no_standing_is_kept = Vec::new();
        let no_record_is_kept = smoozy_dependencies::record::InMemoryRecord::default();
        let mut neighbours = smoozy_desktop::record::neighbouring_records::TheNeighbours {
            machine: self.machine.clone(),
            standings: &mut no_standing_is_kept,
            readings: &mut held.readings,
            record: &no_record_is_kept,
            layout: &self.layout,
            came_up_through_files: &smoozy_dependencies::standing::nothing_came_up_through_files,
        };
        smoozy_file_browser::publication::at_this_publication(landing_path, Some(outcome), &mut neighbours);
    }
}

/// **What one upload through Files came to**: the box it asked, where one was
/// asked, and what the files came to.
struct WhatTheUploadDid {
    the_box: Option<TheCollisionQuestion>,
    came_to: WhatTheUploadCameTo,
}

/// **An upload through Files**, in `the_upload_is_confirmed`'s order and then
/// `the_already_exists_question_was_answered`'s: `File Browser` X1's walk with
/// the request written first, the already-exists box composed with its two
/// warnings read from the local record (`File Browser` W2), `answer` given to
/// it where it asks — *Replace* with `File Browser` W8's telling standing first
/// — `File Browser` U4's answer written, and the files sent, each publication
/// marked through [`ThePublicationsMarked`]. **What needs the window is left
/// out**: the picker (the user's choice is `chosen`) and the screens told.
async fn an_upload(
    connection: &TheConnection<'_>,
    chosen: Vec<PathBuf>,
    destination: &str,
    answer: CollisionAnswer,
) -> WhatTheUploadDid {
    let row = connection.the_row().clone();
    let installation = connection.machine.installation().clone();
    let run = connection
        .desk
        .record()
        .the_start()
        .lock()
        .expect("the start")
        .run
        .clone()
        .expect("the start holds its run");
    let asked = WhatIsBeingUploaded {
        server_row: connection.server.clone(),
        installation: Some(installation.clone()),
        destination: destination.as_bytes().to_vec(),
        chosen,
        run,
    };
    let record: &TheDesksRecord = connection.desk.record().as_ref();
    let mut ready = an_upload_is_prepared(&asked, &connection.point.sessions, record)
        .await
        .unwrap_or_else(|why| panic!("`File Browser` X1: the upload is prepared: {why:?}"));
    let mut the_box = None;
    let answer = if ready.something_collides() {
        let reading = connection.the_last_queue_reading();
        let holds = record.read(|c| what_the_record_holds_for_the_row(c, &row, reading.as_ref()));
        let (placed, named, labels) = the_upload_warnings(&ready.the_walk, holds.as_ref());
        the_box = Some(the_collision_question(
            TheCollidingAct::AnUpload,
            &ready.the_walk,
            destination.as_bytes(),
            placed,
            &named,
            &labels,
            ready.the_reason_line.as_ref().map(|line| {
                line.the_last_file_it_landed
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|since| since.as_secs() as i64)
                    .unwrap_or_default()
            }),
        ));
        if answer == CollisionAnswer::Replace {
            assert!(
                the_telling_stands_first(&mut ready, &named, record).is_ok(),
                "`File Browser` W8: the telling stands before anything moves"
            );
        }
        Some(answer)
    } else {
        None
    };
    let left = the_answer_is_applied(&mut ready, answer, &connection.point.sessions, record).await;
    assert_eq!(left, WhatTheAnswerLeft::ItGoes, "`File Browser` U4: the answer is written and the files go");
    let door: Arc<dyn TheRecordAnUploadWrites> = Arc::clone(connection.desk.record()) as Arc<dyn TheRecordAnUploadWrites>;
    let publishing = ThePublicationsMarked {
        record: Arc::clone(connection.desk.record()),
        machine: MachineKey::of(connection.server.clone(), installation),
        layout: connection.machine.layout().clone(),
    };
    let came_to = the_upload_runs(ready, &connection.point.sessions, &TheRunsTransfers::for_this_run(), door, &publishing).await;
    WhatTheUploadDid { the_box, came_to }
}

// ════════════════════════════════════════════════════════════════════════════
// Render Settings
// ════════════════════════════════════════════════════════════════════════════

/// **The page opened on `file`** — what `the_file_was_pointed_at` and
/// `the_version_list_is_asked_for` do when the page opens, mirrored without
/// the window: `File Browser`'s saved-with tag read off the file on the machine
/// (`Render Settings` B2's one source), and the machine's Blenders searched
/// and each asked its version (`Render Settings` B1), both landed in the
/// page's run-held memory.
async fn the_page_is_opened_on(connection: &TheConnection<'_>, file: &str) -> String {
    let server = connection.server_text();
    let saved_with = the_saved_with_version_to_hold(Some(&connection.point.sessions), &server, file.as_bytes(), "").await;
    the_saved_with_version_lands(&mut the_run_holds().lock().expect("the page's memory"), &server, file, &saved_with);
    let own_area = smoozy_server_setup::blender::the_blender_area(connection.machine.layout());
    let asked = the_blenders_on_the_machine(&connection.point.sessions, &server, Some(&own_area)).await;
    the_version_list_came_back(&mut the_run_holds().lock().expect("the page's memory"), &server, asked);
    saved_with
}

/// **What the page reads of the local record before it draws** — the
/// desktop's private `the_record_now` and `the_check_is_read_now`, mirrored:
/// the record's own settings, the server's name, `Rendering` W4a's switch as
/// the shared service answers it, the row's layout, and the dependency band's
/// standing for this file on this machine (`Rendering` W2), read by the page's
/// own `the_check_the_band_reads`.
///
/// **One thing is not the page's**: the system's proposed downloads folder is
/// read from this Mac by `WhatTheRecordHolds::read_now`, and it is the
/// Creator's own. It is put back as a folder of the desk's, so nothing a job
/// records names a folder of the Creator's.
fn what_the_page_reads(connection: &TheConnection<'_>, file: &str) -> WhatTheRecordHolds {
    let row = connection.the_row();
    let started = connection.desk.record().the_start().lock().expect("the start");
    let mut record = WhatTheRecordHolds::read_now(started.record.as_ref());
    record.the_servers_name = Some(row.label.clone().unwrap_or_default());
    record.the_dependency_warning = match smoozy_desktop::surface::said_once::the_warning_is_switched_off(
        &started,
        smoozy_rendering::warning::THE_SUPPRESSION_ROW.0,
        smoozy_rendering::warning::THE_SUPPRESSION_ROW.1,
    ) {
        smoozy_desktop::surface::said_once::WhetherItIsSwitchedOff::Yes => TheSwitchReads::SwitchedOff,
        smoozy_desktop::surface::said_once::WhetherItIsSwitchedOff::No => TheSwitchReads::NotSwitchedOff,
        smoozy_desktop::surface::said_once::WhetherItIsSwitchedOff::TheReadFailed => TheSwitchReads::TheReadFailed,
    };
    record.the_layout = smoozy_desktop::servers::installation::the_layout_recorded_for(row);
    record.the_servers_fallback_folder = row.chosen_fallback_folder.clone();
    let machine = MachineKey::of(connection.server.clone(), connection.machine.installation().clone());
    record.the_check = the_check_the_band_reads(started.record.as_ref().expect("the record").connection(), &machine, file)
        .unwrap_or_default();
    record.the_proposed_folder = Some(
        connection.desk.the_users_folder().join("Downloads").as_os_str().as_encoded_bytes().to_vec(),
    );
    record
}

/// **The page as it is drawn now** — `the_render_settings_panel`'s own
/// composition over what the page reads.
fn the_page(connection: &TheConnection<'_>, file: &str) -> TheRenderSettings {
    let record = what_the_page_reads(connection, file);
    let held = the_run_holds().lock().expect("the page's memory");
    the_panel_starting_from(&held, &connection.server_text(), file, &record)
}

/// **An answer typed on the page**, refused in place where the page refuses it.
fn typed(connection: &TheConnection<'_>, file: &str, which: WhichSetting, text: &str) {
    let refused = the_answer_is_taken(&mut the_run_holds().lock().expect("the page's memory"), &connection.server_text(), file, which, text);
    assert!(refused.is_none(), "the page takes {text:?} for {which:?}: {refused:?}");
}

/// **A sentence as the page writes it.**
fn the_sentence(sentence: &ASentence) -> String {
    sentence
        .iter()
        .map(|words| {
            let mut said = words.lead.as_str().to_string();
            for value in &words.values {
                said.push(' ');
                said.push_str(value.as_str());
            }
            if let Some(rest) = &words.rest {
                if !words.the_rest_joins {
                    said.push(' ');
                }
                said.push_str(rest.as_str());
            }
            said.trim().to_string()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// **The dependency band, as the page draws it**, its sentence and its ways
/// on — `Design/Render Settings § The dependency warning`.
fn the_band_as_drawn(page: &TheRenderSettings) -> Option<(String, String)> {
    page.the_dependency_band.as_ref().map(|band| {
        (
            band.which.as_str().to_string(),
            format!(
                "{} [{}] [{}] [{}] [{}]",
                the_sentence(&band.said),
                band.the_way_to_the_check.as_str(),
                band.render_one_frame_first.as_ref().map(|it| it.as_str()).unwrap_or(""),
                band.submit_anyway.as_str(),
                band.switch_it_off.as_str(),
            ),
        )
    })
}

/// ***Render* pressed on the page**: the act composed as the page composes it
/// (`the_act_as_the_page_composed_it`) and performed by the desktop's own
/// submit act. **One thing of the composition is put back**: the page names
/// Smoozy's own temporary area on this computer, which on this Mac is the
/// Creator's, and the act is given the desk's instead.
async fn render_is_pressed(connection: &TheConnection<'_>, file: &str) -> (WhatTheActCameTo, Identity, String) {
    let record = what_the_page_reads(connection, file);
    let mut given = {
        let held = the_run_holds().lock().expect("the page's memory");
        the_act_as_the_page_composed_it(
            &held,
            &connection.server_text(),
            file,
            &record,
            connection.the_row(),
            connection.desk.record().the_start().lock().expect("the start").run.clone().expect("the run"),
        )
        .unwrap_or_else(|said| panic!("the page composes the act: {}", the_sentence(&said)))
    };
    assert!(!given.the_download_as_it_renders_choice, "auto-download is off");
    given.the_temporary_area = Some(connection.desk.temporary_area().to_path_buf());
    let (sessions, record) = (connection.point.sessions.clone(), Arc::clone(connection.desk.record()));
    let (came_to, jobs) = tokio::task::spawn_blocking(move || {
        the_act_is_performed_naming_its_jobs(given, &sessions, &TheRunsTransfers::for_this_run(), record.as_ref())
    })
    .await
    .expect("the act ran");
    let said = the_sentence(&what_the_act_says(&came_to));
    assert_eq!(
        came_to,
        WhatTheActCameTo::TheseJobs(vec![WhatBecameOfOneJob::ItIsInTheLine]),
        "the job is in the machine's line: {}",
        connection.machine.the_supervisors_complaints()
    );
    (came_to, jobs[0].job.clone(), said)
}

// ════════════════════════════════════════════════════════════════════════════
// A job, from the queue to the user's folder
// ════════════════════════════════════════════════════════════════════════════

/// **Passes until History holds the job finished and its record holds the
/// job's folder** — the moment *Download all to…* has a folder to bring down
/// (SC2 found the two arrive in two writes).
async fn until_it_has_finished(connection: &TheConnection<'_>, job: &Identity, within: Duration) {
    let job_text = job.as_text();
    let layout = connection.machine.layout().clone();
    connection
        .passes_until("History holds the job, finished, with its folder", within, |it| {
            let held = it.desk.record().held();
            let finished = what_is_kept(Some(&*held), None)
                .rows
                .iter()
                .any(|row| row.job == job_text && row.ending == Some(AnEnding::Finished));
            finished
                && JobRecords::over(held.connection())
                    .read(job)
                    .ok()
                    .flatten()
                    .is_some_and(|record| record.the_job_output_folder(&layout).is_some())
        })
        .await;
}

/// **A captured stream read back whole over the reading session** — the
/// desktop's private `OverTheReadingSession` in `src/servers/the_words_kept.rs`,
/// mirrored line for line.
struct TheStreamsReadBack {
    sessions: smoozy_desktop::servers::sessions::ThisServersSessions,
}

impl ReadsAStreamBackWhole for TheStreamsReadBack {
    fn whole(&self, path: &str, length: u64) -> Result<Vec<u8>, String> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|why| why.to_string())?;
        let path = path.as_bytes().to_vec();
        let read = runtime.block_on(self.sessions.a_reading(false, move |subsystem| {
            Box::pin(async move {
                Ok(smoozy_dependencies::acting::checking::read_a_stream_back_over_the_file_session(subsystem, &path, length).await)
            })
        }));
        match read {
            Ok(Ok(read)) => read,
            Ok(Err(failure)) => Err(format!("{failure:?}")),
            Err(not_lent) => Err(format!("{not_lent:?}")),
        }
    }
}

/// **`Queue & History` E9: the words taken in**, as the pass the application
/// starts beside a read forward that learned an ending takes them
/// (`are_taken_in_beside`, which needs the window): the job's captured streams
/// fetched into its history record, so History offers *Blender output*.
async fn the_words_are_taken_in(connection: &TheConnection<'_>) {
    let (server, record, mut machine, reads) = (
        connection.server.clone(),
        Arc::clone(connection.desk.record()),
        connection.machine.the_supervisor_over(&connection.point),
        TheStreamsReadBack {
            sessions: connection.point.sessions.clone(),
        },
    );
    let (did, _) = tokio::task::spawn_blocking(move || {
        the_words_owed_are_taken_in(&server, record.as_ref(), &mut machine, &reads, &HashSet::new())
    })
    .await
    .expect("the words' pass ran");
    assert!(did.landed > 0 || did.still_owed == 0, "`Queue & History` E9: the words landed: {did:?}");
}

/// **The pictures a download brought down**, in their names' order.
fn the_pictures_in(landed: &[PathBuf]) -> Vec<PathBuf> {
    let mut pictures: Vec<PathBuf> = landed.iter().filter(|path| path.extension().is_some_and(|ext| ext == "png")).cloned().collect();
    pictures.sort();
    pictures
}

/// **Every file under a folder on the user's computer**, by its path inside
/// it, in order.
fn the_files_under(folder: &Path) -> Vec<String> {
    let mut found = Vec::new();
    let mut folders = vec![folder.to_path_buf()];
    while let Some(one) = folders.pop() {
        for entry in std::fs::read_dir(&one).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                folders.push(path);
            } else if let Ok(inside) = path.strip_prefix(folder) {
                found.push(inside.display().to_string());
            }
        }
    }
    found.sort();
    found
}

/// A line of parts as the page writes it.
fn parts(parts: &[APart]) -> String {
    parts.iter().map(|part| format!("{}{}{}", part.lead, part.value, part.trail)).collect()
}

/// One step's account line, printed for the output.
fn said(step: &str, what: impl AsRef<str>) {
    println!("SC5 {step}: {}", what.as_ref());
}

// ════════════════════════════════════════════════════════════════════════════
// The scenario
// ════════════════════════════════════════════════════════════════════════════

/// **The user adds a texture and renders again**, its situation and its nine
/// steps.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "run on purpose at the desk with a real Blender: SMOOZY_DESK_BLENDER names its program"]
async fn the_user_adds_a_texture_and_renders_again() {
    let blender = PathBuf::from(
        std::env::var_os("SMOOZY_DESK_BLENDER")
            .expect("SMOOZY_DESK_BLENDER names the Blender this test runs, e.g. /Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
    );
    assert!(blender.is_file(), "{} is a Blender", blender.display());
    let began = Instant::now();
    let desk = TheDesk::opened("sc5").expect("the desk's record opens");
    let Some(machine) = TheDeskMachine::stood_up("sc5", &desk, "RunPod 4090").await else {
        return;
    };
    let server = Identity::read_back(IdentityKind::LocalServer, &machine.the_row().local_server_identifier)
        .expect("the row's own identity");

    // ── The situation ────────────────────────────────────────────────────
    // **"Blender is already installed on the machine."**
    let the_build = machine.a_real_blender_is_installed(&blender);
    // **"Smoozy is connected to it"**: a connection lent by the server's own
    // point, and step 7 run on it as the hand-over runs it.
    let point = machine.a_point_lending().await.expect("a connection through the line is lent");
    let holds = Arc::new(WhatThisRunHolds::default());
    let connection = holds.a_connection_begins(&server.as_text());
    {
        let (mut supervisor, holds, server, record, sessions) = (
            machine.the_supervisor_over(&point),
            Arc::clone(&holds),
            server.clone(),
            Arc::clone(desk.record()),
            point.sessions.clone(),
        );
        let did = tokio::task::spawn_blocking(move || {
            let is_live = || sessions.a_connection_is_live() && holds.is_current(&server.as_text(), connection);
            the_step_seven(&server, connection, record.as_ref(), &mut supervisor, &holds, &is_live)
        })
        .await
        .expect("step 7 ran");
        assert!(did.the_queue_was_read, "`State, identity and recovery` V1 step 5: the queue was read: {did:?}");
    }
    let the_connection = TheConnection {
        machine: &machine,
        desk: &desk,
        point,
        holds,
        server: server.clone(),
        connection,
    };

    // **"Yesterday the user rendered a 200-frame animation from
    // shot_040.blend on this machine."** The project on the user's computer:
    // the `.blend` and the one texture it needs, beside it.
    let project = desk.the_users_folder().join("shot_040");
    let the_users_blend = project.join("shot_040.blend");
    let made = the_users_blender(
        &blender,
        &desk,
        &[
            "yesterday".to_string(),
            the_users_blend.display().to_string(),
            THE_FRAMES.to_string(),
            THE_WIDTH.to_string(),
            THE_HEIGHT.to_string(),
            THE_SAMPLES.to_string(),
        ],
    );
    assert_eq!(made["images"]["crate.png"], "//textures/crate.png", "the file records its texture beside itself: {made}");
    // **"The .blend and every asset it needs went up for that job"**: the
    // project folder sent up through Files into the machine's asset area.
    let the_asset_area = machine.layout().area(Area::Asset);
    let the_folder = format!("{the_asset_area}/shot_040");
    let file = format!("{the_folder}/shot_040.blend");
    let up = an_upload(&the_connection, vec![project.clone()], &the_asset_area, CollisionAnswer::Stop).await;
    assert!(up.the_box.is_none(), "nothing stood there yesterday, so nothing was asked");
    // **Every file of the project folder**: the `.blend`, the backup Blender
    // keeps of it on a second save, and the texture.
    let the_projects_files = the_files_under(&project);
    assert_eq!(up.came_to.uploaded, the_projects_files.len(), "the whole project went up: {:?} of {the_projects_files:?}", up.came_to);
    // **Yesterday's page**: opened, the file read, the range typed, and
    // *Render*.
    let saved_with = the_page_is_opened_on(&the_connection, &file).await;
    let read = the_file_is_read(
        ThisServersMachine {
            sessions: &the_connection.point.sessions,
            package_area: &machine.package_area(),
            server: &server.as_text(),
        },
        the_run_holds(),
        &server.as_text(),
        &file,
    )
    .await;
    assert!(
        matches!(
            &read,
            WhatTheReadCameTo::ThePreLoad {
                went: smoozy_render_settings::acting::pre_load::HowThePreLoadWent::AReading {
                    landed: smoozy_render_settings::file_half::Landed::Stored,
                    ..
                },
                ..
            }
        ),
        "`Render Settings` Q6: yesterday's reading is stored: {read:?}\n{}",
        machine.the_supervisors_complaints()
    );
    let yesterdays_page = the_page(&the_connection, &file);
    assert!(yesterdays_page.a_reading_stands, "the page holds yesterday's reading");
    typed(&the_connection, &file, WhichSetting::JobName, THE_JOBS_NAME);
    typed(&the_connection, &file, WhichSetting::DownloadAsTheyRender, "off");
    typed(&the_connection, &file, WhichSetting::FrameRange, "1-200");
    let (_, yesterdays_job, _) = render_is_pressed(&the_connection, &file).await;
    // **"It finished, and it is in that machine's history marked as
    // finished."**
    until_it_has_finished(&the_connection, &yesterdays_job, Duration::from_secs(400)).await;
    // **"The user has already downloaded yesterday's frames and watched them
    // back."** *Download all to…*, then the frames opened.
    let yesterdays_frames_here = desk.the_users_folder().join("shot_040 frames, yesterday");
    let downloaded = a_whole_job_is_downloaded_to(&desk, &the_connection.point.sessions, &yesterdays_job, &yesterdays_frames_here)
        .await
        .expect("`Download` M1: yesterday's whole job is brought down");
    let yesterdays_frames = the_pictures_in(&downloaded.landed);
    assert_eq!(yesterdays_frames.len() as i64, THE_FRAMES, "all 200 frames came down: {downloaded:?}");
    let listing = desk.at().join("yesterday's frames.json");
    std::fs::write(&listing, serde_json::json!({ "frames": yesterdays_frames }).to_string()).expect("the list");
    let watched = the_users_blender(&blender, &desk, &["open".to_string(), listing.display().to_string()]);
    assert_eq!(watched["opened"], THE_FRAMES, "{watched}");
    assert_eq!(watched["sizes"], serde_json::json!([[THE_WIDTH, THE_HEIGHT]]), "{watched}");
    assert_eq!(
        watched["magenta_at_most"].as_f64(),
        Some(0.0),
        "`Specs/What a job writes`: a picture the render could not find comes out flat magenta, and yesterday's had \
         every one: {watched}"
    );
    said(
        "SITUATION (played)",
        format!(
            "RunPod 4090 connected, step 7 run; the user's Blender is {:?}, the machine's is {} (the page read the \
             file as saved with {saved_with:?}); shot_040/ with its .blend and textures/crate.png went up through \
             Files ({} files); yesterday's page read the file, *Render* sent Shot 40 over 1-200, it finished, and \
             *Download all to…* brought {} frames down, which open at {}×{} with no flat magenta anywhere, so the crate's \
             texture was found ({:?} in)",
            made["version"],
            the_build.display(),
            up.came_to.uploaded,
            yesterdays_frames.len(),
            THE_WIDTH,
            THE_HEIGHT,
            began.elapsed(),
        ),
    );
    {
        let held = desk.record().held();
        let screen = what_is_held(&the_connection.holds, Some(&*held), None);
        assert!(screen.lines.iter().all(|line| line.rows.is_empty()), "`Queue & History` C11: nothing is queued or running");
    }

    // ── Step 1: watching yesterday's frames, the user decides the floor
    // needs a texture ─────────────────────────────────────────────────────
    said("STEP 1 (nothing of Smoozy's to play)", "the user decides, watching the frames already opened above");

    // ── Step 2: the floor points at a displacement map, and the file is saved
    let changed = the_users_blender(&blender, &desk, &["today".to_string(), the_users_blend.display().to_string(), THE_MAPS_SIDE.to_string()]);
    assert_eq!(changed["images"]["floor_displacement.png"], "//textures/floor_displacement.png", "{changed}");
    let the_map_here = project.join("textures").join("floor_displacement.png");
    assert!(the_map_here.is_file(), "the map sits in the project's texture folder on the user's computer");
    said(
        "STEP 2 (played, in Blender on the user's computer)",
        format!(
            "the floor's material points at //textures/floor_displacement.png ({} bytes, {THE_MAPS_SIDE}×{THE_MAPS_SIDE}) \
             through a displacement node, and shot_040.blend is saved",
            changed["map_bytes"],
        ),
    );

    // ── Step 3: Files on RunPod 4090, and the .blend sent up to its folder ─
    let before = files_on(&the_connection, &the_folder).await;
    let textures_there = files_on(&the_connection, &format!("{the_folder}/textures")).await;
    let the_textures: Vec<String> = the_rows(&textures_there).into_iter().map(|(name, _)| name).collect();
    assert_eq!(the_textures, vec!["crate.png".to_string()], "\"That map has never been on the machine\"");
    let up = an_upload(&the_connection, vec![the_users_blend.clone()], &the_folder, CollisionAnswer::Replace).await;

    // ── Step 4: Smoozy asks about the file already being there; Replace ──
    let the_box = up.the_box.as_ref().expect("`File Browser` U4: the box asks, because shot_040.blend is there");
    let answers: Vec<String> = the_box.answers.iter().map(|one| one.label.as_str().to_string()).collect();
    assert!(answers.iter().any(|label| label == "Replace"), "the box offers Replace: {answers:?}");
    assert_eq!(the_box.names.len(), 1, "one file collides");
    assert!(the_box.live_jobs.is_none(), "`File Browser` W5a: no live job reads the file, so no live-job warning");
    assert_eq!(up.came_to.uploaded, 1, "`File Browser` U11: the one file uploaded: {:?}", up.came_to);
    assert!(up.came_to.failed.is_empty() && up.came_to.skipped == 0 && up.came_to.did_not_finish == 0, "{:?}", up.came_to);
    let after = files_on(&the_connection, &the_folder).await;
    let on_the_machine = std::fs::read(&file).expect("the machine's copy reads");
    assert_eq!(on_the_machine, std::fs::read(&the_users_blend).expect("the user's reads"), "the machine holds today's file");
    let textures_after: Vec<String> = the_rows(&files_on(&the_connection, &format!("{the_folder}/textures")).await)
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    assert_eq!(textures_after, vec!["crate.png".to_string()], "nothing sent the map up");
    said(
        "STEPS 3 AND 4 (played)",
        format!(
            "Files on {the_folder} lists {:?}, and textures/ lists {the_textures:?}; the user sends shot_040.blend up \
             there; the box says {:?} — names {:?}, answers {answers:?}, no live-job warning{} — and the user picks \
             Replace; it says the upload came to {} uploaded, and Files then lists {:?}; textures/ still lists \
             {textures_after:?} (`File Browser` X1, `File Browser` U4, `File Browser` U11)",
            the_rows(&before),
            as_written(&the_box.headline),
            the_box.names.iter().map(|name| name.as_str().to_string()).collect::<Vec<_>>(),
            the_box
                .interrupted
                .as_ref()
                .map(|line| format!(", and the line {:?}", as_written(line)))
                .unwrap_or_default(),
            up.came_to.uploaded,
            the_rows(&after),
        ),
    );

    // ── Step 5: the render settings, and the same 200-frame range ────────
    let saved_with = the_page_is_opened_on(&the_connection, &file).await;
    let page = the_page(&the_connection, &file);
    assert!(
        !page.a_reading_stands,
        "`Render Settings` J10: yesterday's reading went with the file it described, so the page shows no values \
         read from a file that is no longer there"
    );
    assert!(page.the_read_is_offered, "*Read the file* is offered again");
    typed(&the_connection, &file, WhichSetting::FrameRange, "1-200");
    let page = the_page(&the_connection, &file);
    assert_eq!(
        page.the_range.value.as_ref().map(|value| value.as_str().to_string()).as_deref(),
        Some("1-200"),
        "the range field reads what the user typed"
    );
    said(
        "STEP 5 (played)",
        format!(
            "the page opens on shot_040.blend saved with {saved_with:?}, versions {:?}; no reading stands (yesterday's \
             went with the file it described, `Render Settings` J10) and *{}* is offered; the user types 1-200 and the \
             range reads {:?} — {}; the job's name still reads {:?} and auto-download {}",
            page.the_versions.iter().map(|one| format!("{} {}", one.answered.as_str(), one.build.as_str())).collect::<Vec<_>>(),
            page.read_the_file.as_str(),
            page.the_range.value.as_ref().map(|value| value.as_str()),
            the_sentence(&page.the_frame_count),
            page.the_job_name.as_str(),
            if page.the_download_is_on { "on" } else { "off" },
        ),
    );

    // ── Step 6: the user submits, and looks at what Smoozy shows ─────────
    let (which, band) = the_band_as_drawn(&page).expect("`Rendering` W1: the warning is drawn, as nothing shows this file was checked");
    assert_eq!(which, "never", "`Rendering` W2: nobody has checked this file on this server");
    assert!(band.contains("RunPod 4090"), "the band names the server by what the user calls it: {band}");
    let (_, todays_job, the_answer) = render_is_pressed(&the_connection, &file).await;
    said(
        "STEP 6 (played)",
        format!(
            "before *Render* the page draws the dependency band {which:?}: {band:?} (`Rendering` W1, `Rendering` W2, \
             `Design/Render Settings § The dependency warning`); its band line reads {:?}; the user presses \
             *{}*, and the act says {the_answer:?} (`Rendering` S10)",
            the_sentence(&page.the_band),
            the_sentence(&page.the_render_control),
        ),
    );

    // ── Step 7: the user looks at the queue ──────────────────────────────
    let todays_text = todays_job.as_text();
    the_connection
        .passes_until("the queue reads today's job rendering with a frame saved", Duration::from_secs(120), |it| {
            it.the_job_as_last_read(&todays_job).is_some_and(|read| {
                matches!(read.state, QueueState::Rendering { .. })
                    && read.live.the_figures().is_some_and(|figures| figures.newest_frame_saved.is_some())
            })
        })
        .await;
    {
        let held = desk.record().held();
        let screen = what_is_held(&the_connection.holds, Some(&*held), Some(&todays_text));
        let entry = screen.entry.as_ref().expect("today's entry");
        assert_eq!(entry.name, THE_JOBS_NAME);
        let marks: Vec<String> = entry.marks.iter().map(|mark| format!("{}{}", mark.said, parts(&mark.parts))).collect();
        said(
            "STEP 7 (played)",
            format!(
                "the queue holds {} job, {:?}: status {:?}, progress {:?}; its marks {marks:?}",
                screen.jobs,
                entry.name,
                parts(&entry.status),
                entry.progress.as_ref().map(|progress| (parts(&progress.now), progress.now_at, progress.whole)),
            ),
        );
    }

    // ── Step 8: the job finishes; History, and the frames downloaded ─────
    until_it_has_finished(&the_connection, &todays_job, Duration::from_secs(400)).await;
    the_words_are_taken_in(&the_connection).await;
    let todays_frames_here = desk.the_users_folder().join("shot_040 frames, today");
    {
        let held = desk.record().held();
        let history = what_is_kept(Some(&*held), Some(&todays_text));
        let row = history.rows.iter().find(|row| row.job == todays_text).expect("today's job is in History");
        assert_eq!(row.ending, Some(AnEnding::Finished), "History says it finished: {row:?}");
        let entry = history.entry.as_ref().expect("its History entry");
        let record = JobRecords::over(held.connection()).read(&todays_job).expect("it reads").expect("it stands");
        assert_eq!(record.the_ending_learned(), Some(AJobsEnding::Finished));
        let marks: Vec<String> = entry.entry.marks.iter().map(|mark| format!("{}{}", mark.said, parts(&mark.parts))).collect();
        assert!(
            marks.iter().any(|mark| mark.starts_with("Dependencies were not checked")),
            "`Queue & History` N15: the entry says nothing checked the file's dependencies: {marks:?}"
        );
        assert!(entry.blender_output, "`Queue & History` E9: the words are kept, so History offers *Blender output*");
        // **What *Blender output* holds**: the words kept with the job, read
        // from the record as the panel reads them.
        let kept = smoozy_desktop::record::kept_words::the_words_kept_for(held.connection(), &todays_job).expect("the words read");
        let naming_the_map: Vec<String> = kept
            .iter()
            .flat_map(|one| String::from_utf8_lossy(&one.the_words).lines().map(str::to_string).collect::<Vec<_>>())
            .filter(|line| line.contains("floor_displacement"))
            .collect();
        said(
            "STEP 8 (played)",
            format!(
                "History holds {} jobs; today's reads {:?} {:?}, frames {:?}; its entry's marks {marks:?} \
                 (`Queue & History` N15); it offers *Blender output* (`Queue & History` E9), whose words name the \
                 map in {} lines, the first {:?}",
                history.jobs,
                row.name,
                row.ending,
                row.frames,
                naming_the_map.len(),
                naming_the_map.first().map(|line| line.trim()),
            ),
        );
    }
    let downloaded = a_whole_job_is_downloaded_to(&desk, &the_connection.point.sessions, &todays_job, &todays_frames_here)
        .await
        .expect("`Download` M1: today's whole job is brought down");
    let todays_frames = the_pictures_in(&downloaded.landed);
    assert_eq!(todays_frames.len() as i64, THE_FRAMES, "`Download` M16: all 200 frames came down: {downloaded:?}");
    said(
        "STEP 8 (played)",
        format!(
            "*Download all to…* asked {} and brought {} frames down into the user's own folder (`Download` M16)",
            if downloaded.it_asked { "first" } else { "nothing" },
            downloaded.came_to.fetched,
        ),
    );

    // ── Step 9: the user opens the frames ────────────────────────────────
    // **What the floor looks like with the map, rendered on the user's own
    // computer, where the map is** — the picture the user meant, frame 1.
    let the_picture_meant = desk.at().join("frame 1 as the user meant it.png");
    the_users_blender(
        &blender,
        &desk,
        &["one-frame".to_string(), the_users_blend.display().to_string(), "1".to_string(), the_picture_meant.display().to_string()],
    );
    let listing = desk.at().join("today's frames.json");
    std::fs::write(
        &listing,
        serde_json::json!({ "frames": todays_frames, "against": yesterdays_frames, "reference": the_picture_meant }).to_string(),
    )
    .expect("the list");
    let opened = the_users_blender(&blender, &desk, &["open".to_string(), listing.display().to_string()]);
    assert_eq!(opened["opened"], THE_FRAMES, "{opened}");
    assert_eq!(opened["sizes"], serde_json::json!([[THE_WIDTH, THE_HEIGHT]]), "{opened}");
    let each: Vec<f64> = opened["each_against_its_pair"]
        .as_array()
        .expect("each frame against yesterday's")
        .iter()
        .filter_map(serde_json::Value::as_f64)
        .collect();
    let most_against_yesterday = each.iter().copied().fold(0.0_f64, f64::max);
    let meant_against_yesterday = opened["the_reference_against_the_pairs_first"].as_f64().expect("the picture meant against yesterday's");
    let meant_against_today = opened["the_reference_against_the_first"].as_f64().expect("the picture meant against today's");
    // **The map shows where it is**: the picture the user meant is not
    // yesterday's, so a floor carrying the map can be told from one without.
    assert!(
        meant_against_yesterday > 0.01,
        "with the map, frame 1 differs from yesterday's: {opened}"
    );
    // **And today's frames are yesterday's**: the machine's Blender did not
    // find a map nobody sent, the render finished as a success, and nothing on
    // the floor changed — `Specs/Dependency Upload & Remap`, *Iterating*: *"a
    // texture linked for the first time is not on the machine, nothing is
    // checked at a submission, and the render will not find it"*; and `Specs/What
    // a job writes`: a missing displacement shows nothing in the picture.
    assert!(
        most_against_yesterday < meant_against_yesterday / 4.0 && meant_against_today > meant_against_yesterday / 2.0,
        "today's frames do not carry the map: {opened}"
    );
    said(
        "STEP 9 (played, in Blender on the user's computer)",
        format!(
            "the {} frames open at {}×{}, flat magenta at most {:.4} of a frame; against yesterday's, frame for frame, \
             they differ by at most {most_against_yesterday:.5} (mean {:.5}); frame 1 rendered on the user's computer, \
             where the map is, differs from yesterday's by {meant_against_yesterday:.5} and from today's by \
             {meant_against_today:.5} — so today's floor carries no map, as the plans say a texture never sent up \
             renders (`Specs/Dependency Upload & Remap`, *Iterating*); nothing in the picture says the map is \
             missing, and what said so was the band at step 6 and Blender's own line behind *Blender output* at \
             step 8 ({:?} in all)",
            opened["opened"],
            THE_WIDTH,
            THE_HEIGHT,
            opened["magenta_at_most"].as_f64().unwrap_or_default(),
            each.iter().sum::<f64>() / each.len().max(1) as f64,
            began.elapsed(),
        ),
    );
}

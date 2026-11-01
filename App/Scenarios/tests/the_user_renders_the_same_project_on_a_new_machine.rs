//! **`Preparation/Scenarios/The user renders the same project on a new
//! machine.md`, played at the desk with a real Blender** — step 12's piece SC7.
//!
//! The scenario's situation and its nine steps, unchanged, through the
//! application's own acts, against two desk machines one after the other. At
//! each step the test prints what the user did, what Smoozy hands the page to
//! show, and which clause says so, and asserts all three. **A step the desk
//! cannot play is said in the output and never skipped quietly.**
//!
//! **Ignored, and run on purpose with the Blender's path set**, the way SC5
//! (`tests/the_user_adds_a_texture_and_renders_again.rs`) is:
//!
//! ```text
//! SMOOZY_DESK_BLENDER='/Applications/Blender 5.1.2.app/Contents/MacOS/Blender' \
//! cargo test -p smoozy-scenarios --test the_user_renders_the_same_project_on_a_new_machine \
//!   -- --ignored --nocapture
//! ```
//!
//! **The sizes, as SC5's.** 200 frames, as the scenario says; 160 × 90
//! pixels at 100 %, 8 samples, Cycles on the CPU, no denoising, no adaptive
//! sampling — the week before. **This morning's higher resolution is 200 %**,
//! so the new job's frames are 320 × 180. The project's one texture is a
//! 256 × 256 PNG of planks the test writes itself.
//!
//! **What is real here.** OpenSSH's own server behind the harness's line, for
//! each machine; the application's own transport and each server's own point;
//! `Servers` Q1 step 7 on each connection; Files' own upload — the walk, the
//! already-exists box where one asks, the answer written, the files sent and
//! each publication marked; Render Settings' own page — the saved-with tag, the
//! machine's Blenders searched and asked their version, *Read the file*
//! through the machine's file-reading worker, the answers typed and the act
//! composed — and the desktop's own submit act, its remap composed from the
//! record; the removal box and the removal the window's *Remove* runs;
//! *Submit again*'s own composition (`the_press_composed`) and the one write
//! into the page's store (`the_setup_is_loaded`); *Check dependencies…*
//! through the desktop's own sitting (`the_check_runs`) and its own
//! resolution of the Blender it reads with; the found files sent by the
//! desktop's own send (`the_found_files_go_up`) and the check run again by
//! its own rule (`the_check_runs_again_after`); the queue and History
//! screens' own readings; a whole job's *Download all to…*; and the machine's
//! own program, its real supervisor, render worker and file-inspection worker.
//! **And Blender itself**: one of this Mac's own, started on each machine by
//! the machine's program, and on the user's computer by the test to make the
//! user's file and open frames.
//!
//! **Where the application's own wiring needs the window, it is mirrored and
//! said**, as in SC5, whose connection, passes, upload and page this file
//! copies. **Three more of the desktop's private pieces are mirrored here, and
//! each says so where it stands**: what *Submit again*'s press reads of the
//! record before it composes (`what_the_record_holds`, which needs the
//! window's state); the run-held pick that waits for a file, which the test
//! holds itself and lands with the same one write `the_pick_lands` makes; and
//! the gathering *Check dependencies…* does before its sitting
//! (`the_check_is_begun`).
//!
//! **What the desk cannot do.**
//!
//! - **A week.** Last week is a minute ago, in the same sitting of Smoozy.
//! - **The provider's destroy.** The harness's destroy act stands in: the
//!   open connection is reset and every new one refused, as SC2 plays it.
//! - **A Linux machine and the user's computer apart.** Both machines and the
//!   user's computer are one Mac, so a file the machine's Blender looks for by
//!   an absolute path on the user's computer would be found. **The user's file
//!   records its texture relative to itself**, and this morning the `.blend`
//!   alone sits on the new machine, so its Blender finds the texture only
//!   where the remap points it.
//! - **The machine's Blender is started through a launcher that adds the
//!   factory-startup switch** (the harness's `a_real_blender_is_installed`).
//! - **What the machine holds is read off its disk** at the desk, which a
//!   user could not do; the scenario's own user only looks at the queue and
//!   at History, and the test says which reading is the desk's.
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

use smoozy_dependencies::bytes::{MachineLocation, StoredLocation};
use smoozy_dependencies::record::{HoldingFile, MachineKey};
use smoozy_dependencies::rows::RowState;
use smoozy_dependencies::standing::Completion;
use smoozy_desktop::a_new_job_from_an_old_one::{
    the_press_composed, the_setup_is_loaded, TheBar, ThePressComposed, TheWayIn, WhatTheBlendIs, WhatTheServerIs,
    WhereThePressLeads,
};
use smoozy_desktop::checking_a_file::{the_check_runs, TheCheckCameTo, TheSittingIsOf, TheUsersOwnComputer};
use smoozy_desktop::downloading::TheRunsTransfers;
use smoozy_desktop::reading_the_file::{the_file_is_read, ThisServersMachine, WhatTheReadCameTo};
use smoozy_desktop::record::dependency_records::TheDependencyRecord;
use smoozy_desktop::record::job_records::{JobRecords, TheKeptSetup};
use smoozy_desktop::record::server_rows;
use smoozy_desktop::servers::a_period::the_facts_behind;
use smoozy_desktop::servers::the_machines_work::{a_read_forward_pass, the_queue_reading_from, the_step_seven, WhatThisRunHolds};
use smoozy_desktop::servers::the_words_kept::{the_words_owed_are_taken_in, ReadsAStreamBackWhole};
use smoozy_desktop::submitting::{the_act_is_performed_naming_its_jobs, the_blenders_on_the_machine, WhatBecameOfOneJob, WhatTheActCameTo};
use smoozy_desktop::surface::files::{
    a_folder_is_read, the_collision_question, the_find_bar, the_tab_after_the_reading, the_upload_warnings,
    what_the_record_holds_for_the_row, ABarAcrossTheTop, TheCollidingAct, TheCollisionQuestion, TheListing, TheOrder,
};
use smoozy_desktop::surface::render_settings::{
    the_act_as_the_page_composed_it, the_answer_is_taken, the_check_the_band_reads, the_panel_starting_from, the_run_holds,
    the_saved_with_version_lands, the_saved_with_version_to_hold, the_version_list_came_back, what_the_act_says, ASentence,
    TheRenderSettings, TheSwitchReads, WhatTheRecordHolds, WhichSetting,
};
use smoozy_desktop::surface::servers::{the_removal_box_for, the_removal_is_performed_and_its_drawings_go, TheAnswer};
use smoozy_desktop::surface::the_dependency_check::{the_blender_the_check_reads_with, the_check_runs_again_after, the_panel};
use smoozy_desktop::surface::the_history::{what_is_kept, AnEnding};
use smoozy_desktop::surface::the_queue::{what_is_held, APart};
use smoozy_desktop::the_found_files_sent::{the_found_files_go_up, the_rows_the_upload_sends, WhatIsSent, WhatTheSendCameTo};
use smoozy_desktop::uploading::{
    an_upload_is_prepared, the_answer_is_applied, the_telling_stands_first, the_upload_runs, TheRecordAnUploadWrites,
    WhatIsBeingUploaded, WhatTheAnswerLeft, WhatTheUploadCameTo, WhenAFileIsPublished,
};
use smoozy_file_browser::already_exists::CollisionAnswer;
use smoozy_file_browser::request::KeepsTheRequest as _;
use smoozy_file_browser::uploading::{where_it_came_from, WhereItCameFrom};
use smoozy_job_files::layout::{Area, TheLayout};
use smoozy_queue_history::ending::Ending as AJobsEnding;
use smoozy_queue_history::line::QueueState;
use smoozy_scenarios::the_desk_machine::{
    a_whole_job_is_downloaded_to, APointLending, HowTheMachineGoes, TheDesk, TheDeskMachine, TheDesksRecord,
};
use smoozy_servers::condition::ObservedValue;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, Reading};

// ════════════════════════════════════════════════════════════════════════════
// The sizes
// ════════════════════════════════════════════════════════════════════════════

/// **The scenario's own 200 frames.**
const THE_FRAMES: i64 = 200;

/// **The picture last week, and how hard Cycles works on it** — SC5's sizes.
const THE_WIDTH: u32 = 160;
const THE_HEIGHT: u32 = 90;
const THE_SAMPLES: u32 = 8;

/// **This morning's higher resolution**, as the user types it on the page.
const THE_HIGHER_RESOLUTION: u32 = 200;

/// **How long one Blender on the user's computer may take** — a test's own
/// bound, so a Blender that never ends fails the run rather than holding it.
const ONE_BLENDER_AT_MOST: Duration = Duration::from_secs(300);

/// The job's name, as the user typed it last week.
const THE_JOBS_NAME: &str = "Shot 40";

/// The two machines, as the user calls them.
const LAST_WEEKS_MACHINE: &str = "RunPod 4090";
const THE_NEW_MACHINE: &str = "RunPod 4090 (2)";

// ════════════════════════════════════════════════════════════════════════════
// Blender on the user's own computer
// ════════════════════════════════════════════════════════════════════════════

/// **What the user does in Blender on their own computer, as one script**:
/// make the project's file a week ago, and open frames to look at them. Each
/// act prints one line, `SC7-SAID` followed by what it found, which the test
/// reads. The texture is written by the script itself as a plain PNG from
/// Python's own `zlib`. The scene is SC5's.
const THE_USERS_BLENDER_SCRIPT: &str = r#"
import bpy, sys, os, json, math, zlib, struct
import numpy as np

argv = sys.argv[sys.argv.index("--") + 1:]
mode = argv[0]

def said(what):
    print("SC7-SAID " + json.dumps(what), flush=True)

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

def the_images():
    return {image.name: image.filepath for image in bpy.data.images if image.source == "FILE"}

if mode == "last-week":
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
    image = bpy.data.images.load(os.path.join(project, "textures", "crate.png"))
    image.filepath = "//textures/crate.png"
    texture.image = image
    tree.links.new(texture.outputs["Color"], the_node(tree, "BSDF_PRINCIPLED").inputs["Base Color"])
    bpy.ops.wm.save_mainfile()
    said({"saved": blend, "version": list(bpy.app.version), "images": the_images()})

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
    said({
        "opened": len(opened),
        "sizes": sorted({size for size, _ in opened}),
        "magenta_at_most": max(magenta(values) for _, values in opened),
        "mean_brightness": float(np.mean([np.mean(values) for _, values in opened])),
    })
"#;

/// **Blender, on the user's own computer**: the user's own act in `mode`, with
/// the factory-startup switch and a throwaway home so nothing of the
/// Creator's is read or written, and what it found handed back. Copied from
/// SC5.
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
        .find_map(|line| line.strip_prefix("SC7-SAID "))
        .unwrap_or_else(|| panic!("Blender said nothing the test reads:\n{stdout}"));
    serde_json::from_str(line).expect("what Blender said reads")
}

/// **Frames opened in Blender on the user's computer**: how many, their
/// sizes, and how much of any frame is flat magenta.
fn the_frames_opened(blender: &Path, desk: &TheDesk, named: &str, frames: &[PathBuf]) -> serde_json::Value {
    let listing = desk.at().join(format!("{named}.json"));
    std::fs::write(&listing, serde_json::json!({ "frames": frames }).to_string()).expect("the list");
    the_users_blender(blender, desk, &["open".to_string(), listing.display().to_string()])
}

// ════════════════════════════════════════════════════════════════════════════
// One connection, and the passes on it
// ════════════════════════════════════════════════════════════════════════════

/// **A server's connection, as the run holds it**: the point lending it, the
/// run's own holding of the machines' work — one for the whole run, both
/// machines' — and the connection's number. Copied from SC5.
struct TheConnection<'a> {
    machine: &'a TheDeskMachine,
    desk: &'a TheDesk,
    point: APointLending,
    holds: Arc<WhatThisRunHolds>,
    server: Identity,
    connection: u64,
}

/// **"Smoozy is connected to it"**: a connection lent by the server's own
/// point, and step 7 run on it as the hand-over runs it.
async fn connected<'a>(machine: &'a TheDeskMachine, desk: &'a TheDesk, holds: &Arc<WhatThisRunHolds>) -> TheConnection<'a> {
    let server = Identity::read_back(IdentityKind::LocalServer, &machine.the_row().local_server_identifier)
        .expect("the row's own identity");
    let point = machine.a_point_lending().await.expect("a connection through the line is lent");
    let connection = holds.a_connection_begins(&server.as_text());
    {
        let (mut supervisor, holds, server, record, sessions) = (
            machine.the_supervisor_over(&point),
            Arc::clone(holds),
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
    TheConnection {
        machine,
        desk,
        point,
        holds: Arc::clone(holds),
        server,
        connection,
    }
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

    /// **The machine key a check and a publication through Files are keyed
    /// by** — the row's server under its installation.
    fn machine_key(&self) -> MachineKey {
        MachineKey::of(self.server.clone(), self.machine.installation().clone())
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Files
// ════════════════════════════════════════════════════════════════════════════

/// **The Files screen on one folder of the machine**, as `the_folder_is_read`
/// composes it. Copied from SC5.
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

/// **The names Files lists**, each with its size as drawn, with *Show hidden
/// files* unticked. Copied from SC5.
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
/// line for line. Copied from SC5.
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
/// `the_already_exists_question_was_answered`'s. Copied from SC5.
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
        machine: connection.machine_key(),
        layout: connection.machine.layout().clone(),
    };
    let came_to = the_upload_runs(ready, &connection.point.sessions, &TheRunsTransfers::for_this_run(), door, &publishing).await;
    WhatTheUploadDid { the_box, came_to }
}

// ════════════════════════════════════════════════════════════════════════════
// Render Settings
// ════════════════════════════════════════════════════════════════════════════

/// **The saved-with tag read off the file on the machine and landed in the
/// page's memory** — the first half of `the_file_was_pointed_at`, mirrored
/// without the window (`Render Settings` B2's one source).
async fn the_saved_with_tag_lands(connection: &TheConnection<'_>, file: &str) -> String {
    let server = connection.server_text();
    let saved_with = the_saved_with_version_to_hold(Some(&connection.point.sessions), &server, file.as_bytes(), "").await;
    the_saved_with_version_lands(&mut the_run_holds().lock().expect("the page's memory"), &server, file, &saved_with);
    saved_with
}

/// **The machine's Blenders searched and each asked its version** —
/// `the_version_list_is_asked_for`, mirrored without the window (`Render
/// Settings` B1).
async fn the_version_list_is_asked_for(connection: &TheConnection<'_>) {
    let server = connection.server_text();
    let own_area = smoozy_server_setup::blender::the_blender_area(connection.machine.layout());
    let asked = the_blenders_on_the_machine(&connection.point.sessions, &server, Some(&own_area)).await;
    the_version_list_came_back(&mut the_run_holds().lock().expect("the page's memory"), &server, asked);
}

/// **What the page reads of the local record before it draws** — the
/// desktop's private `the_record_now` and `the_check_is_read_now`, mirrored.
/// Copied from SC5, with its one thing put back: the system's proposed
/// downloads folder is a folder of the desk's, not the Creator's.
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
    record.the_check = the_check_the_band_reads(started.record.as_ref().expect("the record").connection(), &connection.machine_key(), file)
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

/// ***Render* pressed on the page**: the act composed as the page composes it
/// and performed by the desktop's own submit act. Copied from SC5, with its
/// one thing put back: Smoozy's own temporary area is the desk's.
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
// Submit again
// ════════════════════════════════════════════════════════════════════════════

/// ***Submit again* pressed on an entry** — `the_press_is_taken`, whose first
/// half reads the window's state, mirrored: the entry and its kept setup read
/// from the record (`Queue & History` N10), every server row read, whether
/// the entry's own server still stands (`Queue & History` N17's draw-time
/// read), and which servers have a live connection, which `Rendering` A1a's
/// opening rule counts. **Then the desktop's own composition**,
/// `the_press_composed`. The `.blend`'s attribute read (`Rendering` A10) is
/// taken only where the entry's own server is connected, which it is not here.
fn submit_again_is_pressed(desk: &TheDesk, job: &Identity, live: &[&TheConnection<'_>]) -> ThePressComposed {
    let (record, kept, rows) = desk.record().read(|c| {
        let records = JobRecords::over(c);
        let record = records.read(job).expect("the entry reads").expect("the entry stands");
        let kept = match records.the_kept_setup_of(job).expect("the kept setup reads") {
            Some(TheKeptSetup::Kept(payload)) => payload,
            other => panic!("`Queue & History` N10: the entry keeps its setup: {other:?}"),
        };
        (record, kept, server_rows::all(c).expect("the rows read"))
    });
    let its_server = record.server.local_server.as_text();
    let the_server_stands = rows.iter().any(|row| row.local_server_identifier == its_server);
    let connected: Vec<String> = rows
        .iter()
        .map(|row| row.local_server_identifier.clone())
        .filter(|server| live.iter().any(|one| &one.server_text() == server && one.point.sessions.a_connection_is_live()))
        .collect();
    let server = if the_server_stands {
        if live.iter().any(|one| one.server_text() == its_server) {
            WhatTheServerIs::Connected
        } else {
            WhatTheServerIs::NotConnected
        }
    } else {
        WhatTheServerIs::Gone { connected }
    };
    the_press_composed(&record, &kept, &TheWayIn::SubmitAgain, &server, WhatTheBlendIs::There, None, None)
        .unwrap_or_else(|said| panic!("`Rendering` A1: the press composes: {said}"))
}

// ════════════════════════════════════════════════════════════════════════════
// Check dependencies…
// ════════════════════════════════════════════════════════════════════════════

/// The key the check's run-held memory is kept under.
fn the_checks_key(server: &str, file: &str) -> String {
    format!("{server}\u{1f}{file}")
}

/// ***Check dependencies…* on a `.blend` in Files** — `the_check_is_begun`'s
/// gathering, mirrored (it needs the window's state), and then the desktop's
/// own sitting, `the_check_runs`, over the server's own point onto the desk's
/// record:
///
/// - **`Dependency Upload & Remap` G2: the Blender the job will render
///   with**, resolved by the desktop's own `the_blender_the_check_reads_with`
///   over the page's memory, which asks the machine for its Blenders and reads
///   the file's saved-with tag where the page holds neither.
/// - **`Dependency Upload & Remap` K3's holding file**: `File Browser` N6's
///   answer from the upload records, else `Dependency Upload & Remap` K5's machine
///   location.
/// - **The row's layout and package area**, as the row records them.
///
/// `Server Setup` R1b's comparison at the list ask needs the window and is
/// not played; it compares a driver and touches nothing the check reads.
async fn the_check_is_run(connection: &TheConnection<'_>, file: &str) -> (TheCheckCameTo, TheSittingIsOf) {
    let row = connection.the_row().clone();
    let server = connection.server_text();
    let installation = Identity::read_back(
        IdentityKind::Installation,
        row.last_confirmed_installation_identity.as_deref().expect("the row's installation"),
    )
    .expect("the installation reads");
    let layout = smoozy_desktop::servers::installation::the_layout_recorded_for(&row);
    let smoozy_desktop::servers::installation::WhereThePackageAreaIs::At(package_area) =
        smoozy_desktop::servers::installation::where_the_package_area_is(&row)
    else {
        panic!("the row records its package area");
    };
    let own_area = layout.as_ref().map(smoozy_server_setup::blender::the_blender_area);
    let requests = connection
        .desk
        .record()
        .read(|c| smoozy_desktop::record::upload_requests::UploadRequests::over(c).every_request_for(&connection.server));
    let holding = match requests
        .ok()
        .map(|requests| where_it_came_from(&requests, &connection.server, file.as_bytes(), Some(&installation)))
    {
        Some(WhereItCameFrom::TheSourcePath(source)) => {
            HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(&source))
        }
        _ => HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
            machine: connection.machine_key(),
            location: MachineLocation::composed(file.as_bytes().to_vec()),
        },
    };
    let with = {
        let (sessions, server, file) = (connection.point.sessions.clone(), server.clone(), file.to_string());
        tokio::task::spawn_blocking(move || {
            let handle = tokio::runtime::Handle::current();
            the_blender_the_check_reads_with(
                the_run_holds(),
                &server,
                &file,
                || handle.block_on(the_blenders_on_the_machine(&sessions, &server, own_area.as_deref())),
                || {
                    handle.block_on(smoozy_desktop::surface::files::the_saved_with_version_of(
                        &sessions,
                        &server,
                        file.as_bytes(),
                        true,
                    ))
                },
            )
        })
        .await
        .expect("the Blender is resolved")
        .unwrap_or_else(|why| panic!("`Dependency Upload & Remap` G2: a Blender to read with: {why:?}"))
    };
    let sitting = TheSittingIsOf {
        server: server.clone(),
        file: file.to_string(),
        root: MachineLocation::composed(file.as_bytes().to_vec()),
        holding,
        with,
        machine: connection.machine_key(),
        layout,
    };
    let came_to = the_check_runs(
        &mut ThisServersMachine {
            sessions: &connection.point.sessions,
            package_area: &package_area,
            server: &server,
        },
        connection.desk.record().as_ref(),
        &TheUsersOwnComputer,
        smoozy_desktop::surface::the_dependency_check::the_run_holds(),
        &sitting,
        &|| {},
    )
    .await;
    (came_to, sitting)
}

/// **Every row of the sitting**, as (state, what the file records, where it
/// was found) — read off the check's run-held memory, as the desk's own check
/// test reads it.
fn the_checks_rows(of: &TheSittingIsOf) -> Vec<(RowState, String, Option<String>)> {
    let held = smoozy_desktop::surface::the_dependency_check::the_run_holds().lock().expect("the check's memory");
    held.sittings[&the_checks_key(&of.server, &of.file)]
        .rows
        .iter()
        .map(|row| {
            (
                row.state,
                String::from_utf8_lossy(row.row.ordering_key().as_bytes()).to_string(),
                row.the_answer()
                    .and_then(|answer| answer.resolution.as_ref())
                    .map(|resolution| String::from_utf8_lossy(resolution.to.location().as_bytes()).to_string()),
            )
        })
        .collect()
}

/// **The check's panel, as the page draws it**: its summary, its statements,
/// its rows and its foot, as the words the page writes.
fn the_checks_panel(of: &TheSittingIsOf, name: &str) -> serde_json::Value {
    let held = smoozy_desktop::surface::the_dependency_check::the_run_holds().lock().expect("the check's memory");
    let panel = the_panel(&held, &of.server, &of.file, Some(name));
    serde_json::json!({
        "summary": panel.the_summary,
        "statements": panel.the_statements,
        "table": panel.the_table,
        "foot": panel.the_foot,
        "ending": panel.the_ending,
        "nothing_was_read": panel.nothing_was_read,
    })
}

/// **Every piece of prose and every value a drawn value holds**, in order,
/// joined, a no-break space read as a space — so a panel's words can be
/// read as the user reads them.
fn the_words_in(value: &serde_json::Value) -> String {
    fn walk(value: &serde_json::Value, into: &mut Vec<String>) {
        match value {
            serde_json::Value::String(text) => into.push(text.replace('\u{a0}', " ")),
            serde_json::Value::Array(items) => items.iter().for_each(|item| walk(item, into)),
            serde_json::Value::Object(fields) => fields
                .iter()
                .filter(|(name, _)| !matches!(name.as_str(), "colour" | "which" | "act" | "is" | "at" | "adds_a_row_with"))
                .for_each(|(_, item)| walk(item, into)),
            _ => {}
        }
    }
    let mut said = Vec::new();
    walk(value, &mut said);
    said.join(" ")
}

/// **The check's standing for the file on this machine**, as the record
/// keeps it (`Dependency Upload & Remap` V10).
fn the_standing(desk: &TheDesk, of: &TheSittingIsOf) -> smoozy_dependencies::standing::CheckStanding {
    desk.record().read(|c| {
        TheDependencyRecord::over(c)
            .the_standing(&of.root, &of.machine)
            .expect("the standing reads")
            .expect("a standing was written")
    })
}

/// ***Upload* pressed on the check** — `a_send_is_asked_for`, mirrored without
/// the window: the rows the upload sends (`the_rows_the_upload_sends`), sent
/// by the desktop's own `the_found_files_go_up`; and **the check run again by
/// itself where the desktop's own rule says** (`the_check_runs_again_after`,
/// `Specs/Dependency Upload & Remap`, *Checking a file*'s one exception).
async fn upload_is_pressed(connection: &TheConnection<'_>, of: &TheSittingIsOf) -> (WhatTheSendCameTo, Option<TheCheckCameTo>) {
    let rows = {
        let held = smoozy_desktop::surface::the_dependency_check::the_run_holds().lock().expect("the check's memory");
        the_rows_the_upload_sends(&held.sittings[&the_checks_key(&of.server, &of.file)].rows)
    };
    let asked = WhatIsSent {
        of: of.clone(),
        run: connection.desk.record().the_start().lock().expect("the start").run.clone().expect("the run"),
        rows,
        replacing: false,
    };
    let door: Arc<dyn TheRecordAnUploadWrites> = Arc::clone(connection.desk.record()) as Arc<dyn TheRecordAnUploadWrites>;
    let came_to = the_found_files_go_up(
        &asked,
        &connection.point.sessions,
        &TheRunsTransfers::for_this_run(),
        door,
        smoozy_desktop::surface::the_dependency_check::the_run_holds(),
    )
    .await;
    let mut again = false;
    the_check_runs_again_after(&came_to, || again = true);
    let ran_again = if again {
        Some(the_check_is_run(connection, &of.file).await.0)
    } else {
        None
    };
    (came_to, ran_again)
}

// ════════════════════════════════════════════════════════════════════════════
// A job, from the queue to the user's folder
// ════════════════════════════════════════════════════════════════════════════

/// **Passes until History holds the job finished and its record holds the
/// job's folder**. Copied from SC5.
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
/// desktop's private `OverTheReadingSession`, mirrored. Copied from SC5.
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

/// **`Queue & History` E9: the words taken in**, as the pass beside a read
/// forward that learned an ending takes them. Copied from SC5.
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

/// **The pictures under a folder**, in their names' order.
fn the_pictures_in(landed: &[PathBuf]) -> Vec<PathBuf> {
    let mut pictures: Vec<PathBuf> = landed.iter().filter(|path| path.extension().is_some_and(|ext| ext == "png")).cloned().collect();
    pictures.sort();
    pictures
}

/// **Every file under a folder**, by its whole path, in order.
fn the_files_under(folder: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut folders = vec![folder.to_path_buf()];
    while let Some(one) = folders.pop() {
        for entry in std::fs::read_dir(&one).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                folders.push(path);
            } else {
                found.push(path);
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
    println!("SC7 {step}: {}", what.as_ref());
}

// ════════════════════════════════════════════════════════════════════════════
// The scenario
// ════════════════════════════════════════════════════════════════════════════

/// **The user renders the same project on a new machine**, its situation and
/// its nine steps.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "run on purpose at the desk with a real Blender: SMOOZY_DESK_BLENDER names its program"]
async fn the_user_renders_the_same_project_on_a_new_machine() {
    let blender = PathBuf::from(
        std::env::var_os("SMOOZY_DESK_BLENDER")
            .expect("SMOOZY_DESK_BLENDER names the Blender this test runs, e.g. /Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
    );
    assert!(blender.is_file(), "{} is a Blender", blender.display());
    let began = Instant::now();
    let desk = TheDesk::opened("sc7").expect("the desk's record opens");
    // **One run of Smoozy holds both machines' work**, as one window does.
    let holds = Arc::new(WhatThisRunHolds::default());
    let project = desk.the_users_folder().join("shot_040");
    let the_users_blend = project.join("shot_040.blend");
    let the_users_texture = project.join("textures").join("crate.png");

    // ════════════════════════════════════════════════════════════════════
    // The situation: a week ago
    // ════════════════════════════════════════════════════════════════════
    // **"A week ago the user rendered a 200-frame animation from
    // shot_040.blend on a machine called RunPod 4090. It finished, they
    // downloaded the frames, they destroyed the machine at the provider, and
    // they removed the server from Smoozy."**
    let (last_weeks_job, _last_weeks_server) = {
        let Some(machine) = TheDeskMachine::stood_up("sc7-last-week", &desk, LAST_WEEKS_MACHINE).await else {
            return;
        };
        machine.a_real_blender_is_installed(&blender);
        let the_connection = connected(&machine, &desk, &holds).await;
        let made = the_users_blender(
            &blender,
            &desk,
            &[
                "last-week".to_string(),
                the_users_blend.display().to_string(),
                THE_FRAMES.to_string(),
                THE_WIDTH.to_string(),
                THE_HEIGHT.to_string(),
                THE_SAMPLES.to_string(),
            ],
        );
        assert_eq!(made["images"]["crate.png"], "//textures/crate.png", "the file records its texture beside itself: {made}");
        // **The project went up through Files**, the `.blend` and its texture.
        let the_asset_area = machine.layout().area(Area::Asset);
        let file = format!("{the_asset_area}/shot_040/shot_040.blend");
        let up = an_upload(&the_connection, vec![project.clone()], &the_asset_area, CollisionAnswer::Stop).await;
        assert!(up.the_box.is_none(), "nothing stood there, so nothing was asked");
        assert_eq!(up.came_to.uploaded, the_files_under(&project).len(), "the whole project went up: {:?}", up.came_to);
        // **The page**: opened, the file read, the name, the range and
        // auto-download off typed, and *Render*.
        the_saved_with_tag_lands(&the_connection, &file).await;
        the_version_list_is_asked_for(&the_connection).await;
        let read = the_file_is_read(
            ThisServersMachine {
                sessions: &the_connection.point.sessions,
                package_area: &machine.package_area(),
                server: &the_connection.server_text(),
            },
            the_run_holds(),
            &the_connection.server_text(),
            &file,
        )
        .await;
        assert!(
            matches!(&read, WhatTheReadCameTo::ThePreLoad { .. }),
            "`Render Settings` Q6: last week's file was read: {read:?}\n{}",
            machine.the_supervisors_complaints()
        );
        typed(&the_connection, &file, WhichSetting::JobName, THE_JOBS_NAME);
        typed(&the_connection, &file, WhichSetting::DownloadAsTheyRender, "off");
        typed(&the_connection, &file, WhichSetting::FrameRange, "1-200");
        let (_, job, _) = render_is_pressed(&the_connection, &file).await;
        until_it_has_finished(&the_connection, &job, Duration::from_secs(400)).await;
        // **"They downloaded the frames"**: *Download all to…*, then opened.
        let here = desk.the_users_folder().join("shot_040 frames, last week");
        let downloaded = a_whole_job_is_downloaded_to(&desk, &the_connection.point.sessions, &job, &here)
            .await
            .expect("`Download` M1: last week's whole job is brought down");
        let frames = the_pictures_in(&downloaded.landed);
        assert_eq!(frames.len() as i64, THE_FRAMES, "all 200 frames came down: {downloaded:?}");
        let watched = the_frames_opened(&blender, &desk, "last week's frames", &frames);
        assert_eq!(watched["sizes"], serde_json::json!([[THE_WIDTH, THE_HEIGHT]]), "{watched}");
        assert_eq!(
            watched["magenta_at_most"].as_f64(),
            Some(0.0),
            "`Specs/What a job writes`: a picture the render could not find comes out flat magenta, and last week's \
             had its texture: {watched}"
        );
        // **"They destroyed the machine at the provider"** — the harness's
        // destroy, the connection reset and every new one refused.
        let server_text = the_connection.server_text();
        drop(the_connection);
        holds.the_connection_ended(&server_text);
        machine.is_destroyed(HowTheMachineGoes::WithARefusal).await;
        // **"And they removed the server from Smoozy"**: the removal box,
        // *Remove*, and the period ended at the forget (`Servers` S10 item 6).
        let the_box = {
            let started = desk.record().the_start().lock().expect("the start");
            the_removal_box_for(&started, the_facts_behind(ObservedValue::Disconnected, false, false), Some(&*holds), &server_text)
                .expect("`Servers` W19: the removal box is composed")
        };
        assert_eq!(the_box.offers.as_str(), "a-plain-confirmation", "nothing queued or running: {the_box:?}");
        let answer = {
            let started = desk.record().the_start().lock().expect("the start");
            the_removal_is_performed_and_its_drawings_go(&started, None, None, None, &server_text, false)
        };
        assert_eq!(answer, TheAnswer::ItWasSaved, "the removal landed");
        holds.the_period_ended(&server_text);
        said(
            "SITUATION, A WEEK AGO (played)",
            format!(
                "{LAST_WEEKS_MACHINE} connected, step 7 run; the user's Blender {:?} made shot_040.blend with \
                 textures/crate.png recorded as //textures/crate.png; the project folder went up through Files \
                 ({} files); the page read the file, *Render* sent {THE_JOBS_NAME} over 1-200, it finished, and \
                 *Download all to…* brought {} frames down, which open at {THE_WIDTH}×{THE_HEIGHT} with no flat \
                 magenta; the machine was destroyed, and the removal box offered {} — Remove pressed and the \
                 removal landed ({:?} in)",
                made["version"],
                up.came_to.uploaded,
                frames.len(),
                the_box.offers.as_str(),
                began.elapsed(),
            ),
        );
        (job, server_text)
    };
    let last_weeks_text = last_weeks_job.as_text();

    // ════════════════════════════════════════════════════════════════════
    // The situation: this morning
    // ════════════════════════════════════════════════════════════════════
    // **"The user has rented a new machine, given the provider the same key
    // as before, and it is now saved in Smoozy as RunPod 4090 (2) and
    // connected. Blender is installed on it."** The desk's machine is added
    // with the desk's one key, as the first was.
    let Some(machine) = TheDeskMachine::stood_up("sc7-new", &desk, THE_NEW_MACHINE).await else {
        return;
    };
    let the_build = machine.a_real_blender_is_installed(&blender);
    // **"Nothing has been uploaded to the new machine."** The harness stands
    // every machine up with a stand-in `.blend` in its asset area, which no
    // upload of the user's put there; it is taken away before the scenario
    // starts, so the asset area holds what a fresh machine's does.
    std::fs::remove_file(machine.the_blend()).expect("the harness's stand-in .blend is taken away");
    let the_connection = connected(&machine, &desk, &holds).await;
    let new_server = the_connection.server_text();
    let the_asset_area = machine.layout().area(Area::Asset);
    {
        let held = desk.record().held();
        let screen = what_is_held(&holds, Some(&*held), None);
        assert!(screen.lines.iter().all(|line| line.rows.is_empty()), "`Queue & History` C11: nothing is queued or running");
        let rows = server_rows::all(held.connection()).expect("the rows read");
        assert_eq!(
            rows.iter().map(|row| row.label.clone().unwrap_or_default()).collect::<Vec<_>>(),
            vec![THE_NEW_MACHINE.to_string()],
            "one server in Smoozy, the new one"
        );
    }
    let empty = the_rows(&files_on(&the_connection, &the_asset_area).await);
    assert!(empty.is_empty(), "nothing has been uploaded to the new machine: {empty:?}");
    // **"The .blend and all its assets are still on the user's own computer,
    // exactly where they were a week ago."**
    assert!(the_users_blend.is_file() && the_users_texture.is_file(), "the project is where it was");
    said(
        "SITUATION, THIS MORNING (played)",
        format!(
            "{THE_NEW_MACHINE} added and connected, step 7 run, its Blender {} found by the machine's search; it is \
             the only server; the queue is empty; Files lists nothing in {the_asset_area}; the project is on the \
             user's computer where it was",
            the_build.display(),
        ),
    );

    // ── Step 1: the user opens the history and finds last week's job ─────
    {
        let held = desk.record().held();
        let history = what_is_kept(Some(&*held), Some(&last_weeks_text));
        let row = history.rows.iter().find(|row| row.job == last_weeks_text).expect("`Queue & History`: last week's job is in History");
        assert_eq!(row.ending, Some(AnEnding::Finished), "it is marked finished: {row:?}");
        assert_eq!(row.server, LAST_WEEKS_MACHINE, "`Queue & History` N12: the entry keeps the server's name as it stood");
        assert_eq!(row.name, THE_JOBS_NAME);
        let entry = history.entry.as_ref().expect("its entry");
        assert!(
            !entry.entry.reach_backs,
            "`Specs/Queue & History`: an entry outliving its server keeps its setup but loses its reach"
        );
        let again = entry.again.as_ref().expect("`Rendering` A1: *Submit again* is offered on the entry");
        assert_eq!(again.submit_again as i64, THE_FRAMES, "*Submit again* names the job's 200 frames");
        assert!(again.carry_on.is_none(), "a clean finish shows only *Submit again*");
        said(
            "STEP 1 (played)",
            format!(
                "History holds {} job: {:?}, {:?}, frames {:?}, on {:?}, ended {:?}; its entry offers the machine's \
                 controls: {}; its ways to render again: *Submit again — {} frames*, no carry-on",
                history.jobs, row.name, row.ending, row.frames, row.server, row.when, entry.entry.reach_backs, again.submit_again,
            ),
        );
    }

    // ── Step 2: the user asks to submit that job again ───────────────────
    let press = submit_again_is_pressed(&desk, &last_weeks_job, &[&the_connection]);
    assert_eq!(
        press.leads,
        WhereThePressLeads::TheBrowserOpens { server: new_server.clone() },
        "`Rendering` A1a: its own machine is gone and exactly one is connected, so the browser opens on that one"
    );
    assert_eq!(press.the_bar, Some(TheBar::Find), "`Rendering` A1a: the bar of a re-submission whose machine is gone");
    assert!(!press.came_from.the_simplify_offer_is_drawn, "`Rendering` A1b: a finished entry draws no Simplify offer");
    said(
        "STEP 2 (played)",
        format!(
            "*Submit again* on {THE_JOBS_NAME}: the press leads to {:?} with the bar {:?}; the setup is held for the \
             pick ({})",
            press.leads,
            press.the_bar,
            match &press.the_setup {
                smoozy_desktop::a_new_job_from_an_old_one::TheSetupHeld::AScene(_) => "a scene's setup",
                smoozy_desktop::a_new_job_from_an_old_one::TheSetupHeld::NoScene { .. } => "a setup with no scene",
            },
        ),
    );

    // ── Step 3: the user looks at what Smoozy shows them and asks for ────
    // **The bar the Files tab carries while the pick is in hand** — what
    // `the_bar_in_hand` composes for the held pick, which the test holds
    // itself (see the head of this file).
    let ABarAcrossTheTop::Find { said: the_bar_says, way_out } = the_find_bar(&press.came_from.the_entrys_label) else {
        panic!("the find bar");
    };
    let the_bar_says = as_written(&the_bar_says);
    assert!(the_bar_says.contains(THE_JOBS_NAME), "the bar names the job: {the_bar_says}");
    assert!(
        the_bar_says.contains("find its .blend on this server"),
        "`Specs/Rendering`: it says the file has to be found on this machine: {the_bar_says}"
    );
    assert!(
        the_bar_says.contains("does not exist here"),
        "`Specs/Rendering`: and that where it sat on the old one carries over nowhere: {the_bar_says}"
    );
    let first_look = files_on(&the_connection, &the_asset_area).await;
    said(
        "STEP 3 (played)",
        format!(
            "Files opens on {THE_NEW_MACHINE} with the bar {the_bar_says:?} and its way out {:?} (`Rendering` A1a, \
             `File Browser` §12, `Design/File Browser § The browser opened so the user can point at a file`); \
             {the_asset_area} lists {:?}",
            way_out.as_str(),
            the_rows(&first_look),
        ),
    );

    // ── Step 4: the user sends shot_040.blend up from their own computer ──
    let file = format!("{the_asset_area}/shot_040.blend");
    let up = an_upload(&the_connection, vec![the_users_blend.clone()], &the_asset_area, CollisionAnswer::Stop).await;
    assert!(up.the_box.is_none(), "nothing stood there, so nothing was asked");
    assert_eq!(up.came_to.uploaded, 1, "`File Browser` U11: the one file uploaded: {:?}", up.came_to);
    assert!(up.came_to.failed.is_empty() && up.came_to.skipped == 0 && up.came_to.did_not_finish == 0, "{:?}", up.came_to);
    let after = the_rows(&files_on(&the_connection, &the_asset_area).await);
    assert_eq!(after.iter().map(|(name, _)| name.as_str()).collect::<Vec<_>>(), vec!["shot_040.blend"], "{after:?}");
    assert_eq!(std::fs::read(&file).expect("the machine's copy"), std::fs::read(&the_users_blend).expect("the user's"));
    said(
        "STEP 4 (played)",
        format!(
            "shot_040.blend sent up into {the_asset_area} through Files: {} uploaded, nothing asked; Files lists \
             {after:?}; the machine's copy is the user's byte for byte (`File Browser` X1, `File Browser` U11)",
            up.came_to.uploaded,
        ),
    );

    // ── Step 5: the user runs a check on that file ────────────────────────
    // ***Check dependencies…* on the `.blend` in Files** — the check is
    // reached from Render Settings and from a `.blend` in Files
    // (`Design/Dependency Upload & Remap § Where it sits`); the user is in
    // Files.
    let (came_to, of) = the_check_is_run(&the_connection, &file).await;
    assert_eq!(
        came_to,
        TheCheckCameTo::TheSittingStands(Completion::Completed),
        "`Dependency Upload & Remap` V10: the check completes: {}",
        machine.the_supervisors_complaints()
    );
    assert!(
        matches!(&of.holding, HoldingFile::OnTheUsersComputer(source) if source.as_bytes() == the_users_blend.as_os_str().as_encoded_bytes()),
        "`File Browser` N6: the file came up from the user's computer, so its relative paths are measured from there: {:?}",
        of.holding
    );
    said(
        "STEP 5 (played)",
        format!(
            "*Check dependencies…* on shot_040.blend: read on the machine with {} {:?} \
             (`Dependency Upload & Remap` G2), its holding file the user's own shot_040.blend (`File Browser` N6); \
             the sitting came to {came_to:?}",
            of.with.path,
            of.with.version,
        ),
    );

    // ── Step 6: the user looks at what the check found ───────────────────
    let rows = the_checks_rows(&of);
    let texture_here = the_users_texture.display().to_string();
    assert_eq!(
        rows,
        vec![(RowState::FoundOnYourComputer, "//textures/crate.png".to_string(), Some(texture_here.clone()))],
        "`Specs/Dependency Upload & Remap`: one row, the texture, found on the user's computer where the file says"
    );
    let panel = the_checks_panel(&of, THE_NEW_MACHINE);
    let the_panels_words = the_words_in(&panel);
    assert!(
        the_panels_words.contains("Nothing is missing. 1 file to upload"),
        "`Design/Dependency Upload & Remap § Above the rows`: the summary says what will upload: {the_panels_words}"
    );
    let standing = the_standing(&desk, &of);
    assert_eq!(standing.completion, Completion::Completed, "{standing:?}");
    let band_now = the_check_the_band_reads_now(&the_connection, &file);
    said(
        "STEP 6 (played)",
        format!(
            "the check's rows: {rows:?}; the panel reads {the_panels_words:?}; the standing is {:?} with every row \
             confirmed or set aside: {}; the band would now read {band_now:?}",
            standing.completion, standing.every_row_confirmed_or_set_aside,
        ),
    );

    // ── Step 7: the user sends up what has been found ────────────────────
    let (sent, ran_again) = upload_is_pressed(&the_connection, &of).await;
    assert_eq!((sent.published, sent.failed, sent.did_not_finish), (1, 0, 0), "`Dependency Upload & Remap`: the texture went up: {sent:?}");
    let mirror = smoozy_dependencies::landing::mirror_of(
        machine.layout(),
        &StoredLocation::as_the_system_returned_it(the_users_texture.as_os_str().as_encoded_bytes()),
    )
    .expect("`Dependency Upload & Remap` A4: the mirror fits");
    let mirror = PathBuf::from(String::from_utf8(mirror.as_bytes().to_vec()).expect("the mirror is text"));
    assert_eq!(
        std::fs::read(&mirror).expect("the texture sits at its mirror on the machine (read off the machine's disk at the desk)"),
        std::fs::read(&the_users_texture).expect("the user's texture"),
        "`Specs/Dependency Upload & Remap`: an asset keeps the location it had on the user's computer"
    );
    assert!(
        !Path::new(&the_asset_area).join("textures").exists(),
        "nothing sits beside the .blend on the machine, so only the remap can point the render at the texture"
    );
    assert_eq!(
        ran_again,
        Some(TheCheckCameTo::TheSittingStands(Completion::Completed)),
        "`Specs/Dependency Upload & Remap`, *Checking a file*: an upload the check started that ends with files arrived \
         runs the check again by itself"
    );
    let rows_after = the_checks_rows(&of);
    assert_eq!(
        rows_after.iter().map(|row| row.0).collect::<Vec<_>>(),
        vec![RowState::AlreadyOnTheMachine],
        "the row now reads already on the machine: {rows_after:?}"
    );
    let standing = the_standing(&desk, &of);
    assert!(
        standing.completion == Completion::Completed && standing.every_row_confirmed_or_set_aside,
        "`Specs/Dependency Upload & Remap`, *When the warning stops*: a check that leaves nothing unfound: {standing:?}"
    );
    let panel_after = the_words_in(&the_checks_panel(&of, THE_NEW_MACHINE));
    said(
        "STEP 7 (played)",
        format!(
            "*Upload*: {} published, {} failed; the texture sits at its mirror {} byte for byte (read off the \
             machine's disk at the desk), and nothing sits beside the .blend; the check ran again by itself and came \
             to {ran_again:?}; its rows {rows_after:?}; the panel reads {panel_after:?}",
            sent.published,
            sent.failed,
            mirror.display(),
        ),
    );

    // ── Step 8: the user changes the resolution, and submits the job ─────
    // **The pick lands**: the user points at shot_040.blend in the browser
    // the bar is across, which is what the held setup waits for. What
    // `the_file_was_pointed_at` does: the saved-with tag lands, then the
    // pick — `the_pick_lands`, which takes the held pick from its private
    // run-held home and makes this one write — and the page opens and asks
    // the machine for its Blenders.
    let saved_with = the_saved_with_tag_lands(&the_connection, &file).await;
    the_setup_is_loaded(
        &mut the_run_holds().lock().expect("the page's memory"),
        &new_server,
        &file,
        press.the_setup.clone(),
        press.the_range.as_ref(),
        press.came_from.clone(),
    );
    the_version_list_is_asked_for(&the_connection).await;
    let page = the_page(&the_connection, &file);
    let came_from = the_sentence(&page.where_it_came_from);
    said(
        "STEP 8, THE PAGE OPENS (played)",
        format!(
            "the page opens on {file} over {THE_NEW_MACHINE}, saved with {saved_with:?}, versions {:?}; it says where \
             its values came from: {came_from:?} (waiting: {}); the range reads {:?} — {}; samples {:?}; resolution \
             {:?}; the job's name {:?}; auto-download {}; the band line {:?}; the dependency band {:?}; submitting is \
             offered: {}",
            page.the_versions.iter().map(|one| format!("{} {}", one.answered.as_str(), one.build.as_str())).collect::<Vec<_>>(),
            page.where_it_came_from_is_waiting,
            page.the_range.value.as_ref().map(|value| value.as_str()),
            the_sentence(&page.the_frame_count),
            page.samples.value.as_ref().map(|value| value.as_str()),
            page.resolution.value.as_ref().map(|value| value.as_str()),
            page.the_job_name.as_str(),
            if page.the_download_is_on { "on" } else { "off" },
            the_sentence(&page.the_band),
            page.the_dependency_band.as_ref().map(|band| band.which.as_str()),
            page.submitting_is_offered,
        ),
    );
    // **The page says where its values came from, and says it truly.**
    // `Design/Render Settings § What this document left open`: the band says
    // the values came out of that job on that date; *"the band has a second
    // shape for no server connected"*. This page is over RunPod 4090 (2),
    // which is connected — Rule 8: Smoozy telling the truth about what is so.
    // The displayed sentence uses the application's nonbreaking text spacing.
    let submitting_again = smoozy_desktop::typography::Prose::new("Submitting again.");
    assert!(
        !page.where_it_came_from_is_waiting && came_from.starts_with(submitting_again.as_str()),
        "`Design/Render Settings § What this document left open`, `Rendering` A1a: the page over a connected server \
         says the values came from {THE_JOBS_NAME}, not that no server is connected: {came_from:?}"
    );
    assert!(came_from.contains(THE_JOBS_NAME), "the band names the job: {came_from:?}");
    assert_eq!(
        page.the_range.value.as_ref().map(|value| value.as_str().to_string()).as_deref(),
        Some("1-200"),
        "`Render Settings` Y15: the kept setup's frame range is loaded"
    );
    assert!(
        page.the_dependency_band.is_none(),
        "`Rendering` W2, `Specs/Dependency Upload & Remap`, *When the warning stops*: the complete check on this file on \
         this machine stops the warning"
    );
    // **"Auto-download is off"** — the scenario's own situation, and
    // `Specs/Download`, *Auto-download*: *"It is off unless the user turns it
    // on"*; `Specs/Rendering`: the switch is not carried over from the entry
    // and is offered fresh.
    assert!(
        !page.the_download_is_on,
        "`Specs/Download`, *Auto-download*: off unless the user turns it on, and `Specs/Rendering`: offered fresh"
    );
    typed(&the_connection, &file, WhichSetting::ResolutionPercentage, &THE_HIGHER_RESOLUTION.to_string());
    let page = the_page(&the_connection, &file);
    assert_eq!(
        page.resolution.value.as_ref().map(|value| value.as_str().to_string()),
        Some(THE_HIGHER_RESOLUTION.to_string()),
        "the resolution field reads what the user typed"
    );
    let (_, new_job, the_answer) = render_is_pressed(&the_connection, &file).await;
    assert_ne!(new_job, last_weeks_job, "`Rendering` A1: what comes back is a new job");
    said(
        "STEP 8, SUBMITTED (played)",
        format!(
            "the user types {THE_HIGHER_RESOLUTION} % and the resolution reads {:?}, size {:?}; *{}* is pressed and \
             the act says {the_answer:?} (`Rendering` S10)",
            page.resolution.value.as_ref().map(|value| value.as_str()),
            page.the_size_reading.as_ref().map(|size| size.as_str()),
            the_sentence(&page.the_render_control),
        ),
    );

    // ── Step 9: the user looks at the queue and at the history ───────────
    let new_text = new_job.as_text();
    the_connection
        .passes_until("the queue reads the new job rendering with a frame saved", Duration::from_secs(120), |it| {
            it.the_job_as_last_read(&new_job).is_some_and(|read| {
                matches!(read.state, QueueState::Rendering { .. })
                    && read.live.the_figures().is_some_and(|figures| figures.newest_frame_saved.is_some())
            })
        })
        .await;
    {
        let held = desk.record().held();
        let screen = what_is_held(&holds, Some(&*held), Some(&new_text));
        assert_eq!(screen.jobs, 1, "one job in the queue");
        let entry = screen.entry.as_ref().expect("the new job's entry");
        let marks: Vec<String> = entry.marks.iter().map(|mark| format!("{}{}", mark.said, parts(&mark.parts))).collect();
        assert!(
            !marks.iter().any(|mark| mark.starts_with("Dependencies were not checked")),
            "`Queue & History` N15: the file was checked on this machine, so no unchecked mark: {marks:?}"
        );
        let history = what_is_kept(Some(&*held), Some(&last_weeks_text));
        let last_week = history.rows.iter().find(|row| row.job == last_weeks_text).expect("last week's entry is still there");
        let again = history.entry.as_ref().and_then(|entry| entry.again.as_ref());
        assert!(again.is_some(), "`Specs/Rendering`: an entry is not spent by being used once");
        said(
            "STEP 9, THE QUEUE (played)",
            format!(
                "the queue holds {} job, {:?} on {THE_NEW_MACHINE}: status {:?}, progress {:?}; its marks {marks:?}; \
                 History holds {} job(s), last week's {:?} {:?} on {:?}, still offering *Submit again — {} frames*",
                screen.jobs,
                entry.name,
                parts(&entry.status),
                entry.progress.as_ref().map(|progress| (parts(&progress.now), progress.now_at, progress.whole)),
                history.jobs,
                last_week.name,
                last_week.ending,
                last_week.server,
                again.map(|again| again.submit_again).unwrap_or_default(),
            ),
        );
    }
    until_it_has_finished(&the_connection, &new_job, Duration::from_secs(500)).await;
    the_words_are_taken_in(&the_connection).await;
    let the_new_frames = {
        let held = desk.record().held();
        let history = what_is_kept(Some(&*held), Some(&new_text));
        let row = history.rows.iter().find(|row| row.job == new_text).expect("the new job is in History");
        assert_eq!(row.ending, Some(AnEnding::Finished), "History says it finished: {row:?}");
        assert_eq!(row.server, THE_NEW_MACHINE);
        assert!(
            history.rows.iter().any(|row| row.job == last_weeks_text),
            "last week's entry stands beside it: {:?}",
            history.rows
        );
        let record = JobRecords::over(held.connection()).read(&new_job).expect("it reads").expect("it stands");
        assert_eq!(record.the_ending_learned(), Some(AJobsEnding::Finished));
        let entry = history.entry.as_ref().expect("its entry");
        let marks: Vec<String> = entry.entry.marks.iter().map(|mark| format!("{}{}", mark.said, parts(&mark.parts))).collect();
        let kept = smoozy_desktop::record::kept_words::the_words_kept_for(held.connection(), &new_job).expect("the words read");
        let missing: Vec<String> = kept
            .iter()
            .flat_map(|one| String::from_utf8_lossy(&one.the_words).lines().map(str::to_string).collect::<Vec<_>>())
            .filter(|line| line.contains("crate.png") && line.contains("does not exist"))
            .collect();
        assert!(missing.is_empty(), "Blender found the texture through the remap: {missing:?}");
        let folder = record.the_job_output_folder(machine.layout()).expect("the job's folder");
        said(
            "STEP 9, HISTORY (played)",
            format!(
                "the new job finished; History holds {} jobs: {:?}; the new entry's marks {marks:?}; *Blender output* \
                 offered: {}, and no line of it says crate.png does not exist; its folder on the machine is {folder}",
                history.jobs,
                history.rows.iter().map(|row| (row.name.clone(), row.server.clone(), row.ending)).collect::<Vec<_>>(),
                entry.blender_output,
            ),
        );
        the_pictures_in(&the_files_under(Path::new(&folder)))
    };
    // **What the machine holds** — the new job's frames, read off the
    // machine's disk at the desk (the scenario's user does not download them;
    // auto-download is off) and opened in Blender on the user's computer.
    assert_eq!(the_new_frames.len() as i64, THE_FRAMES, "200 frames on the machine: {the_new_frames:?}");
    let opened = the_frames_opened(&blender, &desk, "the new frames", &the_new_frames);
    let (wide, high) = (THE_WIDTH * THE_HIGHER_RESOLUTION / 100, THE_HEIGHT * THE_HIGHER_RESOLUTION / 100);
    assert_eq!(opened["sizes"], serde_json::json!([[wide, high]]), "the new frames are at the higher resolution: {opened}");
    assert_eq!(
        opened["magenta_at_most"].as_f64(),
        Some(0.0),
        "`Specs/What a job writes`: no flat magenta, so the render found the texture the check sent: {opened}"
    );
    said(
        "STEP 9, WHAT THE MACHINE HOLDS (read at the desk)",
        format!(
            "the machine holds {} frames, which open at {wide}×{high} with flat magenta at most {:?}, so the texture \
             sent up by the check was found through the remap ({:?} in all)",
            the_new_frames.len(),
            opened["magenta_at_most"],
            began.elapsed(),
        ),
    );
}

/// **What the dependency band would read on the page now** — the page's own
/// reading of the check's standing for this file on this machine
/// (`Rendering` W2).
fn the_check_the_band_reads_now(connection: &TheConnection<'_>, file: &str) -> String {
    format!("{:?}", what_the_page_reads(connection, file).the_check)
}

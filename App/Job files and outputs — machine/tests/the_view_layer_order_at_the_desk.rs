//! **Owed piece 8, the second check: the read's per-frame View Layer order is
//! the order Blender lists them** — note 4's first unresolved item, the
//! preview's View Layer dropdown, which lists a frame's layers in the frame's
//! own order (`Queue & History` P1 and P4, `Job files and outputs` R4 item 5).
//! Kept in the tree and ignored. Run by hand at the desk:
//!
//! ```text
//! cargo test -p smoozy-job-files-machine --test the_view_layer_order_at_the_desk -- --ignored --nocapture
//! ```
//!
//! Each of the five Blenders on the Mac is found at its usual place, or at
//! `SMOOZY_DESK_BLENDER_45`, `_50`, `_51`, `_52` and `_53`. **This crate's own
//! code starts no process**; this file is a test of it, not part of it.
//!
//! **What it does, in every version:** one file is saved with five View
//! Layers, made in an order that is not alphabetical, one of them renamed
//! after it was made, one switched off for good and one whose switch is keyed
//! off from frame 3. Then two runs of the same Blender open the saved file:
//!
//! - **Smoozy's real reading script**, composed by this crate, which prints
//!   the per-frame set (`per_frame[].view_layers_that_render`) and the
//!   settings half's list of the scene's layers;
//! - **a plain listing of the test's own**, with no Smoozy code in it, which
//!   prints the scene's View Layers as Blender's Python lists them
//!   (`scene.view_layers`) and, at each frame, the ones switched on.
//!
//! The two must agree at every frame, in order, and the settings half must
//! list the layers as Blender does. **The test has teeth only because Blender's
//! order is not the alphabetical one**, which it also asserts, so a reading
//! that sorted the names would fail it.
//!
//! **What it does not prove**: the order Blender's own on-screen View Layer
//! menu shows. That needs a window, and every run here has none. The order
//! compared is the one Blender's Python gives for the scene's layers, which
//! is the order the reading walks.
//!
//! **Names used**, each read off all five Blenders' own property lists at the
//! desk on 2026-10-17 (`Audits/Build/Step 10c/Owed piece 8 — what landed.md`):
//! `Scene.view_layers`, `ViewLayers.new`, `ViewLayer.name`, `ViewLayer.use`.
//! `keyframe_insert` on `use` and `frame_set` are what
//! `tests/stepping_at_the_desk.rs` already uses.

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
scene = bpy.context.scene
scene.name = "Layers"
scene.frame_start = 1
scene.frame_end = 4
scene.view_layers[0].name = "Mid"
for name in ("Zebra", "Alpha", "Kitchen", "Beta"):
    scene.view_layers.new(name)
scene.view_layers["Zebra"].name = "Aardvark"
alpha = scene.view_layers["Alpha"]
alpha.use = True
alpha.keyframe_insert("use", frame=1)
alpha.use = False
alpha.keyframe_insert("use", frame=3)
scene.view_layers["Kitchen"].use = False
print("SMOOZY-DESK-MADE", [layer.name for layer in scene.view_layers], flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

/// **Blender's own list**, with no Smoozy code in it.
const BLENDERS_OWN_LIST: &str = r#"
import bpy, json
scene = bpy.data.scenes["Layers"]
listed = {"layers": [layer.name for layer in scene.view_layers], "on_at": {}}
for frame in range(scene.frame_start - 1, scene.frame_end + 2):
    scene.frame_set(frame)
    listed["on_at"][str(frame)] = [layer.name for layer in scene.view_layers if layer.use]
print("SMOOZY-DESK-LISTED " + json.dumps(listed), flush=True)
"#;

fn the_five() -> Vec<(u16, u16, PathBuf)> {
    [
        (4u16, 5u16, "SMOOZY_DESK_BLENDER_45", "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"),
        (5, 0, "SMOOZY_DESK_BLENDER_50", "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"),
        (5, 1, "SMOOZY_DESK_BLENDER_51", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
        (5, 2, "SMOOZY_DESK_BLENDER_52", "/Applications/Blender.app/Contents/MacOS/Blender"),
        (5, 3, "SMOOZY_DESK_BLENDER_53", "/Applications/Blender 5.3b.app/Contents/MacOS/Blender"),
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

fn names(of: &[u8]) -> String {
    String::from_utf8_lossy(of).to_string()
}

#[test]
#[ignore = "run by hand at the desk with the five Blenders"]
fn the_reads_per_frame_view_layer_order_is_the_order_blender_lists_them_in_all_five() {
    let at = std::env::temp_dir().join(format!("smoozy-view-layer-order-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_FILE_MADE).expect("written");
    let lister = at.join("list.py");
    std::fs::write(&lister, BLENDERS_OWN_LIST).expect("written");

    for (major, minor, blender) in the_five() {
        let named = format!("{major}.{minor}");
        let file = at.join(format!("five-layers-{major}{minor}.blend"));
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
        let said = String::from_utf8_lossy(&said).to_string();
        println!(
            "{named}: {}",
            said.lines().find(|line| line.starts_with("SMOOZY-DESK-MADE")).unwrap_or("no made line")
        );

        // **Blender's own list**, from the saved file.
        let (listed, stream) = run(
            &blender,
            &[
                "--factory-startup",
                "--background",
                "--python-exit-code",
                "63",
                file.to_str().unwrap(),
                "--python",
                lister.to_str().unwrap(),
            ],
        );
        assert_eq!(listed, Some(0), "{named}: Blender lists the layers");
        let stream = String::from_utf8_lossy(&stream).to_string();
        let listed: serde_json::Value = serde_json::from_str(
            stream
                .lines()
                .find_map(|line| line.strip_prefix("SMOOZY-DESK-LISTED "))
                .expect("the listing is printed"),
        )
        .expect("the listing reads");
        let blenders_layers: Vec<String> = listed["layers"]
            .as_array()
            .expect("the layers")
            .iter()
            .map(|one| one.as_str().unwrap().to_string())
            .collect();
        println!("{named}: Blender lists {blenders_layers:?}");
        let mut sorted = blenders_layers.clone();
        sorted.sort();
        assert_ne!(blenders_layers, sorted, "{named}: Blender's order is not the alphabetical one, so the check bites");
        assert_eq!(blenders_layers.len(), 5, "{named}: five layers");

        // **Smoozy's real reading script**, on the same file.
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
        assert!(printed.errors.is_empty(), "{named}: {:?}", printed.errors);

        // **The per-frame set, frame by frame, against Blender's own.**
        // The read also covers the frame either side of a key (0 here), so
        // Blender's own list steps one frame further each way.
        let read_frames: Vec<i64> = printed.outputs.per_frame.iter().map(|at| at.frame).collect();
        assert!((1..=4).all(|frame| read_frames.contains(&frame)), "{named}: {read_frames:?}");
        for at in &printed.outputs.per_frame {
            let smoozys: Vec<String> = at.view_layers_that_render.iter().map(|one| names(one.as_bytes())).collect();
            let blenders: Vec<String> = listed["on_at"][at.frame.to_string()]
                .as_array()
                .expect("Blender's set at the frame")
                .iter()
                .map(|one| one.as_str().unwrap().to_string())
                .collect();
            println!("{named}: frame {}: the read {smoozys:?}, Blender {blenders:?}", at.frame);
            assert_eq!(smoozys, blenders, "{named}: frame {}", at.frame);
        }
        // The keyed switch and the switched-off layer are really in play.
        let at_frame = |frame: i64| {
            printed
                .outputs
                .per_frame
                .iter()
                .find(|at| at.frame == frame)
                .map(|at| at.view_layers_that_render.len())
        };
        assert_eq!(at_frame(1), Some(4), "{named}: four layers render at frame 1");
        assert_eq!(at_frame(3), Some(3), "{named}: three render from frame 3");

        // **The settings half lists the scene's layers as Blender does.**
        let scenes = printed.settings.0["scenes"].as_array().expect("the scenes");
        let one = scenes.iter().find(|s| s["name"] == "Layers").expect("the scene is read");
        let settings_layers: Vec<String> = one["view_layers"]
            .as_array()
            .expect("its layers")
            .iter()
            .map(|layer| layer["name"].as_str().unwrap().to_string())
            .collect();
        assert_eq!(settings_layers, blenders_layers, "{named}: the settings half's list");
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

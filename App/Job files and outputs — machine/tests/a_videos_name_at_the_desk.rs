//! **A video job's predicted name over part of the file's range — proved with
//! real Blenders, kept in the tree and ignored.** The live Telegram pass's
//! fault 4 (`Audits/Build/Step 10c/The live Telegram pass's faults — what each
//! piece is, which files it owns, and in what order.md`): a 2-frame video job
//! finished with *Video file missing or empty* while `…_0001-0002.mp4` was on
//! the server. Run by hand at the desk:
//!
//! ```text
//! cargo test -p smoozy-job-files-machine --test a_videos_name_at_the_desk -- --ignored --nocapture
//! ```
//!
//! Each of the four Blenders is found at its usual place on this Mac, or at
//! `SMOOZY_DESK_BLENDER_45`, `_50`, `_51` and `_52`; `SMOOZY_DESK_ONLY=45` (or
//! `50`, `51`, `52`) runs one alone. **This crate's own code starts no
//! process**; this file is a test of it, not part of it.
//!
//! **What it proves**, in every version: a file whose regular output path
//! writes an MPEG-4 video over frames 1 to 12, read for a job of frames 1 and
//! 2, predicts `vid_0001-0002.mp4` — the name `-s 1 -e 2 -a` writes on that
//! file (`Audits/Build/Step 10c/A desk test — the name of a video over part of
//! its range.md`, six Blenders). Read for the file's own range it still
//! predicts `vid_0001-0012.mp4`; read for frames 14 to 16, past the file's
//! end, `vid_0014-0016.mp4`; and the scene's start and end the reading
//! reports are the file's own, 1 and 12, either way. **Nothing is rendered
//! here**: the name written is that desk test's measurement.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use smoozy_job_files_machine::composing::{
    the_reading_and_everything_derived_from, WhatTheReadingIsComposedAgainst,
};
use smoozy_job_files_machine::inventory::WhatTheInventoryIs;
use smoozy_job_files_machine::main_output::WhereTheRenderEnters;
use smoozy_job_files_machine::reading::{ArtistsText, RecordedLocation, TheInvocation, TheVersionInHand};
use smoozy_job_files_machine::reading_script::{the_printed_reading_in, the_reading_script};

/// **No run of Blender here is waited on past this** — a reading of a tiny file.
const ONE_RUN_AT_MOST: Duration = Duration::from_secs(300);

/// The desk test's own setup: 64 × 64, Workbench, FFmpeg video, MPEG-4, H.264,
/// frames 1 to 12, the output path `//out/vid_`.
const THE_FILE_MADE: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
scene = bpy.context.scene
scene.frame_start = 1
scene.frame_end = 12
scene.render.engine = "BLENDER_WORKBENCH"
scene.render.resolution_x = 64
scene.render.resolution_y = 64
scene.render.resolution_percentage = 100
scene.render.filepath = "//out/vid_"
if hasattr(scene.render.image_settings, "media_type"):
    scene.render.image_settings.media_type = "VIDEO"
scene.render.image_settings.file_format = "FFMPEG"
scene.render.ffmpeg.format = "MPEG4"
scene.render.ffmpeg.codec = "H264"
bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

fn the_four() -> Vec<(u16, u16, PathBuf)> {
    let only = std::env::var("SMOOZY_DESK_ONLY").ok();
    [
        (4u16, 5u16, "SMOOZY_DESK_BLENDER_45", "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"),
        (5, 0, "SMOOZY_DESK_BLENDER_50", "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"),
        (5, 1, "SMOOZY_DESK_BLENDER_51", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
        (5, 2, "SMOOZY_DESK_BLENDER_52", "/Applications/Blender.app/Contents/MacOS/Blender"),
    ]
    .into_iter()
    .filter(|(major, minor, _, _)| only.as_deref().map_or(true, |one| one == format!("{major}{minor}")))
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

/// The one name the inventory predicts for the video, without its folder, and
/// the scene's start and end as the reading reports them.
fn the_predicted_name(
    blender: &Path,
    major: u16,
    minor: u16,
    file: &Path,
    at: &Path,
    frames: Vec<i64>,
) -> (String, (i64, i64)) {
    let named = format!("{major}.{minor}");
    let script = the_reading_script(
        &TheInvocation {
            the_blender_build_path: blender.display().to_string(),
            the_blend_path_on_the_machine: RecordedLocation::of(file.to_str().unwrap().as_bytes().to_vec()),
            the_version_it_will_render_by: TheVersionInHand::of(major, minor),
            the_frames_the_job_covers: frames.clone(),
            the_check_is_written_as_a_picture: false,
            the_check_is_marked: false,
        },
        None,
    )
    .expect("every name is on record");
    let reading = at.join(format!("reading-{major}{minor}-{}.py", frames.len()));
    std::fs::write(&reading, script).expect("written");
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
    assert_eq!(status, Some(0), "{named}: the reading ends at success");
    let printed = the_printed_reading_in(&stream).expect("the document is between its marks");
    let scenes = printed.settings.0["scenes"].as_array().expect("the scenes").clone();
    let scene = scenes.iter().find(|s| s["name"] == "Scene").expect("the scene is read");
    let range = (
        scene["frame_start"].as_i64().expect("a start"),
        scene["frame_end"].as_i64().expect("an end"),
    );

    let job = at.join(format!("job-{major}{minor}"));
    let job_folder = job.to_str().unwrap().to_string();
    let enters_at = WhereTheRenderEnters {
        scene: ArtistsText::of(b"Scene".to_vec()),
        view_layer: ArtistsText::of(b"ViewLayer".to_vec()),
    };
    let composed = the_reading_and_everything_derived_from(
        printed,
        WhatTheReadingIsComposedAgainst {
            the_folder_the_blend_sits_in: file.parent().unwrap().to_str().unwrap().as_bytes(),
            job_output_folder: &job_folder,
            enters_at: &enters_at,
            the_jobs_pins: &[],
        },
    )
    .expect("a reading");
    let WhatTheInventoryIs::It(inventory) = &composed.inventory else {
        panic!("{named}: the inventory is composed: {:?}", composed.inventory);
    };
    assert!(inventory.a_destination_is_video_shaped, "{named}: the video is read as one");
    let names: Vec<String> = inventory
        .entries
        .iter()
        .map(|entry| {
            let path = String::from_utf8_lossy(&entry.predicted_path).into_owned();
            path.rsplit('/').next().unwrap_or(&path).to_string()
        })
        .collect();
    println!("{named}: frames {frames:?} predict {names:?}, the scene reported at {range:?}");
    let mut distinct = names.clone();
    distinct.dedup();
    assert_eq!(distinct.len(), 1, "{named}: one video file: {names:?}");
    (distinct.remove(0), range)
}

#[test]
#[ignore = "run by hand at the desk with the four Blenders"]
fn a_video_read_for_part_of_its_range_is_named_for_that_part_in_all_four() {
    let at = std::env::temp_dir().join(format!("smoozy-a-videos-name-at-the-desk-{}", std::process::id()));
    std::fs::create_dir_all(&at).expect("a folder");
    let maker = at.join("make.py");
    std::fs::write(&maker, THE_FILE_MADE).expect("written");

    for (major, minor, blender) in the_four() {
        let named = format!("{major}.{minor}");
        let file = at.join(format!("video-{major}{minor}.blend"));
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

        let (part, part_range) = the_predicted_name(&blender, major, minor, &file, &at, vec![1, 2]);
        assert_eq!(part, "vid_0001-0002.mp4", "{named}: a job of frames 1 and 2");
        assert_eq!(part_range, (1, 12), "{named}: the scene's own range is reported");

        // **A job starting after the file's own end**: the end is moved first,
        // so the start never passes it, and both are put back.
        let (later, later_range) = the_predicted_name(&blender, major, minor, &file, &at, vec![14, 15, 16]);
        assert_eq!(later, "vid_0014-0016.mp4", "{named}: a job of frames 14 to 16");
        assert_eq!(later_range, (1, 12), "{named}: the scene's own range is reported");

        let (own, own_range) = the_predicted_name(&blender, major, minor, &file, &at, Vec::new());
        assert_eq!(own, "vid_0001-0012.mp4", "{named}: a job of the file's own range");
        assert_eq!(own_range, (1, 12), "{named}: the scene's own range is reported");
    }
    std::fs::remove_dir_all(&at).expect("the folder is removed");
}

//! **Drivers followed without a move, and cheap moves — proved with real
//! Blenders, kept in the tree and ignored.** `Job files and outputs` R4 item
//! 5 as of 2026-11-14 (the build runner's brief, `Audits/Build/The four
//! questions/Brief — drivers and cheap moves, built from the research.md`).
//! Run by hand at the desk, one Blender at a time:
//!
//! ```text
//! cargo test -p smoozy-job-files-machine --test drivers_and_cheap_moves_at_the_desk -- --ignored --nocapture --test-threads=1
//! ```
//!
//! **What it proves**: every `.blend` under the folders named in
//! `SMOOZY_DESK_DRIVER_FILES` (separated by `:`; by default the Creator's
//! `~/Smoozy driver test files`) and the cases this file makes itself is read
//! twice in each of the five Blenders, 4.5 to 5.3 — by today's walk, which is
//! the reading script's body as it stood at [`TODAYS_COMMIT`], and by the
//! reading as built — in both modes: *Read the file* (the pre-load reading)
//! and a render's start (the file's own scene named, its first, middle and
//! last frame). **The two printed readings must be byte-identical.** Each run
//! is timed whole, from start to Blender quitting, and its moves are counted.
//! A file made in a newer Blender than the one reading it is skipped and said
//! so; `SMOOZY_DESK_NO_CASES` leaves this file's own cases out. Results go line by line to `results.jsonl` in the work folder
//! (`SMOOZY_DESK_WORK`, else a fresh temporary one).
//!
//! The reading runs as the machine runs it, with the file's own scripts
//! allowed (`--enable-autoexec`), and with `--factory-startup` and Blender's
//! settings folders pointed into the work folder, as the desk's other tests.
//! **This crate's own code starts no process**; this file is a test of it.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use smoozy_job_files_machine::reading::{ArtistsText, RecordedLocation, TheInvocation, TheVersionInHand};
use smoozy_job_files_machine::reading_script::{the_pre_load_reading_script, the_reading_script};

/// The commit whose reading-script body is *today's walk*.
const TODAYS_COMMIT: &str = "38e3da887";

/// **No run of Blender here is waited on past this.** A heavy production
/// file walked at every frame can take minutes.
const ONE_RUN_AT_MOST: Duration = Duration::from_secs(1800);

fn the_five() -> Vec<(u16, u16, PathBuf)> {
    [
        (4u16, 5u16, "SMOOZY_DESK_BLENDER_45", "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"),
        (5, 0, "SMOOZY_DESK_BLENDER_50", "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"),
        (5, 1, "SMOOZY_DESK_BLENDER_51", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
        (5, 2, "SMOOZY_DESK_BLENDER_52", "/Applications/Blender 5.2.4.app/Contents/MacOS/Blender"),
        (5, 3, "SMOOZY_DESK_BLENDER_53", "/Applications/Blender 5.3.app/Contents/MacOS/Blender"),
    ]
    .into_iter()
    .map(|(major, minor, named, usual)| {
        (major, minor, PathBuf::from(std::env::var(named).unwrap_or_else(|_| usual.to_string())))
    })
    .collect()
}

/// The reading script's body at [`TODAYS_COMMIT`], read out of the history.
fn todays_body() -> String {
    let crate_folder = Path::new(env!("CARGO_MANIFEST_DIR"));
    let shown = Command::new("git")
        .arg("-C")
        .arg(crate_folder)
        .arg("show")
        .arg(format!("{TODAYS_COMMIT}:./src/reading_script.rs"))
        .output()
        .expect("git runs");
    assert!(shown.status.success(), "{}", String::from_utf8_lossy(&shown.stderr));
    let text = String::from_utf8(shown.stdout).expect("text");
    the_body_in(&text).to_string()
}

fn the_body_in(source: &str) -> &str {
    let opens = source.find("const THE_BODY: &str = r#\"").expect("the body opens");
    let starts = opens + "const THE_BODY: &str = r#\"".len();
    let ends = starts + source[starts..].find("\"#;").expect("the body closes");
    &source[starts..ends]
}

/// The composed script with today's body in place of the body as built.
fn as_today(script: &str, today: &str) -> String {
    let built = the_body_in(include_str!("../src/reading_script.rs"));
    assert!(script.ends_with(built), "the composed script ends with the body as built");
    format!("{}{}", &script[..script.len() - built.len()], today)
}

/// The script with its one step wrapped, so each move prints the scene's name.
fn counting_each_move(script: &str) -> String {
    let wrapped = script.replace("scene.frame_set(", "smoozy_desk_step(scene, ");
    assert!(wrapped != script, "the script's one step is where the test looks for it");
    wrapped.replacen(
        "import bpy, json, os, sys, time\n",
        "import bpy, json, os, sys, time\n\n\ndef smoozy_desk_step(scene, frame):\n    print(\"\\nSMOOZY-DESK-STEPPED\", scene.name, flush=True)\n    scene.frame_set(frame)\n\n",
        1,
    )
}

fn the_work_folder() -> PathBuf {
    let at = std::env::var("SMOOZY_DESK_WORK")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join(format!("smoozy-drivers-and-cheap-moves-{}", std::process::id())));
    for one in ["blender/tmp", "blender/config", "blender/scripts", "blender/ext", "blender/datafiles", "scripts", "made"] {
        std::fs::create_dir_all(at.join(one)).expect("a folder");
    }
    at
}

/// One run of Blender with its settings folders in the work folder, waited
/// on until it ends or [`ONE_RUN_AT_MOST`] passes.
fn run(blender: &Path, work: &Path, arguments: &[&str]) -> (Option<i32>, Vec<u8>, Duration) {
    let started = Instant::now();
    let mut child = Command::new(blender)
        .args(arguments)
        .env("TMPDIR", work.join("blender/tmp"))
        .env("BLENDER_USER_CONFIG", work.join("blender/config"))
        .env("BLENDER_USER_SCRIPTS", work.join("blender/scripts"))
        .env("BLENDER_USER_EXTENSIONS", work.join("blender/ext"))
        .env("BLENDER_USER_DATAFILES", work.join("blender/datafiles"))
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
    let status = loop {
        if let Some(status) = child.try_wait().expect("it can be asked") {
            break status;
        }
        if started.elapsed() > ONE_RUN_AT_MOST {
            let _ = child.kill();
            let _ = child.wait();
            panic!("{} {arguments:?} ran past {ONE_RUN_AT_MOST:?}", blender.display());
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let took = started.elapsed();
    (status.code(), reader.join().expect("the reader ends"), took)
}

/// What sits between the document's two marks, byte for byte; None where
/// there is no document.
fn between_the_marks(stream: &[u8]) -> Option<Vec<u8>> {
    use smoozy_job_files_machine::reading::{THE_CLOSING_SENTINEL, THE_OPENING_SENTINEL};
    let find = |hay: &[u8], needle: &[u8]| hay.windows(needle.len()).position(|w| w == needle);
    let opens = find(stream, THE_OPENING_SENTINEL)? + THE_OPENING_SENTINEL.len();
    let closes = find(&stream[opens..], THE_CLOSING_SENTINEL)?;
    Some(stream[opens..opens + closes].to_vec())
}

fn moves_in(stream: &[u8]) -> usize {
    String::from_utf8_lossy(stream).lines().filter(|line| line.starts_with("SMOOZY-DESK-STEPPED ")).count()
}

/// Reads `file` with `script`; gives back the status, the document, the
/// moves and the time.
fn read(blender: &Path, work: &Path, file: &Path, script: &str, called: &str) -> (Option<i32>, Option<Vec<u8>>, usize, Duration) {
    let reading = work.join("scripts").join(called);
    std::fs::write(&reading, script).expect("written");
    let (status, stream, took) = run(
        blender,
        work,
        &[
            "--factory-startup",
            "--background",
            "--enable-autoexec",
            "--python-exit-code",
            "63",
            file.to_str().unwrap(),
            "--python",
            reading.to_str().unwrap(),
        ],
    );
    (status, between_the_marks(&stream), moves_in(&stream), took)
}

/// Every `.blend` under `folder`, its own helpers' folders and back-up copies
/// left out, in name order.
fn the_blend_files_under(folder: &Path, into: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(folder) else { return };
    let mut entries: Vec<PathBuf> = entries.filter_map(|one| one.ok().map(|one| one.path())).collect();
    entries.sort();
    for path in entries {
        let name = path.file_name().map(|one| one.to_string_lossy().into_owned()).unwrap_or_default();
        if path.is_dir() {
            if !name.starts_with('_') {
                the_blend_files_under(&path, into);
            }
        } else if name.ends_with(".blend") && !name.contains("library") {
            into.push(path);
        }
    }
}

/// The version that saved the file, the scene it opens on and that scene's
/// frames, from a pre-load document.
fn what_the_document_says(document: &[u8]) -> ((u16, u16), Option<String>, Vec<i64>) {
    let value: serde_json::Value = serde_json::from_slice(document).expect("the document is JSON");
    let scenes = value["settings"]["scenes"].as_array().cloned().unwrap_or_default();
    let saved = scenes
        .first()
        .and_then(|one| one["saved_with"].as_str())
        .map(|text| {
            let mut parts = text.split('.').map(|one| one.parse::<u16>().unwrap_or(0));
            (parts.next().unwrap_or(0), parts.next().unwrap_or(0))
        })
        .unwrap_or((0, 0));
    let read: Option<String> = value["the_scene_it_read"]
        .as_array()
        .map(|bytes| bytes.iter().map(|one| one.as_u64().unwrap_or(0) as u8).collect::<Vec<u8>>())
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned());
    let mut frames = Vec::new();
    if let Some(name) = &read {
        if let Some(scene) = scenes.iter().find(|one| one["name"].as_str() == Some(name.as_str())) {
            let start = scene["frame_start"].as_i64().unwrap_or(1);
            let end = scene["frame_end"].as_i64().unwrap_or(start).max(start);
            frames = vec![start, (start + end) / 2, end];
            frames.dedup();
        }
    }
    (saved, read, frames)
}

/// **The cases the Creator's set lacks**, made neutral and written with
/// `bpy.data.libraries.write`, as the research's hand tests write theirs (an
/// ordinary save works the scene out and writes worked-out values over stored
/// ones). Each says in its name what it holds.
const THE_CASES_THE_SET_LACKS: &str = include_str!("drivers_and_cheap_moves_cases.py");

#[test]
#[ignore = "run by hand at the desk with the five Blenders"]
fn every_file_reads_the_same_by_todays_walk_and_the_read_as_built_in_all_five() {
    let work = the_work_folder();
    let today = todays_body();
    let maker = work.join("scripts").join("make-cases.py");
    std::fs::write(&maker, THE_CASES_THE_SET_LACKS).expect("written");
    let mut files = Vec::new();
    for (major, minor, blender) in the_five() {
        if (major, minor) != (4, 5) && (major, minor) != (5, 3) || std::env::var("SMOOZY_DESK_NO_CASES").is_ok() {
            continue;
        }
        let into = work.join("made").join(format!("{major}.{minor}"));
        let (made, said, _) = run(
            &blender,
            &work,
            &["--factory-startup", "--background", "--python-exit-code", "63", "--python", maker.to_str().unwrap(), "--", into.to_str().unwrap()],
        );
        assert_eq!(made, Some(0), "{major}.{minor}: the cases are made: {}", String::from_utf8_lossy(&said));
        the_blend_files_under(&into, &mut files);
    }
    let named = std::env::var("SMOOZY_DESK_DRIVER_FILES").unwrap_or_else(|_| {
        format!("{}/Smoozy driver test files", std::env::var("HOME").expect("a home"))
    });
    for folder in named.split(':').filter(|one| !one.is_empty()) {
        the_blend_files_under(Path::new(folder), &mut files);
    }
    if let Ok(only) = std::env::var("SMOOZY_DESK_ONLY") {
        files.retain(|one| one.to_string_lossy().contains(&only));
    }
    let mut results = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(work.join("results.jsonl"))
        .expect("the results file");
    let five = the_five();
    let newest = five.last().expect("5.3").clone();
    let mut differing = Vec::new();
    for file in &files {
        // What the file is, read by today's walk in the newest Blender.
        let invocation = |major: u16, minor: u16, blender: &Path, frames: Vec<i64>| TheInvocation {
            the_blender_build_path: blender.display().to_string(),
            the_blend_path_on_the_machine: RecordedLocation::of(file.to_str().unwrap().as_bytes().to_vec()),
            the_version_it_will_render_by: TheVersionInHand::of(major, minor),
            the_frames_the_job_covers: frames,
            the_check_is_written_as_a_picture: false,
            the_check_is_marked: false,
        };
        let first = the_pre_load_reading_script(&invocation(newest.0, newest.1, &newest.2, Vec::new())).expect("composed");
        let (_, document, _, _) = read(&newest.2, &work, file, &as_today(&first, &today), "probe.py");
        let Some(document) = document else {
            println!("{}: 5.3 gave no document; read in every Blender all the same", file.display());
            for (major, minor, blender) in &five {
                let script = the_pre_load_reading_script(&invocation(*major, *minor, blender, Vec::new())).expect("composed");
                let (today_status, today_document, _, today_took) = read(blender, &work, file, &counting_each_move(&as_today(&script, &today)), "today.py");
                let (new_status, new_document, _, new_took) = read(blender, &work, file, &counting_each_move(&script), "new.py");
                let same = today_document == new_document;
                if !same {
                    differing.push(format!("{} {major}.{minor} pre-load", file.display()));
                }
                let line = serde_json::json!({"file": file, "blender": format!("{major}.{minor}"), "mode": "pre-load",
                    "identical": same, "today_status": today_status, "new_status": new_status,
                    "today_seconds": today_took.as_secs_f64(), "new_seconds": new_took.as_secs_f64(), "no_document": true});
                writeln!(results, "{line}").expect("written");
            }
            continue;
        };
        let (saved, scene, frames) = what_the_document_says(&document);
        for (major, minor, blender) in &five {
            if saved > (*major, *minor) {
                let line = serde_json::json!({"file": file, "blender": format!("{major}.{minor}"),
                    "skipped": format!("made in {}.{}", saved.0, saved.1)});
                writeln!(results, "{line}").expect("written");
                println!("{} {major}.{minor}: skipped, made in {}.{}", file.display(), saved.0, saved.1);
                continue;
            }
            let pre_load = the_pre_load_reading_script(&invocation(*major, *minor, blender, Vec::new())).expect("composed");
            let mut readings = vec![("pre-load", pre_load)];
            if let Some(scene) = &scene {
                let render_start = the_reading_script(
                    &invocation(*major, *minor, blender, frames.clone()),
                    Some(&ArtistsText::of(scene.as_bytes().to_vec())),
                )
                .expect("composed");
                readings.push(("render-start", render_start));
            }
            for (mode, script) in readings {
                let (today_status, today_document, today_moves, today_took) =
                    read(blender, &work, file, &counting_each_move(&as_today(&script, &today)), "today.py");
                let (new_status, new_document, new_moves, new_took) =
                    read(blender, &work, file, &counting_each_move(&script), "new.py");
                let same = today_document == new_document && today_status == new_status;
                if !same {
                    differing.push(format!("{} {major}.{minor} {mode}", file.display()));
                    let stem = format!("differs-{}-{major}{minor}-{mode}", differing.len());
                    std::fs::write(work.join(format!("{stem}-today.json")), today_document.clone().unwrap_or_default()).expect("kept");
                    std::fs::write(work.join(format!("{stem}-new.json")), new_document.clone().unwrap_or_default()).expect("kept");
                }
                let line = serde_json::json!({"file": file, "blender": format!("{major}.{minor}"), "mode": mode,
                    "identical": same, "today_status": today_status, "new_status": new_status,
                    "today_document": today_document.is_some(), "new_document": new_document.is_some(),
                    "today_moves": today_moves, "new_moves": new_moves,
                    "today_seconds": today_took.as_secs_f64(), "new_seconds": new_took.as_secs_f64()});
                writeln!(results, "{line}").expect("written");
                results.flush().expect("flushed");
                println!(
                    "{} {major}.{minor} {mode}: identical {same}; today {:.2} s, {today_moves} moves; new {:.2} s, {new_moves} moves",
                    file.file_name().unwrap().to_string_lossy(),
                    today_took.as_secs_f64(),
                    new_took.as_secs_f64()
                );
            }
        }
    }
    assert!(differing.is_empty(), "these readings differ: {differing:#?}");
}

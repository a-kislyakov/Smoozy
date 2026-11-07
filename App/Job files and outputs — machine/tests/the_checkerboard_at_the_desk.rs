//! **The tenth notes, note 7: a copy with transparency comes out over the
//! checkerboard** (`Job files and outputs` B28, the Creator's tenth notes,
//! note 7). Kept in the tree and ignored. Run by hand at the desk, alone, one
//! Blender at a time:
//!
//! ```text
//! SMOOZY_DESK_OUT=<an empty folder> cargo test -p smoozy-job-files-machine \
//!     --test the_checkerboard_at_the_desk -- --ignored --nocapture
//! ```
//!
//! The Blender is 5.1.2 at its usual place, or `SMOOZY_DESK_BLENDER_51`. Every
//! run has `BLENDER_USER_RESOURCES` pointed at an empty folder of the test's
//! own, so no setting of the Creator's is read. **This crate's own code starts
//! no process**; this file is a test of it, not part of it.
//!
//! **What it does.** One Blender run renders the factory cube with Workbench,
//! at 480 by 270, as a transparent PNG, a transparent multi-layer OpenEXR, an
//! opaque PNG and an opaque multi-layer OpenEXR, and at 9000 by 3000 as a
//! transparent PNG the route scales. Each is made into a copy by **this
//! crate's own composed script** ([`the_derivatives_script`]), through the
//! size handshake the preview worker speaks. **Beside each, the same script
//! with the checkerboard's one call taken out** makes the copy as it was
//! before this piece. Then a run of the bundled image library alone — no
//! `bpy`, never Blender's own loader — reads every copy back to raw levels,
//! and the comparison is made here:
//!
//! - **every new copy is a PNG with no alpha channel** (its header's colour
//!   type, read here off the file's bytes, is 2);
//! - **where the picture was fully transparent, the copy is the board**: light
//!   grey in the top-left square and white, each square a 24th of the copy's
//!   shorter side;
//! - **where it was fully opaque, the copy's colours are the old copy's**,
//!   level for level — so an opaque picture is unchanged (a multi-layer
//!   file's clear and opaque pixels are allowed one level: its route works in
//!   floats, and an alpha a hair off nought or one still reads as 0 or 255 in
//!   the old copy — 5.1.2's Workbench writes an opaque frame's alpha as
//!   0.998046875);
//! - **where it was partly transparent, the copy is the picture laid over the
//!   board**, within two levels.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use smoozy_job_files_machine::derivatives::{
    it_was_made, the_derivatives_script, the_size_in, the_size_told, the_words_of_a_derivative,
    ADerivativeToMake, TheTreatment, SQUARES_ALONG_THE_SHORTER_SIDE, THE_BOARDS_GREY, THE_BOARDS_WHITE,
};
use smoozy_job_files_machine::main_output::the_target_size;

/// **No run of Blender here is waited on past this.**
const ONE_RUN_AT_MOST: Duration = Duration::from_secs(300);

const THE_SOURCES: &str = r#"
import bpy, sys
out = sys.argv[sys.argv.index("--") + 1]
scene = bpy.context.scene
scene.render.engine = "BLENDER_WORKBENCH"
scene.render.resolution_percentage = 100
settings = scene.render.image_settings
def write(name, multi_layer, transparent, width=480, height=270):
    scene.render.resolution_x = width
    scene.render.resolution_y = height
    scene.render.film_transparent = transparent
    if hasattr(settings, "media_type"):
        settings.media_type = "MULTI_LAYER_IMAGE" if multi_layer else "IMAGE"
    if not (multi_layer and hasattr(settings, "media_type")):
        settings.file_format = "OPEN_EXR_MULTILAYER" if multi_layer else "PNG"
    if not multi_layer:
        settings.color_mode = "RGBA"
        settings.color_depth = "8"
    scene.render.filepath = out + "/" + name
    bpy.ops.render.render(write_still=True)
write("transparent.png", False, True)
write("transparent-ml.exr", True, True)
write("opaque.png", False, False)
write("opaque-ml.exr", True, False)
write("wide-transparent.png", False, True, 9000, 3000)
print("SMOOZY-DESK-VIEW " + scene.view_settings.view_transform, flush=True)
"#;

/// **The bundled library alone reads every copy back**: its size, its
/// channels, and its levels as raw bytes beside it.
const THE_READING_BACK: &str = r#"
import json, sys
import OpenImageIO
for path in sys.argv[sys.argv.index("--") + 1:]:
    buf = OpenImageIO.ImageBuf(path)
    spec = buf.spec()
    pixels = buf.get_pixels(OpenImageIO.UINT8)
    with open(path + ".raw", "wb") as raw:
        raw.write(pixels.tobytes())
    print("SMOOZY-DESK-READ " + json.dumps({
        "path": path, "width": spec.width, "height": spec.height,
        "channels": list(spec.channelnames), "error": buf.geterror(),
    }), flush=True)
"#;

/// The checkerboard's one call in each route, and what stands in for it in
/// the copy as it was before this piece.
const THE_CALLS: [(&str, &str); 2] = [
    ("flat = laid_over_the_checkerboard(shown)", "flat = shown"),
    ("flat = laid_over_the_checkerboard(copy)", "flat = copy"),
];

fn the_blender() -> PathBuf {
    PathBuf::from(
        std::env::var("SMOOZY_DESK_BLENDER_51")
            .unwrap_or_else(|_| "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender".to_string()),
    )
}

/// One run of Blender, waited on until it ends or [`ONE_RUN_AT_MOST`] passes.
/// `answer` is handed what it has said so far and gives the line to tell it
/// on its standard input, once.
fn run(
    blender: &Path,
    no_settings: &Path,
    arguments: &[&str],
    answer: Option<Box<dyn Fn(&[u8]) -> Option<String> + Send>>,
) -> (Option<i32>, Vec<u8>, Vec<u8>) {
    let mut child = Command::new(blender)
        .args(arguments)
        .env("BLENDER_USER_RESOURCES", no_settings)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Blender starts");
    let mut input = child.stdin.take();
    if answer.is_none() {
        input = None;
    }
    let mut stdout = child.stdout.take().expect("its stream");
    let mut stderr = child.stderr.take().expect("its complaints");
    let heard = Arc::new(Mutex::new(Vec::new()));
    let heard_by_the_reader = Arc::clone(&heard);
    let reader = std::thread::spawn(move || {
        let mut chunk = [0u8; 65536];
        loop {
            let got = stdout.read(&mut chunk).unwrap_or(0);
            if got == 0 {
                break;
            }
            let mut all = heard_by_the_reader.lock().unwrap();
            all.extend_from_slice(&chunk[..got]);
            if let (Some(told), Some(answer)) = (input.as_mut(), answer.as_ref()) {
                if let Some(line) = answer(&all) {
                    let _ = told.write_all(line.as_bytes());
                    let _ = told.flush();
                    input = None;
                }
            }
        }
    });
    let complaints = std::thread::spawn(move || {
        let mut all = Vec::new();
        let _ = stderr.read_to_end(&mut all);
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
    reader.join().expect("the reader ends");
    let complaints = complaints.join().expect("the complaints end");
    let said = heard.lock().unwrap().clone();
    (status.code(), said, complaints)
}

/// **A PNG's colour type**, off its header's own bytes: 2 is RGB, 6 is RGBA.
fn the_png_colour_type(path: &Path) -> u8 {
    let bytes = std::fs::read(path).expect("the copy reads");
    assert_eq!(&bytes[1..4], b"PNG", "{} is a PNG", path.display());
    bytes[25]
}

/// One copy as the bundled library read it back.
struct ReadBack {
    width: usize,
    height: usize,
    channels: usize,
    levels: Vec<u8>,
}

impl ReadBack {
    fn at(&self, x: usize, y: usize) -> &[u8] {
        let from = (y * self.width + x) * self.channels;
        &self.levels[from..from + self.channels]
    }
}

/// **The board's level at a pixel**: grey in the top-left square.
fn the_board_at(x: usize, y: usize, side: usize) -> u8 {
    let grey = (THE_BOARDS_GREY[0] * 255.0).round() as u8;
    let white = (THE_BOARDS_WHITE[0] * 255.0).round() as u8;
    if (x / side + y / side) % 2 == 0 {
        grey
    } else {
        white
    }
}

#[test]
#[ignore = "run by hand at the desk with Blender 5.1.2, alone"]
fn tenth_notes_7_a_transparent_copy_comes_out_over_the_checkerboard_in_a_real_blender() {
    let out = PathBuf::from(std::env::var("SMOOZY_DESK_OUT").expect("SMOOZY_DESK_OUT"));
    std::fs::create_dir_all(&out).expect("the folder");
    let no_settings = out.join("no-user-settings");
    std::fs::create_dir_all(&no_settings).expect("an empty folder for Blender's user files");
    let blender = the_blender();
    let script_at = |name: &str, text: &str| {
        let at = out.join(name);
        std::fs::write(&at, text).expect("the script is written");
        at
    };

    // **The sources, rendered.**
    let sources = script_at("sources.py", THE_SOURCES);
    let (status, said, complaints) = run(
        &blender,
        &no_settings,
        &["-b", "--factory-startup", "--python-exit-code", "1", "--python", sources.to_str().unwrap(), "--", out.to_str().unwrap()],
        None,
    );
    assert_eq!(status, Some(0), "the sources render: {}", String::from_utf8_lossy(&complaints));
    let view = String::from_utf8_lossy(&said)
        .lines()
        .find_map(|line| line.strip_prefix("SMOOZY-DESK-VIEW ").map(str::to_string))
        .expect("the factory scene's view transform");
    println!("the factory scene's view transform: {view}");

    // **Each source made into a copy, by the composed script and by the
    // same script without the checkerboard.**
    let cases: [(&str, bool); 5] = [
        ("transparent.png", false),
        ("transparent-ml.exr", true),
        ("opaque.png", false),
        ("opaque-ml.exr", true),
        ("wide-transparent.png", false),
    ];
    let mut copies = Vec::new();
    for (name, multi_layer) in cases {
        for (made_as, before) in [("after", false), ("before", true)] {
            let staged = out.join(format!("{name}.{made_as}.png"));
            let mut script = the_derivatives_script(&ADerivativeToMake {
                the_composited_picture: None,
                source: out.join(name).to_str().unwrap().as_bytes().to_vec(),
                staged_at: staged.to_str().unwrap().to_string(),
                treatment: if multi_layer {
                    TheTreatment::TheJobsOwnViewTransform(view.as_bytes().to_vec())
                } else {
                    TheTreatment::ItAlreadyCarriesItsTreatment
                },
                the_picture_inside: multi_layer.then(|| b"ViewLayer.Combined".to_vec()),
            });
            if before {
                let (call, plain) = THE_CALLS[usize::from(!multi_layer)];
                assert_eq!(script.matches(call).count(), 1, "{name}: the call is in the script once");
                script = script.replace(call, plain);
            }
            let at = script_at(&format!("{name}.{made_as}.py"), &script);
            let words: Vec<String> = the_words_of_a_derivative(at.to_str().unwrap())
                .into_iter()
                .map(|word| String::from_utf8(word).unwrap())
                .collect();
            let words: Vec<&str> = words.iter().map(String::as_str).collect();
            let started = Instant::now();
            let (status, said, complaints) = run(
                &blender,
                &no_settings,
                &words,
                Some(Box::new(|heard: &[u8]| {
                    the_size_in(heard).map(|(width, height)| the_size_told(the_target_size(width, height)))
                })),
            );
            println!(
                "{name} ({made_as}): status {status:?}, size {:?}, made {}, {:.1} s",
                the_size_in(&said),
                it_was_made(&said),
                started.elapsed().as_secs_f64()
            );
            assert_eq!(status, Some(0), "{name} ({made_as}): {}", String::from_utf8_lossy(&complaints));
            assert!(it_was_made(&said), "{name} ({made_as}): the made mark");
            copies.push((name, before, staged));
        }
    }

    // **Every copy read back by the bundled library alone.**
    let reader = script_at("read-back.py", THE_READING_BACK);
    let mut arguments: Vec<String> = ["-b", "--factory-startup", "--python-exit-code", "1", "--python"]
        .iter()
        .map(|word| word.to_string())
        .collect();
    arguments.push(reader.to_str().unwrap().to_string());
    arguments.push("--".to_string());
    arguments.extend(copies.iter().map(|(_, _, at)| at.to_str().unwrap().to_string()));
    let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    let (status, said, complaints) = run(&blender, &no_settings, &arguments, None);
    assert_eq!(status, Some(0), "the copies are read back: {}", String::from_utf8_lossy(&complaints));
    let said = String::from_utf8_lossy(&said).to_string();
    let read_back = |at: &Path| -> ReadBack {
        let line: serde_json::Value = said
            .lines()
            .filter_map(|line| line.strip_prefix("SMOOZY-DESK-READ "))
            .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("a reading"))
            .find(|line| line["path"] == at.to_str().unwrap())
            .unwrap_or_else(|| panic!("{} was read back", at.display()));
        assert_eq!(line["error"], "", "{}", at.display());
        let width = line["width"].as_u64().unwrap() as usize;
        let height = line["height"].as_u64().unwrap() as usize;
        let channels = line["channels"].as_array().unwrap().len();
        let levels = std::fs::read(format!("{}.raw", at.display())).expect("the levels");
        assert_eq!(levels.len(), width * height * channels, "{}", at.display());
        ReadBack { width, height, channels, levels }
    };

    for (name, _) in cases {
        let after_at = out.join(format!("{name}.after.png"));
        let before_at = out.join(format!("{name}.before.png"));
        let after = read_back(&after_at);
        let before = read_back(&before_at);
        assert_eq!(the_png_colour_type(&after_at), 2, "{name}: the copy is RGB, no alpha");
        assert_eq!(after.channels, 3, "{name}: the copy has three channels");
        assert_eq!(the_png_colour_type(&before_at), 6, "{name}: the old copy kept an alpha channel");
        assert_eq!(before.channels, 4, "{name}");
        assert_eq!((after.width, after.height), (before.width, before.height), "{name}: the same size");
        let side = (after.width.min(after.height) / SQUARES_ALONG_THE_SHORTER_SIDE as usize).max(1);
        // **The multi-layer route works in floats**: an alpha a hair off
        // nought or one reads as 0 or 255 in the old eight-bit copy and still
        // moves the new one by a level. Observed here: 5.1.2's Workbench
        // writes the opaque multi-layer file's alpha as 0.998046875
        // everywhere, not 1. The PNG route's match is exact.
        let slack = if name.ends_with("-ml.exr") { 1 } else { 0 };
        let off_by = |new: &[u8], expected: [u8; 3]| -> i32 {
            (0..3).map(|c| (i32::from(new[c]) - i32::from(expected[c])).abs()).max().unwrap()
        };
        let (mut clear, mut opaque, mut partial, mut worst) = (0usize, 0usize, 0usize, 0i32);
        let (mut worst_clear, mut worst_opaque) = (0i32, 0i32);
        for y in 0..after.height {
            for x in 0..after.width {
                let new = after.at(x, y);
                let old = before.at(x, y);
                let alpha = old[3];
                let board = the_board_at(x, y, side);
                match alpha {
                    0 => {
                        clear += 1;
                        let off = off_by(new, [board, board, board]);
                        worst_clear = worst_clear.max(off);
                        assert!(off <= slack, "{name}: ({x}, {y}) is the board: {new:?}, {board}");
                    }
                    255 => {
                        opaque += 1;
                        let off = off_by(new, [old[0], old[1], old[2]]);
                        worst_opaque = worst_opaque.max(off);
                        assert!(off <= slack, "{name}: ({x}, {y}) is the picture, unchanged: {new:?}, {old:?}");
                    }
                    _ => {
                        partial += 1;
                        // The library reads the old copy's colour already
                        // multiplied by its alpha.
                        let a = f64::from(alpha) / 255.0;
                        for channel in 0..3 {
                            let expected = f64::from(old[channel]) + f64::from(board) * (1.0 - a);
                            let off = (f64::from(new[channel]) - expected).round().abs() as i32;
                            worst = worst.max(off);
                            assert!(off <= 2, "{name}: ({x}, {y}) channel {channel}: {} against {expected:.1}", new[channel]);
                        }
                    }
                }
            }
        }
        println!(
            "{name}: {} x {}, squares of {side}; {clear} clear (worst {worst_clear} levels off), {opaque} opaque (worst {worst_opaque}), {partial} partly clear (worst {worst}); top-left {:?}, one square right {:?}",
            after.width,
            after.height,
            after.at(0, 0),
            after.at(side.min(after.width - 1), 0)
        );
        if name.starts_with("transparent") || name.starts_with("wide") {
            assert!(clear > 0 && opaque > 0 && partial > 0, "{name}: all three kinds of pixel are in play");
            // The corner is clear: grey there, white one square along.
            assert_eq!(after.at(0, 0), [204, 204, 204], "{name}: the top-left square is light grey");
            assert_eq!(after.at(side, 0), [255, 255, 255], "{name}: the next square is white");
            assert_eq!(after.at(side - 1, 0), [204, 204, 204], "{name}: a square is {side} wide");
        } else {
            assert_eq!(clear + partial, 0, "{name}: an opaque picture");
        }
    }
    let wide = read_back(&out.join("wide-transparent.png.after.png"));
    assert_eq!((wide.width, wide.height), (7500, 2500), "the wide copy is scaled to its target");
}

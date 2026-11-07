//! **The 5.3 integration's readings through the render worker's own driver,
//! with real Blenders** — kept in the tree and ignored. Each one reads what a
//! Blender does with Smoozy's own reading script, launch material and composed
//! line; none of them is about progress or speed. Run by hand at the desk, one
//! Blender at a time:
//!
//! ```text
//! SMOOZY_DESK_OUT=<scratch folder> cargo test -p smoozy-rendering-machine the_5_3_readings_at_the_desk -- --ignored --nocapture --test-threads=1
//! ```
//!
//! `SMOOZY_DESK_ONLY=53` narrows a run to one Blender. Each Blender is found at
//! its usual place, or at `SMOOZY_DESK_BLENDER_45`, `_50`, `_51`, `_52`, `_53`
//! and `_54`, and the runs are kept under `SMOOZY_DESK_OUT` so what came back can
//! be looked at. The results are written up in `Audits/Build/Blender 5.3/`, in
//! the accounts of pieces 1, 3, 4 and 5; each test's assertions hold what was
//! observed on the builds named there, so a run against a later build fails
//! where Blender moved.
//!
//! **What runs is the driver, whole**, as [`super::simplify_at_the_desk`] runs
//! it: the real version ask, the real reading of the file, everything composed
//! from it, the script before the file and the launch material placed and read
//! back on disk, and the render launched with the line's own words. Only the
//! machine's own records are stand-ins. The build path is a small wrapper
//! adding the factory-startup switch at the head, as the desk's other
//! real-Blender tests do, and it notes the words it was given.
//!
//! **Piece 1's three**, by the name each has in `What 5.3 changed`:
//!
//! - **The old name's warning (Dependencies 17, `Triage A`, *Beside it*).** A
//!   real job's shape, a File Output node in the scene's tree: which of the
//!   render's two streams, and of the reading's, carries the warning. The
//!   ordinary stream is the Output tab and the complaints stream the Errors tab
//!   (`presentation/the_machines_own_words.rs`, `Rendering` L7).
//! - **`{project_root}` in a File Output node's folder (File output nodes 5).**
//!   No project, as a job on a rented machine has none: where the files land.
//!   Piece 5 made it assert, with its neighbours.
//! - **The video-to-still switch (Writing 6, `Rendering` L10c).** A video-only
//!   scene as a first-frame check, alone and writing into its range job's
//!   folder.
//!
//! **Piece 3 added the compositor as a stack of effects.** **Piece 4 added a
//! typed count under EEVEE against a file's time limit, and a bake folder's
//! template through to the picture.** **Piece 5 added a node in a nested group
//! in the versions before 5.3.** Each has its own account beside piece 1's.
//!
//! **What this does not prove**: Linux or a graphics card. Every run is on the
//! processor, on this Mac, with files saved by a Blender with no window.
//!
//! **Names used in the files made here**, each read off the three Blenders'
//! own property lists at the desk on 2026-11-02 (the same account).

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
use crate::acting::reading::{the_words_of_a_reading, write_the_reads_load_time_script, TheReadingOnTheMachine};
use crate::acting::the_optix_library::LooksForTheOptixLibrary;
use crate::composition::tests::a_manifest;
use crate::material::tests::{a_blender, a_job, an_untouched_payload, four_values};

/// **No run of Blender here is waited on past this.**
const ONE_RUN_AT_MOST: Duration = Duration::from_secs(300);

/// What every file here shares: the factory scene on the processor, 64 pixels
/// square, written as a picture unless the case says otherwise.
pub(super) const THE_SMALL_SCENE: &str = r#"
import bpy, sys, warnings
warnings.simplefilter("ignore", DeprecationWarning)
out = sys.argv[sys.argv.index("--") + 1]
scene = bpy.context.scene
scene.name = "Scene"
scene.render.engine = "CYCLES"
scene.cycles.device = "CPU"
scene.cycles.samples = 2
scene.cycles.use_adaptive_sampling = False
scene.cycles.use_denoising = False
scene.render.resolution_x = 64
scene.render.resolution_y = 64
scene.render.resolution_percentage = 100
scene.render.filepath = "//render/f_"
scene.render.image_settings.file_format = "PNG"
scene.frame_start = 1
scene.frame_end = 2
"#;

/// **A File Output node in the scene's tree**, its folder handed in. The tree
/// goes on by the name every version from 5.0 has, which in 5.3 and 5.4 makes
/// the one effect a file saved by 5.2 opens with.
fn a_file_with_a_file_output_node(folder: &str) -> String {
    format!(
        r#"{THE_SMALL_SCENE}
tree = bpy.data.node_groups.new("Compositor", "CompositorNodeTree")
scene.compositing_node_group = tree
rl = tree.nodes.new("CompositorNodeRLayers")
rl.scene = scene
rl.layer = scene.view_layers[0].name
fo = tree.nodes.new("CompositorNodeOutputFile")
fo.name = "File Output"
fo.directory = {folder:?}
fo.file_name = "pass_"
fo.format.media_type = "IMAGE"
fo.format.file_format = "PNG"
fo.file_output_items.clear()
fo.file_output_items.new("RGBA", "Image")
tree.links.new(rl.outputs[0], fo.inputs[0])
print("SMOOZY-DESK-MADE", bpy.app.version_string, bpy.app.build_hash.decode(), repr(fo.directory), flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#
    )
}

/// **A scene whose only output is a video**, no compositor.
fn a_video_only_file() -> String {
    format!(
        r#"{THE_SMALL_SCENE}
settings = scene.render.image_settings
settings.media_type = "VIDEO"
settings.file_format = "FFMPEG"
scene.render.ffmpeg.format = "MPEG4"
scene.render.ffmpeg.codec = "H264"
scene.render.filepath = "//render/v_"
print("SMOOZY-DESK-MADE", bpy.app.version_string, bpy.app.build_hash.decode(), settings.media_type, settings.file_format, scene.render.is_movie_format, flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#
    )
}

pub(super) fn the_blenders(wanted: &[&'static str]) -> Vec<(&'static str, PathBuf)> {
    [
        ("4.5", "45", "SMOOZY_DESK_BLENDER_45", "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"),
        ("5.0", "50", "SMOOZY_DESK_BLENDER_50", "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"),
        ("5.1", "51", "SMOOZY_DESK_BLENDER_51", "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"),
        ("5.2", "52", "SMOOZY_DESK_BLENDER_52", "/Applications/Blender.app/Contents/MacOS/Blender"),
        ("5.3", "53", "SMOOZY_DESK_BLENDER_53", "/Applications/Blender 5.3b.app/Contents/MacOS/Blender"),
        ("5.4", "54", "SMOOZY_DESK_BLENDER_54", "/Applications/Blender 5.4a.app/Contents/MacOS/Blender"),
    ]
    .into_iter()
    .filter(|(named, _, _, _)| wanted.contains(named))
    .filter(|(_, short, _, _)| std::env::var("SMOOZY_DESK_ONLY").map_or(true, |only| only == *short))
    .map(|(named, _, variable, usual)| {
        (named, PathBuf::from(std::env::var(variable).unwrap_or_else(|_| usual.to_string())))
    })
    .collect()
}

/// One run of a program, its two streams kept apart, waited on until it ends or
/// [`ONE_RUN_AT_MOST`] passes.
pub(super) fn run(program: &Path, arguments: &[String]) -> (Option<i32>, Vec<u8>, Vec<u8>) {
    let mut child = std::process::Command::new(program)
        .args(arguments)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("it starts");
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
    let status = wait_for(&mut child, &format!("{}", program.display()));
    (status, out.join().expect("the reader ends"), err.join().expect("the reader ends"))
}

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

/// A file made by a script of its own, by the Blender under test.
pub(super) fn make(blender: &Path, script: &str, here: &Path, name: &str) -> PathBuf {
    std::fs::create_dir_all(here).expect("a folder");
    let maker = here.join(format!("make-{name}.py"));
    std::fs::write(&maker, script).expect("written");
    let file = here.join(format!("{name}.blend"));
    let (status, out, err) = run(
        blender,
        &[
            "--factory-startup".to_string(),
            "--background".to_string(),
            "--python-exit-code".to_string(),
            "63".to_string(),
            "--python".to_string(),
            maker.display().to_string(),
            "--".to_string(),
            file.display().to_string(),
        ],
    );
    let said = String::from_utf8_lossy(&out);
    println!("    {}", said.lines().find(|line| line.starts_with("SMOOZY-DESK-MADE")).unwrap_or("no made line"));
    assert_eq!(status, Some(0), "{name} is made: {said} {}", String::from_utf8_lossy(&err));
    file
}

/// **The machine's own records, as stand-ins that answer and never judge**,
/// folders made for real, and what the driver hands them kept.
struct TheDesksMachine {
    remap: RemapMaterial,
    /// Where a sent bake's copy really lands, and which folders were copied.
    copied: Vec<(PathBuf, PathBuf)>,
    pub(super) holds: Vec<String>,
    readings: Vec<TheReadingAndWhatFollows>,
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
    fn write_the_recorded_reading(&mut self, composed: &TheReadingAndWhatFollows) {
        self.readings.push(composed.clone());
    }
    fn ask_the_supervisor_to_place_the_hold(&mut self, reason: &str) -> Option<Identity> {
        self.holds.push(reason.to_string());
        Identity::issue(IdentityKind::Operation).ok()
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
    /// A folder copied for real, as the machine's own program does. The list of
    /// what was placed is the machine's bookkeeping and nothing here reads it.
    fn copy_into_the_job_folder(&mut self, from: &[u8], to: &[u8]) -> Result<Vec<APlacedFile>, String> {
        let from = PathBuf::from(String::from_utf8_lossy(from).into_owned());
        let to = PathBuf::from(String::from_utf8_lossy(to).into_owned());
        a_folder_copied(&from, &to);
        self.copied.push((from, to));
        Ok(Vec::new())
    }
    fn write_the_placed_copies(&mut self, _placed: &ThePlacedCopies) {}
    fn make_the_opening_folder(
        &mut self,
        folder: &smoozy_dependencies_machine::opening::TheOpeningFolder,
    ) -> Result<(), String> {
        crate::acting::opening::make_the_opening_folder(folder)
    }
    fn record_the_render_process(
        &mut self,
        _pair: smoozy_runtime::execution_group::RecordedPair,
    ) -> Result<(), String> {
        Ok(())
    }
}

/// L24b's lookup is asked only where the job names `OPTIX`. **This Mac has no
/// OptiX library, and it says so**: until 2026-11-02 this answered that it had,
/// which no job here reached. So a job that names `OPTIX` ends here before
/// anything is launched, as on a machine with no such library.
///
/// **Reading the film's scene is the one exception**: its file names `OPTIX`, and
/// that test's wrapper never renders (`a_reading_sees_its_libraries_at_the_desk`).
struct NoCards;
impl LooksForTheOptixLibrary for NoCards {
    fn what_the_loader_finds(&self) -> WhatTheLoaderFound {
        if std::env::var_os("SMOOZY_DESK_FILM").is_some() {
            return WhatTheLoaderFound::TheLibrary;
        }
        WhatTheLoaderFound::NoLibrary
    }
}

/// What one job came back with.
pub(super) struct WhatCameBack {
    /// How the attempt ended, in words, where no render was launched.
    pub(super) not_launched: Option<String>,
    holds: Vec<String>,
    pub(super) the_renders_status: Option<i32>,
    pub(super) the_renders_output: String,
    pub(super) the_renders_errors: String,
    the_readings_output: String,
    the_readings_errors: String,
    /// **How long the reading took**, the same words the driver used, run again
    /// with both streams kept.
    pub(super) the_reading_took: Duration,
    /// Every file under the job's folder, relative to it.
    pub(super) in_the_jobs_folder: Vec<String>,
    /// Every file under the folder the `.blend` sits in, outside the job's
    /// folder and Smoozy's own working files.
    beside_the_file: Vec<String>,
    /// The last line of the wrapper's note: the render's own words.
    pub(super) the_line: String,
    /// The driver's composed reading, map, inventory and picks, where one was.
    pub(super) the_composed: Option<TheReadingAndWhatFollows>,
}

impl WhatCameBack {
    pub(super) fn holds(&self) -> &[String] {
        &self.holds
    }

    fn the_lines_with_invalid_path_template(&self) -> Vec<&str> {
        the_lines_with(&self.the_renders_output, "Invalid path template")
    }
}

fn every_file_under(folder: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut waiting = vec![folder.to_path_buf()];
    while let Some(at) = waiting.pop() {
        for entry in std::fs::read_dir(&at).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                waiting.push(path);
            } else {
                out.push(path.strip_prefix(folder).unwrap().to_string_lossy().into_owned());
            }
        }
    }
    out.sort();
    out
}

fn a_folder_copied(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a folder");
    for entry in std::fs::read_dir(from).expect("it lists").flatten() {
        let at = to.join(entry.file_name());
        if entry.path().is_dir() {
            a_folder_copied(&entry.path(), &at);
        } else {
            std::fs::copy(entry.path(), &at).expect("copied");
        }
    }
}

/// **One job through the render worker's driver**, every stream kept on disk
/// under `root`.
fn one_job(blender: &Path, file: &Path, root: &Path, payload: &Payload, frames: Vec<i64>) -> WhatCameBack {
    one_job_owing(blender, file, root, "Scene", payload, frames, Vec::new(), None)
}

/// **The same, with the remap material's pairs** (`Dependency Upload & Remap` M8b
/// and `Dependency Upload & Remap` M4b, in `libraries_at_load_at_the_desk`).
pub(super) fn one_job_with_pairs(
    blender: &Path,
    file: &Path,
    root: &Path,
    payload: &Payload,
    frames: Vec<i64>,
    pairs: Vec<smoozy_dependencies_machine::remap::Pair>,
) -> WhatCameBack {
    one_job_whole(blender, file, root, "Scene", payload, frames, Vec::new(), None, pairs)
}

/// **The same, with the copies a sent bake owes, and a folder already beside the
/// file on the machine**, as a bake uploaded with it would be: `(from, as)`.
#[allow(clippy::too_many_arguments)]
fn one_job_owing(
    blender: &Path,
    file: &Path,
    root: &Path,
    scene: &str,
    payload: &Payload,
    frames: Vec<i64>,
    copies_owed: Vec<smoozy_dependencies_machine::placed_copies::TheCopyTheMachineOwes>,
    beside_it: Option<(&Path, &str)>,
) -> WhatCameBack {
    one_job_whole(blender, file, root, scene, payload, frames, copies_owed, beside_it, Vec::new())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn one_job_whole(
    blender: &Path,
    file: &Path,
    root: &Path,
    scene: &str,
    payload: &Payload,
    frames: Vec<i64>,
    copies_owed: Vec<smoozy_dependencies_machine::placed_copies::TheCopyTheMachineOwes>,
    beside_it: Option<(&Path, &str)>,
    pairs: Vec<smoozy_dependencies_machine::remap::Pair>,
) -> WhatCameBack {
    std::fs::create_dir_all(root).expect("the job's folder");
    let at = |name: &str| root.join(name).display().to_string();
    let wrapper = root.join("blender-with-factory-startup");
    std::fs::write(
        &wrapper,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nexec '{}' --factory-startup \"$@\"\n",
            at("words.log"),
            blender.display()
        ),
    )
    .expect("the wrapper");
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).expect("runnable");
    let the_files_own_folder = root.join("rendering");
    std::fs::create_dir_all(&the_files_own_folder).expect("a folder");
    let blend = the_files_own_folder.join(file.file_name().expect("a name"));
    std::fs::copy(file, &blend).expect("the file is put in place");
    if let Some((from, named)) = beside_it {
        a_folder_copied(from, &the_files_own_folder.join(named));
    }
    let job_folder = root.join("job");

    // **L14's ask, for real**, so the manifest names what the binary names.
    let ask = TheShortInvocationOnTheMachine {
        the_scripts_path: at("ask.py"),
    };
    let version = ask.what_version_it_names(&wrapper.display().to_string()).expect("the Blender names a version");
    let version_text = format!("{}.{}.{}", version.major, version.minor, version.patch);

    let job = a_job();
    let mut manifest = a_manifest(&job, &version_text, scene);
    manifest.frame_range = FrameRange::TheUsersParsedIntegers(frames);
    manifest.blend_path_on_the_machine = blend.display().to_string();
    let mut machine = TheDesksMachine {
        remap: RemapMaterial::composed(&job, &a_blender(), pairs).with_the_copies_owed(copies_owed),
        copied: Vec::new(),
        holds: Vec::new(),
        readings: Vec::new(),
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
            the_folder_the_blend_sits_in: the_files_own_folder.display().to_string().as_bytes(),
            previous_attempts: &[],
            number: 1,
            the_machines_clock: &clock,
            the_manifests_reading: None,
            the_manifests_settings_half: None,
        })
    };
    let (not_launched, the_renders_status, ordinary, complaints) = match went.ended {
        WhereTheAttemptEnded::TheRenderWasLaunched(mut started) => {
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
            (None, status, ordinary.join().expect("the reader ends"), complaints.join().expect("the reader ends"))
        }
        other => (Some(format!("{other:?}")), None, Vec::new(), Vec::new()),
    };
    let the_line = std::fs::read_to_string(at("words.log"))
        .unwrap_or_default()
        .lines()
        .last()
        .unwrap_or("")
        .to_string();
    std::fs::write(root.join("render-output.log"), &ordinary).ok();
    std::fs::write(root.join("render-errors.log"), &complaints).ok();

    // **The reading's own two streams**, by the same words and the same script
    // the driver just used. The driver itself holds only the ordinary stream
    // (`acting/reading.rs`); the desktop's page holds both.
    let (load_time, _throwaway) = write_the_reads_load_time_script(&at("reading.py")).expect("the script before the file");
    // **Through the folder the driver read it through**, where it made one
    // (`Job files and outputs` R1a).
    let opened_at = root.join("opening-1").join(file.file_name().expect("a name"));
    let the_file_read = if opened_at.exists() { opened_at } else { blend.clone() };
    let words = the_words_of_a_reading(the_file_read.to_str().unwrap(), &load_time, &at("reading.py")).expect("the words");
    let began = Instant::now();
    let (_, the_readings_output, the_readings_errors) = run(&wrapper, &words);
    let the_reading_took = began.elapsed();
    std::fs::write(root.join("reading-output.log"), &the_readings_output).ok();
    std::fs::write(root.join("reading-errors.log"), &the_readings_errors).ok();

    let the_inventory = machine
        .readings
        .last()
        .map(|composed| format!("{:#?}", composed.inventory))
        .unwrap_or_else(|| "no reading was composed".to_string());
    std::fs::write(root.join("inventory.txt"), &the_inventory).ok();
    if let Some(composed) = machine.readings.last() {
        std::fs::write(
            root.join("reading.json"),
            serde_json::to_string_pretty(&composed.reading).expect("the reading writes out"),
        )
        .ok();
    }

    WhatCameBack {
        not_launched,
        holds: machine.holds.clone(),
        the_renders_status,
        the_renders_output: String::from_utf8_lossy(&ordinary).to_string(),
        the_renders_errors: String::from_utf8_lossy(&complaints).to_string(),
        the_readings_output: String::from_utf8_lossy(&the_readings_output).to_string(),
        the_readings_errors: String::from_utf8_lossy(&the_readings_errors).to_string(),
        the_reading_took,
        in_the_jobs_folder: every_file_under(&job_folder),
        beside_the_file: every_file_under(&the_files_own_folder)
            .into_iter()
            .filter(|name| !name.ends_with(".blend"))
            .collect(),
        the_line,
        the_composed: machine.readings.pop(),
    }
}

fn the_lines_with<'a>(stream: &'a str, words: &str) -> Vec<&'a str> {
    stream.lines().filter(|line| line.contains(words)).collect()
}

pub(super) fn the_folder() -> PathBuf {
    let out = std::env::var("SMOOZY_DESK_OUT").map(PathBuf::from).unwrap_or_else(|_| {
        std::env::temp_dir().join(format!(
            "smoozy-5-3-readings-{}",
            Identity::issue(IdentityKind::Operation).unwrap().as_text()
        ))
    });
    std::fs::create_dir_all(&out).expect("a folder");
    println!("the runs are kept under {}", out.display());
    out
}

/// **Dependencies 17 on a real job's shape: where the warning goes.** Until
/// piece 3 it reached the Errors stream of both runs on 5.3 and 5.4 (`Piece 1,
/// the desk readings — what landed.md`, reading 2). Now no stream holds it.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2, 5.3 beta and 5.4 alpha, one at a time"]
fn the_old_names_warning_reaches_which_stream() {
    const THE_WARNING: &str = "DeprecationWarning";
    let out = the_folder().join("the-warning");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.2", "5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let file = make(&blender, &a_file_with_a_file_output_node("//passes"), &here, "job");
        let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1, 2]);
        println!("    the line: {}", came.the_line);
        println!("    not launched: {:?}; held: {:?}; the render ended at {:?}", came.not_launched, came.holds, came.the_renders_status);
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        let count = |stream: &str| the_lines_with(stream, THE_WARNING).len();
        println!(
            "    the warning, lines: the render's Output {}, the render's Errors {}, the reading's Output {}, the reading's Errors {}",
            count(&came.the_renders_output),
            count(&came.the_renders_errors),
            count(&came.the_readings_output),
            count(&came.the_readings_errors),
        );
        println!("    the render's Errors, whole: {:?}", came.the_renders_errors);
        println!("    the reading's Errors, whole: {:?}", came.the_readings_errors);
        if came.the_renders_status != Some(0) {
            faults.push(format!("{named}: the render ended at {:?}", came.the_renders_status));
        }
        if count(&came.the_renders_errors) + count(&came.the_readings_errors) + count(&came.the_renders_output) + count(&came.the_readings_output) != 0 {
            faults.push(format!("{named}: a line of Smoozy's own making"));
        }
        // **Piece 3 ended it**: `Job files and outputs` D8a. On
        // 5.2 the name is no warning at all, on 5.3 and 5.4 it is no longer read.
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **File output nodes 5, and `Job files and outputs` P7b: an expression in a
/// node's folder.** The file makes a node with each folder in turn, no project,
/// as a job on a rented machine has none. Left alone in 5.2.3, 5.3 beta and 5.4
/// alpha, `{project_root}`, a format specifier and an unknown name each print
/// `Invalid path template`, and the node writes nothing; `{blend_dir}` names the
/// file's folder; `{scene_name}`, `{node_name}` and `{fps}` are worked out
/// (`Audits/Build/Blender 5.3/Piece 5, what the desk tests found — what
/// landed.md`). **Through Smoozy every one writes under the job's folder and
/// pushes nothing else deep**, and each one that clause replaces is under a folder with no
/// brace in it, and counted.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2, 5.3 beta and 5.4 alpha, one at a time"]
fn a_project_root_folder_with_no_project_lands_where() {
    let out = the_folder().join("project-root");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.2", "5.3", "5.4"]) {
        println!("{named}:");
        for (at, (folder, the_node_lands_in, it_is_counted)) in [
            ("{project_root}/out", "project_root/out", true),
            ("//renders/{project_root}/out", "renders/project_root/out", true),
            ("{blend_dir}/out", "blend_dir/out", true),
            ("//{blend_dir}/out", "blend_dir/out", true),
            ("{scene_name}/out", "scene_name/out", true),
            ("{node_name}/out", "node_name/out", true),
            ("//{fps:03}/out", "fps_03/out", true),
            ("//{unknown}/out", "unknown/out", true),
            ("//{unclosed/out", "unclosed/out", true),
            ("{blend_name}/out", "job/out", true),
            ("{{lit}}/out", "{lit}/out", true),
            // What Blender works out from the file's own place, as it was: the
            // folder as the user would have it, and not counted (`Job files and outputs` I2).
            ("//{scene_name}/out", "Scene/out", false),
        ]
        .into_iter()
        .enumerate()
        {
            let here = out.join(named).join(at.to_string());
            let file = make(&blender, &a_file_with_a_file_output_node(folder), &here, "job");
            let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1]);
            println!("    {folder}: the render ended at {:?}; in the job's folder: {:?}; beside the file: {:?}", came.the_renders_status, came.in_the_jobs_folder, came.beside_the_file);
            let wanted = format!("{the_node_lands_in}/pass_Image0001.png");
            if !came.in_the_jobs_folder.contains(&wanted) {
                faults.push(format!("{named}, {folder}: no {wanted} in {:?}", came.in_the_jobs_folder));
            }
            if came.in_the_jobs_folder.iter().any(|name| name.starts_with("private/") || name.starts_with("Users/")) {
                faults.push(format!("{named}, {folder}: pushed deep: {:?}", came.in_the_jobs_folder));
            }
            if !came.the_lines_with_invalid_path_template().is_empty() {
                faults.push(format!("{named}, {folder}: Blender still refused the path"));
            }
            if it_is_counted {
                the_inventory_matches_the_disk(&format!("{named}, {folder}"), &came, &mut faults);
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **`Job files and outputs` P7b: two nodes whose folders held expressions keep
/// apart from each other and from a node whose own folder has that name.**
#[test]
#[ignore = "run by hand at the desk with Blender 5.2, 5.3 beta and 5.4 alpha, one at a time"]
fn two_nodes_whose_folders_held_expressions_do_not_collide() {
    let out = the_folder().join("expressions-apart");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.2", "5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let script = format!(
            r#"{}
def another(name, folder, file_name):
    two = tree.nodes.new("CompositorNodeOutputFile")
    two.name = name
    two.directory = folder
    two.file_name = file_name
    two.format.media_type = "IMAGE"
    two.format.file_format = "PNG"
    two.file_output_items.clear()
    two.file_output_items.new("RGBA", "Image")
    tree.links.new(rl.outputs[0], two.inputs[0])
another("Same expression", "{{project_root}}/out", "other_")
another("Another expression", "{{unknown}}/out", "pass_")
another("Spelled the same", "//render/project_root/out", "pass_")
another("By its own name", "{{node_name}}/out", "pass_")
another("By its own name too", "{{node_name}}/out", "pass_")
bpy.ops.wm.save_as_mainfile(filepath=out)
"#,
            a_file_with_a_file_output_node("{project_root}/out")
        );
        let file = make(&blender, &script, &here, "job");
        let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1]);
        println!("    in the job's folder: {:?}; held: {:?}", came.in_the_jobs_folder, came.holds);
        for wanted in [
            "unknown/out/pass_Image0001.png",
            "project_root/out/pass_Image0001.png",
            "project_root/out-2/pass_Image0001.png",
            "project_root/out-2/other_Image0001.png",
            "node_name/out/pass_Image0001.png",
            "node_name/out-2/pass_Image0001.png",
        ] {
            if !came.in_the_jobs_folder.iter().any(|name| name == wanted) {
                faults.push(format!("{named}: no {wanted} in {:?}", came.in_the_jobs_folder));
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **What 5.3's `{project_root}` and its neighbours do in the regular output
/// path's folder** (`Job files and outputs` P7b). `{blend_dir}` is worked out, so
/// the folder is replaced like a node's and the file counted; `{project_root}`
/// stops the reading before any render, as Blender stops it (`Job files and outputs` I1), and is
/// printed, not asserted, because nothing here changes that.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2, 5.3 beta and 5.4 alpha, one at a time"]
fn an_expression_in_the_regular_output_path_lands_where() {
    let out = the_folder().join("regular-output-expressions");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.2", "5.3", "5.4"]) {
        println!("{named}:");
        for (at, folder) in ["{blend_dir}/r/f_", "{scene_name}/r/f_", "//{scene_name}/f_", "{project_root}/r/f_"].into_iter().enumerate() {
            let here = out.join(named).join(at.to_string());
            let script = format!(
                r#"{THE_SMALL_SCENE}
scene.render.filepath = {folder:?}
print("SMOOZY-DESK-MADE", bpy.app.version_string, flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#
            );
            let file = make(&blender, &script, &here, "job");
            let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1]);
            println!("    {folder}: not launched {:?}; the render ended at {:?}; in the job's folder: {:?}; beside the file: {:?}", came.not_launched, came.the_renders_status, came.in_the_jobs_folder, came.beside_the_file);
            if folder == "{project_root}/r/f_" {
                if came.the_renders_status.is_some() {
                    faults.push(format!("{named}, {folder}: it was rendered: {:?}", came.in_the_jobs_folder));
                }
            } else if folder == "//{scene_name}/f_" {
                // **Worked out from the file's own place: as it was**, and `Job files and outputs` I2 owes
                // no entry for it.
                if came.in_the_jobs_folder != ["f_0001.png"] {
                    faults.push(format!("{named}, {folder}: {:?}", came.in_the_jobs_folder));
                }
            } else {
                the_inventory_matches_the_disk(&format!("{named}, {folder}"), &came, &mut faults);
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **Writing 6 and `Rendering` L10c: a video-only scene as a first-frame check.**
#[test]
#[ignore = "run by hand at the desk with Blender 5.2, 5.3 beta and 5.4 alpha, one at a time"]
fn a_video_only_scene_through_the_first_frame_check_writes_what() {
    let out = the_folder().join("video-to-still");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.2", "5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let file = make(&blender, &a_video_only_file(), &here, "video");

        let mut alone = an_untouched_payload();
        alone.the_check_is_written_as_a_picture = true;
        let mut into_its_range_job = alone.clone();
        into_its_range_job.the_check_writes_into = Some(a_job().as_text().to_string());

        for (case, payload) in [("alone", &alone), ("writing into its range job", &into_its_range_job)] {
            let came = one_job(&blender, &file, &here.join(case), payload, vec![1]);
            println!("    {case}: the line: {}", came.the_line);
            println!(
                "    {case}: not launched: {:?}; held: {:?}; the render ended at {:?}",
                came.not_launched, came.holds, came.the_renders_status
            );
            println!("    {case}: in the job's folder: {:?}", came.in_the_jobs_folder);
            println!("    {case}: beside the file: {:?}", came.beside_the_file);
            for line in the_lines_with(&came.the_renders_output, "written_as_a_picture")
                .into_iter()
                .chain(the_lines_with(&came.the_renders_output, "Saved:"))
            {
                println!("    {case}: the render says: {}", line.chars().take(200).collect::<String>());
            }
            if came.the_renders_status != Some(0) {
                faults.push(format!("{named}, {case}: the render ended at {:?}", came.the_renders_status));
            }
            let pictures: Vec<&String> = came.in_the_jobs_folder.iter().filter(|name| name.ends_with(".png")).collect();
            if pictures.len() != 1 || came.in_the_jobs_folder.len() != 1 || !pictures[0].contains("0001") {
                faults.push(format!("{named}, {case}: wrote {:?}", came.in_the_jobs_folder));
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

// ---------------------------------------------------------------------------
// The 5.3 integration's piece 3: the compositor as a stack of effects
// ---------------------------------------------------------------------------
//
// The plan's clauses are `Job files and outputs`' stack-of-effects lines, through
// Smoozy's own reading, inventory and redirect. `SMOOZY_DESK_ONLY=53` narrows a
// run as above; the write-up is `Audits/Build/Blender 5.3/Piece 3, the
// compositor as a stack of effects — what landed.md`.

/// **What every file below shares**: one effect's group built by hand with a
/// File Output node, its folder and name handed in, fed from a Render Layers
/// node. Every name is off `Piece 1, the desk readings — what landed.md`,
/// reading 1.
const THE_EFFECTS_HEAD: &str = r#"
def an_effect(named, group_name, node_name, folder, front):
    effect = scene.compositor_effects.new(named)
    tree = bpy.data.node_groups.new(group_name, "CompositorNodeTree")
    tree.interface.new_socket("Image", in_out="OUTPUT", socket_type="NodeSocketColor")
    tree.interface.new_socket("Image", in_out="INPUT", socket_type="NodeSocketColor")
    effect.node_group = tree
    rl = tree.nodes.new("CompositorNodeRLayers")
    rl.scene = scene
    rl.layer = scene.view_layers[0].name
    fo = tree.nodes.new("CompositorNodeOutputFile")
    fo.name = node_name
    fo.directory = folder
    fo.file_name = front
    fo.format.media_type = "IMAGE"
    fo.format.file_format = "PNG"
    fo.file_output_items.clear()
    fo.file_output_items.new("RGBA", "Image")
    tree.links.new(rl.outputs[0], fo.inputs[0])
    return effect


def saved():
    print("SMOOZY-DESK-MADE", bpy.app.version_string, bpy.app.build_hash.decode(), [one.name for one in scene.compositor_effects], flush=True)
    bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

/// A file: the small scene, the head above, and the case's own lines.
fn an_effects_file(the_case: &str) -> String {
    format!("{THE_SMALL_SCENE}\n{THE_EFFECTS_HEAD}\n{the_case}\nsaved()\n")
}

/// The names the inventory owes, whole, under the job's folder.
fn what_the_inventory_owes(came: &WhatCameBack) -> Vec<(Option<i64>, String)> {
    use smoozy_job_files_machine::inventory::WhatTheInventoryIs;
    let composed = came.the_composed.as_ref().expect("a reading was composed");
    let WhatTheInventoryIs::It(inventory) = &composed.inventory else {
        panic!("no inventory: {:?}", composed.inventory);
    };
    inventory
        .entries
        .iter()
        .map(|entry| (entry.frame, String::from_utf8_lossy(&entry.predicted_path).into_owned()))
        .collect()
}

/// **Every file the job wrote is one the inventory owes, and the other way
/// about**, by the name under the job's folder. Blender's own working files
/// are not in the folder, so a stray is a fault.
fn the_inventory_matches_the_disk(named: &str, came: &WhatCameBack, faults: &mut Vec<String>) {
    let owed: Vec<String> = what_the_inventory_owes(came)
        .into_iter()
        .filter_map(|(_, path)| path.split("/job/").nth(1).map(str::to_string))
        .collect();
    let mut owed_sorted = owed.clone();
    owed_sorted.sort();
    owed_sorted.dedup();
    if owed_sorted != came.in_the_jobs_folder {
        faults.push(format!("{named}: owed {owed_sorted:?}, written {:?}", came.in_the_jobs_folder));
    }
    if !came.beside_the_file.is_empty() {
        faults.push(format!("{named}: left beside the file, outside the job: {:?}", came.beside_the_file));
    }
    if came.the_renders_status != Some(0) {
        faults.push(format!("{named}: the render ended at {:?}, not launched: {:?}, held: {:?}", came.the_renders_status, came.not_launched, came.holds));
    }
}

/// **The brief's first proof: two effects, a File Output node in each, one of
/// the nodes in the non-active effect.** Both land under the job's folder and
/// both are counted.
#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta and 5.4 alpha, one at a time"]
fn two_effects_each_with_a_node_both_land_and_are_counted() {
    let out = the_folder().join("two-effects");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let script = an_effects_file(
            r#"
an_effect("First", "Group one", "Node one", "//one", "one_")
an_effect('Sec"ond/é', "Group two", "Node two", "//two", "two_")
scene.compositor_effects.active = scene.compositor_effects[1]
"#,
        );
        let file = make(&blender, &script, &here, "job");
        let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1, 2]);
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        println!("    beside the file: {:?}", came.beside_the_file);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        the_errors_hold_nothing_of_ours(named, &came, &mut faults);
        for wanted in ["one_", "two_"] {
            if came.in_the_jobs_folder.iter().filter(|name| name.contains(wanted)).count() != 2 {
                faults.push(format!("{named}: {wanted} did not land twice under the job"));
            }
        }
        // **`Job files and outputs` P18a through the stack**: a first-frame check writing into its range
        // job's folder marks the names of both effects' nodes.
        let mut the_check = an_untouched_payload();
        the_check.the_check_writes_into = Some(a_job().as_text().to_string());
        let came = one_job(&blender, &file, &here.join("run-check"), &the_check, vec![1]);
        println!("    the check, in the job's folder: {:?}", came.in_the_jobs_folder);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        for wanted in ["check_one_", "check_two_", "check_f_"] {
            if !came.in_the_jobs_folder.iter().any(|name| name.contains(wanted)) {
                faults.push(format!("{named}: the check did not write {wanted}"));
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **No line of Smoozy's own making on the Errors stream**, the render's or the
/// reading's. Blender's own words would be left alone; these are the two kinds
/// Smoozy's scripts could put there.
fn the_errors_hold_nothing_of_ours(named: &str, came: &WhatCameBack, faults: &mut Vec<String>) {
    for (stream, said) in [("the render's", &came.the_renders_errors), ("the reading's", &came.the_readings_errors)] {
        for words in ["DeprecationWarning", "compositing_node_group", "Traceback", "smoozy:"] {
            if said.contains(words) {
                faults.push(format!("{named}: {stream} Errors hold {words}: {said}"));
            }
        }
    }
}

fn the_reading_of(came: &WhatCameBack) -> &smoozy_job_files_machine::reading::TheOutputsHalf {
    &came.the_composed.as_ref().expect("a reading was composed").reading.outputs
}

/// **The brief's second proof: an effect off for render.** Its node owes
/// nothing, writes nothing, and is promised nowhere. A second file has every
/// effect off: the raw render lands, with no walk needed.
#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta and 5.4 alpha, one at a time"]
fn an_effect_off_for_render_owes_nothing() {
    use smoozy_job_files_machine::main_output::WhatTheMainOutputIs;
    let out = the_folder().join("effect-off");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let script = an_effects_file(
            r#"
an_effect("On", "Group one", "Node on", "//on", "on_")
an_effect("Off", "Group two", "Node off", "//off", "off_").enable_for_render = False
"#,
        );
        let file = make(&blender, &script, &here, "job");
        let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1, 2]);
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        the_errors_hold_nothing_of_ours(named, &came, &mut faults);
        if came.in_the_jobs_folder.iter().any(|name| name.contains("off_")) {
            faults.push(format!("{named}: the node in the effect off wrote"));
        }
        if the_reading_of(&came).destinations.iter().any(|one| one.origin.in_words().contains("Node off")) {
            faults.push(format!("{named}: the node in the effect off is promised"));
        }

        let every_one_off = an_effects_file(
            r#"
an_effect("Off", "Group one", "Node off", "//off", "off_").enable_for_render = False
"#,
        );
        let file = make(&blender, &every_one_off, &here, "all-off");
        let came = one_job(&blender, &file, &here.join("run-all-off"), &an_untouched_payload(), vec![1]);
        println!("    every effect off, in the job's folder: {:?}", came.in_the_jobs_folder);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        let reading = the_reading_of(&came);
        let main = smoozy_job_files_machine::composing::the_main_output_of(reading);
        println!("    every effect off, the main output: {main:?}");
        if main != WhatTheMainOutputIs::TheRegularOutputPath || reading.gating.a_compositing_tree_is_present {
            faults.push(format!("{named}: every effect off does not leave the raw render as the main output: {main:?}"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **The brief's third proof: a keyed effect switch, and a driven one.** The
/// frames counted match the frames written.
#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta and 5.4 alpha, one at a time"]
fn a_keyed_effect_switch_counts_the_frames_it_writes() {
    let out = the_folder().join("effect-keyed");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let script = an_effects_file(
            r#"
scene.frame_end = 4
keyed = an_effect("Keyed", "Group one", "Node keyed", "//keyed", "keyed_")
keyed.enable_for_render = True
keyed.keyframe_insert("enable_for_render", frame=1)
keyed.enable_for_render = False
keyed.keyframe_insert("enable_for_render", frame=3)
driven = an_effect("Driven", "Group two", "Node driven", "//driven", "driven_")
curve = driven.driver_add("enable_for_render")
curve.driver.expression = "frame >= 3"
"#,
        );
        let file = make(&blender, &script, &here, "job");
        let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1, 2, 3, 4]);
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        the_errors_hold_nothing_of_ours(named, &came, &mut faults);
        let wrote = |front: &str| -> Vec<&String> { came.in_the_jobs_folder.iter().filter(|name| name.contains(front)).collect() };
        if wrote("keyed_").len() != 2 || !wrote("keyed_")[0].contains("0001") || !wrote("keyed_")[1].contains("0002") {
            faults.push(format!("{named}: the keyed node wrote {:?}", wrote("keyed_")));
        }
        if wrote("driven_").len() != 2 || !wrote("driven_")[0].contains("0003") || !wrote("driven_")[1].contains("0004") {
            faults.push(format!("{named}: the driven node wrote {:?}", wrote("driven_")));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **The brief's fourth proof: two effects with one node name in both**, in two
/// folders, and a third case for the walk's sake: a node inside a group inside
/// an effect, which the redirect reaches through the nest.
#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta and 5.4 alpha, one at a time"]
fn two_effects_with_one_node_name_do_not_collide() {
    let out = the_folder().join("one-name");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let script = an_effects_file(
            r#"
an_effect("One", "Group one", "File Output", "//one", "a_")
an_effect("Two [x] é", "Group two", "File Output", "//two", "b_")
outer = an_effect("Three", "Group three", "Unused", "//unused", "c_").node_group
outer.nodes.remove(outer.nodes["Unused"])
inner = bpy.data.node_groups.new("Inner", "CompositorNodeTree")
inner.interface.new_socket("Image", in_out="INPUT", socket_type="NodeSocketColor")
gi = inner.nodes.new("NodeGroupInput")
fo = inner.nodes.new("CompositorNodeOutputFile")
fo.name = "Inside"
fo.directory = "//nested"
fo.file_name = "n_"
fo.format.media_type = "IMAGE"
fo.format.file_format = "PNG"
fo.file_output_items.clear()
fo.file_output_items.new("RGBA", "Image")
inner.links.new(gi.outputs[0], fo.inputs[0])
holder = outer.nodes.new("CompositorNodeGroup")
holder.node_tree = inner
outer.links.new(outer.nodes["Render Layers"].outputs[0], holder.inputs[0])
"#,
        );
        let file = make(&blender, &script, &here, "job");
        let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1]);
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        the_errors_hold_nothing_of_ours(named, &came, &mut faults);
        for wanted in ["one/a_", "two/b_", "nested/n_"] {
            if !came.in_the_jobs_folder.iter().any(|name| name.contains(wanted)) {
                faults.push(format!("{named}: {wanted} did not land under the job"));
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **The brief's fifth proof: a compositor as 5.3's New button makes it**, by
/// the operator behind the button. A main output, the right *show outputs*
/// folder, the right preview, and the view layer named. A second file feeds a
/// File Output node from that Group Input, and it is the main output; a
/// third chains a second effect behind it.
#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta and 5.4 alpha, one at a time"]
fn the_tree_the_new_button_makes_has_a_main_output() {
    use smoozy_job_files_machine::composing::the_main_output_of;
    use smoozy_job_files_machine::main_output::{WhatTheMainOutputIs, WhatTheWalkFound};
    let out = the_folder().join("new-button");
    let mut faults = Vec::new();
    const THE_NEW_BUTTON: &str = r#"
bpy.ops.scene.new_compositor_effect_node_group()
effect = scene.compositor_effects[0]
tree = effect.node_group
print("SMOOZY-DESK-NEW", effect.name, tree.name, [node.bl_idname for node in tree.nodes], [one.name for one in tree.nodes["Group Input"].outputs], flush=True)
"#;
    for (named, blender) in the_blenders(&["5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);

        let file = make(&blender, &an_effects_file(THE_NEW_BUTTON), &here, "plain");
        let came = one_job(&blender, &file, &here.join("run-plain"), &an_untouched_payload(), vec![1]);
        println!("    plain, in the job's folder: {:?}", came.in_the_jobs_folder);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        the_errors_hold_nothing_of_ours(named, &came, &mut faults);
        let reading = the_reading_of(&came);
        println!("    plain, the walk: {:?}", reading.the_beauty_walk);
        if the_main_output_of(reading) != WhatTheMainOutputIs::TheRegularOutputPath {
            faults.push(format!("{named}: the plain tree's main output is {:?}", the_main_output_of(reading)));
        }

        let fed = format!(
            r#"{THE_NEW_BUTTON}
fo = tree.nodes.new("CompositorNodeOutputFile")
fo.name = "Fed"
fo.directory = "//fed"
fo.file_name = "fed_"
fo.format.media_type = "IMAGE"
fo.format.file_format = "PNG"
fo.file_output_items.clear()
fo.file_output_items.new("RGBA", "Image")
tree.links.new(tree.nodes["Group Input"].outputs["Image"], fo.inputs[0])
"#
        );
        let file = make(&blender, &an_effects_file(&fed), &here, "fed");
        let came = one_job(&blender, &file, &here.join("run-fed"), &an_untouched_payload(), vec![1]);
        println!("    fed, in the job's folder: {:?}", came.in_the_jobs_folder);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        the_errors_hold_nothing_of_ours(named, &came, &mut faults);
        let composed = came.the_composed.as_ref().expect("composed");
        let reading = &composed.reading.outputs;
        println!("    fed, the walk: {:?}", reading.the_beauty_walk);
        println!("    fed, the pick: {:?}", composed.picks.the_main_output);
        if the_main_output_of(reading) != WhatTheMainOutputIs::ADestinationInsideAFileOutputNode {
            faults.push(format!("{named}: the fed node is not the main output"));
        }
        let the_fed_one = reading.destinations.iter().find(|one| one.origin.in_words().contains("Fed")).expect("the node's destination");
        let layers: Vec<String> = the_fed_one.the_view_layers_that_feed_it.iter().map(|one| String::from_utf8_lossy(one.as_bytes()).into_owned()).collect();
        println!("    fed, the layers feeding it: {layers:?}");
        if layers != ["ViewLayer"] {
            faults.push(format!("{named}: the layer named is {layers:?}"));
        }
        if !composed.picks.the_main_output.as_ref().is_some_and(|main| main.the_effect.is_some()) {
            faults.push(format!("{named}: the pick names no effect"));
        }
        // **The preview's rung, off the same reading.** The fed file is the
        // beauty rung, and what *show outputs* opens is its folder.
        let made = smoozy_job_files_machine::derivatives::what_one_frame_is_made_of(
            1,
            reading,
            &composed.inventory,
            &composed.picks,
            &[],
            &[smoozy_job_files_machine::reading::ArtistsText::of(b"ViewLayer".to_vec())],
            &[1],
            false,
            &|path: &[u8]| std::fs::metadata(std::ffi::OsStr::new(std::str::from_utf8(path).expect("utf-8"))).ok().map(|one| one.len()),
        )
        .expect("a frame is made of something");
        println!("    fed, the preview's source: {}", String::from_utf8_lossy(&made.the_source.file.path));
        if !String::from_utf8_lossy(&made.the_source.file.path).contains("/fed/") {
            faults.push(format!("{named}: the preview did not take the fed file"));
        }

        let chained = format!(
            r#"{THE_NEW_BUTTON}
second = scene.compositor_effects.new("Second")
group = bpy.data.node_groups.new("Group two", "CompositorNodeTree")
group.interface.new_socket("Image", in_out="OUTPUT", socket_type="NodeSocketColor")
group.interface.new_socket("Image", in_out="INPUT", socket_type="NodeSocketColor")
second.node_group = group
gi = group.nodes.new("NodeGroupInput")
go = group.nodes.new("NodeGroupOutput")
invert = group.nodes.new("CompositorNodeInvert")
group.links.new(gi.outputs[0], invert.inputs["Color"])
group.links.new(invert.outputs[0], go.inputs[0])
"#
        );
        let file = make(&blender, &an_effects_file(&chained), &here, "chained");
        let came = one_job(&blender, &file, &here.join("run-chained"), &an_untouched_payload(), vec![1]);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        let reading = the_reading_of(&came);
        println!("    chained, the walk: {:?}", reading.the_beauty_walk);
        // The chain's second effect holds a node the reading cannot see through
        // or can: either way the walk says so, and no node is invented.
        if matches!(reading.the_beauty_walk, WhatTheWalkFound::NoEntry) {
            faults.push(format!("{named}: the chained tree has no entry"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// A derivative made for real, by the job's own script, with what it reads
/// back: the copy's mean level, 0 to 255, over its three colour channels.
fn a_copys_mean_level(blender: &Path, here: &Path, name: &str, source: &[u8], view: &[u8], picture: &[u8], composited: Option<&[u8]>) -> f64 {
    use smoozy_job_files_machine::derivatives::{
        the_derivatives_script, the_size_in, the_size_told, the_words_of_a_derivative, ADerivativeToMake, TheTreatment,
    };
    let staged = here.join(format!("{name}.png"));
    let script = the_derivatives_script(&ADerivativeToMake {
        source: source.to_vec(),
        staged_at: staged.display().to_string(),
        treatment: TheTreatment::TheJobsOwnViewTransform(view.to_vec()),
        the_picture_inside: Some(picture.to_vec()),
        the_composited_picture: composited.map(<[u8]>::to_vec),
    });
    let at = here.join(format!("{name}.py"));
    std::fs::write(&at, script).expect("written");
    let words: Vec<String> = the_words_of_a_derivative(at.to_str().expect("utf-8"))
        .into_iter()
        .map(|word| String::from_utf8(word).expect("utf-8"))
        .collect();
    let mut child = std::process::Command::new(blender)
        .args(&words)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("it starts");
    let mut stdout = child.stdout.take().expect("its stream");
    let mut heard = Vec::new();
    let mut piece = [0u8; 4096];
    let mut stdin = child.stdin.take().expect("its stream");
    loop {
        let got = std::io::Read::read(&mut stdout, &mut piece).expect("it reads");
        if got == 0 {
            break;
        }
        heard.extend_from_slice(&piece[..got]);
        if let Some((width, height)) = the_size_in(&heard) {
            std::io::Write::write_all(&mut stdin, the_size_told(smoozy_job_files_machine::main_output::the_target_size(width, height)).as_bytes()).expect("told");
            std::io::Write::write_all(&mut stdin, b"\n").expect("told");
            break;
        }
    }
    std::io::Read::read_to_end(&mut stdout, &mut heard).ok();
    let status = wait_for(&mut child, "the derivative");
    assert_eq!(status, Some(0), "{name}: {}", String::from_utf8_lossy(&heard));
    let reader = here.join(format!("{name}-read.py"));
    std::fs::write(
        &reader,
        format!(
            "import bpy\nimage = bpy.data.images.load({:?})\nimage.colorspace_settings.name = 'Non-Color'\npixels = list(image.pixels)\nmean = sum(one for at, one in enumerate(pixels) if at % 4 != 3) * 4 / 3 / len(pixels) * 255\nprint('SMOOZY-MEAN', mean)\n",
            staged.display().to_string()
        ),
    )
    .expect("written");
    let (_, said, _) = run(blender, &["-b".to_string(), "--factory-startup".to_string(), "--python".to_string(), reader.display().to_string()]);
    let said = String::from_utf8_lossy(&said).to_string();
    the_lines_with(&said, "SMOOZY-MEAN")
        .first()
        .and_then(|line| line.split(' ').nth(1))
        .and_then(|figure| figure.parse().ok())
        .unwrap_or_else(|| panic!("{name}: no mean in {said}"))
}

/// **The brief's sixth proof: a multi-layer file with a grade.** The main
/// preview holds the graded picture, and another layer's copy stays as it is.
#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta and 5.4 alpha, one at a time"]
fn a_multi_layer_files_main_preview_holds_the_grade() {
    use smoozy_job_files_machine::derivatives::THE_COMPOSITED_PICTURES_NAME;
    let out = the_folder().join("the-grade");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let script = an_effects_file(
            r#"
scene.render.filepath = "//render/f_"
scene.render.image_settings.media_type = "MULTI_LAYER_IMAGE"
scene.render.image_settings.file_format = "OPEN_EXR_MULTILAYER"
scene.world.use_nodes = True
scene.world.node_tree.nodes["Background"].inputs[0].default_value = (0.02, 0.02, 0.02, 1)
scene.view_settings.view_transform = "Standard"
bpy.ops.scene.new_compositor_effect_node_group()
tree = scene.compositor_effects[0].node_group
invert = tree.nodes.new("CompositorNodeInvert")
tree.links.remove(tree.links[0])
tree.links.new(tree.nodes["Group Input"].outputs[0], invert.inputs["Color"])
tree.links.new(invert.outputs[0], tree.nodes["Group Output"].inputs[0])
"#,
        );
        let file = make(&blender, &script, &here, "job");
        let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1]);
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        the_errors_hold_nothing_of_ours(named, &came, &mut faults);
        let composed = came.the_composed.as_ref().expect("composed");
        let reading = &composed.reading.outputs;
        let made = smoozy_job_files_machine::derivatives::what_one_frame_is_made_of(
            1,
            reading,
            &composed.inventory,
            &composed.picks,
            &[],
            &[smoozy_job_files_machine::reading::ArtistsText::of(b"ViewLayer".to_vec())],
            &[1],
            false,
            &|path: &[u8]| std::fs::metadata(std::ffi::OsStr::new(std::str::from_utf8(path).expect("utf-8"))).ok().map(|one| one.len()),
        )
        .expect("a frame is made of something");
        println!("    the main copy tries {:?} then {:?}", made.the_composited_picture, made.the_picture_inside);
        if made.the_composited_picture.as_ref().map(|one| one.as_bytes()) != Some(THE_COMPOSITED_PICTURES_NAME) {
            faults.push(format!("{named}: the main copy does not try the composited picture"));
        }
        let source = made.the_source.file.path.clone();
        let picture = made.the_picture_inside.as_ref().expect("the multi-layer route").as_bytes().to_vec();
        let view = made.treatment.the_view_transform().to_vec();
        let copies = here.join("copies");
        std::fs::create_dir_all(&copies).expect("a folder");
        let graded = a_copys_mean_level(&blender, &copies, "graded", &source, &view, &picture, Some(THE_COMPOSITED_PICTURES_NAME));
        let plain = a_copys_mean_level(&blender, &copies, "plain", &source, &view, &picture, None);
        let fell_back = a_copys_mean_level(&blender, &copies, "fell-back", &source, &view, &picture, Some(b"No such picture"));
        println!("    mean level: the main copy {graded:.1}, the layer's own {plain:.1}, a name the file lacks {fell_back:.1}");
        if graded <= plain + 50.0 {
            faults.push(format!("{named}: the main copy does not hold the invert: {graded} against {plain}"));
        }
        if (fell_back - plain).abs() > 1.0 {
            faults.push(format!("{named}: a name the file lacks did not fall back: {fell_back} against {plain}"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **The brief's last proof: a file saved by 5.2.3 with a tree, opened in 5.3b
/// and 5.4a**, comes through correctly; and on 5.2.3 itself the reading holds
/// none of the new words.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2, 5.3 beta and 5.4 alpha, one at a time"]
fn a_file_saved_by_five_two_comes_through() {
    let out = the_folder().join("from-five-two");
    let mut faults = Vec::new();
    let Some((_, five_two)) = the_blenders(&["5.2"]).into_iter().next() else {
        println!("no 5.2 in this run");
        return;
    };
    let file = make(&five_two, &a_file_with_a_file_output_node("//passes"), &out, "job");
    for (named, blender) in the_blenders(&["5.2", "5.3", "5.4"]) {
        println!("{named}:");
        let came = one_job(&blender, &file, &out.join(named), &an_untouched_payload(), vec![1, 2]);
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        the_inventory_matches_the_disk(named, &came, &mut faults);
        the_errors_hold_nothing_of_ours(named, &came, &mut faults);
        let reading = the_reading_of(&came);
        let names_an_effect = reading.destinations.iter().any(|one| one.the_effect().is_some());
        let the_json = serde_json::to_string(&came.the_composed.as_ref().expect("composed").reading).expect("it writes out");
        println!("    names an effect: {names_an_effect}; the walk: {:?}", reading.the_beauty_walk);
        if named == "5.2" {
            if names_an_effect || the_json.contains("the_effect") {
                faults.push(format!("{named}: the control's reading names an effect"));
            }
        } else if !names_an_effect {
            faults.push(format!("{named}: the opened file's node names no effect"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **A file set to GPU under Cycles and then switched to EEVEE**, the common way
/// to `Rendering` L21's bullet of 2026-11-02.
/// **A job's scene holding one Scene strip of another scene**, each with its
/// own recorded output path, the job's relative and the strip scene's in a
/// folder of its own at the top of this run's folder, and a File Output node in
/// the strip scene. `the_output` is what the job's scene writes.
fn a_file_with_a_scene_strip(the_output: &str, the_strip_scenes_folder: &str) -> String {
    format!(
        r#"{THE_SMALL_SCENE}
shot = bpy.data.scenes.new("Shot")
shot.render.engine = "CYCLES"
shot.cycles.device = "CPU"
shot.cycles.samples = 2
shot.cycles.use_adaptive_sampling = False
shot.cycles.use_denoising = False
shot.render.resolution_x = 64
shot.render.resolution_y = 64
shot.render.filepath = {the_strip_scenes_folder:?}
shot.render.image_settings.file_format = "PNG"
shot.frame_start = 1
shot.frame_end = 2
camera = bpy.data.objects["Camera"]
shot.collection.objects.link(camera)
shot.camera = camera
{the_output}
editor = scene.sequence_editor_create()
strips = editor.strips if hasattr(editor, "strips") else editor.sequences
strip = strips.new_scene("to_shot", shot, 1, 1)
strip.frame_final_duration = 2
scene.render.use_sequencer = True
print("SMOOZY-DESK-MADE", bpy.app.version_string, bpy.app.build_hash.decode(), scene.render.filepath, shot.render.filepath, flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#
    )
}

/// **`Job files and outputs` R4 item 3 as corrected and `Job files and outputs` D10a, through the
/// worker's own driver: a job from a scene with strips writes where the job
/// says, a video and pictures alike, and the strip scene's own recorded folder
/// gets nothing, in every version.** Each case's inventory is held to the disk,
/// so the count finds what was written, which is what `Rendering` K14b reads.
/// **What this cannot show**: a card, or Linux.
#[test]
#[ignore = "run by hand at the desk with every Blender, one at a time"]
fn a_scene_with_strips_writes_where_the_job_says() {
    let out = the_folder().join("scene-strips");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["4.5", "5.0", "5.1", "5.2", "5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        std::fs::create_dir_all(&here).expect("a folder");
        let unused = here.join("the-strip-scenes-own-folder");
        let the_strip_scenes_path = format!("{}/s_", unused.display());
        let video = if named == "4.5" {
            r#"scene.render.image_settings.file_format = "FFMPEG"
scene.render.ffmpeg.format = "MPEG4"
scene.render.ffmpeg.codec = "H264"
scene.render.filepath = "//board/v_""#
        } else {
            r#"scene.render.image_settings.media_type = "VIDEO"
scene.render.image_settings.file_format = "FFMPEG"
scene.render.ffmpeg.format = "MPEG4"
scene.render.ffmpeg.codec = "H264"
scene.render.filepath = "//board/v_""#
        };
        for (case, the_output, frames) in [
            ("pictures", r#"scene.render.filepath = "//board/p_""#, vec![1, 2]),
            ("a video", video, vec![1, 2]),
        ] {
            let file = make(
                &blender,
                &a_file_with_a_scene_strip(the_output, &the_strip_scenes_path),
                &here.join(case),
                "strips",
            );
            let came = one_job(&blender, &file, &here.join(case).join("run"), &an_untouched_payload(), frames);
            println!("    {case}: the line: {}", came.the_line);
            println!("    {case}: in the job's folder: {:?}", came.in_the_jobs_folder);
            for line in the_lines_with(&came.the_renders_output, "Saved:")
                .into_iter()
                .chain(the_lines_with(&came.the_renders_output, "Append frame"))
            {
                println!("    {case}: the render says: {}", line.chars().take(200).collect::<String>());
            }
            let reading = the_reading_of(&came);
            let regulars = reading
                .destinations
                .iter()
                .filter(|one| one.kind == smoozy_job_files_machine::reading::DestinationKind::TheRegularOutputPath)
                .count();
            if regulars != 1 {
                faults.push(format!("{named}, {case}: {regulars} regular output paths in the reading"));
            }
            if unused.exists() {
                faults.push(format!("{named}, {case}: the strip scene's own folder was written"));
            }
            if came.in_the_jobs_folder.is_empty() {
                faults.push(format!("{named}, {case}: nothing written"));
            }
            the_inventory_matches_the_disk(&format!("{named}, {case}"), &came, &mut faults);
        }
        // **What a strip's scene really writes through a File Output node keeps
        // its place**, from 5.0 by the node's own names. The inventory is
        // withheld there (`Job files and outputs` I4, a frame number the node
        // does not stamp), so this reads the disk and the reading.
        if named != "4.5" {
            let a_node = r#"scene.render.filepath = "//board/p_"
tree = bpy.data.node_groups.new("Compositor", "CompositorNodeTree")
shot.compositing_node_group = tree
shot.render.use_compositing = True
rl = tree.nodes.new("CompositorNodeRLayers")
rl.scene = shot
rl.layer = shot.view_layers[0].name
fo = tree.nodes.new("CompositorNodeOutputFile")
fo.name = "File Output"
fo.directory = "//shot_nodes"
fo.file_name = "pass_"
fo.format.media_type = "IMAGE"
fo.format.file_format = "PNG"
fo.file_output_items.clear()
fo.file_output_items.new("RGBA", "Image")
tree.links.new(rl.outputs[0], fo.inputs[0])"#;
            let case = "a node";
            let file = make(&blender, &a_file_with_a_scene_strip(a_node, &the_strip_scenes_path), &here.join(case), "strips");
            let came = one_job(&blender, &file, &here.join(case).join("run"), &an_untouched_payload(), vec![1, 2]);
            println!("    {case}: in the job's folder: {:?}", came.in_the_jobs_folder);
            let the_nodes_in_the_strips_scene = the_reading_of(&came)
                .destinations
                .iter()
                .filter(|one| {
                    matches!(one.scene, smoozy_job_files_machine::reading::WhichScene::ReachedThroughASequencerStrip { .. })
                        && one.is_one_the_render_asked_for()
                })
                .count();
            if the_nodes_in_the_strips_scene != 1 {
                faults.push(format!("{named}, {case}: {the_nodes_in_the_strips_scene} destinations in the strip's scene"));
            }
            if !came.in_the_jobs_folder.iter().any(|name| name.starts_with("shot_nodes/"))
                || !came.in_the_jobs_folder.iter().any(|name| name.starts_with("board/p_0001"))
            {
                faults.push(format!("{named}, {case}: wrote {:?}", came.in_the_jobs_folder));
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **The real file the live pass found this on**, given by
/// `SMOOZY_DESK_REAL_FILE` (`cowboi_storytools.blend`, CC-BY): its scene *Edit*,
/// nine Scene strips, a video on the file's own recorded path. **Run on the
/// Mac's 5.3 Beta and never changed**: `one_job` copies it first.
#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta and the real file"]
fn the_storyboarding_file_writes_its_video_where_the_count_looks() {
    let Ok(real) = std::env::var("SMOOZY_DESK_REAL_FILE") else {
        panic!("SMOOZY_DESK_REAL_FILE names the file");
    };
    let out = the_folder().join("the-real-file");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.3"]) {
        let frames: Vec<i64> = (1..=24).collect();
        let came = one_job_owing(
            &blender,
            Path::new(&real),
            &out.join(named),
            "Edit",
            &an_untouched_payload(),
            frames,
            Vec::new(),
            None,
        );
        println!("{named}: the line: {}", came.the_line);
        println!("not launched: {:?}, held: {:?}, ended at: {:?}", came.not_launched, came.holds, came.the_renders_status);
        println!("in the job's folder: {:?}", came.in_the_jobs_folder);
        println!("the inventory owes: {:?}", what_the_inventory_owes(&came));
        for destination in &the_reading_of(&came).destinations {
            if destination.is_one_the_render_asked_for() {
                println!("    a destination: {:?} {:?}", destination.scene, destination.recorded_location);
            }
        }
        the_inventory_matches_the_disk(named, &came, &mut faults);
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

fn an_eevee_file_saved_with_the_cycles_device_on_gpu() -> String {
    format!(
        r#"{THE_SMALL_SCENE}
scene.cycles.device = "GPU"
scene.render.engine = "BLENDER_EEVEE"
scene.eevee.taa_render_samples = 4
print("SMOOZY-DESK-MADE", bpy.app.version_string, bpy.app.build_hash.decode(), scene.render.engine, scene.cycles.device, flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#
    )
}

/// **The same file left on Cycles**, the control: it still names the technology.
fn a_cycles_file_saved_on_gpu() -> String {
    format!(
        r#"{THE_SMALL_SCENE}
scene.cycles.device = "GPU"
print("SMOOZY-DESK-MADE", bpy.app.version_string, bpy.app.build_hash.decode(), scene.render.engine, scene.cycles.device, flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#
    )
}

/// **`Rendering` L21's bullet of 2026-11-02 and L24b, through Smoozy's own
/// composition, on a machine with no OptiX library and a manifest saying
/// NVIDIA.** An EEVEE file saved with the Cycles device on GPU, and a Cycles file
/// on GPU with the user's EEVEE pick, each launch with no device argument and
/// write their picture. The control, the Cycles file untouched, ends at L24b
/// before anything is launched, so the case is live. **What this cannot show**:
/// a card or a driver, this being a Mac with no NVIDIA card.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2 and 5.3 beta, one at a time"]
fn an_eevee_job_saved_on_gpu_renders_with_no_technology_named() {
    use smoozy_render_settings_machine::engine::{EngineEntry, OfferedEngine};
    let out = the_folder().join("eevee-saved-on-gpu");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["5.2", "5.3"]) {
        println!("{named}:");
        let here = out.join(named);
        let eevee = make(&blender, &an_eevee_file_saved_with_the_cycles_device_on_gpu(), &here, "eevee");
        let cycles = make(&blender, &a_cycles_file_saved_on_gpu(), &here, "cycles");
        let mut picked = an_untouched_payload();
        picked.engine = Some(EngineEntry { pick: OfferedEngine::Eevee, identifier: None });
        for (what, folder, file, payload) in [
            ("the file's own EEVEE", "own", &eevee, an_untouched_payload()),
            ("the user's EEVEE pick", "picked", &cycles, picked),
        ] {
            let came = one_job(&blender, file, &here.join(folder), &payload, vec![1]);
            println!("    {what}: the line: {}", came.the_line);
            println!("    not launched: {:?}; the render ended at {:?}", came.not_launched, came.the_renders_status);
            println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
            if came.the_renders_status != Some(0) {
                faults.push(format!("{named}, {what}: not launched {:?}, ended {:?}", came.not_launched, came.the_renders_status));
            }
            if came.the_line.contains("OPTIX") || came.the_line.contains(" -- ") {
                faults.push(format!("{named}, {what}: a device argument: {}", came.the_line));
            }
            if !came.in_the_jobs_folder.iter().any(|name| name.ends_with(".png")) {
                faults.push(format!("{named}, {what}: no picture in {:?}", came.in_the_jobs_folder));
            }
            the_inventory_matches_the_disk(&format!("{named}, {what}"), &came, &mut faults);
        }
        let control = one_job(&blender, &cycles, &here.join("control"), &an_untouched_payload(), vec![1]);
        println!("    the control, Cycles: not launched: {:?}", control.not_launched);
        if !control.not_launched.as_deref().is_some_and(|ended| ended.contains("TheOptixLibraryWasNotFound")) {
            faults.push(format!("{named}, the control: {:?}", control.not_launched));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **The largest `Rendering n / m samples` EEVEE printed**, n and its m.
fn the_most_samples_said(output: &str) -> Option<(u64, u64)> {
    output
        .lines()
        .filter_map(|line| {
            let after = &line[line.find("Rendering ")? + "Rendering ".len()..];
            let (done, of) = after.strip_suffix(" samples").unwrap_or(after).split_once(" / ")?;
            Some((done.trim().parse().ok()?, of.trim().parse().ok()?))
        })
        .max()
}

/// **An EEVEE file saved with a time limit**, where the version has one, large
/// enough that the typed count takes longer than the limit on this Mac: 5.3's
/// comparison stopped at 275 of 4,096 in two seconds at this size.
fn an_eevee_file_with_a_time_limit() -> String {
    format!(
        r#"{THE_SMALL_SCENE}
scene.render.engine = "BLENDER_EEVEE"
scene.eevee.taa_render_samples = 4096
scene.render.resolution_x = 400
scene.render.resolution_y = 400
if hasattr(scene.eevee, "time_limit"):
    scene.eevee.time_limit = 1.0
print("SMOOZY-DESK-MADE", bpy.app.version_string, bpy.app.build_hash.decode(), scene.render.engine, getattr(scene.eevee, "time_limit", "no limit here"), flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#
    )
}

/// **`Render Settings` C1 item 3, through Smoozy's own composition, with a
/// control on either side of it.** On 5.3 and 5.4, the count alone leaves the
/// file's limit cutting the frame short, and Smoozy's job renders the typed
/// number. On 5.2.3, which has no such setting, the older version still runs.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2, 5.3 beta and 5.4 alpha, one at a time"]
fn a_typed_count_under_eevee_outlasts_the_time_limit_the_file_was_saved_with() {
    const TYPED: u64 = 600;
    let out = the_folder().join("eevee-time-limit");
    for (named, blender) in the_blenders(&["5.2", "5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let file = make(&blender, &an_eevee_file_with_a_time_limit(), &here, "job");

        // **The fault, without Smoozy**: the count alone, as the set was before.
        let began = Instant::now();
        let (status, said, _) = run(
            &blender,
            &[
                "--factory-startup".to_string(),
                "--background".to_string(),
                file.display().to_string(),
                "--python-expr".to_string(),
                format!("import bpy; bpy.context.scene.eevee.taa_render_samples = {TYPED}"),
                "--render-output".to_string(),
                here.join("plain/f_").display().to_string(),
                "--render-frame".to_string(),
                "1".to_string(),
            ],
        );
        let plain = the_most_samples_said(&String::from_utf8_lossy(&said));
        println!("    the count alone: ended at {status:?} after {:?}, {plain:?}", began.elapsed());

        let mut payload = an_untouched_payload();
        payload.sample_count = Some(TYPED);
        let began = Instant::now();
        let came = one_job(&blender, &file, &here.join("run"), &payload, vec![1]);
        let through_smoozy = the_most_samples_said(&came.the_renders_output);
        // **`Render Settings` Q2 item 1: the reading takes EEVEE's limit from 5.3.**
        assert_eq!(
            came.the_readings_output.contains("\"eevee_time_limit\": 1.0"),
            named != "5.2",
            "{named}: the reading of EEVEE's limit"
        );
        println!(
            "    through Smoozy: not launched {:?}, held {:?}, ended at {:?} after {:?}, {through_smoozy:?}, errors {:?}",
            came.not_launched,
            came.holds,
            came.the_renders_status,
            began.elapsed(),
            came.the_renders_errors.lines().filter(|line| !line.contains("DeprecationWarning") && !line.contains("getattr")).collect::<Vec<_>>(),
        );
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        assert_eq!(came.not_launched, None, "{named} composes and launches");
        assert_eq!(came.the_renders_status, Some(0), "{named} ends well");
        assert_eq!(through_smoozy, Some((TYPED, TYPED)), "{named} renders the typed number");
        if named != "5.2" {
            let (done, of) = plain.expect("a count was printed");
            assert_eq!(of, TYPED);
            assert!(done < TYPED, "{named}'s limit stands with the count alone, so this proves nothing");
        }
    }
}

/// **A bake folder as a real bake makes it**: a Geometry Nodes Bake node on the
/// factory cube, baked to disk, and then **cut off from its source**, so the
/// cube is in the picture only if the bake is read. Printed: every folder the
/// bake made.
pub(super) fn a_file_baked_into(folder: &str) -> String {
    format!(
        r#"{THE_SMALL_SCENE}
import os
obj = bpy.data.objects["Cube"]
tree = bpy.data.node_groups.new("G", "GeometryNodeTree")
tree.interface.new_socket("Geometry", in_out="INPUT", socket_type="NodeSocketGeometry")
tree.interface.new_socket("Geometry", in_out="OUTPUT", socket_type="NodeSocketGeometry")
source = tree.nodes.new("NodeGroupInput")
end = tree.nodes.new("NodeGroupOutput")
keep = tree.nodes.new("GeometryNodeBake")
keep.bake_items.new("GEOMETRY", "Geometry")
tree.links.new(source.outputs[0], keep.inputs[0])
tree.links.new(keep.outputs[0], end.inputs[0])
modifier = obj.modifiers.new("GN", "NODES")
modifier.node_group = tree
modifier.bake_target = "DISK"
modifier.bake_directory = {folder:?}
bpy.ops.wm.save_as_mainfile(filepath=out)
bake = modifier.bakes[0]
bake.bake_mode = "ANIMATION"
print("SMOOZY-DESK-BAKED", bpy.ops.object.geometry_node_bake_single(session_uid=obj.session_uid, modifier_name=modifier.name, bake_id=bake.bake_id), flush=True)
for link in list(tree.links):
    if link.to_node == keep:
        tree.links.remove(link)
print("SMOOZY-DESK-MADE", bpy.app.version_string, bpy.app.build_hash.decode(), repr(modifier.bake_directory), modifier.bake_target, flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#
    )
}

/// **How many of a picture's pixels are not its first pixel's colour**, read by
/// the Blender under test: the factory world is one flat colour, so anything
/// over none is the cube.
pub(super) fn how_much_of_the_picture_is_not_background(blender: &Path, picture: &Path, here: &Path) -> usize {
    let looker = here.join("look.py");
    std::fs::write(
        &looker,
        r#"
import bpy, sys
picture = bpy.data.images.load(sys.argv[sys.argv.index("--") + 1])
pixels = list(picture.pixels)
first = pixels[:3]
count = 0
for at in range(0, len(pixels), 4):
    if max(abs(pixels[at + i] - first[i]) for i in range(3)) > 0.02:
        count += 1
print("SMOOZY-DESK-NOT-BACKGROUND", count, flush=True)
"#,
    )
    .expect("written");
    let (_, said, _) = run(
        blender,
        &[
            "--factory-startup".to_string(),
            "--background".to_string(),
            "--python".to_string(),
            looker.display().to_string(),
            "--".to_string(),
            picture.display().to_string(),
        ],
    );
    let said = String::from_utf8_lossy(&said);
    said.lines()
        .find_map(|line| line.strip_prefix("SMOOZY-DESK-NOT-BACKGROUND "))
        .expect("a count")
        .trim()
        .parse()
        .expect("a number")
}

/// **`Dependency Upload & Remap` G9a's render half.** A bake made under
/// `//bakes/{blend_name}/`, through the driver: uploaded beside the `.blend` and
/// read where it is (`Job files and outputs` D1a), sent and copied into the job's
/// folder (`Dependency Upload & Remap` M22a), and absent, which is the control that the picture turns on the
/// bake. **On 5.2.3 the same words make a folder really called `{blend_name}`**;
/// and on 5.3 and 5.4 a variable that fell back to packing comes with the file.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2, 5.3 beta and 5.4 alpha, one at a time"]
fn a_bake_folders_template_lands_where_the_render_looks() {
    use smoozy_dependencies_machine::bytes::{MachineLocation, RecordedLocation};
    use smoozy_dependencies_machine::placed_copies::TheCopyTheMachineOwes;
    use smoozy_job_files_machine::reading::AnAddedWritePlace;

    const THE_FOLDER: &str = "//bakes/{blend_name}/";
    let out = the_folder().join("bake-template");
    for (named, blender) in the_blenders(&["5.2", "5.3", "5.4"]) {
        println!("{named}:");
        let here = out.join(named);
        let file = make(&blender, &a_file_baked_into(THE_FOLDER), &here, "t");
        let folder_on_disk = if named == "5.2" { "{blend_name}" } else { "t" };
        let the_bake = here.join("bakes").join(folder_on_disk);
        assert!(the_bake.is_dir(), "{named} baked into {}: {:?}", the_bake.display(), every_file_under(&here.join("bakes")));
        println!("    the bake: {:?}", every_file_under(&the_bake));
        let the_cube = |came: &WhatCameBack, what: &str| -> usize {
            println!("    {what}: not launched {:?}, held {:?}, ended at {:?}", came.not_launched, came.holds, came.the_renders_status);
            println!("    {what}: in the job's folder {:?}", came.in_the_jobs_folder);
            assert_eq!(came.not_launched, None, "{named}, {what}");
            assert_eq!(came.the_renders_status, Some(0), "{named}, {what}");
            let frame = came
                .in_the_jobs_folder
                .iter()
                .find(|name| name.ends_with("f_0001.png"))
                .unwrap_or_else(|| panic!("{named}, {what}: no frame"));
            let job = here.join(what).join("job").join(frame);
            let cube = how_much_of_the_picture_is_not_background(&blender, &job, &here);
            println!("    {what}: {cube} pixels of cube");
            cube
        };

        let came = one_job(&blender, &file, &here.join("absent"), &an_untouched_payload(), vec![1]);
        assert_eq!(the_cube(&came, "absent"), 0, "{named}: the picture turns on the bake");

        let beside = format!("bakes/{folder_on_disk}");
        let came = one_job_owing(
            &blender,
            &file,
            &here.join("beside"),
            "Scene",
            &an_untouched_payload(),
            vec![1],
            Vec::new(),
            Some((&the_bake, &beside)),
        );
        assert!(the_cube(&came, "beside") > 0, "{named}: uploaded beside the file, read where it is");

        let copy = TheCopyTheMachineOwes {
            from: MachineLocation::composed(the_bake.display().to_string().into_bytes()),
            place: AnAddedWritePlace::TheGeometryNodesBakeFolder,
            recorded: RecordedLocation::as_the_reading_carried_it(THE_FOLDER.as_bytes()),
        };
        let came = one_job_owing(
            &blender,
            &file,
            &here.join("sent"),
            "Scene",
            &an_untouched_payload(),
            vec![1],
            vec![copy],
            None,
        );
        assert!(the_cube(&came, "sent") > 0, "{named}: sent, and copied where the file is pointed");
        assert!(
            came.in_the_jobs_folder.iter().all(|name| !name.contains('{')),
            "{named}: no folder under the job's holds an expression: {:?}",
            came.in_the_jobs_folder
        );

        if named != "5.2" {
            let packed = here.join("packed");
            let file = make(&blender, &a_file_baked_into("//bakes/{scene_name}/"), &packed, "two");
            assert!(!packed.join("bakes").exists(), "{named} made a folder after all");
            let came = one_job(&blender, &file, &packed.join("run"), &an_untouched_payload(), vec![1]);
            println!("    packed: ended at {:?}, {:?}", came.the_renders_status, came.in_the_jobs_folder);
            assert_eq!(came.the_renders_status, Some(0));
            let frame = came.in_the_jobs_folder.iter().find(|name| name.ends_with("f_0001.png")).expect("a frame");
            let cube = how_much_of_the_picture_is_not_background(&blender, &packed.join("run/job").join(frame), &here);
            println!("    packed: {cube} pixels of cube");
            assert!(cube > 0, "{named}: a packed bake comes with the file");
        }
    }
}

/// **`Dependency Upload & Remap` G9a's linked case, with no Smoozy in it: what
/// `{blend_name}` is when the object lives in a linked file.** `L.blend` bakes
/// under `//bakes/{blend_name}/` and is linked into `T.blend`; the object has
/// its cube only where the bake is read. `//` is measured from the library, and
/// the name is the file opened.
#[test]
#[ignore = "run by hand at the desk with Blender 5.3 beta and 5.4 alpha, one at a time"]
fn a_linked_files_blend_name_is_the_file_blender_opened() {
    const LINKER: &str = r#"
import bpy, sys
library, top = sys.argv[sys.argv.index("--") + 1:]
for one in list(bpy.data.objects):
    if one.type == "MESH":
        bpy.data.objects.remove(one)
bpy.ops.wm.save_as_mainfile(filepath=top)
with bpy.data.libraries.load(library, link=True, relative=True) as (there, here):
    here.objects = ["Cube"]
for one in here.objects:
    bpy.context.scene.collection.objects.link(one)
bpy.ops.wm.save_as_mainfile(filepath=top, relative_remap=True)
"#;
    const COUNTER: &str = r#"
import bpy
bpy.context.scene.frame_set(1)
one = bpy.data.objects["Cube"].evaluated_get(bpy.context.evaluated_depsgraph_get())
print("SMOOZY-DESK-CUBE", len(one.data.vertices), flush=True)
"#;
    let out = the_folder().join("linked-bake");
    for (named, blender) in the_blenders(&["5.3", "5.4"]) {
        let here = out.join(named);
        let assets = here.join("assets");
        let library = make(&blender, &a_file_baked_into("//bakes/{blend_name}/"), &assets, "L");
        let linker = here.join("link.py");
        let counter = here.join("count.py");
        std::fs::write(&linker, LINKER).expect("written");
        std::fs::write(&counter, COUNTER).expect("written");
        let top = here.join("T.blend");
        let words = |file: &Path, script: &Path, more: &[String]| {
            let mut all = vec![
                "--factory-startup".to_string(),
                "--background".to_string(),
                file.display().to_string(),
                "--python".to_string(),
                script.display().to_string(),
            ];
            all.extend_from_slice(more);
            all
        };
        let (status, _, _) = run(
            &blender,
            &[
                "--factory-startup".to_string(),
                "--background".to_string(),
                "--python-exit-code".to_string(),
                "63".to_string(),
                "--python".to_string(),
                linker.display().to_string(),
                "--".to_string(),
                library.display().to_string(),
                top.display().to_string(),
            ],
        );
        assert_eq!(status, Some(0), "{named} links");
        let the_cube_has = |file: &Path| -> usize {
            let (_, said, _) = run(&blender, &words(file, &counter, &[]));
            let said = String::from_utf8_lossy(&said);
            said.lines()
                .find_map(|line| line.strip_prefix("SMOOZY-DESK-CUBE "))
                .expect("a count")
                .trim()
                .parse()
                .expect("a number")
        };
        let bakes = assets.join("bakes");
        let opened_alone = the_cube_has(&library);
        let linked_with_the_bake_under_l = the_cube_has(&top);
        std::fs::rename(bakes.join("L"), bakes.join("T")).expect("moved");
        let linked_with_the_bake_under_t = the_cube_has(&top);
        println!(
            "{named}: L opened alone {opened_alone}; linked into T, bake at <L's folder>/bakes/L {linked_with_the_bake_under_l}; at <L's folder>/bakes/T {linked_with_the_bake_under_t}"
        );
        assert_eq!(
            (opened_alone, linked_with_the_bake_under_l, linked_with_the_bake_under_t),
            (8, 0, 8),
            "{named}"
        );
    }
}

/// **What the nested tests share: a tree for this version's route and a node
/// for its property names.** 4.5 holds its tree at `node_tree` and its outputs as
/// slots; from 5.0 the compositing group and an item list.
const THE_NESTED_HEAD: &str = r#"
v = bpy.app.version
def a_top_tree():
    if v >= (5, 0, 0):
        top = bpy.data.node_groups.new("Compositor", "CompositorNodeTree")
        scene.compositing_node_group = top
        return top
    scene.use_nodes = True
    scene.node_tree.nodes.clear()
    return scene.node_tree
def a_file_output(tree, name, folder, file_name):
    fo = tree.nodes.new("CompositorNodeOutputFile")
    fo.name = name
    if v >= (5, 0, 0):
        fo.directory = folder
        fo.file_name = file_name
        fo.format.media_type = "IMAGE"
        fo.format.file_format = "PNG"
        fo.file_output_items.clear()
        fo.file_output_items.new("RGBA", "Image")
    else:
        fo.format.file_format = "PNG"
        fo.base_path = folder
        fo.file_slots.clear()
        fo.file_slots.new(file_name + "Image")
    return fo
def a_group(name):
    inner = bpy.data.node_groups.new(name, "CompositorNodeTree")
    inner.interface.new_socket("Image", in_out="INPUT", socket_type="NodeSocketColor")
    inner.nodes.new("NodeGroupInput")
    return inner
def into(inner, holds):
    inner.links.new(inner.nodes["Group Input"].outputs[0], holds.inputs[0])
def held_in(tree, inner, source):
    holder = tree.nodes.new("CompositorNodeGroup")
    holder.node_tree = inner
    tree.links.new(source, holder.inputs[0])
    return holder
top = a_top_tree()
rl = top.nodes.new("CompositorNodeRLayers")
rl.scene = scene
rl.layer = scene.view_layers[0].name
beauty = rl.outputs[0]
def saved():
    print("SMOOZY-DESK-MADE", bpy.app.version_string, bpy.app.build_hash.decode(), flush=True)
    bpy.ops.wm.save_as_mainfile(filepath=out)
"#;

/// **`Job files and outputs` D8b: a File Output node inside a nested group, in
/// the versions before 5.3.** Until piece 5 the redirect there looked only at
/// the scene's top tree while the reading counted nested nodes, so the job ended
/// at status 63 with nothing written (5.2.3, `KeyError`). Each case is one file;
/// the node lands under the job's folder and the inventory matches the disk. Run
/// on 5.3 and 5.4 too, because the same walk serves them.
#[test]
#[ignore = "run by hand at the desk with Blender 4.5, 5.0, 5.1, 5.2, 5.3 beta and 5.4 alpha, one at a time"]
fn a_node_in_a_nested_group_before_five_three_ends_where() {
    let out = the_folder().join("nested-before");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["4.5", "5.0", "5.1", "5.2", "5.3", "5.4"]) {
        println!("{named}:");
        for (case, lines, wanted) in [
            (
                "one nested node",
                r#"
inner = a_group("Inner")
into(inner, a_file_output(inner, "Inside", "//nested", "n_"))
held_in(top, inner, beauty)
"#,
                &["nested/n_Image0001.png"][..],
            ),
            (
                "a group inside a group",
                r#"
deep = a_group("Deep")
into(deep, a_file_output(deep, "Inside", "//deep", "d_"))
middle = a_group("Middle")
held_in(middle, deep, middle.nodes["Group Input"].outputs[0])
held_in(top, middle, beauty)
"#,
                &["deep/d_Image0001.png"][..],
            ),
            (
                "one name at the top and in a group",
                r#"
top.links.new(beauty, a_file_output(top, "Same", "//up", "u_").inputs[0])
inner = a_group("Inner")
into(inner, a_file_output(inner, "Same", "//down", "d_"))
held_in(top, inner, beauty)
"#,
                &["up/u_Image0001.png", "down/d_Image0001.png"][..],
            ),
            (
                "one name in two groups",
                r#"
one = a_group("One")
into(one, a_file_output(one, "Same", "//one", "o_"))
held_in(top, one, beauty)
two = a_group("Two")
into(two, a_file_output(two, "Same", "//two", "t_"))
held_in(top, two, beauty)
"#,
                &["one/o_Image0001.png", "two/t_Image0001.png"][..],
            ),
            (
                "one group held twice",
                r#"
inner = a_group("Inner")
into(inner, a_file_output(inner, "Inside", "//shared", "s_"))
held_in(top, inner, beauty)
held_in(top, inner, beauty)
"#,
                &["shared/s_Image0001.png"][..],
            ),
            (
                "one group held by two scenes",
                r#"
inner = a_group("Inner")
into(inner, a_file_output(inner, "Inside", "//shared", "s_"))
held_in(top, inner, beauty)
other = bpy.data.scenes.new("Other")
if v >= (5, 0, 0):
    other_top = bpy.data.node_groups.new("Other compositor", "CompositorNodeTree")
    other.compositing_node_group = other_top
else:
    other.use_nodes = True
    other_top = other.node_tree
    other_top.nodes.clear()
other_rl = other_top.nodes.new("CompositorNodeRLayers")
other_rl.scene = other
held_in(other_top, inner, other_rl.outputs[0])
"#,
                &["shared/s_Image0001.png"][..],
            ),
        ] {
            let here = out.join(named).join(case.replace(' ', "-"));
            let script = format!("{THE_SMALL_SCENE}\n{THE_NESTED_HEAD}\n{lines}\nsaved()\n");
            let file = make(&blender, &script, &here, "job");
            let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1]);
            println!(
                "    {case}: the render ended at {:?}; in the job's folder: {:?}; beside the file: {:?}; held: {:?}",
                came.the_renders_status, came.in_the_jobs_folder, came.beside_the_file, came.holds
            );
            for name in wanted {
                if !came.in_the_jobs_folder.iter().any(|one| one == name) {
                    faults.push(format!("{named}, {case}: no {name} in {:?}", came.in_the_jobs_folder));
                }
            }
            the_inventory_matches_the_disk(&format!("{named}, {case}"), &came, &mut faults);
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// **`Job files and outputs` D8b's other half: a file with no nested node comes
/// through as it did.** Two nodes in the top tree, on every older Blender. Its
/// assertions were written from a run before that walk existed, and the same
/// run after it answered the same.
#[test]
#[ignore = "run by hand at the desk with Blender 4.5, 5.0, 5.1 and 5.2, one at a time"]
fn a_file_with_no_nested_node_is_redirected_as_it_was() {
    let out = the_folder().join("no-nested");
    let mut faults = Vec::new();
    for (named, blender) in the_blenders(&["4.5", "5.0", "5.1", "5.2"]) {
        let here = out.join(named);
        let script = format!(
            r#"{THE_SMALL_SCENE}
{THE_NESTED_HEAD}
top.links.new(beauty, a_file_output(top, "One", "//one", "o_").inputs[0])
top.links.new(beauty, a_file_output(top, "Two", "//two", "t_").inputs[0])
saved()
"#
        );
        let file = make(&blender, &script, &here, "job");
        let came = one_job(&blender, &file, &here.join("run"), &an_untouched_payload(), vec![1, 2]);
        println!("{named}: the render ended at {:?}; in the job's folder: {:?}", came.the_renders_status, came.in_the_jobs_folder);
        let wanted = [
            "one/o_Image0001.png",
            "one/o_Image0002.png",
            "render/f_0001.png",
            "render/f_0002.png",
            "two/t_Image0001.png",
            "two/t_Image0002.png",
        ];
        if came.in_the_jobs_folder != wanted {
            faults.push(format!("{named}: {:?}", came.in_the_jobs_folder));
        }
        the_inventory_matches_the_disk(named, &came, &mut faults);
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

//! **§2 P1 step 4's own act: the render-start reading, performed as one short
//! headless run of the Blender the manifest names.**
//!
//! # What this is
//!
//! `Job files and outputs` R1: *"one invocation of the Blender the manifest
//! names, headless, over the command channel, returning one document between
//! sentinels"*, with **all three payloads out of one read**. R3 puts the same
//! script on both sides — the desktop's pre-load and this one — so **nothing
//! here composes a script**: the text is
//! [`smoozy_job_files_machine::reading_script::the_reading_script`]'s, and what
//! is here is the launch that carries it and the reading of what came back.
//!
//! **It is the shape [`crate::acting::launch::TheShortInvocationOnTheMachine`]
//! already has** for L14's version ask: a script written at a path Smoozy
//! chose, a headless run of the binary about to be launched, and the answer
//! taken from between `Job files and outputs` R1's two marks. One shape rather
//! than two.
//!
//! # R7 — it launches the way the render launches
//!
//! **The machine's own settings, and no factory-startup switch.** The runner's
//! ruling of 2026-09-20 takes R2's reason at its word: the reading must be
//! taken by the Blender that will render, and **which engines are registered
//! follows from the machine's settings** (`Render Settings` §1.4). A factory
//! startup would let the reading see a different set of engines from the render
//! minutes later, which is the one thing R2 exists to prevent — and `Rendering`
//! L9 already forbids the switch on the render's own line for the neighbouring
//! reason, that the machine's saved preferences are where the add-ons a job
//! needs are enabled.
//!
//! **And the file's own scripts run, as they do in every render** (the
//! Creator, 2026-10-08: *"Yes to running scripts when reading."*; `Render
//! Settings` V3, `Rendering` L9): the switch stands right after the no-window
//! mode, where the render's line has it.
//!
//! **The dependency check's factory startup is not a counter-example**: that
//! module states its own reason — it is the condition every observation it
//! rests on was taken under — and that reason does not reach a reading of the
//! user's own file.
//!
//! **At the desk the same act is run the other way round, deliberately.** The
//! four Blenders in `/Applications` are the Creator's, so every run of this
//! code taken at the desk for proving is started with `--factory-startup`,
//! which is **not** how the machine launches it. What the desk proves is the
//! composition and the parse; the launch itself is proved on the rental.
//!
//! # The line, and why it is not [`crate::line::LaunchLine`]
//!
//! **That type is the render's contract** — L8's window, the render
//! instruction, the bare double dash and Cycles' own line — and a reading is
//! none of those. What the two share is the spellings, so every word below
//! comes from [`crate::spellings`] rather than being written out here: the
//! no-window mode (L3), the failure-to-exit-status switch with its number
//! (L8a), and the script argument (L8 item 7). **The file comes before the
//! script argument**, which is L8's own order and the only part of it a reading
//! has.
//!
//! **No bare double dash and nothing after one** (L8's third edge), and **no
//! device argument**: a reading renders nothing and has nothing to say to
//! Cycles.
//!
//! # `Job files and outputs` R18 — a read writes nothing beside the `.blend`
//!
//! **Added 2026-10-02, simulation fix piece 5.** The reading steps the frames
//! it is asked about, and at the desk that wrote a whole cache beside the
//! user's `.blend`: 96 files for a Replay smoke and 48 for an unbaked cloth, in
//! 4.5.12 and 5.2.2. So the line carries L8b's script before the file, whose
//! handler ([`crate::load_time::for_a_reading`]) points every Replay fluid
//! cache and every disk cache at a throwaway folder beside the reading's own
//! script — [`the_readings_own_working_paths`] — and the folder is removed
//! when the run ends. The script reads what the handler changed as the file
//! had it, so what it reports is unchanged.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` C9, R1, R1a, R2, R3, R4a, R7, R18.
//! `Render Settings` Q7. `Dependency Upload & Remap` M8b.

use std::io::Read;
use std::process::{Command, Stdio};

use smoozy_job_files_machine::reading::{RecordedLocation, TheInvocation, TheVersionInHand};
use smoozy_job_files_machine::reading_script::{
    the_printed_reading_in, the_reading_script, ThePrintedReading,
};

use crate::spellings::{
    Spelling, FAILURE_TO_EXIT_STATUS_SWITCH, LAUNCH_MATERIAL_ARGUMENT, NO_WINDOW_MODE,
    SCRIPTS_RUN_SWITCH, THE_FAILURE_STATUS,
};

/// **What one reading run is handed** — R1's invocation, plus the two paths
/// the act itself needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheReadingToTake<'a> {
    /// **The Blender the manifest names, by path** — a version alone cannot
    /// identify a build (`Server Setup` B13).
    pub the_blender_build_path: &'a str,
    /// **The `.blend` to read: through the folder the render opens it from, where
    /// `Dependency Upload & Remap` M8b makes one, and otherwise as it sits on the
    /// machine** (`Job files and outputs` R1a). Read away from its libraries, a file
    /// shows nothing that sits on a linked object.
    pub the_blend_path_on_the_machine: &'a str,
    /// **R2: the reading is taken by the version that will render**, and the
    /// script is composed for that version and refuses to run in another.
    pub the_version_it_will_render_by: TheVersionInHand,
    /// **The frames the job covers**, which `Job files and outputs` R4 item 5
    /// is evaluated at.
    pub the_frames_the_job_covers: &'a [i64],
    /// **The job's scene**, as the manifest names it. `None` where the job
    /// names none, in which case the script reads the file's own scene — the
    /// scene a render with no scene argument would use (`Rendering` L15's
    /// unnamed branch).
    pub the_jobs_scene: Option<&'a [u8]>,
    /// **Where the script is written.** Smoozy's own path — it carries no
    /// value of the machine's or the user's.
    pub the_scripts_path: &'a str,
    /// **`Job files and outputs` R4c: whether the job's payload carries
    /// `Render Settings` Y1b's entry**, so the reading writes a first-frame
    /// check of a video-only job as a still picture. Added 2026-10-27.
    pub the_check_is_written_as_a_picture: bool,
    /// **`Job files and outputs` P18a: whether the job's payload names a range
    /// job as *writes into*** (`Rendering` S13), so the reading marks every
    /// name the check will write `check_`. Added 2026-10-31.
    pub the_check_is_marked: bool,
}

/// **The words after the program's own path**, from the spellings.
///
/// **The order is L8's**: the process options at the head, then the file, then
/// the script. Nothing is added and nothing is re-ordered.
pub fn the_words_of_a_reading(
    the_blend_path_on_the_machine: &str,
    the_load_time_scripts_path: &str,
    the_scripts_path: &str,
) -> Result<Vec<String>, crate::spellings::MissingSpelling> {
    fn text(spelling: Spelling) -> Result<String, crate::spellings::MissingSpelling> {
        spelling.text().map(str::to_string)
    }
    Ok(vec![
        text(NO_WINDOW_MODE)?,
        // **L9's switch, as on the render's line**: the file's own scripts run.
        text(SCRIPTS_RUN_SWITCH)?,
        text(FAILURE_TO_EXIT_STATUS_SWITCH)?,
        THE_FAILURE_STATUS.to_string(),
        // **L8b's script before the file** (`Job files and outputs` R18).
        text(LAUNCH_MATERIAL_ARGUMENT)?,
        the_load_time_scripts_path.to_string(),
        the_blend_path_on_the_machine.to_string(),
        text(LAUNCH_MATERIAL_ARGUMENT)?,
        the_scripts_path.to_string(),
    ])
}

/// **Where a read's two working files go, beside its own script**: L8b's
/// script before the file, and the throwaway folder its handler points every
/// Replay fluid cache and every disk cache at (`Job files and outputs` R18).
/// Smoozy's own paths, carrying no value of the machine's or the user's.
pub fn the_readings_own_working_paths(the_scripts_path: &str) -> (String, String) {
    let stem = the_scripts_path
        .strip_suffix(".py")
        .unwrap_or(the_scripts_path);
    (format!("{stem}-load-time.py"), format!("{stem}-caches"))
}

/// **R18: the read's load-time script, written, with its throwaway folder
/// cleared first** — so a folder left by an earlier run is not mistaken for
/// this one's. Hands back the script's path and the folder's.
pub fn write_the_reads_load_time_script(the_scripts_path: &str) -> Result<(String, String), String> {
    let (script, throwaway) = the_readings_own_working_paths(the_scripts_path);
    remove_the_throwaway_folder(&throwaway);
    let text = super::load_time::the_load_time_scripts_text(&crate::load_time::for_a_reading(
        throwaway.as_bytes(),
    ));
    std::fs::write(&script, text).map_err(|why| format!("{script}: {why}"))?;
    Ok((script, throwaway))
}

/// **R18's *removed after***: the throwaway folder and everything under it.
/// A folder that is not there is the ordinary case for a file with no cache.
pub fn remove_the_throwaway_folder(throwaway: &str) {
    let _ = std::fs::remove_dir_all(throwaway);
}

/// Taking the reading, behind a trait — **the one place in this crate that
/// runs a Blender for anything but a render.**
pub trait TakesTheReading {
    /// **R1's one invocation.** The reason it could not be taken, verbatim,
    /// where it could not — and `Job files and outputs` C9 makes that a failed
    /// attempt rather than an empty reading.
    fn take(&self, taking: &TheReadingToTake<'_>) -> Result<ThePrintedReading, String>;
}

/// The real end, on the machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheReadingOnTheMachine;

impl TakesTheReading for TheReadingOnTheMachine {
    fn take(&self, taking: &TheReadingToTake<'_>) -> Result<ThePrintedReading, String> {
        let invocation = TheInvocation {
            the_blender_build_path: taking.the_blender_build_path.to_string(),
            // **R4a: the path crosses as bytes and is never decoded to be
            // carried.** It is handed to the kernel as an argument, not
            // interpolated into any line (`Server protocol and transfers`
            // §1.1).
            the_blend_path_on_the_machine: RecordedLocation::of(
                taking.the_blend_path_on_the_machine.as_bytes().to_vec(),
            ),
            the_version_it_will_render_by: taking.the_version_it_will_render_by,
            the_frames_the_job_covers: taking.the_frames_the_job_covers.to_vec(),
            the_check_is_written_as_a_picture: taking.the_check_is_written_as_a_picture,
            the_check_is_marked: taking.the_check_is_marked,
        };
        let scene = taking
            .the_jobs_scene
            .map(|name| smoozy_job_files_machine::reading::ArtistsText::of(name.to_vec()));
        let script = the_reading_script(&invocation, scene.as_ref())
            .map_err(|missing| format!("a spelling is not on record: {}", missing.what))?;
        std::fs::write(taking.the_scripts_path, script)
            .map_err(|why| format!("{}: {why}", taking.the_scripts_path))?;
        let (load_time, throwaway) = write_the_reads_load_time_script(taking.the_scripts_path)?;
        let words = the_words_of_a_reading(
            taking.the_blend_path_on_the_machine,
            &load_time,
            taking.the_scripts_path,
        )
        .map_err(|missing| format!("a spelling is not on record: {}", missing.what))?;
        let mut started = Command::new(taking.the_blender_build_path)
            .args(&words)
            // **Held by nothing a connection owns**, the same as the render's
            // own start: standard input closed rather than inherited.
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|why| {
                format!(
                    "the reading could not be started: {} could not be run: {why}",
                    taking.the_blender_build_path
                )
            })?;
        let mut answered = Vec::new();
        started
            .stdout
            .take()
            .ok_or_else(|| "the reading's own output was not held".to_string())?
            .read_to_end(&mut answered)
            .map_err(|why| format!("the reading's own output could not be read: {why}"))?;
        // **The ending is reaped rather than left**, so the reading leaves no
        // process behind whatever it answers.
        let ended = started.wait().ok();
        // **R18: the throwaway folder goes with the run.**
        remove_the_throwaway_folder(&throwaway);
        // **`Render Settings` Q8's start sentinel, said in the reason**: a
        // reading whose script started and then ended with no document — the
        // Local View crash of 2026-10-02 — is not a file this Blender could
        // not open, and the attempt's own words say which it was.
        let started = smoozy_job_files_machine::reading::the_reading_started_in(&answered);
        the_printed_reading_in(&answered).map_err(|why| {
            format!(
                "the reading printed no document ({why:?}); {}; the run ended at {}",
                if started {
                    "the reading's script started and did not finish"
                } else {
                    "the reading's script never started"
                },
                match ended.and_then(|ended| ended.code()) {
                    Some(status) => status.to_string(),
                    None => "no status".to_string(),
                }
            )
        })
    }
}

/// **R7, held as a value: the reading launches the way the render launches.**
///
/// The runner's ruling of 2026-09-20. There is no factory-startup switch in
/// [`the_words_of_a_reading`] and no [`Spelling`] for one anywhere in this
/// crate, which is L9's own arrangement — so the two sides cannot differ by
/// anything this file does.
pub const THE_READING_LAUNCHES_THE_WAY_THE_RENDER_LAUNCHES: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spellings::THE_BARE_DOUBLE_DASH;

    /// **The line is five things and the file sits before the script, with
    /// L8b's script right before the file** (`Job files and outputs` R18, since
    /// 2026-10-02). A script argument before the file would run against no file
    /// at all; L8b's must run before it, to register its handler.
    #[test]
    fn the_line_carries_the_process_options_then_the_file_then_the_script() {
        let words = the_words_of_a_reading(
            "/ephemeral/smoozy/assets/kitchen.blend",
            "/root/job-records/j1/reading-1-load-time.py",
            "/root/job-records/j1/reading-1.py",
        )
        .expect("every spelling");
        assert_eq!(
            words,
            vec![
                "--background".to_string(),
                "--enable-autoexec".to_string(),
                "--python-exit-code".to_string(),
                "63".to_string(),
                "--python".to_string(),
                "/root/job-records/j1/reading-1-load-time.py".to_string(),
                "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
                "--python".to_string(),
                "/root/job-records/j1/reading-1.py".to_string(),
            ]
        );
    }

    /// **The reading lets the file's own scripts run, as every render does**
    /// (the Creator, 2026-10-08: *"Yes to running scripts when reading."*;
    /// `Render Settings` V3) — the switch once, right after the no-window
    /// mode, where the render's line has it.
    #[test]
    fn the_reading_lets_the_files_own_scripts_run_once_where_the_render_does() {
        let switch = crate::spellings::SCRIPTS_RUN_SWITCH.text().unwrap();
        let words = the_words_of_a_reading("/a.blend", "/a-load-time.py", "/a.py").expect("every spelling");
        assert_eq!(words.iter().filter(|word| *word == switch).count(), 1, "{words:?}");
        assert_eq!(&words[..2], &["--background", switch], "{words:?}");
    }

    /// **R7: no factory-startup switch, and L8's third edge: no bare double
    /// dash.** The first is the whole of the runner's ruling; the second would
    /// put the script argument where Blender reads nothing of its own.
    #[test]
    fn the_line_carries_no_factory_startup_switch_and_no_bare_double_dash() {
        let words = the_words_of_a_reading("/a.blend", "/a-load-time.py", "/a.py").expect("every spelling");
        assert!(
            !words.iter().any(|word| word.contains("factory")),
            "R7: the machine's own settings"
        );
        assert!(!words.iter().any(|word| word == THE_BARE_DOUBLE_DASH));
        assert!(THE_READING_LAUNCHES_THE_WAY_THE_RENDER_LAUNCHES);
    }

    /// **A stand-in for Blender, the way piece 5 proved the render worker**: a
    /// shell script named where a build's path goes, printing a document
    /// between `Job files and outputs` R1's own two marks. **Nothing here says
    /// what a real Blender prints** — what is proved is that the act writes the
    /// script, runs the binary, holds its output and reads the document out of
    /// it.
    fn a_stand_in_that_prints(what: &str) -> (Result<ThePrintedReading, String>, String) {
        use smoozy_state::identities::{Identity, IdentityKind};
        use std::os::unix::fs::PermissionsExt;
        // **The folder is the unit**: a fresh identity's own bytes, never a
        // clock, and removed whole.
        let root = std::env::temp_dir().join(format!(
            "smoozy-a-reading-{}",
            Identity::issue(IdentityKind::Operation)
                .expect("randomness")
                .as_text()
        ));
        std::fs::create_dir_all(&root).expect("the folder");
        let stand_in = root.join("a-stand-in-for-blender");
        std::fs::write(&stand_in, what).expect("the stand-in");
        std::fs::set_permissions(&stand_in, std::fs::Permissions::from_mode(0o755))
            .expect("runnable");
        let script = root.join("reading-1.py");
        let frames = [1i64, 2, 3];
        let answered = TheReadingOnTheMachine.take(&TheReadingToTake {
            the_blender_build_path: &stand_in.display().to_string(),
            the_blend_path_on_the_machine: "/ephemeral/smoozy/assets/kitchen.blend",
            the_version_it_will_render_by: TheVersionInHand::of(4, 5),
            the_frames_the_job_covers: &frames,
            the_jobs_scene: Some(b"Kitchen"),
            the_scripts_path: &script.display().to_string(),
            the_check_is_written_as_a_picture: false,
            the_check_is_marked: false,
        });
        let written = std::fs::read_to_string(&script).unwrap_or_default();
        let (load_time, throwaway) = the_readings_own_working_paths(&script.display().to_string());
        let load_time_written = std::fs::read_to_string(&load_time).unwrap_or_default();
        let the_throwaway_is_gone = !std::path::Path::new(&throwaway).exists();
        std::fs::remove_dir_all(&root).ok();
        let named = load_time_written.contains(&serde_json::to_string(throwaway.as_bytes()).unwrap());
        LAST_LOAD_TIME.with(|last| {
            *last.borrow_mut() = (load_time_written, the_throwaway_is_gone && named)
        });
        (answered, written)
    }

    thread_local! {
        /// What the last stand-in run left: L8b's script's text, and whether
        /// the throwaway folder was gone afterwards.
        static LAST_LOAD_TIME: std::cell::RefCell<(String, bool)> =
            std::cell::RefCell::new((String::new(), false));
    }

    /// **`Job files and outputs` R18 at the act: the reading writes L8b's
    /// script beside its own, pointing at a throwaway folder beside it, and
    /// the folder is gone when the run ends** — here a stand-in that writes a
    /// cache file into that folder, the way a Replay smoke or an unbaked cloth
    /// did beside the `.blend` at the desk.
    #[test]
    fn the_reading_carries_a_throwaway_folder_for_caches_and_removes_it_after() {
        let (answered, _) = a_stand_in_that_prints(
            // `$6` is L8b's script, which sits beside the reading's own.
            "#!/bin/sh\nmkdir -p \"$(dirname \"$6\")/reading-1-caches/0\" || exit 9\necho cache > \"$(dirname \"$6\")/reading-1-caches/0/fluid_data_0001.vdb\" || exit 9\nprintf '<<<SMOOZY-READING>>>{}<<<END-SMOOZY-READING>>>'\n",
        );
        // The stand-in made its cache file, or it ended at 9 and printed none.
        assert!(
            !matches!(&answered, Err(why) if why.contains("ended at 9")),
            "{answered:?}"
        );
        LAST_LOAD_TIME.with(|last| {
            let (text, gone) = &*last.borrow();
            assert!(text.contains("bpy.app.handlers.load_post.append(smoozy_as_the_file_loads)"));
            assert!(text.contains(r#""carried_by":"EveryReplayFluidDomain""#));
            assert!(text.contains(r#""carried_by":"EverythingCarryingAPointCache""#));
            assert!(
                *gone,
                "the script names the throwaway folder beside it, and it is removed when the run ends"
            );
        });
        assert_eq!(
            the_readings_own_working_paths("/r/j1/reading-2.py"),
            ("/r/j1/reading-2-load-time.py".to_string(), "/r/j1/reading-2-caches".to_string())
        );
    }

    /// **The script is written, the run is made, and what it printed between
    /// the marks is read back.**
    #[test]
    fn the_script_is_written_and_the_document_is_taken_from_between_the_marks() {
        let (answered, written) = a_stand_in_that_prints(
            "#!/bin/sh\necho 'Blender quit'\nprintf '<<<SMOOZY-READING>>>'\ncat <<'EOF'\n{\"outputs\":{\"taken_by\":{\"major\":4,\"minor\":5},\"gating\":{\"compositing_switch\":true,\"a_compositing_tree_is_present\":false,\"a_live_final_output_node\":false,\"the_final_output_node_is_muted\":false,\"four_fives_use_nodes_switch\":true,\"save_output_switch\":null,\"the_render_goes_out_through_a_sequencer_holding_a_scene_strip\":false},\"destinations\":[],\"scenes_reached_through_a_sequencer_strip\":[],\"the_tree_is_shared_with\":null,\"per_frame\":[],\"recorded_locations\":[],\"views\":\"One\",\"the_compositing_tree\":null},\"settings\":{},\"dependencies\":{},\"errors\":{}}\nEOF\nprintf '<<<END-SMOOZY-READING>>>'\n",
        );
        let printed = answered.expect("the marks were both there");
        assert!(printed.errors.is_empty());
        assert_eq!(printed.outputs.taken_by, TheVersionInHand::of(4, 5));
        // **The script the run was handed is the one this workspace composes**,
        // carrying the frames the job covers and the job's scene.
        assert!(written.contains("the_frames_the_job_covers"));
        assert!(written.contains("Kitchen"), "the job's scene is handed in");
        assert!(
            !written.contains("factory"),
            "nothing in the script names a factory startup either"
        );
    }

    /// **`Job files and outputs` C9 from the other end: a run that printed no
    /// document is not a reading**, and the reason says what the run ended at
    /// rather than leaving an empty answer.
    #[test]
    fn a_run_that_printed_no_document_is_no_reading_and_says_what_it_ended_at() {
        let (answered, _) = a_stand_in_that_prints("#!/bin/sh\necho 'a traceback'\nexit 63\n");
        let why = answered.expect_err("no marks, no reading");
        assert!(why.contains("printed no document"), "{why}");
        assert!(why.contains("63"), "{why}");
        assert!(why.contains("never started"), "{why}");
    }

    /// **The Local View crash, 2026-10-02**: a reading whose script wrote its
    /// started mark and then ended by a signal says the script started and did
    /// not finish — never that it never ran — and says it ended at no status.
    #[test]
    fn a_reading_that_crashed_after_its_first_act_says_its_script_started() {
        let (answered, written) = a_stand_in_that_prints(
            "#!/bin/sh\nprintf '<<<SMOOZY-READING-STARTED>>>'\nkill -SEGV $$\n",
        );
        let why = answered.expect_err("no document, no reading");
        assert!(why.contains("started and did not finish"), "{why}");
        assert!(why.contains("no status"), "{why}");
        assert!(
            written.starts_with("import sys\nsys.stdout.write(\"<<<SMOOZY-READING-STARTED>>>\")"),
            "the mark is the script's first act"
        );
    }

    /// A build path that is not there is a refusal naming it, and never a
    /// reading taken from nothing.
    #[test]
    fn a_build_that_cannot_be_run_is_a_refusal_naming_it() {
        let script = std::env::temp_dir().join(format!(
            "smoozy-a-reading-{}.py",
            smoozy_state::identities::Identity::issue(
                smoozy_state::identities::IdentityKind::Operation
            )
            .expect("randomness")
            .as_text()
        ));
        let why = TheReadingOnTheMachine
            .take(&TheReadingToTake {
                the_blender_build_path: "/no/such/blender",
                the_blend_path_on_the_machine: "/a.blend",
                the_version_it_will_render_by: TheVersionInHand::of(5, 2),
                the_frames_the_job_covers: &[1],
                the_jobs_scene: None,
                the_scripts_path: &script.display().to_string(),
                the_check_is_written_as_a_picture: false,
                the_check_is_marked: false,
            })
            .expect_err("nothing to run");
        // **The read writes its load-time script beside its own**, and both go.
        let (load_time, throwaway) = the_readings_own_working_paths(&script.display().to_string());
        std::fs::remove_file(&script).ok();
        std::fs::remove_file(&load_time).ok();
        remove_the_throwaway_folder(&throwaway);
        assert!(!std::path::Path::new(&load_time).exists(), "the load-time script was left behind");
        assert!(why.contains("/no/such/blender"), "{why}");
    }
}

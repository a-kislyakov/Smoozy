//! **The relaunch helper: Windows' one short-lived helper for an update** —
//! Z18 to Z20, P3's second helper, Z21's Windows bullet and P2's handover, with
//! L13 used and `Reporting a problem` U5b. Built at step 11's Windows update piece.
//!
//! **What it is.** This same program, started by the old process with
//! [`THE_HELPERS_ARGUMENT`] and three paths: the new program, the previous one,
//! and the local record. `src/main.rs` reaches [`be_the_helper`] straight after
//! the askpass branch and before anything else — so the helper takes no hold
//! of its own, opens no record, sweeps nothing and builds no window. **It holds
//! no state, binds no listener, takes no secret** (Z19): everything it knows is
//! on its own command line, and it ends when the relaunch ends. **It is never a
//! service and never registered with anything.**
//!
//! **Why it is the running build's own program and not the new one's.** Z20
//! makes the helper the artifact's own component, checked as the artifact was,
//! and the running build is the one known to start. **So a new build that
//! cannot start at all is still rolled back** — the Windows half of §22 item
//! 46, which says Windows does not have that case because the helper outlives
//! the application. A helper inside the new build would share its fate.
//!
//! **What it does** ([`the_relaunch_is_done`]):
//!
//! 1. **The probe started**: the new program in the update-launch mode with a
//!    fresh one-use token (`crate::updating::the_probe`), its output and input
//!    piped back here. The probe waits for the hold, which the old process lets
//!    go as it exits (P2).
//! 2. **Watched until the launch deadline** (Z21).
//! 3. **On the token, the probe is let go and the helper ends.** The probe
//!    clears the record when its input closes, removing the previous build's
//!    folder where it can (`crate::updating::the_windows_side_by_side`).
//! 4. **On anything else, the rollback** (Z21's Windows bullet): the probe
//!    ended, **the hold taken back** (P2's last clause names this helper as the
//!    actor on Windows), **L8's copy restored first** (L13) — a file put back,
//!    never the record opened — **the shortcut pointed back at the previous
//!    program** (Z15a), **the previous program started in the update-launch
//!    mode alone, so it makes no crash offer** (Z21b's last bullet, `Reporting a
//!    problem` U5b), and the hold let go for it. **The previous build's own
//!    start finishes the rest** — it finds the record naming another version,
//!    removes the failed build's folder and clears the record, which is Z21a's
//!    fourth bullet for a rollback whose actor holds no record.
//!
//! **What it does not close.** A helper that dies part way leaves the record
//! and the shortcut where they were at that moment; the next ordinary launch,
//! through the shortcut, meets the record as Z21a says. Nothing says what
//! happens to a helper that dies (§22 item 46's last sentence), and no helper
//! is ever started again to finish.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Reporting a problem` U5b.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site, or
//! in the list above.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use super::the_probe::{self, AProbeUnderWatch, WhatTheProbeCameTo};
use crate::record;
use crate::start::THE_UPDATE_LAUNCH_ARGUMENT;

/// **The argument that makes a launch the relaunch helper.** Only the Windows
/// route passes it, and only as the first argument.
pub const THE_HELPERS_ARGUMENT: &str = "--smoozy-relaunch-helper";

/// **Everything the helper is told**, on its command line and nowhere else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheHelpersWork {
    /// The verified new program, in its new per-user folder (Z15).
    pub new_program: PathBuf,
    /// The program the old process ran from: the retained previous build.
    pub previous_program: PathBuf,
    /// The local record, whose L8 copy a rollback restores.
    pub record: PathBuf,
}

impl TheHelpersWork {
    /// The helper's whole command line after the program's name.
    pub fn as_arguments(&self) -> Vec<OsString> {
        vec![
            OsString::from(THE_HELPERS_ARGUMENT),
            self.new_program.clone().into_os_string(),
            self.previous_program.clone().into_os_string(),
            self.record.clone().into_os_string(),
        ]
    }

    /// **Read back**: the argument first, then exactly the three paths.
    /// Anything else is not the helper.
    pub fn from_arguments(arguments: impl IntoIterator<Item = OsString>) -> Option<TheHelpersWork> {
        let mut arguments = arguments.into_iter();
        if arguments.next()? != THE_HELPERS_ARGUMENT {
            return None;
        }
        let new_program = PathBuf::from(arguments.next()?);
        let previous_program = PathBuf::from(arguments.next()?);
        let record = PathBuf::from(arguments.next()?);
        if arguments.next().is_some() {
            return None;
        }
        Some(TheHelpersWork {
            new_program,
            previous_program,
            record,
        })
    }
}

/// **This launch's work, where it is the helper.**
pub fn this_launch_is_the_helper() -> Option<TheHelpersWork> {
    TheHelpersWork::from_arguments(std::env::args_os().skip(1))
}

/// **The helper's acts on the world**, so its order can be held at the desk.
/// The real form is [`TheRealActs`]; the tests keep their own.
pub trait TheHelpersActs {
    /// **The probe**: the new program in the update-launch mode with the token.
    fn the_probe(&self, new_program: &Path, token: &str) -> Result<AProbeUnderWatch, String>;
    /// **P2's last clause**: the hold taken back once the probe is ended.
    fn the_hold_is_taken_back(&mut self) -> bool;
    /// **Z15a**: the shortcut's target.
    fn the_shortcut_points_at(&self, program: &Path) -> Result<(), String>;
    /// **The build a rollback starts**: the update-launch mode alone.
    fn the_previous_build(&self, program: &Path) -> Result<(), String>;
    /// The hold let go, for the previous build to take.
    fn the_hold_is_let_go(&mut self);
}

/// **What the relaunch came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheRelaunchCameTo {
    /// The new build said its token.
    TheNewBuildRuns,
    /// The new build did not start, and the rollback ran. `left_undone` is any
    /// act that would not go, in the system's words.
    RolledBack { why: String, left_undone: Vec<String> },
}

/// **The helper's whole job** (Z19). See the head of this file for each step.
pub fn the_relaunch_is_done(
    work: &TheHelpersWork,
    acts: &mut dyn TheHelpersActs,
    deadline: Duration,
) -> WhatTheRelaunchCameTo {
    let Some(token) = the_probe::a_token() else {
        return the_rollback(work, acts, "no token could be made".into());
    };
    let mut watched = match acts.the_probe(&work.new_program, &token) {
        Ok(watched) => watched,
        Err(failed) => return the_rollback(work, acts, failed),
    };
    match watched.is_watched(deadline) {
        WhatTheProbeCameTo::ItStarted => {
            // **Let go and not waited for**: the probe runs on as the
            // application, and its input's end tells it to clear the record.
            drop(watched.is_let_go());
            WhatTheRelaunchCameTo::TheNewBuildRuns
        }
        failed => {
            watched.is_ended();
            the_rollback(work, acts, format!("{failed:?}"))
        }
    }
}

/// **Z21's Windows rollback**, every act tried and any that would not go said.
fn the_rollback(work: &TheHelpersWork, acts: &mut dyn TheHelpersActs, why: String) -> WhatTheRelaunchCameTo {
    let mut left_undone = Vec::new();
    if !acts.the_hold_is_taken_back() {
        left_undone.push("the hold could not be taken back".to_string());
    }
    // **L13: the copy is restored first, before the previous build reads the
    // record.** No copy means the probe ran no migration.
    if record::copy_beside(&work.record).exists() {
        if let Err(failed) = record::restore_the_copy(&work.record) {
            left_undone.push(format!("L8's copy was not restored: {failed:?}"));
        }
    }
    if let Err(failed) = acts.the_shortcut_points_at(&work.previous_program) {
        left_undone.push(format!("the shortcut was not pointed back: {failed}"));
    }
    if let Err(failed) = acts.the_previous_build(&work.previous_program) {
        left_undone.push(format!("the previous build was not started: {failed}"));
    }
    acts.the_hold_is_let_go();
    WhatTheRelaunchCameTo::RolledBack { why, left_undone }
}

/// **The real acts**: the programs started directly, the hold through
/// `crate::the_one_instance`, the shortcut through `crate::notifications`.
pub struct TheRealActs {
    hold: Option<crate::the_one_instance::TheHold>,
}

impl TheHelpersActs for TheRealActs {
    fn the_probe(&self, new_program: &Path, token: &str) -> Result<AProbeUnderWatch, String> {
        let mut command = Command::new(new_program);
        command.arg(THE_UPDATE_LAUNCH_ARGUMENT);
        AProbeUnderWatch::starts(command, token)
    }

    fn the_hold_is_taken_back(&mut self) -> bool {
        match crate::the_one_instance::the_launch_waits_for_the_hold() {
            crate::the_one_instance::TheLaunch::Runs(hold) => {
                self.hold = Some(hold);
                true
            }
            _ => false,
        }
    }

    fn the_shortcut_points_at(&self, program: &Path) -> Result<(), String> {
        crate::notifications::the_shortcut_is_pointed_at(program)
    }

    fn the_previous_build(&self, program: &Path) -> Result<(), String> {
        Command::new(program)
            .arg(THE_UPDATE_LAUNCH_ARGUMENT)
            .env_remove(the_probe::THE_PROBES_TOKEN)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map(drop)
            .map_err(|failed| failed.to_string())
    }

    fn the_hold_is_let_go(&mut self) {
        if let Some(hold) = self.hold.take() {
            crate::the_one_instance::the_hold_is_handed_to_the_probe(hold);
        }
    }
}

/// **The helper, run** — `src/main.rs`'s one call, whose return is the exit
/// code: zero for a new build that runs or a rollback with nothing left undone.
pub fn be_the_helper(work: &TheHelpersWork) -> i32 {
    let mut acts = TheRealActs { hold: None };
    match the_relaunch_is_done(work, &mut acts, the_probe::THE_LAUNCH_DEADLINE) {
        WhatTheRelaunchCameTo::TheNewBuildRuns => 0,
        WhatTheRelaunchCameTo::RolledBack { left_undone, .. } if left_undone.is_empty() => 0,
        WhatTheRelaunchCameTo::RolledBack { .. } => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::{update_trust, LocalRecord};
    use crate::updating::the_probe::tests::{a_probe_record, the_probe_child_command};
    use crate::updating::the_probe::{what_the_record_asks, WhatTheRecordAsks};
    use std::sync::Mutex;

    /// **The acts' test form**: this test program re-entered as the probe, the
    /// real hold at the test's folder, and a note of every act in order.
    struct TheTestsActs {
        folder: PathBuf,
        version: String,
        probe_acts: String,
        the_probe_will_not_start: bool,
        hold: Option<crate::the_one_instance::TheHold>,
        said: Mutex<Vec<String>>,
        previous: Mutex<Vec<(PathBuf, Vec<String>)>>,
    }

    impl TheTestsActs {
        fn at(folder: &Path, probe_acts: &str) -> TheTestsActs {
            TheTestsActs {
                folder: folder.to_path_buf(),
                version: "0.2.0".into(),
                probe_acts: probe_acts.into(),
                the_probe_will_not_start: false,
                hold: None,
                said: Mutex::new(Vec::new()),
                previous: Mutex::new(Vec::new()),
            }
        }
        fn said(&self) -> Vec<String> {
            self.said.lock().unwrap().clone()
        }
    }

    impl TheHelpersActs for TheTestsActs {
        fn the_probe(&self, new_program: &Path, token: &str) -> Result<AProbeUnderWatch, String> {
            self.said.lock().unwrap().push(format!("probe {}", new_program.display()));
            if self.the_probe_will_not_start {
                return Err("the probe would not start".into());
            }
            let mut command = the_probe_child_command(&self.folder, &self.version, &self.probe_acts);
            command
                .args(["--", THE_UPDATE_LAUNCH_ARGUMENT])
                .env("SMOOZY_TEST_THE_PROBE_CHILD_WANTS_THE_MODE", "1");
            AProbeUnderWatch::starts(command, token)
        }

        fn the_hold_is_taken_back(&mut self) -> bool {
            self.said.lock().unwrap().push("hold back".into());
            match crate::the_one_instance::the_hold_is_waited_for_at(
                &self.folder.join("running-copy.lock"),
                &self.folder.join("running-copy.door"),
            ) {
                crate::the_one_instance::TheLaunch::Runs(hold) => {
                    self.hold = Some(hold);
                    true
                }
                _ => false,
            }
        }

        fn the_shortcut_points_at(&self, program: &Path) -> Result<(), String> {
            self.said.lock().unwrap().push(format!("shortcut {}", program.display()));
            Ok(())
        }

        fn the_previous_build(&self, program: &Path) -> Result<(), String> {
            self.said.lock().unwrap().push(format!("previous {}", program.display()));
            self.previous
                .lock()
                .unwrap()
                .push((program.to_path_buf(), vec![THE_UPDATE_LAUNCH_ARGUMENT.to_string()]));
            Ok(())
        }

        fn the_hold_is_let_go(&mut self) {
            self.said.lock().unwrap().push("hold let go".into());
            if let Some(hold) = self.hold.take() {
                crate::the_one_instance::the_hold_is_handed_to_the_probe(hold);
            }
        }
    }

    /// One throwaway handover: the old process's record with Z21a's record in
    /// it, and the hold the old process took and then let go as it exited.
    /// **The old process has gone, so no connection to the record is kept
    /// open here** — on Windows a rename over an open record could be refused.
    struct AHandover {
        folder: PathBuf,
        work: TheHelpersWork,
    }

    impl AHandover {
        fn the_record(&self) -> LocalRecord {
            LocalRecord::open_at(&self.work.record).unwrap()
        }
    }

    fn a_handover(name: &str) -> AHandover {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-helper-{name}-{}", std::process::id()));
        let path = folder.join("local-record.sqlite3");
        {
            let record = LocalRecord::open_at(&path).unwrap();
            update_trust::an_update_goes_into_probe(
                record.connection(),
                &a_probe_record("0.2.0", &path, &folder.join("Apps/.Smoozy previous.app")),
            )
            .unwrap();
        }
        // The old process held the hold and let it go as it exited.
        let crate::the_one_instance::TheLaunch::Runs(hold) = crate::the_one_instance::the_hold_is_tried_at(
            &folder.join("running-copy.lock"),
            &folder.join("running-copy.door"),
        ) else {
            panic!("the old process takes the hold");
        };
        crate::the_one_instance::the_hold_is_handed_to_the_probe(hold);
        AHandover {
            work: TheHelpersWork {
                new_program: folder.join("windows-update/0.2.0/smoozy-desktop.exe"),
                previous_program: folder.join("windows-update/0.1.8/smoozy-desktop.exe"),
                record: path,
            },
            folder,
        }
    }

    /// **The helper's command line is three paths after its argument, read
    /// back exactly**, and nothing else is the helper.
    #[test]
    fn the_helpers_work_is_its_command_line_and_nothing_else() {
        let work = TheHelpersWork {
            new_program: PathBuf::from("C:/Users/a/AppData/Local/Smoozy/windows-update/0.2.0/smoozy-desktop.exe"),
            previous_program: PathBuf::from("C:/Users/a/Downloads/smoozy desktop.exe"),
            record: PathBuf::from("C:/Users/a/AppData/Local/Smoozy/local-record.sqlite3"),
        };
        assert_eq!(TheHelpersWork::from_arguments(work.as_arguments()), Some(work.clone()));
        let mut more = work.as_arguments();
        more.push("--extra".into());
        assert_eq!(TheHelpersWork::from_arguments(more), None);
        assert_eq!(TheHelpersWork::from_arguments(work.as_arguments().into_iter().skip(1)), None);
        assert_eq!(TheHelpersWork::from_arguments(work.as_arguments().into_iter().take(3)), None);
        assert_eq!(TheHelpersWork::from_arguments(Vec::<OsString>::new()), None);
        assert_eq!(
            crate::start::LaunchMode::from_arguments(work.as_arguments()),
            crate::start::LaunchMode::Ordinary,
            "the helper's command line reads as an update launch"
        );
    }

    /// **A probe that says its token is let go, and the helper does nothing
    /// else**: no shortcut moved back, no previous build started, the hold
    /// never taken — and the probe, once let go, clears the record.
    #[test]
    fn a_passing_probe_is_let_go_and_nothing_is_rolled_back() {
        let handover = a_handover("passes");
        let mut acts = TheTestsActs::at(&handover.folder, "starts");
        let came = the_relaunch_is_done(&handover.work, &mut acts, Duration::from_secs(30));
        assert_eq!(came, WhatTheRelaunchCameTo::TheNewBuildRuns);
        assert_eq!(acts.said(), vec![format!("probe {}", handover.work.new_program.display())]);
        // The probe ends once let go and lets the hold go; waited for on a
        // thread of its own, to a fixed deadline.
        let (got, getting) = std::sync::mpsc::channel();
        let folder = handover.folder.clone();
        std::thread::spawn(move || {
            let taken = crate::the_one_instance::the_hold_is_waited_for_at(
                &folder.join("running-copy.lock"),
                &folder.join("running-copy.door"),
            );
            let _ = got.send(matches!(taken, crate::the_one_instance::TheLaunch::Runs(_)));
        });
        assert_eq!(getting.recv_timeout(Duration::from_secs(30)), Ok(true), "the probe never let the hold go");
        assert_eq!(update_trust::the_update_in_probe(handover.the_record().connection()).unwrap(), None);
    }

    /// **A failing probe is rolled back in Z21's Windows order**: ended, the
    /// hold taken back, L8's copy restored over the probe's write, the shortcut
    /// pointed back, the previous build started **in the update-launch mode
    /// alone, which `the_crash_to_offer` answers with no offer** (`Reporting a
    /// problem` U5b), and the hold let go. The record is left naming the new
    /// version, for the previous build's own start to finish. A probe that
    /// hangs is ended at the deadline and rolled back the same way.
    #[test]
    fn a_failing_probe_is_rolled_back_in_order_with_no_crash_offer() {
        for probe_acts in ["migrates-and-fails", "hangs"] {
            let handover = a_handover(probe_acts);
            let mut acts = TheTestsActs::at(&handover.folder, probe_acts);
            let came = the_relaunch_is_done(
                &handover.work,
                &mut acts,
                Duration::from_secs(if probe_acts == "hangs" { 2 } else { 30 }),
            );
            let WhatTheRelaunchCameTo::RolledBack { left_undone, .. } = came else {
                panic!("{probe_acts}: {came:?}");
            };
            assert_eq!(left_undone, Vec::<String>::new(), "{probe_acts}");
            assert_eq!(
                acts.said(),
                vec![
                    format!("probe {}", handover.work.new_program.display()),
                    "hold back".to_string(),
                    format!("shortcut {}", handover.work.previous_program.display()),
                    format!("previous {}", handover.work.previous_program.display()),
                    "hold let go".to_string(),
                ],
                "{probe_acts}"
            );
            let restored = handover.the_record();
            assert_eq!(
                update_trust::the_declined_version(restored.connection()).unwrap(),
                None,
                "{probe_acts}: L8's copy was not restored over the probe's write"
            );
            assert!(!record::copy_beside(&handover.work.record).exists(), "{probe_acts}: the copy was not spent");
            assert!(
                matches!(
                    what_the_record_asks(restored.connection(), "0.1.8").unwrap(),
                    WhatTheRecordAsks::ARollbackDidNotFinish(_)
                ),
                "{probe_acts}: the previous build's start is owed the record"
            );
            let previous = acts.previous.lock().unwrap().clone();
            let mode = crate::start::LaunchMode::from_arguments(previous[0].1.iter().map(OsString::from));
            assert_eq!(mode, crate::start::LaunchMode::Update);
            let dead = smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::Run).unwrap();
            assert_eq!(
                crate::start::the_crash_to_offer(
                    true,
                    mode,
                    &smoozy_state::run_mark::WhatTheLaunchFound::ThePreviousRunDidNot { the_run: dead }
                ),
                None,
                "the rolled-back build offered a crash report"
            );
            assert!(acts.hold.is_none(), "{probe_acts}: the hold was kept by the helper");
        }
    }

    /// **A probe that cannot even be started is rolled back too** — the case a
    /// new build that cannot start at all comes to, and why the helper is the
    /// running build's program (§22 item 46's Windows half).
    #[test]
    fn a_probe_that_cannot_start_is_rolled_back() {
        let handover = a_handover("never-starts");
        let mut acts = TheTestsActs::at(&handover.folder, "starts");
        acts.the_probe_will_not_start = true;
        let came = the_relaunch_is_done(&handover.work, &mut acts, Duration::from_secs(5));
        assert!(matches!(came, WhatTheRelaunchCameTo::RolledBack { ref left_undone, .. } if left_undone.is_empty()));
        assert_eq!(acts.said()[1..], [
            "hold back".to_string(),
            format!("shortcut {}", handover.work.previous_program.display()),
            format!("previous {}", handover.work.previous_program.display()),
            "hold let go".to_string(),
        ]);
    }

    /// **Z19 read off the source**: the helper opens no record, takes no hold
    /// but in a rollback, and starts the previous build with the update-launch
    /// argument and the token taken off; and `src/main.rs` reaches it straight
    /// after the askpass branch, before the hold and before the shell.
    #[test]
    fn the_helper_holds_nothing_and_is_reached_before_anything_else() {
        let (working, _) = include_str!("the_relaunch_helper.rs")
            .split_once("#[cfg(test)]\nmod tests")
            .expect("the tests are last");
        assert!(!working.contains("LocalRecord::"), "the helper opens the record");
        assert!(!working.contains("the_launch_takes_the_hold"), "the helper takes an ordinary hold");
        let previous = working
            .split_once("fn the_previous_build(&self, program: &Path) -> Result<(), String> {\n        Command::new")
            .and_then(|(_, rest)| rest.split_once(".spawn()"))
            .map(|(body, _)| body)
            .expect("the real previous-build start is here");
        assert!(previous.contains(".arg(THE_UPDATE_LAUNCH_ARGUMENT)"));
        assert!(previous.contains(".env_remove(the_probe::THE_PROBES_TOKEN)"));

        let main = include_str!("../main.rs");
        let askpass = main.find("keys::askpass::be_the_helper(&channel)").expect("the askpass branch");
        let helper = main
            .find("updating::the_relaunch_helper::be_the_helper(&work)")
            .expect("the relaunch helper's branch");
        let hold = main.find("this_launch_is_decided(").expect("the hold");
        let shell = main.find("tauri::Builder::default()").expect("the shell");
        assert!(askpass < helper && helper < hold && hold < shell);
    }
}

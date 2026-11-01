//! **The probe: a new build started to show it runs, watched by the build it
//! replaces** — Z21, Z21a, Z21b and P2's handover, with `Reporting a problem`
//! U5a and `Reporting a problem` U5b. Built at step 11's Mac update piece.
//!
//! **What a successful start is** (Z21): the new build verified its own
//! release identity, ran or accepted the migration, opened the record, composed
//! its first module and returned **a one-use success token** to the actor that
//! launched it. **Anything short of that before the launch deadline is a failed
//! start.**
//!
//! **How the token travels, and it is Build's.** The old process starts the
//! probe as its own child, with the update-launch argument
//! (`--smoozy-update-launch`, R5's one mark; `crate::start::LaunchMode`) and
//! the token in the variable [`THE_PROBES_TOKEN`] — **the token is the probe
//! mode**: the update-launch argument alone is the build a rollback starts
//! (Z21b's last bullet), and the argument with the token is the probe. The
//! probe's output and input are two pipes back to the old process.
//!
//! 1. **The probe waits for the hold** (`crate::the_one_instance`'s waiting
//!    take), which the old process hands over only after it has stopped
//!    writing (P2).
//! 2. **It starts as any launch does** — the sweep, the record opened and
//!    migrated (L8's copy taken where a migration runs), the run-mark read and
//!    written, the diary's first line — and **makes no crash offer** (Z21b,
//!    `Reporting a problem` U5a).
//! 3. **It verifies its own release identity** against the update-in-probe
//!    record the old process wrote before publication (Z21a): the record must
//!    name this build's version. A probe that finds no record, or one naming
//!    another version, never says the token.
//! 4. **Once its window is built** — its first module composed — it writes
//!    [`THE_TOKENS_LINE`] and the token on its output, once.
//! 5. **The old process reads the token and exits.** That closes the probe's
//!    input, and **the probe then clears the record**, removing Z7's named
//!    backup and L8's copy with it (Z21a's *what clears it*). It clears at the
//!    input's end and not at the token's writing, so an old process that has
//!    already decided the start failed — and is ending the probe — never finds
//!    the backup gone under its own rollback.
//!
//! **What a launch that finds an uncleared record does** (Z21a's fourth
//! bullet), whichever kind of launch it is: a record naming this build's own
//! version is a probe that got this far without its watcher — **it completes the
//! probe itself**; a record naming another version is the retained previous
//! build running after a rollback that did not finish — **it completes that
//! rollback's remaining acts**. On this side both come to the same three acts
//! ([`what_the_record_kept_is_cleared`]): L8's copy discarded, the named backup
//! removed, the record cleared — the last, so a launch that dies part way does
//! it again next time.
//!
//! **What it does not close** is §22 item 46: a new build that cannot start at
//! all never reaches the record, and on macOS nothing rolls it back.
//!
//! **On Windows the watcher is the relaunch helper**
//! (`crate::updating::the_relaunch_helper`, step 11's Windows update piece), not the old process,
//! and the same token, output and input are used between them.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `State, identity and recovery` I9, R8a.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site, or
//! in the list above.

use std::io::{BufRead, Read, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use smoozy_state::identities::{Identity, IdentityKind};

use crate::platform::files;
use crate::record::update_trust::{self, TheUpdateInProbe};
use crate::record::{self, RecordFailure};

/// **The variable the probe's token arrives in.** Its presence is the probe
/// mode; nothing else sets it, and every other launch this program makes has
/// it taken off.
pub const THE_PROBES_TOKEN: &str = "SMOOZY_THE_PROBES_TOKEN";

/// **What the probe writes before the token**, so the token is a line of its
/// own among whatever else the program prints.
pub const THE_TOKENS_LINE: &str = "smoozy-probe-started ";

/// **The launch deadline — two minutes. Build's decision (Z21 names none).**
/// A start is a sweep, a migration and a window: seconds on this desk. Two
/// minutes leaves room for a large record's migration on a slow disk, and is
/// short enough that a build which hangs at its start is rolled back before a
/// user has given up on it.
pub const THE_LAUNCH_DEADLINE: Duration = Duration::from_secs(120);

/// **A fresh one-use token**: the text of a newly issued identity, which is
/// random bytes from the system and nothing else (`State, identity and
/// recovery` Y5). Nothing where the system gives none — and then no probe is
/// started at all.
pub fn a_token() -> Option<String> {
    Identity::issue(IdentityKind::Run).ok().map(|issued| issued.as_text())
}

// ════════════════════════════════════════════════════════════════════════════
// The old process's side: the probe started and watched
// ════════════════════════════════════════════════════════════════════════════

/// **What the watch came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheProbeCameTo {
    /// The token came back: a successful start.
    ItStarted,
    /// The probe ended before saying it, in the system's words for how.
    ItEndedFirst(String),
    /// The deadline passed with no token.
    TheDeadlinePassed,
}

/// **A probe, started and under watch.**
pub struct AProbeUnderWatch {
    child: Child,
    lines: mpsc::Receiver<String>,
    expected: String,
}

impl AProbeUnderWatch {
    /// **Started**, with the token in [`THE_PROBES_TOKEN`] and its output and
    /// input piped back here. `command` names the program and its arguments;
    /// what is set here is the rest.
    pub fn starts(mut command: Command, token: &str) -> Result<AProbeUnderWatch, String> {
        let mut child = command
            .env(THE_PROBES_TOKEN, token)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|failed| failed.to_string())?;
        let output = child.stdout.take().ok_or("the probe has no output")?;
        let (said, lines) = mpsc::channel();
        // **The probe's output, read line by line until it ends.** The reader
        // ends with the output; nothing waits on it.
        let _ = std::thread::Builder::new()
            .name("smoozy probe's output".into())
            .spawn(move || {
                for line in std::io::BufReader::new(output).lines().map_while(Result::ok) {
                    if said.send(line).is_err() {
                        return;
                    }
                }
            });
        Ok(AProbeUnderWatch {
            child,
            lines,
            expected: format!("{THE_TOKENS_LINE}{token}"),
        })
    }

    /// **Watched until the token comes back, the probe ends, or `within`
    /// passes** — a fixed deadline, read once.
    pub fn is_watched(&mut self, within: Duration) -> WhatTheProbeCameTo {
        let deadline = Instant::now() + within;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return WhatTheProbeCameTo::TheDeadlinePassed;
            }
            match self.lines.recv_timeout(left) {
                // **The line ends with the token**: whatever the program wrote
                // before it on the same line without ending one is not the
                // probe's to answer for, and the token is random and used once.
                Ok(line) if line.trim_end().ends_with(&self.expected) => return WhatTheProbeCameTo::ItStarted,
                Ok(_) => continue,
                Err(mpsc::RecvTimeoutError::Timeout) => return WhatTheProbeCameTo::TheDeadlinePassed,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    let how = match self.child.wait() {
                        Ok(status) => status.to_string(),
                        Err(failed) => failed.to_string(),
                    };
                    return WhatTheProbeCameTo::ItEndedFirst(how);
                }
            }
        }
    }

    /// **A failed start: the probe is ended** (P2: *"the probe is ended before
    /// the actor that launched the probe takes the instance lock back"*), and
    /// waited for, so it holds nothing afterwards.
    pub fn is_ended(mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }

    /// **A successful start: the probe is let go.** Its input closes here —
    /// and closes anyway when this process exits — which is what tells it to
    /// clear the record. The probe runs on; nothing here waits for it.
    pub fn is_let_go(mut self) -> Child {
        drop(self.child.stdin.take());
        self.child
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The probe's side, and every launch's
// ════════════════════════════════════════════════════════════════════════════

/// **This launch's token, where it is a probe.** An empty value is none.
pub fn this_launch_is_a_probe() -> Option<String> {
    std::env::var(THE_PROBES_TOKEN).ok().filter(|token| !token.trim().is_empty())
}

/// **What the update-in-probe record asks of this launch** (Z21a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheRecordAsks {
    /// No update is in its probe.
    Nothing,
    /// **The record names this build**: a probe, or a probe that got this far
    /// with no watcher.
    ThisBuildIsTheRelease(TheUpdateInProbe),
    /// **The record names another build**: this is the previous one, running
    /// after a rollback that did not finish.
    ARollbackDidNotFinish(TheUpdateInProbe),
}

/// **The record read against this build's own release identity** — its
/// package version, the one value the release stamps into every build (E15's
/// note of 2026-10-24).
pub fn what_the_record_asks(
    connection: &rusqlite::Connection,
    this_builds_version: &str,
) -> Result<WhatTheRecordAsks, RecordFailure> {
    Ok(match update_trust::the_update_in_probe(connection)? {
        None => WhatTheRecordAsks::Nothing,
        Some(probe) if probe.version == this_builds_version => WhatTheRecordAsks::ThisBuildIsTheRelease(probe),
        Some(probe) => WhatTheRecordAsks::ARollbackDidNotFinish(probe),
    })
}

/// **Z21a's clearing, with what the record kept**: L8's copy discarded, Z7's
/// named backup removed, and the record cleared, **in that order and the
/// record last**, so a launch that dies part way finds the record and does it
/// again.
///
/// **Nothing is removed because a record names it.** The copy is discarded only
/// where the record names this record's own copy, and the backup only where it
/// names `the_backup_kept_here` (this copy's own [`crate::platform::locations`]
/// place) **and is not the bundle this program is running from**, which is
/// how a previous build started from its backup does not remove itself.
pub fn what_the_record_kept_is_cleared(
    connection: &rusqlite::Connection,
    the_records_own_path: &Path,
    probe: &TheUpdateInProbe,
    the_backup_kept_here: Option<&Path>,
    the_bundle_running_now: Option<&Path>,
) -> Result<(), String> {
    // **On Linux the retained artifact is the cached package** (Z11), and
    // its clearing is the Linux route's (step 11's Linux piece).
    #[cfg(target_os = "linux")]
    if let Ok(cache) = super::the_linux_package::Cache::here() {
        if let Some(cleared) =
            super::the_linux_package::cleared_if_it_names(connection, the_records_own_path, &cache, probe)
        {
            return cleared;
        }
    }
    // **On Windows the retained artifact is the previous build's own folder**
    // (Z15), and its clearing is the Windows route's (step 11's Windows update piece).
    #[cfg(target_os = "windows")]
    if let Ok(places) = super::the_windows_side_by_side::ThePlaces::for_this_copy() {
        let _ = (the_backup_kept_here, the_bundle_running_now);
        return super::the_windows_side_by_side::what_the_record_kept_is_cleared_here(
            connection,
            the_records_own_path,
            probe,
            &places,
            super::checking::THIS_BUILDS_VERSION,
        );
    }
    let copy = record::copy_beside(the_records_own_path);
    if probe.the_record_copy == path_bytes(&copy) {
        record::discard_the_copy(the_records_own_path).map_err(|failed| format!("{failed:?}"))?;
    }
    if let Some(backup) = the_backup_kept_here {
        let names_it = probe.the_previous_artifact == path_bytes(backup);
        let runs_from_it = the_bundle_running_now.is_some_and(|running| running == backup);
        if names_it && !runs_from_it {
            files::a_folder_is_removed_whole(backup).map_err(|failed| failed.reported)?;
        }
    }
    update_trust::the_probe_is_cleared(connection).map_err(|failed| format!("{failed:?}"))
}

/// A path as its own bytes, as the record keeps every path.
pub fn path_bytes(path: &Path) -> Vec<u8> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    }
    #[cfg(not(unix))]
    {
        path.to_string_lossy().into_owned().into_bytes()
    }
}

/// **Step 4: the token said, once, on `out`.**
pub fn the_token_is_said(token: &str, out: &mut dyn Write) -> std::io::Result<()> {
    writeln!(out, "{THE_TOKENS_LINE}{token}")?;
    out.flush()
}

/// **Step 5's wait: until the watcher lets go**, which is the input ending.
/// Whatever arrives on it is read and not kept.
pub fn the_watcher_lets_go(input: &mut dyn Read) {
    let mut sink = [0u8; 256];
    loop {
        match input.read(&mut sink) {
            Ok(0) => return,
            Ok(_) => continue,
            Err(failed) if failed.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => return,
        }
    }
}

/// **On this program's own output and input**, in the running application: the
/// token said, then the output pointed at nothing, so a write after the old
/// process has gone meets no closed pipe; then the wait for the input's end,
/// and the input pointed at nothing too.
#[cfg(unix)]
pub fn the_token_is_said_on_this_programs_output(token: &str) -> std::io::Result<()> {
    let said = the_token_is_said(token, &mut std::io::stdout().lock());
    pointed_at_nothing(1);
    said
}

/// The wait for the input's end, on this program's own input.
#[cfg(unix)]
pub fn the_watcher_lets_go_of_this_program() {
    the_watcher_lets_go(&mut std::io::stdin().lock());
    pointed_at_nothing(0);
}

/// **The same on Windows** (step 11's Windows update piece): the token said, then the output
/// pointed at the null device by `SetStdHandle` — after which `GetStdHandle`
/// *"will return the redirected handle"*, and the standard library asks it
/// afresh at every write — so a write after the helper has gone meets no
/// closed pipe.
#[cfg(windows)]
pub fn the_token_is_said_on_this_programs_output(token: &str) -> std::io::Result<()> {
    let said = the_token_is_said(token, &mut std::io::stdout().lock());
    pointed_at_the_null_device(windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE);
    said
}

/// The wait for the input's end, on Windows.
#[cfg(windows)]
pub fn the_watcher_lets_go_of_this_program() {
    the_watcher_lets_go(&mut std::io::stdin().lock());
    pointed_at_the_null_device(windows_sys::Win32::System::Console::STD_INPUT_HANDLE);
}

/// `CreateFileW` on `NUL`, then `SetStdHandle`; the old handle closed. Where
/// the device will not open, the handle is left as it was.
#[cfg(windows)]
fn pointed_at_the_null_device(which: windows_sys::Win32::System::Console::STD_HANDLE) {
    use windows_sys::Win32::Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    };
    use windows_sys::Win32::System::Console::{GetStdHandle, SetStdHandle};
    let name: Vec<u16> = "NUL".encode_utf16().chain(Some(0)).collect();
    // SAFETY: `name` is a terminated wide string alive for the call; the
    // handles are the process's own and are not used after they are closed.
    unsafe {
        let nothing = CreateFileW(
            name.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            std::ptr::null_mut(),
        );
        if nothing == INVALID_HANDLE_VALUE {
            return;
        }
        let before = GetStdHandle(which);
        if SetStdHandle(which, nothing) != 0 && !before.is_null() && before != INVALID_HANDLE_VALUE {
            CloseHandle(before);
        }
    }
}

#[cfg(unix)]
fn pointed_at_nothing(descriptor: i32) {
    use std::os::unix::io::AsRawFd;
    if let Ok(nothing) = std::fs::OpenOptions::new().read(true).write(true).open("/dev/null") {
        // SAFETY: both descriptors are open for the length of the call.
        unsafe {
            libc::dup2(nothing.as_raw_fd(), descriptor);
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::record::LocalRecord;
    use crate::the_one_instance;

    pub(crate) fn a_probe_record(version: &str, record_path: &Path, backup: &Path) -> TheUpdateInProbe {
        TheUpdateInProbe {
            release_sequence: 7,
            version: version.into(),
            the_previous_artifact: path_bytes(backup),
            the_record_copy: path_bytes(&record::copy_beside(record_path)),
        }
    }

    /// **Z21a: the record read against this build** — nothing, this build, or
    /// another build after a rollback that did not finish.
    #[test]
    fn the_record_is_read_against_this_builds_own_version() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-probe-asks-{}", std::process::id()));
        let path = folder.join("local-record.sqlite3");
        let record = LocalRecord::open_at(&path).unwrap();
        let connection = record.connection();
        assert_eq!(what_the_record_asks(connection, "0.2.0").unwrap(), WhatTheRecordAsks::Nothing);
        let probe = a_probe_record("0.2.0", &path, &folder.join(".Smoozy previous.app"));
        update_trust::an_update_goes_into_probe(connection, &probe).unwrap();
        assert_eq!(
            what_the_record_asks(connection, "0.2.0").unwrap(),
            WhatTheRecordAsks::ThisBuildIsTheRelease(probe.clone())
        );
        assert_eq!(
            what_the_record_asks(connection, "0.1.8").unwrap(),
            WhatTheRecordAsks::ARollbackDidNotFinish(probe)
        );
    }

    /// **An uncleared probe record is completed at the next start** (Z21a's
    /// fourth bullet): the copy and the backup go, then the record — and a
    /// second launch finds nothing to do. **A backup this program runs from,
    /// or one the record does not name as this copy's own, is never removed.**
    // Windows clears its recorded build folders through the side-by-side route.
    #[cfg(not(target_os = "windows"))]
    #[test]
    fn an_uncleared_probe_record_is_completed_at_the_next_start() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-probe-clear-{}", std::process::id()));
        let path = folder.join("local-record.sqlite3");
        let backup = folder.join(".Smoozy previous.app");
        let record = LocalRecord::open_at(&path).unwrap();
        std::fs::create_dir_all(backup.join("Contents")).unwrap();
        record::take_the_copy(&path).unwrap();
        let probe = a_probe_record("0.2.0", &path, &backup);
        update_trust::an_update_goes_into_probe(record.connection(), &probe).unwrap();

        // The backup the program runs from stays; the rest goes.
        what_the_record_kept_is_cleared(record.connection(), &path, &probe, Some(&backup), Some(&backup)).unwrap();
        assert!(backup.exists(), "a build removed the bundle it runs from");
        assert!(!record::copy_beside(&path).exists(), "L8's copy was not discarded");
        assert_eq!(update_trust::the_update_in_probe(record.connection()).unwrap(), None);

        // The ordinary case: the backup goes too.
        update_trust::an_update_goes_into_probe(record.connection(), &probe).unwrap();
        what_the_record_kept_is_cleared(record.connection(), &path, &probe, Some(&backup), None).unwrap();
        assert!(!backup.exists(), "Z7's backup outlived the record");
        assert_eq!(what_the_record_asks(record.connection(), "0.2.0").unwrap(), WhatTheRecordAsks::Nothing);

        // A record naming a place that is not this copy's own removes nothing.
        let elsewhere = folder.join("someone else's.app");
        std::fs::create_dir_all(&elsewhere).unwrap();
        let odd = a_probe_record("0.2.0", &path, &elsewhere);
        update_trust::an_update_goes_into_probe(record.connection(), &odd).unwrap();
        what_the_record_kept_is_cleared(record.connection(), &path, &odd, Some(&backup), None).unwrap();
        assert!(elsewhere.exists(), "a folder was removed because a record named it");
    }

    /// The variables that make the test below a probe, and what it is to do.
    pub(crate) const THE_PROBE_CHILDS_FOLDER: &str = "SMOOZY_TEST_THE_PROBE_CHILDS_FOLDER";
    pub(crate) const THE_PROBE_CHILDS_VERSION: &str = "SMOOZY_TEST_THE_PROBE_CHILDS_VERSION";
    pub(crate) const THE_PROBE_CHILDS_ACTS: &str = "SMOOZY_TEST_THE_PROBE_CHILDS_ACTS";

    /// **Not a test on its own**: it does nothing unless a test started it as
    /// a probe, and then it is one — the hold waited for at the test's folder,
    /// the record opened there, its own version read against the record, and
    /// then what it was asked to do:
    ///
    /// - `starts` — the token said; then, once let go, the record cleared.
    /// - `migrates-and-fails` — L8's copy taken and a write made, as a
    ///   migration would, then an ending with no token.
    /// - `hangs` — nothing said, and a wait that only being ended ends.
    #[test]
    fn the_probe_child() {
        let Some(folder) = std::env::var_os(THE_PROBE_CHILDS_FOLDER) else {
            return;
        };
        let folder = std::path::PathBuf::from(folder);
        let version = std::env::var(THE_PROBE_CHILDS_VERSION).unwrap_or_default();
        let acts = std::env::var(THE_PROBE_CHILDS_ACTS).unwrap_or_default();
        let token = this_launch_is_a_probe().expect("started as a probe");
        if std::env::var_os("SMOOZY_TEST_THE_PROBE_CHILD_WANTS_THE_MODE").is_some() {
            assert_eq!(
                crate::start::LaunchMode::from_arguments(std::env::args_os().skip(1)),
                crate::start::LaunchMode::Update,
                "the probe was not started in the update-launch mode"
            );
        }
        let held = the_one_instance::the_hold_is_waited_for_at(
            &folder.join("running-copy.lock"),
            &folder.join("running-copy.door"),
        );
        assert!(matches!(held, the_one_instance::TheLaunch::Runs(_)), "the probe has the hold");
        let path = folder.join("local-record.sqlite3");
        let record = LocalRecord::open_at(&path).expect("the probe opens the record");
        let asks = what_the_record_asks(record.connection(), &version).unwrap();
        match acts.as_str() {
            "starts" => {
                let WhatTheRecordAsks::ThisBuildIsTheRelease(probe) = asks else {
                    std::process::exit(4);
                };
                the_token_is_said(&token, &mut std::io::stdout().lock()).unwrap();
                the_watcher_lets_go(&mut std::io::stdin().lock());
                what_the_record_kept_is_cleared(
                    record.connection(),
                    &path,
                    &probe,
                    Some(&folder.join("Apps/.Smoozy previous.app")),
                    None,
                )
                .unwrap();
                std::process::exit(0);
            }
            "migrates-and-fails" => {
                record::take_the_copy(&path).unwrap();
                update_trust::this_version_was_declined(record.connection(), "the probe's own write").unwrap();
                std::process::exit(3);
            }
            "hangs" => {
                the_watcher_lets_go(&mut std::io::stdin().lock());
                std::process::exit(5);
            }
            _ => std::process::exit(6),
        }
    }

    /// **This test program, re-entered as one probe** — the real form's
    /// process shape, with the test's folder in place of a bundle.
    pub(crate) fn the_probe_child_command(folder: &Path, version: &str, acts: &str) -> Command {
        let mut command = Command::new(std::env::current_exe().expect("this test program"));
        command
            .args(["--exact", "updating::the_probe::tests::the_probe_child", "--nocapture", "--test-threads=1"])
            .env(THE_PROBE_CHILDS_FOLDER, folder)
            .env(THE_PROBE_CHILDS_VERSION, version)
            .env(THE_PROBE_CHILDS_ACTS, acts);
        command
    }

    /// **The watch: the token is a successful start, an ending without it is a
    /// failed one, and so is silence past the deadline.**
    #[test]
    fn the_watch_tells_a_start_from_an_ending_and_from_silence() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-probe-watch-{}", std::process::id()));
        let path = folder.join("local-record.sqlite3");
        let record = LocalRecord::open_at(&path).unwrap();
        update_trust::an_update_goes_into_probe(
            record.connection(),
            &a_probe_record("0.2.0", &path, &folder.join("Apps/.Smoozy previous.app")),
        )
        .unwrap();

        let token = a_token().unwrap();
        let mut probe = AProbeUnderWatch::starts(the_probe_child_command(&folder, "0.2.0", "starts"), &token).unwrap();
        assert_eq!(probe.is_watched(Duration::from_secs(30)), WhatTheProbeCameTo::ItStarted);
        let mut child = probe.is_let_go();
        assert!(child.wait().unwrap().success(), "the probe did not clear and end once let go");
        assert_eq!(update_trust::the_update_in_probe(record.connection()).unwrap(), None);

        // A probe that finds a record naming another build says nothing.
        update_trust::an_update_goes_into_probe(
            record.connection(),
            &a_probe_record("0.1.8", &path, &folder.join("Apps/.Smoozy previous.app")),
        )
        .unwrap();
        let mut probe = AProbeUnderWatch::starts(the_probe_child_command(&folder, "0.2.0", "starts"), &token).unwrap();
        assert!(matches!(probe.is_watched(Duration::from_secs(30)), WhatTheProbeCameTo::ItEndedFirst(_)));

        let mut probe = AProbeUnderWatch::starts(the_probe_child_command(&folder, "0.2.0", "hangs"), &token).unwrap();
        assert_eq!(probe.is_watched(Duration::from_millis(1500)), WhatTheProbeCameTo::TheDeadlinePassed);
        probe.is_ended();
    }
}

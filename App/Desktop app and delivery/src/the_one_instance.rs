//! **One running copy per user** — P2, built at step 11's R1. Until then
//! nothing kept it, and a second launch started a second copy writing the same
//! local record.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Reporting a problem` U5a.
//!
//! **What it is.** The first thing an ordinary launch does after the askpass
//! branch is take a hold: an exclusive lock on one empty file in Smoozy's own
//! folder ([`Location::TheRunningCopysHold`]). **Taken**, the launch is the
//! running copy and goes on. **Not taken**, another copy holds it: the launch
//! asks that copy's window to come forward and exits, having opened no record,
//! swept nothing and built no window — so it does no part of `Reporting a
//! problem` U3 step 1, which is what that step and `Reporting a problem` C1's
//! first condition ask.
//!
//! **Per user, because the folder is.** The file sits in the per-user
//! application-data folder, so two users on one computer each have their own,
//! and a throwaway copy pointed at its own folder has its own too.
//!
//! **Why a file lock, and the outside fact it rests on** (`Reporting a problem`
//! §12 item 12). The lock is the standard library's `File::try_lock`, which
//! its own documentation says is `flock` with `LOCK_EX` and `LOCK_NB` on Unix
//! and `LockFileEx` with `LOCKFILE_EXCLUSIVE_LOCK` and
//! `LOCKFILE_FAIL_IMMEDIATELY` on Windows, released when the file is closed.
//! **Each system's own publication says the lock goes when the process dies
//! without running any code of its own** — macOS's manual pages for `close(2)`
//! and `_exit(2)`, Linux's for `flock(2)` and `_exit(2)` with POSIX's signal
//! actions, and Microsoft's pages for `LockFileEx` and process termination.
//! The words and the sources are in `Audits/Build/Step 11/R1, one running copy
//! — what landed.md`. **Windows adds one caveat of its own**: the unlocking
//! after a process ends *"depends upon available system resources"*, so a
//! launch in the moment straight after a crash can find the hold still taken.
//! That launch raises nothing and exits, and the next launch runs.
//!
//! **The descriptor is never inherited.** The standard library opens every file
//! close-on-exec, so neither the askpass helper nor any other child keeps the
//! hold alive after Smoozy is gone.
//!
//! **How the second launch reaches the first** ([`Location::TheRunningCopysDoor`]).
//! - **macOS and Linux:** a named pipe in the same folder. The running copy
//!   holds it open at both ends, so a knock never finds nobody there, and one
//!   thread waits on it with a plain blocking read — no timer and no poll. A
//!   knock is one byte; the running copy brings its own window forward through
//!   the shell.
//! - **Windows:** a file naming the running copy's process. The second launch
//!   reads it and brings that process's window forward itself, which Windows
//!   permits a process the user has just started.
//!
//! **A launch that cannot try the hold runs anyway** — a folder the system will
//! not name, a file that will not open, a disk that does not do locks. A
//! Smoozy with no window would be Smoozy going quiet. What it cannot claim is
//! `Reporting a problem` C1's first condition, so [`ThisRunsHold`] says so and
//! the offer that condition guards stays closed.

use std::fs::{File, OpenOptions, TryLockError};
use std::path::Path;

use crate::platform::files;
use crate::platform::locations::{self, Location};

/// **The hold, kept for the whole run.** Dropping it, or the process ending in
/// any way at all, lets it go.
pub struct TheHold {
    /// The locked file. Never read and never written.
    the_held_file: File,
    /// How a second launch reaches this copy, where it could be made.
    door: Option<TheDoor>,
}

/// **What the running copy keeps for its whole run**, in the shell's own
/// managed state, so the hold lives exactly as long as the application.
pub struct ThisRunsHold {
    the_hold: Option<TheHold>,
    /// Why the hold could not be tried, where it could not.
    pub trouble: Option<String>,
}

impl ThisRunsHold {
    /// What a launch that is the running copy keeps.
    pub fn held(hold: TheHold) -> Self {
        ThisRunsHold {
            the_hold: Some(hold),
            trouble: None,
        }
    }

    /// What a launch that runs unchecked keeps.
    pub fn unchecked(trouble: String) -> Self {
        ThisRunsHold {
            the_hold: None,
            trouble: Some(trouble),
        }
    }

    /// **`Reporting a problem` C1's first condition**: this run holds the one
    /// running copy's hold. The reader is R5's offer.
    pub fn this_run_is_the_one_running_copy(&self) -> bool {
        self.the_hold.is_some()
    }

    /// The hold itself, for the knock answering and for the update's handover.
    pub fn the_hold(&mut self) -> Option<&mut TheHold> {
        self.the_hold.as_mut()
    }

    /// **The hold handed out of this run's keeping**, for P2's handover — the
    /// Mac update route takes it here and gives it to
    /// [`the_hold_is_handed_to_the_probe`]. Nothing, where this run never held
    /// it or has handed it out already.
    pub fn the_hold_is_handed_out(&mut self) -> Option<TheHold> {
        self.the_hold.take()
    }

    /// **The hold back in this run's keeping**, after a failed start took it
    /// back (P2's last clause).
    pub fn the_hold_is_kept_again(&mut self, hold: TheHold) {
        self.the_hold = Some(hold);
    }
}

/// What trying the hold came to.
pub enum TheLaunch {
    /// Taken: this launch is the running copy.
    Runs(TheHold),
    /// Another copy holds it.
    AnotherCopyRuns,
    /// The hold could not be tried at all, in the system's own words.
    CouldNotBeTried(String),
}

/// What this launch does.
pub enum ThisLaunch {
    /// It is the one running copy.
    IsTheRunningCopy(TheHold),
    /// It runs without having been able to try the hold, and why.
    RunsUnchecked(String),
    /// Another copy runs; it has been asked to come forward, and this launch
    /// ends here.
    Ends,
}

/// **The decision, with both of its outside acts handed in**, so a test can
/// hold it without starting a second Smoozy.
pub fn this_launch_is_decided(
    take_the_hold: impl FnOnce() -> TheLaunch,
    ask_the_running_copy: impl FnOnce(),
) -> ThisLaunch {
    match take_the_hold() {
        TheLaunch::Runs(hold) => ThisLaunch::IsTheRunningCopy(hold),
        TheLaunch::CouldNotBeTried(trouble) => ThisLaunch::RunsUnchecked(trouble),
        TheLaunch::AnotherCopyRuns => {
            ask_the_running_copy();
            ThisLaunch::Ends
        }
    }
}

/// **The hold, tried in Smoozy's own folder** — F11's two places for it.
pub fn the_launch_takes_the_hold() -> TheLaunch {
    let places = (
        locations::resolve(Location::PerUserApplicationData),
        locations::resolve(Location::TheRunningCopysHold),
        locations::resolve(Location::TheRunningCopysDoor),
    );
    let (folder, hold, door) = match places {
        (Ok(folder), Ok(hold), Ok(door)) => (folder, hold, door),
        (Err(not_named), _, _) | (_, Err(not_named), _) | (_, _, Err(not_named)) => {
            return TheLaunch::CouldNotBeTried(not_named.reported)
        }
    };
    if let Err(failure) = files::ensure_directory(&folder) {
        return TheLaunch::CouldNotBeTried(failure.reported);
    }
    the_hold_is_tried_at(&hold, &door)
}

/// **The hold, tried at a named place.** This exists so it can be tested away
/// from the real folder; every rule above is exercised through it.
pub fn the_hold_is_tried_at(hold: &Path, door: &Path) -> TheLaunch {
    // **Opened, never truncated and never written**: a second launch opening it
    // must not disturb the copy that holds it. Windows wants it open for
    // writing before it will lock it.
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(hold)
    {
        Ok(file) => file,
        Err(error) => return TheLaunch::CouldNotBeTried(error.to_string()),
    };
    match file.try_lock() {
        Ok(()) => TheLaunch::Runs(TheHold {
            the_held_file: file,
            door: the_door_is_made(door).ok(),
        }),
        Err(TryLockError::WouldBlock) => TheLaunch::AnotherCopyRuns,
        Err(TryLockError::Error(error)) => TheLaunch::CouldNotBeTried(error.to_string()),
    }
}

/// **The hold waited for, in Smoozy's own folder** — what a launch the update
/// mechanism started does instead of [`the_launch_takes_the_hold`]: the probe
/// is started before the old process hands the hold over (P2), and the build a
/// rollback starts is started before the old process has gone, so each must
/// wait for the hold rather than find it taken and leave. **Only an
/// update-launch-mode launch waits** (`Reporting a problem` U5a and
/// `Reporting a problem` U5b); an ordinary launch never does.
pub fn the_launch_waits_for_the_hold() -> TheLaunch {
    let places = (
        locations::resolve(Location::PerUserApplicationData),
        locations::resolve(Location::TheRunningCopysHold),
        locations::resolve(Location::TheRunningCopysDoor),
    );
    let (folder, hold, door) = match places {
        (Ok(folder), Ok(hold), Ok(door)) => (folder, hold, door),
        (Err(not_named), _, _) | (_, Err(not_named), _) | (_, _, Err(not_named)) => {
            return TheLaunch::CouldNotBeTried(not_named.reported)
        }
    };
    if let Err(failure) = files::ensure_directory(&folder) {
        return TheLaunch::CouldNotBeTried(failure.reported);
    }
    the_hold_is_waited_for_at(&hold, &door)
}

/// **The hold waited for at a named place** — the standard library's
/// `File::lock`, whose own page says *"Blocks until the lock can be acquired"*
/// and that it *"currently corresponds to the `flock` function on Unix with the
/// `LOCK_EX` flag, and the `LockFileEx` function on Windows with the
/// `LOCKFILE_EXCLUSIVE_LOCK` flag"* (read 2026-10-25). The handover's other
/// half: the old process's [`the_hold_is_handed_to_the_probe`] is what ends the
/// wait, and so does that process ending in any way at all.
pub fn the_hold_is_waited_for_at(hold: &Path, door: &Path) -> TheLaunch {
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(hold)
    {
        Ok(file) => file,
        Err(error) => return TheLaunch::CouldNotBeTried(error.to_string()),
    };
    match file.lock() {
        Ok(()) => TheLaunch::Runs(TheHold {
            the_held_file: file,
            door: the_door_is_made(door).ok(),
        }),
        Err(error) => TheLaunch::CouldNotBeTried(error.to_string()),
    }
}

/// **The second launch's one act: the running copy is asked to come forward.**
/// Nothing reports what it came to — a copy that could not be reached is one
/// that is starting or ending, and either way this launch exits.
pub fn the_running_copy_is_asked_to_come_forward() {
    if let Ok(door) = locations::resolve(Location::TheRunningCopysDoor) {
        let _ = the_copy_behind_this_door_is_asked(&door);
    }
}

impl TheHold {
    /// **A knock at the door runs `come_forward`**, on one thread of its own
    /// that waits on a blocking read. Called once, after the window exists.
    /// On Windows there is nothing to answer: the second launch does the
    /// raising itself.
    pub fn knocks_are_answered_by(&mut self, come_forward: impl Fn() + Send + 'static) {
        #[cfg(unix)]
        if let Some(door) = self.door.as_mut() {
            if let Some(mut reading) = door.reading.take() {
                let _ = std::thread::Builder::new()
                    .name("smoozy second launch".to_string())
                    .spawn(move || {
                        use std::io::Read;
                        let mut knocks = [0u8; 64];
                        loop {
                            match reading.read(&mut knocks) {
                                Ok(0) => return,
                                Ok(_) => come_forward(),
                                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                                Err(_) => return,
                            }
                        }
                    });
            }
        }
        #[cfg(not(unix))]
        let _ = come_forward;
    }
}

/// **P2's handover** — shaped at R1, and called by step 11's Mac update route
/// (`crate::updating::the_mac`) once the old process has stopped writing and the
/// probe has been started.
///
/// P2: in an update the old process *"stops writing and hands the instance
/// lock to the probe before the probe runs"* the record's migration. **This is
/// the handing**: the hold is let go and nothing of this copy's is left on it,
/// so the probe's own start takes it by the ordinary route. Three things are the
/// caller's and not this function's:
///
/// - **The old process has already stopped writing** when it calls this. Taking
///   the hold by value is what makes it the last thing done with it.
/// - **The probe must wait for the hold rather than give up**, because it is
///   started before this is called. A probe-mode launch takes it with
///   [`the_launch_waits_for_the_hold`] instead of the ordinary launch's
///   `try_lock`, and only an update-launch-mode launch does (`Reporting a
///   problem` U5a).
/// - **Taking it back on a failed start** is this module's ordinary route, run
///   by whoever P2 names — the old process on macOS and Linux, Z19's relaunch
///   helper on Windows — once the probe has been ended.
///
/// **The door goes with the hold**, so a second launch during the handover
/// finds no one to knock for. The thread already waiting on it may still
/// answer one knock until this process exits; that raises a window about to
/// close and nothing else.
pub fn the_hold_is_handed_to_the_probe(hold: TheHold) {
    let TheHold {
        the_held_file,
        door,
    } = hold;
    let _ = the_held_file.unlock();
    drop(the_held_file);
    drop(door);
}

/// **The running copy's own window brought forward** — what a knock does on
/// macOS and Linux. Unhidden first on a Mac, where *Hide Smoozy* hides the
/// whole application rather than the window.
pub fn the_window_comes_forward<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    use tauri::Manager;
    #[cfg(target_os = "macos")]
    let _ = app.show();
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

// ---------------------------------------------------------------------------
// The door: macOS and Linux
// ---------------------------------------------------------------------------

/// The named pipe, held open at both ends by the running copy.
#[cfg(unix)]
struct TheDoor {
    /// Handed to the answering thread once.
    reading: Option<File>,
    /// Never written. Held so the pipe always has a writer, and a read never
    /// ends because the last knock's writer closed.
    _kept_open: File,
}

#[cfg(unix)]
fn the_door_is_made(path: &Path) -> Result<TheDoor, String> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::{FileTypeExt, OpenOptionsExt};
    use std::os::unix::io::AsRawFd;

    let name = CString::new(path.as_os_str().as_bytes()).map_err(|error| error.to_string())?;
    // SAFETY: `name` is a valid C string for the length of the call.
    if unsafe { libc::mkfifo(name.as_ptr(), 0o600) } != 0 {
        let error = std::io::Error::last_os_error();
        if error.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(error.to_string());
        }
    }
    if !std::fs::symlink_metadata(path)
        .map_err(|error| error.to_string())?
        .file_type()
        .is_fifo()
    {
        return Err("the door is not a named pipe".to_string());
    }
    // **Opened for reading without waiting**, then for writing, which finds a
    // reader and so does not wait either; then the reading end is made to wait
    // on a read again.
    let reading = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(path)
        .map_err(|error| error.to_string())?;
    let kept_open = OpenOptions::new()
        .write(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    let fd = reading.as_raw_fd();
    // SAFETY: `fd` is open for the length of both calls.
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFL);
        if flags == -1 || libc::fcntl(fd, libc::F_SETFL, flags & !libc::O_NONBLOCK) == -1 {
            return Err(std::io::Error::last_os_error().to_string());
        }
    }
    Ok(TheDoor {
        reading: Some(reading),
        _kept_open: kept_open,
    })
}

/// **One knock.** Opened without waiting, so a door nobody holds answers at
/// once rather than hanging this launch; and only a named pipe is written to.
#[cfg(unix)]
fn the_copy_behind_this_door_is_asked(path: &Path) -> Result<(), String> {
    use std::io::Write;
    use std::os::unix::fs::{FileTypeExt, OpenOptionsExt};

    if !std::fs::symlink_metadata(path)
        .map_err(|error| error.to_string())?
        .file_type()
        .is_fifo()
    {
        return Err("the door is not a named pipe".to_string());
    }
    let mut door = OpenOptions::new()
        .write(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(path)
        .map_err(|error| error.to_string())?;
    door.write_all(b"\n").map_err(|error| error.to_string())
}

// ---------------------------------------------------------------------------
// The door: Windows
// ---------------------------------------------------------------------------

/// Nothing is held open: the door is a file naming this process.
#[cfg(windows)]
struct TheDoor;

#[cfg(windows)]
fn the_door_is_made(path: &Path) -> Result<TheDoor, String> {
    let written = files::write_whole_file(
        path,
        std::process::id().to_string().as_bytes(),
        files::Durability::Ordinary,
    );
    if written.wrote_the_file() {
        Ok(TheDoor)
    } else {
        Err("the door could not be written".to_string())
    }
}

/// **The running copy's window brought forward from here.** The top-level
/// window of that process that has no owner and carries Smoozy's own title is
/// shown if hidden, restored if minimised, and brought to the foreground — the
/// title is what tells it from the shell's own unseen helper windows.
/// `SetForegroundWindow`,
/// which Microsoft permits when *"the calling process was started by the
/// foreground process"*, the case of a launch the user just made.
#[cfg(windows)]
fn the_copy_behind_this_door_is_asked(path: &Path) -> Result<(), String> {
    use windows_sys::core::BOOL;
    use windows_sys::Win32::Foundation::{HWND, LPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindow, GetWindowTextW, GetWindowThreadProcessId, IsIconic,
        SetForegroundWindow, ShowWindowAsync, GW_OWNER, SW_RESTORE, SW_SHOW,
    };

    /// The title `main.rs` gives the one window.
    const THE_TITLE: &str = "Smoozy";

    struct TheSearch {
        process: u32,
        found: Vec<HWND>,
    }

    unsafe extern "system" fn each(window: HWND, search: LPARAM) -> BOOL {
        // SAFETY: `search` is the `TheSearch` handed to `EnumWindows` below,
        // alive for the whole enumeration.
        let search = unsafe { &mut *(search as *mut TheSearch) };
        let mut process = 0u32;
        unsafe { GetWindowThreadProcessId(window, &mut process) };
        if process != search.process || !unsafe { GetWindow(window, GW_OWNER) }.is_null() {
            return 1;
        }
        let mut title = [0u16; 64];
        let length = unsafe { GetWindowTextW(window, title.as_mut_ptr(), title.len() as i32) };
        if length > 0 && String::from_utf16_lossy(&title[..length as usize]) == THE_TITLE {
            search.found.push(window);
        }
        1
    }

    let process: u32 = std::fs::read_to_string(path)
        .map_err(|error| error.to_string())?
        .trim()
        .parse()
        .map_err(|_| "the door names no process".to_string())?;
    if process == std::process::id() {
        return Err("the door names this launch".to_string());
    }
    let mut search = TheSearch {
        process,
        found: Vec::new(),
    };
    // SAFETY: `each` is called only during this call, with the pointer given.
    unsafe { EnumWindows(Some(each), &mut search as *mut TheSearch as LPARAM) };
    if search.found.is_empty() {
        return Err("the running copy has no window yet".to_string());
    }
    for window in search.found {
        // SAFETY: plain calls on a window handle; a handle that has gone since
        // the enumeration makes each of them fail and nothing else.
        unsafe {
            // **Asynchronous**, so a running copy whose window is busy cannot
            // hold this launch open.
            ShowWindowAsync(window, if IsIconic(window) != 0 { SW_RESTORE } else { SW_SHOW });
            SetForegroundWindow(window);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scratch_folders::a_folder;

    fn the_two_places(test: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let folder = a_folder(format!("smoozy-one-copy-{test}-{}", std::process::id()));
        (folder.join("running-copy.lock"), folder.join("running-copy.door"))
    }

    fn runs(launch: &TheLaunch) -> bool {
        matches!(launch, TheLaunch::Runs(_))
    }

    fn another_copy_runs(launch: &TheLaunch) -> bool {
        matches!(launch, TheLaunch::AnotherCopyRuns)
    }

    /// **A second launch in the same user session does not run.** The first
    /// takes the hold; the second, at the same place, finds it taken; once the
    /// first lets go, a launch runs again.
    #[test]
    fn a_second_launch_does_not_run() {
        let (hold, door) = the_two_places("second");
        let first = the_hold_is_tried_at(&hold, &door);
        assert!(runs(&first), "the first launch takes the hold");

        let second = the_hold_is_tried_at(&hold, &door);
        assert!(another_copy_runs(&second), "a second launch finds the hold taken");

        drop(first);
        assert!(
            runs(&the_hold_is_tried_at(&hold, &door)),
            "with the first copy gone, a launch runs"
        );
    }

    /// **A launch that finds the hold taken asks the running copy to come
    /// forward, once, and ends**; one that takes it asks nobody.
    #[test]
    fn a_second_launch_asks_and_ends() {
        let (hold, door) = the_two_places("decided");
        let mut asked = 0;
        let first = this_launch_is_decided(|| the_hold_is_tried_at(&hold, &door), || asked += 1);
        assert!(matches!(first, ThisLaunch::IsTheRunningCopy(_)));
        assert_eq!(asked, 0, "the running copy asks nobody");

        let second = this_launch_is_decided(|| the_hold_is_tried_at(&hold, &door), || asked += 1);
        assert!(matches!(second, ThisLaunch::Ends), "the second launch ends");
        assert_eq!(asked, 1, "the second launch asks the running copy once");

        let unchecked = this_launch_is_decided(
            || TheLaunch::CouldNotBeTried("no locks here".to_string()),
            || asked += 1,
        );
        assert!(matches!(unchecked, ThisLaunch::RunsUnchecked(_)));
        assert_eq!(asked, 1, "a launch that could not try the hold asks nobody");
    }

    /// **The decision comes before anything else a launch does.** `main.rs`
    /// decides before the shell is built and so before the start's sweep, the
    /// record and the run-mark — `Reporting a problem` U3 step 1's *no part of
    /// this step* — and after the askpass branch, which is never a launch.
    #[test]
    fn the_decision_comes_before_the_start() {
        let main = include_str!("main.rs");
        let at = |text: &str| main.find(text).unwrap_or_else(|| panic!("main.rs holds {text}"));
        let decided = at("the_one_instance::this_launch_is_decided(");
        assert!(at("keys::askpass::this_launch_is_the_helper()") < decided);
        assert!(decided < at("tauri::Builder::default()"));
        assert!(decided < at("start::start("));
    }

    /// **The first copy's window is asked to come forward.** A knock from a
    /// second launch reaches the copy that holds the hold — and reaches it
    /// even when it comes before that copy has started answering, because the
    /// pipe keeps it.
    #[cfg(unix)]
    #[test]
    fn the_running_copy_hears_the_second_launch() {
        let (hold, door) = the_two_places("knock");
        let TheLaunch::Runs(mut first) = the_hold_is_tried_at(&hold, &door) else {
            panic!("the first launch takes the hold");
        };
        assert!(another_copy_runs(&the_hold_is_tried_at(&hold, &door)));
        the_copy_behind_this_door_is_asked(&door).expect("the knock is delivered");

        let (heard, hearing) = std::sync::mpsc::channel();
        first.knocks_are_answered_by(move || {
            let _ = heard.send(());
        });
        hearing
            .recv_timeout(std::time::Duration::from_secs(5))
            .expect("the running copy was asked to come forward");

        the_copy_behind_this_door_is_asked(&door).expect("a second knock is delivered");
        hearing
            .recv_timeout(std::time::Duration::from_secs(5))
            .expect("and asked again");
    }

    /// **A door nobody holds answers at once**, rather than leaving the second
    /// launch waiting for a reader that will never come.
    #[cfg(unix)]
    #[test]
    fn a_door_nobody_holds_does_not_wait() {
        let (hold, door) = the_two_places("nobody");
        let first = the_hold_is_tried_at(&hold, &door);
        assert!(runs(&first));
        drop(first);
        assert!(the_copy_behind_this_door_is_asked(&door).is_err());
        assert!(the_copy_behind_this_door_is_asked(&hold).is_err(), "only a pipe is knocked on");
    }

    /// **P2's handover lets the hold go**, so the probe's own start takes it.
    #[test]
    fn the_handover_lets_the_hold_go() {
        let (hold, door) = the_two_places("handover");
        let TheLaunch::Runs(old) = the_hold_is_tried_at(&hold, &door) else {
            panic!("the old copy takes the hold");
        };
        the_hold_is_handed_to_the_probe(old);
        assert!(runs(&the_hold_is_tried_at(&hold, &door)), "the probe takes it");
    }

    /// **The update's handover, across two real processes' worth of opens**:
    /// a launch that waits for the hold is let in the moment the holder hands
    /// it over, and not before.
    #[test]
    fn a_waiting_launch_is_let_in_when_the_hold_is_handed_over() {
        let (hold, door) = the_two_places("waits");
        let TheLaunch::Runs(old) = the_hold_is_tried_at(&hold, &door) else {
            panic!("the old copy takes the hold");
        };
        let mut keeping = ThisRunsHold::held(old);
        let (got, getting) = std::sync::mpsc::channel();
        let (waiting_hold, waiting_door) = (hold.clone(), door.clone());
        let probe = std::thread::spawn(move || {
            let launch = the_hold_is_waited_for_at(&waiting_hold, &waiting_door);
            let _ = got.send(runs(&launch));
            launch
        });
        assert!(
            getting.recv_timeout(std::time::Duration::from_millis(500)).is_err(),
            "the waiting launch got in while the old copy held the hold"
        );
        the_hold_is_handed_to_the_probe(keeping.the_hold_is_handed_out().expect("held"));
        assert!(!keeping.this_run_is_the_one_running_copy(), "the hold is handed out");
        assert_eq!(
            getting.recv_timeout(std::time::Duration::from_secs(5)),
            Ok(true),
            "the waiting launch was let in"
        );
        drop(probe.join().unwrap());
        let TheLaunch::Runs(back) = the_hold_is_tried_at(&hold, &door) else {
            panic!("taken back once the probe is gone");
        };
        keeping.the_hold_is_kept_again(back);
        assert!(keeping.this_run_is_the_one_running_copy());
    }

    /// **The variable that makes the child test below a holder.** Its value is
    /// the folder.
    const THE_HOLDING_CHILDS_FOLDER: &str = "SMOOZY_TEST_THE_HOLDING_CHILDS_FOLDER";
    const HELD: &str = "smoozy-one-copy: held";

    /// **Not a test on its own**: it does nothing unless the test below started
    /// it as a child, and then it takes the hold, says so, and waits on its
    /// input until it is killed.
    #[cfg(unix)]
    #[test]
    fn the_holding_child() {
        let Some(folder) = std::env::var_os(THE_HOLDING_CHILDS_FOLDER) else {
            return;
        };
        let folder = std::path::PathBuf::from(folder);
        let launch =
            the_hold_is_tried_at(&folder.join("running-copy.lock"), &folder.join("running-copy.door"));
        if runs(&launch) {
            println!("{HELD}");
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line);
        }
        drop(launch);
    }

    /// **The hold survives nothing after a hard kill** — `Reporting a problem`
    /// §12 item 12, observed. A child process takes the hold and is killed
    /// outright, with no chance to run anything of its own; the next launch
    /// takes the hold.
    ///
    /// **Unix only**, because the kill is `SIGKILL` there; on Windows Microsoft
    /// documents a delay before the lock goes, and a test that raced it would
    /// prove nothing.
    #[cfg(unix)]
    #[test]
    fn a_hard_kill_leaves_no_hold_behind() {
        use std::io::BufRead;
        use std::process::{Command, Stdio};

        let (hold, door) = the_two_places("killed");
        let folder = hold.parent().expect("a folder").to_path_buf();
        let mut child = Command::new(std::env::current_exe().expect("this test program"))
            .args(["--exact", "the_one_instance::tests::the_holding_child", "--nocapture"])
            .env(THE_HOLDING_CHILDS_FOLDER, &folder)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("the child starts");

        let said = std::io::BufReader::new(child.stdout.take().expect("its output"))
            .lines()
            .map_while(Result::ok)
            .find(|line| line.contains(HELD));
        assert!(said.is_some(), "the child took the hold");
        assert!(
            another_copy_runs(&the_hold_is_tried_at(&hold, &door)),
            "while the child lives, a launch finds the hold taken"
        );

        child.kill().expect("the child is killed outright");
        let ended = child.wait().expect("the child is gone");
        assert!(!ended.success(), "the child was killed, not ended");

        assert!(
            runs(&the_hold_is_tried_at(&hold, &door)),
            "after a hard kill, the next launch takes the hold"
        );
    }
}

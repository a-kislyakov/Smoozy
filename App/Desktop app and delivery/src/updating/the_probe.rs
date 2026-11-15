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
//! probe with the update-launch argument (`--smoozy-update-launch`, R5's one
//! mark; `crate::start::LaunchMode`) and the token in the variable
//! [`THE_PROBES_TOKEN`] — **the token is the probe mode**: the update-launch
//! argument alone is the build a rollback starts (Z21b's last bullet), and the
//! argument with the token is the probe. On Linux and Windows the probe is
//! the watcher's own child, and its output and input are two pipes back to
//! it ([`AProbeUnderWatch::starts`]).
//!
//! **On a Mac, from 2026-11-09, the probe is started through the system's own
//! app launcher** ([`AProbeUnderWatch::launched`]; the Creator's
//! twenty-eighth notes, note 5): `NSWorkspace`'s
//! `openApplicationAtURL:configuration:completionHandler:`, with a new
//! instance asked for, the argument, and the token in the environment. A
//! program the launcher never started is what left a second Smoozy in the
//! Dock, as Part A of those notes reads it. The launched program is no child,
//! so **the token and the let-go travel over two named pipes** in E18's
//! staging folder, named in [`THE_PROBES_TOKEN_PIPE`] and
//! [`THE_PROBES_LET_GO_PIPE`]: the probe writes its token line into the
//! first, and the second's closing is the input ending. **Whether the probe
//! ended early is read from the process number the launcher hands back**,
//! and a failed start ends that process. A probe started the old way, by a
//! build from before this change, finds neither variable and uses its own
//! output and input, as before.
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

/// **The named pipe the probe writes its token line into**, where the system's
/// launcher started it (macOS, from 2026-11-09). Absent, the token goes on the
/// probe's own output.
pub const THE_PROBES_TOKEN_PIPE: &str = "SMOOZY_THE_PROBES_TOKEN_PIPE";

/// **The named pipe whose closing lets the probe go**, where the system's
/// launcher started it. Absent, the probe's own input is waited on.
pub const THE_PROBES_LET_GO_PIPE: &str = "SMOOZY_THE_PROBES_LET_GO_PIPE";

/// **Each of the probe's variables, handed empty** — what the build a rollback
/// starts is given through the system's launcher, which hands the launched app
/// the caller's own environment too (observed at the desk, 2026-11-09). An
/// empty token is no token ([`this_launch_is_a_probe`]).
pub const NONE_OF_THE_PROBES_VARIABLES: [(&str, &str); 3] =
    [(THE_PROBES_TOKEN, ""), (THE_PROBES_TOKEN_PIPE, ""), (THE_PROBES_LET_GO_PIPE, "")];

/// The two pipes' names in the staging folder.
const THE_TOKEN_PIPES_NAME: &str = "probe-token";
const THE_LET_GO_PIPES_NAME: &str = "probe-let-go";
/// **This attempt's token, left beside the two pipes** for the probe to hold
/// its own against ([`this_attempt_is_the_one_awaited`]).
const THE_ATTEMPTS_NAME: &str = "probe-attempt";

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
    probe: TheProbe,
    lines: mpsc::Receiver<String>,
    expected: String,
}

/// **How the probe was started**, which is how it is ended and let go.
enum TheProbe {
    /// The watcher's own child, its output and input piped back (Linux,
    /// Windows, and the tests).
    AChild(Child),
    /// **Started through the system's launcher** (macOS): its process number,
    /// and the watcher's own ends of the two pipes, held open for the watch.
    #[cfg(unix)]
    Launched {
        process: i32,
        token_pipe: std::fs::File,
        let_go: std::fs::File,
    },
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
            probe: TheProbe::AChild(child),
            lines,
            expected: format!("{THE_TOKENS_LINE}{token}"),
        })
    }

    /// **Started through the system's own app launcher** (macOS, the
    /// twenty-eighth notes, note 5): the two pipes made in `folder` and held
    /// open here, read and write, so the probe's own opening of either never
    /// waits; `opens` asked to start `bundle` with `arguments` and the token
    /// and both pipes in its environment, and to hand back the process number.
    /// `opens` is [`the_bundle_is_opened`] in the running application.
    #[cfg(unix)]
    pub fn launched(
        bundle: &Path,
        arguments: &[&str],
        token: &str,
        folder: &Path,
        opens: &dyn Fn(&Path, &[&str], &[(&str, &str)]) -> Result<i32, String>,
    ) -> Result<AProbeUnderWatch, String> {
        let token_path = folder.join(THE_TOKEN_PIPES_NAME);
        let let_go_path = folder.join(THE_LET_GO_PIPES_NAME);
        let token_pipe = a_pipe_held_open_at(&token_path)?;
        let let_go = a_pipe_held_open_at(&let_go_path)?;
        let reading = token_pipe.try_clone().map_err(|failed| failed.to_string())?;
        let (said, lines) = mpsc::channel();
        // **The token pipe, read line by line.** This side holds a writer
        // too, so the read never ends on its own; the watch's ending writes
        // one empty line to wake it after the receiver has gone.
        let _ = std::thread::Builder::new()
            .name("smoozy probe's token pipe".into())
            .spawn(move || {
                for line in std::io::BufReader::new(reading).lines().map_while(Result::ok) {
                    if said.send(line).is_err() {
                        return;
                    }
                }
            });
        let token_text = token_path.to_string_lossy().into_owned();
        let let_go_text = let_go_path.to_string_lossy().into_owned();
        // **This attempt's token, left for the probe before the launcher is
        // asked** ([`this_attempt_is_the_one_awaited`]). One that cannot be
        // left is a probe that is not started: an earlier attempt's file
        // could be standing, and this attempt's probe would be told no.
        let left = files::write_whole_file(
            &folder.join(THE_ATTEMPTS_NAME),
            token.as_bytes(),
            files::Durability::Ordinary,
        );
        if !left.wrote_the_file() {
            drop(lines);
            let _ = (&token_pipe).write_all(b"\n");
            return Err(left
                .first_error
                .map(|failed| failed.reported)
                .unwrap_or_else(|| "this attempt's token could not be left for the probe".into()));
        }
        let environment = [
            (THE_PROBES_TOKEN, token),
            (THE_PROBES_TOKEN_PIPE, token_text.as_str()),
            (THE_PROBES_LET_GO_PIPE, let_go_text.as_str()),
        ];
        let process = match opens(bundle, arguments, &environment) {
            Ok(process) => process,
            Err(failed) => {
                drop(lines);
                let _ = (&token_pipe).write_all(b"\n");
                return Err(failed);
            }
        };
        Ok(AProbeUnderWatch {
            probe: TheProbe::Launched { process, token_pipe, let_go },
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
            // **A launched probe is no child**, so nothing closes its pipe
            // when it ends: its process is asked after each short wait.
            let wait = match &self.probe {
                TheProbe::AChild(_) => left,
                #[cfg(unix)]
                TheProbe::Launched { .. } => left.min(THE_LOOK_AT_A_LAUNCHED_PROBE),
            };
            match self.lines.recv_timeout(wait) {
                // **The line ends with the token**: whatever the program wrote
                // before it on the same line without ending one is not the
                // probe's to answer for, and the token is random and used once.
                Ok(line) if line.trim_end().ends_with(&self.expected) => return WhatTheProbeCameTo::ItStarted,
                Ok(_) => continue,
                Err(mpsc::RecvTimeoutError::Timeout) => match &self.probe {
                    TheProbe::AChild(_) => return WhatTheProbeCameTo::TheDeadlinePassed,
                    #[cfg(unix)]
                    TheProbe::Launched { process, .. } => {
                        if !a_process_runs(*process) {
                            return WhatTheProbeCameTo::ItEndedFirst(format!("process {process} ended"));
                        }
                    }
                },
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    let how = match &mut self.probe {
                        TheProbe::AChild(child) => match child.wait() {
                            Ok(status) => status.to_string(),
                            Err(failed) => failed.to_string(),
                        },
                        #[cfg(unix)]
                        TheProbe::Launched { process, .. } => format!("process {process}'s pipe ended"),
                    };
                    return WhatTheProbeCameTo::ItEndedFirst(how);
                }
            }
        }
    }

    /// **A failed start: the probe is ended** (P2: *"the probe is ended before
    /// the actor that launched the probe takes the instance lock back"*), and
    /// waited for, so it holds nothing afterwards.
    pub fn is_ended(self) {
        let AProbeUnderWatch { probe, lines, .. } = self;
        match probe {
            TheProbe::AChild(mut child) => {
                drop(lines);
                let _ = child.kill();
                let _ = child.wait();
            }
            // **A launched probe is ended by its process number, and waited
            // for to a fixed deadline** — it is no child, so its going is
            // read rather than waited on.
            #[cfg(unix)]
            TheProbe::Launched { process, token_pipe, let_go } => {
                // SAFETY: a signal to a process number the launcher handed
                // back for this probe; nothing is read or written through it.
                unsafe {
                    libc::kill(process, libc::SIGKILL);
                }
                let deadline = Instant::now() + THE_WAIT_FOR_AN_ENDED_PROBE;
                while a_process_runs(process) && Instant::now() < deadline {
                    std::thread::sleep(THE_LOOK_AT_A_LAUNCHED_PROBE);
                }
                the_pipes_are_let_go(lines, token_pipe, let_go);
            }
        }
    }

    /// **A successful start: the probe is let go.** Its input closes here —
    /// and closes anyway when this process exits — which is what tells it to
    /// clear the record. The probe runs on; nothing here waits for it. The
    /// child, where it is one.
    pub fn is_let_go(self) -> Option<Child> {
        let AProbeUnderWatch { probe, lines, .. } = self;
        match probe {
            TheProbe::AChild(mut child) => {
                drop(lines);
                drop(child.stdin.take());
                Some(child)
            }
            #[cfg(unix)]
            TheProbe::Launched { token_pipe, let_go, .. } => {
                the_pipes_are_let_go(lines, token_pipe, let_go);
                None
            }
        }
    }
}

/// **How often a launched probe's process is asked after**, and how long an
/// ended one is waited for. Build's: a start is seconds, and a quarter of a
/// second is well inside the launch deadline.
#[cfg(unix)]
const THE_LOOK_AT_A_LAUNCHED_PROBE: Duration = Duration::from_millis(250);
#[cfg(unix)]
const THE_WAIT_FOR_AN_ENDED_PROBE: Duration = Duration::from_secs(10);

/// Whether a process by that number is there.
#[cfg(unix)]
fn a_process_runs(process: i32) -> bool {
    // SAFETY: signal 0 sends nothing; it asks only whether the number is a
    // process this user may signal.
    let asked = unsafe { libc::kill(process, 0) };
    asked == 0 || std::io::Error::last_os_error().raw_os_error() == Some(libc::EPERM)
}

/// **The watcher's ends of both pipes let go**: the let-go pipe closed, which
/// is the probe's input ending, and the token pipe's reader woken with one
/// empty line once nothing listens, so it ends too.
#[cfg(unix)]
fn the_pipes_are_let_go(lines: mpsc::Receiver<String>, token_pipe: std::fs::File, let_go: std::fs::File) {
    drop(let_go);
    drop(lines);
    let _ = (&token_pipe).write_all(b"\n");
}

/// **A named pipe made afresh at `path` and held open, read and write**, so
/// neither end's opening waits on the other.
#[cfg(unix)]
fn a_pipe_held_open_at(path: &Path) -> Result<std::fs::File, String> {
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::OpenOptionsExt;
    let _ = std::fs::remove_file(path);
    let name = std::ffi::CString::new(path.as_os_str().as_bytes()).map_err(|failed| failed.to_string())?;
    // SAFETY: `name` is a terminated string alive for the call.
    if unsafe { libc::mkfifo(name.as_ptr(), 0o600) } != 0 {
        return Err(format!("{}: {}", path.display(), std::io::Error::last_os_error()));
    }
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(path)
        .and_then(|file| {
            the_pipe_waits(&file)?;
            Ok(file)
        })
        .map_err(|failed| format!("{}: {failed}", path.display()))
}

/// The pipe's reads and writes wait again, once it is open.
#[cfg(unix)]
fn the_pipe_waits(file: &std::fs::File) -> std::io::Result<()> {
    use std::os::unix::io::AsRawFd;
    // SAFETY: the descriptor is open for the length of both calls.
    unsafe {
        let flags = libc::fcntl(file.as_raw_fd(), libc::F_GETFL);
        if flags < 0 || libc::fcntl(file.as_raw_fd(), libc::F_SETFL, flags & !libc::O_NONBLOCK) < 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(())
}

/// **`bundle` opened through the system's own app launcher** — `NSWorkspace`'s
/// `openApplicationAtURL:configuration:completionHandler:` (AppKit, read
/// 2026-11-09: *"Launches the app at the specified URL and asynchronously
/// reports back on the app's status"*), with `createsNewApplicationInstance`
/// set because the old copy is still running, `arguments` and `environment`.
/// The process number of the launched app, or the system's words for why
/// not. **The answer is waited for to a fixed deadline**, a minute.
#[cfg(target_os = "macos")]
pub fn the_bundle_is_opened(bundle: &Path, arguments: &[&str], environment: &[(&str, &str)]) -> Result<i32, String> {
    use block2::RcBlock;
    use objc2_app_kit::{NSRunningApplication, NSWorkspace, NSWorkspaceOpenConfiguration};
    use objc2_foundation::{NSArray, NSDictionary, NSError, NSString, NSURL};
    let url = NSURL::fileURLWithPath(&NSString::from_str(&bundle.to_string_lossy()));
    let configuration = NSWorkspaceOpenConfiguration::configuration();
    configuration.setCreatesNewApplicationInstance(true);
    configuration.setAddsToRecentItems(false);
    let arguments: Vec<_> = arguments.iter().map(|one| NSString::from_str(one)).collect();
    configuration.setArguments(&NSArray::from_retained_slice(&arguments));
    let keys: Vec<_> = environment.iter().map(|(key, _)| NSString::from_str(key)).collect();
    let values: Vec<_> = environment.iter().map(|(_, value)| NSString::from_str(value)).collect();
    let keys: Vec<&NSString> = keys.iter().map(|one| &**one).collect();
    let values: Vec<&NSString> = values.iter().map(|one| &**one).collect();
    configuration.setEnvironment(&NSDictionary::from_slices(&keys, &values));
    let (say, heard) = mpsc::channel::<Result<i32, String>>();
    let handler = RcBlock::new(move |launched: *mut NSRunningApplication, error: *mut NSError| {
        // SAFETY: the system hands both for the length of the call, either
        // possibly empty, and nothing here outlives it.
        let answer = match (unsafe { launched.as_ref() }, unsafe { error.as_ref() }) {
            (Some(launched), _) => Ok(launched.processIdentifier()),
            (None, Some(error)) => Err(error.localizedDescription().to_string()),
            (None, None) => Err("the system's launcher answered with nothing".to_string()),
        };
        let _ = say.send(answer);
    });
    NSWorkspace::sharedWorkspace().openApplicationAtURL_configuration_completionHandler(
        &url,
        &configuration,
        Some(&handler),
    );
    heard
        .recv_timeout(Duration::from_secs(60))
        .unwrap_or_else(|_| Err("the system's launcher did not answer within a minute".to_string()))
}

/// Off a Mac there is no such launcher, and asking for it is refused.
#[cfg(not(target_os = "macos"))]
pub fn the_bundle_is_opened(_bundle: &Path, _arguments: &[&str], _environment: &[(&str, &str)]) -> Result<i32, String> {
    Err("the system's app launcher is a Mac's".into())
}


// ════════════════════════════════════════════════════════════════════════════
// The probe's side, and every launch's
// ════════════════════════════════════════════════════════════════════════════

/// **This launch's token, where it is a probe.** An empty value is none.
pub fn this_launch_is_a_probe() -> Option<String> {
    std::env::var(THE_PROBES_TOKEN).ok().filter(|token| !token.trim().is_empty())
}

/// **Whether the record names this launch as the release in its probe**,
/// asked by a launch carrying the token once it holds the hold and **before
/// anything opens the record** (Z21, *a probe reads Z21a's record before it
/// opens the record*, 2026-11-11, the Fable review's fourth sitting, fault
/// 1). `src/main.rs` ends the launch where this says no.
///
/// **Why.** Opening the record migrates it (L6). A probe the record does not
/// name is one whose update was already rolled back: on a Mac, the system's
/// launcher answered later than [`the_bundle_is_opened`] waits, the old
/// process rolled back with no process to end, and the late probe then took
/// the hold, migrated the record the previous build had just been handed,
/// and ran on from a bundle no longer installed.
pub fn the_record_names_this_probe() -> bool {
    let Some(token) = this_launch_is_a_probe() else {
        return false;
    };
    let the_token_pipe = std::env::var_os(THE_PROBES_TOKEN_PIPE)
        .filter(|path| !path.is_empty())
        .map(std::path::PathBuf::from);
    match crate::platform::locations::resolve(crate::platform::locations::Location::LocalRecord) {
        Ok(path) => {
            the_record_at_names_this_probe(&path, super::checking::THIS_BUILDS_VERSION)
                && this_attempt_is_the_one_awaited(&token, the_token_pipe.as_deref())
        }
        Err(_) => false,
    }
}

/// **Whether this probe's own attempt is the one being watched** (the same
/// paragraph; Sol's second point on this sitting's fixes). The record names a
/// release, not an attempt: after a late launch and a rollback, the same
/// version installed again writes the same record, and the first attempt's
/// probe, still waiting for the hold, would pass.
///
/// **The watcher leaves its attempt's token beside the two pipes**
/// ([`AProbeUnderWatch::launched`]), and a probe handed a token pipe holds its
/// own token against it. **No such file is a watcher from before this
/// change**, and the probe goes on: refusing would roll back every update
/// from that build. A probe that is its watcher's own child has no pipe and
/// is not asked; the watcher has that child to end.
pub fn this_attempt_is_the_one_awaited(token: &str, the_token_pipe: Option<&Path>) -> bool {
    let Some(folder) = the_token_pipe.and_then(Path::parent) else {
        return true;
    };
    match std::fs::read_to_string(folder.join(THE_ATTEMPTS_NAME)) {
        Ok(awaited) => awaited.trim() == token,
        Err(_) => true,
    }
}

/// The same, at a named path and for a named version. **The record is read
/// as it stands: no migration, no creation, no setting changed.** No file, no
/// table, no row, another release, or a reading that fails are all *no*.
pub fn the_record_at_names_this_probe(the_records_own_path: &Path, this_builds_version: &str) -> bool {
    let Ok(connection) =
        rusqlite::Connection::open_with_flags(the_records_own_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)
    else {
        return false;
    };
    matches!(
        update_trust::the_update_in_probe(&connection),
        Ok(Some(probe)) if probe.version == this_builds_version
    )
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
/// named backup removed, E18's staging folder removed (from 2026-11-09), and
/// the record cleared, **in that order and the record last**, so a launch that
/// dies part way finds the record and does it again.
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
    the_staging_folder_here: Option<&Path>,
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
        let _ = (the_backup_kept_here, the_bundle_running_now, the_staging_folder_here);
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
    // **E18's staging folder goes with the backup** (its build detail of
    // 2026-11-09, the twenty-eighth notes): the leftover
    // `/Applications/.Smoozy update/`. This copy's own place, never one a
    // record names; the probe's two pipes in it are done with by now.
    if let Some(staging) = the_staging_folder_here {
        files::a_folder_is_removed_whole(staging).map_err(|failed| failed.reported)?;
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
///
/// **Where the system's launcher started this probe** (macOS, from
/// 2026-11-09), the token goes into the named pipe [`THE_PROBES_TOKEN_PIPE`]
/// names instead, opened without waiting, so a watcher that has gone leaves
/// nothing to wait on.
#[cfg(unix)]
pub fn the_token_is_said_on_this_programs_output(token: &str) -> std::io::Result<()> {
    if let Some(pipe) = std::env::var_os(THE_PROBES_TOKEN_PIPE) {
        use std::os::unix::fs::OpenOptionsExt;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(pipe)?;
        the_pipe_waits(&file)?;
        return the_token_is_said(token, &mut file);
    }
    let said = the_token_is_said(token, &mut std::io::stdout().lock());
    pointed_at_nothing(1);
    said
}

/// The wait for the input's end, on this program's own input — **or for the
/// let-go pipe's closing**, where [`THE_PROBES_LET_GO_PIPE`] names one. It is
/// opened without waiting, so a watcher that has already gone is a let-go at
/// once.
#[cfg(unix)]
pub fn the_watcher_lets_go_of_this_program() {
    if let Some(pipe) = std::env::var_os(THE_PROBES_LET_GO_PIPE) {
        use std::os::unix::fs::OpenOptionsExt;
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(pipe)
        {
            if the_pipe_waits(&file).is_ok() {
                the_watcher_lets_go(&mut file);
            }
        }
        return;
    }
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

    /// **Z21, *a probe reads Z21a's record before it opens the record*
    /// (2026-11-11, the Fable review's fourth sitting, fault 1): in the
    /// program's own start, a launch carrying the token asks whether the
    /// record names it after the hold is decided and before anything opens
    /// the record.** The start is `main.rs`, which no test runs, so its order
    /// is read from its text, as `crate::start`'s own tests read it.
    #[test]
    fn a_probe_asks_whether_the_record_names_it_before_the_record_is_opened() {
        let main = include_str!("../main.rs");
        let the_hold = main.find("this_launch_is_decided(").expect("the hold's decision");
        let the_start = main.find("start::start(&queue)").expect("the start, which opens the record");
        let the_question = main
            .find("the_record_names_this_probe()")
            .expect("a launch carrying the token never asks whether the record names it");
        assert!(
            the_hold < the_question && the_question < the_start,
            "the question is asked once the hold is held and before the record is opened"
        );
    }

    /// **The same fault, the question itself: a probe the record does not
    /// name is told no, and the record is left exactly as the previous build
    /// had it** — not migrated, no copy taken, and no file made where there
    /// was none. A probe the record names is told yes, and the record is
    /// still untouched by the asking.
    #[test]
    fn a_probe_the_record_does_not_name_is_told_so_and_the_record_is_left_as_it_was() {
        use crate::record::migrations::{run_one_for_tests, ALL};
        use crate::record::schema::{version_in, BINARY_KNOWS};
        let folder = crate::scratch_folders::a_folder(format!("smoozy-probe-names-{}", std::process::id()));
        let path = folder.join("local-record.sqlite3");
        let the_previous_builds = BINARY_KNOWS - 1;

        // No record at all: no, and none is made.
        assert!(!the_record_at_names_this_probe(&path, "0.9.0"));
        assert!(!path.exists(), "asking made a record");

        // The previous build's record, after a rollback cleared the probe.
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in ALL.iter().filter(|one| one.produces <= the_previous_builds) {
                run_one_for_tests(&connection, migration).unwrap();
            }
        }
        let as_it_stood = std::fs::read(&path).unwrap();
        assert!(!the_record_at_names_this_probe(&path, "0.9.0"), "a probe no record names was let on");

        // An update in its probe, of another release.
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            update_trust::an_update_goes_into_probe(&connection, &a_probe_record("0.9.1", &path, &folder.join("kept")))
                .unwrap();
        }
        assert!(!the_record_at_names_this_probe(&path, "0.9.0"), "a probe of another release was let on");
        // And of this one.
        assert!(the_record_at_names_this_probe(&path, "0.9.1"));

        let connection = rusqlite::Connection::open(&path).unwrap();
        assert_eq!(version_in(&connection).unwrap(), the_previous_builds, "asking migrated the record");
        assert!(!record::copy_beside(&path).exists(), "asking took a copy");
        update_trust::the_probe_is_cleared(&connection).unwrap();
        drop(connection);
        assert_eq!(std::fs::read(&path).unwrap().len(), as_it_stood.len());
    }

    /// **Sol's second point on the fourth sitting's fixes: a probe from an
    /// earlier attempt at the same version is told no.** The watcher leaves
    /// its attempt's token beside the two pipes; a probe whose own token is
    /// another one is not the probe being watched. No such file is a watcher
    /// from before this change, and the probe goes on, as does a probe that is
    /// its watcher's own child and has no pipe.
    #[test]
    fn a_probe_from_an_earlier_attempt_at_the_same_version_is_told_no() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-probe-attempt-{}", std::process::id()));
        let pipe = folder.join(THE_TOKEN_PIPES_NAME);
        assert!(this_attempt_is_the_one_awaited("first", Some(&pipe)), "a watcher that left no token refused its probe");
        std::fs::write(folder.join(THE_ATTEMPTS_NAME), "second").unwrap();
        assert!(this_attempt_is_the_one_awaited("second", Some(&pipe)));
        assert!(this_attempt_is_the_one_awaited("first", None), "a child probe has no pipe and is not asked");
        assert!(
            !this_attempt_is_the_one_awaited("first", Some(&pipe)),
            "the first attempt's probe was let on while the second attempt is watched"
        );
    }

    /// **And the watcher leaves it**: by the time the system's launcher is
    /// asked for the probe, this attempt's token is beside the two pipes.
    #[cfg(unix)]
    #[test]
    fn a_launched_probe_finds_its_attempts_token_beside_the_pipes() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-probe-attempt-left-{}", std::process::id()));
        let found = std::sync::Mutex::new(None);
        let came = AProbeUnderWatch::launched(
            &folder.join("Smoozy.app"),
            &[],
            "this-attempt",
            &folder,
            &|_, _, _| {
                *found.lock().unwrap() = std::fs::read_to_string(folder.join(THE_ATTEMPTS_NAME)).ok();
                Err("the launcher is not asked in this test".into())
            },
        );
        assert!(came.is_err());
        assert_eq!(
            found.lock().unwrap().as_deref(),
            Some("this-attempt"),
            "the launcher was asked before this attempt's token was left for the probe"
        );
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
    /// fourth bullet): the copy, the backup and E18's staging folder go, then
    /// the record — and a second launch finds nothing to do. **A backup this
    /// program runs from, or one the record does not name as this copy's own,
    /// is never removed.** The staging folder is the twenty-eighth notes'
    /// leftover, and went nowhere before 2026-11-09.
    // Windows clears its recorded build folders through the side-by-side route.
    #[cfg(not(target_os = "windows"))]
    #[test]
    fn an_uncleared_probe_record_is_completed_at_the_next_start() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-probe-clear-{}", std::process::id()));
        let path = folder.join("local-record.sqlite3");
        let backup = folder.join(".Smoozy previous.app");
        let staging = folder.join(".Smoozy update");
        let record = LocalRecord::open_at(&path).unwrap();
        std::fs::create_dir_all(backup.join("Contents")).unwrap();
        std::fs::create_dir_all(staging.join("unpacked")).unwrap();
        std::fs::write(staging.join("Smoozy-macos.zip"), b"13 MB, once").unwrap();
        record::take_the_copy(&path).unwrap();
        let probe = a_probe_record("0.2.0", &path, &backup);
        update_trust::an_update_goes_into_probe(record.connection(), &probe).unwrap();

        // The backup the program runs from stays; the rest goes.
        what_the_record_kept_is_cleared(record.connection(), &path, &probe, Some(&backup), Some(&backup), Some(&staging)).unwrap();
        assert!(backup.exists(), "a build removed the bundle it runs from");
        assert!(!record::copy_beside(&path).exists(), "L8's copy was not discarded");
        assert!(!staging.exists(), "the twenty-eighth notes: the staging folder outlived a good start");
        assert_eq!(update_trust::the_update_in_probe(record.connection()).unwrap(), None);

        // The ordinary case: the backup goes too.
        update_trust::an_update_goes_into_probe(record.connection(), &probe).unwrap();
        what_the_record_kept_is_cleared(record.connection(), &path, &probe, Some(&backup), None, Some(&staging)).unwrap();
        assert!(!backup.exists(), "Z7's backup outlived the record");
        assert_eq!(what_the_record_asks(record.connection(), "0.2.0").unwrap(), WhatTheRecordAsks::Nothing);

        // A record naming a place that is not this copy's own removes nothing.
        let elsewhere = folder.join("someone else's.app");
        std::fs::create_dir_all(&elsewhere).unwrap();
        let odd = a_probe_record("0.2.0", &path, &elsewhere);
        update_trust::an_update_goes_into_probe(record.connection(), &odd).unwrap();
        what_the_record_kept_is_cleared(record.connection(), &path, &odd, Some(&backup), None, None).unwrap();
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
    ///
    /// **The token and the wait go through the running application's own two
    /// functions**, so a probe started with the two pipes named uses them, and
    /// one started as a child uses its output and input.
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
                the_token_is_said_on_this_programs_output(&token).unwrap();
                the_watcher_lets_go_of_this_program();
                what_the_record_kept_is_cleared(
                    record.connection(),
                    &path,
                    &probe,
                    Some(&folder.join("Apps/.Smoozy previous.app")),
                    None,
                    Some(&folder.join("Apps/.Smoozy update")),
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
                the_watcher_lets_go_of_this_program();
                std::process::exit(5);
            }
            _ => std::process::exit(6),
        }
    }

    /// **The launcher's test form**: this test program started as the probe
    /// child with the environment the launcher is handed and nothing piped,
    /// and **reaped on a thread of its own**, so its ending reads as the
    /// launcher's would — a process number that has gone. The real form is
    /// [`the_bundle_is_opened`].
    #[cfg(unix)]
    pub(crate) fn opens_this_test_program(
        folder: &Path,
        version: &str,
        acts: &str,
    ) -> impl Fn(&Path, &[&str], &[(&str, &str)]) -> Result<i32, String> {
        let (folder, version, acts) = (folder.to_path_buf(), version.to_string(), acts.to_string());
        move |_bundle, arguments, environment| {
            let mut command = the_probe_child_command(&folder, &version, &acts);
            command.arg("--").args(arguments);
            for (key, value) in environment {
                command.env(key, value);
            }
            let mut child = command
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|failed| failed.to_string())?;
            let process = child.id() as i32;
            std::thread::spawn(move || {
                let _ = child.wait();
            });
            Ok(process)
        }
    }

    /// **The twenty-eighth notes, note 5: a probe started the way the system's
    /// launcher starts one** — no child pipes, the token and the let-go over
    /// the two named pipes, the ending read from the process number. The
    /// token is a successful start and the let-go clears the record; an ending
    /// without it is a failed one; silence past the deadline is too, and the
    /// probe is ended by its number; and the pipes stand in the folder named.
    #[cfg(unix)]
    #[test]
    fn twenty_eighth_5_a_launched_probe_is_watched_over_its_two_pipes() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-probe-launched-{}", std::process::id()));
        let path = folder.join("local-record.sqlite3");
        let record = LocalRecord::open_at(&path).unwrap();
        let pipes = folder.join("Apps/.Smoozy update");
        std::fs::create_dir_all(&pipes).unwrap();
        let bundle = folder.join("Apps/Smoozy.app");
        update_trust::an_update_goes_into_probe(
            record.connection(),
            &a_probe_record("0.2.0", &path, &folder.join("Apps/.Smoozy previous.app")),
        )
        .unwrap();
        let token = a_token().unwrap();
        let hold = || {
            let the_one_instance::TheLaunch::Runs(hold) = the_one_instance::the_hold_is_tried_at(
                &folder.join("running-copy.lock"),
                &folder.join("running-copy.door"),
            ) else {
                panic!("the watcher takes the hold");
            };
            the_one_instance::the_hold_is_handed_to_the_probe(hold);
        };

        hold();
        let opens = opens_this_test_program(&folder, "0.2.0", "starts");
        let mut probe = AProbeUnderWatch::launched(&bundle, &[crate::start::THE_UPDATE_LAUNCH_ARGUMENT], &token, &pipes, &opens).unwrap();
        {
            use std::os::unix::fs::FileTypeExt;
            for name in [THE_TOKEN_PIPES_NAME, THE_LET_GO_PIPES_NAME] {
                assert!(std::fs::metadata(pipes.join(name)).unwrap().file_type().is_fifo(), "{name} is not a named pipe");
            }
        }
        assert_eq!(probe.is_watched(Duration::from_secs(30)), WhatTheProbeCameTo::ItStarted);
        assert!(probe.is_let_go().is_none(), "a launched probe is no child");
        let deadline = Instant::now() + Duration::from_secs(30);
        while update_trust::the_update_in_probe(record.connection()).unwrap().is_some() {
            assert!(Instant::now() < deadline, "the launched probe never cleared once let go");
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(!pipes.exists(), "the staging folder outlived the good start");

        // An ending with no token is a failed start.
        std::fs::create_dir_all(&pipes).unwrap();
        update_trust::an_update_goes_into_probe(
            record.connection(),
            &a_probe_record("0.1.8", &path, &folder.join("Apps/.Smoozy previous.app")),
        )
        .unwrap();
        hold();
        let opens = opens_this_test_program(&folder, "0.2.0", "starts");
        let mut probe = AProbeUnderWatch::launched(&bundle, &[], &token, &pipes, &opens).unwrap();
        assert!(matches!(probe.is_watched(Duration::from_secs(30)), WhatTheProbeCameTo::ItEndedFirst(_)));
        probe.is_ended();

        // Silence past the deadline, and the probe ended by its number.
        let opens = opens_this_test_program(&folder, "0.2.0", "hangs");
        let mut probe = AProbeUnderWatch::launched(&bundle, &[], &token, &pipes, &opens).unwrap();
        assert_eq!(probe.is_watched(Duration::from_millis(1500)), WhatTheProbeCameTo::TheDeadlinePassed);
        let process = match &probe.probe {
            TheProbe::Launched { process, .. } => *process,
            TheProbe::AChild(_) => panic!("launched as a child"),
        };
        probe.is_ended();
        assert!(!a_process_runs(process), "the hung probe was not ended");

        // A launcher that refuses is said, and nothing is left watching.
        let refused = AProbeUnderWatch::launched(&bundle, &[], &token, &pipes, &|_, _, _| Err("refused".into()));
        assert_eq!(refused.err().as_deref(), Some("refused"));
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

    /// **The twenty-eighth notes, note 5, proved at the desk: a throwaway
    /// bundle started through the system's own app launcher**, as the real
    /// form starts the new Smoozy and a rolled-back one. Its program is a
    /// four-line script that runs this test program as the probe child, so
    /// the token goes over the named pipe, the let-go clears the record and
    /// E18's staging folder, a probe that hangs is ended by the process
    /// number the launcher handed back, and the build a rollback starts gets
    /// the update-launch argument and no token.
    ///
    /// **Ignored in ordinary runs, on purpose**: it registers a throwaway
    /// bundle with the system's launcher and brings it forward for a moment,
    /// which an ordinary test run should not do to the desk. Run it by name
    /// with `--ignored`. It proves the launch; **it cannot prove the Dock's one
    /// icon**, which only an update between two signed releases can show.
    #[cfg(target_os = "macos")]
    #[test]
    #[ignore = "starts a throwaway app through the system's launcher; run by name with --ignored"]
    fn twenty_eighth_5_a_throwaway_bundle_starts_through_the_systems_launcher() {
        use crate::updating::the_mac::{StartsABuild, ThroughTheSystemsLauncher};
        let folder = crate::scratch_folders::a_folder(format!("smoozy-the-launcher-{}", std::process::id()));
        let path = folder.join("local-record.sqlite3");
        let record = LocalRecord::open_at(&path).unwrap();
        let bundle = folder.join("Apps/Smoozy.app");
        let staging = folder.join("Apps/.Smoozy update");
        std::fs::create_dir_all(bundle.join("Contents/MacOS")).unwrap();
        std::fs::create_dir_all(&staging).unwrap();
        std::fs::write(
            bundle.join("Contents/Info.plist"),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\"><dict><key>CFBundleExecutable</key><string>probe</string><key>CFBundleIdentifier</key><string>dev.smoozy.throwaway.launch</string><key>CFBundlePackageType</key><string>APPL</string><key>LSUIElement</key><true/></dict></plist>\n",
        )
        .unwrap();
        // **A copy of this test program, out of the Desktop**: an app the
        // launcher starts is its own party to the system's folder privacy, so
        // a program under `~/Desktop` would wait on a permission prompt
        // (observed at this desk, 2026-11-09: the system's own open, inside
        // the loader, never returned).
        let program = folder.join("the-test-program");
        std::fs::copy(std::env::current_exe().unwrap(), &program).unwrap();
        let quoted = |path: &Path| format!("'{}'", path.display().to_string().replace('\'', "'\\''"));
        let script = format!(
            "#!/bin/sh\nACTS=\"$(cat {acts})\"\nenv > {seen}.\"$ACTS\"\necho \"$@\" > {arguments}.\"$ACTS\"\n[ \"$ACTS\" = previous ] && exit 0\nexport {folder_var}={folder} {version_var}=0.2.0 {acts_var}=\"$ACTS\" SMOOZY_TEST_THE_PROBE_CHILD_WANTS_THE_MODE=1\nexec {program} --exact updating::the_probe::tests::the_probe_child --nocapture --test-threads=1 -- \"$@\" > {output}.\"$ACTS\" 2>&1\n",
            acts = quoted(&folder.join("acts")),
            seen = quoted(&folder.join("environment")),
            arguments = quoted(&folder.join("arguments")),
            folder_var = THE_PROBE_CHILDS_FOLDER,
            folder = quoted(&folder),
            version_var = THE_PROBE_CHILDS_VERSION,
            acts_var = THE_PROBE_CHILDS_ACTS,
            program = quoted(&program),
            output = quoted(&folder.join("output")),
        );
        let probe_program = bundle.join("Contents/MacOS/probe");
        std::fs::write(&probe_program, script).unwrap();
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&probe_program, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        let read = |name: &str| std::fs::read_to_string(folder.join(name)).unwrap_or_default();
        let token = a_token().unwrap();

        // **The new build: the token over the pipe, and the let-go.**
        std::fs::write(folder.join("acts"), "starts").unwrap();
        update_trust::an_update_goes_into_probe(
            record.connection(),
            &a_probe_record("0.2.0", &path, &folder.join("Apps/.Smoozy previous.app")),
        )
        .unwrap();
        let mut probe = ThroughTheSystemsLauncher.the_probe(&bundle, &token).expect("the launcher started the bundle");
        let came = probe.is_watched(Duration::from_secs(60));
        assert_eq!(came, WhatTheProbeCameTo::ItStarted, "the probe said: {}", read("output.starts"));
        assert!(read("arguments.starts").contains(crate::start::THE_UPDATE_LAUNCH_ARGUMENT));
        assert!(read("environment.starts").contains(&format!("{THE_PROBES_TOKEN}={token}")));
        assert!(probe.is_let_go().is_none());
        let deadline = Instant::now() + Duration::from_secs(30);
        while update_trust::the_update_in_probe(record.connection()).unwrap().is_some() {
            assert!(Instant::now() < deadline, "the launched probe never cleared once let go");
            std::thread::sleep(Duration::from_millis(50));
        }
        assert!(!staging.exists(), "the staging folder outlived the good start");
        eprintln!(
            "OBSERVED: the launched program saw HOME {} and PATH {}",
            if read("environment.starts").lines().any(|line| line.starts_with("HOME=")) { "set" } else { "unset" },
            if read("environment.starts").lines().any(|line| line.starts_with("PATH=")) { "set" } else { "unset" },
        );

        // **A probe that hangs, ended by its process number.**
        std::fs::write(folder.join("acts"), "hangs").unwrap();
        let mut probe = ThroughTheSystemsLauncher.the_probe(&bundle, &token).expect("the launcher started the bundle");
        assert_eq!(probe.is_watched(Duration::from_secs(5)), WhatTheProbeCameTo::TheDeadlinePassed);
        let process = match &probe.probe {
            TheProbe::Launched { process, .. } => *process,
            TheProbe::AChild(_) => panic!("the real form started a child"),
        };
        probe.is_ended();
        assert!(!a_process_runs(process), "the hung probe was not ended");

        // **The build a rollback starts: the argument alone, no token** —
        // even from an old process that was itself a probe, whose own
        // environment the launcher passes on.
        std::fs::write(folder.join("acts"), "previous").unwrap();
        // SAFETY: this ignored test runs alone, by name, on one thread.
        unsafe {
            std::env::set_var(THE_PROBES_TOKEN, "an-old-token");
        }
        ThroughTheSystemsLauncher.the_previous_build(&bundle).expect("the launcher started the previous build");
        let deadline = Instant::now() + Duration::from_secs(30);
        while read("arguments.previous").is_empty() {
            assert!(Instant::now() < deadline, "the previous build never ran");
            std::thread::sleep(Duration::from_millis(50));
        }
        assert!(read("arguments.previous").contains(crate::start::THE_UPDATE_LAUNCH_ARGUMENT));
        unsafe {
            std::env::remove_var(THE_PROBES_TOKEN);
        }
        let handed = |name: &str| {
            read("environment.previous")
                .lines()
                .find_map(|line| line.strip_prefix(&format!("{name}=")).map(str::to_string))
                .unwrap_or_default()
        };
        for name in [THE_PROBES_TOKEN, THE_PROBES_TOKEN_PIPE, THE_PROBES_LET_GO_PIPE] {
            assert_eq!(handed(name), "", "the previous build was handed {name}");
        }
        eprintln!("OBSERVED: the rolled-back build's arguments: {}", read("arguments.previous").trim());
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
        let mut child = probe.is_let_go().expect("the probe is this program's child");
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

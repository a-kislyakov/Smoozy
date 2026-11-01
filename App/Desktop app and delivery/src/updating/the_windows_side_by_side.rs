//! **The Windows update route: side by side, never in place** — E16's Windows
//! half, Z13 to Z15a, Z17, Z21's Windows bullet, Z21a, Z24, Z31 and Z32, with
//! L8 and L13 used. Built at step 11's Windows update piece. The relaunch helper (Z18 to Z20, P3)
//! is [`super::the_relaunch_helper`]. **No route skips the check**:
//! publication takes an [`AVerifiedBuild`], and the one function that makes
//! one runs the fetch's length and digest, then requires the system's own
//! signature check when the running build itself passes that same call.
//!
//! **The order, and each step is a gate on the next.**
//!
//! 1. **The other builds are removed** — every folder Smoozy installed for
//!    itself but the one this program runs from (Z15's *one previous build is
//!    kept and no more*). Never while an update-in-probe record is uncleared,
//!    and never a place Smoozy did not make.
//! 2. **Fetched** ([`super::fetching`]) straight into a new per-user folder
//!    named by the version (Z15), held to the signed manifest's length and
//!    digest (E16). The download is the program itself — the per-user
//!    unpackaged executable E3 names — so nothing is unpacked, and a download
//!    that is not one `.exe` is refused.
//! 3. **E16's conditional system signature check.** The running program is
//!    checked once per run through [`ChecksTheProgram`], with the same call
//!    and E12's no-interactive-user value. If it passes, the new program must
//!    pass too. Otherwise the signed manifest's exact length and digest are
//!    sufficient: an unsigned running build can update without a certificate.
//!    Windows' own execution policy may still block a launch (E8a).
//! 4. **Held while anything is running** ([`super::the_hold`], Z31) — the
//!    installed, verified build is kept for the run, and the hold is read
//!    again at the next check. **Z32's press goes past the hold and past
//!    nothing else.**
//! 5. **Published** ([`the_verified_build_is_published`]): the window put away
//!    and the record's one lock taken and kept (Z6's rule, which binds every
//!    system's publication); L8's copy from an earlier migration discarded; the
//!    accepted trust record kept (E15); Z21a's update-in-probe record written,
//!    naming the running program as the retained previous build; **the
//!    Start-menu shortcut's target rewritten to the new program** (Z15a) — a
//!    complete new shortcut written beside it and renamed over it through the
//!    platform file adapter (Z24); and **the relaunch helper started** (Z19):
//!    this same program, started with the helper's argument.
//! 6. **The hold let go, and the old process exits** writing nothing on the way
//!    out. The helper starts the probe, watches it, and rolls back if it fails
//!    ([`super::the_relaunch_helper`]).
//!
//! **What a publication that does not happen is.** E15's *an update that did
//! not happen*: the shortcut is pointed back where it was, the probe record
//! is cleared, the window comes back, and About reads *Update installation
//! failed.* The next check tries again.
//!
//! **What a later launch clears** (Z21a's fourth bullet, from
//! `crate::updating::the_probe`): a launch of the new build that finds the
//! record naming itself removes the previous build's folder; a launch of the
//! previous build after a rollback removes the failed one's. **Only a folder
//! directly inside Smoozy's own builds folder is ever removed, and never the
//! one the program runs from** — so the program a user first downloaded and
//! ran, which sits wherever they put it, is never removed by Smoozy.
//!
//! **Everything Windows-only is small and behind `cfg(windows)`**: the
//! signature call here, the shortcut's writing in `crate::notifications`, and
//! the probe's output in `crate::updating::the_probe`. **Every deciding step
//! compiles and is tested on every system.** **Nothing here has run on
//! Windows** (§22 item 2): every Windows fact is Microsoft's own page, cited in
//! the account of this piece's sources file, and the next hosted Windows run is
//! the first compile.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `State, identity and recovery` I9.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site, or
//! in the list above.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, OnceLock};

use super::checking::{AdmitsTheUpdateFetch, ReadsTheUpdateFile, TheReading, WhatTheRequestSays};
use super::fetching::{self, WhyTheFetchDidNotPass};
use super::the_mac::HandsTheHold;
use super::the_probe;
use super::the_relaunch_helper::TheHelpersWork;
use super::trust::TheUpdateTrustRecord;
use super::verifying::{self, AVerifiedUpdate};
use crate::platform::files;
use crate::record::{self, update_trust, LocalRecord};

// ════════════════════════════════════════════════════════════════════════════
// The places
// ════════════════════════════════════════════════════════════════════════════

/// **Smoozy's own builds folder and the program running now** — F11's builds
/// folder, resolved through `crate::platform::locations` and composed nowhere
/// else, and the running program read from the system's own report of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePlaces {
    /// Z15's per-user folders sit directly inside this one, one per version.
    pub builds: PathBuf,
    /// The program this process runs from: the retained previous build of an
    /// update it publishes.
    pub running: PathBuf,
}

impl ThePlaces {
    /// **This copy's own.**
    pub fn for_this_copy() -> Result<ThePlaces, String> {
        let builds = crate::platform::locations::the_windows_builds_folder().map_err(|not| not.reported)?;
        let running = std::env::current_exe().map_err(|failed| failed.to_string())?;
        Ok(ThePlaces { builds, running })
    }

    /// **The folder a version is installed into**, or nothing where the
    /// version is not whole numbers joined by dots — a version is put into a
    /// path only when it can name nothing but one folder inside the builds
    /// folder.
    pub fn the_folder_for(&self, version: &str) -> Option<PathBuf> {
        a_plain_version(version).then(|| self.builds.join(version))
    }

    /// **The folder Smoozy made that holds `program`**, or nothing where the
    /// program does not sit directly inside a folder directly inside the
    /// builds folder. This is the only test of whether a build is Smoozy's
    /// own to remove.
    pub fn smoozys_own_folder_holding(&self, program: &Path) -> Option<PathBuf> {
        let folder = program.parent()?;
        (folder.parent()? == self.builds.as_path()).then(|| folder.to_path_buf())
    }
}

/// Whole numbers joined by dots, and nothing else.
fn a_plain_version(version: &str) -> bool {
    !version.is_empty()
        && version
            .split('.')
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
}

/// **Whether a download is one Windows program**: its name ends `.exe`, in any
/// case. The name has already been held to a plain file name by the fetch.
fn a_program_name(name: &str) -> bool {
    name.len() > 4 && name.to_ascii_lowercase().ends_with(".exe")
}

/// **Step 1: every build folder but those kept is removed**, each whole. Only
/// folders directly inside the builds folder are looked at; a removal that
/// fails is said and the rest go on. Nothing, where the builds folder is not
/// there.
pub fn the_other_builds_are_removed(builds: &Path, keep: &[&Path]) -> Vec<String> {
    let mut not_removed = Vec::new();
    let Ok(entries) = std::fs::read_dir(builds) else {
        return not_removed;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() || keep.iter().any(|kept| *kept == path.as_path()) {
            continue;
        }
        if let Err(failed) = files::a_folder_is_removed_whole(&path) {
            not_removed.push(format!("{}: {}", path.display(), failed.reported));
        }
    }
    not_removed
}

// ════════════════════════════════════════════════════════════════════════════
// The system's own check — an adapter
// ════════════════════════════════════════════════════════════════════════════

/// **The system's own signature verification over the installed program**
/// (E16). **Its real form is [`TheSystemsSignatureCheck`]; its test form
/// exists only in tests**, and nothing outside them can be handed one.
pub trait ChecksTheProgram {
    /// Accepted, or the system's own answer for why not.
    fn accepts(&self, program: &Path) -> Result<(), String>;
}

/// **`WinVerifyTrust` with the Authenticode action, and no one there to ask**
/// (E12, E16). Microsoft's worked example for a program file, followed: the
/// action `WINTRUST_ACTION_GENERIC_VERIFY_V2` (*"Verify a file or object using
/// the Authenticode policy provider"*), no interface (`WTD_UI_NONE`), the file
/// named (`WTD_CHOICE_FILE`), no revocation checking (`WTD_REVOKE_NONE`, as
/// the example has it), a verify then a close; and the window argument
/// `INVALID_HANDLE_VALUE`, whose meaning is *"There is no interactive user."*
/// **Zero is the only acceptance** — *"No other value besides zero should be
/// considered a successful return."* An unsigned program is refused
/// (`TRUST_E_NOSIGNATURE`). Elsewhere than Windows it refuses everything.
/// Sources: this piece's sources file, in `Audits/Build/Step 11/Sources/`.
pub struct TheSystemsSignatureCheck;

impl ChecksTheProgram for TheSystemsSignatureCheck {
    fn accepts(&self, program: &Path) -> Result<(), String> {
        #[cfg(windows)]
        {
            the_signature_is_verified(program)
        }
        #[cfg(not(windows))]
        {
            let _ = program;
            Err("the system's own signature check is Windows'".into())
        }
    }
}

#[cfg(windows)]
fn the_signature_is_verified(program: &Path) -> Result<(), String> {
    use windows_sys::Win32::Foundation::{HWND, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Security::WinTrust::{
        WinVerifyTrust, WINTRUST_ACTION_GENERIC_VERIFY_V2, WINTRUST_DATA, WINTRUST_DATA_0, WINTRUST_FILE_INFO,
        WTD_CHOICE_FILE, WTD_REVOKE_NONE, WTD_STATEACTION_CLOSE, WTD_STATEACTION_VERIFY, WTD_UI_NONE,
    };
    let path: Vec<u16> = program.to_string_lossy().encode_utf16().chain(Some(0)).collect();
    let mut file = WINTRUST_FILE_INFO {
        cbStruct: std::mem::size_of::<WINTRUST_FILE_INFO>() as u32,
        pcwszFilePath: path.as_ptr(),
        hFile: std::ptr::null_mut(),
        pgKnownSubject: std::ptr::null_mut(),
    };
    let mut data = WINTRUST_DATA::default();
    data.cbStruct = std::mem::size_of::<WINTRUST_DATA>() as u32;
    data.dwUIChoice = WTD_UI_NONE;
    data.fdwRevocationChecks = WTD_REVOKE_NONE;
    data.dwUnionChoice = WTD_CHOICE_FILE;
    data.Anonymous = WINTRUST_DATA_0 { pFile: &mut file };
    data.dwStateAction = WTD_STATEACTION_VERIFY;
    let mut action = WINTRUST_ACTION_GENERIC_VERIFY_V2;
    let no_one_there = INVALID_HANDLE_VALUE as HWND;
    // SAFETY: every pointer handed over is to a value alive for both calls,
    // and the close releases what the verify kept, as Microsoft's example does.
    let status = unsafe {
        WinVerifyTrust(no_one_there, &mut action, &mut data as *mut WINTRUST_DATA as *mut core::ffi::c_void)
    };
    data.dwStateAction = WTD_STATEACTION_CLOSE;
    unsafe {
        WinVerifyTrust(no_one_there, &mut action, &mut data as *mut WINTRUST_DATA as *mut core::ffi::c_void);
    }
    if status == 0 {
        Ok(())
    } else {
        Err(format!("the system's signature check refused it: {:#010x}", status as u32))
    }
}

/// **E16: the running executable's platform standing, read once for this
/// run**, through the very same adapter that checks a candidate. A failure
/// means the manifest alone authenticates updates; a success requires the
/// candidate to pass the platform call too.
#[derive(Default)]
pub struct TheRunningBuildsStanding {
    requires_signed_updates: OnceLock<bool>,
}

impl TheRunningBuildsStanding {
    fn requires_a_signature(&self, running: &Path, check: &dyn ChecksTheProgram) -> bool {
        *self.requires_signed_updates.get_or_init(|| check.accepts(running).is_ok())
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Installed and verified
// ════════════════════════════════════════════════════════════════════════════

/// **Why an update did not happen.** Each is E15's *an update that did not
/// happen*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheUpdateDidNotHappen {
    /// The builds folder or the running program could not be named.
    NoPlaces(String),
    /// The verified manifest carries nothing for this copy's row.
    NothingForThisRow,
    /// **The version is not one that can name a folder**, or it is the
    /// version this program already runs from.
    NoFolderForTheVersion,
    /// **The download is not one Windows program.**
    NotAProgram,
    /// The new folder could not be made ready.
    TheFolder(String),
    /// The fetch did not pass (length, digest, network, the disk).
    TheFetch(WhyTheFetchDidNotPass),
    /// **The system's own signature check refused the program** (E16).
    TheSystemsOwnCheckRefusedIt(String),
    /// The record would not take E15's or Z21a's write before publication.
    TheRecordBeforePublication(String),
    /// **The shortcut could not be pointed at the new program** (Z15a).
    TheShortcut(String),
    /// **The relaunch helper could not be started** (Z19).
    TheHelper(String),
}

/// **A program installed in a new per-user folder, held to the signed
/// manifest and E16's conditional platform requirement.** The fields are
/// private and the one function that makes one is [`the_build_is_fetched_and_verified`]: **that is what *no
/// route skips the check* is, by construction.**
#[derive(Debug)]
pub struct AVerifiedBuild {
    program: PathBuf,
    version: String,
    release_sequence: u64,
    the_trust_record_to_keep: TheUpdateTrustRecord,
}

impl AVerifiedBuild {
    /// The version it is.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Where it is installed.
    pub fn program(&self) -> &Path {
        &self.program
    }
}

/// **Steps 2 and 3, whole.** The version's folder is emptied and made afresh,
/// the program fetched into it and held to its length and digest, and then
/// offered to the system's own check where it now sits only if the running
/// build itself passes that same call (E16).
pub fn the_build_is_fetched_and_verified(
    file: &dyn ReadsTheUpdateFile,
    admission: &dyn AdmitsTheUpdateFetch,
    verified: &AVerifiedUpdate,
    says: &WhatTheRequestSays,
    places: &ThePlaces,
    check: &dyn ChecksTheProgram,
    standing: &TheRunningBuildsStanding,
) -> Result<AVerifiedBuild, WhyTheUpdateDidNotHappen> {
    use WhyTheUpdateDidNotHappen as Why;
    let entry = verified.the_entry_for_this_row.as_ref().ok_or(Why::NothingForThisRow)?;
    if !a_program_name(&entry.file_name) {
        return Err(Why::NotAProgram);
    }
    let folder = places
        .the_folder_for(&verified.manifest.version)
        .ok_or(Why::NoFolderForTheVersion)?;
    // **Never the folder this program runs from.**
    if places.running.parent() == Some(folder.as_path()) {
        return Err(Why::NoFolderForTheVersion);
    }
    files::a_folder_is_removed_whole(&folder).map_err(|failed| Why::TheFolder(failed.reported))?;
    files::ensure_directory(&folder).map_err(|failed| Why::TheFolder(failed.reported))?;
    let fetched = match fetching::the_artifact_is_fetched(file, admission, entry, says, &folder) {
        Ok(fetched) => fetched,
        Err(failed) => {
            let _ = files::a_folder_is_removed_whole(&folder);
            return Err(Why::TheFetch(failed));
        }
    };
    if standing.requires_a_signature(&places.running, check) {
        if let Err(refused) = check.accepts(fetched.path()) {
            let _ = files::a_folder_is_removed_whole(&folder);
            return Err(Why::TheSystemsOwnCheckRefusedIt(refused));
        }
    }
    Ok(AVerifiedBuild {
        program: fetched.path().to_path_buf(),
        version: verified.manifest.version.clone(),
        release_sequence: verified.manifest.release_sequence,
        the_trust_record_to_keep: verified.the_trust_record_to_keep.clone(),
    })
}

// ════════════════════════════════════════════════════════════════════════════
// Published
// ════════════════════════════════════════════════════════════════════════════

/// **The Start-menu shortcut, as publication moves it** (Z15a). The real form
/// is [`TheStartMenuShortcut`]; the tests keep a note instead.
pub trait PointsTheShortcut {
    /// The shortcut's target becomes `program`, whole and at once.
    fn points_at(&self, program: &Path) -> Result<(), String>;
}

/// **The real form: T3's own shortcut, rewritten** — `crate::notifications`,
/// which owns it.
pub struct TheStartMenuShortcut;

impl PointsTheShortcut for TheStartMenuShortcut {
    fn points_at(&self, program: &Path) -> Result<(), String> {
        crate::notifications::the_shortcut_is_pointed_at(program)
    }
}

/// **How the relaunch helper is started** (Z19). The real form is
/// [`ThisProgramAsTheHelper`]; the tests keep a note instead.
pub trait StartsTheHelper {
    fn the_helper(&self, work: &TheHelpersWork) -> Result<(), String>;
}

/// **The real form: this same program, started with the helper's argument**
/// (Z20: the helper is the artifact's own component, checked as the artifact
/// was). Started from the running build — the one known to start — so a new
/// build that cannot start at all is still rolled back (§22 item 46's Windows
/// half). **The child is let go and not waited for**: a parent's ending does
/// not end the processes it created, in Microsoft's own words, and the
/// standard library's child handle has no ending of its own when dropped.
pub struct ThisProgramAsTheHelper;

impl StartsTheHelper for ThisProgramAsTheHelper {
    fn the_helper(&self, work: &TheHelpersWork) -> Result<(), String> {
        let this = std::env::current_exe().map_err(|failed| failed.to_string())?;
        Command::new(this)
            .args(work.as_arguments())
            .env_remove(the_probe::THE_PROBES_TOKEN)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map(drop)
            .map_err(|failed| failed.to_string())
    }
}

/// **What publication came to** in the old process. It never learns whether
/// the new build started: that is the helper's to watch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThePublicationCameTo {
    /// **Nothing was published**: an update that did not happen, and the old
    /// process carries on.
    NotPublished(WhyTheUpdateDidNotHappen),
    /// **The shortcut names the new program and the helper is running**, with
    /// the hold let go: the old process exits now.
    HandedToTheHelper,
}

/// **Step 5, whole, and the hold let go.** `record` is the old process's own
/// open record, whose one lock the caller holds for the whole of this.
pub fn the_verified_build_is_published(
    built: AVerifiedBuild,
    places: &ThePlaces,
    record: &LocalRecord,
    shortcut: &dyn PointsTheShortcut,
    helper: &dyn StartsTheHelper,
    hold: &mut dyn HandsTheHold,
) -> WhatThePublicationCameTo {
    use WhatThePublicationCameTo as Came;
    use WhyTheUpdateDidNotHappen as Why;
    let connection = record.connection();
    let record_path = record.path().to_path_buf();
    // **L13's discard, owed since the last migration.**
    if let Err(failed) = record::discard_the_copy(&record_path) {
        return Came::NotPublished(Why::TheRecordBeforePublication(format!("{failed:?}")));
    }
    // **E15**: the accepted sequence stored before publication.
    if let Err(failed) = verifying::keep(connection, &built.the_trust_record_to_keep) {
        return Came::NotPublished(Why::TheRecordBeforePublication(format!("{failed:?}")));
    }
    // **Z21a**: the record written immediately before publication, naming the
    // running program as the retained previous build.
    let probe = update_trust::TheUpdateInProbe {
        release_sequence: built.release_sequence,
        version: built.version.clone(),
        the_previous_artifact: the_probe::path_bytes(&places.running),
        the_record_copy: the_probe::path_bytes(&record::copy_beside(&record_path)),
    };
    if let Err(failed) = update_trust::an_update_goes_into_probe(connection, &probe) {
        return Came::NotPublished(Why::TheRecordBeforePublication(format!("{failed:?}")));
    }
    // **Z15a: the shortcut names the new program before the helper starts the
    // probe.**
    if let Err(failed) = shortcut.points_at(&built.program) {
        // A rename that did not happen left the old shortcut whole; pointing
        // it back costs nothing where it did.
        let _ = shortcut.points_at(&places.running);
        let _ = update_trust::the_probe_is_cleared(connection);
        return Came::NotPublished(Why::TheShortcut(failed));
    }
    // **Z19: the helper.**
    let work = TheHelpersWork {
        new_program: built.program.clone(),
        previous_program: places.running.clone(),
        record: record_path.clone(),
    };
    if let Err(failed) = helper.the_helper(&work) {
        let _ = shortcut.points_at(&places.running);
        let _ = update_trust::the_probe_is_cleared(connection);
        return Came::NotPublished(Why::TheHelper(failed));
    }
    // **P2: the old process has stopped writing; the hold goes**, and the
    // probe the helper starts takes it.
    hold.to_the_probe();
    Came::HandedToTheHelper
}

// ════════════════════════════════════════════════════════════════════════════
// What a later launch clears (Z21a's fourth bullet)
// ════════════════════════════════════════════════════════════════════════════

/// **The one build folder a launch that completes an uncleared record
/// removes**, or none.
///
/// - **The record names this build** — the new build started without its
///   watcher, or its watcher let go: the retained previous build's folder.
/// - **The record names another build** — this is the previous build after a
///   rollback: the failed build's folder.
///
/// Either way only a folder directly inside the builds folder, and never the
/// one this program runs from.
pub fn the_build_a_completion_removes(
    places: &ThePlaces,
    probe: &update_trust::TheUpdateInProbe,
    this_builds_version: &str,
) -> Option<PathBuf> {
    let folder = if probe.version == this_builds_version {
        let previous = PathBuf::from(String::from_utf8_lossy(&probe.the_previous_artifact).into_owned());
        places.smoozys_own_folder_holding(&previous)?
    } else {
        places.the_folder_for(&probe.version)?
    };
    (places.running.parent() != Some(folder.as_path())).then_some(folder)
}

/// **Z21a's clearing on Windows**: L8's copy discarded where the record names
/// this record's own, the one build folder removed, and the record cleared —
/// **in that order and the record last**, so a launch that could not remove
/// the folder (the helper may still be ending, its program still mapped)
/// leaves the record for the next launch to finish.
pub fn what_the_record_kept_is_cleared_here(
    connection: &rusqlite::Connection,
    the_records_own_path: &Path,
    probe: &update_trust::TheUpdateInProbe,
    places: &ThePlaces,
    this_builds_version: &str,
) -> Result<(), String> {
    let copy = record::copy_beside(the_records_own_path);
    if probe.the_record_copy == the_probe::path_bytes(&copy) {
        record::discard_the_copy(the_records_own_path).map_err(|failed| format!("{failed:?}"))?;
    }
    if let Some(folder) = the_build_a_completion_removes(places, probe, this_builds_version) {
        files::a_folder_is_removed_whole(&folder).map_err(|failed| failed.reported)?;
    }
    update_trust::the_probe_is_cleared(connection).map_err(|failed| format!("{failed:?}"))
}

// ════════════════════════════════════════════════════════════════════════════
// In the running application
// ════════════════════════════════════════════════════════════════════════════

/// **The route, kept for the run**: the installed, verified build held while
/// publication waits (Z31). Made by the update check's sequence and used on
/// that one sequence alone.
pub struct TheWindowsRoute {
    built: Option<AVerifiedBuild>,
    standing: TheRunningBuildsStanding,
}

impl Default for TheWindowsRoute {
    fn default() -> Self {
        TheWindowsRoute::new()
    }
}

impl TheWindowsRoute {
    pub fn new() -> TheWindowsRoute {
        TheWindowsRoute { built: None, standing: TheRunningBuildsStanding::default() }
    }

    /// **A check found a newer version**: installed and verified (or the one
    /// installed for that version kept), then published unless held. The
    /// reading About is to show, where it changes.
    pub fn a_newer_version_was_found<R: tauri::Runtime>(
        &mut self,
        handle: &tauri::AppHandle<R>,
        verified: Option<&AVerifiedUpdate>,
    ) -> Option<TheReading> {
        if !cfg!(target_os = "windows") {
            return None;
        }
        let verified = verified?;
        let places = match ThePlaces::for_this_copy() {
            Ok(places) => places,
            Err(_) => return Some(TheReading::CouldNotInstall),
        };
        let already = self
            .built
            .as_ref()
            .is_some_and(|built| built.version == verified.manifest.version && built.program.exists());
        if !already {
            self.built = None;
            let Some(transfers) = handle_state::<R, Arc<crate::downloading::TheRunsTransfers>>(handle) else {
                return Some(TheReading::CouldNotInstall);
            };
            // **Step 1**, only where no update-in-probe record is uncleared.
            if !a_probe_record_is_uncleared(handle) {
                let running_folder = places.running.parent().map(Path::to_path_buf);
                let keep: Vec<&Path> = running_folder.iter().map(PathBuf::as_path).collect();
                let _ = the_other_builds_are_removed(&places.builds, &keep);
            }
            match the_build_is_fetched_and_verified(
                &super::checking::OverHttps,
                transfers.as_ref(),
                verified,
                &WhatTheRequestSays::for_this_copy(),
                &places,
                &TheSystemsSignatureCheck,
                &self.standing,
            ) {
                Ok(built) => self.built = Some(built),
                Err(_) => return Some(TheReading::CouldNotInstall),
            }
        }
        let version = verified.manifest.version.clone();
        if super::the_mac::publication_is_held_here(handle) {
            return Some(TheReading::ANewerVersion { version, held: true });
        }
        self.published(handle, places)
    }

    /// **Z32: the user installs the held update anyway** — Z31's held
    /// publication and nothing else. Nothing, where nothing is installed.
    pub fn the_held_update_goes_in<R: tauri::Runtime>(&mut self, handle: &tauri::AppHandle<R>) -> Option<TheReading> {
        if self.built.is_none() {
            return None;
        }
        let places = match ThePlaces::for_this_copy() {
            Ok(places) => places,
            Err(_) => return Some(TheReading::CouldNotInstall),
        };
        self.published(handle, places)
    }

    fn published<R: tauri::Runtime>(&mut self, handle: &tauri::AppHandle<R>, places: ThePlaces) -> Option<TheReading> {
        use tauri::Manager;
        let built = self.built.take()?;
        let window = handle.get_webview_window("main");
        if let Some(window) = window.as_ref() {
            let _ = window.hide();
        }
        let started = handle.try_state::<Mutex<crate::start::Started>>()?;
        // **The record's one lock, kept until this process exits** — no new
        // ordinary work is admitted from here.
        let started = started.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let came = match started.record.as_ref() {
            Some(record) => {
                let mut hold = super::the_mac::TheShellsHold { handle };
                the_verified_build_is_published(
                    built,
                    &places,
                    record,
                    &TheStartMenuShortcut,
                    &ThisProgramAsTheHelper,
                    &mut hold,
                )
            }
            None => WhatThePublicationCameTo::NotPublished(WhyTheUpdateDidNotHappen::TheRecordBeforePublication(
                "the record is not open".into(),
            )),
        };
        match came {
            WhatThePublicationCameTo::NotPublished(_) => {
                drop(started);
                if let Some(window) = window.as_ref() {
                    let _ = window.show();
                }
                Some(TheReading::CouldNotInstall)
            }
            // **The old process's run ends here, writing nothing on the way
            // out** (`Reporting a problem` U3 step 3): the run-mark is the new
            // build's now.
            WhatThePublicationCameTo::HandedToTheHelper => std::process::exit(0),
        }
    }
}

/// Whether the record holds an uncleared update-in-probe record (`State,
/// identity and recovery` I9). A record that will not answer counts as one.
fn a_probe_record_is_uncleared<R: tauri::Runtime>(handle: &tauri::AppHandle<R>) -> bool {
    use tauri::Manager;
    let Some(started) = handle.try_state::<Mutex<crate::start::Started>>() else {
        return true;
    };
    let started = started.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    match started.record.as_ref() {
        Some(record) => !matches!(update_trust::the_update_in_probe(record.connection()), Ok(None)),
        None => true,
    }
}

/// The shell's own managed state, where there is any.
fn handle_state<R: tauri::Runtime, T: Send + Sync + Clone + 'static>(handle: &tauri::AppHandle<R>) -> Option<T> {
    use tauri::Manager;
    handle.try_state::<T>().map(|state| state.inner().clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downloading::TheRunsTransfers;
    use crate::updating::fetching::tests::AnArtifactAtTheSeam;
    use crate::updating::manifest::{AnEntry, UpdateManifest, THE_MANIFEST_FORMAT, THE_WINDOWS_ROW};
    use crate::updating::verifying::WhyTheArtifactIsNotTheOneNamed;
    use smoozy_server_protocol::transfers::integrity::Digest;

    // ── The test forms ─────────────────────────────────────────────────────

    /// **The check's test form**: it accepts or refuses as told, and says
    /// which programs it was shown and what each held when shown.
    struct ACheckThatSays {
        accepts: bool,
        running_accepts: bool,
        shown: Mutex<Vec<(PathBuf, Vec<u8>)>>,
    }

    impl ACheckThatSays {
        fn accepting(accepts: bool) -> ACheckThatSays {
            ACheckThatSays {
                accepts,
                running_accepts: accepts,
                shown: Mutex::new(Vec::new()),
            }
        }
    }

    impl ACheckThatSays {
        fn for_builds(running_accepts: bool, accepts: bool) -> Self {
            Self { running_accepts, ..Self::accepting(accepts) }
        }
    }

    impl ChecksTheProgram for ACheckThatSays {
        fn accepts(&self, program: &Path) -> Result<(), String> {
            self.shown
                .lock()
                .unwrap()
                .push((program.to_path_buf(), std::fs::read(program).unwrap_or_default()));
            let running = std::fs::read(program).unwrap_or_default() == b"the running build";
            let accepts = if running { self.running_accepts } else { self.accepts };
            if accepts {
                Ok(())
            } else {
                Err("the test's check refused it".into())
            }
        }
    }

    /// Every act the publication makes, in order.
    #[derive(Default)]
    struct TheActs {
        said: Mutex<Vec<String>>,
        the_shortcut_refuses: bool,
        the_helper_refuses: bool,
    }

    impl PointsTheShortcut for TheActs {
        fn points_at(&self, program: &Path) -> Result<(), String> {
            self.said.lock().unwrap().push(format!("shortcut {}", program.display()));
            if self.the_shortcut_refuses {
                Err("the shortcut would not be written".into())
            } else {
                Ok(())
            }
        }
    }

    impl StartsTheHelper for TheActs {
        fn the_helper(&self, work: &TheHelpersWork) -> Result<(), String> {
            self.said.lock().unwrap().push(format!(
                "helper {} {} {}",
                work.new_program.display(),
                work.previous_program.display(),
                work.record.display()
            ));
            if self.the_helper_refuses {
                Err("the helper would not start".into())
            } else {
                Ok(())
            }
        }
    }

    struct AHold {
        handed: bool,
    }

    impl HandsTheHold for AHold {
        fn to_the_probe(&mut self) {
            self.handed = true;
        }
        fn back(&mut self) -> bool {
            self.handed = false;
            true
        }
    }

    // ── The throwaway installation ─────────────────────────────────────────

    /// One throwaway installation: the builds folder, the running program in
    /// `0.1.8` inside it, the record, and the download of `0.2.0`.
    struct AnInstallation {
        folder: PathBuf,
        places: ThePlaces,
        record: LocalRecord,
        download: Vec<u8>,
        standing: TheRunningBuildsStanding,
    }

    fn an_installation(name: &str) -> AnInstallation {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-windows-{name}-{}", std::process::id()));
        let builds = folder.join("windows-update");
        let running = builds.join("0.1.8").join("smoozy-desktop.exe");
        std::fs::create_dir_all(running.parent().unwrap()).unwrap();
        std::fs::write(&running, b"the running build").unwrap();
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        AnInstallation {
            places: ThePlaces { builds, running },
            folder,
            record,
            download: b"MZ the new build, 0.2.0".to_vec(),
            standing: TheRunningBuildsStanding::default(),
        }
    }

    fn the_windows_entry(bytes: &[u8], name: &str) -> AnEntry {
        AnEntry {
            row: THE_WINDOWS_ROW.to_string(),
            file_name: name.to_string(),
            byte_length: bytes.len() as u64,
            digest: Digest::of(bytes).bytes(),
        }
    }

    fn a_verified_update(version: &str, entry: AnEntry) -> AVerifiedUpdate {
        AVerifiedUpdate {
            manifest: UpdateManifest {
                manifest_format: THE_MANIFEST_FORMAT.into(),
                release_sequence: 9,
                version: version.into(),
                signing_keys: vec!["online-1".into()],
                entries: vec![entry.clone()],
            },
            the_entry_for_this_row: Some(entry),
            signed_by: vec!["online-1".into()],
            the_trust_record_to_keep: TheUpdateTrustRecord {
                greatest_accepted_release_sequence: 9,
                greatest_accepted_trust_sequence: 1,
                the_statement_accepted_at_that_trust_sequence: b"a statement".to_vec(),
                its_recovery_signature: [3; 64],
            },
        }
    }

    fn windows_asks() -> WhatTheRequestSays {
        WhatTheRequestSays {
            version: "0.1.8".into(),
            system: THE_WINDOWS_ROW.into(),
        }
    }

    fn installed_in(
        installation: &AnInstallation,
        serves: Vec<u8>,
        version: &str,
        name: &str,
        check: &dyn ChecksTheProgram,
    ) -> Result<AVerifiedBuild, WhyTheUpdateDidNotHappen> {
        let seam = AnArtifactAtTheSeam::serving(Some(serves));
        the_build_is_fetched_and_verified(
            &seam,
            &TheRunsTransfers::for_this_run(),
            &a_verified_update(version, the_windows_entry(&installation.download, name)),
            &windows_asks(),
            &installation.places,
            check,
            &installation.standing,
        )
    }

    // ── Steps 2 and 3 ──────────────────────────────────────────────────────

    /// **Z15: the new build goes into a new per-user folder named by its
    /// version, beside the running one, and the system's check is shown the
    /// program where it now sits** — and the running build is never touched.
    #[test]
    fn the_new_build_is_installed_side_by_side_and_checked_where_it_sits() {
        let installation = an_installation("side-by-side");
        let check = ACheckThatSays::accepting(true);
        let built = installed_in(&installation, installation.download.clone(), "0.2.0", "smoozy-desktop.exe", &check)
            .expect("installed and accepted");
        let expected = installation.places.builds.join("0.2.0").join("smoozy-desktop.exe");
        assert_eq!(built.program(), expected.as_path());
        assert_eq!(built.version(), "0.2.0");
        assert_eq!(std::fs::read(&expected).unwrap(), installation.download);
        assert_eq!(
            check.shown.lock().unwrap().clone(),
            vec![
                (installation.places.running.clone(), b"the running build".to_vec()),
                (expected.clone(), installation.download.clone()),
            ],
            "the check was not shown the program where it is installed"
        );
        assert_eq!(std::fs::read(&installation.places.running).unwrap(), b"the running build");
    }

    /// **A signed running build refuses an unsigned candidate**: an update
    /// that did not happen, with nothing left in the new folder.
    #[test]
    fn a_signed_running_build_refuses_an_unsigned_program() {
        let installation = an_installation("refused");
        let folder = installation.places.builds.join("0.2.0");

        let refusing = ACheckThatSays::for_builds(true, false);
        assert!(matches!(
            installed_in(&installation, installation.download.clone(), "0.2.0", "smoozy-desktop.exe", &refusing),
            Err(WhyTheUpdateDidNotHappen::TheSystemsOwnCheckRefusedIt(_))
        ));
        assert!(!folder.exists(), "a refused program was left installed");

        assert_eq!(std::fs::read(&installation.places.running).unwrap(), b"the running build");
    }

    /// Neither signature policy can offer corrupt bytes to the platform check.
    #[test]
    fn length_and_fingerprint_fail_before_any_platform_check_either_way() {
        for running in [false, true] {
            for bad_length in [false, true] {
                let installation = an_installation(&format!("bad-bytes-{running}-{bad_length}"));
                let mut bytes = installation.download.clone();
                let refused = if bad_length {
                    bytes.push(0);
                    WhyTheArtifactIsNotTheOneNamed::ItsLengthIsNotTheManifests {
                        named: installation.download.len() as u64,
                        found: bytes.len() as u64,
                    }
                } else {
                    bytes[3] ^= 1;
                    WhyTheArtifactIsNotTheOneNamed::ItsDigestIsNotTheManifests
                };
                let never = ACheckThatSays::for_builds(running, true);
                assert_eq!(
                    installed_in(&installation, bytes, "0.2.0", "smoozy-desktop.exe", &never).err(),
                    Some(WhyTheUpdateDidNotHappen::TheFetch(WhyTheFetchDidNotPass::ItIsNotTheArtifactNamed(refused)))
                );
                assert!(never.shown.lock().unwrap().is_empty(), "the system's check saw bytes the manifest refused");
                assert!(!installation.places.builds.join("0.2.0").exists());
                assert_eq!(std::fs::read(&installation.places.running).unwrap(), b"the running build");
            }
        }
    }

    /// E16: an unsigned running copy relies on the signed manifest's exact bytes.
    #[test]
    fn an_unsigned_running_build_accepts_an_unsigned_manifest_verified_program() {
        let installation = an_installation("unsigned-update");
        let check = ACheckThatSays::for_builds(false, false);
        let built = installed_in(&installation, installation.download.clone(), "0.2.0", "smoozy-desktop.exe", &check)
            .expect("the manifest vouches for an unsigned update to an unsigned copy");
        assert_eq!(std::fs::read(built.program()).unwrap(), installation.download);
        let shown = check.shown.lock().unwrap();
        assert_eq!(shown.len(), 1, "an unsigned running copy need not check the candidate's platform signature");
        assert_eq!(shown[0].0, installation.places.running);
    }

    /// E16: gaining a platform signature is allowed; losing one is refused.
    #[test]
    fn only_a_running_build_that_passes_requires_the_new_program_to_pass() {
        for (running, candidate) in [(false, true), (true, true), (true, false)] {
            let installation = an_installation(&format!("signature-{running}-{candidate}"));
            let check = ACheckThatSays::for_builds(running, candidate);
            let result = installed_in(&installation, installation.download.clone(), "0.2.0", "smoozy-desktop.exe", &check);
            assert_eq!(result.is_ok(), !running || candidate);
            if running && !candidate {
                assert!(matches!(result, Err(WhyTheUpdateDidNotHappen::TheSystemsOwnCheckRefusedIt(_))));
                assert!(!installation.places.builds.join("0.2.0").exists());
            }
            let shown = check.shown.lock().unwrap();
            assert_eq!(shown[0].0, installation.places.running);
            assert_eq!(shown.len(), if running { 2 } else { 1 });
        }
    }

    /// Read the running copy once across multiple verified downloads in this run.
    #[test]
    fn the_running_builds_standing_is_read_once_for_the_run() {
        for running in [false, true] {
            let installation = an_installation(&format!("standing-{running}"));
            let check = ACheckThatSays::for_builds(running, true);
            for version in ["0.2.0", "0.3.0"] {
                installed_in(&installation, installation.download.clone(), version, "smoozy-desktop.exe", &check).unwrap();
            }
            let shown = check.shown.lock().unwrap();
            assert_eq!(shown.iter().filter(|(path, _)| path == &installation.places.running).count(), 1);
            assert_eq!(shown.len(), if running { 3 } else { 1 });
        }
    }

    /// **Only one Windows program, and only into a folder its version alone
    /// can name** — never the folder the running program sits in.
    #[test]
    fn only_a_program_and_only_into_a_folder_its_version_names() {
        let installation = an_installation("odd");
        let check = ACheckThatSays::accepting(true);
        assert_eq!(
            installed_in(&installation, installation.download.clone(), "0.2.0", "Smoozy-windows.zip", &check).err(),
            Some(WhyTheUpdateDidNotHappen::NotAProgram)
        );
        for version in ["", "0.2", "0..2", "0.2.0-beta", "..", "0.2.0/../x"] {
            let result = installed_in(&installation, installation.download.clone(), version, "smoozy-desktop.exe", &check);
            if version == "0.2" {
                assert!(result.is_ok(), "{version:?}");
            } else {
                assert_eq!(result.err(), Some(WhyTheUpdateDidNotHappen::NoFolderForTheVersion), "{version:?}");
            }
        }
        assert_eq!(
            installed_in(&installation, installation.download.clone(), "0.1.8", "smoozy-desktop.exe", &check).err(),
            Some(WhyTheUpdateDidNotHappen::NoFolderForTheVersion),
            "the running build's own folder was offered as a new one"
        );
        assert_eq!(std::fs::read(&installation.places.running).unwrap(), b"the running build");
        assert!(a_program_name("SMOOZY.EXE") && !a_program_name(".exe") && !a_program_name("smoozy.exe.zip"));
    }

    /// **Z15's one previous build and no more**: every folder in the builds
    /// folder but the kept ones goes, and nothing outside it is looked at.
    #[test]
    fn every_other_build_is_removed_and_nothing_outside_the_builds_folder() {
        let installation = an_installation("others");
        let builds = &installation.places.builds;
        for old in ["0.1.6", "0.1.7", "0.3.0"] {
            std::fs::create_dir_all(builds.join(old)).unwrap();
            std::fs::write(builds.join(old).join("smoozy-desktop.exe"), old).unwrap();
        }
        let beside = installation.folder.join("Downloads");
        std::fs::create_dir_all(&beside).unwrap();
        std::fs::write(beside.join("smoozy-desktop.exe"), b"the user's own download").unwrap();
        let running_folder = installation.places.running.parent().unwrap().to_path_buf();
        assert!(the_other_builds_are_removed(builds, &[&running_folder]).is_empty());
        let left: Vec<String> = std::fs::read_dir(builds)
            .unwrap()
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(left, vec!["0.1.8".to_string()]);
        assert!(beside.join("smoozy-desktop.exe").exists(), "a file outside the builds folder was removed");
        assert!(the_other_builds_are_removed(&installation.folder.join("nowhere"), &[]).is_empty());
    }

    /// **No route skips the manifest's checks**: the one maker runs length
    /// and digest before E16's conditional platform check; publication takes
    /// nothing else.
    #[test]
    fn no_route_skips_the_check() {
        let (working, _) = include_str!("the_windows_side_by_side.rs")
            .split_once("#[cfg(test)]\nmod tests")
            .expect("the tests are last");
        assert_eq!(working.matches("Ok(AVerifiedBuild {").count(), 1);
        let maker = working
            .split_once("pub fn the_build_is_fetched_and_verified(")
            .and_then(|(_, rest)| rest.split_once("Ok(AVerifiedBuild {"))
            .map(|(body, _)| body)
            .expect("the maker is here");
        let fetched = maker.find("fetching::the_artifact_is_fetched(").expect("no fetch");
        let checked = maker.find("check.accepts(fetched.path())").expect("no system check");
        let standing = maker
            .find("standing.requires_a_signature(&places.running, check)")
            .expect("no running-build rule");
        assert!(fetched < standing && standing < checked);
        assert!(working.contains("built: AVerifiedBuild,\n    places: &ThePlaces,"));
        assert_eq!(working.matches("impl ChecksTheProgram for").count(), 1, "a second real form");
        // **E12's value, and nothing a user is asked**: the real call names no
        // one there, and draws no interface.
        assert!(working.contains("INVALID_HANDLE_VALUE as HWND"));
        assert!(working.contains("data.dwUIChoice = WTD_UI_NONE;"));
        assert!(working.contains("if status == 0 {"));
    }

    /// **The route is on the check's one sequence on Windows**, for a newer
    /// version found and for Z32's press, and the clearing a later launch does
    /// is this file's on Windows — read off the sources, since none of it can
    /// run here.
    #[test]
    fn the_route_is_wired_into_the_sequence_and_the_clearing() {
        let checking = include_str!("checking.rs");
        assert!(checking.contains(
            "} else if cfg!(target_os = \"windows\") {\n                            windows.a_newer_version_was_found(&going_on, verified.borrow().as_ref())"
        ));
        assert!(checking.contains(
            "} else if cfg!(target_os = \"windows\") {\n                            windows.the_held_update_goes_in(&going_on)"
        ));
        let probe = include_str!("the_probe.rs");
        let clearing = probe
            .split_once("pub fn what_the_record_kept_is_cleared(")
            .map(|(_, rest)| rest)
            .expect("the clearing is there");
        assert!(clearing.contains(
            "#[cfg(target_os = \"windows\")]\n    if let Ok(places) = super::the_windows_side_by_side::ThePlaces::for_this_copy()"
        ));
        assert!(probe.contains("#[cfg(windows)]\npub fn the_token_is_said_on_this_programs_output"));
        assert!(probe.contains("#[cfg(windows)]\npub fn the_watcher_lets_go_of_this_program"));
        let main = include_str!("../main.rs");
        assert!(!main.contains("#[cfg(not(unix))]\n            let _ = owed;"), "the probe's token is said on no Windows");
    }

    /// **The real check on this desk refuses**, because it is Windows' and
    /// this is not Windows — so nothing a Mac or Linux build could ever hand
    /// it passes.
    #[cfg(not(windows))]
    #[test]
    fn the_real_check_refuses_everything_off_windows() {
        let installation = an_installation("real-check");
        assert!(TheSystemsSignatureCheck.accepts(&installation.places.running).is_err());
    }

    /// **On Windows, the real check refuses an unsigned program** — this test
    /// program's own copy, which nothing has signed. **Runs on the hosted
    /// Windows job only.**
    #[cfg(windows)]
    #[test]
    fn the_real_check_refuses_an_unsigned_program_on_windows() {
        let installation = an_installation("real-check-windows");
        let copy = installation.folder.join("unsigned.exe");
        std::fs::copy(std::env::current_exe().unwrap(), &copy).unwrap();
        let refused = TheSystemsSignatureCheck.accepts(&copy);
        assert!(refused.is_err(), "an unsigned program passed WinVerifyTrust");
    }

    /// The real call refuses this unsigned executable, but E16 permits its
    /// manifest-verified copy as an update to an unsigned running executable.
    #[cfg(windows)]
    #[test]
    fn the_real_unsigned_running_build_accepts_a_manifest_verified_unsigned_update_on_windows() {
        let mut installation = an_installation("real-unsigned-update");
        std::fs::copy(std::env::current_exe().unwrap(), &installation.places.running).unwrap();
        installation.download = std::fs::read(&installation.places.running).unwrap();
        let built = installed_in(
            &installation,
            installation.download.clone(),
            "0.2.0",
            "smoozy-desktop.exe",
            &TheSystemsSignatureCheck,
        )
        .expect("an unsigned running copy accepts a manifest-verified unsigned update");
        assert!(built.program().exists());
        assert_eq!(installation.standing.requires_signed_updates.get(), Some(&false));
    }

    // ── Step 5 ─────────────────────────────────────────────────────────────

    /// **Publication, in Z15a's order**: E15's record and Z21a's record kept,
    /// the shortcut pointed at the new program, then the helper started with
    /// the new program, the running one and the record — and only then the
    /// hold let go. The running build is never touched.
    #[test]
    fn publication_points_the_shortcut_then_starts_the_helper_then_lets_the_hold_go() {
        let installation = an_installation("publish");
        let built = installed_in(
            &installation,
            installation.download.clone(),
            "0.2.0",
            "smoozy-desktop.exe",
            &ACheckThatSays::accepting(true),
        )
        .unwrap();
        let new_program = built.program().to_path_buf();
        record::take_the_copy(installation.record.path()).unwrap();
        let acts = TheActs::default();
        let mut hold = AHold { handed: false };
        let came = the_verified_build_is_published(built, &installation.places, &installation.record, &acts, &acts, &mut hold);
        assert_eq!(came, WhatThePublicationCameTo::HandedToTheHelper);
        assert!(hold.handed, "the hold was not let go");
        assert_eq!(
            acts.said.lock().unwrap().clone(),
            vec![
                format!("shortcut {}", new_program.display()),
                format!(
                    "helper {} {} {}",
                    new_program.display(),
                    installation.places.running.display(),
                    installation.record.path().display()
                ),
            ]
        );
        let probe = update_trust::the_update_in_probe(installation.record.connection()).unwrap().expect("Z21a's record");
        assert_eq!(probe.version, "0.2.0");
        assert_eq!(probe.the_previous_artifact, the_probe::path_bytes(&installation.places.running));
        assert_eq!(
            update_trust::read_the_trust_record(installation.record.connection())
                .unwrap()
                .map(|kept| kept.greatest_accepted_release_sequence),
            Some(9),
            "E15: the accepted sequence was not kept before publication"
        );
        assert!(!record::copy_beside(installation.record.path()).exists(), "a stale L8 copy was kept");
        assert_eq!(std::fs::read(&installation.places.running).unwrap(), b"the running build");
    }

    /// **A shortcut or a helper that will not go is an update that did not
    /// happen**: the shortcut is pointed back at the running program, the
    /// record is cleared, the hold is kept, and no helper is left running.
    #[test]
    fn a_shortcut_or_helper_that_will_not_go_publishes_nothing() {
        for (shortcut, helper) in [(true, false), (false, true)] {
            let installation = an_installation(if shortcut { "no-shortcut" } else { "no-helper" });
            let built = installed_in(
                &installation,
                installation.download.clone(),
                "0.2.0",
                "smoozy-desktop.exe",
                &ACheckThatSays::accepting(true),
            )
            .unwrap();
            let new_program = built.program().to_path_buf();
            let acts = TheActs {
                the_shortcut_refuses: shortcut,
                the_helper_refuses: helper,
                ..TheActs::default()
            };
            let mut hold = AHold { handed: false };
            let came = the_verified_build_is_published(built, &installation.places, &installation.record, &acts, &acts, &mut hold);
            assert!(matches!(
                came,
                WhatThePublicationCameTo::NotPublished(
                    WhyTheUpdateDidNotHappen::TheShortcut(_) | WhyTheUpdateDidNotHappen::TheHelper(_)
                )
            ));
            assert!(!hold.handed, "the hold was let go for an update that did not happen");
            assert_eq!(update_trust::the_update_in_probe(installation.record.connection()).unwrap(), None);
            let said = acts.said.lock().unwrap().clone();
            assert_eq!(said.first(), Some(&format!("shortcut {}", new_program.display())));
            assert_eq!(
                said.last(),
                Some(&format!("shortcut {}", installation.places.running.display())),
                "the shortcut was not pointed back at the running program"
            );
        }
    }

    // ── What a later launch clears ─────────────────────────────────────────

    /// **Z21a's fourth bullet on Windows**: the new build completing its own
    /// record removes the previous build's folder; the previous build after a
    /// rollback removes the failed one's; **a first download outside the
    /// builds folder, and the folder a program runs from, are never removed**;
    /// and the record is cleared last.
    #[test]
    fn a_completing_launch_removes_one_build_and_never_its_own_or_the_users_download() {
        let installation = an_installation("completes");
        let builds = installation.places.builds.clone();
        let previous = installation.places.running.clone();
        let new_program = builds.join("0.2.0").join("smoozy-desktop.exe");
        std::fs::create_dir_all(new_program.parent().unwrap()).unwrap();
        std::fs::write(&new_program, b"new").unwrap();
        let record_path = installation.record.path().to_path_buf();
        let probe_naming = |previous: &Path| update_trust::TheUpdateInProbe {
            release_sequence: 9,
            version: "0.2.0".into(),
            the_previous_artifact: the_probe::path_bytes(previous),
            the_record_copy: the_probe::path_bytes(&record::copy_beside(&record_path)),
        };

        // The new build, completing its own record: the previous folder goes.
        let as_the_new = ThePlaces { builds: builds.clone(), running: new_program.clone() };
        let probe = probe_naming(&previous);
        update_trust::an_update_goes_into_probe(installation.record.connection(), &probe).unwrap();
        record::take_the_copy(&record_path).unwrap();
        what_the_record_kept_is_cleared_here(installation.record.connection(), &record_path, &probe, &as_the_new, "0.2.0").unwrap();
        assert!(!previous.parent().unwrap().exists(), "the previous build outlived the new one's start");
        assert!(new_program.exists());
        assert!(!record::copy_beside(&record_path).exists(), "L8's copy was not discarded");
        assert_eq!(update_trust::the_update_in_probe(installation.record.connection()).unwrap(), None);

        // The previous build after a rollback: the failed folder goes.
        std::fs::create_dir_all(previous.parent().unwrap()).unwrap();
        std::fs::write(&previous, b"the running build").unwrap();
        let as_the_previous = installation.places.clone();
        update_trust::an_update_goes_into_probe(installation.record.connection(), &probe).unwrap();
        what_the_record_kept_is_cleared_here(installation.record.connection(), &record_path, &probe, &as_the_previous, "0.1.8").unwrap();
        assert!(!new_program.parent().unwrap().exists(), "the failed build outlived the rollback");
        assert!(previous.exists(), "the previous build removed itself");

        // A first download outside the builds folder is never removed.
        let download = installation.folder.join("Downloads").join("smoozy-desktop.exe");
        std::fs::create_dir_all(download.parent().unwrap()).unwrap();
        std::fs::write(&download, b"the user's own download").unwrap();
        let first = probe_naming(&download);
        assert_eq!(the_build_a_completion_removes(&as_the_new, &first, "0.2.0"), None);
        // Nor the folder the program runs from, whatever the record says.
        let itself = probe_naming(&new_program);
        assert_eq!(the_build_a_completion_removes(&as_the_new, &itself, "0.2.0"), None);
        let odd = update_trust::TheUpdateInProbe { version: "../x".into(), ..first.clone() };
        assert_eq!(the_build_a_completion_removes(&as_the_previous, &odd, "0.1.8"), None);
    }

    /// **A folder that will not go leaves the record**, so the next launch
    /// finishes it — the record is the last thing cleared.
    #[cfg(unix)]
    #[test]
    fn a_folder_that_will_not_go_leaves_the_record_for_the_next_launch() {
        use std::os::unix::fs::PermissionsExt;
        let installation = an_installation("stays");
        let builds = installation.places.builds.clone();
        let new_program = builds.join("0.2.0").join("smoozy-desktop.exe");
        std::fs::create_dir_all(new_program.parent().unwrap()).unwrap();
        std::fs::write(&new_program, b"new").unwrap();
        let previous = installation.places.running.clone();
        let probe = update_trust::TheUpdateInProbe {
            release_sequence: 9,
            version: "0.2.0".into(),
            the_previous_artifact: the_probe::path_bytes(&previous),
            the_record_copy: Vec::new(),
        };
        update_trust::an_update_goes_into_probe(installation.record.connection(), &probe).unwrap();
        // The builds folder made unwritable, so the previous folder cannot go.
        std::fs::set_permissions(&builds, std::fs::Permissions::from_mode(0o500)).unwrap();
        let as_the_new = ThePlaces { builds: builds.clone(), running: new_program };
        let cleared = what_the_record_kept_is_cleared_here(
            installation.record.connection(),
            installation.record.path(),
            &probe,
            &as_the_new,
            "0.2.0",
        );
        std::fs::set_permissions(&builds, std::fs::Permissions::from_mode(0o700)).unwrap();
        assert!(cleared.is_err());
        assert!(update_trust::the_update_in_probe(installation.record.connection()).unwrap().is_some());
    }
}

//! **The Linux update route: the box, the package transaction, the probe and
//! the reinstall** — Z10, Z10a, Z10b, Z11, Z12, Z21's Linux bullet, Z21a,
//! Z29, Z31 and Z32, with L8 and L13 used. Begun by GPT-6 (Codex) and finished
//! by Claude Opus 5.5 at step 11's Linux update piece.
//!
//! **What runs where.** Everything that decides — the offer, the declined
//! mark, the order of the transaction, the probe and the reinstall, the
//! retained package, the durable rollback phase and its completion at a later
//! launch — is ordinary code compiled and tested on every system. **The only
//! Linux-only code is [`ThePackageSystem`]'s real form**, at the foot of this
//! file: PackageKit's transaction on the system bus, and `dpkg-deb` and
//! `dpkg-query` read for one field at a time. **Every Linux fact behind it is
//! documentation alone (Z12)**; the readings are in the Linux update piece's
//! account and its sources file.
//!
//! **The order.**
//!
//! 1. **Found** ([`LinuxRoute::found`]): a check found a newer version. The
//!    box is offered where Z10b and Z29 allow it and nothing is running
//!    (Z31); otherwise About says what stands.
//! 2. **Answered** on the check's one sequence. *Not now*, Escape, an orderly
//!    quit with the box open and a declined system authentication write the
//!    declined mark with the offered version (Z10b). **A failed install writes
//!    none** (step 11's D6b, the runner's ruling): it is not a decline.
//! 3. **Prepared** ([`prepare`]): the exact running release's signed update
//!    file and package fetched from that release, verified and retained under
//!    the per-user cache (Z11); then the new package fetched. Both through the
//!    shared admitted, budgeted fetch and its length and digest check (E16).
//! 4. **Before publication**: L8's stale copy discarded, E15's record kept,
//!    Z21a's record written naming the retained package.
//! 5. **The transaction** ([`transact`]): the system's own authentication is
//!    the one request (Z10a). A declined authentication is an update that did
//!    not happen. A failed transaction that left the running version installed
//!    is one too; any other failure is treated as published.
//! 6. **The probe**: the installed program started in the update-launch mode
//!    with the token, P2's hold handed over, the token watched for.
//! 7. **On anything but the token, the rollback**: the rollback phase written
//!    first, the retained package reinstalled with no interaction allowed, the
//!    phase moved on, **L8's copy restored by an opening that migrates
//!    nothing**, the record cleared as the last act, and the previous build
//!    started in the update-launch mode alone.
//! 8. **Where the system will not take the reinstall unattended** (item 47),
//!    the phase stays written, **the old process starts the installed program
//!    in the update-launch mode** and exits, so the later launch comes at once
//!    rather than Smoozy disappearing (D6b). **That launch meets the phase
//!    before any ordinary module** ([`what_this_launch_meets`]) **and shows the
//!    one box Design draws for it**: the reinstall is asked for only when the
//!    user presses *Reinstall*, the rest of step 7 follows, and a declined or
//!    failed request is said in the same box, which offers it again
//!    ([`TheRollbackBox`]). A launch that owes only the restore and cleanup
//!    finishes them with no box and no request.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` I9.

use super::checking::{self, AdmitsTheUpdateFetch, ReadsTheUpdateFile, TheReading};
use super::the_mac::HandsTheHold;
use super::{fetching, manifest::AnEntry, the_probe, verifying};
use crate::platform::files;
use crate::record::{update_trust, LocalRecord};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

// ════════════════════════════════════════════════════════════════════════════
// The offer and the mark
// ════════════════════════════════════════════════════════════════════════════

/// **Whether the box is offered** (Z10b, Z29, Z31): a version above the
/// running one, nothing held, and — on the automatic route only — above the
/// version the declined mark names.
pub fn should_offer(
    version: &str,
    running: &str,
    declined: Option<&str>,
    user_asked: bool,
    held: bool,
) -> bool {
    !held
        && checking::a_version_above(version, running)
        && (user_asked
            || declined.is_none_or(|declined| checking::a_version_above(version, declined)))
}

/// **What a found version comes to on Linux.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheFindingComesTo {
    /// Nothing is offered: on the automatic route, the mark names it. About
    /// reads the Linux form, never that it will install by itself.
    Quiet,
    /// Something is running (Z31): About's held form, and no box.
    Held,
    /// The box is offered.
    TheBox,
}

/// **The decision [`LinuxRoute::found`] acts on.** `held` is Z31's reading,
/// asked only where the box would otherwise be offered.
pub fn what_the_finding_comes_to(
    version: &str,
    running: &str,
    declined: Option<&str>,
    user_asked: bool,
    held: &dyn Fn() -> bool,
) -> WhatTheFindingComesTo {
    if !should_offer(version, running, declined, user_asked, false) {
        WhatTheFindingComesTo::Quiet
    } else if held() {
        WhatTheFindingComesTo::Held
    } else {
        WhatTheFindingComesTo::TheBox
    }
}

/// **Z10b's mark, written at the dismissal** with the version offered.
pub fn dismissed(record: &rusqlite::Connection, version: &str) -> Result<(), String> {
    update_trust::this_version_was_declined(record, version).map_err(|e| format!("{e:?}"))
}

// ════════════════════════════════════════════════════════════════════════════
// The transaction, the probe and the reinstall
// ════════════════════════════════════════════════════════════════════════════

/// What a package transaction came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallFailure {
    /// **The system's own authentication was declined or refused** (Z10a).
    Authorization,
    /// Anything else, in the system's words.
    Failed(String),
}

/// PackageKit's error code read: only *not authorized* (48 in
/// `PkErrorEnum`) is a declined authentication. A cancelled transaction (17)
/// may have begun its work, so it is a failure (documentation alone).
fn package_error(code: u32, details: String) -> InstallFailure {
    if code == 48 {
        InstallFailure::Authorization
    } else {
        InstallFailure::Failed(details)
    }
}

/// PackageKit's ending read: 1 in `PkExitEnum` is success, and only with no
/// error code before it (documentation alone).
fn package_ending(exit: u32, failure: Option<InstallFailure>) -> Result<(), InstallFailure> {
    match (exit, failure) {
        (1, None) => Ok(()),
        (_, Some(e)) => Err(e),
        _ => Err(InstallFailure::Failed(format!("PackageKit exit {exit}"))),
    }
}

/// **The acts the transaction is made of**, so the order is tested at the
/// desk. The real form is [`Steps`]; the tests hand in stand-ins.
pub trait PackageSteps {
    /// The new package (`previous` false) or the retained one, through the
    /// system's package mechanism. `may_ask` is whether the system may ask
    /// for authentication.
    fn install(&mut self, previous: bool, may_ask: bool) -> Result<(), InstallFailure>;
    /// After a failed forward transaction: whether the package database
    /// still has the running version correctly installed, so nothing was
    /// published.
    fn nothing_was_published(&mut self) -> bool;
    /// Z21's probe: whether the token came back within the deadline.
    fn probe(&mut self) -> bool;
    /// L13: L8's copy put back, the phase carried into it.
    fn restore_record(&mut self) -> Result<(), String>;
    /// The record cleared as the last act, then the previous build started in
    /// the update-launch mode alone.
    fn restart_previous(&mut self) -> Result<(), String>;
    /// **Where the rollback cannot finish now** (D6b): the installed program
    /// started in the update-launch mode, so the later launch that finishes
    /// it, and says so, comes at once rather than Smoozy disappearing.
    fn the_next_launch_is_started(&mut self) -> Result<(), String>;
}

/// What the transaction came to.
#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The system's authentication was declined: the mark is written.
    Declined,
    /// The transaction failed and the running version is still installed:
    /// an update that did not happen.
    NotPublished,
    /// The new build said its token.
    Installed,
    /// The previous build is back and started.
    RolledBack,
    /// The rollback could not finish now; the phase is written and a later
    /// launch finishes it.
    RecoveryOwed,
    /// The declined mark could not be written.
    Failed,
}

/// **Steps 5 to 7 of the order at the head of this file.**
pub fn transact(record: &LocalRecord, version: &str, steps: &mut dyn PackageSteps) -> Outcome {
    match steps.install(false, true) {
        Err(InstallFailure::Authorization) => {
            return if dismissed(record.connection(), version).is_ok() {
                Outcome::Declined
            } else {
                Outcome::Failed
            };
        }
        // **A package mechanism can change files before it fails**, so a
        // failure is an update that did not happen only where the package
        // database says the running version is still correctly installed.
        Err(InstallFailure::Failed(_)) => {
            if steps.nothing_was_published() {
                return Outcome::NotPublished;
            }
        }
        Ok(()) => {
            if steps.probe() {
                return Outcome::Installed;
            }
        }
    }
    let outcome = the_rollback(record.connection(), steps);
    // **Smoozy does not go quiet here** (Rule 8, D6b): the later launch is
    // started now, and it meets the phase and shows the box. A start that
    // fails leaves the phase for the user's own next start, which shows it.
    if outcome == Outcome::RecoveryOwed {
        let _ = steps.the_next_launch_is_started();
    }
    outcome
}

/// **Whether the declined mark is written once *Install*'s transaction is
/// over** (Z10b, as the runner ruled at step 11's D6b): only a dismissal writes
/// it, and a declined system request is one — [`transact`] has written it, and
/// it is written again here only where that write failed. **A failed install
/// is not a decline**, so it writes none and the automatic route may offer the
/// version again.
pub fn the_mark_is_written_after(result: &Result<Outcome, String>) -> bool {
    matches!(result, Ok(Outcome::Failed))
}

/// **Z21's Linux rollback.** The phase is written **before** the reinstall,
/// so a death during it still owes the rollback; it moves on once the
/// reinstall is done, so a later launch never asks for a package twice.
fn the_rollback(record: &rusqlite::Connection, steps: &mut dyn PackageSteps) -> Outcome {
    if update_trust::a_linux_rollback_is_required(record).is_err() {
        return Outcome::RecoveryOwed;
    }
    if steps.install(true, false).is_err() {
        return Outcome::RecoveryOwed;
    }
    if update_trust::the_linux_reinstall_finished(record).is_err() {
        return Outcome::RecoveryOwed;
    }
    if steps.restore_record().is_err() {
        return Outcome::RecoveryOwed;
    }
    if steps.restart_previous().is_err() {
        return Outcome::RecoveryOwed;
    }
    Outcome::RolledBack
}

/// **Z10a's fallback, at a later launch**: the user is there by definition.
/// One package request where the reinstall is still owed, then the record
/// restored, then the previous build, with no ordinary work between.
pub fn finish_recovery(steps: &mut dyn PackageSteps, already_reinstalled: bool) -> Result<(), String> {
    if !already_reinstalled {
        steps.install(true, true).map_err(|e| format!("{e:?}"))?;
    }
    steps.restore_record()?;
    steps.restart_previous()
}

// ════════════════════════════════════════════════════════════════════════════
// The retained package (Z11)
// ════════════════════════════════════════════════════════════════════════════

/// **The per-user cache** (F11's per-user application data, `linux-update`):
/// `previous` holds the running release's verified package and its signed
/// update file, `next` the new package. Its place comes from the platform
/// adapter alone; a record never names a different cache into use.
pub struct Cache {
    pub root: PathBuf,
}

impl Cache {
    pub fn here() -> Result<Self, String> {
        Ok(Self {
            root: crate::platform::locations::the_linux_update_cache()
                .map_err(|e| format!("{e:?}"))?,
        })
    }
    fn previous(&self) -> PathBuf {
        self.root.join("previous")
    }
    fn next(&self) -> PathBuf {
        self.root.join("next")
    }
    fn proof(&self) -> PathBuf {
        self.previous().join(super::THE_UPDATE_FILES_NAME)
    }
    /// Whether the record's retained artifact is in this cache's `previous`.
    fn is_named_by(&self, probe: &update_trust::TheUpdateInProbe) -> bool {
        path_from_bytes(&probe.the_previous_artifact)
            .is_ok_and(|named| named.parent() == Some(self.previous().as_path()))
    }
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let written = files::write_whole_file(path, bytes, files::Durability::ToTheDevice);
    if written.wrote_the_file() {
        Ok(())
    } else {
        Err(format!("{written:?}"))
    }
}

/// E16's length and digest, read again from the file as it stands.
fn check_bytes(path: &Path, entry: &AnEntry) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    verifying::the_artifact_is_the_one_named(
        entry,
        bytes.len() as u64,
        &smoozy_server_protocol::transfers::integrity::Digest::of(&bytes),
    )
    .map_err(|e| format!("{e:?}"))
}

/// **Both packages, verified and retained.** Only [`prepare`] makes one, so
/// the package mechanism is handed nothing that skipped the checks.
pub struct VerifiedPackages {
    next: fetching::AFetchedArtifact,
    previous: fetching::AFetchedArtifact,
    update: verifying::AVerifiedUpdate,
}

/// **Step 3**: the exact running release first, from that release's own
/// address (E10a's repository, the own-release form `Server protocol and
/// transfers` B8d uses), then the new package by the ordinary fetch.
#[allow(clippy::too_many_arguments)]
pub fn prepare(
    cache: &Cache,
    file: &dyn ReadsTheUpdateFile,
    admission: &dyn AdmitsTheUpdateFetch,
    carried: &super::trust::TheCarriedSet,
    remembered: Option<&super::trust::TheUpdateTrustRecord>,
    update: &verifying::AVerifiedUpdate,
    running: &str,
    row: &str,
) -> Result<VerifiedPackages, String> {
    if !checking::a_version_above(&update.manifest.version, running) || !is_linux_row(row) {
        return Err("the release or Linux row is not eligible".into());
    }
    let next_entry = update
        .the_entry_for_this_row
        .as_ref()
        .filter(|e| e.row == row && e.file_name.ends_with(".deb"))
        .ok_or("no package for this Linux row")?;
    files::a_folder_is_removed_whole(&cache.root).map_err(|e| e.reported)?;
    files::ensure_directory_at_mode(&cache.previous(), 0o700).map_err(|e| format!("{e:?}"))?;
    files::ensure_directory_at_mode(&cache.next(), 0o700).map_err(|e| format!("{e:?}"))?;
    let says = checking::WhatTheRequestSays {
        version: running.into(),
        system: format!("linux:{}", std::env::consts::ARCH),
    };
    let (repository, _) = checking::WHERE_THE_UPDATE_FILE_IS_READ_FROM
        .split_once("/releases/")
        .ok_or("no release repository")?;
    let own_file = format!(
        "{repository}/releases/download/v{running}/{}",
        super::THE_UPDATE_FILES_NAME
    );
    let bytes = fetching::bytes_are_fetched(
        file,
        admission,
        &own_file,
        &says,
        checking::THE_MOST_THE_UPDATE_FILE_MAY_BE,
    )
    .map_err(|e| format!("{e:?}"))?;
    // **The rollback-only reading** (Z21a's last paragraph): the running
    // release is below the stored high-water once a refused install raised
    // it, so the floor alone is set aside here. The trusted keys bind, the
    // stored sequence is never lowered, and the package is never admitted
    // as a forward update.
    let mut previous_trust = remembered.cloned();
    if let Some(remembered) = previous_trust.as_mut() {
        remembered.greatest_accepted_release_sequence = 0;
    }
    let previous =
        verifying::verify_an_update_file_against(carried, previous_trust.as_ref(), &bytes, row)
            .map_err(|e| format!("{e:?}"))?;
    if previous.manifest.version != running
        || previous.manifest.release_sequence >= update.manifest.release_sequence
    {
        return Err("the retained package is not the immediately previous running release".into());
    }
    let previous_entry = previous
        .the_entry_for_this_row
        .as_ref()
        .filter(|e| e.file_name.ends_with(".deb"))
        .ok_or("no previous package")?;
    let (own_folder, _) = own_file.rsplit_once('/').ok_or("no own release folder")?;
    let previous = fetching::the_artifact_is_fetched_from(
        file,
        admission,
        previous_entry,
        &says,
        &cache.previous(),
        &format!("{own_folder}/{}", previous_entry.file_name),
    )
    .map_err(|e| format!("{e:?}"))?;
    write(&cache.proof(), &bytes)?;
    let next = fetching::the_artifact_is_fetched(file, admission, next_entry, &says, &cache.next())
        .map_err(|e| format!("{e:?}"))?;
    Ok(VerifiedPackages {
        next,
        previous,
        update: update.clone(),
    })
}

fn is_linux_row(row: &str) -> bool {
    [
        super::manifest::THE_UBUNTU_24_04_ROW,
        super::manifest::THE_UBUNTU_26_04_ROW,
        super::manifest::THE_DEBIAN_13_ROW,
    ]
    .contains(&row)
}

/// **Step 4**: both digests read again, L8's stale copy discarded (L13), E15's
/// record kept, and Z21a's record written, immediately before publication.
fn before_publication(record: &LocalRecord, packages: &VerifiedPackages) -> Result<(), String> {
    check_bytes(packages.next.path(), packages.next.entry())?;
    check_bytes(packages.previous.path(), packages.previous.entry())?;
    crate::record::discard_the_copy(record.path()).map_err(|e| format!("{e:?}"))?;
    verifying::keep(record.connection(), &packages.update.the_trust_record_to_keep)
        .map_err(|e| format!("{e:?}"))?;
    update_trust::an_update_goes_into_probe(
        record.connection(),
        &update_trust::TheUpdateInProbe {
            release_sequence: packages.update.manifest.release_sequence,
            version: packages.update.manifest.version.clone(),
            the_previous_artifact: the_probe::path_bytes(packages.previous.path()),
            the_record_copy: the_probe::path_bytes(&crate::record::copy_beside(record.path())),
        },
    )
    .map_err(|e| format!("{e:?}"))
}

/// **Z21a's clearing on Linux**: L8's copy discarded, the cache removed, the
/// record cleared last — and only where the record names this cache and this
/// record's own copy.
pub fn cleared(
    record: &LocalRecord,
    cache: &Cache,
    probe: &update_trust::TheUpdateInProbe,
) -> Result<(), String> {
    cleared_at(record.connection(), record.path(), cache, probe)
}

pub(crate) fn cleared_at(
    connection: &rusqlite::Connection,
    record_path: &Path,
    cache: &Cache,
    probe: &update_trust::TheUpdateInProbe,
) -> Result<(), String> {
    if !cache.is_named_by(probe) {
        return Err("the probe does not name this cache".into());
    }
    if probe.the_record_copy != the_probe::path_bytes(&crate::record::copy_beside(record_path)) {
        return Err("the probe does not name this record's copy".into());
    }
    crate::record::discard_the_copy(record_path).map_err(|e| format!("{e:?}"))?;
    files::a_folder_is_removed_whole(&cache.root).map_err(|e| e.reported)?;
    update_trust::the_probe_is_cleared(connection).map_err(|e| format!("{e:?}"))
}

/// **The probe's own clearing and an ordinary launch's** (Z21a), where the
/// record names `cache`: `None` where it names something else, which is then
/// the shared clearing's.
pub(crate) fn cleared_if_it_names(
    connection: &rusqlite::Connection,
    record_path: &Path,
    cache: &Cache,
    probe: &update_trust::TheUpdateInProbe,
) -> Option<Result<(), String>> {
    cache
        .is_named_by(probe)
        .then(|| cleared_at(connection, record_path, cache, probe))
}

fn path_from_bytes(bytes: &[u8]) -> Result<PathBuf, String> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        Ok(PathBuf::from(std::ffi::OsString::from_vec(bytes.to_vec())))
    }
    #[cfg(not(unix))]
    {
        let _ = bytes;
        Err("Linux paths require Unix".into())
    }
}

/// **The retained package, at a later launch**: its signed file verified
/// against the trusted keys with the floor alone set aside, its version below
/// the failed one, its place the record's and this cache's, its bytes the
/// signed ones.
fn retained_package(
    cache: &Cache,
    record: &rusqlite::Connection,
    probe: &update_trust::TheUpdateInProbe,
    carried: &super::trust::TheCarriedSet,
    row: &str,
) -> Result<(PathBuf, AnEntry), String> {
    let mut remembered =
        verifying::the_kept_record_as_it_stands(record, carried).map_err(|e| format!("{e:?}"))?;
    if let Some(remembered) = remembered.as_mut() {
        remembered.greatest_accepted_release_sequence = 0;
    }
    let bytes = std::fs::read(cache.proof()).map_err(|e| e.to_string())?;
    let verified =
        verifying::verify_an_update_file_against(carried, remembered.as_ref(), &bytes, row)
            .map_err(|e| format!("{e:?}"))?;
    if !checking::a_version_above(&probe.version, &verified.manifest.version)
        || verified.manifest.release_sequence >= probe.release_sequence
    {
        return Err("the cached release is not previous".into());
    }
    let entry = verified
        .the_entry_for_this_row
        .filter(|e| e.file_name.ends_with(".deb"))
        .ok_or("no previous package")?;
    if fetching::where_the_artifact_is(&entry.file_name).is_none() {
        return Err("invalid cached name".into());
    }
    let path = cache.previous().join(&entry.file_name);
    if probe.the_previous_artifact != the_probe::path_bytes(&path) {
        return Err("the record names another package".into());
    }
    check_bytes(&path, &entry)?;
    Ok((path, entry))
}

/// **L13's restore for a Linux rollback**, the phase carried into the
/// restored record so an interrupted cleanup is finished without a package.
/// **The record is opened without migrating**: at a later launch the build
/// doing this is the new one, and migrating the restored copy would hand the
/// previous build a record it cannot read.
fn the_record_is_restored_owing_cleanup(
    record_path: &Path,
    probe: &update_trust::TheUpdateInProbe,
) -> Result<(), String> {
    if crate::record::copy_beside(record_path).exists() {
        crate::record::restore_the_copy(record_path).map_err(|e| format!("{e:?}"))?;
    }
    let restored = opened_as_it_stands(record_path)?;
    update_trust::an_update_goes_into_probe(&restored, probe).map_err(|e| format!("{e:?}"))?;
    update_trust::the_linux_reinstall_finished(&restored).map_err(|e| format!("{e:?}"))
}

/// **The rollback's last act** (Z21a), through the same kind of opening.
fn the_rollback_is_cleared(
    record_path: &Path,
    cache: &Cache,
    probe: &update_trust::TheUpdateInProbe,
) -> Result<(), String> {
    let restored = opened_as_it_stands(record_path)?;
    cleared_at(&restored, record_path, cache, probe)
}

/// The record opened as it stands: no migration, no creation.
fn opened_as_it_stands(record_path: &Path) -> Result<rusqlite::Connection, String> {
    rusqlite::Connection::open_with_flags(record_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE)
        .map_err(|e| e.to_string())
}

/// **The build a rollback starts, and the later launch that finishes one**:
/// the installed program in the update-launch mode alone, no token (Z21b's
/// last bullet, `Reporting a problem` U5b). The mode is also what makes the
/// later launch wait for the old process's hold rather than end (P2).
fn the_installed_program_is_started(program: &Path) -> Result<(), String> {
    std::process::Command::new(program)
        .arg(crate::start::THE_UPDATE_LAUNCH_ARGUMENT)
        .env_remove(the_probe::THE_PROBES_TOKEN)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map(drop)
        .map_err(|e| e.to_string())
}

// ════════════════════════════════════════════════════════════════════════════
// The system's own package mechanism, as an adapter
// ════════════════════════════════════════════════════════════════════════════

/// **The system's package calls.** The real form is Linux's alone and sits
/// at the foot of this file; nothing at the desk calls a package tool.
pub trait ThePackageSystem {
    /// One transaction installing the local package at `package`.
    /// `previous` allows the reinstall and the downgrade; `may_ask` is whether
    /// the system may ask for authentication.
    fn install(&self, package: &Path, previous: bool, may_ask: bool) -> Result<(), InstallFailure>;
    /// The package's own name, version, CPU and release-matrix row fields
    /// agree with what the signed manifest says it is.
    fn names(&self, package: &Path, version: &str, row: &str) -> Result<(), String>;
    /// The package database has `version` of this package correctly
    /// installed.
    fn has_installed(&self, version: &str) -> bool;
}

/// **The real acts of the forward transaction**, in the old process.
struct Steps<'a, P: ThePackageSystem, H: HandsTheHold> {
    system: &'a P,
    packages: &'a VerifiedPackages,
    record_path: PathBuf,
    cache: &'a Cache,
    hold: H,
    program: PathBuf,
    owns_hold: bool,
}

impl<P: ThePackageSystem, H: HandsTheHold> PackageSteps for Steps<'_, P, H> {
    fn install(&mut self, previous: bool, may_ask: bool) -> Result<(), InstallFailure> {
        if previous && !self.owns_hold {
            return Err(InstallFailure::Failed("the instance hold was not taken back".into()));
        }
        let artifact = if previous {
            &self.packages.previous
        } else {
            &self.packages.next
        };
        check_bytes(artifact.path(), artifact.entry()).map_err(InstallFailure::Failed)?;
        self.system.install(artifact.path(), previous, may_ask)
    }

    fn nothing_was_published(&mut self) -> bool {
        self.system.has_installed(checking::THIS_BUILDS_VERSION)
    }

    fn probe(&mut self) -> bool {
        let Some(token) = the_probe::a_token() else {
            return false;
        };
        let mut command = std::process::Command::new(&self.program);
        command.arg(crate::start::THE_UPDATE_LAUNCH_ARGUMENT);
        let Ok(mut watched) = the_probe::AProbeUnderWatch::starts(command, &token) else {
            return false;
        };
        self.hold.to_the_probe();
        self.owns_hold = false;
        if watched.is_watched(the_probe::THE_LAUNCH_DEADLINE)
            == the_probe::WhatTheProbeCameTo::ItStarted
        {
            drop(watched.is_let_go());
            true
        } else {
            watched.is_ended();
            self.owns_hold = self.hold.back();
            false
        }
    }

    fn restore_record(&mut self) -> Result<(), String> {
        let probe = the_probe_as_it_stands(&self.record_path)?;
        the_record_is_restored_owing_cleanup(&self.record_path, &probe)
    }

    fn restart_previous(&mut self) -> Result<(), String> {
        let probe = the_probe_as_it_stands(&self.record_path)?;
        the_rollback_is_cleared(&self.record_path, self.cache, &probe)?;
        the_installed_program_is_started(&self.program)
    }

    /// The installed program is the build that failed its probe, or the
    /// previous one where the reinstall finished; either meets the phase.
    fn the_next_launch_is_started(&mut self) -> Result<(), String> {
        the_installed_program_is_started(&self.program)
    }
}

/// Z21a's record read through an opening that migrates nothing.
fn the_probe_as_it_stands(record_path: &Path) -> Result<update_trust::TheUpdateInProbe, String> {
    update_trust::the_update_in_probe(&opened_as_it_stands(record_path)?)
        .map_err(|e| format!("{e:?}"))?
        .ok_or_else(|| "no recovery record".to_string())
}

/// **The acts at a later launch**, before the shell is built.
struct RecoverySteps<P: ThePackageSystem> {
    system: Option<P>,
    package: PathBuf,
    record_path: PathBuf,
    cache: Cache,
    probe: update_trust::TheUpdateInProbe,
    program: PathBuf,
}

impl<P: ThePackageSystem> PackageSteps for RecoverySteps<P> {
    fn install(&mut self, previous: bool, may_ask: bool) -> Result<(), InstallFailure> {
        if !previous {
            return Err(InstallFailure::Failed("recovery installs no new package".into()));
        }
        let system = self
            .system
            .as_ref()
            .ok_or_else(|| InstallFailure::Failed("the reinstall already finished".into()))?;
        system.install(&self.package, true, may_ask)
    }

    fn nothing_was_published(&mut self) -> bool {
        false
    }

    fn probe(&mut self) -> bool {
        false
    }

    fn restore_record(&mut self) -> Result<(), String> {
        the_record_is_restored_owing_cleanup(&self.record_path, &self.probe)
    }

    fn restart_previous(&mut self) -> Result<(), String> {
        the_rollback_is_cleared(&self.record_path, &self.cache, &self.probe)?;
        the_installed_program_is_started(&self.program)
    }

    /// This is that later launch, so it starts none.
    fn the_next_launch_is_started(&mut self) -> Result<(), String> {
        Err("this is the later launch".into())
    }
}

/// **Step 8, at a later launch**: called with the instance hold, before the
/// shell and every ordinary module. Whether a rollback was finished, in which
/// case this launch ends. An error leaves the phase written and admits no
/// ordinary work.
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn recover_with<P: ThePackageSystem>(
    record_path: &Path,
    cache: Cache,
    row: Option<&str>,
    carried: &super::trust::TheCarriedSet,
    connect: &dyn Fn() -> Result<P, String>,
    program: PathBuf,
) -> Result<bool, String> {
    if !record_path.exists() {
        return Ok(false);
    }
    let connection = rusqlite::Connection::open_with_flags(
        record_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .map_err(|e| e.to_string())?;
    // **Read without migrating**: the column is version 52's, and a record
    // below it owes nothing of this route.
    if crate::record::schema::version_in(&connection).map_err(|e| format!("{e:?}"))? < 52 {
        return Ok(false);
    }
    let phase = update_trust::the_linux_rollback_phase(&connection).map_err(|e| format!("{e:?}"))?;
    if phase == 0 {
        return Ok(false);
    }
    let probe = update_trust::the_update_in_probe(&connection)
        .map_err(|e| format!("{e:?}"))?
        .ok_or("no recovery record")?;
    let package = match phase {
        1 => {
            let row = row.ok_or("unlisted Linux system")?;
            retained_package(&cache, &connection, &probe, carried, row)?.0
        }
        2 => {
            if !cache.is_named_by(&probe) {
                return Err("the probe does not name this cache".into());
            }
            path_from_bytes(&probe.the_previous_artifact)?
        }
        _ => return Err("unknown Linux recovery phase".into()),
    };
    drop(connection);
    let system = if phase == 1 { Some(connect()?) } else { None };
    let mut steps = RecoverySteps {
        system,
        package,
        record_path: record_path.to_path_buf(),
        cache,
        probe,
        program,
    };
    finish_recovery(&mut steps, phase == 2)?;
    Ok(true)
}

// ════════════════════════════════════════════════════════════════════════════
// The later launch's box (D6b)
// ════════════════════════════════════════════════════════════════════════════

/// **What a launch meets before anything else, on Linux** — step 8, and the
/// D6b line under Z11: Smoozy does not go quiet while a rollback is owed.
pub enum TheLaunchMeets {
    /// Nothing is owed: the ordinary launch goes on.
    Nothing,
    /// Only the restore and cleanup were owed, and they finished with no
    /// request: the previous build is started and this launch ends.
    TheRollbackFinished,
    /// **The box, and nothing else in the window**: the reinstall is owed, or
    /// the rest of the rollback could not finish.
    TheBox(TheRollbackBox),
    /// **The record could not be read for the phase at that moment**
    /// (2026-11-01, the D6b line under Z11): a box with *Quit* alone, and
    /// **never the ordinary launch**, which could complete the failed build's
    /// probe (Z21a). Until then this launch ended with nothing on screen.
    TheRecordCouldNotBeRead(TheUnreadableRecordBox),
}

impl TheLaunchMeets {
    /// **The box this launch shows, posted on the one queue and kept in the
    /// shell's managed state**, where its answers arrive (`surface::menus`).
    /// The two outcomes that show no box post nothing.
    pub fn is_posted_and_kept<R: tauri::Runtime, M: tauri::Manager<R>>(
        self,
        app: &M,
        queue: &crate::events::TheOneQueue,
    ) {
        match self {
            Self::TheBox(the_box) => {
                the_box.is_posted_on(queue, the_box.opens_failed);
                app.manage(Arc::new(the_box));
            }
            Self::TheRecordCouldNotBeRead(the_box) => {
                the_box.is_posted_on(queue);
                app.manage(Arc::new(the_box));
            }
            Self::Nothing | Self::TheRollbackFinished => {}
        }
    }
}

/// **The box a launch shows where the record could not be read for the
/// phase**: no version to name and nothing to reinstall from, so it says what
/// happened and offers *Quit* alone. Kept in the shell's managed state, so
/// *Quit* has somewhere to arrive.
pub struct TheUnreadableRecordBox {
    /// Whether an update is known to have been under way: the record named an
    /// owed phase before a later read failed, or Z11's cache still holds the
    /// previous package. Only then does the box name the update.
    pub the_update_was_underway: bool,
}

impl TheUnreadableRecordBox {
    /// The box posted on the one queue.
    pub fn is_posted_on(&self, queue: &crate::events::TheOneQueue) {
        let _ = queue.read_then_post(
            || Ok::<_, ()>(()),
            |_| crate::surface::menus::the_unreadable_record_box(self.the_update_was_underway),
        );
    }
}

/// **The box a later launch shows over an owed rollback** (Design, *Updating*,
/// D6b). Kept in the shell's managed state for the run, which is the box's
/// whole run: no ordinary module opens beside it.
pub struct TheRollbackBox {
    record_path: PathBuf,
    cache_root: PathBuf,
    row: Option<String>,
    program: PathBuf,
    /// The version that could not start, from Z21a's record.
    pub failed_version: String,
    /// Whether the box opens on *Reinstall failed.*
    pub opens_failed: bool,
    /// Whether the system will be asked: false where only the restore and
    /// cleanup are left, so the box says nothing about a password.
    pub asks_the_system: bool,
    /// One reinstall at a time; a press while one runs is not a second.
    trying: std::sync::atomic::AtomicBool,
}

/// **The decision [`what_this_launch_meets`] acts on**, over any package
/// system. The record is read without migrating; **the system is asked for
/// nothing here** — the reinstall is the box's press.
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn what_the_launch_meets_with<P: ThePackageSystem>(
    record_path: &Path,
    cache: Cache,
    row: Option<&str>,
    carried: &super::trust::TheCarriedSet,
    connect: &dyn Fn() -> Result<P, String>,
    program: PathBuf,
) -> Result<TheLaunchMeets, String> {
    if !record_path.exists() {
        return Ok(TheLaunchMeets::Nothing);
    }
    // **A record that cannot be read for the phase is said** (2026-11-01, the
    // D6b line under Z11), with the update named only where it is known.
    let unread = |known: bool, cache: &Cache| {
        TheLaunchMeets::TheRecordCouldNotBeRead(TheUnreadableRecordBox {
            the_update_was_underway: known || cache.previous().exists(),
        })
    };
    let (phase, failed_version) = {
        let Ok(connection) = rusqlite::Connection::open_with_flags(
            record_path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        ) else {
            return Ok(unread(false, &cache));
        };
        match crate::record::schema::version_in(&connection) {
            Ok(version) if version < 52 => return Ok(TheLaunchMeets::Nothing),
            Ok(_) => {}
            Err(_) => return Ok(unread(false, &cache)),
        }
        let Ok(phase) = update_trust::the_linux_rollback_phase(&connection) else {
            return Ok(unread(false, &cache));
        };
        if phase == 0 {
            return Ok(TheLaunchMeets::Nothing);
        }
        let Ok(Some(probe)) = update_trust::the_update_in_probe(&connection) else {
            return Ok(unread(true, &cache));
        };
        (phase, probe.version)
    };
    let the_box = |opens_failed| TheRollbackBox {
        record_path: record_path.to_path_buf(),
        cache_root: cache.root.clone(),
        row: row.map(str::to_string),
        program: program.clone(),
        failed_version: failed_version.clone(),
        opens_failed,
        asks_the_system: phase == 1,
        trying: std::sync::atomic::AtomicBool::new(false),
    };
    if phase != 2 {
        return Ok(TheLaunchMeets::TheBox(the_box(false)));
    }
    // **The reinstall finished at an earlier launch**: nothing is asked of the
    // system, so the rest is finished at once, and only a failure is shown.
    let the_box = the_box(true);
    match recover_with(record_path, cache, row, carried, connect, program) {
        Ok(true) => Ok(TheLaunchMeets::TheRollbackFinished),
        Ok(false) => Ok(TheLaunchMeets::Nothing),
        Err(_) => Ok(TheLaunchMeets::TheBox(the_box)),
    }
}

impl TheRollbackBox {
    /// ***Reinstall* was pressed**: the system is asked, with the user there,
    /// and the rest of step 7 follows. `Ok` where the rollback finished, the
    /// previous build having been started; an error leaves everything owed,
    /// and the box says *Reinstall failed.* `None` where a reinstall is
    /// already running.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    fn the_reinstall_is_tried_with<P: ThePackageSystem>(
        &self,
        carried: &super::trust::TheCarriedSet,
        connect: &dyn Fn() -> Result<P, String>,
    ) -> Option<Result<(), String>> {
        use std::sync::atomic::Ordering;
        if self.trying.swap(true, Ordering::SeqCst) {
            return None;
        }
        let tried = recover_with(
            &self.record_path,
            Cache { root: self.cache_root.clone() },
            self.row.as_deref(),
            carried,
            connect,
            self.program.clone(),
        )
        .and_then(|finished| {
            // Owed no longer, by whatever route: the installed program is
            // started as a finished rollback would start it.
            if finished {
                Ok(())
            } else {
                the_installed_program_is_started(&self.program)
            }
        });
        self.trying.store(false, Ordering::SeqCst);
        Some(tried)
    }

    /// **The press, in the running box**: off the interface thread, since the
    /// system's request waits on the user. A finished rollback ends this
    /// launch; anything else posts the box again with *Reinstall failed.*
    pub fn reinstall_is_pressed<R: tauri::Runtime>(
        self: &Arc<Self>,
        handle: &tauri::AppHandle<R>,
    ) {
        let the_box = self.clone();
        let handle = handle.clone();
        let _ = std::thread::Builder::new()
            .name("smoozy linux rollback".to_string())
            .spawn(move || {
                #[cfg(target_os = "linux")]
                let tried = the_box.the_reinstall_is_tried_with(
                    &super::trust::TheCarriedSet::this_build(),
                    &system::ThePackageKit::connect,
                );
                #[cfg(not(target_os = "linux"))]
                let tried: Option<Result<(), String>> = Some(Err("Linux alone".into()));
                match tried {
                    None => {}
                    Some(Ok(())) => std::process::exit(0),
                    Some(Err(_)) => the_box.is_shown(&handle, true),
                }
            });
    }

    /// The box posted on the one queue, opening as told.
    pub fn is_shown<R: tauri::Runtime>(&self, handle: &tauri::AppHandle<R>, failed: bool) {
        use tauri::Manager;
        if let Some(queue) = handle.try_state::<Arc<crate::events::TheOneQueue>>() {
            self.is_posted_on(&queue, failed);
        }
    }

    /// The same, onto a queue in hand.
    pub fn is_posted_on(&self, queue: &crate::events::TheOneQueue, failed: bool) {
        let _ = queue.read_then_post(
            || Ok::<_, ()>(()),
            |_| {
                crate::surface::menus::the_rollback_box(
                    &self.failed_version,
                    self.asks_the_system,
                    failed,
                )
            },
        );
    }
}

/// **Step 8 in the running application** — `src/main.rs`'s one call, on
/// Linux, after the instance hold and before the shell's ordinary start.
/// **Where the record, the cache or this program cannot be found, the launch
/// shows the unreadable box** (2026-11-01): never nothing, and never the
/// ordinary launch.
#[cfg(target_os = "linux")]
pub fn what_this_launch_meets() -> TheLaunchMeets {
    let cache = Cache::here();
    let unread = |cache: Option<&Cache>| {
        TheLaunchMeets::TheRecordCouldNotBeRead(TheUnreadableRecordBox {
            the_update_was_underway: cache.is_some_and(|cache| cache.previous().exists()),
        })
    };
    let Ok(cache) = cache else {
        return unread(None);
    };
    let (Ok(record_path), Ok(program)) = (
        crate::platform::locations::resolve(crate::platform::locations::Location::LocalRecord),
        std::env::current_exe(),
    ) else {
        return unread(Some(&cache));
    };
    let underway = cache.previous().exists();
    what_the_launch_meets_with(
        &record_path,
        cache,
        checking::this_copys_row(),
        &super::trust::TheCarriedSet::this_build(),
        &system::ThePackageKit::connect,
        program,
    )
    .unwrap_or(TheLaunchMeets::TheRecordCouldNotBeRead(TheUnreadableRecordBox {
        the_update_was_underway: underway,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// In the running application
// ════════════════════════════════════════════════════════════════════════════

/// **The route, kept for the run** on the check's one sequence: the version
/// last found, and whether the box on screen was opened past Z31's hold.
#[derive(Default)]
pub struct LinuxRoute {
    offered: Option<verifying::AVerifiedUpdate>,
    override_hold: bool,
}

impl LinuxRoute {
    /// **A version was found, or asked for again.** `user_asked` is About's
    /// *Install…* and Z32's press; a check the user asked for, or joined, is
    /// read from `check` (Z29). `override_hold` is Z32's. The reading About is
    /// to show, where it changes.
    pub fn found<R: tauri::Runtime>(
        &mut self,
        handle: &tauri::AppHandle<R>,
        update: Option<&verifying::AVerifiedUpdate>,
        check: &checking::TheUpdateCheck,
        user_asked: bool,
        override_hold: bool,
    ) -> Option<TheReading> {
        use tauri::Manager;
        if !cfg!(target_os = "linux") || check.has_linux_offer() {
            return None;
        }
        let update = update?;
        self.offered = Some(update.clone());
        let version = update.manifest.version.clone();
        let user_asked =
            user_asked || override_hold || check.the_completed_check_was_user_asked();
        let declined = if user_asked {
            None
        } else {
            let started = handle.try_state::<Mutex<crate::start::Started>>()?;
            let started = started.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            match started.record.as_ref().map(|r| update_trust::the_declined_version(r.connection())) {
                Some(Ok(declined)) => declined,
                _ => return Some(TheReading::CouldNotInstall),
            }
        };
        let held = || !override_hold && super::the_mac::publication_is_held_here(handle);
        match what_the_finding_comes_to(
            &version,
            checking::THIS_BUILDS_VERSION,
            declined.as_deref(),
            user_asked,
            &held,
        ) {
            WhatTheFindingComesTo::Quiet => {
                return Some(TheReading::ANewerVersion { version, held: false })
            }
            WhatTheFindingComesTo::Held => {
                return Some(TheReading::ANewerVersion { version, held: true })
            }
            WhatTheFindingComesTo::TheBox => {}
        }
        if check.offer_linux_version(&version) {
            self.override_hold = override_hold;
            if let Some(queue) = handle.try_state::<Arc<crate::events::TheOneQueue>>() {
                let _ = queue.read_then_post(
                    || Ok::<_, ()>(()),
                    |_| crate::surface::menus::the_linux_offer(&version),
                );
            }
        }
        None
    }

    /// **Z32's *Install now…***: the box, past the hold, without the mark.
    pub fn held<R: tauri::Runtime>(
        &mut self,
        handle: &tauri::AppHandle<R>,
        check: &checking::TheUpdateCheck,
    ) -> Option<TheReading> {
        let offered = self.offered.clone();
        self.found(handle, offered.as_ref(), check, true, true)
    }

    /// **About's *Install…***: the box for the version found, without the
    /// mark, and held again if something started running since.
    pub fn asked<R: tauri::Runtime>(
        &mut self,
        handle: &tauri::AppHandle<R>,
        check: &checking::TheUpdateCheck,
    ) -> Option<TheReading> {
        let offered = self.offered.clone();
        self.found(handle, offered.as_ref(), check, true, false)
    }

    /// **The box was answered.** The check has already refused an answer for
    /// any version but the one on screen.
    pub fn answered<R: tauri::Runtime>(
        &mut self,
        handle: &tauri::AppHandle<R>,
        check: &checking::TheUpdateCheck,
        install: bool,
    ) -> Option<TheReading> {
        let _ = check;
        let update = self.offered.clone()?;
        #[cfg(target_os = "linux")]
        {
            return self.answered_with(handle, update, install, &system::ThePackageKit::connect);
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (handle, update, install);
            None
        }
    }

    /// The answer, over any package system: the real one on Linux.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    fn answered_with<R: tauri::Runtime, P: ThePackageSystem>(
        &mut self,
        handle: &tauri::AppHandle<R>,
        update: verifying::AVerifiedUpdate,
        install: bool,
        connect: &dyn Fn() -> Result<P, String>,
    ) -> Option<TheReading> {
        use tauri::Manager;
        let version = update.manifest.version.clone();
        let started = handle.try_state::<Mutex<crate::start::Started>>()?;
        // **Z6: the record's one lock, kept through publication and the
        // probe**, so no ordinary work is admitted once the install begins.
        let started = started.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let record = started.record.as_ref()?;
        if !install {
            return dismissed(record.connection(), &version)
                .err()
                .map(|_| TheReading::CouldNotInstall);
        }
        let Some(transfers) = handle.try_state::<Arc<crate::downloading::TheRunsTransfers>>() else {
            return Some(TheReading::CouldNotInstall);
        };
        // **Z31 read again**: work may have started while the box was open.
        if !self.override_hold
            && super::the_hold::publication_is_held(record.connection(), transfers.inner().as_ref())
        {
            return Some(TheReading::ANewerVersion { version, held: true });
        }
        let window = handle.get_webview_window("main");
        let result = (|| -> Result<Outcome, String> {
            // Never erase a cache another update still owes a rollback from.
            if update_trust::the_update_in_probe(record.connection())
                .map_err(|e| format!("{e:?}"))?
                .is_some()
            {
                return Err("an update still owes recovery".into());
            }
            let row = checking::this_copys_row().ok_or("unlisted Linux system")?;
            let cache = Cache::here()?;
            let carried = super::trust::TheCarriedSet::this_build();
            let remembered = verifying::the_kept_record_as_it_stands(record.connection(), &carried)
                .map_err(|e| format!("{e:?}"))?;
            let packages = prepare(
                &cache,
                &checking::OverHttps,
                transfers.inner().as_ref(),
                &carried,
                remembered.as_ref(),
                &update,
                checking::THIS_BUILDS_VERSION,
                row,
            )?;
            let system = connect()?;
            system.names(packages.next.path(), &version, row)?;
            system.names(packages.previous.path(), checking::THIS_BUILDS_VERSION, row)?;
            // Read before publication: afterwards the path holds the new build.
            let program = std::env::current_exe().map_err(|e| e.to_string())?;
            before_publication(record, &packages)?;
            if let Some(window) = window.as_ref() {
                let _ = window.hide();
            }
            let mut steps = Steps {
                system: &system,
                packages: &packages,
                record_path: record.path().to_path_buf(),
                cache: &cache,
                hold: super::the_mac::TheShellsHold { handle },
                program,
                owns_hold: true,
            };
            let outcome = transact(record, &version, &mut steps);
            // **Nothing was published**: the cache and the record go, as an
            // update that did not happen leaves nothing retained (Z10a).
            if matches!(outcome, Outcome::Declined | Outcome::NotPublished | Outcome::Failed) {
                let probe = update_trust::the_update_in_probe(record.connection())
                    .map_err(|e| format!("{e:?}"))?
                    .ok_or("no probe")?;
                cleared(record, &cache, &probe)?;
            }
            Ok(outcome)
        })();
        if the_mark_is_written_after(&result) {
            let _ = dismissed(record.connection(), &version);
        }
        match result {
            // **The old process's run ends here, writing nothing on the way
            // out** (`Reporting a problem` U3 step 3). Where a rollback is still
            // owed, every retained file and the phase stay, and [`transact`]
            // has started the later launch that shows the box (D6b).
            Ok(Outcome::Installed | Outcome::RolledBack | Outcome::RecoveryOwed) => {
                std::process::exit(0)
            }
            Ok(Outcome::Declined) => {
                if let Some(window) = window.as_ref() {
                    let _ = window.show();
                }
                None
            }
            // **A failed install is not a decline and writes no mark** (D6b,
            // the runner's ruling): About says it failed, and the automatic
            // route may offer the version again.
            Ok(Outcome::NotPublished | Outcome::Failed) | Err(_) => {
                if let Some(window) = window.as_ref() {
                    let _ = window.show();
                }
                Some(TheReading::CouldNotInstall)
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The real package system — Linux alone, and documentation alone (Z12)
// ════════════════════════════════════════════════════════════════════════════

#[cfg(target_os = "linux")]
mod system {
    use super::*;
    use futures_util::StreamExt;

    /// **PackageKit's own transaction interface** (1.2.8's publication, the
    /// same in 1.3.1 and 1.3.4), on the system bus. **The connection is made
    /// once and kept across both installs**, and only the forward call lets
    /// the system ask for authentication. An absent or different service
    /// refuses the update.
    pub struct ThePackageKit {
        runtime: tokio::runtime::Runtime,
        bus: zbus::Connection,
    }

    impl ThePackageKit {
        pub fn connect() -> Result<Self, String> {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| e.to_string())?;
            let bus = runtime
                .block_on(zbus::Connection::system())
                .map_err(|e| e.to_string())?;
            Ok(Self { runtime, bus })
        }
    }

    impl ThePackageSystem for ThePackageKit {
        fn install(&self, path: &Path, previous: bool, may_ask: bool) -> Result<(), InstallFailure> {
            let path = path
                .to_str()
                .ok_or_else(|| InstallFailure::Failed("the package path is not text".into()))?
                .to_string();
            self.runtime.block_on(async {
                let manager = zbus::Proxy::new(
                    &self.bus,
                    "org.freedesktop.PackageKit",
                    "/org/freedesktop/PackageKit",
                    "org.freedesktop.PackageKit",
                )
                .await
                .map_err(bus_error)?;
                let transaction: zbus::zvariant::OwnedObjectPath =
                    manager.call("CreateTransaction", &()).await.map_err(bus_error)?;
                let transaction = zbus::Proxy::new(
                    &self.bus,
                    "org.freedesktop.PackageKit",
                    transaction,
                    "org.freedesktop.PackageKit.Transaction",
                )
                .await
                .map_err(bus_error)?;
                let hints: Vec<&str> =
                    vec![if may_ask { "interactive=true" } else { "interactive=false" }];
                let _: () = transaction.call("SetHints", &(hints,)).await.map_err(bus_error)?;
                // Subscribed before the call: a refusal can finish at once.
                let mut signals = transaction.receive_all_signals().await.map_err(bus_error)?;
                // A bitfield of `PkTransactionFlagEnum` (`pk-bitfield.h`:
                // 1 << value): ALLOW_REINSTALL is 4 and ALLOW_DOWNGRADE 6.
                // ONLY_TRUSTED is not set: the system does not know Smoozy's
                // signing keys, and Smoozy verified the file itself.
                let flags: u64 = if previous { (1 << 4) | (1 << 6) } else { 0 };
                let paths: Vec<String> = vec![path];
                let _: () = transaction
                    .call("InstallFiles", &(flags, paths))
                    .await
                    .map_err(bus_error)?;
                let mut failure = None;
                while let Some(message) = signals.next().await {
                    let header = message.header();
                    match header.member().map(|m| m.as_str()) {
                        Some("ErrorCode") => {
                            let (code, details): (u32, String) = message
                                .body()
                                .deserialize()
                                .map_err(|e| InstallFailure::Failed(e.to_string()))?;
                            failure = Some(package_error(code, details));
                        }
                        Some("Finished") => {
                            let (exit, _runtime): (u32, u32) = message
                                .body()
                                .deserialize()
                                .map_err(|e| InstallFailure::Failed(e.to_string()))?;
                            return package_ending(exit, failure);
                        }
                        _ => {}
                    }
                }
                Err(InstallFailure::Failed("the package transaction disconnected".into()))
            })
        }

        /// `dpkg-deb --field` reads one control field at a time; the row field
        /// is the one the release writes into each Linux package.
        fn names(&self, package: &Path, version: &str, row: &str) -> Result<(), String> {
            for (field, expected) in [
                ("Package", "smoozy"),
                ("Version", version),
                ("Architecture", "amd64"),
                ("X-Smoozy-Release-Matrix-Row", row.strip_suffix("-amd64").unwrap_or(row)),
            ] {
                let answer = std::process::Command::new("/usr/bin/dpkg-deb")
                    .arg("--field")
                    .arg(package)
                    .arg(field)
                    .output()
                    .map_err(|e| e.to_string())?;
                if !answer.status.success()
                    || String::from_utf8_lossy(&answer.stdout).trim() != expected
                {
                    return Err(format!("package field {field} differs"));
                }
            }
            Ok(())
        }

        /// `dpkg-query --show` with its own format: the status word
        /// (`installed` is *correctly unpacked and configured*, `dpkg(1)`) and
        /// the version, for the one package.
        fn has_installed(&self, version: &str) -> bool {
            std::process::Command::new("/usr/bin/dpkg-query")
                .arg("--show")
                .arg("--showformat=${db:Status-Status} ${Version}")
                .arg("smoozy")
                .output()
                .is_ok_and(|answer| {
                    answer.status.success()
                        && String::from_utf8_lossy(&answer.stdout).trim()
                            == format!("installed {version}")
                })
        }
    }

    /// A refusal by the bus or by polkit before the transaction ran is a
    /// declined authentication too.
    fn bus_error(error: zbus::Error) -> InstallFailure {
        match &error {
            zbus::Error::MethodError(name, _, _)
                if [
                    "org.freedesktop.PackageKit.NotAuthorized",
                    "org.freedesktop.PolicyKit1.Error.NotAuthorized",
                    "org.freedesktop.DBus.Error.AccessDenied",
                ]
                .contains(&name.as_str()) =>
            {
                InstallFailure::Authorization
            }
            _ => InstallFailure::Failed(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(name: &str) -> (std::path::PathBuf, LocalRecord) {
        let folder = std::env::temp_dir().join(format!("smoozy-d6-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let record = LocalRecord::open_at(&folder.join("record.sqlite3")).unwrap();
        (folder, record)
    }

    #[test]
    fn d6_the_offer_is_once_per_version_and_only_when_nothing_runs() {
        assert!(should_offer("0.3.0", "0.1.8", None, false, false));
        assert!(!should_offer("0.3.0", "0.1.8", Some("0.3.0"), false, false));
        assert!(!should_offer("0.2.0", "0.1.8", Some("0.3.0"), false, false));
        assert!(should_offer("0.4.0", "0.1.8", Some("0.3.0"), false, false));
        assert!(!should_offer("0.3.0", "0.1.8", None, false, true));
        assert!(!should_offer("0.1.8", "0.1.8", None, true, false));
        assert!(!should_offer("bad", "0.1.8", None, true, false));
    }

    /// **A declined version is quiet on the automatic route even while
    /// something runs**, so About never says it will install by itself; the
    /// hold is read only where the box would be offered.
    #[test]
    fn d6_a_finding_is_quiet_held_or_the_box() {
        use WhatTheFindingComesTo::*;
        let asked = std::cell::Cell::new(0);
        let running = || {
            asked.set(asked.get() + 1);
            true
        };
        let nothing_runs = || false;
        assert_eq!(what_the_finding_comes_to("0.3.0", "0.1.8", Some("0.3.0"), false, &running), Quiet);
        assert_eq!(asked.get(), 0, "the hold was read for a quiet version");
        assert_eq!(what_the_finding_comes_to("0.3.0", "0.1.8", Some("0.3.0"), true, &running), Held);
        assert_eq!(what_the_finding_comes_to("0.3.0", "0.1.8", Some("0.3.0"), true, &nothing_runs), TheBox);
        assert_eq!(what_the_finding_comes_to("0.4.0", "0.1.8", Some("0.3.0"), false, &nothing_runs), TheBox);
        assert_eq!(what_the_finding_comes_to("0.4.0", "0.1.8", None, false, &running), Held);
    }

    #[test]
    fn d6_a_user_asked_check_ignores_the_declined_mark() {
        assert!(should_offer("0.3.0", "0.1.8", Some("0.9.0"), true, false));
        assert!(!should_offer("0.3.0", "0.1.8", Some("0.9.0"), true, true));
    }

    #[test]
    fn d6_dismissal_is_durable_and_keeps_the_check_moment() {
        let (folder, record) = record("dismissal");
        update_trust::a_check_completed(record.connection(), std::time::UNIX_EPOCH).unwrap();
        dismissed(record.connection(), "0.3.0").unwrap();
        drop(record);
        let record = LocalRecord::open_at(&folder.join("record.sqlite3")).unwrap();
        assert_eq!(
            update_trust::the_declined_version(record.connection())
                .unwrap()
                .as_deref(),
            Some("0.3.0")
        );
        assert_eq!(
            update_trust::the_last_completed_check(record.connection()).unwrap(),
            Some(std::time::UNIX_EPOCH)
        );
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    struct Steps {
        log: Vec<&'static str>,
        forward: Result<(), InstallFailure>,
        rollback: Result<(), InstallFailure>,
        starts: bool,
        untouched: bool,
    }
    impl PackageSteps for Steps {
        fn nothing_was_published(&mut self) -> bool {
            self.log.push("ask the package database");
            self.untouched
        }
        fn install(&mut self, previous: bool, may_ask: bool) -> Result<(), InstallFailure> {
            self.log.push(match (previous, may_ask) {
                (false, true) => "install with system authentication",
                (true, false) => "reinstall without interaction",
                (true, true) => "reinstall at the next launch",
                _ => panic!("an unexpected password request"),
            });
            if previous {
                self.rollback.clone()
            } else {
                self.forward.clone()
            }
        }
        fn probe(&mut self) -> bool {
            self.log.push("probe");
            self.starts
        }
        fn restore_record(&mut self) -> Result<(), String> {
            self.log.push("restore record");
            Ok(())
        }
        fn restart_previous(&mut self) -> Result<(), String> {
            self.log.push("restart previous in update mode");
            Ok(())
        }
        fn the_next_launch_is_started(&mut self) -> Result<(), String> {
            self.log.push("start the installed program for the later launch");
            Ok(())
        }
    }
    fn steps(starts: bool) -> Steps {
        Steps {
            log: vec![],
            forward: Ok(()),
            rollback: Ok(()),
            starts,
            untouched: false,
        }
    }

    #[test]
    fn d6_a_successful_start_never_reinstalls() {
        let (folder, record) = record("success");
        let mut steps = steps(true);
        assert_eq!(transact(&record, "0.3.0", &mut steps), Outcome::Installed);
        assert_eq!(steps.log, ["install with system authentication", "probe"]);
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn d6_a_failed_start_reinstalls_before_restoring_and_restarting() {
        let (folder, record) = record("rollback");
        let mut steps = steps(false);
        assert_eq!(transact(&record, "0.3.0", &mut steps), Outcome::RolledBack);
        assert_eq!(
            steps.log,
            [
                "install with system authentication",
                "probe",
                "reinstall without interaction",
                "restore record",
                "restart previous in update mode"
            ]
        );
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn d6_declined_authentication_is_a_decline_not_a_failure() {
        let (folder, record) = record("authentication");
        let mut steps = steps(false);
        steps.forward = Err(InstallFailure::Authorization);
        assert_eq!(transact(&record, "0.3.0", &mut steps), Outcome::Declined);
        assert_eq!(steps.log, ["install with system authentication"]);
        assert_eq!(
            update_trust::the_declined_version(record.connection())
                .unwrap()
                .as_deref(),
            Some("0.3.0")
        );
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// **D6b: Smoozy does not go quiet when the rollback cannot finish.** No
    /// second password request, and the later launch — which meets the phase
    /// and shows the box — is started at once rather than left to the user.
    #[test]
    fn d6_an_expired_authorization_leaves_recovery_owed_without_another_prompt() {
        let (folder, record) = record("expired");
        let mut steps = steps(false);
        steps.rollback = Err(InstallFailure::Authorization);
        assert_eq!(
            transact(&record, "0.3.0", &mut steps),
            Outcome::RecoveryOwed
        );
        assert_eq!(
            steps.log,
            [
                "install with system authentication",
                "probe",
                "reinstall without interaction",
                "start the installed program for the later launch"
            ]
        );
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// **D6b: only a dismissal writes the declined mark.** A declined system
    /// request is one ([`transact`] writes it, and the answer writes it again
    /// only where that failed); a failed install, a transaction that left the
    /// running version, and a failure before the transaction are not.
    #[test]
    fn d6b_a_failed_install_writes_no_declined_mark() {
        assert!(!the_mark_is_written_after(&Ok(Outcome::NotPublished)));
        assert!(!the_mark_is_written_after(&Err("the package could not be fetched".into())));
        assert!(the_mark_is_written_after(&Ok(Outcome::Failed)));
        for ends in [Outcome::Installed, Outcome::RolledBack, Outcome::RecoveryOwed, Outcome::Declined] {
            assert!(!the_mark_is_written_after(&Ok(ends)));
        }
        // And the transaction itself writes none for a failed install.
        let (folder, record) = record("failed-install-no-mark");
        let mut steps = steps(true);
        steps.forward = Err(InstallFailure::Failed("dependencies".into()));
        steps.untouched = true;
        assert_eq!(transact(&record, "0.3.0", &mut steps), Outcome::NotPublished);
        assert_eq!(update_trust::the_declined_version(record.connection()).unwrap(), None);
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }
    #[test]
    fn d6_the_rollback_debt_survives_reopen_and_clears_with_the_probe() {
        let (folder, record) = record("debt");
        let probe = update_trust::TheUpdateInProbe {
            release_sequence: 9,
            version: "0.3.0".into(),
            the_previous_artifact: b"previous.deb".to_vec(),
            the_record_copy: b"copy".to_vec(),
        };
        update_trust::an_update_goes_into_probe(record.connection(), &probe).unwrap();
        assert!(!update_trust::a_linux_rollback_is_owed(record.connection()).unwrap());
        let mut steps = steps(false);
        steps.rollback = Err(InstallFailure::Authorization);
        assert_eq!(
            transact(&record, "0.3.0", &mut steps),
            Outcome::RecoveryOwed
        );
        drop(record);
        let record = LocalRecord::open_at(&folder.join("record.sqlite3")).unwrap();
        assert!(update_trust::a_linux_rollback_is_owed(record.connection()).unwrap());
        assert_eq!(
            update_trust::the_update_in_probe(record.connection()).unwrap(),
            Some(probe)
        );
        update_trust::the_probe_is_cleared(record.connection()).unwrap();
        assert!(!update_trust::a_linux_rollback_is_owed(record.connection()).unwrap());
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// Real signatures under the existing public test seeds, never a real key.
    fn signed_file(
        version: &str,
        sequence: u64,
        content: &[u8],
    ) -> (
        super::super::trust::TheCarriedSet,
        Vec<u8>,
        verifying::AVerifiedUpdate,
    ) {
        use super::super::{
            manifest::{UpdateManifest, THE_DEBIAN_13_ROW, THE_MANIFEST_FORMAT},
            trust::{tests::*, WhichStatement},
            verifying::{ASignedStatement, TheUpdateFile},
        };
        let carried = a_build_with_both_recovery_keys();
        let statement = a_statement(
            1,
            WhichStatement::Transition,
            vec![an_online_key("online-1", &online_1())],
        );
        let (bytes, its_recovery_signature) = signed(&recovery_a(), &statement);
        let manifest = UpdateManifest {
            manifest_format: THE_MANIFEST_FORMAT.into(),
            version: version.into(),
            release_sequence: sequence,
            signing_keys: vec!["online-1".into()],
            entries: vec![AnEntry {
                row: THE_DEBIAN_13_ROW.into(),
                file_name: format!("Smoozy_{version}_debian-13_amd64.deb"),
                byte_length: content.len() as u64,
                digest: smoozy_server_protocol::transfers::integrity::Digest::of(content).bytes(),
            }],
        };
        let manifest = manifest.canonical_bytes();
        let signature = smoozy_server_protocol::release::a_signature_over(
            &online_1().secret,
            super::super::THE_MANIFEST_SIGNING_CONTEXT,
            &manifest,
        );
        let file = TheUpdateFile {
            statements: vec![ASignedStatement {
                bytes,
                its_recovery_signature,
            }],
            manifest,
            manifest_signatures: vec![signature],
        }
        .canonical_bytes();
        let verified =
            verifying::verify_an_update_file_against(&carried, None, &file, THE_DEBIAN_13_ROW)
                .unwrap();
        (carried, file, verified)
    }

    struct Hosting {
        proof: Vec<u8>,
        previous: Vec<u8>,
        next: Vec<u8>,
        asked: Mutex<Vec<String>>,
    }
    impl checking::ReadsTheUpdateFile for Hosting {
        fn read(
            &self,
            from: &str,
            _says: &checking::WhatTheRequestSays,
            _most: u64,
            draw: &mut dyn smoozy_server_protocol::transfers::moving::DrawsFromTheSharedBudget,
        ) -> Result<Vec<u8>, String> {
            self.asked.lock().unwrap().push(from.into());
            let bytes = if from.ends_with("desktop-update") {
                &self.proof
            } else if from.contains("/latest/") {
                &self.next
            } else {
                &self.previous
            };
            checking::read_drawing_from_the_budget(
                &mut bytes.as_slice(),
                bytes.len() as u64 + 1,
                draw,
            )
        }
    }
    fn staging(
        name: &str,
    ) -> (
        std::path::PathBuf,
        LocalRecord,
        Cache,
        super::super::trust::TheCarriedSet,
        Hosting,
        verifying::AVerifiedUpdate,
    ) {
        let (folder, record) = record(name);
        let (carried, proof, _) = signed_file("0.2.0", 8, b"previous package");
        let (_, _, next) = signed_file("0.3.0", 9, b"next package");
        let cache = Cache {
            root: folder.join("linux-update"),
        };
        let hosting = Hosting {
            proof,
            previous: b"previous package".to_vec(),
            next: b"next package".to_vec(),
            asked: Mutex::new(vec![]),
        };
        (folder, record, cache, carried, hosting, next)
    }
    fn stage(
        cache: &Cache,
        carried: &super::super::trust::TheCarriedSet,
        hosting: &Hosting,
        update: &verifying::AVerifiedUpdate,
    ) -> Result<VerifiedPackages, String> {
        prepare(
            cache,
            hosting,
            &crate::downloading::TheRunsTransfers::for_this_run(),
            carried,
            None,
            update,
            "0.2.0",
            super::super::manifest::THE_DEBIAN_13_ROW,
        )
    }

    // The Linux cache reads Unix paths; Windows uses its own update route.
    #[cfg(unix)]
    #[test]
    fn d6_the_previous_package_is_verified_and_cached_before_publication() {
        let (folder, record, cache, carried, hosting, update) = staging("staging");
        let packages = stage(&cache, &carried, &hosting, &update).unwrap();
        assert_eq!(
            std::fs::read(packages.previous.path()).unwrap(),
            b"previous package"
        );
        let asked = hosting.asked.lock().unwrap();
        assert!(asked[0].ends_with("/releases/download/v0.2.0/desktop-update"));
        assert!(asked[1].contains("/releases/download/v0.2.0/"));
        assert!(asked[2].contains("/releases/latest/download/"));
        before_publication(&record, &packages).unwrap();
        let probe = update_trust::the_update_in_probe(record.connection())
            .unwrap()
            .unwrap();
        assert_eq!(probe.version, "0.3.0");
        assert_eq!(
            update_trust::read_the_trust_record(record.connection())
                .unwrap()
                .unwrap()
                .greatest_accepted_release_sequence,
            9
        );
        let (previous, _) = retained_package(
            &cache,
            record.connection(),
            &probe,
            &carried,
            super::super::manifest::THE_DEBIAN_13_ROW,
        )
        .unwrap();
        assert_eq!(previous, packages.previous.path());
        cleared(&record, &cache, &probe).unwrap();
        assert!(!cache.root.exists());
        assert!(update_trust::the_update_in_probe(record.connection())
            .unwrap()
            .is_none());
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn d6_no_wrong_signature_package_row_version_size_or_digest_reaches_publication() {
        for fault in 0..7 {
            let (folder, record, cache, carried, mut hosting, mut update) =
                staging(&format!("refusal-{fault}"));
            match fault {
                0 => { let last = hosting.proof.len() - 1; hosting.proof[last] ^= 1; },
                1 => hosting.previous.push(0),
                2 => hosting.next[0] ^= 1,
                3 => {
                    update.the_entry_for_this_row.as_mut().unwrap().row =
                        super::super::manifest::THE_MAC_ROW.into()
                }
                4 => update.manifest.version = "0.1.0".into(),
                5 => {
                    hosting.proof = signed_file("0.1.0", 7, b"previous package").1;
                }
                _ => {
                    update.the_entry_for_this_row.as_mut().unwrap().file_name =
                        "../other.deb".into()
                }
            }
            assert!(
                stage(&cache, &carried, &hosting, &update).is_err(),
                "fault {fault}"
            );
            assert!(update_trust::the_update_in_probe(record.connection())
                .unwrap()
                .is_none());
            drop(record);
            std::fs::remove_dir_all(folder).unwrap();
        }
    }

    #[test]
    fn d6_a_changed_cached_package_and_a_record_naming_another_path_are_refused() {
        let (folder, record, cache, carried, hosting, update) = staging("changed-cache");
        let packages = stage(&cache, &carried, &hosting, &update).unwrap();
        before_publication(&record, &packages).unwrap();
        let mut probe = update_trust::the_update_in_probe(record.connection())
            .unwrap()
            .unwrap();
        probe.the_previous_artifact = b"/unrelated.deb".to_vec();
        assert!(retained_package(
            &cache,
            record.connection(),
            &probe,
            &carried,
            super::super::manifest::THE_DEBIAN_13_ROW
        )
        .is_err());
        probe.the_previous_artifact = the_probe::path_bytes(packages.previous.path());
        write(packages.previous.path(), b"changed").unwrap();
        assert!(retained_package(
            &cache,
            record.connection(),
            &probe,
            &carried,
            super::super::manifest::THE_DEBIAN_13_ROW
        )
        .is_err());
        assert!(before_publication(&record, &packages).is_err());
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn d6_a_user_press_before_or_during_the_automatic_check_reaches_the_same_offer() {
        use checking::{TheCheckEnded, TheUpdateCheck, TheUpdatesAct, WhatTheCheckFound};
        for before in [true, false] {
            let check = TheUpdateCheck::new(false, std::time::Duration::from_secs(3600));
            if before {
                check.the_user_asks();
            }
            let mut offered = false;
            check.the_sequence_runs_on(
                &mut || {
                    if !before {
                        assert_eq!(
                            check.the_user_asks(),
                            checking::ThePress::JoinsTheCheckUnderWay
                        );
                    }
                    TheCheckEnded::Completed(WhatTheCheckFound::ANewerVersion {
                        version: "0.3.0".into(),
                    })
                },
                &mut |_, _, _| {},
                &|| false,
                &mut |act| {
                    assert_eq!(act, TheUpdatesAct::ANewerVersionWasFound);
                    assert!(check.the_completed_check_was_user_asked());
                    offered = true;
                    check.the_run_ends();
                    None
                },
            );
            assert!(offered);
        }
    }

    #[test]
    fn d6_a_stale_or_double_answer_is_refused_and_quitting_dismisses_the_open_box() {
        let (folder, record) = record("stale-offer");
        let check = checking::TheUpdateCheck::new(false, std::time::Duration::from_secs(3600));
        assert!(check.offer_linux_version("0.3.0"));
        assert!(!check.offer_linux_version("0.4.0"));
        assert!(!check.answer_linux_offer("0.4.0", true));
        assert!(check.answer_linux_offer("0.3.0", false));
        assert!(!check.answer_linux_offer("0.3.0", true));
        check.dismiss_linux_on_quit(record.connection()).unwrap();
        assert_eq!(
            update_trust::the_declined_version(record.connection())
                .unwrap()
                .as_deref(),
            Some("0.3.0")
        );
        assert!(!check.has_linux_offer());
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn d6_next_launch_recovery_reinstalls_before_reading_the_old_record() {
        let mut steps = steps(false);
        finish_recovery(&mut steps, false).unwrap();
        assert_eq!(
            steps.log,
            [
                "reinstall at the next launch",
                "restore record",
                "restart previous in update mode"
            ]
        );
        let mut refused = super::tests::steps(false);
        refused.rollback = Err(InstallFailure::Authorization);
        assert!(finish_recovery(&mut refused, false).is_err());
        assert_eq!(refused.log, ["reinstall at the next launch"]);
    }

    /// A failure that changed the package is published as far as the rollback
    /// is concerned: no probe, the retained package back, the record restored.
    #[test]
    fn d6_a_failed_transaction_that_changed_the_package_is_rolled_back_without_a_probe() {
        let (folder, record) = record("package-failure");
        let mut steps = steps(true);
        steps.forward = Err(InstallFailure::Failed(
            "package transaction interrupted".into(),
        ));
        assert_eq!(transact(&record, "0.3.0", &mut steps), Outcome::RolledBack);
        assert_eq!(
            steps.log,
            [
                "install with system authentication",
                "ask the package database",
                "reinstall without interaction",
                "restore record",
                "restart previous in update mode"
            ]
        );
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// **A refused transaction that left the running version installed is an
    /// update that did not happen** — the dependency check refusing an
    /// unlisted system is the ordinary one (Z10). No rollback, no restart.
    #[test]
    fn d6_a_failed_transaction_that_left_the_running_version_is_not_published() {
        let (folder, record) = record("not-published");
        let mut steps = steps(true);
        steps.forward = Err(InstallFailure::Failed("dependencies".into()));
        steps.untouched = true;
        assert_eq!(transact(&record, "0.3.0", &mut steps), Outcome::NotPublished);
        assert_eq!(
            steps.log,
            ["install with system authentication", "ask the package database"]
        );
        assert!(!update_trust::a_linux_rollback_is_owed(record.connection()).unwrap());
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn d6_version_52_preserves_an_old_probe_and_defaults_to_no_rollback_debt() {
        let (folder, record) = record("migration");
        let path = record.path().to_path_buf();
        drop(record);
        std::fs::remove_file(&path).unwrap();
        let connection = rusqlite::Connection::open(&path).unwrap();
        for migration in crate::record::migrations::ALL
            .iter()
            .filter(|m| m.produces <= 51)
        {
            crate::record::migrations::run_one_for_tests(&connection, migration).unwrap();
        }
        let probe = update_trust::TheUpdateInProbe {
            release_sequence: 9,
            version: "0.3.0".into(),
            the_previous_artifact: b"previous".to_vec(),
            the_record_copy: b"copy".to_vec(),
        };
        update_trust::an_update_goes_into_probe(&connection, &probe).unwrap();
        update_trust::this_version_was_declined(&connection, "0.2.0").unwrap();
        drop(connection);
        let record = LocalRecord::open_at(&path).unwrap();
        assert_eq!(record.schema_version().unwrap(), crate::record::schema::BINARY_KNOWS);
        assert_eq!(
            update_trust::the_update_in_probe(record.connection()).unwrap(),
            Some(probe)
        );
        assert!(!update_trust::a_linux_rollback_is_owed(record.connection()).unwrap());
        assert_eq!(
            update_trust::the_declined_version(record.connection())
                .unwrap()
                .as_deref(),
            Some("0.2.0")
        );
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    // The Linux cache reads Unix paths; Windows uses its own update route.
    #[cfg(unix)]
    #[test]
    fn d6_a_declined_install_does_not_prevent_retaining_the_running_package_later() {
        let (folder, record, cache, carried, hosting, update) = staging("floor");
        let packages = stage(&cache, &carried, &hosting, &update).unwrap();
        before_publication(&record, &packages).unwrap();
        let probe = update_trust::the_update_in_probe(record.connection())
            .unwrap()
            .unwrap();
        cleared(&record, &cache, &probe).unwrap();
        let remembered = verifying::the_kept_record_as_it_stands(record.connection(), &carried)
            .unwrap()
            .unwrap();
        assert_eq!(remembered.greatest_accepted_release_sequence, 9);
        assert!(prepare(
            &cache,
            &hosting,
            &crate::downloading::TheRunsTransfers::for_this_run(),
            &carried,
            Some(&remembered),
            &update,
            "0.2.0",
            super::super::manifest::THE_DEBIAN_13_ROW
        )
        .is_ok());
        assert_eq!(
            update_trust::read_the_trust_record(record.connection())
                .unwrap()
                .unwrap()
                .greatest_accepted_release_sequence,
            9
        );
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }
    #[test]
    fn d6_cancelled_package_work_is_not_a_declined_password_request() {
        assert_eq!(package_error(48, "authentication refused".into()), InstallFailure::Authorization);
        assert_eq!(package_error(17, "transaction cancelled".into()), InstallFailure::Failed("transaction cancelled".into()));
    }

    #[test]
    fn d6_a_cancelled_ending_cannot_discard_the_recovery_package() {
        assert!(matches!(package_ending(3, None), Err(InstallFailure::Failed(_))));
        assert_eq!(package_ending(2, Some(InstallFailure::Authorization)), Err(InstallFailure::Authorization));
        assert!(matches!(package_ending(1, Some(InstallFailure::Failed("unpack failed".into()))), Err(InstallFailure::Failed(_))));
        assert_eq!(package_ending(1, None), Ok(()));
    }

    #[test]
    fn d6_an_interrupted_cleanup_needs_no_package_or_password_request() {
        let mut steps = steps(false);
        steps.rollback = Err(InstallFailure::Authorization);
        finish_recovery(&mut steps, true).unwrap();
        assert_eq!(steps.log, ["restore record", "restart previous in update mode"]);
    }

    // The Linux cache reads Unix paths; Windows uses its own update route.
    #[cfg(unix)]
    #[test]
    fn d6_the_completed_reinstall_phase_survives_after_the_cache_is_removed() {
        let (folder, record, cache, carried, hosting, update) = staging("cleanup-phase");
        let packages = stage(&cache, &carried, &hosting, &update).unwrap();
        before_publication(&record, &packages).unwrap();
        let probe = update_trust::the_update_in_probe(record.connection()).unwrap().unwrap();
        update_trust::a_linux_rollback_is_required(record.connection()).unwrap();
        update_trust::the_linux_reinstall_finished(record.connection()).unwrap();
        assert_eq!(update_trust::the_linux_rollback_phase(record.connection()).unwrap(), 2);
        files::a_folder_is_removed_whole(&cache.root).unwrap();
        let path = record.path().to_path_buf(); drop(record);
        let record = LocalRecord::open_at(&path).unwrap();
        assert_eq!(update_trust::the_linux_rollback_phase(record.connection()).unwrap(), 2);
        assert!(update_trust::a_linux_rollback_is_owed(record.connection()).unwrap());
        cleared(&record, &cache, &probe).unwrap();
        assert_eq!(update_trust::the_linux_rollback_phase(record.connection()).unwrap(), 0);
        drop(record); std::fs::remove_dir_all(folder).unwrap();
    }

    // ── Finished at the desk by the second builder ─────────────────────────

    /// **The test form of the package system**: it says what it was asked,
    /// and answers as told. Nothing here reaches a package tool.
    #[derive(Clone)]
    struct ASystem {
        asked: Arc<Mutex<Vec<(PathBuf, bool, bool)>>>,
        forward: Result<(), InstallFailure>,
        previous: Result<(), InstallFailure>,
        installed: Option<String>,
    }
    impl ASystem {
        fn answering(forward: Result<(), InstallFailure>, previous: Result<(), InstallFailure>) -> ASystem {
            ASystem { asked: Arc::new(Mutex::new(vec![])), forward, previous, installed: None }
        }
        fn asked(&self) -> Vec<(PathBuf, bool, bool)> {
            self.asked.lock().unwrap().clone()
        }
    }
    impl ThePackageSystem for ASystem {
        fn install(&self, package: &Path, previous: bool, may_ask: bool) -> Result<(), InstallFailure> {
            self.asked.lock().unwrap().push((package.to_path_buf(), previous, may_ask));
            if previous { self.previous.clone() } else { self.forward.clone() }
        }
        fn names(&self, _: &Path, _: &str, _: &str) -> Result<(), String> {
            Ok(())
        }
        fn has_installed(&self, version: &str) -> bool {
            self.installed.as_deref() == Some(version)
        }
    }
    struct AHold;
    impl HandsTheHold for AHold {
        fn to_the_probe(&mut self) {}
        fn back(&mut self) -> bool {
            true
        }
    }
    /// What the build a rollback starts is, in these tests: the system's own
    /// do-nothing program, so no Smoozy is ever started.
    fn a_harmless_program() -> PathBuf {
        PathBuf::from("/usr/bin/true")
    }
    fn raw(path: &Path) -> rusqlite::Connection {
        rusqlite::Connection::open(path).unwrap()
    }

    /// **L8's copy is put back without migrating it.** At a later launch the
    /// build restoring it is the new one; migrating the copy would hand the
    /// previous build a record it cannot read. Here the copy reads version 51
    /// while carrying version 52's column, so a migrating opening fails.
    #[test]
    fn d6_the_restored_record_keeps_its_own_version_and_carries_the_phase() {
        let (folder, record) = record("restored-as-it-stands");
        let path = record.path().to_path_buf();
        drop(record);
        std::fs::remove_file(&path).unwrap();
        let connection = raw(&path);
        for migration in crate::record::migrations::ALL.iter().filter(|m| m.produces <= 51) {
            crate::record::migrations::run_one_for_tests(&connection, migration).unwrap();
        }
        connection.execute_batch(crate::record::schema::VERSION_52[0]).unwrap();
        let probe = update_trust::TheUpdateInProbe {
            release_sequence: 9,
            version: "0.3.0".into(),
            the_previous_artifact: b"previous".to_vec(),
            the_record_copy: the_probe::path_bytes(&crate::record::copy_beside(&path)),
        };
        update_trust::an_update_goes_into_probe(&connection, &probe).unwrap();
        drop(connection);
        std::fs::copy(&path, crate::record::copy_beside(&path)).unwrap();
        // The probe's own write, after the copy: the restore undoes it.
        dismissed(&raw(&path), "9.9.9").unwrap();

        the_record_is_restored_owing_cleanup(&path, &probe).unwrap();
        let restored = raw(&path);
        assert_eq!(crate::record::schema::version_in(&restored).unwrap(), 51);
        assert_eq!(update_trust::the_linux_rollback_phase(&restored).unwrap(), 2);
        assert_eq!(update_trust::the_declined_version(&restored).unwrap(), None);
        assert!(!crate::record::copy_beside(&path).exists());
        drop(restored);
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// The shared clearing hands a record naming this cache to the Linux
    /// clearing, and leaves any other record to itself.
    // The Linux cache reads Unix paths; Windows uses its own update route.
    #[cfg(unix)]
    #[test]
    fn d6_only_a_record_naming_the_cache_is_cleared_as_linuxs() {
        let (folder, record, cache, carried, hosting, update) = staging("names-the-cache");
        let packages = stage(&cache, &carried, &hosting, &update).unwrap();
        before_publication(&record, &packages).unwrap();
        let probe = update_trust::the_update_in_probe(record.connection()).unwrap().unwrap();
        let mut other = probe.clone();
        other.the_previous_artifact = b"/Applications/.Smoozy previous.app".to_vec();
        assert!(cleared_if_it_names(record.connection(), record.path(), &cache, &other).is_none());
        assert!(cache.root.exists());
        assert_eq!(
            cleared_if_it_names(record.connection(), record.path(), &cache, &probe),
            Some(Ok(()))
        );
        assert!(!cache.root.exists());
        assert!(update_trust::the_update_in_probe(record.connection()).unwrap().is_none());
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// A staged, published update whose new build migrated the record (L8's
    /// copy taken, then a write), with the rollback phase written.
    fn an_update_owing_its_rollback(
        name: &str,
        phase: u8,
    ) -> (PathBuf, PathBuf, Cache, super::super::trust::TheCarriedSet, PathBuf) {
        let (folder, record, cache, carried, hosting, update) = staging(name);
        let packages = stage(&cache, &carried, &hosting, &update).unwrap();
        before_publication(&record, &packages).unwrap();
        crate::record::take_the_copy(record.path()).unwrap();
        dismissed(record.connection(), "9.9.9").unwrap();
        update_trust::a_linux_rollback_is_required(record.connection()).unwrap();
        if phase == 2 {
            update_trust::the_linux_reinstall_finished(record.connection()).unwrap();
        }
        let path = record.path().to_path_buf();
        let previous = packages.previous.path().to_path_buf();
        drop(record);
        (folder, path, cache, carried, previous)
    }

    /// **Z10a's fallback, whole, at a later launch**: one request for the
    /// retained package, with the system allowed to ask; L8's copy back; the
    /// record cleared; the cache gone.
    // The Linux cache reads Unix paths; Windows uses its own update route.
    #[cfg(unix)]
    #[test]
    fn d6_a_later_launch_reinstalls_restores_clears_and_starts_the_previous_build() {
        let (folder, path, cache, carried, previous) = an_update_owing_its_rollback("later-launch", 1);
        let root = cache.root.clone();
        let system = ASystem::answering(Ok(()), Ok(()));
        let made = system.clone();
        let finished = recover_with(
            &path,
            cache,
            Some(super::super::manifest::THE_DEBIAN_13_ROW),
            &carried,
            &|| Ok(made.clone()),
            a_harmless_program(),
        );
        assert_eq!(finished, Ok(true));
        assert_eq!(system.asked(), [(previous, true, true)]);
        let record = raw(&path);
        assert!(update_trust::the_update_in_probe(&record).unwrap().is_none());
        assert_eq!(update_trust::the_declined_version(&record).unwrap(), None, "L8's copy was not restored");
        assert!(!root.exists());
        assert!(!crate::record::copy_beside(&path).exists());
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// **A refused request at that launch keeps everything owed**: the phase,
    /// the cache and L8's copy stay, and no ordinary work is admitted.
    #[test]
    fn d6_a_refused_request_at_a_later_launch_keeps_the_rollback_owed() {
        let (folder, path, cache, carried, _) = an_update_owing_its_rollback("later-refused", 1);
        let root = cache.root.clone();
        let system = ASystem::answering(Ok(()), Err(InstallFailure::Authorization));
        let made = system.clone();
        assert!(recover_with(
            &path,
            cache,
            Some(super::super::manifest::THE_DEBIAN_13_ROW),
            &carried,
            &|| Ok(made.clone()),
            a_harmless_program(),
        )
        .is_err());
        let record = raw(&path);
        assert_eq!(update_trust::the_linux_rollback_phase(&record).unwrap(), 1);
        assert!(root.exists());
        assert!(crate::record::copy_beside(&path).exists());
        drop(record);
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// **A launch after the reinstall finished asks the system for nothing**,
    /// even with the cache already gone, and finishes the rest.
    // The Linux cache reads Unix paths; Windows uses its own update route.
    #[cfg(unix)]
    #[test]
    fn d6_a_later_launch_after_the_reinstall_asks_for_no_package() {
        let (folder, path, cache, carried, _) = an_update_owing_its_rollback("later-phase-2", 2);
        files::a_folder_is_removed_whole(&cache.root).unwrap();
        let system = ASystem::answering(Ok(()), Ok(()));
        let made = system.clone();
        assert_eq!(
            recover_with(&path, cache, None, &carried, &|| Ok(made.clone()), a_harmless_program()),
            Ok(true)
        );
        assert!(system.asked().is_empty());
        assert!(update_trust::the_update_in_probe(&raw(&path)).unwrap().is_none());
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// **D6b: a later launch owing the reinstall shows the box and asks the
    /// system nothing until *Reinstall* is pressed.** Until D6b it asked at
    /// once with nothing on screen, and a declined request ended the launch
    /// with nothing on screen either. A declined press keeps everything owed
    /// and the box says so; a second press while one runs is not a second
    /// request; an accepted one finishes the rollback.
    // The Linux cache reads Unix paths; Windows uses its own update route.
    #[cfg(unix)]
    #[test]
    fn d6b_a_later_launch_shows_the_box_and_asks_nothing_until_pressed() {
        let (folder, path, cache, carried, previous) = an_update_owing_its_rollback("d6b-box", 1);
        let root = cache.root.clone();
        let declining = ASystem::answering(Ok(()), Err(InstallFailure::Authorization));
        let made = declining.clone();
        let met = what_the_launch_meets_with(
            &path,
            cache,
            Some(super::super::manifest::THE_DEBIAN_13_ROW),
            &carried,
            &|| Ok(made.clone()),
            a_harmless_program(),
        )
        .unwrap();
        let TheLaunchMeets::TheBox(the_box) = met else {
            panic!("a launch owing the reinstall did not show the box");
        };
        assert!(declining.asked().is_empty(), "the system was asked before the box was answered");
        assert_eq!(the_box.failed_version, "0.3.0");
        assert!(the_box.asks_the_system && !the_box.opens_failed);
        assert_eq!(update_trust::the_linux_rollback_phase(&raw(&path)).unwrap(), 1);

        let made = declining.clone();
        assert!(matches!(the_box.the_reinstall_is_tried_with(&carried, &|| Ok(made.clone())), Some(Err(_))));
        assert_eq!(declining.asked(), [(previous.clone(), true, true)]);
        assert_eq!(update_trust::the_linux_rollback_phase(&raw(&path)).unwrap(), 1);
        assert!(root.exists() && crate::record::copy_beside(&path).exists());

        the_box.trying.store(true, std::sync::atomic::Ordering::SeqCst);
        let made = declining.clone();
        assert!(the_box.the_reinstall_is_tried_with(&carried, &|| Ok(made.clone())).is_none());
        assert_eq!(declining.asked().len(), 1, "a press while one ran asked again");
        the_box.trying.store(false, std::sync::atomic::Ordering::SeqCst);

        let accepting = ASystem::answering(Ok(()), Ok(()));
        let made = accepting.clone();
        assert_eq!(the_box.the_reinstall_is_tried_with(&carried, &|| Ok(made.clone())), Some(Ok(())));
        assert_eq!(accepting.asked(), [(previous, true, true)]);
        assert!(update_trust::the_update_in_probe(&raw(&path)).unwrap().is_none());
        assert!(!root.exists());
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// **D6b: a launch owing only the restore and cleanup finishes them with no
    /// box and no request; where they cannot finish, the box says so** and
    /// says nothing about a password, since none will be asked for.
    // The Linux cache reads Unix paths; Windows uses its own update route.
    #[cfg(unix)]
    #[test]
    fn d6b_a_later_launch_owing_the_cleanup_finishes_it_or_shows_the_failure() {
        let (folder, path, cache, carried, _) = an_update_owing_its_rollback("d6b-cleanup", 2);
        let system = ASystem::answering(Ok(()), Ok(()));
        let made = system.clone();
        let elsewhere = Cache { root: folder.join("another-cache") };
        let met = what_the_launch_meets_with(&path, elsewhere, None, &carried, &|| Ok(made.clone()), a_harmless_program()).unwrap();
        let TheLaunchMeets::TheBox(the_box) = met else {
            panic!("a cleanup that could not finish showed nothing");
        };
        assert!(the_box.opens_failed && !the_box.asks_the_system);
        assert_eq!(update_trust::the_linux_rollback_phase(&raw(&path)).unwrap(), 2);
        let made = system.clone();
        let met = what_the_launch_meets_with(&path, cache, None, &carried, &|| Ok(made.clone()), a_harmless_program()).unwrap();
        assert!(matches!(met, TheLaunchMeets::TheRollbackFinished));
        assert!(system.asked().is_empty());
        assert!(update_trust::the_update_in_probe(&raw(&path)).unwrap().is_none());
        let made = system.clone();
        let met = what_the_launch_meets_with(&path, Cache { root: folder.join("linux-update") }, None, &carried, &|| Ok(made.clone()), a_harmless_program()).unwrap();
        assert!(matches!(met, TheLaunchMeets::Nothing), "a finished rollback still showed something");
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// **A record that cannot be read for the phase shows a box with *Quit*
    /// alone, never the ordinary launch** (2026-11-01, the D6b line under
    /// Z11). Until then the launch ended with nothing on screen. The system is
    /// asked nothing and nothing is removed; the box names the update where
    /// the cache still holds the previous package, and not where it holds
    /// nothing.
    // The Linux cache reads Unix paths; Windows uses its own update route.
    #[cfg(unix)]
    #[test]
    fn a_launch_whose_record_cannot_be_read_shows_a_box_and_not_the_ordinary_launch() {
        let (folder, path, cache, carried, previous) = an_update_owing_its_rollback("unreadable", 1);
        let root = cache.root.clone();
        for beside in ["-wal", "-shm"] {
            let mut name = path.clone().into_os_string();
            name.push(beside);
            let _ = std::fs::remove_file(name);
        }
        std::fs::write(&path, b"this is not a record").unwrap();
        let system = ASystem::answering(Ok(()), Ok(()));
        for (cache, underway) in [(cache, true), (Cache { root: folder.join("no-cache") }, false)] {
            let made = system.clone();
            let met = what_the_launch_meets_with(
                &path,
                cache,
                Some(super::super::manifest::THE_DEBIAN_13_ROW),
                &carried,
                &|| Ok(made.clone()),
                a_harmless_program(),
            );
            let Ok(TheLaunchMeets::TheRecordCouldNotBeRead(the_box)) = met else {
                panic!("a record that could not be read did not show the box");
            };
            assert_eq!(the_box.the_update_was_underway, underway);
        }
        assert!(system.asked().is_empty(), "the system was asked with nothing to reinstall from");
        assert!(root.exists() && previous.exists());
        assert_eq!(std::fs::read(&path).unwrap(), b"this is not a record");
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// A record with no phase, or none at all, owes this route nothing.
    #[test]
    fn d6_a_launch_with_no_phase_owes_nothing() {
        let (folder, record, cache, carried, hosting, update) = staging("no-phase");
        let packages = stage(&cache, &carried, &hosting, &update).unwrap();
        before_publication(&record, &packages).unwrap();
        let path = record.path().to_path_buf();
        drop(record);
        let system = ASystem::answering(Ok(()), Ok(()));
        let made = system.clone();
        let root = cache.root.clone();
        assert_eq!(
            recover_with(&path, cache, None, &carried, &|| Ok(made.clone()), a_harmless_program()),
            Ok(false)
        );
        assert!(system.asked().is_empty());
        assert!(root.exists());
        let nowhere = folder.join("no-record.sqlite3");
        assert_eq!(
            recover_with(&nowhere, Cache { root: root.clone() }, None, &carried, &|| Ok(made.clone()), a_harmless_program()),
            Ok(false)
        );
        assert!(!nowhere.exists());
        std::fs::remove_dir_all(folder).unwrap();
    }

    /// **The real acts of the forward transaction, at the desk**, with the
    /// package system's test form: a failure that changed the package rolls
    /// back with no probe, asking nothing the second time; one that left the
    /// running version installed publishes nothing.
    // The Linux cache reads Unix paths; Windows uses its own update route.
    #[cfg(unix)]
    #[test]
    fn d6_the_forward_transactions_acts_reinstall_restore_and_clear_in_order() {
        for untouched in [false, true] {
            let (folder, record, cache, carried, hosting, update) =
                staging(&format!("forward-acts-{untouched}"));
            let packages = stage(&cache, &carried, &hosting, &update).unwrap();
            before_publication(&record, &packages).unwrap();
            let mut system =
                ASystem::answering(Err(InstallFailure::Failed("unpacking".into())), Ok(()));
            if untouched {
                system.installed = Some(checking::THIS_BUILDS_VERSION.into());
            }
            let mut steps = super::Steps {
                system: &system,
                packages: &packages,
                record_path: record.path().to_path_buf(),
                cache: &cache,
                hold: AHold,
                program: a_harmless_program(),
                owns_hold: true,
            };
            let outcome = transact(&record, "0.3.0", &mut steps);
            let next = packages.next.path().to_path_buf();
            let previous = packages.previous.path().to_path_buf();
            if untouched {
                assert_eq!(outcome, Outcome::NotPublished);
                assert_eq!(system.asked(), [(next, false, true)]);
                assert!(cache.root.exists(), "the caller clears what was not published");
            } else {
                assert_eq!(outcome, Outcome::RolledBack);
                assert_eq!(system.asked(), [(next, false, true), (previous, true, false)]);
                assert!(!cache.root.exists());
                assert!(update_trust::the_update_in_probe(&raw(record.path())).unwrap().is_none());
            }
            drop(record);
            std::fs::remove_dir_all(folder).unwrap();
        }
    }

    /// **About's *Install…* opens the box for the version already found, and
    /// starts no check** (Z26's three triggers). It is taken only from the
    /// unheld newer-version reading.
    #[test]
    fn d6_abouts_install_reaches_the_box_with_no_second_check() {
        use checking::{TheCheckEnded, TheUpdateCheck, TheUpdatesAct, WhatTheCheckFound};
        let check = TheUpdateCheck::new(false, std::time::Duration::from_secs(3600));
        assert!(!check.the_user_opens_the_linux_box(), "nothing was found yet");
        let mut checks = 0;
        let mut pressed = None;
        let mut acts = vec![];
        check.the_sequence_runs_on(
            &mut || {
                checks += 1;
                TheCheckEnded::Completed(WhatTheCheckFound::ANewerVersion { version: "0.3.0".into() })
            },
            &mut |reading, _, ended| {
                if ended.is_none()
                    && pressed.is_none()
                    && matches!(reading, TheReading::ANewerVersion { held: false, .. })
                {
                    pressed = Some(check.the_user_opens_the_linux_box());
                }
            },
            &|| false,
            &mut |act| {
                acts.push(act);
                match act {
                    TheUpdatesAct::ANewerVersionWasFound if acts.len() == 1 => {
                        Some(TheReading::ANewerVersion { version: "0.3.0".into(), held: false })
                    }
                    _ => {
                        check.the_run_ends();
                        None
                    }
                }
            },
        );
        assert_eq!(pressed, Some(true));
        assert_eq!(checks, 1, "the press started a check");
        assert_eq!(acts, [TheUpdatesAct::ANewerVersionWasFound, TheUpdatesAct::TheLinuxBoxWasAsked]);

        let held = TheUpdateCheck::new(false, std::time::Duration::from_secs(3600));
        held.the_sequence_runs_on(
            &mut || TheCheckEnded::Completed(WhatTheCheckFound::ANewerVersion { version: "0.3.0".into() }),
            &mut |_, _, _| {},
            &|| true,
            &mut |_| {
                held.the_run_ends();
                None
            },
        );
        assert!(!held.the_user_opens_the_linux_box(), "the held form has Install now, not Install");
        let open = TheUpdateCheck::new(false, std::time::Duration::from_secs(3600));
        assert!(open.offer_linux_version("0.3.0"));
        assert!(!open.the_user_opens_the_linux_box(), "a box is already open");
    }
}

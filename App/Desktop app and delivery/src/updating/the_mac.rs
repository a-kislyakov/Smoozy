//! **The Mac update route: stage, verify, hold, publish, probe, roll back** —
//! E16's macOS half, E18, Z5 to Z9, Z21's macOS bullet, Z21a, Z24, Z25, Z31
//! and Z32, with L8 and L13 used. Built at step 11's Mac update piece. **No
//! route skips the check**: publication takes an [`AVerifiedStagedBundle`],
//! and the one function that makes one runs the fetch's length and digest and
//! then the system's own check over the complete staged bundle.
//!
//! **The order, and each step is a gate on the next.**
//!
//! 1. **Fetched** ([`super::fetching`]) into the staging folder beside the
//!    installed bundle (E18), held to the signed manifest's length and digest
//!    (E16).
//! 2. **Unpacked** by the system's own `ditto` into a folder of its own, and
//!    exactly one bundle must be in it.
//! 3. **The system's own check over the complete staged bundle** (E16:
//!    *"Smoozy's manifest signature and the operating system's signature and
//!    notarization checks over the complete bundle"*) — [`ChecksTheBundle`], an
//!    adapter whose real form is [`TheSystemsOwnCheck`] and whose test form
//!    exists only in tests. **On a Mac whose Gatekeeper assessments are on, an
//!    ad-hoc bundle cannot pass it**, and no signed, notarized release exists
//!    yet, so nothing a user would get from this route has been watched (see
//!    the account of this piece).
//! 4. **The update box** ([`super::the_box`], Z33, from 2026-11-09): the
//!    staged, verified bundle is kept for the run, and **nothing goes on
//!    until the user presses *Install now***. *Skip this version* writes the declined
//!    mark (Z10b). Until then this step was Z31's hold, and publication went
//!    on by itself once nothing ran.
//! 5. **Published** ([`the_verified_bundle_is_published`]): the window put
//!    away and the record's one lock taken and kept, so no new ordinary work is
//!    admitted (Z6); L8's copy from an earlier migration discarded, since the
//!    running build has long since started successfully (L13); the accepted
//!    trust record kept, which is E15's *"stores the highest accepted sequence
//!    in the local record before publication"*; Z21a's update-in-probe record
//!    written; and **the complete bundle replaced at its installed path with a
//!    named backup of the complete previous one** (Z5, Z7, Z8, Z24), through
//!    the platform file adapter.
//! 6. **The probe** ([`super::the_probe`]): the new bundle started **through
//!    the system's own app launcher** (from 2026-11-09,
//!    [`ThroughTheSystemsLauncher`]) in the update-launch mode with a one-use
//!    token, the hold handed over (P2), and the token watched for until the
//!    launch deadline.
//! 7. **On the token, the old process exits.** **On anything else, the
//!    rollback** (Z21's macOS bullet): the probe ended, the hold taken back,
//!    **L8's copy restored first** (L13), the complete named backup put back at
//!    the installed path, the record cleared as the rollback's last act (Z21a),
//!    and **the previous bundle started in the update-launch mode alone, so it
//!    makes no crash offer** (Z21b's last bullet, `Reporting a problem` U5b),
//!    through the launcher too. Then the old process exits.
//!
//! **The old process continues only for the handover** (Z6): from step 5 on
//! it writes nothing but Z21a's record and the rollback's acts, and it never
//! writes inside either bundle (Z5, Z25).
//!
//! **What a publication that does not happen is.** E15's *an update that did
//! not happen*: nothing is replaced, the probe record is cleared, E18's
//! staging folder is removed, the window comes back, and About reads *Update
//! installation failed.* — Design’s separate reading for a found update that
//! could not be installed. The next check tries again.
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
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::checking::{AdmitsTheUpdateFetch, ReadsTheUpdateFile, TheReading, TheStepThatFailed, WhatTheRequestSays};
use super::fetching::{self, WhyTheFetchDidNotPass};
use super::the_probe::{self, AProbeUnderWatch, WhatTheProbeCameTo};
use super::trust::TheUpdateTrustRecord;
use super::verifying::{self, AVerifiedUpdate};
use crate::platform::files;
use crate::platform::locations::{self, Location};
use crate::record::{self, update_trust, LocalRecord};

// ════════════════════════════════════════════════════════════════════════════
// The three places
// ════════════════════════════════════════════════════════════════════════════

/// **The installed bundle and the two places beside it** — F11's, resolved
/// through `crate::platform::locations` and composed nowhere else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePlaces {
    /// Z8's installed path.
    pub installed: PathBuf,
    /// E18's staging folder, on the same filesystem.
    pub staging: PathBuf,
    /// Z7's named backup of the complete previous bundle.
    pub kept: PathBuf,
}

impl ThePlaces {
    /// **This copy's own**, or why there are none — a program not running from
    /// a bundle has nothing to replace.
    pub fn for_this_copy() -> Result<ThePlaces, String> {
        let installed = locations::resolve(Location::TheInstalledBundle).map_err(|not| not.reported)?;
        Ok(ThePlaces::beside(&installed))
    }

    /// **The places beside a named bundle** — the same composition, so a test
    /// can ask it of a throwaway one.
    pub fn beside(installed: &Path) -> ThePlaces {
        ThePlaces {
            installed: installed.to_path_buf(),
            staging: locations::beside_the_installed_bundle(Location::TheUpdatesStagingFolder, installed),
            kept: locations::beside_the_installed_bundle(Location::ThePreviousBundleKept, installed),
        }
    }

    /// The backup's name, as the replacement call takes it.
    fn the_kept_name(&self) -> String {
        self.kept
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default()
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The system's own check — an adapter
// ════════════════════════════════════════════════════════════════════════════

/// **The system's own verification over a complete staged bundle** (E16).
/// **Its real form is [`TheSystemsOwnCheck`]; its test form exists only in
/// tests**, and nothing outside them can be handed one.
pub trait ChecksTheBundle {
    /// Accepted, or the system's own words for why not.
    fn accepts(&self, bundle: &Path) -> Result<(), String>;
}

/// **The system's checks plus E18's same Developer ID team.**
/// `codesign(1)` display supplies each team; display alone is not verification,
/// so `--verify --deep --strict -R` also requires Apple's Developer ID
/// Application chain and that team's certificate OU (Apple TN3127).
/// `spctl --assess --type exec` still must pass. The team check holds even
/// where the user has switched Gatekeeper assessments off.
/// Sources: `Audits/Build/Step 11/Sources/D5b, the same team and two words — what landed.sources.md`.
pub struct TheSystemsOwnCheck;

impl ChecksTheBundle for TheSystemsOwnCheck {
    fn accepts(&self, bundle: &Path) -> Result<(), String> {
        if !cfg!(target_os = "macos") {
            return Err("the system's own bundle check is a Mac's".into());
        }
        let running = std::env::current_exe().map_err(|failed| failed.to_string())?;
        the_systems_check(bundle, &running, &TheCodeSigningTools)
    }
}

/// The system's answers, with a stand-in only in tests.
trait AnswersCodeSigning {
    fn run(&self, program: &str, arguments: &[&str], code: &Path) -> Result<String, String>;
}

struct TheCodeSigningTools;

impl AnswersCodeSigning for TheCodeSigningTools {
    fn run(&self, program: &str, arguments: &[&str], code: &Path) -> Result<String, String> {
        let ran = Command::new(program)
            .args(arguments)
            .arg(code)
            .stdin(std::process::Stdio::null())
            .output()
            .map_err(|failed| format!("{program}: {failed}"))?;
        if !ran.status.success() {
            return Err(format!("{program} ended {}: {}", ran.status, String::from_utf8_lossy(&ran.stderr).trim()));
        }
        let stdout = String::from_utf8(ran.stdout).map_err(|failed| failed.to_string())?;
        let stderr = String::from_utf8(ran.stderr).map_err(|failed| failed.to_string())?;
        Ok(format!("{stdout}\n{stderr}"))
    }
}

// Apple's Developer ID Application requirement (TN3127); no embedded team.
const THE_DEVELOPER_ID_REQUIREMENT: &str = "anchor apple generic and certificate 1[field.1.2.840.113635.100.6.2.6] exists and certificate leaf[field.1.2.840.113635.100.6.1.13] exists";

fn the_team_in(answer: &str) -> Result<&str, String> {
    let mut teams = answer.lines().filter_map(|line| line.trim().strip_prefix("TeamIdentifier="));
    let team = teams.next().ok_or("the code signature has no team")?;
    if teams.next().is_some() || team.is_empty() || !team.bytes().all(|it| it.is_ascii_alphanumeric()) {
        return Err("the code signature has no unambiguous team".into());
    }
    Ok(team)
}

fn the_systems_check(bundle: &Path, running: &Path, tools: &dyn AnswersCodeSigning) -> Result<(), String> {
    let running_answer = tools.run("/usr/bin/codesign", &["--display", "--verbose=4"], running)?;
    let running_team = the_team_in(&running_answer)?;
    let staged_answer = tools.run("/usr/bin/codesign", &["--display", "--verbose=4"], bundle)?;
    let staged_team = the_team_in(&staged_answer)?;
    if running_team != staged_team {
        return Err("the staged bundle's team differs from the running copy's".into());
    }
    // The parsed value is alphanumeric; it cannot introduce requirement text.
    // Verify both certificates against the team, rather than trusting display
    // text or Gatekeeper's policy as proof of a Developer ID signer.
    let requirement = format!("={THE_DEVELOPER_ID_REQUIREMENT} and certificate leaf[subject.OU] = \"{running_team}\"");
    for code in [running, bundle] {
        tools.run("/usr/bin/codesign", &["--verify", "--deep", "--strict", "-vvv", "-R", &requirement], code)?;
    }
    tools.run("/usr/sbin/spctl", &["-vvv", "--assess", "--type", "exec"], bundle)?;
    Ok(())
}

// ════════════════════════════════════════════════════════════════════════════
// Staged and verified
// ════════════════════════════════════════════════════════════════════════════

/// **Why an update did not happen.** Each is E15's *an update that did not
/// happen*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheUpdateDidNotHappen {
    /// The places beside the installed bundle could not be named.
    NoPlaces(String),
    /// The verified manifest carries nothing for this copy's row.
    NothingForThisRow,
    /// The fetch did not pass (length, digest, network, the disk).
    TheFetch(WhyTheFetchDidNotPass),
    /// The staging folder could not be made ready, or the download unpacked.
    TheStaging(String),
    /// **The download does not hold exactly one bundle.**
    NotOneBundleInTheDownload,
    /// **The system's own check refused the staged bundle** (E16).
    TheSystemsOwnCheckRefusedIt(String),
    /// The record would not take E15's or Z21a's write before publication.
    TheRecordBeforePublication(String),
    /// The replacement itself failed; the installed bundle is what it was.
    TheReplacement(String),
    /// **The staging folder could not be made beside the installed bundle
    /// because that folder will not take one**: its volume is read-only, or
    /// this login may not write there. Nothing was fetched. Added 2026-11-14,
    /// the Rough what-if answers, share 5 (426, and 61's update half): until
    /// then it was [`Self::TheStaging`], which About says with no reason.
    ItsFolderCannotBeWritten { read_only: bool },
}

impl WhyTheUpdateDidNotHappen {
    /// **The step About names** (Design *Updating*; the four questions,
    /// Sol's S5).
    pub fn the_step(&self) -> TheStepThatFailed {
        use WhyTheUpdateDidNotHappen as Why;
        match self {
            Why::TheFetch(failed) => failed.the_step(),
            Why::TheStaging(_) | Why::ItsFolderCannotBeWritten { .. } => TheStepThatFailed::Download,
            Why::NothingForThisRow | Why::NotOneBundleInTheDownload | Why::TheSystemsOwnCheckRefusedIt(_) => {
                TheStepThatFailed::Verification
            }
            Why::NoPlaces(_) | Why::TheRecordBeforePublication(_) | Why::TheReplacement(_) => {
                TheStepThatFailed::Installation
            }
        }
    }
    /// **The reason About says after the step**, where one is known (Design
    /// *Updating*, 2026-11-12): the fetch's own, and the system's check.
    pub fn the_reason(&self) -> Option<super::checking::TheReasonBehind> {
        match self {
            Self::TheFetch(failed) => failed.the_reason(),
            Self::TheSystemsOwnCheckRefusedIt(_) => Some(super::checking::TheReasonBehind::TheSystemRefusedIt),
            Self::ItsFolderCannotBeWritten { read_only: true } => {
                Some(super::checking::TheReasonBehind::ItsFolderIsReadOnly)
            }
            Self::ItsFolderCannotBeWritten { read_only: false } => {
                Some(super::checking::TheReasonBehind::NoPermissionForItsFolder)
            }
            _ => None,
        }
    }
}

/// **Why the staging folder could not be made**, where the folder the
/// installed bundle sits in will not take one: its volume is read-only, or
/// the refusal was a permission refusal. Anything else stays
/// [`WhyTheUpdateDidNotHappen::TheStaging`] with the system's words.
fn the_staging_folder_was_refused(places: &ThePlaces, failed: files::Failure) -> WhyTheUpdateDidNotHappen {
    let beside = places.installed.parent().unwrap_or(&places.installed);
    if files::the_volume_is_read_only(beside) {
        WhyTheUpdateDidNotHappen::ItsFolderCannotBeWritten { read_only: true }
    } else if failed.refusal == files::Refusal::PermissionRefused {
        WhyTheUpdateDidNotHappen::ItsFolderCannotBeWritten { read_only: false }
    } else {
        WhyTheUpdateDidNotHappen::TheStaging(failed.reported)
    }
}

/// **A complete bundle, staged beside the installed one and accepted by the
/// system's own check.** The fields are private and the one function that
/// makes one is [`the_update_is_fetched_and_verified`]: **that is what *no
/// route skips the check* is, by construction.**
#[derive(Debug)]
pub struct AVerifiedStagedBundle {
    bundle: PathBuf,
    version: String,
    release_sequence: u64,
    the_trust_record_to_keep: TheUpdateTrustRecord,
}

impl AVerifiedStagedBundle {
    /// The version it is.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Where it is staged.
    pub fn bundle(&self) -> &Path {
        &self.bundle
    }
}

/// **Steps 1 to 3, whole.** The staging folder is emptied and made afresh,
/// the artifact fetched into it and held to its length and digest, unpacked
/// into a folder of its own, and the one bundle in it offered to the system's
/// own check.
pub fn the_update_is_fetched_and_verified(
    file: &dyn ReadsTheUpdateFile,
    admission: &dyn AdmitsTheUpdateFetch,
    verified: &AVerifiedUpdate,
    says: &WhatTheRequestSays,
    places: &ThePlaces,
    check: &dyn ChecksTheBundle,
) -> Result<AVerifiedStagedBundle, WhyTheUpdateDidNotHappen> {
    use WhyTheUpdateDidNotHappen as Why;
    let entry = verified.the_entry_for_this_row.as_ref().ok_or(Why::NothingForThisRow)?;
    files::a_folder_is_removed_whole(&places.staging).map_err(|failed| the_staging_folder_was_refused(places, failed))?;
    files::ensure_directory(&places.staging).map_err(|failed| the_staging_folder_was_refused(places, failed))?;
    let fetched = fetching::the_artifact_is_fetched(file, admission, entry, says, &places.staging)
        .map_err(Why::TheFetch)?;
    let unpacked = places.staging.join("unpacked");
    files::the_archive_is_unpacked(fetched.path(), &unpacked).map_err(|failed| Why::TheStaging(failed.reported))?;
    let bundles: Vec<PathBuf> = std::fs::read_dir(&unpacked)
        .map_err(|failed| Why::TheStaging(failed.to_string()))?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && path.extension().is_some_and(|it| it == "app"))
        .collect();
    let [bundle] = bundles.as_slice() else {
        return Err(Why::NotOneBundleInTheDownload);
    };
    check.accepts(bundle).map_err(Why::TheSystemsOwnCheckRefusedIt)?;
    Ok(AVerifiedStagedBundle {
        bundle: bundle.clone(),
        version: verified.manifest.version.clone(),
        release_sequence: verified.manifest.release_sequence,
        the_trust_record_to_keep: verified.the_trust_record_to_keep.clone(),
    })
}

// ════════════════════════════════════════════════════════════════════════════
// Published, probed, rolled back
// ════════════════════════════════════════════════════════════════════════════

/// **How a build is started**, for the probe and for the rollback. The real
/// form is [`FromTheBundle`]; the tests start this test program instead.
pub trait StartsABuild {
    /// **The probe**: `bundle` started in the update-launch mode with the
    /// token (Z21, `Reporting a problem` U5a).
    fn the_probe(&self, bundle: &Path, token: &str) -> Result<AProbeUnderWatch, String>;
    /// **The build a rollback starts**: `bundle` started in the update-launch
    /// mode alone, with no token (Z21b's last bullet, and
    /// `Reporting a problem` U5b).
    fn the_previous_build(&self, bundle: &Path) -> Result<(), String>;
}

/// **R5's one mark on a launch the update mechanism starts** — the argument
/// `crate::start::LaunchMode` reads.
pub const THE_UPDATE_LAUNCH_ARGUMENT: &str = crate::start::THE_UPDATE_LAUNCH_ARGUMENT;

/// **The real form, from 2026-11-09: the bundle started through the system's
/// own app launcher** (the Creator's twenty-eighth notes, note 5). Until then
/// the bundle's program was run directly as a child of the old process, and a
/// Smoozy kept in the Dock came back as a second icon after an update; a
/// program the launcher never started is the cause as Part A of those notes
/// reads it, and that reading is not proved (see the account of piece 3).
/// **Z21's macOS bullet holds unchanged**: the new bundle is started in P2's
/// probe mode and watched for the token, now through
/// [`AProbeUnderWatch::launched`] with its two pipes in E18's staging folder;
/// and the build a rollback starts is started the same way, with the
/// argument alone.
pub struct ThroughTheSystemsLauncher;

impl StartsABuild for ThroughTheSystemsLauncher {
    fn the_probe(&self, bundle: &Path, token: &str) -> Result<AProbeUnderWatch, String> {
        let staging = locations::beside_the_installed_bundle(Location::TheUpdatesStagingFolder, bundle);
        files::ensure_directory(&staging).map_err(|failed| failed.reported)?;
        #[cfg(unix)]
        return AProbeUnderWatch::launched(
            bundle,
            &[THE_UPDATE_LAUNCH_ARGUMENT],
            token,
            &staging,
            &the_probe::the_bundle_is_opened,
        );
        #[cfg(not(unix))]
        {
            let _ = token;
            Err("the system's app launcher is a Mac's".into())
        }
    }

    fn the_previous_build(&self, bundle: &Path) -> Result<(), String> {
        // **The argument alone, and no token** (Z21b's last bullet). The
        // launched app is handed this process's own environment as well as
        // the one given (observed at the desk, 2026-11-09), and this process
        // may itself have been a probe, so **each of the probe's variables is
        // handed empty**, which reads as none.
        the_probe::the_bundle_is_opened(bundle, &[THE_UPDATE_LAUNCH_ARGUMENT], &the_probe::NONE_OF_THE_PROBES_VARIABLES)
            .map(drop)
    }
}

/// **P2's hold, as the handover moves it.**
pub trait HandsTheHold {
    /// Handed to the probe; the old process has stopped writing by now.
    fn to_the_probe(&mut self);
    /// Taken back after the probe is ended; whether it was.
    fn back(&mut self) -> bool;
}

/// **What publication came to.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThePublicationCameTo {
    /// **Nothing was replaced**: an update that did not happen, and the old
    /// process carries on.
    NotPublished(WhyTheUpdateDidNotHappen),
    /// **The new build said its token**: the old process exits now.
    TheNewBuildRuns,
    /// **The new build did not start, and the rollback ran.** The old process
    /// exits now. `why` is the start's failure; `left_undone` is any rollback
    /// act that would not go, in the system's words, which a later launch
    /// finishes (Z21a).
    RolledBack { why: String, left_undone: Vec<String> },
}

/// **Steps 5 to 7.** `record` is the old process's own open record, whose one
/// lock the caller holds for the whole of this; nothing else writes it.
/// **A publication that does not happen removes E18's staging folder** (its
/// build detail of 2026-11-09): nothing was published, so nothing in it is
/// owed.
pub fn the_verified_bundle_is_published(
    staged: AVerifiedStagedBundle,
    places: &ThePlaces,
    record: &LocalRecord,
    starts: &dyn StartsABuild,
    hold: &mut dyn HandsTheHold,
    deadline: Duration,
) -> WhatThePublicationCameTo {
    let came = the_publication(staged, places, record, starts, hold, deadline);
    if matches!(came, WhatThePublicationCameTo::NotPublished(_)) {
        the_staging_folder_goes(places);
    }
    came
}

fn the_publication(
    staged: AVerifiedStagedBundle,
    places: &ThePlaces,
    record: &LocalRecord,
    starts: &dyn StartsABuild,
    hold: &mut dyn HandsTheHold,
    deadline: Duration,
) -> WhatThePublicationCameTo {
    use WhatThePublicationCameTo as Came;
    use WhyTheUpdateDidNotHappen as Why;
    let connection = record.connection();
    let record_path = record.path().to_path_buf();
    // **L13's discard, owed since the last migration**: the running build
    // started successfully long ago, and a stale copy left beside the record
    // would be what a rollback restored.
    if let Err(failed) = record::discard_the_copy(&record_path) {
        return Came::NotPublished(Why::TheRecordBeforePublication(format!("{failed:?}")));
    }
    // **E15**: the accepted sequence stored before publication.
    if let Err(failed) = verifying::keep(connection, &staged.the_trust_record_to_keep) {
        return Came::NotPublished(Why::TheRecordBeforePublication(format!("{failed:?}")));
    }
    // **Z21a**: the record written immediately before publication.
    let probe = update_trust::TheUpdateInProbe {
        release_sequence: staged.release_sequence,
        version: staged.version.clone(),
        the_previous_artifact: the_probe::path_bytes(&places.kept),
        the_record_copy: the_probe::path_bytes(&record::copy_beside(&record_path)),
    };
    if let Err(failed) = update_trust::an_update_goes_into_probe(connection, &probe) {
        return Came::NotPublished(Why::TheRecordBeforePublication(format!("{failed:?}")));
    }
    // **Z5, Z7, Z8, Z24**: the complete bundle replaced, the previous kept.
    if let Err(failed) = files::the_bundle_is_replaced(&places.installed, &staged.bundle, Some(&places.the_kept_name())) {
        // **The installed bundle stays what it was.** Where the call left the
        // installed path empty and the backup beside it, the backup goes back.
        if !places.installed.exists() && places.kept.exists() {
            let _ = files::publish(&places.kept, &places.installed);
        }
        let _ = update_trust::the_probe_is_cleared(connection);
        return Came::NotPublished(Why::TheReplacement(failed.reported));
    }

    // **Z21: the probe, and P2's handover.**
    let Some(token) = the_probe::a_token() else {
        return the_rollback(places, &record_path, starts, hold, "no token could be made".into(), false);
    };
    let mut watched = match starts.the_probe(&places.installed, &token) {
        Ok(watched) => watched,
        Err(failed) => return the_rollback(places, &record_path, starts, hold, failed, false),
    };
    hold.to_the_probe();
    match watched.is_watched(deadline) {
        WhatTheProbeCameTo::ItStarted => {
            drop(watched.is_let_go());
            Came::TheNewBuildRuns
        }
        failed => {
            watched.is_ended();
            the_rollback(places, &record_path, starts, hold, format!("{failed:?}"), true)
        }
    }
}

/// **Z21's macOS rollback**, every act tried and any that would not go said.
fn the_rollback(
    places: &ThePlaces,
    record_path: &Path,
    starts: &dyn StartsABuild,
    hold: &mut dyn HandsTheHold,
    why: String,
    the_hold_was_handed_over: bool,
) -> WhatThePublicationCameTo {
    let mut left_undone = Vec::new();
    if the_hold_was_handed_over && !hold.back() {
        left_undone.push("the hold could not be taken back".to_string());
    }
    // **L13: the copy is restored first, before the previous build reads the
    // record.** No copy means the probe ran no migration.
    if record::copy_beside(record_path).exists() {
        if let Err(failed) = record::restore_the_copy(record_path) {
            left_undone.push(format!("L8's copy was not restored: {failed:?}"));
        }
    }
    // **The complete named backup put back at the installed path.**
    if let Err(failed) = files::the_bundle_is_replaced(&places.installed, &places.kept, None) {
        left_undone.push(format!("the previous bundle was not put back: {}", failed.reported));
    }
    // **Z21a: the rollback's last act clears the record**, through a fresh
    // opening of the restored file — the old connection is on the file the
    // restore replaced.
    match LocalRecord::open_at(record_path) {
        Ok(restored) => {
            if let Err(failed) = update_trust::the_probe_is_cleared(restored.connection()) {
                left_undone.push(format!("the probe record was not cleared: {failed:?}"));
            }
        }
        Err(failed) => left_undone.push(format!("the record would not open: {failed:?}")),
    }
    // **The previous bundle started in the update-launch mode alone**
    // (Z21b's last bullet), so it makes no crash offer.
    if let Err(failed) = starts.the_previous_build(&places.installed) {
        left_undone.push(format!("the previous build was not started: {failed}"));
    }
    WhatThePublicationCameTo::RolledBack { why, left_undone }
}

// ════════════════════════════════════════════════════════════════════════════
// In the running application
// ════════════════════════════════════════════════════════════════════════════

/// **The route, kept for the run**: the staged bundle, and the verified update
/// it is staged from. Made by the update check's sequence and used on that one
/// sequence alone. **Z33: nothing here publishes but the box's *Install
/// now*** ([`TheMacRoute::answered`]).
pub struct TheMacRoute {
    staged: Option<AVerifiedStagedBundle>,
    offered: Option<AVerifiedUpdate>,
}

impl Default for TheMacRoute {
    fn default() -> Self {
        TheMacRoute::new()
    }
}

impl TheMacRoute {
    pub fn new() -> TheMacRoute {
        TheMacRoute { staged: None, offered: None }
    }

    /// **A check found a newer version, or About asked for the box** (Z33):
    /// where the box is due, staged and verified (or the one staged for that
    /// version kept), then the box offered. The reading About is to show,
    /// where it changes.
    pub fn found<R: tauri::Runtime>(
        &mut self,
        handle: &tauri::AppHandle<R>,
        verified: Option<&AVerifiedUpdate>,
        check: &super::checking::TheUpdateCheck,
        user_asked: bool,
    ) -> Option<TheReading> {
        if !cfg!(target_os = "macos") {
            return None;
        }
        let verified = verified?;
        self.offered = Some(verified.clone());
        let version = verified.manifest.version.clone();
        if let Err(reading) = super::the_box::the_box_is_due(handle, check, &version, user_asked) {
            return reading;
        }
        if let Err(reading) = self.staged_for(handle, verified) {
            return Some(reading);
        }
        super::the_box::the_box_is_shown(handle, check, &version);
        None
    }

    /// **About's *Install now…***: the box for the version found, without the
    /// mark (Z29's ground).
    pub fn asked<R: tauri::Runtime>(
        &mut self,
        handle: &tauri::AppHandle<R>,
        check: &super::checking::TheUpdateCheck,
    ) -> Option<TheReading> {
        let offered = self.offered.clone();
        self.found(handle, offered.as_ref(), check, true)
    }

    /// **The box was answered.** *Skip this version* writes the mark (Z10b); *Install now*
    /// is Z33's one press, and publishes. The check has already refused an
    /// answer for any version but the one on screen.
    pub fn answered<R: tauri::Runtime>(
        &mut self,
        handle: &tauri::AppHandle<R>,
        check: &super::checking::TheUpdateCheck,
        install: bool,
    ) -> Option<TheReading> {
        let _ = check;
        let verified = self.offered.clone()?;
        if !install {
            return super::the_box::the_dismissal_is_written(handle, &verified.manifest.version);
        }
        if let Err(reading) = self.staged_for(handle, &verified) {
            return Some(reading);
        }
        let places = match ThePlaces::for_this_copy() {
            Ok(places) => places,
            Err(_) => return Some(TheReading::CouldNotInstall(TheStepThatFailed::Installation, None)),
        };
        self.published(handle, places)
    }

    /// **Steps 1 to 3 for `verified`**, or the one already staged for it kept
    /// (Z33: the staged unit is kept for the run).
    fn staged_for<R: tauri::Runtime>(
        &mut self,
        handle: &tauri::AppHandle<R>,
        verified: &AVerifiedUpdate,
    ) -> Result<(), TheReading> {
        let already = self
            .staged
            .as_ref()
            .is_some_and(|staged| staged.version == verified.manifest.version && staged.bundle.exists());
        if already {
            return Ok(());
        }
        self.staged = None;
        let places = ThePlaces::for_this_copy()
            .map_err(|_| TheReading::CouldNotInstall(TheStepThatFailed::Installation, None))?;
        let transfers = handle_state::<R, Arc<crate::downloading::TheRunsTransfers>>(handle)
            .ok_or(TheReading::CouldNotInstall(TheStepThatFailed::Installation, None))?;
        match the_update_is_fetched_and_verified(
            &super::checking::OverHttps,
            transfers.as_ref(),
            verified,
            &WhatTheRequestSays::for_this_copy(),
            &places,
            &TheSystemsOwnCheck,
        ) {
            Ok(staged) => {
                self.staged = Some(staged);
                Ok(())
            }
            Err(why) => {
                // **The leftover folder** (E18's build detail): an update that
                // did not happen leaves no download behind.
                the_staging_folder_goes(&places);
                Err(TheReading::CouldNotInstall(why.the_step(), why.the_reason()))
            }
        }
    }

    fn published<R: tauri::Runtime>(&mut self, handle: &tauri::AppHandle<R>, places: ThePlaces) -> Option<TheReading> {
        use tauri::Manager;
        let staged = self.staged.take()?;
        let window = handle.get_webview_window("main");
        if let Some(window) = window.as_ref() {
            let _ = window.hide();
        }
        let started = handle.try_state::<Mutex<crate::start::Started>>()?;
        // **Z6: the record's one lock, kept until this process exits** — no new
        // ordinary work is admitted from here.
        let started = started.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let came = match started.record.as_ref() {
            Some(record) => {
                let mut hold = TheShellsHold { handle };
                the_verified_bundle_is_published(
                    staged,
                    &places,
                    record,
                    &ThroughTheSystemsLauncher,
                    &mut hold,
                    the_probe::THE_LAUNCH_DEADLINE,
                )
            }
            None => WhatThePublicationCameTo::NotPublished(WhyTheUpdateDidNotHappen::TheRecordBeforePublication(
                "the record is not open".into(),
            )),
        };
        match came {
            WhatThePublicationCameTo::NotPublished(why) => {
                drop(started);
                if let Some(window) = window.as_ref() {
                    let _ = window.show();
                }
                Some(TheReading::CouldNotInstall(why.the_step(), why.the_reason()))
            }
            // **The old process's run ends here, writing nothing on the way
            // out** (`Reporting a problem` U3 step 3): the shell's own exit
            // would write the run-mark, which is the new build's now.
            WhatThePublicationCameTo::TheNewBuildRuns | WhatThePublicationCameTo::RolledBack { .. } => {
                std::process::exit(0)
            }
        }
    }
}

/// **E18's staging folder removed whole** (its build detail of 2026-11-09,
/// the twenty-eighth notes): the leftover `/Applications/.Smoozy update/`.
/// Nothing is said where it will not go; the next start tries again.
pub fn the_staging_folder_goes(places: &ThePlaces) {
    let _ = files::a_folder_is_removed_whole(&places.staging);
}

/// The shell's own managed state, where there is any.
fn handle_state<R: tauri::Runtime, T: Send + Sync + Clone + 'static>(handle: &tauri::AppHandle<R>) -> Option<T> {
    use tauri::Manager;
    handle.try_state::<T>().map(|state| state.inner().clone())
}

/// **P2's hold, in the shell's own keeping** (`Mutex<ThisRunsHold>`).
pub(crate) struct TheShellsHold<'a, R: tauri::Runtime> {
    pub(crate) handle: &'a tauri::AppHandle<R>,
}

impl<R: tauri::Runtime> HandsTheHold for TheShellsHold<'_, R> {
    fn to_the_probe(&mut self) {
        use tauri::Manager;
        if let Some(keeping) = self.handle.try_state::<Mutex<crate::the_one_instance::ThisRunsHold>>() {
            let mut keeping = keeping.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            if let Some(hold) = keeping.the_hold_is_handed_out() {
                crate::the_one_instance::the_hold_is_handed_to_the_probe(hold);
            }
        }
    }

    fn back(&mut self) -> bool {
        use tauri::Manager;
        let crate::the_one_instance::TheLaunch::Runs(hold) = crate::the_one_instance::the_launch_waits_for_the_hold()
        else {
            return false;
        };
        if let Some(keeping) = self.handle.try_state::<Mutex<crate::the_one_instance::ThisRunsHold>>() {
            keeping
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .the_hold_is_kept_again(hold);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downloading::TheRunsTransfers;
    use crate::updating::fetching::tests::{an_entry_for, AnArtifactAtTheSeam};
    use crate::updating::manifest::{UpdateManifest, THE_MANIFEST_FORMAT, THE_MAC_ROW};
    use crate::updating::the_probe::tests::the_probe_child_command;
    use crate::updating::verifying::WhyTheArtifactIsNotTheOneNamed;

    // ── The test forms ─────────────────────────────────────────────────────

    /// **The check's test form**: it accepts or refuses as told, and says
    /// which bundles it was shown.
    struct ACheckThatSays {
        accepts: bool,
        shown: Mutex<Vec<PathBuf>>,
    }

    impl ACheckThatSays {
        fn accepting(accepts: bool) -> ACheckThatSays {
            ACheckThatSays {
                accepts,
                shown: Mutex::new(Vec::new()),
            }
        }
    }

    impl ChecksTheBundle for ACheckThatSays {
        fn accepts(&self, bundle: &Path) -> Result<(), String> {
            self.shown.lock().unwrap().push(bundle.to_path_buf());
            if self.accepts {
                Ok(())
            } else {
                Err("the test's check refused it".into())
            }
        }
    }

    /// **The starts' test form**: this test program re-entered as the probe,
    /// at the test's folder; and a note of each previous build started, with
    /// the arguments it was started with.
    struct StartsThisTestProgram {
        folder: PathBuf,
        version: String,
        acts: String,
        previous: Mutex<Vec<(PathBuf, Vec<String>)>>,
        /// **Started the way the system's launcher starts one** (the
        /// twenty-eighth notes, note 5): the real form's two pipes, made in
        /// the staging folder beside the bundle, and nothing piped.
        launched: bool,
    }

    impl StartsABuild for StartsThisTestProgram {
        fn the_probe(&self, bundle: &Path, token: &str) -> Result<AProbeUnderWatch, String> {
            assert!(bundle.ends_with("Smoozy.app"), "the probe is the installed bundle");
            #[cfg(unix)]
            if self.launched {
                let staging = locations::beside_the_installed_bundle(Location::TheUpdatesStagingFolder, bundle);
                let opens = crate::updating::the_probe::tests::opens_this_test_program(&self.folder, &self.version, &self.acts);
                return AProbeUnderWatch::launched(bundle, &[THE_UPDATE_LAUNCH_ARGUMENT], token, &staging, &opens);
            }
            let mut command = the_probe_child_command(&self.folder, &self.version, &self.acts);
            // After `--`, so the test harness reads it as a name and not an
            // option; the probe child holds that it arrived.
            command.args(["--", THE_UPDATE_LAUNCH_ARGUMENT]).env("SMOOZY_TEST_THE_PROBE_CHILD_WANTS_THE_MODE", "1");
            AProbeUnderWatch::starts(command, token)
        }

        fn the_previous_build(&self, bundle: &Path) -> Result<(), String> {
            self.previous
                .lock()
                .unwrap()
                .push((bundle.to_path_buf(), vec![THE_UPDATE_LAUNCH_ARGUMENT.to_string()]));
            Ok(())
        }
    }

    /// **The hold's test form**: the real hold at the test's folder.
    struct TheHoldAt {
        folder: PathBuf,
        held: Option<crate::the_one_instance::TheHold>,
    }

    impl TheHoldAt {
        fn taken(folder: &Path) -> TheHoldAt {
            let crate::the_one_instance::TheLaunch::Runs(hold) = crate::the_one_instance::the_hold_is_tried_at(
                &folder.join("running-copy.lock"),
                &folder.join("running-copy.door"),
            ) else {
                panic!("the old process takes the hold");
            };
            TheHoldAt {
                folder: folder.to_path_buf(),
                held: Some(hold),
            }
        }
    }

    impl HandsTheHold for TheHoldAt {
        fn to_the_probe(&mut self) {
            if let Some(hold) = self.held.take() {
                crate::the_one_instance::the_hold_is_handed_to_the_probe(hold);
            }
        }

        fn back(&mut self) -> bool {
            match crate::the_one_instance::the_hold_is_waited_for_at(
                &self.folder.join("running-copy.lock"),
                &self.folder.join("running-copy.door"),
            ) {
                crate::the_one_instance::TheLaunch::Runs(hold) => {
                    self.held = Some(hold);
                    true
                }
                _ => false,
            }
        }
    }

    // ── The throwaway installation ─────────────────────────────────────────

    fn a_bundle_at(path: &Path, says: &str) {
        std::fs::create_dir_all(path.join("Contents/MacOS")).unwrap();
        std::fs::write(path.join("Contents/MacOS/which"), says).unwrap();
    }

    fn which(path: &Path) -> String {
        std::fs::read_to_string(path.join("Contents/MacOS/which")).unwrap_or_default()
    }

    /// The Mac download for a bundle that says `says`, zipped by the system's
    /// own tool as the release zips it.
    fn a_download_saying(folder: &Path, says: &str) -> Vec<u8> {
        let making = folder.join(format!("making-{says}"));
        let _ = std::fs::remove_dir_all(&making);
        a_bundle_at(&making.join("Smoozy.app"), says);
        let zip = making.join("Smoozy-macos.zip");
        let made = Command::new("/usr/bin/ditto")
            .args(["-c", "-k", "--keepParent"])
            .arg(making.join("Smoozy.app"))
            .arg(&zip)
            .status()
            .unwrap();
        assert!(made.success());
        std::fs::read(zip).unwrap()
    }

    /// A verified update offering `bytes` for the Mac row, as the reading half's
    /// hands one on.
    fn a_verified_update(version: &str, bytes: &[u8]) -> AVerifiedUpdate {
        AVerifiedUpdate {
            manifest: UpdateManifest {
                manifest_format: THE_MANIFEST_FORMAT.into(),
                release_sequence: 9,
                version: version.into(),
                signing_keys: vec!["online-1".into()],
                entries: vec![an_entry_for(bytes)],
            },
            the_entry_for_this_row: Some(an_entry_for(bytes)),
            signed_by: vec!["online-1".into()],
            the_trust_record_to_keep: TheUpdateTrustRecord {
                greatest_accepted_release_sequence: 9,
                greatest_accepted_trust_sequence: 1,
                the_statement_accepted_at_that_trust_sequence: b"a statement".to_vec(),
                its_recovery_signature: [3; 64],
            },
        }
    }

    fn the_mac_asks() -> WhatTheRequestSays {
        WhatTheRequestSays {
            version: "0.1.8".into(),
            system: THE_MAC_ROW.into(),
        }
    }

    /// **One throwaway installation**: `Apps/Smoozy.app` saying `old`, its
    /// record beside the hold, and a download of a bundle saying `new`.
    struct AnInstallation {
        folder: PathBuf,
        places: ThePlaces,
        record: LocalRecord,
        download: Vec<u8>,
    }

    fn an_installation(name: &str) -> AnInstallation {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-mac-{name}-{}", std::process::id()));
        let installed = folder.join("Apps/Smoozy.app");
        a_bundle_at(&installed, "old");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let download = a_download_saying(&folder, "new");
        AnInstallation {
            places: ThePlaces::beside(&installed),
            folder,
            record,
            download,
        }
    }

    fn staged_in(installation: &AnInstallation, check: &dyn ChecksTheBundle) -> Result<AVerifiedStagedBundle, WhyTheUpdateDidNotHappen> {
        let seam = AnArtifactAtTheSeam::serving(Some(installation.download.clone()));
        the_update_is_fetched_and_verified(
            &seam,
            &TheRunsTransfers::for_this_run(),
            &a_verified_update("0.2.0", &installation.download),
            &the_mac_asks(),
            &installation.places,
            check,
        )
    }

    struct TheSigningAnswers {
        running: &'static str,
        staged: &'static str,
        calls: Mutex<Vec<(String, Vec<String>, PathBuf)>>,
    }

    impl AnswersCodeSigning for TheSigningAnswers {
        fn run(&self, program: &str, arguments: &[&str], code: &Path) -> Result<String, String> {
            self.calls.lock().unwrap().push((program.into(), arguments.iter().map(|one| (*one).into()).collect(), code.into()));
            if arguments.contains(&"--display") {
                Ok(if code == Path::new("running-copy") { self.running } else { self.staged }.into())
            } else {
                Ok(String::new())
            }
        }
    }

    impl ChecksTheBundle for TheSigningAnswers {
        fn accepts(&self, bundle: &Path) -> Result<(), String> {
            the_systems_check(bundle, Path::new("running-copy"), self)
        }
    }

    #[cfg(target_os = "macos")]
    fn d5b_staging_with(running: &'static str, staged: &'static str, name: &str) -> (bool, TheSigningAnswers) {
        let installation = an_installation(name);
        let answers = TheSigningAnswers { running, staged, calls: Mutex::new(Vec::new()) };
        let accepted = match staged_in(&installation, &answers) {
            Ok(_) => true,
            Err(WhyTheUpdateDidNotHappen::TheSystemsOwnCheckRefusedIt(_)) => false,
            Err(other) => panic!("the staging failed before its signer check: {other:?}"),
        };
        assert_eq!(which(&installation.places.installed), "old");
        assert!(update_trust::the_update_in_probe(installation.record.connection()).unwrap().is_none());
        (accepted, answers)
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn d5b_a_matching_developer_id_team_passes_the_staging_gate() {
        let (accepted, answers) = d5b_staging_with("TeamIdentifier=TEAMONE123\n", "TeamIdentifier=TEAMONE123\n", "d5b-match");
        assert!(accepted);
        let calls = answers.calls.lock().unwrap();
        assert_eq!(calls.iter().filter(|(_, args, _)| args.iter().any(|arg| arg == "--display")).count(), 2, "both teams must be read");
        let verifies: Vec<_> = calls.iter().filter(|(_, args, _)| args.iter().any(|arg| arg == "--verify")).collect();
        assert_eq!(verifies.len(), 2, "both Developer ID signatures must be verified");
        for (_, args, _) in verifies {
            assert!(args.iter().any(|arg| arg == "--strict"));
            assert!(args.iter().any(|arg| arg.contains("certificate leaf[subject.OU] = \"TEAMONE123\"")));
            assert!(args.iter().any(|arg| arg.contains(THE_DEVELOPER_ID_REQUIREMENT)));
        }
        assert_eq!(calls.last().unwrap().0, "/usr/sbin/spctl");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn d5b_a_different_team_cannot_pass_the_staging_gate() {
        assert!(!d5b_staging_with("TeamIdentifier=TEAMONE123\n", "TeamIdentifier=TEAMTWO456\n", "d5b-other").0);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn d5b_a_staged_bundle_without_a_team_cannot_pass_the_staging_gate() {
        assert!(!d5b_staging_with("TeamIdentifier=TEAMONE123\n", "TeamIdentifier=not set\n", "d5b-no-staged").0);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn d5b_a_running_copy_without_a_team_cannot_pass_the_staging_gate() {
        assert!(!d5b_staging_with("Signature=adhoc\n", "TeamIdentifier=TEAMONE123\n", "d5b-no-running").0);
    }

    #[test]
    fn d5b_missing_ambiguous_and_malformed_team_answers_are_refused() {
        for answer in ["", "TeamIdentifier=", "TeamIdentifier=not set", "TeamIdentifier=TEAMONE123\nTeamIdentifier=TEAMONE123", "TeamIdentifier=TEAM\"ONE"] {
            assert!(the_team_in(answer).is_err(), "accepted {answer:?}");
        }
    }

    #[test]
    fn d5b_a_failed_signing_tool_or_developer_id_verification_refuses_the_update() {
        struct RefusesAt { call: usize, seen: Mutex<usize> }
        impl AnswersCodeSigning for RefusesAt {
            fn run(&self, _: &str, _: &[&str], _: &Path) -> Result<String, String> {
                let mut seen = self.seen.lock().unwrap();
                *seen += 1;
                if *seen == self.call { Err("refused by the system".into()) }
                else { Ok("TeamIdentifier=TEAMONE123\n".into()) }
            }
        }
        // Running display, staged display, running Developer ID verification,
        // staged Developer ID verification, and Gatekeeper: every one binds.
        for call in 1..=5 {
            let tools = RefusesAt { call, seen: Mutex::new(0) };
            assert!(the_systems_check(Path::new("staged-copy"), Path::new("running-copy"), &tools).is_err());
            assert_eq!(*tools.seen.lock().unwrap(), call);
        }
    }

    // ── Steps 1 to 3 ───────────────────────────────────────────────────────

    /// **A fetched file refused for its size, its fingerprint or the system's
    /// check is an update that did not happen**, and only a bundle the check
    /// accepted is ever staged as verified. The installed bundle is never
    /// touched by any of it.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_fetched_file_is_refused_for_its_size_its_fingerprint_or_the_systems_check() {
        let installation = an_installation("refused");
        let accepting = ACheckThatSays::accepting(true);
        let staged = staged_in(&installation, &accepting).expect("staged and accepted");
        assert_eq!(staged.version(), "0.2.0");
        assert_eq!(which(staged.bundle()), "new");
        assert_eq!(staged.bundle(), installation.places.staging.join("unpacked/Smoozy.app"));
        assert_eq!(accepting.shown.lock().unwrap().clone(), vec![staged.bundle().to_path_buf()]);

        // The system's check refuses it.
        let refusing = ACheckThatSays::accepting(false);
        assert!(matches!(
            staged_in(&installation, &refusing),
            Err(WhyTheUpdateDidNotHappen::TheSystemsOwnCheckRefusedIt(_))
        ));
        // A body one byte longer, and one changed byte: the check is never asked.
        let verified = a_verified_update("0.2.0", &installation.download);
        let mut longer = installation.download.clone();
        longer.push(0);
        let mut changed = installation.download.clone();
        changed[10] ^= 1;
        for (bytes, refused) in [
            (longer, WhyTheArtifactIsNotTheOneNamed::ItsLengthIsNotTheManifests {
                named: installation.download.len() as u64,
                found: installation.download.len() as u64 + 1,
            }),
            (changed, WhyTheArtifactIsNotTheOneNamed::ItsDigestIsNotTheManifests),
        ] {
            let never = ACheckThatSays::accepting(true);
            let seam = AnArtifactAtTheSeam::serving(Some(bytes));
            assert_eq!(
                the_update_is_fetched_and_verified(
                    &seam,
                    &TheRunsTransfers::for_this_run(),
                    &verified,
                    &the_mac_asks(),
                    &installation.places,
                    &never,
                )
                .err(),
                Some(WhyTheUpdateDidNotHappen::TheFetch(WhyTheFetchDidNotPass::ItIsNotTheArtifactNamed(refused)))
            );
            assert!(never.shown.lock().unwrap().is_empty(), "the system's check saw a file the manifest refused");
        }
        // Nothing for this row.
        let mut nothing = a_verified_update("0.2.0", &installation.download);
        nothing.the_entry_for_this_row = None;
        let seam = AnArtifactAtTheSeam::serving(Some(installation.download.clone()));
        assert_eq!(
            the_update_is_fetched_and_verified(
                &seam,
                &TheRunsTransfers::for_this_run(),
                &nothing,
                &the_mac_asks(),
                &installation.places,
                &accepting,
            )
            .err(),
            Some(WhyTheUpdateDidNotHappen::NothingForThisRow)
        );
        assert_eq!(which(&installation.places.installed), "old", "the installed bundle was touched");
    }

    /// The real check refuses unsigned and ad-hoc bundles regardless of
    /// Gatekeeper's policy. Only throwaway bundles and this test program are
    /// read; no installed copy is read and no signing key is used.
    #[cfg(target_os = "macos")]
    #[test]
    fn the_systems_own_check_refuses_an_unsigned_and_an_ad_hoc_bundle() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-mac-real-check-{}", std::process::id()));
        let bundle = folder.join("Throwaway.app");
        std::fs::create_dir_all(bundle.join("Contents/MacOS")).unwrap();
        std::fs::write(
            bundle.join("Contents/Info.plist"),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\"><dict><key>CFBundleExecutable</key><string>throwaway</string><key>CFBundleIdentifier</key><string>dev.smoozy.throwaway.check</string><key>CFBundlePackageType</key><string>APPL</string></dict></plist>\n",
        )
        .unwrap();
        std::fs::copy("/bin/echo", bundle.join("Contents/MacOS/throwaway")).unwrap();
        // The copy of a system program keeps Apple's own signature on the
        // executable, so the bundle itself is what is unsigned.
        let unsigned = TheSystemsOwnCheck.accepts(&bundle);
        assert!(unsigned.is_err(), "an unsigned bundle passed the system's own check");

        let signed = Command::new("/usr/bin/codesign")
            .args(["--force", "--deep", "--sign", "-"])
            .arg(&bundle)
            .output()
            .unwrap();
        assert!(signed.status.success(), "{}", String::from_utf8_lossy(&signed.stderr));
        let ad_hoc = TheSystemsOwnCheck.accepts(&bundle);
        assert!(ad_hoc.is_err(), "an ad-hoc copy passed E18's signer check");
        let displayed = TheCodeSigningTools.run("/usr/bin/codesign", &["--display", "--verbose=4"], &bundle).unwrap();
        assert!(the_team_in(&displayed).is_err(), "ad-hoc signing supplied a team");
        let requirement = format!("={THE_DEVELOPER_ID_REQUIREMENT}");
        assert!(TheCodeSigningTools.run(
            "/usr/bin/codesign",
            &["--verify", "--deep", "--strict", "-R", &requirement],
            &bundle,
        ).is_err(), "ad-hoc signing passed the real Developer ID requirement");
    }

    /// **No route skips the check**: the one place a verified staged bundle is
    /// made is the function that ran the fetch's length and digest and the
    /// system's own check, and publication takes nothing else.
    #[test]
    fn no_route_skips_the_check() {
        let (working, _) = include_str!("the_mac.rs").split_once("#[cfg(test)]").expect("the tests are last");
        assert_eq!(working.matches("Ok(AVerifiedStagedBundle {").count(), 1);
        let maker = working
            .split_once("pub fn the_update_is_fetched_and_verified(")
            .and_then(|(_, rest)| rest.split_once("Ok(AVerifiedStagedBundle {"))
            .map(|(body, _)| body)
            .expect("the maker is here");
        let fetched = maker.find("fetching::the_artifact_is_fetched(").expect("no fetch");
        let checked = maker.find("check.accepts(bundle)").expect("no system check");
        assert!(fetched < checked);
        assert!(working.contains("staged: AVerifiedStagedBundle,\n    places: &ThePlaces,"));
        assert!(!working.contains("impl ChecksTheBundle for ACheck"), "a test form outside the tests");
        assert_eq!(working.matches("impl ChecksTheBundle for").count(), 1, "a second real form");
    }

    // ── Steps 5 to 7 ───────────────────────────────────────────────────────

    /// **The replace and its backup, and the probe passing and clearing its
    /// record**: the installed bundle is the new one, the probe — a real second
    /// process — waits for the handed-over hold, says its token, and once let
    /// go clears the record and removes the named backup. E15's record is kept
    /// before publication.
    ///
    /// **Both ways a probe is started**: as this process's child, and the way
    /// the system's launcher starts one (the twenty-eighth notes, note 5). And
    /// **E18's staging folder goes with the record** — the leftover
    /// `/Applications/.Smoozy update/`.
    #[cfg(target_os = "macos")]
    #[test]
    fn the_replace_keeps_its_backup_and_a_passing_probe_clears_its_record() {
      for launched in [false, true] {
        let installation = an_installation(if launched { "passes-launched" } else { "passes" });
        let staged = staged_in(&installation, &ACheckThatSays::accepting(true)).unwrap();
        let starts = StartsThisTestProgram {
            folder: installation.folder.clone(),
            version: "0.2.0".into(),
            acts: "starts".into(),
            previous: Mutex::new(Vec::new()),
            launched,
        };
        let mut hold = TheHoldAt::taken(&installation.folder);
        let came = the_verified_bundle_is_published(
            staged,
            &installation.places,
            &installation.record,
            &starts,
            &mut hold,
            Duration::from_secs(30),
        );
        assert_eq!(came, WhatThePublicationCameTo::TheNewBuildRuns);
        assert_eq!(which(&installation.places.installed), "new", "the complete bundle was not replaced");
        assert!(starts.previous.lock().unwrap().is_empty(), "a passing probe rolled back");
        assert_eq!(
            update_trust::read_the_trust_record(installation.record.connection()).unwrap().map(|kept| kept.greatest_accepted_release_sequence),
            Some(9),
            "E15: the accepted sequence was not kept before publication"
        );
        // **The probe clears once let go**: its ending lets the hold go, which
        // is waited for on a thread of its own, to a fixed deadline.
        let (got, getting) = std::sync::mpsc::channel();
        let folder = installation.folder.clone();
        std::thread::spawn(move || {
            let mut back = TheHoldAt { folder, held: None };
            let _ = got.send(back.back());
        });
        assert_eq!(getting.recv_timeout(Duration::from_secs(30)), Ok(true), "the probe never let the hold go");
        assert_eq!(update_trust::the_update_in_probe(installation.record.connection()).unwrap(), None);
        assert!(!installation.places.kept.exists(), "Z7's backup outlived the probe's success");
        assert!(!installation.places.staging.exists(), "the staging folder outlived the probe's success");
      }
    }

    /// **The probe failing, and the rollback restoring the old build**: the
    /// probe — a real second process — takes L8's copy, writes as a migration
    /// would, and ends with no token. The old process takes the hold back,
    /// restores L8's copy first, puts the complete previous bundle back,
    /// clears the record, and starts the previous build **in the update-launch
    /// mode alone, so it makes no crash offer** (`Reporting a problem` U5b). A probe that hangs is
    /// ended at the deadline and rolled back the same way.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_failing_probe_is_rolled_back_to_the_old_build_with_no_crash_offer() {
        for (acts, launched) in [("migrates-and-fails", false), ("hangs", false), ("migrates-and-fails", true), ("hangs", true)] {
            let installation = an_installation(&format!("{acts}-{launched}"));
            let staged = staged_in(&installation, &ACheckThatSays::accepting(true)).unwrap();
            let starts = StartsThisTestProgram {
                folder: installation.folder.clone(),
                version: "0.2.0".into(),
                acts: acts.into(),
                previous: Mutex::new(Vec::new()),
                launched,
            };
            let mut hold = TheHoldAt::taken(&installation.folder);
            let came = the_verified_bundle_is_published(
                staged,
                &installation.places,
                &installation.record,
                &starts,
                &mut hold,
                Duration::from_secs(if acts == "hangs" { 2 } else { 30 }),
            );
            let WhatThePublicationCameTo::RolledBack { left_undone, .. } = came else {
                panic!("{acts}: {came:?}");
            };
            assert_eq!(left_undone, Vec::<String>::new(), "{acts}");
            assert_eq!(which(&installation.places.installed), "old", "{acts}: the old build is not back");
            assert!(!installation.places.kept.exists(), "{acts}: the backup was not spent");
            let restored = LocalRecord::open_at(installation.record.path()).unwrap();
            assert_eq!(update_trust::the_update_in_probe(restored.connection()).unwrap(), None, "{acts}");
            assert_eq!(
                update_trust::the_declined_version(restored.connection()).unwrap(),
                None,
                "{acts}: L8's copy was not restored over the probe's write"
            );
            assert!(!record::copy_beside(installation.record.path()).exists(), "{acts}: the copy was not spent");
            assert!(hold.held.is_some(), "{acts}: the hold was not taken back");

            // **`Reporting a problem` U5b: the previous build is started in the update-launch mode
            // alone, and that launch makes no crash offer.**
            let previous = starts.previous.lock().unwrap().clone();
            assert_eq!(previous.len(), 1, "{acts}");
            assert_eq!(previous[0].0, installation.places.installed);
            let mode = crate::start::LaunchMode::from_arguments(previous[0].1.iter().map(std::ffi::OsString::from));
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
        }
        // **The real form starts the previous build through the system's
        // launcher, with the argument alone and nothing in the environment**
        // — read off its own source (the twenty-eighth notes, note 5).
        let (working, _) = include_str!("the_mac.rs").split_once("#[cfg(test)]").unwrap();
        let real = working
            .split_once("impl StartsABuild for ThroughTheSystemsLauncher {")
            .map(|(_, rest)| rest)
            .and_then(|rest| rest.split_once("\n}\n"))
            .map(|(body, _)| body)
            .expect("the real form is here");
        assert!(real.contains("the_probe::the_bundle_is_opened(bundle, &[THE_UPDATE_LAUNCH_ARGUMENT], &the_probe::NONE_OF_THE_PROBES_VARIABLES)"));
        assert!(real.contains("AProbeUnderWatch::launched("));
        assert!(!real.contains("Command::new"), "the real form starts a program directly");
        assert!(working.contains("&ThroughTheSystemsLauncher,\n                    &mut hold,"), "publication does not use the launcher");
    }

    /// **A replacement that fails is an update that did not happen**: nothing
    /// is published, the probe record is cleared, and no probe is started.
    ///
    /// **And E18's staging folder goes** (the twenty-eighth notes' leftover).
    #[cfg(target_os = "macos")]
    #[test]
    fn a_replacement_that_fails_publishes_nothing_and_clears_its_record() {
        let installation = an_installation("no-replace");
        let staged = staged_in(&installation, &ACheckThatSays::accepting(true)).unwrap();
        // The staged bundle goes before publication.
        std::fs::remove_dir_all(staged.bundle()).unwrap();
        assert!(installation.places.staging.exists());
        let starts = StartsThisTestProgram {
            folder: installation.folder.clone(),
            version: "0.2.0".into(),
            acts: "starts".into(),
            previous: Mutex::new(Vec::new()),
            launched: false,
        };
        let mut hold = TheHoldAt::taken(&installation.folder);
        let came = the_verified_bundle_is_published(
            staged,
            &installation.places,
            &installation.record,
            &starts,
            &mut hold,
            Duration::from_secs(5),
        );
        assert!(matches!(came, WhatThePublicationCameTo::NotPublished(WhyTheUpdateDidNotHappen::TheReplacement(_))));
        assert_eq!(which(&installation.places.installed), "old");
        assert!(!installation.places.staging.exists(), "the staging folder outlived an update that did not happen");
        assert_eq!(update_trust::the_update_in_probe(installation.record.connection()).unwrap(), None);
        assert!(hold.held.is_some(), "the hold was handed over for an update that did not happen");
    }

    /// **The four questions, Sol's S5: a failed update names the step it
    /// stopped at** (Design *Updating*) — every reason the Mac route has.
    #[test]
    fn four_questions_s5_each_reason_names_its_step() {
        use super::super::checking::TheStepThatFailed as Step;
        use super::super::fetching::WhyTheFetchDidNotPass as Fetch;
        use super::super::verifying::WhyTheArtifactIsNotTheOneNamed as NotIt;
        use WhyTheUpdateDidNotHappen as Why;
        for (why, step) in [
            (Why::TheFetch(Fetch::CouldNotBeReached("no answer".into())), Step::Download),
            (Why::TheFetch(Fetch::CouldNotBeStaged("disk full".into())), Step::Download),
            (Why::TheStaging("could not unpack".into()), Step::Download),
            (Why::TheFetch(Fetch::ItsNameIsNotAPlainFileName), Step::Verification),
            (Why::TheFetch(Fetch::ItIsNotTheArtifactNamed(NotIt::ItsDigestIsNotTheManifests)), Step::Verification),
            (Why::NothingForThisRow, Step::Verification),
            (Why::NotOneBundleInTheDownload, Step::Verification),
            (Why::TheSystemsOwnCheckRefusedIt("rejected".into()), Step::Verification),
            (Why::NoPlaces("no bundle".into()), Step::Installation),
            (Why::TheRecordBeforePublication("locked".into()), Step::Installation),
            (Why::TheReplacement("permission denied".into()), Step::Installation),
        ] {
            assert_eq!(why.the_step(), step, "{why:?}");
        }
    }

    /// **The Rough what-if answers, share 5 (426, and 61's update half): a
    /// folder beside the application that will not take the staging folder is
    /// said, with what to do.** A folder this login may not write in is a
    /// permission refusal; nothing is fetched, the step is the download, and
    /// About's reason names the folder. Before, it was the staging's own
    /// failure, which About says with no reason. **The Mac's alone**, as the
    /// other staging tests here are: its download is made by the Mac's own
    /// `/usr/bin/ditto`, which a Linux machine does not have (hosted run
    /// 43727887590, 2026-11-15, where it was gated for every Unix).
    #[cfg(target_os = "macos")]
    #[test]
    fn rough_share_5_a_folder_that_will_not_take_the_staging_is_said() {
        use super::super::checking::TheReasonBehind as Reason;
        use std::os::unix::fs::PermissionsExt;
        let installation = an_installation("rough-5-no-permission");
        let apps = installation.places.installed.parent().unwrap().to_path_buf();
        std::fs::set_permissions(&apps, std::fs::Permissions::from_mode(0o555)).unwrap();
        let staged = staged_in(&installation, &ACheckThatSays::accepting(true));
        std::fs::set_permissions(&apps, std::fs::Permissions::from_mode(0o755)).unwrap();
        // **Run as the superuser the folder takes it anyway**, and there is
        // nothing to say.
        if staged.is_ok() {
            eprintln!("SKIPPED: this login writes in a read-only folder");
            return;
        }
        let why = staged.unwrap_err();
        assert_eq!(why, WhyTheUpdateDidNotHappen::ItsFolderCannotBeWritten { read_only: false });
        assert_eq!(why.the_step(), TheStepThatFailed::Download);
        assert_eq!(why.the_reason(), Some(Reason::NoPermissionForItsFolder));
        assert!(!installation.places.staging.exists(), "something was staged");
        assert_eq!(which(&installation.places.installed), "old");
        assert_eq!(
            WhyTheUpdateDidNotHappen::ItsFolderCannotBeWritten { read_only: true }.the_reason(),
            Some(Reason::ItsFolderIsReadOnly)
        );
        // **The read-only volume is read off the mount**: this Mac's system
        // volume is mounted read-only, a scratch folder is not.
        assert!(!files::the_volume_is_read_only(&installation.folder), "a scratch folder read as read-only");
        assert!(files::the_volume_is_read_only(Path::new("/System")), "the system volume did not read as read-only");
    }

    /// **The reason About says after the step, where one is known** (Design
    /// *Updating*, 2026-11-12): the network, the disk, the release's own
    /// length or digest, and the system's check; nothing for the rest.
    #[test]
    fn sixth_part_each_reason_names_what_about_can_say() {
        use super::super::checking::TheReasonBehind as Reason;
        use super::super::fetching::WhyTheFetchDidNotPass as Fetch;
        use super::super::verifying::WhyTheArtifactIsNotTheOneNamed as NotIt;
        use WhyTheUpdateDidNotHappen as Why;
        for (why, reason) in [
            (Why::TheFetch(Fetch::CouldNotBeReached("no answer".into())), Some(Reason::ANetworkError)),
            (Why::TheFetch(Fetch::CouldNotBeStaged("disk full".into())), Some(Reason::TheDiskRefused)),
            (Why::TheFetch(Fetch::ItIsNotTheArtifactNamed(NotIt::ItsDigestIsNotTheManifests)), Some(Reason::NotTheReleasedFile)),
            (Why::TheSystemsOwnCheckRefusedIt("rejected".into()), Some(Reason::TheSystemRefusedIt)),
            (Why::TheFetch(Fetch::ItsNameIsNotAPlainFileName), None),
            (Why::TheStaging("could not unpack".into()), None),
            (Why::NotOneBundleInTheDownload, None),
            (Why::TheReplacement("permission denied".into()), None),
        ] {
            assert_eq!(why.the_reason(), reason, "{why:?}");
        }
    }
}

//! **The update box: once per version, on every system, and the install is
//! the user's press** — Z33, with Z10b and Z29 widened from Linux to all three
//! systems, and Z31's reading kept for the box's one sentence. The Creator's
//! twenty-eighth notes, note 4 (2026-11-09): *"The user should have a choice
//! whether they want that update or not in the first place. Moreover,
//! restarting Smoozy without the user's consent is also bad."*
//!
//! **Two steps, and each route takes both.** [`the_box_is_due`] decides,
//! before anything is fetched, whether the box would be offered: no box
//! already on screen, and on the automatic route a version above the one the
//! declined mark names (Z10b). Then the route makes the version ready — the
//! Mac and Windows routes fetch, stage and verify it first, so *Install now*
//! is quick; Linux fetches after the press — and [`the_box_is_shown`] offers
//! it, with the sentence about running jobs only while something runs.
//!
//! **Nothing here publishes.** The box's *Install now* reaches the route on
//! the check's one sequence (`TheUpdatesAct::TheInstallWasAccepted`), and that
//! is the only thing that starts a publication on any system.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this
//! crate's own plan, `Preparation/Architecture/Desktop app and delivery.md`.
//! Every other document's marker is written with that document at the site.

use std::sync::{Arc, Mutex};

use super::checking::{self, TheReading, TheStepThatFailed, TheUpdateCheck};
use crate::record::update_trust;

/// **Whether the box is offered** (Z10b, Z29): a version above the running
/// one, and — on the automatic route only — above the version the declined
/// mark names.
pub fn should_offer(version: &str, running: &str, declined: Option<&str>, user_asked: bool) -> bool {
    checking::a_version_above(version, running)
        && (user_asked || declined.is_none_or(|declined| checking::a_version_above(version, declined)))
}

/// **Z10b's mark, written at the dismissal** with the version offered.
pub fn dismissed(record: &rusqlite::Connection, version: &str) -> Result<(), String> {
    update_trust::this_version_was_declined(record, version).map_err(|failed| format!("{failed:?}"))
}

/// **Whether the box is due for `version`.** `Ok` where it is; otherwise the
/// reading About is to show, or nothing where a box is already on screen. A
/// check the user asked for, or joined, is read from `check` (Z29).
pub fn the_box_is_due<R: tauri::Runtime>(
    handle: &tauri::AppHandle<R>,
    check: &TheUpdateCheck,
    version: &str,
    user_asked: bool,
) -> Result<(), Option<TheReading>> {
    use tauri::Manager;
    if check.has_offer() {
        return Err(None);
    }
    let user_asked = user_asked || check.the_completed_check_was_user_asked();
    let declined = if user_asked {
        None
    } else {
        let started = handle
            .try_state::<Mutex<crate::start::Started>>()
            .ok_or(Some(TheReading::CouldNotInstall(TheStepThatFailed::Installation, None)))?;
        let started = started.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        match started.record.as_ref().map(|record| update_trust::the_declined_version(record.connection())) {
            Some(Ok(declined)) => declined,
            _ => return Err(Some(TheReading::CouldNotInstall(TheStepThatFailed::Installation, None))),
        }
    };
    if should_offer(version, checking::THIS_BUILDS_VERSION, declined.as_deref(), user_asked) {
        Ok(())
    } else {
        // **Quiet**: the mark names it. About still reads the version found,
        // with *Install now…*.
        Err(Some(TheReading::ANewerVersion { version: version.to_string() }))
    }
}

/// **The box shown for `version`**, one at a time, with Z32's sentence where
/// anything is running (Z31's reading, read now), and the setups sentence
/// where this copy drives a setup run on any server, read now too (Z33,
/// widened 2026-11-14; the Rough what-if answers, share 5, 424).
pub fn the_box_is_shown<R: tauri::Runtime>(handle: &tauri::AppHandle<R>, check: &TheUpdateCheck, version: &str) {
    use tauri::Manager;
    if !check.offer_version(version) {
        return;
    }
    let running = super::the_hold::something_is_running_here(handle);
    let setups = crate::setting_up::a_setup_run_is_being_driven();
    if let Some(queue) = handle.try_state::<Arc<crate::events::TheOneQueue>>() {
        let version = version.to_string();
        let _ = queue.read_then_post(
            || Ok::<_, ()>(()),
            move |_| crate::surface::menus::the_update_box(&version, running, setups, cfg!(target_os = "linux")),
        );
    }
}

/// **The box answered *Skip this version*** — the mark written through the record's one
/// lock. About keeps the version found; a mark that would not write is said as
/// an install that failed, so the box is not quietly lost.
pub fn the_dismissal_is_written<R: tauri::Runtime>(handle: &tauri::AppHandle<R>, version: &str) -> Option<TheReading> {
    use tauri::Manager;
    let started = handle.try_state::<Mutex<crate::start::Started>>()?;
    let started = started.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let record = started.record.as_ref()?;
    dismissed(record.connection(), version)
        .err()
        .map(|_| TheReading::CouldNotInstall(TheStepThatFailed::Installation, None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;

    /// **Z10b and Z29 on every system** (Z33): once per version on the
    /// automatic route, any newer version on the user's, and never the running
    /// version or an older one.
    #[test]
    fn twenty_eighth_4_the_box_is_once_per_version_on_every_system() {
        assert!(should_offer("0.3.2", "0.3.1", None, false));
        assert!(!should_offer("0.3.2", "0.3.1", Some("0.3.2"), false), "a declined version was offered again");
        assert!(should_offer("0.3.3", "0.3.1", Some("0.3.2"), false), "a newer version than the declined one was not offered");
        assert!(should_offer("0.3.2", "0.3.1", Some("0.3.2"), true), "a check the user asked for read the mark");
        assert!(!should_offer("0.3.1", "0.3.1", None, true));
        assert!(!should_offer("0.3.0", "0.3.1", None, true));
    }

    /// **The mark is the one value, on every system**, and *Skip this version* writes it.
    #[test]
    fn twenty_eighth_4_later_writes_the_one_mark() {
        let folder = crate::scratch_folders::a_folder(format!("smoozy-the-box-{}", std::process::id()));
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        assert_eq!(update_trust::the_declined_version(record.connection()).unwrap(), None);
        dismissed(record.connection(), "0.3.2").unwrap();
        assert_eq!(update_trust::the_declined_version(record.connection()).unwrap().as_deref(), Some("0.3.2"));
    }

    /// **Nothing here publishes, and nothing here restarts the
    /// application** — read off this file's own working code.
    #[test]
    fn twenty_eighth_4_offering_the_box_starts_nothing() {
        let (working, _) = include_str!("the_box.rs").split_once("#[cfg(test)]").expect("the tests are last");
        for never in ["published", "process::exit", "the_bundle_is_replaced", "Command::new", "transact("] {
            assert!(!working.contains(never), "the box's code reaches {never}");
        }
    }
}

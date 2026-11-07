//! **Settings — what belongs to this computer or to the whole app, in one
//! screen.** The page half is `web/settings.js`, which draws into the place the
//! mounting contract at the head of `web/the-rail.js` calls `stgview`.
//!
//! **This file is the surface and not the deciding.** Which folders a search
//! reads and in what order is `Dependency Upload & Remap`'s; the restore control is
//! [`crate::surface::said_once`]'s; the four notification kinds are
//! [`crate::notifications`]'. **Nothing here re-decides any of them.**
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! **The Design document and its headings**, and **the mockup state names**.
//! `Preparation/Design/Settings.md` carries no clause marker:
//!
//! - `Design/Settings § Where it sits` — a screen at the bottom of the rail,
//!   and the menu bar's *Settings…*, both of which the shell already routes
//!   here.
//! - `§ What is in it, and what is not` — the section list down its own side,
//!   one section showing at a time, no General section.
//! - `§ Transfers` — the download folder, the two caps and the line saying
//!   they are totals.
//! - `§ Search folders` — the numbered list, *could not be read*, Remove, the
//!   line saying a folder covers everything inside it; and `§ What this
//!   document left open` for adding one, a folder already on the list, and the
//!   up, down and drag controls.
//! - `§ Warnings and notifications` — the four switches, the line under them,
//!   and the restore control, whose count is `§ What this document left open`'s.
//!
//! **The four mockup states this piece draws** are
//! [`THE_STATES_THIS_PIECE_DRAWS`]; **the Telegram section's fourteen** are
//! [`crate::surface::telegram::THE_STATES_THIS_PIECE_DRAWS`] (seven until the
//! bot step was drawn again, the seventeenth notes' piece 7). A test reads the
//! eighteen retained states out of `Preparation/Design/Mockup/Smoozy.html`'s own state list. The
//! accounts are `Audits/Build/Step 10a/Piece 11 — what landed.md` and
//! `Audits/Build/Step 10/Piece 8 — what landed.md`.
//!
//! ## Telegram is the fourth section, and is `crate::surface::telegram`'s
//!
//! **Step 10 piece 8 built it**, 2026-09-14. Until then the list held four and
//! the line under the notification switches dropped its second sentence,
//! the one pointing at Telegram, because neither was true of this
//! application. Both are drawn now.
//!
//! ## What can actually happen today, said plainly
//!
//! - **The search-folder list is real.** `Dependency Upload & Remap` F1's one
//!   app-wide ordered list is the local record's `dependency_searched_folder`
//!   table, which had no keeper; the keeper is [`the_list`], below. Adding a
//!   folder opens the system picker; moving and removing rewrite the list in one
//!   transaction. **The mark is the last search's and never the row's**
//!   (`Dependency Upload & Remap` F1a, F7): it is read from the dependency
//!   check's own run-held store, and nothing reads a folder because this screen
//!   was opened. **It carries the moment of that search**
//!   (`Dependency Upload & Remap` F1a, `State, identity and recovery` D5):
//!   the search's own report takes it, the store
//!   keeps it beside the verdicts, and the row says *could not be read at
//!   09:20*. *(Until step 10c's piece 17 of the 38 the store dropped the
//!   moment, and this said it was not drawn.)* After a relaunch no row is
//!   marked until a search runs, which is `Dependency Upload & Remap` F1a's
//!   own rule.
//! - **The restore control is real**, and so is its count.
//! - **The seven settings are saved for real.** The download folder, the two
//!   caps and the four notification switches are *settings*, user-held
//!   (`State, identity and recovery` §11.1), and version 17 of the local record
//!   gave them their home; the keeper is [`crate::record::settings`]. **Every
//!   change is written, then read back, then drawn** (P7): a control never
//!   shows a value the record did not answer (L8a), and a change that did not
//!   land is said beside the control with the record's own value put back.
//!   *(Until 2026-09-13 every one of the seven answered that it could not be
//!   saved yet; the account is `Audits/Build/Step 10a/The settings store — what
//!   landed.md`.)*
//! - **A switch is handed to the notification service as soon as it is
//!   written**, and the service asks it before anything else — T17a's third
//!   bullet, before T7's permission request.
//! - **A cap is typed in MB/s and kept in bytes per second**, and the one
//!   conversion is here and nowhere else (R2). **A megabyte is 1,000,000
//!   bytes**, ruled 2026-09-13.
//! - ~~**What still reads a saved cap or the saved folder: nothing.**~~
//!   **Corrected at step 10c's piece 17 of the 38: both are read.** A cap is
//!   taken into the run's budgets at the start and at every change
//!   ([`a_speed_cap_is_set`], R3 and `Server protocol and transfers` R8), and
//!   the run's admission point is what transfers go through; the saved folder
//!   is read where a download is given its destination (`src/downloading.rs`).
//! - **Each cap has what is actually running beside it** (`Design/Settings §
//!   Transfers`) — *running at 12.4 MB/s*. The figure is R15 and R15a's
//!   reading, taken at the run's one desktop-side admission point, and it is
//!   the pair Transfers already draws
//!   ([`crate::surface::transfers::the_live_readings`]); **nothing here works
//!   it out a second time.** Nothing is drawn where that direction has no
//!   channel. The page asks again whenever Transfers is told a transfer moved
//!   ([`crate::surface::transfers::TRANSFERS_MOVED`]), so the reading follows
//!   the transfer without anything polling. *(Until step 10c's piece 17 of the
//!   38 no reading was drawn here at all.)*
//! - **The download folder is the one the user chose, and §8's proposal where
//!   they chose none**, read fresh through `src/platform/downloads_folder.rs`
//!   and never written into the record (W3). Where neither exists,
//!   `Download` H3's answer — the user is asked before a download starts — is
//!   what the section says.
//!
//! ## `WhoseSentenceItIs` is not reached
//!
//! **Nothing here raises a failure record.** A refusal on this screen is the
//! answer to the press the user just made, drawn in the one failure shape and
//! gone at the next reading, as on pieces 8 and 9's pages. A test says so.
//!
//! ## How a clause marker is written in this file
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every marker of
//! another document is written with that document at the site**, and no header
//! list is used.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::SystemTime;

use tauri::{AppHandle, Manager, Runtime, State};

use smoozy_dependencies::search::{FolderVerdict, SearchedFolderList};

use smoozy_server_protocol::transfers::Direction;

use crate::downloading::TheRunsTransfers;
use crate::events::{Event, Said, Subject, TheOneQueue};
use crate::notifications::kinds::Kind;
use crate::notifications::TheNotificationService;
use crate::picker::{self, Asking, HowMany, Picked, WhatMayBeChosen};
use crate::record::settings::{self as the_store, TheSettings};
use crate::record::LocalRecord;
use crate::scheduling::{BytesPerSecond, TheCap};
use crate::start::Started;
use crate::typography::{Prose, Verbatim};

// ════════════════════════════════════════════════════════════════════════════
// The names this piece and its page agree on
// ════════════════════════════════════════════════════════════════════════════

/// **The screen's own words, and the seven settings as the record holds them.**
/// `web/settings.js` names the same string.
pub const THE_SCREEN: &str = "settings:the-screen";

/// **The search-folder list.**
pub const THE_SEARCH_FOLDERS: &str = "settings:the-search-folders";

/// **What is actually running beside each cap** — `Design/Settings §
/// Transfers`. Its own event, so the page can ask for it as a transfer moves
/// without the rest of the screen being drawn again under the user's typing.
pub const THE_SPEEDS: &str = "settings:the-speeds";

/// **Where an answer to a change is drawn**, as the page's holders are named.
/// `web/settings.js` names the same three strings.
pub const THE_FOLDERS_ANSWER: &str = "download-folder";
pub const THE_CAPS_ANSWER: &str = "caps";
pub const THE_SWITCHES_ANSWER: &str = "notifications";
/// **The Telegram section's own holder, for the idle switch drawn there.** A
/// switch's answer is drawn beside the switch that was pressed and nowhere else
/// — must-do 16 of step 10a round 1 of the queue, history, settings and
/// downloads. `web/telegram.js` names the same string.
pub const THE_TELEGRAM_IDLE_ANSWER: &str = "telegram-idle";

/// The four retained non-Telegram Settings states. The five withdrawn paid
/// states remain in the mockup until wave 4, and the test excludes only those.
pub const THE_STATES_THIS_PIECE_DRAWS: [&str; 4] = [
    "set",
    "set-drag",
    "set-dupe",
    "set-nowarn",
];

// ════════════════════════════════════════════════════════════════════════════
// Every word the user reads
// ════════════════════════════════════════════════════════════════════════════

/// **Crate-visible so that Transfers draws the caps in this screen's own words**
/// — step 10a round 1 of the queue, history, settings and downloads, must-do 4.
pub(crate) mod words {
    // `§ What is in it, and what is not`.
    pub const SETTINGS: &str = "Settings";
    pub const THE_SECTIONS: &str = "Settings sections";
    pub const TRANSFERS: &str = "Transfers";
    pub const SEARCH_FOLDERS: &str = "Search folders";
    pub const WARNINGS_AND_NOTIFICATIONS: &str = "Warnings and notifications";
    pub const TELEGRAM: &str = "Telegram";

    // `§ Transfers`.
    pub const DOWNLOAD_FOLDER: &str = "Download folder";
    pub const CHOOSE: &str = "Choose…";
    pub const NO_FOLDER_IS_SET: &str = "No folder set.";
    pub const SMOOZY_ASKS_BEFORE_A_DOWNLOAD: &str =
        "You are asked before each download.";
    pub const ONE_DOWNLOAD_ELSEWHERE: &str =
        "Each download can use a different folder.";
    pub const SPEED_LIMITS: &str = "Speed limits";
    pub const UPLOADING: &str = "Uploading";
    pub const DOWNLOADING: &str = "Downloading";
    pub const NO_CAP: &str = "no cap";
    pub const MB_PER_SECOND: &str = "MB/s";
    pub const UPLOAD_CAP: &str = "Upload cap";
    pub const DOWNLOAD_CAP: &str = "Download cap";
    pub const EACH_CAP_IS_A_TOTAL: &str =
        "Each cap limits all transfers on all servers combined.";
    pub const EMPTY_MEANS_NO_CAP: &str = "Empty means no cap.";
    /// **`Design/Settings § Transfers`' reading beside a cap** — the drawing's
    /// own *running at 12.4 MB/s*. The figure and its unit are a value.
    pub const RUNNING_AT: &str = "running at";

    // `§ Search folders`, and `§ What this document left open`.
    pub const A_FOLDER_COVERS_EVERYTHING: &str =
        "Subfolders are included.";
    pub const CHOOSE_THE_FOLDER_HOLDING_ALL: &str =
        "Add the folder that holds all your work, not each project.";
    pub const ADD_A_FOLDER: &str = "Add a folder…";
    pub const ADD_A_SEARCH_FOLDER: &str = "Add a search folder";
    pub const DRAG_OR_USE_THE_ARROWS: &str = "Drag a row, or use the arrows.";
    /// *Changed 2026-11-06, the last words read before the release*: it went
    /// on *, so the order decides* **which folder** *a file comes from, never
    /// which file inside it.*, which said again what this says.
    pub const THE_FIRST_FOLDER_HOLDING_A_MATCH: &str = "The first folder with a match is used.";
    pub const WHERE_ONE_FOLDER_HOLDS_TWO: &str =
        "If one folder has two matches, one is used and named.";
    pub const MOVE_UP: &str = "Move up";
    pub const MOVE_DOWN: &str = "Move down";
    pub const REMOVE: &str = "Remove";
    /// **`Design/Settings § Search folders`' one word**, which the dependency
    /// check's panel draws in the same words — a test holds the two together.
    pub const COULD_NOT_BE_READ: &str = "could not be read";
    /// **The word before the moment of the search** — the drawing's own
    /// *could not be read at 09:20*. The moment is a value.
    pub const AT: &str = "at";
    pub const THE_LIST_WOULD_NOT_READ: &str = "Could not read search folders.";
    pub const THE_CHANGE_DID_NOT_SAVE: &str = "Could not save the change.";

    // `§ Warnings and notifications`.
    pub const NOTIFICATIONS: &str = "Notifications";
    pub const A_JOB_FINISHES: &str = "A job finishes";
    pub const WITH_THE_FRAME: &str = "with the frame, where the job has one";
    pub const A_JOB_FAILS: &str = "A job fails or is abandoned";
    pub const A_CONNECTION_COULD_NOT_BE_RESTORED: &str = "A reconnection fails";
    pub const A_SERVER_HAS_HAD_NOTHING_TO_DO: &str =
        "Server idle for 30\u{a0}minutes";
    pub const THESE_ARRIVE_ONLY_WHILE_RUNNING: &str = "These need Smoozy to be running.";
    /// **`§ Warnings and notifications`: the line *"points at Telegram"*.**
    pub const TELEGRAM_WORKS_WHEN_IT_IS_NOT: &str = "Telegram does not.";

    // The seven settings, saved. **Design gives no words for any of these
    // three**, so they are this piece's, in the standard words Rule 9 asks for.
    pub const CHOOSE_A_DOWNLOAD_FOLDER: &str = "Choose a download folder";
    pub const THE_SETTINGS_WOULD_NOT_READ: &str = "Could not read settings.";
    pub const THE_SETTING_DID_NOT_SAVE: &str = "Could not save the setting.";
    pub const A_CAP_IS_A_SPEED_ABOVE_NOUGHT: &str =
        "Enter a speed above 0\u{a0}MB/s, or leave empty for no cap.";

}

fn prose(event: Event, name: &'static str, said: &'static str) -> Event {
    event.saying(name, Said::Prose(Prose::new(said)))
}

fn value(event: Event, name: &'static str, said: impl Into<String>) -> Event {
    event.saying(name, Said::Value(Verbatim::new(said.into())))
}

/// **An indexed field name, made once and kept.** An event's field names are
/// `&'static str` by [`Event::saying`]'s own signature and a list has as many
/// rows as the user made, so each name is leaked **the first time it is needed
/// and never again** — a list redrawn a thousand times costs its longest length
/// in names, not a thousand.
///
/// **The one name-maker for every list on a screen**, and crate-visible so the
/// queue, history and their shared entry use it rather than a fixed table that
/// cuts a list short (L8a) — must-do 3 of step 10a round 1 of the queue,
/// history, settings and downloads.
pub(crate) fn a_name(prefix: &str, at: usize, tail: &str) -> &'static str {
    a_kept_name(format!("{prefix}{at}{tail}"))
}

/// The same keeping, for a name with no index in it.
pub(crate) fn a_kept_name(key: String) -> &'static str {
    static NAMES: OnceLock<Mutex<BTreeMap<String, &'static str>>> = OnceLock::new();
    let mut held = NAMES
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(name) = held.get(&key) {
        return name;
    }
    let name: &'static str = Box::leak(key.clone().into_boxed_str());
    held.insert(key, name);
    name
}

// ════════════════════════════════════════════════════════════════════════════
// The screen's own words, and the settings with nowhere to be kept
// ════════════════════════════════════════════════════════════════════════════

/// **What the screen is drawn from**: the proposal, read fresh, and the seven
/// settings as the record answered them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheScreenHolds {
    /// §8's proposal, where the route named one. **Never written anywhere** (W3).
    pub the_proposal: Option<Vec<u8>>,
    /// **`None` is a record that did not answer**, and then no setting is drawn
    /// at all rather than a default in its place (L8a).
    pub the_settings: Option<TheSettings>,
    /// **The answer to the change the user just made**, where it did not land:
    /// which holder it is drawn in, which switch it is about where it is about
    /// one, and the sentence.
    pub the_answer: Option<TheAnswer>,
}

/// **An answer to one change, and where it is drawn.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheAnswer {
    /// The holder the page draws it in.
    pub holder: &'static str,
    /// **The switch it is about**, so a failed *A job finishes* is never drawn
    /// under the idle switch (must-do 16).
    pub about: Option<Kind>,
    pub sentence: &'static str,
}

/// **Where a notification switch was pressed** — H34's closed request type.
/// The idle switch is drawn twice, in Settings and in its Telegram section, and
/// the answer goes beside the one that was pressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TheSwitchWasPressedIn {
    Settings,
    Telegram,
}

/// **The field a kind's on-or-off travels in, beside its words' own field.**
fn the_switch_field_of(kind: Kind) -> &'static str {
    match kind {
        Kind::AJobFinished => "aJobFinishesIsOn",
        Kind::AJobFailedOrWasGivenUp => "aJobFailsIsOn",
        Kind::AConnectionCouldNotBeRestored => "aConnectionCouldNotBeRestoredIsOn",
        Kind::AMachineHasBeenIdle => "aServerHasHadNothingToDoIsOn",
    }
}

/// **What the screen says** — the four section names, the transfers section,
/// the notification switches, and the seven settings as values beside them.
///
/// **The folder drawn is the user's own where they chose one, and the proposal
/// otherwise** (`Download` H2); where there is neither, the section says what
/// `Download` H3 does about that.
pub fn the_screen_says(holds: &WhatTheScreenHolds) -> Event {
    let mut event = Event::about(Subject::TheRun, THE_SCREEN);
    for (name, said) in [
        ("title", words::SETTINGS),
        ("theSections", words::THE_SECTIONS),
        ("transfersSection", words::TRANSFERS),
        ("searchFoldersSection", words::SEARCH_FOLDERS),
        ("warningsSection", words::WARNINGS_AND_NOTIFICATIONS),
        ("telegramSection", words::TELEGRAM),
        // `§ Transfers`.
        ("downloadFolder", words::DOWNLOAD_FOLDER),
        ("choose", words::CHOOSE),
        ("oneDownloadElsewhere", words::ONE_DOWNLOAD_ELSEWHERE),
        ("speedLimits", words::SPEED_LIMITS),
        ("uploading", words::UPLOADING),
        ("downloading", words::DOWNLOADING),
        ("noCap", words::NO_CAP),
        ("perSecond", words::MB_PER_SECOND),
        ("uploadCap", words::UPLOAD_CAP),
        ("downloadCap", words::DOWNLOAD_CAP),
        ("eachCapIsATotal", words::EACH_CAP_IS_A_TOTAL),
        ("emptyMeansNoCap", words::EMPTY_MEANS_NO_CAP),
        // `§ Warnings and notifications`. **`Desktop app and delivery` T17a
        // bounds what a switch may claim: the kind, never delivery.** The
        // finished-job qualifier is Design's own answer to that bound.
        ("notifications", words::NOTIFICATIONS),
        ("aJobFinishes", words::A_JOB_FINISHES),
        ("withTheFrame", words::WITH_THE_FRAME),
        ("aJobFails", words::A_JOB_FAILS),
        ("aConnectionCouldNotBeRestored", words::A_CONNECTION_COULD_NOT_BE_RESTORED),
        ("aServerHasHadNothingToDo", words::A_SERVER_HAS_HAD_NOTHING_TO_DO),
        ("theseArriveOnlyWhileRunning", words::THESE_ARRIVE_ONLY_WHILE_RUNNING),
        ("telegramWorksWhenItIsNot", words::TELEGRAM_WORKS_WHEN_IT_IS_NOT),
    ] {
        event = prose(event, name, said);
    }
    if let Some(answer) = holds.the_answer {
        event = value(event, "theAnswerFor", answer.holder);
        if let Some(kind) = answer.about {
            event = value(event, "theAnswerAbout", the_store::the_stored_name_of(kind));
        }
        event = prose(event, "theAnswer", answer.sentence);
    }
    let Some(settings) = holds.the_settings.as_ref() else {
        // **L8a: no folder, no cap and no switch is drawn from a read that did
        // not answer** — not the proposal either, which would be a default put
        // where the user's own choice may be.
        return prose(event, "theSettingsWouldNotRead", words::THE_SETTINGS_WOULD_NOT_READ);
    };
    let folder = settings
        .the_chosen_download_folder
        .as_ref()
        .or(holds.the_proposal.as_ref());
    event = match folder {
        Some(folder) => value(event, "theDownloadFolder", String::from_utf8_lossy(folder)),
        None => {
            let event = prose(event, "noFolderIsSet", words::NO_FOLDER_IS_SET);
            prose(event, "smoozyAsksBeforeADownload", words::SMOOZY_ASKS_BEFORE_A_DOWNLOAD)
        }
    };
    // **R8: no cap is an empty field**, so no value travels for it.
    for (field, cap) in [
        ("uploadCapIs", settings.the_upload_cap),
        ("downloadCapIs", settings.the_download_cap),
    ] {
        if let TheCap::Of(bytes) = cap {
            event = value(event, field, megabytes_per_second(bytes));
        }
    }
    for kind in Kind::ALL {
        let on = settings.the_switches.attempts(kind);
        event = value(event, the_switch_field_of(kind), if on { "yes" } else { "no" });
    }
    event
}

/// **R2's one conversion, outward**: bytes per second to the MB/s the field
/// shows. **A megabyte is 1,000,000 bytes**, and the figure is written with no
/// trailing noughts so that what the user typed comes back as they typed it.
pub fn megabytes_per_second(rate: BytesPerSecond) -> String {
    let whole = rate.0 / 1_000_000;
    let part = rate.0 % 1_000_000;
    if part == 0 {
        return whole.to_string();
    }
    let part = format!("{part:06}");
    format!("{whole}.{}", part.trim_end_matches('0'))
}

/// **R2's one conversion, inward**: what the user typed in MB/s, as a cap.
///
/// **An empty field is no cap** (`Design/Settings § Transfers`, and R8).
/// Anything else has to be a speed above nought that comes to at least one
/// byte a second; a nought would stop every transfer, and a field that is not a
/// number is not a cap. **Refused rather than guessed at**, and the user is told.
pub fn the_cap_typed(typed: &str) -> Result<TheCap, ()> {
    let typed = typed.trim();
    if typed.is_empty() {
        return Ok(TheCap::NoCapIsSet);
    }
    let (whole, part) = match typed.split_once('.') {
        Some((whole, part)) => (whole, part),
        None => (typed, ""),
    };
    let digits = |text: &str| text.bytes().all(|byte| byte.is_ascii_digit());
    if !digits(whole) || !digits(part) || (whole.is_empty() && part.is_empty()) || part.len() > 6 {
        return Err(());
    }
    let whole: u64 = if whole.is_empty() { 0 } else { whole.parse().map_err(|_| ())? };
    let part: u64 = format!("{part:0<6}").parse().map_err(|_| ())?;
    let bytes = whole
        .checked_mul(1_000_000)
        .and_then(|bytes| bytes.checked_add(part))
        .filter(|bytes| *bytes > 0 && i64::try_from(*bytes).is_ok())
        .ok_or(())?;
    Ok(TheCap::Of(BytesPerSecond(bytes)))
}

/// **What the screen holds now**, read fresh: the proposal (§8, W3: nothing
/// caches it) and the record's answer.
fn what_the_screen_holds(record: Option<&LocalRecord>) -> WhatTheScreenHolds {
    let the_proposal = match crate::platform::downloads_folder::the_folder_the_system_proposes() {
        crate::platform::downloads_folder::TheProposal::TheSystemNamesThisFolder(folder) => {
            Some(folder.as_os_str().as_encoded_bytes().to_vec())
        }
        crate::platform::downloads_folder::TheProposal::TheSystemNamesNoFolder(_) => None,
    };
    WhatTheScreenHolds {
        the_proposal,
        the_settings: record.and_then(|record| the_store::read(record.connection()).ok()),
        the_answer: None,
    }
}

/// **One change, written, read back and drawn** — P7's first door.
///
/// `write` is the user's change and answers a sentence where it was refused or
/// did not land. **Whatever it answers, the screen is then drawn from what the
/// record holds**, so a control the change did not reach goes back to the
/// record's own value rather than keeping what was typed (L8a). **The
/// notification service is handed the switches the record read back**, so a
/// switch takes effect on the next post and never on one the record does not
/// hold.
fn a_setting_is_written(
    record: Option<&LocalRecord>,
    queue: &TheOneQueue,
    notifications: Option<&TheNotificationService>,
    holder: &'static str,
    about: Option<Kind>,
    write: impl FnOnce(&rusqlite::Connection) -> Result<(), &'static str>,
) {
    let _ = queue.write_then_post::<WhatTheScreenHolds, std::convert::Infallible>(
        || Ok(the_change_is_made(record, notifications, holder, about, write)),
        the_screen_says,
    );
}

/// The whole of [`a_setting_is_written`] but the posting, so a test can stand
/// where a command stands.
fn the_change_is_made(
    record: Option<&LocalRecord>,
    notifications: Option<&TheNotificationService>,
    holder: &'static str,
    about: Option<Kind>,
    write: impl FnOnce(&rusqlite::Connection) -> Result<(), &'static str>,
) -> WhatTheScreenHolds {
    let refused = match record {
        Some(record) => write(record.connection()).err(),
        None => Some(words::THE_SETTING_DID_NOT_SAVE),
    };
    let mut holds = what_the_screen_holds(record);
    if let Some(service) = notifications {
        service.the_switches_are(holds.the_settings.as_ref().map(|settings| settings.the_switches));
    }
    holds.the_answer = refused.map(|sentence| TheAnswer { holder, about, sentence });
    holds
}

/// **Which direction a cap is for** — H34's closed request type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TheDirection {
    Upload,
    Download,
}

// ════════════════════════════════════════════════════════════════════════════
// The search-folder list — `Dependency Upload & Remap` F1
// ════════════════════════════════════════════════════════════════════════════

/// **The keeper of `dependency_searched_folder`**, which that module's own
/// `arriving_later` names as absent under its F1 entry.
///
/// **It lives here rather than beside the other keepers in `src/record/`**
/// because this piece's brief names the files it may make and that folder is
/// not among them. It is the whole of what reads or writes the table; moving it
/// there is one cut and one `pub mod` line.
///
/// **The position is the user's order** (`Dependency Upload & Remap` F4
/// decides on it), so the list is always written whole, in one transaction,
/// with positions counted from one: a move or a removal leaves no gap and no
/// half-moved list.
pub mod the_list {
    use smoozy_dependencies::search::SearchedFolderList;

    pub fn read(connection: &rusqlite::Connection) -> rusqlite::Result<SearchedFolderList> {
        let mut statement = connection
            .prepare("SELECT folder FROM dependency_searched_folder ORDER BY position")?;
        let folders = statement
            .query_map([], |row| row.get::<_, Vec<u8>>(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(SearchedFolderList { folders })
    }

    pub fn write(
        connection: &rusqlite::Connection,
        list: &SearchedFolderList,
    ) -> rusqlite::Result<()> {
        connection.execute_batch("BEGIN IMMEDIATE")?;
        let done = (|| -> rusqlite::Result<()> {
            connection.execute("DELETE FROM dependency_searched_folder", [])?;
            for (at, folder) in list.folders.iter().enumerate() {
                connection.execute(
                    "INSERT INTO dependency_searched_folder (position, folder) VALUES (?1, ?2)",
                    rusqlite::params![(at + 1) as i64, folder],
                )?;
            }
            Ok(())
        })();
        match done {
            Ok(()) => connection.execute_batch("COMMIT"),
            Err(failure) => {
                let _ = connection.execute_batch("ROLLBACK");
                Err(failure)
            }
        }
    }
}

/// **A folder added.** `§ What this document left open`, second bullet: *"A
/// folder already on the list is not added twice and is not refused either:
/// the list flashes the row it is already on … A folder inside one already on
/// the list **is** added."* So the only answer that is not a new list is the
/// row it is already on.
pub fn the_list_with(list: &SearchedFolderList, folder: Vec<u8>) -> Result<SearchedFolderList, usize> {
    if let Some(at) = list.folders.iter().position(|held| *held == folder) {
        return Err(at);
    }
    let mut folders = list.folders.clone();
    folders.push(folder);
    Ok(SearchedFolderList { folders })
}

/// **A row moved** from one place to another, both counted from nought. A place
/// outside the list is refused — H34's validation before anything is touched.
pub fn the_list_moved(list: &SearchedFolderList, from: usize, to: usize) -> Option<SearchedFolderList> {
    if from >= list.folders.len() || to >= list.folders.len() {
        return None;
    }
    let mut folders = list.folders.clone();
    let moving = folders.remove(from);
    folders.insert(to, moving);
    Some(SearchedFolderList { folders })
}

/// **A row removed.** `§ Search folders`: *"removing a marked folder is no
/// different from removing any other."*
pub fn the_list_without(list: &SearchedFolderList, at: usize) -> Option<SearchedFolderList> {
    if at >= list.folders.len() {
        return None;
    }
    let mut folders = list.folders.clone();
    folders.remove(at);
    Some(SearchedFolderList { folders })
}

/// **H34's closed response for the list**, composed and then posted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheListSays {
    pub the_list: TheList,
    /// The row a folder already on the list is on, which the page flashes.
    pub already_on_it: Option<usize>,
    /// **A change that did not save adds a line and takes none away** — the
    /// list as it stands is still drawn, which is `said_once`'s own C-2 shape.
    pub the_change_did_not_save: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheList {
    Read(Vec<AListedFolder>),
    /// **L8a: a list that would not read is said, never drawn short.**
    ItWouldNotRead,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AListedFolder {
    pub folder: Vec<u8>,
    /// **The last search's verdict and never the row's** (`Dependency Upload &
    /// Remap` F1a).
    pub could_not_be_read: bool,
    /// **The moment of that search** (`Dependency Upload & Remap` F1a:
    /// *"Where the verdict is drawn it
    /// carries the moment of the search it came from"*). Nothing where the row
    /// carries no mark.
    pub searched_at: Option<SystemTime>,
}

/// **The folders the last search in this run could not read, each with the
/// moment of that search**, out of the dependency check's own run-held store
/// (`Dependency Upload & Remap` F7). **This reads no folder**; it asks what a
/// search already found.
fn the_folders_a_search_could_not_read() -> BTreeMap<Vec<u8>, Option<SystemTime>> {
    let Ok(held) = crate::surface::the_dependency_check::the_run_holds().lock() else {
        return BTreeMap::new();
    };
    the_folders_the_last_search_could_not_read(
        held.sittings.values().filter_map(|sitting| sitting.the_search.as_ref()),
    )
}

/// **The last search's verdict on each folder, and only *could not be read*
/// kept** — `Design/Settings § Search folders`: *"the mark is the last
/// search's, not the row's"*. Several checks in one run can each have searched
/// a folder; the one with the latest moment speaks for it, so a folder a later
/// search read whole carries no mark. A search still running has no verdict
/// yet and says nothing.
fn the_folders_the_last_search_could_not_read<'a>(
    searches: impl Iterator<Item = &'a crate::surface::the_dependency_check::TheSearchInHand>,
) -> BTreeMap<Vec<u8>, Option<SystemTime>> {
    let mut last: BTreeMap<Vec<u8>, (Option<SystemTime>, FolderVerdict)> = BTreeMap::new();
    for search in searches {
        for (folder, verdict) in &search.folders {
            let Some(verdict) = verdict else { continue };
            let later = match last.get(folder) {
                Some((held_at, _)) => search.searched_at >= *held_at,
                None => true,
            };
            if later {
                last.insert(folder.clone(), (search.searched_at, *verdict));
            }
        }
    }
    last.into_iter()
        .filter(|(_, (_, verdict))| *verdict == FolderVerdict::CouldNotBeRead)
        .map(|(folder, (at, _))| (folder, at))
        .collect()
}

fn the_list_as_it_stands(
    read: rusqlite::Result<SearchedFolderList>,
    unreadable: &BTreeMap<Vec<u8>, Option<SystemTime>>,
) -> TheList {
    match read {
        Ok(list) => TheList::Read(
            list.folders
                .into_iter()
                .map(|folder| AListedFolder {
                    could_not_be_read: unreadable.contains_key(&folder),
                    searched_at: unreadable.get(&folder).copied().flatten(),
                    folder,
                })
                .collect(),
        ),
        Err(_) => TheList::ItWouldNotRead,
    }
}

/// **What the list says, with every path by the verbatim route** (X7, X9) and
/// every word beside it as prose.
pub fn the_search_folders_say(says: &WhatTheListSays) -> Event {
    let mut event = Event::about(Subject::TheRun, THE_SEARCH_FOLDERS);
    for (name, said) in [
        ("heading", words::SEARCH_FOLDERS),
        ("aFolderCoversEverything", words::A_FOLDER_COVERS_EVERYTHING),
        ("chooseTheFolderHoldingAll", words::CHOOSE_THE_FOLDER_HOLDING_ALL),
        ("addAFolder", words::ADD_A_FOLDER),
        ("dragOrUseTheArrows", words::DRAG_OR_USE_THE_ARROWS),
        ("theFirstFolderHoldingAMatch", words::THE_FIRST_FOLDER_HOLDING_A_MATCH),
        ("whereOneFolderHoldsTwo", words::WHERE_ONE_FOLDER_HOLDS_TWO),
        ("moveUp", words::MOVE_UP),
        ("moveDown", words::MOVE_DOWN),
        ("remove", words::REMOVE),
        ("couldNotBeRead", words::COULD_NOT_BE_READ),
        ("at", words::AT),
    ] {
        event = prose(event, name, said);
    }
    match &says.the_list {
        TheList::ItWouldNotRead => {
            event = prose(event, "theListWouldNotRead", words::THE_LIST_WOULD_NOT_READ);
        }
        TheList::Read(rows) => {
            event = value(event, "folderCount", rows.len().to_string());
            for (at, row) in rows.iter().enumerate() {
                event = value(
                    event,
                    a_name("folder", at, ""),
                    String::from_utf8_lossy(&row.folder),
                );
                if row.could_not_be_read {
                    event = value(event, a_name("folder", at, "Unreadable"), "yes");
                    // **`Dependency Upload & Remap` F1a: the moment of the
                    // search the mark came from**, as
                    // seconds for the page to write in the user's own clock.
                    if let Some(searched_at) = row.searched_at {
                        event = value(
                            event,
                            a_name("folder", at, "SearchedAt"),
                            seconds_since_the_epoch(searched_at),
                        );
                    }
                }
            }
        }
    }
    if let Some(at) = says.already_on_it {
        event = value(event, "alreadyOnIt", at.to_string());
    }
    if says.the_change_did_not_save {
        event = prose(event, "theChangeDidNotSave", words::THE_CHANGE_DID_NOT_SAVE);
    }
    event
}

/// **The one door every list act goes through**: the change is written, then
/// the list is read back and posted (P7). A change that did not land posts the
/// list as it still stands with the line beside it.
fn the_list_is_changed(
    started: &Started,
    queue: &TheOneQueue,
    change: impl FnOnce(&SearchedFolderList) -> Result<Option<SearchedFolderList>, usize>,
) {
    let Some(record) = started.record.as_ref() else {
        return;
    };
    let connection = record.connection();
    let _ = queue.write_then_post::<WhatTheListSays, std::convert::Infallible>(
        || {
            let mut already_on_it = None;
            let mut did_not_save = false;
            match the_list::read(connection) {
                Ok(list) => match change(&list) {
                    Ok(Some(changed)) => {
                        did_not_save = the_list::write(connection, &changed).is_err();
                    }
                    Ok(None) => {}
                    Err(at) => already_on_it = Some(at),
                },
                Err(_) => did_not_save = true,
            }
            Ok(WhatTheListSays {
                the_list: the_list_as_it_stands(
                    the_list::read(connection),
                    &the_folders_a_search_could_not_read(),
                ),
                already_on_it,
                the_change_did_not_save: did_not_save,
            })
        },
        the_search_folders_say,
    );
}

fn the_list_is_posted(started: &Started, queue: &TheOneQueue) {
    let Some(record) = started.record.as_ref() else {
        return;
    };
    let connection = record.connection();
    let _ = queue.read_then_post::<WhatTheListSays, std::convert::Infallible>(
        || {
            Ok(WhatTheListSays {
                the_list: the_list_as_it_stands(
                    the_list::read(connection),
                    &the_folders_a_search_could_not_read(),
                ),
                already_on_it: None,
                the_change_did_not_save: false,
            })
        },
        the_search_folders_say,
    );
}

// ════════════════════════════════════════════════════════════════════════════
// What is actually running beside each cap — `Design/Settings § Transfers`
// ════════════════════════════════════════════════════════════════════════════

/// **The reading in each direction, as Transfers draws it**, upload then
/// download; nothing where that direction has no channel.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheSpeedsBesideTheCaps {
    pub up: Option<String>,
    pub down: Option<String>,
}

/// **The reading, from the run's own admission point and nowhere else** —
/// `Desktop app and delivery` R15 has it taken *"at the same accounting point
/// as the enforcement"*, and the pair is
/// [`crate::surface::transfers::the_live_readings`], which Transfers and its
/// title-bar figure already draw from. **Not worked out a second time here.**
pub fn the_speeds_now(transfers: &TheRunsTransfers) -> TheSpeedsBesideTheCaps {
    let (up, down) = crate::surface::transfers::the_live_readings(transfers);
    TheSpeedsBesideTheCaps { up, down }
}

/// **What the two caps have beside them.** The words are prose and each
/// figure, with its unit, is a value (X7); a direction with no reading sends
/// no value, and the page draws nothing beside that cap.
pub fn the_speeds_say(speeds: &TheSpeedsBesideTheCaps) -> Event {
    let mut event = prose(Event::about(Subject::TheRun, THE_SPEEDS), "runningAt", words::RUNNING_AT);
    if let Some(up) = &speeds.up {
        event = value(event, "uploadSpeed", up.clone());
    }
    if let Some(down) = &speeds.down {
        event = value(event, "downloadSpeed", down.clone());
    }
    event
}

fn the_speeds_are_posted(queue: &TheOneQueue, transfers: &TheRunsTransfers) {
    let speeds = the_speeds_now(transfers);
    let _ = queue.read_then_post::<TheSpeedsBesideTheCaps, std::convert::Infallible>(
        || Ok(speeds),
        the_speeds_say,
    );
}

fn seconds_since_the_epoch(at: SystemTime) -> String {
    at.duration_since(SystemTime::UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or(0)
        .to_string()
}

// ════════════════════════════════════════════════════════════════════════════
// The commands
// ════════════════════════════════════════════════════════════════════════════

/// **The page asks for the screen.** Four events answer: the screen's words,
/// the readings beside the caps, the list and the Telegram section (P6). The restore control's count is
/// `said_once`'s own command, which the page asks beside this one.
#[tauri::command]
pub fn the_settings_screen(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, Arc<TheOneQueue>>,
    transfers: State<'_, Arc<TheRunsTransfers>>,
) {
    the_speeds_are_posted(&queue, &transfers);
    let Ok(started) = started.lock() else {
        return;
    };
    // **`Desktop app and delivery` §8's route, taken fresh** (W3: nothing caches
    // it), beside the seven settings as the record holds them. A route that
    // names nothing proposes nothing (W5), and the section says what
    // `Download` H3 does then.
    let _ = queue.read_then_post::<WhatTheScreenHolds, std::convert::Infallible>(
        || Ok(what_the_screen_holds(started.record.as_ref())),
        the_screen_says,
    );
    the_list_is_posted(&started, &queue);
    crate::surface::telegram::the_section_is_posted(&started, &queue, None);
}

/// **The page asks for the readings beside the caps again** — when Transfers
/// is told a transfer moved, and only while Settings is showing. The rest of
/// the screen is not drawn again, so a cap being typed is left alone.
#[tauri::command]
pub fn the_speeds_in_settings(
    queue: State<'_, Arc<TheOneQueue>>,
    transfers: State<'_, Arc<TheRunsTransfers>>,
) {
    the_speeds_are_posted(&queue, &transfers);
}

/// **Add a folder…** — the system's own picker is the whole of it (`§ What this
/// document left open`, second bullet), and nothing is held while it is up.
///
/// **`async`, because the system's folder panel is put up here** and a plain
/// command runs on the drawing thread — see [`crate::picker::asked`].
#[tauri::command]
pub async fn a_search_folder_is_added<R: Runtime>(app: AppHandle<R>) {
    let picked = picker::asked(
        &app,
        Asking {
            what: WhatMayBeChosen::Folders,
            how_many: HowMany::One,
            starting_at: None,
            title: Prose::new(words::ADD_A_SEARCH_FOLDER),
        },
    )
    .await;
    // A user who closed the panel chose nothing, which is an ordinary answer.
    let Picked::TheUserChose(chosen) = picked else {
        return;
    };
    let Some(folder) = chosen
        .first()
        .map(|one| one.as_the_user_gave_it().as_os_str().as_encoded_bytes().to_vec())
    else {
        return;
    };
    let (Some(started), Some(queue)) = (
        app.try_state::<Mutex<Started>>(),
        app.try_state::<Arc<TheOneQueue>>(),
    ) else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    the_list_is_changed(&started, &queue, |list| the_list_with(list, folder).map(Some));
}

/// **A row moved** — the up and down controls and a finished drag.
#[tauri::command]
pub fn a_search_folder_is_moved(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, Arc<TheOneQueue>>,
    from: u32,
    to: u32,
) {
    let Ok(started) = started.lock() else {
        return;
    };
    the_list_is_changed(&started, &queue, |list| {
        Ok(the_list_moved(list, from as usize, to as usize))
    });
}

/// **A row removed.**
#[tauri::command]
pub fn a_search_folder_is_removed(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, Arc<TheOneQueue>>,
    position: u32,
) {
    let Ok(started) = started.lock() else {
        return;
    };
    the_list_is_changed(&started, &queue, |list| Ok(the_list_without(list, position as usize)));
}

/// **Choose…** beside the download folder — `Download` H2's *"the folder the
/// user pointed at instead"*. The system's own picker is the whole of it, and
/// nothing is held while it is up.
///
/// **`async`, because the system's folder panel is put up here** and a plain
/// command runs on the drawing thread — see [`crate::picker::asked`].
#[tauri::command]
pub async fn the_download_folder_is_chosen<R: Runtime>(app: AppHandle<R>) {
    let picked = picker::asked(
        &app,
        Asking {
            what: WhatMayBeChosen::Folders,
            how_many: HowMany::One,
            starting_at: None,
            title: Prose::new(words::CHOOSE_A_DOWNLOAD_FOLDER),
        },
    )
    .await;
    let Picked::TheUserChose(chosen) = picked else {
        return;
    };
    let Some(folder) = chosen
        .first()
        .map(|one| one.as_the_user_gave_it().as_os_str().as_encoded_bytes().to_vec())
    else {
        return;
    };
    let (Some(started), Some(queue)) = (
        app.try_state::<Mutex<Started>>(),
        app.try_state::<Arc<TheOneQueue>>(),
    ) else {
        return;
    };
    let notifications = app.try_state::<Arc<TheNotificationService>>();
    let Ok(started) = started.lock() else {
        return;
    };
    a_setting_is_written(
        started.record.as_ref(),
        &queue,
        notifications.as_deref().map(|service| service.as_ref()),
        THE_FOLDERS_ANSWER,
        None,
        |connection| {
            the_store::the_download_folder_was_chosen(connection, &folder)
                .map_err(|_| words::THE_SETTING_DID_NOT_SAVE)
        },
    );
}

/// **A cap was typed**, in MB/s, and is kept in bytes per second (R2, R3).
///
/// **And this is where the saved cap reaches the budget it is a cap on.** R3
/// splits the durable setting from the object in process memory, so the two
/// meet at exactly two moments: the application's start, and here. Until step
/// 10b piece 7 there was no second moment and no budget to reach — a cap the
/// user set was saved, drawn back on this screen and on Transfers, and bound
/// nothing at all.
///
/// **The caps are read back out of the record rather than taken from what was
/// typed**, so a write the record refused leaves the budgets exactly where
/// they were and the screen's own answer says what happened. **`Server
/// protocol and transfers` R8** does the rest: a cap changed while transfers
/// are in flight takes effect on each running channel at that channel's next
/// accounting interval, nothing is restarted, re-queued or closed for it.
#[tauri::command]
pub fn a_speed_cap_is_set(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, Arc<TheOneQueue>>,
    notifications: State<'_, Arc<TheNotificationService>>,
    transfers: State<'_, Arc<TheRunsTransfers>>,
    direction: TheDirection,
    typed: String,
) {
    let Ok(started) = started.lock() else {
        return;
    };
    a_setting_is_written(
        started.record.as_ref(),
        &queue,
        Some(&notifications),
        THE_CAPS_ANSWER,
        None,
        |connection| a_cap_is_written(connection, direction, &typed),
    );
    transfers.the_caps_are_read_from(started.record.as_ref());
}

fn a_cap_is_written(
    connection: &rusqlite::Connection,
    direction: TheDirection,
    typed: &str,
) -> Result<(), &'static str> {
    let cap = the_cap_typed(typed).map_err(|()| words::A_CAP_IS_A_SPEED_ABOVE_NOUGHT)?;
    let direction = match direction {
        TheDirection::Upload => Direction::Up,
        TheDirection::Download => Direction::Down,
    };
    the_store::a_cap_was_set(connection, direction, cap).map_err(|_| words::THE_SETTING_DID_NOT_SAVE)
}

/// **Which holder a switch's answer is drawn in**, by where it was pressed.
pub fn the_holder_for_a_switch(pressed_in: TheSwitchWasPressedIn) -> &'static str {
    match pressed_in {
        TheSwitchWasPressedIn::Settings => THE_SWITCHES_ANSWER,
        TheSwitchWasPressedIn::Telegram => THE_TELEGRAM_IDLE_ANSWER,
    }
}

/// **Whether a switch is told to the servers** — `Telegram` V2: the idle
/// message's alone, the one of the four about a server rather than this
/// computer.
fn servers_are_told_of(kind: Kind) -> bool {
    kind == Kind::AMachineHasBeenIdle
}

/// **A notification switch was turned** — T8. `kind` is the stored name
/// [`crate::record::settings::the_stored_name_of`] spells, and a name that is
/// none of the four is refused before anything is touched (H34).
#[tauri::command]
pub fn a_notification_switch_is_turned<R: Runtime>(
    app: AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    queue: State<'_, Arc<TheOneQueue>>,
    notifications: State<'_, Arc<TheNotificationService>>,
    kind: String,
    on: bool,
    pressed_in: TheSwitchWasPressedIn,
) {
    let Some(kind) = the_store::the_kind_stored_as(&kind) else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    let mut saved = false;
    a_setting_is_written(
        started.record.as_ref(),
        &queue,
        Some(&notifications),
        the_holder_for_a_switch(pressed_in),
        Some(kind),
        |connection| {
            let written = the_store::a_switch_was_turned(connection, kind, on)
                .map_err(|_| words::THE_SETTING_DID_NOT_SAVE);
            saved = written.is_ok();
            written
        },
    );
    drop(started);
    // **The live Telegram pass, fault 11, carried to the idle switch**
    // (`Telegram` L2 as changed 2026-10-31, V2): it reaches every server
    // connected now, at once, and the other three switches no server at all.
    if saved && servers_are_told_of(kind) {
        crate::setting_up::the_tellings_are_delivered_on_every_live_connection(&app);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::fs;
    use std::path::Path;

    fn the_working_half() -> &'static str {
        include_str!("settings.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half")
    }

    /// **The live Telegram pass, fault 11, carried to the idle switch: it
    /// reaches every server connected now, at once** — `Telegram` L2 as changed
    /// 2026-10-31, and V2's switch travels by the told-record. **Only the idle
    /// message's switch is told to a server**; the other three are this
    /// computer's own. Told only once the switch has saved, after the record's
    /// lock is given back.
    #[test]
    fn fault_11_the_idle_switch_is_told_to_every_server_connected_now() {
        assert!(servers_are_told_of(Kind::AMachineHasBeenIdle));
        for kind in Kind::ALL {
            if kind != Kind::AMachineHasBeenIdle {
                assert!(!servers_are_told_of(kind), "{kind:?} is this computer's own");
            }
        }
        let working = the_working_half();
        let at = working.find("pub fn a_notification_switch_is_turned<R: Runtime>(").expect("the command takes the application");
        let body = &working[at..at + working[at..].find("\n}\n").expect("it ends")];
        let telling = body
            .find("crate::setting_up::the_tellings_are_delivered_on_every_live_connection(&app)")
            .expect("the idle switch tells no connected server");
        let lock_let_go = body.find("drop(started)").expect("the lock is given back");
        assert!(lock_let_go < telling);
        assert!(body[..telling].contains("servers_are_told_of(kind)") && body[..telling].contains("saved"));
    }

    fn the_page() -> String {
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web/settings.js"))
            .expect("the page module is readable")
    }

    fn the_page_without_its_comments(page: &str) -> String {
        page.lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn the_mockup() -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the drawing is readable")
    }

    fn a_record(name: &str) -> (std::path::PathBuf, crate::record::LocalRecord) {
        let mut folder = std::env::temp_dir();
        folder.push(format!("smoozy-settings-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a folder to put the record in");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3"))
            .expect("the record opens");
        (folder, record)
    }

    fn a_list(folders: &[&str]) -> SearchedFolderList {
        SearchedFolderList {
            folders: folders.iter().map(|one| one.as_bytes().to_vec()).collect(),
        }
    }

    fn at(seconds: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(seconds)
    }

    // ── Step 10c's piece 17 of the 38: Settings' figures ────────────────────

    /// **Whether an event says this, under this name**, read through its own
    /// `Debug` — the fields are the event's own.
    fn says(event: &Event, name: &str, said: Said) -> bool {
        format!("{event:?}").contains(&format!("{name:?}: {said:?}"))
    }

    /// **`Design/Settings § Transfers`: each cap has what is actually running
    /// beside it.** A running download draws its reading beside the download
    /// cap and nothing beside the upload cap; with nothing moving, neither
    /// draws anything.
    #[test]
    fn a_running_download_draws_its_speed_beside_the_download_cap_and_none_draws_nothing() {
        let the_run = TheRunsTransfers::for_this_run();
        let idle = the_speeds_now(&the_run);
        assert_eq!(idle, TheSpeedsBesideTheCaps::default(), "nothing moving drew a reading");
        let said = format!("{:?}", the_speeds_say(&idle));
        assert!(!said.contains("uploadSpeed") && !said.contains("downloadSpeed"), "{said}");

        let mut channel = the_run
            .admit(Direction::Down, &["manual-download"])
            .expect("a channel is admitted");
        channel.draw_at(4_100_000, std::time::Instant::now());
        let moving = the_speeds_now(&the_run);
        assert_eq!(moving.up, None, "a download drew a reading beside the upload cap");
        // **The figure itself moves with the clock**
        // (`Server protocol and transfers` R12a's divisor is the
        // time since the channel began), so its shape is held here and its
        // source by the test below.
        let down = moving.down.clone().expect("a running download drew no reading");
        assert!(down.ends_with("\u{a0}MB/s"), "{down}");
        let event = the_speeds_say(&moving);
        assert_eq!(event.kind(), THE_SPEEDS);
        assert!(says(&event, "downloadSpeed", Said::Value(Verbatim::new(down))), "the figure travels as a value");
        assert!(says(&event, "runningAt", Said::Prose(Prose::new(words::RUNNING_AT))));
        assert!(!format!("{event:?}").contains("\"uploadSpeed\""));
    }

    /// **The rail's figure is read from where it is already worked out**: the
    /// pair Transfers draws, and never a second reading or conversion here.
    #[test]
    fn the_reading_beside_a_cap_is_the_one_transfers_draws() {
        let the_run = TheRunsTransfers::for_this_run();
        let mut channel = the_run
            .admit(Direction::Down, &["manual-download"])
            .expect("a channel is admitted");
        channel.draw_at(12_400_000, std::time::Instant::now());
        // **Both directions agree on whether there is a reading**; the figure
        // moves with the clock between the two askings, so the source is held
        // by reading the working half below.
        let theirs = crate::surface::transfers::the_live_readings(&the_run);
        let ours = the_speeds_now(&the_run);
        assert_eq!((ours.up.is_some(), ours.down.is_some()), (theirs.0.is_some(), theirs.1.is_some()));
        let working = the_working_half();
        assert!(working.contains("the_live_readings("), "Settings does not read Transfers' own pair");
        assert!(
            !working.contains(".the_reading("),
            "Settings takes the accounting point's reading itself — a second working-out"
        );
        let registered = include_str!("mod.rs");
        assert!(registered.contains("settings::the_speeds_in_settings"), "the command is not registered");
    }

    /// **The page draws the reading beside each cap from `THE_SPEEDS` alone,
    /// and asks again when Transfers is told a transfer moved** — and only
    /// while Settings is showing.
    #[test]
    fn the_page_draws_the_reading_and_asks_again_when_a_transfer_moves() {
        let page = the_page_without_its_comments(&the_page());
        for named in [
            "\"transfers:moved\"",
            "whenTold(TRANSFERS_MOVED",
            "whenTold(THE_SPEEDS",
            "said.uploadSpeed",
            "said.downloadSpeed",
            "said.runningAt",
            "\"the_speeds_in_settings\"",
            "itIsShowing",
        ] {
            assert!(page.contains(named), "the page does not hold {named}");
        }
        assert_eq!(crate::surface::transfers::TRANSFERS_MOVED, "transfers:moved");
        let speeds = page
            .split("whenTold(THE_SPEEDS")
            .nth(1)
            .and_then(|rest| rest.split("\n});").next())
            .expect("the speeds handler");
        assert!(speeds.contains("cap.speed.hidden = !reading"), "a direction with no reading still draws one");
    }

    /// **`Dependency Upload & Remap` F1a: where the verdict is drawn it carries
    /// the moment of the search it came from.**
    #[test]
    fn a_folder_that_could_not_be_read_says_when_it_was_searched() {
        let unreadable: BTreeMap<Vec<u8>, Option<SystemTime>> =
            [(b"/Volumes/Old".to_vec(), Some(at(1_759_472_400)))].into_iter().collect();
        let list = the_list_as_it_stands(Ok(a_list(&["/Users/a/Work", "/Volumes/Old"])), &unreadable);
        let event = the_search_folders_say(&WhatTheListSays {
            the_list: list,
            already_on_it: None,
            the_change_did_not_save: false,
        });
        assert!(says(&event, "folder1Unreadable", Said::Value(Verbatim::new("yes"))));
        assert!(says(&event, "folder1SearchedAt", Said::Value(Verbatim::new("1759472400"))), "the mark carries no moment of its search");
        assert!(!format!("{event:?}").contains("\"folder0SearchedAt\""), "a row with no mark carries a moment");
        assert!(says(&event, "at", Said::Prose(Prose::new(words::AT))));
        let page = the_page_without_its_comments(&the_page());
        for named in ["SearchedAt`]", "said.at", "\"fwhen\"", "theMomentReads(searchedAt)"] {
            assert!(page.contains(named), "the page does not hold {named}");
        }
    }

    /// **The mark is the last search's** — `Design/Settings § Search folders`.
    /// A later search that read the folder whole takes the mark away; a later
    /// one that could not read it puts its own moment on it; a search still
    /// running says nothing.
    #[test]
    fn the_mark_and_its_moment_are_the_last_searchs() {
        use crate::surface::the_dependency_check::TheSearchInHand;
        let search = |verdict, moment| TheSearchInHand {
            folders: vec![(b"/Volumes/Old".to_vec(), verdict)],
            the_matches_came_from: None,
            running: false,
            searched_at: Some(at(moment)),
        };
        let earlier_unread = search(Some(FolderVerdict::CouldNotBeRead), 100);
        let later_whole = search(Some(FolderVerdict::SearchedWhole), 200);
        let later_unread = search(Some(FolderVerdict::CouldNotBeRead), 300);
        let still_running = search(None, 400);

        let marks = the_folders_the_last_search_could_not_read([&earlier_unread, &later_whole].into_iter());
        assert!(marks.is_empty(), "a folder a later search read whole is still marked");

        let marks = the_folders_the_last_search_could_not_read(
            [&later_unread, &earlier_unread, &later_whole, &still_running].into_iter(),
        );
        assert_eq!(marks.get(b"/Volumes/Old".as_slice()), Some(&Some(at(300))));
    }

    /// **This screen's four and its Telegram section's fourteen are the
    /// drawing's retained `set` states.** *(The name
    /// counts sixteen, from before the bot step was drawn again on 2026-10-24;
    /// it is kept so the briefs and accounts that name this test still find it.)*
    #[test]
    fn the_sixteen_drawn_are_the_drawings_own() {
        let drawing = the_mockup();
        let list = drawing
            .split("<select id=\"state\"")
            .nth(1)
            .and_then(|rest| rest.split("</select>").next())
            .expect("the drawing has a state list");
        let mut drawn: Vec<&str> = Vec::new();
        for (at, _) in list.match_indices("<option value=\"set") {
            let rest = &list[at + "<option value=\"".len()..];
            let one = &rest[..rest.find('"').expect("the value closes")];
            if one == "set" || one.starts_with("set-") {
                drawn.push(one);
            }
        }
        let mut ours: Vec<&str> = THE_STATES_THIS_PIECE_DRAWS
            .iter()
            .chain(crate::surface::telegram::THE_STATES_THIS_PIECE_DRAWS.iter())
            .copied()
            .collect();
        drawn.sort_unstable();
        ours.sort_unstable();
        assert_eq!(drawn, ours, "the drawing's set states and the two pieces' lists have drifted");
    }

    /// **The Telegram section is in the list, in the drawing's place, and the
    /// line under the switches points at it** — `§ Warnings and notifications`.
    #[test]
    fn telegram_is_the_fourth_section_and_the_line_points_at_it() {
        let said = format!(
            "{:?}",
            the_screen_says(&WhatTheScreenHolds { the_proposal: None, the_settings: None, the_answer: None })
        );
        assert!(said.contains(&format!("{:?}", Prose::new(words::TELEGRAM_WORKS_WHEN_IT_IS_NOT))));
        let page = the_page_without_its_comments(&the_page());
        assert!(page.contains(
            "[TRANSFERS_SECTION, SEARCH_FOLDERS_SECTION, WARNINGS_SECTION, TELEGRAM_SECTION]"
        ));
        assert!(page.contains("\"s-tg\""));
    }

    #[test]
    fn the_page_and_this_file_name_the_same_things() {
        let page = the_page();
        for named in [
            THE_SCREEN,
            THE_SEARCH_FOLDERS,
            crate::surface::said_once::THE_WARNINGS_SWITCHED_OFF,
            "the_settings_screen",
            "a_search_folder_is_added",
            "a_search_folder_is_moved",
            "a_search_folder_is_removed",
            "the_download_folder_is_chosen",
            "a_speed_cap_is_set",
            "a_notification_switch_is_turned",
            "the_speeds_in_settings",
            THE_SPEEDS,
            "the_warnings_switched_off",
            "every_switched_off_warning_is_restored",
        ] {
            assert!(page.contains(named), "the page does not name {named}");
        }
        for holder in [THE_FOLDERS_ANSWER, THE_CAPS_ANSWER, THE_SWITCHES_ANSWER] {
            assert!(page.contains(&format!("\"{holder}\"")), "the page has no holder named {holder}");
        }
        for direction in ["upload", "download"] {
            assert!(page.contains(&format!("\"{direction}\"")), "the page never hands back {direction}");
            let parsed: Result<TheDirection, serde::de::value::Error> = serde::Deserialize::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(direction),
            );
            assert!(parsed.is_ok(), "{direction} is not a closed direction");
        }
        for kind in Kind::ALL {
            let field = the_switch_field_of(kind);
            assert!(page.contains(&format!("\"{}\"", field.trim_end_matches("IsOn"))), "the page never reads {field}");
        }
    }

    /// **`§ What this document left open`: a folder already on the list flashes
    /// its row and is not added; a folder inside one is added.**
    #[test]
    fn a_folder_is_added_once_and_one_inside_it_is_added_too() {
        let list = a_list(&["/Users/a/Work", "/Volumes/Archive"]);
        assert_eq!(the_list_with(&list, b"/Volumes/Archive".to_vec()), Err(1));
        let wider = the_list_with(&list, b"/Users/a/Work/Current".to_vec()).expect("a narrower folder is added");
        assert_eq!(wider, a_list(&["/Users/a/Work", "/Volumes/Archive", "/Users/a/Work/Current"]));
    }

    #[test]
    fn a_move_changes_the_order_and_nothing_else() {
        let list = a_list(&["one", "two", "three", "four"]);
        assert_eq!(the_list_moved(&list, 3, 0), Some(a_list(&["four", "one", "two", "three"])));
        assert_eq!(the_list_moved(&list, 0, 2), Some(a_list(&["two", "three", "one", "four"])));
        assert_eq!(the_list_moved(&list, 1, 1), Some(list.clone()));
        assert_eq!(the_list_moved(&list, 4, 0), None, "a place outside the list is refused");
        assert_eq!(the_list_moved(&list, 0, 4), None);
        assert_eq!(the_list_without(&list, 2), Some(a_list(&["one", "two", "four"])));
        assert_eq!(the_list_without(&list, 4), None);
    }

    /// **The keeper writes the whole list in the user's order and reads it
    /// back in that order**, with positions counted from one and no gap.
    #[test]
    fn the_list_is_written_and_read_back_in_the_users_order() {
        let (folder, record) = a_record("round-trip");
        let connection = record.connection();
        assert_eq!(the_list::read(connection).expect("it reads"), SearchedFolderList::default());
        let list = a_list(&["/c", "/a", "/b"]);
        the_list::write(connection, &list).expect("it writes");
        assert_eq!(the_list::read(connection).expect("it reads"), list);
        let shorter = the_list_without(&list, 0).expect("it removes");
        the_list::write(connection, &shorter).expect("it writes");
        let positions: Vec<i64> = connection
            .prepare("SELECT position FROM dependency_searched_folder ORDER BY position")
            .expect("it prepares")
            .query_map([], |row| row.get(0))
            .expect("it queries")
            .collect::<rusqlite::Result<_>>()
            .expect("it reads");
        assert_eq!(positions, vec![1, 2]);
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    /// **L8a: a list that would not read is said, never drawn empty.**
    #[test]
    fn a_list_that_would_not_read_is_said_rather_than_drawn_empty() {
        let unread = the_list_as_it_stands(Err(rusqlite::Error::InvalidQuery), &BTreeMap::new());
        assert_eq!(unread, TheList::ItWouldNotRead);
        let said = format!(
            "{:?}",
            the_search_folders_say(&WhatTheListSays {
                the_list: unread,
                already_on_it: None,
                the_change_did_not_save: false,
            })
        );
        assert!(said.contains(&format!("{:?}", Prose::new(words::THE_LIST_WOULD_NOT_READ))));
        assert!(!said.contains("folderCount"), "an unread list was drawn as an empty one: {said}");
    }

    /// **X7: a path travels by the verbatim route and never inside a
    /// sentence**, and the mark is the last search's.
    #[test]
    fn a_path_and_its_mark_travel_beside_the_words() {
        let unreadable: BTreeMap<Vec<u8>, Option<SystemTime>> =
            [(b"/Volumes/Old Projects".to_vec(), None)].into_iter().collect();
        let list = the_list_as_it_stands(Ok(a_list(&["/Users/a/Work", "/Volumes/Old Projects"])), &unreadable);
        let event = the_search_folders_say(&WhatTheListSays {
            the_list: list,
            already_on_it: Some(0),
            the_change_did_not_save: true,
        });
        let said = format!("{event:?}");
        assert!(said.contains(&format!("{:?}", Verbatim::new("/Volumes/Old Projects"))));
        assert!(said.contains("folder1Unreadable"));
        assert!(!said.contains("folder0Unreadable"), "a folder no search failed on carries the mark");
        assert!(said.contains("alreadyOnIt"));
        assert!(said.contains(&format!("{:?}", Prose::new(words::THE_CHANGE_DID_NOT_SAVE))));
        assert!(said.contains("folderCount"), "a change that did not save took the list off: {said}");
    }

    /// **The dependency check's panel names this state in these words**, so
    /// the two stay one word.
    #[test]
    fn the_mark_is_the_dependency_checks_own_word() {
        let theirs = include_str!("the_dependency_check.rs");
        assert!(theirs.contains(&format!("\"{}\"", words::COULD_NOT_BE_READ)));
    }

    #[test]
    fn no_setting_answers_that_it_cannot_be_saved_any_more() {
        let page = the_page_without_its_comments(&the_page());
        assert!(!page.contains("a_setting_is_changed"));
        assert!(!page.contains("a-setting-is-not-saved"));
        assert!(!the_working_half().contains("cannot save this setting"));
    }

    fn the_defaults() -> TheSettings {
        TheSettings {
            the_chosen_download_folder: None,
            the_upload_cap: TheCap::NoCapIsSet,
            the_download_cap: TheCap::NoCapIsSet,
            the_switches: crate::record::settings::TheSwitches::nobody_has_turned_one(),
        }
    }

    /// **No folder at all, and the section says `Download` H3's answer rather
    /// than drawing an empty path.**
    #[test]
    fn with_no_folder_the_section_says_so() {
        let holds = WhatTheScreenHolds {
            the_proposal: None,
            the_settings: Some(the_defaults()),
            the_answer: None,
        };
        let said = format!("{:?}", the_screen_says(&holds));
        assert!(said.contains(&format!("{:?}", Prose::new(words::NO_FOLDER_IS_SET))));
        assert!(!said.contains("theDownloadFolder"));
    }

    /// **`Download` H2: the folder the user chose is drawn over the proposal,
    /// and the proposal is drawn where they chose none.**
    #[test]
    fn the_users_own_folder_is_drawn_over_the_proposal() {
        let mut holds = WhatTheScreenHolds {
            the_proposal: Some(b"/Users/a/Downloads".to_vec()),
            the_settings: Some(the_defaults()),
            the_answer: None,
        };
        let said = format!("{:?}", the_screen_says(&holds));
        assert!(said.contains(&format!("{:?}", Verbatim::new("/Users/a/Downloads"))));
        assert!(!said.contains("noFolderIsSet"));

        holds.the_settings.as_mut().unwrap().the_chosen_download_folder =
            Some(b"/Volumes/Work/Renders".to_vec());
        let said = format!("{:?}", the_screen_says(&holds));
        assert!(said.contains(&format!("{:?}", Verbatim::new("/Volumes/Work/Renders"))));
        assert!(!said.contains(&format!("{:?}", Verbatim::new("/Users/a/Downloads"))));
    }

    /// **The caps and the switches travel as values, and no cap travels as no
    /// value** (R8: an empty field).
    #[test]
    fn the_caps_and_the_switches_are_drawn_from_what_the_record_holds() {
        let mut settings = the_defaults();
        settings.the_download_cap = TheCap::Of(BytesPerSecond(12_500_000));
        settings.the_switches = settings.the_switches.with(Kind::AMachineHasBeenIdle, false);
        let said = format!(
            "{:?}",
            the_screen_says(&WhatTheScreenHolds {
                the_proposal: None,
                the_settings: Some(settings),
                the_answer: None,
            })
        );
        assert!(!said.contains("uploadCapIs"), "no cap was drawn as a figure: {said}");
        assert!(said.contains("downloadCapIs"));
        assert!(said.contains(&format!("{:?}", Verbatim::new("12.5"))));
        for kind in Kind::ALL {
            assert!(said.contains(the_switch_field_of(kind)));
        }
        assert_eq!(
            said.matches(&format!("{:?}", Verbatim::new("no"))).count(),
            1,
            "one switch is off and exactly one was drawn off: {said}"
        );
    }

    /// **L8a: a record that did not answer draws no setting at all** — not a
    /// cap, not a switch, and not the proposal in the place of the user's own
    /// folder — and says so.
    #[test]
    fn a_record_that_did_not_answer_draws_no_setting_and_no_default() {
        let said = format!(
            "{:?}",
            the_screen_says(&WhatTheScreenHolds {
                the_proposal: Some(b"/Users/a/Downloads".to_vec()),
                the_settings: None,
                the_answer: None,
            })
        );
        assert!(said.contains(&format!("{:?}", Prose::new(words::THE_SETTINGS_WOULD_NOT_READ))));
        for absent in ["theDownloadFolder", "noFolderIsSet", "uploadCapIs", "downloadCapIs", "IsOn"] {
            assert!(!said.contains(absent), "{absent} was drawn from a read that did not answer: {said}");
        }
    }

    /// **R2's one conversion, both ways.** What a user types comes back as they
    /// typed it, an empty field is no cap, and nothing that is not a speed above
    /// nought is taken.
    #[test]
    fn a_cap_is_typed_in_megabytes_and_kept_in_bytes() {
        for (typed, bytes) in [
            ("12.5", 12_500_000),
            ("40", 40_000_000),
            (" 0.25 ", 250_000),
            ("0.000001", 1),
            (".5", 500_000),
            ("1.", 1_000_000),
        ] {
            assert_eq!(the_cap_typed(typed), Ok(TheCap::Of(BytesPerSecond(bytes))), "{typed}");
        }
        for (bytes, shown) in [(12_500_000, "12.5"), (40_000_000, "40"), (250_000, "0.25"), (1, "0.000001")] {
            assert_eq!(megabytes_per_second(BytesPerSecond(bytes)), shown);
            assert_eq!(the_cap_typed(shown), Ok(TheCap::Of(BytesPerSecond(bytes))));
        }
        assert_eq!(the_cap_typed(""), Ok(TheCap::NoCapIsSet));
        assert_eq!(the_cap_typed("   "), Ok(TheCap::NoCapIsSet));
        for refused in ["0", "0.0", "-5", "abc", "1e3", "1,5", ".", "0.0000001", "99999999999999999999"] {
            assert_eq!(the_cap_typed(refused), Err(()), "{refused} was taken as a cap");
        }
    }

    /// A platform that counts what it is asked, so the service a change hands
    /// its switches to can be seen acting on them.
    #[derive(Default)]
    struct ThePostsMade(Mutex<Vec<Kind>>);

    impl crate::notifications::ThePlatform for Arc<ThePostsMade> {
        fn the_machinery(&self) -> crate::notifications::TheMachinery {
            crate::notifications::TheMachinery::ItIsInPlace
        }
        fn the_permission(&self) -> crate::notifications::permission::WhatTheSystemSays {
            crate::notifications::permission::WhatTheSystemSays::ItIsGranted
        }
        fn ask_for_permission_now(&self) {}
        fn post(&self, post: &crate::notifications::APost) {
            self.0.lock().unwrap().push(post.kind);
        }
        fn take_down(&self, _key: &crate::presentation::the_presentation_key::ThePresentationKey) {}
    }

    fn a_post_of(kind: Kind) -> crate::notifications::APost {
        crate::notifications::APost {
            key: crate::presentation::the_presentation_key::ThePresentationKey::ACondition(
                smoozy_state::identities::Identity::issue(
                    smoozy_state::identities::IdentityKind::Condition,
                )
                .expect("the desk has randomness"),
            ),
            kind,
            said: Prose::new("finished"),
            about: Verbatim::new("Kitchen"),
            body: Prose::new(""),
            on: Verbatim::new(""),
            picture: crate::notifications::picture::ThePicture::None(
                crate::notifications::picture::WhyThereIsNoPicture::ThisSystemTakesNone,
            ),
        }
    }

    /// **The whole of a command but the posting, over a real record**: a
    /// switch turned off is written, read back, drawn off, and handed to the
    /// notification service, which then attempts no post of that kind.
    #[test]
    fn a_switch_turned_off_is_saved_drawn_and_stops_that_kind_being_attempted() {
        let (folder, record) = a_record("switch");
        let system = Arc::new(ThePostsMade::default());
        let service = TheNotificationService::on(Box::new(system.clone()));
        service.the_switches_are(Some(crate::record::settings::TheSwitches::nobody_has_turned_one()));

        let holds = the_change_is_made(Some(&record), Some(&service), THE_SWITCHES_ANSWER, Some(Kind::AJobFinished), |c| {
            the_store::a_switch_was_turned(c, Kind::AJobFinished, false).map_err(|_| "no")
        });
        assert_eq!(holds.the_answer, None);
        assert!(!holds.the_settings.as_ref().unwrap().the_switches.attempts(Kind::AJobFinished));

        service.post(a_post_of(Kind::AJobFinished));
        service.post(a_post_of(Kind::AJobFailedOrWasGivenUp));
        assert_eq!(*system.0.lock().unwrap(), vec![Kind::AJobFailedOrWasGivenUp]);

        // **And it survives the record being closed and opened again**, which
        // is what *durable* means here.
        let path = record.path().to_path_buf();
        drop(record);
        let record = LocalRecord::open_at(&path).unwrap();
        assert!(!the_store::read(record.connection()).unwrap().the_switches.attempts(Kind::AJobFinished));
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    /// **A cap that is not a speed is refused, said beside the caps, and the
    /// record keeps the cap it had** — which is what the field is then drawn
    /// from.
    #[test]
    fn a_cap_that_is_not_a_speed_is_refused_and_the_saved_one_is_drawn() {
        let (folder, record) = a_record("cap");
        let holds = the_change_is_made(Some(&record), None, THE_CAPS_ANSWER, None, |c| {
            a_cap_is_written(c, TheDirection::Upload, "12.5")
        });
        assert_eq!(holds.the_answer, None);
        let holds = the_change_is_made(Some(&record), None, THE_CAPS_ANSWER, None, |c| {
            a_cap_is_written(c, TheDirection::Upload, "0")
        });
        assert_eq!(
            holds.the_answer,
            Some(TheAnswer { holder: THE_CAPS_ANSWER, about: None, sentence: words::A_CAP_IS_A_SPEED_ABOVE_NOUGHT })
        );
        assert_eq!(
            holds.the_settings.unwrap().the_upload_cap,
            TheCap::Of(BytesPerSecond(12_500_000)),
            "a refused cap moved the one the record held"
        );
        let holds = the_change_is_made(Some(&record), None, THE_CAPS_ANSWER, None, |c| {
            a_cap_is_written(c, TheDirection::Upload, "")
        });
        assert_eq!(holds.the_settings.unwrap().the_upload_cap, TheCap::NoCapIsSet);
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    /// **With no record there is nowhere to save**, and the change says so and
    /// hands the notification service nothing to attempt under.
    #[test]
    fn with_no_record_a_change_says_it_did_not_save() {
        let system = Arc::new(ThePostsMade::default());
        let service = TheNotificationService::on(Box::new(system.clone()));
        service.the_switches_are(Some(crate::record::settings::TheSwitches::nobody_has_turned_one()));
        let holds = the_change_is_made(None, Some(&service), THE_SWITCHES_ANSWER, Some(Kind::AJobFinished), |_| Ok(()));
        assert_eq!(
            holds.the_answer,
            Some(TheAnswer {
                holder: THE_SWITCHES_ANSWER,
                about: Some(Kind::AJobFinished),
                sentence: words::THE_SETTING_DID_NOT_SAVE
            })
        );
        assert_eq!(holds.the_settings, None);
        service.post(a_post_of(Kind::AJobFinished));
        assert!(system.0.lock().unwrap().is_empty());
    }

    /// **`web/said-once.js` no longer draws into the pane, and this screen
    /// calls its builder.**
    #[test]
    fn the_proof_surface_is_gone_and_this_screen_draws_the_control() {
        let said_once = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web/said-once.js"))
            .expect("the page module is readable");
        let working = the_page_without_its_comments(&said_once);
        assert!(!working.contains("getElementById(\"pane\")"), "said-once.js still draws into the pane");
        assert!(!working.contains("whenTold("), "said-once.js still listens for itself");
        let page = the_page_without_its_comments(&the_page());
        assert!(page.contains("theWarningsSwitchedOff({"), "settings.js does not call the builder");
    }

    /// **How a section is opened from elsewhere is one exported call, and the
    /// head of the page names it** — the link Transfers owes.
    #[test]
    fn a_section_is_opened_from_elsewhere_by_one_exported_call() {
        let page = the_page();
        let head: String = page.lines().take_while(|line| line.starts_with("//")).collect::<Vec<_>>().join("\n");
        assert!(head.contains("theSettingsSectionIsOpened(TRANSFERS_SECTION)"));
        assert!(page.contains("export function theSettingsSectionIsOpened(section)"));
        for section in ["TRANSFERS_SECTION", "SEARCH_FOLDERS_SECTION", "WARNINGS_SECTION", "TELEGRAM_SECTION"] {
            assert!(page.contains(&format!("export const {section}")), "{section} is not exported");
        }
    }

    /// **Nothing here raises a failure record**, so `WhoseSentenceItIs` is not
    /// reached.
    #[test]
    fn no_failure_record_is_raised_here() {
        let working = the_working_half();
        for raising in ["the_two_hooks", "failure_records", "a_condition_was_raised"] {
            assert!(!working.contains(&format!("{raising}::")), "this file reaches {raising}");
        }
    }

    /// **Must-do 13: under a minute the reading is words alone**, and never
    /// *0 minutes*; one of a unit takes the singular.
    #[test]
    fn a_switchs_answer_is_drawn_only_beside_the_switch_that_was_pressed() {
        let said = format!(
            "{:?}",
            the_screen_says(&WhatTheScreenHolds {
                the_proposal: None,
                the_settings: Some(the_defaults()),
                the_answer: Some(TheAnswer {
                    holder: the_holder_for_a_switch(TheSwitchWasPressedIn::Settings),
                    about: Some(Kind::AJobFinished),
                    sentence: words::THE_SETTING_DID_NOT_SAVE,
                }),
            })
        );
        assert!(said.contains("theAnswerAbout"));
        assert!(said.contains(the_store::the_stored_name_of(Kind::AJobFinished)));
        assert_ne!(
            the_holder_for_a_switch(TheSwitchWasPressedIn::Settings),
            the_holder_for_a_switch(TheSwitchWasPressedIn::Telegram)
        );
        let telegram = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web/telegram.js"))
            .expect("the page module is readable");
        assert!(telegram.contains(&format!("\"{THE_TELEGRAM_IDLE_ANSWER}\"")));
        assert!(telegram.contains("pressedIn: \"telegram\""));
        assert!(
            !telegram.contains("said.theAnswerFor === THE_SWITCHES_ANSWER"),
            "the Telegram section still draws an answer to a switch pressed in Settings"
        );
        let page = the_page_without_its_comments(&the_page());
        assert!(page.contains("pressedIn: \"settings\""));
        assert!(page.contains("said.theAnswerAbout === which"), "Settings draws a switch's answer beside every switch");
    }
}

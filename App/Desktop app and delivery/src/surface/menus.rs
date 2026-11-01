//! **The four menus' own end of the page and of the native bar** —
//! `Desktop app and delivery` §18.2, §18.3 and §18.5.
//!
//! [`crate::menus`] is the four menus and every rule about them. This file is
//! what joins them to the two arrangements: **the model the page draws on
//! Windows and Linux**, and **the one place a chosen item turns into an act**,
//! whichever of the two arrangements the choosing happened in.
//!
//! **One item, one act, one definition.** The native bar's menu event and the
//! page's own click both arrive at [`the_item_was_chosen`], so an act cannot
//! mean one thing on a Mac and another on Windows. That is the whole reason
//! this file exists rather than the two routes each doing their own.
//!
//! **A going is the application telling the page** — P6 — and it goes on the one
//! ordered queue like everything else. **Nothing here returns a module's outcome
//! from a command**, which is the second route P6 forbids.

use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use tauri::{AppHandle, Manager, Runtime, State};

use crate::events::{Event, Posted, Said, Subject, TheOneQueue};
use crate::menus::{self, Act, Focused, TheSystemsAct, Where};
use crate::typography::{Prose, Verbatim};
use crate::updating::checking::{TheReading, TheUpdateCheck};

/// **The user chose a menu item that goes somewhere.**
///
/// The kind is declared beside the service that posts it, which is the command
/// surface's own rule. The page's own module names the same string.
pub const THE_USER_CHOSE_A_GOING: &str = "menus:a-going";

/// **About's reading changed** — a check began or ended (Z23a, Z30). Posted
/// by the update check, `crate::updating::checking`, and drawn by the page
/// only where About is open: it never opens the box itself. The page's own
/// module names the same string.
pub const THE_ABOUTS_READING: &str = "menus:abouts-reading";

// ── What the page is given ──────────────────────────────────────────────────

/// One item, as the page draws it.
#[derive(serde::Serialize)]
pub struct DrawnItem {
    pub id: &'static str,
    /// **K8 and X5: prose, through the typography point**, even though no engine
    /// draws a menu label on the system that has a native bar.
    pub label: Prose,
    /// **The label beside the item, as this system spells it** — absent where
    /// the system publishes no convention for the act (K15). It is derived from
    /// the combination below rather than written a second time.
    pub key: Option<Prose>,
    /// **The same combination in the form the page's key handler reads** — K14:
    /// on Windows and Linux catching the keystroke is a key handler in the page,
    /// the shell publishing no route for it there.
    pub press: Option<DrawnPress>,
    /// **Where the page performs the act itself** — the Edit menu's four, which
    /// act on whatever has focus (K7). Absent for every other item, which the
    /// page hands back to the application.
    pub on_whatever_has_focus: Option<&'static str>,
}

/// A combination as the page reads a key press.
#[derive(serde::Serialize)]
pub struct DrawnPress {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub key: &'static str,
}

/// One menu, as the page draws it.
#[derive(serde::Serialize)]
pub struct DrawnMenu {
    pub id: &'static str,
    pub title: Prose,
    /// Items and the rules between them, in Design's order. A rule is an entry
    /// with no item.
    pub entries: Vec<DrawnEntry>,
}

#[derive(serde::Serialize)]
pub struct DrawnEntry {
    pub item: Option<DrawnItem>,
}

/// **What the page is told about the menus.**
#[derive(serde::Serialize)]
pub struct TheMenuBar {
    /// **False on macOS**, where the four go in the system's own bar and no bar
    /// is drawn in the window at all (K1, K9). The mockup's line saying so is
    /// the mockup's own and is not Smoozy's.
    pub drawn_by_the_application: bool,
    /// **The button's own accessible name**, for the one button the four sit
    /// behind on Windows and Linux (K10). Prose, like every other word the user
    /// reads.
    pub the_buttons_name: Prose,
    pub menus: Vec<DrawnMenu>,
}

/// **The four menus, shaped for whichever arrangement this system has.**
///
/// H34: a closed response type, no length and no enumeration taken from the
/// page, and nothing beneath the application-service surface anywhere near it.
#[tauri::command]
pub fn the_menu_bar() -> TheMenuBar {
    let system = std::env::consts::OS;
    TheMenuBar {
        drawn_by_the_application: menus::where_the_menus_go(system) == Where::DrawnByTheApplication,
        // **Design drew a bar rather than the button, so this word is ruled
        // here** under the standing delegation of 2026-08-31 and written into
        // the decisions record. *Menu* is the word the rest of the industry
        // uses, which is Rule 9.
        the_buttons_name: Prose::new("Menu"),
        menus: menus::the_four_menus()
            .into_iter()
            .map(|menu| DrawnMenu {
                id: menu.id,
                title: menu.title(),
                entries: menu
                    .entries
                    .iter()
                    // **An item this system does not draw leaves the list
                    // entirely** — `crate::menus::Item::is_drawn_on`, which
                    // carries the whole of why, and today the one such item is
                    // *Hide Smoozy* off macOS. It is dropped rather than drawn
                    // unavailable, the way an absent key is absent rather than
                    // invented (K15, K6).
                    .filter(|entry| match entry {
                        menus::Entry::ASeparator => true,
                        menus::Entry::AnItem(item) => item.is_drawn_on(system),
                    })
                    .map(|entry| DrawnEntry {
                        item: match entry {
                            menus::Entry::ASeparator => None,
                            menus::Entry::AnItem(item) => {
                                let combination = item.keys.on(system);
                                Some(DrawnItem {
                                    id: item.id,
                                    label: item.label(),
                                    key: combination.map(|combination| {
                                        Prose::new(combination.as_the_application_draws_it())
                                    }),
                                    press: combination.map(|combination| DrawnPress {
                                        ctrl: combination.ctrl,
                                        shift: combination.shift,
                                        alt: combination.alt,
                                        key: combination.key,
                                    }),
                                    on_whatever_has_focus: match item.act {
                                        Act::OnWhateverHasFocus(Focused::Cut) => Some("cut"),
                                        Act::OnWhateverHasFocus(Focused::Copy) => Some("copy"),
                                        Act::OnWhateverHasFocus(Focused::Paste) => Some("paste"),
                                        Act::OnWhateverHasFocus(Focused::SelectAll) => {
                                            Some("selectAll")
                                        }
                                        _ => None,
                                    },
                                })
                            }
                        },
                    })
                    .collect(),
            })
            .collect(),
    }
}

// ── What happens when one is chosen ─────────────────────────────────────────

/// What the caller is told, and it is about the ask rather than about any
/// module's outcome — P6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum TheChoice {
    /// The act was performed, or the going was posted.
    ItWasCarriedOut,
    /// **The four menus name no such item.** H34: the request is validated
    /// against the closed set before anything acts, and the closed set is the
    /// menus themselves rather than a list beside them.
    ThereIsNoSuchItem,
    /// The system refused the fixed Source code address request.
    ItCouldNotBeOpened,
}

/// **The page's own route in** — Windows and Linux, where the four menus are the
/// application's own drawing.
#[tauri::command]
pub fn a_menu_item_was_chosen<R: Runtime>(
    app: AppHandle<R>,
    queue: State<'_, Arc<TheOneQueue>>,
    item: String,
) -> TheChoice {
    the_item_was_chosen(&app, &queue, &item)
}

/// **The one place a chosen item turns into an act**, whichever arrangement the
/// choosing happened in. The native bar's menu event arrives here too.
pub fn the_item_was_chosen<R: Runtime>(
    app: &AppHandle<R>,
    queue: &TheOneQueue,
    id: &str,
) -> TheChoice {
    if id == "about.source" {
        return the_source_is_asked_with(crate::the_address_opener::open);
    }
    let Some(item) = menus::the_item(id) else {
        return TheChoice::ThereIsNoSuchItem;
    };
    // **An item this system does not draw is an item this system does not
    // perform.** The page can only name what it was handed, so on the two
    // systems that draw their own menus there is genuinely no such item — and a
    // command is reachable without the page's own list, which is why the refusal
    // is here as well as in the list above. `crate::menus::Item::is_drawn_on`
    // carries the reasoning; today the one such item is *Hide Smoozy* off macOS,
    // where hiding the one window Smoozy has leaves nothing that brings it back.
    if !item.is_drawn_on(std::env::consts::OS) {
        return TheChoice::ThereIsNoSuchItem;
    }
    match item.act {
        Act::TheSystems(act) => {
            the_system_acts(app, act);
            TheChoice::ItWasCarriedOut
        }
        // **K7: the focused thing decides, and on the two systems that draw
        // their own menus the focused thing is in the page.** So the page
        // performs these itself and they never arrive here; an id that does
        // arrive is answered rather than ignored, because silence would be the
        // one outcome nobody could see.
        Act::OnWhateverHasFocus(_) => TheChoice::ItWasCarriedOut,
        Act::AGoing => {
            // ***Check for updates…* is Z26's act reached a second way**: the
            // press goes to the run's one check, which starts it or joins the
            // one under way (Z27), and About opens on the reading the press
            // left — *Checking…* from the press (Z30).
            if id == THE_CHECK_ITEM {
                if let Some(check) = app.try_state::<Arc<TheUpdateCheck>>() {
                    check.the_user_asks();
                }
            }
            let _ = the_going_with(queue, id, &|| what_about_reads_here(app));
            TheChoice::ItWasCarriedOut
        }
    }
}

// The source action uses the existing command, with a fixed name and no URL.
fn the_source_is_asked_with(
    ask: impl FnOnce(crate::the_address_opener::WhichAddress) -> crate::the_address_opener::Opened,
) -> TheChoice {
    use crate::the_address_opener::{Opened, WhichAddress};
    match ask(WhichAddress::Source) {
        Opened::ItWasAsked => TheChoice::ItWasCarriedOut,
        Opened::ItCouldNotBeOpened => TheChoice::ItCouldNotBeOpened,
    }
}

/// The menu item that asks for a check.
const THE_CHECK_ITEM: &str = "smoozy.check-for-updates";

/// **Z32: *Install now*, pressed in About's held form.** It goes to the run's
/// one check, which hands it to this system's route on its own sequence (the
/// Windows one from step 11's Windows update piece): Z31's held
/// publication, now, and every check bound unchanged. **A press while About
/// reads anything but the held form does nothing**, the button being drawn on
/// that form alone. Nothing is answered: what the press comes to is the
/// application restarting, or About's next reading (P6).
///
/// **On Linux the same registered command carries the update box's answers**
/// (step 11's Linux piece): `linux` names the offered version and what was
/// pressed, and the run's one check refuses an answer for any other version.
///
/// **And the rollback box's** (step 11's D6b): in a launch that shows that box
/// and nothing else, *Reinstall* and *Quit* arrive here too, so no command is
/// added to the one registration list.
#[tauri::command]
pub fn the_held_update_is_installed_anyway<R: Runtime>(
    app: AppHandle<R>,
    linux: Option<LinuxBoxChoice>,
) {
    if let Some(the_box) =
        app.try_state::<Arc<crate::updating::the_linux_package::TheRollbackBox>>()
    {
        match linux.as_ref().map(|choice| choice.action.as_str()) {
            Some("reinstall") => the_box.reinstall_is_pressed(&app),
            // **Quit leaves the rollback owed**: the next start shows the box
            // again. Nothing is written on the way out; no record is open.
            Some("quit") => app.exit(0),
            _ => {}
        }
        return;
    }
    if let Some(check) = app.try_state::<Arc<TheUpdateCheck>>() {
        if let Some(choice) = linux {
            if !cfg!(target_os = "linux") {
                return;
            }
            match choice.action.as_str() {
                // About's *Install…*: the box for the version already found,
                // with no second check (Z26's trigger list stays three).
                "offer" => {
                    check.the_user_opens_the_linux_box();
                }
                "install" => {
                    check.answer_linux_offer(&choice.version, true);
                }
                "decline" => {
                    check.answer_linux_offer(&choice.version, false);
                }
                _ => {}
            }
        } else {
            check.the_user_installs_anyway();
        }
    }
}

/// Uses the existing registered update command; no second install route.
#[derive(serde::Deserialize)]
pub struct LinuxBoxChoice {
    pub version: String,
    pub action: String,
}

pub const THE_LINUX_UPDATE_OFFER: &str = "menus:linux-update";

/// The drawing's Linux update box. Every word passes through Rust's prose
/// route, and the release number remains a separately copyable value.
pub fn the_linux_offer(version: &str) -> Event {
    let prose = |s: &str| Said::Prose(Prose::new(s));
    Event::about(Subject::TheRun, THE_LINUX_UPDATE_OFFER)
        .saying("beforeVersion", prose("Smoozy"))
        .saying("version", Said::Value(Verbatim::new(version)))
        .saying("afterVersion", prose("is ready to install"))
        .saying("password", prose("Your system asks for your computer password to install."))
        .saying("declining", prose("Not now keeps this version until the next release."))
        .saying("notNow", prose("Not now"))
        .saying("install", prose("Install"))
}

/// **The rollback box** (step 11's D6b, Design *Updating*): what a launch
/// shows, and nothing else, where an update could not start and the previous
/// version is still to be put back. Its own kind, so the update box's page
/// code is not asked to tell the two apart.
pub const THE_LINUX_ROLLBACK_BOX: &str = "menus:linux-rollback";

/// **The box's words.** `asks` is whether the system will be asked for the
/// password — not where only the restore and cleanup are left; `failed` is
/// whether the last attempt failed, which draws *Reinstall failed.* in the
/// failure shape. The version is a separately copyable value.
pub fn the_rollback_box(version: &str, asks: bool, failed: bool) -> Event {
    let prose = |s: &str| Said::Prose(Prose::new(s));
    let mut event = Event::about(Subject::TheRun, THE_LINUX_ROLLBACK_BOX)
        .saying("beforeVersion", prose("Smoozy"))
        .saying("version", Said::Value(Verbatim::new(version)))
        .saying("afterVersion", prose("could not start"))
        .saying("until", prose("Reinstall the previous version to open Smoozy."))
        .saying("quit", prose("Quit"))
        .saying("reinstall", prose("Reinstall"));
    if asks {
        event = event.saying(
            "password",
            prose("Your system asks for your computer password to reinstall."),
        );
    }
    if failed {
        event = event.saying("failed", prose("Reinstall failed."));
    }
    event
}

/// **What About reads, through the shell's own managed state**: the run's
/// check reading with its number, and the last completed check's moment from
/// the record (`State, identity and recovery` I4).
fn what_about_reads_here<R: Runtime>(app: &AppHandle<R>) -> WhatAboutReads {
    let (reading, number) = app
        .try_state::<Arc<TheUpdateCheck>>()
        .map(|check| check.the_reading())
        .unwrap_or((TheReading::NoneYet, 0));
    let last_checked = app
        .try_state::<Mutex<crate::start::Started>>()
        .and_then(|started| {
            let started = started.lock().ok()?;
            let record = started.record.as_ref()?;
            crate::record::update_trust::the_last_completed_check(record.connection())
                .ok()
                .flatten()
        });
    WhatAboutReads {
        reading,
        number,
        last_checked,
    }
}

/// **What About is drawn from**: the reading, the number that orders it
/// against another reading, and the stored moment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WhatAboutReads {
    pub reading: TheReading,
    pub number: u64,
    pub last_checked: Option<SystemTime>,
}

/// **The going, on the one ordered queue.**
///
/// **Why this is a read rather than a write, said plainly.** P7 forbids an event
/// carrying state that has not already been written durably, **and a going
/// carries no state at all** — it says which of a fixed set of items the user
/// chose, and that set is the application's own and is compiled into it. The
/// door taken is therefore the reading one, and **the thing read is the menus'
/// own definition**: an identifier the four menus do not name posts nothing,
/// which is what makes the door mean something here rather than being a form
/// filled in.
fn the_going(queue: &TheOneQueue, id: &str) -> Result<Posted, ()> {
    the_going_with(queue, id, &|| WhatAboutReads {
        reading: TheReading::NoneYet,
        number: 0,
        last_checked: None,
    })
}

/// **The going, with what About reads** — read inside the read door, so the
/// box opens on what the record and the run's check held at that moment (P7).
fn the_going_with(
    queue: &TheOneQueue,
    id: &str,
    about: &dyn Fn() -> WhatAboutReads,
) -> Result<Posted, ()> {
    // ***Report a problem…* opens the report box, with nothing handed over** —
    // `Reporting a problem` X2 and `Reporting a problem` X3. The box is the
    // drawing of this going, so the going is the box's own event and not a
    // second one beside it. The proposal is §8's, taken fresh (W3).
    if id == crate::surface::reporting::THE_MENU_ITEM && menus::the_item(id).is_some() {
        return crate::surface::reporting::the_box_opens(
            &crate::surface::reporting::THE_BOX,
            queue,
            crate::surface::reporting::TheOpening::TheMenu,
            crate::platform::downloads_folder::the_folder_the_system_proposes(),
        );
    }
    queue.read_then_post(
        || {
            let item = menus::the_item(id).ok_or(())?;
            let reads = the_box_it_opens(item.id).map(|TheBox::About| about());
            Ok((item, reads))
        },
        |(item, reads)| {
            let going = Event::about(Subject::TheRun, THE_USER_CHOSE_A_GOING)
                // A value: the identifier is what the page matches on, and
                // nothing about it is prose (X7, X9).
                .saying("item", Said::Value(Verbatim::new(item.id)))
                // Prose: the same label the menu drew, through the point (K8).
                .saying("label", Said::Prose(item.label()));
            match reads {
                Some(reads) => about_is_said(going, reads.clone()),
                None => going,
            }
        },
    )
}

// ── The two items that open About ──────────────────────────────────────────
//
// **The 38, piece 12.** *About Smoozy*, *Check for updates…* and *Report a
// problem…* posted a going that nothing drew, so pressing any of them did
// nothing and said nothing — and Rule 8 makes Smoozy going quiet a fault
// however small the thing. **The going now says the box each opens**, and
// `web/menus.js` draws it from what the going says and from nothing else (P6).
//
// ***Report a problem…* left this list at step 11's piece R4**: it opens the
// report box itself, `crate::surface::reporting`, and the stand-in box that
// said *not built yet* went with it.
//
// **About, from step 11's check piece** — `Design/Whole app`, *Checking for
// updates*: five readings, one box — the version and when it was last
// checked; checking; a new version found, with the restart on Mac and Windows
// and the update box on Linux; a check that could not be reached; and a found
// update that could not be installed. Both failures name no cause. **The third has a held form** while jobs or transfers run
// (Z31). **Both items open the same box** — *"It does its work in About, and
// there is no second box"* — and *Check for updates…* asks for a check first.
//
// **What is not here, and whose it is.** *Install…* on Linux, which leads to
// the update box, is step 11's Linux piece's, and **is not drawn until the act
// behind it exists**, because a button that does nothing is the fault Rule 8
// names. **The held form's *Install now* is drawn from step 11's Mac update
// piece** (Z32), under its sentence saying what it does, where the act behind
// it exists: the Mac route's, the Linux route's, and from step 11's Windows update piece the
// Windows route's, whose words are the Mac's. Its press is
// [`the_held_update_is_installed_anyway`], which reaches the run's one check
// and through it the route; nothing else installs.
//
// **The moment is a moment and never an age** (Z23a): the application hands
// the stored moment as a value and the page writes it in this computer's own
// way, as every moment Smoozy shows is written. **No word reaches the page
// but through this file** (A6, X1).

/// **Every word About says, and nowhere else.**
pub(crate) mod words {
    /// About's heading — the drawing's own, which is the app's name.
    pub const ABOUTS_HEADING: &str = "Smoozy";
    /// The word the version stands after — the drawing's *Version 2.4.1*.
    pub const BEFORE_THE_VERSION: &str = "Version";
    pub const CLOSE: &str = "Close";
    pub const COPYRIGHT: &str = "Copyright © 2026 Alexander Kislyakov";
    pub const FREE_SOFTWARE: &str = "Smoozy is free software. You may share and modify it under the GNU General Public License, version 3 or later. Smoozy comes with no warranty.";
    pub const LICENCE: &str = "Licence";
    pub const THIRD_PARTY: &str = "Third-party licences";
    pub const SOURCE_CODE: &str = "Source code";
    pub const SOURCE_FAILED: &str = "Could not open the source code page.";
    /// The first reading, before the moment: the drawing's *Up to date, last
    /// checked 25 Aug 13:52.*
    pub const UP_TO_DATE_LAST_CHECKED: &str = "Up to date, last checked";
    /// The first reading, after the moment.
    pub const SMOOZY_CHECKS_ON_ITS_OWN: &str = "Updates install automatically.";
    /// The second reading.
    pub const CHECKING: &str = "Checking\u{2026}";
    /// The third, before the version: the drawing's *Smoozy 2.5.0 is ready.*
    pub const BEFORE_THE_NEW_VERSION: &str = "Smoozy";
    /// The third, after the version.
    pub const IS_READY: &str = "is ready.";
    /// The third on Mac and Windows.
    pub const RESTARTS_TO_INSTALL: &str = "Smoozy will restart to install it.";
    /// The third's held form (Z31).
    pub const IT_WILL_INSTALL_WHEN_NOTHING_RUNS: &str =
        "Installs when no jobs or transfers are running.";
    /// **The held form's one sentence above *Install now*, said before it is
    /// pressed** (Z32) — the drawing's three, each its own string (X10).
    pub const INSTALLING_NOW_RESTARTS: &str = "Installing now restarts Smoozy.";
    pub const JOBS_KEEP_RENDERING: &str = "Jobs keep rendering on their servers.";
    pub const TRANSFERS_STOP: &str = "Uploads and downloads in progress stop.";
    /// The held form's button.
    pub const INSTALL_NOW: &str = "Install now";
    /// The held form's button on Linux, which opens the update box.
    pub const INSTALL_NOW_LINUX: &str = "Install now\u{2026}";
    /// The unheld Linux form's button, which opens the update box.
    pub const INSTALL: &str = "Install\u{2026}";
    /// The fourth, in the failure shape.
    pub const COULD_NOT_REACH: &str = "Could not reach the update service.";
    /// A found update could not be installed.
    pub const INSTALLATION_FAILED: &str = "Update installation failed.";
    /// **A copy that never checks** (E10a) — Build's words, a development
    /// build's only: Design draws no test copy, and a press that drew nothing
    /// would be Smoozy going quiet.
    pub const THIS_COPY_NEVER_CHECKS: &str = "This test copy does not check for updates.";
}

/// **Which box an item opens**, for the two that open one here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TheBox {
    About,
}

fn the_box_it_opens(id: &str) -> Option<TheBox> {
    match id {
        "smoozy.about" | THE_CHECK_ITEM => Some(TheBox::About),
        _ => None,
    }
}

/// **About's reading, as the page is told it when it changes** — the whole
/// box, so the page can draw whichever of two readings is the newer.
pub fn abouts_reading(reading: &TheReading, number: u64, last_checked: Option<SystemTime>) -> Event {
    about_is_said(
        Event::about(Subject::TheRun, THE_ABOUTS_READING),
        WhatAboutReads {
            reading: reading.clone(),
            number,
            last_checked,
        },
    )
}

/// **The reading as the page draws it** — one value naming which, as the
/// drawing names its states.
fn which_reading(reading: &TheReading) -> &'static str {
    match reading {
        TheReading::NoneYet | TheReading::UpToDate => "idle",
        TheReading::Checking => "checking",
        TheReading::ANewerVersion { held: true, .. } => "held",
        TheReading::ANewerVersion { held: false, .. } => {
            if cfg!(target_os = "linux") {
                "new-linux"
            } else {
                "new"
            }
        }
        TheReading::CouldNotBeReached => "fail",
        TheReading::CouldNotInstall => "install-fail",
        TheReading::ThisCopyNeverChecks => "never",
    }
}

// Build-generated, target-specific copies. They never require a network read.
const THE_LICENCE: &str = include_str!(concat!(env!("OUT_DIR"), "/COPYING"));
const THE_THIRD_PARTY_NOTICES: &str = include_str!(concat!(env!("OUT_DIR"), "/THIRD-PARTY-LICENSES.txt"));

/// **What About says.** Prose through the point, the version and the moment
/// by the verbatim route (X7, X9) — a user copies a version into a report.
fn about_is_said(event: Event, reads: WhatAboutReads) -> Event {
    let prose = |words: &str| Said::Prose(Prose::new(words));
    let which = which_reading(&reads.reading);
    let mut about = event
        .saying("close", prose(words::CLOSE))
        .saying("copyright", prose(words::COPYRIGHT))
        .saying("freeSoftware", prose(words::FREE_SOFTWARE))
        .saying("licence", prose(words::LICENCE))
        .saying("thirdParty", prose(words::THIRD_PARTY))
        .saying("sourceCode", prose(words::SOURCE_CODE))
        .saying("sourceFailed", prose(words::SOURCE_FAILED))
        .saying("licenceText", Said::Value(Verbatim::new(THE_LICENCE)))
        .saying("thirdPartyText", Said::Value(Verbatim::new(THE_THIRD_PARTY_NOTICES)))
        .saying("heading", prose(words::ABOUTS_HEADING))
        .saying("beforeTheVersion", prose(words::BEFORE_THE_VERSION))
        .saying("version", Said::Value(Verbatim::new(env!("CARGO_PKG_VERSION"))))
        .saying("reading", Said::Value(Verbatim::new(which)))
        .saying("readingNumber", Said::Value(Verbatim::new(reads.number.to_string())));
    match &reads.reading {
        TheReading::NoneYet | TheReading::UpToDate => {
            // **A moment, or nothing invented**: where no check has ever
            // completed, About says the version alone.
            let seconds = reads
                .last_checked
                .and_then(|at| at.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|since| since.as_secs().to_string());
            if let Some(seconds) = seconds {
                about = about
                    .saying("upToDate", prose(words::UP_TO_DATE_LAST_CHECKED))
                    .saying("lastChecked", Said::Value(Verbatim::new(seconds)))
                    .saying("checksOnItsOwn", prose(words::SMOOZY_CHECKS_ON_ITS_OWN));
            }
        }
        TheReading::Checking => about = about.saying("checking", prose(words::CHECKING)),
        TheReading::ANewerVersion { version, held } => {
            about = about
                .saying("beforeTheNewVersion", prose(words::BEFORE_THE_NEW_VERSION))
                .saying("newVersion", Said::Value(Verbatim::new(version.clone())))
                .saying("isReady", prose(words::IS_READY));
            if *held {
                about = about.saying("afterReady", prose(words::IT_WILL_INSTALL_WHEN_NOTHING_RUNS));
                // **Z32's control and the sentence before it, where the act
                // behind it exists** — the Mac, Linux and Windows routes' (the
                // last from step 11's Windows update piece).
                if cfg!(any(target_os = "macos", target_os = "linux", target_os = "windows")) {
                    about = about
                        .saying("installingRestarts", prose(words::INSTALLING_NOW_RESTARTS))
                        .saying("jobsKeepRendering", prose(words::JOBS_KEEP_RENDERING))
                        .saying("transfersStop", prose(words::TRANSFERS_STOP))
                        .saying(
                            "installNow",
                            // **On Linux it leads to the update box** (Design,
                            // *Checking for updates*), so it reads *Install now…*.
                            prose(if cfg!(target_os = "linux") {
                                words::INSTALL_NOW_LINUX
                            } else {
                                words::INSTALL_NOW
                            }),
                        );
                }
            } else if which == "new" {
                about = about.saying("afterReady", prose(words::RESTARTS_TO_INSTALL));
            } else if which == "new-linux" {
                about = about.saying("installLinux", prose(words::INSTALL));
            }
        }
        TheReading::CouldNotBeReached => {
            about = about.saying("couldNotReach", prose(words::COULD_NOT_REACH))
        }
        TheReading::CouldNotInstall => {
            about = about.saying("couldNotInstall", prose(words::INSTALLATION_FAILED))
        }
        TheReading::ThisCopyNeverChecks => {
            about = about.saying("neverChecks", prose(words::THIS_COPY_NEVER_CHECKS))
        }
    }
    about
}

/// The window acts, on the two systems where the application performs them
/// itself. On macOS the platform's own menu items do all five and none of them
/// reaches here.
fn the_system_acts<R: Runtime>(app: &AppHandle<R>, act: TheSystemsAct) {
    if let TheSystemsAct::Quit = act {
        // **Through the shell's own ask rather than around it**, so that
        // `State, identity and recovery` I2's run-mark is written at the moment
        // the user asks to quit. `main.rs` holds both routes out.
        app.exit(0);
        return;
    }
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let _ = match act {
        TheSystemsAct::Hide => window.hide(),
        TheSystemsAct::Minimise => window.minimize(),
        TheSystemsAct::Zoom => match window.is_maximized() {
            Ok(true) => window.unmaximize(),
            _ => window.maximize(),
        },
        TheSystemsAct::FullScreen => {
            let now = window.is_fullscreen().unwrap_or(false);
            window.set_fullscreen(!now)
        }
        TheSystemsAct::Quit => Ok(()),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The page is handed the four menus and nothing composed.**
    ///
    /// It also shows the derivation that keeps the combination in one place:
    /// the drawn label and the key handler's own reading both come out of the
    /// same [`crate::menus::Combination`], so a change to the map cannot leave
    /// the label saying one thing and the keystroke doing another.
    #[test]
    fn the_page_is_handed_the_four_menus_with_their_keys_derived_once() {
        let bar = the_menu_bar();
        assert_eq!(bar.menus.len(), 4);
        assert_eq!(
            bar.menus.iter().map(|menu| menu.id).collect::<Vec<_>>(),
            vec!["smoozy", "edit", "go", "window"]
        );

        let cut = bar.menus[1].entries[0]
            .item
            .as_ref()
            .expect("Cut is an item");
        assert_eq!(cut.id, "edit.cut");
        assert_eq!(cut.on_whatever_has_focus, Some("cut"));
        match (cut.key.as_ref(), cut.press.as_ref()) {
            // macOS: the system's own bar carries the item and its key, so the
            // page is handed neither. That is K14's split rather than a gap.
            (None, None) => assert!(cfg!(target_os = "macos")),
            (Some(key), Some(press)) => {
                assert_eq!(key.as_str(), "Ctrl+X");
                assert!(press.ctrl && !press.alt && !press.shift);
                assert_eq!(press.key, "x");
            }
            _ => panic!("the drawn label and the key handler's reading disagree"),
        }
    }

    /// **The rules between the items survive the crossing**, because a menu
    /// whose groups ran together would be Design's list redrawn.
    #[test]
    fn the_separators_reach_the_page_as_entries_with_no_item() {
        let bar = the_menu_bar();
        let smoozy = &bar.menus[0];
        let rules = smoozy
            .entries
            .iter()
            .filter(|entry| entry.item.is_none())
            .count();
        assert_eq!(rules, 2, "the Smoozy menu's two rules are Design's");
    }

    /// **The page is handed no item this system does not draw, and choosing one
    /// anyway performs nothing** — F-1, carrying C-1.
    ///
    /// **Both halves are checked because both can be reached.** The list is what
    /// the user meets; the command is reachable without the list, exactly as a
    /// pane width is reachable without the page's own clamping.
    #[test]
    fn an_item_this_system_does_not_draw_is_neither_handed_over_nor_performed() {
        let bar = the_menu_bar();
        let drawn: Vec<&str> = bar
            .menus
            .iter()
            .flat_map(|menu| menu.entries.iter())
            .filter_map(|entry| entry.item.as_ref().map(|item| item.id))
            .collect();

        if cfg!(target_os = "macos") {
            assert!(
                drawn.contains(&"smoozy.hide"),
                "the Mac no longer draws Hide, and on that system it is the \
                 platform's own item with the Dock as the way back"
            );
        } else {
            assert!(
                !drawn.contains(&"smoozy.hide"),
                "the page is handed *Hide Smoozy* on a system with nothing to \
                 bring the window back. **There is no tray icon in this crate, \
                 no second window, and a second launch starts a second copy** — \
                 so the act is not *hide*, it is *make the application \
                 unreachable*, with the render still running"
            );
        }

        // **And the one route in refuses it too.** A command is reachable
        // without the page's own list, so the list alone is not the guard.
        let (working, _) = include_str!("menus.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        assert!(
            working.contains("if !item.is_drawn_on(std::env::consts::OS) {"),
            "the one route from a chosen item to an act no longer asks whether \
             this system draws it, so an item the page was never handed can \
             still be performed by naming it"
        );

        // The rules between the items survive either way: dropping Hide leaves
        // the rule above it beside Quit, which is Design's own grouping.
        assert_eq!(
            bar.menus[0]
                .entries
                .iter()
                .filter(|entry| entry.item.is_none())
                .count(),
            2
        );
    }

    /// **H34: an identifier the four menus do not name is refused**, and the
    /// closed set is the menus themselves rather than a second list that could
    /// disagree with them.
    #[test]
    fn an_item_the_menus_do_not_name_is_refused() {
        let queue = TheOneQueue::new();
        assert!(the_going(&queue, "smoozy.take-over-the-world").is_err());
        assert!(the_going(&queue, "smoozy.about").is_ok());
    }

    /// **The going reaches the page as a going, and it says the two things it
    /// says** — one by the prose route and one by the verbatim route, which is
    /// A6 and X9 and is the only pair of routes there is.
    #[test]
    fn a_going_is_posted_on_the_one_queue_saying_which_item_it_was() {
        use std::sync::Mutex;

        struct Held(Arc<Mutex<Vec<crate::events::Delivery>>>);
        impl crate::events::Delivers for Held {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                self.0
                    .lock()
                    .expect("the list is held")
                    .push(delivery.clone());
            }
        }

        let seen = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Held(seen.clone())));

        the_going(&queue, "go.queue").expect("the item is one of the four menus'");

        let seen = seen.lock().expect("the list is held");
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].kind, THE_USER_CHOSE_A_GOING);
        assert_eq!(
            seen[0].said.get("item"),
            Some(&Said::Value(Verbatim::new("go.queue")))
        );
        assert_eq!(
            seen[0].said.get("label"),
            Some(&Said::Prose(Prose::new("Queue")))
        );
    }

    /// **What one going said, as the page is handed it.**
    fn what_the_going_said(id: &str) -> crate::events::Delivery {
        use std::sync::Mutex;

        struct Held(Arc<Mutex<Vec<crate::events::Delivery>>>);
        impl crate::events::Delivers for Held {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                self.0
                    .lock()
                    .expect("the list is held")
                    .push(delivery.clone());
            }
        }

        let seen = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Held(seen.clone())));
        the_going(&queue, id).expect("the item is one of the four menus'");
        let mut seen = seen.lock().expect("the list is held");
        assert_eq!(seen.len(), 1, "{id} posted one going");
        seen.remove(0)
    }

    /// **About opens on the build's own version and its reading, and nothing
    /// invented** — `Design/Whole app`, *Checking for updates*. With no check
    /// ever completed and none begun, the version alone: no installed moment
    /// (`State, identity and recovery` I4 holds none), no last-checked moment,
    /// no age.
    ///
    /// **The version goes by the verbatim route** (X7, X9), as the run's own
    /// first event sends it: a user copies a version into a report.
    #[test]
    fn wave4_source_action_asks_the_named_opener_and_reports_refusal() {
        use crate::the_address_opener::{Opened, WhichAddress};
        for (answer, expected) in [(Opened::ItWasAsked, TheChoice::ItWasCarriedOut),
                                   (Opened::ItCouldNotBeOpened, TheChoice::ItCouldNotBeOpened)] {
            assert_eq!(the_source_is_asked_with(|which| {
                assert_eq!(which, WhichAddress::Source);
                answer
            }), expected);
        }
    }

    #[test]
    fn wave4_about_carries_the_notices_and_offline_texts() {
        let about = what_the_going_said("smoozy.about");
        for key in ["copyright", "freeSoftware", "licence", "thirdParty", "sourceCode", "licenceText", "thirdPartyText"] {
            assert!(about.said.contains_key(key), "About is missing {key}");
        }
        assert!(!about.said.contains_key("donate"));
        assert_eq!(about.said.get("licenceText"), Some(&Said::Value(Verbatim::new(include_str!("../../../../COPYING")))));
        assert!(THE_THIRD_PARTY_NOTICES.contains("smoozy-server — x86_64-unknown-linux-musl"));
        assert!(THE_THIRD_PARTY_NOTICES.contains("ring 0.17.14"));
    }

    #[test]
    fn about_opens_a_box_with_the_builds_own_version_and_nothing_invented() {
        let about = what_the_going_said("smoozy.about");
        assert_eq!(
            about.said.get("version"),
            Some(&Said::Value(Verbatim::new(env!("CARGO_PKG_VERSION")))),
            "*About Smoozy* does not say the build's own version"
        );
        assert_eq!(
            about.said.get("heading"),
            Some(&Said::Prose(Prose::new("Smoozy"))),
            "the box has no heading, so the page has no box to draw"
        );
        assert_eq!(
            about.said.get("beforeTheVersion"),
            Some(&Said::Prose(Prose::new("Version")))
        );
        assert_eq!(
            about.said.get("close"),
            Some(&Said::Prose(Prose::new("Close")))
        );
        assert_eq!(
            about.said.get("reading"),
            Some(&Said::Value(Verbatim::new("idle")))
        );

        let mut said: Vec<&str> = about.said.keys().copied().collect();
        said.sort_unstable();
        assert_eq!(
            said,
            vec![
                "beforeTheVersion",
                "close",
                "copyright",
                "freeSoftware",
                "heading",
                "item",
                "label",
                "licence",
                "licenceText",
                "reading",
                "readingNumber",
                "sourceCode",
                "sourceFailed",
                "thirdParty",
                "thirdPartyText",
                "version"
            ],
            "*About* says something besides the version where no check has \
             completed. **Nothing invented**: no installed moment exists, and no \
             last-checked moment either"
        );
    }

    /// What one reading of About says, as the page is handed it.
    fn about_reads(
        reading: TheReading,
        last_checked: Option<SystemTime>,
    ) -> crate::events::Delivery {
        use std::sync::Mutex;

        struct Held(Arc<Mutex<Vec<crate::events::Delivery>>>);
        impl crate::events::Delivers for Held {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                self.0
                    .lock()
                    .expect("the list is held")
                    .push(delivery.clone());
            }
        }
        let seen = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Held(seen.clone())));
        let reads = WhatAboutReads {
            reading,
            number: 7,
            last_checked,
        };
        the_going_with(&queue, THE_CHECK_ITEM, &|| reads.clone()).expect("the item is the menus'");
        let mut seen = seen.lock().expect("the list is held");
        assert_eq!(seen.len(), 1);
        seen.remove(0)
    }

    #[test]
    fn d5b_about_says_that_the_update_restarts_smoozy() {
        let said = about_reads(
            TheReading::ANewerVersion {
                version: "0.2.0".into(),
                held: false,
            },
            None,
        );
        if !cfg!(target_os = "linux") {
            assert_eq!(
                said.said.get("afterReady"),
                Some(&Said::Prose(Prose::new(
                    "Smoozy will restart to install it."
                )))
            );
        }
    }

    #[test]
    fn d5b_about_has_words_for_an_installation_that_failed() {
        let failed = about_reads(TheReading::CouldNotInstall, None);
        assert_eq!(
            failed.said.get("couldNotInstall"),
            Some(&Said::Prose(Prose::new("Update installation failed.")))
        );
        assert_eq!(failed.said.get("couldNotReach"), None);
    }

    /// **Design's five readings and the held form, each said in its own
    /// words** — and a copy that never checks. *Check for updates…* opens About
    /// and no second box, carrying the version as About does.
    ///
    /// **The first reading names a moment and never an age** (Z23a): the
    /// stored moment crosses as seconds on the verbatim route, and nothing
    /// here subtracts it from anything.
    #[test]
    fn every_reading_of_about_is_said_in_its_own_words() {
        let prose = |words: &str| Some(Said::Prose(Prose::new(words)));
        let value = |words: &str| Some(Said::Value(Verbatim::new(words)));
        let at = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_756_129_920);

        let idle = about_reads(TheReading::UpToDate, Some(at));
        assert_eq!(idle.kind, THE_USER_CHOSE_A_GOING);
        assert_eq!(
            idle.said.get("heading").cloned(),
            prose("Smoozy"),
            "the check opens a second box"
        );
        assert_eq!(
            idle.said.get("version").cloned(),
            value(env!("CARGO_PKG_VERSION"))
        );
        assert_eq!(idle.said.get("reading").cloned(), value("idle"));
        assert_eq!(idle.said.get("readingNumber").cloned(), value("7"));
        assert_eq!(
            idle.said.get("upToDate").cloned(),
            prose(words::UP_TO_DATE_LAST_CHECKED)
        );
        assert_eq!(idle.said.get("lastChecked").cloned(), value("1756129920"));
        assert_eq!(
            idle.said.get("checksOnItsOwn").cloned(),
            prose(words::SMOOZY_CHECKS_ON_ITS_OWN)
        );
        assert_eq!(
            idle.said.get("notBuiltYet"),
            None,
            "the stand-in sentence is back"
        );

        let checking = about_reads(TheReading::Checking, Some(at));
        assert_eq!(checking.said.get("reading").cloned(), value("checking"));
        assert_eq!(
            checking.said.get("checking").cloned(),
            prose("Checking\u{2026}")
        );
        assert_eq!(checking.said.get("lastChecked"), None);

        let new = about_reads(
            TheReading::ANewerVersion {
                version: "0.2.0".into(),
                held: false,
            },
            Some(at),
        );
        let which = if cfg!(target_os = "linux") {
            "new-linux"
        } else {
            "new"
        };
        assert_eq!(new.said.get("reading").cloned(), value(which));
        assert_eq!(
            new.said.get("beforeTheNewVersion").cloned(),
            prose("Smoozy")
        );
        assert_eq!(new.said.get("newVersion").cloned(), value("0.2.0"));
        assert_eq!(new.said.get("isReady").cloned(), prose("is ready."));
        if which == "new" {
            assert_eq!(
                new.said.get("afterReady").cloned(),
                prose(words::RESTARTS_TO_INSTALL)
            );
        } else {
            assert_eq!(
                new.said.get("afterReady"),
                None,
                "Linux promises a next start"
            );
        }

        let held = about_reads(
            TheReading::ANewerVersion {
                version: "0.2.0".into(),
                held: true,
            },
            Some(at),
        );
        assert_eq!(held.said.get("reading").cloned(), value("held"));
        assert_eq!(held.said.get("newVersion").cloned(), value("0.2.0"));
        assert_eq!(
            held.said.get("afterReady").cloned(),
            prose(words::IT_WILL_INSTALL_WHEN_NOTHING_RUNS)
        );
        // **Z32: the held form says what *Install now* does, before the
        // press, in the drawing's three sentences**, where the act exists.
        let installs = |key: &str| held.said.get(key).cloned();
        if cfg!(any(target_os = "macos", target_os = "linux", target_os = "windows")) {
            assert_eq!(
                installs("installingRestarts"),
                prose("Installing now restarts Smoozy.")
            );
            assert_eq!(
                installs("jobsKeepRendering"),
                prose("Jobs keep rendering on their servers.")
            );
            assert_eq!(
                installs("transfersStop"),
                prose("Uploads and downloads in progress stop.")
            );
            let install_now = if cfg!(target_os = "linux") {
                "Install now\u{2026}"
            } else {
                "Install now"
            };
            assert_eq!(installs("installNow"), prose(install_now));
        } else {
            assert_eq!(
                installs("installNow"),
                None,
                "a button with nothing behind it"
            );
        }
        // The unheld form has no button.
        assert_eq!(new.said.get("installNow"), None);

        let fail = about_reads(TheReading::CouldNotBeReached, Some(at));
        assert_eq!(fail.said.get("reading").cloned(), value("fail"));
        assert_eq!(
            fail.said.get("couldNotReach").cloned(),
            prose(words::COULD_NOT_REACH)
        );
        assert_eq!(
            fail.said.get("lastChecked"),
            None,
            "the failure names a moment"
        );

        let never = about_reads(TheReading::ThisCopyNeverChecks, Some(at));
        assert_eq!(never.said.get("reading").cloned(), value("never"));
        assert_eq!(
            never.said.get("neverChecks").cloned(),
            prose(words::THIS_COPY_NEVER_CHECKS)
        );

        // **No age, anywhere**: nothing in this file reads the clock to
        // subtract a moment.
        let (working, _) = include_str!("menus.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        assert!(!working.contains("SystemTime::now()"));
        assert!(!working.contains(".elapsed()"));

        // **The report box, opened from the menu with nothing handed over** —
        // the box's own event, asking, naming no failure.
        let report = what_the_going_said("smoozy.report-a-problem");
        assert_eq!(report.kind, crate::surface::reporting::THE_REPORT_BOX);
        assert_eq!(
            report.said.get("notBuiltYet"),
            None,
            "the stand-in box is back"
        );
        assert_eq!(
            report.said.get("version"),
            None,
            "the report box is not About"
        );
    }

    /// **The press reaches the run's one check before About is told**, and
    /// About opens on what the press left.
    #[test]
    fn the_press_asks_the_runs_check_before_about_is_drawn() {
        let (working, _) = include_str!("menus.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        let route = working
            .split_once("Act::AGoing => {")
            .and_then(|(_, rest)| rest.split_once("TheChoice::ItWasCarriedOut"))
            .map(|(route, _)| route)
            .expect("the going's route is here");
        let asks = route
            .find("check.the_user_asks();")
            .expect("the press asks for no check");
        let told = route
            .find("the_going_with(queue, id, &|| what_about_reads_here(app))")
            .expect("About is not told what the run's check reads");
        assert!(asks < told);
    }

    /// **The page draws the box from the going and from nothing else** — P6,
    /// and the page's rule that no prose is written into a page module: every
    /// word arrives on the event.
    #[test]
    fn the_page_draws_the_box_from_what_the_going_says() {
        let page = the_page_module();
        for read in [
            "said.heading",
            "said.beforeTheVersion",
            "said.version",
            "said.reading",
            "said.readingNumber",
            "said.upToDate",
            "said.lastChecked",
            "said.checksOnItsOwn",
            "said.checking",
            "said.beforeTheNewVersion",
            "said.newVersion",
            "said.isReady",
            "said.afterReady",
            "said.couldNotReach",
            "said.neverChecks",
            "said.installingRestarts",
            "said.jobsKeepRendering",
            "said.transfersStop",
            "said.installNow",
            "said.close",
        ] {
            assert!(
                page.contains(read),
                "web/menus.js does not read `{read}` off the going, so the box \
                 the application said is not drawn"
            );
        }
        assert!(
            page.contains("theBoxOpens(theNewerOf(delivery.said))"),
            "web/menus.js no longer opens the box when a going says one"
        );
        assert!(page.contains("THE_ABOUTS_READING = \"menus:abouts-reading\""));
        // **It is the drawing's own top box**, the one Design draws About in.
        assert!(page.contains("\"modal top\"") && page.contains("\"mbox\""));
    }

    /// The page's own menu module.
    fn the_page_module() -> String {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/menus.js"),
        )
        .expect("the page's menu module is readable")
    }

    /// **The cut, copy, paste and select-all presses are the engine's, and
    /// Smoozy neither matches nor cancels them on the two systems that draw
    /// their own menus** — C-2 and F-2.
    ///
    /// **What the old code cost.** A Linux user copies a host name from their
    /// provider's page in their browser, clicks into Smoozy's field and presses
    /// paste. **Nothing appears and nothing says why.** They try *Edit, Paste*,
    /// which does the identical thing through the identical call, so nothing
    /// appears there either. There is no third route.
    ///
    /// **Why the engine gets them.** K18 was written to permit exactly that, and
    /// K16 carries the one published Windows behaviour anybody has read on this
    /// point — *"with a text box focused, the copy accelerator copies the
    /// selected text and app-defined accelerators are ignored — a control can
    /// win"*. Cancelling the press made the application win instead, which is
    /// the opposite of it. **The worst case of leaving them to the engine is the
    /// case §22 item 52 already prices**; the worst case of the old code was a
    /// paste that could never work by any route.
    #[test]
    fn the_clipboard_presses_are_left_to_the_engine() {
        let page = the_page_module();
        assert!(
            page.contains("if (entry.item.on_whatever_has_focus) continue;"),
            "web/menus.js matches the Edit menu's four presses again. **They are \
             the engine's** — K18 permits it and K16 carries the one published \
             behaviour, which says a focused control wins — and a Smoozy that \
             cancels the press and performs the act itself through one \
             unestablished call leaves a user with no working paste at all"
        );

        // **The drawn labels stay**, which cost nothing and are K15's, and the
        // menu items keep acting by click.
        let bar = the_menu_bar();
        let cut = bar.menus[1].entries[0]
            .item
            .as_ref()
            .expect("Cut is an item");
        assert_eq!(cut.on_whatever_has_focus, Some("cut"));
        if !cfg!(target_os = "macos") {
            assert_eq!(
                cut.key
                    .as_ref()
                    .map(|key| key.as_str().to_string())
                    .as_deref(),
                Some("Ctrl+X"),
                "the drawn combination went with the key handler. **K15 draws \
                 the label and K14 splits the mechanism** — the label is not \
                 the thing that was wrong"
            );
        }
    }

    /// **What has focus is kept as it changes and never read at the moment of
    /// the click** — K7 and F-8.
    ///
    /// The module's own comment had the worry exactly right — *"the menu takes
    /// focus when it opens, so this module puts it back before it acts"* — and
    /// the line did not answer it, because `document.activeElement` was read
    /// **during the click**, which is after the press has already moved focus to
    /// the menu button. **One listener on the document settles it rather than
    /// resting on what an engine does with focus on a press.**
    #[test]
    fn what_had_focus_is_kept_as_it_changes_rather_than_read_at_the_click() {
        let page = the_page_module();
        assert!(
            page.contains("\"focusin\""),
            "web/menus.js no longer keeps a note of what has focus as it \
             changes. **K7: cut, copy and paste act on whatever has focus** — \
             and a read taken inside a click handler reads the menu button, \
             because the press has already moved focus there"
        );
        // The prose above and below says the words in order to forbid them, so
        // a comment is not an instance.
        let working: String = page
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(
            working.matches("document.activeElement").count(),
            0,
            "web/menus.js reads what has focus at the moment it acts. **That is \
             the fault F-8 named**: the click is raised after the press has \
             moved focus, so what it reads is the button the user just pressed"
        );
    }

    /// **The menu button is not a window button and does not sit in a second
    /// bar** — F-9, and Design's *one bar at one height everywhere*.
    ///
    /// The module had no stylesheet and borrowed two classes, and both brought
    /// their drawing with them: the mockup's menu-bar strip, which carries its
    /// own height, its own sunken ground and a bottom border, and the window
    /// button, which nothing narrowed — so the menu button became a fourth grey
    /// circle beside three identical ones on Linux, and a full-height plate on
    /// Windows.
    #[test]
    fn the_menu_button_has_its_own_drawing_and_borrows_no_window_buttons() {
        let page = the_page_module();
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");

        for borrowed in ["\"sysbar\"", "\"wbtn\""] {
            assert!(
                !page.contains(borrowed),
                "web/menus.js draws with {borrowed} again. **`.sysbar` is the \
                 mockup's menu-bar strip and `.wbtn` is the window button**, and \
                 each brings its own drawing — a short sunken box with a rule \
                 under it inside the title bar, and a fourth button wearing the \
                 window buttons' own shape. Design settles that Smoozy draws one \
                 bar at one height everywhere"
            );
        }
        assert!(page.contains("\"mnu\"") && page.contains("\"mnu-btn\""));

        let own = std::fs::read_to_string(web.join("menus.css"))
            .expect("the menu module has a stylesheet of its own");
        assert!(own.contains(".mnu-btn"), "the menu button has no rules of its own");

        let list = std::fs::read_to_string(web.join("smoozy.css"))
            .expect("the import list is readable");
        assert!(
            list.contains("@import \"menus.css\";"),
            "web/menus.css is not in the page's one import list, so nothing \
             loads it and the button draws with no rules at all"
        );

        // **And the window-button rules are narrowed to the two containers they
        // are for**, so the class cannot be borrowed again by accident.
        let chrome = std::fs::read_to_string(web.join("chrome.css"))
            .expect("the chrome stylesheet is readable");
        for line in chrome.lines() {
            let line = line.trim();
            if !line.contains(".wbtn") || line.starts_with('*') || line.starts_with("/*") {
                continue;
            }
            assert!(
                line.contains(".lights .wbtn"),
                "a window-button rule is not narrowed to the containers the \
                 window buttons live in: `{line}`. **Anything at all can wear \
                 the class while it is written that wide**, which is how the \
                 menu button came to be drawn as one"
            );
        }
    }

    // ── About in a real browser ─────────────────────────────────────────────

    /// One delivery, as the page is handed it, from one composed event.
    fn delivered(post: impl FnOnce(&TheOneQueue)) -> serde_json::Value {
        use std::sync::Mutex;

        struct Held(Arc<Mutex<Vec<crate::events::Delivery>>>);
        impl crate::events::Delivers for Held {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                self.0
                    .lock()
                    .expect("the list is held")
                    .push(delivery.clone());
            }
        }
        let seen = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Held(seen.clone())));
        post(&queue);
        let seen = seen.lock().expect("the list is held");
        assert_eq!(seen.len(), 1);
        serde_json::to_value(&seen[0]).expect("a delivery serialises")
    }

    fn a_going(
        id: &'static str,
        reading: TheReading,
        number: u64,
        last_checked: Option<SystemTime>,
    ) -> serde_json::Value {
        delivered(|queue| {
            let reads = WhatAboutReads {
                reading,
                number,
                last_checked,
            };
            the_going_with(queue, id, &|| reads.clone()).expect("the item is the menus'");
        })
    }

    fn a_reading(
        reading: TheReading,
        number: u64,
        last_checked: Option<SystemTime>,
    ) -> serde_json::Value {
        delivered(|queue| {
            queue
                .read_then_post(
                    || Ok::<_, ()>(()),
                    |_| abouts_reading(&reading, number, last_checked),
                )
                .expect("posted");
        })
    }

    fn a_chrome() -> Option<std::path::PathBuf> {
        [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ]
        .into_iter()
        .map(std::path::PathBuf::from)
        .find(|one| one.exists())
    }

    /// **About's readings, drawn in a real browser** — headless Chrome draws
    /// the box from `web/menus.js` and the application's own stylesheets, fed
    /// deliveries composed by this file's own functions.
    ///
    /// Each reading as the drawing draws it (`box-about`): the version and the
    /// moment the last check completed, written as a moment; *Checking…* with
    /// the turning mark; a new version found; its held form; both failure readings;
    /// a copy that never checks; and the version alone where no check ever
    /// completed. **A reading changes an open box and never opens a closed
    /// one**, and **a going that crosses a newer reading draws the newer**.
    /// Every reading fits the window at 1280 and 720 with nothing scrolling
    /// sideways, and *Close* is the only ordinary action beside the links. A computer with no Chrome
    /// skips it.
    #[test]
    fn wave4_every_reading_of_about_is_drawn_in_a_real_browser() {
        use std::fs;
        use std::path::Path;
        use std::time::Duration;

        let Some(chrome) = a_chrome() else {
            eprintln!("SKIPPED: no Chrome on this computer, so About was not drawn");
            return;
        };
        let at = std::time::UNIX_EPOCH + Duration::from_secs(1_756_129_920);
        let newer = |held| TheReading::ANewerVersion {
            version: "0.2.0".into(),
            held,
        };
        let linux = delivered(|queue| {
            let _ = queue.read_then_post(|| Ok::<_, ()>(()), |_| the_linux_offer("0.2.0"));
        });
        let deliveries = serde_json::json!({
            "idle": a_going("smoozy.about", TheReading::UpToDate, 1, Some(at)),
            "checking": a_reading(TheReading::Checking, 2, Some(at)),
            "new": a_reading(newer(false), 3, Some(at)),
            "held": a_reading(newer(true), 4, Some(at)),
            "fail": a_reading(TheReading::CouldNotBeReached, 5, Some(at)),
            "installFail": a_reading(TheReading::CouldNotInstall, 6, Some(at)),
            "whileShut": a_reading(TheReading::UpToDate, 7, Some(at)),
            "crossed": a_going(THE_CHECK_ITEM, TheReading::Checking, 5, Some(at)),
            "never": a_going("smoozy.about", TheReading::ThisCopyNeverChecks, 8, None),
            "versionAlone": a_going("smoozy.about", TheReading::NoneYet, 9, None),
            "linux": linux,
            "rollback": delivered(|queue| {
                let _ = queue.read_then_post(|| Ok::<_, ()>(()), |_| the_rollback_box("0.3.0", true, false));
            }),
            "rollbackFailed": delivered(|queue| {
                let _ = queue.read_then_post(|| Ok::<_, ()>(()), |_| the_rollback_box("0.3.0", true, true));
            }),
            "rollbackCleanup": delivered(|queue| {
                let _ = queue.read_then_post(|| Ok::<_, ()>(()), |_| the_rollback_box("0.3.0", false, true));
            }),
        });

        let folder =
            crate::scratch_folders::a_folder(format!("smoozy-about-page-{}", std::process::id()));
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for entry in fs::read_dir(&web).expect("the page folder is readable") {
            let path = entry.expect("an entry").path();
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if path.is_file() && (extension == "js" || extension == "css") {
                fs::copy(&path, folder.join(path.file_name().unwrap()))
                    .expect("the page is copied");
            }
        }
        fs::write(
            folder.join("deliveries.js"),
            format!("export default {deliveries};\n"),
        )
        .expect("the deliveries are written");
        fs::write(folder.join("index.html"), THE_ABOUT_PAGE).expect("the page is written");

        let mut answers = Vec::new();
        for width in [1280, 720] {
            let dumped = folder.join(format!("dumped-{width}.html"));
            let browser_errors = folder.join(format!("chrome-errors-{width}.txt"));
            let mut running = std::process::Command::new(&chrome)
                .args([
                    "--headless",
                    "--disable-gpu",
                    "--no-first-run",
                    "--allow-file-access-from-files",
                    "--virtual-time-budget=8000",
                    "--dump-dom",
                ])
                .arg(format!("--window-size={width},800"))
                .arg(format!(
                    "--user-data-dir={}",
                    folder.join(format!("profile-{width}")).display()
                ))
                .arg(format!("file://{}", folder.join("index.html").display()))
                .stdout(fs::File::create(&dumped).expect("the dump is written"))
                .stderr(fs::File::create(&browser_errors).expect("the browser's errors are kept"))
                .spawn()
                .expect("Chrome starts");
            // A cancellable fixed deadline; no wait loop.
            let pid = running.id();
            #[cfg(windows)]
            let handle = {
                use std::os::windows::io::AsRawHandle;
                running.as_raw_handle() as usize
            };
            let (cancel, deadline) = std::sync::mpsc::channel::<()>();
            let guard = std::thread::spawn(move || {
                if deadline.recv_timeout(Duration::from_secs(60)).is_err() {
                    #[cfg(unix)]
                    unsafe {
                        libc::kill(pid as i32, libc::SIGKILL);
                    }
                    #[cfg(windows)]
                    unsafe {
                        #[link(name = "kernel32")]
                        unsafe extern "system" {
                            fn TerminateProcess(handle: *mut std::ffi::c_void, code: u32) -> i32;
                        }
                        TerminateProcess(handle as *mut std::ffi::c_void, 1);
                    }
                    #[cfg(not(any(unix, windows)))]
                    let _ = pid;
                }
            });
            let ended = running.wait().expect("Chrome is watched");
            let _ = cancel.send(());
            guard.join().expect("the deadline ends");
            assert!(
                ended.success(),
                "Chrome did not finish: {ended}: {}",
                fs::read_to_string(&browser_errors).unwrap_or_default()
            );
            let page = fs::read_to_string(&dumped).unwrap_or_default();
            let out = page
                .split("<pre id=\"out\">")
                .nth(1)
                .and_then(|rest| rest.split("</pre>").next())
                .filter(|said| !said.is_empty())
                .unwrap_or_else(|| {
                    panic!(
                        "the page drew nothing at {width}: {page}: {}",
                        fs::read_to_string(&browser_errors).unwrap_or_default()
                    )
                })
                .replace("&nbsp;", "\u{a0}")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&amp;", "&");
            answers.push((width, serde_json::from_str::<serde_json::Value>(&out).expect("one JSON answer")));
        }

        let flat = |text: &serde_json::Value| text.as_str().unwrap_or("").replace('\u{a0}', " ");
        let plain = |words: &str| Prose::new(words).as_str().replace('\u{a0}', " ");
        let version = env!("CARGO_PKG_VERSION");
        for (width, out) in &answers {
            assert!(out["error"].is_null(), "{width}: the page stopped: {} {}", out["error"], out["troubles"]);
            assert_eq!(out["troubles"], serde_json::json!([]), "{width}: the page reported trouble");
            assert_eq!(flat(&out["notices"]["copyright"]), "Copyright © 2026 Alexander Kislyakov");
            assert_eq!(out["notices"]["links"], serde_json::json!([Prose::new(words::LICENCE).as_str(), Prose::new(words::THIRD_PARTY).as_str(), Prose::new(words::SOURCE_CODE).as_str()]));
            assert_eq!(flat(&out["notices"]["notice"]), words::FREE_SOFTWARE);
            assert_eq!(out["readers"].as_array().unwrap().len(), 2);
            for reader in out["readers"].as_array().unwrap() {
                for field in ["inside", "exact", "selectable", "fits", "returns"] {
                    assert_eq!(reader[field], true, "{width}: {field}: {reader}");
                }
            }
            assert_eq!(out["source"]["choices"], serde_json::json!([
                {"command": "a_menu_item_was_chosen", "args": {"item": "about.source"}},
                {"command": "a_menu_item_was_chosen", "args": {"item": "about.source"}}
            ]));
            assert_eq!(flat(&out["source"]["failure"]), words::SOURCE_FAILED);
            let states = &out["states"];
            let at = |state: &str| -> &serde_json::Value {
                let one = &states[state];
                assert!(!one.is_null(), "{width}: {state} was not reached: {out}");
                one
            };
            for state in ["idle", "checking", "new", "held", "fail", "installFail", "crossed", "never", "versionAlone"] {
                let one = at(state);
                assert_eq!(one["shown"], true, "{width} {state}: the box is not shown");
                assert_eq!(one["fits"], true, "{width} {state}: the box does not fit: {one}");
                assert_eq!(one["sideways"], false, "{width} {state}: the box scrolls sideways");
                assert_eq!(flat(&one["version"]), format!("Version {version}"), "{width} {state}");
                // **Z32's *Install now* is the held form's, on every system**, where
                // the act behind it exists; every other reading has *Close*
                // alone.
                let buttons = if state == "held" && cfg!(any(target_os = "macos", target_os = "linux", target_os = "windows")) {
                    let install_now = if cfg!(target_os = "linux") {
                        words::INSTALL_NOW_LINUX
                    } else {
                        words::INSTALL_NOW
                    };
                    serde_json::json!(["Close", Prose::new(install_now).as_str()])
                } else if state == "new" && cfg!(target_os = "linux") {
                    serde_json::json!(["Close", Prose::new(words::INSTALL).as_str()])
                } else {
                    serde_json::json!(["Close"])
                };
                assert_eq!(one["buttons"], buttons, "{width} {state}: a button with nothing behind it");
                assert_eq!(one["focused"], "Close", "{width} {state}");
            }
            let lines = |state: &str| at(state)["lines"].clone();

            // The first reading: the moment, written as a moment.
            let idle = at("idle");
            assert_eq!(lines("idle"), serde_json::json!(["idle"]), "{width}");
            let moment = flat(&idle["moment"]);
            assert!(!moment.is_empty(), "{width}: no moment");
            assert!(!moment.contains("ago") && moment.contains(':'), "{width}: {moment:?} is not a moment");
            assert_eq!(
                flat(&idle["line"]),
                format!(
                    "{} {moment}. {}",
                    plain(words::UP_TO_DATE_LAST_CHECKED),
                    plain(words::SMOOZY_CHECKS_ON_ITS_OWN)
                ),
                "{width}"
            );

            // The second: *Checking…*, the line carrying the turning mark.
            assert_eq!(lines("checking"), serde_json::json!(["check"]), "{width}");
            assert_eq!(at("checking")["turning"], true, "{width}: no turning mark");
            assert_eq!(flat(&at("checking")["line"]), plain(words::CHECKING));

            // The third, and its held form.
            let ready = format!("{} 0.2.0 {}", plain(words::BEFORE_THE_NEW_VERSION), plain(words::IS_READY));
            assert_eq!(lines("new"), serde_json::json!([if cfg!(target_os = "linux") { "new-linux" } else { "new" }]));
            assert_eq!(flat(&at("new")["bold"]), ready, "{width}");
            if !cfg!(target_os = "linux") {
                assert_eq!(flat(&at("new")["line"]), format!("{ready} {}", plain(words::RESTARTS_TO_INSTALL)));
            }
            if cfg!(any(target_os = "macos", target_os = "linux", target_os = "windows")) {
                assert_eq!(lines("held"), serde_json::json!(["held", "held"]), "{width}");
                assert_eq!(
                    flat(&at("held")["second"]),
                    format!(
                        "{} {} {}",
                        plain(words::INSTALLING_NOW_RESTARTS),
                        plain(words::JOBS_KEEP_RENDERING),
                        plain(words::TRANSFERS_STOP)
                    ),
                    "{width}: the sentence above *Install now* is not the drawing's"
                );
            } else {
                assert_eq!(lines("held"), serde_json::json!(["held"]), "{width}");
            }
            assert_eq!(flat(&at("held")["bold"]), ready, "{width}");
            assert_eq!(
                flat(&at("held")["line"]),
                format!("{ready} {}", plain(words::IT_WILL_INSTALL_WHEN_NOTHING_RUNS))
            );

            // The fourth: the failure shape, naming no cause.
            assert_eq!(lines("fail"), serde_json::json!(["fail"]), "{width}");
            assert_eq!(at("fail")["dot"], true, "{width}: no red dot");
            assert_eq!(flat(&at("fail")["bold"]), plain(words::COULD_NOT_REACH));
            assert_eq!(flat(&at("fail")["line"]), plain(words::COULD_NOT_REACH));
            assert_eq!(lines("installFail"), serde_json::json!(["install-fail"]), "{width}");
            assert_eq!(at("installFail")["dot"], true, "{width}: installation failure has no red dot");
            assert_eq!(flat(&at("installFail")["line"]), plain("Update installation failed."));
            assert_eq!(flat(&at("installFail")["bold"]), plain("Update installation failed."));

            // A reading never opens a closed box; a crossing going draws the
            // newer reading.
            assert_eq!(out["whileShut"], true, "{width}: a reading opened a closed box");
            assert_eq!(lines("crossed"), serde_json::json!(["idle"]), "{width}: the older reading was drawn");

            // A copy that never checks, and the version alone.
            assert_eq!(lines("never"), serde_json::json!(["never"]), "{width}");
            assert_eq!(flat(&at("never")["line"]), plain(words::THIS_COPY_NEVER_CHECKS));
            assert_eq!(lines("versionAlone"), serde_json::json!([]), "{width}: something invented");
            let linux = &out["linux"];
            assert_eq!(
                linux["fits"], true,
                "{width}: Linux update box does not fit"
            );
            assert_eq!(
                linux["passwordFields"], 0,
                "{width}: Smoozy must never ask for a password"
            );
            assert_eq!(flat(&linux["heading"]), "Smoozy 0.2.0 is ready to install");
            assert_eq!(
                linux["buttons"],
                serde_json::json!([Prose::new("Not now").as_str(), "Install"])
            );
            assert_eq!(flat(&linux["password"]), Prose::new("Your system asks for your computer password to install.").as_str().replace('\u{a0}', " "));
            assert_eq!(
                out["linuxChoices"],
                serde_json::json!([
                    {"version":"0.2.0", "action":"decline"},
                    {"version":"0.2.0", "action":"decline"},
                    {"version":"0.2.0", "action":"install"}
                ])
            );

            // **Step 11's D6b: the rollback box** — the heading, the password
            // sentence, the sentence saying Smoozy cannot open until then,
            // Quit and Reinstall; no password field; Escape closes nothing;
            // Reinstall takes the underway shape and a second press sends
            // nothing; a failed attempt draws *Reinstall failed.* in the
            // failure shape; and only the restore left says nothing about a
            // password.
            let password = plain("Your system asks for your computer password to reinstall.");
            let until = plain("Reinstall the previous version to open Smoozy.");
            let buttons = serde_json::json!([Prose::new("Quit").as_str(), Prose::new("Reinstall").as_str()]);
            for (state, failed, lines) in [
                ("rollback", false, vec![password.clone(), until.clone()]),
                ("rollbackFailed", true, vec![password.clone(), until.clone()]),
                ("rollbackCleanup", true, vec![until.clone()]),
            ] {
                let one = &out[state];
                assert_eq!(one["fits"], true, "{width} {state}: the rollback box does not fit: {one}");
                assert_eq!(one["boxes"], 1, "{width} {state}: one box at a time");
                assert_eq!(one["passwordFields"], 0, "{width} {state}: Smoozy must never ask for a password");
                assert_eq!(flat(&one["heading"]), "Smoozy 0.3.0 could not start", "{width} {state}");
                let drawn: Vec<String> = one["lines"].as_array().unwrap().iter().map(|line| flat(line)).collect();
                assert_eq!(drawn, lines, "{width} {state}");
                assert_eq!(one["buttons"], buttons, "{width} {state}");
                if failed {
                    assert_eq!(flat(&one["failed"]), plain("Reinstall failed."), "{width} {state}");
                    assert_eq!(one["dot"], true, "{width} {state}: no red dot");
                } else {
                    assert!(one["failed"].is_null(), "{width} {state}: a failure drawn before any attempt");
                    assert_eq!(flat(&one["focused"]), plain("Reinstall"), "{width}: Reinstall is not focused");
                }
            }
            assert_eq!(out["rollbackAfterEscape"], 1, "{width}: Escape closed the rollback box");
            assert_eq!(out["rollbackUnderway"], serde_json::json!({"mark": true, "pressable": "true"}), "{width}");
            assert_eq!(
                out["rollbackChoices"],
                serde_json::json!([
                    {"version":"0.3.0", "action":"reinstall"},
                    {"version":"0.3.0", "action":"quit"}
                ]),
                "{width}: the rollback box's answers"
            );
        }

        if let Some(keep) = std::env::var_os("SMOOZY_THE_ABOUT_PAGE") {
            let keep = std::path::PathBuf::from(keep);
            let _ = fs::create_dir_all(&keep);
            for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
                if entry.path().is_file() {
                    let _ = fs::copy(entry.path(), keep.join(entry.file_name()));
                }
            }
        }
    }

    /// **Step 11's D6b: the rollback box says what happened and what the user
    /// can do**, every word through the prose route and the version a value.
    /// The password sentence is there only where the system will be asked,
    /// and *Reinstall failed.* only after an attempt that did not finish.
    #[test]
    fn d6b_the_rollback_box_says_what_happened_and_what_can_be_done() {
        let pairs = |event: Event| -> Vec<(String, String)> {
            let delivered = delivered(|queue| {
                let _ = queue.read_then_post(|| Ok::<_, ()>(()), |_| event.clone());
            });
            assert_eq!(delivered["kind"], THE_LINUX_ROLLBACK_BOX);
            delivered["said"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(key, said)| (key.clone(), said.as_str().unwrap().replace('\u{a0}', " ")))
                .collect()
        };
        let asking = pairs(the_rollback_box("0.3.0", true, false));
        let keys: Vec<&str> = asking.iter().map(|(key, _)| key.as_str()).collect();
        assert_eq!(keys, ["afterVersion", "beforeVersion", "password", "quit", "reinstall", "until", "version"]);
        let get = |said: &[(String, String)], key: &str| said.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        assert_eq!(get(&asking, "version").as_deref(), Some("0.3.0"));
        assert_eq!(get(&asking, "afterVersion").as_deref(), Some("could not start"));
        assert!(get(&asking, "failed").is_none());
        let failed = pairs(the_rollback_box("0.3.0", true, true));
        assert_eq!(get(&failed, "failed").as_deref(), Some("Reinstall failed."));
        let cleanup = pairs(the_rollback_box("0.3.0", false, true));
        assert!(get(&cleanup, "password").is_none(), "a password was mentioned where none will be asked for");
        assert_eq!(get(&cleanup, "until").as_deref(), Some("Reinstall the previous version to open Smoozy."));
    }

    /// The page About is drawn in: the application's stylesheets, a stand-in
    /// for the shell, and a script that tells the box each reading.
    const THE_ABOUT_PAGE: &str = r##"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
</head><body><div class="app"><div class="pane" id="pane"></div></div><pre id="out"></pre>
<script>
  window.__TAURI__ = {
    core: { invoke(command, args) { if(command === "a_menu_item_was_chosen") { window.__sourceChoices.push({command, args}); return Promise.resolve(window.__sourceRefused ? "ItCouldNotBeOpened" : "ItWasCarriedOut"); } if(args && args.linux) window.__linuxChoices.push(args.linux); return Promise.resolve(command === "the_menu_bar" ? { drawn_by_the_application: false } : null); } },
    event: { listen(name, back) { window.__listener = back; return Promise.resolve(() => {}); } },
  };
  window.__troubles = [];
  window.__linuxChoices = [];
  window.__sourceChoices = [];
  window.__sourceRefused = false;
  console.error = (...said) => window.__troubles.push(said.map((one) => String(one && one.stack || one)).join(" "));
</script>
<script type="module">
import { theRouteIsOpen } from "./event-route.js";
import "./menus.js";
import deliveries from "./deliveries.js";
const out = { states: {} };
const write = () => { out.troubles = window.__troubles; document.getElementById("out").textContent = JSON.stringify(out); };
window.addEventListener("error", (event) => { out.error = String(event.message); write(); });
window.addEventListener("unhandledrejection", (event) => { out.error = String(event.reason && event.reason.stack || event.reason); write(); });
try {
  await theRouteIsOpen();
  let sequence = 0;
  const settle = async () => { for (let at = 0; at < 6; at += 1) await new Promise((go) => setTimeout(go, 0)); };
  const tell = async (delivery) => {
    const one = structuredClone(delivery);
    sequence += 1;
    one.sequence = 1000 + sequence;
    window.__listener({ payload: one });
    await settle();
  };
  const shown = (el) => Boolean(el) && !el.closest("[hidden]") && el.getClientRects().length > 0;
  const root = () => document.querySelector(".modal.top");
  const look = (state) => {
    const b = root().querySelector(".mbox");
    const rect = b.getBoundingClientRect();
    const lines = [...b.querySelectorAll("p[data-ab]")].filter(shown);
    const line = lines[0];
    out.states[state] = {
      shown: shown(b),
      fits: rect.left >= 0 && rect.top >= 0 && rect.right <= window.innerWidth && rect.bottom <= window.innerHeight,
      sideways: b.scrollWidth > b.clientWidth || document.documentElement.scrollWidth > window.innerWidth,
      version: b.querySelector("h3 + p").textContent,
      lines: lines.map((one) => one.dataset.ab),
      line: line ? line.textContent : "",
      second: lines[1] ? lines[1].textContent : "",
      bold: line && line.querySelector("b") ? line.querySelector("b").textContent : "",
      moment: line && line.querySelector(".ab-at") ? line.querySelector(".ab-at").textContent : "",
      turning: Boolean(line && line.querySelector(".ckdot") && shown(line.querySelector(".ckdot"))),
      dot: Boolean(line && line.querySelector(".fdot")),
      buttons: [...b.querySelectorAll(".mact button")].filter(shown).map((one) => one.textContent),
      focused: document.activeElement ? document.activeElement.textContent : "",
    };
  };
  await tell(deliveries.idle); look("idle");
  out.notices = {
    copyright: root().querySelector(".ab-copyright")?.textContent || "",
    notice: root().querySelector(".ab-notice")?.textContent || "",
    links: [...root().querySelectorAll(".ab-links button")].map(el => el.textContent),
  };
  out.readers = [];
  for (const [index, key] of [[0, "licenceText"], [1, "thirdPartyText"]]) {
    root().querySelectorAll(".ab-links button")[index].click();
    const reader = root().querySelector(".ab-text");
    const box = reader.closest(".mbox");
    const rect = box.getBoundingClientRect();
    out.readers.push({ inside: shown(reader) && root().querySelectorAll(".mbox").length === 1,
      exact: reader.textContent === deliveries.idle.said[key],
      selectable: getComputedStyle(reader).userSelect !== "none",
      fits: rect.left >= 0 && rect.right <= innerWidth && rect.top >= 0 && rect.bottom <= innerHeight && box.scrollWidth <= box.clientWidth });
    if (index === 0) {
      box.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true }));
    } else {
      box.querySelector(".mact button").click();
    }
    out.readers[index].returns = reader.hidden && !root().hidden;
  }
  root().querySelectorAll(".ab-links button")[2].click(); await settle();
  window.__sourceRefused = true;
  root().querySelectorAll(".ab-links button")[2].click(); await settle();
  out.source = { choices: window.__sourceChoices,
    failure: root().querySelector(".ax-bad:not([hidden])")?.textContent || "" };
  await tell(deliveries.checking); look("checking");
  await tell(deliveries.new); look("new");
  await tell(deliveries.held); look("held");
  await tell(deliveries.fail); look("fail");
  await tell(deliveries.installFail); look("installFail");
  [...root().querySelectorAll(".mact button")].find(shown).click();
  await tell(deliveries.whileShut);
  out.whileShut = root().hidden;
  await tell(deliveries.crossed); look("crossed");
  await tell(deliveries.never); look("never");
  await tell(deliveries.versionAlone); look("versionAlone");
  await tell(deliveries.linux);
  let linux = document.querySelector(".linux-update .mbox");
  const rect = linux.getBoundingClientRect();
  out.linux = {
    fits: rect.left >= 0 && rect.right <= innerWidth && rect.top >= 0 && rect.bottom <= innerHeight && linux.scrollWidth <= linux.clientWidth,
    heading: linux.querySelector("h3").textContent,
    password: linux.querySelector("p").textContent,
    passwordFields: linux.querySelectorAll("input").length,
    buttons: [...linux.querySelectorAll("button")].map(b => b.textContent)
  };
  linux.querySelector("button").click();
  await tell(deliveries.linux);
  linux = document.querySelector(".linux-update .mbox");
  linux.dispatchEvent(new KeyboardEvent("keydown", { key:"Escape", bubbles:true, cancelable:true }));
  await tell(deliveries.linux);
  linux = document.querySelector(".linux-update .mbox");
  linux.querySelectorAll("button")[1].click();
  out.linuxChoices = window.__linuxChoices;
  window.__linuxChoices = [];
  const theRollbackBox = () => document.querySelector(".linux-rollback .mbox");
  const lookAtTheRollback = () => {
    const b = theRollbackBox();
    const r = b.getBoundingClientRect();
    const failed = b.querySelector("p.ax-bad");
    return {
      fits: r.left >= 0 && r.right <= innerWidth && r.top >= 0 && r.bottom <= innerHeight && b.scrollWidth <= b.clientWidth,
      boxes: document.querySelectorAll(".linux-rollback").length,
      heading: b.querySelector("h3").textContent,
      lines: [...b.querySelectorAll("p.msub")].map((one) => one.textContent),
      failed: failed ? failed.textContent : null,
      dot: Boolean(failed && failed.querySelector(".fdot")),
      passwordFields: b.querySelectorAll("input").length,
      buttons: [...b.querySelectorAll("button")].map((one) => one.textContent),
      focused: document.activeElement ? document.activeElement.textContent : "",
    };
  };
  await tell(deliveries.rollback);
  out.rollback = lookAtTheRollback();
  theRollbackBox().dispatchEvent(new KeyboardEvent("keydown", { key:"Escape", bubbles:true, cancelable:true }));
  await settle();
  out.rollbackAfterEscape = document.querySelectorAll(".linux-rollback").length;
  const reinstall = theRollbackBox().querySelectorAll("button")[1];
  reinstall.click();
  reinstall.click();
  out.rollbackUnderway = { mark: reinstall.classList.contains("uway") && Boolean(reinstall.querySelector(".spin")), pressable: reinstall.getAttribute("aria-disabled") };
  await tell(deliveries.rollbackFailed);
  out.rollbackFailed = lookAtTheRollback();
  theRollbackBox().querySelectorAll("button")[0].click();
  await tell(deliveries.rollbackCleanup);
  out.rollbackCleanup = lookAtTheRollback();
  out.rollbackChoices = window.__linuxChoices;
} catch (trouble) {
  out.error = String(trouble && trouble.stack || trouble);
}
write();
</script></body></html>"##;

    /// **One item, one act, one definition** — the native bar and the page both
    /// arrive at the same function.
    ///
    /// The check is over this file's own text, because the fault it catches is a
    /// second route being added beside the first and working perfectly on the
    /// one system whoever added it could run.
    #[test]
    fn the_native_bar_and_the_page_share_the_one_route_in() {
        let (working, _) = include_str!("menus.rs")
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");
        assert_eq!(
            working.matches("pub fn the_item_was_chosen").count(),
            1,
            "there is more than one route from a chosen item to an act"
        );
        assert!(
            working.contains("the_item_was_chosen(&app, &queue, &item)"),
            "the page's command no longer goes through the one route, so an act \
             can mean one thing on a Mac and another on Windows"
        );
    }
}

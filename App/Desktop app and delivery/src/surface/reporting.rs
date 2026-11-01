//! **The report box: its two ways in, the write and the reveal** —
//! `Reporting a problem` §7 and §8, built at step 11's piece R4. The page half
//! is `web/reporting.js`.
//!
//! # What the user meets
//!
//! **One box, in the readings the drawing draws** (`Preparation/Design/Mockup/
//! Smoozy.html`, the `rp-…` states): *asking*, with the folder the system
//! proposes or with *No folder chosen* where it proposes none; *while the
//! report is prepared*; and *saved*, in two forms — the folder opened, or the
//! folder that did not open. **Every word the user reads is in [`words`] and
//! is the drawing's own**, with the one exception that module names and says
//! why.
//!
//! # The two ways in, and what each hands over
//!
//! `Reporting a problem` X1: **two entry points open one act and differ only
//! in what they hand it.**
//!
//! - **The Smoozy menu's *Report a problem…* hands it nothing**
//!   (`Reporting a problem` X2, `Reporting a problem` X3). `crate::surface::menus`
//!   opens the box through [`the_box_opens`] with [`TheOpening::TheMenu`]; the
//!   slice is this run and the one before, and no machine is touched.
//! - **Every failure's *Report a problem…* hands it M33's handover**
//!   (`Reporting a problem` X4): the record's kind, its condition identity or
//!   subject outcome mark, its durable subject, the presented sentence and the
//!   moment. [`the_report_box_is_opened_from_a_failure`] reads it back, and
//!   **the mapping from record to slice is `Reporting a problem` X4's, applied
//!   here** ([`the_slice_for`]): a job record or a history record is the
//!   failed-job slice, keyed by that job, and every other subject is this run
//!   and the one before, with the failure named.
//!
//! **The box names the failure it came from** (`Reporting a problem` X5): the
//! presented sentence, the subject's own label and the moment, drawn from the
//! handover and never recomposed, and the same three go into the file's head.
//!
//! # The press, and what happens after it, in order
//!
//! `Reporting a problem` V5a's order, and the press is the explicit direction
//! W4 requires (`Reporting a problem` V4):
//!
//! 1. **The folder.** The downloads-folder proposal (§8, W1), or the folder the
//!    user chose through the picker (K11, `Reporting a problem` V2). **Where
//!    the route returns nothing, nothing is proposed** (W5): the row reads *No
//!    folder chosen*, and *Save report* opens the picker first — so the file
//!    goes only into a folder the user chose.
//! 2. **The box says the report is being prepared**, and nothing claims a file
//!    exists until the re-check says so.
//! 3. **The machine take**, for a report naming a job (`Reporting a problem`
//!    G1) — R3a's [`crate::the_report_take`], with the captured streams'
//!    recorded outcomes read from the local record beside it.
//! 4. **The file is composed** by `smoozy_reporting::report_file::compose` and
//!    **written** (`Reporting a problem` V6): staged whole beside the final
//!    name and forced to the device, then published by C22's create-intended
//!    publication — the exclusive create of the final name, then the rename
//!    over it. **A taken name is never overwritten and never prompted over**
//!    (`Reporting a problem` V5): it is composed again with the next suffix
//!    and judged again. **F1's re-check by path is what reports the outcome.**
//! 5. **The *report written* act line** (`Reporting a problem` V9, §11's own
//!    declaration), through the diary service.
//! 6. **The reveal** (K20, `Reporting a problem` V7) — the file selected in
//!    its folder — **and the saved reading is true to what it reported**
//!    (`Reporting a problem` V8): the *just opened* sentence only where the
//!    reveal said it opened. Nothing retries it and no failure record is
//!    raised for it.
//!
//! **Smoozy points at the file and never at a mail program**
//! (`Reporting a problem` V10): the address is drawn and nothing more.
//!
//! # The seam
//!
//! **The proposal and the reveal reach the system, and both are seams here** so
//! that the whole write and reveal can be driven against a throwaway folder:
//! the proposal is handed in as a value ([`the_box_opens`]), and the reveal is
//! [`TheRoute`]. **No test reaches the user's own folders**, and the system's
//! own routes are each tested in their own files (`src/platform/
//! downloads_folder.rs`, `src/reveal.rs`).
//!
//! # Where the box's state is held
//!
//! **Run-held, in process memory, and nowhere durable** (`State, identity and
//! recovery` §11.1): which way the box was opened, the folder, and how far the
//! press has got. A box cancelled while the report is being prepared writes
//! nothing — the press is checked again before the write.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Reporting a problem` J2a, J6, N3, U7, V5a.
//!
//! **Every other marker of `Reporting a problem` is written out with that
//! document at the site**, because this crate's own plan defines clauses of
//! the same names (X1 to X5, V1 to V10, G1 to G5, S1 to S7).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tauri::{AppHandle, Manager, Runtime, State};

use smoozy_reporting::report::{Slice, CEILING_BYTES};
use smoozy_reporting::report_file::{
    self, ARecordedCollection, OpenedFrom, RecordedCollection, TheNamingLine, TheStream,
    WhatTheMachineAdds, WhatTheReportIsMadeFrom,
};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::Clock;

use crate::diary_service::{ModulesDiary, TheDiaryReadBack};
use crate::events::{Event, Posted, Said, Subject, TheOneQueue};
use crate::picker::{self, Asking, HowMany, Picked, WhatMayBeChosen};
use crate::platform::collision::{self, WhyNothingWasPublished};
use crate::platform::downloads_folder::TheProposal;
use crate::platform::files::{self, Durability, Recheck};
use crate::record::failure_records::{DurableSubject, Kind};
use crate::reveal::Revealed;
use crate::start::Started;
use crate::the_report_take::TheTake;
use crate::typography::{Prose, Verbatim};

/// **The kind the page listens for: the box, in whichever reading it is in.**
///
/// Declared here, beside the service that posts it, and copied into
/// `web/reporting.js`. **One kind for every reading**, because the box is one
/// thing and each event says the whole of what it shows.
pub const THE_REPORT_BOX: &str = "reporting:the-report-box";

/// **The name `Reporting a problem` §11's declaration is registered under**
/// (`crate::start`), and the module the *report written* line is written as.
pub const THE_MODULE: &str = "Reporting a problem";

/// **The menu item that opens the box with nothing handed over** —
/// `crate::menus`'s own identifier for it.
pub const THE_MENU_ITEM: &str = "smoozy.report-a-problem";

/// **The address the saved reading names** — `Specs/Reporting a problem`,
/// Creator-stated, and drawn on the box. A value: the user copies it.
pub const THE_ADDRESS: &str = "hello@smoozy.app";

/// **Every word the box says, and nowhere else.**
///
/// **Each is the drawing's own, character for character** — `Smoozy.html`'s
/// `#rpbox` — save the one marked below. **Each sentence is its own string**
/// (X10), so the paragraph under the field is three of them.
pub(crate) mod words {
    pub const CRASH_HEADING: &str = "Smoozy closed unexpectedly last time";
    pub const NOTHING_WAS_LOST: &str = "Nothing was lost.";
    pub const IF_YOU_REPORT_IT: &str = "A report includes what Smoozy was doing when it closed.";
    pub const NOT_NOW: &str = "Not now";
    pub const REPORT_IT: &str = "Report it…";
    pub const HEADING: &str = "Report a problem";
    pub const WHAT_WENT_WRONG: &str = "What went wrong?";
    pub const THE_FILE_HOLDS: &str = "The report contains Smoozy’s log, your servers’ replies, and the names of files, folders, scenes, servers and users.";
    /// The free-software drawing’s own sentence, 2026-10-27.
    pub const NEVER_YOUR_KEYS: &str = "Never your keys.";
    pub const SMOOZY_SENDS_NOTHING: &str = "You send it yourself.";
    pub const SAVE_IN: &str = "Save in";
    pub const NO_FOLDER_CHOSEN: &str = "No folder chosen";
    pub const CHOOSE: &str = "Choose…";
    pub const CANCEL: &str = "Cancel";
    pub const SAVE_THE_REPORT: &str = "Save report";
    /// The underway control's name for anyone who cannot see it — the
    /// drawing's `aria-label` on *Save report* while the report is
    /// prepared.
    pub const PREPARING_THE_REPORT: &str = "Preparing report";
    pub const THE_REPORT_IS_SAVED: &str = "Report saved";
    /// After the file's name, where the folder is the downloads folder the
    /// system proposed and the reveal said it opened.
    pub const IS_IN_YOUR_DOWNLOADS_FOLDER_WHICH_SMOOZY_HAS_JUST_OPENED: &str =
        "is in your Downloads folder, which has just opened.";
    /// After the file's name, before the folder's path — the drawing's
    /// not-opened sentence, and the opened one where the folder is another.
    pub const IS_IN: &str = "is in";
    /// After a folder that is not the downloads folder, where the reveal said
    /// it opened — the drawing's own clause, after the path the drawing names
    /// in its other saved reading.
    pub const WHICH_SMOOZY_HAS_JUST_OPENED: &str = ", which has just opened.";
    pub const FULL_STOP: &str = ".";
    pub const SMOOZY_COULD_NOT_OPEN_THE_FOLDER: &str = "Could not open the folder.";
    pub const ATTACH_IT_TO_AN_EMAIL_TO: &str = "Attach it to an email to";
    pub const CLOSE: &str = "Close";
    /// **The one sentence the drawing does not hold.** A report the write did
    /// not land is a case the drawing never drew, and Rule 8 makes a box that
    /// goes quiet there — or says *saved* — a fault. **Ruled at this site** in
    /// the short form the app's failures use (*Could not read settings.*), and
    /// owed to Design.
    pub const THE_REPORT_WAS_NOT_SAVED: &str = "Could not save the report.";
}

// ── What the box was opened with ────────────────────────────────────────────

/// **Which way the box was opened** — `Reporting a problem` X1's two.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheOpening {
    /// `Reporting a problem` X2: nothing handed over.
    TheMenu,
    /// `Reporting a problem` C3: the mark's run, never a failure record.
    ACrash { run: Identity, naming: Option<TheNamingLine> },
    /// `Reporting a problem` X4: a failure's handover, read back.
    AFailure(TheHandover),
}

/// **M33's handover, read back from the page and judged** (H34), with the
/// subject's own label read from the local record beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheHandover {
    pub kind: Kind,
    /// The condition identity, or the subject outcome mark.
    pub identity_or_mark: String,
    pub subject: DurableSubject,
    /// **The sentence M32 captured, before the point** — never recomposed.
    pub the_presented_sentence: String,
    /// The moment the handover carried, on this computer's clock.
    pub at: SystemTime,
    /// **The subject's own label**, where the subject has one the record
    /// holds — a server's name, a key's name, a job's name. `Reporting a
    /// problem` X5's naming line carries it beside the sentence.
    pub the_subjects_label: Option<String>,
}

/// **The longest presented sentence a handover may carry.** It is Smoozy's own
/// sentence and is short; anything longer did not come from a failure shape.
const THE_LONGEST_PRESENTED_SENTENCE: usize = 4096;

/// **The longest mark a handover may carry**, for the same reason.
const THE_LONGEST_MARK: usize = 1024;

impl TheHandover {
    /// **The page's handover, judged before anything is touched** — H34: the
    /// kind is one of the two, the subject one of `State, identity and
    /// recovery` §8.7's closed list read back as itself, the sentence present
    /// and bounded. **Anything else is refused** and the caller says what it
    /// does instead.
    pub fn read_back(
        kind: &str,
        identity_or_mark: Option<&str>,
        durable_subject: Option<&str>,
        subject_kind: Option<&str>,
        the_presented_sentence: Option<&str>,
        at: Option<u64>,
        the_moment_of_the_press: SystemTime,
    ) -> Option<TheHandover> {
        let kind = Kind::from_text(kind)?;
        let subject = crate::surface::the_failure_shape::a_subject(
            subject_kind?,
            durable_subject.unwrap_or(""),
        )?;
        let sentence = the_presented_sentence?;
        if sentence.trim().is_empty() || sentence.len() > THE_LONGEST_PRESENTED_SENTENCE {
            return None;
        }
        let mark = identity_or_mark.unwrap_or("");
        if mark.len() > THE_LONGEST_MARK {
            return None;
        }
        // **A condition's mark is its condition identity**, so it must read
        // back as one; an ending's is the subject outcome, which is text.
        if kind == Kind::Condition && Identity::read_back(IdentityKind::Condition, mark).is_none() {
            return None;
        }
        Some(TheHandover {
            kind,
            identity_or_mark: mark.to_string(),
            subject,
            the_presented_sentence: sentence.to_string(),
            // **Where the presentation carried no moment, the moment the user
            // pressed the control stands in** — the moment they were looking
            // at the failure, which is what the naming line is for.
            at: at
                .map(|seconds| UNIX_EPOCH + Duration::from_secs(seconds))
                .unwrap_or(the_moment_of_the_press),
            the_subjects_label: None,
        })
    }

    /// **The identities the *report written* line carries** — §11: the
    /// condition identity where the record is a condition, and the durable
    /// subject's identity where it has one. An ending's mark is text, not an
    /// identity, and is not invented into one.
    pub fn the_identities_it_carries(&self) -> Vec<Identity> {
        let mut out = Vec::new();
        if self.kind == Kind::Condition {
            if let Some(condition) = Identity::read_back(IdentityKind::Condition, &self.identity_or_mark) {
                out.push(condition);
            }
        }
        if let Some(identity) = self.subject.identity() {
            out.push(identity.clone());
        }
        out
    }
}

/// **The subject's own label, as the local record holds it** — `Reporting a
/// problem` X5's middle part. **Nothing here composes one**: a server is called
/// what every screen calls it, a job what its entry calls it, a key by its own
/// name. The other subjects on §8.7's list have no label of their own.
pub fn the_subjects_own_label(
    connection: &rusqlite::Connection,
    subject: &DurableSubject,
) -> Option<String> {
    match subject {
        DurableSubject::ServerRow(server) => crate::record::server_rows::all(connection)
            .ok()?
            .into_iter()
            .find(|row| row.local_server_identifier == server.as_text())
            .map(|row| crate::surface::servers::what_a_server_is_called(&row)),
        DurableSubject::KeyRow(key) => crate::record::key_rows::all(connection)
            .ok()?
            .into_iter()
            .find(|row| &row.identifier == key)
            .map(|row| match &row.name {
                Some(name) => name.as_str().to_string(),
                None => row
                    .path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_default(),
            })
            .filter(|label| !label.is_empty()),
        DurableSubject::JobRecord(job) | DurableSubject::HistoryRecord(job) => {
            crate::record::job_records::JobRecords::over(connection)
                .read(job)
                .ok()
                .flatten()
                .map(|record| crate::surface::the_queue::what_the_job_is_called(&record))
                .filter(|label| !label.is_empty())
        }
        DurableSubject::TransferArrangement(_)
        | DurableSubject::OutcomeRecord(_)
        | DurableSubject::LicenceRecord => None,
    }
}

/// **`Reporting a problem` X4's mapping from record to slice, fixed there and
/// applied here.** A job record or a history record is the failed-job slice,
/// keyed by that job; every other subject, and the menu, is this run and the
/// one before (`Reporting a problem` S2).
pub fn the_slice_for(opening: &TheOpening, this_run: &Identity) -> Slice {
    if let TheOpening::ACrash { run, .. } = opening {
        return Slice::ACrash { run: run.clone() };
    }
    match the_job_it_names(opening) {
        Some(job) => Slice::AFailedJob { job },
        None => Slice::NothingInParticular {
            this_run: this_run.clone(),
        },
    }
}

/// The job the report names, where it names one — which is also whether
/// `Reporting a problem` G1's machine take applies at all.
pub fn the_job_it_names(opening: &TheOpening) -> Option<Identity> {
    match opening {
        TheOpening::AFailure(handover) => match &handover.subject {
            DurableSubject::JobRecord(job) | DurableSubject::HistoryRecord(job) => Some(job.clone()),
            _ => None,
        },
        TheOpening::TheMenu | TheOpening::ACrash { .. } => None,
    }
}

/// **What the file's head says the report was opened from** —
/// `Reporting a problem` X5's naming line, the same three things the box shows.
pub fn opened_from(opening: &TheOpening) -> OpenedFrom {
    match opening {
        TheOpening::TheMenu => OpenedFrom::TheMenu,
        TheOpening::ACrash { naming, .. } => OpenedFrom::ACrash { the_naming_line: naming.clone() },
        TheOpening::AFailure(handover) => OpenedFrom::AFailure(TheNamingLine {
            the_presented_sentence: handover.the_presented_sentence.clone(),
            the_subjects_label: handover.the_subjects_label.clone().unwrap_or_default(),
            at: handover.at,
            clock: Clock::ThisComputer,
        }),
    }
}

// ── The box's own state, run-held ───────────────────────────────────────────

/// Where the report will be saved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheFolder {
    /// §8's proposal, as the system named it (W1). **Nothing has been written
    /// there** (W4).
    TheDownloadsFolder(PathBuf),
    /// **A path from the user** (K13), through the picker.
    ChosenByTheUser(PathBuf),
}

impl TheFolder {
    pub fn path(&self) -> &Path {
        match self {
            TheFolder::TheDownloadsFolder(path) | TheFolder::ChosenByTheUser(path) => path,
        }
    }
}

/// How far the box has got.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ThePhase {
    Asking,
    /// `Reporting a problem` V5a: after the press and before the write's own
    /// outcome.
    Composing,
    Saved {
        the_file_name: String,
        the_folder: PathBuf,
        in_the_downloads_folder: bool,
        the_folder_opened: bool,
    },
    /// The write did not land. The box asks again, with the user's sentence
    /// still in the field.
    NotSaved,
}

/// One open box.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TheOpenBox {
    opening: TheOpening,
    folder: Option<TheFolder>,
    /// The downloads folder the system proposed when the box opened, so the
    /// saved reading can say *your Downloads folder* where that is where the
    /// file went.
    the_downloads_folder: Option<PathBuf>,
    phase: ThePhase,
}

/// **The box's state, held for the run.**
#[derive(Debug)]
pub struct TheBoxHeld {
    open: Option<TheOpenBox>,
    /// **Moves at every opening and every closing**, so a press whose box has
    /// since been shut — or shut and opened again — writes nothing and posts
    /// nothing.
    generation: u64,
}

impl TheBoxHeld {
    pub const fn new() -> Self {
        TheBoxHeld {
            open: None,
            generation: 0,
        }
    }
}

impl Default for TheBoxHeld {
    fn default() -> Self {
        Self::new()
    }
}

/// **The one box this run has.** One box, because it is modal: nothing else
/// can be pressed while it is up.
pub static THE_BOX: Mutex<TheBoxHeld> = Mutex::new(TheBoxHeld::new());

fn held(the_box: &Mutex<TheBoxHeld>) -> std::sync::MutexGuard<'_, TheBoxHeld> {
    the_box.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// **The box opens, asking.** Both ways in arrive here.
///
/// `proposal` is §8's route's answer, taken by the caller — the seam that keeps
/// a test out of the user's own folders.
pub fn the_box_opens(
    the_box: &Mutex<TheBoxHeld>,
    queue: &TheOneQueue,
    opening: TheOpening,
    proposal: TheProposal,
) -> Result<Posted, ()> {
    let the_downloads_folder = match proposal {
        TheProposal::TheSystemNamesThisFolder(folder) => Some(folder),
        // **W5: the route returned nothing, so nothing is proposed.** The
        // system's own words are not shown.
        TheProposal::TheSystemNamesNoFolder(_) => None,
    };
    let (generation, open) = {
        let mut held = held(the_box);
        held.generation += 1;
        let open = TheOpenBox {
            opening,
            folder: the_downloads_folder.clone().map(TheFolder::TheDownloadsFolder),
            the_downloads_folder,
            phase: ThePhase::Asking,
        };
        held.open = Some(open.clone());
        (held.generation, open)
    };
    the_box_is_said(queue, generation, &open)
}

/// **The user chose a folder through the picker.** The box asks again with it.
pub fn a_folder_was_chosen(
    the_box: &Mutex<TheBoxHeld>,
    queue: &TheOneQueue,
    generation: u64,
    folder: PathBuf,
) -> Result<Posted, ()> {
    let open = {
        let mut held = held(the_box);
        if held.generation != generation {
            return Err(());
        }
        let Some(open) = held.open.as_mut() else {
            return Err(());
        };
        if !matches!(open.phase, ThePhase::Asking | ThePhase::NotSaved) {
            return Err(());
        }
        open.folder = Some(TheFolder::ChosenByTheUser(folder));
        open.clone()
    };
    the_box_is_said(queue, generation, &open)
}

/// **What a press of *Save report* starts from**: the box's generation,
/// the folder where one is held, and the opening. Nothing where no box is open
/// or the box is not asking.
pub fn the_save_begins(the_box: &Mutex<TheBoxHeld>) -> Option<(u64, Option<TheFolder>, TheOpening)> {
    let held = held(the_box);
    let open = held.open.as_ref()?;
    if !matches!(open.phase, ThePhase::Asking | ThePhase::NotSaved) {
        return None;
    }
    Some((held.generation, open.folder.clone(), open.opening.clone()))
}

/// **`Reporting a problem` V5a: the box says the report is being prepared.**
/// False where the box has been shut since the press.
pub fn the_box_is_underway(
    the_box: &Mutex<TheBoxHeld>,
    queue: &TheOneQueue,
    generation: u64,
    folder: TheFolder,
) -> bool {
    let open = {
        let mut held = held(the_box);
        if held.generation != generation {
            return false;
        }
        let Some(open) = held.open.as_mut() else {
            return false;
        };
        open.folder = Some(folder);
        open.phase = ThePhase::Composing;
        open.clone()
    };
    let _ = the_box_is_said(queue, generation, &open);
    true
}

/// **Whether the press that began at `generation` is still the box's** — the
/// question asked again just before the write, so a box cancelled while the
/// report was being prepared writes nothing.
pub fn the_press_still_stands(the_box: &Mutex<TheBoxHeld>, generation: u64) -> bool {
    let held = held(the_box);
    held.generation == generation
        && held
            .open
            .as_ref()
            .is_some_and(|open| open.phase == ThePhase::Composing)
}

/// **The press ended, and the box says what it came to.**
pub fn the_save_ended(
    the_box: &Mutex<TheBoxHeld>,
    queue: &TheOneQueue,
    generation: u64,
    came_to: &WhatTheSaveCameTo,
) -> Result<Posted, ()> {
    let open = {
        let mut held = held(the_box);
        if held.generation != generation {
            return Err(());
        }
        let Some(open) = held.open.as_mut() else {
            return Err(());
        };
        let the_downloads_folder = open.the_downloads_folder.clone();
        open.phase = match came_to {
            WhatTheSaveCameTo::Saved {
                the_file_name,
                the_folder,
                the_folder_opened,
            } => ThePhase::Saved {
                the_file_name: the_file_name.clone(),
                the_folder: the_folder.clone(),
                in_the_downloads_folder: the_downloads_folder.as_deref().is_some_and(|downloads| {
                    downloads == the_folder.as_path()
                        || files::folders_are_the_same(downloads, the_folder) == files::SameFolder::Yes
                }),
                the_folder_opened: *the_folder_opened,
            },
            WhatTheSaveCameTo::NotSaved { .. } => ThePhase::NotSaved,
        };
        open.clone()
    };
    the_box_is_said(queue, generation, &open)
}

/// **The box is shut** — *Cancel* or *Close*. Whatever press is under way
/// writes nothing after this.
pub fn the_box_is_shut_in(the_box: &Mutex<TheBoxHeld>) {
    let mut held = held(the_box);
    held.open = None;
    held.generation += 1;
}

// ── What the page is told ───────────────────────────────────────────────────

/// The saved sentence's parts: up to five, each words or a value, in order.
const THE_SAVED_PARTS: [(&str, &str, &str); 5] = [
    ("saved0Words", "saved0Value", "saved0Clings"),
    ("saved1Words", "saved1Value", "saved1Clings"),
    ("saved2Words", "saved2Value", "saved2Clings"),
    ("saved3Words", "saved3Value", "saved3Clings"),
    ("saved4Words", "saved4Value", "saved4Clings"),
];

/// One part of the saved sentence.
enum APart {
    Words(&'static str),
    /// Words that follow the value before them with no space — a comma or a
    /// full stop.
    WordsThatCling(&'static str),
    Value(String),
}

/// **The saved sentence, in its parts** — `Reporting a problem` V8. The *just
/// opened* clause only where the reveal said it opened.
fn the_saved_sentence(
    the_file_name: &str,
    the_folder: &Path,
    in_the_downloads_folder: bool,
    the_folder_opened: bool,
) -> Vec<APart> {
    let folder = the_folder.to_string_lossy().into_owned();
    match (the_folder_opened, in_the_downloads_folder) {
        (true, true) => vec![
            APart::Value(the_file_name.to_string()),
            APart::Words(words::IS_IN_YOUR_DOWNLOADS_FOLDER_WHICH_SMOOZY_HAS_JUST_OPENED),
        ],
        (true, false) => vec![
            APart::Value(the_file_name.to_string()),
            APart::Words(words::IS_IN),
            APart::Value(folder),
            APart::WordsThatCling(words::WHICH_SMOOZY_HAS_JUST_OPENED),
        ],
        (false, _) => vec![
            APart::Value(the_file_name.to_string()),
            APart::Words(words::IS_IN),
            APart::Value(folder),
            APart::WordsThatCling(words::FULL_STOP),
            // Its own string, being its own sentence (X10).
            APart::Words(words::SMOOZY_COULD_NOT_OPEN_THE_FOLDER),
        ],
    }
}

/// **The box, as one event**: every word through the point, every name, path
/// and figure by the verbatim route (A6, X7, X9), and the reading it is in.
fn the_box_says(generation: u64, open: &TheOpenBox) -> Event {
    let prose = |text: &str| Said::Prose(Prose::new(text));
    let value = |text: &str| Said::Value(Verbatim::new(text));
    let reading = match open.phase {
        ThePhase::Asking => "asking",
        ThePhase::Composing => "composing",
        ThePhase::Saved { .. } => "saved",
        ThePhase::NotSaved => "not-saved",
    };
    let mut event = Event::about(Subject::TheRun, THE_REPORT_BOX)
        .saying("reading", value(reading))
        // **Which opening this is**, so the page empties the field for a box
        // opened anew and keeps the user's sentence across the readings of
        // one.
        .saying("opening", value(&generation.to_string()))
        .saying("heading", prose(words::HEADING))
        .saying("whatWentWrong", prose(words::WHAT_WENT_WRONG))
        .saying("whatTheFileHolds0", prose(words::THE_FILE_HOLDS))
        .saying("whatTheFileHolds1", prose(words::NEVER_YOUR_KEYS))
        .saying("whatTheFileHolds2", prose(words::SMOOZY_SENDS_NOTHING))
        .saying("saveIn", prose(words::SAVE_IN))
        .saying("choose", prose(words::CHOOSE))
        .saying("cancel", prose(words::CANCEL))
        .saying("save", prose(words::SAVE_THE_REPORT))
        .saying("preparing", prose(words::PREPARING_THE_REPORT));
    event = match &open.folder {
        Some(folder) => event.saying("folder", value(&folder.path().to_string_lossy())),
        // **W5: nothing is proposed, and nothing is invented.**
        None => event.saying("noFolderChosen", prose(words::NO_FOLDER_CHOSEN)),
    };
    // `Reporting a problem` X5: the line naming the failure, drawn from the
    // handover. **Absent from the menu**, where there is no failure to name.
    if let TheOpening::AFailure(handover) = &open.opening {
        event = event
            // A screen, so through the point; the diary and the file keep the
            // string as it was captured.
            .saying("aboutSentence", prose(&handover.the_presented_sentence))
            .saying(
                "aboutAt",
                value(
                    &handover
                        .at
                        .duration_since(UNIX_EPOCH)
                        .map(|since| since.as_secs())
                        .unwrap_or(0)
                        .to_string(),
                ),
            );
        if let Some(label) = &handover.the_subjects_label {
            event = event.saying("aboutLabel", value(label));
        }
    }
    if let TheOpening::ACrash { naming: Some(naming), .. } = &open.opening {
        event = event.saying("aboutSentence", prose(&naming.the_presented_sentence))
            .saying("aboutAt", value(&naming.at.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs().to_string()));
    }
    match &open.phase {
        ThePhase::Saved {
            the_file_name,
            the_folder,
            in_the_downloads_folder,
            the_folder_opened,
        } => {
            event = event
                .saying("savedHeading", prose(words::THE_REPORT_IS_SAVED))
                .saying("attachIt", prose(words::ATTACH_IT_TO_AN_EMAIL_TO))
                .saying("address", value(THE_ADDRESS))
                .saying("close", prose(words::CLOSE));
            let parts = the_saved_sentence(
                the_file_name,
                the_folder,
                *in_the_downloads_folder,
                *the_folder_opened,
            );
            for (part, (words_key, value_key, clings_key)) in parts.iter().zip(THE_SAVED_PARTS) {
                event = match part {
                    APart::Words(text) => event.saying(words_key, prose(text)),
                    APart::WordsThatCling(text) => event
                        .saying(words_key, prose(text))
                        .saying(clings_key, value("true")),
                    APart::Value(text) => event.saying(value_key, value(text)),
                };
            }
            event = event.saying("savedCount", value(&parts.len().to_string()));
            if *the_folder_opened {
                event = event.saying("folderOpened", value("true"));
            }
        }
        ThePhase::NotSaved => {
            event = event.saying("notSaved", prose(words::THE_REPORT_WAS_NOT_SAVED));
        }
        ThePhase::Asking | ThePhase::Composing => {}
    }
    event
}

fn the_box_is_said(queue: &TheOneQueue, generation: u64, open: &TheOpenBox) -> Result<Posted, ()> {
    // **The read door** (P7): the box's state is run-held and written nowhere
    // durable, so the event carries nothing that should have been written
    // first.
    queue.read_then_post(|| Ok::<(), ()>(()), |()| the_box_says(generation, open))
}

// ── The route's seam ────────────────────────────────────────────────────────

/// **The reveal, as a seam** — K20's service in the running application, and a
/// stand-in in a test so nothing puts a window on anybody's screen.
pub trait TheRoute {
    fn the_file_is_revealed(&self, file: &Path) -> Revealed;
}

/// **The system's own reveal** — `crate::reveal`, K20 to K24.
pub struct TheSystemsOwnRoute;

impl TheRoute for TheSystemsOwnRoute {
    fn the_file_is_revealed(&self, file: &Path) -> Revealed {
        crate::reveal::a_file_in_its_folder(file)
    }
}

// ── The write ───────────────────────────────────────────────────────────────

/// **What a press of *Save report* is made from.**
#[derive(Debug, Clone)]
pub struct TheSaveRequest {
    /// `Reporting a problem` V1: verbatim.
    pub the_users_sentence: String,
    /// This computer's zone at the press, seconds east of UTC — the name's
    /// moment is the desktop clock's, a presentation use (`Reporting a
    /// problem` V3).
    pub utc_offset_seconds: i32,
    pub opening: TheOpening,
    pub folder: PathBuf,
    pub made_at: SystemTime,
}

/// What the press came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheSaveCameTo {
    /// F1's re-check found the whole file at its name.
    Saved {
        the_file_name: String,
        the_folder: PathBuf,
        /// K22's outcome, as the reveal reported it.
        the_folder_opened: bool,
    },
    /// Nothing claims a file. `reported` is for the diary of whoever reads
    /// this code and is never shown.
    NotSaved { reported: String },
}

/// What the write came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheWriteCameTo {
    Written { the_file_name: String, path: PathBuf },
    NotWritten { reported: String },
}

/// **The most names the write tries before it says it could not.** A folder
/// holding a thousand reports from one minute is not a case anybody meets, and
/// a bound is what keeps a refusal from becoming a loop.
const THE_MOST_NAMES_TRIED: u32 = 1000;

/// **The staged file's name**: hidden, Smoozy's own, and unique — a fresh
/// identity, so it can never be a name the user's own file holds. It stands
/// beside the final name, so the rename is on one filesystem by construction
/// (F15).
fn the_staged_name() -> Option<String> {
    Identity::issue(IdentityKind::Operation)
        .ok()
        .map(|one| format!(".smoozy-report-{}.smoozy-part", one.as_text()))
}

/// **`Reporting a problem` V6 and `Reporting a problem` V5: the file written
/// atomically, under a name that never collides, and re-checked by path.**
///
/// The text is staged whole and forced to the device; then each name in turn
/// is put to C22's publication — the exclusive create of the final name, and
/// the rename over it. **A taken name is left exactly as it is** and the next
/// is composed (C12); any other refusal ends the write. **F1's re-check by path
/// is what reports the outcome**, never the calls' own answers.
pub fn the_file_is_written(text: &str, folder: &Path, made_at: SystemTime, utc_offset_seconds: i32) -> WhatTheWriteCameTo {
    let Some(staged_name) = the_staged_name() else {
        return WhatTheWriteCameTo::NotWritten {
            reported: "this computer would not supply random bytes for the staged name".to_string(),
        };
    };
    let staged = folder.join(staged_name);
    let bytes = text.as_bytes();
    let written = files::write_whole_file(&staged, bytes, Durability::ToTheDevice);
    if !written.wrote_the_file() {
        let _ = files::remove_file(&staged);
        return WhatTheWriteCameTo::NotWritten {
            reported: match written.first_error {
                Some(failure) => failure.reported,
                None => format!("the staged file read back as {:?}", written.recheck),
            },
        };
    }
    for the_try in 1..=THE_MOST_NAMES_TRIED {
        let the_file_name = report_file::the_file_name(made_at, utc_offset_seconds, the_try);
        let final_name = folder.join(&the_file_name);
        match collision::publish_create_intended(&staged, &final_name) {
            Ok(()) => {
                return match files::outside_program_wrote(&final_name) {
                    Recheck::Present { bytes: found } if found == bytes.len() as u64 => {
                        WhatTheWriteCameTo::Written {
                            the_file_name,
                            path: final_name,
                        }
                    }
                    other => WhatTheWriteCameTo::NotWritten {
                        reported: format!("the published file read back as {other:?}"),
                    },
                };
            }
            Err(WhyNothingWasPublished::TheNameIsTaken { .. }) => continue,
            Err(WhyNothingWasPublished::ItWasRefused(failure)) => {
                let _ = files::remove_file(&staged);
                return WhatTheWriteCameTo::NotWritten {
                    reported: failure.reported,
                };
            }
        }
    }
    let _ = files::remove_file(&staged);
    WhatTheWriteCameTo::NotWritten {
        reported: format!("every one of {THE_MOST_NAMES_TRIED} names was taken"),
    }
}

/// **`Desktop app and delivery` M10's recorded outcome, as the report carries
/// it**, from the outcome the local record kept (`Queue & History` E9). A row
/// whose outcome is not written yet (M10a) is none of the four and is left out.
fn as_the_report_carries_it(
    kept: Vec<(u32, crate::presentation::the_machines_own_words::Stream, Option<smoozy_runtime::capture::CollectionOutcome>)>,
) -> Vec<ARecordedCollection> {
    use crate::presentation::the_machines_own_words::Stream;
    use smoozy_runtime::capture::CollectionOutcome;
    kept.into_iter()
        .filter_map(|(attempt, stream, outcome)| {
            Some(ARecordedCollection {
                attempt,
                stream: match stream {
                    Stream::TheOrdinaryOutput => TheStream::StandardOutput,
                    Stream::TheComplaints => TheStream::StandardError,
                },
                outcome: match outcome? {
                    CollectionOutcome::Collected => RecordedCollection::Collected,
                    CollectionOutcome::CollectedAndTheStreamWasEmpty => {
                        RecordedCollection::CollectedAndTheStreamWasEmpty
                    }
                    CollectionOutcome::CollectionIncomplete => RecordedCollection::CollectionIncomplete,
                    CollectionOutcome::NeverEstablished => RecordedCollection::NeverEstablished,
                },
            })
        })
        .collect()
}

/// **What a reachable machine adds, put together for the file** —
/// `Reporting a problem` §6: the take's three parts as they came back, the
/// captured streams' recorded outcomes from the local record, and the crash
/// file's part.
pub fn what_the_machine_adds(take: TheTake, captured: Vec<ARecordedCollection>) -> WhatTheMachineAdds {
    WhatTheMachineAdds {
        the_captured_output: captured,
        the_journal_tail: take.the_journal_tail,
        the_crash_file: take.the_crash_file,
        the_per_job_note: take.the_per_job_note,
    }
}

/// What the save read out of the start report, before anything reached a
/// machine.
struct WhatWasReadFirst {
    the_diary: TheDiaryReadBack,
    this_run: Identity,
    the_job: Option<Identity>,
    captured: Vec<ARecordedCollection>,
}

fn what_is_read_first(started: &Mutex<Started>, opening: &TheOpening) -> Result<WhatWasReadFirst, String> {
    let started = started.lock().map_err(|_| "the start report is poisoned".to_string())?;
    let diary = started
        .diary
        .as_ref()
        .ok_or_else(|| "there is no diary to cut the report from".to_string())?;
    let the_diary = diary.read_back()?;
    let this_run = started
        .run
        .clone()
        .ok_or_else(|| "this run has no identity to cut the report on".to_string())?;
    let the_job = the_job_it_names(opening);
    let captured = match (&the_job, started.record.as_ref()) {
        (Some(job), Some(record)) => as_the_report_carries_it(
            crate::record::kept_words::the_outcomes_kept_for(record.connection(), job)
                .map_err(|failure| format!("the kept outcomes would not read: {failure:?}"))?,
        ),
        _ => Vec::new(),
    };
    Ok(WhatWasReadFirst {
        the_diary,
        this_run,
        the_job,
        captured,
    })
}

/// **The press, from the read to the reveal** — everything after the box has
/// said the report is being prepared, in `Reporting a problem` V5a's order.
///
/// `take` is R3a's take in the running application and a stand-in in a test;
/// `route` is the reveal; `still_wanted` is asked once the take has ended and
/// before anything is written, so a box shut meanwhile writes nothing.
pub fn the_save_is_carried_out(
    started: &Mutex<Started>,
    take: impl FnOnce(Option<&Identity>) -> Option<TheTake>,
    route: &dyn TheRoute,
    still_wanted: impl Fn() -> bool,
    request: &TheSaveRequest,
) -> WhatTheSaveCameTo {
    let read = match what_is_read_first(started, &request.opening) {
        Ok(read) => read,
        Err(reported) => return WhatTheSaveCameTo::NotSaved { reported },
    };
    // `Reporting a problem` G1: only for a report naming a job, and the take
    // itself answers *unreachable* where the job's server is not connected.
    let adds = take(read.the_job.as_ref()).map(|took| what_the_machine_adds(took, read.captured));
    if !still_wanted() {
        return WhatTheSaveCameTo::NotSaved {
            reported: "the box was shut before the report was written".to_string(),
        };
    }
    let opened = opened_from(&request.opening);
    let slice = the_slice_for(&request.opening, &read.this_run);
    let file = report_file::compose(&WhatTheReportIsMadeFrom {
        the_users_sentence: &request.the_users_sentence,
        opened_from: &opened,
        smoozys_version: env!("CARGO_PKG_VERSION"),
        the_system: std::env::consts::OS,
        made_at: request.made_at,
        the_diary: &read.the_diary.lines,
        the_cap_has_dropped_older_lines: read.the_diary.the_cap_has_dropped_older_lines,
        the_slice: &slice,
        what_the_machine_adds: adds.as_ref(),
    });
    let (the_file_name, path) = match the_file_is_written(
        &file.text,
        &request.folder,
        request.made_at,
        request.utc_offset_seconds,
    ) {
        WhatTheWriteCameTo::Written { the_file_name, path } => (the_file_name, path),
        WhatTheWriteCameTo::NotWritten { reported } => return WhatTheSaveCameTo::NotSaved { reported },
    };
    // `Reporting a problem` V9 and §11: the act line, once the file is there.
    if let Ok(started) = started.lock() {
        if let Some(service) = started.diary.as_ref() {
            let identities = match &request.opening {
                TheOpening::AFailure(handover) => handover.the_identities_it_carries(),
                TheOpening::TheMenu => Vec::new(),
                TheOpening::ACrash { run, .. } => vec![run.clone()],
            };
            let line = report_file::the_report_written_line(
                &read.this_run,
                SystemTime::now(),
                &request.folder.to_string_lossy(),
                &the_file_name,
                &identities,
            );
            // **L16: nothing is owed either way.** A line the diary did not
            // take loses the line and nothing else; the file is the user's.
            let _ = ModulesDiary {
                service,
                declarations: &started.declarations,
                run: read.this_run.clone(),
                module: THE_MODULE,
            }
            .write(&line);
        }
    }
    let revealed = route.the_file_is_revealed(&path);
    WhatTheSaveCameTo::Saved {
        the_file_name,
        the_folder: request.folder.clone(),
        the_folder_opened: revealed.it_opened(),
    }
}

// ── The commands ────────────────────────────────────────────────────────────

/// **A failure's *Report a problem…*** — `Reporting a problem` X4: the
/// handover M33 supplies, from whichever failure shape the user pressed it on.
///
/// **H34: the handover is judged before anything is touched.** One that will
/// not read back opens the box as the menu does — with nothing handed over —
/// rather than nothing at all: a control that does nothing and says nothing is
/// the fault Rule 8 names, and the menu's way in works with nothing.
#[tauri::command]
pub fn the_report_box_is_opened_from_a_failure(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, Arc<TheOneQueue>>,
    kind: String,
    identity_or_mark: Option<String>,
    durable_subject: Option<String>,
    subject_kind: Option<String>,
    the_presented_sentence: Option<String>,
    at: Option<u64>,
) {
    let handover = TheHandover::read_back(
        &kind,
        identity_or_mark.as_deref(),
        durable_subject.as_deref(),
        subject_kind.as_deref(),
        the_presented_sentence.as_deref(),
        at,
        SystemTime::now(),
    );
    let opening = match handover {
        Some(mut handover) => {
            if let Ok(started) = started.lock() {
                if let Some(record) = started.record.as_ref() {
                    handover.the_subjects_label =
                        the_subjects_own_label(record.connection(), &handover.subject);
                }
            }
            TheOpening::AFailure(handover)
        }
        None => TheOpening::TheMenu,
    };
    let _ = the_box_opens(
        &THE_BOX,
        &queue,
        opening,
        crate::platform::downloads_folder::the_folder_the_system_proposes(),
    );
}

/// The picker's ask for the report's folder: one folder, opening where the
/// box's folder is, titled with the row it answers.
fn the_folder_ask(starting_at: Option<PathBuf>) -> Asking {
    Asking {
        what: WhatMayBeChosen::Folders,
        how_many: HowMany::One,
        starting_at,
        title: Prose::new(words::SAVE_IN),
    }
}

/// **The folder the user chose**, or nothing where they chose none.
async fn a_folder_is_asked_for<R: Runtime>(app: &AppHandle<R>, starting_at: Option<PathBuf>) -> Option<PathBuf> {
    match picker::asked(app, the_folder_ask(starting_at)).await {
        Picked::TheUserChose(chosen) => chosen
            .first()
            .map(|one| one.as_the_user_gave_it().to_path_buf()),
        Picked::TheUserChoseNothing | Picked::ThereIsNoPanelForThisAsk(_) => None,
    }
}

/// ***Choose…*** — `Reporting a problem` V2: the folder through K11's picker,
/// and the user picks only the folder.
///
/// **`async`, because the system's folder panel is put up here** and a plain
/// command runs on the drawing thread — see [`crate::picker::asked`].
#[tauri::command]
pub async fn a_folder_is_chosen_for_the_report<R: Runtime>(app: AppHandle<R>) {
    let Some((generation, folder, _)) = the_save_begins(&THE_BOX) else {
        return;
    };
    let Some(chosen) = a_folder_is_asked_for(&app, folder.map(|one| one.path().to_path_buf())).await else {
        return;
    };
    if let Some(queue) = app.try_state::<Arc<TheOneQueue>>() {
        let _ = a_folder_was_chosen(&THE_BOX, &queue, generation, chosen);
    }
}

/// **The longest sentence the box takes from the page** — H34. The report's
/// own ceiling (`Reporting a problem` S3): nothing longer could leave the file
/// under it.
const THE_LONGEST_USERS_SENTENCE: usize = CEILING_BYTES as usize;

/// **The widest zone this computer can be in**, in seconds either side of UTC.
const THE_WIDEST_ZONE: i32 = 14 * 3600;

/// ***Save report*** — the press `Reporting a problem` V4 names as the
/// explicit direction, and everything after it in `Reporting a problem` V5a's
/// order. **With no folder held, the picker comes first** (W5), and a user who
/// chooses none is still asking.
///
/// `utc_offset_seconds` is the page's own zone at the press, used for the
/// file's name alone (`Reporting a problem` V3).
#[tauri::command]
pub async fn the_report_is_saved<R: Runtime>(
    app: AppHandle<R>,
    the_users_sentence: String,
    utc_offset_seconds: i32,
) {
    let Some(queue) = app.try_state::<Arc<TheOneQueue>>().map(|queue| queue.inner().clone()) else {
        return;
    };
    let Some((generation, folder, opening)) = the_save_begins(&THE_BOX) else {
        return;
    };
    let utc_offset_seconds = if utc_offset_seconds.abs() <= THE_WIDEST_ZONE {
        utc_offset_seconds
    } else {
        0
    };
    if the_users_sentence.len() > THE_LONGEST_USERS_SENTENCE {
        let _ = the_save_ended(
            &THE_BOX,
            &queue,
            generation,
            &WhatTheSaveCameTo::NotSaved {
                reported: "the sentence is longer than the report's ceiling".to_string(),
            },
        );
        return;
    }
    let folder = match folder {
        Some(folder) => folder,
        None => match a_folder_is_asked_for(&app, None).await {
            Some(chosen) => TheFolder::ChosenByTheUser(chosen),
            None => return,
        },
    };
    let request = TheSaveRequest {
        the_users_sentence,
        utc_offset_seconds,
        opening,
        folder: folder.path().to_path_buf(),
        made_at: SystemTime::now(),
    };
    if !the_box_is_underway(&THE_BOX, &queue, generation, folder) {
        return;
    }
    // **Off the drawing thread**: the take waits on the server's own point,
    // the write forces bytes to the device, and the reveal asks the system.
    let worker = app.clone();
    let came_to = tauri::async_runtime::spawn_blocking(move || {
        let Some(started) = worker.try_state::<Mutex<Started>>() else {
            return WhatTheSaveCameTo::NotSaved {
                reported: "the start report is not there".to_string(),
            };
        };
        the_save_is_carried_out(
            &started,
            |job| crate::the_report_take::the_take_at_the_press(&worker, job),
            &TheSystemsOwnRoute,
            || the_press_still_stands(&THE_BOX, generation),
            &request,
        )
    })
    .await
    .unwrap_or_else(|_| WhatTheSaveCameTo::NotSaved {
        reported: "the worker did not come back".to_string(),
    });
    let _ = the_save_ended(&THE_BOX, &queue, generation, &came_to);
}

/// ***Cancel*** and ***Close*** — the box is shut, and a press under way
/// writes nothing after this.
#[tauri::command]
pub fn the_report_box_is_shut(
    after_crash: Option<AfterCrash>,
    queue: State<'_, Arc<TheOneQueue>>,
) {
    if let Some(choice) = after_crash {
        if let Some(opening) = the_crash_choice(&THE_CRASH_OFFER, choice) {
            let _ = the_box_opens(&THE_BOX, &queue, opening,
                crate::platform::downloads_folder::the_folder_the_system_proposes());
        }
    } else { the_box_is_shut_in(&THE_BOX); }
}


/// `Reporting a problem` C2 and `Reporting a problem` C4: run-held only; no failure or suppression row.
#[derive(Default)]
pub struct TheCrashOffer { offered: bool, opening: Option<TheOpening> }
pub static THE_CRASH_OFFER: Mutex<TheCrashOffer> = Mutex::new(TheCrashOffer { offered: false, opening: None });
pub const THE_CRASH_BOX: &str = "reporting:the-crash-box";

/// Decide once, after the mark was read and overwritten by `start`.
pub fn the_launch_may_offer_a_crash(
    held: &Mutex<TheCrashOffer>, queue: &TheOneQueue, started: &Started,
    holds_the_instance: bool, mode: crate::start::LaunchMode,
) {
    let mut held = held.lock().unwrap_or_else(|e| e.into_inner());
    if held.offered { return; }
    held.offered = true;
    let Some(run) = crate::start::the_crash_to_offer(holds_the_instance, mode, &started.the_previous_run) else { return; };
    // Last/failure lines only supply a naming line. They never decide whether
    // a run crashed; the mark is the only reader of that state (`Reporting a problem` U4).
    let lines = started.diary.as_ref().and_then(|d| d.read_back().ok()).map(|d| d.lines).unwrap_or_default();
    let naming = [smoozy_reporting::lines::LineKind::Last, smoozy_reporting::lines::LineKind::Failure]
        .into_iter().find_map(|kind| lines.iter().rev().find(|l| l.run == run && l.kind == kind))
        .map(|line| TheNamingLine { the_presented_sentence: line.text.clone(), the_subjects_label: String::new(), at: line.at, clock: line.clock.clone() });
    held.opening = Some(TheOpening::ACrash { run, naming });
    let prose = |s: &str| Said::Prose(Prose::new(s));
    let _ = queue.read_then_post(|| Ok::<(), ()>(()), |_| Event::about(Subject::TheRun, THE_CRASH_BOX)
        .saying("heading", prose(words::CRASH_HEADING))
        .saying("nothingWasLost", prose(words::NOTHING_WAS_LOST))
        .saying("ifYouReportIt", prose(words::IF_YOU_REPORT_IT))
        .saying("notNow", prose(words::NOT_NOW))
        .saying("reportIt", prose(words::REPORT_IT)));
}

/// The existing close command carries this closed, optional choice; no new
/// command registration and no run identity accepted from the page.
#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AfterCrash { NotNow, Report }

/// *Not now* only consumes memory. It has no record or diary reference.
pub fn the_crash_choice(held: &Mutex<TheCrashOffer>, choice: AfterCrash) -> Option<TheOpening> {
    let opening = held.lock().unwrap_or_else(|e| e.into_inner()).opening.take();
    match choice { AfterCrash::Report => opening, AfterCrash::NotNow => None }
}

#[cfg(test)]
mod tests {
    use smoozy_reporting::report_file::Taken;
    use super::*;
    use crate::diary_service::DiaryService;
    use crate::events::{Delivers, Delivery};
    use smoozy_reporting::lines::LineKind;
    use std::fs;

    // ── What the tests stand on ─────────────────────────────────────────────

    /// The deliveries one queue made, as the page receives them.
    struct Kept(Arc<Mutex<Vec<Delivery>>>);
    impl Delivers for Kept {
        fn deliver(&self, delivery: &Delivery) {
            self.0.lock().unwrap().push(delivery.clone());
        }
    }

    fn a_queue() -> (TheOneQueue, Arc<Mutex<Vec<Delivery>>>) {
        let kept = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Kept(kept.clone())));
        (queue, kept)
    }

    fn the_last(kept: &Arc<Mutex<Vec<Delivery>>>) -> Delivery {
        kept.lock().unwrap().last().cloned().expect("something was posted")
    }

    fn said<'a>(delivery: &'a Delivery, name: &str) -> Option<&'a str> {
        delivery.said.get(name).map(|said| match said {
            Said::Prose(prose) => prose.as_str(),
            Said::Value(value) => value.as_str(),
        })
    }

    fn prose(text: &str) -> String {
        Prose::new(text).as_str().to_string()
    }

    fn a_scratch_name(name: &str) -> String {
        format!("smoozy-reporting-{name}-{}", std::process::id())
    }

    fn an_identity(kind: IdentityKind) -> Identity {
        Identity::issue(kind).expect("the desk has randomness")
    }

    /// **A start report over a throwaway record and a throwaway diary**, with
    /// every declaration registered as the running application registers them.
    fn a_started(name: &str) -> (Mutex<Started>, PathBuf, Identity) {
        let record_path = crate::scratch_folders::a_file(format!("{}.sqlite3", a_scratch_name(name)));
        let area = crate::scratch_folders::a_folder(a_scratch_name(&format!("{name}-diary")));
        let run = an_identity(IdentityKind::Run);
        let started = Started {
            the_sweep_left_behind: Vec::new(),
            record: Some(crate::record::LocalRecord::open_at(&record_path).expect("a record")),
            record_trouble: None,
            diary: Some(DiaryService::open_at(area.clone()).expect("a diary")),
            declarations: crate::start::the_declarations(),
            what_modules_declare: crate::presentation::declarations::TheDeclarations::default(),
            said_once: crate::presentation::said_once::TheThingsSaidOnce::new(),
            run: Some(run.clone()),
            the_previous_run: smoozy_state::run_mark::WhatTheLaunchFound::NoPreviousRun,

        };
        (Mutex::new(started), area, run)
    }

    fn a_line(run: &Identity, kind: LineKind, text: &str, concerning: Vec<Identity>) -> smoozy_reporting::lines::Line {
        smoozy_reporting::lines::Line {
            run: run.clone(),
            kind,
            at: UNIX_EPOCH + Duration::from_secs(1_756_132_000),
            clock: Clock::ThisComputer,
            text: text.to_string(),
            concerning,
        }
    }

    fn the_diary_holds(started: &Mutex<Started>) -> Vec<smoozy_reporting::lines::Line> {
        started.lock().unwrap().diary.as_ref().unwrap().read_back().unwrap().lines
    }

    /// The reveal, stood in for: it says what it was told to say and keeps what
    /// it was asked to show.
    struct AReveal {
        opens: bool,
        asked: Mutex<Vec<PathBuf>>,
    }
    impl TheRoute for AReveal {
        fn the_file_is_revealed(&self, file: &Path) -> Revealed {
            self.asked.lock().unwrap().push(file.to_path_buf());
            if self.opens {
                Revealed::ItOpened
            } else {
                Revealed::ItCouldNotBeOpened
            }
        }
    }

    fn a_reveal(opens: bool) -> AReveal {
        AReveal {
            opens,
            asked: Mutex::new(Vec::new()),
        }
    }

    const MADE_AT: u64 = 1_756_132_320; // 2025-08-25 14:32:00 UTC

    fn a_request(opening: TheOpening, folder: &Path) -> TheSaveRequest {
        TheSaveRequest {
            the_users_sentence: "The frames aren’t on the drive but Smoozy says they arrived.".to_string(),
            utc_offset_seconds: 0,
            opening,
            folder: folder.to_path_buf(),
            made_at: UNIX_EPOCH + Duration::from_secs(MADE_AT),
        }
    }

    fn a_history_handover(job: &Identity) -> TheHandover {
        TheHandover {
            kind: Kind::Ending,
            identity_or_mark: "the render did not finish".to_string(),
            subject: DurableSubject::HistoryRecord(job.clone()),
            the_presented_sentence: "The render did not finish. Smoozy does not know why.".to_string(),
            at: UNIX_EPOCH + Duration::from_secs(1_754_694_840),
            the_subjects_label: Some("Kitchen — final".to_string()),
        }
    }

    fn the_files_in(folder: &Path) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(folder)
            .unwrap()
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    // ── The two ways in ─────────────────────────────────────────────────────

    /// **`Reporting a problem` X2: the menu opens the box asking, with the
    /// downloads proposal, and names no failure.** Every word on it is the
    /// drawing's.
    #[test]
    fn the_menu_opens_the_box_asking_in_the_proposed_folder_and_names_nothing() {
        let the_box = Mutex::new(TheBoxHeld::new());
        let (queue, kept) = a_queue();
        let downloads = PathBuf::from("/Users/artist/Downloads");
        the_box_opens(
            &the_box,
            &queue,
            TheOpening::TheMenu,
            TheProposal::TheSystemNamesThisFolder(downloads.clone()),
        )
        .expect("posted");
        let asking = the_last(&kept);
        assert_eq!(asking.kind, THE_REPORT_BOX);
        assert_eq!(said(&asking, "reading"), Some("asking"));
        assert_eq!(said(&asking, "folder"), Some("/Users/artist/Downloads"));
        assert_eq!(said(&asking, "noFolderChosen"), None);
        assert_eq!(said(&asking, "aboutSentence"), None, "the menu hands over no failure");
        for (key, words) in [
            ("heading", words::HEADING),
            ("whatWentWrong", words::WHAT_WENT_WRONG),
            ("whatTheFileHolds0", words::THE_FILE_HOLDS),
            ("whatTheFileHolds1", words::NEVER_YOUR_KEYS),
            ("whatTheFileHolds2", words::SMOOZY_SENDS_NOTHING),
            ("saveIn", words::SAVE_IN),
            ("choose", words::CHOOSE),
            ("cancel", words::CANCEL),
            ("save", words::SAVE_THE_REPORT),
        ] {
            assert_eq!(said(&asking, key), Some(prose(words).as_str()), "{key}");
        }
        let (_, folder, opening) = the_save_begins(&the_box).expect("the box is asking");
        assert_eq!(folder, Some(TheFolder::TheDownloadsFolder(downloads)));
        assert_eq!(opening, TheOpening::TheMenu);
    }

    /// **W5 and `Reporting a problem` V4: where the route returns nothing,
    /// nothing is proposed** — the row says *No folder chosen*, no path is
    /// invented, and a press must choose a folder first.
    #[test]
    fn where_the_route_names_no_folder_nothing_is_proposed() {
        let the_box = Mutex::new(TheBoxHeld::new());
        let (queue, kept) = a_queue();
        the_box_opens(
            &the_box,
            &queue,
            TheOpening::TheMenu,
            TheProposal::TheSystemNamesNoFolder("HOME is unset".to_string()),
        )
        .unwrap();
        let asking = the_last(&kept);
        assert_eq!(said(&asking, "folder"), None);
        assert_eq!(said(&asking, "noFolderChosen"), Some(prose(words::NO_FOLDER_CHOSEN).as_str()));
        assert!(
            !asking.said.values().any(|one| matches!(one, Said::Value(value) if value.as_str().contains("HOME"))),
            "the system's own words reached the box"
        );
        let (generation, folder, _) = the_save_begins(&the_box).unwrap();
        assert_eq!(folder, None, "a press with nothing proposed has to choose first");

        // ***Choose…*** fills the row, and the folder is the user's.
        a_folder_was_chosen(&the_box, &queue, generation, PathBuf::from("/Volumes/Work/reports")).unwrap();
        let chosen = the_last(&kept);
        assert_eq!(said(&chosen, "folder"), Some("/Volumes/Work/reports"));
        assert_eq!(said(&chosen, "noFolderChosen"), None);
        let (_, folder, _) = the_save_begins(&the_box).unwrap();
        assert_eq!(folder, Some(TheFolder::ChosenByTheUser(PathBuf::from("/Volumes/Work/reports"))));
    }

    /// **H34 and `Reporting a problem` X4: a failure's handover is judged
    /// before anything is touched.** Each of the five has to read back as
    /// itself; a condition's mark has to be a condition identity; a missing
    /// moment is the press's.
    #[test]
    fn a_failures_handover_is_read_back_and_judged() {
        let job = an_identity(IdentityKind::Job).as_text();
        let server = an_identity(IdentityKind::LocalServer).as_text();
        let condition = an_identity(IdentityKind::Condition).as_text();
        let pressed = UNIX_EPOCH + Duration::from_secs(1_800_000_000);
        let read = |kind: &str, mark: Option<&str>, subject: Option<&str>, subject_kind: Option<&str>, sentence: Option<&str>, at: Option<u64>| {
            TheHandover::read_back(kind, mark, subject, subject_kind, sentence, at, pressed)
        };

        let ending = read("ending", Some("the render did not finish"), Some(&job), Some("history record"), Some("The render did not finish."), Some(1_754_694_840))
            .expect("a history entry's handover reads back");
        assert_eq!(ending.kind, Kind::Ending);
        assert_eq!(ending.subject, DurableSubject::HistoryRecord(Identity::read_back(IdentityKind::Job, &job).unwrap()));
        assert_eq!(ending.at, UNIX_EPOCH + Duration::from_secs(1_754_694_840));

        let live = read("condition", Some(&condition), Some(&server), Some("server row"), Some("Smoozy could not reach the server."), None)
            .expect("a condition's handover reads back");
        assert_eq!(live.at, pressed, "no moment on the presentation: the press stands in");
        assert_eq!(live.the_identities_it_carries().len(), 2, "the condition and the server");

        // **The licence record has no identity**, and is still a subject.
        assert!(read("condition", Some(&condition), None, Some("licence record"), Some("Smoozy could not read the licence record."), None).is_some());

        for refused in [
            read("a feeling", Some("x"), Some(&job), Some("history record"), Some("s"), None),
            read("ending", Some("x"), Some(&job), Some("the whole machine"), Some("s"), None),
            read("ending", Some("x"), Some("not an identity"), Some("history record"), Some("s"), None),
            read("ending", Some("x"), Some(&job), None, Some("s"), None),
            read("ending", Some("x"), Some(&job), Some("history record"), None, None),
            read("ending", Some("x"), Some(&job), Some("history record"), Some("  "), None),
            read("ending", Some("x"), Some(&job), Some("history record"), Some(&"x".repeat(5000)), None),
            read("condition", Some("not a condition identity"), Some(&server), Some("server row"), Some("s"), None),
        ] {
            assert!(refused.is_none(), "{refused:?}");
        }
    }

    /// **The history page's handover names its subject in §8.7's written
    /// form**, which reads back as a history record — the failed-job slice.
    #[test]
    fn the_history_entrys_subject_reads_back_as_a_history_record() {
        let job = an_identity(IdentityKind::Job);
        let subject = crate::surface::the_failure_shape::a_subject(
            crate::surface::the_history::THE_SUBJECT_AN_ENDING_IS_RECORDED_AGAINST,
            &job.as_text(),
        )
        .expect("the words are §8.7's");
        assert_eq!(subject, DurableSubject::HistoryRecord(job.clone()));
        assert_eq!(subject.kind_as_text(), crate::surface::the_history::THE_SUBJECT_AN_ENDING_IS_RECORDED_AGAINST);
    }

    /// **`Reporting a problem` X5: the box names the failure it came from** —
    /// the presented sentence through the point, the subject's own label and
    /// the moment, each on its own route.
    #[test]
    fn a_failure_opens_the_box_naming_it() {
        let the_box = Mutex::new(TheBoxHeld::new());
        let (queue, kept) = a_queue();
        let job = an_identity(IdentityKind::Job);
        the_box_opens(
            &the_box,
            &queue,
            TheOpening::AFailure(a_history_handover(&job)),
            TheProposal::TheSystemNamesThisFolder(PathBuf::from("/Users/artist/Downloads")),
        )
        .unwrap();
        let asking = the_last(&kept);
        assert_eq!(
            said(&asking, "aboutSentence"),
            Some(prose("The render did not finish. Smoozy does not know why.").as_str())
        );
        assert!(matches!(asking.said.get("aboutSentence"), Some(Said::Prose(_))));
        assert_eq!(said(&asking, "aboutLabel"), Some("Kitchen — final"));
        assert!(matches!(asking.said.get("aboutLabel"), Some(Said::Value(_))));
        assert_eq!(said(&asking, "aboutAt"), Some("1754694840"));
    }

    /// **`Reporting a problem` X4's mapping, applied**: a job record or a
    /// history record is the failed-job slice keyed by that job; any other
    /// subject, and the menu, is this run and the one before.
    #[test]
    fn a_job_or_history_record_is_the_failed_job_slice_and_every_other_subject_is_the_third() {
        let run = an_identity(IdentityKind::Run);
        let job = an_identity(IdentityKind::Job);
        for subject in [DurableSubject::JobRecord(job.clone()), DurableSubject::HistoryRecord(job.clone())] {
            let mut handover = a_history_handover(&job);
            handover.subject = subject;
            let opening = TheOpening::AFailure(handover);
            assert_eq!(the_slice_for(&opening, &run), Slice::AFailedJob { job: job.clone() });
            assert_eq!(the_job_it_names(&opening), Some(job.clone()));
        }
        for subject in [
            DurableSubject::ServerRow(an_identity(IdentityKind::LocalServer)),
            DurableSubject::KeyRow(an_identity(IdentityKind::LocalKey)),
            DurableSubject::TransferArrangement(an_identity(IdentityKind::Transfer)),
            DurableSubject::OutcomeRecord(an_identity(IdentityKind::Operation)),
            DurableSubject::LicenceRecord,
        ] {
            let mut handover = a_history_handover(&job);
            handover.subject = subject;
            let opening = TheOpening::AFailure(handover);
            assert_eq!(the_slice_for(&opening, &run), Slice::NothingInParticular { this_run: run.clone() });
            assert_eq!(the_job_it_names(&opening), None, "no machine is touched");
        }
        assert_eq!(the_slice_for(&TheOpening::TheMenu, &run), Slice::NothingInParticular { this_run: run.clone() });
        assert_eq!(opened_from(&TheOpening::TheMenu), OpenedFrom::TheMenu);
    }

    /// **The subject's own label is the record's**: a job is called what its
    /// entry calls it, a server what every screen calls it, and a subject with
    /// no label of its own has none rather than a made-up one.
    #[test]
    fn the_subjects_label_is_read_from_the_local_record() {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        let (started, _, _) = a_started("labels");
        let started = started.lock().unwrap();
        let connection = started.record.as_ref().unwrap().connection();

        let mut row = smoozy_servers::store::ServerRow::saved(
            an_identity(IdentityKind::LocalServer).as_text(),
            Host::what_the_user_typed("5.180.183.77").unwrap(),
            Port::what_the_user_typed(22).unwrap(),
            Username::what_the_user_typed("root").unwrap(),
            KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
        );
        row.label = Some("RunPod 4090".into());
        crate::record::server_rows::add(connection, &row).unwrap();
        crate::record::server_rows::write(connection, &row).unwrap();
        let server = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier).unwrap();
        assert_eq!(
            the_subjects_own_label(connection, &DurableSubject::ServerRow(server.clone())).as_deref(),
            Some("RunPod 4090")
        );

        let record = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — final");
        crate::record::job_records::JobRecords::over(connection)
            .write_the_provisional_record(&record)
            .unwrap();
        for subject in [DurableSubject::JobRecord(record.job.clone()), DurableSubject::HistoryRecord(record.job.clone())] {
            assert_eq!(the_subjects_own_label(connection, &subject).as_deref(), Some("Kitchen — final"));
        }

        assert_eq!(the_subjects_own_label(connection, &DurableSubject::LicenceRecord), None);
        assert_eq!(
            the_subjects_own_label(connection, &DurableSubject::JobRecord(an_identity(IdentityKind::Job))),
            None,
            "a job the record does not hold has no label"
        );
    }

    // ── The write ───────────────────────────────────────────────────────────

    /// **`Reporting a problem` V6 and `Reporting a problem` V3: the file lands
    /// whole, under the drawn name, and nothing is left staged.**
    #[test]
    fn the_report_lands_whole_under_the_drawn_name() {
        let folder = crate::scratch_folders::a_folder(a_scratch_name("write"));
        let at = UNIX_EPOCH + Duration::from_secs(MADE_AT);
        match the_file_is_written("the whole report\n", &folder, at, 0) {
            WhatTheWriteCameTo::Written { the_file_name, path } => {
                assert_eq!(the_file_name, "smoozy-report-25-aug-14-32.txt");
                assert_eq!(path, folder.join(&the_file_name));
                assert_eq!(fs::read_to_string(&path).unwrap(), "the whole report\n");
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(the_files_in(&folder), vec!["smoozy-report-25-aug-14-32.txt"], "something was left staged");
    }

    /// **`Reporting a problem` V5: a taken name is never overwritten and never
    /// prompted over.** A second report in the same minute lands beside the
    /// first under its own name; a file of the user's at the next name is left
    /// exactly as it was.
    #[test]
    fn a_taken_name_is_left_alone_and_the_report_lands_beside_it() {
        let folder = crate::scratch_folders::a_folder(a_scratch_name("taken"));
        let at = UNIX_EPOCH + Duration::from_secs(MADE_AT);
        fs::write(folder.join("smoozy-report-25-aug-14-32-2.txt"), "the user's own").unwrap();
        let first = the_file_is_written("first\n", &folder, at, 0);
        let second = the_file_is_written("second\n", &folder, at, 0);
        assert!(matches!(&first, WhatTheWriteCameTo::Written { the_file_name, .. } if the_file_name == "smoozy-report-25-aug-14-32.txt"));
        assert!(matches!(&second, WhatTheWriteCameTo::Written { the_file_name, .. } if the_file_name == "smoozy-report-25-aug-14-32-3.txt"));
        assert_eq!(fs::read_to_string(folder.join("smoozy-report-25-aug-14-32.txt")).unwrap(), "first\n");
        assert_eq!(fs::read_to_string(folder.join("smoozy-report-25-aug-14-32-2.txt")).unwrap(), "the user's own");
        assert_eq!(fs::read_to_string(folder.join("smoozy-report-25-aug-14-32-3.txt")).unwrap(), "second\n");
        assert_eq!(the_files_in(&folder).len(), 3, "something was left staged");
    }

    /// **The name's moment is this computer's own time** — the zone the page
    /// sent at the press (`Reporting a problem` V3).
    #[test]
    fn the_name_is_in_the_users_own_time() {
        let folder = crate::scratch_folders::a_folder(a_scratch_name("zone"));
        let at = UNIX_EPOCH + Duration::from_secs(MADE_AT);
        let written = the_file_is_written("x", &folder, at, 2 * 3600);
        assert!(matches!(&written, WhatTheWriteCameTo::Written { the_file_name, .. } if the_file_name == "smoozy-report-25-aug-16-32.txt"), "{written:?}");
    }

    /// **A folder that cannot take the file says so, and nothing claims a file
    /// exists** — F1: the outcome is the re-check's, never the calls'.
    #[test]
    fn a_folder_that_will_not_take_the_file_is_not_written_and_says_so() {
        let folder = crate::scratch_folders::a_folder(a_scratch_name("gone")).join("not there");
        let at = UNIX_EPOCH + Duration::from_secs(MADE_AT);
        assert!(matches!(the_file_is_written("x", &folder, at, 0), WhatTheWriteCameTo::NotWritten { .. }));
        assert!(!folder.exists(), "the write made a folder the user never chose");
    }

    // ── The press, whole ────────────────────────────────────────────────────

    /// **The menu's report, from the read to the reveal**: no machine is asked,
    /// the file holds this run's lines and the user's sentence verbatim, the
    /// *report written* line names the folder and the file and carries no
    /// failure, and the reveal is asked for the file itself.
    #[test]
    fn the_menus_report_is_written_its_act_line_kept_and_the_file_revealed() {
        let (started, _, run) = a_started("menu-press");
        let folder = crate::scratch_folders::a_folder(a_scratch_name("menu-press-folder"));
        {
            let held = started.lock().unwrap();
            let diary = held.diary.as_ref().unwrap();
            diary.write(&a_line(&run, LineKind::Session, "session\t0.1.8\tmacos", vec![run.clone()]));
            diary.write(&a_line(&run, LineKind::Act, "a server was connected", Vec::new()));
        }
        let reveal = a_reveal(true);
        let mut asked_the_machine = Vec::new();
        let came_to = the_save_is_carried_out(
            &started,
            |job| {
                asked_the_machine.push(job.cloned());
                None
            },
            &reveal,
            || true,
            &a_request(TheOpening::TheMenu, &folder),
        );
        let WhatTheSaveCameTo::Saved { the_file_name, the_folder, the_folder_opened } = came_to else {
            panic!("{came_to:?}");
        };
        assert!(the_folder_opened);
        assert_eq!(the_folder, folder);
        assert_eq!(asked_the_machine, vec![None], "the menu's report named a job");
        let text = fs::read_to_string(folder.join(&the_file_name)).unwrap();
        assert!(text.contains("The frames aren’t on the drive but Smoozy says they arrived."), "{text}");
        assert!(text.contains("a server was connected"), "{text}");
        assert_eq!(*reveal.asked.lock().unwrap(), vec![folder.join(&the_file_name)]);

        let lines = the_diary_holds(&started);
        let written = lines.last().expect("the act line");
        assert_eq!(written.kind, LineKind::Act);
        assert_eq!(written.text, format!("Report written: {the_file_name} in {}", folder.display()));
        assert!(written.concerning.is_empty(), "the menu hands over no identities");
        assert!(!written.text.contains("arrived"), "the user's sentence reached the diary");
    }

    /// **A failed job's report: the take is asked for that job, and what came
    /// back goes into the file as it came** — `Reporting a problem` G1 and
    /// `Reporting a problem` X4's failed-job slice, with the crash file whole.
    #[test]
    fn a_failed_jobs_report_takes_the_machine_and_names_the_failure() {
        let (started, _, run) = a_started("job-press");
        let folder = crate::scratch_folders::a_folder(a_scratch_name("job-press-folder"));
        let job = an_identity(IdentityKind::Job);
        let other = an_identity(IdentityKind::Job);
        {
            let held = started.lock().unwrap();
            let diary = held.diary.as_ref().unwrap();
            diary.write(&a_line(&run, LineKind::Session, "session\t0.1.8\tmacos", vec![run.clone()]));
            diary.write(&a_line(&run, LineKind::Act, "the job was submitted", vec![job.clone()]));
            let an_earlier_run = an_identity(IdentityKind::Run);
            diary.write(&a_line(&an_earlier_run, LineKind::Act, "another run's job", vec![other.clone()]));
        }
        let mut asked = Vec::new();
        let came_to = the_save_is_carried_out(
            &started,
            |named| {
                asked.push(named.cloned());
                Some(TheTake {
                    the_per_job_note: Taken::Whole("ending: failed\nreason: out of memory\n".to_string()),
                    the_journal_tail: Taken::Unreachable,
                    the_crash_file: Taken::Whole(smoozy_reporting::report_file::TheCrashFile {
                        name: "Kitchen.crash.txt".to_string(),
                        text: "the kept crash file, unchanged\r\n".to_string(),
                    }),
                })
            },
            &a_reveal(false),
            || true,
            &a_request(TheOpening::AFailure(a_history_handover(&job)), &folder),
        );
        let WhatTheSaveCameTo::Saved { the_file_name, the_folder_opened, .. } = came_to else {
            panic!("{came_to:?}");
        };
        assert!(!the_folder_opened, "the reveal said no and the box must not say it opened");
        assert_eq!(asked, vec![Some(job.clone())]);
        let text = fs::read_to_string(folder.join(&the_file_name)).unwrap();
        for words in [
            "The render did not finish. Smoozy does not know why.",
            "Kitchen — final",
            "the job was submitted",
            "reason: out of memory",
            "the kept crash file, unchanged\r\n",
            "Kitchen.crash.txt",
        ] {
            assert!(text.contains(words), "{words:?} is not in the file:\n{text}");
        }
        assert!(!text.contains("another run's job"), "the slice reached a run the job is not in");
        let written = the_diary_holds(&started).pop().unwrap();
        assert_eq!(written.concerning, vec![job.clone()], "the line carries the durable subject");
    }

    /// **`Reporting a problem` V5a: a box shut while the report was being
    /// prepared writes nothing** — the press is asked again before the write.
    #[test]
    fn a_box_shut_while_the_report_is_prepared_writes_nothing() {
        let (started, _, run) = a_started("shut-press");
        let folder = crate::scratch_folders::a_folder(a_scratch_name("shut-press-folder"));
        started.lock().unwrap().diary.as_ref().unwrap().write(&a_line(
            &run,
            LineKind::Session,
            "session",
            vec![run.clone()],
        ));
        let before = the_diary_holds(&started).len();
        let reveal = a_reveal(true);
        let came_to = the_save_is_carried_out(&started, |_| None, &reveal, || false, &a_request(TheOpening::TheMenu, &folder));
        assert!(matches!(came_to, WhatTheSaveCameTo::NotSaved { .. }));
        assert!(the_files_in(&folder).is_empty());
        assert_eq!(the_diary_holds(&started).len(), before, "an act line for a report never written");
        assert!(reveal.asked.lock().unwrap().is_empty());
    }

    /// **A diary that cannot be read is not a report that can be cut**, and
    /// the press says it did not save rather than writing a file that claims
    /// an empty record.
    #[test]
    fn a_save_with_no_diary_to_cut_is_not_saved() {
        let (started, _, _) = a_started("no-diary");
        started.lock().unwrap().diary = None;
        let folder = crate::scratch_folders::a_folder(a_scratch_name("no-diary-folder"));
        let came_to = the_save_is_carried_out(&started, |_| None, &a_reveal(true), || true, &a_request(TheOpening::TheMenu, &folder));
        assert!(matches!(came_to, WhatTheSaveCameTo::NotSaved { .. }));
        assert!(the_files_in(&folder).is_empty());
    }

    /// **The captured streams' outcomes are carried as the record kept them**
    /// (`Desktop app and delivery` M10, M11), and an outcome not written yet is
    /// none of the four and is left out.
    #[test]
    fn the_kept_outcomes_are_carried_as_recorded() {
        use crate::presentation::the_machines_own_words::Stream;
        use smoozy_runtime::capture::CollectionOutcome;
        let carried = as_the_report_carries_it(vec![
            (1, Stream::TheOrdinaryOutput, Some(CollectionOutcome::Collected)),
            (1, Stream::TheComplaints, Some(CollectionOutcome::CollectedAndTheStreamWasEmpty)),
            (2, Stream::TheOrdinaryOutput, Some(CollectionOutcome::CollectionIncomplete)),
            (2, Stream::TheComplaints, Some(CollectionOutcome::NeverEstablished)),
            (3, Stream::TheOrdinaryOutput, None),
        ]);
        assert_eq!(
            carried,
            vec![
                ARecordedCollection { attempt: 1, stream: TheStream::StandardOutput, outcome: RecordedCollection::Collected },
                ARecordedCollection { attempt: 1, stream: TheStream::StandardError, outcome: RecordedCollection::CollectedAndTheStreamWasEmpty },
                ARecordedCollection { attempt: 2, stream: TheStream::StandardOutput, outcome: RecordedCollection::CollectionIncomplete },
                ARecordedCollection { attempt: 2, stream: TheStream::StandardError, outcome: RecordedCollection::NeverEstablished },
            ]
        );
    }

    // ── The readings, in order ──────────────────────────────────────────────

    /// **The press moves the box through its readings in
    /// `Reporting a problem` V5a's order** — asking, then being prepared, then
    /// saved — and nothing says *saved* before the write's own outcome.
    #[test]
    fn the_press_moves_the_box_from_asking_to_prepared_to_saved() {
        let the_box = Mutex::new(TheBoxHeld::new());
        let (queue, kept) = a_queue();
        let downloads = PathBuf::from("/Users/artist/Downloads");
        the_box_opens(&the_box, &queue, TheOpening::TheMenu, TheProposal::TheSystemNamesThisFolder(downloads.clone())).unwrap();
        let (generation, folder, _) = the_save_begins(&the_box).unwrap();
        assert!(the_box_is_underway(&the_box, &queue, generation, folder.unwrap()));
        let composing = the_last(&kept);
        assert_eq!(said(&composing, "reading"), Some("composing"));
        assert_eq!(said(&composing, "preparing"), Some(prose(words::PREPARING_THE_REPORT).as_str()));
        assert_eq!(said(&composing, "savedHeading"), None, "a file is claimed before the re-check");
        assert!(the_save_begins(&the_box).is_none(), "a second press while the first runs");
        assert!(the_press_still_stands(&the_box, generation));

        the_save_ended(&the_box, &queue, generation, &WhatTheSaveCameTo::Saved {
            the_file_name: "smoozy-report-25-aug-14-32.txt".into(),
            the_folder: downloads.clone(),
            the_folder_opened: true,
        })
        .unwrap();
        let saved = the_last(&kept);
        assert_eq!(said(&saved, "reading"), Some("saved"));
        assert_eq!(said(&saved, "savedHeading"), Some(prose(words::THE_REPORT_IS_SAVED).as_str()));
        assert_eq!(said(&saved, "address"), Some(THE_ADDRESS));
        assert_eq!(said(&saved, "attachIt"), Some(prose(words::ATTACH_IT_TO_AN_EMAIL_TO).as_str()));
        assert_eq!(said(&saved, "close"), Some(prose(words::CLOSE).as_str()));
        assert!(the_save_begins(&the_box).is_none(), "a saved box takes no second press");
    }

    /// **`Reporting a problem` V8: the saved reading is true to the reveal.**
    /// The *just opened* sentence only where it opened; otherwise the file
    /// and its folder are named and the box says the folder did not open.
    #[test]
    fn the_saved_reading_says_the_folder_opened_only_where_it_did() {
        let downloads = PathBuf::from("/Users/artist/Downloads");
        let a_saved = |folder: &Path, opened: bool| {
            let the_box = Mutex::new(TheBoxHeld::new());
            let (queue, kept) = a_queue();
            the_box_opens(&the_box, &queue, TheOpening::TheMenu, TheProposal::TheSystemNamesThisFolder(downloads.clone())).unwrap();
            let (generation, held, _) = the_save_begins(&the_box).unwrap();
            the_box_is_underway(&the_box, &queue, generation, held.unwrap());
            the_save_ended(&the_box, &queue, generation, &WhatTheSaveCameTo::Saved {
                the_file_name: "smoozy-report-25-aug-14-32.txt".into(),
                the_folder: folder.to_path_buf(),
                the_folder_opened: opened,
            })
            .unwrap();
            let saved = the_last(&kept);
            let count: usize = said(&saved, "savedCount").unwrap().parse().unwrap();
            let parts: Vec<(String, bool)> = THE_SAVED_PARTS[..count]
                .iter()
                .map(|(words_key, value_key, clings_key)| {
                    let text = said(&saved, words_key).or(said(&saved, value_key)).unwrap().to_string();
                    (text, said(&saved, clings_key).is_some())
                })
                .collect();
            (parts, said(&saved, "folderOpened").is_some())
        };
        let name = || ("smoozy-report-25-aug-14-32.txt".to_string(), false);

        let (opened_in_downloads, opened) = a_saved(&downloads, true);
        assert!(opened);
        assert_eq!(opened_in_downloads, vec![name(), (prose(words::IS_IN_YOUR_DOWNLOADS_FOLDER_WHICH_SMOOZY_HAS_JUST_OPENED), false)]);

        let elsewhere = PathBuf::from("/Volumes/Work/reports");
        let (opened_elsewhere, _) = a_saved(&elsewhere, true);
        assert_eq!(
            opened_elsewhere,
            vec![name(), (prose(words::IS_IN), false), ("/Volumes/Work/reports".to_string(), false), (prose(words::WHICH_SMOOZY_HAS_JUST_OPENED), true)]
        );

        let (did_not_open, opened) = a_saved(&downloads, false);
        assert!(!opened);
        assert_eq!(
            did_not_open,
            vec![
                name(),
                (prose(words::IS_IN), false),
                ("/Users/artist/Downloads".to_string(), false),
                (prose(words::FULL_STOP), true),
                (prose(words::SMOOZY_COULD_NOT_OPEN_THE_FOLDER), false),
            ]
        );
        assert!(did_not_open.iter().all(|(text, _)| !text.contains("just opened")));
    }

    /// **A write that did not land is said, and the box asks again** — never a
    /// saved reading, and never silence (Rule 8).
    #[test]
    fn a_write_that_did_not_land_is_said_and_the_box_asks_again() {
        let the_box = Mutex::new(TheBoxHeld::new());
        let (queue, kept) = a_queue();
        the_box_opens(&the_box, &queue, TheOpening::TheMenu, TheProposal::TheSystemNamesThisFolder("/nowhere".into())).unwrap();
        let (generation, folder, _) = the_save_begins(&the_box).unwrap();
        the_box_is_underway(&the_box, &queue, generation, folder.unwrap());
        the_save_ended(&the_box, &queue, generation, &WhatTheSaveCameTo::NotSaved { reported: "full".into() }).unwrap();
        let again = the_last(&kept);
        assert_eq!(said(&again, "reading"), Some("not-saved"));
        assert_eq!(said(&again, "notSaved"), Some(prose(words::THE_REPORT_WAS_NOT_SAVED).as_str()));
        assert_eq!(said(&again, "savedHeading"), None);
        assert!(!again.said.values().any(|one| matches!(one, Said::Value(value) if value.as_str() == "full")), "the system's own words reached the box");
        assert!(the_save_begins(&the_box).is_some(), "the user cannot press again");
    }

    /// **A box shut, or shut and opened again, takes nothing more from a press
    /// that began before** — no reading is posted and no folder is set.
    #[test]
    fn a_press_whose_box_was_shut_posts_nothing() {
        let the_box = Mutex::new(TheBoxHeld::new());
        let (queue, kept) = a_queue();
        the_box_opens(&the_box, &queue, TheOpening::TheMenu, TheProposal::TheSystemNamesThisFolder("/a".into())).unwrap();
        let (generation, folder, _) = the_save_begins(&the_box).unwrap();
        the_box_is_underway(&the_box, &queue, generation, folder.unwrap());
        the_box_is_shut_in(&the_box);
        assert!(!the_press_still_stands(&the_box, generation));
        the_box_opens(&the_box, &queue, TheOpening::TheMenu, TheProposal::TheSystemNamesThisFolder("/a".into())).unwrap();
        let posted = kept.lock().unwrap().len();
        assert!(the_save_ended(&the_box, &queue, generation, &WhatTheSaveCameTo::NotSaved { reported: String::new() }).is_err());
        assert!(a_folder_was_chosen(&the_box, &queue, generation, "/b".into()).is_err());
        assert_eq!(kept.lock().unwrap().len(), posted);
        assert_eq!(said(&the_last(&kept), "reading"), Some("asking"));
    }

    // ── The words ───────────────────────────────────────────────────────────

    /// **Every word the box says is the drawing's**, read out of
    /// `Smoozy.html`'s `#rpbox` itself — save the one sentence the drawing
    /// never drew, which says so at its own site.
    #[test]
    fn every_word_on_the_box_is_the_drawings_own() {
        let mockup = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the drawing is readable");
        let the_box = mockup
            .split("<div class=\"modal top\" id=\"rpbox\" hidden>")
            .nth(1)
            .and_then(|rest| rest.split("<!-- ══ After a crash").next())
            .expect("the drawing's report box");
        // The drawing's text, its entities and its markup taken out, its
        // spaces made one kind.
        let mut text = String::new();
        let mut in_a_tag = false;
        let mut in_a_comment = false;
        let bytes: Vec<char> = the_box.chars().collect();
        let mut at = 0;
        while at < bytes.len() {
            let rest: String = bytes[at..bytes.len().min(at + 4)].iter().collect();
            if !in_a_tag && rest.starts_with("<!--") {
                in_a_comment = true;
            }
            if in_a_comment {
                if rest.starts_with("-->") {
                    in_a_comment = false;
                    at += 3;
                    continue;
                }
                at += 1;
                continue;
            }
            match bytes[at] {
                '<' => in_a_tag = true,
                '>' => {
                    in_a_tag = false;
                    text.push(' ');
                }
                one if !in_a_tag => text.push(one),
                _ => {}
            }
            at += 1;
        }
        let text = text
            .replace("&#8212;", "—")
            .replace("&#8217;", "’")
            .replace("&#8230;", "…")
            .replace("&#183;", "·")
            .replace("&nbsp;", " ");
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        let drawn = |words: &str| text.contains(&words.split_whitespace().collect::<Vec<_>>().join(" "));
        for words in [
            words::HEADING,
            words::WHAT_WENT_WRONG,
            words::THE_FILE_HOLDS,
            words::NEVER_YOUR_KEYS,
            words::SMOOZY_SENDS_NOTHING,
            words::SAVE_IN,
            words::NO_FOLDER_CHOSEN,
            words::CHOOSE,
            words::CANCEL,
            words::SAVE_THE_REPORT,
            words::THE_REPORT_IS_SAVED,
            words::IS_IN_YOUR_DOWNLOADS_FOLDER_WHICH_SMOOZY_HAS_JUST_OPENED,
            words::IS_IN,
            words::SMOOZY_COULD_NOT_OPEN_THE_FOLDER,
            words::ATTACH_IT_TO_AN_EMAIL_TO,
            words::CLOSE,
            THE_ADDRESS,
        ] {
            assert!(drawn(words), "{words:?} is not the drawing's: {text}");
        }
        // The clause after another folder is the drawing's own clause.
        assert!(drawn(words::WHICH_SMOOZY_HAS_JUST_OPENED.trim_start_matches(", ")));
        // The underway control's name is the drawing's, in its script.
        assert!(mockup.contains("save.setAttribute('aria-label', 'Preparing report')"));
        assert_eq!(words::PREPARING_THE_REPORT, "Preparing report");
        // And the one the drawing does not hold says so.
        assert!(!drawn(words::THE_REPORT_WAS_NOT_SAVED));
        assert!(include_str!("reporting.rs").contains("**The one sentence the drawing does not hold.**"));
    }

    /// **The three failure pages hand their handover to the box**, and the
    /// handover parked on the document with nothing to take it is gone.
    #[test]
    fn every_failure_page_hands_its_handover_to_the_box() {
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["the-failure-shape.js", "the-history.js", "the-machines-own-words.js"] {
            let text = fs::read_to_string(web.join(page)).unwrap();
            let working: String = text
                .lines()
                .filter(|line| !line.trim_start().starts_with("//"))
                .collect::<Vec<_>>()
                .join("\n");
            // **No screen takes the handover for itself**, so the shape's own
            // control opens the box.
            assert!(
                !working.contains("onReport:") && !working.contains("onReport("),
                "{page} takes the handover itself"
            );
            assert!(!working.contains("theFailureHandover"), "{page} still parks the handover");
            // **The opener is the failure shape's**, so no page that draws a
            // failure imports the box's module — every page harness that loads
            // the shape keeps loading only what it loaded before.
            assert!(!working.contains("from \"./reporting.js\""), "{page} imports the box's module");
        }
        let shape = fs::read_to_string(web.join("the-failure-shape.js")).unwrap();
        assert!(shape.contains("export function theReportBoxOpensFrom(handover) {"));
        assert!(shape.contains("const opens = parts.onReport || theReportBoxOpensFrom;"));
        assert!(shape.contains("shell.core.invoke(\"the_report_box_is_opened_from_a_failure\", {"));
        let history = fs::read_to_string(web.join("the-history.js")).unwrap();
        assert!(history.contains("subject: said.entryJob,") && history.contains("subjectKind: said.failSubjectKind,"));
    }

    #[test]
    fn every_crash_word_is_the_drawings_own() {
        let drawing=include_str!("../../../../Preparation/Design/Mockup/Smoozy.html")
            .replace("&nbsp;"," ").replace("&#8230;","…");
        let crash=drawing.split("id=\"rpcrash\"").nth(1).unwrap().split("<!-- ══════════════ NOTIFICATIONS").next().unwrap();
        for words in [words::CRASH_HEADING,words::NOTHING_WAS_LOST,words::IF_YOU_REPORT_IT,words::NOT_NOW,words::REPORT_IT] {
            assert!(crash.contains(words),"{words:?} is not in the crash drawing");
        }
    }

    #[test]
    fn the_crash_offer_is_once_and_not_now_changes_no_record_or_diary() {
        let (started, area, _) = a_started("crash-dismiss");
        let mut started = started.into_inner().unwrap();
        let run = an_identity(IdentityKind::Run);
        started.the_previous_run = smoozy_state::run_mark::WhatTheLaunchFound::ThePreviousRunDidNot { the_run: run.clone() };
        let held = Mutex::new(TheCrashOffer::default());
        let (queue, kept) = a_queue();
        let before = started.diary.as_ref().unwrap().read_back().unwrap();
        let record = started.record.as_ref().unwrap().connection();
        let changes = record.total_changes();
        the_launch_may_offer_a_crash(&held,&queue,&started,true,crate::start::LaunchMode::Ordinary);
        assert_eq!(kept.lock().unwrap().len(),1);
        let offer = the_last(&kept);
        assert_eq!(offer.kind,THE_CRASH_BOX);
        assert_eq!(said(&offer,"heading"),Some(prose(words::CRASH_HEADING).as_str()));
        assert_eq!(said(&offer,"nothingWasLost"),Some(prose(words::NOTHING_WAS_LOST).as_str()));
        assert_eq!(the_crash_choice(&held,AfterCrash::NotNow),None);
        assert_eq!(the_crash_choice(&held,AfterCrash::Report),None);
        the_launch_may_offer_a_crash(&held,&queue,&started,true,crate::start::LaunchMode::Ordinary);
        assert_eq!(kept.lock().unwrap().len(),1,"the same crash was offered twice");
        assert_eq!(before,started.diary.as_ref().unwrap().read_back().unwrap());
        assert_eq!(record.total_changes(),changes,"dismissal wrote to the record");
        drop(started); let _=fs::remove_dir_all(area);
    }

    #[test]
    fn a_crash_report_cuts_only_the_marked_run_and_names_its_last_line() {
        use smoozy_reporting::lines::{Line,LineKind};
        let (started,area,this_run)=a_started("crash-report");
        let mut started=started.into_inner().unwrap();
        let crashed=an_identity(IdentityKind::Run);
        let unrelated=an_identity(IdentityKind::Run);
        let diary=started.diary.as_ref().unwrap();
        let at=UNIX_EPOCH+Duration::from_secs(1_756_132_000);
        for (run,kind,text) in [(unrelated,LineKind::Session,"other session"),(crashed.clone(),LineKind::Session,"dead session"),(crashed.clone(),LineKind::Failure,"A previously shown failure."),(crashed.clone(),LineKind::Last,"SIGTERM"),(this_run.clone(),LineKind::Session,"new session")] {
            assert_eq!(diary.write(&Line{run,kind,text:text.into(),at,clock:Clock::ThisComputer,concerning:vec![]}),crate::diary_service::Wrote::Yes);
        }
        started.the_previous_run=smoozy_state::run_mark::WhatTheLaunchFound::ThePreviousRunDidNot{the_run:crashed.clone()};
        let held=Mutex::new(TheCrashOffer::default()); let (queue,_)=a_queue();
        the_launch_may_offer_a_crash(&held,&queue,&started,true,crate::start::LaunchMode::Ordinary);
        let opening=the_crash_choice(&held,AfterCrash::Report).unwrap();
        assert_eq!(the_slice_for(&opening,&this_run),Slice::ACrash{run:crashed.clone()});
        assert_eq!(the_job_it_names(&opening),None);
        assert!(matches!(opened_from(&opening),OpenedFrom::ACrash{the_naming_line:Some(n)} if n.the_presented_sentence=="SIGTERM" && n.at==at));
        let the_box=Mutex::new(TheBoxHeld::new()); let (queue,kept)=a_queue();
        the_box_opens(&the_box,&queue,opening.clone(),TheProposal::TheSystemNamesNoFolder(String::new())).unwrap();
        assert_eq!(said(&the_last(&kept),"aboutSentence"),Some("SIGTERM"));
        let destination=area.join("saved"); fs::create_dir_all(&destination).unwrap();
        let outcome=the_save_is_carried_out(&Mutex::new(started), |job| { assert!(job.is_none(), "a crash report named a machine's job"); None },&a_reveal(false),||true,&a_request(opening,&destination));
        let WhatTheSaveCameTo::Saved{the_file_name,..}=outcome else { panic!("{outcome:?}") };
        let report=fs::read_to_string(destination.join(the_file_name)).unwrap();
        assert!(report.contains("dead session") && report.contains("SIGTERM"));
        assert!(!report.contains("other session") && !report.contains("new session"));
    }

    #[test]
    fn a_crash_with_no_last_line_still_opens_and_takes_no_machine() {
        let (started,_,_)=a_started("silent-crash"); let mut started=started.into_inner().unwrap();
        let run=an_identity(IdentityKind::Run);
        started.the_previous_run=smoozy_state::run_mark::WhatTheLaunchFound::ThePreviousRunDidNot{the_run:run.clone()};
        for (holds,mode) in [(false,crate::start::LaunchMode::Ordinary),(true,crate::start::LaunchMode::Update)] {
            let (queue,kept)=a_queue(); let held=Mutex::new(TheCrashOffer::default());
            the_launch_may_offer_a_crash(&held,&queue,&started,holds,mode);
            assert!(kept.lock().unwrap().is_empty());
        }
        let (queue,kept)=a_queue();let held=Mutex::new(TheCrashOffer::default());
        the_launch_may_offer_a_crash(&held,&queue,&started,true,crate::start::LaunchMode::Ordinary);
        assert_eq!(kept.lock().unwrap().len(),1);
        assert_eq!(the_crash_choice(&held,AfterCrash::Report),Some(TheOpening::ACrash{run,naming:None}));
    }

    // ── The page, in a real browser ─────────────────────────────────────────

    /// **One delivery as the page receives it**, through the real queue.
    fn as_the_page_gets_it(delivery: &Delivery) -> serde_json::Value {
        serde_json::to_value(delivery).expect("a delivery serialises")
    }

    /// The box's deliveries for every reading the drawing draws, composed by
    /// the box's own functions.
    fn the_deliveries() -> serde_json::Value {
        let downloads = PathBuf::from("/Users/artist/Downloads");
        let job = an_identity(IdentityKind::Job);
        let mut out = serde_json::Map::new();
        let mut keep = |name: &str, kept: &Arc<Mutex<Vec<Delivery>>>| {
            out.insert(name.to_string(), as_the_page_gets_it(&the_last(kept)));
        };

        let (queue, kept) = a_queue();
        let the_box = Mutex::new(TheBoxHeld::new());
        the_box_opens(&the_box, &queue, TheOpening::TheMenu, TheProposal::TheSystemNamesThisFolder(downloads.clone())).unwrap();
        keep("menu", &kept);
        let (started,_,_)=a_started("crash-page");let mut started=started.into_inner().unwrap();
        started.the_previous_run=smoozy_state::run_mark::WhatTheLaunchFound::ThePreviousRunDidNot{the_run:an_identity(IdentityKind::Run)};
        let crash=Mutex::new(TheCrashOffer::default());
        the_launch_may_offer_a_crash(&crash,&queue,&started,true,crate::start::LaunchMode::Ordinary);
        keep("crash", &kept);
        let opening=the_crash_choice(&crash,AfterCrash::Report).unwrap();
        the_box_opens(&the_box,&queue,opening,TheProposal::TheSystemNamesThisFolder(downloads.clone())).unwrap();
        keep("crashReport",&kept);


        let (queue, kept) = a_queue();
        let the_box = Mutex::new(TheBoxHeld::new());
        the_box_opens(&the_box, &queue, TheOpening::AFailure(a_history_handover(&job)), TheProposal::TheSystemNamesThisFolder(downloads.clone())).unwrap();
        keep("rp", &kept);
        let (generation, folder, _) = the_save_begins(&the_box).unwrap();
        the_box_is_underway(&the_box, &queue, generation, folder.unwrap());
        keep("composing", &kept);
        the_save_ended(&the_box, &queue, generation, &WhatTheSaveCameTo::NotSaved { reported: String::new() }).unwrap();
        keep("notSaved", &kept);
        let (generation, folder, _) = the_save_begins(&the_box).unwrap();
        the_box_is_underway(&the_box, &queue, generation, folder.unwrap());
        the_save_ended(&the_box, &queue, generation, &WhatTheSaveCameTo::Saved {
            the_file_name: "smoozy-report-25-aug-14-32.txt".into(),
            the_folder: downloads.clone(),
            the_folder_opened: true,
        })
        .unwrap();
        keep("saved", &kept);

        let (queue, kept) = a_queue();
        let the_box = Mutex::new(TheBoxHeld::new());
        the_box_opens(&the_box, &queue, TheOpening::AFailure(a_history_handover(&job)), TheProposal::TheSystemNamesThisFolder(downloads.clone())).unwrap();
        let (generation, folder, _) = the_save_begins(&the_box).unwrap();
        the_box_is_underway(&the_box, &queue, generation, folder.unwrap());
        the_save_ended(&the_box, &queue, generation, &WhatTheSaveCameTo::Saved {
            the_file_name: "smoozy-report-25-aug-14-32.txt".into(),
            the_folder: downloads.clone(),
            the_folder_opened: false,
        })
        .unwrap();
        keep("savedNoOpen", &kept);

        let (queue, kept) = a_queue();
        let the_box = Mutex::new(TheBoxHeld::new());
        the_box_opens(&the_box, &queue, TheOpening::TheMenu, TheProposal::TheSystemNamesNoFolder(String::new())).unwrap();
        keep("noFolder", &kept);
        let (generation, _, _) = the_save_begins(&the_box).unwrap();
        a_folder_was_chosen(&the_box, &queue, generation, "/Volumes/Work/reports".into()).unwrap();
        keep("chosen", &kept);
        let (generation, folder, _) = the_save_begins(&the_box).unwrap();
        the_box_is_underway(&the_box, &queue, generation, folder.unwrap());
        the_save_ended(&the_box, &queue, generation, &WhatTheSaveCameTo::Saved {
            the_file_name: "smoozy-report-25-aug-14-32.txt".into(),
            the_folder: "/Volumes/Work/reports".into(),
            the_folder_opened: true,
        })
        .unwrap();
        keep("savedElsewhere", &kept);
        serde_json::Value::Object(out)
    }

    fn a_chrome() -> Option<PathBuf> {
        [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ]
        .into_iter()
        .map(PathBuf::from)
        .find(|one| one.exists())
    }

    /// **Every reading of the box, drawn in a real browser** — headless Chrome
    /// draws it from `web/reporting.js` and the application's own stylesheets,
    /// fed the box's own deliveries, and the page script presses what a user
    /// presses. The commands are a stand-in that keeps what was asked.
    ///
    /// The readings, each checked as the drawing draws it: from the menu, no
    /// line naming a failure (`rp`, the menu's form); from a failure, the line
    /// with the sentence, the label and the moment (`rp`); *No folder chosen*
    /// with both controls live (`rp-nofolder`); while the report is prepared,
    /// the field and *Choose…* locked, *Save report* underway with its
    /// spoken name, *Cancel* live and nothing saved said (`rp-composing`);
    /// saved, with the drawing's opened sentence and the address (`rp-saved`);
    /// saved where the folder did not open, the folder named and no *just
    /// opened* (`rp-saved-noopen`); and the two this piece adds — saved in
    /// another folder, and the write that did not land. **Every box fits the
    /// window at 1280 and 720 with nothing scrolling sideways.** A computer
    /// with no Chrome skips it.
    #[test]
    fn every_reading_of_the_box_is_drawn_in_a_real_browser() {
        let Some(chrome) = a_chrome() else {
            eprintln!("SKIPPED: no Chrome on this computer, so the report box was not drawn");
            return;
        };
        let deliveries = the_deliveries();
        let folder = crate::scratch_folders::a_folder(a_scratch_name("the-box-page"));
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for entry in fs::read_dir(&web).expect("the page folder is readable") {
            let path = entry.expect("an entry").path();
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if path.is_file() && (extension == "js" || extension == "css") {
                fs::copy(&path, folder.join(path.file_name().unwrap())).expect("the page is copied");
            }
        }
        fs::write(folder.join("deliveries.js"), format!("export default {deliveries};\n")).expect("the deliveries are written");
        fs::write(folder.join("index.html"), THE_BOX_PAGE).expect("the page is written");
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
                .arg(format!("--user-data-dir={}", folder.join(format!("profile-{width}")).display()))
                .arg(format!("file://{}", folder.join("index.html").display()))
                .stdout(fs::File::create(&dumped).expect("the dump is written"))
                .stderr(fs::File::create(&browser_errors).expect("the browser's errors are kept"))
                .spawn()
                .expect("Chrome starts");
            // A cancellable fixed deadline; no wait loop.
            let pid = running.id();
            #[cfg(windows)]
            let handle = { use std::os::windows::io::AsRawHandle; running.as_raw_handle() as usize };
            let (cancel, deadline) = std::sync::mpsc::channel();
            let guard = std::thread::spawn(move || {
                if deadline.recv_timeout(Duration::from_secs(60)).is_err() {
                    #[cfg(unix)] unsafe { libc::kill(pid as i32, libc::SIGKILL); }
                    #[cfg(windows)] unsafe {
                        #[link(name="kernel32")] unsafe extern "system" { fn TerminateProcess(handle: *mut std::ffi::c_void, code: u32) -> i32; }
                        TerminateProcess(handle as *mut std::ffi::c_void, 1);
                    }
                }
            });
            let ended = running.wait().expect("Chrome is watched");
            let _ = cancel.send(()); guard.join().expect("the deadline ends");
            assert!(ended.success(), "Chrome did not finish: {ended}: {}", fs::read_to_string(&browser_errors).unwrap_or_default());
            let page = fs::read_to_string(&dumped).unwrap_or_default();
            let out = page
                .split("<pre id=\"out\">")
                .nth(1)
                .and_then(|rest| rest.split("</pre>").next())
                .filter(|said| !said.is_empty())
                .unwrap_or_else(|| panic!("the page drew nothing at {width}: {page}: {}", fs::read_to_string(&browser_errors).unwrap_or_default()))
                .replace("&nbsp;", "\u{a0}")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&amp;", "&");
            answers.push((width, serde_json::from_str::<serde_json::Value>(&out).expect("one JSON answer")));
        }
        // **Where `SMOOZY_THE_REPORT_BOX_PAGE` names a folder, the page and
        // its deliveries are left there** — for the builder's own look beside
        // the drawing's pictures, which no assertion stands in for.
        if let Some(keep) = std::env::var_os("SMOOZY_THE_REPORT_BOX_PAGE") {
            let keep = PathBuf::from(keep);
            let _ = fs::create_dir_all(&keep);
            for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
                if entry.path().is_file() {
                    let _ = fs::copy(entry.path(), keep.join(entry.file_name()));
                }
            }
        }

        let flat = |text: &serde_json::Value| text.as_str().unwrap_or("").replace('\u{a0}', " ");
        let plain = |words: &str| words.replace('\u{a0}', " ");
        for (width, out) in &answers {
            assert!(out["error"].is_null(), "{width}: the page stopped: {} {}", out["error"], out["troubles"]);
            assert_eq!(out["troubles"], serde_json::json!([]), "{width}: the page reported trouble");
            let crash=&out["crash"];
            assert_eq!(crash["shown"],true,"{width}: {crash}");
            assert_eq!(crash["fits"],true,"{width}: {crash}");
            assert_eq!(crash["sideways"],false,"{width}: {crash}");
            assert_eq!(crash["focused"],"rpfromcrash");
            assert_eq!(flat(&crash["bold"]),plain(&prose(words::NOTHING_WAS_LOST)));
            for word in [words::CRASH_HEADING,words::IF_YOU_REPORT_IT,words::NOT_NOW,words::REPORT_IT] {
                assert!(flat(&crash["text"]).contains(&plain(&prose(word))),"{width}: {word}: {crash}");
            }
            for (name,choice) in [("crashDismiss","not-now"),("crashEscape","not-now"),("crashReport","report")] {
                assert_eq!(out[name]["hidden"],true);
                assert_eq!(out[name]["calls"],serde_json::json!([{"command":"the_report_box_is_shut","args":{"afterCrash":choice}}]));
            }
            assert_eq!(out["states"]["rp-crash"]["about"],false);
            let states = &out["states"];
            let at = |state: &str| -> &serde_json::Value {
                let one = &states[state];
                assert!(!one.is_null(), "{width}: {state} was not reached: {out}");
                one
            };
            let has = |state: &str, words: &str| {
                let text = flat(&at(state)["text"]);
                assert!(text.contains(&plain(words)), "{width} {state}: the box does not say {words:?}: {text:?}");
            };
            let lacks = |state: &str, words: &str| {
                let text = flat(&at(state)["text"]);
                assert!(!text.contains(&plain(words)), "{width} {state}: the box says {words:?}: {text:?}");
            };
            for state in states.as_object().unwrap().keys() {
                let one = at(state);
                assert_eq!(one["shown"], true, "{width} {state}: the box is not shown");
                assert_eq!(one["fits"], true, "{width} {state}: the box does not fit the window: {one}");
                assert_eq!(one["sideways"], false, "{width} {state}: the box scrolls sideways: {one}");
            }
            // From the menu: the drawing's asking reading with no failure named.
            let menu = at("rp-menu");
            assert_eq!(menu["about"], false, "{width}: the menu's box names a failure");
            assert_eq!(menu["folder"], "/Users/artist/Downloads");
            assert_eq!(menu["noFolder"], false);
            assert_eq!(menu["focused"], "rpsay", "{width}: the field does not have the focus");
            for words in [words::HEADING, words::WHAT_WENT_WRONG, words::THE_FILE_HOLDS, words::NEVER_YOUR_KEYS, words::SMOOZY_SENDS_NOTHING, words::SAVE_IN, words::CHOOSE, words::CANCEL, words::SAVE_THE_REPORT] {
                has("rp-menu", words);
            }
            lacks("rp-menu", words::THE_REPORT_IS_SAVED);
            assert_eq!(menu["saveLive"], true);
            assert_eq!(menu["chooseLive"], true);
            // From a failure: the line naming it, sentence bold.
            let failure = at("rp");
            assert_eq!(failure["about"], true);
            assert_eq!(flat(&failure["aboutBold"]), "The render did not finish. Smoozy does not know why.");
            let about = flat(&failure["aboutText"]);
            assert!(about.contains(" · Kitchen — final · "), "{width}: {about:?}");
            assert!(about.ends_with(&flat(&failure["aboutMoment"])) && !flat(&failure["aboutMoment"]).is_empty(), "{width}: {about:?}");
            // No folder proposed.
            let none = at("rp-nofolder");
            assert_eq!(none["noFolder"], true);
            assert_eq!(none["folder"], "");
            has("rp-nofolder", words::NO_FOLDER_CHOSEN);
            assert_eq!(none["saveLive"], true, "{width}: Save report must open the chooser");
            assert_eq!(none["chooseLive"], true);
            let chosen = at("rp-chosen");
            assert_eq!(chosen["folder"], "/Volumes/Work/reports");
            assert_eq!(chosen["noFolder"], false);
            // While the report is prepared.
            let composing = at("rp-composing");
            assert_eq!(composing["saveLive"], false);
            assert_eq!(composing["saveUnderway"], true);
            assert_eq!(flat(&composing["saveName"]), plain(words::PREPARING_THE_REPORT));
            assert_eq!(composing["fieldReadOnly"], true);
            assert_eq!(composing["chooseLive"], false);
            assert_eq!(composing["cancelLive"], true);
            assert_eq!(composing["typed"], "The frames aren’t on the drive but Smoozy says they arrived.", "{width}: the field lost the sentence");
            lacks("rp-composing", words::THE_REPORT_IS_SAVED);
            // Saved, the folder opened.
            let saved = at("rp-saved");
            has("rp-saved", words::THE_REPORT_IS_SAVED);
            assert_eq!(flat(&saved["saved"]), plain(&format!("smoozy-report-25-aug-14-32.txt {}", prose(words::IS_IN_YOUR_DOWNLOADS_FOLDER_WHICH_SMOOZY_HAS_JUST_OPENED))));
            assert_eq!(saved["savedMono"], serde_json::json!(["smoozy-report-25-aug-14-32.txt"]));
            has("rp-saved", words::ATTACH_IT_TO_AN_EMAIL_TO);
            assert_eq!(saved["address"], THE_ADDRESS);
            assert_eq!(saved["focused"], "close");
            assert_eq!(saved["askShown"], false, "{width}: the asking parts show on the saved reading");
            lacks("rp-saved", words::SAVE_THE_REPORT);
            // Saved, the folder did not open.
            let no_open = at("rp-saved-noopen");
            assert_eq!(
                flat(&no_open["saved"]),
                plain(&format!(
                    "smoozy-report-25-aug-14-32.txt {} /Users/artist/Downloads{} {}",
                    prose(words::IS_IN),
                    prose(words::FULL_STOP),
                    prose(words::SMOOZY_COULD_NOT_OPEN_THE_FOLDER)
                ))
            );
            assert_eq!(no_open["savedMono"], serde_json::json!(["smoozy-report-25-aug-14-32.txt", "/Users/artist/Downloads"]));
            lacks("rp-saved-noopen", "just opened");
            assert_eq!(no_open["address"], THE_ADDRESS);
            // Saved in another folder.
            let elsewhere = at("rp-saved-elsewhere");
            assert_eq!(
                flat(&elsewhere["saved"]),
                plain(&format!(
                    "smoozy-report-25-aug-14-32.txt {} /Volumes/Work/reports{}",
                    prose(words::IS_IN),
                    prose(words::WHICH_SMOOZY_HAS_JUST_OPENED)
                ))
            );
            // The write that did not land: the one failure shape, the field kept.
            let not_saved = at("rp-not-saved");
            assert_eq!(flat(&not_saved["failure"]), plain(&prose(words::THE_REPORT_WAS_NOT_SAVED)));
            assert_eq!(not_saved["failureDot"], true);
            assert_eq!(not_saved["typed"], "The frames aren’t on the drive but Smoozy says they arrived.");
            assert_eq!(not_saved["saveLive"], true);
            lacks("rp-not-saved", words::THE_REPORT_IS_SAVED);

            // **The presses.**
            let presses = &out["presses"];
            assert_eq!(
                presses["save"],
                serde_json::json!({"command": "the_report_is_saved", "args": {"theUsersSentence": "The frames aren’t on the drive but Smoozy says they arrived.", "utcOffsetSeconds": presses["save"]["args"]["utcOffsetSeconds"]}})
            );
            assert!(presses["save"]["args"]["utcOffsetSeconds"].is_i64(), "{width}: no zone was sent");
            assert_eq!(presses["choose"], "a_folder_is_chosen_for_the_report");
            assert_eq!(presses["cancel"], serde_json::json!({"command": "the_report_box_is_shut", "hidden": true}));
            assert_eq!(presses["escape"], serde_json::json!({"command": "the_report_box_is_shut", "hidden": true}));
            assert_eq!(presses["fresh"], "", "{width}: a box opened anew kept the last sentence");
            assert_eq!(presses["savedAgain"], 0, "{width}: a press reached the application while the report was prepared");
            // **A failure shape's *Report a problem…* opens the box with M33's
            // handover**, through the shared half's own drawing.
            assert_eq!(
                presses["fromAFailure"],
                serde_json::json!({
                    "command": "the_report_box_is_opened_from_a_failure",
                    "args": {
                        "kind": "condition",
                        "identityOrMark": "0123456789abcdef0123456789abcdef",
                        "durableSubject": "fedcba9876543210fedcba9876543210",
                        "subjectKind": "server row",
                        "thePresentedSentence": "Smoozy could not reach the server.",
                        "at": 1754694840
                    }
                })
            );
        }
    }

    /// The page the box is drawn in: the application's stylesheets, a stand-in
    /// for the shell that keeps every command asked, and a script that tells
    /// the box each reading and presses what a user presses.
    const THE_BOX_PAGE: &str = r##"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
</head><body><div class="app"><div class="pane" id="pane"></div></div><pre id="out"></pre>
<script>
  window.__calls = [];
  window.__TAURI__ = {
    core: { invoke(command, args) { window.__calls.push({ command, args: args || {} }); return Promise.resolve(null); } },
    event: { listen(name, back) { window.__listener = back; return Promise.resolve(() => {}); } },
  };
  window.__troubles = [];
  console.error = (...said) => window.__troubles.push(said.map((one) => String(one && one.stack || one)).join(" "));
</script>
<script type="module">
import { theRouteIsOpen } from "./event-route.js";
import "./the-failure-shape.js";
import "./reporting.js";
import "./the-escape-key.js";
import deliveries from "./deliveries.js";
const out = { states: {}, presses: {} };
const write = () => { out.troubles = window.__troubles; document.getElementById("out").textContent = JSON.stringify(out); };
window.addEventListener("error", (event) => { out.error = String(event.message); write(); });
window.addEventListener("unhandledrejection", (event) => { out.error = String(event.reason && event.reason.stack || event.reason); write(); });
try {
  await theRouteIsOpen();
  let sequence = 0;
  const settle = async () => { for (let at = 0; at < 12; at += 1) await new Promise((go) => setTimeout(go, 0)); };
  const tell = async (delivery) => {
    const one = structuredClone(delivery);
    sequence += 1;
    one.sequence = 1000 + sequence;
    window.__listener({ payload: one });
    await settle();
  };
  const shown = (el) => Boolean(el) && !el.closest("[hidden]") && el.getClientRects().length > 0;
  const root = () => document.getElementById("rpbox");
  const box = () => root().querySelector(".mbox");
  const byText = (words) => [...box().querySelectorAll("button")].filter(shown).find((b) => b.textContent.replace(/\u00a0/g, " ").trim() === words);
  const look = (state) => {
    const b = box();
    const rect = b.getBoundingClientRect();
    const about = document.getElementById("rpabout");
    const save = document.getElementById("rpsave");
    const choose = document.getElementById("rpchoose");
    const field = document.getElementById("rpsay");
    const cancel = [...b.querySelectorAll(".mact .btn.quiet")].find(shown);
    const savedLine = document.getElementById("rpsaved");
    const failure = b.querySelector(".rp-not-saved .failure-said");
    out.states[state] = {
      shown: shown(b),
      fits: rect.left >= 0 && rect.top >= 0 && rect.right <= window.innerWidth && rect.bottom <= window.innerHeight,
      sideways: b.scrollWidth > b.clientWidth || document.documentElement.scrollWidth > window.innerWidth,
      text: b.innerText,
      about: shown(about),
      aboutBold: shown(about) ? about.querySelector("b").textContent : "",
      aboutText: shown(about) ? about.textContent : "",
      aboutMoment: shown(about) ? about.lastElementChild.textContent : "",
      folder: shown(document.getElementById("rpwp")) ? document.getElementById("rpwp").textContent : "",
      noFolder: shown(document.getElementById("rpwpnone")),
      saveLive: shown(save) && !save.disabled,
      saveUnderway: shown(save) && save.classList.contains("wk"),
      saveName: save.getAttribute("aria-label") || "",
      chooseLive: shown(choose) && !choose.disabled,
      cancelLive: Boolean(cancel) && !cancel.disabled,
      fieldReadOnly: field.readOnly,
      typed: field.value,
      focused: document.activeElement ? (document.activeElement.id || (document.activeElement.textContent === "Close" ? "close" : document.activeElement.tagName)) : "",
      askShown: shown(field),
      saved: shown(savedLine) ? savedLine.textContent : "",
      savedMono: shown(savedLine) ? [...savedLine.querySelectorAll(".mono")].map((one) => one.textContent) : [],
      address: shown(b.querySelector(".rp-addr")) ? b.querySelector(".rp-addr").textContent : "",
      failure: failure && shown(failure) ? failure.textContent : "",
      failureDot: Boolean(failure && failure.querySelector(".fdot")),
    };
  };
  const sentence = "The frames aren’t on the drive but Smoozy says they arrived.";

  const crashLook = () => {
    const root=document.getElementById("rpcrash"), b=root.querySelector(".mbox"), r=b.getBoundingClientRect();
    return { text:b.innerText, bold:b.querySelector("b").textContent, shown:shown(root), focused:document.activeElement.id,
      fits:r.left>=0 && r.top>=0 && r.right<=innerWidth && r.bottom<=innerHeight, sideways:b.scrollWidth>b.clientWidth };
  };
  await tell(deliveries.crash); out.crash=crashLook();
  window.__calls.length=0; document.getElementById("rpcrash-not-now").click();
  out.crashDismiss={calls:structuredClone(window.__calls),hidden:document.getElementById("rpcrash").hidden};
  await tell(deliveries.crash); window.__calls.length=0; document.getElementById("rpfromcrash").click();
  out.crashReport={calls:structuredClone(window.__calls),hidden:document.getElementById("rpcrash").hidden};
  await tell(deliveries.crashReport);look("rp-crash");
  await tell(deliveries.crash);window.__calls.length=0;
  document.querySelector("#rpcrash .mbox").dispatchEvent(new KeyboardEvent("keydown",{key:"Escape",bubbles:true,cancelable:true}));
  out.crashEscape={calls:structuredClone(window.__calls),hidden:document.getElementById("rpcrash").hidden};
  await tell(deliveries.menu); look("rp-menu");
  document.getElementById("rpsay").value = sentence;
  window.__calls.length = 0;
  document.getElementById("rpsave").click();
  out.presses.save = window.__calls[0] || null;
  window.__calls.length = 0;
  document.getElementById("rpchoose").click();
  out.presses.choose = (window.__calls[0] || {}).command || null;
  window.__calls.length = 0;
  byText("Cancel").click();
  out.presses.cancel = { command: (window.__calls[0] || {}).command || null, hidden: root().hidden };

  await tell(deliveries.rp); look("rp");
  out.presses.fresh = document.getElementById("rpsay").value;
  document.getElementById("rpsay").value = sentence;
  await tell(deliveries.composing); look("rp-composing");
  window.__calls.length = 0;
  document.getElementById("rpsave").click();
  out.presses.savedAgain = window.__calls.length;
  await tell(deliveries.notSaved); look("rp-not-saved");
  await tell(deliveries.saved); look("rp-saved");
  window.__calls.length = 0;
  box().dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true }));
  out.presses.escape = { command: (window.__calls[0] || {}).command || null, hidden: root().hidden };

  await tell(deliveries.savedNoOpen); look("rp-saved-noopen");
  await tell(deliveries.noFolder); look("rp-nofolder");
  await tell(deliveries.chosen); look("rp-chosen");
  await tell(deliveries.savedElsewhere); look("rp-saved-elsewhere");

  // A failure shape, drawn by the shared half from its own event, and its
  // Report a problem… pressed.
  window.__calls.length = 0;
  await tell({
    kind: "presentation:a-failure-is-standing",
    subject: "run",
    said: {
      sentence: "Smoozy could not reach the server.",
      kind: "condition",
      subject: "fedcba9876543210fedcba9876543210",
      subjectKind: "server row",
      identityOrMark: "0123456789abcdef0123456789abcdef",
      key: "k",
      theSentenceForTheReport: "Smoozy could not reach the server.",
      at: "1754694840",
      controlCount: "1",
      control0Is: "report",
      control0Label: "Report a problem…",
    },
  });
  const report = [...document.querySelectorAll("#the-failures .rp-ask button")].find(shown);
  if (!report) throw new Error("the failure shape drew no report control");
  report.click();
  out.presses.fromAFailure = window.__calls.find((call) => call.command === "the_report_box_is_opened_from_a_failure") || null;
} catch (trouble) {
  out.error = String(trouble && trouble.stack || trouble);
}
write();
</script></body></html>"##;
}

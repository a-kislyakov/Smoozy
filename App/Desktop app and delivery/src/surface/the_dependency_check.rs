//! **The dependency check — the twin of the render settings panel.**
//!
//! The page half is `web/the-dependency-check.js`, and it draws inside piece
//! 5's panel-over-a-server mechanism: **the contract at the head of
//! `web/a-server.js` is what this piece obeys**, and nothing here builds a panel
//! of its own.
//!
//! **This file is the surface and not the deciding.** What a row's state is and
//! the action that fits it, the qualifiers a located answer carries, the order a
//! group's rows sit in, what a reading that gave nothing is, which endings a
//! sitting can have and what the live-job warning says are all another module's:
//! `App/Dependency Upload & Remap/`, `App/Render Settings/` for the four read
//! endings, and `src/surface/files.rs` for the browser's own live-job warning.
//! **Nothing here re-decides any of them.**
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! **The Design document and the heading**, and **the mockup state names**.
//! `Preparation/Design/Dependency Upload & Remap.md` carries no clause marker, so
//! the headings are the citation:
//!
//! - `Design/Dependency Upload & Remap § Where it sits` — a panel over the
//!   server, collapsing to a strip that shows the summary sentence.
//! - `§ Above the rows` — the summary sentence first under the title, the
//!   search's progress with its Stop, a folder that could not be read named
//!   in Settings' own words, and *Search folders…*.
//! - `§ A row`, `§ The seven states`, `§ How the rows are grouped and sorted`,
//!   `§ When more than one reference points at one thing`, `§ Acting on many
//!   rows`, `§ A guess the user rejected` — the rows, named once, in their
//!   groups, chosen the way a file manager chooses them.
//! - `§ Along the bottom` — the upload control alone.
//!
//! **Rebuilt 2026-10-18 for the dependency check's fresh look** (approved by
//! the Creator that day, both decisions yes; the account is `Audits/Build/
//! Step 10c/The dependency check rebuilt, with W1 — what landed.md`): the
//! row names the file once, with its notes in a fixed order; the seven
//! groups in the new order and names, two folded; no checkbox; the summary
//! sentence on the panel and the strip; a foot of one button; choosing rows
//! by click, Shift-click and Cmd-click.
//! - `§ This panel's own overwrite warning` — replace or stop, and the folder
//!   reading.
//! - `§ When the file could not be read at all` — the four endings, no rows.
//! - `§ When the connection drops mid-check` — the band, and the three acts
//!   that need the server made unavailable.
//! - `§ While it is still reading below a linked file` — the header's own line,
//!   and *Reading the file…* there while the file itself is read, with nothing
//!   under the header (the Creator's note 8 on the test copy).
//! - `§ The search folders on this panel` — the folders line, with the
//!   folders named and *Check again* on it wherever a check ran; where none
//!   of the file is held, no folders line, only *File not read* and one
//!   *Check dependencies* (the Creator's thirteenth notes, note 2); and the
//!   add box,
//!   which *Add* closes before the panel says once what the folder found
//!   (the Creator's note 5 on 0.1.8, the account `Audits/Build/Step 10c/The
//!   Creator's notes on 0.1.8, piece 3 — what landed.md`).
//! - `§ What this document left open, and how it was answered` — the
//!   statements above the rows.
//! - `Design/Whole app § Panels over a server` and `§ The failure shape` —
//!   piece 5's frame and the one shape, obeyed and not re-decided.
//!
//! **The mockup states this piece draws** are [`THE_STATES_THIS_PIECE_DRAWS`],
//! read out of `Preparation/Design/Mockup/Smoozy.html`'s own state list by a
//! test below. The account is `Audits/Build/Step 10a/Piece 9 — what landed.md`.
//!
//! ## What happens, said plainly — from step 10c's dependency wave piece 5
//!
//! - **Opening the check runs one.** [`the_check_is_started`] gathers what a
//!   sitting starts from and hands it to `crate::checking_a_file`, which reads
//!   each `.blend` through the machine's own worker, looks for every reference
//!   on the user's computer, writes each reading's findings through the
//!   record's keeper as it lands, and writes the check standing at the end.
//!   The page is told each time there is something new to draw. **What stops a
//!   check before anything is asked of the machine** — no live connection, no
//!   placement, no Blender to read with — is answered at once, and a root that
//!   gave nothing lands one of the four endings in [`the_check_gave_nothing`],
//!   which hands it to the diary.
//! - **What runs in the run's memory alone**: choosing rows (and the row a
//!   Shift-click runs from), the search's Stop
//!   (which also tells a running sitting to stop), and the cross on a failure.
//!   The selection is run-held by the plan's own footing (`Dependency Upload &
//!   Remap` F22), the search's per-folder outcome is run-held by decision
//!   (`Dependency Upload & Remap` F7), and a cleared failure changes nothing a
//!   record holds.
//! - **Every decision about the user's own computer is saved** — Locate…,
//!   ignore (the spec's *set aside*), restore, accept, use it, undo a guess, choose a folder, undo the
//!   shift, a search folder added — through `crate::checking_a_file`, one save
//!   per row, with the application's lock taken before the run's.
//! - **The acts that send something to the server send it** — *Upload*, a
//!   row's *Try again*, the overwrite box's *Replace* and a row's *Verify* —
//!   through `crate::the_found_files_sent`, off the drawing thread, since step
//!   10c's dependency wave piece 6. **The upload is taken here and then
//!   Transfers is shown**, where its request is. A row reads *already on the
//!   server* where this module's own placement record says so; see that
//!   file's head. **Admin access is offered nowhere** (`File Browser` P6a).
//!
//! ## The way in
//!
//! **A check is of a file, and the way in is `theCheckIsOpenedOn(server,
//! file)`** in `web/the-dependency-check.js`, with the file's exact bytes
//! (`File Browser` L1a). **`theCheckIsShownOn(server, file)` is the second way
//! in**: it opens the panel on the rows already there and starts nothing, which
//! is what the band's *Open the check* is, and Transfers' since the Creator's
//! note 8 on the test copy. `web/render-settings.js`'s band
//! hands it the file those settings are of, and `web/files.js`'s *Check
//! dependencies…* menu item and toolbar control hand it the chosen row's path —
//! closed on 2026-09-13, and a test in each of those two files' halves fails if
//! either goes back to the bare panel. A panel opened with no file still says
//! exactly that rather than guessing which file was meant.
//!
//! ## How a clause marker is written in this file
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every marker of
//! another document is written with that document at the site.** This module's
//! plan and this crate's collide on many letters, so no header list is used.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex, OnceLock};

use tauri::{Manager, State};

use smoozy_dependencies::bytes::{MachineLocation, RecordedLocation, StoredLocation};
use smoozy_dependencies::check::{compare_recorded, Attribution, Kind, LinkedItemStatement, Row};
use smoozy_dependencies::record::{HoldingFile, HowResolved, LocatedAnswer, ResolvedTo, ShiftKey};
use smoozy_dependencies::search::{Outstanding, WantedShape};
use smoozy_dependencies::rows::{the_qualifiers_of, Action, Qualifier, RowState};
use smoozy_dependencies::search::FolderVerdict;
use smoozy_dependencies::standing::Completion;
use smoozy_file_browser::warnings::AJobNamed;
use smoozy_render_settings::endings::Ending;
use smoozy_render_settings::file_half::{version_text, BlenderIdentity};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::events::{Event, Said, Subject, TheOneQueue};
use crate::surface::files::{the_live_job_warning, TheLiveJobWarning, Words};
use crate::surface::servers::{AControl, Words as AStripsWords};
use crate::typography::{Prose, Verbatim};

// ════════════════════════════════════════════════════════════════════════════
// The names this piece and its page agree on
// ════════════════════════════════════════════════════════════════════════════

/// **The panel key, and it is the drawing's own.** The same string
/// `src/surface/a_server.rs` holds; importing it rather than typing it is what
/// that piece's contract asks for.
pub const THE_DEPENDENCY_CHECK: &str = crate::surface::a_server::THE_DEPENDENCY_CHECK;

/// **The kind, declared beside the service that posts it.**
/// `web/the-dependency-check.js` names the same string.
pub const AN_ACT_ON_THIS_PANEL: &str = "the-dependency-check:an-act";

/// **The eighteen mockup states this piece draws**, exactly as
/// `Preparation/Design/Mockup/Smoozy.html`'s own `<select id="state">` names
/// them — the `chk-` options of its *A server* group. **The runner ruled on
/// 2026-09-12 that the mockup state names are what this half is counted by**,
/// and [`tests::the_eighteen_states_are_the_drawings_own`] reads the drawing.
/// *(Nine until the plan pass for the Creator's notes on 0.1.8 drew `chk-added`
/// and `chk-folders`; eleven until the drawing pass for the Creator's notes on
/// the test copy drew `chk-reading`; twelve until the dependency check's fresh
/// look drew `chk-clean` and `chk-sel`, 2026-10-18; fourteen until the drawing
/// pass for the Creator's thirteenth notes drew `chk-none`, the same day;
/// fifteen until the drawing pass for their fourteenth notes drew
/// `chk-nosearch`, the same day; sixteen until the eighteenth notes' design
/// pass drew `chk-sending`, 2026-10-24 — a row already uploading reads
/// *Uploading* and the foot counts only the rest. **Built 2026-10-25 by that
/// work's piece 3**, which reads this run's sends per file into the rows and
/// the foot, `Dependency Upload & Remap` A9a.)*
pub const THE_STATES_THIS_PIECE_DRAWS: [&str; 21] = [
    "chk-walk",
    "chk-unchecked",
    "chk-unsetup",
    "chk-clean",
    "chk-sel",
    "chk-cut",
    "chk-unread",
    "chk-unsrv",
    "chk-unpart",
    "chk-unstop",
    "chk-many",
    "chk-wasdep",
    "chk-over",
    "chk-overset",
    "chk-added",
    "chk-folders",
    "chk-reading",
    "chk-none",
    "chk-nosearch",
    "chk-onserver",
    "chk-sending",
];

// ════════════════════════════════════════════════════════════════════════════
// Every word the user reads
// ════════════════════════════════════════════════════════════════════════════

mod words {
    // The header. `§ Where it sits`, and `§ While it is still reading below a
    // linked file`.
    /// The panel's title as the drawing writes it — *Dependencies —
    /// kitchen_v14.blend* — the words with the dash, then the file's name as a
    /// value. The render settings panel's title is built the same way
    /// (must-do 17).
    pub const DEPENDENCIES: &str = "Dependencies —";
    pub const READ_WITH_BLENDER: &str = "Read with Blender";
    pub const LINKED_FILES: &str = "linked files";
    pub const LINKED_FILE: &str = "linked file";
    pub const STILL_READING_BELOW: &str = "Reading files linked from";
    pub const CANNOT_SAY_HOW_FAR: &str = "— progress unknown";
    /// **While the file itself is read**, in place of the reading line —
    /// that section's closing paragraph, the Creator's note 8 on the test
    /// copy.
    pub const READING_THE_FILE: &str = "Reading the file…";
    /// **While the walk over a reading's files runs**, in place of either line
    /// above, with *Stop* beside it — that section, 2026-11-04.
    pub const CHECKING_FILES_ON_THE_SERVER: &str = "Checking files on the server:";

    // The statements above the rows. `§ What this document left open, and how
    // it was answered`, first bullet, whose words Design settled.
    pub const IS_HERE_BUT_SOMETHING_IS_GONE: &str =
        "is here, but data linked from it is missing.";
    pub const BLENDER_DOES_NOT_SAY_WHICH: &str = "Blender does not say which.";
    // `Dependency Upload & Remap` G9a.
    pub const CANNOT_WORK_OUT_THE_BAKE_FOLDER: &str = "Can't resolve the bake folder";
    pub const BLENDER_MAY_HAVE_PACKED_IT: &str = "It may be packed in the .blend.";
    pub const UPLOADED_AS_A_DEPENDENCY_BEFORE: &str =
        "Uploaded before as a dependency.";
    pub const ITS_LOCATIONS_STILL_POINT_HOME: &str =
        "Its paths still point to your computer.";
    pub const THE_CONNECTION_DROPPED: &str = "Connection lost.";
    pub const THERE_MAY_BE_MORE: &str = "Check incomplete.";
    // `§ The search folders on this panel`: the folders line names them, three
    // then how many more, and *Check again* sits on it. The Creator's note 5 on
    // 0.1.8.
    pub const SEARCHED: &str = "Searched";
    pub const AND: &str = "and";
    pub const MORE_FOLDERS: &str = "more.";
    pub const YOUR_SEARCH_FOLDERS_CHANGED: &str = "Search folders changed since this search.";
    // **A searched folder's link, where the computer's file manager would not
    // open it** — worded as a job's downloaded files say it where they cannot
    // be opened (`Download` H8; the Creator's note 5 on the test copy). *Step
    // 13's words sweep took Smoozy's name off this copy and off Transfers'
    // own, a separate literal with the same words.*
    pub const COULD_NOT_OPEN_THE_FOLDER: &str = "Could not open this folder.";
    // What *Add* says once, in front of the statements.
    pub const ADDED: &str = "Added";
    pub const FOUND_AND_MORE: &str = "found,";
    pub const FOUND_AND_NO_MORE: &str = "found.";
    pub const STILL_MISSING: &str = "still missing.";
    pub const NOTHING_MORE_WAS_FOUND: &str = "Nothing more found.";
    pub const FULL_STOP: &str = ".";
    pub const COMMA: &str = ",";

    // `§ The seven states` — several failing for one reason, said once.
    pub const THESE_FILES_COULD_NOT_BE_WRITTEN: &str =
        "Could not upload these files.";
    pub const NO_PERMISSION_THERE: &str = "Permission denied.";
    pub const WHAT_THE_SERVER_SAID: &str = "What the server said";
    pub const FILES: &str = "Files";
    pub const FOLDER: &str = "Folder";

    // `§ Above the rows`. *Could not be read* is `Design/Settings § Search
    // folders`' own word for that state, and this panel names it in those words.
    pub const SEARCHING_YOUR_FOLDERS: &str = "Searching your folders —";
    pub const OF: &str = "of";
    pub const DONE: &str = "done";
    pub const STOP: &str = "Stop";
    pub const COULD_NOT_BE_READ: &str = "could not be read";
    pub const SEARCH_FOLDERS: &str = "Search folders…";

    // `§ Acting on many rows` — the bar, once two or more rows are chosen:
    // *20 selected · Accept 3 · Choose a folder… · Ignore · Reject match*.
    // The dependency check's fresh look, decision 2.
    pub const SELECTED: &str = "selected";
    pub const ACCEPT: &str = "Accept";
    pub const CHOOSE_A_FOLDER: &str = "Choose a folder…";
    pub const IGNORE: &str = "Ignore";
    pub const UNDO_SMOOZYS_GUESS: &str = "Reject match";

    // `§ Above the rows` — a shift, stated once at the top.
    pub const EVERYTHING_RECORDED_UNDER: &str = "Files recorded under";
    pub const WAS_FOUND_UNDER: &str = "were found under";
    pub const ROWS_RESOLVED: &str = "rows resolved.";
    pub const ROW_RESOLVED: &str = "row resolved.";
    pub const UNDO_THIS_CHANGE: &str = "Undo this change";

    // `§ When the file could not be read at all` — the four endings, worded for
    // a check. Only the one that knows what to do next carries a second
    // sentence.
    pub const THIS_BLENDER_COULD_NOT_OPEN: &str = "This Blender could not open this file.";
    pub const PICK_A_DIFFERENT_BLENDER: &str =
        "Pick another Blender in Render settings, then check again.";
    pub const COULD_NOT_REACH_THE_SERVER: &str = "Could not reach the server.";
    pub const THE_READ_DID_NOT_FINISH: &str = "Reading the file did not finish.";
    /// **`§ When the file could not be read at all`, 2026-11-04: the server took
    /// no reading and this server's setup stands stopped.**
    pub const SETUP_STOPPED: &str = "Setup stopped.";
    pub const FINISH_SETUP_THEN_CHECK_AGAIN: &str = "Finish setup, then check again.";
    pub const SET_UP_AGAIN: &str = "Set up again";
    pub const THE_CHECK_WAS_STOPPED: &str = "Check stopped.";
    pub const BLENDER: &str = "Blender";
    pub const SERVER: &str = "Server";
    pub const CHECK_AGAIN: &str = "Check again";
    /// **Where no check of the file has run since Smoozy was opened**, the
    /// one button under *File not read*: the check's own name, with no
    /// ellipsis, since it starts the check where it stands — `§ The search
    /// folders on this panel`, the Creator's thirteenth notes, note 2.
    pub const CHECK_DEPENDENCIES: &str = "Check dependencies";
    pub const BLENDER_OUTPUT: &str = "Blender output";
    pub const NOTHING_WAS_READ: &str =
        "File not read; dependencies unknown.";

    // `§ A row`, rewritten 2026-10-18 for the dependency check's fresh look
    // (approved by the Creator that day, both decisions yes): the name, then
    // its kind; one location; notes in a fixed order; the size; at most one
    // visible action; ⋯. No column headings and no checkbox.
    pub const MORE: &str = "More";
    /// Between a kind and how many files a set stands for — *Cache · 340
    /// files* — and between the parts of the summary sentence. Punctuation.
    pub const BETWEEN: &str = "·";
    pub const FILES_WORD: &str = "files";
    pub const FILE_WORD: &str = "file";
    // The kinds, in the industry's own words (Rule 9). *Image*, *Linked
    // .blend*, *Font*, *Cache* and *Sequence* are the Design document's own;
    // the rest are this builder's, one per kind the reading names.
    pub const LINKED_BLEND: &str = "Linked .blend";
    pub const IMAGE: &str = "Image";
    pub const SEQUENCE: &str = "Sequence";
    pub const MOVIE_CLIP: &str = "Movie clip";
    pub const SOUND: &str = "Sound";
    pub const FONT: &str = "Font";
    pub const VOLUME: &str = "Volume";
    pub const CACHE: &str = "Cache";
    pub const TEXT: &str = "Text";
    pub const OSL_SCRIPT: &str = "OSL script";
    pub const IES_PROFILE: &str = "IES profile";
    pub const DISPLACEMENT: &str = "Displacement";
    pub const BAKE: &str = "Bake";
    pub const STRIP: &str = "Strip";
    pub const PROXY: &str = "Proxy";
    pub const BRUSH_ICON: &str = "Brush icon";
    pub const IMPORTED_FILE: &str = "Imported file";
    pub const A_FILE: &str = "File";
    /// The one location where nothing was found: *The file says:* and the
    /// folder as the `.blend` writes it.
    pub const THE_FILE_SAYS: &str = "The file says:";

    // `§ A row`'s notes, in their fixed order.
    // 1. A change since the check ran, in the new state's colour.
    pub const FILE_NO_LONGER_FOUND: &str = "File no longer found";
    pub const READY_TO_UPLOAD: &str = "Ready to upload";
    // 2. A failure's own reason, in red.
    pub const UPLOAD_FAILED_NO_PERMISSION: &str = "Upload failed: permission denied";
    pub const COULD_NOT_UPDATE_ITS_FILE_PATHS: &str = "Uploaded; remap failed: a path did not update";
    pub const COULD_NOT_OPEN_IT_TO_REMAP: &str = "Uploaded; remap failed: Blender could not open it";
    pub const UPLOAD_FAILED_THE_PATH_IS_TOO_LONG: &str = "Upload failed: path too long for the server";
    pub const UPLOAD_FAILED: &str = "Upload failed";
    /// Transfers' own sentence for the same file, not a second one.
    pub const COULD_NOT_READ_ON_YOUR_COMPUTER: &str =
        crate::surface::transfers::words::COULD_NOT_READ_ON_YOUR_COMPUTER;
    // 3. How it was found, where that is not where the file says.
    pub const FOUND_THROUGH_SEARCH_FOLDER: &str = "Found in search folder";
    pub const MATCHES_IN: &str = "matches in";
    pub const SMOOZY_CHOSE_ONE: &str = "arbitrary pick";
    pub const SEMICOLON: &str = ";";
    pub const FOUND_BY_YOUR_MOVE: &str = "Found by your path change";
    pub const FROM: &str = "from";
    pub const TO: &str = "to";
    pub const YOU_CHOSE_THIS_FILE: &str = "You chose this file";
    pub const ONLY_THE_CAPITALS_DIFFER: &str = "Only the case differs:";
    pub const YOU_REJECTED: &str = "You rejected";
    // 4. What the row stands for.
    pub const THE_FILE_POINTS_TO_THIS: &str = "Referenced";
    pub const TIMES: &str = "times";
    pub const IT_MAY_ADD_MORE_ROWS: &str = "Uploading it may add more rows.";
    // 5. The spec's one exception to saying nothing about scenes —
    // `Specs/Dependency Upload & Remap`, *Checking a file*, Creator-stated
    // 2026-10-18.
    pub const NO_SCENE_USES_IT: &str = "Not used by any scene.";

    // `§ The seven states` — the screen's names, the spec keeping its own
    // (Rule 9), and the visible actions.
    pub const FAILED: &str = "Failed";
    pub const MISSING: &str = "Missing";
    pub const UPLOADED_BEFORE_NOT_VERIFIED: &str = "Uploaded before, not verified";
    pub const TO_UPLOAD_HEADING: &str = "To upload";
    pub const ALREADY_ON_THE_SERVER: &str = "Already on the server";
    pub const IGNORED: &str = "Ignored";
    pub const LOCATE: &str = "Locate…";
    pub const VERIFY: &str = "Verify";
    pub const RESTORE: &str = "Restore";
    pub const TRY_AGAIN: &str = "Retry";
    pub const USE_IT: &str = "Use it";

    // `§ Above the rows` — the summary sentence, the answer to the whole
    // check; collapsed, the strip shows it. `§ Where it sits` keeps
    // *Uploading 4 of 12* while the check's uploads run (the Creator's note 6
    // on 0.1.8).
    pub const NOTHING_IS_MISSING: &str = "Nothing is missing.";
    /// **A row to upload is a linked file nobody has read**, so what lies below
    /// it is not known — `§ Above the rows`, 2026-11-04.
    pub const LINKED_FILES_NOT_CHECKED_YET: &str = "Linked files not checked yet.";
    pub const FAILED_FIGURE: &str = "failed";
    pub const MISSING_FIGURE: &str = "missing";
    pub const NOT_VERIFIED_FIGURE: &str = "not verified";
    pub const FILES_TO_UPLOAD: &str = "files to upload,";
    pub const FILE_TO_UPLOAD: &str = "file to upload,";
    pub const ALREADY_ON_THE_SERVER_FIGURE: &str = "already on the server";
    pub const FOUND_SO_FAR_AT_THE_END: &str = "found so far.";
    pub const UPLOADING: &str = "Uploading";
    // `§ Along the bottom`: a press that sent nothing says why, once.
    // `§ A row`, `Dependency Upload & Remap` G6f.
    pub const LINKED_BY_FULL_PATH: &str =
        "Absolute path. Scripts may not reach it, so the render may differ. Make the path relative.";
    pub const NOTHING_TO_UPLOAD: &str = "Nothing to upload.";
    pub const FILE_IS_ALREADY_THERE: &str = "file is already on the server.";
    pub const FILES_ARE_ALREADY_THERE: &str = "files are already on the server.";
    pub const THEY_ARE_ALREADY_UPLOADING: &str = "These files are already uploading.";

    // `§ Along the bottom` — only the button.
    pub const UPLOAD: &str = "Upload";
    pub const OPENS: &str = "(";
    pub const CLOSES: &str = ")";
    pub const UPLOAD_THE: &str = "Upload the";
    pub const FOUND_SO_FAR: &str = "found so far";

    // `§ This panel's own overwrite warning`.
    pub const A_DIFFERENT_FILE_IS_THERE: &str = "A different file is already at that location";
    pub const SMOOZY_READ_THE_FILE_THERE: &str = "Compared by contents.";
    pub const IT_IS_NOT_THE_ONE: &str = "They differ.";
    pub const THIS_ROW_IS_A_FOLDER: &str = "This row is a folder.";
    pub const OF_ITS: &str = "of its";
    pub const FILES_DIFFER_AND_THE_SERVER_HOLDS: &str =
        "files differ, and the folder on the server holds";
    pub const FILES_YOURS_DOES_NOT: &str = "files yours does not.";
    pub const REPLACE: &str = "Replace";

    // `§ The search folders on this panel`.
    pub const ADD_A_SEARCH_FOLDER: &str = "Add a search folder";
    /// The box's one sentence, as Design writes it since the Creator's note 5
    /// on 0.1.8 (three sentences before).
    pub const IT_IS_ADDED_AND_SEARCHED: &str =
        "Adds it to your search folders in Settings and searches it for missing files.";
    pub const CANCEL: &str = "Cancel";
    pub const ADD: &str = "Add";
    /// **The box lists Settings' folders first, under this, with *Change
    /// them in Settings* below them** — the Creator's note 7 on the test copy,
    /// as the drawing's `#ckfold` writes it.
    pub const YOUR_SEARCH_FOLDERS: &str = "Your search folders";
    pub const CHANGE_THEM_IN_SETTINGS: &str = "Change them in Settings";

    // What an act answers where it cannot happen. **L8a: the screen says so
    // rather than showing a control that does nothing.** *(The sentence every
    // server's screen answered an act with while nothing instructed the
    // server went from this panel at step 10c's dependency wave piece 6: the
    // last acts that answered it now send.)*
    /// **A decision the record refused** — the Servers screen's own sentence
    /// for a write the record refused, not a fresh one (Rule 9).
    pub const THE_CHANGE_WAS_NOT_SAVED: &str = crate::surface::servers::words::THE_CHANGE_WAS_NOT_SAVED;
    /// **A server with no placement recorded, or none confirmed** — the render
    /// settings panel's own sentence for the same server.
    pub const NOT_SET_UP_ON_THIS_SERVER: &str = crate::surface::render_settings::words::NOT_SET_UP_ON_THIS_SERVER;
    /// **`Dependency Upload & Remap` G2's honest case: no Blender to read
    /// with** — the render settings panel's own sentence for `Render Settings`
    /// B5.
    pub const NO_BLENDER_ON_THIS_SERVER: &str = crate::surface::render_settings::words::NO_BLENDER_ON_THIS_SERVER;
    /// **`Dependency Upload & Remap` G2's other two cases, where the server has
    /// Blenders and none resolves for this file** — the render settings
    /// panel's own sentences for `Render Settings` B5's below-the-floor case
    /// and for a file with no saved-with version to match on (`Render Settings` B2a;
    /// `Render Settings` B5a). Borrowed rather than written again (Rule 9).
    /// Step 10c's twelfth-pass fix piece 6.
    pub const NOTHING_TO_MATCH_ON: &str = crate::surface::render_settings::words::NOTHING_TO_MATCH_ON;
    pub const SMOOZY_DOES_NOT_CHOOSE_BELOW_THE_FLOOR: &str =
        crate::surface::render_settings::words::SMOOZY_DOES_NOT_CHOOSE_BELOW_THE_FLOOR;
    /// **`Dependency Upload & Remap` K5's limit, said at the top once** —
    /// `Design/Dependency Upload & Remap § Above the rows`, E1-3's line as
    /// it stands there. Step 10c's twelfth-pass fix piece 6.
    ///
    /// *Changed 2026-10-01, fix piece 6b.* It read *relative paths are looked
    /// for only on the server*, and the check does not search the server for
    /// one: such a path has no folder on the user's computer to be measured
    /// from (`Dependency Upload & Remap` K5; `Dependency Upload & Remap` F27),
    /// so it is found, where it is, by the search of the user's search folders
    /// (`Dependency Upload & Remap` F3).
    pub const NO_RECORD_OF_WHERE_IT_CAME_FROM: &str =
        "Original location unknown. Searched in your search folders; matches apply to this server \
         only.";
    pub const OPENED_WITHOUT_A_FILE: &str = "No file chosen.";
    pub const THE_RECORD_WOULD_NOT_READ: &str = "Could not read Smoozy’s database.";
}

// ════════════════════════════════════════════════════════════════════════════
// What the run holds, and it is process memory for the life of the run
// ════════════════════════════════════════════════════════════════════════════

/// **What this panel is composed from.**
///
/// **A sitting is the readings of a machine and the decisions taken over them**
/// (`Dependency Upload & Remap` G14 and `Dependency Upload & Remap` G16), put
/// here by `crate::checking_a_file` as each reading lands. **The endings are Q9's**
/// (`Render Settings` Q9: a failed read changes nothing), so they are held beside
/// the panel and cleared by the cross. **The selection is run-held on the
/// plan's own footing** (`Dependency Upload & Remap` F22).
///
/// **It is a static rather than a value the shell manages**, because `main.rs` is
/// shared and a piece adds nothing to it — the same answer piece 8 gave.
#[derive(Default)]
pub struct WhatTheRunHolds {
    pub sittings: BTreeMap<String, ASitting>,
    pub endings: BTreeMap<String, AnEnding>,
    pub overwrites: BTreeMap<String, AnOverwrite>,
    pub selections: BTreeMap<String, BTreeSet<usize>>,
    /// **Where the last plain click or Cmd-click landed**, which a Shift-click
    /// chooses the range from — `§ Acting on many rows`, run-held beside the
    /// selection (`Dependency Upload & Remap` F22). An anchor with nothing
    /// chosen, or one past the rows, is not used.
    pub anchors: BTreeMap<String, usize>,
    /// **The sittings running now**, and whether the user asked each to stop
    /// — `Dependency Upload & Remap` G15: stopping is the desktop ceasing to
    /// issue readings, and `Dependency Upload & Remap` F6 stops the search.
    /// Written and read by `crate::checking_a_file`.
    pub checks_running: BTreeMap<String, bool>,
    /// **The checks reading the file itself**, by the panel's key, until that
    /// reading lands — `Design/Dependency Upload & Remap § While it is still
    /// reading below a linked file`, its closing paragraph: *Reading the
    /// file…*, and no table, no tools and no foot. Put in by
    /// [`the_file_is_about_to_be_read`] when a check is asked for, and by
    /// `crate::checking_a_file` when its sitting starts; taken off there
    /// when the root's reading lands and on every way out. **It stands over
    /// whatever the run held before**, since a fresh sitting rebuilds and
    /// never carries (`Dependency Upload & Remap` G17). The Creator's note 8
    /// on the test copy.
    pub files_being_read: BTreeSet<String>,
    /// **`Dependency Upload & Remap` G17a: how many search folders have been
    /// added to each check's sitting**, which a reading compares when its rows
    /// are about to land.
    pub the_adds_taken: BTreeMap<String, u32>,
    /// **`Dependency Upload & Remap` G17b: the readings this run keeps**, in
    /// memory alone.
    pub the_kept_readings: crate::kept_readings::TheKeptReadings,
    /// **`Dependency Upload & Remap` G17b: files going up to each machine**, which
    /// decides whether a reading may be kept.
    pub the_files_going_up: crate::kept_readings::TheFilesGoingUp,
    /// **`Dependency Upload & Remap` A9c: what a press that sent nothing said**,
    /// said once and let go of at the next act.
    pub what_a_send_said: BTreeMap<String, WhatASendSaid>,
    /// **`Dependency Upload & Remap` G14a: the checks a send ended during**, which
    /// go round once more when their walk ends.
    pub a_round_is_owed: BTreeSet<String>,
    /// **`Dependency Upload & Remap` A9e: the presses of the upload control whose
    /// send has not ended and whose check after it has not begun**, by check.
    pub the_sends_in_hand: BTreeMap<String, u32>,
    /// **`Dependency Upload & Remap` A9e's (b): the linked `.blend`s landed on each
    /// check, and how many of them a sitting has answered.**
    pub the_libraries_landed: BTreeMap<String, TheLibrariesLanded>,
}

/// **`Dependency Upload & Remap` A9e's (b).**
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TheLibrariesLanded {
    pub landed: u64,
    pub answered: u64,
}

/// **`Dependency Upload & Remap` A9e: why *Render* makes no job now.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyRenderIsHeld {
    /// A send of this file's check is counted, running or waiting.
    ASendIsGoing,
    /// A linked `.blend` landed, and the check that fixes it is running.
    ItsCheckIsRunning,
    /// A linked `.blend` landed, and no check is running or coming.
    CheckFirst,
}

/// **`Dependency Upload & Remap` A9e: whether this file's check holds *Render* on
/// this server**, read at the page's drawing and again at the press. **The send is
/// read before what it leaves behind**, and a library's landing is counted before
/// its file reads ended, so a press never finds a gap between the two.
pub fn what_holds_render(
    held: &Mutex<WhatTheRunHolds>,
    server: &str,
    file: &str,
) -> Option<WhyRenderIsHeld> {
    let key = keyed(server, file);
    let a_send_goes = crate::the_found_files_sent::a_send_is_going(server, file);
    let held = held.lock().ok()?;
    if a_send_goes || held.the_sends_in_hand.contains_key(&key) {
        return Some(WhyRenderIsHeld::ASendIsGoing);
    }
    let libraries = held.the_libraries_landed.get(&key)?;
    (libraries.landed > libraries.answered).then(|| {
        if held.checks_running.contains_key(&key) || held.files_being_read.contains(&key) {
            WhyRenderIsHeld::ItsCheckIsRunning
        } else {
            WhyRenderIsHeld::CheckFirst
        }
    })
}

/// **`Dependency Upload & Remap` A9e's (b): a linked `.blend` of this check was
/// published.**
pub fn a_library_landed(held: &Mutex<WhatTheRunHolds>, key: &str) {
    if let Ok(mut held) = held.lock() {
        held.the_libraries_landed.entry(key.to_string()).or_default().landed += 1;
    }
}

/// **`Dependency Upload & Remap` A9e's (a): a press of the upload control, held
/// until its send has ended and the check after it has begun**, however the press
/// ends.
struct TheSendInHand(String);

impl TheSendInHand {
    fn of(key: &str) -> TheSendInHand {
        if let Ok(mut held) = the_run_holds().lock() {
            *held.the_sends_in_hand.entry(key.to_string()).or_default() += 1;
        }
        TheSendInHand(key.to_string())
    }
}

impl Drop for TheSendInHand {
    fn drop(&mut self) {
        if let Ok(mut held) = the_run_holds().lock() {
            if let Some(count) = held.the_sends_in_hand.get_mut(&self.0) {
                *count -= 1;
                if *count == 0 {
                    held.the_sends_in_hand.remove(&self.0);
                }
            }
        }
    }
}

/// **`Dependency Upload & Remap` A9c: why a press of the upload control sent
/// nothing.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatASendSaid {
    /// Every file was found on the server already.
    TheyAreAlreadyOnTheServer(usize),
    /// Every file is on its way already.
    TheyAreAlreadyUploading,
}

/// **A check of this file is about to read it**, so the panel says so from
/// the moment it is asked for rather than from the moment the machine
/// answers its first question. **Answers whether this call put the mark
/// in**: a check already running owns its own mark, and one whose file is
/// already read is not marked again.
pub fn the_file_is_about_to_be_read(held: &mut WhatTheRunHolds, key: &str) -> bool {
    if held.checks_running.contains_key(key) {
        return false;
    }
    held.files_being_read.insert(key.to_string())
}

/// One of `Render Settings` Q8's four endings, with the Blender Smoozy's own
/// record says it ran where it ran one.
pub struct AnEnding {
    pub ending: Ending,
    pub read_with: Option<BlenderIdentity>,
    /// **The server took no reading and this server's setup stands stopped**, read
    /// off the machine as the ending landed — `§ When the file could not be read
    /// at all`, 2026-11-04.
    pub the_setup_stands_stopped: bool,
}

/// **One check sitting over one file on one server.**
pub struct ASitting {
    /// `Dependency Upload & Remap` G2: the version the check read with.
    pub read_with: BlenderIdentity,
    /// How many linked `.blend` files the loop has read so far.
    pub linked_files_read: usize,
    /// `Dependency Upload & Remap` G15: the actual machine path of the linked
    /// file being read, cleared before searching its returned rows.
    pub still_reading_below: Option<RecordedLocation>,
    /// **`Dependency Upload & Remap` Q5a: how far the walk over a reading's files
    /// has got** — done, of whole — while it runs.
    pub the_walk: Option<(usize, usize)>,
    /// `Dependency Upload & Remap` V10's three endings, or nothing while the
    /// sitting runs.
    pub completion: Option<Completion>,
    /// `Dependency Upload & Remap` G6a.
    pub linked_items: Vec<LinkedItemStatement>,
    /// `Dependency Upload & Remap` G9a.
    pub the_bake_folders_not_worked_out: Vec<smoozy_dependencies::check::TheBakeFolderNotWorkedOut>,
    /// `Dependency Upload & Remap` V9a.
    pub sent_up_as_a_dependency_before: bool,
    pub the_search: Option<TheSearchInHand>,
    pub the_shift: Option<TheShiftInHand>,
    pub the_once: Option<SeveralForOneReason>,
    pub rows: Vec<ACheckedRow>,
    /// **The folder chosen in the add box and not yet added** —
    /// `Design/Dependency Upload & Remap § The search folders on this panel`.
    pub the_chosen_search_folder: Option<Vec<u8>>,
    /// **Settings' list of search folders as this sitting read it**, in that
    /// list's order — what the folders line is compared with when the panel
    /// is drawn, so a list changed in Settings since is said (`Design/Dependency
    /// Upload & Remap § The search folders on this panel`). A folder added from
    /// the panel is added here too. The Creator's note 5 on 0.1.8.
    pub the_list_it_searched: Vec<Vec<u8>>,
    /// **What the last *Add* found**, said once in front of the statements and
    /// let go of at the user's next act on this panel. Run-held, never
    /// durable. The Creator's note 5 on 0.1.8.
    pub the_folder_added: Option<TheFolderAdded>,
    /// **What the sitting is of** — the root, its holding file, the machine
    /// and its layout — which the acts that need the server send from. Step
    /// 10c's dependency wave piece 6.
    pub of: Option<crate::checking_a_file::TheSittingIsOf>,
}

/// `Dependency Upload & Remap` F7: the search's per-folder outcome, run-held.
pub struct TheSearchInHand {
    pub folders: Vec<(Vec<u8>, Option<FolderVerdict>)>,
    /// **No longer said.** *The matches came from …* was struck on 2026-10-04
    /// (the Creator's note 5 on the test copy); the search still hands it in,
    /// and nothing on this panel reads it.
    pub the_matches_came_from: Option<Vec<u8>>,
    pub running: bool,
    /// **The moment of the search these verdicts came from** — `Dependency
    /// Upload & Remap` F1a: *"Where the verdict is drawn it carries the moment
    /// of the search it came from"* (`State, identity and recovery` D5). It is
    /// the search's own report's reading (`PerFolderOutcome::at`), carried
    /// here rather than taken a second time; `None` where the search reported
    /// no folder. Settings draws it beside *could not be read* (step 10c's
    /// piece 17 of the 38).
    pub searched_at: Option<std::time::SystemTime>,
}

/// **A search folder added from the panel, and what its search came to** —
/// the rows that were *Not found* when it was added, counted as the ones it
/// found and the ones still not found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheFolderAdded {
    pub folder: Vec<u8>,
    pub found: usize,
    pub still_not_found: usize,
}

/// `Dependency Upload & Remap` F19's move, as it stands on screen.
pub struct TheShiftInHand {
    pub old_root: Vec<u8>,
    pub new_root: Vec<u8>,
    /// `Dependency Upload & Remap` K6's key, which F20's undo enumerates by.
    pub key: ShiftKey,
}

/// **Several rows failing for one reason** — `Design/Whole app`'s once rule, as
/// `§ The seven states` applies it.
pub struct SeveralForOneReason {
    pub folder: Vec<u8>,
    pub rows: Vec<usize>,
    pub cleared: bool,
}

/// **Why a row did not go up** — the row's own reason (`Dependency Upload &
/// Remap` V3a names it a statement and not a warning).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyItDidNotGoUp {
    TheServerRefusedTheWrite,
    SmoozyCouldNotOpenItToRemapItsPaths,
    /// **A linked `.blend` opened, and not every path in it could be pointed at
    /// the server** — the fixing's own report (`Dependency Upload & Remap`
    /// M14), the second of the spec's two reasons a row is *could not be
    /// fixed*. The 38's piece 7.
    SmoozyCouldNotRemapItsPaths,
    /// **`Dependency Upload & Remap` A5**: the mirror will not fit the
    /// machine's byte budgets. Step 10c's dependency wave piece 6.
    ThePathIsTooLong,
    /// The file could not be read on the user's computer.
    ItCouldNotBeReadHere,
    /// Any other ending that did not publish.
    TheUploadFailed,
}

/// **One row as the sitting holds it.** Every field is another module's own
/// type where one exists.
#[derive(Clone)]
pub struct ACheckedRow {
    /// `Dependency Upload & Remap` G10: the references it stands for.
    pub row: Row,
    pub kind: Kind,
    /// `Dependency Upload & Remap` G20a.
    pub scenes: Attribution,
    /// **Which group the row sits in**, which does not move under the user's
    /// hand — `§ How the rows are grouped and sorted`.
    pub state_when_the_check_ran: RowState,
    pub state: RowState,
    /// **`Dependency Upload & Remap` K3's holding file** the row's references
    /// sit in.
    pub holding: HoldingFile,
    /// **`Dependency Upload & Remap` K4's located answer for every reference
    /// behind the row**, in the order of `row.references` — the record stays
    /// per reference while the row is per thing (`Dependency Upload & Remap`
    /// G10), so a decision on the row writes each of them.
    pub answers: Vec<LocatedAnswer>,
    /// What each reference is looked for as (`Dependency Upload & Remap` F28),
    /// in the same order.
    pub wanted: Vec<Outstanding>,
    /// `Dependency Upload & Remap` F10's capitals-only proposal.
    pub proposal: Option<StoredLocation>,
    /// `Dependency Upload & Remap` G23: how many files a set row stands for, and
    /// `Dependency Upload & Remap` F28's line of 2026-11-02: how many a
    /// folder-shaped row sends.
    pub members: Option<usize>,
    pub size: Option<u64>,
    pub why_it_did_not_go_up: Option<WhyItDidNotGoUp>,
    pub its_failure_was_cleared: bool,
    /// **`Dependency Upload & Remap` Q5a: the files and bytes of a set or folder row
    /// Q5 found already up**, which the send will not take.
    pub the_part_already_up: Option<(usize, u64)>,
}

/// **`Dependency Upload & Remap` G11b: the rows standing for one real file, in
/// the same state when the check ran, the first of each at its head.** In the
/// order the rows first name them.
pub fn the_real_files(rows: &[ACheckedRow]) -> Vec<Vec<usize>> {
    let mut places: std::collections::HashMap<Vec<u8>, Vec<(RowState, usize)>> =
        std::collections::HashMap::new();
    let mut files: Vec<Vec<usize>> = Vec::new();
    for (at, one) in rows.iter().enumerate() {
        let of_this_file = places.entry(one.the_real_file()).or_default();
        match of_this_file.iter().find(|(state, _)| *state == one.state_when_the_check_ran) {
            Some((_, place)) => files[*place].push(at),
            None => {
                of_this_file.push((one.state_when_the_check_ran, files.len()));
                files.push(vec![at]);
            }
        }
    }
    files
}

/// **Every figure counts a real file once** (`Dependency Upload & Remap` G11b):
/// the rows matching, the first of each real file.
pub fn one_row_per_real_file(rows: &[ACheckedRow], wanted: impl Fn(&ACheckedRow) -> bool) -> Vec<usize> {
    let mut seen = std::collections::HashSet::new();
    rows.iter()
        .enumerate()
        .filter(|(_, one)| wanted(one) && seen.insert(one.the_real_file()))
        .map(|(at, _)| at)
        .collect()
}

/// `Dependency Upload & Remap` G11b and `Dependency Upload & Remap` V12:
/// the warning uses the panel's file count in each state.
/// Confirmation still checks every reference and its holding file.
pub(crate) fn the_warning_counts(
    rows: &[ACheckedRow],
    mut counts: smoozy_dependencies::standing::Counts,
) -> smoozy_dependencies::standing::Counts {
    let count = |state| one_row_per_real_file(rows, |one| one.state == state).len();
    counts.not_found = count(RowState::NotFound);
    counts.could_not_be_sent = 0;
    counts.could_not_be_fixed = 0;
    // The panel has one failed group for both kinds of failure.
    for at in one_row_per_real_file(rows, |one| is_a_failure(one.state)) {
        match rows[at].state {
            RowState::CouldNotBeSent => counts.could_not_be_sent += 1,
            RowState::CouldNotBeFixed => counts.could_not_be_fixed += 1,
            _ => unreachable!(),
        }
    }
    counts.found_but_not_sent = count(RowState::FoundOnYourComputer);
    counts
}

impl ACheckedRow {
    /// **`Dependency Upload & Remap` Q5a: the files and bytes a send of this row
    /// will take.**
    pub fn what_a_send_would_take(&self) -> (usize, u64) {
        let (files, bytes) = self.the_part_already_up.unwrap_or((0, 0));
        (
            self.members.unwrap_or(1).saturating_sub(files),
            self.size.unwrap_or(0).saturating_sub(bytes),
        )
    }

    /// **`Dependency Upload & Remap` G11b: what makes two rows one real file** —
    /// the location found, a set's with its pattern; with nothing found, the
    /// whole location the render reads, and where that cannot be worked out, the
    /// holding file's own text of it.
    pub fn the_real_file(&self) -> Vec<u8> {
        let resolved = self.the_answer().and_then(|answer| answer.resolution.as_ref());
        if let Some(resolution) = resolved {
            let mut file = b"found ".to_vec();
            file.extend_from_slice(resolution.to.location().as_bytes());
            if matches!(resolution.to, ResolvedTo::AFolderForASetRow(_)) {
                file.push(0);
                if let Some(pattern) = self.wanted.first().and_then(|wanted| wanted.the_sets_pattern()) {
                    file.extend_from_slice(pattern);
                }
            }
            return file;
        }
        let source = crate::checking_a_file::the_source_folder(&self.holding);
        let whole = self.wanted.first().and_then(|wanted| {
            smoozy_dependencies::shift::the_full_old_location(&wanted.the_location_read, source.as_deref())
        });
        match whole {
            Some(whole) => {
                let mut file = b"asked ".to_vec();
                file.extend_from_slice(&whole);
                file
            }
            None => format!("{:?} {:?}", self.holding, self.row.ordering_key()).into_bytes(),
        }
    }

    /// **The answer the row shows** — its ordering key's (`Dependency Upload &
    /// Remap` G11a), which is the reference the column draws.
    pub fn the_answer(&self) -> Option<&LocatedAnswer> {
        let key = self.row.ordering_key();
        self.answers
            .iter()
            .find(|answer| &answer.key.reference == key)
            .or_else(|| self.answers.first())
    }
}

/// **The overwrite question** — `Dependency Upload & Remap` Q8, over one row.
pub struct AnOverwrite {
    pub row: usize,
    pub location: Vec<u8>,
    pub jobs: Vec<AJobNamed>,
    pub labels: Vec<(String, String)>,
    pub the_folder: Option<TheFolderDiffers>,
}

/// Q8's widening over a set row: the members a send would write over, and the
/// members the local folder does not hold (`Dependency Upload & Remap` Q8).
pub struct TheFolderDiffers {
    pub differ: usize,
    pub members: usize,
    pub only_on_the_server: usize,
}

/// The one of these there is.
pub fn the_run_holds() -> &'static Mutex<WhatTheRunHolds> {
    static HELD: OnceLock<Mutex<WhatTheRunHolds>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(WhatTheRunHolds::default()))
}

/// The one key every map above is keyed by. **A rendering and never a meaning**.
pub(crate) fn keyed(server: &str, file: &str) -> String {
    format!("{server}\u{1f}{file}")
}

// ════════════════════════════════════════════════════════════════════════════
// What the page is handed
// ════════════════════════════════════════════════════════════════════════════

/// **The whole panel, in one answer** — H34's closed response type.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheCheck {
    pub server: Verbatim,
    /// **The file as this panel keys it**; an act's event carries the same
    /// text, which is how the page tells its own acts apart.
    pub file: Verbatim,
    /// **The file's exact bytes**, which every act hands back (`File Browser`
    /// L1a, must-do 22).
    pub file_bytes: Vec<u8>,
    pub title: Words,
    /// **The file's name in the title is a link into Files** — the folder and
    /// the name it hands over, composed the way Render Settings composes its
    /// own (`render_settings::the_file_in_files`). The ninth notes, note 5.
    pub show_in_files: Option<crate::surface::render_settings::TheFileInFiles>,
    /// **What is said in place of the check**, where there is nothing to check.
    pub instead: Option<Prose>,
    pub the_reading_line: Vec<Words>,
    pub still_reading: Option<Words>,
    /// **The summary sentence, first under the title** — `§ Above the rows`.
    /// Nothing where nothing was read.
    pub the_summary: Option<TheSummary>,
    pub the_statements: Vec<AStatement>,
    pub the_once: Option<TheFailure>,
    pub the_ending: Option<TheFailure>,
    /// ***Stop* beside the walk's count**, the one line it is drawn on.
    pub the_walks_stop: Option<Prose>,
    pub nothing_was_read: Option<Prose>,
    /// **The one button under *File not read* where no check of the file
    /// is held** — *Check dependencies*, the drawing's `.btn` in `.cnone`
    /// (`chk-none`). There is no folders line then, since nothing was
    /// searched, and *Check again* is only for a check that ran. The
    /// Creator's thirteenth notes, note 2.
    pub the_check_to_start: Option<AControl>,
    pub the_tools: Option<TheTools>,
    pub the_selection: Option<TheSelectionBar>,
    pub the_shift: Option<TheShift>,
    pub the_table: Option<TheTable>,
    pub the_foot: Option<TheFoot>,
    pub the_overwrite: Option<TheOverwrite>,
    pub the_folder_box: TheFolderBox,
    /// **Which of `State, identity and recovery` §8.7's durable subjects the
    /// machine's own words are asked for**, so the page names none itself.
    pub the_machines_words_subject: Verbatim,
    pub the_strip_title: AStripsWords,
    /// **What the strip says, in parts drawn side by side** — the summary
    /// sentence (`Design/Dependency Upload & Remap § Where it sits`, since the
    /// dependency check's fresh look of 2026-10-18), or *Uploading 4 of 12*
    /// while the check's uploads run (the Creator's note 6 on 0.1.8). Empty
    /// where there is nothing to say.
    pub the_strip_reading: Vec<AStripsWords>,
}

/// One statement above the rows — not a row, not a warning, no action.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AStatement {
    pub which: Verbatim,
    pub said: Words,
    /// The drawing bolds the first sentence of the dropped-connection band.
    pub its_first_sentence_is_bold: bool,
    /// **The drawing bolds the last sentence of the folders line** where it
    /// says the search folders changed since this search.
    pub its_last_sentence_is_bold: bool,
    /// **The full path of each of the first values, in order**, which the page
    /// shows on hover — the folders line names a folder by its last folder
    /// name (`Design/Dependency Upload & Remap § The search folders on this
    /// panel`). Empty where nothing is shown on hover.
    pub the_full_paths: Vec<Verbatim>,
    /// **The act each named folder is a link to**, handed back with the
    /// folder's place in the line as its row — on the folders line, opening
    /// it in the computer's own file manager (the Creator's note 5 on the
    /// test copy). `None` where the names are not links.
    pub its_folders_open_with: Option<Verbatim>,
    /// **The one control on the line** — *Check again* on the folders line.
    pub control: Option<AControl>,
}

/// A statement with no hover, no control and nothing bold but what is asked.
fn a_statement(which: &str, said: Words, its_first_sentence_is_bold: bool) -> AStatement {
    AStatement {
        which: Verbatim::new(which),
        said,
        its_first_sentence_is_bold,
        its_last_sentence_is_bold: false,
        the_full_paths: Vec::new(),
        its_folders_open_with: None,
        control: None,
    }
}

/// **The one failure shape's parts, composed here and drawn by
/// `theFailureShape`.** The order of the controls is this list's.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheFailure {
    pub which: Verbatim,
    /// **The plain sentence, and it is whole.** X7: a name, a version or a path
    /// never travels inside it; they sit under it, each under its own name.
    pub sentence: Prose,
    pub the_second_sentence: Option<Prose>,
    pub values: Vec<AValue>,
    pub controls: Vec<AShapesControl>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AValue {
    pub name: Prose,
    pub value: Verbatim,
}

/// One control in the failure shape's own order. `is` is the token
/// `web/the-failure-shape.js` routes on.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AShapesControl {
    pub is: Verbatim,
    pub label: Prose,
    pub act: Verbatim,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheTools {
    pub how_far: Option<Words>,
    pub stop: Option<Prose>,
    pub unreadable: Vec<Words>,
    pub search_folders: Prose,
}

/// **The bar over a selection of two or more rows** — `§ Acting on many
/// rows`: *20 selected · Accept 3 · Choose a folder… · Ignore · Reject
/// match*. **It shows only the acts that apply to at least one chosen row**,
/// so an act that applies to none is `None` and is not drawn.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheSelectionBar {
    pub how_many: Words,
    /// *Accept K*, K the chosen rows with a proposal standing.
    pub accept: Option<Words>,
    pub choose_a_folder: Option<Prose>,
    pub ignore: Option<Prose>,
    pub undo_the_guess: Option<Prose>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheShift {
    pub said: Words,
    pub resolved: Words,
    pub undo: Prose,
}

/// **The summary sentence — the answer to the whole check** — `§ Above the
/// rows`, the first line under the title. **In stretches, each in its own
/// colour** (failed and missing red, not verified amber, already on the
/// server green, the rest none), drawn side by side with a space between
/// them unless a stretch joins the one before it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheSummary {
    pub parts: Vec<AFigure>,
}

/// **One stretch of the summary sentence and its colour.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AFigure {
    pub said: Words,
    pub colour: Verbatim,
    /// **It follows the stretch before it with nothing between them** — the
    /// comma of *, found so far.* and the closing full stop.
    pub joins: bool,
}

/// **The rows, in their groups.** No column headings and no checkbox since
/// the dependency check's fresh look (`§ A row`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheTable {
    pub groups: Vec<AGroup>,
    /// **The key that adds a row to the chosen ones or takes it away** —
    /// `meta` (Cmd) on a Mac, `ctrl` elsewhere (`§ Acting on many rows`).
    /// Which computer this is is the application's to say, so the page reads
    /// nothing of the window to know it.
    pub adds_a_row_with: Verbatim,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AGroup {
    pub which: Verbatim,
    pub heading: Prose,
    /// **The state's colour, which says what the user has to do** — `bad`,
    /// `busy`, `ok`, `idle`, or empty for *To upload*, which takes no colour
    /// and no dot. `Design/Dependency Upload & Remap § The seven states`,
    /// Creator-stated 2026-10-08 (sixth notes, note 4).
    pub colour: Verbatim,
    /// **How many rows the group holds**, and on *Already on the server* its
    /// size beside: *1 · 128 MB*.
    pub how_many: Words,
    /// **Folded away** — *Already on the server* and *Ignored*.
    pub folded: bool,
    pub rows: Vec<ARow>,
}

/// **One row, naming the file once** — `§ A row` since 2026-10-18.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ARow {
    /// The row's place in the sitting, which is what an act hands back.
    pub at: Verbatim,
    /// **The name** — the last part of what the file records.
    pub name: Verbatim,
    /// `§ A row`: *a row that is itself another `.blend` is coloured*, on its
    /// name.
    pub a_linked_blend: bool,
    /// **Its kind**, and how many files where it stands for a set:
    /// *Image*, *Cache · 340 files*.
    pub kind: Words,
    /// **One location**: the folder on this computer it uploads from, or *The
    /// file says:* and the folder as the `.blend` writes it.
    pub location: Words,
    /// **`Dependency Upload & Remap` G3a: the reading's own bytes**, shown on
    /// hover over the location, exactly as the `.blend` writes them.
    pub the_file_says: Verbatim,
    /// **Every note that applies, in `§ A row`'s fixed order.**
    pub notes: Vec<ANote>,
    pub size: Option<Verbatim>,
    /// **At most one visible action, and only where the state needs one.**
    pub action: Option<AControl>,
    pub more: Prose,
    pub more_items: Vec<AControl>,
    /// **Whether the row is chosen**, which takes the selected ground.
    pub chosen: bool,
    /// **Whether a click chooses it at all** — an ignored row is not in the
    /// list a selection is made from (`§ Acting on many rows`).
    pub choosable: bool,
}

/// **One note on a row**, in its colour, with what it names in full on hover
/// where it names a file by its last part.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ANote {
    pub said: Words,
    pub colour: Verbatim,
    pub hover: Option<Verbatim>,
    /// **A failure still standing**: the note is its reason, in red, and the
    /// row carries a cross last among its buttons, after its action and ⋯ —
    /// the drawing's *Failed* rows. `said` is empty.
    pub failure: Option<TheRowsFailure>,
}

/// **A failure standing on a row**: the reason, drawn as a red note, and the
/// cross's label, drawn among the row's buttons.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheRowsFailure {
    pub sentence: Prose,
    pub clear_this: Prose,
}

/// **`§ Along the bottom`: only the button** — *Upload 2 files (90 MB)*, or
/// *Upload the 4 found so far*. The figures are the summary sentence's.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheFoot {
    pub upload: Words,
    pub upload_is_unavailable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheOverwrite {
    pub title: Prose,
    pub what_was_read: Vec<Prose>,
    pub location: Verbatim,
    /// **The browser's own live-job warning, not a second one** —
    /// `Dependency Upload & Remap` V3 gives its wording to `File Browser`, and
    /// this is that module's composition called.
    pub live_jobs: Option<TheLiveJobWarning>,
    pub the_folder: Option<Words>,
    pub stop: Prose,
    pub replace: Prose,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheFolderBox {
    pub opener: Prose,
    pub title: Prose,
    /// **Settings' search folders, listed first**, full paths in Settings'
    /// order, to read and never to edit — `Design/Dependency Upload & Remap §
    /// The search folders on this panel`, the Creator's note 7 on the test
    /// copy. Values (X7, X9). Empty where the list is empty or would not read,
    /// and then none of the three is drawn.
    pub the_folders: Vec<Verbatim>,
    pub your_search_folders: Prose,
    /// ***Change them in Settings***, which opens Settings' own list.
    pub change_them_in_settings: Prose,
    pub choose_a_folder: Prose,
    /// **The folder chosen and not yet added**, a value (X7, X9).
    pub chosen: Option<Verbatim>,
    pub what_adding_does: Vec<Words>,
    pub cancel: Prose,
    pub add: Prose,
}

// ════════════════════════════════════════════════════════════════════════════
// The commands
// ════════════════════════════════════════════════════════════════════════════

/// **The whole panel, in one answer.** A command rather than an event: the page
/// asking for the facts it draws, which is the shape every screen before this
/// one has. **A page that comes back asks again** (P8).
#[tauri::command]
pub fn the_dependency_check_panel<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    started: State<'_, Mutex<crate::start::Started>>,
    server: String,
    file: Vec<u8>,
) -> TheCheck {
    let file = crate::surface::render_settings::the_path_as_it_is_keyed(&file);
    // **What the user calls the server, and Settings' list of search folders**,
    // read once and let go of before the run's own lock is taken (must-do 5).
    // The list is read through its keeper, never here (`Dependency Upload &
    // Remap` F1); where it would not read, nothing is compared.
    let (the_servers_name, the_list_in_settings) = match started.lock() {
        Ok(started) => (
            crate::surface::files::what_the_server_is_called(&started, &server),
            started.record.as_ref().and_then(|record| {
                crate::record::dependency_records::the_searched_folder_list::read(record.connection())
                    .ok()
                    .map(|list| list.folders)
            }),
        ),
        Err(_) => (None, None),
    };
    // **The check's uploads of this file, as Transfers lists them**, read
    // with no lock of this panel's held — the Creator's note 6 on 0.1.8.
    let uploading = crate::surface::transfers::the_checks_uploads_on(&app, &server, &file);
    // **And which files are already on their way to this server, file by
    // file** — every time the panel is shown, so coming back to it from
    // Transfers shows which are going (`Dependency Upload & Remap` A9a, the
    // Creator's eighteenth notes, note 6).
    let on_its_way = crate::the_found_files_sent::the_files_on_their_way_to(&server);
    let a_send_is_being_counted = crate::the_found_files_sent::a_send_is_being_counted(&server, &file);
    let known = WhatElseIsKnown { the_list_in_settings, uploading, on_its_way, a_send_is_being_counted };
    match the_run_holds().lock() {
        Ok(held) => the_panel_knowing(&held, &server, &file, the_servers_name.as_deref(), &known),
        // **L8a rather than an emptied panel.**
        Err(_) => {
            let mut panel = the_panel(&WhatTheRunHolds::default(), &server, "", None);
            panel.instead = Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ));
            panel
        }
    }
}

/// **The check is asked for** — the user asked for it by opening it.
///
/// **`Dependency Upload & Remap` G1 and `Dependency Upload & Remap` G14, driven**: the sitting runs off the
/// drawing thread (P5) in `crate::checking_a_file`, and the page is told each
/// time it has something new to draw (P7). **What this answers at once** is
/// only what stops a check before anything is asked of the machine — no live
/// connection, no placement, no Blender to read with (`Dependency Upload &
/// Remap` G2) — and nothing else. **Where the Blender cannot be resolved
/// without asking the machine for its list, the answer comes from the
/// sitting's thread** (step 10c's twelfth-pass fix piece 6).
#[tauri::command]
pub fn the_check_is_started<R: tauri::Runtime>(app: tauri::AppHandle<R>, server: String, file: Vec<u8>) {
    let keyed_as = crate::surface::render_settings::the_path_as_it_is_keyed(&file);
    let said = if keyed_as.is_empty() {
        None
    } else {
        the_check_is_begun(&app, &server, &keyed_as, &file)
    };
    if let Some(queue) = app.try_state::<Arc<TheOneQueue>>() {
        the_queue_is_told(&queue, an_act_said(&server, &keyed_as, said));
    }
}

/// **The local record, reached by the sitting off the drawing thread** — the
/// application's lock taken for each act alone.
struct TheApplicationsRecord<R: tauri::Runtime> {
    app: tauri::AppHandle<R>,
}

impl<R: tauri::Runtime> crate::uploading::TheRecordAnUploadWrites for TheApplicationsRecord<R> {
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
        use tauri::Manager;
        let Some(started) = self.app.try_state::<Mutex<crate::start::Started>>() else {
            return false;
        };
        let Ok(started) = started.lock() else {
            return false;
        };
        match started.record.as_ref() {
            Some(record) => {
                act(record.connection());
                true
            }
            None => false,
        }
    }
}

/// **Everything a sitting starts from**, gathered with the application's lock
/// held for the gathering alone, and the sitting started.
fn the_check_is_begun<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    file: &str,
    bytes: &[u8],
) -> Option<Prose> {
    use crate::checking_a_file::{TheSittingIsOf, TheUsersOwnComputer};
    use smoozy_dependencies::bytes::{MachineLocation, StoredLocation};
    use smoozy_dependencies::record::MachineKey;
    use smoozy_file_browser::request::KeepsTheRequest as _;
    use smoozy_file_browser::uploading::{where_it_came_from, WhereItCameFrom};
    use tauri::Manager;

    let queue = app.try_state::<Arc<TheOneQueue>>().map(|queue| Arc::clone(queue.inner()))?;
    let point = app
        .try_state::<Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_sessions_of(server))
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live);

    // **The application's own lock, for the gathering alone.**
    let started = app.try_state::<Mutex<crate::start::Started>>()?;
    let (row, requests, name) = {
        let Ok(started) = started.lock() else {
            return Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ));
        };
        let name = crate::surface::files::what_the_server_is_called(&started, server);
        let Some(record) = started.record.as_ref() else {
            return Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ));
        };
        let row = crate::record::server_rows::all(record.connection())
            .ok()
            .and_then(|rows| rows.into_iter().find(|row| row.local_server_identifier == server));
        let requests = crate::record::upload_requests::UploadRequests::over(record.connection())
            .every_request_for(&an_identity(server));
        (row, requests, name)
    };

    // **No live connection: `Render Settings` Q8's first ending, and nothing
    // asked** — handed to the diary once, the way every ending is.
    let Some(point) = point else {
        the_ending_lands(app, server, file, Ending::TheMachineCouldNotBeReached, None, false, name.as_deref());
        return None;
    };
    let Some(row) = row else {
        return Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ));
    };
    let crate::servers::installation::WhereThePackageAreaIs::At(package_area) =
        crate::servers::installation::where_the_package_area_is(&row)
    else {
        return Some(Prose::new(words::NOT_SET_UP_ON_THIS_SERVER));
    };
    let Some(installation) = row
        .last_confirmed_installation_identity
        .as_deref()
        .and_then(|text| Identity::read_back(IdentityKind::Installation, text))
    else {
        return Some(Prose::new(words::NOT_SET_UP_ON_THIS_SERVER));
    };
    let server_row = an_identity(server);
    // **`Server Setup` B9: Smoozy's own Blender area is not a candidate**, at
    // the exact path the row's recorded layout composes — what the render
    // settings page passes to the same ask.
    let layout = crate::servers::installation::the_layout_recorded_for(&row);
    let own_area = layout.as_ref().map(smoozy_server_setup::blender::the_blender_area);

    // **`Dependency Upload & Remap` G2: the Blender the job will render with**,
    // resolved at once where the page holds the server's list, and answered at
    // once where none resolves. **Where no list is held, or no saved-with
    // version, it is resolved on the sitting's thread**, because that asks the
    // machine ([`the_blender_the_check_reads_with`]).
    let with_now = match crate::surface::render_settings::the_run_holds().lock() {
        Ok(page) => match crate::surface::render_settings::the_version_resolved(&page, server, file) {
            smoozy_render_settings::version::Resolution::NoVersionResolves(
                smoozy_render_settings::version::WhyNone::NoMachineInHand
                | smoozy_render_settings::version::WhyNone::NoSavedWithVersionToMatchOn,
            ) => None,
            resolution => Some(the_blender_resolved(&resolution)),
        },
        Err(_) => return Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ)),
    };
    let with_now = match with_now {
        Some(Ok(with)) => Some(with),
        Some(Err(NoBlenderToReadWith::Said(said))) => return Some(Prose::new(said)),
        Some(Err(NoBlenderToReadWith::TheServerCouldNotBeAsked)) | None => None,
    };

    // **`Dependency Upload & Remap` K3's holding file: `File Browser` N6's
    // answer**, or K5's machine location where it has none (`File Browser`
    // N7a's third state). A request record that would not read is K5's case
    // too: nothing is guessed.
    let holding = match requests
        .ok()
        .map(|requests| where_it_came_from(&requests, &server_row, bytes, Some(&installation)))
    {
        Some(WhereItCameFrom::TheSourcePath(source)) => {
            HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(&source))
        }
        _ => HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
            machine: MachineKey::of(server_row.clone(), installation.clone()),
            location: MachineLocation::composed(bytes.to_vec()),
        },
    };
    let machine = MachineKey::of(server_row, installation);
    let bytes = bytes.to_vec();
    let the_files_bytes = bytes.clone();

    // **From here the check is reading the file, and the panel says so** —
    // before the Blender is resolved and before the machine answers its first
    // question, so the panel asked for next never draws *nothing was read*
    // over a check that is starting. The Creator's note 8 on the test copy.
    let key = keyed(server, file);
    let this_press_marked_it = the_run_holds()
        .lock()
        .map(|mut held| the_file_is_about_to_be_read(&mut held, &key))
        .unwrap_or(false);
    // **A check that ends before its sitting starts takes the mark off**;
    // once it starts, `crate::checking_a_file` owns the mark.
    let no_longer_read = move || {
        if this_press_marked_it {
            if let Ok(mut held) = the_run_holds().lock() {
                held.files_being_read.remove(&key);
            }
        }
    };

    // **P5: the sitting runs off the drawing thread**, and the page is told
    // each time it has something new to draw. **The Blender is resolved there
    // too**, since resolving it may ask the machine for its list.
    let app = app.clone();
    let (server, file) = (server.to_string(), file.to_string());
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().ok();
        // **`Dependency Upload & Remap` G2: the Blender the job will render
        // with**, and where there is none, why — said the way every answer
        // an act cannot give is said.
        let with = match with_now {
            Some(with) => Ok(with),
            None => the_blender_the_check_reads_with(
                crate::surface::render_settings::the_run_holds(),
                &server,
                &file,
                || {
                    let asked = match &runtime {
                        Some(runtime) => runtime.block_on(crate::submitting::the_blenders_on_the_machine(
                            &point,
                            &server,
                            own_area.as_deref(),
                        )),
                        None => smoozy_render_settings::acting::versions::TheVersionListAsk::TheMachineCouldNotBeAsked,
                    };
                    // **`Server Setup` R1b's comparison, at `Server Setup`
                    // B12's moment of `Render Settings` asking for the version
                    // list**, which this check asks on that page's behalf.
                    // Added 2026-10-12.
                    crate::servers::the_driver_swap::the_search_is_compared(&app, &server, &asked);
                    asked
                },
                || {
                    runtime.as_ref().and_then(|runtime| {
                        runtime.block_on(crate::surface::files::the_saved_with_version_of(
                            &point,
                            &server,
                            &the_files_bytes,
                            true,
                        ))
                    })
                },
            ),
        };
        let with = match with {
            Ok(with) => with,
            Err(NoBlenderToReadWith::Said(said)) => {
                no_longer_read();
                the_queue_is_told(&queue, an_act_said(&server, &file, Some(Prose::new(said))));
                return;
            }
            Err(NoBlenderToReadWith::TheServerCouldNotBeAsked) => {
                the_ending_lands(&app, &server, &file, Ending::TheMachineCouldNotBeReached, None, false, name.as_deref());
                no_longer_read();
                the_queue_is_told(&queue, an_act_said(&server, &file, None));
                return;
            }
        };
        let sitting = TheSittingIsOf {
            server: server.clone(),
            file: file.clone(),
            root: MachineLocation::composed(bytes.clone()),
            holding,
            with: with.clone(),
            machine,
            layout,
        };
        let told = {
            let (queue, server, file) = (Arc::clone(&queue), server.clone(), file.clone());
            move || the_queue_is_told(&queue, an_act_said(&server, &file, None))
        };
        let door = TheApplicationsRecord { app: app.clone() };
        let came_to = match &runtime {
            Some(runtime) => runtime.block_on(crate::checking_a_file::the_check_runs(
                &mut crate::reading_the_file::ThisServersMachine {
                    sessions: &point,
                    package_area: &package_area,
                    server: &server,
                },
                &door,
                &TheUsersOwnComputer,
                the_run_holds(),
                &sitting,
                &told,
            )),
            // **No sitting ran, so its mark is taken off here**; a sitting
            // that ran takes its own off, and one that joined a running check
            // leaves that check's mark alone.
            None => {
                no_longer_read();
                crate::checking_a_file::TheCheckCameTo::ItGaveNothing(Ending::TheMachineCouldNotBeReached)
            }
        };
        // **`§ When the file could not be read at all`, 2026-11-04: where the server
        // took no reading, the Set up tab's own reading says whether this server's
        // setup stands stopped.** A Blender that ran says nothing about a setup.
        let the_setup_stands_stopped = matches!(
            came_to,
            crate::checking_a_file::TheCheckCameTo::ItGaveNothing(Ending::NotAReadingAndNoneOfTheThreeEndingsFits)
        ) && crate::setting_up::the_setup_on(&app, &point, &server)
            .is_some_and(|read| crate::surface::set_up::a_stopped_setup_stands(&read));
        let said = the_sitting_came_to(&app, &server, &file, came_to, &with, name.as_deref(), the_setup_stands_stopped);
        the_queue_is_told(&queue, an_act_said(&server, &file, said));
    });
    None
}

/// **Why no check runs, where no Blender resolves for this file.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoBlenderToReadWith {
    /// Said at once, in the render settings panel's own words.
    Said(&'static str),
    /// **The server could not be asked for its Blenders**, so the check ends
    /// the way a check that could not reach the server ends (`Render Settings`
    /// Q8's first ending).
    TheServerCouldNotBeAsked,
}

/// **`Dependency Upload & Remap` G2: the Blender the job will render with**,
/// resolved by `Render Settings` B2a whether or not that page is open.
///
/// **Where no page holds the server's list, the check asks for it** —
/// `Render Settings` B2a's own *"a fresh … list … the same list ask"*,
/// through the one ask the page uses — and the list it gets is
/// landed where the page keeps its own (`Server Setup` B12: period-held), so
/// the two cannot resolve differently. *(Until step 10c's twelfth-pass fix
/// piece 6 a check started from Files on a server whose render settings had
/// not been opened resolved over no list, and said* This server has no
/// Blender. *on a server holding two.)*
///
/// **Where no page holds the file's saved-with version, the check reads it** —
/// `Render Settings` B2's one source, `File Browser` L7's reading of the file's
/// opening bytes, the same reading the page takes — and lands it where the
/// page keeps its own. *(Until step 10c's twelfth-pass fix piece 6b nothing in
/// the application took that reading, so a check with no Blender picked said*
/// This file does not say which Blender saved it *of every file.)*
///
/// **Where none resolves, the reason is named in the render settings panel's
/// own words** (`Render Settings` B5; `Render Settings` B5a; met at the check): *This server
/// has no Blender.* only where the list is empty.
pub fn the_blender_the_check_reads_with(
    page: &Mutex<crate::surface::render_settings::WhatTheRunHolds>,
    server: &str,
    file: &str,
    ask_for_the_list: impl FnOnce() -> smoozy_render_settings::acting::versions::TheVersionListAsk,
    read_the_saved_with_version: impl FnOnce() -> Option<String>,
) -> Result<BlenderIdentity, NoBlenderToReadWith> {
    use smoozy_render_settings::version::{Resolution, WhyNone};
    let resolved = || match page.lock() {
        Ok(page) => Ok(crate::surface::render_settings::the_version_resolved(&page, server, file)),
        Err(_) => Err(NoBlenderToReadWith::Said(words::THE_RECORD_WOULD_NOT_READ)),
    };
    let mut resolution = resolved()?;
    if resolution == Resolution::NoVersionResolves(WhyNone::NoMachineInHand) {
        // **The ask runs with no lock held**: it reads the machine.
        let asked = ask_for_the_list();
        match page.lock() {
            Ok(mut page) => crate::surface::render_settings::the_version_list_came_back(&mut page, server, asked),
            Err(_) => return Err(NoBlenderToReadWith::Said(words::THE_RECORD_WOULD_NOT_READ)),
        }
        resolution = resolved()?;
    }
    if resolution == Resolution::NoVersionResolves(WhyNone::NoSavedWithVersionToMatchOn) {
        // **`Render Settings` B2's one source, read where no page handed it**
        // — `File Browser` L7's reading, taken with no lock held, and landed
        // where the page keeps its own so the two cannot resolve differently.
        if let Some(version) = read_the_saved_with_version() {
            match page.lock() {
                Ok(mut page) => crate::surface::render_settings::the_saved_with_version_lands(
                    &mut page, server, file, &version,
                ),
                Err(_) => return Err(NoBlenderToReadWith::Said(words::THE_RECORD_WOULD_NOT_READ)),
            }
            resolution = resolved()?;
        }
    }
    the_blender_resolved(&resolution)
}

/// **The resolution, read as the Blender to read with or the reason there is
/// none.**
fn the_blender_resolved(
    resolution: &smoozy_render_settings::version::Resolution,
) -> Result<BlenderIdentity, NoBlenderToReadWith> {
    use smoozy_render_settings::version::WhyNone;
    match smoozy_dependencies::check::the_version_to_read_with(resolution) {
        Ok(with) => Ok(with),
        Err(smoozy_dependencies::check::NoCheckRuns::NoVersionResolves(why)) => Err(match why {
            WhyNone::TheListIsEmpty => NoBlenderToReadWith::Said(words::NO_BLENDER_ON_THIS_SERVER),
            WhyNone::NoSavedWithVersionToMatchOn => NoBlenderToReadWith::Said(words::NOTHING_TO_MATCH_ON),
            WhyNone::NothingButBuildsBelowTheFloor => {
                NoBlenderToReadWith::Said(words::SMOOZY_DOES_NOT_CHOOSE_BELOW_THE_FLOOR)
            }
            // **No list, after the ask: the machine could not be asked.**
            WhyNone::NoMachineInHand => NoBlenderToReadWith::TheServerCouldNotBeAsked,
        }),
    }
}

/// **Where a sitting's ending lands, and what the panel is told.**
fn the_sitting_came_to<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    file: &str,
    came_to: crate::checking_a_file::TheCheckCameTo,
    with: &BlenderIdentity,
    name: Option<&str>,
    the_setup_stands_stopped: bool,
) -> Option<Prose> {
    use crate::checking_a_file::TheCheckCameTo;
    match came_to {
        TheCheckCameTo::TheSittingStands(_) | TheCheckCameTo::ItJoinedTheOneRunning => None,
        TheCheckCameTo::ItGaveNothing(ending) => {
            the_ending_lands(app, server, file, ending, Some(with.clone()), the_setup_stands_stopped, name);
            None
        }
        TheCheckCameTo::TheRecordRefusedAWrite => Some(Prose::new(words::THE_CHANGE_WAS_NOT_SAVED)),
        TheCheckCameTo::TheRecordWouldNotRead => Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ)),
    }
}

/// **An ending, landed in the one place endings land** — the application's
/// lock before the run's.
fn the_ending_lands<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    file: &str,
    ending: Ending,
    read_with: Option<BlenderIdentity>,
    the_setup_stands_stopped: bool,
    name: Option<&str>,
) {
    use tauri::Manager;
    let started = app.try_state::<Mutex<crate::start::Started>>();
    let started = started.as_ref().and_then(|started| started.lock().ok());
    let Ok(mut held) = the_run_holds().lock() else {
        return;
    };
    let _ = an_ending_is_held(
        &mut held,
        server,
        file,
        ending,
        read_with,
        the_setup_stands_stopped,
        &crate::surface::render_settings::TheDiaryInHand {
            diary: started.as_ref().and_then(|started| started.diary.as_ref()),
            run: started.as_ref().and_then(|started| started.run.as_ref()),
            the_servers_name: name.unwrap_or_default(),
        },
    );
}

/// **What an act on this panel is.** H34: a closed request type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TheAct {
    CheckAgain,
    // **The same act as *Check again*, under the check's own name**, where no
    // check of the file is held — the Creator's thirteenth notes, note 2.
    CheckDependencies,
    ClearTheEnding,
    ClearTheStatement,
    ClearARowsFailure,
    // **Choosing rows the way a file manager does** — `§ Acting on many
    // rows`, the dependency check's fresh look, decision 2: a click chooses
    // that row alone, Shift-click the range from the last one clicked, and
    // Cmd-click (Ctrl-click on Windows) adds a row or takes it away. Until
    // then each row had a checkbox (`select-a-row`, `select-every-row`).
    ChooseARow,
    ChooseARange,
    AddOrTakeARow,
    StopTheSearch,
    StopTheOverwrite,
    // A decision about the user's own computer — `Dependency Upload & Remap` K4.
    Locate,
    SetAside,
    Restore,
    UndoTheGuess,
    Accept,
    UseIt,
    ChooseAFolder,
    UndoTheShift,
    ChooseASearchFolder,
    AddASearchFolder,
    // A searched folder's name on the folders line, opened in the computer's
    // own file manager — the Creator's note 5 on the test copy. `row` is its
    // place in the line.
    OpenASearchedFolder,
    // An act that needs the server.
    TryAgain,
    Confirm,
    Upload,
    AdminAccess,
    Replace,
}

/// **An act on this panel.** The answer comes back as an event and the page asks
/// again (P6).
///
/// **`async`, because three acts put the system's own picker up first** — see
/// [`crate::picker::asked`].
#[tauri::command]
pub async fn an_act_on_the_check<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    server: String,
    file: Vec<u8>,
    act: TheAct,
    row: Option<u32>,
) {
    use tauri::Manager;
    let bytes = file;
    let file = crate::surface::render_settings::the_path_as_it_is_keyed(&bytes);
    let row = row.map(|at| at as usize);
    let said = the_act_is_answered(&app, &server, &file, &bytes, act, row).await;
    if let Some(queue) = app.try_state::<Arc<TheOneQueue>>() {
        the_queue_is_told(&queue, an_act_said(&server, &file, said));
    }
}

/// **A decision, taken with the application's lock before the run's**, over
/// the connection that lock holds.
fn a_decision_is_saved<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    file: &str,
    decision: crate::checking_a_file::ADecision,
    row: Option<usize>,
) -> Option<Prose> {
    use crate::checking_a_file::{a_decision_is_taken, OverOneConnection, TheUsersOwnComputer, WhatADecisionCameTo};
    use tauri::Manager;
    let started = app.try_state::<Mutex<crate::start::Started>>()?;
    let Ok(started) = started.lock() else {
        return Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ));
    };
    let Some(record) = started.record.as_ref() else {
        return Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ));
    };
    let Ok(mut held) = the_run_holds().lock() else {
        return Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ));
    };
    match a_decision_is_taken(
        &mut held,
        &keyed(server, file),
        decision,
        row,
        &OverOneConnection(record.connection()),
        &TheUsersOwnComputer,
    ) {
        WhatADecisionCameTo::Taken | WhatADecisionCameTo::NothingToTake => None,
        WhatADecisionCameTo::NotSaved => Some(Prose::new(words::THE_CHANGE_WAS_NOT_SAVED)),
    }
}

/// **The door a send writes through**: the application's own record, one short
/// act at a time, and **the panel and Transfers told** as its files move —
/// `Desktop app and delivery` P7 and P8.
struct TheSendsDoor<R: tauri::Runtime> {
    app: tauri::AppHandle<R>,
    server: String,
    file: String,
}

impl<R: tauri::Runtime> crate::uploading::TheRecordAnUploadWrites for TheSendsDoor<R> {
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
        crate::uploading::TheRecordAnUploadWrites::with_the_record(&TheApplicationsRecord { app: self.app.clone() }, act)
    }

    /// The run's own reading, for the overwrite box's live-job line (`File
    /// Browser` W5a). Step 10c's Files' owed acts piece 1.
    fn the_last_queue_reading(
        &self,
        server: &str,
    ) -> Option<smoozy_queue_history::acting::adoption::TheMachinesWorkingState> {
        crate::surface::files::the_last_queue_reading(&self.app, server)
    }

    fn a_transfer_moved(&self) {
        use tauri::Manager;
        crate::surface::transfers::the_transfers_moved(&self.app);
        if let Some(queue) = self.app.try_state::<Arc<TheOneQueue>>() {
            the_queue_is_told(&queue, an_act_said(&self.server, &self.file, None));
        }
    }

    /// **Bytes moved inside a file: Transfers is told and this panel is
    /// not**, since nothing it draws moves with them.
    fn a_transfer_is_moving(&self) {
        crate::surface::transfers::the_transfers_moved(&self.app);
    }
}

/// **The acts that need the server** — *Upload*, a row's *Try again*, the
/// overwrite box's *Replace* and a row's *Confirm* — taken through
/// `crate::the_found_files_sent` off the drawing thread (P5). Step 10c's
/// dependency wave piece 6.
///
/// **What this answers at once** is only what stops the act before anything is
/// asked of the machine: no sitting to send from, no live connection, no
/// placement recorded.
fn a_send_is_asked_for<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    file: &str,
    act: TheAct,
    row: Option<usize>,
) -> Option<Prose> {
    use crate::the_found_files_sent::{a_row_is_confirmed, the_found_files_go_up, the_rows_the_upload_sends, WhatIsSent};
    let key = keyed(server, file);
    let (of, rows) = {
        let Ok(mut held) = the_run_holds().lock() else {
            return Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ));
        };
        let replacing = match act {
            TheAct::Replace => held.overwrites.remove(&key).map(|asked| asked.row),
            _ => None,
        };
        let sitting = held.sittings.get(&key)?;
        let of = sitting.of.clone()?;
        let rows: Vec<usize> = match act {
            TheAct::Upload => the_rows_the_upload_sends(&sitting.rows),
            TheAct::Replace => replacing.into_iter().collect(),
            _ => row.into_iter().filter(|at| *at < sitting.rows.len()).collect(),
        };
        (of, rows)
    };
    if rows.is_empty() {
        return None;
    }
    if of.layout.is_none() {
        return Some(Prose::new(words::NOT_SET_UP_ON_THIS_SERVER));
    }
    let point = app
        .try_state::<Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_sessions_of(server))
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live);
    let Some(point) = point else {
        return Some(Prose::new(words::COULD_NOT_REACH_THE_SERVER));
    };
    let the_run = app
        .try_state::<Arc<crate::downloading::TheRunsTransfers>>()
        .map(|the_run| Arc::clone(the_run.inner()))?;
    let run = app
        .try_state::<Mutex<crate::start::Started>>()
        .and_then(|started| started.lock().ok().and_then(|started| started.run.clone()))?;
    let door: Arc<dyn crate::uploading::TheRecordAnUploadWrites> = Arc::new(TheSendsDoor {
        app: app.clone(),
        server: server.to_string(),
        file: file.to_string(),
    });
    let asked = WhatIsSent {
        of,
        run,
        rows,
        replacing: act == TheAct::Replace,
    };
    let app = app.clone();
    // **`Dependency Upload & Remap` A9e: from the press, *Render* is held.**
    let in_hand = (act != TheAct::Confirm).then(|| TheSendInHand::of(&key));
    tauri::async_runtime::spawn(async move {
        if act == TheAct::Confirm {
            if let Some(at) = asked.rows.first() {
                a_row_is_confirmed(&point, door.as_ref(), the_run_holds(), &asked.of, *at).await;
            }
        } else {
            let came_to =
                the_found_files_go_up(&asked, &point, &the_run, Arc::clone(&door), the_run_holds()).await;
            // **`Dependency Upload & Remap` G14: an upload that ends with files
            // arrived runs the check again**, the same act as *Check again* —
            // see [`the_check_runs_again_after`] for when.
            the_check_runs_again_after(&came_to, || {
                // **`Dependency Upload & Remap` G14a: a check running goes round
                // once more**, and the press below joins it.
                crate::checking_a_file::a_send_ended(the_run_holds(), &keyed(&asked.of.server, &asked.of.file));
                let bytes = asked.of.root.as_bytes().to_vec();
                if let Some(said) = the_check_is_begun(&app, &asked.of.server, &asked.of.file, &bytes) {
                    if let Some(queue) = app.try_state::<Arc<TheOneQueue>>() {
                        the_queue_is_told(&queue, an_act_said(&asked.of.server, &asked.of.file, Some(said)));
                    }
                }
            });
        }
        drop(in_hand);
        door.a_transfer_moved();
    });
    None
}

/// **When an upload the check started runs the check again by itself, and
/// it does so once** — `Specs/Dependency Upload & Remap` *Checking a file*'s
/// one exception and `Dependency Upload & Remap` G14, both widened
/// 2026-10-17 (the Creator's tenth notes, note 4, Creator-stated): *"when an
/// upload the check started ends with files arrived, the check runs again by
/// itself"*. **It is the same act as *Check again*** — the caller hands
/// [`the_check_is_begun`] in — so a linked `.blend` the upload put on the
/// machine is read by it, its own reading, as `Dependency Upload & Remap`
/// G14 has always asked; until
/// that day the check ran again only where a linked `.blend` had gone up (the
/// 38's piece 7).
///
/// - **At least one file arrived**: published at its mirror. Rows found
///   already on the server, or tried again on their own
///   (`Dependency Upload & Remap` M16a), are not an
///   arrival.
/// - **Nothing was stopped or cut**: an upload the user stopped, or one a lost
///   connection cut, starts none, since nothing resumes on its own (the spec's
///   own line, and `Design/Dependency Upload & Remap`'s *nothing searches by
///   itself*). A file that did not finish is either.
/// - **No row is waiting on the overwrite box.** *Replace or stop* is a
///   question the upload is still asking, so it has not ended; and a check
///   run under the box would rebuild the rows the box names one of by its
///   place. The send *Replace* starts ends on its own and is judged here
///   again. The builder's reading, 2026-10-17: the plans do not say.
pub fn the_check_runs_again_after(
    came_to: &crate::the_found_files_sent::WhatTheSendCameTo,
    check_again: impl FnOnce(),
) {
    if came_to.published > 0 && came_to.did_not_finish == 0 && came_to.asks_to_replace.is_none() {
        check_again();
    }
}

/// **The system's own picker, for one file or one folder.** Nothing where the
/// user closed it, which is an ordinary answer.
async fn one_picked<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    what: crate::picker::WhatMayBeChosen,
    title: &str,
) -> Option<Vec<u8>> {
    use crate::picker::{asked, Asking, HowMany, Picked};
    let Picked::TheUserChose(chosen) = asked(
        app,
        Asking {
            what,
            how_many: HowMany::One,
            starting_at: None,
            title: Prose::new(title),
        },
    )
    .await
    else {
        return None;
    };
    chosen
        .first()
        .map(|one| one.as_the_user_gave_it().as_os_str().as_encoded_bytes().to_vec())
}

/// **`Dependency Upload & Remap` F1c: *Locate…* and *Choose a folder…*, off the
/// drawing thread, with no lock held while they look at the disk.**
async fn a_decision_looks_at_the_disk<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    file: &str,
    decision: crate::checking_a_file::ADecision,
    row: Option<usize>,
) -> Option<Prose> {
    use crate::checking_a_file::{a_decision_that_looks_is_taken, TheUsersOwnComputer, WhatADecisionCameTo};
    let (app, key) = (app.clone(), keyed(server, file));
    tauri::async_runtime::spawn_blocking(move || {
        match a_decision_that_looks_is_taken(the_run_holds(), &key, &decision, row, &TheApplicationsRecord { app }, &TheUsersOwnComputer) {
            WhatADecisionCameTo::Taken | WhatADecisionCameTo::NothingToTake => None,
            WhatADecisionCameTo::NotSaved => Some(Prose::new(words::THE_CHANGE_WAS_NOT_SAVED)),
        }
    })
    .await
    .ok()
    .flatten()
}

/// **What each act does.** The acts that stay in the run's memory are
/// [`the_act_is_taken`]'s; every decision about the user's own computer is
/// saved through `crate::checking_a_file`; checking again starts a sitting.
async fn the_act_is_answered<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    file: &str,
    bytes: &[u8],
    act: TheAct,
    row: Option<usize>,
) -> Option<Prose> {
    use crate::checking_a_file::ADecision;
    use crate::picker::WhatMayBeChosen;
    // **What *Add* said is said once**: the next act lets it go.
    if let Ok(mut held) = the_run_holds().lock() {
        the_added_statement_is_let_go(&mut held, server, file, act);
    }
    let decision = match act {
        TheAct::CheckAgain | TheAct::CheckDependencies => return the_check_is_begun(app, server, file, bytes),
        TheAct::OpenASearchedFolder => {
            let folder = match the_run_holds().lock() {
                Ok(held) => the_searched_folder_at(&held, server, file, row),
                Err(_) => return Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ)),
            };
            return a_searched_folder_is_opened(folder.as_deref(), crate::reveal::a_folder);
        }
        // **The acts that need the server** — step 10c's dependency wave
        // piece 6.
        TheAct::Upload | TheAct::TryAgain | TheAct::Replace | TheAct::Confirm => {
            return a_send_is_asked_for(app, server, file, act, row)
        }
        TheAct::SetAside => ADecision::SetAside,
        TheAct::Restore => ADecision::Restore,
        TheAct::UndoTheGuess => ADecision::UndoTheGuess,
        TheAct::Accept => ADecision::Accept,
        TheAct::UseIt => ADecision::UseIt,
        TheAct::UndoTheShift => ADecision::UndoTheShift,
        // **`Dependency Upload & Remap` F1b: off the drawing thread, and no lock
        // held while it searches.**
        TheAct::AddASearchFolder => {
            let (app, server, file) = (app.clone(), server.to_string(), file.to_string());
            return tauri::async_runtime::spawn_blocking(move || {
                use crate::checking_a_file::{the_chosen_folder_is_added, TheUsersOwnComputer, WhatADecisionCameTo};
                use tauri::Manager;
                let door = TheApplicationsRecord { app: app.clone() };
                let told = || {
                    if let Some(queue) = app.try_state::<Arc<TheOneQueue>>() {
                        the_queue_is_told(&queue, an_act_said(&server, &file, None));
                    }
                };
                match the_chosen_folder_is_added(
                    the_run_holds(),
                    &keyed(&server, &file),
                    &door,
                    &TheUsersOwnComputer,
                    &told,
                ) {
                    WhatADecisionCameTo::Taken | WhatADecisionCameTo::NothingToTake => None,
                    WhatADecisionCameTo::NotSaved => Some(Prose::new(words::THE_CHANGE_WAS_NOT_SAVED)),
                }
            })
            .await
            .ok()
            .flatten();
        }
        // **`Dependency Upload & Remap` F28 and `Dependency Upload & Remap` G23: a set row, or a
        // folder-shaped one, is pointed at a folder**; everything else at a
        // file.
        TheAct::Locate => {
            let what = match the_run_holds().lock() {
                Ok(held) => match held
                    .sittings
                    .get(&keyed(server, file))
                    .and_then(|sitting| row.and_then(|at| sitting.rows.get(at)))
                    .and_then(|one| one.wanted.first())
                    .map(|wanted| wanted.shape)
                {
                    Some(smoozy_dependencies::search::WantedShape::AFile) => WhatMayBeChosen::Files,
                    Some(_) => WhatMayBeChosen::Folders,
                    None => return None,
                },
                Err(_) => return Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ)),
            };
            let path = one_picked(app, what, words::LOCATE).await?;
            return a_decision_looks_at_the_disk(app, server, file, ADecision::Locate(path), row).await;
        }
        TheAct::ChooseAFolder => {
            let folder = one_picked(app, WhatMayBeChosen::Folders, words::CHOOSE_A_FOLDER).await?;
            return a_decision_looks_at_the_disk(app, server, file, ADecision::ChooseAFolder(folder), row).await;
        }
        // **The folder is held until Add**, and the box shows it.
        TheAct::ChooseASearchFolder => {
            let chosen = one_picked(app, WhatMayBeChosen::Folders, words::ADD_A_SEARCH_FOLDER).await?;
            if let Ok(mut held) = the_run_holds().lock() {
                if let Some(sitting) = held.sittings.get_mut(&keyed(server, file)) {
                    sitting.the_chosen_search_folder = Some(chosen);
                }
            }
            return None;
        }
        _ => {
            return match the_run_holds().lock() {
                Ok(mut held) => the_act_is_taken(&mut held, server, file, act, row),
                Err(_) => Some(Prose::new(words::THE_RECORD_WOULD_NOT_READ)),
            }
        }
    };
    a_decision_is_saved(app, server, file, decision, row)
}

// ════════════════════════════════════════════════════════════════════════════
// The acts, with the state handed in so the whole of each can be tested
// ════════════════════════════════════════════════════════════════════════════

/// **Where a real check's ending lands, and the one place it is handed to the
/// diary** — `Dependency Upload & Remap` §11's declaration, through `Desktop app
/// and delivery` §16.8 M32's second limb (the upstream must-do of the step 10a round 1
/// audit of these pieces).
///
/// **No record is raised** — `Render Settings` Q9, as this panel applies it: a
/// read that gave nothing changes nothing — so the ending is held beside the
/// panel. **What the user met still reaches the diary, once for this request**:
/// the sentence this panel composes for the ending, before the typography point,
/// with the server row, the user's label for it and the file's recorded path.
/// **It is called where the request's ending first lands and never on a
/// redraw.** A reading is not an ending and hands nothing over.
pub fn the_check_gave_nothing(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    ending: Ending,
    read_with: Option<BlenderIdentity>,
    the_diary: &crate::surface::render_settings::TheDiaryInHand<'_>,
) -> crate::presentation::the_two_hooks::TheDiaryWasHanded {
    an_ending_is_held(held, server, file, ending, read_with, false, the_diary)
}

/// **The same, knowing whether this server's setup stands stopped** —
/// [`AnEnding::the_setup_stands_stopped`], whose sentence the diary is handed.
fn an_ending_is_held(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    ending: Ending,
    read_with: Option<BlenderIdentity>,
    the_setup_stands_stopped: bool,
    the_diary: &crate::surface::render_settings::TheDiaryInHand<'_>,
) -> crate::presentation::the_two_hooks::TheDiaryWasHanded {
    use crate::presentation::the_two_hooks::{
        the_diary_is_handed_a_sentence_with_no_record, APresentationWithNoRecord, TheDiaryWasHanded,
        WhoseSentenceItIs,
    };
    let sentence = if the_setup_stands_stopped {
        Some(words::SETUP_STOPPED)
    } else {
        the_endings_sentence(&ending)
    };
    held.endings.insert(keyed(server, file), AnEnding { ending, read_with, the_setup_stands_stopped });
    let Some(sentence) = sentence else {
        return TheDiaryWasHanded::Nothing {
            reported: "a reading is not an ending".to_string(),
        };
    };
    let server_row = an_identity(server);
    the_diary_is_handed_a_sentence_with_no_record(
        the_diary.diary,
        the_diary.run,
        &APresentationWithNoRecord {
            server_row: &server_row,
            the_users_label: the_diary.the_servers_name,
            the_files_recorded_path: file,
        },
        WhoseSentenceItIs::TheComposersOwn(sentence),
    )
}

/// **The acts that stay in the run's memory, and the acts that need the
/// server.** A decision about the user's own computer is not taken here — it
/// is saved through `crate::checking_a_file` by [`an_act_on_the_check`] — and
/// neither is checking again, which starts a sitting.
pub fn the_act_is_taken(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    act: TheAct,
    row: Option<usize>,
) -> Option<Prose> {
    let key = keyed(server, file);
    match act {
        TheAct::ClearTheEnding => {
            held.endings.remove(&key);
            None
        }
        TheAct::ClearTheStatement => {
            if let Some(once) = held.sittings.get_mut(&key).and_then(|s| s.the_once.as_mut()) {
                once.cleared = true;
            }
            None
        }
        TheAct::ClearARowsFailure => {
            if let (Some(sitting), Some(at)) = (held.sittings.get_mut(&key), row) {
                if let Some(one) = sitting.rows.get_mut(at) {
                    one.its_failure_was_cleared = true;
                }
            }
            None
        }
        // **`Dependency Upload & Remap` F22: a selection is run-held**, so
        // choosing rows works whole. **Only a row in the list a selection is
        // made from is chosen** — an ignored row is not (`§ Acting on many
        // rows`).
        TheAct::ChooseARow | TheAct::ChooseARange | TheAct::AddOrTakeARow => {
            let (Some(at), Some(sitting)) = (row, held.sittings.get(&key)) else {
                return None;
            };
            let order = the_order_a_selection_runs_in(sitting);
            let Some(here) = order.iter().position(|one| *one == at) else {
                return None;
            };
            let chosen = held.selections.entry(key.clone()).or_default();
            let anchor = held
                .anchors
                .get(&key)
                .and_then(|anchor| order.iter().position(|one| one == anchor))
                .filter(|_| !chosen.is_empty());
            match (act, anchor) {
                (TheAct::ChooseARange, Some(from)) => {
                    let (low, high) = (from.min(here), from.max(here));
                    *chosen = order[low..=high].iter().copied().collect();
                    return None;
                }
                (TheAct::AddOrTakeARow, _) => {
                    if !chosen.remove(&at) {
                        chosen.insert(at);
                    }
                }
                _ => {
                    chosen.clear();
                    chosen.insert(at);
                }
            }
            held.anchors.insert(key, at);
            None
        }
        // **`Dependency Upload & Remap` F6 and F7: a stopped search keeps what
        // it has**, and a folder the stopped walk had not finished was stopped
        // part-way. **`Dependency Upload & Remap` G15: the sitting running is
        // told to stop**, which it reads between readings and inside the walk;
        // a reading already on the machine is not ended (see
        // `crate::checking_a_file`'s head).
        TheAct::StopTheSearch => {
            if let Some(stop) = held.checks_running.get_mut(&key) {
                *stop = true;
            }
            if let Some(search) = held.sittings.get_mut(&key).and_then(|s| s.the_search.as_mut()) {
                search.running = false;
                for (_, verdict) in search.folders.iter_mut() {
                    if verdict.is_none() {
                        *verdict = Some(FolderVerdict::StoppedPartWay);
                    }
                }
            }
            None
        }
        TheAct::StopTheOverwrite => {
            held.overwrites.remove(&key);
            None
        }
        // **Saved through `crate::checking_a_file`**, never here.
        TheAct::CheckAgain
        | TheAct::CheckDependencies
        | TheAct::Locate
        | TheAct::SetAside
        | TheAct::Restore
        | TheAct::UndoTheGuess
        | TheAct::Accept
        | TheAct::UseIt
        | TheAct::ChooseAFolder
        | TheAct::UndoTheShift
        | TheAct::ChooseASearchFolder
        | TheAct::AddASearchFolder => None,
        // **Sent through `crate::the_found_files_sent`**, never here — step
        // 10c's dependency wave piece 6.
        TheAct::TryAgain | TheAct::Confirm | TheAct::Upload | TheAct::Replace => None,
        // **Opened by the existing reveal** in [`an_act_on_the_check`], never
        // here — the Creator's note 5 on the test copy.
        TheAct::OpenASearchedFolder => None,
        // **No admin-access offer is drawn any longer** (`File Browser` P6a,
        // and this panel's own once statement), so a page drawn before that
        // is the only way here, and it says so and no more — `Servers`' own
        // *not finished* words, as Transfers says them.
        TheAct::AdminAccess => Some(Prose::new(crate::surface::servers::words::NOT_BUILT_YET)),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The event, which is what an act answers with
// ════════════════════════════════════════════════════════════════════════════

fn the_queue_is_told(queue: &TheOneQueue, event: Event) {
    let _ = queue.read_then_post::<Event, std::convert::Infallible>(
        || Ok(event.clone()),
        |event| event.clone(),
    );
}

/// **What an act says.** The page asks for the panel again; this carries only
/// what the panel cannot — the answer, where the act could not happen.
pub fn an_act_said(server: &str, file: &str, said: Option<Prose>) -> Event {
    let event = Event::about(Subject::TheRun, AN_ACT_ON_THIS_PANEL)
        .saying("server", Said::Value(Verbatim::new(server.to_string())))
        .saying("file", Said::Value(Verbatim::new(file.to_string())));
    match said {
        Some(said) => event.saying("said", Said::Prose(said)),
        None => event,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Composing the panel
// ════════════════════════════════════════════════════════════════════════════

fn lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}

/// The last component of a path, which is what the header and the strip name.
fn the_last_part(bytes: &[u8]) -> String {
    let text = lossy(bytes);
    let trimmed = text.trim_end_matches('/');
    trimmed
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or(trimmed)
        .to_string()
}

/// **The size, in the industry's own units** — the same composition
/// `src/surface/files.rs` uses for a listed size, whose own function is private
/// to a file this piece may not edit. **Nothing decides anything from it.**
fn the_size(bytes: u64) -> String {
    use crate::typography::a_figure_with_its_unit;
    const UNITS: [&str; 5] = ["bytes", "KB", "MB", "GB", "TB"];
    if bytes < 1024 {
        return a_figure_with_its_unit(bytes, UNITS[0]);
    }
    let mut value = bytes as f64;
    let mut at = 0;
    while value >= 1024.0 && at + 1 < UNITS.len() {
        value /= 1024.0;
        at += 1;
    }
    if value < 10.0 {
        a_figure_with_its_unit(format!("{value:.1}"), UNITS[at])
    } else {
        a_figure_with_its_unit(value.round() as u64, UNITS[at])
    }
}

fn an_identity(server: &str) -> Identity {
    Identity::read_back(IdentityKind::LocalServer, server).unwrap_or_else(|| {
        Identity::read_back(IdentityKind::LocalServer, &"0".repeat(32))
            .expect("thirty-two zeroes are an identity's own rendering")
    })
}

/// **Whether a row's state is a failure of the upload rather than of the
/// search** — `§ The seven states`: *the last two*.
fn is_a_failure(state: RowState) -> bool {
    matches!(state, RowState::CouldNotBeSent | RowState::CouldNotBeFixed)
}

/// **One group as `§ How the rows are grouped and sorted` sets it out** — its
/// key, its screen name, its colour, and whether it is folded away.
struct AGroupsPlace {
    which: &'static str,
    heading: &'static str,
    colour: &'static str,
    folded: bool,
}

/// **`§ How the rows are grouped and sorted`, since the dependency check's
/// fresh look (2026-10-18): Failed, Missing, Uploaded before not verified, To
/// upload, Already on the server (folded, its size on the heading), Ignored
/// (folded).** Each with its colour: red to act, amber to look, green when
/// done, grey where the user has answered, and none where Smoozy uploads the
/// file itself. The keys are the ones the page and the tests have always used.
const THE_GROUPS: [AGroupsPlace; 6] = [
    AGroupsPlace { which: "failed", heading: words::FAILED, colour: "bad", folded: false },
    AGroupsPlace { which: "not-found", heading: words::MISSING, colour: "bad", folded: false },
    AGroupsPlace {
        which: "uploaded-before",
        heading: words::UPLOADED_BEFORE_NOT_VERIFIED,
        colour: "busy",
        folded: false,
    },
    AGroupsPlace { which: "found", heading: words::TO_UPLOAD_HEADING, colour: "", folded: false },
    AGroupsPlace { which: "already-there", heading: words::ALREADY_ON_THE_SERVER, colour: "ok", folded: true },
    AGroupsPlace { which: "ignored", heading: words::IGNORED, colour: "idle", folded: true },
];

/// The place in [`THE_GROUPS`] of the Ignored group, whose rows are not in
/// the list a selection is made from.
const THE_IGNORED_GROUP: usize = 5;

/// Which of [`THE_GROUPS`] a state sits in.
fn the_group_of(state: RowState) -> usize {
    match state {
        RowState::CouldNotBeSent | RowState::CouldNotBeFixed => 0,
        RowState::NotFound => 1,
        RowState::SentBeforeNotConfirmed => 2,
        RowState::FoundOnYourComputer => 3,
        RowState::AlreadyOnTheMachine => 4,
        RowState::SetAside => THE_IGNORED_GROUP,
    }
}

/// **The action that fits the state is `Dependency Upload & Remap`'s own**,
/// read through `RowState::the_action_that_fits`, and only the words are this
/// page's — *Verify* for the module's Confirm (`§ The seven states`, from
/// 2026-10-18).
fn the_action(state: RowState, the_connection_dropped: bool) -> Option<AControl> {
    let (label, act, needs_the_server) = match state.the_action_that_fits() {
        Action::Locate => (words::LOCATE, "locate", false),
        Action::Nothing => return None,
        Action::Confirm => (words::VERIFY, "confirm", true),
        Action::BringBack => (words::RESTORE, "restore", false),
        Action::TryAgainTheSend | Action::TryAgainTheFixing => (words::TRY_AGAIN, "try-again", true),
    };
    Some(a_control(label, act, needs_the_server && the_connection_dropped))
}

/// **The one action a row shows, where its state needs one** — `§ A row`:
/// *Locate…* on a missing row, *Accept* where only the capitals differ, *Use
/// it* where the user rejected Smoozy's guess, *Verify*, *Try again*,
/// *Restore*. **A row Smoozy will upload, or has on the server already, shows
/// none**: the module's *Locate…* on a found row sits behind its ⋯.
fn the_visible_action(one: &ACheckedRow, the_connection_dropped: bool) -> Option<AControl> {
    match one.state {
        RowState::FoundOnYourComputer | RowState::AlreadyOnTheMachine => None,
        RowState::NotFound => {
            if the_rejected_guess(one).is_some() {
                Some(a_control(words::USE_IT, "use-it", false))
            } else if one.proposal.is_some() && the_qualifiers(one).contains(&Qualifier::ACaseNearMissProposal) {
                Some(a_control(words::ACCEPT, "accept", false))
            } else {
                the_action(one.state, the_connection_dropped)
            }
        }
        _ => the_action(one.state, the_connection_dropped),
    }
}

/// The guess the user rejected on this row, where one stands (`Dependency
/// Upload & Remap` K4).
fn the_rejected_guess(one: &ACheckedRow) -> Option<&StoredLocation> {
    one.the_answer().and_then(|answer| answer.rejected_guess.as_ref())
}

/// **Whether Smoozy guessed this row's location** — the search or a shift
/// supplied it (`Dependency Upload & Remap` K4), which is what *Reject match*
/// undoes.
fn a_guess(one: &ACheckedRow) -> bool {
    matches!(
        one.the_answer()
            .and_then(|answer| answer.resolution.as_ref())
            .map(|resolution| &resolution.how),
        Some(HowResolved::TheSearch { .. }) | Some(HowResolved::AShift(_))
    )
}

fn a_control(label: &str, act: &str, unavailable: bool) -> AControl {
    AControl {
        label: Prose::new(label),
        act: Verbatim::new(act),
        unavailable,
    }
}

/// **What the panel is told beside what the run holds**, read by the command
/// before the run's lock is taken.
#[derive(Debug, Clone, Default)]
pub struct WhatElseIsKnown {
    /// **Settings' list of search folders as it stands now**, which the
    /// folders line is compared with (`Design/Dependency Upload & Remap § The
    /// search folders on this panel`). `None` where it would not read, and
    /// then nothing is said about it.
    pub the_list_in_settings: Option<Vec<Vec<u8>>>,
    /// **How far this check's uploads have got** — files uploaded, of files
    /// being sent — read from what Transfers lists
    /// (`crate::surface::transfers::the_checks_uploads_on`), the same reading
    /// Render Settings counts by Render. `None` where nothing is going.
    pub uploading: Option<(usize, usize)>,
    /// **The landing place of every file a send of this run is carrying to
    /// this server, running or waiting** — `Dependency Upload & Remap` A9a's
    /// per-file reading (`crate::the_found_files_sent::the_files_on_their_way_to`).
    /// A row any of whose files is among them says *Uploading* and the foot
    /// leaves it out. Empty where nothing is going.
    pub on_its_way: Vec<MachineLocation>,
    /// **`Dependency Upload & Remap` A9c: a press of this check's upload control
    /// is still counting what goes.**
    pub a_send_is_being_counted: bool,
}

/// **The whole panel, composed from what the run holds and from nothing else.**
pub fn the_panel(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    the_servers_name: Option<&str>,
) -> TheCheck {
    the_panel_knowing(held, server, file, the_servers_name, &WhatElseIsKnown::default())
}

/// **The whole panel, composed from what the run holds and what the command
/// read beside it.**
pub fn the_panel_knowing(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    the_servers_name: Option<&str>,
    known: &WhatElseIsKnown,
) -> TheCheck {
    let key = keyed(server, file);
    let name = the_last_part(file.as_bytes());
    let mut check = TheCheck {
        server: Verbatim::new(server.to_string()),
        file: Verbatim::new(file.to_string()),
        file_bytes: file.as_bytes().to_vec(),
        title: Words::plain(words::DEPENDENCIES).and(name.clone()),
        show_in_files: crate::surface::render_settings::the_file_in_files(file.as_bytes()),
        instead: None,
        the_reading_line: Vec::new(),
        still_reading: None,
        the_summary: None,
        the_statements: Vec::new(),
        the_once: None,
        the_ending: None,
        the_walks_stop: None,
        nothing_was_read: None,
        the_check_to_start: None,
        the_tools: None,
        the_selection: None,
        the_shift: None,
        the_table: None,
        the_foot: None,
        the_overwrite: None,
        the_folder_box: the_folder_box(),
        the_machines_words_subject: Verbatim::new(
            crate::record::failure_records::DurableSubject::ServerRow(an_identity(server))
                .kind_as_text(),
        ),
        the_strip_title: AStripsWords {
            lead: Prose::new(words::DEPENDENCIES),
            values: vec![Verbatim::new(name)],
            rest: None,
            the_rest_joins: false,
        },
        the_strip_reading: Vec::new(),
    };

    if file.is_empty() {
        check.instead = Some(Prose::new(words::OPENED_WITHOUT_A_FILE));
        return check;
    }

    // **The box lists Settings' folders first, in Settings' order** — the
    // Creator's note 7 on the test copy. Read through the list's keeper by the
    // command (`Dependency Upload & Remap` F1), shown and never changed here.
    check.the_folder_box.the_folders = known
        .the_list_in_settings
        .iter()
        .flatten()
        .map(|folder| Verbatim::new(lossy(folder)))
        .collect();

    // **While the file itself is being read: *Reading the file…* on the
    // header's own line, with the underway mark the page puts on that line,
    // and no table, no tools and no foot until its reading lands** —
    // `Design/Dependency Upload & Remap § While it is still reading below a
    // linked file`, its closing paragraph. An empty table there is the empty
    // list `Dependency Upload & Remap` G3 forbids. **It stands over whatever
    // the run held before** — an earlier sitting's rows or an earlier ending —
    // because the fresh sitting rebuilds and carries none of it (`Dependency
    // Upload & Remap` G17). The Creator's note 8 on the test copy.
    if held.files_being_read.contains(&key) {
        check.still_reading = Some(Words::plain(words::READING_THE_FILE));
        // **`Dependency Upload & Remap` Q5a's walk over the file's own rows counts in its place**, with
        // *Stop* beside it (`§ While it is still reading below a linked file`).
        if let Some(walk) = held.sittings.get(&key).and_then(|sitting| sitting.the_walk) {
            check.still_reading = Some(the_walks_line(walk));
            check.the_walks_stop = Some(Prose::new(words::STOP));
        }
        return check;
    }

    // **`Dependency Upload & Remap` G3: a read that gave nothing is no rows and
    // never an empty list**, and it stands over any sitting there was.
    if let Some(ending) = held.endings.get(&key) {
        if let Some(read_with) = &ending.read_with {
            check.the_reading_line =
                vec![Words::with(words::READ_WITH_BLENDER, version_text(read_with.version))];
        }
        check.the_ending = if ending.the_setup_stands_stopped {
            Some(the_ending_of_a_stopped_setup())
        } else {
            the_ending(&ending.ending, the_servers_name, ending.read_with.as_ref())
        };
        check.nothing_was_read = Some(Prose::new(words::NOTHING_WAS_READ));
        return check;
    }

    // **No check of this file held — after a restart, or the strip pressed on
    // a file never checked since Smoozy opened: nothing was read, and under it
    // one *Check dependencies*, which starts the check.** No folders line and
    // no tools bar, since nothing was searched; *Check again* is only for a
    // check that ran (`Design/Dependency Upload & Remap § The search folders
    // on this panel`, the drawing's `chk-none`; the Creator's thirteenth
    // notes, note 2). Opening the panel starts nothing (`Dependency Upload &
    // Remap` G17; `Design/Transfers`, the way back). The map's hole 3 for the
    // Creator's note 8 on the test copy.
    let Some(sitting) = held.sittings.get(&key) else {
        check.nothing_was_read = Some(Prose::new(words::NOTHING_WAS_READ));
        check.the_check_to_start = Some(a_control(words::CHECK_DEPENDENCIES, "check-dependencies", false));
        return check;
    };

    let dropped = sitting.completion == Some(Completion::TheConnectionDropped);
    // **A total that reads as final on a check that is not finished is the lie
    // an empty list would tell, one number smaller** — `§ When the connection
    // drops mid-check` and `§ While it is still reading below a linked file`.
    let so_far = sitting.completion != Some(Completion::Completed);

    check.the_reading_line = vec![Words::with(words::READ_WITH_BLENDER, version_text(sitting.read_with.version))];
    // **The linked files read, where there were any** — the drawing's
    // `chk-clean`, a file that links none, reads *Read with Blender 5.2.2*
    // and nothing after it.
    if sitting.linked_files_read > 0 {
        check.the_reading_line.push(Words::after(
            sitting.linked_files_read.to_string(),
            if sitting.linked_files_read == 1 {
                words::LINKED_FILE
            } else {
                words::LINKED_FILES
            },
        ));
    }
    // **A dropped connection is reading nothing**, whatever the drawing leaves
    // standing in that state.
    if !dropped {
        check.still_reading = sitting.still_reading_below.as_ref().map(|library| {
            Words::with(words::STILL_READING_BELOW, the_last_part(library.as_bytes()))
                .then(words::CANNOT_SAY_HOW_FAR)
        });
        // **`Dependency Upload & Remap` Q5a's walk counts in its place**, with *Stop* beside it.
        if let Some(walk) = sitting.the_walk {
            check.still_reading = Some(the_walks_line(walk));
            check.the_walks_stop = Some(Prose::new(words::STOP));
        }
    }

    check.the_statements = the_statements(sitting, dropped, known.the_list_in_settings.as_deref());
    check.the_once = sitting
        .the_once
        .as_ref()
        .filter(|once| !once.cleared)
        .map(the_once);
    check.the_tools = Some(the_tools(sitting));
    check.the_folder_box.chosen = sitting
        .the_chosen_search_folder
        .as_ref()
        .map(|folder| Verbatim::new(lossy(folder)));
    let chosen = held.selections.get(&key).cloned().unwrap_or_default();
    check.the_selection = the_selection_bar(sitting, &chosen);
    check.the_shift = sitting.the_shift.as_ref().map(|shift| the_shift(sitting, shift));
    check.the_table = Some(the_table(sitting, &chosen, dropped, &known.on_its_way));
    check.the_foot = the_foot(sitting, dropped, so_far, &known.on_its_way, known.a_send_is_being_counted);
    if let Some(said) = held.what_a_send_said.get(&key) {
        check.the_statements.push(what_the_send_said(*said));
    }
    check.the_overwrite = held
        .overwrites
        .get(&key)
        .map(the_overwrite);

    // **The summary sentence, under the title and on the strip** — `§ Above
    // the rows` and `§ Where it sits`, since the dependency check's fresh
    // look: collapsed, the strip shows the same sentence.
    let stretches = the_summary_of(sitting, so_far);
    check.the_summary = (!stretches.is_empty()).then(|| TheSummary {
        parts: stretches.iter().map(AStretch::on_the_panel).collect(),
    });
    check.the_strip_reading = match known.uploading {
        // **While the check's uploads run, the strip says so in place of its
        // sentence** — files uploaded so far, of all it is sending, as
        // Transfers lists them. The Creator's note 6 on 0.1.8.
        Some((done, whole)) => vec![
            AStripsWords {
                lead: Prose::new(words::UPLOADING),
                values: vec![Verbatim::new(done.to_string())],
                rest: Some(Prose::new(words::OF)),
                the_rest_joins: false,
            },
            AStripsWords {
                lead: Prose::new(""),
                values: vec![Verbatim::new(whole.to_string())],
                rest: None,
                the_rest_joins: false,
            },
        ],
        None => on_the_strip(&stretches),
    };
    check
}

/// **`§ When the file could not be read at all`: the read ends four ways, and
/// they are the four `Render Settings` sets out.** Only the second carries a
/// second sentence, and the version named is Smoozy's own record of the build
/// it ran.
/// **The plain sentence each ending is said with**, before the typography
/// point — **one composition**, so what the panel shows and what the diary is
/// handed (M32's second limb) cannot differ.
fn the_endings_sentence(ending: &Ending) -> Option<&'static str> {
    match ending {
        Ending::AReading(_) => None,
        Ending::TheMachineCouldNotBeReached => Some(words::COULD_NOT_REACH_THE_SERVER),
        Ending::TheFileCouldNotBeOpenedByThisVersion => Some(words::THIS_BLENDER_COULD_NOT_OPEN),
        Ending::TheReadRanAndSomethingInItFailed | Ending::NotAReadingAndNoneOfTheThreeEndingsFits => {
            Some(words::THE_READ_DID_NOT_FINISH)
        }
        Ending::SmoozyStoppedIt => Some(words::THE_CHECK_WAS_STOPPED),
    }
}

/// ***Checking files on the server: 120 of 480***, the walk's own count.
fn the_walks_line((done, whole): (usize, usize)) -> Words {
    Words::plain(words::CHECKING_FILES_ON_THE_SERVER)
        .and(done.to_string())
        .then(words::OF)
        .and(whole.to_string())
}

/// **`§ When the file could not be read at all`, 2026-11-04.** No read ran, so
/// none of the four endings is true. *Set up again* is the page's: it opens the
/// Set up tab and asks for the ready action there, as that tab's own does.
fn the_ending_of_a_stopped_setup() -> TheFailure {
    TheFailure {
        which: Verbatim::new("setup-stopped"),
        sentence: Prose::new(words::SETUP_STOPPED),
        the_second_sentence: Some(Prose::new(words::FINISH_SETUP_THEN_CHECK_AGAIN)),
        values: Vec::new(),
        controls: vec![
            AShapesControl {
                is: Verbatim::new("retry"),
                label: Prose::new(words::SET_UP_AGAIN),
                act: Verbatim::new("set-up-again"),
            },
            AShapesControl {
                is: Verbatim::new("retry"),
                label: Prose::new(words::CHECK_AGAIN),
                act: Verbatim::new("check-again"),
            },
            AShapesControl {
                is: Verbatim::new("cross"),
                label: Prose::new(crate::presentation::the_failure_shape::CLEAR_THIS),
                act: Verbatim::new("clear-the-ending"),
            },
        ],
    }
}

fn the_ending(
    ending: &Ending,
    the_servers_name: Option<&str>,
    read_with: Option<&BlenderIdentity>,
) -> Option<TheFailure> {
    let check_again = AShapesControl {
        is: Verbatim::new("retry"),
        label: Prose::new(words::CHECK_AGAIN),
        act: Verbatim::new("check-again"),
    };
    let cross = AShapesControl {
        is: Verbatim::new("cross"),
        label: Prose::new(crate::presentation::the_failure_shape::CLEAR_THIS),
        act: Verbatim::new("clear-the-ending"),
    };
    let sentence = the_endings_sentence(ending)?;
    let (which, second, values, raw) = match ending {
        Ending::AReading(_) => return None,
        // **The value beside it is what the user calls the server**, never the
        // local identifier, which is a key (must-do 5); where the record would
        // not give the name, no value is drawn.
        Ending::TheMachineCouldNotBeReached => (
            "the-machine",
            None,
            the_servers_name
                .map(|name| AValue {
                    name: Prose::new(words::SERVER),
                    value: Verbatim::new(name.to_string()),
                })
                .into_iter()
                .collect(),
            false,
        ),
        Ending::TheFileCouldNotBeOpenedByThisVersion => (
            "the-version",
            Some(Prose::new(words::PICK_A_DIFFERENT_BLENDER)),
            read_with
                .map(|read_with| {
                    vec![AValue {
                        name: Prose::new(words::BLENDER),
                        value: Verbatim::new(version_text(read_with.version)),
                    }]
                })
                .unwrap_or_default(),
            false,
        ),
        Ending::TheReadRanAndSomethingInItFailed | Ending::NotAReadingAndNoneOfTheThreeEndingsFits => {
            ("the-read", None, Vec::new(), true)
        }
        Ending::SmoozyStoppedIt => ("smoozy-stopped-it", None, Vec::new(), false),
    };
    let mut controls = vec![check_again];
    if raw {
        controls.push(AShapesControl {
            is: Verbatim::new("raw-text"),
            label: Prose::new(words::BLENDER_OUTPUT),
            act: Verbatim::new("blender-output"),
        });
    }
    controls.push(cross);
    Some(TheFailure {
        which: Verbatim::new(which),
        sentence: Prose::new(sentence),
        the_second_sentence: second,
        values,
        controls,
    })
}

/// **The statements above the rows**, in the drawing's order.
fn the_statements(sitting: &ASitting, dropped: bool, the_list_in_settings: Option<&[Vec<u8>]>) -> Vec<AStatement> {
    let mut out = Vec::new();
    // **`Dependency Upload & Remap` K5: where Smoozy has no record of where
    // the file came from on this computer, the check says so at the top,
    // once** — `Design/Dependency Upload & Remap § Above the rows`, E1-3's
    // line. The rows then read as they always do. Step 10c's twelfth-pass fix
    // piece 6: the twelfth pass read 58 rows of *Not found* with no reason.
    if sitting.of.as_ref().is_some_and(|of| {
        matches!(of.holding, HoldingFile::OnTheMachineWithNoKnownLocalOrigin { .. })
    }) {
        out.push(a_statement(
            "no-record-of-where-it-came-from",
            Words::plain(words::NO_RECORD_OF_WHERE_IT_CAME_FROM),
            false,
        ));
    }
    // **What the last *Add* found, said once** — `§ The search folders on
    // this panel`: *"Add closes the box, and the panel says once what it
    // did"*. The Creator's note 5 on 0.1.8.
    if let Some(added) = &sitting.the_folder_added {
        out.push(the_folder_added(added));
    }
    // `Dependency Upload & Remap` G6a: names the library, never the item.
    for linked in &sitting.linked_items {
        out.push(a_statement(
            "a-linked-item",
            Words::plain("")
                .and(the_last_part(linked.library.as_bytes()))
                .then(words::IS_HERE_BUT_SOMETHING_IS_GONE)
                .then(words::BLENDER_DOES_NOT_SAY_WHICH),
            false,
        ));
    }
    // `Dependency Upload & Remap` G9a: never *Missing*, since no folder is
    // predicted.
    for bake in &sitting.the_bake_folders_not_worked_out {
        out.push(a_statement(
            "a-bake-folder-not-worked-out",
            Words::plain(words::CANNOT_WORK_OUT_THE_BAKE_FOLDER)
                .and(lossy(bake.recorded.as_bytes()))
                .then(words::FULL_STOP)
                .joined()
                .then(words::BLENDER_MAY_HAVE_PACKED_IT),
            false,
        ));
    }
    // `Dependency Upload & Remap` V9a.
    if sitting.sent_up_as_a_dependency_before {
        out.push(a_statement(
            "sent-up-before",
            Words::plain(words::UPLOADED_AS_A_DEPENDENCY_BEFORE).then(words::ITS_LOCATIONS_STILL_POINT_HOME),
            false,
        ));
    }
    // `Dependency Upload & Remap` G16.
    if dropped {
        out.push(a_statement(
            "the-connection-dropped",
            Words::plain(words::THE_CONNECTION_DROPPED).then(words::THERE_MAY_BE_MORE),
            true,
        ));
    }
    out.push(the_folders_line(sitting, the_list_in_settings));
    out
}

/// **How many folders the line names before it says how many more** — the
/// runner's ruling on the map's hole 5: three.
const FOLDERS_NAMED: usize = 3;

/// **`§ The search folders on this panel`, the first of its two lines: the
/// folders this search used, named, and *Check again* on it at all times.**
///
/// - **The folders are named by their last folder name**, in Settings' order,
///   three and then how many more, and **each carries its full path for
///   hover**. Only where a search ran: a sitting with nothing to look for, or
///   no folders to look in, searched nothing and says nothing about it.
/// - **Each name is a link that opens its folder in the computer's own file
///   manager** ([`TheAct::OpenASearchedFolder`], through `crate::reveal`),
///   and ***The matches came from …* is gone** — the Creator's note 5 on the
///   test copy, `§ The search folders on this panel` of 2026-10-04.
/// - **Where Settings' list is no longer the list this sitting read**, the
///   line says so. That is a comparison when the panel is drawn; **nothing
///   searches by itself**.
/// - ***Check again* is the existing act** (`TheAct::CheckAgain`, which starts a
///   sitting), and `Dependency Upload & Remap` K4 keeps every answer the user
///   gave through it.
///
/// The Creator's note 5 on 0.1.8.
fn the_folders_line(sitting: &ASitting, the_list_in_settings: Option<&[Vec<u8>]>) -> AStatement {
    let mut said = Words::plain("");
    let mut the_full_paths = Vec::new();
    let searched = sitting.the_search.as_ref().filter(|search| !search.folders.is_empty());
    if let Some(search) = searched {
        said = Words::plain(words::SEARCHED);
        let named = search.folders.len().min(FOLDERS_NAMED);
        for (at, (folder, _)) in search.folders.iter().take(named).enumerate() {
            if at > 0 {
                said = said.then(words::COMMA).joined();
            }
            said = said.and(the_last_part(folder));
            the_full_paths.push(Verbatim::new(lossy(folder)));
        }
        let more = search.folders.len() - named;
        said = if more > 0 {
            said.then(words::AND).and(more.to_string()).then(words::MORE_FOLDERS)
        } else {
            said.then(words::FULL_STOP).joined()
        };
    }
    let changed = the_list_in_settings.is_some_and(|now| now != sitting.the_list_it_searched.as_slice());
    if changed {
        said = said.then(words::YOUR_SEARCH_FOLDERS_CHANGED);
    }
    the_check_again_line(said, the_full_paths, changed)
}

/// **The folders line, with *Check again* on it** — the existing act
/// (`TheAct::CheckAgain`), drawn as the failure shapes' small button since
/// the Creator's note 6 on the test copy.
fn the_check_again_line(said: Words, the_full_paths: Vec<Verbatim>, its_last_sentence_is_bold: bool) -> AStatement {
    AStatement {
        which: Verbatim::new("the-folders"),
        said,
        its_first_sentence_is_bold: false,
        its_last_sentence_is_bold,
        the_full_paths,
        its_folders_open_with: Some(Verbatim::new("open-a-searched-folder")),
        control: Some(a_control(words::CHECK_AGAIN, "check-again", false)),
    }
}

/// **The folder a searched folder's link names**: the one at `row` in the
/// list this sitting's search used, which is the order the folders line names
/// them in. Nothing where there is no such folder.
fn the_searched_folder_at(held: &WhatTheRunHolds, server: &str, file: &str, row: Option<usize>) -> Option<Vec<u8>> {
    let search = held.sittings.get(&keyed(server, file))?.the_search.as_ref()?;
    search.folders.get(row?).map(|(folder, _)| folder.clone())
}

/// **A searched folder, opened by the existing reveal's second act**
/// (`Desktop app and delivery` K20 to K24), **and a refusal said, not
/// dropped** — K22, and the sentence `Download` H8 says for a job's files.
/// The folder's bytes become a path by `crate::checking_a_file::a_path`, the
/// rule the search itself was given them by.
fn a_searched_folder_is_opened(
    folder: Option<&[u8]>,
    open: impl FnOnce(&std::path::Path) -> crate::reveal::Revealed,
) -> Option<Prose> {
    let opened = folder
        .and_then(crate::checking_a_file::a_path)
        .is_some_and(|path| open(&path).it_opened());
    (!opened).then(|| Prose::new(words::COULD_NOT_OPEN_THE_FOLDER))
}

/// ***Added Assets. 4 found, 3 still missing.*** — or *Nothing more was
/// found.*, or no *still missing* where none is left (the screen's word since
/// 2026-10-18).
fn the_folder_added(added: &TheFolderAdded) -> AStatement {
    let said = Words::plain(words::ADDED)
        .and(the_last_part(&added.folder))
        .then(words::FULL_STOP)
        .joined();
    let said = match (added.found, added.still_not_found) {
        (0, _) => said.then(words::NOTHING_MORE_WAS_FOUND),
        (found, 0) => said.and(found.to_string()).then(words::FOUND_AND_NO_MORE),
        (found, still) => said
            .and(found.to_string())
            .then(words::FOUND_AND_MORE)
            .and(still.to_string())
            .then(words::STILL_MISSING),
    };
    AStatement {
        which: Verbatim::new("a-folder-added"),
        said,
        its_first_sentence_is_bold: true,
        its_last_sentence_is_bold: false,
        the_full_paths: vec![Verbatim::new(lossy(&added.folder))],
        its_folders_open_with: None,
        control: None,
    }
}

/// **`Dependency Upload & Remap` A9c: a press that sent nothing says why, once.**
fn what_the_send_said(said: WhatASendSaid) -> AStatement {
    let said = match said {
        WhatASendSaid::TheyAreAlreadyOnTheServer(files) => Words::plain(words::NOTHING_TO_UPLOAD)
            .and(files.to_string())
            .then(if files == 1 { words::FILE_IS_ALREADY_THERE } else { words::FILES_ARE_ALREADY_THERE }),
        WhatASendSaid::TheyAreAlreadyUploading => {
            Words::plain(words::NOTHING_TO_UPLOAD).then(words::THEY_ARE_ALREADY_UPLOADING)
        }
    };
    AStatement {
        which: Verbatim::new("a-send-sent-nothing"),
        said,
        its_first_sentence_is_bold: true,
        its_last_sentence_is_bold: false,
        the_full_paths: Vec::new(),
        its_folders_open_with: None,
        control: None,
    }
}

/// **What *Add* said is let go of at the user's next act on this panel**, so
/// it is said once — every act but the add itself, which says it anew.
pub fn the_added_statement_is_let_go(held: &mut WhatTheRunHolds, server: &str, file: &str, act: TheAct) {
    held.what_a_send_said.remove(&keyed(server, file));
    // **Opening a searched folder is not an act on the check**: it leaves the
    // panel as it was.
    if act == TheAct::AddASearchFolder || act == TheAct::OpenASearchedFolder {
        return;
    }
    if let Some(sitting) = held.sittings.get_mut(&keyed(server, file)) {
        sitting.the_folder_added = None;
    }
}

/// **Several refused for one reason, said once above the group** — each row
/// keeps its own state and its own *Try again*.
fn the_once(once: &SeveralForOneReason) -> TheFailure {
    TheFailure {
        which: Verbatim::new("several-for-one-reason"),
        sentence: Prose::new(words::THESE_FILES_COULD_NOT_BE_WRITTEN),
        the_second_sentence: Some(Prose::new(words::NO_PERMISSION_THERE)),
        values: vec![
            AValue {
                name: Prose::new(words::FILES),
                value: Verbatim::new(once.rows.len().to_string()),
            },
            AValue {
                name: Prose::new(words::FOLDER),
                value: Verbatim::new(lossy(&once.folder)),
            },
        ],
        // **No admin-access offer** — `Design/Dependency Upload & Remap § The
        // seven states`, conformed 2026-09-27 under Rule 10: these files are
        // uploads, and `File Browser` P6a gives admin access no upload to
        // retry. The drawing's own statement carries none either.
        controls: vec![
            AShapesControl {
                is: Verbatim::new("raw-text"),
                label: Prose::new(words::WHAT_THE_SERVER_SAID),
                act: Verbatim::new("what-the-server-said"),
            },
            AShapesControl {
                is: Verbatim::new("cross"),
                label: Prose::new(crate::presentation::the_failure_shape::CLEAR_THIS),
                act: Verbatim::new("clear-the-statement"),
            },
        ],
    }
}

fn the_tools(sitting: &ASitting) -> TheTools {
    let Some(search) = &sitting.the_search else {
        return TheTools {
            how_far: None,
            stop: None,
            unreadable: Vec::new(),
            search_folders: Prose::new(words::SEARCH_FOLDERS),
        };
    };
    let done = search
        .folders
        .iter()
        .filter(|(_, verdict)| verdict.is_some())
        .count();
    TheTools {
        how_far: search.running.then(|| {
            Words::plain(words::SEARCHING_YOUR_FOLDERS)
                .and(done.to_string())
                .then(words::OF)
                .and(search.folders.len().to_string())
                .then(words::DONE)
        }),
        stop: search.running.then(|| Prose::new(words::STOP)),
        unreadable: search
            .folders
            .iter()
            .filter(|(_, verdict)| *verdict == Some(FolderVerdict::CouldNotBeRead))
            .map(|(folder, _)| Words::plain("").and(lossy(folder)).then(words::COULD_NOT_BE_READ))
            .collect(),
        search_folders: Prose::new(words::SEARCH_FOLDERS),
    }
}

/// **The bar over the chosen rows, at two or more** — `§ Acting on many
/// rows`. **It shows only the acts that apply to at least one chosen row**:
/// *Accept* where a chosen row has a proposal, *Choose a folder…* where one is
/// missing (`crate::checking_a_file` points only those), *Ignore* where one is
/// not ignored, *Reject match* where Smoozy guessed one it will
/// upload. **The count on Accept says how many will be accepted**, and the
/// bar says how many are chosen apart from it.
fn the_selection_bar(sitting: &ASitting, chosen: &BTreeSet<usize>) -> Option<TheSelectionBar> {
    if chosen.len() < 2 {
        return None;
    }
    let rows: Vec<&ACheckedRow> = chosen.iter().filter_map(|at| sitting.rows.get(*at)).collect();
    let proposals = rows
        .iter()
        .filter(|one| one.state == RowState::NotFound && one.proposal.is_some())
        .count();
    let missing = rows.iter().any(|one| one.state == RowState::NotFound);
    let ignorable = rows.iter().any(|one| one.state != RowState::SetAside);
    let guessed = rows
        .iter()
        .any(|one| one.state == RowState::FoundOnYourComputer && a_guess(one));
    Some(TheSelectionBar {
        how_many: Words::after(chosen.len().to_string(), words::SELECTED),
        accept: (proposals > 0).then(|| Words::with(words::ACCEPT, proposals.to_string())),
        choose_a_folder: missing.then(|| Prose::new(words::CHOOSE_A_FOLDER)),
        ignore: ignorable.then(|| Prose::new(words::IGNORE)),
        undo_the_guess: guessed.then(|| Prose::new(words::UNDO_SMOOZYS_GUESS)),
    })
}

fn the_shift(sitting: &ASitting, shift: &TheShiftInHand) -> TheShift {
    let resolved = one_row_per_real_file(&sitting.rows, |one| the_qualifiers(one).contains(&Qualifier::FromAShift)).len();
    TheShift {
        said: Words::plain(words::EVERYTHING_RECORDED_UNDER)
            .and(lossy(&shift.old_root))
            .then(words::WAS_FOUND_UNDER)
            .and(lossy(&shift.new_root))
            .then(words::FULL_STOP)
            .joined(),
        resolved: Words::after(
            resolved.to_string(),
            if resolved == 1 {
                words::ROW_RESOLVED
            } else {
                words::ROWS_RESOLVED
            },
        ),
        undo: Prose::new(words::UNDO_THIS_CHANGE),
    }
}

/// **The qualifiers are `Dependency Upload & Remap`'s own**, read off the
/// located answer and the proposal by `the_qualifiers_of`.
fn the_qualifiers(one: &ACheckedRow) -> Vec<Qualifier> {
    match one.the_answer() {
        Some(answer) => the_qualifiers_of(answer, one.proposal.is_some()),
        None if one.proposal.is_some() => vec![Qualifier::ACaseNearMissProposal],
        None => Vec::new(),
    }
}

/// **The rows as they are drawn, group by group** — [`THE_GROUPS`]' order,
/// each row in the group it was in when the check ran (`§ How the rows are
/// grouped and sorted`: nothing re-sorts under the user's hand). **Inside a
/// group: a row that is itself another `.blend` at its head, then the location
/// the file records** — `Dependency Upload & Remap` G11a's comparison, the
/// module's own.
fn the_rows_in_their_groups(sitting: &ASitting) -> Vec<Vec<usize>> {
    // **`Dependency Upload & Remap` G11b: one real file, one row.**
    let mut order: Vec<usize> = the_real_files(&sitting.rows).into_iter().map(|file| file[0]).collect();
    order.sort_by(|one, other| {
        let (a, b) = (&sitting.rows[*one], &sitting.rows[*other]);
        (b.kind == Kind::Library)
            .cmp(&(a.kind == Kind::Library))
            .then_with(|| compare_recorded(a.row.ordering_key(), b.row.ordering_key()))
    });
    let mut groups = vec![Vec::new(); THE_GROUPS.len()];
    for at in order {
        groups[the_group_of(sitting.rows[at].state_when_the_check_ran)].push(at);
    }
    groups
}

/// **The order a Shift-click's range runs in**: the rows as they are drawn,
/// less the Ignored group, which is not in the list a selection is made from
/// (`§ Acting on many rows`).
fn the_order_a_selection_runs_in(sitting: &ASitting) -> Vec<usize> {
    the_rows_in_their_groups(sitting)
        .into_iter()
        .enumerate()
        .filter(|(place, _)| *place != THE_IGNORED_GROUP)
        .flat_map(|(_, rows)| rows)
        .collect()
}

fn the_table(
    sitting: &ASitting,
    chosen: &BTreeSet<usize>,
    dropped: bool,
    on_its_way: &[MachineLocation],
) -> TheTable {
    let layout = sitting.of.as_ref().and_then(|of| of.layout.as_ref());
    let real_files: std::collections::HashMap<usize, Vec<usize>> = the_real_files(&sitting.rows)
        .into_iter()
        .map(|file| (file[0], file))
        .collect();
    let groups = the_rows_in_their_groups(sitting)
        .into_iter()
        .enumerate()
        .filter(|(_, rows)| !rows.is_empty())
        .map(|(place, rows)| {
            let group = &THE_GROUPS[place];
            let choosable = place != THE_IGNORED_GROUP;
            // **A heading's count is of rows** — and *Already on the server*
            // carries its size beside it: *1 · 128 MB*.
            let mut how_many = Words::plain("").and(rows.len().to_string());
            if group.which == "already-there" {
                let size: u64 = rows.iter().filter_map(|at| sitting.rows[*at].size).sum();
                if size > 0 {
                    how_many = how_many.then(words::BETWEEN).and(the_size(size));
                }
            }
            AGroup {
                which: Verbatim::new(group.which),
                heading: Prose::new(group.heading),
                colour: Verbatim::new(group.colour),
                how_many,
                folded: group.folded,
                rows: rows
                    .iter()
                    .map(|at| {
                        a_row(
                            *at,
                            &sitting.rows[*at],
                            &real_files
                                .get(at)
                                .into_iter()
                                .flatten()
                                .map(|one| &sitting.rows[*one])
                                .collect::<Vec<_>>(),
                            choosable && chosen.contains(at),
                            choosable,
                            dropped,
                            sitting.the_shift.as_ref(),
                            crate::the_found_files_sent::the_row_is_on_its_way(
                                &sitting.rows[*at],
                                layout,
                                on_its_way,
                            ),
                        )
                    })
                    .collect(),
            }
        })
        .collect();
    TheTable {
        groups,
        adds_a_row_with: Verbatim::new(if cfg!(target_os = "macos") { "meta" } else { "ctrl" }),
    }
}

// ── A location's parts, read off its bytes ──────────────────────────────────

/// A separator between the steps of a location — either system's, since a
/// `.blend` saved on Windows records its own (step 10c's dependency wave
/// piece 10b).
fn a_separator(byte: &u8) -> bool {
    *byte == b'/' || *byte == b'\\'
}

/// The location with any separator at its end let go of.
fn without_the_last_separator(bytes: &[u8]) -> &[u8] {
    let mut end = bytes.len();
    while end > 0 && a_separator(&bytes[end - 1]) {
        end -= 1;
    }
    &bytes[..end]
}

/// **The name**: the last step of a location. Where it has no step at all
/// (`//`), the location as it is.
fn the_name_in(bytes: &[u8]) -> &[u8] {
    let trimmed = without_the_last_separator(bytes);
    if trimmed.is_empty() {
        return bytes;
    }
    match trimmed.iter().rposition(a_separator) {
        Some(at) => &trimmed[at + 1..],
        None => trimmed,
    }
}

/// **The folder a file sits in**, as stored on this computer.
fn the_folder_holding(bytes: &[u8]) -> &[u8] {
    let trimmed = without_the_last_separator(bytes);
    match trimmed.iter().rposition(a_separator) {
        Some(0) => &trimmed[..1],
        Some(at) => &trimmed[..at],
        None => trimmed,
    }
}

/// **The folder as the `.blend` writes it**, separator and all — *//lib/* of
/// *//lib/props.blend*. A location recorded as a folder is that folder.
fn the_recorded_folder(bytes: &[u8]) -> &[u8] {
    if bytes.last().is_some_and(a_separator) {
        return bytes;
    }
    match bytes.iter().rposition(a_separator) {
        Some(at) => &bytes[..=at],
        None => bytes,
    }
}

/// **A row's kind, in the industry's own words, and how many files where it
/// stands for a set** — `§ A row`: *Image*, *Linked .blend*, *Sequence · 240
/// files*, *Cache · 340 files*. Every kind the reading names has a word, so a
/// kind added there fails here to compile rather than reaching the screen
/// unnamed.
fn the_kind(kind: Kind, members: Option<usize>) -> Words {
    let word = match kind {
        Kind::Library => words::LINKED_BLEND,
        Kind::Image | Kind::MovieClip if members.is_some() => words::SEQUENCE,
        Kind::Image => words::IMAGE,
        Kind::MovieClip => words::MOVIE_CLIP,
        Kind::Sound => words::SOUND,
        Kind::Font => words::FONT,
        Kind::Volume => words::VOLUME,
        Kind::CacheArchive
        | Kind::CacheLayer
        | Kind::PhysicsPointCache
        | Kind::FluidCacheFolder
        | Kind::DynamicPaintOutputFolder
        | Kind::MeshCacheExternalDisplacement => words::CACHE,
        Kind::Text => words::TEXT,
        Kind::OslScript => words::OSL_SCRIPT,
        Kind::IesProfile => words::IES_PROFILE,
        Kind::MultiresExternalDisplacement => words::DISPLACEMENT,
        Kind::GeometryNodeBakeFolder | Kind::BakeFolder | Kind::OceanBakeFolder => words::BAKE,
        Kind::SequencerStripFolder => words::SEQUENCE,
        Kind::SequencerStripFile => words::STRIP,
        Kind::ProxyFolder => words::PROXY,
        Kind::BrushIcon => words::BRUSH_ICON,
        Kind::ImportNodePath => words::IMPORTED_FILE,
        Kind::LibraryWeakReference | Kind::PackedFileRecord => words::A_FILE,
    };
    match members {
        Some(members) => Words::plain(word)
            .then(words::BETWEEN)
            .and(members.to_string())
            .then(if members == 1 { words::FILE_WORD } else { words::FILES_WORD }),
        None => Words::plain(word),
    }
}

/// **Where the reading's scene walk starts from a data-block that Blender's
/// record of what uses what holds as a key** (`bpy.data.user_map()`, the
/// walk `Dependency Upload & Remap` G19 takes outwards), so that reaching no
/// scene is a finding about use and not about where the walk began.
///
/// **Observed at the desk on 2026-10-18, Blender 5.1.2 headless, on a copy of
/// the Creator's `liquid in a pipe.blend`**: the walk counts as a scene using
/// a file every `Scene` it reaches by stepping from the file's data-block to
/// whatever uses it, and on to whatever uses that — `pipe_00001.png` is used
/// by the node group *Compositor Nodes*, which `Scene.001` uses as its
/// compositor, so it reaches `Scene.001`. **And three starts can never reach
/// one**: a material's own node tree is not a key in that record at all, a
/// `Library` is used only by other libraries there, and a mark a `Scene`
/// itself owns starts the walk at that scene, which is never its own user.
/// So a linked `.blend`, an OSL script, an IES profile, an imported file, and
/// the scene's own bake folder, rigid-body cache, strips and proxies would
/// each read *no scene* whatever the file does with them. **For those the
/// note says nothing**, which is what an incomplete walk does
/// (`Dependency Upload & Remap` G20).
fn the_walk_starts_where_a_scene_can_be_reached(kind: Kind) -> bool {
    matches!(
        kind,
        Kind::Image
            | Kind::MovieClip
            | Kind::Sound
            | Kind::Font
            | Kind::Volume
            | Kind::CacheArchive
            | Kind::CacheLayer
            | Kind::Text
            | Kind::FluidCacheFolder
            | Kind::GeometryNodeBakeFolder
            | Kind::OceanBakeFolder
            | Kind::MultiresExternalDisplacement
            | Kind::MeshCacheExternalDisplacement
            | Kind::DynamicPaintOutputFolder
    )
}

/// A note in its colour, with what it names in full on hover.
fn a_note(said: Words, colour: &str, hover: Option<&[u8]>) -> ANote {
    ANote {
        said,
        colour: Verbatim::new(colour),
        hover: hover.map(|bytes| Verbatim::new(lossy(bytes))),
        failure: None,
    }
}

/// **A failure's own reason** — `§ A row`'s second note, in red.
fn the_reason(why: Option<WhyItDidNotGoUp>) -> &'static str {
    match why {
        // **`Dependency Upload & Remap` M14a: a failed row says which.**
        Some(WhyItDidNotGoUp::SmoozyCouldNotOpenItToRemapItsPaths) => words::COULD_NOT_OPEN_IT_TO_REMAP,
        Some(WhyItDidNotGoUp::SmoozyCouldNotRemapItsPaths) => words::COULD_NOT_UPDATE_ITS_FILE_PATHS,
        Some(WhyItDidNotGoUp::ThePathIsTooLong) => words::UPLOAD_FAILED_THE_PATH_IS_TOO_LONG,
        Some(WhyItDidNotGoUp::ItCouldNotBeReadHere) => words::COULD_NOT_READ_ON_YOUR_COMPUTER,
        Some(WhyItDidNotGoUp::TheUploadFailed) => words::UPLOAD_FAILED,
        Some(WhyItDidNotGoUp::TheServerRefusedTheWrite) | None => words::UPLOAD_FAILED_NO_PERMISSION,
    }
}

/// **One row, naming the file once** — `§ A row`, rewritten 2026-10-18.
///
/// **`on_its_way`: a file of it is already uploading** — `§ The seven
/// states`' paragraph of 2026-10-24 and `Dependency Upload & Remap` A9a: the
/// row says *Uploading* among its notes, in `busy`, and stays in its group.
#[allow(clippy::too_many_arguments)]
fn a_row(
    at: usize,
    one: &ACheckedRow,
    of_this_file: &[&ACheckedRow],
    chosen: bool,
    choosable: bool,
    dropped: bool,
    shift: Option<&TheShiftInHand>,
    on_its_way: bool,
) -> ARow {
    let recorded = one.row.ordering_key().as_bytes();
    let resolution = one.the_answer().and_then(|answer| answer.resolution.as_ref());
    // **`Dependency Upload & Remap` G6d: *The file says* reads as the file does.**
    // Blender writes a library's relative location again as the file loads, so
    // for a library the file's own text is what was first looked for.
    let the_file_says = if one.kind == Kind::Library {
        one.wanted
            .iter()
            .find(|wanted| wanted.reference.as_bytes() == recorded)
            .map_or(recorded, |wanted| wanted.the_location_read.as_bytes())
    } else {
        recorded
    };

    // **One location: the folder on this computer it uploads from** — a
    // file's folder, or the folder itself where the row is a folder or a set;
    // **where nothing was found, *The file says:* and the folder as the
    // `.blend` writes it.**
    let location = match resolution {
        Some(resolution) => {
            let whole = resolution.to.location().as_bytes();
            let a_file = matches!(resolution.to, ResolvedTo::AWholeLocation(_))
                && one.wanted.first().is_none_or(|wanted| wanted.shape == WantedShape::AFile);
            let folder = if a_file {
                the_folder_holding(whole)
            } else {
                without_the_last_separator(whole)
            };
            Words::plain("").and(lossy(folder))
        }
        None => Words::plain(words::THE_FILE_SAYS).and(lossy(the_recorded_folder(the_file_says))),
    };

    // **The notes, every one that applies, in `§ A row`'s fixed order.**
    let mut notes = Vec::new();
    // **A file of it already uploading: *Uploading*, first, in work in
    // progress's colour**, from the moment its send is running or waiting
    // until it ends — when the row takes the state its ending gives. The
    // drawing's `chk-sending`.
    if on_its_way {
        notes.push(a_note(Words::plain(words::UPLOADING), "busy", None));
    }
    // 1. **A change since the check ran, in the new state's colour** — the
    // row stays in its group (nothing re-sorts under the user's hand).
    if one.state != one.state_when_the_check_ran {
        let now = match (one.state_when_the_check_ran, one.state) {
            (RowState::SetAside, RowState::NotFound) => Some((words::MISSING, "bad")),
            (_, RowState::NotFound) => Some((words::FILE_NO_LONGER_FOUND, "bad")),
            (_, RowState::FoundOnYourComputer) => Some((words::READY_TO_UPLOAD, "")),
            (_, RowState::SetAside) => Some((words::IGNORED, "idle")),
            (_, RowState::AlreadyOnTheMachine) => Some((words::ALREADY_ON_THE_SERVER, "ok")),
            (_, RowState::SentBeforeNotConfirmed) => Some((words::UPLOADED_BEFORE_NOT_VERIFIED, "busy")),
            // A failure's own reason, the next note, says it.
            (_, RowState::CouldNotBeSent | RowState::CouldNotBeFixed) => None,
        };
        if let Some((said, colour)) = now {
            notes.push(a_note(Words::plain(said), colour, None));
        }
    }
    // 2. **A failure's own reason, in red** — with a cross among the row's
    // buttons while it stands, without one once its cross was pressed.
    if is_a_failure(one.state) {
        let reason = the_reason(one.why_it_did_not_go_up);
        if one.its_failure_was_cleared {
            notes.push(a_note(Words::plain(reason), "bad", None));
        } else {
            notes.push(ANote {
                said: Words::plain(""),
                colour: Verbatim::new("bad"),
                hover: None,
                failure: Some(TheRowsFailure {
                    sentence: Prose::new(reason),
                    clear_this: Prose::new(crate::presentation::the_failure_shape::CLEAR_THIS),
                }),
            });
        }
    }
    // 3. **How it was found, where that is not where the file says.** *Also
    // in Textures, later in your search folders* is not drawn: the search's
    // answer keeps no candidate outside the folder that won (`Dependency
    // Upload & Remap` F4), so nothing here could say it.
    match resolution.map(|resolution| &resolution.how) {
        Some(HowResolved::TheSearch { from_folder, arbitrary }) => {
            notes.push(a_note(
                Words::with(words::FOUND_THROUGH_SEARCH_FOLDER, lossy(the_name_in(from_folder)))
                    .then(words::FULL_STOP)
                    .joined()
                    .then(words::THE_FILE_SAYS)
                    .and(lossy(the_recorded_folder(the_file_says))),
                "",
                Some(from_folder),
            ));
            if let Some(pick) = arbitrary {
                notes.push(a_note(
                    Words::after(pick.among.len().to_string(), words::MATCHES_IN)
                        .and(lossy(the_name_in(from_folder)))
                        .then(words::SEMICOLON)
                        .joined()
                        .then(words::SMOOZY_CHOSE_ONE),
                    "",
                    Some(from_folder),
                ));
            }
        }
        Some(HowResolved::AShift(key)) => {
            let said = match shift.filter(|shift| shift.key == *key) {
                Some(shift) => Words::plain(words::FOUND_BY_YOUR_MOVE)
                    .then(words::FROM)
                    .and(lossy(&shift.old_root))
                    .then(words::TO)
                    .and(lossy(&shift.new_root)),
                None => Words::plain(words::FOUND_BY_YOUR_MOVE),
            };
            notes.push(a_note(said, "", None));
        }
        Some(HowResolved::TheUsersOwnHand) => notes.push(a_note(Words::plain(words::YOU_CHOSE_THIS_FILE), "", None)),
        Some(HowResolved::TheRecordedLocationItself) | None => {}
    }
    if one.state == RowState::NotFound {
        // **A capitals-only proposal is a proposal, not a match** — the
        // found name, its whole location on hover.
        if let (Some(proposal), true) = (&one.proposal, the_qualifiers(one).contains(&Qualifier::ACaseNearMissProposal)) {
            notes.push(a_note(
                Words::with(words::ONLY_THE_CAPITALS_DIFFER, lossy(the_name_in(proposal.as_bytes()))),
                "",
                Some(proposal.as_bytes()),
            ));
        }
        // `§ A guess the user rejected`, in red.
        if let Some(rejected) = the_rejected_guess(one) {
            notes.push(a_note(
                Words::with(words::YOU_REJECTED, lossy(the_name_in(rejected.as_bytes()))),
                "bad",
                Some(rejected.as_bytes()),
            ));
        }
    }
    // 4. **What the row stands for** — `§ When more than one reference points
    // at one thing`, and a missing linked `.blend`.
    // **Counted over every row of this real file** (`Dependency Upload & Remap`
    // G11b): what the row keeps of who wants it.
    let references: usize = of_this_file.iter().map(|row| row.row.references.len()).sum();
    if references > 1 {
        notes.push(a_note(
            Words::with(words::THE_FILE_POINTS_TO_THIS, references.to_string()).then(words::TIMES),
            "",
            None,
        ));
    }
    if one.state == RowState::NotFound && one.kind == Kind::Library {
        notes.push(a_note(Words::plain(words::IT_MAY_ADD_MORE_ROWS), "", None));
    }
    // **`Dependency Upload & Remap` G6f: a library linked by a full path says so**,
    // read off the file's own text of it (`Dependency Upload & Remap` G6d). Nothing
    // is blocked.
    if one.kind == Kind::Library
        && of_this_file
            .iter()
            .flat_map(|row| row.wanted.iter())
            .any(|wanted| !wanted.the_location_read.as_bytes().starts_with(b"//"))
    {
        notes.push(a_note(Words::plain(words::LINKED_BY_FULL_PATH), "", None));
    }
    // 5. **The spec's one exception to saying nothing about scenes**: only
    // where the reading finished for every reference the row stands for
    // (`Dependency Upload & Remap` G20a's *established*, which the sitting
    // carries per row) and reached no scene — and only where the walk starts
    // from something it could reach a scene from. It acts on nothing.
    if !of_this_file.is_empty()
        && of_this_file
            .iter()
            .all(|row| matches!(&row.scenes, Attribution::Established(scenes) if scenes.is_empty()))
        && the_walk_starts_where_a_scene_can_be_reached(one.kind)
    {
        notes.push(a_note(Words::plain(words::NO_SCENE_USES_IT), "", None));
    }

    // **At most one visible action**, and **⋯ holding the rest**: *Locate…*,
    // *Ignore*, *Restore*, and *Reject match* where Smoozy guessed.
    let action = the_visible_action(one, dropped);
    let visible = action.as_ref().map(|action| action.act.as_str().to_string());
    let mut more_items = Vec::new();
    if visible.as_deref() != Some("locate") {
        more_items.push(a_control(words::LOCATE, "locate", false));
    }
    if one.state == RowState::SetAside {
        if visible.as_deref() != Some("restore") {
            more_items.push(a_control(words::RESTORE, "restore", false));
        }
    } else {
        more_items.push(a_control(words::IGNORE, "set-aside", false));
    }
    if a_guess(one) && one.state == RowState::FoundOnYourComputer {
        more_items.push(a_control(words::UNDO_SMOOZYS_GUESS, "undo-the-guess", false));
    }
    if is_a_failure(one.state) {
        more_items.push(a_control(words::WHAT_THE_SERVER_SAID, "what-the-server-said", false));
    }

    ARow {
        at: Verbatim::new(at.to_string()),
        // `Dependency Upload & Remap` G9a: the folder the render reads, while
        // *The file says* keeps the recorded bytes.
        name: Verbatim::new(lossy(the_name_in(
            one.wanted
                .iter()
                .find(|wanted| wanted.reference.as_bytes() == recorded)
                .map_or(recorded, |wanted| wanted.the_location_read.as_bytes()),
        ))),
        a_linked_blend: one.kind == Kind::Library,
        kind: the_kind(one.kind, one.members),
        location,
        the_file_says: Verbatim::new(lossy(the_file_says)),
        notes,
        size: one.size.map(|bytes| Verbatim::new(the_size(bytes))),
        action,
        more: Prose::new(words::MORE),
        more_items,
        chosen,
        choosable,
    }
}

/// **What is to upload, in files and bytes** — every row found on this
/// computer, a linked `.blend` among them since the 38's piece 7
/// (`crate::the_found_files_sent`'s head). **Counted in files**, so a folder
/// sent as one counts every file in it (`§ How the rows are grouped and
/// sorted`). **The summary sentence's figure**, which counts a row already
/// uploading too, since nothing of it has landed — the drawing's
/// `chk-sending` keeps *2 files to upload, 90 MB*; the foot counts only the
/// rest ([`the_foot`]).
fn the_upload(sitting: &ASitting) -> (usize, u64) {
    let sent = crate::the_found_files_sent::the_rows_the_upload_sends(&sitting.rows);
    let files = sent.iter().map(|at| sitting.rows[*at].what_a_send_would_take().0).sum();
    let size = sent.iter().map(|at| sitting.rows[*at].what_a_send_would_take().1).sum();
    (files, size)
}

/// **`§ Along the bottom`: only the button**, naming the number of files and
/// their size — *Upload 2 files (90 MB)*, or *Upload the 4 found so far*
/// while the check is still reading. The figures are the summary sentence's.
///
/// **It counts only the rows not already on their way**, and **where every
/// row under *To upload* is on its way it is not there** — the strip's
/// *Uploading 4 of 12* says what is happening (`§ Along the bottom`'s
/// paragraph of 2026-10-24, `Dependency Upload & Remap` A9a). The press
/// decides again from what is going at that moment.
/// The drawing's `chk-onserver` also has no button when every file is there.
fn the_foot(
    sitting: &ASitting,
    dropped: bool,
    so_far: bool,
    on_its_way: &[MachineLocation],
    being_counted: bool,
) -> Option<TheFoot> {
    // **`Dependency Upload & Remap` A9c: a press shows at once.**
    if being_counted {
        return Some(TheFoot {
            upload: Words::plain(crate::surface::transfers::words::COUNTING_FILES),
            upload_is_unavailable: true,
        });
    }
    let layout = sitting.of.as_ref().and_then(|of| of.layout.as_ref());
    // **`Dependency Upload & Remap` Q5a: never a file the send will not take.**
    let left: Vec<usize> = crate::the_found_files_sent::the_rows_left_to_upload(&sitting.rows, layout, on_its_way)
        .into_iter()
        .filter(|at| sitting.rows[*at].what_a_send_would_take().0 > 0)
        .collect();
    if left.is_empty() {
        return None;
    }
    let files: usize = left.iter().map(|at| sitting.rows[*at].what_a_send_would_take().0).sum();
    let size: u64 = left.iter().map(|at| sitting.rows[*at].what_a_send_would_take().1).sum();
    let upload = if so_far {
        Words::with(words::UPLOAD_THE, files.to_string()).then(words::FOUND_SO_FAR)
    } else {
        let said = Words::with(words::UPLOAD, files.to_string()).then(if files == 1 {
            words::FILE_WORD
        } else {
            words::FILES_WORD
        });
        if files > 0 {
            said.then(words::OPENS).and(the_size(size)).joined().then(words::CLOSES).joined()
        } else {
            said
        }
    };
    Some(TheFoot {
        upload,
        // **Sending needs the machine** — `§ When the connection drops
        // mid-check`.
        upload_is_unavailable: dropped || files == 0,
    })
}

/// **One stretch of the summary sentence**: words, a figure, more words, in
/// one colour — or words alone that join the stretch before.
struct AStretch {
    lead: &'static str,
    value: Option<String>,
    rest: &'static str,
    colour: &'static str,
    joins: bool,
}

impl AStretch {
    fn a_figure(value: String, rest: &'static str, colour: &'static str) -> AStretch {
        AStretch { lead: "", value: Some(value), rest, colour, joins: false }
    }

    fn words(said: &'static str) -> AStretch {
        AStretch { lead: said, value: None, rest: "", colour: "", joins: false }
    }

    fn joined_words(lead: &'static str, rest: &'static str) -> AStretch {
        AStretch { lead, value: None, rest, colour: "", joins: true }
    }

    fn on_the_panel(&self) -> AFigure {
        let mut said = Words::plain(self.lead);
        if let Some(value) = &self.value {
            said = said.and(value.clone());
        }
        AFigure {
            said: said.then(self.rest),
            colour: Verbatim::new(self.colour),
            joins: self.joins,
        }
    }
}

/// **The summary sentence, composed once for the panel and the strip** —
/// `§ Above the rows`, since the dependency check's fresh look:
///
/// - **where nothing is open, it says so first**: *Nothing is missing.* —
///   and **never while any row is failed, missing or not verified, or while
///   the check is still reading**;
/// - **then what is open, worst first in the order of the groups**, every zero
///   left out: *2 failed · 3 missing · 1 not verified*;
/// - **then what will upload, counted in files, and its size**, and the size
///   already on the server: *344 files to upload, 2.2 GB · 128 MB already on
///   the server*;
/// - **ending *, found so far.* while the check is still reading.**
///
/// **Ignored rows are not in it**, since ignoring a row is the user's answer.
/// Nothing is said where nothing would be: a check still reading that has
/// found nothing yet.
fn the_summary_of(sitting: &ASitting, so_far: bool) -> Vec<AStretch> {
    // **`Dependency Upload & Remap` G11b: each real file once, in every figure.**
    let count = |wanted: &dyn Fn(RowState) -> bool| one_row_per_real_file(&sitting.rows, |one| wanted(one.state)).len();
    let failed = count(&is_a_failure);
    let missing = count(&|state| state == RowState::NotFound);
    let not_verified = count(&|state| state == RowState::SentBeforeNotConfirmed);
    let (files, size) = the_upload(sitting);
    let on_the_server: u64 = one_row_per_real_file(&sitting.rows, |one| one.state == RowState::AlreadyOnTheMachine)
        .into_iter()
        .filter_map(|at| sitting.rows[at].size)
        .sum();

    let mut figures: Vec<Vec<AStretch>> = Vec::new();
    if failed > 0 {
        figures.push(vec![AStretch::a_figure(failed.to_string(), words::FAILED_FIGURE, "bad")]);
    }
    if missing > 0 {
        figures.push(vec![AStretch::a_figure(missing.to_string(), words::MISSING_FIGURE, "bad")]);
    }
    if not_verified > 0 {
        figures.push(vec![AStretch::a_figure(not_verified.to_string(), words::NOT_VERIFIED_FIGURE, "busy")]);
    }
    if files > 0 {
        figures.push(vec![
            AStretch::a_figure(
                files.to_string(),
                if files == 1 { words::FILE_TO_UPLOAD } else { words::FILES_TO_UPLOAD },
                "",
            ),
            AStretch::a_figure(the_size(size), "", ""),
        ]);
    }
    if on_the_server > 0 {
        figures.push(vec![AStretch::a_figure(
            the_size(on_the_server),
            words::ALREADY_ON_THE_SERVER_FIGURE,
            "ok",
        )]);
    }

    // **`§ Above the rows`, 2026-11-04: a linked file going up has not been read**,
    // because one not yet on the server cannot be, so what lies below it is not
    // known and *Nothing is missing* would be false about the file. *, found so
    // far* already says it while the check is reading.
    let linked_files_not_checked = !so_far
        && sitting.rows.iter().any(|one| one.kind == Kind::Library && one.state == RowState::FoundOnYourComputer);
    let nothing_is_open = failed == 0 && missing == 0 && not_verified == 0 && !so_far && !linked_files_not_checked;
    let mut out = Vec::new();
    if nothing_is_open {
        out.push(AStretch::words(words::NOTHING_IS_MISSING));
    } else if figures.is_empty() {
        return out;
    }
    let any = !figures.is_empty();
    for (at, figure) in figures.into_iter().enumerate() {
        if at > 0 {
            out.push(AStretch::words(words::BETWEEN));
        }
        out.extend(figure);
    }
    if so_far {
        out.push(AStretch::joined_words(words::COMMA, words::FOUND_SO_FAR_AT_THE_END));
    } else if any {
        out.push(AStretch::joined_words(words::FULL_STOP, ""));
    }
    if linked_files_not_checked {
        out.push(AStretch::words(words::LINKED_FILES_NOT_CHECKED_YET));
    }
    out
}

/// **The same sentence on the strip**, in the parts `web/a-server.js` draws
/// side by side. **The strip's words are plain**: its colour stays the
/// strip's, as the drawing's `#ckstripd` has it. A stretch that joins the one
/// before it is set against that one's words.
fn on_the_strip(stretches: &[AStretch]) -> Vec<AStripsWords> {
    let mut out: Vec<(String, Vec<String>, Option<String>, bool)> = Vec::new();
    for one in stretches {
        if one.joins {
            let said = [one.lead, one.rest]
                .iter()
                .filter(|piece| !piece.is_empty())
                .copied()
                .collect::<Vec<_>>()
                .join(" ");
            if let Some(last) = out.last_mut() {
                match &mut last.2 {
                    Some(rest) => rest.push_str(&said),
                    None => {
                        last.2 = Some(said);
                        last.3 = true;
                    }
                }
                continue;
            }
        }
        out.push((
            one.lead.to_string(),
            one.value.iter().cloned().collect(),
            (!one.rest.is_empty()).then(|| one.rest.to_string()),
            false,
        ));
    }
    out.into_iter()
        .map(|(lead, values, rest, the_rest_joins)| AStripsWords {
            lead: Prose::new(lead),
            values: values.into_iter().map(Verbatim::new).collect(),
            rest: rest.map(Prose::new),
            the_rest_joins,
        })
        .collect()
}

fn the_overwrite(asked: &AnOverwrite) -> TheOverwrite {
    TheOverwrite {
        title: Prose::new(words::A_DIFFERENT_FILE_IS_THERE),
        what_was_read: vec![
            Prose::new(words::SMOOZY_READ_THE_FILE_THERE),
            Prose::new(words::IT_IS_NOT_THE_ONE),
        ],
        location: Verbatim::new(lossy(&asked.location)),
        live_jobs: the_live_job_warning(&asked.jobs, &asked.labels),
        the_folder: asked.the_folder.as_ref().map(|folder| {
            Words::plain(words::THIS_ROW_IS_A_FOLDER)
                .and(folder.differ.to_string())
                .then(words::OF_ITS)
                .and(folder.members.to_string())
                .then(words::FILES_DIFFER_AND_THE_SERVER_HOLDS)
                .and(folder.only_on_the_server.to_string())
                .then(words::FILES_YOURS_DOES_NOT)
        }),
        stop: Prose::new(words::STOP),
        replace: Prose::new(words::REPLACE),
    }
}

fn the_folder_box() -> TheFolderBox {
    TheFolderBox {
        opener: Prose::new(words::SEARCH_FOLDERS),
        title: Prose::new(words::ADD_A_SEARCH_FOLDER),
        the_folders: Vec::new(),
        your_search_folders: Prose::new(words::YOUR_SEARCH_FOLDERS),
        change_them_in_settings: Prose::new(words::CHANGE_THEM_IN_SETTINGS),
        choose_a_folder: Prose::new(words::CHOOSE_A_FOLDER),
        chosen: None,
        what_adding_does: vec![Words::plain(words::IT_IS_ADDED_AND_SEARCHED)],
        cancel: Prose::new(words::CANCEL),
        add: Prose::new(words::ADD),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_dependencies::check::GroupedBy;
    use smoozy_dependencies::record::{HoldingFile, LocatedAnswerKey, Resolution, ResolvedTo, ShiftKey};
    use smoozy_server_setup::blender::Version;
    use std::fs;
    use std::path::Path;

    /// Everything before the tests. The sweeps read it rather than the whole
    /// file, because the things they forbid are written out here to forbid them.
    fn the_working_half() -> &'static str {
        include_str!("the_dependency_check.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half")
    }

    fn the_page() -> String {
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-dependency-check.js"))
            .expect("the page module is readable")
    }

    fn the_mockup() -> String {
        fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the drawing is readable")
    }

    const SERVER: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const FILE: &str = "/Users/alex/work/kitchen/kitchen_v14.blend";

    fn a_blender() -> BlenderIdentity {
        BlenderIdentity {
            path: "/root/blender/blender-5.1.0/blender".to_string(),
            version: Version { major: 5, minor: 1, patch: 0 },
        }
    }

    fn a_row(reference: &[u8], kind: Kind, state: RowState) -> ACheckedRow {
        ACheckedRow {
            row: Row {
                references: vec![RecordedLocation::as_the_reading_carried_it(reference)],
                grouped_by: GroupedBy::TidiedText(reference.to_vec()),
            },
            kind,
            scenes: Attribution::Established(vec!["Interior".to_string()]),
            state_when_the_check_ran: state,
            state,
            holding: HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(FILE.as_bytes())),
            answers: vec![LocatedAnswer {
                key: LocatedAnswerKey {
                    holding_file: HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(
                        FILE.as_bytes(),
                    )),
                    reference: RecordedLocation::as_the_reading_carried_it(reference),
                },
                resolution: None,
                set_aside: state == RowState::SetAside,
                rejected_guess: None,
                read_as: Vec::new(),
            }],
            wanted: vec![Outstanding::a_file(RecordedLocation::as_the_reading_carried_it(reference))],
            proposal: None,
            members: None,
            size: Some(1024 * 1024),
            why_it_did_not_go_up: None,
            the_part_already_up: None,
            its_failure_was_cleared: false,
        }
    }

    fn found_by_a_shift(reference: &[u8]) -> ACheckedRow {
        let mut one = a_row(reference, Kind::Image, RowState::FoundOnYourComputer);
        one.answers = vec![LocatedAnswer {
            key: LocatedAnswerKey {
                holding_file: HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(FILE.as_bytes())),
                reference: RecordedLocation::as_the_reading_carried_it(reference),
            },
            resolution: Some(Resolution {
                to: ResolvedTo::AWholeLocation(StoredLocation::as_the_system_returned_it(b"/Users/alex/Archive/x.png")),
                how: HowResolved::AShift(ShiftKey(1)),
            }),
            set_aside: false,
            rejected_guess: None,
            read_as: Vec::new(),
        }];
        one
    }

    fn a_sitting(rows: Vec<ACheckedRow>) -> ASitting {
        ASitting {
            read_with: a_blender(),
            linked_files_read: 2,
            still_reading_below: None,
            the_walk: None,
            completion: Some(Completion::Completed),
            linked_items: Vec::new(),
            the_bake_folders_not_worked_out: Vec::new(),
            sent_up_as_a_dependency_before: false,
            the_search: None,
            the_shift: None,
            the_once: None,
            rows,
            the_chosen_search_folder: None,
            the_list_it_searched: Vec::new(),
            the_folder_added: None,
            of: None,
        }
    }

    fn a_run(sitting: ASitting) -> WhatTheRunHolds {
        let mut held = WhatTheRunHolds::default();
        held.sittings.insert(keyed(SERVER, FILE), sitting);
        held
    }

    fn the_texts(words: &Words) -> String {
        format!("{words:?}")
    }

    /// **The eighteen states are the drawing's own `chk-` set**, read out of
    /// its state list rather than agreed with by hand. *(Nine until the plan
    /// pass for the Creator's notes on 0.1.8 drew `chk-added` and
    /// `chk-folders`; eleven until `chk-reading`; twelve until `chk-clean` and
    /// `chk-sel`; fourteen until `chk-none`; fifteen until `chk-nosearch`;
    /// sixteen until `chk-sending`, built by the eighteenth notes' piece 3.)*
    #[test]
    fn the_eighteen_states_are_the_drawings_own() {
        let drawing = the_mockup();
        let list = drawing
            .split("<select id=\"state\"")
            .nth(1)
            .and_then(|rest| rest.split("</select>").next())
            .expect("the drawing has a state list");
        let mut drawn: Vec<&str> = Vec::new();
        for (at, _) in list.match_indices("<option value=\"chk-") {
            let rest = &list[at + "<option value=\"".len()..];
            drawn.push(&rest[..rest.find('"').expect("the value closes")]);
        }
        let mut ours = THE_STATES_THIS_PIECE_DRAWS.to_vec();
        drawn.sort_unstable();
        ours.sort_unstable();
        assert_eq!(drawn, ours, "the drawing's chk- states and this piece's list have drifted");
    }

    #[test]
    fn the_page_and_this_file_name_the_same_things() {
        let page = the_page();
        for named in [
            "the_dependency_check_panel",
            "the_check_is_started",
            "an_act_on_the_check",
            AN_ACT_ON_THIS_PANEL,
            "THE_DEPENDENCY_CHECK",
        ] {
            assert!(page.contains(named), "the page does not name {named}");
        }
        // Every act the page hands back is one the closed request type takes.
        for act in [
            "check-again", "check-dependencies", "clear-the-ending", "clear-the-statement", "clear-a-rows-failure",
            "choose-a-row", "choose-a-range", "add-or-take-a-row", "stop-the-search", "stop-the-overwrite",
            "locate", "set-aside", "restore", "undo-the-guess", "accept", "use-it",
            "choose-a-folder", "undo-the-shift", "choose-a-search-folder", "add-a-search-folder",
            "try-again", "confirm", "upload", "admin-access", "replace",
        ] {
            let parsed: Result<TheAct, serde::de::value::Error> = serde::Deserialize::deserialize(serde::de::value::StrDeserializer::<serde::de::value::Error>::new(act));
            assert!(parsed.is_ok(), "{act} is not a closed act");
        }
    }

    /// **The upload control is this panel's act and goes to Transfers** —
    /// `Design/Dependency Upload & Remap`: *"its uploads run in Transfers"*,
    /// and the drawing's `data-go="t"` on this control. The act is taken here,
    /// where the rows and the file's bytes are, since step 10c's dependency
    /// wave piece 6.
    #[test]
    fn the_upload_goes_to_transfers() {
        let page = the_page();
        assert!(page.contains("import { aTransferIsAskedFor } from \"./transfers.js\";"));
        assert!(page.contains("const UPLOAD_THE_FOUND = \"upload-the-found\";"));
        assert!(page.contains(
            "theActIsTaken(server, \"upload\").finally(() => aTransferIsAskedFor(UPLOAD_THE_FOUND, server))"
        ));
    }

    /// **Every word is in `mod words`**, and no composition writes one.
    #[test]
    fn every_word_is_in_one_place() {
        let source = the_working_half();
        let after = source.split("mod words {").nth(1).expect("the words are a module");
        let after = after.split("\n}").nth(1).expect("the words module closes");
        let mut faults = Vec::new();
        for opener in ["Prose::new(\"", "plain(\"", "then(\"", "with(\"", "after("] {
            for (at, _) in after.match_indices(opener) {
                let rest = &after[at + opener.len()..];
                let literal = rest.split('"').next().unwrap_or_default();
                if opener.ends_with('"') && literal.chars().any(|c| c.is_ascii_alphabetic()) {
                    faults.push(literal.to_string());
                }
            }
        }
        assert!(faults.is_empty(), "a word is written outside `mod words`: {faults:#?}");
    }

    /// **Asking for a check drives one** — step 10c's dependency wave piece
    /// 5. *(Until then this test asserted that the press answered the
    /// not-built sentence and recorded nothing: the gap, as the page composed
    /// it.)* The command gathers what a sitting starts from and hands it to
    /// `crate::checking_a_file`, and **the run's memory records nothing before
    /// a reading lands**: no ending, no rows, never an empty list.
    #[test]
    fn asking_for_a_check_drives_one_and_nothing_is_drawn_before_a_reading_lands() {
        let mut held = WhatTheRunHolds::default();
        assert!(the_act_is_taken(&mut held, SERVER, FILE, TheAct::CheckAgain, None).is_none());
        assert!(held.endings.is_empty() && held.sittings.is_empty());
        let panel = the_panel(&held, SERVER, FILE, None);
        assert!(panel.the_ending.is_none());
        assert!(panel.nothing_was_read.is_some());
        assert!(panel.the_table.is_none() && panel.the_foot.is_none() && panel.the_tools.is_none());
        let working = the_working_half();
        let begun = working
            .split("fn the_check_is_begun<")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the gathering");
        assert!(begun.contains("crate::checking_a_file::the_check_runs("), "the press drives no sitting");
        assert!(!working.contains("A_CONNECTION_IS_NOT_BUILT)\n    }"), "the press answers the not-built sentence");
        let started = working
            .split("pub fn the_check_is_started<")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the command");
        assert!(started.contains("the_check_is_begun("));
    }

    /// **Where the run holds no check of the file — after a restart, or the
    /// strip pressed on a file never checked since Smoozy opened — there is no
    /// folders line and no tools bar: *File not read*, and under it one
    /// *Check dependencies*, which starts a check; composing it starts
    /// nothing.** *Check again* stays for every state where a check ran.
    /// `Design/Dependency Upload & Remap § The search folders on this panel`
    /// and `Design/Transfers`, both 2026-10-18; the drawing's `chk-none`. The
    /// Creator's thirteenth notes, note 2: *"it's only "again" when it has
    /// been done before"*. *(Until then this test asserted the folders line
    /// with nothing searched and *Check again* on it.)*
    #[test]
    fn with_no_check_held_there_is_no_folders_line_and_check_dependencies_starts_one() {
        let held = WhatTheRunHolds::default();
        let panel = the_panel_knowing(
            &held,
            SERVER,
            FILE,
            None,
            &WhatElseIsKnown { the_list_in_settings: Some(vec![b"/Users/alex/Work".to_vec()]), uploading: None, on_its_way: Vec::new(), a_send_is_being_counted: false },
        );
        assert_eq!(panel.nothing_was_read, Some(Prose::new(words::NOTHING_WAS_READ)));
        assert!(panel.the_statements.is_empty(), "a folders line where nothing was searched");
        assert!(panel.the_tools.is_none(), "the Search folders… bar where no check is held");
        assert!(panel.the_reading_line.is_empty() && panel.the_summary.is_none(), "a version line before anything was read");
        assert!(panel.the_table.is_none() && panel.the_foot.is_none() && panel.the_ending.is_none());
        let button = panel.the_check_to_start.as_ref().expect("Check dependencies under the sentence");
        assert_eq!(button.label.as_str().replace('\u{a0}', " "), "Check dependencies");
        assert!(!button.unavailable);
        // **It starts the check**: the act the page hands back is the closed
        // request type's `CheckDependencies`, and the dispatch begins a
        // sitting for it exactly as it does for *Check again*.
        let parsed: TheAct = serde::Deserialize::deserialize(serde::de::value::StrDeserializer::<
            serde::de::value::Error,
        >::new(button.act.as_str()))
        .expect("a closed act");
        assert_eq!(parsed, TheAct::CheckDependencies);
        let dispatch = the_working_half()
            .split("let decision = match act {")
            .nth(1)
            .and_then(|rest| rest.split("\n    };").next())
            .expect("the dispatch");
        assert!(
            dispatch.contains("TheAct::CheckAgain | TheAct::CheckDependencies => return the_check_is_begun(app, server, file, bytes),"),
            "Check dependencies does not begin a check"
        );
        // **Composing it started nothing.**
        assert!(held.checks_running.is_empty() && held.sittings.is_empty());

        // **Where a check ran, *Check again* and no *Check dependencies***: a
        // complete sitting keeps its folders line, and an ending keeps the
        // failure shape's own *Check again*.
        let ran = a_run(a_sitting(vec![a_row(b"//a/on.png", Kind::Image, RowState::AlreadyOnTheMachine)]));
        let panel = the_panel(&ran, SERVER, FILE, None);
        assert!(panel.the_check_to_start.is_none(), "Check dependencies over a check that ran");
        assert_eq!(the_folders_line_of(&panel).control.as_ref().map(|c| c.act.as_str()), Some("check-again"));
        let mut ended = WhatTheRunHolds::default();
        ended.endings.insert(keyed(SERVER, FILE), AnEnding { ending: Ending::TheMachineCouldNotBeReached, read_with: None, the_setup_stands_stopped: false });
        let panel = the_panel(&ended, SERVER, FILE, None);
        assert!(panel.the_check_to_start.is_none(), "Check dependencies beside an ending");
        assert!(panel.the_ending.expect("the ending").controls.iter().any(|c| c.act.as_str() == "check-again"));

        // The words are Design's own, and the drawing draws the state.
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains(&format!("***{}***", words::CHECK_DEPENDENCIES)));
        let drawing = the_mockup();
        assert!(drawing.contains("<option value=\"chk-none\">"));
        assert!(drawing.contains(&format!("<button type=\"button\" class=\"btn\" data-go=\"chk-reading\">{}</button>", words::CHECK_DEPENDENCIES)));
    }

    // ── The Creator's notes 8, 6 and 7 on the test copy ─────────────────────

    /// **While the file itself is being read: *Reading the file…* on the
    /// header's own line, and no table, no tools, no foot and nothing else
    /// under the header — over an earlier sitting's rows and an earlier
    /// ending alike**, since the fresh sitting carries neither (`Dependency
    /// Upload & Remap` G17). The drawing's `chk-reading`. Once the mark is
    /// off, the rows. The Creator's note 8 on the test copy; the map's hole 4.
    #[test]
    fn a_check_reading_its_own_file_says_so_over_whatever_was_held() {
        let key = keyed(SERVER, FILE);
        let reading = |held: &WhatTheRunHolds| {
            let panel = the_panel_knowing(
                held,
                SERVER,
                FILE,
                None,
                &WhatElseIsKnown { the_list_in_settings: Some(vec![b"/Users/alex/Work".to_vec()]), uploading: None, on_its_way: Vec::new(), a_send_is_being_counted: false },
            );
            assert_eq!(panel.still_reading, Some(Words::plain(words::READING_THE_FILE)));
            assert!(panel.the_reading_line.is_empty(), "a reading line before anything was read");
            assert!(panel.the_table.is_none(), "a table while the file itself is read");
            assert!(panel.the_tools.is_none() && panel.the_foot.is_none());
            assert!(panel.the_statements.is_empty() && panel.the_once.is_none());
            assert!(panel.the_selection.is_none() && panel.the_shift.is_none() && panel.the_overwrite.is_none());
            assert!(panel.the_ending.is_none() && panel.nothing_was_read.is_none() && panel.instead.is_none());
            assert!(panel.the_strip_reading.is_empty(), "the strip counts a file not yet read");
        };
        // Nothing held before.
        let mut held = WhatTheRunHolds::default();
        held.files_being_read.insert(key.clone());
        reading(&held);
        // An earlier sitting's rows, an earlier ending and an overwrite asked.
        let mut held = a_run(a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]));
        held.endings.insert(key.clone(), AnEnding { ending: Ending::TheMachineCouldNotBeReached, read_with: None, the_setup_stands_stopped: false });
        held.files_being_read.insert(key.clone());
        reading(&held);
        // **The mark off: the rows**, as before.
        held.endings.clear();
        held.files_being_read.clear();
        let panel = the_panel(&held, SERVER, FILE, None);
        assert!(panel.still_reading.is_none());
        assert_eq!(panel.the_table.expect("the rows").groups.iter().map(|group| group.rows.len()).sum::<usize>(), 1);
        // **Another file's mark is not this one's.**
        held.files_being_read.insert(keyed(SERVER, "/Users/alex/other.blend"));
        assert!(the_panel(&held, SERVER, FILE, None).the_table.is_some());
        // The words are Design's own, and the drawing draws the state.
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains(&format!("*{}*", words::READING_THE_FILE)));
        assert!(the_mockup().contains("<option value=\"chk-reading\">"));
    }

    /// **A press marks the file read at once, so the panel asked for next
    /// never says *nothing was read* over a check that is starting — unless a
    /// check of it is already running, which owns its own mark.** The command
    /// marks it before the sitting's thread starts and takes it off on each
    /// way out that never reaches a sitting (no Blender resolved, the server
    /// not asked, no runtime); `crate::checking_a_file` owns it from the
    /// sitting's start.
    #[test]
    fn a_press_marks_the_file_read_unless_a_check_of_it_is_running() {
        let key = keyed(SERVER, FILE);
        let mut held = WhatTheRunHolds::default();
        assert!(the_file_is_about_to_be_read(&mut held, &key));
        assert!(held.files_being_read.contains(&key));
        assert!(!the_file_is_about_to_be_read(&mut held, &key), "a mark already there is not this press's");
        let mut held = WhatTheRunHolds::default();
        held.checks_running.insert(key.clone(), false);
        assert!(!the_file_is_about_to_be_read(&mut held, &key));
        assert!(held.files_being_read.is_empty(), "a press joining a running check marked the file");
        let begun = the_working_half()
            .split("fn the_check_is_begun<")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the gathering");
        let marked = begun.find("the_file_is_about_to_be_read(&mut held, &key)").expect("the press marks nothing");
        assert!(marked < begun.find("std::thread::spawn").expect("the thread"), "marked after the sitting's thread");
        assert_eq!(begun.matches("no_longer_read();").count(), 3, "a way out that never reaches a sitting keeps its mark");
    }

    /// **The box lists Settings' search folders first, full paths, in
    /// Settings' order, with *Change them in Settings*; none where the list
    /// would not read.** The Creator's note 7 on the test copy.
    #[test]
    fn the_folder_box_lists_settings_folders_in_settings_order() {
        let held = a_run(a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]));
        let list = vec![b"/Volumes/Archive/2023".to_vec(), b"/Users/alex/Work".to_vec(), b"/Users/alex/Old".to_vec()];
        let panel = the_panel_knowing(
            &held,
            SERVER,
            FILE,
            None,
            &WhatElseIsKnown { the_list_in_settings: Some(list), uploading: None, on_its_way: Vec::new(), a_send_is_being_counted: false },
        );
        let shown: Vec<&str> = panel.the_folder_box.the_folders.iter().map(Verbatim::as_str).collect();
        assert_eq!(shown, vec!["/Volumes/Archive/2023", "/Users/alex/Work", "/Users/alex/Old"]);
        assert_eq!(panel.the_folder_box.your_search_folders, Prose::new(words::YOUR_SEARCH_FOLDERS));
        assert_eq!(panel.the_folder_box.change_them_in_settings, Prose::new(words::CHANGE_THEM_IN_SETTINGS));
        assert!(the_panel(&held, SERVER, FILE, None).the_folder_box.the_folders.is_empty());
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains(&format!("*{}*", words::CHANGE_THEM_IN_SETTINGS)));
        let drawing = the_mockup();
        for drawn in [words::YOUR_SEARCH_FOLDERS, words::CHANGE_THEM_IN_SETTINGS] {
            assert!(drawing.contains(&format!(">{drawn}<")), "the drawing's box does not say {drawn}");
        }
    }

    /// **Transfers' *Open the check* shows the check and starts nothing** —
    /// `Design/Transfers`: *"The way back goes to the thing as it stands, and
    /// starts nothing"*; `Dependency Upload & Remap` G17. `web/transfers.js`
    /// and this panel's page run together under Node, the panel answered with
    /// what this file composes where nothing is held (after a restart): *nothing
    /// was read*, and under it, inside the drawing's `.cnone`, ***Check
    /// dependencies*** as the `.btn`, with no folders line and no *Check
    /// again*; **pressing it starts the check** (the Creator's thirteenth
    /// notes, note 2 — until then *Check again* sat on an empty folders line).
    /// Until the Creator's note 8 on the test copy the press started a whole
    /// new check. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn transfers_open_the_check_shows_the_check_and_starts_nothing() {
        const THE_BLEND: &str = "/home/ubuntu/kitchen_v14.blend";
        let nothing_held = the_panel(&WhatTheRunHolds::default(), SERVER, THE_BLEND, None);
        let answers = serde_json::json!({ "panel": nothing_held, "server": SERVER, "the_blend": THE_BLEND });
        let folder = std::env::temp_dir().join(format!("smoozy-check-way-back-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["transfers.js", "the-dependency-check.js"] {
            fs::copy(web.join(page), folder.join(page)).expect("the page");
        }
        for (name, text) in THE_WAY_BACKS_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so Transfers' way back was not run");
            return;
        };
        assert!(ran.status.success(), "the pages did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        assert_eq!(
            got,
            serde_json::json!({
                "went": ["machineview", SERVER],
                "opened": [[SERVER, "chk"]],
                "invoked": [["the_dependency_check_panel", THE_BLEND]],
                "nothingWasRead": Prose::new(words::NOTHING_WAS_READ).as_str(),
                "checkAgain": null,
                "foldersLine": false,
                "checkDependencies": {
                    "label": Prose::new(words::CHECK_DEPENDENCIES).as_str(),
                    "looks": "btn",
                    "inside": "cnone",
                    "after": "cempty"
                },
                "pressed": [["an_act_on_the_check", THE_BLEND, "check-dependencies"]]
            })
        );
    }

    /// What the two pages import beside each other, standing in, and a
    /// document with what both draw into.
    const THE_WAY_BACKS_STAND_INS: [(&str, &str); 12] = [
        ("package.json", r#"{ "type": "module" }"#),
        ("files.js", "export function theBrowserIsOpenedAt() {}"),
        ("event-route.js", "export function whenTold() {}"),
        (
            "the-rail.js",
            "export const TRANSFERS = 'transfersview'; export const A_SERVER = 'machineview'; \
             export function aScreenIsRegistered(screen) { (globalThis.screens ||= new Map()).set(screen.place, screen); } \
             export function theCurrentPlaceIs(place, server) { globalThis.went = [place, server]; } \
             export function theRowsFigureIs() {}",
        ),
        (
            "a-server.js",
            "export const THE_DEPENDENCY_CHECK = 'chk'; export const FILES = 'files'; export const SET_UP = 'setupview'; export const panels = []; \
             export function aPanelIsRegistered(panel) { panels.push(panel); } \
             export function thePanelIsOpenedOver(server, panel) { (globalThis.opened ||= []).push([server, panel]); } \
             export function theStripReads() {} export function theCurrentTabIs() {} \
             export function thePanelCollapsesAsItsChevronDoes() {}",
        ),
        (
            "the-failure-shape.js",
            "export const A_COLLAPSED_ROW = 'c'; export const EXPANDED = 'e'; \
             export function theFailureShape() { return document.createElement('span'); }",
        ),
        (
            "the-underway-shape.js",
            "export function theMarkOnARow() { return document.createElement('span'); } \
             export function theCountdown() { return document.createElement('span'); }",
        ),
        ("the-machines-own-words.js", "export function theMachinesOwnWordsFor() {}"),
        (
            "settings.js",
            "export const TRANSFERS_SECTION = 's-tr'; export const SEARCH_FOLDERS_SECTION = 's-fo'; \
             export function theSettingsSectionIsOpened() {}",
        ),
        ("the-history.js", "export function theHistoryEntryIsOpened() {}"),
        ("the-value-drawn.js", "export function theValueIsDrawnInto(node, value) { node.textContent = value; } \
             export function theValueDrawn(value) { const node = document.createElement('span'); node.textContent = value; return node; }"),
        (
            "harness.js",
            r#"import { readFileSync } from "node:fs";
function anElement(tag) {
  const names = new Set(); const listeners = new Map();
  const e = { tagName: tag.toUpperCase(), children: [], parentNode: null, dataset: {}, hidden: false, text: "", type: "", id: "", title: "",
    classList: { add: (...m) => m.forEach((o) => names.add(o)), remove: (...m) => m.forEach((o) => names.delete(o)), contains: (o) => names.has(o), toggle() {} },
    set className(v) { names.clear(); String(v).split(" ").filter(Boolean).forEach((o) => names.add(o)); }, get className() { return [...names].join(" "); },
    set textContent(v) { e.children = []; e.text = v == null ? "" : String(v); }, get textContent() { return e.text + e.children.map((o) => o.textContent || o.text || "").join(""); },
    appendChild(c) { c.parentNode = e; e.children.push(c); return c; }, removeChild(c) { e.children = e.children.filter((o) => o !== c); return c; },
    replaceChildren(...f) { e.children = []; f.forEach((o) => e.appendChild(o)); }, remove() {}, setAttribute() {},
    addEventListener(k, h) { (listeners.get(k) || listeners.set(k, []).get(k)).push(h); },
    click() { for (const h of listeners.get("click") || []) h({ target: e, preventDefault() {} }); }, focus() {},
    contains(o) { for (let at = o; at; at = at.parentNode) if (at === e) return true; return false; },
    get parentElement() { return e.parentNode; }, get childElementCount() { return e.children.filter((o) => o.tagName).length; },
    querySelector() { return null; }, querySelectorAll() { return []; } };
  return e;
}
const every = (e, into = []) => { into.push(e); for (const c of e.children || []) every(c, into); return into; };
globalThis.document = { createElement: anElement, createTextNode: (text) => ({ text, children: [] }), addEventListener() {}, body: anElement("body"), activeElement: null };
const answers = JSON.parse(readFileSync("answers.json", "utf8"));
const words = (s) => ({ pieces: [{ said: s, value: null }] });
const screen = { heading: "Transfers", speeds: null, caps: words(""), settings: "Settings", show: "Show", every_server: "All", servers: [], showing: null, could_not_read: null, nothing_here: null, foot: words(""), the_rails_figure: "",
  requests: [{ id: "op-1", collapsed: true, tag: "Upload", tag_class: "up", arrow: "↑", name: words("kitchen_v14.blend"), count: words("12 of 12"), where_it_goes: words(""), badge: null, bar: null, lines: [], files: [],
    control: { label: "Open dependency check", act: "open-the-check" }, server: answers.server, the_blend: answers.the_blend }] };
let invoked = [];
globalThis.window = { __TAURI__: { core: { invoke(command, args) {
  invoked.push(args && args.file ? (args.act ? [command, String.fromCharCode(...args.file), args.act] : [command, String.fromCharCode(...args.file)]) : [command]);
  if (command === "the_transfers_screen") return Promise.resolve(screen);
  if (command === "the_dependency_check_panel") return Promise.resolve(answers.panel);
  return Promise.resolve(null);
} } } };
await import("./transfers.js");
const { panels } = await import("./a-server.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
const sheet = anElement("div"); const head = sheet.appendChild(anElement("div")); const body = sheet.appendChild(anElement("div"));
const transfers = globalThis.screens.get("transfersview");
const host = anElement("div"); transfers.itIsDrawn(host); transfers.itIsCurrentNow();
await settle();
const control = every(host).find((one) => one.tagName === "BUTTON" && one.textContent === "Open dependency check");
invoked = [];
control.click();
await settle();
panels[0].itIsDrawn({ head, body }, answers.server);
await settle();
const theEmpty = every(body).find((one) => one.classList && one.classList.contains("cempty"));
const again = every(body).find((one) => one.dataset && one.dataset.box === "check-again");
const folders = every(body).some((one) => one.dataset && one.dataset.statement);
const start = every(body).find((one) => one.dataset && one.dataset.box === "check-dependencies");
const wayBack = invoked.filter((one, at) => at === invoked.findIndex((o) => o[0] === one[0]));
let pressed = [];
if (start) {
  invoked = [];
  start.click();
  await settle();
  pressed = invoked.filter((one) => one[0] === "an_act_on_the_check");
}
const holder = start && start.parentNode;
const before = holder && holder.children[holder.children.indexOf(start) - 1];
console.log(JSON.stringify({
  went: globalThis.went, opened: globalThis.opened || [], invoked: wayBack,
  nothingWasRead: theEmpty ? theEmpty.textContent : null,
  checkAgain: again ? { label: again.textContent, looks: again.className } : null,
  foldersLine: folders,
  checkDependencies: start ? { label: start.textContent, looks: start.className, inside: holder ? holder.className : null, after: before ? before.className : null } : null,
  pressed,
}));
"#,
        ),
    ];

    /// **`chk-unsrv` stays drawable, from a real attempt's ending, and the
    /// value beside it is the server's name — never its identifier** (must-dos
    /// 5 and 19).
    #[test]
    fn a_real_attempt_that_could_not_reach_the_server_names_it_by_what_the_user_calls_it() {
        let mut held = WhatTheRunHolds::default();
        let the_diary = crate::surface::render_settings::TheDiaryInHand {
            diary: None,
            run: None,
            the_servers_name: "the big box",
        };
        the_check_gave_nothing(&mut held, SERVER, FILE, Ending::TheMachineCouldNotBeReached, None, &the_diary);
        let panel = the_panel(&held, SERVER, FILE, Some("the big box"));
        let ending = panel.the_ending.expect("the check ended");
        assert_eq!(ending.which.as_str(), "the-machine");
        assert!(ending.the_second_sentence.is_none());
        assert_eq!(ending.values.len(), 1);
        assert_eq!(ending.values[0].value.as_str(), "the big box");
        assert!(!format!("{ending:?}").contains(SERVER), "the local identifier is drawn as the name");
        // With no name read, no value is drawn at all.
        let unnamed = the_panel(&held, SERVER, FILE, None).the_ending.expect("the check ended");
        assert!(unnamed.values.is_empty());
    }

    /// **M32's second limb, from this panel: each real ending is handed to the
    /// diary once per request, and asking for a check that cannot run hands it
    /// nothing** (the Verdict's upstream must-do).
    #[test]
    fn a_checks_ending_reaches_the_diary_once_per_request() {
        let mut area = std::env::temp_dir();
        area.push(format!("smoozy-check-diary-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&area);
        let diary = crate::diary_service::DiaryService::open_at(area.clone()).expect("a diary");
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let the_diary = crate::surface::render_settings::TheDiaryInHand {
            diary: Some(&diary),
            run: Some(&run),
            the_servers_name: "the big box",
        };
        let mut held = WhatTheRunHolds::default();
        for _ in 0..2 {
            assert_eq!(
                the_check_gave_nothing(
                    &mut held,
                    SERVER,
                    FILE,
                    Ending::TheFileCouldNotBeOpenedByThisVersion,
                    Some(a_blender()),
                    &the_diary
                ),
                crate::presentation::the_two_hooks::TheDiaryWasHanded::TheSentence
            );
        }
        let _ = the_act_is_taken(&mut held, SERVER, FILE, TheAct::CheckAgain, None);
        let mut kept = String::new();
        for entry in std::fs::read_dir(&area).expect("the area").flatten() {
            kept.push_str(&std::fs::read_to_string(entry.path()).unwrap_or_default());
        }
        assert_eq!(kept.matches(words::THIS_BLENDER_COULD_NOT_OPEN).count(), 2);
        assert!(kept.contains("the big box") && kept.contains("kitchen_v14.blend"));
        assert!(!kept.contains(crate::surface::files::A_CONNECTION_IS_NOT_BUILT), "a cannot-do-this-yet answer was captured");
        let _ = std::fs::remove_dir_all(&area);
    }

    #[test]
    fn the_four_endings_each_say_something_different_and_only_one_carries_a_second_sentence() {
        let endings = [
            Ending::TheMachineCouldNotBeReached,
            Ending::TheFileCouldNotBeOpenedByThisVersion,
            Ending::TheReadRanAndSomethingInItFailed,
            Ending::SmoozyStoppedIt,
        ];
        let mut said = BTreeSet::new();
        let mut seconds = 0;
        for ending in endings {
            let failure = the_ending(&ending, Some("the big box"), Some(&a_blender())).expect("an ending");
            said.insert(failure.sentence.as_str().to_string());
            if failure.the_second_sentence.is_some() {
                seconds += 1;
                assert_eq!(failure.which.as_str(), "the-version");
                assert_eq!(failure.values[0].value.as_str(), "5.1.0");
            }
            let first = &failure.controls[0];
            assert_eq!(first.is.as_str(), "retry");
            assert_eq!(failure.controls.last().expect("a cross").is.as_str(), "cross");
            let raw = failure.controls.iter().any(|one| one.is.as_str() == "raw-text");
            assert_eq!(raw, failure.which.as_str() == "the-read");
        }
        assert_eq!(said.len(), 4);
        assert_eq!(seconds, 1);
    }

    // ── The dependency check's fresh look, 2026-10-18 ───────────────────────
    //
    // `Audits/Build/Step 10c/The dependency check, a fresh look — the
    // proposal.md`, approved by the Creator with both decisions yes, and the
    // plan and drawing passes after it. Pieces 1 to 3.

    /// Every row the table draws, group by group.
    fn every_row(table: &TheTable) -> Vec<&ARow> {
        table.groups.iter().flat_map(|group| group.rows.iter()).collect()
    }

    /// **A sentence as the page draws it** — each piece after the one before
    /// with a space, unless it joins.
    fn reads(words: &Words) -> String {
        let mut out = String::new();
        for (at, piece) in words.pieces.iter().enumerate() {
            if at > 0 && !piece.joins {
                out.push(' ');
            }
            if let Some(said) = &piece.said {
                out.push_str(said.as_str());
            }
            if let Some(value) = &piece.value {
                out.push_str(value.as_str());
            }
        }
        out.replace('\u{a0}', " ")
    }

    /// **The summary sentence as the page draws it.**
    fn the_summary_reads(panel: &TheCheck) -> String {
        let mut out = String::new();
        for (at, part) in panel.the_summary.as_ref().expect("a summary sentence").parts.iter().enumerate() {
            if at > 0 && !part.joins {
                out.push(' ');
            }
            out.push_str(&reads(&part.said));
        }
        out
    }

    /// **The strip as `web/a-server.js` draws it**: each part's lead, values
    /// and rest, the parts side by side with a space.
    fn the_strip_reads(panel: &TheCheck) -> String {
        let parts: Vec<String> = panel
            .the_strip_reading
            .iter()
            .map(|part| {
                let mut out = part.lead.as_str().to_string();
                for value in &part.values {
                    if !out.is_empty() {
                        out.push(' ');
                    }
                    out.push_str(value.as_str());
                }
                if let Some(rest) = &part.rest {
                    if !part.the_rest_joins && !out.is_empty() {
                        out.push(' ');
                    }
                    out.push_str(rest.as_str());
                }
                out
            })
            .collect();
        parts.join(" ").replace('\u{a0}', " ")
    }

    fn a_row_found_at(reference: &[u8], kind: Kind, found: &[u8], how: HowResolved) -> ACheckedRow {
        let mut one = a_row(reference, kind, RowState::FoundOnYourComputer);
        one.answers[0].resolution = Some(Resolution {
            to: ResolvedTo::AWholeLocation(StoredLocation::as_the_system_returned_it(found)),
            how,
        });
        one
    }

    const MB: u64 = 1024 * 1024;

    /// **`§ How the rows are grouped and sorted`, since 2026-10-18: Failed,
    /// Missing, Uploaded before not verified, To upload, Already on the
    /// server (folded, its size on the heading), Ignored (folded)**, each in
    /// its colour, and a linked `.blend` at the head of its group. *(Until
    /// then: Not found, Found on your computer, Uploaded before not
    /// confirmed, Already on the server open, Set aside folded.)*
    #[test]
    fn the_groups_are_designs_new_order_with_two_folded_and_a_linked_blend_heads_its_group() {
        let held = a_run(a_sitting(vec![
            a_row(b"//z/aside.png", Kind::Image, RowState::SetAside),
            a_row(b"//a/on.png", Kind::Image, RowState::AlreadyOnTheMachine),
            a_row(b"//a/missing.png", Kind::Image, RowState::NotFound),
            a_row(b"//z/lib.blend", Kind::Library, RowState::NotFound),
            a_row(b"//a/sent.png", Kind::Image, RowState::CouldNotBeSent),
            a_row(b"//a/found.png", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"//a/before.png", Kind::Image, RowState::SentBeforeNotConfirmed),
        ]));
        let table = the_panel(&held, SERVER, FILE, None).the_table.expect("a table");
        let groups: Vec<(String, String, String, bool)> = table
            .groups
            .iter()
            .map(|group| {
                (
                    group.which.as_str().to_string(),
                    group.heading.as_str().replace('\u{a0}', " "),
                    group.colour.as_str().to_string(),
                    group.folded,
                )
            })
            .collect();
        let expected = [
            ("failed", "Failed", "bad", false),
            ("not-found", "Missing", "bad", false),
            ("uploaded-before", "Uploaded before, not verified", "busy", false),
            ("found", "To upload", "", false),
            ("already-there", "Already on the server", "ok", true),
            ("ignored", "Ignored", "idle", true),
        ];
        assert_eq!(
            groups,
            expected
                .iter()
                .map(|(which, heading, colour, folded)| (which.to_string(), heading.to_string(), colour.to_string(), *folded))
                .collect::<Vec<_>>()
        );
        assert_eq!(table.groups[1].rows[0].name.as_str(), "lib.blend");
        assert!(table.groups[1].rows[0].a_linked_blend);
        // **A heading's count is of rows, and Already on the server says its
        // size beside it.**
        assert_eq!(reads(&table.groups[0].how_many), "1");
        assert_eq!(reads(&table.groups[4].how_many), "1 · 1.0 MB");
        // **An ignored row is not in the list a selection is made from.**
        for group in &table.groups {
            for row in &group.rows {
                assert_eq!(row.choosable, group.which.as_str() != "ignored", "{}", group.which.as_str());
            }
        }
        // **The page folds what the application folds**, in the drawing's
        // `details.cside`, its colour on it.
        let page = the_page();
        for drawn in ["group.folded", "\"cside\"", "folded.classList.add(group.colour)", "held.foldedOpen[group.which]"] {
            assert!(page.contains(drawn), "the page does not draw {drawn}");
        }
        // The names are Design's own.
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains("*Failed*, *Missing*, *Uploaded before, not verified*, *To upload*, *Already on the server*, *Ignored*"));
        assert!(design.contains("*● Already on the server 1 · 128 MB*"));
    }

    /// **`§ A row`: the name, then its kind; one location — the folder it
    /// uploads from, or *The file says:* and the folder as the `.blend` writes
    /// it — and the file's own text on hover.** A set says how many files. No
    /// column headings and no checkbox. *(Until 2026-10-18 the row named the
    /// location twice, under *What the file records* and *What Smoozy
    /// found*.)*
    #[test]
    fn a_row_names_the_file_once_with_its_kind_one_location_and_the_files_own_text_on_hover() {
        let mut sequence = a_row(b"//render/frame_####.png", Kind::Image, RowState::FoundOnYourComputer);
        sequence.members = Some(240);
        sequence.answers[0].resolution = Some(Resolution {
            to: ResolvedTo::AFolderForASetRow(StoredLocation::as_the_system_returned_it(b"/Users/alex/render/")),
            how: HowResolved::TheRecordedLocationItself,
        });
        let mut cache = a_row(b"//caches/smoke/", Kind::FluidCacheFolder, RowState::FoundOnYourComputer);
        cache.members = Some(340);
        cache.answers[0].resolution = Some(Resolution {
            to: ResolvedTo::AFolderForASetRow(StoredLocation::as_the_system_returned_it(b"/Users/alex/caches/smoke/")),
            how: HowResolved::TheRecordedLocationItself,
        });
        let held = a_run(a_sitting(vec![
            a_row_found_at(
                b"//../Documents/Blender/HDRi/studio.exr",
                Kind::Image,
                b"/Users/alex/Documents/Blender/HDRi/studio.exr",
                HowResolved::TheRecordedLocationItself,
            ),
            a_row(b"//lib/props.blend", Kind::Library, RowState::NotFound),
            sequence,
            cache,
            a_row(b"C:\\work\\tex\\wood.png", Kind::Image, RowState::NotFound),
        ]));
        let table = the_panel(&held, SERVER, FILE, None).the_table.expect("a table");
        let rows = every_row(&table);
        let mut drawn: Vec<(&str, String, String, &str)> = rows
            .iter()
            .map(|row| (row.name.as_str(), reads(&row.kind), reads(&row.location), row.the_file_says.as_str()))
            .collect();
        drawn.sort();
        assert_eq!(
            drawn,
            vec![
                ("frame_####.png", "Sequence · 240 files".to_string(), "/Users/alex/render".to_string(), "//render/frame_####.png"),
                ("props.blend", "Linked .blend".to_string(), "The file says: //lib/".to_string(), "//lib/props.blend"),
                ("smoke", "Cache · 340 files".to_string(), "/Users/alex/caches/smoke".to_string(), "//caches/smoke/"),
                (
                    "studio.exr",
                    "Image".to_string(),
                    "/Users/alex/Documents/Blender/HDRi".to_string(),
                    "//../Documents/Blender/HDRi/studio.exr"
                ),
                ("wood.png", "Image".to_string(), "The file says: C:\\work\\tex\\".to_string(), "C:\\work\\tex\\wood.png"),
            ]
        );
        let row = |name: &str| *rows.iter().find(|row| row.name.as_str() == name).expect("the row");
        // **No visible action where Smoozy uploads the file itself; Locate…
        // where it is missing; ⋯ holding the rest.**
        let acts = |row: &ARow| -> (Option<String>, Vec<String>) {
            (
                row.action.as_ref().map(|action| action.act.as_str().to_string()),
                row.more_items.iter().map(|item| item.act.as_str().to_string()).collect(),
            )
        };
        assert_eq!(acts(row("props.blend")), (Some("locate".to_string()), vec!["set-aside".to_string()]));
        assert_eq!(acts(row("studio.exr")), (None, vec!["locate".to_string(), "set-aside".to_string()]));
        // **The table names no column and carries no checkbox.**
        let answer = serde_json::to_value(&table).expect("the table serialises").to_string();
        for gone in ["what_the_file_records", "what_smoozy_found", "select_every_row", "select_this_row", "\"records\"", "\"found\":"] {
            assert!(!answer.contains(gone), "the table still carries {gone}");
        }
        let page = the_page();
        for gone in ["\"rhead\"", "aCheckbox", "\"cb", "\"rec ", "\"got", "\"cnow\"", "\"via\"", "\"cnote\"", "\"fig\""] {
            assert!(!page.contains(gone), "the page still draws {gone}");
        }
        for drawn in ["\"nm mono lib\"", "\"kd\"", "\"loc\"", "where.title = row.the_file_says"] {
            assert!(page.contains(drawn), "the page does not draw {drawn}");
        }
        // **Every kind word is the drawing's or Design's own where they name
        // one.**
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        for named in ["*Sequence · 240 files*", "*Cache · 340 files*", "*Linked .blend*", "*The file says:*"] {
            assert!(design.contains(named), "Design does not say {named}");
        }
    }

    /// **The notes: every one that applies, in `§ A row`'s fixed order** —
    /// a change since the check ran; a failure's own reason; how it was found;
    /// what the row stands for; the scene note — each in its colour.
    #[test]
    fn the_notes_come_in_designs_fixed_order_and_colours() {
        let search = HowResolved::TheSearch {
            from_folder: b"/Volumes/Lib/Textures".to_vec(),
            arbitrary: Some(smoozy_dependencies::record::ArbitraryPick {
                taken: StoredLocation::as_the_system_returned_it(b"/Volumes/Lib/Textures/a/x.png"),
                among: vec![
                    StoredLocation::as_the_system_returned_it(b"/Volumes/Lib/Textures/a/x.png"),
                    StoredLocation::as_the_system_returned_it(b"/Volumes/Lib/Textures/b/x.png"),
                    StoredLocation::as_the_system_returned_it(b"/Volumes/Lib/Textures/c/x.png"),
                ],
            }),
        };
        let mut failed = a_row_found_at(b"//tex/x.png", Kind::Image, b"/Volumes/Lib/Textures/a/x.png", search);
        failed.state = RowState::CouldNotBeSent;
        failed.why_it_did_not_go_up = Some(WhyItDidNotGoUp::TheServerRefusedTheWrite);
        failed.row.references = vec![
            RecordedLocation::as_the_reading_carried_it(b"//tex/x.png"),
            RecordedLocation::as_the_reading_carried_it(b"//TEX/x.png"),
            RecordedLocation::as_the_reading_carried_it(b"//tex/./x.png"),
            RecordedLocation::as_the_reading_carried_it(b"//tex//x.png"),
        ];
        failed.scenes = Attribution::Established(Vec::new());
        let the_file_says = format!(
            "Found in search folder Textures. The file says: {}",
            lossy(the_recorded_folder(failed.row.ordering_key().as_bytes()))
        );
        let mut gone = a_row(b"//tex/gone.png", Kind::Image, RowState::FoundOnYourComputer);
        gone.state = RowState::NotFound;
        let mut ignored_since = a_row(b"//tex/later.png", Kind::Image, RowState::FoundOnYourComputer);
        ignored_since.state = RowState::SetAside;
        let mut restored = a_row(b"//tex/back.png", Kind::Image, RowState::SetAside);
        restored.state = RowState::NotFound;
        let mut cleared = a_row(b"//tex/cleared.png", Kind::Image, RowState::CouldNotBeFixed);
        cleared.why_it_did_not_go_up = Some(WhyItDidNotGoUp::SmoozyCouldNotRemapItsPaths);
        cleared.its_failure_was_cleared = true;
        let held = a_run(a_sitting(vec![failed, gone, ignored_since, restored, cleared]));
        let table = the_panel(&held, SERVER, FILE, None).the_table.expect("a table");
        let notes_of = |name: &str| -> Vec<(String, String, bool)> {
            every_row(&table)
                .into_iter()
                .find(|row| row.name.as_str() == name)
                .expect("the row")
                .notes
                .iter()
                .map(|note| {
                    (
                        note.failure
                            .as_ref()
                            .map_or_else(|| reads(&note.said), |failure| failure.sentence.as_str().replace('\u{a0}', " ")),
                        note.colour.as_str().to_string(),
                        note.failure.is_some(),
                    )
                })
                .collect()
        };
        let note = |said: &str, colour: &str, a_failure: bool| (said.to_string(), colour.to_string(), a_failure);
        assert_eq!(
            notes_of("x.png"),
            vec![
                note("Upload failed: permission denied", "bad", true),
                note(&the_file_says, "", false),
                note("3 matches in Textures; arbitrary pick", "", false),
                note("Referenced 4 times", "", false),
                note("Not used by any scene.", "", false),
            ]
        );
        assert_eq!(notes_of("gone.png"), vec![note("File no longer found", "bad", false)]);
        assert_eq!(notes_of("later.png"), vec![note("Ignored", "idle", false)]);
        assert_eq!(notes_of("back.png"), vec![note("Missing", "bad", false)]);
        assert_eq!(notes_of("cleared.png"), vec![note("Uploaded; remap failed: a path did not update", "bad", false)]);
        // **`Dependency Upload & Remap` M14a: a failed row says which of two.**
        assert_eq!(the_reason(Some(WhyItDidNotGoUp::SmoozyCouldNotOpenItToRemapItsPaths)), "Uploaded; remap failed: Blender could not open it");
        // **The search folder named by its last part carries its path on
        // hover.**
        let x = every_row(&table).into_iter().find(|row| row.name.as_str() == "x.png").expect("the row");
        assert_eq!(x.notes[1].hover.as_ref().map(Verbatim::as_str), Some("/Volumes/Lib/Textures"));
        // **Each failure keeps its own Try again, and What the server said
        // waits behind ⋯.**
        assert_eq!(x.action.as_ref().map(|action| action.act.as_str()), Some("try-again"));
        assert!(x.more_items.iter().any(|item| item.act.as_str() == "what-the-server-said"));
        // The words are Design's own.
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        for said in [
            words::FILE_NO_LONGER_FOUND,
            words::READY_TO_UPLOAD,
            words::UPLOAD_FAILED_NO_PERMISSION,
            words::COULD_NOT_UPDATE_ITS_FILE_PATHS,
            words::YOU_CHOSE_THIS_FILE,
            words::IT_MAY_ADD_MORE_ROWS,
            words::NO_SCENE_USES_IT,
        ] {
            let said = said.trim_end_matches('.');
            assert!(design.contains(&format!("*{said}")), "Design does not say {said}");
        }
        assert!(design.contains("*Referenced 4 times*"));
        assert!(design.contains("*3 matches in Archive; arbitrary pick*"));
    }

    /// **A capitals-only proposal is a note and *Accept* the row's one
    /// action; a rejected guess is a red note with *Use it*; a move and a
    /// hand say how the file was found** — `§ A row`, `§ A guess the user
    /// rejected`. *(Until 2026-10-18: red* Nothing matched *and* Found as …,
    /// which differs only in capitals*.)*
    #[test]
    fn a_proposal_a_rejected_guess_a_move_and_a_hand_each_say_how_and_carry_their_one_action() {
        let mut near = a_row(b"//textures/METallic.png", Kind::Image, RowState::NotFound);
        near.proposal = Some(StoredLocation::as_the_system_returned_it(b"/Users/alex/Archive/textures/metallic.png"));
        let mut turned_down = a_row(b"//textures/wall_plaster.png", Kind::Image, RowState::NotFound);
        turned_down.answers[0].rejected_guess =
            Some(StoredLocation::as_the_system_returned_it(b"/Users/alex/Archive/old/wall_plaster.png"));
        let moved = found_by_a_shift(b"/work/x.png");
        let by_hand = a_row_found_at(b"//tex/y.png", Kind::Image, b"/Users/alex/y.png", HowResolved::TheUsersOwnHand);
        let mut sitting = a_sitting(vec![near, turned_down, moved, by_hand]);
        sitting.the_shift = Some(TheShiftInHand {
            old_root: b"/work/".to_vec(),
            new_root: b"/Users/alex/Archive/".to_vec(),
            key: ShiftKey(1),
        });
        let table = the_panel(&a_run(sitting), SERVER, FILE, None).the_table.expect("a table");
        let row = |name: &str| every_row(&table).into_iter().find(|row| row.name.as_str() == name).expect("the row").clone();

        let near = row("METallic.png");
        assert_eq!(reads(&near.location), "The file says: //textures/");
        assert_eq!(reads(&near.notes[0].said), "Only the case differs: metallic.png");
        assert_eq!(near.notes[0].hover.as_ref().map(Verbatim::as_str), Some("/Users/alex/Archive/textures/metallic.png"));
        assert_eq!(near.action.as_ref().map(|action| action.act.as_str()), Some("accept"));
        assert!(near.more_items.iter().any(|item| item.act.as_str() == "locate"), "Locate… is not behind ⋯");

        let turned_down = row("wall_plaster.png");
        assert_eq!(reads(&turned_down.notes[0].said), "You rejected wall_plaster.png");
        assert_eq!(turned_down.notes[0].colour.as_str(), "bad");
        assert_eq!(turned_down.action.as_ref().map(|action| action.act.as_str()), Some("use-it"));

        let moved = row("x.png");
        assert_eq!(reads(&moved.notes[0].said), "Found by your path change from /work/ to /Users/alex/Archive/");
        assert!(moved.action.is_none());
        assert!(moved.more_items.iter().any(|item| item.act.as_str() == "undo-the-guess"));

        let by_hand = row("y.png");
        assert_eq!(reads(&by_hand.notes[0].said), "You chose this file");
        assert!(!by_hand.more_items.iter().any(|item| item.act.as_str() == "undo-the-guess"), "a hand is not a guess");

        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains("*Only the case differs: metallic.png*"));
        assert!(design.contains("*You rejected METallic.png*"));
        assert!(design.contains("*Found by your path change from /work to /Archive*"));
    }

    /// **The scene note — *Not used by any scene.* — only where
    /// the reading finished for every reference the row stands for and
    /// reached no scene** (`Specs/Dependency Upload & Remap`, *Checking a
    /// file*, Creator-stated 2026-10-18). Not where any reference's walk did
    /// not finish, not where it reached a scene, and **not where the walk
    /// starts from something that can never reach one** — a linked `.blend`,
    /// a scene's own strips and bake folder, a shader node's script — as the
    /// desk's reading of the Creator's file showed (see
    /// `the_walk_starts_where_a_scene_can_be_reached`).
    #[test]
    fn the_scene_note_shows_only_where_the_walk_finished_and_reached_no_scene() {
        let with = |kind: Kind, scenes: Attribution| {
            let mut one = a_row(b"//a/x.png", kind, RowState::FoundOnYourComputer);
            one.scenes = scenes;
            let table = the_panel(&a_run(a_sitting(vec![one])), SERVER, FILE, None).the_table.expect("a table");
            table.groups[0].rows[0].notes.iter().any(|note| reads(&note.said) == words::NO_SCENE_USES_IT)
        };
        let none = || Attribution::Established(Vec::new());
        assert!(with(Kind::Image, none()), "an image no scene uses carries no note");
        assert!(with(Kind::FluidCacheFolder, none()));
        assert!(!with(Kind::Image, Attribution::Established(vec!["Scene.001".to_string()])));
        assert!(!with(Kind::Image, Attribution::NotEstablished), "an unfinished walk said something");
        for kind in [
            Kind::Library,
            Kind::SequencerStripFile,
            Kind::SequencerStripFolder,
            Kind::ProxyFolder,
            Kind::BakeFolder,
            Kind::PhysicsPointCache,
            Kind::OslScript,
            Kind::IesProfile,
            Kind::ImportNodePath,
        ] {
            assert!(!with(kind, none()), "{kind:?}: a walk that cannot reach a scene said none uses it");
        }
        // **And no scene's name ever reaches the screen** — the Creator
        // dropped that line as noise on 2026-10-08 (sixth notes, note 3).
        let table = the_panel(&a_run(a_sitting(vec![a_row(b"//a/x.png", Kind::Image, RowState::FoundOnYourComputer)])), SERVER, FILE, None)
            .the_table
            .expect("a table");
        let answer = serde_json::to_value(&table).expect("it serialises").to_string();
        assert!(!answer.contains("Interior"), "a scene's name reached the screen: {answer}");
        assert!(!answer.contains("\"scenes\""));
    }

    /// **The Creator's own `liquid in a pipe.blend`, as the desk read it on
    /// 2026-10-18** — a copy, one headless Blender 5.1.2, Smoozy's own
    /// reading script (`smoozy_dependencies_machine::check_script`): two
    /// pictures, `studio_white_soft_light_02.exr` reaching `Scene` and
    /// `pipe_00001.png` reaching `Scene.001` through that scene's compositor
    /// (*Compositor Nodes*, its `compositing_node_group`). **Both found where
    /// the file says, so the drawing's `chk-clean`: *Nothing is missing. 2
    /// files to upload, 90 MB.*, two rows under *To upload*, no scene note
    /// on `pipe_00001.png`, and *Upload 2 files (90 MB)*.**
    #[test]
    fn the_creators_pipe_file_reads_nothing_is_missing_and_its_stray_frame_carries_no_scene_note() {
        const THE_PIPE: &str = "/Users/artist/Desktop/liquid in a pipe.blend";
        let mut hdri = a_row_found_at(
            b"//../Documents/Blender/HDRi/studio_white_soft_light_02.exr",
            Kind::Image,
            b"/Users/artist/Documents/Blender/HDRi/studio_white_soft_light_02.exr",
            HowResolved::TheRecordedLocationItself,
        );
        hdri.scenes = Attribution::Established(vec!["Scene".to_string()]);
        hdri.size = Some(89 * MB);
        let mut frame = a_row_found_at(
            b"//../Downloads/pipe_assets/PNG/pipe_00001.png",
            Kind::Image,
            b"/Users/artist/Downloads/pipe_assets/PNG/pipe_00001.png",
            HowResolved::TheRecordedLocationItself,
        );
        frame.scenes = Attribution::Established(vec!["Scene.001".to_string()]);
        frame.size = Some(918 * 1024);
        let mut sitting = a_sitting(vec![hdri, frame]);
        sitting.linked_files_read = 0;
        let mut held = WhatTheRunHolds::default();
        held.sittings.insert(keyed(SERVER, THE_PIPE), sitting);
        let panel = the_panel(&held, SERVER, THE_PIPE, None);

        assert_eq!(panel.the_reading_line.len(), 1, "a file that links nothing says how many it linked");
        assert_eq!(the_summary_reads(&panel), "Nothing is missing. 2 files to upload, 90 MB.");
        assert_eq!(the_strip_reads(&panel), "Nothing is missing. 2 files to upload, 90 MB.");
        let table = panel.the_table.as_ref().expect("a table");
        assert_eq!(table.groups.len(), 1);
        assert_eq!(table.groups[0].heading.as_str().replace('\u{a0}', " "), "To upload");
        let rows = every_row(table);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1].name.as_str(), "pipe_00001.png");
        assert_eq!(reads(&rows[1].location), "/Users/artist/Downloads/pipe_assets/PNG");
        assert!(rows.iter().all(|row| row.notes.is_empty()), "a row found where the file says carries a note");
        assert!(rows.iter().all(|row| row.action.is_none()));
        assert_eq!(reads(&panel.the_foot.as_ref().expect("a foot").upload), "Upload 2 files (90 MB)");
        // **The drawing's `chk-clean` says the same.**
        let drawing = the_mockup();
        assert!(drawing.contains("<p class=\"csum\" data-ck=\"pipe\" hidden>Nothing is missing. <b>2</b> files to upload, <b>90&#160;MB</b>.</p>"));
        assert!(drawing.contains(">Upload 2 files (90&#160;MB)</button>"));
    }

    /// **The drawing's `chk-sending`: a row whose file is already uploading
    /// says *Uploading*, the foot counts only the rest, and the foot is gone
    /// once everything is on its way** — `Design/Dependency Upload & Remap §
    /// The seven states`' and `§ Along the bottom`'s paragraphs of 2026-10-24,
    /// `Dependency Upload & Remap` A9a; the Creator's eighteenth notes, note 6.
    /// Before the eighteenth notes' piece 3 the row read as any other and the
    /// button offered both files again.
    #[test]
    fn a_row_on_its_way_says_uploading_and_the_foot_counts_only_the_rest() {
        use crate::checking_a_file::TheSittingIsOf;
        use smoozy_dependencies::landing::mirror_of;
        use smoozy_dependencies::record::MachineKey;
        use smoozy_job_files::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
        const THE_PIPE: &str = "/Users/artist/Desktop/liquid in a pipe.blend";
        const THE_HDRI: &[u8] = b"/Users/artist/Documents/Blender/HDRi/studio_white_soft_light_02.exr";
        const THE_FRAME: &[u8] = b"/Users/artist/Downloads/pipe_assets/PNG/pipe_00001.png";
        let layout = TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        let mut hdri = a_row_found_at(
            b"//../Documents/Blender/HDRi/studio_white_soft_light_02.exr",
            Kind::Image,
            THE_HDRI,
            HowResolved::TheRecordedLocationItself,
        );
        hdri.scenes = Attribution::Established(vec!["Scene".to_string()]);
        hdri.size = Some(89 * MB);
        let mut frame = a_row_found_at(
            b"//../Downloads/pipe_assets/PNG/pipe_00001.png",
            Kind::Image,
            THE_FRAME,
            HowResolved::TheRecordedLocationItself,
        );
        frame.scenes = Attribution::Established(vec!["Scene.001".to_string()]);
        frame.size = Some(918 * 1024);
        let mut sitting = a_sitting(vec![hdri, frame]);
        sitting.linked_files_read = 0;
        sitting.of = Some(TheSittingIsOf {
            server: SERVER.to_string(),
            file: THE_PIPE.to_string(),
            root: MachineLocation::composed(b"/ephemeral/smoozy/jobs/liquid in a pipe.blend".to_vec()),
            holding: HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(THE_PIPE.as_bytes())),
            with: a_blender(),
            machine: MachineKey::of(
                an_identity(SERVER),
                Identity::issue(IdentityKind::Installation).expect("an installation"),
            ),
            layout: Some(layout.clone()),
        });
        let mut held = WhatTheRunHolds::default();
        held.sittings.insert(keyed(SERVER, THE_PIPE), sitting);
        let the_panel_with = |going: &[&[u8]]| {
            the_panel_knowing(
                &held,
                SERVER,
                THE_PIPE,
                None,
                &WhatElseIsKnown {
                    the_list_in_settings: None,
                    uploading: None,
                    on_its_way: going
                        .iter()
                        .map(|source| {
                            mirror_of(&layout, &StoredLocation::as_the_system_returned_it(source)).expect("fits")
                        })
                        .collect(),
                    a_send_is_being_counted: false,
                },
            )
        };
        let the_notes = |panel: &TheCheck| -> Vec<Vec<(String, String)>> {
            every_row(panel.the_table.as_ref().expect("a table"))
                .iter()
                .map(|row| {
                    row.notes
                        .iter()
                        .map(|note| (reads(&note.said), note.colour.as_str().to_string()))
                        .collect()
                })
                .collect()
        };

        // **Nothing on its way: no note, and the button counts both.**
        let idle = the_panel_with(&[]);
        assert_eq!(the_notes(&idle), vec![vec![], vec![]]);
        assert_eq!(reads(&idle.the_foot.as_ref().expect("a foot").upload), "Upload 2 files (90 MB)");

        // **The HDRI on its way: *Uploading* on its row in `busy`, still under
        // *To upload* with no action, and the button counts the frame alone.**
        let sending = the_panel_with(&[THE_HDRI]);
        assert_eq!(
            the_notes(&sending),
            vec![vec![("Uploading".to_string(), "busy".to_string())], vec![]]
        );
        let table = sending.the_table.as_ref().expect("a table");
        assert_eq!(table.groups.len(), 1);
        assert_eq!(table.groups[0].heading.as_str().replace('\u{a0}', " "), "To upload");
        assert!(every_row(table).iter().all(|row| row.action.is_none()));
        assert_eq!(reads(&sending.the_foot.as_ref().expect("a foot").upload), "Upload 1 file (918 KB)");
        // **The summary sentence still counts both**, since nothing has landed.
        assert_eq!(the_summary_reads(&sending), "Nothing is missing. 2 files to upload, 90 MB.");

        // **A file of the same name going elsewhere is not this row's.**
        let elsewhere = the_panel_with(&[b"/Users/artist/Archive/studio_white_soft_light_02.exr"]);
        assert_eq!(the_notes(&elsewhere), vec![vec![], vec![]]);

        // **Everything on its way: the button is not there.**
        let all = the_panel_with(&[THE_HDRI, THE_FRAME]);
        assert!(all.the_foot.is_none(), "{:?}", all.the_foot);
        assert_eq!(the_notes(&all).iter().filter(|notes| notes.len() == 1).count(), 2);

        // **The drawing's `chk-sending` says the same.**
        let drawing = the_mockup();
        assert!(drawing.contains("<span class=\"note busy\" data-e18=\"chk-sending\" hidden>Uploading</span>"));
        assert!(drawing.contains("data-e18=\"chk-sending\" hidden>Upload 1 file (918&#160;KB)</button>"));
        // **The panel's command reads the sends file by file, each time it is
        // asked**, so coming back from Transfers shows which are going.
        let command = the_working_half()
            .split("pub fn the_dependency_check_panel<")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the command");
        assert!(command.contains("crate::the_found_files_sent::the_files_on_their_way_to(&server)"));
    }

    /// A sitting with every group in it — the drawing's kitchen file, read
    /// to its end or still reading.

    #[test]
    fn film_page_the_warning_counts_the_same_real_files_as_the_panel() {
        let mut rows = vec![
            a_row(b"/textures/missing.png", Kind::Image, RowState::NotFound),
            a_row(b"/textures/missing.png", Kind::Image, RowState::NotFound),
            a_row(b"/textures/unsent.png", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"/textures/unsent.png", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"/textures/failed.png", Kind::Image, RowState::CouldNotBeSent),
            a_row(b"/textures/failed.png", Kind::Image, RowState::CouldNotBeFixed),
            a_row(b"/textures/ignored.png", Kind::Image, RowState::SetAside),
        ];
        rows[1].holding = HoldingFile::OnTheUsersComputer(StoredLocation::as_the_system_returned_it(b"/project/library.blend"));
        let counts = the_warning_counts(&rows, Default::default());
        assert_eq!(counts.not_found, 1);
        assert_eq!(counts.found_but_not_sent, 1);
        assert_eq!(counts.could_not_be_sent, 1);
        assert_eq!(counts.could_not_be_fixed, 0);
        let panel = the_panel(&a_run(a_sitting(rows)), SERVER, FILE, None);
        assert!(the_summary_reads(&panel).contains("1 failed · 1 missing"));
    }

    fn the_kitchen(completion: Option<Completion>) -> ASitting {
        let mut rows = vec![
            a_row(b"//lib/rig.blend", Kind::Library, RowState::CouldNotBeFixed),
            a_row(b"//textures/bark_rough.png", Kind::Image, RowState::CouldNotBeSent),
            a_row(b"//lib/props.blend", Kind::Library, RowState::NotFound),
            a_row(b"//textures/METallic.png", Kind::Image, RowState::NotFound),
            a_row(b"//textures/wall_plaster.png", Kind::Image, RowState::NotFound),
            a_row(b"//fonts/NeueHaas.otf", Kind::Font, RowState::SentBeforeNotConfirmed),
            a_row(b"//caches/smoke/", Kind::FluidCacheFolder, RowState::FoundOnYourComputer),
            a_row(b"//textures/floor_tile.png", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"//textures/metal_grid.png", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"//textures/wood_oak.png", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"//ref/old_reference.jpg", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"//../hdri/studio_4k.exr", Kind::Image, RowState::AlreadyOnTheMachine),
            a_row(b"//old/backup_plate.png", Kind::Image, RowState::SetAside),
        ];
        rows[6].members = Some(340);
        rows[6].size = Some(2100 * MB);
        for (at, size) in [(7, 64), (8, 18), (9, 42), (10, 3)] {
            rows[at].size = Some(size * MB);
        }
        rows[11].size = Some(128 * MB);
        let mut sitting = a_sitting(rows);
        sitting.completion = completion;
        sitting
    }

    /// **The summary sentence, the answer to the whole check** — `§ Above
    /// the rows`: worst first in the order of the groups, every zero left
    /// out, what will upload counted in files, the size already on the
    /// server kept, *, found so far* while reading, ignored rows not counted,
    /// and **never *Nothing is missing* while anything is open or the check
    /// is still reading.** Its figures keep the foot's old colours. *(Until
    /// 2026-10-18 the figures were along the bottom.)*
    #[test]
    fn the_summary_sentence_is_worst_first_leaves_out_every_zero_and_says_nothing_is_missing_only_when_true() {
        let finished = the_panel(&a_run(the_kitchen(Some(Completion::Completed))), SERVER, FILE, None);
        let sentence = the_summary_reads(&finished);
        assert_eq!(sentence, "2 failed · 3 missing · 1 not verified · 344 files to upload, 2.2 GB · 128 MB already on the server.");
        // **Design's own example, word for word.**
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains(&format!("*{sentence}*")), "Design's example and the composition differ: {sentence}");
        let colours: Vec<(String, &str)> = finished
            .the_summary
            .as_ref()
            .expect("a sentence")
            .parts
            .iter()
            .filter(|part| !part.colour.as_str().is_empty())
            .map(|part| (reads(&part.said), part.colour.as_str()))
            .collect();
        assert_eq!(
            colours,
            vec![
                ("2 failed".to_string(), "bad"),
                ("3 missing".to_string(), "bad"),
                ("1 not verified".to_string(), "busy"),
                ("128 MB already on the server".to_string(), "ok"),
            ]
        );
        // **Still reading: the same, ending *, found so far.*** — the
        // drawing's own kitchen line.
        let reading = the_panel(&a_run(the_kitchen(None)), SERVER, FILE, None);
        assert_eq!(
            the_summary_reads(&reading),
            "2 failed · 3 missing · 1 not verified · 344 files to upload, 2.2 GB · 128 MB already on the server, found so far."
        );
        assert!(the_mockup().contains("344 files to upload, 2.2\\u00a0GB \\u00b7 128\\u00a0MB already on the server, found so\\u00a0far."));

        let said = |rows: Vec<ACheckedRow>, completion: Option<Completion>| {
            let mut sitting = a_sitting(rows);
            sitting.completion = completion;
            let panel = the_panel(&a_run(sitting), SERVER, FILE, None);
            panel.the_summary.as_ref().map(|_| the_summary_reads(&panel))
        };
        let done = Some(Completion::Completed);
        let found = || a_row(b"//a/found.png", Kind::Image, RowState::FoundOnYourComputer);
        // **Ignored rows are the user's answer and are not counted.**
        assert_eq!(
            said(vec![found(), a_row(b"//a/aside.png", Kind::Image, RowState::SetAside)], done),
            Some("Nothing is missing. 1 file to upload, 1.0 MB.".to_string())
        );
        assert_eq!(
            said(vec![a_row(b"//a/on.png", Kind::Image, RowState::AlreadyOnTheMachine)], done),
            Some("Nothing is missing. 1.0 MB already on the server.".to_string())
        );
        assert_eq!(said(vec![], done), Some("Nothing is missing.".to_string()));
        // **Never *Nothing is missing* with anything open, or while reading.**
        for (rows, why) in [
            (vec![found(), a_row(b"//a/gone.png", Kind::Image, RowState::NotFound)], "missing"),
            (vec![found(), a_row(b"//a/sent.png", Kind::Image, RowState::CouldNotBeSent)], "failed"),
            (vec![found(), a_row(b"//a/before.png", Kind::Image, RowState::SentBeforeNotConfirmed)], "not verified"),
        ] {
            let sentence = said(rows, done).expect("a sentence");
            assert!(!sentence.contains(words::NOTHING_IS_MISSING), "{why}: {sentence}");
            assert!(sentence.starts_with(&format!("1 {why}")), "{sentence}");
        }
        assert_eq!(said(vec![found()], None), Some("1 file to upload, 1.0 MB, found so far.".to_string()));
        assert_eq!(said(vec![found()], Some(Completion::TheConnectionDropped)), Some("1 file to upload, 1.0 MB, found so far.".to_string()));
        assert_eq!(said(vec![], None), None, "a check still reading that found nothing said something");
        assert!(design.contains(&format!("*{}*", "Nothing is missing. 90 MB already on the server.")));
    }

    /// **Collapsed, the strip shows the summary sentence** — `§ Where it
    /// sits`, since 2026-10-18; *Uploading 4 of 12* stays while the check's
    /// uploads run. *(Until then it read* 6 of 12 found · 2.2 GB to upload ·
    /// 3 not found*.)*
    #[test]
    fn the_strip_shows_the_summary_sentence() {
        for completion in [Some(Completion::Completed), None] {
            let panel = the_panel(&a_run(the_kitchen(completion)), SERVER, FILE, None);
            assert_eq!(the_strip_reads(&panel), the_summary_reads(&panel));
        }
        let held = a_run(the_kitchen(None));
        let sending = the_panel_knowing(&held, SERVER, FILE, None, &WhatElseIsKnown { the_list_in_settings: None, uploading: Some((4, 12)), on_its_way: Vec::new(), a_send_is_being_counted: false });
        assert_eq!(the_strip_reads(&sending), "Uploading 4 of 12");
    }

    /// **`§ Along the bottom`: only the button, naming the files and their
    /// size** — *Upload 2 files (90 MB)*, *Upload the 344 found so far*.
    /// Nothing else is in the foot. *(Until 2026-10-18 the figures sat there
    /// too.)*
    #[test]
    fn along_the_bottom_is_only_the_button_naming_the_files_and_their_size() {
        let foot = |sitting: ASitting| the_panel(&a_run(sitting), SERVER, FILE, None).the_foot.expect("a foot");
        assert_eq!(reads(&foot(the_kitchen(None)).upload), "Upload the 344 found so far");
        assert_eq!(reads(&foot(the_kitchen(Some(Completion::Completed))).upload), "Upload 344 files (2.2 GB)");
        let one = foot(a_sitting(vec![a_row(b"//a/found.png", Kind::Image, RowState::FoundOnYourComputer)]));
        assert_eq!(reads(&one.upload), "Upload 1 file (1.0 MB)");
        assert!(!one.upload_is_unavailable);
        let nothing = the_panel(&a_run(a_sitting(vec![a_row(b"//a/gone.png", Kind::Image, RowState::NotFound)])), SERVER, FILE, None);
        assert!(nothing.the_foot.is_none());
        let answer = serde_json::to_value(&one).expect("it serialises");
        let fields: Vec<&String> = answer.as_object().expect("an object").keys().collect();
        assert_eq!(fields, vec!["upload", "upload_is_unavailable"]);
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains("*Upload 2 files (90 MB)*") && design.contains("*Upload the 4 found so far*"));
    }

    #[test]
    fn every_file_already_on_the_server_leaves_no_upload_button() {
        let mut row = a_row(b"//a/found.png", Kind::Image, RowState::AlreadyOnTheMachine);
        row.size = Some(195 * 1024);
        let panel = the_panel(&a_run(a_sitting(vec![row])), SERVER, FILE, None);
        assert_eq!(the_summary_reads(&panel), "Nothing is missing. 195 KB already on the server.");
        assert!(panel.the_foot.is_none(), "{:?}", panel.the_foot);
        let table = panel.the_table.as_ref().expect("the server's file still has a row");
        assert_eq!(every_row(table).len(), 1);
    }

    /// **Every row's action that fits its state is the module's own**, and
    /// only the words are this page's — *Verify* for the module's Confirm.
    #[test]
    fn every_rows_action_is_the_modules_own() {
        for state in RowState::ALL {
            let drawn = the_action(state, false);
            let expected = match state.the_action_that_fits() {
                Action::Nothing => None,
                Action::Locate => Some(("locate", words::LOCATE)),
                Action::Confirm => Some(("confirm", words::VERIFY)),
                Action::BringBack => Some(("restore", words::RESTORE)),
                Action::TryAgainTheSend | Action::TryAgainTheFixing => Some(("try-again", words::TRY_AGAIN)),
            };
            assert_eq!(
                drawn.map(|one| (one.act.as_str().to_string(), one.label.as_str().to_string())),
                expected.map(|(act, label)| (act.to_string(), Prose::new(label).as_str().to_string())),
                "{state:?}"
            );
        }
        // **The one visible on the row is Design's**: none where Smoozy
        // uploads the file or has it, the module's Locate… behind ⋯.
        for state in RowState::ALL {
            let shown = the_visible_action(&a_row(b"//a/x.png", Kind::Image, state), false).map(|one| one.act.as_str().to_string());
            let expected = match state {
                RowState::FoundOnYourComputer | RowState::AlreadyOnTheMachine => None,
                RowState::NotFound => Some("locate"),
                RowState::SentBeforeNotConfirmed => Some("confirm"),
                RowState::SetAside => Some("restore"),
                RowState::CouldNotBeSent | RowState::CouldNotBeFixed => Some("try-again"),
            };
            assert_eq!(shown, expected.map(str::to_string), "{state:?}");
        }
    }

    /// **`§ When the connection drops mid-check`**: the acts that need the
    /// server go unavailable, the local ones stay, and the upload and the
    /// summary say they are a total so far.
    #[test]
    fn a_dropped_connection_takes_the_server_acts_and_the_upload_says_so_far() {
        let mut sitting = a_sitting(vec![
            a_row(b"//a/found.png", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"//a/sent.png", Kind::Image, RowState::CouldNotBeSent),
            a_row(b"//a/before.png", Kind::Image, RowState::SentBeforeNotConfirmed),
            a_row(b"//a/missing.png", Kind::Image, RowState::NotFound),
        ]);
        sitting.completion = Some(Completion::TheConnectionDropped);
        sitting.still_reading_below = Some(RecordedLocation::as_the_reading_carried_it(b"//lib/props.blend"));
        let panel = the_panel(&a_run(sitting), SERVER, FILE, None);
        assert!(panel.still_reading.is_none(), "a dropped connection is reading nothing");
        assert!(panel.the_statements.iter().any(|one| one.which.as_str() == "the-connection-dropped"));
        assert!(the_summary_reads(&panel).ends_with(", found so far."));
        let table = panel.the_table.as_ref().expect("a table");
        for row in every_row(table) {
            if let Some(action) = &row.action {
                let needs_the_server = matches!(action.act.as_str(), "try-again" | "confirm");
                assert_eq!(action.unavailable, needs_the_server, "{}", action.act.as_str());
            }
        }
        let foot = panel.the_foot.expect("a foot");
        assert!(foot.upload_is_unavailable);
        assert_eq!(reads(&foot.upload), "Upload the 1 found so far");
    }

    /// **`§ While it is still reading below a linked file`, 2026-11-04: the walk's
    /// count takes the place of both lines, with *Stop* beside it, the search's own
    /// act**, and a dropped connection is counting nothing.
    #[test]
    fn the_walk_counts_in_place_of_either_line_with_stop_beside_it() {
        let reads_line = |panel: &TheCheck| reads(panel.still_reading.as_ref().expect("a line"));
        // Below a linked file.
        let mut sitting = a_sitting(vec![a_row(b"//a/found.png", Kind::Image, RowState::FoundOnYourComputer)]);
        sitting.completion = None;
        sitting.still_reading_below = Some(RecordedLocation::as_the_reading_carried_it(b"//lib/props.blend"));
        let mut held = a_run(sitting);
        let before = the_panel(&held, SERVER, FILE, None);
        assert!(reads_line(&before).contains(words::STILL_READING_BELOW));
        assert!(before.the_walks_stop.is_none());
        held.sittings.get_mut(&keyed(SERVER, FILE)).expect("it").the_walk = Some((120, 480));
        let walking = the_panel(&held, SERVER, FILE, None);
        assert_eq!(reads_line(&walking), "Checking files on the server: 120 of 480");
        assert_eq!(walking.the_walks_stop, Some(Prose::new(words::STOP)));
        assert!(walking.the_table.is_some(), "the rows on screen are true and stay usable");

        // On the file itself: no table, no tools, no foot, as under the line it replaced.
        held.files_being_read.insert(keyed(SERVER, FILE));
        let the_root = the_panel(&held, SERVER, FILE, None);
        assert_eq!(reads_line(&the_root), "Checking files on the server: 120 of 480");
        assert_eq!(the_root.the_walks_stop, Some(Prose::new(words::STOP)));
        assert!(the_root.the_table.is_none() && the_root.the_tools.is_none() && the_root.the_foot.is_none());
        held.files_being_read.clear();

        // A dropped connection is counting nothing.
        let standing = held.sittings.get_mut(&keyed(SERVER, FILE)).expect("it");
        standing.completion = Some(Completion::TheConnectionDropped);
        let dropped = the_panel(&held, SERVER, FILE, None);
        assert!(dropped.still_reading.is_none() && dropped.the_walks_stop.is_none());

        // Design's words, the drawing's state, and the page's one act.
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains("*Checking files on the server: 120 of 480*"));
        assert!(the_mockup().contains("<option value=\"chk-walk\">"));
        assert!(the_mockup().contains("Checking files on the server: 120 of 480"));
        let page = include_str!("../../web/the-dependency-check.js");
        assert!(page.contains("said.the_walks_stop, \"stop-the-search\""));
    }

    /// **`§ Above the rows`, 2026-11-04: a row to upload that is a linked file nobody
    /// has read: the summary says so and never *Nothing is missing*** — the film's
    /// *Nothing is missing. 5 files to upload*, with 500 more in the next round.
    #[test]
    fn a_linked_file_going_up_unread_is_said_and_nothing_is_missing_is_not() {
        let mut library = a_row(b"//lib/fx.blend", Kind::Library, RowState::FoundOnYourComputer);
        library.size = Some(1024 * 1024);
        let panel = the_panel(&a_run(a_sitting(vec![library.clone()])), SERVER, FILE, None);
        assert_eq!(the_summary_reads(&panel), "1 file to upload, 1.0 MB. Linked files not checked yet.");
        assert_eq!(the_strip_reads(&panel), "1 file to upload, 1.0 MB. Linked files not checked yet.");

        // A picture going up has nothing below it.
        let picture = a_row(b"//a/wood.png", Kind::Image, RowState::FoundOnYourComputer);
        let panel = the_panel(&a_run(a_sitting(vec![picture])), SERVER, FILE, None);
        assert_eq!(the_summary_reads(&panel), "Nothing is missing. 1 file to upload, 1.0 MB.");

        // A linked file already on the server has been read.
        let read = a_row(b"//lib/fx.blend", Kind::Library, RowState::AlreadyOnTheMachine);
        let panel = the_panel(&a_run(a_sitting(vec![read])), SERVER, FILE, None);
        assert!(the_summary_reads(&panel).starts_with("Nothing is missing."));

        // *, found so far* already says it while the check reads.
        let mut reading = a_sitting(vec![library]);
        reading.completion = None;
        let panel = the_panel(&a_run(reading), SERVER, FILE, None);
        assert!(the_summary_reads(&panel).ends_with(", found so far."));

        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains("*5 files to upload, 136 MB. Linked files not checked yet.*"));
        assert!(the_mockup().contains("<option value=\"chk-unchecked\">"));
    }

    /// **`§ When the file could not be read at all`, 2026-11-04, a stand-in at the
    /// desk: where the server took no reading and this server's setup stands
    /// stopped, the panel says so, with *Set up again***, and the diary is handed
    /// the same sentence. **The same ending on a setup that stands finished keeps
    /// the read's words.**
    #[test]
    fn a_stopped_setup_says_so_where_the_check_could_not_read() {
        let the_diary = crate::surface::render_settings::TheDiaryInHand { diary: None, run: None, the_servers_name: "" };
        let mut held = WhatTheRunHolds::default();
        let _ = an_ending_is_held(
            &mut held,
            SERVER,
            FILE,
            Ending::NotAReadingAndNoneOfTheThreeEndingsFits,
            None,
            true,
            &the_diary,
        );
        let panel = the_panel(&held, SERVER, FILE, None);
        let failure = panel.the_ending.expect("the ending");
        assert_eq!(failure.sentence, Prose::new("Setup stopped."));
        assert_eq!(failure.the_second_sentence, Some(Prose::new("Finish setup, then check again.")));
        assert_eq!(
            failure.controls.iter().map(|one| one.act.as_str().to_string()).collect::<Vec<_>>(),
            ["set-up-again", "check-again", "clear-the-ending"]
        );
        assert_eq!(failure.controls[0].label, Prose::new("Set up again"));
        assert!(panel.the_table.is_none() && panel.nothing_was_read.is_some());

        let mut held = WhatTheRunHolds::default();
        let _ = the_check_gave_nothing(
            &mut held,
            SERVER,
            FILE,
            Ending::NotAReadingAndNoneOfTheThreeEndingsFits,
            None,
            &the_diary,
        );
        let failure = the_panel(&held, SERVER, FILE, None).the_ending.expect("the ending");
        assert_eq!(failure.sentence, Prose::new(words::THE_READ_DID_NOT_FINISH));

        // The landing asks the Set up tab's own rule, the press starts on that
        // tab, and Design and the drawing say the same.
        let working = include_str!("the_dependency_check.rs");
        assert!(working.contains("crate::surface::set_up::a_stopped_setup_stands(&read)"));
        let page = include_str!("../../web/the-dependency-check.js");
        assert!(page.contains("act === \"set-up-again\""));
        assert!(page.contains("a_set_up_act_that_needs_a_connection"));
        assert!(page.contains(&format!("act: \"{}\"", crate::surface::set_up::THE_READY_ACTION)));
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains("*Setup stopped.* and *Finish setup, then check again.*"));
        assert!(the_mockup().contains("<option value=\"chk-unsetup\">"));
    }

    /// **Several refused for one reason are said once, above their rows**, and
    /// each row keeps its own reason and its own *Try again*.
    #[test]
    fn several_refused_for_one_reason_are_said_once_and_each_row_keeps_its_retry() {
        let mut sitting = a_sitting(vec![
            a_row(b"//a/one.png", Kind::Image, RowState::CouldNotBeSent),
            a_row(b"//a/two.png", Kind::Image, RowState::CouldNotBeSent),
        ]);
        sitting.the_once = Some(SeveralForOneReason {
            folder: b"/ephemeral/textures/".to_vec(),
            rows: vec![0, 1],
            cleared: false,
        });
        let mut held = a_run(sitting);
        let panel = the_panel(&held, SERVER, FILE, None);
        let once = panel.the_once.expect("the once statement");
        assert_eq!(once.values[0].value.as_str(), "2");
        assert!(!once.sentence.as_str().contains('2'), "a figure travels inside the sentence");
        let rows = &panel.the_table.expect("a table").groups[0].rows;
        assert!(rows.iter().all(|row| row.action.is_some() && row.notes.iter().any(|note| note.failure.is_some())));
        the_act_is_taken(&mut held, SERVER, FILE, TheAct::ClearTheStatement, None);
        assert!(the_panel(&held, SERVER, FILE, None).the_once.is_none());
    }

    /// **A linked `.blend` found on this computer is sent, and the upload
    /// control counts it** — its fixing is wired since the 38's piece 7
    /// (`Dependency Upload & Remap` M13). Until then it owed a fixing nobody
    /// performed and was held back (step 10c's dependency wave piece 6).
    #[test]
    fn a_linked_blend_is_sent_and_the_upload_counts_it() {
        let sitting = a_sitting(vec![
            a_row(b"//tex/wood.png", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"//lib/props.blend", Kind::Library, RowState::FoundOnYourComputer),
            a_row(b"//tex/gone.png", Kind::Image, RowState::NotFound),
        ]);
        assert_eq!(crate::the_found_files_sent::the_rows_the_upload_sends(&sitting.rows), vec![0, 1]);
        let foot = the_panel(&a_run(sitting), SERVER, FILE, None).the_foot.expect("a foot");
        assert_eq!(reads(&foot.upload), "Upload 2 files (2.0 MB)");
    }

    /// **Choosing rows the way a file manager does** — `§ Acting on many
    /// rows`, decision 2: a click chooses that row alone, Cmd-click (Ctrl on
    /// Windows) adds a row or takes it away, and Shift-click chooses the range
    /// from the last row clicked, **in the order the rows are drawn**. An
    /// ignored row is not chosen. *(Until 2026-10-18 each row had a
    /// checkbox.)*
    #[test]
    fn rows_are_chosen_the_way_a_file_manager_chooses_them() {
        // In the sitting's order: found, missing, failed, ignored, missing.
        // Drawn: failed (2), missing (1, 4), to upload (0); ignored (3).
        let mut held = a_run(a_sitting(vec![
            a_row(b"//a/0.png", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"//a/1.png", Kind::Image, RowState::NotFound),
            a_row(b"//a/2.png", Kind::Image, RowState::CouldNotBeSent),
            a_row(b"//a/3.png", Kind::Image, RowState::SetAside),
            a_row(b"//a/4.png", Kind::Image, RowState::NotFound),
        ]));
        let chosen = |held: &WhatTheRunHolds| -> Vec<usize> {
            held.selections.get(&keyed(SERVER, FILE)).map(|set| set.iter().copied().collect()).unwrap_or_default()
        };
        let act = |held: &mut WhatTheRunHolds, act: TheAct, row: usize| {
            assert!(the_act_is_taken(held, SERVER, FILE, act, Some(row)).is_none());
        };
        // **Shift-click with nothing chosen chooses that row alone.**
        act(&mut held, TheAct::ChooseARange, 1);
        assert_eq!(chosen(&held), vec![1]);
        act(&mut held, TheAct::ChooseARow, 4);
        assert_eq!(chosen(&held), vec![4]);
        act(&mut held, TheAct::AddOrTakeARow, 2);
        assert_eq!(chosen(&held), vec![2, 4]);
        act(&mut held, TheAct::AddOrTakeARow, 4);
        assert_eq!(chosen(&held), vec![2]);
        // **The range runs from the last row clicked**, even one just taken
        // away: from 4 down to the found row (0).
        act(&mut held, TheAct::ChooseARange, 0);
        assert_eq!(chosen(&held), vec![0, 4]);
        // **And in the drawn order** — from the failed row (2) down to the
        // found one (0) is 2, 1, 4, 0: every row, not 0 to 2.
        act(&mut held, TheAct::ChooseARow, 2);
        act(&mut held, TheAct::ChooseARange, 0);
        assert_eq!(chosen(&held), vec![0, 1, 2, 4]);
        // The anchor stays where the last plain or Cmd-click was.
        act(&mut held, TheAct::ChooseARange, 1);
        assert_eq!(chosen(&held), vec![1, 2]);
        // **A plain click chooses that row alone.**
        act(&mut held, TheAct::ChooseARow, 0);
        assert_eq!(chosen(&held), vec![0]);
        // **An ignored row is not in the list a selection is made from.**
        act(&mut held, TheAct::ChooseARow, 3);
        act(&mut held, TheAct::AddOrTakeARow, 3);
        assert_eq!(chosen(&held), vec![0]);
        // **The bar appears at two or more**, and a chosen row says so.
        assert!(the_panel(&held, SERVER, FILE, None).the_selection.is_none(), "a bar over one row");
        act(&mut held, TheAct::AddOrTakeARow, 1);
        let panel = the_panel(&held, SERVER, FILE, None);
        assert_eq!(reads(&panel.the_selection.as_ref().expect("the bar").how_many), "2 selected");
        let chosen_rows: Vec<&str> = every_row(panel.the_table.as_ref().expect("a table"))
            .into_iter()
            .filter(|row| row.chosen)
            .map(|row| row.name.as_str())
            .collect();
        assert_eq!(chosen_rows, vec!["1.png", "0.png"]);
        // **Which key adds a row is the application's to say.**
        let adds = panel.the_table.as_ref().expect("a table").adds_a_row_with.as_str().to_string();
        assert_eq!(adds, if cfg!(target_os = "macos") { "meta" } else { "ctrl" });
    }

    /// **The bar shows only the acts that apply to at least one chosen row**,
    /// and says how many are chosen apart from how many *Accept* takes —
    /// `§ Acting on many rows`.
    #[test]
    fn the_bar_shows_only_the_acts_that_apply_to_a_chosen_row() {
        let bar_over = |rows: Vec<ACheckedRow>| {
            let mut held = a_run(a_sitting(rows));
            held.selections.insert(keyed(SERVER, FILE), [0, 1].into_iter().collect());
            the_panel(&held, SERVER, FILE, None).the_selection.expect("the bar")
        };
        let plain = bar_over(vec![
            a_row(b"//a/0.png", Kind::Image, RowState::FoundOnYourComputer),
            a_row(b"//a/1.png", Kind::Image, RowState::AlreadyOnTheMachine),
        ]);
        assert_eq!(reads(&plain.how_many), "2 selected");
        assert!(plain.accept.is_none() && plain.choose_a_folder.is_none() && plain.undo_the_guess.is_none());
        assert_eq!(plain.ignore.as_ref().map(Prose::as_str), Some(words::IGNORE));

        let mut near = a_row(b"//a/METAL.png", Kind::Image, RowState::NotFound);
        near.proposal = Some(StoredLocation::as_the_system_returned_it(b"/Users/alex/metal.png"));
        let every = bar_over(vec![near, found_by_a_shift(b"/work/x.png")]);
        assert_eq!(every.accept.as_ref().map(reads), Some("Accept 1".to_string()));
        assert!(every.choose_a_folder.is_some() && every.ignore.is_some() && every.undo_the_guess.is_some());

        let page = the_page();
        for drawn in ["if (chosen.accept)", "if (chosen.choose_a_folder)", "if (chosen.ignore)", "if (chosen.undo_the_guess)"] {
            assert!(page.contains(drawn), "the page draws an act the application did not send: {drawn}");
        }
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains("it shows only the acts that apply to at least one chosen row"));
    }

    /// **A decision is saved and is never taken on the run's memory alone** —
    /// step 10c's dependency wave piece 5. *(Until then every decision answered
    /// that saving it was not built.)* The run-held acts stay whole, and the
    /// decisions go through `crate::checking_a_file::a_decision_is_taken`,
    /// whose own tests write and read them back.
    #[test]
    fn a_decision_goes_to_the_record_and_the_run_held_acts_stay_whole() {
        let mut held = a_run(a_sitting(vec![found_by_a_shift(b"/work/x.png"), a_row(b"//a/y.png", Kind::Image, RowState::NotFound)]));
        for act in [TheAct::SetAside, TheAct::Locate, TheAct::UndoTheGuess, TheAct::AddASearchFolder] {
            assert!(the_act_is_taken(&mut held, SERVER, FILE, act, Some(0)).is_none());
        }
        assert_eq!(held.sittings[&keyed(SERVER, FILE)].rows[0].state, RowState::FoundOnYourComputer);
        let working = the_working_half();
        assert!(!working.contains("SAVING_IS_NOT_BUILT"), "a decision still says saving is not built");
        assert!(working.contains("a_decision_is_taken("), "no decision reaches the record");
        // The selection is run-held and works whole.
        assert!(the_act_is_taken(&mut held, SERVER, FILE, TheAct::ChooseARow, Some(0)).is_none());
        assert!(the_act_is_taken(&mut held, SERVER, FILE, TheAct::AddOrTakeARow, Some(1)).is_none());
        assert!(the_panel(&held, SERVER, FILE, None).the_selection.is_some());
        // And a shift's rows are counted by the module's own qualifier.
        let mut sitting = a_sitting(vec![found_by_a_shift(b"/work/x.png")]);
        sitting.the_shift = Some(TheShiftInHand {
            old_root: b"/work/".to_vec(),
            new_root: b"/Users/alex/Archive/".to_vec(),
            key: ShiftKey(1),
        });
        let shift = the_panel(&a_run(sitting), SERVER, FILE, None).the_shift.expect("the shift");
        assert!(the_texts(&shift.resolved).contains("\"1\""));
    }

    /// **The strip is told what to say, and its title is words beside a
    /// value**; a panel opened without a file says so.
    #[test]
    fn the_strip_counts_and_a_panel_with_no_file_says_so() {
        let held = a_run(a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]));
        let panel = the_panel(&held, SERVER, FILE, None);
        assert_eq!(panel.the_strip_title.values[0].as_str(), "kitchen_v14.blend");
        assert_eq!(the_strip_reads(&panel), "1 missing.");
        let none = the_panel(&held, SERVER, "", None);
        assert!(none.instead.is_some() && none.the_table.is_none());
    }

    /// **The page, under Node: a row drawn once — name, kind, one location
    /// with the file's text on hover, its notes — the summary sentence in its
    /// colours, a foot of one button, no checkbox and no column headings; and
    /// rows chosen like a file manager**: a click, Shift-click and the
    /// computer's own adding key each send their act, and **a press on a
    /// row's own button or ⋯ chooses nothing.** Skipped, and says so, where no
    /// Node is installed.
    #[test]
    fn the_page_draws_the_row_once_and_chooses_rows_like_a_file_manager() {
        let mut sitting = the_kitchen(Some(Completion::Completed));
        sitting.rows[3].proposal = Some(StoredLocation::as_the_system_returned_it(b"/Users/alex/Archive/textures/metallic.png"));
        let mut held = a_run(sitting);
        held.selections.insert(keyed(SERVER, FILE), [7, 8].into_iter().collect());
        let panel = the_panel(&held, SERVER, FILE, None);
        let answers = serde_json::json!({ "panel": panel });

        let folder = std::env::temp_dir().join(format!("smoozy-check-rows-page-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("the-dependency-check.js"), folder.join("the-dependency-check.js")).expect("the page");
        for (name, text) in THE_CHECKS_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_ROWS_HARNESS).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the check's page code was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        let adds = if cfg!(target_os = "macos") { "metaKey" } else { "ctrlKey" };
        assert_eq!(got["adds"], serde_json::json!(adds));
        assert_eq!(
            got["summary"],
            serde_json::json!({
                "looks": "csum",
                "reads": "2 failed · 3 missing · 1 not verified · 344 files to upload, 2.2 GB · 128 MB already on the server.",
                "colours": ["bad", "", "bad", "", "busy", "", "", "", "", "ok", ""]
            })
        );
        assert_eq!(
            got["firstRow"],
            serde_json::json!({
                "parts": ["nm mono lib", "kd", "loc", "note bad"],
                "name": "rig.blend",
                "kind": "Linked .blend",
                "hover": "//lib/rig.blend",
                "location": "The file says: //lib/"
            })
        );
        assert_eq!(got["chosenRows"], serde_json::json!(["7", "8"]));
        assert_eq!(got["foot"], serde_json::json!(["grow", "btn"]));
        assert_eq!(got["leftovers"], serde_json::json!([]), "a checkbox or a column heading is still drawn");
        assert_eq!(
            got["acts"],
            serde_json::json!([
                ["choose-a-row", 0],
                ["choose-a-range", 0],
                ["add-or-take-a-row", 0],
                ["choose-a-row", 0]
            ]),
            "a press on a row chose the wrong way, or a row's own control chose a row"
        );
        assert_eq!(got["groupsFolded"], serde_json::json!([["already-there", "cside ok"], ["ignored", "cside idle"]]));
    }

    /// Draws the kitchen panel, reads its summary, its first row and its
    /// foot, then presses rows: plainly, with Shift, with the adding key, with
    /// the other key, and on the row's own ⋯ and action.
    const THE_ROWS_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
const acts = [];
globalThis.window = { __TAURI__: { core: { invoke(command, args) {
  if (command === "the_dependency_check_panel") return Promise.resolve(answers.panel);
  if (command === "an_act_on_the_check") acts.push([args.act, args.row]);
  return Promise.resolve(null);
} } } };
const { panels } = await import("./a-server.js");
await import("./the-dependency-check.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
const sheet = document.createElement("div");
const head = sheet.appendChild(document.createElement("div"));
const body = sheet.appendChild(document.createElement("div"));
panels[0].itIsDrawn({ head, body }, "server");
await settle();
const out = {};
const adds = answers.panel.the_table.adds_a_row_with === "meta" ? "metaKey" : "ctrlKey";
const other = adds === "metaKey" ? "ctrlKey" : "metaKey";
out.adds = adds;
const summary = everyPiece(body).find((one) => one.names.has("csum"));
out.summary = {
  looks: summary.className,
  reads: summary.textContent.replace(/\u00a0/g, " "),
  colours: summary.children.filter((one) => one.tagName === "SPAN").map((one) => one.className),
};
const rows = everyPiece(body).filter((one) => one.names.has("row"));
const first = rows[0];
const what = first.children.find((one) => one.names.has("what"));
const loc = what.children.find((one) => one.names.has("loc"));
out.firstRow = {
  parts: what.children.map((one) => one.className),
  name: what.children[0].textContent,
  kind: what.children[1].textContent.replace(/\u00a0/g, " "),
  hover: loc.title,
  location: loc.textContent.replace(/\u00a0/g, " "),
};
out.chosenRows = rows.filter((one) => one.names.has("sel")).map((one) => one.dataset.row);
const foot = everyPiece(body).find((one) => one.names.has("cfoot"));
out.foot = foot.children.map((one) => one.className);
out.leftovers = everyPiece(sheet).filter((one) => ["cb", "rhead", "rec", "got", "fig", "cnote"].some((gone) => one.names.has(gone))).map((one) => one.className);
out.groupsFolded = everyPiece(body).filter((one) => one.tagName === "DETAILS").map((one) => [one.dataset.group, one.className]);
const press = (row, how) => { for (const handler of row.listeners.click || []) handler({ preventDefault() {}, target: how.target || row, shiftKey: false, metaKey: false, ctrlKey: false, ...how }); };
press(first, {});
press(first, { shiftKey: true });
press(first, { [adds]: true });
press(first, { [other]: true });
// A row's own controls: the press reaches the row with the control as its target.
const more = everyPiece(first).find((one) => one.tagName === "BUTTON" && one.names.has("more"));
const action = everyPiece(first).find((one) => one.tagName === "BUTTON" && one.names.has("a"));
press(first, { target: more });
press(first, { target: action });
out.acts = acts.filter((one) => one[0] !== "what-the-server-said");
console.log(JSON.stringify(out));
"#;

    /// **A failed row's reason is a plain red note, and its cross sits among
    /// the row's buttons after *Try again* and ⋯** — the *Failed* rows of
    /// `chk-sel` in the drawing. Pressing the cross clears the row's failure
    /// and chooses nothing. **A failure already cleared keeps its red note
    /// and has no cross.** *(Until 2026-10-18 the reason was the collapsed
    /// failure shape inside the notes, cross and all.)* Skipped, and says so,
    /// where no Node is installed.
    #[test]
    fn a_failed_rows_reason_is_a_red_note_and_its_cross_sits_after_try_again_and_the_dots() {
        let mut sitting = the_kitchen(Some(Completion::Completed));
        sitting.rows[0].why_it_did_not_go_up = Some(WhyItDidNotGoUp::SmoozyCouldNotRemapItsPaths);
        sitting.rows[1].its_failure_was_cleared = true;
        let panel = the_panel(&a_run(sitting), SERVER, FILE, None);
        let answers = serde_json::json!({ "panel": panel });

        let folder = std::env::temp_dir().join(format!("smoozy-check-failed-row-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("the-dependency-check.js"), folder.join("the-dependency-check.js")).expect("the page");
        for (name, text) in THE_CHECKS_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_FAILED_ROW_HARNESS).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the check's page code was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        assert_eq!(
            got["standing"],
            serde_json::json!({
                "note": ["note bad", "Uploaded; remap failed: a path did not update", 0],
                "acts": [
                    ["BUTTON", "a", "", "Retry"],
                    ["BUTTON", "more", "", "\u{22ef}"],
                    ["BUTTON", "fx", "Dismiss", "\u{d7}"]
                ]
            }),
            "a standing failure's reason is not a plain red note, or its cross is not last among the row's buttons"
        );
        assert_eq!(
            got["cleared"],
            serde_json::json!({
                "note": ["note bad", "Upload failed: permission denied", 0],
                "acts": [
                    ["BUTTON", "a", "", "Retry"],
                    ["BUTTON", "more", "", "\u{22ef}"]
                ]
            }),
            "a cleared failure still carries a cross"
        );
        assert_eq!(
            got["acts"],
            serde_json::json!([["clear-a-rows-failure", 0]]),
            "the cross did not clear the row's failure, or pressing it chose the row"
        );
    }

    /// Draws the kitchen with its second failure cleared, reads both failed
    /// rows' notes and buttons, then presses the first row's cross — once on
    /// the cross, once as the row sees it.
    const THE_FAILED_ROW_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
const acts = [];
globalThis.window = { __TAURI__: { core: { invoke(command, args) {
  if (command === "the_dependency_check_panel") return Promise.resolve(answers.panel);
  if (command === "an_act_on_the_check") acts.push([args.act, args.row]);
  return Promise.resolve(null);
} } } };
const { panels } = await import("./a-server.js");
await import("./the-dependency-check.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
const sheet = document.createElement("div");
const head = sheet.appendChild(document.createElement("div"));
const body = sheet.appendChild(document.createElement("div"));
panels[0].itIsDrawn({ head, body }, "server");
await settle();
const rows = everyPiece(body).filter((one) => one.names.has("row"));
const read = (row) => {
  const what = row.children.find((one) => one.names.has("what"));
  const notes = what.children.filter((one) => one.names.has("note"));
  const buttons = row.children.find((one) => one.names.has("acts")).children;
  return {
    note: notes.length === 1
      ? [notes[0].className, notes[0].textContent.replace(/\u00a0/g, " "), everyPiece(notes[0]).filter((one) => one.tagName === "BUTTON" || one.tagName === "DIV").length]
      : notes.map((one) => one.className),
    acts: buttons.map((one) => [one.tagName, one.className, one.attributes["aria-label"] === one.title ? one.title.replace(/\u00a0/g, " ") : "the label and the hover differ", one.textContent.replace(/\u00a0/g, " ")]),
  };
};
const out = { standing: read(rows[0]), cleared: read(rows[1]) };
// The "Retry" and the dots carry no hover here; only the cross does.
for (const which of [out.standing, out.cleared]) for (const act of which.acts) if (act[1] !== "fx") act[2] = "";
const cross = everyPiece(rows[0]).find((one) => one.tagName === "BUTTON" && one.names.has("fx"));
if (cross) {
  cross.press();
  for (const handler of rows[0].listeners.click || []) handler({ preventDefault() {}, target: cross, shiftKey: false, metaKey: false, ctrlKey: false });
}
await settle();
out.acts = acts;
console.log(JSON.stringify(out));
"#;

    /// **The Creator's tenth notes, note 4: an upload that ends with files
    /// arrived runs the check again once, and one that was stopped or cut, or
    /// is still asking *Replace or stop*, runs none** —
    /// `Dependency Upload & Remap` G14 as widened 2026-10-17. Pictures alone
    /// are enough; a linked `.blend` still is. **And the send itself goes
    /// through this rule**, so the page's own path cannot keep the older one. It fails if the check runs
    /// again only for a linked `.blend`, or after a stopped upload.
    #[test]
    fn an_upload_with_files_arrived_checks_again_once_and_a_stopped_one_does_not() {
        use crate::the_found_files_sent::WhatTheSendCameTo;
        let checks_after = |came_to: WhatTheSendCameTo| {
            let mut checks = 0;
            the_check_runs_again_after(&came_to, || checks += 1);
            checks
        };
        let pictures_alone = WhatTheSendCameTo { published: 3, ..Default::default() };
        assert_eq!(checks_after(pictures_alone.clone()), 1, "pictures alone went up and nothing was read again");
        assert_eq!(
            checks_after(WhatTheSendCameTo { published: 2, libraries_published: 1, ..Default::default() }),
            1,
            "a linked .blend went up and nothing was read again"
        );
        assert_eq!(
            checks_after(WhatTheSendCameTo { published: 1, failed: 1, ..Default::default() }),
            1,
            "a refused file is not a stop, and one file arrived"
        );
        for (why, came_to) in [
            ("nothing arrived", WhatTheSendCameTo { already_there: 4, ..Default::default() }),
            ("nothing arrived but a refusal", WhatTheSendCameTo { failed: 2, ..Default::default() }),
            (
                "stopped or cut",
                WhatTheSendCameTo { published: 2, libraries_published: 1, did_not_finish: 1, ..Default::default() },
            ),
            ("still asking Replace or stop", WhatTheSendCameTo { asks_to_replace: Some(2), ..pictures_alone }),
            ("the lock closed it", WhatTheSendCameTo { the_lock_closed_it: true, ..Default::default() }),
        ] {
            assert_eq!(checks_after(came_to), 0, "{why}, and the check ran again");
        }

        let working = the_working_half();
        let the_send = working
            .split("fn a_send_is_asked_for<")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the send");
        assert!(the_send.contains("the_check_runs_again_after(&came_to"), "the send does not ask the rule");
        assert!(!the_send.contains("libraries_published > 0"), "the send keeps the older rule beside it");
    }

    /// **Replace or stop, and the folder reading over a set row.**
    #[test]
    fn the_overwrite_box_is_replace_or_stop_and_widens_to_the_folder() {
        let mut held = a_run(a_sitting(vec![a_row(b"//a/x.png", Kind::Image, RowState::FoundOnYourComputer)]));
        held.overwrites.insert(
            keyed(SERVER, FILE),
            AnOverwrite {
                row: 0,
                location: b"/Users/alex/x/".to_vec(),
                jobs: Vec::new(),
                labels: Vec::new(),
                the_folder: Some(TheFolderDiffers { differ: 12, members: 340, only_on_the_server: 3 }),
            },
        );
        let asked = the_panel(&held, SERVER, FILE, None).the_overwrite.expect("the box");
        assert!(asked.live_jobs.is_none(), "no job, no live-job line");
        let folder = the_texts(&asked.the_folder.expect("the folder reading"));
        for figure in ["\"12\"", "\"340\"", "\"3\""] {
            assert!(folder.contains(figure), "{figure} is not a value of its own");
        }
        // **Replace is sent** (step 10c's dependency wave piece 6), never
        // answered on the run's memory.
        assert!(the_act_is_taken(&mut held, SERVER, FILE, TheAct::Replace, None).is_none());
        the_act_is_taken(&mut held, SERVER, FILE, TheAct::StopTheOverwrite, None);
        assert!(the_panel(&held, SERVER, FILE, None).the_overwrite.is_none());
    }

    // ── The Creator's note 5 on 0.1.8: the search folders on the check ──────

    fn a_search_over(folders: &[&[u8]]) -> TheSearchInHand {
        TheSearchInHand {
            folders: folders.iter().map(|folder| (folder.to_vec(), Some(FolderVerdict::SearchedWhole))).collect(),
            the_matches_came_from: Some(b"/Users/alex/Archive/".to_vec()),
            running: false,
            searched_at: None,
        }
    }

    fn the_folders_line_of(panel: &TheCheck) -> &AStatement {
        panel
            .the_statements
            .iter()
            .find(|one| one.which.as_str() == "the-folders")
            .expect("the folders line")
    }

    /// **The folders line names the folders searched by their last folder
    /// name, three and then how many more, each with its full path for hover**
    /// — `Design/Dependency Upload & Remap § The search folders on this panel`.
    /// Until the Creator's note 5 on 0.1.8 it said *5 folders searched.*
    #[test]
    fn the_folders_line_names_three_and_says_how_many_more_with_each_path_on_hover() {
        let five: [&[u8]; 5] = [
            b"/Users/alex/Work",
            b"/Users/alex/Libraries/Assets/",
            b"/Volumes/Archive/Textures",
            b"/Volumes/Archive/HDRI",
            b"/Volumes/Archive/2023",
        ];
        let mut sitting = a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]);
        sitting.the_search = Some(a_search_over(&five));
        sitting.the_list_it_searched = five.iter().map(|one| one.to_vec()).collect();
        let held = a_run(sitting);
        let panel = the_panel(&held, SERVER, FILE, None);
        let line = the_folders_line_of(&panel);
        assert_eq!(line.said.values(), vec!["Work", "Assets", "Textures", "2"]);
        assert_eq!(line.said.said().replace('\u{a0}', " "), "Searched , , and more.");
        let paths: Vec<&str> = line.the_full_paths.iter().map(Verbatim::as_str).collect();
        assert_eq!(paths, vec!["/Users/alex/Work", "/Users/alex/Libraries/Assets/", "/Volumes/Archive/Textures"]);
        assert!(!line.its_last_sentence_is_bold);
        assert!(!format!("{:?}", line.said).contains("folders searched"), "the line still gives a count");

        // Three or fewer are named whole, with no *more*.
        let mut sitting = a_sitting(Vec::new());
        sitting.the_search = Some(a_search_over(&five[..3]));
        let panel = the_panel(&a_run(sitting), SERVER, FILE, None);
        let line = the_folders_line_of(&panel);
        assert_eq!(line.said.values(), vec!["Work", "Assets", "Textures"]);
        assert!(!line.said.said().replace('\u{a0}', " ").contains("more"));
    }

    /// **Each folder the line names is a link to its folder, with its full
    /// path, and *The matches came from …* is not composed** — the Creator's
    /// note 5 on the test copy (`Design/Dependency Upload & Remap § The search
    /// folders on this panel`, 2026-10-04). The search still hands in where
    /// the matches came from ([`a_search_over`] does); the line says nothing
    /// of it, and the words are gone from this file.
    #[test]
    fn each_searched_folder_is_a_link_with_its_path_and_the_matches_are_not_said() {
        let three: [&[u8]; 3] = [b"/Volumes/SAMSUNG T7/MijnCommerce", b"/Users/alex/Blender", b"/Volumes/Archive/2023"];
        let mut sitting = a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]);
        sitting.the_search = Some(a_search_over(&three));
        sitting.the_list_it_searched = three.iter().map(|one| one.to_vec()).collect();
        let panel = the_panel(&a_run(sitting), SERVER, FILE, None);
        let line = the_folders_line_of(&panel);
        assert_eq!(line.said.values(), vec!["MijnCommerce", "Blender", "2023"]);
        let paths: Vec<&str> = line.the_full_paths.iter().map(Verbatim::as_str).collect();
        assert_eq!(paths, vec!["/Volumes/SAMSUNG T7/MijnCommerce", "/Users/alex/Blender", "/Volumes/Archive/2023"]);
        // **The act each name hands back is a closed one**, and it is the
        // opening.
        let act = line.its_folders_open_with.as_ref().expect("the names are links");
        let parsed: TheAct = serde::Deserialize::deserialize(serde::de::value::StrDeserializer::<
            serde::de::value::Error,
        >::new(act.as_str()))
        .expect("a closed act");
        assert_eq!(parsed, TheAct::OpenASearchedFolder);
        assert!(!format!("{:?}", line.said).contains("matches"), "the line still says where the matches came from");
        assert!(!the_working_half().contains(&["THE_MATCHES", "CAME_FROM"].join("_")), "the words are still in the file");
        // *Added 2023.* names a folder with its path on hover, and is no link.
        let added = the_folder_added(&TheFolderAdded { folder: three[2].to_vec(), found: 1, still_not_found: 0 });
        assert!(added.its_folders_open_with.is_none());
    }

    /// **A folder's link opens that folder through the existing reveal's
    /// second act, and a refusal is said** — `Desktop app and delivery` K22,
    /// with the sentence `Download` H8 gives a job's files. Its place in the
    /// line is its place in the list the search used; a place with no folder
    /// opens nothing and says so. Opening one leaves *Added …* where it is.
    #[test]
    fn a_searched_folders_link_opens_that_folder_and_a_refusal_is_said() {
        let two: [&[u8]; 2] = [b"/Volumes/SAMSUNG T7/MijnCommerce", b"/Users/alex/Blender"];
        let mut sitting = a_sitting(Vec::new());
        sitting.the_search = Some(a_search_over(&two));
        sitting.the_folder_added = Some(TheFolderAdded { folder: two[1].to_vec(), found: 1, still_not_found: 0 });
        let mut held = a_run(sitting);
        assert_eq!(the_searched_folder_at(&held, SERVER, FILE, Some(1)), Some(two[1].to_vec()));
        assert_eq!(the_searched_folder_at(&held, SERVER, FILE, Some(2)), None);
        assert_eq!(the_searched_folder_at(&held, SERVER, FILE, None), None);
        assert_eq!(the_searched_folder_at(&held, SERVER, "another.blend", Some(0)), None);

        let mut asked = Vec::new();
        let opened = a_searched_folder_is_opened(Some(two[0]), |path| {
            asked.push(path.to_path_buf());
            crate::reveal::Revealed::ItOpened
        });
        assert_eq!(opened, None, "an opened folder says nothing");
        assert_eq!(asked, vec![std::path::PathBuf::from("/Volumes/SAMSUNG T7/MijnCommerce")]);
        let refused = a_searched_folder_is_opened(Some(two[0]), |_| crate::reveal::Revealed::ItCouldNotBeOpened);
        assert_eq!(refused, Some(Prose::new(words::COULD_NOT_OPEN_THE_FOLDER)));
        let nothing = a_searched_folder_is_opened(None, |_| panic!("nothing is asked for"));
        assert_eq!(nothing, Some(Prose::new(words::COULD_NOT_OPEN_THE_FOLDER)));

        // **The act is wired to the existing reveal**, not a second one.
        let answered = the_working_half()
            .split("async fn the_act_is_answered<")
            .nth(1)
            .and_then(|rest| rest.split("let decision = match act").nth(1))
            .expect("the act's answer");
        assert!(answered.contains("a_searched_folder_is_opened(folder.as_deref(), crate::reveal::a_folder)"));

        the_added_statement_is_let_go(&mut held, SERVER, FILE, TheAct::OpenASearchedFolder);
        assert!(held.sittings[&keyed(SERVER, FILE)].the_folder_added.is_some(), "opening a folder let *Added* go");
    }

    /// ***Check again* sits on the folders line at all times**, the existing
    /// act — on a complete check, and on one where nothing was searched.
    /// Until the Creator's note 5 on 0.1.8 it was offered only where the file
    /// could not be read.
    #[test]
    fn check_again_is_offered_on_a_complete_check() {
        let held = a_run(a_sitting(vec![a_row(b"//a/on.png", Kind::Image, RowState::AlreadyOnTheMachine)]));
        let panel = the_panel(&held, SERVER, FILE, None);
        assert!(panel.the_ending.is_none());
        let line = the_folders_line_of(&panel);
        let control = line.control.as_ref().expect("Check again on the folders line");
        assert_eq!(control.label.as_str().replace('\u{a0}', " "), words::CHECK_AGAIN);
        assert!(!control.unavailable);
        // **The existing act**, and nothing new: the page hands back
        // `check-again`, which is `TheAct::CheckAgain`.
        let parsed: TheAct = serde::Deserialize::deserialize(serde::de::value::StrDeserializer::<
            serde::de::value::Error,
        >::new(control.act.as_str()))
        .expect("a closed act");
        assert_eq!(parsed, TheAct::CheckAgain);
        // With no search run, the line names nothing and still offers it.
        assert!(line.said.pieces.is_empty());
    }

    /// **A list changed in Settings since this search is said, beside *Check
    /// again*, and an unchanged one is not** — a comparison when the panel is
    /// drawn; nothing searches by itself.
    #[test]
    fn a_list_changed_in_settings_is_said_and_an_unchanged_one_is_not() {
        let mut sitting = a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]);
        sitting.the_search = Some(a_search_over(&[b"/Users/alex/Work"]));
        sitting.the_list_it_searched = vec![b"/Users/alex/Work".to_vec()];
        let held = a_run(sitting);
        let said = |list: Option<Vec<Vec<u8>>>| {
            let panel = the_panel_knowing(&held, SERVER, FILE, None, &WhatElseIsKnown { the_list_in_settings: list, uploading: None, on_its_way: Vec::new(), a_send_is_being_counted: false });
            the_folders_line_of(&panel).clone()
        };
        let unchanged = said(Some(vec![b"/Users/alex/Work".to_vec()]));
        assert!(!unchanged.said.said().replace('\u{a0}', " ").contains(words::YOUR_SEARCH_FOLDERS_CHANGED));
        assert_eq!(unchanged.said.said().replace('\u{a0}', " "), "Searched .");
        let changed = said(Some(vec![b"/Users/alex/Work".to_vec(), b"/Volumes/Assets".to_vec()]));
        assert!(changed.said.said().replace('\u{a0}', " ").ends_with(words::YOUR_SEARCH_FOLDERS_CHANGED), "{}", changed.said.said().replace('\u{a0}', " "));
        assert!(changed.its_last_sentence_is_bold);
        assert!(changed.control.is_some());
        // A folder removed in Settings is a change too, and a list that would
        // not read says nothing about it.
        assert!(said(Some(Vec::new())).said.said().replace('\u{a0}', " ").contains(words::YOUR_SEARCH_FOLDERS_CHANGED));
        assert!(!said(None).said.said().replace('\u{a0}', " ").contains(words::YOUR_SEARCH_FOLDERS_CHANGED));
        // **The note's own case**: no folders when the check ran, one added in
        // Settings since — nothing was searched, and the line says the list
        // changed.
        let empty = a_run(a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]));
        let panel = the_panel_knowing(
            &empty,
            SERVER,
            FILE,
            None,
            &WhatElseIsKnown { the_list_in_settings: Some(vec![b"/Volumes/Assets".to_vec()]), uploading: None, on_its_way: Vec::new(), a_send_is_being_counted: false },
        );
        assert_eq!(the_folders_line_of(&panel).said.said().replace('\u{a0}', " "), words::YOUR_SEARCH_FOLDERS_CHANGED);
        // The words are Design's own.
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains(words::YOUR_SEARCH_FOLDERS_CHANGED));
    }

    /// ***Add* closes the box and the panel says once what it found**: *Added
    /// Assets. 4 found, 3 still missing.*, or *Nothing more found.* — in
    /// front of the statements, the name with its full path on hover, and let
    /// go of at the user's next act. The box says what adding does in one
    /// sentence, Design's own.
    #[test]
    fn what_add_found_is_said_once_and_let_go_at_the_next_act() {
        let mut sitting = a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]);
        sitting.the_folder_added = Some(TheFolderAdded {
            folder: b"/Users/alex/Libraries/Assets".to_vec(),
            found: 4,
            still_not_found: 3,
        });
        let mut held = a_run(sitting);
        let panel = the_panel(&held, SERVER, FILE, None);
        let added = &panel.the_statements[0];
        assert_eq!(added.which.as_str(), "a-folder-added");
        assert_eq!(added.said.values(), vec!["Assets", "4", "3"]);
        assert_eq!(added.said.said().replace('\u{a0}', " "), "Added . found, still missing.");
        assert_eq!(added.the_full_paths[0].as_str(), "/Users/alex/Libraries/Assets");
        assert!(added.its_first_sentence_is_bold && added.control.is_none());

        // The other two readings.
        for ((found, still), reads) in [((0, 3), "Added . Nothing more found."), ((4, 0), "Added . found.")] {
            if let Some(one) = held.sittings.get_mut(&keyed(SERVER, FILE)).and_then(|s| s.the_folder_added.as_mut()) {
                one.found = found;
                one.still_not_found = still;
            }
            assert_eq!(the_panel(&held, SERVER, FILE, None).the_statements[0].said.said().replace('\u{a0}', " "), reads);
        }

        // **Said once**: the add itself keeps it, the next act lets it go.
        the_added_statement_is_let_go(&mut held, SERVER, FILE, TheAct::AddASearchFolder);
        assert!(the_panel(&held, SERVER, FILE, None).the_statements.iter().any(|one| one.which.as_str() == "a-folder-added"));
        the_added_statement_is_let_go(&mut held, SERVER, FILE, TheAct::ChooseARow);
        assert!(!the_panel(&held, SERVER, FILE, None).the_statements.iter().any(|one| one.which.as_str() == "a-folder-added"));

        // **The box's one sentence**, as Design writes it.
        let the_box = the_panel(&held, SERVER, FILE, None).the_folder_box;
        assert_eq!(the_box.what_adding_does, vec![Words::plain(words::IT_IS_ADDED_AND_SEARCHED)]);
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(design.contains(&format!("*{}*", words::IT_IS_ADDED_AND_SEARCHED)));
        // **Every act lets it go first**, before it is answered.
        let answered = the_working_half()
            .split("async fn the_act_is_answered<")
            .nth(1)
            .and_then(|rest| rest.split("let decision = match act").next())
            .expect("the act's answer");
        assert!(answered.contains("the_added_statement_is_let_go(&mut held, server, file, act)"));
    }

    /// **The page, under Node: *Add* closes the box, the panel says what the
    /// folder found, the folders carry their paths on hover, and *Check
    /// again* on the folders line asks for a check.** Before the Creator's
    /// note 5 on 0.1.8 the box was drawn again, empty, after *Add*, and the
    /// line said only a count (`Audits/Build/Step 10c/The Creator's notes on
    /// 0.1.8, piece 3 — what landed.md`). **Skipped, and says so, where no
    /// Node is installed.**
    #[test]
    fn add_closes_the_box_and_the_page_draws_what_it_found_and_check_again() {
        let folders: [&[u8]; 4] = [
            b"/Users/alex/Work",
            b"/Users/alex/Libraries/HDRI",
            b"/Users/alex/Libraries/Textures",
            b"/Users/alex/Old",
        ];
        let mut before = a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]);
        before.the_search = Some(a_search_over(&folders));
        before.the_list_it_searched = folders.iter().map(|one| one.to_vec()).collect();
        before.the_chosen_search_folder = Some(b"/Volumes/Archive/2023".to_vec());
        // **Settings' list as it stands**, which the box lists first.
        let before_panel = the_panel_knowing(
            &a_run(before),
            SERVER,
            FILE,
            None,
            &WhatElseIsKnown {
                the_list_in_settings: Some(folders.iter().map(|one| one.to_vec()).collect()),
                uploading: None,
                on_its_way: Vec::new(),
                a_send_is_being_counted: false,
            },
        );
        let mut nothing_chosen = before_panel.clone();
        nothing_chosen.the_folder_box.chosen = None;
        let mut after = a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::FoundOnYourComputer)]);
        let mut five = folders.to_vec();
        five.push(b"/Volumes/Archive/2023");
        after.the_search = Some(a_search_over(&five));
        after.the_list_it_searched = five.iter().map(|one| one.to_vec()).collect();
        after.the_folder_added = Some(TheFolderAdded {
            folder: b"/Volumes/Archive/2023".to_vec(),
            found: 4,
            still_not_found: 3,
        });
        let after_panel = the_panel(&a_run(after), SERVER, FILE, None);
        let answers = serde_json::json!({ "before": before_panel, "after": after_panel, "nothing": nothing_chosen });

        let folder = std::env::temp_dir().join(format!("smoozy-check-folders-page-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("the-dependency-check.js"), folder.join("the-dependency-check.js")).expect("the page");
        for (name, text) in THE_CHECKS_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_FOLDERS_HARNESS).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the check's page code was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        assert_eq!(
            got,
            serde_json::json!({
                "addWaitsForAFolder": true,
                "opened": { "box": true, "chosen": true },
                "theBox": {
                    "order": ["H3", "P", "the-search-folders", "change-them-in-settings", "choose-a-search-folder", "P", "DIV"],
                    "heading": Prose::new(words::YOUR_SEARCH_FOLDERS).as_str(),
                    "folders": [
                        "/Users/alex/Work",
                        "/Users/alex/Libraries/HDRI",
                        "/Users/alex/Libraries/Textures",
                        "/Users/alex/Old"
                    ],
                    "listLooks": "clist mono"
                },
                "changed": { "sections": ["s-fo"], "theBoxIsOpen": false },
                "afterAdd": {
                    "acts": ["add-a-search-folder"],
                    "theBoxIsDrawnAgain": false,
                    "statements": [
                        "• Added 2023. 4 found, 3 still missing.",
                        "• Searched Work, HDRI, Textures and 2 more. Check again"
                    ],
                    "bold": ["Added"],
                    "hover": [
                        "/Volumes/Archive/2023",
                        "/Users/alex/Work",
                        "/Users/alex/Libraries/HDRI",
                        "/Users/alex/Libraries/Textures"
                    ]
                },
                "checkAgainLooks": "mini",
                "checkAgain": ["add-a-search-folder", "check-again"],
                // **Each folder the line names is a link drawn as the page's
                // other links, with its full path**, and *Added 2023.* is
                // not — the Creator's note 5 on the test copy.
                "links": [
                    ["ckfl", "#", "/Users/alex/Work", "Work"],
                    ["ckfl", "#", "/Users/alex/Libraries/HDRI", "HDRI"],
                    ["ckfl", "#", "/Users/alex/Libraries/Textures", "Textures"]
                ],
                "linksElsewhere": 0,
                "aFolderOpened": ["open-a-searched-folder", 1]
            })
        );
    }

    /// **The Creator's fourteenth notes, note 3, on the page: where nothing
    /// was searched, the folders line is *Check again* alone — no bullet, and
    /// the paragraph takes `alone`.** A complete check whose files were all
    /// where the `.blend` says composes the folders line with no words; the
    /// page draws it as the button and nothing before it. Where a search ran
    /// the line keeps its bullet and its words, and no `alone`. The drawing's
    /// `chk-nosearch` and `.cstate p.alone .mini`.
    ///
    /// **What it bites on**: the lone `• ` before the button (screenshot 1 of
    /// the fourteenth notes), or `alone` on a line with words. **Skipped, and
    /// says so, where no Node is installed.**
    #[test]
    fn fourteenth_notes_3_a_folders_line_with_no_words_draws_no_bullet() {
        let nothing_searched = the_panel(
            &a_run(a_sitting(vec![a_row(b"//a/on.png", Kind::Image, RowState::AlreadyOnTheMachine)])),
            SERVER,
            FILE,
            None,
        );
        assert!(the_folders_line_of(&nothing_searched).said.pieces.is_empty(), "the folders line has words where nothing was searched");
        let mut searched = a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]);
        searched.the_search = Some(a_search_over(&[b"/Users/alex/Work"]));
        searched.the_list_it_searched = vec![b"/Users/alex/Work".to_vec()];
        let searched = the_panel(&a_run(searched), SERVER, FILE, None);
        let answers = serde_json::json!({ "nothing": nothing_searched, "searched": searched });

        let folder = std::env::temp_dir().join(format!("smoozy-check-lone-button-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("the-dependency-check.js"), folder.join("the-dependency-check.js")).expect("the page");
        for (name, text) in THE_CHECKS_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(
            folder.join("harness.mjs"),
            r#"import { everyPiece } from "./document.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
let stage = "nothing";
globalThis.window = { __TAURI__: { core: { invoke(command) {
  return Promise.resolve(command === "the_dependency_check_panel" ? answers[stage] : null);
} } } };
const { panels } = await import("./a-server.js");
await import("./the-dependency-check.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
const sheet = document.createElement("div");
const head = sheet.appendChild(document.createElement("div"));
const body = sheet.appendChild(document.createElement("div"));
const theLine = () => {
  const line = everyPiece(body).find((one) => one.dataset.statement === "the-folders");
  return {
    alone: line.classList.contains("alone"),
    first: line.children[0].tagName || "text",
    reads: line.textContent.replace(/ /g, " "),
  };
};
panels[0].itIsDrawn({ head, body }, "server");
await settle();
const out = { nothing: theLine() };
stage = "searched";
globalThis.told["the-dependency-check:an-act"]({ said: { server: "server", file: answers.searched.file } });
await settle();
out.searched = theLine();
console.log(JSON.stringify(out));
"#,
        )
        .expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the check's page code was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        let check_again = Prose::new(words::CHECK_AGAIN).as_str().replace('\u{a0}', " ");
        assert_eq!(
            got,
            serde_json::json!({
                "nothing": { "alone": true, "first": "BUTTON", "reads": check_again },
                "searched": { "alone": false, "first": "text", "reads": format!("• Searched Work. {check_again}") },
            })
        );
        // **The drawing draws the same line, and its rule is the shipped sheet's.**
        assert!(the_mockup().contains("<option value=\"chk-nosearch\">"));
        let sheet = fs::read_to_string(web.join("the-drawing.css")).expect("the drawing's sheet");
        assert!(sheet.contains(".cstate p.alone .mini { margin-block: 0; }"));
    }

    /// **While the check's uploads run, the strip reads *Uploading 4 of 12* in
    /// place of its figures** — `Design/Dependency Upload & Remap § Where it
    /// sits`, the drawing's `dep-sending` — the two figures values, in two
    /// parts drawn side by side, which `web/a-server.js` takes. The Creator's
    /// note 6 on 0.1.8.
    #[test]
    fn the_strip_reads_uploading_while_the_checks_files_go() {
        let held = a_run(a_sitting(vec![a_row(b"//a/missing.png", Kind::Image, RowState::NotFound)]));
        let reading = |uploading| {
            the_panel_knowing(&held, SERVER, FILE, None, &WhatElseIsKnown { the_list_in_settings: None, uploading, on_its_way: Vec::new(), a_send_is_being_counted: false })
                .the_strip_reading
        };
        let sending = reading(Some((4, 12)));
        let read: Vec<String> = sending
            .iter()
            .flat_map(|part| {
                std::iter::once(part.lead.as_str().to_string())
                    .chain(part.values.iter().map(|value| value.as_str().to_string()))
                    .chain(part.rest.iter().map(|rest| rest.as_str().to_string()))
            })
            .filter(|one| !one.is_empty())
            .collect();
        assert_eq!(read.join(" "), "Uploading 4 of 12");
        assert_eq!(sending[0].values[0].as_str(), "4");
        assert_eq!(sending[1].values[0].as_str(), "12");
        // Otherwise its *not found* figure, as before.
        let counting = reading(None);
        assert_eq!(counting.len(), 1);
        assert_eq!(counting[0].values[0].as_str(), "1");
        // **The drawing reads the same**, and the strip draws a reading in
        // parts side by side.
        assert!(the_mockup().contains("? 'Uploading 4 of 12'"));
        let server_page = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web/a-server.js"))
            .expect("the panel mechanism's page");
        assert!(server_page.contains("if (Array.isArray(reading.reading)) {"), "the strip draws one part only");
        assert!(the_page().contains("reading: said.the_strip_reading.length ? said.the_strip_reading : null,"));
        // **The panel's command reads it through Transfers' own reading.**
        let command = the_working_half()
            .split("pub fn the_dependency_check_panel<")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the command");
        assert!(command.contains("crate::surface::transfers::the_checks_uploads_on(&app, &server, &file)"));
    }

    /// The modules `web/the-dependency-check.js` imports, standing in, and a
    /// document with enough of the real one for the panel to draw into.
    /// **The Creator's seventh notes, note 2: the check opened from its strip
    /// while it was never opened over that server starts no check.** The
    /// page's registration hands `web/a-server.js` the file it is open on, and
    /// its way in from the strip is `theCheckIsShownOn` on Render Settings'
    /// file. **It fails if the strip's way in goes through
    /// `theCheckIsOpenedOn`**, which asks the server to start a check. Piece
    /// Z6. **Skipped, and says so, where no Node is installed.**
    #[test]
    fn opened_from_its_strip_the_check_starts_no_check() {
        let folder = std::env::temp_dir().join(format!("smoozy-check-from-a-strip-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("the-dependency-check.js"), folder.join("the-dependency-check.js")).expect("the page");
        for (name, text) in THE_CHECKS_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("harness.mjs"), THE_FROM_A_STRIP_HARNESS).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the check's page code was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        assert_eq!(
            got,
            serde_json::json!({
                // Nothing drawn yet, so not even the panel is asked for — and
                // above all no `the_check_is_started`.
                "asked": [],
                "after": [1, 2, 3]
            }),
            "the check opened from its strip asked for more than its panel"
        );
    }

    /// **The file's name in the panel's title is a link into Files** —
    /// `Design/Whole app § Panels over a server`, added 2026-10-12 (the
    /// Creator's ninth notes, note 5), drawn as the drawing's `.golink` inside
    /// the `h2` with the hover *Show in Files*.
    ///
    /// The page runs under Node over the panel this file composes. The name is
    /// a button in the title, the hover is the application's, and pressing it
    /// collapses the panel as its chevron does, hands Files **the folder and
    /// the name the application sent**, and shows Files. A panel opened with
    /// no file has no name to press. **It fails if the name is plain text
    /// again, if the press leaves the panel up, or if what Files is handed is
    /// anything but the application's two.** Skipped, and says so, where no
    /// Node is installed.
    #[test]
    fn the_files_name_in_the_title_shows_it_in_files() {
        let with_a_file = the_panel(&a_run(a_sitting(vec![])), SERVER, FILE, None);
        let there = with_a_file.show_in_files.clone().expect("the title's name has somewhere to go");
        assert_eq!(there.folder, b"/Users/alex/work/kitchen".to_vec());
        assert_eq!(there.name, b"kitchen_v14.blend".to_vec());
        let with_no_file = the_panel(&a_run(a_sitting(vec![])), SERVER, "", None);
        assert_eq!(with_no_file.show_in_files, None, "a panel with no file sends Files nowhere");
        let answers = serde_json::json!({ "file": with_a_file, "none": with_no_file });

        let folder = std::env::temp_dir().join(format!("smoozy-check-name-link-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("the-dependency-check.js"), folder.join("the-dependency-check.js")).expect("the page");
        for (name, text) in THE_CHECKS_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_NAME_LINK_HARNESS).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the check's page code was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        let bytes = |text: &str| serde_json::json!(text.as_bytes());
        assert_eq!(
            got,
            serde_json::json!({
                "withAFile": {
                    "reads": format!("{} kitchen_v14.blend", Prose::new(words::DEPENDENCIES).as_str()),
                    "link": {
                        "looks": "golink",
                        "type": "button",
                        "hover": Prose::new(crate::surface::render_settings::words::SHOW_IN_FILES).as_str(),
                        "says": "kitchen_v14.blend"
                    }
                },
                "pressed": [
                    ["collapsed", "server"],
                    ["opened-at", "server", bytes("/Users/alex/work/kitchen"), bytes("kitchen_v14.blend")],
                    ["tab", "files"]
                ],
                "withNoFile": { "link": false }
            }),
            "the file's name in the dependency check's title is not a link into Files"
        );
    }

    /// Draws the panel, reads its title and presses the name; then draws a
    /// panel opened with no file.
    const THE_NAME_LINK_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
let stage = "file";
globalThis.window = { __TAURI__: { core: { invoke(command) {
  return Promise.resolve(command === "the_dependency_check_panel" ? answers[stage] : null);
} } } };
const { panels } = await import("./a-server.js");
await import("./the-dependency-check.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
const theTitleOn = async (server) => {
  const sheet = document.createElement("div");
  const head = sheet.appendChild(document.createElement("div"));
  sheet.appendChild(document.createElement("div"));
  panels[0].itIsDrawn({ head, body: sheet.children[1] }, server);
  await settle();
  const heading = everyPiece(head).find((one) => one.tagName === "H2");
  return { heading, link: heading ? everyPiece(heading).find((one) => one.tagName === "BUTTON") || null : null };
};
const out = {};
let { heading, link } = await theTitleOn("server");
out.withAFile = {
  reads: heading ? heading.textContent : null,
  link: link ? { looks: link.className, type: link.type, hover: link.title, says: link.textContent } : null,
};
globalThis.calls = [];
if (link) link.press();
await settle();
out.pressed = globalThis.calls;
stage = "none";
({ link } = await theTitleOn("another"));
out.withNoFile = { link: Boolean(link) };
console.log(JSON.stringify(out));
"#;

    const THE_FROM_A_STRIP_HARNESS: &str = r#"import "./document.mjs";
const asked = [];
globalThis.window = { __TAURI__: { core: { invoke(command, args) {
  asked.push([command, (args && args.file) || null]);
  return Promise.resolve(null);
} } } };
const { panels } = await import("./a-server.js");
await import("./the-dependency-check.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
await panels[0].itIsShownOn("S", [1, 2, 3]);
await settle();
console.log(JSON.stringify({ asked, after: panels[0].theFileItIsOn("S") }));
"#;

    const THE_CHECKS_STAND_INS: [(&str, &str); 10] = [
        (
            "a-server.js",
            "export const THE_DEPENDENCY_CHECK = 'chk'; export const FILES = 'files'; export const SET_UP = 'setupview'; export const panels = []; \
             export function aPanelIsRegistered(panel) { panels.push(panel); } \
             export function thePanelIsOpenedOver() {} export function theStripReads() {} \
             export function theCurrentTabIs(tab) { (globalThis.calls ||= []).push(['tab', tab]); } \
             export function thePanelCollapsesAsItsChevronDoes(server) { (globalThis.calls ||= []).push(['collapsed', server]); }",
        ),
        (
            "files.js",
            "export function theBrowserIsOpenedAt(...args) { (globalThis.calls ||= []).push(['opened-at', ...args]); }",
        ),
        ("event-route.js", "export function whenTold(kind, handler) { (globalThis.told ||= {})[kind] = handler; }"),
        (
            "the-failure-shape.js",
            "export const EXPANDED = 'e'; export const A_COLLAPSED_ROW = 'c'; \
             export function theFailureShape() { return document.createElement('div'); }",
        ),
        ("the-machines-own-words.js", "export function theMachinesOwnWordsFor() {}"),
        ("the-underway-shape.js", "export function theMarkOnARow() { return document.createElement('span'); }"),
        ("transfers.js", "export function aTransferIsAskedFor() {}"),
        (
            "settings.js",
            "export const SEARCH_FOLDERS_SECTION = 's-fo'; \
             export function theSettingsSectionIsOpened(section) { (globalThis.sections ||= []).push(section); }",
        ),
        (
            "the-value-drawn.js",
            "export function theValueIsDrawnInto(node, value) { node.textContent = value; } \
             export function theValueDrawn(value) { const node = document.createElement('span'); node.textContent = value; return node; }",
        ),
        (
            "document.mjs",
            r#"class TextPiece {
  constructor(text) { this.data = String(text); this.parentNode = null; }
  get textContent() { return this.data; }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
}
class Piece {
  constructor(tag) {
    this.tagName = tag.toUpperCase(); this.children = []; this.own = ""; this.title = "";
    this.dataset = {}; this.hidden = false; this.type = ""; this.disabled = false;
    this.attributes = {}; this.parentNode = null; this.open = false; this.listeners = {};
    const names = new Set();
    this.names = names;
    this.classList = { add: (...more) => more.forEach((one) => names.add(one)), remove: (...more) => more.forEach((one) => names.delete(one)), contains: (one) => names.has(one) };
  }
  set className(value) { this.names.clear(); String(value).split(/\s+/).filter(Boolean).forEach((one) => this.names.add(one)); }
  get className() { return [...this.names].join(" "); }
  set textContent(value) { this.children.forEach((one) => { one.parentNode = null; }); this.children = []; this.own = String(value); }
  get textContent() { return this.own + this.children.map((one) => one.textContent).join(""); }
  appendChild(child) { if (child.parentNode) child.parentNode.removeChild(child); child.parentNode = this; this.children.push(child); return child; }
  removeChild(child) { const at = this.children.indexOf(child); if (at >= 0) this.children.splice(at, 1); child.parentNode = null; return child; }
  replaceChildren(...more) { this.textContent = ""; more.forEach((one) => this.appendChild(one)); }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
  contains(node) { for (let at = node; at; at = at.parentNode) if (at === this) return true; return false; }
  get parentElement() { return this.parentNode; }
  get childElementCount() { return this.children.filter((one) => one instanceof Piece).length; }
  setAttribute(name, value) { this.attributes[name] = String(value); }
  addEventListener(kind, handler) { (this.listeners[kind] ||= []).push(handler); }
  press() { for (const handler of this.listeners.click || []) handler({ preventDefault() {}, target: this }); }
  focus() { document.activeElement = this; }
  querySelector(selector) {
    const box = /^\[data-box="(.*)"\]$/.exec(selector);
    return box ? everyPiece(this).find((one) => one !== this && one.dataset.box === box[1]) || null : null;
  }
}
export function everyPiece(root, found = []) {
  if (!(root instanceof Piece)) return found;
  found.push(root);
  root.children.forEach((one) => everyPiece(one, found));
  return found;
}
globalThis.document = { createElement: (tag) => new Piece(tag), createTextNode: (text) => new TextPiece(text), activeElement: null };
"#,
        ),
    ];

    /// **Opens the box, presses *Add*, and reads what the panel draws after
    /// the application's answer** — the act's event delivered as the
    /// application posts it, the panel asked again.
    const THE_FOLDERS_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
let stage = "nothing";
const acts = [];
globalThis.window = { __TAURI__: { core: { invoke(command, args) {
  if (command === "the_dependency_check_panel") return Promise.resolve(answers[stage]);
  if (command === "an_act_on_the_check") {
    acts.push(args.act);
    globalThis.lastAct = [args.act, args.row];
    if (args.act === "add-a-search-folder") stage = "after";
    setImmediate(() => globalThis.told["the-dependency-check:an-act"]({ said: { server: args.server, file: answers[stage].file } }));
  }
  return Promise.resolve(null);
} } } };
const { panels } = await import("./a-server.js");
await import("./the-dependency-check.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
const sheet = document.createElement("div");
const head = sheet.appendChild(document.createElement("div"));
const body = sheet.appendChild(document.createElement("div"));
panels[0].itIsDrawn({ head, body }, "server");
await settle();
const withBox = (box) => everyPiece(sheet).find((one) => one.dataset.box === box);
withBox("search-folders").press();
await settle();
const out = { addWaitsForAFolder: withBox("add-a-search-folder").disabled };
withBox("cancel-the-folder").press();
stage = "before";
globalThis.told["the-dependency-check:an-act"]({ said: { server: "server", file: answers.before.file } });
await settle();
withBox("search-folders").press();
await settle();
out.opened = { box: Boolean(withBox("search-folders-box")), chosen: withBox("search-folders-box").textContent.includes("/Volumes/Archive/2023") };
{
  // **Settings' folders first, above Choose a folder…** — the Creator's note 7.
  const box = withBox("search-folders-box");
  const inner = box.children[0];
  const list = withBox("the-search-folders");
  out.theBox = {
    order: inner.children.map((one) => one.dataset.box || (one.children[0] && one.children[0].dataset.box) || one.tagName),
    heading: inner.children[1].textContent,
    folders: list ? list.children.map((one) => one.textContent) : [],
    listLooks: list ? list.className : null,
  };
  withBox("change-them-in-settings").press();
  await settle();
  out.changed = { sections: globalThis.sections || [], theBoxIsOpen: Boolean(withBox("search-folders-box")) };
  withBox("search-folders").press();
  await settle();
}
withBox("add-a-search-folder").press();
await settle();
const lines = everyPiece(body).filter((one) => one.dataset.statement);
out.afterAdd = {
  acts: acts.slice(),
  theBoxIsDrawnAgain: Boolean(withBox("search-folders-box")),
  statements: lines.map((one) => one.textContent.replace(/ /g, " ")),
  bold: lines.flatMap((one) => everyPiece(one).filter((piece) => piece.tagName === "B" && !piece.names.has("mono")).map((piece) => piece.textContent)),
  hover: lines.flatMap((one) => everyPiece(one).filter((piece) => piece.title).map((piece) => piece.title)),
};
out.checkAgainLooks = withBox("check-again").className;
withBox("check-again").press();
await settle();
out.checkAgain = acts.slice();
{
  // **The folders line's names are links; pressing one asks for that folder
  // by its place in the line** — the Creator's note 5 on the test copy.
  const line = everyPiece(body).find((one) => one.dataset.statement === "the-folders");
  const links = everyPiece(line).filter((one) => one.tagName === "A");
  out.links = links.map((one) => [one.className, one.href, one.title, one.textContent]);
  out.linksElsewhere = everyPiece(body).filter((one) => one.tagName === "A" && !line.contains(one)).length;
  links[1].press();
  await settle();
  out.aFolderOpened = globalThis.lastAct;
}
console.log(JSON.stringify(out));
"#;

    #[test]
    fn no_shared_shape_is_rebuilt_here() {
        let page = the_page();
        for prefix in ["qfail", "svfail", "tffail", "rdfail", "cbad", "ckdot"] {
            assert!(!page.contains(&format!("\"{prefix}")), "the page writes {prefix}");
        }
        for exported in ["theFailureShape", "theMarkOnARow", "theMachinesOwnWordsFor", "theStripReads"] {
            assert!(page.contains(exported), "the page does not call {exported}");
        }
        // A box over the panel is a dialog, which is what piece 5's Escape
        // steps aside for.
        assert!(page.matches("setAttribute(\"role\", \"dialog\")").count() >= 3);
    }

    /// **The one place this panel reaches the diary is M32's second limb, it
    /// passes the composer's own sentence, and it raises no record.** The
    /// endings and the once statement are drawn in the one shape, a row's
    /// reason is a red note with its cross among the row's buttons, and none
    /// raises one. *(Until the Verdict's upstream must-do this asserted the panel never
    /// reached the capture at all.)*
    #[test]
    fn the_diary_is_reached_once_and_with_the_composers_own_sentence() {
        let working = the_working_half();
        assert_eq!(working.matches("WhoseSentenceItIs::TheComposersOwn(").count(), 1);
        assert!(!working.contains("AStandInThePlumbingComposed"));
        assert_eq!(working.matches("the_diary_is_handed_a_sentence_with_no_record(").count(), 1);
        assert!(!working.contains("failure_records::raise"));
    }

    #[test]
    fn the_page_never_measures_itself_against_the_window() {
        let page = the_page();
        for reached in ["axlocked", "titlebar", "getBoundingClientRect", "window.inner", "documentElement"] {
            assert!(!page.contains(reached), "the page reaches for {reached}");
        }
    }

    #[test]
    fn every_handler_reads_the_deliverys_said_and_never_the_delivery() {
        let page = the_page();
        for (at, _) in page.match_indices("whenTold(") {
            let line = page[at..].lines().next().unwrap_or_default();
            assert!(line.contains("(delivery)"), "{line}");
        }
    }

    #[test]
    fn nothing_here_repeats_on_a_timer() {
        let page = the_page();
        for started in ["setInterval", "setTimeout", "requestAnimationFrame"] {
            assert!(!page.contains(started), "the page starts {started}");
        }
    }

    #[test]
    fn the_page_writes_no_markup_and_no_inline_style() {
        let page: String = the_page()
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for forbidden in ["innerHTML", "outerHTML", "insertAdjacentHTML", ".style."] {
            assert!(!page.contains(forbidden), "the page writes {forbidden}");
        }
    }

    /// **The record is reached only through its keepers.** A sitting's
    /// findings, a decision and a standing are written by
    /// `crate::checking_a_file` through `crate::record::dependency_records`;
    /// this file lends it a connection and reads the server rows and the
    /// request records through their own keepers, and **writes no statement of
    /// its own**. The selection, the search's per-folder outcome and an ending
    /// stay run-held. **The server's name is read through `File Browser`'s
    /// surface reader** (must-do 5).
    #[test]
    fn the_record_is_reached_only_through_its_keepers() {
        let source = the_working_half();
        for reached in ["schema::", "migrations::", "INSERT ", "UPDATE ", "DELETE ", "suppressed_warnings", "prepare("] {
            assert!(!source.contains(reached), "this piece reaches the record at {reached}");
        }
    }

    /// **The temporary seeding used to photograph the nine states is gone.**
    #[test]
    fn nothing_seeded_is_left() {
        let mark = ["WAT", "CH"].concat();
        let seed = ["watch", "_seed"].concat();
        assert!(!the_working_half().contains(&mark) && !the_working_half().contains(&seed));
        assert!(!the_page().contains(&mark) && !the_page().contains(&seed));
        assert!(!include_str!("mod.rs").contains(&seed));
    }

    // ── Step 10c's twelfth-pass fix piece 6 ─────────────────────────────────

    fn a_blender_list(versions: &[(u32, u32, u32)]) -> smoozy_render_settings::acting::versions::TheVersionListAsk {
        smoozy_render_settings::acting::versions::TheVersionListAsk::Answered(
            smoozy_render_settings::version::TheListThisPageAsksFor {
                entries: versions
                    .iter()
                    .map(|(major, minor, patch)| smoozy_server_setup::search::ACandidate {
                        path: format!("/home/ubuntu/blender-{major}.{minor}.{patch}/blender"),
                        answered: Some(Version { major: *major, minor: *minor, patch: *patch }),
                    })
                    .collect(),
                answer: smoozy_server_setup::search::WhatTheAnswerSays {
                    the_search_is_by_name: false,
                    folders_refused: 0,
                    did_not_answer: Vec::new(),
                },
                at: std::time::SystemTime::now(),
            },
        )
    }

    fn a_page_holding(
        list: Option<smoozy_render_settings::acting::versions::TheVersionListAsk>,
    ) -> Mutex<crate::surface::render_settings::WhatTheRunHolds> {
        let mut page = crate::surface::render_settings::WhatTheRunHolds::default();
        if let Some(list) = list {
            crate::surface::render_settings::the_version_list_came_back(&mut page, SERVER, list);
        }
        Mutex::new(page)
    }

    fn never_asked() -> smoozy_render_settings::acting::versions::TheVersionListAsk {
        panic!("the list was asked for while the page held one")
    }

    /// **`Dependency Upload & Remap` G2, with `Render Settings` B2a,
    /// `Render Settings` B5 and `Render Settings` B5a: a check with no Blender
    /// resolved says why, in the render settings
    /// panel's own words, and *This server has no Blender.* stays for a server
    /// that has none.** The twelfth pass on Hyperstack read *This server has no
    /// Blender.* with 5.2.2 and 4.5.14 on the machine.
    #[test]
    fn a_check_with_no_blender_resolved_says_why_in_the_render_settings_words() {
        let said = |list| {
            the_blender_the_check_reads_with(&a_page_holding(Some(list)), SERVER, FILE, never_asked, || None)
        };
        assert_eq!(
            said(a_blender_list(&[(4, 5, 14), (5, 2, 2)])),
            Err(NoBlenderToReadWith::Said(words::NOTHING_TO_MATCH_ON)),
            "a server with Blenders, and a file with nothing to match on"
        );
        assert_eq!(
            said(a_blender_list(&[(4, 2, 0)])),
            Err(NoBlenderToReadWith::Said(words::SMOOZY_DOES_NOT_CHOOSE_BELOW_THE_FLOOR))
        );
        assert_eq!(
            said(a_blender_list(&[])),
            Err(NoBlenderToReadWith::Said(words::NO_BLENDER_ON_THIS_SERVER))
        );
        // **Each is the render settings panel's own sentence**, not a fresh one.
        assert_eq!(words::NOTHING_TO_MATCH_ON, crate::surface::render_settings::words::NOTHING_TO_MATCH_ON);
    }

    /// **`Render Settings` B2a: *a fresh … list* — the check asks for the
    /// server's Blenders where no page has**, rather than saying the server has
    /// none. A check started from Files, with the render settings never opened
    /// on that server, holds no list.
    #[test]
    fn a_check_with_no_list_in_hand_asks_the_server_for_one() {
        let page = a_page_holding(None);
        let mut asked = 0;
        let came_to = the_blender_the_check_reads_with(
            &page,
            SERVER,
            FILE,
            || {
                asked += 1;
                a_blender_list(&[(4, 5, 14), (5, 2, 2)])
            },
            || None,
        );
        assert_eq!(asked, 1, "the server was not asked for its Blenders");
        assert_eq!(came_to, Err(NoBlenderToReadWith::Said(words::NOTHING_TO_MATCH_ON)));
        assert!(
            page.lock().expect("the page").version_lists.contains_key(SERVER),
            "the list the check asked for is not held for the page"
        );
        // A server that could not be asked is the check's own first ending,
        // never *This server has no Blender.*
        let came_to = the_blender_the_check_reads_with(
            &a_page_holding(None),
            SERVER,
            FILE,
            || smoozy_render_settings::acting::versions::TheVersionListAsk::TheMachineCouldNotBeAsked,
            || None,
        );
        assert_eq!(came_to, Err(NoBlenderToReadWith::TheServerCouldNotBeAsked));
    }

    /// **`Dependency Upload & Remap` K5 with E1-3: a check on a file Smoozy
    /// has no record of the origin of says so at the top, once, and draws its
    /// rows as always.** The twelfth pass checked a file uploaded before its
    /// server was set up and read 58 rows of *Not found* with no reason given.
    #[test]
    fn a_check_of_a_file_with_no_known_origin_says_k5s_limit_at_the_top_once() {
        use crate::checking_a_file::TheSittingIsOf;
        use smoozy_dependencies::bytes::MachineLocation;
        use smoozy_dependencies::record::MachineKey;
        const THE_LINE: &str = "no-record-of-where-it-came-from";
        let installation = Identity::issue(IdentityKind::Installation).expect("an installation");
        let of = |holding| TheSittingIsOf {
            server: SERVER.to_string(),
            file: FILE.to_string(),
            root: MachineLocation::composed(b"/home/ubuntu/battle.blend".to_vec()),
            holding,
            with: a_blender(),
            machine: MachineKey::of(an_identity(SERVER), installation.clone()),
            layout: None,
        };
        let a_sitting_holding = |holding| {
            let mut sitting = a_sitting(vec![
                a_row(b"//textures/rock.png", Kind::Image, RowState::NotFound),
                a_row(b"//textures/moss.png", Kind::Image, RowState::NotFound),
            ]);
            sitting.the_search = Some(TheSearchInHand {
                folders: vec![(b"/Users/alex/Assets".to_vec(), None)],
                the_matches_came_from: None,
                running: false,
                searched_at: None,
            });
            sitting.of = Some(of(holding));
            sitting
        };
        let panel = the_panel(
            &a_run(a_sitting_holding(HoldingFile::OnTheMachineWithNoKnownLocalOrigin {
                machine: MachineKey::of(an_identity(SERVER), installation.clone()),
                location: MachineLocation::composed(b"/home/ubuntu/battle.blend".to_vec()),
            })),
            SERVER,
            FILE,
            None,
        );
        assert_eq!(
            panel.the_statements.iter().filter(|one| one.which.as_str() == THE_LINE).count(),
            1,
            "K5's limit is not said once"
        );
        assert_eq!(panel.the_statements[0].which.as_str(), THE_LINE, "K5's limit is not at the top");
        assert_eq!(panel.the_statements[0].said, Words::plain(words::NO_RECORD_OF_WHERE_IT_CAME_FROM));
        let rows: usize = panel.the_table.expect("the rows").groups.iter().map(|group| group.rows.len()).sum();
        assert_eq!(rows, 2, "the rows are not drawn as always");

        // A file whose origin on this computer is known carries no such line.
        let panel = the_panel(
            &a_run(a_sitting_holding(HoldingFile::OnTheUsersComputer(
                StoredLocation::as_the_system_returned_it(FILE.as_bytes()),
            ))),
            SERVER,
            FILE,
            None,
        );
        assert!(!panel.the_statements.iter().any(|one| one.which.as_str() == THE_LINE));
    }

    // ── Step 10c's twelfth-pass fix piece 6b ────────────────────────────────

    /// **`Render Settings` B2's one source, read where no page handed it.** A
    /// check started from Files holds no saved-with version, and until piece
    /// 6b nothing read one, so a check with no Blender picked said *This file
    /// does not say which Blender saved it* of every file. **The reading lands
    /// where the page keeps its own**, so the check and the page resolve alike.
    #[test]
    fn a_check_with_no_saved_with_version_held_reads_it_and_matches_without_a_pick() {
        let page = a_page_holding(Some(a_blender_list(&[(4, 5, 14), (5, 2, 2)])));
        let mut read = 0;
        let came_to = the_blender_the_check_reads_with(&page, SERVER, FILE, never_asked, || {
            read += 1;
            Some("5.2".to_string())
        });
        assert_eq!(read, 1, "the file's saved-with version was not read");
        let with = came_to.expect("a Blender resolved with no pick");
        assert_eq!((with.version.major, with.version.minor, with.version.patch), (5, 2, 2));
        assert!(
            matches!(
                crate::surface::render_settings::the_version_resolved(&page.lock().expect("the page"), SERVER, FILE),
                smoozy_render_settings::version::Resolution::Automatic { .. }
            ),
            "the version the check read is not held for the page"
        );
        // **A blank reading is the file not saying**, in the page's own words.
        let came_to =
            the_blender_the_check_reads_with(&a_page_holding(Some(a_blender_list(&[(5, 2, 2)]))), SERVER, FILE, never_asked, || None);
        assert_eq!(came_to, Err(NoBlenderToReadWith::Said(words::NOTHING_TO_MATCH_ON)));
    }

    /// **E1-3's line and the code agree, and the Design document holds the
    /// same words** (`Dependency Upload & Remap` K5; `Dependency Upload & Remap` F3;
    /// `Dependency Upload & Remap` F27). The line said
    /// relative paths are looked for only on the server; the check does not
    /// search the server for one, and the search of the user's search folders finds it by name.
    #[test]
    fn k5s_line_says_where_relative_paths_are_looked_for_and_the_design_says_the_same() {
        assert!(
            words::NO_RECORD_OF_WHERE_IT_CAME_FROM.contains("Searched in your search folders"),
            "{}",
            words::NO_RECORD_OF_WHERE_IT_CAME_FROM
        );
        assert!(!words::NO_RECORD_OF_WHERE_IT_CAME_FROM.contains("on the server,"));
        let design = include_str!("../../../../Preparation/Design/Dependency Upload & Remap.md");
        assert!(
            design.contains(&format!("*{}*", words::NO_RECORD_OF_WHERE_IT_CAME_FROM)),
            "the Design document's line and the code's differ"
        );
    }
}
